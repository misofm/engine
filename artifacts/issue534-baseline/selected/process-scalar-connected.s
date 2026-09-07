_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_:
.Lfunc_begin38:
	.loc	6 995 0 is_stmt 1
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
	subq	$472, %rsp
	.cfi_def_cfa_offset 528
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r8
	movq	%rsi, %r10
	movq	%rdi, %r15
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqu	%ymm0, 432(%rsp)
	vmovdqu	%ymm0, 400(%rsp)
	vmovdqu	%ymm0, 368(%rsp)
	vmovdqu	%ymm0, 336(%rsp)
	vmovdqu	%ymm0, 304(%rsp)
	vmovdqu	%ymm0, 272(%rsp)
	vmovdqu	%ymm0, 240(%rsp)
	vmovdqu	%ymm0, 208(%rsp)
	vmovdqu	%ymm0, 176(%rsp)
	vmovdqu	%ymm0, 144(%rsp)
.Ltmp6684:
	.loc	6 997 31 prologue_end
	movq	32(%rdx), %r12
	movq	40(%rdx), %rax
	.loc	6 997 49 is_stmt 0
	movq	80(%rdx), %rdx
	movl	$0, 24(%rsp)
	movl	$0, 32(%rsp)
	movl	$0, 40(%rsp)
	movl	$0, 48(%rsp)
	movl	$0, 56(%rsp)
	movl	$0, 64(%rsp)
	movl	$0, 72(%rsp)
	movl	$0, 80(%rsp)
.Ltmp6685:
	.loc	38 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp6686:
	.loc	19 180 28
	je	.LBB38_42
.Ltmp6687:
	.loc	19 0 28 is_stmt 0
	leaq	56(%rsp), %rsi
	leaq	(%rax,%rax,4), %rax
	leaq	(%r12,%rax,8), %rbx
	movl	92(%r10), %edi
	movb	$1, %al
	movl	%eax, 16(%rsp)
	xorl	%ebp, %ebp
	leaq	24(%rsp), %r9
	leaq	.Lalloc_d0058eaee52f1ba8b2e5577cef0c3c7f(%rip), %r11
	xorl	%r14d, %r14d
	movq	%r15, 88(%rsp)
	jmp	.LBB38_2
	.p2align	4
.LBB38_91:
	addq	$40, %r12
.Ltmp6688:
	.loc	15 2428 13 is_stmt 1
	incq	%rbp
	movq	$-1, %rax
	cmoveq	%rax, %rbp
.Ltmp6689:
	.loc	34 82 9
	incq	%r14
.Ltmp6690:
	.loc	38 1714 9
	cmpq	%rbx, %r12
.Ltmp6691:
	.loc	19 180 28
	je	.LBB38_41
.Ltmp6692:
.LBB38_2:
	.loc	6 613 33
	movl	32(%r12), %eax
	.loc	6 613 27 is_stmt 0
	cmpl	$1, %eax
	je	.LBB38_3
	cmpl	$2, %eax
	jne	.LBB38_91
	.loc	6 0 27
	movl	$1, %eax
	movq	%rsi, %r13
	jmp	.LBB38_79
	.p2align	4
.LBB38_3:
	xorl	%eax, %eax
	movq	%r9, %r13
.LBB38_79:
.Ltmp6693:
	.loc	6 621 35 is_stmt 1
	movl	16(%r12), %r15d
.Ltmp6694:
	.loc	15 3178 26
	testl	%r15d, %r15d
.Ltmp6695:
	.loc	46 459 8
	js	.LBB38_91
.Ltmp6696:
	.loc	6 630 25
	cmpq	%rdi, %r14
	jae	.LBB38_91
	cmpl	$3, %r15d
	ja	.LBB38_91
	.loc	6 632 20
	cmpl	$1, 28(%r12)
	jne	.LBB38_91
	.loc	6 633 20
	cmpq	%rdx, (%r12)
	jne	.LBB38_91
	.loc	6 634 20
	cmpq	%rdx, 8(%r12)
	jne	.LBB38_91
	.loc	6 635 20
	vmovd	20(%r12), %xmm0
.Ltmp6697:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6698:
	.loc	6 635 20
	cmpl	%ecx, 24(%r12)
	jne	.LBB38_91
.Ltmp6699:
	.loc	6 0 0 is_stmt 0
	leal	(%rax,%r15,2), %eax
	movl	%eax, 12(%rsp)
