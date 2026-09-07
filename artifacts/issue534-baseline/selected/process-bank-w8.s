_RNvXse_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank:
.Lfunc_begin46:
	.loc	6 1069 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
.Ltmp7759:
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	andq	$-32, %rsp
	subq	$1152, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.loc	6 1070 29 prologue_end
	movzbl	2624(%rsi), %eax
.Ltmp7760:
	.loc	18 966 15
	cmpb	$2, %al
	.loc	18 966 9 is_stmt 0
	je	.LBB46_189
.Ltmp7761:
	.loc	20 186 45 is_stmt 1
	cmpb	%al, 108(%rdx)
.Ltmp7762:
	.loc	6 1075 20
	jne	.LBB46_60
	cmpq	$0, 64(%rdx)
	jne	.LBB46_60
	.loc	6 0 20 is_stmt 0
	movq	%rsi, %r10
	movb	%al, 79(%rsp)
	movq	%rdi, 568(%rsp)
	.loc	6 1080 35 is_stmt 1
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqa	%ymm0, 1088(%rsp)
	vmovdqa	%ymm0, 1056(%rsp)
	vmovdqa	%ymm0, 1024(%rsp)
	vmovdqa	%ymm0, 992(%rsp)
	vmovdqa	%ymm0, 960(%rsp)
	vmovdqa	%ymm0, 928(%rsp)
	vmovdqa	%ymm0, 896(%rsp)
	vmovdqa	%ymm0, 864(%rsp)
	vmovdqa	%ymm0, 832(%rsp)
	vmovdqa	%ymm0, 800(%rsp)
	movq	48(%rdx), %r11
	movq	56(%rdx), %r8
	movq	32(%rdx), %rax
	movq	%rax, 128(%rsp)
	movq	40(%rdx), %rax
	movq	%rax, 64(%rsp)
	movq	%rdx, 448(%rsp)
	movq	96(%rdx), %rax
	movq	%rax, 56(%rsp)
	leaq	1280(%rsi), %rax
	movq	%rax, 408(%rsp)
.Ltmp7763:
	.loc	11 900 12
	cmpq	$1, %r8
	movq	%r8, %rax
	adcq	$-1, %rax
	movq	%rax, 96(%rsp)
	xorl	%ebx, %ebx
	vmovss	.LCPI46_0(%rip), %xmm5
	vmovss	.LCPI46_1(%rip), %xmm6
	movq	%rsi, 48(%rsp)
	movq	%r8, 104(%rsp)
	movq	%r11, 120(%rsp)
	jmp	.LBB46_5
	.loc	11 0 12 is_stmt 0
.Ltmp7764:
	.p2align	4
.LBB46_4:
	movq	80(%rsp), %rax
	movq	%rax, %rbx
.Ltmp7765:
	.loc	8 1916 50 is_stmt 1
	cmpq	$8, %rax
.Ltmp7766:
	.loc	11 900 12
	je	.LBB46_61
.Ltmp7767:
.LBB46_5:
	.loc	6 1082 33
	cmpq	%r8, %rbx
	je	.LBB46_199
.Ltmp7768:
	.loc	6 0 0 is_stmt 0
	leaq	1(%rbx), %rax
.Ltmp7769:
	.loc	6 1083 31 is_stmt 1
	cmpq	96(%rsp), %rbx
	je	.LBB46_200
	.loc	6 0 31 is_stmt 0
	movl	(%r11,%rbx,4), %edi
	.loc	6 1083 31
	movl	(%r11,%rax,4), %esi
.Ltmp7770:
	.loc	15 1050 16 is_stmt 1
	cmpl	%edi, %esi
	jb	.LBB46_176
	cmpq	%rsi, 64(%rsp)
	jb	.LBB46_176
.Ltmp7771:
	.loc	15 0 16 is_stmt 0
	movq	%rax, 80(%rsp)
	movl	$0, 256(%rsp)
	movl	$0, 264(%rsp)
	movl	$0, 272(%rsp)
	movl	$0, 280(%rsp)
	movl	$0, 288(%rsp)
	movl	$0, 296(%rsp)
	movl	$0, 304(%rsp)
	movl	$0, 312(%rsp)
.Ltmp7772:
	.loc	38 1714 9 is_stmt 1
	cmpl	%edi, %esi
.Ltmp7773:
	.loc	19 180 28
	jne	.LBB46_44
.Ltmp7774:
.LBB46_10:
	.loc	19 0 28 is_stmt 0
	movq	408(%rsp), %rax
	xorl	%ecx, %ecx
.Ltmp7775:
	.loc	19 180 28
	jmp	.LBB46_13
.Ltmp7776:
	.loc	19 0 28
.Ltmp7777:
	.p2align	4
.LBB46_11:
	.loc	6 396 5 is_stmt 1
	vmovaps	384(%rax), %ymm4
	vmovaps	%ymm4, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm1
	vmovaps	%ymm1, 384(%rax)
