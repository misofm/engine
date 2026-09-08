_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_:
.Lfunc_begin36:
	.loc	1 2189 0 is_stmt 1
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
	movq	%r9, 232(%rsp)
	movq	%rcx, %rbp
	movq	%rdx, %r14
	movq	%rsi, %r11
	movq	%rdi, %r13
.Ltmp9885:
	.loc	1 2208 21 prologue_end
	movzbl	781(%rdi), %edx
	.loc	1 0 0 is_stmt 0
	movq	264(%rdi), %rcx
	movq	272(%rdi), %rax
	.loc	1 2208 21
	cmpb	96(%rdi), %dl
	movq	%r8, 296(%rsp)
	jne	.LBB36_31
.Ltmp9886:
	.loc	6 314 17 is_stmt 1
	movq	%rax, %rdx
	shlq	$4, %rdx
	movq	%rcx, %rsi
	.loc	6 0 17 is_stmt 0
.Ltmp9887:
	.p2align	4
.LBB36_2:
.Ltmp9888:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9889:
	.loc	6 180 28
	je	.LBB36_5
.Ltmp9890:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9891:
	.loc	6 315 25
	jne	.LBB36_31
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB36_2
	jmp	.LBB36_31
.Ltmp9892:
.LBB36_5:
	.loc	1 2210 37 is_stmt 1
	movq	280(%r13), %rsi
	movq	288(%r13), %rdx
.Ltmp9893:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp9894:
	.p2align	4
.LBB36_6:
.Ltmp9895:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9896:
	.loc	6 180 28
	je	.LBB36_9
.Ltmp9897:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9898:
	.loc	6 315 25
	jne	.LBB36_31
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB36_6
	jmp	.LBB36_31
.Ltmp9899:
.LBB36_9:
	.loc	1 2211 37 is_stmt 1
	movq	464(%r13), %rsi
	movq	472(%r13), %rdx
.Ltmp9900:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp9901:
	.p2align	4
.LBB36_10:
.Ltmp9902:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9903:
	.loc	6 180 28
	je	.LBB36_13
.Ltmp9904:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9905:
	.loc	6 315 25
	jne	.LBB36_31
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB36_10
	jmp	.LBB36_31
.Ltmp9906:
.LBB36_13:
	.loc	1 2212 37 is_stmt 1
	movq	480(%r13), %rsi
	movq	488(%r13), %rdx
.Ltmp9907:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp9908:
	.p2align	4
.LBB36_14:
.Ltmp9909:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9910:
	.loc	6 180 28
	je	.LBB36_17
.Ltmp9911:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9912:
	.loc	6 315 25
	jne	.LBB36_31
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB36_14
	jmp	.LBB36_31
.Ltmp9913:
.LBB36_17:
	.loc	6 0 25
	movq	232(%rsp), %rsi
	cmpq	%r14, %rsi
.Ltmp9914:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB36_23
.Ltmp9915:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	%r11, %rsi
	.p2align	4
.LBB36_19:
.Ltmp9916:
	.loc	15 1504 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB36_25
.Ltmp9917:
	.loc	10 1078 5
	cmpq	$32, %rdx
	movl	$32, %edi
	cmovbq	%rdx, %rdi
.Ltmp9918:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp9919:
	.loc	16 0 18 is_stmt 0
.Ltmp9920:
	.p2align	4
.LBB36_21:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp9921:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp9922:
	.loc	6 180 28
	jne	.LBB36_21
.Ltmp9923:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp9924:
	.loc	19 2054 74
	subq	%rdi, %rdx
.Ltmp9925:
	.loc	17 136 12
	testl	%r8d, %r8d
	movq	296(%rsp), %r8
	je	.LBB36_19
	jmp	.LBB36_31
.Ltmp9926:
.LBB36_25:
	.loc	17 0 12 is_stmt 0
	movq	232(%rsp), %rsi
.Ltmp9927:
	.loc	5 438 16 is_stmt 1
	cmpq	%r8, %rsi
	ja	.LBB36_661
.Ltmp9928:
	.loc	5 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	%rbp, %rsi
	.p2align	4
.LBB36_27:
.Ltmp9929:
	.loc	15 1504 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB36_600
.Ltmp9930:
	.loc	10 1078 5
	cmpq	$32, %rdx
	movl	$32, %edi
	cmovbq	%rdx, %rdi
.Ltmp9931:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp9932:
	.loc	16 0 18 is_stmt 0
.Ltmp9933:
	.p2align	4
.LBB36_29:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp9934:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp9935:
	.loc	6 180 28
	jne	.LBB36_29
.Ltmp9936:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp9937:
	.loc	19 2054 74
	subq	%rdi, %rdx
.Ltmp9938:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB36_27
.Ltmp9939:
.LBB36_31:
	.loc	17 0 12 is_stmt 0
	xorl	%r9d, %r9d
.LBB36_32:
.Ltmp9940:
	.loc	1 2244 13 is_stmt 1
	leaq	536(%r13), %rdx
	movq	%rdx, 1200(%rsp)
	.loc	1 2245 13
	leaq	136(%r13), %rdx
	movq	%rdx, 1192(%rsp)
	.loc	1 2246 13
	leaq	336(%r13), %rbx
.Ltmp9941:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp9942:
	.p2align	4
.LBB36_33:
.Ltmp9943:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp9944:
	.loc	6 180 28
	je	.LBB36_36
.Ltmp9945:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp9946:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp9947:
	.loc	6 315 25
	jne	.LBB36_49
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB36_33
	jmp	.LBB36_49
.Ltmp9948:
.LBB36_36:
	.loc	1 708 33 is_stmt 1
	movq	280(%r13), %rcx
	movq	288(%r13), %rax
.Ltmp9949:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp9950:
	.p2align	4
.LBB36_37:
.Ltmp9951:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp9952:
	.loc	6 180 28
	je	.LBB36_40
.Ltmp9953:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp9954:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp9955:
	.loc	6 315 25
	jne	.LBB36_49
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB36_37
	jmp	.LBB36_49
.Ltmp9956:
.LBB36_40:
	.loc	1 709 33 is_stmt 1
	movq	464(%r13), %rcx
	movq	472(%r13), %rax
.Ltmp9957:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp9958:
	.p2align	4
.LBB36_41:
.Ltmp9959:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp9960:
	.loc	6 180 28
	je	.LBB36_44
.Ltmp9961:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp9962:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp9963:
	.loc	6 315 25
	jne	.LBB36_49
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB36_41
	jmp	.LBB36_49
.Ltmp9964:
.LBB36_44:
	.loc	1 710 33 is_stmt 1
	movq	480(%r13), %rcx
	movq	488(%r13), %rax
.Ltmp9965:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp9966:
	.p2align	4
.LBB36_45:
.Ltmp9967:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp9968:
	.loc	6 180 28
	je	.LBB36_46
.Ltmp9969:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp9970:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp9971:
	.loc	6 315 25
	jne	.LBB36_49
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB36_45
	jmp	.LBB36_49
.Ltmp9972:
.LBB36_46:
	.loc	6 0 25
	movb	$1, %dl
.LBB36_49:
.Ltmp9973:
	.loc	1 1116 5 is_stmt 1
	movq	320(%r13), %rcx
	testq	%rcx, %rcx
	movq	%r13, 936(%rsp)
	movq	%r14, 960(%rsp)
	movq	%r11, 680(%rsp)
	movq	%rbp, 928(%rsp)
	movq	%rbx, 1184(%rsp)
	movl	%r9d, 1532(%rsp)
	je	.LBB36_55
	.loc	1 0 5 is_stmt 0
	movq	312(%r13), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rsi
	.p2align	4
.LBB36_51:
.Ltmp9974:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp9975:
	.loc	6 180 28
	je	.LBB36_55
.Ltmp9976:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rax), %edi
	jne	.LBB36_69
	movl	4(%rsi), %edi
	cmpl	4(%rax), %edi
	jne	.LBB36_69
	movl	8(%rsi), %edi
.Ltmp9977:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp9978:
	.loc	1 390 34
	cmpl	8(%rax), %edi
.Ltmp9979:
	.loc	6 315 25
	je	.LBB36_51
	jmp	.LBB36_69
.Ltmp9980:
.LBB36_55:
	.loc	1 1117 12
	movq	256(%r13), %rax
	testq	%rax, %rax
	je	.LBB36_59
	.loc	1 0 12 is_stmt 0
	movq	248(%r13), %rcx
	shlq	$2, %rax
	xorl	%esi, %esi
	.p2align	4
.LBB36_57:
.Ltmp9981:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rax
.Ltmp9982:
	.loc	6 180 28
	je	.LBB36_59
.Ltmp9983:
	.loc	6 315 25
	movl	(%rcx,%rsi), %edi
	addq	$4, %rsi
.Ltmp9984:
	.loc	1 1117 43
	cmpl	(%rcx), %edi
.Ltmp9985:
	.loc	6 315 25
	je	.LBB36_57
	jmp	.LBB36_69
.Ltmp9986:
.LBB36_59:
	.loc	1 1116 5
	movq	520(%r13), %rcx
	testq	%rcx, %rcx
	je	.LBB36_65
	.loc	1 0 5 is_stmt 0
	movq	512(%r13), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rsi
	.p2align	4
.LBB36_61:
.Ltmp9987:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp9988:
	.loc	6 180 28
	je	.LBB36_65
.Ltmp9989:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rax), %edi
	jne	.LBB36_69
	movl	4(%rsi), %edi
	cmpl	4(%rax), %edi
	jne	.LBB36_69
	movl	8(%rsi), %edi
.Ltmp9990:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp9991:
	.loc	1 390 34
	cmpl	8(%rax), %edi
.Ltmp9992:
	.loc	6 315 25
	je	.LBB36_61
	jmp	.LBB36_69
.Ltmp9993:
.LBB36_65:
	.loc	1 1117 12
	movq	456(%r13), %rax
	testq	%rax, %rax
	je	.LBB36_414
	.loc	1 0 12 is_stmt 0
	movq	448(%r13), %rcx
	shlq	$2, %rax
	xorl	%esi, %esi
	.p2align	4
.LBB36_67:
.Ltmp9994:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rax
.Ltmp9995:
	.loc	6 180 28
	je	.LBB36_414
.Ltmp9996:
	.loc	6 315 25
	movl	(%rcx,%rsi), %edi
	addq	$4, %rsi
.Ltmp9997:
	.loc	1 1117 43
	cmpl	(%rcx), %edi
.Ltmp9998:
	.loc	6 315 25
	je	.LBB36_67
.Ltmp9999:
.LBB36_69:
	.loc	1 1706 12
	testb	%dl, %dl
	je	.LBB36_70
	.loc	1 0 12 is_stmt 0
	leaq	1224(%rsp), %rdi
.Ltmp10000:
	.loc	1 1795 24 is_stmt 1
	leaq	136(%r13), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	1316(%rsp), %rdi
.Ltmp10001:
	.loc	1 1796 25
	movq	%rbx, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp10002:
	.loc	1 1801 19
	movzbl	776(%r13), %r12d
.Ltmp10003:
	.loc	1 1802 21
	movzbl	777(%r13), %ebx
.Ltmp10004:
	.loc	1 1803 27
	movl	560(%r13), %eax
	movq	%rax, 800(%rsp)
.Ltmp10005:
	.loc	1 1804 27
	movl	564(%r13), %eax
	movq	%rax, (%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%ymm0, 832(%rsp)
	leaq	2648(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%r15
	leaq	1616(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
	movq	232(%rsp), %rcx
.Ltmp10006:
	.loc	8 446 20
	testq	%rcx, %rcx
	je	.LBB36_256
.Ltmp10007:
	.loc	1 0 0 is_stmt 0
	movl	%r12d, %eax
	negl	%eax
	movl	%eax, 824(%rsp)
.Ltmp10008:
	movl	%ebx, %eax
	negl	%eax
	movl	%eax, 360(%rsp)
.Ltmp10009:
	.loc	4 3756 21 is_stmt 1
	movq	%rcx, %rdx
	shrq	$5, %rdx
.Ltmp10010:
	.loc	4 3757 21
	movl	%ecx, %eax
	andl	$31, %eax
.Ltmp10011:
	.loc	4 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %rdx
	decl	%r12d
	decl	%ebx
	movl	%ebx, 288(%rsp)
.Ltmp10012:
	.loc	8 446 20
	movq	%r14, %rax
	negq	%rax
	movq	%rax, 952(%rsp)
	movq	296(%rsp), %r10
	movq	%r10, %rax
	negq	%rax
	movq	%rax, 1488(%rsp)
	movq	$0, 40(%rsp)
	movq	%rbp, 696(%rsp)
	movq	680(%rsp), %rbx
	movq	%rbx, 424(%rsp)
	xorl	%r8d, %r8d
	movl	%r12d, 84(%rsp)
	jmp	.LBB36_262
.Ltmp10013:
	.loc	8 0 20 is_stmt 0
.Ltmp10014:
	.p2align	4
.LBB36_260:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp10015:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp10016:
	.loc	1 1411 0 is_stmt 1
	vmovss	%xmm12, 1308(%rsp)
.Ltmp10017:
	.loc	1 1411 0 is_stmt 0
	vmovss	%xmm14, 1400(%rsp)
.Ltmp10018:
.LBB36_261:
	.loc	1 0 0
	movq	112(%rsp), %rdx
	addq	$32, %r8
	decq	%rdx
	movq	184(%rsp), %rcx
.Ltmp10019:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	subq	$-128, 424(%rsp)
	subq	$-128, 696(%rsp)
	testq	%rdx, %rdx
	je	.LBB36_257
.LBB36_262:
	.loc	8 0 20 is_stmt 0
	movq	%rdx, 112(%rsp)
.Ltmp10020:
	.loc	4 2584 13 is_stmt 1
	cmpq	$1, %rcx
	movq	%rcx, 184(%rsp)
	movq	%rcx, %rsi
	adcq	$0, %rsi
	cmpq	$32, %rsi
	movl	$32, %eax
	cmovaeq	%rax, %rsi
.Ltmp10021:
	.loc	1 1759 23
	vmovss	1224(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	1228(%rsp), %xmm1
	vmovss	1232(%rsp), %xmm3
	vmovss	1236(%rsp), %xmm12
	vmovss	1240(%rsp), %xmm10
	vmovss	1244(%rsp), %xmm13
	vmovss	1248(%rsp), %xmm7
	vmovss	1252(%rsp), %xmm15
	vmovss	1256(%rsp), %xmm5
	vmovss	1260(%rsp), %xmm14
	vmovss	1264(%rsp), %xmm11
.Ltmp10022:
	.loc	10 1916 50
	cmpq	%r8, 232(%rsp)
.Ltmp10023:
	.loc	3 900 12
	jne	.LBB36_264
.Ltmp10024:
	.loc	1 0 0 is_stmt 0
	vmovss	1268(%rsp), %xmm8
.Ltmp10025:
	.loc	3 900 12
	jmp	.LBB36_268
.Ltmp10026:
	.loc	3 0 12
.Ltmp10027:
	.p2align	4
.LBB36_264:
	vmovss	584(%r13), %xmm0
	vmovss	%xmm0, 672(%rsp)
	vmovss	588(%r13), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	592(%r13), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	596(%r13), %xmm0
	vmovss	%xmm0, 192(%rsp)
	vmovss	600(%r13), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	604(%r13), %xmm0
	vmovss	%xmm0, 248(%rsp)
	movq	952(%rsp), %rax
.Ltmp10028:
	.loc	5 568 12 is_stmt 1
	leaq	(%rax,%r8), %rcx
	xorl	%edx, %edx
	vmovss	%xmm11, 144(%rsp)
	vmovss	%xmm14, 624(%rsp)
	vmovss	%xmm5, 608(%rsp)
	vmovss	%xmm15, 160(%rsp)
	vmovss	%xmm7, 48(%rsp)
	vmovaps	%xmm13, %xmm4
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm12, %xmm2
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm1, %xmm6
	vmovss	608(%r13), %xmm1
	vmovss	%xmm1, 600(%rsp)
	vmovss	612(%r13), %xmm1
	vmovss	%xmm1, 592(%rsp)
	vmovss	616(%r13), %xmm1
	vmovss	%xmm1, 240(%rsp)
	vmovss	620(%r13), %xmm1
	vmovss	%xmm1, 352(%rsp)
	vmovss	624(%r13), %xmm1
	vmovss	%xmm1, 344(%rsp)
	vmovss	628(%r13), %xmm1
	vmovss	%xmm1, 336(%rsp)
	vmovss	632(%r13), %xmm1
	vmovss	%xmm1, 280(%rsp)
	vmovss	636(%r13), %xmm1
	vmovss	%xmm1, 328(%rsp)
	vmovss	640(%r13), %xmm1
	vmovss	%xmm1, 320(%rsp)
	vmovss	644(%r13), %xmm1
	vmovss	%xmm1, 272(%rsp)
	vmovss	648(%r13), %xmm1
	vmovss	%xmm1, 488(%rsp)
	vmovss	652(%r13), %xmm1
	vmovss	%xmm1, 480(%rsp)
	vmovss	656(%r13), %xmm1
	vmovss	%xmm1, 472(%rsp)
	vmovss	660(%r13), %xmm1
	vmovss	%xmm1, 408(%rsp)
	vmovss	664(%r13), %xmm1
	vmovss	%xmm1, 312(%rsp)
	vmovss	668(%r13), %xmm1
	vmovss	%xmm1, 464(%rsp)
	vmovss	672(%r13), %xmm1
	vmovss	%xmm1, 400(%rsp)
	vmovss	676(%r13), %xmm1
	vmovss	%xmm1, 68(%rsp)
	vmovss	680(%r13), %xmm1
	vmovss	%xmm1, 456(%rsp)
	vmovss	684(%r13), %xmm1
	vmovss	%xmm1, 304(%rsp)
	vmovss	688(%r13), %xmm1
	vmovss	%xmm1, 392(%rsp)
	vmovss	692(%r13), %xmm1
	vmovss	%xmm1, 16(%rsp)
	vmovss	696(%r13), %xmm1
	vmovss	%xmm1, 12(%rsp)
	vmovss	700(%r13), %xmm1
	vmovss	%xmm1, 264(%rsp)
	vmovss	704(%r13), %xmm1
	vmovss	%xmm1, 384(%rsp)
	vmovss	708(%r13), %xmm1
	vmovss	%xmm1, 72(%rsp)
	vmovss	712(%r13), %xmm1
	vmovss	%xmm1, 448(%rsp)
	vmovss	716(%r13), %xmm1
	vmovss	%xmm1, 440(%rsp)
	vmovss	720(%r13), %xmm1
	vmovss	%xmm1, 96(%rsp)
	vmovss	724(%r13), %xmm1
	vmovss	%xmm1, 88(%rsp)
	vmovss	728(%r13), %xmm1
	vmovss	%xmm1, 28(%rsp)
	vmovss	732(%r13), %xmm1
	vmovss	%xmm1, 24(%rsp)
	vmovss	736(%r13), %xmm1
	vmovss	%xmm1, 20(%rsp)
	vmovss	740(%r13), %xmm1
	vmovss	%xmm1, 664(%rsp)
	vmovss	744(%r13), %xmm1
	vmovss	%xmm1, 656(%rsp)
	vmovss	748(%r13), %xmm1
	vmovss	%xmm1, 140(%rsp)
	vmovss	752(%r13), %xmm1
	vmovss	%xmm1, 136(%rsp)
	vmovss	756(%r13), %xmm1
	vmovss	%xmm1, 132(%rsp)
	vmovss	760(%r13), %xmm1
	vmovss	%xmm1, 128(%rsp)
	vmovss	764(%r13), %xmm1
	vmovss	%xmm1, 124(%rsp)
	vmovss	768(%r13), %xmm1
	vmovss	%xmm1, 432(%rsp)
	vmovss	772(%r13), %xmm1
	vmovss	%xmm1, 648(%rsp)
	.loc	5 0 12 is_stmt 0
.Ltmp10029:
	.p2align	4
.LBB36_265:
	vmovss	56(%rsp), %xmm10
	vmovss	48(%rsp), %xmm15
	vmovss	160(%rsp), %xmm11
	vmovss	608(%rsp), %xmm14
	vmovss	624(%rsp), %xmm3
	vmovss	144(%rsp), %xmm5
	.loc	5 568 12 is_stmt 1
	leaq	(%r8,%rdx), %rax
	cmpq	%r14, %rax
	ja	.LBB36_664
.Ltmp10030:
	.loc	5 0 12 is_stmt 0
	vmovss	%xmm5, 32(%rsp)
	vmovss	%xmm3, 368(%rsp)
	.loc	1 1762 0 is_stmt 1
	movq	%rcx, %rax
	addq	%rdx, %rax
.Ltmp10031:
	.loc	52 51 9
	je	.LBB36_81
	.loc	52 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm8
	vmovaps	%xmm4, %xmm7
	vmovaps	%xmm2, %xmm13
	vmovaps	%xmm9, %xmm1
	vmovaps	%xmm6, %xmm3
	movq	424(%rsp), %rax
	.loc	52 51 9
	vmovss	(%rax,%rdx,4), %xmm6
	vbroadcastss	.LCPI36_0(%rip), %xmm12
.Ltmp10032:
	.loc	52 103 24 is_stmt 1
	vandps	%xmm4, %xmm12, %xmm0
	vmovaps	%xmm0, 144(%rsp)
.Ltmp10033:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm6, %xmm0
	vxorps	%xmm9, %xmm9, %xmm9
.Ltmp10034:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10035:
	.loc	52 71 9
	vmulss	256(%rsp), %xmm6, %xmm4
.Ltmp10036:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10037:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm6, %xmm5
.Ltmp10038:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
	vmovss	%xmm6, 56(%rsp)
.Ltmp10039:
	.loc	52 71 9
	vmulss	192(%rsp), %xmm6, %xmm6
.Ltmp10040:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovss	%xmm10, 208(%rsp)
.Ltmp10041:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm10, %xmm9
.Ltmp10042:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10043:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm10, %xmm9
.Ltmp10044:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10045:
	.loc	52 71 9
	vmulss	600(%rsp), %xmm10, %xmm9
.Ltmp10046:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10047:
	.loc	52 71 9
	vmulss	592(%rsp), %xmm10, %xmm9
.Ltmp10048:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10049:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm3, %xmm9
.Ltmp10050:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10051:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm3, %xmm9
.Ltmp10052:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10053:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm3, %xmm9
.Ltmp10054:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10055:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm3, %xmm9
.Ltmp10056:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10057:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm1, %xmm9
.Ltmp10058:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10059:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm1, %xmm9
.Ltmp10060:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10061:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm1, %xmm9
.Ltmp10062:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10063:
	.loc	52 71 9
	vmulss	272(%rsp), %xmm1, %xmm9
.Ltmp10064:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm2, %xmm10
.Ltmp10065:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm13, %xmm9
.Ltmp10066:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10067:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm13, %xmm9
.Ltmp10068:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10069:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm13, %xmm9
.Ltmp10070:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10071:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm13, %xmm9
.Ltmp10072:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10073:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm8, %xmm9
.Ltmp10074:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10075:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm8, %xmm9
.Ltmp10076:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10077:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm8, %xmm9
.Ltmp10078:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10079:
	.loc	52 71 9
	vmulss	68(%rsp), %xmm8, %xmm9
.Ltmp10080:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10081:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm7, %xmm9
.Ltmp10082:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10083:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm7, %xmm9
.Ltmp10084:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10085:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm7, %xmm9
.Ltmp10086:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10087:
	.loc	52 71 9
	vmulss	16(%rsp), %xmm7, %xmm9
.Ltmp10088:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10089:
	.loc	52 71 9
	vmulss	12(%rsp), %xmm15, %xmm9
.Ltmp10090:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10091:
	.loc	52 71 9
	vmulss	264(%rsp), %xmm15, %xmm9
.Ltmp10092:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10093:
	.loc	52 71 9
	vmulss	384(%rsp), %xmm15, %xmm9
.Ltmp10094:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm2
.Ltmp10095:
	.loc	52 71 9
	vmulss	72(%rsp), %xmm15, %xmm9
.Ltmp10096:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm11, %xmm5
.Ltmp10097:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm11, %xmm9
.Ltmp10098:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10099:
	.loc	52 71 9
	vmulss	440(%rsp), %xmm11, %xmm9
.Ltmp10100:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10101:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm11, %xmm9
.Ltmp10102:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10103:
	.loc	52 71 9
	vmulss	88(%rsp), %xmm11, %xmm9
.Ltmp10104:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10105:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm14, %xmm9
.Ltmp10106:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10107:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm14, %xmm9
.Ltmp10108:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10109:
	.loc	52 71 9
	vmulss	20(%rsp), %xmm14, %xmm9
.Ltmp10110:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10111:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm14, %xmm9
.Ltmp10112:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovss	368(%rsp), %xmm11
.Ltmp10113:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm11, %xmm9
.Ltmp10114:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10115:
	.loc	52 71 9
	vmulss	140(%rsp), %xmm11, %xmm9
.Ltmp10116:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10117:
	.loc	52 71 9
	vmulss	136(%rsp), %xmm11, %xmm9
.Ltmp10118:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10119:
	.loc	52 71 9
	vmulss	132(%rsp), %xmm11, %xmm9
.Ltmp10120:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm8, %xmm13
	vmovss	32(%rsp), %xmm8
.Ltmp10121:
	.loc	52 71 9
	vmulss	128(%rsp), %xmm8, %xmm9
.Ltmp10122:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10123:
	.loc	52 71 9
	vmulss	124(%rsp), %xmm8, %xmm9
.Ltmp10124:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10125:
	.loc	52 71 9
	vmulss	432(%rsp), %xmm8, %xmm9
.Ltmp10126:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10127:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm8, %xmm9
.Ltmp10128:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm12, %xmm9
.Ltmp10129:
	.loc	52 103 24
	vandps	%xmm0, %xmm12, %xmm0
	vmovaps	144(%rsp), %xmm12
.Ltmp10130:
	.loc	52 161 24
	vmaxss	%xmm0, %xmm12, %xmm0
	vmovaps	%xmm1, %xmm12
	vmovss	208(%rsp), %xmm1
.Ltmp10131:
	.loc	52 103 24
	vandps	%xmm4, %xmm9, %xmm4
.Ltmp10132:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp10133:
	.loc	52 103 24
	vandps	%xmm2, %xmm9, %xmm4
.Ltmp10134:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp10135:
	.loc	52 103 24
	vandps	%xmm6, %xmm9, %xmm4
.Ltmp10136:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp10137:
	.loc	52 56 9
	vmovss	%xmm0, 2648(%rsp,%rdx,4)
.Ltmp10138:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm11, 144(%rsp)
	vmovss	%xmm14, 624(%rsp)
	vmovss	%xmm5, 608(%rsp)
	vmovss	%xmm15, 160(%rsp)
	vmovss	%xmm7, 48(%rsp)
	vmovaps	%xmm13, %xmm4
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm12, %xmm2
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm1, %xmm6
.Ltmp10139:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %rsi
.Ltmp10140:
	.loc	3 900 12
	jne	.LBB36_265
.Ltmp10141:
.LBB36_268:
	.loc	3 0 12 is_stmt 0
	vmovss	56(%rsp), %xmm0
	.loc	1 1765 5 is_stmt 1
	vmovss	%xmm0, 1224(%rsp)
	vmovss	%xmm1, 1228(%rsp)
	vmovss	%xmm3, 1232(%rsp)
	vmovss	%xmm12, 1236(%rsp)
	vmovss	%xmm10, 1240(%rsp)
	vmovss	%xmm13, 1244(%rsp)
	vmovss	%xmm7, 1248(%rsp)
	vmovss	%xmm15, 1252(%rsp)
	vmovss	%xmm5, 1256(%rsp)
	vmovss	%xmm14, 1260(%rsp)
	vmovss	%xmm11, 1264(%rsp)
	vmovss	%xmm8, 1268(%rsp)
.Ltmp10142:
	.loc	1 1759 23
	vmovss	1316(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	1320(%rsp), %xmm1
	vmovss	1324(%rsp), %xmm3
	vmovss	1328(%rsp), %xmm12
	vmovss	1332(%rsp), %xmm10
	vmovss	1336(%rsp), %xmm13
	vmovss	1340(%rsp), %xmm7
	vmovss	1344(%rsp), %xmm15
	vmovss	1348(%rsp), %xmm5
	vmovss	1352(%rsp), %xmm14
	vmovss	1356(%rsp), %xmm11
.Ltmp10143:
	.loc	10 1916 50
	cmpq	%r8, 232(%rsp)
.Ltmp10144:
	.loc	3 900 12
	jne	.LBB36_270
.Ltmp10145:
	.loc	1 0 0 is_stmt 0
	vmovss	1360(%rsp), %xmm8
.Ltmp10146:
	.loc	3 900 12
	jmp	.LBB36_274
.Ltmp10147:
	.loc	3 0 12
.Ltmp10148:
	.p2align	4
.LBB36_270:
	vmovss	584(%r13), %xmm0
	vmovss	%xmm0, 672(%rsp)
	vmovss	588(%r13), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	592(%r13), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	596(%r13), %xmm0
	vmovss	%xmm0, 192(%rsp)
	vmovss	600(%r13), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	604(%r13), %xmm0
	vmovss	%xmm0, 248(%rsp)
	movq	1488(%rsp), %rax
.Ltmp10149:
	.loc	5 568 12 is_stmt 1
	leaq	(%rax,%r8), %rcx
	xorl	%edx, %edx
	vmovss	%xmm11, 144(%rsp)
	vmovss	%xmm14, 624(%rsp)
	vmovss	%xmm5, 608(%rsp)
	vmovss	%xmm15, 160(%rsp)
	vmovss	%xmm7, 48(%rsp)
	vmovaps	%xmm13, %xmm4
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm12, %xmm2
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm1, %xmm6
	vmovss	608(%r13), %xmm1
	vmovss	%xmm1, 600(%rsp)
	vmovss	612(%r13), %xmm1
	vmovss	%xmm1, 592(%rsp)
	vmovss	616(%r13), %xmm1
	vmovss	%xmm1, 240(%rsp)
	vmovss	620(%r13), %xmm1
	vmovss	%xmm1, 352(%rsp)
	vmovss	624(%r13), %xmm1
	vmovss	%xmm1, 344(%rsp)
	vmovss	628(%r13), %xmm1
	vmovss	%xmm1, 336(%rsp)
	vmovss	632(%r13), %xmm1
	vmovss	%xmm1, 280(%rsp)
	vmovss	636(%r13), %xmm1
	vmovss	%xmm1, 328(%rsp)
	vmovss	640(%r13), %xmm1
	vmovss	%xmm1, 320(%rsp)
	vmovss	644(%r13), %xmm1
	vmovss	%xmm1, 272(%rsp)
	vmovss	648(%r13), %xmm1
	vmovss	%xmm1, 488(%rsp)
	vmovss	652(%r13), %xmm1
	vmovss	%xmm1, 480(%rsp)
	vmovss	656(%r13), %xmm1
	vmovss	%xmm1, 472(%rsp)
	vmovss	660(%r13), %xmm1
	vmovss	%xmm1, 408(%rsp)
	vmovss	664(%r13), %xmm1
	vmovss	%xmm1, 312(%rsp)
	vmovss	668(%r13), %xmm1
	vmovss	%xmm1, 464(%rsp)
	vmovss	672(%r13), %xmm1
	vmovss	%xmm1, 400(%rsp)
	vmovss	676(%r13), %xmm1
	vmovss	%xmm1, 68(%rsp)
	vmovss	680(%r13), %xmm1
	vmovss	%xmm1, 456(%rsp)
	vmovss	684(%r13), %xmm1
	vmovss	%xmm1, 304(%rsp)
	vmovss	688(%r13), %xmm1
	vmovss	%xmm1, 392(%rsp)
	vmovss	692(%r13), %xmm1
	vmovss	%xmm1, 16(%rsp)
	vmovss	696(%r13), %xmm1
	vmovss	%xmm1, 12(%rsp)
	vmovss	700(%r13), %xmm1
	vmovss	%xmm1, 264(%rsp)
	vmovss	704(%r13), %xmm1
	vmovss	%xmm1, 384(%rsp)
	vmovss	708(%r13), %xmm1
	vmovss	%xmm1, 72(%rsp)
	vmovss	712(%r13), %xmm1
	vmovss	%xmm1, 448(%rsp)
	vmovss	716(%r13), %xmm1
	vmovss	%xmm1, 440(%rsp)
	vmovss	720(%r13), %xmm1
	vmovss	%xmm1, 96(%rsp)
	vmovss	724(%r13), %xmm1
	vmovss	%xmm1, 88(%rsp)
	vmovss	728(%r13), %xmm1
	vmovss	%xmm1, 28(%rsp)
	vmovss	732(%r13), %xmm1
	vmovss	%xmm1, 24(%rsp)
	vmovss	736(%r13), %xmm1
	vmovss	%xmm1, 20(%rsp)
	vmovss	740(%r13), %xmm1
	vmovss	%xmm1, 664(%rsp)
	vmovss	744(%r13), %xmm1
	vmovss	%xmm1, 656(%rsp)
	vmovss	748(%r13), %xmm1
	vmovss	%xmm1, 140(%rsp)
	vmovss	752(%r13), %xmm1
	vmovss	%xmm1, 136(%rsp)
	vmovss	756(%r13), %xmm1
	vmovss	%xmm1, 132(%rsp)
	vmovss	760(%r13), %xmm1
	vmovss	%xmm1, 128(%rsp)
	vmovss	764(%r13), %xmm1
	vmovss	%xmm1, 124(%rsp)
	vmovss	768(%r13), %xmm1
	vmovss	%xmm1, 432(%rsp)
	vmovss	772(%r13), %xmm1
	vmovss	%xmm1, 648(%rsp)
	.loc	5 0 12 is_stmt 0
.Ltmp10150:
	.p2align	4
.LBB36_271:
	vmovss	56(%rsp), %xmm10
	vmovss	48(%rsp), %xmm15
	vmovss	160(%rsp), %xmm11
	vmovss	608(%rsp), %xmm14
	vmovss	624(%rsp), %xmm3
	vmovss	144(%rsp), %xmm5
	.loc	5 568 12 is_stmt 1
	leaq	(%r8,%rdx), %rax
	cmpq	%r10, %rax
	ja	.LBB36_279
.Ltmp10151:
	.loc	5 0 12 is_stmt 0
	vmovss	%xmm5, 32(%rsp)
	vmovss	%xmm3, 368(%rsp)
	.loc	1 1762 0 is_stmt 1
	movq	%rcx, %rax
	addq	%rdx, %rax
.Ltmp10152:
	.loc	52 51 9
	je	.LBB36_81
	.loc	52 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm8
	vmovaps	%xmm4, %xmm7
	vmovaps	%xmm2, %xmm13
	vmovaps	%xmm9, %xmm1
	vmovaps	%xmm6, %xmm3
	movq	696(%rsp), %rax
	.loc	52 51 9
	vmovss	(%rax,%rdx,4), %xmm6
	vbroadcastss	.LCPI36_0(%rip), %xmm12
.Ltmp10153:
	.loc	52 103 24 is_stmt 1
	vandps	%xmm4, %xmm12, %xmm0
	vmovaps	%xmm0, 144(%rsp)
.Ltmp10154:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm6, %xmm0
	vxorps	%xmm9, %xmm9, %xmm9
.Ltmp10155:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10156:
	.loc	52 71 9
	vmulss	256(%rsp), %xmm6, %xmm4
.Ltmp10157:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10158:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm6, %xmm5
.Ltmp10159:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
	vmovss	%xmm6, 56(%rsp)
.Ltmp10160:
	.loc	52 71 9
	vmulss	192(%rsp), %xmm6, %xmm6
.Ltmp10161:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovss	%xmm10, 208(%rsp)
.Ltmp10162:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm10, %xmm9
.Ltmp10163:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10164:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm10, %xmm9
.Ltmp10165:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10166:
	.loc	52 71 9
	vmulss	600(%rsp), %xmm10, %xmm9
.Ltmp10167:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10168:
	.loc	52 71 9
	vmulss	592(%rsp), %xmm10, %xmm9
.Ltmp10169:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10170:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm3, %xmm9
.Ltmp10171:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10172:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm3, %xmm9
.Ltmp10173:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10174:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm3, %xmm9
.Ltmp10175:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10176:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm3, %xmm9
.Ltmp10177:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10178:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm1, %xmm9
.Ltmp10179:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10180:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm1, %xmm9
.Ltmp10181:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10182:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm1, %xmm9
.Ltmp10183:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10184:
	.loc	52 71 9
	vmulss	272(%rsp), %xmm1, %xmm9
.Ltmp10185:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm2, %xmm10
.Ltmp10186:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm13, %xmm9
.Ltmp10187:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10188:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm13, %xmm9
.Ltmp10189:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10190:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm13, %xmm9
.Ltmp10191:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10192:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm13, %xmm9
.Ltmp10193:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10194:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm8, %xmm9
.Ltmp10195:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10196:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm8, %xmm9
.Ltmp10197:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10198:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm8, %xmm9
.Ltmp10199:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10200:
	.loc	52 71 9
	vmulss	68(%rsp), %xmm8, %xmm9
.Ltmp10201:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10202:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm7, %xmm9
.Ltmp10203:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10204:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm7, %xmm9
.Ltmp10205:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10206:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm7, %xmm9
.Ltmp10207:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp10208:
	.loc	52 71 9
	vmulss	16(%rsp), %xmm7, %xmm9
.Ltmp10209:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10210:
	.loc	52 71 9
	vmulss	12(%rsp), %xmm15, %xmm9
.Ltmp10211:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10212:
	.loc	52 71 9
	vmulss	264(%rsp), %xmm15, %xmm9
.Ltmp10213:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10214:
	.loc	52 71 9
	vmulss	384(%rsp), %xmm15, %xmm9
.Ltmp10215:
	.loc	52 61 9
	vaddss	%xmm5, %xmm9, %xmm2
.Ltmp10216:
	.loc	52 71 9
	vmulss	72(%rsp), %xmm15, %xmm9
.Ltmp10217:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm11, %xmm5
.Ltmp10218:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm11, %xmm9
.Ltmp10219:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10220:
	.loc	52 71 9
	vmulss	440(%rsp), %xmm11, %xmm9
.Ltmp10221:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10222:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm11, %xmm9
.Ltmp10223:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10224:
	.loc	52 71 9
	vmulss	88(%rsp), %xmm11, %xmm9
.Ltmp10225:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp10226:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm14, %xmm9
.Ltmp10227:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10228:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm14, %xmm9
.Ltmp10229:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10230:
	.loc	52 71 9
	vmulss	20(%rsp), %xmm14, %xmm9
.Ltmp10231:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10232:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm14, %xmm9
.Ltmp10233:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovss	368(%rsp), %xmm11
.Ltmp10234:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm11, %xmm9
.Ltmp10235:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10236:
	.loc	52 71 9
	vmulss	140(%rsp), %xmm11, %xmm9
.Ltmp10237:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10238:
	.loc	52 71 9
	vmulss	136(%rsp), %xmm11, %xmm9
.Ltmp10239:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10240:
	.loc	52 71 9
	vmulss	132(%rsp), %xmm11, %xmm9
.Ltmp10241:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm8, %xmm13
	vmovss	32(%rsp), %xmm8
.Ltmp10242:
	.loc	52 71 9
	vmulss	128(%rsp), %xmm8, %xmm9
.Ltmp10243:
	.loc	52 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp10244:
	.loc	52 71 9
	vmulss	124(%rsp), %xmm8, %xmm9
.Ltmp10245:
	.loc	52 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp10246:
	.loc	52 71 9
	vmulss	432(%rsp), %xmm8, %xmm9
.Ltmp10247:
	.loc	52 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp10248:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm8, %xmm9
.Ltmp10249:
	.loc	52 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm12, %xmm9
.Ltmp10250:
	.loc	52 103 24
	vandps	%xmm0, %xmm12, %xmm0
	vmovaps	144(%rsp), %xmm12
.Ltmp10251:
	.loc	52 161 24
	vmaxss	%xmm0, %xmm12, %xmm0
	vmovaps	%xmm1, %xmm12
	vmovss	208(%rsp), %xmm1
.Ltmp10252:
	.loc	52 103 24
	vandps	%xmm4, %xmm9, %xmm4
.Ltmp10253:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp10254:
	.loc	52 103 24
	vandps	%xmm2, %xmm9, %xmm4
.Ltmp10255:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp10256:
	.loc	52 103 24
	vandps	%xmm6, %xmm9, %xmm4
.Ltmp10257:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp10258:
	.loc	52 56 9
	vmovss	%xmm0, 1616(%rsp,%rdx,4)
.Ltmp10259:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm11, 144(%rsp)
	vmovss	%xmm14, 624(%rsp)
	vmovss	%xmm5, 608(%rsp)
	vmovss	%xmm15, 160(%rsp)
	vmovss	%xmm7, 48(%rsp)
	vmovaps	%xmm13, %xmm4
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm12, %xmm2
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm1, %xmm6
.Ltmp10260:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %rsi
.Ltmp10261:
	.loc	3 900 12
	jne	.LBB36_271
.Ltmp10262:
.LBB36_274:
	.loc	3 0 12 is_stmt 0
	vmovss	56(%rsp), %xmm0
	.loc	1 1765 5 is_stmt 1
	vmovss	%xmm0, 1316(%rsp)
	vmovss	%xmm1, 1320(%rsp)
	vmovss	%xmm3, 1324(%rsp)
	vmovss	%xmm12, 1328(%rsp)
	vmovss	%xmm10, 1332(%rsp)
	vmovss	%xmm13, 1336(%rsp)
	vmovss	%xmm7, 1340(%rsp)
	vmovss	%xmm15, 1344(%rsp)
	vmovss	%xmm5, 1348(%rsp)
	vmovss	%xmm14, 1352(%rsp)
	vmovss	%xmm11, 1356(%rsp)
	vmovss	%xmm8, 1360(%rsp)
.Ltmp10263:
	.loc	10 1916 50
	cmpq	%r8, 232(%rsp)
	je	.LBB36_261
.Ltmp10264:
	.loc	10 0 50 is_stmt 0
	vmovss	1272(%rsp), %xmm8
	vmovss	1288(%rsp), %xmm9
	vmovss	1364(%rsp), %xmm10
	vmovss	1380(%rsp), %xmm11
	movq	544(%r13), %rcx
	movq	552(%r13), %rax
	movq	%rax, 672(%rsp)
	vmovss	1312(%rsp), %xmm13
	vmovss	1404(%rsp), %xmm15
	vmovss	1308(%rsp), %xmm12
	vmovss	1304(%rsp), %xmm7
	vmovss	1400(%rsp), %xmm14
	vmovss	1396(%rsp), %xmm6
	xorl	%edx, %edx
	vmovss	.LCPI36_2(%rip), %xmm5
	movq	%r8, 816(%rsp)
	movq	%rsi, 104(%rsp)
	.p2align	4
.LBB36_276:
.Ltmp10265:
	.loc	52 51 9 is_stmt 1
	vmovss	2648(%rsp,%rdx,4), %xmm0
.Ltmp10266:
	.loc	52 51 9 is_stmt 0
	vmovss	1616(%rsp,%rdx,4), %xmm1
.Ltmp10267:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
.Ltmp10268:
	.loc	1 1835 24
	leaq	(%rdx,%r8), %rdi
.Ltmp10269:
	.loc	41 1244 18
	vmovd	%xmm1, %r9d
.Ltmp10270:
	.loc	41 1244 18 is_stmt 0
	vmovd	%xmm0, %eax
.Ltmp10271:
	.loc	52 161 24 is_stmt 1
	movl	%r9d, %r10d
	cmovbel	%eax, %r10d
.Ltmp10272:
	.loc	5 568 12
	cmpq	%r14, %rdi
	ja	.LBB36_377
.Ltmp10273:
	.loc	52 51 9
	je	.LBB36_278
.Ltmp10274:
	.loc	1 0 0 is_stmt 0
	andl	824(%rsp), %r10d
	andl	%r12d, %eax
	orl	%r10d, %eax
	vmovd	%eax, %xmm1
.Ltmp10275:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm8, %xmm1
	vmovaps	%xmm5, %xmm0
.Ltmp10276:
	.loc	52 161 24
	jbe	.LBB36_283
.Ltmp10277:
	.loc	52 76 9
	vdivss	%xmm1, %xmm8, %xmm0
.Ltmp10278:
.LBB36_283:
	.loc	1 0 0 is_stmt 0
	movq	328(%r13), %r11
.Ltmp10279:
	.loc	1 1392 25 is_stmt 1
	movq	176(%r13), %rsi
	movq	%r11, 160(%rsp)
	.loc	1 1392 45 is_stmt 0
	imulq	(%rsp), %r11
.Ltmp10280:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %r11
	ja	.LBB36_346
.Ltmp10281:
	.loc	5 0 12 is_stmt 0
	movl	%r10d, 32(%rsp)
	movl	%r9d, 368(%rsp)
	movq	%rdx, 208(%rsp)
.Ltmp10282:
	.loc	52 56 9 is_stmt 1
	je	.LBB36_285
.Ltmp10283:
	.loc	52 0 9 is_stmt 0
	movq	%rdi, 608(%rsp)
	vmovss	(%rbx,%rdi,4), %xmm3
.Ltmp10284:
	.loc	1 1392 0 is_stmt 1
	movq	168(%r13), %rax
	movq	%r11, 48(%rsp)
.Ltmp10285:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%r11,4)
.Ltmp10286:
	.loc	1 1259 17
	movq	328(%r13), %rbx
.Ltmp10287:
	.loc	12 37 12
	testq	%rbx, %rbx
	je	.LBB36_301
.Ltmp10288:
	.loc	12 0 12 is_stmt 0
	movq	936(%rsp), %rdx
	movq	320(%rdx), %rax
	movq	%rax, 56(%rsp)
	movq	(%rsp), %rax
	leaq	1(%rax), %r8
	cmpq	%rcx, %r8
	movq	%rcx, %rax
	movl	$0, %esi
	cmovbq	%rsi, %rax
	movq	312(%rdx), %rdi
	subq	%rax, %r8
	movq	168(%rdx), %r14
	movq	176(%rdx), %rsi
	movq	256(%rdx), %rax
	movq	%rax, 144(%rsp)
	movq	248(%rdx), %r15
	movq	224(%rdx), %r9
	movq	216(%rdx), %rdx
	imulq	%rbx, %r8
	movq	%r8, 624(%rsp)
	movq	%rbx, %r13
	xorl	%r11d, %r11d
	xorl	%ebp, %ebp
	jmp	.LBB36_288
	.p2align	4
.LBB36_345:
	xorl	%r10d, %r10d
.LBB36_300:
	decq	%r13
	addq	$4, %rbp
.Ltmp10289:
	movl	%r10d, (%r15,%r11,4)
.Ltmp10290:
	incq	%r11
.Ltmp10291:
	.loc	12 37 12 is_stmt 1
	testq	%r13, %r13
	je	.LBB36_301
.LBB36_288:
.Ltmp10292:
	.loc	7 1714 9
	cmpq	$32, %rbp
.Ltmp10293:
	.loc	6 180 28
	je	.LBB36_301
.Ltmp10294:
	.loc	1 1263 21
	cmpq	56(%rsp), %r11
	je	.LBB36_378
	leaq	(%r11,%r11,2), %r8
	movl	4(%rdi,%r8,4), %eax
.Ltmp10295:
	.loc	1 1265 23
	addq	(%rsp), %rax
.Ltmp10296:
	.loc	1 1266 12
	cmpq	%rcx, %rax
	movl	$0, %r10d
	cmovaeq	%rcx, %r10
	subq	%r10, %rax
.Ltmp10297:
	.loc	1 1273 42
	movq	%rax, %r12
	imulq	%rbx, %r12
	addq	%r11, %r12
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_379
.Ltmp10298:
	.loc	1 1274 24 is_stmt 1
	cmpq	144(%rsp), %r11
	je	.LBB36_380
.Ltmp10299:
	.loc	1 0 0 is_stmt 0
	movl	(%rdi,%r8,4), %r8d
.Ltmp10300:
	vmovss	(%r14,%r12,4), %xmm0
.Ltmp10301:
	.loc	1 1274 24
	movl	(%r15,%r11,4), %r10d
	testq	%r10, %r10
.Ltmp10302:
	.loc	1 1275 26 is_stmt 1
	je	.LBB36_295
	.loc	1 1278 24
	cmpq	%r9, %r11
	jae	.LBB36_381
	vmovss	(%rdx,%r11,4), %xmm1
.Ltmp10303:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB36_295
.Ltmp10304:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB36_295:
.Ltmp10305:
	.loc	1 1280 9 is_stmt 1
	cmpq	%r9, %r11
	je	.LBB36_383
	vmovss	%xmm0, (%rdx,%r11,4)
	.loc	1 1281 24
	incq	%r10
	cmpq	%r8, %r10
.Ltmp10306:
	.loc	1 1282 23
	jne	.LBB36_297
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 832(%rsp,%rbp)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%r14,%r12,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp10307:
	.p2align	4
.LBB36_342:
.Ltmp10308:
	.loc	1 1291 65 is_stmt 1
	movq	%rax, %r12
	imulq	%rbx, %r12
	addq	%r11, %r12
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_343
.Ltmp10309:
	.loc	1 903 8 is_stmt 1
	vminss	(%r14,%r12,4), %xmm0, %xmm0
.Ltmp10310:
	.loc	1 1292 17
	vmovss	%xmm0, (%r14,%r12,4)
	.loc	1 1293 20
	testq	%rax, %rax
	cmoveq	%rcx, %rax
	.loc	1 1296 17
	decq	%rax
.Ltmp10311:
	.loc	10 1916 50
	decq	%r8
.Ltmp10312:
	.loc	3 900 12
	jne	.LBB36_342
	jmp	.LBB36_345
.Ltmp10313:
	.loc	3 0 12 is_stmt 0
.Ltmp10314:
	.p2align	4
.LBB36_297:
	movq	624(%rsp), %rax
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%rax), %r12
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_298
	vmovss	(%r14,%r12,4), %xmm1
.Ltmp10315:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp10316:
	.loc	1 1282 9
	vmovss	%xmm0, 832(%rsp,%rbp)
	jmp	.LBB36_300
.Ltmp10317:
	.loc	1 0 9 is_stmt 0
.Ltmp10318:
	.p2align	4
.LBB36_301:
	.loc	52 51 9 is_stmt 1
	vmovss	832(%rsp), %xmm1
.Ltmp10319:
	.loc	52 71 9
	vmulss	.LCPI36_3(%rip), %xmm1, %xmm0
.Ltmp10320:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp10321:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm0
	movq	936(%rsp), %r13
.Ltmp10322:
	.loc	1 1412 26
	movq	192(%r13), %rsi
	movq	160(%rsp), %r15
.Ltmp10323:
	.loc	12 37 12
	testq	%r15, %r15
	movq	296(%rsp), %r10
	movq	680(%rsp), %rbx
	movl	288(%rsp), %r14d
	je	.LBB36_306
.Ltmp10324:
	.loc	12 0 12 is_stmt 0
	movq	%r10, %rdi
	movq	320(%r13), %r10
.Ltmp10325:
	.loc	1 1403 42 is_stmt 1
	testq	%r10, %r10
	je	.LBB36_350
	.loc	1 0 42 is_stmt 0
	movq	312(%r13), %r11
	.loc	1 1403 42
	movl	8(%r11), %r8d
	.loc	1 1403 28
	addq	(%rsp), %r8
.Ltmp10326:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%r15, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	.loc	1 0 25
	movq	184(%r13), %r9
	.loc	1 1407 25
	vmovss	(%r9,%r8,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 832(%rsp)
.Ltmp10327:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %r15
	je	.LBB36_305
.Ltmp10328:
	.loc	1 1403 42
	cmpq	$1, %r10
	je	.LBB36_348
	movl	20(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10329:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r15, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	4(%r9,%rax,4), %xmm2
	.loc	1 1407 13
	vmovss	%xmm2, 836(%rsp)
.Ltmp10330:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %r15
	je	.LBB36_305
.Ltmp10331:
	.loc	1 1403 42
	cmpq	$2, %r10
	je	.LBB36_354
	movl	32(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10332:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r15, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	8(%r9,%rax,4), %xmm2
	.loc	1 1407 13
	vmovss	%xmm2, 840(%rsp)
.Ltmp10333:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %r15
	je	.LBB36_305
.Ltmp10334:
	.loc	1 1403 42
	cmpq	$3, %r10
	je	.LBB36_358
	movl	44(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10335:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r15, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	12(%r9,%rax,4), %xmm2
	.loc	1 1407 13
	vmovss	%xmm2, 844(%rsp)
.Ltmp10336:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %r15
	je	.LBB36_305
.Ltmp10337:
	.loc	1 1403 42
	cmpq	$4, %r10
	je	.LBB36_362
	movl	56(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10338:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r15, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	16(%r9,%rax,4), %xmm2
	.loc	1 1407 13
	vmovss	%xmm2, 848(%rsp)
.Ltmp10339:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %r15
	je	.LBB36_305
.Ltmp10340:
	.loc	1 1403 42
	cmpq	$5, %r10
	je	.LBB36_366
	movl	68(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10341:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r15, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	20(%r9,%rax,4), %xmm2
	.loc	1 1407 13
	vmovss	%xmm2, 852(%rsp)
.Ltmp10342:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %r15
	je	.LBB36_305
.Ltmp10343:
	.loc	1 1403 42
	cmpq	$6, %r10
	je	.LBB36_370
	movl	80(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10344:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r15, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	24(%r9,%rax,4), %xmm2
	.loc	1 1407 13
	vmovss	%xmm2, 856(%rsp)
.Ltmp10345:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %r15
	je	.LBB36_305
.Ltmp10346:
	.loc	1 1403 42
	cmpq	$7, %r10
	je	.LBB36_374
	movl	92(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10347:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	%r15, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	28(%r9,%rax,4), %xmm2
	.loc	1 1407 13
	vmovss	%xmm2, 860(%rsp)
.Ltmp10348:
	.loc	1 0 13
.Ltmp10349:
	.p2align	4
.LBB36_305:
	movq	%rdi, %r10
.LBB36_306:
	movq	48(%rsp), %rdi
.Ltmp10350:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %rdi
	ja	.LBB36_666
.Ltmp10351:
	.loc	5 0 12 is_stmt 0
	movq	928(%rsp), %rbp
	movl	84(%rsp), %r12d
.Ltmp10352:
	.loc	52 56 9 is_stmt 1
	je	.LBB36_285
.Ltmp10353:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm0, %xmm12, %xmm2
	vsubss	%xmm1, %xmm2, %xmm12
	.loc	1 1412 0 is_stmt 1
	movq	184(%r13), %rax
.Ltmp10354:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp10355:
	.loc	52 76 9
	vdivss	%xmm13, %xmm12, %xmm0
.Ltmp10356:
	.loc	52 66 9
	vsubss	%xmm0, %xmm5, %xmm0
.Ltmp10357:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm7, %xmm0, %xmm2
.Ltmp10358:
	.loc	52 92 9 is_stmt 1
	vmulss	%xmm2, %xmm9, %xmm2
	vaddss	%xmm2, %xmm7, %xmm2
.Ltmp10359:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm0, %xmm0
.Ltmp10360:
	.loc	52 103 24
	vbroadcastss	.LCPI36_0(%rip), %xmm2
	vandps	%xmm2, %xmm0, %xmm4
.Ltmp10361:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm4, %xmm4
	vandps	%xmm0, %xmm4, %xmm7
.Ltmp10362:
	.loc	1 1420 28
	movq	160(%r13), %rdx
	.loc	1 1420 44 is_stmt 0
	imulq	800(%rsp), %r15
	movq	%r15, %rsi
.Ltmp10363:
	.loc	5 568 12 is_stmt 1
	cmpq	%rdx, %r15
	movq	608(%rsp), %rdi
	ja	.LBB36_390
.Ltmp10364:
	.loc	52 51 9
	je	.LBB36_278
.Ltmp10365:
	.loc	1 0 0 is_stmt 0
	vsubss	%xmm7, %xmm5, %xmm0
.Ltmp10366:
	.loc	1 1420 0 is_stmt 1
	movq	152(%r13), %rax
.Ltmp10367:
	.loc	52 51 9
	vmovss	(%rax,%rsi,4), %xmm4
.Ltmp10368:
	.loc	52 56 9
	vmovss	%xmm3, (%rax,%rsi,4)
.Ltmp10369:
	.loc	52 71 9
	vmulss	%xmm4, %xmm0, %xmm0
.Ltmp10370:
	.loc	41 1244 18
	vmovd	%xmm4, %eax
.Ltmp10371:
	.loc	52 161 24
	andl	360(%rsp), %eax
.Ltmp10372:
	.loc	41 1244 18
	vmovd	%xmm0, %edx
.Ltmp10373:
	.loc	52 161 44
	andl	%r14d, %edx
	.loc	52 161 24 is_stmt 0
	orl	%eax, %edx
.Ltmp10374:
	.loc	52 56 9 is_stmt 1
	movl	%edx, (%rbx,%rdi,4)
.Ltmp10375:
	.loc	5 568 12
	cmpq	%r10, %rdi
	ja	.LBB36_413
.Ltmp10376:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_278
	movl	368(%rsp), %eax
	andl	%r12d, %eax
	movl	32(%rsp), %edx
	orl	%eax, %edx
	vmovd	%edx, %xmm3
.Ltmp10377:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm3
	vmovaps	%xmm5, %xmm0
.Ltmp10378:
	.loc	52 161 24
	jbe	.LBB36_314
.Ltmp10379:
	.loc	52 76 9
	vdivss	%xmm3, %xmm10, %xmm0
.Ltmp10380:
.LBB36_314:
	.loc	1 0 0 is_stmt 0
	movq	528(%r13), %rdx
.Ltmp10381:
	.loc	1 1392 25 is_stmt 1
	movq	376(%r13), %rsi
	.loc	1 1392 45 is_stmt 0
	movq	%rdx, %r15
	imulq	(%rsp), %r15
.Ltmp10382:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %r15
	ja	.LBB36_388
.Ltmp10383:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_285
	vmovss	(%rbp,%rdi,4), %xmm3
.Ltmp10384:
	.loc	1 1392 0 is_stmt 1
	movq	368(%r13), %rax
.Ltmp10385:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%r15,4)
.Ltmp10386:
	.loc	1 1259 17
	movq	528(%r13), %r14
.Ltmp10387:
	.loc	12 37 12
	testq	%r14, %r14
	movq	%rdx, 160(%rsp)
	je	.LBB36_331
.Ltmp10388:
	.loc	12 0 12 is_stmt 0
	movq	%r15, 48(%rsp)
	movq	520(%r13), %rax
	movq	%rax, 56(%rsp)
	movq	(%rsp), %rax
	leaq	1(%rax), %r8
	cmpq	%rcx, %r8
	movq	%rcx, %rax
	movl	$0, %edx
	cmovbq	%rdx, %rax
	movq	512(%r13), %rdi
	subq	%rax, %r8
	movq	368(%r13), %rbx
	movq	376(%r13), %rsi
	movq	456(%r13), %rax
	movq	%rax, 144(%rsp)
	movq	448(%r13), %r15
	movq	424(%r13), %r9
	movq	416(%r13), %r13
	imulq	%r14, %r8
	movq	%r8, 624(%rsp)
	movq	%r14, %rdx
	xorl	%r11d, %r11d
	xorl	%ebp, %ebp
	jmp	.LBB36_318
	.p2align	4
.LBB36_387:
	xorl	%r10d, %r10d
.LBB36_329:
	decq	%rdx
	addq	$4, %rbp
.Ltmp10389:
	movl	%r10d, (%r15,%r11,4)
.Ltmp10390:
	incq	%r11
.Ltmp10391:
	.loc	12 37 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB36_330
.LBB36_318:
.Ltmp10392:
	.loc	7 1714 9
	cmpq	$32, %rbp
.Ltmp10393:
	.loc	6 180 28
	je	.LBB36_330
.Ltmp10394:
	.loc	1 1263 21
	cmpq	56(%rsp), %r11
	je	.LBB36_378
	leaq	(%r11,%r11,2), %r8
	movl	4(%rdi,%r8,4), %eax
.Ltmp10395:
	.loc	1 1265 23
	addq	(%rsp), %rax
.Ltmp10396:
	.loc	1 1266 12
	cmpq	%rcx, %rax
	movl	$0, %r10d
	cmovaeq	%rcx, %r10
	subq	%r10, %rax
.Ltmp10397:
	.loc	1 1273 42
	movq	%rax, %r12
	imulq	%r14, %r12
	addq	%r11, %r12
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_379
.Ltmp10398:
	.loc	1 1274 24 is_stmt 1
	cmpq	144(%rsp), %r11
	je	.LBB36_380
.Ltmp10399:
	.loc	1 0 0 is_stmt 0
	movl	(%rdi,%r8,4), %r8d
.Ltmp10400:
	vmovss	(%rbx,%r12,4), %xmm0
.Ltmp10401:
	.loc	1 1274 24
	movl	(%r15,%r11,4), %r10d
	testq	%r10, %r10
	je	.LBB36_325
.Ltmp10402:
	.loc	1 1278 24 is_stmt 1
	cmpq	%r9, %r11
	jae	.LBB36_381
	vmovss	(%r13,%r11,4), %xmm1
.Ltmp10403:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB36_325
.Ltmp10404:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB36_325:
.Ltmp10405:
	.loc	1 1280 9 is_stmt 1
	cmpq	%r9, %r11
	je	.LBB36_383
	vmovss	%xmm0, (%r13,%r11,4)
	.loc	1 1281 24
	incq	%r10
	cmpq	%r8, %r10
.Ltmp10406:
	.loc	1 1282 23
	jne	.LBB36_327
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 832(%rsp,%rbp)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rbx,%r12,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp10407:
	.p2align	4
.LBB36_385:
.Ltmp10408:
	.loc	1 1291 65 is_stmt 1
	movq	%rax, %r12
	imulq	%r14, %r12
	addq	%r11, %r12
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_343
.Ltmp10409:
	.loc	1 903 8 is_stmt 1
	vminss	(%rbx,%r12,4), %xmm0, %xmm0
.Ltmp10410:
	.loc	1 1292 17
	vmovss	%xmm0, (%rbx,%r12,4)
	.loc	1 1293 20
	testq	%rax, %rax
	cmoveq	%rcx, %rax
	.loc	1 1296 17
	decq	%rax
.Ltmp10411:
	.loc	10 1916 50
	decq	%r8
.Ltmp10412:
	.loc	3 900 12
	jne	.LBB36_385
	jmp	.LBB36_387
.Ltmp10413:
	.loc	3 0 12 is_stmt 0
.Ltmp10414:
	.p2align	4
.LBB36_327:
	movq	624(%rsp), %rax
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%rax), %r12
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_298
	vmovss	(%rbx,%r12,4), %xmm1
.Ltmp10415:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp10416:
	.loc	1 1282 9
	vmovss	%xmm0, 832(%rsp,%rbp)
	jmp	.LBB36_329
.Ltmp10417:
	.loc	1 0 9 is_stmt 0
.Ltmp10418:
	.p2align	4
.LBB36_330:
	.loc	52 51 9 is_stmt 1
	vmovss	832(%rsp), %xmm1
	movq	936(%rsp), %r13
	movq	296(%rsp), %r10
	movq	680(%rsp), %rbx
	movq	928(%rsp), %rbp
	movl	84(%rsp), %r12d
	movq	608(%rsp), %rdi
	movq	160(%rsp), %rdx
	movq	48(%rsp), %r15
.Ltmp10419:
.LBB36_331:
	.loc	52 71 9
	vmulss	.LCPI36_3(%rip), %xmm1, %xmm0
.Ltmp10420:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp10421:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm0
.Ltmp10422:
	.loc	1 1412 26
	movq	392(%r13), %rsi
.Ltmp10423:
	.loc	12 37 12
	testq	%rdx, %rdx
	movq	960(%rsp), %r14
	je	.LBB36_336
.Ltmp10424:
	.loc	12 0 12 is_stmt 0
	movl	%r12d, %r9d
	movq	%rbp, %r12
	movq	%r10, %rbp
	movq	520(%r13), %r10
.Ltmp10425:
	.loc	1 1403 42 is_stmt 1
	testq	%r10, %r10
	je	.LBB36_350
	.loc	1 0 42 is_stmt 0
	movq	512(%r13), %r11
	.loc	1 1403 42
	movl	8(%r11), %r8d
	.loc	1 1403 28
	addq	(%rsp), %r8
.Ltmp10426:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%rdx, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	.loc	1 0 25
	movq	384(%r13), %rdi
	.loc	1 1407 25
	vmovss	(%rdi,%r8,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 832(%rsp)
.Ltmp10427:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %rdx
	je	.LBB36_335
.Ltmp10428:
	.loc	1 1403 42
	cmpq	$1, %r10
	je	.LBB36_348
	movl	20(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10429:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	movq	160(%rsp), %rdx
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	4(%rdi,%rax,4), %xmm4
	.loc	1 1407 13
	vmovss	%xmm4, 836(%rsp)
.Ltmp10430:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %rdx
	je	.LBB36_335
.Ltmp10431:
	.loc	1 1403 42
	cmpq	$2, %r10
	je	.LBB36_354
	movl	32(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10432:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	movq	160(%rsp), %rdx
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	8(%rdi,%rax,4), %xmm4
	.loc	1 1407 13
	vmovss	%xmm4, 840(%rsp)
.Ltmp10433:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %rdx
	je	.LBB36_335
.Ltmp10434:
	.loc	1 1403 42
	cmpq	$3, %r10
	je	.LBB36_358
	movl	44(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10435:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	movq	160(%rsp), %rdx
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	12(%rdi,%rax,4), %xmm4
	.loc	1 1407 13
	vmovss	%xmm4, 844(%rsp)
.Ltmp10436:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %rdx
	je	.LBB36_335
.Ltmp10437:
	.loc	1 1403 42
	cmpq	$4, %r10
	je	.LBB36_362
	movl	56(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10438:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	movq	160(%rsp), %rdx
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	16(%rdi,%rax,4), %xmm4
	.loc	1 1407 13
	vmovss	%xmm4, 848(%rsp)
.Ltmp10439:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %rdx
	je	.LBB36_335
.Ltmp10440:
	.loc	1 1403 42
	cmpq	$5, %r10
	je	.LBB36_366
	movl	68(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10441:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	movq	160(%rsp), %rdx
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	20(%rdi,%rax,4), %xmm4
	.loc	1 1407 13
	vmovss	%xmm4, 852(%rsp)
.Ltmp10442:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %rdx
	je	.LBB36_335
.Ltmp10443:
	.loc	1 1403 42
	cmpq	$6, %r10
	je	.LBB36_370
	movl	80(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10444:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	movq	160(%rsp), %rdx
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	24(%rdi,%rax,4), %xmm4
	.loc	1 1407 13
	vmovss	%xmm4, 856(%rsp)
.Ltmp10445:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %rdx
	je	.LBB36_335
.Ltmp10446:
	.loc	1 1403 42
	cmpq	$7, %r10
	je	.LBB36_374
	movl	92(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10447:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edx
	cmovaeq	%rcx, %rdx
	subq	%rdx, %rax
	.loc	1 1407 40
	imulq	160(%rsp), %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_412
	vmovss	28(%rdi,%rax,4), %xmm4
	.loc	1 1407 13
	vmovss	%xmm4, 860(%rsp)
.Ltmp10448:
	.loc	1 0 13
.Ltmp10449:
	.p2align	4
.LBB36_335:
	movq	608(%rsp), %rdi
	movq	%rbp, %r10
	movq	%r12, %rbp
	movl	%r9d, %r12d
.LBB36_336:
.Ltmp10450:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %r15
	ja	.LBB36_667
.Ltmp10451:
	.loc	52 56 9
	je	.LBB36_285
.Ltmp10452:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm0, %xmm14, %xmm4
	vsubss	%xmm1, %xmm4, %xmm14
	.loc	1 1412 0 is_stmt 1
	movq	384(%r13), %rax
.Ltmp10453:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%r15,4)
.Ltmp10454:
	.loc	52 76 9
	vdivss	%xmm15, %xmm14, %xmm0
.Ltmp10455:
	.loc	52 66 9
	vsubss	%xmm0, %xmm5, %xmm0
.Ltmp10456:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm6, %xmm0, %xmm1
.Ltmp10457:
	.loc	52 92 9 is_stmt 1
	vmulss	%xmm1, %xmm11, %xmm1
	vaddss	%xmm1, %xmm6, %xmm1
.Ltmp10458:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp10459:
	.loc	52 103 24
	vandps	%xmm2, %xmm0, %xmm1
.Ltmp10460:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm1, %xmm1
	vandps	%xmm0, %xmm1, %xmm6
.Ltmp10461:
	.loc	1 1420 28
	movq	360(%r13), %rdx
	movq	160(%rsp), %rax
	.loc	1 1420 44 is_stmt 0
	imulq	800(%rsp), %rax
	movq	%rax, %rsi
.Ltmp10462:
	.loc	5 568 12 is_stmt 1
	cmpq	%rdx, %rax
	movq	816(%rsp), %r8
	ja	.LBB36_390
.Ltmp10463:
	.loc	52 51 9
	je	.LBB36_278
.Ltmp10464:
	.loc	52 0 9 is_stmt 0
	movq	208(%rsp), %r9
	incq	%r9
.Ltmp10465:
	vsubss	%xmm6, %xmm5, %xmm0
.Ltmp10466:
	.loc	1 1420 0 is_stmt 1
	movq	352(%r13), %rax
.Ltmp10467:
	.loc	52 51 9
	vmovss	(%rax,%rsi,4), %xmm1
.Ltmp10468:
	.loc	52 56 9
	vmovss	%xmm3, (%rax,%rsi,4)
.Ltmp10469:
	.loc	52 71 9
	vmulss	%xmm1, %xmm0, %xmm0
.Ltmp10470:
	.loc	41 1244 18
	vmovd	%xmm1, %eax
.Ltmp10471:
	.loc	52 161 24
	andl	360(%rsp), %eax
.Ltmp10472:
	.loc	41 1244 18
	vmovd	%xmm0, %edx
.Ltmp10473:
	.loc	52 161 44
	andl	288(%rsp), %edx
	.loc	52 161 24 is_stmt 0
	orl	%eax, %edx
.Ltmp10474:
	.loc	52 56 9 is_stmt 1
	movl	%edx, (%rbp,%rdi,4)
	movq	800(%rsp), %rax
.Ltmp10475:
	.loc	1 1881 13
	incq	%rax
	.loc	1 1882 16
	cmpq	672(%rsp), %rax
	movl	$0, %esi
	cmoveq	%rsi, %rax
	movq	%rax, 800(%rsp)
	movq	(%rsp), %rax
	.loc	1 1885 13
	incq	%rax
	.loc	1 1886 16
	cmpq	%rcx, %rax
	movl	$0, %edx
	movq	%rdx, 40(%rsp)
	movq	%r9, %rdx
	cmoveq	%rsi, %rax
	movq	%rax, (%rsp)
	movq	104(%rsp), %rsi
.Ltmp10476:
	.loc	10 1916 50
	cmpq	%rsi, %r9
.Ltmp10477:
	.loc	3 900 12
	jne	.LBB36_276
	jmp	.LBB36_260
.Ltmp10478:
.LBB36_70:
	.loc	3 0 12 is_stmt 0
	leaq	708(%rsp), %rdi
.Ltmp10479:
	.loc	1 1795 24 is_stmt 1
	leaq	136(%r13), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	496(%rsp), %rdi
.Ltmp10480:
	.loc	1 1796 25
	movq	%rbx, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp10481:
	.loc	1 1801 19
	movzbl	776(%r13), %r12d
.Ltmp10482:
	.loc	1 1802 21
	movzbl	777(%r13), %ebx
.Ltmp10483:
	.loc	1 1803 27
	movl	560(%r13), %eax
	movq	%rax, 800(%rsp)
.Ltmp10484:
	.loc	1 1804 27
	movl	564(%r13), %eax
	movq	%rax, (%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%ymm0, 1408(%rsp)
	leaq	2648(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%r15
	leaq	1616(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
	movq	232(%rsp), %rcx
.Ltmp10485:
	.loc	8 446 20
	testq	%rcx, %rcx
	je	.LBB36_71
.Ltmp10486:
	.loc	1 0 0 is_stmt 0
	movl	%r12d, %eax
	negl	%eax
	movl	%eax, 112(%rsp)
.Ltmp10487:
	movl	%ebx, %eax
	negl	%eax
	movl	%eax, 424(%rsp)
.Ltmp10488:
	.loc	4 3756 21 is_stmt 1
	movq	%rcx, %rsi
	shrq	$5, %rsi
.Ltmp10489:
	.loc	4 3757 21
	movl	%ecx, %eax
	andl	$31, %eax
.Ltmp10490:
	.loc	4 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %rsi
	decl	%r12d
	movl	%r12d, 416(%rsp)
	decl	%ebx
	movl	%ebx, 192(%rsp)
.Ltmp10491:
	.loc	8 446 20
	movq	%r14, %rax
	negq	%rax
	movq	%rax, 952(%rsp)
	movq	296(%rsp), %r9
	movq	%r9, %rax
	negq	%rax
	movq	%rax, 1488(%rsp)
	movq	$0, 184(%rsp)
	movl	$32, %edi
	movq	%rbp, 592(%rsp)
	movq	680(%rsp), %rbx
	movq	%rbx, 600(%rsp)
	xorl	%r8d, %r8d
	jmp	.LBB36_76
.Ltmp10492:
	.loc	8 0 20 is_stmt 0
.Ltmp10493:
	.p2align	4
.LBB36_91:
	movq	40(%rsp), %rcx
.LBB36_75:
	addq	$32, %r8
	decq	%rsi
.Ltmp10494:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	subq	$-128, 600(%rsp)
	subq	$-128, 592(%rsp)
	testq	%rsi, %rsi
	je	.LBB36_72
.LBB36_76:
.Ltmp10495:
	.loc	4 2584 13
	cmpq	$1, %rcx
	movq	%rcx, 40(%rsp)
	adcq	$0, %rcx
	cmpq	$32, %rcx
	cmovaeq	%rdi, %rcx
	movq	%rcx, 608(%rsp)
.Ltmp10496:
	.loc	1 1759 23
	vmovss	708(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	712(%rsp), %xmm4
	vmovss	716(%rsp), %xmm6
	vmovss	720(%rsp), %xmm3
	vmovss	724(%rsp), %xmm0
	vmovss	728(%rsp), %xmm9
	vmovss	732(%rsp), %xmm2
	vmovss	736(%rsp), %xmm14
	vmovss	740(%rsp), %xmm15
	vmovss	744(%rsp), %xmm5
	vmovss	748(%rsp), %xmm8
.Ltmp10497:
	.loc	10 1916 50
	cmpq	%r8, 232(%rsp)
.Ltmp10498:
	.loc	3 900 12
	jne	.LBB36_78
.Ltmp10499:
	.loc	1 0 0 is_stmt 0
	vmovss	752(%rsp), %xmm12
.Ltmp10500:
	.loc	3 900 12
	jmp	.LBB36_84
.Ltmp10501:
	.loc	3 0 12
.Ltmp10502:
	.p2align	4
.LBB36_78:
	vmovss	584(%r13), %xmm1
	vmovss	%xmm1, 696(%rsp)
	vmovss	588(%r13), %xmm1
	vmovss	%xmm1, 672(%rsp)
	vmovss	592(%r13), %xmm1
	vmovss	%xmm1, 256(%rsp)
	vmovss	596(%r13), %xmm1
	vmovss	%xmm1, 200(%rsp)
	vmovss	600(%r13), %xmm1
	vmovss	%xmm1, 248(%rsp)
	vmovss	604(%r13), %xmm1
	vmovss	%xmm1, 240(%rsp)
	movq	952(%rsp), %rax
.Ltmp10503:
	.loc	5 568 12 is_stmt 1
	leaq	(%rax,%r8), %rcx
	xorl	%edx, %edx
	vmovss	%xmm8, 144(%rsp)
	vmovss	%xmm5, 624(%rsp)
	vmovss	%xmm15, 160(%rsp)
	vmovss	%xmm14, 48(%rsp)
	vmovss	%xmm2, 368(%rsp)
	vmovaps	%xmm9, %xmm1
	vmovaps	%xmm0, %xmm11
	vmovaps	%xmm3, %xmm10
	vmovaps	%xmm6, %xmm7
	vmovaps	%xmm4, %xmm13
	vmovss	608(%r13), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	612(%r13), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	616(%r13), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	620(%r13), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	624(%r13), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	628(%r13), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	632(%r13), %xmm0
	vmovss	%xmm0, 272(%rsp)
	vmovss	636(%r13), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	640(%r13), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	644(%r13), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	648(%r13), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	652(%r13), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	656(%r13), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	660(%r13), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	664(%r13), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	668(%r13), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	672(%r13), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	676(%r13), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	680(%r13), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	684(%r13), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	688(%r13), %xmm0
	vmovss	%xmm0, 264(%rsp)
	vmovss	692(%r13), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	696(%r13), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	700(%r13), %xmm0
	vmovss	%xmm0, 448(%rsp)
	vmovss	704(%r13), %xmm0
	vmovss	%xmm0, 440(%rsp)
	vmovss	708(%r13), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	712(%r13), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	716(%r13), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	720(%r13), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	724(%r13), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	728(%r13), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	732(%r13), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	736(%r13), %xmm0
	vmovss	%xmm0, 140(%rsp)
	vmovss	740(%r13), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	744(%r13), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	748(%r13), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	752(%r13), %xmm0
	vmovss	%xmm0, 124(%rsp)
	vmovss	756(%r13), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	760(%r13), %xmm0
	vmovss	%xmm0, 648(%rsp)
	vmovss	764(%r13), %xmm0
	vmovss	%xmm0, 816(%rsp)
	vmovss	768(%r13), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	772(%r13), %xmm0
	vmovss	%xmm0, 824(%rsp)
	.loc	5 0 12 is_stmt 0
.Ltmp10504:
	.p2align	4
.LBB36_79:
	vmovss	56(%rsp), %xmm9
	vmovss	368(%rsp), %xmm14
	vmovss	48(%rsp), %xmm15
	vmovss	160(%rsp), %xmm0
	vmovss	624(%rsp), %xmm3
	vmovss	144(%rsp), %xmm6
	.loc	5 568 12 is_stmt 1
	leaq	(%r8,%rdx), %rax
	cmpq	%r14, %rax
	ja	.LBB36_664
.Ltmp10505:
	.loc	5 0 12 is_stmt 0
	vmovss	%xmm6, 360(%rsp)
	vmovss	%xmm3, 288(%rsp)
	vmovss	%xmm0, 32(%rsp)
	.loc	1 1762 0 is_stmt 1
	movq	%rcx, %rax
	addq	%rdx, %rax
.Ltmp10506:
	.loc	52 51 9
	je	.LBB36_81
	.loc	52 0 9 is_stmt 0
	vmovaps	%xmm13, %xmm4
	vmovaps	%xmm1, %xmm13
	vmovaps	%xmm11, %xmm5
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm7, %xmm3
	vmovaps	%xmm4, %xmm6
	movq	600(%rsp), %rax
	.loc	52 51 9
	vmovss	(%rax,%rdx,4), %xmm8
	vbroadcastss	.LCPI36_0(%rip), %xmm12
.Ltmp10507:
	.loc	52 103 24 is_stmt 1
	vandps	%xmm1, %xmm12, %xmm1
.Ltmp10508:
	.loc	52 71 9
	vmulss	696(%rsp), %xmm8, %xmm2
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp10509:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10510:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm8, %xmm4
.Ltmp10511:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10512:
	.loc	52 71 9
	vmulss	256(%rsp), %xmm8, %xmm7
.Ltmp10513:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
	vmovss	%xmm8, 56(%rsp)
.Ltmp10514:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm8, %xmm10
.Ltmp10515:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	%xmm9, 84(%rsp)
.Ltmp10516:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm9, %xmm11
.Ltmp10517:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10518:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm9, %xmm11
.Ltmp10519:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10520:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm9, %xmm11
.Ltmp10521:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10522:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm9, %xmm11
.Ltmp10523:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10524:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm6, %xmm11
.Ltmp10525:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10526:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm6, %xmm11
.Ltmp10527:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10528:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm6, %xmm11
.Ltmp10529:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10530:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm6, %xmm11
.Ltmp10531:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10532:
	.loc	52 71 9
	vmulss	272(%rsp), %xmm3, %xmm11
.Ltmp10533:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10534:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm3, %xmm11
.Ltmp10535:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10536:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm3, %xmm11
.Ltmp10537:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10538:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm3, %xmm11
.Ltmp10539:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10540:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm0, %xmm11
.Ltmp10541:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10542:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm0, %xmm11
.Ltmp10543:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10544:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm0, %xmm11
.Ltmp10545:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10546:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm0, %xmm11
.Ltmp10547:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovaps	%xmm5, %xmm9
.Ltmp10548:
	.loc	52 71 9
	vmulss	68(%rsp), %xmm5, %xmm11
.Ltmp10549:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10550:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm5, %xmm11
.Ltmp10551:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10552:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm5, %xmm11
.Ltmp10553:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10554:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm5, %xmm11
.Ltmp10555:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10556:
	.loc	52 71 9
	vmulss	16(%rsp), %xmm13, %xmm11
.Ltmp10557:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10558:
	.loc	52 71 9
	vmulss	12(%rsp), %xmm13, %xmm11
.Ltmp10559:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10560:
	.loc	52 71 9
	vmulss	264(%rsp), %xmm13, %xmm11
.Ltmp10561:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10562:
	.loc	52 71 9
	vmulss	384(%rsp), %xmm13, %xmm11
.Ltmp10563:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10564:
	.loc	52 71 9
	vmulss	72(%rsp), %xmm14, %xmm11
.Ltmp10565:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10566:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm14, %xmm11
.Ltmp10567:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10568:
	.loc	52 71 9
	vmulss	440(%rsp), %xmm14, %xmm11
.Ltmp10569:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10570:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm14, %xmm11
.Ltmp10571:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10572:
	.loc	52 71 9
	vmulss	88(%rsp), %xmm15, %xmm11
.Ltmp10573:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10574:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm15, %xmm11
.Ltmp10575:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10576:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm15, %xmm11
.Ltmp10577:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10578:
	.loc	52 71 9
	vmulss	20(%rsp), %xmm15, %xmm11
.Ltmp10579:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	32(%rsp), %xmm5
.Ltmp10580:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm5, %xmm11
.Ltmp10581:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10582:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm5, %xmm11
.Ltmp10583:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10584:
	.loc	52 71 9
	vmulss	140(%rsp), %xmm5, %xmm11
.Ltmp10585:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10586:
	.loc	52 71 9
	vmulss	136(%rsp), %xmm5, %xmm11
.Ltmp10587:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	288(%rsp), %xmm8
.Ltmp10588:
	.loc	52 71 9
	vmulss	132(%rsp), %xmm8, %xmm11
.Ltmp10589:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10590:
	.loc	52 71 9
	vmulss	128(%rsp), %xmm8, %xmm11
.Ltmp10591:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10592:
	.loc	52 71 9
	vmulss	124(%rsp), %xmm8, %xmm11
.Ltmp10593:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10594:
	.loc	52 71 9
	vmulss	432(%rsp), %xmm8, %xmm11
.Ltmp10595:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	360(%rsp), %xmm12
.Ltmp10596:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm12, %xmm11
.Ltmp10597:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10598:
	.loc	52 71 9
	vmulss	816(%rsp), %xmm12, %xmm11
.Ltmp10599:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10600:
	.loc	52 71 9
	vmulss	104(%rsp), %xmm12, %xmm11
.Ltmp10601:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10602:
	.loc	52 71 9
	vmulss	824(%rsp), %xmm12, %xmm11
.Ltmp10603:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vbroadcastss	.LCPI36_0(%rip), %xmm11
.Ltmp10604:
	.loc	52 103 24
	vandps	%xmm2, %xmm11, %xmm2
.Ltmp10605:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp10606:
	.loc	52 103 24
	vandps	%xmm4, %xmm11, %xmm2
.Ltmp10607:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp10608:
	.loc	52 103 24
	vandps	%xmm7, %xmm11, %xmm2
.Ltmp10609:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp10610:
	.loc	52 103 24
	vandps	%xmm11, %xmm10, %xmm2
.Ltmp10611:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp10612:
	.loc	52 56 9
	vmovss	%xmm1, 2648(%rsp,%rdx,4)
.Ltmp10613:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm8, 144(%rsp)
	vmovss	%xmm5, 624(%rsp)
	vmovss	%xmm15, 160(%rsp)
	vmovss	%xmm14, 48(%rsp)
	vmovaps	%xmm13, %xmm2
	vmovss	%xmm13, 368(%rsp)
	vmovaps	%xmm9, %xmm1
	vmovaps	%xmm0, %xmm11
	vmovaps	%xmm3, %xmm10
	vmovaps	%xmm6, %xmm7
	vmovss	84(%rsp), %xmm13
	vmovaps	%xmm13, %xmm4
.Ltmp10614:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, 608(%rsp)
.Ltmp10615:
	.loc	3 900 12
	jne	.LBB36_79
.Ltmp10616:
.LBB36_84:
	.loc	3 0 12 is_stmt 0
	vmovss	56(%rsp), %xmm1
	.loc	1 1765 5 is_stmt 1
	vmovss	%xmm1, 708(%rsp)
	vmovss	%xmm4, 712(%rsp)
	vmovss	%xmm6, 716(%rsp)
	vmovss	%xmm3, 720(%rsp)
	vmovss	%xmm0, 724(%rsp)
	vmovss	%xmm9, 728(%rsp)
	vmovss	%xmm2, 732(%rsp)
	vmovss	%xmm14, 736(%rsp)
	vmovss	%xmm15, 740(%rsp)
	vmovss	%xmm5, 744(%rsp)
	vmovss	%xmm8, 748(%rsp)
	vmovss	%xmm12, 752(%rsp)
.Ltmp10617:
	.loc	1 1759 23
	vmovss	496(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	500(%rsp), %xmm4
	vmovss	504(%rsp), %xmm6
	vmovss	508(%rsp), %xmm3
	vmovss	512(%rsp), %xmm0
	vmovss	516(%rsp), %xmm9
	vmovss	520(%rsp), %xmm2
	vmovss	524(%rsp), %xmm14
	vmovss	528(%rsp), %xmm15
	vmovss	532(%rsp), %xmm5
	vmovss	536(%rsp), %xmm8
.Ltmp10618:
	.loc	10 1916 50
	cmpq	%r8, 232(%rsp)
.Ltmp10619:
	.loc	3 900 12
	jne	.LBB36_86
.Ltmp10620:
	.loc	1 0 0 is_stmt 0
	vmovss	540(%rsp), %xmm12
.Ltmp10621:
	.loc	3 900 12
	jmp	.LBB36_90
.Ltmp10622:
	.loc	3 0 12
.Ltmp10623:
	.p2align	4
.LBB36_86:
	vmovss	584(%r13), %xmm1
	vmovss	%xmm1, 696(%rsp)
	vmovss	588(%r13), %xmm1
	vmovss	%xmm1, 672(%rsp)
	vmovss	592(%r13), %xmm1
	vmovss	%xmm1, 256(%rsp)
	vmovss	596(%r13), %xmm1
	vmovss	%xmm1, 200(%rsp)
	vmovss	600(%r13), %xmm1
	vmovss	%xmm1, 248(%rsp)
	vmovss	604(%r13), %xmm1
	vmovss	%xmm1, 240(%rsp)
	movq	1488(%rsp), %rax
.Ltmp10624:
	.loc	5 568 12 is_stmt 1
	leaq	(%rax,%r8), %rcx
	xorl	%edx, %edx
	vmovss	%xmm8, 144(%rsp)
	vmovss	%xmm5, 624(%rsp)
	vmovss	%xmm15, 160(%rsp)
	vmovss	%xmm14, 48(%rsp)
	vmovss	%xmm2, 368(%rsp)
	vmovaps	%xmm9, %xmm1
	vmovaps	%xmm0, %xmm11
	vmovaps	%xmm3, %xmm10
	vmovaps	%xmm6, %xmm7
	vmovaps	%xmm4, %xmm13
	vmovss	608(%r13), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	612(%r13), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	616(%r13), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	620(%r13), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	624(%r13), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	628(%r13), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	632(%r13), %xmm0
	vmovss	%xmm0, 272(%rsp)
	vmovss	636(%r13), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	640(%r13), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	644(%r13), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	648(%r13), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	652(%r13), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	656(%r13), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	660(%r13), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	664(%r13), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	668(%r13), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	672(%r13), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	676(%r13), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	680(%r13), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	684(%r13), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	688(%r13), %xmm0
	vmovss	%xmm0, 264(%rsp)
	vmovss	692(%r13), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	696(%r13), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	700(%r13), %xmm0
	vmovss	%xmm0, 448(%rsp)
	vmovss	704(%r13), %xmm0
	vmovss	%xmm0, 440(%rsp)
	vmovss	708(%r13), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	712(%r13), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	716(%r13), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	720(%r13), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	724(%r13), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	728(%r13), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	732(%r13), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	736(%r13), %xmm0
	vmovss	%xmm0, 140(%rsp)
	vmovss	740(%r13), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	744(%r13), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	748(%r13), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	752(%r13), %xmm0
	vmovss	%xmm0, 124(%rsp)
	vmovss	756(%r13), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	760(%r13), %xmm0
	vmovss	%xmm0, 648(%rsp)
	vmovss	764(%r13), %xmm0
	vmovss	%xmm0, 816(%rsp)
	vmovss	768(%r13), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	772(%r13), %xmm0
	vmovss	%xmm0, 824(%rsp)
	.loc	5 0 12 is_stmt 0
.Ltmp10625:
	.p2align	4
.LBB36_87:
	vmovss	56(%rsp), %xmm9
	vmovss	368(%rsp), %xmm14
	vmovss	48(%rsp), %xmm15
	vmovss	160(%rsp), %xmm0
	vmovss	624(%rsp), %xmm3
	vmovss	144(%rsp), %xmm6
	.loc	5 568 12 is_stmt 1
	leaq	(%r8,%rdx), %rax
	cmpq	%r9, %rax
	ja	.LBB36_662
.Ltmp10626:
	.loc	5 0 12 is_stmt 0
	vmovss	%xmm6, 360(%rsp)
	vmovss	%xmm3, 288(%rsp)
	vmovss	%xmm0, 32(%rsp)
	.loc	1 1762 0 is_stmt 1
	movq	%rcx, %rax
	addq	%rdx, %rax
.Ltmp10627:
	.loc	52 51 9
	je	.LBB36_81
	.loc	52 0 9 is_stmt 0
	vmovaps	%xmm13, %xmm4
	vmovaps	%xmm1, %xmm13
	vmovaps	%xmm11, %xmm5
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm7, %xmm3
	vmovaps	%xmm4, %xmm6
	movq	592(%rsp), %rax
	.loc	52 51 9
	vmovss	(%rax,%rdx,4), %xmm8
	vbroadcastss	.LCPI36_0(%rip), %xmm12
.Ltmp10628:
	.loc	52 103 24 is_stmt 1
	vandps	%xmm1, %xmm12, %xmm1
.Ltmp10629:
	.loc	52 71 9
	vmulss	696(%rsp), %xmm8, %xmm2
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp10630:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10631:
	.loc	52 71 9
	vmulss	672(%rsp), %xmm8, %xmm4
.Ltmp10632:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10633:
	.loc	52 71 9
	vmulss	256(%rsp), %xmm8, %xmm7
.Ltmp10634:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
	vmovss	%xmm8, 56(%rsp)
.Ltmp10635:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm8, %xmm10
.Ltmp10636:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	%xmm9, 84(%rsp)
.Ltmp10637:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm9, %xmm11
.Ltmp10638:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10639:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm9, %xmm11
.Ltmp10640:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10641:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm9, %xmm11
.Ltmp10642:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10643:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm9, %xmm11
.Ltmp10644:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10645:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm6, %xmm11
.Ltmp10646:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10647:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm6, %xmm11
.Ltmp10648:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10649:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm6, %xmm11
.Ltmp10650:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10651:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm6, %xmm11
.Ltmp10652:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10653:
	.loc	52 71 9
	vmulss	272(%rsp), %xmm3, %xmm11
.Ltmp10654:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10655:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm3, %xmm11
.Ltmp10656:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10657:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm3, %xmm11
.Ltmp10658:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10659:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm3, %xmm11
.Ltmp10660:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10661:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm0, %xmm11
.Ltmp10662:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10663:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm0, %xmm11
.Ltmp10664:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10665:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm0, %xmm11
.Ltmp10666:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10667:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm0, %xmm11
.Ltmp10668:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovaps	%xmm5, %xmm9
.Ltmp10669:
	.loc	52 71 9
	vmulss	68(%rsp), %xmm5, %xmm11
.Ltmp10670:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10671:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm5, %xmm11
.Ltmp10672:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10673:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm5, %xmm11
.Ltmp10674:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10675:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm5, %xmm11
.Ltmp10676:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10677:
	.loc	52 71 9
	vmulss	16(%rsp), %xmm13, %xmm11
.Ltmp10678:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10679:
	.loc	52 71 9
	vmulss	12(%rsp), %xmm13, %xmm11
.Ltmp10680:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10681:
	.loc	52 71 9
	vmulss	264(%rsp), %xmm13, %xmm11
.Ltmp10682:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10683:
	.loc	52 71 9
	vmulss	384(%rsp), %xmm13, %xmm11
.Ltmp10684:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10685:
	.loc	52 71 9
	vmulss	72(%rsp), %xmm14, %xmm11
.Ltmp10686:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10687:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm14, %xmm11
.Ltmp10688:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10689:
	.loc	52 71 9
	vmulss	440(%rsp), %xmm14, %xmm11
.Ltmp10690:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10691:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm14, %xmm11
.Ltmp10692:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp10693:
	.loc	52 71 9
	vmulss	88(%rsp), %xmm15, %xmm11
.Ltmp10694:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10695:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm15, %xmm11
.Ltmp10696:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10697:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm15, %xmm11
.Ltmp10698:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10699:
	.loc	52 71 9
	vmulss	20(%rsp), %xmm15, %xmm11
.Ltmp10700:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	32(%rsp), %xmm5
.Ltmp10701:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm5, %xmm11
.Ltmp10702:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10703:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm5, %xmm11
.Ltmp10704:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10705:
	.loc	52 71 9
	vmulss	140(%rsp), %xmm5, %xmm11
.Ltmp10706:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10707:
	.loc	52 71 9
	vmulss	136(%rsp), %xmm5, %xmm11
.Ltmp10708:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	288(%rsp), %xmm8
.Ltmp10709:
	.loc	52 71 9
	vmulss	132(%rsp), %xmm8, %xmm11
.Ltmp10710:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10711:
	.loc	52 71 9
	vmulss	128(%rsp), %xmm8, %xmm11
.Ltmp10712:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10713:
	.loc	52 71 9
	vmulss	124(%rsp), %xmm8, %xmm11
.Ltmp10714:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10715:
	.loc	52 71 9
	vmulss	432(%rsp), %xmm8, %xmm11
.Ltmp10716:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	360(%rsp), %xmm12
.Ltmp10717:
	.loc	52 71 9
	vmulss	648(%rsp), %xmm12, %xmm11
.Ltmp10718:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp10719:
	.loc	52 71 9
	vmulss	816(%rsp), %xmm12, %xmm11
.Ltmp10720:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp10721:
	.loc	52 71 9
	vmulss	104(%rsp), %xmm12, %xmm11
.Ltmp10722:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp10723:
	.loc	52 71 9
	vmulss	824(%rsp), %xmm12, %xmm11
.Ltmp10724:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vbroadcastss	.LCPI36_0(%rip), %xmm11
.Ltmp10725:
	.loc	52 103 24
	vandps	%xmm2, %xmm11, %xmm2
.Ltmp10726:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp10727:
	.loc	52 103 24
	vandps	%xmm4, %xmm11, %xmm2
.Ltmp10728:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp10729:
	.loc	52 103 24
	vandps	%xmm7, %xmm11, %xmm2
.Ltmp10730:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp10731:
	.loc	52 103 24
	vandps	%xmm11, %xmm10, %xmm2
.Ltmp10732:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp10733:
	.loc	52 56 9
	vmovss	%xmm1, 1616(%rsp,%rdx,4)
.Ltmp10734:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm8, 144(%rsp)
	vmovss	%xmm5, 624(%rsp)
	vmovss	%xmm15, 160(%rsp)
	vmovss	%xmm14, 48(%rsp)
	vmovaps	%xmm13, %xmm2
	vmovss	%xmm13, 368(%rsp)
	vmovaps	%xmm9, %xmm1
	vmovaps	%xmm0, %xmm11
	vmovaps	%xmm3, %xmm10
	vmovaps	%xmm6, %xmm7
	vmovss	84(%rsp), %xmm13
	vmovaps	%xmm13, %xmm4
.Ltmp10735:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, 608(%rsp)
.Ltmp10736:
	.loc	3 900 12
	jne	.LBB36_87
.Ltmp10737:
.LBB36_90:
	.loc	3 0 12 is_stmt 0
	vmovss	56(%rsp), %xmm1
	.loc	1 1765 5 is_stmt 1
	vmovss	%xmm1, 496(%rsp)
	vmovss	%xmm4, 500(%rsp)
	vmovss	%xmm6, 504(%rsp)
	vmovss	%xmm3, 508(%rsp)
	vmovss	%xmm0, 512(%rsp)
	vmovss	%xmm9, 516(%rsp)
	vmovss	%xmm2, 520(%rsp)
	vmovss	%xmm14, 524(%rsp)
	vmovss	%xmm15, 528(%rsp)
	vmovss	%xmm5, 532(%rsp)
	vmovss	%xmm8, 536(%rsp)
	vmovss	%xmm12, 540(%rsp)
.Ltmp10738:
	.loc	10 1916 50
	cmpq	%r8, 232(%rsp)
.Ltmp10739:
	.loc	3 900 12
	je	.LBB36_91
.Ltmp10740:
	.loc	3 0 12 is_stmt 0
	movq	%rsi, 272(%rsp)
	vmovss	756(%rsp), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	760(%rsp), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	776(%rsp), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	544(%rsp), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	548(%rsp), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	564(%rsp), %xmm0
	vmovss	%xmm0, 320(%rsp)
	movq	544(%r13), %rbp
	movq	552(%r13), %rax
	movq	%rax, 344(%rsp)
	vmovss	796(%rsp), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	584(%rsp), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	768(%rsp), %xmm10
	vmovss	764(%rsp), %xmm12
	vmovss	784(%rsp), %xmm9
	vmovss	772(%rsp), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	780(%rsp), %xmm11
	vmovss	556(%rsp), %xmm1
	vmovss	552(%rsp), %xmm14
	vmovss	572(%rsp), %xmm0
	xorl	%esi, %esi
	vmovss	560(%rsp), %xmm2
	vmovss	%xmm2, 32(%rsp)
	vmovss	568(%rsp), %xmm4
	vmovss	792(%rsp), %xmm8
	vmovss	788(%rsp), %xmm15
	vmovss	580(%rsp), %xmm7
	vmovss	576(%rsp), %xmm3
	movl	416(%rsp), %r12d
	vxorps	%xmm6, %xmm6, %xmm6
	movq	%r8, 208(%rsp)
	.p2align	4
.LBB36_93:
	vmovss	.LCPI36_1(%rip), %xmm5
.Ltmp10741:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm5, %xmm10, %xmm13
.Ltmp10742:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm13
.Ltmp10743:
	.loc	52 161 24
	ja	.LBB36_94
.Ltmp10744:
	.loc	52 0 24 is_stmt 0
	vmovss	336(%rsp), %xmm10
	jmp	.LBB36_96
	.p2align	4
.LBB36_94:
	vmovss	248(%rsp), %xmm10
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm12, %xmm10, %xmm10
.Ltmp10745:
.LBB36_96:
	.loc	1 0 0 is_stmt 0
	vmovss	.LCPI36_2(%rip), %xmm2
.Ltmp10746:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm5, %xmm9, %xmm9
	vmovss	%xmm9, 256(%rsp)
.Ltmp10747:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm9
	vmovss	%xmm13, 672(%rsp)
.Ltmp10748:
	.loc	52 161 24
	ja	.LBB36_97
.Ltmp10749:
	.loc	52 0 24 is_stmt 0
	vmovss	280(%rsp), %xmm9
	vmovss	%xmm9, 368(%rsp)
.Ltmp10750:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm5, %xmm1, %xmm13
.Ltmp10751:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm13
.Ltmp10752:
	.loc	52 161 24
	jbe	.LBB36_101
.Ltmp10753:
.LBB36_100:
	.loc	52 0 24 is_stmt 0
	vmovss	48(%rsp), %xmm1
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm1
	vmovss	%xmm1, 48(%rsp)
.Ltmp10754:
	.loc	52 66 9
	vaddss	%xmm5, %xmm0, %xmm5
.Ltmp10755:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm5
.Ltmp10756:
	.loc	52 161 24
	jbe	.LBB36_104
.Ltmp10757:
.LBB36_103:
	.loc	52 0 24 is_stmt 0
	vmovss	32(%rsp), %xmm0
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm4, %xmm0, %xmm0
	jmp	.LBB36_105
.Ltmp10758:
	.loc	1 0 0 is_stmt 0
.Ltmp10759:
	.p2align	4
.LBB36_97:
	vmovss	368(%rsp), %xmm9
.Ltmp10760:
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm11, %xmm9, %xmm9
	vmovss	%xmm9, 368(%rsp)
.Ltmp10761:
	.loc	52 66 9
	vaddss	%xmm5, %xmm1, %xmm13
.Ltmp10762:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm13
.Ltmp10763:
	.loc	52 161 24
	ja	.LBB36_100
.Ltmp10764:
.LBB36_101:
	.loc	52 0 24 is_stmt 0
	vmovss	328(%rsp), %xmm1
	vmovss	%xmm1, 48(%rsp)
.Ltmp10765:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm5, %xmm0, %xmm5
.Ltmp10766:
	.loc	52 124 14
	vucomiss	%xmm6, %xmm5
.Ltmp10767:
	.loc	52 161 24
	ja	.LBB36_103
.Ltmp10768:
.LBB36_104:
	.loc	52 0 24 is_stmt 0
	vmovss	320(%rsp), %xmm0
.LBB36_105:
	vmovss	%xmm0, 32(%rsp)
.Ltmp10769:
	.loc	52 51 9 is_stmt 1
	vmovss	2648(%rsp,%rsi,4), %xmm0
.Ltmp10770:
	.loc	52 51 9 is_stmt 0
	vmovss	1616(%rsp,%rsi,4), %xmm1
.Ltmp10771:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
.Ltmp10772:
	.loc	1 0 0 is_stmt 0
	leaq	(%rsi,%r8), %rdi
.Ltmp10773:
	.loc	41 1244 18 is_stmt 1
	vmovd	%xmm1, %ecx
.Ltmp10774:
	.loc	41 1244 18 is_stmt 0
	vmovd	%xmm0, %eax
.Ltmp10775:
	.loc	52 161 24 is_stmt 1
	movl	%ecx, %edx
	cmovbel	%eax, %edx
.Ltmp10776:
	.loc	5 568 12
	cmpq	%r14, %rdi
	ja	.LBB36_221
.Ltmp10777:
	.loc	52 51 9
	je	.LBB36_107
.Ltmp10778:
	.loc	52 0 9 is_stmt 0
	movq	%rsi, 200(%rsp)
	andl	112(%rsp), %edx
	andl	%r12d, %eax
	orl	%edx, %eax
	vmovd	%eax, %xmm1
.Ltmp10779:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm1
	vmovaps	%xmm2, %xmm0
.Ltmp10780:
	.loc	52 161 24
	jbe	.LBB36_110
.Ltmp10781:
	.loc	52 76 9
	vdivss	%xmm1, %xmm10, %xmm0
.Ltmp10782:
.LBB36_110:
	.loc	1 0 0 is_stmt 0
	movq	328(%r13), %rax
.Ltmp10783:
	.loc	1 1392 25 is_stmt 1
	movq	176(%r13), %rsi
	.loc	1 1392 45 is_stmt 0
	movq	%rax, %r9
	imulq	(%rsp), %r9
.Ltmp10784:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %r9
	ja	.LBB36_154
.Ltmp10785:
	.loc	5 0 12 is_stmt 0
	movq	%rax, 288(%rsp)
	movl	%edx, 696(%rsp)
	movl	%ecx, 84(%rsp)
.Ltmp10786:
	.loc	52 56 9 is_stmt 1
	je	.LBB36_112
.Ltmp10787:
	.loc	52 0 9 is_stmt 0
	movq	%rdi, 160(%rsp)
	vmovss	(%rbx,%rdi,4), %xmm6
.Ltmp10788:
	.loc	1 1392 0 is_stmt 1
	movq	168(%r13), %rax
	movq	%r9, 360(%rsp)
.Ltmp10789:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%r9,4)
.Ltmp10790:
	.loc	1 1259 17
	movq	328(%r13), %rbx
.Ltmp10791:
	.loc	12 37 12
	testq	%rbx, %rbx
	je	.LBB36_140
.Ltmp10792:
	.loc	12 0 12 is_stmt 0
	movq	936(%rsp), %rdx
	movq	320(%rdx), %rax
	movq	%rax, 56(%rsp)
	movq	(%rsp), %rax
	leaq	1(%rax), %rdi
	cmpq	%rbp, %rdi
	movq	%rbp, %rax
	movl	$0, %ecx
	cmovbq	%rcx, %rax
	movq	312(%rdx), %rcx
	subq	%rax, %rdi
	movq	168(%rdx), %r13
	movq	176(%rdx), %rsi
	movq	256(%rdx), %rax
	movq	%rax, 144(%rsp)
	movq	248(%rdx), %r15
	movq	224(%rdx), %r9
	movq	216(%rdx), %rdx
	imulq	%rbx, %rdi
	movq	%rdi, 624(%rsp)
	movq	%rbx, %r14
	xorl	%r11d, %r11d
	xorl	%eax, %eax
	jmp	.LBB36_115
	.p2align	4
.LBB36_153:
	xorl	%r10d, %r10d
.LBB36_139:
	decq	%r14
	addq	$4, %rax
.Ltmp10793:
	movl	%r10d, (%r15,%r11,4)
.Ltmp10794:
	incq	%r11
.Ltmp10795:
	.loc	12 37 12 is_stmt 1
	testq	%r14, %r14
	je	.LBB36_140
.LBB36_115:
.Ltmp10796:
	.loc	7 1714 9
	cmpq	$32, %rax
.Ltmp10797:
	.loc	6 180 28
	je	.LBB36_140
.Ltmp10798:
	.loc	1 1263 21
	cmpq	56(%rsp), %r11
	je	.LBB36_127
	leaq	(%r11,%r11,2), %r8
	movl	4(%rcx,%r8,4), %edi
.Ltmp10799:
	.loc	1 1265 23
	addq	(%rsp), %rdi
.Ltmp10800:
	.loc	1 1266 12
	cmpq	%rbp, %rdi
	movl	$0, %r10d
	cmovaeq	%rbp, %r10
	subq	%r10, %rdi
.Ltmp10801:
	.loc	1 1273 42
	movq	%rdi, %r12
	imulq	%rbx, %r12
	addq	%r11, %r12
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_129
.Ltmp10802:
	.loc	1 1274 24 is_stmt 1
	cmpq	144(%rsp), %r11
	je	.LBB36_131
.Ltmp10803:
	.loc	1 0 0 is_stmt 0
	movl	(%rcx,%r8,4), %r8d
.Ltmp10804:
	vmovss	(%r13,%r12,4), %xmm0
.Ltmp10805:
	.loc	1 1274 24
	movl	(%r15,%r11,4), %r10d
	testq	%r10, %r10
	je	.LBB36_122
.Ltmp10806:
	.loc	1 1278 24 is_stmt 1
	cmpq	%r9, %r11
	jae	.LBB36_133
	vmovss	(%rdx,%r11,4), %xmm1
.Ltmp10807:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB36_122
.Ltmp10808:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB36_122:
.Ltmp10809:
	.loc	1 1280 9 is_stmt 1
	cmpq	%r9, %r11
	je	.LBB36_136
	vmovss	%xmm0, (%rdx,%r11,4)
	.loc	1 1281 24
	incq	%r10
	cmpq	%r8, %r10
.Ltmp10810:
	.loc	1 1282 23
	jne	.LBB36_124
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 1408(%rsp,%rax)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%r13,%r12,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp10811:
	.p2align	4
.LBB36_149:
.Ltmp10812:
	.loc	1 1291 65 is_stmt 1
	movq	%rdi, %r12
	imulq	%rbx, %r12
	addq	%r11, %r12
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_150
.Ltmp10813:
	.loc	1 903 8 is_stmt 1
	vminss	(%r13,%r12,4), %xmm0, %xmm0
.Ltmp10814:
	.loc	1 1292 17
	vmovss	%xmm0, (%r13,%r12,4)
	.loc	1 1293 20
	testq	%rdi, %rdi
	cmoveq	%rbp, %rdi
	.loc	1 1296 17
	decq	%rdi
.Ltmp10815:
	.loc	10 1916 50
	decq	%r8
.Ltmp10816:
	.loc	3 900 12
	jne	.LBB36_149
	jmp	.LBB36_153
.Ltmp10817:
	.loc	3 0 12 is_stmt 0
.Ltmp10818:
	.p2align	4
.LBB36_124:
	movq	624(%rsp), %rdi
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%rdi), %r12
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_125
	vmovss	(%r13,%r12,4), %xmm1
.Ltmp10819:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp10820:
	.loc	1 1282 9
	vmovss	%xmm0, 1408(%rsp,%rax)
	jmp	.LBB36_139
.Ltmp10821:
	.loc	1 0 9 is_stmt 0
.Ltmp10822:
	.p2align	4
.LBB36_140:
	vmovss	%xmm10, 248(%rsp)
.Ltmp10823:
	.loc	52 51 9 is_stmt 1
	vmovss	1408(%rsp), %xmm9
.Ltmp10824:
	.loc	52 71 9
	vmulss	.LCPI36_3(%rip), %xmm9, %xmm0
.Ltmp10825:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp10826:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm10
	movq	936(%rsp), %r13
.Ltmp10827:
	.loc	1 1412 26
	movq	192(%r13), %rsi
	movq	288(%rsp), %r14
.Ltmp10828:
	.loc	12 37 12
	testq	%r14, %r14
	movq	680(%rsp), %rbx
	movl	192(%rsp), %r15d
	je	.LBB36_155
.Ltmp10829:
	.loc	12 0 12 is_stmt 0
	movq	320(%r13), %r10
.Ltmp10830:
	.loc	1 1403 42 is_stmt 1
	testq	%r10, %r10
	je	.LBB36_147
	.loc	1 0 42 is_stmt 0
	movq	312(%r13), %r11
	.loc	1 1403 42
	movl	8(%r11), %r8d
	.loc	1 1403 28
	addq	(%rsp), %r8
.Ltmp10831:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %r8
	movl	$0, %eax
	cmovaeq	%rbp, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%r14, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	184(%r13), %r9
	.loc	1 1407 25
	vmovss	(%r9,%r8,4), %xmm9
	.loc	1 1407 13
	vmovss	%xmm9, 1408(%rsp)
.Ltmp10832:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %r14
	je	.LBB36_155
.Ltmp10833:
	.loc	1 1403 42
	cmpq	$1, %r10
	je	.LBB36_145
	movl	20(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10834:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	vmovss	4(%r9,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1412(%rsp)
.Ltmp10835:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %r14
	je	.LBB36_155
.Ltmp10836:
	.loc	1 1403 42
	cmpq	$2, %r10
	je	.LBB36_196
	movl	32(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10837:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	vmovss	8(%r9,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1416(%rsp)
.Ltmp10838:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %r14
	je	.LBB36_155
.Ltmp10839:
	.loc	1 1403 42
	cmpq	$3, %r10
	je	.LBB36_200
	movl	44(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10840:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	vmovss	12(%r9,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1420(%rsp)
.Ltmp10841:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %r14
	je	.LBB36_155
.Ltmp10842:
	.loc	1 1403 42
	cmpq	$4, %r10
	je	.LBB36_204
	movl	56(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10843:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	vmovss	16(%r9,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1424(%rsp)
.Ltmp10844:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %r14
	je	.LBB36_155
.Ltmp10845:
	.loc	1 1403 42
	cmpq	$5, %r10
	je	.LBB36_208
	movl	68(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10846:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	vmovss	20(%r9,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1428(%rsp)
.Ltmp10847:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %r14
	je	.LBB36_155
.Ltmp10848:
	.loc	1 1403 42
	cmpq	$6, %r10
	je	.LBB36_212
	movl	80(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10849:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	vmovss	24(%r9,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1432(%rsp)
.Ltmp10850:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %r14
	je	.LBB36_155
.Ltmp10851:
	.loc	1 1403 42
	cmpq	$7, %r10
	je	.LBB36_216
	movl	92(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10852:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%r14, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	vmovss	28(%r9,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 1436(%rsp)
.Ltmp10853:
	.loc	1 0 13
.Ltmp10854:
	.p2align	4
.LBB36_155:
	.loc	52 61 9 is_stmt 1
	vaddss	%xmm8, %xmm10, %xmm0
.Ltmp10855:
	.loc	52 66 9
	vsubss	%xmm9, %xmm0, %xmm8
	movq	360(%rsp), %rdi
.Ltmp10856:
	.loc	5 580 12
	cmpq	%rsi, %rdi
	ja	.LBB36_190
.Ltmp10857:
	.loc	5 0 12 is_stmt 0
	movl	416(%rsp), %r12d
.Ltmp10858:
	.loc	52 56 9 is_stmt 1
	je	.LBB36_112
.Ltmp10859:
	.loc	1 1412 0
	movq	184(%r13), %rax
.Ltmp10860:
	.loc	52 56 9
	vmovss	%xmm10, (%rax,%rdi,4)
.Ltmp10861:
	.loc	52 76 9
	vdivss	240(%rsp), %xmm8, %xmm0
.Ltmp10862:
	.loc	52 66 9
	vsubss	%xmm0, %xmm2, %xmm0
.Ltmp10863:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm15, %xmm0, %xmm1
.Ltmp10864:
	.loc	52 92 9 is_stmt 1
	vmulss	368(%rsp), %xmm1, %xmm1
	vaddss	%xmm1, %xmm15, %xmm1
.Ltmp10865:
	.loc	52 103 24
	vbroadcastss	.LCPI36_0(%rip), %xmm10
.Ltmp10866:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp10867:
	.loc	52 103 24
	vandps	%xmm0, %xmm10, %xmm1
.Ltmp10868:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm1, %xmm1
	vandps	%xmm0, %xmm1, %xmm15
.Ltmp10869:
	.loc	1 1417 5
	vmovss	%xmm15, 788(%rsp)
.Ltmp10870:
	.loc	1 1420 28
	movq	160(%r13), %rdx
	.loc	1 1420 44 is_stmt 0
	imulq	800(%rsp), %r14
.Ltmp10871:
	.loc	5 568 12 is_stmt 1
	cmpq	%rdx, %r14
	movq	208(%rsp), %r8
	movq	160(%rsp), %rdi
	ja	.LBB36_192
.Ltmp10872:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_107
	vsubss	%xmm15, %xmm2, %xmm0
.Ltmp10873:
	.loc	1 1420 0 is_stmt 1
	movq	152(%r13), %rax
.Ltmp10874:
	.loc	52 51 9
	vmovss	(%rax,%r14,4), %xmm1
.Ltmp10875:
	.loc	52 56 9
	vmovss	%xmm6, (%rax,%r14,4)
.Ltmp10876:
	.loc	52 71 9
	vmulss	%xmm1, %xmm0, %xmm0
.Ltmp10877:
	.loc	41 1244 18
	vmovd	%xmm1, %eax
.Ltmp10878:
	.loc	52 161 24
	andl	424(%rsp), %eax
.Ltmp10879:
	.loc	41 1244 18
	vmovd	%xmm0, %ecx
.Ltmp10880:
	.loc	52 161 44
	andl	%r15d, %ecx
	.loc	52 161 24 is_stmt 0
	orl	%eax, %ecx
.Ltmp10881:
	.loc	52 56 9 is_stmt 1
	movl	%ecx, (%rbx,%rdi,4)
.Ltmp10882:
	.loc	5 568 12
	cmpq	296(%rsp), %rdi
	ja	.LBB36_254
.Ltmp10883:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_107
	movl	84(%rsp), %eax
	andl	%r12d, %eax
	movl	696(%rsp), %ecx
	orl	%eax, %ecx
	vmovd	%ecx, %xmm1
.Ltmp10884:
	.loc	52 124 14 is_stmt 1
	vucomiss	48(%rsp), %xmm1
	vmovaps	%xmm2, %xmm0
.Ltmp10885:
	.loc	52 161 24
	jbe	.LBB36_163
.Ltmp10886:
	.loc	52 0 24 is_stmt 0
	vmovss	48(%rsp), %xmm0
.Ltmp10887:
	.loc	52 76 9 is_stmt 1
	vdivss	%xmm1, %xmm0, %xmm0
.Ltmp10888:
.LBB36_163:
	.loc	1 0 0 is_stmt 0
	movq	528(%r13), %rcx
.Ltmp10889:
	.loc	1 1392 25 is_stmt 1
	movq	376(%r13), %rsi
	.loc	1 1392 45 is_stmt 0
	movq	%rcx, %rdx
	imulq	(%rsp), %rdx
.Ltmp10890:
	.loc	5 580 12 is_stmt 1
	cmpq	%rsi, %rdx
	ja	.LBB36_229
.Ltmp10891:
	.loc	5 0 12 is_stmt 0
	je	.LBB36_112
	movq	928(%rsp), %rax
	vmovss	(%rax,%rdi,4), %xmm6
.Ltmp10892:
	.loc	1 1392 0 is_stmt 1
	movq	368(%r13), %rax
.Ltmp10893:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%rdx,4)
.Ltmp10894:
	.loc	1 1259 17
	movq	528(%r13), %r14
.Ltmp10895:
	.loc	12 37 12
	testq	%r14, %r14
	je	.LBB36_180
.Ltmp10896:
	.loc	12 0 12 is_stmt 0
	movq	%rdx, 360(%rsp)
	movq	%rcx, 288(%rsp)
	movq	520(%r13), %rax
	movq	%rax, 56(%rsp)
	movq	(%rsp), %rax
	leaq	1(%rax), %rdx
	cmpq	%rbp, %rdx
	movq	%rbp, %rax
	movl	$0, %ecx
	cmovbq	%rcx, %rax
	movq	512(%r13), %rcx
	subq	%rax, %rdx
	movq	368(%r13), %rbx
	movq	376(%r13), %rsi
	movq	456(%r13), %rax
	movq	%rax, 144(%rsp)
	movq	448(%r13), %r15
	movq	424(%r13), %r9
	movq	416(%r13), %r13
	imulq	%r14, %rdx
	movq	%rdx, 624(%rsp)
	movq	%r14, %rdx
	xorl	%r11d, %r11d
	xorl	%eax, %eax
	jmp	.LBB36_167
	.p2align	4
.LBB36_228:
	xorl	%r10d, %r10d
.LBB36_178:
	decq	%rdx
	addq	$4, %rax
.Ltmp10897:
	movl	%r10d, (%r15,%r11,4)
.Ltmp10898:
	incq	%r11
.Ltmp10899:
	.loc	12 37 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB36_179
.LBB36_167:
.Ltmp10900:
	.loc	7 1714 9
	cmpq	$32, %rax
.Ltmp10901:
	.loc	6 180 28
	je	.LBB36_179
.Ltmp10902:
	.loc	1 1263 21
	cmpq	56(%rsp), %r11
	je	.LBB36_127
	leaq	(%r11,%r11,2), %r8
	movl	4(%rcx,%r8,4), %edi
.Ltmp10903:
	.loc	1 1265 23
	addq	(%rsp), %rdi
.Ltmp10904:
	.loc	1 1266 12
	cmpq	%rbp, %rdi
	movl	$0, %r10d
	cmovaeq	%rbp, %r10
	subq	%r10, %rdi
.Ltmp10905:
	.loc	1 1273 42
	movq	%rdi, %r12
	imulq	%r14, %r12
	addq	%r11, %r12
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_129
.Ltmp10906:
	.loc	1 1274 24 is_stmt 1
	cmpq	144(%rsp), %r11
	je	.LBB36_131
.Ltmp10907:
	.loc	1 0 0 is_stmt 0
	movl	(%rcx,%r8,4), %r8d
.Ltmp10908:
	vmovss	(%rbx,%r12,4), %xmm0
.Ltmp10909:
	.loc	1 1274 24
	movl	(%r15,%r11,4), %r10d
	testq	%r10, %r10
	je	.LBB36_174
.Ltmp10910:
	.loc	1 1278 24 is_stmt 1
	cmpq	%r9, %r11
	jae	.LBB36_133
	vmovss	(%r13,%r11,4), %xmm1
.Ltmp10911:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB36_174
.Ltmp10912:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB36_174:
.Ltmp10913:
	.loc	1 1280 9 is_stmt 1
	cmpq	%r9, %r11
	je	.LBB36_136
	vmovss	%xmm0, (%r13,%r11,4)
	.loc	1 1281 24
	incq	%r10
	cmpq	%r8, %r10
.Ltmp10914:
	.loc	1 1282 23
	jne	.LBB36_176
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 1408(%rsp,%rax)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rbx,%r12,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp10915:
	.p2align	4
.LBB36_226:
.Ltmp10916:
	.loc	1 1291 65 is_stmt 1
	movq	%rdi, %r12
	imulq	%r14, %r12
	addq	%r11, %r12
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_150
.Ltmp10917:
	.loc	1 903 8 is_stmt 1
	vminss	(%rbx,%r12,4), %xmm0, %xmm0
.Ltmp10918:
	.loc	1 1292 17
	vmovss	%xmm0, (%rbx,%r12,4)
	.loc	1 1293 20
	testq	%rdi, %rdi
	cmoveq	%rbp, %rdi
	.loc	1 1296 17
	decq	%rdi
.Ltmp10919:
	.loc	10 1916 50
	decq	%r8
.Ltmp10920:
	.loc	3 900 12
	jne	.LBB36_226
	jmp	.LBB36_228
.Ltmp10921:
	.loc	3 0 12 is_stmt 0
.Ltmp10922:
	.p2align	4
.LBB36_176:
	movq	624(%rsp), %rdi
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%rdi), %r12
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB36_125
	vmovss	(%rbx,%r12,4), %xmm1
.Ltmp10923:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp10924:
	.loc	1 1282 9
	vmovss	%xmm0, 1408(%rsp,%rax)
	jmp	.LBB36_178
.Ltmp10925:
	.loc	1 0 9 is_stmt 0
.Ltmp10926:
	.p2align	4
.LBB36_179:
	.loc	52 51 9 is_stmt 1
	vmovss	1408(%rsp), %xmm9
	movq	936(%rsp), %r13
	movq	680(%rsp), %rbx
	movl	416(%rsp), %r12d
	movl	192(%rsp), %r15d
	movq	208(%rsp), %r8
	movq	160(%rsp), %rdi
	movq	288(%rsp), %rcx
	movq	360(%rsp), %rdx
.Ltmp10927:
.LBB36_180:
	.loc	52 71 9
	vmulss	.LCPI36_3(%rip), %xmm9, %xmm0
.Ltmp10928:
	.loc	41 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp10929:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm0, %xmm0
.Ltmp10930:
	.loc	1 1412 26
	movq	392(%r13), %rsi
.Ltmp10931:
	.loc	12 37 12
	testq	%rcx, %rcx
	movq	960(%rsp), %r14
	je	.LBB36_185
.Ltmp10932:
	.loc	12 0 12 is_stmt 0
	movq	520(%r13), %r10
.Ltmp10933:
	.loc	1 1403 42 is_stmt 1
	testq	%r10, %r10
	je	.LBB36_147
	.loc	1 0 42 is_stmt 0
	movq	512(%r13), %r11
	.loc	1 1403 42
	movl	8(%r11), %r8d
	.loc	1 1403 28
	addq	(%rsp), %r8
.Ltmp10934:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %r8
	movl	$0, %eax
	cmovaeq	%rbp, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%rcx, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	384(%r13), %r9
	.loc	1 1407 25
	vmovss	(%r9,%r8,4), %xmm9
	.loc	1 1407 13
	vmovss	%xmm9, 1408(%rsp)
.Ltmp10935:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %rcx
	jne	.LBB36_232
.Ltmp10936:
	.loc	12 0 12 is_stmt 0
	movq	208(%rsp), %r8
	movq	160(%rsp), %rdi
	jmp	.LBB36_185
	.p2align	4
.LBB36_232:
.Ltmp10937:
	.loc	1 1403 42 is_stmt 1
	cmpq	$1, %r10
	je	.LBB36_145
	.loc	1 0 42 is_stmt 0
	movq	%r14, %r15
	movq	%rdx, %r14
	movq	%rcx, %rdx
	.loc	1 1403 42
	movl	20(%r11), %eax
	.loc	1 1403 28
	addq	(%rsp), %rax
.Ltmp10938:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	%rdx, %rcx
	.loc	1 1407 25
	vmovss	4(%r9,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1412(%rsp)
.Ltmp10939:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %rdx
	je	.LBB36_235
.Ltmp10940:
	.loc	1 1403 42
	cmpq	$2, %r10
	je	.LBB36_196
	movl	32(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10941:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	%rdx, %rcx
	.loc	1 1407 25
	vmovss	8(%r9,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1416(%rsp)
.Ltmp10942:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %rdx
	je	.LBB36_235
.Ltmp10943:
	.loc	1 1403 42
	cmpq	$3, %r10
	je	.LBB36_200
	movl	44(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10944:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	%rdx, %rcx
	.loc	1 1407 25
	vmovss	12(%r9,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1420(%rsp)
.Ltmp10945:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %rdx
	je	.LBB36_235
.Ltmp10946:
	.loc	1 1403 42
	cmpq	$4, %r10
	je	.LBB36_204
	movl	56(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10947:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	%rdx, %rcx
	.loc	1 1407 25
	vmovss	16(%r9,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1424(%rsp)
.Ltmp10948:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %rdx
	je	.LBB36_235
.Ltmp10949:
	.loc	1 1403 42
	cmpq	$5, %r10
	je	.LBB36_208
	movl	68(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10950:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	%rdx, %rcx
	.loc	1 1407 25
	vmovss	20(%r9,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1428(%rsp)
.Ltmp10951:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %rdx
	je	.LBB36_235
.Ltmp10952:
	.loc	1 1403 42
	cmpq	$6, %r10
	je	.LBB36_212
	movl	80(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10953:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	%rdx, %rcx
	.loc	1 1407 25
	vmovss	24(%r9,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1432(%rsp)
.Ltmp10954:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %rdx
	je	.LBB36_235
.Ltmp10955:
	.loc	1 1403 42
	cmpq	$7, %r10
	je	.LBB36_216
	movl	92(%r11), %eax
	.loc	1 1403 28 is_stmt 0
	addq	(%rsp), %rax
.Ltmp10956:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdx, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB36_219
	.loc	1 0 25
	movq	%rdx, %rcx
	.loc	1 1407 25
	vmovss	28(%r9,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 1436(%rsp)
.Ltmp10957:
	.loc	1 0 13
.Ltmp10958:
	.p2align	4
.LBB36_235:
	movq	208(%rsp), %r8
	movq	160(%rsp), %rdi
	movq	%r14, %rdx
	movq	%r15, %r14
	movl	192(%rsp), %r15d
.LBB36_185:
.Ltmp10959:
	.loc	52 61 9 is_stmt 1
	vaddss	%xmm7, %xmm0, %xmm1
.Ltmp10960:
	.loc	52 66 9
	vsubss	%xmm9, %xmm1, %xmm7
.Ltmp10961:
	.loc	5 580 12
	cmpq	%rsi, %rdx
	ja	.LBB36_663
.Ltmp10962:
	.loc	52 56 9
	je	.LBB36_112
.Ltmp10963:
	.loc	1 1412 0
	movq	384(%r13), %rax
.Ltmp10964:
	.loc	52 56 9
	vmovss	%xmm0, (%rax,%rdx,4)
.Ltmp10965:
	.loc	52 76 9
	vdivss	352(%rsp), %xmm7, %xmm0
.Ltmp10966:
	.loc	52 66 9
	vsubss	%xmm0, %xmm2, %xmm0
.Ltmp10967:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm3, %xmm0, %xmm1
.Ltmp10968:
	.loc	52 92 9 is_stmt 1
	vmulss	32(%rsp), %xmm1, %xmm1
	vaddss	%xmm1, %xmm3, %xmm1
.Ltmp10969:
	.loc	52 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp10970:
	.loc	52 103 24
	vandps	%xmm0, %xmm10, %xmm1
.Ltmp10971:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm1, %xmm1
	vandps	%xmm0, %xmm1, %xmm3
.Ltmp10972:
	.loc	1 1417 5
	vmovss	%xmm3, 576(%rsp)
.Ltmp10973:
	.loc	1 1420 28
	movq	360(%r13), %rdx
	.loc	1 1420 44 is_stmt 0
	imulq	800(%rsp), %rcx
.Ltmp10974:
	.loc	5 568 12 is_stmt 1
	cmpq	%rdx, %rcx
	movq	200(%rsp), %rsi
	ja	.LBB36_231
.Ltmp10975:
	.loc	52 51 9
	je	.LBB36_107
.Ltmp10976:
	.loc	1 0 0 is_stmt 0
	incq	%rsi
	vxorps	%xmm2, %xmm2, %xmm2
	vmovss	672(%rsp), %xmm0
.Ltmp10977:
	vmaxss	%xmm2, %xmm0, %xmm10
	vcmpltss	%xmm0, %xmm2, %xmm0
	vandps	%xmm0, %xmm12, %xmm12
	vmovss	256(%rsp), %xmm0
	vmaxss	%xmm2, %xmm0, %xmm9
	vcmpltss	%xmm0, %xmm2, %xmm0
	vandps	%xmm0, %xmm11, %xmm11
.Ltmp10978:
	vmaxss	%xmm2, %xmm13, %xmm1
	vcmpltss	%xmm13, %xmm2, %xmm0
	vandps	%xmm0, %xmm14, %xmm14
	vmaxss	%xmm2, %xmm5, %xmm0
	vcmpltss	%xmm5, %xmm2, %xmm2
	vandps	%xmm4, %xmm2, %xmm4
.Ltmp10979:
	vmovss	.LCPI36_2(%rip), %xmm2
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp10980:
	.loc	1 1420 0 is_stmt 1
	movq	352(%r13), %rax
.Ltmp10981:
	.loc	52 51 9
	vmovss	(%rax,%rcx,4), %xmm5
.Ltmp10982:
	.loc	52 56 9
	vmovss	%xmm6, (%rax,%rcx,4)
	vxorps	%xmm6, %xmm6, %xmm6
.Ltmp10983:
	.loc	52 71 9
	vmulss	%xmm5, %xmm2, %xmm2
.Ltmp10984:
	.loc	41 1244 18
	vmovd	%xmm5, %eax
.Ltmp10985:
	.loc	52 161 24
	andl	424(%rsp), %eax
.Ltmp10986:
	.loc	41 1244 18
	vmovd	%xmm2, %ecx
.Ltmp10987:
	.loc	52 161 44
	andl	%r15d, %ecx
	.loc	52 161 24 is_stmt 0
	orl	%eax, %ecx
	movq	928(%rsp), %rax
.Ltmp10988:
	.loc	52 56 9 is_stmt 1
	movl	%ecx, (%rax,%rdi,4)
	movq	800(%rsp), %rax
.Ltmp10989:
	.loc	1 1881 13
	incq	%rax
	.loc	1 1882 16
	cmpq	344(%rsp), %rax
	movl	$0, %edx
	cmoveq	%rdx, %rax
	movq	%rax, 800(%rsp)
	movq	(%rsp), %rax
	.loc	1 1885 13
	incq	%rax
	.loc	1 1886 16
	cmpq	%rbp, %rax
	movl	$0, %ecx
	movq	%rcx, 184(%rsp)
	cmoveq	%rdx, %rax
	movq	%rax, (%rsp)
.Ltmp10990:
	.loc	10 1916 50
	cmpq	608(%rsp), %rsi
	movq	40(%rsp), %rcx
.Ltmp10991:
	.loc	3 900 12
	jne	.LBB36_93
.Ltmp10992:
	.loc	1 1411 5
	vmovss	%xmm8, 792(%rsp)
.Ltmp10993:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp10994:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm10, 768(%rsp)
	vmovss	248(%rsp), %xmm2
.Ltmp10995:
	.loc	1 853 0
	vmovss	%xmm2, 756(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm12, 764(%rsp)
.Ltmp10996:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm9, 784(%rsp)
	vmovss	368(%rsp), %xmm2
.Ltmp10997:
	.loc	1 853 0
	vmovss	%xmm2, 772(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm11, 780(%rsp)
.Ltmp10998:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm1, 556(%rsp)
	vmovss	48(%rsp), %xmm1
.Ltmp10999:
	.loc	1 853 0
	vmovss	%xmm1, 544(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm14, 552(%rsp)
.Ltmp11000:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm0, 572(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp11001:
	.loc	1 853 0
	vmovss	%xmm0, 560(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm4, 568(%rsp)
	movq	296(%rsp), %r9
	movq	928(%rsp), %rbp
	movq	272(%rsp), %rsi
	movl	$32, %edi
	jmp	.LBB36_75
.Ltmp11002:
.LBB36_256:
	movq	680(%rsp), %rbx
.LBB36_257:
	leaq	1224(%rsp), %rdi
	movq	1192(%rsp), %rsi
.Ltmp11003:
	.loc	1 1892 14 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	leaq	1316(%rsp), %rdi
	jmp	.LBB36_258
.Ltmp11004:
.LBB36_71:
	.loc	1 0 14 is_stmt 0
	movq	680(%rsp), %rbx
.LBB36_72:
.Ltmp11005:
	.loc	1 1892 5 is_stmt 1
	vmovups	708(%rsp), %ymm0
	vmovups	740(%rsp), %ymm1
	vmovups	768(%rsp), %ymm2
	vmovups	%ymm2, 892(%rsp)
	vmovups	%ymm1, 864(%rsp)
	vmovups	%ymm0, 832(%rsp)
	leaq	832(%rsp), %rdi
	movq	1192(%rsp), %rsi
	.loc	1 1892 14 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	.loc	1 1893 5 is_stmt 1
	vmovups	496(%rsp), %ymm0
	vmovups	528(%rsp), %ymm1
	vmovups	556(%rsp), %ymm2
	vmovups	%ymm2, 892(%rsp)
	vmovups	%ymm1, 864(%rsp)
	vmovups	%ymm0, 832(%rsp)
	leaq	832(%rsp), %rdi
.Ltmp11006:
.LBB36_258:
	.loc	1 0 5 is_stmt 0
	movq	1184(%rsp), %rsi
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	800(%rsp), %rax
	movl	%eax, 560(%r13)
	movq	(%rsp), %rax
.LBB36_584:
	movl	%eax, 564(%r13)
.Ltmp11007:
	.loc	1 2260 35 is_stmt 1
	cmpb	$0, 1532(%rsp)
	je	.LBB36_585
	.loc	1 0 35 is_stmt 0
	movq	1192(%rsp), %rdi
	.loc	1 2261 26 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2261 16 is_stmt 0
	testb	%al, %al
	je	.LBB36_585
	.loc	1 0 16
	movq	1184(%rsp), %rdi
	.loc	1 2262 27 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2262 16 is_stmt 0
	testb	%al, %al
	je	.LBB36_585
	.loc	1 0 16
	movq	232(%rsp), %rsi
	cmpq	%r14, %rsi
	movq	296(%rsp), %r9
.Ltmp11008:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB36_687
.Ltmp11009:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rax
	movq	%rbx, %rcx
	.p2align	4
.LBB36_641:
.Ltmp11010:
	.loc	15 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB36_646
.Ltmp11011:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp11012:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%esi, %esi
	xorl	%r8d, %r8d
.Ltmp11013:
	.loc	16 0 18 is_stmt 0
.Ltmp11014:
	.p2align	4
.LBB36_643:
	.loc	17 134 13 is_stmt 1
	orl	(%rcx,%r8), %esi
.Ltmp11015:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp11016:
	.loc	6 180 28
	jne	.LBB36_643
.Ltmp11017:
	.loc	18 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp11018:
	.loc	19 2054 74
	subq	%rdx, %rax
.Ltmp11019:
	.loc	17 136 12
	testl	%esi, %esi
	je	.LBB36_641
.Ltmp11020:
	.loc	17 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	1200(%rsp), %rcx
	.loc	1 2260 35 is_stmt 1
	jmp	.LBB36_586
.LBB36_585:
	.loc	1 0 35 is_stmt 0
	xorl	%eax, %eax
	movq	1200(%rsp), %rcx
	movq	296(%rsp), %r9
.LBB36_586:
	.loc	1 2260 9 is_stmt 1
	movb	%al, 780(%r13)
	.loc	1 2265 30
	movzbl	96(%r13), %eax
	.loc	1 2265 9 is_stmt 0
	movb	%al, 781(%r13)
	.loc	1 2266 21 is_stmt 1
	movq	16(%rcx), %rax
	movq	%rax, 1632(%rsp)
	vmovups	(%rcx), %xmm0
	vmovaps	%xmm0, 1616(%rsp)
.Ltmp11021:
	.loc	1 2267 20
	movl	72(%r13), %edi
.Ltmp11022:
	.loc	1 2273 64
	movq	104(%r13), %r12
	movq	112(%r13), %r15
	movq	120(%r13), %rsi
	movq	128(%r13), %r8
.Ltmp11023:
	.loc	19 2155 12
	testq	%r14, %r14
	je	.LBB36_590
.Ltmp11024:
	.loc	19 0 12 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI36_0(%rip), %xmm0
	vmovss	.LCPI36_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB36_588:
.Ltmp11025:
	.loc	52 103 24 is_stmt 1
	vmovss	(%rbx,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp11026:
	.loc	52 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp11027:
	.loc	52 139 9
	cmovbel	%ecx, %eax
.Ltmp11028:
	.loc	19 2155 12
	incq	%rdx
	cmpq	%rdx, %r14
	jne	.LBB36_588
.Ltmp11029:
	.loc	52 154 9
	cmpl	$-1, %eax
.Ltmp11030:
	.loc	17 208 8
	jne	.LBB36_653
.LBB36_590:
.Ltmp11031:
	.loc	19 2155 12
	testq	%r9, %r9
	je	.LBB36_636
.Ltmp11032:
	.loc	19 0 12 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI36_0(%rip), %xmm0
	vmovss	.LCPI36_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB36_592:
.Ltmp11033:
	.loc	52 103 24 is_stmt 1
	vmovss	(%rbp,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp11034:
	.loc	52 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp11035:
	.loc	52 139 9
	cmovbel	%ecx, %eax
.Ltmp11036:
	.loc	19 2155 12
	incq	%rdx
	cmpq	%rdx, %r9
	jne	.LBB36_592
.Ltmp11037:
	.loc	52 154 9
	cmpl	$-1, %eax
.Ltmp11038:
	.loc	17 208 34
	je	.LBB36_636
.Ltmp11039:
	.loc	19 2155 12
	testq	%r14, %r14
.Ltmp11040:
	.loc	19 2155 12 is_stmt 0
	je	.LBB36_595
.Ltmp11041:
.LBB36_653:
	.loc	19 0 12
	movl	$-1, %ecx
	xorl	%eax, %eax
	vbroadcastss	.LCPI36_0(%rip), %xmm0
	vmovss	.LCPI36_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB36_654:
.Ltmp11042:
	.loc	52 103 24 is_stmt 1
	vmovss	(%rbx,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp11043:
	.loc	52 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp11044:
	.loc	52 139 9
	cmovbel	%eax, %ecx
.Ltmp11045:
	.loc	19 2155 12
	incq	%rdx
	cmpq	%rdx, %r14
	jne	.LBB36_654
.Ltmp11046:
	.loc	17 185 12
	notl	%ecx
	xorl	%eax, %eax
	testl	$1065353216, %ecx
	setne	%al
.Ltmp11047:
	.loc	19 2155 12
	testq	%r9, %r9
	movq	%rsi, (%rsp)
	movq	%r8, 56(%rsp)
	jne	.LBB36_596
.Ltmp11048:
	.loc	19 0 12 is_stmt 0
	movl	%edi, 144(%rsp)
	.loc	17 211 5 is_stmt 1
	movl	%eax, 576(%r13)
	.loc	17 212 31
	movq	568(%r13), %rax
.Ltmp11049:
	.loc	4 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp11050:
	.loc	17 212 5
	movq	%rcx, 568(%r13)
.Ltmp11051:
	.loc	6 180 28
	jmp	.LBB36_659
.Ltmp11052:
.LBB36_414:
	.loc	1 1696 12
	testb	%dl, %dl
	je	.LBB36_415
	.loc	1 0 12 is_stmt 0
	leaq	1000(%rsp), %rdi
	leaq	136(%r13), %rsi
.Ltmp11053:
	.loc	1 1946 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	1092(%rsp), %rdi
.Ltmp11054:
	.loc	1 1947 25
	movq	%rbx, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp11055:
	.loc	1 1952 19
	movzbl	776(%r13), %eax
	movl	%eax, 288(%rsp)
.Ltmp11056:
	.loc	1 1953 21
	movzbl	777(%r13), %eax
	movl	%eax, 144(%rsp)
.Ltmp11057:
	.loc	1 1954 16
	movq	544(%r13), %r12
.Ltmp11058:
	.loc	1 1955 16
	movq	552(%r13), %r15
.Ltmp11059:
	.loc	1 1956 27
	movl	560(%r13), %eax
	movq	%rax, 1512(%rsp)
.Ltmp11060:
	.loc	1 1957 27
	movl	564(%r13), %eax
	movq	%rax, 816(%rsp)
	leaq	2648(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %rbx
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	leaq	1616(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	leaq	496(%rsp), %rdi
.Ltmp11061:
	.loc	1 1964 32
	leaq	136(%r13), %rsi
	movq	%r12, %rdx
	movq	%r15, 1600(%rsp)
	movq	%r15, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
.Ltmp11062:
	.loc	1 1965 33
	movq	544(%r13), %rdx
	movq	552(%r13), %rcx
	leaq	832(%rsp), %rdi
	leaq	336(%r13), %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
	movq	232(%rsp), %rcx
.Ltmp11063:
	.loc	8 446 20
	testq	%rcx, %rcx
	je	.LBB36_510
.Ltmp11064:
	.loc	8 0 20 is_stmt 0
	movl	288(%rsp), %esi
	movl	%esi, %eax
	negl	%eax
	movl	%eax, 648(%rsp)
	movl	144(%rsp), %r8d
.Ltmp11065:
	movl	%r8d, %eax
	negl	%eax
	movl	%eax, 360(%rsp)
.Ltmp11066:
	.loc	4 3756 21 is_stmt 1
	movq	%rcx, %rdx
	shrq	$5, %rdx
.Ltmp11067:
	.loc	4 3757 21
	movl	%ecx, %eax
	andl	$31, %eax
.Ltmp11068:
	.loc	4 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %rdx
	decl	%esi
	movl	%esi, 288(%rsp)
	decl	%r8d
	vmovss	1000(%rsp), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	1004(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	1008(%rsp), %xmm0
	vmovss	%xmm0, 996(%rsp)
	vmovss	1012(%rsp), %xmm0
	vmovss	%xmm0, 992(%rsp)
	vmovss	1016(%rsp), %xmm6
	vmovss	1020(%rsp), %xmm2
	vmovss	1024(%rsp), %xmm0
	vmovaps	%xmm0, 1552(%rsp)
	vmovss	1028(%rsp), %xmm10
	vmovss	1032(%rsp), %xmm0
	vmovss	%xmm0, 988(%rsp)
	vmovss	1036(%rsp), %xmm1
	vmovss	1040(%rsp), %xmm0
	vmovss	%xmm0, 984(%rsp)
	vmovss	1044(%rsp), %xmm0
	vmovss	%xmm0, 624(%rsp)
	vmovss	1092(%rsp), %xmm7
	vmovss	1096(%rsp), %xmm9
	vmovss	1100(%rsp), %xmm0
	vmovss	%xmm0, 980(%rsp)
	vmovss	1104(%rsp), %xmm13
	movl	572(%rsp), %r10d
	movl	908(%rsp), %r9d
.Ltmp11069:
	.loc	8 446 20
	movq	%r14, %rax
	negq	%rax
	movq	%rax, 1584(%rsp)
	movq	296(%rsp), %rax
	negq	%rax
	movq	%rax, 1576(%rsp)
	movl	$32, %esi
	movq	%rbp, 672(%rsp)
	movq	680(%rsp), %rbx
	movq	%rbx, 696(%rsp)
	movq	%rcx, %rax
	xorl	%r11d, %r11d
	vmovss	1108(%rsp), %xmm12
	vmovss	1112(%rsp), %xmm15
	vmovss	1116(%rsp), %xmm3
	vmovss	1120(%rsp), %xmm4
	vmovss	1124(%rsp), %xmm5
	vmovss	1128(%rsp), %xmm0
	vmovss	%xmm0, 976(%rsp)
	vmovss	1132(%rsp), %xmm0
	vmovss	%xmm0, 972(%rsp)
	vmovss	1136(%rsp), %xmm0
	vmovss	%xmm0, 800(%rsp)
	vmovss	568(%rsp), %xmm0
	vmovss	%xmm0, 692(%rsp)
	vmovss	904(%rsp), %xmm0
	vmovss	%xmm0, 688(%rsp)
	vmovss	1084(%rsp), %xmm8
	vmovss	1080(%rsp), %xmm11
	vmovss	1176(%rsp), %xmm0
	vmovss	1172(%rsp), %xmm14
.LBB36_514:
	.loc	8 0 20 is_stmt 0
	vmovss	%xmm5, 948(%rsp)
	vmovss	%xmm4, 1220(%rsp)
	vmovaps	%xmm3, 1488(%rsp)
	vmovss	%xmm15, 824(%rsp)
	vmovss	%xmm12, 952(%rsp)
	vmovss	%xmm8, 1208(%rsp)
	vmovss	%xmm13, 184(%rsp)
	vmovss	%xmm9, (%rsp)
	vmovss	%xmm11, 40(%rsp)
	vmovss	%xmm0, 1212(%rsp)
	vmovss	%xmm14, 1216(%rsp)
	vmovss	%xmm7, 104(%rsp)
	movq	%rdx, 1536(%rsp)
.Ltmp11070:
	.loc	4 2584 13 is_stmt 1
	cmpq	$32, %rax
	movl	$32, %edx
	movq	%rax, 1592(%rsp)
	cmovbq	%rax, %rdx
	cmpq	$1, %rdx
	movq	%rdx, 1608(%rsp)
	movq	%rdx, %rax
	adcq	$0, %rax
.Ltmp11071:
	.loc	10 1916 50
	movq	%rcx, %rdx
	subq	%r11, %rdx
.Ltmp11072:
	.loc	10 1078 5
	cmpq	$32, %rdx
	cmovaeq	%rsi, %rdx
	movq	%rdx, 1544(%rsp)
.Ltmp11073:
	.loc	10 1916 50
	cmpq	%r11, %rcx
.Ltmp11074:
	.loc	3 900 12
	jne	.LBB36_516
	.loc	3 0 12 is_stmt 0
	vmovss	112(%rsp), %xmm0
	vmovss	%xmm0, 144(%rsp)
	vmovss	56(%rsp), %xmm14
	vmovss	996(%rsp), %xmm0
	vmovss	992(%rsp), %xmm5
	vmovaps	%xmm6, %xmm12
	vmovaps	%xmm2, %xmm11
	vmovaps	1552(%rsp), %xmm15
	vmovaps	%xmm10, %xmm9
	vmovss	988(%rsp), %xmm10
	vmovaps	%xmm1, %xmm13
	vmovss	984(%rsp), %xmm7
	vmovss	624(%rsp), %xmm3
	.loc	3 900 12
	jmp	.LBB36_523
.Ltmp11075:
.LBB36_516:
	.loc	3 0 12
	vmovss	584(%r13), %xmm0
	vmovss	%xmm0, 84(%rsp)
	vmovss	588(%r13), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	592(%r13), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	596(%r13), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	600(%r13), %xmm0
	vmovss	%xmm0, 192(%rsp)
	vmovss	604(%r13), %xmm0
	vmovss	%xmm0, 416(%rsp)
	movq	1584(%rsp), %rcx
.Ltmp11076:
	.loc	5 568 12 is_stmt 1
	leaq	(%rcx,%r11), %rcx
	movl	$0, %edx
	vmovss	984(%rsp), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	%xmm1, 48(%rsp)
	vmovss	988(%rsp), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	%xmm10, 32(%rsp)
	vmovaps	1552(%rsp), %xmm4
	vmovaps	%xmm2, 208(%rsp)
	vmovss	992(%rsp), %xmm2
	vmovss	996(%rsp), %xmm8
	vmovss	56(%rsp), %xmm1
	vmovss	608(%r13), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	612(%r13), %xmm0
	vmovss	%xmm0, 600(%rsp)
	vmovss	616(%r13), %xmm0
	vmovss	%xmm0, 592(%rsp)
	vmovss	620(%r13), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	624(%r13), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	628(%r13), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	632(%r13), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	636(%r13), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	640(%r13), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	644(%r13), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	648(%r13), %xmm0
	vmovss	%xmm0, 272(%rsp)
	vmovss	652(%r13), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	656(%r13), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	660(%r13), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	664(%r13), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	668(%r13), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	672(%r13), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	676(%r13), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	680(%r13), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	684(%r13), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	688(%r13), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	692(%r13), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	696(%r13), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	700(%r13), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	704(%r13), %xmm0
	vmovss	%xmm0, 264(%rsp)
	vmovss	708(%r13), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	712(%r13), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	716(%r13), %xmm0
	vmovss	%xmm0, 448(%rsp)
	vmovss	720(%r13), %xmm0
	vmovss	%xmm0, 440(%rsp)
	vmovss	724(%r13), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	728(%r13), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	732(%r13), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	736(%r13), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	740(%r13), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	744(%r13), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	748(%r13), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	752(%r13), %xmm0
	vmovss	%xmm0, 140(%rsp)
	vmovss	756(%r13), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	760(%r13), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	764(%r13), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	768(%r13), %xmm0
	vmovss	%xmm0, 124(%rsp)
	vmovss	772(%r13), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	112(%rsp), %xmm7
	vmovss	%xmm7, 144(%rsp)
	.loc	5 0 12 is_stmt 0
.Ltmp11077:
	.p2align	4
.LBB36_517:
	vmovss	144(%rsp), %xmm7
	vmovaps	%xmm1, %xmm0
	vmovaps	%xmm2, %xmm12
	vmovaps	208(%rsp), %xmm15
	vmovss	32(%rsp), %xmm10
	vmovss	368(%rsp), %xmm13
	vmovss	48(%rsp), %xmm1
	vmovss	160(%rsp), %xmm2
	.loc	5 568 12 is_stmt 1
	leaq	(%r11,%rdx), %rdi
	cmpq	%r14, %rdi
	ja	.LBB36_520
.Ltmp11078:
	.loc	5 0 12 is_stmt 0
	vmovss	%xmm6, 608(%rsp)
	vmovss	%xmm2, 624(%rsp)
	.loc	1 1762 0 is_stmt 1
	movq	%rcx, %rsi
	addq	%rdx, %rsi
.Ltmp11079:
	.loc	52 51 9
	je	.LBB36_519
	.loc	52 0 9 is_stmt 0
	vmovaps	%xmm4, %xmm9
	vmovaps	%xmm8, %xmm11
	vmovaps	%xmm0, %xmm5
	vmovaps	%xmm7, %xmm14
	movq	696(%rsp), %rsi
	.loc	52 51 9
	vmovss	(%rsi,%rdx,4), %xmm0
	vbroadcastss	.LCPI36_0(%rip), %xmm2
.Ltmp11080:
	.loc	52 103 24 is_stmt 1
	vandps	%xmm2, %xmm15, %xmm2
	vmovaps	%xmm2, 160(%rsp)
.Ltmp11081:
	.loc	52 71 9
	vmulss	84(%rsp), %xmm0, %xmm2
	vxorps	%xmm7, %xmm7, %xmm7
.Ltmp11082:
	.loc	52 61 9
	vaddss	%xmm7, %xmm2, %xmm2
.Ltmp11083:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm0, %xmm3
.Ltmp11084:
	.loc	52 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp11085:
	.loc	52 71 9
	vmulss	256(%rsp), %xmm0, %xmm4
.Ltmp11086:
	.loc	52 61 9
	vaddss	%xmm7, %xmm4, %xmm4
	vmovss	%xmm0, 144(%rsp)
.Ltmp11087:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm0, %xmm6
.Ltmp11088:
	.loc	52 61 9
	vaddss	%xmm7, %xmm6, %xmm6
	vmovss	%xmm14, 56(%rsp)
.Ltmp11089:
	.loc	52 71 9
	vmulss	192(%rsp), %xmm14, %xmm8
.Ltmp11090:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11091:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm14, %xmm8
.Ltmp11092:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11093:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm14, %xmm8
.Ltmp11094:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11095:
	.loc	52 71 9
	vmulss	600(%rsp), %xmm14, %xmm8
.Ltmp11096:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovaps	%xmm5, %xmm14
.Ltmp11097:
	.loc	52 71 9
	vmulss	592(%rsp), %xmm5, %xmm8
.Ltmp11098:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11099:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm5, %xmm8
.Ltmp11100:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11101:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm5, %xmm8
.Ltmp11102:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11103:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm5, %xmm8
.Ltmp11104:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovaps	%xmm11, %xmm5
.Ltmp11105:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm11, %xmm8
.Ltmp11106:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11107:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm11, %xmm8
.Ltmp11108:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11109:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm11, %xmm8
.Ltmp11110:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11111:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm11, %xmm8
.Ltmp11112:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovaps	%xmm12, %xmm11
.Ltmp11113:
	.loc	52 71 9
	vmulss	272(%rsp), %xmm12, %xmm8
.Ltmp11114:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11115:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm12, %xmm8
.Ltmp11116:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11117:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm12, %xmm8
.Ltmp11118:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm0
.Ltmp11119:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm12, %xmm8
.Ltmp11120:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovss	608(%rsp), %xmm4
.Ltmp11121:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm4, %xmm8
.Ltmp11122:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11123:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm4, %xmm8
.Ltmp11124:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11125:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm4, %xmm8
.Ltmp11126:
	.loc	52 61 9
	vaddss	%xmm0, %xmm8, %xmm0
.Ltmp11127:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm4, %xmm8
.Ltmp11128:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11129:
	.loc	52 71 9
	vmulss	68(%rsp), %xmm15, %xmm8
.Ltmp11130:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11131:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm15, %xmm8
.Ltmp11132:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11133:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm15, %xmm8
.Ltmp11134:
	.loc	52 61 9
	vaddss	%xmm0, %xmm8, %xmm0
.Ltmp11135:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm15, %xmm8
.Ltmp11136:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11137:
	.loc	52 71 9
	vmulss	16(%rsp), %xmm9, %xmm8
.Ltmp11138:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11139:
	.loc	52 71 9
	vmulss	12(%rsp), %xmm9, %xmm8
.Ltmp11140:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11141:
	.loc	52 71 9
	vmulss	264(%rsp), %xmm9, %xmm8
.Ltmp11142:
	.loc	52 61 9
	vaddss	%xmm0, %xmm8, %xmm0
.Ltmp11143:
	.loc	52 71 9
	vmulss	384(%rsp), %xmm9, %xmm8
.Ltmp11144:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11145:
	.loc	52 71 9
	vmulss	72(%rsp), %xmm10, %xmm8
.Ltmp11146:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11147:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm10, %xmm8
.Ltmp11148:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11149:
	.loc	52 71 9
	vmulss	440(%rsp), %xmm10, %xmm8
.Ltmp11150:
	.loc	52 61 9
	vaddss	%xmm0, %xmm8, %xmm0
.Ltmp11151:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm10, %xmm8
.Ltmp11152:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11153:
	.loc	52 71 9
	vmulss	88(%rsp), %xmm13, %xmm8
.Ltmp11154:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11155:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm13, %xmm8
.Ltmp11156:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11157:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm13, %xmm8
.Ltmp11158:
	.loc	52 61 9
	vaddss	%xmm0, %xmm8, %xmm0
.Ltmp11159:
	.loc	52 71 9
	vmulss	20(%rsp), %xmm13, %xmm8
.Ltmp11160:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovaps	%xmm1, %xmm7
.Ltmp11161:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm1, %xmm8
.Ltmp11162:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11163:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm1, %xmm8
.Ltmp11164:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm1
.Ltmp11165:
	.loc	52 71 9
	vmulss	140(%rsp), %xmm7, %xmm8
.Ltmp11166:
	.loc	52 61 9
	vaddss	%xmm0, %xmm8, %xmm12
.Ltmp11167:
	.loc	52 71 9
	vmulss	136(%rsp), %xmm7, %xmm8
.Ltmp11168:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovss	624(%rsp), %xmm3
.Ltmp11169:
	.loc	52 71 9
	vmulss	132(%rsp), %xmm3, %xmm8
.Ltmp11170:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11171:
	.loc	52 71 9
	vmulss	128(%rsp), %xmm3, %xmm8
.Ltmp11172:
	.loc	52 61 9
	vaddss	%xmm1, %xmm8, %xmm0
.Ltmp11173:
	.loc	52 71 9
	vmulss	124(%rsp), %xmm3, %xmm8
.Ltmp11174:
	.loc	52 61 9
	vaddss	%xmm8, %xmm12, %xmm12
.Ltmp11175:
	.loc	52 71 9
	vmulss	432(%rsp), %xmm3, %xmm8
.Ltmp11176:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vbroadcastss	.LCPI36_0(%rip), %xmm8
.Ltmp11177:
	.loc	52 103 24
	vandps	%xmm2, %xmm8, %xmm2
	vmovaps	160(%rsp), %xmm1
.Ltmp11178:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11179:
	.loc	52 103 24
	vandps	%xmm0, %xmm8, %xmm2
.Ltmp11180:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11181:
	.loc	52 103 24
	vandps	%xmm8, %xmm12, %xmm2
.Ltmp11182:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11183:
	.loc	52 103 24
	vandps	%xmm6, %xmm8, %xmm2
.Ltmp11184:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11185:
	.loc	52 56 9
	vmovss	%xmm1, 2648(%rsp,%rdx,4)
.Ltmp11186:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm7, 160(%rsp)
	vmovss	%xmm13, 48(%rsp)
	vmovss	%xmm10, 368(%rsp)
	vmovss	%xmm9, 32(%rsp)
	vmovaps	%xmm4, 208(%rsp)
	vmovaps	%xmm11, %xmm6
	vmovaps	%xmm5, %xmm2
	vmovaps	%xmm14, %xmm8
	vmovaps	%xmm14, %xmm0
	vmovss	56(%rsp), %xmm14
	vmovaps	%xmm11, %xmm12
	vmovaps	%xmm4, %xmm11
	vmovaps	%xmm15, %xmm4
	vmovaps	%xmm14, %xmm1
.Ltmp11187:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp11188:
	.loc	3 900 12
	jne	.LBB36_517
.Ltmp11189:
.LBB36_523:
	.loc	3 0 12 is_stmt 0
	vmovss	%xmm11, 608(%rsp)
	vmovss	%xmm12, 1520(%rsp)
	vmovss	144(%rsp), %xmm1
	vmovss	%xmm1, 112(%rsp)
	vmovss	%xmm14, 56(%rsp)
	.loc	1 1765 5 is_stmt 1
	vmovss	%xmm14, 1004(%rsp)
	vmovss	%xmm0, 996(%rsp)
	vmovss	%xmm0, 1008(%rsp)
	vmovss	%xmm5, 992(%rsp)
	vmovss	%xmm5, 1012(%rsp)
	vmovss	%xmm12, 1016(%rsp)
	vmovss	%xmm11, 1020(%rsp)
	vmovaps	%xmm15, 1552(%rsp)
	vmovss	%xmm15, 1024(%rsp)
	vmovss	%xmm9, 1528(%rsp)
	vmovss	%xmm9, 1028(%rsp)
	vmovss	%xmm10, 988(%rsp)
	vmovss	%xmm10, 1032(%rsp)
	vmovss	%xmm13, 1524(%rsp)
	vmovss	%xmm13, 1036(%rsp)
	vmovss	%xmm7, 984(%rsp)
	vmovss	%xmm7, 1040(%rsp)
	vmovss	%xmm3, 624(%rsp)
	vmovss	%xmm3, 1044(%rsp)
.Ltmp11190:
	.loc	10 1916 50
	cmpq	%r11, 232(%rsp)
.Ltmp11191:
	.loc	3 900 12
	jne	.LBB36_525
	.loc	3 0 12 is_stmt 0
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 144(%rsp)
	vmovss	(%rsp), %xmm1
	vmovss	980(%rsp), %xmm14
	vmovss	184(%rsp), %xmm5
	vmovss	952(%rsp), %xmm12
	vmovss	824(%rsp), %xmm11
	vmovaps	1488(%rsp), %xmm3
	vmovss	1220(%rsp), %xmm15
	vmovss	948(%rsp), %xmm2
	vmovss	976(%rsp), %xmm10
	vmovss	972(%rsp), %xmm9
	vmovss	800(%rsp), %xmm7
	.loc	3 900 12
	jmp	.LBB36_529
.Ltmp11192:
.LBB36_525:
	.loc	3 0 12
	vmovss	584(%r13), %xmm0
	vmovss	%xmm0, 84(%rsp)
	vmovss	588(%r13), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	592(%r13), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	596(%r13), %xmm0
	vmovss	%xmm0, 200(%rsp)
	vmovss	600(%r13), %xmm0
	vmovss	%xmm0, 192(%rsp)
	vmovss	604(%r13), %xmm0
	vmovss	%xmm0, 416(%rsp)
	movq	1576(%rsp), %rcx
.Ltmp11193:
	.loc	5 568 12 is_stmt 1
	leaq	(%rcx,%r11), %rcx
	movl	$0, %edx
	vmovss	972(%rsp), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	976(%rsp), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	948(%rsp), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	1220(%rsp), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovaps	1488(%rsp), %xmm3
	vmovss	%xmm3, 208(%rsp)
	vmovss	824(%rsp), %xmm6
	vmovss	952(%rsp), %xmm8
	vmovss	184(%rsp), %xmm4
	vmovss	980(%rsp), %xmm13
	vmovss	(%rsp), %xmm0
	vmovss	608(%r13), %xmm2
	vmovss	%xmm2, 248(%rsp)
	vmovss	612(%r13), %xmm2
	vmovss	%xmm2, 600(%rsp)
	vmovss	616(%r13), %xmm2
	vmovss	%xmm2, 592(%rsp)
	vmovss	620(%r13), %xmm2
	vmovss	%xmm2, 240(%rsp)
	vmovss	624(%r13), %xmm2
	vmovss	%xmm2, 352(%rsp)
	vmovss	628(%r13), %xmm2
	vmovss	%xmm2, 344(%rsp)
	vmovss	632(%r13), %xmm2
	vmovss	%xmm2, 336(%rsp)
	vmovss	636(%r13), %xmm2
	vmovss	%xmm2, 280(%rsp)
	vmovss	640(%r13), %xmm2
	vmovss	%xmm2, 328(%rsp)
	vmovss	644(%r13), %xmm2
	vmovss	%xmm2, 320(%rsp)
	vmovss	648(%r13), %xmm2
	vmovss	%xmm2, 272(%rsp)
	vmovss	652(%r13), %xmm2
	vmovss	%xmm2, 488(%rsp)
	vmovss	656(%r13), %xmm2
	vmovss	%xmm2, 480(%rsp)
	vmovss	660(%r13), %xmm2
	vmovss	%xmm2, 472(%rsp)
	vmovss	664(%r13), %xmm2
	vmovss	%xmm2, 408(%rsp)
	vmovss	668(%r13), %xmm2
	vmovss	%xmm2, 312(%rsp)
	vmovss	672(%r13), %xmm2
	vmovss	%xmm2, 464(%rsp)
	vmovss	676(%r13), %xmm2
	vmovss	%xmm2, 400(%rsp)
	vmovss	680(%r13), %xmm2
	vmovss	%xmm2, 68(%rsp)
	vmovss	684(%r13), %xmm2
	vmovss	%xmm2, 456(%rsp)
	vmovss	688(%r13), %xmm2
	vmovss	%xmm2, 304(%rsp)
	vmovss	692(%r13), %xmm2
	vmovss	%xmm2, 392(%rsp)
	vmovss	696(%r13), %xmm2
	vmovss	%xmm2, 16(%rsp)
	vmovss	700(%r13), %xmm2
	vmovss	%xmm2, 12(%rsp)
	vmovss	704(%r13), %xmm2
	vmovss	%xmm2, 264(%rsp)
	vmovss	708(%r13), %xmm2
	vmovss	%xmm2, 384(%rsp)
	vmovss	712(%r13), %xmm2
	vmovss	%xmm2, 72(%rsp)
	vmovss	716(%r13), %xmm2
	vmovss	%xmm2, 448(%rsp)
	vmovss	720(%r13), %xmm2
	vmovss	%xmm2, 440(%rsp)
	vmovss	724(%r13), %xmm2
	vmovss	%xmm2, 96(%rsp)
	vmovss	728(%r13), %xmm2
	vmovss	%xmm2, 88(%rsp)
	vmovss	732(%r13), %xmm2
	vmovss	%xmm2, 28(%rsp)
	vmovss	736(%r13), %xmm2
	vmovss	%xmm2, 24(%rsp)
	vmovss	740(%r13), %xmm2
	vmovss	%xmm2, 20(%rsp)
	vmovss	744(%r13), %xmm2
	vmovss	%xmm2, 664(%rsp)
	vmovss	748(%r13), %xmm2
	vmovss	%xmm2, 656(%rsp)
	vmovss	752(%r13), %xmm2
	vmovss	%xmm2, 140(%rsp)
	vmovss	756(%r13), %xmm2
	vmovss	%xmm2, 136(%rsp)
	vmovss	760(%r13), %xmm2
	vmovss	%xmm2, 132(%rsp)
	vmovss	764(%r13), %xmm2
	vmovss	%xmm2, 128(%rsp)
	vmovss	768(%r13), %xmm2
	vmovss	%xmm2, 124(%rsp)
	vmovss	772(%r13), %xmm2
	vmovss	%xmm2, 432(%rsp)
	vmovss	104(%rsp), %xmm2
	vmovss	%xmm2, 144(%rsp)
	.loc	5 0 12 is_stmt 0
.Ltmp11194:
	.p2align	4
.LBB36_526:
	vmovss	144(%rsp), %xmm7
	vmovaps	%xmm13, %xmm5
	vmovss	208(%rsp), %xmm15
	vmovss	32(%rsp), %xmm13
	vmovss	368(%rsp), %xmm10
	vmovss	48(%rsp), %xmm1
	vmovss	160(%rsp), %xmm2
	.loc	5 568 12 is_stmt 1
	leaq	(%r11,%rdx), %rdi
	cmpq	296(%rsp), %rdi
	ja	.LBB36_439
.Ltmp11195:
	.loc	5 0 12 is_stmt 0
	vmovss	%xmm2, 800(%rsp)
	.loc	1 1762 0 is_stmt 1
	movq	%rcx, %rsi
	addq	%rdx, %rsi
.Ltmp11196:
	.loc	52 51 9
	je	.LBB36_519
	.loc	52 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm14
	vmovaps	%xmm4, %xmm12
	vmovaps	%xmm8, %xmm11
	vmovaps	%xmm6, %xmm0
	movq	672(%rsp), %rsi
	.loc	52 51 9
	vmovss	(%rsi,%rdx,4), %xmm6
	vbroadcastss	.LCPI36_0(%rip), %xmm9
.Ltmp11197:
	.loc	52 103 24 is_stmt 1
	vandps	%xmm0, %xmm9, %xmm2
	vmovaps	%xmm2, 160(%rsp)
.Ltmp11198:
	.loc	52 71 9
	vmulss	84(%rsp), %xmm6, %xmm2
	vxorps	%xmm8, %xmm8, %xmm8
.Ltmp11199:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11200:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm6, %xmm3
.Ltmp11201:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11202:
	.loc	52 71 9
	vmulss	256(%rsp), %xmm6, %xmm4
.Ltmp11203:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
	vmovss	%xmm6, 144(%rsp)
.Ltmp11204:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm6, %xmm6
.Ltmp11205:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11206:
	.loc	52 71 9
	vmulss	192(%rsp), %xmm7, %xmm8
.Ltmp11207:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11208:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm7, %xmm8
.Ltmp11209:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11210:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm7, %xmm8
.Ltmp11211:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11212:
	.loc	52 71 9
	vmulss	600(%rsp), %xmm7, %xmm8
.Ltmp11213:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11214:
	.loc	52 71 9
	vmulss	592(%rsp), %xmm14, %xmm8
.Ltmp11215:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11216:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm14, %xmm8
.Ltmp11217:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11218:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm14, %xmm8
.Ltmp11219:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11220:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm14, %xmm8
.Ltmp11221:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11222:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm5, %xmm8
.Ltmp11223:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11224:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm5, %xmm8
.Ltmp11225:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11226:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm5, %xmm8
.Ltmp11227:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11228:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm5, %xmm8
.Ltmp11229:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11230:
	.loc	52 71 9
	vmulss	272(%rsp), %xmm12, %xmm8
.Ltmp11231:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11232:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm12, %xmm8
.Ltmp11233:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11234:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm12, %xmm8
.Ltmp11235:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11236:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm12, %xmm8
.Ltmp11237:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11238:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm11, %xmm8
.Ltmp11239:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11240:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm11, %xmm8
.Ltmp11241:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11242:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm11, %xmm8
.Ltmp11243:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11244:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm11, %xmm8
.Ltmp11245:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11246:
	.loc	52 71 9
	vmulss	68(%rsp), %xmm0, %xmm8
.Ltmp11247:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11248:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm0, %xmm8
.Ltmp11249:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11250:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm0, %xmm8
.Ltmp11251:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11252:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm0, %xmm8
.Ltmp11253:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11254:
	.loc	52 71 9
	vmulss	16(%rsp), %xmm15, %xmm8
.Ltmp11255:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11256:
	.loc	52 71 9
	vmulss	12(%rsp), %xmm15, %xmm8
.Ltmp11257:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11258:
	.loc	52 71 9
	vmulss	264(%rsp), %xmm15, %xmm8
.Ltmp11259:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11260:
	.loc	52 71 9
	vmulss	384(%rsp), %xmm15, %xmm8
.Ltmp11261:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11262:
	.loc	52 71 9
	vmulss	72(%rsp), %xmm13, %xmm8
.Ltmp11263:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11264:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm13, %xmm8
.Ltmp11265:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11266:
	.loc	52 71 9
	vmulss	440(%rsp), %xmm13, %xmm8
.Ltmp11267:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11268:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm13, %xmm8
.Ltmp11269:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11270:
	.loc	52 71 9
	vmulss	88(%rsp), %xmm10, %xmm8
.Ltmp11271:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11272:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm10, %xmm8
.Ltmp11273:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11274:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm10, %xmm8
.Ltmp11275:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11276:
	.loc	52 71 9
	vmulss	20(%rsp), %xmm10, %xmm8
.Ltmp11277:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovaps	%xmm1, %xmm9
.Ltmp11278:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm1, %xmm8
.Ltmp11279:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11280:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm1, %xmm8
.Ltmp11281:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11282:
	.loc	52 71 9
	vmulss	140(%rsp), %xmm1, %xmm8
.Ltmp11283:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11284:
	.loc	52 71 9
	vmulss	136(%rsp), %xmm1, %xmm8
.Ltmp11285:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vmovss	%xmm7, (%rsp)
	vmovss	800(%rsp), %xmm7
.Ltmp11286:
	.loc	52 71 9
	vmulss	132(%rsp), %xmm7, %xmm8
.Ltmp11287:
	.loc	52 61 9
	vaddss	%xmm2, %xmm8, %xmm2
.Ltmp11288:
	.loc	52 71 9
	vmulss	128(%rsp), %xmm7, %xmm8
.Ltmp11289:
	.loc	52 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp11290:
	.loc	52 71 9
	vmulss	124(%rsp), %xmm7, %xmm8
.Ltmp11291:
	.loc	52 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp11292:
	.loc	52 71 9
	vmulss	432(%rsp), %xmm7, %xmm8
.Ltmp11293:
	.loc	52 61 9
	vaddss	%xmm6, %xmm8, %xmm6
	vbroadcastss	.LCPI36_0(%rip), %xmm8
.Ltmp11294:
	.loc	52 103 24
	vandps	%xmm2, %xmm8, %xmm2
	vmovaps	160(%rsp), %xmm1
.Ltmp11295:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11296:
	.loc	52 103 24
	vandps	%xmm3, %xmm8, %xmm2
.Ltmp11297:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11298:
	.loc	52 103 24
	vandps	%xmm4, %xmm8, %xmm2
.Ltmp11299:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11300:
	.loc	52 103 24
	vandps	%xmm6, %xmm8, %xmm2
.Ltmp11301:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11302:
	.loc	52 56 9
	vmovss	%xmm1, 1616(%rsp,%rdx,4)
.Ltmp11303:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm9, 160(%rsp)
	vmovss	%xmm10, 48(%rsp)
	vmovss	%xmm13, 368(%rsp)
	vmovss	%xmm15, 32(%rsp)
	vmovss	%xmm0, 208(%rsp)
	vmovaps	%xmm11, %xmm6
	vmovaps	%xmm12, %xmm8
	vmovaps	%xmm5, %xmm4
	vmovaps	%xmm0, %xmm3
	vmovaps	%xmm13, %xmm2
	vmovaps	%xmm14, %xmm13
	vmovss	(%rsp), %xmm0
	vmovaps	%xmm0, %xmm1
.Ltmp11304:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp11305:
	.loc	3 900 12
	jne	.LBB36_526
.Ltmp11306:
.LBB36_529:
	.loc	3 0 12 is_stmt 0
	vmovss	%xmm11, 824(%rsp)
	vmovss	144(%rsp), %xmm0
	vmovss	%xmm1, (%rsp)
	.loc	1 1765 5 is_stmt 1
	vmovss	%xmm1, 1096(%rsp)
	vmovss	%xmm14, 980(%rsp)
	vmovss	%xmm14, 1100(%rsp)
	vmovss	%xmm5, 1104(%rsp)
	vmovss	%xmm12, 1108(%rsp)
	vmovss	%xmm11, 1112(%rsp)
	vmovss	%xmm3, 1116(%rsp)
	vmovss	%xmm15, 1120(%rsp)
	vmovss	%xmm2, 1124(%rsp)
	vmovss	%xmm10, 976(%rsp)
	vmovss	%xmm10, 1128(%rsp)
	vmovss	%xmm9, 972(%rsp)
	vmovss	%xmm9, 1132(%rsp)
	vmovss	%xmm7, 800(%rsp)
	vmovss	%xmm7, 1136(%rsp)
.Ltmp11307:
	.loc	10 1916 50
	cmpq	%r11, 232(%rsp)
	vmovaps	%xmm5, %xmm13
	vmovaps	%xmm2, %xmm5
	vmovss	%xmm0, 104(%rsp)
.Ltmp11308:
	.loc	1 1989 19
	jne	.LBB36_531
.Ltmp11309:
	.loc	1 0 19 is_stmt 0
	vmovaps	%xmm15, %xmm4
	movq	1536(%rsp), %rdx
	vmovss	1216(%rsp), %xmm14
	vmovss	1212(%rsp), %xmm0
	vmovss	1208(%rsp), %xmm8
.LBB36_513:
	addq	$32, %r11
	decq	%rdx
	movq	1592(%rsp), %rax
.Ltmp11310:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rax
	subq	$-128, 696(%rsp)
	subq	$-128, 672(%rsp)
	testq	%rdx, %rdx
	movq	232(%rsp), %rcx
	movl	$32, %esi
	vmovss	1528(%rsp), %xmm10
	vmovss	1524(%rsp), %xmm1
	vmovss	1520(%rsp), %xmm6
	vmovss	608(%rsp), %xmm2
	vmovss	(%rsp), %xmm9
	vmovss	40(%rsp), %xmm11
	vmovss	824(%rsp), %xmm15
	vmovss	104(%rsp), %xmm7
	jne	.LBB36_514
	jmp	.LBB36_574
.Ltmp11311:
.LBB36_531:
	.loc	8 0 20 is_stmt 0
	vmovss	%xmm13, 184(%rsp)
	vmovss	%xmm5, 948(%rsp)
	vmovaps	%xmm3, 1488(%rsp)
	vmovss	%xmm12, 952(%rsp)
	movq	%r11, 264(%rsp)
	movl	%r8d, 144(%rsp)
	movq	552(%rsp), %r8
	movq	560(%rsp), %rbx
	movq	888(%rsp), %rax
	movq	%rax, 72(%rsp)
	movq	896(%rsp), %r11
	movq	496(%rsp), %r13
	movq	504(%rsp), %rdx
	movq	544(%rsp), %rsi
	movq	512(%rsp), %rax
	movq	%rax, 328(%rsp)
	movq	520(%rsp), %rax
	movq	%rax, 240(%rsp)
	movq	536(%rsp), %rax
	movq	%rax, 280(%rsp)
	movq	528(%rsp), %rax
	movq	%rax, 488(%rsp)
	movq	832(%rsp), %rcx
	movq	880(%rsp), %rax
.Ltmp11312:
	.loc	1 1989 19 is_stmt 1
	cmpq	$1, %rsi
	movq	%rsi, 352(%rsp)
	adcq	$0, %rsi
	movq	%rsi, 344(%rsp)
	cmpq	$1, %rax
	movq	%rax, 480(%rsp)
	adcq	$0, %rax
	movq	%rax, 472(%rsp)
	vmovss	1048(%rsp), %xmm0
	vmovss	%xmm0, 84(%rsp)
	vmovss	1064(%rsp), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	1140(%rsp), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	1156(%rsp), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	1088(%rsp), %xmm0
	vmovss	%xmm0, 272(%rsp)
	vmovss	1180(%rsp), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	688(%rsp), %xmm4
	vmovss	%xmm4, 16(%rsp)
	vmovss	692(%rsp), %xmm13
	vmovss	%xmm13, 12(%rsp)
	movq	840(%rsp), %rax
	movq	848(%rsp), %rsi
	movq	%rsi, 464(%rsp)
	movq	856(%rsp), %rsi
	movq	%rsi, 408(%rsp)
	movq	872(%rsp), %rsi
	movq	%rsi, 312(%rsp)
	movq	864(%rsp), %rsi
	movq	%rsi, 456(%rsp)
	movl	%r9d, 96(%rsp)
	movl	%r9d, %esi
	movq	%rsi, 208(%rsp)
	movl	%r10d, 88(%rsp)
	movl	%r10d, %esi
	movq	%rsi, 368(%rsp)
	xorl	%edi, %edi
	movq	1512(%rsp), %r15
	movq	816(%rsp), %r14
	vmovss	1216(%rsp), %xmm6
	vmovss	1212(%rsp), %xmm10
	xorl	%r10d, %r10d
	vmovss	1208(%rsp), %xmm12
	vmovss	.LCPI36_2(%rip), %xmm3
	movq	1544(%rsp), %rsi
	vmovss	40(%rsp), %xmm0
	movq	%r8, 440(%rsp)
	movq	%rbx, 384(%rsp)
	movq	%r11, 448(%rsp)
.LBB36_532:
	.loc	1 0 19 is_stmt 0
	movq	%rdi, 304(%rsp)
	movq	%r14, %rbp
	.loc	1 1994 21 is_stmt 1
	movq	%rsi, %r14
	subq	%rdi, %r14
	movq	936(%rsp), %rsi
.Ltmp11313:
	.loc	1 1575 16
	movq	544(%rsi), %r9
.Ltmp11314:
	.loc	1 1576 16
	movq	552(%rsi), %rbx
.Ltmp11315:
	.loc	1 1577 25
	leaq	1(%rbp), %rsi
.Ltmp11316:
	.loc	1 1148 8
	cmpq	%r9, %rsi
	movq	%r9, %rsi
	cmovbq	%r10, %rsi
	negq	%rsi
.Ltmp11317:
	.loc	1 1578 28
	leaq	(%r8,%rbp), %r11
.Ltmp11318:
	.loc	1 1148 8
	cmpq	%r9, %r11
	movq	%r9, %r8
	cmovbq	%r10, %r8
	subq	%r8, %r11
	movq	72(%rsp), %r8
.Ltmp11319:
	.loc	1 1579 29
	leaq	(%r8,%rbp), %r10
.Ltmp11320:
	.loc	1 1148 8
	cmpq	%r9, %r10
	movq	%r9, %r8
	movl	$0, %edi
	cmovbq	%rdi, %r8
	subq	%r8, %r10
	movq	%r10, 200(%rsp)
	movq	384(%rsp), %r8
.Ltmp11321:
	.loc	1 1580 33
	leaq	(%r8,%rbp), %r10
.Ltmp11322:
	.loc	1 1148 8
	cmpq	%r9, %r10
	movq	%r9, %r8
	movl	$0, %edi
	cmovbq	%rdi, %r8
	subq	%r8, %r10
	movq	%r10, 256(%rsp)
	movq	448(%rsp), %rdi
.Ltmp11323:
	.loc	1 1581 34
	addq	%rbp, %rdi
.Ltmp11324:
	.loc	1 1148 8
	cmpq	%r9, %rdi
	movq	%r9, %r8
	movl	$0, %r10d
	cmovbq	%r10, %r8
.Ltmp11325:
	.loc	1 1148 8 is_stmt 0
	addq	%rbp, %rsi
	incq	%rsi
.Ltmp11326:
	.loc	1 1148 8
	subq	%r8, %rdi
.Ltmp11327:
	.loc	1 1583 14 is_stmt 1
	movq	%r9, %r10
	movq	%rbp, 816(%rsp)
	subq	%rbp, %r10
.Ltmp11328:
	.loc	10 1078 5
	cmpq	%r14, %r10
	cmovbq	%r10, %r14
.Ltmp11329:
	.loc	1 1584 14
	subq	%r15, %rbx
.Ltmp11330:
	.loc	10 1078 5
	cmpq	%r14, %rbx
	movq	%rbx, 32(%rsp)
	cmovbq	%rbx, %r14
.Ltmp11331:
	.loc	1 1585 14
	movq	%r9, %rbx
	movq	%rsi, 416(%rsp)
	subq	%rsi, %rbx
.Ltmp11332:
	.loc	10 1078 5
	cmpq	%r14, %rbx
	cmovbq	%rbx, %r14
.Ltmp11333:
	.loc	1 1586 14
	movq	%r9, %rbp
	movq	%r11, 192(%rsp)
	subq	%r11, %rbp
.Ltmp11334:
	.loc	10 1078 5
	cmpq	%r14, %rbp
	cmovbq	%rbp, %r14
.Ltmp11335:
	.loc	1 1587 14
	movq	%r9, %r11
	movq	200(%rsp), %rsi
	subq	%rsi, %r11
.Ltmp11336:
	.loc	10 1078 5
	cmpq	%r14, %r11
	cmovbq	%r11, %r14
.Ltmp11337:
	.loc	1 1588 14
	movq	%r9, %r8
	movq	256(%rsp), %rsi
	subq	%rsi, %r8
.Ltmp11338:
	.loc	10 1078 5
	cmpq	%r14, %r8
	cmovbq	%r8, %r14
	movq	%rdi, 248(%rsp)
.Ltmp11339:
	.loc	1 1589 14
	subq	%rdi, %r9
.Ltmp11340:
	.loc	10 1078 5
	cmpq	%r14, %r9
	cmovbq	%r9, %r14
	movq	264(%rsp), %rsi
	movq	304(%rsp), %rdi
.Ltmp11341:
	.loc	1 2000 28
	addq	%rsi, %rdi
	movq	%r14, 392(%rsp)
.Ltmp11342:
	.loc	1 2002 55
	addq	%rdi, %r14
.Ltmp11343:
	.loc	4 1050 16
	jb	.LBB36_676
	cmpq	960(%rsp), %r14
	ja	.LBB36_676
.Ltmp11344:
	.loc	5 451 16
	cmpq	296(%rsp), %r14
	ja	.LBB36_677
.Ltmp11345:
	.loc	14 304 12
	cmpq	$0, 392(%rsp)
	movq	816(%rsp), %r14
	je	.LBB36_536
.Ltmp11346:
	.loc	14 0 12 is_stmt 0
	movq	680(%rsp), %rsi
	leaq	(%rsi,%rdi,4), %rsi
	movq	%rsi, 160(%rsp)
.Ltmp11347:
	.loc	14 304 12
	cmpq	%r11, %rbp
	cmovbq	%rbp, %r11
	movq	928(%rsp), %rsi
.Ltmp11348:
	.loc	1 0 0
	leaq	(%rsi,%rdi,4), %rsi
	movq	%rsi, 48(%rsp)
.Ltmp11349:
	.loc	14 304 12
	cmpq	%r8, %r11
	cmovaeq	%r8, %r11
	movq	304(%rsp), %rsi
.Ltmp11350:
	.loc	1 0 0
	leaq	(%rsp,%rsi,4), %rdi
	addq	$2648, %rdi
	movq	%rdi, 600(%rsp)
.Ltmp11351:
	.loc	14 304 12
	cmpq	%r9, %r11
	cmovaeq	%r9, %r11
.Ltmp11352:
	.loc	1 0 0
	leaq	(%rsp,%rsi,4), %rdi
	addq	$1616, %rdi
	movq	%rdi, 592(%rsp)
.Ltmp11353:
	.loc	14 304 12
	cmpq	%rbx, %r11
	cmovaeq	%rbx, %r11
	cmpq	%r10, %r11
	cmovaeq	%r10, %r11
	movq	32(%rsp), %rdi
	cmpq	%rdi, %r11
	cmovaeq	%rdi, %r11
	movq	1608(%rsp), %rdi
	subq	%rsi, %rdi
	cmpq	%rdi, %r11
	cmovbq	%r11, %rdi
	movq	%rdi, 336(%rsp)
	vmovaps	%xmm6, %xmm14
	vmovaps	%xmm10, %xmm7
	vmovss	%xmm0, 40(%rsp)
	vmovaps	%xmm0, %xmm9
	vmovaps	%xmm12, %xmm8
	xorl	%r9d, %r9d
.Ltmp11354:
	.loc	14 0 12
.Ltmp11355:
	.p2align	4
.LBB36_538:
	movq	600(%rsp), %rsi
.Ltmp11356:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rsi,%r9,4), %xmm0
	movq	592(%rsp), %rsi
.Ltmp11357:
	.loc	52 51 9 is_stmt 0
	vmovss	(%rsi,%r9,4), %xmm1
.Ltmp11358:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
.Ltmp11359:
	.loc	41 1244 18
	vmovd	%xmm1, %esi
.Ltmp11360:
	.loc	41 1244 18 is_stmt 0
	vmovd	%xmm0, %edi
.Ltmp11361:
	.loc	52 161 24 is_stmt 1
	movl	%esi, %ebx
	cmovbel	%edi, %ebx
.Ltmp11362:
	.loc	1 1502 26
	leaq	(%r9,%r14), %r10
.Ltmp11363:
	.loc	4 1050 16
	cmpq	%rdx, %r10
	jae	.LBB36_678
.Ltmp11364:
	.loc	1 0 0 is_stmt 0
	andl	648(%rsp), %ebx
.Ltmp11365:
	.loc	52 161 44 is_stmt 1
	andl	288(%rsp), %edi
	.loc	52 161 24 is_stmt 0
	orl	%ebx, %edi
.Ltmp11366:
	.loc	41 1291 18 is_stmt 1
	vmovd	%edi, %xmm0
.Ltmp11367:
	.loc	52 124 14
	vucomiss	84(%rsp), %xmm0
	vmovaps	%xmm3, %xmm2
.Ltmp11368:
	.loc	52 161 24
	jbe	.LBB36_541
.Ltmp11369:
	.loc	52 0 24 is_stmt 0
	vmovss	84(%rsp), %xmm1
.Ltmp11370:
	.loc	52 76 9 is_stmt 1
	vdivss	%xmm0, %xmm1, %xmm2
.Ltmp11371:
.LBB36_541:
	.loc	52 0 9 is_stmt 0
	movq	160(%rsp), %rdi
	vmovss	(%rdi,%r9,4), %xmm5
	movq	48(%rsp), %rdi
	vmovss	(%rdi,%r9,4), %xmm0
	movq	192(%rsp), %rdi
	addq	%r9, %rdi
.Ltmp11372:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm2, (%r13,%r10,4)
.Ltmp11373:
	.loc	4 1050 16
	cmpq	%rdx, %rdi
	movq	240(%rsp), %r11
	movq	368(%rsp), %rbp
	jae	.LBB36_542
.Ltmp11374:
	.loc	52 51 9
	vmovss	(%r13,%rdi,4), %xmm2
	vmovaps	%xmm2, %xmm11
.Ltmp11375:
	.loc	1 1206 22
	testl	%ebp, %ebp
	je	.LBB36_545
.Ltmp11376:
	.loc	1 0 22 is_stmt 0
	vminss	%xmm2, %xmm13, %xmm11
.LBB36_545:
	movq	416(%rsp), %r8
	addq	%r9, %r8
	movq	%r8, 32(%rsp)
.Ltmp11377:
	movl	%ebp, %ebp
.Ltmp11378:
	.loc	1 1212 20 is_stmt 1
	incq	%rbp
	movq	344(%rsp), %r8
	cmpq	352(%rsp), %rbp
.Ltmp11379:
	.loc	1 1213 22
	jne	.LBB36_546
	.loc	1 0 22 is_stmt 0
.Ltmp11380:
	.p2align	4
.LBB36_548:
.Ltmp11381:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB36_542
.Ltmp11382:
	.loc	52 161 24
	vminss	(%r13,%rdi,4), %xmm2, %xmm2
.Ltmp11383:
	.loc	52 56 9
	vmovss	%xmm2, (%r13,%rdi,4)
.Ltmp11384:
	.loc	1 1224 16
	testq	%rdi, %rdi
	cmoveq	%r12, %rdi
	.loc	1 1227 13
	decq	%rdi
.Ltmp11385:
	.loc	10 1916 50
	decq	%r8
.Ltmp11386:
	.loc	3 900 12
	jne	.LBB36_548
.Ltmp11387:
	.loc	3 0 12 is_stmt 0
	xorl	%ebp, %ebp
	vmovaps	%xmm11, %xmm2
	jmp	.LBB36_551
	.p2align	4
.LBB36_546:
	movq	32(%rsp), %rdi
.Ltmp11388:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB36_542
.Ltmp11389:
	.loc	52 51 9
	vmovss	(%r13,%rdi,4), %xmm2
.Ltmp11390:
	.loc	52 161 24
	vminss	%xmm11, %xmm2, %xmm2
.Ltmp11391:
.LBB36_551:
	.loc	52 0 24 is_stmt 0
	movq	256(%rsp), %rdi
	addq	%r9, %rdi
.Ltmp11392:
	.loc	52 71 9 is_stmt 1
	vmulss	.LCPI36_3(%rip), %xmm2, %xmm2
.Ltmp11393:
	.loc	41 1783 9
	vroundss	$9, %xmm2, %xmm2, %xmm2
.Ltmp11394:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm2, %xmm2
.Ltmp11395:
	.loc	4 1050 16
	cmpq	%r11, %rdi
	jae	.LBB36_679
.Ltmp11396:
	.loc	1 0 0 is_stmt 0
	leaq	(%r9,%r14), %r8
	incq	%r8
	cmpq	%r11, %r8
.Ltmp11397:
	.loc	4 1050 16
	ja	.LBB36_680
.Ltmp11398:
	.loc	4 0 16
	movq	%rbp, 368(%rsp)
	leaq	(%r9,%r15), %rbp
	vmovaps	%xmm3, %xmm1
.Ltmp11399:
	vaddss	%xmm2, %xmm8, %xmm3
	movq	328(%rsp), %r11
	vsubss	(%r11,%rdi,4), %xmm3, %xmm8
.Ltmp11400:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm2, (%r11,%r10,4)
.Ltmp11401:
	.loc	52 76 9
	vdivss	272(%rsp), %xmm8, %xmm2
.Ltmp11402:
	.loc	52 66 9
	vsubss	%xmm2, %xmm1, %xmm2
.Ltmp11403:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm9, %xmm2, %xmm3
.Ltmp11404:
	.loc	52 92 9 is_stmt 1
	vmulss	320(%rsp), %xmm3, %xmm3
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp11405:
	.loc	52 103 24
	vbroadcastss	.LCPI36_0(%rip), %xmm13
.Ltmp11406:
	.loc	52 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp11407:
	.loc	52 103 24
	vandps	%xmm2, %xmm13, %xmm3
.Ltmp11408:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm3, %xmm3
.Ltmp11409:
	.loc	4 1050 16
	cmpq	280(%rsp), %rbp
	jae	.LBB36_681
.Ltmp11410:
	.loc	4 0 16 is_stmt 0
	movq	%r14, %r11
	movq	%r15, %r14
	vandps	%xmm2, %xmm3, %xmm9
.Ltmp11411:
	.loc	52 66 9 is_stmt 1
	vsubss	%xmm9, %xmm1, %xmm2
	movq	488(%rsp), %rdi
.Ltmp11412:
	.loc	52 51 9
	vmovss	(%rdi,%rbp,4), %xmm3
.Ltmp11413:
	.loc	52 56 9
	vmovss	%xmm5, (%rdi,%rbp,4)
.Ltmp11414:
	.loc	52 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp11415:
	.loc	41 1244 18
	vmovd	%xmm3, %edi
.Ltmp11416:
	.loc	52 161 24
	andl	360(%rsp), %edi
.Ltmp11417:
	.loc	41 1244 18
	vmovd	%xmm2, %r15d
.Ltmp11418:
	.loc	52 161 44
	andl	144(%rsp), %r15d
	.loc	52 161 24 is_stmt 0
	orl	%edi, %r15d
	movq	160(%rsp), %rdi
.Ltmp11419:
	.loc	52 56 9 is_stmt 1
	movl	%r15d, (%rdi,%r9,4)
	cmpq	%rax, %r8
.Ltmp11420:
	.loc	4 1050 16
	ja	.LBB36_555
.Ltmp11421:
	.loc	1 0 0 is_stmt 0
	andl	288(%rsp), %esi
	orl	%esi, %ebx
	vmovd	%ebx, %xmm2
.Ltmp11422:
	.loc	52 124 14 is_stmt 1
	vucomiss	424(%rsp), %xmm2
	vmovaps	%xmm1, %xmm5
	vmovaps	%xmm1, %xmm3
.Ltmp11423:
	.loc	52 161 24
	jbe	.LBB36_558
.Ltmp11424:
	.loc	52 0 24 is_stmt 0
	vmovss	424(%rsp), %xmm1
.Ltmp11425:
	.loc	52 76 9 is_stmt 1
	vdivss	%xmm2, %xmm1, %xmm5
.Ltmp11426:
.LBB36_558:
	.loc	52 0 9 is_stmt 0
	movq	200(%rsp), %rsi
	leaq	(%r9,%rsi), %rdi
.Ltmp11427:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm5, (%rcx,%r10,4)
.Ltmp11428:
	.loc	4 1050 16
	cmpq	%rax, %rdi
	movq	208(%rsp), %rbx
	jae	.LBB36_559
.Ltmp11429:
	.loc	4 0 16 is_stmt 0
	movq	%r14, %r15
.Ltmp11430:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm1
	vmovaps	%xmm1, %xmm5
.Ltmp11431:
	.loc	1 1206 22
	testl	%ebx, %ebx
	je	.LBB36_562
.Ltmp11432:
	.loc	1 0 22 is_stmt 0
	vminss	%xmm1, %xmm4, %xmm5
.LBB36_562:
	movl	%ebx, %ebx
.Ltmp11433:
	.loc	1 1212 20 is_stmt 1
	incq	%rbx
	movq	472(%rsp), %rsi
	cmpq	480(%rsp), %rbx
	movq	%r11, %r14
.Ltmp11434:
	.loc	1 1213 22
	jne	.LBB36_563
	.loc	1 0 22 is_stmt 0
.Ltmp11435:
	.p2align	4
.LBB36_565:
.Ltmp11436:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rax, %rdi
	jae	.LBB36_559
.Ltmp11437:
	.loc	52 161 24
	vminss	(%rcx,%rdi,4), %xmm1, %xmm1
.Ltmp11438:
	.loc	52 56 9
	vmovss	%xmm1, (%rcx,%rdi,4)
.Ltmp11439:
	.loc	1 1224 16
	testq	%rdi, %rdi
	cmoveq	%r12, %rdi
	.loc	1 1227 13
	decq	%rdi
.Ltmp11440:
	.loc	10 1916 50
	decq	%rsi
.Ltmp11441:
	.loc	3 900 12
	jne	.LBB36_565
.Ltmp11442:
	.loc	3 0 12 is_stmt 0
	xorl	%ebx, %ebx
	vmovaps	%xmm5, %xmm1
	jmp	.LBB36_568
	.p2align	4
.LBB36_563:
	movq	32(%rsp), %rdi
.Ltmp11443:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rax, %rdi
	jae	.LBB36_559
.Ltmp11444:
	.loc	52 51 9
	vmovss	(%rcx,%rdi,4), %xmm1
.Ltmp11445:
	.loc	52 161 24
	vminss	%xmm5, %xmm1, %xmm1
.Ltmp11446:
.LBB36_568:
	.loc	52 0 24 is_stmt 0
	movq	248(%rsp), %rsi
	leaq	(%r9,%rsi), %rdi
.Ltmp11447:
	.loc	52 71 9 is_stmt 1
	vmulss	.LCPI36_3(%rip), %xmm1, %xmm1
.Ltmp11448:
	.loc	41 1783 9
	vroundss	$9, %xmm1, %xmm1, %xmm1
.Ltmp11449:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm1, %xmm1
	movq	408(%rsp), %rsi
.Ltmp11450:
	.loc	4 1050 16
	cmpq	%rsi, %rdi
	jae	.LBB36_569
.Ltmp11451:
	.loc	4 0 16 is_stmt 0
	cmpq	%rsi, %r8
.Ltmp11452:
	.loc	4 1050 16
	ja	.LBB36_682
.Ltmp11453:
	.loc	1 0 0
	leaq	1(%rbp), %rsi
.Ltmp11454:
	vaddss	%xmm7, %xmm1, %xmm2
	movq	464(%rsp), %r8
	vsubss	(%r8,%rdi,4), %xmm2, %xmm7
.Ltmp11455:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm1, (%r8,%r10,4)
	cmpq	312(%rsp), %rsi
.Ltmp11456:
	.loc	4 1050 16
	ja	.LBB36_683
.Ltmp11457:
	.loc	4 0 16 is_stmt 0
	movq	%rbx, 208(%rsp)
.Ltmp11458:
	.loc	52 76 9 is_stmt 1
	vdivss	68(%rsp), %xmm7, %xmm1
.Ltmp11459:
	.loc	52 66 9
	vsubss	%xmm1, %xmm3, %xmm1
.Ltmp11460:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm14, %xmm1, %xmm2
.Ltmp11461:
	.loc	52 92 9 is_stmt 1
	vmulss	400(%rsp), %xmm2, %xmm2
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp11462:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm1, %xmm1
.Ltmp11463:
	.loc	52 103 24
	vandps	%xmm1, %xmm13, %xmm2
.Ltmp11464:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm2, %xmm2
	vandps	%xmm1, %xmm2, %xmm14
.Ltmp11465:
	.loc	52 66 9
	vsubss	%xmm14, %xmm3, %xmm1
	movq	456(%rsp), %rsi
.Ltmp11466:
	.loc	52 51 9
	vmovss	(%rsi,%rbp,4), %xmm2
.Ltmp11467:
	.loc	52 56 9
	vmovss	%xmm0, (%rsi,%rbp,4)
.Ltmp11468:
	.loc	52 71 9
	vmulss	%xmm2, %xmm1, %xmm0
.Ltmp11469:
	.loc	41 1244 18
	vmovd	%xmm2, %esi
.Ltmp11470:
	.loc	52 161 24
	andl	360(%rsp), %esi
.Ltmp11471:
	.loc	41 1244 18
	vmovd	%xmm0, %edi
.Ltmp11472:
	.loc	52 161 44
	andl	144(%rsp), %edi
	.loc	52 161 24 is_stmt 0
	orl	%esi, %edi
	movq	48(%rsp), %rsi
.Ltmp11473:
	.loc	52 56 9 is_stmt 1
	movl	%edi, (%rsi,%r9,4)
.Ltmp11474:
	.loc	1 0 0 is_stmt 0
	incq	%r9
	vmovaps	%xmm5, %xmm4
	vmovaps	%xmm11, %xmm13
.Ltmp11475:
	.loc	14 304 12 is_stmt 1
	cmpq	336(%rsp), %r9
	movq	928(%rsp), %rbp
	jne	.LBB36_538
	jmp	.LBB36_573
.Ltmp11476:
.LBB36_536:
	.loc	14 0 12 is_stmt 0
	vmovss	16(%rsp), %xmm5
	vmovss	12(%rsp), %xmm11
	vmovaps	%xmm6, %xmm14
	vmovaps	%xmm10, %xmm7
	vmovaps	%xmm0, %xmm9
	vmovaps	%xmm12, %xmm8
	movq	928(%rsp), %rbp
.LBB36_573:
	movq	304(%rsp), %rdi
	movq	392(%rsp), %r8
	addq	%r8, %rdi
.Ltmp11477:
	.loc	1 2061 39 is_stmt 1
	addq	%r8, %r14
.Ltmp11478:
	.loc	1 1148 8
	cmpq	%r12, %r14
	movq	%r12, %rsi
	movl	$0, %r10d
	cmovbq	%r10, %rsi
	subq	%rsi, %r14
.Ltmp11479:
	.loc	1 2062 39
	addq	%r8, %r15
	movq	1600(%rsp), %rsi
.Ltmp11480:
	.loc	1 1148 8
	cmpq	%rsi, %r15
	cmovbq	%r10, %rsi
	subq	%rsi, %r15
	vmovss	%xmm5, 16(%rsp)
	vmovss	%xmm11, 12(%rsp)
	vmovaps	%xmm14, %xmm6
	vmovaps	%xmm7, %xmm10
	vmovaps	%xmm9, %xmm0
	vmovaps	%xmm8, %xmm12
	movq	1544(%rsp), %rsi
.Ltmp11481:
	.loc	1 1989 19
	cmpq	%rsi, %rdi
	movq	440(%rsp), %r8
	jb	.LBB36_532
.Ltmp11482:
	.loc	1 0 19 is_stmt 0
	vmovaps	%xmm15, %xmm4
	vmovss	%xmm5, 688(%rsp)
	vmovss	%xmm11, 692(%rsp)
	movq	%r14, 816(%rsp)
	movq	%r15, 1512(%rsp)
	vmovss	%xmm8, 1084(%rsp)
	vmovss	%xmm9, 1080(%rsp)
	vmovss	%xmm7, 1176(%rsp)
	vmovss	%xmm14, 1172(%rsp)
	movq	208(%rsp), %rax
	movl	%eax, %r9d
	movq	368(%rsp), %rax
	movl	%eax, %r10d
	vmovaps	%xmm7, %xmm0
	vmovss	%xmm9, 40(%rsp)
	movq	936(%rsp), %r13
	movq	960(%rsp), %r14
	movq	680(%rsp), %rbx
	movl	144(%rsp), %r8d
	movq	1536(%rsp), %rdx
	movq	264(%rsp), %r11
	vmovss	952(%rsp), %xmm12
	vmovaps	1488(%rsp), %xmm3
	vmovss	948(%rsp), %xmm5
	vmovss	184(%rsp), %xmm13
	jmp	.LBB36_513
.LBB36_415:
	leaq	708(%rsp), %rdi
	leaq	136(%r13), %rsi
.Ltmp11483:
	.loc	1 1946 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	496(%rsp), %rdi
.Ltmp11484:
	.loc	1 1947 25
	movq	%rbx, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp11485:
	.loc	1 1952 19
	movzbl	776(%r13), %eax
	movl	%eax, 200(%rsp)
.Ltmp11486:
	.loc	1 1953 21
	movzbl	777(%r13), %eax
	movl	%eax, 48(%rsp)
.Ltmp11487:
	.loc	1 1954 16
	movq	544(%r13), %r12
.Ltmp11488:
	.loc	1 1955 16
	movq	552(%r13), %r15
.Ltmp11489:
	.loc	1 1956 27
	movl	560(%r13), %eax
	movq	%rax, 648(%rsp)
.Ltmp11490:
	.loc	1 1957 27
	movl	564(%r13), %eax
	movq	%rax, 432(%rsp)
	leaq	2648(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %rbx
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	leaq	1616(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	leaq	1408(%rsp), %rdi
.Ltmp11491:
	.loc	1 1964 32
	leaq	136(%r13), %rsi
	movq	%r12, %rdx
	movq	%r15, 40(%rsp)
	movq	%r15, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
.Ltmp11492:
	.loc	1 1965 33
	movq	544(%r13), %rdx
	movq	552(%r13), %rcx
	leaq	832(%rsp), %rdi
	leaq	336(%r13), %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
	movq	232(%rsp), %rcx
.Ltmp11493:
	.loc	8 446 20
	testq	%rcx, %rcx
	je	.LBB36_416
.Ltmp11494:
	.loc	8 0 20 is_stmt 0
	movl	200(%rsp), %r8d
	movl	%r8d, %eax
	negl	%eax
	movl	%eax, 816(%rsp)
	movl	48(%rsp), %r9d
.Ltmp11495:
	movl	%r9d, %eax
	negl	%eax
	movl	%eax, 84(%rsp)
.Ltmp11496:
	.loc	4 3756 21 is_stmt 1
	movq	%rcx, %r10
	shrq	$5, %r10
.Ltmp11497:
	.loc	4 3757 21
	movl	%ecx, %eax
	andl	$31, %eax
.Ltmp11498:
	.loc	4 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %r10
	decl	%r8d
	decl	%r9d
.Ltmp11499:
	.loc	8 446 20
	movq	%r14, %r11
	negq	%r11
	movq	296(%rsp), %rax
	negq	%rax
	movq	%rax, 184(%rsp)
	movl	$32, %edx
	movq	%rbp, 672(%rsp)
	movq	680(%rsp), %rbx
	movq	%rbx, 696(%rsp)
	movq	%rcx, %rax
	xorl	%esi, %esi
.LBB36_422:
.Ltmp11500:
	.loc	4 2584 13
	cmpq	$32, %rax
	movl	$32, %edi
	movq	%rax, 112(%rsp)
	cmovbq	%rax, %rdi
	cmpq	$1, %rdi
	movq	%rdi, 824(%rsp)
	movq	%rdi, %rax
	adcq	$0, %rax
.Ltmp11501:
	.loc	10 1916 50
	movq	%rcx, %rdi
	subq	%rsi, %rdi
.Ltmp11502:
	.loc	10 1078 5
	cmpq	$32, %rdi
	cmovaeq	%rdx, %rdi
	movq	%rdi, 104(%rsp)
.Ltmp11503:
	.loc	1 1759 23
	vmovss	708(%rsp), %xmm0
	vmovss	%xmm0, (%rsp)
	vmovss	712(%rsp), %xmm4
	vmovss	716(%rsp), %xmm7
	vmovss	720(%rsp), %xmm3
	vmovss	724(%rsp), %xmm0
	vmovss	728(%rsp), %xmm6
	vmovss	732(%rsp), %xmm13
	vmovss	736(%rsp), %xmm14
	vmovss	740(%rsp), %xmm15
	vmovss	744(%rsp), %xmm8
	vmovss	748(%rsp), %xmm5
.Ltmp11504:
	.loc	10 1916 50
	cmpq	%rsi, %rcx
	movq	%rsi, 56(%rsp)
.Ltmp11505:
	.loc	3 900 12
	jne	.LBB36_424
.Ltmp11506:
	.loc	1 0 0 is_stmt 0
	vmovss	752(%rsp), %xmm9
	vmovaps	%xmm7, %xmm1
	vmovaps	%xmm4, %xmm12
.Ltmp11507:
	.loc	3 900 12
	jmp	.LBB36_428
.Ltmp11508:
.LBB36_424:
	.loc	3 0 12
	vmovss	584(%r13), %xmm2
	vmovss	%xmm2, 288(%rsp)
	vmovss	588(%r13), %xmm2
	vmovss	%xmm2, 360(%rsp)
	vmovss	592(%r13), %xmm2
	vmovss	%xmm2, 424(%rsp)
	vmovss	596(%r13), %xmm2
	vmovss	%xmm2, 256(%rsp)
	vmovss	600(%r13), %xmm1
	vmovss	%xmm1, 200(%rsp)
	vmovss	604(%r13), %xmm1
	vmovss	%xmm1, 192(%rsp)
.Ltmp11509:
	.loc	5 568 12 is_stmt 1
	leaq	(%r11,%rsi), %rcx
	xorl	%edx, %edx
	vmovss	%xmm5, 144(%rsp)
	vmovss	%xmm8, 624(%rsp)
	vmovss	%xmm15, 800(%rsp)
	vmovss	%xmm14, 608(%rsp)
	vmovss	%xmm13, 160(%rsp)
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm0, %xmm11
	vmovaps	%xmm3, %xmm10
	vmovss	608(%r13), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	612(%r13), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	616(%r13), %xmm0
	vmovss	%xmm0, 600(%rsp)
	vmovss	620(%r13), %xmm0
	vmovss	%xmm0, 592(%rsp)
	vmovss	624(%r13), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	628(%r13), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	632(%r13), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	636(%r13), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	640(%r13), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	644(%r13), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	648(%r13), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	652(%r13), %xmm0
	vmovss	%xmm0, 272(%rsp)
	vmovss	656(%r13), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	660(%r13), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	664(%r13), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	668(%r13), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	672(%r13), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	676(%r13), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	680(%r13), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	684(%r13), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	688(%r13), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	692(%r13), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	696(%r13), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	700(%r13), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	704(%r13), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	708(%r13), %xmm0
	vmovss	%xmm0, 264(%rsp)
	vmovss	712(%r13), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	716(%r13), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	720(%r13), %xmm0
	vmovss	%xmm0, 448(%rsp)
	vmovss	724(%r13), %xmm0
	vmovss	%xmm0, 440(%rsp)
	vmovss	728(%r13), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	732(%r13), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	736(%r13), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	740(%r13), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	744(%r13), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	748(%r13), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	752(%r13), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	756(%r13), %xmm0
	vmovss	%xmm0, 140(%rsp)
	vmovss	760(%r13), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	764(%r13), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	768(%r13), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	772(%r13), %xmm0
	vmovss	%xmm0, 124(%rsp)
	.loc	5 0 12 is_stmt 0
.Ltmp11510:
	.p2align	4
.LBB36_425:
	vmovss	(%rsp), %xmm6
	vmovss	160(%rsp), %xmm14
	vmovss	608(%rsp), %xmm15
	vmovss	800(%rsp), %xmm0
	vmovss	624(%rsp), %xmm1
	vmovss	144(%rsp), %xmm3
	.loc	5 568 12 is_stmt 1
	leaq	(%rsi,%rdx), %rdi
	cmpq	%r14, %rdi
	ja	.LBB36_521
.Ltmp11511:
	.loc	5 0 12 is_stmt 0
	vmovss	%xmm3, 32(%rsp)
	vmovss	%xmm1, 368(%rsp)
	vmovss	%xmm0, 48(%rsp)
	.loc	1 1762 0 is_stmt 1
	movq	%rcx, %rsi
	addq	%rdx, %rsi
.Ltmp11512:
	.loc	52 51 9
	je	.LBB36_81
	.loc	52 0 9 is_stmt 0
	vmovaps	%xmm2, %xmm13
	vmovaps	%xmm11, %xmm5
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm7, %xmm3
	vmovaps	%xmm4, %xmm1
	movq	696(%rsp), %rsi
	.loc	52 51 9
	vmovss	(%rsi,%rdx,4), %xmm8
	vbroadcastss	.LCPI36_0(%rip), %xmm12
.Ltmp11513:
	.loc	52 103 24 is_stmt 1
	vandps	%xmm2, %xmm12, %xmm2
	vmovaps	%xmm2, 144(%rsp)
.Ltmp11514:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm8, %xmm2
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp11515:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11516:
	.loc	52 71 9
	vmulss	360(%rsp), %xmm8, %xmm4
.Ltmp11517:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11518:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm8, %xmm7
.Ltmp11519:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
	vmovss	%xmm8, (%rsp)
.Ltmp11520:
	.loc	52 71 9
	vmulss	256(%rsp), %xmm8, %xmm10
.Ltmp11521:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	%xmm6, 208(%rsp)
.Ltmp11522:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm6, %xmm11
.Ltmp11523:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11524:
	.loc	52 71 9
	vmulss	192(%rsp), %xmm6, %xmm11
.Ltmp11525:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11526:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm6, %xmm11
.Ltmp11527:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11528:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm6, %xmm11
.Ltmp11529:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11530:
	.loc	52 71 9
	vmulss	600(%rsp), %xmm1, %xmm11
.Ltmp11531:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11532:
	.loc	52 71 9
	vmulss	592(%rsp), %xmm1, %xmm11
.Ltmp11533:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11534:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm1, %xmm11
.Ltmp11535:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11536:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm1, %xmm11
.Ltmp11537:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11538:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm3, %xmm11
.Ltmp11539:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11540:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm3, %xmm11
.Ltmp11541:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11542:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm3, %xmm11
.Ltmp11543:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11544:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm3, %xmm11
.Ltmp11545:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11546:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm0, %xmm11
.Ltmp11547:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11548:
	.loc	52 71 9
	vmulss	272(%rsp), %xmm0, %xmm11
.Ltmp11549:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11550:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm0, %xmm11
.Ltmp11551:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11552:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm0, %xmm11
.Ltmp11553:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovaps	%xmm5, %xmm6
.Ltmp11554:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm5, %xmm11
.Ltmp11555:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11556:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm5, %xmm11
.Ltmp11557:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11558:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm5, %xmm11
.Ltmp11559:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11560:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm5, %xmm11
.Ltmp11561:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11562:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm13, %xmm11
.Ltmp11563:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11564:
	.loc	52 71 9
	vmulss	68(%rsp), %xmm13, %xmm11
.Ltmp11565:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11566:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm13, %xmm11
.Ltmp11567:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11568:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm13, %xmm11
.Ltmp11569:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11570:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm14, %xmm11
.Ltmp11571:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11572:
	.loc	52 71 9
	vmulss	16(%rsp), %xmm14, %xmm11
.Ltmp11573:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11574:
	.loc	52 71 9
	vmulss	12(%rsp), %xmm14, %xmm11
.Ltmp11575:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11576:
	.loc	52 71 9
	vmulss	264(%rsp), %xmm14, %xmm11
.Ltmp11577:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11578:
	.loc	52 71 9
	vmulss	384(%rsp), %xmm15, %xmm11
.Ltmp11579:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11580:
	.loc	52 71 9
	vmulss	72(%rsp), %xmm15, %xmm11
.Ltmp11581:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11582:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm15, %xmm11
.Ltmp11583:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11584:
	.loc	52 71 9
	vmulss	440(%rsp), %xmm15, %xmm11
.Ltmp11585:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	48(%rsp), %xmm8
.Ltmp11586:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm8, %xmm11
.Ltmp11587:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11588:
	.loc	52 71 9
	vmulss	88(%rsp), %xmm8, %xmm11
.Ltmp11589:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11590:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm8, %xmm11
.Ltmp11591:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11592:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm8, %xmm11
.Ltmp11593:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	368(%rsp), %xmm5
.Ltmp11594:
	.loc	52 71 9
	vmulss	20(%rsp), %xmm5, %xmm11
.Ltmp11595:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11596:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm5, %xmm11
.Ltmp11597:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11598:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm5, %xmm11
.Ltmp11599:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11600:
	.loc	52 71 9
	vmulss	140(%rsp), %xmm5, %xmm11
.Ltmp11601:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	32(%rsp), %xmm9
.Ltmp11602:
	.loc	52 71 9
	vmulss	136(%rsp), %xmm9, %xmm11
.Ltmp11603:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11604:
	.loc	52 71 9
	vmulss	132(%rsp), %xmm9, %xmm11
.Ltmp11605:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11606:
	.loc	52 71 9
	vmulss	128(%rsp), %xmm9, %xmm11
.Ltmp11607:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11608:
	.loc	52 71 9
	vmulss	124(%rsp), %xmm9, %xmm11
.Ltmp11609:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovaps	%xmm12, %xmm11
.Ltmp11610:
	.loc	52 103 24
	vandps	%xmm2, %xmm12, %xmm2
	vmovaps	144(%rsp), %xmm12
.Ltmp11611:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm12, %xmm2
	vmovss	208(%rsp), %xmm12
.Ltmp11612:
	.loc	52 103 24
	vandps	%xmm4, %xmm11, %xmm4
.Ltmp11613:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm2, %xmm2
.Ltmp11614:
	.loc	52 103 24
	vandps	%xmm7, %xmm11, %xmm4
.Ltmp11615:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm2, %xmm2
.Ltmp11616:
	.loc	52 103 24
	vandps	%xmm11, %xmm10, %xmm4
.Ltmp11617:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm2, %xmm2
.Ltmp11618:
	.loc	52 56 9
	vmovss	%xmm2, 2648(%rsp,%rdx,4)
.Ltmp11619:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm5, 144(%rsp)
	vmovss	%xmm8, 624(%rsp)
	vmovss	%xmm15, 800(%rsp)
	vmovss	%xmm14, 608(%rsp)
	vmovss	%xmm13, 160(%rsp)
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm0, %xmm11
	vmovaps	%xmm3, %xmm10
	vmovaps	%xmm1, %xmm7
	vmovaps	%xmm12, %xmm4
.Ltmp11620:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %rax
	movq	56(%rsp), %rsi
.Ltmp11621:
	.loc	3 900 12
	jne	.LBB36_425
.Ltmp11622:
.LBB36_428:
	.loc	3 0 12 is_stmt 0
	vmovss	(%rsp), %xmm2
	.loc	1 1765 5 is_stmt 1
	vmovss	%xmm2, 708(%rsp)
	vmovss	%xmm12, 712(%rsp)
	vmovss	%xmm1, 716(%rsp)
	vmovss	%xmm3, 720(%rsp)
	vmovss	%xmm0, 724(%rsp)
	vmovss	%xmm6, 728(%rsp)
	vmovss	%xmm13, 732(%rsp)
	vmovss	%xmm14, 736(%rsp)
	vmovss	%xmm15, 740(%rsp)
	vmovss	%xmm8, 744(%rsp)
	vmovss	%xmm5, 748(%rsp)
	vmovss	%xmm9, 752(%rsp)
.Ltmp11623:
	.loc	1 1759 23
	vmovss	496(%rsp), %xmm0
	vmovss	%xmm0, (%rsp)
	vmovss	500(%rsp), %xmm4
	vmovss	504(%rsp), %xmm7
	vmovss	508(%rsp), %xmm3
	vmovss	512(%rsp), %xmm0
	vmovss	516(%rsp), %xmm6
	vmovss	520(%rsp), %xmm13
	vmovss	524(%rsp), %xmm14
	vmovss	528(%rsp), %xmm15
	vmovss	532(%rsp), %xmm8
	vmovss	536(%rsp), %xmm5
.Ltmp11624:
	.loc	10 1916 50
	cmpq	%rsi, 232(%rsp)
.Ltmp11625:
	.loc	3 900 12
	jne	.LBB36_430
.Ltmp11626:
	.loc	1 0 0 is_stmt 0
	vmovss	540(%rsp), %xmm9
	vmovaps	%xmm7, %xmm1
	vmovaps	%xmm4, %xmm12
.Ltmp11627:
	.loc	3 900 12
	jmp	.LBB36_434
.Ltmp11628:
.LBB36_430:
	.loc	3 0 12
	vmovss	584(%r13), %xmm2
	vmovss	%xmm2, 288(%rsp)
	vmovss	588(%r13), %xmm2
	vmovss	%xmm2, 360(%rsp)
	vmovss	592(%r13), %xmm2
	vmovss	%xmm2, 424(%rsp)
	vmovss	596(%r13), %xmm2
	vmovss	%xmm2, 256(%rsp)
	vmovss	600(%r13), %xmm1
	vmovss	%xmm1, 200(%rsp)
	vmovss	604(%r13), %xmm1
	vmovss	%xmm1, 192(%rsp)
	movq	184(%rsp), %rcx
.Ltmp11629:
	.loc	5 568 12 is_stmt 1
	addq	%rsi, %rcx
	xorl	%edx, %edx
	vmovss	%xmm5, 144(%rsp)
	vmovss	%xmm8, 624(%rsp)
	vmovss	%xmm15, 800(%rsp)
	vmovss	%xmm14, 608(%rsp)
	vmovss	%xmm13, 160(%rsp)
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm0, %xmm11
	vmovaps	%xmm3, %xmm10
	vmovss	608(%r13), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	612(%r13), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	616(%r13), %xmm0
	vmovss	%xmm0, 600(%rsp)
	vmovss	620(%r13), %xmm0
	vmovss	%xmm0, 592(%rsp)
	vmovss	624(%r13), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	628(%r13), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	632(%r13), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	636(%r13), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	640(%r13), %xmm0
	vmovss	%xmm0, 280(%rsp)
	vmovss	644(%r13), %xmm0
	vmovss	%xmm0, 328(%rsp)
	vmovss	648(%r13), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	652(%r13), %xmm0
	vmovss	%xmm0, 272(%rsp)
	vmovss	656(%r13), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	660(%r13), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	664(%r13), %xmm0
	vmovss	%xmm0, 472(%rsp)
	vmovss	668(%r13), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	672(%r13), %xmm0
	vmovss	%xmm0, 312(%rsp)
	vmovss	676(%r13), %xmm0
	vmovss	%xmm0, 464(%rsp)
	vmovss	680(%r13), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	684(%r13), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	688(%r13), %xmm0
	vmovss	%xmm0, 456(%rsp)
	vmovss	692(%r13), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	696(%r13), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	700(%r13), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	704(%r13), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	708(%r13), %xmm0
	vmovss	%xmm0, 264(%rsp)
	vmovss	712(%r13), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	716(%r13), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	720(%r13), %xmm0
	vmovss	%xmm0, 448(%rsp)
	vmovss	724(%r13), %xmm0
	vmovss	%xmm0, 440(%rsp)
	vmovss	728(%r13), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	732(%r13), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	736(%r13), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	740(%r13), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	744(%r13), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	748(%r13), %xmm0
	vmovss	%xmm0, 664(%rsp)
	vmovss	752(%r13), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	756(%r13), %xmm0
	vmovss	%xmm0, 140(%rsp)
	vmovss	760(%r13), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	764(%r13), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	768(%r13), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	772(%r13), %xmm0
	vmovss	%xmm0, 124(%rsp)
	.loc	5 0 12 is_stmt 0
.Ltmp11630:
	.p2align	4
.LBB36_431:
	vmovss	(%rsp), %xmm6
	vmovss	160(%rsp), %xmm14
	vmovss	608(%rsp), %xmm15
	vmovss	800(%rsp), %xmm0
	vmovss	624(%rsp), %xmm1
	vmovss	144(%rsp), %xmm3
	.loc	5 568 12 is_stmt 1
	leaq	(%rsi,%rdx), %rdi
	cmpq	296(%rsp), %rdi
	ja	.LBB36_440
.Ltmp11631:
	.loc	5 0 12 is_stmt 0
	vmovss	%xmm3, 32(%rsp)
	vmovss	%xmm1, 368(%rsp)
	vmovss	%xmm0, 48(%rsp)
	.loc	1 1762 0 is_stmt 1
	movq	%rcx, %rsi
	addq	%rdx, %rsi
.Ltmp11632:
	.loc	52 51 9
	je	.LBB36_81
	.loc	52 0 9 is_stmt 0
	vmovaps	%xmm2, %xmm13
	vmovaps	%xmm11, %xmm5
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm7, %xmm3
	vmovaps	%xmm4, %xmm1
	movq	672(%rsp), %rsi
	.loc	52 51 9
	vmovss	(%rsi,%rdx,4), %xmm8
	vbroadcastss	.LCPI36_0(%rip), %xmm12
.Ltmp11633:
	.loc	52 103 24 is_stmt 1
	vandps	%xmm2, %xmm12, %xmm2
	vmovaps	%xmm2, 144(%rsp)
.Ltmp11634:
	.loc	52 71 9
	vmulss	288(%rsp), %xmm8, %xmm2
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp11635:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11636:
	.loc	52 71 9
	vmulss	360(%rsp), %xmm8, %xmm4
.Ltmp11637:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11638:
	.loc	52 71 9
	vmulss	424(%rsp), %xmm8, %xmm7
.Ltmp11639:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
	vmovss	%xmm8, (%rsp)
.Ltmp11640:
	.loc	52 71 9
	vmulss	256(%rsp), %xmm8, %xmm10
.Ltmp11641:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	%xmm6, 208(%rsp)
.Ltmp11642:
	.loc	52 71 9
	vmulss	200(%rsp), %xmm6, %xmm11
.Ltmp11643:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11644:
	.loc	52 71 9
	vmulss	192(%rsp), %xmm6, %xmm11
.Ltmp11645:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11646:
	.loc	52 71 9
	vmulss	416(%rsp), %xmm6, %xmm11
.Ltmp11647:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11648:
	.loc	52 71 9
	vmulss	248(%rsp), %xmm6, %xmm11
.Ltmp11649:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11650:
	.loc	52 71 9
	vmulss	600(%rsp), %xmm1, %xmm11
.Ltmp11651:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11652:
	.loc	52 71 9
	vmulss	592(%rsp), %xmm1, %xmm11
.Ltmp11653:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11654:
	.loc	52 71 9
	vmulss	240(%rsp), %xmm1, %xmm11
.Ltmp11655:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11656:
	.loc	52 71 9
	vmulss	352(%rsp), %xmm1, %xmm11
.Ltmp11657:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11658:
	.loc	52 71 9
	vmulss	344(%rsp), %xmm3, %xmm11
.Ltmp11659:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11660:
	.loc	52 71 9
	vmulss	336(%rsp), %xmm3, %xmm11
.Ltmp11661:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11662:
	.loc	52 71 9
	vmulss	280(%rsp), %xmm3, %xmm11
.Ltmp11663:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11664:
	.loc	52 71 9
	vmulss	328(%rsp), %xmm3, %xmm11
.Ltmp11665:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11666:
	.loc	52 71 9
	vmulss	320(%rsp), %xmm0, %xmm11
.Ltmp11667:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11668:
	.loc	52 71 9
	vmulss	272(%rsp), %xmm0, %xmm11
.Ltmp11669:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11670:
	.loc	52 71 9
	vmulss	488(%rsp), %xmm0, %xmm11
.Ltmp11671:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11672:
	.loc	52 71 9
	vmulss	480(%rsp), %xmm0, %xmm11
.Ltmp11673:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovaps	%xmm5, %xmm6
.Ltmp11674:
	.loc	52 71 9
	vmulss	472(%rsp), %xmm5, %xmm11
.Ltmp11675:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11676:
	.loc	52 71 9
	vmulss	408(%rsp), %xmm5, %xmm11
.Ltmp11677:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11678:
	.loc	52 71 9
	vmulss	312(%rsp), %xmm5, %xmm11
.Ltmp11679:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11680:
	.loc	52 71 9
	vmulss	464(%rsp), %xmm5, %xmm11
.Ltmp11681:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11682:
	.loc	52 71 9
	vmulss	400(%rsp), %xmm13, %xmm11
.Ltmp11683:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11684:
	.loc	52 71 9
	vmulss	68(%rsp), %xmm13, %xmm11
.Ltmp11685:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11686:
	.loc	52 71 9
	vmulss	456(%rsp), %xmm13, %xmm11
.Ltmp11687:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11688:
	.loc	52 71 9
	vmulss	304(%rsp), %xmm13, %xmm11
.Ltmp11689:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11690:
	.loc	52 71 9
	vmulss	392(%rsp), %xmm14, %xmm11
.Ltmp11691:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11692:
	.loc	52 71 9
	vmulss	16(%rsp), %xmm14, %xmm11
.Ltmp11693:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11694:
	.loc	52 71 9
	vmulss	12(%rsp), %xmm14, %xmm11
.Ltmp11695:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11696:
	.loc	52 71 9
	vmulss	264(%rsp), %xmm14, %xmm11
.Ltmp11697:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
.Ltmp11698:
	.loc	52 71 9
	vmulss	384(%rsp), %xmm15, %xmm11
.Ltmp11699:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11700:
	.loc	52 71 9
	vmulss	72(%rsp), %xmm15, %xmm11
.Ltmp11701:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11702:
	.loc	52 71 9
	vmulss	448(%rsp), %xmm15, %xmm11
.Ltmp11703:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11704:
	.loc	52 71 9
	vmulss	440(%rsp), %xmm15, %xmm11
.Ltmp11705:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	48(%rsp), %xmm8
.Ltmp11706:
	.loc	52 71 9
	vmulss	96(%rsp), %xmm8, %xmm11
.Ltmp11707:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11708:
	.loc	52 71 9
	vmulss	88(%rsp), %xmm8, %xmm11
.Ltmp11709:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11710:
	.loc	52 71 9
	vmulss	28(%rsp), %xmm8, %xmm11
.Ltmp11711:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11712:
	.loc	52 71 9
	vmulss	24(%rsp), %xmm8, %xmm11
.Ltmp11713:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	368(%rsp), %xmm5
.Ltmp11714:
	.loc	52 71 9
	vmulss	20(%rsp), %xmm5, %xmm11
.Ltmp11715:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11716:
	.loc	52 71 9
	vmulss	664(%rsp), %xmm5, %xmm11
.Ltmp11717:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11718:
	.loc	52 71 9
	vmulss	656(%rsp), %xmm5, %xmm11
.Ltmp11719:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11720:
	.loc	52 71 9
	vmulss	140(%rsp), %xmm5, %xmm11
.Ltmp11721:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovss	32(%rsp), %xmm9
.Ltmp11722:
	.loc	52 71 9
	vmulss	136(%rsp), %xmm9, %xmm11
.Ltmp11723:
	.loc	52 61 9
	vaddss	%xmm2, %xmm11, %xmm2
.Ltmp11724:
	.loc	52 71 9
	vmulss	132(%rsp), %xmm9, %xmm11
.Ltmp11725:
	.loc	52 61 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp11726:
	.loc	52 71 9
	vmulss	128(%rsp), %xmm9, %xmm11
.Ltmp11727:
	.loc	52 61 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp11728:
	.loc	52 71 9
	vmulss	124(%rsp), %xmm9, %xmm11
.Ltmp11729:
	.loc	52 61 9
	vaddss	%xmm11, %xmm10, %xmm10
	vmovaps	%xmm12, %xmm11
.Ltmp11730:
	.loc	52 103 24
	vandps	%xmm2, %xmm12, %xmm2
	vmovaps	144(%rsp), %xmm12
.Ltmp11731:
	.loc	52 161 24
	vmaxss	%xmm2, %xmm12, %xmm2
	vmovss	208(%rsp), %xmm12
.Ltmp11732:
	.loc	52 103 24
	vandps	%xmm4, %xmm11, %xmm4
.Ltmp11733:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm2, %xmm2
.Ltmp11734:
	.loc	52 103 24
	vandps	%xmm7, %xmm11, %xmm4
.Ltmp11735:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm2, %xmm2
.Ltmp11736:
	.loc	52 103 24
	vandps	%xmm11, %xmm10, %xmm4
.Ltmp11737:
	.loc	52 161 24
	vmaxss	%xmm4, %xmm2, %xmm2
.Ltmp11738:
	.loc	52 56 9
	vmovss	%xmm2, 1616(%rsp,%rdx,4)
.Ltmp11739:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm5, 144(%rsp)
	vmovss	%xmm8, 624(%rsp)
	vmovss	%xmm15, 800(%rsp)
	vmovss	%xmm14, 608(%rsp)
	vmovss	%xmm13, 160(%rsp)
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm0, %xmm11
	vmovaps	%xmm3, %xmm10
	vmovaps	%xmm1, %xmm7
	vmovaps	%xmm12, %xmm4
.Ltmp11740:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %rax
	movq	56(%rsp), %rsi
.Ltmp11741:
	.loc	3 900 12
	jne	.LBB36_431
.Ltmp11742:
.LBB36_434:
	.loc	3 0 12 is_stmt 0
	vmovss	(%rsp), %xmm2
	.loc	1 1765 5 is_stmt 1
	vmovss	%xmm2, 496(%rsp)
	vmovss	%xmm12, 500(%rsp)
	vmovss	%xmm1, 504(%rsp)
	vmovss	%xmm3, 508(%rsp)
	vmovss	%xmm0, 512(%rsp)
	vmovss	%xmm6, 516(%rsp)
	vmovss	%xmm13, 520(%rsp)
	vmovss	%xmm14, 524(%rsp)
	vmovss	%xmm15, 528(%rsp)
	vmovss	%xmm8, 532(%rsp)
	vmovss	%xmm5, 536(%rsp)
	vmovss	%xmm9, 540(%rsp)
.Ltmp11743:
	.loc	10 1916 50
	cmpq	%rsi, 232(%rsp)
	jne	.LBB36_435
.Ltmp11744:
.LBB36_421:
	.loc	1 0 0 is_stmt 0
	addq	$32, %rsi
	decq	%r10
	movq	112(%rsp), %rax
.Ltmp11745:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rax
	subq	$-128, 696(%rsp)
	subq	$-128, 672(%rsp)
	testq	%r10, %r10
	movq	232(%rsp), %rcx
	movl	$32, %edx
	jne	.LBB36_422
	jmp	.LBB36_417
.Ltmp11746:
.LBB36_435:
	.loc	8 0 20 is_stmt 0
	movq	%r11, 656(%rsp)
	movq	%r10, 664(%rsp)
	movl	%r9d, 48(%rsp)
	movl	%r8d, 200(%rsp)
	movq	1464(%rsp), %rdi
	movq	1472(%rsp), %r8
	movq	888(%rsp), %r10
	movq	896(%rsp), %r15
	movq	1408(%rsp), %r13
	movq	1416(%rsp), %rdx
	movq	1456(%rsp), %rsi
	movq	1424(%rsp), %rax
	movq	%rax, 320(%rsp)
	movq	1432(%rsp), %rax
	movq	%rax, 328(%rsp)
	movq	1448(%rsp), %rax
	movq	%rax, 272(%rsp)
	movq	1440(%rsp), %rax
	movq	%rax, 480(%rsp)
	movq	832(%rsp), %rcx
	movq	880(%rsp), %rax
	vmovss	1480(%rsp), %xmm10
	vmovss	904(%rsp), %xmm9
.Ltmp11747:
	.loc	1 1989 19 is_stmt 1
	cmpq	$1, %rsi
	movq	%rsi, 352(%rsp)
	adcq	$0, %rsi
	movq	%rsi, 344(%rsp)
	cmpq	$1, %rax
	movq	%rax, 472(%rsp)
	adcq	$0, %rax
	movq	%rax, 408(%rsp)
	vmovss	756(%rsp), %xmm15
	vmovss	760(%rsp), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	776(%rsp), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	544(%rsp), %xmm0
	vmovss	%xmm0, (%rsp)
	vmovss	548(%rsp), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	564(%rsp), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	796(%rsp), %xmm0
	vmovss	%xmm0, 488(%rsp)
	vmovss	584(%rsp), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	768(%rsp), %xmm12
	vmovss	764(%rsp), %xmm0
	vmovaps	%xmm0, 624(%rsp)
	vmovss	784(%rsp), %xmm2
	vmovss	772(%rsp), %xmm5
	vmovss	780(%rsp), %xmm0
	vmovaps	%xmm0, 800(%rsp)
	vmovss	556(%rsp), %xmm0
	vmovss	%xmm0, 72(%rsp)
	movq	840(%rsp), %rbp
	movq	848(%rsp), %rax
	movq	%rax, 464(%rsp)
	movq	856(%rsp), %rax
	movq	%rax, 312(%rsp)
	movq	872(%rsp), %rax
	movq	%rax, 400(%rsp)
	movq	864(%rsp), %rax
	movq	%rax, 456(%rsp)
	movl	1484(%rsp), %eax
	movq	%rax, 360(%rsp)
	movl	908(%rsp), %eax
	movq	%rax, 160(%rsp)
	xorl	%esi, %esi
	vmovss	552(%rsp), %xmm0
	vmovaps	%xmm0, 608(%rsp)
	vmovss	572(%rsp), %xmm3
	vmovss	560(%rsp), %xmm0
	vmovss	%xmm0, 144(%rsp)
	vmovss	568(%rsp), %xmm4
	vmovss	792(%rsp), %xmm13
	vmovss	788(%rsp), %xmm8
	vmovss	580(%rsp), %xmm11
	vmovss	576(%rsp), %xmm1
	vmovss	%xmm9, 424(%rsp)
	vmovaps	%xmm10, %xmm6
	movq	648(%rsp), %r9
	movq	432(%rsp), %r11
	xorl	%r14d, %r14d
	vmovss	.LCPI36_1(%rip), %xmm0
	movq	104(%rsp), %rax
	movq	%r15, 440(%rsp)
	movq	%rdi, 96(%rsp)
	movq	%r8, 448(%rsp)
	movq	%r10, 88(%rsp)
	jmp	.LBB36_436
.LBB36_499:
	.loc	1 0 19 is_stmt 0
	movq	%rbx, 160(%rsp)
.Ltmp11748:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
	vmovaps	368(%rsp), %xmm14
.Ltmp11749:
	.loc	1 1660 5
	vmovss	%xmm14, 788(%rsp)
	vmovss	32(%rsp), %xmm11
.Ltmp11750:
	.loc	1 1654 5
	vmovss	%xmm11, 580(%rsp)
	vmovaps	208(%rsp), %xmm6
.Ltmp11751:
	.loc	1 1660 5
	vmovss	%xmm6, 576(%rsp)
.Ltmp11752:
	.loc	1 853 0
	vmovss	%xmm15, 756(%rsp)
	vmovaps	624(%rsp), %xmm9
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm9, 764(%rsp)
.Ltmp11753:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm2, 784(%rsp)
.Ltmp11754:
	.loc	1 853 0
	vmovss	%xmm5, 772(%rsp)
	vmovaps	800(%rsp), %xmm9
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm9, 780(%rsp)
	vmovss	(%rsp), %xmm9
.Ltmp11755:
	.loc	1 853 0
	vmovss	%xmm9, 544(%rsp)
	vmovaps	608(%rsp), %xmm9
	.loc	1 0 0
	vmovss	%xmm9, 552(%rsp)
.Ltmp11756:
	.loc	1 851 0 is_stmt 1
	vmovss	%xmm3, 572(%rsp)
	vmovss	144(%rsp), %xmm9
.Ltmp11757:
	.loc	1 853 0
	vmovss	%xmm9, 560(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm4, 568(%rsp)
	vmovaps	%xmm12, %xmm9
	vmovaps	%xmm7, %xmm10
	vmovss	%xmm1, 72(%rsp)
	vmovaps	%xmm6, %xmm1
	vmovss	%xmm12, 424(%rsp)
	vmovaps	%xmm8, %xmm12
	vmovaps	%xmm14, %xmm8
	vmovaps	%xmm7, %xmm6
.Ltmp11758:
.LBB36_500:
	movq	432(%rsp), %rsi
	movq	264(%rsp), %rdi
	movq	384(%rsp), %r8
	addq	%r8, %rdi
.Ltmp11759:
	.loc	1 2061 39 is_stmt 1
	addq	%r8, %rsi
.Ltmp11760:
	.loc	1 1148 8
	cmpq	%r12, %rsi
	movq	%r12, %rax
	movl	$0, %r14d
	cmovbq	%r14, %rax
	subq	%rax, %rsi
	movq	648(%rsp), %r9
.Ltmp11761:
	.loc	1 2062 39
	addq	%r8, %r9
	movq	40(%rsp), %rax
.Ltmp11762:
	.loc	1 1148 8
	cmpq	%rax, %r9
	cmovbq	%r14, %rax
	subq	%rax, %r9
	movq	104(%rsp), %rax
.Ltmp11763:
	.loc	1 1989 19
	cmpq	%rax, %rdi
	movq	%rsi, %r11
	movq	%rdi, %rsi
	movq	96(%rsp), %rdi
	movq	88(%rsp), %r10
	jae	.LBB36_420
.LBB36_436:
	.loc	1 0 19 is_stmt 0
	movq	%rsi, 264(%rsp)
	.loc	1 1994 21 is_stmt 1
	subq	%rsi, %rax
	movq	936(%rsp), %r15
.Ltmp11764:
	.loc	1 1575 16
	movq	544(%r15), %r8
.Ltmp11765:
	.loc	1 1576 16
	movq	552(%r15), %rbx
.Ltmp11766:
	.loc	1 1577 25
	leaq	1(%r11), %rsi
.Ltmp11767:
	.loc	1 1148 8
	cmpq	%r8, %rsi
	movq	%r8, %rsi
	cmovbq	%r14, %rsi
	negq	%rsi
.Ltmp11768:
	.loc	1 1578 28
	leaq	(%rdi,%r11), %r15
.Ltmp11769:
	.loc	1 1148 8
	cmpq	%r8, %r15
	movq	%r8, %rdi
	cmovbq	%r14, %rdi
	subq	%rdi, %r15
	movq	%r15, 256(%rsp)
.Ltmp11770:
	.loc	1 1579 29
	addq	%r11, %r10
.Ltmp11771:
	.loc	1 1148 8
	cmpq	%r8, %r10
	movq	%r8, %rdi
	cmovbq	%r14, %rdi
	subq	%rdi, %r10
	movq	%r10, 416(%rsp)
	movq	448(%rsp), %rdi
.Ltmp11772:
	.loc	1 1580 33
	leaq	(%rdi,%r11), %r10
.Ltmp11773:
	.loc	1 1148 8
	cmpq	%r8, %r10
	movq	%r8, %rdi
	cmovbq	%r14, %rdi
	subq	%rdi, %r10
	movq	%r10, 192(%rsp)
	movq	440(%rsp), %rdi
.Ltmp11774:
	.loc	1 1581 34
	leaq	(%rdi,%r11), %r15
.Ltmp11775:
	.loc	1 1148 8
	cmpq	%r8, %r15
	movq	%r8, %rdi
	cmovbq	%r14, %rdi
.Ltmp11776:
	.loc	1 1148 8 is_stmt 0
	leaq	(%r11,%rsi), %r14
	incq	%r14
.Ltmp11777:
	.loc	1 1148 8
	subq	%rdi, %r15
.Ltmp11778:
	.loc	1 1583 14 is_stmt 1
	movq	%r8, %r10
	movq	%r11, 432(%rsp)
	subq	%r11, %r10
.Ltmp11779:
	.loc	10 1078 5
	cmpq	%rax, %r10
	cmovbq	%r10, %rax
	movq	%r9, 648(%rsp)
.Ltmp11780:
	.loc	1 1584 14
	subq	%r9, %rbx
.Ltmp11781:
	.loc	10 1078 5
	cmpq	%rax, %rbx
	movq	%rbx, 240(%rsp)
	cmovbq	%rbx, %rax
.Ltmp11782:
	.loc	1 1585 14
	movq	%r8, %rbx
	movq	%r14, 248(%rsp)
	subq	%r14, %rbx
.Ltmp11783:
	.loc	10 1078 5
	cmpq	%rax, %rbx
	cmovbq	%rbx, %rax
.Ltmp11784:
	.loc	1 1586 14
	movq	%r8, %r14
	movq	256(%rsp), %rsi
	subq	%rsi, %r14
.Ltmp11785:
	.loc	10 1078 5
	cmpq	%rax, %r14
	cmovbq	%r14, %rax
.Ltmp11786:
	.loc	1 1587 14
	movq	%r8, %r11
	movq	416(%rsp), %rsi
	subq	%rsi, %r11
.Ltmp11787:
	.loc	10 1078 5
	cmpq	%rax, %r11
	cmovbq	%r11, %rax
.Ltmp11788:
	.loc	1 1588 14
	movq	%r8, %r9
	movq	192(%rsp), %rsi
	subq	%rsi, %r9
.Ltmp11789:
	.loc	10 1078 5
	cmpq	%rax, %r9
	cmovbq	%r9, %rax
	movq	%r15, 600(%rsp)
.Ltmp11790:
	.loc	1 1589 14
	subq	%r15, %r8
.Ltmp11791:
	.loc	10 1078 5
	cmpq	%rax, %r8
	cmovbq	%r8, %rax
	movq	56(%rsp), %rsi
	movq	264(%rsp), %rdi
.Ltmp11792:
	.loc	1 2000 28
	addq	%rsi, %rdi
	movq	%rax, 384(%rsp)
.Ltmp11793:
	.loc	1 2002 55
	addq	%rdi, %rax
.Ltmp11794:
	.loc	4 1050 16
	jb	.LBB36_438
	cmpq	960(%rsp), %rax
	ja	.LBB36_438
.Ltmp11795:
	.loc	5 451 16
	cmpq	296(%rsp), %rax
	ja	.LBB36_668
.Ltmp11796:
	.loc	14 304 12
	cmpq	$0, 384(%rsp)
	je	.LBB36_500
	.loc	14 0 12 is_stmt 0
	vmovaps	%xmm8, 368(%rsp)
	vmovss	%xmm11, 32(%rsp)
	vmovss	%xmm10, 24(%rsp)
	vmovaps	%xmm1, 208(%rsp)
	vmovss	%xmm9, 28(%rsp)
	.loc	14 304 12
	cmpq	%r11, %r14
	cmovbq	%r14, %r11
	cmpq	%r9, %r11
	cmovaeq	%r9, %r11
	movq	680(%rsp), %rsi
.Ltmp11797:
	.loc	1 0 0
	leaq	(%rsi,%rdi,4), %rax
	movq	%rax, 288(%rsp)
.Ltmp11798:
	.loc	14 304 12
	cmpq	%r8, %r11
	cmovaeq	%r8, %r11
	movq	928(%rsp), %rsi
.Ltmp11799:
	.loc	1 0 0
	leaq	(%rsi,%rdi,4), %r8
.Ltmp11800:
	.loc	14 304 12
	cmpq	%rbx, %r11
	cmovaeq	%rbx, %r11
	movq	264(%rsp), %rsi
.Ltmp11801:
	.loc	1 0 0
	leaq	(%rsp,%rsi,4), %rax
	addq	$2648, %rax
	movq	%rax, 592(%rsp)
.Ltmp11802:
	.loc	14 304 12
	cmpq	%r10, %r11
	cmovaeq	%r10, %r11
	movq	240(%rsp), %rax
	cmpq	%rax, %r11
	cmovaeq	%rax, %r11
	movq	824(%rsp), %rax
	subq	%rsi, %rax
	cmpq	%rax, %r11
	cmovbq	%r11, %rax
	movq	%rax, 280(%rsp)
.Ltmp11803:
	.loc	1 0 0
	leaq	1616(%rsp,%rsi,4), %rax
	movq	%rax, 240(%rsp)
	vmovss	72(%rsp), %xmm1
	vmovss	%xmm12, 20(%rsp)
	vmovaps	%xmm12, %xmm8
	xorl	%r14d, %r14d
	movl	200(%rsp), %r9d
	vxorps	%xmm11, %xmm11, %xmm11
	movq	160(%rsp), %rbx
	movq	%r8, 336(%rsp)
	.p2align	4
.LBB36_445:
.Ltmp11804:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm0, %xmm8, %xmm9
.Ltmp11805:
	.loc	52 124 14
	vucomiss	%xmm11, %xmm9
	movq	432(%rsp), %rsi
	vxorps	%xmm7, %xmm7, %xmm7
.Ltmp11806:
	.loc	52 161 24
	ja	.LBB36_446
.Ltmp11807:
	.loc	52 0 24 is_stmt 0
	vmovss	304(%rsp), %xmm15
	jmp	.LBB36_448
	.p2align	4
.LBB36_446:
	.loc	1 853 0 is_stmt 1
	vaddss	624(%rsp), %xmm15, %xmm15
.Ltmp11808:
.LBB36_448:
	.loc	1 0 0 is_stmt 0
	vmovss	.LCPI36_2(%rip), %xmm11
	vmovaps	368(%rsp), %xmm8
	vmovaps	%xmm5, %xmm12
.Ltmp11809:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm0, %xmm2, %xmm14
.Ltmp11810:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm14
.Ltmp11811:
	.loc	52 161 24
	ja	.LBB36_449
.Ltmp11812:
	.loc	52 0 24 is_stmt 0
	vmovss	392(%rsp), %xmm12
.Ltmp11813:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm0, %xmm1, %xmm10
.Ltmp11814:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm10
.Ltmp11815:
	.loc	52 161 24
	jbe	.LBB36_453
.Ltmp11816:
.LBB36_452:
	.loc	52 0 24 is_stmt 0
	vmovss	(%rsp), %xmm1
	.loc	1 853 0 is_stmt 1
	vaddss	608(%rsp), %xmm1, %xmm1
	jmp	.LBB36_454
.Ltmp11817:
	.loc	1 0 0 is_stmt 0
.Ltmp11818:
	.p2align	4
.LBB36_449:
	.loc	1 853 0 is_stmt 1
	vaddss	800(%rsp), %xmm12, %xmm12
.Ltmp11819:
	.loc	52 66 9
	vaddss	%xmm0, %xmm1, %xmm10
.Ltmp11820:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm10
.Ltmp11821:
	.loc	52 161 24
	ja	.LBB36_452
.Ltmp11822:
.LBB36_453:
	.loc	52 0 24 is_stmt 0
	vmovss	16(%rsp), %xmm1
.LBB36_454:
	vmovss	%xmm1, (%rsp)
.Ltmp11823:
	.loc	52 66 9 is_stmt 1
	vaddss	%xmm0, %xmm3, %xmm0
.Ltmp11824:
	.loc	52 124 14
	vucomiss	%xmm7, %xmm0
	movq	%rbx, 160(%rsp)
.Ltmp11825:
	.loc	52 161 24
	ja	.LBB36_455
.Ltmp11826:
	.loc	52 0 24 is_stmt 0
	vmovss	12(%rsp), %xmm1
	jmp	.LBB36_457
	.p2align	4
.LBB36_455:
	vmovss	144(%rsp), %xmm1
	.loc	1 853 0 is_stmt 1
	vaddss	%xmm4, %xmm1, %xmm1
.Ltmp11827:
.LBB36_457:
	.loc	1 0 0 is_stmt 0
	vmovss	%xmm1, 144(%rsp)
	movq	592(%rsp), %rax
.Ltmp11828:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rax,%r14,4), %xmm1
	movq	240(%rsp), %rax
.Ltmp11829:
	.loc	52 51 9 is_stmt 0
	vmovss	(%rax,%r14,4), %xmm2
.Ltmp11830:
	.loc	52 124 14 is_stmt 1
	vucomiss	%xmm1, %xmm2
.Ltmp11831:
	.loc	41 1244 18
	vmovd	%xmm2, %r15d
.Ltmp11832:
	.loc	41 1244 18 is_stmt 0
	vmovd	%xmm1, %eax
.Ltmp11833:
	.loc	52 161 24 is_stmt 1
	movl	%r15d, %ebx
	cmovbel	%eax, %ebx
.Ltmp11834:
	.loc	1 1502 26
	leaq	(%r14,%rsi), %r10
.Ltmp11835:
	.loc	4 1050 16
	cmpq	%rdx, %r10
	jae	.LBB36_669
.Ltmp11836:
	.loc	1 0 0 is_stmt 0
	andl	816(%rsp), %ebx
.Ltmp11837:
	.loc	52 161 44 is_stmt 1
	andl	%r9d, %eax
	.loc	52 161 24 is_stmt 0
	orl	%ebx, %eax
.Ltmp11838:
	.loc	41 1291 18 is_stmt 1
	vmovd	%eax, %xmm2
.Ltmp11839:
	.loc	52 124 14
	vucomiss	%xmm15, %xmm2
	vmovaps	%xmm11, %xmm1
.Ltmp11840:
	.loc	52 161 24
	jbe	.LBB36_460
.Ltmp11841:
	.loc	52 76 9
	vdivss	%xmm2, %xmm15, %xmm1
.Ltmp11842:
.LBB36_460:
	.loc	52 0 9 is_stmt 0
	movq	288(%rsp), %rax
	vmovss	(%rax,%r14,4), %xmm3
	vmovss	(%r8,%r14,4), %xmm2
	movq	256(%rsp), %rax
	leaq	(%r14,%rax), %rdi
.Ltmp11843:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm1, (%r13,%r10,4)
.Ltmp11844:
	.loc	4 1050 16
	cmpq	%rdx, %rdi
	movq	360(%rsp), %r9
	jae	.LBB36_461
.Ltmp11845:
	.loc	52 51 9
	vmovss	(%r13,%rdi,4), %xmm1
	vmovaps	%xmm1, %xmm7
.Ltmp11846:
	.loc	1 1206 22
	testl	%r9d, %r9d
	je	.LBB36_464
.Ltmp11847:
	.loc	1 0 22 is_stmt 0
	vminss	%xmm1, %xmm6, %xmm7
.LBB36_464:
	movq	248(%rsp), %rax
	leaq	(%r14,%rax), %r11
.Ltmp11848:
	movl	%r9d, %r9d
.Ltmp11849:
	.loc	1 1212 20 is_stmt 1
	incq	%r9
	movq	344(%rsp), %rax
	cmpq	352(%rsp), %r9
.Ltmp11850:
	.loc	1 1213 22
	jne	.LBB36_465
	.loc	1 0 22 is_stmt 0
.Ltmp11851:
	.p2align	4
.LBB36_467:
.Ltmp11852:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB36_461
.Ltmp11853:
	.loc	52 161 24
	vminss	(%r13,%rdi,4), %xmm1, %xmm1
.Ltmp11854:
	.loc	52 56 9
	vmovss	%xmm1, (%r13,%rdi,4)
.Ltmp11855:
	.loc	1 1224 16
	testq	%rdi, %rdi
	cmoveq	%r12, %rdi
	.loc	1 1227 13
	decq	%rdi
.Ltmp11856:
	.loc	10 1916 50
	decq	%rax
.Ltmp11857:
	.loc	3 900 12
	jne	.LBB36_467
.Ltmp11858:
	.loc	3 0 12 is_stmt 0
	xorl	%r9d, %r9d
	vmovaps	%xmm7, %xmm1
	jmp	.LBB36_470
	.p2align	4
.LBB36_465:
.Ltmp11859:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rdx, %r11
	jae	.LBB36_670
.Ltmp11860:
	.loc	52 51 9
	vmovss	(%r13,%r11,4), %xmm1
.Ltmp11861:
	.loc	52 161 24
	vminss	%xmm7, %xmm1, %xmm1
.Ltmp11862:
.LBB36_470:
	.loc	52 0 24 is_stmt 0
	movq	192(%rsp), %rax
	leaq	(%r14,%rax), %rdi
.Ltmp11863:
	.loc	52 71 9 is_stmt 1
	vmulss	.LCPI36_3(%rip), %xmm1, %xmm1
.Ltmp11864:
	.loc	41 1783 9
	vroundss	$9, %xmm1, %xmm1, %xmm1
.Ltmp11865:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm1, %xmm1
	movq	328(%rsp), %rax
.Ltmp11866:
	.loc	4 1050 16
	cmpq	%rax, %rdi
	jae	.LBB36_471
.Ltmp11867:
	.loc	1 0 0 is_stmt 0
	leaq	(%r14,%rsi), %r8
	incq	%r8
.Ltmp11868:
	.loc	52 61 9 is_stmt 1
	vaddss	%xmm1, %xmm13, %xmm6
	movq	320(%rsp), %rsi
.Ltmp11869:
	.loc	52 66 9
	vsubss	(%rsi,%rdi,4), %xmm6, %xmm6
	cmpq	%rax, %r8
.Ltmp11870:
	.loc	4 1050 16
	ja	.LBB36_474
.Ltmp11871:
	.loc	4 0 16 is_stmt 0
	movq	648(%rsp), %rax
	addq	%r14, %rax
.Ltmp11872:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm1, (%rsi,%r10,4)
	vmovaps	%xmm6, %xmm13
.Ltmp11873:
	.loc	52 76 9
	vdivss	488(%rsp), %xmm6, %xmm1
.Ltmp11874:
	.loc	52 66 9
	vsubss	%xmm1, %xmm11, %xmm1
.Ltmp11875:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm8, %xmm1, %xmm6
	vmovaps	%xmm12, %xmm5
.Ltmp11876:
	.loc	52 92 9 is_stmt 1
	vmulss	%xmm6, %xmm12, %xmm6
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11877:
	.loc	52 161 24
	vmaxss	%xmm6, %xmm1, %xmm6
.Ltmp11878:
	.loc	52 103 24
	vbroadcastss	.LCPI36_0(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm8
.Ltmp11879:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm8, %xmm8
	vandps	%xmm6, %xmm8, %xmm12
.Ltmp11880:
	.loc	4 1050 16
	cmpq	272(%rsp), %rax
	jae	.LBB36_671
.Ltmp11881:
	.loc	4 0 16 is_stmt 0
	movq	%r9, 360(%rsp)
.Ltmp11882:
	.loc	52 66 9 is_stmt 1
	vsubss	%xmm12, %xmm11, %xmm6
	movq	480(%rsp), %rsi
.Ltmp11883:
	.loc	52 51 9
	vmovss	(%rsi,%rax,4), %xmm8
.Ltmp11884:
	.loc	52 56 9
	vmovss	%xmm3, (%rsi,%rax,4)
.Ltmp11885:
	.loc	52 71 9
	vmulss	%xmm6, %xmm8, %xmm3
.Ltmp11886:
	.loc	41 1244 18
	vmovd	%xmm8, %edi
.Ltmp11887:
	.loc	52 161 24
	andl	84(%rsp), %edi
.Ltmp11888:
	.loc	41 1244 18
	vmovd	%xmm3, %r9d
.Ltmp11889:
	.loc	52 161 44
	andl	48(%rsp), %r9d
	.loc	52 161 24 is_stmt 0
	orl	%edi, %r9d
	movq	288(%rsp), %rsi
.Ltmp11890:
	.loc	52 56 9 is_stmt 1
	movl	%r9d, (%rsi,%r14,4)
	cmpq	%rbp, %r8
.Ltmp11891:
	.loc	4 1050 16
	ja	.LBB36_672
.Ltmp11892:
	.loc	4 0 16 is_stmt 0
	movl	200(%rsp), %r9d
	andl	%r9d, %r15d
	orl	%r15d, %ebx
	vmovd	%ebx, %xmm6
.Ltmp11893:
	.loc	52 124 14 is_stmt 1
	vucomiss	(%rsp), %xmm6
	vmovaps	%xmm11, %xmm3
.Ltmp11894:
	.loc	52 161 24
	jbe	.LBB36_480
.Ltmp11895:
	.loc	52 0 24 is_stmt 0
	vmovss	(%rsp), %xmm3
.Ltmp11896:
	.loc	52 76 9 is_stmt 1
	vdivss	%xmm6, %xmm3, %xmm3
.Ltmp11897:
.LBB36_480:
	.loc	52 0 9 is_stmt 0
	movq	416(%rsp), %rsi
	leaq	(%r14,%rsi), %rdi
.Ltmp11898:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm3, (%rcx,%r10,4)
.Ltmp11899:
	.loc	4 1050 16
	cmpq	%rbp, %rdi
	movq	160(%rsp), %rbx
	jae	.LBB36_481
.Ltmp11900:
	.loc	4 0 16 is_stmt 0
	vmovaps	%xmm12, 368(%rsp)
.Ltmp11901:
	.loc	52 51 9 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm3
	vmovaps	%xmm3, %xmm12
.Ltmp11902:
	.loc	1 1206 22
	testl	%ebx, %ebx
	je	.LBB36_485
.Ltmp11903:
	.loc	1 0 22 is_stmt 0
	vmovss	424(%rsp), %xmm6
	vminss	%xmm3, %xmm6, %xmm12
.LBB36_485:
	movl	%ebx, %ebx
.Ltmp11904:
	.loc	1 1212 20 is_stmt 1
	incq	%rbx
	movq	408(%rsp), %rsi
	cmpq	472(%rsp), %rbx
.Ltmp11905:
	.loc	1 1213 22
	jne	.LBB36_486
	.loc	1 0 22 is_stmt 0
.Ltmp11906:
	.p2align	4
.LBB36_488:
.Ltmp11907:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rbp, %rdi
	jae	.LBB36_489
.Ltmp11908:
	.loc	52 161 24
	vminss	(%rcx,%rdi,4), %xmm3, %xmm3
.Ltmp11909:
	.loc	52 56 9
	vmovss	%xmm3, (%rcx,%rdi,4)
.Ltmp11910:
	.loc	1 1224 16
	testq	%rdi, %rdi
	cmoveq	%r12, %rdi
	.loc	1 1227 13
	decq	%rdi
.Ltmp11911:
	.loc	10 1916 50
	decq	%rsi
.Ltmp11912:
	.loc	3 900 12
	jne	.LBB36_488
.Ltmp11913:
	.loc	3 0 12 is_stmt 0
	xorl	%ebx, %ebx
	vmovaps	%xmm12, %xmm3
	jmp	.LBB36_492
	.p2align	4
.LBB36_486:
.Ltmp11914:
	.loc	4 1050 16 is_stmt 1
	cmpq	%rbp, %r11
	jae	.LBB36_673
.Ltmp11915:
	.loc	52 51 9
	vmovss	(%rcx,%r11,4), %xmm3
.Ltmp11916:
	.loc	52 161 24
	vminss	%xmm12, %xmm3, %xmm3
.Ltmp11917:
.LBB36_492:
	.loc	52 0 24 is_stmt 0
	movq	600(%rsp), %rsi
	leaq	(%r14,%rsi), %rdi
.Ltmp11918:
	.loc	52 71 9 is_stmt 1
	vmulss	.LCPI36_3(%rip), %xmm3, %xmm3
.Ltmp11919:
	.loc	41 1783 9
	vroundss	$9, %xmm3, %xmm3, %xmm3
.Ltmp11920:
	.loc	52 71 9
	vmulss	.LCPI36_4(%rip), %xmm3, %xmm3
	movq	312(%rsp), %rsi
.Ltmp11921:
	.loc	4 1050 16
	cmpq	%rsi, %rdi
	movq	464(%rsp), %r11
	jae	.LBB36_493
.Ltmp11922:
	.loc	52 61 9
	vaddss	32(%rsp), %xmm3, %xmm6
.Ltmp11923:
	.loc	52 66 9
	vsubss	(%r11,%rdi,4), %xmm6, %xmm6
	cmpq	%rsi, %r8
.Ltmp11924:
	.loc	4 1050 16
	ja	.LBB36_674
.Ltmp11925:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rax), %rsi
.Ltmp11926:
	.loc	52 56 9 is_stmt 1
	vmovss	%xmm3, (%r11,%r10,4)
	vmovss	%xmm6, 32(%rsp)
.Ltmp11927:
	.loc	52 76 9
	vdivss	68(%rsp), %xmm6, %xmm3
.Ltmp11928:
	.loc	52 66 9
	vsubss	%xmm3, %xmm11, %xmm3
	vmovaps	208(%rsp), %xmm8
.Ltmp11929:
	.loc	52 66 9 is_stmt 0
	vsubss	%xmm8, %xmm3, %xmm6
.Ltmp11930:
	.loc	52 92 9 is_stmt 1
	vmulss	144(%rsp), %xmm6, %xmm6
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp11931:
	.loc	52 161 24
	vmaxss	%xmm6, %xmm3, %xmm3
.Ltmp11932:
	.loc	52 103 24
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp11933:
	.loc	52 166 24
	vcmpnltss	.LCPI36_5(%rip), %xmm1, %xmm1
	vandps	%xmm3, %xmm1, %xmm1
	cmpq	400(%rsp), %rsi
.Ltmp11934:
	.loc	4 1050 16
	ja	.LBB36_675
.Ltmp11935:
	.loc	4 0 16 is_stmt 0
	vmovaps	%xmm1, 208(%rsp)
.Ltmp11936:
	.loc	52 66 9 is_stmt 1
	vsubss	%xmm1, %xmm11, %xmm1
	movq	456(%rsp), %rsi
.Ltmp11937:
	.loc	52 51 9
	vmovss	(%rsi,%rax,4), %xmm3
.Ltmp11938:
	.loc	52 56 9
	vmovss	%xmm2, (%rsi,%rax,4)
.Ltmp11939:
	.loc	52 71 9
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp11940:
	.loc	41 1244 18
	vmovd	%xmm3, %eax
.Ltmp11941:
	.loc	52 161 24
	andl	84(%rsp), %eax
.Ltmp11942:
	.loc	41 1244 18
	vmovd	%xmm1, %esi
.Ltmp11943:
	.loc	52 161 44
	andl	48(%rsp), %esi
	.loc	52 161 24 is_stmt 0
	orl	%eax, %esi
	movq	336(%rsp), %r8
.Ltmp11944:
	.loc	52 56 9 is_stmt 1
	movl	%esi, (%r8,%r14,4)
.Ltmp11945:
	.loc	1 0 0 is_stmt 0
	incq	%r14
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp11946:
	vmaxss	%xmm11, %xmm9, %xmm8
	vcmpltss	%xmm9, %xmm11, %xmm1
	vmovaps	624(%rsp), %xmm2
	vandps	%xmm2, %xmm1, %xmm2
	vmovaps	%xmm2, 624(%rsp)
	vmaxss	%xmm11, %xmm14, %xmm2
	vcmpltss	%xmm14, %xmm11, %xmm1
	vmovaps	800(%rsp), %xmm3
	vandps	%xmm3, %xmm1, %xmm3
	vmovaps	%xmm3, 800(%rsp)
.Ltmp11947:
	vmaxss	%xmm11, %xmm10, %xmm1
	vcmpltss	%xmm10, %xmm11, %xmm3
	vmovaps	608(%rsp), %xmm6
	vandps	%xmm6, %xmm3, %xmm6
	vmovaps	%xmm6, 608(%rsp)
	vmaxss	%xmm11, %xmm0, %xmm3
	vcmpltss	%xmm0, %xmm11, %xmm0
	vandps	%xmm4, %xmm0, %xmm4
	vmovss	%xmm12, 424(%rsp)
	vmovaps	%xmm7, %xmm6
.Ltmp11948:
	.loc	14 304 12 is_stmt 1
	cmpq	280(%rsp), %r14
	vmovss	.LCPI36_1(%rip), %xmm0
	jne	.LBB36_445
	jmp	.LBB36_499
.Ltmp11949:
.LBB36_420:
	.loc	14 0 12 is_stmt 0
	movq	%r11, 432(%rsp)
	movq	%r9, 648(%rsp)
	vmovss	%xmm12, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
.Ltmp11950:
	vmovss	%xmm10, 1480(%rsp)
.Ltmp11951:
	vmovss	%xmm9, 904(%rsp)
	movq	360(%rsp), %rax
	movl	%eax, 1484(%rsp)
	movq	160(%rsp), %rax
	movl	%eax, 908(%rsp)
	movq	936(%rsp), %r13
	movq	960(%rsp), %r14
	movq	680(%rsp), %rbx
	movq	928(%rsp), %rbp
	movl	200(%rsp), %r8d
	movl	48(%rsp), %r9d
	movq	664(%rsp), %r10
	movq	656(%rsp), %r11
	movq	56(%rsp), %rsi
	jmp	.LBB36_421
.Ltmp11952:
.LBB36_595:
	movq	%rsi, (%rsp)
	movq	%r8, 56(%rsp)
	xorl	%eax, %eax
.LBB36_596:
	movl	$-1, %ecx
	xorl	%edx, %edx
	vbroadcastss	.LCPI36_0(%rip), %xmm0
	vmovss	.LCPI36_6(%rip), %xmm1
	xorl	%esi, %esi
	.p2align	4
.LBB36_597:
.Ltmp11953:
	.loc	52 103 24 is_stmt 1
	vmovss	(%rbp,%rsi,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp11954:
	.loc	52 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp11955:
	.loc	52 139 9
	cmovbel	%edx, %ecx
.Ltmp11956:
	.loc	19 2155 12
	incq	%rsi
	cmpq	%rsi, %r9
	jne	.LBB36_597
.Ltmp11957:
	.loc	19 0 12 is_stmt 0
	movl	%edi, 144(%rsp)
.Ltmp11958:
	.loc	17 185 12 is_stmt 1
	notl	%ecx
	xorl	%edx, %edx
	testl	$1065353216, %ecx
	setne	%dl
.Ltmp11959:
	.loc	17 211 5
	orl	%edx, %eax
	.loc	17 212 31
	movq	568(%r13), %rcx
.Ltmp11960:
	.loc	4 2428 13
	incq	%rcx
	movq	$-1, %rdx
	cmovneq	%rcx, %rdx
.Ltmp11961:
	.loc	17 211 5
	movl	%eax, 576(%r13)
	.loc	17 212 5
	movq	%rdx, 568(%r13)
.Ltmp11962:
	.loc	19 2155 12
	testq	%r14, %r14
.Ltmp11963:
	.loc	6 180 28
	je	.LBB36_599
.Ltmp11964:
	.loc	16 961 18
	shlq	$2, %r14
.Ltmp11965:
	.loc	20 25 13
	movq	%rbx, %rdi
	xorl	%esi, %esi
	movq	%r14, %rdx
	movq	%r9, %r14
	callq	*memset@GOTPCREL(%rip)
	jmp	.LBB36_658
.Ltmp11966:
.LBB36_599:
	.loc	20 0 13 is_stmt 0
	movq	%r9, %r14
.LBB36_658:
	movq	%rbp, %rbx
.LBB36_659:
.Ltmp11967:
	.loc	16 961 18 is_stmt 1
	shlq	$2, %r14
.Ltmp11968:
	.loc	20 25 13
	movq	%rbx, %rdi
	xorl	%esi, %esi
	movq	%r14, %rdx
	callq	*memset@GOTPCREL(%rip)
.Ltmp11969:
	.loc	1 2274 18
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %rbx
	leaq	1616(%rsp), %r14
	movq	1192(%rsp), %rdi
	movq	%r14, %rsi
	movq	%r12, %rdx
	movq	%r15, %rcx
	movl	144(%rsp), %ebp
	movl	%ebp, %r8d
	callq	*%rbx
	movq	1184(%rsp), %rdi
	.loc	1 2275 19
	movq	%r14, %rsi
	movq	(%rsp), %rdx
	movq	56(%rsp), %rcx
	movl	%ebp, %r8d
	callq	*%rbx
	.loc	1 2276 13
	movq	$0, 560(%r13)
.Ltmp11970:
.LBB36_636:
	.loc	1 2278 6 epilogue_begin
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
.LBB36_664:
	.cfi_def_cfa_offset 3728
.Ltmp11971:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_283c8d9f84e10f29a75d8e6c3a347bea(%rip), %rcx
	movq	%rax, %rdi
	movq	%r14, %rsi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_390:
.Ltmp11972:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp11973:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp11974:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rsi, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp11975:
.LBB36_646:
	.loc	5 438 16
	cmpq	%r9, 232(%rsp)
	ja	.LBB36_660
.Ltmp11976:
	.loc	5 0 16 is_stmt 0
	movq	%rbp, %rax
	movq	232(%rsp), %rdi
	.p2align	4
.LBB36_648:
.Ltmp11977:
	.loc	17 131 18 is_stmt 1
	movq	%rdi, %rcx
.Ltmp11978:
	.loc	15 1504 12
	testq	%rdi, %rdi
	je	.LBB36_652
.Ltmp11979:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp11980:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%esi, %esi
	xorl	%r8d, %r8d
.Ltmp11981:
	.loc	16 0 18 is_stmt 0
.Ltmp11982:
	.p2align	4
.LBB36_650:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r8), %esi
.Ltmp11983:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp11984:
	.loc	6 180 28
	jne	.LBB36_650
.Ltmp11985:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp11986:
	.loc	19 2054 74
	movq	%rcx, %rdi
	subq	%rdx, %rdi
.Ltmp11987:
	.loc	17 136 12
	testl	%esi, %esi
	je	.LBB36_648
.Ltmp11988:
.LBB36_652:
	.loc	15 1504 12
	testq	%rcx, %rcx
	sete	%al
	movq	1200(%rsp), %rcx
.Ltmp11989:
	.loc	1 2260 35
	jmp	.LBB36_586
.LBB36_510:
	.loc	1 0 35 is_stmt 0
	movq	680(%rsp), %rbx
.Ltmp11990:
	.loc	52 56 9 is_stmt 1
	cmpq	$0, 224(%r13)
	je	.LBB36_418
.Ltmp11991:
.LBB36_576:
	.loc	1 0 0 is_stmt 0
	vmovss	568(%rsp), %xmm1
	movl	572(%rsp), %ecx
	vmovss	904(%rsp), %xmm0
	movl	908(%rsp), %eax
.Ltmp11992:
	.loc	1 2077 0 is_stmt 1
	movq	216(%r13), %rdx
.Ltmp11993:
	.loc	52 56 9
	vmovss	%xmm1, (%rdx)
.Ltmp11994:
	.loc	1 2078 5
	movq	256(%r13), %rdx
.Ltmp11995:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp11996:
	.loc	6 180 28
	je	.LBB36_579
.Ltmp11997:
	.loc	6 0 28 is_stmt 0
	movq	248(%r13), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB36_578:
.Ltmp11998:
	.loc	20 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp11999:
	.loc	7 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp12000:
	.loc	6 180 28
	jne	.LBB36_578
.Ltmp12001:
.LBB36_579:
	.loc	52 56 9
	cmpq	$0, 424(%r13)
	je	.LBB36_418
.Ltmp12002:
	.loc	1 2079 0
	movq	416(%r13), %rcx
.Ltmp12003:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx)
.Ltmp12004:
	.loc	1 2080 5
	movq	456(%r13), %rcx
.Ltmp12005:
	.loc	7 1714 9
	testq	%rcx, %rcx
.Ltmp12006:
	.loc	6 180 28
	je	.LBB36_583
.Ltmp12007:
	.loc	6 0 28 is_stmt 0
	movq	448(%r13), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB36_582:
.Ltmp12008:
	.loc	20 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp12009:
	.loc	7 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp12010:
	.loc	6 180 28
	jne	.LBB36_582
.Ltmp12011:
.LBB36_583:
	.loc	6 0 28 is_stmt 0
	leaq	1000(%rsp), %rdi
	movq	1192(%rsp), %rsi
	.loc	1 2082 14 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	leaq	1092(%rsp), %rdi
	movq	1184(%rsp), %rsi
	.loc	1 2083 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	1512(%rsp), %rax
	.loc	1 2084 5
	movl	%eax, 560(%r13)
	movq	816(%rsp), %rax
	jmp	.LBB36_584
.Ltmp12012:
.LBB36_416:
	.loc	1 0 5 is_stmt 0
	movq	680(%rsp), %rbx
.LBB36_417:
.Ltmp12013:
	.loc	52 56 9 is_stmt 1
	cmpq	$0, 224(%r13)
	je	.LBB36_418
.Ltmp12014:
	.loc	1 0 0 is_stmt 0
	vmovss	1480(%rsp), %xmm1
	movl	1484(%rsp), %ecx
	vmovss	904(%rsp), %xmm0
	movl	908(%rsp), %eax
.Ltmp12015:
	.loc	1 2077 0 is_stmt 1
	movq	216(%r13), %rdx
.Ltmp12016:
	.loc	52 56 9
	vmovss	%xmm1, (%rdx)
.Ltmp12017:
	.loc	1 2078 5
	movq	256(%r13), %rdx
.Ltmp12018:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp12019:
	.loc	6 180 28
	je	.LBB36_504
.Ltmp12020:
	.loc	6 0 28 is_stmt 0
	movq	248(%r13), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB36_503:
.Ltmp12021:
	.loc	20 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp12022:
	.loc	7 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp12023:
	.loc	6 180 28
	jne	.LBB36_503
.Ltmp12024:
.LBB36_504:
	.loc	52 56 9
	cmpq	$0, 424(%r13)
	je	.LBB36_418
.Ltmp12025:
	.loc	1 2079 0
	movq	416(%r13), %rcx
.Ltmp12026:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx)
.Ltmp12027:
	.loc	1 2080 5
	movq	456(%r13), %rcx
.Ltmp12028:
	.loc	7 1714 9
	testq	%rcx, %rcx
.Ltmp12029:
	.loc	6 180 28
	je	.LBB36_508
.Ltmp12030:
	.loc	6 0 28 is_stmt 0
	movq	448(%r13), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB36_507:
.Ltmp12031:
	.loc	20 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp12032:
	.loc	7 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp12033:
	.loc	6 180 28
	jne	.LBB36_507
.Ltmp12034:
.LBB36_508:
	.loc	1 2082 5
	vmovups	708(%rsp), %ymm0
	vmovups	740(%rsp), %ymm1
	vmovups	768(%rsp), %ymm2
	vmovups	%ymm2, 892(%rsp)
	vmovups	%ymm1, 864(%rsp)
	vmovups	%ymm0, 832(%rsp)
	leaq	832(%rsp), %rdi
	movq	1192(%rsp), %rsi
	.loc	1 2082 14 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	.loc	1 2083 5 is_stmt 1
	vmovups	496(%rsp), %ymm0
	vmovups	528(%rsp), %ymm1
	vmovups	556(%rsp), %ymm2
	vmovups	%ymm2, 892(%rsp)
	vmovups	%ymm1, 864(%rsp)
	vmovups	%ymm0, 832(%rsp)
	leaq	832(%rsp), %rdi
	movq	1184(%rsp), %rsi
	.loc	1 2083 15 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	648(%rsp), %rax
	.loc	1 2084 5 is_stmt 1
	movl	%eax, 560(%r13)
	movq	432(%rsp), %rax
	.loc	1 2085 5
	jmp	.LBB36_584
.Ltmp12035:
.LBB36_279:
	.loc	5 569 13
	leaq	.Lalloc_283c8d9f84e10f29a75d8e6c3a347bea(%rip), %rcx
	movq	%rax, %rdi
.Ltmp12036:
	.loc	1 0 0 is_stmt 0
	movq	%r10, %rsi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12037:
.LBB36_662:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_283c8d9f84e10f29a75d8e6c3a347bea(%rip), %rcx
	movq	%rax, %rdi
	movq	%r9, %rsi
	movq	%r9, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12038:
.LBB36_377:
	.loc	1 1417 5
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12039:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12040:
	.loc	1 0 0
	leaq	.Lalloc_0a0a1af21ea56de7dc8d858c82432bb4(%rip), %rcx
	movq	%r14, %rsi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_221:
.Ltmp12041:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12042:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12043:
	.loc	1 0 0
	leaq	.Lalloc_0a0a1af21ea56de7dc8d858c82432bb4(%rip), %rcx
	movq	%r14, %rsi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_154:
.Ltmp12044:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12045:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12046:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
	movq	%r9, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12047:
.LBB36_346:
	.loc	1 1417 5
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12048:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12049:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
	movq	%r11, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12050:
.LBB36_190:
	.loc	1 1411 5
	vmovss	%xmm8, 792(%rsp)
.Ltmp12051:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12052:
	.loc	1 0 0
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_666:
.Ltmp12053:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12054:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12055:
	.loc	1 0 0
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_192:
.Ltmp12056:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12057:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12058:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%r14, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12059:
.LBB36_413:
	.loc	1 1417 5
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12060:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12061:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_af093af980f3691b22b0bed94901ed34(%rip), %rcx
.Ltmp12062:
	.loc	1 0 0 is_stmt 0
	movq	%r10, %rsi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12063:
.LBB36_254:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12064:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12065:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_af093af980f3691b22b0bed94901ed34(%rip), %rcx
	movq	296(%rsp), %rdx
.Ltmp12066:
	.loc	1 0 0 is_stmt 0
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_388:
.Ltmp12067:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12068:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12069:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
.Ltmp12070:
	.loc	5 581 13 is_stmt 0
	movq	%r15, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12071:
.LBB36_229:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12072:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12073:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
.Ltmp12074:
	.loc	5 581 13 is_stmt 0
	movq	%rdx, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12075:
.LBB36_667:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12076:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12077:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
.Ltmp12078:
	.loc	5 581 13 is_stmt 0
	movq	%r15, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12079:
.LBB36_663:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12080:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12081:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
.Ltmp12082:
	.loc	5 581 13 is_stmt 0
	movq	%rdx, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12083:
.LBB36_231:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12084:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
	movq	%rcx, %rdi
.Ltmp12085:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12086:
.LBB36_542:
	.loc	5 0 13 is_stmt 0
	vmovss	112(%rsp), %xmm0
.Ltmp12087:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12088:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12089:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12090:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12091:
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_559:
	vmovss	112(%rsp), %xmm0
.Ltmp12092:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12093:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %ecx
	movl	%ecx, 572(%rsp)
	movl	96(%rsp), %ecx
	movl	%ecx, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12094:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12095:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12096:
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_461:
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12097:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12098:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12099:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
.Ltmp12100:
	.loc	1 1660 5
	vmovss	%xmm8, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12101:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12102:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12103:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_489:
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12104:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12105:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12106:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
	vmovaps	368(%rsp), %xmm0
.Ltmp12107:
	.loc	1 1660 5
	vmovss	%xmm0, 788(%rsp)
.Ltmp12108:
.LBB36_482:
	.loc	1 0 5 is_stmt 0
	vmovss	32(%rsp), %xmm0
.Ltmp12109:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12110:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12111:
	.loc	1 1131 25
	leaq	1(%rdi), %rsi
.Ltmp12112:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12113:
.LBB36_150:
	.loc	1 1411 5
	vmovss	%xmm8, 792(%rsp)
.Ltmp12114:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12115:
	.loc	1 0 0
	leaq	.Lalloc_91a4c6b9b17ebf4d863f9a70b6dc929a(%rip), %rdx
	movq	%r12, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_343:
.Ltmp12116:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12117:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12118:
	.loc	1 0 0
	leaq	.Lalloc_91a4c6b9b17ebf4d863f9a70b6dc929a(%rip), %rdx
	movq	%r12, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12119:
.LBB36_600:
	movb	$1, %r9b
	.loc	1 2215 12 is_stmt 1
	cmpb	$0, 780(%r13)
	je	.LBB36_32
.Ltmp12120:
	.loc	1 663 31
	movq	256(%r13), %rsi
	.loc	1 663 57 is_stmt 0
	movq	320(%r13), %rax
.Ltmp12121:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rsi, %rax
	cmovbq	%rax, %rsi
.Ltmp12122:
	.loc	14 304 12
	testq	%rsi, %rsi
	movq	232(%rsp), %rbx
	je	.LBB36_611
.Ltmp12123:
	.loc	14 0 12 is_stmt 0
	movq	248(%r13), %rdi
	movq	312(%r13), %r8
	xorl	%r9d, %r9d
	movq	%rbx, %r10
	shrq	$32, %r10
	jmp	.LBB36_603
.LBB36_609:
.Ltmp12124:
	.loc	1 665 22 is_stmt 1
	movq	%rcx, %rax
	xorl	%edx, %edx
	divq	%r11
.LBB36_610:
	.loc	1 665 13 is_stmt 0
	movl	%edx, (%rdi,%r9,4)
.Ltmp12125:
	.loc	1 0 0
	incq	%r9
.Ltmp12126:
	.loc	14 304 12 is_stmt 1
	addq	$12, %r8
	cmpq	%r9, %rsi
	je	.LBB36_611
.Ltmp12127:
.LBB36_603:
	.loc	1 664 26
	movl	(%r8), %r11d
	testq	%r11, %r11
.Ltmp12128:
	.loc	1 665 42
	je	.LBB36_684
	.loc	1 665 24 is_stmt 0
	movl	(%rdi,%r9,4), %ecx
	.loc	1 665 42
	testq	%r10, %r10
	je	.LBB36_605
	movq	%rbx, %rax
	xorl	%edx, %edx
	divq	%r11
	.loc	1 665 23
	addq	%rdx, %rcx
	.loc	1 665 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB36_609
	jmp	.LBB36_608
.LBB36_605:
	.loc	1 665 42
	movl	%ebx, %eax
	xorl	%edx, %edx
	divl	%r11d
	.loc	1 665 23
	addq	%rdx, %rcx
	.loc	1 665 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB36_609
.LBB36_608:
	movl	%ecx, %eax
	xorl	%edx, %edx
	divl	%r11d
	jmp	.LBB36_610
.Ltmp12129:
.LBB36_611:
	.loc	1 663 31 is_stmt 1
	movq	456(%r13), %rsi
	.loc	1 663 57 is_stmt 0
	movq	520(%r13), %rax
.Ltmp12130:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rsi, %rax
	cmovbq	%rax, %rsi
.Ltmp12131:
	.loc	14 304 12
	testq	%rsi, %rsi
	je	.LBB36_621
.Ltmp12132:
	.loc	14 0 12 is_stmt 0
	movq	448(%r13), %rdi
	movq	512(%r13), %r8
	xorl	%r9d, %r9d
	movq	%rbx, %r10
	shrq	$32, %r10
	jmp	.LBB36_613
.LBB36_619:
.Ltmp12133:
	.loc	1 665 22 is_stmt 1
	movq	%rcx, %rax
	xorl	%edx, %edx
	divq	%r11
.LBB36_620:
	.loc	1 665 13 is_stmt 0
	movl	%edx, (%rdi,%r9,4)
.Ltmp12134:
	.loc	1 0 0
	incq	%r9
.Ltmp12135:
	.loc	14 304 12 is_stmt 1
	addq	$12, %r8
	cmpq	%r9, %rsi
	je	.LBB36_621
.Ltmp12136:
.LBB36_613:
	.loc	1 664 26
	movl	(%r8), %r11d
	testq	%r11, %r11
.Ltmp12137:
	.loc	1 665 42
	je	.LBB36_684
	.loc	1 665 24 is_stmt 0
	movl	(%rdi,%r9,4), %ecx
	.loc	1 665 42
	testq	%r10, %r10
	je	.LBB36_615
	movq	%rbx, %rax
	xorl	%edx, %edx
	divq	%r11
	.loc	1 665 23
	addq	%rdx, %rcx
	.loc	1 665 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB36_619
	jmp	.LBB36_618
.LBB36_615:
	.loc	1 665 42
	movl	%ebx, %eax
	xorl	%edx, %edx
	divl	%r11d
	.loc	1 665 23
	addq	%rdx, %rcx
	.loc	1 665 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB36_619
.LBB36_618:
	movl	%ecx, %eax
	xorl	%edx, %edx
	divl	%r11d
	jmp	.LBB36_620
.Ltmp12138:
.LBB36_621:
	.loc	1 2232 26 is_stmt 1
	movq	552(%r13), %rsi
.Ltmp12139:
	.loc	1 455 44
	testq	%rsi, %rsi
	je	.LBB36_685
.Ltmp12140:
	.loc	1 2232 26
	movq	544(%r13), %rcx
.Ltmp12141:
	.loc	1 455 23
	movl	560(%r13), %edi
	.loc	1 455 44 is_stmt 0
	movq	%rbx, %rax
	orq	%rsi, %rax
	shrq	$32, %rax
	je	.LBB36_623
	movq	%rbx, %rax
	xorl	%edx, %edx
	divq	%rsi
	movq	%rdx, %rax
	jmp	.LBB36_625
.LBB36_623:
	movl	%ebx, %eax
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB36_625:
	.loc	1 455 22
	addq	%rdi, %rax
	.loc	1 455 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB36_626
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB36_628
.LBB36_626:
	xorl	%edx, %edx
	divl	%esi
.LBB36_628:
	.loc	1 455 9
	movl	%edx, 560(%r13)
	.loc	1 456 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB36_686
	.loc	1 456 23 is_stmt 0
	movl	564(%r13), %esi
	.loc	1 456 44
	movq	%rbx, %rax
	orq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB36_630
	movq	%rbx, %rax
	xorl	%edx, %edx
	divq	%rcx
	movq	%rdx, %rax
	jmp	.LBB36_632
.LBB36_630:
	movl	%ebx, %eax
	xorl	%edx, %edx
	divl	%ecx
	movl	%edx, %eax
.LBB36_632:
	.loc	1 456 22
	addq	%rsi, %rax
	.loc	1 456 21
	movq	%rax, %rdx
	orq	%rcx, %rdx
	shrq	$32, %rdx
	je	.LBB36_633
	xorl	%edx, %edx
	divq	%rcx
	jmp	.LBB36_635
.LBB36_633:
	xorl	%edx, %edx
	divl	%ecx
.LBB36_635:
	.loc	1 456 9
	movl	%edx, 564(%r13)
	jmp	.LBB36_636
.Ltmp12142:
.LBB36_574:
	.loc	1 0 9
	vmovss	112(%rsp), %xmm0
.Ltmp12143:
	vmovss	%xmm0, 1000(%rsp)
.Ltmp12144:
	vmovss	%xmm7, 1092(%rsp)
	movl	%r10d, 572(%rsp)
	vmovss	692(%rsp), %xmm0
	vmovss	%xmm0, 568(%rsp)
	movl	%r9d, 908(%rsp)
	vmovss	688(%rsp), %xmm0
	vmovss	%xmm0, 904(%rsp)
.Ltmp12145:
	.loc	52 56 9 is_stmt 1
	cmpq	$0, 224(%r13)
	jne	.LBB36_576
.Ltmp12146:
.LBB36_418:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_678:
	vmovss	112(%rsp), %xmm0
.Ltmp12147:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12148:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12149:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12150:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12151:
	.loc	14 304 12 is_stmt 1
	cmpq	%rdx, %r14
	cmovbeq	%rdx, %r14
	incq	%r14
.Ltmp12152:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r14, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12153:
.LBB36_669:
	.loc	5 0 13 is_stmt 0
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12154:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12155:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12156:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
.Ltmp12157:
	.loc	1 1660 5
	vmovss	%xmm8, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12158:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12159:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12160:
	.loc	14 304 12
	cmpq	%rdx, %rsi
	cmovbeq	%rdx, %rsi
	incq	%rsi
.Ltmp12161:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12162:
.LBB36_471:
	.loc	5 0 13 is_stmt 0
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12163:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12164:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12165:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
.Ltmp12166:
	.loc	1 1660 5
	vmovss	%xmm8, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12167:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12168:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12169:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_679:
	vmovss	112(%rsp), %xmm0
.Ltmp12170:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12171:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12172:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12173:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12174:
	.loc	1 1131 25 is_stmt 1
	leaq	1(%rdi), %rsi
.Ltmp12175:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12176:
.LBB36_474:
	.loc	5 0 13 is_stmt 0
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12177:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12178:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12179:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm6, 792(%rsp)
.Ltmp12180:
	.loc	1 1660 5
	vmovss	%xmm8, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12181:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12182:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
	jmp	.LBB36_475
.Ltmp12183:
.LBB36_680:
	.loc	1 0 5 is_stmt 0
	vmovss	112(%rsp), %xmm0
.Ltmp12184:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12185:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12186:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12187:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12188:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r8, %rsi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12189:
.LBB36_681:
	.loc	5 0 13 is_stmt 0
	vmovss	112(%rsp), %xmm0
.Ltmp12190:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12191:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12192:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12193:
	vmovss	%xmm0, 904(%rsp)
	movq	280(%rsp), %rdx
.Ltmp12194:
	.loc	14 304 12 is_stmt 1
	cmpq	%rdx, %r15
	cmovbeq	%rdx, %r15
	incq	%r15
.Ltmp12195:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rbp, %rdi
	movq	%r15, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12196:
.LBB36_671:
	.loc	5 0 13 is_stmt 0
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12197:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12198:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12199:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
.Ltmp12200:
	.loc	1 1660 5
	vmovss	%xmm12, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12201:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12202:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
	movq	648(%rsp), %rsi
	movq	272(%rsp), %rdx
.Ltmp12203:
	.loc	14 304 12
	cmpq	%rdx, %rsi
	cmovbeq	%rdx, %rsi
	incq	%rsi
.Ltmp12204:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rax, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12205:
.LBB36_672:
	.loc	5 0 13 is_stmt 0
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12206:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12207:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12208:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
.Ltmp12209:
	.loc	1 1660 5
	vmovss	%xmm12, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12210:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12211:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12212:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r8, %rsi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12213:
.LBB36_555:
	.loc	5 0 13 is_stmt 0
	vmovss	112(%rsp), %xmm0
.Ltmp12214:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12215:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %ecx
	movl	%ecx, 572(%rsp)
	movl	96(%rsp), %ecx
	movl	%ecx, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12216:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12217:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12218:
.LBB36_475:
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r8, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_481:
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12219:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12220:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12221:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
.Ltmp12222:
	.loc	1 1660 5
	vmovss	%xmm12, 788(%rsp)
	jmp	.LBB36_482
.Ltmp12223:
.LBB36_569:
	.loc	1 0 5 is_stmt 0
	vmovss	112(%rsp), %xmm0
.Ltmp12224:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12225:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12226:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12227:
	vmovss	%xmm0, 904(%rsp)
	movq	%rsi, %rdx
.Ltmp12228:
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_493:
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12229:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12230:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12231:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
	vmovaps	368(%rsp), %xmm0
.Ltmp12232:
	.loc	1 1660 5
	vmovss	%xmm0, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12233:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12234:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
	movq	%rsi, %rdx
.Ltmp12235:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_674:
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12236:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12237:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12238:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
	vmovaps	368(%rsp), %xmm0
.Ltmp12239:
	.loc	1 1660 5
	vmovss	%xmm0, 788(%rsp)
.Ltmp12240:
	.loc	1 1654 5
	vmovss	%xmm6, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12241:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12242:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r8, %rsi
	movq	312(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12243:
.LBB36_682:
	.loc	5 0 13 is_stmt 0
	vmovss	112(%rsp), %xmm0
.Ltmp12244:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12245:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12246:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12247:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12248:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdi
	movq	%r8, %rsi
	movq	408(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12249:
.LBB36_683:
	.loc	5 0 13 is_stmt 0
	vmovss	112(%rsp), %xmm0
.Ltmp12250:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12251:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12252:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12253:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12254:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rbp, %rdi
	movq	312(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12255:
.LBB36_675:
	.loc	5 0 13 is_stmt 0
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12256:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12257:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12258:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
	vmovaps	368(%rsp), %xmm0
.Ltmp12259:
	.loc	1 1660 5
	vmovss	%xmm0, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12260:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
.Ltmp12261:
	.loc	1 1660 5
	vmovss	%xmm1, 576(%rsp)
.Ltmp12262:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rax, %rdi
	movq	400(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12263:
.LBB36_520:
	.loc	5 0 13 is_stmt 0
	vmovss	112(%rsp), %xmm0
.Ltmp12264:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12265:
	vmovss	%xmm0, 1092(%rsp)
	movl	%r10d, 572(%rsp)
	vmovss	692(%rsp), %xmm0
	vmovss	%xmm0, 568(%rsp)
	movl	%r9d, 908(%rsp)
	vmovss	688(%rsp), %xmm0
	vmovss	%xmm0, 904(%rsp)
.Ltmp12266:
.LBB36_521:
	leaq	.Lalloc_283c8d9f84e10f29a75d8e6c3a347bea(%rip), %rcx
	movq	%r14, %rsi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_439:
	vmovss	112(%rsp), %xmm0
.Ltmp12267:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12268:
	vmovss	%xmm0, 1092(%rsp)
	movl	%r10d, 572(%rsp)
	vmovss	692(%rsp), %xmm0
	vmovss	%xmm0, 568(%rsp)
	movl	%r9d, 908(%rsp)
	vmovss	688(%rsp), %xmm0
	vmovss	%xmm0, 904(%rsp)
.Ltmp12269:
.LBB36_440:
	leaq	.Lalloc_283c8d9f84e10f29a75d8e6c3a347bea(%rip), %rcx
	movq	296(%rsp), %rdx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB36_378:
.Ltmp12270:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12271:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12272:
	.loc	1 0 0
	leaq	.Lalloc_4a8785a681d008a9bfd0cd82628ea9cb(%rip), %rdx
	movq	56(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_127:
.Ltmp12273:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12274:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12275:
	.loc	1 0 0
	leaq	.Lalloc_4a8785a681d008a9bfd0cd82628ea9cb(%rip), %rdx
	movq	56(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_136:
.Ltmp12276:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12277:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12278:
	.loc	1 0 0
	leaq	.Lalloc_2b690e2c7763f11809942906fc2ca813(%rip), %rdx
	movq	%r9, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_383:
.Ltmp12279:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12280:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12281:
	.loc	1 0 0
	leaq	.Lalloc_2b690e2c7763f11809942906fc2ca813(%rip), %rdx
	movq	%r9, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_379:
.Ltmp12282:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12283:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12284:
	.loc	1 0 0
	leaq	.Lalloc_cbce7773ac40979e4ba2385da3aec116(%rip), %rdx
	movq	%r12, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_129:
.Ltmp12285:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12286:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12287:
	.loc	1 0 0
	leaq	.Lalloc_cbce7773ac40979e4ba2385da3aec116(%rip), %rdx
	movq	%r12, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_380:
.Ltmp12288:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12289:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12290:
	.loc	1 0 0
	leaq	.Lalloc_ec0d48f73ebfc2755df5cedaa60b5c0a(%rip), %rdx
	movq	144(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_131:
.Ltmp12291:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12292:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12293:
	.loc	1 0 0
	leaq	.Lalloc_ec0d48f73ebfc2755df5cedaa60b5c0a(%rip), %rdx
	movq	144(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_670:
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp12294:
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12295:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12296:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
.Ltmp12297:
	.loc	1 1660 5
	vmovss	%xmm8, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12298:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12299:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12300:
	.loc	1 1131 25
	leaq	1(%r11), %rsi
.Ltmp12301:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12302:
.LBB36_673:
	.loc	5 0 13 is_stmt 0
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 1480(%rsp)
	vmovss	28(%rsp), %xmm0
.Ltmp12303:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12304:
	.loc	1 1654 5 is_stmt 1
	vmovss	%xmm13, 792(%rsp)
	vmovaps	368(%rsp), %xmm0
.Ltmp12305:
	.loc	1 1660 5
	vmovss	%xmm0, 788(%rsp)
	vmovss	32(%rsp), %xmm0
.Ltmp12306:
	.loc	1 1654 5
	vmovss	%xmm0, 580(%rsp)
	vmovaps	208(%rsp), %xmm0
.Ltmp12307:
	.loc	1 1660 5
	vmovss	%xmm0, 576(%rsp)
.Ltmp12308:
	.loc	1 1131 25
	leaq	1(%r11), %rsi
.Ltmp12309:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12310:
.LBB36_381:
	.loc	1 1417 5
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12311:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12312:
	.loc	1 0 0
	leaq	.Lalloc_2c461872bb652d4796cdcf89c28c82c8(%rip), %rdx
	movq	%r11, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_133:
.Ltmp12313:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12314:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12315:
	.loc	1 0 0
	leaq	.Lalloc_2c461872bb652d4796cdcf89c28c82c8(%rip), %rdx
	movq	%r11, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_125:
.Ltmp12316:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12317:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12318:
	.loc	1 0 0
	leaq	.Lalloc_9fa421ae81817f58fcfc4a3243223891(%rip), %rdx
	movq	%r12, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_298:
.Ltmp12319:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12320:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12321:
	.loc	1 0 0
	leaq	.Lalloc_9fa421ae81817f58fcfc4a3243223891(%rip), %rdx
	movq	%r12, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_107:
.Ltmp12322:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12323:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12324:
	.loc	1 0 0
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_112:
.Ltmp12325:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12326:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12327:
	.loc	1 0 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_285:
.Ltmp12328:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12329:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12330:
	.loc	1 0 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_278:
.Ltmp12331:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12332:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12333:
.LBB36_81:
	.loc	1 0 0
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_438:
	vmovss	%xmm12, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
.Ltmp12334:
	vmovss	%xmm10, 1480(%rsp)
.Ltmp12335:
	vmovss	%xmm9, 904(%rsp)
.Ltmp12336:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_efa26be4ae64153d8eb817c4a24fadb5(%rip), %rcx
	movq	%rax, %rsi
	movq	960(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12337:
.LBB36_676:
	.loc	5 0 13 is_stmt 0
	vmovaps	%xmm0, %xmm1
	vmovss	112(%rsp), %xmm0
.Ltmp12338:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12339:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	%xmm1, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12340:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12341:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12342:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_efa26be4ae64153d8eb817c4a24fadb5(%rip), %rcx
	movq	%r14, %rsi
	movq	960(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12343:
.LBB36_412:
	.loc	1 1417 5
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12344:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12345:
	.loc	1 0 0
	leaq	.Lalloc_b305c1483509cfb31fdec21ff8752674(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_219:
.Ltmp12346:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12347:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12348:
	.loc	1 0 0
	leaq	.Lalloc_b305c1483509cfb31fdec21ff8752674(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12349:
.LBB36_687:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_bf99c8ff01c64f6a0b34983a37a16939(%rip), %rcx
.Ltmp12350:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12351:
.LBB36_677:
	.loc	5 0 13
	vmovaps	%xmm0, %xmm1
	vmovss	112(%rsp), %xmm0
.Ltmp12352:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12353:
	vmovss	%xmm0, 1092(%rsp)
	movl	88(%rsp), %eax
	movl	%eax, 572(%rsp)
	movl	96(%rsp), %eax
	movl	%eax, 908(%rsp)
	vmovss	%xmm12, 1084(%rsp)
	vmovss	%xmm1, 1080(%rsp)
	vmovss	%xmm10, 1176(%rsp)
	vmovss	%xmm6, 1172(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp12354:
	vmovss	%xmm0, 568(%rsp)
	vmovss	16(%rsp), %xmm0
.Ltmp12355:
	vmovss	%xmm0, 904(%rsp)
.Ltmp12356:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_fcc53ff99bd917af3a52e0ac8a8cbd22(%rip), %rcx
	movq	%r14, %rsi
	movq	296(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12357:
.LBB36_668:
	.loc	5 0 13 is_stmt 0
	vmovss	%xmm12, 768(%rsp)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 556(%rsp)
.Ltmp12358:
	vmovss	%xmm10, 1480(%rsp)
.Ltmp12359:
	vmovss	%xmm9, 904(%rsp)
.Ltmp12360:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_fcc53ff99bd917af3a52e0ac8a8cbd22(%rip), %rcx
	movq	%rax, %rsi
	movq	296(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12361:
.LBB36_23:
	.loc	5 443 13
	leaq	.Lalloc_f0379187153dd7a3d2397dd31aa160d9(%rip), %rcx
.Ltmp12362:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12363:
.LBB36_348:
	.loc	5 0 13
	movl	$1, %eax
	jmp	.LBB36_349
.LBB36_145:
	movl	$1, %eax
	jmp	.LBB36_146
.LBB36_660:
.Ltmp12364:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_15903c4e5f99181283027046d243e508(%rip), %rcx
	xorl	%edi, %edi
	movq	232(%rsp), %rsi
	movq	%r9, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12365:
.LBB36_354:
	.loc	5 0 13 is_stmt 0
	movl	$2, %eax
	jmp	.LBB36_349
.LBB36_196:
	movl	$2, %eax
	jmp	.LBB36_146
.LBB36_661:
.Ltmp12366:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_637d5bfd284109ae56bb817a8357666c(%rip), %rcx
	xorl	%edi, %edi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp12367:
.LBB36_200:
	.loc	5 0 13 is_stmt 0
	movl	$3, %eax
	jmp	.LBB36_146
.LBB36_358:
	movl	$3, %eax
	jmp	.LBB36_349
.LBB36_362:
	movl	$4, %eax
	jmp	.LBB36_349
.LBB36_204:
	movl	$4, %eax
	jmp	.LBB36_146
.LBB36_519:
	vmovss	112(%rsp), %xmm0
.Ltmp12368:
	vmovss	%xmm0, 1000(%rsp)
	vmovss	104(%rsp), %xmm0
.Ltmp12369:
	vmovss	%xmm0, 1092(%rsp)
	movl	%r10d, 572(%rsp)
	vmovss	692(%rsp), %xmm0
	vmovss	%xmm0, 568(%rsp)
	movl	%r9d, 908(%rsp)
	vmovss	688(%rsp), %xmm0
	vmovss	%xmm0, 904(%rsp)
.Ltmp12370:
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB36_208:
	movl	$5, %eax
	jmp	.LBB36_146
.LBB36_366:
	movl	$5, %eax
	jmp	.LBB36_349
.LBB36_212:
	movl	$6, %eax
	jmp	.LBB36_146
.LBB36_370:
	movl	$6, %eax
	jmp	.LBB36_349
.LBB36_374:
	movl	$7, %eax
.LBB36_349:
	movq	%rax, 40(%rsp)
.LBB36_350:
.Ltmp12371:
	.loc	1 1417 5 is_stmt 1
	vmovss	%xmm7, 1304(%rsp)
.Ltmp12372:
	.loc	1 1417 5 is_stmt 0
	vmovss	%xmm6, 1396(%rsp)
.Ltmp12373:
	.loc	1 1403 42 is_stmt 1
	leaq	.Lalloc_a6feb40b34112df5f214f84424dd2c7c(%rip), %rdx
	movq	40(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12374:
.LBB36_216:
	.loc	1 0 42 is_stmt 0
	movl	$7, %eax
.LBB36_146:
	movq	%rax, 184(%rsp)
.LBB36_147:
.Ltmp12375:
	.loc	1 1411 5 is_stmt 1
	vmovss	%xmm8, 792(%rsp)
.Ltmp12376:
	.loc	1 1411 5 is_stmt 0
	vmovss	%xmm7, 580(%rsp)
.Ltmp12377:
	.loc	1 1403 42 is_stmt 1
	leaq	.Lalloc_a6feb40b34112df5f214f84424dd2c7c(%rip), %rdx
	movq	184(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp12378:
.LBB36_684:
	.loc	1 665 42
	leaq	.Lalloc_bac57976a2bdbfad4a3a85d5d1c7648c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp12379:
.LBB36_685:
	.loc	1 455 44
	leaq	.Lalloc_f0ee36f67d9a332211aa5518dd2ebfd5(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB36_686:
	.loc	1 456 44
	leaq	.Lalloc_33d4d33e0a850133578789055882dcf9(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp12380:
.Lfunc_end36:
	.size	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_, .Lfunc_end36-_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_