.Ltmp6700:
	.loc	6 636 42 is_stmt 1
	leaq	(%r15,%r15,4), %rax
	movq	%rdi, 96(%rsp)
	leaq	(%r11,%rax,8), %rdi
	movq	%r10, 120(%rsp)
	movq	%r8, 112(%rsp)
	movq	%rdx, 104(%rsp)
	vmovdqa	%xmm0, 128(%rsp)
	.loc	6 636 20 is_stmt 0
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	128(%rsp), %xmm1
	leaq	.Lalloc_d0058eaee52f1ba8b2e5577cef0c3c7f(%rip), %r11
	leaq	24(%rsp), %r9
	movq	96(%rsp), %rdi
	leaq	56(%rsp), %rsi
	movq	104(%rsp), %rdx
	movq	112(%rsp), %r8
	movq	120(%rsp), %r10
	movl	12(%rsp), %ecx
	cmpl	20(%rsp), %ecx
	seta	%cl
	testb	%al, %al
	je	.LBB38_91
	orb	16(%rsp), %cl
	testb	$1, %cl
	je	.LBB38_91
.Ltmp6701:
	.loc	6 639 17 is_stmt 1
	cmpb	$0, (%r13,%r15,8)
	jne	.LBB38_91
.Ltmp6702:
	.loc	6 0 0 is_stmt 0
	movq	%rbp, 160(%rsp)
.Ltmp6703:
	.loc	12 110 8 is_stmt 1
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
.Ltmp6704:
	.loc	6 644 13
	movl	$1, (%r13,%r15,8)
	vmovss	%xmm0, 4(%r13,%r15,8)
.Ltmp6705:
	.loc	38 1714 9
	addq	$40, %r12
.Ltmp6706:
	.loc	19 180 28
	incq	%r14
	movl	$0, 16(%rsp)
	movl	12(%rsp), %eax
	movl	%eax, 20(%rsp)
.Ltmp6707:
	.loc	38 1714 9
	cmpq	%rbx, %r12
	movq	88(%rsp), %r15
.Ltmp6708:
	.loc	19 180 28
	jne	.LBB38_2
	jmp	.LBB38_42
.Ltmp6709:
.LBB38_41:
	.loc	6 0 0 is_stmt 0
	movq	%rbp, 160(%rsp)
	movq	88(%rsp), %r15
.Ltmp6710:
.LBB38_42:
	.loc	6 648 21 is_stmt 1
	cmpl	$1, 24(%rsp)
	je	.LBB38_43
	cmpl	$1, 32(%rsp)
	je	.LBB38_52
.LBB38_60:
	cmpl	$1, 40(%rsp)
	je	.LBB38_61
.LBB38_69:
	cmpl	$1, 48(%rsp)
	je	.LBB38_70
.LBB38_4:
	cmpl	$1, 56(%rsp)
	je	.LBB38_5
.LBB38_13:
	cmpl	$1, 64(%rsp)
	je	.LBB38_14
.LBB38_22:
	cmpl	$1, 72(%rsp)
	je	.LBB38_23
.LBB38_31:
	cmpl	$1, 80(%rsp)
	je	.LBB38_32
	jmp	.LBB38_40
.LBB38_43:
	.loc	6 648 26 is_stmt 0
	vmovd	28(%rsp), %xmm0
