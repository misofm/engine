_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_:
.Lfunc_begin36:
	.loc	1 2184 0 is_stmt 1
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
	subq	$3672, %rsp
	.cfi_def_cfa_offset 3728
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%r9, 120(%rsp)
	movq	%r8, 136(%rsp)
	movq	%rcx, %rbp
	movq	%rdx, %r15
	movq	%rsi, %r13
	movq	%rdi, %r12
.Ltmp9826:
	.loc	1 2203 21 prologue_end
	movzbl	781(%rdi), %edx
	.loc	1 0 0 is_stmt 0
	movq	264(%rdi), %rcx
	movq	272(%rdi), %rax
	.loc	1 2203 21
	cmpb	96(%rdi), %dl
	jne	.LBB36_29
.Ltmp9827:
	.loc	6 314 17 is_stmt 1
	movq	%rax, %rdx
	shlq	$4, %rdx
	movq	%rcx, %rsi
	.loc	6 0 17 is_stmt 0
.Ltmp9828:
	.p2align	4
.LBB36_2:
.Ltmp9829:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9830:
	.loc	6 180 28
	je	.LBB36_5
.Ltmp9831:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9832:
	.loc	6 315 25
	jne	.LBB36_29
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB36_2
	jmp	.LBB36_29
.Ltmp9833:
.LBB36_5:
	.loc	1 2205 37 is_stmt 1
	movq	280(%r12), %rsi
	movq	288(%r12), %rdx
.Ltmp9834:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp9835:
	.p2align	4
.LBB36_6:
.Ltmp9836:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9837:
	.loc	6 180 28
	je	.LBB36_9
.Ltmp9838:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9839:
	.loc	6 315 25
	jne	.LBB36_29
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB36_6
	jmp	.LBB36_29
.Ltmp9840:
.LBB36_9:
	.loc	1 2206 37 is_stmt 1
	movq	464(%r12), %rsi
	movq	472(%r12), %rdx
.Ltmp9841:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp9842:
	.p2align	4
.LBB36_10:
.Ltmp9843:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9844:
	.loc	6 180 28
	je	.LBB36_13
.Ltmp9845:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9846:
	.loc	6 315 25
	jne	.LBB36_29
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB36_10
	jmp	.LBB36_29
.Ltmp9847:
.LBB36_13:
	.loc	1 2207 37 is_stmt 1
	movq	480(%r12), %rsi
	movq	488(%r12), %rdx
.Ltmp9848:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp9849:
	.p2align	4
.LBB36_14:
.Ltmp9850:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9851:
	.loc	6 180 28
	je	.LBB36_17
.Ltmp9852:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9853:
	.loc	6 315 25
	jne	.LBB36_29
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB36_14
	jmp	.LBB36_29
.Ltmp9854:
.LBB36_17:
	.loc	6 0 25
	movq	120(%rsp), %rsi
	cmpq	%r15, %rsi
.Ltmp9855:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB36_626
.Ltmp9856:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	%r13, %rsi
	.p2align	4
.LBB36_19:
.Ltmp9857:
	.loc	15 1504 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB36_23
.Ltmp9858:
	.loc	10 1078 5
	cmpq	$32, %rdx
	movl	$32, %edi
	cmovbq	%rdx, %rdi
.Ltmp9859:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp9860:
	.loc	16 0 18 is_stmt 0
.Ltmp9861:
	.p2align	4
.LBB36_21:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp9862:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp9863:
	.loc	6 180 28
	jne	.LBB36_21
.Ltmp9864:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp9865:
	.loc	19 2054 74
	subq	%rdi, %rdx
.Ltmp9866:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB36_19
	jmp	.LBB36_29
.Ltmp9867:
.LBB36_23:
	.loc	17 0 12 is_stmt 0
	movq	120(%rsp), %rsi
.Ltmp9868:
	.loc	5 438 16 is_stmt 1
	cmpq	136(%rsp), %rsi
	ja	.LBB36_632
.Ltmp9869:
	.loc	5 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	%rbp, %rsi
	.p2align	4
.LBB36_25:
.Ltmp9870:
	.loc	15 1504 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB36_466
.Ltmp9871:
	.loc	10 1078 5
	cmpq	$32, %rdx
	movl	$32, %edi
	cmovbq	%rdx, %rdi
.Ltmp9872:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp9873:
	.loc	16 0 18 is_stmt 0
.Ltmp9874:
	.p2align	4
.LBB36_27:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp9875:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp9876:
	.loc	6 180 28
	jne	.LBB36_27
.Ltmp9877:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp9878:
	.loc	19 2054 74
	subq	%rdi, %rdx
.Ltmp9879:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB36_25
.Ltmp9880:
.LBB36_29:
	.loc	17 0 12 is_stmt 0
	xorl	%edx, %edx
.LBB36_30:
	movl	%edx, 1532(%rsp)
.Ltmp9881:
	.loc	1 2239 13 is_stmt 1
	leaq	536(%r12), %rdx
	movq	%rdx, 1216(%rsp)
	.loc	1 2240 13
	leaq	136(%r12), %rdx
	movq	%rdx, 952(%rsp)
	.loc	1 2241 13
	leaq	336(%r12), %rdx
	movq	%rdx, 944(%rsp)
.Ltmp9882:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp9883:
	.p2align	4
.LBB36_31:
.Ltmp9884:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp9885:
	.loc	6 180 28
	je	.LBB36_34
.Ltmp9886:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp9887:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp9888:
	.loc	6 315 25
	jne	.LBB36_47
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB36_31
	jmp	.LBB36_47
.Ltmp9889:
.LBB36_34:
	.loc	1 708 33 is_stmt 1
	movq	280(%r12), %rcx
	movq	288(%r12), %rax
.Ltmp9890:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp9891:
	.p2align	4
.LBB36_35:
.Ltmp9892:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp9893:
	.loc	6 180 28
	je	.LBB36_38
.Ltmp9894:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp9895:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp9896:
	.loc	6 315 25
	jne	.LBB36_47
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB36_35
	jmp	.LBB36_47
.Ltmp9897:
.LBB36_38:
	.loc	1 709 33 is_stmt 1
	movq	464(%r12), %rcx
	movq	472(%r12), %rax
.Ltmp9898:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp9899:
	.p2align	4
.LBB36_39:
.Ltmp9900:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp9901:
	.loc	6 180 28
	je	.LBB36_42
.Ltmp9902:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp9903:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp9904:
	.loc	6 315 25
	jne	.LBB36_47
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB36_39
	jmp	.LBB36_47
.Ltmp9905:
.LBB36_42:
	.loc	1 710 33 is_stmt 1
	movq	480(%r12), %rcx
	movq	488(%r12), %rax
.Ltmp9906:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp9907:
	.p2align	4
.LBB36_43:
.Ltmp9908:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp9909:
	.loc	6 180 28
	je	.LBB36_46
.Ltmp9910:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp9911:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp9912:
	.loc	6 315 25
	jne	.LBB36_47
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB36_43
	jmp	.LBB36_47
.Ltmp9913:
.LBB36_46:
	.loc	6 0 25
	movb	$1, %dl
.LBB36_47:
.Ltmp9914:
	.loc	1 1116 5 is_stmt 1
	movq	320(%r12), %rcx
	testq	%rcx, %rcx
	movq	%r12, 696(%rsp)
	movq	%r15, 968(%rsp)
	movq	%r13, 976(%rsp)
	movq	%rbp, 840(%rsp)
	je	.LBB36_53
	.loc	1 0 5 is_stmt 0
	movq	312(%r12), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rsi
	.p2align	4
.LBB36_49:
.Ltmp9915:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp9916:
	.loc	6 180 28
	je	.LBB36_53
.Ltmp9917:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rax), %edi
	jne	.LBB36_67
	movl	4(%rsi), %edi
	cmpl	4(%rax), %edi
	jne	.LBB36_67
	movl	8(%rsi), %edi
.Ltmp9918:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp9919:
	.loc	1 390 34
	cmpl	8(%rax), %edi
.Ltmp9920:
	.loc	6 315 25
	je	.LBB36_49
	jmp	.LBB36_67
.Ltmp9921:
.LBB36_53:
	.loc	1 1117 12
	movq	256(%r12), %rax
	testq	%rax, %rax
	je	.LBB36_57
	.loc	1 0 12 is_stmt 0
	movq	248(%r12), %rcx
	shlq	$2, %rax
	xorl	%esi, %esi
	.p2align	4
.LBB36_55:
.Ltmp9922:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rax
.Ltmp9923:
	.loc	6 180 28
	je	.LBB36_57
.Ltmp9924:
	.loc	6 315 25
	movl	(%rcx,%rsi), %edi
	addq	$4, %rsi
.Ltmp9925:
	.loc	1 1117 43
	cmpl	(%rcx), %edi
.Ltmp9926:
	.loc	6 315 25
	je	.LBB36_55
	jmp	.LBB36_67
.Ltmp9927:
.LBB36_57:
	.loc	1 1116 5
	movq	520(%r12), %rcx
	testq	%rcx, %rcx
	je	.LBB36_63
	.loc	1 0 5 is_stmt 0
	movq	512(%r12), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rsi
	.p2align	4
.LBB36_59:
.Ltmp9928:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp9929:
	.loc	6 180 28
	je	.LBB36_63
.Ltmp9930:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rax), %edi
	jne	.LBB36_67
	movl	4(%rsi), %edi
	cmpl	4(%rax), %edi
	jne	.LBB36_67
	movl	8(%rsi), %edi
.Ltmp9931:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp9932:
	.loc	1 390 34
	cmpl	8(%rax), %edi
.Ltmp9933:
	.loc	6 315 25
	je	.LBB36_59
	jmp	.LBB36_67
.Ltmp9934:
.LBB36_63:
	.loc	1 1117 12
	movq	456(%r12), %rax
	testq	%rax, %rax
	je	.LBB36_364
	.loc	1 0 12 is_stmt 0
	movq	448(%r12), %rcx
	shlq	$2, %rax
	xorl	%esi, %esi
	.p2align	4
.LBB36_65:
.Ltmp9935:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rax
.Ltmp9936:
	.loc	6 180 28
	je	.LBB36_364
.Ltmp9937:
	.loc	6 315 25
	movl	(%rcx,%rsi), %edi
	addq	$4, %rsi
.Ltmp9938:
	.loc	1 1117 43
	cmpl	(%rcx), %edi
.Ltmp9939:
	.loc	6 315 25
	je	.LBB36_65
.Ltmp9940:
.LBB36_67:
	.loc	1 1706 12
	testb	%dl, %dl
	je	.LBB36_196
	.loc	1 0 12 is_stmt 0
	leaq	1240(%rsp), %rdi
.Ltmp9941:
	.loc	1 1794 24 is_stmt 1
	leaq	136(%r12), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	1332(%rsp), %rdi
.Ltmp9942:
	.loc	1 1795 25
	leaq	336(%r12), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp9943:
	.loc	1 1800 19
	movzbl	776(%r12), %eax
	movl	%eax, 112(%rsp)
.Ltmp9944:
	.loc	1 1801 21
	movzbl	777(%r12), %ebx
.Ltmp9945:
	.loc	1 1802 27
	movl	560(%r12), %eax
	movq	%rax, 432(%rsp)
.Ltmp9946:
	.loc	1 1803 27
	movl	564(%r12), %eax
	movq	%rax, 8(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%ymm0, 848(%rsp)
	leaq	2648(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r14
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%r14
	leaq	1616(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r14
	movq	120(%rsp), %rcx
.Ltmp9947:
	.loc	8 446 20
	testq	%rcx, %rcx
	je	.LBB36_336
.Ltmp9948:
	.loc	8 0 20 is_stmt 0
	movl	112(%rsp), %r8d
	movl	%r8d, %eax
	negl	%eax
	movl	%eax, 364(%rsp)
.Ltmp9949:
	movl	%ebx, %eax
	negl	%eax
	movl	%eax, 160(%rsp)
.Ltmp9950:
	.loc	4 3756 21 is_stmt 1
	movq	%rcx, %rdx
	shrq	$5, %rdx
.Ltmp9951:
	.loc	4 3757 21
	movl	%ecx, %eax
	andl	$31, %eax
.Ltmp9952:
	.loc	4 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %rdx
	decl	%r8d
	decl	%ebx
	movq	$0, 128(%rsp)
	vxorps	%xmm5, %xmm5, %xmm5
	movq	%rbp, %rsi
	movq	%r13, 1168(%rsp)
	movq	%rcx, %rax
	xorl	%edi, %edi
	movl	%r8d, 112(%rsp)
	movl	%ebx, 592(%rsp)
.Ltmp9953:
	.loc	8 446 20
	jmp	.LBB36_72
.Ltmp9954:
	.loc	8 0 20 is_stmt 0
.Ltmp9955:
	.p2align	4
.LBB36_70:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp9956:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp9957:
	.loc	1 1411 0 is_stmt 1
	vmovss	%xmm12, 1324(%rsp)
.Ltmp9958:
	.loc	1 1411 0 is_stmt 0
	vmovss	%xmm14, 1416(%rsp)
	movq	120(%rsp), %rcx
.Ltmp9959:
.LBB36_71:
	.loc	1 0 0
	movq	144(%rsp), %rdx
	vxorps	%xmm5, %xmm5, %xmm5
	addq	$32, %rdi
	decq	%rdx
	movq	960(%rsp), %rax
.Ltmp9960:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rax
	subq	$-128, 1168(%rsp)
	subq	$-128, %rsi
	testq	%rdx, %rdx
	je	.LBB36_336
.LBB36_72:
	.loc	8 0 20 is_stmt 0
	movq	%rsi, 96(%rsp)
.Ltmp9961:
	.loc	4 2584 13 is_stmt 1
	cmpq	$1, %rax
	movq	%rax, 960(%rsp)
	movq	%rax, %rsi
	adcq	$0, %rsi
	cmpq	$32, %rsi
	movl	$32, %eax
	cmovaeq	%rax, %rsi
	movq	%rsi, 616(%rsp)
.Ltmp9962:
	.loc	1 1815 51
	movq	%rcx, %rsi
	subq	%rdi, %rsi
.Ltmp9963:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%rax, %rsi
.Ltmp9964:
	.loc	1 1820 35
	addq	%rdi, %rsi
.Ltmp9965:
	.loc	4 1050 16
	jb	.LBB36_489
	cmpq	%r15, %rsi
	ja	.LBB36_489
.Ltmp9966:
	.loc	4 0 16 is_stmt 0
	movq	%rdx, 144(%rsp)
.Ltmp9967:
	.loc	1 1759 23 is_stmt 1
	vmovss	1240(%rsp), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	1244(%rsp), %xmm7
	vmovss	1248(%rsp), %xmm12
	vmovss	1252(%rsp), %xmm13
	vmovss	1256(%rsp), %xmm15
	vmovss	1260(%rsp), %xmm3
	vmovss	1264(%rsp), %xmm8
	vmovss	1268(%rsp), %xmm11
	vmovss	1272(%rsp), %xmm9
	vmovss	1276(%rsp), %xmm14
	vmovss	1280(%rsp), %xmm10
.Ltmp9968:
	.loc	11 304 12
	cmpq	%rdi, %rcx
	jne	.LBB36_76
.Ltmp9969:
	.loc	1 0 0 is_stmt 0
	vmovss	1284(%rsp), %xmm4
	vmovss	48(%rsp), %xmm1
.Ltmp9970:
	.loc	11 304 12
	jmp	.LBB36_78
.Ltmp9971:
	.loc	11 0 12
.Ltmp9972:
	.p2align	4
.LBB36_76:
	vmovss	584(%r12), %xmm0
	vmovss	%xmm0, 624(%rsp)
	vmovss	588(%r12), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	592(%r12), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	596(%r12), %xmm0
	vmovss	%xmm0, 152(%rsp)
	vmovss	600(%r12), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	604(%r12), %xmm0
	vmovss	%xmm0, 352(%rsp)
	xorl	%eax, %eax
	vmovss	%xmm10, 16(%rsp)
	vmovss	%xmm14, 40(%rsp)
	vmovss	%xmm9, 368(%rsp)
	vmovss	%xmm11, 256(%rsp)
	vmovss	%xmm8, 176(%rsp)
	vmovaps	%xmm3, 704(%rsp)
	vmovss	%xmm15, 36(%rsp)
	vmovaps	%xmm13, %xmm2
	vmovaps	%xmm12, %xmm6
	vmovaps	%xmm7, %xmm0
	vmovss	608(%r12), %xmm1
	vmovss	%xmm1, 344(%rsp)
	vmovss	612(%r12), %xmm1
	vmovss	%xmm1, 232(%rsp)
	vmovss	616(%r12), %xmm1
	vmovss	%xmm1, 168(%rsp)
	vmovss	620(%r12), %xmm1
	vmovss	%xmm1, 224(%rsp)
	vmovss	624(%r12), %xmm1
	vmovss	%xmm1, 336(%rsp)
	vmovss	628(%r12), %xmm1
	vmovss	%xmm1, 328(%rsp)
	vmovss	632(%r12), %xmm1
	vmovss	%xmm1, 216(%rsp)
	vmovss	636(%r12), %xmm1
	vmovss	%xmm1, 320(%rsp)
	vmovss	640(%r12), %xmm1
	vmovss	%xmm1, 208(%rsp)
	vmovss	644(%r12), %xmm1
	vmovss	%xmm1, 60(%rsp)
	vmovss	648(%r12), %xmm1
	vmovss	%xmm1, 312(%rsp)
	vmovss	652(%r12), %xmm1
	vmovss	%xmm1, 304(%rsp)
	vmovss	656(%r12), %xmm1
	vmovss	%xmm1, 480(%rsp)
	vmovss	660(%r12), %xmm1
	vmovss	%xmm1, 472(%rsp)
	vmovss	664(%r12), %xmm1
	vmovss	%xmm1, 424(%rsp)
	vmovss	668(%r12), %xmm1
	vmovss	%xmm1, 416(%rsp)
	vmovss	672(%r12), %xmm1
	vmovss	%xmm1, 76(%rsp)
	vmovss	676(%r12), %xmm1
	vmovss	%xmm1, 464(%rsp)
	vmovss	680(%r12), %xmm1
	vmovss	%xmm1, 456(%rsp)
	vmovss	684(%r12), %xmm1
	vmovss	%xmm1, 296(%rsp)
	vmovss	688(%r12), %xmm1
	vmovss	%xmm1, 200(%rsp)
	vmovss	692(%r12), %xmm1
	vmovss	%xmm1, 408(%rsp)
	vmovss	696(%r12), %xmm1
	vmovss	%xmm1, 288(%rsp)
	vmovss	700(%r12), %xmm1
	vmovss	%xmm1, 400(%rsp)
	vmovss	704(%r12), %xmm1
	vmovss	%xmm1, 280(%rsp)
	vmovss	708(%r12), %xmm1
	vmovss	%xmm1, 4(%rsp)
	vmovss	712(%r12), %xmm1
	vmovss	%xmm1, 80(%rsp)
	vmovss	716(%r12), %xmm1
	vmovss	%xmm1, 392(%rsp)
	vmovss	720(%r12), %xmm1
	vmovss	%xmm1, 688(%rsp)
	vmovss	724(%r12), %xmm1
	vmovss	%xmm1, 680(%rsp)
	vmovss	728(%r12), %xmm1
	vmovss	%xmm1, 672(%rsp)
	vmovss	732(%r12), %xmm1
	vmovss	%xmm1, 32(%rsp)
	vmovss	736(%r12), %xmm1
	vmovss	%xmm1, 28(%rsp)
	vmovss	740(%r12), %xmm1
	vmovss	%xmm1, 24(%rsp)
	vmovss	744(%r12), %xmm1
	vmovss	%xmm1, 664(%rsp)
	vmovss	748(%r12), %xmm1
	vmovss	%xmm1, 656(%rsp)
	vmovss	752(%r12), %xmm1
	vmovss	%xmm1, 648(%rsp)
	vmovss	756(%r12), %xmm1
	vmovss	%xmm1, 640(%rsp)
	vmovss	760(%r12), %xmm1
	vmovss	%xmm1, 92(%rsp)
	vmovss	764(%r12), %xmm1
	vmovss	%xmm1, 448(%rsp)
	vmovss	768(%r12), %xmm1
	vmovss	%xmm1, 632(%rsp)
	vmovss	772(%r12), %xmm1
	vmovss	%xmm1, 728(%rsp)
	movq	1168(%rsp), %rdx
	movq	616(%rsp), %r9
	vmovss	48(%rsp), %xmm1
	.p2align	4
.LBB36_77:
.Ltmp9973:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rdx,%rax,4), %xmm7
.Ltmp9974:
	.loc	52 71 9
	vmulss	624(%rsp), %xmm7, %xmm3
.Ltmp9975:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp9976:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm7, %xmm4
.Ltmp9977:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
.Ltmp9978:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm7, %xmm8
	vmovss	%xmm7, 48(%rsp)
.Ltmp9979:
	.loc	52 61 9
	vaddss	%xmm5, %xmm8, %xmm8
	vmovss	%xmm1, 64(%rsp)
.Ltmp9980:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm1, %xmm9
.Ltmp9981:
	.loc	52 61 9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp9982:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm1, %xmm9
.Ltmp9983:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp9984:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm1, %xmm9
.Ltmp9985:
	.loc	52 61 9
	vaddss	%xmm9, %xmm8, %xmm8
.Ltmp9986:
	.loc	52 71 9
	vmulss	152(%rsp), %xmm7, %xmm9
.Ltmp9987:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm9
.Ltmp9988:
	.loc	52 71 9
	vmulss	232(%rsp), %xmm1, %xmm10
.Ltmp9989:
	.loc	52 61 9
	vaddss	%xmm10, %xmm9, %xmm9
	vmovaps	%xmm0, %xmm12
.Ltmp9990:
	.loc	52 71 9
	vmulss	168(%rsp), %xmm0, %xmm7
.Ltmp9991:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp9992:
	.loc	52 71 9
	vmulss	224(%rsp), %xmm0, %xmm7
.Ltmp9993:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp9994:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm0, %xmm7
.Ltmp9995:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
.Ltmp9996:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm0, %xmm8
.Ltmp9997:
	.loc	52 61 9
	vaddss	%xmm8, %xmm9, %xmm8
	vmovaps	%xmm6, %xmm13
.Ltmp9998:
	.loc	52 71 9
	vmulss	216(%rsp), %xmm6, %xmm6
.Ltmp9999:
	.loc	52 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp10000:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm13, %xmm6
.Ltmp10001:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
.Ltmp10002:
	.loc	52 71 9
	vmulss	208(%rsp), %xmm13, %xmm6
.Ltmp10003:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
.Ltmp10004:
	.loc	52 71 9
	vmulss	60(%rsp), %xmm13, %xmm7
.Ltmp10005:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
	vmovaps	%xmm2, %xmm15
.Ltmp10006:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm2, %xmm2
.Ltmp10007:
	.loc	52 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp10008:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm15, %xmm3
.Ltmp10009:
	.loc	52 61 9
	vaddss	%xmm3, %xmm4, %xmm4
.Ltmp10010:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm15, %xmm3
.Ltmp10011:
	.loc	52 61 9
	vaddss	%xmm3, %xmm6, %xmm6
.Ltmp10012:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm15, %xmm3
.Ltmp10013:
	.loc	52 61 9
	vaddss	%xmm3, %xmm7, %xmm7
	vmovss	36(%rsp), %xmm3
.Ltmp10014:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm3, %xmm0
.Ltmp10015:
	.loc	52 61 9
	vaddss	%xmm0, %xmm2, %xmm0
.Ltmp10016:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm3, %xmm2
.Ltmp10017:
	.loc	52 61 9
	vaddss	%xmm2, %xmm4, %xmm2
.Ltmp10018:
	.loc	52 71 9
	vmulss	76(%rsp), %xmm3, %xmm4
.Ltmp10019:
	.loc	52 61 9
	vaddss	%xmm4, %xmm6, %xmm4
.Ltmp10020:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm3, %xmm6
.Ltmp10021:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
	vmovaps	704(%rsp), %xmm1
.Ltmp10022:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm1, %xmm7
.Ltmp10023:
	.loc	52 61 9
	vaddss	%xmm7, %xmm0, %xmm0
.Ltmp10024:
	.loc	52 71 9
	vmulss	296(%rsp), %xmm1, %xmm7
.Ltmp10025:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp10026:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm1, %xmm7
.Ltmp10027:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp10028:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm1, %xmm7
.Ltmp10029:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm6
	vmovss	176(%rsp), %xmm11
.Ltmp10030:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm11, %xmm7
.Ltmp10031:
	.loc	52 61 9
	vaddss	%xmm7, %xmm0, %xmm0
.Ltmp10032:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm11, %xmm7
.Ltmp10033:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp10034:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm11, %xmm7
.Ltmp10035:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp10036:
	.loc	52 71 9
	vmulss	4(%rsp), %xmm11, %xmm7
.Ltmp10037:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm6
	vmovss	256(%rsp), %xmm8
.Ltmp10038:
	.loc	52 71 9
	vmulss	80(%rsp), %xmm8, %xmm7
.Ltmp10039:
	.loc	52 61 9
	vaddss	%xmm7, %xmm0, %xmm0
.Ltmp10040:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm8, %xmm7
.Ltmp10041:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp10042:
	.loc	52 71 9
	vmulss	688(%rsp), %xmm8, %xmm7
.Ltmp10043:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp10044:
	.loc	52 71 9
	vmulss	680(%rsp), %xmm8, %xmm7
.Ltmp10045:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm6
	vmovss	368(%rsp), %xmm14
.Ltmp10046:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm14, %xmm7
.Ltmp10047:
	.loc	52 61 9
	vaddss	%xmm7, %xmm0, %xmm0
.Ltmp10048:
	.loc	52 71 9
	vmulss	32(%rsp), %xmm14, %xmm7
.Ltmp10049:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp10050:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm14, %xmm7
.Ltmp10051:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp10052:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm14, %xmm7
.Ltmp10053:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm6
	vmovss	40(%rsp), %xmm10
.Ltmp10054:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm10, %xmm7
.Ltmp10055:
	.loc	52 61 9
	vaddss	%xmm7, %xmm0, %xmm0
.Ltmp10056:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm10, %xmm7
.Ltmp10057:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp10058:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm10, %xmm7
.Ltmp10059:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm7
.Ltmp10060:
	.loc	52 71 9
	vmulss	640(%rsp), %xmm10, %xmm4
.Ltmp10061:
	.loc	52 61 9
	vaddss	%xmm4, %xmm6, %xmm6
	vmovss	16(%rsp), %xmm4
.Ltmp10062:
	.loc	52 71 9
	vmulss	92(%rsp), %xmm4, %xmm5
.Ltmp10063:
	.loc	52 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp10064:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm4, %xmm5
.Ltmp10065:
	.loc	52 61 9
	vaddss	%xmm5, %xmm2, %xmm2
.Ltmp10066:
	.loc	52 71 9
	vmulss	632(%rsp), %xmm4, %xmm5
.Ltmp10067:
	.loc	52 61 9
	vaddss	%xmm5, %xmm7, %xmm5
.Ltmp10068:
	.loc	52 71 9
	vmulss	728(%rsp), %xmm4, %xmm7
.Ltmp10069:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm6
	vbroadcastss	.LCPI36_0(%rip), %xmm9
.Ltmp10070:
	.loc	52 103 24
	vandps	%xmm0, %xmm9, %xmm0
.Ltmp10071:
	.loc	52 103 24 is_stmt 0
	vandps	%xmm1, %xmm9, %xmm7
.Ltmp10072:
	.loc	52 161 24 is_stmt 1
	vmaxss	%xmm0, %xmm7, %xmm0
.Ltmp10073:
	.loc	52 103 24
	vandps	%xmm2, %xmm9, %xmm2
.Ltmp10074:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm0, %xmm0
.Ltmp10075:
	.loc	52 103 24
	vandps	%xmm5, %xmm9, %xmm2
	vxorps	%xmm5, %xmm5, %xmm5
.Ltmp10076:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm0, %xmm0
.Ltmp10077:
	.loc	52 103 24
	vandps	%xmm6, %xmm9, %xmm2
	vmovaps	%xmm8, %xmm9
	vmovaps	%xmm1, %xmm8
	vmovss	48(%rsp), %xmm1
.Ltmp10078:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm0, %xmm0
.Ltmp10079:
	.loc	52 56 9
	vmovss	%xmm0, 2648(%rsp,%rax,4)
.Ltmp10080:
	.loc	11 308 13
	incq	%rax
	vmovss	%xmm10, 16(%rsp)
	vmovss	%xmm14, 40(%rsp)
	vmovss	%xmm9, 368(%rsp)
	vmovss	%xmm11, 256(%rsp)
	vmovss	%xmm8, 176(%rsp)
	vmovaps	%xmm3, 704(%rsp)
	vmovss	%xmm15, 36(%rsp)
	vmovaps	%xmm13, %xmm2
	vmovaps	%xmm12, %xmm6
	vmovss	64(%rsp), %xmm0
	vmovaps	%xmm0, %xmm7
.Ltmp10081:
	.loc	11 304 12
	cmpq	%rax, %r9
	jne	.LBB36_77
.Ltmp10082:
.LBB36_78:
	.loc	1 1764 5
	vmovss	%xmm1, 1240(%rsp)
	vmovss	%xmm7, 1244(%rsp)
	vmovss	%xmm12, 1248(%rsp)
	vmovss	%xmm13, 1252(%rsp)
	vmovss	%xmm15, 1256(%rsp)
	vmovss	%xmm3, 1260(%rsp)
	vmovss	%xmm8, 1264(%rsp)
	vmovss	%xmm11, 1268(%rsp)
	vmovss	%xmm9, 1272(%rsp)
	vmovss	%xmm14, 1276(%rsp)
	vmovss	%xmm10, 1280(%rsp)
	vmovss	%xmm4, 1284(%rsp)
.Ltmp10083:
	.loc	5 438 16
	cmpq	136(%rsp), %rsi
	ja	.LBB36_583
.Ltmp10084:
	.loc	1 1759 23
	vmovss	1332(%rsp), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	1336(%rsp), %xmm6
	vmovss	1340(%rsp), %xmm12
	vmovss	1344(%rsp), %xmm13
	vmovss	1348(%rsp), %xmm14
	vmovss	1352(%rsp), %xmm2
	vmovss	1356(%rsp), %xmm7
	vmovss	1360(%rsp), %xmm11
	vmovss	1364(%rsp), %xmm8
	vmovss	1368(%rsp), %xmm15
	vmovss	1372(%rsp), %xmm10
.Ltmp10085:
	.loc	11 304 12
	cmpq	%rdi, %rcx
.Ltmp10086:
	.loc	11 304 12 is_stmt 0
	jne	.LBB36_81
.Ltmp10087:
	.loc	1 0 0
	vmovss	1376(%rsp), %xmm4
	movq	96(%rsp), %rsi
	vmovss	48(%rsp), %xmm1
.Ltmp10088:
	.loc	11 304 12
	jmp	.LBB36_83
.Ltmp10089:
	.loc	11 0 12
.Ltmp10090:
	.p2align	4
.LBB36_81:
	vmovss	584(%r12), %xmm0
	vmovss	%xmm0, 624(%rsp)
	vmovss	588(%r12), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	592(%r12), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	596(%r12), %xmm0
	vmovss	%xmm0, 152(%rsp)
	vmovss	600(%r12), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	604(%r12), %xmm0
	vmovss	%xmm0, 352(%rsp)
	xorl	%eax, %eax
	vmovss	%xmm10, 16(%rsp)
	vmovss	%xmm15, 40(%rsp)
	vmovss	%xmm8, 368(%rsp)
	vmovss	%xmm11, 256(%rsp)
	vmovss	%xmm7, 176(%rsp)
	vmovaps	%xmm2, 704(%rsp)
	vmovss	%xmm14, 36(%rsp)
	vmovaps	%xmm13, %xmm3
	vmovaps	%xmm12, %xmm0
	vmovaps	%xmm6, %xmm5
	vmovss	608(%r12), %xmm1
	vmovss	%xmm1, 344(%rsp)
	vmovss	612(%r12), %xmm1
	vmovss	%xmm1, 232(%rsp)
	vmovss	616(%r12), %xmm1
	vmovss	%xmm1, 168(%rsp)
	vmovss	620(%r12), %xmm1
	vmovss	%xmm1, 224(%rsp)
	vmovss	624(%r12), %xmm1
	vmovss	%xmm1, 336(%rsp)
	vmovss	628(%r12), %xmm1
	vmovss	%xmm1, 328(%rsp)
	vmovss	632(%r12), %xmm1
	vmovss	%xmm1, 216(%rsp)
	vmovss	636(%r12), %xmm1
	vmovss	%xmm1, 320(%rsp)
	vmovss	640(%r12), %xmm1
	vmovss	%xmm1, 208(%rsp)
	vmovss	644(%r12), %xmm1
	vmovss	%xmm1, 60(%rsp)
	vmovss	648(%r12), %xmm1
	vmovss	%xmm1, 312(%rsp)
	vmovss	652(%r12), %xmm1
	vmovss	%xmm1, 304(%rsp)
	vmovss	656(%r12), %xmm1
	vmovss	%xmm1, 480(%rsp)
	vmovss	660(%r12), %xmm1
	vmovss	%xmm1, 472(%rsp)
	vmovss	664(%r12), %xmm1
	vmovss	%xmm1, 424(%rsp)
	vmovss	668(%r12), %xmm1
	vmovss	%xmm1, 416(%rsp)
	vmovss	672(%r12), %xmm1
	vmovss	%xmm1, 76(%rsp)
	vmovss	676(%r12), %xmm1
	vmovss	%xmm1, 464(%rsp)
	vmovss	680(%r12), %xmm1
	vmovss	%xmm1, 456(%rsp)
	vmovss	684(%r12), %xmm1
	vmovss	%xmm1, 296(%rsp)
	vmovss	688(%r12), %xmm1
	vmovss	%xmm1, 200(%rsp)
	vmovss	692(%r12), %xmm1
	vmovss	%xmm1, 408(%rsp)
	vmovss	696(%r12), %xmm1
	vmovss	%xmm1, 288(%rsp)
	vmovss	700(%r12), %xmm1
	vmovss	%xmm1, 400(%rsp)
	vmovss	704(%r12), %xmm1
	vmovss	%xmm1, 280(%rsp)
	vmovss	708(%r12), %xmm1
	vmovss	%xmm1, 4(%rsp)
	vmovss	712(%r12), %xmm1
	vmovss	%xmm1, 80(%rsp)
	vmovss	716(%r12), %xmm1
	vmovss	%xmm1, 392(%rsp)
	vmovss	720(%r12), %xmm1
	vmovss	%xmm1, 688(%rsp)
	vmovss	724(%r12), %xmm1
	vmovss	%xmm1, 680(%rsp)
	vmovss	728(%r12), %xmm1
	vmovss	%xmm1, 672(%rsp)
	vmovss	732(%r12), %xmm1
	vmovss	%xmm1, 32(%rsp)
	vmovss	736(%r12), %xmm1
	vmovss	%xmm1, 28(%rsp)
	vmovss	740(%r12), %xmm1
	vmovss	%xmm1, 24(%rsp)
	vmovss	744(%r12), %xmm1
	vmovss	%xmm1, 664(%rsp)
	vmovss	748(%r12), %xmm1
	vmovss	%xmm1, 656(%rsp)
	vmovss	752(%r12), %xmm1
	vmovss	%xmm1, 648(%rsp)
	vmovss	756(%r12), %xmm1
	vmovss	%xmm1, 640(%rsp)
	vmovss	760(%r12), %xmm1
	vmovss	%xmm1, 92(%rsp)
	vmovss	764(%r12), %xmm1
	vmovss	%xmm1, 448(%rsp)
	vmovss	768(%r12), %xmm1
	vmovss	%xmm1, 632(%rsp)
	vmovss	772(%r12), %xmm1
	vmovss	%xmm1, 728(%rsp)
	movq	96(%rsp), %rsi
	movq	616(%rsp), %rdx
	vmovss	48(%rsp), %xmm1
	vxorps	%xmm9, %xmm9, %xmm9
	.p2align	4
.LBB36_82:
	vmovss	%xmm1, 64(%rsp)
.Ltmp10091:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rsi,%rax,4), %xmm6
.Ltmp10092:
	.loc	52 71 9
	vmulss	624(%rsp), %xmm6, %xmm2
.Ltmp10093:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10094:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm6, %xmm4
.Ltmp10095:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10096:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm6, %xmm7
	vmovss	%xmm6, 48(%rsp)
.Ltmp10097:
	.loc	52 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp10098:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm1, %xmm8
.Ltmp10099:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp10100:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm1, %xmm8
.Ltmp10101:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp10102:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm1, %xmm8
.Ltmp10103:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
.Ltmp10104:
	.loc	52 71 9
	vmulss	152(%rsp), %xmm6, %xmm8
.Ltmp10105:
	.loc	52 61 9
	vaddss	%xmm9, %xmm8, %xmm8
.Ltmp10106:
	.loc	52 71 9
	vmulss	232(%rsp), %xmm1, %xmm10
.Ltmp10107:
	.loc	52 61 9
	vaddss	%xmm10, %xmm8, %xmm8
	vmovaps	%xmm5, %xmm12
.Ltmp10108:
	.loc	52 71 9
	vmulss	168(%rsp), %xmm5, %xmm6
.Ltmp10109:
	.loc	52 61 9
	vaddss	%xmm6, %xmm2, %xmm2
.Ltmp10110:
	.loc	52 71 9
	vmulss	224(%rsp), %xmm5, %xmm6
.Ltmp10111:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
.Ltmp10112:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm5, %xmm6
.Ltmp10113:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
.Ltmp10114:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm5, %xmm7
.Ltmp10115:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
	vmovaps	%xmm0, %xmm13
.Ltmp10116:
	.loc	52 71 9
	vmulss	216(%rsp), %xmm0, %xmm8
.Ltmp10117:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp10118:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm0, %xmm8
.Ltmp10119:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp10120:
	.loc	52 71 9
	vmulss	208(%rsp), %xmm0, %xmm8
.Ltmp10121:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp10122:
	.loc	52 71 9
	vmulss	60(%rsp), %xmm0, %xmm8
.Ltmp10123:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
	vmovaps	%xmm3, %xmm14
.Ltmp10124:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm3, %xmm3
.Ltmp10125:
	.loc	52 61 9
	vaddss	%xmm3, %xmm2, %xmm3
.Ltmp10126:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm14, %xmm2
.Ltmp10127:
	.loc	52 61 9
	vaddss	%xmm2, %xmm4, %xmm4
.Ltmp10128:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm14, %xmm2
.Ltmp10129:
	.loc	52 61 9
	vaddss	%xmm2, %xmm6, %xmm6
.Ltmp10130:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm14, %xmm2
.Ltmp10131:
	.loc	52 61 9
	vaddss	%xmm2, %xmm7, %xmm7
	vmovss	36(%rsp), %xmm2
.Ltmp10132:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm2, %xmm0
.Ltmp10133:
	.loc	52 61 9
	vaddss	%xmm0, %xmm3, %xmm0
.Ltmp10134:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm2, %xmm3
.Ltmp10135:
	.loc	52 61 9
	vaddss	%xmm3, %xmm4, %xmm3
.Ltmp10136:
	.loc	52 71 9
	vmulss	76(%rsp), %xmm2, %xmm4
.Ltmp10137:
	.loc	52 61 9
	vaddss	%xmm4, %xmm6, %xmm4
.Ltmp10138:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm2, %xmm6
.Ltmp10139:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
	vmovaps	704(%rsp), %xmm1
.Ltmp10140:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm1, %xmm8
.Ltmp10141:
	.loc	52 61 9
	vaddss	%xmm0, %xmm8, %xmm0
.Ltmp10142:
	.loc	52 71 9
	vmulss	296(%rsp), %xmm1, %xmm8
.Ltmp10143:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp10144:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm1, %xmm8
.Ltmp10145:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp10146:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm1, %xmm8
.Ltmp10147:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovss	176(%rsp), %xmm11
.Ltmp10148:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm11, %xmm8
.Ltmp10149:
	.loc	52 61 9
	vaddss	%xmm0, %xmm8, %xmm0
.Ltmp10150:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm11, %xmm8
.Ltmp10151:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp10152:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm11, %xmm8
.Ltmp10153:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp10154:
	.loc	52 71 9
	vmulss	4(%rsp), %xmm11, %xmm8
.Ltmp10155:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovss	256(%rsp), %xmm7
.Ltmp10156:
	.loc	52 71 9
	vmulss	80(%rsp), %xmm7, %xmm9
.Ltmp10157:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10158:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm7, %xmm9
.Ltmp10159:
	.loc	52 61 9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp10160:
	.loc	52 71 9
	vmulss	688(%rsp), %xmm7, %xmm9
.Ltmp10161:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10162:
	.loc	52 71 9
	vmulss	680(%rsp), %xmm7, %xmm9
.Ltmp10163:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovss	368(%rsp), %xmm15
.Ltmp10164:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm15, %xmm9
.Ltmp10165:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10166:
	.loc	52 71 9
	vmulss	32(%rsp), %xmm15, %xmm9
.Ltmp10167:
	.loc	52 61 9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp10168:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm15, %xmm9
.Ltmp10169:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10170:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm15, %xmm9
.Ltmp10171:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovss	40(%rsp), %xmm10
.Ltmp10172:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm10, %xmm9
.Ltmp10173:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10174:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm10, %xmm9
.Ltmp10175:
	.loc	52 61 9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp10176:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm10, %xmm9
.Ltmp10177:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm9
.Ltmp10178:
	.loc	52 71 9
	vmulss	640(%rsp), %xmm10, %xmm4
.Ltmp10179:
	.loc	52 61 9
	vaddss	%xmm4, %xmm6, %xmm6
	vmovss	16(%rsp), %xmm4
.Ltmp10180:
	.loc	52 71 9
	vmulss	92(%rsp), %xmm4, %xmm5
.Ltmp10181:
	.loc	52 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp10182:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm4, %xmm5
.Ltmp10183:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp10184:
	.loc	52 71 9
	vmulss	632(%rsp), %xmm4, %xmm5
.Ltmp10185:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10186:
	.loc	52 71 9
	vmulss	728(%rsp), %xmm4, %xmm9
.Ltmp10187:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vbroadcastss	.LCPI36_0(%rip), %xmm8
.Ltmp10188:
	.loc	52 103 24
	vandps	%xmm0, %xmm8, %xmm0
.Ltmp10189:
	.loc	52 103 24 is_stmt 0
	vandps	%xmm1, %xmm8, %xmm9
.Ltmp10190:
	.loc	52 161 24 is_stmt 1
	vmaxss	%xmm0, %xmm9, %xmm0
.Ltmp10191:
	.loc	52 103 24
	vandps	%xmm3, %xmm8, %xmm3
.Ltmp10192:
	.loc	52 161 24
	vmaxss	%xmm3, %xmm0, %xmm0
.Ltmp10193:
	.loc	52 103 24
	vandps	%xmm5, %xmm8, %xmm3
	vxorps	%xmm9, %xmm9, %xmm9
.Ltmp10194:
	.loc	52 161 24
	vmaxss	%xmm3, %xmm0, %xmm0
.Ltmp10195:
	.loc	52 103 24
	vandps	%xmm6, %xmm8, %xmm3
	vmovaps	%xmm7, %xmm8
	vmovaps	%xmm1, %xmm7
	vmovss	48(%rsp), %xmm1
.Ltmp10196:
	.loc	52 161 24
	vmaxss	%xmm3, %xmm0, %xmm0
.Ltmp10197:
	.loc	52 56 9
	vmovss	%xmm0, 1616(%rsp,%rax,4)
.Ltmp10198:
	.loc	11 308 13
	incq	%rax
	vmovss	%xmm10, 16(%rsp)
	vmovss	%xmm15, 40(%rsp)
	vmovss	%xmm8, 368(%rsp)
	vmovss	%xmm11, 256(%rsp)
	vmovss	%xmm7, 176(%rsp)
	vmovaps	%xmm2, 704(%rsp)
	vmovss	%xmm14, 36(%rsp)
	vmovaps	%xmm13, %xmm3
	vmovaps	%xmm12, %xmm0
	vmovss	64(%rsp), %xmm5
	vmovaps	%xmm5, %xmm6
.Ltmp10199:
	.loc	11 304 12
	cmpq	%rax, %rdx
	jne	.LBB36_82
.Ltmp10200:
.LBB36_83:
	.loc	1 1764 5
	vmovss	%xmm1, 1332(%rsp)
	vmovss	%xmm6, 1336(%rsp)
	vmovss	%xmm12, 1340(%rsp)
	vmovss	%xmm13, 1344(%rsp)
	vmovss	%xmm14, 1348(%rsp)
	vmovss	%xmm2, 1352(%rsp)
	vmovss	%xmm7, 1356(%rsp)
	vmovss	%xmm11, 1360(%rsp)
	vmovss	%xmm8, 1364(%rsp)
	vmovss	%xmm15, 1368(%rsp)
	vmovss	%xmm10, 1372(%rsp)
	vmovss	%xmm4, 1376(%rsp)
.Ltmp10201:
	.loc	11 304 12
	cmpq	%rdi, %rcx
.Ltmp10202:
	.loc	3 900 12
	je	.LBB36_71
.Ltmp10203:
	.loc	3 0 12 is_stmt 0
	vmovss	1288(%rsp), %xmm8
	vmovss	1304(%rsp), %xmm9
	vmovss	1380(%rsp), %xmm10
	vmovss	1396(%rsp), %xmm0
	vmovss	%xmm0, 624(%rsp)
	movq	544(%r12), %rcx
	movq	552(%r12), %rax
	movq	%rax, 248(%rsp)
	vmovss	1328(%rsp), %xmm13
	vmovss	1420(%rsp), %xmm15
	vmovss	1324(%rsp), %xmm12
	vmovss	1320(%rsp), %xmm7
	vmovss	1416(%rsp), %xmm14
	vmovss	1412(%rsp), %xmm6
	xorl	%eax, %eax
	vmovss	.LCPI36_2(%rip), %xmm5
	movq	%rdi, 104(%rsp)
	.p2align	4
.LBB36_85:
.Ltmp10204:
	.loc	52 51 9 is_stmt 1
	vmovss	2648(%rsp,%rax,4), %xmm0
.Ltmp10205:
	.loc	52 51 9 is_stmt 0
	vmovss	1616(%rsp,%rax,4), %xmm1
.Ltmp10206:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
	movq	%rax, 64(%rsp)
.Ltmp10207:
	.loc	1 1832 24
	leaq	(%rax,%rdi), %rdx
.Ltmp10208:
	.loc	41 1244 18
	vmovd	%xmm1, %r9d
.Ltmp10209:
	.loc	41 1244 18 is_stmt 0
	vmovd	%xmm0, %eax
.Ltmp10210:
	.loc	52 161 24 is_stmt 1
	movl	%r9d, %r10d
	cmovbel	%eax, %r10d
.Ltmp10211:
	.loc	5 568 12
	cmpq	%r15, %rdx
	ja	.LBB36_449
.Ltmp10212:
	.loc	52 51 9
	je	.LBB36_614
.Ltmp10213:
	.loc	1 0 0 is_stmt 0
	andl	364(%rsp), %r10d
	andl	%r8d, %eax
	orl	%r10d, %eax
	vmovd	%eax, %xmm1
.Ltmp10214:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm8, %xmm1
	vmovaps	%xmm5, %xmm0
.Ltmp10215:
	.loc	52 161 24
	jbe	.LBB36_89
.Ltmp10216:
	.loc	52 76 9
	vdivss	%xmm1, %xmm8, %xmm0
.Ltmp10217:
.LBB36_89:
	.loc	1 0 0 is_stmt 0
	movq	328(%r12), %r11
.Ltmp10218:
	.loc	1 1392 25 is_stmt 1
	movq	176(%r12), %rsi
	movq	%r11, 176(%rsp)
	.loc	1 1392 45 is_stmt 0
	imulq	8(%rsp), %r11
.Ltmp10219:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %r11
	ja	.LBB36_450
.Ltmp10220:
	.loc	5 0 12 is_stmt 0
	movl	%r10d, 36(%rsp)
	movl	%r9d, 704(%rsp)
.Ltmp10221:
	.loc	52 56 9 is_stmt 1
	je	.LBB36_611
.Ltmp10222:
	.loc	52 0 9 is_stmt 0
	movq	%rdx, 368(%rsp)
	vmovss	(%r13,%rdx,4), %xmm3
.Ltmp10223:
	.loc	1 1392 0 is_stmt 1
	movq	168(%r12), %rax
	movq	%r11, 256(%rsp)
.Ltmp10224:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%r11,4)
.Ltmp10225:
	.loc	1 1259 17
	movq	328(%r12), %rbx
.Ltmp10226:
	.loc	13 37 12
	testq	%rbx, %rbx
	je	.LBB36_110
.Ltmp10227:
	.loc	13 0 12 is_stmt 0
	movq	696(%rsp), %rdi
	movq	320(%rdi), %rax
	movq	%rax, 48(%rsp)
	movq	8(%rsp), %rax
	leaq	1(%rax), %r9
	cmpq	%rcx, %r9
	movq	%rcx, %rdx
	movl	$0, %eax
	cmovbq	%rax, %rdx
	movq	312(%rdi), %rax
	subq	%rdx, %r9
	movq	168(%rdi), %r13
	movq	176(%rdi), %rsi
	movq	256(%rdi), %rdx
	movq	%rdx, 16(%rsp)
	movq	248(%rdi), %rbp
	movq	224(%rdi), %r10
	movq	216(%rdi), %r8
	imulq	%rbx, %r9
	movq	%r9, 40(%rsp)
	movq	%rbx, %rdi
	xorl	%r12d, %r12d
	xorl	%edx, %edx
	jmp	.LBB36_95
	.p2align	4
.LBB36_93:
	xorl	%r11d, %r11d
.LBB36_94:
	decq	%rdi
	addq	$4, %rdx
.Ltmp10228:
	movl	%r11d, (%rbp,%r12,4)
.Ltmp10229:
	incq	%r12
.Ltmp10230:
	.loc	13 37 12 is_stmt 1
	testq	%rdi, %rdi
	je	.LBB36_110
.LBB36_95:
.Ltmp10231:
	.loc	7 1714 9
	cmpq	$32, %rdx
.Ltmp10232:
	.loc	6 180 28
	je	.LBB36_110
.Ltmp10233:
	.loc	1 1263 21
	cmpq	48(%rsp), %r12
	je	.LBB36_597
	leaq	(%r12,%r12,2), %r9
	movl	4(%rax,%r9,4), %r14d
.Ltmp10234:
	.loc	1 1265 23
	addq	8(%rsp), %r14
.Ltmp10235:
	.loc	1 1266 12
	cmpq	%rcx, %r14
	movl	$0, %r11d
	cmovaeq	%rcx, %r11
	subq	%r11, %r14
.Ltmp10236:
	.loc	1 1273 42
	movq	%r14, %r15
	imulq	%rbx, %r15
	addq	%r12, %r15
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB36_601
.Ltmp10237:
	.loc	1 1274 24 is_stmt 1
	cmpq	16(%rsp), %r12
	je	.LBB36_603
.Ltmp10238:
	.loc	1 0 0 is_stmt 0
	movl	(%rax,%r9,4), %r9d
.Ltmp10239:
	vmovss	(%r13,%r15,4), %xmm0
.Ltmp10240:
	.loc	1 1274 24
	movl	(%rbp,%r12,4), %r11d
	testq	%r11, %r11
.Ltmp10241:
	.loc	1 1275 26 is_stmt 1
	je	.LBB36_103
	.loc	1 1278 24
	cmpq	%r10, %r12
	jae	.LBB36_607
	vmovss	(%r8,%r12,4), %xmm1
.Ltmp10242:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB36_103
.Ltmp10243:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB36_103:
.Ltmp10244:
	.loc	1 1280 9 is_stmt 1
	cmpq	%r10, %r12
	je	.LBB36_599
	vmovss	%xmm0, (%r8,%r12,4)
	.loc	1 1281 24
	incq	%r11
	cmpq	%r9, %r11
.Ltmp10245:
	.loc	1 1282 23
	jne	.LBB36_108
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 848(%rsp,%rdx)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%r13,%r15,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp10246:
	.p2align	4
.LBB36_106:
.Ltmp10247:
	.loc	1 1291 65 is_stmt 1
	movq	%r14, %r15
	imulq	%rbx, %r15
	addq	%r12, %r15
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB36_464
.Ltmp10248:
	.loc	1 903 8 is_stmt 1
	vminss	(%r13,%r15,4), %xmm0, %xmm0
.Ltmp10249:
	.loc	1 1292 17
	vmovss	%xmm0, (%r13,%r15,4)
	.loc	1 1293 20
	testq	%r14, %r14
	cmoveq	%rcx, %r14
	.loc	1 1296 17
	decq	%r14
.Ltmp10250:
	.loc	10 1916 50
	decq	%r9
.Ltmp10251:
	.loc	3 900 12
	jne	.LBB36_106
	jmp	.LBB36_93
.Ltmp10252:
	.loc	3 0 12 is_stmt 0
.Ltmp10253:
	.p2align	4
.LBB36_108:
	movq	40(%rsp), %r9
	.loc	1 1285 44 is_stmt 1
	leaq	(%r12,%r9), %r15
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB36_609
	vmovss	(%r13,%r15,4), %xmm1
.Ltmp10254:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp10255:
	.loc	1 1282 9
	vmovss	%xmm0, 848(%rsp,%rdx)
	jmp	.LBB36_94
.Ltmp10256:
	.loc	1 0 9 is_stmt 0
.Ltmp10257:
	.p2align	4
.LBB36_110:
	.loc	52 51 9 is_stmt 1
	vmovss	848(%rsp), %xmm1
.Ltmp10258:
	.loc	52 71 9
	vmulss	.LCPI36_3(%rip), %xmm1, %xmm0
.Ltmp10259:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp10260:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm2
	movq	696(%rsp), %r12
.Ltmp10261:
	.loc	1 1412 26
	movq	192(%r12), %rsi
	movq	176(%rsp), %rdi
.Ltmp10262:
	.loc	13 37 12
	testq	%rdi, %rdi
	movl	592(%rsp), %ebx
	movq	256(%rsp), %r8
	je	.LBB36_135
.Ltmp10263:
	.loc	13 0 12 is_stmt 0
	movq	320(%r12), %r11
.Ltmp10264:
	.loc	1 1403 42 is_stmt 1
	testq	%r11, %r11
	je	.LBB36_643
	.loc	1 0 42 is_stmt 0
	movq	312(%r12), %r15
	.loc	1 1403 42
	movl	8(%r15), %r9d
	.loc	1 1403 28
	addq	8(%rsp), %r9
.Ltmp10265:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %r9
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r9
	.loc	1 1407 40
	imulq	%rdi, %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	.loc	1 0 25
	movq	184(%r12), %r10
	.loc	1 1407 25
	vmovss	(%r10,%r9,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 848(%rsp)
.Ltmp10266:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB36_135
.Ltmp10267:
	.loc	1 1403 42
	cmpq	$1, %r11
	je	.LBB36_627
	movl	20(%r15), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10268:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	4(%r10,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 852(%rsp)
.Ltmp10269:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB36_135
.Ltmp10270:
	.loc	1 1403 42
	cmpq	$2, %r11
	je	.LBB36_630
	movl	32(%r15), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10271:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	8(%r10,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 856(%rsp)
.Ltmp10272:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB36_135
.Ltmp10273:
	.loc	1 1403 42
	cmpq	$3, %r11
	je	.LBB36_633
	movl	44(%r15), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10274:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	12(%r10,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 860(%rsp)
.Ltmp10275:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB36_135
.Ltmp10276:
	.loc	1 1403 42
	cmpq	$4, %r11
	je	.LBB36_635
	movl	56(%r15), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10277:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	16(%r10,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 864(%rsp)
.Ltmp10278:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB36_135
.Ltmp10279:
	.loc	1 1403 42
	cmpq	$5, %r11
	je	.LBB36_637
	movl	68(%r15), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10280:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	20(%r10,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 868(%rsp)
.Ltmp10281:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB36_135
.Ltmp10282:
	.loc	1 1403 42
	cmpq	$6, %r11
	je	.LBB36_639
	movl	80(%r15), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10283:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	24(%r10,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 872(%rsp)
.Ltmp10284:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB36_135
.Ltmp10285:
	.loc	1 1403 42
	cmpq	$7, %r11
	je	.LBB36_641
	movl	92(%r15), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10286:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	28(%r10,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 876(%rsp)
.Ltmp10287:
	.loc	1 0 13
.Ltmp10288:
	.p2align	4
.LBB36_135:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %r8
	ja	.LBB36_451
.Ltmp10289:
	.loc	5 0 12 is_stmt 0
	movq	840(%rsp), %rbp
	movl	112(%rsp), %r8d
.Ltmp10290:
	.loc	52 56 9 is_stmt 1
	je	.LBB36_611
.Ltmp10291:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm2, %xmm12, %xmm0
	vsubss	%xmm1, %xmm0, %xmm12
	.loc	1 1412 0 is_stmt 1
	movq	184(%r12), %rax
	movq	256(%rsp), %rdx
.Ltmp10292:
	.loc	52 56 9
	vmovss	%xmm2, (%rax,%rdx,4)
.Ltmp10293:
	.loc	52 76 9
	vdivss	%xmm13, %xmm12, %xmm0
.Ltmp10294:
	.loc	52 66 9
	vsubss	%xmm0, %xmm5, %xmm0
.Ltmp10295:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm7, %xmm0, %xmm2
.Ltmp10296:
	.loc	52 92 9 is_stmt 1
	vmulss	%xmm2, %xmm9, %xmm2
	vaddss	%xmm2, %xmm7, %xmm2
.Ltmp10297:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm0, %xmm0
.Ltmp10298:
	.loc	52 103 24
	vbroadcastss	.LCPI36_0(%rip), %xmm2
	vandps	%xmm2, %xmm0, %xmm4
.Ltmp10299:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm4, %xmm4
	vandps	%xmm0, %xmm4, %xmm7
.Ltmp10300:
	.loc	1 1420 28
	movq	160(%r12), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	432(%rsp), %rdi
.Ltmp10301:
	.loc	5 568 12 is_stmt 1
	cmpq	%rsi, %rdi
	movq	976(%rsp), %r13
	movq	368(%rsp), %r10
	ja	.LBB36_454
.Ltmp10302:
	.loc	52 51 9
	je	.LBB36_614
.Ltmp10303:
	.loc	1 0 0 is_stmt 0
	vsubss	%xmm7, %xmm5, %xmm0
.Ltmp10304:
	.loc	1 1420 0 is_stmt 1
	movq	152(%r12), %rax
.Ltmp10305:
	.loc	52 51 9
	vmovss	(%rax,%rdi,4), %xmm4
.Ltmp10306:
	.loc	52 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp10307:
	.loc	52 71 9
	vmulss	%xmm4, %xmm0, %xmm0
.Ltmp10308:
	.loc	41 1244 18
	vmovd	%xmm4, %eax
.Ltmp10309:
	.loc	52 161 24
	andl	160(%rsp), %eax
.Ltmp10310:
	.loc	41 1244 18
	vmovd	%xmm0, %edx
.Ltmp10311:
	.loc	52 161 44
	andl	%ebx, %edx
	.loc	52 161 24 is_stmt 0
	orl	%eax, %edx
.Ltmp10312:
	.loc	52 56 9 is_stmt 1
	movl	%edx, (%r13,%r10,4)
	movq	136(%rsp), %rdx
.Ltmp10313:
	.loc	5 568 12
	cmpq	%rdx, %r10
	ja	.LBB36_456
.Ltmp10314:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_614
	movl	704(%rsp), %eax
	andl	%r8d, %eax
	movl	36(%rsp), %edx
	orl	%eax, %edx
	vmovd	%edx, %xmm3
.Ltmp10315:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm3
	vmovaps	%xmm5, %xmm0
.Ltmp10316:
	.loc	52 161 24
	jbe	.LBB36_143
.Ltmp10317:
	.loc	52 76 9
	vdivss	%xmm3, %xmm10, %xmm0
.Ltmp10318:
.LBB36_143:
	.loc	1 0 0 is_stmt 0
	movq	528(%r12), %r14
.Ltmp10319:
	.loc	1 1392 25 is_stmt 1
	movq	376(%r12), %rsi
	.loc	1 1392 45 is_stmt 0
	movq	%r14, %rdi
	imulq	8(%rsp), %rdi
.Ltmp10320:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %rdi
	ja	.LBB36_457
.Ltmp10321:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_611
	vmovss	(%rbp,%r10,4), %xmm3
.Ltmp10322:
	.loc	1 1392 0 is_stmt 1
	movq	368(%r12), %rax
	movq	%rdi, 256(%rsp)
.Ltmp10323:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp10324:
	.loc	1 1259 17
	movq	528(%r12), %rdi
.Ltmp10325:
	.loc	13 37 12
	testq	%rdi, %rdi
	je	.LBB36_165
.Ltmp10326:
	.loc	13 0 12 is_stmt 0
	movq	%r14, 176(%rsp)
	movq	520(%r12), %rax
	movq	%rax, 48(%rsp)
	movq	8(%rsp), %rax
	leaq	1(%rax), %r8
	cmpq	%rcx, %r8
	movq	%rcx, %rdx
	movl	$0, %eax
	cmovbq	%rax, %rdx
	movq	512(%r12), %rax
	subq	%rdx, %r8
	movq	368(%r12), %r14
	movq	376(%r12), %rsi
	movq	456(%r12), %rdx
	movq	%rdx, 16(%rsp)
	movq	448(%r12), %r13
	movq	424(%r12), %r10
	movq	416(%r12), %rbp
	imulq	%rdi, %r8
	movq	%r8, 40(%rsp)
	movq	%rdi, %r8
	xorl	%r12d, %r12d
	xorl	%edx, %edx
	jmp	.LBB36_149
	.p2align	4
.LBB36_147:
	xorl	%r11d, %r11d
.LBB36_148:
	decq	%r8
	addq	$4, %rdx
.Ltmp10327:
	movl	%r11d, (%r13,%r12,4)
.Ltmp10328:
	incq	%r12
.Ltmp10329:
	.loc	13 37 12 is_stmt 1
	testq	%r8, %r8
	je	.LBB36_164
.LBB36_149:
.Ltmp10330:
	.loc	7 1714 9
	cmpq	$32, %rdx
.Ltmp10331:
	.loc	6 180 28
	je	.LBB36_164
.Ltmp10332:
	.loc	1 1263 21
	cmpq	48(%rsp), %r12
	je	.LBB36_597
	leaq	(%r12,%r12,2), %r9
	movl	4(%rax,%r9,4), %ebx
.Ltmp10333:
	.loc	1 1265 23
	addq	8(%rsp), %rbx
.Ltmp10334:
	.loc	1 1266 12
	cmpq	%rcx, %rbx
	movl	$0, %r11d
	cmovaeq	%rcx, %r11
	subq	%r11, %rbx
.Ltmp10335:
	.loc	1 1273 42
	movq	%rbx, %r15
	imulq	%rdi, %r15
	addq	%r12, %r15
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB36_601
.Ltmp10336:
	.loc	1 1274 24 is_stmt 1
	cmpq	16(%rsp), %r12
	je	.LBB36_603
.Ltmp10337:
	.loc	1 0 0 is_stmt 0
	movl	(%rax,%r9,4), %r9d
.Ltmp10338:
	vmovss	(%r14,%r15,4), %xmm0
.Ltmp10339:
	.loc	1 1274 24
	movl	(%r13,%r12,4), %r11d
	testq	%r11, %r11
	je	.LBB36_157
.Ltmp10340:
	.loc	1 1278 24 is_stmt 1
	cmpq	%r10, %r12
	jae	.LBB36_607
	vmovss	(%rbp,%r12,4), %xmm1
.Ltmp10341:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB36_157
.Ltmp10342:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB36_157:
.Ltmp10343:
	.loc	1 1280 9 is_stmt 1
	cmpq	%r10, %r12
	je	.LBB36_599
	vmovss	%xmm0, (%rbp,%r12,4)
	.loc	1 1281 24
	incq	%r11
	cmpq	%r9, %r11
.Ltmp10344:
	.loc	1 1282 23
	jne	.LBB36_162
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 848(%rsp,%rdx)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%r14,%r15,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp10345:
	.p2align	4
.LBB36_160:
.Ltmp10346:
	.loc	1 1291 65 is_stmt 1
	movq	%rbx, %r15
	imulq	%rdi, %r15
	addq	%r12, %r15
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB36_464
.Ltmp10347:
	.loc	1 903 8 is_stmt 1
	vminss	(%r14,%r15,4), %xmm0, %xmm0
.Ltmp10348:
	.loc	1 1292 17
	vmovss	%xmm0, (%r14,%r15,4)
	.loc	1 1293 20
	testq	%rbx, %rbx
	cmoveq	%rcx, %rbx
	.loc	1 1296 17
	decq	%rbx
.Ltmp10349:
	.loc	10 1916 50
	decq	%r9
.Ltmp10350:
	.loc	3 900 12
	jne	.LBB36_160
	jmp	.LBB36_147
.Ltmp10351:
	.loc	3 0 12 is_stmt 0
.Ltmp10352:
	.p2align	4
.LBB36_162:
	movq	40(%rsp), %r9
	.loc	1 1285 44 is_stmt 1
	leaq	(%r12,%r9), %r15
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB36_609
	vmovss	(%r14,%r15,4), %xmm1
.Ltmp10353:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp10354:
	.loc	1 1282 9
	vmovss	%xmm0, 848(%rsp,%rdx)
	jmp	.LBB36_148
.Ltmp10355:
	.loc	1 0 9 is_stmt 0
.Ltmp10356:
	.p2align	4
.LBB36_164:
	.loc	52 51 9 is_stmt 1
	vmovss	848(%rsp), %xmm1
	movq	696(%rsp), %r12
	movq	976(%rsp), %r13
	movq	840(%rsp), %rbp
	movl	112(%rsp), %r8d
	movq	368(%rsp), %r10
	movq	176(%rsp), %r14
.Ltmp10357:
.LBB36_165:
	.loc	52 71 9
	vmulss	.LCPI36_3(%rip), %xmm1, %xmm0
.Ltmp10358:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp10359:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm11
.Ltmp10360:
	.loc	1 1412 26
	movq	392(%r12), %rsi
.Ltmp10361:
	.loc	13 37 12
	testq	%r14, %r14
	movq	968(%rsp), %r15
	je	.LBB36_191
.Ltmp10362:
	.loc	13 0 12 is_stmt 0
	movq	520(%r12), %r10
.Ltmp10363:
	.loc	1 1403 42 is_stmt 1
	testq	%r10, %r10
	je	.LBB36_643
	.loc	1 0 42 is_stmt 0
	movq	512(%r12), %r11
	.loc	1 1403 42
	movl	8(%r11), %r9d
	.loc	1 1403 28
	addq	8(%rsp), %r9
.Ltmp10364:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %r9
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r9
	.loc	1 1407 40
	imulq	%r14, %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	.loc	1 0 25
	movq	384(%r12), %rdi
	.loc	1 1407 25
	vmovss	(%rdi,%r9,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 848(%rsp)
.Ltmp10365:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %r14
	je	.LBB36_190
.Ltmp10366:
	.loc	1 1403 42
	cmpq	$1, %r10
	je	.LBB36_627
	movl	20(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10367:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	1(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	4(%rdi,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 852(%rsp)
.Ltmp10368:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %r14
	je	.LBB36_190
.Ltmp10369:
	.loc	1 1403 42
	cmpq	$2, %r10
	je	.LBB36_630
	movl	32(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10370:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	2(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	8(%rdi,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 856(%rsp)
.Ltmp10371:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %r14
	je	.LBB36_190
.Ltmp10372:
	.loc	1 1403 42
	cmpq	$3, %r10
	je	.LBB36_633
	movl	44(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10373:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	3(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	12(%rdi,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 860(%rsp)
.Ltmp10374:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %r14
	je	.LBB36_190
.Ltmp10375:
	.loc	1 1403 42
	cmpq	$4, %r10
	je	.LBB36_635
	movl	56(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10376:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	4(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	16(%rdi,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 864(%rsp)
.Ltmp10377:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %r14
	je	.LBB36_190
.Ltmp10378:
	.loc	1 1403 42
	cmpq	$5, %r10
	je	.LBB36_637
	movl	68(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10379:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	5(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	20(%rdi,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 868(%rsp)
.Ltmp10380:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %r14
	je	.LBB36_190
.Ltmp10381:
	.loc	1 1403 42
	cmpq	$6, %r10
	je	.LBB36_639
	movl	80(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10382:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	6(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	24(%rdi,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 872(%rsp)
.Ltmp10383:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %r14
	je	.LBB36_190
.Ltmp10384:
	.loc	1 1403 42
	cmpq	$7, %r10
	je	.LBB36_641
	movl	92(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10385:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	7(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_617
	vmovss	28(%rdi,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 876(%rsp)
.Ltmp10386:
	.loc	1 0 13
.Ltmp10387:
	.p2align	4
.LBB36_190:
	movq	368(%rsp), %r10
.LBB36_191:
	movq	256(%rsp), %rdi
.Ltmp10388:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %rdi
	movl	592(%rsp), %ebx
	ja	.LBB36_458
.Ltmp10389:
	.loc	52 56 9
	je	.LBB36_611
.Ltmp10390:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm14, %xmm11, %xmm0
	vsubss	%xmm1, %xmm0, %xmm14
	.loc	1 1412 0 is_stmt 1
	movq	384(%r12), %rax
.Ltmp10391:
	.loc	52 56 9
	vmovss	%xmm11, (%rax,%rdi,4)
.Ltmp10392:
	.loc	52 76 9
	vdivss	%xmm15, %xmm14, %xmm0
.Ltmp10393:
	.loc	52 66 9
	vsubss	%xmm0, %xmm5, %xmm0
.Ltmp10394:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm6, %xmm0, %xmm1
.Ltmp10395:
	.loc	52 92 9 is_stmt 1
	vmulss	624(%rsp), %xmm1, %xmm1
	vaddss	%xmm1, %xmm6, %xmm1
.Ltmp10396:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp10397:
	.loc	52 103 24
	vandps	%xmm2, %xmm0, %xmm1
.Ltmp10398:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm1, %xmm1
	vandps	%xmm0, %xmm1, %xmm6
.Ltmp10399:
	.loc	1 1420 28
	movq	360(%r12), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	432(%rsp), %r14
.Ltmp10400:
	.loc	5 568 12 is_stmt 1
	cmpq	%rsi, %r14
	movq	104(%rsp), %rdi
	ja	.LBB36_460
.Ltmp10401:
	.loc	52 51 9
	je	.LBB36_614
.Ltmp10402:
	.loc	52 0 9 is_stmt 0
	movq	64(%rsp), %r9
	incq	%r9
.Ltmp10403:
	vsubss	%xmm6, %xmm5, %xmm0
.Ltmp10404:
	.loc	1 1420 0 is_stmt 1
	movq	352(%r12), %rax
.Ltmp10405:
	.loc	52 51 9
	vmovss	(%rax,%r14,4), %xmm1
.Ltmp10406:
	.loc	52 56 9
	vmovss	%xmm3, (%rax,%r14,4)
.Ltmp10407:
	.loc	52 71 9
	vmulss	%xmm1, %xmm0, %xmm0
.Ltmp10408:
	.loc	41 1244 18
	vmovd	%xmm1, %eax
.Ltmp10409:
	.loc	52 161 24
	andl	160(%rsp), %eax
.Ltmp10410:
	.loc	41 1244 18
	vmovd	%xmm0, %edx
.Ltmp10411:
	.loc	52 161 44
	andl	%ebx, %edx
	.loc	52 161 24 is_stmt 0
	orl	%eax, %edx
.Ltmp10412:
	.loc	52 56 9 is_stmt 1
	movl	%edx, (%rbp,%r10,4)
	movq	432(%rsp), %rax
.Ltmp10413:
	.loc	1 1878 13
	incq	%rax
	.loc	1 1879 16
	cmpq	248(%rsp), %rax
	movl	$0, %esi
	cmoveq	%rsi, %rax
	movq	%rax, 432(%rsp)
	movq	8(%rsp), %rax
	.loc	1 1882 13
	incq	%rax
	.loc	1 1883 16
	cmpq	%rcx, %rax
	movl	$0, %edx
	movq	%rdx, 128(%rsp)
	cmoveq	%rsi, %rax
	movq	%rax, 8(%rsp)
	movq	%r9, %rax
.Ltmp10414:
	.loc	10 1916 50
	cmpq	616(%rsp), %r9
	movq	96(%rsp), %rsi
.Ltmp10415:
	.loc	3 900 12
	jne	.LBB36_85
	jmp	.LBB36_70
.Ltmp10416:
.LBB36_196:
	.loc	3 0 12 is_stmt 0
	leaq	740(%rsp), %rdi
.Ltmp10417:
	.loc	1 1794 24 is_stmt 1
	leaq	136(%r12), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	496(%rsp), %rdi
.Ltmp10418:
	.loc	1 1795 25
	leaq	336(%r12), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp10419:
	.loc	1 1800 19
	movzbl	776(%r12), %r14d
.Ltmp10420:
	.loc	1 1801 21
	movzbl	777(%r12), %ebx
.Ltmp10421:
	.loc	1 1802 27
	movl	560(%r12), %eax
	movq	%rax, 368(%rsp)
.Ltmp10422:
	.loc	1 1803 27
	movl	564(%r12), %eax
	movq	%rax, 8(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%ymm0, 1424(%rsp)
	leaq	2648(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
	leaq	1616(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	movq	120(%rsp), %rcx
.Ltmp10423:
	.loc	8 446 20
	testq	%rcx, %rcx
	je	.LBB36_335
.Ltmp10424:
	.loc	1 0 0 is_stmt 0
	movl	%r14d, %eax
	negl	%eax
	movl	%eax, 960(%rsp)
.Ltmp10425:
	movl	%ebx, %eax
	negl	%eax
	movl	%eax, 488(%rsp)
.Ltmp10426:
	.loc	4 3756 21 is_stmt 1
	movq	%rcx, %rdx
	shrq	$5, %rdx
.Ltmp10427:
	.loc	4 3757 21
	movl	%ecx, %eax
	andl	$31, %eax
.Ltmp10428:
	.loc	4 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %rdx
	decl	%r14d
	movl	%r14d, 624(%rsp)
	decl	%ebx
	movl	%ebx, 248(%rsp)
	movq	$0, 144(%rsp)
	vxorps	%xmm6, %xmm6, %xmm6
	movq	%rbp, %rax
	movq	%r13, %r8
	movq	%rcx, %r9
	xorl	%esi, %esi
.Ltmp10429:
	.loc	8 446 20
	jmp	.LBB36_200
.Ltmp10430:
	.loc	8 0 20 is_stmt 0
.Ltmp10431:
	.p2align	4
.LBB36_198:
	movq	128(%rsp), %rax
.LBB36_199:
	addq	$32, %rsi
	decq	%rdx
.Ltmp10432:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %r9
	subq	$-128, %r8
	subq	$-128, %rax
	testq	%rdx, %rdx
	je	.LBB36_335
.LBB36_200:
	.loc	8 0 20 is_stmt 0
	movq	%rax, 128(%rsp)
.Ltmp10433:
	.loc	4 2584 13 is_stmt 1
	cmpq	$1, %r9
	movq	%r9, %r10
	adcq	$0, %r10
	cmpq	$32, %r10
	movl	$32, %eax
	cmovaeq	%rax, %r10
	movq	%rsi, %rdi
.Ltmp10434:
	.loc	1 1815 51
	movq	%rcx, %rsi
	subq	%rdi, %rsi
.Ltmp10435:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%rax, %rsi
	movq	%rdi, 224(%rsp)
.Ltmp10436:
	.loc	1 1820 35
	addq	%rdi, %rsi
.Ltmp10437:
	.loc	4 1050 16
	jb	.LBB36_488
	cmpq	%r15, %rsi
	ja	.LBB36_488
.Ltmp10438:
	.loc	1 1759 23
	vmovss	740(%rsp), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	744(%rsp), %xmm5
	vmovss	748(%rsp), %xmm9
	vmovss	752(%rsp), %xmm11
	vmovss	756(%rsp), %xmm15
	vmovss	760(%rsp), %xmm4
	vmovss	764(%rsp), %xmm8
	vmovss	768(%rsp), %xmm10
	vmovss	772(%rsp), %xmm13
	vmovss	776(%rsp), %xmm1
	vmovss	780(%rsp), %xmm14
.Ltmp10439:
	.loc	11 304 12
	cmpq	224(%rsp), %rcx
	jne	.LBB36_204
.Ltmp10440:
	.loc	1 0 0 is_stmt 0
	vmovss	784(%rsp), %xmm3
	vmovss	48(%rsp), %xmm0
.Ltmp10441:
	.loc	11 304 12
	jmp	.LBB36_206
.Ltmp10442:
	.loc	11 0 12
.Ltmp10443:
	.p2align	4
.LBB36_204:
	vmovss	584(%r12), %xmm0
	vmovss	%xmm0, 152(%rsp)
	vmovss	588(%r12), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	592(%r12), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	596(%r12), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	600(%r12), %xmm0
	vmovss	%xmm0, 232(%rsp)
	vmovss	604(%r12), %xmm0
	vmovss	%xmm0, 168(%rsp)
	xorl	%eax, %eax
	vmovss	%xmm14, 16(%rsp)
	vmovss	%xmm1, 40(%rsp)
	vmovss	%xmm13, 432(%rsp)
	vmovss	%xmm10, 256(%rsp)
	vmovss	%xmm8, 176(%rsp)
	vmovaps	%xmm4, 704(%rsp)
	vmovss	%xmm15, 36(%rsp)
	vmovaps	%xmm11, %xmm7
	vmovaps	%xmm9, %xmm12
	vmovaps	%xmm5, %xmm2
	vmovss	608(%r12), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	612(%r12), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	616(%r12), %xmm0
	vmovss	%xmm0, 216(%rsp)
	vmovss	620(%r12), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	624(%r12), %xmm0
	vmovss	%xmm0, 208(%rsp)
	vmovss	628(%r12), %xmm0
	vmovss	%xmm0, 60(%rsp)
	vmovss	632(%r12), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	636(%r12), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	640(%r12), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	644(%r12), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	648(%r12), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	652(%r12), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	656(%r12), %xmm0
	vmovss	%xmm0, 76(%rsp)
	vmovss	660(%r12), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	664(%r12), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	668(%r12), %xmm0
	vmovss	%xmm0, 296(%rsp)
	vmovss	672(%r12), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	676(%r12), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	680(%r12), %xmm0
	vmovss	%xmm0, 288(%rsp)
	vmovss	684(%r12), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	688(%r12), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	692(%r12), %xmm0
	vmovss	%xmm0, 4(%rsp)
	vmovss	696(%r12), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	700(%r12), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	704(%r12), %xmm0
	vmovss	%xmm0, 688(%rsp)
	vmovss	708(%r12), %xmm0
	vmovss	%xmm0, 680(%rsp)
	vmovss	712(%r12), %xmm0
	vmovss	%xmm0, 672(%rsp)
	vmovss	716(%r12), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	720(%r12), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	724(%r12), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	728(%r12), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	732(%r12), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	736(%r12), %xmm0
	vmovss	%xmm0, 648(%rsp)
	vmovss	740(%r12), %xmm0
	vmovss	%xmm0, 640(%rsp)
	vmovss	744(%r12), %xmm0
	vmovss	%xmm0, 92(%rsp)
	vmovss	748(%r12), %xmm0
	vmovss	%xmm0, 448(%rsp)
	vmovss	752(%r12), %xmm0
	vmovss	%xmm0, 632(%rsp)
	vmovss	756(%r12), %xmm0
	vmovss	%xmm0, 728(%rsp)
	vmovss	760(%r12), %xmm0
	vmovss	%xmm0, 616(%rsp)
	vmovss	764(%r12), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	768(%r12), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	772(%r12), %xmm0
	vmovss	%xmm0, 364(%rsp)
	vmovss	48(%rsp), %xmm0
	.p2align	4
.LBB36_205:
.Ltmp10444:
	.loc	52 51 9 is_stmt 1
	vmovss	(%r8,%rax,4), %xmm5
.Ltmp10445:
	.loc	52 71 9
	vmulss	152(%rsp), %xmm5, %xmm1
.Ltmp10446:
	.loc	52 61 9
	vaddss	%xmm6, %xmm1, %xmm1
.Ltmp10447:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm5, %xmm3
.Ltmp10448:
	.loc	52 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp10449:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm5, %xmm4
	vmovss	%xmm5, 48(%rsp)
.Ltmp10450:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
	vmovss	%xmm0, 160(%rsp)
.Ltmp10451:
	.loc	52 71 9
	vmulss	232(%rsp), %xmm0, %xmm8
.Ltmp10452:
	.loc	52 61 9
	vaddss	%xmm1, %xmm8, %xmm1
.Ltmp10453:
	.loc	52 71 9
	vmulss	168(%rsp), %xmm0, %xmm8
.Ltmp10454:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp10455:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm0, %xmm8
.Ltmp10456:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp10457:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm5, %xmm8
.Ltmp10458:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm8
.Ltmp10459:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm0, %xmm9
.Ltmp10460:
	.loc	52 61 9
	vaddss	%xmm9, %xmm8, %xmm8
	vmovaps	%xmm2, %xmm9
.Ltmp10461:
	.loc	52 71 9
	vmulss	216(%rsp), %xmm2, %xmm5
.Ltmp10462:
	.loc	52 61 9
	vaddss	%xmm5, %xmm1, %xmm1
.Ltmp10463:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm2, %xmm5
.Ltmp10464:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp10465:
	.loc	52 71 9
	vmulss	208(%rsp), %xmm2, %xmm5
.Ltmp10466:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
.Ltmp10467:
	.loc	52 71 9
	vmulss	60(%rsp), %xmm2, %xmm5
.Ltmp10468:
	.loc	52 61 9
	vaddss	%xmm5, %xmm8, %xmm5
	vmovaps	%xmm12, %xmm11
.Ltmp10469:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm12, %xmm8
.Ltmp10470:
	.loc	52 61 9
	vaddss	%xmm1, %xmm8, %xmm1
.Ltmp10471:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm12, %xmm8
.Ltmp10472:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp10473:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm12, %xmm8
.Ltmp10474:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp10475:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm12, %xmm8
.Ltmp10476:
	.loc	52 61 9
	vaddss	%xmm5, %xmm8, %xmm5
	vmovaps	%xmm7, %xmm15
.Ltmp10477:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm7, %xmm7
.Ltmp10478:
	.loc	52 61 9
	vaddss	%xmm7, %xmm1, %xmm1
.Ltmp10479:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm15, %xmm7
.Ltmp10480:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp10481:
	.loc	52 71 9
	vmulss	76(%rsp), %xmm15, %xmm7
.Ltmp10482:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm7
.Ltmp10483:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm15, %xmm4
.Ltmp10484:
	.loc	52 61 9
	vaddss	%xmm4, %xmm5, %xmm5
	vmovss	36(%rsp), %xmm4
.Ltmp10485:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm4, %xmm2
.Ltmp10486:
	.loc	52 61 9
	vaddss	%xmm2, %xmm1, %xmm1
.Ltmp10487:
	.loc	52 71 9
	vmulss	296(%rsp), %xmm4, %xmm2
.Ltmp10488:
	.loc	52 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp10489:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm4, %xmm3
.Ltmp10490:
	.loc	52 61 9
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp10491:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm4, %xmm7
.Ltmp10492:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
	vmovaps	704(%rsp), %xmm0
.Ltmp10493:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm0, %xmm7
.Ltmp10494:
	.loc	52 61 9
	vaddss	%xmm7, %xmm1, %xmm1
.Ltmp10495:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm0, %xmm7
.Ltmp10496:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp10497:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm0, %xmm7
.Ltmp10498:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp10499:
	.loc	52 71 9
	vmulss	4(%rsp), %xmm0, %xmm7
.Ltmp10500:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
	vmovss	176(%rsp), %xmm10
.Ltmp10501:
	.loc	52 71 9
	vmulss	80(%rsp), %xmm10, %xmm7
.Ltmp10502:
	.loc	52 61 9
	vaddss	%xmm7, %xmm1, %xmm1
.Ltmp10503:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm10, %xmm7
.Ltmp10504:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp10505:
	.loc	52 71 9
	vmulss	688(%rsp), %xmm10, %xmm7
.Ltmp10506:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp10507:
	.loc	52 71 9
	vmulss	680(%rsp), %xmm10, %xmm7
.Ltmp10508:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
	vmovss	256(%rsp), %xmm8
.Ltmp10509:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm8, %xmm7
.Ltmp10510:
	.loc	52 61 9
	vaddss	%xmm7, %xmm1, %xmm7
.Ltmp10511:
	.loc	52 71 9
	vmulss	32(%rsp), %xmm8, %xmm1
.Ltmp10512:
	.loc	52 61 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp10513:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm8, %xmm1
.Ltmp10514:
	.loc	52 61 9
	vaddss	%xmm1, %xmm3, %xmm3
.Ltmp10515:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm8, %xmm1
.Ltmp10516:
	.loc	52 61 9
	vaddss	%xmm1, %xmm5, %xmm5
	vmovss	432(%rsp), %xmm1
.Ltmp10517:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm1, %xmm12
.Ltmp10518:
	.loc	52 61 9
	vaddss	%xmm7, %xmm12, %xmm7
.Ltmp10519:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm1, %xmm12
.Ltmp10520:
	.loc	52 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp10521:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm1, %xmm12
.Ltmp10522:
	.loc	52 61 9
	vaddss	%xmm3, %xmm12, %xmm3
.Ltmp10523:
	.loc	52 71 9
	vmulss	640(%rsp), %xmm1, %xmm12
.Ltmp10524:
	.loc	52 61 9
	vaddss	%xmm5, %xmm12, %xmm5
	vmovss	40(%rsp), %xmm14
.Ltmp10525:
	.loc	52 71 9
	vmulss	92(%rsp), %xmm14, %xmm12
.Ltmp10526:
	.loc	52 61 9
	vaddss	%xmm7, %xmm12, %xmm7
.Ltmp10527:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm14, %xmm12
.Ltmp10528:
	.loc	52 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp10529:
	.loc	52 71 9
	vmulss	632(%rsp), %xmm14, %xmm12
.Ltmp10530:
	.loc	52 61 9
	vaddss	%xmm3, %xmm12, %xmm12
.Ltmp10531:
	.loc	52 71 9
	vmulss	728(%rsp), %xmm14, %xmm3
.Ltmp10532:
	.loc	52 61 9
	vaddss	%xmm3, %xmm5, %xmm5
	vmovss	16(%rsp), %xmm3
.Ltmp10533:
	.loc	52 71 9
	vmulss	616(%rsp), %xmm3, %xmm6
.Ltmp10534:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
.Ltmp10535:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm3, %xmm7
.Ltmp10536:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp10537:
	.loc	52 71 9
	vmulss	104(%rsp), %xmm3, %xmm7
.Ltmp10538:
	.loc	52 61 9
	vaddss	%xmm7, %xmm12, %xmm7
.Ltmp10539:
	.loc	52 71 9
	vmulss	364(%rsp), %xmm3, %xmm12
.Ltmp10540:
	.loc	52 61 9
	vaddss	%xmm5, %xmm12, %xmm5
	vbroadcastss	.LCPI36_0(%rip), %xmm13
.Ltmp10541:
	.loc	52 103 24
	vandps	%xmm6, %xmm13, %xmm6
.Ltmp10542:
	.loc	52 103 24 is_stmt 0
	vandps	%xmm0, %xmm13, %xmm12
.Ltmp10543:
	.loc	52 161 24 is_stmt 1
	vmaxss	%xmm6, %xmm12, %xmm6
.Ltmp10544:
	.loc	52 103 24
	vandps	%xmm2, %xmm13, %xmm2
.Ltmp10545:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm6, %xmm2
.Ltmp10546:
	.loc	52 103 24
	vandps	%xmm7, %xmm13, %xmm6
.Ltmp10547:
	.loc	52 161 24
	vmaxss	%xmm6, %xmm2, %xmm2
	vxorps	%xmm6, %xmm6, %xmm6
.Ltmp10548:
	.loc	52 103 24
	vandps	%xmm5, %xmm13, %xmm5
	vmovaps	%xmm8, %xmm13
	vmovaps	%xmm0, %xmm8
	vmovss	48(%rsp), %xmm0
.Ltmp10549:
	.loc	52 161 24
	vmaxss	%xmm5, %xmm2, %xmm2
.Ltmp10550:
	.loc	52 56 9
	vmovss	%xmm2, 2648(%rsp,%rax,4)
.Ltmp10551:
	.loc	11 308 13
	incq	%rax
	vmovss	%xmm14, 16(%rsp)
	vmovss	%xmm1, 40(%rsp)
	vmovss	%xmm13, 432(%rsp)
	vmovss	%xmm10, 256(%rsp)
	vmovss	%xmm8, 176(%rsp)
	vmovaps	%xmm4, 704(%rsp)
	vmovss	%xmm15, 36(%rsp)
	vmovaps	%xmm11, %xmm7
	vmovaps	%xmm9, %xmm12
	vmovss	160(%rsp), %xmm2
	vmovaps	%xmm2, %xmm5
.Ltmp10552:
	.loc	11 304 12
	cmpq	%rax, %r10
	jne	.LBB36_205
.Ltmp10553:
.LBB36_206:
	.loc	1 1764 5
	vmovss	%xmm0, 740(%rsp)
	vmovss	%xmm5, 744(%rsp)
	vmovss	%xmm9, 748(%rsp)
	vmovss	%xmm11, 752(%rsp)
	vmovss	%xmm15, 756(%rsp)
	vmovss	%xmm4, 760(%rsp)
	vmovss	%xmm8, 764(%rsp)
	vmovss	%xmm10, 768(%rsp)
	vmovss	%xmm13, 772(%rsp)
	vmovss	%xmm1, 776(%rsp)
	vmovss	%xmm14, 780(%rsp)
	vmovss	%xmm3, 784(%rsp)
.Ltmp10554:
	.loc	5 438 16
	cmpq	136(%rsp), %rsi
	ja	.LBB36_582
.Ltmp10555:
	.loc	1 1759 23
	vmovss	496(%rsp), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	500(%rsp), %xmm4
	vmovss	504(%rsp), %xmm9
	vmovss	508(%rsp), %xmm11
	vmovss	512(%rsp), %xmm10
	vmovss	516(%rsp), %xmm3
	vmovss	520(%rsp), %xmm7
	vmovss	524(%rsp), %xmm1
	vmovss	528(%rsp), %xmm12
	vmovss	532(%rsp), %xmm15
	vmovss	536(%rsp), %xmm14
	movq	224(%rsp), %rsi
.Ltmp10556:
	.loc	11 304 12
	cmpq	%rsi, %rcx
.Ltmp10557:
	.loc	11 304 12 is_stmt 0
	jne	.LBB36_209
.Ltmp10558:
	.loc	1 0 0
	vmovss	540(%rsp), %xmm5
	vmovss	48(%rsp), %xmm0
.Ltmp10559:
	.loc	11 304 12
	jmp	.LBB36_211
.Ltmp10560:
	.loc	11 0 12
.Ltmp10561:
	.p2align	4
.LBB36_209:
	vmovss	584(%r12), %xmm0
	vmovss	%xmm0, 152(%rsp)
	vmovss	588(%r12), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	592(%r12), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	596(%r12), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	600(%r12), %xmm0
	vmovss	%xmm0, 232(%rsp)
	vmovss	604(%r12), %xmm0
	vmovss	%xmm0, 168(%rsp)
	xorl	%eax, %eax
	vmovss	%xmm14, 16(%rsp)
	vmovss	%xmm15, 40(%rsp)
	vmovss	%xmm12, 432(%rsp)
	vmovss	%xmm1, 256(%rsp)
	vmovss	%xmm7, 176(%rsp)
	vmovaps	%xmm3, 704(%rsp)
	vmovss	%xmm10, 36(%rsp)
	vmovaps	%xmm11, %xmm8
	vmovaps	%xmm9, %xmm13
	vmovaps	%xmm4, %xmm2
	vmovss	608(%r12), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	612(%r12), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	616(%r12), %xmm0
	vmovss	%xmm0, 216(%rsp)
	vmovss	620(%r12), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	624(%r12), %xmm0
	vmovss	%xmm0, 208(%rsp)
	vmovss	628(%r12), %xmm0
	vmovss	%xmm0, 60(%rsp)
	vmovss	632(%r12), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	636(%r12), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	640(%r12), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	644(%r12), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	648(%r12), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	652(%r12), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	656(%r12), %xmm0
	vmovss	%xmm0, 76(%rsp)
	vmovss	660(%r12), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	664(%r12), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	668(%r12), %xmm0
	vmovss	%xmm0, 296(%rsp)
	vmovss	672(%r12), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	676(%r12), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	680(%r12), %xmm0
	vmovss	%xmm0, 288(%rsp)
	vmovss	684(%r12), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	688(%r12), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	692(%r12), %xmm0
	vmovss	%xmm0, 4(%rsp)
	vmovss	696(%r12), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	700(%r12), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	704(%r12), %xmm0
	vmovss	%xmm0, 688(%rsp)
	vmovss	708(%r12), %xmm0
	vmovss	%xmm0, 680(%rsp)
	vmovss	712(%r12), %xmm0
	vmovss	%xmm0, 672(%rsp)
	vmovss	716(%r12), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	720(%r12), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	724(%r12), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	728(%r12), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	732(%r12), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	736(%r12), %xmm0
	vmovss	%xmm0, 648(%rsp)
	vmovss	740(%r12), %xmm0
	vmovss	%xmm0, 640(%rsp)
	vmovss	744(%r12), %xmm0
	vmovss	%xmm0, 92(%rsp)
	vmovss	748(%r12), %xmm0
	vmovss	%xmm0, 448(%rsp)
	vmovss	752(%r12), %xmm0
	vmovss	%xmm0, 632(%rsp)
	vmovss	756(%r12), %xmm0
	vmovss	%xmm0, 728(%rsp)
	vmovss	760(%r12), %xmm0
	vmovss	%xmm0, 616(%rsp)
	vmovss	764(%r12), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	768(%r12), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	772(%r12), %xmm0
	vmovss	%xmm0, 364(%rsp)
	movq	128(%rsp), %rdi
	vmovss	48(%rsp), %xmm0
	.p2align	4
.LBB36_210:
.Ltmp10562:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rdi,%rax,4), %xmm4
.Ltmp10563:
	.loc	52 71 9
	vmulss	152(%rsp), %xmm4, %xmm1
.Ltmp10564:
	.loc	52 61 9
	vaddss	%xmm6, %xmm1, %xmm1
.Ltmp10565:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm4, %xmm3
.Ltmp10566:
	.loc	52 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp10567:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm4, %xmm5
	vmovss	%xmm4, 48(%rsp)
.Ltmp10568:
	.loc	52 61 9
	vaddss	%xmm6, %xmm5, %xmm5
	vmovss	%xmm0, 160(%rsp)
.Ltmp10569:
	.loc	52 71 9
	vmulss	232(%rsp), %xmm0, %xmm7
.Ltmp10570:
	.loc	52 61 9
	vaddss	%xmm7, %xmm1, %xmm1
.Ltmp10571:
	.loc	52 71 9
	vmulss	168(%rsp), %xmm0, %xmm7
.Ltmp10572:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp10573:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm0, %xmm7
.Ltmp10574:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp10575:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm4, %xmm7
.Ltmp10576:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm7
.Ltmp10577:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm0, %xmm9
.Ltmp10578:
	.loc	52 61 9
	vaddss	%xmm7, %xmm9, %xmm7
	vmovaps	%xmm2, %xmm9
.Ltmp10579:
	.loc	52 71 9
	vmulss	216(%rsp), %xmm2, %xmm4
.Ltmp10580:
	.loc	52 61 9
	vaddss	%xmm4, %xmm1, %xmm1
.Ltmp10581:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm2, %xmm4
.Ltmp10582:
	.loc	52 61 9
	vaddss	%xmm4, %xmm3, %xmm3
.Ltmp10583:
	.loc	52 71 9
	vmulss	208(%rsp), %xmm2, %xmm4
.Ltmp10584:
	.loc	52 61 9
	vaddss	%xmm4, %xmm5, %xmm4
.Ltmp10585:
	.loc	52 71 9
	vmulss	60(%rsp), %xmm2, %xmm5
.Ltmp10586:
	.loc	52 61 9
	vaddss	%xmm5, %xmm7, %xmm5
	vmovaps	%xmm13, %xmm11
.Ltmp10587:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm13, %xmm7
.Ltmp10588:
	.loc	52 61 9
	vaddss	%xmm7, %xmm1, %xmm1
.Ltmp10589:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm13, %xmm7
.Ltmp10590:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp10591:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm13, %xmm7
.Ltmp10592:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp10593:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm13, %xmm7
.Ltmp10594:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
	vmovaps	%xmm8, %xmm10
.Ltmp10595:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm8, %xmm7
.Ltmp10596:
	.loc	52 61 9
	vaddss	%xmm7, %xmm1, %xmm1
.Ltmp10597:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm8, %xmm7
.Ltmp10598:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm7
.Ltmp10599:
	.loc	52 71 9
	vmulss	76(%rsp), %xmm8, %xmm3
.Ltmp10600:
	.loc	52 61 9
	vaddss	%xmm3, %xmm4, %xmm4
.Ltmp10601:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm8, %xmm3
.Ltmp10602:
	.loc	52 61 9
	vaddss	%xmm3, %xmm5, %xmm5
	vmovss	36(%rsp), %xmm0
.Ltmp10603:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm0, %xmm2
.Ltmp10604:
	.loc	52 61 9
	vaddss	%xmm2, %xmm1, %xmm1
.Ltmp10605:
	.loc	52 71 9
	vmulss	296(%rsp), %xmm0, %xmm2
.Ltmp10606:
	.loc	52 61 9
	vaddss	%xmm2, %xmm7, %xmm2
.Ltmp10607:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm0, %xmm7
.Ltmp10608:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp10609:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm0, %xmm7
.Ltmp10610:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
	vmovaps	704(%rsp), %xmm7
.Ltmp10611:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm7, %xmm8
.Ltmp10612:
	.loc	52 61 9
	vaddss	%xmm1, %xmm8, %xmm8
.Ltmp10613:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm7, %xmm1
.Ltmp10614:
	.loc	52 61 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp10615:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm7, %xmm1
.Ltmp10616:
	.loc	52 61 9
	vaddss	%xmm1, %xmm4, %xmm4
.Ltmp10617:
	.loc	52 71 9
	vmulss	4(%rsp), %xmm7, %xmm1
.Ltmp10618:
	.loc	52 61 9
	vaddss	%xmm1, %xmm5, %xmm5
	vmovss	176(%rsp), %xmm1
.Ltmp10619:
	.loc	52 71 9
	vmulss	80(%rsp), %xmm1, %xmm12
.Ltmp10620:
	.loc	52 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp10621:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm1, %xmm12
.Ltmp10622:
	.loc	52 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp10623:
	.loc	52 71 9
	vmulss	688(%rsp), %xmm1, %xmm12
.Ltmp10624:
	.loc	52 61 9
	vaddss	%xmm4, %xmm12, %xmm4
.Ltmp10625:
	.loc	52 71 9
	vmulss	680(%rsp), %xmm1, %xmm12
.Ltmp10626:
	.loc	52 61 9
	vaddss	%xmm5, %xmm12, %xmm5
	vmovss	256(%rsp), %xmm12
.Ltmp10627:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm12, %xmm13
.Ltmp10628:
	.loc	52 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp10629:
	.loc	52 71 9
	vmulss	32(%rsp), %xmm12, %xmm13
.Ltmp10630:
	.loc	52 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp10631:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm12, %xmm13
.Ltmp10632:
	.loc	52 61 9
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp10633:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm12, %xmm13
.Ltmp10634:
	.loc	52 61 9
	vaddss	%xmm5, %xmm13, %xmm5
	vmovss	432(%rsp), %xmm15
.Ltmp10635:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm15, %xmm13
.Ltmp10636:
	.loc	52 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp10637:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm15, %xmm13
.Ltmp10638:
	.loc	52 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp10639:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm15, %xmm13
.Ltmp10640:
	.loc	52 61 9
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp10641:
	.loc	52 71 9
	vmulss	640(%rsp), %xmm15, %xmm13
.Ltmp10642:
	.loc	52 61 9
	vaddss	%xmm5, %xmm13, %xmm5
	vmovss	40(%rsp), %xmm14
.Ltmp10643:
	.loc	52 71 9
	vmulss	92(%rsp), %xmm14, %xmm13
.Ltmp10644:
	.loc	52 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp10645:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm14, %xmm13
.Ltmp10646:
	.loc	52 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp10647:
	.loc	52 71 9
	vmulss	632(%rsp), %xmm14, %xmm13
.Ltmp10648:
	.loc	52 61 9
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp10649:
	.loc	52 71 9
	vmulss	728(%rsp), %xmm14, %xmm13
.Ltmp10650:
	.loc	52 61 9
	vaddss	%xmm5, %xmm13, %xmm13
	vmovss	16(%rsp), %xmm5
.Ltmp10651:
	.loc	52 71 9
	vmulss	616(%rsp), %xmm5, %xmm6
.Ltmp10652:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp10653:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm5, %xmm8
.Ltmp10654:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp10655:
	.loc	52 71 9
	vmulss	104(%rsp), %xmm5, %xmm8
.Ltmp10656:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp10657:
	.loc	52 71 9
	vmulss	364(%rsp), %xmm5, %xmm8
.Ltmp10658:
	.loc	52 61 9
	vaddss	%xmm8, %xmm13, %xmm8
	vbroadcastss	.LCPI36_0(%rip), %xmm3
.Ltmp10659:
	.loc	52 103 24
	vandps	%xmm3, %xmm6, %xmm6
.Ltmp10660:
	.loc	52 103 24 is_stmt 0
	vandps	%xmm3, %xmm7, %xmm13
.Ltmp10661:
	.loc	52 161 24 is_stmt 1
	vmaxss	%xmm6, %xmm13, %xmm6
.Ltmp10662:
	.loc	52 103 24
	vandps	%xmm3, %xmm2, %xmm2
.Ltmp10663:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm6, %xmm2
	vxorps	%xmm6, %xmm6, %xmm6
.Ltmp10664:
	.loc	52 103 24
	vandps	%xmm3, %xmm4, %xmm4
.Ltmp10665:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm2, %xmm2
.Ltmp10666:
	.loc	52 103 24
	vandps	%xmm3, %xmm8, %xmm4
	vmovaps	%xmm0, %xmm3
	vmovss	48(%rsp), %xmm0
.Ltmp10667:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm2, %xmm2
.Ltmp10668:
	.loc	52 56 9
	vmovss	%xmm2, 1616(%rsp,%rax,4)
.Ltmp10669:
	.loc	11 308 13
	incq	%rax
	vmovss	%xmm14, 16(%rsp)
	vmovss	%xmm15, 40(%rsp)
	vmovss	%xmm12, 432(%rsp)
	vmovss	%xmm1, 256(%rsp)
	vmovss	%xmm7, 176(%rsp)
	vmovaps	%xmm3, 704(%rsp)
	vmovss	%xmm10, 36(%rsp)
	vmovaps	%xmm11, %xmm8
	vmovaps	%xmm9, %xmm13
	vmovss	160(%rsp), %xmm2
	vmovaps	%xmm2, %xmm4
.Ltmp10670:
	.loc	11 304 12
	cmpq	%rax, %r10
	jne	.LBB36_210
.Ltmp10671:
.LBB36_211:
	.loc	1 1764 5
	vmovss	%xmm0, 496(%rsp)
	vmovss	%xmm4, 500(%rsp)
	vmovss	%xmm9, 504(%rsp)
	vmovss	%xmm11, 508(%rsp)
	vmovss	%xmm10, 512(%rsp)
	vmovss	%xmm3, 516(%rsp)
	vmovss	%xmm7, 520(%rsp)
	vmovss	%xmm1, 524(%rsp)
	vmovss	%xmm12, 528(%rsp)
	vmovss	%xmm15, 532(%rsp)
	vmovss	%xmm14, 536(%rsp)
	vmovss	%xmm5, 540(%rsp)
.Ltmp10672:
	.loc	11 304 12
	cmpq	%rsi, %rcx
.Ltmp10673:
	.loc	3 900 12
	je	.LBB36_198
.Ltmp10674:
	.loc	3 0 12 is_stmt 0
	movq	%rdx, 304(%rsp)
	vmovss	788(%rsp), %xmm0
	vmovss	%xmm0, 168(%rsp)
	vmovss	792(%rsp), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	808(%rsp), %xmm0
	vmovss	%xmm0, 208(%rsp)
	vmovss	544(%rsp), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	548(%rsp), %xmm0
	vmovss	%xmm0, 60(%rsp)
	vmovss	564(%rsp), %xmm0
	vmovss	%xmm0, 312(%rsp)
	movq	544(%r12), %rcx
	movq	552(%r12), %rax
	movq	%rax, 216(%rsp)
	vmovss	828(%rsp), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	584(%rsp), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	800(%rsp), %xmm10
	vmovss	796(%rsp), %xmm12
	vmovss	816(%rsp), %xmm9
	vmovss	804(%rsp), %xmm0
	vmovss	%xmm0, 704(%rsp)
	vmovss	812(%rsp), %xmm11
	vmovss	556(%rsp), %xmm1
	vmovss	552(%rsp), %xmm14
	vmovss	572(%rsp), %xmm0
	movq	$0, 176(%rsp)
	vmovss	560(%rsp), %xmm2
	vmovss	%xmm2, 36(%rsp)
	vmovss	568(%rsp), %xmm4
	vmovss	824(%rsp), %xmm8
	vmovss	820(%rsp), %xmm15
	vmovss	580(%rsp), %xmm7
	vmovss	576(%rsp), %xmm3
	movq	%r8, 112(%rsp)
	movq	%r9, 592(%rsp)
	movq	%r10, 64(%rsp)
	.p2align	4
.LBB36_213:
	vmovss	.LCPI36_1(%rip), %xmm5
.Ltmp10675:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm5, %xmm10, %xmm13
.Ltmp10676:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm13
.Ltmp10677:
	.loc	52 161 24
	ja	.LBB36_215
.Ltmp10678:
	.loc	52 0 24 is_stmt 0
	vmovss	320(%rsp), %xmm10
	jmp	.LBB36_216
	.p2align	4
.LBB36_215:
	vmovss	168(%rsp), %xmm10
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm12, %xmm10, %xmm10
.Ltmp10679:
.LBB36_216:
	.loc	1 0 0 is_stmt 0
	movl	624(%rsp), %edx
	vmovss	.LCPI36_2(%rip), %xmm2
.Ltmp10680:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm5, %xmm9, %xmm9
	vmovss	%xmm9, 232(%rsp)
.Ltmp10681:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm9
	vmovss	%xmm13, 344(%rsp)
.Ltmp10682:
	.loc	52 161 24
	ja	.LBB36_220
.Ltmp10683:
	.loc	52 0 24 is_stmt 0
	vmovss	208(%rsp), %xmm9
	vmovss	%xmm9, 704(%rsp)
.Ltmp10684:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm5, %xmm1, %xmm13
.Ltmp10685:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm13
.Ltmp10686:
	.loc	52 161 24
	jbe	.LBB36_221
.Ltmp10687:
.LBB36_218:
	.loc	52 0 24 is_stmt 0
	vmovss	256(%rsp), %xmm1
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm1
	vmovss	%xmm1, 256(%rsp)
.Ltmp10688:
	.loc	52 66 9
	vaddss	%xmm5, %xmm0, %xmm5
.Ltmp10689:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm5
.Ltmp10690:
	.loc	52 161 24
	jbe	.LBB36_222
.Ltmp10691:
.LBB36_219:
	.loc	52 0 24 is_stmt 0
	vmovss	36(%rsp), %xmm0
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm4, %xmm0, %xmm0
	jmp	.LBB36_223
.Ltmp10692:
	.loc	1 0 0 is_stmt 0
.Ltmp10693:
	.p2align	4
.LBB36_220:
	vmovss	704(%rsp), %xmm9
.Ltmp10694:
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm11, %xmm9, %xmm9
	vmovss	%xmm9, 704(%rsp)
.Ltmp10695:
	.loc	52 66 9
	vaddss	%xmm5, %xmm1, %xmm13
.Ltmp10696:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm13
.Ltmp10697:
	.loc	52 161 24
	ja	.LBB36_218
.Ltmp10698:
.LBB36_221:
	.loc	52 0 24 is_stmt 0
	vmovss	60(%rsp), %xmm1
	vmovss	%xmm1, 256(%rsp)
.Ltmp10699:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm5, %xmm0, %xmm5
.Ltmp10700:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm5
.Ltmp10701:
	.loc	52 161 24
	ja	.LBB36_219
.Ltmp10702:
.LBB36_222:
	.loc	52 0 24 is_stmt 0
	vmovss	312(%rsp), %xmm0
.LBB36_223:
	vmovss	%xmm0, 36(%rsp)
	movq	176(%rsp), %rax
.Ltmp10703:
	.loc	52 51 9 is_stmt 1
	vmovss	2648(%rsp,%rax,4), %xmm0
.Ltmp10704:
	.loc	52 51 9 is_stmt 0
	vmovss	1616(%rsp,%rax,4), %xmm1
.Ltmp10705:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
.Ltmp10706:
	.loc	1 0 0 is_stmt 0
	leaq	(%rax,%rsi), %rdi
.Ltmp10707:
	.loc	41 1244 18 is_stmt 1
	vmovd	%xmm1, %r11d
.Ltmp10708:
	.loc	41 1244 18 is_stmt 0
	vmovd	%xmm0, %eax
.Ltmp10709:
	.loc	52 161 24 is_stmt 1
	movl	%r11d, %ebx
	cmovbel	%eax, %ebx
	movq	%rdi, 432(%rsp)
.Ltmp10710:
	.loc	5 568 12
	cmpq	%r15, %rdi
	ja	.LBB36_448
.Ltmp10711:
	.loc	52 51 9
	je	.LBB36_613
.Ltmp10712:
	.loc	1 0 0 is_stmt 0
	andl	960(%rsp), %ebx
	andl	%edx, %eax
	orl	%ebx, %eax
	vmovd	%eax, %xmm1
.Ltmp10713:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm1
	vmovaps	%xmm2, %xmm0
.Ltmp10714:
	.loc	52 161 24
	jbe	.LBB36_227
.Ltmp10715:
	.loc	52 76 9
	vdivss	%xmm1, %xmm10, %xmm0
.Ltmp10716:
.LBB36_227:
	.loc	1 0 0 is_stmt 0
	movq	328(%r12), %rax
.Ltmp10717:
	.loc	1 1392 25 is_stmt 1
	movq	176(%r12), %rsi
	.loc	1 1392 45 is_stmt 0
	movq	%rax, %rdi
	imulq	8(%rsp), %rdi
.Ltmp10718:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %rdi
	ja	.LBB36_431
.Ltmp10719:
	.loc	5 0 12 is_stmt 0
	movq	%rax, 160(%rsp)
	movl	%ebx, 352(%rsp)
	movl	%r11d, 240(%rsp)
.Ltmp10720:
	.loc	52 56 9 is_stmt 1
	je	.LBB36_612
.Ltmp10721:
	.loc	52 0 9 is_stmt 0
	movq	432(%rsp), %rax
	vmovss	(%r13,%rax,4), %xmm6
.Ltmp10722:
	.loc	1 1392 0 is_stmt 1
	movq	168(%r12), %rax
	movq	%rdi, 152(%rsp)
.Ltmp10723:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp10724:
	.loc	1 1259 17
	movq	328(%r12), %rbx
.Ltmp10725:
	.loc	13 37 12
	testq	%rbx, %rbx
	je	.LBB36_248
.Ltmp10726:
	.loc	13 0 12 is_stmt 0
	movq	696(%rsp), %rdx
	movq	320(%rdx), %rax
	movq	%rax, 48(%rsp)
	movq	8(%rsp), %rax
	leaq	1(%rax), %rdi
	cmpq	%rcx, %rdi
	movq	%rcx, %rax
	movl	$0, %esi
	cmovbq	%rsi, %rax
	movq	312(%rdx), %r15
	subq	%rax, %rdi
	movq	168(%rdx), %rbp
	movq	176(%rdx), %rsi
	movq	256(%rdx), %rax
	movq	%rax, 16(%rsp)
	movq	248(%rdx), %r13
	movq	224(%rdx), %r10
	movq	216(%rdx), %r8
	imulq	%rbx, %rdi
	movq	%rdi, 40(%rsp)
	movq	%rbx, %rax
	xorl	%r12d, %r12d
	xorl	%edx, %edx
	jmp	.LBB36_233
	.p2align	4
.LBB36_231:
	xorl	%r11d, %r11d
.LBB36_232:
	decq	%rax
	addq	$4, %rdx
.Ltmp10727:
	movl	%r11d, (%r13,%r12,4)
.Ltmp10728:
	incq	%r12
.Ltmp10729:
	.loc	13 37 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB36_248
.LBB36_233:
.Ltmp10730:
	.loc	7 1714 9
	cmpq	$32, %rdx
.Ltmp10731:
	.loc	6 180 28
	je	.LBB36_248
.Ltmp10732:
	.loc	1 1263 21
	cmpq	48(%rsp), %r12
	je	.LBB36_598
	leaq	(%r12,%r12,2), %r9
	movl	4(%r15,%r9,4), %r14d
.Ltmp10733:
	.loc	1 1265 23
	addq	8(%rsp), %r14
.Ltmp10734:
	.loc	1 1266 12
	cmpq	%rcx, %r14
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %r14
.Ltmp10735:
	.loc	1 1273 42
	movq	%r14, %rdi
	imulq	%rbx, %rdi
	addq	%r12, %rdi
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB36_602
.Ltmp10736:
	.loc	1 1274 24 is_stmt 1
	cmpq	16(%rsp), %r12
	je	.LBB36_604
.Ltmp10737:
	.loc	1 0 0 is_stmt 0
	movl	(%r15,%r9,4), %r9d
.Ltmp10738:
	vmovss	(%rbp,%rdi,4), %xmm0
.Ltmp10739:
	.loc	1 1274 24
	movl	(%r13,%r12,4), %r11d
	testq	%r11, %r11
	je	.LBB36_241
.Ltmp10740:
	.loc	1 1278 24 is_stmt 1
	cmpq	%r10, %r12
	jae	.LBB36_608
	vmovss	(%r8,%r12,4), %xmm1
.Ltmp10741:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB36_241
.Ltmp10742:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB36_241:
.Ltmp10743:
	.loc	1 1280 9 is_stmt 1
	cmpq	%r10, %r12
	je	.LBB36_600
	vmovss	%xmm0, (%r8,%r12,4)
	.loc	1 1281 24
	incq	%r11
	cmpq	%r9, %r11
.Ltmp10744:
	.loc	1 1282 23
	jne	.LBB36_246
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 1424(%rsp,%rdx)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rbp,%rdi,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp10745:
	.p2align	4
.LBB36_244:
.Ltmp10746:
	.loc	1 1291 65 is_stmt 1
	movq	%r14, %rdi
	imulq	%rbx, %rdi
	addq	%r12, %rdi
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB36_465
.Ltmp10747:
	.loc	1 903 8 is_stmt 1
	vminss	(%rbp,%rdi,4), %xmm0, %xmm0
.Ltmp10748:
	.loc	1 1292 17
	vmovss	%xmm0, (%rbp,%rdi,4)
	.loc	1 1293 20
	testq	%r14, %r14
	cmoveq	%rcx, %r14
	.loc	1 1296 17
	decq	%r14
.Ltmp10749:
	.loc	10 1916 50
	decq	%r9
.Ltmp10750:
	.loc	3 900 12
	jne	.LBB36_244
	jmp	.LBB36_231
.Ltmp10751:
	.loc	3 0 12 is_stmt 0
.Ltmp10752:
	.p2align	4
.LBB36_246:
	movq	40(%rsp), %rdi
	.loc	1 1285 44 is_stmt 1
	addq	%r12, %rdi
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB36_610
	vmovss	(%rbp,%rdi,4), %xmm1
.Ltmp10753:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp10754:
	.loc	1 1282 9
	vmovss	%xmm0, 1424(%rsp,%rdx)
	jmp	.LBB36_232
.Ltmp10755:
	.loc	1 0 9 is_stmt 0
.Ltmp10756:
	.p2align	4
.LBB36_248:
	vmovss	%xmm10, 168(%rsp)
.Ltmp10757:
	.loc	52 51 9 is_stmt 1
	vmovss	1424(%rsp), %xmm9
.Ltmp10758:
	.loc	52 71 9
	vmulss	.LCPI36_3(%rip), %xmm9, %xmm0
.Ltmp10759:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp10760:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm10
	movq	696(%rsp), %r12
.Ltmp10761:
	.loc	1 1412 26
	movq	192(%r12), %rsi
	movq	160(%rsp), %rdi
.Ltmp10762:
	.loc	13 37 12
	testq	%rdi, %rdi
	movq	840(%rsp), %rbp
	movl	248(%rsp), %r8d
	movq	152(%rsp), %r14
	je	.LBB36_273
.Ltmp10763:
	.loc	13 0 12 is_stmt 0
	movq	320(%r12), %r10
.Ltmp10764:
	.loc	1 1403 42 is_stmt 1
	testq	%r10, %r10
	je	.LBB36_646
	.loc	1 0 42 is_stmt 0
	movq	312(%r12), %r11
	.loc	1 1403 42
	movl	8(%r11), %r9d
	.loc	1 1403 28
	addq	8(%rsp), %r9
.Ltmp10765:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %r9
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r9
	.loc	1 1407 40
	imulq	%rdi, %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	.loc	1 0 25
	movq	184(%r12), %rbx
	.loc	1 1407 25
	vmovss	(%rbx,%r9,4), %xmm9
	.loc	1 1407 13
	vmovss	%xmm9, 1424(%rsp)
.Ltmp10766:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB36_273
.Ltmp10767:
	.loc	1 1403 42
	cmpq	$1, %r10
	je	.LBB36_628
	movl	20(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10768:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	vmovss	4(%rbx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1428(%rsp)
.Ltmp10769:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB36_273
.Ltmp10770:
	.loc	1 1403 42
	cmpq	$2, %r10
	je	.LBB36_631
	movl	32(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10771:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	vmovss	8(%rbx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1432(%rsp)
.Ltmp10772:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB36_273
.Ltmp10773:
	.loc	1 1403 42
	cmpq	$3, %r10
	je	.LBB36_634
	movl	44(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10774:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	vmovss	12(%rbx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1436(%rsp)
.Ltmp10775:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB36_273
.Ltmp10776:
	.loc	1 1403 42
	cmpq	$4, %r10
	je	.LBB36_636
	movl	56(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10777:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	vmovss	16(%rbx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1440(%rsp)
.Ltmp10778:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB36_273
.Ltmp10779:
	.loc	1 1403 42
	cmpq	$5, %r10
	je	.LBB36_638
	movl	68(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10780:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	vmovss	20(%rbx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1444(%rsp)
.Ltmp10781:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB36_273
.Ltmp10782:
	.loc	1 1403 42
	cmpq	$6, %r10
	je	.LBB36_640
	movl	80(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10783:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	vmovss	24(%rbx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1448(%rsp)
.Ltmp10784:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB36_273
.Ltmp10785:
	.loc	1 1403 42
	cmpq	$7, %r10
	je	.LBB36_644
	movl	92(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp10786:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	vmovss	28(%rbx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1452(%rsp)
.Ltmp10787:
	.loc	1 0 13
.Ltmp10788:
	.p2align	4
.LBB36_273:
	.loc	52 61 9 is_stmt 1
	vaddss	%xmm8, %xmm10, %xmm0
.Ltmp10789:
	.loc	52 66 9
	vsubss	%xmm9, %xmm0, %xmm8
.Ltmp10790:
	.loc	5 580 12
	cmpq	%rsi, %r14
	movq	64(%rsp), %r10
	ja	.LBB36_452
.Ltmp10791:
	.loc	52 56 9
	je	.LBB36_612
.Ltmp10792:
	.loc	1 1412 0
	movq	184(%r12), %rax
.Ltmp10793:
	.loc	52 56 9
	vmovss	%xmm10, (%rax,%r14,4)
.Ltmp10794:
	.loc	52 76 9
	vdivss	336(%rsp), %xmm8, %xmm0
.Ltmp10795:
	.loc	52 66 9
	vsubss	%xmm0, %xmm2, %xmm0
.Ltmp10796:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm15, %xmm0, %xmm1
.Ltmp10797:
	.loc	52 92 9 is_stmt 1
	vmulss	704(%rsp), %xmm1, %xmm1
	vaddss	%xmm1, %xmm15, %xmm1
.Ltmp10798:
	.loc	52 103 24
	vbroadcastss	.LCPI36_0(%rip), %xmm10
.Ltmp10799:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp10800:
	.loc	52 103 24
	vandps	%xmm0, %xmm10, %xmm1
.Ltmp10801:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm1, %xmm1
	vandps	%xmm0, %xmm1, %xmm15
.Ltmp10802:
	.loc	1 1417 5
	vmovss	%xmm15, 820(%rsp)
.Ltmp10803:
	.loc	1 1420 28
	movq	160(%r12), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	368(%rsp), %rdi
.Ltmp10804:
	.loc	5 568 12 is_stmt 1
	cmpq	%rsi, %rdi
	movq	976(%rsp), %r13
	movq	592(%rsp), %r9
	ja	.LBB36_453
.Ltmp10805:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_613
	vsubss	%xmm15, %xmm2, %xmm0
.Ltmp10806:
	.loc	1 1420 0 is_stmt 1
	movq	152(%r12), %rax
.Ltmp10807:
	.loc	52 51 9
	vmovss	(%rax,%rdi,4), %xmm1
.Ltmp10808:
	.loc	52 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp10809:
	.loc	52 71 9
	vmulss	%xmm1, %xmm0, %xmm0
.Ltmp10810:
	.loc	41 1244 18
	vmovd	%xmm1, %eax
.Ltmp10811:
	.loc	52 161 24
	andl	488(%rsp), %eax
.Ltmp10812:
	.loc	41 1244 18
	vmovd	%xmm0, %edx
.Ltmp10813:
	.loc	52 161 44
	andl	%r8d, %edx
	.loc	52 161 24 is_stmt 0
	orl	%eax, %edx
	movq	432(%rsp), %rax
.Ltmp10814:
	.loc	52 56 9 is_stmt 1
	movl	%edx, (%r13,%rax,4)
	movq	136(%rsp), %rdx
.Ltmp10815:
	.loc	5 568 12
	cmpq	%rdx, %rax
	ja	.LBB36_455
.Ltmp10816:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_613
	movl	240(%rsp), %eax
	andl	624(%rsp), %eax
	movl	352(%rsp), %edx
	orl	%eax, %edx
	vmovd	%edx, %xmm1
.Ltmp10817:
	.loc	52 124 14 is_stmt 1
	vucomiss	256(%rsp), %xmm1
	vmovaps	%xmm2, %xmm0
	movq	112(%rsp), %r8
.Ltmp10818:
	.loc	52 161 24
	jbe	.LBB36_281
.Ltmp10819:
	.loc	52 0 24 is_stmt 0
	vmovss	256(%rsp), %xmm0
.Ltmp10820:
	.loc	52 76 9 is_stmt 1
	vdivss	%xmm1, %xmm0, %xmm0
.Ltmp10821:
.LBB36_281:
	.loc	1 0 0 is_stmt 0
	movq	528(%r12), %r11
.Ltmp10822:
	.loc	1 1392 25 is_stmt 1
	movq	376(%r12), %rsi
	.loc	1 1392 45 is_stmt 0
	movq	%r11, %rdi
	imulq	8(%rsp), %rdi
.Ltmp10823:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %rdi
	ja	.LBB36_431
.Ltmp10824:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_612
	movq	432(%rsp), %rax
	vmovss	(%rbp,%rax,4), %xmm6
.Ltmp10825:
	.loc	1 1392 0 is_stmt 1
	movq	368(%r12), %rax
.Ltmp10826:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp10827:
	.loc	1 1259 17
	movq	528(%r12), %rax
.Ltmp10828:
	.loc	13 37 12
	testq	%rax, %rax
	je	.LBB36_303
.Ltmp10829:
	.loc	13 0 12 is_stmt 0
	movq	%rdi, 152(%rsp)
	movq	%r11, 160(%rsp)
	movq	520(%r12), %rdx
	movq	%rdx, 48(%rsp)
	movq	8(%rsp), %rdx
	leaq	1(%rdx), %rdi
	cmpq	%rcx, %rdi
	movq	%rcx, %rdx
	movl	$0, %esi
	cmovbq	%rsi, %rdx
	movq	512(%r12), %r14
	subq	%rdx, %rdi
	movq	368(%r12), %r15
	movq	376(%r12), %rsi
	movq	456(%r12), %rdx
	movq	%rdx, 16(%rsp)
	movq	448(%r12), %r13
	movq	424(%r12), %r10
	movq	416(%r12), %rbp
	imulq	%rax, %rdi
	movq	%rdi, 40(%rsp)
	movq	%rax, %r8
	xorl	%r12d, %r12d
	xorl	%edx, %edx
	jmp	.LBB36_287
	.p2align	4
.LBB36_285:
	xorl	%r11d, %r11d
.LBB36_286:
	decq	%r8
	addq	$4, %rdx
.Ltmp10830:
	movl	%r11d, (%r13,%r12,4)
.Ltmp10831:
	incq	%r12
.Ltmp10832:
	.loc	13 37 12 is_stmt 1
	testq	%r8, %r8
	je	.LBB36_302
.LBB36_287:
.Ltmp10833:
	.loc	7 1714 9
	cmpq	$32, %rdx
.Ltmp10834:
	.loc	6 180 28
	je	.LBB36_302
.Ltmp10835:
	.loc	1 1263 21
	cmpq	48(%rsp), %r12
	je	.LBB36_598
	leaq	(%r12,%r12,2), %r9
	movl	4(%r14,%r9,4), %ebx
.Ltmp10836:
	.loc	1 1265 23
	addq	8(%rsp), %rbx
.Ltmp10837:
	.loc	1 1266 12
	cmpq	%rcx, %rbx
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rbx
.Ltmp10838:
	.loc	1 1273 42
	movq	%rbx, %rdi
	imulq	%rax, %rdi
	addq	%r12, %rdi
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB36_602
.Ltmp10839:
	.loc	1 1274 24 is_stmt 1
	cmpq	16(%rsp), %r12
	je	.LBB36_604
.Ltmp10840:
	.loc	1 0 0 is_stmt 0
	movl	(%r14,%r9,4), %r9d
.Ltmp10841:
	vmovss	(%r15,%rdi,4), %xmm0
.Ltmp10842:
	.loc	1 1274 24
	movl	(%r13,%r12,4), %r11d
	testq	%r11, %r11
	je	.LBB36_295
.Ltmp10843:
	.loc	1 1278 24 is_stmt 1
	cmpq	%r10, %r12
	jae	.LBB36_608
	vmovss	(%rbp,%r12,4), %xmm1
.Ltmp10844:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB36_295
.Ltmp10845:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB36_295:
.Ltmp10846:
	.loc	1 1280 9 is_stmt 1
	cmpq	%r10, %r12
	je	.LBB36_600
	vmovss	%xmm0, (%rbp,%r12,4)
	.loc	1 1281 24
	incq	%r11
	cmpq	%r9, %r11
.Ltmp10847:
	.loc	1 1282 23
	jne	.LBB36_300
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 1424(%rsp,%rdx)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%r15,%rdi,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp10848:
	.p2align	4
.LBB36_298:
.Ltmp10849:
	.loc	1 1291 65 is_stmt 1
	movq	%rbx, %rdi
	imulq	%rax, %rdi
	addq	%r12, %rdi
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB36_465
.Ltmp10850:
	.loc	1 903 8 is_stmt 1
	vminss	(%r15,%rdi,4), %xmm0, %xmm0
.Ltmp10851:
	.loc	1 1292 17
	vmovss	%xmm0, (%r15,%rdi,4)
	.loc	1 1293 20
	testq	%rbx, %rbx
	cmoveq	%rcx, %rbx
	.loc	1 1296 17
	decq	%rbx
.Ltmp10852:
	.loc	10 1916 50
	decq	%r9
.Ltmp10853:
	.loc	3 900 12
	jne	.LBB36_298
	jmp	.LBB36_285
.Ltmp10854:
	.loc	3 0 12 is_stmt 0
.Ltmp10855:
	.p2align	4
.LBB36_300:
	movq	40(%rsp), %rdi
	.loc	1 1285 44 is_stmt 1
	addq	%r12, %rdi
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB36_610
	vmovss	(%r15,%rdi,4), %xmm1
.Ltmp10856:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp10857:
	.loc	1 1282 9
	vmovss	%xmm0, 1424(%rsp,%rdx)
	jmp	.LBB36_286
.Ltmp10858:
	.loc	1 0 9 is_stmt 0
.Ltmp10859:
	.p2align	4
.LBB36_302:
	.loc	52 51 9 is_stmt 1
	vmovss	1424(%rsp), %xmm9
	movq	696(%rsp), %r12
	movq	976(%rsp), %r13
	movq	840(%rsp), %rbp
	movq	112(%rsp), %r8
	movq	592(%rsp), %r9
	movq	64(%rsp), %r10
	movq	160(%rsp), %r11
	movq	152(%rsp), %rdi
.Ltmp10860:
.LBB36_303:
	.loc	52 71 9
	vmulss	.LCPI36_3(%rip), %xmm9, %xmm0
.Ltmp10861:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp10862:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm0
.Ltmp10863:
	.loc	1 1412 26
	movq	392(%r12), %rsi
.Ltmp10864:
	.loc	13 37 12
	testq	%r11, %r11
	movq	968(%rsp), %r15
	je	.LBB36_329
.Ltmp10865:
	.loc	13 0 12 is_stmt 0
	movq	520(%r12), %rbx
.Ltmp10866:
	.loc	1 1403 42 is_stmt 1
	testq	%rbx, %rbx
	je	.LBB36_646
	.loc	1 0 42 is_stmt 0
	movq	512(%r12), %r10
	.loc	1 1403 42
	movl	8(%r10), %r9d
	.loc	1 1403 28
	addq	8(%rsp), %r9
.Ltmp10867:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %r9
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r9
	.loc	1 1407 40
	imulq	%r11, %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	.loc	1 0 25
	movq	384(%r12), %rax
	.loc	1 1407 25
	vmovss	(%rax,%r9,4), %xmm9
	.loc	1 1407 13
	vmovss	%xmm9, 1424(%rsp)
.Ltmp10868:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %r11
	je	.LBB36_328
.Ltmp10869:
	.loc	1 1403 42
	cmpq	$1, %rbx
	je	.LBB36_628
	movl	20(%r10), %edx
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rdx
.Ltmp10870:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rdx
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rdx
	.loc	1 1407 40
	imulq	%r11, %rdx
	leaq	1(%rdx), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	movq	112(%rsp), %r8
	jae	.LBB36_618
	vmovss	4(%rax,%rdx,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1428(%rsp)
.Ltmp10871:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %r11
	je	.LBB36_328
.Ltmp10872:
	.loc	1 1403 42
	cmpq	$2, %rbx
	je	.LBB36_631
	movl	32(%r10), %edx
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rdx
.Ltmp10873:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rdx
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rdx
	.loc	1 1407 40
	imulq	%r11, %rdx
	leaq	2(%rdx), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	movq	112(%rsp), %r8
	jae	.LBB36_618
	vmovss	8(%rax,%rdx,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1432(%rsp)
.Ltmp10874:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %r11
	je	.LBB36_328
.Ltmp10875:
	.loc	1 1403 42
	cmpq	$3, %rbx
	je	.LBB36_634
	movl	44(%r10), %edx
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rdx
.Ltmp10876:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rdx
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rdx
	.loc	1 1407 40
	imulq	%r11, %rdx
	leaq	3(%rdx), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	movq	112(%rsp), %r8
	jae	.LBB36_618
	vmovss	12(%rax,%rdx,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1436(%rsp)
.Ltmp10877:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %r11
	je	.LBB36_328
.Ltmp10878:
	.loc	1 1403 42
	cmpq	$4, %rbx
	je	.LBB36_636
	movl	56(%r10), %edx
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rdx
.Ltmp10879:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rdx
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rdx
	.loc	1 1407 40
	imulq	%r11, %rdx
	leaq	4(%rdx), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	movq	112(%rsp), %r8
	jae	.LBB36_618
	vmovss	16(%rax,%rdx,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1440(%rsp)
.Ltmp10880:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %r11
	je	.LBB36_328
.Ltmp10881:
	.loc	1 1403 42
	cmpq	$5, %rbx
	je	.LBB36_638
	movl	68(%r10), %edx
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rdx
.Ltmp10882:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rdx
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rdx
	.loc	1 1407 40
	imulq	%r11, %rdx
	leaq	5(%rdx), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	movq	112(%rsp), %r8
	jae	.LBB36_618
	vmovss	20(%rax,%rdx,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1444(%rsp)
.Ltmp10883:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %r11
	je	.LBB36_328
.Ltmp10884:
	.loc	1 1403 42
	cmpq	$6, %rbx
	je	.LBB36_640
	movl	80(%r10), %edx
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rdx
.Ltmp10885:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rdx
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rdx
	.loc	1 1407 40
	imulq	%r11, %rdx
	leaq	6(%rdx), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	movq	112(%rsp), %r8
	jae	.LBB36_618
	vmovss	24(%rax,%rdx,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1448(%rsp)
.Ltmp10886:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %r11
	je	.LBB36_328
.Ltmp10887:
	.loc	1 1403 42
	cmpq	$7, %rbx
	je	.LBB36_644
	movl	92(%r10), %edx
	.loc	1 1403 28 is_stmt 0
	addq	8(%rsp), %rdx
.Ltmp10888:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rdx
	movl	$0, %r9d
	cmovaeq	%rcx, %r9
	subq	%r9, %rdx
	.loc	1 1407 40
	imulq	%r11, %rdx
	leaq	7(%rdx), %r9
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB36_618
	vmovss	28(%rax,%rdx,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1452(%rsp)
.Ltmp10889:
	.loc	1 0 13
.Ltmp10890:
	.p2align	4
.LBB36_328:
	movq	592(%rsp), %r9
	movq	64(%rsp), %r10
.LBB36_329:
.Ltmp10891:
	.loc	52 61 9 is_stmt 1
	vaddss	%xmm7, %xmm0, %xmm1
.Ltmp10892:
	.loc	52 66 9
	vsubss	%xmm9, %xmm1, %xmm7
.Ltmp10893:
	.loc	5 580 12
	cmpq	%rsi, %rdi
	ja	.LBB36_459
.Ltmp10894:
	.loc	52 56 9
	je	.LBB36_612
.Ltmp10895:
	.loc	1 1412 0
	movq	384(%r12), %rax
.Ltmp10896:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp10897:
	.loc	52 76 9
	vdivss	328(%rsp), %xmm7, %xmm0
.Ltmp10898:
	.loc	52 66 9
	vsubss	%xmm0, %xmm2, %xmm0
.Ltmp10899:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm3, %xmm0, %xmm1
.Ltmp10900:
	.loc	52 92 9 is_stmt 1
	vmulss	36(%rsp), %xmm1, %xmm1
	vaddss	%xmm1, %xmm3, %xmm1
.Ltmp10901:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp10902:
	.loc	52 103 24
	vandps	%xmm0, %xmm10, %xmm1
.Ltmp10903:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm1, %xmm1
	vandps	%xmm0, %xmm1, %xmm3
.Ltmp10904:
	.loc	1 1417 5
	vmovss	%xmm3, 576(%rsp)
.Ltmp10905:
	.loc	1 1420 28
	movq	360(%r12), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	368(%rsp), %r11
.Ltmp10906:
	.loc	5 568 12 is_stmt 1
	cmpq	%rsi, %r11
	ja	.LBB36_461
.Ltmp10907:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_613
	movq	176(%rsp), %rdi
	incq	%rdi
	vxorps	%xmm2, %xmm2, %xmm2
	vmovss	344(%rsp), %xmm0
.Ltmp10908:
	vmaxss	%xmm2, %xmm0, %xmm10
	vcmpltss	%xmm0, %xmm2, %xmm0
	vandps	%xmm0, %xmm12, %xmm12
	vmovss	232(%rsp), %xmm0
	vmaxss	%xmm2, %xmm0, %xmm9
	vcmpltss	%xmm0, %xmm2, %xmm0
	vandps	%xmm0, %xmm11, %xmm11
.Ltmp10909:
	vmaxss	%xmm2, %xmm13, %xmm1
	vcmpltss	%xmm13, %xmm2, %xmm0
	vandps	%xmm0, %xmm14, %xmm14
	vmaxss	%xmm2, %xmm5, %xmm0
	vcmpltss	%xmm5, %xmm2, %xmm2
	vandps	%xmm4, %xmm2, %xmm4
.Ltmp10910:
	vmovss	.LCPI36_2(%rip), %xmm2
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp10911:
	.loc	1 1420 0 is_stmt 1
	movq	352(%r12), %rax
.Ltmp10912:
	.loc	52 51 9
	vmovss	(%rax,%r11,4), %xmm5
.Ltmp10913:
	.loc	52 56 9
	vmovss	%xmm6, (%rax,%r11,4)
	vxorps	%xmm6, %xmm6, %xmm6
.Ltmp10914:
	.loc	52 71 9
	vmulss	%xmm5, %xmm2, %xmm2
.Ltmp10915:
	.loc	41 1244 18
	vmovd	%xmm5, %eax
.Ltmp10916:
	.loc	52 161 24
	andl	488(%rsp), %eax
.Ltmp10917:
	.loc	41 1244 18
	vmovd	%xmm2, %edx
.Ltmp10918:
	.loc	52 161 44
	andl	248(%rsp), %edx
	.loc	52 161 24 is_stmt 0
	orl	%eax, %edx
	movq	432(%rsp), %rax
.Ltmp10919:
	.loc	52 56 9 is_stmt 1
	movl	%edx, (%rbp,%rax,4)
	movq	368(%rsp), %rax
.Ltmp10920:
	.loc	1 1878 13
	incq	%rax
	.loc	1 1879 16
	cmpq	216(%rsp), %rax
	movl	$0, %esi
	cmoveq	%rsi, %rax
	movq	%rax, 368(%rsp)
	movq	8(%rsp), %rax
	.loc	1 1882 13
	incq	%rax
	.loc	1 1883 16
	cmpq	%rcx, %rax
	movl	$0, %edx
	movq	%rdx, 144(%rsp)
	cmoveq	%rsi, %rax
	movq	%rax, 8(%rsp)
	movq	%rdi, 176(%rsp)
.Ltmp10921:
	.loc	10 1916 50
	cmpq	%r10, %rdi
	movq	224(%rsp), %rsi
	movq	128(%rsp), %rax
.Ltmp10922:
	.loc	3 900 12
	jne	.LBB36_213
.Ltmp10923:
	.loc	1 1411 5
	vmovss	%xmm8, 824(%rsp)
.Ltmp10924:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp10925:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm10, 800(%rsp)
	vmovss	168(%rsp), %xmm2
.Ltmp10926:
	.loc	1 853 0
	vmovss	%xmm2, 788(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm12, 796(%rsp)
.Ltmp10927:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm9, 816(%rsp)
	vmovss	704(%rsp), %xmm2
.Ltmp10928:
	.loc	1 853 0
	vmovss	%xmm2, 804(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm11, 812(%rsp)
.Ltmp10929:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm1, 556(%rsp)
	vmovss	256(%rsp), %xmm1
.Ltmp10930:
	.loc	1 853 0
	vmovss	%xmm1, 544(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm14, 552(%rsp)
.Ltmp10931:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm0, 572(%rsp)
	vmovss	36(%rsp), %xmm0
.Ltmp10932:
	.loc	1 853 0
	vmovss	%xmm0, 560(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm4, 568(%rsp)
	movq	120(%rsp), %rcx
	movq	304(%rsp), %rdx
	jmp	.LBB36_199
.Ltmp10933:
.LBB36_335:
	.loc	1 1889 5 is_stmt 1
	vmovups	740(%rsp), %ymm0
	vmovups	772(%rsp), %ymm1
	vmovups	800(%rsp), %ymm2
	vmovups	%ymm2, 908(%rsp)
	vmovups	%ymm1, 880(%rsp)
	vmovups	%ymm0, 848(%rsp)
	leaq	848(%rsp), %rdi
	movq	952(%rsp), %rsi
	.loc	1 1889 14 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	.loc	1 1890 5 is_stmt 1
	vmovups	496(%rsp), %ymm0
	vmovups	528(%rsp), %ymm1
	vmovups	556(%rsp), %ymm2
	vmovups	%ymm2, 908(%rsp)
	vmovups	%ymm1, 880(%rsp)
	vmovups	%ymm0, 848(%rsp)
	leaq	848(%rsp), %rdi
	movq	944(%rsp), %rsi
	.loc	1 1890 15 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	368(%rsp), %rax
	.loc	1 1891 5 is_stmt 1
	jmp	.LBB36_337
.Ltmp10934:
.LBB36_336:
	.loc	1 0 5 is_stmt 0
	leaq	1240(%rsp), %rdi
	movq	952(%rsp), %rsi
.Ltmp10935:
	.loc	1 1889 14 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	leaq	1332(%rsp), %rdi
	movq	944(%rsp), %rsi
	.loc	1 1890 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	432(%rsp), %rax
.Ltmp10936:
.LBB36_337:
	.loc	1 0 0 is_stmt 0
	movl	%eax, 560(%r12)
	movq	8(%rsp), %rax
	movl	%eax, 564(%r12)
.Ltmp10937:
	.loc	1 2255 35 is_stmt 1
	cmpb	$0, 1532(%rsp)
	je	.LBB36_349
.LBB36_340:
	.loc	1 0 35 is_stmt 0
	movq	952(%rsp), %rdi
	.loc	1 2256 26 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2256 16 is_stmt 0
	testb	%al, %al
	je	.LBB36_349
	.loc	1 0 16
	movq	944(%rsp), %rdi
	.loc	1 2257 27 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2257 16 is_stmt 0
	testb	%al, %al
	je	.LBB36_349
	.loc	1 0 16
	movq	120(%rsp), %rax
	cmpq	%r15, %rax
	movq	136(%rsp), %rsi
.Ltmp10938:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB36_621
.Ltmp10939:
	.loc	4 0 16 is_stmt 0
	movq	%r13, %rcx
	.p2align	4
.LBB36_344:
.Ltmp10940:
	.loc	15 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB36_432
.Ltmp10941:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp10942:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%esi, %esi
	xorl	%r8d, %r8d
.Ltmp10943:
	.loc	16 0 18 is_stmt 0
.Ltmp10944:
	.p2align	4
.LBB36_346:
	.loc	17 134 13 is_stmt 1
	orl	(%rcx,%r8), %esi
.Ltmp10945:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp10946:
	.loc	6 180 28
	jne	.LBB36_346
.Ltmp10947:
	.loc	18 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp10948:
	.loc	19 2054 74
	subq	%rdx, %rax
.Ltmp10949:
	.loc	17 136 12
	testl	%esi, %esi
	movq	136(%rsp), %rsi
	je	.LBB36_344
.Ltmp10950:
	.loc	17 0 12 is_stmt 0
	xorl	%eax, %eax
	jmp	.LBB36_351
.LBB36_364:
.Ltmp10951:
	.loc	1 1696 12 is_stmt 1
	testb	%dl, %dl
	je	.LBB36_421
	.loc	1 0 12 is_stmt 0
	leaq	984(%rsp), %rdi
	leaq	136(%r12), %rsi
.Ltmp10952:
	.loc	1 1943 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	1076(%rsp), %rdi
	leaq	336(%r12), %rsi
.Ltmp10953:
	.loc	1 1944 25
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp10954:
	.loc	1 1949 19
	movzbl	776(%r12), %eax
	movl	%eax, 36(%rsp)
.Ltmp10955:
	.loc	1 1950 21
	movzbl	777(%r12), %eax
	movl	%eax, 64(%rsp)
	movq	%r12, %r14
.Ltmp10956:
	.loc	1 1951 16
	movq	544(%r12), %r12
.Ltmp10957:
	.loc	1 1952 16
	movq	552(%r14), %rbx
.Ltmp10958:
	.loc	1 1953 27
	movl	560(%r14), %eax
	movq	%rax, 728(%rsp)
.Ltmp10959:
	.loc	1 1954 27
	movl	564(%r14), %eax
	movq	%rax, 200(%rsp)
	leaq	2648(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	1616(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	496(%rsp), %rdi
	movq	952(%rsp), %rsi
.Ltmp10960:
	.loc	1 1961 32
	movq	%r12, %rdx
	movq	%rbx, 1600(%rsp)
	movq	%rbx, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
.Ltmp10961:
	.loc	1 1962 33
	movq	544(%r14), %rdx
	movq	552(%r14), %rcx
	leaq	848(%rsp), %rdi
	movq	944(%rsp), %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
	movq	120(%rsp), %rcx
.Ltmp10962:
	.loc	8 446 20
	testq	%rcx, %rcx
	je	.LBB36_439
.Ltmp10963:
	.loc	8 0 20 is_stmt 0
	movl	36(%rsp), %edx
	movl	%edx, %eax
	negl	%eax
	movl	%eax, 632(%rsp)
	movl	64(%rsp), %r10d
.Ltmp10964:
	movl	%r10d, %eax
	negl	%eax
	movl	%eax, 592(%rsp)
.Ltmp10965:
	.loc	4 3756 21 is_stmt 1
	movq	%rcx, %r11
	shrq	$5, %r11
.Ltmp10966:
	.loc	4 3757 21
	movl	%ecx, %eax
	andl	$31, %eax
.Ltmp10967:
	.loc	4 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %r11
	decl	%edx
	movl	%edx, 36(%rsp)
	decl	%r10d
	vmovss	984(%rsp), %xmm5
	vmovss	988(%rsp), %xmm13
	vmovss	992(%rsp), %xmm0
	vmovss	%xmm0, 1212(%rsp)
	vmovss	996(%rsp), %xmm11
	vmovss	1000(%rsp), %xmm8
	vmovss	1004(%rsp), %xmm0
	vmovss	%xmm0, 1208(%rsp)
	vmovss	1008(%rsp), %xmm0
	vmovaps	%xmm0, 1568(%rsp)
	vmovss	1012(%rsp), %xmm0
	vmovss	%xmm0, 1204(%rsp)
	vmovss	1016(%rsp), %xmm0
	vmovss	%xmm0, 1200(%rsp)
	vmovss	1020(%rsp), %xmm0
	vmovss	%xmm0, 1196(%rsp)
	vmovss	1024(%rsp), %xmm0
	vmovss	%xmm0, 1192(%rsp)
	vmovss	1028(%rsp), %xmm0
	vmovss	%xmm0, 1228(%rsp)
	vmovss	1076(%rsp), %xmm10
	vmovss	1080(%rsp), %xmm14
	vmovss	1084(%rsp), %xmm12
	vmovss	1088(%rsp), %xmm0
	vmovss	%xmm0, 1188(%rsp)
	movl	572(%rsp), %r9d
	movl	924(%rsp), %edi
	movl	$32, %edx
	movq	%rbp, %rsi
	movq	%r13, 1536(%rsp)
	movq	%rcx, %r8
	xorl	%eax, %eax
	vmovss	1092(%rsp), %xmm7
	vmovss	1096(%rsp), %xmm0
	vmovss	%xmm0, 1184(%rsp)
	vmovss	1100(%rsp), %xmm0
	vmovaps	%xmm0, 1552(%rsp)
	vmovss	1104(%rsp), %xmm9
	vmovss	1108(%rsp), %xmm15
	vmovss	1112(%rsp), %xmm1
	vmovss	1116(%rsp), %xmm2
	vmovss	1120(%rsp), %xmm3
	vmovss	568(%rsp), %xmm0
	vmovss	%xmm0, 836(%rsp)
	vmovss	920(%rsp), %xmm0
	vmovss	%xmm0, 832(%rsp)
	vmovss	1068(%rsp), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	1064(%rsp), %xmm0
	vmovss	1160(%rsp), %xmm4
	vmovss	1156(%rsp), %xmm6
	movq	200(%rsp), %rbx
	movl	%r10d, 64(%rsp)
.Ltmp10968:
.LBB36_367:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, 1512(%rsp)
.Ltmp10969:
	.loc	4 2584 13 is_stmt 1
	cmpq	$32, %r8
	movl	$32, %esi
	movq	%r8, 1592(%rsp)
	cmovbq	%r8, %rsi
	cmpq	$1, %rsi
	movq	%rsi, 1608(%rsp)
	movq	%rsi, %r8
	adcq	$0, %r8
.Ltmp10970:
	.loc	1 1967 55
	movq	%rcx, %rsi
	subq	%rax, %rsi
.Ltmp10971:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%rdx, %rsi
	movq	%rsi, 1544(%rsp)
	movq	%rax, 616(%rsp)
.Ltmp10972:
	.loc	1 1972 39
	addq	%rax, %rsi
.Ltmp10973:
	.loc	4 1050 16
	jb	.LBB36_620
	cmpq	%r15, %rsi
	ja	.LBB36_620
.Ltmp10974:
	.loc	4 0 16 is_stmt 0
	vmovss	%xmm5, 8(%rsp)
	vmovss	%xmm3, 1180(%rsp)
	vmovss	%xmm2, 1168(%rsp)
	vmovss	%xmm1, 144(%rsp)
	vmovss	%xmm15, 960(%rsp)
	vmovss	%xmm9, 128(%rsp)
	vmovss	%xmm7, 364(%rsp)
	vmovss	%xmm12, 48(%rsp)
	vmovss	%xmm0, 1232(%rsp)
	vmovss	%xmm4, 104(%rsp)
	vmovss	%xmm6, 96(%rsp)
	vmovss	%xmm14, 1236(%rsp)
	vmovss	%xmm10, 16(%rsp)
.Ltmp10975:
	.loc	11 304 12 is_stmt 1
	cmpq	616(%rsp), %rcx
	jne	.LBB36_371
	.loc	11 0 12 is_stmt 0
	vmovaps	%xmm8, %xmm10
	vmovaps	%xmm13, %xmm8
	vmovss	1212(%rsp), %xmm14
	vmovss	1208(%rsp), %xmm15
	vmovaps	1568(%rsp), %xmm9
	vmovss	1204(%rsp), %xmm4
	vmovss	1200(%rsp), %xmm13
	vmovss	1196(%rsp), %xmm12
	vmovss	1192(%rsp), %xmm6
	vmovss	1228(%rsp), %xmm5
	.loc	11 304 12
	jmp	.LBB36_373
.Ltmp10976:
.LBB36_371:
	.loc	11 0 12
	movq	696(%rsp), %rdx
	vmovss	584(%rdx), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	588(%rdx), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	592(%rdx), %xmm0
	vmovss	%xmm0, 624(%rsp)
	vmovss	596(%rdx), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	600(%rdx), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	604(%rdx), %xmm0
	vmovss	%xmm0, 152(%rsp)
	xorl	%ecx, %ecx
	vmovss	1192(%rsp), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	1196(%rsp), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	1200(%rsp), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	1204(%rsp), %xmm0
	vmovss	%xmm0, 176(%rsp)
	vmovaps	1568(%rsp), %xmm3
	vmovss	%xmm3, 704(%rsp)
	vmovss	1208(%rsp), %xmm1
	vmovaps	%xmm8, %xmm0
	vmovaps	%xmm11, %xmm2
	vmovss	1212(%rsp), %xmm7
	vmovaps	%xmm13, %xmm3
	vmovss	608(%rdx), %xmm4
	vmovss	%xmm4, 240(%rsp)
	vmovss	612(%rdx), %xmm4
	vmovss	%xmm4, 352(%rsp)
	vmovss	616(%rdx), %xmm4
	vmovss	%xmm4, 344(%rsp)
	vmovss	620(%rdx), %xmm4
	vmovss	%xmm4, 232(%rsp)
	vmovss	624(%rdx), %xmm4
	vmovss	%xmm4, 168(%rsp)
	vmovss	628(%rdx), %xmm4
	vmovss	%xmm4, 224(%rsp)
	vmovss	632(%rdx), %xmm4
	vmovss	%xmm4, 336(%rsp)
	vmovss	636(%rdx), %xmm4
	vmovss	%xmm4, 328(%rsp)
	vmovss	640(%rdx), %xmm4
	vmovss	%xmm4, 216(%rsp)
	vmovss	644(%rdx), %xmm4
	vmovss	%xmm4, 320(%rsp)
	vmovss	648(%rdx), %xmm4
	vmovss	%xmm4, 208(%rsp)
	vmovss	652(%rdx), %xmm4
	vmovss	%xmm4, 60(%rsp)
	vmovss	656(%rdx), %xmm4
	vmovss	%xmm4, 312(%rsp)
	vmovss	660(%rdx), %xmm4
	vmovss	%xmm4, 304(%rsp)
	vmovss	664(%rdx), %xmm4
	vmovss	%xmm4, 480(%rsp)
	vmovss	668(%rdx), %xmm4
	vmovss	%xmm4, 472(%rsp)
	vmovss	672(%rdx), %xmm4
	vmovss	%xmm4, 424(%rsp)
	vmovss	676(%rdx), %xmm4
	vmovss	%xmm4, 416(%rsp)
	vmovss	680(%rdx), %xmm4
	vmovss	%xmm4, 76(%rsp)
	vmovss	684(%rdx), %xmm4
	vmovss	%xmm4, 464(%rsp)
	vmovss	688(%rdx), %xmm4
	vmovss	%xmm4, 456(%rsp)
	vmovss	692(%rdx), %xmm4
	vmovss	%xmm4, 296(%rsp)
	vmovss	696(%rdx), %xmm4
	vmovss	%xmm4, 200(%rsp)
	vmovss	700(%rdx), %xmm4
	vmovss	%xmm4, 408(%rsp)
	vmovss	704(%rdx), %xmm4
	vmovss	%xmm4, 288(%rsp)
	vmovss	708(%rdx), %xmm4
	vmovss	%xmm4, 400(%rsp)
	vmovss	712(%rdx), %xmm4
	vmovss	%xmm4, 280(%rsp)
	vmovss	716(%rdx), %xmm4
	vmovss	%xmm4, 4(%rsp)
	vmovss	720(%rdx), %xmm4
	vmovss	%xmm4, 80(%rsp)
	vmovss	724(%rdx), %xmm4
	vmovss	%xmm4, 392(%rsp)
	vmovss	728(%rdx), %xmm4
	vmovss	%xmm4, 688(%rsp)
	vmovss	732(%rdx), %xmm4
	vmovss	%xmm4, 680(%rsp)
	vmovss	736(%rdx), %xmm4
	vmovss	%xmm4, 672(%rsp)
	vmovss	740(%rdx), %xmm4
	vmovss	%xmm4, 32(%rsp)
	vmovss	744(%rdx), %xmm4
	vmovss	%xmm4, 28(%rsp)
	vmovss	748(%rdx), %xmm4
	vmovss	%xmm4, 24(%rsp)
	vmovss	752(%rdx), %xmm4
	vmovss	%xmm4, 664(%rsp)
	vmovss	756(%rdx), %xmm4
	vmovss	%xmm4, 656(%rsp)
	vmovss	760(%rdx), %xmm4
	vmovss	%xmm4, 648(%rsp)
	vmovss	764(%rdx), %xmm4
	vmovss	%xmm4, 640(%rsp)
	vmovss	768(%rdx), %xmm4
	vmovss	%xmm4, 92(%rsp)
	vmovss	772(%rdx), %xmm4
	vmovss	%xmm4, 448(%rsp)
	movq	1536(%rsp), %rdx
	.p2align	4
.LBB36_372:
	vmovss	8(%rsp), %xmm6
.Ltmp10977:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rdx,%rcx,4), %xmm9
	vmovss	%xmm9, 8(%rsp)
.Ltmp10978:
	.loc	52 71 9
	vmulss	160(%rsp), %xmm9, %xmm14
	vxorps	%xmm5, %xmm5, %xmm5
.Ltmp10979:
	.loc	52 61 9
	vaddss	%xmm5, %xmm14, %xmm14
.Ltmp10980:
	.loc	52 71 9
	vmulss	112(%rsp), %xmm9, %xmm12
.Ltmp10981:
	.loc	52 61 9
	vaddss	%xmm5, %xmm12, %xmm12
.Ltmp10982:
	.loc	52 71 9
	vmulss	624(%rsp), %xmm9, %xmm4
.Ltmp10983:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
	vmovss	%xmm6, 40(%rsp)
.Ltmp10984:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm6, %xmm8
.Ltmp10985:
	.loc	52 61 9
	vaddss	%xmm8, %xmm14, %xmm8
.Ltmp10986:
	.loc	52 71 9
	vmulss	152(%rsp), %xmm6, %xmm14
.Ltmp10987:
	.loc	52 61 9
	vaddss	%xmm14, %xmm12, %xmm12
.Ltmp10988:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm6, %xmm14
.Ltmp10989:
	.loc	52 61 9
	vaddss	%xmm4, %xmm14, %xmm4
.Ltmp10990:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm9, %xmm14
.Ltmp10991:
	.loc	52 61 9
	vaddss	%xmm5, %xmm14, %xmm14
.Ltmp10992:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm6, %xmm9
.Ltmp10993:
	.loc	52 61 9
	vaddss	%xmm9, %xmm14, %xmm9
	vmovaps	%xmm3, %xmm14
.Ltmp10994:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm3, %xmm10
.Ltmp10995:
	.loc	52 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp10996:
	.loc	52 71 9
	vmulss	232(%rsp), %xmm3, %xmm10
.Ltmp10997:
	.loc	52 61 9
	vaddss	%xmm10, %xmm12, %xmm10
.Ltmp10998:
	.loc	52 71 9
	vmulss	168(%rsp), %xmm3, %xmm12
.Ltmp10999:
	.loc	52 61 9
	vaddss	%xmm4, %xmm12, %xmm4
.Ltmp11000:
	.loc	52 71 9
	vmulss	224(%rsp), %xmm3, %xmm12
.Ltmp11001:
	.loc	52 61 9
	vaddss	%xmm12, %xmm9, %xmm9
	vmovaps	%xmm7, %xmm11
.Ltmp11002:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm7, %xmm7
.Ltmp11003:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
.Ltmp11004:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm11, %xmm8
.Ltmp11005:
	.loc	52 61 9
	vaddss	%xmm8, %xmm10, %xmm8
.Ltmp11006:
	.loc	52 71 9
	vmulss	216(%rsp), %xmm11, %xmm10
.Ltmp11007:
	.loc	52 61 9
	vaddss	%xmm4, %xmm10, %xmm4
.Ltmp11008:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm11, %xmm10
.Ltmp11009:
	.loc	52 61 9
	vaddss	%xmm10, %xmm9, %xmm9
	vmovaps	%xmm2, %xmm10
.Ltmp11010:
	.loc	52 71 9
	vmulss	208(%rsp), %xmm2, %xmm6
.Ltmp11011:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
.Ltmp11012:
	.loc	52 71 9
	vmulss	60(%rsp), %xmm2, %xmm7
.Ltmp11013:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
.Ltmp11014:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm2, %xmm8
.Ltmp11015:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11016:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm2, %xmm8
.Ltmp11017:
	.loc	52 61 9
	vaddss	%xmm8, %xmm9, %xmm8
	vmovaps	%xmm0, %xmm15
.Ltmp11018:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm0, %xmm0
.Ltmp11019:
	.loc	52 61 9
	vaddss	%xmm0, %xmm6, %xmm0
.Ltmp11020:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm15, %xmm6
.Ltmp11021:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
.Ltmp11022:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm15, %xmm7
.Ltmp11023:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp11024:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm15, %xmm7
.Ltmp11025:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
	vbroadcastss	.LCPI36_0(%rip), %xmm8
	vmovaps	%xmm1, %xmm9
.Ltmp11026:
	.loc	52 71 9
	vmulss	76(%rsp), %xmm1, %xmm1
.Ltmp11027:
	.loc	52 61 9
	vaddss	%xmm1, %xmm0, %xmm0
.Ltmp11028:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm9, %xmm1
.Ltmp11029:
	.loc	52 61 9
	vaddss	%xmm1, %xmm6, %xmm1
.Ltmp11030:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm9, %xmm6
.Ltmp11031:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
.Ltmp11032:
	.loc	52 71 9
	vmulss	296(%rsp), %xmm9, %xmm6
.Ltmp11033:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
	vmovss	704(%rsp), %xmm7
.Ltmp11034:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm7, %xmm3
.Ltmp11035:
	.loc	52 61 9
	vaddss	%xmm3, %xmm0, %xmm0
.Ltmp11036:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm7, %xmm3
.Ltmp11037:
	.loc	52 61 9
	vaddss	%xmm3, %xmm1, %xmm1
.Ltmp11038:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm7, %xmm3
.Ltmp11039:
	.loc	52 61 9
	vaddss	%xmm3, %xmm4, %xmm3
.Ltmp11040:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm7, %xmm4
.Ltmp11041:
	.loc	52 61 9
	vaddss	%xmm4, %xmm6, %xmm4
	vmovss	176(%rsp), %xmm13
.Ltmp11042:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm13, %xmm5
.Ltmp11043:
	.loc	52 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp11044:
	.loc	52 71 9
	vmulss	4(%rsp), %xmm13, %xmm5
.Ltmp11045:
	.loc	52 61 9
	vaddss	%xmm5, %xmm1, %xmm1
.Ltmp11046:
	.loc	52 71 9
	vmulss	80(%rsp), %xmm13, %xmm5
.Ltmp11047:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp11048:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm13, %xmm5
.Ltmp11049:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
	vmovss	256(%rsp), %xmm12
.Ltmp11050:
	.loc	52 71 9
	vmulss	688(%rsp), %xmm12, %xmm5
.Ltmp11051:
	.loc	52 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp11052:
	.loc	52 71 9
	vmulss	680(%rsp), %xmm12, %xmm5
.Ltmp11053:
	.loc	52 61 9
	vaddss	%xmm5, %xmm1, %xmm1
.Ltmp11054:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm12, %xmm5
.Ltmp11055:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp11056:
	.loc	52 71 9
	vmulss	32(%rsp), %xmm12, %xmm5
.Ltmp11057:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
	vmovss	368(%rsp), %xmm6
.Ltmp11058:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm6, %xmm5
.Ltmp11059:
	.loc	52 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp11060:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm6, %xmm5
.Ltmp11061:
	.loc	52 61 9
	vaddss	%xmm5, %xmm1, %xmm1
.Ltmp11062:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm6, %xmm5
.Ltmp11063:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp11064:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm6, %xmm5
.Ltmp11065:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
	vmovss	432(%rsp), %xmm5
.Ltmp11066:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm5, %xmm2
.Ltmp11067:
	.loc	52 61 9
	vaddss	%xmm2, %xmm0, %xmm0
.Ltmp11068:
	.loc	52 71 9
	vmulss	640(%rsp), %xmm5, %xmm2
.Ltmp11069:
	.loc	52 61 9
	vaddss	%xmm2, %xmm1, %xmm1
.Ltmp11070:
	.loc	52 71 9
	vmulss	92(%rsp), %xmm5, %xmm2
.Ltmp11071:
	.loc	52 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp11072:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm5, %xmm3
.Ltmp11073:
	.loc	52 61 9
	vaddss	%xmm3, %xmm4, %xmm3
.Ltmp11074:
	.loc	52 103 24
	vandps	%xmm0, %xmm8, %xmm0
.Ltmp11075:
	.loc	52 103 24 is_stmt 0
	vandps	%xmm8, %xmm9, %xmm4
.Ltmp11076:
	.loc	52 161 24 is_stmt 1
	vmaxss	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm7, %xmm4
.Ltmp11077:
	.loc	52 103 24
	vandps	%xmm1, %xmm8, %xmm1
.Ltmp11078:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp11079:
	.loc	52 103 24
	vandps	%xmm2, %xmm8, %xmm1
.Ltmp11080:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp11081:
	.loc	52 103 24
	vandps	%xmm3, %xmm8, %xmm1
.Ltmp11082:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp11083:
	.loc	52 56 9
	vmovss	%xmm0, 2648(%rsp,%rcx,4)
.Ltmp11084:
	.loc	11 308 13
	incq	%rcx
	vmovss	%xmm6, 432(%rsp)
	vmovss	%xmm12, 368(%rsp)
	vmovss	%xmm13, 256(%rsp)
	vmovss	%xmm7, 176(%rsp)
	vmovss	%xmm9, 704(%rsp)
	vmovaps	%xmm15, %xmm1
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm11, %xmm2
	vmovaps	%xmm14, %xmm7
	vmovss	40(%rsp), %xmm3
	vmovaps	%xmm3, %xmm8
.Ltmp11085:
	.loc	11 304 12
	cmpq	%rcx, %r8
	jne	.LBB36_372
.Ltmp11086:
.LBB36_373:
	.loc	11 0 12 is_stmt 0
	vmovss	8(%rsp), %xmm0
	vmovss	%xmm8, 40(%rsp)
	vmovss	%xmm14, 1212(%rsp)
	.loc	1 1764 5 is_stmt 1
	vmovss	%xmm14, 992(%rsp)
	vmovss	%xmm11, 996(%rsp)
	vmovss	%xmm10, 1000(%rsp)
	vmovss	%xmm15, 1208(%rsp)
	vmovss	%xmm15, 1004(%rsp)
	vmovss	%xmm9, 1008(%rsp)
	vmovss	%xmm4, 1012(%rsp)
	vmovss	%xmm13, 1016(%rsp)
	vmovss	%xmm12, 1020(%rsp)
	vmovss	%xmm6, 1024(%rsp)
	vmovss	%xmm5, 1028(%rsp)
.Ltmp11087:
	.loc	5 438 16
	cmpq	136(%rsp), %rsi
	ja	.LBB36_624
.Ltmp11088:
	.loc	5 0 16 is_stmt 0
	vmovss	%xmm5, 1228(%rsp)
	vmovss	%xmm6, 1192(%rsp)
	vmovss	%xmm12, 1196(%rsp)
	vmovss	%xmm13, 1200(%rsp)
	vmovss	%xmm4, 1204(%rsp)
	vmovaps	%xmm9, 1568(%rsp)
	vmovss	%xmm10, 1524(%rsp)
	vmovss	%xmm11, 1528(%rsp)
	movq	616(%rsp), %rcx
.Ltmp11089:
	.loc	11 304 12 is_stmt 1
	cmpq	%rcx, 120(%rsp)
	vmovss	1236(%rsp), %xmm14
	vmovss	48(%rsp), %xmm11
.Ltmp11090:
	.loc	11 304 12 is_stmt 0
	jne	.LBB36_376
	.loc	11 0 12
	movl	$32, %edx
	movq	1512(%rsp), %rsi
	vmovss	1188(%rsp), %xmm12
	vmovss	364(%rsp), %xmm1
	vmovss	1184(%rsp), %xmm4
	vmovaps	1552(%rsp), %xmm8
	vmovss	128(%rsp), %xmm9
	vmovss	960(%rsp), %xmm15
	vmovss	144(%rsp), %xmm6
	vmovss	1168(%rsp), %xmm7
	vmovss	1180(%rsp), %xmm5
	.loc	11 304 12
	jmp	.LBB36_378
.Ltmp11091:
.LBB36_376:
	.loc	11 0 12
	movq	696(%rsp), %rdx
	vmovss	584(%rdx), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	588(%rdx), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	592(%rdx), %xmm0
	vmovss	%xmm0, 624(%rsp)
	vmovss	596(%rdx), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	600(%rdx), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	604(%rdx), %xmm0
	vmovss	%xmm0, 152(%rsp)
	xorl	%ecx, %ecx
	vmovss	1168(%rsp), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	144(%rsp), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	960(%rsp), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	128(%rsp), %xmm0
	vmovss	%xmm0, 176(%rsp)
	vmovaps	1552(%rsp), %xmm1
	vmovss	%xmm1, 704(%rsp)
	vmovss	1184(%rsp), %xmm0
	vmovss	364(%rsp), %xmm13
	vmovss	1188(%rsp), %xmm2
	vmovaps	%xmm11, %xmm3
	vmovaps	%xmm14, %xmm10
	vmovss	608(%rdx), %xmm4
	vmovss	%xmm4, 240(%rsp)
	vmovss	612(%rdx), %xmm4
	vmovss	%xmm4, 352(%rsp)
	vmovss	616(%rdx), %xmm4
	vmovss	%xmm4, 344(%rsp)
	vmovss	620(%rdx), %xmm4
	vmovss	%xmm4, 232(%rsp)
	vmovss	624(%rdx), %xmm4
	vmovss	%xmm4, 168(%rsp)
	vmovss	628(%rdx), %xmm4
	vmovss	%xmm4, 224(%rsp)
	vmovss	632(%rdx), %xmm4
	vmovss	%xmm4, 336(%rsp)
	vmovss	636(%rdx), %xmm4
	vmovss	%xmm4, 328(%rsp)
	vmovss	640(%rdx), %xmm4
	vmovss	%xmm4, 216(%rsp)
	vmovss	644(%rdx), %xmm4
	vmovss	%xmm4, 320(%rsp)
	vmovss	648(%rdx), %xmm4
	vmovss	%xmm4, 208(%rsp)
	vmovss	652(%rdx), %xmm4
	vmovss	%xmm4, 60(%rsp)
	vmovss	656(%rdx), %xmm4
	vmovss	%xmm4, 312(%rsp)
	vmovss	660(%rdx), %xmm4
	vmovss	%xmm4, 304(%rsp)
	vmovss	664(%rdx), %xmm4
	vmovss	%xmm4, 480(%rsp)
	vmovss	668(%rdx), %xmm4
	vmovss	%xmm4, 472(%rsp)
	vmovss	672(%rdx), %xmm4
	vmovss	%xmm4, 424(%rsp)
	vmovss	676(%rdx), %xmm4
	vmovss	%xmm4, 416(%rsp)
	vmovss	680(%rdx), %xmm4
	vmovss	%xmm4, 76(%rsp)
	vmovss	684(%rdx), %xmm4
	vmovss	%xmm4, 464(%rsp)
	vmovss	688(%rdx), %xmm4
	vmovss	%xmm4, 456(%rsp)
	vmovss	692(%rdx), %xmm4
	vmovss	%xmm4, 296(%rsp)
	vmovss	696(%rdx), %xmm4
	vmovss	%xmm4, 200(%rsp)
	vmovss	700(%rdx), %xmm4
	vmovss	%xmm4, 408(%rsp)
	vmovss	704(%rdx), %xmm4
	vmovss	%xmm4, 288(%rsp)
	vmovss	708(%rdx), %xmm4
	vmovss	%xmm4, 400(%rsp)
	vmovss	712(%rdx), %xmm4
	vmovss	%xmm4, 280(%rsp)
	vmovss	716(%rdx), %xmm4
	vmovss	%xmm4, 4(%rsp)
	vmovss	720(%rdx), %xmm4
	vmovss	%xmm4, 80(%rsp)
	vmovss	724(%rdx), %xmm4
	vmovss	%xmm4, 392(%rsp)
	vmovss	728(%rdx), %xmm4
	vmovss	%xmm4, 688(%rsp)
	vmovss	732(%rdx), %xmm4
	vmovss	%xmm4, 680(%rsp)
	vmovss	736(%rdx), %xmm4
	vmovss	%xmm4, 672(%rsp)
	vmovss	740(%rdx), %xmm4
	vmovss	%xmm4, 32(%rsp)
	vmovss	744(%rdx), %xmm4
	vmovss	%xmm4, 28(%rsp)
	vmovss	748(%rdx), %xmm4
	vmovss	%xmm4, 24(%rsp)
	vmovss	752(%rdx), %xmm4
	vmovss	%xmm4, 664(%rsp)
	vmovss	756(%rdx), %xmm4
	vmovss	%xmm4, 656(%rsp)
	vmovss	760(%rdx), %xmm4
	vmovss	%xmm4, 648(%rsp)
	vmovss	764(%rdx), %xmm4
	vmovss	%xmm4, 640(%rsp)
	vmovss	768(%rdx), %xmm4
	vmovss	%xmm4, 92(%rsp)
	vmovss	772(%rdx), %xmm4
	vmovss	%xmm4, 448(%rsp)
	movl	$32, %edx
	movq	1512(%rsp), %rsi
	.p2align	4
.LBB36_377:
	vmovss	16(%rsp), %xmm14
.Ltmp11092:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rsi,%rcx,4), %xmm5
.Ltmp11093:
	.loc	52 71 9
	vmulss	160(%rsp), %xmm5, %xmm8
	vxorps	%xmm4, %xmm4, %xmm4
.Ltmp11094:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm8
.Ltmp11095:
	.loc	52 71 9
	vmulss	112(%rsp), %xmm5, %xmm9
.Ltmp11096:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm9
.Ltmp11097:
	.loc	52 71 9
	vmulss	624(%rsp), %xmm5, %xmm15
.Ltmp11098:
	.loc	52 61 9
	vaddss	%xmm4, %xmm15, %xmm15
.Ltmp11099:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm14, %xmm12
.Ltmp11100:
	.loc	52 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp11101:
	.loc	52 71 9
	vmulss	152(%rsp), %xmm14, %xmm12
.Ltmp11102:
	.loc	52 61 9
	vaddss	%xmm12, %xmm9, %xmm9
.Ltmp11103:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm14, %xmm12
.Ltmp11104:
	.loc	52 61 9
	vaddss	%xmm12, %xmm15, %xmm12
	vmovss	%xmm5, 16(%rsp)
.Ltmp11105:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm5, %xmm15
.Ltmp11106:
	.loc	52 61 9
	vaddss	%xmm4, %xmm15, %xmm15
.Ltmp11107:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm14, %xmm4
.Ltmp11108:
	.loc	52 61 9
	vaddss	%xmm4, %xmm15, %xmm4
	vmovaps	%xmm10, %xmm1
	vmovss	%xmm10, 48(%rsp)
.Ltmp11109:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm10, %xmm10
.Ltmp11110:
	.loc	52 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp11111:
	.loc	52 71 9
	vmulss	232(%rsp), %xmm1, %xmm10
.Ltmp11112:
	.loc	52 61 9
	vaddss	%xmm10, %xmm9, %xmm9
.Ltmp11113:
	.loc	52 71 9
	vmulss	168(%rsp), %xmm1, %xmm10
.Ltmp11114:
	.loc	52 61 9
	vaddss	%xmm10, %xmm12, %xmm10
.Ltmp11115:
	.loc	52 71 9
	vmulss	224(%rsp), %xmm1, %xmm12
.Ltmp11116:
	.loc	52 61 9
	vaddss	%xmm4, %xmm12, %xmm4
	vmovaps	%xmm3, %xmm12
.Ltmp11117:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm3, %xmm7
.Ltmp11118:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
.Ltmp11119:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm3, %xmm8
.Ltmp11120:
	.loc	52 61 9
	vaddss	%xmm8, %xmm9, %xmm8
.Ltmp11121:
	.loc	52 71 9
	vmulss	216(%rsp), %xmm3, %xmm9
.Ltmp11122:
	.loc	52 61 9
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp11123:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm3, %xmm10
.Ltmp11124:
	.loc	52 61 9
	vaddss	%xmm4, %xmm10, %xmm4
.Ltmp11125:
	.loc	52 71 9
	vmulss	208(%rsp), %xmm2, %xmm6
.Ltmp11126:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
.Ltmp11127:
	.loc	52 71 9
	vmulss	60(%rsp), %xmm2, %xmm7
.Ltmp11128:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
.Ltmp11129:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm2, %xmm8
.Ltmp11130:
	.loc	52 61 9
	vaddss	%xmm8, %xmm9, %xmm8
.Ltmp11131:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm2, %xmm9
.Ltmp11132:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp11133:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm13, %xmm9
.Ltmp11134:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp11135:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm13, %xmm9
.Ltmp11136:
	.loc	52 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp11137:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm13, %xmm9
.Ltmp11138:
	.loc	52 61 9
	vaddss	%xmm9, %xmm8, %xmm8
.Ltmp11139:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm13, %xmm9
.Ltmp11140:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
	vmovaps	%xmm2, %xmm11
	vmovaps	%xmm0, %xmm10
.Ltmp11141:
	.loc	52 71 9
	vmulss	76(%rsp), %xmm0, %xmm0
.Ltmp11142:
	.loc	52 61 9
	vaddss	%xmm0, %xmm6, %xmm0
.Ltmp11143:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm10, %xmm6
.Ltmp11144:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
.Ltmp11145:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm10, %xmm7
.Ltmp11146:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
.Ltmp11147:
	.loc	52 71 9
	vmulss	296(%rsp), %xmm10, %xmm8
.Ltmp11148:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
	vbroadcastss	.LCPI36_0(%rip), %xmm8
	vmovss	704(%rsp), %xmm9
.Ltmp11149:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm9, %xmm1
.Ltmp11150:
	.loc	52 61 9
	vaddss	%xmm1, %xmm0, %xmm0
.Ltmp11151:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm9, %xmm1
.Ltmp11152:
	.loc	52 61 9
	vaddss	%xmm1, %xmm6, %xmm1
.Ltmp11153:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm9, %xmm6
.Ltmp11154:
	.loc	52 61 9
	vaddss	%xmm6, %xmm7, %xmm6
.Ltmp11155:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm9, %xmm7
.Ltmp11156:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
	vmovss	176(%rsp), %xmm15
.Ltmp11157:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm15, %xmm3
.Ltmp11158:
	.loc	52 61 9
	vaddss	%xmm3, %xmm0, %xmm0
.Ltmp11159:
	.loc	52 71 9
	vmulss	4(%rsp), %xmm15, %xmm3
.Ltmp11160:
	.loc	52 61 9
	vaddss	%xmm3, %xmm1, %xmm1
.Ltmp11161:
	.loc	52 71 9
	vmulss	80(%rsp), %xmm15, %xmm3
.Ltmp11162:
	.loc	52 61 9
	vaddss	%xmm3, %xmm6, %xmm3
.Ltmp11163:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm15, %xmm6
.Ltmp11164:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
	vmovss	256(%rsp), %xmm6
.Ltmp11165:
	.loc	52 71 9
	vmulss	688(%rsp), %xmm6, %xmm5
.Ltmp11166:
	.loc	52 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp11167:
	.loc	52 71 9
	vmulss	680(%rsp), %xmm6, %xmm5
.Ltmp11168:
	.loc	52 61 9
	vaddss	%xmm5, %xmm1, %xmm1
.Ltmp11169:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm6, %xmm5
.Ltmp11170:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp11171:
	.loc	52 71 9
	vmulss	32(%rsp), %xmm6, %xmm5
.Ltmp11172:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
	vmovss	368(%rsp), %xmm7
.Ltmp11173:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm7, %xmm5
.Ltmp11174:
	.loc	52 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp11175:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm7, %xmm5
.Ltmp11176:
	.loc	52 61 9
	vaddss	%xmm5, %xmm1, %xmm1
.Ltmp11177:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm7, %xmm5
.Ltmp11178:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp11179:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm7, %xmm5
.Ltmp11180:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
	vmovss	432(%rsp), %xmm5
.Ltmp11181:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm5, %xmm2
.Ltmp11182:
	.loc	52 61 9
	vaddss	%xmm2, %xmm0, %xmm0
.Ltmp11183:
	.loc	52 71 9
	vmulss	640(%rsp), %xmm5, %xmm2
.Ltmp11184:
	.loc	52 61 9
	vaddss	%xmm2, %xmm1, %xmm1
.Ltmp11185:
	.loc	52 71 9
	vmulss	92(%rsp), %xmm5, %xmm2
.Ltmp11186:
	.loc	52 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp11187:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm5, %xmm3
.Ltmp11188:
	.loc	52 61 9
	vaddss	%xmm3, %xmm4, %xmm3
.Ltmp11189:
	.loc	52 103 24
	vandps	%xmm0, %xmm8, %xmm0
.Ltmp11190:
	.loc	52 103 24 is_stmt 0
	vandps	%xmm8, %xmm10, %xmm4
.Ltmp11191:
	.loc	52 161 24 is_stmt 1
	vmaxss	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm13, %xmm4
.Ltmp11192:
	.loc	52 103 24
	vandps	%xmm1, %xmm8, %xmm1
.Ltmp11193:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp11194:
	.loc	52 103 24
	vandps	%xmm2, %xmm8, %xmm1
.Ltmp11195:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp11196:
	.loc	52 103 24
	vandps	%xmm3, %xmm8, %xmm1
	vmovaps	%xmm10, %xmm8
.Ltmp11197:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp11198:
	.loc	52 56 9
	vmovss	%xmm0, 1616(%rsp,%rcx,4)
.Ltmp11199:
	.loc	11 308 13
	incq	%rcx
	vmovss	%xmm7, 432(%rsp)
	vmovss	%xmm6, 368(%rsp)
	vmovss	%xmm15, 256(%rsp)
	vmovss	%xmm9, 176(%rsp)
	vmovss	%xmm10, 704(%rsp)
	vmovaps	%xmm13, %xmm0
	vmovaps	%xmm11, %xmm1
	vmovaps	%xmm11, %xmm13
	vmovaps	%xmm12, %xmm2
	vmovss	48(%rsp), %xmm3
	vmovaps	%xmm3, %xmm11
	vmovaps	%xmm14, %xmm10
.Ltmp11200:
	.loc	11 304 12
	cmpq	%rcx, %r8
	jne	.LBB36_377
.Ltmp11201:
.LBB36_378:
	.loc	11 0 12 is_stmt 0
	vmovss	16(%rsp), %xmm10
	vmovaps	%xmm11, %xmm0
	.loc	1 1764 5 is_stmt 1
	vmovss	%xmm11, 1084(%rsp)
	vmovss	%xmm12, 1188(%rsp)
	vmovss	%xmm12, 1088(%rsp)
	vmovaps	%xmm1, %xmm11
	vmovss	%xmm1, 1092(%rsp)
	vmovss	%xmm4, 1184(%rsp)
	vmovss	%xmm4, 1096(%rsp)
	vmovaps	%xmm8, 1552(%rsp)
	vmovss	%xmm8, 1100(%rsp)
	vmovss	%xmm9, 1104(%rsp)
	vmovss	%xmm15, 1108(%rsp)
	vmovss	%xmm6, 1112(%rsp)
	vmovss	%xmm7, 1116(%rsp)
	vmovss	%xmm5, 1120(%rsp)
	movq	120(%rsp), %rcx
	movq	616(%rsp), %rax
.Ltmp11202:
	.loc	11 304 12
	cmpq	%rax, %rcx
	vmovaps	%xmm6, %xmm1
	vmovaps	%xmm7, %xmm2
	vmovaps	%xmm5, %xmm3
	vmovaps	%xmm11, %xmm7
	vmovaps	%xmm0, %xmm12
.Ltmp11203:
	.loc	1 1984 19
	jne	.LBB36_381
.Ltmp11204:
	.loc	1 0 19 is_stmt 0
	vmovss	96(%rsp), %xmm6
	vmovss	8(%rsp), %xmm5
	vmovss	104(%rsp), %xmm4
	vmovss	40(%rsp), %xmm13
	vmovss	1232(%rsp), %xmm0
.LBB36_380:
	addq	$32, %rax
	decq	%r11
	movq	1592(%rsp), %r8
.Ltmp11205:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %r8
	subq	$-128, 1536(%rsp)
	subq	$-128, %rsi
	testq	%r11, %r11
	vmovss	1528(%rsp), %xmm11
	vmovss	1524(%rsp), %xmm8
	jne	.LBB36_367
	jmp	.LBB36_580
.Ltmp11206:
.LBB36_381:
	.loc	8 0 20 is_stmt 0
	vmovss	%xmm7, 364(%rsp)
	vmovss	%xmm3, 1180(%rsp)
	vmovss	%xmm2, 1168(%rsp)
	vmovss	%xmm1, 144(%rsp)
	vmovss	%xmm15, 960(%rsp)
	vmovss	%xmm9, 128(%rsp)
	vmovss	%xmm12, 48(%rsp)
	movq	%r11, 392(%rsp)
	movq	552(%rsp), %r11
	movq	560(%rsp), %rax
	movq	%rax, 288(%rsp)
	movl	$0, %r10d
	movq	904(%rsp), %r14
	movq	912(%rsp), %rax
	movq	%rax, 400(%rsp)
	movq	496(%rsp), %rcx
	movq	504(%rsp), %rdx
	movq	544(%rsp), %rsi
	movq	512(%rsp), %rax
	movq	%rax, 320(%rsp)
	movq	520(%rsp), %r8
	movq	536(%rsp), %rax
	movq	%rax, 216(%rsp)
	movq	528(%rsp), %rax
	movq	%rax, 312(%rsp)
	movq	848(%rsp), %r13
	movq	896(%rsp), %rax
.Ltmp11207:
	.loc	1 1984 19 is_stmt 1
	cmpq	$1, %rsi
	movq	%rsi, 168(%rsp)
	adcq	$0, %rsi
	movq	%rsi, 224(%rsp)
	cmpq	$1, %rax
	movq	%rax, 304(%rsp)
	adcq	$0, %rax
	movq	%rax, 480(%rsp)
	vmovss	1032(%rsp), %xmm0
	vmovss	%xmm0, 704(%rsp)
	vmovss	1048(%rsp), %xmm0
	vmovss	%xmm0, 208(%rsp)
	vmovss	1124(%rsp), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	1140(%rsp), %xmm0
	vmovss	%xmm0, 76(%rsp)
	vmovss	1072(%rsp), %xmm0
	vmovss	%xmm0, 60(%rsp)
	vmovss	1164(%rsp), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	832(%rsp), %xmm11
	vmovaps	%xmm11, %xmm9
	vmovss	836(%rsp), %xmm12
	vmovaps	%xmm12, %xmm8
	movq	856(%rsp), %rax
	movq	864(%rsp), %rsi
	movq	%rsi, 416(%rsp)
	movq	872(%rsp), %rsi
	movq	%rsi, 472(%rsp)
	movq	888(%rsp), %rsi
	movq	%rsi, 424(%rsp)
	movq	880(%rsp), %rsi
	movq	%rsi, 456(%rsp)
	movl	%edi, 4(%rsp)
	movl	%edi, %esi
	movq	%rsi, 432(%rsp)
	movl	%r9d, 80(%rsp)
	movl	%r9d, %esi
	movq	%rsi, 256(%rsp)
	xorl	%edi, %edi
	movq	728(%rsp), %r15
	vmovss	1232(%rsp), %xmm15
	vmovss	88(%rsp), %xmm0
	movq	1544(%rsp), %rsi
	movq	%r8, 240(%rsp)
	movq	%r11, 232(%rsp)
	movq	%r14, 280(%rsp)
.LBB36_382:
	.loc	1 0 19 is_stmt 0
	movq	%r15, 728(%rsp)
	movq	%rdi, 296(%rsp)
	movq	%rbx, %rbp
	.loc	1 1989 21 is_stmt 1
	movq	%rsi, %r15
	subq	%rdi, %r15
	movq	696(%rsp), %rsi
.Ltmp11208:
	.loc	1 1575 16
	movq	544(%rsi), %r9
.Ltmp11209:
	.loc	1 1576 16
	movq	552(%rsi), %rbx
.Ltmp11210:
	.loc	1 1577 25
	leaq	1(%rbp), %rsi
.Ltmp11211:
	.loc	1 1148 8
	cmpq	%r9, %rsi
	movq	%r9, %rsi
	cmovbq	%r10, %rsi
	negq	%rsi
.Ltmp11212:
	.loc	1 1578 28
	addq	%rbp, %r11
.Ltmp11213:
	.loc	1 1148 8
	cmpq	%r9, %r11
	movq	%r9, %r8
	cmovbq	%r10, %r8
	subq	%r8, %r11
.Ltmp11214:
	.loc	1 1579 29
	leaq	(%r14,%rbp), %rdi
.Ltmp11215:
	.loc	1 1148 8
	cmpq	%r9, %rdi
	movq	%r9, %r8
	cmovbq	%r10, %r8
	subq	%r8, %rdi
	movq	%rdi, 112(%rsp)
	movq	288(%rsp), %rdi
.Ltmp11216:
	.loc	1 1580 33
	leaq	(%rdi,%rbp), %r14
.Ltmp11217:
	.loc	1 1148 8
	cmpq	%r9, %r14
	movq	%r9, %r8
	cmovbq	%r10, %r8
	subq	%r8, %r14
	movq	400(%rsp), %rdi
.Ltmp11218:
	.loc	1 1581 34
	addq	%rbp, %rdi
	movq	%rbp, 200(%rsp)
.Ltmp11219:
	.loc	1 1148 8
	cmpq	%r9, %rdi
	movq	%r9, %r8
	cmovbq	%r10, %r8
.Ltmp11220:
	.loc	1 1148 8 is_stmt 0
	addq	%rbp, %rsi
	incq	%rsi
.Ltmp11221:
	.loc	1 1148 8
	subq	%r8, %rdi
.Ltmp11222:
	.loc	1 1583 14 is_stmt 1
	movq	%r9, %r10
	subq	%rbp, %r10
.Ltmp11223:
	.loc	10 1078 5
	cmpq	%r15, %r10
	cmovbq	%r10, %r15
.Ltmp11224:
	.loc	1 1584 14
	subq	728(%rsp), %rbx
.Ltmp11225:
	.loc	10 1078 5
	cmpq	%r15, %rbx
	movq	%rbx, 176(%rsp)
	cmovbq	%rbx, %r15
.Ltmp11226:
	.loc	1 1585 14
	movq	%r9, %rbx
	movq	%rsi, 248(%rsp)
	subq	%rsi, %rbx
.Ltmp11227:
	.loc	10 1078 5
	cmpq	%r15, %rbx
	cmovbq	%rbx, %r15
.Ltmp11228:
	.loc	1 1586 14
	movq	%r9, %rbp
	movq	%r11, 624(%rsp)
	subq	%r11, %rbp
.Ltmp11229:
	.loc	10 1078 5
	cmpq	%r15, %rbp
	cmovbq	%rbp, %r15
.Ltmp11230:
	.loc	1 1587 14
	movq	%r9, %r11
	movq	112(%rsp), %rsi
	subq	%rsi, %r11
.Ltmp11231:
	.loc	10 1078 5
	cmpq	%r15, %r11
	cmovbq	%r11, %r15
.Ltmp11232:
	.loc	1 1588 14
	movq	%r9, %r8
	movq	%r14, 488(%rsp)
	subq	%r14, %r8
.Ltmp11233:
	.loc	10 1078 5
	cmpq	%r15, %r8
	cmovbq	%r8, %r15
	movq	%rdi, 152(%rsp)
.Ltmp11234:
	.loc	1 1589 14
	subq	%rdi, %r9
.Ltmp11235:
	.loc	10 1078 5
	cmpq	%r15, %r9
	cmovbq	%r9, %r15
	movq	616(%rsp), %rsi
	movq	296(%rsp), %rdi
.Ltmp11236:
	.loc	1 1995 28
	addq	%rsi, %rdi
	movq	%r15, 408(%rsp)
.Ltmp11237:
	.loc	1 1997 55
	addq	%rdi, %r15
	movq	200(%rsp), %r14
.Ltmp11238:
	.loc	4 1050 16
	jb	.LBB36_615
	cmpq	968(%rsp), %r15
	ja	.LBB36_615
.Ltmp11239:
	.loc	5 451 16
	cmpq	136(%rsp), %r15
	ja	.LBB36_622
.Ltmp11240:
	.loc	11 304 12
	cmpq	$0, 408(%rsp)
	movq	728(%rsp), %r15
	je	.LBB36_418
.Ltmp11241:
	.loc	11 0 12 is_stmt 0
	movq	976(%rsp), %rsi
	leaq	(%rsi,%rdi,4), %rsi
	movq	%rsi, 368(%rsp)
.Ltmp11242:
	.loc	11 304 12
	cmpq	%r11, %rbp
	cmovbq	%rbp, %r11
	movq	840(%rsp), %rsi
.Ltmp11243:
	.loc	1 0 0
	leaq	(%rsi,%rdi,4), %rdi
.Ltmp11244:
	.loc	11 304 12
	cmpq	%r8, %r11
	cmovaeq	%r8, %r11
	movq	296(%rsp), %rsi
.Ltmp11245:
	.loc	1 0 0
	leaq	(%rsp,%rsi,4), %r8
	addq	$2648, %r8
	movq	%r8, 352(%rsp)
.Ltmp11246:
	.loc	11 304 12
	cmpq	%r9, %r11
	cmovaeq	%r9, %r11
.Ltmp11247:
	.loc	1 0 0
	leaq	(%rsp,%rsi,4), %r8
	addq	$1616, %r8
	movq	%r8, 344(%rsp)
.Ltmp11248:
	.loc	11 304 12
	cmpq	%rbx, %r11
	cmovaeq	%rbx, %r11
	cmpq	%r10, %r11
	cmovaeq	%r10, %r11
	movq	176(%rsp), %r8
	cmpq	%r8, %r11
	cmovaeq	%r8, %r11
	movq	1608(%rsp), %r8
	subq	%rsi, %r8
	cmpq	%r8, %r11
	cmovbq	%r11, %r8
	movq	%r8, 328(%rsp)
	vmovss	96(%rsp), %xmm6
	vmovss	104(%rsp), %xmm4
	vmovaps	%xmm15, %xmm10
	vmovss	%xmm0, 88(%rsp)
	vmovaps	%xmm0, %xmm7
	xorl	%r9d, %r9d
	movq	240(%rsp), %r8
	movq	%rdi, 336(%rsp)
.Ltmp11249:
	.loc	11 0 12
.Ltmp11250:
	.p2align	4
.LBB36_387:
	movq	352(%rsp), %rsi
.Ltmp11251:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rsi,%r9,4), %xmm0
	movq	344(%rsp), %rsi
.Ltmp11252:
	.loc	52 51 9 is_stmt 0
	vmovss	(%rsi,%r9,4), %xmm1
.Ltmp11253:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
.Ltmp11254:
	.loc	41 1244 18
	vmovd	%xmm1, %ebp
.Ltmp11255:
	.loc	41 1244 18 is_stmt 0
	vmovd	%xmm0, %esi
.Ltmp11256:
	.loc	52 161 24 is_stmt 1
	movl	%ebp, %ebx
	cmovbel	%esi, %ebx
.Ltmp11257:
	.loc	1 1502 26
	leaq	(%r9,%r14), %r10
.Ltmp11258:
	.loc	4 1050 16
	cmpq	%rdx, %r10
	jae	.LBB36_584
.Ltmp11259:
	.loc	1 0 0 is_stmt 0
	andl	632(%rsp), %ebx
.Ltmp11260:
	.loc	52 161 44 is_stmt 1
	andl	36(%rsp), %esi
	.loc	52 161 24 is_stmt 0
	orl	%ebx, %esi
.Ltmp11261:
	.loc	41 1291 18 is_stmt 1
	vmovd	%esi, %xmm1
.Ltmp11262:
	.loc	52 124 14
	vucomiss	704(%rsp), %xmm1
	vmovss	.LCPI36_2(%rip), %xmm0
.Ltmp11263:
	.loc	52 161 24
	jbe	.LBB36_390
.Ltmp11264:
	.loc	52 0 24 is_stmt 0
	vmovss	704(%rsp), %xmm0
.Ltmp11265:
	.loc	52 76 9 is_stmt 1
	vdivss	%xmm1, %xmm0, %xmm0
.Ltmp11266:
.LBB36_390:
	.loc	52 0 9 is_stmt 0
	movq	368(%rsp), %rsi
	vmovss	(%rsi,%r9,4), %xmm1
	vmovss	(%rdi,%r9,4), %xmm13
	movq	624(%rsp), %rsi
	leaq	(%r9,%rsi), %rdi
.Ltmp11267:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm0, (%rcx,%r10,4)
.Ltmp11268:
	.loc	4 1050 16
	cmpq	%rdx, %rdi
	movq	256(%rsp), %r11
	jae	.LBB36_462
.Ltmp11269:
	.loc	52 51 9
	vmovss	(%rcx,%rdi,4), %xmm0
	vmovaps	%xmm0, %xmm5
.Ltmp11270:
	.loc	1 1206 22
	testl	%r11d, %r11d
	je	.LBB36_393
.Ltmp11271:
	.loc	1 0 22 is_stmt 0
	vminss	%xmm0, %xmm12, %xmm5
.LBB36_393:
	movq	248(%rsp), %rsi
	addq	%r9, %rsi
	movq	%rsi, 176(%rsp)
.Ltmp11272:
	movl	%r11d, %r11d
.Ltmp11273:
	.loc	1 1212 20 is_stmt 1
	incq	%r11
	movq	224(%rsp), %rsi
	cmpq	168(%rsp), %r11
	vmovss	.LCPI36_2(%rip), %xmm12
.Ltmp11274:
	.loc	1 1213 22
	jne	.LBB36_397
	.loc	1 0 22 is_stmt 0
.Ltmp11275:
	.p2align	4
.LBB36_394:
.Ltmp11276:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB36_462
.Ltmp11277:
	.loc	52 161 24
	vminss	(%rcx,%rdi,4), %xmm0, %xmm0
.Ltmp11278:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%rdi,4)
.Ltmp11279:
	.loc	1 1224 16
	testq	%rdi, %rdi
	cmoveq	%r12, %rdi
	.loc	1 1227 13
	decq	%rdi
.Ltmp11280:
	.loc	10 1916 50
	decq	%rsi
.Ltmp11281:
	.loc	3 900 12
	jne	.LBB36_394
.Ltmp11282:
	.loc	3 0 12 is_stmt 0
	xorl	%r11d, %r11d
	vmovaps	%xmm5, %xmm0
	jmp	.LBB36_399
	.p2align	4
.LBB36_397:
	movq	176(%rsp), %rdi
.Ltmp11283:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB36_462
.Ltmp11284:
	.loc	52 51 9
	vmovss	(%rcx,%rdi,4), %xmm0
.Ltmp11285:
	.loc	52 161 24
	vminss	%xmm5, %xmm0, %xmm0
.Ltmp11286:
.LBB36_399:
	.loc	52 0 24 is_stmt 0
	movq	488(%rsp), %rsi
	leaq	(%r9,%rsi), %rdi
.Ltmp11287:
	.loc	52 71 9 is_stmt 1
	vmulss	.LCPI36_3(%rip), %xmm0, %xmm0
.Ltmp11288:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp11289:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm0
.Ltmp11290:
	.loc	4 1050 16
	cmpq	%r8, %rdi
	jae	.LBB36_566
.Ltmp11291:
	.loc	1 0 0 is_stmt 0
	leaq	(%r9,%r14), %rsi
	incq	%rsi
	cmpq	%r8, %rsi
.Ltmp11292:
	.loc	4 1050 16
	ja	.LBB36_567
.Ltmp11293:
	.loc	4 0 16
	movq	%r11, 256(%rsp)
	leaq	(%r9,%r15), %r11
.Ltmp11294:
	vaddss	%xmm7, %xmm0, %xmm2
	movq	320(%rsp), %r8
	vsubss	(%r8,%rdi,4), %xmm2, %xmm7
.Ltmp11295:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm0, (%r8,%r10,4)
.Ltmp11296:
	.loc	52 76 9
	vdivss	60(%rsp), %xmm7, %xmm0
.Ltmp11297:
	.loc	52 66 9
	vsubss	%xmm0, %xmm12, %xmm2
.Ltmp11298:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm10, %xmm2, %xmm0
.Ltmp11299:
	.loc	52 92 9 is_stmt 1
	vmulss	208(%rsp), %xmm0, %xmm0
	vaddss	%xmm0, %xmm10, %xmm3
.Ltmp11300:
	.loc	52 103 24
	vbroadcastss	.LCPI36_0(%rip), %xmm0
.Ltmp11301:
	.loc	52 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp11302:
	.loc	52 103 24
	vandps	%xmm0, %xmm2, %xmm3
.Ltmp11303:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm3, %xmm3
.Ltmp11304:
	.loc	4 1050 16
	cmpq	216(%rsp), %r11
	jae	.LBB36_588
.Ltmp11305:
	.loc	4 0 16 is_stmt 0
	movq	%r14, %r8
	movq	%r15, %r14
	vandps	%xmm2, %xmm3, %xmm10
.Ltmp11306:
	.loc	52 66 9 is_stmt 1
	vsubss	%xmm10, %xmm12, %xmm2
	movq	312(%rsp), %rdi
.Ltmp11307:
	.loc	52 51 9
	vmovss	(%rdi,%r11,4), %xmm3
.Ltmp11308:
	.loc	52 56 9
	vmovss	%xmm1, (%rdi,%r11,4)
.Ltmp11309:
	.loc	52 71 9
	vmulss	%xmm3, %xmm2, %xmm1
.Ltmp11310:
	.loc	41 1244 18
	vmovd	%xmm3, %edi
.Ltmp11311:
	.loc	52 161 24
	andl	592(%rsp), %edi
.Ltmp11312:
	.loc	41 1244 18
	vmovd	%xmm1, %r15d
.Ltmp11313:
	.loc	52 161 44
	andl	64(%rsp), %r15d
	.loc	52 161 24 is_stmt 0
	orl	%edi, %r15d
	movq	368(%rsp), %rdi
.Ltmp11314:
	.loc	52 56 9 is_stmt 1
	movl	%r15d, (%rdi,%r9,4)
	cmpq	%rax, %rsi
.Ltmp11315:
	.loc	4 1050 16
	ja	.LBB36_590
.Ltmp11316:
	.loc	1 0 0 is_stmt 0
	andl	36(%rsp), %ebp
	orl	%ebp, %ebx
	vmovd	%ebx, %xmm2
.Ltmp11317:
	.loc	52 124 14 is_stmt 1
	vucomiss	160(%rsp), %xmm2
	vmovaps	%xmm12, %xmm1
.Ltmp11318:
	.loc	52 161 24
	jbe	.LBB36_405
.Ltmp11319:
	.loc	52 0 24 is_stmt 0
	vmovss	160(%rsp), %xmm1
.Ltmp11320:
	.loc	52 76 9 is_stmt 1
	vdivss	%xmm2, %xmm1, %xmm1
.Ltmp11321:
.LBB36_405:
	.loc	52 0 9 is_stmt 0
	movq	112(%rsp), %rdi
	addq	%r9, %rdi
.Ltmp11322:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm1, (%r13,%r10,4)
.Ltmp11323:
	.loc	4 1050 16
	cmpq	%rax, %rdi
	movq	840(%rsp), %rbp
	jae	.LBB36_463
.Ltmp11324:
	.loc	4 0 16 is_stmt 0
	movq	%r14, %r15
.Ltmp11325:
	.loc	52 51 9 is_stmt 1
	vmovss	(%r13,%rdi,4), %xmm1
	vmovaps	%xmm1, %xmm3
	movq	432(%rsp), %r14
.Ltmp11326:
	.loc	1 1206 22
	testl	%r14d, %r14d
	je	.LBB36_408
.Ltmp11327:
	.loc	1 0 22 is_stmt 0
	vminss	%xmm1, %xmm11, %xmm3
.LBB36_408:
	movl	%r14d, %r14d
.Ltmp11328:
	.loc	1 1212 20 is_stmt 1
	incq	%r14
	movq	480(%rsp), %rbx
	movq	%r14, 432(%rsp)
	cmpq	304(%rsp), %r14
	movq	%r8, %r14
.Ltmp11329:
	.loc	1 1213 22
	jne	.LBB36_412
	.loc	1 0 22 is_stmt 0
.Ltmp11330:
	.p2align	4
.LBB36_409:
.Ltmp11331:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rax, %rdi
	jae	.LBB36_463
.Ltmp11332:
	.loc	52 161 24
	vminss	(%r13,%rdi,4), %xmm1, %xmm1
.Ltmp11333:
	.loc	52 56 9
	vmovss	%xmm1, (%r13,%rdi,4)
.Ltmp11334:
	.loc	1 1224 16
	testq	%rdi, %rdi
	cmoveq	%r12, %rdi
	.loc	1 1227 13
	decq	%rdi
.Ltmp11335:
	.loc	10 1916 50
	decq	%rbx
.Ltmp11336:
	.loc	3 900 12
	jne	.LBB36_409
.Ltmp11337:
	.loc	3 0 12 is_stmt 0
	movq	$0, 432(%rsp)
	vmovaps	%xmm3, %xmm1
	jmp	.LBB36_414
	.p2align	4
.LBB36_412:
	movq	176(%rsp), %rdi
.Ltmp11338:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rax, %rdi
	jae	.LBB36_463
.Ltmp11339:
	.loc	52 51 9
	vmovss	(%r13,%rdi,4), %xmm1
.Ltmp11340:
	.loc	52 161 24
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp11341:
.LBB36_414:
	.loc	52 0 24 is_stmt 0
	movq	152(%rsp), %rdi
	addq	%r9, %rdi
.Ltmp11342:
	.loc	52 71 9 is_stmt 1
	vmulss	.LCPI36_3(%rip), %xmm1, %xmm1
.Ltmp11343:
	.loc	41 1783 9
	vroundss	$9, %xmm1, %xmm1, %xmm1
.Ltmp11344:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm1, %xmm1
	movq	472(%rsp), %r8
.Ltmp11345:
	.loc	4 1050 16
	cmpq	%r8, %rdi
	jae	.LBB36_566
.Ltmp11346:
	.loc	4 0 16 is_stmt 0
	cmpq	%r8, %rsi
.Ltmp11347:
	.loc	4 1050 16
	ja	.LBB36_567
.Ltmp11348:
	.loc	1 0 0
	leaq	1(%r11), %rsi
.Ltmp11349:
	vaddss	%xmm4, %xmm1, %xmm2
	movq	416(%rsp), %r8
	vsubss	(%r8,%rdi,4), %xmm2, %xmm4
.Ltmp11350:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm1, (%r8,%r10,4)
	cmpq	424(%rsp), %rsi
.Ltmp11351:
	.loc	4 1050 16
	ja	.LBB36_596
.Ltmp11352:
	.loc	52 76 9
	vdivss	464(%rsp), %xmm4, %xmm1
.Ltmp11353:
	.loc	52 66 9
	vsubss	%xmm1, %xmm12, %xmm1
.Ltmp11354:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm6, %xmm1, %xmm2
.Ltmp11355:
	.loc	52 92 9 is_stmt 1
	vmulss	76(%rsp), %xmm2, %xmm2
	vaddss	%xmm2, %xmm6, %xmm2
.Ltmp11356:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11357:
	.loc	52 103 24
	vandps	%xmm0, %xmm1, %xmm0
.Ltmp11358:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm0, %xmm0
	vandps	%xmm1, %xmm0, %xmm6
.Ltmp11359:
	.loc	52 66 9
	vsubss	%xmm6, %xmm12, %xmm0
	movq	456(%rsp), %rsi
.Ltmp11360:
	.loc	52 51 9
	vmovss	(%rsi,%r11,4), %xmm1
.Ltmp11361:
	.loc	52 56 9
	vmovss	%xmm13, (%rsi,%r11,4)
.Ltmp11362:
	.loc	52 71 9
	vmulss	%xmm1, %xmm0, %xmm0
.Ltmp11363:
	.loc	41 1244 18
	vmovd	%xmm1, %esi
.Ltmp11364:
	.loc	52 161 24
	andl	592(%rsp), %esi
.Ltmp11365:
	.loc	41 1244 18
	vmovd	%xmm0, %edi
.Ltmp11366:
	.loc	52 161 44
	andl	64(%rsp), %edi
	.loc	52 161 24 is_stmt 0
	orl	%esi, %edi
	movq	336(%rsp), %rsi
.Ltmp11367:
	.loc	52 56 9 is_stmt 1
	movl	%edi, (%rsi,%r9,4)
	movq	%rsi, %rdi
.Ltmp11368:
	.loc	1 0 0 is_stmt 0
	incq	%r9
	vmovaps	%xmm3, %xmm11
	vmovaps	%xmm5, %xmm12
.Ltmp11369:
	.loc	11 304 12 is_stmt 1
	cmpq	328(%rsp), %r9
	movq	240(%rsp), %r8
	movq	232(%rsp), %r11
	jne	.LBB36_387
	jmp	.LBB36_419
.Ltmp11370:
.LBB36_418:
	.loc	11 0 12 is_stmt 0
	vmovaps	%xmm9, %xmm3
	vmovaps	%xmm8, %xmm5
	vmovss	96(%rsp), %xmm6
	vmovss	104(%rsp), %xmm4
	vmovaps	%xmm15, %xmm10
	vmovaps	%xmm0, %xmm7
	movq	840(%rsp), %rbp
	movq	232(%rsp), %r11
.LBB36_419:
	movq	296(%rsp), %rdi
	movq	408(%rsp), %r9
	addq	%r9, %rdi
.Ltmp11371:
	.loc	1 2056 39 is_stmt 1
	addq	%r9, %r14
.Ltmp11372:
	.loc	1 1148 8
	cmpq	%r12, %r14
	movq	%r12, %rsi
	movq	%r14, %rbx
	movl	$0, %r10d
	cmovbq	%r10, %rsi
	subq	%rsi, %rbx
.Ltmp11373:
	.loc	1 2057 39
	addq	%r9, %r15
	movq	1600(%rsp), %rsi
.Ltmp11374:
	.loc	1 1148 8
	cmpq	%rsi, %r15
	cmovbq	%r10, %rsi
	subq	%rsi, %r15
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm5, %xmm8
	vmovss	%xmm6, 96(%rsp)
	vmovss	%xmm4, 104(%rsp)
	vmovaps	%xmm10, %xmm15
	vmovaps	%xmm7, %xmm0
	movq	1544(%rsp), %rsi
.Ltmp11375:
	.loc	1 1984 19
	cmpq	%rsi, %rdi
	movq	280(%rsp), %r14
	jb	.LBB36_382
.Ltmp11376:
	.loc	1 0 19 is_stmt 0
	vmovss	%xmm3, 832(%rsp)
	vmovss	%xmm5, 836(%rsp)
	movq	%r15, 728(%rsp)
	vmovss	%xmm7, 1068(%rsp)
	vmovss	%xmm10, 1064(%rsp)
	vmovss	%xmm4, 1160(%rsp)
	vmovss	%xmm6, 1156(%rsp)
	movq	432(%rsp), %rax
	movl	%eax, %edi
	movq	256(%rsp), %rax
	movl	%eax, %r9d
	vmovaps	%xmm10, %xmm0
	vmovss	%xmm7, 88(%rsp)
	movq	968(%rsp), %r15
	movq	120(%rsp), %rcx
	movq	976(%rsp), %r13
	movq	392(%rsp), %r11
	vmovss	40(%rsp), %xmm13
	movl	$32, %edx
	movq	1512(%rsp), %rsi
	movq	616(%rsp), %rax
	vmovss	48(%rsp), %xmm12
	vmovss	128(%rsp), %xmm9
	vmovss	960(%rsp), %xmm15
	vmovss	144(%rsp), %xmm1
	vmovss	1168(%rsp), %xmm2
	vmovss	1180(%rsp), %xmm3
	vmovss	8(%rsp), %xmm5
	vmovss	364(%rsp), %xmm7
	vmovss	16(%rsp), %xmm10
	jmp	.LBB36_380
.LBB36_421:
	leaq	740(%rsp), %rdi
	leaq	136(%r12), %rsi
.Ltmp11377:
	.loc	1 1943 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	496(%rsp), %rdi
	leaq	336(%r12), %rsi
.Ltmp11378:
	.loc	1 1944 25
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp11379:
	.loc	1 1949 19
	movzbl	776(%r12), %eax
	movl	%eax, 704(%rsp)
.Ltmp11380:
	.loc	1 1950 21
	movzbl	777(%r12), %eax
	movl	%eax, 36(%rsp)
	movq	%r12, %r14
.Ltmp11381:
	.loc	1 1951 16
	movq	544(%r12), %r12
.Ltmp11382:
	.loc	1 1952 16
	movq	552(%r14), %rbx
.Ltmp11383:
	.loc	1 1953 27
	movl	560(%r14), %eax
	movq	%rax, 632(%rsp)
.Ltmp11384:
	.loc	1 1954 27
	movl	564(%r14), %eax
	movq	%rax, 448(%rsp)
	leaq	2648(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	1616(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	1424(%rsp), %rdi
	movq	952(%rsp), %rsi
	movq	%r12, 8(%rsp)
.Ltmp11385:
	.loc	1 1961 32
	movq	%r12, %rdx
	movq	%rbx, 104(%rsp)
	movq	%rbx, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
.Ltmp11386:
	.loc	1 1962 33
	movq	544(%r14), %rdx
	movq	552(%r14), %rcx
	leaq	848(%rsp), %rdi
	movq	944(%rsp), %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
	movq	120(%rsp), %rcx
.Ltmp11387:
	.loc	8 446 20
	testq	%rcx, %rcx
	je	.LBB36_554
.Ltmp11388:
	.loc	8 0 20 is_stmt 0
	movl	704(%rsp), %edx
	movl	%edx, %eax
	negl	%eax
	movl	%eax, 728(%rsp)
	movl	36(%rsp), %r10d
.Ltmp11389:
	movl	%r10d, %eax
	negl	%eax
	movl	%eax, 624(%rsp)
.Ltmp11390:
	.loc	4 3756 21 is_stmt 1
	movq	%rcx, %r8
	shrq	$5, %r8
.Ltmp11391:
	.loc	4 3757 21
	movl	%ecx, %eax
	andl	$31, %eax
.Ltmp11392:
	.loc	4 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %r8
	decl	%edx
	movl	%edx, 704(%rsp)
	decl	%r10d
	movl	$32, %r9d
	movq	%rbp, %r11
	movq	%r13, %rbx
	movq	%rcx, %r14
	xorl	%edi, %edi
	movl	%r10d, 36(%rsp)
.Ltmp11393:
	.loc	8 446 20
	jmp	.LBB36_492
.Ltmp11394:
.LBB36_431:
	.loc	1 1411 5
	vmovss	%xmm8, 824(%rsp)
.Ltmp11395:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp11396:
	.loc	1 0 0
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp11397:
.LBB36_432:
	.loc	5 438 16 is_stmt 1
	cmpq	%rsi, 120(%rsp)
	ja	.LBB36_629
.Ltmp11398:
	.loc	5 0 16 is_stmt 0
	movq	%rbp, %rax
	movq	120(%rsp), %rdi
	.p2align	4
.LBB36_434:
.Ltmp11399:
	.loc	17 131 18 is_stmt 1
	movq	%rdi, %rcx
.Ltmp11400:
	.loc	15 1504 12
	testq	%rdi, %rdi
	je	.LBB36_438
.Ltmp11401:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp11402:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%esi, %esi
	xorl	%r8d, %r8d
.Ltmp11403:
	.loc	16 0 18 is_stmt 0
.Ltmp11404:
	.p2align	4
.LBB36_436:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r8), %esi
.Ltmp11405:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp11406:
	.loc	6 180 28
	jne	.LBB36_436
.Ltmp11407:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp11408:
	.loc	19 2054 74
	movq	%rcx, %rdi
	subq	%rdx, %rdi
.Ltmp11409:
	.loc	17 136 12
	testl	%esi, %esi
	je	.LBB36_434
.Ltmp11410:
.LBB36_438:
	.loc	15 1504 12
	testq	%rcx, %rcx
	sete	%al
	jmp	.LBB36_350
.Ltmp11411:
.LBB36_439:
	.loc	15 0 12 is_stmt 0
	movq	200(%rsp), %rbx
	movq	696(%rsp), %r12
.Ltmp11412:
	.loc	52 56 9 is_stmt 1
	cmpq	$0, 224(%r12)
	je	.LBB36_581
.Ltmp11413:
.LBB36_440:
	.loc	1 0 0 is_stmt 0
	vmovss	568(%rsp), %xmm1
	movl	572(%rsp), %ecx
	vmovss	920(%rsp), %xmm0
	movl	924(%rsp), %eax
.Ltmp11414:
	.loc	1 2072 0 is_stmt 1
	movq	216(%r12), %rdx
.Ltmp11415:
	.loc	52 56 9
	vmovss	%xmm1, (%rdx)
.Ltmp11416:
	.loc	1 2073 5
	movq	256(%r12), %rdx
.Ltmp11417:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp11418:
	.loc	6 180 28
	je	.LBB36_443
.Ltmp11419:
	.loc	6 0 28 is_stmt 0
	movq	248(%r12), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB36_442:
.Ltmp11420:
	.loc	20 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp11421:
	.loc	7 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp11422:
	.loc	6 180 28
	jne	.LBB36_442
.Ltmp11423:
.LBB36_443:
	.loc	52 56 9
	cmpq	$0, 424(%r12)
	je	.LBB36_581
.Ltmp11424:
	.loc	1 2074 0
	movq	416(%r12), %rcx
.Ltmp11425:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx)
.Ltmp11426:
	.loc	1 2075 5
	movq	456(%r12), %rcx
.Ltmp11427:
	.loc	7 1714 9
	testq	%rcx, %rcx
.Ltmp11428:
	.loc	6 180 28
	je	.LBB36_447
.Ltmp11429:
	.loc	6 0 28 is_stmt 0
	movq	448(%r12), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB36_446:
.Ltmp11430:
	.loc	20 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp11431:
	.loc	7 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp11432:
	.loc	6 180 28
	jne	.LBB36_446
.Ltmp11433:
.LBB36_447:
	.loc	6 0 28 is_stmt 0
	leaq	984(%rsp), %rdi
	movq	952(%rsp), %rsi
	.loc	1 2077 14 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	leaq	1076(%rsp), %rdi
	movq	944(%rsp), %rsi
	.loc	1 2078 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	728(%rsp), %rax
	.loc	1 2079 5
	movl	%eax, 560(%r12)
	.loc	1 2080 5
	movl	%ebx, 564(%r12)
.Ltmp11434:
	.loc	1 2255 35
	cmpb	$0, 1532(%rsp)
	je	.LBB36_349
	jmp	.LBB36_340
.LBB36_448:
.Ltmp11435:
	.loc	1 1411 5
	vmovss	%xmm8, 824(%rsp)
.Ltmp11436:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp11437:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_422064f3ca430d31d9007f55b436c6ca(%rip), %rcx
	movq	432(%rsp), %rdi
.Ltmp11438:
	.loc	1 0 0 is_stmt 0
	movq	%r15, %rsi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_449:
.Ltmp11439:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11440:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11441:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_422064f3ca430d31d9007f55b436c6ca(%rip), %rcx
	movq	%rdx, %rdi
.Ltmp11442:
	.loc	1 0 0 is_stmt 0
	movq	%r15, %rsi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_450:
.Ltmp11443:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11444:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11445:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
.Ltmp11446:
	.loc	1 0 0 is_stmt 0
	movq	%r11, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_451:
.Ltmp11447:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11448:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11449:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp11450:
.LBB36_452:
	.loc	1 1411 5
	vmovss	%xmm8, 824(%rsp)
.Ltmp11451:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp11452:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
.Ltmp11453:
	.loc	1 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_453:
.Ltmp11454:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp11455:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp11456:
	.loc	1 0 0
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_454:
.Ltmp11457:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11458:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11459:
	.loc	1 0 0
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_455:
.Ltmp11460:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp11461:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp11462:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_54f0ebc64763f3f022885a8e201aab88(%rip), %rcx
	movq	432(%rsp), %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp11463:
.LBB36_456:
	.loc	1 1417 5
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11464:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11465:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_54f0ebc64763f3f022885a8e201aab88(%rip), %rcx
	movq	%r10, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp11466:
.LBB36_457:
	.loc	1 1417 5
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11467:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11468:
	.loc	1 0 0
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_458:
.Ltmp11469:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11470:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11471:
	.loc	1 0 0
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_459:
.Ltmp11472:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp11473:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp11474:
	.loc	1 0 0
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_460:
.Ltmp11475:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11476:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11477:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
.Ltmp11478:
	.loc	1 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_461:
.Ltmp11479:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp11480:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp11481:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
.Ltmp11482:
	.loc	1 0 0 is_stmt 0
	movq	%r11, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_462:
	vmovss	8(%rsp), %xmm0
.Ltmp11483:
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp11484:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 924(%rsp)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp11485:
	vmovss	%xmm8, 568(%rsp)
.Ltmp11486:
	vmovss	%xmm9, 920(%rsp)
.Ltmp11487:
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_463:
	vmovss	8(%rsp), %xmm0
.Ltmp11488:
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp11489:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %ecx
	movl	%ecx, 572(%rsp)
	movl	4(%rsp), %ecx
	movl	%ecx, 924(%rsp)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp11490:
	vmovss	%xmm8, 568(%rsp)
.Ltmp11491:
	vmovss	%xmm9, 920(%rsp)
.Ltmp11492:
	.loc	1 1131 25 is_stmt 1
	leaq	1(%rdi), %rsi
.Ltmp11493:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp11494:
.LBB36_464:
	.loc	1 1417 5
	vmovss	%xmm7, 1320(%rsp)
.Ltmp11495:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp11496:
	.loc	1 1291 45 is_stmt 1
	leaq	.Lalloc_91a4c6b9b17ebf4d863f9a70b6dc929a(%rip), %rdx
	movq	%r15, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp11497:
.LBB36_465:
	.loc	1 1411 5
	vmovss	%xmm8, 824(%rsp)
.Ltmp11498:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp11499:
	.loc	1 1291 45 is_stmt 1
	leaq	.Lalloc_91a4c6b9b17ebf4d863f9a70b6dc929a(%rip), %rdx
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp11500:
.LBB36_466:
	.loc	1 0 45 is_stmt 0
	movb	$1, %dl
	.loc	1 2210 12 is_stmt 1
	cmpb	$0, 780(%r12)
	je	.LBB36_30
.Ltmp11501:
	.loc	1 663 31
	movq	256(%r12), %rsi
	.loc	1 663 57 is_stmt 0
	movq	320(%r12), %rax
.Ltmp11502:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rsi, %rax
	cmovbq	%rax, %rsi
.Ltmp11503:
	.loc	11 304 12
	testq	%rsi, %rsi
	movq	120(%rsp), %rcx
	je	.LBB36_476
.Ltmp11504:
	.loc	11 0 12 is_stmt 0
	movq	248(%r12), %rdi
	movq	312(%r12), %r8
	xorl	%r9d, %r9d
	movq	%rcx, %r10
	shrq	$32, %r10
	jmp	.LBB36_471
.LBB36_474:
	movq	120(%rsp), %rax
.Ltmp11505:
	.loc	1 665 42 is_stmt 1
	xorl	%edx, %edx
	divl	%r11d
	.loc	1 665 23 is_stmt 0
	addq	%rdx, %rcx
	.loc	1 665 22
	movq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB36_475
.LBB36_469:
	movq	%rcx, %rax
	xorl	%edx, %edx
	divq	%r11
.LBB36_470:
	.loc	1 665 13
	movl	%edx, (%rdi,%r9,4)
.Ltmp11506:
	.loc	1 0 0
	incq	%r9
.Ltmp11507:
	.loc	11 304 12 is_stmt 1
	addq	$12, %r8
	cmpq	%r9, %rsi
	movq	120(%rsp), %rcx
	je	.LBB36_476
.Ltmp11508:
.LBB36_471:
	.loc	1 664 26
	movl	(%r8), %r11d
	testq	%r11, %r11
.Ltmp11509:
	.loc	1 665 42
	je	.LBB36_647
	.loc	1 665 24 is_stmt 0
	movl	(%rdi,%r9,4), %ecx
	.loc	1 665 42
	testq	%r10, %r10
	je	.LBB36_474
	.loc	1 0 42
	movq	120(%rsp), %rax
	.loc	1 665 42
	xorl	%edx, %edx
	divq	%r11
	.loc	1 665 23
	addq	%rdx, %rcx
	.loc	1 665 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB36_469
.LBB36_475:
	movl	%ecx, %eax
	xorl	%edx, %edx
	divl	%r11d
	jmp	.LBB36_470
.Ltmp11510:
.LBB36_476:
	.loc	1 663 31 is_stmt 1
	movq	456(%r12), %rsi
	.loc	1 663 57 is_stmt 0
	movq	520(%r12), %rax
.Ltmp11511:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rsi, %rax
	cmovbq	%rax, %rsi
.Ltmp11512:
	.loc	11 304 12
	testq	%rsi, %rsi
	je	.LBB36_485
.Ltmp11513:
	.loc	11 0 12 is_stmt 0
	movq	448(%r12), %rdi
	movq	512(%r12), %r8
	xorl	%r9d, %r9d
	movq	%rcx, %r10
	shrq	$32, %r10
	jmp	.LBB36_480
.LBB36_483:
	movq	120(%rsp), %rax
.Ltmp11514:
	.loc	1 665 42 is_stmt 1
	xorl	%edx, %edx
	divl	%r11d
	.loc	1 665 23 is_stmt 0
	addq	%rdx, %rcx
	.loc	1 665 22
	movq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB36_484
.LBB36_478:
	movq	%rcx, %rax
	xorl	%edx, %edx
	divq	%r11
.LBB36_479:
	.loc	1 665 13
	movl	%edx, (%rdi,%r9,4)
.Ltmp11515:
	.loc	1 0 0
	incq	%r9
.Ltmp11516:
	.loc	11 304 12 is_stmt 1
	addq	$12, %r8
	cmpq	%r9, %rsi
	je	.LBB36_485
.Ltmp11517:
.LBB36_480:
	.loc	1 664 26
	movl	(%r8), %r11d
	testq	%r11, %r11
.Ltmp11518:
	.loc	1 665 42
	je	.LBB36_647
	.loc	1 665 24 is_stmt 0
	movl	(%rdi,%r9,4), %ecx
	.loc	1 665 42
	testq	%r10, %r10
	je	.LBB36_483
	.loc	1 0 42
	movq	120(%rsp), %rax
	.loc	1 665 42
	xorl	%edx, %edx
	divq	%r11
	.loc	1 665 23
	addq	%rdx, %rcx
	.loc	1 665 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB36_478
.LBB36_484:
	movl	%ecx, %eax
	xorl	%edx, %edx
	divl	%r11d
	jmp	.LBB36_479
.Ltmp11519:
.LBB36_485:
	.loc	1 2227 26 is_stmt 1
	movq	552(%r12), %rsi
.Ltmp11520:
	.loc	1 455 44
	testq	%rsi, %rsi
	je	.LBB36_648
.Ltmp11521:
	.loc	1 2227 26
	movq	544(%r12), %rcx
.Ltmp11522:
	.loc	1 455 23
	movl	560(%r12), %edi
	movq	120(%rsp), %rax
	.loc	1 455 44 is_stmt 0
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB36_568
	xorl	%edx, %edx
	divq	%rsi
	movq	%rdx, %rax
	jmp	.LBB36_569
.Ltmp11523:
.LBB36_488:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_5cbcacccdbb7b907c55dbc42154fcf08(%rip), %rcx
	movq	224(%rsp), %rdi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp11524:
.LBB36_489:
	.loc	5 443 13
	leaq	.Lalloc_5cbcacccdbb7b907c55dbc42154fcf08(%rip), %rcx
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp11525:
.LBB36_490:
	.loc	5 0 13 is_stmt 0
	movq	%rbx, 448(%rsp)
	movq	%r11, 632(%rsp)
	vmovss	%xmm12, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
.Ltmp11526:
	vmovss	%xmm10, 1496(%rsp)
.Ltmp11527:
	vmovss	%xmm9, 920(%rsp)
	movq	48(%rsp), %rax
	movl	%eax, 1500(%rsp)
	movq	280(%rsp), %rax
	movl	%eax, 924(%rsp)
	movq	968(%rsp), %r15
	movq	120(%rsp), %rcx
	movq	976(%rsp), %r13
	movq	664(%rsp), %r8
	movl	$32, %r9d
	movq	80(%rsp), %rdi
	movq	656(%rsp), %r11
	movq	648(%rsp), %rbx
	movq	640(%rsp), %r14
.Ltmp11528:
.LBB36_491:
	addq	$32, %rdi
	decq	%r8
.Ltmp11529:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %r14
	subq	$-128, %rbx
	subq	$-128, %r11
	testq	%r8, %r8
	je	.LBB36_554
.LBB36_492:
.Ltmp11530:
	.loc	4 2584 13
	cmpq	$32, %r14
	movl	$32, %eax
	cmovbq	%r14, %rax
	cmpq	$1, %rax
	movq	%rax, 96(%rsp)
	adcq	$0, %rax
.Ltmp11531:
	.loc	1 1967 55
	movq	%rcx, %rsi
	subq	%rdi, %rsi
.Ltmp11532:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%r9, %rsi
	movq	%rsi, 616(%rsp)
.Ltmp11533:
	.loc	1 1972 39
	addq	%rdi, %rsi
.Ltmp11534:
	.loc	4 1050 16
	jb	.LBB36_619
	cmpq	%r15, %rsi
	ja	.LBB36_619
.Ltmp11535:
	.loc	1 1759 23
	vmovss	740(%rsp), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	744(%rsp), %xmm13
	vmovss	748(%rsp), %xmm10
	vmovss	752(%rsp), %xmm11
	vmovss	756(%rsp), %xmm8
	vmovss	760(%rsp), %xmm4
	vmovss	764(%rsp), %xmm9
	vmovss	768(%rsp), %xmm2
	vmovss	772(%rsp), %xmm6
	vmovss	776(%rsp), %xmm7
	vmovss	780(%rsp), %xmm14
.Ltmp11536:
	.loc	11 304 12
	cmpq	%rdi, %rcx
	jne	.LBB36_496
.Ltmp11537:
	.loc	1 0 0 is_stmt 0
	vmovss	784(%rsp), %xmm3
.Ltmp11538:
	.loc	11 304 12
	jmp	.LBB36_498
.Ltmp11539:
.LBB36_496:
	.loc	11 0 12
	movq	696(%rsp), %rdx
	vmovss	584(%rdx), %xmm0
	vmovss	%xmm0, 64(%rsp)
	vmovss	588(%rdx), %xmm0
	vmovss	%xmm0, 592(%rsp)
	vmovss	592(%rdx), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	596(%rdx), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	600(%rdx), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	604(%rdx), %xmm0
	vmovss	%xmm0, 488(%rsp)
	xorl	%ecx, %ecx
	vmovss	%xmm14, 16(%rsp)
	vmovss	%xmm7, 40(%rsp)
	vmovss	%xmm6, 432(%rsp)
	vmovss	%xmm2, 368(%rsp)
	vmovaps	%xmm4, 256(%rsp)
	vmovaps	%xmm8, %xmm1
	vmovaps	%xmm11, %xmm15
	vmovaps	%xmm10, %xmm12
	vmovaps	%xmm13, %xmm5
	vmovss	608(%rdx), %xmm0
	vmovss	%xmm0, 152(%rsp)
	vmovss	612(%rdx), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	616(%rdx), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	620(%rdx), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	624(%rdx), %xmm0
	vmovss	%xmm0, 232(%rsp)
	vmovss	628(%rdx), %xmm0
	vmovss	%xmm0, 168(%rsp)
	vmovss	632(%rdx), %xmm0
	vmovss	%xmm0, 224(%rsp)
	vmovss	636(%rdx), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	640(%rdx), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	644(%rdx), %xmm0
	vmovss	%xmm0, 216(%rsp)
	vmovss	648(%rdx), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	652(%rdx), %xmm0
	vmovss	%xmm0, 208(%rsp)
	vmovss	656(%rdx), %xmm0
	vmovss	%xmm0, 60(%rsp)
	vmovss	660(%rdx), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	664(%rdx), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	668(%rdx), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	672(%rdx), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	676(%rdx), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	680(%rdx), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	684(%rdx), %xmm0
	vmovss	%xmm0, 76(%rsp)
	vmovss	688(%rdx), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	692(%rdx), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	696(%rdx), %xmm0
	vmovss	%xmm0, 296(%rsp)
	vmovss	700(%rdx), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	704(%rdx), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	708(%rdx), %xmm0
	vmovss	%xmm0, 288(%rsp)
	vmovss	712(%rdx), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	716(%rdx), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	720(%rdx), %xmm0
	vmovss	%xmm0, 4(%rsp)
	vmovss	724(%rdx), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	728(%rdx), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	732(%rdx), %xmm0
	vmovss	%xmm0, 688(%rsp)
	vmovss	736(%rdx), %xmm0
	vmovss	%xmm0, 680(%rsp)
	vmovss	740(%rdx), %xmm0
	vmovss	%xmm0, 672(%rsp)
	vmovss	744(%rdx), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	748(%rdx), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	752(%rdx), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	756(%rdx), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	760(%rdx), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	764(%rdx), %xmm0
	vmovss	%xmm0, 648(%rsp)
	vmovss	768(%rdx), %xmm0
	vmovss	%xmm0, 640(%rsp)
	vmovss	772(%rdx), %xmm0
	vmovss	%xmm0, 92(%rsp)
	vmovaps	%xmm9, %xmm0
	.p2align	4
.LBB36_497:
	vmovss	48(%rsp), %xmm8
.Ltmp11540:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rbx,%rcx,4), %xmm9
	vmovss	%xmm9, 48(%rsp)
.Ltmp11541:
	.loc	52 71 9
	vmulss	64(%rsp), %xmm9, %xmm2
	vxorps	%xmm7, %xmm7, %xmm7
.Ltmp11542:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp11543:
	.loc	52 71 9
	vmulss	592(%rsp), %xmm9, %xmm3
.Ltmp11544:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp11545:
	.loc	52 71 9
	vmulss	160(%rsp), %xmm9, %xmm4
.Ltmp11546:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
	vmovss	%xmm8, 176(%rsp)
.Ltmp11547:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm8, %xmm6
.Ltmp11548:
	.loc	52 61 9
	vaddss	%xmm6, %xmm2, %xmm2
.Ltmp11549:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm8, %xmm6
.Ltmp11550:
	.loc	52 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp11551:
	.loc	52 71 9
	vmulss	152(%rsp), %xmm8, %xmm6
.Ltmp11552:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
.Ltmp11553:
	.loc	52 71 9
	vmulss	112(%rsp), %xmm9, %xmm6
.Ltmp11554:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm6
.Ltmp11555:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm8, %xmm7
.Ltmp11556:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm6
	vmovaps	%xmm5, %xmm10
.Ltmp11557:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm5, %xmm5
.Ltmp11558:
	.loc	52 61 9
	vaddss	%xmm5, %xmm2, %xmm2
.Ltmp11559:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm10, %xmm5
.Ltmp11560:
	.loc	52 61 9
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp11561:
	.loc	52 71 9
	vmulss	232(%rsp), %xmm10, %xmm5
.Ltmp11562:
	.loc	52 61 9
	vaddss	%xmm5, %xmm4, %xmm4
.Ltmp11563:
	.loc	52 71 9
	vmulss	168(%rsp), %xmm10, %xmm5
.Ltmp11564:
	.loc	52 61 9
	vaddss	%xmm5, %xmm6, %xmm5
	vmovaps	%xmm12, %xmm11
.Ltmp11565:
	.loc	52 71 9
	vmulss	224(%rsp), %xmm12, %xmm6
.Ltmp11566:
	.loc	52 61 9
	vaddss	%xmm6, %xmm2, %xmm2
.Ltmp11567:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm12, %xmm6
.Ltmp11568:
	.loc	52 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp11569:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm12, %xmm6
.Ltmp11570:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
.Ltmp11571:
	.loc	52 71 9
	vmulss	216(%rsp), %xmm12, %xmm6
.Ltmp11572:
	.loc	52 61 9
	vaddss	%xmm6, %xmm5, %xmm5
	vmovaps	%xmm15, %xmm8
.Ltmp11573:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm15, %xmm6
.Ltmp11574:
	.loc	52 61 9
	vaddss	%xmm6, %xmm2, %xmm2
.Ltmp11575:
	.loc	52 71 9
	vmulss	208(%rsp), %xmm15, %xmm6
.Ltmp11576:
	.loc	52 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp11577:
	.loc	52 71 9
	vmulss	60(%rsp), %xmm15, %xmm6
.Ltmp11578:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm6
.Ltmp11579:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm15, %xmm4
.Ltmp11580:
	.loc	52 61 9
	vaddss	%xmm4, %xmm5, %xmm5
	vmovaps	%xmm1, %xmm4
.Ltmp11581:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm1, %xmm1
.Ltmp11582:
	.loc	52 61 9
	vaddss	%xmm1, %xmm2, %xmm1
.Ltmp11583:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm4, %xmm2
.Ltmp11584:
	.loc	52 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp11585:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm4, %xmm3
.Ltmp11586:
	.loc	52 61 9
	vaddss	%xmm3, %xmm6, %xmm3
.Ltmp11587:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm4, %xmm6
.Ltmp11588:
	.loc	52 61 9
	vaddss	%xmm6, %xmm5, %xmm5
	vmovaps	256(%rsp), %xmm9
.Ltmp11589:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm9, %xmm6
.Ltmp11590:
	.loc	52 61 9
	vaddss	%xmm6, %xmm1, %xmm1
.Ltmp11591:
	.loc	52 71 9
	vmulss	76(%rsp), %xmm9, %xmm6
.Ltmp11592:
	.loc	52 61 9
	vaddss	%xmm6, %xmm2, %xmm6
.Ltmp11593:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm9, %xmm2
.Ltmp11594:
	.loc	52 61 9
	vaddss	%xmm2, %xmm3, %xmm3
.Ltmp11595:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm9, %xmm2
.Ltmp11596:
	.loc	52 61 9
	vaddss	%xmm2, %xmm5, %xmm5
.Ltmp11597:
	.loc	52 71 9
	vmulss	296(%rsp), %xmm0, %xmm7
.Ltmp11598:
	.loc	52 61 9
	vaddss	%xmm7, %xmm1, %xmm1
.Ltmp11599:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm0, %xmm7
.Ltmp11600:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm7
.Ltmp11601:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm0, %xmm6
.Ltmp11602:
	.loc	52 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp11603:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm0, %xmm6
.Ltmp11604:
	.loc	52 61 9
	vaddss	%xmm6, %xmm5, %xmm5
	vmovss	368(%rsp), %xmm6
.Ltmp11605:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm6, %xmm12
.Ltmp11606:
	.loc	52 61 9
	vaddss	%xmm1, %xmm12, %xmm1
.Ltmp11607:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm6, %xmm12
.Ltmp11608:
	.loc	52 61 9
	vaddss	%xmm7, %xmm12, %xmm12
.Ltmp11609:
	.loc	52 71 9
	vmulss	4(%rsp), %xmm6, %xmm7
.Ltmp11610:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp11611:
	.loc	52 71 9
	vmulss	80(%rsp), %xmm6, %xmm7
.Ltmp11612:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
	vmovss	432(%rsp), %xmm7
.Ltmp11613:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm7, %xmm14
.Ltmp11614:
	.loc	52 61 9
	vaddss	%xmm1, %xmm14, %xmm1
.Ltmp11615:
	.loc	52 71 9
	vmulss	688(%rsp), %xmm7, %xmm14
.Ltmp11616:
	.loc	52 61 9
	vaddss	%xmm14, %xmm12, %xmm12
.Ltmp11617:
	.loc	52 71 9
	vmulss	680(%rsp), %xmm7, %xmm14
.Ltmp11618:
	.loc	52 61 9
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp11619:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm7, %xmm14
.Ltmp11620:
	.loc	52 61 9
	vaddss	%xmm5, %xmm14, %xmm5
	vmovss	40(%rsp), %xmm14
.Ltmp11621:
	.loc	52 71 9
	vmulss	32(%rsp), %xmm14, %xmm15
.Ltmp11622:
	.loc	52 61 9
	vaddss	%xmm1, %xmm15, %xmm1
.Ltmp11623:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm14, %xmm15
.Ltmp11624:
	.loc	52 61 9
	vaddss	%xmm15, %xmm12, %xmm12
.Ltmp11625:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm14, %xmm15
.Ltmp11626:
	.loc	52 61 9
	vaddss	%xmm3, %xmm15, %xmm15
.Ltmp11627:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm14, %xmm3
.Ltmp11628:
	.loc	52 61 9
	vaddss	%xmm3, %xmm5, %xmm5
	vmovss	16(%rsp), %xmm3
.Ltmp11629:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm3, %xmm13
.Ltmp11630:
	.loc	52 61 9
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp11631:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm3, %xmm13
.Ltmp11632:
	.loc	52 61 9
	vaddss	%xmm13, %xmm12, %xmm12
.Ltmp11633:
	.loc	52 71 9
	vmulss	640(%rsp), %xmm3, %xmm13
.Ltmp11634:
	.loc	52 61 9
	vaddss	%xmm13, %xmm15, %xmm13
.Ltmp11635:
	.loc	52 71 9
	vmulss	92(%rsp), %xmm3, %xmm15
.Ltmp11636:
	.loc	52 61 9
	vaddss	%xmm5, %xmm15, %xmm5
	vbroadcastss	.LCPI36_0(%rip), %xmm2
.Ltmp11637:
	.loc	52 103 24
	vandps	%xmm2, %xmm1, %xmm1
.Ltmp11638:
	.loc	52 103 24 is_stmt 0
	vandps	%xmm2, %xmm9, %xmm15
.Ltmp11639:
	.loc	52 161 24 is_stmt 1
	vmaxss	%xmm1, %xmm15, %xmm1
.Ltmp11640:
	.loc	52 103 24
	vandps	%xmm2, %xmm12, %xmm12
.Ltmp11641:
	.loc	52 161 24
	vmaxss	%xmm12, %xmm1, %xmm1
.Ltmp11642:
	.loc	52 103 24
	vandps	%xmm2, %xmm13, %xmm12
	vmovss	176(%rsp), %xmm13
.Ltmp11643:
	.loc	52 161 24
	vmaxss	%xmm12, %xmm1, %xmm1
.Ltmp11644:
	.loc	52 103 24
	vandps	%xmm2, %xmm5, %xmm5
	vmovaps	%xmm0, %xmm2
.Ltmp11645:
	.loc	52 161 24
	vmaxss	%xmm5, %xmm1, %xmm1
.Ltmp11646:
	.loc	52 56 9
	vmovss	%xmm1, 2648(%rsp,%rcx,4)
.Ltmp11647:
	.loc	11 308 13
	incq	%rcx
	vmovss	%xmm14, 16(%rsp)
	vmovss	%xmm7, 40(%rsp)
	vmovss	%xmm6, 432(%rsp)
	vmovss	%xmm0, 368(%rsp)
	vmovaps	%xmm9, %xmm0
	vmovaps	%xmm4, 256(%rsp)
	vmovaps	%xmm8, %xmm1
	vmovaps	%xmm11, %xmm15
	vmovaps	%xmm10, %xmm12
	vmovaps	%xmm13, %xmm5
.Ltmp11648:
	.loc	11 304 12
	cmpq	%rcx, %rax
	jne	.LBB36_497
.Ltmp11649:
.LBB36_498:
	.loc	11 0 12 is_stmt 0
	vmovss	48(%rsp), %xmm0
	.loc	1 1764 5 is_stmt 1
	vmovss	%xmm0, 740(%rsp)
	vmovss	%xmm13, 744(%rsp)
	vmovss	%xmm10, 748(%rsp)
	vmovss	%xmm11, 752(%rsp)
	vmovss	%xmm8, 756(%rsp)
	vmovss	%xmm4, 760(%rsp)
	vmovss	%xmm9, 764(%rsp)
	vmovss	%xmm2, 768(%rsp)
	vmovss	%xmm6, 772(%rsp)
	vmovss	%xmm7, 776(%rsp)
	vmovss	%xmm14, 780(%rsp)
	vmovss	%xmm3, 784(%rsp)
.Ltmp11650:
	.loc	5 438 16
	cmpq	136(%rsp), %rsi
	ja	.LBB36_625
.Ltmp11651:
	.loc	1 1759 23
	vmovss	496(%rsp), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	500(%rsp), %xmm13
	vmovss	504(%rsp), %xmm10
	vmovss	508(%rsp), %xmm11
	vmovss	512(%rsp), %xmm15
	vmovss	516(%rsp), %xmm3
	vmovss	520(%rsp), %xmm7
	vmovss	524(%rsp), %xmm0
	vmovss	528(%rsp), %xmm2
	vmovss	532(%rsp), %xmm8
	vmovss	536(%rsp), %xmm9
.Ltmp11652:
	.loc	11 304 12
	cmpq	%rdi, 120(%rsp)
.Ltmp11653:
	.loc	11 304 12 is_stmt 0
	jne	.LBB36_501
.Ltmp11654:
	.loc	1 0 0
	vmovss	540(%rsp), %xmm5
.Ltmp11655:
	.loc	11 304 12
	jmp	.LBB36_503
.Ltmp11656:
.LBB36_501:
	.loc	11 0 12
	movq	696(%rsp), %rdx
	vmovaps	%xmm0, %xmm5
	vmovss	584(%rdx), %xmm0
	vmovss	%xmm0, 64(%rsp)
	vmovss	588(%rdx), %xmm0
	vmovss	%xmm0, 592(%rsp)
	vmovss	592(%rdx), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	596(%rdx), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	600(%rdx), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	604(%rdx), %xmm0
	vmovss	%xmm0, 488(%rsp)
	xorl	%ecx, %ecx
	vmovss	%xmm9, 16(%rsp)
	vmovss	%xmm8, 40(%rsp)
	vmovss	%xmm2, 432(%rsp)
	vmovaps	%xmm5, %xmm14
	vmovss	%xmm7, 368(%rsp)
	vmovaps	%xmm3, 256(%rsp)
	vmovaps	%xmm15, %xmm1
	vmovaps	%xmm11, %xmm6
	vmovaps	%xmm10, %xmm12
	vmovaps	%xmm13, %xmm4
	vmovss	608(%rdx), %xmm0
	vmovss	%xmm0, 152(%rsp)
	vmovss	612(%rdx), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	616(%rdx), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	620(%rdx), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	624(%rdx), %xmm0
	vmovss	%xmm0, 232(%rsp)
	vmovss	628(%rdx), %xmm0
	vmovss	%xmm0, 168(%rsp)
	vmovss	632(%rdx), %xmm0
	vmovss	%xmm0, 224(%rsp)
	vmovss	636(%rdx), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	640(%rdx), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	644(%rdx), %xmm0
	vmovss	%xmm0, 216(%rsp)
	vmovss	648(%rdx), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	652(%rdx), %xmm0
	vmovss	%xmm0, 208(%rsp)
	vmovss	656(%rdx), %xmm0
	vmovss	%xmm0, 60(%rsp)
	vmovss	660(%rdx), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	664(%rdx), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	668(%rdx), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	672(%rdx), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	676(%rdx), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	680(%rdx), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	684(%rdx), %xmm0
	vmovss	%xmm0, 76(%rsp)
	vmovss	688(%rdx), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	692(%rdx), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	696(%rdx), %xmm0
	vmovss	%xmm0, 296(%rsp)
	vmovss	700(%rdx), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	704(%rdx), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	708(%rdx), %xmm0
	vmovss	%xmm0, 288(%rsp)
	vmovss	712(%rdx), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	716(%rdx), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	720(%rdx), %xmm0
	vmovss	%xmm0, 4(%rsp)
	vmovss	724(%rdx), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	728(%rdx), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	732(%rdx), %xmm0
	vmovss	%xmm0, 688(%rsp)
	vmovss	736(%rdx), %xmm0
	vmovss	%xmm0, 680(%rsp)
	vmovss	740(%rdx), %xmm0
	vmovss	%xmm0, 672(%rsp)
	vmovss	744(%rdx), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	748(%rdx), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	752(%rdx), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	756(%rdx), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	760(%rdx), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	764(%rdx), %xmm0
	vmovss	%xmm0, 648(%rsp)
	vmovss	768(%rdx), %xmm0
	vmovss	%xmm0, 640(%rsp)
	vmovss	772(%rdx), %xmm0
	vmovss	%xmm0, 92(%rsp)
	.p2align	4
.LBB36_502:
	vmovss	48(%rsp), %xmm9
.Ltmp11657:
	.loc	52 51 9 is_stmt 1
	vmovss	(%r11,%rcx,4), %xmm8
	vmovss	%xmm8, 48(%rsp)
.Ltmp11658:
	.loc	52 71 9
	vmulss	64(%rsp), %xmm8, %xmm2
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp11659:
	.loc	52 61 9
	vaddss	%xmm0, %xmm2, %xmm2
.Ltmp11660:
	.loc	52 71 9
	vmulss	592(%rsp), %xmm8, %xmm3
.Ltmp11661:
	.loc	52 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp11662:
	.loc	52 71 9
	vmulss	160(%rsp), %xmm8, %xmm5
.Ltmp11663:
	.loc	52 61 9
	vaddss	%xmm0, %xmm5, %xmm5
	vmovss	%xmm9, 176(%rsp)
.Ltmp11664:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm9, %xmm7
.Ltmp11665:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp11666:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm9, %xmm7
.Ltmp11667:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp11668:
	.loc	52 71 9
	vmulss	152(%rsp), %xmm9, %xmm7
.Ltmp11669:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp11670:
	.loc	52 71 9
	vmulss	112(%rsp), %xmm8, %xmm7
.Ltmp11671:
	.loc	52 61 9
	vaddss	%xmm0, %xmm7, %xmm7
.Ltmp11672:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm9, %xmm8
.Ltmp11673:
	.loc	52 61 9
	vaddss	%xmm7, %xmm8, %xmm7
	vmovaps	%xmm4, %xmm10
.Ltmp11674:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm4, %xmm4
.Ltmp11675:
	.loc	52 61 9
	vaddss	%xmm4, %xmm2, %xmm2
.Ltmp11676:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm10, %xmm4
.Ltmp11677:
	.loc	52 61 9
	vaddss	%xmm4, %xmm3, %xmm3
.Ltmp11678:
	.loc	52 71 9
	vmulss	232(%rsp), %xmm10, %xmm4
.Ltmp11679:
	.loc	52 61 9
	vaddss	%xmm4, %xmm5, %xmm4
.Ltmp11680:
	.loc	52 71 9
	vmulss	168(%rsp), %xmm10, %xmm5
.Ltmp11681:
	.loc	52 61 9
	vaddss	%xmm5, %xmm7, %xmm5
	vmovaps	%xmm12, %xmm11
.Ltmp11682:
	.loc	52 71 9
	vmulss	224(%rsp), %xmm12, %xmm7
.Ltmp11683:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp11684:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm12, %xmm7
.Ltmp11685:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp11686:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm12, %xmm7
.Ltmp11687:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp11688:
	.loc	52 71 9
	vmulss	216(%rsp), %xmm12, %xmm7
.Ltmp11689:
	.loc	52 61 9
	vaddss	%xmm7, %xmm5, %xmm5
	vmovaps	%xmm6, %xmm15
.Ltmp11690:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm6, %xmm6
.Ltmp11691:
	.loc	52 61 9
	vaddss	%xmm6, %xmm2, %xmm2
.Ltmp11692:
	.loc	52 71 9
	vmulss	208(%rsp), %xmm15, %xmm6
.Ltmp11693:
	.loc	52 61 9
	vaddss	%xmm6, %xmm3, %xmm6
.Ltmp11694:
	.loc	52 71 9
	vmulss	60(%rsp), %xmm15, %xmm3
.Ltmp11695:
	.loc	52 61 9
	vaddss	%xmm3, %xmm4, %xmm4
.Ltmp11696:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm15, %xmm3
.Ltmp11697:
	.loc	52 61 9
	vaddss	%xmm3, %xmm5, %xmm5
	vmovaps	%xmm1, %xmm3
.Ltmp11698:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm1, %xmm1
.Ltmp11699:
	.loc	52 61 9
	vaddss	%xmm1, %xmm2, %xmm1
.Ltmp11700:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm3, %xmm2
.Ltmp11701:
	.loc	52 61 9
	vaddss	%xmm2, %xmm6, %xmm2
.Ltmp11702:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm3, %xmm6
.Ltmp11703:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
.Ltmp11704:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm3, %xmm6
.Ltmp11705:
	.loc	52 61 9
	vaddss	%xmm6, %xmm5, %xmm5
	vmovaps	%xmm14, %xmm7
	vmovaps	256(%rsp), %xmm14
.Ltmp11706:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm14, %xmm6
.Ltmp11707:
	.loc	52 61 9
	vaddss	%xmm6, %xmm1, %xmm1
.Ltmp11708:
	.loc	52 71 9
	vmulss	76(%rsp), %xmm14, %xmm6
.Ltmp11709:
	.loc	52 61 9
	vaddss	%xmm6, %xmm2, %xmm2
.Ltmp11710:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm14, %xmm6
.Ltmp11711:
	.loc	52 61 9
	vaddss	%xmm6, %xmm4, %xmm4
.Ltmp11712:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm14, %xmm6
.Ltmp11713:
	.loc	52 61 9
	vaddss	%xmm6, %xmm5, %xmm5
	vmovss	368(%rsp), %xmm0
.Ltmp11714:
	.loc	52 71 9
	vmulss	296(%rsp), %xmm0, %xmm6
.Ltmp11715:
	.loc	52 61 9
	vaddss	%xmm6, %xmm1, %xmm1
.Ltmp11716:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm0, %xmm6
.Ltmp11717:
	.loc	52 61 9
	vaddss	%xmm6, %xmm2, %xmm6
.Ltmp11718:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm0, %xmm2
.Ltmp11719:
	.loc	52 61 9
	vaddss	%xmm2, %xmm4, %xmm4
.Ltmp11720:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm0, %xmm2
.Ltmp11721:
	.loc	52 61 9
	vaddss	%xmm2, %xmm5, %xmm5
.Ltmp11722:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm7, %xmm8
.Ltmp11723:
	.loc	52 61 9
	vaddss	%xmm1, %xmm8, %xmm1
.Ltmp11724:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm7, %xmm8
.Ltmp11725:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11726:
	.loc	52 71 9
	vmulss	4(%rsp), %xmm7, %xmm8
.Ltmp11727:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11728:
	.loc	52 71 9
	vmulss	80(%rsp), %xmm7, %xmm8
.Ltmp11729:
	.loc	52 61 9
	vaddss	%xmm5, %xmm8, %xmm5
	vmovss	432(%rsp), %xmm8
.Ltmp11730:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm8, %xmm12
.Ltmp11731:
	.loc	52 61 9
	vaddss	%xmm1, %xmm12, %xmm1
.Ltmp11732:
	.loc	52 71 9
	vmulss	688(%rsp), %xmm8, %xmm12
.Ltmp11733:
	.loc	52 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp11734:
	.loc	52 71 9
	vmulss	680(%rsp), %xmm8, %xmm12
.Ltmp11735:
	.loc	52 61 9
	vaddss	%xmm4, %xmm12, %xmm4
.Ltmp11736:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm8, %xmm12
.Ltmp11737:
	.loc	52 61 9
	vaddss	%xmm5, %xmm12, %xmm5
	vmovss	40(%rsp), %xmm9
.Ltmp11738:
	.loc	52 71 9
	vmulss	32(%rsp), %xmm9, %xmm12
.Ltmp11739:
	.loc	52 61 9
	vaddss	%xmm1, %xmm12, %xmm1
.Ltmp11740:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm9, %xmm12
.Ltmp11741:
	.loc	52 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp11742:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm9, %xmm12
.Ltmp11743:
	.loc	52 61 9
	vaddss	%xmm4, %xmm12, %xmm4
.Ltmp11744:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm9, %xmm12
.Ltmp11745:
	.loc	52 61 9
	vaddss	%xmm5, %xmm12, %xmm12
	vmovss	16(%rsp), %xmm5
.Ltmp11746:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm5, %xmm13
.Ltmp11747:
	.loc	52 61 9
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp11748:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm5, %xmm13
.Ltmp11749:
	.loc	52 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp11750:
	.loc	52 71 9
	vmulss	640(%rsp), %xmm5, %xmm13
.Ltmp11751:
	.loc	52 61 9
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp11752:
	.loc	52 71 9
	vmulss	92(%rsp), %xmm5, %xmm13
.Ltmp11753:
	.loc	52 61 9
	vaddss	%xmm13, %xmm12, %xmm12
	vbroadcastss	.LCPI36_0(%rip), %xmm2
.Ltmp11754:
	.loc	52 103 24
	vandps	%xmm2, %xmm1, %xmm1
.Ltmp11755:
	.loc	52 103 24 is_stmt 0
	vandps	%xmm2, %xmm14, %xmm13
.Ltmp11756:
	.loc	52 161 24 is_stmt 1
	vmaxss	%xmm1, %xmm13, %xmm1
	vmovss	176(%rsp), %xmm13
.Ltmp11757:
	.loc	52 103 24
	vandps	%xmm2, %xmm6, %xmm6
.Ltmp11758:
	.loc	52 161 24
	vmaxss	%xmm6, %xmm1, %xmm1
.Ltmp11759:
	.loc	52 103 24
	vandps	%xmm2, %xmm4, %xmm4
.Ltmp11760:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm1, %xmm1
.Ltmp11761:
	.loc	52 103 24
	vandps	%xmm2, %xmm12, %xmm4
	vmovaps	%xmm7, %xmm2
	vmovaps	%xmm14, %xmm7
.Ltmp11762:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm1, %xmm1
.Ltmp11763:
	.loc	52 56 9
	vmovss	%xmm1, 1616(%rsp,%rcx,4)
.Ltmp11764:
	.loc	11 308 13
	incq	%rcx
	vmovss	%xmm9, 16(%rsp)
	vmovss	%xmm8, 40(%rsp)
	vmovss	%xmm2, 432(%rsp)
	vmovaps	%xmm0, %xmm14
	vmovss	%xmm7, 368(%rsp)
	vmovaps	%xmm3, 256(%rsp)
	vmovaps	%xmm15, %xmm1
	vmovaps	%xmm11, %xmm6
	vmovaps	%xmm10, %xmm12
	vmovaps	%xmm13, %xmm4
.Ltmp11765:
	.loc	11 304 12
	cmpq	%rcx, %rax
	jne	.LBB36_502
.Ltmp11766:
.LBB36_503:
	.loc	11 0 12 is_stmt 0
	vmovss	48(%rsp), %xmm1
	.loc	1 1764 5 is_stmt 1
	vmovss	%xmm1, 496(%rsp)
	vmovss	%xmm13, 500(%rsp)
	vmovss	%xmm10, 504(%rsp)
	vmovss	%xmm11, 508(%rsp)
	vmovss	%xmm15, 512(%rsp)
	vmovss	%xmm3, 516(%rsp)
	vmovss	%xmm7, 520(%rsp)
	vmovss	%xmm0, 524(%rsp)
	vmovss	%xmm2, 528(%rsp)
	vmovss	%xmm8, 532(%rsp)
	vmovss	%xmm9, 536(%rsp)
	vmovss	%xmm5, 540(%rsp)
	movq	120(%rsp), %rcx
.Ltmp11767:
	.loc	11 304 12
	cmpq	%rdi, %rcx
.Ltmp11768:
	.loc	1 1984 19
	je	.LBB36_491
	.loc	1 0 19 is_stmt 0
	movq	%r14, 640(%rsp)
	movq	%rbx, 648(%rsp)
	movq	%r11, 656(%rsp)
	movq	%rdi, 80(%rsp)
	movq	%r8, 664(%rsp)
	movq	1480(%rsp), %rdi
	movq	1488(%rsp), %r9
	movq	904(%rsp), %r10
	movq	912(%rsp), %r12
	movq	1424(%rsp), %rcx
	movq	1432(%rsp), %rdx
	movq	1472(%rsp), %rsi
	movq	1440(%rsp), %rax
	movq	%rax, 224(%rsp)
	movq	1448(%rsp), %rax
	movq	%rax, 320(%rsp)
	movq	1464(%rsp), %rax
	movq	%rax, 208(%rsp)
	movq	1456(%rsp), %rax
	movq	%rax, 312(%rsp)
	movq	848(%rsp), %r13
	movq	896(%rsp), %rax
	vmovss	1496(%rsp), %xmm10
	vmovss	920(%rsp), %xmm9
	.loc	1 1984 19
	cmpq	$1, %rsi
	movq	%rsi, 336(%rsp)
	adcq	$0, %rsi
	movq	%rsi, 328(%rsp)
	cmpq	$1, %rax
	movq	%rax, 480(%rsp)
	adcq	$0, %rax
	movq	%rax, 472(%rsp)
	vmovss	788(%rsp), %xmm15
	vmovss	792(%rsp), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	808(%rsp), %xmm0
	vmovss	%xmm0, 296(%rsp)
	vmovss	544(%rsp), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	548(%rsp), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	564(%rsp), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	828(%rsp), %xmm0
	vmovss	%xmm0, 60(%rsp)
	vmovss	584(%rsp), %xmm0
	vmovss	%xmm0, 76(%rsp)
	vmovss	800(%rsp), %xmm12
	vmovss	796(%rsp), %xmm0
	vmovaps	%xmm0, 432(%rsp)
	vmovss	816(%rsp), %xmm2
	vmovss	804(%rsp), %xmm5
	vmovss	812(%rsp), %xmm0
	vmovaps	%xmm0, 368(%rsp)
	vmovss	556(%rsp), %xmm0
	vmovss	%xmm0, 4(%rsp)
	movq	856(%rsp), %r14
	movq	864(%rsp), %rax
	movq	%rax, 424(%rsp)
	movq	872(%rsp), %rax
	movq	%rax, 304(%rsp)
	movq	888(%rsp), %rax
	movq	%rax, 416(%rsp)
	movq	880(%rsp), %rax
	movq	%rax, 464(%rsp)
	movl	1500(%rsp), %eax
	movq	%rax, 48(%rsp)
	movl	924(%rsp), %eax
	movq	%rax, 280(%rsp)
	xorl	%r8d, %r8d
	vmovss	552(%rsp), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	572(%rsp), %xmm0
	vmovss	560(%rsp), %xmm1
	vmovss	%xmm1, 40(%rsp)
	vmovss	568(%rsp), %xmm4
	vmovss	824(%rsp), %xmm13
	vmovss	820(%rsp), %xmm8
	vmovss	580(%rsp), %xmm11
	vmovss	576(%rsp), %xmm1
	vmovss	%xmm9, 248(%rsp)
	vmovaps	%xmm10, %xmm6
	movq	632(%rsp), %r11
	movq	448(%rsp), %rbx
	xorl	%r15d, %r15d
	vmovss	.LCPI36_1(%rip), %xmm3
	movq	616(%rsp), %rax
	movq	%rdi, 392(%rsp)
	movq	%r9, 688(%rsp)
	movq	%r10, 680(%rsp)
	movq	%r12, 672(%rsp)
.LBB36_505:
	.loc	1 0 19
	movq	%r8, 288(%rsp)
	.loc	1 1989 21 is_stmt 1
	subq	%r8, %rax
	movq	696(%rsp), %rsi
	movq	%r10, %rbp
	movq	%r12, %r8
	movq	%r9, %r12
.Ltmp11769:
	.loc	1 1575 16
	movq	544(%rsi), %r9
.Ltmp11770:
	.loc	1 1576 16
	movq	552(%rsi), %rsi
	movq	%rsi, 176(%rsp)
.Ltmp11771:
	.loc	1 1577 25
	leaq	1(%rbx), %rsi
.Ltmp11772:
	.loc	1 1148 8
	cmpq	%r9, %rsi
	movq	%r9, %rsi
	cmovbq	%r15, %rsi
	negq	%rsi
.Ltmp11773:
	.loc	1 1578 28
	leaq	(%rdi,%rbx), %r10
.Ltmp11774:
	.loc	1 1148 8
	cmpq	%r9, %r10
	movq	%r9, %rdi
	cmovbq	%r15, %rdi
	subq	%rdi, %r10
	movq	%r10, 488(%rsp)
.Ltmp11775:
	.loc	1 1579 29
	addq	%rbx, %rbp
.Ltmp11776:
	.loc	1 1148 8
	cmpq	%r9, %rbp
	movq	%r9, %rdi
	cmovbq	%r15, %rdi
	subq	%rdi, %rbp
.Ltmp11777:
	.loc	1 1580 33
	leaq	(%r12,%rbx), %r10
	movq	176(%rsp), %r12
.Ltmp11778:
	.loc	1 1148 8
	cmpq	%r9, %r10
	movq	%r9, %rdi
	cmovbq	%r15, %rdi
	subq	%rdi, %r10
	movq	%r10, 152(%rsp)
.Ltmp11779:
	.loc	1 1581 34
	addq	%rbx, %r8
.Ltmp11780:
	.loc	1 1148 8
	cmpq	%r9, %r8
	movq	%r9, %rdi
	cmovbq	%r15, %rdi
.Ltmp11781:
	.loc	1 1148 8 is_stmt 0
	leaq	(%rbx,%rsi), %r15
	incq	%r15
.Ltmp11782:
	.loc	1 1148 8
	subq	%rdi, %r8
.Ltmp11783:
	.loc	1 1583 14 is_stmt 1
	movq	%r9, %r10
	movq	%rbx, 448(%rsp)
	subq	%rbx, %r10
.Ltmp11784:
	.loc	10 1078 5
	cmpq	%rax, %r10
	cmovbq	%r10, %rax
	movq	%r11, 632(%rsp)
.Ltmp11785:
	.loc	1 1584 14
	subq	%r11, %r12
.Ltmp11786:
	.loc	10 1078 5
	cmpq	%rax, %r12
	cmovbq	%r12, %rax
.Ltmp11787:
	.loc	1 1585 14
	movq	%r9, %rbx
	movq	%r15, 240(%rsp)
	subq	%r15, %rbx
.Ltmp11788:
	.loc	10 1078 5
	cmpq	%rax, %rbx
	cmovbq	%rbx, %rax
.Ltmp11789:
	.loc	1 1586 14
	movq	%r9, %r15
	movq	488(%rsp), %rsi
	subq	%rsi, %r15
.Ltmp11790:
	.loc	10 1078 5
	cmpq	%rax, %r15
	cmovbq	%r15, %rax
.Ltmp11791:
	.loc	1 1587 14
	movq	%r9, %r11
	movq	%rbp, 352(%rsp)
	subq	%rbp, %r11
.Ltmp11792:
	.loc	10 1078 5
	cmpq	%rax, %r11
	cmovbq	%r11, %rax
.Ltmp11793:
	.loc	1 1588 14
	movq	%r9, %rbp
	movq	152(%rsp), %rsi
	subq	%rsi, %rbp
.Ltmp11794:
	.loc	10 1078 5
	cmpq	%rax, %rbp
	cmovbq	%rbp, %rax
	movq	%r8, 344(%rsp)
.Ltmp11795:
	.loc	1 1589 14
	subq	%r8, %r9
.Ltmp11796:
	.loc	10 1078 5
	cmpq	%rax, %r9
	cmovbq	%r9, %rax
	movq	80(%rsp), %rsi
	movq	288(%rsp), %rdi
.Ltmp11797:
	.loc	1 1995 28
	addq	%rsi, %rdi
	movq	%rax, 400(%rsp)
.Ltmp11798:
	.loc	1 1997 55
	movq	%rax, %rsi
	addq	%rdi, %rsi
.Ltmp11799:
	.loc	4 1050 16
	jb	.LBB36_616
	cmpq	968(%rsp), %rsi
	ja	.LBB36_616
.Ltmp11800:
	.loc	5 451 16
	cmpq	136(%rsp), %rsi
	movq	280(%rsp), %rax
	ja	.LBB36_623
.Ltmp11801:
	.loc	11 304 12
	cmpq	$0, 400(%rsp)
	je	.LBB36_552
	.loc	11 0 12 is_stmt 0
	vmovaps	%xmm8, 176(%rsp)
	vmovss	%xmm11, 64(%rsp)
	vmovss	%xmm10, 28(%rsp)
	vmovaps	%xmm1, 592(%rsp)
	vmovss	%xmm9, 32(%rsp)
	.loc	11 304 12
	cmpq	%r11, %r15
	cmovbq	%r15, %r11
	cmpq	%rbp, %r11
	cmovaeq	%rbp, %r11
	movq	976(%rsp), %rsi
.Ltmp11802:
	.loc	1 0 0
	leaq	(%rsi,%rdi,4), %rsi
	movq	%rsi, 160(%rsp)
.Ltmp11803:
	.loc	11 304 12
	cmpq	%r9, %r11
	cmovaeq	%r9, %r11
	movq	840(%rsp), %rsi
.Ltmp11804:
	.loc	1 0 0
	leaq	(%rsi,%rdi,4), %rsi
	movq	%rsi, 112(%rsp)
.Ltmp11805:
	.loc	11 304 12
	cmpq	%rbx, %r11
	cmovaeq	%rbx, %r11
	movq	288(%rsp), %rsi
.Ltmp11806:
	.loc	1 0 0
	leaq	(%rsp,%rsi,4), %rdi
	addq	$2648, %rdi
	movq	%rdi, 232(%rsp)
.Ltmp11807:
	.loc	11 304 12
	cmpq	%r10, %r11
	cmovaeq	%r10, %r11
	cmpq	%r12, %r11
	cmovaeq	%r12, %r11
	movq	96(%rsp), %rdi
	subq	%rsi, %rdi
	cmpq	%rdi, %r11
	cmovbq	%r11, %rdi
	movq	%rdi, 216(%rsp)
.Ltmp11808:
	.loc	1 0 0
	leaq	1616(%rsp,%rsi,4), %rsi
	movq	%rsi, 168(%rsp)
	vmovss	4(%rsp), %xmm1
	vmovss	%xmm12, 24(%rsp)
	vmovaps	%xmm12, %xmm8
	xorl	%r15d, %r15d
	vxorps	%xmm11, %xmm11, %xmm11
	.p2align	4
.LBB36_510:
.Ltmp11809:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm3, %xmm8, %xmm9
.Ltmp11810:
	.loc	52 124 14
	vucomiss	%xmm11, %xmm9
	vxorps	%xmm7, %xmm7, %xmm7
.Ltmp11811:
	.loc	52 161 24
	ja	.LBB36_512
.Ltmp11812:
	.loc	52 0 24 is_stmt 0
	vmovss	456(%rsp), %xmm15
	jmp	.LBB36_513
	.p2align	4
.LBB36_512:
	.loc	1 853 0 is_stmt 1
	vaddss	432(%rsp), %xmm15, %xmm15
.Ltmp11813:
.LBB36_513:
	.loc	1 0 0 is_stmt 0
	movq	448(%rsp), %rsi
	movl	704(%rsp), %r8d
	vmovss	.LCPI36_2(%rip), %xmm11
	movq	224(%rsp), %r12
	vmovaps	176(%rsp), %xmm8
	vmovaps	%xmm5, %xmm12
.Ltmp11814:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm3, %xmm2, %xmm14
.Ltmp11815:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm14
.Ltmp11816:
	.loc	52 161 24
	ja	.LBB36_517
.Ltmp11817:
	.loc	52 0 24 is_stmt 0
	vmovss	296(%rsp), %xmm12
.Ltmp11818:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm3, %xmm1, %xmm10
.Ltmp11819:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm10
.Ltmp11820:
	.loc	52 161 24
	jbe	.LBB36_518
.Ltmp11821:
.LBB36_515:
	.loc	52 0 24 is_stmt 0
	vmovss	16(%rsp), %xmm1
	.loc	1 853 0 is_stmt 1
	vaddss	256(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 16(%rsp)
.Ltmp11822:
	.loc	52 66 9
	vaddss	%xmm3, %xmm0, %xmm3
.Ltmp11823:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm3
.Ltmp11824:
	.loc	52 161 24
	jbe	.LBB36_519
.Ltmp11825:
.LBB36_516:
	.loc	52 0 24 is_stmt 0
	vmovss	40(%rsp), %xmm0
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm4, %xmm0, %xmm0
	jmp	.LBB36_520
.Ltmp11826:
	.loc	1 0 0 is_stmt 0
.Ltmp11827:
	.p2align	4
.LBB36_517:
	.loc	1 853 0 is_stmt 1
	vaddss	368(%rsp), %xmm12, %xmm12
.Ltmp11828:
	.loc	52 66 9
	vaddss	%xmm3, %xmm1, %xmm10
.Ltmp11829:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm10
.Ltmp11830:
	.loc	52 161 24
	ja	.LBB36_515
.Ltmp11831:
.LBB36_518:
	.loc	52 0 24 is_stmt 0
	vmovss	200(%rsp), %xmm1
	vmovss	%xmm1, 16(%rsp)
.Ltmp11832:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm3, %xmm0, %xmm3
.Ltmp11833:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm3
.Ltmp11834:
	.loc	52 161 24
	ja	.LBB36_516
.Ltmp11835:
.LBB36_519:
	.loc	52 0 24 is_stmt 0
	vmovss	408(%rsp), %xmm0
.LBB36_520:
	vmovss	%xmm0, 40(%rsp)
	movq	232(%rsp), %rdi
.Ltmp11836:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rdi,%r15,4), %xmm0
	movq	168(%rsp), %rdi
.Ltmp11837:
	.loc	52 51 9 is_stmt 0
	vmovss	(%rdi,%r15,4), %xmm1
.Ltmp11838:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
.Ltmp11839:
	.loc	41 1244 18
	vmovd	%xmm1, %r9d
.Ltmp11840:
	.loc	41 1244 18 is_stmt 0
	vmovd	%xmm0, %edi
.Ltmp11841:
	.loc	52 161 24 is_stmt 1
	movl	%r9d, %ebx
	cmovbel	%edi, %ebx
.Ltmp11842:
	.loc	1 1502 26
	leaq	(%r15,%rsi), %r10
.Ltmp11843:
	.loc	4 1050 16
	cmpq	%rdx, %r10
	jae	.LBB36_585
.Ltmp11844:
	.loc	1 0 0 is_stmt 0
	andl	728(%rsp), %ebx
.Ltmp11845:
	.loc	52 161 44 is_stmt 1
	andl	%r8d, %edi
	.loc	52 161 24 is_stmt 0
	orl	%ebx, %edi
.Ltmp11846:
	.loc	41 1291 18 is_stmt 1
	vmovd	%edi, %xmm0
.Ltmp11847:
	.loc	52 124 14
	vucomiss	%xmm15, %xmm0
	vmovaps	%xmm11, %xmm1
.Ltmp11848:
	.loc	52 161 24
	jbe	.LBB36_523
.Ltmp11849:
	.loc	52 76 9
	vdivss	%xmm0, %xmm15, %xmm1
.Ltmp11850:
.LBB36_523:
	.loc	52 0 9 is_stmt 0
	movq	160(%rsp), %rdi
	vmovss	(%rdi,%r15,4), %xmm2
	movq	112(%rsp), %rdi
	vmovss	(%rdi,%r15,4), %xmm0
	movq	488(%rsp), %rdi
	addq	%r15, %rdi
.Ltmp11851:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm1, (%rcx,%r10,4)
.Ltmp11852:
	.loc	4 1050 16
	cmpq	%rdx, %rdi
	jae	.LBB36_563
.Ltmp11853:
	.loc	4 0 16 is_stmt 0
	movq	%rax, %r8
.Ltmp11854:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm1
	vmovaps	%xmm1, %xmm7
.Ltmp11855:
	.loc	1 1206 22
	cmpl	$0, 48(%rsp)
	je	.LBB36_526
.Ltmp11856:
	.loc	1 0 22 is_stmt 0
	vminss	%xmm1, %xmm6, %xmm7
.LBB36_526:
	movq	240(%rsp), %rax
	leaq	(%r15,%rax), %rbp
	movq	48(%rsp), %rax
.Ltmp11857:
	movl	%eax, %eax
.Ltmp11858:
	.loc	1 1212 20 is_stmt 1
	incq	%rax
	movq	328(%rsp), %r11
	movq	%rax, 48(%rsp)
	cmpq	336(%rsp), %rax
	movq	%r8, %rax
.Ltmp11859:
	.loc	1 1213 22
	jne	.LBB36_530
	.loc	1 0 22 is_stmt 0
.Ltmp11860:
	.p2align	4
.LBB36_527:
.Ltmp11861:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB36_563
.Ltmp11862:
	.loc	52 161 24
	vminss	(%rcx,%rdi,4), %xmm1, %xmm1
.Ltmp11863:
	.loc	52 56 9
	vmovss	%xmm1, (%rcx,%rdi,4)
.Ltmp11864:
	.loc	1 1224 16
	testq	%rdi, %rdi
	cmoveq	8(%rsp), %rdi
	.loc	1 1227 13
	decq	%rdi
.Ltmp11865:
	.loc	10 1916 50
	decq	%r11
.Ltmp11866:
	.loc	3 900 12
	jne	.LBB36_527
.Ltmp11867:
	.loc	3 0 12 is_stmt 0
	movq	$0, 48(%rsp)
	vmovaps	%xmm7, %xmm1
	movq	%rbp, %r8
	jmp	.LBB36_532
	.p2align	4
.LBB36_530:
	movq	%rbp, %r8
.Ltmp11868:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rdx, %rbp
	jae	.LBB36_605
.Ltmp11869:
	.loc	52 51 9
	vmovss	(%rcx,%r8,4), %xmm1
.Ltmp11870:
	.loc	52 161 24
	vminss	%xmm7, %xmm1, %xmm1
.Ltmp11871:
.LBB36_532:
	.loc	52 0 24 is_stmt 0
	movq	152(%rsp), %rdi
	addq	%r15, %rdi
.Ltmp11872:
	.loc	52 71 9 is_stmt 1
	vmulss	.LCPI36_3(%rip), %xmm1, %xmm1
.Ltmp11873:
	.loc	41 1783 9
	vroundss	$9, %xmm1, %xmm1, %xmm1
.Ltmp11874:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm1, %xmm1
	movq	320(%rsp), %r11
.Ltmp11875:
	.loc	4 1050 16
	cmpq	%r11, %rdi
	jae	.LBB36_586
.Ltmp11876:
	.loc	1 0 0 is_stmt 0
	addq	%r15, %rsi
	incq	%rsi
.Ltmp11877:
	.loc	52 61 9 is_stmt 1
	vaddss	%xmm1, %xmm13, %xmm6
.Ltmp11878:
	.loc	52 66 9
	vsubss	(%r12,%rdi,4), %xmm6, %xmm6
	cmpq	%r11, %rsi
.Ltmp11879:
	.loc	4 1050 16
	ja	.LBB36_587
.Ltmp11880:
	.loc	4 0 16 is_stmt 0
	movq	632(%rsp), %rdi
	leaq	(%r15,%rdi), %r11
.Ltmp11881:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm1, (%r12,%r10,4)
	vmovaps	%xmm6, %xmm13
.Ltmp11882:
	.loc	52 76 9
	vdivss	60(%rsp), %xmm6, %xmm1
.Ltmp11883:
	.loc	52 66 9
	vsubss	%xmm1, %xmm11, %xmm1
.Ltmp11884:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm8, %xmm1, %xmm6
	vmovaps	%xmm12, %xmm5
.Ltmp11885:
	.loc	52 92 9 is_stmt 1
	vmulss	%xmm6, %xmm12, %xmm6
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11886:
	.loc	52 161 24
	vmaxss	%xmm6, %xmm1, %xmm6
.Ltmp11887:
	.loc	52 103 24
	vbroadcastss	.LCPI36_0(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm8
.Ltmp11888:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm8, %xmm8
	vandps	%xmm6, %xmm8, %xmm12
.Ltmp11889:
	.loc	4 1050 16
	cmpq	208(%rsp), %r11
	jae	.LBB36_589
.Ltmp11890:
	.loc	52 66 9
	vsubss	%xmm12, %xmm11, %xmm6
	movq	312(%rsp), %rdi
.Ltmp11891:
	.loc	52 51 9
	vmovss	(%rdi,%r11,4), %xmm8
.Ltmp11892:
	.loc	52 56 9
	vmovss	%xmm2, (%rdi,%r11,4)
.Ltmp11893:
	.loc	52 71 9
	vmulss	%xmm6, %xmm8, %xmm2
.Ltmp11894:
	.loc	41 1244 18
	vmovd	%xmm8, %edi
.Ltmp11895:
	.loc	52 161 24
	andl	624(%rsp), %edi
.Ltmp11896:
	.loc	41 1244 18
	vmovd	%xmm2, %ebp
.Ltmp11897:
	.loc	52 161 44
	andl	36(%rsp), %ebp
	.loc	52 161 24 is_stmt 0
	orl	%edi, %ebp
	movq	160(%rsp), %rdi
.Ltmp11898:
	.loc	52 56 9 is_stmt 1
	movl	%ebp, (%rdi,%r15,4)
	cmpq	%r14, %rsi
.Ltmp11899:
	.loc	4 1050 16
	ja	.LBB36_591
.Ltmp11900:
	.loc	1 0 0 is_stmt 0
	andl	704(%rsp), %r9d
	orl	%r9d, %ebx
	vmovd	%ebx, %xmm6
.Ltmp11901:
	.loc	52 124 14 is_stmt 1
	vucomiss	16(%rsp), %xmm6
	vmovaps	%xmm11, %xmm2
.Ltmp11902:
	.loc	52 161 24
	jbe	.LBB36_538
.Ltmp11903:
	.loc	52 0 24 is_stmt 0
	vmovss	16(%rsp), %xmm2
.Ltmp11904:
	.loc	52 76 9 is_stmt 1
	vdivss	%xmm6, %xmm2, %xmm2
.Ltmp11905:
.LBB36_538:
	.loc	52 0 9 is_stmt 0
	movq	352(%rsp), %rdi
	addq	%r15, %rdi
.Ltmp11906:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm2, (%r13,%r10,4)
.Ltmp11907:
	.loc	4 1050 16
	cmpq	%r14, %rdi
	movq	840(%rsp), %rbp
	movq	304(%rsp), %rbx
	jae	.LBB36_592
.Ltmp11908:
	.loc	4 0 16 is_stmt 0
	vmovaps	%xmm12, 176(%rsp)
.Ltmp11909:
	.loc	52 51 9 is_stmt 1
	vmovss	(%r13,%rdi,4), %xmm2
	vmovaps	%xmm2, %xmm12
.Ltmp11910:
	.loc	1 1206 22
	testl	%eax, %eax
	je	.LBB36_541
.Ltmp11911:
	.loc	1 0 22 is_stmt 0
	vmovss	248(%rsp), %xmm6
	vminss	%xmm2, %xmm6, %xmm12
.LBB36_541:
	movl	%eax, %eax
.Ltmp11912:
	.loc	1 1212 20 is_stmt 1
	incq	%rax
	movq	472(%rsp), %r9
	cmpq	480(%rsp), %rax
.Ltmp11913:
	.loc	1 1213 22
	jne	.LBB36_545
	.loc	1 0 22 is_stmt 0
.Ltmp11914:
	.p2align	4
.LBB36_542:
.Ltmp11915:
	.loc	4 1050 16 is_stmt 1
	cmpq	%r14, %rdi
	jae	.LBB36_564
.Ltmp11916:
	.loc	52 161 24
	vminss	(%r13,%rdi,4), %xmm2, %xmm2
.Ltmp11917:
	.loc	52 56 9
	vmovss	%xmm2, (%r13,%rdi,4)
.Ltmp11918:
	.loc	1 1224 16
	testq	%rdi, %rdi
	cmoveq	8(%rsp), %rdi
	.loc	1 1227 13
	decq	%rdi
.Ltmp11919:
	.loc	10 1916 50
	decq	%r9
.Ltmp11920:
	.loc	3 900 12
	jne	.LBB36_542
.Ltmp11921:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	vmovaps	%xmm12, %xmm2
	jmp	.LBB36_547
	.p2align	4
.LBB36_545:
.Ltmp11922:
	.loc	4 1050 16 is_stmt 1
	cmpq	%r14, %r8
	jae	.LBB36_606
.Ltmp11923:
	.loc	52 51 9
	vmovss	(%r13,%r8,4), %xmm2
.Ltmp11924:
	.loc	52 161 24
	vminss	%xmm12, %xmm2, %xmm2
.Ltmp11925:
.LBB36_547:
	.loc	52 0 24 is_stmt 0
	movq	344(%rsp), %rdi
	addq	%r15, %rdi
.Ltmp11926:
	.loc	52 71 9 is_stmt 1
	vmulss	.LCPI36_3(%rip), %xmm2, %xmm2
.Ltmp11927:
	.loc	41 1783 9
	vroundss	$9, %xmm2, %xmm2, %xmm2
.Ltmp11928:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm2, %xmm2
.Ltmp11929:
	.loc	4 1050 16
	cmpq	%rbx, %rdi
	jae	.LBB36_593
.Ltmp11930:
	.loc	52 61 9
	vaddss	64(%rsp), %xmm2, %xmm6
	movq	424(%rsp), %r8
.Ltmp11931:
	.loc	52 66 9
	vsubss	(%r8,%rdi,4), %xmm6, %xmm6
	cmpq	%rbx, %rsi
.Ltmp11932:
	.loc	4 1050 16
	ja	.LBB36_594
.Ltmp11933:
	.loc	1 0 0 is_stmt 0
	leaq	1(%r11), %rsi
.Ltmp11934:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm2, (%r8,%r10,4)
	vmovss	%xmm6, 64(%rsp)
.Ltmp11935:
	.loc	52 76 9
	vdivss	76(%rsp), %xmm6, %xmm2
.Ltmp11936:
	.loc	52 66 9
	vsubss	%xmm2, %xmm11, %xmm2
	vmovaps	592(%rsp), %xmm8
.Ltmp11937:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm8, %xmm2, %xmm6
.Ltmp11938:
	.loc	52 92 9 is_stmt 1
	vmulss	40(%rsp), %xmm6, %xmm6
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11939:
	.loc	52 161 24
	vmaxss	%xmm6, %xmm2, %xmm2
.Ltmp11940:
	.loc	52 103 24
	vandps	%xmm1, %xmm2, %xmm1
.Ltmp11941:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm1, %xmm1
	vandps	%xmm2, %xmm1, %xmm1
	cmpq	416(%rsp), %rsi
.Ltmp11942:
	.loc	4 1050 16
	ja	.LBB36_595
.Ltmp11943:
	.loc	4 0 16 is_stmt 0
	vmovaps	%xmm1, 592(%rsp)
.Ltmp11944:
	.loc	52 66 9 is_stmt 1
	vsubss	%xmm1, %xmm11, %xmm1
	movq	464(%rsp), %rsi
.Ltmp11945:
	.loc	52 51 9
	vmovss	(%rsi,%r11,4), %xmm2
.Ltmp11946:
	.loc	52 56 9
	vmovss	%xmm0, (%rsi,%r11,4)
.Ltmp11947:
	.loc	52 71 9
	vmulss	%xmm2, %xmm1, %xmm0
.Ltmp11948:
	.loc	41 1244 18
	vmovd	%xmm2, %esi
.Ltmp11949:
	.loc	52 161 24
	andl	624(%rsp), %esi
.Ltmp11950:
	.loc	41 1244 18
	vmovd	%xmm0, %edi
.Ltmp11951:
	.loc	52 161 44
	andl	36(%rsp), %edi
	.loc	52 161 24 is_stmt 0
	orl	%esi, %edi
	movq	112(%rsp), %rsi
.Ltmp11952:
	.loc	52 56 9 is_stmt 1
	movl	%edi, (%rsi,%r15,4)
.Ltmp11953:
	.loc	1 0 0 is_stmt 0
	incq	%r15
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp11954:
	vmaxss	%xmm11, %xmm9, %xmm8
	vcmpltss	%xmm9, %xmm11, %xmm0
	vmovaps	432(%rsp), %xmm1
	vandps	%xmm1, %xmm0, %xmm1
	vmovaps	%xmm1, 432(%rsp)
	vmaxss	%xmm11, %xmm14, %xmm2
	vcmpltss	%xmm14, %xmm11, %xmm0
	vmovaps	368(%rsp), %xmm1
	vandps	%xmm1, %xmm0, %xmm1
	vmovaps	%xmm1, 368(%rsp)
.Ltmp11955:
	vmaxss	%xmm11, %xmm10, %xmm1
	vcmpltss	%xmm10, %xmm11, %xmm0
	vmovaps	256(%rsp), %xmm6
	vandps	%xmm6, %xmm0, %xmm6
	vmovaps	%xmm6, 256(%rsp)
	vmaxss	%xmm11, %xmm3, %xmm0
	vcmpltss	%xmm3, %xmm11, %xmm3
	vandps	%xmm4, %xmm3, %xmm4
	vmovss	%xmm12, 248(%rsp)
	vmovaps	%xmm7, %xmm6
.Ltmp11956:
	.loc	11 304 12 is_stmt 1
	cmpq	216(%rsp), %r15
	vmovss	.LCPI36_1(%rip), %xmm3
	jne	.LBB36_510
.Ltmp11957:
	.loc	11 0 12 is_stmt 0
	movq	%rax, 280(%rsp)
.Ltmp11958:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
	vmovaps	176(%rsp), %xmm14
.Ltmp11959:
	.loc	1 1660 5
	vmovss	%xmm14, 820(%rsp)
	vmovss	64(%rsp), %xmm11
.Ltmp11960:
	.loc	1 1654 5
	vmovss	%xmm11, 580(%rsp)
	vmovaps	592(%rsp), %xmm6
.Ltmp11961:
	.loc	1 1660 5
	vmovss	%xmm6, 576(%rsp)
.Ltmp11962:
	.loc	1 853 0
	vmovss	%xmm15, 788(%rsp)
	vmovaps	432(%rsp), %xmm9
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm9, 796(%rsp)
.Ltmp11963:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm2, 816(%rsp)
.Ltmp11964:
	.loc	1 853 0
	vmovss	%xmm5, 804(%rsp)
	vmovaps	368(%rsp), %xmm9
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm9, 812(%rsp)
	vmovss	16(%rsp), %xmm9
.Ltmp11965:
	.loc	1 853 0
	vmovss	%xmm9, 544(%rsp)
	vmovaps	256(%rsp), %xmm9
	.loc	1 0 0
	vmovss	%xmm9, 552(%rsp)
.Ltmp11966:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm0, 572(%rsp)
	vmovss	40(%rsp), %xmm9
.Ltmp11967:
	.loc	1 853 0
	vmovss	%xmm9, 560(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm4, 568(%rsp)
	vmovaps	%xmm12, %xmm9
	vmovaps	%xmm7, %xmm10
	vmovss	%xmm1, 4(%rsp)
	vmovaps	%xmm6, %xmm1
	vmovss	%xmm12, 248(%rsp)
	vmovaps	%xmm8, %xmm12
	vmovaps	%xmm14, %xmm8
	vmovaps	%xmm7, %xmm6
	jmp	.LBB36_553
.Ltmp11968:
.LBB36_552:
	movq	840(%rsp), %rbp
.LBB36_553:
	movq	392(%rsp), %rdi
	movq	288(%rsp), %r8
	movq	400(%rsp), %rax
	addq	%rax, %r8
	movq	448(%rsp), %rbx
.Ltmp11969:
	.loc	1 2056 39 is_stmt 1
	addq	%rax, %rbx
	movq	8(%rsp), %rsi
.Ltmp11970:
	.loc	1 1148 8
	cmpq	%rsi, %rbx
	movl	$0, %r15d
	cmovbq	%r15, %rsi
	subq	%rsi, %rbx
	movq	632(%rsp), %r11
.Ltmp11971:
	.loc	1 2057 39
	addq	%rax, %r11
	movq	104(%rsp), %rax
.Ltmp11972:
	.loc	1 1148 8
	cmpq	%rax, %r11
	cmovbq	%r15, %rax
	subq	%rax, %r11
	movq	616(%rsp), %rax
.Ltmp11973:
	.loc	1 1984 19
	cmpq	%rax, %r8
	movq	688(%rsp), %r9
	movq	680(%rsp), %r10
	movq	672(%rsp), %r12
	jb	.LBB36_505
	jmp	.LBB36_490
.Ltmp11974:
.LBB36_554:
	.loc	1 0 19 is_stmt 0
	movq	696(%rsp), %r12
.Ltmp11975:
	.loc	52 56 9 is_stmt 1
	cmpq	$0, 224(%r12)
	je	.LBB36_581
.Ltmp11976:
	.loc	1 0 0 is_stmt 0
	vmovss	1496(%rsp), %xmm1
	movl	1500(%rsp), %ecx
	vmovss	920(%rsp), %xmm0
	movl	924(%rsp), %eax
.Ltmp11977:
	.loc	1 2072 0 is_stmt 1
	movq	216(%r12), %rdx
.Ltmp11978:
	.loc	52 56 9
	vmovss	%xmm1, (%rdx)
.Ltmp11979:
	.loc	1 2073 5
	movq	256(%r12), %rdx
.Ltmp11980:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp11981:
	.loc	6 180 28
	je	.LBB36_558
.Ltmp11982:
	.loc	6 0 28 is_stmt 0
	movq	248(%r12), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB36_557:
.Ltmp11983:
	.loc	20 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp11984:
	.loc	7 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp11985:
	.loc	6 180 28
	jne	.LBB36_557
.Ltmp11986:
.LBB36_558:
	.loc	52 56 9
	cmpq	$0, 424(%r12)
	je	.LBB36_581
.Ltmp11987:
	.loc	1 2074 0
	movq	416(%r12), %rcx
.Ltmp11988:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx)
.Ltmp11989:
	.loc	1 2075 5
	movq	456(%r12), %rcx
.Ltmp11990:
	.loc	7 1714 9
	testq	%rcx, %rcx
.Ltmp11991:
	.loc	6 180 28
	je	.LBB36_562
.Ltmp11992:
	.loc	6 0 28 is_stmt 0
	movq	448(%r12), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB36_561:
.Ltmp11993:
	.loc	20 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp11994:
	.loc	7 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp11995:
	.loc	6 180 28
	jne	.LBB36_561
.Ltmp11996:
.LBB36_562:
	.loc	1 2077 5
	vmovups	740(%rsp), %ymm0
	vmovups	772(%rsp), %ymm1
	vmovups	800(%rsp), %ymm2
	vmovups	%ymm2, 908(%rsp)
	vmovups	%ymm1, 880(%rsp)
	vmovups	%ymm0, 848(%rsp)
	leaq	848(%rsp), %rdi
	movq	952(%rsp), %rsi
	.loc	1 2077 14 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	.loc	1 2078 5 is_stmt 1
	vmovups	496(%rsp), %ymm0
	vmovups	528(%rsp), %ymm1
	vmovups	556(%rsp), %ymm2
	vmovups	%ymm2, 908(%rsp)
	vmovups	%ymm1, 880(%rsp)
	vmovups	%ymm0, 848(%rsp)
	leaq	848(%rsp), %rdi
	movq	944(%rsp), %rsi
	.loc	1 2078 15 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	632(%rsp), %rax
	.loc	1 2079 5 is_stmt 1
	movl	%eax, 560(%r12)
	movq	448(%rsp), %rax
.Ltmp11997:
	.loc	1 0 0 is_stmt 0
	movl	%eax, 564(%r12)
.Ltmp11998:
	.loc	1 2255 35 is_stmt 1
	cmpb	$0, 1532(%rsp)
	jne	.LBB36_340
.LBB36_349:
	.loc	1 0 35 is_stmt 0
	xorl	%eax, %eax
.LBB36_350:
	movq	136(%rsp), %rsi
.LBB36_351:
	movq	1216(%rsp), %rcx
	.loc	1 2255 9 is_stmt 1
	movb	%al, 780(%r12)
	.loc	1 2260 30
	movzbl	96(%r12), %eax
	.loc	1 2260 9 is_stmt 0
	movb	%al, 781(%r12)
	.loc	1 2261 21 is_stmt 1
	movq	16(%rcx), %rax
	movq	%rax, 1632(%rsp)
	vmovups	(%rcx), %xmm0
	vmovaps	%xmm0, 1616(%rsp)
	movq	%r12, %rax
.Ltmp11999:
	.loc	1 2262 20
	movl	72(%r12), %r8d
.Ltmp12000:
	.loc	1 2268 64
	movq	104(%r12), %r12
	movq	112(%rax), %r11
	movq	120(%rax), %r9
	movq	128(%rax), %r10
.Ltmp12001:
	.loc	19 2155 12
	testq	%r15, %r15
	je	.LBB36_355
.Ltmp12002:
	.loc	19 0 12 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI36_0(%rip), %xmm0
	vmovss	.LCPI36_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB36_353:
.Ltmp12003:
	.loc	52 103 24 is_stmt 1
	vmovss	(%r13,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp12004:
	.loc	52 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp12005:
	.loc	52 139 9
	cmovbel	%ecx, %eax
.Ltmp12006:
	.loc	19 2155 12
	incq	%rdx
	cmpq	%rdx, %r15
	jne	.LBB36_353
.Ltmp12007:
	.loc	52 154 9
	cmpl	$-1, %eax
.Ltmp12008:
	.loc	17 208 8
	jne	.LBB36_360
.LBB36_355:
.Ltmp12009:
	.loc	19 2155 12
	testq	%rsi, %rsi
	je	.LBB36_430
.Ltmp12010:
	.loc	19 0 12 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI36_0(%rip), %xmm0
	vmovss	.LCPI36_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB36_357:
.Ltmp12011:
	.loc	52 103 24 is_stmt 1
	vmovss	(%rbp,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp12012:
	.loc	52 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp12013:
	.loc	52 139 9
	cmovbel	%ecx, %eax
.Ltmp12014:
	.loc	19 2155 12
	incq	%rdx
	cmpq	%rdx, %rsi
	jne	.LBB36_357
.Ltmp12015:
	.loc	52 154 9
	cmpl	$-1, %eax
.Ltmp12016:
	.loc	17 208 34
	je	.LBB36_430
.Ltmp12017:
	.loc	19 2155 12
	testq	%r15, %r15
.Ltmp12018:
	.loc	19 2155 12 is_stmt 0
	je	.LBB36_423
.Ltmp12019:
.LBB36_360:
	.loc	19 0 12
	movl	$-1, %ecx
	xorl	%eax, %eax
	vbroadcastss	.LCPI36_0(%rip), %xmm0
	vmovss	.LCPI36_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB36_361:
.Ltmp12020:
	.loc	52 103 24 is_stmt 1
	vmovss	(%r13,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp12021:
	.loc	52 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp12022:
	.loc	52 139 9
	cmovbel	%eax, %ecx
.Ltmp12023:
	.loc	19 2155 12
	incq	%rdx
	cmpq	%rdx, %r15
	jne	.LBB36_361
.Ltmp12024:
	.loc	17 185 12
	notl	%ecx
	xorl	%eax, %eax
	testl	$1065353216, %ecx
	setne	%al
.Ltmp12025:
	.loc	19 2155 12
	testq	%rsi, %rsi
	jne	.LBB36_424
.Ltmp12026:
	.loc	19 0 12 is_stmt 0
	movq	%r11, 40(%rsp)
	movq	%r10, 48(%rsp)
	movl	%r8d, 16(%rsp)
	movq	%r9, 8(%rsp)
	movq	696(%rsp), %rdx
	.loc	17 211 5 is_stmt 1
	movl	%eax, 576(%rdx)
	.loc	17 212 31
	movq	568(%rdx), %rax
.Ltmp12027:
	.loc	4 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp12028:
	.loc	17 212 5
	movq	%rcx, 568(%rdx)
	jmp	.LBB36_429
.LBB36_423:
	.loc	17 0 5 is_stmt 0
	xorl	%eax, %eax
.LBB36_424:
	movl	$-1, %ecx
	xorl	%edx, %edx
	vbroadcastss	.LCPI36_0(%rip), %xmm0
	vmovss	.LCPI36_6(%rip), %xmm1
	xorl	%esi, %esi
	movq	136(%rsp), %rdi
	.p2align	4
.LBB36_425:
.Ltmp12029:
	.loc	52 103 24 is_stmt 1
	vmovss	(%rbp,%rsi,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp12030:
	.loc	52 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp12031:
	.loc	52 139 9
	cmovbel	%edx, %ecx
.Ltmp12032:
	.loc	19 2155 12
	incq	%rsi
	cmpq	%rsi, %rdi
	jne	.LBB36_425
.Ltmp12033:
	.loc	17 185 12
	notl	%ecx
	xorl	%edx, %edx
	testl	$1065353216, %ecx
	setne	%dl
.Ltmp12034:
	.loc	17 211 5
	orl	%edx, %eax
	movq	696(%rsp), %rsi
	.loc	17 212 31
	movq	568(%rsi), %rcx
.Ltmp12035:
	.loc	4 2428 13
	incq	%rcx
	movq	$-1, %rdx
	cmovneq	%rcx, %rdx
.Ltmp12036:
	.loc	17 211 5
	movl	%eax, 576(%rsi)
	.loc	17 212 5
	movq	%rdx, 568(%rsi)
.Ltmp12037:
	.loc	19 2155 12
	testq	%r15, %r15
	movq	%r9, 8(%rsp)
	movq	%r10, 48(%rsp)
	movl	%r8d, 16(%rsp)
	movq	%r11, 40(%rsp)
	je	.LBB36_428
.Ltmp12038:
	.loc	16 961 18
	shlq	$2, %r15
.Ltmp12039:
	.loc	20 25 13
	movq	%r13, %rdi
	xorl	%esi, %esi
	movq	%r15, %rdx
	callq	*memset@GOTPCREL(%rip)
.Ltmp12040:
.LBB36_428:
	.loc	20 0 13 is_stmt 0
	movq	136(%rsp), %r15
	movq	%rbp, %r13
.LBB36_429:
.Ltmp12041:
	.loc	16 961 18 is_stmt 1
	shlq	$2, %r15
.Ltmp12042:
	.loc	20 25 13
	movq	%r13, %rdi
	xorl	%esi, %esi
	movq	%r15, %rdx
	callq	*memset@GOTPCREL(%rip)
.Ltmp12043:
	.loc	1 2269 18
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %rbx
	leaq	1616(%rsp), %r14
	movq	952(%rsp), %rdi
	movq	%r14, %rsi
	movq	%r12, %rdx
	movq	40(%rsp), %rcx
	movl	16(%rsp), %ebp
	movl	%ebp, %r8d
	callq	*%rbx
	movq	944(%rsp), %rdi
	.loc	1 2270 19
	movq	%r14, %rsi
	movq	8(%rsp), %rdx
	movq	48(%rsp), %rcx
	movl	%ebp, %r8d
	callq	*%rbx
	movq	696(%rsp), %rax
	.loc	1 2271 13
	movq	$0, 560(%rax)
.Ltmp12044:
.LBB36_430:
	.loc	1 2273 6 epilogue_begin
	addq	$3672, %rsp
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
.LBB36_563:
	.cfi_def_cfa_offset 3728
	.loc	1 0 6 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12045:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12046:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12047:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
.Ltmp12048:
	.loc	1 1660 5
	vmovss	%xmm8, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12049:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12050:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12051:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_564:
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12052:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12053:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12054:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
	vmovaps	176(%rsp), %xmm0
.Ltmp12055:
	.loc	1 1660 5
	vmovss	%xmm0, 820(%rsp)
.Ltmp12056:
.LBB36_565:
	.loc	1 0 5 is_stmt 0
	vmovss	64(%rsp), %xmm0
.Ltmp12057:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12058:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12059:
	.loc	1 1131 25
	leaq	1(%rdi), %rsi
.Ltmp12060:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12061:
.LBB36_566:
	.loc	5 0 13 is_stmt 0
	vmovss	8(%rsp), %xmm0
.Ltmp12062:
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12063:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 924(%rsp)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp12064:
	vmovss	%xmm8, 568(%rsp)
.Ltmp12065:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12066:
	.loc	1 1131 25 is_stmt 1
	leaq	1(%rdi), %rsi
.Ltmp12067:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12068:
.LBB36_567:
	.loc	5 0 13 is_stmt 0
	vmovss	8(%rsp), %xmm0
.Ltmp12069:
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12070:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 924(%rsp)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp12071:
	vmovss	%xmm8, 568(%rsp)
.Ltmp12072:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12073:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12074:
.LBB36_568:
	.loc	1 455 44
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB36_569:
	.loc	1 455 22 is_stmt 0
	addq	%rdi, %rax
	.loc	1 455 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB36_571
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB36_572
.LBB36_571:
	xorl	%edx, %edx
	divl	%esi
.LBB36_572:
	.loc	1 455 9
	movl	%edx, 560(%r12)
	.loc	1 456 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB36_649
	.loc	1 456 23 is_stmt 0
	movl	564(%r12), %esi
	movq	120(%rsp), %rax
	.loc	1 456 44
	movq	%rax, %rdx
	orq	%rcx, %rdx
	shrq	$32, %rdx
	je	.LBB36_575
	xorl	%edx, %edx
	divq	%rcx
	movq	%rdx, %rax
	jmp	.LBB36_576
.LBB36_575:
	xorl	%edx, %edx
	divl	%ecx
	movl	%edx, %eax
.LBB36_576:
	.loc	1 456 22
	addq	%rsi, %rax
	.loc	1 456 21
	movq	%rax, %rdx
	orq	%rcx, %rdx
	shrq	$32, %rdx
	je	.LBB36_578
	xorl	%edx, %edx
	divq	%rcx
	jmp	.LBB36_579
.LBB36_578:
	xorl	%edx, %edx
	divl	%ecx
.LBB36_579:
	.loc	1 456 9
	movl	%edx, 564(%r12)
	jmp	.LBB36_430
.Ltmp12075:
.LBB36_580:
	.loc	1 0 0
	vmovss	%xmm5, 984(%rsp)
	vmovss	%xmm13, 988(%rsp)
.Ltmp12076:
	vmovss	%xmm10, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	%r9d, 572(%rsp)
	vmovss	836(%rsp), %xmm0
	vmovss	%xmm0, 568(%rsp)
	movl	%edi, 924(%rsp)
	vmovss	832(%rsp), %xmm0
	vmovss	%xmm0, 920(%rsp)
	movq	696(%rsp), %r12
.Ltmp12077:
	.loc	52 56 9 is_stmt 1
	cmpq	$0, 224(%r12)
	jne	.LBB36_440
.Ltmp12078:
.LBB36_581:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_582:
.Ltmp12079:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_f6bbc99b95dcf27c100d71299ef7abde(%rip), %rcx
	movq	224(%rsp), %rdi
	movq	136(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12080:
.LBB36_583:
	.loc	5 443 13
	leaq	.Lalloc_f6bbc99b95dcf27c100d71299ef7abde(%rip), %rcx
	movq	136(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12081:
.LBB36_584:
	.loc	5 0 13 is_stmt 0
	vmovss	8(%rsp), %xmm0
.Ltmp12082:
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12083:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 924(%rsp)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp12084:
	vmovss	%xmm8, 568(%rsp)
.Ltmp12085:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12086:
	.loc	11 304 12 is_stmt 1
	cmpq	%rdx, %r14
	cmovbeq	%rdx, %r14
	incq	%r14
.Ltmp12087:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r14, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12088:
.LBB36_585:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12089:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12090:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12091:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
.Ltmp12092:
	.loc	1 1660 5
	vmovss	%xmm8, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12093:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12094:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12095:
	.loc	11 304 12
	cmpq	%rdx, %rsi
	cmovbeq	%rdx, %rsi
	incq	%rsi
.Ltmp12096:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12097:
.LBB36_586:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12098:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12099:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12100:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
.Ltmp12101:
	.loc	1 1660 5
	vmovss	%xmm8, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12102:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12103:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12104:
	.loc	1 1131 25
	leaq	1(%rdi), %rsi
.Ltmp12105:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12106:
.LBB36_587:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12107:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12108:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12109:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm6, 824(%rsp)
.Ltmp12110:
	.loc	1 1660 5
	vmovss	%xmm8, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12111:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12112:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12113:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12114:
.LBB36_588:
	.loc	5 0 13 is_stmt 0
	vmovss	8(%rsp), %xmm0
.Ltmp12115:
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12116:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 924(%rsp)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp12117:
	vmovss	%xmm8, 568(%rsp)
.Ltmp12118:
	vmovss	%xmm9, 920(%rsp)
	movq	216(%rsp), %rdx
.Ltmp12119:
	.loc	11 304 12 is_stmt 1
	cmpq	%rdx, %r15
	cmovbeq	%rdx, %r15
	incq	%r15
.Ltmp12120:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdi
	movq	%r15, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12121:
.LBB36_589:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12122:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12123:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12124:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
.Ltmp12125:
	.loc	1 1660 5
	vmovss	%xmm12, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12126:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12127:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
	movq	632(%rsp), %rsi
	movq	208(%rsp), %rdx
.Ltmp12128:
	.loc	11 304 12
	cmpq	%rdx, %rsi
	cmovbeq	%rdx, %rsi
	incq	%rsi
.Ltmp12129:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12130:
.LBB36_590:
	.loc	5 0 13 is_stmt 0
	vmovss	8(%rsp), %xmm0
.Ltmp12131:
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12132:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %ecx
	movl	%ecx, 572(%rsp)
	movl	4(%rsp), %ecx
	movl	%ecx, 924(%rsp)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp12133:
	vmovss	%xmm8, 568(%rsp)
.Ltmp12134:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12135:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12136:
.LBB36_591:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12137:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12138:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12139:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
.Ltmp12140:
	.loc	1 1660 5
	vmovss	%xmm12, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12141:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12142:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12143:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12144:
.LBB36_592:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12145:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12146:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12147:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
.Ltmp12148:
	.loc	1 1660 5
	vmovss	%xmm12, 820(%rsp)
	jmp	.LBB36_565
.Ltmp12149:
.LBB36_593:
	.loc	1 0 5 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12150:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12151:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12152:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
	vmovaps	176(%rsp), %xmm0
.Ltmp12153:
	.loc	1 1660 5
	vmovss	%xmm0, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12154:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12155:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12156:
	.loc	1 1131 25
	leaq	1(%rdi), %rsi
.Ltmp12157:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rbx, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12158:
.LBB36_594:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12159:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12160:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12161:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
	vmovaps	176(%rsp), %xmm0
.Ltmp12162:
	.loc	1 1660 5
	vmovss	%xmm0, 820(%rsp)
.Ltmp12163:
	.loc	1 1654 5
	vmovss	%xmm6, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12164:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12165:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%rbx, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12166:
.LBB36_595:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12167:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12168:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12169:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
	vmovaps	176(%rsp), %xmm0
.Ltmp12170:
	.loc	1 1660 5
	vmovss	%xmm0, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12171:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
.Ltmp12172:
	.loc	1 1660 5
	vmovss	%xmm1, 576(%rsp)
.Ltmp12173:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdi
	movq	416(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12174:
.LBB36_596:
	.loc	5 0 13 is_stmt 0
	vmovss	8(%rsp), %xmm0
.Ltmp12175:
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12176:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 924(%rsp)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp12177:
	vmovss	%xmm8, 568(%rsp)
.Ltmp12178:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12179:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdi
	movq	424(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12180:
.LBB36_597:
	.loc	1 1417 5
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12181:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12182:
	.loc	1 0 0
	leaq	.Lalloc_4a8785a681d008a9bfd0cd82628ea9cb(%rip), %rdx
	movq	48(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_598:
.Ltmp12183:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp12184:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12185:
	.loc	1 0 0
	leaq	.Lalloc_4a8785a681d008a9bfd0cd82628ea9cb(%rip), %rdx
	movq	48(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_599:
.Ltmp12186:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12187:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12188:
	.loc	1 0 0
	leaq	.Lalloc_2b690e2c7763f11809942906fc2ca813(%rip), %rdx
	movq	%r10, %rdi
	movq	%r10, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_600:
.Ltmp12189:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp12190:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12191:
	.loc	1 0 0
	leaq	.Lalloc_2b690e2c7763f11809942906fc2ca813(%rip), %rdx
	movq	%r10, %rdi
	movq	%r10, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_601:
.Ltmp12192:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12193:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12194:
	.loc	1 1273 22 is_stmt 1
	leaq	.Lalloc_cbce7773ac40979e4ba2385da3aec116(%rip), %rdx
	movq	%r15, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12195:
.LBB36_602:
	.loc	1 1411 5
	vmovss	%xmm8, 824(%rsp)
.Ltmp12196:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12197:
	.loc	1 1273 22 is_stmt 1
	leaq	.Lalloc_cbce7773ac40979e4ba2385da3aec116(%rip), %rdx
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12198:
.LBB36_603:
	.loc	1 1417 5
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12199:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12200:
	.loc	1 0 0
	leaq	.Lalloc_ec0d48f73ebfc2755df5cedaa60b5c0a(%rip), %rdx
	movq	16(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_604:
.Ltmp12201:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp12202:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12203:
	.loc	1 0 0
	leaq	.Lalloc_ec0d48f73ebfc2755df5cedaa60b5c0a(%rip), %rdx
	movq	16(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_605:
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12204:
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12205:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12206:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
.Ltmp12207:
	.loc	1 1660 5
	vmovss	%xmm8, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12208:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12209:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12210:
	.loc	1 1131 25
	leaq	1(%r8), %rsi
.Ltmp12211:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r8, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12212:
.LBB36_606:
	.loc	5 0 13 is_stmt 0
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	28(%rsp), %xmm0
	vmovss	%xmm0, 1496(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12213:
	vmovss	%xmm0, 920(%rsp)
.Ltmp12214:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 824(%rsp)
	vmovaps	176(%rsp), %xmm0
.Ltmp12215:
	.loc	1 1660 5
	vmovss	%xmm0, 820(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp12216:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	592(%rsp), %xmm0
.Ltmp12217:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12218:
	.loc	1 1131 25
	leaq	1(%r8), %rsi
.Ltmp12219:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r8, %rdi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12220:
.LBB36_607:
	.loc	1 1417 5
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12221:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12222:
	.loc	1 0 0
	leaq	.Lalloc_2c461872bb652d4796cdcf89c28c82c8(%rip), %rdx
	movq	%r12, %rdi
	movq	%r10, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_608:
.Ltmp12223:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp12224:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12225:
	.loc	1 0 0
	leaq	.Lalloc_2c461872bb652d4796cdcf89c28c82c8(%rip), %rdx
	movq	%r12, %rdi
	movq	%r10, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_609:
.Ltmp12226:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12227:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12228:
	.loc	1 1285 24 is_stmt 1
	leaq	.Lalloc_9fa421ae81817f58fcfc4a3243223891(%rip), %rdx
	movq	%r15, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12229:
.LBB36_610:
	.loc	1 1411 5
	vmovss	%xmm8, 824(%rsp)
.Ltmp12230:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12231:
	.loc	1 1285 24 is_stmt 1
	leaq	.Lalloc_9fa421ae81817f58fcfc4a3243223891(%rip), %rdx
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12232:
.LBB36_611:
	.loc	1 1417 5
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12233:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12234:
	.loc	1 0 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_612:
.Ltmp12235:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp12236:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12237:
	.loc	1 0 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_613:
.Ltmp12238:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp12239:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12240:
	.loc	1 0 0
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_614:
.Ltmp12241:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12242:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12243:
	.loc	1 0 0
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_615:
	vmovss	8(%rsp), %xmm1
.Ltmp12244:
	vmovss	%xmm1, 984(%rsp)
	vmovaps	%xmm0, %xmm1
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12245:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 924(%rsp)
	vmovss	%xmm1, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp12246:
	vmovss	%xmm8, 568(%rsp)
.Ltmp12247:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12248:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_debdb702bca99cd8ca93ec127a5c320a(%rip), %rcx
	movq	%r15, %rsi
	movq	968(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12249:
.LBB36_616:
	.loc	5 0 13 is_stmt 0
	vmovss	%xmm12, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
.Ltmp12250:
	vmovss	%xmm10, 1496(%rsp)
.Ltmp12251:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12252:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_debdb702bca99cd8ca93ec127a5c320a(%rip), %rcx
	movq	968(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12253:
.LBB36_617:
	.loc	1 1417 5
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12254:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12255:
	.loc	1 0 0
	leaq	.Lalloc_b305c1483509cfb31fdec21ff8752674(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_618:
.Ltmp12256:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp12257:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12258:
	.loc	1 0 0
	leaq	.Lalloc_b305c1483509cfb31fdec21ff8752674(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_619:
.Ltmp12259:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_e7f134ea71d3d762bf72c0ef5353d5ff(%rip), %rcx
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12260:
.LBB36_620:
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm5, 984(%rsp)
	vmovss	%xmm13, 988(%rsp)
.Ltmp12261:
	vmovss	%xmm10, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	%r9d, 572(%rsp)
	vmovss	836(%rsp), %xmm0
	vmovss	%xmm0, 568(%rsp)
	movl	%edi, 924(%rsp)
	vmovss	832(%rsp), %xmm0
	vmovss	%xmm0, 920(%rsp)
.Ltmp12262:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_e7f134ea71d3d762bf72c0ef5353d5ff(%rip), %rcx
	movq	616(%rsp), %rdi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12263:
.LBB36_621:
	.loc	5 0 13 is_stmt 0
	movq	%rax, %rsi
.Ltmp12264:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_5792a3affd2091ba045d8adc49b05663(%rip), %rcx
.Ltmp12265:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12266:
.LBB36_622:
	.loc	5 0 13
	vmovss	8(%rsp), %xmm1
.Ltmp12267:
	vmovss	%xmm1, 984(%rsp)
	vmovaps	%xmm0, %xmm1
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12268:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	%xmm14, 1080(%rsp)
	movl	80(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 924(%rsp)
	vmovss	%xmm1, 1068(%rsp)
	vmovss	%xmm15, 1064(%rsp)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 1160(%rsp)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 1156(%rsp)
.Ltmp12269:
	vmovss	%xmm8, 568(%rsp)
.Ltmp12270:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12271:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_01912ef845d3ed33a157086defa4d008(%rip), %rcx
	movq	%r15, %rsi
	movq	136(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12272:
.LBB36_623:
	.loc	5 0 13 is_stmt 0
	vmovss	%xmm12, 800(%rsp)
	vmovss	4(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
.Ltmp12273:
	vmovss	%xmm10, 1496(%rsp)
.Ltmp12274:
	vmovss	%xmm9, 920(%rsp)
.Ltmp12275:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_01912ef845d3ed33a157086defa4d008(%rip), %rcx
	movq	136(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12276:
.LBB36_624:
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm0, 984(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12277:
	vmovss	%xmm0, 1076(%rsp)
	vmovss	1236(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	movl	%r9d, 572(%rsp)
	vmovss	836(%rsp), %xmm0
	vmovss	%xmm0, 568(%rsp)
	movl	%edi, 924(%rsp)
	vmovss	832(%rsp), %xmm0
	vmovss	%xmm0, 920(%rsp)
.Ltmp12278:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_480b8302a9d65cc7541e43747df38ed7(%rip), %rcx
	movq	616(%rsp), %rdi
	movq	136(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12279:
.LBB36_625:
	.loc	5 443 13
	leaq	.Lalloc_480b8302a9d65cc7541e43747df38ed7(%rip), %rcx
	movq	136(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12280:
.LBB36_626:
	.loc	5 443 13
	leaq	.Lalloc_34e4746889305f9c657ea46721b345d0(%rip), %rcx
.Ltmp12281:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12282:
.LBB36_627:
	.loc	5 0 13
	movl	$1, %eax
	jmp	.LBB36_642
.LBB36_628:
	movl	$1, %eax
	jmp	.LBB36_645
.LBB36_629:
.Ltmp12283:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_e8e5016f8f28771294ff88b89dfac103(%rip), %rcx
	xorl	%edi, %edi
	movq	120(%rsp), %rsi
	movq	136(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12284:
.LBB36_630:
	.loc	5 0 13 is_stmt 0
	movl	$2, %eax
	jmp	.LBB36_642
.LBB36_631:
	movl	$2, %eax
	jmp	.LBB36_645
.LBB36_632:
.Ltmp12285:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_22a7c5212c1e2b1f20d68ba89a9d6a79(%rip), %rcx
	xorl	%edi, %edi
	movq	136(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12286:
.LBB36_633:
	.loc	5 0 13 is_stmt 0
	movl	$3, %eax
	jmp	.LBB36_642
.LBB36_634:
	movl	$3, %eax
	jmp	.LBB36_645
.LBB36_635:
	movl	$4, %eax
	jmp	.LBB36_642
.LBB36_636:
	movl	$4, %eax
	jmp	.LBB36_645
.LBB36_637:
	movl	$5, %eax
	jmp	.LBB36_642
.LBB36_638:
	movl	$5, %eax
	jmp	.LBB36_645
.LBB36_639:
	movl	$6, %eax
	jmp	.LBB36_642
.LBB36_640:
	movl	$6, %eax
	jmp	.LBB36_645
.LBB36_641:
	movl	$7, %eax
.LBB36_642:
	movq	%rax, 128(%rsp)
.LBB36_643:
.Ltmp12287:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1320(%rsp)
.Ltmp12288:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1412(%rsp)
.Ltmp12289:
	.loc	1 1403 42 is_stmt 1
	leaq	.Lalloc_a6feb40b34112df5f214f84424dd2c7c(%rip), %rdx
	movq	128(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12290:
.LBB36_644:
	.loc	1 0 42 is_stmt 0
	movl	$7, %eax
.LBB36_645:
	movq	%rax, 144(%rsp)
.LBB36_646:
.Ltmp12291:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 824(%rsp)
.Ltmp12292:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12293:
	.loc	1 1403 42 is_stmt 1
	leaq	.Lalloc_a6feb40b34112df5f214f84424dd2c7c(%rip), %rdx
	movq	144(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12294:
.LBB36_647:
	.loc	1 665 42
	leaq	.Lalloc_bac57976a2bdbfad4a3a85d5d1c7648c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp12295:
.LBB36_648:
	.loc	1 455 44
	leaq	.Lalloc_f0ee36f67d9a332211aa5518dd2ebfd5(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB36_649:
	.loc	1 456 44
	leaq	.Lalloc_33d4d33e0a850133578789055882dcf9(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp12296:
.Lfunc_end36:
	.size	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_, .Lfunc_end36-_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_