.Ltmp7778:
	.loc	6 396 5
	vmovaps	416(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, 416(%rax)
.Ltmp7779:
	.loc	6 396 5
	vmovaps	448(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm2, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, 448(%rax)
.Ltmp7780:
	.loc	6 396 5
	vmovaps	480(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm3, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovdqa	(%rsp), %ymm0
	vmovdqa	%ymm0, 480(%rax)
.Ltmp7781:
	.loc	6 661 17
	movl	$64, 2620(%r10)
.Ltmp7782:
.LBB46_12:
	.loc	6 0 0 is_stmt 0
	addq	$32, %rcx
.Ltmp7783:
	.loc	38 1714 9 is_stmt 1
	addq	$608, %rax
	cmpq	$64, %rcx
.Ltmp7784:
	.loc	19 180 28
	je	.LBB46_4
.Ltmp7785:
.LBB46_13:
	.loc	6 648 21
	cmpl	$1, 256(%rsp,%rcx)
	jne	.LBB46_14
	.loc	6 648 26 is_stmt 0
	vmovd	260(%rsp,%rcx), %xmm0
.Ltmp7786:
	.loc	6 651 39 is_stmt 1
	vmovaps	(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
.Ltmp7787:
	.loc	6 390 5
	vmovd	(%rsp,%rbx,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp7788:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp7789:
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
	jne	.LBB46_18
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	je	.LBB46_36
.LBB46_19:
	.loc	47 0 9
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB46_21
.Ltmp7790:
.LBB46_20:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB46_21:
.Ltmp7791:
	.loc	6 396 5 is_stmt 1
	vmovaps	(%rax), %ymm4
	vmovaps	%ymm4, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm1
	vmovaps	%ymm1, (%rax)
.Ltmp7792:
	.loc	6 396 5
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, 32(%rax)
.Ltmp7793:
	.loc	6 396 5
	vmovaps	64(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm2, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, 64(%rax)
.Ltmp7794:
	.loc	6 396 5
	vmovaps	96(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm3, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovdqa	(%rsp), %ymm0
	vmovdqa	%ymm0, 96(%rax)
.Ltmp7795:
	.loc	6 661 17
	movl	$64, 2620(%r10)
.Ltmp7796:
	.loc	6 648 21
	cmpl	$1, 264(%rsp,%rcx)
	je	.LBB46_22
.LBB46_15:
	cmpl	$1, 272(%rsp,%rcx)
	jne	.LBB46_16
.LBB46_27:
	.loc	6 648 26 is_stmt 0
	vmovd	276(%rsp,%rcx), %xmm0
.Ltmp7797:
	.loc	6 651 39 is_stmt 1
	vmovaps	256(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
.Ltmp7798:
	.loc	6 390 5
	vmovd	(%rsp,%rbx,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp7799:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp7800:
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
	jne	.LBB46_28
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	je	.LBB46_40
.LBB46_29:
	.loc	47 0 9
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB46_31
.Ltmp7801:
.LBB46_30:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB46_31:
.Ltmp7802:
	.loc	6 396 5 is_stmt 1
	vmovaps	256(%rax), %ymm4
	vmovaps	%ymm4, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm1
	vmovaps	%ymm1, 256(%rax)
.Ltmp7803:
	.loc	6 396 5
	vmovaps	288(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, 288(%rax)
.Ltmp7804:
	.loc	6 396 5
	vmovaps	320(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm2, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, 320(%rax)
.Ltmp7805:
	.loc	6 396 5
	vmovaps	352(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm3, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovdqa	(%rsp), %ymm0
	vmovdqa	%ymm0, 352(%rax)
.Ltmp7806:
	.loc	6 661 17
	movl	$64, 2620(%r10)
.Ltmp7807:
	.loc	6 648 21
	cmpl	$1, 280(%rsp,%rcx)
	jne	.LBB46_12
	jmp	.LBB46_32
	.loc	6 0 21 is_stmt 0
.Ltmp7808:
	.p2align	4
.LBB46_14:
	.loc	6 648 21
	cmpl	$1, 264(%rsp,%rcx)
	jne	.LBB46_15
.LBB46_22:
	.loc	6 648 26
	vmovd	268(%rsp,%rcx), %xmm0
.Ltmp7809:
	.loc	6 651 39 is_stmt 1
	vmovaps	128(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
.Ltmp7810:
	.loc	6 390 5
	vmovd	(%rsp,%rbx,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp7811:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp7812:
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
	jne	.LBB46_23
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	je	.LBB46_38
.LBB46_24:
	.loc	47 0 9
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB46_26
.Ltmp7813:
.LBB46_25:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB46_26:
.Ltmp7814:
	.loc	6 396 5 is_stmt 1
	vmovaps	128(%rax), %ymm4
	vmovaps	%ymm4, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm1
	vmovaps	%ymm1, 128(%rax)
.Ltmp7815:
	.loc	6 396 5
	vmovaps	160(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, 160(%rax)
.Ltmp7816:
	.loc	6 396 5
	vmovaps	192(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm2, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, 192(%rax)
.Ltmp7817:
	.loc	6 396 5
	vmovaps	224(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm3, (%rsp,%rbx,4)
	.loc	6 398 5
	vmovdqa	(%rsp), %ymm0
	vmovdqa	%ymm0, 224(%rax)
.Ltmp7818:
	.loc	6 661 17
	movl	$64, 2620(%r10)
.Ltmp7819:
	.loc	6 648 21
	cmpl	$1, 272(%rsp,%rcx)
	je	.LBB46_27
.LBB46_16:
	cmpl	$1, 280(%rsp,%rcx)
	jne	.LBB46_12
.LBB46_32:
	.loc	6 648 26 is_stmt 0
	vmovd	284(%rsp,%rcx), %xmm0
.Ltmp7820:
	.loc	6 651 39 is_stmt 1
	vmovaps	384(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
.Ltmp7821:
	.loc	6 390 5
	vmovd	(%rsp,%rbx,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp7822:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp7823:
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
	jne	.LBB46_33
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	je	.LBB46_42
.LBB46_34:
	.loc	47 0 9
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB46_11
	jmp	.LBB46_43
	.loc	47 0 9
.Ltmp7824:
	.p2align	4
.LBB46_18:
	.loc	47 112 9
	jne	.LBB46_19
.LBB46_36:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB46_20
	jmp	.LBB46_21
	.loc	47 0 9
.Ltmp7825:
	.p2align	4
.LBB46_28:
	.loc	47 112 9
	jne	.LBB46_29
.LBB46_40:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB46_30
	jmp	.LBB46_31
	.loc	47 0 9
.Ltmp7826:
	.p2align	4
.LBB46_23:
	.loc	47 112 9
	jne	.LBB46_24
.LBB46_38:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB46_25
	jmp	.LBB46_26
	.loc	47 0 9
.Ltmp7827:
	.p2align	4
.LBB46_33:
	.loc	47 112 9
	jne	.LBB46_34
.LBB46_42:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB46_11
.Ltmp7828:
.LBB46_43:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
	jmp	.LBB46_11
	.p2align	4
.LBB46_44:
	subq	%rdi, %rsi
	leaq	(%rdi,%rdi,4), %rax
	movq	128(%rsp), %rcx
	leaq	(%rcx,%rax,8), %r14
	leaq	(%rsi,%rsi,4), %rax
	leaq	(%r14,%rax,8), %rdx
	.loc	6 1088 25 is_stmt 1
	leaq	(%rbx,%rbx,4), %rax
	leaq	800(%rsp,%rax,8), %rsi
	movl	2596(%r10), %edi
	movq	816(%rsp,%rax,8), %r13
	movb	$1, %al
	movl	%eax, 88(%rsp)
	xorl	%r15d, %r15d
	jmp	.LBB46_45
	.loc	6 0 25 is_stmt 0
.Ltmp7829:
	.p2align	4
.LBB46_58:
.Ltmp7830:
	addq	$40, %r14
.Ltmp7831:
	.loc	15 2428 13 is_stmt 1
	incq	%r13
	movq	$-1, %rax
	cmoveq	%rax, %r13
.Ltmp7832:
	.loc	6 0 0 is_stmt 0
	movq	%r13, 16(%rsi)
.Ltmp7833:
	.loc	34 82 9 is_stmt 1
	incq	%r15
.Ltmp7834:
	.loc	38 1714 9
	cmpq	%rdx, %r14
.Ltmp7835:
	.loc	19 180 28
	je	.LBB46_10
.Ltmp7836:
.LBB46_45:
	.loc	6 613 33
	movl	32(%r14), %eax
	.loc	6 613 27 is_stmt 0
	cmpl	$1, %eax
	je	.LBB46_48
	cmpl	$2, %eax
	jne	.LBB46_58
	.loc	6 0 27
	movl	$1, %eax
	leaq	288(%rsp), %rcx
	movq	%rcx, 40(%rsp)
.Ltmp7837:
	.loc	6 621 35 is_stmt 1
	movl	16(%r14), %r12d
.Ltmp7838:
	.loc	15 3178 26
	testl	%r12d, %r12d
.Ltmp7839:
	.loc	46 459 8
	jns	.LBB46_49
	jmp	.LBB46_58
.Ltmp7840:
	.loc	46 0 8 is_stmt 0
.Ltmp7841:
	.p2align	4
.LBB46_48:
	xorl	%eax, %eax
	leaq	256(%rsp), %rcx
	movq	%rcx, 40(%rsp)
	.loc	6 621 35 is_stmt 1
	movl	16(%r14), %r12d
.Ltmp7842:
	.loc	15 3178 26
	testl	%r12d, %r12d
.Ltmp7843:
	.loc	46 459 8
	js	.LBB46_58
.Ltmp7844:
.LBB46_49:
	.loc	6 630 25
	cmpq	%rdi, %r15
	jae	.LBB46_58
	cmpl	$3, %r12d
	ja	.LBB46_58
	.loc	6 632 20
	cmpl	$1, 28(%r14)
	jne	.LBB46_58
	.loc	6 0 20 is_stmt 0
	movq	56(%rsp), %rcx
	.loc	6 633 20 is_stmt 1
	cmpq	%rcx, (%r14)
	jne	.LBB46_58
	.loc	6 0 20 is_stmt 0
	movq	56(%rsp), %rcx
	.loc	6 634 20 is_stmt 1
	cmpq	%rcx, 8(%r14)
	jne	.LBB46_58
	.loc	6 635 20
	vmovd	20(%r14), %xmm0
.Ltmp7845:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7846:
	.loc	6 635 20
	cmpl	%ecx, 24(%r14)
	jne	.LBB46_58
.Ltmp7847:
	.loc	6 0 0 is_stmt 0
	leal	(%rax,%r12,2), %eax
	movl	%eax, 152(%rsp)
.Ltmp7848:
	.loc	6 636 42 is_stmt 1
	leaq	(%r12,%r12,4), %rax
	leaq	.Lalloc_d0058eaee52f1ba8b2e5577cef0c3c7f(%rip), %rcx
	movq	%rdi, 224(%rsp)
	leaq	(%rcx,%rax,8), %rdi
	movq	%rdx, 112(%rsp)
	movq	%rsi, 320(%rsp)
	vmovdqa	%xmm0, 160(%rsp)
	.loc	6 636 20 is_stmt 0
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	160(%rsp), %xmm1
	movq	224(%rsp), %rdi
	movq	320(%rsp), %rsi
	movq	112(%rsp), %rdx
	vmovss	.LCPI46_1(%rip), %xmm6
	vmovss	.LCPI46_0(%rip), %xmm5
	movq	120(%rsp), %r11
	movq	104(%rsp), %r8
	movq	48(%rsp), %r10
	movl	152(%rsp), %ecx
	movl	%ecx, %r9d
	cmpl	480(%rsp), %ecx
	seta	%cl
	testb	%al, %al
	je	.LBB46_58
	orb	88(%rsp), %cl
	testb	$1, %cl
	je	.LBB46_58
	.loc	6 0 20
	movq	40(%rsp), %rax
.Ltmp7849:
	.loc	6 639 17 is_stmt 1
	cmpb	$0, (%rax,%r12,8)
	jne	.LBB46_58
.Ltmp7850:
	.loc	12 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
	movq	40(%rsp), %rax
.Ltmp7851:
	.loc	6 644 13
	movl	$1, (%rax,%r12,8)
	vmovss	%xmm0, 4(%rax,%r12,8)
.Ltmp7852:
	.loc	38 1714 9
	addq	$40, %r14
.Ltmp7853:
	.loc	19 180 28
	incq	%r15
	movl	$0, 88(%rsp)
	movl	%r9d, 480(%rsp)
.Ltmp7854:
	.loc	38 1714 9
	cmpq	%rdx, %r14
.Ltmp7855:
	.loc	19 180 28
	jne	.LBB46_45
	jmp	.LBB46_10
.Ltmp7856:
.LBB46_60:
	.loc	6 1076 28
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%ymm0, 288(%rdi)
	vmovups	%ymm0, 256(%rdi)
	vmovups	%ymm0, 224(%rdi)
	vmovups	%ymm0, 192(%rdi)
	vmovups	%ymm0, 160(%rdi)
	vmovups	%ymm0, 128(%rdi)
	vmovups	%ymm0, 96(%rdi)
	vmovups	%ymm0, 64(%rdi)
	vmovups	%ymm0, 32(%rdi)
	vmovups	%ymm0, (%rdi)
	movb	%al, 320(%rdi)
	jmp	.LBB46_175
.LBB46_61:
	.loc	6 0 28 is_stmt 0
	movq	448(%rsp), %rdx
.Ltmp7857:
	.loc	6 1091 30 is_stmt 1
	movl	104(%rdx), %ecx
.Ltmp7858:
	.loc	6 1092 32
	movq	(%rdx), %rax
	movq	%rax, 208(%rsp)
	movq	8(%rdx), %rax
	movq	%rax, 184(%rsp)
	.loc	6 1092 44 is_stmt 0
	movq	16(%rdx), %rax
	movq	%rax, 200(%rsp)
	movq	24(%rdx), %rax
	movq	%rax, 176(%rsp)
.Ltmp7859:
	.loc	6 682 25 is_stmt 1
	movl	2620(%r10), %eax
.Ltmp7860:
	.loc	8 1078 5
	cmpl	%eax, %ecx
	movl	%eax, %edx
	cmovbl	%ecx, %edx
.Ltmp7861:
	.loc	6 683 12
	testl	%edx, %edx
	movq	%rcx, 152(%rsp)
	movq	%rdx, 192(%rsp)
	je	.LBB46_103
	.loc	6 684 25
	leaq	(,%rdx,8), %rsi
	cmpq	184(%rsp), %rsi
.Ltmp7862:
	.loc	15 1050 16
	ja	.LBB46_190
.Ltmp7863:
	.loc	25 451 16
	cmpq	176(%rsp), %rsi
	ja	.LBB46_191
.Ltmp7864:
	.loc	25 0 16 is_stmt 0
	movl	%eax, 220(%rsp)
.Ltmp7865:
	.loc	6 730 27 is_stmt 1
	movq	1152(%r10), %rax
	movq	%rax, 80(%rsp)
	movq	1160(%r10), %r13
	.loc	6 735 27
	movq	1216(%r10), %rax
	movq	%rax, 64(%rsp)
	movq	1224(%r10), %rbx
	.loc	6 741 24
	movl	2612(%r10), %r15d
.Ltmp7866:
	.loc	21 238 16
	movl	2608(%r10), %r12d
.Ltmp7867:
	.loc	11 900 12
	movl	%r12d, %eax
	subl	2616(%r10), %eax
	movq	%rax, 736(%rsp)
	xorl	%r14d, %r14d
	vbroadcastss	.LCPI46_4(%rip), %ymm1
	vmovaps	%ymm1, 704(%rsp)
	vbroadcastsd	.LCPI46_8(%rip), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vbroadcastsd	.LCPI46_9(%rip), %ymm0
	vmovaps	%ymm0, 224(%rsp)
	movq	208(%rsp), %rdx
	movq	200(%rsp), %r8
	movq	%r13, 96(%rsp)
	movq	%rbx, 88(%rsp)
	movq	%r12, 112(%rsp)
	movl	%r15d, 56(%rsp)
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp7868:
	.loc	11 0 12 is_stmt 0
.Ltmp7869:
	.p2align	4
.LBB46_65:
	.loc	21 246 22 is_stmt 1
	leal	(%r12,%r14), %edi
	andl	%r15d, %edi
.Ltmp7870:
	.loc	21 247 77
	leaq	8(,%rdi,8), %rsi
.Ltmp7871:
	.loc	21 246 21
	shlq	$3, %rdi
.Ltmp7872:
	.loc	25 451 16
	cmpq	%r13, %rsi
	ja	.LBB46_178
.Ltmp7873:
	.loc	1 551 14
	vmovups	(%rdx), %ymm1
	movq	80(%rsp), %rax
	vmovups	%ymm1, (%rax,%rdi,4)
.Ltmp7874:
	.loc	25 451 16
	cmpq	%rbx, %rsi
	ja	.LBB46_179
.Ltmp7875:
	.loc	1 551 14
	vmovups	(%r8), %ymm1
	movq	64(%rsp), %rax
	vmovups	%ymm1, (%rax,%rdi,4)
	movq	736(%rsp), %rax
.Ltmp7876:
	.loc	21 255 21
	leal	(%rax,%r14), %r9d
	andl	%r15d, %r9d
.Ltmp7877:
	.loc	21 256 54
	leaq	8(,%r9,8), %rsi
.Ltmp7878:
	.loc	21 255 20
	shlq	$3, %r9
.Ltmp7879:
	.loc	25 438 16
	cmpq	%r13, %rsi
	ja	.LBB46_180
.Ltmp7880:
	.loc	25 438 16 is_stmt 0
	cmpq	%rbx, %rsi
	ja	.LBB46_181
.Ltmp7881:
	.loc	21 271 24 is_stmt 1
	leal	(%r12,%r14), %esi
	movl	%esi, %r11d
	subl	1184(%r10), %r11d
	andl	%r15d, %r11d
	.loc	21 271 23 is_stmt 0
	shlq	$3, %r11
.Ltmp7882:
	.loc	21 274 29 is_stmt 1
	cmpq	%r13, %r11
	jae	.LBB46_230
	.loc	21 275 29
	cmpq	%rbx, %r11
	jae	.LBB46_228
.Ltmp7883:
	.loc	21 0 0 is_stmt 0
	subl	1248(%r10), %esi
	andl	%r15d, %esi
	shlq	$3, %rsi
.Ltmp7884:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbx, %rsi
	jae	.LBB46_221
	.loc	21 277 29
	cmpq	%r13, %rsi
	jae	.LBB46_226
.Ltmp7885:
	.loc	21 271 24
	leal	(%r12,%r14), %eax
	movl	%eax, %ecx
	subl	1188(%r10), %ecx
	andl	%r15d, %ecx
	.loc	21 271 23 is_stmt 0
	leaq	1(,%rcx,8), %rdi
.Ltmp7886:
	.loc	21 274 29 is_stmt 1
	cmpq	%r13, %rdi
	jae	.LBB46_229
	.loc	21 275 29
	cmpq	%rbx, %rdi
	jae	.LBB46_219
.Ltmp7887:
	.loc	21 0 0 is_stmt 0
	subl	1252(%r10), %eax
	andl	%r15d, %eax
	leaq	1(,%rax,8), %rax
.Ltmp7888:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbx, %rax
	jae	.LBB46_198
	.loc	21 0 29 is_stmt 0
	movq	%rax, 160(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%r13, %rax
	jae	.LBB46_214
.Ltmp7889:
	.loc	21 271 24
	leal	(%r12,%r14), %eax
	movl	%eax, %ecx
	subl	1192(%r10), %ecx
	andl	%r15d, %ecx
	.loc	21 271 23 is_stmt 0
	leaq	2(,%rcx,8), %rcx
.Ltmp7890:
	.loc	21 274 29 is_stmt 1
	cmpq	%r13, %rcx
	jae	.LBB46_207
	.loc	21 275 29
	cmpq	%rbx, %rcx
	jae	.LBB46_202
.Ltmp7891:
	.loc	21 0 0 is_stmt 0
	subl	1256(%r10), %eax
	andl	%r15d, %eax
	leaq	2(,%rax,8), %rax
.Ltmp7892:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbx, %rax
	jae	.LBB46_198
	.loc	21 0 29 is_stmt 0
	movq	%rcx, 104(%rsp)
	movq	%rax, 128(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%r13, %rax
	jae	.LBB46_208
.Ltmp7893:
	.loc	21 271 24
	leal	(%r12,%r14), %eax
	movl	%eax, %ecx
	subl	1196(%r10), %ecx
	andl	%r15d, %ecx
	.loc	21 271 23 is_stmt 0
	leaq	3(,%rcx,8), %rcx
.Ltmp7894:
	.loc	21 274 29 is_stmt 1
	cmpq	%r13, %rcx
	jae	.LBB46_207
	.loc	21 275 29
	cmpq	%rbx, %rcx
	jae	.LBB46_202
.Ltmp7895:
	.loc	21 0 0 is_stmt 0
	subl	1260(%r10), %eax
	andl	%r15d, %eax
	leaq	3(,%rax,8), %r10
.Ltmp7896:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbx, %r10
	jae	.LBB46_216
	.loc	21 0 29 is_stmt 0
	movq	%rcx, 448(%rsp)
	movq	%r8, 480(%rsp)
	movq	%rdx, 120(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%r13, %r10
	jae	.LBB46_210
.Ltmp7897:
	.loc	21 271 24
	leal	(%r12,%r14), %eax
	movl	%eax, %ecx
	movq	48(%rsp), %rdx
	subl	1200(%rdx), %ecx
	andl	%r15d, %ecx
	.loc	21 271 23 is_stmt 0
	leaq	4(,%rcx,8), %r8
.Ltmp7898:
	.loc	21 274 29 is_stmt 1
	cmpq	%r13, %r8
	jae	.LBB46_204
	.loc	21 275 29
	cmpq	%rbx, %r8
	jae	.LBB46_224
.Ltmp7899:
	.loc	21 0 29 is_stmt 0
	movq	48(%rsp), %rcx
	subl	1264(%rcx), %eax
	andl	%r15d, %eax
	leaq	4(,%rax,8), %rdx
.Ltmp7900:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbx, %rdx
	jae	.LBB46_217
	.loc	21 0 29 is_stmt 0
	movq	%rdi, 512(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%r13, %rdx
	jae	.LBB46_212
.Ltmp7901:
	.loc	21 271 24
	leal	(%r12,%r14), %ecx
	movl	%ecx, %eax
	movq	48(%rsp), %rdi
	subl	1204(%rdi), %eax
	andl	%r15d, %eax
	.loc	21 271 23 is_stmt 0
	leaq	5(,%rax,8), %rax
.Ltmp7902:
	.loc	21 274 29 is_stmt 1
	cmpq	%r13, %rax
	jae	.LBB46_205
	.loc	21 0 29 is_stmt 0
	movq	%r14, 40(%rsp)
	.loc	21 275 29 is_stmt 1
	cmpq	%rbx, %rax
	jae	.LBB46_227
.Ltmp7903:
	.loc	21 0 29 is_stmt 0
	movq	48(%rsp), %rdi
	subl	1268(%rdi), %ecx
	andl	%r15d, %ecx
	leaq	5(,%rcx,8), %r14
.Ltmp7904:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbx, %r14
	jae	.LBB46_218
	.loc	21 0 29 is_stmt 0
	movq	%r10, 672(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%r13, %r14
	jae	.LBB46_213
.Ltmp7905:
	.loc	21 0 29 is_stmt 0
	movq	40(%rsp), %rcx
	.loc	21 271 24 is_stmt 1
	addl	%r12d, %ecx
	movl	%ecx, %edi
	movq	48(%rsp), %r15
	subl	1208(%r15), %edi
	movl	56(%rsp), %r10d
	andl	%r10d, %edi
	.loc	21 271 23 is_stmt 0
	leaq	6(,%rdi,8), %r15
.Ltmp7906:
	.loc	21 274 29 is_stmt 1
	cmpq	%r13, %r15
	jae	.LBB46_206
	.loc	21 275 29
	cmpq	%rbx, %r15
	jae	.LBB46_201
.Ltmp7907:
	.loc	21 0 29 is_stmt 0
	movq	48(%rsp), %rdi
	subl	1272(%rdi), %ecx
	andl	%r10d, %ecx
	leaq	6(,%rcx,8), %r12
.Ltmp7908:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbx, %r12
	jae	.LBB46_220
	.loc	21 0 29 is_stmt 0
	movq	%r15, 416(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%r13, %r12
	jae	.LBB46_215
.Ltmp7909:
	.loc	21 0 29 is_stmt 0
	movq	%r13, %rbx
	movq	112(%rsp), %rcx
	movq	40(%rsp), %rdi
	.loc	21 271 24 is_stmt 1
	addl	%ecx, %edi
	movl	%edi, %ecx
	movq	48(%rsp), %r13
	subl	1212(%r13), %ecx
	andl	%r10d, %ecx
	.loc	21 271 23 is_stmt 0
	leaq	7(,%rcx,8), %rcx
.Ltmp7910:
	.loc	21 274 29 is_stmt 1
	cmpq	%rbx, %rcx
	jae	.LBB46_209
	.loc	21 0 29 is_stmt 0
	movq	88(%rsp), %rbx
	.loc	21 275 29 is_stmt 1
	cmpq	%rbx, %rcx
	jae	.LBB46_202
.Ltmp7911:
	.loc	21 0 29 is_stmt 0
	movq	48(%rsp), %r13
	subl	1276(%r13), %edi
	andl	56(%rsp), %edi
	leaq	7(,%rdi,8), %rdi
.Ltmp7912:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbx, %rdi
	jae	.LBB46_211
	.loc	21 277 29
	cmpq	96(%rsp), %rdi
	jae	.LBB46_225
.Ltmp7913:
	.loc	21 0 29 is_stmt 0
	movq	%rcx, %rbx
	movq	64(%rsp), %rcx
.Ltmp7914:
	.loc	1 551 14 is_stmt 1
	vmovd	(%rcx,%rdx,4), %xmm1
	movq	80(%rsp), %r10
.Ltmp7915:
	.loc	1 551 14 is_stmt 0
	vmovd	(%r10,%rdx,4), %xmm4
.Ltmp7916:
	.loc	1 551 14
	vpinsrd	$1, (%rcx,%r14,4), %xmm1, %xmm1
.Ltmp7917:
	.loc	1 551 14
	vpinsrd	$1, (%r10,%r14,4), %xmm4, %xmm4
.Ltmp7918:
	.loc	1 551 14
	vpinsrd	$2, (%rcx,%r12,4), %xmm1, %xmm1
.Ltmp7919:
	.loc	1 551 14
	vpinsrd	$2, (%r10,%r12,4), %xmm4, %xmm4
.Ltmp7920:
	.loc	21 0 0
	vmovups	(%r10,%r9,4), %ymm0
	vmovaps	%ymm0, 608(%rsp)
.Ltmp7921:
	.loc	1 551 14
	vpinsrd	$3, (%rcx,%rdi,4), %xmm1, %xmm0
	vmovdqa	%xmm0, 368(%rsp)
.Ltmp7922:
	.loc	21 0 0
	vmovups	(%rcx,%r9,4), %ymm0
	vmovaps	%ymm0, 640(%rsp)
.Ltmp7923:
	.loc	1 551 14
	vpinsrd	$3, (%r10,%rdi,4), %xmm4, %xmm0
	vmovdqa	%xmm0, 384(%rsp)
.Ltmp7924:
	.loc	21 0 0
	movl	(%r10,%r8,4), %edx
.Ltmp7925:
	.loc	1 551 14
	vmovd	%edx, %xmm1
.Ltmp7926:
	.loc	21 0 0
	movl	(%r10,%rax,4), %edx
.Ltmp7927:
	.loc	1 551 14
	vpinsrd	$1, %edx, %xmm1, %xmm1
	movq	416(%rsp), %r15
.Ltmp7928:
	.loc	21 0 0
	movl	(%r10,%r15,4), %edx
.Ltmp7929:
	.loc	1 551 14
	vpinsrd	$2, %edx, %xmm1, %xmm1
	movq	%rax, %rdi
	movq	%rbx, %rax
.Ltmp7930:
	.loc	21 0 0
	movl	(%r10,%rbx,4), %edx
.Ltmp7931:
	.loc	1 551 14
	vpinsrd	$3, %edx, %xmm1, %xmm5
.Ltmp7932:
	.loc	21 0 0
	movl	(%r10,%r11,4), %edx
	movl	(%rcx,%r11,4), %r11d
.Ltmp7933:
	.loc	1 551 14
	vmovd	%edx, %xmm1
	movq	512(%rsp), %r9
.Ltmp7934:
	.loc	21 0 0
	movl	(%r10,%r9,4), %edx
.Ltmp7935:
	.loc	1 551 14
	vpinsrd	$1, %edx, %xmm1, %xmm1
	movq	104(%rsp), %rbx
.Ltmp7936:
	.loc	21 0 0
	movl	(%r10,%rbx,4), %edx
.Ltmp7937:
	.loc	1 551 14
	vpinsrd	$2, %edx, %xmm1, %xmm1
	movq	448(%rsp), %r13
.Ltmp7938:
	.loc	21 0 0
	movl	(%r10,%r13,4), %edx
.Ltmp7939:
	.loc	1 551 14
	vpinsrd	$3, %edx, %xmm1, %xmm8
.Ltmp7940:
	.loc	21 0 0
	movl	(%rcx,%rsi,4), %edx
	movl	%edx, 768(%rsp)
	movl	(%r10,%rsi,4), %edx
	movl	%edx, 576(%rsp)
	movl	(%rcx,%r9,4), %r9d
	movq	160(%rsp), %rdx
	movl	(%rcx,%rdx,4), %r14d
	movl	(%r10,%rdx,4), %edx
	movl	%edx, 160(%rsp)
	movl	(%rcx,%rbx,4), %esi
	movq	128(%rsp), %rdx
	movl	(%rcx,%rdx,4), %r12d
	movl	(%r10,%rdx,4), %edx
	movl	%edx, 128(%rsp)
	movl	(%rcx,%r13,4), %r13d
	movq	672(%rsp), %rdx
	movl	(%rcx,%rdx,4), %ebx
	movl	(%r10,%rdx,4), %edx
	movl	%edx, 104(%rsp)
	movl	(%rcx,%r8,4), %r8d
	movl	(%rcx,%rdi,4), %edx
	movl	(%rcx,%r15,4), %r15d
	movl	(%rcx,%rax,4), %ecx
	movq	48(%rsp), %r10
.Ltmp7941:
	.loc	21 325 27 is_stmt 1
	vmovaps	1280(%r10), %ymm4
.Ltmp7942:
	.loc	21 323 26
	vmovaps	1376(%r10), %ymm6
.Ltmp7943:
	.loc	21 325 27
	vmovaps	1408(%r10), %ymm1
.Ltmp7944:
	.loc	21 323 26
	vmovaps	1504(%r10), %ymm7
.Ltmp7945:
	.file	57 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs"
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm6, %ymm9
.Ltmp7946:
	.loc	57 48 14
	vaddps	1344(%r10), %ymm4, %ymm10
	vbroadcastss	.LCPI46_2(%rip), %ymm11
.Ltmp7947:
	.loc	57 871 14
	vcmpeqps	%ymm6, %ymm11, %ymm15
.Ltmp7948:
	.loc	57 585 19
	vblendvps	%ymm15, 1312(%r10), %ymm10, %ymm10
.Ltmp7949:
	.loc	21 326 13
	vmaskmovps	%ymm10, %ymm9, 1280(%r10)
	.loc	21 331 13
	vmaskmovps	%ymm2, %ymm15, 1344(%r10)
.Ltmp7950:
	.loc	1 551 14
	vmovd	%r8d, %xmm15
	vpinsrd	$1, %edx, %xmm15, %xmm15
	vbroadcastss	.LCPI46_3(%rip), %ymm3
.Ltmp7951:
	.loc	57 347 14
	vaddps	%ymm3, %ymm6, %ymm6
.Ltmp7952:
	.loc	21 332 13
	vmaskmovps	%ymm6, %ymm9, 1376(%r10)
.Ltmp7953:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm7, %ymm6
.Ltmp7954:
	.loc	57 48 14
	vaddps	1472(%r10), %ymm1, %ymm13
	vmovaps	%ymm11, %ymm0
.Ltmp7955:
	.loc	57 871 14
	vcmpeqps	%ymm7, %ymm11, %ymm14
.Ltmp7956:
	.loc	57 585 19
	vblendvps	%ymm14, 1440(%r10), %ymm13, %ymm13
.Ltmp7957:
	.loc	21 326 13
	vmaskmovps	%ymm13, %ymm6, 1408(%r10)
	.loc	21 331 13
	vmaskmovps	%ymm2, %ymm14, 1472(%r10)
.Ltmp7958:
	.loc	1 551 14
	vpinsrd	$2, %r15d, %xmm15, %xmm14
.Ltmp7959:
	.loc	57 347 14
	vaddps	%ymm3, %ymm7, %ymm7
.Ltmp7960:
	.loc	21 332 13
	vmaskmovps	%ymm7, %ymm6, 1504(%r10)
.Ltmp7961:
	.loc	57 585 19
	vblendvps	%ymm9, %ymm10, %ymm4, %ymm4
.Ltmp7962:
	.loc	21 323 26
	vmovaps	1632(%r10), %ymm7
.Ltmp7963:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm7, %ymm9
.Ltmp7964:
	.loc	21 325 27
	vmovaps	1536(%r10), %ymm10
.Ltmp7965:
	.loc	57 48 14
	vaddps	1600(%r10), %ymm10, %ymm15
.Ltmp7966:
	.loc	57 871 14
	vcmpeqps	%ymm7, %ymm11, %ymm11
.Ltmp7967:
	.loc	57 585 19
	vblendvps	%ymm11, 1568(%r10), %ymm15, %ymm15
.Ltmp7968:
	.loc	21 326 13
	vmaskmovps	%ymm15, %ymm9, 1536(%r10)
	.loc	21 331 13
	vmaskmovps	%ymm2, %ymm11, 1600(%r10)
.Ltmp7969:
	.loc	1 551 14
	vpinsrd	$3, %ecx, %xmm14, %xmm11
	vmovd	%r11d, %xmm14
	vpinsrd	$1, %r9d, %xmm14, %xmm14
	vpinsrd	$2, %esi, %xmm14, %xmm14
.Ltmp7970:
	.loc	57 585 19
	vblendvps	%ymm6, %ymm13, %ymm1, %ymm6
.Ltmp7971:
	.loc	57 347 14
	vaddps	%ymm3, %ymm7, %ymm1
.Ltmp7972:
	.loc	21 332 13
	vmaskmovps	%ymm1, %ymm9, 1632(%r10)
.Ltmp7973:
	.loc	57 585 19
	vblendvps	%ymm9, %ymm15, %ymm10, %ymm7
.Ltmp7974:
	.loc	21 325 27
	vmovaps	1664(%r10), %ymm1
.Ltmp7975:
	.loc	21 323 26
	vmovaps	1760(%r10), %ymm9
.Ltmp7976:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm9, %ymm10
.Ltmp7977:
	.loc	57 48 14
	vaddps	1728(%r10), %ymm1, %ymm13
.Ltmp7978:
	.loc	57 871 14
	vcmpeqps	%ymm0, %ymm9, %ymm15
.Ltmp7979:
	.loc	57 585 19
	vblendvps	%ymm15, 1696(%r10), %ymm13, %ymm13
.Ltmp7980:
	.loc	21 326 13
	vmaskmovps	%ymm13, %ymm10, 1664(%r10)
	.loc	21 331 13
	vmaskmovps	%ymm2, %ymm15, 1728(%r10)
.Ltmp7981:
	.loc	1 551 14
	vpinsrd	$3, %r13d, %xmm14, %xmm14
.Ltmp7982:
	.loc	57 347 14
	vaddps	%ymm3, %ymm9, %ymm9
.Ltmp7983:
	.loc	21 332 13
	vmaskmovps	%ymm9, %ymm10, 1760(%r10)
.Ltmp7984:
	.loc	57 585 19
	vblendvps	%ymm10, %ymm13, %ymm1, %ymm1
.Ltmp7985:
	.loc	21 392 65
	vmovaps	800(%r10), %ymm9
.Ltmp7986:
	.loc	21 398 49
	vmovaps	864(%r10), %ymm0
	vmovaps	%ymm0, 448(%rsp)
.Ltmp7987:
	.loc	21 343 23
	vmovaps	896(%r10), %ymm10
	.loc	21 345 9
	vmovaps	928(%r10), %ymm13
.Ltmp7988:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm13, %ymm13
.Ltmp7989:
	.loc	1 551 14
	vinserti128	$1, %xmm5, %ymm8, %ymm5
	vmovdqa	704(%rsp), %ymm0
.Ltmp7990:
	.loc	57 82 19
	vpand	%ymm0, %ymm5, %ymm5
	vbroadcastss	.LCPI46_5(%rip), %ymm15
.Ltmp7991:
	.loc	57 283 14
	vmulps	%ymm5, %ymm15, %ymm8
.Ltmp7992:
	.loc	1 551 14
	vinserti128	$1, %xmm11, %ymm14, %ymm11
.Ltmp7993:
	.loc	57 82 19
	vpand	%ymm0, %ymm11, %ymm11
.Ltmp7994:
	.loc	57 283 14
	vmulps	%ymm15, %ymm11, %ymm14
.Ltmp7995:
	.loc	57 48 14
	vaddps	%ymm14, %ymm8, %ymm8
.Ltmp7996:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm10, %ymm10
.Ltmp7997:
	.loc	57 233 14
	vmaxps	%ymm11, %ymm5, %ymm11
.Ltmp7998:
	.loc	57 585 19
	vblendvps	%ymm10, %ymm11, %ymm5, %ymm5
.Ltmp7999:
	.loc	57 585 19 is_stmt 0
	vblendvps	%ymm13, %ymm8, %ymm5, %ymm5
	vbroadcastss	.LCPI46_6(%rip), %ymm8
.Ltmp8000:
	.loc	57 233 14 is_stmt 1
	vmaxps	%ymm8, %ymm5, %ymm5
	vbroadcastss	.LCPI46_7(%rip), %ymm8
.Ltmp8001:
	.loc	57 233 14 is_stmt 0
	vmaxps	%ymm8, %ymm5, %ymm5
.Ltmp8002:
	.file	58 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs"
	.loc	58 264 24 is_stmt 1
	vandps	320(%rsp), %ymm5, %ymm8
.Ltmp8003:
	.loc	58 2315 14
	vorps	224(%rsp), %ymm8, %ymm8
.Ltmp8004:
	.loc	57 347 14
	vaddps	%ymm3, %ymm8, %ymm8
	vbroadcastss	.LCPI46_10(%rip), %ymm10
.Ltmp8005:
	.loc	57 283 14
	vmulps	%ymm10, %ymm8, %ymm10
	vbroadcastss	.LCPI46_11(%rip), %ymm11
.Ltmp8006:
	.loc	57 48 14
	vsubps	%ymm10, %ymm11, %ymm10
.Ltmp8007:
	.loc	57 283 14
	vmulps	%ymm10, %ymm8, %ymm10
	vbroadcastss	.LCPI46_12(%rip), %ymm11
.Ltmp8008:
	.loc	57 48 14
	vaddps	%ymm11, %ymm10, %ymm10
.Ltmp8009:
	.loc	57 283 14
	vmulps	%ymm10, %ymm8, %ymm10
	vbroadcastss	.LCPI46_13(%rip), %ymm11
.Ltmp8010:
	.loc	57 48 14
	vaddps	%ymm11, %ymm10, %ymm10
.Ltmp8011:
	.loc	57 283 14
	vmulps	%ymm10, %ymm8, %ymm10
	vbroadcastss	.LCPI46_14(%rip), %ymm11
.Ltmp8012:
	.loc	57 48 14
	vaddps	%ymm11, %ymm10, %ymm10
.Ltmp8013:
	.loc	57 283 14
	vmulps	%ymm10, %ymm8, %ymm10
	vbroadcastss	.LCPI46_15(%rip), %ymm11
.Ltmp8014:
	.loc	57 48 14
	vaddps	%ymm11, %ymm10, %ymm10
.Ltmp8015:
	.loc	58 3217 24
	vpsrld	$23, %ymm5, %ymm11
.Ltmp8016:
	.loc	58 2315 24
	vpbroadcastd	.LCPI46_16(%rip), %ymm5
	vpor	%ymm5, %ymm11, %ymm11
	vbroadcastss	.LCPI46_17(%rip), %ymm12
.Ltmp8017:
	.loc	57 347 14
	vaddps	%ymm12, %ymm11, %ymm11
.Ltmp8018:
	.loc	57 283 14
	vmulps	%ymm10, %ymm8, %ymm8
.Ltmp8019:
	.loc	57 48 14
	vaddps	%ymm8, %ymm11, %ymm8
	vbroadcastss	.LCPI46_18(%rip), %ymm10
.Ltmp8020:
	.loc	57 283 14
	vmulps	%ymm10, %ymm8, %ymm8
	vbroadcastss	.LCPI46_19(%rip), %ymm10
.Ltmp8021:
	.loc	57 257 14
	vminps	%ymm10, %ymm8, %ymm8
	vbroadcastss	.LCPI46_20(%rip), %ymm10
.Ltmp8022:
	.loc	57 233 14
	vmaxps	%ymm10, %ymm8, %ymm8
.Ltmp8023:
	.loc	21 362 20
	vmovaps	1792(%r10), %ymm10
.Ltmp8024:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm10, %ymm10
.Ltmp8025:
	.loc	57 871 14 is_stmt 0
	vcmpge_oqps	%ymm4, %ymm8, %ymm11
.Ltmp8026:
	.loc	57 347 14 is_stmt 1
	vsubps	%ymm1, %ymm4, %ymm1
.Ltmp8027:
	.loc	57 871 14
	vcmpge_oqps	%ymm1, %ymm8, %ymm1
.Ltmp8028:
	.loc	57 82 19
	vandnps	%ymm11, %ymm10, %ymm11
.Ltmp8029:
	.loc	57 82 19 is_stmt 0
	vandps	%ymm1, %ymm10, %ymm13
.Ltmp8030:
	.loc	57 117 19 is_stmt 1
	vorps	%ymm11, %ymm13, %ymm11
.Ltmp8031:
	.loc	21 370 47
	vmovaps	1824(%r10), %ymm13
.Ltmp8032:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm13, %ymm14
.Ltmp8033:
	.loc	57 82 19
	vandnps	%ymm14, %ymm1, %ymm1
	vandps	%ymm1, %ymm10, %ymm1
.Ltmp8034:
	.loc	57 347 14
	vaddps	%ymm3, %ymm13, %ymm10
.Ltmp8035:
	.loc	57 585 19
	vblendvps	%ymm1, %ymm10, %ymm13, %ymm10
.Ltmp8036:
	.loc	57 117 19
	vorps	%ymm1, %ymm11, %ymm1
.Ltmp8037:
	.loc	57 585 19
	vblendvps	%ymm11, 832(%r10), %ymm10, %ymm10
.Ltmp8038:
	.loc	57 585 19 is_stmt 0
	vpcmpgtd	%ymm1, %ymm2, %ymm1
	vpbroadcastd	.LCPI46_2(%rip), %ymm15
	vpand	%ymm1, %ymm15, %ymm1
.Ltmp8039:
	.loc	21 373 5 is_stmt 1
	vmovaps	%ymm10, 1824(%r10)
	.loc	21 382 5
	vmovdqa	%ymm1, 1792(%r10)
.Ltmp8040:
	.loc	21 392 36
	vmovaps	1856(%r10), %ymm10
.Ltmp8041:
	.loc	57 347 14
	vaddps	%ymm3, %ymm6, %ymm6
.Ltmp8042:
	.loc	57 347 14 is_stmt 0
	vsubps	%ymm4, %ymm8, %ymm4
.Ltmp8043:
	.loc	57 283 14 is_stmt 1
	vmulps	%ymm4, %ymm6, %ymm4
.Ltmp8044:
	.loc	57 713 19
	vbroadcastss	.LCPI46_21(%rip), %ymm6
	vxorps	%ymm6, %ymm7, %ymm7
.Ltmp8045:
	.loc	57 233 14
	vmaxps	%ymm7, %ymm4, %ymm4
.Ltmp8046:
	.loc	57 257 14
	vminps	%ymm2, %ymm4, %ymm4
.Ltmp8047:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp8048:
	.loc	57 585 19
	vpcmpgtd	%ymm1, %ymm2, %ymm1
	vpandn	%ymm4, %ymm1, %ymm1
.Ltmp8049:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm1, %ymm4
.Ltmp8050:
	.loc	57 585 19
	vblendvps	%ymm4, 768(%r10), %ymm9, %ymm4
.Ltmp8051:
	.loc	57 347 14
	vsubps	%ymm10, %ymm1, %ymm1
.Ltmp8052:
	.loc	57 283 14
	vmulps	%ymm4, %ymm1, %ymm1
.Ltmp8053:
	.loc	57 48 14
	vaddps	%ymm1, %ymm10, %ymm1
.Ltmp8054:
	.loc	57 82 19
	vandps	%ymm0, %ymm1, %ymm4
	vbroadcastss	.LCPI46_22(%rip), %ymm7
.Ltmp8055:
	.loc	57 871 14
	vcmplt_oqps	%ymm7, %ymm4, %ymm4
.Ltmp8056:
	.loc	57 82 19
	vandnps	%ymm1, %ymm4, %ymm7
.Ltmp8057:
	.loc	21 394 5
	vmovaps	%ymm7, 1856(%r10)
.Ltmp8058:
	.loc	21 323 26
	vmovaps	1984(%r10), %ymm1
.Ltmp8059:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm1, %ymm4
.Ltmp8060:
	.loc	21 325 27
	vmovaps	1888(%r10), %ymm8
.Ltmp8061:
	.loc	57 48 14
	vaddps	1952(%r10), %ymm8, %ymm9
.Ltmp8062:
	.loc	57 871 14
	vcmpeqps	%ymm1, %ymm15, %ymm10
.Ltmp8063:
	.loc	57 585 19
	vblendvps	%ymm10, 1920(%r10), %ymm9, %ymm9
.Ltmp8064:
	.loc	21 326 13
	vmaskmovps	%ymm9, %ymm4, 1888(%r10)
	.loc	21 331 13
	vmaskmovps	%ymm2, %ymm10, 1952(%r10)
.Ltmp8065:
	.loc	57 347 14
	vaddps	%ymm3, %ymm1, %ymm1
.Ltmp8066:
	.loc	21 332 13
	vmaskmovps	%ymm1, %ymm4, 1984(%r10)
.Ltmp8067:
	.loc	21 325 27
	vmovaps	2016(%r10), %ymm1
.Ltmp8068:
	.loc	21 323 26
	vmovaps	2112(%r10), %ymm10
.Ltmp8069:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm10, %ymm11
.Ltmp8070:
	.loc	57 48 14
	vaddps	2080(%r10), %ymm1, %ymm13
.Ltmp8071:
	.loc	57 871 14
	vcmpeqps	%ymm15, %ymm10, %ymm14
.Ltmp8072:
	.loc	57 585 19
	vblendvps	%ymm14, 2048(%r10), %ymm13, %ymm13
.Ltmp8073:
	.loc	21 326 13
	vmaskmovps	%ymm13, %ymm11, 2016(%r10)
	.loc	21 331 13
	vmaskmovps	%ymm2, %ymm14, 2080(%r10)
.Ltmp8074:
	.loc	57 347 14
	vaddps	%ymm3, %ymm10, %ymm10
.Ltmp8075:
	.loc	21 332 13
	vmaskmovps	%ymm10, %ymm11, 2112(%r10)
.Ltmp8076:
	.loc	57 585 19
	vblendvps	%ymm4, %ymm9, %ymm8, %ymm9
.Ltmp8077:
	.loc	21 325 27
	vmovaps	2144(%r10), %ymm4
.Ltmp8078:
	.loc	21 323 26
	vmovaps	2240(%r10), %ymm8
.Ltmp8079:
	.loc	57 871 14
	vcmpeqps	%ymm15, %ymm8, %ymm10
.Ltmp8080:
	.loc	57 48 14
	vaddps	2208(%r10), %ymm4, %ymm14
.Ltmp8081:
	.loc	57 585 19
	vblendvps	%ymm10, 2176(%r10), %ymm14, %ymm14
.Ltmp8082:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm8, %ymm12
.Ltmp8083:
	.loc	21 326 13
	vmaskmovps	%ymm14, %ymm12, 2144(%r10)
	.loc	21 331 13
	vmaskmovps	%ymm2, %ymm10, 2208(%r10)
.Ltmp8084:
	.loc	57 585 19
	vblendvps	%ymm11, %ymm13, %ymm1, %ymm1
	vmovaps	%ymm1, 512(%rsp)
.Ltmp8085:
	.loc	57 347 14
	vaddps	%ymm3, %ymm8, %ymm1
.Ltmp8086:
	.loc	21 332 13
	vmaskmovps	%ymm1, %ymm12, 2240(%r10)
.Ltmp8087:
	.loc	57 585 19
	vblendvps	%ymm12, %ymm14, %ymm4, %ymm4
.Ltmp8088:
	.loc	21 325 27
	vmovaps	2272(%r10), %ymm1
.Ltmp8089:
	.loc	21 323 26
	vmovaps	2368(%r10), %ymm11
.Ltmp8090:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm11, %ymm8
.Ltmp8091:
	.loc	57 48 14
	vaddps	2336(%r10), %ymm1, %ymm12
.Ltmp8092:
	.loc	57 871 14
	vcmpeqps	%ymm15, %ymm11, %ymm13
.Ltmp8093:
	.loc	57 585 19
	vblendvps	%ymm13, 2304(%r10), %ymm12, %ymm12
.Ltmp8094:
	.loc	21 326 13
	vmaskmovps	%ymm12, %ymm8, 2272(%r10)
	.loc	21 331 13
	vmaskmovps	%ymm2, %ymm13, 2336(%r10)
.Ltmp8095:
	.loc	1 551 14
	vmovd	768(%rsp), %xmm13
	vpinsrd	$1, %r14d, %xmm13, %xmm13
	vpinsrd	$2, %r12d, %xmm13, %xmm13
.Ltmp8096:
	.loc	57 347 14
	vaddps	%ymm3, %ymm11, %ymm11
.Ltmp8097:
	.loc	21 332 13
	vmaskmovps	%ymm11, %ymm8, 2368(%r10)
	vbroadcastss	.LCPI46_23(%rip), %ymm11
.Ltmp8098:
	.loc	57 283 14
	vmulps	%ymm7, %ymm11, %ymm11
	vbroadcastss	.LCPI46_24(%rip), %ymm14
.Ltmp8099:
	.loc	57 233 14
	vmaxps	%ymm14, %ymm11, %ymm11
	vbroadcastss	.LCPI46_25(%rip), %ymm14
.Ltmp8100:
	.loc	57 257 14
	vminps	%ymm14, %ymm11, %ymm11
.Ltmp8101:
	.loc	57 585 19
	vblendvps	%ymm8, %ymm12, %ymm1, %ymm1
.Ltmp8102:
	.loc	57 471 14
	vroundps	$9, %ymm11, %ymm8
.Ltmp8103:
	.loc	57 347 14
	vsubps	%ymm8, %ymm11, %ymm11
	vbroadcastss	.LCPI46_26(%rip), %ymm12
.Ltmp8104:
	.loc	57 283 14
	vmulps	%ymm12, %ymm11, %ymm12
	vbroadcastss	.LCPI46_27(%rip), %ymm14
.Ltmp8105:
	.loc	57 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp8106:
	.loc	57 283 14
	vmulps	%ymm12, %ymm11, %ymm12
	vbroadcastss	.LCPI46_28(%rip), %ymm14
.Ltmp8107:
	.loc	57 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp8108:
	.loc	57 283 14
	vmulps	%ymm12, %ymm11, %ymm12
	vbroadcastss	.LCPI46_29(%rip), %ymm14
.Ltmp8109:
	.loc	57 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp8110:
	.loc	57 283 14
	vmulps	%ymm12, %ymm11, %ymm12
	vbroadcastss	.LCPI46_30(%rip), %ymm14
.Ltmp8111:
	.loc	57 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp8112:
	.loc	57 283 14
	vmulps	%ymm12, %ymm11, %ymm11
.Ltmp8113:
	.loc	1 551 14
	vpinsrd	$3, %ebx, %xmm13, %xmm12
.Ltmp8114:
	.loc	57 48 14
	vaddps	%ymm15, %ymm11, %ymm11
	vbroadcastss	.LCPI46_31(%rip), %ymm13
.Ltmp8115:
	.loc	57 48 14 is_stmt 0
	vaddps	%ymm13, %ymm8, %ymm8
.Ltmp8116:
	.loc	58 2798 24 is_stmt 1
	vpslld	$23, %ymm8, %ymm8
.Ltmp8117:
	.loc	57 283 14
	vmulps	%ymm8, %ymm11, %ymm8
.Ltmp8118:
	.loc	1 551 14
	vmovd	576(%rsp), %xmm11
	vpinsrd	$1, 160(%rsp), %xmm11, %xmm11
	vpinsrd	$2, 128(%rsp), %xmm11, %xmm11
	vpinsrd	$3, 104(%rsp), %xmm11, %xmm11
.Ltmp8119:
	.loc	1 551 14 is_stmt 0
	vinserti128	$1, 368(%rsp), %ymm12, %ymm12
.Ltmp8120:
	.loc	21 343 23 is_stmt 1
	vmovaps	1088(%r10), %ymm13
	vmovaps	%ymm0, %ymm10
.Ltmp8121:
	.loc	57 82 19
	vpand	%ymm0, %ymm12, %ymm12
.Ltmp8122:
	.loc	1 551 14
	vinserti128	$1, 384(%rsp), %ymm11, %ymm11
.Ltmp8123:
	.loc	57 82 19
	vpand	%ymm0, %ymm11, %ymm11
.Ltmp8124:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm13, %ymm13
.Ltmp8125:
	.loc	57 233 14
	vmaxps	%ymm11, %ymm12, %ymm14
.Ltmp8126:
	.loc	57 585 19
	vblendvps	%ymm13, %ymm14, %ymm12, %ymm13
.Ltmp8127:
	.loc	57 871 14
	vcmpeqps	%ymm2, %ymm7, %ymm7
	vmovaps	448(%rsp), %ymm14
.Ltmp8128:
	.loc	57 871 14 is_stmt 0
	vcmpgt_oqps	%ymm2, %ymm14, %ymm14
	vbroadcastss	.LCPI46_5(%rip), %ymm0
.Ltmp8129:
	.loc	57 283 14 is_stmt 1
	vmulps	%ymm0, %ymm12, %ymm12
.Ltmp8130:
	.loc	57 283 14 is_stmt 0
	vmulps	%ymm0, %ymm11, %ymm11
.Ltmp8131:
	.loc	57 48 14 is_stmt 1
	vaddps	%ymm11, %ymm12, %ymm11
.Ltmp8132:
	.loc	21 345 9
	vmovaps	1120(%r10), %ymm12
.Ltmp8133:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm12, %ymm12
.Ltmp8134:
	.loc	57 585 19
	vblendvps	%ymm12, %ymm11, %ymm13, %ymm11
.Ltmp8135:
	.loc	57 117 19
	vorps	%ymm7, %ymm14, %ymm7
.Ltmp8136:
	.loc	57 233 14
	vbroadcastss	.LCPI46_6(%rip), %ymm0
	vmaxps	%ymm0, %ymm11, %ymm11
.Ltmp8137:
	.loc	57 233 14 is_stmt 0
	vbroadcastss	.LCPI46_7(%rip), %ymm0
	vmaxps	%ymm0, %ymm11, %ymm11
.Ltmp8138:
	.loc	58 264 24 is_stmt 1
	vandps	320(%rsp), %ymm11, %ymm12
.Ltmp8139:
	.loc	58 2315 14
	vorps	224(%rsp), %ymm12, %ymm12
.Ltmp8140:
	.loc	57 347 14
	vaddps	%ymm3, %ymm12, %ymm12
.Ltmp8141:
	.loc	57 283 14
	vbroadcastss	.LCPI46_10(%rip), %ymm0
	vmulps	%ymm0, %ymm12, %ymm13
.Ltmp8142:
	.loc	57 48 14
	vbroadcastss	.LCPI46_11(%rip), %ymm0
	vsubps	%ymm13, %ymm0, %ymm13
.Ltmp8143:
	.loc	57 283 14
	vmulps	%ymm13, %ymm12, %ymm13
.Ltmp8144:
	.loc	57 48 14
	vbroadcastss	.LCPI46_12(%rip), %ymm0
	vaddps	%ymm0, %ymm13, %ymm13
.Ltmp8145:
	.loc	57 283 14
	vmulps	%ymm13, %ymm12, %ymm13
.Ltmp8146:
	.loc	57 48 14
	vbroadcastss	.LCPI46_13(%rip), %ymm0
	vaddps	%ymm0, %ymm13, %ymm13
.Ltmp8147:
	.loc	57 283 14
	vmulps	%ymm13, %ymm12, %ymm13
.Ltmp8148:
	.loc	57 48 14
	vbroadcastss	.LCPI46_14(%rip), %ymm0
	vaddps	%ymm0, %ymm13, %ymm13
.Ltmp8149:
	.loc	57 283 14
	vmulps	%ymm13, %ymm12, %ymm13
.Ltmp8150:
	.loc	57 48 14
	vbroadcastss	.LCPI46_15(%rip), %ymm0
	vaddps	%ymm0, %ymm13, %ymm13
.Ltmp8151:
	.loc	58 3217 24
	vpsrld	$23, %ymm11, %ymm11
.Ltmp8152:
	.loc	58 2315 24
	vpor	%ymm5, %ymm11, %ymm5
.Ltmp8153:
	.loc	57 347 14
	vbroadcastss	.LCPI46_17(%rip), %ymm0
	vaddps	%ymm0, %ymm5, %ymm5
.Ltmp8154:
	.loc	57 283 14
	vmulps	%ymm13, %ymm12, %ymm11
.Ltmp8155:
	.loc	57 48 14
	vaddps	%ymm5, %ymm11, %ymm5
.Ltmp8156:
	.loc	57 283 14
	vbroadcastss	.LCPI46_18(%rip), %ymm0
	vmulps	%ymm0, %ymm5, %ymm5
.Ltmp8157:
	.loc	57 257 14
	vbroadcastss	.LCPI46_19(%rip), %ymm0
	vminps	%ymm0, %ymm5, %ymm5
.Ltmp8158:
	.loc	57 233 14
	vbroadcastss	.LCPI46_20(%rip), %ymm0
	vmaxps	%ymm0, %ymm5, %ymm5
.Ltmp8159:
	.loc	21 362 20
	vmovaps	2400(%r10), %ymm11
.Ltmp8160:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm11, %ymm11
.Ltmp8161:
	.loc	57 871 14 is_stmt 0
	vcmpge_oqps	%ymm9, %ymm5, %ymm12
.Ltmp8162:
	.loc	57 347 14 is_stmt 1
	vsubps	%ymm1, %ymm9, %ymm1
.Ltmp8163:
	.loc	57 871 14
	vcmpge_oqps	%ymm1, %ymm5, %ymm1
.Ltmp8164:
	.loc	57 82 19
	vandnps	%ymm12, %ymm11, %ymm12
.Ltmp8165:
	.loc	57 82 19 is_stmt 0
	vandps	%ymm1, %ymm11, %ymm13
.Ltmp8166:
	.loc	57 117 19 is_stmt 1
	vorps	%ymm12, %ymm13, %ymm12
.Ltmp8167:
	.loc	21 370 47
	vmovaps	2432(%r10), %ymm13
.Ltmp8168:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm13, %ymm14
.Ltmp8169:
	.loc	57 82 19
	vandnps	%ymm14, %ymm1, %ymm1
	vandps	%ymm1, %ymm11, %ymm1
.Ltmp8170:
	.loc	57 347 14
	vaddps	%ymm3, %ymm13, %ymm11
.Ltmp8171:
	.loc	57 585 19
	vblendvps	%ymm1, %ymm11, %ymm13, %ymm11
.Ltmp8172:
	.loc	21 392 65
	vmovaps	992(%r10), %ymm13
.Ltmp8173:
	.loc	57 117 19
	vorps	%ymm1, %ymm12, %ymm14
.Ltmp8174:
	.loc	57 585 19
	vblendvps	%ymm12, 1024(%r10), %ymm11, %ymm11
.Ltmp8175:
	.loc	21 398 49
	vmovaps	1056(%r10), %ymm1
.Ltmp8176:
	.loc	21 373 5
	vmovaps	%ymm11, 2432(%r10)
.Ltmp8177:
	.loc	57 347 14
	vsubps	%ymm9, %ymm5, %ymm5
.Ltmp8178:
	.loc	57 347 14 is_stmt 0
	vaddps	512(%rsp), %ymm3, %ymm9
.Ltmp8179:
	.loc	57 283 14 is_stmt 1
	vmulps	%ymm5, %ymm9, %ymm5
.Ltmp8180:
	.loc	57 713 19
	vxorps	%ymm6, %ymm4, %ymm4
.Ltmp8181:
	.loc	57 585 19
	vpcmpgtd	%ymm14, %ymm2, %ymm6
	vpand	%ymm6, %ymm15, %ymm6
.Ltmp8182:
	.loc	21 382 5
	vmovdqa	%ymm6, 2400(%r10)
.Ltmp8183:
	.loc	57 233 14
	vmaxps	%ymm4, %ymm5, %ymm4
.Ltmp8184:
	.loc	21 392 36
	vmovaps	2464(%r10), %ymm5
.Ltmp8185:
	.loc	57 257 14
	vminps	%ymm2, %ymm4, %ymm4
.Ltmp8186:
	.loc	57 871 14
	vcmpgt_oqps	%ymm2, %ymm6, %ymm6
.Ltmp8187:
	.loc	57 585 19
	vpcmpgtd	%ymm6, %ymm2, %ymm6
	vpandn	%ymm4, %ymm6, %ymm4
.Ltmp8188:
	.loc	57 871 14
	vcmpgt_oqps	%ymm5, %ymm4, %ymm6
.Ltmp8189:
	.loc	57 585 19
	vblendvps	%ymm6, 960(%r10), %ymm13, %ymm6
	vmovaps	608(%rsp), %ymm3
.Ltmp8190:
	.loc	57 283 14
	vmulps	%ymm3, %ymm8, %ymm8
.Ltmp8191:
	.loc	57 585 19
	vblendvps	%ymm7, %ymm3, %ymm8, %ymm7
.Ltmp8192:
	.loc	57 347 14
	vsubps	%ymm5, %ymm4, %ymm4
.Ltmp8193:
	.loc	57 283 14
	vmulps	%ymm6, %ymm4, %ymm4
.Ltmp8194:
	.loc	57 48 14
	vaddps	%ymm4, %ymm5, %ymm4
.Ltmp8195:
	.loc	57 82 19
	vandps	%ymm4, %ymm10, %ymm5
.Ltmp8196:
	.loc	57 871 14
	vbroadcastss	.LCPI46_22(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm5, %ymm5
.Ltmp8197:
	.loc	57 82 19
	vandnps	%ymm4, %ymm5, %ymm4
.Ltmp8198:
	.loc	57 283 14
	vbroadcastss	.LCPI46_23(%rip), %ymm0
	vmulps	%ymm0, %ymm4, %ymm5
.Ltmp8199:
	.loc	57 233 14
	vbroadcastss	.LCPI46_24(%rip), %ymm0
	vmaxps	%ymm0, %ymm5, %ymm5
.Ltmp8200:
	.loc	57 257 14
	vbroadcastss	.LCPI46_25(%rip), %ymm0
	vminps	%ymm0, %ymm5, %ymm5
.Ltmp8201:
	.loc	57 471 14
	vroundps	$9, %ymm5, %ymm6
.Ltmp8202:
	.loc	57 347 14
	vsubps	%ymm6, %ymm5, %ymm5
.Ltmp8203:
	.loc	57 283 14
	vbroadcastss	.LCPI46_26(%rip), %ymm0
	vmulps	%ymm0, %ymm5, %ymm8
.Ltmp8204:
	.loc	57 48 14
	vbroadcastss	.LCPI46_27(%rip), %ymm0
	vaddps	%ymm0, %ymm8, %ymm8
.Ltmp8205:
	.loc	57 283 14
	vmulps	%ymm5, %ymm8, %ymm8
.Ltmp8206:
	.loc	57 48 14
	vbroadcastss	.LCPI46_28(%rip), %ymm0
	vaddps	%ymm0, %ymm8, %ymm8
.Ltmp8207:
	.loc	57 283 14
	vmulps	%ymm5, %ymm8, %ymm8
.Ltmp8208:
	.loc	57 48 14
	vbroadcastss	.LCPI46_29(%rip), %ymm0
	vaddps	%ymm0, %ymm8, %ymm8
.Ltmp8209:
	.loc	57 283 14
	vmulps	%ymm5, %ymm8, %ymm8
.Ltmp8210:
	.loc	57 48 14
	vbroadcastss	.LCPI46_30(%rip), %ymm0
	vaddps	%ymm0, %ymm8, %ymm8
.Ltmp8211:
	.loc	57 283 14
	vmulps	%ymm5, %ymm8, %ymm5
.Ltmp8212:
	.loc	57 48 14
	vaddps	%ymm5, %ymm15, %ymm5
.Ltmp8213:
	.loc	57 48 14 is_stmt 0
	vbroadcastss	.LCPI46_31(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp8214:
	.loc	58 2798 24 is_stmt 1
	vpslld	$23, %ymm6, %ymm6
.Ltmp8215:
	.loc	57 283 14
	vmulps	%ymm6, %ymm5, %ymm5
.Ltmp8216:
	.loc	57 871 14
	vcmpeqps	%ymm2, %ymm4, %ymm6
.Ltmp8217:
	.loc	57 871 14 is_stmt 0
	vcmpgt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp8218:
	.loc	57 117 19 is_stmt 1
	vorps	%ymm6, %ymm1, %ymm1
	vmovaps	640(%rsp), %ymm0
.Ltmp8219:
	.loc	57 283 14
	vmulps	%ymm5, %ymm0, %ymm5
.Ltmp8220:
	.loc	57 585 19
	vblendvps	%ymm1, %ymm0, %ymm5, %ymm1
.Ltmp8221:
	.loc	21 394 5
	vmovaps	%ymm4, 2464(%r10)
	movq	120(%rsp), %rdx
.Ltmp8222:
	.loc	1 551 14
	vmovups	%ymm7, (%rdx)
	movq	480(%rsp), %r8
.Ltmp8223:
	.loc	1 551 14 is_stmt 0
	vmovups	%ymm1, (%r8)
	movq	40(%rsp), %r14
.Ltmp8224:
	.loc	21 0 0
	incq	%r14
.Ltmp8225:
	.loc	8 1916 50 is_stmt 1
	addq	$32, %r8
	addq	$32, %rdx
	cmpq	%r14, 192(%rsp)
	movq	152(%rsp), %rcx
	movq	96(%rsp), %r13
	movq	88(%rsp), %rbx
	movq	112(%rsp), %r12
	movl	56(%rsp), %r15d
.Ltmp8226:
	.loc	11 900 12
	jne	.LBB46_65
.Ltmp8227:
	.loc	11 0 12 is_stmt 0
	movq	192(%rsp), %rdx
.Ltmp8228:
	.loc	15 2584 13 is_stmt 1
	leal	(%rdx,%r12), %eax
.Ltmp8229:
	.loc	21 297 5
	movl	%eax, 2608(%r10)
	movl	220(%rsp), %eax
.Ltmp8230:
.LBB46_103:
	.loc	6 688 12
	cmpl	%eax, %ecx
	jbe	.LBB46_147
	.loc	6 0 12 is_stmt 0
	movq	192(%rsp), %rax
	.loc	6 689 25 is_stmt 1
	leaq	(,%rax,8), %rdi
	movq	184(%rsp), %rsi
.Ltmp8231:
	.loc	25 580 12
	subq	%rdi, %rsi
	jb	.LBB46_192
.Ltmp8232:
	.loc	25 0 12 is_stmt 0
	movq	176(%rsp), %r15
.Ltmp8233:
	.loc	25 580 12
	subq	%rdi, %r15
	jb	.LBB46_193
.Ltmp8234:
	.loc	25 0 12
	movq	208(%rsp), %rax
.Ltmp8235:
	.loc	25 101 24 is_stmt 1
	leaq	(%rax,%rdi,4), %rax
	movq	%rax, 672(%rsp)
	movq	200(%rsp), %rax
.Ltmp8236:
	.loc	25 101 24 is_stmt 0
	leaq	(%rax,%rdi,4), %rax
	movq	%rax, 640(%rsp)
.Ltmp8237:
	.loc	6 695 17 is_stmt 1
	subq	192(%rsp), %rcx
	movq	%rcx, 448(%rsp)
.Ltmp8238:
	.loc	6 730 27
	movq	1152(%r10), %r9
	movq	1160(%r10), %rdx
	.loc	6 735 27
	movq	1216(%r10), %r14
	movq	1224(%r10), %r13
	.loc	6 741 24
	movl	2612(%r10), %eax
	movl	%eax, 40(%rsp)
.Ltmp8239:
	.loc	21 238 16
	movl	2608(%r10), %ebx
.Ltmp8240:
	.loc	11 900 12
	movq	%r15, %rax
	shrq	$3, %rax
	movq	%rax, 416(%rsp)
	movq	%rsi, 704(%rsp)
	movl	%ebx, %eax
	subl	2616(%r10), %eax
	movq	%rax, 608(%rsp)
	shrq	$3, %rsi
	movq	%rsi, 512(%rsp)
	xorl	%r8d, %r8d
	vbroadcastss	.LCPI46_4(%rip), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	vbroadcastsd	.LCPI46_8(%rip), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	vbroadcastsd	.LCPI46_9(%rip), %ymm0
	vmovaps	%ymm0, 736(%rsp)
	xorl	%r12d, %r12d
	movq	%r13, 80(%rsp)
	movq	%rdx, 64(%rsp)
	movq	%r15, 384(%rsp)
	movq	%rbx, 96(%rsp)
	movq	%r14, 368(%rsp)
	vxorps	%xmm10, %xmm10, %xmm10
	vpbroadcastd	.LCPI46_16(%rip), %ymm15
.Ltmp8241:
	.loc	11 0 12 is_stmt 0
.Ltmp8242:
	.p2align	4
.LBB46_107:
	.loc	15 1050 16 is_stmt 1
	cmpq	%r12, 512(%rsp)
	je	.LBB46_177
.Ltmp8243:
	.loc	21 0 0 is_stmt 0
	leal	(%rbx,%r12), %r11d
	andl	40(%rsp), %r11d
	shlq	$3, %r11
.Ltmp8244:
	.loc	21 247 77 is_stmt 1
	leaq	8(%r11), %rsi
.Ltmp8245:
	.loc	25 451 16
	cmpq	%rdx, %rsi
	ja	.LBB46_184
.Ltmp8246:
	.loc	25 0 16 is_stmt 0
	movq	672(%rsp), %rax
	.loc	21 247 0 is_stmt 1
	leaq	(%rax,%r8,4), %rcx
.Ltmp8247:
	.loc	1 551 14
	vmovups	(%rcx), %ymm2
	vmovups	%ymm2, (%r9,%r11,4)
.Ltmp8248:
	.loc	25 438 16
	cmpq	%r12, 416(%rsp)
	je	.LBB46_185
.Ltmp8249:
	.loc	25 451 16
	cmpq	%r13, %rsi
	ja	.LBB46_186
.Ltmp8250:
	.loc	25 0 16 is_stmt 0
	movq	640(%rsp), %rax
	.loc	21 248 0 is_stmt 1
	leaq	(%rax,%r8,4), %rdi
.Ltmp8251:
	.loc	1 551 14
	vmovups	(%rdi), %ymm2
	vmovups	%ymm2, (%r14,%r11,4)
	movq	608(%rsp), %rax
.Ltmp8252:
	.loc	21 255 21
	leal	(%rax,%r12), %r11d
	andl	40(%rsp), %r11d
.Ltmp8253:
	.loc	21 256 54
	leaq	8(,%r11,8), %rsi
.Ltmp8254:
	.loc	21 255 20
	shlq	$3, %r11
.Ltmp8255:
	.loc	25 438 16
	cmpq	%rdx, %rsi
	ja	.LBB46_187
.Ltmp8256:
	.loc	25 438 16 is_stmt 0
	cmpq	%r13, %rsi
	ja	.LBB46_188
.Ltmp8257:
	.loc	25 0 16
	movq	%rdi, 88(%rsp)
.Ltmp8258:
	.loc	21 271 24 is_stmt 1
	leal	(%rbx,%r12), %esi
	movl	%esi, %edi
	subl	1184(%r10), %edi
	andl	40(%rsp), %edi
	.loc	21 271 23 is_stmt 0
	shlq	$3, %rdi
.Ltmp8259:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB46_248
	.loc	21 275 29
	cmpq	%r13, %rdi
	jae	.LBB46_245
.Ltmp8260:
	.loc	21 0 0 is_stmt 0
	subl	1248(%r10), %esi
	andl	40(%rsp), %esi
	shlq	$3, %rsi
.Ltmp8261:
	.loc	21 276 29 is_stmt 1
	cmpq	%r13, %rsi
	jae	.LBB46_195
	.loc	21 0 29 is_stmt 0
	movq	%rcx, 112(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB46_250
.Ltmp8262:
	.loc	21 271 24
	leal	(%rbx,%r12), %eax
	movl	%eax, %ecx
	subl	1188(%r10), %ecx
	andl	40(%rsp), %ecx
	.loc	21 271 23 is_stmt 0
	leaq	1(,%rcx,8), %rcx
.Ltmp8263:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_196
	.loc	21 275 29
	cmpq	%r13, %rcx
	jae	.LBB46_197
.Ltmp8264:
	.loc	21 0 0 is_stmt 0
	subl	1252(%r10), %eax
	andl	40(%rsp), %eax
	leaq	1(,%rax,8), %rax
.Ltmp8265:
	.loc	21 276 29 is_stmt 1
	cmpq	%r13, %rax
	jae	.LBB46_194
	.loc	21 0 29 is_stmt 0
	movq	%rcx, 224(%rsp)
	movq	%rax, 320(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB46_246
.Ltmp8266:
	.loc	21 271 24
	leal	(%rbx,%r12), %eax
	movl	%eax, %ecx
	subl	1192(%r10), %ecx
	andl	40(%rsp), %ecx
	.loc	21 271 23 is_stmt 0
	leaq	2(,%rcx,8), %rcx
.Ltmp8267:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_196
	.loc	21 275 29
	cmpq	%r13, %rcx
	jae	.LBB46_197
.Ltmp8268:
	.loc	21 0 0 is_stmt 0
	subl	1256(%r10), %eax
	andl	40(%rsp), %eax
	leaq	2(,%rax,8), %rax
.Ltmp8269:
	.loc	21 276 29 is_stmt 1
	cmpq	%r13, %rax
	jae	.LBB46_194
	.loc	21 0 29 is_stmt 0
	movq	%rcx, 128(%rsp)
	movq	%rax, 160(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB46_239
.Ltmp8270:
	.loc	21 0 29 is_stmt 0
	movq	%r12, 56(%rsp)
	.loc	21 271 24 is_stmt 1
	leal	(%rbx,%r12), %eax
	movl	%eax, %ecx
	subl	1196(%r10), %ecx
	andl	40(%rsp), %ecx
	.loc	21 271 23 is_stmt 0
	leaq	3(,%rcx,8), %r12
.Ltmp8271:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %r12
	jae	.LBB46_232
	.loc	21 275 29
	cmpq	%r13, %r12
	jae	.LBB46_242
.Ltmp8272:
	.loc	21 0 0 is_stmt 0
	subl	1260(%r10), %eax
	andl	40(%rsp), %eax
	leaq	3(,%rax,8), %rax
.Ltmp8273:
	.loc	21 276 29 is_stmt 1
	cmpq	%r13, %rax
	jae	.LBB46_194
	.loc	21 0 29 is_stmt 0
	movq	%r8, 120(%rsp)
	movq	%rax, 104(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB46_235
.Ltmp8274:
	.loc	21 0 29 is_stmt 0
	movq	56(%rsp), %rax
	.loc	21 271 24 is_stmt 1
	addl	%ebx, %eax
	movl	%eax, %ecx
	subl	1200(%r10), %ecx
	andl	40(%rsp), %ecx
	.loc	21 271 23 is_stmt 0
	leaq	4(,%rcx,8), %r8
.Ltmp8275:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %r8
	jae	.LBB46_247
	.loc	21 275 29
	cmpq	%r13, %r8
	jae	.LBB46_240
.Ltmp8276:
	.loc	21 0 0 is_stmt 0
	subl	1264(%r10), %eax
	andl	40(%rsp), %eax
	leaq	4(,%rax,8), %rax
.Ltmp8277:
	.loc	21 276 29 is_stmt 1
	cmpq	%r13, %rax
	jae	.LBB46_194
	.loc	21 0 29 is_stmt 0
	movq	%rax, 480(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB46_234
.Ltmp8278:
	.loc	21 0 29 is_stmt 0
	movq	56(%rsp), %rax
	.loc	21 271 24 is_stmt 1
	addl	%ebx, %eax
	movl	%eax, %ecx
	subl	1204(%r10), %ecx
	andl	40(%rsp), %ecx
	.loc	21 271 23 is_stmt 0
	leaq	5(,%rcx,8), %r10
.Ltmp8279:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %r10
	jae	.LBB46_243
	.loc	21 275 29
	cmpq	%r13, %r10
	jae	.LBB46_237
.Ltmp8280:
	.loc	21 0 29 is_stmt 0
	movq	48(%rsp), %rcx
	subl	1268(%rcx), %eax
	andl	40(%rsp), %eax
	leaq	5(,%rax,8), %rax
.Ltmp8281:
	.loc	21 276 29 is_stmt 1
	cmpq	%r13, %rax
	jae	.LBB46_194
	.loc	21 277 29
	cmpq	%rdx, %rax
	jae	.LBB46_231
.Ltmp8282:
	.loc	21 0 29 is_stmt 0
	movq	%r9, %r14
	movq	56(%rsp), %rcx
	.loc	21 271 24 is_stmt 1
	addl	%ecx, %ebx
	movl	%ebx, %ecx
	movq	48(%rsp), %r9
	subl	1208(%r9), %ecx
	andl	40(%rsp), %ecx
	.loc	21 271 23 is_stmt 0
	leaq	6(,%rcx,8), %rcx
.Ltmp8283:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_196
	.loc	21 275 29
	cmpq	%r13, %rcx
	jae	.LBB46_197
.Ltmp8284:
	.loc	21 0 29 is_stmt 0
	movq	48(%rsp), %r15
	subl	1272(%r15), %ebx
	andl	40(%rsp), %ebx
	leaq	6(,%rbx,8), %r15
.Ltmp8285:
	.loc	21 276 29 is_stmt 1
	cmpq	%r13, %r15
	jae	.LBB46_238
	.loc	21 277 29
	cmpq	%rdx, %r15
	movq	96(%rsp), %r9
	jae	.LBB46_249
.Ltmp8286:
	.loc	21 0 29 is_stmt 0
	movq	56(%rsp), %rbx
	.loc	21 271 24 is_stmt 1
	leal	(%r9,%rbx), %r13d
	movl	%r13d, %ebx
	movq	48(%rsp), %r9
	subl	1212(%r9), %ebx
	andl	40(%rsp), %ebx
	.loc	21 271 23 is_stmt 0
	leaq	7(,%rbx,8), %rbx
.Ltmp8287:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %rbx
	jae	.LBB46_241
	.loc	21 0 29 is_stmt 0
	movq	80(%rsp), %rdx
	.loc	21 275 29 is_stmt 1
	cmpq	%rdx, %rbx
	jae	.LBB46_244
.Ltmp8288:
	.loc	21 0 29 is_stmt 0
	movq	48(%rsp), %r9
	subl	1276(%r9), %r13d
	andl	40(%rsp), %r13d
	leaq	7(,%r13,8), %r13
.Ltmp8289:
	.loc	21 276 29 is_stmt 1
	cmpq	%rdx, %r13
	jae	.LBB46_233
	.loc	21 277 29
	cmpq	64(%rsp), %r13
	jae	.LBB46_236
.Ltmp8290:
	.loc	21 0 29 is_stmt 0
	movq	%r14, %r9
.Ltmp8291:
	.loc	1 551 14 is_stmt 1
	vmovd	(%r14,%r8,4), %xmm2
	movq	368(%rsp), %r14
.Ltmp8292:
	.loc	1 551 14 is_stmt 0
	vmovd	(%r14,%r8,4), %xmm3
.Ltmp8293:
	.loc	1 551 14
	vpinsrd	$1, (%r9,%r10,4), %xmm2, %xmm2
.Ltmp8294:
	.loc	1 551 14
	vpinsrd	$1, (%r14,%r10,4), %xmm3, %xmm3
.Ltmp8295:
	.loc	1 551 14
	vpinsrd	$2, (%r9,%rcx,4), %xmm2, %xmm2
.Ltmp8296:
	.loc	1 551 14
	vpinsrd	$2, (%r14,%rcx,4), %xmm3, %xmm3
.Ltmp8297:
	.loc	1 551 14
	vpinsrd	$3, (%r9,%rbx,4), %xmm2, %xmm12
.Ltmp8298:
	.loc	1 551 14
	vpinsrd	$3, (%r14,%rbx,4), %xmm3, %xmm11
.Ltmp8299:
	.loc	1 551 14
	vmovd	(%r9,%rdi,4), %xmm2
.Ltmp8300:
	.loc	1 551 14
	vmovd	(%r14,%rdi,4), %xmm3
	movq	224(%rsp), %rcx
.Ltmp8301:
	.loc	1 551 14
	vpinsrd	$1, (%r9,%rcx,4), %xmm2, %xmm2
.Ltmp8302:
	.loc	1 551 14
	vpinsrd	$1, (%r14,%rcx,4), %xmm3, %xmm3
	movq	128(%rsp), %rcx
.Ltmp8303:
	.loc	1 551 14
	vpinsrd	$2, (%r9,%rcx,4), %xmm2, %xmm2
.Ltmp8304:
	.loc	1 551 14
	vpinsrd	$2, (%r14,%rcx,4), %xmm3, %xmm3
.Ltmp8305:
	.loc	1 551 14
	vpinsrd	$3, (%r9,%r12,4), %xmm2, %xmm14
.Ltmp8306:
	.loc	1 551 14
	vpinsrd	$3, (%r14,%r12,4), %xmm3, %xmm13
	movq	480(%rsp), %rcx
.Ltmp8307:
	.loc	1 551 14
	vmovd	(%r14,%rcx,4), %xmm2
.Ltmp8308:
	.loc	1 551 14
	vmovd	(%r9,%rcx,4), %xmm3
.Ltmp8309:
	.loc	1 551 14
	vpinsrd	$1, (%r14,%rax,4), %xmm2, %xmm2
.Ltmp8310:
	.loc	1 551 14
	vpinsrd	$1, (%r9,%rax,4), %xmm3, %xmm3
.Ltmp8311:
	.loc	1 551 14
	vpinsrd	$2, (%r14,%r15,4), %xmm2, %xmm2
.Ltmp8312:
	.loc	1 551 14
	vpinsrd	$2, (%r9,%r15,4), %xmm3, %xmm3
.Ltmp8313:
	.loc	1 551 14
	vpinsrd	$3, (%r14,%r13,4), %xmm2, %xmm0
	vmovdqa	%xmm0, 128(%rsp)
.Ltmp8314:
	.loc	1 551 14
	vpinsrd	$3, (%r9,%r13,4), %xmm3, %xmm0
	vmovdqa	%xmm0, 224(%rsp)
.Ltmp8315:
	.loc	21 0 0
	movl	(%r14,%rsi,4), %eax
.Ltmp8316:
	.loc	1 551 14
	vmovd	%eax, %xmm2
	movq	320(%rsp), %rcx
.Ltmp8317:
	.loc	21 0 0
	movl	(%r14,%rcx,4), %eax
.Ltmp8318:
	.loc	1 551 14
	vpinsrd	$1, %eax, %xmm2, %xmm2
	movq	160(%rsp), %rdx
.Ltmp8319:
	.loc	21 0 0
	movl	(%r14,%rdx,4), %eax
.Ltmp8320:
	.loc	1 551 14
	vpinsrd	$2, %eax, %xmm2, %xmm2
	movq	104(%rsp), %rdi
.Ltmp8321:
	.loc	21 0 0
	movl	(%r14,%rdi,4), %eax
.Ltmp8322:
	.loc	1 551 14
	vpinsrd	$3, %eax, %xmm2, %xmm0
	vmovdqa	%ymm0, 480(%rsp)
	movq	48(%rsp), %r10
.Ltmp8323:
	.loc	21 343 23 is_stmt 1
	vmovaps	896(%r10), %ymm8
	.loc	21 345 9
	vmovaps	928(%r10), %ymm1
.Ltmp8324:
	.loc	21 335 21
	vmovaps	1280(%r10), %ymm3
.Ltmp8325:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm1, %ymm1
.Ltmp8326:
	.loc	1 551 14
	vinserti128	$1, %xmm12, %ymm14, %ymm12
	vmovdqa	576(%rsp), %ymm7
.Ltmp8327:
	.loc	57 82 19
	vpand	%ymm7, %ymm12, %ymm12
	vbroadcastss	.LCPI46_5(%rip), %ymm2
.Ltmp8328:
	.loc	57 283 14
	vmulps	%ymm2, %ymm12, %ymm14
.Ltmp8329:
	.loc	1 551 14
	vinserti128	$1, %xmm11, %ymm13, %ymm11
.Ltmp8330:
	.loc	57 82 19
	vpand	%ymm7, %ymm11, %ymm11
.Ltmp8331:
	.loc	57 283 14
	vmulps	%ymm2, %ymm11, %ymm13
.Ltmp8332:
	.loc	57 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8333:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm8, %ymm8
.Ltmp8334:
	.loc	57 233 14
	vmaxps	%ymm11, %ymm12, %ymm11
.Ltmp8335:
	.loc	57 585 19
	vblendvps	%ymm8, %ymm11, %ymm12, %ymm8
.Ltmp8336:
	.loc	57 585 19 is_stmt 0
	vblendvps	%ymm1, %ymm13, %ymm8, %ymm1
	vbroadcastss	.LCPI46_6(%rip), %ymm6
.Ltmp8337:
	.loc	57 233 14 is_stmt 1
	vmaxps	%ymm6, %ymm1, %ymm1
	vbroadcastss	.LCPI46_7(%rip), %ymm0
.Ltmp8338:
	.loc	57 233 14 is_stmt 0
	vmaxps	%ymm0, %ymm1, %ymm1
	vmovaps	768(%rsp), %ymm12
.Ltmp8339:
	.loc	58 264 24 is_stmt 1
	vandps	%ymm1, %ymm12, %ymm8
	vmovaps	736(%rsp), %ymm6
.Ltmp8340:
	.loc	58 2315 14
	vorps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI46_3(%rip), %ymm0
.Ltmp8341:
	.loc	57 347 14
	vaddps	%ymm0, %ymm8, %ymm8
	vbroadcastss	.LCPI46_10(%rip), %ymm4
.Ltmp8342:
	.loc	57 283 14
	vmulps	%ymm4, %ymm8, %ymm11
	vbroadcastss	.LCPI46_11(%rip), %ymm4
.Ltmp8343:
	.loc	57 48 14
	vsubps	%ymm11, %ymm4, %ymm11
.Ltmp8344:
	.loc	57 283 14
	vmulps	%ymm11, %ymm8, %ymm11
	vbroadcastss	.LCPI46_12(%rip), %ymm4
.Ltmp8345:
	.loc	57 48 14
	vaddps	%ymm4, %ymm11, %ymm11
.Ltmp8346:
	.loc	57 283 14
	vmulps	%ymm11, %ymm8, %ymm11
	vbroadcastss	.LCPI46_13(%rip), %ymm4
.Ltmp8347:
	.loc	57 48 14
	vaddps	%ymm4, %ymm11, %ymm11
.Ltmp8348:
	.loc	57 283 14
	vmulps	%ymm11, %ymm8, %ymm11
	vbroadcastss	.LCPI46_14(%rip), %ymm4
.Ltmp8349:
	.loc	57 48 14
	vaddps	%ymm4, %ymm11, %ymm11
.Ltmp8350:
	.loc	57 283 14
	vmulps	%ymm11, %ymm8, %ymm11
	vbroadcastss	.LCPI46_15(%rip), %ymm4
.Ltmp8351:
	.loc	57 48 14
	vaddps	%ymm4, %ymm11, %ymm11
.Ltmp8352:
	.loc	58 3217 24
	vpsrld	$23, %ymm1, %ymm1
.Ltmp8353:
	.loc	58 2315 24
	vpor	%ymm1, %ymm15, %ymm1
	vbroadcastss	.LCPI46_17(%rip), %ymm9
.Ltmp8354:
	.loc	57 347 14
	vaddps	%ymm1, %ymm9, %ymm1
.Ltmp8355:
	.loc	57 283 14
	vmulps	%ymm11, %ymm8, %ymm8
.Ltmp8356:
	.loc	57 48 14
	vaddps	%ymm1, %ymm8, %ymm1
	vbroadcastss	.LCPI46_18(%rip), %ymm4
.Ltmp8357:
	.loc	57 283 14
	vmulps	%ymm4, %ymm1, %ymm1
	vbroadcastss	.LCPI46_19(%rip), %ymm4
.Ltmp8358:
	.loc	57 257 14
	vminps	%ymm4, %ymm1, %ymm1
	vbroadcastss	.LCPI46_20(%rip), %ymm4
.Ltmp8359:
	.loc	57 233 14
	vmaxps	%ymm4, %ymm1, %ymm1
.Ltmp8360:
	.loc	21 362 20
	vmovaps	1792(%r10), %ymm8
.Ltmp8361:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm8, %ymm8
.Ltmp8362:
	.loc	57 871 14 is_stmt 0
	vcmpge_oqps	%ymm3, %ymm1, %ymm11
.Ltmp8363:
	.loc	57 347 14 is_stmt 1
	vsubps	1664(%r10), %ymm3, %ymm13
.Ltmp8364:
	.loc	57 871 14
	vcmpge_oqps	%ymm13, %ymm1, %ymm13
.Ltmp8365:
	.loc	57 82 19
	vandnps	%ymm11, %ymm8, %ymm11
.Ltmp8366:
	.loc	57 82 19 is_stmt 0
	vandps	%ymm8, %ymm13, %ymm14
.Ltmp8367:
	.loc	57 117 19 is_stmt 1
	vorps	%ymm11, %ymm14, %ymm14
.Ltmp8368:
	.loc	21 370 47
	vmovaps	1824(%r10), %ymm11
.Ltmp8369:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm11, %ymm4
.Ltmp8370:
	.loc	57 82 19
	vandnps	%ymm4, %ymm13, %ymm4
.Ltmp8371:
	.loc	21 0 0 is_stmt 0
	vmovups	(%r9,%r11,4), %ymm13
.Ltmp8372:
	.loc	57 82 19
	vandps	%ymm4, %ymm8, %ymm4
.Ltmp8373:
	.loc	57 347 14 is_stmt 1
	vaddps	%ymm0, %ymm11, %ymm8
.Ltmp8374:
	.loc	57 585 19
	vblendvps	%ymm4, %ymm8, %ymm11, %ymm8
.Ltmp8375:
	.loc	21 0 0 is_stmt 0
	vmovups	(%r14,%r11,4), %ymm11
.Ltmp8376:
	movl	(%r9,%rsi,4), %eax
	movl	(%r9,%rcx,4), %ecx
	movl	(%r9,%rdx,4), %edx
	movl	(%r9,%rdi,4), %esi
.Ltmp8377:
	.loc	57 585 19
	vblendvps	%ymm14, 832(%r10), %ymm8, %ymm8
.Ltmp8378:
	.loc	57 117 19 is_stmt 1
	vorps	%ymm4, %ymm14, %ymm4
.Ltmp8379:
	.loc	21 392 65
	vmovaps	800(%r10), %ymm5
.Ltmp8380:
	.loc	57 585 19
	vpcmpgtd	%ymm4, %ymm10, %ymm4
	vpbroadcastd	.LCPI46_2(%rip), %ymm2
	vpand	%ymm2, %ymm4, %ymm4
.Ltmp8381:
	.loc	21 373 5
	vmovaps	%ymm8, 1824(%r10)
	.loc	21 382 5
	vmovdqa	%ymm4, 1792(%r10)
.Ltmp8382:
	.loc	57 347 14
	vaddps	1408(%r10), %ymm0, %ymm8
.Ltmp8383:
	.loc	57 347 14 is_stmt 0
	vsubps	%ymm3, %ymm1, %ymm1
.Ltmp8384:
	.loc	57 283 14 is_stmt 1
	vmulps	%ymm1, %ymm8, %ymm1
.Ltmp8385:
	.loc	57 713 19
	vbroadcastss	.LCPI46_21(%rip), %ymm14
	vxorps	1536(%r10), %ymm14, %ymm3
.Ltmp8386:
	.loc	57 233 14
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp8387:
	.loc	21 392 36
	vmovaps	1856(%r10), %ymm3
.Ltmp8388:
	.loc	57 257 14
	vminps	%ymm10, %ymm1, %ymm1
.Ltmp8389:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm4, %ymm4
.Ltmp8390:
	.loc	57 585 19
	vpcmpgtd	%ymm4, %ymm10, %ymm4
	vpandn	%ymm1, %ymm4, %ymm1
.Ltmp8391:
	.loc	57 871 14
	vcmpgt_oqps	%ymm3, %ymm1, %ymm4
.Ltmp8392:
	.loc	57 585 19
	vblendvps	%ymm4, 768(%r10), %ymm5, %ymm4
.Ltmp8393:
	.loc	57 347 14
	vsubps	%ymm3, %ymm1, %ymm1
.Ltmp8394:
	.loc	57 283 14
	vmulps	%ymm4, %ymm1, %ymm1
.Ltmp8395:
	.loc	57 48 14
	vaddps	%ymm1, %ymm3, %ymm1
.Ltmp8396:
	.loc	57 82 19
	vandps	%ymm7, %ymm1, %ymm3
	vbroadcastss	.LCPI46_22(%rip), %ymm4
.Ltmp8397:
	.loc	57 871 14
	vcmplt_oqps	%ymm4, %ymm3, %ymm3
.Ltmp8398:
	.loc	57 82 19
	vandnps	%ymm1, %ymm3, %ymm1
	vbroadcastss	.LCPI46_23(%rip), %ymm3
.Ltmp8399:
	.loc	57 283 14
	vmulps	%ymm3, %ymm1, %ymm3
	vbroadcastss	.LCPI46_24(%rip), %ymm4
.Ltmp8400:
	.loc	57 233 14
	vmaxps	%ymm4, %ymm3, %ymm3
	vbroadcastss	.LCPI46_25(%rip), %ymm4
.Ltmp8401:
	.loc	57 257 14
	vminps	%ymm4, %ymm3, %ymm3
.Ltmp8402:
	.loc	57 471 14
	vroundps	$9, %ymm3, %ymm4
.Ltmp8403:
	.loc	57 347 14
	vsubps	%ymm4, %ymm3, %ymm3
	vbroadcastss	.LCPI46_26(%rip), %ymm5
.Ltmp8404:
	.loc	57 283 14
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI46_27(%rip), %ymm8
.Ltmp8405:
	.loc	57 48 14
	vaddps	%ymm5, %ymm8, %ymm5
.Ltmp8406:
	.loc	57 283 14
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI46_28(%rip), %ymm8
.Ltmp8407:
	.loc	57 48 14
	vaddps	%ymm5, %ymm8, %ymm5
.Ltmp8408:
	.loc	57 283 14
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI46_29(%rip), %ymm8
.Ltmp8409:
	.loc	57 48 14
	vaddps	%ymm5, %ymm8, %ymm5
.Ltmp8410:
	.loc	57 283 14
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI46_30(%rip), %ymm8
.Ltmp8411:
	.loc	57 48 14
	vaddps	%ymm5, %ymm8, %ymm5
.Ltmp8412:
	.loc	57 283 14
	vmulps	%ymm5, %ymm3, %ymm3
.Ltmp8413:
	.loc	1 551 14
	vmovd	%eax, %xmm5
	vpinsrd	$1, %ecx, %xmm5, %xmm5
	vpinsrd	$2, %edx, %xmm5, %xmm5
	vpinsrd	$3, %esi, %xmm5, %xmm5
.Ltmp8414:
	.loc	21 394 5
	vmovaps	%ymm1, 1856(%r10)
.Ltmp8415:
	.loc	57 48 14
	vaddps	%ymm2, %ymm3, %ymm3
	vbroadcastss	.LCPI46_31(%rip), %ymm8
.Ltmp8416:
	.loc	57 48 14 is_stmt 0
	vaddps	%ymm4, %ymm8, %ymm4
.Ltmp8417:
	.loc	58 2798 24 is_stmt 1
	vpslld	$23, %ymm4, %ymm4
.Ltmp8418:
	.loc	57 283 14
	vmulps	%ymm4, %ymm3, %ymm3
.Ltmp8419:
	.loc	57 871 14
	vcmpeqps	%ymm1, %ymm10, %ymm1
.Ltmp8420:
	.loc	21 398 49
	vmovaps	864(%r10), %ymm4
.Ltmp8421:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm4, %ymm4
.Ltmp8422:
	.loc	57 117 19
	vorps	%ymm1, %ymm4, %ymm1
.Ltmp8423:
	.loc	57 283 14
	vmulps	%ymm3, %ymm13, %ymm3
.Ltmp8424:
	.loc	57 585 19
	vblendvps	%ymm1, %ymm13, %ymm3, %ymm13
.Ltmp8425:
	.loc	21 343 23
	vmovaps	1088(%r10), %ymm1
	.loc	21 345 9
	vmovaps	1120(%r10), %ymm3
.Ltmp8426:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm3, %ymm3
	vmovaps	480(%rsp), %ymm0
.Ltmp8427:
	.loc	1 551 14
	vinsertf128	$1, 128(%rsp), %ymm0, %ymm2
.Ltmp8428:
	.loc	57 82 19
	vandps	%ymm7, %ymm2, %ymm2
	vbroadcastss	.LCPI46_5(%rip), %ymm0
.Ltmp8429:
	.loc	57 283 14
	vmulps	%ymm0, %ymm2, %ymm4
.Ltmp8430:
	.loc	1 551 14
	vinserti128	$1, 224(%rsp), %ymm5, %ymm5
.Ltmp8431:
	.loc	57 82 19
	vpand	%ymm7, %ymm5, %ymm5
.Ltmp8432:
	.loc	57 283 14
	vmulps	%ymm0, %ymm5, %ymm8
.Ltmp8433:
	.loc	57 48 14
	vaddps	%ymm4, %ymm8, %ymm4
.Ltmp8434:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm1, %ymm1
.Ltmp8435:
	.loc	57 233 14
	vmaxps	%ymm5, %ymm2, %ymm5
.Ltmp8436:
	.loc	57 585 19
	vblendvps	%ymm1, %ymm5, %ymm2, %ymm1
.Ltmp8437:
	.loc	57 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm4, %ymm1, %ymm1
.Ltmp8438:
	.loc	57 233 14 is_stmt 1
	vbroadcastss	.LCPI46_6(%rip), %ymm0
	vmaxps	%ymm0, %ymm1, %ymm1
.Ltmp8439:
	.loc	57 233 14 is_stmt 0
	vbroadcastss	.LCPI46_7(%rip), %ymm0
	vmaxps	%ymm0, %ymm1, %ymm1
.Ltmp8440:
	.loc	58 264 24 is_stmt 1
	vandps	%ymm1, %ymm12, %ymm2
.Ltmp8441:
	.loc	58 2315 14
	vorps	%ymm6, %ymm2, %ymm2
	vbroadcastss	.LCPI46_3(%rip), %ymm6
.Ltmp8442:
	.loc	57 347 14
	vaddps	%ymm6, %ymm2, %ymm3
.Ltmp8443:
	.loc	57 283 14
	vbroadcastss	.LCPI46_10(%rip), %ymm0
	vmulps	%ymm0, %ymm3, %ymm2
.Ltmp8444:
	.loc	57 48 14
	vbroadcastss	.LCPI46_11(%rip), %ymm0
	vsubps	%ymm2, %ymm0, %ymm2
.Ltmp8445:
	.loc	57 283 14
	vmulps	%ymm2, %ymm3, %ymm2
.Ltmp8446:
	.loc	57 48 14
	vbroadcastss	.LCPI46_12(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
.Ltmp8447:
	.loc	57 283 14
	vmulps	%ymm2, %ymm3, %ymm2
.Ltmp8448:
	.loc	57 48 14
	vbroadcastss	.LCPI46_13(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
.Ltmp8449:
	.loc	57 283 14
	vmulps	%ymm2, %ymm3, %ymm2
.Ltmp8450:
	.loc	57 48 14
	vbroadcastss	.LCPI46_14(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
.Ltmp8451:
	.loc	57 283 14
	vmulps	%ymm2, %ymm3, %ymm2
.Ltmp8452:
	.loc	57 48 14
	vbroadcastss	.LCPI46_15(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm4
.Ltmp8453:
	.loc	58 3217 24
	vpsrld	$23, %ymm1, %ymm1
.Ltmp8454:
	.loc	58 2315 24
	vpor	%ymm1, %ymm15, %ymm1
.Ltmp8455:
	.loc	57 347 14
	vaddps	%ymm1, %ymm9, %ymm1
.Ltmp8456:
	.loc	21 335 21
	vmovaps	1888(%r10), %ymm2
.Ltmp8457:
	.loc	57 283 14
	vmulps	%ymm4, %ymm3, %ymm3
.Ltmp8458:
	.loc	57 48 14
	vaddps	%ymm3, %ymm1, %ymm1
.Ltmp8459:
	.loc	57 283 14
	vbroadcastss	.LCPI46_18(%rip), %ymm0
	vmulps	%ymm0, %ymm1, %ymm1
.Ltmp8460:
	.loc	57 257 14
	vbroadcastss	.LCPI46_19(%rip), %ymm0
	vminps	%ymm0, %ymm1, %ymm1
.Ltmp8461:
	.loc	57 233 14
	vbroadcastss	.LCPI46_20(%rip), %ymm0
	vmaxps	%ymm0, %ymm1, %ymm1
.Ltmp8462:
	.loc	21 362 20
	vmovaps	2400(%r10), %ymm3
.Ltmp8463:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm3, %ymm3
.Ltmp8464:
	.loc	57 871 14 is_stmt 0
	vcmpge_oqps	%ymm2, %ymm1, %ymm4
.Ltmp8465:
	.loc	57 347 14 is_stmt 1
	vsubps	2272(%r10), %ymm2, %ymm5
.Ltmp8466:
	.loc	57 871 14
	vcmpge_oqps	%ymm5, %ymm1, %ymm5
.Ltmp8467:
	.loc	57 82 19
	vandnps	%ymm4, %ymm3, %ymm4
.Ltmp8468:
	.loc	21 370 47
	vmovaps	2432(%r10), %ymm8
.Ltmp8469:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm8, %ymm9
.Ltmp8470:
	.loc	57 82 19
	vandnps	%ymm9, %ymm5, %ymm9
.Ltmp8471:
	.loc	57 82 19 is_stmt 0
	vandps	%ymm3, %ymm5, %ymm5
.Ltmp8472:
	.loc	57 82 19
	vandps	%ymm3, %ymm9, %ymm3
.Ltmp8473:
	.loc	57 347 14 is_stmt 1
	vaddps	%ymm6, %ymm8, %ymm9
.Ltmp8474:
	.loc	57 585 19
	vblendvps	%ymm3, %ymm9, %ymm8, %ymm8
.Ltmp8475:
	.loc	57 117 19
	vorps	%ymm4, %ymm5, %ymm4
.Ltmp8476:
	.loc	57 585 19
	vblendvps	%ymm4, 1024(%r10), %ymm8, %ymm5
.Ltmp8477:
	.loc	57 117 19
	vorps	%ymm4, %ymm3, %ymm3
.Ltmp8478:
	.loc	57 347 14
	vaddps	2016(%r10), %ymm6, %ymm4
.Ltmp8479:
	.loc	57 713 19
	vxorps	2144(%r10), %ymm14, %ymm8
.Ltmp8480:
	.loc	21 373 5
	vmovaps	%ymm5, 2432(%r10)
.Ltmp8481:
	.loc	57 347 14
	vsubps	%ymm2, %ymm1, %ymm1
.Ltmp8482:
	.loc	57 585 19
	vpcmpgtd	%ymm3, %ymm10, %ymm2
	vpbroadcastd	.LCPI46_2(%rip), %ymm0
	vpand	%ymm0, %ymm2, %ymm2
.Ltmp8483:
	.loc	21 382 5
	vmovdqa	%ymm2, 2400(%r10)
.Ltmp8484:
	.loc	57 283 14
	vmulps	%ymm1, %ymm4, %ymm1
.Ltmp8485:
	.loc	21 392 36
	vmovaps	2464(%r10), %ymm3
.Ltmp8486:
	.loc	57 233 14
	vmaxps	%ymm8, %ymm1, %ymm1
.Ltmp8487:
	.loc	21 392 65
	vmovaps	992(%r10), %ymm4
.Ltmp8488:
	.loc	57 257 14
	vminps	%ymm10, %ymm1, %ymm1
.Ltmp8489:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm2
.Ltmp8490:
	.loc	57 585 19
	vpcmpgtd	%ymm2, %ymm10, %ymm2
	vpandn	%ymm1, %ymm2, %ymm1
.Ltmp8491:
	.loc	57 871 14
	vcmpgt_oqps	%ymm3, %ymm1, %ymm2
.Ltmp8492:
	.loc	57 585 19
	vblendvps	%ymm2, 960(%r10), %ymm4, %ymm2
.Ltmp8493:
	.loc	57 347 14
	vsubps	%ymm3, %ymm1, %ymm1
.Ltmp8494:
	.loc	57 283 14
	vmulps	%ymm2, %ymm1, %ymm1
.Ltmp8495:
	.loc	57 48 14
	vaddps	%ymm1, %ymm3, %ymm1
.Ltmp8496:
	.loc	57 82 19
	vandps	%ymm7, %ymm1, %ymm2
.Ltmp8497:
	.loc	57 871 14
	vbroadcastss	.LCPI46_22(%rip), %ymm3
	vcmplt_oqps	%ymm3, %ymm2, %ymm2
.Ltmp8498:
	.loc	57 82 19
	vandnps	%ymm1, %ymm2, %ymm1
.Ltmp8499:
	.loc	57 283 14
	vbroadcastss	.LCPI46_23(%rip), %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
.Ltmp8500:
	.loc	57 233 14
	vbroadcastss	.LCPI46_24(%rip), %ymm3
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp8501:
	.loc	57 257 14
	vbroadcastss	.LCPI46_25(%rip), %ymm3
	vminps	%ymm3, %ymm2, %ymm2
.Ltmp8502:
	.loc	57 471 14
	vroundps	$9, %ymm2, %ymm3
.Ltmp8503:
	.loc	57 347 14
	vsubps	%ymm3, %ymm2, %ymm2
.Ltmp8504:
	.loc	57 283 14
	vbroadcastss	.LCPI46_26(%rip), %ymm4
	vmulps	%ymm4, %ymm2, %ymm4
.Ltmp8505:
	.loc	57 48 14
	vbroadcastss	.LCPI46_27(%rip), %ymm5
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8506:
	.loc	57 283 14
	vmulps	%ymm4, %ymm2, %ymm4
.Ltmp8507:
	.loc	57 48 14
	vbroadcastss	.LCPI46_28(%rip), %ymm5
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8508:
	.loc	57 283 14
	vmulps	%ymm4, %ymm2, %ymm4
.Ltmp8509:
	.loc	57 48 14
	vbroadcastss	.LCPI46_29(%rip), %ymm5
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8510:
	.loc	57 283 14
	vmulps	%ymm4, %ymm2, %ymm4
.Ltmp8511:
	.loc	57 48 14
	vbroadcastss	.LCPI46_30(%rip), %ymm5
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8512:
	.loc	57 283 14
	vmulps	%ymm4, %ymm2, %ymm2
.Ltmp8513:
	.loc	21 394 5
	vmovaps	%ymm1, 2464(%r10)
.Ltmp8514:
	.loc	57 48 14
	vaddps	%ymm0, %ymm2, %ymm2
.Ltmp8515:
	.loc	57 48 14 is_stmt 0
	vbroadcastss	.LCPI46_31(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm3
.Ltmp8516:
	.loc	58 2798 24 is_stmt 1
	vpslld	$23, %ymm3, %ymm3
.Ltmp8517:
	.loc	57 283 14
	vmulps	%ymm3, %ymm2, %ymm2
.Ltmp8518:
	.loc	57 871 14
	vcmpeqps	%ymm1, %ymm10, %ymm1
.Ltmp8519:
	.loc	21 398 49
	vmovaps	1056(%r10), %ymm3
.Ltmp8520:
	.loc	57 871 14
	vcmpgt_oqps	%ymm10, %ymm3, %ymm3
.Ltmp8521:
	.loc	57 117 19
	vorps	%ymm1, %ymm3, %ymm1
.Ltmp8522:
	.loc	57 283 14
	vmulps	%ymm2, %ymm11, %ymm2
.Ltmp8523:
	.loc	57 585 19
	vblendvps	%ymm1, %ymm11, %ymm2, %ymm1
	movq	112(%rsp), %rax
.Ltmp8524:
	.loc	1 551 14
	vmovups	%ymm13, (%rax)
	movq	88(%rsp), %rax
.Ltmp8525:
	.loc	1 551 14 is_stmt 0
	vmovups	%ymm1, (%rax)
	movq	56(%rsp), %r12
.Ltmp8526:
	.loc	21 0 0
	incq	%r12
	movq	120(%rsp), %r8
.Ltmp8527:
	.loc	8 1916 50 is_stmt 1
	addq	$8, %r8
	cmpq	%r12, 448(%rsp)
	movq	152(%rsp), %rcx
	movq	64(%rsp), %rdx
	movq	80(%rsp), %r13
	movq	384(%rsp), %r15
	movq	96(%rsp), %rbx
.Ltmp8528:
	.loc	11 900 12
	jne	.LBB46_107
.Ltmp8529:
	.loc	11 0 12 is_stmt 0
	movq	448(%rsp), %rax
.Ltmp8530:
	.loc	15 2584 13 is_stmt 1
	addl	%ebx, %eax
.Ltmp8531:
	.loc	21 297 5
	movl	%eax, 2608(%r10)
.Ltmp8532:
.LBB46_147:
	.loc	21 0 5 is_stmt 0
	movq	192(%rsp), %rax
	.loc	6 698 9 is_stmt 1
	subl	%eax, 2620(%r10)
.Ltmp8533:
	.loc	6 765 33
	movq	$0, 256(%rsp)
	movq	$2, 264(%rsp)
	movq	208(%rsp), %rax
	movq	%rax, 272(%rsp)
	movq	184(%rsp), %rax
	movq	%rax, 280(%rsp)
	movq	200(%rsp), %rax
	movq	%rax, 288(%rsp)
	movq	176(%rsp), %rax
	movq	%rax, 296(%rsp)
	movq	$0, 304(%rsp)
	leaq	1152(%r10), %rax
	movq	%rax, 608(%rsp)
	leaq	512(%r10), %rax
	movq	%rax, 384(%rsp)
	leaq	768(%r10), %rax
	movq	%rax, 368(%rsp)
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
	vmovaps	%ymm0, 416(%rsp)
.Ltmp8534:
	.loc	27 131 12
	leaq	(,%rcx,8), %r9
	movl	$24, %edx
	vbroadcastss	.LCPI46_4(%rip), %ymm4
	vbroadcastss	.LCPI46_32(%rip), %ymm5
	vbroadcastss	.LCPI46_2(%rip), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	vmovsd	.LCPI46_33(%rip), %xmm6
	vmovsd	.LCPI46_34(%rip), %xmm7
	vmovsd	.LCPI46_35(%rip), %xmm8
	xorl	%eax, %eax
	xorl	%esi, %esi
	movq	%r9, 128(%rsp)
	vmovaps	%ymm4, 672(%rsp)
	jmp	.LBB46_149
	.loc	27 0 12 is_stmt 0
.Ltmp8535:
	.p2align	4
.LBB46_148:
	movl	$1, %esi
	movl	$32, %edx
	.loc	27 131 12 is_stmt 1
	testb	$1, 640(%rsp)
	movb	$1, %al
	jne	.LBB46_174
.Ltmp8536:
.LBB46_149:
	.loc	27 0 12 is_stmt 0
	movq	%rax, 640(%rsp)
.Ltmp8537:
	.loc	25 253 13 is_stmt 1
	movq	%rsi, %rcx
	shlq	$4, %rcx
	leaq	272(%rsp), %rax
.Ltmp8538:
	.loc	1 1733 9
	movq	(%rax,%rcx), %r14
	movq	%rcx, 40(%rsp)
	movq	8(%rax,%rcx), %r15
.Ltmp8539:
	.loc	16 2155 12
	movq	%r15, %rax
	vmovaps	416(%rsp), %ymm0
	andq	$-8, %rax
	je	.LBB46_152
.Ltmp8540:
	.loc	16 0 12 is_stmt 0
	xorl	%ecx, %ecx
	vmovaps	416(%rsp), %ymm0
	.p2align	4
.LBB46_151:
.Ltmp8541:
	.loc	57 82 19 is_stmt 1
	vandps	(%r14,%rcx,4), %ymm4, %ymm1
.Ltmp8542:
	.loc	57 871 14
	vcmplt_oqps	%ymm5, %ymm1, %ymm1
.Ltmp8543:
	.loc	57 82 19
	vandps	%ymm1, %ymm0, %ymm0
.Ltmp8544:
	.loc	16 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB46_151
.Ltmp8545:
.LBB46_152:
	.loc	6 0 0 is_stmt 0
	imulq	$608, %rsi, %rax
.Ltmp8546:
	.file	59 "/home/bl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wide-1.6.1" "src/f32x8_.rs"
	.loc	59 176 9 is_stmt 1
	vpcmpeqd	%ymm1, %ymm1, %ymm1
	vtestps	%ymm1, %ymm0
	vandps	1856(%r10,%rax), %ymm4, %ymm0
.Ltmp8547:
	.loc	6 767 16
	jae	.LBB46_154
.Ltmp8548:
	.loc	57 871 14
	vcmplt_oqps	%ymm5, %ymm0, %ymm1
.Ltmp8549:
	.loc	57 82 19
	vandps	416(%rsp), %ymm1, %ymm1
.Ltmp8550:
	.loc	59 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp8551:
	.loc	6 767 43
	jb	.LBB46_148
.LBB46_154:
	.loc	6 0 43 is_stmt 0
	movq	%rax, 96(%rsp)
	movq	%rsi, 64(%rsp)
	movq	%rdx, 80(%rsp)
.Ltmp8552:
	.loc	16 2155 12 is_stmt 1
	movq	%r15, %rax
	vmovaps	416(%rsp), %ymm1
	movabsq	$2305843009213693944, %rcx
	andq	%rcx, %rax
	je	.LBB46_157
.Ltmp8553:
	.loc	16 0 12 is_stmt 0
	xorl	%ecx, %ecx
	vmovaps	416(%rsp), %ymm1
	.p2align	4
.LBB46_156:
.Ltmp8554:
	.loc	57 82 19 is_stmt 1
	vandps	(%r14,%rcx,4), %ymm4, %ymm2
.Ltmp8555:
	.loc	57 871 14
	vcmplt_oqps	%ymm5, %ymm2, %ymm2
.Ltmp8556:
	.loc	57 82 19
	vandps	%ymm2, %ymm1, %ymm1
.Ltmp8557:
	.loc	16 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB46_156
.Ltmp8558:
.LBB46_157:
	.loc	57 871 14
	vcmplt_oqps	%ymm5, %ymm0, %ymm0
.Ltmp8559:
	.loc	57 82 19
	vandps	416(%rsp), %ymm0, %ymm0
.Ltmp8560:
	.loc	57 585 19
	vpsrad	$31, %ymm1, %ymm1
	vmovdqa	576(%rsp), %ymm2
	vpandn	%ymm2, %ymm1, %ymm1
.Ltmp8561:
	.loc	28 185 12
	vmovd	%xmm1, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movq	%rcx, 56(%rsp)
	vpextrd	$1, %xmm1, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movq	%rcx, 320(%rsp)
	vpextrd	$2, %xmm1, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movq	%rcx, 112(%rsp)
	vpextrd	$3, %xmm1, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movq	%rcx, 88(%rsp)
	vextracti128	$1, %ymm1, %xmm1
	vmovd	%xmm1, %eax
	xorl	%r13d, %r13d
	testl	%eax, %eax
	setne	%r13b
	shll	$4, %r13d
	vpextrd	$1, %xmm1, %eax
	xorl	%ebx, %ebx
	testl	%eax, %eax
	setne	%bl
	vpextrd	$2, %xmm1, %eax
	shll	$5, %ebx
	xorl	%r9d, %r9d
	testl	%eax, %eax
	setne	%r9b
	shll	$6, %r9d
	vpextrd	$3, %xmm1, %eax
	xorl	%r12d, %r12d
	testl	%eax, %eax
	setne	%r12b
	shll	$7, %r12d
.Ltmp8562:
	.loc	57 585 19
	vandnps	%ymm2, %ymm0, %ymm0
.Ltmp8563:
	.loc	28 185 12
	vmovd	%xmm0, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movl	%ecx, 224(%rsp)
	vpextrd	$1, %xmm0, %eax
	xorl	%r11d, %r11d
	testl	%eax, %eax
	setne	%r11b
	addl	%r11d, %r11d
	vpextrd	$2, %xmm0, %ecx
	xorl	%eax, %eax
	testl	%ecx, %ecx
	setne	%al
	shll	$2, %eax
	vpextrd	$3, %xmm0, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	shll	$3, %edx
	vextracti128	$1, %ymm0, %xmm0
	vmovd	%xmm0, %esi
	xorl	%ecx, %ecx
	testl	%esi, %esi
	setne	%cl
	shll	$4, %ecx
	vpextrd	$1, %xmm0, %edi
	xorl	%esi, %esi
	testl	%edi, %edi
	setne	%sil
	shll	$5, %esi
	vpextrd	$2, %xmm0, %edi
	xorl	%r8d, %r8d
	testl	%edi, %edi
	setne	%r8b
	shll	$6, %r8d
	vpextrd	$3, %xmm0, %r10d
	xorl	%edi, %edi
	testl	%r10d, %r10d
	setne	%dil
	shll	$7, %edi
.Ltmp8564:
	.loc	28 185 12 is_stmt 0
	orl	%r8d, %edi
	movq	56(%rsp), %r8
	movq	320(%rsp), %r10
.Ltmp8565:
	.loc	28 185 12
	leal	(%r8,%r10,2), %r8d
	movq	112(%rsp), %r10
	leal	(%r8,%r10,4), %r8d
	movq	88(%rsp), %r10
	leal	(%r8,%r10,8), %r8d
	orl	%r13d, %r8d
	orl	%ebx, %r9d
	orl	%r12d, %r9d
	orl	%r8d, %r9d
.Ltmp8566:
	.loc	28 185 12
	orl	224(%rsp), %r11d
	orl	%eax, %r11d
	orl	%edx, %r11d
	orl	%ecx, %r11d
	orl	%esi, %r11d
	orl	%r9d, %r11d
.Ltmp8567:
	.loc	6 771 17 is_stmt 1
	orl	%edi, %r11d
	movq	64(%rsp), %rcx
.Ltmp8568:
	.loc	6 772 13
	movq	%rcx, %rax
	shlq	$5, %rax
	movq	48(%rsp), %r10
	addq	%r10, %rax
	movq	%rax, 56(%rsp)
	shlq	$6, %rcx
	movq	608(%rsp), %rax
	leaq	(%rax,%rcx), %r13
	movq	40(%rsp), %rax
.Ltmp8569:
	.loc	11 900 12
	addq	384(%rsp), %rax
	movq	%rax, 40(%rsp)
	leaq	(%r10,%rcx), %rax
	movq	%rax, 120(%rsp)
	leaq	(%rcx,%rcx,2), %rax
	movq	368(%rsp), %rcx
	leaq	(%rcx,%rax), %rdx
	movq	%rdx, 480(%rsp)
	leaq	64(%rcx,%rax), %rcx
	movq	%rcx, 64(%rsp)
	leaq	832(%r10,%rax), %rax
	movq	%rax, 448(%rsp)
	movq	408(%rsp), %rax
	movq	96(%rsp), %rdx
	leaq	(%rax,%rdx), %rcx
	movq	%rcx, 512(%rsp)
	leaq	576(%rax,%rdx), %rbx
	movq	80(%rsp), %rax
	addq	%rsp, %rax
	addq	$800, %rax
	movq	%rax, 80(%rsp)
	xorl	%r12d, %r12d
	movq	152(%rsp), %r8
	movq	128(%rsp), %r9
	movl	%r11d, 104(%rsp)
	jmp	.LBB46_160
.Ltmp8570:
	.loc	11 0 12 is_stmt 0
.Ltmp8571:
	.p2align	4
.LBB46_158:
	.loc	6 781 34 is_stmt 1
	leaq	(%r12,%r12,4), %rax
	movq	80(%rsp), %rsi
	movq	(%rsi,%rax,8), %rcx
.Ltmp8572:
	.loc	15 2428 13
	addq	%r8, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp8573:
	.loc	6 786 17
	movq	%rcx, (%rsi,%rax,8)
.Ltmp8574:
.LBB46_159:
	.loc	11 0 0 is_stmt 0
	incq	%r12
	.loc	11 900 12 is_stmt 1
	addq	$4, %r14
.Ltmp8575:
	.loc	8 1916 50
	cmpq	$8, %r12
.Ltmp8576:
	.loc	11 900 12
	je	.LBB46_148
.Ltmp8577:
.LBB46_160:
	.loc	6 773 20
	btl	%r12d, %r11d
	jae	.LBB46_159
	.loc	6 0 20 is_stmt 0
	testl	%r8d, %r8d
.Ltmp8578:
	.loc	11 900 12 is_stmt 1
	je	.LBB46_165
.Ltmp8579:
	.loc	11 0 12 is_stmt 0
	xorl	%eax, %eax
	.p2align	4
.LBB46_163:
.Ltmp8580:
	.loc	6 777 21 is_stmt 1
	leaq	(%r12,%rax), %rdi
	cmpq	%r15, %rdi
	jae	.LBB46_182
	movl	$0, (%r14,%rax,4)
.Ltmp8581:
	.loc	8 1916 50
	addq	$8, %rax
	cmpq	%rax, %r9
.Ltmp8582:
	.loc	11 900 12
	jne	.LBB46_163
.Ltmp8583:
.LBB46_165:
	.loc	6 559 21
	movq	2496(%r10), %rax
.Ltmp8584:
	.loc	8 1916 50
	testq	%rax, %rax
.Ltmp8585:
	.loc	11 900 12
	je	.LBB46_169
.Ltmp8586:
	.loc	11 0 12 is_stmt 0
	movq	8(%r13), %rsi
	movq	%r12, %rdi
	.p2align	4
.LBB46_167:
.Ltmp8587:
	.loc	6 562 13 is_stmt 1
	cmpq	%rsi, %rdi
	jae	.LBB46_183
	movq	(%r13), %rcx
	movl	$0, (%rcx,%rdi,4)
.Ltmp8588:
	.loc	8 1916 50
	addq	$8, %rdi
	decq	%rax
.Ltmp8589:
	.loc	11 900 12
	jne	.LBB46_167
.Ltmp8590:
.LBB46_169:
	.loc	6 506 22
	movq	%r12, %rax
	shlq	$6, %rax
	movq	56(%rsp), %rcx
	vmovss	(%rcx,%rax), %xmm12
	vmovss	4(%rcx,%rax), %xmm11
	vmovss	8(%rcx,%rax), %xmm10
	vmovss	12(%rcx,%rax), %xmm9
	vmovss	16(%rcx,%rax), %xmm0
	vmovss	20(%rcx,%rax), %xmm2
	vmovss	24(%rcx,%rax), %xmm13
	vmovss	28(%rcx,%rax), %xmm1
.Ltmp8591:
	.loc	6 507 9
	movq	%r12, %rax
	shlq	$5, %rax
	movq	40(%rsp), %rcx
	vmovss	%xmm1, (%rcx,%rax)
	vmovss	%xmm0, 4(%rcx,%rax)
	vmovss	%xmm2, 8(%rcx,%rax)
	vmovss	%xmm13, 12(%rcx,%rax)
.Ltmp8592:
	.loc	6 530 27
	movl	2576(%r10), %eax
.Ltmp8593:
	.loc	9 82 17
	vcvtss2sd	%xmm1, %xmm1, %xmm1
.Ltmp8594:
	.loc	9 82 17 is_stmt 0
	vcvtsi2sd	%rax, %xmm15, %xmm3
.Ltmp8595:
	.loc	6 403 20 is_stmt 1
	vmulsd	%xmm3, %xmm1, %xmm1
	vdivsd	%xmm6, %xmm1, %xmm1
	.loc	6 403 19 is_stmt 0
	vaddsd	%xmm7, %xmm1, %xmm1
.Ltmp8596:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm1, %xmm1, %xmm1
	vmovq	%xmm1, %rcx
.Ltmp8597:
	.loc	6 404 9
	testq	%rcx, %rcx
	sets	%dl
	movabsq	$9223372036854775807, %rsi
	andq	%rsi, %rcx
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
	jne	.LBB46_158
	vucomisd	%xmm8, %xmm1
	ja	.LBB46_158
.Ltmp8598:
	.loc	9 82 17
	vcvtss2sd	%xmm2, %xmm2, %xmm2
.Ltmp8599:
	.loc	6 403 20
	vmulsd	%xmm3, %xmm2, %xmm2
	vdivsd	%xmm6, %xmm2, %xmm2
	.loc	6 403 19 is_stmt 0
	vaddsd	%xmm7, %xmm2, %xmm2
.Ltmp8600:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm2, %xmm2, %xmm2
	vmovq	%xmm2, %rcx
.Ltmp8601:
	.loc	6 404 9
	testq	%rcx, %rcx
	sets	%dl
	movabsq	$9223372036854775807, %rsi
	andq	%rsi, %rcx
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
	jne	.LBB46_158
	vucomisd	%xmm8, %xmm2
	ja	.LBB46_158
.Ltmp8602:
	.loc	6 531 80
	movl	2616(%r10), %ecx
	vxorpd	%xmm3, %xmm3, %xmm3
.Ltmp8603:
	.loc	6 407 10
	vmaxsd	%xmm1, %xmm3, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rdx
.Ltmp8604:
	.loc	6 407 10 is_stmt 0
	vmaxsd	%xmm2, %xmm3, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rsi
.Ltmp8605:
	.loc	6 533 9 is_stmt 1
	subl	%edx, %ecx
	movl	$0, %edx
	cmovbl	%edx, %ecx
	movq	120(%rsp), %rdx
	movl	%ecx, 1184(%rdx,%r12,4)
	.loc	6 534 62
	movl	%esi, %ecx
	vcvtsi2ss	%rcx, %xmm15, %xmm1
	movq	64(%rsp), %rcx
.Ltmp8606:
	.loc	6 396 5
	vmovaps	(%rcx), %ymm2
	vmovaps	%ymm2, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm1
	vmovaps	%ymm1, (%rcx)
	movq	%rax, %rdi
	movq	%rax, 224(%rsp)
	vmovss	%xmm9, 96(%rsp)
	vmovss	%xmm10, 88(%rsp)
	vmovss	%xmm11, 112(%rsp)
	vmovss	%xmm12, 320(%rsp)
	vmovss	%xmm13, 160(%rsp)
.Ltmp8607:
	.loc	6 538 13
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movq	480(%rsp), %rax
.Ltmp8608:
	.loc	6 396 5
	vmovaps	(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, (%rax)
	vmovss	160(%rsp), %xmm0
	movq	224(%rsp), %rdi
.Ltmp8609:
	.loc	6 543 13
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movl	104(%rsp), %r11d
	vmovsd	.LCPI46_35(%rip), %xmm8
	vmovsd	.LCPI46_34(%rip), %xmm7
	vmovsd	.LCPI46_33(%rip), %xmm6
	vbroadcastss	.LCPI46_32(%rip), %ymm5
	vmovaps	672(%rsp), %ymm4
	movq	128(%rsp), %r9
	movq	152(%rsp), %r8
	movq	48(%rsp), %r10
	movq	64(%rsp), %rax
.Ltmp8610:
	.loc	6 396 5
	vmovaps	-32(%rax), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -32(%rax)
	movq	448(%rsp), %rax
.Ltmp8611:
	.loc	6 514 29
	vmovaps	(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
.Ltmp8612:
	.loc	6 390 5
	vmovss	(%rsp,%r12,4), %xmm0
.Ltmp8613:
	.loc	6 396 5
	vmovaps	(%rbx), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm1
	vmovaps	%ymm1, (%rbx)
.Ltmp8614:
	.loc	6 396 5
	vmovaps	-64(%rbx), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	movl	$1065353216, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm1
	vmovaps	%ymm1, -64(%rbx)
.Ltmp8615:
	.loc	6 396 5
	vmovaps	-32(%rbx), %ymm1
	vmovaps	%ymm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -32(%rbx)
	movq	512(%rsp), %rax
.Ltmp8616:
	.loc	6 396 5
	vmovaps	(%rax), %ymm0
	vmovaps	%ymm0, (%rsp)
	vmovss	320(%rsp), %xmm1
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, (%rax)
.Ltmp8617:
	.loc	6 396 5
	vmovaps	-544(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -544(%rbx)
.Ltmp8618:
	.loc	6 396 5
	vmovaps	-512(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -512(%rbx)
.Ltmp8619:
	.loc	6 396 5
	vmovaps	-480(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -480(%rbx)
.Ltmp8620:
	.loc	6 396 5
	vmovaps	-448(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	vmovss	112(%rsp), %xmm1
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -448(%rbx)
.Ltmp8621:
	.loc	6 396 5
	vmovaps	-416(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -416(%rbx)
.Ltmp8622:
	.loc	6 396 5
	vmovaps	-384(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -384(%rbx)
.Ltmp8623:
	.loc	6 396 5
	vmovaps	-352(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -352(%rbx)
.Ltmp8624:
	.loc	6 396 5
	vmovaps	-320(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	vmovss	88(%rsp), %xmm1
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -320(%rbx)
.Ltmp8625:
	.loc	6 396 5
	vmovaps	-288(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -288(%rbx)
.Ltmp8626:
	.loc	6 396 5
	vmovaps	-256(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -256(%rbx)
.Ltmp8627:
	.loc	6 396 5
	vmovaps	-224(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -224(%rbx)
.Ltmp8628:
	.loc	6 396 5
	vmovaps	-192(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	vmovss	96(%rsp), %xmm1
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -192(%rbx)
.Ltmp8629:
	.loc	6 396 5
	vmovaps	-160(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -160(%rbx)
.Ltmp8630:
	.loc	6 396 5
	vmovaps	-128(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -128(%rbx)
.Ltmp8631:
	.loc	6 396 5
	vmovaps	-96(%rbx), %ymm0
	vmovaps	%ymm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %ymm0
	vmovaps	%ymm0, -96(%rbx)
	jmp	.LBB46_158
.Ltmp8632:
.LBB46_174:
	.loc	6 0 5 is_stmt 0
	leaq	800(%rsp), %rsi
	.loc	6 1094 17 is_stmt 1
	movl	$320, %edx
	movq	568(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	%rbx, %rdi
	movzbl	79(%rsp), %eax
	movb	%al, 320(%rbx)
.Ltmp8633:
.LBB46_175:
	.loc	6 1095 14
	movq	%rdi, %rax
	leaq	-40(%rbp), %rsp
	.loc	6 1095 14 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	vzeroupper
	retq
.LBB46_176:
	.cfi_def_cfa %rbp, 16
.Ltmp8634:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_71049c4faf961883bcf0219fde9a65c7(%rip), %rcx
	movq	64(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8635:
.LBB46_177:
	.loc	25 443 13
	leaq	8(%r8), %rsi
	leaq	.Lalloc_97ed0782c9e7425b1b215f66cb64a347(%rip), %rcx
	movq	%r8, %rdi
	movq	704(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8636:
.LBB46_178:
	.loc	25 456 13
	leaq	.Lalloc_cad5a7b24766ffcdda9cbfe066eb4078(%rip), %rcx
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8637:
.LBB46_179:
	.loc	25 456 13
	leaq	.Lalloc_72e24c5578221343e8a784545a3910d2(%rip), %rcx
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8638:
.LBB46_180:
	.loc	25 443 13
	leaq	.Lalloc_ba99eeeb3482270ebe1c674b4013dd6a(%rip), %rcx
	movq	%r9, %rdi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8639:
.LBB46_181:
	.loc	25 443 13
	leaq	.Lalloc_0f4c27429e4885e1b619f1e4a3587acc(%rip), %rcx
	movq	%r9, %rdi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8640:
.LBB46_182:
	.loc	6 777 21
	leaq	.Lalloc_30d570589940b68d7f77af9df77eb2ee(%rip), %rdx
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8641:
.LBB46_183:
	.loc	6 562 13
	leaq	.Lalloc_f8f0512af3f0ba047152c3c522a44b59(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8642:
.LBB46_184:
	.loc	25 456 13
	leaq	.Lalloc_cad5a7b24766ffcdda9cbfe066eb4078(%rip), %rcx
	movq	%r11, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8643:
.LBB46_185:
	.loc	25 0 13 is_stmt 0
	movabsq	$2305843009213693944, %rsi
.Ltmp8644:
	.loc	11 900 12 is_stmt 1
	andq	%r15, %rsi
	addq	$8, %rsi
.Ltmp8645:
	.loc	25 443 13
	leaq	.Lalloc_aebeae245abdbd708a3d39b73262e641(%rip), %rcx
	movq	%r8, %rdi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8646:
.LBB46_186:
	.loc	25 456 13
	leaq	.Lalloc_72e24c5578221343e8a784545a3910d2(%rip), %rcx
.Ltmp8647:
	.loc	21 0 0 is_stmt 0
	movq	%r11, %rdi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB46_187:
.Ltmp8648:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_ba99eeeb3482270ebe1c674b4013dd6a(%rip), %rcx
	movq	%r11, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8649:
.LBB46_188:
	.loc	25 443 13
	leaq	.Lalloc_0f4c27429e4885e1b619f1e4a3587acc(%rip), %rcx
.Ltmp8650:
	.loc	21 0 0 is_stmt 0
	movq	%r11, %rdi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8651:
.LBB46_189:
	.loc	18 968 21 is_stmt 1
	leaq	.Lalloc_376120b9c5efdf3d59386c16952a74b7(%rip), %rdi
	leaq	.Lalloc_71049c4faf961883bcf0219fde9a65c7(%rip), %rdx
	movl	$31, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core6option13expect_failed@GOTPCREL(%rip)
.Ltmp8652:
.LBB46_190:
	.loc	25 456 13
	leaq	.Lalloc_bc75afababbf20974edb8b966373fd0c(%rip), %rcx
	xorl	%edi, %edi
	movq	184(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8653:
.LBB46_191:
	.loc	25 456 13
	leaq	.Lalloc_a0e50f2d670fa7dbeabdb68abcb7ac10(%rip), %rcx
	xorl	%edi, %edi
	movq	176(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8654:
.LBB46_192:
	.loc	25 581 13
	leaq	.Lalloc_e6adad6d678d00eb6b02b8162f35ebb4(%rip), %rcx
	movq	184(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8655:
.LBB46_193:
	.loc	25 581 13
	leaq	.Lalloc_9c634077742a23e4340a846355ecc484(%rip), %rcx
	movq	176(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8656:
.LBB46_194:
	.loc	25 0 13 is_stmt 0
	movq	%rax, %rsi
.LBB46_195:
.Ltmp8657:
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8658:
.LBB46_202:
	.loc	21 0 29 is_stmt 0
	movq	%rcx, %r11
.Ltmp8659:
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8660:
.LBB46_196:
	.loc	21 0 29 is_stmt 0
	movq	%rcx, %rdi
.Ltmp8661:
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_197:
	.loc	21 0 29 is_stmt 0
	movq	%rcx, %rdi
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8662:
.LBB46_198:
	.loc	21 0 29 is_stmt 0
	movq	%rax, %rsi
.Ltmp8663:
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8664:
.LBB46_199:
	.loc	6 1082 33
	leaq	.Lalloc_71049c4faf961883bcf0219fde9a65c7(%rip), %rdx
	movq	%r8, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_200:
.Ltmp8665:
	.loc	6 1083 31
	leaq	.Lalloc_71049c4faf961883bcf0219fde9a65c7(%rip), %rdx
	movq	%rax, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8666:
.LBB46_201:
	.loc	6 0 31 is_stmt 0
	movq	%r15, %r11
.Ltmp8667:
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_204:
	.loc	21 0 29 is_stmt 0
	movq	%r8, %r11
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_205:
	.loc	21 0 29 is_stmt 0
	movq	%rax, %r11
	.loc	21 274 29
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_206:
	.loc	21 0 29
	movq	%r15, %r11
	.loc	21 274 29
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_208:
	.loc	21 0 29
	movq	128(%rsp), %rsi
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_209:
	.loc	21 0 29 is_stmt 0
	movq	%rbx, %r13
.LBB46_207:
	movq	%rcx, %r11
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_210:
	.loc	21 0 29 is_stmt 0
	movq	%r10, %rsi
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_211:
	.loc	21 0 29 is_stmt 0
	movq	%rdi, %rsi
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_212:
	.loc	21 0 29 is_stmt 0
	movq	%rdx, %rsi
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_213:
	.loc	21 0 29 is_stmt 0
	movq	%r14, %rsi
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_214:
	.loc	21 0 29
	movq	160(%rsp), %rsi
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_215:
	.loc	21 0 29
	movq	%r12, %rsi
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_216:
	.loc	21 0 29
	movq	%r10, %rsi
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_217:
	.loc	21 0 29 is_stmt 0
	movq	%rdx, %rsi
	.loc	21 276 29
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_218:
	.loc	21 0 29
	movq	%r14, %rsi
	.loc	21 276 29
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_219:
	.loc	21 0 29
	movq	%rdi, %r11
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_220:
	.loc	21 0 29 is_stmt 0
	movq	%r12, %rsi
	movq	88(%rsp), %rbx
.LBB46_221:
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_224:
	.loc	21 0 29 is_stmt 0
	movq	%r8, %r11
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_225:
	.loc	21 0 29 is_stmt 0
	movq	%rdi, %rsi
.LBB46_226:
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_227:
	.loc	21 0 29 is_stmt 0
	movq	%rax, %r11
.LBB46_228:
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_229:
	.loc	21 0 29 is_stmt 0
	movq	%rdi, %r11
.LBB46_230:
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	%r13, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8668:
.LBB46_231:
	.loc	21 0 29 is_stmt 0
	movq	%rax, %rsi
.Ltmp8669:
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_232:
	.loc	21 0 29 is_stmt 0
	movq	%r12, %rdi
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_233:
	.loc	21 0 29 is_stmt 0
	movq	%r13, %rsi
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_234:
	.loc	21 0 29 is_stmt 0
	movq	480(%rsp), %rsi
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_235:
	.loc	21 0 29 is_stmt 0
	movq	104(%rsp), %rsi
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_236:
	.loc	21 0 29
	movq	%r13, %rsi
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_237:
	.loc	21 0 29
	movq	%r10, %rdi
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_238:
	.loc	21 0 29 is_stmt 0
	movq	%r15, %rsi
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rsi, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_239:
	.loc	21 0 29 is_stmt 0
	movq	160(%rsp), %rsi
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_240:
	.loc	21 0 29 is_stmt 0
	movq	%r8, %rdi
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_241:
	.loc	21 0 29 is_stmt 0
	movq	%rbx, %rdi
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_242:
	.loc	21 0 29 is_stmt 0
	movq	%r12, %rdi
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_243:
	.loc	21 0 29 is_stmt 0
	movq	%r10, %rdi
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_244:
	.loc	21 0 29 is_stmt 0
	movq	%rbx, %rdi
.LBB46_245:
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_246:
	.loc	21 0 29 is_stmt 0
	movq	320(%rsp), %rsi
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_247:
	.loc	21 0 29 is_stmt 0
	movq	%r8, %rdi
.LBB46_248:
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_249:
	.loc	21 0 29 is_stmt 0
	movq	%r15, %rsi
.LBB46_250:
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8670:
.Lfunc_end46:
	.size	_RNvXse_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank, .Lfunc_end46-_RNvXse_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank
