_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_:
.Lfunc_begin40:
	.loc	1 1883 0
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
	subq	$192, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, 2648(%rsp)
.Ltmp5722:
	.loc	1 1884 51 prologue_end
	movzbl	5424(%rsi), %eax
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqa	%ymm0, 2944(%rsp)
	vmovdqa	%ymm0, 2912(%rsp)
	vmovdqa	%ymm0, 2880(%rsp)
	vmovdqa	%ymm0, 2848(%rsp)
	vmovdqa	%ymm0, 2816(%rsp)
	vmovdqa	%ymm0, 2784(%rsp)
	vmovdqa	%ymm0, 2752(%rsp)
	vmovdqa	%ymm0, 2720(%rsp)
	vmovdqa	%ymm0, 2688(%rsp)
	vmovdqa	%ymm0, 2656(%rsp)
.Ltmp5723:
	.loc	43 1032 9
	movb	%al, 2976(%rsp)
.Ltmp5724:
	.loc	43 186 45
	cmpb	%al, 108(%rdx)
.Ltmp5725:
	.loc	1 1888 12
	jne	.LBB40_538
	cmpq	$0, 64(%rdx)
	jne	.LBB40_538
	.loc	1 0 12 is_stmt 0
	movq	%rsi, %r14
	movq	48(%rdx), %r10
	movq	56(%rdx), %r8
	movq	32(%rdx), %rax
	movq	%rax, 24(%rsp)
	movq	40(%rdx), %rax
	movq	%rax, 1696(%rsp)
	movq	%rdx, 2240(%rsp)
	movq	96(%rdx), %r11
.Ltmp5726:
	.loc	3 900 12 is_stmt 1
	cmpq	$1, %r8
	movq	%r8, %rax
	adcq	$-1, %rax
	movq	%rax, 1760(%rsp)
	leaq	156(%rsi), %rbx
	xorl	%eax, %eax
	vmovss	.LCPI40_0(%rip), %xmm2
	movq	%rsi, 48(%rsp)
	movq	%r8, 16(%rsp)
	movq	%r10, 1824(%rsp)
	movq	%r11, 2016(%rsp)
	jmp	.LBB40_4
	.loc	3 0 12 is_stmt 0
.Ltmp5727:
	.p2align	4
.LBB40_3:
	.loc	3 900 12 is_stmt 1
	addq	$160, %rbx
	movq	1664(%rsp), %rcx
	movq	%rcx, %rax
.Ltmp5728:
	.loc	2 1916 50
	cmpq	$8, %rcx
	movq	48(%rsp), %r14
.Ltmp5729:
	.loc	3 900 12
	je	.LBB40_100
.Ltmp5730:
.LBB40_4:
	.loc	1 1892 25
	cmpq	%r8, %rax
	je	.LBB40_576
.Ltmp5731:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rax), %rcx
.Ltmp5732:
	.loc	1 1893 23 is_stmt 1
	cmpq	1760(%rsp), %rax
	je	.LBB40_577
	.loc	1 0 23 is_stmt 0
	movl	(%r10,%rax,4), %edi
	.loc	1 1893 23
	movl	(%r10,%rcx,4), %esi
.Ltmp5733:
	.loc	38 1050 16 is_stmt 1
	cmpl	%edi, %esi
	jb	.LBB40_548
	cmpq	%rsi, 1696(%rsp)
	jb	.LBB40_548
.Ltmp5734:
	.loc	38 0 16 is_stmt 0
	movq	%rcx, 1664(%rsp)
	movq	%rbx, 192(%rsp)
	.loc	1 1897 17 is_stmt 1
	movl	5524(%r14), %edx
	movl	$0, 352(%rsp)
	movl	$0, 360(%rsp)
	movl	$0, 368(%rsp)
	movl	$0, 376(%rsp)
	movl	$0, 384(%rsp)
	movl	$0, 392(%rsp)
	movl	$0, 400(%rsp)
	movl	$0, 408(%rsp)
	movl	$0, 416(%rsp)
	movl	$0, 424(%rsp)
	movl	$0, 432(%rsp)
	movl	$0, 440(%rsp)
	movl	$0, 448(%rsp)
	movl	$0, 456(%rsp)
	movl	$0, 464(%rsp)
	movl	$0, 472(%rsp)
	movl	$0, 480(%rsp)
	movl	$0, 488(%rsp)
	movl	$0, 496(%rsp)
	movl	$0, 504(%rsp)
.Ltmp5735:
	.loc	32 1714 9
	cmpl	%edi, %esi
.Ltmp5736:
	.loc	33 180 28
	jne	.LBB40_81
.Ltmp5737:
.LBB40_9:
	.loc	33 0 28 is_stmt 0
	movl	$76, %eax
	movq	192(%rsp), %rbx
	movq	%rbx, %rcx
.Ltmp5738:
	.loc	33 180 28
	jmp	.LBB40_13
.Ltmp5739:
	.loc	33 0 28
.Ltmp5740:
	.p2align	4
.LBB40_10:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
.LBB40_11:
	.loc	36 0 0
	vmovd	%xmm0, -4(%rcx)
	movl	%edx, (%rcx)
.Ltmp5741:
.LBB40_12:
	.loc	32 1714 9 is_stmt 1
	addq	$80, %rax
	addq	$2624, %rcx
	cmpq	$236, %rax
.Ltmp5742:
	.loc	33 180 28
	je	.LBB40_3
.Ltmp5743:
.LBB40_13:
	.loc	1 1392 24
	cmpl	$1, 276(%rsp,%rax)
	jne	.LBB40_14
	.loc	1 1392 29 is_stmt 0
	vmovd	280(%rsp,%rax), %xmm0
.Ltmp5744:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -152(%rcx)
	.loc	36 81 48
	vmovd	-156(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5745:
	.loc	36 112 9
	jg	.LBB40_27
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_27
	negl	%edx
	jo	.LBB40_27
.Ltmp5746:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -156(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -148(%rcx)
	movl	%edx, -144(%rcx)
.Ltmp5747:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 284(%rsp,%rax)
	je	.LBB40_29
	.loc	1 0 24 is_stmt 0
.Ltmp5748:
	.p2align	4
.LBB40_15:
	.loc	1 1392 24
	cmpl	$1, 292(%rsp,%rax)
	jne	.LBB40_16
.LBB40_35:
	.loc	1 1392 29
	vmovd	296(%rsp,%rax), %xmm0
.Ltmp5749:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -120(%rcx)
	.loc	36 81 48
	vmovd	-124(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5750:
	.loc	36 112 9
	jg	.LBB40_39
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_39
	negl	%edx
	jo	.LBB40_39
.Ltmp5751:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -124(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -116(%rcx)
	movl	%edx, -112(%rcx)
.Ltmp5752:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 300(%rsp,%rax)
	je	.LBB40_41
	.loc	1 0 24 is_stmt 0
.Ltmp5753:
	.p2align	4
.LBB40_17:
	.loc	1 1392 24
	cmpl	$1, 308(%rsp,%rax)
	jne	.LBB40_18
.LBB40_47:
	.loc	1 1392 29
	vmovd	312(%rsp,%rax), %xmm0
.Ltmp5754:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -88(%rcx)
	.loc	36 81 48
	vmovd	-92(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5755:
	.loc	36 112 9
	jg	.LBB40_51
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_51
	negl	%edx
	jo	.LBB40_51
.Ltmp5756:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -92(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -84(%rcx)
	movl	%edx, -80(%rcx)
.Ltmp5757:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 316(%rsp,%rax)
	je	.LBB40_53
	.loc	1 0 24 is_stmt 0
.Ltmp5758:
	.p2align	4
.LBB40_19:
	.loc	1 1392 24
	cmpl	$1, 324(%rsp,%rax)
	jne	.LBB40_20
.LBB40_59:
	.loc	1 1392 29
	vmovd	328(%rsp,%rax), %xmm0
.Ltmp5759:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -56(%rcx)
	.loc	36 81 48
	vmovd	-60(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5760:
	.loc	36 112 9
	jg	.LBB40_63
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_63
	negl	%edx
	jo	.LBB40_63
.Ltmp5761:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -60(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -52(%rcx)
	movl	%edx, -48(%rcx)
.Ltmp5762:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 332(%rsp,%rax)
	je	.LBB40_65
	.loc	1 0 24 is_stmt 0
.Ltmp5763:
	.p2align	4
.LBB40_21:
	.loc	1 1392 24
	cmpl	$1, 340(%rsp,%rax)
	jne	.LBB40_22
.LBB40_71:
	.loc	1 1392 29
	vmovd	344(%rsp,%rax), %xmm0
.Ltmp5764:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -24(%rcx)
	.loc	36 81 48
	vmovd	-28(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5765:
	.loc	36 112 9
	jg	.LBB40_75
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_75
	negl	%edx
	jo	.LBB40_75
.Ltmp5766:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -28(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -20(%rcx)
	movl	%edx, -16(%rcx)
.Ltmp5767:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 348(%rsp,%rax)
	jne	.LBB40_12
	jmp	.LBB40_77
	.loc	1 0 24 is_stmt 0
.Ltmp5768:
	.p2align	4
.LBB40_14:
	.loc	1 1392 24
	cmpl	$1, 284(%rsp,%rax)
	jne	.LBB40_15
.LBB40_29:
	.loc	1 1392 29
	vmovd	288(%rsp,%rax), %xmm0
.Ltmp5769:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -136(%rcx)
	.loc	36 81 48
	vmovd	-140(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5770:
	.loc	36 112 9
	jg	.LBB40_33
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_33
	negl	%edx
	jo	.LBB40_33
.Ltmp5771:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -140(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -132(%rcx)
	movl	%edx, -128(%rcx)
.Ltmp5772:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 292(%rsp,%rax)
	jne	.LBB40_16
	jmp	.LBB40_35
	.loc	1 0 24 is_stmt 0
.Ltmp5773:
	.p2align	4
.LBB40_27:
.Ltmp5774:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -148(%rcx)
	movl	%edx, -144(%rcx)
.Ltmp5775:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 284(%rsp,%rax)
	jne	.LBB40_15
	jmp	.LBB40_29
	.loc	1 0 24 is_stmt 0
.Ltmp5776:
	.p2align	4
.LBB40_39:
.Ltmp5777:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -116(%rcx)
	movl	%edx, -112(%rcx)
.Ltmp5778:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 300(%rsp,%rax)
	jne	.LBB40_17
	jmp	.LBB40_41
	.loc	1 0 24 is_stmt 0
.Ltmp5779:
	.p2align	4
.LBB40_51:
.Ltmp5780:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -84(%rcx)
	movl	%edx, -80(%rcx)
.Ltmp5781:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 316(%rsp,%rax)
	jne	.LBB40_19
	jmp	.LBB40_53
	.loc	1 0 24 is_stmt 0
.Ltmp5782:
	.p2align	4
.LBB40_63:
.Ltmp5783:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -52(%rcx)
	movl	%edx, -48(%rcx)
.Ltmp5784:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 332(%rsp,%rax)
	jne	.LBB40_21
	jmp	.LBB40_65
	.loc	1 0 24 is_stmt 0
.Ltmp5785:
	.p2align	4
.LBB40_75:
.Ltmp5786:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -20(%rcx)
	movl	%edx, -16(%rcx)
.Ltmp5787:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 348(%rsp,%rax)
	jne	.LBB40_12
	jmp	.LBB40_77
	.loc	1 0 24 is_stmt 0
.Ltmp5788:
	.p2align	4
.LBB40_33:
.Ltmp5789:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -132(%rcx)
	movl	%edx, -128(%rcx)
.Ltmp5790:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 292(%rsp,%rax)
	je	.LBB40_35
	.loc	1 0 24 is_stmt 0
.Ltmp5791:
	.p2align	4
.LBB40_16:
	.loc	1 1392 24
	cmpl	$1, 300(%rsp,%rax)
	jne	.LBB40_17
.LBB40_41:
	.loc	1 1392 29
	vmovd	304(%rsp,%rax), %xmm0
.Ltmp5792:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -104(%rcx)
	.loc	36 81 48
	vmovd	-108(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5793:
	.loc	36 112 9
	jg	.LBB40_45
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_45
	negl	%edx
	jo	.LBB40_45
.Ltmp5794:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -108(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -100(%rcx)
	movl	%edx, -96(%rcx)
.Ltmp5795:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 308(%rsp,%rax)
	jne	.LBB40_18
	jmp	.LBB40_47
	.loc	1 0 24 is_stmt 0
.Ltmp5796:
	.p2align	4
.LBB40_45:
.Ltmp5797:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -100(%rcx)
	movl	%edx, -96(%rcx)
.Ltmp5798:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 308(%rsp,%rax)
	je	.LBB40_47
	.loc	1 0 24 is_stmt 0
.Ltmp5799:
	.p2align	4
.LBB40_18:
	.loc	1 1392 24
	cmpl	$1, 316(%rsp,%rax)
	jne	.LBB40_19
.LBB40_53:
	.loc	1 1392 29
	vmovd	320(%rsp,%rax), %xmm0
.Ltmp5800:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -72(%rcx)
	.loc	36 81 48
	vmovd	-76(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5801:
	.loc	36 112 9
	jg	.LBB40_57
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_57
	negl	%edx
	jo	.LBB40_57
.Ltmp5802:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -76(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -68(%rcx)
	movl	%edx, -64(%rcx)
.Ltmp5803:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 324(%rsp,%rax)
	jne	.LBB40_20
	jmp	.LBB40_59
	.loc	1 0 24 is_stmt 0
.Ltmp5804:
	.p2align	4
.LBB40_57:
.Ltmp5805:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -68(%rcx)
	movl	%edx, -64(%rcx)
.Ltmp5806:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 324(%rsp,%rax)
	je	.LBB40_59
	.loc	1 0 24 is_stmt 0
.Ltmp5807:
	.p2align	4
.LBB40_20:
	.loc	1 1392 24
	cmpl	$1, 332(%rsp,%rax)
	jne	.LBB40_21
.LBB40_65:
	.loc	1 1392 29
	vmovd	336(%rsp,%rax), %xmm0
.Ltmp5808:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -40(%rcx)
	.loc	36 81 48
	vmovd	-44(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5809:
	.loc	36 112 9
	jg	.LBB40_69
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_69
	negl	%edx
	jo	.LBB40_69
.Ltmp5810:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -44(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -36(%rcx)
	movl	%edx, -32(%rcx)
.Ltmp5811:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 340(%rsp,%rax)
	jne	.LBB40_22
	jmp	.LBB40_71
	.loc	1 0 24 is_stmt 0
.Ltmp5812:
	.p2align	4
.LBB40_69:
.Ltmp5813:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -36(%rcx)
	movl	%edx, -32(%rcx)
.Ltmp5814:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 340(%rsp,%rax)
	je	.LBB40_71
	.loc	1 0 24 is_stmt 0
.Ltmp5815:
	.p2align	4
.LBB40_22:
	.loc	1 1392 24
	cmpl	$1, 348(%rsp,%rax)
	jne	.LBB40_12
.LBB40_77:
	.loc	1 1392 29
	vmovd	352(%rsp,%rax), %xmm0
.Ltmp5816:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -8(%rcx)
	.loc	36 81 48
	vmovd	-12(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp5817:
	.loc	36 112 9
	jg	.LBB40_10
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_10
	negl	%edx
	jo	.LBB40_10
.Ltmp5818:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -12(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 89 6
	jmp	.LBB40_11
.Ltmp5819:
	.loc	36 0 6 is_stmt 0
.Ltmp5820:
	.p2align	4
.LBB40_81:
	subq	%rdi, %rsi
	leaq	(%rdi,%rdi,4), %rcx
	movq	%rdx, 1728(%rsp)
	movq	24(%rsp), %rdx
	leaq	(%rdx,%rcx,8), %r12
	movq	1728(%rsp), %rdx
	leaq	(%rsi,%rsi,4), %rcx
	leaq	(%r12,%rcx,8), %rsi
	.loc	1 1899 17 is_stmt 1
	leaq	(%rax,%rax,4), %rax
	leaq	2656(%rsp,%rax,8), %r14
	movq	2672(%rsp,%rax,8), %rbx
	movb	$1, %al
	movl	%eax, 320(%rsp)
	xorl	%r13d, %r13d
	jmp	.LBB40_82
	.loc	1 0 17 is_stmt 0
.Ltmp5821:
	.p2align	4
.LBB40_98:
.Ltmp5822:
	addq	$40, %r12
.Ltmp5823:
	.loc	38 2428 13 is_stmt 1
	incq	%rbx
	movq	$-1, %rax
	cmoveq	%rax, %rbx
.Ltmp5824:
	.loc	1 0 0 is_stmt 0
	movq	%rbx, 16(%r14)
.Ltmp5825:
	.loc	4 82 9 is_stmt 1
	incq	%r13
.Ltmp5826:
	.loc	32 1714 9
	cmpq	%rsi, %r12
.Ltmp5827:
	.loc	33 180 28
	je	.LBB40_9
.Ltmp5828:
.LBB40_82:
	.loc	1 1354 30
	movl	32(%r12), %eax
	.loc	1 1354 24 is_stmt 0
	cmpl	$1, %eax
	je	.LBB40_86
	cmpl	$2, %eax
	jne	.LBB40_98
	.loc	1 0 24
	movl	$1, %eax
	leaq	432(%rsp), %r15
.Ltmp5829:
	.loc	1 1362 29 is_stmt 1
	movl	16(%r12), %edi
	cmpq	$2, %rdi
.Ltmp5830:
	.loc	38 1050 16
	jae	.LBB40_87
.Ltmp5831:
.LBB40_85:
	.loc	38 0 16 is_stmt 0
	xorl	%ecx, %ecx
	cmpq	%rdx, %r13
.Ltmp5832:
	.loc	1 1370 25 is_stmt 1
	jb	.LBB40_88
	jmp	.LBB40_98
.Ltmp5833:
	.loc	1 0 25 is_stmt 0
.Ltmp5834:
	.p2align	4
.LBB40_86:
	xorl	%eax, %eax
	leaq	352(%rsp), %r15
	.loc	1 1362 29 is_stmt 1
	movl	16(%r12), %edi
	cmpq	$2, %rdi
.Ltmp5835:
	.loc	38 1050 16
	jb	.LBB40_85
.LBB40_87:
	.loc	38 1054 31
	leaq	-2(%rdi), %rcx
	movq	%rcx, 256(%rsp)
.Ltmp5836:
	.loc	28 1580 16
	xorl	%ecx, %ecx
	cmpl	$12, %edi
	setb	%cl
	cmpq	%rdx, %r13
.Ltmp5837:
	.loc	1 1370 25
	jae	.LBB40_98
.LBB40_88:
	cmpq	$1, %rcx
	jne	.LBB40_98
	.loc	1 1372 20
	cmpl	$1, 28(%r12)
	jne	.LBB40_98
	.loc	1 1373 20
	cmpq	%r11, (%r12)
	jne	.LBB40_98
	.loc	1 1374 20
	cmpq	%r11, 8(%r12)
	jne	.LBB40_98
	.loc	1 1375 20
	vmovd	20(%r12), %xmm0
.Ltmp5838:
	.loc	23 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5839:
	.loc	1 1375 20
	cmpl	%ecx, 24(%r12)
	jne	.LBB40_98
	.loc	1 0 20 is_stmt 0
	movq	%rsi, 64(%rsp)
	.loc	1 1376 43 is_stmt 1
	cmpl	$11, %edi
	ja	.LBB40_580
	.loc	1 0 43 is_stmt 0
	leal	(%rax,%rdi,2), %eax
	movl	%eax, 224(%rsp)
	.loc	1 1376 42
	leaq	(%rdi,%rdi,4), %rax
	leaq	.Lalloc_cc33a3b9cd8c16d253f2168b5461d31d(%rip), %rcx
	leaq	(%rcx,%rax,8), %rdi
	vmovdqa	%xmm0, 160(%rsp)
	.loc	1 1376 20
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	160(%rsp), %xmm1
	movl	224(%rsp), %r9d
	cmpl	312(%rsp), %r9d
	seta	%cl
	testb	%al, %al
	movq	16(%rsp), %r8
	movq	1824(%rsp), %r10
	movq	2016(%rsp), %r11
	vmovss	.LCPI40_0(%rip), %xmm2
	movq	1728(%rsp), %rdx
	movq	64(%rsp), %rsi
	je	.LBB40_98
	orb	320(%rsp), %cl
	testb	$1, %cl
	je	.LBB40_98
	.loc	1 0 20
	movq	256(%rsp), %rdi
.Ltmp5840:
	.loc	1 1378 45 is_stmt 1
	cmpq	$9, %rdi
	ja	.LBB40_582
.Ltmp5841:
	.loc	47 430 9
	cmpl	$0, (%r15,%rdi,8)
.Ltmp5842:
	.loc	1 1383 17
	jne	.LBB40_98
.Ltmp5843:
	.loc	31 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
	movq	256(%rsp), %rax
.Ltmp5844:
	.loc	1 1388 13
	movl	$1, (%r15,%rax,8)
	vmovss	%xmm0, 4(%r15,%rax,8)
.Ltmp5845:
	.loc	32 1714 9
	addq	$40, %r12
.Ltmp5846:
	.loc	33 180 28
	incq	%r13
	movl	$0, 320(%rsp)
	movl	%r9d, 312(%rsp)
.Ltmp5847:
	.loc	32 1714 9
	cmpq	%rsi, %r12
.Ltmp5848:
	.loc	33 180 28
	jne	.LBB40_82
	jmp	.LBB40_9
.Ltmp5849:
.LBB40_100:
	.loc	33 0 28 is_stmt 0
	movq	2240(%rsp), %rcx
	.loc	1 1904 13 is_stmt 1
	movq	(%rcx), %rax
	movq	%rax, 296(%rsp)
	movq	8(%rcx), %rax
	movq	%rax, 40(%rsp)
	.loc	1 1905 13
	movq	16(%rcx), %rax
	movq	%rax, 304(%rsp)
	movq	24(%rcx), %rax
	movq	%rax, 32(%rsp)
	.loc	1 1906 13
	movl	104(%rcx), %esi
.Ltmp5850:
	.loc	1 1142 12
	movl	5284(%r14), %ecx
	.loc	1 1142 27 is_stmt 0
	movzbl	5288(%r14), %eax
	.loc	1 1142 5
	cmpl	$1, %ecx
	movq	%rsi, 1656(%rsp)
	je	.LBB40_105
	cmpl	$2, %ecx
	jne	.LBB40_108
	testb	%al, %al
	jne	.LBB40_109
.Ltmp5851:
	.loc	1 1086 11 is_stmt 1
	testq	%rsi, %rsi
	je	.LBB40_511
.Ltmp5852:
	.loc	1 0 11 is_stmt 0
	movl	5280(%r14), %eax
	movl	%eax, 1808(%rsp)
	movq	5264(%r14), %rbx
	xorl	%r15d, %r15d
	movq	%rbx, 2016(%rsp)
	jmp	.LBB40_154
.LBB40_105:
	.loc	1 1142 5 is_stmt 1
	testb	%al, %al
	jne	.LBB40_109
.Ltmp5853:
	.loc	1 1086 11
	testq	%rsi, %rsi
	je	.LBB40_511
.Ltmp5854:
	.loc	1 0 11 is_stmt 0
	movl	5280(%r14), %eax
	movl	%eax, 2140(%rsp)
	movq	5264(%r14), %rdi
	leaq	2624(%r14), %rax
	movq	%rax, 2632(%rsp)
	xorl	%r15d, %r15d
	movq	%rdi, 2208(%rsp)
	jmp	.LBB40_273
.LBB40_108:
	.loc	1 1142 5 is_stmt 1
	testb	%al, %al
	je	.LBB40_390
.LBB40_109:
.Ltmp5855:
	.loc	1 1086 11
	testq	%rsi, %rsi
	je	.LBB40_511
	.loc	1 0 11 is_stmt 0
	movl	5280(%r14), %eax
	movl	%eax, 64(%rsp)
	movq	5264(%r14), %rax
	movq	%rax, 256(%rsp)
	leaq	2624(%r14), %rax
	movq	%rax, 160(%rsp)
	leaq	1408(%r14), %rax
	movq	%rax, 192(%rsp)
	leaq	4032(%r14), %rax
	movq	%rax, 1664(%rsp)
	leaq	2304(%r14), %rax
	movq	%rax, 1728(%rsp)
	leaq	4928(%r14), %rax
	movq	%rax, 1696(%rsp)
	xorl	%ebx, %ebx
	xorl	%r13d, %r13d
	jmp	.LBB40_113
	.p2align	4
.LBB40_111:
.Ltmp5856:
	.loc	1 1057 5 is_stmt 1
	vmovaps	3808(%rsp), %ymm0
	vmovaps	3840(%rsp), %ymm1
	vmovaps	3872(%rsp), %ymm2
	vmovaps	3904(%rsp), %ymm3
	movq	192(%rsp), %rax
	vmovaps	%ymm3, 96(%rax)
	vmovaps	%ymm2, 64(%rax)
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1058 5
	vmovaps	3936(%rsp), %ymm0
	vmovaps	3968(%rsp), %ymm1
	vmovaps	4000(%rsp), %ymm2
	vmovaps	4032(%rsp), %ymm3
	movq	1664(%rsp), %rax
	vmovaps	%ymm3, 96(%rax)
	vmovaps	%ymm2, 64(%rax)
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1059 5
	vmovaps	4064(%rsp), %ymm0
	vmovaps	4096(%rsp), %ymm1
	movq	1728(%rsp), %rax
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1060 5
	vmovdqa	4128(%rsp), %ymm0
	vmovaps	4160(%rsp), %ymm1
	movq	1696(%rsp), %rax
	vmovaps	%ymm1, 32(%rax)
	vmovdqa	%ymm0, (%rax)
	movq	48(%rsp), %r14
	.loc	1 1061 5
	movq	%r9, 5272(%r14)
	movq	1656(%rsp), %rsi
.Ltmp5857:
.LBB40_112:
	.loc	1 0 5 is_stmt 0
	movq	320(%rsp), %r13
	.loc	1 1086 11 is_stmt 1
	cmpq	%rsi, %r13
	jae	.LBB40_511
.LBB40_113:
	.loc	1 1087 42
	subq	%r13, %rsi
	.loc	1 1087 29 is_stmt 0
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E12plan_segmentB5_
	movb	%dl, 1760(%rsp)
	movq	%rax, %r15
.Ltmp5858:
	.loc	1 1176 33 is_stmt 1
	vmovss	(%r14), %xmm0
.Ltmp5859:
	.loc	1 1089 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp5860:
	.loc	1 1176 33
	vmovss	160(%r14), %xmm0
.Ltmp5861:
	.loc	1 1089 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp5862:
	.loc	1 1176 33
	vmovss	320(%r14), %xmm0
.Ltmp5863:
	.loc	1 1089 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp5864:
	.loc	1 1176 33
	vmovss	480(%r14), %xmm0
.Ltmp5865:
	.loc	1 1089 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp5866:
	.loc	1 1176 33
	vmovss	640(%r14), %xmm0
.Ltmp5867:
	.loc	1 1089 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp5868:
	.loc	1 1176 33
	vmovss	800(%r14), %xmm0
.Ltmp5869:
	.loc	1 1089 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp5870:
	.loc	1 1176 33
	vmovss	960(%r14), %xmm0
.Ltmp5871:
	.loc	1 1089 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp5872:
	.loc	1 1176 33
	vmovss	1120(%r14), %xmm0
.Ltmp5873:
	.loc	1 1089 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp5874:
	.loc	1 1176 33
	vmovss	16(%r14), %xmm0
.Ltmp5875:
	.loc	1 1089 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp5876:
	.loc	1 1176 33
	vmovss	176(%r14), %xmm0
.Ltmp5877:
	.loc	1 1089 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp5878:
	.loc	1 1176 33
	vmovss	336(%r14), %xmm0
.Ltmp5879:
	.loc	1 1089 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp5880:
	.loc	1 1176 33
	vmovss	496(%r14), %xmm0
.Ltmp5881:
	.loc	1 1089 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp5882:
	.loc	1 1176 33
	vmovss	656(%r14), %xmm0
.Ltmp5883:
	.loc	1 1089 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp5884:
	.loc	1 1176 33
	vmovss	816(%r14), %xmm0
.Ltmp5885:
	.loc	1 1089 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp5886:
	.loc	1 1176 33
	vmovss	976(%r14), %xmm0
.Ltmp5887:
	.loc	1 1089 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp5888:
	.loc	1 1176 33
	vmovss	1136(%r14), %xmm0
.Ltmp5889:
	.loc	1 1089 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp5890:
	.loc	1 1176 33
	vmovss	32(%r14), %xmm0
.Ltmp5891:
	.loc	1 1089 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp5892:
	.loc	1 1176 33
	vmovss	192(%r14), %xmm0
.Ltmp5893:
	.loc	1 1089 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp5894:
	.loc	1 1176 33
	vmovss	352(%r14), %xmm0
.Ltmp5895:
	.loc	1 1089 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp5896:
	.loc	1 1176 33
	vmovss	512(%r14), %xmm0
.Ltmp5897:
	.loc	1 1089 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp5898:
	.loc	1 1176 33
	vmovss	672(%r14), %xmm0
.Ltmp5899:
	.loc	1 1089 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp5900:
	.loc	1 1176 33
	vmovss	832(%r14), %xmm0
.Ltmp5901:
	.loc	1 1089 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp5902:
	.loc	1 1176 33
	vmovss	992(%r14), %xmm0
.Ltmp5903:
	.loc	1 1089 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp5904:
	.loc	1 1176 33
	vmovss	1152(%r14), %xmm0
.Ltmp5905:
	.loc	1 1089 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp5906:
	.loc	1 1176 33
	vmovss	48(%r14), %xmm0
.Ltmp5907:
	.loc	1 1089 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp5908:
	.loc	1 1176 33
	vmovss	208(%r14), %xmm0
.Ltmp5909:
	.loc	1 1089 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp5910:
	.loc	1 1176 33
	vmovss	368(%r14), %xmm0
.Ltmp5911:
	.loc	1 1089 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp5912:
	.loc	1 1176 33
	vmovss	528(%r14), %xmm0
.Ltmp5913:
	.loc	1 1089 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp5914:
	.loc	1 1176 33
	vmovss	688(%r14), %xmm0
.Ltmp5915:
	.loc	1 1089 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp5916:
	.loc	1 1176 33
	vmovss	848(%r14), %xmm0
.Ltmp5917:
	.loc	1 1089 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp5918:
	.loc	1 1176 33
	vmovss	1008(%r14), %xmm0
.Ltmp5919:
	.loc	1 1089 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp5920:
	.loc	1 1176 33
	vmovss	1168(%r14), %xmm0
.Ltmp5921:
	.loc	1 1089 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp5922:
	.loc	1 1176 33
	vmovss	64(%r14), %xmm0
.Ltmp5923:
	.loc	1 1089 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp5924:
	.loc	1 1176 33
	vmovss	224(%r14), %xmm0
.Ltmp5925:
	.loc	1 1089 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp5926:
	.loc	1 1176 33
	vmovss	384(%r14), %xmm0
.Ltmp5927:
	.loc	1 1089 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp5928:
	.loc	1 1176 33
	vmovss	544(%r14), %xmm0
.Ltmp5929:
	.loc	1 1089 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp5930:
	.loc	1 1176 33
	vmovss	704(%r14), %xmm0
.Ltmp5931:
	.loc	1 1089 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp5932:
	.loc	1 1176 33
	vmovss	864(%r14), %xmm0
.Ltmp5933:
	.loc	1 1089 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp5934:
	.loc	1 1176 33
	vmovss	1024(%r14), %xmm0
.Ltmp5935:
	.loc	1 1089 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp5936:
	.loc	1 1176 33
	vmovss	1184(%r14), %xmm0
.Ltmp5937:
	.loc	1 1089 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp5938:
	.loc	1 1176 33
	vmovss	80(%r14), %xmm0
.Ltmp5939:
	.loc	1 1089 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp5940:
	.loc	1 1176 33
	vmovss	240(%r14), %xmm0
.Ltmp5941:
	.loc	1 1089 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp5942:
	.loc	1 1176 33
	vmovss	400(%r14), %xmm0
.Ltmp5943:
	.loc	1 1089 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp5944:
	.loc	1 1176 33
	vmovss	560(%r14), %xmm0
.Ltmp5945:
	.loc	1 1089 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp5946:
	.loc	1 1176 33
	vmovss	720(%r14), %xmm0
.Ltmp5947:
	.loc	1 1089 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp5948:
	.loc	1 1176 33
	vmovss	880(%r14), %xmm0
.Ltmp5949:
	.loc	1 1089 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp5950:
	.loc	1 1176 33
	vmovss	1040(%r14), %xmm0
.Ltmp5951:
	.loc	1 1089 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp5952:
	.loc	1 1176 33
	vmovss	1200(%r14), %xmm0
.Ltmp5953:
	.loc	1 1089 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp5954:
	.loc	1 1176 33
	vmovss	96(%r14), %xmm0
.Ltmp5955:
	.loc	1 1089 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp5956:
	.loc	1 1176 33
	vmovss	256(%r14), %xmm0
.Ltmp5957:
	.loc	1 1089 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp5958:
	.loc	1 1176 33
	vmovss	416(%r14), %xmm0
.Ltmp5959:
	.loc	1 1089 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp5960:
	.loc	1 1176 33
	vmovss	576(%r14), %xmm0
.Ltmp5961:
	.loc	1 1089 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp5962:
	.loc	1 1176 33
	vmovss	736(%r14), %xmm0
.Ltmp5963:
	.loc	1 1089 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp5964:
	.loc	1 1176 33
	vmovss	896(%r14), %xmm0
.Ltmp5965:
	.loc	1 1089 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp5966:
	.loc	1 1176 33
	vmovss	1056(%r14), %xmm0
.Ltmp5967:
	.loc	1 1089 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp5968:
	.loc	1 1176 33
	vmovss	1216(%r14), %xmm0
.Ltmp5969:
	.loc	1 1089 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp5970:
	.loc	1 1176 33
	vmovss	112(%r14), %xmm0
.Ltmp5971:
	.loc	1 1089 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp5972:
	.loc	1 1176 33
	vmovss	272(%r14), %xmm0
.Ltmp5973:
	.loc	1 1089 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp5974:
	.loc	1 1176 33
	vmovss	432(%r14), %xmm0
.Ltmp5975:
	.loc	1 1089 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp5976:
	.loc	1 1176 33
	vmovss	592(%r14), %xmm0
.Ltmp5977:
	.loc	1 1089 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp5978:
	.loc	1 1176 33
	vmovss	752(%r14), %xmm0
.Ltmp5979:
	.loc	1 1089 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp5980:
	.loc	1 1176 33
	vmovss	912(%r14), %xmm0
.Ltmp5981:
	.loc	1 1089 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp5982:
	.loc	1 1176 33
	vmovss	1072(%r14), %xmm0
.Ltmp5983:
	.loc	1 1089 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp5984:
	.loc	1 1176 33
	vmovss	1232(%r14), %xmm0
.Ltmp5985:
	.loc	1 1089 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp5986:
	.loc	1 1176 33
	vmovss	128(%r14), %xmm0
.Ltmp5987:
	.loc	1 1089 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp5988:
	.loc	1 1176 33
	vmovss	288(%r14), %xmm0
.Ltmp5989:
	.loc	1 1089 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp5990:
	.loc	1 1176 33
	vmovss	448(%r14), %xmm0
.Ltmp5991:
	.loc	1 1089 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp5992:
	.loc	1 1176 33
	vmovss	608(%r14), %xmm0
.Ltmp5993:
	.loc	1 1089 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp5994:
	.loc	1 1176 33
	vmovss	768(%r14), %xmm0
.Ltmp5995:
	.loc	1 1089 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp5996:
	.loc	1 1176 33
	vmovss	928(%r14), %xmm0
.Ltmp5997:
	.loc	1 1089 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp5998:
	.loc	1 1176 33
	vmovss	1088(%r14), %xmm0
.Ltmp5999:
	.loc	1 1089 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp6000:
	.loc	1 1176 33
	vmovss	1248(%r14), %xmm0
.Ltmp6001:
	.loc	1 1089 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp6002:
	.loc	1 1176 33
	vmovss	144(%r14), %xmm0
.Ltmp6003:
	.loc	1 1089 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp6004:
	.loc	1 1176 33
	vmovss	304(%r14), %xmm0
.Ltmp6005:
	.loc	1 1089 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp6006:
	.loc	1 1176 33
	vmovss	464(%r14), %xmm0
.Ltmp6007:
	.loc	1 1089 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp6008:
	.loc	1 1176 33
	vmovss	624(%r14), %xmm0
.Ltmp6009:
	.loc	1 1089 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp6010:
	.loc	1 1176 33
	vmovss	784(%r14), %xmm0
.Ltmp6011:
	.loc	1 1089 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp6012:
	.loc	1 1176 33
	vmovss	944(%r14), %xmm0
.Ltmp6013:
	.loc	1 1089 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp6014:
	.loc	1 1176 33
	vmovss	1104(%r14), %xmm0
.Ltmp6015:
	.loc	1 1089 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp6016:
	.loc	1 1176 33
	vmovss	1264(%r14), %xmm0
.Ltmp6017:
	.loc	1 1089 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp6018:
	.loc	1 1177 32
	vmovss	8(%r14), %xmm0
.Ltmp6019:
	.loc	1 1089 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp6020:
	.loc	1 1177 32
	vmovss	168(%r14), %xmm0
.Ltmp6021:
	.loc	1 1089 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp6022:
	.loc	1 1177 32
	vmovss	328(%r14), %xmm0
.Ltmp6023:
	.loc	1 1089 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp6024:
	.loc	1 1177 32
	vmovss	488(%r14), %xmm0
.Ltmp6025:
	.loc	1 1089 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp6026:
	.loc	1 1177 32
	vmovss	648(%r14), %xmm0
.Ltmp6027:
	.loc	1 1089 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp6028:
	.loc	1 1177 32
	vmovss	808(%r14), %xmm0
.Ltmp6029:
	.loc	1 1089 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp6030:
	.loc	1 1177 32
	vmovss	968(%r14), %xmm0
.Ltmp6031:
	.loc	1 1089 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp6032:
	.loc	1 1177 32
	vmovss	1128(%r14), %xmm0
.Ltmp6033:
	.loc	1 1089 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp6034:
	.loc	1 1177 32
	vmovss	24(%r14), %xmm0
.Ltmp6035:
	.loc	1 1089 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp6036:
	.loc	1 1177 32
	vmovss	184(%r14), %xmm0
.Ltmp6037:
	.loc	1 1089 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp6038:
	.loc	1 1177 32
	vmovss	344(%r14), %xmm0
.Ltmp6039:
	.loc	1 1089 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp6040:
	.loc	1 1177 32
	vmovss	504(%r14), %xmm0
.Ltmp6041:
	.loc	1 1089 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp6042:
	.loc	1 1177 32
	vmovss	664(%r14), %xmm0
.Ltmp6043:
	.loc	1 1089 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp6044:
	.loc	1 1177 32
	vmovss	824(%r14), %xmm0
.Ltmp6045:
	.loc	1 1089 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp6046:
	.loc	1 1177 32
	vmovss	984(%r14), %xmm0
.Ltmp6047:
	.loc	1 1089 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp6048:
	.loc	1 1177 32
	vmovss	1144(%r14), %xmm0
.Ltmp6049:
	.loc	1 1089 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp6050:
	.loc	1 1177 32
	vmovss	40(%r14), %xmm0
.Ltmp6051:
	.loc	1 1089 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp6052:
	.loc	1 1177 32
	vmovss	200(%r14), %xmm0
.Ltmp6053:
	.loc	1 1089 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp6054:
	.loc	1 1177 32
	vmovss	360(%r14), %xmm0
.Ltmp6055:
	.loc	1 1089 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp6056:
	.loc	1 1177 32
	vmovss	520(%r14), %xmm0
.Ltmp6057:
	.loc	1 1089 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp6058:
	.loc	1 1177 32
	vmovss	680(%r14), %xmm0
.Ltmp6059:
	.loc	1 1089 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp6060:
	.loc	1 1177 32
	vmovss	840(%r14), %xmm0
.Ltmp6061:
	.loc	1 1089 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp6062:
	.loc	1 1177 32
	vmovss	1000(%r14), %xmm0
.Ltmp6063:
	.loc	1 1089 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp6064:
	.loc	1 1177 32
	vmovss	1160(%r14), %xmm0
.Ltmp6065:
	.loc	1 1089 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp6066:
	.loc	1 1177 32
	vmovss	56(%r14), %xmm0
.Ltmp6067:
	.loc	1 1089 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp6068:
	.loc	1 1177 32
	vmovss	216(%r14), %xmm0
.Ltmp6069:
	.loc	1 1089 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp6070:
	.loc	1 1177 32
	vmovss	376(%r14), %xmm0
.Ltmp6071:
	.loc	1 1089 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp6072:
	.loc	1 1177 32
	vmovss	536(%r14), %xmm0
.Ltmp6073:
	.loc	1 1089 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp6074:
	.loc	1 1177 32
	vmovss	696(%r14), %xmm0
.Ltmp6075:
	.loc	1 1089 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp6076:
	.loc	1 1177 32
	vmovss	856(%r14), %xmm0
.Ltmp6077:
	.loc	1 1089 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp6078:
	.loc	1 1177 32
	vmovss	1016(%r14), %xmm0
.Ltmp6079:
	.loc	1 1089 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp6080:
	.loc	1 1177 32
	vmovss	1176(%r14), %xmm0
.Ltmp6081:
	.loc	1 1089 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp6082:
	.loc	1 1177 32
	vmovss	72(%r14), %xmm0
.Ltmp6083:
	.loc	1 1089 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp6084:
	.loc	1 1177 32
	vmovss	232(%r14), %xmm0
.Ltmp6085:
	.loc	1 1089 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp6086:
	.loc	1 1177 32
	vmovss	392(%r14), %xmm0
.Ltmp6087:
	.loc	1 1089 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp6088:
	.loc	1 1177 32
	vmovss	552(%r14), %xmm0
.Ltmp6089:
	.loc	1 1089 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp6090:
	.loc	1 1177 32
	vmovss	712(%r14), %xmm0
.Ltmp6091:
	.loc	1 1089 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp6092:
	.loc	1 1177 32
	vmovss	872(%r14), %xmm0
.Ltmp6093:
	.loc	1 1089 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp6094:
	.loc	1 1177 32
	vmovss	1032(%r14), %xmm0
.Ltmp6095:
	.loc	1 1089 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp6096:
	.loc	1 1177 32
	vmovss	1192(%r14), %xmm0
.Ltmp6097:
	.loc	1 1089 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp6098:
	.loc	1 1177 32
	vmovss	88(%r14), %xmm0
.Ltmp6099:
	.loc	1 1089 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp6100:
	.loc	1 1177 32
	vmovss	248(%r14), %xmm0
.Ltmp6101:
	.loc	1 1089 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp6102:
	.loc	1 1177 32
	vmovss	408(%r14), %xmm0
.Ltmp6103:
	.loc	1 1089 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp6104:
	.loc	1 1177 32
	vmovss	568(%r14), %xmm0
.Ltmp6105:
	.loc	1 1089 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp6106:
	.loc	1 1177 32
	vmovss	728(%r14), %xmm0
.Ltmp6107:
	.loc	1 1089 28
	vmovss	%xmm0, 848(%rsp)
.Ltmp6108:
	.loc	1 1177 32
	vmovss	888(%r14), %xmm0
.Ltmp6109:
	.loc	1 1089 28
	vmovss	%xmm0, 852(%rsp)
.Ltmp6110:
	.loc	1 1177 32
	vmovss	1048(%r14), %xmm0
.Ltmp6111:
	.loc	1 1089 28
	vmovss	%xmm0, 856(%rsp)
.Ltmp6112:
	.loc	1 1177 32
	vmovss	1208(%r14), %xmm0
.Ltmp6113:
	.loc	1 1089 28
	vmovss	%xmm0, 860(%rsp)
.Ltmp6114:
	.loc	1 1177 32
	vmovss	104(%r14), %xmm0
.Ltmp6115:
	.loc	1 1089 28
	vmovss	%xmm0, 864(%rsp)
.Ltmp6116:
	.loc	1 1177 32
	vmovss	264(%r14), %xmm0
.Ltmp6117:
	.loc	1 1089 28
	vmovss	%xmm0, 868(%rsp)
.Ltmp6118:
	.loc	1 1177 32
	vmovss	424(%r14), %xmm0
.Ltmp6119:
	.loc	1 1089 28
	vmovss	%xmm0, 872(%rsp)
.Ltmp6120:
	.loc	1 1177 32
	vmovss	584(%r14), %xmm0
.Ltmp6121:
	.loc	1 1089 28
	vmovss	%xmm0, 876(%rsp)
.Ltmp6122:
	.loc	1 1177 32
	vmovss	744(%r14), %xmm0
.Ltmp6123:
	.loc	1 1089 28
	vmovss	%xmm0, 880(%rsp)
.Ltmp6124:
	.loc	1 1177 32
	vmovss	904(%r14), %xmm0
.Ltmp6125:
	.loc	1 1089 28
	vmovss	%xmm0, 884(%rsp)
.Ltmp6126:
	.loc	1 1177 32
	vmovss	1064(%r14), %xmm0
.Ltmp6127:
	.loc	1 1089 28
	vmovss	%xmm0, 888(%rsp)
.Ltmp6128:
	.loc	1 1177 32
	vmovss	1224(%r14), %xmm0
.Ltmp6129:
	.loc	1 1089 28
	vmovss	%xmm0, 892(%rsp)
.Ltmp6130:
	.loc	1 1177 32
	vmovss	120(%r14), %xmm0
.Ltmp6131:
	.loc	1 1089 28
	vmovss	%xmm0, 896(%rsp)
.Ltmp6132:
	.loc	1 1177 32
	vmovss	280(%r14), %xmm0
.Ltmp6133:
	.loc	1 1089 28
	vmovss	%xmm0, 900(%rsp)
.Ltmp6134:
	.loc	1 1177 32
	vmovss	440(%r14), %xmm0
.Ltmp6135:
	.loc	1 1089 28
	vmovss	%xmm0, 904(%rsp)
.Ltmp6136:
	.loc	1 1177 32
	vmovss	600(%r14), %xmm0
.Ltmp6137:
	.loc	1 1089 28
	vmovss	%xmm0, 908(%rsp)
.Ltmp6138:
	.loc	1 1177 32
	vmovss	760(%r14), %xmm0
.Ltmp6139:
	.loc	1 1089 28
	vmovss	%xmm0, 912(%rsp)
.Ltmp6140:
	.loc	1 1177 32
	vmovss	920(%r14), %xmm0
.Ltmp6141:
	.loc	1 1089 28
	vmovss	%xmm0, 916(%rsp)
.Ltmp6142:
	.loc	1 1177 32
	vmovss	1080(%r14), %xmm0
.Ltmp6143:
	.loc	1 1089 28
	vmovss	%xmm0, 920(%rsp)
.Ltmp6144:
	.loc	1 1177 32
	vmovss	1240(%r14), %xmm0
.Ltmp6145:
	.loc	1 1089 28
	vmovss	%xmm0, 924(%rsp)
.Ltmp6146:
	.loc	1 1177 32
	vmovss	136(%r14), %xmm0
.Ltmp6147:
	.loc	1 1089 28
	vmovss	%xmm0, 928(%rsp)
.Ltmp6148:
	.loc	1 1177 32
	vmovss	296(%r14), %xmm0
.Ltmp6149:
	.loc	1 1089 28
	vmovss	%xmm0, 932(%rsp)
.Ltmp6150:
	.loc	1 1177 32
	vmovss	456(%r14), %xmm0
.Ltmp6151:
	.loc	1 1089 28
	vmovss	%xmm0, 936(%rsp)
.Ltmp6152:
	.loc	1 1177 32
	vmovss	616(%r14), %xmm0
.Ltmp6153:
	.loc	1 1089 28
	vmovss	%xmm0, 940(%rsp)
.Ltmp6154:
	.loc	1 1177 32
	vmovss	776(%r14), %xmm0
.Ltmp6155:
	.loc	1 1089 28
	vmovss	%xmm0, 944(%rsp)
.Ltmp6156:
	.loc	1 1177 32
	vmovss	936(%r14), %xmm0
.Ltmp6157:
	.loc	1 1089 28
	vmovss	%xmm0, 948(%rsp)
.Ltmp6158:
	.loc	1 1177 32
	vmovss	1096(%r14), %xmm0
.Ltmp6159:
	.loc	1 1089 28
	vmovss	%xmm0, 952(%rsp)
.Ltmp6160:
	.loc	1 1177 32
	vmovss	1256(%r14), %xmm0
.Ltmp6161:
	.loc	1 1089 28
	vmovss	%xmm0, 956(%rsp)
.Ltmp6162:
	.loc	1 1177 32
	vmovss	152(%r14), %xmm0
.Ltmp6163:
	.loc	1 1089 28
	vmovss	%xmm0, 960(%rsp)
.Ltmp6164:
	.loc	1 1177 32
	vmovss	312(%r14), %xmm0
.Ltmp6165:
	.loc	1 1089 28
	vmovss	%xmm0, 964(%rsp)
.Ltmp6166:
	.loc	1 1177 32
	vmovss	472(%r14), %xmm0
.Ltmp6167:
	.loc	1 1089 28
	vmovss	%xmm0, 968(%rsp)
.Ltmp6168:
	.loc	1 1177 32
	vmovss	632(%r14), %xmm0
.Ltmp6169:
	.loc	1 1089 28
	vmovss	%xmm0, 972(%rsp)
.Ltmp6170:
	.loc	1 1177 32
	vmovss	792(%r14), %xmm0
.Ltmp6171:
	.loc	1 1089 28
	vmovss	%xmm0, 976(%rsp)
.Ltmp6172:
	.loc	1 1177 32
	vmovss	952(%r14), %xmm0
.Ltmp6173:
	.loc	1 1089 28
	vmovss	%xmm0, 980(%rsp)
.Ltmp6174:
	.loc	1 1177 32
	vmovss	1112(%r14), %xmm0
.Ltmp6175:
	.loc	1 1089 28
	vmovss	%xmm0, 984(%rsp)
.Ltmp6176:
	.loc	1 1177 32
	vmovss	1272(%r14), %xmm0
.Ltmp6177:
	.loc	1 1089 28
	vmovss	%xmm0, 988(%rsp)
.Ltmp6178:
	.loc	1 1176 33
	vmovss	2624(%r14), %xmm0
.Ltmp6179:
	.loc	1 1089 28
	vmovss	%xmm0, 992(%rsp)
.Ltmp6180:
	.loc	1 1176 33
	vmovss	2784(%r14), %xmm0
.Ltmp6181:
	.loc	1 1089 28
	vmovss	%xmm0, 996(%rsp)
.Ltmp6182:
	.loc	1 1176 33
	vmovss	2944(%r14), %xmm0
.Ltmp6183:
	.loc	1 1089 28
	vmovss	%xmm0, 1000(%rsp)
.Ltmp6184:
	.loc	1 1176 33
	vmovss	3104(%r14), %xmm0
.Ltmp6185:
	.loc	1 1089 28
	vmovss	%xmm0, 1004(%rsp)
.Ltmp6186:
	.loc	1 1176 33
	vmovss	3264(%r14), %xmm0
.Ltmp6187:
	.loc	1 1089 28
	vmovss	%xmm0, 1008(%rsp)
.Ltmp6188:
	.loc	1 1176 33
	vmovss	3424(%r14), %xmm0
.Ltmp6189:
	.loc	1 1089 28
	vmovss	%xmm0, 1012(%rsp)
.Ltmp6190:
	.loc	1 1176 33
	vmovss	3584(%r14), %xmm0
.Ltmp6191:
	.loc	1 1089 28
	vmovss	%xmm0, 1016(%rsp)
.Ltmp6192:
	.loc	1 1176 33
	vmovss	3744(%r14), %xmm0
.Ltmp6193:
	.loc	1 1089 28
	vmovss	%xmm0, 1020(%rsp)
.Ltmp6194:
	.loc	1 1176 33
	vmovss	2640(%r14), %xmm0
.Ltmp6195:
	.loc	1 1089 28
	vmovss	%xmm0, 1024(%rsp)
.Ltmp6196:
	.loc	1 1176 33
	vmovss	2800(%r14), %xmm0
.Ltmp6197:
	.loc	1 1089 28
	vmovss	%xmm0, 1028(%rsp)
.Ltmp6198:
	.loc	1 1176 33
	vmovss	2960(%r14), %xmm0
.Ltmp6199:
	.loc	1 1089 28
	vmovss	%xmm0, 1032(%rsp)
.Ltmp6200:
	.loc	1 1176 33
	vmovss	3120(%r14), %xmm0
.Ltmp6201:
	.loc	1 1089 28
	vmovss	%xmm0, 1036(%rsp)
.Ltmp6202:
	.loc	1 1176 33
	vmovss	3280(%r14), %xmm0
.Ltmp6203:
	.loc	1 1089 28
	vmovss	%xmm0, 1040(%rsp)
.Ltmp6204:
	.loc	1 1176 33
	vmovss	3440(%r14), %xmm0
.Ltmp6205:
	.loc	1 1089 28
	vmovss	%xmm0, 1044(%rsp)
.Ltmp6206:
	.loc	1 1176 33
	vmovss	3600(%r14), %xmm0
.Ltmp6207:
	.loc	1 1089 28
	vmovss	%xmm0, 1048(%rsp)
.Ltmp6208:
	.loc	1 1176 33
	vmovss	3760(%r14), %xmm0
.Ltmp6209:
	.loc	1 1089 28
	vmovss	%xmm0, 1052(%rsp)
.Ltmp6210:
	.loc	1 1176 33
	vmovss	2656(%r14), %xmm0
.Ltmp6211:
	.loc	1 1089 28
	vmovss	%xmm0, 1056(%rsp)
.Ltmp6212:
	.loc	1 1176 33
	vmovss	2816(%r14), %xmm0
.Ltmp6213:
	.loc	1 1089 28
	vmovss	%xmm0, 1060(%rsp)
.Ltmp6214:
	.loc	1 1176 33
	vmovss	2976(%r14), %xmm0
.Ltmp6215:
	.loc	1 1089 28
	vmovss	%xmm0, 1064(%rsp)
.Ltmp6216:
	.loc	1 1176 33
	vmovss	3136(%r14), %xmm0
.Ltmp6217:
	.loc	1 1089 28
	vmovss	%xmm0, 1068(%rsp)
.Ltmp6218:
	.loc	1 1176 33
	vmovss	3296(%r14), %xmm0
.Ltmp6219:
	.loc	1 1089 28
	vmovss	%xmm0, 1072(%rsp)
.Ltmp6220:
	.loc	1 1176 33
	vmovss	3456(%r14), %xmm0
.Ltmp6221:
	.loc	1 1089 28
	vmovss	%xmm0, 1076(%rsp)
.Ltmp6222:
	.loc	1 1176 33
	vmovss	3616(%r14), %xmm0
.Ltmp6223:
	.loc	1 1089 28
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6224:
	.loc	1 1176 33
	vmovss	3776(%r14), %xmm0
.Ltmp6225:
	.loc	1 1089 28
	vmovss	%xmm0, 1084(%rsp)
.Ltmp6226:
	.loc	1 1176 33
	vmovss	2672(%r14), %xmm0
.Ltmp6227:
	.loc	1 1089 28
	vmovss	%xmm0, 1088(%rsp)
.Ltmp6228:
	.loc	1 1176 33
	vmovss	2832(%r14), %xmm0
.Ltmp6229:
	.loc	1 1089 28
	vmovss	%xmm0, 1092(%rsp)
.Ltmp6230:
	.loc	1 1176 33
	vmovss	2992(%r14), %xmm0
.Ltmp6231:
	.loc	1 1089 28
	vmovss	%xmm0, 1096(%rsp)
.Ltmp6232:
	.loc	1 1176 33
	vmovss	3152(%r14), %xmm0
.Ltmp6233:
	.loc	1 1089 28
	vmovss	%xmm0, 1100(%rsp)
.Ltmp6234:
	.loc	1 1176 33
	vmovss	3312(%r14), %xmm0
.Ltmp6235:
	.loc	1 1089 28
	vmovss	%xmm0, 1104(%rsp)
.Ltmp6236:
	.loc	1 1176 33
	vmovss	3472(%r14), %xmm0
.Ltmp6237:
	.loc	1 1089 28
	vmovss	%xmm0, 1108(%rsp)
.Ltmp6238:
	.loc	1 1176 33
	vmovss	3632(%r14), %xmm0
.Ltmp6239:
	.loc	1 1089 28
	vmovss	%xmm0, 1112(%rsp)
.Ltmp6240:
	.loc	1 1176 33
	vmovss	3792(%r14), %xmm0
.Ltmp6241:
	.loc	1 1089 28
	vmovss	%xmm0, 1116(%rsp)
.Ltmp6242:
	.loc	1 1176 33
	vmovss	2688(%r14), %xmm0
.Ltmp6243:
	.loc	1 1089 28
	vmovss	%xmm0, 1120(%rsp)
.Ltmp6244:
	.loc	1 1176 33
	vmovss	2848(%r14), %xmm0
.Ltmp6245:
	.loc	1 1089 28
	vmovss	%xmm0, 1124(%rsp)
.Ltmp6246:
	.loc	1 1176 33
	vmovss	3008(%r14), %xmm0
.Ltmp6247:
	.loc	1 1089 28
	vmovss	%xmm0, 1128(%rsp)
.Ltmp6248:
	.loc	1 1176 33
	vmovss	3168(%r14), %xmm0
.Ltmp6249:
	.loc	1 1089 28
	vmovss	%xmm0, 1132(%rsp)
.Ltmp6250:
	.loc	1 1176 33
	vmovss	3328(%r14), %xmm0
.Ltmp6251:
	.loc	1 1089 28
	vmovss	%xmm0, 1136(%rsp)
.Ltmp6252:
	.loc	1 1176 33
	vmovss	3488(%r14), %xmm0
.Ltmp6253:
	.loc	1 1089 28
	vmovss	%xmm0, 1140(%rsp)
.Ltmp6254:
	.loc	1 1176 33
	vmovss	3648(%r14), %xmm0
.Ltmp6255:
	.loc	1 1089 28
	vmovss	%xmm0, 1144(%rsp)
.Ltmp6256:
	.loc	1 1176 33
	vmovss	3808(%r14), %xmm0
.Ltmp6257:
	.loc	1 1089 28
	vmovss	%xmm0, 1148(%rsp)
.Ltmp6258:
	.loc	1 1176 33
	vmovss	2704(%r14), %xmm0
.Ltmp6259:
	.loc	1 1089 28
	vmovss	%xmm0, 1152(%rsp)
.Ltmp6260:
	.loc	1 1176 33
	vmovss	2864(%r14), %xmm0
.Ltmp6261:
	.loc	1 1089 28
	vmovss	%xmm0, 1156(%rsp)
.Ltmp6262:
	.loc	1 1176 33
	vmovss	3024(%r14), %xmm0
.Ltmp6263:
	.loc	1 1089 28
	vmovss	%xmm0, 1160(%rsp)
.Ltmp6264:
	.loc	1 1176 33
	vmovss	3184(%r14), %xmm0
.Ltmp6265:
	.loc	1 1089 28
	vmovss	%xmm0, 1164(%rsp)
.Ltmp6266:
	.loc	1 1176 33
	vmovss	3344(%r14), %xmm0
.Ltmp6267:
	.loc	1 1089 28
	vmovss	%xmm0, 1168(%rsp)
.Ltmp6268:
	.loc	1 1176 33
	vmovss	3504(%r14), %xmm0
.Ltmp6269:
	.loc	1 1089 28
	vmovss	%xmm0, 1172(%rsp)
.Ltmp6270:
	.loc	1 1176 33
	vmovss	3664(%r14), %xmm0
.Ltmp6271:
	.loc	1 1089 28
	vmovss	%xmm0, 1176(%rsp)
.Ltmp6272:
	.loc	1 1176 33
	vmovss	3824(%r14), %xmm0
.Ltmp6273:
	.loc	1 1089 28
	vmovss	%xmm0, 1180(%rsp)
.Ltmp6274:
	.loc	1 1176 33
	vmovss	2720(%r14), %xmm0
.Ltmp6275:
	.loc	1 1089 28
	vmovss	%xmm0, 1184(%rsp)
.Ltmp6276:
	.loc	1 1176 33
	vmovss	2880(%r14), %xmm0
.Ltmp6277:
	.loc	1 1089 28
	vmovss	%xmm0, 1188(%rsp)
.Ltmp6278:
	.loc	1 1176 33
	vmovss	3040(%r14), %xmm0
.Ltmp6279:
	.loc	1 1089 28
	vmovss	%xmm0, 1192(%rsp)
.Ltmp6280:
	.loc	1 1176 33
	vmovss	3200(%r14), %xmm0
.Ltmp6281:
	.loc	1 1089 28
	vmovss	%xmm0, 1196(%rsp)
.Ltmp6282:
	.loc	1 1176 33
	vmovss	3360(%r14), %xmm0
.Ltmp6283:
	.loc	1 1089 28
	vmovss	%xmm0, 1200(%rsp)
.Ltmp6284:
	.loc	1 1176 33
	vmovss	3520(%r14), %xmm0
.Ltmp6285:
	.loc	1 1089 28
	vmovss	%xmm0, 1204(%rsp)
.Ltmp6286:
	.loc	1 1176 33
	vmovss	3680(%r14), %xmm0
.Ltmp6287:
	.loc	1 1089 28
	vmovss	%xmm0, 1208(%rsp)
.Ltmp6288:
	.loc	1 1176 33
	vmovss	3840(%r14), %xmm0
.Ltmp6289:
	.loc	1 1089 28
	vmovss	%xmm0, 1212(%rsp)
.Ltmp6290:
	.loc	1 1176 33
	vmovss	2736(%r14), %xmm0
.Ltmp6291:
	.loc	1 1089 28
	vmovss	%xmm0, 1216(%rsp)
.Ltmp6292:
	.loc	1 1176 33
	vmovss	2896(%r14), %xmm0
.Ltmp6293:
	.loc	1 1089 28
	vmovss	%xmm0, 1220(%rsp)
.Ltmp6294:
	.loc	1 1176 33
	vmovss	3056(%r14), %xmm0
.Ltmp6295:
	.loc	1 1089 28
	vmovss	%xmm0, 1224(%rsp)
.Ltmp6296:
	.loc	1 1176 33
	vmovss	3216(%r14), %xmm0
.Ltmp6297:
	.loc	1 1089 28
	vmovss	%xmm0, 1228(%rsp)
.Ltmp6298:
	.loc	1 1176 33
	vmovss	3376(%r14), %xmm0
.Ltmp6299:
	.loc	1 1089 28
	vmovss	%xmm0, 1232(%rsp)
.Ltmp6300:
	.loc	1 1176 33
	vmovss	3536(%r14), %xmm0
.Ltmp6301:
	.loc	1 1089 28
	vmovss	%xmm0, 1236(%rsp)
.Ltmp6302:
	.loc	1 1176 33
	vmovss	3696(%r14), %xmm0
.Ltmp6303:
	.loc	1 1089 28
	vmovss	%xmm0, 1240(%rsp)
.Ltmp6304:
	.loc	1 1176 33
	vmovss	3856(%r14), %xmm0
.Ltmp6305:
	.loc	1 1089 28
	vmovss	%xmm0, 1244(%rsp)
.Ltmp6306:
	.loc	1 1176 33
	vmovss	2752(%r14), %xmm0
.Ltmp6307:
	.loc	1 1089 28
	vmovss	%xmm0, 1248(%rsp)
.Ltmp6308:
	.loc	1 1176 33
	vmovss	2912(%r14), %xmm0
.Ltmp6309:
	.loc	1 1089 28
	vmovss	%xmm0, 1252(%rsp)
.Ltmp6310:
	.loc	1 1176 33
	vmovss	3072(%r14), %xmm0
.Ltmp6311:
	.loc	1 1089 28
	vmovss	%xmm0, 1256(%rsp)
.Ltmp6312:
	.loc	1 1176 33
	vmovss	3232(%r14), %xmm0
.Ltmp6313:
	.loc	1 1089 28
	vmovss	%xmm0, 1260(%rsp)
.Ltmp6314:
	.loc	1 1176 33
	vmovss	3392(%r14), %xmm0
.Ltmp6315:
	.loc	1 1089 28
	vmovss	%xmm0, 1264(%rsp)
.Ltmp6316:
	.loc	1 1176 33
	vmovss	3552(%r14), %xmm0
.Ltmp6317:
	.loc	1 1089 28
	vmovss	%xmm0, 1268(%rsp)
.Ltmp6318:
	.loc	1 1176 33
	vmovss	3712(%r14), %xmm0
.Ltmp6319:
	.loc	1 1089 28
	vmovss	%xmm0, 1272(%rsp)
.Ltmp6320:
	.loc	1 1176 33
	vmovss	3872(%r14), %xmm0
.Ltmp6321:
	.loc	1 1089 28
	vmovss	%xmm0, 1276(%rsp)
.Ltmp6322:
	.loc	1 1176 33
	vmovss	2768(%r14), %xmm0
.Ltmp6323:
	.loc	1 1089 28
	vmovss	%xmm0, 1280(%rsp)
.Ltmp6324:
	.loc	1 1176 33
	vmovss	2928(%r14), %xmm0
.Ltmp6325:
	.loc	1 1089 28
	vmovss	%xmm0, 1284(%rsp)
.Ltmp6326:
	.loc	1 1176 33
	vmovss	3088(%r14), %xmm0
.Ltmp6327:
	.loc	1 1089 28
	vmovss	%xmm0, 1288(%rsp)
.Ltmp6328:
	.loc	1 1176 33
	vmovss	3248(%r14), %xmm0
.Ltmp6329:
	.loc	1 1089 28
	vmovss	%xmm0, 1292(%rsp)
.Ltmp6330:
	.loc	1 1176 33
	vmovss	3408(%r14), %xmm0
.Ltmp6331:
	.loc	1 1089 28
	vmovss	%xmm0, 1296(%rsp)
.Ltmp6332:
	.loc	1 1176 33
	vmovss	3568(%r14), %xmm0
.Ltmp6333:
	.loc	1 1089 28
	vmovss	%xmm0, 1300(%rsp)
.Ltmp6334:
	.loc	1 1176 33
	vmovss	3728(%r14), %xmm0
.Ltmp6335:
	.loc	1 1089 28
	vmovss	%xmm0, 1304(%rsp)
.Ltmp6336:
	.loc	1 1176 33
	vmovss	3888(%r14), %xmm0
.Ltmp6337:
	.loc	1 1089 28
	vmovss	%xmm0, 1308(%rsp)
.Ltmp6338:
	.loc	1 1177 32
	vmovss	2632(%r14), %xmm0
.Ltmp6339:
	.loc	1 1089 28
	vmovss	%xmm0, 1312(%rsp)
.Ltmp6340:
	.loc	1 1177 32
	vmovss	2792(%r14), %xmm0
.Ltmp6341:
	.loc	1 1089 28
	vmovss	%xmm0, 1316(%rsp)
.Ltmp6342:
	.loc	1 1177 32
	vmovss	2952(%r14), %xmm0
.Ltmp6343:
	.loc	1 1089 28
	vmovss	%xmm0, 1320(%rsp)
.Ltmp6344:
	.loc	1 1177 32
	vmovss	3112(%r14), %xmm0
.Ltmp6345:
	.loc	1 1089 28
	vmovss	%xmm0, 1324(%rsp)
.Ltmp6346:
	.loc	1 1177 32
	vmovss	3272(%r14), %xmm0
.Ltmp6347:
	.loc	1 1089 28
	vmovss	%xmm0, 1328(%rsp)
.Ltmp6348:
	.loc	1 1177 32
	vmovss	3432(%r14), %xmm0
.Ltmp6349:
	.loc	1 1089 28
	vmovss	%xmm0, 1332(%rsp)
.Ltmp6350:
	.loc	1 1177 32
	vmovss	3592(%r14), %xmm0
.Ltmp6351:
	.loc	1 1089 28
	vmovss	%xmm0, 1336(%rsp)
.Ltmp6352:
	.loc	1 1177 32
	vmovss	3752(%r14), %xmm0
.Ltmp6353:
	.loc	1 1089 28
	vmovss	%xmm0, 1340(%rsp)
.Ltmp6354:
	.loc	1 1177 32
	vmovss	2648(%r14), %xmm0
.Ltmp6355:
	.loc	1 1089 28
	vmovss	%xmm0, 1344(%rsp)
.Ltmp6356:
	.loc	1 1177 32
	vmovss	2808(%r14), %xmm0
.Ltmp6357:
	.loc	1 1089 28
	vmovss	%xmm0, 1348(%rsp)
.Ltmp6358:
	.loc	1 1177 32
	vmovss	2968(%r14), %xmm0
.Ltmp6359:
	.loc	1 1089 28
	vmovss	%xmm0, 1352(%rsp)
.Ltmp6360:
	.loc	1 1177 32
	vmovss	3128(%r14), %xmm0
.Ltmp6361:
	.loc	1 1089 28
	vmovss	%xmm0, 1356(%rsp)
.Ltmp6362:
	.loc	1 1177 32
	vmovss	3288(%r14), %xmm0
.Ltmp6363:
	.loc	1 1089 28
	vmovss	%xmm0, 1360(%rsp)
.Ltmp6364:
	.loc	1 1177 32
	vmovss	3448(%r14), %xmm0
.Ltmp6365:
	.loc	1 1089 28
	vmovss	%xmm0, 1364(%rsp)
.Ltmp6366:
	.loc	1 1177 32
	vmovss	3608(%r14), %xmm0
.Ltmp6367:
	.loc	1 1089 28
	vmovss	%xmm0, 1368(%rsp)
.Ltmp6368:
	.loc	1 1177 32
	vmovss	3768(%r14), %xmm0
.Ltmp6369:
	.loc	1 1089 28
	vmovss	%xmm0, 1372(%rsp)
.Ltmp6370:
	.loc	1 1177 32
	vmovss	2664(%r14), %xmm0
.Ltmp6371:
	.loc	1 1089 28
	vmovss	%xmm0, 1376(%rsp)
.Ltmp6372:
	.loc	1 1177 32
	vmovss	2824(%r14), %xmm0
.Ltmp6373:
	.loc	1 1089 28
	vmovss	%xmm0, 1380(%rsp)
.Ltmp6374:
	.loc	1 1177 32
	vmovss	2984(%r14), %xmm0
.Ltmp6375:
	.loc	1 1089 28
	vmovss	%xmm0, 1384(%rsp)
.Ltmp6376:
	.loc	1 1177 32
	vmovss	3144(%r14), %xmm0
.Ltmp6377:
	.loc	1 1089 28
	vmovss	%xmm0, 1388(%rsp)
.Ltmp6378:
	.loc	1 1177 32
	vmovss	3304(%r14), %xmm0
.Ltmp6379:
	.loc	1 1089 28
	vmovss	%xmm0, 1392(%rsp)
.Ltmp6380:
	.loc	1 1177 32
	vmovss	3464(%r14), %xmm0
.Ltmp6381:
	.loc	1 1089 28
	vmovss	%xmm0, 1396(%rsp)
.Ltmp6382:
	.loc	1 1177 32
	vmovss	3624(%r14), %xmm0
.Ltmp6383:
	.loc	1 1089 28
	vmovss	%xmm0, 1400(%rsp)
.Ltmp6384:
	.loc	1 1177 32
	vmovss	3784(%r14), %xmm0
.Ltmp6385:
	.loc	1 1089 28
	vmovss	%xmm0, 1404(%rsp)
.Ltmp6386:
	.loc	1 1177 32
	vmovss	2680(%r14), %xmm0
.Ltmp6387:
	.loc	1 1089 28
	vmovss	%xmm0, 1408(%rsp)
.Ltmp6388:
	.loc	1 1177 32
	vmovss	2840(%r14), %xmm0
.Ltmp6389:
	.loc	1 1089 28
	vmovss	%xmm0, 1412(%rsp)
.Ltmp6390:
	.loc	1 1177 32
	vmovss	3000(%r14), %xmm0
.Ltmp6391:
	.loc	1 1089 28
	vmovss	%xmm0, 1416(%rsp)
.Ltmp6392:
	.loc	1 1177 32
	vmovss	3160(%r14), %xmm0
.Ltmp6393:
	.loc	1 1089 28
	vmovss	%xmm0, 1420(%rsp)
.Ltmp6394:
	.loc	1 1177 32
	vmovss	3320(%r14), %xmm0
.Ltmp6395:
	.loc	1 1089 28
	vmovss	%xmm0, 1424(%rsp)
.Ltmp6396:
	.loc	1 1177 32
	vmovss	3480(%r14), %xmm0
.Ltmp6397:
	.loc	1 1089 28
	vmovss	%xmm0, 1428(%rsp)
.Ltmp6398:
	.loc	1 1177 32
	vmovss	3640(%r14), %xmm0
.Ltmp6399:
	.loc	1 1089 28
	vmovss	%xmm0, 1432(%rsp)
.Ltmp6400:
	.loc	1 1177 32
	vmovss	3800(%r14), %xmm0
.Ltmp6401:
	.loc	1 1089 28
	vmovss	%xmm0, 1436(%rsp)
.Ltmp6402:
	.loc	1 1177 32
	vmovss	2696(%r14), %xmm0
.Ltmp6403:
	.loc	1 1089 28
	vmovss	%xmm0, 1440(%rsp)
.Ltmp6404:
	.loc	1 1177 32
	vmovss	2856(%r14), %xmm0
.Ltmp6405:
	.loc	1 1089 28
	vmovss	%xmm0, 1444(%rsp)
.Ltmp6406:
	.loc	1 1177 32
	vmovss	3016(%r14), %xmm0
.Ltmp6407:
	.loc	1 1089 28
	vmovss	%xmm0, 1448(%rsp)
.Ltmp6408:
	.loc	1 1177 32
	vmovss	3176(%r14), %xmm0
.Ltmp6409:
	.loc	1 1089 28
	vmovss	%xmm0, 1452(%rsp)
.Ltmp6410:
	.loc	1 1177 32
	vmovss	3336(%r14), %xmm0
.Ltmp6411:
	.loc	1 1089 28
	vmovss	%xmm0, 1456(%rsp)
.Ltmp6412:
	.loc	1 1177 32
	vmovss	3496(%r14), %xmm0
.Ltmp6413:
	.loc	1 1089 28
	vmovss	%xmm0, 1460(%rsp)
.Ltmp6414:
	.loc	1 1177 32
	vmovss	3656(%r14), %xmm0
.Ltmp6415:
	.loc	1 1089 28
	vmovss	%xmm0, 1464(%rsp)
.Ltmp6416:
	.loc	1 1177 32
	vmovss	3816(%r14), %xmm0
.Ltmp6417:
	.loc	1 1089 28
	vmovss	%xmm0, 1468(%rsp)
.Ltmp6418:
	.loc	1 1177 32
	vmovss	2712(%r14), %xmm0
.Ltmp6419:
	.loc	1 1089 28
	vmovss	%xmm0, 1472(%rsp)
.Ltmp6420:
	.loc	1 1177 32
	vmovss	2872(%r14), %xmm0
.Ltmp6421:
	.loc	1 1089 28
	vmovss	%xmm0, 1476(%rsp)
.Ltmp6422:
	.loc	1 1177 32
	vmovss	3032(%r14), %xmm0
.Ltmp6423:
	.loc	1 1089 28
	vmovss	%xmm0, 1480(%rsp)
.Ltmp6424:
	.loc	1 1177 32
	vmovss	3192(%r14), %xmm0
.Ltmp6425:
	.loc	1 1089 28
	vmovss	%xmm0, 1484(%rsp)
.Ltmp6426:
	.loc	1 1177 32
	vmovss	3352(%r14), %xmm0
.Ltmp6427:
	.loc	1 1089 28
	vmovss	%xmm0, 1488(%rsp)
.Ltmp6428:
	.loc	1 1177 32
	vmovss	3512(%r14), %xmm0
.Ltmp6429:
	.loc	1 1089 28
	vmovss	%xmm0, 1492(%rsp)
.Ltmp6430:
	.loc	1 1177 32
	vmovss	3672(%r14), %xmm0
.Ltmp6431:
	.loc	1 1089 28
	vmovss	%xmm0, 1496(%rsp)
.Ltmp6432:
	.loc	1 1177 32
	vmovss	3832(%r14), %xmm0
.Ltmp6433:
	.loc	1 1089 28
	vmovss	%xmm0, 1500(%rsp)
.Ltmp6434:
	.loc	1 1177 32
	vmovss	2728(%r14), %xmm0
.Ltmp6435:
	.loc	1 1089 28
	vmovss	%xmm0, 1504(%rsp)
.Ltmp6436:
	.loc	1 1177 32
	vmovss	2888(%r14), %xmm0
.Ltmp6437:
	.loc	1 1089 28
	vmovss	%xmm0, 1508(%rsp)
.Ltmp6438:
	.loc	1 1177 32
	vmovss	3048(%r14), %xmm0
.Ltmp6439:
	.loc	1 1089 28
	vmovss	%xmm0, 1512(%rsp)
.Ltmp6440:
	.loc	1 1177 32
	vmovss	3208(%r14), %xmm0
.Ltmp6441:
	.loc	1 1089 28
	vmovss	%xmm0, 1516(%rsp)
.Ltmp6442:
	.loc	1 1177 32
	vmovss	3368(%r14), %xmm0
.Ltmp6443:
	.loc	1 1089 28
	vmovss	%xmm0, 1520(%rsp)
.Ltmp6444:
	.loc	1 1177 32
	vmovss	3528(%r14), %xmm0
.Ltmp6445:
	.loc	1 1089 28
	vmovss	%xmm0, 1524(%rsp)
.Ltmp6446:
	.loc	1 1177 32
	vmovss	3688(%r14), %xmm0
.Ltmp6447:
	.loc	1 1089 28
	vmovss	%xmm0, 1528(%rsp)
.Ltmp6448:
	.loc	1 1177 32
	vmovss	3848(%r14), %xmm0
.Ltmp6449:
	.loc	1 1089 28
	vmovss	%xmm0, 1532(%rsp)
.Ltmp6450:
	.loc	1 1177 32
	vmovss	2744(%r14), %xmm0
.Ltmp6451:
	.loc	1 1089 28
	vmovss	%xmm0, 1536(%rsp)
.Ltmp6452:
	.loc	1 1177 32
	vmovss	2904(%r14), %xmm0
.Ltmp6453:
	.loc	1 1089 28
	vmovss	%xmm0, 1540(%rsp)
.Ltmp6454:
	.loc	1 1177 32
	vmovss	3064(%r14), %xmm0
.Ltmp6455:
	.loc	1 1089 28
	vmovss	%xmm0, 1544(%rsp)
.Ltmp6456:
	.loc	1 1177 32
	vmovss	3224(%r14), %xmm0
.Ltmp6457:
	.loc	1 1089 28
	vmovss	%xmm0, 1548(%rsp)
.Ltmp6458:
	.loc	1 1177 32
	vmovss	3384(%r14), %xmm0
.Ltmp6459:
	.loc	1 1089 28
	vmovss	%xmm0, 1552(%rsp)
.Ltmp6460:
	.loc	1 1177 32
	vmovss	3544(%r14), %xmm0
.Ltmp6461:
	.loc	1 1089 28
	vmovss	%xmm0, 1556(%rsp)
.Ltmp6462:
	.loc	1 1177 32
	vmovss	3704(%r14), %xmm0
.Ltmp6463:
	.loc	1 1089 28
	vmovss	%xmm0, 1560(%rsp)
.Ltmp6464:
	.loc	1 1177 32
	vmovss	3864(%r14), %xmm0
.Ltmp6465:
	.loc	1 1089 28
	vmovss	%xmm0, 1564(%rsp)
.Ltmp6466:
	.loc	1 1177 32
	vmovss	2760(%r14), %xmm0
.Ltmp6467:
	.loc	1 1089 28
	vmovss	%xmm0, 1568(%rsp)
.Ltmp6468:
	.loc	1 1177 32
	vmovss	2920(%r14), %xmm0
.Ltmp6469:
	.loc	1 1089 28
	vmovss	%xmm0, 1572(%rsp)
.Ltmp6470:
	.loc	1 1177 32
	vmovss	3080(%r14), %xmm0
.Ltmp6471:
	.loc	1 1089 28
	vmovss	%xmm0, 1576(%rsp)
.Ltmp6472:
	.loc	1 1177 32
	vmovss	3240(%r14), %xmm0
.Ltmp6473:
	.loc	1 1089 28
	vmovss	%xmm0, 1580(%rsp)
.Ltmp6474:
	.loc	1 1177 32
	vmovss	3400(%r14), %xmm0
.Ltmp6475:
	.loc	1 1089 28
	vmovss	%xmm0, 1584(%rsp)
.Ltmp6476:
	.loc	1 1177 32
	vmovss	3560(%r14), %xmm0
.Ltmp6477:
	.loc	1 1089 28
	vmovss	%xmm0, 1588(%rsp)
.Ltmp6478:
	.loc	1 1177 32
	vmovss	3720(%r14), %xmm0
.Ltmp6479:
	.loc	1 1089 28
	vmovss	%xmm0, 1592(%rsp)
.Ltmp6480:
	.loc	1 1177 32
	vmovss	3880(%r14), %xmm0
.Ltmp6481:
	.loc	1 1089 28
	vmovss	%xmm0, 1596(%rsp)
.Ltmp6482:
	.loc	1 1177 32
	vmovss	2776(%r14), %xmm0
.Ltmp6483:
	.loc	1 1089 28
	vmovss	%xmm0, 1600(%rsp)
.Ltmp6484:
	.loc	1 1177 32
	vmovss	2936(%r14), %xmm0
.Ltmp6485:
	.loc	1 1089 28
	vmovss	%xmm0, 1604(%rsp)
.Ltmp6486:
	.loc	1 1177 32
	vmovss	3096(%r14), %xmm0
.Ltmp6487:
	.loc	1 1089 28
	vmovss	%xmm0, 1608(%rsp)
.Ltmp6488:
	.loc	1 1177 32
	vmovss	3256(%r14), %xmm0
.Ltmp6489:
	.loc	1 1089 28
	vmovss	%xmm0, 1612(%rsp)
.Ltmp6490:
	.loc	1 1177 32
	vmovss	3416(%r14), %xmm0
.Ltmp6491:
	.loc	1 1089 28
	vmovss	%xmm0, 1616(%rsp)
.Ltmp6492:
	.loc	1 1177 32
	vmovss	3576(%r14), %xmm0
.Ltmp6493:
	.loc	1 1089 28
	vmovss	%xmm0, 1620(%rsp)
.Ltmp6494:
	.loc	1 1177 32
	vmovss	3736(%r14), %xmm0
.Ltmp6495:
	.loc	1 1089 28
	vmovss	%xmm0, 1624(%rsp)
.Ltmp6496:
	.loc	1 1177 32
	vmovss	3896(%r14), %xmm0
.Ltmp6497:
	.loc	1 1089 28
	vmovss	%xmm0, 1628(%rsp)
.Ltmp6498:
	.loc	1 1091 31
	leaq	3008(%rsp), %rdi
	movq	%r14, %rsi
	movl	64(%rsp), %r12d
	movl	%r12d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	3200(%rsp), %rdi
	movq	160(%rsp), %rsi
	movl	%r12d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
.Ltmp6499:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%r13), %rax
	shlq	$3, %r13
	leaq	(,%rax,8), %rsi
	.loc	1 1094 12 is_stmt 1
	testb	$1, 1760(%rsp)
	movq	%rax, 320(%rsp)
	je	.LBB40_134
.Ltmp6500:
	.loc	38 1050 16
	cmpq	%r13, %rsi
.Ltmp6501:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_567
.Ltmp6502:
	.loc	48 451 16 is_stmt 1
	cmpq	40(%rsp), %rsi
	ja	.LBB40_567
.Ltmp6503:
	.loc	48 451 16 is_stmt 0
	cmpq	32(%rsp), %rsi
	ja	.LBB40_571
.Ltmp6504:
	.loc	48 0 16
	movq	192(%rsp), %rax
.Ltmp6505:
	.loc	1 972 27 is_stmt 1
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	64(%rax), %ymm2
	vmovaps	96(%rax), %ymm3
	vmovaps	%ymm3, 3488(%rsp)
	vmovaps	%ymm2, 3456(%rsp)
	vmovaps	%ymm1, 3424(%rsp)
	vmovaps	%ymm0, 3392(%rsp)
	movq	1664(%rsp), %rax
.Ltmp6506:
	.loc	1 973 26
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	64(%rax), %ymm2
	vmovaps	96(%rax), %ymm3
	vmovaps	%ymm3, 3616(%rsp)
	vmovaps	%ymm2, 3584(%rsp)
	vmovaps	%ymm1, 3552(%rsp)
	vmovaps	%ymm0, 3520(%rsp)
	movq	1728(%rsp), %rax
.Ltmp6507:
	.loc	1 974 25
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 3680(%rsp)
	vmovaps	%ymm0, 3648(%rsp)
	movq	1696(%rsp), %rax
.Ltmp6508:
	.loc	1 975 24
	vmovdqa	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 3744(%rsp)
	vmovdqa	%ymm0, 3712(%rsp)
.Ltmp6509:
	.loc	1 976 24
	movq	5272(%r14), %r9
.Ltmp6510:
	.loc	2 1916 50
	testq	%r15, %r15
	je	.LBB40_130
.Ltmp6511:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,8), %rdx
	movq	296(%rsp), %rax
	leaq	(%rax,%r13,4), %r10
	movq	304(%rsp), %rax
	leaq	(%rax,%r13,4), %r11
.Ltmp6512:
	.loc	3 900 12 is_stmt 1
	movq	%r15, %r14
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r14
	xorl	%edi, %edi
	xorl	%r13d, %r13d
.Ltmp6513:
	.loc	3 0 12 is_stmt 0
.Ltmp6514:
	.p2align	4
.LBB40_119:
	.loc	1 981 21 is_stmt 1
	vmovaps	352(%rsp), %ymm0
	vmovaps	384(%rsp), %ymm1
	vmovaps	416(%rsp), %ymm2
.Ltmp6515:
	.loc	14 48 14
	vaddps	672(%rsp), %ymm0, %ymm0
.Ltmp6516:
	.loc	1 980 17
	vmovaps	%ymm0, 352(%rsp)
	.loc	1 983 21
	vmovaps	992(%rsp), %ymm0
.Ltmp6517:
	.loc	14 48 14
	vaddps	1312(%rsp), %ymm0, %ymm0
.Ltmp6518:
	.loc	1 982 17
	vmovaps	%ymm0, 992(%rsp)
.Ltmp6519:
	.loc	14 48 14
	vaddps	704(%rsp), %ymm1, %ymm0
.Ltmp6520:
	.loc	1 980 17
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 983 21
	vmovaps	1024(%rsp), %ymm0
.Ltmp6521:
	.loc	14 48 14
	vaddps	1344(%rsp), %ymm0, %ymm0
.Ltmp6522:
	.loc	1 982 17
	vmovaps	%ymm0, 1024(%rsp)
.Ltmp6523:
	.loc	14 48 14
	vaddps	736(%rsp), %ymm2, %ymm0
.Ltmp6524:
	.loc	1 980 17
	vmovaps	%ymm0, 416(%rsp)
	.loc	1 983 21
	vmovaps	1056(%rsp), %ymm0
.Ltmp6525:
	.loc	14 48 14
	vaddps	1376(%rsp), %ymm0, %ymm0
.Ltmp6526:
	.loc	1 982 17
	vmovaps	%ymm0, 1056(%rsp)
	.loc	1 981 21
	vmovaps	448(%rsp), %ymm0
.Ltmp6527:
	.loc	14 48 14
	vaddps	768(%rsp), %ymm0, %ymm0
.Ltmp6528:
	.loc	1 980 17
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 983 21
	vmovaps	1088(%rsp), %ymm0
.Ltmp6529:
	.loc	14 48 14
	vaddps	1408(%rsp), %ymm0, %ymm0
.Ltmp6530:
	.loc	1 982 17
	vmovaps	%ymm0, 1088(%rsp)
	.loc	1 981 21
	vmovaps	480(%rsp), %ymm0
.Ltmp6531:
	.loc	14 48 14
	vaddps	800(%rsp), %ymm0, %ymm0
.Ltmp6532:
	.loc	1 980 17
	vmovaps	%ymm0, 480(%rsp)
	.loc	1 983 21
	vmovaps	1120(%rsp), %ymm0
.Ltmp6533:
	.loc	14 48 14
	vaddps	1440(%rsp), %ymm0, %ymm0
.Ltmp6534:
	.loc	1 982 17
	vmovaps	%ymm0, 1120(%rsp)
	.loc	1 981 21
	vmovaps	512(%rsp), %ymm0
.Ltmp6535:
	.loc	14 48 14
	vaddps	832(%rsp), %ymm0, %ymm0
.Ltmp6536:
	.loc	1 980 17
	vmovaps	%ymm0, 512(%rsp)
	.loc	1 983 21
	vmovaps	1152(%rsp), %ymm0
.Ltmp6537:
	.loc	14 48 14
	vaddps	1472(%rsp), %ymm0, %ymm0
.Ltmp6538:
	.loc	1 982 17
	vmovaps	%ymm0, 1152(%rsp)
	.loc	1 981 21
	vmovaps	544(%rsp), %ymm0
.Ltmp6539:
	.loc	14 48 14
	vaddps	864(%rsp), %ymm0, %ymm0
.Ltmp6540:
	.loc	1 980 17
	vmovaps	%ymm0, 544(%rsp)
	.loc	1 983 21
	vmovaps	1184(%rsp), %ymm0
.Ltmp6541:
	.loc	14 48 14
	vaddps	1504(%rsp), %ymm0, %ymm0
.Ltmp6542:
	.loc	1 982 17
	vmovaps	%ymm0, 1184(%rsp)
	.loc	1 981 21
	vmovaps	576(%rsp), %ymm0
.Ltmp6543:
	.loc	14 48 14
	vaddps	896(%rsp), %ymm0, %ymm0
.Ltmp6544:
	.loc	1 980 17
	vmovaps	%ymm0, 576(%rsp)
	.loc	1 983 21
	vmovaps	1216(%rsp), %ymm0
.Ltmp6545:
	.loc	14 48 14
	vaddps	1536(%rsp), %ymm0, %ymm0
.Ltmp6546:
	.loc	1 982 17
	vmovaps	%ymm0, 1216(%rsp)
	.loc	1 981 21
	vmovaps	608(%rsp), %ymm0
.Ltmp6547:
	.loc	14 48 14
	vaddps	928(%rsp), %ymm0, %ymm0
.Ltmp6548:
	.loc	1 980 17
	vmovaps	%ymm0, 608(%rsp)
	.loc	1 983 21
	vmovaps	1248(%rsp), %ymm0
.Ltmp6549:
	.loc	14 48 14
	vaddps	1568(%rsp), %ymm0, %ymm0
.Ltmp6550:
	.loc	1 982 17
	vmovaps	%ymm0, 1248(%rsp)
	.loc	1 981 21
	vmovaps	640(%rsp), %ymm0
.Ltmp6551:
	.loc	14 48 14
	vaddps	960(%rsp), %ymm0, %ymm0
.Ltmp6552:
	.loc	1 980 17
	vmovaps	%ymm0, 640(%rsp)
	.loc	1 983 21
	vmovaps	1280(%rsp), %ymm0
.Ltmp6553:
	.loc	14 48 14
	vaddps	1600(%rsp), %ymm0, %ymm0
.Ltmp6554:
	.loc	1 982 17
	vmovaps	%ymm0, 1280(%rsp)
.Ltmp6555:
	.loc	1 987 28
	leaq	1(%r9), %rax
	movq	256(%rsp), %rcx
.Ltmp6556:
	.loc	1 857 8
	cmpq	%rcx, %rax
	movl	$0, %r12d
	cmovaeq	%rcx, %r12
.Ltmp6557:
	.loc	48 568 12
	cmpq	%rdx, %rdi
	ja	.LBB40_549
.Ltmp6558:
	.loc	48 438 16
	cmpq	%r13, %r14
	je	.LBB40_547
.Ltmp6559:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r9,8), %rax
.Ltmp6560:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%r11,%rdi,4), %ymm0
	vmovdqa	%ymm0, 3776(%rsp)
	movq	48(%rsp), %rcx
.Ltmp6561:
	.loc	1 991 35
	movq	2600(%rcx), %rsi
.Ltmp6562:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_550
.Ltmp6563:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp6564:
	.loc	1 0 0 is_stmt 0
	leaq	(%r10,%rdi,4), %rcx
	movq	48(%rsp), %r8
.Ltmp6565:
	.loc	1 991 35 is_stmt 1
	movq	2592(%r8), %rsi
.Ltmp6566:
	.loc	8 551 14
	vmovdqu	(%rcx), %ymm0
	vmovdqu	%ymm0, (%rsi,%rax,4)
.Ltmp6567:
	.loc	1 992 34
	movq	5224(%r8), %rsi
.Ltmp6568:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_551
.Ltmp6569:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp6570:
	.loc	1 0 0 is_stmt 0
	negq	%r12
	addq	%r12, %r9
	incq	%r9
	leaq	(,%r9,8), %r8
	movq	48(%rsp), %r12
.Ltmp6571:
	.loc	1 992 34 is_stmt 1
	movq	5216(%r12), %rsi
.Ltmp6572:
	.loc	8 551 14
	vmovaps	3776(%rsp), %ymm0
	vmovups	%ymm0, (%rsi,%rax,4)
.Ltmp6573:
	.loc	1 993 22
	movq	2600(%r12), %rsi
.Ltmp6574:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB40_552
.Ltmp6575:
	.loc	48 438 16
	cmpq	$7, %rax
	jbe	.LBB40_546
.Ltmp6576:
	.loc	48 0 16 is_stmt 0
	movq	48(%rsp), %rsi
	.loc	1 993 22 is_stmt 1
	movq	2592(%rsi), %rax
.Ltmp6577:
	.loc	8 551 14
	vmovups	(%rax,%r8,4), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp6578:
	.loc	1 994 22
	movq	5224(%rsi), %rsi
.Ltmp6579:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB40_553
.Ltmp6580:
	.loc	48 438 16
	cmpq	$7, %rax
	jbe	.LBB40_546
.Ltmp6581:
	.loc	1 0 0 is_stmt 0
	incq	%r13
.Ltmp6582:
	leaq	(%r11,%rdi,4), %rax
	movq	48(%rsp), %rcx
.Ltmp6583:
	.loc	1 994 22 is_stmt 1
	movq	5216(%rcx), %rcx
.Ltmp6584:
	.loc	8 551 14
	vmovdqu	(%rcx,%r8,4), %ymm0
	vmovdqu	%ymm0, (%rax)
.Ltmp6585:
	.loc	2 1916 50
	addq	$8, %rdi
	cmpq	%r13, %r15
.Ltmp6586:
	.loc	3 900 12
	jne	.LBB40_119
.Ltmp6587:
.LBB40_130:
	.loc	1 1057 5
	vmovaps	3392(%rsp), %ymm0
	vmovaps	3424(%rsp), %ymm1
	vmovaps	3456(%rsp), %ymm2
	vmovaps	3488(%rsp), %ymm3
	movq	192(%rsp), %rax
	vmovaps	%ymm3, 96(%rax)
	vmovaps	%ymm2, 64(%rax)
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1058 5
	vmovaps	3520(%rsp), %ymm0
	vmovaps	3552(%rsp), %ymm1
	vmovaps	3584(%rsp), %ymm2
	vmovaps	3616(%rsp), %ymm3
	movq	1664(%rsp), %rax
	vmovaps	%ymm3, 96(%rax)
	vmovaps	%ymm2, 64(%rax)
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1059 5
	vmovaps	3648(%rsp), %ymm0
	vmovaps	3680(%rsp), %ymm1
	movq	1728(%rsp), %rax
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1060 5
	vmovaps	3712(%rsp), %ymm0
	vmovaps	3744(%rsp), %ymm1
	movq	1696(%rsp), %rax
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	movq	48(%rsp), %r14
	.loc	1 1061 5
	movq	%r9, 5272(%r14)
	xorl	%eax, %eax
.Ltmp6588:
	.loc	1 0 5 is_stmt 0
.Ltmp6589:
	.p2align	4
.LBB40_131:
	.loc	1 1194 13 is_stmt 1
	vmovss	352(%rsp,%rax,2), %xmm3
	vmovss	356(%rsp,%rax,2), %xmm4
	vmovss	360(%rsp,%rax,2), %xmm5
	vmovss	364(%rsp,%rax,2), %xmm6
	vmovss	368(%rsp,%rax,2), %xmm7
	vmovss	372(%rsp,%rax,2), %xmm2
	vmovss	376(%rsp,%rax,2), %xmm1
	vmovd	380(%rsp,%rax,2), %xmm0
.Ltmp6590:
	.loc	1 1197 17
	vmovss	%xmm3, (%r14,%rax)
	.loc	1 1198 34
	movl	12(%r14,%rax), %ecx
	movl	172(%r14,%rax), %edx
.Ltmp6591:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6592:
	.loc	1 1198 17
	movl	%ecx, 12(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm4, 160(%r14,%rax)
.Ltmp6593:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%ebx, %edx
.Ltmp6594:
	.loc	1 1198 17
	movl	%edx, 172(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm5, 320(%r14,%rax)
	.loc	1 1198 34
	movl	332(%r14,%rax), %ecx
.Ltmp6595:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6596:
	.loc	1 1198 17
	movl	%ecx, 332(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm6, 480(%r14,%rax)
	.loc	1 1198 34
	movl	492(%r14,%rax), %ecx
.Ltmp6597:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6598:
	.loc	1 1198 17
	movl	%ecx, 492(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm7, 640(%r14,%rax)
	.loc	1 1198 34
	movl	652(%r14,%rax), %ecx
.Ltmp6599:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6600:
	.loc	1 1198 17
	movl	%ecx, 652(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 800(%r14,%rax)
	.loc	1 1198 34
	movl	812(%r14,%rax), %ecx
.Ltmp6601:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6602:
	.loc	1 1198 17
	movl	%ecx, 812(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 960(%r14,%rax)
	.loc	1 1198 34
	movl	972(%r14,%rax), %ecx
.Ltmp6603:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6604:
	.loc	1 1198 17
	movl	%ecx, 972(%r14,%rax)
	.loc	1 1197 17
	vmovd	%xmm0, 1120(%r14,%rax)
	.loc	1 1198 34
	movl	1132(%r14,%rax), %ecx
.Ltmp6605:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6606:
	.loc	1 1198 17
	movl	%ecx, 1132(%r14,%rax)
.Ltmp6607:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp6608:
	.loc	3 900 12
	jne	.LBB40_131
.Ltmp6609:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	1656(%rsp), %rsi
	.p2align	4
.LBB40_133:
.Ltmp6610:
	.loc	1 1194 13 is_stmt 1
	vmovss	992(%rsp,%rax,2), %xmm3
	vmovss	996(%rsp,%rax,2), %xmm4
	vmovss	1000(%rsp,%rax,2), %xmm5
	vmovss	1004(%rsp,%rax,2), %xmm6
	vmovss	1008(%rsp,%rax,2), %xmm7
	vmovss	1012(%rsp,%rax,2), %xmm2
	vmovss	1016(%rsp,%rax,2), %xmm1
	vmovd	1020(%rsp,%rax,2), %xmm0
.Ltmp6611:
	.loc	1 1197 17
	vmovss	%xmm3, 2624(%r14,%rax)
	.loc	1 1198 34
	movl	2636(%r14,%rax), %ecx
	movl	2796(%r14,%rax), %edx
.Ltmp6612:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6613:
	.loc	1 1198 17
	movl	%ecx, 2636(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm4, 2784(%r14,%rax)
.Ltmp6614:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%ebx, %edx
.Ltmp6615:
	.loc	1 1198 17
	movl	%edx, 2796(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm5, 2944(%r14,%rax)
	.loc	1 1198 34
	movl	2956(%r14,%rax), %ecx
.Ltmp6616:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6617:
	.loc	1 1198 17
	movl	%ecx, 2956(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm6, 3104(%r14,%rax)
	.loc	1 1198 34
	movl	3116(%r14,%rax), %ecx
.Ltmp6618:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6619:
	.loc	1 1198 17
	movl	%ecx, 3116(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm7, 3264(%r14,%rax)
	.loc	1 1198 34
	movl	3276(%r14,%rax), %ecx
.Ltmp6620:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6621:
	.loc	1 1198 17
	movl	%ecx, 3276(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 3424(%r14,%rax)
	.loc	1 1198 34
	movl	3436(%r14,%rax), %ecx
.Ltmp6622:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6623:
	.loc	1 1198 17
	movl	%ecx, 3436(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 3584(%r14,%rax)
	.loc	1 1198 34
	movl	3596(%r14,%rax), %ecx
.Ltmp6624:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6625:
	.loc	1 1198 17
	movl	%ecx, 3596(%r14,%rax)
	.loc	1 1197 17
	vmovd	%xmm0, 3744(%r14,%rax)
	.loc	1 1198 34
	movl	3756(%r14,%rax), %ecx
.Ltmp6626:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp6627:
	.loc	1 1198 17
	movl	%ecx, 3756(%r14,%rax)
.Ltmp6628:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp6629:
	.loc	3 900 12
	jne	.LBB40_133
	jmp	.LBB40_112
.Ltmp6630:
	.loc	3 0 12 is_stmt 0
.Ltmp6631:
	.p2align	4
.LBB40_134:
	.loc	38 1050 16 is_stmt 1
	cmpq	%r13, %rsi
.Ltmp6632:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_566
.Ltmp6633:
	.loc	48 451 16 is_stmt 1
	cmpq	40(%rsp), %rsi
	ja	.LBB40_566
.Ltmp6634:
	.loc	48 451 16 is_stmt 0
	cmpq	32(%rsp), %rsi
	ja	.LBB40_570
.Ltmp6635:
	.loc	48 0 16
	movq	192(%rsp), %rax
.Ltmp6636:
	.loc	1 972 27 is_stmt 1
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	64(%rax), %ymm2
	vmovaps	96(%rax), %ymm3
	vmovaps	%ymm3, 3904(%rsp)
	vmovaps	%ymm2, 3872(%rsp)
	vmovaps	%ymm1, 3840(%rsp)
	vmovaps	%ymm0, 3808(%rsp)
	movq	1664(%rsp), %rax
.Ltmp6637:
	.loc	1 973 26
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	64(%rax), %ymm2
	vmovaps	96(%rax), %ymm3
	vmovaps	%ymm3, 4032(%rsp)
	vmovaps	%ymm2, 4000(%rsp)
	vmovaps	%ymm1, 3968(%rsp)
	vmovaps	%ymm0, 3936(%rsp)
	movq	1728(%rsp), %rax
.Ltmp6638:
	.loc	1 974 25
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 4096(%rsp)
	vmovaps	%ymm0, 4064(%rsp)
	movq	1696(%rsp), %rax
.Ltmp6639:
	.loc	1 975 24
	vmovdqa	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 4160(%rsp)
	vmovdqa	%ymm0, 4128(%rsp)
.Ltmp6640:
	.loc	1 976 24
	movq	5272(%r14), %r9
.Ltmp6641:
	.loc	2 1916 50
	testq	%r15, %r15
	je	.LBB40_111
.Ltmp6642:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,8), %rdx
	movq	296(%rsp), %rax
	leaq	(%rax,%r13,4), %r10
	movq	304(%rsp), %rax
	leaq	(%rax,%r13,4), %r11
.Ltmp6643:
	.loc	48 568 12 is_stmt 1
	movq	%r15, %r14
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r14
	xorl	%edi, %edi
	xorl	%r13d, %r13d
.Ltmp6644:
	.loc	48 0 12 is_stmt 0
.Ltmp6645:
	.p2align	4
.LBB40_139:
	.loc	1 987 28 is_stmt 1
	leaq	1(%r9), %rax
	movq	256(%rsp), %r12
.Ltmp6646:
	.loc	1 857 8
	cmpq	%r12, %rax
	jae	.LBB40_141
.Ltmp6647:
	.loc	1 0 8 is_stmt 0
	xorl	%r12d, %r12d
.LBB40_141:
.Ltmp6648:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB40_549
.Ltmp6649:
	.loc	48 438 16
	cmpq	%r13, %r14
	je	.LBB40_547
.Ltmp6650:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r9,8), %rax
.Ltmp6651:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%r11,%rdi,4), %ymm0
	vmovdqa	%ymm0, 4192(%rsp)
	movq	48(%rsp), %rcx
.Ltmp6652:
	.loc	1 991 35
	movq	2600(%rcx), %rsi
.Ltmp6653:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_550
.Ltmp6654:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp6655:
	.loc	1 0 0 is_stmt 0
	leaq	(%r10,%rdi,4), %rcx
	movq	48(%rsp), %r8
.Ltmp6656:
	.loc	1 991 35 is_stmt 1
	movq	2592(%r8), %rsi
.Ltmp6657:
	.loc	8 551 14
	vmovdqu	(%rcx), %ymm0
	vmovdqu	%ymm0, (%rsi,%rax,4)
.Ltmp6658:
	.loc	1 992 34
	movq	5224(%r8), %rsi
.Ltmp6659:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_551
.Ltmp6660:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp6661:
	.loc	1 0 0 is_stmt 0
	negq	%r12
	addq	%r12, %r9
	incq	%r9
	leaq	(,%r9,8), %r8
	movq	48(%rsp), %r12
.Ltmp6662:
	.loc	1 992 34 is_stmt 1
	movq	5216(%r12), %rsi
.Ltmp6663:
	.loc	8 551 14
	vmovaps	4192(%rsp), %ymm0
	vmovups	%ymm0, (%rsi,%rax,4)
.Ltmp6664:
	.loc	1 993 22
	movq	2600(%r12), %rsi
.Ltmp6665:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB40_552
.Ltmp6666:
	.loc	48 438 16
	cmpq	$7, %rax
	jbe	.LBB40_546
.Ltmp6667:
	.loc	48 0 16 is_stmt 0
	movq	48(%rsp), %rsi
	.loc	1 993 22 is_stmt 1
	movq	2592(%rsi), %rax
.Ltmp6668:
	.loc	8 551 14
	vmovups	(%rax,%r8,4), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp6669:
	.loc	1 994 22
	movq	5224(%rsi), %rsi
.Ltmp6670:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB40_553
.Ltmp6671:
	.loc	48 438 16
	cmpq	$7, %rax
	jbe	.LBB40_546
.Ltmp6672:
	.loc	1 0 0 is_stmt 0
	incq	%r13
.Ltmp6673:
	leaq	(%r11,%rdi,4), %rax
	movq	48(%rsp), %rcx
.Ltmp6674:
	.loc	1 994 22 is_stmt 1
	movq	5216(%rcx), %rcx
.Ltmp6675:
	.loc	8 551 14
	vmovdqu	(%rcx,%r8,4), %ymm0
	vmovdqu	%ymm0, (%rax)
.Ltmp6676:
	.loc	2 1916 50
	addq	$8, %rdi
	cmpq	%r13, %r15
.Ltmp6677:
	.loc	3 900 12
	jne	.LBB40_139
	jmp	.LBB40_111
.Ltmp6678:
.LBB40_152:
	.loc	3 0 12 is_stmt 0
	vmovaps	1760(%rsp), %ymm0
.Ltmp6679:
	.loc	1 1057 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r14)
	vmovaps	1664(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r14)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r14)
	vmovaps	1696(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r14)
	vmovaps	256(%rsp), %ymm0
	.loc	1 1058 5
	vmovaps	%ymm0, 4032(%r14)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r14)
	vmovaps	1920(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r14)
	vmovaps	1824(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r14)
	vmovaps	1984(%rsp), %ymm0
	.loc	1 1059 5
	vmovaps	%ymm0, 2304(%r14)
	vmovaps	1856(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r14)
	vmovdqa	1888(%rsp), %ymm0
	.loc	1 1060 5
	vmovdqa	%ymm0, 4928(%r14)
	vmovaps	%ymm8, 4960(%r14)
	.loc	1 1061 5
	movq	%r10, 5272(%r14)
	movq	1656(%rsp), %rsi
.Ltmp6680:
.LBB40_153:
	.loc	1 0 5 is_stmt 0
	movq	1816(%rsp), %r15
	.loc	1 1086 11 is_stmt 1
	cmpq	%rsi, %r15
	jae	.LBB40_511
.LBB40_154:
	.loc	1 1087 42
	subq	%r15, %rsi
	.loc	1 1087 29 is_stmt 0
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E12plan_segmentB5_
	movl	%edx, %r13d
	movq	%rax, %r12
.Ltmp6681:
	.loc	1 1176 33 is_stmt 1
	vmovss	(%r14), %xmm0
.Ltmp6682:
	.loc	1 1089 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp6683:
	.loc	1 1176 33
	vmovss	160(%r14), %xmm0
.Ltmp6684:
	.loc	1 1089 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp6685:
	.loc	1 1176 33
	vmovss	320(%r14), %xmm0
.Ltmp6686:
	.loc	1 1089 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp6687:
	.loc	1 1176 33
	vmovss	480(%r14), %xmm0
.Ltmp6688:
	.loc	1 1089 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp6689:
	.loc	1 1176 33
	vmovss	640(%r14), %xmm0
.Ltmp6690:
	.loc	1 1089 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp6691:
	.loc	1 1176 33
	vmovss	800(%r14), %xmm0
.Ltmp6692:
	.loc	1 1089 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp6693:
	.loc	1 1176 33
	vmovss	960(%r14), %xmm0
.Ltmp6694:
	.loc	1 1089 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp6695:
	.loc	1 1176 33
	vmovss	1120(%r14), %xmm0
.Ltmp6696:
	.loc	1 1089 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp6697:
	.loc	1 1176 33
	vmovss	16(%r14), %xmm0
.Ltmp6698:
	.loc	1 1089 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp6699:
	.loc	1 1176 33
	vmovss	176(%r14), %xmm0
.Ltmp6700:
	.loc	1 1089 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp6701:
	.loc	1 1176 33
	vmovss	336(%r14), %xmm0
.Ltmp6702:
	.loc	1 1089 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp6703:
	.loc	1 1176 33
	vmovss	496(%r14), %xmm0
.Ltmp6704:
	.loc	1 1089 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp6705:
	.loc	1 1176 33
	vmovss	656(%r14), %xmm0
.Ltmp6706:
	.loc	1 1089 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp6707:
	.loc	1 1176 33
	vmovss	816(%r14), %xmm0
.Ltmp6708:
	.loc	1 1089 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp6709:
	.loc	1 1176 33
	vmovss	976(%r14), %xmm0
.Ltmp6710:
	.loc	1 1089 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp6711:
	.loc	1 1176 33
	vmovss	1136(%r14), %xmm0
.Ltmp6712:
	.loc	1 1089 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp6713:
	.loc	1 1176 33
	vmovss	32(%r14), %xmm0
.Ltmp6714:
	.loc	1 1089 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp6715:
	.loc	1 1176 33
	vmovss	192(%r14), %xmm0
.Ltmp6716:
	.loc	1 1089 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp6717:
	.loc	1 1176 33
	vmovss	352(%r14), %xmm0
.Ltmp6718:
	.loc	1 1089 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp6719:
	.loc	1 1176 33
	vmovss	512(%r14), %xmm0
.Ltmp6720:
	.loc	1 1089 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp6721:
	.loc	1 1176 33
	vmovss	672(%r14), %xmm0
.Ltmp6722:
	.loc	1 1089 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp6723:
	.loc	1 1176 33
	vmovss	832(%r14), %xmm0
.Ltmp6724:
	.loc	1 1089 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp6725:
	.loc	1 1176 33
	vmovss	992(%r14), %xmm0
.Ltmp6726:
	.loc	1 1089 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp6727:
	.loc	1 1176 33
	vmovss	1152(%r14), %xmm0
.Ltmp6728:
	.loc	1 1089 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp6729:
	.loc	1 1176 33
	vmovss	48(%r14), %xmm0
.Ltmp6730:
	.loc	1 1089 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp6731:
	.loc	1 1176 33
	vmovss	208(%r14), %xmm0
.Ltmp6732:
	.loc	1 1089 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp6733:
	.loc	1 1176 33
	vmovss	368(%r14), %xmm0
.Ltmp6734:
	.loc	1 1089 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp6735:
	.loc	1 1176 33
	vmovss	528(%r14), %xmm0
.Ltmp6736:
	.loc	1 1089 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp6737:
	.loc	1 1176 33
	vmovss	688(%r14), %xmm0
.Ltmp6738:
	.loc	1 1089 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp6739:
	.loc	1 1176 33
	vmovss	848(%r14), %xmm0
.Ltmp6740:
	.loc	1 1089 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp6741:
	.loc	1 1176 33
	vmovss	1008(%r14), %xmm0
.Ltmp6742:
	.loc	1 1089 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp6743:
	.loc	1 1176 33
	vmovss	1168(%r14), %xmm0
.Ltmp6744:
	.loc	1 1089 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp6745:
	.loc	1 1176 33
	vmovss	64(%r14), %xmm0
.Ltmp6746:
	.loc	1 1089 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp6747:
	.loc	1 1176 33
	vmovss	224(%r14), %xmm0
.Ltmp6748:
	.loc	1 1089 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp6749:
	.loc	1 1176 33
	vmovss	384(%r14), %xmm0
.Ltmp6750:
	.loc	1 1089 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp6751:
	.loc	1 1176 33
	vmovss	544(%r14), %xmm0
.Ltmp6752:
	.loc	1 1089 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp6753:
	.loc	1 1176 33
	vmovss	704(%r14), %xmm0
.Ltmp6754:
	.loc	1 1089 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp6755:
	.loc	1 1176 33
	vmovss	864(%r14), %xmm0
.Ltmp6756:
	.loc	1 1089 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp6757:
	.loc	1 1176 33
	vmovss	1024(%r14), %xmm0
.Ltmp6758:
	.loc	1 1089 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp6759:
	.loc	1 1176 33
	vmovss	1184(%r14), %xmm0
.Ltmp6760:
	.loc	1 1089 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp6761:
	.loc	1 1176 33
	vmovss	80(%r14), %xmm0
.Ltmp6762:
	.loc	1 1089 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp6763:
	.loc	1 1176 33
	vmovss	240(%r14), %xmm0
.Ltmp6764:
	.loc	1 1089 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp6765:
	.loc	1 1176 33
	vmovss	400(%r14), %xmm0
.Ltmp6766:
	.loc	1 1089 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp6767:
	.loc	1 1176 33
	vmovss	560(%r14), %xmm0
.Ltmp6768:
	.loc	1 1089 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp6769:
	.loc	1 1176 33
	vmovss	720(%r14), %xmm0
.Ltmp6770:
	.loc	1 1089 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp6771:
	.loc	1 1176 33
	vmovss	880(%r14), %xmm0
.Ltmp6772:
	.loc	1 1089 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp6773:
	.loc	1 1176 33
	vmovss	1040(%r14), %xmm0
.Ltmp6774:
	.loc	1 1089 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp6775:
	.loc	1 1176 33
	vmovss	1200(%r14), %xmm0
.Ltmp6776:
	.loc	1 1089 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp6777:
	.loc	1 1176 33
	vmovss	96(%r14), %xmm0
.Ltmp6778:
	.loc	1 1089 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp6779:
	.loc	1 1176 33
	vmovss	256(%r14), %xmm0
.Ltmp6780:
	.loc	1 1089 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp6781:
	.loc	1 1176 33
	vmovss	416(%r14), %xmm0
.Ltmp6782:
	.loc	1 1089 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp6783:
	.loc	1 1176 33
	vmovss	576(%r14), %xmm0
.Ltmp6784:
	.loc	1 1089 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp6785:
	.loc	1 1176 33
	vmovss	736(%r14), %xmm0
.Ltmp6786:
	.loc	1 1089 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp6787:
	.loc	1 1176 33
	vmovss	896(%r14), %xmm0
.Ltmp6788:
	.loc	1 1089 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp6789:
	.loc	1 1176 33
	vmovss	1056(%r14), %xmm0
.Ltmp6790:
	.loc	1 1089 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp6791:
	.loc	1 1176 33
	vmovss	1216(%r14), %xmm0
.Ltmp6792:
	.loc	1 1089 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp6793:
	.loc	1 1176 33
	vmovss	112(%r14), %xmm0
.Ltmp6794:
	.loc	1 1089 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp6795:
	.loc	1 1176 33
	vmovss	272(%r14), %xmm0
.Ltmp6796:
	.loc	1 1089 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp6797:
	.loc	1 1176 33
	vmovss	432(%r14), %xmm0
.Ltmp6798:
	.loc	1 1089 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp6799:
	.loc	1 1176 33
	vmovss	592(%r14), %xmm0
.Ltmp6800:
	.loc	1 1089 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp6801:
	.loc	1 1176 33
	vmovss	752(%r14), %xmm0
.Ltmp6802:
	.loc	1 1089 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp6803:
	.loc	1 1176 33
	vmovss	912(%r14), %xmm0
.Ltmp6804:
	.loc	1 1089 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp6805:
	.loc	1 1176 33
	vmovss	1072(%r14), %xmm0
.Ltmp6806:
	.loc	1 1089 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp6807:
	.loc	1 1176 33
	vmovss	1232(%r14), %xmm0
.Ltmp6808:
	.loc	1 1089 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp6809:
	.loc	1 1176 33
	vmovss	128(%r14), %xmm0
.Ltmp6810:
	.loc	1 1089 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp6811:
	.loc	1 1176 33
	vmovss	288(%r14), %xmm0
.Ltmp6812:
	.loc	1 1089 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp6813:
	.loc	1 1176 33
	vmovss	448(%r14), %xmm0
.Ltmp6814:
	.loc	1 1089 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp6815:
	.loc	1 1176 33
	vmovss	608(%r14), %xmm0
.Ltmp6816:
	.loc	1 1089 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp6817:
	.loc	1 1176 33
	vmovss	768(%r14), %xmm0
.Ltmp6818:
	.loc	1 1089 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp6819:
	.loc	1 1176 33
	vmovss	928(%r14), %xmm0
.Ltmp6820:
	.loc	1 1089 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp6821:
	.loc	1 1176 33
	vmovss	1088(%r14), %xmm0
.Ltmp6822:
	.loc	1 1089 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp6823:
	.loc	1 1176 33
	vmovss	1248(%r14), %xmm0
.Ltmp6824:
	.loc	1 1089 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp6825:
	.loc	1 1176 33
	vmovss	144(%r14), %xmm0
.Ltmp6826:
	.loc	1 1089 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp6827:
	.loc	1 1176 33
	vmovss	304(%r14), %xmm0
.Ltmp6828:
	.loc	1 1089 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp6829:
	.loc	1 1176 33
	vmovss	464(%r14), %xmm0
.Ltmp6830:
	.loc	1 1089 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp6831:
	.loc	1 1176 33
	vmovss	624(%r14), %xmm0
.Ltmp6832:
	.loc	1 1089 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp6833:
	.loc	1 1176 33
	vmovss	784(%r14), %xmm0
.Ltmp6834:
	.loc	1 1089 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp6835:
	.loc	1 1176 33
	vmovss	944(%r14), %xmm0
.Ltmp6836:
	.loc	1 1089 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp6837:
	.loc	1 1176 33
	vmovss	1104(%r14), %xmm0
.Ltmp6838:
	.loc	1 1089 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp6839:
	.loc	1 1176 33
	vmovss	1264(%r14), %xmm0
.Ltmp6840:
	.loc	1 1089 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp6841:
	.loc	1 1177 32
	vmovss	8(%r14), %xmm0
.Ltmp6842:
	.loc	1 1089 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp6843:
	.loc	1 1177 32
	vmovss	168(%r14), %xmm0
.Ltmp6844:
	.loc	1 1089 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp6845:
	.loc	1 1177 32
	vmovss	328(%r14), %xmm0
.Ltmp6846:
	.loc	1 1089 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp6847:
	.loc	1 1177 32
	vmovss	488(%r14), %xmm0
.Ltmp6848:
	.loc	1 1089 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp6849:
	.loc	1 1177 32
	vmovss	648(%r14), %xmm0
.Ltmp6850:
	.loc	1 1089 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp6851:
	.loc	1 1177 32
	vmovss	808(%r14), %xmm0
.Ltmp6852:
	.loc	1 1089 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp6853:
	.loc	1 1177 32
	vmovss	968(%r14), %xmm0
.Ltmp6854:
	.loc	1 1089 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp6855:
	.loc	1 1177 32
	vmovss	1128(%r14), %xmm0
.Ltmp6856:
	.loc	1 1089 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp6857:
	.loc	1 1177 32
	vmovss	24(%r14), %xmm0
.Ltmp6858:
	.loc	1 1089 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp6859:
	.loc	1 1177 32
	vmovss	184(%r14), %xmm0
.Ltmp6860:
	.loc	1 1089 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp6861:
	.loc	1 1177 32
	vmovss	344(%r14), %xmm0
.Ltmp6862:
	.loc	1 1089 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp6863:
	.loc	1 1177 32
	vmovss	504(%r14), %xmm0
.Ltmp6864:
	.loc	1 1089 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp6865:
	.loc	1 1177 32
	vmovss	664(%r14), %xmm0
.Ltmp6866:
	.loc	1 1089 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp6867:
	.loc	1 1177 32
	vmovss	824(%r14), %xmm0
.Ltmp6868:
	.loc	1 1089 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp6869:
	.loc	1 1177 32
	vmovss	984(%r14), %xmm0
.Ltmp6870:
	.loc	1 1089 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp6871:
	.loc	1 1177 32
	vmovss	1144(%r14), %xmm0
.Ltmp6872:
	.loc	1 1089 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp6873:
	.loc	1 1177 32
	vmovss	40(%r14), %xmm0
.Ltmp6874:
	.loc	1 1089 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp6875:
	.loc	1 1177 32
	vmovss	200(%r14), %xmm0
.Ltmp6876:
	.loc	1 1089 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp6877:
	.loc	1 1177 32
	vmovss	360(%r14), %xmm0
.Ltmp6878:
	.loc	1 1089 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp6879:
	.loc	1 1177 32
	vmovss	520(%r14), %xmm0
.Ltmp6880:
	.loc	1 1089 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp6881:
	.loc	1 1177 32
	vmovss	680(%r14), %xmm0
.Ltmp6882:
	.loc	1 1089 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp6883:
	.loc	1 1177 32
	vmovss	840(%r14), %xmm0
.Ltmp6884:
	.loc	1 1089 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp6885:
	.loc	1 1177 32
	vmovss	1000(%r14), %xmm0
.Ltmp6886:
	.loc	1 1089 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp6887:
	.loc	1 1177 32
	vmovss	1160(%r14), %xmm0
.Ltmp6888:
	.loc	1 1089 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp6889:
	.loc	1 1177 32
	vmovss	56(%r14), %xmm0
.Ltmp6890:
	.loc	1 1089 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp6891:
	.loc	1 1177 32
	vmovss	216(%r14), %xmm0
.Ltmp6892:
	.loc	1 1089 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp6893:
	.loc	1 1177 32
	vmovss	376(%r14), %xmm0
.Ltmp6894:
	.loc	1 1089 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp6895:
	.loc	1 1177 32
	vmovss	536(%r14), %xmm0
.Ltmp6896:
	.loc	1 1089 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp6897:
	.loc	1 1177 32
	vmovss	696(%r14), %xmm0
.Ltmp6898:
	.loc	1 1089 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp6899:
	.loc	1 1177 32
	vmovss	856(%r14), %xmm0
.Ltmp6900:
	.loc	1 1089 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp6901:
	.loc	1 1177 32
	vmovss	1016(%r14), %xmm0
.Ltmp6902:
	.loc	1 1089 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp6903:
	.loc	1 1177 32
	vmovss	1176(%r14), %xmm0
.Ltmp6904:
	.loc	1 1089 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp6905:
	.loc	1 1177 32
	vmovss	72(%r14), %xmm0
.Ltmp6906:
	.loc	1 1089 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp6907:
	.loc	1 1177 32
	vmovss	232(%r14), %xmm0
.Ltmp6908:
	.loc	1 1089 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp6909:
	.loc	1 1177 32
	vmovss	392(%r14), %xmm0
.Ltmp6910:
	.loc	1 1089 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp6911:
	.loc	1 1177 32
	vmovss	552(%r14), %xmm0
.Ltmp6912:
	.loc	1 1089 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp6913:
	.loc	1 1177 32
	vmovss	712(%r14), %xmm0
.Ltmp6914:
	.loc	1 1089 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp6915:
	.loc	1 1177 32
	vmovss	872(%r14), %xmm0
.Ltmp6916:
	.loc	1 1089 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp6917:
	.loc	1 1177 32
	vmovss	1032(%r14), %xmm0
.Ltmp6918:
	.loc	1 1089 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp6919:
	.loc	1 1177 32
	vmovss	1192(%r14), %xmm0
.Ltmp6920:
	.loc	1 1089 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp6921:
	.loc	1 1177 32
	vmovss	88(%r14), %xmm0
.Ltmp6922:
	.loc	1 1089 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp6923:
	.loc	1 1177 32
	vmovss	248(%r14), %xmm0
.Ltmp6924:
	.loc	1 1089 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp6925:
	.loc	1 1177 32
	vmovss	408(%r14), %xmm0
.Ltmp6926:
	.loc	1 1089 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp6927:
	.loc	1 1177 32
	vmovss	568(%r14), %xmm0
.Ltmp6928:
	.loc	1 1089 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp6929:
	.loc	1 1177 32
	vmovss	728(%r14), %xmm0
.Ltmp6930:
	.loc	1 1089 28
	vmovss	%xmm0, 848(%rsp)
.Ltmp6931:
	.loc	1 1177 32
	vmovss	888(%r14), %xmm0
.Ltmp6932:
	.loc	1 1089 28
	vmovss	%xmm0, 852(%rsp)
.Ltmp6933:
	.loc	1 1177 32
	vmovss	1048(%r14), %xmm0
.Ltmp6934:
	.loc	1 1089 28
	vmovss	%xmm0, 856(%rsp)
.Ltmp6935:
	.loc	1 1177 32
	vmovss	1208(%r14), %xmm0
.Ltmp6936:
	.loc	1 1089 28
	vmovss	%xmm0, 860(%rsp)
.Ltmp6937:
	.loc	1 1177 32
	vmovss	104(%r14), %xmm0
.Ltmp6938:
	.loc	1 1089 28
	vmovss	%xmm0, 864(%rsp)
.Ltmp6939:
	.loc	1 1177 32
	vmovss	264(%r14), %xmm0
.Ltmp6940:
	.loc	1 1089 28
	vmovss	%xmm0, 868(%rsp)
.Ltmp6941:
	.loc	1 1177 32
	vmovss	424(%r14), %xmm0
.Ltmp6942:
	.loc	1 1089 28
	vmovss	%xmm0, 872(%rsp)
.Ltmp6943:
	.loc	1 1177 32
	vmovss	584(%r14), %xmm0
.Ltmp6944:
	.loc	1 1089 28
	vmovss	%xmm0, 876(%rsp)
.Ltmp6945:
	.loc	1 1177 32
	vmovss	744(%r14), %xmm0
.Ltmp6946:
	.loc	1 1089 28
	vmovss	%xmm0, 880(%rsp)
.Ltmp6947:
	.loc	1 1177 32
	vmovss	904(%r14), %xmm0
.Ltmp6948:
	.loc	1 1089 28
	vmovss	%xmm0, 884(%rsp)
.Ltmp6949:
	.loc	1 1177 32
	vmovss	1064(%r14), %xmm0
.Ltmp6950:
	.loc	1 1089 28
	vmovss	%xmm0, 888(%rsp)
.Ltmp6951:
	.loc	1 1177 32
	vmovss	1224(%r14), %xmm0
.Ltmp6952:
	.loc	1 1089 28
	vmovss	%xmm0, 892(%rsp)
.Ltmp6953:
	.loc	1 1177 32
	vmovss	120(%r14), %xmm0
.Ltmp6954:
	.loc	1 1089 28
	vmovss	%xmm0, 896(%rsp)
.Ltmp6955:
	.loc	1 1177 32
	vmovss	280(%r14), %xmm0
.Ltmp6956:
	.loc	1 1089 28
	vmovss	%xmm0, 900(%rsp)
.Ltmp6957:
	.loc	1 1177 32
	vmovss	440(%r14), %xmm0
.Ltmp6958:
	.loc	1 1089 28
	vmovss	%xmm0, 904(%rsp)
.Ltmp6959:
	.loc	1 1177 32
	vmovss	600(%r14), %xmm0
.Ltmp6960:
	.loc	1 1089 28
	vmovss	%xmm0, 908(%rsp)
.Ltmp6961:
	.loc	1 1177 32
	vmovss	760(%r14), %xmm0
.Ltmp6962:
	.loc	1 1089 28
	vmovss	%xmm0, 912(%rsp)
.Ltmp6963:
	.loc	1 1177 32
	vmovss	920(%r14), %xmm0
.Ltmp6964:
	.loc	1 1089 28
	vmovss	%xmm0, 916(%rsp)
.Ltmp6965:
	.loc	1 1177 32
	vmovss	1080(%r14), %xmm0
.Ltmp6966:
	.loc	1 1089 28
	vmovss	%xmm0, 920(%rsp)
.Ltmp6967:
	.loc	1 1177 32
	vmovss	1240(%r14), %xmm0
.Ltmp6968:
	.loc	1 1089 28
	vmovss	%xmm0, 924(%rsp)
.Ltmp6969:
	.loc	1 1177 32
	vmovss	136(%r14), %xmm0
.Ltmp6970:
	.loc	1 1089 28
	vmovss	%xmm0, 928(%rsp)
.Ltmp6971:
	.loc	1 1177 32
	vmovss	296(%r14), %xmm0
.Ltmp6972:
	.loc	1 1089 28
	vmovss	%xmm0, 932(%rsp)
.Ltmp6973:
	.loc	1 1177 32
	vmovss	456(%r14), %xmm0
.Ltmp6974:
	.loc	1 1089 28
	vmovss	%xmm0, 936(%rsp)
.Ltmp6975:
	.loc	1 1177 32
	vmovss	616(%r14), %xmm0
.Ltmp6976:
	.loc	1 1089 28
	vmovss	%xmm0, 940(%rsp)
.Ltmp6977:
	.loc	1 1177 32
	vmovss	776(%r14), %xmm0
.Ltmp6978:
	.loc	1 1089 28
	vmovss	%xmm0, 944(%rsp)
.Ltmp6979:
	.loc	1 1177 32
	vmovss	936(%r14), %xmm0
.Ltmp6980:
	.loc	1 1089 28
	vmovss	%xmm0, 948(%rsp)
.Ltmp6981:
	.loc	1 1177 32
	vmovss	1096(%r14), %xmm0
.Ltmp6982:
	.loc	1 1089 28
	vmovss	%xmm0, 952(%rsp)
.Ltmp6983:
	.loc	1 1177 32
	vmovss	1256(%r14), %xmm0
.Ltmp6984:
	.loc	1 1089 28
	vmovss	%xmm0, 956(%rsp)
.Ltmp6985:
	.loc	1 1177 32
	vmovss	152(%r14), %xmm0
.Ltmp6986:
	.loc	1 1089 28
	vmovss	%xmm0, 960(%rsp)
.Ltmp6987:
	.loc	1 1177 32
	vmovss	312(%r14), %xmm0
.Ltmp6988:
	.loc	1 1089 28
	vmovss	%xmm0, 964(%rsp)
.Ltmp6989:
	.loc	1 1177 32
	vmovss	472(%r14), %xmm0
.Ltmp6990:
	.loc	1 1089 28
	vmovss	%xmm0, 968(%rsp)
.Ltmp6991:
	.loc	1 1177 32
	vmovss	632(%r14), %xmm0
.Ltmp6992:
	.loc	1 1089 28
	vmovss	%xmm0, 972(%rsp)
.Ltmp6993:
	.loc	1 1177 32
	vmovss	792(%r14), %xmm0
.Ltmp6994:
	.loc	1 1089 28
	vmovss	%xmm0, 976(%rsp)
.Ltmp6995:
	.loc	1 1177 32
	vmovss	952(%r14), %xmm0
.Ltmp6996:
	.loc	1 1089 28
	vmovss	%xmm0, 980(%rsp)
.Ltmp6997:
	.loc	1 1177 32
	vmovss	1112(%r14), %xmm0
.Ltmp6998:
	.loc	1 1089 28
	vmovss	%xmm0, 984(%rsp)
.Ltmp6999:
	.loc	1 1177 32
	vmovss	1272(%r14), %xmm0
.Ltmp7000:
	.loc	1 1089 28
	vmovss	%xmm0, 988(%rsp)
.Ltmp7001:
	.loc	1 1176 33
	vmovss	2624(%r14), %xmm0
.Ltmp7002:
	.loc	1 1089 28
	vmovss	%xmm0, 992(%rsp)
.Ltmp7003:
	.loc	1 1176 33
	vmovss	2784(%r14), %xmm0
.Ltmp7004:
	.loc	1 1089 28
	vmovss	%xmm0, 996(%rsp)
.Ltmp7005:
	.loc	1 1176 33
	vmovss	2944(%r14), %xmm0
.Ltmp7006:
	.loc	1 1089 28
	vmovss	%xmm0, 1000(%rsp)
.Ltmp7007:
	.loc	1 1176 33
	vmovss	3104(%r14), %xmm0
.Ltmp7008:
	.loc	1 1089 28
	vmovss	%xmm0, 1004(%rsp)
.Ltmp7009:
	.loc	1 1176 33
	vmovss	3264(%r14), %xmm0
.Ltmp7010:
	.loc	1 1089 28
	vmovss	%xmm0, 1008(%rsp)
.Ltmp7011:
	.loc	1 1176 33
	vmovss	3424(%r14), %xmm0
.Ltmp7012:
	.loc	1 1089 28
	vmovss	%xmm0, 1012(%rsp)
.Ltmp7013:
	.loc	1 1176 33
	vmovss	3584(%r14), %xmm0
.Ltmp7014:
	.loc	1 1089 28
	vmovss	%xmm0, 1016(%rsp)
.Ltmp7015:
	.loc	1 1176 33
	vmovss	3744(%r14), %xmm0
.Ltmp7016:
	.loc	1 1089 28
	vmovss	%xmm0, 1020(%rsp)
.Ltmp7017:
	.loc	1 1176 33
	vmovss	2640(%r14), %xmm0
.Ltmp7018:
	.loc	1 1089 28
	vmovss	%xmm0, 1024(%rsp)
.Ltmp7019:
	.loc	1 1176 33
	vmovss	2800(%r14), %xmm0
.Ltmp7020:
	.loc	1 1089 28
	vmovss	%xmm0, 1028(%rsp)
.Ltmp7021:
	.loc	1 1176 33
	vmovss	2960(%r14), %xmm0
.Ltmp7022:
	.loc	1 1089 28
	vmovss	%xmm0, 1032(%rsp)
.Ltmp7023:
	.loc	1 1176 33
	vmovss	3120(%r14), %xmm0
.Ltmp7024:
	.loc	1 1089 28
	vmovss	%xmm0, 1036(%rsp)
.Ltmp7025:
	.loc	1 1176 33
	vmovss	3280(%r14), %xmm0
.Ltmp7026:
	.loc	1 1089 28
	vmovss	%xmm0, 1040(%rsp)
.Ltmp7027:
	.loc	1 1176 33
	vmovss	3440(%r14), %xmm0
.Ltmp7028:
	.loc	1 1089 28
	vmovss	%xmm0, 1044(%rsp)
.Ltmp7029:
	.loc	1 1176 33
	vmovss	3600(%r14), %xmm0
.Ltmp7030:
	.loc	1 1089 28
	vmovss	%xmm0, 1048(%rsp)
.Ltmp7031:
	.loc	1 1176 33
	vmovss	3760(%r14), %xmm0
.Ltmp7032:
	.loc	1 1089 28
	vmovss	%xmm0, 1052(%rsp)
.Ltmp7033:
	.loc	1 1176 33
	vmovss	2656(%r14), %xmm0
.Ltmp7034:
	.loc	1 1089 28
	vmovss	%xmm0, 1056(%rsp)
.Ltmp7035:
	.loc	1 1176 33
	vmovss	2816(%r14), %xmm0
.Ltmp7036:
	.loc	1 1089 28
	vmovss	%xmm0, 1060(%rsp)
.Ltmp7037:
	.loc	1 1176 33
	vmovss	2976(%r14), %xmm0
.Ltmp7038:
	.loc	1 1089 28
	vmovss	%xmm0, 1064(%rsp)
.Ltmp7039:
	.loc	1 1176 33
	vmovss	3136(%r14), %xmm0
.Ltmp7040:
	.loc	1 1089 28
	vmovss	%xmm0, 1068(%rsp)
.Ltmp7041:
	.loc	1 1176 33
	vmovss	3296(%r14), %xmm0
.Ltmp7042:
	.loc	1 1089 28
	vmovss	%xmm0, 1072(%rsp)
.Ltmp7043:
	.loc	1 1176 33
	vmovss	3456(%r14), %xmm0
.Ltmp7044:
	.loc	1 1089 28
	vmovss	%xmm0, 1076(%rsp)
.Ltmp7045:
	.loc	1 1176 33
	vmovss	3616(%r14), %xmm0
.Ltmp7046:
	.loc	1 1089 28
	vmovss	%xmm0, 1080(%rsp)
.Ltmp7047:
	.loc	1 1176 33
	vmovss	3776(%r14), %xmm0
.Ltmp7048:
	.loc	1 1089 28
	vmovss	%xmm0, 1084(%rsp)
.Ltmp7049:
	.loc	1 1176 33
	vmovss	2672(%r14), %xmm0
.Ltmp7050:
	.loc	1 1089 28
	vmovss	%xmm0, 1088(%rsp)
.Ltmp7051:
	.loc	1 1176 33
	vmovss	2832(%r14), %xmm0
.Ltmp7052:
	.loc	1 1089 28
	vmovss	%xmm0, 1092(%rsp)
.Ltmp7053:
	.loc	1 1176 33
	vmovss	2992(%r14), %xmm0
.Ltmp7054:
	.loc	1 1089 28
	vmovss	%xmm0, 1096(%rsp)
.Ltmp7055:
	.loc	1 1176 33
	vmovss	3152(%r14), %xmm0
.Ltmp7056:
	.loc	1 1089 28
	vmovss	%xmm0, 1100(%rsp)
.Ltmp7057:
	.loc	1 1176 33
	vmovss	3312(%r14), %xmm0
.Ltmp7058:
	.loc	1 1089 28
	vmovss	%xmm0, 1104(%rsp)
.Ltmp7059:
	.loc	1 1176 33
	vmovss	3472(%r14), %xmm0
.Ltmp7060:
	.loc	1 1089 28
	vmovss	%xmm0, 1108(%rsp)
.Ltmp7061:
	.loc	1 1176 33
	vmovss	3632(%r14), %xmm0
.Ltmp7062:
	.loc	1 1089 28
	vmovss	%xmm0, 1112(%rsp)
.Ltmp7063:
	.loc	1 1176 33
	vmovss	3792(%r14), %xmm0
.Ltmp7064:
	.loc	1 1089 28
	vmovss	%xmm0, 1116(%rsp)
.Ltmp7065:
	.loc	1 1176 33
	vmovss	2688(%r14), %xmm0
.Ltmp7066:
	.loc	1 1089 28
	vmovss	%xmm0, 1120(%rsp)
.Ltmp7067:
	.loc	1 1176 33
	vmovss	2848(%r14), %xmm0
.Ltmp7068:
	.loc	1 1089 28
	vmovss	%xmm0, 1124(%rsp)
.Ltmp7069:
	.loc	1 1176 33
	vmovss	3008(%r14), %xmm0
.Ltmp7070:
	.loc	1 1089 28
	vmovss	%xmm0, 1128(%rsp)
.Ltmp7071:
	.loc	1 1176 33
	vmovss	3168(%r14), %xmm0
.Ltmp7072:
	.loc	1 1089 28
	vmovss	%xmm0, 1132(%rsp)
.Ltmp7073:
	.loc	1 1176 33
	vmovss	3328(%r14), %xmm0
.Ltmp7074:
	.loc	1 1089 28
	vmovss	%xmm0, 1136(%rsp)
.Ltmp7075:
	.loc	1 1176 33
	vmovss	3488(%r14), %xmm0
.Ltmp7076:
	.loc	1 1089 28
	vmovss	%xmm0, 1140(%rsp)
.Ltmp7077:
	.loc	1 1176 33
	vmovss	3648(%r14), %xmm0
.Ltmp7078:
	.loc	1 1089 28
	vmovss	%xmm0, 1144(%rsp)
.Ltmp7079:
	.loc	1 1176 33
	vmovss	3808(%r14), %xmm0
.Ltmp7080:
	.loc	1 1089 28
	vmovss	%xmm0, 1148(%rsp)
.Ltmp7081:
	.loc	1 1176 33
	vmovss	2704(%r14), %xmm0
.Ltmp7082:
	.loc	1 1089 28
	vmovss	%xmm0, 1152(%rsp)
.Ltmp7083:
	.loc	1 1176 33
	vmovss	2864(%r14), %xmm0
.Ltmp7084:
	.loc	1 1089 28
	vmovss	%xmm0, 1156(%rsp)
.Ltmp7085:
	.loc	1 1176 33
	vmovss	3024(%r14), %xmm0
.Ltmp7086:
	.loc	1 1089 28
	vmovss	%xmm0, 1160(%rsp)
.Ltmp7087:
	.loc	1 1176 33
	vmovss	3184(%r14), %xmm0
.Ltmp7088:
	.loc	1 1089 28
	vmovss	%xmm0, 1164(%rsp)
.Ltmp7089:
	.loc	1 1176 33
	vmovss	3344(%r14), %xmm0
.Ltmp7090:
	.loc	1 1089 28
	vmovss	%xmm0, 1168(%rsp)
.Ltmp7091:
	.loc	1 1176 33
	vmovss	3504(%r14), %xmm0
.Ltmp7092:
	.loc	1 1089 28
	vmovss	%xmm0, 1172(%rsp)
.Ltmp7093:
	.loc	1 1176 33
	vmovss	3664(%r14), %xmm0
.Ltmp7094:
	.loc	1 1089 28
	vmovss	%xmm0, 1176(%rsp)
.Ltmp7095:
	.loc	1 1176 33
	vmovss	3824(%r14), %xmm0
.Ltmp7096:
	.loc	1 1089 28
	vmovss	%xmm0, 1180(%rsp)
.Ltmp7097:
	.loc	1 1176 33
	vmovss	2720(%r14), %xmm0
.Ltmp7098:
	.loc	1 1089 28
	vmovss	%xmm0, 1184(%rsp)
.Ltmp7099:
	.loc	1 1176 33
	vmovss	2880(%r14), %xmm0
.Ltmp7100:
	.loc	1 1089 28
	vmovss	%xmm0, 1188(%rsp)
.Ltmp7101:
	.loc	1 1176 33
	vmovss	3040(%r14), %xmm0
.Ltmp7102:
	.loc	1 1089 28
	vmovss	%xmm0, 1192(%rsp)
.Ltmp7103:
	.loc	1 1176 33
	vmovss	3200(%r14), %xmm0
.Ltmp7104:
	.loc	1 1089 28
	vmovss	%xmm0, 1196(%rsp)
.Ltmp7105:
	.loc	1 1176 33
	vmovss	3360(%r14), %xmm0
.Ltmp7106:
	.loc	1 1089 28
	vmovss	%xmm0, 1200(%rsp)
.Ltmp7107:
	.loc	1 1176 33
	vmovss	3520(%r14), %xmm0
.Ltmp7108:
	.loc	1 1089 28
	vmovss	%xmm0, 1204(%rsp)
.Ltmp7109:
	.loc	1 1176 33
	vmovss	3680(%r14), %xmm0
.Ltmp7110:
	.loc	1 1089 28
	vmovss	%xmm0, 1208(%rsp)
.Ltmp7111:
	.loc	1 1176 33
	vmovss	3840(%r14), %xmm0
.Ltmp7112:
	.loc	1 1089 28
	vmovss	%xmm0, 1212(%rsp)
.Ltmp7113:
	.loc	1 1176 33
	vmovss	2736(%r14), %xmm0
.Ltmp7114:
	.loc	1 1089 28
	vmovss	%xmm0, 1216(%rsp)
.Ltmp7115:
	.loc	1 1176 33
	vmovss	2896(%r14), %xmm0
.Ltmp7116:
	.loc	1 1089 28
	vmovss	%xmm0, 1220(%rsp)
.Ltmp7117:
	.loc	1 1176 33
	vmovss	3056(%r14), %xmm0
.Ltmp7118:
	.loc	1 1089 28
	vmovss	%xmm0, 1224(%rsp)
.Ltmp7119:
	.loc	1 1176 33
	vmovss	3216(%r14), %xmm0
.Ltmp7120:
	.loc	1 1089 28
	vmovss	%xmm0, 1228(%rsp)
.Ltmp7121:
	.loc	1 1176 33
	vmovss	3376(%r14), %xmm0
.Ltmp7122:
	.loc	1 1089 28
	vmovss	%xmm0, 1232(%rsp)
.Ltmp7123:
	.loc	1 1176 33
	vmovss	3536(%r14), %xmm0
.Ltmp7124:
	.loc	1 1089 28
	vmovss	%xmm0, 1236(%rsp)
.Ltmp7125:
	.loc	1 1176 33
	vmovss	3696(%r14), %xmm0
.Ltmp7126:
	.loc	1 1089 28
	vmovss	%xmm0, 1240(%rsp)
.Ltmp7127:
	.loc	1 1176 33
	vmovss	3856(%r14), %xmm0
.Ltmp7128:
	.loc	1 1089 28
	vmovss	%xmm0, 1244(%rsp)
.Ltmp7129:
	.loc	1 1176 33
	vmovss	2752(%r14), %xmm0
.Ltmp7130:
	.loc	1 1089 28
	vmovss	%xmm0, 1248(%rsp)
.Ltmp7131:
	.loc	1 1176 33
	vmovss	2912(%r14), %xmm0
.Ltmp7132:
	.loc	1 1089 28
	vmovss	%xmm0, 1252(%rsp)
.Ltmp7133:
	.loc	1 1176 33
	vmovss	3072(%r14), %xmm0
.Ltmp7134:
	.loc	1 1089 28
	vmovss	%xmm0, 1256(%rsp)
.Ltmp7135:
	.loc	1 1176 33
	vmovss	3232(%r14), %xmm0
.Ltmp7136:
	.loc	1 1089 28
	vmovss	%xmm0, 1260(%rsp)
.Ltmp7137:
	.loc	1 1176 33
	vmovss	3392(%r14), %xmm0
.Ltmp7138:
	.loc	1 1089 28
	vmovss	%xmm0, 1264(%rsp)
.Ltmp7139:
	.loc	1 1176 33
	vmovss	3552(%r14), %xmm0
.Ltmp7140:
	.loc	1 1089 28
	vmovss	%xmm0, 1268(%rsp)
.Ltmp7141:
	.loc	1 1176 33
	vmovss	3712(%r14), %xmm0
.Ltmp7142:
	.loc	1 1089 28
	vmovss	%xmm0, 1272(%rsp)
.Ltmp7143:
	.loc	1 1176 33
	vmovss	3872(%r14), %xmm0
.Ltmp7144:
	.loc	1 1089 28
	vmovss	%xmm0, 1276(%rsp)
.Ltmp7145:
	.loc	1 1176 33
	vmovss	2768(%r14), %xmm0
.Ltmp7146:
	.loc	1 1089 28
	vmovss	%xmm0, 1280(%rsp)
.Ltmp7147:
	.loc	1 1176 33
	vmovss	2928(%r14), %xmm0
.Ltmp7148:
	.loc	1 1089 28
	vmovss	%xmm0, 1284(%rsp)
.Ltmp7149:
	.loc	1 1176 33
	vmovss	3088(%r14), %xmm0
.Ltmp7150:
	.loc	1 1089 28
	vmovss	%xmm0, 1288(%rsp)
.Ltmp7151:
	.loc	1 1176 33
	vmovss	3248(%r14), %xmm0
.Ltmp7152:
	.loc	1 1089 28
	vmovss	%xmm0, 1292(%rsp)
.Ltmp7153:
	.loc	1 1176 33
	vmovss	3408(%r14), %xmm0
.Ltmp7154:
	.loc	1 1089 28
	vmovss	%xmm0, 1296(%rsp)
.Ltmp7155:
	.loc	1 1176 33
	vmovss	3568(%r14), %xmm0
.Ltmp7156:
	.loc	1 1089 28
	vmovss	%xmm0, 1300(%rsp)
.Ltmp7157:
	.loc	1 1176 33
	vmovss	3728(%r14), %xmm0
.Ltmp7158:
	.loc	1 1089 28
	vmovss	%xmm0, 1304(%rsp)
.Ltmp7159:
	.loc	1 1176 33
	vmovss	3888(%r14), %xmm0
.Ltmp7160:
	.loc	1 1089 28
	vmovss	%xmm0, 1308(%rsp)
.Ltmp7161:
	.loc	1 1177 32
	vmovss	2632(%r14), %xmm0
.Ltmp7162:
	.loc	1 1089 28
	vmovss	%xmm0, 1312(%rsp)
.Ltmp7163:
	.loc	1 1177 32
	vmovss	2792(%r14), %xmm0
.Ltmp7164:
	.loc	1 1089 28
	vmovss	%xmm0, 1316(%rsp)
.Ltmp7165:
	.loc	1 1177 32
	vmovss	2952(%r14), %xmm0
.Ltmp7166:
	.loc	1 1089 28
	vmovss	%xmm0, 1320(%rsp)
.Ltmp7167:
	.loc	1 1177 32
	vmovss	3112(%r14), %xmm0
.Ltmp7168:
	.loc	1 1089 28
	vmovss	%xmm0, 1324(%rsp)
.Ltmp7169:
	.loc	1 1177 32
	vmovss	3272(%r14), %xmm0
.Ltmp7170:
	.loc	1 1089 28
	vmovss	%xmm0, 1328(%rsp)
.Ltmp7171:
	.loc	1 1177 32
	vmovss	3432(%r14), %xmm0
.Ltmp7172:
	.loc	1 1089 28
	vmovss	%xmm0, 1332(%rsp)
.Ltmp7173:
	.loc	1 1177 32
	vmovss	3592(%r14), %xmm0
.Ltmp7174:
	.loc	1 1089 28
	vmovss	%xmm0, 1336(%rsp)
.Ltmp7175:
	.loc	1 1177 32
	vmovss	3752(%r14), %xmm0
.Ltmp7176:
	.loc	1 1089 28
	vmovss	%xmm0, 1340(%rsp)
.Ltmp7177:
	.loc	1 1177 32
	vmovss	2648(%r14), %xmm0
.Ltmp7178:
	.loc	1 1089 28
	vmovss	%xmm0, 1344(%rsp)
.Ltmp7179:
	.loc	1 1177 32
	vmovss	2808(%r14), %xmm0
.Ltmp7180:
	.loc	1 1089 28
	vmovss	%xmm0, 1348(%rsp)
.Ltmp7181:
	.loc	1 1177 32
	vmovss	2968(%r14), %xmm0
.Ltmp7182:
	.loc	1 1089 28
	vmovss	%xmm0, 1352(%rsp)
.Ltmp7183:
	.loc	1 1177 32
	vmovss	3128(%r14), %xmm0
.Ltmp7184:
	.loc	1 1089 28
	vmovss	%xmm0, 1356(%rsp)
.Ltmp7185:
	.loc	1 1177 32
	vmovss	3288(%r14), %xmm0
.Ltmp7186:
	.loc	1 1089 28
	vmovss	%xmm0, 1360(%rsp)
.Ltmp7187:
	.loc	1 1177 32
	vmovss	3448(%r14), %xmm0
.Ltmp7188:
	.loc	1 1089 28
	vmovss	%xmm0, 1364(%rsp)
.Ltmp7189:
	.loc	1 1177 32
	vmovss	3608(%r14), %xmm0
.Ltmp7190:
	.loc	1 1089 28
	vmovss	%xmm0, 1368(%rsp)
.Ltmp7191:
	.loc	1 1177 32
	vmovss	3768(%r14), %xmm0
.Ltmp7192:
	.loc	1 1089 28
	vmovss	%xmm0, 1372(%rsp)
.Ltmp7193:
	.loc	1 1177 32
	vmovss	2664(%r14), %xmm0
.Ltmp7194:
	.loc	1 1089 28
	vmovss	%xmm0, 1376(%rsp)
.Ltmp7195:
	.loc	1 1177 32
	vmovss	2824(%r14), %xmm0
.Ltmp7196:
	.loc	1 1089 28
	vmovss	%xmm0, 1380(%rsp)
.Ltmp7197:
	.loc	1 1177 32
	vmovss	2984(%r14), %xmm0
.Ltmp7198:
	.loc	1 1089 28
	vmovss	%xmm0, 1384(%rsp)
.Ltmp7199:
	.loc	1 1177 32
	vmovss	3144(%r14), %xmm0
.Ltmp7200:
	.loc	1 1089 28
	vmovss	%xmm0, 1388(%rsp)
.Ltmp7201:
	.loc	1 1177 32
	vmovss	3304(%r14), %xmm0
.Ltmp7202:
	.loc	1 1089 28
	vmovss	%xmm0, 1392(%rsp)
.Ltmp7203:
	.loc	1 1177 32
	vmovss	3464(%r14), %xmm0
.Ltmp7204:
	.loc	1 1089 28
	vmovss	%xmm0, 1396(%rsp)
.Ltmp7205:
	.loc	1 1177 32
	vmovss	3624(%r14), %xmm0
.Ltmp7206:
	.loc	1 1089 28
	vmovss	%xmm0, 1400(%rsp)
.Ltmp7207:
	.loc	1 1177 32
	vmovss	3784(%r14), %xmm0
.Ltmp7208:
	.loc	1 1089 28
	vmovss	%xmm0, 1404(%rsp)
.Ltmp7209:
	.loc	1 1177 32
	vmovss	2680(%r14), %xmm0
.Ltmp7210:
	.loc	1 1089 28
	vmovss	%xmm0, 1408(%rsp)
.Ltmp7211:
	.loc	1 1177 32
	vmovss	2840(%r14), %xmm0
.Ltmp7212:
	.loc	1 1089 28
	vmovss	%xmm0, 1412(%rsp)
.Ltmp7213:
	.loc	1 1177 32
	vmovss	3000(%r14), %xmm0
.Ltmp7214:
	.loc	1 1089 28
	vmovss	%xmm0, 1416(%rsp)
.Ltmp7215:
	.loc	1 1177 32
	vmovss	3160(%r14), %xmm0
.Ltmp7216:
	.loc	1 1089 28
	vmovss	%xmm0, 1420(%rsp)
.Ltmp7217:
	.loc	1 1177 32
	vmovss	3320(%r14), %xmm0
.Ltmp7218:
	.loc	1 1089 28
	vmovss	%xmm0, 1424(%rsp)
.Ltmp7219:
	.loc	1 1177 32
	vmovss	3480(%r14), %xmm0
.Ltmp7220:
	.loc	1 1089 28
	vmovss	%xmm0, 1428(%rsp)
.Ltmp7221:
	.loc	1 1177 32
	vmovss	3640(%r14), %xmm0
.Ltmp7222:
	.loc	1 1089 28
	vmovss	%xmm0, 1432(%rsp)
.Ltmp7223:
	.loc	1 1177 32
	vmovss	3800(%r14), %xmm0
.Ltmp7224:
	.loc	1 1089 28
	vmovss	%xmm0, 1436(%rsp)
.Ltmp7225:
	.loc	1 1177 32
	vmovss	2696(%r14), %xmm0
.Ltmp7226:
	.loc	1 1089 28
	vmovss	%xmm0, 1440(%rsp)
.Ltmp7227:
	.loc	1 1177 32
	vmovss	2856(%r14), %xmm0
.Ltmp7228:
	.loc	1 1089 28
	vmovss	%xmm0, 1444(%rsp)
.Ltmp7229:
	.loc	1 1177 32
	vmovss	3016(%r14), %xmm0
.Ltmp7230:
	.loc	1 1089 28
	vmovss	%xmm0, 1448(%rsp)
.Ltmp7231:
	.loc	1 1177 32
	vmovss	3176(%r14), %xmm0
.Ltmp7232:
	.loc	1 1089 28
	vmovss	%xmm0, 1452(%rsp)
.Ltmp7233:
	.loc	1 1177 32
	vmovss	3336(%r14), %xmm0
.Ltmp7234:
	.loc	1 1089 28
	vmovss	%xmm0, 1456(%rsp)
.Ltmp7235:
	.loc	1 1177 32
	vmovss	3496(%r14), %xmm0
.Ltmp7236:
	.loc	1 1089 28
	vmovss	%xmm0, 1460(%rsp)
.Ltmp7237:
	.loc	1 1177 32
	vmovss	3656(%r14), %xmm0
.Ltmp7238:
	.loc	1 1089 28
	vmovss	%xmm0, 1464(%rsp)
.Ltmp7239:
	.loc	1 1177 32
	vmovss	3816(%r14), %xmm0
.Ltmp7240:
	.loc	1 1089 28
	vmovss	%xmm0, 1468(%rsp)
.Ltmp7241:
	.loc	1 1177 32
	vmovss	2712(%r14), %xmm0
.Ltmp7242:
	.loc	1 1089 28
	vmovss	%xmm0, 1472(%rsp)
.Ltmp7243:
	.loc	1 1177 32
	vmovss	2872(%r14), %xmm0
.Ltmp7244:
	.loc	1 1089 28
	vmovss	%xmm0, 1476(%rsp)
.Ltmp7245:
	.loc	1 1177 32
	vmovss	3032(%r14), %xmm0
.Ltmp7246:
	.loc	1 1089 28
	vmovss	%xmm0, 1480(%rsp)
.Ltmp7247:
	.loc	1 1177 32
	vmovss	3192(%r14), %xmm0
.Ltmp7248:
	.loc	1 1089 28
	vmovss	%xmm0, 1484(%rsp)
.Ltmp7249:
	.loc	1 1177 32
	vmovss	3352(%r14), %xmm0
.Ltmp7250:
	.loc	1 1089 28
	vmovss	%xmm0, 1488(%rsp)
.Ltmp7251:
	.loc	1 1177 32
	vmovss	3512(%r14), %xmm0
.Ltmp7252:
	.loc	1 1089 28
	vmovss	%xmm0, 1492(%rsp)
.Ltmp7253:
	.loc	1 1177 32
	vmovss	3672(%r14), %xmm0
.Ltmp7254:
	.loc	1 1089 28
	vmovss	%xmm0, 1496(%rsp)
.Ltmp7255:
	.loc	1 1177 32
	vmovss	3832(%r14), %xmm0
.Ltmp7256:
	.loc	1 1089 28
	vmovss	%xmm0, 1500(%rsp)
.Ltmp7257:
	.loc	1 1177 32
	vmovss	2728(%r14), %xmm0
.Ltmp7258:
	.loc	1 1089 28
	vmovss	%xmm0, 1504(%rsp)
.Ltmp7259:
	.loc	1 1177 32
	vmovss	2888(%r14), %xmm0
.Ltmp7260:
	.loc	1 1089 28
	vmovss	%xmm0, 1508(%rsp)
.Ltmp7261:
	.loc	1 1177 32
	vmovss	3048(%r14), %xmm0
.Ltmp7262:
	.loc	1 1089 28
	vmovss	%xmm0, 1512(%rsp)
.Ltmp7263:
	.loc	1 1177 32
	vmovss	3208(%r14), %xmm0
.Ltmp7264:
	.loc	1 1089 28
	vmovss	%xmm0, 1516(%rsp)
.Ltmp7265:
	.loc	1 1177 32
	vmovss	3368(%r14), %xmm0
.Ltmp7266:
	.loc	1 1089 28
	vmovss	%xmm0, 1520(%rsp)
.Ltmp7267:
	.loc	1 1177 32
	vmovss	3528(%r14), %xmm0
.Ltmp7268:
	.loc	1 1089 28
	vmovss	%xmm0, 1524(%rsp)
.Ltmp7269:
	.loc	1 1177 32
	vmovss	3688(%r14), %xmm0
.Ltmp7270:
	.loc	1 1089 28
	vmovss	%xmm0, 1528(%rsp)
.Ltmp7271:
	.loc	1 1177 32
	vmovss	3848(%r14), %xmm0
.Ltmp7272:
	.loc	1 1089 28
	vmovss	%xmm0, 1532(%rsp)
.Ltmp7273:
	.loc	1 1177 32
	vmovss	2744(%r14), %xmm0
.Ltmp7274:
	.loc	1 1089 28
	vmovss	%xmm0, 1536(%rsp)
.Ltmp7275:
	.loc	1 1177 32
	vmovss	2904(%r14), %xmm0
.Ltmp7276:
	.loc	1 1089 28
	vmovss	%xmm0, 1540(%rsp)
.Ltmp7277:
	.loc	1 1177 32
	vmovss	3064(%r14), %xmm0
.Ltmp7278:
	.loc	1 1089 28
	vmovss	%xmm0, 1544(%rsp)
.Ltmp7279:
	.loc	1 1177 32
	vmovss	3224(%r14), %xmm0
.Ltmp7280:
	.loc	1 1089 28
	vmovss	%xmm0, 1548(%rsp)
.Ltmp7281:
	.loc	1 1177 32
	vmovss	3384(%r14), %xmm0
.Ltmp7282:
	.loc	1 1089 28
	vmovss	%xmm0, 1552(%rsp)
.Ltmp7283:
	.loc	1 1177 32
	vmovss	3544(%r14), %xmm0
.Ltmp7284:
	.loc	1 1089 28
	vmovss	%xmm0, 1556(%rsp)
.Ltmp7285:
	.loc	1 1177 32
	vmovss	3704(%r14), %xmm0
.Ltmp7286:
	.loc	1 1089 28
	vmovss	%xmm0, 1560(%rsp)
.Ltmp7287:
	.loc	1 1177 32
	vmovss	3864(%r14), %xmm0
.Ltmp7288:
	.loc	1 1089 28
	vmovss	%xmm0, 1564(%rsp)
.Ltmp7289:
	.loc	1 1177 32
	vmovss	2760(%r14), %xmm0
.Ltmp7290:
	.loc	1 1089 28
	vmovss	%xmm0, 1568(%rsp)
.Ltmp7291:
	.loc	1 1177 32
	vmovss	2920(%r14), %xmm0
.Ltmp7292:
	.loc	1 1089 28
	vmovss	%xmm0, 1572(%rsp)
.Ltmp7293:
	.loc	1 1177 32
	vmovss	3080(%r14), %xmm0
.Ltmp7294:
	.loc	1 1089 28
	vmovss	%xmm0, 1576(%rsp)
.Ltmp7295:
	.loc	1 1177 32
	vmovss	3240(%r14), %xmm0
.Ltmp7296:
	.loc	1 1089 28
	vmovss	%xmm0, 1580(%rsp)
.Ltmp7297:
	.loc	1 1177 32
	vmovss	3400(%r14), %xmm0
.Ltmp7298:
	.loc	1 1089 28
	vmovss	%xmm0, 1584(%rsp)
.Ltmp7299:
	.loc	1 1177 32
	vmovss	3560(%r14), %xmm0
.Ltmp7300:
	.loc	1 1089 28
	vmovss	%xmm0, 1588(%rsp)
.Ltmp7301:
	.loc	1 1177 32
	vmovss	3720(%r14), %xmm0
.Ltmp7302:
	.loc	1 1089 28
	vmovss	%xmm0, 1592(%rsp)
.Ltmp7303:
	.loc	1 1177 32
	vmovss	3880(%r14), %xmm0
.Ltmp7304:
	.loc	1 1089 28
	vmovss	%xmm0, 1596(%rsp)
.Ltmp7305:
	.loc	1 1177 32
	vmovss	2776(%r14), %xmm0
.Ltmp7306:
	.loc	1 1089 28
	vmovss	%xmm0, 1600(%rsp)
.Ltmp7307:
	.loc	1 1177 32
	vmovss	2936(%r14), %xmm0
.Ltmp7308:
	.loc	1 1089 28
	vmovss	%xmm0, 1604(%rsp)
.Ltmp7309:
	.loc	1 1177 32
	vmovss	3096(%r14), %xmm0
.Ltmp7310:
	.loc	1 1089 28
	vmovss	%xmm0, 1608(%rsp)
.Ltmp7311:
	.loc	1 1177 32
	vmovss	3256(%r14), %xmm0
.Ltmp7312:
	.loc	1 1089 28
	vmovss	%xmm0, 1612(%rsp)
.Ltmp7313:
	.loc	1 1177 32
	vmovss	3416(%r14), %xmm0
.Ltmp7314:
	.loc	1 1089 28
	vmovss	%xmm0, 1616(%rsp)
.Ltmp7315:
	.loc	1 1177 32
	vmovss	3576(%r14), %xmm0
.Ltmp7316:
	.loc	1 1089 28
	vmovss	%xmm0, 1620(%rsp)
.Ltmp7317:
	.loc	1 1177 32
	vmovss	3736(%r14), %xmm0
.Ltmp7318:
	.loc	1 1089 28
	vmovss	%xmm0, 1624(%rsp)
.Ltmp7319:
	.loc	1 1177 32
	vmovss	3896(%r14), %xmm0
.Ltmp7320:
	.loc	1 1089 28
	vmovss	%xmm0, 1628(%rsp)
.Ltmp7321:
	.loc	1 1091 31
	leaq	3008(%rsp), %rdi
	movq	%r14, %rsi
	movl	1808(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	3200(%rsp), %rdi
	leaq	2624(%r14), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovaps	3008(%rsp), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	3040(%rsp), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	3072(%rsp), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	3104(%rsp), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	3136(%rsp), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	3168(%rsp), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	3200(%rsp), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	3232(%rsp), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	3328(%rsp), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	3360(%rsp), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
.Ltmp7322:
	.loc	1 0 0 is_stmt 0
	leaq	(%r12,%r15), %rax
	shlq	$3, %r15
	leaq	(,%rax,8), %rsi
	.loc	1 1094 12 is_stmt 1
	testb	$1, %r13b
	movq	%r12, 312(%rsp)
	movq	%rax, 1816(%rsp)
	je	.LBB40_215
.Ltmp7323:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp7324:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_569
.Ltmp7325:
	.loc	48 451 16 is_stmt 1
	cmpq	40(%rsp), %rsi
	ja	.LBB40_569
.Ltmp7326:
	.loc	48 451 16 is_stmt 0
	cmpq	32(%rsp), %rsi
	ja	.LBB40_573
.Ltmp7327:
	.loc	1 972 27 is_stmt 1
	vmovaps	1408(%r14), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1440(%r14), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1472(%r14), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1504(%r14), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp7328:
	.loc	1 973 26
	vmovaps	4032(%r14), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	4064(%r14), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r14), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	4128(%r14), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
.Ltmp7329:
	.loc	1 974 25
	vmovaps	2304(%r14), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	2336(%r14), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
.Ltmp7330:
	.loc	1 975 24
	vmovaps	4928(%r14), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	4960(%r14), %ymm6
.Ltmp7331:
	.loc	1 976 24
	movq	5272(%r14), %r13
.Ltmp7332:
	.loc	2 1916 50
	testq	%r12, %r12
	movq	2016(%rsp), %rbx
	je	.LBB40_211
.Ltmp7333:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r12,8), %rax
	movq	%rax, 2048(%rsp)
	movq	296(%rsp), %rax
	leaq	(%rax,%r15,4), %r10
	movq	304(%rsp), %rax
	leaq	(%rax,%r15,4), %r15
.Ltmp7334:
	.loc	3 900 12 is_stmt 1
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r12
	movq	%r12, 2176(%rsp)
	xorl	%edi, %edi
	xorl	%r11d, %r11d
	movq	%r10, 2144(%rsp)
	movq	%r15, 56(%rsp)
.Ltmp7335:
	.loc	3 0 12 is_stmt 0
.Ltmp7336:
	.p2align	4
.LBB40_160:
	.loc	1 981 21 is_stmt 1
	vmovaps	352(%rsp), %ymm0
	vmovaps	384(%rsp), %ymm1
	vmovaps	416(%rsp), %ymm2
.Ltmp7337:
	.loc	14 48 14
	vaddps	672(%rsp), %ymm0, %ymm0
.Ltmp7338:
	.loc	1 980 17
	vmovaps	%ymm0, 352(%rsp)
	.loc	1 983 21
	vmovaps	992(%rsp), %ymm0
.Ltmp7339:
	.loc	14 48 14
	vaddps	1312(%rsp), %ymm0, %ymm0
.Ltmp7340:
	.loc	1 982 17
	vmovaps	%ymm0, 992(%rsp)
.Ltmp7341:
	.loc	14 48 14
	vaddps	704(%rsp), %ymm1, %ymm0
.Ltmp7342:
	.loc	1 980 17
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 983 21
	vmovaps	1024(%rsp), %ymm0
.Ltmp7343:
	.loc	14 48 14
	vaddps	1344(%rsp), %ymm0, %ymm0
.Ltmp7344:
	.loc	1 982 17
	vmovaps	%ymm0, 1024(%rsp)
.Ltmp7345:
	.loc	14 48 14
	vaddps	736(%rsp), %ymm2, %ymm0
.Ltmp7346:
	.loc	1 980 17
	vmovaps	%ymm0, 416(%rsp)
	.loc	1 983 21
	vmovaps	1056(%rsp), %ymm0
.Ltmp7347:
	.loc	14 48 14
	vaddps	1376(%rsp), %ymm0, %ymm0
.Ltmp7348:
	.loc	1 982 17
	vmovaps	%ymm0, 1056(%rsp)
	.loc	1 981 21
	vmovaps	448(%rsp), %ymm0
.Ltmp7349:
	.loc	14 48 14
	vaddps	768(%rsp), %ymm0, %ymm0
.Ltmp7350:
	.loc	1 980 17
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 983 21
	vmovaps	1088(%rsp), %ymm0
.Ltmp7351:
	.loc	14 48 14
	vaddps	1408(%rsp), %ymm0, %ymm0
.Ltmp7352:
	.loc	1 982 17
	vmovaps	%ymm0, 1088(%rsp)
	.loc	1 981 21
	vmovaps	480(%rsp), %ymm0
.Ltmp7353:
	.loc	14 48 14
	vaddps	800(%rsp), %ymm0, %ymm0
.Ltmp7354:
	.loc	1 980 17
	vmovaps	%ymm0, 480(%rsp)
	.loc	1 983 21
	vmovaps	1120(%rsp), %ymm0
.Ltmp7355:
	.loc	14 48 14
	vaddps	1440(%rsp), %ymm0, %ymm0
.Ltmp7356:
	.loc	1 982 17
	vmovaps	%ymm0, 1120(%rsp)
	.loc	1 981 21
	vmovaps	512(%rsp), %ymm0
.Ltmp7357:
	.loc	14 48 14
	vaddps	832(%rsp), %ymm0, %ymm0
.Ltmp7358:
	.loc	1 980 17
	vmovaps	%ymm0, 512(%rsp)
	.loc	1 983 21
	vmovaps	1152(%rsp), %ymm0
.Ltmp7359:
	.loc	14 48 14
	vaddps	1472(%rsp), %ymm0, %ymm0
.Ltmp7360:
	.loc	1 982 17
	vmovaps	%ymm0, 1152(%rsp)
	.loc	1 981 21
	vmovaps	544(%rsp), %ymm0
.Ltmp7361:
	.loc	14 48 14
	vaddps	864(%rsp), %ymm0, %ymm0
.Ltmp7362:
	.loc	1 980 17
	vmovaps	%ymm0, 544(%rsp)
	.loc	1 983 21
	vmovaps	1184(%rsp), %ymm0
.Ltmp7363:
	.loc	14 48 14
	vaddps	1504(%rsp), %ymm0, %ymm0
.Ltmp7364:
	.loc	1 982 17
	vmovaps	%ymm0, 1184(%rsp)
	.loc	1 981 21
	vmovaps	576(%rsp), %ymm0
.Ltmp7365:
	.loc	14 48 14
	vaddps	896(%rsp), %ymm0, %ymm0
.Ltmp7366:
	.loc	1 980 17
	vmovaps	%ymm0, 576(%rsp)
	.loc	1 983 21
	vmovaps	1216(%rsp), %ymm0
.Ltmp7367:
	.loc	14 48 14
	vaddps	1536(%rsp), %ymm0, %ymm0
.Ltmp7368:
	.loc	1 982 17
	vmovaps	%ymm0, 1216(%rsp)
	.loc	1 981 21
	vmovaps	608(%rsp), %ymm0
.Ltmp7369:
	.loc	14 48 14
	vaddps	928(%rsp), %ymm0, %ymm0
.Ltmp7370:
	.loc	1 980 17
	vmovaps	%ymm0, 608(%rsp)
	.loc	1 983 21
	vmovaps	1248(%rsp), %ymm0
.Ltmp7371:
	.loc	14 48 14
	vaddps	1568(%rsp), %ymm0, %ymm0
.Ltmp7372:
	.loc	1 982 17
	vmovaps	%ymm0, 1248(%rsp)
	.loc	1 981 21
	vmovaps	640(%rsp), %ymm0
.Ltmp7373:
	.loc	14 48 14
	vaddps	960(%rsp), %ymm0, %ymm0
.Ltmp7374:
	.loc	1 980 17
	vmovaps	%ymm0, 640(%rsp)
	.loc	1 983 21
	vmovaps	1280(%rsp), %ymm0
.Ltmp7375:
	.loc	14 48 14
	vaddps	1600(%rsp), %ymm0, %ymm0
.Ltmp7376:
	.loc	1 982 17
	vmovaps	%ymm0, 1280(%rsp)
.Ltmp7377:
	.loc	1 987 28
	leaq	1(%r13), %rax
.Ltmp7378:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %r9d
	cmovaeq	%rbx, %r9
.Ltmp7379:
	.loc	48 568 12
	cmpq	2048(%rsp), %rdi
	ja	.LBB40_555
.Ltmp7380:
	.loc	48 438 16
	cmpq	%r11, 2176(%rsp)
	je	.LBB40_547
.Ltmp7381:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r13,8), %rax
.Ltmp7382:
	.loc	1 1000 29 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp7383:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_554
.Ltmp7384:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp7385:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm6, 2208(%rsp)
	vmovups	(%r10,%rdi,4), %ymm1
.Ltmp7386:
	vmovups	(%r15,%rdi,4), %ymm15
.Ltmp7387:
	vmovaps	1280(%r14), %ymm10
	vmovaps	1312(%r14), %ymm11
	vmovaps	1344(%r14), %ymm0
	vmovaps	3904(%r14), %ymm14
	vmovaps	3936(%r14), %ymm9
	vmovaps	3968(%r14), %ymm13
	vmovaps	1664(%rsp), %ymm6
	vsubps	%ymm6, %ymm1, %ymm2
	vmulps	%ymm2, %ymm11, %ymm3
	vmovaps	1760(%rsp), %ymm7
	vmulps	%ymm7, %ymm10, %ymm4
	vaddps	%ymm3, %ymm4, %ymm5
	vaddps	%ymm5, %ymm7, %ymm3
	vmulps	%ymm7, %ymm11, %ymm4
	vmulps	%ymm0, %ymm2, %ymm2
	vaddps	%ymm2, %ymm4, %ymm4
	vaddps	%ymm4, %ymm6, %ymm2
	vmulps	1376(%r14), %ymm3, %ymm12
	vmovaps	1696(%rsp), %ymm6
	vsubps	%ymm6, %ymm2, %ymm3
	vmulps	1728(%rsp), %ymm11, %ymm2
	vmulps	%ymm3, %ymm0, %ymm0
	vaddps	%ymm0, %ymm2, %ymm0
	vaddps	%ymm0, %ymm6, %ymm2
.Ltmp7388:
	vsubps	192(%rsp), %ymm15, %ymm6
	vmulps	%ymm6, %ymm9, %ymm7
	vmovaps	256(%rsp), %ymm8
	vmovaps	%ymm14, 1952(%rsp)
	vmulps	%ymm14, %ymm8, %ymm14
	vaddps	%ymm7, %ymm14, %ymm14
	vaddps	%ymm14, %ymm8, %ymm7
	vmulps	4000(%r14), %ymm7, %ymm7
.Ltmp7389:
	.loc	1 1000 29 is_stmt 1
	movq	2592(%r14), %rcx
.Ltmp7390:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp7391:
	.loc	1 1001 30
	movq	2616(%r14), %rsi
.Ltmp7392:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_559
.Ltmp7393:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp7394:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm1, %ymm12, %ymm1
	vsubps	%ymm2, %ymm1, %ymm1
.Ltmp7395:
	.loc	1 1001 30 is_stmt 1
	movq	2608(%r14), %rcx
.Ltmp7396:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rax,4)
.Ltmp7397:
	.loc	1 1002 28
	movq	5224(%r14), %rsi
.Ltmp7398:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_560
.Ltmp7399:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp7400:
	.loc	1 0 0 is_stmt 0
	vmulps	256(%rsp), %ymm9, %ymm1
	vmulps	%ymm6, %ymm13, %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
	vaddps	192(%rsp), %ymm1, %ymm2
	vmovaps	1920(%rsp), %ymm8
	vsubps	%ymm8, %ymm2, %ymm6
	vmulps	320(%rsp), %ymm9, %ymm2
	vmulps	%ymm6, %ymm13, %ymm12
	vaddps	%ymm2, %ymm12, %ymm13
	vaddps	%ymm13, %ymm8, %ymm2
.Ltmp7401:
	.loc	1 1002 28 is_stmt 1
	movq	5216(%r14), %rcx
.Ltmp7402:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp7403:
	.loc	1 1003 29
	movq	5240(%r14), %rsi
.Ltmp7404:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_561
.Ltmp7405:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp7406:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm7, %ymm15, %ymm7
	vsubps	%ymm2, %ymm7, %ymm2
.Ltmp7407:
	.loc	1 1003 29 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp7408:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
	movq	2368(%r14), %rcx
.Ltmp7409:
	.loc	1 877 35
	addq	%r13, %rcx
.Ltmp7410:
	.loc	1 857 8
	cmpq	%rbx, %rcx
	movl	$0, %eax
	cmovaeq	%rbx, %rax
.Ltmp7411:
	.loc	1 1006 34
	movq	2600(%r14), %rsi
.Ltmp7412:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp7413:
	.loc	1 877 30
	shlq	$3, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	.loc	1 0 25
	movq	2376(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7414:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7415:
	.loc	1 877 30
	leaq	1(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_591
	.loc	1 0 25
	movq	2384(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7416:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7417:
	.loc	1 877 30
	leaq	2(,%rax,8), %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_588
	.loc	1 0 25
	movq	2392(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7418:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7419:
	.loc	1 877 30
	leaq	3(,%rax,8), %rax
	movq	%rax, 64(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_587
	.loc	1 0 25
	movq	2400(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7420:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7421:
	.loc	1 877 30
	leaq	4(,%rax,8), %rax
	movq	%rax, 160(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_595
	.loc	1 0 25
	movq	2408(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7422:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7423:
	.loc	1 877 30
	leaq	5(,%rax,8), %rax
	movq	%rax, 224(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_597
	.loc	1 0 25
	movq	2416(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7424:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7425:
	.loc	1 877 30
	leaq	6(,%rax,8), %rax
	movq	%rax, 24(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_594
	.loc	1 0 25
	movq	2424(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7426:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7427:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp7428:
	.loc	1 1008 34 is_stmt 1
	movq	2616(%r14), %rsi
.Ltmp7429:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	cmpq	%rsi, %r12
	jae	.LBB40_591
	cmpq	%rsi, %r8
	jae	.LBB40_588
	cmpq	%rsi, 64(%rsp)
	jae	.LBB40_619
	cmpq	%rsi, 160(%rsp)
	jae	.LBB40_641
	cmpq	%rsi, 224(%rsp)
	jae	.LBB40_597
	cmpq	%rsi, 24(%rsp)
	jae	.LBB40_594
	.loc	1 0 25 is_stmt 0
	movq	%r8, 2080(%rsp)
	.loc	1 877 25
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp7430:
	.loc	1 0 25
	movq	%rax, 152(%rsp)
	movq	4992(%r14), %r8
.Ltmp7431:
	.loc	1 877 35
	addq	%r13, %r8
.Ltmp7432:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r8
	movl	$0, %eax
	cmovaeq	%rbx, %rax
.Ltmp7433:
	.loc	1 1010 34
	movq	5224(%r14), %rsi
.Ltmp7434:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp7435:
	.loc	1 877 30
	shlq	$3, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_575
	.loc	1 0 25
	movq	%r11, 144(%rsp)
	movq	5000(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7436:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7437:
	.loc	1 877 30
	leaq	1(,%rax,8), %rax
	movq	%rax, 16(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_574
	.loc	1 0 25
	movq	5008(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7438:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7439:
	.loc	1 877 30
	leaq	2(,%rax,8), %rax
	movq	%rax, 1824(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_600
	.loc	1 0 25
	movq	%r12, 136(%rsp)
	movq	5016(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7440:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7441:
	.loc	1 877 30
	leaq	3(,%rax,8), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_606
	.loc	1 0 25
	movq	%rdi, 128(%rsp)
	movq	5024(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7442:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp7443:
	.loc	1 877 30
	leaq	4(,%rax,8), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_607
	.loc	1 0 25
	movq	5032(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7444:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp7445:
	.loc	1 877 30
	leaq	5(,%rax,8), %r10
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r10
	jae	.LBB40_605
	.loc	1 0 25
	movq	5040(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7446:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp7447:
	.loc	1 877 30
	leaq	6(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_586
	.loc	1 0 25
	movq	5048(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp7448:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp7449:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_589
.Ltmp7450:
	.loc	1 1012 34 is_stmt 1
	movq	5240(%r14), %rsi
.Ltmp7451:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB40_575
	cmpq	%rsi, 16(%rsp)
	jae	.LBB40_574
	cmpq	%rsi, 1824(%rsp)
	jae	.LBB40_596
	cmpq	%rsi, %rdx
	jae	.LBB40_606
	cmpq	%rsi, %r15
	jae	.LBB40_598
	cmpq	%rsi, %r10
	jae	.LBB40_590
	cmpq	%rsi, %r12
	jae	.LBB40_586
	cmpq	%rsi, %rax
	jae	.LBB40_589
.Ltmp7452:
	.loc	1 0 0 is_stmt 0
	negq	%r9
	leaq	(%r9,%r13), %r11
	incq	%r11
	movq	%r11, 120(%rsp)
	leaq	(,%r11,8), %rdi
.Ltmp7453:
	.loc	1 1047 36 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp7454:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_562
.Ltmp7455:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp7456:
	.loc	1 1049 27
	movq	2616(%r14), %rsi
.Ltmp7457:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_563
.Ltmp7458:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp7459:
	.loc	1 1050 35
	movq	5224(%r14), %rsi
.Ltmp7460:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_564
.Ltmp7461:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp7462:
	.loc	1 1052 27
	movq	5240(%r14), %rsi
.Ltmp7463:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_565
.Ltmp7464:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp7465:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm5, %ymm5, %ymm2
	vaddps	1760(%rsp), %ymm2, %ymm2
	vbroadcastss	.LCPI40_1(%rip), %ymm5
	vandps	%ymm5, %ymm2, %ymm7
	vbroadcastss	.LCPI40_2(%rip), %ymm12
	vcmplt_oqps	%ymm12, %ymm7, %ymm7
	vandnps	%ymm2, %ymm7, %ymm2
	vmovaps	%ymm2, 1760(%rsp)
	vaddps	%ymm4, %ymm4, %ymm2
	vaddps	1664(%rsp), %ymm2, %ymm2
	vandps	%ymm5, %ymm2, %ymm4
	vcmplt_oqps	%ymm12, %ymm4, %ymm4
	vandnps	%ymm2, %ymm4, %ymm2
	vmovaps	%ymm2, 1664(%rsp)
	vmulps	%ymm3, %ymm11, %ymm2
	vmovaps	1728(%rsp), %ymm4
	vmulps	%ymm4, %ymm10, %ymm3
	vaddps	%ymm2, %ymm3, %ymm2
	vaddps	%ymm2, %ymm2, %ymm2
	vaddps	%ymm2, %ymm4, %ymm2
	vandps	%ymm5, %ymm2, %ymm3
	vcmplt_oqps	%ymm12, %ymm3, %ymm3
	vandnps	%ymm2, %ymm3, %ymm2
	vmovaps	%ymm2, 1728(%rsp)
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	1696(%rsp), %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm2
	vcmplt_oqps	%ymm12, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp7466:
	vaddps	%ymm14, %ymm14, %ymm0
	vaddps	256(%rsp), %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm2
	vcmplt_oqps	%ymm12, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vaddps	%ymm1, %ymm1, %ymm0
	vaddps	192(%rsp), %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm1
	vcmplt_oqps	%ymm12, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmulps	%ymm6, %ymm9, %ymm0
	vmovaps	320(%rsp), %ymm2
	vmulps	1952(%rsp), %ymm2, %ymm1
	vaddps	%ymm0, %ymm1, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
.Ltmp7467:
	movq	2592(%r14), %rsi
	movq	160(%rsp), %r11
	vmovd	(%rsi,%r11,4), %xmm1
	movq	%r10, 104(%rsp)
	movq	%rdx, 112(%rsp)
	movq	224(%rsp), %rdx
	vpinsrd	$1, (%rsi,%rdx,4), %xmm1, %xmm1
	movq	%r12, %r10
	movq	24(%rsp), %r12
	vpinsrd	$2, (%rsi,%r12,4), %xmm1, %xmm1
.Ltmp7468:
	vaddps	%ymm0, %ymm2, %ymm0
	movq	%r15, %r13
	movq	152(%rsp), %r15
.Ltmp7469:
	vpinsrd	$3, (%rsi,%r15,4), %xmm1, %xmm1
	vmovd	(%rsi,%rcx,4), %xmm2
	movq	136(%rsp), %r9
	vpinsrd	$1, (%rsi,%r9,4), %xmm2, %xmm2
	movq	2080(%rsp), %rbx
	vpinsrd	$2, (%rsi,%rbx,4), %xmm2, %xmm2
	movq	%rdi, 1952(%rsp)
	movq	%rax, %rdi
	movq	64(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm2, %xmm2
.Ltmp7470:
	vandps	%ymm5, %ymm0, %ymm3
	vcmplt_oqps	%ymm12, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vaddps	%ymm13, %ymm13, %ymm0
	vaddps	%ymm0, %ymm8, %ymm0
	vandps	%ymm5, %ymm0, %ymm3
	vcmplt_oqps	%ymm12, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm0
	vmovaps	%ymm0, 1920(%rsp)
.Ltmp7471:
	movq	2608(%r14), %rsi
	vmovd	(%rsi,%r11,4), %xmm0
	vpinsrd	$1, (%rsi,%rdx,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%r12,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r15,4), %xmm0, %xmm0
.Ltmp7472:
	vinserti128	$1, %xmm1, %ymm2, %ymm1
.Ltmp7473:
	vmovd	(%rsi,%rcx,4), %xmm2
	vpinsrd	$1, (%rsi,%r9,4), %xmm2, %xmm2
	vpinsrd	$2, (%rsi,%rbx,4), %xmm2, %xmm2
	vpinsrd	$3, (%rsi,%rax,4), %xmm2, %xmm2
.Ltmp7474:
	movq	5216(%r14), %rcx
	vmovd	(%rcx,%r13,4), %xmm3
	movq	104(%rsp), %r9
	vpinsrd	$1, (%rcx,%r9,4), %xmm3, %xmm3
	vpinsrd	$2, (%rcx,%r10,4), %xmm3, %xmm3
.Ltmp7475:
	vinserti128	$1, %xmm0, %ymm2, %ymm0
.Ltmp7476:
	vpinsrd	$3, (%rcx,%rdi,4), %xmm3, %xmm2
	vmovd	(%rcx,%r8,4), %xmm3
	movq	16(%rsp), %rax
	vpinsrd	$1, (%rcx,%rax,4), %xmm3, %xmm3
	movq	1824(%rsp), %rdx
	vpinsrd	$2, (%rcx,%rdx,4), %xmm3, %xmm3
	movq	112(%rsp), %rsi
	vpinsrd	$3, (%rcx,%rsi,4), %xmm3, %xmm3
.Ltmp7477:
	movq	5232(%r14), %rcx
	vmovd	(%rcx,%r13,4), %xmm4
	vpinsrd	$1, (%rcx,%r9,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%r10,4), %xmm4, %xmm4
.Ltmp7478:
	vinserti128	$1, %xmm2, %ymm3, %ymm2
.Ltmp7479:
	vpinsrd	$3, (%rcx,%rdi,4), %xmm4, %xmm3
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rax,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rsi,4), %xmm4, %xmm4
	vinserti128	$1, %xmm3, %ymm4, %ymm3
.Ltmp7480:
	vpand	%ymm5, %ymm1, %ymm1
	vpand	%ymm5, %ymm2, %ymm2
	vmaxps	%ymm2, %ymm1, %ymm2
.Ltmp7481:
	vpand	%ymm5, %ymm0, %ymm0
	vpand	%ymm5, %ymm3, %ymm1
	vmaxps	%ymm1, %ymm0, %ymm1
	vbroadcastss	.LCPI40_4(%rip), %ymm9
.Ltmp7482:
	vmaxps	%ymm9, %ymm2, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm3
	vbroadcastss	.LCPI40_5(%rip), %ymm9
	vmaxps	%ymm9, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm2
	vbroadcastsd	.LCPI40_7(%rip), %ymm4
	vorps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_8(%rip), %ymm11
	vaddps	%ymm2, %ymm11, %ymm2
	vbroadcastss	.LCPI40_9(%rip), %ymm13
	vmulps	%ymm2, %ymm13, %ymm6
	vbroadcastss	.LCPI40_10(%rip), %ymm14
	vaddps	%ymm6, %ymm14, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_11(%rip), %ymm15
	vaddps	%ymm6, %ymm15, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_12(%rip), %ymm7
	vaddps	%ymm7, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_13(%rip), %ymm7
	vaddps	%ymm7, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_14(%rip), %ymm7
	vaddps	%ymm7, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm2
	vpsrld	$23, %ymm0, %ymm0
	vpbroadcastd	.LCPI40_15(%rip), %ymm6
	vpor	%ymm6, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm10
	vaddps	%ymm0, %ymm10, %ymm0
	vaddps	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm13
	vmulps	%ymm0, %ymm13, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm14
	vmaxps	%ymm14, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm15
	vminps	%ymm15, %ymm0, %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vsubps	352(%rsp), %ymm0, %ymm2
	vbroadcastss	.LCPI40_20(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm7
	vmulps	%ymm7, %ymm7, %ymm7
	vbroadcastss	.LCPI40_22(%rip), %ymm12
	vmulps	%ymm7, %ymm12, %ymm7
	vcmpgt_oqps	%ymm0, %ymm2, %ymm8
	vblendvps	%ymm8, %ymm2, %ymm7, %ymm7
	vbroadcastss	.LCPI40_21(%rip), %ymm11
	vcmple_oqps	%ymm11, %ymm2, %ymm2
	vmulps	2240(%rsp), %ymm7, %ymm7
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm2, %ymm0, %ymm2
	vpandn	%ymm7, %ymm2, %ymm2
	vbroadcastss	.LCPI40_23(%rip), %ymm7
	vmaxps	%ymm7, %ymm2, %ymm2
	vminps	%ymm0, %ymm2, %ymm2
	vmovaps	1984(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm2, %ymm7
	vmovaps	2560(%rsp), %ymm8
	vblendvps	%ymm7, 2592(%rsp), %ymm8, %ymm7
	vsubps	%ymm2, %ymm0, %ymm8
	vmulps	%ymm7, %ymm8, %ymm7
	vaddps	%ymm7, %ymm2, %ymm2
	vandps	%ymm5, %ymm2, %ymm7
	vbroadcastss	.LCPI40_2(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm7, %ymm7
	vandnps	%ymm2, %ymm7, %ymm0
.Ltmp7483:
	vbroadcastss	.LCPI40_4(%rip), %ymm2
	vmaxps	%ymm2, %ymm1, %ymm1
	vmaxps	%ymm9, %ymm1, %ymm1
	vandps	%ymm3, %ymm1, %ymm2
	vorps	%ymm4, %ymm2, %ymm2
	vpsrld	$23, %ymm1, %ymm1
	vpor	%ymm6, %ymm1, %ymm1
	vbroadcastss	.LCPI40_8(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_9(%rip), %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_10(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_11(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_12(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_13(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_14(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
	vaddps	%ymm1, %ymm10, %ymm1
	vaddps	%ymm2, %ymm1, %ymm1
	vmulps	%ymm1, %ymm13, %ymm1
	vmaxps	%ymm14, %ymm1, %ymm1
	vminps	%ymm15, %ymm1, %ymm1
	vmovaps	%ymm1, 224(%rsp)
	vsubps	512(%rsp), %ymm1, %ymm1
	vbroadcastss	.LCPI40_20(%rip), %ymm3
	vaddps	%ymm3, %ymm1, %ymm2
	vmulps	%ymm2, %ymm2, %ymm2
	vmulps	%ymm2, %ymm12, %ymm2
	vcmpgt_oqps	%ymm3, %ymm1, %ymm4
	vblendvps	%ymm4, %ymm1, %ymm2, %ymm2
	vcmple_oqps	%ymm11, %ymm1, %ymm1
	vmulps	2528(%rsp), %ymm2, %ymm2
	vxorps	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%ymm1, %ymm3, %ymm1
	vpandn	%ymm2, %ymm1, %ymm1
	vmovaps	%ymm0, 1984(%rsp)
.Ltmp7484:
	vaddps	480(%rsp), %ymm0, %ymm2
	vbroadcastss	.LCPI40_24(%rip), %ymm9
	vmulps	%ymm2, %ymm9, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm11
	vmaxps	%ymm11, %ymm2, %ymm2
	vbroadcastss	.LCPI40_26(%rip), %ymm13
	vminps	%ymm13, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm14
	vmulps	%ymm2, %ymm14, %ymm6
	vbroadcastss	.LCPI40_28(%rip), %ymm15
	vaddps	%ymm6, %ymm15, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_29(%rip), %ymm12
	vaddps	%ymm6, %ymm12, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_30(%rip), %ymm11
	vaddps	%ymm6, %ymm11, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_31(%rip), %ymm10
	vaddps	%ymm6, %ymm10, %ymm6
	vbroadcastss	.LCPI40_23(%rip), %ymm14
.Ltmp7485:
	vmaxps	%ymm14, %ymm1, %ymm1
	vminps	%ymm3, %ymm1, %ymm1
	vmovaps	1856(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm1, %ymm7
	vmovaps	2464(%rsp), %ymm8
	vblendvps	%ymm7, 2496(%rsp), %ymm8, %ymm7
.Ltmp7486:
	vmulps	%ymm6, %ymm2, %ymm2
.Ltmp7487:
	vsubps	%ymm1, %ymm0, %ymm6
	vmulps	%ymm7, %ymm6, %ymm6
	vaddps	%ymm6, %ymm1, %ymm1
	vandps	%ymm5, %ymm1, %ymm6
	vbroadcastss	.LCPI40_2(%rip), %ymm13
	vcmplt_oqps	%ymm13, %ymm6, %ymm6
	vandnps	%ymm1, %ymm6, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm7
.Ltmp7488:
	vaddps	%ymm7, %ymm2, %ymm1
	vbroadcastss	.LCPI40_33(%rip), %ymm8
	vaddps	%ymm4, %ymm8, %ymm2
	vpslld	$23, %ymm2, %ymm2
	vmovaps	%ymm0, 1856(%rsp)
.Ltmp7489:
	vaddps	640(%rsp), %ymm0, %ymm4
.Ltmp7490:
	vmulps	%ymm2, %ymm1, %ymm0
	vmovaps	%ymm0, 160(%rsp)
.Ltmp7491:
	vmulps	%ymm4, %ymm9, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm3
	vmulps	%ymm3, %ymm2, %ymm6
	vaddps	%ymm6, %ymm15, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm12, %ymm6
	vmovaps	%ymm12, %ymm15
	vmulps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm11, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm10, %ymm6
	vmulps	%ymm6, %ymm2, %ymm2
	vaddps	%ymm7, %ymm2, %ymm2
	vaddps	%ymm4, %ymm8, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmovaps	64(%rsp), %ymm0
.Ltmp7492:
	vsubps	992(%rsp), %ymm0, %ymm6
.Ltmp7493:
	vmulps	%ymm4, %ymm2, %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm8
.Ltmp7494:
	vaddps	%ymm6, %ymm8, %ymm2
	vmulps	%ymm2, %ymm2, %ymm2
	vbroadcastss	.LCPI40_22(%rip), %ymm12
	vmulps	%ymm2, %ymm12, %ymm2
	vcmpgt_oqps	%ymm8, %ymm6, %ymm4
	vblendvps	%ymm4, %ymm6, %ymm2, %ymm2
	vbroadcastss	.LCPI40_21(%rip), %ymm11
	vcmple_oqps	%ymm11, %ymm6, %ymm4
	vmulps	2432(%rsp), %ymm2, %ymm2
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm4, %ymm0, %ymm4
	vpandn	%ymm2, %ymm4, %ymm2
	vmaxps	%ymm14, %ymm2, %ymm2
	vminps	%ymm0, %ymm2, %ymm2
	vxorps	%xmm10, %xmm10, %xmm10
	vmovaps	1888(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm2, %ymm4
	vmovaps	2368(%rsp), %ymm6
	vblendvps	%ymm4, 2400(%rsp), %ymm6, %ymm4
	vsubps	%ymm2, %ymm0, %ymm6
	vmulps	%ymm4, %ymm6, %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vandps	%ymm5, %ymm2, %ymm4
	vcmplt_oqps	%ymm13, %ymm4, %ymm4
	vandnps	%ymm2, %ymm4, %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vaddps	1120(%rsp), %ymm0, %ymm2
	vmulps	%ymm2, %ymm9, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm7
	vmaxps	%ymm7, %ymm2, %ymm2
	vminps	%ymm1, %ymm2, %ymm2
	vmovaps	%ymm1, %ymm13
	vroundps	$9, %ymm2, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm3, %ymm2, %ymm6
	vbroadcastss	.LCPI40_28(%rip), %ymm1
	vaddps	%ymm1, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm15, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_30(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_31(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm2
	vbroadcastss	.LCPI40_32(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_33(%rip), %ymm0
	vaddps	%ymm0, %ymm4, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmulps	%ymm4, %ymm2, %ymm4
	vmovaps	224(%rsp), %ymm0
.Ltmp7495:
	vsubps	1152(%rsp), %ymm0, %ymm2
	vaddps	%ymm2, %ymm8, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm12, %ymm3
	vcmpgt_oqps	%ymm8, %ymm2, %ymm6
	vblendvps	%ymm6, %ymm2, %ymm3, %ymm3
	vcmple_oqps	%ymm11, %ymm2, %ymm2
	vmulps	2336(%rsp), %ymm3, %ymm3
	vpcmpgtd	%ymm2, %ymm10, %ymm2
	vpandn	%ymm3, %ymm2, %ymm2
	vmaxps	%ymm14, %ymm2, %ymm2
	vminps	%ymm10, %ymm2, %ymm2
	vmovaps	2208(%rsp), %ymm6
	vcmplt_oqps	%ymm6, %ymm2, %ymm3
	vmovaps	2272(%rsp), %ymm0
	vblendvps	%ymm3, 2304(%rsp), %ymm0, %ymm3
	vsubps	%ymm2, %ymm6, %ymm6
	vmulps	%ymm3, %ymm6, %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vandps	%ymm5, %ymm2, %ymm3
	vbroadcastss	.LCPI40_2(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm3, %ymm3
	vandnps	%ymm2, %ymm3, %ymm6
	vaddps	1280(%rsp), %ymm6, %ymm2
	vmulps	%ymm2, %ymm9, %ymm2
	vmaxps	%ymm7, %ymm2, %ymm2
	vminps	%ymm13, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm3
	vsubps	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm0
	vmulps	%ymm0, %ymm2, %ymm5
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_30(%rip), %ymm0
	vaddps	%ymm0, %ymm5, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm0
	vaddps	%ymm0, %ymm5, %ymm5
	vmulps	%ymm5, %ymm2, %ymm2
	vbroadcastss	.LCPI40_32(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_33(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm3
	vpslld	$23, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
.Ltmp7496:
	movq	2592(%r14), %rcx
	movq	1952(%rsp), %rax
	vmovaps	160(%rsp), %ymm0
	vmulps	(%rcx,%rax,4), %ymm0, %ymm1
	movq	2608(%r14), %rcx
	vmovaps	64(%rsp), %ymm0
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
	vaddps	%ymm0, %ymm1, %ymm0
.Ltmp7497:
	movq	5216(%r14), %rcx
	vmulps	(%rcx,%rax,4), %ymm4, %ymm1
	.loc	1 1052 27 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp7498:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm2, %ymm2
.Ltmp7499:
	.loc	14 48 14
	vaddps	%ymm2, %ymm1, %ymm1
	movq	2144(%rsp), %r10
	movq	128(%rsp), %rdi
.Ltmp7500:
	.loc	8 551 14
	vmovups	%ymm0, (%r10,%rdi,4)
	movq	56(%rsp), %r15
.Ltmp7501:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm1, (%r15,%rdi,4)
	movq	144(%rsp), %r11
.Ltmp7502:
	.loc	1 0 0
	incq	%r11
.Ltmp7503:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	movq	312(%rsp), %r12
	cmpq	%r11, %r12
	movq	2016(%rsp), %rbx
	movq	120(%rsp), %r13
.Ltmp7504:
	.loc	3 900 12
	jne	.LBB40_160
.Ltmp7505:
.LBB40_211:
	.loc	3 0 12 is_stmt 0
	vmovaps	1760(%rsp), %ymm0
	.loc	1 1057 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r14)
	vmovaps	1664(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r14)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r14)
	vmovaps	1696(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r14)
	vmovaps	256(%rsp), %ymm0
	.loc	1 1058 5
	vmovaps	%ymm0, 4032(%r14)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r14)
	vmovaps	320(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r14)
	vmovaps	1920(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r14)
	vmovaps	1984(%rsp), %ymm0
	.loc	1 1059 5
	vmovaps	%ymm0, 2304(%r14)
	vmovaps	1856(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r14)
	vmovaps	1888(%rsp), %ymm0
	.loc	1 1060 5
	vmovaps	%ymm0, 4928(%r14)
	vmovaps	%ymm6, 4960(%r14)
	.loc	1 1061 5
	movq	%r13, 5272(%r14)
	xorl	%eax, %eax
	xorl	%edi, %edi
.Ltmp7506:
	.loc	1 0 5 is_stmt 0
.Ltmp7507:
	.p2align	4
.LBB40_212:
	.loc	1 1194 13 is_stmt 1
	vmovss	352(%rsp,%rax,2), %xmm3
	vmovss	356(%rsp,%rax,2), %xmm4
	vmovss	360(%rsp,%rax,2), %xmm5
	vmovss	364(%rsp,%rax,2), %xmm6
	vmovss	368(%rsp,%rax,2), %xmm7
	vmovss	372(%rsp,%rax,2), %xmm2
	vmovss	376(%rsp,%rax,2), %xmm1
	vmovd	380(%rsp,%rax,2), %xmm0
.Ltmp7508:
	.loc	1 1197 17
	vmovss	%xmm3, (%r14,%rax)
	.loc	1 1198 34
	movl	12(%r14,%rax), %ecx
	movl	172(%r14,%rax), %edx
.Ltmp7509:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7510:
	.loc	1 1198 17
	movl	%ecx, 12(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm4, 160(%r14,%rax)
.Ltmp7511:
	.loc	38 2472 13
	subl	%r12d, %edx
	cmovbl	%edi, %edx
.Ltmp7512:
	.loc	1 1198 17
	movl	%edx, 172(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm5, 320(%r14,%rax)
	.loc	1 1198 34
	movl	332(%r14,%rax), %ecx
.Ltmp7513:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7514:
	.loc	1 1198 17
	movl	%ecx, 332(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm6, 480(%r14,%rax)
	.loc	1 1198 34
	movl	492(%r14,%rax), %ecx
.Ltmp7515:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7516:
	.loc	1 1198 17
	movl	%ecx, 492(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm7, 640(%r14,%rax)
	.loc	1 1198 34
	movl	652(%r14,%rax), %ecx
.Ltmp7517:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7518:
	.loc	1 1198 17
	movl	%ecx, 652(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 800(%r14,%rax)
	.loc	1 1198 34
	movl	812(%r14,%rax), %ecx
.Ltmp7519:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7520:
	.loc	1 1198 17
	movl	%ecx, 812(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 960(%r14,%rax)
	.loc	1 1198 34
	movl	972(%r14,%rax), %ecx
.Ltmp7521:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7522:
	.loc	1 1198 17
	movl	%ecx, 972(%r14,%rax)
	.loc	1 1197 17
	vmovd	%xmm0, 1120(%r14,%rax)
	.loc	1 1198 34
	movl	1132(%r14,%rax), %ecx
.Ltmp7523:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7524:
	.loc	1 1198 17
	movl	%ecx, 1132(%r14,%rax)
.Ltmp7525:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp7526:
	.loc	3 900 12
	jne	.LBB40_212
.Ltmp7527:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	1656(%rsp), %rsi
	.p2align	4
.LBB40_214:
.Ltmp7528:
	.loc	1 1194 13 is_stmt 1
	vmovss	992(%rsp,%rax,2), %xmm3
	vmovss	996(%rsp,%rax,2), %xmm4
	vmovss	1000(%rsp,%rax,2), %xmm5
	vmovss	1004(%rsp,%rax,2), %xmm6
	vmovss	1008(%rsp,%rax,2), %xmm7
	vmovss	1012(%rsp,%rax,2), %xmm2
	vmovss	1016(%rsp,%rax,2), %xmm1
	vmovd	1020(%rsp,%rax,2), %xmm0
.Ltmp7529:
	.loc	1 1197 17
	vmovss	%xmm3, 2624(%r14,%rax)
	.loc	1 1198 34
	movl	2636(%r14,%rax), %ecx
	movl	2796(%r14,%rax), %edx
.Ltmp7530:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7531:
	.loc	1 1198 17
	movl	%ecx, 2636(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm4, 2784(%r14,%rax)
.Ltmp7532:
	.loc	38 2472 13
	subl	%r12d, %edx
	cmovbl	%edi, %edx
.Ltmp7533:
	.loc	1 1198 17
	movl	%edx, 2796(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm5, 2944(%r14,%rax)
	.loc	1 1198 34
	movl	2956(%r14,%rax), %ecx
.Ltmp7534:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7535:
	.loc	1 1198 17
	movl	%ecx, 2956(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm6, 3104(%r14,%rax)
	.loc	1 1198 34
	movl	3116(%r14,%rax), %ecx
.Ltmp7536:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7537:
	.loc	1 1198 17
	movl	%ecx, 3116(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm7, 3264(%r14,%rax)
	.loc	1 1198 34
	movl	3276(%r14,%rax), %ecx
.Ltmp7538:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7539:
	.loc	1 1198 17
	movl	%ecx, 3276(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 3424(%r14,%rax)
	.loc	1 1198 34
	movl	3436(%r14,%rax), %ecx
.Ltmp7540:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7541:
	.loc	1 1198 17
	movl	%ecx, 3436(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 3584(%r14,%rax)
	.loc	1 1198 34
	movl	3596(%r14,%rax), %ecx
.Ltmp7542:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7543:
	.loc	1 1198 17
	movl	%ecx, 3596(%r14,%rax)
	.loc	1 1197 17
	vmovd	%xmm0, 3744(%r14,%rax)
	.loc	1 1198 34
	movl	3756(%r14,%rax), %ecx
.Ltmp7544:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp7545:
	.loc	1 1198 17
	movl	%ecx, 3756(%r14,%rax)
.Ltmp7546:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp7547:
	.loc	3 900 12
	jne	.LBB40_214
	jmp	.LBB40_153
.Ltmp7548:
.LBB40_215:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp7549:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_568
.Ltmp7550:
	.loc	48 451 16 is_stmt 1
	cmpq	40(%rsp), %rsi
	ja	.LBB40_568
.Ltmp7551:
	.loc	48 451 16 is_stmt 0
	cmpq	32(%rsp), %rsi
	ja	.LBB40_572
.Ltmp7552:
	.loc	1 972 27 is_stmt 1
	vmovaps	1408(%r14), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1440(%r14), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1472(%r14), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1504(%r14), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp7553:
	.loc	1 973 26
	vmovaps	4032(%r14), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	4064(%r14), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r14), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	4128(%r14), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
.Ltmp7554:
	.loc	1 974 25
	vmovaps	2304(%r14), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	2336(%r14), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
.Ltmp7555:
	.loc	1 975 24
	vmovaps	4928(%r14), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	4960(%r14), %ymm8
.Ltmp7556:
	.loc	1 976 24
	movq	5272(%r14), %r10
.Ltmp7557:
	.loc	2 1916 50
	testq	%r12, %r12
	movq	2016(%rsp), %rbx
.Ltmp7558:
	.loc	3 900 12
	je	.LBB40_152
.Ltmp7559:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r12,8), %rax
	movq	%rax, 2048(%rsp)
	movq	296(%rsp), %rax
	leaq	(%rax,%r15,4), %r11
	movq	304(%rsp), %rax
	leaq	(%rax,%r15,4), %r15
.Ltmp7560:
	.loc	48 568 12 is_stmt 1
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r12
	movq	%r12, 2176(%rsp)
	xorl	%edi, %edi
	xorl	%r12d, %r12d
	movq	%r11, 2144(%rsp)
	movq	%r15, 56(%rsp)
.Ltmp7561:
	.loc	48 0 12 is_stmt 0
.Ltmp7562:
	.p2align	4
.LBB40_220:
	.loc	1 987 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp7563:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %r9d
	cmovaeq	%rbx, %r9
.Ltmp7564:
	.loc	48 568 12
	cmpq	2048(%rsp), %rdi
	ja	.LBB40_555
.Ltmp7565:
	.loc	48 438 16
	cmpq	%r12, 2176(%rsp)
	je	.LBB40_547
.Ltmp7566:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,8), %rax
.Ltmp7567:
	.loc	1 1000 29 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp7568:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_554
.Ltmp7569:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp7570:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm8, 2208(%rsp)
	vmovups	(%r11,%rdi,4), %ymm3
.Ltmp7571:
	vmovups	(%r15,%rdi,4), %ymm5
.Ltmp7572:
	vmovaps	1280(%r14), %ymm10
	vmovaps	1312(%r14), %ymm11
	vmovaps	1344(%r14), %ymm2
	vmovaps	3904(%r14), %ymm15
	vmovaps	3936(%r14), %ymm9
	vmovaps	3968(%r14), %ymm1
	vmovaps	1664(%rsp), %ymm7
	vsubps	%ymm7, %ymm3, %ymm0
	vmulps	%ymm0, %ymm11, %ymm4
	vmovaps	1760(%rsp), %ymm8
	vmulps	%ymm10, %ymm8, %ymm6
	vaddps	%ymm4, %ymm6, %ymm13
	vaddps	%ymm13, %ymm8, %ymm4
	vmulps	%ymm11, %ymm8, %ymm6
	vmulps	%ymm2, %ymm0, %ymm0
	vaddps	%ymm0, %ymm6, %ymm12
	vaddps	%ymm7, %ymm12, %ymm6
	vmulps	1376(%r14), %ymm4, %ymm0
	vmovaps	1696(%rsp), %ymm8
	vsubps	%ymm8, %ymm6, %ymm7
	vmulps	1728(%rsp), %ymm11, %ymm4
	vmulps	%ymm7, %ymm2, %ymm2
	vaddps	%ymm2, %ymm4, %ymm4
	vaddps	%ymm4, %ymm8, %ymm6
.Ltmp7573:
	vsubps	192(%rsp), %ymm5, %ymm14
	vmulps	%ymm9, %ymm14, %ymm2
	vmovaps	256(%rsp), %ymm8
	vmovaps	%ymm15, 1952(%rsp)
	vmulps	%ymm15, %ymm8, %ymm15
	vaddps	%ymm2, %ymm15, %ymm2
	vaddps	%ymm2, %ymm8, %ymm15
	vmulps	4000(%r14), %ymm15, %ymm15
.Ltmp7574:
	.loc	1 1000 29 is_stmt 1
	movq	2592(%r14), %rcx
.Ltmp7575:
	.loc	8 551 14
	vmovups	%ymm6, (%rcx,%rax,4)
.Ltmp7576:
	.loc	1 1001 30
	movq	2616(%r14), %rsi
.Ltmp7577:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_559
.Ltmp7578:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp7579:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm0, %ymm3, %ymm0
	vsubps	%ymm6, %ymm0, %ymm0
.Ltmp7580:
	.loc	1 1001 30 is_stmt 1
	movq	2608(%r14), %rcx
.Ltmp7581:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp7582:
	.loc	1 1002 28
	movq	5224(%r14), %rsi
.Ltmp7583:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_560
.Ltmp7584:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp7585:
	.loc	1 0 0 is_stmt 0
	vmulps	256(%rsp), %ymm9, %ymm0
	vmulps	%ymm1, %ymm14, %ymm3
	vaddps	%ymm3, %ymm0, %ymm3
	vaddps	192(%rsp), %ymm3, %ymm0
	vmovaps	1824(%rsp), %ymm6
	vsubps	%ymm6, %ymm0, %ymm14
	vmovaps	1920(%rsp), %ymm8
	vmulps	%ymm9, %ymm8, %ymm0
	vmulps	%ymm1, %ymm14, %ymm1
	vaddps	%ymm1, %ymm0, %ymm1
	vaddps	%ymm1, %ymm6, %ymm0
.Ltmp7586:
	.loc	1 1002 28 is_stmt 1
	movq	5216(%r14), %rcx
.Ltmp7587:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp7588:
	.loc	1 1003 29
	movq	5240(%r14), %rsi
.Ltmp7589:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_561
.Ltmp7590:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp7591:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm5, %ymm15, %ymm5
	vsubps	%ymm0, %ymm5, %ymm0
.Ltmp7592:
	.loc	1 1003 29 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp7593:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
	movq	2368(%r14), %rcx
.Ltmp7594:
	.loc	1 877 35
	addq	%r10, %rcx
.Ltmp7595:
	.loc	1 857 8
	cmpq	%rbx, %rcx
	movl	$0, %eax
	cmovaeq	%rbx, %rax
.Ltmp7596:
	.loc	1 1006 34
	movq	2600(%r14), %rsi
.Ltmp7597:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp7598:
	.loc	1 877 30
	shlq	$3, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	.loc	1 0 25
	movq	2376(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7599:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7600:
	.loc	1 877 30
	leaq	1(,%rax,8), %r13
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_578
	.loc	1 0 25
	movq	2384(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7601:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7602:
	.loc	1 877 30
	leaq	2(,%rax,8), %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_588
	.loc	1 0 25
	movq	2392(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7603:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7604:
	.loc	1 877 30
	leaq	3(,%rax,8), %rax
	movq	%rax, 320(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_583
	.loc	1 0 25
	movq	2400(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7605:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7606:
	.loc	1 877 30
	leaq	4(,%rax,8), %rax
	movq	%rax, 64(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_587
	.loc	1 0 25
	movq	2408(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7607:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7608:
	.loc	1 877 30
	leaq	5(,%rax,8), %rax
	movq	%rax, 160(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_595
	.loc	1 0 25
	movq	2416(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7609:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7610:
	.loc	1 877 30
	leaq	6(,%rax,8), %rax
	movq	%rax, 224(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_597
	.loc	1 0 25
	movq	2424(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7611:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7612:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp7613:
	.loc	1 1008 34 is_stmt 1
	movq	2616(%r14), %rsi
.Ltmp7614:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	cmpq	%rsi, %r13
	jae	.LBB40_578
	cmpq	%rsi, %r8
	jae	.LBB40_616
	cmpq	%rsi, 320(%rsp)
	jae	.LBB40_627
	cmpq	%rsi, 64(%rsp)
	jae	.LBB40_587
	cmpq	%rsi, 160(%rsp)
	jae	.LBB40_595
	cmpq	%rsi, 224(%rsp)
	jae	.LBB40_597
	.loc	1 0 25 is_stmt 0
	movq	%r8, 2080(%rsp)
	.loc	1 877 25
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp7615:
	.loc	1 0 25
	movq	%rax, 152(%rsp)
	movq	4992(%r14), %r8
.Ltmp7616:
	.loc	1 877 35
	addq	%r10, %r8
.Ltmp7617:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r8
	movl	$0, %eax
	cmovaeq	%rbx, %rax
.Ltmp7618:
	.loc	1 1010 34
	movq	5224(%r14), %rsi
.Ltmp7619:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp7620:
	.loc	1 877 30
	shlq	$3, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_575
	.loc	1 0 25
	movq	%r13, 144(%rsp)
	movq	5000(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7621:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7622:
	.loc	1 877 30
	leaq	1(,%rax,8), %rax
	movq	%rax, 24(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_601
	.loc	1 0 25
	movq	5008(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7623:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7624:
	.loc	1 877 30
	leaq	2(,%rax,8), %rax
	movq	%rax, 16(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_574
	.loc	1 0 25
	movq	%r12, 136(%rsp)
	movq	5016(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7625:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp7626:
	.loc	1 877 30
	leaq	3(,%rax,8), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_585
	.loc	1 0 25
	movq	%rdi, 128(%rsp)
	movq	5024(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7627:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp7628:
	.loc	1 877 30
	leaq	4(,%rax,8), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_607
	.loc	1 0 25
	movq	5032(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7629:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp7630:
	.loc	1 877 30
	leaq	5(,%rax,8), %r13
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_592
	.loc	1 0 25
	movq	5040(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7631:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp7632:
	.loc	1 877 30
	leaq	6(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_586
	.loc	1 0 25
	movq	5048(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp7633:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp7634:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_618
.Ltmp7635:
	.loc	1 1012 34 is_stmt 1
	movq	5240(%r14), %rsi
.Ltmp7636:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB40_575
	cmpq	%rsi, 24(%rsp)
	jae	.LBB40_593
	cmpq	%rsi, 16(%rsp)
	jae	.LBB40_574
	cmpq	%rsi, %rdx
	jae	.LBB40_606
	cmpq	%rsi, %r15
	jae	.LBB40_607
	cmpq	%rsi, %r13
	jae	.LBB40_624
	cmpq	%rsi, %r12
	jae	.LBB40_586
	cmpq	%rsi, %rax
	jae	.LBB40_589
.Ltmp7637:
	.loc	1 0 0 is_stmt 0
	negq	%r9
	addq	%r9, %r10
	incq	%r10
	leaq	(,%r10,8), %rdi
.Ltmp7638:
	.loc	1 1047 36 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp7639:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_562
.Ltmp7640:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp7641:
	.loc	1 1049 27
	movq	2616(%r14), %rsi
.Ltmp7642:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_563
.Ltmp7643:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp7644:
	.loc	1 1050 35
	movq	5224(%r14), %rsi
.Ltmp7645:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_564
.Ltmp7646:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp7647:
	.loc	1 1052 27
	movq	5240(%r14), %rsi
.Ltmp7648:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_565
.Ltmp7649:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp7650:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm13, %ymm13, %ymm0
	vaddps	1760(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_1(%rip), %ymm13
	vandps	%ymm0, %ymm13, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm15
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vaddps	%ymm12, %ymm12, %ymm0
	vaddps	1664(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm5
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmulps	%ymm7, %ymm11, %ymm0
	vmovaps	1728(%rsp), %ymm7
	vmulps	%ymm7, %ymm10, %ymm5
	vaddps	%ymm0, %ymm5, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	%ymm0, %ymm7, %ymm0
	vandps	%ymm0, %ymm13, %ymm5
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vaddps	%ymm4, %ymm4, %ymm0
	vaddps	1696(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm4
	vcmplt_oqps	%ymm15, %ymm4, %ymm4
	vandnps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp7651:
	vaddps	%ymm2, %ymm2, %ymm0
	vaddps	256(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm2
	vcmplt_oqps	%ymm15, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vaddps	%ymm3, %ymm3, %ymm0
	vaddps	192(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm2
	vcmplt_oqps	%ymm15, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmulps	%ymm14, %ymm9, %ymm0
	vmulps	1952(%rsp), %ymm8, %ymm2
	vaddps	%ymm0, %ymm2, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
.Ltmp7652:
	movq	2592(%r14), %rsi
	movq	%rdx, 120(%rsp)
	movq	%r10, %rdx
	movq	64(%rsp), %r10
	vmovd	(%rsi,%r10,4), %xmm2
	movq	%r12, 112(%rsp)
	movq	160(%rsp), %r12
	vpinsrd	$1, (%rsi,%r12,4), %xmm2, %xmm2
	movq	%r15, 104(%rsp)
	movq	224(%rsp), %r15
	vpinsrd	$2, (%rsi,%r15,4), %xmm2, %xmm2
.Ltmp7653:
	vaddps	%ymm0, %ymm8, %ymm0
	movq	152(%rsp), %r11
.Ltmp7654:
	vpinsrd	$3, (%rsi,%r11,4), %xmm2, %xmm2
	vmovd	(%rsi,%rcx,4), %xmm3
	movq	144(%rsp), %r9
	vpinsrd	$1, (%rsi,%r9,4), %xmm3, %xmm3
	movq	2080(%rsp), %rbx
	vpinsrd	$2, (%rsi,%rbx,4), %xmm3, %xmm3
	movq	%rdi, 1952(%rsp)
	movq	%rax, %rdi
	movq	320(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm3, %xmm3
.Ltmp7655:
	vandps	%ymm0, %ymm13, %ymm4
	vcmplt_oqps	%ymm15, %ymm4, %ymm4
	vandnps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vaddps	%ymm1, %ymm1, %ymm0
	vaddps	%ymm0, %ymm6, %ymm0
	vandps	%ymm0, %ymm13, %ymm1
	vcmplt_oqps	%ymm15, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 1824(%rsp)
.Ltmp7656:
	movq	2608(%r14), %rsi
	vmovd	(%rsi,%r10,4), %xmm0
	movq	%rdx, %r10
	vpinsrd	$1, (%rsi,%r12,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%r15,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r11,4), %xmm0, %xmm0
.Ltmp7657:
	vinserti128	$1, %xmm2, %ymm3, %ymm1
.Ltmp7658:
	vmovd	(%rsi,%rcx,4), %xmm2
	vpinsrd	$1, (%rsi,%r9,4), %xmm2, %xmm2
	vpinsrd	$2, (%rsi,%rbx,4), %xmm2, %xmm2
	vpinsrd	$3, (%rsi,%rax,4), %xmm2, %xmm2
.Ltmp7659:
	movq	5216(%r14), %rcx
	movq	104(%rsp), %r11
	vmovd	(%rcx,%r11,4), %xmm3
	vpinsrd	$1, (%rcx,%r13,4), %xmm3, %xmm3
	movq	112(%rsp), %r9
	vpinsrd	$2, (%rcx,%r9,4), %xmm3, %xmm3
.Ltmp7660:
	vinserti128	$1, %xmm0, %ymm2, %ymm0
.Ltmp7661:
	vpinsrd	$3, (%rcx,%rdi,4), %xmm3, %xmm2
	vmovd	(%rcx,%r8,4), %xmm3
	movq	24(%rsp), %rax
	vpinsrd	$1, (%rcx,%rax,4), %xmm3, %xmm3
	movq	16(%rsp), %rdx
	vpinsrd	$2, (%rcx,%rdx,4), %xmm3, %xmm3
	movq	120(%rsp), %rsi
	vpinsrd	$3, (%rcx,%rsi,4), %xmm3, %xmm3
.Ltmp7662:
	movq	5232(%r14), %rcx
	vmovd	(%rcx,%r11,4), %xmm4
	vpinsrd	$1, (%rcx,%r13,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%r9,4), %xmm4, %xmm4
.Ltmp7663:
	vinserti128	$1, %xmm2, %ymm3, %ymm2
.Ltmp7664:
	vpinsrd	$3, (%rcx,%rdi,4), %xmm4, %xmm3
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rax,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rsi,4), %xmm4, %xmm4
	vinserti128	$1, %xmm3, %ymm4, %ymm3
.Ltmp7665:
	vpand	%ymm1, %ymm13, %ymm1
	vpand	%ymm2, %ymm13, %ymm2
	vmaxps	%ymm2, %ymm1, %ymm1
.Ltmp7666:
	vpand	%ymm0, %ymm13, %ymm0
	vpand	%ymm3, %ymm13, %ymm2
	vmaxps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_4(%rip), %ymm8
.Ltmp7667:
	vmaxps	%ymm8, %ymm1, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm3
	vbroadcastss	.LCPI40_5(%rip), %ymm9
	vmaxps	%ymm9, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm1
	vbroadcastsd	.LCPI40_7(%rip), %ymm4
	vorps	%ymm4, %ymm1, %ymm1
	vbroadcastss	.LCPI40_8(%rip), %ymm10
	vaddps	%ymm1, %ymm10, %ymm1
	vbroadcastss	.LCPI40_9(%rip), %ymm11
	vmulps	%ymm1, %ymm11, %ymm5
	vbroadcastss	.LCPI40_10(%rip), %ymm12
	vaddps	%ymm5, %ymm12, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_11(%rip), %ymm14
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_12(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_13(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_14(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm1, %ymm1
	vpsrld	$23, %ymm0, %ymm0
	vpbroadcastd	.LCPI40_15(%rip), %ymm5
	vpor	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm9
	vaddps	%ymm0, %ymm9, %ymm0
	vaddps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm8
	vmulps	%ymm0, %ymm8, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm12
	vmaxps	%ymm12, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm14
	vminps	%ymm14, %ymm0, %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vsubps	352(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_20(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm6
	vmulps	%ymm6, %ymm6, %ymm6
	vbroadcastss	.LCPI40_22(%rip), %ymm11
	vmulps	%ymm6, %ymm11, %ymm6
	vcmpgt_oqps	%ymm1, %ymm0, %ymm7
	vblendvps	%ymm7, %ymm0, %ymm6, %ymm6
	vbroadcastss	.LCPI40_21(%rip), %ymm10
	vcmple_oqps	%ymm10, %ymm0, %ymm0
	vmulps	2240(%rsp), %ymm6, %ymm6
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm0, %ymm1, %ymm0
	vpandn	%ymm6, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm6
	vmaxps	%ymm6, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	1984(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm6
	vmovaps	2560(%rsp), %ymm7
	vblendvps	%ymm6, 2592(%rsp), %ymm7, %ymm6
	vsubps	%ymm0, %ymm1, %ymm7
	vmulps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm6
	vcmplt_oqps	%ymm15, %ymm6, %ymm6
	vandnps	%ymm0, %ymm6, %ymm6
.Ltmp7668:
	vbroadcastss	.LCPI40_4(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm0
	vbroadcastss	.LCPI40_5(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm2
	vorps	%ymm4, %ymm2, %ymm2
	vpsrld	$23, %ymm0, %ymm0
	vpor	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_8(%rip), %ymm1
	vaddps	%ymm1, %ymm2, %ymm2
	vbroadcastss	.LCPI40_9(%rip), %ymm1
	vmulps	%ymm1, %ymm2, %ymm3
	vbroadcastss	.LCPI40_10(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_11(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_12(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_13(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_14(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
	vaddps	%ymm0, %ymm9, %ymm0
	vaddps	%ymm2, %ymm0, %ymm0
	vmulps	%ymm0, %ymm8, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm14, %ymm0, %ymm0
	vmovaps	%ymm0, 160(%rsp)
	vsubps	512(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_20(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm2
	vmulps	%ymm2, %ymm2, %ymm2
	vmulps	%ymm2, %ymm11, %ymm2
	vcmpgt_oqps	%ymm1, %ymm0, %ymm4
	vblendvps	%ymm4, %ymm0, %ymm2, %ymm2
	vcmple_oqps	%ymm10, %ymm0, %ymm0
	vmulps	2528(%rsp), %ymm2, %ymm2
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm0, %ymm1, %ymm0
	vpandn	%ymm2, %ymm0, %ymm0
	vmovaps	%ymm6, 1984(%rsp)
.Ltmp7669:
	vaddps	480(%rsp), %ymm6, %ymm2
	vbroadcastss	.LCPI40_24(%rip), %ymm7
	vmulps	%ymm7, %ymm2, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm8
	vmaxps	%ymm8, %ymm2, %ymm2
	vbroadcastss	.LCPI40_26(%rip), %ymm3
	vminps	%ymm3, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm12
	vmulps	%ymm2, %ymm12, %ymm5
	vbroadcastss	.LCPI40_28(%rip), %ymm14
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_29(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_30(%rip), %ymm10
	vaddps	%ymm5, %ymm10, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm9
	vaddps	%ymm5, %ymm9, %ymm5
	vbroadcastss	.LCPI40_23(%rip), %ymm12
.Ltmp7670:
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	1856(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm6
	vmovaps	2464(%rsp), %ymm7
	vblendvps	%ymm6, 2496(%rsp), %ymm7, %ymm6
.Ltmp7671:
	vmulps	%ymm5, %ymm2, %ymm2
.Ltmp7672:
	vsubps	%ymm0, %ymm1, %ymm5
	vmulps	%ymm6, %ymm5, %ymm5
	vaddps	%ymm5, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm11
	vcmplt_oqps	%ymm11, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm1
	vbroadcastss	.LCPI40_32(%rip), %ymm6
.Ltmp7673:
	vaddps	%ymm6, %ymm2, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm7
	vaddps	%ymm7, %ymm4, %ymm2
	vpslld	$23, %ymm2, %ymm2
	vmovaps	%ymm1, 1856(%rsp)
.Ltmp7674:
	vaddps	640(%rsp), %ymm1, %ymm4
.Ltmp7675:
	vmulps	%ymm2, %ymm0, %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vbroadcastss	.LCPI40_24(%rip), %ymm2
.Ltmp7676:
	vmulps	%ymm2, %ymm4, %ymm0
	vmaxps	%ymm8, %ymm0, %ymm0
	vmovaps	%ymm8, %ymm14
	vminps	%ymm3, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm4
	vsubps	%ymm4, %ymm0, %ymm0
	vbroadcastss	.LCPI40_27(%rip), %ymm3
	vmulps	%ymm3, %ymm0, %ymm5
	vbroadcastss	.LCPI40_28(%rip), %ymm8
	vaddps	%ymm5, %ymm8, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm10, %ymm5
	vmovaps	%ymm10, %ymm15
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm9, %ymm5
	vmulps	%ymm5, %ymm0, %ymm0
	vaddps	%ymm6, %ymm0, %ymm0
	vaddps	%ymm7, %ymm4, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmovaps	320(%rsp), %ymm1
.Ltmp7677:
	vsubps	992(%rsp), %ymm1, %ymm5
.Ltmp7678:
	vmulps	%ymm4, %ymm0, %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm6
.Ltmp7679:
	vaddps	%ymm6, %ymm5, %ymm0
	vmulps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI40_22(%rip), %ymm9
	vmulps	%ymm0, %ymm9, %ymm0
	vcmpgt_oqps	%ymm6, %ymm5, %ymm4
	vblendvps	%ymm4, %ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_21(%rip), %ymm10
	vcmple_oqps	%ymm10, %ymm5, %ymm4
	vmulps	2432(%rsp), %ymm0, %ymm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm4, %ymm1, %ymm4
	vpandn	%ymm0, %ymm4, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vxorps	%xmm7, %xmm7, %xmm7
	vmovaps	1888(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm4
	vmovaps	2368(%rsp), %ymm5
	vblendvps	%ymm4, 2400(%rsp), %ymm5, %ymm4
	vsubps	%ymm0, %ymm1, %ymm5
	vmulps	%ymm4, %ymm5, %ymm4
	vaddps	%ymm4, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm4
	vcmplt_oqps	%ymm11, %ymm4, %ymm4
	vandnps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vaddps	1120(%rsp), %ymm0, %ymm0
	vmulps	%ymm2, %ymm0, %ymm0
	vmaxps	%ymm14, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm4
	vsubps	%ymm4, %ymm0, %ymm0
	vmulps	%ymm3, %ymm0, %ymm5
	vaddps	%ymm5, %ymm8, %ymm5
	vmovaps	%ymm8, %ymm11
	vmulps	%ymm5, %ymm0, %ymm5
	vbroadcastss	.LCPI40_29(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm1
	vaddps	%ymm1, %ymm4, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmulps	%ymm4, %ymm0, %ymm4
	vmovaps	160(%rsp), %ymm0
.Ltmp7680:
	vsubps	1152(%rsp), %ymm0, %ymm0
	vaddps	%ymm6, %ymm0, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm9, %ymm3
	vcmpgt_oqps	%ymm6, %ymm0, %ymm5
	vblendvps	%ymm5, %ymm0, %ymm3, %ymm3
	vcmple_oqps	%ymm10, %ymm0, %ymm0
	vmulps	2336(%rsp), %ymm3, %ymm3
	vpcmpgtd	%ymm0, %ymm7, %ymm0
	vpandn	%ymm3, %ymm0, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm7, %ymm0, %ymm0
	vmovaps	2208(%rsp), %ymm5
	vcmplt_oqps	%ymm5, %ymm0, %ymm3
	vmovaps	2272(%rsp), %ymm1
	vblendvps	%ymm3, 2304(%rsp), %ymm1, %ymm3
	vsubps	%ymm0, %ymm5, %ymm5
	vmulps	%ymm3, %ymm5, %ymm3
	vaddps	%ymm3, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm3
	vbroadcastss	.LCPI40_2(%rip), %ymm1
	vcmplt_oqps	%ymm1, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm8
	vaddps	1280(%rsp), %ymm8, %ymm0
	vmulps	%ymm2, %ymm0, %ymm0
	vmaxps	%ymm14, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm3
	vsubps	%ymm3, %ymm0, %ymm0
	vbroadcastss	.LCPI40_27(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vbroadcastss	.LCPI40_29(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vpslld	$23, %ymm3, %ymm3
	vmulps	%ymm3, %ymm0, %ymm0
.Ltmp7681:
	movq	2592(%r14), %rcx
	movq	1952(%rsp), %rax
	vmovaps	64(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm2
	movq	2608(%r14), %rcx
	vmovaps	320(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm1
	vaddps	%ymm1, %ymm2, %ymm1
.Ltmp7682:
	movq	5216(%r14), %rcx
	vmulps	(%rcx,%rax,4), %ymm4, %ymm2
	.loc	1 1052 27 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp7683:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
.Ltmp7684:
	.loc	14 48 14
	vaddps	%ymm0, %ymm2, %ymm0
	movq	2144(%rsp), %r11
	movq	128(%rsp), %rdi
.Ltmp7685:
	.loc	8 551 14
	vmovups	%ymm1, (%r11,%rdi,4)
	movq	56(%rsp), %r15
.Ltmp7686:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm0, (%r15,%rdi,4)
	movq	136(%rsp), %r12
.Ltmp7687:
	.loc	1 0 0
	incq	%r12
.Ltmp7688:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r12, 312(%rsp)
	movq	2016(%rsp), %rbx
.Ltmp7689:
	.loc	3 900 12
	jne	.LBB40_220
	jmp	.LBB40_152
.Ltmp7690:
.LBB40_271:
	.loc	3 0 12 is_stmt 0
	vmovaps	1760(%rsp), %ymm0
.Ltmp7691:
	.loc	1 1057 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r14)
	vmovaps	1664(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r14)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r14)
	vmovaps	1696(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r14)
	vmovaps	256(%rsp), %ymm0
	.loc	1 1058 5
	vmovaps	%ymm0, 4032(%r14)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r14)
	vmovaps	1920(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r14)
	vmovaps	320(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r14)
	vmovaps	1888(%rsp), %ymm0
	.loc	1 1059 5
	vmovaps	%ymm0, 2304(%r14)
	vmovaps	1856(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r14)
	vmovdqa	1984(%rsp), %ymm0
	.loc	1 1060 5
	vmovdqa	%ymm0, 4928(%r14)
	vmovaps	%ymm3, 4960(%r14)
	.loc	1 1061 5
	movq	%r10, 5272(%r14)
	movq	1656(%rsp), %rsi
.Ltmp7692:
.LBB40_272:
	.loc	1 0 5 is_stmt 0
	movq	2640(%rsp), %r15
	.loc	1 1086 11 is_stmt 1
	cmpq	%rsi, %r15
	jae	.LBB40_511
.LBB40_273:
	.loc	1 1087 42
	subq	%r15, %rsi
	.loc	1 1087 29 is_stmt 0
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E12plan_segmentB5_
	movl	%edx, %r13d
	movq	%rax, %r12
.Ltmp7693:
	.loc	1 1176 33 is_stmt 1
	vmovss	(%r14), %xmm0
.Ltmp7694:
	.loc	1 1089 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp7695:
	.loc	1 1176 33
	vmovss	160(%r14), %xmm0
.Ltmp7696:
	.loc	1 1089 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp7697:
	.loc	1 1176 33
	vmovss	320(%r14), %xmm0
.Ltmp7698:
	.loc	1 1089 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp7699:
	.loc	1 1176 33
	vmovss	480(%r14), %xmm0
.Ltmp7700:
	.loc	1 1089 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp7701:
	.loc	1 1176 33
	vmovss	640(%r14), %xmm0
.Ltmp7702:
	.loc	1 1089 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp7703:
	.loc	1 1176 33
	vmovss	800(%r14), %xmm0
.Ltmp7704:
	.loc	1 1089 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp7705:
	.loc	1 1176 33
	vmovss	960(%r14), %xmm0
.Ltmp7706:
	.loc	1 1089 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp7707:
	.loc	1 1176 33
	vmovss	1120(%r14), %xmm0
.Ltmp7708:
	.loc	1 1089 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp7709:
	.loc	1 1176 33
	vmovss	16(%r14), %xmm0
.Ltmp7710:
	.loc	1 1089 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp7711:
	.loc	1 1176 33
	vmovss	176(%r14), %xmm0
.Ltmp7712:
	.loc	1 1089 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp7713:
	.loc	1 1176 33
	vmovss	336(%r14), %xmm0
.Ltmp7714:
	.loc	1 1089 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp7715:
	.loc	1 1176 33
	vmovss	496(%r14), %xmm0
.Ltmp7716:
	.loc	1 1089 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp7717:
	.loc	1 1176 33
	vmovss	656(%r14), %xmm0
.Ltmp7718:
	.loc	1 1089 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp7719:
	.loc	1 1176 33
	vmovss	816(%r14), %xmm0
.Ltmp7720:
	.loc	1 1089 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp7721:
	.loc	1 1176 33
	vmovss	976(%r14), %xmm0
.Ltmp7722:
	.loc	1 1089 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp7723:
	.loc	1 1176 33
	vmovss	1136(%r14), %xmm0
.Ltmp7724:
	.loc	1 1089 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp7725:
	.loc	1 1176 33
	vmovss	32(%r14), %xmm0
.Ltmp7726:
	.loc	1 1089 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp7727:
	.loc	1 1176 33
	vmovss	192(%r14), %xmm0
.Ltmp7728:
	.loc	1 1089 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp7729:
	.loc	1 1176 33
	vmovss	352(%r14), %xmm0
.Ltmp7730:
	.loc	1 1089 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp7731:
	.loc	1 1176 33
	vmovss	512(%r14), %xmm0
.Ltmp7732:
	.loc	1 1089 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp7733:
	.loc	1 1176 33
	vmovss	672(%r14), %xmm0
.Ltmp7734:
	.loc	1 1089 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp7735:
	.loc	1 1176 33
	vmovss	832(%r14), %xmm0
.Ltmp7736:
	.loc	1 1089 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp7737:
	.loc	1 1176 33
	vmovss	992(%r14), %xmm0
.Ltmp7738:
	.loc	1 1089 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp7739:
	.loc	1 1176 33
	vmovss	1152(%r14), %xmm0
.Ltmp7740:
	.loc	1 1089 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp7741:
	.loc	1 1176 33
	vmovss	48(%r14), %xmm0
.Ltmp7742:
	.loc	1 1089 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp7743:
	.loc	1 1176 33
	vmovss	208(%r14), %xmm0
.Ltmp7744:
	.loc	1 1089 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp7745:
	.loc	1 1176 33
	vmovss	368(%r14), %xmm0
.Ltmp7746:
	.loc	1 1089 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp7747:
	.loc	1 1176 33
	vmovss	528(%r14), %xmm0
.Ltmp7748:
	.loc	1 1089 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp7749:
	.loc	1 1176 33
	vmovss	688(%r14), %xmm0
.Ltmp7750:
	.loc	1 1089 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp7751:
	.loc	1 1176 33
	vmovss	848(%r14), %xmm0
.Ltmp7752:
	.loc	1 1089 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp7753:
	.loc	1 1176 33
	vmovss	1008(%r14), %xmm0
.Ltmp7754:
	.loc	1 1089 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp7755:
	.loc	1 1176 33
	vmovss	1168(%r14), %xmm0
.Ltmp7756:
	.loc	1 1089 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp7757:
	.loc	1 1176 33
	vmovss	64(%r14), %xmm0
.Ltmp7758:
	.loc	1 1089 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp7759:
	.loc	1 1176 33
	vmovss	224(%r14), %xmm0
.Ltmp7760:
	.loc	1 1089 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp7761:
	.loc	1 1176 33
	vmovss	384(%r14), %xmm0
.Ltmp7762:
	.loc	1 1089 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp7763:
	.loc	1 1176 33
	vmovss	544(%r14), %xmm0
.Ltmp7764:
	.loc	1 1089 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp7765:
	.loc	1 1176 33
	vmovss	704(%r14), %xmm0
.Ltmp7766:
	.loc	1 1089 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp7767:
	.loc	1 1176 33
	vmovss	864(%r14), %xmm0
.Ltmp7768:
	.loc	1 1089 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp7769:
	.loc	1 1176 33
	vmovss	1024(%r14), %xmm0
.Ltmp7770:
	.loc	1 1089 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp7771:
	.loc	1 1176 33
	vmovss	1184(%r14), %xmm0
.Ltmp7772:
	.loc	1 1089 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp7773:
	.loc	1 1176 33
	vmovss	80(%r14), %xmm0
.Ltmp7774:
	.loc	1 1089 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp7775:
	.loc	1 1176 33
	vmovss	240(%r14), %xmm0
.Ltmp7776:
	.loc	1 1089 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp7777:
	.loc	1 1176 33
	vmovss	400(%r14), %xmm0
.Ltmp7778:
	.loc	1 1089 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp7779:
	.loc	1 1176 33
	vmovss	560(%r14), %xmm0
.Ltmp7780:
	.loc	1 1089 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp7781:
	.loc	1 1176 33
	vmovss	720(%r14), %xmm0
.Ltmp7782:
	.loc	1 1089 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp7783:
	.loc	1 1176 33
	vmovss	880(%r14), %xmm0
.Ltmp7784:
	.loc	1 1089 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp7785:
	.loc	1 1176 33
	vmovss	1040(%r14), %xmm0
.Ltmp7786:
	.loc	1 1089 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp7787:
	.loc	1 1176 33
	vmovss	1200(%r14), %xmm0
.Ltmp7788:
	.loc	1 1089 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp7789:
	.loc	1 1176 33
	vmovss	96(%r14), %xmm0
.Ltmp7790:
	.loc	1 1089 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp7791:
	.loc	1 1176 33
	vmovss	256(%r14), %xmm0
.Ltmp7792:
	.loc	1 1089 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp7793:
	.loc	1 1176 33
	vmovss	416(%r14), %xmm0
.Ltmp7794:
	.loc	1 1089 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp7795:
	.loc	1 1176 33
	vmovss	576(%r14), %xmm0
.Ltmp7796:
	.loc	1 1089 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp7797:
	.loc	1 1176 33
	vmovss	736(%r14), %xmm0
.Ltmp7798:
	.loc	1 1089 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp7799:
	.loc	1 1176 33
	vmovss	896(%r14), %xmm0
.Ltmp7800:
	.loc	1 1089 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp7801:
	.loc	1 1176 33
	vmovss	1056(%r14), %xmm0
.Ltmp7802:
	.loc	1 1089 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp7803:
	.loc	1 1176 33
	vmovss	1216(%r14), %xmm0
.Ltmp7804:
	.loc	1 1089 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp7805:
	.loc	1 1176 33
	vmovss	112(%r14), %xmm0
.Ltmp7806:
	.loc	1 1089 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp7807:
	.loc	1 1176 33
	vmovss	272(%r14), %xmm0
.Ltmp7808:
	.loc	1 1089 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp7809:
	.loc	1 1176 33
	vmovss	432(%r14), %xmm0
.Ltmp7810:
	.loc	1 1089 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp7811:
	.loc	1 1176 33
	vmovss	592(%r14), %xmm0
.Ltmp7812:
	.loc	1 1089 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp7813:
	.loc	1 1176 33
	vmovss	752(%r14), %xmm0
.Ltmp7814:
	.loc	1 1089 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp7815:
	.loc	1 1176 33
	vmovss	912(%r14), %xmm0
.Ltmp7816:
	.loc	1 1089 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp7817:
	.loc	1 1176 33
	vmovss	1072(%r14), %xmm0
.Ltmp7818:
	.loc	1 1089 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp7819:
	.loc	1 1176 33
	vmovss	1232(%r14), %xmm0
.Ltmp7820:
	.loc	1 1089 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp7821:
	.loc	1 1176 33
	vmovss	128(%r14), %xmm0
.Ltmp7822:
	.loc	1 1089 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp7823:
	.loc	1 1176 33
	vmovss	288(%r14), %xmm0
.Ltmp7824:
	.loc	1 1089 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp7825:
	.loc	1 1176 33
	vmovss	448(%r14), %xmm0
.Ltmp7826:
	.loc	1 1089 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp7827:
	.loc	1 1176 33
	vmovss	608(%r14), %xmm0
.Ltmp7828:
	.loc	1 1089 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp7829:
	.loc	1 1176 33
	vmovss	768(%r14), %xmm0
.Ltmp7830:
	.loc	1 1089 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp7831:
	.loc	1 1176 33
	vmovss	928(%r14), %xmm0
.Ltmp7832:
	.loc	1 1089 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp7833:
	.loc	1 1176 33
	vmovss	1088(%r14), %xmm0
.Ltmp7834:
	.loc	1 1089 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp7835:
	.loc	1 1176 33
	vmovss	1248(%r14), %xmm0
.Ltmp7836:
	.loc	1 1089 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp7837:
	.loc	1 1176 33
	vmovss	144(%r14), %xmm0
.Ltmp7838:
	.loc	1 1089 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp7839:
	.loc	1 1176 33
	vmovss	304(%r14), %xmm0
.Ltmp7840:
	.loc	1 1089 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp7841:
	.loc	1 1176 33
	vmovss	464(%r14), %xmm0
.Ltmp7842:
	.loc	1 1089 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp7843:
	.loc	1 1176 33
	vmovss	624(%r14), %xmm0
.Ltmp7844:
	.loc	1 1089 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp7845:
	.loc	1 1176 33
	vmovss	784(%r14), %xmm0
.Ltmp7846:
	.loc	1 1089 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp7847:
	.loc	1 1176 33
	vmovss	944(%r14), %xmm0
.Ltmp7848:
	.loc	1 1089 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp7849:
	.loc	1 1176 33
	vmovss	1104(%r14), %xmm0
.Ltmp7850:
	.loc	1 1089 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp7851:
	.loc	1 1176 33
	vmovss	1264(%r14), %xmm0
.Ltmp7852:
	.loc	1 1089 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp7853:
	.loc	1 1177 32
	vmovss	8(%r14), %xmm0
.Ltmp7854:
	.loc	1 1089 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp7855:
	.loc	1 1177 32
	vmovss	168(%r14), %xmm0
.Ltmp7856:
	.loc	1 1089 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp7857:
	.loc	1 1177 32
	vmovss	328(%r14), %xmm0
.Ltmp7858:
	.loc	1 1089 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp7859:
	.loc	1 1177 32
	vmovss	488(%r14), %xmm0
.Ltmp7860:
	.loc	1 1089 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp7861:
	.loc	1 1177 32
	vmovss	648(%r14), %xmm0
.Ltmp7862:
	.loc	1 1089 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp7863:
	.loc	1 1177 32
	vmovss	808(%r14), %xmm0
.Ltmp7864:
	.loc	1 1089 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp7865:
	.loc	1 1177 32
	vmovss	968(%r14), %xmm0
.Ltmp7866:
	.loc	1 1089 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp7867:
	.loc	1 1177 32
	vmovss	1128(%r14), %xmm0
.Ltmp7868:
	.loc	1 1089 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp7869:
	.loc	1 1177 32
	vmovss	24(%r14), %xmm0
.Ltmp7870:
	.loc	1 1089 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp7871:
	.loc	1 1177 32
	vmovss	184(%r14), %xmm0
.Ltmp7872:
	.loc	1 1089 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp7873:
	.loc	1 1177 32
	vmovss	344(%r14), %xmm0
.Ltmp7874:
	.loc	1 1089 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp7875:
	.loc	1 1177 32
	vmovss	504(%r14), %xmm0
.Ltmp7876:
	.loc	1 1089 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp7877:
	.loc	1 1177 32
	vmovss	664(%r14), %xmm0
.Ltmp7878:
	.loc	1 1089 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp7879:
	.loc	1 1177 32
	vmovss	824(%r14), %xmm0
.Ltmp7880:
	.loc	1 1089 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp7881:
	.loc	1 1177 32
	vmovss	984(%r14), %xmm0
.Ltmp7882:
	.loc	1 1089 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp7883:
	.loc	1 1177 32
	vmovss	1144(%r14), %xmm0
.Ltmp7884:
	.loc	1 1089 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp7885:
	.loc	1 1177 32
	vmovss	40(%r14), %xmm0
.Ltmp7886:
	.loc	1 1089 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp7887:
	.loc	1 1177 32
	vmovss	200(%r14), %xmm0
.Ltmp7888:
	.loc	1 1089 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp7889:
	.loc	1 1177 32
	vmovss	360(%r14), %xmm0
.Ltmp7890:
	.loc	1 1089 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp7891:
	.loc	1 1177 32
	vmovss	520(%r14), %xmm0
.Ltmp7892:
	.loc	1 1089 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp7893:
	.loc	1 1177 32
	vmovss	680(%r14), %xmm0
.Ltmp7894:
	.loc	1 1089 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp7895:
	.loc	1 1177 32
	vmovss	840(%r14), %xmm0
.Ltmp7896:
	.loc	1 1089 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp7897:
	.loc	1 1177 32
	vmovss	1000(%r14), %xmm0
.Ltmp7898:
	.loc	1 1089 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp7899:
	.loc	1 1177 32
	vmovss	1160(%r14), %xmm0
.Ltmp7900:
	.loc	1 1089 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp7901:
	.loc	1 1177 32
	vmovss	56(%r14), %xmm0
.Ltmp7902:
	.loc	1 1089 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp7903:
	.loc	1 1177 32
	vmovss	216(%r14), %xmm0
.Ltmp7904:
	.loc	1 1089 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp7905:
	.loc	1 1177 32
	vmovss	376(%r14), %xmm0
.Ltmp7906:
	.loc	1 1089 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp7907:
	.loc	1 1177 32
	vmovss	536(%r14), %xmm0
.Ltmp7908:
	.loc	1 1089 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp7909:
	.loc	1 1177 32
	vmovss	696(%r14), %xmm0
.Ltmp7910:
	.loc	1 1089 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp7911:
	.loc	1 1177 32
	vmovss	856(%r14), %xmm0
.Ltmp7912:
	.loc	1 1089 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp7913:
	.loc	1 1177 32
	vmovss	1016(%r14), %xmm0
.Ltmp7914:
	.loc	1 1089 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp7915:
	.loc	1 1177 32
	vmovss	1176(%r14), %xmm0
.Ltmp7916:
	.loc	1 1089 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp7917:
	.loc	1 1177 32
	vmovss	72(%r14), %xmm0
.Ltmp7918:
	.loc	1 1089 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp7919:
	.loc	1 1177 32
	vmovss	232(%r14), %xmm0
.Ltmp7920:
	.loc	1 1089 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp7921:
	.loc	1 1177 32
	vmovss	392(%r14), %xmm0
.Ltmp7922:
	.loc	1 1089 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp7923:
	.loc	1 1177 32
	vmovss	552(%r14), %xmm0
.Ltmp7924:
	.loc	1 1089 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp7925:
	.loc	1 1177 32
	vmovss	712(%r14), %xmm0
.Ltmp7926:
	.loc	1 1089 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp7927:
	.loc	1 1177 32
	vmovss	872(%r14), %xmm0
.Ltmp7928:
	.loc	1 1089 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp7929:
	.loc	1 1177 32
	vmovss	1032(%r14), %xmm0
.Ltmp7930:
	.loc	1 1089 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp7931:
	.loc	1 1177 32
	vmovss	1192(%r14), %xmm0
.Ltmp7932:
	.loc	1 1089 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp7933:
	.loc	1 1177 32
	vmovss	88(%r14), %xmm0
.Ltmp7934:
	.loc	1 1089 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp7935:
	.loc	1 1177 32
	vmovss	248(%r14), %xmm0
.Ltmp7936:
	.loc	1 1089 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp7937:
	.loc	1 1177 32
	vmovss	408(%r14), %xmm0
.Ltmp7938:
	.loc	1 1089 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp7939:
	.loc	1 1177 32
	vmovss	568(%r14), %xmm0
.Ltmp7940:
	.loc	1 1089 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp7941:
	.loc	1 1177 32
	vmovss	728(%r14), %xmm0
.Ltmp7942:
	.loc	1 1089 28
	vmovss	%xmm0, 848(%rsp)
.Ltmp7943:
	.loc	1 1177 32
	vmovss	888(%r14), %xmm0
.Ltmp7944:
	.loc	1 1089 28
	vmovss	%xmm0, 852(%rsp)
.Ltmp7945:
	.loc	1 1177 32
	vmovss	1048(%r14), %xmm0
.Ltmp7946:
	.loc	1 1089 28
	vmovss	%xmm0, 856(%rsp)
.Ltmp7947:
	.loc	1 1177 32
	vmovss	1208(%r14), %xmm0
.Ltmp7948:
	.loc	1 1089 28
	vmovss	%xmm0, 860(%rsp)
.Ltmp7949:
	.loc	1 1177 32
	vmovss	104(%r14), %xmm0
.Ltmp7950:
	.loc	1 1089 28
	vmovss	%xmm0, 864(%rsp)
.Ltmp7951:
	.loc	1 1177 32
	vmovss	264(%r14), %xmm0
.Ltmp7952:
	.loc	1 1089 28
	vmovss	%xmm0, 868(%rsp)
.Ltmp7953:
	.loc	1 1177 32
	vmovss	424(%r14), %xmm0
.Ltmp7954:
	.loc	1 1089 28
	vmovss	%xmm0, 872(%rsp)
.Ltmp7955:
	.loc	1 1177 32
	vmovss	584(%r14), %xmm0
.Ltmp7956:
	.loc	1 1089 28
	vmovss	%xmm0, 876(%rsp)
.Ltmp7957:
	.loc	1 1177 32
	vmovss	744(%r14), %xmm0
.Ltmp7958:
	.loc	1 1089 28
	vmovss	%xmm0, 880(%rsp)
.Ltmp7959:
	.loc	1 1177 32
	vmovss	904(%r14), %xmm0
.Ltmp7960:
	.loc	1 1089 28
	vmovss	%xmm0, 884(%rsp)
.Ltmp7961:
	.loc	1 1177 32
	vmovss	1064(%r14), %xmm0
.Ltmp7962:
	.loc	1 1089 28
	vmovss	%xmm0, 888(%rsp)
.Ltmp7963:
	.loc	1 1177 32
	vmovss	1224(%r14), %xmm0
.Ltmp7964:
	.loc	1 1089 28
	vmovss	%xmm0, 892(%rsp)
.Ltmp7965:
	.loc	1 1177 32
	vmovss	120(%r14), %xmm0
.Ltmp7966:
	.loc	1 1089 28
	vmovss	%xmm0, 896(%rsp)
.Ltmp7967:
	.loc	1 1177 32
	vmovss	280(%r14), %xmm0
.Ltmp7968:
	.loc	1 1089 28
	vmovss	%xmm0, 900(%rsp)
.Ltmp7969:
	.loc	1 1177 32
	vmovss	440(%r14), %xmm0
.Ltmp7970:
	.loc	1 1089 28
	vmovss	%xmm0, 904(%rsp)
.Ltmp7971:
	.loc	1 1177 32
	vmovss	600(%r14), %xmm0
.Ltmp7972:
	.loc	1 1089 28
	vmovss	%xmm0, 908(%rsp)
.Ltmp7973:
	.loc	1 1177 32
	vmovss	760(%r14), %xmm0
.Ltmp7974:
	.loc	1 1089 28
	vmovss	%xmm0, 912(%rsp)
.Ltmp7975:
	.loc	1 1177 32
	vmovss	920(%r14), %xmm0
.Ltmp7976:
	.loc	1 1089 28
	vmovss	%xmm0, 916(%rsp)
.Ltmp7977:
	.loc	1 1177 32
	vmovss	1080(%r14), %xmm0
.Ltmp7978:
	.loc	1 1089 28
	vmovss	%xmm0, 920(%rsp)
.Ltmp7979:
	.loc	1 1177 32
	vmovss	1240(%r14), %xmm0
.Ltmp7980:
	.loc	1 1089 28
	vmovss	%xmm0, 924(%rsp)
.Ltmp7981:
	.loc	1 1177 32
	vmovss	136(%r14), %xmm0
.Ltmp7982:
	.loc	1 1089 28
	vmovss	%xmm0, 928(%rsp)
.Ltmp7983:
	.loc	1 1177 32
	vmovss	296(%r14), %xmm0
.Ltmp7984:
	.loc	1 1089 28
	vmovss	%xmm0, 932(%rsp)
.Ltmp7985:
	.loc	1 1177 32
	vmovss	456(%r14), %xmm0
.Ltmp7986:
	.loc	1 1089 28
	vmovss	%xmm0, 936(%rsp)
.Ltmp7987:
	.loc	1 1177 32
	vmovss	616(%r14), %xmm0
.Ltmp7988:
	.loc	1 1089 28
	vmovss	%xmm0, 940(%rsp)
.Ltmp7989:
	.loc	1 1177 32
	vmovss	776(%r14), %xmm0
.Ltmp7990:
	.loc	1 1089 28
	vmovss	%xmm0, 944(%rsp)
.Ltmp7991:
	.loc	1 1177 32
	vmovss	936(%r14), %xmm0
.Ltmp7992:
	.loc	1 1089 28
	vmovss	%xmm0, 948(%rsp)
.Ltmp7993:
	.loc	1 1177 32
	vmovss	1096(%r14), %xmm0
.Ltmp7994:
	.loc	1 1089 28
	vmovss	%xmm0, 952(%rsp)
.Ltmp7995:
	.loc	1 1177 32
	vmovss	1256(%r14), %xmm0
.Ltmp7996:
	.loc	1 1089 28
	vmovss	%xmm0, 956(%rsp)
.Ltmp7997:
	.loc	1 1177 32
	vmovss	152(%r14), %xmm0
.Ltmp7998:
	.loc	1 1089 28
	vmovss	%xmm0, 960(%rsp)
.Ltmp7999:
	.loc	1 1177 32
	vmovss	312(%r14), %xmm0
.Ltmp8000:
	.loc	1 1089 28
	vmovss	%xmm0, 964(%rsp)
.Ltmp8001:
	.loc	1 1177 32
	vmovss	472(%r14), %xmm0
.Ltmp8002:
	.loc	1 1089 28
	vmovss	%xmm0, 968(%rsp)
.Ltmp8003:
	.loc	1 1177 32
	vmovss	632(%r14), %xmm0
.Ltmp8004:
	.loc	1 1089 28
	vmovss	%xmm0, 972(%rsp)
.Ltmp8005:
	.loc	1 1177 32
	vmovss	792(%r14), %xmm0
.Ltmp8006:
	.loc	1 1089 28
	vmovss	%xmm0, 976(%rsp)
.Ltmp8007:
	.loc	1 1177 32
	vmovss	952(%r14), %xmm0
.Ltmp8008:
	.loc	1 1089 28
	vmovss	%xmm0, 980(%rsp)
.Ltmp8009:
	.loc	1 1177 32
	vmovss	1112(%r14), %xmm0
.Ltmp8010:
	.loc	1 1089 28
	vmovss	%xmm0, 984(%rsp)
.Ltmp8011:
	.loc	1 1177 32
	vmovss	1272(%r14), %xmm0
.Ltmp8012:
	.loc	1 1089 28
	vmovss	%xmm0, 988(%rsp)
.Ltmp8013:
	.loc	1 1176 33
	vmovss	2624(%r14), %xmm0
.Ltmp8014:
	.loc	1 1089 28
	vmovss	%xmm0, 992(%rsp)
.Ltmp8015:
	.loc	1 1176 33
	vmovss	2784(%r14), %xmm0
.Ltmp8016:
	.loc	1 1089 28
	vmovss	%xmm0, 996(%rsp)
.Ltmp8017:
	.loc	1 1176 33
	vmovss	2944(%r14), %xmm0
.Ltmp8018:
	.loc	1 1089 28
	vmovss	%xmm0, 1000(%rsp)
.Ltmp8019:
	.loc	1 1176 33
	vmovss	3104(%r14), %xmm0
.Ltmp8020:
	.loc	1 1089 28
	vmovss	%xmm0, 1004(%rsp)
.Ltmp8021:
	.loc	1 1176 33
	vmovss	3264(%r14), %xmm0
.Ltmp8022:
	.loc	1 1089 28
	vmovss	%xmm0, 1008(%rsp)
.Ltmp8023:
	.loc	1 1176 33
	vmovss	3424(%r14), %xmm0
.Ltmp8024:
	.loc	1 1089 28
	vmovss	%xmm0, 1012(%rsp)
.Ltmp8025:
	.loc	1 1176 33
	vmovss	3584(%r14), %xmm0
.Ltmp8026:
	.loc	1 1089 28
	vmovss	%xmm0, 1016(%rsp)
.Ltmp8027:
	.loc	1 1176 33
	vmovss	3744(%r14), %xmm0
.Ltmp8028:
	.loc	1 1089 28
	vmovss	%xmm0, 1020(%rsp)
.Ltmp8029:
	.loc	1 1176 33
	vmovss	2640(%r14), %xmm0
.Ltmp8030:
	.loc	1 1089 28
	vmovss	%xmm0, 1024(%rsp)
.Ltmp8031:
	.loc	1 1176 33
	vmovss	2800(%r14), %xmm0
.Ltmp8032:
	.loc	1 1089 28
	vmovss	%xmm0, 1028(%rsp)
.Ltmp8033:
	.loc	1 1176 33
	vmovss	2960(%r14), %xmm0
.Ltmp8034:
	.loc	1 1089 28
	vmovss	%xmm0, 1032(%rsp)
.Ltmp8035:
	.loc	1 1176 33
	vmovss	3120(%r14), %xmm0
.Ltmp8036:
	.loc	1 1089 28
	vmovss	%xmm0, 1036(%rsp)
.Ltmp8037:
	.loc	1 1176 33
	vmovss	3280(%r14), %xmm0
.Ltmp8038:
	.loc	1 1089 28
	vmovss	%xmm0, 1040(%rsp)
.Ltmp8039:
	.loc	1 1176 33
	vmovss	3440(%r14), %xmm0
.Ltmp8040:
	.loc	1 1089 28
	vmovss	%xmm0, 1044(%rsp)
.Ltmp8041:
	.loc	1 1176 33
	vmovss	3600(%r14), %xmm0
.Ltmp8042:
	.loc	1 1089 28
	vmovss	%xmm0, 1048(%rsp)
.Ltmp8043:
	.loc	1 1176 33
	vmovss	3760(%r14), %xmm0
.Ltmp8044:
	.loc	1 1089 28
	vmovss	%xmm0, 1052(%rsp)
.Ltmp8045:
	.loc	1 1176 33
	vmovss	2656(%r14), %xmm0
.Ltmp8046:
	.loc	1 1089 28
	vmovss	%xmm0, 1056(%rsp)
.Ltmp8047:
	.loc	1 1176 33
	vmovss	2816(%r14), %xmm0
.Ltmp8048:
	.loc	1 1089 28
	vmovss	%xmm0, 1060(%rsp)
.Ltmp8049:
	.loc	1 1176 33
	vmovss	2976(%r14), %xmm0
.Ltmp8050:
	.loc	1 1089 28
	vmovss	%xmm0, 1064(%rsp)
.Ltmp8051:
	.loc	1 1176 33
	vmovss	3136(%r14), %xmm0
.Ltmp8052:
	.loc	1 1089 28
	vmovss	%xmm0, 1068(%rsp)
.Ltmp8053:
	.loc	1 1176 33
	vmovss	3296(%r14), %xmm0
.Ltmp8054:
	.loc	1 1089 28
	vmovss	%xmm0, 1072(%rsp)
.Ltmp8055:
	.loc	1 1176 33
	vmovss	3456(%r14), %xmm0
.Ltmp8056:
	.loc	1 1089 28
	vmovss	%xmm0, 1076(%rsp)
.Ltmp8057:
	.loc	1 1176 33
	vmovss	3616(%r14), %xmm0
.Ltmp8058:
	.loc	1 1089 28
	vmovss	%xmm0, 1080(%rsp)
.Ltmp8059:
	.loc	1 1176 33
	vmovss	3776(%r14), %xmm0
.Ltmp8060:
	.loc	1 1089 28
	vmovss	%xmm0, 1084(%rsp)
.Ltmp8061:
	.loc	1 1176 33
	vmovss	2672(%r14), %xmm0
.Ltmp8062:
	.loc	1 1089 28
	vmovss	%xmm0, 1088(%rsp)
.Ltmp8063:
	.loc	1 1176 33
	vmovss	2832(%r14), %xmm0
.Ltmp8064:
	.loc	1 1089 28
	vmovss	%xmm0, 1092(%rsp)
.Ltmp8065:
	.loc	1 1176 33
	vmovss	2992(%r14), %xmm0
.Ltmp8066:
	.loc	1 1089 28
	vmovss	%xmm0, 1096(%rsp)
.Ltmp8067:
	.loc	1 1176 33
	vmovss	3152(%r14), %xmm0
.Ltmp8068:
	.loc	1 1089 28
	vmovss	%xmm0, 1100(%rsp)
.Ltmp8069:
	.loc	1 1176 33
	vmovss	3312(%r14), %xmm0
.Ltmp8070:
	.loc	1 1089 28
	vmovss	%xmm0, 1104(%rsp)
.Ltmp8071:
	.loc	1 1176 33
	vmovss	3472(%r14), %xmm0
.Ltmp8072:
	.loc	1 1089 28
	vmovss	%xmm0, 1108(%rsp)
.Ltmp8073:
	.loc	1 1176 33
	vmovss	3632(%r14), %xmm0
.Ltmp8074:
	.loc	1 1089 28
	vmovss	%xmm0, 1112(%rsp)
.Ltmp8075:
	.loc	1 1176 33
	vmovss	3792(%r14), %xmm0
.Ltmp8076:
	.loc	1 1089 28
	vmovss	%xmm0, 1116(%rsp)
.Ltmp8077:
	.loc	1 1176 33
	vmovss	2688(%r14), %xmm0
.Ltmp8078:
	.loc	1 1089 28
	vmovss	%xmm0, 1120(%rsp)
.Ltmp8079:
	.loc	1 1176 33
	vmovss	2848(%r14), %xmm0
.Ltmp8080:
	.loc	1 1089 28
	vmovss	%xmm0, 1124(%rsp)
.Ltmp8081:
	.loc	1 1176 33
	vmovss	3008(%r14), %xmm0
.Ltmp8082:
	.loc	1 1089 28
	vmovss	%xmm0, 1128(%rsp)
.Ltmp8083:
	.loc	1 1176 33
	vmovss	3168(%r14), %xmm0
.Ltmp8084:
	.loc	1 1089 28
	vmovss	%xmm0, 1132(%rsp)
.Ltmp8085:
	.loc	1 1176 33
	vmovss	3328(%r14), %xmm0
.Ltmp8086:
	.loc	1 1089 28
	vmovss	%xmm0, 1136(%rsp)
.Ltmp8087:
	.loc	1 1176 33
	vmovss	3488(%r14), %xmm0
.Ltmp8088:
	.loc	1 1089 28
	vmovss	%xmm0, 1140(%rsp)
.Ltmp8089:
	.loc	1 1176 33
	vmovss	3648(%r14), %xmm0
.Ltmp8090:
	.loc	1 1089 28
	vmovss	%xmm0, 1144(%rsp)
.Ltmp8091:
	.loc	1 1176 33
	vmovss	3808(%r14), %xmm0
.Ltmp8092:
	.loc	1 1089 28
	vmovss	%xmm0, 1148(%rsp)
.Ltmp8093:
	.loc	1 1176 33
	vmovss	2704(%r14), %xmm0
.Ltmp8094:
	.loc	1 1089 28
	vmovss	%xmm0, 1152(%rsp)
.Ltmp8095:
	.loc	1 1176 33
	vmovss	2864(%r14), %xmm0
.Ltmp8096:
	.loc	1 1089 28
	vmovss	%xmm0, 1156(%rsp)
.Ltmp8097:
	.loc	1 1176 33
	vmovss	3024(%r14), %xmm0
.Ltmp8098:
	.loc	1 1089 28
	vmovss	%xmm0, 1160(%rsp)
.Ltmp8099:
	.loc	1 1176 33
	vmovss	3184(%r14), %xmm0
.Ltmp8100:
	.loc	1 1089 28
	vmovss	%xmm0, 1164(%rsp)
.Ltmp8101:
	.loc	1 1176 33
	vmovss	3344(%r14), %xmm0
.Ltmp8102:
	.loc	1 1089 28
	vmovss	%xmm0, 1168(%rsp)
.Ltmp8103:
	.loc	1 1176 33
	vmovss	3504(%r14), %xmm0
.Ltmp8104:
	.loc	1 1089 28
	vmovss	%xmm0, 1172(%rsp)
.Ltmp8105:
	.loc	1 1176 33
	vmovss	3664(%r14), %xmm0
.Ltmp8106:
	.loc	1 1089 28
	vmovss	%xmm0, 1176(%rsp)
.Ltmp8107:
	.loc	1 1176 33
	vmovss	3824(%r14), %xmm0
.Ltmp8108:
	.loc	1 1089 28
	vmovss	%xmm0, 1180(%rsp)
.Ltmp8109:
	.loc	1 1176 33
	vmovss	2720(%r14), %xmm0
.Ltmp8110:
	.loc	1 1089 28
	vmovss	%xmm0, 1184(%rsp)
.Ltmp8111:
	.loc	1 1176 33
	vmovss	2880(%r14), %xmm0
.Ltmp8112:
	.loc	1 1089 28
	vmovss	%xmm0, 1188(%rsp)
.Ltmp8113:
	.loc	1 1176 33
	vmovss	3040(%r14), %xmm0
.Ltmp8114:
	.loc	1 1089 28
	vmovss	%xmm0, 1192(%rsp)
.Ltmp8115:
	.loc	1 1176 33
	vmovss	3200(%r14), %xmm0
.Ltmp8116:
	.loc	1 1089 28
	vmovss	%xmm0, 1196(%rsp)
.Ltmp8117:
	.loc	1 1176 33
	vmovss	3360(%r14), %xmm0
.Ltmp8118:
	.loc	1 1089 28
	vmovss	%xmm0, 1200(%rsp)
.Ltmp8119:
	.loc	1 1176 33
	vmovss	3520(%r14), %xmm0
.Ltmp8120:
	.loc	1 1089 28
	vmovss	%xmm0, 1204(%rsp)
.Ltmp8121:
	.loc	1 1176 33
	vmovss	3680(%r14), %xmm0
.Ltmp8122:
	.loc	1 1089 28
	vmovss	%xmm0, 1208(%rsp)
.Ltmp8123:
	.loc	1 1176 33
	vmovss	3840(%r14), %xmm0
.Ltmp8124:
	.loc	1 1089 28
	vmovss	%xmm0, 1212(%rsp)
.Ltmp8125:
	.loc	1 1176 33
	vmovss	2736(%r14), %xmm0
.Ltmp8126:
	.loc	1 1089 28
	vmovss	%xmm0, 1216(%rsp)
.Ltmp8127:
	.loc	1 1176 33
	vmovss	2896(%r14), %xmm0
.Ltmp8128:
	.loc	1 1089 28
	vmovss	%xmm0, 1220(%rsp)
.Ltmp8129:
	.loc	1 1176 33
	vmovss	3056(%r14), %xmm0
.Ltmp8130:
	.loc	1 1089 28
	vmovss	%xmm0, 1224(%rsp)
.Ltmp8131:
	.loc	1 1176 33
	vmovss	3216(%r14), %xmm0
.Ltmp8132:
	.loc	1 1089 28
	vmovss	%xmm0, 1228(%rsp)
.Ltmp8133:
	.loc	1 1176 33
	vmovss	3376(%r14), %xmm0
.Ltmp8134:
	.loc	1 1089 28
	vmovss	%xmm0, 1232(%rsp)
.Ltmp8135:
	.loc	1 1176 33
	vmovss	3536(%r14), %xmm0
.Ltmp8136:
	.loc	1 1089 28
	vmovss	%xmm0, 1236(%rsp)
.Ltmp8137:
	.loc	1 1176 33
	vmovss	3696(%r14), %xmm0
.Ltmp8138:
	.loc	1 1089 28
	vmovss	%xmm0, 1240(%rsp)
.Ltmp8139:
	.loc	1 1176 33
	vmovss	3856(%r14), %xmm0
.Ltmp8140:
	.loc	1 1089 28
	vmovss	%xmm0, 1244(%rsp)
.Ltmp8141:
	.loc	1 1176 33
	vmovss	2752(%r14), %xmm0
.Ltmp8142:
	.loc	1 1089 28
	vmovss	%xmm0, 1248(%rsp)
.Ltmp8143:
	.loc	1 1176 33
	vmovss	2912(%r14), %xmm0
.Ltmp8144:
	.loc	1 1089 28
	vmovss	%xmm0, 1252(%rsp)
.Ltmp8145:
	.loc	1 1176 33
	vmovss	3072(%r14), %xmm0
.Ltmp8146:
	.loc	1 1089 28
	vmovss	%xmm0, 1256(%rsp)
.Ltmp8147:
	.loc	1 1176 33
	vmovss	3232(%r14), %xmm0
.Ltmp8148:
	.loc	1 1089 28
	vmovss	%xmm0, 1260(%rsp)
.Ltmp8149:
	.loc	1 1176 33
	vmovss	3392(%r14), %xmm0
.Ltmp8150:
	.loc	1 1089 28
	vmovss	%xmm0, 1264(%rsp)
.Ltmp8151:
	.loc	1 1176 33
	vmovss	3552(%r14), %xmm0
.Ltmp8152:
	.loc	1 1089 28
	vmovss	%xmm0, 1268(%rsp)
.Ltmp8153:
	.loc	1 1176 33
	vmovss	3712(%r14), %xmm0
.Ltmp8154:
	.loc	1 1089 28
	vmovss	%xmm0, 1272(%rsp)
.Ltmp8155:
	.loc	1 1176 33
	vmovss	3872(%r14), %xmm0
.Ltmp8156:
	.loc	1 1089 28
	vmovss	%xmm0, 1276(%rsp)
.Ltmp8157:
	.loc	1 1176 33
	vmovss	2768(%r14), %xmm0
.Ltmp8158:
	.loc	1 1089 28
	vmovss	%xmm0, 1280(%rsp)
.Ltmp8159:
	.loc	1 1176 33
	vmovss	2928(%r14), %xmm0
.Ltmp8160:
	.loc	1 1089 28
	vmovss	%xmm0, 1284(%rsp)
.Ltmp8161:
	.loc	1 1176 33
	vmovss	3088(%r14), %xmm0
.Ltmp8162:
	.loc	1 1089 28
	vmovss	%xmm0, 1288(%rsp)
.Ltmp8163:
	.loc	1 1176 33
	vmovss	3248(%r14), %xmm0
.Ltmp8164:
	.loc	1 1089 28
	vmovss	%xmm0, 1292(%rsp)
.Ltmp8165:
	.loc	1 1176 33
	vmovss	3408(%r14), %xmm0
.Ltmp8166:
	.loc	1 1089 28
	vmovss	%xmm0, 1296(%rsp)
.Ltmp8167:
	.loc	1 1176 33
	vmovss	3568(%r14), %xmm0
.Ltmp8168:
	.loc	1 1089 28
	vmovss	%xmm0, 1300(%rsp)
.Ltmp8169:
	.loc	1 1176 33
	vmovss	3728(%r14), %xmm0
.Ltmp8170:
	.loc	1 1089 28
	vmovss	%xmm0, 1304(%rsp)
.Ltmp8171:
	.loc	1 1176 33
	vmovss	3888(%r14), %xmm0
.Ltmp8172:
	.loc	1 1089 28
	vmovss	%xmm0, 1308(%rsp)
.Ltmp8173:
	.loc	1 1177 32
	vmovss	2632(%r14), %xmm0
.Ltmp8174:
	.loc	1 1089 28
	vmovss	%xmm0, 1312(%rsp)
.Ltmp8175:
	.loc	1 1177 32
	vmovss	2792(%r14), %xmm0
.Ltmp8176:
	.loc	1 1089 28
	vmovss	%xmm0, 1316(%rsp)
.Ltmp8177:
	.loc	1 1177 32
	vmovss	2952(%r14), %xmm0
.Ltmp8178:
	.loc	1 1089 28
	vmovss	%xmm0, 1320(%rsp)
.Ltmp8179:
	.loc	1 1177 32
	vmovss	3112(%r14), %xmm0
.Ltmp8180:
	.loc	1 1089 28
	vmovss	%xmm0, 1324(%rsp)
.Ltmp8181:
	.loc	1 1177 32
	vmovss	3272(%r14), %xmm0
.Ltmp8182:
	.loc	1 1089 28
	vmovss	%xmm0, 1328(%rsp)
.Ltmp8183:
	.loc	1 1177 32
	vmovss	3432(%r14), %xmm0
.Ltmp8184:
	.loc	1 1089 28
	vmovss	%xmm0, 1332(%rsp)
.Ltmp8185:
	.loc	1 1177 32
	vmovss	3592(%r14), %xmm0
.Ltmp8186:
	.loc	1 1089 28
	vmovss	%xmm0, 1336(%rsp)
.Ltmp8187:
	.loc	1 1177 32
	vmovss	3752(%r14), %xmm0
.Ltmp8188:
	.loc	1 1089 28
	vmovss	%xmm0, 1340(%rsp)
.Ltmp8189:
	.loc	1 1177 32
	vmovss	2648(%r14), %xmm0
.Ltmp8190:
	.loc	1 1089 28
	vmovss	%xmm0, 1344(%rsp)
.Ltmp8191:
	.loc	1 1177 32
	vmovss	2808(%r14), %xmm0
.Ltmp8192:
	.loc	1 1089 28
	vmovss	%xmm0, 1348(%rsp)
.Ltmp8193:
	.loc	1 1177 32
	vmovss	2968(%r14), %xmm0
.Ltmp8194:
	.loc	1 1089 28
	vmovss	%xmm0, 1352(%rsp)
.Ltmp8195:
	.loc	1 1177 32
	vmovss	3128(%r14), %xmm0
.Ltmp8196:
	.loc	1 1089 28
	vmovss	%xmm0, 1356(%rsp)
.Ltmp8197:
	.loc	1 1177 32
	vmovss	3288(%r14), %xmm0
.Ltmp8198:
	.loc	1 1089 28
	vmovss	%xmm0, 1360(%rsp)
.Ltmp8199:
	.loc	1 1177 32
	vmovss	3448(%r14), %xmm0
.Ltmp8200:
	.loc	1 1089 28
	vmovss	%xmm0, 1364(%rsp)
.Ltmp8201:
	.loc	1 1177 32
	vmovss	3608(%r14), %xmm0
.Ltmp8202:
	.loc	1 1089 28
	vmovss	%xmm0, 1368(%rsp)
.Ltmp8203:
	.loc	1 1177 32
	vmovss	3768(%r14), %xmm0
.Ltmp8204:
	.loc	1 1089 28
	vmovss	%xmm0, 1372(%rsp)
.Ltmp8205:
	.loc	1 1177 32
	vmovss	2664(%r14), %xmm0
.Ltmp8206:
	.loc	1 1089 28
	vmovss	%xmm0, 1376(%rsp)
.Ltmp8207:
	.loc	1 1177 32
	vmovss	2824(%r14), %xmm0
.Ltmp8208:
	.loc	1 1089 28
	vmovss	%xmm0, 1380(%rsp)
.Ltmp8209:
	.loc	1 1177 32
	vmovss	2984(%r14), %xmm0
.Ltmp8210:
	.loc	1 1089 28
	vmovss	%xmm0, 1384(%rsp)
.Ltmp8211:
	.loc	1 1177 32
	vmovss	3144(%r14), %xmm0
.Ltmp8212:
	.loc	1 1089 28
	vmovss	%xmm0, 1388(%rsp)
.Ltmp8213:
	.loc	1 1177 32
	vmovss	3304(%r14), %xmm0
.Ltmp8214:
	.loc	1 1089 28
	vmovss	%xmm0, 1392(%rsp)
.Ltmp8215:
	.loc	1 1177 32
	vmovss	3464(%r14), %xmm0
.Ltmp8216:
	.loc	1 1089 28
	vmovss	%xmm0, 1396(%rsp)
.Ltmp8217:
	.loc	1 1177 32
	vmovss	3624(%r14), %xmm0
.Ltmp8218:
	.loc	1 1089 28
	vmovss	%xmm0, 1400(%rsp)
.Ltmp8219:
	.loc	1 1177 32
	vmovss	3784(%r14), %xmm0
.Ltmp8220:
	.loc	1 1089 28
	vmovss	%xmm0, 1404(%rsp)
.Ltmp8221:
	.loc	1 1177 32
	vmovss	2680(%r14), %xmm0
.Ltmp8222:
	.loc	1 1089 28
	vmovss	%xmm0, 1408(%rsp)
.Ltmp8223:
	.loc	1 1177 32
	vmovss	2840(%r14), %xmm0
.Ltmp8224:
	.loc	1 1089 28
	vmovss	%xmm0, 1412(%rsp)
.Ltmp8225:
	.loc	1 1177 32
	vmovss	3000(%r14), %xmm0
.Ltmp8226:
	.loc	1 1089 28
	vmovss	%xmm0, 1416(%rsp)
.Ltmp8227:
	.loc	1 1177 32
	vmovss	3160(%r14), %xmm0
.Ltmp8228:
	.loc	1 1089 28
	vmovss	%xmm0, 1420(%rsp)
.Ltmp8229:
	.loc	1 1177 32
	vmovss	3320(%r14), %xmm0
.Ltmp8230:
	.loc	1 1089 28
	vmovss	%xmm0, 1424(%rsp)
.Ltmp8231:
	.loc	1 1177 32
	vmovss	3480(%r14), %xmm0
.Ltmp8232:
	.loc	1 1089 28
	vmovss	%xmm0, 1428(%rsp)
.Ltmp8233:
	.loc	1 1177 32
	vmovss	3640(%r14), %xmm0
.Ltmp8234:
	.loc	1 1089 28
	vmovss	%xmm0, 1432(%rsp)
.Ltmp8235:
	.loc	1 1177 32
	vmovss	3800(%r14), %xmm0
.Ltmp8236:
	.loc	1 1089 28
	vmovss	%xmm0, 1436(%rsp)
.Ltmp8237:
	.loc	1 1177 32
	vmovss	2696(%r14), %xmm0
.Ltmp8238:
	.loc	1 1089 28
	vmovss	%xmm0, 1440(%rsp)
.Ltmp8239:
	.loc	1 1177 32
	vmovss	2856(%r14), %xmm0
.Ltmp8240:
	.loc	1 1089 28
	vmovss	%xmm0, 1444(%rsp)
.Ltmp8241:
	.loc	1 1177 32
	vmovss	3016(%r14), %xmm0
.Ltmp8242:
	.loc	1 1089 28
	vmovss	%xmm0, 1448(%rsp)
.Ltmp8243:
	.loc	1 1177 32
	vmovss	3176(%r14), %xmm0
.Ltmp8244:
	.loc	1 1089 28
	vmovss	%xmm0, 1452(%rsp)
.Ltmp8245:
	.loc	1 1177 32
	vmovss	3336(%r14), %xmm0
.Ltmp8246:
	.loc	1 1089 28
	vmovss	%xmm0, 1456(%rsp)
.Ltmp8247:
	.loc	1 1177 32
	vmovss	3496(%r14), %xmm0
.Ltmp8248:
	.loc	1 1089 28
	vmovss	%xmm0, 1460(%rsp)
.Ltmp8249:
	.loc	1 1177 32
	vmovss	3656(%r14), %xmm0
.Ltmp8250:
	.loc	1 1089 28
	vmovss	%xmm0, 1464(%rsp)
.Ltmp8251:
	.loc	1 1177 32
	vmovss	3816(%r14), %xmm0
.Ltmp8252:
	.loc	1 1089 28
	vmovss	%xmm0, 1468(%rsp)
.Ltmp8253:
	.loc	1 1177 32
	vmovss	2712(%r14), %xmm0
.Ltmp8254:
	.loc	1 1089 28
	vmovss	%xmm0, 1472(%rsp)
.Ltmp8255:
	.loc	1 1177 32
	vmovss	2872(%r14), %xmm0
.Ltmp8256:
	.loc	1 1089 28
	vmovss	%xmm0, 1476(%rsp)
.Ltmp8257:
	.loc	1 1177 32
	vmovss	3032(%r14), %xmm0
.Ltmp8258:
	.loc	1 1089 28
	vmovss	%xmm0, 1480(%rsp)
.Ltmp8259:
	.loc	1 1177 32
	vmovss	3192(%r14), %xmm0
.Ltmp8260:
	.loc	1 1089 28
	vmovss	%xmm0, 1484(%rsp)
.Ltmp8261:
	.loc	1 1177 32
	vmovss	3352(%r14), %xmm0
.Ltmp8262:
	.loc	1 1089 28
	vmovss	%xmm0, 1488(%rsp)
.Ltmp8263:
	.loc	1 1177 32
	vmovss	3512(%r14), %xmm0
.Ltmp8264:
	.loc	1 1089 28
	vmovss	%xmm0, 1492(%rsp)
.Ltmp8265:
	.loc	1 1177 32
	vmovss	3672(%r14), %xmm0
.Ltmp8266:
	.loc	1 1089 28
	vmovss	%xmm0, 1496(%rsp)
.Ltmp8267:
	.loc	1 1177 32
	vmovss	3832(%r14), %xmm0
.Ltmp8268:
	.loc	1 1089 28
	vmovss	%xmm0, 1500(%rsp)
.Ltmp8269:
	.loc	1 1177 32
	vmovss	2728(%r14), %xmm0
.Ltmp8270:
	.loc	1 1089 28
	vmovss	%xmm0, 1504(%rsp)
.Ltmp8271:
	.loc	1 1177 32
	vmovss	2888(%r14), %xmm0
.Ltmp8272:
	.loc	1 1089 28
	vmovss	%xmm0, 1508(%rsp)
.Ltmp8273:
	.loc	1 1177 32
	vmovss	3048(%r14), %xmm0
.Ltmp8274:
	.loc	1 1089 28
	vmovss	%xmm0, 1512(%rsp)
.Ltmp8275:
	.loc	1 1177 32
	vmovss	3208(%r14), %xmm0
.Ltmp8276:
	.loc	1 1089 28
	vmovss	%xmm0, 1516(%rsp)
.Ltmp8277:
	.loc	1 1177 32
	vmovss	3368(%r14), %xmm0
.Ltmp8278:
	.loc	1 1089 28
	vmovss	%xmm0, 1520(%rsp)
.Ltmp8279:
	.loc	1 1177 32
	vmovss	3528(%r14), %xmm0
.Ltmp8280:
	.loc	1 1089 28
	vmovss	%xmm0, 1524(%rsp)
.Ltmp8281:
	.loc	1 1177 32
	vmovss	3688(%r14), %xmm0
.Ltmp8282:
	.loc	1 1089 28
	vmovss	%xmm0, 1528(%rsp)
.Ltmp8283:
	.loc	1 1177 32
	vmovss	3848(%r14), %xmm0
.Ltmp8284:
	.loc	1 1089 28
	vmovss	%xmm0, 1532(%rsp)
.Ltmp8285:
	.loc	1 1177 32
	vmovss	2744(%r14), %xmm0
.Ltmp8286:
	.loc	1 1089 28
	vmovss	%xmm0, 1536(%rsp)
.Ltmp8287:
	.loc	1 1177 32
	vmovss	2904(%r14), %xmm0
.Ltmp8288:
	.loc	1 1089 28
	vmovss	%xmm0, 1540(%rsp)
.Ltmp8289:
	.loc	1 1177 32
	vmovss	3064(%r14), %xmm0
.Ltmp8290:
	.loc	1 1089 28
	vmovss	%xmm0, 1544(%rsp)
.Ltmp8291:
	.loc	1 1177 32
	vmovss	3224(%r14), %xmm0
.Ltmp8292:
	.loc	1 1089 28
	vmovss	%xmm0, 1548(%rsp)
.Ltmp8293:
	.loc	1 1177 32
	vmovss	3384(%r14), %xmm0
.Ltmp8294:
	.loc	1 1089 28
	vmovss	%xmm0, 1552(%rsp)
.Ltmp8295:
	.loc	1 1177 32
	vmovss	3544(%r14), %xmm0
.Ltmp8296:
	.loc	1 1089 28
	vmovss	%xmm0, 1556(%rsp)
.Ltmp8297:
	.loc	1 1177 32
	vmovss	3704(%r14), %xmm0
.Ltmp8298:
	.loc	1 1089 28
	vmovss	%xmm0, 1560(%rsp)
.Ltmp8299:
	.loc	1 1177 32
	vmovss	3864(%r14), %xmm0
.Ltmp8300:
	.loc	1 1089 28
	vmovss	%xmm0, 1564(%rsp)
.Ltmp8301:
	.loc	1 1177 32
	vmovss	2760(%r14), %xmm0
.Ltmp8302:
	.loc	1 1089 28
	vmovss	%xmm0, 1568(%rsp)
.Ltmp8303:
	.loc	1 1177 32
	vmovss	2920(%r14), %xmm0
.Ltmp8304:
	.loc	1 1089 28
	vmovss	%xmm0, 1572(%rsp)
.Ltmp8305:
	.loc	1 1177 32
	vmovss	3080(%r14), %xmm0
.Ltmp8306:
	.loc	1 1089 28
	vmovss	%xmm0, 1576(%rsp)
.Ltmp8307:
	.loc	1 1177 32
	vmovss	3240(%r14), %xmm0
.Ltmp8308:
	.loc	1 1089 28
	vmovss	%xmm0, 1580(%rsp)
.Ltmp8309:
	.loc	1 1177 32
	vmovss	3400(%r14), %xmm0
.Ltmp8310:
	.loc	1 1089 28
	vmovss	%xmm0, 1584(%rsp)
.Ltmp8311:
	.loc	1 1177 32
	vmovss	3560(%r14), %xmm0
.Ltmp8312:
	.loc	1 1089 28
	vmovss	%xmm0, 1588(%rsp)
.Ltmp8313:
	.loc	1 1177 32
	vmovss	3720(%r14), %xmm0
.Ltmp8314:
	.loc	1 1089 28
	vmovss	%xmm0, 1592(%rsp)
.Ltmp8315:
	.loc	1 1177 32
	vmovss	3880(%r14), %xmm0
.Ltmp8316:
	.loc	1 1089 28
	vmovss	%xmm0, 1596(%rsp)
.Ltmp8317:
	.loc	1 1177 32
	vmovss	2776(%r14), %xmm0
.Ltmp8318:
	.loc	1 1089 28
	vmovss	%xmm0, 1600(%rsp)
.Ltmp8319:
	.loc	1 1177 32
	vmovss	2936(%r14), %xmm0
.Ltmp8320:
	.loc	1 1089 28
	vmovss	%xmm0, 1604(%rsp)
.Ltmp8321:
	.loc	1 1177 32
	vmovss	3096(%r14), %xmm0
.Ltmp8322:
	.loc	1 1089 28
	vmovss	%xmm0, 1608(%rsp)
.Ltmp8323:
	.loc	1 1177 32
	vmovss	3256(%r14), %xmm0
.Ltmp8324:
	.loc	1 1089 28
	vmovss	%xmm0, 1612(%rsp)
.Ltmp8325:
	.loc	1 1177 32
	vmovss	3416(%r14), %xmm0
.Ltmp8326:
	.loc	1 1089 28
	vmovss	%xmm0, 1616(%rsp)
.Ltmp8327:
	.loc	1 1177 32
	vmovss	3576(%r14), %xmm0
.Ltmp8328:
	.loc	1 1089 28
	vmovss	%xmm0, 1620(%rsp)
.Ltmp8329:
	.loc	1 1177 32
	vmovss	3736(%r14), %xmm0
.Ltmp8330:
	.loc	1 1089 28
	vmovss	%xmm0, 1624(%rsp)
.Ltmp8331:
	.loc	1 1177 32
	vmovss	3896(%r14), %xmm0
.Ltmp8332:
	.loc	1 1089 28
	vmovss	%xmm0, 1628(%rsp)
.Ltmp8333:
	.loc	1 1091 31
	leaq	3008(%rsp), %rdi
	movq	%r14, %rsi
	movl	2140(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	3200(%rsp), %rdi
	movq	2632(%rsp), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovaps	3008(%rsp), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	3040(%rsp), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	3072(%rsp), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	3104(%rsp), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	3136(%rsp), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	3168(%rsp), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	3200(%rsp), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	3232(%rsp), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	3328(%rsp), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovdqa	3360(%rsp), %ymm0
	vmovdqa	%ymm0, 2272(%rsp)
.Ltmp8334:
	.loc	1 0 0 is_stmt 0
	leaq	(%r12,%r15), %rax
	shlq	$3, %r15
	leaq	(,%rax,8), %rsi
	.loc	1 1094 12 is_stmt 1
	testb	$1, %r13b
	movq	%r12, 312(%rsp)
	movq	%rax, 2640(%rsp)
	je	.LBB40_334
.Ltmp8335:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp8336:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_569
.Ltmp8337:
	.loc	48 451 16 is_stmt 1
	cmpq	40(%rsp), %rsi
	ja	.LBB40_569
.Ltmp8338:
	.loc	48 451 16 is_stmt 0
	cmpq	32(%rsp), %rsi
	ja	.LBB40_573
.Ltmp8339:
	.loc	1 972 27 is_stmt 1
	vmovaps	1408(%r14), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1440(%r14), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1472(%r14), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1504(%r14), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp8340:
	.loc	1 973 26
	vmovaps	4032(%r14), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	4064(%r14), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r14), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	4128(%r14), %ymm0
	vmovaps	%ymm0, 320(%rsp)
.Ltmp8341:
	.loc	1 974 25
	vmovaps	2304(%r14), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	2336(%r14), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
.Ltmp8342:
	.loc	1 975 24
	vmovaps	4928(%r14), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	4960(%r14), %ymm3
.Ltmp8343:
	.loc	1 976 24
	movq	5272(%r14), %r11
.Ltmp8344:
	.loc	2 1916 50
	testq	%r12, %r12
	je	.LBB40_330
.Ltmp8345:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r12,8), %rax
	movq	%rax, 56(%rsp)
	movq	296(%rsp), %rax
	leaq	(%rax,%r15,4), %rbx
	movq	304(%rsp), %rax
	leaq	(%rax,%r15,4), %r10
.Ltmp8346:
	.loc	3 900 12 is_stmt 1
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r12
	movq	%r12, 1816(%rsp)
	xorl	%edi, %edi
	xorl	%r13d, %r13d
	movq	%rbx, 1808(%rsp)
	movq	%r10, 2128(%rsp)
.Ltmp8347:
	.loc	3 0 12 is_stmt 0
.Ltmp8348:
	.p2align	4
.LBB40_279:
	.loc	1 981 21 is_stmt 1
	vmovaps	352(%rsp), %ymm0
	vmovaps	384(%rsp), %ymm1
	vmovaps	416(%rsp), %ymm2
.Ltmp8349:
	.loc	14 48 14
	vaddps	672(%rsp), %ymm0, %ymm0
.Ltmp8350:
	.loc	1 980 17
	vmovaps	%ymm0, 352(%rsp)
	.loc	1 983 21
	vmovaps	992(%rsp), %ymm0
.Ltmp8351:
	.loc	14 48 14
	vaddps	1312(%rsp), %ymm0, %ymm0
.Ltmp8352:
	.loc	1 982 17
	vmovaps	%ymm0, 992(%rsp)
.Ltmp8353:
	.loc	14 48 14
	vaddps	704(%rsp), %ymm1, %ymm0
.Ltmp8354:
	.loc	1 980 17
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 983 21
	vmovaps	1024(%rsp), %ymm0
.Ltmp8355:
	.loc	14 48 14
	vaddps	1344(%rsp), %ymm0, %ymm0
.Ltmp8356:
	.loc	1 982 17
	vmovaps	%ymm0, 1024(%rsp)
.Ltmp8357:
	.loc	14 48 14
	vaddps	736(%rsp), %ymm2, %ymm0
.Ltmp8358:
	.loc	1 980 17
	vmovaps	%ymm0, 416(%rsp)
	.loc	1 983 21
	vmovaps	1056(%rsp), %ymm0
.Ltmp8359:
	.loc	14 48 14
	vaddps	1376(%rsp), %ymm0, %ymm0
.Ltmp8360:
	.loc	1 982 17
	vmovaps	%ymm0, 1056(%rsp)
	.loc	1 981 21
	vmovaps	448(%rsp), %ymm0
.Ltmp8361:
	.loc	14 48 14
	vaddps	768(%rsp), %ymm0, %ymm0
.Ltmp8362:
	.loc	1 980 17
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 983 21
	vmovaps	1088(%rsp), %ymm0
.Ltmp8363:
	.loc	14 48 14
	vaddps	1408(%rsp), %ymm0, %ymm0
.Ltmp8364:
	.loc	1 982 17
	vmovaps	%ymm0, 1088(%rsp)
	.loc	1 981 21
	vmovaps	480(%rsp), %ymm0
.Ltmp8365:
	.loc	14 48 14
	vaddps	800(%rsp), %ymm0, %ymm0
.Ltmp8366:
	.loc	1 980 17
	vmovaps	%ymm0, 480(%rsp)
	.loc	1 983 21
	vmovaps	1120(%rsp), %ymm0
.Ltmp8367:
	.loc	14 48 14
	vaddps	1440(%rsp), %ymm0, %ymm0
.Ltmp8368:
	.loc	1 982 17
	vmovaps	%ymm0, 1120(%rsp)
	.loc	1 981 21
	vmovaps	512(%rsp), %ymm0
.Ltmp8369:
	.loc	14 48 14
	vaddps	832(%rsp), %ymm0, %ymm0
.Ltmp8370:
	.loc	1 980 17
	vmovaps	%ymm0, 512(%rsp)
	.loc	1 983 21
	vmovaps	1152(%rsp), %ymm0
.Ltmp8371:
	.loc	14 48 14
	vaddps	1472(%rsp), %ymm0, %ymm0
.Ltmp8372:
	.loc	1 982 17
	vmovaps	%ymm0, 1152(%rsp)
	.loc	1 981 21
	vmovaps	544(%rsp), %ymm0
.Ltmp8373:
	.loc	14 48 14
	vaddps	864(%rsp), %ymm0, %ymm0
.Ltmp8374:
	.loc	1 980 17
	vmovaps	%ymm0, 544(%rsp)
	.loc	1 983 21
	vmovaps	1184(%rsp), %ymm0
.Ltmp8375:
	.loc	14 48 14
	vaddps	1504(%rsp), %ymm0, %ymm0
.Ltmp8376:
	.loc	1 982 17
	vmovaps	%ymm0, 1184(%rsp)
	.loc	1 981 21
	vmovaps	576(%rsp), %ymm0
.Ltmp8377:
	.loc	14 48 14
	vaddps	896(%rsp), %ymm0, %ymm0
.Ltmp8378:
	.loc	1 980 17
	vmovaps	%ymm0, 576(%rsp)
	.loc	1 983 21
	vmovaps	1216(%rsp), %ymm0
.Ltmp8379:
	.loc	14 48 14
	vaddps	1536(%rsp), %ymm0, %ymm0
.Ltmp8380:
	.loc	1 982 17
	vmovaps	%ymm0, 1216(%rsp)
	.loc	1 981 21
	vmovaps	608(%rsp), %ymm0
.Ltmp8381:
	.loc	14 48 14
	vaddps	928(%rsp), %ymm0, %ymm0
.Ltmp8382:
	.loc	1 980 17
	vmovaps	%ymm0, 608(%rsp)
	.loc	1 983 21
	vmovaps	1248(%rsp), %ymm0
.Ltmp8383:
	.loc	14 48 14
	vaddps	1568(%rsp), %ymm0, %ymm0
.Ltmp8384:
	.loc	1 982 17
	vmovaps	%ymm0, 1248(%rsp)
	.loc	1 981 21
	vmovaps	640(%rsp), %ymm0
.Ltmp8385:
	.loc	14 48 14
	vaddps	960(%rsp), %ymm0, %ymm0
.Ltmp8386:
	.loc	1 980 17
	vmovaps	%ymm0, 640(%rsp)
	.loc	1 983 21
	vmovaps	1280(%rsp), %ymm0
.Ltmp8387:
	.loc	14 48 14
	vaddps	1600(%rsp), %ymm0, %ymm0
.Ltmp8388:
	.loc	1 982 17
	vmovaps	%ymm0, 1280(%rsp)
.Ltmp8389:
	.loc	1 987 28
	leaq	1(%r11), %rax
	movq	2208(%rsp), %r15
.Ltmp8390:
	.loc	1 857 8
	cmpq	%r15, %rax
	movl	$0, %r9d
	cmovaeq	%r15, %r9
.Ltmp8391:
	.loc	48 568 12
	cmpq	56(%rsp), %rdi
	ja	.LBB40_557
.Ltmp8392:
	.loc	48 438 16
	cmpq	%r13, 1816(%rsp)
	je	.LBB40_547
.Ltmp8393:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r11,8), %rax
.Ltmp8394:
	.loc	1 1000 29 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp8395:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_554
.Ltmp8396:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp8397:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm3, 1984(%rsp)
	vmovups	(%rbx,%rdi,4), %ymm0
.Ltmp8398:
	vmovups	(%r10,%rdi,4), %ymm6
.Ltmp8399:
	vmovaps	1280(%r14), %ymm5
	vmovaps	1312(%r14), %ymm15
	vmovaps	1344(%r14), %ymm1
	vmovaps	3904(%r14), %ymm11
	vmovaps	3936(%r14), %ymm9
	vmovaps	3968(%r14), %ymm4
	vmovaps	1664(%rsp), %ymm7
	vsubps	%ymm7, %ymm0, %ymm2
	vmulps	%ymm2, %ymm15, %ymm3
	vmovaps	1760(%rsp), %ymm8
	vmovaps	%ymm5, 2080(%rsp)
	vmulps	%ymm5, %ymm8, %ymm5
	vaddps	%ymm3, %ymm5, %ymm10
	vaddps	%ymm10, %ymm8, %ymm3
	vmulps	%ymm15, %ymm8, %ymm5
	vmulps	%ymm1, %ymm2, %ymm2
	vaddps	%ymm2, %ymm5, %ymm13
	vaddps	%ymm7, %ymm13, %ymm2
	vmulps	1376(%r14), %ymm3, %ymm12
	vmovaps	1696(%rsp), %ymm3
	vsubps	%ymm3, %ymm2, %ymm5
	vmulps	1728(%rsp), %ymm15, %ymm2
	vmulps	%ymm5, %ymm1, %ymm1
	vaddps	%ymm1, %ymm2, %ymm2
	vaddps	%ymm2, %ymm3, %ymm1
.Ltmp8400:
	vsubps	192(%rsp), %ymm6, %ymm14
	vmulps	%ymm9, %ymm14, %ymm3
	vmovaps	256(%rsp), %ymm8
	vmovaps	%ymm11, 1952(%rsp)
	vmulps	%ymm11, %ymm8, %ymm7
	vaddps	%ymm3, %ymm7, %ymm3
	vaddps	%ymm3, %ymm8, %ymm7
	vmulps	4000(%r14), %ymm7, %ymm7
.Ltmp8401:
	.loc	1 1000 29 is_stmt 1
	movq	2592(%r14), %rcx
.Ltmp8402:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rax,4)
.Ltmp8403:
	.loc	1 1001 30
	movq	2616(%r14), %rsi
.Ltmp8404:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_559
.Ltmp8405:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp8406:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm0, %ymm12, %ymm0
	vsubps	%ymm1, %ymm0, %ymm0
.Ltmp8407:
	.loc	1 1001 30 is_stmt 1
	movq	2608(%r14), %rcx
.Ltmp8408:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp8409:
	.loc	1 1002 28
	movq	5224(%r14), %rsi
.Ltmp8410:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_560
.Ltmp8411:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp8412:
	.loc	1 0 0 is_stmt 0
	vmulps	256(%rsp), %ymm9, %ymm0
	vmulps	%ymm4, %ymm14, %ymm1
	vaddps	%ymm1, %ymm0, %ymm14
	vaddps	192(%rsp), %ymm14, %ymm0
	vmovaps	320(%rsp), %ymm12
	vsubps	%ymm12, %ymm0, %ymm0
	vmovaps	2016(%rsp), %ymm8
	vmulps	%ymm9, %ymm8, %ymm1
	vmulps	%ymm0, %ymm4, %ymm4
	vaddps	%ymm4, %ymm1, %ymm4
	vaddps	%ymm4, %ymm12, %ymm1
.Ltmp8413:
	.loc	1 1002 28 is_stmt 1
	movq	5216(%r14), %rcx
.Ltmp8414:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rax,4)
.Ltmp8415:
	.loc	1 1003 29
	movq	5240(%r14), %rsi
.Ltmp8416:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_561
.Ltmp8417:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp8418:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm7, %ymm6, %ymm6
	vsubps	%ymm1, %ymm6, %ymm1
.Ltmp8419:
	.loc	1 1003 29 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp8420:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rax,4)
	movq	2368(%r14), %rcx
.Ltmp8421:
	.loc	1 877 35
	addq	%r11, %rcx
.Ltmp8422:
	.loc	1 857 8
	cmpq	%r15, %rcx
	movl	$0, %eax
	cmovaeq	%r15, %rax
.Ltmp8423:
	.loc	1 1006 34
	movq	2600(%r14), %rsi
.Ltmp8424:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp8425:
	.loc	1 877 30
	shlq	$3, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	.loc	1 0 25
	movq	2376(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8426:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8427:
	.loc	1 877 30
	leaq	1(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_591
	.loc	1 0 25
	movq	2384(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8428:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8429:
	.loc	1 877 30
	leaq	2(,%rax,8), %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_588
	.loc	1 0 25
	movq	2392(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8430:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8431:
	.loc	1 877 30
	leaq	3(,%rax,8), %rax
	movq	%rax, 64(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_587
	.loc	1 0 25
	movq	2400(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8432:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8433:
	.loc	1 877 30
	leaq	4(,%rax,8), %rax
	movq	%rax, 160(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_595
	.loc	1 0 25
	movq	2408(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8434:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8435:
	.loc	1 877 30
	leaq	5(,%rax,8), %rax
	movq	%rax, 224(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_597
	.loc	1 0 25
	movq	2416(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8436:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8437:
	.loc	1 877 30
	leaq	6(,%rax,8), %rax
	movq	%rax, 24(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_594
	.loc	1 0 25
	movq	2424(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8438:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8439:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_612
.Ltmp8440:
	.loc	1 1008 34 is_stmt 1
	movq	2616(%r14), %rsi
.Ltmp8441:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	cmpq	%rsi, %r12
	jae	.LBB40_629
	cmpq	%rsi, %r8
	jae	.LBB40_588
	cmpq	%rsi, 64(%rsp)
	jae	.LBB40_587
	cmpq	%rsi, 160(%rsp)
	jae	.LBB40_595
	cmpq	%rsi, 224(%rsp)
	jae	.LBB40_611
	cmpq	%rsi, 24(%rsp)
	jae	.LBB40_602
	.loc	1 0 25 is_stmt 0
	movq	%r8, 152(%rsp)
	.loc	1 877 25
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp8442:
	.loc	1 0 25
	movq	%rax, 144(%rsp)
	movq	4992(%r14), %r8
.Ltmp8443:
	.loc	1 877 35
	addq	%r11, %r8
.Ltmp8444:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %r8
	movl	$0, %eax
	cmovaeq	%r15, %rax
.Ltmp8445:
	.loc	1 1010 34
	movq	5224(%r14), %rsi
.Ltmp8446:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp8447:
	.loc	1 877 30
	shlq	$3, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_575
	.loc	1 0 25
	movq	%r13, 128(%rsp)
	movq	%rdi, 136(%rsp)
	movq	5000(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8448:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8449:
	.loc	1 877 30
	leaq	1(,%rax,8), %rax
	movq	%rax, 16(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_574
	.loc	1 0 25
	movq	5008(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8450:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8451:
	.loc	1 877 30
	leaq	2(,%rax,8), %rax
	movq	%rax, 1824(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_596
	.loc	1 0 25
	movq	%r12, 120(%rsp)
	movq	5016(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8452:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8453:
	.loc	1 877 30
	leaq	3(,%rax,8), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_606
	.loc	1 0 25
	movq	5024(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8454:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edi
	cmovaeq	%r15, %rdi
	subq	%rdi, %rax
.Ltmp8455:
	.loc	1 877 30
	leaq	4(,%rax,8), %r10
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r10
	jae	.LBB40_590
	.loc	1 0 25
	movq	5032(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8456:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edi
	cmovaeq	%r15, %rdi
	subq	%rdi, %rax
.Ltmp8457:
	.loc	1 877 30
	leaq	5(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_604
	.loc	1 0 25
	movq	5040(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8458:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edi
	cmovaeq	%r15, %rdi
	subq	%rdi, %rax
.Ltmp8459:
	.loc	1 877 30
	leaq	6(,%rax,8), %r13
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_624
	.loc	1 0 25
	movq	5048(%r14), %rax
	.loc	1 877 35
	addq	%r11, %rax
.Ltmp8460:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movq	%r14, %rbx
	movl	$0, %r14d
	cmovaeq	%r15, %r14
	subq	%r14, %rax
.Ltmp8461:
	.loc	1 877 30
	leaq	7(,%rax,8), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_598
.Ltmp8462:
	.loc	1 1012 34 is_stmt 1
	movq	5240(%rbx), %rsi
.Ltmp8463:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB40_575
	cmpq	%rsi, 16(%rsp)
	jae	.LBB40_574
	cmpq	%rsi, 1824(%rsp)
	jae	.LBB40_596
	cmpq	%rsi, %rdx
	jae	.LBB40_585
	cmpq	%rsi, %r10
	jae	.LBB40_590
	cmpq	%rsi, %r12
	jae	.LBB40_586
	cmpq	%rsi, %r13
	jae	.LBB40_624
	cmpq	%rsi, %r15
	jae	.LBB40_598
.Ltmp8464:
	.loc	1 0 25 is_stmt 0
	movq	%rbx, %r14
	negq	%r9
	addq	%r9, %r11
	incq	%r11
	leaq	(,%r11,8), %rdi
.Ltmp8465:
	.loc	1 1047 36 is_stmt 1
	movq	2600(%rbx), %rsi
.Ltmp8466:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_562
.Ltmp8467:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp8468:
	.loc	1 1049 27
	movq	2616(%r14), %rsi
.Ltmp8469:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_563
.Ltmp8470:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp8471:
	.loc	1 1050 35
	movq	5224(%r14), %rsi
.Ltmp8472:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_564
.Ltmp8473:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp8474:
	.loc	1 1052 27
	movq	5240(%r14), %rsi
.Ltmp8475:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_565
.Ltmp8476:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp8477:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm10, %ymm10, %ymm1
	vaddps	1760(%rsp), %ymm1, %ymm1
	vbroadcastss	.LCPI40_1(%rip), %ymm10
	vandps	%ymm1, %ymm10, %ymm6
	vbroadcastss	.LCPI40_2(%rip), %ymm12
	vcmplt_oqps	%ymm12, %ymm6, %ymm6
	vandnps	%ymm1, %ymm6, %ymm1
	vmovaps	%ymm1, 1760(%rsp)
	vaddps	%ymm13, %ymm13, %ymm1
	vaddps	1664(%rsp), %ymm1, %ymm1
	vandps	%ymm1, %ymm10, %ymm6
	vcmplt_oqps	%ymm12, %ymm6, %ymm6
	vandnps	%ymm1, %ymm6, %ymm1
	vmovaps	%ymm1, 1664(%rsp)
	vmulps	%ymm5, %ymm15, %ymm1
	vmovaps	1728(%rsp), %ymm6
	vmulps	2080(%rsp), %ymm6, %ymm5
	vaddps	%ymm1, %ymm5, %ymm1
	vaddps	%ymm1, %ymm1, %ymm1
	vaddps	%ymm1, %ymm6, %ymm1
	vandps	%ymm1, %ymm10, %ymm5
	vcmplt_oqps	%ymm12, %ymm5, %ymm5
	vandnps	%ymm1, %ymm5, %ymm1
	vmovaps	%ymm1, 1728(%rsp)
	vaddps	%ymm2, %ymm2, %ymm1
	vaddps	1696(%rsp), %ymm1, %ymm1
	vandps	%ymm1, %ymm10, %ymm2
	vcmplt_oqps	%ymm12, %ymm2, %ymm2
	vandnps	%ymm1, %ymm2, %ymm1
	vmovaps	%ymm1, 1696(%rsp)
.Ltmp8478:
	vaddps	%ymm3, %ymm3, %ymm1
	vaddps	256(%rsp), %ymm1, %ymm1
	vandps	%ymm1, %ymm10, %ymm2
	vcmplt_oqps	%ymm12, %ymm2, %ymm2
	vandnps	%ymm1, %ymm2, %ymm1
	vmovaps	%ymm1, 256(%rsp)
	vaddps	%ymm14, %ymm14, %ymm1
	vaddps	192(%rsp), %ymm1, %ymm1
	vandps	%ymm1, %ymm10, %ymm2
	vcmplt_oqps	%ymm12, %ymm2, %ymm2
	vandnps	%ymm1, %ymm2, %ymm1
	vmovaps	%ymm1, 192(%rsp)
	vmulps	%ymm0, %ymm9, %ymm0
	vmulps	1952(%rsp), %ymm8, %ymm1
	vaddps	%ymm0, %ymm1, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
.Ltmp8479:
	movq	2592(%r14), %rsi
	movq	%rdx, 2080(%rsp)
	movq	160(%rsp), %rdx
	vmovd	(%rsi,%rdx,4), %xmm1
	movq	%r12, 112(%rsp)
	movq	224(%rsp), %r12
	vpinsrd	$1, (%rsi,%r12,4), %xmm1, %xmm1
	movq	%r11, 1952(%rsp)
	movq	24(%rsp), %r11
	vpinsrd	$2, (%rsi,%r11,4), %xmm1, %xmm1
.Ltmp8480:
	vaddps	%ymm0, %ymm8, %ymm0
	movq	%r10, 104(%rsp)
	movq	144(%rsp), %r10
.Ltmp8481:
	vpinsrd	$3, (%rsi,%r10,4), %xmm1, %xmm1
	vmovd	(%rsi,%rcx,4), %xmm2
	movq	120(%rsp), %r9
	vpinsrd	$1, (%rsi,%r9,4), %xmm2, %xmm2
	movq	152(%rsp), %rbx
	vpinsrd	$2, (%rsi,%rbx,4), %xmm2, %xmm2
	movq	64(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm2, %xmm2
.Ltmp8482:
	vandps	%ymm0, %ymm10, %ymm3
	vcmplt_oqps	%ymm12, %ymm3, %ymm3
	vmovaps	%ymm12, %ymm8
	vandnps	%ymm0, %ymm3, %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vaddps	%ymm4, %ymm4, %ymm0
	vaddps	320(%rsp), %ymm0, %ymm4
.Ltmp8483:
	vinserti128	$1, %xmm1, %ymm2, %ymm0
.Ltmp8484:
	vpand	%ymm0, %ymm10, %ymm0
	vbroadcastss	.LCPI40_4(%rip), %ymm11
.Ltmp8485:
	vmaxps	%ymm11, %ymm0, %ymm1
	vbroadcastsd	.LCPI40_6(%rip), %ymm0
	vbroadcastss	.LCPI40_5(%rip), %ymm13
	vmaxps	%ymm13, %ymm1, %ymm1
	vandps	%ymm0, %ymm1, %ymm3
	vmovaps	%ymm0, %ymm13
	vmovaps	%ymm0, 2176(%rsp)
	vbroadcastsd	.LCPI40_7(%rip), %ymm12
	vorps	%ymm3, %ymm12, %ymm3
	vmovaps	%ymm12, 2144(%rsp)
	vbroadcastss	.LCPI40_8(%rip), %ymm14
	vaddps	%ymm3, %ymm14, %ymm3
	vbroadcastss	.LCPI40_9(%rip), %ymm15
	vmulps	%ymm3, %ymm15, %ymm5
	vbroadcastss	.LCPI40_10(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_11(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_12(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_13(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_14(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vpsrld	$23, %ymm1, %ymm1
	vpbroadcastd	.LCPI40_15(%rip), %ymm2
	vpor	%ymm2, %ymm1, %ymm1
	vmovdqa	%ymm2, 2048(%rsp)
	vbroadcastss	.LCPI40_16(%rip), %ymm14
	vaddps	%ymm1, %ymm14, %ymm1
	vaddps	%ymm5, %ymm1, %ymm1
	vbroadcastss	.LCPI40_17(%rip), %ymm15
	vmulps	%ymm1, %ymm15, %ymm1
	vbroadcastss	.LCPI40_18(%rip), %ymm5
	vmaxps	%ymm5, %ymm1, %ymm1
	vbroadcastss	.LCPI40_19(%rip), %ymm15
	vminps	%ymm15, %ymm1, %ymm1
	vsubps	352(%rsp), %ymm1, %ymm5
	vbroadcastss	.LCPI40_20(%rip), %ymm11
	vaddps	%ymm5, %ymm11, %ymm1
	vmulps	%ymm1, %ymm1, %ymm1
	vbroadcastss	.LCPI40_22(%rip), %ymm6
	vmulps	%ymm6, %ymm1, %ymm1
	vcmpgt_oqps	%ymm11, %ymm5, %ymm6
	vblendvps	%ymm6, %ymm5, %ymm1, %ymm6
.Ltmp8486:
	movq	2608(%r14), %rsi
	vmovd	(%rsi,%rdx,4), %xmm1
	vpinsrd	$1, (%rsi,%r12,4), %xmm1, %xmm1
	vpinsrd	$2, (%rsi,%r11,4), %xmm1, %xmm1
	movq	1952(%rsp), %r11
	vpinsrd	$3, (%rsi,%r10,4), %xmm1, %xmm1
.Ltmp8487:
	vandps	%ymm4, %ymm10, %ymm7
	vmovaps	%ymm8, %ymm0
	vcmplt_oqps	%ymm8, %ymm7, %ymm7
	vandnps	%ymm4, %ymm7, %ymm4
	vmovaps	%ymm4, 320(%rsp)
.Ltmp8488:
	vmovd	(%rsi,%rcx,4), %xmm4
	vpinsrd	$1, (%rsi,%r9,4), %xmm4, %xmm4
	vpinsrd	$2, (%rsi,%rbx,4), %xmm4, %xmm4
	vpinsrd	$3, (%rsi,%rax,4), %xmm4, %xmm4
.Ltmp8489:
	movq	5216(%r14), %rcx
	movq	104(%rsp), %r10
	vmovd	(%rcx,%r10,4), %xmm7
	movq	112(%rsp), %r9
	vpinsrd	$1, (%rcx,%r9,4), %xmm7, %xmm7
	vpinsrd	$2, (%rcx,%r13,4), %xmm7, %xmm7
	vpinsrd	$3, (%rcx,%r15,4), %xmm7, %xmm7
.Ltmp8490:
	vinserti128	$1, %xmm1, %ymm4, %ymm1
.Ltmp8491:
	vmovd	(%rcx,%r8,4), %xmm4
	movq	16(%rsp), %rax
	vpinsrd	$1, (%rcx,%rax,4), %xmm4, %xmm4
	movq	1824(%rsp), %rdx
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
	movq	2080(%rsp), %rsi
	vpinsrd	$3, (%rcx,%rsi,4), %xmm4, %xmm4
.Ltmp8492:
	movq	5232(%r14), %rcx
	vmovd	(%rcx,%r10,4), %xmm8
	vpinsrd	$1, (%rcx,%r9,4), %xmm8, %xmm8
	vpinsrd	$2, (%rcx,%r13,4), %xmm8, %xmm8
	vpinsrd	$3, (%rcx,%r15,4), %xmm8, %xmm9
.Ltmp8493:
	vinserti128	$1, %xmm7, %ymm4, %ymm3
	vmovdqa	%ymm3, 224(%rsp)
.Ltmp8494:
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rax,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rsi,4), %xmm4, %xmm4
	vinserti128	$1, %xmm9, %ymm4, %ymm3
	vmovdqa	%ymm3, 64(%rsp)
	vbroadcastss	.LCPI40_21(%rip), %ymm4
.Ltmp8495:
	vcmple_oqps	%ymm4, %ymm5, %ymm5
	vmulps	2240(%rsp), %ymm6, %ymm6
	vpxor	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%ymm5, %ymm3, %ymm5
	vpandn	%ymm6, %ymm5, %ymm5
	vbroadcastss	.LCPI40_23(%rip), %ymm14
	vmaxps	%ymm14, %ymm5, %ymm5
	vminps	%ymm3, %ymm5, %ymm5
	vmovaps	1920(%rsp), %ymm9
	vcmplt_oqps	%ymm9, %ymm5, %ymm6
	vmovaps	2560(%rsp), %ymm7
	vblendvps	%ymm6, 2592(%rsp), %ymm7, %ymm6
	vsubps	%ymm5, %ymm9, %ymm7
	vmulps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vandps	%ymm5, %ymm10, %ymm6
	vcmplt_oqps	%ymm0, %ymm6, %ymm6
	vandnps	%ymm5, %ymm6, %ymm7
.Ltmp8496:
	vpand	%ymm1, %ymm10, %ymm1
.Ltmp8497:
	vbroadcastss	.LCPI40_4(%rip), %ymm0
	vmaxps	%ymm0, %ymm1, %ymm1
	vbroadcastss	.LCPI40_5(%rip), %ymm0
	vmaxps	%ymm0, %ymm1, %ymm1
	vandps	%ymm1, %ymm13, %ymm5
	vorps	%ymm5, %ymm12, %ymm5
	vbroadcastss	.LCPI40_8(%rip), %ymm0
	vaddps	%ymm0, %ymm5, %ymm5
	vbroadcastss	.LCPI40_9(%rip), %ymm0
	vmulps	%ymm0, %ymm5, %ymm6
	vbroadcastss	.LCPI40_10(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm6
	vbroadcastss	.LCPI40_11(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm6
	vbroadcastss	.LCPI40_12(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm6
	vbroadcastss	.LCPI40_13(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm6
	vbroadcastss	.LCPI40_14(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm5
	vpsrld	$23, %ymm1, %ymm1
	vpor	%ymm2, %ymm1, %ymm1
	vbroadcastss	.LCPI40_16(%rip), %ymm3
	vaddps	%ymm3, %ymm1, %ymm1
	vaddps	%ymm5, %ymm1, %ymm1
	vbroadcastss	.LCPI40_17(%rip), %ymm3
	vmulps	%ymm3, %ymm1, %ymm1
	vbroadcastss	.LCPI40_18(%rip), %ymm3
	vmaxps	%ymm3, %ymm1, %ymm1
	vminps	%ymm15, %ymm1, %ymm1
	vsubps	512(%rsp), %ymm1, %ymm1
	vaddps	%ymm1, %ymm11, %ymm5
	vmulps	%ymm5, %ymm5, %ymm5
	vbroadcastss	.LCPI40_22(%rip), %ymm3
	vmulps	%ymm3, %ymm5, %ymm5
	vcmpgt_oqps	%ymm11, %ymm1, %ymm6
	vblendvps	%ymm6, %ymm1, %ymm5, %ymm5
	vcmple_oqps	%ymm4, %ymm1, %ymm1
	vmulps	2528(%rsp), %ymm5, %ymm5
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm1, %ymm0, %ymm1
	vpandn	%ymm5, %ymm1, %ymm1
	vmovaps	%ymm7, 1920(%rsp)
.Ltmp8498:
	vaddps	480(%rsp), %ymm7, %ymm5
	vbroadcastss	.LCPI40_24(%rip), %ymm13
	vmulps	%ymm5, %ymm13, %ymm5
	vbroadcastss	.LCPI40_25(%rip), %ymm11
	vmaxps	%ymm11, %ymm5, %ymm5
	vbroadcastss	.LCPI40_26(%rip), %ymm15
	vminps	%ymm15, %ymm5, %ymm5
	vroundps	$9, %ymm5, %ymm6
	vsubps	%ymm6, %ymm5, %ymm5
	vbroadcastss	.LCPI40_27(%rip), %ymm12
	vmulps	%ymm5, %ymm12, %ymm7
	vbroadcastss	.LCPI40_28(%rip), %ymm8
	vaddps	%ymm7, %ymm8, %ymm7
	vmulps	%ymm7, %ymm5, %ymm7
	vbroadcastss	.LCPI40_29(%rip), %ymm2
	vaddps	%ymm2, %ymm7, %ymm7
	vmulps	%ymm7, %ymm5, %ymm7
	vbroadcastss	.LCPI40_30(%rip), %ymm3
	vaddps	%ymm3, %ymm7, %ymm7
	vmulps	%ymm7, %ymm5, %ymm7
	vbroadcastss	.LCPI40_31(%rip), %ymm4
	vaddps	%ymm4, %ymm7, %ymm7
	vbroadcastss	.LCPI40_23(%rip), %ymm9
.Ltmp8499:
	vmaxps	%ymm9, %ymm1, %ymm1
	vminps	%ymm0, %ymm1, %ymm1
	vmovaps	1888(%rsp), %ymm14
	vcmplt_oqps	%ymm14, %ymm1, %ymm9
	vmovaps	2464(%rsp), %ymm13
	vblendvps	%ymm9, 2496(%rsp), %ymm13, %ymm9
.Ltmp8500:
	vmulps	%ymm7, %ymm5, %ymm5
.Ltmp8501:
	vsubps	%ymm1, %ymm14, %ymm7
	vmulps	%ymm7, %ymm9, %ymm7
	vaddps	%ymm7, %ymm1, %ymm1
	vandps	%ymm1, %ymm10, %ymm7
	vbroadcastss	.LCPI40_2(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm7, %ymm7
	vandnps	%ymm1, %ymm7, %ymm7
	vbroadcastss	.LCPI40_32(%rip), %ymm13
.Ltmp8502:
	vaddps	%ymm5, %ymm13, %ymm1
	vbroadcastss	.LCPI40_33(%rip), %ymm14
	vaddps	%ymm6, %ymm14, %ymm5
	vpslld	$23, %ymm5, %ymm5
	vmovaps	%ymm7, 1888(%rsp)
.Ltmp8503:
	vaddps	640(%rsp), %ymm7, %ymm6
.Ltmp8504:
	vmulps	%ymm5, %ymm1, %ymm1
	vmovaps	%ymm1, 160(%rsp)
.Ltmp8505:
	vbroadcastss	.LCPI40_24(%rip), %ymm0
	vmulps	%ymm0, %ymm6, %ymm1
	vmaxps	%ymm11, %ymm1, %ymm1
	vminps	%ymm15, %ymm1, %ymm1
	vroundps	$9, %ymm1, %ymm6
	vsubps	%ymm6, %ymm1, %ymm1
	vmulps	%ymm1, %ymm12, %ymm7
	vaddps	%ymm7, %ymm8, %ymm7
	vmulps	%ymm7, %ymm1, %ymm7
	vaddps	%ymm2, %ymm7, %ymm7
	vmulps	%ymm7, %ymm1, %ymm7
	vaddps	%ymm3, %ymm7, %ymm7
	vmulps	%ymm7, %ymm1, %ymm7
	vaddps	%ymm4, %ymm7, %ymm7
	vmulps	%ymm7, %ymm1, %ymm1
.Ltmp8506:
	vandps	224(%rsp), %ymm10, %ymm7
.Ltmp8507:
	vbroadcastss	.LCPI40_4(%rip), %ymm0
	vmaxps	%ymm0, %ymm7, %ymm7
	vbroadcastss	.LCPI40_5(%rip), %ymm0
	vmaxps	%ymm0, %ymm7, %ymm7
	vmovaps	2176(%rsp), %ymm3
	vandps	%ymm3, %ymm7, %ymm8
	vmovaps	2144(%rsp), %ymm11
	vorps	%ymm11, %ymm8, %ymm8
	vbroadcastss	.LCPI40_8(%rip), %ymm0
	vaddps	%ymm0, %ymm8, %ymm8
	vbroadcastss	.LCPI40_9(%rip), %ymm0
	vmulps	%ymm0, %ymm8, %ymm9
	vbroadcastss	.LCPI40_10(%rip), %ymm0
	vaddps	%ymm0, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vbroadcastss	.LCPI40_11(%rip), %ymm0
	vaddps	%ymm0, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vbroadcastss	.LCPI40_12(%rip), %ymm0
	vaddps	%ymm0, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vbroadcastss	.LCPI40_13(%rip), %ymm0
	vaddps	%ymm0, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vbroadcastss	.LCPI40_14(%rip), %ymm0
	vaddps	%ymm0, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm8
	vpsrld	$23, %ymm7, %ymm7
	vmovdqa	2048(%rsp), %ymm2
	vpor	%ymm2, %ymm7, %ymm7
	vbroadcastss	.LCPI40_16(%rip), %ymm15
	vaddps	%ymm7, %ymm15, %ymm7
	vaddps	%ymm7, %ymm8, %ymm7
.Ltmp8508:
	vaddps	%ymm1, %ymm13, %ymm1
	vmovaps	%ymm14, %ymm8
	vaddps	%ymm6, %ymm14, %ymm6
	vpslld	$23, %ymm6, %ymm6
	vbroadcastss	.LCPI40_17(%rip), %ymm14
.Ltmp8509:
	vmulps	%ymm7, %ymm14, %ymm7
	vbroadcastss	.LCPI40_18(%rip), %ymm12
	vmaxps	%ymm12, %ymm7, %ymm7
	vbroadcastss	.LCPI40_19(%rip), %ymm0
	vminps	%ymm0, %ymm7, %ymm7
	vsubps	992(%rsp), %ymm7, %ymm7
.Ltmp8510:
	vmulps	%ymm6, %ymm1, %ymm0
	vmovaps	%ymm0, 224(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm4
.Ltmp8511:
	vaddps	%ymm4, %ymm7, %ymm1
	vmulps	%ymm1, %ymm1, %ymm1
	vbroadcastss	.LCPI40_22(%rip), %ymm5
	vmulps	%ymm5, %ymm1, %ymm1
	vcmpgt_oqps	%ymm4, %ymm7, %ymm6
	vblendvps	%ymm6, %ymm7, %ymm1, %ymm1
	vbroadcastss	.LCPI40_21(%rip), %ymm12
	vcmple_oqps	%ymm12, %ymm7, %ymm6
	vmulps	2432(%rsp), %ymm1, %ymm1
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm6, %ymm0, %ymm6
	vpandn	%ymm1, %ymm6, %ymm1
	vbroadcastss	.LCPI40_23(%rip), %ymm6
	vmaxps	%ymm6, %ymm1, %ymm1
	vminps	%ymm0, %ymm1, %ymm1
	vmovaps	1856(%rsp), %ymm9
	vcmplt_oqps	%ymm9, %ymm1, %ymm6
	vmovaps	2368(%rsp), %ymm7
	vblendvps	%ymm6, 2400(%rsp), %ymm7, %ymm6
	vsubps	%ymm1, %ymm9, %ymm7
	vmulps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm1, %ymm1
	vandps	%ymm1, %ymm10, %ymm6
	vbroadcastss	.LCPI40_2(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm6, %ymm6
	vandnps	%ymm1, %ymm6, %ymm1
	vmovaps	%ymm1, 1856(%rsp)
	vaddps	1120(%rsp), %ymm1, %ymm1
	vbroadcastss	.LCPI40_24(%rip), %ymm14
	vmulps	%ymm1, %ymm14, %ymm1
	vbroadcastss	.LCPI40_25(%rip), %ymm15
	vmaxps	%ymm15, %ymm1, %ymm1
	vbroadcastss	.LCPI40_26(%rip), %ymm0
	vminps	%ymm0, %ymm1, %ymm1
	vroundps	$9, %ymm1, %ymm6
	vsubps	%ymm6, %ymm1, %ymm1
	vbroadcastss	.LCPI40_27(%rip), %ymm0
	vmulps	%ymm0, %ymm1, %ymm7
	vbroadcastss	.LCPI40_28(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm1, %ymm7
	vbroadcastss	.LCPI40_29(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm1, %ymm7
	vbroadcastss	.LCPI40_30(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm1, %ymm7
	vbroadcastss	.LCPI40_31(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm1, %ymm1
	vaddps	%ymm1, %ymm13, %ymm1
	vaddps	%ymm6, %ymm8, %ymm6
	vpslld	$23, %ymm6, %ymm6
	vmulps	%ymm6, %ymm1, %ymm9
.Ltmp8512:
	vandps	64(%rsp), %ymm10, %ymm1
.Ltmp8513:
	vbroadcastss	.LCPI40_4(%rip), %ymm0
	vmaxps	%ymm0, %ymm1, %ymm1
	vbroadcastss	.LCPI40_5(%rip), %ymm0
	vmaxps	%ymm0, %ymm1, %ymm1
	vandps	%ymm3, %ymm1, %ymm0
	vorps	%ymm0, %ymm11, %ymm0
	vpsrld	$23, %ymm1, %ymm1
	vpor	%ymm2, %ymm1, %ymm1
	vbroadcastss	.LCPI40_8(%rip), %ymm2
	vaddps	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_9(%rip), %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_10(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_11(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_12(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_13(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_14(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
	vaddps	%ymm0, %ymm1, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vsubps	1152(%rsp), %ymm0, %ymm0
	vaddps	%ymm4, %ymm0, %ymm1
	vmulps	%ymm1, %ymm1, %ymm1
	vmulps	%ymm5, %ymm1, %ymm1
	vcmpgt_oqps	%ymm4, %ymm0, %ymm2
	vblendvps	%ymm2, %ymm0, %ymm1, %ymm1
	vcmple_oqps	%ymm12, %ymm0, %ymm0
	vmulps	2336(%rsp), %ymm1, %ymm1
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%ymm0, %ymm2, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vminps	%ymm2, %ymm0, %ymm0
	vmovaps	1984(%rsp), %ymm3
	vcmplt_oqps	%ymm3, %ymm0, %ymm1
	vmovaps	2272(%rsp), %ymm2
	vblendvps	%ymm1, 2304(%rsp), %ymm2, %ymm1
	vsubps	%ymm0, %ymm3, %ymm2
	vmulps	%ymm1, %ymm2, %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vandps	%ymm0, %ymm10, %ymm1
	vbroadcastss	.LCPI40_2(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm3
	vaddps	1280(%rsp), %ymm3, %ymm0
	vmulps	%ymm0, %ymm14, %ymm0
	vmaxps	%ymm15, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm1
	vsubps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_27(%rip), %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_28(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_29(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_30(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_31(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm0
	vaddps	%ymm0, %ymm13, %ymm0
	vaddps	%ymm1, %ymm8, %ymm1
	vpslld	$23, %ymm1, %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp8514:
	movq	2592(%r14), %rcx
	vmovaps	160(%rsp), %ymm1
	vmulps	(%rcx,%rdi,4), %ymm1, %ymm1
	movq	2608(%r14), %rcx
	vmovaps	224(%rsp), %ymm2
	vmulps	(%rcx,%rdi,4), %ymm2, %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp8515:
	movq	5216(%r14), %rcx
	vmulps	(%rcx,%rdi,4), %ymm9, %ymm2
	.loc	1 1052 27 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp8516:
	.loc	14 283 14
	vmulps	(%rcx,%rdi,4), %ymm0, %ymm0
.Ltmp8517:
	.loc	14 48 14
	vaddps	%ymm0, %ymm2, %ymm0
	movq	1808(%rsp), %rbx
	movq	136(%rsp), %rdi
.Ltmp8518:
	.loc	8 551 14
	vmovups	%ymm1, (%rbx,%rdi,4)
	movq	2128(%rsp), %r10
.Ltmp8519:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm0, (%r10,%rdi,4)
	movq	128(%rsp), %r13
.Ltmp8520:
	.loc	1 0 0
	incq	%r13
.Ltmp8521:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	movq	312(%rsp), %r12
	cmpq	%r13, %r12
.Ltmp8522:
	.loc	3 900 12
	jne	.LBB40_279
.Ltmp8523:
.LBB40_330:
	.loc	3 0 12 is_stmt 0
	vmovaps	1760(%rsp), %ymm0
	.loc	1 1057 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r14)
	vmovaps	1664(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r14)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r14)
	vmovaps	1696(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r14)
	vmovaps	256(%rsp), %ymm0
	.loc	1 1058 5
	vmovaps	%ymm0, 4032(%r14)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r14)
	vmovaps	2016(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r14)
	vmovaps	320(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r14)
	vmovaps	1920(%rsp), %ymm0
	.loc	1 1059 5
	vmovaps	%ymm0, 2304(%r14)
	vmovaps	1888(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r14)
	vmovaps	1856(%rsp), %ymm0
	.loc	1 1060 5
	vmovaps	%ymm0, 4928(%r14)
	vmovaps	%ymm3, 4960(%r14)
	.loc	1 1061 5
	movq	%r11, 5272(%r14)
	xorl	%eax, %eax
	xorl	%r8d, %r8d
.Ltmp8524:
	.loc	1 0 5 is_stmt 0
.Ltmp8525:
	.p2align	4
.LBB40_331:
	.loc	1 1194 13 is_stmt 1
	vmovss	352(%rsp,%rax,2), %xmm3
	vmovss	356(%rsp,%rax,2), %xmm4
	vmovss	360(%rsp,%rax,2), %xmm5
	vmovss	364(%rsp,%rax,2), %xmm6
	vmovss	368(%rsp,%rax,2), %xmm7
	vmovss	372(%rsp,%rax,2), %xmm2
	vmovss	376(%rsp,%rax,2), %xmm1
	vmovd	380(%rsp,%rax,2), %xmm0
.Ltmp8526:
	.loc	1 1197 17
	vmovss	%xmm3, (%r14,%rax)
	.loc	1 1198 34
	movl	12(%r14,%rax), %ecx
	movl	172(%r14,%rax), %edx
.Ltmp8527:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8528:
	.loc	1 1198 17
	movl	%ecx, 12(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm4, 160(%r14,%rax)
.Ltmp8529:
	.loc	38 2472 13
	subl	%r12d, %edx
	cmovbl	%r8d, %edx
.Ltmp8530:
	.loc	1 1198 17
	movl	%edx, 172(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm5, 320(%r14,%rax)
	.loc	1 1198 34
	movl	332(%r14,%rax), %ecx
.Ltmp8531:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8532:
	.loc	1 1198 17
	movl	%ecx, 332(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm6, 480(%r14,%rax)
	.loc	1 1198 34
	movl	492(%r14,%rax), %ecx
.Ltmp8533:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8534:
	.loc	1 1198 17
	movl	%ecx, 492(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm7, 640(%r14,%rax)
	.loc	1 1198 34
	movl	652(%r14,%rax), %ecx
.Ltmp8535:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8536:
	.loc	1 1198 17
	movl	%ecx, 652(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 800(%r14,%rax)
	.loc	1 1198 34
	movl	812(%r14,%rax), %ecx
.Ltmp8537:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8538:
	.loc	1 1198 17
	movl	%ecx, 812(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 960(%r14,%rax)
	.loc	1 1198 34
	movl	972(%r14,%rax), %ecx
.Ltmp8539:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8540:
	.loc	1 1198 17
	movl	%ecx, 972(%r14,%rax)
	.loc	1 1197 17
	vmovd	%xmm0, 1120(%r14,%rax)
	.loc	1 1198 34
	movl	1132(%r14,%rax), %ecx
.Ltmp8541:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8542:
	.loc	1 1198 17
	movl	%ecx, 1132(%r14,%rax)
.Ltmp8543:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp8544:
	.loc	3 900 12
	jne	.LBB40_331
.Ltmp8545:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	1656(%rsp), %rsi
	.p2align	4
.LBB40_333:
.Ltmp8546:
	.loc	1 1194 13 is_stmt 1
	vmovss	992(%rsp,%rax,2), %xmm3
	vmovss	996(%rsp,%rax,2), %xmm4
	vmovss	1000(%rsp,%rax,2), %xmm5
	vmovss	1004(%rsp,%rax,2), %xmm6
	vmovss	1008(%rsp,%rax,2), %xmm7
	vmovss	1012(%rsp,%rax,2), %xmm2
	vmovss	1016(%rsp,%rax,2), %xmm1
	vmovd	1020(%rsp,%rax,2), %xmm0
.Ltmp8547:
	.loc	1 1197 17
	vmovss	%xmm3, 2624(%r14,%rax)
	.loc	1 1198 34
	movl	2636(%r14,%rax), %ecx
	movl	2796(%r14,%rax), %edx
.Ltmp8548:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8549:
	.loc	1 1198 17
	movl	%ecx, 2636(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm4, 2784(%r14,%rax)
.Ltmp8550:
	.loc	38 2472 13
	subl	%r12d, %edx
	cmovbl	%r8d, %edx
.Ltmp8551:
	.loc	1 1198 17
	movl	%edx, 2796(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm5, 2944(%r14,%rax)
	.loc	1 1198 34
	movl	2956(%r14,%rax), %ecx
.Ltmp8552:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8553:
	.loc	1 1198 17
	movl	%ecx, 2956(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm6, 3104(%r14,%rax)
	.loc	1 1198 34
	movl	3116(%r14,%rax), %ecx
.Ltmp8554:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8555:
	.loc	1 1198 17
	movl	%ecx, 3116(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm7, 3264(%r14,%rax)
	.loc	1 1198 34
	movl	3276(%r14,%rax), %ecx
.Ltmp8556:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8557:
	.loc	1 1198 17
	movl	%ecx, 3276(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 3424(%r14,%rax)
	.loc	1 1198 34
	movl	3436(%r14,%rax), %ecx
.Ltmp8558:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8559:
	.loc	1 1198 17
	movl	%ecx, 3436(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 3584(%r14,%rax)
	.loc	1 1198 34
	movl	3596(%r14,%rax), %ecx
.Ltmp8560:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8561:
	.loc	1 1198 17
	movl	%ecx, 3596(%r14,%rax)
	.loc	1 1197 17
	vmovd	%xmm0, 3744(%r14,%rax)
	.loc	1 1198 34
	movl	3756(%r14,%rax), %ecx
.Ltmp8562:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%r8d, %ecx
.Ltmp8563:
	.loc	1 1198 17
	movl	%ecx, 3756(%r14,%rax)
.Ltmp8564:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp8565:
	.loc	3 900 12
	jne	.LBB40_333
	jmp	.LBB40_272
.Ltmp8566:
.LBB40_334:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp8567:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_568
.Ltmp8568:
	.loc	48 451 16 is_stmt 1
	cmpq	40(%rsp), %rsi
	ja	.LBB40_568
.Ltmp8569:
	.loc	48 451 16 is_stmt 0
	cmpq	32(%rsp), %rsi
	ja	.LBB40_572
.Ltmp8570:
	.loc	1 972 27 is_stmt 1
	vmovaps	1408(%r14), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1440(%r14), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1472(%r14), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1504(%r14), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp8571:
	.loc	1 973 26
	vmovaps	4032(%r14), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	4064(%r14), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r14), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	4128(%r14), %ymm0
	vmovaps	%ymm0, 320(%rsp)
.Ltmp8572:
	.loc	1 974 25
	vmovaps	2304(%r14), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	2336(%r14), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
.Ltmp8573:
	.loc	1 975 24
	vmovaps	4928(%r14), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	4960(%r14), %ymm3
.Ltmp8574:
	.loc	1 976 24
	movq	5272(%r14), %r10
.Ltmp8575:
	.loc	2 1916 50
	testq	%r12, %r12
	je	.LBB40_271
.Ltmp8576:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r12,8), %rax
	movq	%rax, 56(%rsp)
	movq	296(%rsp), %rax
	leaq	(%rax,%r15,4), %rbx
	movq	304(%rsp), %rax
	leaq	(%rax,%r15,4), %r11
.Ltmp8577:
	.loc	48 568 12 is_stmt 1
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r12
	movq	%r12, 1816(%rsp)
	xorl	%edi, %edi
	xorl	%r12d, %r12d
	movq	%rbx, 1808(%rsp)
	movq	%r11, 2128(%rsp)
.Ltmp8578:
	.loc	48 0 12 is_stmt 0
.Ltmp8579:
	.p2align	4
.LBB40_339:
	.loc	1 987 28 is_stmt 1
	leaq	1(%r10), %rax
	movq	2208(%rsp), %r15
.Ltmp8580:
	.loc	1 857 8
	cmpq	%r15, %rax
	movl	$0, %r9d
	cmovaeq	%r15, %r9
.Ltmp8581:
	.loc	48 568 12
	cmpq	56(%rsp), %rdi
	ja	.LBB40_557
.Ltmp8582:
	.loc	48 438 16
	cmpq	%r12, 1816(%rsp)
	je	.LBB40_547
.Ltmp8583:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,8), %rax
.Ltmp8584:
	.loc	1 1000 29 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp8585:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_554
.Ltmp8586:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp8587:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm3, 1952(%rsp)
	vmovups	(%rbx,%rdi,4), %ymm2
.Ltmp8588:
	vmovups	(%r11,%rdi,4), %ymm15
.Ltmp8589:
	vmovaps	1280(%r14), %ymm5
	vmovaps	1312(%r14), %ymm3
	vmovaps	1344(%r14), %ymm1
	vmovaps	3904(%r14), %ymm11
	vmovaps	3936(%r14), %ymm9
	vmovaps	3968(%r14), %ymm12
	vmovaps	1664(%rsp), %ymm6
	vsubps	%ymm6, %ymm2, %ymm0
	vmulps	%ymm3, %ymm0, %ymm4
	vmovaps	1760(%rsp), %ymm7
	vmovaps	%ymm5, 2080(%rsp)
	vmulps	%ymm5, %ymm7, %ymm5
	vaddps	%ymm4, %ymm5, %ymm10
	vaddps	%ymm7, %ymm10, %ymm4
	vmulps	%ymm3, %ymm7, %ymm5
	vmulps	%ymm1, %ymm0, %ymm0
	vaddps	%ymm0, %ymm5, %ymm14
	vaddps	%ymm6, %ymm14, %ymm5
	vmulps	1376(%r14), %ymm4, %ymm0
	vmovaps	1696(%rsp), %ymm6
	vsubps	%ymm6, %ymm5, %ymm13
	vmulps	1728(%rsp), %ymm3, %ymm4
	vmulps	%ymm1, %ymm13, %ymm1
	vaddps	%ymm1, %ymm4, %ymm1
	vaddps	%ymm1, %ymm6, %ymm5
.Ltmp8590:
	vsubps	192(%rsp), %ymm15, %ymm4
	vmulps	%ymm4, %ymm9, %ymm6
	vmovaps	256(%rsp), %ymm8
	vmovaps	%ymm11, 2016(%rsp)
	vmulps	%ymm11, %ymm8, %ymm7
	vaddps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm8, %ymm7
	vmulps	4000(%r14), %ymm7, %ymm7
.Ltmp8591:
	.loc	1 1000 29 is_stmt 1
	movq	2592(%r14), %rcx
.Ltmp8592:
	.loc	8 551 14
	vmovups	%ymm5, (%rcx,%rax,4)
.Ltmp8593:
	.loc	1 1001 30
	movq	2616(%r14), %rsi
.Ltmp8594:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_559
.Ltmp8595:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp8596:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm0, %ymm2, %ymm0
	vsubps	%ymm5, %ymm0, %ymm0
.Ltmp8597:
	.loc	1 1001 30 is_stmt 1
	movq	2608(%r14), %rcx
.Ltmp8598:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp8599:
	.loc	1 1002 28
	movq	5224(%r14), %rsi
.Ltmp8600:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_560
.Ltmp8601:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp8602:
	.loc	1 0 0 is_stmt 0
	vmulps	256(%rsp), %ymm9, %ymm0
	vmulps	%ymm4, %ymm12, %ymm2
	vaddps	%ymm2, %ymm0, %ymm4
	vaddps	192(%rsp), %ymm4, %ymm0
	vmovaps	320(%rsp), %ymm11
	vsubps	%ymm11, %ymm0, %ymm2
	vmovaps	1920(%rsp), %ymm8
	vmulps	%ymm9, %ymm8, %ymm0
	vmulps	%ymm2, %ymm12, %ymm5
	vaddps	%ymm5, %ymm0, %ymm12
	vaddps	%ymm12, %ymm11, %ymm0
.Ltmp8603:
	.loc	1 1002 28 is_stmt 1
	movq	5216(%r14), %rcx
.Ltmp8604:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp8605:
	.loc	1 1003 29
	movq	5240(%r14), %rsi
.Ltmp8606:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_561
.Ltmp8607:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp8608:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm7, %ymm15, %ymm5
	vsubps	%ymm0, %ymm5, %ymm0
.Ltmp8609:
	.loc	1 1003 29 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp8610:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
	movq	2368(%r14), %rcx
.Ltmp8611:
	.loc	1 877 35
	addq	%r10, %rcx
.Ltmp8612:
	.loc	1 857 8
	cmpq	%r15, %rcx
	movl	$0, %eax
	cmovaeq	%r15, %rax
.Ltmp8613:
	.loc	1 1006 34
	movq	2600(%r14), %rsi
.Ltmp8614:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp8615:
	.loc	1 877 30
	shlq	$3, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	.loc	1 0 25
	movq	2376(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8616:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8617:
	.loc	1 877 30
	leaq	1(,%rax,8), %r13
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_578
	.loc	1 0 25
	movq	2384(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8618:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8619:
	.loc	1 877 30
	leaq	2(,%rax,8), %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_588
	.loc	1 0 25
	movq	2392(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8620:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8621:
	.loc	1 877 30
	leaq	3(,%rax,8), %rax
	movq	%rax, 64(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_587
	.loc	1 0 25
	movq	2400(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8622:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8623:
	.loc	1 877 30
	leaq	4(,%rax,8), %rax
	movq	%rax, 160(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_595
	.loc	1 0 25
	movq	2408(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8624:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8625:
	.loc	1 877 30
	leaq	5(,%rax,8), %rax
	movq	%rax, 224(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_597
	.loc	1 0 25
	movq	2416(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8626:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8627:
	.loc	1 877 30
	leaq	6(,%rax,8), %rax
	movq	%rax, 24(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_594
	.loc	1 0 25
	movq	2424(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8628:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8629:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp8630:
	.loc	1 1008 34 is_stmt 1
	movq	2616(%r14), %rsi
.Ltmp8631:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	cmpq	%rsi, %r13
	jae	.LBB40_578
	cmpq	%rsi, %r8
	jae	.LBB40_588
	cmpq	%rsi, 64(%rsp)
	jae	.LBB40_587
	cmpq	%rsi, 160(%rsp)
	jae	.LBB40_595
	cmpq	%rsi, 224(%rsp)
	jae	.LBB40_597
	cmpq	%rsi, 24(%rsp)
	jae	.LBB40_602
	.loc	1 0 25 is_stmt 0
	movq	%r8, 152(%rsp)
	.loc	1 877 25
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp8632:
	.loc	1 0 25
	movq	%rax, 144(%rsp)
	movq	4992(%r14), %r8
.Ltmp8633:
	.loc	1 877 35
	addq	%r10, %r8
.Ltmp8634:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %r8
	movl	$0, %eax
	cmovaeq	%r15, %rax
.Ltmp8635:
	.loc	1 1010 34
	movq	5224(%r14), %rsi
.Ltmp8636:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp8637:
	.loc	1 877 30
	shlq	$3, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_575
	.loc	1 0 25
	movq	%r13, 136(%rsp)
	movq	5000(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8638:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8639:
	.loc	1 877 30
	leaq	1(,%rax,8), %rax
	movq	%rax, 16(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_574
	.loc	1 0 25
	movq	5008(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8640:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8641:
	.loc	1 877 30
	leaq	2(,%rax,8), %rax
	movq	%rax, 1824(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_600
	.loc	1 0 25
	movq	%r12, 120(%rsp)
	movq	%rdi, 128(%rsp)
	movq	5016(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8642:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edx
	cmovaeq	%r15, %rdx
	subq	%rdx, %rax
.Ltmp8643:
	.loc	1 877 30
	leaq	3(,%rax,8), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_585
	.loc	1 0 25
	movq	5024(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8644:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edi
	cmovaeq	%r15, %rdi
	subq	%rdi, %rax
.Ltmp8645:
	.loc	1 877 30
	leaq	4(,%rax,8), %r11
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_584
	.loc	1 0 25
	movq	5032(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8646:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edi
	cmovaeq	%r15, %rdi
	subq	%rdi, %rax
.Ltmp8647:
	.loc	1 877 30
	leaq	5(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_604
	.loc	1 0 25
	movq	5040(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8648:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movl	$0, %edi
	cmovaeq	%r15, %rdi
	subq	%rdi, %rax
.Ltmp8649:
	.loc	1 877 30
	leaq	6(,%rax,8), %r13
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_592
	.loc	1 0 25
	movq	5048(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp8650:
	.loc	1 857 8 is_stmt 1
	cmpq	%r15, %rax
	movq	%r14, %rbx
	movl	$0, %r14d
	cmovaeq	%r15, %r14
	subq	%r14, %rax
.Ltmp8651:
	.loc	1 877 30
	leaq	7(,%rax,8), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_607
.Ltmp8652:
	.loc	1 1012 34 is_stmt 1
	movq	5240(%rbx), %rsi
.Ltmp8653:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB40_575
	cmpq	%rsi, 16(%rsp)
	jae	.LBB40_574
	cmpq	%rsi, 1824(%rsp)
	jae	.LBB40_600
	cmpq	%rsi, %rdx
	jae	.LBB40_606
	cmpq	%rsi, %r11
	jae	.LBB40_584
	cmpq	%rsi, %r12
	jae	.LBB40_586
	cmpq	%rsi, %r13
	jae	.LBB40_592
	cmpq	%rsi, %r15
	jae	.LBB40_598
.Ltmp8654:
	.loc	1 0 25 is_stmt 0
	movq	%rbx, %r14
	negq	%r9
	addq	%r9, %r10
	incq	%r10
	leaq	(,%r10,8), %rdi
.Ltmp8655:
	.loc	1 1047 36 is_stmt 1
	movq	2600(%rbx), %rsi
.Ltmp8656:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_562
.Ltmp8657:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp8658:
	.loc	1 1049 27
	movq	2616(%r14), %rsi
.Ltmp8659:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_563
.Ltmp8660:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp8661:
	.loc	1 1050 35
	movq	5224(%r14), %rsi
.Ltmp8662:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_564
.Ltmp8663:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp8664:
	.loc	1 1052 27
	movq	5240(%r14), %rsi
.Ltmp8665:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_565
.Ltmp8666:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp8667:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm10, %ymm10, %ymm0
	vaddps	1760(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_1(%rip), %ymm10
	vandps	%ymm0, %ymm10, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm15
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vaddps	%ymm14, %ymm14, %ymm0
	vaddps	1664(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm10, %ymm5
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmulps	%ymm3, %ymm13, %ymm0
	vmovaps	1728(%rsp), %ymm5
	vmulps	2080(%rsp), %ymm5, %ymm3
	vaddps	%ymm0, %ymm3, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	%ymm0, %ymm5, %ymm0
	vandps	%ymm0, %ymm10, %ymm3
	vcmplt_oqps	%ymm15, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vaddps	%ymm1, %ymm1, %ymm0
	vaddps	1696(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm10, %ymm1
	vcmplt_oqps	%ymm15, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp8668:
	vaddps	%ymm6, %ymm6, %ymm0
	vaddps	256(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm10, %ymm1
	vcmplt_oqps	%ymm15, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vaddps	%ymm4, %ymm4, %ymm0
	vaddps	192(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm10, %ymm1
	vcmplt_oqps	%ymm15, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmulps	%ymm2, %ymm9, %ymm0
	vmulps	2016(%rsp), %ymm8, %ymm1
	vaddps	%ymm0, %ymm1, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
.Ltmp8669:
	movq	2592(%r14), %rsi
	movq	%rdx, 112(%rsp)
	movq	160(%rsp), %rdx
	vmovd	(%rsi,%rdx,4), %xmm1
	movq	%r12, 104(%rsp)
	movq	224(%rsp), %r12
	vpinsrd	$1, (%rsi,%r12,4), %xmm1, %xmm1
	movq	%r11, 2048(%rsp)
	movq	24(%rsp), %r11
	vpinsrd	$2, (%rsi,%r11,4), %xmm1, %xmm1
.Ltmp8670:
	vaddps	%ymm0, %ymm8, %ymm0
	movq	%r10, 2080(%rsp)
	movq	144(%rsp), %r10
.Ltmp8671:
	vpinsrd	$3, (%rsi,%r10,4), %xmm1, %xmm1
	vmovd	(%rsi,%rcx,4), %xmm2
	movq	136(%rsp), %r9
	vpinsrd	$1, (%rsi,%r9,4), %xmm2, %xmm2
	movq	152(%rsp), %rbx
	vpinsrd	$2, (%rsi,%rbx,4), %xmm2, %xmm2
	movq	64(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm2, %xmm2
.Ltmp8672:
	vandps	%ymm0, %ymm10, %ymm3
	vcmplt_oqps	%ymm15, %ymm3, %ymm3
	vmovaps	%ymm15, %ymm8
	vandnps	%ymm0, %ymm3, %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vaddps	%ymm12, %ymm12, %ymm0
	vaddps	320(%rsp), %ymm0, %ymm4
.Ltmp8673:
	vinserti128	$1, %xmm1, %ymm2, %ymm1
.Ltmp8674:
	vpand	%ymm1, %ymm10, %ymm0
	vbroadcastss	.LCPI40_4(%rip), %ymm11
.Ltmp8675:
	vmaxps	%ymm11, %ymm0, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm15
	vbroadcastss	.LCPI40_5(%rip), %ymm12
	vmaxps	%ymm12, %ymm0, %ymm0
	vandps	%ymm0, %ymm15, %ymm3
	vmovaps	%ymm15, 2144(%rsp)
	vbroadcastsd	.LCPI40_7(%rip), %ymm1
	vmovaps	%ymm1, 2176(%rsp)
	vorps	%ymm1, %ymm3, %ymm3
	vbroadcastss	.LCPI40_8(%rip), %ymm13
	vaddps	%ymm3, %ymm13, %ymm3
	vbroadcastss	.LCPI40_9(%rip), %ymm14
	vmulps	%ymm3, %ymm14, %ymm5
	vbroadcastss	.LCPI40_10(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_11(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_12(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_13(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_14(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vpsrld	$23, %ymm0, %ymm0
	vpbroadcastd	.LCPI40_15(%rip), %ymm1
	vmovdqa	%ymm1, 2016(%rsp)
	vpor	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm13
	vaddps	%ymm0, %ymm13, %ymm0
	vaddps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm14
	vmulps	%ymm0, %ymm14, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm5
	vmaxps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm14
	vminps	%ymm14, %ymm0, %ymm0
	vsubps	352(%rsp), %ymm0, %ymm6
	vbroadcastss	.LCPI40_20(%rip), %ymm11
	vaddps	%ymm6, %ymm11, %ymm0
	vmulps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI40_22(%rip), %ymm5
	vmulps	%ymm5, %ymm0, %ymm0
	vcmpgt_oqps	%ymm11, %ymm6, %ymm5
	vblendvps	%ymm5, %ymm6, %ymm0, %ymm7
.Ltmp8676:
	movq	2608(%r14), %rsi
	vmovd	(%rsi,%rdx,4), %xmm0
	vpinsrd	$1, (%rsi,%r12,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%r11,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r10,4), %xmm0, %xmm0
	movq	2080(%rsp), %r10
.Ltmp8677:
	vandps	%ymm4, %ymm10, %ymm5
	vmovaps	%ymm8, %ymm1
	vcmplt_oqps	%ymm8, %ymm5, %ymm5
	vandnps	%ymm4, %ymm5, %ymm4
	vmovaps	%ymm4, 320(%rsp)
.Ltmp8678:
	vmovd	(%rsi,%rcx,4), %xmm4
	vpinsrd	$1, (%rsi,%r9,4), %xmm4, %xmm4
	vpinsrd	$2, (%rsi,%rbx,4), %xmm4, %xmm4
	vpinsrd	$3, (%rsi,%rax,4), %xmm4, %xmm4
.Ltmp8679:
	movq	5216(%r14), %rcx
	movq	2048(%rsp), %r11
	vmovd	(%rcx,%r11,4), %xmm5
	movq	104(%rsp), %r9
	vpinsrd	$1, (%rcx,%r9,4), %xmm5, %xmm5
	vpinsrd	$2, (%rcx,%r13,4), %xmm5, %xmm5
	vpinsrd	$3, (%rcx,%r15,4), %xmm5, %xmm5
.Ltmp8680:
	vinserti128	$1, %xmm0, %ymm4, %ymm0
.Ltmp8681:
	vmovd	(%rcx,%r8,4), %xmm4
	movq	16(%rsp), %rax
	vpinsrd	$1, (%rcx,%rax,4), %xmm4, %xmm4
	movq	1824(%rsp), %rdx
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
	movq	112(%rsp), %rsi
	vpinsrd	$3, (%rcx,%rsi,4), %xmm4, %xmm4
.Ltmp8682:
	movq	5232(%r14), %rcx
	vmovd	(%rcx,%r11,4), %xmm8
	vpinsrd	$1, (%rcx,%r9,4), %xmm8, %xmm8
	vpinsrd	$2, (%rcx,%r13,4), %xmm8, %xmm8
	vpinsrd	$3, (%rcx,%r15,4), %xmm8, %xmm9
.Ltmp8683:
	vinserti128	$1, %xmm5, %ymm4, %ymm2
	vmovdqa	%ymm2, 224(%rsp)
.Ltmp8684:
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rax,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rsi,4), %xmm4, %xmm4
	vinserti128	$1, %xmm9, %ymm4, %ymm3
	vmovdqa	%ymm3, 64(%rsp)
	vbroadcastss	.LCPI40_21(%rip), %ymm4
.Ltmp8685:
	vcmple_oqps	%ymm4, %ymm6, %ymm5
	vmulps	2240(%rsp), %ymm7, %ymm6
	vpxor	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%ymm5, %ymm3, %ymm5
	vpandn	%ymm6, %ymm5, %ymm5
	vbroadcastss	.LCPI40_23(%rip), %ymm13
	vmaxps	%ymm13, %ymm5, %ymm5
	vminps	%ymm3, %ymm5, %ymm5
	vmovaps	1888(%rsp), %ymm9
	vcmplt_oqps	%ymm9, %ymm5, %ymm6
	vmovaps	2560(%rsp), %ymm7
	vblendvps	%ymm6, 2592(%rsp), %ymm7, %ymm6
	vsubps	%ymm5, %ymm9, %ymm7
	vmulps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vandps	%ymm5, %ymm10, %ymm6
	vcmplt_oqps	%ymm1, %ymm6, %ymm6
	vandnps	%ymm5, %ymm6, %ymm7
.Ltmp8686:
	vpand	%ymm0, %ymm10, %ymm0
.Ltmp8687:
	vbroadcastss	.LCPI40_4(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vandps	%ymm0, %ymm15, %ymm5
	vmovaps	2176(%rsp), %ymm1
	vorps	%ymm1, %ymm5, %ymm5
	vbroadcastss	.LCPI40_8(%rip), %ymm3
	vaddps	%ymm3, %ymm5, %ymm5
	vbroadcastss	.LCPI40_9(%rip), %ymm2
	vmulps	%ymm2, %ymm5, %ymm6
	vbroadcastss	.LCPI40_10(%rip), %ymm2
	vaddps	%ymm2, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm6
	vbroadcastss	.LCPI40_11(%rip), %ymm2
	vaddps	%ymm2, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm6
	vbroadcastss	.LCPI40_12(%rip), %ymm2
	vaddps	%ymm2, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm6
	vbroadcastss	.LCPI40_13(%rip), %ymm2
	vaddps	%ymm2, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm6
	vbroadcastss	.LCPI40_14(%rip), %ymm3
	vaddps	%ymm3, %ymm6, %ymm6
	vmulps	%ymm6, %ymm5, %ymm5
	vpsrld	$23, %ymm0, %ymm0
	vpor	2016(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm3
	vaddps	%ymm3, %ymm0, %ymm0
	vaddps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm3
	vmulps	%ymm3, %ymm0, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm3
	vmaxps	%ymm3, %ymm0, %ymm0
	vminps	%ymm14, %ymm0, %ymm0
	vsubps	512(%rsp), %ymm0, %ymm0
	vaddps	%ymm0, %ymm11, %ymm5
	vmulps	%ymm5, %ymm5, %ymm5
	vbroadcastss	.LCPI40_22(%rip), %ymm3
	vmulps	%ymm3, %ymm5, %ymm5
	vcmpgt_oqps	%ymm11, %ymm0, %ymm6
	vblendvps	%ymm6, %ymm0, %ymm5, %ymm5
	vcmple_oqps	%ymm4, %ymm0, %ymm0
	vmulps	2528(%rsp), %ymm5, %ymm5
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%ymm0, %ymm2, %ymm0
	vpandn	%ymm5, %ymm0, %ymm0
	vmovaps	%ymm7, 1888(%rsp)
.Ltmp8688:
	vaddps	480(%rsp), %ymm7, %ymm5
	vbroadcastss	.LCPI40_24(%rip), %ymm12
	vmulps	%ymm5, %ymm12, %ymm5
	vbroadcastss	.LCPI40_25(%rip), %ymm13
	vmaxps	%ymm13, %ymm5, %ymm5
	vbroadcastss	.LCPI40_26(%rip), %ymm14
	vminps	%ymm14, %ymm5, %ymm5
	vroundps	$9, %ymm5, %ymm6
	vsubps	%ymm6, %ymm5, %ymm5
	vbroadcastss	.LCPI40_27(%rip), %ymm7
	vmulps	%ymm7, %ymm5, %ymm7
	vbroadcastss	.LCPI40_28(%rip), %ymm15
	vaddps	%ymm7, %ymm15, %ymm7
	vmulps	%ymm7, %ymm5, %ymm7
	vbroadcastss	.LCPI40_29(%rip), %ymm4
	vaddps	%ymm4, %ymm7, %ymm7
	vmulps	%ymm7, %ymm5, %ymm7
	vbroadcastss	.LCPI40_30(%rip), %ymm8
	vaddps	%ymm7, %ymm8, %ymm7
	vmulps	%ymm7, %ymm5, %ymm7
	vbroadcastss	.LCPI40_31(%rip), %ymm11
	vaddps	%ymm7, %ymm11, %ymm7
	vbroadcastss	.LCPI40_23(%rip), %ymm3
.Ltmp8689:
	vmaxps	%ymm3, %ymm0, %ymm0
	vminps	%ymm2, %ymm0, %ymm0
	vmovaps	1856(%rsp), %ymm13
	vcmplt_oqps	%ymm13, %ymm0, %ymm9
	vmovaps	2464(%rsp), %ymm12
	vblendvps	%ymm9, 2496(%rsp), %ymm12, %ymm9
.Ltmp8690:
	vmulps	%ymm7, %ymm5, %ymm5
.Ltmp8691:
	vsubps	%ymm0, %ymm13, %ymm7
	vmulps	%ymm7, %ymm9, %ymm7
	vaddps	%ymm7, %ymm0, %ymm0
	vandps	%ymm0, %ymm10, %ymm7
	vbroadcastss	.LCPI40_2(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm7, %ymm7
	vandnps	%ymm0, %ymm7, %ymm7
	vbroadcastss	.LCPI40_32(%rip), %ymm12
.Ltmp8692:
	vaddps	%ymm5, %ymm12, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm13
	vaddps	%ymm6, %ymm13, %ymm5
	vpslld	$23, %ymm5, %ymm5
	vmovaps	%ymm7, 1856(%rsp)
.Ltmp8693:
	vaddps	640(%rsp), %ymm7, %ymm7
.Ltmp8694:
	vmulps	%ymm5, %ymm0, %ymm0
	vmovaps	%ymm0, 160(%rsp)
.Ltmp8695:
	vbroadcastss	.LCPI40_24(%rip), %ymm0
	vmulps	%ymm0, %ymm7, %ymm0
	vbroadcastss	.LCPI40_25(%rip), %ymm2
	vmaxps	%ymm2, %ymm0, %ymm0
	vminps	%ymm14, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm7
	vsubps	%ymm7, %ymm0, %ymm0
	vbroadcastss	.LCPI40_27(%rip), %ymm2
	vmulps	%ymm2, %ymm0, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm4, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm8, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm0, %ymm0
.Ltmp8696:
	vandps	224(%rsp), %ymm10, %ymm5
.Ltmp8697:
	vbroadcastss	.LCPI40_4(%rip), %ymm2
	vmaxps	%ymm2, %ymm5, %ymm5
	vbroadcastss	.LCPI40_5(%rip), %ymm2
	vmaxps	%ymm2, %ymm5, %ymm5
	vmovaps	2144(%rsp), %ymm3
	vandps	%ymm3, %ymm5, %ymm8
	vorps	%ymm1, %ymm8, %ymm8
	vmovaps	%ymm1, %ymm11
	vbroadcastss	.LCPI40_8(%rip), %ymm1
	vaddps	%ymm1, %ymm8, %ymm8
	vbroadcastss	.LCPI40_9(%rip), %ymm1
	vmulps	%ymm1, %ymm8, %ymm9
	vbroadcastss	.LCPI40_10(%rip), %ymm1
	vaddps	%ymm1, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vbroadcastss	.LCPI40_11(%rip), %ymm1
	vaddps	%ymm1, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vbroadcastss	.LCPI40_12(%rip), %ymm1
	vaddps	%ymm1, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vbroadcastss	.LCPI40_13(%rip), %ymm1
	vaddps	%ymm1, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vbroadcastss	.LCPI40_14(%rip), %ymm1
	vaddps	%ymm1, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm8
	vpsrld	$23, %ymm5, %ymm5
	vmovdqa	2016(%rsp), %ymm2
	vpor	%ymm2, %ymm5, %ymm5
	vbroadcastss	.LCPI40_16(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
	vaddps	%ymm5, %ymm8, %ymm5
	vmovaps	%ymm12, %ymm8
.Ltmp8698:
	vaddps	%ymm0, %ymm12, %ymm0
	vaddps	%ymm7, %ymm13, %ymm7
	vpslld	$23, %ymm7, %ymm7
	vbroadcastss	.LCPI40_17(%rip), %ymm14
.Ltmp8699:
	vmulps	%ymm5, %ymm14, %ymm5
	vbroadcastss	.LCPI40_18(%rip), %ymm12
	vmaxps	%ymm12, %ymm5, %ymm5
	vbroadcastss	.LCPI40_19(%rip), %ymm1
	vminps	%ymm1, %ymm5, %ymm5
	vsubps	992(%rsp), %ymm5, %ymm5
.Ltmp8700:
	vmulps	%ymm7, %ymm0, %ymm0
	vmovaps	%ymm0, 224(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm4
.Ltmp8701:
	vaddps	%ymm4, %ymm5, %ymm0
	vmulps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI40_22(%rip), %ymm6
	vmulps	%ymm6, %ymm0, %ymm0
	vcmpgt_oqps	%ymm4, %ymm5, %ymm7
	vblendvps	%ymm7, %ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_21(%rip), %ymm12
	vcmple_oqps	%ymm12, %ymm5, %ymm5
	vmulps	2432(%rsp), %ymm0, %ymm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm5, %ymm1, %ymm5
	vpandn	%ymm0, %ymm5, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm5
	vmaxps	%ymm5, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	1984(%rsp), %ymm9
	vcmplt_oqps	%ymm9, %ymm0, %ymm5
	vmovaps	2368(%rsp), %ymm7
	vblendvps	%ymm5, 2400(%rsp), %ymm7, %ymm5
	vsubps	%ymm0, %ymm9, %ymm7
	vmulps	%ymm5, %ymm7, %ymm5
	vaddps	%ymm5, %ymm0, %ymm0
	vandps	%ymm0, %ymm10, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm1
	vcmplt_oqps	%ymm1, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vaddps	1120(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_24(%rip), %ymm14
	vmulps	%ymm0, %ymm14, %ymm0
	vbroadcastss	.LCPI40_25(%rip), %ymm15
	vmaxps	%ymm15, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm5
	vsubps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_27(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm7
	vbroadcastss	.LCPI40_28(%rip), %ymm1
	vaddps	%ymm1, %ymm7, %ymm7
	vmulps	%ymm7, %ymm0, %ymm7
	vbroadcastss	.LCPI40_29(%rip), %ymm1
	vaddps	%ymm1, %ymm7, %ymm7
	vmulps	%ymm7, %ymm0, %ymm7
	vbroadcastss	.LCPI40_30(%rip), %ymm1
	vaddps	%ymm1, %ymm7, %ymm7
	vmulps	%ymm7, %ymm0, %ymm7
	vbroadcastss	.LCPI40_31(%rip), %ymm1
	vaddps	%ymm1, %ymm7, %ymm7
	vmulps	%ymm7, %ymm0, %ymm0
	vaddps	%ymm0, %ymm8, %ymm0
	vaddps	%ymm5, %ymm13, %ymm5
	vpslld	$23, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm9
.Ltmp8702:
	vandps	64(%rsp), %ymm10, %ymm0
.Ltmp8703:
	vbroadcastss	.LCPI40_4(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_5(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm1
	vorps	%ymm1, %ymm11, %ymm1
	vpsrld	$23, %ymm0, %ymm0
	vpor	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_8(%rip), %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
	vbroadcastss	.LCPI40_9(%rip), %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_10(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_11(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_12(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_13(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_14(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm1
	vbroadcastss	.LCPI40_16(%rip), %ymm2
	vaddps	%ymm2, %ymm0, %ymm0
	vaddps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vsubps	1152(%rsp), %ymm0, %ymm0
	vaddps	%ymm4, %ymm0, %ymm1
	vmulps	%ymm1, %ymm1, %ymm1
	vmulps	%ymm6, %ymm1, %ymm1
	vcmpgt_oqps	%ymm4, %ymm0, %ymm2
	vblendvps	%ymm2, %ymm0, %ymm1, %ymm1
	vcmple_oqps	%ymm12, %ymm0, %ymm0
	vmulps	2336(%rsp), %ymm1, %ymm1
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%ymm0, %ymm2, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vminps	%ymm2, %ymm0, %ymm0
	vmovaps	1952(%rsp), %ymm3
	vcmplt_oqps	%ymm3, %ymm0, %ymm1
	vmovaps	2272(%rsp), %ymm2
	vblendvps	%ymm1, 2304(%rsp), %ymm2, %ymm1
	vsubps	%ymm0, %ymm3, %ymm2
	vmulps	%ymm1, %ymm2, %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vandps	%ymm0, %ymm10, %ymm1
	vbroadcastss	.LCPI40_2(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm3
	vaddps	1280(%rsp), %ymm3, %ymm0
	vmulps	%ymm0, %ymm14, %ymm0
	vmaxps	%ymm15, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm1
	vsubps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_27(%rip), %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_28(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_29(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_30(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_31(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm0
	vaddps	%ymm0, %ymm8, %ymm0
	vaddps	%ymm1, %ymm13, %ymm1
	vpslld	$23, %ymm1, %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp8704:
	movq	2592(%r14), %rcx
	vmovaps	160(%rsp), %ymm1
	vmulps	(%rcx,%rdi,4), %ymm1, %ymm1
	movq	2608(%r14), %rcx
	vmovaps	224(%rsp), %ymm2
	vmulps	(%rcx,%rdi,4), %ymm2, %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp8705:
	movq	5216(%r14), %rcx
	vmulps	(%rcx,%rdi,4), %ymm9, %ymm2
	.loc	1 1052 27 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp8706:
	.loc	14 283 14
	vmulps	(%rcx,%rdi,4), %ymm0, %ymm0
.Ltmp8707:
	.loc	14 48 14
	vaddps	%ymm0, %ymm2, %ymm0
	movq	1808(%rsp), %rbx
	movq	128(%rsp), %rdi
.Ltmp8708:
	.loc	8 551 14
	vmovups	%ymm1, (%rbx,%rdi,4)
	movq	2128(%rsp), %r11
.Ltmp8709:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm0, (%r11,%rdi,4)
	movq	120(%rsp), %r12
.Ltmp8710:
	.loc	1 0 0
	incq	%r12
.Ltmp8711:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r12, 312(%rsp)
.Ltmp8712:
	.loc	3 900 12
	jne	.LBB40_339
	jmp	.LBB40_271
.Ltmp8713:
.LBB40_390:
	.loc	1 1086 11
	testq	%rsi, %rsi
	je	.LBB40_511
	.loc	1 0 11 is_stmt 0
	movl	5280(%r14), %eax
	movl	%eax, 1808(%rsp)
	movq	5264(%r14), %rbx
	xorl	%r15d, %r15d
	movq	%rbx, 2016(%rsp)
	jmp	.LBB40_394
.LBB40_392:
	vmovaps	1760(%rsp), %ymm0
.Ltmp8714:
	.loc	1 1057 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r14)
	vmovaps	1664(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r14)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r14)
	vmovaps	1696(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r14)
	vmovaps	256(%rsp), %ymm0
	.loc	1 1058 5
	vmovaps	%ymm0, 4032(%r14)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r14)
	vmovaps	1920(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r14)
	vmovaps	1824(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r14)
	vmovaps	1984(%rsp), %ymm0
	.loc	1 1059 5
	vmovaps	%ymm0, 2304(%r14)
	vmovaps	1856(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r14)
	vmovdqa	1888(%rsp), %ymm0
	.loc	1 1060 5
	vmovdqa	%ymm0, 4928(%r14)
	vmovaps	%ymm8, 4960(%r14)
	.loc	1 1061 5
	movq	%r10, 5272(%r14)
	movq	1656(%rsp), %rsi
.Ltmp8715:
.LBB40_393:
	.loc	1 0 5 is_stmt 0
	movq	1816(%rsp), %r15
	.loc	1 1086 11 is_stmt 1
	cmpq	%rsi, %r15
	jae	.LBB40_511
.LBB40_394:
	.loc	1 1087 42
	subq	%r15, %rsi
	.loc	1 1087 29 is_stmt 0
	movq	%r14, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E12plan_segmentB5_
	movl	%edx, %r13d
	movq	%rax, %r12
.Ltmp8716:
	.loc	1 1176 33 is_stmt 1
	vmovss	(%r14), %xmm0
.Ltmp8717:
	.loc	1 1089 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp8718:
	.loc	1 1176 33
	vmovss	160(%r14), %xmm0
.Ltmp8719:
	.loc	1 1089 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp8720:
	.loc	1 1176 33
	vmovss	320(%r14), %xmm0
.Ltmp8721:
	.loc	1 1089 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp8722:
	.loc	1 1176 33
	vmovss	480(%r14), %xmm0
.Ltmp8723:
	.loc	1 1089 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp8724:
	.loc	1 1176 33
	vmovss	640(%r14), %xmm0
.Ltmp8725:
	.loc	1 1089 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp8726:
	.loc	1 1176 33
	vmovss	800(%r14), %xmm0
.Ltmp8727:
	.loc	1 1089 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp8728:
	.loc	1 1176 33
	vmovss	960(%r14), %xmm0
.Ltmp8729:
	.loc	1 1089 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp8730:
	.loc	1 1176 33
	vmovss	1120(%r14), %xmm0
.Ltmp8731:
	.loc	1 1089 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp8732:
	.loc	1 1176 33
	vmovss	16(%r14), %xmm0
.Ltmp8733:
	.loc	1 1089 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp8734:
	.loc	1 1176 33
	vmovss	176(%r14), %xmm0
.Ltmp8735:
	.loc	1 1089 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp8736:
	.loc	1 1176 33
	vmovss	336(%r14), %xmm0
.Ltmp8737:
	.loc	1 1089 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp8738:
	.loc	1 1176 33
	vmovss	496(%r14), %xmm0
.Ltmp8739:
	.loc	1 1089 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp8740:
	.loc	1 1176 33
	vmovss	656(%r14), %xmm0
.Ltmp8741:
	.loc	1 1089 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp8742:
	.loc	1 1176 33
	vmovss	816(%r14), %xmm0
.Ltmp8743:
	.loc	1 1089 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp8744:
	.loc	1 1176 33
	vmovss	976(%r14), %xmm0
.Ltmp8745:
	.loc	1 1089 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp8746:
	.loc	1 1176 33
	vmovss	1136(%r14), %xmm0
.Ltmp8747:
	.loc	1 1089 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp8748:
	.loc	1 1176 33
	vmovss	32(%r14), %xmm0
.Ltmp8749:
	.loc	1 1089 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp8750:
	.loc	1 1176 33
	vmovss	192(%r14), %xmm0
.Ltmp8751:
	.loc	1 1089 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp8752:
	.loc	1 1176 33
	vmovss	352(%r14), %xmm0
.Ltmp8753:
	.loc	1 1089 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp8754:
	.loc	1 1176 33
	vmovss	512(%r14), %xmm0
.Ltmp8755:
	.loc	1 1089 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp8756:
	.loc	1 1176 33
	vmovss	672(%r14), %xmm0
.Ltmp8757:
	.loc	1 1089 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp8758:
	.loc	1 1176 33
	vmovss	832(%r14), %xmm0
.Ltmp8759:
	.loc	1 1089 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp8760:
	.loc	1 1176 33
	vmovss	992(%r14), %xmm0
.Ltmp8761:
	.loc	1 1089 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp8762:
	.loc	1 1176 33
	vmovss	1152(%r14), %xmm0
.Ltmp8763:
	.loc	1 1089 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp8764:
	.loc	1 1176 33
	vmovss	48(%r14), %xmm0
.Ltmp8765:
	.loc	1 1089 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp8766:
	.loc	1 1176 33
	vmovss	208(%r14), %xmm0
.Ltmp8767:
	.loc	1 1089 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp8768:
	.loc	1 1176 33
	vmovss	368(%r14), %xmm0
.Ltmp8769:
	.loc	1 1089 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp8770:
	.loc	1 1176 33
	vmovss	528(%r14), %xmm0
.Ltmp8771:
	.loc	1 1089 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp8772:
	.loc	1 1176 33
	vmovss	688(%r14), %xmm0
.Ltmp8773:
	.loc	1 1089 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp8774:
	.loc	1 1176 33
	vmovss	848(%r14), %xmm0
.Ltmp8775:
	.loc	1 1089 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp8776:
	.loc	1 1176 33
	vmovss	1008(%r14), %xmm0
.Ltmp8777:
	.loc	1 1089 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp8778:
	.loc	1 1176 33
	vmovss	1168(%r14), %xmm0
.Ltmp8779:
	.loc	1 1089 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp8780:
	.loc	1 1176 33
	vmovss	64(%r14), %xmm0
.Ltmp8781:
	.loc	1 1089 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp8782:
	.loc	1 1176 33
	vmovss	224(%r14), %xmm0
.Ltmp8783:
	.loc	1 1089 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp8784:
	.loc	1 1176 33
	vmovss	384(%r14), %xmm0
.Ltmp8785:
	.loc	1 1089 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp8786:
	.loc	1 1176 33
	vmovss	544(%r14), %xmm0
.Ltmp8787:
	.loc	1 1089 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp8788:
	.loc	1 1176 33
	vmovss	704(%r14), %xmm0
.Ltmp8789:
	.loc	1 1089 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp8790:
	.loc	1 1176 33
	vmovss	864(%r14), %xmm0
.Ltmp8791:
	.loc	1 1089 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp8792:
	.loc	1 1176 33
	vmovss	1024(%r14), %xmm0
.Ltmp8793:
	.loc	1 1089 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp8794:
	.loc	1 1176 33
	vmovss	1184(%r14), %xmm0
.Ltmp8795:
	.loc	1 1089 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp8796:
	.loc	1 1176 33
	vmovss	80(%r14), %xmm0
.Ltmp8797:
	.loc	1 1089 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp8798:
	.loc	1 1176 33
	vmovss	240(%r14), %xmm0
.Ltmp8799:
	.loc	1 1089 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp8800:
	.loc	1 1176 33
	vmovss	400(%r14), %xmm0
.Ltmp8801:
	.loc	1 1089 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp8802:
	.loc	1 1176 33
	vmovss	560(%r14), %xmm0
.Ltmp8803:
	.loc	1 1089 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp8804:
	.loc	1 1176 33
	vmovss	720(%r14), %xmm0
.Ltmp8805:
	.loc	1 1089 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp8806:
	.loc	1 1176 33
	vmovss	880(%r14), %xmm0
.Ltmp8807:
	.loc	1 1089 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp8808:
	.loc	1 1176 33
	vmovss	1040(%r14), %xmm0
.Ltmp8809:
	.loc	1 1089 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp8810:
	.loc	1 1176 33
	vmovss	1200(%r14), %xmm0
.Ltmp8811:
	.loc	1 1089 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp8812:
	.loc	1 1176 33
	vmovss	96(%r14), %xmm0
.Ltmp8813:
	.loc	1 1089 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp8814:
	.loc	1 1176 33
	vmovss	256(%r14), %xmm0
.Ltmp8815:
	.loc	1 1089 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp8816:
	.loc	1 1176 33
	vmovss	416(%r14), %xmm0
.Ltmp8817:
	.loc	1 1089 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp8818:
	.loc	1 1176 33
	vmovss	576(%r14), %xmm0
.Ltmp8819:
	.loc	1 1089 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp8820:
	.loc	1 1176 33
	vmovss	736(%r14), %xmm0
.Ltmp8821:
	.loc	1 1089 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp8822:
	.loc	1 1176 33
	vmovss	896(%r14), %xmm0
.Ltmp8823:
	.loc	1 1089 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp8824:
	.loc	1 1176 33
	vmovss	1056(%r14), %xmm0
.Ltmp8825:
	.loc	1 1089 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp8826:
	.loc	1 1176 33
	vmovss	1216(%r14), %xmm0
.Ltmp8827:
	.loc	1 1089 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp8828:
	.loc	1 1176 33
	vmovss	112(%r14), %xmm0
.Ltmp8829:
	.loc	1 1089 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp8830:
	.loc	1 1176 33
	vmovss	272(%r14), %xmm0
.Ltmp8831:
	.loc	1 1089 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp8832:
	.loc	1 1176 33
	vmovss	432(%r14), %xmm0
.Ltmp8833:
	.loc	1 1089 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp8834:
	.loc	1 1176 33
	vmovss	592(%r14), %xmm0
.Ltmp8835:
	.loc	1 1089 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp8836:
	.loc	1 1176 33
	vmovss	752(%r14), %xmm0
.Ltmp8837:
	.loc	1 1089 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp8838:
	.loc	1 1176 33
	vmovss	912(%r14), %xmm0
.Ltmp8839:
	.loc	1 1089 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp8840:
	.loc	1 1176 33
	vmovss	1072(%r14), %xmm0
.Ltmp8841:
	.loc	1 1089 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp8842:
	.loc	1 1176 33
	vmovss	1232(%r14), %xmm0
.Ltmp8843:
	.loc	1 1089 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp8844:
	.loc	1 1176 33
	vmovss	128(%r14), %xmm0
.Ltmp8845:
	.loc	1 1089 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp8846:
	.loc	1 1176 33
	vmovss	288(%r14), %xmm0
.Ltmp8847:
	.loc	1 1089 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp8848:
	.loc	1 1176 33
	vmovss	448(%r14), %xmm0
.Ltmp8849:
	.loc	1 1089 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp8850:
	.loc	1 1176 33
	vmovss	608(%r14), %xmm0
.Ltmp8851:
	.loc	1 1089 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp8852:
	.loc	1 1176 33
	vmovss	768(%r14), %xmm0
.Ltmp8853:
	.loc	1 1089 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp8854:
	.loc	1 1176 33
	vmovss	928(%r14), %xmm0
.Ltmp8855:
	.loc	1 1089 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp8856:
	.loc	1 1176 33
	vmovss	1088(%r14), %xmm0
.Ltmp8857:
	.loc	1 1089 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp8858:
	.loc	1 1176 33
	vmovss	1248(%r14), %xmm0
.Ltmp8859:
	.loc	1 1089 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp8860:
	.loc	1 1176 33
	vmovss	144(%r14), %xmm0
.Ltmp8861:
	.loc	1 1089 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp8862:
	.loc	1 1176 33
	vmovss	304(%r14), %xmm0
.Ltmp8863:
	.loc	1 1089 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp8864:
	.loc	1 1176 33
	vmovss	464(%r14), %xmm0
.Ltmp8865:
	.loc	1 1089 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp8866:
	.loc	1 1176 33
	vmovss	624(%r14), %xmm0
.Ltmp8867:
	.loc	1 1089 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp8868:
	.loc	1 1176 33
	vmovss	784(%r14), %xmm0
.Ltmp8869:
	.loc	1 1089 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp8870:
	.loc	1 1176 33
	vmovss	944(%r14), %xmm0
.Ltmp8871:
	.loc	1 1089 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp8872:
	.loc	1 1176 33
	vmovss	1104(%r14), %xmm0
.Ltmp8873:
	.loc	1 1089 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp8874:
	.loc	1 1176 33
	vmovss	1264(%r14), %xmm0
.Ltmp8875:
	.loc	1 1089 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp8876:
	.loc	1 1177 32
	vmovss	8(%r14), %xmm0
.Ltmp8877:
	.loc	1 1089 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp8878:
	.loc	1 1177 32
	vmovss	168(%r14), %xmm0
.Ltmp8879:
	.loc	1 1089 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp8880:
	.loc	1 1177 32
	vmovss	328(%r14), %xmm0
.Ltmp8881:
	.loc	1 1089 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp8882:
	.loc	1 1177 32
	vmovss	488(%r14), %xmm0
.Ltmp8883:
	.loc	1 1089 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp8884:
	.loc	1 1177 32
	vmovss	648(%r14), %xmm0
.Ltmp8885:
	.loc	1 1089 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp8886:
	.loc	1 1177 32
	vmovss	808(%r14), %xmm0
.Ltmp8887:
	.loc	1 1089 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp8888:
	.loc	1 1177 32
	vmovss	968(%r14), %xmm0
.Ltmp8889:
	.loc	1 1089 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp8890:
	.loc	1 1177 32
	vmovss	1128(%r14), %xmm0
.Ltmp8891:
	.loc	1 1089 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp8892:
	.loc	1 1177 32
	vmovss	24(%r14), %xmm0
.Ltmp8893:
	.loc	1 1089 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp8894:
	.loc	1 1177 32
	vmovss	184(%r14), %xmm0
.Ltmp8895:
	.loc	1 1089 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp8896:
	.loc	1 1177 32
	vmovss	344(%r14), %xmm0
.Ltmp8897:
	.loc	1 1089 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp8898:
	.loc	1 1177 32
	vmovss	504(%r14), %xmm0
.Ltmp8899:
	.loc	1 1089 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp8900:
	.loc	1 1177 32
	vmovss	664(%r14), %xmm0
.Ltmp8901:
	.loc	1 1089 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp8902:
	.loc	1 1177 32
	vmovss	824(%r14), %xmm0
.Ltmp8903:
	.loc	1 1089 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp8904:
	.loc	1 1177 32
	vmovss	984(%r14), %xmm0
.Ltmp8905:
	.loc	1 1089 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp8906:
	.loc	1 1177 32
	vmovss	1144(%r14), %xmm0
.Ltmp8907:
	.loc	1 1089 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp8908:
	.loc	1 1177 32
	vmovss	40(%r14), %xmm0
.Ltmp8909:
	.loc	1 1089 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp8910:
	.loc	1 1177 32
	vmovss	200(%r14), %xmm0
.Ltmp8911:
	.loc	1 1089 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp8912:
	.loc	1 1177 32
	vmovss	360(%r14), %xmm0
.Ltmp8913:
	.loc	1 1089 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp8914:
	.loc	1 1177 32
	vmovss	520(%r14), %xmm0
.Ltmp8915:
	.loc	1 1089 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp8916:
	.loc	1 1177 32
	vmovss	680(%r14), %xmm0
.Ltmp8917:
	.loc	1 1089 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp8918:
	.loc	1 1177 32
	vmovss	840(%r14), %xmm0
.Ltmp8919:
	.loc	1 1089 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp8920:
	.loc	1 1177 32
	vmovss	1000(%r14), %xmm0
.Ltmp8921:
	.loc	1 1089 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp8922:
	.loc	1 1177 32
	vmovss	1160(%r14), %xmm0
.Ltmp8923:
	.loc	1 1089 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp8924:
	.loc	1 1177 32
	vmovss	56(%r14), %xmm0
.Ltmp8925:
	.loc	1 1089 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp8926:
	.loc	1 1177 32
	vmovss	216(%r14), %xmm0
.Ltmp8927:
	.loc	1 1089 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp8928:
	.loc	1 1177 32
	vmovss	376(%r14), %xmm0
.Ltmp8929:
	.loc	1 1089 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp8930:
	.loc	1 1177 32
	vmovss	536(%r14), %xmm0
.Ltmp8931:
	.loc	1 1089 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp8932:
	.loc	1 1177 32
	vmovss	696(%r14), %xmm0
.Ltmp8933:
	.loc	1 1089 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp8934:
	.loc	1 1177 32
	vmovss	856(%r14), %xmm0
.Ltmp8935:
	.loc	1 1089 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp8936:
	.loc	1 1177 32
	vmovss	1016(%r14), %xmm0
.Ltmp8937:
	.loc	1 1089 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp8938:
	.loc	1 1177 32
	vmovss	1176(%r14), %xmm0
.Ltmp8939:
	.loc	1 1089 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp8940:
	.loc	1 1177 32
	vmovss	72(%r14), %xmm0
.Ltmp8941:
	.loc	1 1089 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp8942:
	.loc	1 1177 32
	vmovss	232(%r14), %xmm0
.Ltmp8943:
	.loc	1 1089 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp8944:
	.loc	1 1177 32
	vmovss	392(%r14), %xmm0
.Ltmp8945:
	.loc	1 1089 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp8946:
	.loc	1 1177 32
	vmovss	552(%r14), %xmm0
.Ltmp8947:
	.loc	1 1089 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp8948:
	.loc	1 1177 32
	vmovss	712(%r14), %xmm0
.Ltmp8949:
	.loc	1 1089 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp8950:
	.loc	1 1177 32
	vmovss	872(%r14), %xmm0
.Ltmp8951:
	.loc	1 1089 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp8952:
	.loc	1 1177 32
	vmovss	1032(%r14), %xmm0
.Ltmp8953:
	.loc	1 1089 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp8954:
	.loc	1 1177 32
	vmovss	1192(%r14), %xmm0
.Ltmp8955:
	.loc	1 1089 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp8956:
	.loc	1 1177 32
	vmovss	88(%r14), %xmm0
.Ltmp8957:
	.loc	1 1089 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp8958:
	.loc	1 1177 32
	vmovss	248(%r14), %xmm0
.Ltmp8959:
	.loc	1 1089 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp8960:
	.loc	1 1177 32
	vmovss	408(%r14), %xmm0
.Ltmp8961:
	.loc	1 1089 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp8962:
	.loc	1 1177 32
	vmovss	568(%r14), %xmm0
.Ltmp8963:
	.loc	1 1089 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp8964:
	.loc	1 1177 32
	vmovss	728(%r14), %xmm0
.Ltmp8965:
	.loc	1 1089 28
	vmovss	%xmm0, 848(%rsp)
.Ltmp8966:
	.loc	1 1177 32
	vmovss	888(%r14), %xmm0
.Ltmp8967:
	.loc	1 1089 28
	vmovss	%xmm0, 852(%rsp)
.Ltmp8968:
	.loc	1 1177 32
	vmovss	1048(%r14), %xmm0
.Ltmp8969:
	.loc	1 1089 28
	vmovss	%xmm0, 856(%rsp)
.Ltmp8970:
	.loc	1 1177 32
	vmovss	1208(%r14), %xmm0
.Ltmp8971:
	.loc	1 1089 28
	vmovss	%xmm0, 860(%rsp)
.Ltmp8972:
	.loc	1 1177 32
	vmovss	104(%r14), %xmm0
.Ltmp8973:
	.loc	1 1089 28
	vmovss	%xmm0, 864(%rsp)
.Ltmp8974:
	.loc	1 1177 32
	vmovss	264(%r14), %xmm0
.Ltmp8975:
	.loc	1 1089 28
	vmovss	%xmm0, 868(%rsp)
.Ltmp8976:
	.loc	1 1177 32
	vmovss	424(%r14), %xmm0
.Ltmp8977:
	.loc	1 1089 28
	vmovss	%xmm0, 872(%rsp)
.Ltmp8978:
	.loc	1 1177 32
	vmovss	584(%r14), %xmm0
.Ltmp8979:
	.loc	1 1089 28
	vmovss	%xmm0, 876(%rsp)
.Ltmp8980:
	.loc	1 1177 32
	vmovss	744(%r14), %xmm0
.Ltmp8981:
	.loc	1 1089 28
	vmovss	%xmm0, 880(%rsp)
.Ltmp8982:
	.loc	1 1177 32
	vmovss	904(%r14), %xmm0
.Ltmp8983:
	.loc	1 1089 28
	vmovss	%xmm0, 884(%rsp)
.Ltmp8984:
	.loc	1 1177 32
	vmovss	1064(%r14), %xmm0
.Ltmp8985:
	.loc	1 1089 28
	vmovss	%xmm0, 888(%rsp)
.Ltmp8986:
	.loc	1 1177 32
	vmovss	1224(%r14), %xmm0
.Ltmp8987:
	.loc	1 1089 28
	vmovss	%xmm0, 892(%rsp)
.Ltmp8988:
	.loc	1 1177 32
	vmovss	120(%r14), %xmm0
.Ltmp8989:
	.loc	1 1089 28
	vmovss	%xmm0, 896(%rsp)
.Ltmp8990:
	.loc	1 1177 32
	vmovss	280(%r14), %xmm0
.Ltmp8991:
	.loc	1 1089 28
	vmovss	%xmm0, 900(%rsp)
.Ltmp8992:
	.loc	1 1177 32
	vmovss	440(%r14), %xmm0
.Ltmp8993:
	.loc	1 1089 28
	vmovss	%xmm0, 904(%rsp)
.Ltmp8994:
	.loc	1 1177 32
	vmovss	600(%r14), %xmm0
.Ltmp8995:
	.loc	1 1089 28
	vmovss	%xmm0, 908(%rsp)
.Ltmp8996:
	.loc	1 1177 32
	vmovss	760(%r14), %xmm0
.Ltmp8997:
	.loc	1 1089 28
	vmovss	%xmm0, 912(%rsp)
.Ltmp8998:
	.loc	1 1177 32
	vmovss	920(%r14), %xmm0
.Ltmp8999:
	.loc	1 1089 28
	vmovss	%xmm0, 916(%rsp)
.Ltmp9000:
	.loc	1 1177 32
	vmovss	1080(%r14), %xmm0
.Ltmp9001:
	.loc	1 1089 28
	vmovss	%xmm0, 920(%rsp)
.Ltmp9002:
	.loc	1 1177 32
	vmovss	1240(%r14), %xmm0
.Ltmp9003:
	.loc	1 1089 28
	vmovss	%xmm0, 924(%rsp)
.Ltmp9004:
	.loc	1 1177 32
	vmovss	136(%r14), %xmm0
.Ltmp9005:
	.loc	1 1089 28
	vmovss	%xmm0, 928(%rsp)
.Ltmp9006:
	.loc	1 1177 32
	vmovss	296(%r14), %xmm0
.Ltmp9007:
	.loc	1 1089 28
	vmovss	%xmm0, 932(%rsp)
.Ltmp9008:
	.loc	1 1177 32
	vmovss	456(%r14), %xmm0
.Ltmp9009:
	.loc	1 1089 28
	vmovss	%xmm0, 936(%rsp)
.Ltmp9010:
	.loc	1 1177 32
	vmovss	616(%r14), %xmm0
.Ltmp9011:
	.loc	1 1089 28
	vmovss	%xmm0, 940(%rsp)
.Ltmp9012:
	.loc	1 1177 32
	vmovss	776(%r14), %xmm0
.Ltmp9013:
	.loc	1 1089 28
	vmovss	%xmm0, 944(%rsp)
.Ltmp9014:
	.loc	1 1177 32
	vmovss	936(%r14), %xmm0
.Ltmp9015:
	.loc	1 1089 28
	vmovss	%xmm0, 948(%rsp)
.Ltmp9016:
	.loc	1 1177 32
	vmovss	1096(%r14), %xmm0
.Ltmp9017:
	.loc	1 1089 28
	vmovss	%xmm0, 952(%rsp)
.Ltmp9018:
	.loc	1 1177 32
	vmovss	1256(%r14), %xmm0
.Ltmp9019:
	.loc	1 1089 28
	vmovss	%xmm0, 956(%rsp)
.Ltmp9020:
	.loc	1 1177 32
	vmovss	152(%r14), %xmm0
.Ltmp9021:
	.loc	1 1089 28
	vmovss	%xmm0, 960(%rsp)
.Ltmp9022:
	.loc	1 1177 32
	vmovss	312(%r14), %xmm0
.Ltmp9023:
	.loc	1 1089 28
	vmovss	%xmm0, 964(%rsp)
.Ltmp9024:
	.loc	1 1177 32
	vmovss	472(%r14), %xmm0
.Ltmp9025:
	.loc	1 1089 28
	vmovss	%xmm0, 968(%rsp)
.Ltmp9026:
	.loc	1 1177 32
	vmovss	632(%r14), %xmm0
.Ltmp9027:
	.loc	1 1089 28
	vmovss	%xmm0, 972(%rsp)
.Ltmp9028:
	.loc	1 1177 32
	vmovss	792(%r14), %xmm0
.Ltmp9029:
	.loc	1 1089 28
	vmovss	%xmm0, 976(%rsp)
.Ltmp9030:
	.loc	1 1177 32
	vmovss	952(%r14), %xmm0
.Ltmp9031:
	.loc	1 1089 28
	vmovss	%xmm0, 980(%rsp)
.Ltmp9032:
	.loc	1 1177 32
	vmovss	1112(%r14), %xmm0
.Ltmp9033:
	.loc	1 1089 28
	vmovss	%xmm0, 984(%rsp)
.Ltmp9034:
	.loc	1 1177 32
	vmovss	1272(%r14), %xmm0
.Ltmp9035:
	.loc	1 1089 28
	vmovss	%xmm0, 988(%rsp)
.Ltmp9036:
	.loc	1 1176 33
	vmovss	2624(%r14), %xmm0
.Ltmp9037:
	.loc	1 1089 28
	vmovss	%xmm0, 992(%rsp)
.Ltmp9038:
	.loc	1 1176 33
	vmovss	2784(%r14), %xmm0
.Ltmp9039:
	.loc	1 1089 28
	vmovss	%xmm0, 996(%rsp)
.Ltmp9040:
	.loc	1 1176 33
	vmovss	2944(%r14), %xmm0
.Ltmp9041:
	.loc	1 1089 28
	vmovss	%xmm0, 1000(%rsp)
.Ltmp9042:
	.loc	1 1176 33
	vmovss	3104(%r14), %xmm0
.Ltmp9043:
	.loc	1 1089 28
	vmovss	%xmm0, 1004(%rsp)
.Ltmp9044:
	.loc	1 1176 33
	vmovss	3264(%r14), %xmm0
.Ltmp9045:
	.loc	1 1089 28
	vmovss	%xmm0, 1008(%rsp)
.Ltmp9046:
	.loc	1 1176 33
	vmovss	3424(%r14), %xmm0
.Ltmp9047:
	.loc	1 1089 28
	vmovss	%xmm0, 1012(%rsp)
.Ltmp9048:
	.loc	1 1176 33
	vmovss	3584(%r14), %xmm0
.Ltmp9049:
	.loc	1 1089 28
	vmovss	%xmm0, 1016(%rsp)
.Ltmp9050:
	.loc	1 1176 33
	vmovss	3744(%r14), %xmm0
.Ltmp9051:
	.loc	1 1089 28
	vmovss	%xmm0, 1020(%rsp)
.Ltmp9052:
	.loc	1 1176 33
	vmovss	2640(%r14), %xmm0
.Ltmp9053:
	.loc	1 1089 28
	vmovss	%xmm0, 1024(%rsp)
.Ltmp9054:
	.loc	1 1176 33
	vmovss	2800(%r14), %xmm0
.Ltmp9055:
	.loc	1 1089 28
	vmovss	%xmm0, 1028(%rsp)
.Ltmp9056:
	.loc	1 1176 33
	vmovss	2960(%r14), %xmm0
.Ltmp9057:
	.loc	1 1089 28
	vmovss	%xmm0, 1032(%rsp)
.Ltmp9058:
	.loc	1 1176 33
	vmovss	3120(%r14), %xmm0
.Ltmp9059:
	.loc	1 1089 28
	vmovss	%xmm0, 1036(%rsp)
.Ltmp9060:
	.loc	1 1176 33
	vmovss	3280(%r14), %xmm0
.Ltmp9061:
	.loc	1 1089 28
	vmovss	%xmm0, 1040(%rsp)
.Ltmp9062:
	.loc	1 1176 33
	vmovss	3440(%r14), %xmm0
.Ltmp9063:
	.loc	1 1089 28
	vmovss	%xmm0, 1044(%rsp)
.Ltmp9064:
	.loc	1 1176 33
	vmovss	3600(%r14), %xmm0
.Ltmp9065:
	.loc	1 1089 28
	vmovss	%xmm0, 1048(%rsp)
.Ltmp9066:
	.loc	1 1176 33
	vmovss	3760(%r14), %xmm0
.Ltmp9067:
	.loc	1 1089 28
	vmovss	%xmm0, 1052(%rsp)
.Ltmp9068:
	.loc	1 1176 33
	vmovss	2656(%r14), %xmm0
.Ltmp9069:
	.loc	1 1089 28
	vmovss	%xmm0, 1056(%rsp)
.Ltmp9070:
	.loc	1 1176 33
	vmovss	2816(%r14), %xmm0
.Ltmp9071:
	.loc	1 1089 28
	vmovss	%xmm0, 1060(%rsp)
.Ltmp9072:
	.loc	1 1176 33
	vmovss	2976(%r14), %xmm0
.Ltmp9073:
	.loc	1 1089 28
	vmovss	%xmm0, 1064(%rsp)
.Ltmp9074:
	.loc	1 1176 33
	vmovss	3136(%r14), %xmm0
.Ltmp9075:
	.loc	1 1089 28
	vmovss	%xmm0, 1068(%rsp)
.Ltmp9076:
	.loc	1 1176 33
	vmovss	3296(%r14), %xmm0
.Ltmp9077:
	.loc	1 1089 28
	vmovss	%xmm0, 1072(%rsp)
.Ltmp9078:
	.loc	1 1176 33
	vmovss	3456(%r14), %xmm0
.Ltmp9079:
	.loc	1 1089 28
	vmovss	%xmm0, 1076(%rsp)
.Ltmp9080:
	.loc	1 1176 33
	vmovss	3616(%r14), %xmm0
.Ltmp9081:
	.loc	1 1089 28
	vmovss	%xmm0, 1080(%rsp)
.Ltmp9082:
	.loc	1 1176 33
	vmovss	3776(%r14), %xmm0
.Ltmp9083:
	.loc	1 1089 28
	vmovss	%xmm0, 1084(%rsp)
.Ltmp9084:
	.loc	1 1176 33
	vmovss	2672(%r14), %xmm0
.Ltmp9085:
	.loc	1 1089 28
	vmovss	%xmm0, 1088(%rsp)
.Ltmp9086:
	.loc	1 1176 33
	vmovss	2832(%r14), %xmm0
.Ltmp9087:
	.loc	1 1089 28
	vmovss	%xmm0, 1092(%rsp)
.Ltmp9088:
	.loc	1 1176 33
	vmovss	2992(%r14), %xmm0
.Ltmp9089:
	.loc	1 1089 28
	vmovss	%xmm0, 1096(%rsp)
.Ltmp9090:
	.loc	1 1176 33
	vmovss	3152(%r14), %xmm0
.Ltmp9091:
	.loc	1 1089 28
	vmovss	%xmm0, 1100(%rsp)
.Ltmp9092:
	.loc	1 1176 33
	vmovss	3312(%r14), %xmm0
.Ltmp9093:
	.loc	1 1089 28
	vmovss	%xmm0, 1104(%rsp)
.Ltmp9094:
	.loc	1 1176 33
	vmovss	3472(%r14), %xmm0
.Ltmp9095:
	.loc	1 1089 28
	vmovss	%xmm0, 1108(%rsp)
.Ltmp9096:
	.loc	1 1176 33
	vmovss	3632(%r14), %xmm0
.Ltmp9097:
	.loc	1 1089 28
	vmovss	%xmm0, 1112(%rsp)
.Ltmp9098:
	.loc	1 1176 33
	vmovss	3792(%r14), %xmm0
.Ltmp9099:
	.loc	1 1089 28
	vmovss	%xmm0, 1116(%rsp)
.Ltmp9100:
	.loc	1 1176 33
	vmovss	2688(%r14), %xmm0
.Ltmp9101:
	.loc	1 1089 28
	vmovss	%xmm0, 1120(%rsp)
.Ltmp9102:
	.loc	1 1176 33
	vmovss	2848(%r14), %xmm0
.Ltmp9103:
	.loc	1 1089 28
	vmovss	%xmm0, 1124(%rsp)
.Ltmp9104:
	.loc	1 1176 33
	vmovss	3008(%r14), %xmm0
.Ltmp9105:
	.loc	1 1089 28
	vmovss	%xmm0, 1128(%rsp)
.Ltmp9106:
	.loc	1 1176 33
	vmovss	3168(%r14), %xmm0
.Ltmp9107:
	.loc	1 1089 28
	vmovss	%xmm0, 1132(%rsp)
.Ltmp9108:
	.loc	1 1176 33
	vmovss	3328(%r14), %xmm0
.Ltmp9109:
	.loc	1 1089 28
	vmovss	%xmm0, 1136(%rsp)
.Ltmp9110:
	.loc	1 1176 33
	vmovss	3488(%r14), %xmm0
.Ltmp9111:
	.loc	1 1089 28
	vmovss	%xmm0, 1140(%rsp)
.Ltmp9112:
	.loc	1 1176 33
	vmovss	3648(%r14), %xmm0
.Ltmp9113:
	.loc	1 1089 28
	vmovss	%xmm0, 1144(%rsp)
.Ltmp9114:
	.loc	1 1176 33
	vmovss	3808(%r14), %xmm0
.Ltmp9115:
	.loc	1 1089 28
	vmovss	%xmm0, 1148(%rsp)
.Ltmp9116:
	.loc	1 1176 33
	vmovss	2704(%r14), %xmm0
.Ltmp9117:
	.loc	1 1089 28
	vmovss	%xmm0, 1152(%rsp)
.Ltmp9118:
	.loc	1 1176 33
	vmovss	2864(%r14), %xmm0
.Ltmp9119:
	.loc	1 1089 28
	vmovss	%xmm0, 1156(%rsp)
.Ltmp9120:
	.loc	1 1176 33
	vmovss	3024(%r14), %xmm0
.Ltmp9121:
	.loc	1 1089 28
	vmovss	%xmm0, 1160(%rsp)
.Ltmp9122:
	.loc	1 1176 33
	vmovss	3184(%r14), %xmm0
.Ltmp9123:
	.loc	1 1089 28
	vmovss	%xmm0, 1164(%rsp)
.Ltmp9124:
	.loc	1 1176 33
	vmovss	3344(%r14), %xmm0
.Ltmp9125:
	.loc	1 1089 28
	vmovss	%xmm0, 1168(%rsp)
.Ltmp9126:
	.loc	1 1176 33
	vmovss	3504(%r14), %xmm0
.Ltmp9127:
	.loc	1 1089 28
	vmovss	%xmm0, 1172(%rsp)
.Ltmp9128:
	.loc	1 1176 33
	vmovss	3664(%r14), %xmm0
.Ltmp9129:
	.loc	1 1089 28
	vmovss	%xmm0, 1176(%rsp)
.Ltmp9130:
	.loc	1 1176 33
	vmovss	3824(%r14), %xmm0
.Ltmp9131:
	.loc	1 1089 28
	vmovss	%xmm0, 1180(%rsp)
.Ltmp9132:
	.loc	1 1176 33
	vmovss	2720(%r14), %xmm0
.Ltmp9133:
	.loc	1 1089 28
	vmovss	%xmm0, 1184(%rsp)
.Ltmp9134:
	.loc	1 1176 33
	vmovss	2880(%r14), %xmm0
.Ltmp9135:
	.loc	1 1089 28
	vmovss	%xmm0, 1188(%rsp)
.Ltmp9136:
	.loc	1 1176 33
	vmovss	3040(%r14), %xmm0
.Ltmp9137:
	.loc	1 1089 28
	vmovss	%xmm0, 1192(%rsp)
.Ltmp9138:
	.loc	1 1176 33
	vmovss	3200(%r14), %xmm0
.Ltmp9139:
	.loc	1 1089 28
	vmovss	%xmm0, 1196(%rsp)
.Ltmp9140:
	.loc	1 1176 33
	vmovss	3360(%r14), %xmm0
.Ltmp9141:
	.loc	1 1089 28
	vmovss	%xmm0, 1200(%rsp)
.Ltmp9142:
	.loc	1 1176 33
	vmovss	3520(%r14), %xmm0
.Ltmp9143:
	.loc	1 1089 28
	vmovss	%xmm0, 1204(%rsp)
.Ltmp9144:
	.loc	1 1176 33
	vmovss	3680(%r14), %xmm0
.Ltmp9145:
	.loc	1 1089 28
	vmovss	%xmm0, 1208(%rsp)
.Ltmp9146:
	.loc	1 1176 33
	vmovss	3840(%r14), %xmm0
.Ltmp9147:
	.loc	1 1089 28
	vmovss	%xmm0, 1212(%rsp)
.Ltmp9148:
	.loc	1 1176 33
	vmovss	2736(%r14), %xmm0
.Ltmp9149:
	.loc	1 1089 28
	vmovss	%xmm0, 1216(%rsp)
.Ltmp9150:
	.loc	1 1176 33
	vmovss	2896(%r14), %xmm0
.Ltmp9151:
	.loc	1 1089 28
	vmovss	%xmm0, 1220(%rsp)
.Ltmp9152:
	.loc	1 1176 33
	vmovss	3056(%r14), %xmm0
.Ltmp9153:
	.loc	1 1089 28
	vmovss	%xmm0, 1224(%rsp)
.Ltmp9154:
	.loc	1 1176 33
	vmovss	3216(%r14), %xmm0
.Ltmp9155:
	.loc	1 1089 28
	vmovss	%xmm0, 1228(%rsp)
.Ltmp9156:
	.loc	1 1176 33
	vmovss	3376(%r14), %xmm0
.Ltmp9157:
	.loc	1 1089 28
	vmovss	%xmm0, 1232(%rsp)
.Ltmp9158:
	.loc	1 1176 33
	vmovss	3536(%r14), %xmm0
.Ltmp9159:
	.loc	1 1089 28
	vmovss	%xmm0, 1236(%rsp)
.Ltmp9160:
	.loc	1 1176 33
	vmovss	3696(%r14), %xmm0
.Ltmp9161:
	.loc	1 1089 28
	vmovss	%xmm0, 1240(%rsp)
.Ltmp9162:
	.loc	1 1176 33
	vmovss	3856(%r14), %xmm0
.Ltmp9163:
	.loc	1 1089 28
	vmovss	%xmm0, 1244(%rsp)
.Ltmp9164:
	.loc	1 1176 33
	vmovss	2752(%r14), %xmm0
.Ltmp9165:
	.loc	1 1089 28
	vmovss	%xmm0, 1248(%rsp)
.Ltmp9166:
	.loc	1 1176 33
	vmovss	2912(%r14), %xmm0
.Ltmp9167:
	.loc	1 1089 28
	vmovss	%xmm0, 1252(%rsp)
.Ltmp9168:
	.loc	1 1176 33
	vmovss	3072(%r14), %xmm0
.Ltmp9169:
	.loc	1 1089 28
	vmovss	%xmm0, 1256(%rsp)
.Ltmp9170:
	.loc	1 1176 33
	vmovss	3232(%r14), %xmm0
.Ltmp9171:
	.loc	1 1089 28
	vmovss	%xmm0, 1260(%rsp)
.Ltmp9172:
	.loc	1 1176 33
	vmovss	3392(%r14), %xmm0
.Ltmp9173:
	.loc	1 1089 28
	vmovss	%xmm0, 1264(%rsp)
.Ltmp9174:
	.loc	1 1176 33
	vmovss	3552(%r14), %xmm0
.Ltmp9175:
	.loc	1 1089 28
	vmovss	%xmm0, 1268(%rsp)
.Ltmp9176:
	.loc	1 1176 33
	vmovss	3712(%r14), %xmm0
.Ltmp9177:
	.loc	1 1089 28
	vmovss	%xmm0, 1272(%rsp)
.Ltmp9178:
	.loc	1 1176 33
	vmovss	3872(%r14), %xmm0
.Ltmp9179:
	.loc	1 1089 28
	vmovss	%xmm0, 1276(%rsp)
.Ltmp9180:
	.loc	1 1176 33
	vmovss	2768(%r14), %xmm0
.Ltmp9181:
	.loc	1 1089 28
	vmovss	%xmm0, 1280(%rsp)
.Ltmp9182:
	.loc	1 1176 33
	vmovss	2928(%r14), %xmm0
.Ltmp9183:
	.loc	1 1089 28
	vmovss	%xmm0, 1284(%rsp)
.Ltmp9184:
	.loc	1 1176 33
	vmovss	3088(%r14), %xmm0
.Ltmp9185:
	.loc	1 1089 28
	vmovss	%xmm0, 1288(%rsp)
.Ltmp9186:
	.loc	1 1176 33
	vmovss	3248(%r14), %xmm0
.Ltmp9187:
	.loc	1 1089 28
	vmovss	%xmm0, 1292(%rsp)
.Ltmp9188:
	.loc	1 1176 33
	vmovss	3408(%r14), %xmm0
.Ltmp9189:
	.loc	1 1089 28
	vmovss	%xmm0, 1296(%rsp)
.Ltmp9190:
	.loc	1 1176 33
	vmovss	3568(%r14), %xmm0
.Ltmp9191:
	.loc	1 1089 28
	vmovss	%xmm0, 1300(%rsp)
.Ltmp9192:
	.loc	1 1176 33
	vmovss	3728(%r14), %xmm0
.Ltmp9193:
	.loc	1 1089 28
	vmovss	%xmm0, 1304(%rsp)
.Ltmp9194:
	.loc	1 1176 33
	vmovss	3888(%r14), %xmm0
.Ltmp9195:
	.loc	1 1089 28
	vmovss	%xmm0, 1308(%rsp)
.Ltmp9196:
	.loc	1 1177 32
	vmovss	2632(%r14), %xmm0
.Ltmp9197:
	.loc	1 1089 28
	vmovss	%xmm0, 1312(%rsp)
.Ltmp9198:
	.loc	1 1177 32
	vmovss	2792(%r14), %xmm0
.Ltmp9199:
	.loc	1 1089 28
	vmovss	%xmm0, 1316(%rsp)
.Ltmp9200:
	.loc	1 1177 32
	vmovss	2952(%r14), %xmm0
.Ltmp9201:
	.loc	1 1089 28
	vmovss	%xmm0, 1320(%rsp)
.Ltmp9202:
	.loc	1 1177 32
	vmovss	3112(%r14), %xmm0
.Ltmp9203:
	.loc	1 1089 28
	vmovss	%xmm0, 1324(%rsp)
.Ltmp9204:
	.loc	1 1177 32
	vmovss	3272(%r14), %xmm0
.Ltmp9205:
	.loc	1 1089 28
	vmovss	%xmm0, 1328(%rsp)
.Ltmp9206:
	.loc	1 1177 32
	vmovss	3432(%r14), %xmm0
.Ltmp9207:
	.loc	1 1089 28
	vmovss	%xmm0, 1332(%rsp)
.Ltmp9208:
	.loc	1 1177 32
	vmovss	3592(%r14), %xmm0
.Ltmp9209:
	.loc	1 1089 28
	vmovss	%xmm0, 1336(%rsp)
.Ltmp9210:
	.loc	1 1177 32
	vmovss	3752(%r14), %xmm0
.Ltmp9211:
	.loc	1 1089 28
	vmovss	%xmm0, 1340(%rsp)
.Ltmp9212:
	.loc	1 1177 32
	vmovss	2648(%r14), %xmm0
.Ltmp9213:
	.loc	1 1089 28
	vmovss	%xmm0, 1344(%rsp)
.Ltmp9214:
	.loc	1 1177 32
	vmovss	2808(%r14), %xmm0
.Ltmp9215:
	.loc	1 1089 28
	vmovss	%xmm0, 1348(%rsp)
.Ltmp9216:
	.loc	1 1177 32
	vmovss	2968(%r14), %xmm0
.Ltmp9217:
	.loc	1 1089 28
	vmovss	%xmm0, 1352(%rsp)
.Ltmp9218:
	.loc	1 1177 32
	vmovss	3128(%r14), %xmm0
.Ltmp9219:
	.loc	1 1089 28
	vmovss	%xmm0, 1356(%rsp)
.Ltmp9220:
	.loc	1 1177 32
	vmovss	3288(%r14), %xmm0
.Ltmp9221:
	.loc	1 1089 28
	vmovss	%xmm0, 1360(%rsp)
.Ltmp9222:
	.loc	1 1177 32
	vmovss	3448(%r14), %xmm0
.Ltmp9223:
	.loc	1 1089 28
	vmovss	%xmm0, 1364(%rsp)
.Ltmp9224:
	.loc	1 1177 32
	vmovss	3608(%r14), %xmm0
.Ltmp9225:
	.loc	1 1089 28
	vmovss	%xmm0, 1368(%rsp)
.Ltmp9226:
	.loc	1 1177 32
	vmovss	3768(%r14), %xmm0
.Ltmp9227:
	.loc	1 1089 28
	vmovss	%xmm0, 1372(%rsp)
.Ltmp9228:
	.loc	1 1177 32
	vmovss	2664(%r14), %xmm0
.Ltmp9229:
	.loc	1 1089 28
	vmovss	%xmm0, 1376(%rsp)
.Ltmp9230:
	.loc	1 1177 32
	vmovss	2824(%r14), %xmm0
.Ltmp9231:
	.loc	1 1089 28
	vmovss	%xmm0, 1380(%rsp)
.Ltmp9232:
	.loc	1 1177 32
	vmovss	2984(%r14), %xmm0
.Ltmp9233:
	.loc	1 1089 28
	vmovss	%xmm0, 1384(%rsp)
.Ltmp9234:
	.loc	1 1177 32
	vmovss	3144(%r14), %xmm0
.Ltmp9235:
	.loc	1 1089 28
	vmovss	%xmm0, 1388(%rsp)
.Ltmp9236:
	.loc	1 1177 32
	vmovss	3304(%r14), %xmm0
.Ltmp9237:
	.loc	1 1089 28
	vmovss	%xmm0, 1392(%rsp)
.Ltmp9238:
	.loc	1 1177 32
	vmovss	3464(%r14), %xmm0
.Ltmp9239:
	.loc	1 1089 28
	vmovss	%xmm0, 1396(%rsp)
.Ltmp9240:
	.loc	1 1177 32
	vmovss	3624(%r14), %xmm0
.Ltmp9241:
	.loc	1 1089 28
	vmovss	%xmm0, 1400(%rsp)
.Ltmp9242:
	.loc	1 1177 32
	vmovss	3784(%r14), %xmm0
.Ltmp9243:
	.loc	1 1089 28
	vmovss	%xmm0, 1404(%rsp)
.Ltmp9244:
	.loc	1 1177 32
	vmovss	2680(%r14), %xmm0
.Ltmp9245:
	.loc	1 1089 28
	vmovss	%xmm0, 1408(%rsp)
.Ltmp9246:
	.loc	1 1177 32
	vmovss	2840(%r14), %xmm0
.Ltmp9247:
	.loc	1 1089 28
	vmovss	%xmm0, 1412(%rsp)
.Ltmp9248:
	.loc	1 1177 32
	vmovss	3000(%r14), %xmm0
.Ltmp9249:
	.loc	1 1089 28
	vmovss	%xmm0, 1416(%rsp)
.Ltmp9250:
	.loc	1 1177 32
	vmovss	3160(%r14), %xmm0
.Ltmp9251:
	.loc	1 1089 28
	vmovss	%xmm0, 1420(%rsp)
.Ltmp9252:
	.loc	1 1177 32
	vmovss	3320(%r14), %xmm0
.Ltmp9253:
	.loc	1 1089 28
	vmovss	%xmm0, 1424(%rsp)
.Ltmp9254:
	.loc	1 1177 32
	vmovss	3480(%r14), %xmm0
.Ltmp9255:
	.loc	1 1089 28
	vmovss	%xmm0, 1428(%rsp)
.Ltmp9256:
	.loc	1 1177 32
	vmovss	3640(%r14), %xmm0
.Ltmp9257:
	.loc	1 1089 28
	vmovss	%xmm0, 1432(%rsp)
.Ltmp9258:
	.loc	1 1177 32
	vmovss	3800(%r14), %xmm0
.Ltmp9259:
	.loc	1 1089 28
	vmovss	%xmm0, 1436(%rsp)
.Ltmp9260:
	.loc	1 1177 32
	vmovss	2696(%r14), %xmm0
.Ltmp9261:
	.loc	1 1089 28
	vmovss	%xmm0, 1440(%rsp)
.Ltmp9262:
	.loc	1 1177 32
	vmovss	2856(%r14), %xmm0
.Ltmp9263:
	.loc	1 1089 28
	vmovss	%xmm0, 1444(%rsp)
.Ltmp9264:
	.loc	1 1177 32
	vmovss	3016(%r14), %xmm0
.Ltmp9265:
	.loc	1 1089 28
	vmovss	%xmm0, 1448(%rsp)
.Ltmp9266:
	.loc	1 1177 32
	vmovss	3176(%r14), %xmm0
.Ltmp9267:
	.loc	1 1089 28
	vmovss	%xmm0, 1452(%rsp)
.Ltmp9268:
	.loc	1 1177 32
	vmovss	3336(%r14), %xmm0
.Ltmp9269:
	.loc	1 1089 28
	vmovss	%xmm0, 1456(%rsp)
.Ltmp9270:
	.loc	1 1177 32
	vmovss	3496(%r14), %xmm0
.Ltmp9271:
	.loc	1 1089 28
	vmovss	%xmm0, 1460(%rsp)
.Ltmp9272:
	.loc	1 1177 32
	vmovss	3656(%r14), %xmm0
.Ltmp9273:
	.loc	1 1089 28
	vmovss	%xmm0, 1464(%rsp)
.Ltmp9274:
	.loc	1 1177 32
	vmovss	3816(%r14), %xmm0
.Ltmp9275:
	.loc	1 1089 28
	vmovss	%xmm0, 1468(%rsp)
.Ltmp9276:
	.loc	1 1177 32
	vmovss	2712(%r14), %xmm0
.Ltmp9277:
	.loc	1 1089 28
	vmovss	%xmm0, 1472(%rsp)
.Ltmp9278:
	.loc	1 1177 32
	vmovss	2872(%r14), %xmm0
.Ltmp9279:
	.loc	1 1089 28
	vmovss	%xmm0, 1476(%rsp)
.Ltmp9280:
	.loc	1 1177 32
	vmovss	3032(%r14), %xmm0
.Ltmp9281:
	.loc	1 1089 28
	vmovss	%xmm0, 1480(%rsp)
.Ltmp9282:
	.loc	1 1177 32
	vmovss	3192(%r14), %xmm0
.Ltmp9283:
	.loc	1 1089 28
	vmovss	%xmm0, 1484(%rsp)
.Ltmp9284:
	.loc	1 1177 32
	vmovss	3352(%r14), %xmm0
.Ltmp9285:
	.loc	1 1089 28
	vmovss	%xmm0, 1488(%rsp)
.Ltmp9286:
	.loc	1 1177 32
	vmovss	3512(%r14), %xmm0
.Ltmp9287:
	.loc	1 1089 28
	vmovss	%xmm0, 1492(%rsp)
.Ltmp9288:
	.loc	1 1177 32
	vmovss	3672(%r14), %xmm0
.Ltmp9289:
	.loc	1 1089 28
	vmovss	%xmm0, 1496(%rsp)
.Ltmp9290:
	.loc	1 1177 32
	vmovss	3832(%r14), %xmm0
.Ltmp9291:
	.loc	1 1089 28
	vmovss	%xmm0, 1500(%rsp)
.Ltmp9292:
	.loc	1 1177 32
	vmovss	2728(%r14), %xmm0
.Ltmp9293:
	.loc	1 1089 28
	vmovss	%xmm0, 1504(%rsp)
.Ltmp9294:
	.loc	1 1177 32
	vmovss	2888(%r14), %xmm0
.Ltmp9295:
	.loc	1 1089 28
	vmovss	%xmm0, 1508(%rsp)
.Ltmp9296:
	.loc	1 1177 32
	vmovss	3048(%r14), %xmm0
.Ltmp9297:
	.loc	1 1089 28
	vmovss	%xmm0, 1512(%rsp)
.Ltmp9298:
	.loc	1 1177 32
	vmovss	3208(%r14), %xmm0
.Ltmp9299:
	.loc	1 1089 28
	vmovss	%xmm0, 1516(%rsp)
.Ltmp9300:
	.loc	1 1177 32
	vmovss	3368(%r14), %xmm0
.Ltmp9301:
	.loc	1 1089 28
	vmovss	%xmm0, 1520(%rsp)
.Ltmp9302:
	.loc	1 1177 32
	vmovss	3528(%r14), %xmm0
.Ltmp9303:
	.loc	1 1089 28
	vmovss	%xmm0, 1524(%rsp)
.Ltmp9304:
	.loc	1 1177 32
	vmovss	3688(%r14), %xmm0
.Ltmp9305:
	.loc	1 1089 28
	vmovss	%xmm0, 1528(%rsp)
.Ltmp9306:
	.loc	1 1177 32
	vmovss	3848(%r14), %xmm0
.Ltmp9307:
	.loc	1 1089 28
	vmovss	%xmm0, 1532(%rsp)
.Ltmp9308:
	.loc	1 1177 32
	vmovss	2744(%r14), %xmm0
.Ltmp9309:
	.loc	1 1089 28
	vmovss	%xmm0, 1536(%rsp)
.Ltmp9310:
	.loc	1 1177 32
	vmovss	2904(%r14), %xmm0
.Ltmp9311:
	.loc	1 1089 28
	vmovss	%xmm0, 1540(%rsp)
.Ltmp9312:
	.loc	1 1177 32
	vmovss	3064(%r14), %xmm0
.Ltmp9313:
	.loc	1 1089 28
	vmovss	%xmm0, 1544(%rsp)
.Ltmp9314:
	.loc	1 1177 32
	vmovss	3224(%r14), %xmm0
.Ltmp9315:
	.loc	1 1089 28
	vmovss	%xmm0, 1548(%rsp)
.Ltmp9316:
	.loc	1 1177 32
	vmovss	3384(%r14), %xmm0
.Ltmp9317:
	.loc	1 1089 28
	vmovss	%xmm0, 1552(%rsp)
.Ltmp9318:
	.loc	1 1177 32
	vmovss	3544(%r14), %xmm0
.Ltmp9319:
	.loc	1 1089 28
	vmovss	%xmm0, 1556(%rsp)
.Ltmp9320:
	.loc	1 1177 32
	vmovss	3704(%r14), %xmm0
.Ltmp9321:
	.loc	1 1089 28
	vmovss	%xmm0, 1560(%rsp)
.Ltmp9322:
	.loc	1 1177 32
	vmovss	3864(%r14), %xmm0
.Ltmp9323:
	.loc	1 1089 28
	vmovss	%xmm0, 1564(%rsp)
.Ltmp9324:
	.loc	1 1177 32
	vmovss	2760(%r14), %xmm0
.Ltmp9325:
	.loc	1 1089 28
	vmovss	%xmm0, 1568(%rsp)
.Ltmp9326:
	.loc	1 1177 32
	vmovss	2920(%r14), %xmm0
.Ltmp9327:
	.loc	1 1089 28
	vmovss	%xmm0, 1572(%rsp)
.Ltmp9328:
	.loc	1 1177 32
	vmovss	3080(%r14), %xmm0
.Ltmp9329:
	.loc	1 1089 28
	vmovss	%xmm0, 1576(%rsp)
.Ltmp9330:
	.loc	1 1177 32
	vmovss	3240(%r14), %xmm0
.Ltmp9331:
	.loc	1 1089 28
	vmovss	%xmm0, 1580(%rsp)
.Ltmp9332:
	.loc	1 1177 32
	vmovss	3400(%r14), %xmm0
.Ltmp9333:
	.loc	1 1089 28
	vmovss	%xmm0, 1584(%rsp)
.Ltmp9334:
	.loc	1 1177 32
	vmovss	3560(%r14), %xmm0
.Ltmp9335:
	.loc	1 1089 28
	vmovss	%xmm0, 1588(%rsp)
.Ltmp9336:
	.loc	1 1177 32
	vmovss	3720(%r14), %xmm0
.Ltmp9337:
	.loc	1 1089 28
	vmovss	%xmm0, 1592(%rsp)
.Ltmp9338:
	.loc	1 1177 32
	vmovss	3880(%r14), %xmm0
.Ltmp9339:
	.loc	1 1089 28
	vmovss	%xmm0, 1596(%rsp)
.Ltmp9340:
	.loc	1 1177 32
	vmovss	2776(%r14), %xmm0
.Ltmp9341:
	.loc	1 1089 28
	vmovss	%xmm0, 1600(%rsp)
.Ltmp9342:
	.loc	1 1177 32
	vmovss	2936(%r14), %xmm0
.Ltmp9343:
	.loc	1 1089 28
	vmovss	%xmm0, 1604(%rsp)
.Ltmp9344:
	.loc	1 1177 32
	vmovss	3096(%r14), %xmm0
.Ltmp9345:
	.loc	1 1089 28
	vmovss	%xmm0, 1608(%rsp)
.Ltmp9346:
	.loc	1 1177 32
	vmovss	3256(%r14), %xmm0
.Ltmp9347:
	.loc	1 1089 28
	vmovss	%xmm0, 1612(%rsp)
.Ltmp9348:
	.loc	1 1177 32
	vmovss	3416(%r14), %xmm0
.Ltmp9349:
	.loc	1 1089 28
	vmovss	%xmm0, 1616(%rsp)
.Ltmp9350:
	.loc	1 1177 32
	vmovss	3576(%r14), %xmm0
.Ltmp9351:
	.loc	1 1089 28
	vmovss	%xmm0, 1620(%rsp)
.Ltmp9352:
	.loc	1 1177 32
	vmovss	3736(%r14), %xmm0
.Ltmp9353:
	.loc	1 1089 28
	vmovss	%xmm0, 1624(%rsp)
.Ltmp9354:
	.loc	1 1177 32
	vmovss	3896(%r14), %xmm0
.Ltmp9355:
	.loc	1 1089 28
	vmovss	%xmm0, 1628(%rsp)
.Ltmp9356:
	.loc	1 1091 31
	leaq	3008(%rsp), %rdi
	movq	%r14, %rsi
	movl	1808(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	3200(%rsp), %rdi
	leaq	2624(%r14), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovaps	3008(%rsp), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	3040(%rsp), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	3072(%rsp), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	3104(%rsp), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	3136(%rsp), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	3168(%rsp), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	3200(%rsp), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	3232(%rsp), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	3328(%rsp), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	3360(%rsp), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
.Ltmp9357:
	.loc	1 0 0 is_stmt 0
	leaq	(%r12,%r15), %rax
	shlq	$3, %r15
	leaq	(,%rax,8), %rsi
	.loc	1 1094 12 is_stmt 1
	testb	$1, %r13b
	movq	%r12, 312(%rsp)
	movq	%rax, 1816(%rsp)
	je	.LBB40_455
.Ltmp9358:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp9359:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_569
.Ltmp9360:
	.loc	48 451 16 is_stmt 1
	cmpq	40(%rsp), %rsi
	ja	.LBB40_569
.Ltmp9361:
	.loc	48 451 16 is_stmt 0
	cmpq	32(%rsp), %rsi
	ja	.LBB40_573
.Ltmp9362:
	.loc	1 972 27 is_stmt 1
	vmovaps	1408(%r14), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1440(%r14), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1472(%r14), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1504(%r14), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp9363:
	.loc	1 973 26
	vmovaps	4032(%r14), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	4064(%r14), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r14), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	4128(%r14), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
.Ltmp9364:
	.loc	1 974 25
	vmovaps	2304(%r14), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	2336(%r14), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
.Ltmp9365:
	.loc	1 975 24
	vmovaps	4928(%r14), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	4960(%r14), %ymm6
.Ltmp9366:
	.loc	1 976 24
	movq	5272(%r14), %r13
.Ltmp9367:
	.loc	2 1916 50
	testq	%r12, %r12
	movq	2016(%rsp), %rbx
	je	.LBB40_451
.Ltmp9368:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r12,8), %rax
	movq	%rax, 2048(%rsp)
	movq	296(%rsp), %rax
	leaq	(%rax,%r15,4), %r10
	movq	304(%rsp), %rax
	leaq	(%rax,%r15,4), %r15
.Ltmp9369:
	.loc	3 900 12 is_stmt 1
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r12
	movq	%r12, 2176(%rsp)
	xorl	%edi, %edi
	xorl	%r11d, %r11d
	movq	%r10, 2144(%rsp)
	movq	%r15, 56(%rsp)
.Ltmp9370:
	.loc	3 0 12 is_stmt 0
.Ltmp9371:
	.p2align	4
.LBB40_400:
	.loc	1 981 21 is_stmt 1
	vmovaps	352(%rsp), %ymm0
	vmovaps	384(%rsp), %ymm1
	vmovaps	416(%rsp), %ymm2
.Ltmp9372:
	.loc	14 48 14
	vaddps	672(%rsp), %ymm0, %ymm0
.Ltmp9373:
	.loc	1 980 17
	vmovaps	%ymm0, 352(%rsp)
	.loc	1 983 21
	vmovaps	992(%rsp), %ymm0
.Ltmp9374:
	.loc	14 48 14
	vaddps	1312(%rsp), %ymm0, %ymm0
.Ltmp9375:
	.loc	1 982 17
	vmovaps	%ymm0, 992(%rsp)
.Ltmp9376:
	.loc	14 48 14
	vaddps	704(%rsp), %ymm1, %ymm0
.Ltmp9377:
	.loc	1 980 17
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 983 21
	vmovaps	1024(%rsp), %ymm0
.Ltmp9378:
	.loc	14 48 14
	vaddps	1344(%rsp), %ymm0, %ymm0
.Ltmp9379:
	.loc	1 982 17
	vmovaps	%ymm0, 1024(%rsp)
.Ltmp9380:
	.loc	14 48 14
	vaddps	736(%rsp), %ymm2, %ymm0
.Ltmp9381:
	.loc	1 980 17
	vmovaps	%ymm0, 416(%rsp)
	.loc	1 983 21
	vmovaps	1056(%rsp), %ymm0
.Ltmp9382:
	.loc	14 48 14
	vaddps	1376(%rsp), %ymm0, %ymm0
.Ltmp9383:
	.loc	1 982 17
	vmovaps	%ymm0, 1056(%rsp)
	.loc	1 981 21
	vmovaps	448(%rsp), %ymm0
.Ltmp9384:
	.loc	14 48 14
	vaddps	768(%rsp), %ymm0, %ymm0
.Ltmp9385:
	.loc	1 980 17
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 983 21
	vmovaps	1088(%rsp), %ymm0
.Ltmp9386:
	.loc	14 48 14
	vaddps	1408(%rsp), %ymm0, %ymm0
.Ltmp9387:
	.loc	1 982 17
	vmovaps	%ymm0, 1088(%rsp)
	.loc	1 981 21
	vmovaps	480(%rsp), %ymm0
.Ltmp9388:
	.loc	14 48 14
	vaddps	800(%rsp), %ymm0, %ymm0
.Ltmp9389:
	.loc	1 980 17
	vmovaps	%ymm0, 480(%rsp)
	.loc	1 983 21
	vmovaps	1120(%rsp), %ymm0
.Ltmp9390:
	.loc	14 48 14
	vaddps	1440(%rsp), %ymm0, %ymm0
.Ltmp9391:
	.loc	1 982 17
	vmovaps	%ymm0, 1120(%rsp)
	.loc	1 981 21
	vmovaps	512(%rsp), %ymm0
.Ltmp9392:
	.loc	14 48 14
	vaddps	832(%rsp), %ymm0, %ymm0
.Ltmp9393:
	.loc	1 980 17
	vmovaps	%ymm0, 512(%rsp)
	.loc	1 983 21
	vmovaps	1152(%rsp), %ymm0
.Ltmp9394:
	.loc	14 48 14
	vaddps	1472(%rsp), %ymm0, %ymm0
.Ltmp9395:
	.loc	1 982 17
	vmovaps	%ymm0, 1152(%rsp)
	.loc	1 981 21
	vmovaps	544(%rsp), %ymm0
.Ltmp9396:
	.loc	14 48 14
	vaddps	864(%rsp), %ymm0, %ymm0
.Ltmp9397:
	.loc	1 980 17
	vmovaps	%ymm0, 544(%rsp)
	.loc	1 983 21
	vmovaps	1184(%rsp), %ymm0
.Ltmp9398:
	.loc	14 48 14
	vaddps	1504(%rsp), %ymm0, %ymm0
.Ltmp9399:
	.loc	1 982 17
	vmovaps	%ymm0, 1184(%rsp)
	.loc	1 981 21
	vmovaps	576(%rsp), %ymm0
.Ltmp9400:
	.loc	14 48 14
	vaddps	896(%rsp), %ymm0, %ymm0
.Ltmp9401:
	.loc	1 980 17
	vmovaps	%ymm0, 576(%rsp)
	.loc	1 983 21
	vmovaps	1216(%rsp), %ymm0
.Ltmp9402:
	.loc	14 48 14
	vaddps	1536(%rsp), %ymm0, %ymm0
.Ltmp9403:
	.loc	1 982 17
	vmovaps	%ymm0, 1216(%rsp)
	.loc	1 981 21
	vmovaps	608(%rsp), %ymm0
.Ltmp9404:
	.loc	14 48 14
	vaddps	928(%rsp), %ymm0, %ymm0
.Ltmp9405:
	.loc	1 980 17
	vmovaps	%ymm0, 608(%rsp)
	.loc	1 983 21
	vmovaps	1248(%rsp), %ymm0
.Ltmp9406:
	.loc	14 48 14
	vaddps	1568(%rsp), %ymm0, %ymm0
.Ltmp9407:
	.loc	1 982 17
	vmovaps	%ymm0, 1248(%rsp)
	.loc	1 981 21
	vmovaps	640(%rsp), %ymm0
.Ltmp9408:
	.loc	14 48 14
	vaddps	960(%rsp), %ymm0, %ymm0
.Ltmp9409:
	.loc	1 980 17
	vmovaps	%ymm0, 640(%rsp)
	.loc	1 983 21
	vmovaps	1280(%rsp), %ymm0
.Ltmp9410:
	.loc	14 48 14
	vaddps	1600(%rsp), %ymm0, %ymm0
.Ltmp9411:
	.loc	1 982 17
	vmovaps	%ymm0, 1280(%rsp)
.Ltmp9412:
	.loc	1 987 28
	leaq	1(%r13), %rax
.Ltmp9413:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %r9d
	cmovaeq	%rbx, %r9
.Ltmp9414:
	.loc	48 568 12
	cmpq	2048(%rsp), %rdi
	ja	.LBB40_555
.Ltmp9415:
	.loc	48 438 16
	cmpq	%r11, 2176(%rsp)
	je	.LBB40_547
.Ltmp9416:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r13,8), %rax
.Ltmp9417:
	.loc	1 1000 29 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp9418:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_554
.Ltmp9419:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp9420:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm6, 2208(%rsp)
	vmovups	(%r10,%rdi,4), %ymm1
.Ltmp9421:
	vmovups	(%r15,%rdi,4), %ymm15
.Ltmp9422:
	vmovaps	1280(%r14), %ymm10
	vmovaps	1312(%r14), %ymm11
	vmovaps	1344(%r14), %ymm0
	vmovaps	3904(%r14), %ymm14
	vmovaps	3936(%r14), %ymm9
	vmovaps	3968(%r14), %ymm13
	vmovaps	1664(%rsp), %ymm6
	vsubps	%ymm6, %ymm1, %ymm2
	vmulps	%ymm2, %ymm11, %ymm3
	vmovaps	1760(%rsp), %ymm7
	vmulps	%ymm7, %ymm10, %ymm4
	vaddps	%ymm3, %ymm4, %ymm5
	vaddps	%ymm5, %ymm7, %ymm3
	vmulps	%ymm7, %ymm11, %ymm4
	vmulps	%ymm0, %ymm2, %ymm2
	vaddps	%ymm2, %ymm4, %ymm4
	vaddps	%ymm4, %ymm6, %ymm2
	vmulps	1376(%r14), %ymm3, %ymm12
	vmovaps	1696(%rsp), %ymm6
	vsubps	%ymm6, %ymm2, %ymm3
	vmulps	1728(%rsp), %ymm11, %ymm2
	vmulps	%ymm3, %ymm0, %ymm0
	vaddps	%ymm0, %ymm2, %ymm0
	vaddps	%ymm0, %ymm6, %ymm2
.Ltmp9423:
	vsubps	192(%rsp), %ymm15, %ymm6
	vmulps	%ymm6, %ymm9, %ymm7
	vmovaps	256(%rsp), %ymm8
	vmovaps	%ymm14, 1952(%rsp)
	vmulps	%ymm14, %ymm8, %ymm14
	vaddps	%ymm7, %ymm14, %ymm14
	vaddps	%ymm14, %ymm8, %ymm7
	vmulps	4000(%r14), %ymm7, %ymm7
.Ltmp9424:
	.loc	1 1000 29 is_stmt 1
	movq	2592(%r14), %rcx
.Ltmp9425:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp9426:
	.loc	1 1001 30
	movq	2616(%r14), %rsi
.Ltmp9427:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_559
.Ltmp9428:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp9429:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm1, %ymm12, %ymm1
	vsubps	%ymm2, %ymm1, %ymm1
.Ltmp9430:
	.loc	1 1001 30 is_stmt 1
	movq	2608(%r14), %rcx
.Ltmp9431:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rax,4)
.Ltmp9432:
	.loc	1 1002 28
	movq	5224(%r14), %rsi
.Ltmp9433:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_560
.Ltmp9434:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp9435:
	.loc	1 0 0 is_stmt 0
	vmulps	256(%rsp), %ymm9, %ymm1
	vmulps	%ymm6, %ymm13, %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
	vaddps	192(%rsp), %ymm1, %ymm2
	vmovaps	1920(%rsp), %ymm8
	vsubps	%ymm8, %ymm2, %ymm6
	vmulps	320(%rsp), %ymm9, %ymm2
	vmulps	%ymm6, %ymm13, %ymm12
	vaddps	%ymm2, %ymm12, %ymm13
	vaddps	%ymm13, %ymm8, %ymm2
.Ltmp9436:
	.loc	1 1002 28 is_stmt 1
	movq	5216(%r14), %rcx
.Ltmp9437:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp9438:
	.loc	1 1003 29
	movq	5240(%r14), %rsi
.Ltmp9439:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_561
.Ltmp9440:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp9441:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm7, %ymm15, %ymm7
	vsubps	%ymm2, %ymm7, %ymm2
.Ltmp9442:
	.loc	1 1003 29 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp9443:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
	movq	2368(%r14), %rcx
.Ltmp9444:
	.loc	1 877 35
	addq	%r13, %rcx
.Ltmp9445:
	.loc	1 857 8
	cmpq	%rbx, %rcx
	movl	$0, %eax
	cmovaeq	%rbx, %rax
.Ltmp9446:
	.loc	1 1006 34
	movq	2600(%r14), %rsi
.Ltmp9447:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp9448:
	.loc	1 877 30
	shlq	$3, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_637
	.loc	1 0 25
	movq	2376(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9449:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9450:
	.loc	1 877 30
	leaq	1(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_591
	.loc	1 0 25
	movq	2384(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9451:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9452:
	.loc	1 877 30
	leaq	2(,%rax,8), %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_588
	.loc	1 0 25
	movq	2392(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9453:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9454:
	.loc	1 877 30
	leaq	3(,%rax,8), %rax
	movq	%rax, 64(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_587
	.loc	1 0 25
	movq	2400(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9455:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9456:
	.loc	1 877 30
	leaq	4(,%rax,8), %rax
	movq	%rax, 160(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_595
	.loc	1 0 25
	movq	2408(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9457:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9458:
	.loc	1 877 30
	leaq	5(,%rax,8), %rax
	movq	%rax, 224(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_597
	.loc	1 0 25
	movq	2416(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9459:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9460:
	.loc	1 877 30
	leaq	6(,%rax,8), %rax
	movq	%rax, 24(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_594
	.loc	1 0 25
	movq	2424(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9461:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9462:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_612
.Ltmp9463:
	.loc	1 1008 34 is_stmt 1
	movq	2616(%r14), %rsi
.Ltmp9464:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB40_637
	cmpq	%rsi, %r12
	jae	.LBB40_591
	cmpq	%rsi, %r8
	jae	.LBB40_588
	cmpq	%rsi, 64(%rsp)
	jae	.LBB40_636
	cmpq	%rsi, 160(%rsp)
	jae	.LBB40_595
	cmpq	%rsi, 224(%rsp)
	jae	.LBB40_597
	cmpq	%rsi, 24(%rsp)
	jae	.LBB40_602
	.loc	1 0 25 is_stmt 0
	movq	%r8, 2080(%rsp)
	.loc	1 877 25
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp9465:
	.loc	1 0 25
	movq	%rax, 152(%rsp)
	movq	4992(%r14), %r8
.Ltmp9466:
	.loc	1 877 35
	addq	%r13, %r8
.Ltmp9467:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r8
	movl	$0, %eax
	cmovaeq	%rbx, %rax
.Ltmp9468:
	.loc	1 1010 34
	movq	5224(%r14), %rsi
.Ltmp9469:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp9470:
	.loc	1 877 30
	shlq	$3, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_635
	.loc	1 0 25
	movq	%r11, 144(%rsp)
	movq	5000(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9471:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9472:
	.loc	1 877 30
	leaq	1(,%rax,8), %rax
	movq	%rax, 16(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_581
	.loc	1 0 25
	movq	5008(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9473:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9474:
	.loc	1 877 30
	leaq	2(,%rax,8), %rax
	movq	%rax, 1824(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_634
	.loc	1 0 25
	movq	%r12, 136(%rsp)
	movq	5016(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9475:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9476:
	.loc	1 877 30
	leaq	3(,%rax,8), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_585
	.loc	1 0 25
	movq	%rdi, 128(%rsp)
	movq	5024(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9477:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp9478:
	.loc	1 877 30
	leaq	4(,%rax,8), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_598
	.loc	1 0 25
	movq	5032(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9479:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp9480:
	.loc	1 877 30
	leaq	5(,%rax,8), %r10
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r10
	jae	.LBB40_605
	.loc	1 0 25
	movq	5040(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9481:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp9482:
	.loc	1 877 30
	leaq	6(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_586
	.loc	1 0 25
	movq	5048(%r14), %rax
	.loc	1 877 35
	addq	%r13, %rax
.Ltmp9483:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp9484:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_618
.Ltmp9485:
	.loc	1 1012 34 is_stmt 1
	movq	5240(%r14), %rsi
.Ltmp9486:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB40_635
	cmpq	%rsi, 16(%rsp)
	jae	.LBB40_581
	cmpq	%rsi, 1824(%rsp)
	jae	.LBB40_596
	cmpq	%rsi, %rdx
	jae	.LBB40_585
	cmpq	%rsi, %r15
	jae	.LBB40_607
	cmpq	%rsi, %r10
	jae	.LBB40_590
	cmpq	%rsi, %r12
	jae	.LBB40_604
	cmpq	%rsi, %rax
	jae	.LBB40_589
.Ltmp9487:
	.loc	1 0 0 is_stmt 0
	negq	%r9
	leaq	(%r9,%r13), %r11
	incq	%r11
	movq	%r11, 120(%rsp)
	leaq	(,%r11,8), %rdi
.Ltmp9488:
	.loc	1 1047 36 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp9489:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_562
.Ltmp9490:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp9491:
	.loc	1 1049 27
	movq	2616(%r14), %rsi
.Ltmp9492:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_563
.Ltmp9493:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp9494:
	.loc	1 1050 35
	movq	5224(%r14), %rsi
.Ltmp9495:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_564
.Ltmp9496:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp9497:
	.loc	1 1052 27
	movq	5240(%r14), %rsi
.Ltmp9498:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_565
.Ltmp9499:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp9500:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm5, %ymm5, %ymm2
	vaddps	1760(%rsp), %ymm2, %ymm2
	vbroadcastss	.LCPI40_1(%rip), %ymm5
	vandps	%ymm5, %ymm2, %ymm7
	vbroadcastss	.LCPI40_2(%rip), %ymm12
	vcmplt_oqps	%ymm12, %ymm7, %ymm7
	vandnps	%ymm2, %ymm7, %ymm2
	vmovaps	%ymm2, 1760(%rsp)
	vaddps	%ymm4, %ymm4, %ymm2
	vaddps	1664(%rsp), %ymm2, %ymm2
	vandps	%ymm5, %ymm2, %ymm4
	vcmplt_oqps	%ymm12, %ymm4, %ymm4
	vandnps	%ymm2, %ymm4, %ymm2
	vmovaps	%ymm2, 1664(%rsp)
	vmulps	%ymm3, %ymm11, %ymm2
	vmovaps	1728(%rsp), %ymm4
	vmulps	%ymm4, %ymm10, %ymm3
	vaddps	%ymm2, %ymm3, %ymm2
	vaddps	%ymm2, %ymm2, %ymm2
	vaddps	%ymm2, %ymm4, %ymm2
	vandps	%ymm5, %ymm2, %ymm3
	vcmplt_oqps	%ymm12, %ymm3, %ymm3
	vandnps	%ymm2, %ymm3, %ymm2
	vmovaps	%ymm2, 1728(%rsp)
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	1696(%rsp), %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm2
	vcmplt_oqps	%ymm12, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp9501:
	vaddps	%ymm14, %ymm14, %ymm0
	vaddps	256(%rsp), %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm2
	vcmplt_oqps	%ymm12, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vaddps	%ymm1, %ymm1, %ymm0
	vaddps	192(%rsp), %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm1
	vcmplt_oqps	%ymm12, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmulps	%ymm6, %ymm9, %ymm0
	vmovaps	320(%rsp), %ymm2
	vmulps	1952(%rsp), %ymm2, %ymm1
	vaddps	%ymm0, %ymm1, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
.Ltmp9502:
	movq	2592(%r14), %rsi
	movq	160(%rsp), %r11
	vmovd	(%rsi,%r11,4), %xmm1
	movq	%r10, 104(%rsp)
	movq	%rdx, 112(%rsp)
	movq	224(%rsp), %rdx
	vpinsrd	$1, (%rsi,%rdx,4), %xmm1, %xmm1
	movq	%r12, %r10
	movq	24(%rsp), %r12
	vpinsrd	$2, (%rsi,%r12,4), %xmm1, %xmm1
.Ltmp9503:
	vaddps	%ymm0, %ymm2, %ymm0
	movq	%r15, %r13
	movq	152(%rsp), %r15
.Ltmp9504:
	vpinsrd	$3, (%rsi,%r15,4), %xmm1, %xmm1
	vmovd	(%rsi,%rcx,4), %xmm2
	movq	136(%rsp), %r9
	vpinsrd	$1, (%rsi,%r9,4), %xmm2, %xmm2
	movq	2080(%rsp), %rbx
	vpinsrd	$2, (%rsi,%rbx,4), %xmm2, %xmm2
	movq	%rdi, 1952(%rsp)
	movq	%rax, %rdi
	movq	64(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm2, %xmm2
.Ltmp9505:
	vandps	%ymm5, %ymm0, %ymm3
	vcmplt_oqps	%ymm12, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vaddps	%ymm13, %ymm13, %ymm0
	vaddps	%ymm0, %ymm8, %ymm0
	vandps	%ymm5, %ymm0, %ymm3
	vcmplt_oqps	%ymm12, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm0
	vmovaps	%ymm0, 1920(%rsp)
.Ltmp9506:
	movq	2608(%r14), %rsi
	vmovd	(%rsi,%r11,4), %xmm0
	vpinsrd	$1, (%rsi,%rdx,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%r12,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r15,4), %xmm0, %xmm0
.Ltmp9507:
	vinserti128	$1, %xmm1, %ymm2, %ymm1
.Ltmp9508:
	vmovd	(%rsi,%rcx,4), %xmm2
	vpinsrd	$1, (%rsi,%r9,4), %xmm2, %xmm2
	vpinsrd	$2, (%rsi,%rbx,4), %xmm2, %xmm2
	vpinsrd	$3, (%rsi,%rax,4), %xmm2, %xmm2
.Ltmp9509:
	movq	5216(%r14), %rcx
	vmovd	(%rcx,%r13,4), %xmm3
	movq	104(%rsp), %r9
	vpinsrd	$1, (%rcx,%r9,4), %xmm3, %xmm3
	vpinsrd	$2, (%rcx,%r10,4), %xmm3, %xmm3
.Ltmp9510:
	vinserti128	$1, %xmm0, %ymm2, %ymm0
.Ltmp9511:
	vpinsrd	$3, (%rcx,%rdi,4), %xmm3, %xmm2
	vmovd	(%rcx,%r8,4), %xmm3
	movq	16(%rsp), %rax
	vpinsrd	$1, (%rcx,%rax,4), %xmm3, %xmm3
	movq	1824(%rsp), %rdx
	vpinsrd	$2, (%rcx,%rdx,4), %xmm3, %xmm3
	movq	112(%rsp), %rsi
	vpinsrd	$3, (%rcx,%rsi,4), %xmm3, %xmm3
.Ltmp9512:
	movq	5232(%r14), %rcx
	vmovd	(%rcx,%r13,4), %xmm4
	vpinsrd	$1, (%rcx,%r9,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%r10,4), %xmm4, %xmm4
.Ltmp9513:
	vinserti128	$1, %xmm2, %ymm3, %ymm2
.Ltmp9514:
	vpinsrd	$3, (%rcx,%rdi,4), %xmm4, %xmm3
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rax,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rsi,4), %xmm4, %xmm4
	vinserti128	$1, %xmm3, %ymm4, %ymm3
.Ltmp9515:
	vpand	%ymm5, %ymm1, %ymm1
	vpand	%ymm5, %ymm2, %ymm2
	vbroadcastss	.LCPI40_3(%rip), %ymm4
	vmulps	%ymm4, %ymm1, %ymm1
	vmulps	%ymm4, %ymm2, %ymm2
	vaddps	%ymm2, %ymm1, %ymm2
.Ltmp9516:
	vpand	%ymm5, %ymm0, %ymm0
	vpand	%ymm5, %ymm3, %ymm1
	vmulps	%ymm4, %ymm0, %ymm0
	vmulps	%ymm4, %ymm1, %ymm1
	vaddps	%ymm1, %ymm0, %ymm1
	vbroadcastss	.LCPI40_4(%rip), %ymm9
.Ltmp9517:
	vmaxps	%ymm9, %ymm2, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm3
	vbroadcastss	.LCPI40_5(%rip), %ymm9
	vmaxps	%ymm9, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm2
	vbroadcastsd	.LCPI40_7(%rip), %ymm4
	vorps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_8(%rip), %ymm11
	vaddps	%ymm2, %ymm11, %ymm2
	vbroadcastss	.LCPI40_9(%rip), %ymm13
	vmulps	%ymm2, %ymm13, %ymm6
	vbroadcastss	.LCPI40_10(%rip), %ymm14
	vaddps	%ymm6, %ymm14, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_11(%rip), %ymm15
	vaddps	%ymm6, %ymm15, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_12(%rip), %ymm7
	vaddps	%ymm7, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_13(%rip), %ymm7
	vaddps	%ymm7, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_14(%rip), %ymm7
	vaddps	%ymm7, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm2
	vpsrld	$23, %ymm0, %ymm0
	vpbroadcastd	.LCPI40_15(%rip), %ymm6
	vpor	%ymm6, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm10
	vaddps	%ymm0, %ymm10, %ymm0
	vaddps	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm13
	vmulps	%ymm0, %ymm13, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm14
	vmaxps	%ymm14, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm15
	vminps	%ymm15, %ymm0, %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vsubps	352(%rsp), %ymm0, %ymm2
	vbroadcastss	.LCPI40_20(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm7
	vmulps	%ymm7, %ymm7, %ymm7
	vbroadcastss	.LCPI40_22(%rip), %ymm12
	vmulps	%ymm7, %ymm12, %ymm7
	vcmpgt_oqps	%ymm0, %ymm2, %ymm8
	vblendvps	%ymm8, %ymm2, %ymm7, %ymm7
	vbroadcastss	.LCPI40_21(%rip), %ymm11
	vcmple_oqps	%ymm11, %ymm2, %ymm2
	vmulps	2240(%rsp), %ymm7, %ymm7
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm2, %ymm0, %ymm2
	vpandn	%ymm7, %ymm2, %ymm2
	vbroadcastss	.LCPI40_23(%rip), %ymm7
	vmaxps	%ymm7, %ymm2, %ymm2
	vminps	%ymm0, %ymm2, %ymm2
	vmovaps	1984(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm2, %ymm7
	vmovaps	2560(%rsp), %ymm8
	vblendvps	%ymm7, 2592(%rsp), %ymm8, %ymm7
	vsubps	%ymm2, %ymm0, %ymm8
	vmulps	%ymm7, %ymm8, %ymm7
	vaddps	%ymm7, %ymm2, %ymm2
	vandps	%ymm5, %ymm2, %ymm7
	vbroadcastss	.LCPI40_2(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm7, %ymm7
	vandnps	%ymm2, %ymm7, %ymm0
.Ltmp9518:
	vbroadcastss	.LCPI40_4(%rip), %ymm2
	vmaxps	%ymm2, %ymm1, %ymm1
	vmaxps	%ymm9, %ymm1, %ymm1
	vandps	%ymm3, %ymm1, %ymm2
	vorps	%ymm4, %ymm2, %ymm2
	vpsrld	$23, %ymm1, %ymm1
	vpor	%ymm6, %ymm1, %ymm1
	vbroadcastss	.LCPI40_8(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_9(%rip), %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_10(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_11(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_12(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_13(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_14(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
	vaddps	%ymm1, %ymm10, %ymm1
	vaddps	%ymm2, %ymm1, %ymm1
	vmulps	%ymm1, %ymm13, %ymm1
	vmaxps	%ymm14, %ymm1, %ymm1
	vminps	%ymm15, %ymm1, %ymm1
	vmovaps	%ymm1, 224(%rsp)
	vsubps	512(%rsp), %ymm1, %ymm1
	vbroadcastss	.LCPI40_20(%rip), %ymm3
	vaddps	%ymm3, %ymm1, %ymm2
	vmulps	%ymm2, %ymm2, %ymm2
	vmulps	%ymm2, %ymm12, %ymm2
	vcmpgt_oqps	%ymm3, %ymm1, %ymm4
	vblendvps	%ymm4, %ymm1, %ymm2, %ymm2
	vcmple_oqps	%ymm11, %ymm1, %ymm1
	vmulps	2528(%rsp), %ymm2, %ymm2
	vxorps	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%ymm1, %ymm3, %ymm1
	vpandn	%ymm2, %ymm1, %ymm1
	vmovaps	%ymm0, 1984(%rsp)
.Ltmp9519:
	vaddps	480(%rsp), %ymm0, %ymm2
	vbroadcastss	.LCPI40_24(%rip), %ymm9
	vmulps	%ymm2, %ymm9, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm11
	vmaxps	%ymm11, %ymm2, %ymm2
	vbroadcastss	.LCPI40_26(%rip), %ymm13
	vminps	%ymm13, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm14
	vmulps	%ymm2, %ymm14, %ymm6
	vbroadcastss	.LCPI40_28(%rip), %ymm15
	vaddps	%ymm6, %ymm15, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_29(%rip), %ymm12
	vaddps	%ymm6, %ymm12, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_30(%rip), %ymm11
	vaddps	%ymm6, %ymm11, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_31(%rip), %ymm10
	vaddps	%ymm6, %ymm10, %ymm6
	vbroadcastss	.LCPI40_23(%rip), %ymm14
.Ltmp9520:
	vmaxps	%ymm14, %ymm1, %ymm1
	vminps	%ymm3, %ymm1, %ymm1
	vmovaps	1856(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm1, %ymm7
	vmovaps	2464(%rsp), %ymm8
	vblendvps	%ymm7, 2496(%rsp), %ymm8, %ymm7
.Ltmp9521:
	vmulps	%ymm6, %ymm2, %ymm2
.Ltmp9522:
	vsubps	%ymm1, %ymm0, %ymm6
	vmulps	%ymm7, %ymm6, %ymm6
	vaddps	%ymm6, %ymm1, %ymm1
	vandps	%ymm5, %ymm1, %ymm6
	vbroadcastss	.LCPI40_2(%rip), %ymm13
	vcmplt_oqps	%ymm13, %ymm6, %ymm6
	vandnps	%ymm1, %ymm6, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm7
.Ltmp9523:
	vaddps	%ymm7, %ymm2, %ymm1
	vbroadcastss	.LCPI40_33(%rip), %ymm8
	vaddps	%ymm4, %ymm8, %ymm2
	vpslld	$23, %ymm2, %ymm2
	vmovaps	%ymm0, 1856(%rsp)
.Ltmp9524:
	vaddps	640(%rsp), %ymm0, %ymm4
.Ltmp9525:
	vmulps	%ymm2, %ymm1, %ymm0
	vmovaps	%ymm0, 160(%rsp)
.Ltmp9526:
	vmulps	%ymm4, %ymm9, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm3
	vmulps	%ymm3, %ymm2, %ymm6
	vaddps	%ymm6, %ymm15, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm12, %ymm6
	vmovaps	%ymm12, %ymm15
	vmulps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm11, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm10, %ymm6
	vmulps	%ymm6, %ymm2, %ymm2
	vaddps	%ymm7, %ymm2, %ymm2
	vaddps	%ymm4, %ymm8, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmovaps	64(%rsp), %ymm0
.Ltmp9527:
	vsubps	992(%rsp), %ymm0, %ymm6
.Ltmp9528:
	vmulps	%ymm4, %ymm2, %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm8
.Ltmp9529:
	vaddps	%ymm6, %ymm8, %ymm2
	vmulps	%ymm2, %ymm2, %ymm2
	vbroadcastss	.LCPI40_22(%rip), %ymm12
	vmulps	%ymm2, %ymm12, %ymm2
	vcmpgt_oqps	%ymm8, %ymm6, %ymm4
	vblendvps	%ymm4, %ymm6, %ymm2, %ymm2
	vbroadcastss	.LCPI40_21(%rip), %ymm11
	vcmple_oqps	%ymm11, %ymm6, %ymm4
	vmulps	2432(%rsp), %ymm2, %ymm2
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm4, %ymm0, %ymm4
	vpandn	%ymm2, %ymm4, %ymm2
	vmaxps	%ymm14, %ymm2, %ymm2
	vminps	%ymm0, %ymm2, %ymm2
	vxorps	%xmm10, %xmm10, %xmm10
	vmovaps	1888(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm2, %ymm4
	vmovaps	2368(%rsp), %ymm6
	vblendvps	%ymm4, 2400(%rsp), %ymm6, %ymm4
	vsubps	%ymm2, %ymm0, %ymm6
	vmulps	%ymm4, %ymm6, %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vandps	%ymm5, %ymm2, %ymm4
	vcmplt_oqps	%ymm13, %ymm4, %ymm4
	vandnps	%ymm2, %ymm4, %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vaddps	1120(%rsp), %ymm0, %ymm2
	vmulps	%ymm2, %ymm9, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm7
	vmaxps	%ymm7, %ymm2, %ymm2
	vminps	%ymm1, %ymm2, %ymm2
	vmovaps	%ymm1, %ymm13
	vroundps	$9, %ymm2, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm3, %ymm2, %ymm6
	vbroadcastss	.LCPI40_28(%rip), %ymm1
	vaddps	%ymm1, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm15, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_30(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm6
	vbroadcastss	.LCPI40_31(%rip), %ymm0
	vaddps	%ymm0, %ymm6, %ymm6
	vmulps	%ymm6, %ymm2, %ymm2
	vbroadcastss	.LCPI40_32(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_33(%rip), %ymm0
	vaddps	%ymm0, %ymm4, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmulps	%ymm4, %ymm2, %ymm4
	vmovaps	224(%rsp), %ymm0
.Ltmp9530:
	vsubps	1152(%rsp), %ymm0, %ymm2
	vaddps	%ymm2, %ymm8, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm12, %ymm3
	vcmpgt_oqps	%ymm8, %ymm2, %ymm6
	vblendvps	%ymm6, %ymm2, %ymm3, %ymm3
	vcmple_oqps	%ymm11, %ymm2, %ymm2
	vmulps	2336(%rsp), %ymm3, %ymm3
	vpcmpgtd	%ymm2, %ymm10, %ymm2
	vpandn	%ymm3, %ymm2, %ymm2
	vmaxps	%ymm14, %ymm2, %ymm2
	vminps	%ymm10, %ymm2, %ymm2
	vmovaps	2208(%rsp), %ymm6
	vcmplt_oqps	%ymm6, %ymm2, %ymm3
	vmovaps	2272(%rsp), %ymm0
	vblendvps	%ymm3, 2304(%rsp), %ymm0, %ymm3
	vsubps	%ymm2, %ymm6, %ymm6
	vmulps	%ymm3, %ymm6, %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vandps	%ymm5, %ymm2, %ymm3
	vbroadcastss	.LCPI40_2(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm3, %ymm3
	vandnps	%ymm2, %ymm3, %ymm6
	vaddps	1280(%rsp), %ymm6, %ymm2
	vmulps	%ymm2, %ymm9, %ymm2
	vmaxps	%ymm7, %ymm2, %ymm2
	vminps	%ymm13, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm3
	vsubps	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm0
	vmulps	%ymm0, %ymm2, %ymm5
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_30(%rip), %ymm0
	vaddps	%ymm0, %ymm5, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm0
	vaddps	%ymm0, %ymm5, %ymm5
	vmulps	%ymm5, %ymm2, %ymm2
	vbroadcastss	.LCPI40_32(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_33(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm3
	vpslld	$23, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
.Ltmp9531:
	movq	2592(%r14), %rcx
	movq	1952(%rsp), %rax
	vmovaps	160(%rsp), %ymm0
	vmulps	(%rcx,%rax,4), %ymm0, %ymm1
	movq	2608(%r14), %rcx
	vmovaps	64(%rsp), %ymm0
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
	vaddps	%ymm0, %ymm1, %ymm0
.Ltmp9532:
	movq	5216(%r14), %rcx
	vmulps	(%rcx,%rax,4), %ymm4, %ymm1
	.loc	1 1052 27 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp9533:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm2, %ymm2
.Ltmp9534:
	.loc	14 48 14
	vaddps	%ymm2, %ymm1, %ymm1
	movq	2144(%rsp), %r10
	movq	128(%rsp), %rdi
.Ltmp9535:
	.loc	8 551 14
	vmovups	%ymm0, (%r10,%rdi,4)
	movq	56(%rsp), %r15
.Ltmp9536:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm1, (%r15,%rdi,4)
	movq	144(%rsp), %r11
.Ltmp9537:
	.loc	1 0 0
	incq	%r11
.Ltmp9538:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	movq	312(%rsp), %r12
	cmpq	%r11, %r12
	movq	2016(%rsp), %rbx
	movq	120(%rsp), %r13
.Ltmp9539:
	.loc	3 900 12
	jne	.LBB40_400
.Ltmp9540:
.LBB40_451:
	.loc	3 0 12 is_stmt 0
	vmovaps	1760(%rsp), %ymm0
	.loc	1 1057 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r14)
	vmovaps	1664(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r14)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r14)
	vmovaps	1696(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r14)
	vmovaps	256(%rsp), %ymm0
	.loc	1 1058 5
	vmovaps	%ymm0, 4032(%r14)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r14)
	vmovaps	320(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r14)
	vmovaps	1920(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r14)
	vmovaps	1984(%rsp), %ymm0
	.loc	1 1059 5
	vmovaps	%ymm0, 2304(%r14)
	vmovaps	1856(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r14)
	vmovaps	1888(%rsp), %ymm0
	.loc	1 1060 5
	vmovaps	%ymm0, 4928(%r14)
	vmovaps	%ymm6, 4960(%r14)
	.loc	1 1061 5
	movq	%r13, 5272(%r14)
	xorl	%eax, %eax
	xorl	%edi, %edi
.Ltmp9541:
	.loc	1 0 5 is_stmt 0
.Ltmp9542:
	.p2align	4
.LBB40_452:
	.loc	1 1194 13 is_stmt 1
	vmovss	352(%rsp,%rax,2), %xmm3
	vmovss	356(%rsp,%rax,2), %xmm4
	vmovss	360(%rsp,%rax,2), %xmm5
	vmovss	364(%rsp,%rax,2), %xmm6
	vmovss	368(%rsp,%rax,2), %xmm7
	vmovss	372(%rsp,%rax,2), %xmm2
	vmovss	376(%rsp,%rax,2), %xmm1
	vmovd	380(%rsp,%rax,2), %xmm0
.Ltmp9543:
	.loc	1 1197 17
	vmovss	%xmm3, (%r14,%rax)
	.loc	1 1198 34
	movl	12(%r14,%rax), %ecx
	movl	172(%r14,%rax), %edx
.Ltmp9544:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9545:
	.loc	1 1198 17
	movl	%ecx, 12(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm4, 160(%r14,%rax)
.Ltmp9546:
	.loc	38 2472 13
	subl	%r12d, %edx
	cmovbl	%edi, %edx
.Ltmp9547:
	.loc	1 1198 17
	movl	%edx, 172(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm5, 320(%r14,%rax)
	.loc	1 1198 34
	movl	332(%r14,%rax), %ecx
.Ltmp9548:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9549:
	.loc	1 1198 17
	movl	%ecx, 332(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm6, 480(%r14,%rax)
	.loc	1 1198 34
	movl	492(%r14,%rax), %ecx
.Ltmp9550:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9551:
	.loc	1 1198 17
	movl	%ecx, 492(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm7, 640(%r14,%rax)
	.loc	1 1198 34
	movl	652(%r14,%rax), %ecx
.Ltmp9552:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9553:
	.loc	1 1198 17
	movl	%ecx, 652(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 800(%r14,%rax)
	.loc	1 1198 34
	movl	812(%r14,%rax), %ecx
.Ltmp9554:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9555:
	.loc	1 1198 17
	movl	%ecx, 812(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 960(%r14,%rax)
	.loc	1 1198 34
	movl	972(%r14,%rax), %ecx
.Ltmp9556:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9557:
	.loc	1 1198 17
	movl	%ecx, 972(%r14,%rax)
	.loc	1 1197 17
	vmovd	%xmm0, 1120(%r14,%rax)
	.loc	1 1198 34
	movl	1132(%r14,%rax), %ecx
.Ltmp9558:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9559:
	.loc	1 1198 17
	movl	%ecx, 1132(%r14,%rax)
.Ltmp9560:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp9561:
	.loc	3 900 12
	jne	.LBB40_452
.Ltmp9562:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	1656(%rsp), %rsi
	.p2align	4
.LBB40_454:
.Ltmp9563:
	.loc	1 1194 13 is_stmt 1
	vmovss	992(%rsp,%rax,2), %xmm3
	vmovss	996(%rsp,%rax,2), %xmm4
	vmovss	1000(%rsp,%rax,2), %xmm5
	vmovss	1004(%rsp,%rax,2), %xmm6
	vmovss	1008(%rsp,%rax,2), %xmm7
	vmovss	1012(%rsp,%rax,2), %xmm2
	vmovss	1016(%rsp,%rax,2), %xmm1
	vmovd	1020(%rsp,%rax,2), %xmm0
.Ltmp9564:
	.loc	1 1197 17
	vmovss	%xmm3, 2624(%r14,%rax)
	.loc	1 1198 34
	movl	2636(%r14,%rax), %ecx
	movl	2796(%r14,%rax), %edx
.Ltmp9565:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9566:
	.loc	1 1198 17
	movl	%ecx, 2636(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm4, 2784(%r14,%rax)
.Ltmp9567:
	.loc	38 2472 13
	subl	%r12d, %edx
	cmovbl	%edi, %edx
.Ltmp9568:
	.loc	1 1198 17
	movl	%edx, 2796(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm5, 2944(%r14,%rax)
	.loc	1 1198 34
	movl	2956(%r14,%rax), %ecx
.Ltmp9569:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9570:
	.loc	1 1198 17
	movl	%ecx, 2956(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm6, 3104(%r14,%rax)
	.loc	1 1198 34
	movl	3116(%r14,%rax), %ecx
.Ltmp9571:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9572:
	.loc	1 1198 17
	movl	%ecx, 3116(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm7, 3264(%r14,%rax)
	.loc	1 1198 34
	movl	3276(%r14,%rax), %ecx
.Ltmp9573:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9574:
	.loc	1 1198 17
	movl	%ecx, 3276(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm2, 3424(%r14,%rax)
	.loc	1 1198 34
	movl	3436(%r14,%rax), %ecx
.Ltmp9575:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9576:
	.loc	1 1198 17
	movl	%ecx, 3436(%r14,%rax)
	.loc	1 1197 17
	vmovss	%xmm1, 3584(%r14,%rax)
	.loc	1 1198 34
	movl	3596(%r14,%rax), %ecx
.Ltmp9577:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9578:
	.loc	1 1198 17
	movl	%ecx, 3596(%r14,%rax)
	.loc	1 1197 17
	vmovd	%xmm0, 3744(%r14,%rax)
	.loc	1 1198 34
	movl	3756(%r14,%rax), %ecx
.Ltmp9579:
	.loc	38 2472 13
	subl	%r12d, %ecx
	cmovbl	%edi, %ecx
.Ltmp9580:
	.loc	1 1198 17
	movl	%ecx, 3756(%r14,%rax)
.Ltmp9581:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp9582:
	.loc	3 900 12
	jne	.LBB40_454
	jmp	.LBB40_393
.Ltmp9583:
.LBB40_455:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp9584:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_568
.Ltmp9585:
	.loc	48 451 16 is_stmt 1
	cmpq	40(%rsp), %rsi
	ja	.LBB40_568
.Ltmp9586:
	.loc	48 451 16 is_stmt 0
	cmpq	32(%rsp), %rsi
	ja	.LBB40_572
.Ltmp9587:
	.loc	1 972 27 is_stmt 1
	vmovaps	1408(%r14), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1440(%r14), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1472(%r14), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1504(%r14), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp9588:
	.loc	1 973 26
	vmovaps	4032(%r14), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	4064(%r14), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r14), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	4128(%r14), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
.Ltmp9589:
	.loc	1 974 25
	vmovaps	2304(%r14), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	2336(%r14), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
.Ltmp9590:
	.loc	1 975 24
	vmovaps	4928(%r14), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	4960(%r14), %ymm8
.Ltmp9591:
	.loc	1 976 24
	movq	5272(%r14), %r10
.Ltmp9592:
	.loc	2 1916 50
	testq	%r12, %r12
	movq	2016(%rsp), %rbx
.Ltmp9593:
	.loc	3 900 12
	je	.LBB40_392
.Ltmp9594:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r12,8), %rax
	movq	%rax, 2048(%rsp)
	movq	296(%rsp), %rax
	leaq	(%rax,%r15,4), %r11
	movq	304(%rsp), %rax
	leaq	(%rax,%r15,4), %r15
.Ltmp9595:
	.loc	48 568 12 is_stmt 1
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r12
	movq	%r12, 2176(%rsp)
	xorl	%edi, %edi
	xorl	%r12d, %r12d
	movq	%r11, 2144(%rsp)
	movq	%r15, 56(%rsp)
.Ltmp9596:
	.loc	48 0 12 is_stmt 0
.Ltmp9597:
	.p2align	4
.LBB40_460:
	.loc	1 987 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp9598:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %r9d
	cmovaeq	%rbx, %r9
.Ltmp9599:
	.loc	48 568 12
	cmpq	2048(%rsp), %rdi
	ja	.LBB40_555
.Ltmp9600:
	.loc	48 438 16
	cmpq	%r12, 2176(%rsp)
	je	.LBB40_547
.Ltmp9601:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,8), %rax
.Ltmp9602:
	.loc	1 1000 29 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp9603:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_554
.Ltmp9604:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp9605:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm8, 2208(%rsp)
	vmovups	(%r11,%rdi,4), %ymm3
.Ltmp9606:
	vmovups	(%r15,%rdi,4), %ymm5
.Ltmp9607:
	vmovaps	1280(%r14), %ymm10
	vmovaps	1312(%r14), %ymm11
	vmovaps	1344(%r14), %ymm2
	vmovaps	3904(%r14), %ymm15
	vmovaps	3936(%r14), %ymm9
	vmovaps	3968(%r14), %ymm1
	vmovaps	1664(%rsp), %ymm7
	vsubps	%ymm7, %ymm3, %ymm0
	vmulps	%ymm0, %ymm11, %ymm4
	vmovaps	1760(%rsp), %ymm8
	vmulps	%ymm10, %ymm8, %ymm6
	vaddps	%ymm4, %ymm6, %ymm13
	vaddps	%ymm13, %ymm8, %ymm4
	vmulps	%ymm11, %ymm8, %ymm6
	vmulps	%ymm2, %ymm0, %ymm0
	vaddps	%ymm0, %ymm6, %ymm12
	vaddps	%ymm7, %ymm12, %ymm6
	vmulps	1376(%r14), %ymm4, %ymm0
	vmovaps	1696(%rsp), %ymm8
	vsubps	%ymm8, %ymm6, %ymm7
	vmulps	1728(%rsp), %ymm11, %ymm4
	vmulps	%ymm7, %ymm2, %ymm2
	vaddps	%ymm2, %ymm4, %ymm4
	vaddps	%ymm4, %ymm8, %ymm6
.Ltmp9608:
	vsubps	192(%rsp), %ymm5, %ymm14
	vmulps	%ymm9, %ymm14, %ymm2
	vmovaps	256(%rsp), %ymm8
	vmovaps	%ymm15, 1952(%rsp)
	vmulps	%ymm15, %ymm8, %ymm15
	vaddps	%ymm2, %ymm15, %ymm2
	vaddps	%ymm2, %ymm8, %ymm15
	vmulps	4000(%r14), %ymm15, %ymm15
.Ltmp9609:
	.loc	1 1000 29 is_stmt 1
	movq	2592(%r14), %rcx
.Ltmp9610:
	.loc	8 551 14
	vmovups	%ymm6, (%rcx,%rax,4)
.Ltmp9611:
	.loc	1 1001 30
	movq	2616(%r14), %rsi
.Ltmp9612:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_559
.Ltmp9613:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp9614:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm0, %ymm3, %ymm0
	vsubps	%ymm6, %ymm0, %ymm0
.Ltmp9615:
	.loc	1 1001 30 is_stmt 1
	movq	2608(%r14), %rcx
.Ltmp9616:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp9617:
	.loc	1 1002 28
	movq	5224(%r14), %rsi
.Ltmp9618:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_560
.Ltmp9619:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp9620:
	.loc	1 0 0 is_stmt 0
	vmulps	256(%rsp), %ymm9, %ymm0
	vmulps	%ymm1, %ymm14, %ymm3
	vaddps	%ymm3, %ymm0, %ymm3
	vaddps	192(%rsp), %ymm3, %ymm0
	vmovaps	1824(%rsp), %ymm6
	vsubps	%ymm6, %ymm0, %ymm14
	vmovaps	1920(%rsp), %ymm8
	vmulps	%ymm9, %ymm8, %ymm0
	vmulps	%ymm1, %ymm14, %ymm1
	vaddps	%ymm1, %ymm0, %ymm1
	vaddps	%ymm1, %ymm6, %ymm0
.Ltmp9621:
	.loc	1 1002 28 is_stmt 1
	movq	5216(%r14), %rcx
.Ltmp9622:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp9623:
	.loc	1 1003 29
	movq	5240(%r14), %rsi
.Ltmp9624:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_561
.Ltmp9625:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_558
.Ltmp9626:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm5, %ymm15, %ymm5
	vsubps	%ymm0, %ymm5, %ymm0
.Ltmp9627:
	.loc	1 1003 29 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp9628:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
	movq	2368(%r14), %rcx
.Ltmp9629:
	.loc	1 877 35
	addq	%r10, %rcx
.Ltmp9630:
	.loc	1 857 8
	cmpq	%rbx, %rcx
	movl	$0, %eax
	cmovaeq	%rbx, %rax
.Ltmp9631:
	.loc	1 1006 34
	movq	2600(%r14), %rsi
.Ltmp9632:
	.loc	1 857 8
	subq	%rax, %rcx
.Ltmp9633:
	.loc	1 877 30
	shlq	$3, %rcx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	.loc	1 0 25
	movq	2376(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9634:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9635:
	.loc	1 877 30
	leaq	1(,%rax,8), %r13
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_578
	.loc	1 0 25
	movq	2384(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9636:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9637:
	.loc	1 877 30
	leaq	2(,%rax,8), %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_588
	.loc	1 0 25
	movq	2392(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9638:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9639:
	.loc	1 877 30
	leaq	3(,%rax,8), %rax
	movq	%rax, 320(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_583
	.loc	1 0 25
	movq	2400(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9640:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9641:
	.loc	1 877 30
	leaq	4(,%rax,8), %rax
	movq	%rax, 64(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_587
	.loc	1 0 25
	movq	2408(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9642:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9643:
	.loc	1 877 30
	leaq	5(,%rax,8), %rax
	movq	%rax, 160(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_595
	.loc	1 0 25
	movq	2416(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9644:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9645:
	.loc	1 877 30
	leaq	6(,%rax,8), %rax
	movq	%rax, 224(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_597
	.loc	1 0 25
	movq	2424(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9646:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9647:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp9648:
	.loc	1 1008 34 is_stmt 1
	movq	2616(%r14), %rsi
.Ltmp9649:
	.loc	1 877 25
	cmpq	%rsi, %rcx
	jae	.LBB40_579
	cmpq	%rsi, %r13
	jae	.LBB40_578
	cmpq	%rsi, %r8
	jae	.LBB40_616
	cmpq	%rsi, 320(%rsp)
	jae	.LBB40_627
	cmpq	%rsi, 64(%rsp)
	jae	.LBB40_587
	cmpq	%rsi, 160(%rsp)
	jae	.LBB40_595
	cmpq	%rsi, 224(%rsp)
	jae	.LBB40_597
	.loc	1 0 25 is_stmt 0
	movq	%r8, 2080(%rsp)
	.loc	1 877 25
	cmpq	%rsi, %rax
	jae	.LBB40_599
.Ltmp9650:
	.loc	1 0 25
	movq	%rax, 152(%rsp)
	movq	4992(%r14), %r8
.Ltmp9651:
	.loc	1 877 35
	addq	%r10, %r8
.Ltmp9652:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r8
	movl	$0, %eax
	cmovaeq	%rbx, %rax
.Ltmp9653:
	.loc	1 1010 34
	movq	5224(%r14), %rsi
.Ltmp9654:
	.loc	1 857 8
	subq	%rax, %r8
.Ltmp9655:
	.loc	1 877 30
	shlq	$3, %r8
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_575
	.loc	1 0 25
	movq	%r13, 144(%rsp)
	movq	5000(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9656:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9657:
	.loc	1 877 30
	leaq	1(,%rax,8), %rax
	movq	%rax, 24(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_601
	.loc	1 0 25
	movq	5008(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9658:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9659:
	.loc	1 877 30
	leaq	2(,%rax,8), %rax
	movq	%rax, 16(%rsp)
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_574
	.loc	1 0 25
	movq	%r12, 136(%rsp)
	movq	5016(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9660:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rax
.Ltmp9661:
	.loc	1 877 30
	leaq	3(,%rax,8), %rdx
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_585
	.loc	1 0 25
	movq	%rdi, 128(%rsp)
	movq	5024(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9662:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp9663:
	.loc	1 877 30
	leaq	4(,%rax,8), %r15
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_607
	.loc	1 0 25
	movq	5032(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9664:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp9665:
	.loc	1 877 30
	leaq	5(,%rax,8), %r13
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_592
	.loc	1 0 25
	movq	5040(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9666:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp9667:
	.loc	1 877 30
	leaq	6(,%rax,8), %r12
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_586
	.loc	1 0 25
	movq	5048(%r14), %rax
	.loc	1 877 35
	addq	%r10, %rax
.Ltmp9668:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rax
.Ltmp9669:
	.loc	1 877 30
	leaq	7(,%rax,8), %rax
	.loc	1 877 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_618
.Ltmp9670:
	.loc	1 1012 34 is_stmt 1
	movq	5240(%r14), %rsi
.Ltmp9671:
	.loc	1 877 25
	cmpq	%rsi, %r8
	jae	.LBB40_575
	cmpq	%rsi, 24(%rsp)
	jae	.LBB40_593
	cmpq	%rsi, 16(%rsp)
	jae	.LBB40_574
	cmpq	%rsi, %rdx
	jae	.LBB40_606
	cmpq	%rsi, %r15
	jae	.LBB40_607
	cmpq	%rsi, %r13
	jae	.LBB40_624
	cmpq	%rsi, %r12
	jae	.LBB40_586
	cmpq	%rsi, %rax
	jae	.LBB40_589
.Ltmp9672:
	.loc	1 0 0 is_stmt 0
	negq	%r9
	addq	%r9, %r10
	incq	%r10
	leaq	(,%r10,8), %rdi
.Ltmp9673:
	.loc	1 1047 36 is_stmt 1
	movq	2600(%r14), %rsi
.Ltmp9674:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_562
.Ltmp9675:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp9676:
	.loc	1 1049 27
	movq	2616(%r14), %rsi
.Ltmp9677:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_563
.Ltmp9678:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp9679:
	.loc	1 1050 35
	movq	5224(%r14), %rsi
.Ltmp9680:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_564
.Ltmp9681:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp9682:
	.loc	1 1052 27
	movq	5240(%r14), %rsi
.Ltmp9683:
	.loc	48 568 12
	movq	%rsi, %r9
	subq	%rdi, %r9
	jb	.LBB40_565
.Ltmp9684:
	.loc	48 438 16
	cmpq	$7, %r9
	jbe	.LBB40_556
.Ltmp9685:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm13, %ymm13, %ymm0
	vaddps	1760(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_1(%rip), %ymm13
	vandps	%ymm0, %ymm13, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm15
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vaddps	%ymm12, %ymm12, %ymm0
	vaddps	1664(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm5
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmulps	%ymm7, %ymm11, %ymm0
	vmovaps	1728(%rsp), %ymm7
	vmulps	%ymm7, %ymm10, %ymm5
	vaddps	%ymm0, %ymm5, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	%ymm0, %ymm7, %ymm0
	vandps	%ymm0, %ymm13, %ymm5
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vaddps	%ymm4, %ymm4, %ymm0
	vaddps	1696(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm4
	vcmplt_oqps	%ymm15, %ymm4, %ymm4
	vandnps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm0, 1696(%rsp)
.Ltmp9686:
	vaddps	%ymm2, %ymm2, %ymm0
	vaddps	256(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm2
	vcmplt_oqps	%ymm15, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vaddps	%ymm3, %ymm3, %ymm0
	vaddps	192(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm2
	vcmplt_oqps	%ymm15, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmulps	%ymm14, %ymm9, %ymm0
	vmulps	1952(%rsp), %ymm8, %ymm2
	vaddps	%ymm0, %ymm2, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
.Ltmp9687:
	movq	2592(%r14), %rsi
	movq	%rdx, 120(%rsp)
	movq	%r10, %rdx
	movq	64(%rsp), %r10
	vmovd	(%rsi,%r10,4), %xmm2
	movq	%r12, 112(%rsp)
	movq	160(%rsp), %r12
	vpinsrd	$1, (%rsi,%r12,4), %xmm2, %xmm2
	movq	%r15, 104(%rsp)
	movq	224(%rsp), %r15
	vpinsrd	$2, (%rsi,%r15,4), %xmm2, %xmm2
.Ltmp9688:
	vaddps	%ymm0, %ymm8, %ymm0
	movq	152(%rsp), %r11
.Ltmp9689:
	vpinsrd	$3, (%rsi,%r11,4), %xmm2, %xmm2
	vmovd	(%rsi,%rcx,4), %xmm3
	movq	144(%rsp), %r9
	vpinsrd	$1, (%rsi,%r9,4), %xmm3, %xmm3
	movq	2080(%rsp), %rbx
	vpinsrd	$2, (%rsi,%rbx,4), %xmm3, %xmm3
	movq	%rdi, 1952(%rsp)
	movq	%rax, %rdi
	movq	320(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm3, %xmm3
.Ltmp9690:
	vandps	%ymm0, %ymm13, %ymm4
	vcmplt_oqps	%ymm15, %ymm4, %ymm4
	vandnps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vaddps	%ymm1, %ymm1, %ymm0
	vaddps	%ymm0, %ymm6, %ymm0
	vandps	%ymm0, %ymm13, %ymm1
	vcmplt_oqps	%ymm15, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 1824(%rsp)
.Ltmp9691:
	movq	2608(%r14), %rsi
	vmovd	(%rsi,%r10,4), %xmm0
	movq	%rdx, %r10
	vpinsrd	$1, (%rsi,%r12,4), %xmm0, %xmm0
	vpinsrd	$2, (%rsi,%r15,4), %xmm0, %xmm0
	vpinsrd	$3, (%rsi,%r11,4), %xmm0, %xmm0
.Ltmp9692:
	vinserti128	$1, %xmm2, %ymm3, %ymm1
.Ltmp9693:
	vmovd	(%rsi,%rcx,4), %xmm2
	vpinsrd	$1, (%rsi,%r9,4), %xmm2, %xmm2
	vpinsrd	$2, (%rsi,%rbx,4), %xmm2, %xmm2
	vpinsrd	$3, (%rsi,%rax,4), %xmm2, %xmm2
.Ltmp9694:
	movq	5216(%r14), %rcx
	movq	104(%rsp), %r11
	vmovd	(%rcx,%r11,4), %xmm3
	vpinsrd	$1, (%rcx,%r13,4), %xmm3, %xmm3
	movq	112(%rsp), %r9
	vpinsrd	$2, (%rcx,%r9,4), %xmm3, %xmm3
.Ltmp9695:
	vinserti128	$1, %xmm0, %ymm2, %ymm0
.Ltmp9696:
	vpinsrd	$3, (%rcx,%rdi,4), %xmm3, %xmm2
	vmovd	(%rcx,%r8,4), %xmm3
	movq	24(%rsp), %rax
	vpinsrd	$1, (%rcx,%rax,4), %xmm3, %xmm3
	movq	16(%rsp), %rdx
	vpinsrd	$2, (%rcx,%rdx,4), %xmm3, %xmm3
	movq	120(%rsp), %rsi
	vpinsrd	$3, (%rcx,%rsi,4), %xmm3, %xmm3
.Ltmp9697:
	movq	5232(%r14), %rcx
	vmovd	(%rcx,%r11,4), %xmm4
	vpinsrd	$1, (%rcx,%r13,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%r9,4), %xmm4, %xmm4
.Ltmp9698:
	vinserti128	$1, %xmm2, %ymm3, %ymm2
.Ltmp9699:
	vpinsrd	$3, (%rcx,%rdi,4), %xmm4, %xmm3
	vmovd	(%rcx,%r8,4), %xmm4
	vpinsrd	$1, (%rcx,%rax,4), %xmm4, %xmm4
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
	vpinsrd	$3, (%rcx,%rsi,4), %xmm4, %xmm4
	vinserti128	$1, %xmm3, %ymm4, %ymm3
.Ltmp9700:
	vpand	%ymm1, %ymm13, %ymm1
	vpand	%ymm2, %ymm13, %ymm2
	vbroadcastss	.LCPI40_3(%rip), %ymm4
	vmulps	%ymm4, %ymm1, %ymm1
	vmulps	%ymm4, %ymm2, %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp9701:
	vpand	%ymm0, %ymm13, %ymm0
	vpand	%ymm3, %ymm13, %ymm2
	vmulps	%ymm4, %ymm0, %ymm0
	vmulps	%ymm4, %ymm2, %ymm2
	vaddps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_4(%rip), %ymm8
.Ltmp9702:
	vmaxps	%ymm8, %ymm1, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm3
	vbroadcastss	.LCPI40_5(%rip), %ymm9
	vmaxps	%ymm9, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm1
	vbroadcastsd	.LCPI40_7(%rip), %ymm4
	vorps	%ymm4, %ymm1, %ymm1
	vbroadcastss	.LCPI40_8(%rip), %ymm10
	vaddps	%ymm1, %ymm10, %ymm1
	vbroadcastss	.LCPI40_9(%rip), %ymm11
	vmulps	%ymm1, %ymm11, %ymm5
	vbroadcastss	.LCPI40_10(%rip), %ymm12
	vaddps	%ymm5, %ymm12, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_11(%rip), %ymm14
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_12(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_13(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_14(%rip), %ymm6
	vaddps	%ymm6, %ymm5, %ymm5
	vmulps	%ymm5, %ymm1, %ymm1
	vpsrld	$23, %ymm0, %ymm0
	vpbroadcastd	.LCPI40_15(%rip), %ymm5
	vpor	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm9
	vaddps	%ymm0, %ymm9, %ymm0
	vaddps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm8
	vmulps	%ymm0, %ymm8, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm12
	vmaxps	%ymm12, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm14
	vminps	%ymm14, %ymm0, %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vsubps	352(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_20(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm6
	vmulps	%ymm6, %ymm6, %ymm6
	vbroadcastss	.LCPI40_22(%rip), %ymm11
	vmulps	%ymm6, %ymm11, %ymm6
	vcmpgt_oqps	%ymm1, %ymm0, %ymm7
	vblendvps	%ymm7, %ymm0, %ymm6, %ymm6
	vbroadcastss	.LCPI40_21(%rip), %ymm10
	vcmple_oqps	%ymm10, %ymm0, %ymm0
	vmulps	2240(%rsp), %ymm6, %ymm6
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm0, %ymm1, %ymm0
	vpandn	%ymm6, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm6
	vmaxps	%ymm6, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	1984(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm6
	vmovaps	2560(%rsp), %ymm7
	vblendvps	%ymm6, 2592(%rsp), %ymm7, %ymm6
	vsubps	%ymm0, %ymm1, %ymm7
	vmulps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm6
	vcmplt_oqps	%ymm15, %ymm6, %ymm6
	vandnps	%ymm0, %ymm6, %ymm6
.Ltmp9703:
	vbroadcastss	.LCPI40_4(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm0
	vbroadcastss	.LCPI40_5(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm2
	vorps	%ymm4, %ymm2, %ymm2
	vpsrld	$23, %ymm0, %ymm0
	vpor	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_8(%rip), %ymm1
	vaddps	%ymm1, %ymm2, %ymm2
	vbroadcastss	.LCPI40_9(%rip), %ymm1
	vmulps	%ymm1, %ymm2, %ymm3
	vbroadcastss	.LCPI40_10(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_11(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_12(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_13(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm3
	vbroadcastss	.LCPI40_14(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
	vaddps	%ymm0, %ymm9, %ymm0
	vaddps	%ymm2, %ymm0, %ymm0
	vmulps	%ymm0, %ymm8, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm14, %ymm0, %ymm0
	vmovaps	%ymm0, 160(%rsp)
	vsubps	512(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_20(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm2
	vmulps	%ymm2, %ymm2, %ymm2
	vmulps	%ymm2, %ymm11, %ymm2
	vcmpgt_oqps	%ymm1, %ymm0, %ymm4
	vblendvps	%ymm4, %ymm0, %ymm2, %ymm2
	vcmple_oqps	%ymm10, %ymm0, %ymm0
	vmulps	2528(%rsp), %ymm2, %ymm2
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm0, %ymm1, %ymm0
	vpandn	%ymm2, %ymm0, %ymm0
	vmovaps	%ymm6, 1984(%rsp)
.Ltmp9704:
	vaddps	480(%rsp), %ymm6, %ymm2
	vbroadcastss	.LCPI40_24(%rip), %ymm7
	vmulps	%ymm7, %ymm2, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm8
	vmaxps	%ymm8, %ymm2, %ymm2
	vbroadcastss	.LCPI40_26(%rip), %ymm3
	vminps	%ymm3, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm4
	vsubps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm12
	vmulps	%ymm2, %ymm12, %ymm5
	vbroadcastss	.LCPI40_28(%rip), %ymm14
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_29(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_30(%rip), %ymm10
	vaddps	%ymm5, %ymm10, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm9
	vaddps	%ymm5, %ymm9, %ymm5
	vbroadcastss	.LCPI40_23(%rip), %ymm12
.Ltmp9705:
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	1856(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm6
	vmovaps	2464(%rsp), %ymm7
	vblendvps	%ymm6, 2496(%rsp), %ymm7, %ymm6
.Ltmp9706:
	vmulps	%ymm5, %ymm2, %ymm2
.Ltmp9707:
	vsubps	%ymm0, %ymm1, %ymm5
	vmulps	%ymm6, %ymm5, %ymm5
	vaddps	%ymm5, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm11
	vcmplt_oqps	%ymm11, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm1
	vbroadcastss	.LCPI40_32(%rip), %ymm6
.Ltmp9708:
	vaddps	%ymm6, %ymm2, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm7
	vaddps	%ymm7, %ymm4, %ymm2
	vpslld	$23, %ymm2, %ymm2
	vmovaps	%ymm1, 1856(%rsp)
.Ltmp9709:
	vaddps	640(%rsp), %ymm1, %ymm4
.Ltmp9710:
	vmulps	%ymm2, %ymm0, %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vbroadcastss	.LCPI40_24(%rip), %ymm2
.Ltmp9711:
	vmulps	%ymm2, %ymm4, %ymm0
	vmaxps	%ymm8, %ymm0, %ymm0
	vmovaps	%ymm8, %ymm14
	vminps	%ymm3, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm4
	vsubps	%ymm4, %ymm0, %ymm0
	vbroadcastss	.LCPI40_27(%rip), %ymm3
	vmulps	%ymm3, %ymm0, %ymm5
	vbroadcastss	.LCPI40_28(%rip), %ymm8
	vaddps	%ymm5, %ymm8, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm10, %ymm5
	vmovaps	%ymm10, %ymm15
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm9, %ymm5
	vmulps	%ymm5, %ymm0, %ymm0
	vaddps	%ymm6, %ymm0, %ymm0
	vaddps	%ymm7, %ymm4, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmovaps	320(%rsp), %ymm1
.Ltmp9712:
	vsubps	992(%rsp), %ymm1, %ymm5
.Ltmp9713:
	vmulps	%ymm4, %ymm0, %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm6
.Ltmp9714:
	vaddps	%ymm6, %ymm5, %ymm0
	vmulps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI40_22(%rip), %ymm9
	vmulps	%ymm0, %ymm9, %ymm0
	vcmpgt_oqps	%ymm6, %ymm5, %ymm4
	vblendvps	%ymm4, %ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_21(%rip), %ymm10
	vcmple_oqps	%ymm10, %ymm5, %ymm4
	vmulps	2432(%rsp), %ymm0, %ymm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm4, %ymm1, %ymm4
	vpandn	%ymm0, %ymm4, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vxorps	%xmm7, %xmm7, %xmm7
	vmovaps	1888(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm4
	vmovaps	2368(%rsp), %ymm5
	vblendvps	%ymm4, 2400(%rsp), %ymm5, %ymm4
	vsubps	%ymm0, %ymm1, %ymm5
	vmulps	%ymm4, %ymm5, %ymm4
	vaddps	%ymm4, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm4
	vcmplt_oqps	%ymm11, %ymm4, %ymm4
	vandnps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vaddps	1120(%rsp), %ymm0, %ymm0
	vmulps	%ymm2, %ymm0, %ymm0
	vmaxps	%ymm14, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm4
	vsubps	%ymm4, %ymm0, %ymm0
	vmulps	%ymm3, %ymm0, %ymm5
	vaddps	%ymm5, %ymm8, %ymm5
	vmovaps	%ymm8, %ymm11
	vmulps	%ymm5, %ymm0, %ymm5
	vbroadcastss	.LCPI40_29(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm1
	vaddps	%ymm1, %ymm4, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmulps	%ymm4, %ymm0, %ymm4
	vmovaps	160(%rsp), %ymm0
.Ltmp9715:
	vsubps	1152(%rsp), %ymm0, %ymm0
	vaddps	%ymm6, %ymm0, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm9, %ymm3
	vcmpgt_oqps	%ymm6, %ymm0, %ymm5
	vblendvps	%ymm5, %ymm0, %ymm3, %ymm3
	vcmple_oqps	%ymm10, %ymm0, %ymm0
	vmulps	2336(%rsp), %ymm3, %ymm3
	vpcmpgtd	%ymm0, %ymm7, %ymm0
	vpandn	%ymm3, %ymm0, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm7, %ymm0, %ymm0
	vmovaps	2208(%rsp), %ymm5
	vcmplt_oqps	%ymm5, %ymm0, %ymm3
	vmovaps	2272(%rsp), %ymm1
	vblendvps	%ymm3, 2304(%rsp), %ymm1, %ymm3
	vsubps	%ymm0, %ymm5, %ymm5
	vmulps	%ymm3, %ymm5, %ymm3
	vaddps	%ymm3, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm3
	vbroadcastss	.LCPI40_2(%rip), %ymm1
	vcmplt_oqps	%ymm1, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm8
	vaddps	1280(%rsp), %ymm8, %ymm0
	vmulps	%ymm2, %ymm0, %ymm0
	vmaxps	%ymm14, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm3
	vsubps	%ymm3, %ymm0, %ymm0
	vbroadcastss	.LCPI40_27(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vbroadcastss	.LCPI40_29(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm0, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm1
	vaddps	%ymm1, %ymm3, %ymm3
	vpslld	$23, %ymm3, %ymm3
	vmulps	%ymm3, %ymm0, %ymm0
.Ltmp9716:
	movq	2592(%r14), %rcx
	movq	1952(%rsp), %rax
	vmovaps	64(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm2
	movq	2608(%r14), %rcx
	vmovaps	320(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm1
	vaddps	%ymm1, %ymm2, %ymm1
.Ltmp9717:
	movq	5216(%r14), %rcx
	vmulps	(%rcx,%rax,4), %ymm4, %ymm2
	.loc	1 1052 27 is_stmt 1
	movq	5232(%r14), %rcx
.Ltmp9718:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
.Ltmp9719:
	.loc	14 48 14
	vaddps	%ymm0, %ymm2, %ymm0
	movq	2144(%rsp), %r11
	movq	128(%rsp), %rdi
.Ltmp9720:
	.loc	8 551 14
	vmovups	%ymm1, (%r11,%rdi,4)
	movq	56(%rsp), %r15
.Ltmp9721:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm0, (%r15,%rdi,4)
	movq	136(%rsp), %r12
.Ltmp9722:
	.loc	1 0 0
	incq	%r12
.Ltmp9723:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r12, 312(%rsp)
	movq	2016(%rsp), %rbx
.Ltmp9724:
	.loc	3 900 12
	jne	.LBB40_460
	jmp	.LBB40_392
.Ltmp9725:
.LBB40_511:
	.loc	14 871 14
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
	movabsq	$2305843009213693951, %r11
.Ltmp9726:
	.loc	37 1851 23
	addq	$-7, %r11
	movq	40(%rsp), %rax
	vmovaps	%ymm0, %ymm1
.Ltmp9727:
	.loc	10 2155 12
	andq	%r11, %rax
	movq	296(%rsp), %r8
	je	.LBB40_514
.Ltmp9728:
	.loc	10 0 12 is_stmt 0
	vbroadcastss	.LCPI40_1(%rip), %ymm2
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI40_34(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB40_513:
.Ltmp9729:
	.loc	14 82 19 is_stmt 1
	vandps	(%r8,%rcx,4), %ymm2, %ymm4
.Ltmp9730:
	.loc	14 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp9731:
	.loc	14 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp9732:
	.loc	10 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB40_513
.Ltmp9733:
.LBB40_514:
	.loc	16 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
	movq	32(%rsp), %rsi
	movq	304(%rsp), %rdi
.Ltmp9734:
	.loc	50 208 8
	jae	.LBB40_519
.Ltmp9735:
	.loc	10 2155 12
	movq	%rsi, %rcx
	vmovaps	%ymm0, %ymm1
	andq	%r11, %rcx
	je	.LBB40_518
.Ltmp9736:
	.loc	10 0 12 is_stmt 0
	vbroadcastss	.LCPI40_1(%rip), %ymm2
	xorl	%edx, %edx
	vbroadcastss	.LCPI40_34(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB40_517:
.Ltmp9737:
	.loc	14 82 19 is_stmt 1
	vandps	(%rdi,%rdx,4), %ymm2, %ymm4
.Ltmp9738:
	.loc	14 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp9739:
	.loc	14 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp9740:
	.loc	10 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB40_517
.Ltmp9741:
.LBB40_518:
	.loc	16 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp9742:
	.loc	50 208 34
	jb	.LBB40_538
.LBB40_519:
	.loc	50 0 34 is_stmt 0
	vmovaps	%ymm0, %ymm1
.Ltmp9743:
	.loc	10 2155 12 is_stmt 1
	testq	%rax, %rax
.Ltmp9744:
	.loc	10 2155 12 is_stmt 0
	je	.LBB40_522
.Ltmp9745:
	.loc	10 0 12
	vbroadcastss	.LCPI40_1(%rip), %ymm2
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI40_34(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB40_521:
.Ltmp9746:
	.loc	14 82 19 is_stmt 1
	vandps	(%r8,%rcx,4), %ymm2, %ymm4
.Ltmp9747:
	.loc	14 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp9748:
	.loc	14 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp9749:
	.loc	10 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB40_521
.Ltmp9750:
.LBB40_522:
	.loc	14 585 19
	vpsrad	$31, %ymm1, %ymm2
	vpbroadcastd	.LCPI40_32(%rip), %ymm1
	vpandn	%ymm1, %ymm2, %ymm2
.Ltmp9751:
	.loc	50 185 12
	vmovd	%xmm2, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movl	%ecx, 256(%rsp)
	vpextrd	$1, %xmm2, %ecx
	xorl	%eax, %eax
	testl	%ecx, %ecx
	setne	%al
	vpextrd	$2, %xmm2, %ecx
	addl	%eax, %eax
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	shll	$2, %edx
	movl	%edx, 192(%rsp)
	vpextrd	$3, %xmm2, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	shll	$3, %edx
	vextracti128	$1, %ymm2, %xmm2
	vmovd	%xmm2, %ecx
	xorl	%r8d, %r8d
	testl	%ecx, %ecx
	setne	%r8b
	shll	$4, %r8d
	movl	%r8d, 1664(%rsp)
	vpextrd	$1, %xmm2, %ecx
	xorl	%r9d, %r9d
	testl	%ecx, %ecx
	setne	%r9b
	vpextrd	$2, %xmm2, %ecx
	shll	$5, %r9d
	xorl	%r8d, %r8d
	testl	%ecx, %ecx
	setne	%r8b
	shll	$6, %r8d
	vpextrd	$3, %xmm2, %ecx
	xorl	%r10d, %r10d
	testl	%ecx, %ecx
	setne	%r10b
	shll	$7, %r10d
.Ltmp9752:
	.loc	10 2155 12
	andq	%rsi, %r11
	je	.LBB40_525
.Ltmp9753:
	.loc	10 0 12 is_stmt 0
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI40_1(%rip), %ymm2
	vbroadcastss	.LCPI40_34(%rip), %ymm3
	.p2align	4
.LBB40_524:
.Ltmp9754:
	.loc	14 82 19 is_stmt 1
	vandps	(%rdi,%rcx,4), %ymm2, %ymm4
.Ltmp9755:
	.loc	14 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp9756:
	.loc	14 82 19
	vandps	%ymm4, %ymm0, %ymm0
.Ltmp9757:
	.loc	10 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %r11
	jne	.LBB40_524
.Ltmp9758:
.LBB40_525:
	.loc	14 585 19
	vpsrad	$31, %ymm0, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
.Ltmp9759:
	.loc	50 185 12
	vmovd	%xmm0, %ecx
	xorl	%r11d, %r11d
	testl	%ecx, %ecx
	vpextrd	$1, %xmm0, %ecx
	setne	%r11b
	xorl	%ebx, %ebx
	testl	%ecx, %ecx
	setne	%bl
	addl	%ebx, %ebx
	vpextrd	$2, %xmm0, %ecx
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	setne	%r14b
	vpextrd	$3, %xmm0, %ecx
	shll	$2, %r14d
	xorl	%r15d, %r15d
	testl	%ecx, %ecx
	setne	%r15b
	shll	$3, %r15d
	vextracti128	$1, %ymm0, %xmm0
	vmovd	%xmm0, %ecx
	xorl	%r12d, %r12d
	testl	%ecx, %ecx
	setne	%r12b
	vpextrd	$1, %xmm0, %ecx
	shll	$4, %r12d
	xorl	%r13d, %r13d
	testl	%ecx, %ecx
	setne	%r13b
	shll	$5, %r13d
	vpextrd	$2, %xmm0, %ecx
	xorl	%esi, %esi
	testl	%ecx, %ecx
	setne	%sil
	vpextrd	$3, %xmm0, %edi
	shll	$6, %esi
	xorl	%ecx, %ecx
	testl	%edi, %edi
	setne	%cl
	shll	$7, %ecx
.Ltmp9760:
	.loc	50 185 12 is_stmt 0
	orl	%esi, %ecx
.Ltmp9761:
	.loc	50 185 12
	orl	256(%rsp), %eax
	orl	192(%rsp), %eax
	orl	1664(%rsp), %edx
	orl	%r9d, %edx
	orl	%eax, %edx
	orl	%r10d, %r8d
	orl	%edx, %r8d
.Ltmp9762:
	.loc	50 185 12
	orl	%r11d, %r8d
	orl	%ebx, %r8d
	orl	%r14d, %r15d
	orl	%r8d, %r15d
	orl	%r12d, %r13d
	orl	%r15d, %r13d
.Ltmp9763:
	.loc	50 211 5 is_stmt 1
	orl	%ecx, %r13d
	movq	48(%rsp), %rbx
	movl	%r13d, 5256(%rbx)
	.loc	50 212 31
	movq	5248(%rbx), %rax
.Ltmp9764:
	.loc	38 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp9765:
	.loc	50 212 5
	movq	%rcx, 5248(%rbx)
	movq	40(%rsp), %rdx
.Ltmp9766:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp9767:
	.loc	33 180 28
	je	.LBB40_527
.Ltmp9768:
	.loc	34 961 18
	shlq	$2, %rdx
	movq	296(%rsp), %rdi
.Ltmp9769:
	.loc	35 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp9770:
.LBB40_527:
	.loc	35 0 13 is_stmt 0
	movq	32(%rsp), %rdx
.Ltmp9771:
	.loc	32 1714 9 is_stmt 1
	testq	%rdx, %rdx
	movq	304(%rsp), %rdi
.Ltmp9772:
	.loc	33 180 28
	je	.LBB40_529
.Ltmp9773:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp9774:
	.loc	35 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp9775:
.LBB40_529:
	.loc	1 1319 13
	movq	$0, 5272(%rbx)
.Ltmp9776:
	.loc	1 1321 22
	movq	%rbx, %rdi
	vzeroupper
	callq	_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E19discontinuity_resetB5_
.Ltmp9777:
	.loc	32 1714 9
	leaq	2624(%rbx), %rdi
.Ltmp9778:
	.loc	1 1321 22
	callq	_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E19discontinuity_resetB5_
.Ltmp9779:
	.loc	1 1327 20
	movl	5256(%rbx), %eax
.Ltmp9780:
	.loc	1 1330 16
	testb	$1, %al
	movq	1656(%rsp), %rsi
	jne	.LBB40_539
	testb	$2, %al
	jne	.LBB40_540
.LBB40_531:
	testb	$4, %al
	jne	.LBB40_541
.LBB40_532:
	testb	$8, %al
	jne	.LBB40_542
.LBB40_533:
	testb	$16, %al
	jne	.LBB40_543
.LBB40_534:
	testb	$32, %al
	jne	.LBB40_544
.LBB40_535:
	testb	$64, %al
	jne	.LBB40_545
.LBB40_536:
	testb	%al, %al
	jns	.LBB40_538
.LBB40_537:
	.loc	1 0 16 is_stmt 0
	movq	2960(%rsp), %rax
.Ltmp9781:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rax
	movq	$-1, %rcx
	cmovbq	%rcx, %rax
.Ltmp9782:
	.loc	1 1331 17
	movq	%rax, 2960(%rsp)
.Ltmp9783:
	.loc	38 2428 13
	addq	2968(%rsp), %rsi
	cmovbq	%rcx, %rsi
.Ltmp9784:
	.loc	1 1332 17
	movq	%rsi, 2968(%rsp)
.Ltmp9785:
.LBB40_538:
	.loc	1 0 17 is_stmt 0
	leaq	2656(%rsp), %rsi
	movl	$328, %edx
	movq	2648(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp9786:
	.loc	1 1910 6 is_stmt 1
	movq	%rbx, %rax
	leaq	-40(%rbp), %rsp
	.loc	1 1910 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB40_539:
	.cfi_def_cfa %rbp, 16
	.loc	1 0 6
	movq	2680(%rsp), %rcx
.Ltmp9787:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp9788:
	.loc	1 1331 17
	movq	%rcx, 2680(%rsp)
	movq	2688(%rsp), %rcx
.Ltmp9789:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp9790:
	.loc	1 1332 17
	movq	%rcx, 2688(%rsp)
	.loc	1 1330 16
	testb	$2, %al
	je	.LBB40_531
.LBB40_540:
	.loc	1 0 16 is_stmt 0
	movq	2720(%rsp), %rcx
.Ltmp9791:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp9792:
	.loc	1 1331 17
	movq	%rcx, 2720(%rsp)
	movq	2728(%rsp), %rcx
.Ltmp9793:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp9794:
	.loc	1 1332 17
	movq	%rcx, 2728(%rsp)
	.loc	1 1330 16
	testb	$4, %al
	je	.LBB40_532
.LBB40_541:
	.loc	1 0 16 is_stmt 0
	movq	2760(%rsp), %rcx
.Ltmp9795:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp9796:
	.loc	1 1331 17
	movq	%rcx, 2760(%rsp)
	movq	2768(%rsp), %rcx
.Ltmp9797:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp9798:
	.loc	1 1332 17
	movq	%rcx, 2768(%rsp)
	.loc	1 1330 16
	testb	$8, %al
	je	.LBB40_533
.LBB40_542:
	.loc	1 0 16 is_stmt 0
	movq	2800(%rsp), %rcx
.Ltmp9799:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp9800:
	.loc	1 1331 17
	movq	%rcx, 2800(%rsp)
	movq	2808(%rsp), %rcx
.Ltmp9801:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp9802:
	.loc	1 1332 17
	movq	%rcx, 2808(%rsp)
	.loc	1 1330 16
	testb	$16, %al
	je	.LBB40_534
.LBB40_543:
	.loc	1 0 16 is_stmt 0
	movq	2840(%rsp), %rcx
.Ltmp9803:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp9804:
	.loc	1 1331 17
	movq	%rcx, 2840(%rsp)
	movq	2848(%rsp), %rcx
.Ltmp9805:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp9806:
	.loc	1 1332 17
	movq	%rcx, 2848(%rsp)
	.loc	1 1330 16
	testb	$32, %al
	je	.LBB40_535
.LBB40_544:
	.loc	1 0 16 is_stmt 0
	movq	2880(%rsp), %rcx
.Ltmp9807:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp9808:
	.loc	1 1331 17
	movq	%rcx, 2880(%rsp)
	movq	2888(%rsp), %rcx
.Ltmp9809:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp9810:
	.loc	1 1332 17
	movq	%rcx, 2888(%rsp)
	.loc	1 1330 16
	testb	$64, %al
	je	.LBB40_536
.LBB40_545:
	.loc	1 0 16 is_stmt 0
	movq	2920(%rsp), %rcx
.Ltmp9811:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp9812:
	.loc	1 1331 17
	movq	%rcx, 2920(%rsp)
	movq	2928(%rsp), %rcx
.Ltmp9813:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp9814:
	.loc	1 1332 17
	movq	%rcx, 2928(%rsp)
	.loc	1 1330 16
	testb	%al, %al
	js	.LBB40_537
	jmp	.LBB40_538
.Ltmp9815:
.LBB40_558:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_556:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r9, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_546:
.Ltmp9816:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9817:
.LBB40_547:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	xorl	%edx, %edx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9818:
.LBB40_548:
	.loc	48 443 13 is_stmt 1
	leaq	.Lalloc_8ba50ea0f27235615a85de9e15747d68(%rip), %rcx
	movq	1696(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9819:
.LBB40_549:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_e1197c8f188c3c9faae54184f2eb21df(%rip), %rcx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_550:
	leaq	.Lalloc_a5fe42438bf1cd51848d461e43168e01(%rip), %rcx
.Ltmp9820:
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_551:
.Ltmp9821:
	leaq	.Lalloc_947a12da53e0da5ef683a81874315ce0(%rip), %rcx
.Ltmp9822:
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_552:
.Ltmp9823:
	leaq	.Lalloc_3c433846b75a8609e2aa4275946b39b4(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_553:
	leaq	.Lalloc_f7cdc51debee5c62c8dce89684c9c22f(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9824:
.LBB40_554:
	leaq	.Lalloc_f7aeb6b0a3ba8e73c50a5abef6c30558(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_559:
	leaq	.Lalloc_a8e669d1bed0fb747f8a7bff920a6571(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_560:
	leaq	.Lalloc_1e81c2bc19b75441ce2fb90ce8f9eb70(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_561:
	leaq	.Lalloc_007bf1cfdcf9f845db5ef166fc9a1790(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_562:
	leaq	.Lalloc_a45b75cd2d07085fe69fe186ba115725(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_563:
	leaq	.Lalloc_36be93341d7083c8a492c530428430ea(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_564:
	leaq	.Lalloc_8a6f1b2a44d3e33eba5c708677237c81(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_565:
	leaq	.Lalloc_4aa2eaec3d1833a4a887fe1d76c05ca7(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_555:
	leaq	.Lalloc_e1197c8f188c3c9faae54184f2eb21df(%rip), %rcx
	movq	2048(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_557:
.Ltmp9825:
	leaq	.Lalloc_e1197c8f188c3c9faae54184f2eb21df(%rip), %rcx
	movq	56(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9826:
.LBB40_566:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_a05c61cb172b1016332ce1d3ce81e461(%rip), %rcx
.Ltmp9827:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	40(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9828:
.LBB40_567:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_61a2f59006034c74bb8ab3eed52140c6(%rip), %rcx
.Ltmp9829:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	40(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9830:
.LBB40_568:
	.loc	1 0 0
	leaq	.Lalloc_a05c61cb172b1016332ce1d3ce81e461(%rip), %rcx
	movq	%r15, %rdi
	movq	40(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_569:
	leaq	.Lalloc_61a2f59006034c74bb8ab3eed52140c6(%rip), %rcx
	movq	%r15, %rdi
	movq	40(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_570:
.Ltmp9831:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_2f8086566e4ce1876649681c0220d707(%rip), %rcx
.Ltmp9832:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	32(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9833:
.LBB40_571:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_24e70b7e4de4498f6c3c29a55eaeb9ab(%rip), %rcx
.Ltmp9834:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	32(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9835:
.LBB40_573:
	.loc	1 0 0
	leaq	.Lalloc_24e70b7e4de4498f6c3c29a55eaeb9ab(%rip), %rcx
	movq	%r15, %rdi
	movq	32(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_572:
	leaq	.Lalloc_2f8086566e4ce1876649681c0220d707(%rip), %rcx
	movq	%r15, %rdi
	movq	32(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_595:
	movq	160(%rsp), %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_597:
	movq	224(%rsp), %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_588:
	movq	%r8, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_587:
	movq	64(%rsp), %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_599:
	movq	%rax, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_574:
	movq	16(%rsp), %r8
.LBB40_575:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_586:
	movq	%r12, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_607:
	movq	%r15, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9836:
.LBB40_576:
	.loc	1 1892 25 is_stmt 1
	leaq	.Lalloc_9f2c0b91206fa9484b77a809bfc45da2(%rip), %rdx
	movq	%r8, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_577:
.Ltmp9837:
	.loc	1 1893 23
	leaq	.Lalloc_81c87390c70034a3e9ca28b66f0c0cdf(%rip), %rdx
	movq	%rcx, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9838:
.LBB40_578:
	.loc	1 0 23 is_stmt 0
	movq	%r13, %rcx
.LBB40_579:
.Ltmp9839:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_585:
	movq	%rdx, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_606:
	movq	%rdx, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_591:
	movq	%r12, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_598:
	movq	%r15, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_594:
	movq	24(%rsp), %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_589:
	movq	%rax, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_596:
	movq	1824(%rsp), %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_592:
	movq	%r13, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_590:
	movq	%r10, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_624:
	movq	%r13, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_602:
	movq	24(%rsp), %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_600:
	movq	1824(%rsp), %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_604:
	movq	%r12, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_618:
	movq	%rax, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9840:
.LBB40_580:
	.loc	1 1376 43 is_stmt 1
	leaq	.Lalloc_26decc7ea6b284c3e2e2a8674b90acb8(%rip), %rdx
	movl	$12, %esi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9841:
.LBB40_583:
	.loc	1 0 43 is_stmt 0
	movq	320(%rsp), %rcx
.Ltmp9842:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_584:
	movq	%r11, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_581:
	movq	16(%rsp), %r8
.Ltmp9843:
	.loc	1 877 25 is_stmt 1
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9844:
.LBB40_593:
	.loc	1 0 25 is_stmt 0
	movq	24(%rsp), %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_601:
	movq	24(%rsp), %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_612:
	movq	%rax, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_616:
	movq	%r8, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_627:
	movq	320(%rsp), %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_605:
	movq	%r10, %r8
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9845:
.LBB40_582:
	.loc	1 1378 45 is_stmt 1
	leaq	.Lalloc_229cdd12ec7f4a0b614787777e6def50(%rip), %rdx
	movl	$10, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9846:
.LBB40_611:
	.loc	1 0 45 is_stmt 0
	movq	224(%rsp), %rcx
.Ltmp9847:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_619:
	movq	64(%rsp), %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_629:
	movq	%r12, %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_634:
	movq	1824(%rsp), %r8
.LBB40_635:
.Ltmp9848:
	.loc	1 877 25 is_stmt 1
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9849:
.LBB40_636:
	.loc	1 0 25 is_stmt 0
	movq	64(%rsp), %rcx
.LBB40_637:
.Ltmp9850:
	.loc	1 877 25 is_stmt 1
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9851:
.LBB40_641:
	.loc	1 0 25 is_stmt 0
	movq	160(%rsp), %rcx
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%rcx, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9852:
.Lfunc_end40:
	.size	_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_, .Lfunc_end40-_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_