.Ltmp6711:
	.loc	6 651 39 is_stmt 1
	vmovd	792(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp6712:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6713:
	.loc	47 112 9
	cmpl	%ecx, %eax
	setne	%cl
	movl	%eax, %edx
	negl	%edx
	seto	%dl
	orb	%cl, %dl
	andl	$2147483647, %eax
	cmpl	$2139095040, %eax
	setge	%al
	orb	%dl, %al
	vmovdqa	%xmm1, %xmm2
	testb	%al, %al
	jne	.LBB38_45
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB38_45:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB38_46
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB38_49
	jmp	.LBB38_50
.LBB38_46:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI38_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB38_50
.Ltmp6714:
.LBB38_49:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_50:
.Ltmp6715:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 792(%r10)
.Ltmp6716:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 796(%r10)
.Ltmp6717:
	.loc	6 398 5
	vmovss	%xmm1, 800(%r10)
.Ltmp6718:
	.loc	6 398 5
	vmovss	%xmm3, 804(%r10)
.Ltmp6719:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp6720:
	.loc	6 648 21
	cmpl	$1, 32(%rsp)
	jne	.LBB38_60
.LBB38_52:
	.loc	6 648 26 is_stmt 0
	vmovd	36(%rsp), %xmm0
.Ltmp6721:
	.loc	6 651 39 is_stmt 1
	vmovd	808(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp6722:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6723:
	.loc	47 112 9
	cmpl	%ecx, %eax
	setne	%cl
	movl	%eax, %edx
	negl	%edx
	seto	%dl
	orb	%cl, %dl
	andl	$2147483647, %eax
	cmpl	$2139095040, %eax
	setge	%al
	orb	%dl, %al
	vmovdqa	%xmm1, %xmm2
	testb	%al, %al
	jne	.LBB38_54
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB38_54:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB38_55
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB38_58
	jmp	.LBB38_59
.LBB38_55:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI38_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB38_59
.Ltmp6724:
.LBB38_58:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_59:
.Ltmp6725:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 808(%r10)
.Ltmp6726:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 812(%r10)
.Ltmp6727:
	.loc	6 398 5
	vmovss	%xmm1, 816(%r10)
.Ltmp6728:
	.loc	6 398 5
	vmovss	%xmm3, 820(%r10)
.Ltmp6729:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp6730:
	.loc	6 648 21
	cmpl	$1, 40(%rsp)
	jne	.LBB38_69
.LBB38_61:
	.loc	6 648 26 is_stmt 0
	vmovd	44(%rsp), %xmm0
.Ltmp6731:
	.loc	6 651 39 is_stmt 1
	vmovd	824(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp6732:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6733:
	.loc	47 112 9
	cmpl	%ecx, %eax
	setne	%cl
	movl	%eax, %edx
	negl	%edx
	seto	%dl
	orb	%cl, %dl
	andl	$2147483647, %eax
	cmpl	$2139095040, %eax
	setge	%al
	orb	%dl, %al
	vmovdqa	%xmm1, %xmm2
	testb	%al, %al
	jne	.LBB38_63
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB38_63:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB38_64
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB38_67
	jmp	.LBB38_68
.LBB38_64:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI38_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB38_68
.Ltmp6734:
.LBB38_67:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_68:
.Ltmp6735:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 824(%r10)
.Ltmp6736:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 828(%r10)
.Ltmp6737:
	.loc	6 398 5
	vmovss	%xmm1, 832(%r10)
.Ltmp6738:
	.loc	6 398 5
	vmovss	%xmm3, 836(%r10)
.Ltmp6739:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp6740:
	.loc	6 648 21
	cmpl	$1, 48(%rsp)
	jne	.LBB38_4
.LBB38_70:
	.loc	6 648 26 is_stmt 0
	vmovd	52(%rsp), %xmm0
.Ltmp6741:
	.loc	6 651 39 is_stmt 1
	vmovd	840(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp6742:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6743:
	.loc	47 112 9
	cmpl	%ecx, %eax
	setne	%cl
	movl	%eax, %edx
	negl	%edx
	seto	%dl
	orb	%cl, %dl
	andl	$2147483647, %eax
	cmpl	$2139095040, %eax
	setge	%al
	orb	%dl, %al
	vmovdqa	%xmm1, %xmm2
	testb	%al, %al
	jne	.LBB38_72
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB38_72:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB38_73
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB38_76
	jmp	.LBB38_77
.LBB38_73:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI38_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB38_77
.Ltmp6744:
.LBB38_76:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_77:
.Ltmp6745:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 840(%r10)
.Ltmp6746:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 844(%r10)
.Ltmp6747:
	.loc	6 398 5
	vmovss	%xmm1, 848(%r10)
.Ltmp6748:
	.loc	6 398 5
	vmovss	%xmm3, 852(%r10)
.Ltmp6749:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp6750:
	.loc	6 648 21
	cmpl	$1, 56(%rsp)
	jne	.LBB38_13
.LBB38_5:
	.loc	6 648 26 is_stmt 0
	vmovd	60(%rsp), %xmm0
.Ltmp6751:
	.loc	6 651 39 is_stmt 1
	vmovd	868(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp6752:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6753:
	.loc	47 112 9
	cmpl	%ecx, %eax
	setne	%cl
	movl	%eax, %edx
	negl	%edx
	seto	%dl
	orb	%cl, %dl
	andl	$2147483647, %eax
	cmpl	$2139095040, %eax
	setge	%al
	orb	%dl, %al
	vmovdqa	%xmm1, %xmm2
	testb	%al, %al
	jne	.LBB38_7
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB38_7:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB38_8
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB38_11
	jmp	.LBB38_12
.LBB38_8:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI38_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB38_12
.Ltmp6754:
.LBB38_11:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_12:
.Ltmp6755:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 868(%r10)
.Ltmp6756:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 872(%r10)
.Ltmp6757:
	.loc	6 398 5
	vmovss	%xmm1, 876(%r10)
.Ltmp6758:
	.loc	6 398 5
	vmovss	%xmm3, 880(%r10)
.Ltmp6759:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp6760:
	.loc	6 648 21
	cmpl	$1, 64(%rsp)
	jne	.LBB38_22
.LBB38_14:
	.loc	6 648 26 is_stmt 0
	vmovd	68(%rsp), %xmm0
.Ltmp6761:
	.loc	6 651 39 is_stmt 1
	vmovd	884(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp6762:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6763:
	.loc	47 112 9
	cmpl	%ecx, %eax
	setne	%cl
	movl	%eax, %edx
	negl	%edx
	seto	%dl
	orb	%cl, %dl
	andl	$2147483647, %eax
	cmpl	$2139095040, %eax
	setge	%al
	orb	%dl, %al
	vmovdqa	%xmm1, %xmm2
	testb	%al, %al
	jne	.LBB38_16
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB38_16:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB38_17
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB38_20
	jmp	.LBB38_21
.LBB38_17:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI38_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB38_21
.Ltmp6764:
.LBB38_20:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_21:
.Ltmp6765:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 884(%r10)
.Ltmp6766:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 888(%r10)
.Ltmp6767:
	.loc	6 398 5
	vmovss	%xmm1, 892(%r10)
.Ltmp6768:
	.loc	6 398 5
	vmovss	%xmm3, 896(%r10)
.Ltmp6769:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp6770:
	.loc	6 648 21
	cmpl	$1, 72(%rsp)
	jne	.LBB38_31
.LBB38_23:
	.loc	6 648 26 is_stmt 0
	vmovd	76(%rsp), %xmm0
.Ltmp6771:
	.loc	6 651 39 is_stmt 1
	vmovd	900(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp6772:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6773:
	.loc	47 112 9
	cmpl	%ecx, %eax
	setne	%cl
	movl	%eax, %edx
	negl	%edx
	seto	%dl
	orb	%cl, %dl
	andl	$2147483647, %eax
	cmpl	$2139095040, %eax
	setge	%al
	orb	%dl, %al
	vmovdqa	%xmm1, %xmm2
	testb	%al, %al
	jne	.LBB38_25
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB38_25:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB38_26
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB38_29
	jmp	.LBB38_30
.LBB38_26:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI38_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB38_30
.Ltmp6774:
.LBB38_29:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_30:
.Ltmp6775:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 900(%r10)
.Ltmp6776:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 904(%r10)
.Ltmp6777:
	.loc	6 398 5
	vmovss	%xmm1, 908(%r10)
.Ltmp6778:
	.loc	6 398 5
	vmovss	%xmm3, 912(%r10)
.Ltmp6779:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp6780:
	.loc	6 648 21
	cmpl	$1, 80(%rsp)
	jne	.LBB38_40
.LBB38_32:
	.loc	6 648 26 is_stmt 0
	vmovd	84(%rsp), %xmm0
.Ltmp6781:
	.loc	6 651 39 is_stmt 1
	vmovd	916(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp6782:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6783:
	.loc	47 112 9
	cmpl	%ecx, %eax
	setne	%cl
	movl	%eax, %edx
	negl	%edx
	seto	%dl
	orb	%cl, %dl
	andl	$2147483647, %eax
	cmpl	$2139095040, %eax
	setge	%al
	orb	%dl, %al
	vmovdqa	%xmm1, %xmm2
	testb	%al, %al
	jne	.LBB38_34
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB38_34:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB38_35
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB38_38
	jmp	.LBB38_39
.LBB38_35:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI38_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB38_39
.Ltmp6784:
.LBB38_38:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_39:
.Ltmp6785:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 916(%r10)
.Ltmp6786:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 920(%r10)
.Ltmp6787:
	.loc	6 398 5
	vmovss	%xmm1, 924(%r10)
.Ltmp6788:
	.loc	6 398 5
	vmovss	%xmm3, 928(%r10)
.Ltmp6789:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp6790:
.LBB38_40:
	.loc	20 1095 9
	movq	(%r8), %rsi
	movq	8(%r8), %rdx
.Ltmp6791:
	.loc	6 999 40
	leaq	48(%r8), %r9
.Ltmp6792:
	.loc	6 1000 36
	movq	16(%r8), %rcx
	movq	24(%r8), %r8
	leaq	144(%rsp), %rax
	.loc	6 1000 14 is_stmt 0
	movq	%r10, %rdi
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	%rdx
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_
	addq	$16, %rsp
	.cfi_adjust_cfa_offset -16
	.loc	6 1001 9 is_stmt 1
	movq	176(%rsp), %rax
	movq	%rax, 32(%r15)
	vmovups	144(%rsp), %ymm0
	vmovups	%ymm0, (%r15)
.Ltmp6793:
	.loc	6 1002 6
	movq	%r15, %rax
	.loc	6 1002 6 epilogue_begin is_stmt 0
	addq	$472, %rsp
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
.Ltmp6794:
.Lfunc_end38:
	.size	_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_, .Lfunc_end38-_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_
