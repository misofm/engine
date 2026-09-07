_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_:
.Lfunc_begin34:
	.loc	1 1883 0
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
	subq	$2200, %rsp
	.cfi_def_cfa_offset 2256
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdi, 1240(%rsp)
.Ltmp2939:
	.loc	1 1884 51 prologue_end
	movzbl	2816(%rsi), %eax
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqu	%ymm0, 1760(%rsp)
	vmovdqu	%ymm0, 1728(%rsp)
	vmovdqu	%ymm0, 1696(%rsp)
	vmovdqu	%ymm0, 1664(%rsp)
	vmovdqu	%ymm0, 1632(%rsp)
	vmovdqu	%ymm0, 1600(%rsp)
	vmovdqu	%ymm0, 1568(%rsp)
	vmovdqu	%ymm0, 1536(%rsp)
	vmovdqu	%ymm0, 1504(%rsp)
	vmovdqu	%ymm0, 1472(%rsp)
.Ltmp2940:
	.loc	43 1032 9
	movb	%al, 1792(%rsp)
.Ltmp2941:
	.loc	43 186 45
	cmpb	%al, 108(%rdx)
.Ltmp2942:
	.loc	1 1888 12
	jne	.LBB34_442
	cmpq	$0, 64(%rdx)
	jne	.LBB34_442
	.loc	1 0 12 is_stmt 0
	movq	%rsi, %r14
	movq	48(%rdx), %r10
	movq	56(%rdx), %r8
	movq	32(%rdx), %rax
	movq	%rax, 1120(%rsp)
	movq	40(%rdx), %rax
	movq	%rax, 48(%rsp)
	movq	%rdx, 1040(%rsp)
	movq	96(%rdx), %r11
.Ltmp2943:
	.loc	3 900 12 is_stmt 1
	cmpq	$1, %r8
	movq	%r8, %rax
	adcq	$-1, %rax
	movq	%rax, 128(%rsp)
	leaq	348(%rsi), %rbx
	xorl	%eax, %eax
	vmovss	.LCPI34_0(%rip), %xmm2
	movq	%rsi, 8(%rsp)
	movq	%r8, 1104(%rsp)
	movq	%r10, 1088(%rsp)
	movq	%r11, 1072(%rsp)
	jmp	.LBB34_4
	.loc	3 0 12 is_stmt 0
.Ltmp2944:
	.p2align	4
.LBB34_3:
	.loc	3 900 12 is_stmt 1
	addq	$160, %rbx
	movq	80(%rsp), %rcx
	movq	%rcx, %rax
.Ltmp2945:
	.loc	2 1916 50
	cmpq	$4, %rcx
	movq	8(%rsp), %r14
.Ltmp2946:
	.loc	3 900 12
	je	.LBB34_100
.Ltmp2947:
.LBB34_4:
	.loc	1 1892 25
	cmpq	%r8, %rax
	je	.LBB34_489
.Ltmp2948:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rax), %rcx
.Ltmp2949:
	.loc	1 1893 23 is_stmt 1
	cmpq	128(%rsp), %rax
	je	.LBB34_490
	.loc	1 0 23 is_stmt 0
	movl	(%r10,%rax,4), %edi
	.loc	1 1893 23
	movl	(%r10,%rcx,4), %esi
.Ltmp2950:
	.loc	38 1050 16 is_stmt 1
	cmpl	%edi, %esi
	jb	.LBB34_450
	cmpq	%rsi, 48(%rsp)
	jb	.LBB34_450
.Ltmp2951:
	.loc	38 0 16 is_stmt 0
	movq	%rcx, 80(%rsp)
	movq	%rbx, 16(%rsp)
	.loc	1 1897 17 is_stmt 1
	movl	2916(%r14), %edx
	movl	$0, 176(%rsp)
	movl	$0, 184(%rsp)
	movl	$0, 192(%rsp)
	movl	$0, 200(%rsp)
	movl	$0, 208(%rsp)
	movl	$0, 216(%rsp)
	movl	$0, 224(%rsp)
	movl	$0, 232(%rsp)
	movl	$0, 240(%rsp)
	movl	$0, 248(%rsp)
	movl	$0, 256(%rsp)
	movl	$0, 264(%rsp)
	movl	$0, 272(%rsp)
	movl	$0, 280(%rsp)
	movl	$0, 288(%rsp)
	movl	$0, 296(%rsp)
	movl	$0, 304(%rsp)
	movl	$0, 312(%rsp)
	movl	$0, 320(%rsp)
	movl	$0, 328(%rsp)
.Ltmp2952:
	.loc	32 1714 9
	cmpl	%edi, %esi
.Ltmp2953:
	.loc	33 180 28
	jne	.LBB34_81
.Ltmp2954:
.LBB34_9:
	.loc	33 0 28 is_stmt 0
	movl	$76, %eax
	movq	16(%rsp), %rbx
	movq	%rbx, %rcx
.Ltmp2955:
	.loc	33 180 28
	jmp	.LBB34_13
.Ltmp2956:
	.loc	33 0 28
.Ltmp2957:
	.p2align	4
.LBB34_10:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
.LBB34_11:
	.loc	36 0 0
	vmovd	%xmm0, -4(%rcx)
	movl	%edx, (%rcx)
.Ltmp2958:
.LBB34_12:
	.loc	32 1714 9 is_stmt 1
	addq	$80, %rax
	addq	$1328, %rcx
	cmpq	$236, %rax
.Ltmp2959:
	.loc	33 180 28
	je	.LBB34_3
.Ltmp2960:
.LBB34_13:
	.loc	1 1392 24
	cmpl	$1, 100(%rsp,%rax)
	jne	.LBB34_14
	.loc	1 1392 29 is_stmt 0
	vmovd	104(%rsp,%rax), %xmm0
.Ltmp2961:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -152(%rcx)
	.loc	36 81 48
	vmovd	-156(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp2962:
	.loc	36 112 9
	jg	.LBB34_27
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_27
	negl	%edx
	jo	.LBB34_27
.Ltmp2963:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -156(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -148(%rcx)
	movl	%edx, -144(%rcx)
.Ltmp2964:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 108(%rsp,%rax)
	je	.LBB34_29
	.loc	1 0 24 is_stmt 0
.Ltmp2965:
	.p2align	4
.LBB34_15:
	.loc	1 1392 24
	cmpl	$1, 116(%rsp,%rax)
	jne	.LBB34_16
.LBB34_35:
	.loc	1 1392 29
	vmovd	120(%rsp,%rax), %xmm0
.Ltmp2966:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -120(%rcx)
	.loc	36 81 48
	vmovd	-124(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp2967:
	.loc	36 112 9
	jg	.LBB34_39
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_39
	negl	%edx
	jo	.LBB34_39
.Ltmp2968:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -124(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -116(%rcx)
	movl	%edx, -112(%rcx)
.Ltmp2969:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 124(%rsp,%rax)
	je	.LBB34_41
	.loc	1 0 24 is_stmt 0
.Ltmp2970:
	.p2align	4
.LBB34_17:
	.loc	1 1392 24
	cmpl	$1, 132(%rsp,%rax)
	jne	.LBB34_18
.LBB34_47:
	.loc	1 1392 29
	vmovd	136(%rsp,%rax), %xmm0
.Ltmp2971:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -88(%rcx)
	.loc	36 81 48
	vmovd	-92(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp2972:
	.loc	36 112 9
	jg	.LBB34_51
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_51
	negl	%edx
	jo	.LBB34_51
.Ltmp2973:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -92(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -84(%rcx)
	movl	%edx, -80(%rcx)
.Ltmp2974:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 140(%rsp,%rax)
	je	.LBB34_53
	.loc	1 0 24 is_stmt 0
.Ltmp2975:
	.p2align	4
.LBB34_19:
	.loc	1 1392 24
	cmpl	$1, 148(%rsp,%rax)
	jne	.LBB34_20
.LBB34_59:
	.loc	1 1392 29
	vmovd	152(%rsp,%rax), %xmm0
.Ltmp2976:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -56(%rcx)
	.loc	36 81 48
	vmovd	-60(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp2977:
	.loc	36 112 9
	jg	.LBB34_63
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_63
	negl	%edx
	jo	.LBB34_63
.Ltmp2978:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -60(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -52(%rcx)
	movl	%edx, -48(%rcx)
.Ltmp2979:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 156(%rsp,%rax)
	je	.LBB34_65
	.loc	1 0 24 is_stmt 0
.Ltmp2980:
	.p2align	4
.LBB34_21:
	.loc	1 1392 24
	cmpl	$1, 164(%rsp,%rax)
	jne	.LBB34_22
.LBB34_71:
	.loc	1 1392 29
	vmovd	168(%rsp,%rax), %xmm0
.Ltmp2981:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -24(%rcx)
	.loc	36 81 48
	vmovd	-28(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp2982:
	.loc	36 112 9
	jg	.LBB34_75
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_75
	negl	%edx
	jo	.LBB34_75
.Ltmp2983:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -28(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -20(%rcx)
	movl	%edx, -16(%rcx)
.Ltmp2984:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 172(%rsp,%rax)
	jne	.LBB34_12
	jmp	.LBB34_77
	.loc	1 0 24 is_stmt 0
.Ltmp2985:
	.p2align	4
.LBB34_14:
	.loc	1 1392 24
	cmpl	$1, 108(%rsp,%rax)
	jne	.LBB34_15
.LBB34_29:
	.loc	1 1392 29
	vmovd	112(%rsp,%rax), %xmm0
.Ltmp2986:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -136(%rcx)
	.loc	36 81 48
	vmovd	-140(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp2987:
	.loc	36 112 9
	jg	.LBB34_33
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_33
	negl	%edx
	jo	.LBB34_33
.Ltmp2988:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -140(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -132(%rcx)
	movl	%edx, -128(%rcx)
.Ltmp2989:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 116(%rsp,%rax)
	jne	.LBB34_16
	jmp	.LBB34_35
	.loc	1 0 24 is_stmt 0
.Ltmp2990:
	.p2align	4
.LBB34_27:
.Ltmp2991:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -148(%rcx)
	movl	%edx, -144(%rcx)
.Ltmp2992:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 108(%rsp,%rax)
	jne	.LBB34_15
	jmp	.LBB34_29
	.loc	1 0 24 is_stmt 0
.Ltmp2993:
	.p2align	4
.LBB34_39:
.Ltmp2994:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -116(%rcx)
	movl	%edx, -112(%rcx)
.Ltmp2995:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 124(%rsp,%rax)
	jne	.LBB34_17
	jmp	.LBB34_41
	.loc	1 0 24 is_stmt 0
.Ltmp2996:
	.p2align	4
.LBB34_51:
.Ltmp2997:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -84(%rcx)
	movl	%edx, -80(%rcx)
.Ltmp2998:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 140(%rsp,%rax)
	jne	.LBB34_19
	jmp	.LBB34_53
	.loc	1 0 24 is_stmt 0
.Ltmp2999:
	.p2align	4
.LBB34_63:
.Ltmp3000:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -52(%rcx)
	movl	%edx, -48(%rcx)
.Ltmp3001:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 156(%rsp,%rax)
	jne	.LBB34_21
	jmp	.LBB34_65
	.loc	1 0 24 is_stmt 0
.Ltmp3002:
	.p2align	4
.LBB34_75:
.Ltmp3003:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -20(%rcx)
	movl	%edx, -16(%rcx)
.Ltmp3004:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 172(%rsp,%rax)
	jne	.LBB34_12
	jmp	.LBB34_77
	.loc	1 0 24 is_stmt 0
.Ltmp3005:
	.p2align	4
.LBB34_33:
.Ltmp3006:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -132(%rcx)
	movl	%edx, -128(%rcx)
.Ltmp3007:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 116(%rsp,%rax)
	je	.LBB34_35
	.loc	1 0 24 is_stmt 0
.Ltmp3008:
	.p2align	4
.LBB34_16:
	.loc	1 1392 24
	cmpl	$1, 124(%rsp,%rax)
	jne	.LBB34_17
.LBB34_41:
	.loc	1 1392 29
	vmovd	128(%rsp,%rax), %xmm0
.Ltmp3009:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -104(%rcx)
	.loc	36 81 48
	vmovd	-108(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3010:
	.loc	36 112 9
	jg	.LBB34_45
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_45
	negl	%edx
	jo	.LBB34_45
.Ltmp3011:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -108(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -100(%rcx)
	movl	%edx, -96(%rcx)
.Ltmp3012:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 132(%rsp,%rax)
	jne	.LBB34_18
	jmp	.LBB34_47
	.loc	1 0 24 is_stmt 0
.Ltmp3013:
	.p2align	4
.LBB34_45:
.Ltmp3014:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -100(%rcx)
	movl	%edx, -96(%rcx)
.Ltmp3015:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 132(%rsp,%rax)
	je	.LBB34_47
	.loc	1 0 24 is_stmt 0
.Ltmp3016:
	.p2align	4
.LBB34_18:
	.loc	1 1392 24
	cmpl	$1, 140(%rsp,%rax)
	jne	.LBB34_19
.LBB34_53:
	.loc	1 1392 29
	vmovd	144(%rsp,%rax), %xmm0
.Ltmp3017:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -72(%rcx)
	.loc	36 81 48
	vmovd	-76(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3018:
	.loc	36 112 9
	jg	.LBB34_57
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_57
	negl	%edx
	jo	.LBB34_57
.Ltmp3019:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -76(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -68(%rcx)
	movl	%edx, -64(%rcx)
.Ltmp3020:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 148(%rsp,%rax)
	jne	.LBB34_20
	jmp	.LBB34_59
	.loc	1 0 24 is_stmt 0
.Ltmp3021:
	.p2align	4
.LBB34_57:
.Ltmp3022:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -68(%rcx)
	movl	%edx, -64(%rcx)
.Ltmp3023:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 148(%rsp,%rax)
	je	.LBB34_59
	.loc	1 0 24 is_stmt 0
.Ltmp3024:
	.p2align	4
.LBB34_20:
	.loc	1 1392 24
	cmpl	$1, 156(%rsp,%rax)
	jne	.LBB34_21
.LBB34_65:
	.loc	1 1392 29
	vmovd	160(%rsp,%rax), %xmm0
.Ltmp3025:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -40(%rcx)
	.loc	36 81 48
	vmovd	-44(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3026:
	.loc	36 112 9
	jg	.LBB34_69
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_69
	negl	%edx
	jo	.LBB34_69
.Ltmp3027:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -44(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -36(%rcx)
	movl	%edx, -32(%rcx)
.Ltmp3028:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 164(%rsp,%rax)
	jne	.LBB34_22
	jmp	.LBB34_71
	.loc	1 0 24 is_stmt 0
.Ltmp3029:
	.p2align	4
.LBB34_69:
.Ltmp3030:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -36(%rcx)
	movl	%edx, -32(%rcx)
.Ltmp3031:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 164(%rsp,%rax)
	je	.LBB34_71
	.loc	1 0 24 is_stmt 0
.Ltmp3032:
	.p2align	4
.LBB34_22:
	.loc	1 1392 24
	cmpl	$1, 172(%rsp,%rax)
	jne	.LBB34_12
.LBB34_77:
	.loc	1 1392 29
	vmovd	176(%rsp,%rax), %xmm0
.Ltmp3033:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -8(%rcx)
	.loc	36 81 48
	vmovd	-12(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3034:
	.loc	36 112 9
	jg	.LBB34_10
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_10
	negl	%edx
	jo	.LBB34_10
.Ltmp3035:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -12(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 89 6
	jmp	.LBB34_11
.Ltmp3036:
	.loc	36 0 6 is_stmt 0
.Ltmp3037:
	.p2align	4
.LBB34_81:
	subq	%rdi, %rsi
	leaq	(%rdi,%rdi,4), %rcx
	movq	%rdx, 64(%rsp)
	movq	1120(%rsp), %rdx
	leaq	(%rdx,%rcx,8), %r13
	movq	64(%rsp), %rdx
	leaq	(%rsi,%rsi,4), %rcx
	leaq	(%r13,%rcx,8), %rbx
	.loc	1 1899 17 is_stmt 1
	leaq	(%rax,%rax,4), %rax
	leaq	1472(%rsp,%rax,8), %r15
	movq	1488(%rsp,%rax,8), %r14
	movb	$1, %sil
	xorl	%ebp, %ebp
	jmp	.LBB34_82
	.loc	1 0 17 is_stmt 0
.Ltmp3038:
	.p2align	4
.LBB34_98:
.Ltmp3039:
	addq	$40, %r13
.Ltmp3040:
	.loc	38 2428 13 is_stmt 1
	incq	%r14
	movq	$-1, %rax
	cmoveq	%rax, %r14
.Ltmp3041:
	.loc	1 0 0 is_stmt 0
	movq	%r14, 16(%r15)
.Ltmp3042:
	.loc	4 82 9 is_stmt 1
	incq	%rbp
.Ltmp3043:
	.loc	32 1714 9
	cmpq	%rbx, %r13
.Ltmp3044:
	.loc	33 180 28
	je	.LBB34_9
.Ltmp3045:
.LBB34_82:
	.loc	1 1354 30
	movl	32(%r13), %eax
	.loc	1 1354 24 is_stmt 0
	cmpl	$1, %eax
	je	.LBB34_86
	cmpl	$2, %eax
	jne	.LBB34_98
	.loc	1 0 24
	movl	$1, %eax
	leaq	256(%rsp), %r12
.Ltmp3046:
	.loc	1 1362 29 is_stmt 1
	movl	16(%r13), %edi
	cmpq	$2, %rdi
.Ltmp3047:
	.loc	38 1050 16
	jae	.LBB34_87
.Ltmp3048:
.LBB34_85:
	.loc	38 0 16 is_stmt 0
	xorl	%ecx, %ecx
	cmpq	%rdx, %rbp
.Ltmp3049:
	.loc	1 1370 25 is_stmt 1
	jb	.LBB34_88
	jmp	.LBB34_98
.Ltmp3050:
	.loc	1 0 25 is_stmt 0
.Ltmp3051:
	.p2align	4
.LBB34_86:
	xorl	%eax, %eax
	leaq	176(%rsp), %r12
	.loc	1 1362 29 is_stmt 1
	movl	16(%r13), %edi
	cmpq	$2, %rdi
.Ltmp3052:
	.loc	38 1050 16
	jb	.LBB34_85
.LBB34_87:
	.loc	38 1054 31
	leaq	-2(%rdi), %rcx
	movq	%rcx, 32(%rsp)
.Ltmp3053:
	.loc	28 1580 16
	xorl	%ecx, %ecx
	cmpl	$12, %edi
	setb	%cl
	cmpq	%rdx, %rbp
.Ltmp3054:
	.loc	1 1370 25
	jae	.LBB34_98
.LBB34_88:
	cmpq	$1, %rcx
	jne	.LBB34_98
	.loc	1 1372 20
	cmpl	$1, 28(%r13)
	jne	.LBB34_98
	.loc	1 1373 20
	cmpq	%r11, (%r13)
	jne	.LBB34_98
	.loc	1 1374 20
	cmpq	%r11, 8(%r13)
	jne	.LBB34_98
	.loc	1 1375 20
	vmovd	20(%r13), %xmm0
.Ltmp3055:
	.loc	23 1244 18
	vmovd	%xmm0, %ecx
.Ltmp3056:
	.loc	1 1375 20
	cmpl	%ecx, 24(%r13)
	jne	.LBB34_98
	.loc	1 0 20 is_stmt 0
	movl	%esi, 144(%rsp)
	.loc	1 1376 43 is_stmt 1
	cmpl	$11, %edi
	ja	.LBB34_495
	.loc	1 0 43 is_stmt 0
	leal	(%rax,%rdi,2), %eax
	movl	%eax, 1136(%rsp)
	.loc	1 1376 42
	leaq	(%rdi,%rdi,4), %rax
	leaq	.Lalloc_cc33a3b9cd8c16d253f2168b5461d31d(%rip), %rcx
	leaq	(%rcx,%rax,8), %rdi
	vmovdqa	%xmm0, 976(%rsp)
	.loc	1 1376 20
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	976(%rsp), %xmm1
	movl	1136(%rsp), %r9d
	cmpl	1056(%rsp), %r9d
	seta	%cl
	testb	%al, %al
	movq	1104(%rsp), %r8
	movq	1088(%rsp), %r10
	movq	1072(%rsp), %r11
	vmovss	.LCPI34_0(%rip), %xmm2
	movq	64(%rsp), %rdx
	movl	144(%rsp), %esi
	je	.LBB34_98
	orb	%sil, %cl
	testb	$1, %cl
	je	.LBB34_98
	.loc	1 0 20
	movq	32(%rsp), %rdi
.Ltmp3057:
	.loc	1 1378 45 is_stmt 1
	cmpq	$9, %rdi
	ja	.LBB34_498
.Ltmp3058:
	.loc	47 430 9
	cmpl	$0, (%r12,%rdi,8)
.Ltmp3059:
	.loc	1 1383 17
	jne	.LBB34_98
.Ltmp3060:
	.loc	31 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
	movq	32(%rsp), %rax
.Ltmp3061:
	.loc	1 1388 13
	movl	$1, (%r12,%rax,8)
	vmovss	%xmm0, 4(%r12,%rax,8)
.Ltmp3062:
	.loc	32 1714 9
	addq	$40, %r13
.Ltmp3063:
	.loc	33 180 28
	incq	%rbp
	xorl	%esi, %esi
	movl	%r9d, 1056(%rsp)
.Ltmp3064:
	.loc	32 1714 9
	cmpq	%rbx, %r13
.Ltmp3065:
	.loc	33 180 28
	jne	.LBB34_82
	jmp	.LBB34_9
.Ltmp3066:
.LBB34_100:
	.loc	33 0 28 is_stmt 0
	movq	1040(%rsp), %rcx
	.loc	1 1904 13 is_stmt 1
	movq	(%rcx), %r8
	movq	8(%rcx), %rdi
	.loc	1 1905 13
	movq	16(%rcx), %rax
	movq	%rax, 912(%rsp)
	movq	24(%rcx), %rdx
	.loc	1 1906 13
	movl	104(%rcx), %esi
.Ltmp3067:
	.loc	1 1142 12
	movl	2692(%r14), %ecx
	.loc	1 1142 27 is_stmt 0
	movzbl	2696(%r14), %eax
	.loc	1 1142 5
	cmpl	$1, %ecx
	movq	%rsi, 928(%rsp)
	movq	%rdx, 112(%rsp)
	movq	%rdi, 120(%rsp)
	movq	%r8, 920(%rsp)
	je	.LBB34_105
	cmpl	$2, %ecx
	jne	.LBB34_108
	testb	%al, %al
	jne	.LBB34_109
.Ltmp3068:
	.loc	1 1086 11 is_stmt 1
	testq	%rsi, %rsi
	je	.LBB34_419
.Ltmp3069:
	.loc	1 0 11 is_stmt 0
	movl	2688(%r14), %eax
	movl	%eax, 908(%rsp)
	movq	2672(%r14), %r13
	leaq	1328(%r14), %rax
	movq	%rax, 1016(%rsp)
	xorl	%r15d, %r15d
	jmp	.LBB34_245
.LBB34_105:
	.loc	1 1142 5 is_stmt 1
	testb	%al, %al
	jne	.LBB34_109
.Ltmp3070:
	.loc	1 1086 11
	testq	%rsi, %rsi
	je	.LBB34_419
.Ltmp3071:
	.loc	1 0 11 is_stmt 0
	movl	2688(%r14), %eax
	movl	%eax, 908(%rsp)
	movq	2672(%r14), %rdi
	leaq	1328(%r14), %rax
	movq	%rax, 1016(%rsp)
	xorl	%r15d, %r15d
	movq	%rdi, 1024(%rsp)
	jmp	.LBB34_158
.LBB34_108:
	.loc	1 1142 5 is_stmt 1
	testb	%al, %al
	je	.LBB34_330
.LBB34_109:
.Ltmp3072:
	.loc	1 1086 11
	testq	%rsi, %rsi
	je	.LBB34_419
	.loc	1 0 11 is_stmt 0
	movl	2688(%r14), %eax
	movl	%eax, 976(%rsp)
	movq	2672(%r14), %r12
	leaq	1328(%r14), %rax
	movq	%rax, 32(%rsp)
	leaq	96(%r14), %rax
	movq	%rax, 16(%rsp)
	leaq	1424(%r14), %rax
	movq	%rax, 80(%rsp)
	leaq	160(%r14), %rax
	movq	%rax, 64(%rsp)
	leaq	1488(%r14), %rax
	movq	%rax, 48(%rsp)
	xorl	%ebp, %ebp
	xorl	%r13d, %r13d
	jmp	.LBB34_113
	.p2align	4
.LBB34_111:
.Ltmp3073:
	.loc	1 1057 5 is_stmt 1
	vmovups	2000(%rsp), %ymm0
	vmovups	2032(%rsp), %ymm1
	movq	16(%rsp), %rax
	vmovups	%ymm1, 32(%rax)
	vmovups	%ymm0, (%rax)
	.loc	1 1058 5
	vmovups	2064(%rsp), %ymm0
	vmovups	2096(%rsp), %ymm1
	movq	80(%rsp), %rax
	vmovups	%ymm1, 32(%rax)
	vmovups	%ymm0, (%rax)
	.loc	1 1059 5
	vmovups	2128(%rsp), %ymm0
	movq	64(%rsp), %rax
	vmovups	%ymm0, (%rax)
	.loc	1 1060 5
	vmovdqu	2160(%rsp), %ymm0
	movq	48(%rsp), %rax
	vmovdqu	%ymm0, (%rax)
	movq	8(%rsp), %r14
	.loc	1 1061 5
	movq	%r9, 2680(%r14)
	movq	928(%rsp), %rsi
.Ltmp3074:
.LBB34_112:
	.loc	1 0 5 is_stmt 0
	movq	144(%rsp), %r13
	.loc	1 1086 11 is_stmt 1
	cmpq	%rsi, %r13
	jae	.LBB34_419
.LBB34_113:
	.loc	1 1087 42
	subq	%r13, %rsi
	.loc	1 1087 29 is_stmt 0
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E12plan_segmentB5_
	movb	%dl, 128(%rsp)
	movq	%rax, %r15
.Ltmp3075:
	.loc	1 1176 33 is_stmt 1
	vmovss	192(%r14), %xmm0
.Ltmp3076:
	.loc	1 1089 28
	vmovss	%xmm0, 176(%rsp)
.Ltmp3077:
	.loc	1 1176 33
	vmovss	352(%r14), %xmm0
.Ltmp3078:
	.loc	1 1089 28
	vmovss	%xmm0, 180(%rsp)
.Ltmp3079:
	.loc	1 1176 33
	vmovss	512(%r14), %xmm0
.Ltmp3080:
	.loc	1 1089 28
	vmovss	%xmm0, 184(%rsp)
.Ltmp3081:
	.loc	1 1176 33
	vmovss	672(%r14), %xmm0
.Ltmp3082:
	.loc	1 1089 28
	vmovss	%xmm0, 188(%rsp)
.Ltmp3083:
	.loc	1 1176 33
	vmovss	208(%r14), %xmm0
.Ltmp3084:
	.loc	1 1089 28
	vmovss	%xmm0, 192(%rsp)
.Ltmp3085:
	.loc	1 1176 33
	vmovss	368(%r14), %xmm0
.Ltmp3086:
	.loc	1 1089 28
	vmovss	%xmm0, 196(%rsp)
.Ltmp3087:
	.loc	1 1176 33
	vmovss	528(%r14), %xmm0
.Ltmp3088:
	.loc	1 1089 28
	vmovss	%xmm0, 200(%rsp)
.Ltmp3089:
	.loc	1 1176 33
	vmovss	688(%r14), %xmm0
.Ltmp3090:
	.loc	1 1089 28
	vmovss	%xmm0, 204(%rsp)
.Ltmp3091:
	.loc	1 1176 33
	vmovss	224(%r14), %xmm0
.Ltmp3092:
	.loc	1 1089 28
	vmovss	%xmm0, 208(%rsp)
.Ltmp3093:
	.loc	1 1176 33
	vmovss	384(%r14), %xmm0
.Ltmp3094:
	.loc	1 1089 28
	vmovss	%xmm0, 212(%rsp)
.Ltmp3095:
	.loc	1 1176 33
	vmovss	544(%r14), %xmm0
.Ltmp3096:
	.loc	1 1089 28
	vmovss	%xmm0, 216(%rsp)
.Ltmp3097:
	.loc	1 1176 33
	vmovss	704(%r14), %xmm0
.Ltmp3098:
	.loc	1 1089 28
	vmovss	%xmm0, 220(%rsp)
.Ltmp3099:
	.loc	1 1176 33
	vmovss	240(%r14), %xmm0
.Ltmp3100:
	.loc	1 1089 28
	vmovss	%xmm0, 224(%rsp)
.Ltmp3101:
	.loc	1 1176 33
	vmovss	400(%r14), %xmm0
.Ltmp3102:
	.loc	1 1089 28
	vmovss	%xmm0, 228(%rsp)
.Ltmp3103:
	.loc	1 1176 33
	vmovss	560(%r14), %xmm0
.Ltmp3104:
	.loc	1 1089 28
	vmovss	%xmm0, 232(%rsp)
.Ltmp3105:
	.loc	1 1176 33
	vmovss	720(%r14), %xmm0
.Ltmp3106:
	.loc	1 1089 28
	vmovss	%xmm0, 236(%rsp)
.Ltmp3107:
	.loc	1 1176 33
	vmovss	256(%r14), %xmm0
.Ltmp3108:
	.loc	1 1089 28
	vmovss	%xmm0, 240(%rsp)
.Ltmp3109:
	.loc	1 1176 33
	vmovss	416(%r14), %xmm0
.Ltmp3110:
	.loc	1 1089 28
	vmovss	%xmm0, 244(%rsp)
.Ltmp3111:
	.loc	1 1176 33
	vmovss	576(%r14), %xmm0
.Ltmp3112:
	.loc	1 1089 28
	vmovss	%xmm0, 248(%rsp)
.Ltmp3113:
	.loc	1 1176 33
	vmovss	736(%r14), %xmm0
.Ltmp3114:
	.loc	1 1089 28
	vmovss	%xmm0, 252(%rsp)
.Ltmp3115:
	.loc	1 1176 33
	vmovss	272(%r14), %xmm0
.Ltmp3116:
	.loc	1 1089 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp3117:
	.loc	1 1176 33
	vmovss	432(%r14), %xmm0
.Ltmp3118:
	.loc	1 1089 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp3119:
	.loc	1 1176 33
	vmovss	592(%r14), %xmm0
.Ltmp3120:
	.loc	1 1089 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp3121:
	.loc	1 1176 33
	vmovss	752(%r14), %xmm0
.Ltmp3122:
	.loc	1 1089 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp3123:
	.loc	1 1176 33
	vmovss	288(%r14), %xmm0
.Ltmp3124:
	.loc	1 1089 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp3125:
	.loc	1 1176 33
	vmovss	448(%r14), %xmm0
.Ltmp3126:
	.loc	1 1089 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp3127:
	.loc	1 1176 33
	vmovss	608(%r14), %xmm0
.Ltmp3128:
	.loc	1 1089 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp3129:
	.loc	1 1176 33
	vmovss	768(%r14), %xmm0
.Ltmp3130:
	.loc	1 1089 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp3131:
	.loc	1 1176 33
	vmovss	304(%r14), %xmm0
.Ltmp3132:
	.loc	1 1089 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp3133:
	.loc	1 1176 33
	vmovss	464(%r14), %xmm0
.Ltmp3134:
	.loc	1 1089 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp3135:
	.loc	1 1176 33
	vmovss	624(%r14), %xmm0
.Ltmp3136:
	.loc	1 1089 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp3137:
	.loc	1 1176 33
	vmovss	784(%r14), %xmm0
.Ltmp3138:
	.loc	1 1089 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp3139:
	.loc	1 1176 33
	vmovss	320(%r14), %xmm0
.Ltmp3140:
	.loc	1 1089 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp3141:
	.loc	1 1176 33
	vmovss	480(%r14), %xmm0
.Ltmp3142:
	.loc	1 1089 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp3143:
	.loc	1 1176 33
	vmovss	640(%r14), %xmm0
.Ltmp3144:
	.loc	1 1089 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp3145:
	.loc	1 1176 33
	vmovss	800(%r14), %xmm0
.Ltmp3146:
	.loc	1 1089 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp3147:
	.loc	1 1176 33
	vmovss	336(%r14), %xmm0
.Ltmp3148:
	.loc	1 1089 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp3149:
	.loc	1 1176 33
	vmovss	496(%r14), %xmm0
.Ltmp3150:
	.loc	1 1089 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp3151:
	.loc	1 1176 33
	vmovss	656(%r14), %xmm0
.Ltmp3152:
	.loc	1 1089 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp3153:
	.loc	1 1176 33
	vmovss	816(%r14), %xmm0
.Ltmp3154:
	.loc	1 1089 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp3155:
	.loc	1 1177 32
	vmovss	200(%r14), %xmm0
.Ltmp3156:
	.loc	1 1089 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp3157:
	.loc	1 1177 32
	vmovss	360(%r14), %xmm0
.Ltmp3158:
	.loc	1 1089 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp3159:
	.loc	1 1177 32
	vmovss	520(%r14), %xmm0
.Ltmp3160:
	.loc	1 1089 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp3161:
	.loc	1 1177 32
	vmovss	680(%r14), %xmm0
.Ltmp3162:
	.loc	1 1089 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp3163:
	.loc	1 1177 32
	vmovss	216(%r14), %xmm0
.Ltmp3164:
	.loc	1 1089 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp3165:
	.loc	1 1177 32
	vmovss	376(%r14), %xmm0
.Ltmp3166:
	.loc	1 1089 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp3167:
	.loc	1 1177 32
	vmovss	536(%r14), %xmm0
.Ltmp3168:
	.loc	1 1089 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp3169:
	.loc	1 1177 32
	vmovss	696(%r14), %xmm0
.Ltmp3170:
	.loc	1 1089 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp3171:
	.loc	1 1177 32
	vmovss	232(%r14), %xmm0
.Ltmp3172:
	.loc	1 1089 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp3173:
	.loc	1 1177 32
	vmovss	392(%r14), %xmm0
.Ltmp3174:
	.loc	1 1089 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp3175:
	.loc	1 1177 32
	vmovss	552(%r14), %xmm0
.Ltmp3176:
	.loc	1 1089 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp3177:
	.loc	1 1177 32
	vmovss	712(%r14), %xmm0
.Ltmp3178:
	.loc	1 1089 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp3179:
	.loc	1 1177 32
	vmovss	248(%r14), %xmm0
.Ltmp3180:
	.loc	1 1089 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp3181:
	.loc	1 1177 32
	vmovss	408(%r14), %xmm0
.Ltmp3182:
	.loc	1 1089 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp3183:
	.loc	1 1177 32
	vmovss	568(%r14), %xmm0
.Ltmp3184:
	.loc	1 1089 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp3185:
	.loc	1 1177 32
	vmovss	728(%r14), %xmm0
.Ltmp3186:
	.loc	1 1089 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp3187:
	.loc	1 1177 32
	vmovss	264(%r14), %xmm0
.Ltmp3188:
	.loc	1 1089 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp3189:
	.loc	1 1177 32
	vmovss	424(%r14), %xmm0
.Ltmp3190:
	.loc	1 1089 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp3191:
	.loc	1 1177 32
	vmovss	584(%r14), %xmm0
.Ltmp3192:
	.loc	1 1089 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp3193:
	.loc	1 1177 32
	vmovss	744(%r14), %xmm0
.Ltmp3194:
	.loc	1 1089 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp3195:
	.loc	1 1177 32
	vmovss	280(%r14), %xmm0
.Ltmp3196:
	.loc	1 1089 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp3197:
	.loc	1 1177 32
	vmovss	440(%r14), %xmm0
.Ltmp3198:
	.loc	1 1089 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp3199:
	.loc	1 1177 32
	vmovss	600(%r14), %xmm0
.Ltmp3200:
	.loc	1 1089 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp3201:
	.loc	1 1177 32
	vmovss	760(%r14), %xmm0
.Ltmp3202:
	.loc	1 1089 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp3203:
	.loc	1 1177 32
	vmovss	296(%r14), %xmm0
.Ltmp3204:
	.loc	1 1089 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp3205:
	.loc	1 1177 32
	vmovss	456(%r14), %xmm0
.Ltmp3206:
	.loc	1 1089 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp3207:
	.loc	1 1177 32
	vmovss	616(%r14), %xmm0
.Ltmp3208:
	.loc	1 1089 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp3209:
	.loc	1 1177 32
	vmovss	776(%r14), %xmm0
.Ltmp3210:
	.loc	1 1089 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp3211:
	.loc	1 1177 32
	vmovss	312(%r14), %xmm0
.Ltmp3212:
	.loc	1 1089 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp3213:
	.loc	1 1177 32
	vmovss	472(%r14), %xmm0
.Ltmp3214:
	.loc	1 1089 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp3215:
	.loc	1 1177 32
	vmovss	632(%r14), %xmm0
.Ltmp3216:
	.loc	1 1089 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp3217:
	.loc	1 1177 32
	vmovss	792(%r14), %xmm0
.Ltmp3218:
	.loc	1 1089 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp3219:
	.loc	1 1177 32
	vmovss	328(%r14), %xmm0
.Ltmp3220:
	.loc	1 1089 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp3221:
	.loc	1 1177 32
	vmovss	488(%r14), %xmm0
.Ltmp3222:
	.loc	1 1089 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp3223:
	.loc	1 1177 32
	vmovss	648(%r14), %xmm0
.Ltmp3224:
	.loc	1 1089 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp3225:
	.loc	1 1177 32
	vmovss	808(%r14), %xmm0
.Ltmp3226:
	.loc	1 1089 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp3227:
	.loc	1 1177 32
	vmovss	344(%r14), %xmm0
.Ltmp3228:
	.loc	1 1089 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp3229:
	.loc	1 1177 32
	vmovss	504(%r14), %xmm0
.Ltmp3230:
	.loc	1 1089 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp3231:
	.loc	1 1177 32
	vmovss	664(%r14), %xmm0
.Ltmp3232:
	.loc	1 1089 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp3233:
	.loc	1 1177 32
	vmovss	824(%r14), %xmm0
.Ltmp3234:
	.loc	1 1089 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp3235:
	.loc	1 1176 33
	vmovss	1520(%r14), %xmm0
.Ltmp3236:
	.loc	1 1089 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp3237:
	.loc	1 1176 33
	vmovss	1680(%r14), %xmm0
.Ltmp3238:
	.loc	1 1089 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp3239:
	.loc	1 1176 33
	vmovss	1840(%r14), %xmm0
.Ltmp3240:
	.loc	1 1089 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp3241:
	.loc	1 1176 33
	vmovss	2000(%r14), %xmm0
.Ltmp3242:
	.loc	1 1089 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp3243:
	.loc	1 1176 33
	vmovss	1536(%r14), %xmm0
.Ltmp3244:
	.loc	1 1089 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp3245:
	.loc	1 1176 33
	vmovss	1696(%r14), %xmm0
.Ltmp3246:
	.loc	1 1089 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp3247:
	.loc	1 1176 33
	vmovss	1856(%r14), %xmm0
.Ltmp3248:
	.loc	1 1089 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp3249:
	.loc	1 1176 33
	vmovss	2016(%r14), %xmm0
.Ltmp3250:
	.loc	1 1089 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp3251:
	.loc	1 1176 33
	vmovss	1552(%r14), %xmm0
.Ltmp3252:
	.loc	1 1089 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp3253:
	.loc	1 1176 33
	vmovss	1712(%r14), %xmm0
.Ltmp3254:
	.loc	1 1089 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp3255:
	.loc	1 1176 33
	vmovss	1872(%r14), %xmm0
.Ltmp3256:
	.loc	1 1089 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp3257:
	.loc	1 1176 33
	vmovss	2032(%r14), %xmm0
.Ltmp3258:
	.loc	1 1089 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp3259:
	.loc	1 1176 33
	vmovss	1568(%r14), %xmm0
.Ltmp3260:
	.loc	1 1089 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp3261:
	.loc	1 1176 33
	vmovss	1728(%r14), %xmm0
.Ltmp3262:
	.loc	1 1089 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp3263:
	.loc	1 1176 33
	vmovss	1888(%r14), %xmm0
.Ltmp3264:
	.loc	1 1089 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp3265:
	.loc	1 1176 33
	vmovss	2048(%r14), %xmm0
.Ltmp3266:
	.loc	1 1089 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp3267:
	.loc	1 1176 33
	vmovss	1584(%r14), %xmm0
.Ltmp3268:
	.loc	1 1089 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp3269:
	.loc	1 1176 33
	vmovss	1744(%r14), %xmm0
.Ltmp3270:
	.loc	1 1089 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp3271:
	.loc	1 1176 33
	vmovss	1904(%r14), %xmm0
.Ltmp3272:
	.loc	1 1089 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp3273:
	.loc	1 1176 33
	vmovss	2064(%r14), %xmm0
.Ltmp3274:
	.loc	1 1089 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp3275:
	.loc	1 1176 33
	vmovss	1600(%r14), %xmm0
.Ltmp3276:
	.loc	1 1089 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp3277:
	.loc	1 1176 33
	vmovss	1760(%r14), %xmm0
.Ltmp3278:
	.loc	1 1089 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp3279:
	.loc	1 1176 33
	vmovss	1920(%r14), %xmm0
.Ltmp3280:
	.loc	1 1089 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp3281:
	.loc	1 1176 33
	vmovss	2080(%r14), %xmm0
.Ltmp3282:
	.loc	1 1089 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp3283:
	.loc	1 1176 33
	vmovss	1616(%r14), %xmm0
.Ltmp3284:
	.loc	1 1089 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp3285:
	.loc	1 1176 33
	vmovss	1776(%r14), %xmm0
.Ltmp3286:
	.loc	1 1089 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp3287:
	.loc	1 1176 33
	vmovss	1936(%r14), %xmm0
.Ltmp3288:
	.loc	1 1089 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp3289:
	.loc	1 1176 33
	vmovss	2096(%r14), %xmm0
.Ltmp3290:
	.loc	1 1089 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp3291:
	.loc	1 1176 33
	vmovss	1632(%r14), %xmm0
.Ltmp3292:
	.loc	1 1089 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp3293:
	.loc	1 1176 33
	vmovss	1792(%r14), %xmm0
.Ltmp3294:
	.loc	1 1089 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp3295:
	.loc	1 1176 33
	vmovss	1952(%r14), %xmm0
.Ltmp3296:
	.loc	1 1089 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp3297:
	.loc	1 1176 33
	vmovss	2112(%r14), %xmm0
.Ltmp3298:
	.loc	1 1089 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp3299:
	.loc	1 1176 33
	vmovss	1648(%r14), %xmm0
.Ltmp3300:
	.loc	1 1089 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp3301:
	.loc	1 1176 33
	vmovss	1808(%r14), %xmm0
.Ltmp3302:
	.loc	1 1089 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp3303:
	.loc	1 1176 33
	vmovss	1968(%r14), %xmm0
.Ltmp3304:
	.loc	1 1089 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp3305:
	.loc	1 1176 33
	vmovss	2128(%r14), %xmm0
.Ltmp3306:
	.loc	1 1089 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp3307:
	.loc	1 1176 33
	vmovss	1664(%r14), %xmm0
.Ltmp3308:
	.loc	1 1089 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp3309:
	.loc	1 1176 33
	vmovss	1824(%r14), %xmm0
.Ltmp3310:
	.loc	1 1089 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp3311:
	.loc	1 1176 33
	vmovss	1984(%r14), %xmm0
.Ltmp3312:
	.loc	1 1089 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp3313:
	.loc	1 1176 33
	vmovss	2144(%r14), %xmm0
.Ltmp3314:
	.loc	1 1089 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp3315:
	.loc	1 1177 32
	vmovss	1528(%r14), %xmm0
.Ltmp3316:
	.loc	1 1089 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp3317:
	.loc	1 1177 32
	vmovss	1688(%r14), %xmm0
.Ltmp3318:
	.loc	1 1089 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp3319:
	.loc	1 1177 32
	vmovss	1848(%r14), %xmm0
.Ltmp3320:
	.loc	1 1089 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp3321:
	.loc	1 1177 32
	vmovss	2008(%r14), %xmm0
.Ltmp3322:
	.loc	1 1089 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp3323:
	.loc	1 1177 32
	vmovss	1544(%r14), %xmm0
.Ltmp3324:
	.loc	1 1089 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp3325:
	.loc	1 1177 32
	vmovss	1704(%r14), %xmm0
.Ltmp3326:
	.loc	1 1089 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp3327:
	.loc	1 1177 32
	vmovss	1864(%r14), %xmm0
.Ltmp3328:
	.loc	1 1089 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp3329:
	.loc	1 1177 32
	vmovss	2024(%r14), %xmm0
.Ltmp3330:
	.loc	1 1089 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp3331:
	.loc	1 1177 32
	vmovss	1560(%r14), %xmm0
.Ltmp3332:
	.loc	1 1089 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp3333:
	.loc	1 1177 32
	vmovss	1720(%r14), %xmm0
.Ltmp3334:
	.loc	1 1089 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp3335:
	.loc	1 1177 32
	vmovss	1880(%r14), %xmm0
.Ltmp3336:
	.loc	1 1089 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp3337:
	.loc	1 1177 32
	vmovss	2040(%r14), %xmm0
.Ltmp3338:
	.loc	1 1089 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp3339:
	.loc	1 1177 32
	vmovss	1576(%r14), %xmm0
.Ltmp3340:
	.loc	1 1089 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp3341:
	.loc	1 1177 32
	vmovss	1736(%r14), %xmm0
.Ltmp3342:
	.loc	1 1089 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp3343:
	.loc	1 1177 32
	vmovss	1896(%r14), %xmm0
.Ltmp3344:
	.loc	1 1089 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp3345:
	.loc	1 1177 32
	vmovss	2056(%r14), %xmm0
.Ltmp3346:
	.loc	1 1089 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp3347:
	.loc	1 1177 32
	vmovss	1592(%r14), %xmm0
.Ltmp3348:
	.loc	1 1089 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp3349:
	.loc	1 1177 32
	vmovss	1752(%r14), %xmm0
.Ltmp3350:
	.loc	1 1089 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp3351:
	.loc	1 1177 32
	vmovss	1912(%r14), %xmm0
.Ltmp3352:
	.loc	1 1089 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp3353:
	.loc	1 1177 32
	vmovss	2072(%r14), %xmm0
.Ltmp3354:
	.loc	1 1089 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp3355:
	.loc	1 1177 32
	vmovss	1608(%r14), %xmm0
.Ltmp3356:
	.loc	1 1089 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp3357:
	.loc	1 1177 32
	vmovss	1768(%r14), %xmm0
.Ltmp3358:
	.loc	1 1089 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp3359:
	.loc	1 1177 32
	vmovss	1928(%r14), %xmm0
.Ltmp3360:
	.loc	1 1089 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp3361:
	.loc	1 1177 32
	vmovss	2088(%r14), %xmm0
.Ltmp3362:
	.loc	1 1089 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp3363:
	.loc	1 1177 32
	vmovss	1624(%r14), %xmm0
.Ltmp3364:
	.loc	1 1089 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp3365:
	.loc	1 1177 32
	vmovss	1784(%r14), %xmm0
.Ltmp3366:
	.loc	1 1089 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp3367:
	.loc	1 1177 32
	vmovss	1944(%r14), %xmm0
.Ltmp3368:
	.loc	1 1089 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp3369:
	.loc	1 1177 32
	vmovss	2104(%r14), %xmm0
.Ltmp3370:
	.loc	1 1089 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp3371:
	.loc	1 1177 32
	vmovss	1640(%r14), %xmm0
.Ltmp3372:
	.loc	1 1089 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp3373:
	.loc	1 1177 32
	vmovss	1800(%r14), %xmm0
.Ltmp3374:
	.loc	1 1089 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp3375:
	.loc	1 1177 32
	vmovss	1960(%r14), %xmm0
.Ltmp3376:
	.loc	1 1089 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp3377:
	.loc	1 1177 32
	vmovss	2120(%r14), %xmm0
.Ltmp3378:
	.loc	1 1089 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp3379:
	.loc	1 1177 32
	vmovss	1656(%r14), %xmm0
.Ltmp3380:
	.loc	1 1089 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp3381:
	.loc	1 1177 32
	vmovss	1816(%r14), %xmm0
.Ltmp3382:
	.loc	1 1089 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp3383:
	.loc	1 1177 32
	vmovss	1976(%r14), %xmm0
.Ltmp3384:
	.loc	1 1089 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp3385:
	.loc	1 1177 32
	vmovss	2136(%r14), %xmm0
.Ltmp3386:
	.loc	1 1089 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp3387:
	.loc	1 1177 32
	vmovss	1672(%r14), %xmm0
.Ltmp3388:
	.loc	1 1089 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp3389:
	.loc	1 1177 32
	vmovss	1832(%r14), %xmm0
.Ltmp3390:
	.loc	1 1089 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp3391:
	.loc	1 1177 32
	vmovss	1992(%r14), %xmm0
.Ltmp3392:
	.loc	1 1089 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp3393:
	.loc	1 1177 32
	vmovss	2152(%r14), %xmm0
.Ltmp3394:
	.loc	1 1089 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp3395:
	.loc	1 1091 31
	leaq	1248(%rsp), %rdi
	movq	%r14, %rsi
	movl	976(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	1344(%rsp), %rdi
	movq	32(%rsp), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
.Ltmp3396:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%r13), %rax
	shlq	$2, %r13
	leaq	(,%rax,4), %rsi
	.loc	1 1094 12 is_stmt 1
	testb	$1, 128(%rsp)
	movq	%rax, 144(%rsp)
	je	.LBB34_137
.Ltmp3397:
	.loc	38 1050 16
	cmpq	%r13, %rsi
.Ltmp3398:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_479
.Ltmp3399:
	.loc	48 451 16 is_stmt 1
	cmpq	120(%rsp), %rsi
	ja	.LBB34_479
.Ltmp3400:
	.loc	48 451 16 is_stmt 0
	cmpq	112(%rsp), %rsi
	ja	.LBB34_483
.Ltmp3401:
	.loc	48 0 16
	movq	16(%rsp), %rax
.Ltmp3402:
	.loc	1 972 27 is_stmt 1
	vmovups	(%rax), %ymm0
	vmovups	32(%rax), %ymm1
	vmovups	%ymm1, 1840(%rsp)
	vmovups	%ymm0, 1808(%rsp)
	movq	80(%rsp), %rax
.Ltmp3403:
	.loc	1 973 26
	vmovups	(%rax), %ymm0
	vmovups	32(%rax), %ymm1
	vmovups	%ymm1, 1904(%rsp)
	vmovups	%ymm0, 1872(%rsp)
	movq	64(%rsp), %rax
.Ltmp3404:
	.loc	1 974 25
	vmovups	(%rax), %ymm0
	vmovups	%ymm0, 1936(%rsp)
	movq	48(%rsp), %rax
.Ltmp3405:
	.loc	1 975 24
	vmovdqu	(%rax), %ymm0
	vmovdqu	%ymm0, 1968(%rsp)
.Ltmp3406:
	.loc	1 976 24
	movq	2680(%r14), %r9
.Ltmp3407:
	.loc	2 1916 50
	testq	%r15, %r15
	je	.LBB34_133
.Ltmp3408:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,4), %rdx
	movq	920(%rsp), %rax
	leaq	(%rax,%r13,4), %r10
	movq	912(%rsp), %rax
	leaq	(%rax,%r13,4), %r11
.Ltmp3409:
	.loc	3 900 12 is_stmt 1
	movq	%r15, %r14
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	xorl	%edi, %edi
	xorl	%r13d, %r13d
.Ltmp3410:
	.loc	3 0 12 is_stmt 0
.Ltmp3411:
	.p2align	4
.LBB34_119:
	.loc	1 981 21 is_stmt 1
	vmovaps	176(%rsp), %xmm0
	vmovaps	192(%rsp), %xmm1
	vmovaps	208(%rsp), %xmm2
.Ltmp3412:
	.loc	9 36 14
	vaddps	336(%rsp), %xmm0, %xmm0
.Ltmp3413:
	.loc	1 983 21
	vmovaps	496(%rsp), %xmm3
	.loc	1 980 17
	vmovaps	%xmm0, 176(%rsp)
.Ltmp3414:
	.loc	9 36 14
	vaddps	656(%rsp), %xmm3, %xmm0
.Ltmp3415:
	.loc	1 982 17
	vmovaps	%xmm0, 496(%rsp)
.Ltmp3416:
	.loc	9 36 14
	vaddps	352(%rsp), %xmm1, %xmm0
.Ltmp3417:
	.loc	1 980 17
	vmovaps	%xmm0, 192(%rsp)
	.loc	1 983 21
	vmovaps	512(%rsp), %xmm0
.Ltmp3418:
	.loc	9 36 14
	vaddps	672(%rsp), %xmm0, %xmm0
.Ltmp3419:
	.loc	1 982 17
	vmovaps	%xmm0, 512(%rsp)
.Ltmp3420:
	.loc	9 36 14
	vaddps	368(%rsp), %xmm2, %xmm0
.Ltmp3421:
	.loc	1 980 17
	vmovaps	%xmm0, 208(%rsp)
	.loc	1 983 21
	vmovaps	528(%rsp), %xmm0
.Ltmp3422:
	.loc	9 36 14
	vaddps	688(%rsp), %xmm0, %xmm0
.Ltmp3423:
	.loc	1 982 17
	vmovaps	%xmm0, 528(%rsp)
	.loc	1 981 21
	vmovaps	224(%rsp), %xmm0
.Ltmp3424:
	.loc	9 36 14
	vaddps	384(%rsp), %xmm0, %xmm0
.Ltmp3425:
	.loc	1 980 17
	vmovaps	%xmm0, 224(%rsp)
	.loc	1 983 21
	vmovaps	544(%rsp), %xmm0
.Ltmp3426:
	.loc	9 36 14
	vaddps	704(%rsp), %xmm0, %xmm0
.Ltmp3427:
	.loc	1 982 17
	vmovaps	%xmm0, 544(%rsp)
	.loc	1 981 21
	vmovaps	240(%rsp), %xmm0
.Ltmp3428:
	.loc	9 36 14
	vaddps	400(%rsp), %xmm0, %xmm0
.Ltmp3429:
	.loc	1 980 17
	vmovaps	%xmm0, 240(%rsp)
	.loc	1 983 21
	vmovaps	560(%rsp), %xmm0
.Ltmp3430:
	.loc	9 36 14
	vaddps	720(%rsp), %xmm0, %xmm0
.Ltmp3431:
	.loc	1 982 17
	vmovaps	%xmm0, 560(%rsp)
	.loc	1 981 21
	vmovaps	256(%rsp), %xmm0
.Ltmp3432:
	.loc	9 36 14
	vaddps	416(%rsp), %xmm0, %xmm0
.Ltmp3433:
	.loc	1 980 17
	vmovaps	%xmm0, 256(%rsp)
	.loc	1 983 21
	vmovaps	576(%rsp), %xmm0
.Ltmp3434:
	.loc	9 36 14
	vaddps	736(%rsp), %xmm0, %xmm0
.Ltmp3435:
	.loc	1 982 17
	vmovaps	%xmm0, 576(%rsp)
	.loc	1 981 21
	vmovaps	272(%rsp), %xmm0
.Ltmp3436:
	.loc	9 36 14
	vaddps	432(%rsp), %xmm0, %xmm0
.Ltmp3437:
	.loc	1 980 17
	vmovaps	%xmm0, 272(%rsp)
	.loc	1 983 21
	vmovaps	592(%rsp), %xmm0
.Ltmp3438:
	.loc	9 36 14
	vaddps	752(%rsp), %xmm0, %xmm0
.Ltmp3439:
	.loc	1 982 17
	vmovaps	%xmm0, 592(%rsp)
	.loc	1 981 21
	vmovaps	288(%rsp), %xmm0
.Ltmp3440:
	.loc	9 36 14
	vaddps	448(%rsp), %xmm0, %xmm0
.Ltmp3441:
	.loc	1 980 17
	vmovaps	%xmm0, 288(%rsp)
	.loc	1 983 21
	vmovaps	608(%rsp), %xmm0
.Ltmp3442:
	.loc	9 36 14
	vaddps	768(%rsp), %xmm0, %xmm0
.Ltmp3443:
	.loc	1 982 17
	vmovaps	%xmm0, 608(%rsp)
	.loc	1 981 21
	vmovaps	304(%rsp), %xmm0
.Ltmp3444:
	.loc	9 36 14
	vaddps	464(%rsp), %xmm0, %xmm0
.Ltmp3445:
	.loc	1 980 17
	vmovaps	%xmm0, 304(%rsp)
	.loc	1 983 21
	vmovaps	624(%rsp), %xmm0
.Ltmp3446:
	.loc	9 36 14
	vaddps	784(%rsp), %xmm0, %xmm0
.Ltmp3447:
	.loc	1 982 17
	vmovaps	%xmm0, 624(%rsp)
	.loc	1 981 21
	vmovaps	320(%rsp), %xmm0
.Ltmp3448:
	.loc	9 36 14
	vaddps	480(%rsp), %xmm0, %xmm0
.Ltmp3449:
	.loc	1 980 17
	vmovaps	%xmm0, 320(%rsp)
	.loc	1 983 21
	vmovaps	640(%rsp), %xmm0
.Ltmp3450:
	.loc	9 36 14
	vaddps	800(%rsp), %xmm0, %xmm0
.Ltmp3451:
	.loc	1 982 17
	vmovaps	%xmm0, 640(%rsp)
.Ltmp3452:
	.loc	1 987 28
	leaq	1(%r9), %rax
.Ltmp3453:
	.loc	1 857 8
	cmpq	%r12, %rax
	jb	.LBB34_121
.Ltmp3454:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rbx
	jmp	.LBB34_122
	.p2align	4
.LBB34_121:
	xorl	%ebx, %ebx
.LBB34_122:
.Ltmp3455:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB34_451
.Ltmp3456:
	.loc	48 438 16
	cmpq	%r13, %r14
	je	.LBB34_449
.Ltmp3457:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r9,4), %rax
.Ltmp3458:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%r11,%rdi,4), %xmm0
	vmovdqa	%xmm0, 1440(%rsp)
	movq	8(%rsp), %rcx
.Ltmp3459:
	.loc	1 991 35
	movq	8(%rcx), %rsi
.Ltmp3460:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_452
.Ltmp3461:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp3462:
	.loc	1 0 0 is_stmt 0
	leaq	(%r10,%rdi,4), %rcx
	movq	8(%rsp), %r8
.Ltmp3463:
	.loc	1 991 35 is_stmt 1
	movq	(%r8), %rsi
.Ltmp3464:
	.loc	8 551 14
	vmovdqu	(%rcx), %xmm0
	vmovdqu	%xmm0, (%rsi,%rax,4)
.Ltmp3465:
	.loc	1 992 34
	movq	1336(%r8), %rsi
.Ltmp3466:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_453
.Ltmp3467:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp3468:
	.loc	1 0 0 is_stmt 0
	negq	%rbx
	addq	%rbx, %r9
	incq	%r9
	leaq	(,%r9,4), %r8
	movq	8(%rsp), %rbx
.Ltmp3469:
	.loc	1 992 34 is_stmt 1
	movq	1328(%rbx), %rsi
.Ltmp3470:
	.loc	8 551 14
	vmovaps	1440(%rsp), %xmm0
	vmovups	%xmm0, (%rsi,%rax,4)
.Ltmp3471:
	.loc	1 993 22
	movq	8(%rbx), %rsi
.Ltmp3472:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB34_454
.Ltmp3473:
	.loc	48 438 16
	cmpq	$3, %rax
	jbe	.LBB34_448
.Ltmp3474:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 993 22 is_stmt 1
	movq	(%rsi), %rax
.Ltmp3475:
	.loc	8 551 14
	vmovups	(%rax,%r8,4), %xmm0
	vmovups	%xmm0, (%rcx)
.Ltmp3476:
	.loc	1 994 22
	movq	1336(%rsi), %rsi
.Ltmp3477:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB34_455
.Ltmp3478:
	.loc	48 438 16
	cmpq	$3, %rax
	jbe	.LBB34_448
.Ltmp3479:
	.loc	1 0 0 is_stmt 0
	incq	%r13
.Ltmp3480:
	leaq	(%r11,%rdi,4), %rax
	movq	32(%rsp), %rcx
.Ltmp3481:
	.loc	1 994 22 is_stmt 1
	movq	(%rcx), %rcx
.Ltmp3482:
	.loc	8 551 14
	vmovdqu	(%rcx,%r8,4), %xmm0
	vmovdqu	%xmm0, (%rax)
.Ltmp3483:
	.loc	2 1916 50
	addq	$4, %rdi
	cmpq	%r13, %r15
.Ltmp3484:
	.loc	3 900 12
	jne	.LBB34_119
.Ltmp3485:
.LBB34_133:
	.loc	1 1057 5
	vmovups	1808(%rsp), %ymm0
	vmovups	1840(%rsp), %ymm1
	movq	16(%rsp), %rax
	vmovups	%ymm1, 32(%rax)
	vmovups	%ymm0, (%rax)
	.loc	1 1058 5
	vmovups	1872(%rsp), %ymm0
	vmovups	1904(%rsp), %ymm1
	movq	80(%rsp), %rax
	vmovups	%ymm1, 32(%rax)
	vmovups	%ymm0, (%rax)
	.loc	1 1059 5
	vmovups	1936(%rsp), %ymm0
	movq	64(%rsp), %rax
	vmovups	%ymm0, (%rax)
	.loc	1 1060 5
	vmovups	1968(%rsp), %ymm0
	movq	48(%rsp), %rax
	vmovups	%ymm0, (%rax)
	movq	8(%rsp), %r14
	.loc	1 1061 5
	movq	%r9, 2680(%r14)
	xorl	%eax, %eax
.Ltmp3486:
	.loc	1 0 5 is_stmt 0
.Ltmp3487:
	.p2align	4
.LBB34_134:
	.loc	1 1194 13 is_stmt 1
	vmovd	176(%rsp,%rax), %xmm0
	vmovss	180(%rsp,%rax), %xmm1
	vmovss	184(%rsp,%rax), %xmm2
	vmovss	188(%rsp,%rax), %xmm3
.Ltmp3488:
	.loc	1 1197 17
	vmovd	%xmm0, 192(%r14,%rax)
	.loc	1 1198 34
	movl	204(%r14,%rax), %ecx
	movl	364(%r14,%rax), %edx
.Ltmp3489:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3490:
	.loc	1 1198 17
	movl	%ecx, 204(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 352(%r14,%rax)
.Ltmp3491:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%ebp, %edx
.Ltmp3492:
	.loc	1 1198 17
	movl	%edx, 364(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 512(%r14,%rax)
	.loc	1 1198 34
	movl	524(%r14,%rax), %ecx
.Ltmp3493:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3494:
	.loc	1 1198 17
	movl	%ecx, 524(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm3, 672(%r14,%rax)
	.loc	1 1198 34
	movl	684(%r14,%rax), %ecx
.Ltmp3495:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3496:
	.loc	1 1198 17
	movl	%ecx, 684(%r14,%rax)
.Ltmp3497:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp3498:
	.loc	3 900 12
	jne	.LBB34_134
.Ltmp3499:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	928(%rsp), %rsi
	.p2align	4
.LBB34_136:
.Ltmp3500:
	.loc	1 1194 13 is_stmt 1
	vmovd	496(%rsp,%rax), %xmm0
	vmovss	500(%rsp,%rax), %xmm1
	vmovss	504(%rsp,%rax), %xmm2
	vmovss	508(%rsp,%rax), %xmm3
.Ltmp3501:
	.loc	1 1197 17
	vmovd	%xmm0, 1520(%r14,%rax)
	.loc	1 1198 34
	movl	1532(%r14,%rax), %ecx
	movl	1692(%r14,%rax), %edx
.Ltmp3502:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3503:
	.loc	1 1198 17
	movl	%ecx, 1532(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 1680(%r14,%rax)
.Ltmp3504:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%ebp, %edx
.Ltmp3505:
	.loc	1 1198 17
	movl	%edx, 1692(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 1840(%r14,%rax)
	.loc	1 1198 34
	movl	1852(%r14,%rax), %ecx
.Ltmp3506:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3507:
	.loc	1 1198 17
	movl	%ecx, 1852(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm3, 2000(%r14,%rax)
	.loc	1 1198 34
	movl	2012(%r14,%rax), %ecx
.Ltmp3508:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3509:
	.loc	1 1198 17
	movl	%ecx, 2012(%r14,%rax)
.Ltmp3510:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp3511:
	.loc	3 900 12
	jne	.LBB34_136
	jmp	.LBB34_112
.Ltmp3512:
	.loc	3 0 12 is_stmt 0
.Ltmp3513:
	.p2align	4
.LBB34_137:
	.loc	38 1050 16 is_stmt 1
	cmpq	%r13, %rsi
.Ltmp3514:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_478
.Ltmp3515:
	.loc	48 451 16 is_stmt 1
	cmpq	120(%rsp), %rsi
	ja	.LBB34_478
.Ltmp3516:
	.loc	48 451 16 is_stmt 0
	cmpq	112(%rsp), %rsi
	ja	.LBB34_482
.Ltmp3517:
	.loc	48 0 16
	movq	16(%rsp), %rax
.Ltmp3518:
	.loc	1 972 27 is_stmt 1
	vmovups	(%rax), %ymm0
	vmovups	32(%rax), %ymm1
	vmovups	%ymm1, 2032(%rsp)
	vmovups	%ymm0, 2000(%rsp)
	movq	80(%rsp), %rax
.Ltmp3519:
	.loc	1 973 26
	vmovups	(%rax), %ymm0
	vmovups	32(%rax), %ymm1
	vmovups	%ymm1, 2096(%rsp)
	vmovups	%ymm0, 2064(%rsp)
	movq	64(%rsp), %rax
.Ltmp3520:
	.loc	1 974 25
	vmovups	(%rax), %ymm0
	vmovups	%ymm0, 2128(%rsp)
	movq	48(%rsp), %rax
.Ltmp3521:
	.loc	1 975 24
	vmovdqu	(%rax), %ymm0
	vmovdqu	%ymm0, 2160(%rsp)
.Ltmp3522:
	.loc	1 976 24
	movq	2680(%r14), %r9
.Ltmp3523:
	.loc	2 1916 50
	testq	%r15, %r15
	je	.LBB34_111
.Ltmp3524:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,4), %rdx
	movq	920(%rsp), %rax
	leaq	(%rax,%r13,4), %r10
	movq	912(%rsp), %rax
	leaq	(%rax,%r13,4), %r11
.Ltmp3525:
	.loc	48 568 12 is_stmt 1
	movq	%r15, %r14
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	xorl	%edi, %edi
	xorl	%r13d, %r13d
.Ltmp3526:
	.loc	48 0 12 is_stmt 0
.Ltmp3527:
	.p2align	4
.LBB34_142:
	.loc	1 987 28 is_stmt 1
	leaq	1(%r9), %rax
.Ltmp3528:
	.loc	1 857 8
	cmpq	%r12, %rax
	jb	.LBB34_144
.Ltmp3529:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rbx
	jmp	.LBB34_145
	.p2align	4
.LBB34_144:
	xorl	%ebx, %ebx
.LBB34_145:
.Ltmp3530:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB34_451
.Ltmp3531:
	.loc	48 438 16
	cmpq	%r13, %r14
	je	.LBB34_449
.Ltmp3532:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r9,4), %rax
.Ltmp3533:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%r11,%rdi,4), %xmm0
	vmovdqa	%xmm0, 1456(%rsp)
	movq	8(%rsp), %rcx
.Ltmp3534:
	.loc	1 991 35
	movq	8(%rcx), %rsi
.Ltmp3535:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_452
.Ltmp3536:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp3537:
	.loc	1 0 0 is_stmt 0
	leaq	(%r10,%rdi,4), %rcx
	movq	8(%rsp), %r8
.Ltmp3538:
	.loc	1 991 35 is_stmt 1
	movq	(%r8), %rsi
.Ltmp3539:
	.loc	8 551 14
	vmovdqu	(%rcx), %xmm0
	vmovdqu	%xmm0, (%rsi,%rax,4)
.Ltmp3540:
	.loc	1 992 34
	movq	1336(%r8), %rsi
.Ltmp3541:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_453
.Ltmp3542:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp3543:
	.loc	1 0 0 is_stmt 0
	negq	%rbx
	addq	%rbx, %r9
	incq	%r9
	leaq	(,%r9,4), %r8
	movq	8(%rsp), %rbx
.Ltmp3544:
	.loc	1 992 34 is_stmt 1
	movq	1328(%rbx), %rsi
.Ltmp3545:
	.loc	8 551 14
	vmovaps	1456(%rsp), %xmm0
	vmovups	%xmm0, (%rsi,%rax,4)
.Ltmp3546:
	.loc	1 993 22
	movq	8(%rbx), %rsi
.Ltmp3547:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB34_454
.Ltmp3548:
	.loc	48 438 16
	cmpq	$3, %rax
	jbe	.LBB34_448
.Ltmp3549:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 993 22 is_stmt 1
	movq	(%rsi), %rax
.Ltmp3550:
	.loc	8 551 14
	vmovups	(%rax,%r8,4), %xmm0
	vmovups	%xmm0, (%rcx)
.Ltmp3551:
	.loc	1 994 22
	movq	1336(%rsi), %rsi
.Ltmp3552:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB34_455
.Ltmp3553:
	.loc	48 438 16
	cmpq	$3, %rax
	jbe	.LBB34_448
.Ltmp3554:
	.loc	1 0 0 is_stmt 0
	incq	%r13
.Ltmp3555:
	leaq	(%r11,%rdi,4), %rax
	movq	32(%rsp), %rcx
.Ltmp3556:
	.loc	1 994 22 is_stmt 1
	movq	(%rcx), %rcx
.Ltmp3557:
	.loc	8 551 14
	vmovdqu	(%rcx,%r8,4), %xmm0
	vmovdqu	%xmm0, (%rax)
.Ltmp3558:
	.loc	2 1916 50
	addq	$4, %rdi
	cmpq	%r13, %r15
.Ltmp3559:
	.loc	3 900 12
	jne	.LBB34_142
	jmp	.LBB34_111
.Ltmp3560:
.LBB34_156:
	.loc	3 0 12 is_stmt 0
	vmovaps	80(%rsp), %xmm0
.Ltmp3561:
	.loc	1 1057 5 is_stmt 1
	vmovaps	%xmm0, 96(%r14)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 112(%r14)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 128(%r14)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 144(%r14)
	vmovaps	32(%rsp), %xmm0
	.loc	1 1058 5
	vmovaps	%xmm0, 1424(%r14)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%r14)
	vmovaps	864(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%r14)
	vmovaps	144(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%r14)
	vmovaps	848(%rsp), %xmm0
	.loc	1 1059 5
	vmovaps	%xmm0, 160(%r14)
	vmovaps	832(%rsp), %xmm0
	vmovaps	%xmm0, 176(%r14)
	vmovdqa	816(%rsp), %xmm0
	.loc	1 1060 5
	vmovdqa	%xmm0, 1488(%r14)
	vmovaps	%xmm6, 1504(%r14)
	.loc	1 1061 5
	movq	%rcx, 2680(%r14)
	movq	928(%rsp), %rsi
	movq	936(%rsp), %r15
.Ltmp3562:
.LBB34_157:
	.loc	1 1086 11
	cmpq	%rsi, %r15
	jae	.LBB34_419
.LBB34_158:
	.loc	1 1087 42
	subq	%r15, %rsi
	.loc	1 1087 29 is_stmt 0
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r13
.Ltmp3563:
	.loc	1 1176 33 is_stmt 1
	vmovss	192(%r14), %xmm0
.Ltmp3564:
	.loc	1 1089 28
	vmovss	%xmm0, 176(%rsp)
.Ltmp3565:
	.loc	1 1176 33
	vmovss	352(%r14), %xmm0
.Ltmp3566:
	.loc	1 1089 28
	vmovss	%xmm0, 180(%rsp)
.Ltmp3567:
	.loc	1 1176 33
	vmovss	512(%r14), %xmm0
.Ltmp3568:
	.loc	1 1089 28
	vmovss	%xmm0, 184(%rsp)
.Ltmp3569:
	.loc	1 1176 33
	vmovss	672(%r14), %xmm0
.Ltmp3570:
	.loc	1 1089 28
	vmovss	%xmm0, 188(%rsp)
.Ltmp3571:
	.loc	1 1176 33
	vmovss	208(%r14), %xmm0
.Ltmp3572:
	.loc	1 1089 28
	vmovss	%xmm0, 192(%rsp)
.Ltmp3573:
	.loc	1 1176 33
	vmovss	368(%r14), %xmm0
.Ltmp3574:
	.loc	1 1089 28
	vmovss	%xmm0, 196(%rsp)
.Ltmp3575:
	.loc	1 1176 33
	vmovss	528(%r14), %xmm0
.Ltmp3576:
	.loc	1 1089 28
	vmovss	%xmm0, 200(%rsp)
.Ltmp3577:
	.loc	1 1176 33
	vmovss	688(%r14), %xmm0
.Ltmp3578:
	.loc	1 1089 28
	vmovss	%xmm0, 204(%rsp)
.Ltmp3579:
	.loc	1 1176 33
	vmovss	224(%r14), %xmm0
.Ltmp3580:
	.loc	1 1089 28
	vmovss	%xmm0, 208(%rsp)
.Ltmp3581:
	.loc	1 1176 33
	vmovss	384(%r14), %xmm0
.Ltmp3582:
	.loc	1 1089 28
	vmovss	%xmm0, 212(%rsp)
.Ltmp3583:
	.loc	1 1176 33
	vmovss	544(%r14), %xmm0
.Ltmp3584:
	.loc	1 1089 28
	vmovss	%xmm0, 216(%rsp)
.Ltmp3585:
	.loc	1 1176 33
	vmovss	704(%r14), %xmm0
.Ltmp3586:
	.loc	1 1089 28
	vmovss	%xmm0, 220(%rsp)
.Ltmp3587:
	.loc	1 1176 33
	vmovss	240(%r14), %xmm0
.Ltmp3588:
	.loc	1 1089 28
	vmovss	%xmm0, 224(%rsp)
.Ltmp3589:
	.loc	1 1176 33
	vmovss	400(%r14), %xmm0
.Ltmp3590:
	.loc	1 1089 28
	vmovss	%xmm0, 228(%rsp)
.Ltmp3591:
	.loc	1 1176 33
	vmovss	560(%r14), %xmm0
.Ltmp3592:
	.loc	1 1089 28
	vmovss	%xmm0, 232(%rsp)
.Ltmp3593:
	.loc	1 1176 33
	vmovss	720(%r14), %xmm0
.Ltmp3594:
	.loc	1 1089 28
	vmovss	%xmm0, 236(%rsp)
.Ltmp3595:
	.loc	1 1176 33
	vmovss	256(%r14), %xmm0
.Ltmp3596:
	.loc	1 1089 28
	vmovss	%xmm0, 240(%rsp)
.Ltmp3597:
	.loc	1 1176 33
	vmovss	416(%r14), %xmm0
.Ltmp3598:
	.loc	1 1089 28
	vmovss	%xmm0, 244(%rsp)
.Ltmp3599:
	.loc	1 1176 33
	vmovss	576(%r14), %xmm0
.Ltmp3600:
	.loc	1 1089 28
	vmovss	%xmm0, 248(%rsp)
.Ltmp3601:
	.loc	1 1176 33
	vmovss	736(%r14), %xmm0
.Ltmp3602:
	.loc	1 1089 28
	vmovss	%xmm0, 252(%rsp)
.Ltmp3603:
	.loc	1 1176 33
	vmovss	272(%r14), %xmm0
.Ltmp3604:
	.loc	1 1089 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp3605:
	.loc	1 1176 33
	vmovss	432(%r14), %xmm0
.Ltmp3606:
	.loc	1 1089 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp3607:
	.loc	1 1176 33
	vmovss	592(%r14), %xmm0
.Ltmp3608:
	.loc	1 1089 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp3609:
	.loc	1 1176 33
	vmovss	752(%r14), %xmm0
.Ltmp3610:
	.loc	1 1089 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp3611:
	.loc	1 1176 33
	vmovss	288(%r14), %xmm0
.Ltmp3612:
	.loc	1 1089 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp3613:
	.loc	1 1176 33
	vmovss	448(%r14), %xmm0
.Ltmp3614:
	.loc	1 1089 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp3615:
	.loc	1 1176 33
	vmovss	608(%r14), %xmm0
.Ltmp3616:
	.loc	1 1089 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp3617:
	.loc	1 1176 33
	vmovss	768(%r14), %xmm0
.Ltmp3618:
	.loc	1 1089 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp3619:
	.loc	1 1176 33
	vmovss	304(%r14), %xmm0
.Ltmp3620:
	.loc	1 1089 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp3621:
	.loc	1 1176 33
	vmovss	464(%r14), %xmm0
.Ltmp3622:
	.loc	1 1089 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp3623:
	.loc	1 1176 33
	vmovss	624(%r14), %xmm0
.Ltmp3624:
	.loc	1 1089 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp3625:
	.loc	1 1176 33
	vmovss	784(%r14), %xmm0
.Ltmp3626:
	.loc	1 1089 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp3627:
	.loc	1 1176 33
	vmovss	320(%r14), %xmm0
.Ltmp3628:
	.loc	1 1089 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp3629:
	.loc	1 1176 33
	vmovss	480(%r14), %xmm0
.Ltmp3630:
	.loc	1 1089 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp3631:
	.loc	1 1176 33
	vmovss	640(%r14), %xmm0
.Ltmp3632:
	.loc	1 1089 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp3633:
	.loc	1 1176 33
	vmovss	800(%r14), %xmm0
.Ltmp3634:
	.loc	1 1089 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp3635:
	.loc	1 1176 33
	vmovss	336(%r14), %xmm0
.Ltmp3636:
	.loc	1 1089 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp3637:
	.loc	1 1176 33
	vmovss	496(%r14), %xmm0
.Ltmp3638:
	.loc	1 1089 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp3639:
	.loc	1 1176 33
	vmovss	656(%r14), %xmm0
.Ltmp3640:
	.loc	1 1089 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp3641:
	.loc	1 1176 33
	vmovss	816(%r14), %xmm0
.Ltmp3642:
	.loc	1 1089 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp3643:
	.loc	1 1177 32
	vmovss	200(%r14), %xmm0
.Ltmp3644:
	.loc	1 1089 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp3645:
	.loc	1 1177 32
	vmovss	360(%r14), %xmm0
.Ltmp3646:
	.loc	1 1089 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp3647:
	.loc	1 1177 32
	vmovss	520(%r14), %xmm0
.Ltmp3648:
	.loc	1 1089 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp3649:
	.loc	1 1177 32
	vmovss	680(%r14), %xmm0
.Ltmp3650:
	.loc	1 1089 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp3651:
	.loc	1 1177 32
	vmovss	216(%r14), %xmm0
.Ltmp3652:
	.loc	1 1089 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp3653:
	.loc	1 1177 32
	vmovss	376(%r14), %xmm0
.Ltmp3654:
	.loc	1 1089 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp3655:
	.loc	1 1177 32
	vmovss	536(%r14), %xmm0
.Ltmp3656:
	.loc	1 1089 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp3657:
	.loc	1 1177 32
	vmovss	696(%r14), %xmm0
.Ltmp3658:
	.loc	1 1089 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp3659:
	.loc	1 1177 32
	vmovss	232(%r14), %xmm0
.Ltmp3660:
	.loc	1 1089 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp3661:
	.loc	1 1177 32
	vmovss	392(%r14), %xmm0
.Ltmp3662:
	.loc	1 1089 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp3663:
	.loc	1 1177 32
	vmovss	552(%r14), %xmm0
.Ltmp3664:
	.loc	1 1089 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp3665:
	.loc	1 1177 32
	vmovss	712(%r14), %xmm0
.Ltmp3666:
	.loc	1 1089 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp3667:
	.loc	1 1177 32
	vmovss	248(%r14), %xmm0
.Ltmp3668:
	.loc	1 1089 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp3669:
	.loc	1 1177 32
	vmovss	408(%r14), %xmm0
.Ltmp3670:
	.loc	1 1089 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp3671:
	.loc	1 1177 32
	vmovss	568(%r14), %xmm0
.Ltmp3672:
	.loc	1 1089 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp3673:
	.loc	1 1177 32
	vmovss	728(%r14), %xmm0
.Ltmp3674:
	.loc	1 1089 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp3675:
	.loc	1 1177 32
	vmovss	264(%r14), %xmm0
.Ltmp3676:
	.loc	1 1089 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp3677:
	.loc	1 1177 32
	vmovss	424(%r14), %xmm0
.Ltmp3678:
	.loc	1 1089 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp3679:
	.loc	1 1177 32
	vmovss	584(%r14), %xmm0
.Ltmp3680:
	.loc	1 1089 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp3681:
	.loc	1 1177 32
	vmovss	744(%r14), %xmm0
.Ltmp3682:
	.loc	1 1089 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp3683:
	.loc	1 1177 32
	vmovss	280(%r14), %xmm0
.Ltmp3684:
	.loc	1 1089 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp3685:
	.loc	1 1177 32
	vmovss	440(%r14), %xmm0
.Ltmp3686:
	.loc	1 1089 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp3687:
	.loc	1 1177 32
	vmovss	600(%r14), %xmm0
.Ltmp3688:
	.loc	1 1089 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp3689:
	.loc	1 1177 32
	vmovss	760(%r14), %xmm0
.Ltmp3690:
	.loc	1 1089 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp3691:
	.loc	1 1177 32
	vmovss	296(%r14), %xmm0
.Ltmp3692:
	.loc	1 1089 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp3693:
	.loc	1 1177 32
	vmovss	456(%r14), %xmm0
.Ltmp3694:
	.loc	1 1089 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp3695:
	.loc	1 1177 32
	vmovss	616(%r14), %xmm0
.Ltmp3696:
	.loc	1 1089 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp3697:
	.loc	1 1177 32
	vmovss	776(%r14), %xmm0
.Ltmp3698:
	.loc	1 1089 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp3699:
	.loc	1 1177 32
	vmovss	312(%r14), %xmm0
.Ltmp3700:
	.loc	1 1089 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp3701:
	.loc	1 1177 32
	vmovss	472(%r14), %xmm0
.Ltmp3702:
	.loc	1 1089 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp3703:
	.loc	1 1177 32
	vmovss	632(%r14), %xmm0
.Ltmp3704:
	.loc	1 1089 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp3705:
	.loc	1 1177 32
	vmovss	792(%r14), %xmm0
.Ltmp3706:
	.loc	1 1089 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp3707:
	.loc	1 1177 32
	vmovss	328(%r14), %xmm0
.Ltmp3708:
	.loc	1 1089 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp3709:
	.loc	1 1177 32
	vmovss	488(%r14), %xmm0
.Ltmp3710:
	.loc	1 1089 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp3711:
	.loc	1 1177 32
	vmovss	648(%r14), %xmm0
.Ltmp3712:
	.loc	1 1089 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp3713:
	.loc	1 1177 32
	vmovss	808(%r14), %xmm0
.Ltmp3714:
	.loc	1 1089 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp3715:
	.loc	1 1177 32
	vmovss	344(%r14), %xmm0
.Ltmp3716:
	.loc	1 1089 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp3717:
	.loc	1 1177 32
	vmovss	504(%r14), %xmm0
.Ltmp3718:
	.loc	1 1089 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp3719:
	.loc	1 1177 32
	vmovss	664(%r14), %xmm0
.Ltmp3720:
	.loc	1 1089 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp3721:
	.loc	1 1177 32
	vmovss	824(%r14), %xmm0
.Ltmp3722:
	.loc	1 1089 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp3723:
	.loc	1 1176 33
	vmovss	1520(%r14), %xmm0
.Ltmp3724:
	.loc	1 1089 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp3725:
	.loc	1 1176 33
	vmovss	1680(%r14), %xmm0
.Ltmp3726:
	.loc	1 1089 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp3727:
	.loc	1 1176 33
	vmovss	1840(%r14), %xmm0
.Ltmp3728:
	.loc	1 1089 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp3729:
	.loc	1 1176 33
	vmovss	2000(%r14), %xmm0
.Ltmp3730:
	.loc	1 1089 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp3731:
	.loc	1 1176 33
	vmovss	1536(%r14), %xmm0
.Ltmp3732:
	.loc	1 1089 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp3733:
	.loc	1 1176 33
	vmovss	1696(%r14), %xmm0
.Ltmp3734:
	.loc	1 1089 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp3735:
	.loc	1 1176 33
	vmovss	1856(%r14), %xmm0
.Ltmp3736:
	.loc	1 1089 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp3737:
	.loc	1 1176 33
	vmovss	2016(%r14), %xmm0
.Ltmp3738:
	.loc	1 1089 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp3739:
	.loc	1 1176 33
	vmovss	1552(%r14), %xmm0
.Ltmp3740:
	.loc	1 1089 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp3741:
	.loc	1 1176 33
	vmovss	1712(%r14), %xmm0
.Ltmp3742:
	.loc	1 1089 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp3743:
	.loc	1 1176 33
	vmovss	1872(%r14), %xmm0
.Ltmp3744:
	.loc	1 1089 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp3745:
	.loc	1 1176 33
	vmovss	2032(%r14), %xmm0
.Ltmp3746:
	.loc	1 1089 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp3747:
	.loc	1 1176 33
	vmovss	1568(%r14), %xmm0
.Ltmp3748:
	.loc	1 1089 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp3749:
	.loc	1 1176 33
	vmovss	1728(%r14), %xmm0
.Ltmp3750:
	.loc	1 1089 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp3751:
	.loc	1 1176 33
	vmovss	1888(%r14), %xmm0
.Ltmp3752:
	.loc	1 1089 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp3753:
	.loc	1 1176 33
	vmovss	2048(%r14), %xmm0
.Ltmp3754:
	.loc	1 1089 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp3755:
	.loc	1 1176 33
	vmovss	1584(%r14), %xmm0
.Ltmp3756:
	.loc	1 1089 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp3757:
	.loc	1 1176 33
	vmovss	1744(%r14), %xmm0
.Ltmp3758:
	.loc	1 1089 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp3759:
	.loc	1 1176 33
	vmovss	1904(%r14), %xmm0
.Ltmp3760:
	.loc	1 1089 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp3761:
	.loc	1 1176 33
	vmovss	2064(%r14), %xmm0
.Ltmp3762:
	.loc	1 1089 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp3763:
	.loc	1 1176 33
	vmovss	1600(%r14), %xmm0
.Ltmp3764:
	.loc	1 1089 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp3765:
	.loc	1 1176 33
	vmovss	1760(%r14), %xmm0
.Ltmp3766:
	.loc	1 1089 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp3767:
	.loc	1 1176 33
	vmovss	1920(%r14), %xmm0
.Ltmp3768:
	.loc	1 1089 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp3769:
	.loc	1 1176 33
	vmovss	2080(%r14), %xmm0
.Ltmp3770:
	.loc	1 1089 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp3771:
	.loc	1 1176 33
	vmovss	1616(%r14), %xmm0
.Ltmp3772:
	.loc	1 1089 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp3773:
	.loc	1 1176 33
	vmovss	1776(%r14), %xmm0
.Ltmp3774:
	.loc	1 1089 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp3775:
	.loc	1 1176 33
	vmovss	1936(%r14), %xmm0
.Ltmp3776:
	.loc	1 1089 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp3777:
	.loc	1 1176 33
	vmovss	2096(%r14), %xmm0
.Ltmp3778:
	.loc	1 1089 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp3779:
	.loc	1 1176 33
	vmovss	1632(%r14), %xmm0
.Ltmp3780:
	.loc	1 1089 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp3781:
	.loc	1 1176 33
	vmovss	1792(%r14), %xmm0
.Ltmp3782:
	.loc	1 1089 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp3783:
	.loc	1 1176 33
	vmovss	1952(%r14), %xmm0
.Ltmp3784:
	.loc	1 1089 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp3785:
	.loc	1 1176 33
	vmovss	2112(%r14), %xmm0
.Ltmp3786:
	.loc	1 1089 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp3787:
	.loc	1 1176 33
	vmovss	1648(%r14), %xmm0
.Ltmp3788:
	.loc	1 1089 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp3789:
	.loc	1 1176 33
	vmovss	1808(%r14), %xmm0
.Ltmp3790:
	.loc	1 1089 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp3791:
	.loc	1 1176 33
	vmovss	1968(%r14), %xmm0
.Ltmp3792:
	.loc	1 1089 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp3793:
	.loc	1 1176 33
	vmovss	2128(%r14), %xmm0
.Ltmp3794:
	.loc	1 1089 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp3795:
	.loc	1 1176 33
	vmovss	1664(%r14), %xmm0
.Ltmp3796:
	.loc	1 1089 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp3797:
	.loc	1 1176 33
	vmovss	1824(%r14), %xmm0
.Ltmp3798:
	.loc	1 1089 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp3799:
	.loc	1 1176 33
	vmovss	1984(%r14), %xmm0
.Ltmp3800:
	.loc	1 1089 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp3801:
	.loc	1 1176 33
	vmovss	2144(%r14), %xmm0
.Ltmp3802:
	.loc	1 1089 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp3803:
	.loc	1 1177 32
	vmovss	1528(%r14), %xmm0
.Ltmp3804:
	.loc	1 1089 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp3805:
	.loc	1 1177 32
	vmovss	1688(%r14), %xmm0
.Ltmp3806:
	.loc	1 1089 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp3807:
	.loc	1 1177 32
	vmovss	1848(%r14), %xmm0
.Ltmp3808:
	.loc	1 1089 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp3809:
	.loc	1 1177 32
	vmovss	2008(%r14), %xmm0
.Ltmp3810:
	.loc	1 1089 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp3811:
	.loc	1 1177 32
	vmovss	1544(%r14), %xmm0
.Ltmp3812:
	.loc	1 1089 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp3813:
	.loc	1 1177 32
	vmovss	1704(%r14), %xmm0
.Ltmp3814:
	.loc	1 1089 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp3815:
	.loc	1 1177 32
	vmovss	1864(%r14), %xmm0
.Ltmp3816:
	.loc	1 1089 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp3817:
	.loc	1 1177 32
	vmovss	2024(%r14), %xmm0
.Ltmp3818:
	.loc	1 1089 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp3819:
	.loc	1 1177 32
	vmovss	1560(%r14), %xmm0
.Ltmp3820:
	.loc	1 1089 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp3821:
	.loc	1 1177 32
	vmovss	1720(%r14), %xmm0
.Ltmp3822:
	.loc	1 1089 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp3823:
	.loc	1 1177 32
	vmovss	1880(%r14), %xmm0
.Ltmp3824:
	.loc	1 1089 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp3825:
	.loc	1 1177 32
	vmovss	2040(%r14), %xmm0
.Ltmp3826:
	.loc	1 1089 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp3827:
	.loc	1 1177 32
	vmovss	1576(%r14), %xmm0
.Ltmp3828:
	.loc	1 1089 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp3829:
	.loc	1 1177 32
	vmovss	1736(%r14), %xmm0
.Ltmp3830:
	.loc	1 1089 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp3831:
	.loc	1 1177 32
	vmovss	1896(%r14), %xmm0
.Ltmp3832:
	.loc	1 1089 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp3833:
	.loc	1 1177 32
	vmovss	2056(%r14), %xmm0
.Ltmp3834:
	.loc	1 1089 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp3835:
	.loc	1 1177 32
	vmovss	1592(%r14), %xmm0
.Ltmp3836:
	.loc	1 1089 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp3837:
	.loc	1 1177 32
	vmovss	1752(%r14), %xmm0
.Ltmp3838:
	.loc	1 1089 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp3839:
	.loc	1 1177 32
	vmovss	1912(%r14), %xmm0
.Ltmp3840:
	.loc	1 1089 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp3841:
	.loc	1 1177 32
	vmovss	2072(%r14), %xmm0
.Ltmp3842:
	.loc	1 1089 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp3843:
	.loc	1 1177 32
	vmovss	1608(%r14), %xmm0
.Ltmp3844:
	.loc	1 1089 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp3845:
	.loc	1 1177 32
	vmovss	1768(%r14), %xmm0
.Ltmp3846:
	.loc	1 1089 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp3847:
	.loc	1 1177 32
	vmovss	1928(%r14), %xmm0
.Ltmp3848:
	.loc	1 1089 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp3849:
	.loc	1 1177 32
	vmovss	2088(%r14), %xmm0
.Ltmp3850:
	.loc	1 1089 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp3851:
	.loc	1 1177 32
	vmovss	1624(%r14), %xmm0
.Ltmp3852:
	.loc	1 1089 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp3853:
	.loc	1 1177 32
	vmovss	1784(%r14), %xmm0
.Ltmp3854:
	.loc	1 1089 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp3855:
	.loc	1 1177 32
	vmovss	1944(%r14), %xmm0
.Ltmp3856:
	.loc	1 1089 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp3857:
	.loc	1 1177 32
	vmovss	2104(%r14), %xmm0
.Ltmp3858:
	.loc	1 1089 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp3859:
	.loc	1 1177 32
	vmovss	1640(%r14), %xmm0
.Ltmp3860:
	.loc	1 1089 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp3861:
	.loc	1 1177 32
	vmovss	1800(%r14), %xmm0
.Ltmp3862:
	.loc	1 1089 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp3863:
	.loc	1 1177 32
	vmovss	1960(%r14), %xmm0
.Ltmp3864:
	.loc	1 1089 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp3865:
	.loc	1 1177 32
	vmovss	2120(%r14), %xmm0
.Ltmp3866:
	.loc	1 1089 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp3867:
	.loc	1 1177 32
	vmovss	1656(%r14), %xmm0
.Ltmp3868:
	.loc	1 1089 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp3869:
	.loc	1 1177 32
	vmovss	1816(%r14), %xmm0
.Ltmp3870:
	.loc	1 1089 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp3871:
	.loc	1 1177 32
	vmovss	1976(%r14), %xmm0
.Ltmp3872:
	.loc	1 1089 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp3873:
	.loc	1 1177 32
	vmovss	2136(%r14), %xmm0
.Ltmp3874:
	.loc	1 1089 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp3875:
	.loc	1 1177 32
	vmovss	1672(%r14), %xmm0
.Ltmp3876:
	.loc	1 1089 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp3877:
	.loc	1 1177 32
	vmovss	1832(%r14), %xmm0
.Ltmp3878:
	.loc	1 1089 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp3879:
	.loc	1 1177 32
	vmovss	1992(%r14), %xmm0
.Ltmp3880:
	.loc	1 1089 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp3881:
	.loc	1 1177 32
	vmovss	2152(%r14), %xmm0
.Ltmp3882:
	.loc	1 1089 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp3883:
	.loc	1 1091 31
	leaq	1248(%rsp), %rdi
	movq	%r14, %rsi
	movl	908(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	1344(%rsp), %rdi
	movq	1016(%rsp), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovaps	1248(%rsp), %xmm0
	vmovaps	%xmm0, 1136(%rsp)
	vmovaps	1264(%rsp), %xmm0
	vmovaps	%xmm0, 1120(%rsp)
	vmovaps	1280(%rsp), %xmm0
	vmovaps	%xmm0, 1104(%rsp)
	vmovaps	1296(%rsp), %xmm0
	vmovaps	%xmm0, 1088(%rsp)
	vmovaps	1312(%rsp), %xmm0
	vmovaps	%xmm0, 1072(%rsp)
	vmovaps	1328(%rsp), %xmm0
	vmovaps	%xmm0, 1056(%rsp)
	vmovaps	1344(%rsp), %xmm0
	vmovaps	%xmm0, 1040(%rsp)
	vmovaps	1360(%rsp), %xmm0
	vmovaps	%xmm0, 1216(%rsp)
	vmovaps	1376(%rsp), %xmm0
	vmovaps	%xmm0, 1200(%rsp)
	vmovaps	1392(%rsp), %xmm0
	vmovaps	%xmm0, 1184(%rsp)
	vmovaps	1408(%rsp), %xmm0
	vmovaps	%xmm0, 1168(%rsp)
	vmovdqa	1424(%rsp), %xmm0
	vmovdqa	%xmm0, 1152(%rsp)
.Ltmp3884:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%r13), %rcx
	shlq	$2, %r15
	leaq	(,%rcx,4), %rsi
	.loc	1 1094 12 is_stmt 1
	testb	$1, %r12b
	movq	%r13, 976(%rsp)
	movq	%rcx, 936(%rsp)
	je	.LBB34_203
.Ltmp3885:
	.loc	38 1050 16
	cmpq	%r15, %rsi
	movq	912(%rsp), %rax
.Ltmp3886:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_480
.Ltmp3887:
	.loc	48 451 16 is_stmt 1
	cmpq	120(%rsp), %rsi
	ja	.LBB34_480
.Ltmp3888:
	.loc	48 451 16 is_stmt 0
	cmpq	112(%rsp), %rsi
	ja	.LBB34_486
.Ltmp3889:
	.loc	1 972 27 is_stmt 1
	vmovaps	96(%r14), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	112(%r14), %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmovaps	128(%r14), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	144(%r14), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp3890:
	.loc	1 973 26
	vmovaps	1424(%r14), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	1440(%r14), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	1456(%r14), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	1472(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
.Ltmp3891:
	.loc	1 974 25
	vmovaps	160(%r14), %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vmovaps	176(%r14), %xmm0
	vmovaps	%xmm0, 832(%rsp)
.Ltmp3892:
	.loc	1 975 24
	vmovaps	1488(%r14), %xmm0
	vmovaps	%xmm0, 816(%rsp)
	vmovaps	1504(%r14), %xmm6
.Ltmp3893:
	.loc	1 976 24
	movq	2680(%r14), %r11
.Ltmp3894:
	.loc	2 1916 50
	testq	%r13, %r13
	je	.LBB34_199
.Ltmp3895:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r13,4), %rcx
	movq	%rcx, 104(%rsp)
	movq	920(%rsp), %rcx
	leaq	(%rcx,%r15,4), %r10
	leaq	(%rax,%r15,4), %r12
.Ltmp3896:
	.loc	3 900 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r13
	movq	%r13, 896(%rsp)
	xorl	%edi, %edi
	xorl	%edx, %edx
	movq	%r10, 888(%rsp)
	movq	%r12, 880(%rsp)
.Ltmp3897:
	.loc	3 0 12 is_stmt 0
.Ltmp3898:
	.p2align	4
.LBB34_164:
	.loc	1 981 21 is_stmt 1
	vmovaps	176(%rsp), %xmm0
	vmovaps	192(%rsp), %xmm1
	vmovaps	208(%rsp), %xmm2
.Ltmp3899:
	.loc	9 36 14
	vaddps	336(%rsp), %xmm0, %xmm0
.Ltmp3900:
	.loc	1 983 21
	vmovaps	496(%rsp), %xmm3
	.loc	1 980 17
	vmovaps	%xmm0, 176(%rsp)
.Ltmp3901:
	.loc	9 36 14
	vaddps	656(%rsp), %xmm3, %xmm0
.Ltmp3902:
	.loc	1 982 17
	vmovaps	%xmm0, 496(%rsp)
.Ltmp3903:
	.loc	9 36 14
	vaddps	352(%rsp), %xmm1, %xmm0
.Ltmp3904:
	.loc	1 980 17
	vmovaps	%xmm0, 192(%rsp)
	.loc	1 983 21
	vmovaps	512(%rsp), %xmm0
.Ltmp3905:
	.loc	9 36 14
	vaddps	672(%rsp), %xmm0, %xmm0
.Ltmp3906:
	.loc	1 982 17
	vmovaps	%xmm0, 512(%rsp)
.Ltmp3907:
	.loc	9 36 14
	vaddps	368(%rsp), %xmm2, %xmm0
.Ltmp3908:
	.loc	1 980 17
	vmovaps	%xmm0, 208(%rsp)
	.loc	1 983 21
	vmovaps	528(%rsp), %xmm0
.Ltmp3909:
	.loc	9 36 14
	vaddps	688(%rsp), %xmm0, %xmm0
.Ltmp3910:
	.loc	1 982 17
	vmovaps	%xmm0, 528(%rsp)
	.loc	1 981 21
	vmovaps	224(%rsp), %xmm0
.Ltmp3911:
	.loc	9 36 14
	vaddps	384(%rsp), %xmm0, %xmm0
.Ltmp3912:
	.loc	1 980 17
	vmovaps	%xmm0, 224(%rsp)
	.loc	1 983 21
	vmovaps	544(%rsp), %xmm0
.Ltmp3913:
	.loc	9 36 14
	vaddps	704(%rsp), %xmm0, %xmm0
.Ltmp3914:
	.loc	1 982 17
	vmovaps	%xmm0, 544(%rsp)
	.loc	1 981 21
	vmovaps	240(%rsp), %xmm0
.Ltmp3915:
	.loc	9 36 14
	vaddps	400(%rsp), %xmm0, %xmm0
.Ltmp3916:
	.loc	1 980 17
	vmovaps	%xmm0, 240(%rsp)
	.loc	1 983 21
	vmovaps	560(%rsp), %xmm0
.Ltmp3917:
	.loc	9 36 14
	vaddps	720(%rsp), %xmm0, %xmm0
.Ltmp3918:
	.loc	1 982 17
	vmovaps	%xmm0, 560(%rsp)
	.loc	1 981 21
	vmovaps	256(%rsp), %xmm0
.Ltmp3919:
	.loc	9 36 14
	vaddps	416(%rsp), %xmm0, %xmm0
.Ltmp3920:
	.loc	1 980 17
	vmovaps	%xmm0, 256(%rsp)
	.loc	1 983 21
	vmovaps	576(%rsp), %xmm0
.Ltmp3921:
	.loc	9 36 14
	vaddps	736(%rsp), %xmm0, %xmm0
.Ltmp3922:
	.loc	1 982 17
	vmovaps	%xmm0, 576(%rsp)
	.loc	1 981 21
	vmovaps	272(%rsp), %xmm0
.Ltmp3923:
	.loc	9 36 14
	vaddps	432(%rsp), %xmm0, %xmm0
.Ltmp3924:
	.loc	1 980 17
	vmovaps	%xmm0, 272(%rsp)
	.loc	1 983 21
	vmovaps	592(%rsp), %xmm0
.Ltmp3925:
	.loc	9 36 14
	vaddps	752(%rsp), %xmm0, %xmm0
.Ltmp3926:
	.loc	1 982 17
	vmovaps	%xmm0, 592(%rsp)
	.loc	1 981 21
	vmovaps	288(%rsp), %xmm0
.Ltmp3927:
	.loc	9 36 14
	vaddps	448(%rsp), %xmm0, %xmm0
.Ltmp3928:
	.loc	1 980 17
	vmovaps	%xmm0, 288(%rsp)
	.loc	1 983 21
	vmovaps	608(%rsp), %xmm0
.Ltmp3929:
	.loc	9 36 14
	vaddps	768(%rsp), %xmm0, %xmm0
.Ltmp3930:
	.loc	1 982 17
	vmovaps	%xmm0, 608(%rsp)
	.loc	1 981 21
	vmovaps	304(%rsp), %xmm0
.Ltmp3931:
	.loc	9 36 14
	vaddps	464(%rsp), %xmm0, %xmm0
.Ltmp3932:
	.loc	1 980 17
	vmovaps	%xmm0, 304(%rsp)
	.loc	1 983 21
	vmovaps	624(%rsp), %xmm0
.Ltmp3933:
	.loc	9 36 14
	vaddps	784(%rsp), %xmm0, %xmm0
.Ltmp3934:
	.loc	1 982 17
	vmovaps	%xmm0, 624(%rsp)
	.loc	1 981 21
	vmovaps	320(%rsp), %xmm0
.Ltmp3935:
	.loc	9 36 14
	vaddps	480(%rsp), %xmm0, %xmm0
.Ltmp3936:
	.loc	1 980 17
	vmovaps	%xmm0, 320(%rsp)
	.loc	1 983 21
	vmovaps	640(%rsp), %xmm0
.Ltmp3937:
	.loc	9 36 14
	vaddps	800(%rsp), %xmm0, %xmm0
.Ltmp3938:
	.loc	1 982 17
	vmovaps	%xmm0, 640(%rsp)
.Ltmp3939:
	.loc	1 987 28
	leaq	1(%r11), %rax
	movq	1024(%rsp), %r15
.Ltmp3940:
	.loc	1 857 8
	cmpq	%r15, %rax
	movl	$0, %r9d
	cmovaeq	%r15, %r9
.Ltmp3941:
	.loc	48 568 12
	cmpq	104(%rsp), %rdi
	ja	.LBB34_456
.Ltmp3942:
	.loc	48 438 16
	cmpq	%rdx, 896(%rsp)
	je	.LBB34_449
.Ltmp3943:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r11,4), %rax
.Ltmp3944:
	.loc	1 1000 29 is_stmt 1
	movq	8(%r14), %rsi
.Ltmp3945:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_457
.Ltmp3946:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp3947:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm6, 160(%rsp)
	vmovups	(%r10,%rdi,4), %xmm5
.Ltmp3948:
	vmovups	(%r12,%rdi,4), %xmm6
.Ltmp3949:
	vmovaps	32(%r14), %xmm3
	vmovaps	48(%r14), %xmm11
	vmovaps	64(%r14), %xmm0
	vmovaps	1360(%r14), %xmm10
	vmovaps	1376(%r14), %xmm9
	vmovaps	1392(%r14), %xmm14
	vmovaps	64(%rsp), %xmm8
	vsubps	%xmm8, %xmm5, %xmm1
	vmulps	%xmm1, %xmm11, %xmm2
	vmovaps	80(%rsp), %xmm7
	vmovaps	%xmm3, 960(%rsp)
	vmulps	%xmm3, %xmm7, %xmm3
	vaddps	%xmm2, %xmm3, %xmm4
	vaddps	%xmm4, %xmm7, %xmm2
	vmulps	%xmm7, %xmm11, %xmm3
	vmulps	%xmm0, %xmm1, %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vaddps	%xmm3, %xmm8, %xmm1
	vmulps	80(%r14), %xmm2, %xmm12
	vmovaps	128(%rsp), %xmm7
	vsubps	%xmm7, %xmm1, %xmm1
	vmulps	48(%rsp), %xmm11, %xmm2
	vmulps	%xmm1, %xmm0, %xmm0
	vaddps	%xmm0, %xmm2, %xmm2
	vaddps	%xmm2, %xmm7, %xmm0
.Ltmp3950:
	vsubps	16(%rsp), %xmm6, %xmm13
	vmulps	%xmm9, %xmm13, %xmm7
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm10, 944(%rsp)
	vmulps	%xmm10, %xmm8, %xmm15
	vaddps	%xmm7, %xmm15, %xmm15
	vaddps	%xmm15, %xmm8, %xmm7
	vmulps	1408(%r14), %xmm7, %xmm7
.Ltmp3951:
	.loc	1 1000 29 is_stmt 1
	movq	(%r14), %rcx
.Ltmp3952:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp3953:
	.loc	1 1001 30
	movq	24(%r14), %rsi
.Ltmp3954:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_471
.Ltmp3955:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp3956:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm5, %xmm12, %xmm5
	vsubps	%xmm0, %xmm5, %xmm0
.Ltmp3957:
	.loc	1 1001 30 is_stmt 1
	movq	16(%r14), %rcx
.Ltmp3958:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp3959:
	.loc	1 1002 28
	movq	1336(%r14), %rsi
.Ltmp3960:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_472
.Ltmp3961:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp3962:
	.loc	1 0 0 is_stmt 0
	vmulps	32(%rsp), %xmm9, %xmm0
	vmulps	%xmm14, %xmm13, %xmm5
	vaddps	%xmm5, %xmm0, %xmm5
	vaddps	16(%rsp), %xmm5, %xmm0
	vmovaps	144(%rsp), %xmm10
	vsubps	%xmm10, %xmm0, %xmm13
	vmovaps	864(%rsp), %xmm8
	vmulps	%xmm9, %xmm8, %xmm0
	vmulps	%xmm13, %xmm14, %xmm12
	vaddps	%xmm0, %xmm12, %xmm14
	vaddps	%xmm14, %xmm10, %xmm0
.Ltmp3963:
	.loc	1 1002 28 is_stmt 1
	movq	1328(%r14), %rcx
.Ltmp3964:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp3965:
	.loc	1 1003 29
	movq	1352(%r14), %rsi
.Ltmp3966:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_460
.Ltmp3967:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp3968:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm7, %xmm6, %xmm6
	vsubps	%xmm0, %xmm6, %xmm0
.Ltmp3969:
	.loc	1 1003 29 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp3970:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
	movq	1104(%r14), %r8
.Ltmp3971:
	.loc	1 877 35
	addq	%r11, %r8
.Ltmp3972:
	.loc	1 857 8
	cmpq	%r15, %r8
	movl	$0, %eax
	cmovaeq	%r15, %rax
.Ltmp3973:
	.loc	1 1006 34
	movq	8(%r14), %rsi
.Ltmp3974:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp3975:
	.loc	1 877 30
	shlq	$2, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_488
	.loc	1 0 25
	movq	1112(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp3976:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %ecx
	cmovaeq	%r15, %rcx
	subq	%rcx, %rax
.Ltmp3977:
	.loc	1 877 30
	leaq	1(,%rax,4), %rbp
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	.loc	1 0 25
	movq	1120(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp3978:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %ecx
	cmovaeq	%r15, %rcx
	subq	%rcx, %rax
.Ltmp3979:
	.loc	1 877 30
	leaq	2(,%rax,4), %rbx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_508
	.loc	1 0 25
	movq	1128(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp3980:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %ecx
	cmovaeq	%r15, %rcx
	subq	%rcx, %rax
.Ltmp3981:
	.loc	1 877 30
	leaq	3(,%rax,4), %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_510
.Ltmp3982:
	.loc	1 1008 34 is_stmt 1
	movq	24(%r14), %rsi
.Ltmp3983:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB34_488
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	cmpq	%rsi, %rbx
	jae	.LBB34_508
	.loc	1 0 25 is_stmt 0
	movq	%rdx, 992(%rsp)
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB34_510
.Ltmp3984:
	.loc	1 0 25
	movq	2432(%r14), %rax
.Ltmp3985:
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp3986:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
.Ltmp3987:
	.loc	1 1010 34
	movq	1336(%r14), %rsi
.Ltmp3988:
	.loc	1 857 8
	subq	%rdx, %rax
.Ltmp3989:
	.loc	1 877 30
	shlq	$2, %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_494
	.loc	1 0 25
	movq	2440(%r14), %rdx
	.loc	1 877 35
	addq	%r11, %rdx
.Ltmp3990:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rdx
	movl	$0, %r10d
	cmovaeq	%r15, %r10
	subq	%r10, %rdx
.Ltmp3991:
	.loc	1 877 30
	leaq	1(,%rdx,4), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB34_493
	.loc	1 0 25
	movq	2448(%r14), %r10
	.loc	1 877 35
	addq	%r11, %r10
.Ltmp3992:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %r10
	movq	%r14, %r13
	movl	$0, %r14d
	cmovaeq	%r15, %r14
	subq	%r14, %r10
.Ltmp3993:
	.loc	1 877 30
	leaq	2(,%r10,4), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_523
	.loc	1 0 25
	movq	2456(%r13), %r10
	.loc	1 877 35
	addq	%r11, %r10
.Ltmp3994:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %r10
	movl	$0, %r14d
	cmovaeq	%r15, %r14
	subq	%r14, %r10
.Ltmp3995:
	.loc	1 877 30
	leaq	3(,%r10,4), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_514
.Ltmp3996:
	.loc	1 1012 34 is_stmt 1
	movq	1352(%r13), %rsi
.Ltmp3997:
	.loc	1 877 25
	cmpq	%rsi, %rax
	jae	.LBB34_494
	cmpq	%rsi, %rdx
	jae	.LBB34_493
	cmpq	%rsi, %r12
	jae	.LBB34_523
	cmpq	%rsi, %r15
	jae	.LBB34_514
.Ltmp3998:
	.loc	1 0 25 is_stmt 0
	movq	%r13, %r14
	negq	%r9
	addq	%r9, %r11
	incq	%r11
	leaq	(,%r11,4), %r10
.Ltmp3999:
	.loc	1 1047 36 is_stmt 1
	movq	8(%r13), %rsi
.Ltmp4000:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%r10, %r9
	jb	.LBB34_467
.Ltmp4001:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4002:
	.loc	1 1049 27
	movq	24(%r14), %rsi
.Ltmp4003:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%r10, %r9
	jb	.LBB34_468
.Ltmp4004:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4005:
	.loc	1 1050 35
	movq	1336(%r14), %rsi
.Ltmp4006:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%r10, %r9
	jb	.LBB34_469
.Ltmp4007:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4008:
	.loc	1 1052 27
	movq	1352(%r14), %rsi
.Ltmp4009:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%r10, %r9
	jb	.LBB34_470
.Ltmp4010:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4011:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm4, %xmm4, %xmm0
	vaddps	80(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm6
	vandps	%xmm6, %xmm0, %xmm4
	vbroadcastss	.LCPI34_2(%rip), %xmm7
	vcmpltps	%xmm7, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vaddps	%xmm3, %xmm3, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm6, %xmm0, %xmm3
	vcmpltps	%xmm7, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmulps	%xmm1, %xmm11, %xmm0
	vmovaps	48(%rsp), %xmm3
	vmulps	960(%rsp), %xmm3, %xmm1
	vaddps	%xmm0, %xmm1, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm3, %xmm0
	vandps	%xmm6, %xmm0, %xmm1
	vcmpltps	%xmm7, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm2, %xmm2, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm6, %xmm0, %xmm1
	vcmpltps	%xmm7, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4012:
	vaddps	%xmm15, %xmm15, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vandps	%xmm6, %xmm0, %xmm1
	vcmpltps	%xmm7, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm5, %xmm5, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm6, %xmm0, %xmm1
	vcmpltps	%xmm7, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 16(%rsp)
.Ltmp4013:
	movq	(%r14), %rsi
	vmovd	(%rsi,%r8,4), %xmm0
	vpinsrd	$1, (%rsi,%rbp,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%rbx,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%rcx,4), %xmm0, %xmm0
.Ltmp4014:
	movq	16(%r14), %rsi
	vmovd	(%rsi,%r8,4), %xmm1
	vpinsrd	$1, (%rsi,%rbp,4), %xmm1, %xmm1
.Ltmp4015:
	vmulps	%xmm13, %xmm9, %xmm2
.Ltmp4016:
	vpinsrd	$2, (%rsi,%rbx,4), %xmm1, %xmm1
.Ltmp4017:
	vmulps	944(%rsp), %xmm8, %xmm3
	vaddps	%xmm2, %xmm3, %xmm2
.Ltmp4018:
	vpinsrd	$3, (%rsi,%rcx,4), %xmm1, %xmm1
.Ltmp4019:
	vaddps	%xmm2, %xmm2, %xmm2
	vaddps	%xmm2, %xmm8, %xmm2
	vandps	%xmm6, %xmm2, %xmm3
	vcmpltps	%xmm7, %xmm3, %xmm3
	vandnps	%xmm2, %xmm3, %xmm2
	vmovaps	%xmm2, 864(%rsp)
	vaddps	%xmm14, %xmm14, %xmm2
	vaddps	144(%rsp), %xmm2, %xmm2
	vandps	%xmm6, %xmm2, %xmm3
	vcmpltps	%xmm7, %xmm3, %xmm3
	vmovaps	%xmm7, %xmm5
	vandnps	%xmm2, %xmm3, %xmm2
	vmovaps	%xmm2, 144(%rsp)
.Ltmp4020:
	vpand	%xmm6, %xmm1, %xmm1
	vpand	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_4(%rip), %xmm8
.Ltmp4021:
	vmaxps	%xmm8, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm9
	vmaxps	%xmm9, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm10
	vandps	%xmm0, %xmm10, %xmm2
	vbroadcastss	.LCPI34_32(%rip), %xmm11
	vorps	%xmm2, %xmm11, %xmm2
	vbroadcastss	.LCPI34_8(%rip), %xmm12
	vaddps	%xmm2, %xmm12, %xmm2
	vbroadcastss	.LCPI34_9(%rip), %xmm13
	vmulps	%xmm2, %xmm13, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm14
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm10
	vaddps	%xmm3, %xmm10, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm9
	vaddps	%xmm3, %xmm9, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vsubps	176(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm7
	vcmpltps	%xmm0, %xmm7, %xmm2
	vaddps	%xmm7, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm4
	vmulps	%xmm4, %xmm3, %xmm3
	vblendvps	%xmm2, %xmm0, %xmm3, %xmm2
	vbroadcastss	.LCPI34_21(%rip), %xmm3
	vcmpleps	%xmm3, %xmm0, %xmm0
	vmulps	1136(%rsp), %xmm2, %xmm2
	vxorps	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm3, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vminps	%xmm3, %xmm0, %xmm0
	vmovaps	848(%rsp), %xmm4
	vcmpltps	%xmm4, %xmm0, %xmm2
	vmovaps	1104(%rsp), %xmm3
	vblendvps	%xmm2, 1120(%rsp), %xmm3, %xmm2
	vsubps	%xmm0, %xmm4, %xmm3
	vmulps	%xmm2, %xmm3, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vandps	%xmm6, %xmm0, %xmm2
	vcmpltps	%xmm5, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vaddps	240(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_24(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm2
	vsubps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vbroadcastss	.LCPI34_28(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vbroadcastss	.LCPI34_29(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vbroadcastss	.LCPI34_30(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vbroadcastss	.LCPI34_31(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vmovaps	%xmm8, %xmm7
.Ltmp4022:
	vmaxps	%xmm8, %xmm1, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm8
	vmaxps	%xmm8, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm11
	vandps	%xmm0, %xmm11, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm12
	vorps	%xmm1, %xmm12, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm13
	vaddps	%xmm1, %xmm13, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm14
	vmulps	%xmm1, %xmm14, %xmm4
	vbroadcastss	.LCPI34_10(%rip), %xmm15
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm1, %xmm4
	vbroadcastss	.LCPI34_11(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm1, %xmm4
	vbroadcastss	.LCPI34_12(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm1, %xmm4
	vaddps	%xmm4, %xmm10, %xmm4
	vmulps	%xmm4, %xmm1, %xmm4
	vaddps	%xmm4, %xmm9, %xmm4
	vmulps	%xmm4, %xmm1, %xmm1
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vsubps	256(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm1
	vmulps	%xmm1, %xmm1, %xmm1
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm1, %xmm1
	vcmpltps	%xmm0, %xmm4, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm1, %xmm4
.Ltmp4023:
	movq	1328(%r14), %rcx
	vbroadcastss	.LCPI34_32(%rip), %xmm9
.Ltmp4024:
	vaddps	%xmm3, %xmm9, %xmm1
	vbroadcastss	.LCPI34_33(%rip), %xmm10
	vaddps	%xmm2, %xmm10, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vmovaps	%xmm1, 944(%rsp)
.Ltmp4025:
	vmovd	(%rcx,%rax,4), %xmm2
	vpinsrd	$1, (%rcx,%rdx,4), %xmm2, %xmm2
	vpinsrd	$2, (%rcx,%r12,4), %xmm2, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm1
.Ltmp4026:
	vcmpleps	%xmm1, %xmm0, %xmm0
	vmulps	1088(%rsp), %xmm4, %xmm2
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm0, %xmm1, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	832(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm0, %xmm2
	vmovaps	1056(%rsp), %xmm4
	vblendvps	%xmm2, 1072(%rsp), %xmm4, %xmm2
	vsubps	%xmm0, %xmm5, %xmm4
	vmulps	%xmm2, %xmm4, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vandps	%xmm6, %xmm0, %xmm2
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 832(%rsp)
	vaddps	320(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_24(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm2
	vsubps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm4
	vbroadcastss	.LCPI34_28(%rip), %xmm10
	vaddps	%xmm4, %xmm10, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_29(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_30(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_31(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
.Ltmp4027:
	vpinsrd	$3, (%rcx,%r15,4), %xmm3, %xmm3
.Ltmp4028:
	vpand	%xmm6, %xmm3, %xmm3
.Ltmp4029:
	vmaxps	%xmm7, %xmm3, %xmm3
	vmaxps	%xmm8, %xmm3, %xmm3
	vandps	%xmm3, %xmm11, %xmm4
	vorps	%xmm4, %xmm12, %xmm4
	vaddps	%xmm4, %xmm13, %xmm4
	vmulps	%xmm4, %xmm14, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_11(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_12(%rip), %xmm12
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_13(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_14(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm4
	vpsrld	$23, %xmm3, %xmm3
	vpor	.LCPI34_15(%rip), %xmm3, %xmm3
	vbroadcastss	.LCPI34_16(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vaddps	%xmm4, %xmm3, %xmm3
	vmovaps	%xmm9, %xmm1
.Ltmp4030:
	vaddps	%xmm0, %xmm9, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm9
	vaddps	%xmm2, %xmm9, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vbroadcastss	.LCPI34_17(%rip), %xmm13
.Ltmp4031:
	vmulps	%xmm3, %xmm13, %xmm3
	vbroadcastss	.LCPI34_18(%rip), %xmm14
	vmaxps	%xmm14, %xmm3, %xmm3
	vbroadcastss	.LCPI34_19(%rip), %xmm4
	vminps	%xmm4, %xmm3, %xmm3
	vsubps	496(%rsp), %xmm3, %xmm3
.Ltmp4032:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 960(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm2
.Ltmp4033:
	vaddps	%xmm2, %xmm3, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm7
	vmulps	%xmm7, %xmm0, %xmm0
	vcmpltps	%xmm3, %xmm2, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm4
	vcmpleps	%xmm4, %xmm3, %xmm3
	vmulps	1040(%rsp), %xmm0, %xmm0
	vxorps	%xmm4, %xmm4, %xmm4
	vpcmpgtd	%xmm3, %xmm4, %xmm3
	vpandn	%xmm0, %xmm3, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm4, %xmm0, %xmm0
	vxorps	%xmm11, %xmm11, %xmm11
	vmovaps	816(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm0, %xmm3
	vmovaps	1200(%rsp), %xmm4
	vblendvps	%xmm3, 1216(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm5, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm6, %xmm0, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm4
	vcmpltps	%xmm4, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 816(%rsp)
	vaddps	560(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_24(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm3
	vminps	%xmm3, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm8
	vmulps	%xmm0, %xmm8, %xmm4
	vaddps	%xmm4, %xmm10, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_29(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_30(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_31(%rip), %xmm15
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
.Ltmp4034:
	movq	1344(%r14), %rcx
.Ltmp4035:
	vaddps	%xmm1, %xmm0, %xmm0
	vaddps	%xmm3, %xmm9, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
.Ltmp4036:
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%rdx,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%r12,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%r15,4), %xmm0, %xmm0
.Ltmp4037:
	vpand	%xmm6, %xmm0, %xmm0
.Ltmp4038:
	vbroadcastss	.LCPI34_4(%rip), %xmm4
	vmaxps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm4
	vmaxps	%xmm4, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm4
	vorps	.LCPI34_7(%rip), %xmm4, %xmm4
	vbroadcastss	.LCPI34_8(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vbroadcastss	.LCPI34_9(%rip), %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_10(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_11(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_13(%rip), %xmm12
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_14(%rip), %xmm12
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm4, %xmm4
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm0, %xmm13, %xmm0
	vmaxps	%xmm14, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm4
	vminps	%xmm4, %xmm0, %xmm0
	vsubps	576(%rsp), %xmm0, %xmm0
	vaddps	%xmm2, %xmm0, %xmm4
	vmulps	%xmm4, %xmm4, %xmm4
	vmulps	%xmm7, %xmm4, %xmm4
	vcmpltps	%xmm0, %xmm2, %xmm5
	vblendvps	%xmm5, %xmm0, %xmm4, %xmm4
	vbroadcastss	.LCPI34_21(%rip), %xmm2
	vcmpleps	%xmm2, %xmm0, %xmm0
	vmulps	1184(%rsp), %xmm4, %xmm4
	vpcmpgtd	%xmm0, %xmm11, %xmm0
	vpandn	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vmovaps	160(%rsp), %xmm7
	vcmpltps	%xmm7, %xmm0, %xmm4
	vmovaps	1152(%rsp), %xmm5
	vblendvps	%xmm4, 1168(%rsp), %xmm5, %xmm4
	vsubps	%xmm0, %xmm7, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vandps	%xmm6, %xmm0, %xmm4
	vbroadcastss	.LCPI34_2(%rip), %xmm2
	vcmpltps	%xmm2, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm6
	vaddps	640(%rsp), %xmm6, %xmm0
	vbroadcastss	.LCPI34_24(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm0, %xmm8, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm1, %xmm0, %xmm0
	vaddps	%xmm4, %xmm9, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
.Ltmp4039:
	movq	(%r14), %rax
	vmovaps	944(%rsp), %xmm1
	vmulps	(%rax,%r10,4), %xmm1, %xmm1
	movq	16(%r14), %rax
	vmovaps	960(%rsp), %xmm2
	vmulps	(%rax,%r10,4), %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm1
.Ltmp4040:
	movq	1328(%r14), %rax
	vmulps	(%rax,%r10,4), %xmm3, %xmm2
	.loc	1 1052 27 is_stmt 1
	movq	1344(%r14), %rax
.Ltmp4041:
	.loc	9 88 14
	vmulps	(%rax,%r10,4), %xmm0, %xmm0
.Ltmp4042:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	888(%rsp), %r10
.Ltmp4043:
	.loc	8 551 14
	vmovups	%xmm1, (%r10,%rdi,4)
	movq	880(%rsp), %r12
.Ltmp4044:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r12,%rdi,4)
	movq	992(%rsp), %rdx
.Ltmp4045:
	.loc	1 0 0
	incq	%rdx
.Ltmp4046:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	movq	976(%rsp), %r13
	cmpq	%rdx, %r13
.Ltmp4047:
	.loc	3 900 12
	jne	.LBB34_164
.Ltmp4048:
.LBB34_199:
	.loc	3 0 12 is_stmt 0
	vmovaps	80(%rsp), %xmm0
	.loc	1 1057 5 is_stmt 1
	vmovaps	%xmm0, 96(%r14)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 112(%r14)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 128(%r14)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 144(%r14)
	vmovaps	32(%rsp), %xmm0
	.loc	1 1058 5
	vmovaps	%xmm0, 1424(%r14)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%r14)
	vmovaps	864(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%r14)
	vmovaps	144(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%r14)
	vmovaps	848(%rsp), %xmm0
	.loc	1 1059 5
	vmovaps	%xmm0, 160(%r14)
	vmovaps	832(%rsp), %xmm0
	vmovaps	%xmm0, 176(%r14)
	vmovaps	816(%rsp), %xmm0
	.loc	1 1060 5
	vmovaps	%xmm0, 1488(%r14)
	vmovaps	%xmm6, 1504(%r14)
	.loc	1 1061 5
	movq	%r11, 2680(%r14)
	xorl	%eax, %eax
	xorl	%r8d, %r8d
.Ltmp4049:
	.loc	1 0 5 is_stmt 0
.Ltmp4050:
	.p2align	4
.LBB34_200:
	.loc	1 1194 13 is_stmt 1
	vmovd	176(%rsp,%rax), %xmm0
	vmovss	180(%rsp,%rax), %xmm1
	vmovss	184(%rsp,%rax), %xmm2
	vmovss	188(%rsp,%rax), %xmm3
.Ltmp4051:
	.loc	1 1197 17
	vmovd	%xmm0, 192(%r14,%rax)
	.loc	1 1198 34
	movl	204(%r14,%rax), %ecx
	movl	364(%r14,%rax), %edx
.Ltmp4052:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp4053:
	.loc	1 1198 17
	movl	%ecx, 204(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 352(%r14,%rax)
.Ltmp4054:
	.loc	38 2472 13
	subl	%r13d, %edx
	cmovbl	%r8d, %edx
.Ltmp4055:
	.loc	1 1198 17
	movl	%edx, 364(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 512(%r14,%rax)
	.loc	1 1198 34
	movl	524(%r14,%rax), %ecx
.Ltmp4056:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp4057:
	.loc	1 1198 17
	movl	%ecx, 524(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm3, 672(%r14,%rax)
	.loc	1 1198 34
	movl	684(%r14,%rax), %ecx
.Ltmp4058:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp4059:
	.loc	1 1198 17
	movl	%ecx, 684(%r14,%rax)
.Ltmp4060:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp4061:
	.loc	3 900 12
	jne	.LBB34_200
.Ltmp4062:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	928(%rsp), %rsi
	movq	936(%rsp), %r15
	.p2align	4
.LBB34_202:
.Ltmp4063:
	.loc	1 1194 13 is_stmt 1
	vmovd	496(%rsp,%rax), %xmm0
	vmovss	500(%rsp,%rax), %xmm1
	vmovss	504(%rsp,%rax), %xmm2
	vmovss	508(%rsp,%rax), %xmm3
.Ltmp4064:
	.loc	1 1197 17
	vmovd	%xmm0, 1520(%r14,%rax)
	.loc	1 1198 34
	movl	1532(%r14,%rax), %ecx
	movl	1692(%r14,%rax), %edx
.Ltmp4065:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp4066:
	.loc	1 1198 17
	movl	%ecx, 1532(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 1680(%r14,%rax)
.Ltmp4067:
	.loc	38 2472 13
	subl	%r13d, %edx
	cmovbl	%r8d, %edx
.Ltmp4068:
	.loc	1 1198 17
	movl	%edx, 1692(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 1840(%r14,%rax)
	.loc	1 1198 34
	movl	1852(%r14,%rax), %ecx
.Ltmp4069:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp4070:
	.loc	1 1198 17
	movl	%ecx, 1852(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm3, 2000(%r14,%rax)
	.loc	1 1198 34
	movl	2012(%r14,%rax), %ecx
.Ltmp4071:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp4072:
	.loc	1 1198 17
	movl	%ecx, 2012(%r14,%rax)
.Ltmp4073:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp4074:
	.loc	3 900 12
	jne	.LBB34_202
	jmp	.LBB34_157
.Ltmp4075:
.LBB34_203:
	.loc	38 1050 16
	cmpq	%r15, %rsi
	movq	912(%rsp), %rax
.Ltmp4076:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_481
.Ltmp4077:
	.loc	48 451 16 is_stmt 1
	cmpq	120(%rsp), %rsi
	ja	.LBB34_481
.Ltmp4078:
	.loc	48 451 16 is_stmt 0
	cmpq	112(%rsp), %rsi
	ja	.LBB34_484
.Ltmp4079:
	.loc	1 972 27 is_stmt 1
	vmovaps	96(%r14), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	112(%r14), %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmovaps	128(%r14), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	144(%r14), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4080:
	.loc	1 973 26
	vmovaps	1424(%r14), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	1440(%r14), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	1456(%r14), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	1472(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
.Ltmp4081:
	.loc	1 974 25
	vmovaps	160(%r14), %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vmovaps	176(%r14), %xmm0
	vmovaps	%xmm0, 832(%rsp)
.Ltmp4082:
	.loc	1 975 24
	vmovaps	1488(%r14), %xmm0
	vmovaps	%xmm0, 816(%rsp)
	vmovaps	1504(%r14), %xmm6
.Ltmp4083:
	.loc	1 976 24
	movq	2680(%r14), %rcx
.Ltmp4084:
	.loc	2 1916 50
	testq	%r13, %r13
	je	.LBB34_156
.Ltmp4085:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r13,4), %rdx
	movq	%rdx, 104(%rsp)
	movq	920(%rsp), %rdx
	leaq	(%rdx,%r15,4), %r10
	leaq	(%rax,%r15,4), %r11
.Ltmp4086:
	.loc	48 568 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r13
	movq	%r13, 896(%rsp)
	xorl	%edi, %edi
	xorl	%ebx, %ebx
	movq	%r10, 888(%rsp)
	movq	%r11, 880(%rsp)
.Ltmp4087:
	.loc	48 0 12 is_stmt 0
.Ltmp4088:
	.p2align	4
.LBB34_208:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rcx), %rax
	movq	1024(%rsp), %r15
.Ltmp4089:
	.loc	1 857 8
	cmpq	%r15, %rax
	movl	$0, %r9d
	cmovaeq	%r15, %r9
.Ltmp4090:
	.loc	48 568 12
	cmpq	104(%rsp), %rdi
	ja	.LBB34_456
.Ltmp4091:
	.loc	48 438 16
	cmpq	%rbx, 896(%rsp)
	je	.LBB34_449
.Ltmp4092:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rcx,4), %rax
.Ltmp4093:
	.loc	1 1000 29 is_stmt 1
	movq	8(%r14), %rsi
.Ltmp4094:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_457
.Ltmp4095:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4096:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm6, 160(%rsp)
	vmovups	(%r10,%rdi,4), %xmm1
.Ltmp4097:
	vmovups	(%r11,%rdi,4), %xmm14
.Ltmp4098:
	vmovaps	32(%r14), %xmm10
	vmovaps	48(%r14), %xmm11
	vmovaps	64(%r14), %xmm3
	vmovaps	1360(%r14), %xmm15
	vmovaps	1376(%r14), %xmm9
	vmovaps	1392(%r14), %xmm2
	vmovaps	64(%rsp), %xmm8
	vsubps	%xmm8, %xmm1, %xmm0
	vmulps	%xmm0, %xmm11, %xmm4
	vmovaps	80(%rsp), %xmm6
	vmulps	%xmm6, %xmm10, %xmm5
	vaddps	%xmm4, %xmm5, %xmm12
	vaddps	%xmm6, %xmm12, %xmm4
	vmulps	%xmm6, %xmm11, %xmm5
	vmulps	%xmm3, %xmm0, %xmm0
	vaddps	%xmm0, %xmm5, %xmm7
	vaddps	%xmm7, %xmm8, %xmm5
	vmulps	80(%r14), %xmm4, %xmm0
	vmovaps	128(%rsp), %xmm8
	vsubps	%xmm8, %xmm5, %xmm5
	vmulps	48(%rsp), %xmm11, %xmm4
	vmulps	%xmm5, %xmm3, %xmm3
	vaddps	%xmm3, %xmm4, %xmm6
	vaddps	%xmm6, %xmm8, %xmm4
.Ltmp4099:
	vsubps	16(%rsp), %xmm14, %xmm13
	vmulps	%xmm9, %xmm13, %xmm3
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm15, 944(%rsp)
	vmulps	%xmm15, %xmm8, %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vaddps	%xmm3, %xmm8, %xmm15
	vmulps	1408(%r14), %xmm15, %xmm15
.Ltmp4100:
	.loc	1 1000 29 is_stmt 1
	movq	(%r14), %rdx
.Ltmp4101:
	.loc	8 551 14
	vmovups	%xmm4, (%rdx,%rax,4)
.Ltmp4102:
	.loc	1 1001 30
	movq	24(%r14), %rsi
.Ltmp4103:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_471
.Ltmp4104:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4105:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm0, %xmm1, %xmm0
	vsubps	%xmm4, %xmm0, %xmm0
.Ltmp4106:
	.loc	1 1001 30 is_stmt 1
	movq	16(%r14), %rdx
.Ltmp4107:
	.loc	8 551 14
	vmovups	%xmm0, (%rdx,%rax,4)
.Ltmp4108:
	.loc	1 1002 28
	movq	1336(%r14), %rsi
.Ltmp4109:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_472
.Ltmp4110:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4111:
	.loc	1 0 0 is_stmt 0
	vmulps	32(%rsp), %xmm9, %xmm0
	vmulps	%xmm2, %xmm13, %xmm1
	vaddps	%xmm1, %xmm0, %xmm13
	vaddps	16(%rsp), %xmm13, %xmm0
	vmovaps	144(%rsp), %xmm4
	vsubps	%xmm4, %xmm0, %xmm1
	vmovaps	864(%rsp), %xmm8
	vmulps	%xmm9, %xmm8, %xmm0
	vmulps	%xmm1, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm2
	vaddps	%xmm2, %xmm4, %xmm0
.Ltmp4112:
	.loc	1 1002 28 is_stmt 1
	movq	1328(%r14), %rdx
.Ltmp4113:
	.loc	8 551 14
	vmovups	%xmm0, (%rdx,%rax,4)
.Ltmp4114:
	.loc	1 1003 29
	movq	1352(%r14), %rsi
.Ltmp4115:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_460
.Ltmp4116:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4117:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm15, %xmm14, %xmm4
	vsubps	%xmm0, %xmm4, %xmm0
.Ltmp4118:
	.loc	1 1003 29 is_stmt 1
	movq	1344(%r14), %rdx
.Ltmp4119:
	.loc	8 551 14
	vmovups	%xmm0, (%rdx,%rax,4)
	movq	1104(%r14), %r8
.Ltmp4120:
	.loc	1 877 35
	addq	%rcx, %r8
.Ltmp4121:
	.loc	1 857 8
	cmpq	%r15, %r8
	movl	$0, %eax
	cmovaeq	%r15, %rax
.Ltmp4122:
	.loc	1 1006 34
	movq	8(%r14), %rsi
.Ltmp4123:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp4124:
	.loc	1 877 30
	shlq	$2, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_488
	.loc	1 0 25
	movq	1112(%r14), %rax
	.loc	1 877 35
	addq	%rcx, %rax
.Ltmp4125:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp4126:
	.loc	1 877 30
	leaq	1(,%rax,4), %rbp
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	.loc	1 0 25
	movq	%rbx, 960(%rsp)
	movq	1120(%r14), %rax
	.loc	1 877 35
	addq	%rcx, %rax
.Ltmp4127:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp4128:
	.loc	1 877 30
	leaq	2(,%rax,4), %rbx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_508
	.loc	1 0 25
	movq	1128(%r14), %rax
	.loc	1 877 35
	addq	%rcx, %rax
.Ltmp4129:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp4130:
	.loc	1 877 30
	leaq	3(,%rax,4), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_503
.Ltmp4131:
	.loc	1 1008 34 is_stmt 1
	movq	24(%r14), %rsi
.Ltmp4132:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB34_488
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	cmpq	%rsi, %rbx
	jae	.LBB34_508
	cmpq	%rsi, %r12
	jae	.LBB34_503
.Ltmp4133:
	.loc	1 0 25 is_stmt 0
	movq	2432(%r14), %rax
.Ltmp4134:
	.loc	1 877 35
	addq	%rcx, %rax
.Ltmp4135:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
.Ltmp4136:
	.loc	1 1010 34
	movq	1336(%r14), %rsi
.Ltmp4137:
	.loc	1 857 8
	subq	%rdx, %rax
.Ltmp4138:
	.loc	1 877 30
	shlq	$2, %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_494
	.loc	1 0 25
	movq	2440(%r14), %rdx
	.loc	1 877 35
	addq	%rcx, %rdx
.Ltmp4139:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rdx
	movl	$0, %r10d
	cmovaeq	%r15, %r10
	subq	%r10, %rdx
.Ltmp4140:
	.loc	1 877 30
	leaq	1(,%rdx,4), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB34_493
	.loc	1 0 25
	movq	2448(%r14), %r10
	.loc	1 877 35
	addq	%rcx, %r10
.Ltmp4141:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %r10
	movl	$0, %r11d
	cmovaeq	%r15, %r11
	subq	%r11, %r10
.Ltmp4142:
	.loc	1 877 30
	leaq	2(,%r10,4), %r11
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_545
	.loc	1 0 25
	movq	2456(%r14), %r10
	.loc	1 877 35
	addq	%rcx, %r10
.Ltmp4143:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %r10
	movq	%r14, %r13
	movl	$0, %r14d
	cmovaeq	%r15, %r14
	subq	%r14, %r10
.Ltmp4144:
	.loc	1 877 30
	leaq	3(,%r10,4), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_514
.Ltmp4145:
	.loc	1 1012 34 is_stmt 1
	movq	1352(%r13), %rsi
.Ltmp4146:
	.loc	1 877 25
	cmpq	%rsi, %rax
	jae	.LBB34_494
	cmpq	%rsi, %rdx
	jae	.LBB34_493
	cmpq	%rsi, %r11
	jae	.LBB34_545
	cmpq	%rsi, %r15
	jae	.LBB34_514
.Ltmp4147:
	.loc	1 0 25 is_stmt 0
	movq	%r13, %r14
	negq	%r9
	addq	%r9, %rcx
	incq	%rcx
	leaq	(,%rcx,4), %r10
.Ltmp4148:
	.loc	1 1047 36 is_stmt 1
	movq	8(%r13), %rsi
.Ltmp4149:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%r10, %r9
	jb	.LBB34_467
.Ltmp4150:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4151:
	.loc	1 1049 27
	movq	24(%r14), %rsi
.Ltmp4152:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%r10, %r9
	jb	.LBB34_468
.Ltmp4153:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4154:
	.loc	1 1050 35
	movq	1336(%r14), %rsi
.Ltmp4155:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%r10, %r9
	jb	.LBB34_469
.Ltmp4156:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4157:
	.loc	1 1052 27
	movq	1352(%r14), %rsi
.Ltmp4158:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%r10, %r9
	jb	.LBB34_470
.Ltmp4159:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4160:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm12, %xmm12, %xmm0
	vaddps	80(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm12
	vandps	%xmm0, %xmm12, %xmm4
	vbroadcastss	.LCPI34_2(%rip), %xmm14
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vaddps	%xmm7, %xmm7, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmulps	%xmm5, %xmm11, %xmm0
	vmovaps	48(%rsp), %xmm5
	vmulps	%xmm5, %xmm10, %xmm4
	vaddps	%xmm0, %xmm4, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm5, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm6, %xmm6, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4161:
	vaddps	%xmm3, %xmm3, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 16(%rsp)
.Ltmp4162:
	movq	(%r14), %rsi
	vmovd	(%rsi,%r8,4), %xmm0
	vpinsrd	$1, (%rsi,%rbp,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%rbx,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r12,4), %xmm0, %xmm0
.Ltmp4163:
	movq	16(%r14), %rsi
	vmovd	(%rsi,%r8,4), %xmm3
	vpinsrd	$1, (%rsi,%rbp,4), %xmm3, %xmm3
.Ltmp4164:
	vmulps	%xmm1, %xmm9, %xmm1
.Ltmp4165:
	vpinsrd	$2, (%rsi,%rbx,4), %xmm3, %xmm3
.Ltmp4166:
	vmulps	944(%rsp), %xmm8, %xmm4
	vaddps	%xmm1, %xmm4, %xmm1
.Ltmp4167:
	vpinsrd	$3, (%rsi,%r12,4), %xmm3, %xmm3
.Ltmp4168:
	vaddps	%xmm1, %xmm1, %xmm1
	vaddps	%xmm1, %xmm8, %xmm1
	vandps	%xmm1, %xmm12, %xmm4
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm1, %xmm4, %xmm1
	vmovaps	%xmm1, 864(%rsp)
	vaddps	%xmm2, %xmm2, %xmm1
	vaddps	144(%rsp), %xmm1, %xmm1
	vandps	%xmm1, %xmm12, %xmm2
	vcmpltps	%xmm14, %xmm2, %xmm2
	vmovaps	%xmm14, %xmm5
	vandnps	%xmm1, %xmm2, %xmm1
	vmovaps	%xmm1, 144(%rsp)
.Ltmp4169:
	vpand	%xmm3, %xmm12, %xmm1
	vpand	%xmm0, %xmm12, %xmm0
	vbroadcastss	.LCPI34_4(%rip), %xmm14
.Ltmp4170:
	vmaxps	%xmm14, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm7
	vmaxps	%xmm7, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm8
	vandps	%xmm0, %xmm8, %xmm2
	vbroadcastss	.LCPI34_32(%rip), %xmm9
	vorps	%xmm2, %xmm9, %xmm2
	vbroadcastss	.LCPI34_8(%rip), %xmm10
	vaddps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm2, %xmm11, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm9
	vaddps	%xmm3, %xmm9, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm8
	vaddps	%xmm3, %xmm8, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vsubps	176(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm6
	vcmpltps	%xmm0, %xmm6, %xmm2
	vaddps	%xmm6, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm4
	vmulps	%xmm4, %xmm3, %xmm3
	vblendvps	%xmm2, %xmm0, %xmm3, %xmm2
	vbroadcastss	.LCPI34_21(%rip), %xmm3
	vcmpleps	%xmm3, %xmm0, %xmm0
	vmulps	1136(%rsp), %xmm2, %xmm2
	vxorps	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm3, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vminps	%xmm3, %xmm0, %xmm0
	vmovaps	848(%rsp), %xmm4
	vcmpltps	%xmm4, %xmm0, %xmm2
	vmovaps	1104(%rsp), %xmm3
	vblendvps	%xmm2, 1120(%rsp), %xmm3, %xmm2
	vsubps	%xmm0, %xmm4, %xmm3
	vmulps	%xmm2, %xmm3, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm5, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vaddps	240(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_24(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm5
	vsubps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vbroadcastss	.LCPI34_28(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vbroadcastss	.LCPI34_29(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vbroadcastss	.LCPI34_30(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vbroadcastss	.LCPI34_31(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
.Ltmp4171:
	vmaxps	%xmm14, %xmm1, %xmm0
	vmovaps	%xmm7, %xmm6
	vmaxps	%xmm7, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm7
	vandps	%xmm7, %xmm0, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm10
	vorps	%xmm1, %xmm10, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm11
	vaddps	%xmm1, %xmm11, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm13
	vmulps	%xmm1, %xmm13, %xmm4
	vbroadcastss	.LCPI34_10(%rip), %xmm14
	vaddps	%xmm4, %xmm14, %xmm4
	vmulps	%xmm4, %xmm1, %xmm4
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm1, %xmm4
	vbroadcastss	.LCPI34_12(%rip), %xmm2
	vaddps	%xmm2, %xmm4, %xmm4
	vmulps	%xmm4, %xmm1, %xmm4
	vaddps	%xmm4, %xmm9, %xmm4
	vmulps	%xmm4, %xmm1, %xmm4
	vaddps	%xmm4, %xmm8, %xmm4
	vmulps	%xmm4, %xmm1, %xmm1
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vsubps	256(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm1
	vmulps	%xmm1, %xmm1, %xmm1
	vbroadcastss	.LCPI34_22(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vcmpltps	%xmm0, %xmm4, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm1, %xmm4
.Ltmp4172:
	movq	1328(%r14), %rsi
	vbroadcastss	.LCPI34_32(%rip), %xmm8
.Ltmp4173:
	vaddps	%xmm3, %xmm8, %xmm1
	vbroadcastss	.LCPI34_33(%rip), %xmm9
	vaddps	%xmm5, %xmm9, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vmovaps	%xmm1, 944(%rsp)
.Ltmp4174:
	vmovd	(%rsi,%rax,4), %xmm2
	vpinsrd	$1, (%rsi,%rdx,4), %xmm2, %xmm2
	vpinsrd	$2, (%rsi,%r11,4), %xmm2, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm1
.Ltmp4175:
	vcmpleps	%xmm1, %xmm0, %xmm0
	vmulps	1088(%rsp), %xmm4, %xmm2
	vxorps	%xmm4, %xmm4, %xmm4
	vpcmpgtd	%xmm0, %xmm4, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm4, %xmm0, %xmm0
	vmovaps	832(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm0, %xmm2
	vmovaps	1056(%rsp), %xmm4
	vblendvps	%xmm2, 1072(%rsp), %xmm4, %xmm2
	vsubps	%xmm0, %xmm5, %xmm4
	vmulps	%xmm2, %xmm4, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 832(%rsp)
	vaddps	320(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_24(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm8
	vmaxps	%xmm8, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm2
	vsubps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm9
	vmulps	%xmm0, %xmm9, %xmm4
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_29(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_30(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_31(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
.Ltmp4176:
	vpinsrd	$3, (%rsi,%r15,4), %xmm3, %xmm3
.Ltmp4177:
	vpand	%xmm3, %xmm12, %xmm3
.Ltmp4178:
	vbroadcastss	.LCPI34_4(%rip), %xmm1
	vmaxps	%xmm1, %xmm3, %xmm3
	vmaxps	%xmm6, %xmm3, %xmm3
	vandps	%xmm7, %xmm3, %xmm4
	vorps	%xmm4, %xmm10, %xmm4
	vaddps	%xmm4, %xmm11, %xmm4
	vmulps	%xmm4, %xmm13, %xmm5
	vaddps	%xmm5, %xmm14, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_12(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_13(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_14(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm4
	vpsrld	$23, %xmm3, %xmm3
	vpor	.LCPI34_15(%rip), %xmm3, %xmm3
	vbroadcastss	.LCPI34_16(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vaddps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_32(%rip), %xmm1
.Ltmp4179:
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm7
	vaddps	%xmm7, %xmm2, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vbroadcastss	.LCPI34_17(%rip), %xmm14
.Ltmp4180:
	vmulps	%xmm3, %xmm14, %xmm3
	vbroadcastss	.LCPI34_18(%rip), %xmm4
	vmaxps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_19(%rip), %xmm13
	vminps	%xmm13, %xmm3, %xmm3
	vsubps	496(%rsp), %xmm3, %xmm3
.Ltmp4181:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm2
.Ltmp4182:
	vaddps	%xmm2, %xmm3, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm6
	vmulps	%xmm6, %xmm0, %xmm0
	vcmpltps	%xmm3, %xmm2, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm4
	vcmpleps	%xmm4, %xmm3, %xmm3
	vmulps	1040(%rsp), %xmm0, %xmm0
	vxorps	%xmm4, %xmm4, %xmm4
	vpcmpgtd	%xmm3, %xmm4, %xmm3
	vpandn	%xmm0, %xmm3, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm4, %xmm0, %xmm0
	vxorps	%xmm11, %xmm11, %xmm11
	vmovaps	816(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm0, %xmm3
	vmovaps	1200(%rsp), %xmm4
	vblendvps	%xmm3, 1216(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm5, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm10
	vcmpltps	%xmm10, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 816(%rsp)
	vaddps	560(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_24(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
	vmaxps	%xmm8, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm3
	vminps	%xmm3, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vmulps	%xmm0, %xmm9, %xmm4
	vbroadcastss	.LCPI34_28(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_29(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_30(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_31(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
.Ltmp4183:
	movq	1344(%r14), %rsi
.Ltmp4184:
	vaddps	%xmm1, %xmm0, %xmm0
	vaddps	%xmm7, %xmm3, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
.Ltmp4185:
	vmovd	(%rsi,%rax,4), %xmm0
	vpinsrd	$1, (%rsi,%rdx,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%r11,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r15,4), %xmm0, %xmm0
.Ltmp4186:
	vpand	%xmm0, %xmm12, %xmm0
.Ltmp4187:
	vbroadcastss	.LCPI34_4(%rip), %xmm4
	vmaxps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm4
	vmaxps	%xmm4, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm4
	vorps	.LCPI34_7(%rip), %xmm4, %xmm4
	vbroadcastss	.LCPI34_8(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vbroadcastss	.LCPI34_9(%rip), %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_10(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_11(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_13(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_14(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm4, %xmm4
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm0, %xmm14, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm4
	vmaxps	%xmm4, %xmm0, %xmm0
	vminps	%xmm13, %xmm0, %xmm0
	vsubps	576(%rsp), %xmm0, %xmm0
	vaddps	%xmm2, %xmm0, %xmm4
	vmulps	%xmm4, %xmm4, %xmm4
	vmulps	%xmm6, %xmm4, %xmm4
	vcmpltps	%xmm0, %xmm2, %xmm5
	vblendvps	%xmm5, %xmm0, %xmm4, %xmm4
	vbroadcastss	.LCPI34_21(%rip), %xmm2
	vcmpleps	%xmm2, %xmm0, %xmm0
	vmulps	1184(%rsp), %xmm4, %xmm4
	vpcmpgtd	%xmm0, %xmm11, %xmm0
	vpandn	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vmovaps	160(%rsp), %xmm6
	vcmpltps	%xmm6, %xmm0, %xmm4
	vmovaps	1152(%rsp), %xmm5
	vblendvps	%xmm4, 1168(%rsp), %xmm5, %xmm4
	vsubps	%xmm0, %xmm6, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm10, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm6
	vaddps	640(%rsp), %xmm6, %xmm0
	vbroadcastss	.LCPI34_24(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm8, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm0, %xmm9, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm1, %xmm0, %xmm0
	vaddps	%xmm7, %xmm4, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
.Ltmp4188:
	movq	(%r14), %rax
	vmovaps	944(%rsp), %xmm1
	vmulps	(%rax,%r10,4), %xmm1, %xmm1
	movq	16(%r14), %rax
	vmovaps	992(%rsp), %xmm2
	vmulps	(%rax,%r10,4), %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm1
.Ltmp4189:
	movq	1328(%r14), %rax
	vmulps	(%rax,%r10,4), %xmm3, %xmm2
	.loc	1 1052 27 is_stmt 1
	movq	1344(%r14), %rax
.Ltmp4190:
	.loc	9 88 14
	vmulps	(%rax,%r10,4), %xmm0, %xmm0
.Ltmp4191:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	888(%rsp), %r10
.Ltmp4192:
	.loc	8 551 14
	vmovups	%xmm1, (%r10,%rdi,4)
	movq	880(%rsp), %r11
.Ltmp4193:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r11,%rdi,4)
	movq	960(%rsp), %rbx
.Ltmp4194:
	.loc	1 0 0
	incq	%rbx
.Ltmp4195:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	cmpq	%rbx, 976(%rsp)
.Ltmp4196:
	.loc	3 900 12
	jne	.LBB34_208
	jmp	.LBB34_156
.Ltmp4197:
.LBB34_243:
	.loc	3 0 12 is_stmt 0
	vmovaps	80(%rsp), %xmm0
.Ltmp4198:
	.loc	1 1057 5 is_stmt 1
	vmovaps	%xmm0, 96(%r14)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 112(%r14)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 128(%r14)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 144(%r14)
	vmovaps	32(%rsp), %xmm0
	.loc	1 1058 5
	vmovaps	%xmm0, 1424(%r14)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%r14)
	vmovaps	864(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%r14)
	vmovaps	144(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%r14)
	vmovaps	816(%rsp), %xmm0
	.loc	1 1059 5
	vmovaps	%xmm0, 160(%r14)
	vmovaps	832(%rsp), %xmm0
	vmovaps	%xmm0, 176(%r14)
	vmovdqa	848(%rsp), %xmm0
	.loc	1 1060 5
	vmovdqa	%xmm0, 1488(%r14)
	vmovaps	%xmm7, 1504(%r14)
	.loc	1 1061 5
	movq	%r10, 2680(%r14)
	movq	928(%rsp), %rsi
	movq	936(%rsp), %r15
.Ltmp4199:
.LBB34_244:
	.loc	1 1086 11
	cmpq	%rsi, %r15
	jae	.LBB34_419
.LBB34_245:
	.loc	1 1087 42
	subq	%r15, %rsi
	.loc	1 1087 29 is_stmt 0
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %rbp
.Ltmp4200:
	.loc	1 1176 33 is_stmt 1
	vmovss	192(%r14), %xmm0
.Ltmp4201:
	.loc	1 1089 28
	vmovss	%xmm0, 176(%rsp)
.Ltmp4202:
	.loc	1 1176 33
	vmovss	352(%r14), %xmm0
.Ltmp4203:
	.loc	1 1089 28
	vmovss	%xmm0, 180(%rsp)
.Ltmp4204:
	.loc	1 1176 33
	vmovss	512(%r14), %xmm0
.Ltmp4205:
	.loc	1 1089 28
	vmovss	%xmm0, 184(%rsp)
.Ltmp4206:
	.loc	1 1176 33
	vmovss	672(%r14), %xmm0
.Ltmp4207:
	.loc	1 1089 28
	vmovss	%xmm0, 188(%rsp)
.Ltmp4208:
	.loc	1 1176 33
	vmovss	208(%r14), %xmm0
.Ltmp4209:
	.loc	1 1089 28
	vmovss	%xmm0, 192(%rsp)
.Ltmp4210:
	.loc	1 1176 33
	vmovss	368(%r14), %xmm0
.Ltmp4211:
	.loc	1 1089 28
	vmovss	%xmm0, 196(%rsp)
.Ltmp4212:
	.loc	1 1176 33
	vmovss	528(%r14), %xmm0
.Ltmp4213:
	.loc	1 1089 28
	vmovss	%xmm0, 200(%rsp)
.Ltmp4214:
	.loc	1 1176 33
	vmovss	688(%r14), %xmm0
.Ltmp4215:
	.loc	1 1089 28
	vmovss	%xmm0, 204(%rsp)
.Ltmp4216:
	.loc	1 1176 33
	vmovss	224(%r14), %xmm0
.Ltmp4217:
	.loc	1 1089 28
	vmovss	%xmm0, 208(%rsp)
.Ltmp4218:
	.loc	1 1176 33
	vmovss	384(%r14), %xmm0
.Ltmp4219:
	.loc	1 1089 28
	vmovss	%xmm0, 212(%rsp)
.Ltmp4220:
	.loc	1 1176 33
	vmovss	544(%r14), %xmm0
.Ltmp4221:
	.loc	1 1089 28
	vmovss	%xmm0, 216(%rsp)
.Ltmp4222:
	.loc	1 1176 33
	vmovss	704(%r14), %xmm0
.Ltmp4223:
	.loc	1 1089 28
	vmovss	%xmm0, 220(%rsp)
.Ltmp4224:
	.loc	1 1176 33
	vmovss	240(%r14), %xmm0
.Ltmp4225:
	.loc	1 1089 28
	vmovss	%xmm0, 224(%rsp)
.Ltmp4226:
	.loc	1 1176 33
	vmovss	400(%r14), %xmm0
.Ltmp4227:
	.loc	1 1089 28
	vmovss	%xmm0, 228(%rsp)
.Ltmp4228:
	.loc	1 1176 33
	vmovss	560(%r14), %xmm0
.Ltmp4229:
	.loc	1 1089 28
	vmovss	%xmm0, 232(%rsp)
.Ltmp4230:
	.loc	1 1176 33
	vmovss	720(%r14), %xmm0
.Ltmp4231:
	.loc	1 1089 28
	vmovss	%xmm0, 236(%rsp)
.Ltmp4232:
	.loc	1 1176 33
	vmovss	256(%r14), %xmm0
.Ltmp4233:
	.loc	1 1089 28
	vmovss	%xmm0, 240(%rsp)
.Ltmp4234:
	.loc	1 1176 33
	vmovss	416(%r14), %xmm0
.Ltmp4235:
	.loc	1 1089 28
	vmovss	%xmm0, 244(%rsp)
.Ltmp4236:
	.loc	1 1176 33
	vmovss	576(%r14), %xmm0
.Ltmp4237:
	.loc	1 1089 28
	vmovss	%xmm0, 248(%rsp)
.Ltmp4238:
	.loc	1 1176 33
	vmovss	736(%r14), %xmm0
.Ltmp4239:
	.loc	1 1089 28
	vmovss	%xmm0, 252(%rsp)
.Ltmp4240:
	.loc	1 1176 33
	vmovss	272(%r14), %xmm0
.Ltmp4241:
	.loc	1 1089 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp4242:
	.loc	1 1176 33
	vmovss	432(%r14), %xmm0
.Ltmp4243:
	.loc	1 1089 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp4244:
	.loc	1 1176 33
	vmovss	592(%r14), %xmm0
.Ltmp4245:
	.loc	1 1089 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp4246:
	.loc	1 1176 33
	vmovss	752(%r14), %xmm0
.Ltmp4247:
	.loc	1 1089 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp4248:
	.loc	1 1176 33
	vmovss	288(%r14), %xmm0
.Ltmp4249:
	.loc	1 1089 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp4250:
	.loc	1 1176 33
	vmovss	448(%r14), %xmm0
.Ltmp4251:
	.loc	1 1089 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp4252:
	.loc	1 1176 33
	vmovss	608(%r14), %xmm0
.Ltmp4253:
	.loc	1 1089 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp4254:
	.loc	1 1176 33
	vmovss	768(%r14), %xmm0
.Ltmp4255:
	.loc	1 1089 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp4256:
	.loc	1 1176 33
	vmovss	304(%r14), %xmm0
.Ltmp4257:
	.loc	1 1089 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp4258:
	.loc	1 1176 33
	vmovss	464(%r14), %xmm0
.Ltmp4259:
	.loc	1 1089 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp4260:
	.loc	1 1176 33
	vmovss	624(%r14), %xmm0
.Ltmp4261:
	.loc	1 1089 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp4262:
	.loc	1 1176 33
	vmovss	784(%r14), %xmm0
.Ltmp4263:
	.loc	1 1089 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp4264:
	.loc	1 1176 33
	vmovss	320(%r14), %xmm0
.Ltmp4265:
	.loc	1 1089 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp4266:
	.loc	1 1176 33
	vmovss	480(%r14), %xmm0
.Ltmp4267:
	.loc	1 1089 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp4268:
	.loc	1 1176 33
	vmovss	640(%r14), %xmm0
.Ltmp4269:
	.loc	1 1089 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp4270:
	.loc	1 1176 33
	vmovss	800(%r14), %xmm0
.Ltmp4271:
	.loc	1 1089 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp4272:
	.loc	1 1176 33
	vmovss	336(%r14), %xmm0
.Ltmp4273:
	.loc	1 1089 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp4274:
	.loc	1 1176 33
	vmovss	496(%r14), %xmm0
.Ltmp4275:
	.loc	1 1089 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp4276:
	.loc	1 1176 33
	vmovss	656(%r14), %xmm0
.Ltmp4277:
	.loc	1 1089 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp4278:
	.loc	1 1176 33
	vmovss	816(%r14), %xmm0
.Ltmp4279:
	.loc	1 1089 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp4280:
	.loc	1 1177 32
	vmovss	200(%r14), %xmm0
.Ltmp4281:
	.loc	1 1089 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp4282:
	.loc	1 1177 32
	vmovss	360(%r14), %xmm0
.Ltmp4283:
	.loc	1 1089 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp4284:
	.loc	1 1177 32
	vmovss	520(%r14), %xmm0
.Ltmp4285:
	.loc	1 1089 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp4286:
	.loc	1 1177 32
	vmovss	680(%r14), %xmm0
.Ltmp4287:
	.loc	1 1089 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp4288:
	.loc	1 1177 32
	vmovss	216(%r14), %xmm0
.Ltmp4289:
	.loc	1 1089 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp4290:
	.loc	1 1177 32
	vmovss	376(%r14), %xmm0
.Ltmp4291:
	.loc	1 1089 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp4292:
	.loc	1 1177 32
	vmovss	536(%r14), %xmm0
.Ltmp4293:
	.loc	1 1089 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp4294:
	.loc	1 1177 32
	vmovss	696(%r14), %xmm0
.Ltmp4295:
	.loc	1 1089 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp4296:
	.loc	1 1177 32
	vmovss	232(%r14), %xmm0
.Ltmp4297:
	.loc	1 1089 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp4298:
	.loc	1 1177 32
	vmovss	392(%r14), %xmm0
.Ltmp4299:
	.loc	1 1089 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp4300:
	.loc	1 1177 32
	vmovss	552(%r14), %xmm0
.Ltmp4301:
	.loc	1 1089 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp4302:
	.loc	1 1177 32
	vmovss	712(%r14), %xmm0
.Ltmp4303:
	.loc	1 1089 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp4304:
	.loc	1 1177 32
	vmovss	248(%r14), %xmm0
.Ltmp4305:
	.loc	1 1089 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp4306:
	.loc	1 1177 32
	vmovss	408(%r14), %xmm0
.Ltmp4307:
	.loc	1 1089 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp4308:
	.loc	1 1177 32
	vmovss	568(%r14), %xmm0
.Ltmp4309:
	.loc	1 1089 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp4310:
	.loc	1 1177 32
	vmovss	728(%r14), %xmm0
.Ltmp4311:
	.loc	1 1089 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp4312:
	.loc	1 1177 32
	vmovss	264(%r14), %xmm0
.Ltmp4313:
	.loc	1 1089 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp4314:
	.loc	1 1177 32
	vmovss	424(%r14), %xmm0
.Ltmp4315:
	.loc	1 1089 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp4316:
	.loc	1 1177 32
	vmovss	584(%r14), %xmm0
.Ltmp4317:
	.loc	1 1089 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp4318:
	.loc	1 1177 32
	vmovss	744(%r14), %xmm0
.Ltmp4319:
	.loc	1 1089 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp4320:
	.loc	1 1177 32
	vmovss	280(%r14), %xmm0
.Ltmp4321:
	.loc	1 1089 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp4322:
	.loc	1 1177 32
	vmovss	440(%r14), %xmm0
.Ltmp4323:
	.loc	1 1089 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp4324:
	.loc	1 1177 32
	vmovss	600(%r14), %xmm0
.Ltmp4325:
	.loc	1 1089 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp4326:
	.loc	1 1177 32
	vmovss	760(%r14), %xmm0
.Ltmp4327:
	.loc	1 1089 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp4328:
	.loc	1 1177 32
	vmovss	296(%r14), %xmm0
.Ltmp4329:
	.loc	1 1089 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp4330:
	.loc	1 1177 32
	vmovss	456(%r14), %xmm0
.Ltmp4331:
	.loc	1 1089 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp4332:
	.loc	1 1177 32
	vmovss	616(%r14), %xmm0
.Ltmp4333:
	.loc	1 1089 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp4334:
	.loc	1 1177 32
	vmovss	776(%r14), %xmm0
.Ltmp4335:
	.loc	1 1089 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp4336:
	.loc	1 1177 32
	vmovss	312(%r14), %xmm0
.Ltmp4337:
	.loc	1 1089 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp4338:
	.loc	1 1177 32
	vmovss	472(%r14), %xmm0
.Ltmp4339:
	.loc	1 1089 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp4340:
	.loc	1 1177 32
	vmovss	632(%r14), %xmm0
.Ltmp4341:
	.loc	1 1089 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp4342:
	.loc	1 1177 32
	vmovss	792(%r14), %xmm0
.Ltmp4343:
	.loc	1 1089 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp4344:
	.loc	1 1177 32
	vmovss	328(%r14), %xmm0
.Ltmp4345:
	.loc	1 1089 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp4346:
	.loc	1 1177 32
	vmovss	488(%r14), %xmm0
.Ltmp4347:
	.loc	1 1089 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp4348:
	.loc	1 1177 32
	vmovss	648(%r14), %xmm0
.Ltmp4349:
	.loc	1 1089 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp4350:
	.loc	1 1177 32
	vmovss	808(%r14), %xmm0
.Ltmp4351:
	.loc	1 1089 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp4352:
	.loc	1 1177 32
	vmovss	344(%r14), %xmm0
.Ltmp4353:
	.loc	1 1089 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp4354:
	.loc	1 1177 32
	vmovss	504(%r14), %xmm0
.Ltmp4355:
	.loc	1 1089 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp4356:
	.loc	1 1177 32
	vmovss	664(%r14), %xmm0
.Ltmp4357:
	.loc	1 1089 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp4358:
	.loc	1 1177 32
	vmovss	824(%r14), %xmm0
.Ltmp4359:
	.loc	1 1089 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp4360:
	.loc	1 1176 33
	vmovss	1520(%r14), %xmm0
.Ltmp4361:
	.loc	1 1089 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp4362:
	.loc	1 1176 33
	vmovss	1680(%r14), %xmm0
.Ltmp4363:
	.loc	1 1089 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp4364:
	.loc	1 1176 33
	vmovss	1840(%r14), %xmm0
.Ltmp4365:
	.loc	1 1089 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp4366:
	.loc	1 1176 33
	vmovss	2000(%r14), %xmm0
.Ltmp4367:
	.loc	1 1089 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp4368:
	.loc	1 1176 33
	vmovss	1536(%r14), %xmm0
.Ltmp4369:
	.loc	1 1089 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp4370:
	.loc	1 1176 33
	vmovss	1696(%r14), %xmm0
.Ltmp4371:
	.loc	1 1089 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp4372:
	.loc	1 1176 33
	vmovss	1856(%r14), %xmm0
.Ltmp4373:
	.loc	1 1089 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp4374:
	.loc	1 1176 33
	vmovss	2016(%r14), %xmm0
.Ltmp4375:
	.loc	1 1089 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp4376:
	.loc	1 1176 33
	vmovss	1552(%r14), %xmm0
.Ltmp4377:
	.loc	1 1089 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp4378:
	.loc	1 1176 33
	vmovss	1712(%r14), %xmm0
.Ltmp4379:
	.loc	1 1089 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp4380:
	.loc	1 1176 33
	vmovss	1872(%r14), %xmm0
.Ltmp4381:
	.loc	1 1089 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp4382:
	.loc	1 1176 33
	vmovss	2032(%r14), %xmm0
.Ltmp4383:
	.loc	1 1089 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp4384:
	.loc	1 1176 33
	vmovss	1568(%r14), %xmm0
.Ltmp4385:
	.loc	1 1089 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp4386:
	.loc	1 1176 33
	vmovss	1728(%r14), %xmm0
.Ltmp4387:
	.loc	1 1089 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp4388:
	.loc	1 1176 33
	vmovss	1888(%r14), %xmm0
.Ltmp4389:
	.loc	1 1089 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp4390:
	.loc	1 1176 33
	vmovss	2048(%r14), %xmm0
.Ltmp4391:
	.loc	1 1089 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp4392:
	.loc	1 1176 33
	vmovss	1584(%r14), %xmm0
.Ltmp4393:
	.loc	1 1089 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp4394:
	.loc	1 1176 33
	vmovss	1744(%r14), %xmm0
.Ltmp4395:
	.loc	1 1089 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp4396:
	.loc	1 1176 33
	vmovss	1904(%r14), %xmm0
.Ltmp4397:
	.loc	1 1089 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp4398:
	.loc	1 1176 33
	vmovss	2064(%r14), %xmm0
.Ltmp4399:
	.loc	1 1089 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp4400:
	.loc	1 1176 33
	vmovss	1600(%r14), %xmm0
.Ltmp4401:
	.loc	1 1089 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp4402:
	.loc	1 1176 33
	vmovss	1760(%r14), %xmm0
.Ltmp4403:
	.loc	1 1089 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp4404:
	.loc	1 1176 33
	vmovss	1920(%r14), %xmm0
.Ltmp4405:
	.loc	1 1089 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp4406:
	.loc	1 1176 33
	vmovss	2080(%r14), %xmm0
.Ltmp4407:
	.loc	1 1089 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp4408:
	.loc	1 1176 33
	vmovss	1616(%r14), %xmm0
.Ltmp4409:
	.loc	1 1089 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp4410:
	.loc	1 1176 33
	vmovss	1776(%r14), %xmm0
.Ltmp4411:
	.loc	1 1089 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp4412:
	.loc	1 1176 33
	vmovss	1936(%r14), %xmm0
.Ltmp4413:
	.loc	1 1089 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp4414:
	.loc	1 1176 33
	vmovss	2096(%r14), %xmm0
.Ltmp4415:
	.loc	1 1089 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp4416:
	.loc	1 1176 33
	vmovss	1632(%r14), %xmm0
.Ltmp4417:
	.loc	1 1089 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp4418:
	.loc	1 1176 33
	vmovss	1792(%r14), %xmm0
.Ltmp4419:
	.loc	1 1089 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp4420:
	.loc	1 1176 33
	vmovss	1952(%r14), %xmm0
.Ltmp4421:
	.loc	1 1089 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp4422:
	.loc	1 1176 33
	vmovss	2112(%r14), %xmm0
.Ltmp4423:
	.loc	1 1089 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp4424:
	.loc	1 1176 33
	vmovss	1648(%r14), %xmm0
.Ltmp4425:
	.loc	1 1089 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp4426:
	.loc	1 1176 33
	vmovss	1808(%r14), %xmm0
.Ltmp4427:
	.loc	1 1089 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp4428:
	.loc	1 1176 33
	vmovss	1968(%r14), %xmm0
.Ltmp4429:
	.loc	1 1089 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp4430:
	.loc	1 1176 33
	vmovss	2128(%r14), %xmm0
.Ltmp4431:
	.loc	1 1089 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp4432:
	.loc	1 1176 33
	vmovss	1664(%r14), %xmm0
.Ltmp4433:
	.loc	1 1089 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp4434:
	.loc	1 1176 33
	vmovss	1824(%r14), %xmm0
.Ltmp4435:
	.loc	1 1089 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp4436:
	.loc	1 1176 33
	vmovss	1984(%r14), %xmm0
.Ltmp4437:
	.loc	1 1089 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp4438:
	.loc	1 1176 33
	vmovss	2144(%r14), %xmm0
.Ltmp4439:
	.loc	1 1089 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp4440:
	.loc	1 1177 32
	vmovss	1528(%r14), %xmm0
.Ltmp4441:
	.loc	1 1089 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp4442:
	.loc	1 1177 32
	vmovss	1688(%r14), %xmm0
.Ltmp4443:
	.loc	1 1089 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp4444:
	.loc	1 1177 32
	vmovss	1848(%r14), %xmm0
.Ltmp4445:
	.loc	1 1089 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp4446:
	.loc	1 1177 32
	vmovss	2008(%r14), %xmm0
.Ltmp4447:
	.loc	1 1089 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp4448:
	.loc	1 1177 32
	vmovss	1544(%r14), %xmm0
.Ltmp4449:
	.loc	1 1089 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp4450:
	.loc	1 1177 32
	vmovss	1704(%r14), %xmm0
.Ltmp4451:
	.loc	1 1089 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp4452:
	.loc	1 1177 32
	vmovss	1864(%r14), %xmm0
.Ltmp4453:
	.loc	1 1089 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp4454:
	.loc	1 1177 32
	vmovss	2024(%r14), %xmm0
.Ltmp4455:
	.loc	1 1089 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp4456:
	.loc	1 1177 32
	vmovss	1560(%r14), %xmm0
.Ltmp4457:
	.loc	1 1089 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp4458:
	.loc	1 1177 32
	vmovss	1720(%r14), %xmm0
.Ltmp4459:
	.loc	1 1089 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp4460:
	.loc	1 1177 32
	vmovss	1880(%r14), %xmm0
.Ltmp4461:
	.loc	1 1089 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp4462:
	.loc	1 1177 32
	vmovss	2040(%r14), %xmm0
.Ltmp4463:
	.loc	1 1089 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp4464:
	.loc	1 1177 32
	vmovss	1576(%r14), %xmm0
.Ltmp4465:
	.loc	1 1089 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp4466:
	.loc	1 1177 32
	vmovss	1736(%r14), %xmm0
.Ltmp4467:
	.loc	1 1089 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp4468:
	.loc	1 1177 32
	vmovss	1896(%r14), %xmm0
.Ltmp4469:
	.loc	1 1089 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp4470:
	.loc	1 1177 32
	vmovss	2056(%r14), %xmm0
.Ltmp4471:
	.loc	1 1089 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp4472:
	.loc	1 1177 32
	vmovss	1592(%r14), %xmm0
.Ltmp4473:
	.loc	1 1089 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp4474:
	.loc	1 1177 32
	vmovss	1752(%r14), %xmm0
.Ltmp4475:
	.loc	1 1089 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp4476:
	.loc	1 1177 32
	vmovss	1912(%r14), %xmm0
.Ltmp4477:
	.loc	1 1089 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp4478:
	.loc	1 1177 32
	vmovss	2072(%r14), %xmm0
.Ltmp4479:
	.loc	1 1089 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp4480:
	.loc	1 1177 32
	vmovss	1608(%r14), %xmm0
.Ltmp4481:
	.loc	1 1089 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp4482:
	.loc	1 1177 32
	vmovss	1768(%r14), %xmm0
.Ltmp4483:
	.loc	1 1089 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp4484:
	.loc	1 1177 32
	vmovss	1928(%r14), %xmm0
.Ltmp4485:
	.loc	1 1089 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp4486:
	.loc	1 1177 32
	vmovss	2088(%r14), %xmm0
.Ltmp4487:
	.loc	1 1089 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp4488:
	.loc	1 1177 32
	vmovss	1624(%r14), %xmm0
.Ltmp4489:
	.loc	1 1089 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp4490:
	.loc	1 1177 32
	vmovss	1784(%r14), %xmm0
.Ltmp4491:
	.loc	1 1089 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp4492:
	.loc	1 1177 32
	vmovss	1944(%r14), %xmm0
.Ltmp4493:
	.loc	1 1089 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp4494:
	.loc	1 1177 32
	vmovss	2104(%r14), %xmm0
.Ltmp4495:
	.loc	1 1089 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp4496:
	.loc	1 1177 32
	vmovss	1640(%r14), %xmm0
.Ltmp4497:
	.loc	1 1089 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp4498:
	.loc	1 1177 32
	vmovss	1800(%r14), %xmm0
.Ltmp4499:
	.loc	1 1089 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp4500:
	.loc	1 1177 32
	vmovss	1960(%r14), %xmm0
.Ltmp4501:
	.loc	1 1089 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp4502:
	.loc	1 1177 32
	vmovss	2120(%r14), %xmm0
.Ltmp4503:
	.loc	1 1089 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp4504:
	.loc	1 1177 32
	vmovss	1656(%r14), %xmm0
.Ltmp4505:
	.loc	1 1089 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp4506:
	.loc	1 1177 32
	vmovss	1816(%r14), %xmm0
.Ltmp4507:
	.loc	1 1089 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp4508:
	.loc	1 1177 32
	vmovss	1976(%r14), %xmm0
.Ltmp4509:
	.loc	1 1089 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp4510:
	.loc	1 1177 32
	vmovss	2136(%r14), %xmm0
.Ltmp4511:
	.loc	1 1089 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp4512:
	.loc	1 1177 32
	vmovss	1672(%r14), %xmm0
.Ltmp4513:
	.loc	1 1089 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp4514:
	.loc	1 1177 32
	vmovss	1832(%r14), %xmm0
.Ltmp4515:
	.loc	1 1089 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp4516:
	.loc	1 1177 32
	vmovss	1992(%r14), %xmm0
.Ltmp4517:
	.loc	1 1089 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp4518:
	.loc	1 1177 32
	vmovss	2152(%r14), %xmm0
.Ltmp4519:
	.loc	1 1089 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp4520:
	.loc	1 1091 31
	leaq	1248(%rsp), %rdi
	movq	%r14, %rsi
	movl	908(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	1344(%rsp), %rdi
	movq	1016(%rsp), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovaps	1248(%rsp), %xmm0
	vmovaps	%xmm0, 1136(%rsp)
	vmovaps	1264(%rsp), %xmm0
	vmovaps	%xmm0, 1120(%rsp)
	vmovaps	1280(%rsp), %xmm0
	vmovaps	%xmm0, 1104(%rsp)
	vmovaps	1296(%rsp), %xmm0
	vmovaps	%xmm0, 1088(%rsp)
	vmovaps	1312(%rsp), %xmm0
	vmovaps	%xmm0, 1072(%rsp)
	vmovaps	1328(%rsp), %xmm0
	vmovaps	%xmm0, 1056(%rsp)
	vmovaps	1344(%rsp), %xmm0
	vmovaps	%xmm0, 1040(%rsp)
	vmovaps	1360(%rsp), %xmm0
	vmovaps	%xmm0, 1216(%rsp)
	vmovaps	1376(%rsp), %xmm0
	vmovaps	%xmm0, 1200(%rsp)
	vmovaps	1392(%rsp), %xmm0
	vmovaps	%xmm0, 1184(%rsp)
	vmovaps	1408(%rsp), %xmm0
	vmovaps	%xmm0, 1168(%rsp)
	vmovaps	1424(%rsp), %xmm0
	vmovaps	%xmm0, 1152(%rsp)
.Ltmp4521:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%rbp), %rcx
	shlq	$2, %r15
	leaq	(,%rcx,4), %rsi
	.loc	1 1094 12 is_stmt 1
	testb	$1, %r12b
	movq	%rbp, 976(%rsp)
	movq	%rcx, 936(%rsp)
	je	.LBB34_290
.Ltmp4522:
	.loc	38 1050 16
	cmpq	%r15, %rsi
	movq	912(%rsp), %rax
.Ltmp4523:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_480
.Ltmp4524:
	.loc	48 451 16 is_stmt 1
	cmpq	120(%rsp), %rsi
	ja	.LBB34_480
.Ltmp4525:
	.loc	48 451 16 is_stmt 0
	cmpq	112(%rsp), %rsi
	ja	.LBB34_486
.Ltmp4526:
	.loc	1 972 27 is_stmt 1
	vmovaps	96(%r14), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	112(%r14), %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmovaps	128(%r14), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	144(%r14), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4527:
	.loc	1 973 26
	vmovaps	1424(%r14), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	1440(%r14), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	1456(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovaps	1472(%r14), %xmm0
	vmovaps	%xmm0, 864(%rsp)
.Ltmp4528:
	.loc	1 974 25
	vmovaps	160(%r14), %xmm0
	vmovaps	%xmm0, 816(%rsp)
	vmovaps	176(%r14), %xmm0
	vmovaps	%xmm0, 832(%rsp)
.Ltmp4529:
	.loc	1 975 24
	vmovaps	1488(%r14), %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vmovaps	1504(%r14), %xmm7
.Ltmp4530:
	.loc	1 976 24
	movq	2680(%r14), %r11
.Ltmp4531:
	.loc	2 1916 50
	testq	%rbp, %rbp
	je	.LBB34_286
.Ltmp4532:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rbp,4), %rcx
	movq	%rcx, 104(%rsp)
	movq	920(%rsp), %rcx
	leaq	(%rcx,%r15,4), %r10
	leaq	(%rax,%r15,4), %rbx
.Ltmp4533:
	.loc	3 900 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %rbp
	movq	%rbp, 896(%rsp)
	xorl	%edi, %edi
	xorl	%r15d, %r15d
	movq	%r10, 888(%rsp)
	movq	%rbx, 880(%rsp)
.Ltmp4534:
	.loc	3 0 12 is_stmt 0
.Ltmp4535:
	.p2align	4
.LBB34_251:
	.loc	1 981 21 is_stmt 1
	vmovaps	176(%rsp), %xmm0
	vmovaps	192(%rsp), %xmm1
	vmovaps	208(%rsp), %xmm2
.Ltmp4536:
	.loc	9 36 14
	vaddps	336(%rsp), %xmm0, %xmm0
.Ltmp4537:
	.loc	1 983 21
	vmovaps	496(%rsp), %xmm3
	.loc	1 980 17
	vmovaps	%xmm0, 176(%rsp)
.Ltmp4538:
	.loc	9 36 14
	vaddps	656(%rsp), %xmm3, %xmm0
.Ltmp4539:
	.loc	1 982 17
	vmovaps	%xmm0, 496(%rsp)
.Ltmp4540:
	.loc	9 36 14
	vaddps	352(%rsp), %xmm1, %xmm0
.Ltmp4541:
	.loc	1 980 17
	vmovaps	%xmm0, 192(%rsp)
	.loc	1 983 21
	vmovaps	512(%rsp), %xmm0
.Ltmp4542:
	.loc	9 36 14
	vaddps	672(%rsp), %xmm0, %xmm0
.Ltmp4543:
	.loc	1 982 17
	vmovaps	%xmm0, 512(%rsp)
.Ltmp4544:
	.loc	9 36 14
	vaddps	368(%rsp), %xmm2, %xmm0
.Ltmp4545:
	.loc	1 980 17
	vmovaps	%xmm0, 208(%rsp)
	.loc	1 983 21
	vmovaps	528(%rsp), %xmm0
.Ltmp4546:
	.loc	9 36 14
	vaddps	688(%rsp), %xmm0, %xmm0
.Ltmp4547:
	.loc	1 982 17
	vmovaps	%xmm0, 528(%rsp)
	.loc	1 981 21
	vmovaps	224(%rsp), %xmm0
.Ltmp4548:
	.loc	9 36 14
	vaddps	384(%rsp), %xmm0, %xmm0
.Ltmp4549:
	.loc	1 980 17
	vmovaps	%xmm0, 224(%rsp)
	.loc	1 983 21
	vmovaps	544(%rsp), %xmm0
.Ltmp4550:
	.loc	9 36 14
	vaddps	704(%rsp), %xmm0, %xmm0
.Ltmp4551:
	.loc	1 982 17
	vmovaps	%xmm0, 544(%rsp)
	.loc	1 981 21
	vmovaps	240(%rsp), %xmm0
.Ltmp4552:
	.loc	9 36 14
	vaddps	400(%rsp), %xmm0, %xmm0
.Ltmp4553:
	.loc	1 980 17
	vmovaps	%xmm0, 240(%rsp)
	.loc	1 983 21
	vmovaps	560(%rsp), %xmm0
.Ltmp4554:
	.loc	9 36 14
	vaddps	720(%rsp), %xmm0, %xmm0
.Ltmp4555:
	.loc	1 982 17
	vmovaps	%xmm0, 560(%rsp)
	.loc	1 981 21
	vmovaps	256(%rsp), %xmm0
.Ltmp4556:
	.loc	9 36 14
	vaddps	416(%rsp), %xmm0, %xmm0
.Ltmp4557:
	.loc	1 980 17
	vmovaps	%xmm0, 256(%rsp)
	.loc	1 983 21
	vmovaps	576(%rsp), %xmm0
.Ltmp4558:
	.loc	9 36 14
	vaddps	736(%rsp), %xmm0, %xmm0
.Ltmp4559:
	.loc	1 982 17
	vmovaps	%xmm0, 576(%rsp)
	.loc	1 981 21
	vmovaps	272(%rsp), %xmm0
.Ltmp4560:
	.loc	9 36 14
	vaddps	432(%rsp), %xmm0, %xmm0
.Ltmp4561:
	.loc	1 980 17
	vmovaps	%xmm0, 272(%rsp)
	.loc	1 983 21
	vmovaps	592(%rsp), %xmm0
.Ltmp4562:
	.loc	9 36 14
	vaddps	752(%rsp), %xmm0, %xmm0
.Ltmp4563:
	.loc	1 982 17
	vmovaps	%xmm0, 592(%rsp)
	.loc	1 981 21
	vmovaps	288(%rsp), %xmm0
.Ltmp4564:
	.loc	9 36 14
	vaddps	448(%rsp), %xmm0, %xmm0
.Ltmp4565:
	.loc	1 980 17
	vmovaps	%xmm0, 288(%rsp)
	.loc	1 983 21
	vmovaps	608(%rsp), %xmm0
.Ltmp4566:
	.loc	9 36 14
	vaddps	768(%rsp), %xmm0, %xmm0
.Ltmp4567:
	.loc	1 982 17
	vmovaps	%xmm0, 608(%rsp)
	.loc	1 981 21
	vmovaps	304(%rsp), %xmm0
.Ltmp4568:
	.loc	9 36 14
	vaddps	464(%rsp), %xmm0, %xmm0
.Ltmp4569:
	.loc	1 980 17
	vmovaps	%xmm0, 304(%rsp)
	.loc	1 983 21
	vmovaps	624(%rsp), %xmm0
.Ltmp4570:
	.loc	9 36 14
	vaddps	784(%rsp), %xmm0, %xmm0
.Ltmp4571:
	.loc	1 982 17
	vmovaps	%xmm0, 624(%rsp)
	.loc	1 981 21
	vmovaps	320(%rsp), %xmm0
.Ltmp4572:
	.loc	9 36 14
	vaddps	480(%rsp), %xmm0, %xmm0
.Ltmp4573:
	.loc	1 980 17
	vmovaps	%xmm0, 320(%rsp)
	.loc	1 983 21
	vmovaps	640(%rsp), %xmm0
.Ltmp4574:
	.loc	9 36 14
	vaddps	800(%rsp), %xmm0, %xmm0
.Ltmp4575:
	.loc	1 982 17
	vmovaps	%xmm0, 640(%rsp)
.Ltmp4576:
	.loc	1 987 28
	leaq	1(%r11), %rax
.Ltmp4577:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %r9d
	cmovaeq	%r13, %r9
.Ltmp4578:
	.loc	48 568 12
	cmpq	104(%rsp), %rdi
	ja	.LBB34_456
.Ltmp4579:
	.loc	48 438 16
	cmpq	%r15, 896(%rsp)
	je	.LBB34_449
.Ltmp4580:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r11,4), %rax
.Ltmp4581:
	.loc	1 1000 29 is_stmt 1
	movq	8(%r14), %rsi
.Ltmp4582:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_457
.Ltmp4583:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4584:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm7, 1024(%rsp)
	vmovups	(%r10,%rdi,4), %xmm0
.Ltmp4585:
	vmovups	(%rbx,%rdi,4), %xmm6
.Ltmp4586:
	vmovaps	32(%r14), %xmm10
	vmovaps	48(%r14), %xmm11
	vmovaps	64(%r14), %xmm1
	vmovaps	1360(%r14), %xmm13
	vmovaps	1376(%r14), %xmm9
	vmovaps	1392(%r14), %xmm14
	vmovaps	64(%rsp), %xmm8
	vsubps	%xmm8, %xmm0, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vmovaps	80(%rsp), %xmm7
	vmulps	%xmm7, %xmm10, %xmm4
	vaddps	%xmm3, %xmm4, %xmm4
	vaddps	%xmm4, %xmm7, %xmm5
	vmulps	%xmm7, %xmm11, %xmm3
	vmulps	%xmm1, %xmm2, %xmm2
	vaddps	%xmm2, %xmm3, %xmm3
	vaddps	%xmm3, %xmm8, %xmm2
	vmulps	80(%r14), %xmm5, %xmm12
	vmovaps	128(%rsp), %xmm7
	vsubps	%xmm7, %xmm2, %xmm2
	vmulps	48(%rsp), %xmm11, %xmm5
	vmulps	%xmm2, %xmm1, %xmm1
	vaddps	%xmm1, %xmm5, %xmm1
	vaddps	%xmm1, %xmm7, %xmm15
.Ltmp4587:
	vsubps	16(%rsp), %xmm6, %xmm5
	vmulps	%xmm5, %xmm9, %xmm7
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm13, 160(%rsp)
	vmulps	%xmm13, %xmm8, %xmm13
	vaddps	%xmm7, %xmm13, %xmm13
	vaddps	%xmm13, %xmm8, %xmm7
	vmulps	1408(%r14), %xmm7, %xmm7
.Ltmp4588:
	.loc	1 1000 29 is_stmt 1
	movq	(%r14), %rcx
.Ltmp4589:
	.loc	8 551 14
	vmovups	%xmm15, (%rcx,%rax,4)
.Ltmp4590:
	.loc	1 1001 30
	movq	24(%r14), %rsi
.Ltmp4591:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_471
.Ltmp4592:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4593:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm0, %xmm12, %xmm0
	vsubps	%xmm15, %xmm0, %xmm0
.Ltmp4594:
	.loc	1 1001 30 is_stmt 1
	movq	16(%r14), %rcx
.Ltmp4595:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4596:
	.loc	1 1002 28
	movq	1336(%r14), %rsi
.Ltmp4597:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_472
.Ltmp4598:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4599:
	.loc	1 0 0 is_stmt 0
	vmulps	32(%rsp), %xmm9, %xmm0
	vmulps	%xmm5, %xmm14, %xmm5
	vaddps	%xmm5, %xmm0, %xmm5
	vaddps	16(%rsp), %xmm5, %xmm0
	vmovaps	864(%rsp), %xmm15
	vsubps	%xmm15, %xmm0, %xmm0
	vmovaps	144(%rsp), %xmm8
	vmulps	%xmm9, %xmm8, %xmm12
	vmulps	%xmm0, %xmm14, %xmm14
	vaddps	%xmm14, %xmm12, %xmm14
	vaddps	%xmm14, %xmm15, %xmm12
.Ltmp4600:
	.loc	1 1002 28 is_stmt 1
	movq	1328(%r14), %rcx
.Ltmp4601:
	.loc	8 551 14
	vmovups	%xmm12, (%rcx,%rax,4)
.Ltmp4602:
	.loc	1 1003 29
	movq	1352(%r14), %rsi
.Ltmp4603:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_460
.Ltmp4604:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4605:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm7, %xmm6, %xmm6
	vsubps	%xmm12, %xmm6, %xmm6
.Ltmp4606:
	.loc	1 1003 29 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp4607:
	.loc	8 551 14
	vmovups	%xmm6, (%rcx,%rax,4)
	movq	1104(%r14), %rcx
.Ltmp4608:
	.loc	1 877 35
	addq	%r11, %rcx
.Ltmp4609:
	.loc	1 857 8
	cmpq	%r13, %rcx
	movl	$0, %eax
	cmovaeq	%r13, %rax
.Ltmp4610:
	.loc	1 1006 34
	movq	8(%r14), %rsi
.Ltmp4611:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp4612:
	.loc	1 877 30
	shlq	$2, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_492
	.loc	1 0 25
	movq	1112(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp4613:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rax
.Ltmp4614:
	.loc	1 877 30
	leaq	1(,%rax,4), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB34_491
	.loc	1 0 25
	movq	%r15, 944(%rsp)
	movq	1120(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp4615:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %rax
.Ltmp4616:
	.loc	1 877 30
	leaq	2(,%rax,4), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_506
	.loc	1 0 25
	movq	1128(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp4617:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %rax
.Ltmp4618:
	.loc	1 877 30
	leaq	3(,%rax,4), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_504
.Ltmp4619:
	.loc	1 1008 34 is_stmt 1
	movq	24(%r14), %rsi
.Ltmp4620:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB34_492
	cmpq	%rsi, %rdx
	jae	.LBB34_491
	cmpq	%rsi, %r12
	jae	.LBB34_506
	cmpq	%rsi, %r15
	jae	.LBB34_504
.Ltmp4621:
	.loc	1 0 25 is_stmt 0
	movq	2432(%r14), %r8
.Ltmp4622:
	.loc	1 877 35
	addq	%r11, %r8
.Ltmp4623:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %r8
	movl	$0, %eax
	cmovaeq	%r13, %rax
.Ltmp4624:
	.loc	1 1010 34
	movq	1336(%r14), %rsi
.Ltmp4625:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp4626:
	.loc	1 877 30
	shlq	$2, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_488
	.loc	1 0 25
	movq	2440(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp4627:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp4628:
	.loc	1 877 30
	leaq	1(,%rax,4), %rbp
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	.loc	1 0 25
	movq	2448(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp4629:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp4630:
	.loc	1 877 30
	leaq	2(,%rax,4), %r10
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r10
	jae	.LBB34_530
	.loc	1 0 25
	movq	2456(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp4631:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp4632:
	.loc	1 877 30
	leaq	3(,%rax,4), %rbx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_508
.Ltmp4633:
	.loc	1 1012 34 is_stmt 1
	movq	1352(%r14), %rsi
.Ltmp4634:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB34_488
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	cmpq	%rsi, %r10
	jae	.LBB34_530
	cmpq	%rsi, %rbx
	jae	.LBB34_508
.Ltmp4635:
	.loc	1 0 0 is_stmt 0
	negq	%r9
	addq	%r9, %r11
	incq	%r11
	leaq	(,%r11,4), %rax
	movq	8(%rsp), %rsi
.Ltmp4636:
	.loc	1 1047 36 is_stmt 1
	movq	8(%rsi), %rsi
.Ltmp4637:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_462
.Ltmp4638:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4639:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 1049 27 is_stmt 1
	movq	24(%rsi), %rsi
.Ltmp4640:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_463
.Ltmp4641:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4642:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
.Ltmp4643:
	.loc	1 1050 35 is_stmt 1
	movq	1336(%rsi), %rsi
.Ltmp4644:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_464
.Ltmp4645:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4646:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 1052 27 is_stmt 1
	movq	1352(%rsi), %rsi
.Ltmp4647:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_465
.Ltmp4648:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4649:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm4, %xmm4, %xmm4
	vaddps	80(%rsp), %xmm4, %xmm4
	vbroadcastss	.LCPI34_35(%rip), %xmm7
	vandps	%xmm7, %xmm4, %xmm6
	vbroadcastss	.LCPI34_2(%rip), %xmm12
	vcmpltps	%xmm12, %xmm6, %xmm6
	vandnps	%xmm4, %xmm6, %xmm4
	vmovaps	%xmm4, 80(%rsp)
	vaddps	%xmm3, %xmm3, %xmm3
	vaddps	64(%rsp), %xmm3, %xmm3
	vandps	%xmm7, %xmm3, %xmm4
	vcmpltps	%xmm12, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm3
	vmovaps	%xmm3, 64(%rsp)
	vmulps	%xmm2, %xmm11, %xmm2
	vmovaps	48(%rsp), %xmm4
	vmulps	%xmm4, %xmm10, %xmm3
	vaddps	%xmm2, %xmm3, %xmm2
	vaddps	%xmm2, %xmm2, %xmm2
	vaddps	%xmm2, %xmm4, %xmm2
	vandps	%xmm7, %xmm2, %xmm3
	vcmpltps	%xmm12, %xmm3, %xmm3
	vandnps	%xmm2, %xmm3, %xmm2
	vmovaps	%xmm2, 48(%rsp)
	vaddps	%xmm1, %xmm1, %xmm1
	vaddps	128(%rsp), %xmm1, %xmm1
	vandps	%xmm7, %xmm1, %xmm2
	vcmpltps	%xmm12, %xmm2, %xmm2
	vandnps	%xmm1, %xmm2, %xmm1
	vmovaps	%xmm1, 128(%rsp)
.Ltmp4650:
	vaddps	%xmm13, %xmm13, %xmm1
	vaddps	32(%rsp), %xmm1, %xmm1
	vandps	%xmm7, %xmm1, %xmm2
	vcmpltps	%xmm12, %xmm2, %xmm2
	vandnps	%xmm1, %xmm2, %xmm1
	vmovaps	%xmm1, 32(%rsp)
	vaddps	%xmm5, %xmm5, %xmm1
	vaddps	16(%rsp), %xmm1, %xmm1
	vandps	%xmm7, %xmm1, %xmm2
	vcmpltps	%xmm12, %xmm2, %xmm2
	vandnps	%xmm1, %xmm2, %xmm1
	vmovaps	%xmm1, 16(%rsp)
	movq	8(%rsp), %r14
.Ltmp4651:
	movq	(%r14), %rsi
	vmovd	(%rsi,%rcx,4), %xmm1
	vpinsrd	$1, (%rsi,%rdx,4), %xmm1, %xmm1
	vpinsrd	$2, (%rsi,%r12,4), %xmm1, %xmm1
	vpinsrd	$3, (%rsi,%r15,4), %xmm1, %xmm1
.Ltmp4652:
	movq	16(%r14), %rsi
	vmovd	(%rsi,%rcx,4), %xmm2
	vpinsrd	$1, (%rsi,%rdx,4), %xmm2, %xmm2
.Ltmp4653:
	vmulps	%xmm0, %xmm9, %xmm0
.Ltmp4654:
	vpinsrd	$2, (%rsi,%r12,4), %xmm2, %xmm2
.Ltmp4655:
	vmulps	160(%rsp), %xmm8, %xmm3
	vaddps	%xmm0, %xmm3, %xmm0
.Ltmp4656:
	vpinsrd	$3, (%rsi,%r15,4), %xmm2, %xmm2
.Ltmp4657:
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm8, %xmm0
	vandps	%xmm7, %xmm0, %xmm3
	vcmpltps	%xmm12, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vaddps	%xmm14, %xmm14, %xmm0
	vaddps	%xmm0, %xmm15, %xmm0
.Ltmp4658:
	movq	1328(%r14), %rcx
	vmovd	(%rcx,%r8,4), %xmm3
	vpinsrd	$1, (%rcx,%rbp,4), %xmm3, %xmm3
	vpinsrd	$2, (%rcx,%r10,4), %xmm3, %xmm3
	vpinsrd	$3, (%rcx,%rbx,4), %xmm3, %xmm3
.Ltmp4659:
	movq	1344(%r14), %rcx
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rbp,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%r10,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rbx,4), %xmm4, %xmm4
.Ltmp4660:
	vandps	%xmm7, %xmm0, %xmm5
	vcmpltps	%xmm12, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm0
	vmovaps	%xmm0, 864(%rsp)
.Ltmp4661:
	vpand	%xmm7, %xmm1, %xmm0
	vpand	%xmm7, %xmm3, %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
.Ltmp4662:
	vpand	%xmm7, %xmm2, %xmm1
	vpand	%xmm7, %xmm4, %xmm2
	vmaxps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_4(%rip), %xmm12
.Ltmp4663:
	vmaxps	%xmm12, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm6
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm8
	vandps	%xmm0, %xmm8, %xmm2
	vbroadcastss	.LCPI34_32(%rip), %xmm9
	vorps	%xmm2, %xmm9, %xmm2
	vbroadcastss	.LCPI34_8(%rip), %xmm10
	vaddps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm2, %xmm11, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm14
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vsubps	176(%rsp), %xmm0, %xmm2
	vbroadcastss	.LCPI34_20(%rip), %xmm9
	vaddps	%xmm2, %xmm9, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm2, %xmm9, %xmm4
	vblendvps	%xmm4, %xmm2, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm8
	vcmpleps	%xmm8, %xmm2, %xmm2
	vmulps	1136(%rsp), %xmm3, %xmm3
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%xmm2, %xmm0, %xmm2
	vpandn	%xmm3, %xmm2, %xmm2
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm2, %xmm2
	vminps	%xmm0, %xmm2, %xmm2
	vmovaps	816(%rsp), %xmm0
	vcmpltps	%xmm0, %xmm2, %xmm3
	vmovaps	1104(%rsp), %xmm4
	vblendvps	%xmm3, 1120(%rsp), %xmm4, %xmm3
	vsubps	%xmm2, %xmm0, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm2, %xmm2
	vandps	%xmm7, %xmm2, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm0
	vcmpltps	%xmm0, %xmm3, %xmm3
	vandnps	%xmm2, %xmm3, %xmm0
.Ltmp4664:
	vmaxps	%xmm12, %xmm1, %xmm1
	vmaxps	%xmm6, %xmm1, %xmm1
	vandps	.LCPI34_6(%rip), %xmm1, %xmm2
	vorps	.LCPI34_7(%rip), %xmm2, %xmm2
	vaddps	%xmm2, %xmm10, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm1, %xmm1
	vpor	.LCPI34_15(%rip), %xmm1, %xmm1
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm1, %xmm1
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm1, %xmm1
	vmovaps	%xmm1, 992(%rsp)
	vsubps	256(%rsp), %xmm1, %xmm1
	vaddps	%xmm1, %xmm9, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm1, %xmm9, %xmm4
	vblendvps	%xmm4, %xmm1, %xmm3, %xmm3
	vcmpleps	%xmm8, %xmm1, %xmm1
	vmulps	1088(%rsp), %xmm3, %xmm3
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%xmm1, %xmm2, %xmm1
	vpandn	%xmm3, %xmm1, %xmm1
	vmovaps	%xmm0, 816(%rsp)
.Ltmp4665:
	vaddps	240(%rsp), %xmm0, %xmm3
	vbroadcastss	.LCPI34_24(%rip), %xmm10
	vmulps	%xmm3, %xmm10, %xmm3
	vbroadcastss	.LCPI34_25(%rip), %xmm8
	vmaxps	%xmm8, %xmm3, %xmm3
	vbroadcastss	.LCPI34_26(%rip), %xmm13
	vminps	%xmm13, %xmm3, %xmm3
	vroundps	$9, %xmm3, %xmm4
	vsubps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_27(%rip), %xmm14
	vmulps	%xmm3, %xmm14, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm12
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm9
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm11
	vaddps	%xmm5, %xmm11, %xmm5
	vbroadcastss	.LCPI34_23(%rip), %xmm15
.Ltmp4666:
	vmaxps	%xmm15, %xmm1, %xmm1
	vminps	%xmm2, %xmm1, %xmm1
	vmovaps	832(%rsp), %xmm0
	vcmpltps	%xmm0, %xmm1, %xmm6
	vmovaps	1056(%rsp), %xmm10
	vblendvps	%xmm6, 1072(%rsp), %xmm10, %xmm6
.Ltmp4667:
	vmulps	%xmm5, %xmm3, %xmm3
.Ltmp4668:
	vsubps	%xmm1, %xmm0, %xmm5
	vmulps	%xmm6, %xmm5, %xmm5
	vaddps	%xmm5, %xmm1, %xmm1
	vmovaps	%xmm7, %xmm13
	vandps	%xmm7, %xmm1, %xmm5
	vbroadcastss	.LCPI34_2(%rip), %xmm14
	vcmpltps	%xmm14, %xmm5, %xmm5
	vandnps	%xmm1, %xmm5, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm6
.Ltmp4669:
	vaddps	%xmm6, %xmm3, %xmm1
	vbroadcastss	.LCPI34_33(%rip), %xmm10
	vaddps	%xmm4, %xmm10, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmovaps	%xmm0, 832(%rsp)
.Ltmp4670:
	vaddps	320(%rsp), %xmm0, %xmm4
.Ltmp4671:
	vmulps	%xmm3, %xmm1, %xmm0
	vmovaps	%xmm0, 960(%rsp)
	vbroadcastss	.LCPI34_24(%rip), %xmm7
.Ltmp4672:
	vmulps	%xmm7, %xmm4, %xmm3
	vmaxps	%xmm8, %xmm3, %xmm3
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm3, %xmm3
	vroundps	$9, %xmm3, %xmm4
	vsubps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_27(%rip), %xmm2
	vmulps	%xmm2, %xmm3, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm8
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm11, %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vaddps	%xmm6, %xmm3, %xmm3
	vaddps	%xmm4, %xmm10, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmovaps	160(%rsp), %xmm0
.Ltmp4673:
	vsubps	496(%rsp), %xmm0, %xmm5
.Ltmp4674:
	vmulps	%xmm4, %xmm3, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm6
.Ltmp4675:
	vaddps	%xmm6, %xmm5, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm11
	vmulps	%xmm3, %xmm11, %xmm3
	vcmpltps	%xmm5, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm5, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm10
	vcmpleps	%xmm10, %xmm5, %xmm4
	vmulps	1040(%rsp), %xmm3, %xmm3
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%xmm4, %xmm0, %xmm4
	vpandn	%xmm3, %xmm4, %xmm3
	vmaxps	%xmm15, %xmm3, %xmm3
	vminps	%xmm0, %xmm3, %xmm3
	vxorps	%xmm15, %xmm15, %xmm15
	vmovaps	848(%rsp), %xmm0
	vcmpltps	%xmm0, %xmm3, %xmm4
	vmovaps	1200(%rsp), %xmm5
	vblendvps	%xmm4, 1216(%rsp), %xmm5, %xmm4
	vsubps	%xmm3, %xmm0, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vandps	%xmm3, %xmm13, %xmm4
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vaddps	560(%rsp), %xmm0, %xmm3
	vmulps	%xmm7, %xmm3, %xmm3
	vbroadcastss	.LCPI34_25(%rip), %xmm14
	vmaxps	%xmm14, %xmm3, %xmm3
	vminps	%xmm1, %xmm3, %xmm3
	vroundps	$9, %xmm3, %xmm4
	vsubps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm2, %xmm3, %xmm5
	vmovaps	%xmm2, %xmm13
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vbroadcastss	.LCPI34_32(%rip), %xmm0
	vaddps	%xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_33(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm3, %xmm3
	vmovaps	992(%rsp), %xmm0
.Ltmp4676:
	vsubps	576(%rsp), %xmm0, %xmm2
	vaddps	%xmm6, %xmm2, %xmm4
	vmulps	%xmm4, %xmm4, %xmm4
	vmulps	%xmm4, %xmm11, %xmm4
	vcmpltps	%xmm2, %xmm6, %xmm5
	vblendvps	%xmm5, %xmm2, %xmm4, %xmm4
	vcmpleps	%xmm10, %xmm2, %xmm2
	vmulps	1184(%rsp), %xmm4, %xmm4
	vpcmpgtd	%xmm2, %xmm15, %xmm2
	vpandn	%xmm4, %xmm2, %xmm2
	vbroadcastss	.LCPI34_23(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm2
	vminps	%xmm15, %xmm2, %xmm2
	vmovaps	1024(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm2, %xmm4
	vmovaps	1152(%rsp), %xmm0
	vblendvps	%xmm4, 1168(%rsp), %xmm0, %xmm4
	vsubps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm2, %xmm2
	vandps	.LCPI34_1(%rip), %xmm2, %xmm4
	vbroadcastss	.LCPI34_2(%rip), %xmm0
	vcmpltps	%xmm0, %xmm4, %xmm4
	vandnps	%xmm2, %xmm4, %xmm7
	vaddps	640(%rsp), %xmm7, %xmm2
	vbroadcastss	.LCPI34_24(%rip), %xmm0
	vmulps	%xmm0, %xmm2, %xmm2
	vmaxps	%xmm14, %xmm2, %xmm2
	vminps	%xmm1, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm4
	vsubps	%xmm4, %xmm2, %xmm2
	vmulps	%xmm2, %xmm13, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm2, %xmm2
	vbroadcastss	.LCPI34_32(%rip), %xmm0
	vaddps	%xmm0, %xmm2, %xmm2
	vbroadcastss	.LCPI34_33(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm2, %xmm2
.Ltmp4677:
	movq	(%r14), %rcx
	vmovaps	960(%rsp), %xmm0
	vmulps	(%rcx,%rax,4), %xmm0, %xmm1
	movq	16(%r14), %rcx
	vmovaps	160(%rsp), %xmm0
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
	vaddps	%xmm0, %xmm1, %xmm0
.Ltmp4678:
	movq	1328(%r14), %rcx
	vmulps	(%rcx,%rax,4), %xmm3, %xmm1
	.loc	1 1052 27 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp4679:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm2, %xmm2
.Ltmp4680:
	.loc	9 36 14
	vaddps	%xmm2, %xmm1, %xmm1
	movq	888(%rsp), %r10
.Ltmp4681:
	.loc	8 551 14
	vmovups	%xmm0, (%r10,%rdi,4)
	movq	880(%rsp), %rbx
.Ltmp4682:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm1, (%rbx,%rdi,4)
	movq	944(%rsp), %r15
.Ltmp4683:
	.loc	1 0 0
	incq	%r15
.Ltmp4684:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	movq	976(%rsp), %rbp
	cmpq	%r15, %rbp
.Ltmp4685:
	.loc	3 900 12
	jne	.LBB34_251
.Ltmp4686:
.LBB34_286:
	.loc	3 0 12 is_stmt 0
	vmovaps	80(%rsp), %xmm0
	.loc	1 1057 5 is_stmt 1
	vmovaps	%xmm0, 96(%r14)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 112(%r14)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 128(%r14)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 144(%r14)
	vmovaps	32(%rsp), %xmm0
	.loc	1 1058 5
	vmovaps	%xmm0, 1424(%r14)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%r14)
	vmovaps	144(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%r14)
	vmovaps	864(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%r14)
	vmovaps	816(%rsp), %xmm0
	.loc	1 1059 5
	vmovaps	%xmm0, 160(%r14)
	vmovaps	832(%rsp), %xmm0
	vmovaps	%xmm0, 176(%r14)
	vmovaps	848(%rsp), %xmm0
	.loc	1 1060 5
	vmovaps	%xmm0, 1488(%r14)
	vmovaps	%xmm7, 1504(%r14)
	.loc	1 1061 5
	movq	%r11, 2680(%r14)
	xorl	%eax, %eax
	xorl	%edi, %edi
.Ltmp4687:
	.loc	1 0 5 is_stmt 0
.Ltmp4688:
	.p2align	4
.LBB34_287:
	.loc	1 1194 13 is_stmt 1
	vmovd	176(%rsp,%rax), %xmm0
	vmovss	180(%rsp,%rax), %xmm1
	vmovss	184(%rsp,%rax), %xmm2
	vmovss	188(%rsp,%rax), %xmm3
.Ltmp4689:
	.loc	1 1197 17
	vmovd	%xmm0, 192(%r14,%rax)
	.loc	1 1198 34
	movl	204(%r14,%rax), %ecx
	movl	364(%r14,%rax), %edx
.Ltmp4690:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp4691:
	.loc	1 1198 17
	movl	%ecx, 204(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 352(%r14,%rax)
.Ltmp4692:
	.loc	38 2472 13
	subl	%ebp, %edx
	cmovbl	%edi, %edx
.Ltmp4693:
	.loc	1 1198 17
	movl	%edx, 364(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 512(%r14,%rax)
	.loc	1 1198 34
	movl	524(%r14,%rax), %ecx
.Ltmp4694:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp4695:
	.loc	1 1198 17
	movl	%ecx, 524(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm3, 672(%r14,%rax)
	.loc	1 1198 34
	movl	684(%r14,%rax), %ecx
.Ltmp4696:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp4697:
	.loc	1 1198 17
	movl	%ecx, 684(%r14,%rax)
.Ltmp4698:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp4699:
	.loc	3 900 12
	jne	.LBB34_287
.Ltmp4700:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	928(%rsp), %rsi
	movq	936(%rsp), %r15
	.p2align	4
.LBB34_289:
.Ltmp4701:
	.loc	1 1194 13 is_stmt 1
	vmovd	496(%rsp,%rax), %xmm0
	vmovss	500(%rsp,%rax), %xmm1
	vmovss	504(%rsp,%rax), %xmm2
	vmovss	508(%rsp,%rax), %xmm3
.Ltmp4702:
	.loc	1 1197 17
	vmovd	%xmm0, 1520(%r14,%rax)
	.loc	1 1198 34
	movl	1532(%r14,%rax), %ecx
	movl	1692(%r14,%rax), %edx
.Ltmp4703:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp4704:
	.loc	1 1198 17
	movl	%ecx, 1532(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 1680(%r14,%rax)
.Ltmp4705:
	.loc	38 2472 13
	subl	%ebp, %edx
	cmovbl	%edi, %edx
.Ltmp4706:
	.loc	1 1198 17
	movl	%edx, 1692(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 1840(%r14,%rax)
	.loc	1 1198 34
	movl	1852(%r14,%rax), %ecx
.Ltmp4707:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp4708:
	.loc	1 1198 17
	movl	%ecx, 1852(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm3, 2000(%r14,%rax)
	.loc	1 1198 34
	movl	2012(%r14,%rax), %ecx
.Ltmp4709:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp4710:
	.loc	1 1198 17
	movl	%ecx, 2012(%r14,%rax)
.Ltmp4711:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp4712:
	.loc	3 900 12
	jne	.LBB34_289
	jmp	.LBB34_244
.Ltmp4713:
.LBB34_290:
	.loc	38 1050 16
	cmpq	%r15, %rsi
	movq	912(%rsp), %rax
.Ltmp4714:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_481
.Ltmp4715:
	.loc	48 451 16 is_stmt 1
	cmpq	120(%rsp), %rsi
	ja	.LBB34_481
.Ltmp4716:
	.loc	48 451 16 is_stmt 0
	cmpq	112(%rsp), %rsi
	ja	.LBB34_484
.Ltmp4717:
	.loc	1 972 27 is_stmt 1
	vmovaps	96(%r14), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	112(%r14), %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmovaps	128(%r14), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	144(%r14), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4718:
	.loc	1 973 26
	vmovaps	1424(%r14), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	1440(%r14), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	1456(%r14), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	1472(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
.Ltmp4719:
	.loc	1 974 25
	vmovaps	160(%r14), %xmm0
	vmovaps	%xmm0, 816(%rsp)
	vmovaps	176(%r14), %xmm0
	vmovaps	%xmm0, 832(%rsp)
.Ltmp4720:
	.loc	1 975 24
	vmovaps	1488(%r14), %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vmovaps	1504(%r14), %xmm7
.Ltmp4721:
	.loc	1 976 24
	movq	2680(%r14), %r10
.Ltmp4722:
	.loc	2 1916 50
	testq	%rbp, %rbp
.Ltmp4723:
	.loc	3 900 12
	je	.LBB34_243
.Ltmp4724:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rbp,4), %rcx
	movq	%rcx, 104(%rsp)
	movq	920(%rsp), %rcx
	leaq	(%rcx,%r15,4), %rbx
	leaq	(%rax,%r15,4), %r12
.Ltmp4725:
	.loc	48 568 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %rbp
	movq	%rbp, 896(%rsp)
	xorl	%edi, %edi
	xorl	%ebp, %ebp
	movq	%rbx, 888(%rsp)
	movq	%r12, 880(%rsp)
.Ltmp4726:
	.loc	48 0 12 is_stmt 0
.Ltmp4727:
	.p2align	4
.LBB34_295:
	.loc	1 987 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp4728:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %r9d
	cmovaeq	%r13, %r9
.Ltmp4729:
	.loc	48 568 12
	cmpq	104(%rsp), %rdi
	ja	.LBB34_456
.Ltmp4730:
	.loc	48 438 16
	cmpq	%rbp, 896(%rsp)
	je	.LBB34_449
.Ltmp4731:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,4), %rax
.Ltmp4732:
	.loc	1 1000 29 is_stmt 1
	movq	8(%r14), %rsi
.Ltmp4733:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_457
.Ltmp4734:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4735:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm7, 1024(%rsp)
	vmovups	(%rbx,%rdi,4), %xmm4
.Ltmp4736:
	vmovups	(%r12,%rdi,4), %xmm14
.Ltmp4737:
	vmovaps	32(%r14), %xmm10
	vmovaps	48(%r14), %xmm11
	vmovaps	64(%r14), %xmm1
	vmovaps	1360(%r14), %xmm15
	vmovaps	1376(%r14), %xmm9
	vmovaps	1392(%r14), %xmm2
	vmovaps	64(%rsp), %xmm8
	vsubps	%xmm8, %xmm4, %xmm0
	vmulps	%xmm0, %xmm11, %xmm3
	vmovaps	80(%rsp), %xmm6
	vmulps	%xmm6, %xmm10, %xmm5
	vaddps	%xmm3, %xmm5, %xmm12
	vaddps	%xmm6, %xmm12, %xmm3
	vmulps	%xmm6, %xmm11, %xmm5
	vmulps	%xmm1, %xmm0, %xmm0
	vaddps	%xmm0, %xmm5, %xmm7
	vaddps	%xmm7, %xmm8, %xmm5
	vmulps	80(%r14), %xmm3, %xmm0
	vmovaps	128(%rsp), %xmm8
	vsubps	%xmm8, %xmm5, %xmm6
	vmulps	48(%rsp), %xmm11, %xmm3
	vmulps	%xmm6, %xmm1, %xmm1
	vaddps	%xmm1, %xmm3, %xmm5
	vaddps	%xmm5, %xmm8, %xmm3
.Ltmp4738:
	vsubps	16(%rsp), %xmm14, %xmm13
	vmulps	%xmm9, %xmm13, %xmm1
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm15, 160(%rsp)
	vmulps	%xmm15, %xmm8, %xmm15
	vaddps	%xmm1, %xmm15, %xmm1
	vaddps	%xmm1, %xmm8, %xmm15
	vmulps	1408(%r14), %xmm15, %xmm15
.Ltmp4739:
	.loc	1 1000 29 is_stmt 1
	movq	(%r14), %rcx
.Ltmp4740:
	.loc	8 551 14
	vmovups	%xmm3, (%rcx,%rax,4)
.Ltmp4741:
	.loc	1 1001 30
	movq	24(%r14), %rsi
.Ltmp4742:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_471
.Ltmp4743:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4744:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm0, %xmm4, %xmm0
	vsubps	%xmm3, %xmm0, %xmm0
.Ltmp4745:
	.loc	1 1001 30 is_stmt 1
	movq	16(%r14), %rcx
.Ltmp4746:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4747:
	.loc	1 1002 28
	movq	1336(%r14), %rsi
.Ltmp4748:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_472
.Ltmp4749:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4750:
	.loc	1 0 0 is_stmt 0
	vmulps	32(%rsp), %xmm9, %xmm0
	vmulps	%xmm2, %xmm13, %xmm3
	vaddps	%xmm3, %xmm0, %xmm13
	vaddps	16(%rsp), %xmm13, %xmm0
	vmovaps	144(%rsp), %xmm3
	vsubps	%xmm3, %xmm0, %xmm4
	vmovaps	864(%rsp), %xmm8
	vmulps	%xmm9, %xmm8, %xmm0
	vmulps	%xmm4, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm2
	vaddps	%xmm2, %xmm3, %xmm0
.Ltmp4751:
	.loc	1 1002 28 is_stmt 1
	movq	1328(%r14), %rcx
.Ltmp4752:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4753:
	.loc	1 1003 29
	movq	1352(%r14), %rsi
.Ltmp4754:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_460
.Ltmp4755:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp4756:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm15, %xmm14, %xmm3
	vsubps	%xmm0, %xmm3, %xmm0
.Ltmp4757:
	.loc	1 1003 29 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp4758:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
	movq	1104(%r14), %rcx
.Ltmp4759:
	.loc	1 877 35
	addq	%r10, %rcx
.Ltmp4760:
	.loc	1 857 8
	cmpq	%r13, %rcx
	movl	$0, %eax
	cmovaeq	%r13, %rax
.Ltmp4761:
	.loc	1 1006 34
	movq	8(%r14), %rsi
.Ltmp4762:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp4763:
	.loc	1 877 30
	shlq	$2, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_492
	.loc	1 0 25
	movq	1112(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp4764:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rax
.Ltmp4765:
	.loc	1 877 30
	leaq	1(,%rax,4), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB34_491
	.loc	1 0 25
	movq	1120(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp4766:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %rax
.Ltmp4767:
	.loc	1 877 30
	leaq	2(,%rax,4), %r11
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_501
	.loc	1 0 25
	movq	1128(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp4768:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %rax
.Ltmp4769:
	.loc	1 877 30
	leaq	3(,%rax,4), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_504
.Ltmp4770:
	.loc	1 1008 34 is_stmt 1
	movq	24(%r14), %rsi
.Ltmp4771:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB34_492
	cmpq	%rsi, %rdx
	jae	.LBB34_491
	cmpq	%rsi, %r11
	jae	.LBB34_501
	cmpq	%rsi, %r15
	jae	.LBB34_504
.Ltmp4772:
	.loc	1 0 25 is_stmt 0
	movq	%rbp, 944(%rsp)
	movq	2432(%r14), %r8
.Ltmp4773:
	.loc	1 877 35
	addq	%r10, %r8
.Ltmp4774:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %r8
	movl	$0, %eax
	cmovaeq	%r13, %rax
.Ltmp4775:
	.loc	1 1010 34
	movq	1336(%r14), %rsi
.Ltmp4776:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp4777:
	.loc	1 877 30
	shlq	$2, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_488
	.loc	1 0 25
	movq	2440(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp4778:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp4779:
	.loc	1 877 30
	leaq	1(,%rax,4), %rbp
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	.loc	1 0 25
	movq	2448(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp4780:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp4781:
	.loc	1 877 30
	leaq	2(,%rax,4), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_503
	.loc	1 0 25
	movq	2456(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp4782:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp4783:
	.loc	1 877 30
	leaq	3(,%rax,4), %rbx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_508
.Ltmp4784:
	.loc	1 1012 34 is_stmt 1
	movq	1352(%r14), %rsi
.Ltmp4785:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB34_488
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	cmpq	%rsi, %r12
	jae	.LBB34_503
	cmpq	%rsi, %rbx
	jae	.LBB34_508
.Ltmp4786:
	.loc	1 0 0 is_stmt 0
	negq	%r9
	addq	%r9, %r10
	incq	%r10
	leaq	(,%r10,4), %rax
	movq	8(%rsp), %rsi
.Ltmp4787:
	.loc	1 1047 36 is_stmt 1
	movq	8(%rsi), %rsi
.Ltmp4788:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_462
.Ltmp4789:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4790:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 1049 27 is_stmt 1
	movq	24(%rsi), %rsi
.Ltmp4791:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_463
.Ltmp4792:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4793:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
.Ltmp4794:
	.loc	1 1050 35 is_stmt 1
	movq	1336(%rsi), %rsi
.Ltmp4795:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_464
.Ltmp4796:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4797:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 1052 27 is_stmt 1
	movq	1352(%rsi), %rsi
.Ltmp4798:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_465
.Ltmp4799:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp4800:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm12, %xmm12, %xmm0
	vaddps	80(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm12
	vandps	%xmm0, %xmm12, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm14
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vaddps	%xmm7, %xmm7, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmulps	%xmm6, %xmm11, %xmm0
	vmovaps	48(%rsp), %xmm6
	vmulps	%xmm6, %xmm10, %xmm3
	vaddps	%xmm0, %xmm3, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm6, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm5, %xmm5, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4801:
	vaddps	%xmm1, %xmm1, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm1
	vcmpltps	%xmm14, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm1
	vcmpltps	%xmm14, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 16(%rsp)
	movq	8(%rsp), %r14
.Ltmp4802:
	movq	(%r14), %rsi
	vmovd	(%rsi,%rcx,4), %xmm0
	vpinsrd	$1, (%rsi,%rdx,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%r11,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r15,4), %xmm0, %xmm0
.Ltmp4803:
	movq	16(%r14), %rsi
	vmovd	(%rsi,%rcx,4), %xmm1
	vpinsrd	$1, (%rsi,%rdx,4), %xmm1, %xmm1
.Ltmp4804:
	vmulps	%xmm4, %xmm9, %xmm3
.Ltmp4805:
	vpinsrd	$2, (%rsi,%r11,4), %xmm1, %xmm1
.Ltmp4806:
	vmulps	160(%rsp), %xmm8, %xmm4
	vaddps	%xmm3, %xmm4, %xmm3
.Ltmp4807:
	vpinsrd	$3, (%rsi,%r15,4), %xmm1, %xmm1
.Ltmp4808:
	vaddps	%xmm3, %xmm3, %xmm3
	vaddps	%xmm3, %xmm8, %xmm3
	vandps	%xmm3, %xmm12, %xmm4
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm3
	vmovaps	%xmm3, 864(%rsp)
	vaddps	%xmm2, %xmm2, %xmm2
	vaddps	144(%rsp), %xmm2, %xmm2
.Ltmp4809:
	movq	1328(%r14), %rcx
	vmovd	(%rcx,%r8,4), %xmm3
	vpinsrd	$1, (%rcx,%rbp,4), %xmm3, %xmm3
	vpinsrd	$2, (%rcx,%r12,4), %xmm3, %xmm3
	vpinsrd	$3, (%rcx,%rbx,4), %xmm3, %xmm3
.Ltmp4810:
	movq	1344(%r14), %rcx
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rbp,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%r12,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rbx,4), %xmm4, %xmm4
.Ltmp4811:
	vandps	%xmm2, %xmm12, %xmm5
	vcmpltps	%xmm14, %xmm5, %xmm5
	vandnps	%xmm2, %xmm5, %xmm2
	vmovaps	%xmm2, 144(%rsp)
.Ltmp4812:
	vpand	%xmm0, %xmm12, %xmm0
	vpand	%xmm3, %xmm12, %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
.Ltmp4813:
	vpand	%xmm1, %xmm12, %xmm1
	vpand	%xmm4, %xmm12, %xmm2
	vmaxps	%xmm2, %xmm1, %xmm2
	vbroadcastss	.LCPI34_4(%rip), %xmm5
.Ltmp4814:
	vmaxps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm6
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm7
	vandps	%xmm7, %xmm0, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm8
	vorps	%xmm1, %xmm8, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm9
	vaddps	%xmm1, %xmm9, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm10
	vmulps	%xmm1, %xmm10, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm11
	vaddps	%xmm3, %xmm11, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm14
	vmaxps	%xmm14, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm8
	vminps	%xmm8, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vsubps	176(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm0, %xmm1, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm7
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1136(%rsp), %xmm3, %xmm3
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm0, %xmm1, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	816(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm3
	vmovaps	1104(%rsp), %xmm4
	vblendvps	%xmm3, 1120(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm1, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm1
.Ltmp4815:
	vbroadcastss	.LCPI34_4(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm0
	vmaxps	%xmm6, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm2
	vorps	.LCPI34_7(%rip), %xmm2, %xmm2
	vaddps	%xmm2, %xmm9, %xmm2
	vmulps	%xmm2, %xmm10, %xmm3
	vaddps	%xmm3, %xmm11, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm14, %xmm0, %xmm0
	vminps	%xmm8, %xmm0, %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vsubps	256(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm3
	vaddps	%xmm3, %xmm0, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vmulps	%xmm5, %xmm2, %xmm2
	vcmpltps	%xmm0, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm0, %xmm2, %xmm2
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1088(%rsp), %xmm2, %xmm2
	vxorps	%xmm4, %xmm4, %xmm4
	vpcmpgtd	%xmm0, %xmm4, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm1, 816(%rsp)
.Ltmp4816:
	vaddps	240(%rsp), %xmm1, %xmm2
	vbroadcastss	.LCPI34_24(%rip), %xmm9
	vmulps	%xmm2, %xmm9, %xmm2
	vbroadcastss	.LCPI34_25(%rip), %xmm12
	vmaxps	%xmm12, %xmm2, %xmm2
	vbroadcastss	.LCPI34_26(%rip), %xmm11
	vminps	%xmm11, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm3
	vsubps	%xmm3, %xmm2, %xmm2
	vbroadcastss	.LCPI34_27(%rip), %xmm13
	vmulps	%xmm2, %xmm13, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm7
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm8
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vbroadcastss	.LCPI34_23(%rip), %xmm1
.Ltmp4817:
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm4, %xmm0, %xmm0
	vmovaps	832(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm6
	vmovaps	1056(%rsp), %xmm9
	vblendvps	%xmm6, 1072(%rsp), %xmm9, %xmm6
.Ltmp4818:
	vmulps	%xmm5, %xmm2, %xmm2
.Ltmp4819:
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm6, %xmm5, %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm14
	vandps	%xmm0, %xmm14, %xmm5
	vbroadcastss	.LCPI34_2(%rip), %xmm13
	vcmpltps	%xmm13, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm6
.Ltmp4820:
	vaddps	%xmm6, %xmm2, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm9
	vaddps	%xmm3, %xmm9, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vmovaps	%xmm1, 832(%rsp)
.Ltmp4821:
	vaddps	320(%rsp), %xmm1, %xmm3
.Ltmp4822:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 960(%rsp)
	vbroadcastss	.LCPI34_24(%rip), %xmm2
.Ltmp4823:
	vmulps	%xmm2, %xmm3, %xmm0
	vmaxps	%xmm12, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm4
	vmulps	%xmm4, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm10, %xmm5
	vmovaps	%xmm10, %xmm15
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm0
	vaddps	%xmm3, %xmm9, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm1
.Ltmp4824:
	vsubps	496(%rsp), %xmm1, %xmm5
.Ltmp4825:
	vmulps	%xmm3, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm6
.Ltmp4826:
	vaddps	%xmm6, %xmm5, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm10
	vmulps	%xmm0, %xmm10, %xmm0
	vcmpltps	%xmm5, %xmm6, %xmm3
	vblendvps	%xmm3, %xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm9
	vcmpleps	%xmm9, %xmm5, %xmm3
	vmulps	1040(%rsp), %xmm0, %xmm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm3, %xmm1, %xmm3
	vpandn	%xmm0, %xmm3, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vxorps	%xmm12, %xmm12, %xmm12
	vmovaps	848(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm3
	vmovaps	1200(%rsp), %xmm5
	vblendvps	%xmm3, 1216(%rsp), %xmm5, %xmm3
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm3, %xmm5, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm3
	vcmpltps	%xmm13, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vaddps	560(%rsp), %xmm0, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm13
	vmaxps	%xmm13, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vmulps	%xmm4, %xmm0, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmovaps	%xmm7, %xmm11
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm5
	vmovaps	992(%rsp), %xmm0
.Ltmp4827:
	vsubps	576(%rsp), %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm3, %xmm10, %xmm3
	vcmpltps	%xmm0, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vcmpleps	%xmm9, %xmm0, %xmm0
	vmulps	1184(%rsp), %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm12, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm12, %xmm0, %xmm0
	vmovaps	1024(%rsp), %xmm4
	vcmpltps	%xmm4, %xmm0, %xmm3
	vmovaps	1152(%rsp), %xmm1
	vblendvps	%xmm3, 1168(%rsp), %xmm1, %xmm3
	vsubps	%xmm0, %xmm4, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm7
	vaddps	640(%rsp), %xmm7, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm13, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm4
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm11, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm8, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
.Ltmp4828:
	movq	(%r14), %rcx
	vmovaps	960(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm2
	movq	16(%r14), %rcx
	vmovaps	160(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm1
	vaddps	%xmm1, %xmm2, %xmm1
.Ltmp4829:
	movq	1328(%r14), %rcx
	vmulps	(%rcx,%rax,4), %xmm5, %xmm2
	.loc	1 1052 27 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp4830:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
.Ltmp4831:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	888(%rsp), %rbx
.Ltmp4832:
	.loc	8 551 14
	vmovups	%xmm1, (%rbx,%rdi,4)
	movq	880(%rsp), %r12
.Ltmp4833:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r12,%rdi,4)
	movq	944(%rsp), %rbp
.Ltmp4834:
	.loc	1 0 0
	incq	%rbp
.Ltmp4835:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	cmpq	%rbp, 976(%rsp)
.Ltmp4836:
	.loc	3 900 12
	jne	.LBB34_295
	jmp	.LBB34_243
.Ltmp4837:
.LBB34_330:
	.loc	1 1086 11
	testq	%rsi, %rsi
	je	.LBB34_419
	.loc	1 0 11 is_stmt 0
	movl	2688(%r14), %eax
	movl	%eax, 908(%rsp)
	movq	2672(%r14), %r13
	leaq	1328(%r14), %rax
	movq	%rax, 1016(%rsp)
	xorl	%r15d, %r15d
	jmp	.LBB34_334
.LBB34_332:
	vmovaps	80(%rsp), %xmm0
.Ltmp4838:
	.loc	1 1057 5 is_stmt 1
	vmovaps	%xmm0, 96(%r14)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 112(%r14)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 128(%r14)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 144(%r14)
	vmovaps	32(%rsp), %xmm0
	.loc	1 1058 5
	vmovaps	%xmm0, 1424(%r14)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%r14)
	vmovaps	864(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%r14)
	vmovaps	144(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%r14)
	vmovaps	816(%rsp), %xmm0
	.loc	1 1059 5
	vmovaps	%xmm0, 160(%r14)
	vmovaps	832(%rsp), %xmm0
	vmovaps	%xmm0, 176(%r14)
	vmovdqa	848(%rsp), %xmm0
	.loc	1 1060 5
	vmovdqa	%xmm0, 1488(%r14)
	vmovaps	%xmm7, 1504(%r14)
	.loc	1 1061 5
	movq	%r10, 2680(%r14)
	movq	928(%rsp), %rsi
	movq	936(%rsp), %r15
.Ltmp4839:
.LBB34_333:
	.loc	1 1086 11
	cmpq	%rsi, %r15
	jae	.LBB34_419
.LBB34_334:
	.loc	1 1087 42
	subq	%r15, %rsi
	.loc	1 1087 29 is_stmt 0
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %rbp
.Ltmp4840:
	.loc	1 1176 33 is_stmt 1
	vmovss	192(%r14), %xmm0
.Ltmp4841:
	.loc	1 1089 28
	vmovss	%xmm0, 176(%rsp)
.Ltmp4842:
	.loc	1 1176 33
	vmovss	352(%r14), %xmm0
.Ltmp4843:
	.loc	1 1089 28
	vmovss	%xmm0, 180(%rsp)
.Ltmp4844:
	.loc	1 1176 33
	vmovss	512(%r14), %xmm0
.Ltmp4845:
	.loc	1 1089 28
	vmovss	%xmm0, 184(%rsp)
.Ltmp4846:
	.loc	1 1176 33
	vmovss	672(%r14), %xmm0
.Ltmp4847:
	.loc	1 1089 28
	vmovss	%xmm0, 188(%rsp)
.Ltmp4848:
	.loc	1 1176 33
	vmovss	208(%r14), %xmm0
.Ltmp4849:
	.loc	1 1089 28
	vmovss	%xmm0, 192(%rsp)
.Ltmp4850:
	.loc	1 1176 33
	vmovss	368(%r14), %xmm0
.Ltmp4851:
	.loc	1 1089 28
	vmovss	%xmm0, 196(%rsp)
.Ltmp4852:
	.loc	1 1176 33
	vmovss	528(%r14), %xmm0
.Ltmp4853:
	.loc	1 1089 28
	vmovss	%xmm0, 200(%rsp)
.Ltmp4854:
	.loc	1 1176 33
	vmovss	688(%r14), %xmm0
.Ltmp4855:
	.loc	1 1089 28
	vmovss	%xmm0, 204(%rsp)
.Ltmp4856:
	.loc	1 1176 33
	vmovss	224(%r14), %xmm0
.Ltmp4857:
	.loc	1 1089 28
	vmovss	%xmm0, 208(%rsp)
.Ltmp4858:
	.loc	1 1176 33
	vmovss	384(%r14), %xmm0
.Ltmp4859:
	.loc	1 1089 28
	vmovss	%xmm0, 212(%rsp)
.Ltmp4860:
	.loc	1 1176 33
	vmovss	544(%r14), %xmm0
.Ltmp4861:
	.loc	1 1089 28
	vmovss	%xmm0, 216(%rsp)
.Ltmp4862:
	.loc	1 1176 33
	vmovss	704(%r14), %xmm0
.Ltmp4863:
	.loc	1 1089 28
	vmovss	%xmm0, 220(%rsp)
.Ltmp4864:
	.loc	1 1176 33
	vmovss	240(%r14), %xmm0
.Ltmp4865:
	.loc	1 1089 28
	vmovss	%xmm0, 224(%rsp)
.Ltmp4866:
	.loc	1 1176 33
	vmovss	400(%r14), %xmm0
.Ltmp4867:
	.loc	1 1089 28
	vmovss	%xmm0, 228(%rsp)
.Ltmp4868:
	.loc	1 1176 33
	vmovss	560(%r14), %xmm0
.Ltmp4869:
	.loc	1 1089 28
	vmovss	%xmm0, 232(%rsp)
.Ltmp4870:
	.loc	1 1176 33
	vmovss	720(%r14), %xmm0
.Ltmp4871:
	.loc	1 1089 28
	vmovss	%xmm0, 236(%rsp)
.Ltmp4872:
	.loc	1 1176 33
	vmovss	256(%r14), %xmm0
.Ltmp4873:
	.loc	1 1089 28
	vmovss	%xmm0, 240(%rsp)
.Ltmp4874:
	.loc	1 1176 33
	vmovss	416(%r14), %xmm0
.Ltmp4875:
	.loc	1 1089 28
	vmovss	%xmm0, 244(%rsp)
.Ltmp4876:
	.loc	1 1176 33
	vmovss	576(%r14), %xmm0
.Ltmp4877:
	.loc	1 1089 28
	vmovss	%xmm0, 248(%rsp)
.Ltmp4878:
	.loc	1 1176 33
	vmovss	736(%r14), %xmm0
.Ltmp4879:
	.loc	1 1089 28
	vmovss	%xmm0, 252(%rsp)
.Ltmp4880:
	.loc	1 1176 33
	vmovss	272(%r14), %xmm0
.Ltmp4881:
	.loc	1 1089 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp4882:
	.loc	1 1176 33
	vmovss	432(%r14), %xmm0
.Ltmp4883:
	.loc	1 1089 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp4884:
	.loc	1 1176 33
	vmovss	592(%r14), %xmm0
.Ltmp4885:
	.loc	1 1089 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp4886:
	.loc	1 1176 33
	vmovss	752(%r14), %xmm0
.Ltmp4887:
	.loc	1 1089 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp4888:
	.loc	1 1176 33
	vmovss	288(%r14), %xmm0
.Ltmp4889:
	.loc	1 1089 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp4890:
	.loc	1 1176 33
	vmovss	448(%r14), %xmm0
.Ltmp4891:
	.loc	1 1089 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp4892:
	.loc	1 1176 33
	vmovss	608(%r14), %xmm0
.Ltmp4893:
	.loc	1 1089 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp4894:
	.loc	1 1176 33
	vmovss	768(%r14), %xmm0
.Ltmp4895:
	.loc	1 1089 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp4896:
	.loc	1 1176 33
	vmovss	304(%r14), %xmm0
.Ltmp4897:
	.loc	1 1089 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp4898:
	.loc	1 1176 33
	vmovss	464(%r14), %xmm0
.Ltmp4899:
	.loc	1 1089 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp4900:
	.loc	1 1176 33
	vmovss	624(%r14), %xmm0
.Ltmp4901:
	.loc	1 1089 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp4902:
	.loc	1 1176 33
	vmovss	784(%r14), %xmm0
.Ltmp4903:
	.loc	1 1089 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp4904:
	.loc	1 1176 33
	vmovss	320(%r14), %xmm0
.Ltmp4905:
	.loc	1 1089 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp4906:
	.loc	1 1176 33
	vmovss	480(%r14), %xmm0
.Ltmp4907:
	.loc	1 1089 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp4908:
	.loc	1 1176 33
	vmovss	640(%r14), %xmm0
.Ltmp4909:
	.loc	1 1089 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp4910:
	.loc	1 1176 33
	vmovss	800(%r14), %xmm0
.Ltmp4911:
	.loc	1 1089 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp4912:
	.loc	1 1176 33
	vmovss	336(%r14), %xmm0
.Ltmp4913:
	.loc	1 1089 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp4914:
	.loc	1 1176 33
	vmovss	496(%r14), %xmm0
.Ltmp4915:
	.loc	1 1089 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp4916:
	.loc	1 1176 33
	vmovss	656(%r14), %xmm0
.Ltmp4917:
	.loc	1 1089 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp4918:
	.loc	1 1176 33
	vmovss	816(%r14), %xmm0
.Ltmp4919:
	.loc	1 1089 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp4920:
	.loc	1 1177 32
	vmovss	200(%r14), %xmm0
.Ltmp4921:
	.loc	1 1089 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp4922:
	.loc	1 1177 32
	vmovss	360(%r14), %xmm0
.Ltmp4923:
	.loc	1 1089 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp4924:
	.loc	1 1177 32
	vmovss	520(%r14), %xmm0
.Ltmp4925:
	.loc	1 1089 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp4926:
	.loc	1 1177 32
	vmovss	680(%r14), %xmm0
.Ltmp4927:
	.loc	1 1089 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp4928:
	.loc	1 1177 32
	vmovss	216(%r14), %xmm0
.Ltmp4929:
	.loc	1 1089 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp4930:
	.loc	1 1177 32
	vmovss	376(%r14), %xmm0
.Ltmp4931:
	.loc	1 1089 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp4932:
	.loc	1 1177 32
	vmovss	536(%r14), %xmm0
.Ltmp4933:
	.loc	1 1089 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp4934:
	.loc	1 1177 32
	vmovss	696(%r14), %xmm0
.Ltmp4935:
	.loc	1 1089 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp4936:
	.loc	1 1177 32
	vmovss	232(%r14), %xmm0
.Ltmp4937:
	.loc	1 1089 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp4938:
	.loc	1 1177 32
	vmovss	392(%r14), %xmm0
.Ltmp4939:
	.loc	1 1089 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp4940:
	.loc	1 1177 32
	vmovss	552(%r14), %xmm0
.Ltmp4941:
	.loc	1 1089 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp4942:
	.loc	1 1177 32
	vmovss	712(%r14), %xmm0
.Ltmp4943:
	.loc	1 1089 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp4944:
	.loc	1 1177 32
	vmovss	248(%r14), %xmm0
.Ltmp4945:
	.loc	1 1089 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp4946:
	.loc	1 1177 32
	vmovss	408(%r14), %xmm0
.Ltmp4947:
	.loc	1 1089 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp4948:
	.loc	1 1177 32
	vmovss	568(%r14), %xmm0
.Ltmp4949:
	.loc	1 1089 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp4950:
	.loc	1 1177 32
	vmovss	728(%r14), %xmm0
.Ltmp4951:
	.loc	1 1089 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp4952:
	.loc	1 1177 32
	vmovss	264(%r14), %xmm0
.Ltmp4953:
	.loc	1 1089 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp4954:
	.loc	1 1177 32
	vmovss	424(%r14), %xmm0
.Ltmp4955:
	.loc	1 1089 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp4956:
	.loc	1 1177 32
	vmovss	584(%r14), %xmm0
.Ltmp4957:
	.loc	1 1089 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp4958:
	.loc	1 1177 32
	vmovss	744(%r14), %xmm0
.Ltmp4959:
	.loc	1 1089 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp4960:
	.loc	1 1177 32
	vmovss	280(%r14), %xmm0
.Ltmp4961:
	.loc	1 1089 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp4962:
	.loc	1 1177 32
	vmovss	440(%r14), %xmm0
.Ltmp4963:
	.loc	1 1089 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp4964:
	.loc	1 1177 32
	vmovss	600(%r14), %xmm0
.Ltmp4965:
	.loc	1 1089 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp4966:
	.loc	1 1177 32
	vmovss	760(%r14), %xmm0
.Ltmp4967:
	.loc	1 1089 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp4968:
	.loc	1 1177 32
	vmovss	296(%r14), %xmm0
.Ltmp4969:
	.loc	1 1089 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp4970:
	.loc	1 1177 32
	vmovss	456(%r14), %xmm0
.Ltmp4971:
	.loc	1 1089 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp4972:
	.loc	1 1177 32
	vmovss	616(%r14), %xmm0
.Ltmp4973:
	.loc	1 1089 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp4974:
	.loc	1 1177 32
	vmovss	776(%r14), %xmm0
.Ltmp4975:
	.loc	1 1089 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp4976:
	.loc	1 1177 32
	vmovss	312(%r14), %xmm0
.Ltmp4977:
	.loc	1 1089 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp4978:
	.loc	1 1177 32
	vmovss	472(%r14), %xmm0
.Ltmp4979:
	.loc	1 1089 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp4980:
	.loc	1 1177 32
	vmovss	632(%r14), %xmm0
.Ltmp4981:
	.loc	1 1089 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp4982:
	.loc	1 1177 32
	vmovss	792(%r14), %xmm0
.Ltmp4983:
	.loc	1 1089 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp4984:
	.loc	1 1177 32
	vmovss	328(%r14), %xmm0
.Ltmp4985:
	.loc	1 1089 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp4986:
	.loc	1 1177 32
	vmovss	488(%r14), %xmm0
.Ltmp4987:
	.loc	1 1089 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp4988:
	.loc	1 1177 32
	vmovss	648(%r14), %xmm0
.Ltmp4989:
	.loc	1 1089 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp4990:
	.loc	1 1177 32
	vmovss	808(%r14), %xmm0
.Ltmp4991:
	.loc	1 1089 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp4992:
	.loc	1 1177 32
	vmovss	344(%r14), %xmm0
.Ltmp4993:
	.loc	1 1089 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp4994:
	.loc	1 1177 32
	vmovss	504(%r14), %xmm0
.Ltmp4995:
	.loc	1 1089 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp4996:
	.loc	1 1177 32
	vmovss	664(%r14), %xmm0
.Ltmp4997:
	.loc	1 1089 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp4998:
	.loc	1 1177 32
	vmovss	824(%r14), %xmm0
.Ltmp4999:
	.loc	1 1089 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp5000:
	.loc	1 1176 33
	vmovss	1520(%r14), %xmm0
.Ltmp5001:
	.loc	1 1089 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp5002:
	.loc	1 1176 33
	vmovss	1680(%r14), %xmm0
.Ltmp5003:
	.loc	1 1089 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp5004:
	.loc	1 1176 33
	vmovss	1840(%r14), %xmm0
.Ltmp5005:
	.loc	1 1089 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp5006:
	.loc	1 1176 33
	vmovss	2000(%r14), %xmm0
.Ltmp5007:
	.loc	1 1089 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp5008:
	.loc	1 1176 33
	vmovss	1536(%r14), %xmm0
.Ltmp5009:
	.loc	1 1089 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp5010:
	.loc	1 1176 33
	vmovss	1696(%r14), %xmm0
.Ltmp5011:
	.loc	1 1089 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp5012:
	.loc	1 1176 33
	vmovss	1856(%r14), %xmm0
.Ltmp5013:
	.loc	1 1089 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp5014:
	.loc	1 1176 33
	vmovss	2016(%r14), %xmm0
.Ltmp5015:
	.loc	1 1089 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp5016:
	.loc	1 1176 33
	vmovss	1552(%r14), %xmm0
.Ltmp5017:
	.loc	1 1089 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp5018:
	.loc	1 1176 33
	vmovss	1712(%r14), %xmm0
.Ltmp5019:
	.loc	1 1089 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp5020:
	.loc	1 1176 33
	vmovss	1872(%r14), %xmm0
.Ltmp5021:
	.loc	1 1089 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp5022:
	.loc	1 1176 33
	vmovss	2032(%r14), %xmm0
.Ltmp5023:
	.loc	1 1089 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp5024:
	.loc	1 1176 33
	vmovss	1568(%r14), %xmm0
.Ltmp5025:
	.loc	1 1089 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp5026:
	.loc	1 1176 33
	vmovss	1728(%r14), %xmm0
.Ltmp5027:
	.loc	1 1089 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp5028:
	.loc	1 1176 33
	vmovss	1888(%r14), %xmm0
.Ltmp5029:
	.loc	1 1089 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp5030:
	.loc	1 1176 33
	vmovss	2048(%r14), %xmm0
.Ltmp5031:
	.loc	1 1089 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp5032:
	.loc	1 1176 33
	vmovss	1584(%r14), %xmm0
.Ltmp5033:
	.loc	1 1089 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp5034:
	.loc	1 1176 33
	vmovss	1744(%r14), %xmm0
.Ltmp5035:
	.loc	1 1089 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp5036:
	.loc	1 1176 33
	vmovss	1904(%r14), %xmm0
.Ltmp5037:
	.loc	1 1089 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp5038:
	.loc	1 1176 33
	vmovss	2064(%r14), %xmm0
.Ltmp5039:
	.loc	1 1089 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp5040:
	.loc	1 1176 33
	vmovss	1600(%r14), %xmm0
.Ltmp5041:
	.loc	1 1089 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp5042:
	.loc	1 1176 33
	vmovss	1760(%r14), %xmm0
.Ltmp5043:
	.loc	1 1089 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp5044:
	.loc	1 1176 33
	vmovss	1920(%r14), %xmm0
.Ltmp5045:
	.loc	1 1089 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp5046:
	.loc	1 1176 33
	vmovss	2080(%r14), %xmm0
.Ltmp5047:
	.loc	1 1089 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp5048:
	.loc	1 1176 33
	vmovss	1616(%r14), %xmm0
.Ltmp5049:
	.loc	1 1089 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp5050:
	.loc	1 1176 33
	vmovss	1776(%r14), %xmm0
.Ltmp5051:
	.loc	1 1089 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp5052:
	.loc	1 1176 33
	vmovss	1936(%r14), %xmm0
.Ltmp5053:
	.loc	1 1089 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp5054:
	.loc	1 1176 33
	vmovss	2096(%r14), %xmm0
.Ltmp5055:
	.loc	1 1089 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp5056:
	.loc	1 1176 33
	vmovss	1632(%r14), %xmm0
.Ltmp5057:
	.loc	1 1089 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp5058:
	.loc	1 1176 33
	vmovss	1792(%r14), %xmm0
.Ltmp5059:
	.loc	1 1089 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp5060:
	.loc	1 1176 33
	vmovss	1952(%r14), %xmm0
.Ltmp5061:
	.loc	1 1089 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp5062:
	.loc	1 1176 33
	vmovss	2112(%r14), %xmm0
.Ltmp5063:
	.loc	1 1089 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp5064:
	.loc	1 1176 33
	vmovss	1648(%r14), %xmm0
.Ltmp5065:
	.loc	1 1089 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp5066:
	.loc	1 1176 33
	vmovss	1808(%r14), %xmm0
.Ltmp5067:
	.loc	1 1089 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp5068:
	.loc	1 1176 33
	vmovss	1968(%r14), %xmm0
.Ltmp5069:
	.loc	1 1089 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp5070:
	.loc	1 1176 33
	vmovss	2128(%r14), %xmm0
.Ltmp5071:
	.loc	1 1089 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp5072:
	.loc	1 1176 33
	vmovss	1664(%r14), %xmm0
.Ltmp5073:
	.loc	1 1089 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp5074:
	.loc	1 1176 33
	vmovss	1824(%r14), %xmm0
.Ltmp5075:
	.loc	1 1089 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp5076:
	.loc	1 1176 33
	vmovss	1984(%r14), %xmm0
.Ltmp5077:
	.loc	1 1089 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp5078:
	.loc	1 1176 33
	vmovss	2144(%r14), %xmm0
.Ltmp5079:
	.loc	1 1089 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp5080:
	.loc	1 1177 32
	vmovss	1528(%r14), %xmm0
.Ltmp5081:
	.loc	1 1089 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp5082:
	.loc	1 1177 32
	vmovss	1688(%r14), %xmm0
.Ltmp5083:
	.loc	1 1089 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp5084:
	.loc	1 1177 32
	vmovss	1848(%r14), %xmm0
.Ltmp5085:
	.loc	1 1089 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp5086:
	.loc	1 1177 32
	vmovss	2008(%r14), %xmm0
.Ltmp5087:
	.loc	1 1089 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp5088:
	.loc	1 1177 32
	vmovss	1544(%r14), %xmm0
.Ltmp5089:
	.loc	1 1089 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp5090:
	.loc	1 1177 32
	vmovss	1704(%r14), %xmm0
.Ltmp5091:
	.loc	1 1089 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp5092:
	.loc	1 1177 32
	vmovss	1864(%r14), %xmm0
.Ltmp5093:
	.loc	1 1089 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp5094:
	.loc	1 1177 32
	vmovss	2024(%r14), %xmm0
.Ltmp5095:
	.loc	1 1089 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp5096:
	.loc	1 1177 32
	vmovss	1560(%r14), %xmm0
.Ltmp5097:
	.loc	1 1089 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp5098:
	.loc	1 1177 32
	vmovss	1720(%r14), %xmm0
.Ltmp5099:
	.loc	1 1089 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp5100:
	.loc	1 1177 32
	vmovss	1880(%r14), %xmm0
.Ltmp5101:
	.loc	1 1089 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp5102:
	.loc	1 1177 32
	vmovss	2040(%r14), %xmm0
.Ltmp5103:
	.loc	1 1089 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp5104:
	.loc	1 1177 32
	vmovss	1576(%r14), %xmm0
.Ltmp5105:
	.loc	1 1089 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp5106:
	.loc	1 1177 32
	vmovss	1736(%r14), %xmm0
.Ltmp5107:
	.loc	1 1089 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp5108:
	.loc	1 1177 32
	vmovss	1896(%r14), %xmm0
.Ltmp5109:
	.loc	1 1089 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp5110:
	.loc	1 1177 32
	vmovss	2056(%r14), %xmm0
.Ltmp5111:
	.loc	1 1089 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp5112:
	.loc	1 1177 32
	vmovss	1592(%r14), %xmm0
.Ltmp5113:
	.loc	1 1089 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp5114:
	.loc	1 1177 32
	vmovss	1752(%r14), %xmm0
.Ltmp5115:
	.loc	1 1089 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp5116:
	.loc	1 1177 32
	vmovss	1912(%r14), %xmm0
.Ltmp5117:
	.loc	1 1089 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp5118:
	.loc	1 1177 32
	vmovss	2072(%r14), %xmm0
.Ltmp5119:
	.loc	1 1089 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp5120:
	.loc	1 1177 32
	vmovss	1608(%r14), %xmm0
.Ltmp5121:
	.loc	1 1089 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp5122:
	.loc	1 1177 32
	vmovss	1768(%r14), %xmm0
.Ltmp5123:
	.loc	1 1089 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp5124:
	.loc	1 1177 32
	vmovss	1928(%r14), %xmm0
.Ltmp5125:
	.loc	1 1089 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp5126:
	.loc	1 1177 32
	vmovss	2088(%r14), %xmm0
.Ltmp5127:
	.loc	1 1089 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp5128:
	.loc	1 1177 32
	vmovss	1624(%r14), %xmm0
.Ltmp5129:
	.loc	1 1089 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp5130:
	.loc	1 1177 32
	vmovss	1784(%r14), %xmm0
.Ltmp5131:
	.loc	1 1089 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp5132:
	.loc	1 1177 32
	vmovss	1944(%r14), %xmm0
.Ltmp5133:
	.loc	1 1089 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp5134:
	.loc	1 1177 32
	vmovss	2104(%r14), %xmm0
.Ltmp5135:
	.loc	1 1089 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp5136:
	.loc	1 1177 32
	vmovss	1640(%r14), %xmm0
.Ltmp5137:
	.loc	1 1089 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp5138:
	.loc	1 1177 32
	vmovss	1800(%r14), %xmm0
.Ltmp5139:
	.loc	1 1089 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp5140:
	.loc	1 1177 32
	vmovss	1960(%r14), %xmm0
.Ltmp5141:
	.loc	1 1089 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp5142:
	.loc	1 1177 32
	vmovss	2120(%r14), %xmm0
.Ltmp5143:
	.loc	1 1089 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp5144:
	.loc	1 1177 32
	vmovss	1656(%r14), %xmm0
.Ltmp5145:
	.loc	1 1089 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp5146:
	.loc	1 1177 32
	vmovss	1816(%r14), %xmm0
.Ltmp5147:
	.loc	1 1089 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp5148:
	.loc	1 1177 32
	vmovss	1976(%r14), %xmm0
.Ltmp5149:
	.loc	1 1089 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp5150:
	.loc	1 1177 32
	vmovss	2136(%r14), %xmm0
.Ltmp5151:
	.loc	1 1089 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp5152:
	.loc	1 1177 32
	vmovss	1672(%r14), %xmm0
.Ltmp5153:
	.loc	1 1089 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp5154:
	.loc	1 1177 32
	vmovss	1832(%r14), %xmm0
.Ltmp5155:
	.loc	1 1089 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp5156:
	.loc	1 1177 32
	vmovss	1992(%r14), %xmm0
.Ltmp5157:
	.loc	1 1089 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp5158:
	.loc	1 1177 32
	vmovss	2152(%r14), %xmm0
.Ltmp5159:
	.loc	1 1089 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp5160:
	.loc	1 1091 31
	leaq	1248(%rsp), %rdi
	movq	%r14, %rsi
	movl	908(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	1344(%rsp), %rdi
	movq	1016(%rsp), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovaps	1248(%rsp), %xmm0
	vmovaps	%xmm0, 1136(%rsp)
	vmovaps	1264(%rsp), %xmm0
	vmovaps	%xmm0, 1120(%rsp)
	vmovaps	1280(%rsp), %xmm0
	vmovaps	%xmm0, 1104(%rsp)
	vmovaps	1296(%rsp), %xmm0
	vmovaps	%xmm0, 1088(%rsp)
	vmovaps	1312(%rsp), %xmm0
	vmovaps	%xmm0, 1072(%rsp)
	vmovaps	1328(%rsp), %xmm0
	vmovaps	%xmm0, 1056(%rsp)
	vmovaps	1344(%rsp), %xmm0
	vmovaps	%xmm0, 1040(%rsp)
	vmovaps	1360(%rsp), %xmm0
	vmovaps	%xmm0, 1216(%rsp)
	vmovaps	1376(%rsp), %xmm0
	vmovaps	%xmm0, 1200(%rsp)
	vmovaps	1392(%rsp), %xmm0
	vmovaps	%xmm0, 1184(%rsp)
	vmovaps	1408(%rsp), %xmm0
	vmovaps	%xmm0, 1168(%rsp)
	vmovaps	1424(%rsp), %xmm0
	vmovaps	%xmm0, 1152(%rsp)
.Ltmp5161:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%rbp), %rcx
	shlq	$2, %r15
	leaq	(,%rcx,4), %rsi
	.loc	1 1094 12 is_stmt 1
	testb	$1, %r12b
	movq	%rbp, 976(%rsp)
	movq	%rcx, 936(%rsp)
	je	.LBB34_379
.Ltmp5162:
	.loc	38 1050 16
	cmpq	%r15, %rsi
	movq	912(%rsp), %rax
.Ltmp5163:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_480
.Ltmp5164:
	.loc	48 451 16 is_stmt 1
	cmpq	120(%rsp), %rsi
	ja	.LBB34_480
.Ltmp5165:
	.loc	48 451 16 is_stmt 0
	cmpq	112(%rsp), %rsi
	ja	.LBB34_486
.Ltmp5166:
	.loc	1 972 27 is_stmt 1
	vmovaps	96(%r14), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	112(%r14), %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmovaps	128(%r14), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	144(%r14), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5167:
	.loc	1 973 26
	vmovaps	1424(%r14), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	1440(%r14), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	1456(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovaps	1472(%r14), %xmm0
	vmovaps	%xmm0, 864(%rsp)
.Ltmp5168:
	.loc	1 974 25
	vmovaps	160(%r14), %xmm0
	vmovaps	%xmm0, 816(%rsp)
	vmovaps	176(%r14), %xmm0
	vmovaps	%xmm0, 832(%rsp)
.Ltmp5169:
	.loc	1 975 24
	vmovaps	1488(%r14), %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vmovaps	1504(%r14), %xmm7
.Ltmp5170:
	.loc	1 976 24
	movq	2680(%r14), %r11
.Ltmp5171:
	.loc	2 1916 50
	testq	%rbp, %rbp
	je	.LBB34_375
.Ltmp5172:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rbp,4), %rcx
	movq	%rcx, 104(%rsp)
	movq	920(%rsp), %rcx
	leaq	(%rcx,%r15,4), %r10
	leaq	(%rax,%r15,4), %rbx
.Ltmp5173:
	.loc	3 900 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %rbp
	movq	%rbp, 896(%rsp)
	xorl	%edi, %edi
	xorl	%r15d, %r15d
	movq	%r10, 888(%rsp)
	movq	%rbx, 880(%rsp)
.Ltmp5174:
	.loc	3 0 12 is_stmt 0
.Ltmp5175:
	.p2align	4
.LBB34_340:
	.loc	1 981 21 is_stmt 1
	vmovaps	176(%rsp), %xmm0
	vmovaps	192(%rsp), %xmm1
	vmovaps	208(%rsp), %xmm2
.Ltmp5176:
	.loc	9 36 14
	vaddps	336(%rsp), %xmm0, %xmm0
.Ltmp5177:
	.loc	1 983 21
	vmovaps	496(%rsp), %xmm3
	.loc	1 980 17
	vmovaps	%xmm0, 176(%rsp)
.Ltmp5178:
	.loc	9 36 14
	vaddps	656(%rsp), %xmm3, %xmm0
.Ltmp5179:
	.loc	1 982 17
	vmovaps	%xmm0, 496(%rsp)
.Ltmp5180:
	.loc	9 36 14
	vaddps	352(%rsp), %xmm1, %xmm0
.Ltmp5181:
	.loc	1 980 17
	vmovaps	%xmm0, 192(%rsp)
	.loc	1 983 21
	vmovaps	512(%rsp), %xmm0
.Ltmp5182:
	.loc	9 36 14
	vaddps	672(%rsp), %xmm0, %xmm0
.Ltmp5183:
	.loc	1 982 17
	vmovaps	%xmm0, 512(%rsp)
.Ltmp5184:
	.loc	9 36 14
	vaddps	368(%rsp), %xmm2, %xmm0
.Ltmp5185:
	.loc	1 980 17
	vmovaps	%xmm0, 208(%rsp)
	.loc	1 983 21
	vmovaps	528(%rsp), %xmm0
.Ltmp5186:
	.loc	9 36 14
	vaddps	688(%rsp), %xmm0, %xmm0
.Ltmp5187:
	.loc	1 982 17
	vmovaps	%xmm0, 528(%rsp)
	.loc	1 981 21
	vmovaps	224(%rsp), %xmm0
.Ltmp5188:
	.loc	9 36 14
	vaddps	384(%rsp), %xmm0, %xmm0
.Ltmp5189:
	.loc	1 980 17
	vmovaps	%xmm0, 224(%rsp)
	.loc	1 983 21
	vmovaps	544(%rsp), %xmm0
.Ltmp5190:
	.loc	9 36 14
	vaddps	704(%rsp), %xmm0, %xmm0
.Ltmp5191:
	.loc	1 982 17
	vmovaps	%xmm0, 544(%rsp)
	.loc	1 981 21
	vmovaps	240(%rsp), %xmm0
.Ltmp5192:
	.loc	9 36 14
	vaddps	400(%rsp), %xmm0, %xmm0
.Ltmp5193:
	.loc	1 980 17
	vmovaps	%xmm0, 240(%rsp)
	.loc	1 983 21
	vmovaps	560(%rsp), %xmm0
.Ltmp5194:
	.loc	9 36 14
	vaddps	720(%rsp), %xmm0, %xmm0
.Ltmp5195:
	.loc	1 982 17
	vmovaps	%xmm0, 560(%rsp)
	.loc	1 981 21
	vmovaps	256(%rsp), %xmm0
.Ltmp5196:
	.loc	9 36 14
	vaddps	416(%rsp), %xmm0, %xmm0
.Ltmp5197:
	.loc	1 980 17
	vmovaps	%xmm0, 256(%rsp)
	.loc	1 983 21
	vmovaps	576(%rsp), %xmm0
.Ltmp5198:
	.loc	9 36 14
	vaddps	736(%rsp), %xmm0, %xmm0
.Ltmp5199:
	.loc	1 982 17
	vmovaps	%xmm0, 576(%rsp)
	.loc	1 981 21
	vmovaps	272(%rsp), %xmm0
.Ltmp5200:
	.loc	9 36 14
	vaddps	432(%rsp), %xmm0, %xmm0
.Ltmp5201:
	.loc	1 980 17
	vmovaps	%xmm0, 272(%rsp)
	.loc	1 983 21
	vmovaps	592(%rsp), %xmm0
.Ltmp5202:
	.loc	9 36 14
	vaddps	752(%rsp), %xmm0, %xmm0
.Ltmp5203:
	.loc	1 982 17
	vmovaps	%xmm0, 592(%rsp)
	.loc	1 981 21
	vmovaps	288(%rsp), %xmm0
.Ltmp5204:
	.loc	9 36 14
	vaddps	448(%rsp), %xmm0, %xmm0
.Ltmp5205:
	.loc	1 980 17
	vmovaps	%xmm0, 288(%rsp)
	.loc	1 983 21
	vmovaps	608(%rsp), %xmm0
.Ltmp5206:
	.loc	9 36 14
	vaddps	768(%rsp), %xmm0, %xmm0
.Ltmp5207:
	.loc	1 982 17
	vmovaps	%xmm0, 608(%rsp)
	.loc	1 981 21
	vmovaps	304(%rsp), %xmm0
.Ltmp5208:
	.loc	9 36 14
	vaddps	464(%rsp), %xmm0, %xmm0
.Ltmp5209:
	.loc	1 980 17
	vmovaps	%xmm0, 304(%rsp)
	.loc	1 983 21
	vmovaps	624(%rsp), %xmm0
.Ltmp5210:
	.loc	9 36 14
	vaddps	784(%rsp), %xmm0, %xmm0
.Ltmp5211:
	.loc	1 982 17
	vmovaps	%xmm0, 624(%rsp)
	.loc	1 981 21
	vmovaps	320(%rsp), %xmm0
.Ltmp5212:
	.loc	9 36 14
	vaddps	480(%rsp), %xmm0, %xmm0
.Ltmp5213:
	.loc	1 980 17
	vmovaps	%xmm0, 320(%rsp)
	.loc	1 983 21
	vmovaps	640(%rsp), %xmm0
.Ltmp5214:
	.loc	9 36 14
	vaddps	800(%rsp), %xmm0, %xmm0
.Ltmp5215:
	.loc	1 982 17
	vmovaps	%xmm0, 640(%rsp)
.Ltmp5216:
	.loc	1 987 28
	leaq	1(%r11), %rax
.Ltmp5217:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %r9d
	cmovaeq	%r13, %r9
.Ltmp5218:
	.loc	48 568 12
	cmpq	104(%rsp), %rdi
	ja	.LBB34_456
.Ltmp5219:
	.loc	48 438 16
	cmpq	%r15, 896(%rsp)
	je	.LBB34_449
.Ltmp5220:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r11,4), %rax
.Ltmp5221:
	.loc	1 1000 29 is_stmt 1
	movq	8(%r14), %rsi
.Ltmp5222:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_457
.Ltmp5223:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp5224:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm7, 1024(%rsp)
	vmovups	(%r10,%rdi,4), %xmm0
.Ltmp5225:
	vmovups	(%rbx,%rdi,4), %xmm6
.Ltmp5226:
	vmovaps	32(%r14), %xmm10
	vmovaps	48(%r14), %xmm11
	vmovaps	64(%r14), %xmm1
	vmovaps	1360(%r14), %xmm13
	vmovaps	1376(%r14), %xmm9
	vmovaps	1392(%r14), %xmm14
	vmovaps	64(%rsp), %xmm8
	vsubps	%xmm8, %xmm0, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vmovaps	80(%rsp), %xmm7
	vmulps	%xmm7, %xmm10, %xmm4
	vaddps	%xmm3, %xmm4, %xmm4
	vaddps	%xmm4, %xmm7, %xmm5
	vmulps	%xmm7, %xmm11, %xmm3
	vmulps	%xmm1, %xmm2, %xmm2
	vaddps	%xmm2, %xmm3, %xmm3
	vaddps	%xmm3, %xmm8, %xmm2
	vmulps	80(%r14), %xmm5, %xmm12
	vmovaps	128(%rsp), %xmm7
	vsubps	%xmm7, %xmm2, %xmm2
	vmulps	48(%rsp), %xmm11, %xmm5
	vmulps	%xmm2, %xmm1, %xmm1
	vaddps	%xmm1, %xmm5, %xmm1
	vaddps	%xmm1, %xmm7, %xmm15
.Ltmp5227:
	vsubps	16(%rsp), %xmm6, %xmm5
	vmulps	%xmm5, %xmm9, %xmm7
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm13, 160(%rsp)
	vmulps	%xmm13, %xmm8, %xmm13
	vaddps	%xmm7, %xmm13, %xmm13
	vaddps	%xmm13, %xmm8, %xmm7
	vmulps	1408(%r14), %xmm7, %xmm7
.Ltmp5228:
	.loc	1 1000 29 is_stmt 1
	movq	(%r14), %rcx
.Ltmp5229:
	.loc	8 551 14
	vmovups	%xmm15, (%rcx,%rax,4)
.Ltmp5230:
	.loc	1 1001 30
	movq	24(%r14), %rsi
.Ltmp5231:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_471
.Ltmp5232:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp5233:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm0, %xmm12, %xmm0
	vsubps	%xmm15, %xmm0, %xmm0
.Ltmp5234:
	.loc	1 1001 30 is_stmt 1
	movq	16(%r14), %rcx
.Ltmp5235:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5236:
	.loc	1 1002 28
	movq	1336(%r14), %rsi
.Ltmp5237:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_472
.Ltmp5238:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp5239:
	.loc	1 0 0 is_stmt 0
	vmulps	32(%rsp), %xmm9, %xmm0
	vmulps	%xmm5, %xmm14, %xmm5
	vaddps	%xmm5, %xmm0, %xmm5
	vaddps	16(%rsp), %xmm5, %xmm0
	vmovaps	864(%rsp), %xmm15
	vsubps	%xmm15, %xmm0, %xmm0
	vmovaps	144(%rsp), %xmm8
	vmulps	%xmm9, %xmm8, %xmm12
	vmulps	%xmm0, %xmm14, %xmm14
	vaddps	%xmm14, %xmm12, %xmm14
	vaddps	%xmm14, %xmm15, %xmm12
.Ltmp5240:
	.loc	1 1002 28 is_stmt 1
	movq	1328(%r14), %rcx
.Ltmp5241:
	.loc	8 551 14
	vmovups	%xmm12, (%rcx,%rax,4)
.Ltmp5242:
	.loc	1 1003 29
	movq	1352(%r14), %rsi
.Ltmp5243:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_460
.Ltmp5244:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp5245:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm7, %xmm6, %xmm6
	vsubps	%xmm12, %xmm6, %xmm6
.Ltmp5246:
	.loc	1 1003 29 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp5247:
	.loc	8 551 14
	vmovups	%xmm6, (%rcx,%rax,4)
	movq	1104(%r14), %rcx
.Ltmp5248:
	.loc	1 877 35
	addq	%r11, %rcx
.Ltmp5249:
	.loc	1 857 8
	cmpq	%r13, %rcx
	movl	$0, %eax
	cmovaeq	%r13, %rax
.Ltmp5250:
	.loc	1 1006 34
	movq	8(%r14), %rsi
.Ltmp5251:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp5252:
	.loc	1 877 30
	shlq	$2, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_492
	.loc	1 0 25
	movq	1112(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp5253:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rax
.Ltmp5254:
	.loc	1 877 30
	leaq	1(,%rax,4), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB34_496
	.loc	1 0 25
	movq	%r15, 944(%rsp)
	movq	1120(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp5255:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %rax
.Ltmp5256:
	.loc	1 877 30
	leaq	2(,%rax,4), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_506
	.loc	1 0 25
	movq	1128(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp5257:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %rax
.Ltmp5258:
	.loc	1 877 30
	leaq	3(,%rax,4), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_504
.Ltmp5259:
	.loc	1 1008 34 is_stmt 1
	movq	24(%r14), %rsi
.Ltmp5260:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB34_492
	cmpq	%rsi, %rdx
	jae	.LBB34_496
	cmpq	%rsi, %r12
	jae	.LBB34_506
	cmpq	%rsi, %r15
	jae	.LBB34_536
.Ltmp5261:
	.loc	1 0 25 is_stmt 0
	movq	2432(%r14), %r8
.Ltmp5262:
	.loc	1 877 35
	addq	%r11, %r8
.Ltmp5263:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %r8
	movl	$0, %eax
	cmovaeq	%r13, %rax
.Ltmp5264:
	.loc	1 1010 34
	movq	1336(%r14), %rsi
.Ltmp5265:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp5266:
	.loc	1 877 30
	shlq	$2, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_488
	.loc	1 0 25
	movq	2440(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp5267:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp5268:
	.loc	1 877 30
	leaq	1(,%rax,4), %rbp
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_497
	.loc	1 0 25
	movq	2448(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp5269:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp5270:
	.loc	1 877 30
	leaq	2(,%rax,4), %r10
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r10
	jae	.LBB34_530
	.loc	1 0 25
	movq	2456(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp5271:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp5272:
	.loc	1 877 30
	leaq	3(,%rax,4), %rbx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_543
.Ltmp5273:
	.loc	1 1012 34 is_stmt 1
	movq	1352(%r14), %rsi
.Ltmp5274:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB34_488
	cmpq	%rsi, %rbp
	jae	.LBB34_497
	cmpq	%rsi, %r10
	jae	.LBB34_530
	cmpq	%rsi, %rbx
	jae	.LBB34_508
.Ltmp5275:
	.loc	1 0 0 is_stmt 0
	negq	%r9
	addq	%r9, %r11
	incq	%r11
	leaq	(,%r11,4), %rax
	movq	8(%rsp), %rsi
.Ltmp5276:
	.loc	1 1047 36 is_stmt 1
	movq	8(%rsi), %rsi
.Ltmp5277:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_462
.Ltmp5278:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp5279:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 1049 27 is_stmt 1
	movq	24(%rsi), %rsi
.Ltmp5280:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_463
.Ltmp5281:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp5282:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
.Ltmp5283:
	.loc	1 1050 35 is_stmt 1
	movq	1336(%rsi), %rsi
.Ltmp5284:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_464
.Ltmp5285:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp5286:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 1052 27 is_stmt 1
	movq	1352(%rsi), %rsi
.Ltmp5287:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_465
.Ltmp5288:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp5289:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm4, %xmm4, %xmm4
	vaddps	80(%rsp), %xmm4, %xmm4
	vbroadcastss	.LCPI34_35(%rip), %xmm7
	vandps	%xmm7, %xmm4, %xmm6
	vbroadcastss	.LCPI34_2(%rip), %xmm12
	vcmpltps	%xmm12, %xmm6, %xmm6
	vandnps	%xmm4, %xmm6, %xmm4
	vmovaps	%xmm4, 80(%rsp)
	vaddps	%xmm3, %xmm3, %xmm3
	vaddps	64(%rsp), %xmm3, %xmm3
	vandps	%xmm7, %xmm3, %xmm4
	vcmpltps	%xmm12, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm3
	vmovaps	%xmm3, 64(%rsp)
	vmulps	%xmm2, %xmm11, %xmm2
	vmovaps	48(%rsp), %xmm4
	vmulps	%xmm4, %xmm10, %xmm3
	vaddps	%xmm2, %xmm3, %xmm2
	vaddps	%xmm2, %xmm2, %xmm2
	vaddps	%xmm2, %xmm4, %xmm2
	vandps	%xmm7, %xmm2, %xmm3
	vcmpltps	%xmm12, %xmm3, %xmm3
	vandnps	%xmm2, %xmm3, %xmm2
	vmovaps	%xmm2, 48(%rsp)
	vaddps	%xmm1, %xmm1, %xmm1
	vaddps	128(%rsp), %xmm1, %xmm1
	vandps	%xmm7, %xmm1, %xmm2
	vcmpltps	%xmm12, %xmm2, %xmm2
	vandnps	%xmm1, %xmm2, %xmm1
	vmovaps	%xmm1, 128(%rsp)
.Ltmp5290:
	vaddps	%xmm13, %xmm13, %xmm1
	vaddps	32(%rsp), %xmm1, %xmm1
	vandps	%xmm7, %xmm1, %xmm2
	vcmpltps	%xmm12, %xmm2, %xmm2
	vandnps	%xmm1, %xmm2, %xmm1
	vmovaps	%xmm1, 32(%rsp)
	vaddps	%xmm5, %xmm5, %xmm1
	vaddps	16(%rsp), %xmm1, %xmm1
	vandps	%xmm7, %xmm1, %xmm2
	vcmpltps	%xmm12, %xmm2, %xmm2
	vandnps	%xmm1, %xmm2, %xmm1
	vmovaps	%xmm1, 16(%rsp)
	movq	8(%rsp), %r14
.Ltmp5291:
	movq	(%r14), %rsi
	vmovd	(%rsi,%rcx,4), %xmm1
	vpinsrd	$1, (%rsi,%rdx,4), %xmm1, %xmm1
	vpinsrd	$2, (%rsi,%r12,4), %xmm1, %xmm1
	vpinsrd	$3, (%rsi,%r15,4), %xmm1, %xmm1
.Ltmp5292:
	movq	16(%r14), %rsi
	vmovd	(%rsi,%rcx,4), %xmm2
	vpinsrd	$1, (%rsi,%rdx,4), %xmm2, %xmm2
.Ltmp5293:
	vmulps	%xmm0, %xmm9, %xmm0
.Ltmp5294:
	vpinsrd	$2, (%rsi,%r12,4), %xmm2, %xmm2
.Ltmp5295:
	vmulps	160(%rsp), %xmm8, %xmm3
	vaddps	%xmm0, %xmm3, %xmm0
.Ltmp5296:
	vpinsrd	$3, (%rsi,%r15,4), %xmm2, %xmm2
.Ltmp5297:
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm8, %xmm0
	vandps	%xmm7, %xmm0, %xmm3
	vcmpltps	%xmm12, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vaddps	%xmm14, %xmm14, %xmm0
	vaddps	%xmm0, %xmm15, %xmm0
.Ltmp5298:
	movq	1328(%r14), %rcx
	vmovd	(%rcx,%r8,4), %xmm3
	vpinsrd	$1, (%rcx,%rbp,4), %xmm3, %xmm3
	vpinsrd	$2, (%rcx,%r10,4), %xmm3, %xmm3
	vpinsrd	$3, (%rcx,%rbx,4), %xmm3, %xmm3
.Ltmp5299:
	movq	1344(%r14), %rcx
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rbp,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%r10,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rbx,4), %xmm4, %xmm4
.Ltmp5300:
	vandps	%xmm7, %xmm0, %xmm5
	vcmpltps	%xmm12, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm0
	vmovaps	%xmm0, 864(%rsp)
.Ltmp5301:
	vpand	%xmm7, %xmm1, %xmm0
	vpand	%xmm7, %xmm3, %xmm1
	vbroadcastss	.LCPI34_3(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
.Ltmp5302:
	vpand	%xmm7, %xmm2, %xmm1
	vpand	%xmm7, %xmm4, %xmm2
	vmulps	%xmm3, %xmm1, %xmm1
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_4(%rip), %xmm12
.Ltmp5303:
	vmaxps	%xmm12, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm6
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm8
	vandps	%xmm0, %xmm8, %xmm2
	vbroadcastss	.LCPI34_32(%rip), %xmm9
	vorps	%xmm2, %xmm9, %xmm2
	vbroadcastss	.LCPI34_8(%rip), %xmm10
	vaddps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm2, %xmm11, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm14
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vsubps	176(%rsp), %xmm0, %xmm2
	vbroadcastss	.LCPI34_20(%rip), %xmm9
	vaddps	%xmm2, %xmm9, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm2, %xmm9, %xmm4
	vblendvps	%xmm4, %xmm2, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm8
	vcmpleps	%xmm8, %xmm2, %xmm2
	vmulps	1136(%rsp), %xmm3, %xmm3
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%xmm2, %xmm0, %xmm2
	vpandn	%xmm3, %xmm2, %xmm2
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm2, %xmm2
	vminps	%xmm0, %xmm2, %xmm2
	vmovaps	816(%rsp), %xmm0
	vcmpltps	%xmm0, %xmm2, %xmm3
	vmovaps	1104(%rsp), %xmm4
	vblendvps	%xmm3, 1120(%rsp), %xmm4, %xmm3
	vsubps	%xmm2, %xmm0, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm2, %xmm2
	vandps	%xmm7, %xmm2, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm0
	vcmpltps	%xmm0, %xmm3, %xmm3
	vandnps	%xmm2, %xmm3, %xmm0
.Ltmp5304:
	vmaxps	%xmm12, %xmm1, %xmm1
	vmaxps	%xmm6, %xmm1, %xmm1
	vandps	.LCPI34_6(%rip), %xmm1, %xmm2
	vorps	.LCPI34_7(%rip), %xmm2, %xmm2
	vaddps	%xmm2, %xmm10, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm1, %xmm1
	vpor	.LCPI34_15(%rip), %xmm1, %xmm1
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm1, %xmm1
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm1, %xmm1
	vmovaps	%xmm1, 992(%rsp)
	vsubps	256(%rsp), %xmm1, %xmm1
	vaddps	%xmm1, %xmm9, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm1, %xmm9, %xmm4
	vblendvps	%xmm4, %xmm1, %xmm3, %xmm3
	vcmpleps	%xmm8, %xmm1, %xmm1
	vmulps	1088(%rsp), %xmm3, %xmm3
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%xmm1, %xmm2, %xmm1
	vpandn	%xmm3, %xmm1, %xmm1
	vmovaps	%xmm0, 816(%rsp)
.Ltmp5305:
	vaddps	240(%rsp), %xmm0, %xmm3
	vbroadcastss	.LCPI34_24(%rip), %xmm10
	vmulps	%xmm3, %xmm10, %xmm3
	vbroadcastss	.LCPI34_25(%rip), %xmm8
	vmaxps	%xmm8, %xmm3, %xmm3
	vbroadcastss	.LCPI34_26(%rip), %xmm13
	vminps	%xmm13, %xmm3, %xmm3
	vroundps	$9, %xmm3, %xmm4
	vsubps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_27(%rip), %xmm14
	vmulps	%xmm3, %xmm14, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm12
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm9
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm11
	vaddps	%xmm5, %xmm11, %xmm5
	vbroadcastss	.LCPI34_23(%rip), %xmm15
.Ltmp5306:
	vmaxps	%xmm15, %xmm1, %xmm1
	vminps	%xmm2, %xmm1, %xmm1
	vmovaps	832(%rsp), %xmm0
	vcmpltps	%xmm0, %xmm1, %xmm6
	vmovaps	1056(%rsp), %xmm10
	vblendvps	%xmm6, 1072(%rsp), %xmm10, %xmm6
.Ltmp5307:
	vmulps	%xmm5, %xmm3, %xmm3
.Ltmp5308:
	vsubps	%xmm1, %xmm0, %xmm5
	vmulps	%xmm6, %xmm5, %xmm5
	vaddps	%xmm5, %xmm1, %xmm1
	vmovaps	%xmm7, %xmm13
	vandps	%xmm7, %xmm1, %xmm5
	vbroadcastss	.LCPI34_2(%rip), %xmm14
	vcmpltps	%xmm14, %xmm5, %xmm5
	vandnps	%xmm1, %xmm5, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm6
.Ltmp5309:
	vaddps	%xmm6, %xmm3, %xmm1
	vbroadcastss	.LCPI34_33(%rip), %xmm10
	vaddps	%xmm4, %xmm10, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmovaps	%xmm0, 832(%rsp)
.Ltmp5310:
	vaddps	320(%rsp), %xmm0, %xmm4
.Ltmp5311:
	vmulps	%xmm3, %xmm1, %xmm0
	vmovaps	%xmm0, 960(%rsp)
	vbroadcastss	.LCPI34_24(%rip), %xmm7
.Ltmp5312:
	vmulps	%xmm7, %xmm4, %xmm3
	vmaxps	%xmm8, %xmm3, %xmm3
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm3, %xmm3
	vroundps	$9, %xmm3, %xmm4
	vsubps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_27(%rip), %xmm2
	vmulps	%xmm2, %xmm3, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm8
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm11, %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vaddps	%xmm6, %xmm3, %xmm3
	vaddps	%xmm4, %xmm10, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmovaps	160(%rsp), %xmm0
.Ltmp5313:
	vsubps	496(%rsp), %xmm0, %xmm5
.Ltmp5314:
	vmulps	%xmm4, %xmm3, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm6
.Ltmp5315:
	vaddps	%xmm6, %xmm5, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm11
	vmulps	%xmm3, %xmm11, %xmm3
	vcmpltps	%xmm5, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm5, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm10
	vcmpleps	%xmm10, %xmm5, %xmm4
	vmulps	1040(%rsp), %xmm3, %xmm3
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%xmm4, %xmm0, %xmm4
	vpandn	%xmm3, %xmm4, %xmm3
	vmaxps	%xmm15, %xmm3, %xmm3
	vminps	%xmm0, %xmm3, %xmm3
	vxorps	%xmm15, %xmm15, %xmm15
	vmovaps	848(%rsp), %xmm0
	vcmpltps	%xmm0, %xmm3, %xmm4
	vmovaps	1200(%rsp), %xmm5
	vblendvps	%xmm4, 1216(%rsp), %xmm5, %xmm4
	vsubps	%xmm3, %xmm0, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vandps	%xmm3, %xmm13, %xmm4
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vaddps	560(%rsp), %xmm0, %xmm3
	vmulps	%xmm7, %xmm3, %xmm3
	vbroadcastss	.LCPI34_25(%rip), %xmm14
	vmaxps	%xmm14, %xmm3, %xmm3
	vminps	%xmm1, %xmm3, %xmm3
	vroundps	$9, %xmm3, %xmm4
	vsubps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm2, %xmm3, %xmm5
	vmovaps	%xmm2, %xmm13
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vbroadcastss	.LCPI34_32(%rip), %xmm0
	vaddps	%xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_33(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm3, %xmm3
	vmovaps	992(%rsp), %xmm0
.Ltmp5316:
	vsubps	576(%rsp), %xmm0, %xmm2
	vaddps	%xmm6, %xmm2, %xmm4
	vmulps	%xmm4, %xmm4, %xmm4
	vmulps	%xmm4, %xmm11, %xmm4
	vcmpltps	%xmm2, %xmm6, %xmm5
	vblendvps	%xmm5, %xmm2, %xmm4, %xmm4
	vcmpleps	%xmm10, %xmm2, %xmm2
	vmulps	1184(%rsp), %xmm4, %xmm4
	vpcmpgtd	%xmm2, %xmm15, %xmm2
	vpandn	%xmm4, %xmm2, %xmm2
	vbroadcastss	.LCPI34_23(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm2
	vminps	%xmm15, %xmm2, %xmm2
	vmovaps	1024(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm2, %xmm4
	vmovaps	1152(%rsp), %xmm0
	vblendvps	%xmm4, 1168(%rsp), %xmm0, %xmm4
	vsubps	%xmm2, %xmm5, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm2, %xmm2
	vandps	.LCPI34_1(%rip), %xmm2, %xmm4
	vbroadcastss	.LCPI34_2(%rip), %xmm0
	vcmpltps	%xmm0, %xmm4, %xmm4
	vandnps	%xmm2, %xmm4, %xmm7
	vaddps	640(%rsp), %xmm7, %xmm2
	vbroadcastss	.LCPI34_24(%rip), %xmm0
	vmulps	%xmm0, %xmm2, %xmm2
	vmaxps	%xmm14, %xmm2, %xmm2
	vminps	%xmm1, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm4
	vsubps	%xmm4, %xmm2, %xmm2
	vmulps	%xmm2, %xmm13, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vaddps	%xmm5, %xmm12, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm2, %xmm2
	vbroadcastss	.LCPI34_32(%rip), %xmm0
	vaddps	%xmm0, %xmm2, %xmm2
	vbroadcastss	.LCPI34_33(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm2, %xmm2
.Ltmp5317:
	movq	(%r14), %rcx
	vmovaps	960(%rsp), %xmm0
	vmulps	(%rcx,%rax,4), %xmm0, %xmm1
	movq	16(%r14), %rcx
	vmovaps	160(%rsp), %xmm0
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
	vaddps	%xmm0, %xmm1, %xmm0
.Ltmp5318:
	movq	1328(%r14), %rcx
	vmulps	(%rcx,%rax,4), %xmm3, %xmm1
	.loc	1 1052 27 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp5319:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm2, %xmm2
.Ltmp5320:
	.loc	9 36 14
	vaddps	%xmm2, %xmm1, %xmm1
	movq	888(%rsp), %r10
.Ltmp5321:
	.loc	8 551 14
	vmovups	%xmm0, (%r10,%rdi,4)
	movq	880(%rsp), %rbx
.Ltmp5322:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm1, (%rbx,%rdi,4)
	movq	944(%rsp), %r15
.Ltmp5323:
	.loc	1 0 0
	incq	%r15
.Ltmp5324:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	movq	976(%rsp), %rbp
	cmpq	%r15, %rbp
.Ltmp5325:
	.loc	3 900 12
	jne	.LBB34_340
.Ltmp5326:
.LBB34_375:
	.loc	3 0 12 is_stmt 0
	vmovaps	80(%rsp), %xmm0
	.loc	1 1057 5 is_stmt 1
	vmovaps	%xmm0, 96(%r14)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 112(%r14)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 128(%r14)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 144(%r14)
	vmovaps	32(%rsp), %xmm0
	.loc	1 1058 5
	vmovaps	%xmm0, 1424(%r14)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%r14)
	vmovaps	144(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%r14)
	vmovaps	864(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%r14)
	vmovaps	816(%rsp), %xmm0
	.loc	1 1059 5
	vmovaps	%xmm0, 160(%r14)
	vmovaps	832(%rsp), %xmm0
	vmovaps	%xmm0, 176(%r14)
	vmovaps	848(%rsp), %xmm0
	.loc	1 1060 5
	vmovaps	%xmm0, 1488(%r14)
	vmovaps	%xmm7, 1504(%r14)
	.loc	1 1061 5
	movq	%r11, 2680(%r14)
	xorl	%eax, %eax
	xorl	%edi, %edi
.Ltmp5327:
	.loc	1 0 5 is_stmt 0
.Ltmp5328:
	.p2align	4
.LBB34_376:
	.loc	1 1194 13 is_stmt 1
	vmovd	176(%rsp,%rax), %xmm0
	vmovss	180(%rsp,%rax), %xmm1
	vmovss	184(%rsp,%rax), %xmm2
	vmovss	188(%rsp,%rax), %xmm3
.Ltmp5329:
	.loc	1 1197 17
	vmovd	%xmm0, 192(%r14,%rax)
	.loc	1 1198 34
	movl	204(%r14,%rax), %ecx
	movl	364(%r14,%rax), %edx
.Ltmp5330:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp5331:
	.loc	1 1198 17
	movl	%ecx, 204(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 352(%r14,%rax)
.Ltmp5332:
	.loc	38 2472 13
	subl	%ebp, %edx
	cmovbl	%edi, %edx
.Ltmp5333:
	.loc	1 1198 17
	movl	%edx, 364(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 512(%r14,%rax)
	.loc	1 1198 34
	movl	524(%r14,%rax), %ecx
.Ltmp5334:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp5335:
	.loc	1 1198 17
	movl	%ecx, 524(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm3, 672(%r14,%rax)
	.loc	1 1198 34
	movl	684(%r14,%rax), %ecx
.Ltmp5336:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp5337:
	.loc	1 1198 17
	movl	%ecx, 684(%r14,%rax)
.Ltmp5338:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp5339:
	.loc	3 900 12
	jne	.LBB34_376
.Ltmp5340:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	928(%rsp), %rsi
	movq	936(%rsp), %r15
	.p2align	4
.LBB34_378:
.Ltmp5341:
	.loc	1 1194 13 is_stmt 1
	vmovd	496(%rsp,%rax), %xmm0
	vmovss	500(%rsp,%rax), %xmm1
	vmovss	504(%rsp,%rax), %xmm2
	vmovss	508(%rsp,%rax), %xmm3
.Ltmp5342:
	.loc	1 1197 17
	vmovd	%xmm0, 1520(%r14,%rax)
	.loc	1 1198 34
	movl	1532(%r14,%rax), %ecx
	movl	1692(%r14,%rax), %edx
.Ltmp5343:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp5344:
	.loc	1 1198 17
	movl	%ecx, 1532(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 1680(%r14,%rax)
.Ltmp5345:
	.loc	38 2472 13
	subl	%ebp, %edx
	cmovbl	%edi, %edx
.Ltmp5346:
	.loc	1 1198 17
	movl	%edx, 1692(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 1840(%r14,%rax)
	.loc	1 1198 34
	movl	1852(%r14,%rax), %ecx
.Ltmp5347:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp5348:
	.loc	1 1198 17
	movl	%ecx, 1852(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm3, 2000(%r14,%rax)
	.loc	1 1198 34
	movl	2012(%r14,%rax), %ecx
.Ltmp5349:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edi, %ecx
.Ltmp5350:
	.loc	1 1198 17
	movl	%ecx, 2012(%r14,%rax)
.Ltmp5351:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp5352:
	.loc	3 900 12
	jne	.LBB34_378
	jmp	.LBB34_333
.Ltmp5353:
.LBB34_379:
	.loc	38 1050 16
	cmpq	%r15, %rsi
	movq	912(%rsp), %rax
.Ltmp5354:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_481
.Ltmp5355:
	.loc	48 451 16 is_stmt 1
	cmpq	120(%rsp), %rsi
	ja	.LBB34_481
.Ltmp5356:
	.loc	48 451 16 is_stmt 0
	cmpq	112(%rsp), %rsi
	ja	.LBB34_484
.Ltmp5357:
	.loc	1 972 27 is_stmt 1
	vmovaps	96(%r14), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	112(%r14), %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmovaps	128(%r14), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	144(%r14), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5358:
	.loc	1 973 26
	vmovaps	1424(%r14), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	1440(%r14), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	1456(%r14), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	1472(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
.Ltmp5359:
	.loc	1 974 25
	vmovaps	160(%r14), %xmm0
	vmovaps	%xmm0, 816(%rsp)
	vmovaps	176(%r14), %xmm0
	vmovaps	%xmm0, 832(%rsp)
.Ltmp5360:
	.loc	1 975 24
	vmovaps	1488(%r14), %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vmovaps	1504(%r14), %xmm7
.Ltmp5361:
	.loc	1 976 24
	movq	2680(%r14), %r10
.Ltmp5362:
	.loc	2 1916 50
	testq	%rbp, %rbp
.Ltmp5363:
	.loc	3 900 12
	je	.LBB34_332
.Ltmp5364:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rbp,4), %rcx
	movq	%rcx, 104(%rsp)
	movq	920(%rsp), %rcx
	leaq	(%rcx,%r15,4), %rbx
	leaq	(%rax,%r15,4), %r12
.Ltmp5365:
	.loc	48 568 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %rbp
	movq	%rbp, 896(%rsp)
	xorl	%edi, %edi
	xorl	%ebp, %ebp
	movq	%rbx, 888(%rsp)
	movq	%r12, 880(%rsp)
.Ltmp5366:
	.loc	48 0 12 is_stmt 0
.Ltmp5367:
	.p2align	4
.LBB34_384:
	.loc	1 987 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp5368:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %r9d
	cmovaeq	%r13, %r9
.Ltmp5369:
	.loc	48 568 12
	cmpq	104(%rsp), %rdi
	ja	.LBB34_456
.Ltmp5370:
	.loc	48 438 16
	cmpq	%rbp, 896(%rsp)
	je	.LBB34_449
.Ltmp5371:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,4), %rax
.Ltmp5372:
	.loc	1 1000 29 is_stmt 1
	movq	8(%r14), %rsi
.Ltmp5373:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_457
.Ltmp5374:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp5375:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm7, 1024(%rsp)
	vmovups	(%rbx,%rdi,4), %xmm4
.Ltmp5376:
	vmovups	(%r12,%rdi,4), %xmm14
.Ltmp5377:
	vmovaps	32(%r14), %xmm10
	vmovaps	48(%r14), %xmm11
	vmovaps	64(%r14), %xmm1
	vmovaps	1360(%r14), %xmm15
	vmovaps	1376(%r14), %xmm9
	vmovaps	1392(%r14), %xmm2
	vmovaps	64(%rsp), %xmm8
	vsubps	%xmm8, %xmm4, %xmm0
	vmulps	%xmm0, %xmm11, %xmm3
	vmovaps	80(%rsp), %xmm6
	vmulps	%xmm6, %xmm10, %xmm5
	vaddps	%xmm3, %xmm5, %xmm12
	vaddps	%xmm6, %xmm12, %xmm3
	vmulps	%xmm6, %xmm11, %xmm5
	vmulps	%xmm1, %xmm0, %xmm0
	vaddps	%xmm0, %xmm5, %xmm7
	vaddps	%xmm7, %xmm8, %xmm5
	vmulps	80(%r14), %xmm3, %xmm0
	vmovaps	128(%rsp), %xmm8
	vsubps	%xmm8, %xmm5, %xmm6
	vmulps	48(%rsp), %xmm11, %xmm3
	vmulps	%xmm6, %xmm1, %xmm1
	vaddps	%xmm1, %xmm3, %xmm5
	vaddps	%xmm5, %xmm8, %xmm3
.Ltmp5378:
	vsubps	16(%rsp), %xmm14, %xmm13
	vmulps	%xmm9, %xmm13, %xmm1
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm15, 160(%rsp)
	vmulps	%xmm15, %xmm8, %xmm15
	vaddps	%xmm1, %xmm15, %xmm1
	vaddps	%xmm1, %xmm8, %xmm15
	vmulps	1408(%r14), %xmm15, %xmm15
.Ltmp5379:
	.loc	1 1000 29 is_stmt 1
	movq	(%r14), %rcx
.Ltmp5380:
	.loc	8 551 14
	vmovups	%xmm3, (%rcx,%rax,4)
.Ltmp5381:
	.loc	1 1001 30
	movq	24(%r14), %rsi
.Ltmp5382:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_471
.Ltmp5383:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp5384:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm0, %xmm4, %xmm0
	vsubps	%xmm3, %xmm0, %xmm0
.Ltmp5385:
	.loc	1 1001 30 is_stmt 1
	movq	16(%r14), %rcx
.Ltmp5386:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5387:
	.loc	1 1002 28
	movq	1336(%r14), %rsi
.Ltmp5388:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_472
.Ltmp5389:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp5390:
	.loc	1 0 0 is_stmt 0
	vmulps	32(%rsp), %xmm9, %xmm0
	vmulps	%xmm2, %xmm13, %xmm3
	vaddps	%xmm3, %xmm0, %xmm13
	vaddps	16(%rsp), %xmm13, %xmm0
	vmovaps	144(%rsp), %xmm3
	vsubps	%xmm3, %xmm0, %xmm4
	vmovaps	864(%rsp), %xmm8
	vmulps	%xmm9, %xmm8, %xmm0
	vmulps	%xmm4, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm2
	vaddps	%xmm2, %xmm3, %xmm0
.Ltmp5391:
	.loc	1 1002 28 is_stmt 1
	movq	1328(%r14), %rcx
.Ltmp5392:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5393:
	.loc	1 1003 29
	movq	1352(%r14), %rsi
.Ltmp5394:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_460
.Ltmp5395:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_466
.Ltmp5396:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm15, %xmm14, %xmm3
	vsubps	%xmm0, %xmm3, %xmm0
.Ltmp5397:
	.loc	1 1003 29 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp5398:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
	movq	1104(%r14), %rcx
.Ltmp5399:
	.loc	1 877 35
	addq	%r10, %rcx
.Ltmp5400:
	.loc	1 857 8
	cmpq	%r13, %rcx
	movl	$0, %eax
	cmovaeq	%r13, %rax
.Ltmp5401:
	.loc	1 1006 34
	movq	8(%r14), %rsi
.Ltmp5402:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp5403:
	.loc	1 877 30
	shlq	$2, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_492
	.loc	1 0 25
	movq	1112(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp5404:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rax
.Ltmp5405:
	.loc	1 877 30
	leaq	1(,%rax,4), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB34_491
	.loc	1 0 25
	movq	1120(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp5406:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %rax
.Ltmp5407:
	.loc	1 877 30
	leaq	2(,%rax,4), %r11
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_501
	.loc	1 0 25
	movq	1128(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp5408:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %rax
.Ltmp5409:
	.loc	1 877 30
	leaq	3(,%rax,4), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_504
.Ltmp5410:
	.loc	1 1008 34 is_stmt 1
	movq	24(%r14), %rsi
.Ltmp5411:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB34_492
	cmpq	%rsi, %rdx
	jae	.LBB34_491
	cmpq	%rsi, %r11
	jae	.LBB34_501
	cmpq	%rsi, %r15
	jae	.LBB34_504
.Ltmp5412:
	.loc	1 0 25 is_stmt 0
	movq	%rbp, 944(%rsp)
	movq	2432(%r14), %r8
.Ltmp5413:
	.loc	1 877 35
	addq	%r10, %r8
.Ltmp5414:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %r8
	movl	$0, %eax
	cmovaeq	%r13, %rax
.Ltmp5415:
	.loc	1 1010 34
	movq	1336(%r14), %rsi
.Ltmp5416:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp5417:
	.loc	1 877 30
	shlq	$2, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_488
	.loc	1 0 25
	movq	2440(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp5418:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp5419:
	.loc	1 877 30
	leaq	1(,%rax,4), %rbp
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	.loc	1 0 25
	movq	2448(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp5420:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp5421:
	.loc	1 877 30
	leaq	2(,%rax,4), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_503
	.loc	1 0 25
	movq	2456(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp5422:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ebx
	cmovaeq	%r13, %rbx
	subq	%rbx, %rax
.Ltmp5423:
	.loc	1 877 30
	leaq	3(,%rax,4), %rbx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_508
.Ltmp5424:
	.loc	1 1012 34 is_stmt 1
	movq	1352(%r14), %rsi
.Ltmp5425:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB34_488
	cmpq	%rsi, %rbp
	jae	.LBB34_487
	cmpq	%rsi, %r12
	jae	.LBB34_503
	cmpq	%rsi, %rbx
	jae	.LBB34_508
.Ltmp5426:
	.loc	1 0 0 is_stmt 0
	negq	%r9
	addq	%r9, %r10
	incq	%r10
	leaq	(,%r10,4), %rax
	movq	8(%rsp), %rsi
.Ltmp5427:
	.loc	1 1047 36 is_stmt 1
	movq	8(%rsi), %rsi
.Ltmp5428:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_462
.Ltmp5429:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp5430:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 1049 27 is_stmt 1
	movq	24(%rsi), %rsi
.Ltmp5431:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_463
.Ltmp5432:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp5433:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
.Ltmp5434:
	.loc	1 1050 35 is_stmt 1
	movq	1336(%rsi), %rsi
.Ltmp5435:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_464
.Ltmp5436:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp5437:
	.loc	48 0 16 is_stmt 0
	movq	8(%rsp), %rsi
	.loc	1 1052 27 is_stmt 1
	movq	1352(%rsi), %rsi
.Ltmp5438:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rax, %r9
	jb	.LBB34_465
.Ltmp5439:
	.loc	48 438 16
	cmpq	$3, %r9
	jbe	.LBB34_447
.Ltmp5440:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm12, %xmm12, %xmm0
	vaddps	80(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm12
	vandps	%xmm0, %xmm12, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm14
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vaddps	%xmm7, %xmm7, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmulps	%xmm6, %xmm11, %xmm0
	vmovaps	48(%rsp), %xmm6
	vmulps	%xmm6, %xmm10, %xmm3
	vaddps	%xmm0, %xmm3, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm6, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm5, %xmm5, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5441:
	vaddps	%xmm1, %xmm1, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm1
	vcmpltps	%xmm14, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm1
	vcmpltps	%xmm14, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 16(%rsp)
	movq	8(%rsp), %r14
.Ltmp5442:
	movq	(%r14), %rsi
	vmovd	(%rsi,%rcx,4), %xmm0
	vpinsrd	$1, (%rsi,%rdx,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%r11,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r15,4), %xmm0, %xmm0
.Ltmp5443:
	movq	16(%r14), %rsi
	vmovd	(%rsi,%rcx,4), %xmm1
	vpinsrd	$1, (%rsi,%rdx,4), %xmm1, %xmm1
.Ltmp5444:
	vmulps	%xmm4, %xmm9, %xmm3
.Ltmp5445:
	vpinsrd	$2, (%rsi,%r11,4), %xmm1, %xmm1
.Ltmp5446:
	vmulps	160(%rsp), %xmm8, %xmm4
	vaddps	%xmm3, %xmm4, %xmm3
.Ltmp5447:
	vpinsrd	$3, (%rsi,%r15,4), %xmm1, %xmm1
.Ltmp5448:
	vaddps	%xmm3, %xmm3, %xmm3
	vaddps	%xmm3, %xmm8, %xmm3
	vandps	%xmm3, %xmm12, %xmm4
	vcmpltps	%xmm14, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm3
	vmovaps	%xmm3, 864(%rsp)
	vaddps	%xmm2, %xmm2, %xmm2
	vaddps	144(%rsp), %xmm2, %xmm2
.Ltmp5449:
	movq	1328(%r14), %rcx
	vmovd	(%rcx,%r8,4), %xmm3
	vpinsrd	$1, (%rcx,%rbp,4), %xmm3, %xmm3
	vpinsrd	$2, (%rcx,%r12,4), %xmm3, %xmm3
	vpinsrd	$3, (%rcx,%rbx,4), %xmm3, %xmm3
.Ltmp5450:
	movq	1344(%r14), %rcx
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rbp,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%r12,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rbx,4), %xmm4, %xmm4
.Ltmp5451:
	vandps	%xmm2, %xmm12, %xmm5
	vcmpltps	%xmm14, %xmm5, %xmm5
	vandnps	%xmm2, %xmm5, %xmm2
	vmovaps	%xmm2, 144(%rsp)
.Ltmp5452:
	vpand	%xmm0, %xmm12, %xmm0
	vpand	%xmm3, %xmm12, %xmm2
	vbroadcastss	.LCPI34_3(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
.Ltmp5453:
	vpand	%xmm1, %xmm12, %xmm1
	vpand	%xmm4, %xmm12, %xmm2
	vmulps	%xmm3, %xmm1, %xmm1
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm2
	vbroadcastss	.LCPI34_4(%rip), %xmm5
.Ltmp5454:
	vmaxps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm6
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm7
	vandps	%xmm7, %xmm0, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm8
	vorps	%xmm1, %xmm8, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm9
	vaddps	%xmm1, %xmm9, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm10
	vmulps	%xmm1, %xmm10, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm11
	vaddps	%xmm3, %xmm11, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm14
	vmaxps	%xmm14, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm8
	vminps	%xmm8, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vsubps	176(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm0, %xmm1, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm7
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1136(%rsp), %xmm3, %xmm3
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm0, %xmm1, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	816(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm3
	vmovaps	1104(%rsp), %xmm4
	vblendvps	%xmm3, 1120(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm1, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm1
.Ltmp5455:
	vbroadcastss	.LCPI34_4(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm0
	vmaxps	%xmm6, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm2
	vorps	.LCPI34_7(%rip), %xmm2, %xmm2
	vaddps	%xmm2, %xmm9, %xmm2
	vmulps	%xmm2, %xmm10, %xmm3
	vaddps	%xmm3, %xmm11, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm14, %xmm0, %xmm0
	vminps	%xmm8, %xmm0, %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vsubps	256(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm3
	vaddps	%xmm3, %xmm0, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vmulps	%xmm5, %xmm2, %xmm2
	vcmpltps	%xmm0, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm0, %xmm2, %xmm2
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1088(%rsp), %xmm2, %xmm2
	vxorps	%xmm4, %xmm4, %xmm4
	vpcmpgtd	%xmm0, %xmm4, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm1, 816(%rsp)
.Ltmp5456:
	vaddps	240(%rsp), %xmm1, %xmm2
	vbroadcastss	.LCPI34_24(%rip), %xmm9
	vmulps	%xmm2, %xmm9, %xmm2
	vbroadcastss	.LCPI34_25(%rip), %xmm12
	vmaxps	%xmm12, %xmm2, %xmm2
	vbroadcastss	.LCPI34_26(%rip), %xmm11
	vminps	%xmm11, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm3
	vsubps	%xmm3, %xmm2, %xmm2
	vbroadcastss	.LCPI34_27(%rip), %xmm13
	vmulps	%xmm2, %xmm13, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm7
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm8
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vbroadcastss	.LCPI34_23(%rip), %xmm1
.Ltmp5457:
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm4, %xmm0, %xmm0
	vmovaps	832(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm6
	vmovaps	1056(%rsp), %xmm9
	vblendvps	%xmm6, 1072(%rsp), %xmm9, %xmm6
.Ltmp5458:
	vmulps	%xmm5, %xmm2, %xmm2
.Ltmp5459:
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm6, %xmm5, %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm14
	vandps	%xmm0, %xmm14, %xmm5
	vbroadcastss	.LCPI34_2(%rip), %xmm13
	vcmpltps	%xmm13, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm6
.Ltmp5460:
	vaddps	%xmm6, %xmm2, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm9
	vaddps	%xmm3, %xmm9, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vmovaps	%xmm1, 832(%rsp)
.Ltmp5461:
	vaddps	320(%rsp), %xmm1, %xmm3
.Ltmp5462:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 960(%rsp)
	vbroadcastss	.LCPI34_24(%rip), %xmm2
.Ltmp5463:
	vmulps	%xmm2, %xmm3, %xmm0
	vmaxps	%xmm12, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm4
	vmulps	%xmm4, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm10, %xmm5
	vmovaps	%xmm10, %xmm15
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm0
	vaddps	%xmm3, %xmm9, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm1
.Ltmp5464:
	vsubps	496(%rsp), %xmm1, %xmm5
.Ltmp5465:
	vmulps	%xmm3, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm6
.Ltmp5466:
	vaddps	%xmm6, %xmm5, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm10
	vmulps	%xmm0, %xmm10, %xmm0
	vcmpltps	%xmm5, %xmm6, %xmm3
	vblendvps	%xmm3, %xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm9
	vcmpleps	%xmm9, %xmm5, %xmm3
	vmulps	1040(%rsp), %xmm0, %xmm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm3, %xmm1, %xmm3
	vpandn	%xmm0, %xmm3, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vxorps	%xmm12, %xmm12, %xmm12
	vmovaps	848(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm3
	vmovaps	1200(%rsp), %xmm5
	vblendvps	%xmm3, 1216(%rsp), %xmm5, %xmm3
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm3, %xmm5, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm3
	vcmpltps	%xmm13, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vaddps	560(%rsp), %xmm0, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm13
	vmaxps	%xmm13, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vmulps	%xmm4, %xmm0, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmovaps	%xmm7, %xmm11
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm5
	vmovaps	992(%rsp), %xmm0
.Ltmp5467:
	vsubps	576(%rsp), %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm3, %xmm10, %xmm3
	vcmpltps	%xmm0, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vcmpleps	%xmm9, %xmm0, %xmm0
	vmulps	1184(%rsp), %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm12, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm12, %xmm0, %xmm0
	vmovaps	1024(%rsp), %xmm4
	vcmpltps	%xmm4, %xmm0, %xmm3
	vmovaps	1152(%rsp), %xmm1
	vblendvps	%xmm3, 1168(%rsp), %xmm1, %xmm3
	vsubps	%xmm0, %xmm4, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm7
	vaddps	640(%rsp), %xmm7, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm13, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm4
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm11, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm8, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
.Ltmp5468:
	movq	(%r14), %rcx
	vmovaps	960(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm2
	movq	16(%r14), %rcx
	vmovaps	160(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm1
	vaddps	%xmm1, %xmm2, %xmm1
.Ltmp5469:
	movq	1328(%r14), %rcx
	vmulps	(%rcx,%rax,4), %xmm5, %xmm2
	.loc	1 1052 27 is_stmt 1
	movq	1344(%r14), %rcx
.Ltmp5470:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
.Ltmp5471:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	888(%rsp), %rbx
.Ltmp5472:
	.loc	8 551 14
	vmovups	%xmm1, (%rbx,%rdi,4)
	movq	880(%rsp), %r12
.Ltmp5473:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r12,%rdi,4)
	movq	944(%rsp), %rbp
.Ltmp5474:
	.loc	1 0 0
	incq	%rbp
.Ltmp5475:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	cmpq	%rbp, 976(%rsp)
.Ltmp5476:
	.loc	3 900 12
	jne	.LBB34_384
	jmp	.LBB34_332
.Ltmp5477:
.LBB34_419:
	.loc	3 0 12 is_stmt 0
	movabsq	$2305843009213693948, %rax
.Ltmp5478:
	.loc	9 504 14 is_stmt 1
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqps	%xmm0, %xmm0, %xmm0
	movq	120(%rsp), %rdx
.Ltmp5479:
	.loc	10 2155 12
	movq	%rdx, %rcx
	vmovaps	%xmm0, %xmm1
	andq	%rax, %rcx
	movq	920(%rsp), %rdi
	je	.LBB34_422
.Ltmp5480:
	.loc	10 0 12 is_stmt 0
	xorl	%esi, %esi
	vbroadcastss	.LCPI34_35(%rip), %xmm2
	vbroadcastss	.LCPI34_34(%rip), %xmm3
	vmovaps	%xmm0, %xmm1
	.p2align	4
.LBB34_421:
.Ltmp5481:
	.loc	9 257 24 is_stmt 1
	vandps	(%rdi,%rsi,4), %xmm2, %xmm4
.Ltmp5482:
	.loc	9 517 14
	vcmpltps	%xmm3, %xmm4, %xmm4
.Ltmp5483:
	.loc	9 257 24
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp5484:
	.loc	10 2155 12
	addq	$4, %rsi
	cmpq	%rsi, %rcx
	jne	.LBB34_421
.Ltmp5485:
.LBB34_422:
	.loc	13 285 9
	vpcmpeqd	%xmm2, %xmm2, %xmm2
	vtestps	%xmm2, %xmm1
	movq	112(%rsp), %rbx
	movq	912(%rsp), %r9
.Ltmp5486:
	.loc	50 208 8
	jae	.LBB34_427
.Ltmp5487:
	.loc	10 2155 12
	movq	%rbx, %r8
	vmovaps	%xmm0, %xmm1
	andq	%rax, %r8
	je	.LBB34_426
.Ltmp5488:
	.loc	10 0 12 is_stmt 0
	xorl	%esi, %esi
	vbroadcastss	.LCPI34_35(%rip), %xmm2
	vbroadcastss	.LCPI34_34(%rip), %xmm3
	vmovaps	%xmm0, %xmm1
	.p2align	4
.LBB34_425:
.Ltmp5489:
	.loc	9 257 24 is_stmt 1
	vandps	(%r9,%rsi,4), %xmm2, %xmm4
.Ltmp5490:
	.loc	9 517 14
	vcmpltps	%xmm3, %xmm4, %xmm4
.Ltmp5491:
	.loc	9 257 24
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp5492:
	.loc	10 2155 12
	addq	$4, %rsi
	cmpq	%rsi, %r8
	jne	.LBB34_425
.Ltmp5493:
.LBB34_426:
	.loc	13 285 9
	vpcmpeqd	%xmm2, %xmm2, %xmm2
	vtestps	%xmm2, %xmm1
.Ltmp5494:
	.loc	50 208 34
	jb	.LBB34_442
.LBB34_427:
	.loc	50 0 34 is_stmt 0
	vmovaps	%xmm0, %xmm1
.Ltmp5495:
	.loc	10 2155 12 is_stmt 1
	testq	%rcx, %rcx
.Ltmp5496:
	.loc	10 2155 12 is_stmt 0
	je	.LBB34_430
.Ltmp5497:
	.loc	10 0 12
	xorl	%esi, %esi
	vbroadcastss	.LCPI34_35(%rip), %xmm2
	vbroadcastss	.LCPI34_34(%rip), %xmm3
	vmovaps	%xmm0, %xmm1
	.p2align	4
.LBB34_429:
.Ltmp5498:
	.loc	9 257 24 is_stmt 1
	vandps	(%rdi,%rsi,4), %xmm2, %xmm4
.Ltmp5499:
	.loc	9 517 14
	vcmpltps	%xmm3, %xmm4, %xmm4
.Ltmp5500:
	.loc	9 257 24
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp5501:
	.loc	10 2155 12
	addq	$4, %rsi
	cmpq	%rsi, %rcx
	jne	.LBB34_429
.Ltmp5502:
.LBB34_430:
	.file	55 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/sse41.rs"
	.loc	55 131 19
	vpsrad	$31, %xmm1, %xmm2
	vpbroadcastd	.LCPI34_32(%rip), %xmm1
	vpandn	%xmm1, %xmm2, %xmm2
.Ltmp5503:
	.loc	50 185 12
	vmovd	%xmm2, %ecx
	xorl	%ebp, %ebp
	testl	%ecx, %ecx
	setne	%bpl
	vpextrd	$1, %xmm2, %esi
	xorl	%ecx, %ecx
	testl	%esi, %esi
	setne	%cl
	vpextrd	$2, %xmm2, %r8d
	addl	%ecx, %ecx
	xorl	%esi, %esi
	testl	%r8d, %r8d
	setne	%sil
	shll	$2, %esi
	vpextrd	$3, %xmm2, %r8d
	xorl	%r15d, %r15d
	testl	%r8d, %r8d
	setne	%r15b
	shll	$3, %r15d
.Ltmp5504:
	.loc	10 2155 12
	andq	%rbx, %rax
	je	.LBB34_433
.Ltmp5505:
	.loc	10 0 12 is_stmt 0
	xorl	%r8d, %r8d
	vbroadcastss	.LCPI34_35(%rip), %xmm2
	vbroadcastss	.LCPI34_34(%rip), %xmm3
	.p2align	4
.LBB34_432:
.Ltmp5506:
	.loc	9 257 24 is_stmt 1
	vandps	(%r9,%r8,4), %xmm2, %xmm4
.Ltmp5507:
	.loc	9 517 14
	vcmpltps	%xmm3, %xmm4, %xmm4
.Ltmp5508:
	.loc	9 257 24
	vandps	%xmm4, %xmm0, %xmm0
.Ltmp5509:
	.loc	10 2155 12
	addq	$4, %r8
	cmpq	%r8, %rax
	jne	.LBB34_432
.Ltmp5510:
.LBB34_433:
	.loc	55 131 19
	vpsrad	$31, %xmm0, %xmm0
	vpandn	%xmm1, %xmm0, %xmm0
.Ltmp5511:
	.loc	50 185 12
	vmovd	%xmm0, %r8d
	xorl	%eax, %eax
	testl	%r8d, %r8d
	vpextrd	$1, %xmm0, %r9d
	setne	%al
	xorl	%r8d, %r8d
	testl	%r9d, %r9d
	setne	%r8b
	addl	%r8d, %r8d
	vpextrd	$2, %xmm0, %r9d
	xorl	%r10d, %r10d
	testl	%r9d, %r9d
	setne	%r10b
	vpextrd	$3, %xmm0, %r9d
	shll	$2, %r10d
	xorl	%r11d, %r11d
	testl	%r9d, %r9d
	setne	%r11b
	shll	$3, %r11d
.Ltmp5512:
	.loc	50 185 12 is_stmt 0
	orl	%r10d, %r11d
.Ltmp5513:
	.loc	50 185 12
	orl	%ebp, %ecx
	orl	%esi, %ecx
	orl	%r15d, %ecx
.Ltmp5514:
	.loc	50 185 12
	orl	%eax, %ecx
	orl	%r8d, %ecx
.Ltmp5515:
	.loc	50 211 5 is_stmt 1
	orl	%r11d, %ecx
	movl	%ecx, 2664(%r14)
	.loc	50 212 31
	movq	2656(%r14), %rax
.Ltmp5516:
	.loc	38 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp5517:
	.loc	50 212 5
	movq	%rcx, 2656(%r14)
.Ltmp5518:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp5519:
	.loc	33 180 28
	je	.LBB34_435
.Ltmp5520:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp5521:
	.loc	35 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp5522:
.LBB34_435:
	.loc	32 1714 9
	testq	%rbx, %rbx
.Ltmp5523:
	.loc	33 180 28
	je	.LBB34_437
.Ltmp5524:
	.loc	34 961 18
	shlq	$2, %rbx
	movq	912(%rsp), %rdi
.Ltmp5525:
	.loc	35 25 13
	xorl	%esi, %esi
	movq	%rbx, %rdx
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp5526:
.LBB34_437:
	.loc	1 1319 13
	movq	$0, 2680(%r14)
.Ltmp5527:
	.loc	1 1321 22
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E19discontinuity_resetB5_
.Ltmp5528:
	.loc	32 1714 9
	leaq	1328(%r14), %rdi
.Ltmp5529:
	.loc	1 1321 22
	callq	_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E19discontinuity_resetB5_
.Ltmp5530:
	.loc	1 1327 20
	movl	2664(%r14), %eax
.Ltmp5531:
	.loc	1 1330 16
	testb	$1, %al
	movq	928(%rsp), %rsi
	jne	.LBB34_443
	testb	$2, %al
	jne	.LBB34_444
.LBB34_439:
	testb	$4, %al
	jne	.LBB34_445
.LBB34_440:
	testb	$8, %al
	je	.LBB34_442
.LBB34_441:
	.loc	1 0 16 is_stmt 0
	movq	1616(%rsp), %rax
.Ltmp5532:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rax
	movq	$-1, %rcx
	cmovbq	%rcx, %rax
.Ltmp5533:
	.loc	1 1331 17
	movq	%rax, 1616(%rsp)
.Ltmp5534:
	.loc	38 2428 13
	addq	1624(%rsp), %rsi
	cmovbq	%rcx, %rsi
.Ltmp5535:
	.loc	1 1332 17
	movq	%rsi, 1624(%rsp)
.Ltmp5536:
.LBB34_442:
	.loc	1 0 17 is_stmt 0
	leaq	1472(%rsp), %rsi
	movl	$328, %edx
	movq	1240(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp5537:
	.loc	1 1910 6 is_stmt 1
	movq	%rbx, %rax
	.loc	1 1910 6 epilogue_begin is_stmt 0
	addq	$2200, %rsp
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
.LBB34_443:
	.cfi_def_cfa_offset 2256
	.loc	1 0 6
	movq	1496(%rsp), %rcx
.Ltmp5538:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp5539:
	.loc	1 1331 17
	movq	%rcx, 1496(%rsp)
	movq	1504(%rsp), %rcx
.Ltmp5540:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp5541:
	.loc	1 1332 17
	movq	%rcx, 1504(%rsp)
	.loc	1 1330 16
	testb	$2, %al
	je	.LBB34_439
.LBB34_444:
	.loc	1 0 16 is_stmt 0
	movq	1536(%rsp), %rcx
.Ltmp5542:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp5543:
	.loc	1 1331 17
	movq	%rcx, 1536(%rsp)
	movq	1544(%rsp), %rcx
.Ltmp5544:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp5545:
	.loc	1 1332 17
	movq	%rcx, 1544(%rsp)
	.loc	1 1330 16
	testb	$4, %al
	je	.LBB34_440
.LBB34_445:
	.loc	1 0 16 is_stmt 0
	movq	1576(%rsp), %rcx
.Ltmp5546:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp5547:
	.loc	1 1331 17
	movq	%rcx, 1576(%rsp)
	movq	1584(%rsp), %rcx
.Ltmp5548:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp5549:
	.loc	1 1332 17
	movq	%rcx, 1584(%rsp)
	.loc	1 1330 16
	testb	$8, %al
	jne	.LBB34_441
	jmp	.LBB34_442
.Ltmp5550:
.LBB34_466:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_9dcceffc0d89ad4d6a9a2e0685e8ac95(%rip), %rcx
	movl	$4, %esi
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_447:
	leaq	.Lalloc_9dcceffc0d89ad4d6a9a2e0685e8ac95(%rip), %rcx
	movl	$4, %esi
	xorl	%edi, %edi
	movq	%r9, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_448:
.Ltmp5551:
	leaq	.Lalloc_9dcceffc0d89ad4d6a9a2e0685e8ac95(%rip), %rcx
	movl	$4, %esi
	xorl	%edi, %edi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5552:
.LBB34_449:
	leaq	.Lalloc_9dcceffc0d89ad4d6a9a2e0685e8ac95(%rip), %rcx
	movl	$4, %esi
	xorl	%edi, %edi
	xorl	%edx, %edx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5553:
.LBB34_450:
	.loc	48 443 13 is_stmt 1
	leaq	.Lalloc_8ba50ea0f27235615a85de9e15747d68(%rip), %rcx
	movq	48(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5554:
.LBB34_451:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_e1197c8f188c3c9faae54184f2eb21df(%rip), %rcx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_452:
	leaq	.Lalloc_a5fe42438bf1cd51848d461e43168e01(%rip), %rcx
.Ltmp5555:
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_453:
.Ltmp5556:
	leaq	.Lalloc_947a12da53e0da5ef683a81874315ce0(%rip), %rcx
.Ltmp5557:
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_454:
.Ltmp5558:
	leaq	.Lalloc_3c433846b75a8609e2aa4275946b39b4(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_455:
	leaq	.Lalloc_f7cdc51debee5c62c8dce89684c9c22f(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5559:
.LBB34_456:
	leaq	.Lalloc_e1197c8f188c3c9faae54184f2eb21df(%rip), %rcx
	movq	104(%rsp), %rdx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_457:
	leaq	.Lalloc_f7aeb6b0a3ba8e73c50a5abef6c30558(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_471:
	leaq	.Lalloc_a8e669d1bed0fb747f8a7bff920a6571(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_472:
	leaq	.Lalloc_1e81c2bc19b75441ce2fb90ce8f9eb70(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_460:
	leaq	.Lalloc_007bf1cfdcf9f845db5ef166fc9a1790(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_462:
	leaq	.Lalloc_a45b75cd2d07085fe69fe186ba115725(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_463:
	leaq	.Lalloc_36be93341d7083c8a492c530428430ea(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_464:
	leaq	.Lalloc_8a6f1b2a44d3e33eba5c708677237c81(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_465:
	leaq	.Lalloc_4aa2eaec3d1833a4a887fe1d76c05ca7(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_467:
.Ltmp5560:
	leaq	.Lalloc_a45b75cd2d07085fe69fe186ba115725(%rip), %rcx
	movq	%r10, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_468:
	leaq	.Lalloc_36be93341d7083c8a492c530428430ea(%rip), %rcx
	movq	%r10, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_469:
	leaq	.Lalloc_8a6f1b2a44d3e33eba5c708677237c81(%rip), %rcx
	movq	%r10, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_470:
	leaq	.Lalloc_4aa2eaec3d1833a4a887fe1d76c05ca7(%rip), %rcx
	movq	%r10, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5561:
.LBB34_478:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_a05c61cb172b1016332ce1d3ce81e461(%rip), %rcx
.Ltmp5562:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	120(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5563:
.LBB34_479:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_61a2f59006034c74bb8ab3eed52140c6(%rip), %rcx
.Ltmp5564:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	120(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5565:
.LBB34_480:
	.loc	1 0 0
	leaq	.Lalloc_61a2f59006034c74bb8ab3eed52140c6(%rip), %rcx
	movq	%r15, %rdi
	movq	120(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_481:
	leaq	.Lalloc_a05c61cb172b1016332ce1d3ce81e461(%rip), %rcx
	movq	%r15, %rdi
	movq	120(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_482:
.Ltmp5566:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_2f8086566e4ce1876649681c0220d707(%rip), %rcx
.Ltmp5567:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	112(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5568:
.LBB34_483:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_24e70b7e4de4498f6c3c29a55eaeb9ab(%rip), %rcx
.Ltmp5569:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	112(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5570:
.LBB34_484:
	.loc	1 0 0
	leaq	.Lalloc_2f8086566e4ce1876649681c0220d707(%rip), %rcx
	movq	%r15, %rdi
	movq	112(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_486:
	leaq	.Lalloc_24e70b7e4de4498f6c3c29a55eaeb9ab(%rip), %rcx
	movq	%r15, %rdi
	movq	112(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_508:
	movq	%rbx, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_487:
	movq	%rbp, %r8
.LBB34_488:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_504:
	movq	%r15, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5571:
.LBB34_489:
	.loc	1 1892 25 is_stmt 1
	leaq	.Lalloc_9f2c0b91206fa9484b77a809bfc45da2(%rip), %rdx
	movq	%r8, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_490:
.Ltmp5572:
	.loc	1 1893 23
	leaq	.Lalloc_81c87390c70034a3e9ca28b66f0c0cdf(%rip), %rdx
	movq	%rcx, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5573:
.LBB34_491:
	.loc	1 0 23 is_stmt 0
	movq	%rdx, %rcx
.LBB34_492:
.Ltmp5574:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_503:
	movq	%r12, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_493:
	movq	%rdx, %rax
.LBB34_494:
.Ltmp5575:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5576:
.LBB34_506:
	movq	%r12, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_501:
	movq	%r11, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_530:
	movq	%r10, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_514:
	movq	%r15, %rax
.Ltmp5577:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5578:
.LBB34_495:
	.loc	1 1376 43 is_stmt 1
	leaq	.Lalloc_26decc7ea6b284c3e2e2a8674b90acb8(%rip), %rdx
	movl	$12, %esi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5579:
.LBB34_496:
	.loc	1 0 43 is_stmt 0
	movq	%rdx, %rcx
.Ltmp5580:
	.loc	1 877 25 is_stmt 1
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5581:
.LBB34_510:
	.loc	1 0 25 is_stmt 0
	movq	%rcx, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_497:
	movq	%rbp, %r8
.Ltmp5582:
	.loc	1 877 25
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5583:
.LBB34_545:
	.loc	1 0 25
	movq	%r11, %rax
.Ltmp5584:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_523:
	movq	%r12, %rax
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5585:
.LBB34_498:
	.loc	1 1378 45 is_stmt 1
	leaq	.Lalloc_229cdd12ec7f4a0b614787777e6def50(%rip), %rdx
	movl	$10, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5586:
.LBB34_536:
	.loc	1 0 45 is_stmt 0
	movq	%r15, %rcx
.Ltmp5587:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_543:
	movq	%rbx, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5588:
.Lfunc_end34:
	.size	_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_, .Lfunc_end34-_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_
