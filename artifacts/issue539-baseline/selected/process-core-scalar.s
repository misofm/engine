_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_:
.Lfunc_begin34:
	.loc	15 2070 0 is_stmt 1
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
	subq	$3208, %rsp
	.cfi_def_cfa_offset 3264
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%r8, %rbp
	movq	%rcx, 608(%rsp)
	movq	%rdx, %r12
	movq	%rsi, 504(%rsp)
	movq	%rdi, %r13
.Ltmp4909:
	.loc	15 2089 21 prologue_end
	movzbl	781(%rdi), %eax
	cmpb	96(%rdi), %al
	jne	.LBB34_29
	.loc	15 2090 37
	movq	264(%r13), %rcx
	movq	272(%r13), %rax
.Ltmp4910:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp4911:
	.p2align	4
.LBB34_2:
.Ltmp4912:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp4913:
	.loc	17 180 28
	je	.LBB34_5
.Ltmp4914:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp4915:
	.loc	17 315 25
	jne	.LBB34_29
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_2
	jmp	.LBB34_29
.Ltmp4916:
.LBB34_5:
	.loc	15 2091 37 is_stmt 1
	movq	280(%r13), %rcx
	movq	288(%r13), %rax
.Ltmp4917:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp4918:
	.p2align	4
.LBB34_6:
.Ltmp4919:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp4920:
	.loc	17 180 28
	je	.LBB34_9
.Ltmp4921:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp4922:
	.loc	17 315 25
	jne	.LBB34_29
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_6
	jmp	.LBB34_29
.Ltmp4923:
.LBB34_9:
	.loc	15 2092 37 is_stmt 1
	movq	464(%r13), %rcx
	movq	472(%r13), %rax
.Ltmp4924:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp4925:
	.p2align	4
.LBB34_10:
.Ltmp4926:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp4927:
	.loc	17 180 28
	je	.LBB34_13
.Ltmp4928:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp4929:
	.loc	17 315 25
	jne	.LBB34_29
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_10
	jmp	.LBB34_29
.Ltmp4930:
.LBB34_13:
	.loc	15 2093 37 is_stmt 1
	movq	480(%r13), %rcx
	movq	488(%r13), %rax
.Ltmp4931:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp4932:
	.p2align	4
.LBB34_14:
.Ltmp4933:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp4934:
	.loc	17 180 28
	je	.LBB34_17
.Ltmp4935:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp4936:
	.loc	17 315 25
	jne	.LBB34_29
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_14
	jmp	.LBB34_29
.Ltmp4937:
.LBB34_17:
	.loc	17 0 25
	cmpq	%r12, %r9
.Ltmp4938:
	.loc	14 1050 16 is_stmt 1
	ja	.LBB34_419
.Ltmp4939:
	.loc	14 0 16 is_stmt 0
	movq	%r9, %rax
	movq	504(%rsp), %rcx
	.p2align	4
.LBB34_19:
.Ltmp4940:
	.loc	29 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB34_23
.Ltmp4941:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp4942:
	.loc	12 961 18
	leal	(,%rdx,4), %edi
	xorl	%esi, %esi
	xorl	%r8d, %r8d
.Ltmp4943:
	.loc	12 0 18 is_stmt 0
.Ltmp4944:
	.p2align	4
.LBB34_21:
	.loc	34 134 13 is_stmt 1
	orl	(%rcx,%r8), %esi
.Ltmp4945:
	.loc	16 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp4946:
	.loc	17 180 28
	jne	.LBB34_21
.Ltmp4947:
	.loc	35 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp4948:
	.loc	30 2054 74
	subq	%rdx, %rax
.Ltmp4949:
	.loc	34 136 12
	testl	%esi, %esi
	je	.LBB34_19
	jmp	.LBB34_29
.Ltmp4950:
.LBB34_23:
	.loc	37 438 16
	cmpq	%rbp, %r9
	ja	.LBB34_420
.Ltmp4951:
	.loc	37 0 16 is_stmt 0
	movq	%r9, %rax
	movq	608(%rsp), %rcx
	.p2align	4
.LBB34_25:
.Ltmp4952:
	.loc	29 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB34_361
.Ltmp4953:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp4954:
	.loc	12 961 18
	leal	(,%rdx,4), %edi
	xorl	%esi, %esi
	xorl	%r8d, %r8d
.Ltmp4955:
	.loc	12 0 18 is_stmt 0
.Ltmp4956:
	.p2align	4
.LBB34_27:
	.loc	34 134 13 is_stmt 1
	orl	(%rcx,%r8), %esi
.Ltmp4957:
	.loc	16 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp4958:
	.loc	17 180 28
	jne	.LBB34_27
.Ltmp4959:
	.loc	35 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp4960:
	.loc	30 2054 74
	subq	%rdx, %rax
.Ltmp4961:
	.loc	34 136 12
	testl	%esi, %esi
	je	.LBB34_25
.Ltmp4962:
.LBB34_29:
	.loc	34 0 12 is_stmt 0
	xorl	%edi, %edi
.LBB34_30:
.Ltmp4963:
	.loc	15 2126 13 is_stmt 1
	leaq	136(%r13), %rax
	movq	%rax, 968(%rsp)
	.loc	15 2127 13
	leaq	336(%r13), %rax
	movq	%rax, 960(%rsp)
.Ltmp4964:
	.loc	15 1011 5
	movq	320(%r13), %rcx
	testq	%rcx, %rcx
	movq	%r13, 920(%rsp)
	movq	%r12, 912(%rsp)
	movq	%rbp, 904(%rsp)
	movq	%r9, 976(%rsp)
	movl	%edi, 1004(%rsp)
	je	.LBB34_36
	.loc	15 0 5 is_stmt 0
	movq	312(%r13), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rdx
	.p2align	4
.LBB34_32:
.Ltmp4965:
	.loc	16 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp4966:
	.loc	17 180 28
	je	.LBB34_36
.Ltmp4967:
	.loc	15 387 34
	movl	(%rdx), %esi
	cmpl	(%rax), %esi
	jne	.LBB34_50
	movl	4(%rdx), %esi
	cmpl	4(%rax), %esi
	jne	.LBB34_50
	movl	8(%rdx), %esi
.Ltmp4968:
	.loc	17 0 0 is_stmt 0
	addq	$12, %rdx
	.loc	17 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp4969:
	.loc	15 387 34
	cmpl	8(%rax), %esi
.Ltmp4970:
	.loc	17 315 25
	je	.LBB34_32
	jmp	.LBB34_50
.Ltmp4971:
.LBB34_36:
	.loc	15 1012 12
	movq	256(%r13), %rax
	testq	%rax, %rax
	je	.LBB34_40
	.loc	15 0 12 is_stmt 0
	movq	248(%r13), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB34_38:
.Ltmp4972:
	.loc	16 1714 9 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp4973:
	.loc	17 180 28
	je	.LBB34_40
.Ltmp4974:
	.loc	17 315 25
	movl	(%rcx,%rdx), %esi
	addq	$4, %rdx
.Ltmp4975:
	.loc	15 1012 43
	cmpl	(%rcx), %esi
.Ltmp4976:
	.loc	17 315 25
	je	.LBB34_38
	jmp	.LBB34_50
.Ltmp4977:
.LBB34_40:
	.loc	15 1011 5
	movq	520(%r13), %rcx
	testq	%rcx, %rcx
	je	.LBB34_46
	.loc	15 0 5 is_stmt 0
	movq	512(%r13), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rdx
	.p2align	4
.LBB34_42:
.Ltmp4978:
	.loc	16 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp4979:
	.loc	17 180 28
	je	.LBB34_46
.Ltmp4980:
	.loc	15 387 34
	movl	(%rdx), %esi
	cmpl	(%rax), %esi
	jne	.LBB34_50
	movl	4(%rdx), %esi
	cmpl	4(%rax), %esi
	jne	.LBB34_50
	movl	8(%rdx), %esi
.Ltmp4981:
	.loc	17 0 0 is_stmt 0
	addq	$12, %rdx
	.loc	17 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp4982:
	.loc	15 387 34
	cmpl	8(%rax), %esi
.Ltmp4983:
	.loc	17 315 25
	je	.LBB34_42
	jmp	.LBB34_50
.Ltmp4984:
.LBB34_46:
	.loc	15 1012 12
	movq	456(%r13), %rax
	testq	%rax, %rax
	je	.LBB34_237
	.loc	15 0 12 is_stmt 0
	movq	448(%r13), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB34_48:
.Ltmp4985:
	.loc	16 1714 9 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp4986:
	.loc	17 180 28
	je	.LBB34_237
.Ltmp4987:
	.loc	17 315 25
	movl	(%rcx,%rdx), %esi
	addq	$4, %rdx
.Ltmp4988:
	.loc	15 1012 43
	cmpl	(%rcx), %esi
.Ltmp4989:
	.loc	17 315 25
	je	.LBB34_48
.Ltmp4990:
.LBB34_50:
	.loc	17 0 25 is_stmt 0
	leaq	812(%rsp), %rdi
.Ltmp4991:
	.loc	15 1647 24 is_stmt 1
	leaq	136(%r13), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	512(%rsp), %rdi
.Ltmp4992:
	.loc	15 1648 25
	leaq	336(%r13), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp4993:
	.loc	15 1660 43
	movq	264(%r13), %rcx
	movq	272(%r13), %rax
.Ltmp4994:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp4995:
	.p2align	4
.LBB34_51:
.Ltmp4996:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp4997:
	.loc	17 180 28
	je	.LBB34_54
.Ltmp4998:
	.loc	17 0 28 is_stmt 0
	movl	$0, 12(%rsp)
.Ltmp4999:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp5000:
	.loc	17 315 25
	jne	.LBB34_67
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_51
	jmp	.LBB34_67
.Ltmp5001:
.LBB34_54:
	.loc	15 1661 33 is_stmt 1
	movq	280(%r13), %rcx
	movq	288(%r13), %rax
.Ltmp5002:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp5003:
	.p2align	4
.LBB34_55:
.Ltmp5004:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5005:
	.loc	17 180 28
	je	.LBB34_58
.Ltmp5006:
	.loc	17 0 28 is_stmt 0
	movl	$0, 12(%rsp)
.Ltmp5007:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp5008:
	.loc	17 315 25
	jne	.LBB34_67
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_55
	jmp	.LBB34_67
.Ltmp5009:
.LBB34_58:
	.loc	15 1662 33 is_stmt 1
	movq	464(%r13), %rcx
	movq	472(%r13), %rax
.Ltmp5010:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp5011:
	.p2align	4
.LBB34_59:
.Ltmp5012:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5013:
	.loc	17 180 28
	je	.LBB34_62
.Ltmp5014:
	.loc	17 0 28 is_stmt 0
	movl	$0, 12(%rsp)
.Ltmp5015:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp5016:
	.loc	17 315 25
	jne	.LBB34_67
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_59
	jmp	.LBB34_67
.Ltmp5017:
.LBB34_237:
	.loc	17 0 25
	leaq	628(%rsp), %rdi
.Ltmp5018:
	.loc	15 1818 24 is_stmt 1
	leaq	136(%r13), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
	leaq	720(%rsp), %rdi
.Ltmp5019:
	.loc	15 1819 25
	leaq	336(%r13), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
.Ltmp5020:
	.loc	15 1822 43
	movq	264(%r13), %rcx
	movq	272(%r13), %rax
.Ltmp5021:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp5022:
	.p2align	4
.LBB34_238:
.Ltmp5023:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5024:
	.loc	17 180 28
	je	.LBB34_241
.Ltmp5025:
	.loc	17 0 28 is_stmt 0
	xorl	%esi, %esi
.Ltmp5026:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp5027:
	.loc	17 315 25
	jne	.LBB34_254
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_238
	jmp	.LBB34_254
.Ltmp5028:
.LBB34_62:
	.loc	15 1663 33 is_stmt 1
	movq	480(%r13), %rcx
	movq	488(%r13), %rax
.Ltmp5029:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp5030:
	.p2align	4
.LBB34_63:
.Ltmp5031:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5032:
	.loc	17 180 28
	je	.LBB34_64
.Ltmp5033:
	.loc	17 0 28 is_stmt 0
	movl	$0, 12(%rsp)
.Ltmp5034:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp5035:
	.loc	17 315 25
	jne	.LBB34_67
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_63
	jmp	.LBB34_67
.Ltmp5036:
.LBB34_241:
	.loc	15 1823 33 is_stmt 1
	movq	280(%r13), %rcx
	movq	288(%r13), %rax
.Ltmp5037:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp5038:
	.p2align	4
.LBB34_242:
.Ltmp5039:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5040:
	.loc	17 180 28
	je	.LBB34_245
.Ltmp5041:
	.loc	17 0 28 is_stmt 0
	xorl	%esi, %esi
.Ltmp5042:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp5043:
	.loc	17 315 25
	jne	.LBB34_254
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_242
	jmp	.LBB34_254
.Ltmp5044:
.LBB34_64:
	.loc	17 0 25
	movb	$1, %al
	movl	%eax, 12(%rsp)
.LBB34_67:
.Ltmp5045:
	.loc	15 1666 19 is_stmt 1
	movzbl	776(%r13), %r14d
.Ltmp5046:
	.loc	15 1667 21
	movzbl	777(%r13), %r15d
.Ltmp5047:
	.loc	15 1668 27
	movl	560(%r13), %eax
	movq	%rax, 40(%rsp)
.Ltmp5048:
	.loc	15 1669 27
	movl	564(%r13), %eax
	movq	%rax, 32(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%ymm0, 928(%rsp)
	leaq	2184(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %rbx
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%rbx
	leaq	1152(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	movq	976(%rsp), %rbx
.Ltmp5049:
	.loc	40 446 20
	testq	%rbx, %rbx
	je	.LBB34_236
.Ltmp5050:
	.loc	15 0 0 is_stmt 0
	movl	%r14d, %eax
	negl	%eax
	movl	%eax, 392(%rsp)
.Ltmp5051:
	movl	%r15d, %eax
	negl	%eax
	movl	%eax, 236(%rsp)
.Ltmp5052:
	.loc	14 3756 21 is_stmt 1
	movq	%rbx, %rdi
	shrq	$5, %rdi
.Ltmp5053:
	.loc	14 3757 21
	movl	%ebx, %eax
	andl	$31, %eax
.Ltmp5054:
	.loc	14 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %rdi
	decl	%r14d
	movl	%r14d, 416(%rsp)
	decl	%r15d
.Ltmp5055:
	.loc	40 446 20
	movq	%r12, %rax
	negq	%rax
	movq	%rax, 992(%rsp)
	movq	%rbp, %rax
	negq	%rax
	movq	%rax, 984(%rsp)
	movq	$0, 120(%rsp)
	movq	608(%rsp), %r8
	movq	504(%rsp), %r9
	xorl	%r11d, %r11d
	movq	%rbx, %r10
	movl	%r15d, 172(%rsp)
	jmp	.LBB34_71
.Ltmp5056:
	.loc	40 0 20 is_stmt 0
.Ltmp5057:
	.p2align	4
.LBB34_69:
	vmovss	80(%rsp), %xmm0
.Ltmp5058:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5059:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
	vmovss	52(%rsp), %xmm0
.Ltmp5060:
	.loc	15 746 0 is_stmt 1
	vmovss	%xmm0, 872(%rsp)
	vmovss	248(%rsp), %xmm0
.Ltmp5061:
	.loc	15 746 0 is_stmt 0
	vmovss	%xmm0, 572(%rsp)
	movq	976(%rsp), %rbx
	movq	904(%rsp), %rbp
	movq	344(%rsp), %rdi
	movq	264(%rsp), %r8
	movq	336(%rsp), %r9
	movq	328(%rsp), %r10
	movq	64(%rsp), %r11
	movq	912(%rsp), %r12
.Ltmp5062:
.LBB34_70:
	.loc	15 0 0
	addq	$32, %r11
	decq	%rdi
.Ltmp5063:
	.loc	40 446 20 is_stmt 1
	addq	$-32, %r10
	subq	$-128, %r9
	subq	$-128, %r8
	testq	%rdi, %rdi
	je	.LBB34_236
.LBB34_71:
.Ltmp5064:
	.loc	14 2584 13
	cmpq	$1, %r10
	movq	%r10, %r14
	adcq	$0, %r14
	cmpq	$32, %r14
	movl	$32, %eax
	cmovaeq	%rax, %r14
.Ltmp5065:
	.loc	15 1612 23
	vmovss	812(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	816(%rsp), %xmm1
	vmovss	820(%rsp), %xmm3
	vmovss	824(%rsp), %xmm12
	vmovss	828(%rsp), %xmm10
	vmovss	832(%rsp), %xmm13
	vmovss	836(%rsp), %xmm7
	vmovss	840(%rsp), %xmm15
	vmovss	844(%rsp), %xmm6
	vmovss	848(%rsp), %xmm14
	vmovss	852(%rsp), %xmm11
.Ltmp5066:
	.loc	10 1916 50
	cmpq	%r11, %rbx
.Ltmp5067:
	.loc	11 900 12
	jne	.LBB34_73
.Ltmp5068:
	.loc	15 0 0 is_stmt 0
	vmovss	856(%rsp), %xmm8
.Ltmp5069:
	.loc	11 900 12
	jmp	.LBB34_80
.Ltmp5070:
	.loc	11 0 12
.Ltmp5071:
	.p2align	4
.LBB34_73:
	vmovss	584(%r13), %xmm0
	vmovss	%xmm0, 168(%rsp)
	vmovss	588(%r13), %xmm0
	vmovss	%xmm0, 616(%rsp)
	vmovss	592(%r13), %xmm0
	vmovss	%xmm0, 64(%rsp)
	vmovss	596(%r13), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	600(%r13), %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	604(%r13), %xmm0
	vmovss	%xmm0, 76(%rsp)
	movq	992(%rsp), %rax
.Ltmp5072:
	.loc	37 568 12 is_stmt 1
	leaq	(%rax,%r11), %rcx
	xorl	%edx, %edx
	vmovss	%xmm11, 176(%rsp)
	vmovss	%xmm14, 136(%rsp)
	vmovss	%xmm6, 24(%rsp)
	vmovss	%xmm15, 80(%rsp)
	vmovss	%xmm7, 96(%rsp)
	vmovaps	%xmm13, %xmm5
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm12, %xmm2
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm1, %xmm4
	vmovss	608(%r13), %xmm1
	vmovss	%xmm1, 288(%rsp)
	vmovss	612(%r13), %xmm1
	vmovss	%xmm1, 384(%rsp)
	vmovss	616(%r13), %xmm1
	vmovss	%xmm1, 376(%rsp)
	vmovss	620(%r13), %xmm1
	vmovss	%xmm1, 280(%rsp)
	vmovss	624(%r13), %xmm1
	vmovss	%xmm1, 272(%rsp)
	vmovss	628(%r13), %xmm1
	vmovss	%xmm1, 368(%rsp)
	vmovss	632(%r13), %xmm1
	vmovss	%xmm1, 360(%rsp)
	vmovss	636(%r13), %xmm1
	vmovss	%xmm1, 132(%rsp)
	vmovss	640(%r13), %xmm1
	vmovss	%xmm1, 352(%rsp)
	vmovss	644(%r13), %xmm1
	vmovss	%xmm1, 344(%rsp)
	vmovss	648(%r13), %xmm1
	vmovss	%xmm1, 264(%rsp)
	vmovss	652(%r13), %xmm1
	vmovss	%xmm1, 336(%rsp)
	vmovss	656(%r13), %xmm1
	vmovss	%xmm1, 328(%rsp)
	vmovss	660(%r13), %xmm1
	vmovss	%xmm1, 496(%rsp)
	vmovss	664(%r13), %xmm1
	vmovss	%xmm1, 488(%rsp)
	vmovss	668(%r13), %xmm1
	vmovss	%xmm1, 480(%rsp)
	vmovss	672(%r13), %xmm1
	vmovss	%xmm1, 408(%rsp)
	vmovss	676(%r13), %xmm1
	vmovss	%xmm1, 472(%rsp)
	vmovss	680(%r13), %xmm1
	vmovss	%xmm1, 164(%rsp)
	vmovss	684(%r13), %xmm1
	vmovss	%xmm1, 256(%rsp)
	vmovss	688(%r13), %xmm1
	vmovss	%xmm1, 160(%rsp)
	vmovss	692(%r13), %xmm1
	vmovss	%xmm1, 156(%rsp)
	vmovss	696(%r13), %xmm1
	vmovss	%xmm1, 152(%rsp)
	vmovss	700(%r13), %xmm1
	vmovss	%xmm1, 148(%rsp)
	vmovss	704(%r13), %xmm1
	vmovss	%xmm1, 320(%rsp)
	vmovss	708(%r13), %xmm1
	vmovss	%xmm1, 464(%rsp)
	vmovss	712(%r13), %xmm1
	vmovss	%xmm1, 400(%rsp)
	vmovss	716(%r13), %xmm1
	vmovss	%xmm1, 456(%rsp)
	vmovss	720(%r13), %xmm1
	vmovss	%xmm1, 448(%rsp)
	vmovss	724(%r13), %xmm1
	vmovss	%xmm1, 440(%rsp)
	vmovss	728(%r13), %xmm1
	vmovss	%xmm1, 20(%rsp)
	vmovss	732(%r13), %xmm1
	vmovss	%xmm1, 16(%rsp)
	vmovss	736(%r13), %xmm1
	vmovss	%xmm1, 432(%rsp)
	vmovss	740(%r13), %xmm1
	vmovss	%xmm1, 424(%rsp)
	vmovss	744(%r13), %xmm1
	vmovss	%xmm1, 232(%rsp)
	vmovss	748(%r13), %xmm1
	vmovss	%xmm1, 228(%rsp)
	vmovss	752(%r13), %xmm1
	vmovss	%xmm1, 224(%rsp)
	vmovss	756(%r13), %xmm1
	vmovss	%xmm1, 220(%rsp)
	vmovss	760(%r13), %xmm1
	vmovss	%xmm1, 216(%rsp)
	vmovss	764(%r13), %xmm1
	vmovss	%xmm1, 212(%rsp)
	vmovss	768(%r13), %xmm1
	vmovss	%xmm1, 208(%rsp)
	vmovss	772(%r13), %xmm1
	vmovss	%xmm1, 204(%rsp)
	.loc	37 0 12 is_stmt 0
.Ltmp5073:
	.p2align	4
.LBB34_74:
	vmovss	56(%rsp), %xmm10
	vmovss	96(%rsp), %xmm15
	vmovss	80(%rsp), %xmm11
	vmovss	24(%rsp), %xmm14
	vmovss	136(%rsp), %xmm3
	vmovss	176(%rsp), %xmm6
	.loc	37 568 12 is_stmt 1
	leaq	(%r11,%rdx), %rax
	cmpq	%r12, %rax
	ja	.LBB34_78
.Ltmp5074:
	.loc	37 0 12 is_stmt 0
	vmovss	%xmm6, 240(%rsp)
	vmovss	%xmm3, 304(%rsp)
	.loc	15 1615 0 is_stmt 1
	movq	%rcx, %rax
	addq	%rdx, %rax
.Ltmp5075:
	.loc	39 51 9
	je	.LBB34_76
	.loc	39 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm8
	vmovaps	%xmm5, %xmm7
	vmovaps	%xmm2, %xmm13
	vmovaps	%xmm9, %xmm1
	vmovaps	%xmm4, %xmm3
	.loc	39 51 9
	vmovss	(%r9,%rdx,4), %xmm6
	vbroadcastss	.LCPI34_0(%rip), %xmm12
.Ltmp5076:
	.loc	39 103 24 is_stmt 1
	vandps	%xmm5, %xmm12, %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp5077:
	.loc	39 71 9
	vmulss	168(%rsp), %xmm6, %xmm0
	vxorps	%xmm9, %xmm9, %xmm9
.Ltmp5078:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5079:
	.loc	39 71 9
	vmulss	616(%rsp), %xmm6, %xmm4
.Ltmp5080:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5081:
	.loc	39 71 9
	vmulss	64(%rsp), %xmm6, %xmm5
.Ltmp5082:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
	vmovss	%xmm6, 56(%rsp)
.Ltmp5083:
	.loc	39 71 9
	vmulss	248(%rsp), %xmm6, %xmm6
.Ltmp5084:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovss	%xmm10, 296(%rsp)
.Ltmp5085:
	.loc	39 71 9
	vmulss	52(%rsp), %xmm10, %xmm9
.Ltmp5086:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5087:
	.loc	39 71 9
	vmulss	76(%rsp), %xmm10, %xmm9
.Ltmp5088:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5089:
	.loc	39 71 9
	vmulss	288(%rsp), %xmm10, %xmm9
.Ltmp5090:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5091:
	.loc	39 71 9
	vmulss	384(%rsp), %xmm10, %xmm9
.Ltmp5092:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5093:
	.loc	39 71 9
	vmulss	376(%rsp), %xmm3, %xmm9
.Ltmp5094:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5095:
	.loc	39 71 9
	vmulss	280(%rsp), %xmm3, %xmm9
.Ltmp5096:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5097:
	.loc	39 71 9
	vmulss	272(%rsp), %xmm3, %xmm9
.Ltmp5098:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5099:
	.loc	39 71 9
	vmulss	368(%rsp), %xmm3, %xmm9
.Ltmp5100:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5101:
	.loc	39 71 9
	vmulss	360(%rsp), %xmm1, %xmm9
.Ltmp5102:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5103:
	.loc	39 71 9
	vmulss	132(%rsp), %xmm1, %xmm9
.Ltmp5104:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5105:
	.loc	39 71 9
	vmulss	352(%rsp), %xmm1, %xmm9
.Ltmp5106:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5107:
	.loc	39 71 9
	vmulss	344(%rsp), %xmm1, %xmm9
.Ltmp5108:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm2, %xmm10
.Ltmp5109:
	.loc	39 71 9
	vmulss	264(%rsp), %xmm13, %xmm9
.Ltmp5110:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5111:
	.loc	39 71 9
	vmulss	336(%rsp), %xmm13, %xmm9
.Ltmp5112:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5113:
	.loc	39 71 9
	vmulss	328(%rsp), %xmm13, %xmm9
.Ltmp5114:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5115:
	.loc	39 71 9
	vmulss	496(%rsp), %xmm13, %xmm9
.Ltmp5116:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5117:
	.loc	39 71 9
	vmulss	488(%rsp), %xmm8, %xmm9
.Ltmp5118:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5119:
	.loc	39 71 9
	vmulss	480(%rsp), %xmm8, %xmm9
.Ltmp5120:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5121:
	.loc	39 71 9
	vmulss	408(%rsp), %xmm8, %xmm9
.Ltmp5122:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5123:
	.loc	39 71 9
	vmulss	472(%rsp), %xmm8, %xmm9
.Ltmp5124:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5125:
	.loc	39 71 9
	vmulss	164(%rsp), %xmm7, %xmm9
.Ltmp5126:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5127:
	.loc	39 71 9
	vmulss	256(%rsp), %xmm7, %xmm9
.Ltmp5128:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5129:
	.loc	39 71 9
	vmulss	160(%rsp), %xmm7, %xmm9
.Ltmp5130:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5131:
	.loc	39 71 9
	vmulss	156(%rsp), %xmm7, %xmm9
.Ltmp5132:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5133:
	.loc	39 71 9
	vmulss	152(%rsp), %xmm15, %xmm9
.Ltmp5134:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5135:
	.loc	39 71 9
	vmulss	148(%rsp), %xmm15, %xmm9
.Ltmp5136:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5137:
	.loc	39 71 9
	vmulss	320(%rsp), %xmm15, %xmm9
.Ltmp5138:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5139:
	.loc	39 71 9
	vmulss	464(%rsp), %xmm15, %xmm9
.Ltmp5140:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm2
	vmovaps	%xmm11, %xmm6
.Ltmp5141:
	.loc	39 71 9
	vmulss	400(%rsp), %xmm11, %xmm9
.Ltmp5142:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5143:
	.loc	39 71 9
	vmulss	456(%rsp), %xmm11, %xmm9
.Ltmp5144:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5145:
	.loc	39 71 9
	vmulss	448(%rsp), %xmm11, %xmm9
.Ltmp5146:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5147:
	.loc	39 71 9
	vmulss	440(%rsp), %xmm11, %xmm9
.Ltmp5148:
	.loc	39 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp5149:
	.loc	39 71 9
	vmulss	20(%rsp), %xmm14, %xmm9
.Ltmp5150:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5151:
	.loc	39 71 9
	vmulss	16(%rsp), %xmm14, %xmm9
.Ltmp5152:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5153:
	.loc	39 71 9
	vmulss	432(%rsp), %xmm14, %xmm9
.Ltmp5154:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5155:
	.loc	39 71 9
	vmulss	424(%rsp), %xmm14, %xmm9
.Ltmp5156:
	.loc	39 61 9
	vaddss	%xmm2, %xmm9, %xmm2
	vmovss	304(%rsp), %xmm11
.Ltmp5157:
	.loc	39 71 9
	vmulss	232(%rsp), %xmm11, %xmm9
.Ltmp5158:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5159:
	.loc	39 71 9
	vmulss	228(%rsp), %xmm11, %xmm9
.Ltmp5160:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5161:
	.loc	39 71 9
	vmulss	224(%rsp), %xmm11, %xmm9
.Ltmp5162:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5163:
	.loc	39 71 9
	vmulss	220(%rsp), %xmm11, %xmm9
.Ltmp5164:
	.loc	39 61 9
	vaddss	%xmm2, %xmm9, %xmm2
	vmovaps	%xmm8, %xmm13
	vmovss	240(%rsp), %xmm8
.Ltmp5165:
	.loc	39 71 9
	vmulss	216(%rsp), %xmm8, %xmm9
.Ltmp5166:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5167:
	.loc	39 71 9
	vmulss	212(%rsp), %xmm8, %xmm9
.Ltmp5168:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5169:
	.loc	39 71 9
	vmulss	208(%rsp), %xmm8, %xmm9
.Ltmp5170:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5171:
	.loc	39 71 9
	vmulss	204(%rsp), %xmm8, %xmm9
.Ltmp5172:
	.loc	39 61 9
	vaddss	%xmm2, %xmm9, %xmm2
	vmovaps	%xmm12, %xmm9
.Ltmp5173:
	.loc	39 103 24
	vandps	%xmm0, %xmm12, %xmm0
	vmovaps	176(%rsp), %xmm12
.Ltmp5174:
	.loc	39 161 24
	vmaxss	%xmm0, %xmm12, %xmm0
	vmovaps	%xmm1, %xmm12
	vmovss	296(%rsp), %xmm1
.Ltmp5175:
	.loc	39 103 24
	vandps	%xmm4, %xmm9, %xmm4
.Ltmp5176:
	.loc	39 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp5177:
	.loc	39 103 24
	vandps	%xmm5, %xmm9, %xmm4
.Ltmp5178:
	.loc	39 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp5179:
	.loc	39 103 24
	vandps	%xmm2, %xmm9, %xmm4
.Ltmp5180:
	.loc	39 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp5181:
	.loc	39 56 9
	vmovss	%xmm0, 2184(%rsp,%rdx,4)
.Ltmp5182:
	.loc	15 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm11, 176(%rsp)
	vmovss	%xmm14, 136(%rsp)
	vmovss	%xmm6, 24(%rsp)
	vmovss	%xmm15, 80(%rsp)
	vmovss	%xmm7, 96(%rsp)
	vmovaps	%xmm13, %xmm5
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm12, %xmm2
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm1, %xmm4
.Ltmp5183:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %r14
.Ltmp5184:
	.loc	11 900 12
	jne	.LBB34_74
.Ltmp5185:
.LBB34_80:
	.loc	11 0 12 is_stmt 0
	vmovss	56(%rsp), %xmm0
	.loc	15 1618 5 is_stmt 1
	vmovss	%xmm0, 812(%rsp)
	vmovss	%xmm1, 816(%rsp)
	vmovss	%xmm3, 820(%rsp)
	vmovss	%xmm12, 824(%rsp)
	vmovss	%xmm10, 828(%rsp)
	vmovss	%xmm13, 832(%rsp)
	vmovss	%xmm7, 836(%rsp)
	vmovss	%xmm15, 840(%rsp)
	vmovss	%xmm6, 844(%rsp)
	vmovss	%xmm14, 848(%rsp)
	vmovss	%xmm11, 852(%rsp)
	vmovss	%xmm8, 856(%rsp)
.Ltmp5186:
	.loc	15 1612 23
	vmovss	512(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	516(%rsp), %xmm1
	vmovss	520(%rsp), %xmm3
	vmovss	524(%rsp), %xmm12
	vmovss	528(%rsp), %xmm10
	vmovss	532(%rsp), %xmm13
	vmovss	536(%rsp), %xmm7
	vmovss	540(%rsp), %xmm15
	vmovss	544(%rsp), %xmm6
	vmovss	548(%rsp), %xmm14
	vmovss	552(%rsp), %xmm11
.Ltmp5187:
	.loc	10 1916 50
	cmpq	%r11, %rbx
.Ltmp5188:
	.loc	11 900 12
	jne	.LBB34_82
.Ltmp5189:
	.loc	15 0 0 is_stmt 0
	vmovss	556(%rsp), %xmm8
.Ltmp5190:
	.loc	11 900 12
	jmp	.LBB34_86
.Ltmp5191:
	.loc	11 0 12
.Ltmp5192:
	.p2align	4
.LBB34_82:
	vmovss	584(%r13), %xmm0
	vmovss	%xmm0, 168(%rsp)
	vmovss	588(%r13), %xmm0
	vmovss	%xmm0, 616(%rsp)
	vmovss	592(%r13), %xmm0
	vmovss	%xmm0, 64(%rsp)
	vmovss	596(%r13), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	600(%r13), %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	604(%r13), %xmm0
	vmovss	%xmm0, 76(%rsp)
	movq	984(%rsp), %rax
.Ltmp5193:
	.loc	37 568 12 is_stmt 1
	leaq	(%rax,%r11), %rcx
	xorl	%edx, %edx
	vmovss	%xmm11, 176(%rsp)
	vmovss	%xmm14, 136(%rsp)
	vmovss	%xmm6, 24(%rsp)
	vmovss	%xmm15, 80(%rsp)
	vmovss	%xmm7, 96(%rsp)
	vmovaps	%xmm13, %xmm5
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm12, %xmm2
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm1, %xmm4
	vmovss	608(%r13), %xmm1
	vmovss	%xmm1, 288(%rsp)
	vmovss	612(%r13), %xmm1
	vmovss	%xmm1, 384(%rsp)
	vmovss	616(%r13), %xmm1
	vmovss	%xmm1, 376(%rsp)
	vmovss	620(%r13), %xmm1
	vmovss	%xmm1, 280(%rsp)
	vmovss	624(%r13), %xmm1
	vmovss	%xmm1, 272(%rsp)
	vmovss	628(%r13), %xmm1
	vmovss	%xmm1, 368(%rsp)
	vmovss	632(%r13), %xmm1
	vmovss	%xmm1, 360(%rsp)
	vmovss	636(%r13), %xmm1
	vmovss	%xmm1, 132(%rsp)
	vmovss	640(%r13), %xmm1
	vmovss	%xmm1, 352(%rsp)
	vmovss	644(%r13), %xmm1
	vmovss	%xmm1, 344(%rsp)
	vmovss	648(%r13), %xmm1
	vmovss	%xmm1, 264(%rsp)
	vmovss	652(%r13), %xmm1
	vmovss	%xmm1, 336(%rsp)
	vmovss	656(%r13), %xmm1
	vmovss	%xmm1, 328(%rsp)
	vmovss	660(%r13), %xmm1
	vmovss	%xmm1, 496(%rsp)
	vmovss	664(%r13), %xmm1
	vmovss	%xmm1, 488(%rsp)
	vmovss	668(%r13), %xmm1
	vmovss	%xmm1, 480(%rsp)
	vmovss	672(%r13), %xmm1
	vmovss	%xmm1, 408(%rsp)
	vmovss	676(%r13), %xmm1
	vmovss	%xmm1, 472(%rsp)
	vmovss	680(%r13), %xmm1
	vmovss	%xmm1, 164(%rsp)
	vmovss	684(%r13), %xmm1
	vmovss	%xmm1, 256(%rsp)
	vmovss	688(%r13), %xmm1
	vmovss	%xmm1, 160(%rsp)
	vmovss	692(%r13), %xmm1
	vmovss	%xmm1, 156(%rsp)
	vmovss	696(%r13), %xmm1
	vmovss	%xmm1, 152(%rsp)
	vmovss	700(%r13), %xmm1
	vmovss	%xmm1, 148(%rsp)
	vmovss	704(%r13), %xmm1
	vmovss	%xmm1, 320(%rsp)
	vmovss	708(%r13), %xmm1
	vmovss	%xmm1, 464(%rsp)
	vmovss	712(%r13), %xmm1
	vmovss	%xmm1, 400(%rsp)
	vmovss	716(%r13), %xmm1
	vmovss	%xmm1, 456(%rsp)
	vmovss	720(%r13), %xmm1
	vmovss	%xmm1, 448(%rsp)
	vmovss	724(%r13), %xmm1
	vmovss	%xmm1, 440(%rsp)
	vmovss	728(%r13), %xmm1
	vmovss	%xmm1, 20(%rsp)
	vmovss	732(%r13), %xmm1
	vmovss	%xmm1, 16(%rsp)
	vmovss	736(%r13), %xmm1
	vmovss	%xmm1, 432(%rsp)
	vmovss	740(%r13), %xmm1
	vmovss	%xmm1, 424(%rsp)
	vmovss	744(%r13), %xmm1
	vmovss	%xmm1, 232(%rsp)
	vmovss	748(%r13), %xmm1
	vmovss	%xmm1, 228(%rsp)
	vmovss	752(%r13), %xmm1
	vmovss	%xmm1, 224(%rsp)
	vmovss	756(%r13), %xmm1
	vmovss	%xmm1, 220(%rsp)
	vmovss	760(%r13), %xmm1
	vmovss	%xmm1, 216(%rsp)
	vmovss	764(%r13), %xmm1
	vmovss	%xmm1, 212(%rsp)
	vmovss	768(%r13), %xmm1
	vmovss	%xmm1, 208(%rsp)
	vmovss	772(%r13), %xmm1
	vmovss	%xmm1, 204(%rsp)
	.loc	37 0 12 is_stmt 0
.Ltmp5194:
	.p2align	4
.LBB34_83:
	vmovss	56(%rsp), %xmm10
	vmovss	96(%rsp), %xmm15
	vmovss	80(%rsp), %xmm11
	vmovss	24(%rsp), %xmm14
	vmovss	136(%rsp), %xmm3
	vmovss	176(%rsp), %xmm6
	.loc	37 568 12 is_stmt 1
	leaq	(%r11,%rdx), %rax
	cmpq	%rbp, %rax
	ja	.LBB34_421
.Ltmp5195:
	.loc	37 0 12 is_stmt 0
	vmovss	%xmm6, 240(%rsp)
	vmovss	%xmm3, 304(%rsp)
	.loc	15 1615 0 is_stmt 1
	movq	%rcx, %rax
	addq	%rdx, %rax
.Ltmp5196:
	.loc	39 51 9
	je	.LBB34_76
	.loc	39 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm8
	vmovaps	%xmm5, %xmm7
	vmovaps	%xmm2, %xmm13
	vmovaps	%xmm9, %xmm1
	vmovaps	%xmm4, %xmm3
	.loc	39 51 9
	vmovss	(%r8,%rdx,4), %xmm6
	vbroadcastss	.LCPI34_0(%rip), %xmm12
.Ltmp5197:
	.loc	39 103 24 is_stmt 1
	vandps	%xmm5, %xmm12, %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp5198:
	.loc	39 71 9
	vmulss	168(%rsp), %xmm6, %xmm0
	vxorps	%xmm9, %xmm9, %xmm9
.Ltmp5199:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5200:
	.loc	39 71 9
	vmulss	616(%rsp), %xmm6, %xmm4
.Ltmp5201:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5202:
	.loc	39 71 9
	vmulss	64(%rsp), %xmm6, %xmm5
.Ltmp5203:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
	vmovss	%xmm6, 56(%rsp)
.Ltmp5204:
	.loc	39 71 9
	vmulss	248(%rsp), %xmm6, %xmm6
.Ltmp5205:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovss	%xmm10, 296(%rsp)
.Ltmp5206:
	.loc	39 71 9
	vmulss	52(%rsp), %xmm10, %xmm9
.Ltmp5207:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5208:
	.loc	39 71 9
	vmulss	76(%rsp), %xmm10, %xmm9
.Ltmp5209:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5210:
	.loc	39 71 9
	vmulss	288(%rsp), %xmm10, %xmm9
.Ltmp5211:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5212:
	.loc	39 71 9
	vmulss	384(%rsp), %xmm10, %xmm9
.Ltmp5213:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5214:
	.loc	39 71 9
	vmulss	376(%rsp), %xmm3, %xmm9
.Ltmp5215:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5216:
	.loc	39 71 9
	vmulss	280(%rsp), %xmm3, %xmm9
.Ltmp5217:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5218:
	.loc	39 71 9
	vmulss	272(%rsp), %xmm3, %xmm9
.Ltmp5219:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5220:
	.loc	39 71 9
	vmulss	368(%rsp), %xmm3, %xmm9
.Ltmp5221:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5222:
	.loc	39 71 9
	vmulss	360(%rsp), %xmm1, %xmm9
.Ltmp5223:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5224:
	.loc	39 71 9
	vmulss	132(%rsp), %xmm1, %xmm9
.Ltmp5225:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5226:
	.loc	39 71 9
	vmulss	352(%rsp), %xmm1, %xmm9
.Ltmp5227:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5228:
	.loc	39 71 9
	vmulss	344(%rsp), %xmm1, %xmm9
.Ltmp5229:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm2, %xmm10
.Ltmp5230:
	.loc	39 71 9
	vmulss	264(%rsp), %xmm13, %xmm9
.Ltmp5231:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5232:
	.loc	39 71 9
	vmulss	336(%rsp), %xmm13, %xmm9
.Ltmp5233:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5234:
	.loc	39 71 9
	vmulss	328(%rsp), %xmm13, %xmm9
.Ltmp5235:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5236:
	.loc	39 71 9
	vmulss	496(%rsp), %xmm13, %xmm9
.Ltmp5237:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5238:
	.loc	39 71 9
	vmulss	488(%rsp), %xmm8, %xmm9
.Ltmp5239:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5240:
	.loc	39 71 9
	vmulss	480(%rsp), %xmm8, %xmm9
.Ltmp5241:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5242:
	.loc	39 71 9
	vmulss	408(%rsp), %xmm8, %xmm9
.Ltmp5243:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5244:
	.loc	39 71 9
	vmulss	472(%rsp), %xmm8, %xmm9
.Ltmp5245:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5246:
	.loc	39 71 9
	vmulss	164(%rsp), %xmm7, %xmm9
.Ltmp5247:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5248:
	.loc	39 71 9
	vmulss	256(%rsp), %xmm7, %xmm9
.Ltmp5249:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5250:
	.loc	39 71 9
	vmulss	160(%rsp), %xmm7, %xmm9
.Ltmp5251:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5252:
	.loc	39 71 9
	vmulss	156(%rsp), %xmm7, %xmm9
.Ltmp5253:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp5254:
	.loc	39 71 9
	vmulss	152(%rsp), %xmm15, %xmm9
.Ltmp5255:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5256:
	.loc	39 71 9
	vmulss	148(%rsp), %xmm15, %xmm9
.Ltmp5257:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5258:
	.loc	39 71 9
	vmulss	320(%rsp), %xmm15, %xmm9
.Ltmp5259:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5260:
	.loc	39 71 9
	vmulss	464(%rsp), %xmm15, %xmm9
.Ltmp5261:
	.loc	39 61 9
	vaddss	%xmm6, %xmm9, %xmm2
	vmovaps	%xmm11, %xmm6
.Ltmp5262:
	.loc	39 71 9
	vmulss	400(%rsp), %xmm11, %xmm9
.Ltmp5263:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5264:
	.loc	39 71 9
	vmulss	456(%rsp), %xmm11, %xmm9
.Ltmp5265:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5266:
	.loc	39 71 9
	vmulss	448(%rsp), %xmm11, %xmm9
.Ltmp5267:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5268:
	.loc	39 71 9
	vmulss	440(%rsp), %xmm11, %xmm9
.Ltmp5269:
	.loc	39 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp5270:
	.loc	39 71 9
	vmulss	20(%rsp), %xmm14, %xmm9
.Ltmp5271:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5272:
	.loc	39 71 9
	vmulss	16(%rsp), %xmm14, %xmm9
.Ltmp5273:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5274:
	.loc	39 71 9
	vmulss	432(%rsp), %xmm14, %xmm9
.Ltmp5275:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5276:
	.loc	39 71 9
	vmulss	424(%rsp), %xmm14, %xmm9
.Ltmp5277:
	.loc	39 61 9
	vaddss	%xmm2, %xmm9, %xmm2
	vmovss	304(%rsp), %xmm11
.Ltmp5278:
	.loc	39 71 9
	vmulss	232(%rsp), %xmm11, %xmm9
.Ltmp5279:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5280:
	.loc	39 71 9
	vmulss	228(%rsp), %xmm11, %xmm9
.Ltmp5281:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5282:
	.loc	39 71 9
	vmulss	224(%rsp), %xmm11, %xmm9
.Ltmp5283:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5284:
	.loc	39 71 9
	vmulss	220(%rsp), %xmm11, %xmm9
.Ltmp5285:
	.loc	39 61 9
	vaddss	%xmm2, %xmm9, %xmm2
	vmovaps	%xmm8, %xmm13
	vmovss	240(%rsp), %xmm8
.Ltmp5286:
	.loc	39 71 9
	vmulss	216(%rsp), %xmm8, %xmm9
.Ltmp5287:
	.loc	39 61 9
	vaddss	%xmm0, %xmm9, %xmm0
.Ltmp5288:
	.loc	39 71 9
	vmulss	212(%rsp), %xmm8, %xmm9
.Ltmp5289:
	.loc	39 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp5290:
	.loc	39 71 9
	vmulss	208(%rsp), %xmm8, %xmm9
.Ltmp5291:
	.loc	39 61 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5292:
	.loc	39 71 9
	vmulss	204(%rsp), %xmm8, %xmm9
.Ltmp5293:
	.loc	39 61 9
	vaddss	%xmm2, %xmm9, %xmm2
	vmovaps	%xmm12, %xmm9
.Ltmp5294:
	.loc	39 103 24
	vandps	%xmm0, %xmm12, %xmm0
	vmovaps	176(%rsp), %xmm12
.Ltmp5295:
	.loc	39 161 24
	vmaxss	%xmm0, %xmm12, %xmm0
	vmovaps	%xmm1, %xmm12
	vmovss	296(%rsp), %xmm1
.Ltmp5296:
	.loc	39 103 24
	vandps	%xmm4, %xmm9, %xmm4
.Ltmp5297:
	.loc	39 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp5298:
	.loc	39 103 24
	vandps	%xmm5, %xmm9, %xmm4
.Ltmp5299:
	.loc	39 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp5300:
	.loc	39 103 24
	vandps	%xmm2, %xmm9, %xmm4
.Ltmp5301:
	.loc	39 161 24
	vmaxss	%xmm4, %xmm0, %xmm0
.Ltmp5302:
	.loc	39 56 9
	vmovss	%xmm0, 1152(%rsp,%rdx,4)
.Ltmp5303:
	.loc	15 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm11, 176(%rsp)
	vmovss	%xmm14, 136(%rsp)
	vmovss	%xmm6, 24(%rsp)
	vmovss	%xmm15, 80(%rsp)
	vmovss	%xmm7, 96(%rsp)
	vmovaps	%xmm13, %xmm5
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm12, %xmm2
	vmovaps	%xmm3, %xmm9
	vmovaps	%xmm1, %xmm4
.Ltmp5304:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %r14
.Ltmp5305:
	.loc	11 900 12
	jne	.LBB34_83
.Ltmp5306:
.LBB34_86:
	.loc	11 0 12 is_stmt 0
	vmovss	56(%rsp), %xmm0
	.loc	15 1618 5 is_stmt 1
	vmovss	%xmm0, 512(%rsp)
	vmovss	%xmm1, 516(%rsp)
	vmovss	%xmm3, 520(%rsp)
	vmovss	%xmm12, 524(%rsp)
	vmovss	%xmm10, 528(%rsp)
	vmovss	%xmm13, 532(%rsp)
	vmovss	%xmm7, 536(%rsp)
	vmovss	%xmm15, 540(%rsp)
	vmovss	%xmm6, 544(%rsp)
	vmovss	%xmm14, 548(%rsp)
	vmovss	%xmm11, 552(%rsp)
	vmovss	%xmm8, 556(%rsp)
.Ltmp5307:
	.loc	10 1916 50
	cmpq	%r11, %rbx
.Ltmp5308:
	.loc	11 900 12
	je	.LBB34_70
.Ltmp5309:
	.loc	11 0 12 is_stmt 0
	movq	%r14, 280(%rsp)
	movq	%r11, 64(%rsp)
	movq	%r10, 328(%rsp)
	movq	%r9, 336(%rsp)
	movq	%r8, 264(%rsp)
	movq	%rdi, 344(%rsp)
	vmovss	860(%rsp), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	864(%rsp), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	880(%rsp), %xmm0
	vmovss	%xmm0, 360(%rsp)
	vmovss	560(%rsp), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	564(%rsp), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	580(%rsp), %xmm0
	vmovss	%xmm0, 352(%rsp)
	movq	544(%r13), %rbp
	movq	552(%r13), %rax
	movq	%rax, 272(%rsp)
	vmovss	900(%rsp), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	600(%rsp), %xmm0
	vmovss	%xmm0, 376(%rsp)
	vmovss	872(%rsp), %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	868(%rsp), %xmm15
	vmovss	888(%rsp), %xmm0
	vmovss	%xmm0, 76(%rsp)
	vmovss	876(%rsp), %xmm11
	vmovss	884(%rsp), %xmm12
	vmovss	572(%rsp), %xmm0
	vmovss	%xmm0, 248(%rsp)
	vmovss	568(%rsp), %xmm3
	vmovss	588(%rsp), %xmm0
	vmovss	%xmm0, 288(%rsp)
	xorl	%ebx, %ebx
	vmovss	576(%rsp), %xmm4
	vmovss	584(%rsp), %xmm8
	vmovss	896(%rsp), %xmm14
	vmovss	892(%rsp), %xmm10
	vmovss	596(%rsp), %xmm13
	vmovss	592(%rsp), %xmm2
	movq	504(%rsp), %r12
	movl	416(%rsp), %r15d
	vmovss	.LCPI34_2(%rip), %xmm6
	.p2align	4
.LBB34_88:
.Ltmp5310:
	.loc	15 1701 77 is_stmt 1
	cmpb	$0, 12(%rsp)
	jne	.LBB34_102
	.loc	15 0 77 is_stmt 0
	vmovss	52(%rsp), %xmm0
.Ltmp5311:
	.loc	39 66 9 is_stmt 1
	vaddss	.LCPI34_1(%rip), %xmm0, %xmm0
.Ltmp5312:
	.loc	39 124 14
	vxorps	%xmm1, %xmm1, %xmm1
	vmovaps	%xmm3, %xmm7
	vucomiss	%xmm1, %xmm0
.Ltmp5313:
	.loc	39 161 24
	ja	.LBB34_90
.Ltmp5314:
	.loc	39 0 24 is_stmt 0
	vmovss	368(%rsp), %xmm1
	jmp	.LBB34_92
	.p2align	4
.LBB34_90:
	vmovss	80(%rsp), %xmm1
	.loc	15 748 0 is_stmt 1
	vaddss	%xmm1, %xmm15, %xmm1
.LBB34_92:
	.loc	15 0 0 is_stmt 0
	vmovss	%xmm1, 80(%rsp)
	vxorps	%xmm5, %xmm5, %xmm5
.Ltmp5315:
	.loc	39 161 24 is_stmt 1
	vcmpltss	%xmm0, %xmm5, %xmm1
	vandps	%xmm1, %xmm15, %xmm15
.Ltmp5316:
	.loc	15 749 9
	vmovss	%xmm15, 868(%rsp)
	vmovss	76(%rsp), %xmm1
.Ltmp5317:
	.loc	39 66 9
	vaddss	.LCPI34_1(%rip), %xmm1, %xmm1
.Ltmp5318:
	.loc	39 124 14
	vucomiss	%xmm5, %xmm1
.Ltmp5319:
	.loc	39 161 24
	vmaxss	%xmm5, %xmm1, %xmm3
	vmovss	%xmm3, 76(%rsp)
.Ltmp5320:
	.loc	15 746 9
	vmovss	%xmm3, 888(%rsp)
.Ltmp5321:
	.loc	39 161 24
	ja	.LBB34_93
.Ltmp5322:
	.loc	39 0 24 is_stmt 0
	vmovss	360(%rsp), %xmm11
	jmp	.LBB34_95
	.p2align	4
.LBB34_93:
	.loc	15 748 0 is_stmt 1
	vaddss	%xmm12, %xmm11, %xmm11
.LBB34_95:
	.loc	15 0 0 is_stmt 0
	vmovaps	%xmm7, %xmm3
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 876(%rsp)
.Ltmp5323:
	.loc	39 161 24
	vcmpltss	%xmm1, %xmm5, %xmm1
	vandps	%xmm1, %xmm12, %xmm12
.Ltmp5324:
	.loc	15 749 9
	vmovss	%xmm12, 884(%rsp)
	vmovss	248(%rsp), %xmm1
.Ltmp5325:
	.loc	39 66 9
	vaddss	.LCPI34_1(%rip), %xmm1, %xmm1
.Ltmp5326:
	.loc	39 124 14
	vucomiss	%xmm5, %xmm1
.Ltmp5327:
	.loc	39 161 24
	ja	.LBB34_96
.Ltmp5328:
	.loc	39 0 24 is_stmt 0
	vmovss	132(%rsp), %xmm5
	jmp	.LBB34_98
	.p2align	4
.LBB34_96:
	vmovss	96(%rsp), %xmm5
	.loc	15 748 0 is_stmt 1
	vaddss	%xmm3, %xmm5, %xmm5
.LBB34_98:
	.loc	15 0 0 is_stmt 0
	vmovss	%xmm5, 96(%rsp)
	vxorps	%xmm9, %xmm9, %xmm9
.Ltmp5329:
	.loc	39 161 24 is_stmt 1
	vcmpltss	%xmm1, %xmm9, %xmm5
	vandps	%xmm3, %xmm5, %xmm3
.Ltmp5330:
	.loc	15 749 9
	vmovss	%xmm3, 568(%rsp)
	vmovss	288(%rsp), %xmm5
.Ltmp5331:
	.loc	39 66 9
	vaddss	.LCPI34_1(%rip), %xmm5, %xmm5
.Ltmp5332:
	.loc	39 124 14
	vucomiss	%xmm9, %xmm5
.Ltmp5333:
	.loc	39 161 24
	vmaxss	%xmm9, %xmm5, %xmm7
	vmovss	%xmm7, 288(%rsp)
.Ltmp5334:
	.loc	15 746 9
	vmovss	%xmm7, 588(%rsp)
.Ltmp5335:
	.loc	39 161 24
	ja	.LBB34_99
.Ltmp5336:
	.loc	39 0 24 is_stmt 0
	vmovss	352(%rsp), %xmm4
	jmp	.LBB34_101
	.p2align	4
.LBB34_99:
	.loc	15 748 0 is_stmt 1
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp5337:
.LBB34_101:
	.loc	15 0 0 is_stmt 0
	vmaxss	%xmm9, %xmm0, %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmaxss	%xmm9, %xmm1, %xmm0
	vmovss	%xmm0, 248(%rsp)
.Ltmp5338:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm4, 576(%rsp)
.Ltmp5339:
	.loc	39 161 24
	vcmpltss	%xmm5, %xmm9, %xmm0
	vandps	%xmm0, %xmm8, %xmm8
.Ltmp5340:
	.loc	15 749 9
	vmovss	%xmm8, 584(%rsp)
.Ltmp5341:
.LBB34_102:
	.loc	39 51 9
	vmovss	2184(%rsp,%rbx,4), %xmm0
.Ltmp5342:
	.loc	39 51 9 is_stmt 0
	vmovss	1152(%rsp,%rbx,4), %xmm1
.Ltmp5343:
	.loc	39 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
	movq	64(%rsp), %rax
.Ltmp5344:
	.loc	15 0 0 is_stmt 0
	leaq	(%rbx,%rax), %rdi
.Ltmp5345:
	.loc	19 1244 18 is_stmt 1
	vmovd	%xmm1, %ecx
.Ltmp5346:
	.loc	19 1244 18 is_stmt 0
	vmovd	%xmm0, %eax
.Ltmp5347:
	.loc	39 161 24 is_stmt 1
	movl	%ecx, %r8d
	cmovbel	%eax, %r8d
	movq	912(%rsp), %rdx
.Ltmp5348:
	.loc	37 568 12
	cmpq	%rdx, %rdi
	ja	.LBB34_200
.Ltmp5349:
	.loc	39 51 9
	je	.LBB34_104
.Ltmp5350:
	.loc	15 0 0 is_stmt 0
	andl	392(%rsp), %r8d
	andl	%r15d, %eax
	orl	%r8d, %eax
	vmovd	%eax, %xmm1
.Ltmp5351:
	.loc	39 124 14 is_stmt 1
	vucomiss	80(%rsp), %xmm1
	vmovaps	%xmm6, %xmm0
.Ltmp5352:
	.loc	39 161 24
	jbe	.LBB34_107
.Ltmp5353:
	.loc	39 0 24 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp5354:
	.loc	39 76 9 is_stmt 1
	vdivss	%xmm1, %xmm0, %xmm0
.Ltmp5355:
.LBB34_107:
	.loc	15 0 0 is_stmt 0
	movq	328(%r13), %rdx
.Ltmp5356:
	.loc	15 1287 25 is_stmt 1
	movq	176(%r13), %rsi
	movq	%rdx, 304(%rsp)
	.loc	15 1287 45 is_stmt 0
	imulq	32(%rsp), %rdx
.Ltmp5357:
	.loc	37 580 12 is_stmt 1
	cmpq	%rsi, %rdx
	ja	.LBB34_136
.Ltmp5358:
	.loc	37 0 12 is_stmt 0
	movl	%r8d, 296(%rsp)
	movl	%ecx, 168(%rsp)
	movq	%rbx, 616(%rsp)
.Ltmp5359:
	.loc	39 56 9 is_stmt 1
	je	.LBB34_109
.Ltmp5360:
	.loc	39 0 9 is_stmt 0
	movq	%rdi, 24(%rsp)
	vmovss	(%r12,%rdi,4), %xmm9
.Ltmp5361:
	.loc	15 1287 0 is_stmt 1
	movq	168(%r13), %rax
	movq	%rdx, 240(%rsp)
.Ltmp5362:
	.loc	39 56 9
	vmovss	%xmm0, (%rax,%rdx,4)
.Ltmp5363:
	.loc	15 1154 17
	movq	328(%r13), %r12
.Ltmp5364:
	.loc	43 37 12
	testq	%r12, %r12
	je	.LBB34_124
.Ltmp5365:
	.loc	43 0 12 is_stmt 0
	movq	920(%rsp), %rdx
	movq	320(%rdx), %rax
	movq	%rax, 56(%rsp)
	movq	32(%rsp), %rax
	leaq	1(%rax), %rdi
	cmpq	%rbp, %rdi
	movq	%rbp, %rcx
	movl	$0, %eax
	cmovbq	%rax, %rcx
	movq	312(%rdx), %rax
	subq	%rcx, %rdi
	movq	168(%rdx), %rbx
	movq	176(%rdx), %rsi
	movq	256(%rdx), %rcx
	movq	%rcx, 176(%rsp)
	movq	248(%rdx), %r15
	movq	224(%rdx), %r9
	movq	216(%rdx), %rdx
	imulq	%r12, %rdi
	movq	%rdi, 136(%rsp)
	movq	%r12, %r13
	xorl	%r11d, %r11d
	xorl	%ecx, %ecx
	jmp	.LBB34_112
	.p2align	4
.LBB34_135:
	xorl	%r10d, %r10d
.LBB34_123:
	decq	%r13
	addq	$4, %rcx
.Ltmp5366:
	movl	%r10d, (%r15,%r11,4)
.Ltmp5367:
	incq	%r11
.Ltmp5368:
	.loc	43 37 12 is_stmt 1
	testq	%r13, %r13
	je	.LBB34_124
.LBB34_112:
.Ltmp5369:
	.loc	16 1714 9
	cmpq	$32, %rcx
.Ltmp5370:
	.loc	17 180 28
	je	.LBB34_124
.Ltmp5371:
	.loc	15 1158 21
	cmpq	56(%rsp), %r11
	je	.LBB34_201
	leaq	(%r11,%r11,2), %r8
	movl	4(%rax,%r8,4), %edi
.Ltmp5372:
	.loc	15 1160 23
	addq	32(%rsp), %rdi
.Ltmp5373:
	.loc	15 1161 12
	cmpq	%rbp, %rdi
	movl	$0, %r10d
	cmovaeq	%rbp, %r10
	subq	%r10, %rdi
.Ltmp5374:
	.loc	15 1168 42
	movq	%rdi, %r14
	imulq	%r12, %r14
	addq	%r11, %r14
	.loc	15 1168 22 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_202
.Ltmp5375:
	.loc	15 1169 24 is_stmt 1
	cmpq	176(%rsp), %r11
	je	.LBB34_203
.Ltmp5376:
	.loc	15 0 0 is_stmt 0
	movl	(%rax,%r8,4), %r8d
.Ltmp5377:
	vmovss	(%rbx,%r14,4), %xmm0
.Ltmp5378:
	.loc	15 1169 24
	movl	(%r15,%r11,4), %r10d
	testq	%r10, %r10
.Ltmp5379:
	.loc	15 1170 26 is_stmt 1
	je	.LBB34_119
	.loc	15 1173 24
	cmpq	%r9, %r11
	jae	.LBB34_204
	vmovss	(%rdx,%r11,4), %xmm1
.Ltmp5380:
	.loc	15 798 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB34_119
.Ltmp5381:
	.loc	15 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB34_119:
.Ltmp5382:
	.loc	15 1175 9 is_stmt 1
	cmpq	%r9, %r11
	je	.LBB34_206
	vmovss	%xmm0, (%rdx,%r11,4)
	.loc	15 1176 24
	incq	%r10
	cmpq	%r8, %r10
.Ltmp5383:
	.loc	15 1177 23
	jne	.LBB34_121
	.loc	15 1177 9 is_stmt 0
	vmovss	%xmm0, 928(%rsp,%rcx)
	.loc	15 1183 30 is_stmt 1
	vmovss	(%rbx,%r14,4), %xmm0
	.loc	15 0 30 is_stmt 0
.Ltmp5384:
	.p2align	4
.LBB34_133:
.Ltmp5385:
	.loc	15 1186 65 is_stmt 1
	movq	%rdi, %r14
	imulq	%r12, %r14
	addq	%r11, %r14
	.loc	15 1186 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_424
.Ltmp5386:
	.loc	15 798 8 is_stmt 1
	vminss	(%rbx,%r14,4), %xmm0, %xmm0
.Ltmp5387:
	.loc	15 1187 17
	vmovss	%xmm0, (%rbx,%r14,4)
	.loc	15 1188 20
	testq	%rdi, %rdi
	cmoveq	%rbp, %rdi
	.loc	15 1191 17
	decq	%rdi
.Ltmp5388:
	.loc	10 1916 50
	decq	%r8
.Ltmp5389:
	.loc	11 900 12
	jne	.LBB34_133
	jmp	.LBB34_135
.Ltmp5390:
	.loc	11 0 12 is_stmt 0
.Ltmp5391:
	.p2align	4
.LBB34_121:
	movq	136(%rsp), %rdi
	.loc	15 1180 44 is_stmt 1
	leaq	(%r11,%rdi), %r14
	.loc	15 1180 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_423
	vmovss	(%rbx,%r14,4), %xmm1
.Ltmp5392:
	.loc	15 798 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp5393:
	.loc	15 1177 9
	vmovss	%xmm0, 928(%rsp,%rcx)
	jmp	.LBB34_123
.Ltmp5394:
	.loc	15 0 9 is_stmt 0
.Ltmp5395:
	.p2align	4
.LBB34_124:
	.loc	39 51 9 is_stmt 1
	vmovss	928(%rsp), %xmm1
.Ltmp5396:
	.loc	39 71 9
	vmulss	.LCPI34_3(%rip), %xmm1, %xmm0
.Ltmp5397:
	.loc	19 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp5398:
	.loc	39 71 9
	vmulss	.LCPI34_4(%rip), %xmm0, %xmm0
	movq	920(%rsp), %r13
.Ltmp5399:
	.loc	15 1307 26
	movq	192(%r13), %rsi
	movq	304(%rsp), %rbx
.Ltmp5400:
	.loc	43 37 12
	testq	%rbx, %rbx
	je	.LBB34_138
.Ltmp5401:
	.loc	43 0 12 is_stmt 0
	movq	320(%r13), %r10
.Ltmp5402:
	.loc	15 1298 42 is_stmt 1
	testq	%r10, %r10
	je	.LBB34_131
	.loc	15 0 42 is_stmt 0
	movq	312(%r13), %r11
	.loc	15 1298 42
	movl	8(%r11), %r8d
	.loc	15 1298 28
	addq	32(%rsp), %r8
.Ltmp5403:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %r8
	movl	$0, %eax
	cmovaeq	%rbp, %rax
	subq	%rax, %r8
	.loc	15 1302 40
	imulq	%rbx, %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	184(%r13), %r9
	.loc	15 1302 25
	vmovss	(%r9,%r8,4), %xmm1
	.loc	15 1302 13
	vmovss	%xmm1, 928(%rsp)
.Ltmp5404:
	.loc	43 37 12 is_stmt 1
	cmpq	$1, %rbx
	je	.LBB34_138
.Ltmp5405:
	.loc	15 1298 42
	cmpq	$1, %r10
	je	.LBB34_129
	movl	20(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5406:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rbx, %rax
	leaq	1(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	vmovss	4(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 932(%rsp)
.Ltmp5407:
	.loc	43 37 12 is_stmt 1
	cmpq	$2, %rbx
	je	.LBB34_138
.Ltmp5408:
	.loc	15 1298 42
	cmpq	$2, %r10
	je	.LBB34_177
	movl	32(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5409:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rbx, %rax
	leaq	2(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	vmovss	8(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 936(%rsp)
.Ltmp5410:
	.loc	43 37 12 is_stmt 1
	cmpq	$3, %rbx
	je	.LBB34_138
.Ltmp5411:
	.loc	15 1298 42
	cmpq	$3, %r10
	je	.LBB34_181
	movl	44(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5412:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rbx, %rax
	leaq	3(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	vmovss	12(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 940(%rsp)
.Ltmp5413:
	.loc	43 37 12 is_stmt 1
	cmpq	$4, %rbx
	je	.LBB34_138
.Ltmp5414:
	.loc	15 1298 42
	cmpq	$4, %r10
	je	.LBB34_185
	movl	56(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5415:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rbx, %rax
	leaq	4(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	vmovss	16(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 944(%rsp)
.Ltmp5416:
	.loc	43 37 12 is_stmt 1
	cmpq	$5, %rbx
	je	.LBB34_138
.Ltmp5417:
	.loc	15 1298 42
	cmpq	$5, %r10
	je	.LBB34_189
	movl	68(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5418:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rbx, %rax
	leaq	5(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	vmovss	20(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 948(%rsp)
.Ltmp5419:
	.loc	43 37 12 is_stmt 1
	cmpq	$6, %rbx
	je	.LBB34_138
.Ltmp5420:
	.loc	15 1298 42
	cmpq	$6, %r10
	je	.LBB34_193
	movl	80(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5421:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rbx, %rax
	leaq	6(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	vmovss	24(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 952(%rsp)
.Ltmp5422:
	.loc	43 37 12 is_stmt 1
	cmpq	$7, %rbx
	je	.LBB34_138
.Ltmp5423:
	.loc	15 1298 42
	cmpq	$7, %r10
	je	.LBB34_197
	movl	92(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5424:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rbx, %rax
	leaq	7(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	vmovss	28(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 956(%rsp)
.Ltmp5425:
	.loc	15 0 13
.Ltmp5426:
	.p2align	4
.LBB34_138:
	.loc	39 61 9 is_stmt 1
	vaddss	%xmm0, %xmm14, %xmm5
.Ltmp5427:
	.loc	39 66 9
	vsubss	%xmm1, %xmm5, %xmm14
.Ltmp5428:
	.loc	15 1306 5
	vmovss	%xmm14, 896(%rsp)
	movq	240(%rsp), %rdi
.Ltmp5429:
	.loc	37 580 12
	cmpq	%rsi, %rdi
	ja	.LBB34_425
.Ltmp5430:
	.loc	37 0 12 is_stmt 0
	movl	416(%rsp), %r15d
	movl	296(%rsp), %r8d
.Ltmp5431:
	.loc	39 56 9 is_stmt 1
	je	.LBB34_109
.Ltmp5432:
	.loc	39 0 9 is_stmt 0
	vmovaps	%xmm3, %xmm7
	vmovaps	%xmm12, %xmm3
	vmovaps	%xmm15, %xmm12
	.loc	15 1307 0 is_stmt 1
	movq	184(%r13), %rax
.Ltmp5433:
	.loc	39 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp5434:
	.loc	39 76 9
	vdivss	384(%rsp), %xmm14, %xmm0
.Ltmp5435:
	.loc	39 66 9
	vsubss	%xmm0, %xmm6, %xmm0
.Ltmp5436:
	.loc	39 66 9 is_stmt 0
	vsubss	%xmm10, %xmm0, %xmm5
.Ltmp5437:
	.loc	39 92 9 is_stmt 1
	vmulss	%xmm5, %xmm11, %xmm5
	vaddss	%xmm5, %xmm10, %xmm5
.Ltmp5438:
	.loc	39 103 24
	vbroadcastss	.LCPI34_0(%rip), %xmm15
.Ltmp5439:
	.loc	39 161 24
	vmaxss	%xmm5, %xmm0, %xmm0
.Ltmp5440:
	.loc	39 103 24
	vandps	%xmm0, %xmm15, %xmm5
.Ltmp5441:
	.loc	39 166 24
	vcmpnltss	.LCPI34_5(%rip), %xmm5, %xmm5
	vandps	%xmm0, %xmm5, %xmm10
.Ltmp5442:
	.loc	15 1312 5
	vmovss	%xmm10, 892(%rsp)
.Ltmp5443:
	.loc	15 1315 28
	movq	160(%r13), %rdx
	.loc	15 1315 44 is_stmt 0
	imulq	40(%rsp), %rbx
.Ltmp5444:
	.loc	37 568 12 is_stmt 1
	cmpq	%rdx, %rbx
	movq	504(%rsp), %r12
	movq	608(%rsp), %r14
	movl	172(%rsp), %esi
	movq	24(%rsp), %rdi
	ja	.LBB34_173
.Ltmp5445:
	.loc	39 51 9
	je	.LBB34_104
.Ltmp5446:
	.loc	15 0 0 is_stmt 0
	vsubss	%xmm10, %xmm6, %xmm0
.Ltmp5447:
	.loc	15 1315 0 is_stmt 1
	movq	152(%r13), %rax
.Ltmp5448:
	.loc	39 51 9
	vmovss	(%rax,%rbx,4), %xmm5
.Ltmp5449:
	.loc	39 56 9
	vmovss	%xmm9, (%rax,%rbx,4)
.Ltmp5450:
	.loc	39 71 9
	vmulss	%xmm5, %xmm0, %xmm0
.Ltmp5451:
	.loc	19 1244 18
	vmovd	%xmm5, %eax
.Ltmp5452:
	.loc	39 161 24
	andl	236(%rsp), %eax
.Ltmp5453:
	.loc	19 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5454:
	.loc	39 161 44
	andl	%esi, %ecx
	.loc	39 161 24 is_stmt 0
	orl	%eax, %ecx
.Ltmp5455:
	.loc	39 56 9 is_stmt 1
	movl	%ecx, (%r12,%rdi,4)
.Ltmp5456:
	.loc	37 568 12
	cmpq	904(%rsp), %rdi
	ja	.LBB34_235
.Ltmp5457:
	.loc	37 0 12 is_stmt 0
	je	.LBB34_104
	movl	168(%rsp), %eax
	andl	%r15d, %eax
	orl	%eax, %r8d
	vmovd	%r8d, %xmm5
.Ltmp5458:
	.loc	39 124 14 is_stmt 1
	vucomiss	96(%rsp), %xmm5
	vmovaps	%xmm6, %xmm0
.Ltmp5459:
	.loc	39 161 24
	jbe	.LBB34_146
.Ltmp5460:
	.loc	39 0 24 is_stmt 0
	vmovss	96(%rsp), %xmm0
.Ltmp5461:
	.loc	39 76 9 is_stmt 1
	vdivss	%xmm5, %xmm0, %xmm0
.Ltmp5462:
.LBB34_146:
	.loc	15 0 0 is_stmt 0
	movq	528(%r13), %rcx
.Ltmp5463:
	.loc	15 1287 25 is_stmt 1
	movq	376(%r13), %rsi
	.loc	15 1287 45 is_stmt 0
	movq	%rcx, %rdx
	imulq	32(%rsp), %rdx
.Ltmp5464:
	.loc	37 580 12 is_stmt 1
	cmpq	%rsi, %rdx
	ja	.LBB34_136
.Ltmp5465:
	.loc	37 0 12 is_stmt 0
	je	.LBB34_109
	vmovss	(%r14,%rdi,4), %xmm9
.Ltmp5466:
	.loc	15 1287 0 is_stmt 1
	movq	368(%r13), %rax
.Ltmp5467:
	.loc	39 56 9
	vmovss	%xmm0, (%rax,%rdx,4)
.Ltmp5468:
	.loc	15 1154 17
	movq	528(%r13), %rbx
.Ltmp5469:
	.loc	43 37 12
	testq	%rbx, %rbx
	movq	%rdx, 304(%rsp)
	je	.LBB34_163
.Ltmp5470:
	.loc	43 0 12 is_stmt 0
	movq	%rcx, 240(%rsp)
	movq	520(%r13), %rax
	movq	%rax, 56(%rsp)
	movq	32(%rsp), %rax
	leaq	1(%rax), %rdx
	cmpq	%rbp, %rdx
	movq	%rbp, %rcx
	movl	$0, %eax
	cmovbq	%rax, %rcx
	movq	512(%r13), %rax
	subq	%rcx, %rdx
	movq	368(%r13), %r12
	movq	376(%r13), %rsi
	movq	456(%r13), %rcx
	movq	%rcx, 176(%rsp)
	movq	448(%r13), %r15
	movq	424(%r13), %r9
	movq	416(%r13), %r13
	imulq	%rbx, %rdx
	movq	%rdx, 136(%rsp)
	movq	%rbx, %rdx
	xorl	%r11d, %r11d
	xorl	%ecx, %ecx
	jmp	.LBB34_150
	.p2align	4
.LBB34_210:
	xorl	%r10d, %r10d
.LBB34_161:
	decq	%rdx
	addq	$4, %rcx
.Ltmp5471:
	movl	%r10d, (%r15,%r11,4)
.Ltmp5472:
	incq	%r11
.Ltmp5473:
	.loc	43 37 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB34_162
.LBB34_150:
.Ltmp5474:
	.loc	16 1714 9
	cmpq	$32, %rcx
.Ltmp5475:
	.loc	17 180 28
	je	.LBB34_162
.Ltmp5476:
	.loc	15 1158 21
	cmpq	56(%rsp), %r11
	je	.LBB34_201
	leaq	(%r11,%r11,2), %r8
	movl	4(%rax,%r8,4), %edi
.Ltmp5477:
	.loc	15 1160 23
	addq	32(%rsp), %rdi
.Ltmp5478:
	.loc	15 1161 12
	cmpq	%rbp, %rdi
	movl	$0, %r10d
	cmovaeq	%rbp, %r10
	subq	%r10, %rdi
.Ltmp5479:
	.loc	15 1168 42
	movq	%rdi, %r14
	imulq	%rbx, %r14
	addq	%r11, %r14
	.loc	15 1168 22 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_202
.Ltmp5480:
	.loc	15 1169 24 is_stmt 1
	cmpq	176(%rsp), %r11
	je	.LBB34_203
.Ltmp5481:
	.loc	15 0 0 is_stmt 0
	movl	(%rax,%r8,4), %r8d
.Ltmp5482:
	vmovss	(%r12,%r14,4), %xmm0
.Ltmp5483:
	.loc	15 1169 24
	movl	(%r15,%r11,4), %r10d
	testq	%r10, %r10
	je	.LBB34_157
.Ltmp5484:
	.loc	15 1173 24 is_stmt 1
	cmpq	%r9, %r11
	jae	.LBB34_204
	vmovss	(%r13,%r11,4), %xmm1
.Ltmp5485:
	.loc	15 798 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB34_157
.Ltmp5486:
	.loc	15 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB34_157:
.Ltmp5487:
	.loc	15 1175 9 is_stmt 1
	cmpq	%r9, %r11
	je	.LBB34_206
	vmovss	%xmm0, (%r13,%r11,4)
	.loc	15 1176 24
	incq	%r10
	cmpq	%r8, %r10
.Ltmp5488:
	.loc	15 1177 23
	jne	.LBB34_159
	.loc	15 1177 9 is_stmt 0
	vmovss	%xmm0, 928(%rsp,%rcx)
	.loc	15 1183 30 is_stmt 1
	vmovss	(%r12,%r14,4), %xmm0
	.loc	15 0 30 is_stmt 0
.Ltmp5489:
	.p2align	4
.LBB34_208:
.Ltmp5490:
	.loc	15 1186 65 is_stmt 1
	movq	%rdi, %r14
	imulq	%rbx, %r14
	addq	%r11, %r14
	.loc	15 1186 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_424
.Ltmp5491:
	.loc	15 798 8 is_stmt 1
	vminss	(%r12,%r14,4), %xmm0, %xmm0
.Ltmp5492:
	.loc	15 1187 17
	vmovss	%xmm0, (%r12,%r14,4)
	.loc	15 1188 20
	testq	%rdi, %rdi
	cmoveq	%rbp, %rdi
	.loc	15 1191 17
	decq	%rdi
.Ltmp5493:
	.loc	10 1916 50
	decq	%r8
.Ltmp5494:
	.loc	11 900 12
	jne	.LBB34_208
	jmp	.LBB34_210
.Ltmp5495:
	.loc	11 0 12 is_stmt 0
.Ltmp5496:
	.p2align	4
.LBB34_159:
	movq	136(%rsp), %rdi
	.loc	15 1180 44 is_stmt 1
	leaq	(%r11,%rdi), %r14
	.loc	15 1180 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_423
	vmovss	(%r12,%r14,4), %xmm1
.Ltmp5497:
	.loc	15 798 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp5498:
	.loc	15 1177 9
	vmovss	%xmm0, 928(%rsp,%rcx)
	jmp	.LBB34_161
.Ltmp5499:
	.loc	15 0 9 is_stmt 0
.Ltmp5500:
	.p2align	4
.LBB34_162:
	.loc	39 51 9 is_stmt 1
	vmovss	928(%rsp), %xmm1
	movq	920(%rsp), %r13
	movq	504(%rsp), %r12
	movq	608(%rsp), %r14
	movl	416(%rsp), %r15d
	movq	24(%rsp), %rdi
	movq	240(%rsp), %rcx
	movq	304(%rsp), %rdx
.Ltmp5501:
.LBB34_163:
	.loc	39 71 9
	vmulss	.LCPI34_3(%rip), %xmm1, %xmm0
.Ltmp5502:
	.loc	19 1783 9
	vroundss	$9, %xmm0, %xmm0, %xmm0
.Ltmp5503:
	.loc	39 71 9
	vmulss	.LCPI34_4(%rip), %xmm0, %xmm0
.Ltmp5504:
	.loc	15 1307 26
	movq	392(%r13), %rsi
.Ltmp5505:
	.loc	43 37 12
	testq	%rcx, %rcx
	movq	616(%rsp), %rbx
	je	.LBB34_168
.Ltmp5506:
	.loc	43 0 12 is_stmt 0
	movq	520(%r13), %r10
.Ltmp5507:
	.loc	15 1298 42 is_stmt 1
	testq	%r10, %r10
	je	.LBB34_131
	.loc	15 0 42 is_stmt 0
	movq	512(%r13), %r11
	.loc	15 1298 42
	movl	8(%r11), %r8d
	.loc	15 1298 28
	addq	32(%rsp), %r8
.Ltmp5508:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %r8
	movl	$0, %eax
	cmovaeq	%rbp, %rax
	subq	%rax, %r8
	.loc	15 1302 40
	imulq	%rcx, %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	384(%r13), %r9
	.loc	15 1302 25
	vmovss	(%r9,%r8,4), %xmm1
	.loc	15 1302 13
	vmovss	%xmm1, 928(%rsp)
.Ltmp5509:
	.loc	43 37 12 is_stmt 1
	cmpq	$1, %rcx
	jne	.LBB34_212
.Ltmp5510:
	.loc	43 0 12 is_stmt 0
	movq	24(%rsp), %rdi
	jmp	.LBB34_168
	.p2align	4
.LBB34_212:
.Ltmp5511:
	.loc	15 1298 42 is_stmt 1
	cmpq	$1, %r10
	je	.LBB34_129
	.loc	15 0 42 is_stmt 0
	movq	%rcx, %rdx
	.loc	15 1298 42
	movl	20(%r11), %eax
	.loc	15 1298 28
	addq	32(%rsp), %rax
.Ltmp5512:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdx, %rax
	leaq	1(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	%rdx, %rcx
	.loc	15 1302 25
	vmovss	4(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 932(%rsp)
.Ltmp5513:
	.loc	43 37 12 is_stmt 1
	cmpq	$2, %rdx
	je	.LBB34_215
.Ltmp5514:
	.loc	15 1298 42
	cmpq	$2, %r10
	je	.LBB34_177
	movl	32(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5515:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdx, %rax
	leaq	2(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	%rdx, %rcx
	.loc	15 1302 25
	vmovss	8(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 936(%rsp)
.Ltmp5516:
	.loc	43 37 12 is_stmt 1
	cmpq	$3, %rdx
	je	.LBB34_215
.Ltmp5517:
	.loc	15 1298 42
	cmpq	$3, %r10
	je	.LBB34_181
	movl	44(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5518:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdx, %rax
	leaq	3(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	%rdx, %rcx
	.loc	15 1302 25
	vmovss	12(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 940(%rsp)
.Ltmp5519:
	.loc	43 37 12 is_stmt 1
	cmpq	$4, %rdx
	je	.LBB34_215
.Ltmp5520:
	.loc	15 1298 42
	cmpq	$4, %r10
	je	.LBB34_185
	movl	56(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5521:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdx, %rax
	leaq	4(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	%rdx, %rcx
	.loc	15 1302 25
	vmovss	16(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 944(%rsp)
.Ltmp5522:
	.loc	43 37 12 is_stmt 1
	cmpq	$5, %rdx
	je	.LBB34_215
.Ltmp5523:
	.loc	15 1298 42
	cmpq	$5, %r10
	je	.LBB34_189
	movl	68(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5524:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdx, %rax
	leaq	5(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	%rdx, %rcx
	.loc	15 1302 25
	vmovss	20(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 948(%rsp)
.Ltmp5525:
	.loc	43 37 12 is_stmt 1
	cmpq	$6, %rdx
	je	.LBB34_215
.Ltmp5526:
	.loc	15 1298 42
	cmpq	$6, %r10
	je	.LBB34_193
	movl	80(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5527:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdx, %rax
	leaq	6(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	%rdx, %rcx
	.loc	15 1302 25
	vmovss	24(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 952(%rsp)
.Ltmp5528:
	.loc	43 37 12 is_stmt 1
	cmpq	$7, %rdx
	je	.LBB34_215
.Ltmp5529:
	.loc	15 1298 42
	cmpq	$7, %r10
	je	.LBB34_197
	movl	92(%r11), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp5530:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdx, %rax
	leaq	7(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_234
	.loc	15 0 25
	movq	%rdx, %rcx
	.loc	15 1302 25
	vmovss	28(%r9,%rax,4), %xmm5
	.loc	15 1302 13
	vmovss	%xmm5, 956(%rsp)
.Ltmp5531:
	.loc	15 0 13
.Ltmp5532:
	.p2align	4
.LBB34_215:
	movq	24(%rsp), %rdi
	movq	304(%rsp), %rdx
.LBB34_168:
.Ltmp5533:
	.loc	39 61 9 is_stmt 1
	vaddss	%xmm0, %xmm13, %xmm5
.Ltmp5534:
	.loc	39 66 9
	vsubss	%xmm1, %xmm5, %xmm13
.Ltmp5535:
	.loc	15 1306 5
	vmovss	%xmm13, 596(%rsp)
.Ltmp5536:
	.loc	37 580 12
	cmpq	%rsi, %rdx
	ja	.LBB34_426
.Ltmp5537:
	.loc	39 56 9
	je	.LBB34_109
.Ltmp5538:
	.loc	15 1307 0
	movq	384(%r13), %rax
.Ltmp5539:
	.loc	39 56 9
	vmovss	%xmm0, (%rax,%rdx,4)
.Ltmp5540:
	.loc	39 76 9
	vdivss	376(%rsp), %xmm13, %xmm0
.Ltmp5541:
	.loc	39 66 9
	vsubss	%xmm0, %xmm6, %xmm0
.Ltmp5542:
	.loc	39 66 9 is_stmt 0
	vsubss	%xmm2, %xmm0, %xmm1
.Ltmp5543:
	.loc	39 92 9 is_stmt 1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm2, %xmm1
.Ltmp5544:
	.loc	39 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp5545:
	.loc	39 103 24
	vandps	%xmm0, %xmm15, %xmm1
.Ltmp5546:
	.loc	39 166 24
	vcmpnltss	.LCPI34_5(%rip), %xmm1, %xmm1
	vandps	%xmm0, %xmm1, %xmm2
.Ltmp5547:
	.loc	15 1312 5
	vmovss	%xmm2, 592(%rsp)
.Ltmp5548:
	.loc	15 1315 28
	movq	360(%r13), %rdx
	.loc	15 1315 44 is_stmt 0
	imulq	40(%rsp), %rcx
.Ltmp5549:
	.loc	37 568 12 is_stmt 1
	cmpq	%rdx, %rcx
	movl	172(%rsp), %esi
	ja	.LBB34_211
.Ltmp5550:
	.loc	39 51 9
	je	.LBB34_104
.Ltmp5551:
	.loc	15 0 0 is_stmt 0
	incq	%rbx
.Ltmp5552:
	vsubss	%xmm2, %xmm6, %xmm0
.Ltmp5553:
	.loc	15 1315 0 is_stmt 1
	movq	352(%r13), %rax
.Ltmp5554:
	.loc	39 51 9
	vmovss	(%rax,%rcx,4), %xmm1
.Ltmp5555:
	.loc	39 56 9
	vmovss	%xmm9, (%rax,%rcx,4)
.Ltmp5556:
	.loc	39 71 9
	vmulss	%xmm1, %xmm0, %xmm0
.Ltmp5557:
	.loc	19 1244 18
	vmovd	%xmm1, %eax
.Ltmp5558:
	.loc	39 161 24
	andl	236(%rsp), %eax
.Ltmp5559:
	.loc	19 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5560:
	.loc	39 161 44
	andl	%esi, %ecx
	.loc	39 161 24 is_stmt 0
	orl	%eax, %ecx
.Ltmp5561:
	.loc	39 56 9 is_stmt 1
	movl	%ecx, (%r14,%rdi,4)
	movq	40(%rsp), %rax
.Ltmp5562:
	.loc	15 1754 13
	incq	%rax
	.loc	15 1755 16
	cmpq	272(%rsp), %rax
	movl	$0, %edx
	cmoveq	%rdx, %rax
	movq	%rax, 40(%rsp)
	movq	32(%rsp), %rax
	.loc	15 1758 13
	incq	%rax
	.loc	15 1759 16
	cmpq	%rbp, %rax
	movl	$0, %ecx
	movq	%rcx, 120(%rsp)
	cmoveq	%rdx, %rax
	movq	%rax, 32(%rsp)
.Ltmp5563:
	.loc	10 1916 50
	cmpq	280(%rsp), %rbx
	vmovaps	%xmm12, %xmm15
	vmovaps	%xmm3, %xmm12
	vmovaps	%xmm7, %xmm3
.Ltmp5564:
	.loc	11 900 12
	jne	.LBB34_88
	jmp	.LBB34_69
.Ltmp5565:
.LBB34_236:
	.loc	15 1765 5
	vmovups	812(%rsp), %ymm0
	vmovups	844(%rsp), %ymm1
	vmovups	872(%rsp), %ymm2
	vmovups	%ymm2, 1068(%rsp)
	vmovups	%ymm1, 1040(%rsp)
	vmovups	%ymm0, 1008(%rsp)
	leaq	1008(%rsp), %rdi
	movq	968(%rsp), %rsi
	.loc	15 1765 14 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	.loc	15 1766 5 is_stmt 1
	vmovups	512(%rsp), %ymm0
	vmovups	544(%rsp), %ymm1
	vmovups	572(%rsp), %ymm2
	vmovups	%ymm2, 1068(%rsp)
	vmovups	%ymm1, 1040(%rsp)
	vmovups	%ymm0, 1008(%rsp)
	leaq	1008(%rsp), %rdi
	movq	960(%rsp), %rsi
	.loc	15 1766 15 is_stmt 0
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	40(%rsp), %rax
	.loc	15 1767 5 is_stmt 1
	movl	%eax, 560(%r13)
	movq	32(%rsp), %rax
	.loc	15 1768 5
	movl	%eax, 564(%r13)
.Ltmp5566:
	.loc	15 2141 35
	cmpb	$0, 1004(%rsp)
	je	.LBB34_345
.LBB34_398:
	.loc	15 0 35 is_stmt 0
	movq	968(%rsp), %rdi
	.loc	15 2142 26 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	15 2142 16 is_stmt 0
	testb	%al, %al
	je	.LBB34_345
	.loc	15 0 16
	movq	960(%rsp), %rdi
	.loc	15 2143 27 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	15 2143 16 is_stmt 0
	testb	%al, %al
	je	.LBB34_345
	.loc	15 0 16
	cmpq	%r12, %rbx
.Ltmp5567:
	.loc	14 1050 16 is_stmt 1
	ja	.LBB34_441
.Ltmp5568:
	.loc	14 0 16 is_stmt 0
	movq	%rbx, %rax
	movq	504(%rsp), %rcx
	.p2align	4
.LBB34_402:
.Ltmp5569:
	.loc	29 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB34_406
.Ltmp5570:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp5571:
	.loc	12 961 18
	leal	(,%rdx,4), %edi
	xorl	%esi, %esi
	xorl	%r8d, %r8d
.Ltmp5572:
	.loc	12 0 18 is_stmt 0
.Ltmp5573:
	.p2align	4
.LBB34_404:
	.loc	34 134 13 is_stmt 1
	orl	(%rcx,%r8), %esi
.Ltmp5574:
	.loc	16 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp5575:
	.loc	17 180 28
	jne	.LBB34_404
.Ltmp5576:
	.loc	35 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp5577:
	.loc	30 2054 74
	subq	%rdx, %rax
.Ltmp5578:
	.loc	34 136 12
	testl	%esi, %esi
	je	.LBB34_402
	jmp	.LBB34_345
.Ltmp5579:
.LBB34_136:
	.loc	34 0 12 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp5580:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5581:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp5582:
	.loc	37 581 13 is_stmt 1
	leaq	.Lalloc_9d2713d1692431af37d60082290049ed(%rip), %rcx
.Ltmp5583:
	.loc	37 581 13 is_stmt 0
	movq	%rdx, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5584:
.LBB34_78:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_e0592aef22128a0ac53753b9632a8183(%rip), %rcx
	movq	%rax, %rdi
.Ltmp5585:
	.loc	15 0 0 is_stmt 0
	movq	%r12, %rsi
	movq	%r12, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_421:
.Ltmp5586:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_e0592aef22128a0ac53753b9632a8183(%rip), %rcx
	movq	%rax, %rdi
.Ltmp5587:
	.loc	15 0 0 is_stmt 0
	movq	%rbp, %rsi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_245:
.Ltmp5588:
	.loc	15 1824 33 is_stmt 1
	movq	464(%r13), %rcx
	movq	472(%r13), %rax
.Ltmp5589:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp5590:
	.p2align	4
.LBB34_246:
.Ltmp5591:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5592:
	.loc	17 180 28
	je	.LBB34_249
.Ltmp5593:
	.loc	17 0 28 is_stmt 0
	xorl	%esi, %esi
.Ltmp5594:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp5595:
	.loc	17 315 25
	jne	.LBB34_254
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_246
	jmp	.LBB34_254
.Ltmp5596:
.LBB34_200:
	.loc	17 0 25
	vmovss	80(%rsp), %xmm0
.Ltmp5597:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5598:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp5599:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_f473a90cd9861be1576d1010d795a4d4(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5600:
.LBB34_425:
	.loc	37 0 13 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp5601:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5602:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp5603:
	.loc	37 581 13 is_stmt 1
	leaq	.Lalloc_8002ed69501742f3ea2ea25eb68cd581(%rip), %rcx
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5604:
.LBB34_173:
	.loc	37 0 13 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp5605:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5606:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp5607:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_a07d19b424a92543209d516ece5bdf2a(%rip), %rcx
	movq	%rbx, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5608:
.LBB34_235:
	.loc	37 0 13 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp5609:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5610:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp5611:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_59d870179e725b921b323beec09137a0(%rip), %rcx
	movq	904(%rsp), %rdx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5612:
.LBB34_426:
	.loc	37 0 13 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp5613:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5614:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp5615:
	.loc	37 581 13 is_stmt 1
	leaq	.Lalloc_8002ed69501742f3ea2ea25eb68cd581(%rip), %rcx
.Ltmp5616:
	.loc	37 581 13 is_stmt 0
	movq	%rdx, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5617:
.LBB34_211:
	.loc	37 0 13
	vmovss	80(%rsp), %xmm0
.Ltmp5618:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5619:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
	movq	%rcx, %rdi
.Ltmp5620:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_a07d19b424a92543209d516ece5bdf2a(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5621:
.LBB34_406:
	.loc	37 438 16
	cmpq	%rbp, %rbx
	ja	.LBB34_418
.Ltmp5622:
	.loc	37 0 16 is_stmt 0
	movq	608(%rsp), %rax
	.p2align	4
.LBB34_408:
.Ltmp5623:
	.loc	34 131 18 is_stmt 1
	movq	%rbx, %rcx
.Ltmp5624:
	.loc	29 1504 12
	testq	%rbx, %rbx
	je	.LBB34_412
.Ltmp5625:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp5626:
	.loc	12 961 18
	leal	(,%rdx,4), %edi
	xorl	%esi, %esi
	xorl	%r8d, %r8d
.Ltmp5627:
	.loc	12 0 18 is_stmt 0
.Ltmp5628:
	.p2align	4
.LBB34_410:
	.loc	34 134 13 is_stmt 1
	orl	(%rax,%r8), %esi
.Ltmp5629:
	.loc	16 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp5630:
	.loc	17 180 28
	jne	.LBB34_410
.Ltmp5631:
	.loc	35 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp5632:
	.loc	30 2054 74
	movq	%rcx, %rbx
	subq	%rdx, %rbx
.Ltmp5633:
	.loc	34 136 12
	testl	%esi, %esi
	je	.LBB34_408
.Ltmp5634:
.LBB34_412:
	.loc	29 1504 12
	testq	%rcx, %rcx
	sete	%al
.Ltmp5635:
	.loc	15 2141 35
	jmp	.LBB34_346
.LBB34_424:
	.loc	15 0 35 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp5636:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp5637:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp5638:
	.loc	15 1186 45 is_stmt 1
	leaq	.Lalloc_a19fa7d9077791c9eadbe74fafea8de7(%rip), %rdx
	movq	%r14, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5639:
.LBB34_249:
	.loc	15 1825 33
	movq	480(%r13), %rcx
	movq	488(%r13), %rax
.Ltmp5640:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp5641:
	.p2align	4
.LBB34_250:
.Ltmp5642:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5643:
	.loc	17 180 28
	je	.LBB34_251
.Ltmp5644:
	.loc	17 0 28 is_stmt 0
	xorl	%esi, %esi
.Ltmp5645:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp5646:
	.loc	17 315 25
	jne	.LBB34_254
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB34_250
	jmp	.LBB34_254
.Ltmp5647:
.LBB34_251:
	.loc	17 0 25
	movb	$1, %sil
.LBB34_254:
	movl	%esi, 992(%rsp)
.Ltmp5648:
	.loc	15 1828 19 is_stmt 1
	movzbl	776(%r13), %eax
	movl	%eax, 172(%rsp)
.Ltmp5649:
	.loc	15 1829 21
	movzbl	777(%r13), %eax
	movl	%eax, 236(%rsp)
.Ltmp5650:
	.loc	15 1830 16
	movq	544(%r13), %r14
.Ltmp5651:
	.loc	15 1831 16
	movq	552(%r13), %rbx
.Ltmp5652:
	.loc	15 1832 27
	movl	560(%r13), %eax
	movq	%rax, 120(%rsp)
.Ltmp5653:
	.loc	15 1833 27
	movl	564(%r13), %eax
	movq	%rax, 392(%rsp)
	leaq	2184(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
	leaq	1152(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
	leaq	512(%rsp), %rdi
.Ltmp5654:
	.loc	15 1840 32
	leaq	136(%r13), %rsi
	movq	%r14, 32(%rsp)
	movq	%r14, %rdx
	movq	%rbx, 1136(%rsp)
	movq	%rbx, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
.Ltmp5655:
	.loc	15 1841 33
	movq	544(%r13), %rdx
	movq	552(%r13), %rcx
	leaq	1008(%rsp), %rdi
	leaq	336(%r13), %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
	movq	976(%rsp), %rbx
.Ltmp5656:
	.loc	40 446 20
	testq	%rbx, %rbx
	je	.LBB34_334
.Ltmp5657:
	.loc	40 0 20 is_stmt 0
	movl	172(%rsp), %ecx
	movl	%ecx, %eax
	negl	%eax
	movl	%eax, 984(%rsp)
	movl	236(%rsp), %edx
.Ltmp5658:
	movl	%edx, %eax
	negl	%eax
	movl	%eax, 168(%rsp)
.Ltmp5659:
	.loc	14 3756 21 is_stmt 1
	movq	%rbx, %r8
	shrq	$5, %r8
.Ltmp5660:
	.loc	14 3757 21
	movl	%ebx, %eax
	andl	$31, %eax
.Ltmp5661:
	.loc	14 3758 16
	cmpq	$1, %rax
	sbbq	$-1, %r8
	decl	%ecx
	movl	%ecx, 172(%rsp)
	decl	%edx
.Ltmp5662:
	.loc	40 446 20
	movq	%r12, %rax
	negq	%rax
	movq	%rax, 1120(%rsp)
	movq	%rbp, %rax
	negq	%rax
	movq	%rax, 1112(%rsp)
	movl	$32, %r9d
	movq	608(%rsp), %r10
	movq	504(%rsp), %rax
	movq	%rax, 616(%rsp)
	movq	%rbx, %rcx
	xorl	%r15d, %r15d
	movl	%edx, 236(%rsp)
.LBB34_258:
.Ltmp5663:
	.loc	14 2584 13
	cmpq	$32, %rcx
	movl	$32, %eax
	movq	%rcx, 1128(%rsp)
	cmovbq	%rcx, %rax
	cmpq	$1, %rax
	movq	%rax, 1144(%rsp)
	adcq	$0, %rax
.Ltmp5664:
	.loc	10 1916 50
	movq	%rbx, %rcx
	subq	%r15, %rcx
.Ltmp5665:
	.loc	10 1078 5
	cmpq	$32, %rcx
	cmovaeq	%r9, %rcx
	movq	%rcx, 1104(%rsp)
.Ltmp5666:
	.loc	15 1612 23
	vmovss	628(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	632(%rsp), %xmm3
	vmovss	636(%rsp), %xmm4
	vmovss	640(%rsp), %xmm2
	vmovss	644(%rsp), %xmm10
	vmovss	648(%rsp), %xmm13
	vmovss	652(%rsp), %xmm7
	vmovss	656(%rsp), %xmm15
	vmovss	660(%rsp), %xmm6
	vmovss	664(%rsp), %xmm5
	vmovss	668(%rsp), %xmm9
.Ltmp5667:
	.loc	10 1916 50
	cmpq	%r15, %rbx
.Ltmp5668:
	.loc	11 900 12
	jne	.LBB34_260
.Ltmp5669:
	.loc	15 0 0 is_stmt 0
	vmovss	672(%rsp), %xmm14
	vmovaps	%xmm3, %xmm12
.Ltmp5670:
	.loc	11 900 12
	jmp	.LBB34_264
.Ltmp5671:
.LBB34_260:
	.loc	11 0 12
	vmovss	584(%r13), %xmm1
	vmovss	%xmm1, 416(%rsp)
	vmovss	588(%r13), %xmm1
	vmovss	%xmm1, 12(%rsp)
	vmovss	592(%r13), %xmm1
	vmovss	%xmm1, 64(%rsp)
	vmovss	596(%r13), %xmm1
	vmovss	%xmm1, 248(%rsp)
	vmovss	600(%r13), %xmm1
	vmovss	%xmm1, 52(%rsp)
	vmovss	604(%r13), %xmm1
	vmovss	%xmm1, 76(%rsp)
	movq	1120(%rsp), %rcx
.Ltmp5672:
	.loc	37 568 12 is_stmt 1
	addq	%r15, %rcx
	xorl	%edx, %edx
	vmovss	%xmm9, 176(%rsp)
	vmovss	%xmm5, 136(%rsp)
	vmovss	%xmm6, 40(%rsp)
	vmovss	%xmm15, 24(%rsp)
	vmovss	%xmm7, 80(%rsp)
	vmovaps	%xmm13, %xmm1
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm2, %xmm11
	vmovaps	%xmm4, %xmm8
	vmovss	608(%r13), %xmm2
	vmovss	%xmm2, 288(%rsp)
	vmovss	612(%r13), %xmm2
	vmovss	%xmm2, 384(%rsp)
	vmovss	616(%r13), %xmm2
	vmovss	%xmm2, 376(%rsp)
	vmovss	620(%r13), %xmm2
	vmovss	%xmm2, 280(%rsp)
	vmovss	624(%r13), %xmm2
	vmovss	%xmm2, 272(%rsp)
	vmovss	628(%r13), %xmm2
	vmovss	%xmm2, 368(%rsp)
	vmovss	632(%r13), %xmm2
	vmovss	%xmm2, 360(%rsp)
	vmovss	636(%r13), %xmm2
	vmovss	%xmm2, 132(%rsp)
	vmovss	640(%r13), %xmm2
	vmovss	%xmm2, 352(%rsp)
	vmovss	644(%r13), %xmm2
	vmovss	%xmm2, 344(%rsp)
	vmovss	648(%r13), %xmm2
	vmovss	%xmm2, 264(%rsp)
	vmovss	652(%r13), %xmm2
	vmovss	%xmm2, 336(%rsp)
	vmovss	656(%r13), %xmm2
	vmovss	%xmm2, 328(%rsp)
	vmovss	660(%r13), %xmm2
	vmovss	%xmm2, 496(%rsp)
	vmovss	664(%r13), %xmm2
	vmovss	%xmm2, 488(%rsp)
	vmovss	668(%r13), %xmm2
	vmovss	%xmm2, 480(%rsp)
	vmovss	672(%r13), %xmm2
	vmovss	%xmm2, 408(%rsp)
	vmovss	676(%r13), %xmm2
	vmovss	%xmm2, 472(%rsp)
	vmovss	680(%r13), %xmm2
	vmovss	%xmm2, 164(%rsp)
	vmovss	684(%r13), %xmm2
	vmovss	%xmm2, 256(%rsp)
	vmovss	688(%r13), %xmm2
	vmovss	%xmm2, 160(%rsp)
	vmovss	692(%r13), %xmm2
	vmovss	%xmm2, 156(%rsp)
	vmovss	696(%r13), %xmm2
	vmovss	%xmm2, 152(%rsp)
	vmovss	700(%r13), %xmm2
	vmovss	%xmm2, 148(%rsp)
	vmovss	704(%r13), %xmm2
	vmovss	%xmm2, 320(%rsp)
	vmovss	708(%r13), %xmm2
	vmovss	%xmm2, 464(%rsp)
	vmovss	712(%r13), %xmm2
	vmovss	%xmm2, 400(%rsp)
	vmovss	716(%r13), %xmm2
	vmovss	%xmm2, 456(%rsp)
	vmovss	720(%r13), %xmm2
	vmovss	%xmm2, 448(%rsp)
	vmovss	724(%r13), %xmm2
	vmovss	%xmm2, 440(%rsp)
	vmovss	728(%r13), %xmm2
	vmovss	%xmm2, 20(%rsp)
	vmovss	732(%r13), %xmm2
	vmovss	%xmm2, 16(%rsp)
	vmovss	736(%r13), %xmm2
	vmovss	%xmm2, 432(%rsp)
	vmovss	740(%r13), %xmm2
	vmovss	%xmm2, 424(%rsp)
	vmovss	744(%r13), %xmm2
	vmovss	%xmm2, 232(%rsp)
	vmovss	748(%r13), %xmm2
	vmovss	%xmm2, 228(%rsp)
	vmovss	752(%r13), %xmm2
	vmovss	%xmm2, 224(%rsp)
	vmovss	756(%r13), %xmm2
	vmovss	%xmm2, 220(%rsp)
	vmovss	760(%r13), %xmm2
	vmovss	%xmm2, 216(%rsp)
	vmovss	764(%r13), %xmm2
	vmovss	%xmm2, 212(%rsp)
	vmovss	768(%r13), %xmm2
	vmovss	%xmm2, 208(%rsp)
	vmovss	772(%r13), %xmm2
	vmovss	%xmm2, 204(%rsp)
	.loc	37 0 12 is_stmt 0
.Ltmp5673:
	.p2align	4
.LBB34_261:
	vmovss	56(%rsp), %xmm10
	vmovaps	%xmm1, %xmm7
	vmovss	80(%rsp), %xmm15
	vmovss	24(%rsp), %xmm1
	vmovss	40(%rsp), %xmm2
	vmovss	136(%rsp), %xmm4
	vmovss	176(%rsp), %xmm5
	.loc	37 568 12 is_stmt 1
	leaq	(%r15,%rdx), %rdi
	cmpq	%r12, %rdi
	ja	.LBB34_266
.Ltmp5674:
	.loc	37 0 12 is_stmt 0
	vmovss	%xmm5, 240(%rsp)
	vmovss	%xmm4, 304(%rsp)
	vmovss	%xmm2, 96(%rsp)
	.loc	15 1615 0 is_stmt 1
	movq	%rcx, %rsi
	addq	%rdx, %rsi
.Ltmp5675:
	.loc	39 51 9
	je	.LBB34_76
	.loc	39 0 9 is_stmt 0
	vmovaps	%xmm1, %xmm5
	vmovaps	%xmm0, %xmm13
	vmovaps	%xmm11, %xmm0
	vmovaps	%xmm8, %xmm2
	vmovaps	%xmm3, %xmm4
	movq	616(%rsp), %rsi
	.loc	39 51 9
	vmovss	(%rsi,%rdx,4), %xmm8
	vbroadcastss	.LCPI34_0(%rip), %xmm12
.Ltmp5676:
	.loc	39 103 24 is_stmt 1
	vandps	%xmm7, %xmm12, %xmm1
	vmovaps	%xmm1, 176(%rsp)
.Ltmp5677:
	.loc	39 71 9
	vmulss	416(%rsp), %xmm8, %xmm1
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp5678:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5679:
	.loc	39 71 9
	vmulss	12(%rsp), %xmm8, %xmm3
.Ltmp5680:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5681:
	.loc	39 71 9
	vmulss	64(%rsp), %xmm8, %xmm6
.Ltmp5682:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
	vmovss	%xmm8, 56(%rsp)
.Ltmp5683:
	.loc	39 71 9
	vmulss	248(%rsp), %xmm8, %xmm8
.Ltmp5684:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovss	%xmm10, 296(%rsp)
.Ltmp5685:
	.loc	39 71 9
	vmulss	52(%rsp), %xmm10, %xmm11
.Ltmp5686:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5687:
	.loc	39 71 9
	vmulss	76(%rsp), %xmm10, %xmm11
.Ltmp5688:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5689:
	.loc	39 71 9
	vmulss	288(%rsp), %xmm10, %xmm11
.Ltmp5690:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5691:
	.loc	39 71 9
	vmulss	384(%rsp), %xmm10, %xmm11
.Ltmp5692:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5693:
	.loc	39 71 9
	vmulss	376(%rsp), %xmm4, %xmm11
.Ltmp5694:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5695:
	.loc	39 71 9
	vmulss	280(%rsp), %xmm4, %xmm11
.Ltmp5696:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5697:
	.loc	39 71 9
	vmulss	272(%rsp), %xmm4, %xmm11
.Ltmp5698:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5699:
	.loc	39 71 9
	vmulss	368(%rsp), %xmm4, %xmm11
.Ltmp5700:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5701:
	.loc	39 71 9
	vmulss	360(%rsp), %xmm2, %xmm11
.Ltmp5702:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5703:
	.loc	39 71 9
	vmulss	132(%rsp), %xmm2, %xmm11
.Ltmp5704:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5705:
	.loc	39 71 9
	vmulss	352(%rsp), %xmm2, %xmm11
.Ltmp5706:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5707:
	.loc	39 71 9
	vmulss	344(%rsp), %xmm2, %xmm11
.Ltmp5708:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovaps	%xmm0, %xmm10
.Ltmp5709:
	.loc	39 71 9
	vmulss	264(%rsp), %xmm0, %xmm11
.Ltmp5710:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5711:
	.loc	39 71 9
	vmulss	336(%rsp), %xmm0, %xmm11
.Ltmp5712:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5713:
	.loc	39 71 9
	vmulss	328(%rsp), %xmm0, %xmm11
.Ltmp5714:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5715:
	.loc	39 71 9
	vmulss	496(%rsp), %xmm0, %xmm11
.Ltmp5716:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5717:
	.loc	39 71 9
	vmulss	488(%rsp), %xmm13, %xmm11
.Ltmp5718:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5719:
	.loc	39 71 9
	vmulss	480(%rsp), %xmm13, %xmm11
.Ltmp5720:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5721:
	.loc	39 71 9
	vmulss	408(%rsp), %xmm13, %xmm11
.Ltmp5722:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5723:
	.loc	39 71 9
	vmulss	472(%rsp), %xmm13, %xmm11
.Ltmp5724:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5725:
	.loc	39 71 9
	vmulss	164(%rsp), %xmm7, %xmm11
.Ltmp5726:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5727:
	.loc	39 71 9
	vmulss	256(%rsp), %xmm7, %xmm11
.Ltmp5728:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5729:
	.loc	39 71 9
	vmulss	160(%rsp), %xmm7, %xmm11
.Ltmp5730:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5731:
	.loc	39 71 9
	vmulss	156(%rsp), %xmm7, %xmm11
.Ltmp5732:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5733:
	.loc	39 71 9
	vmulss	152(%rsp), %xmm15, %xmm11
.Ltmp5734:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5735:
	.loc	39 71 9
	vmulss	148(%rsp), %xmm15, %xmm11
.Ltmp5736:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5737:
	.loc	39 71 9
	vmulss	320(%rsp), %xmm15, %xmm11
.Ltmp5738:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm0
.Ltmp5739:
	.loc	39 71 9
	vmulss	464(%rsp), %xmm15, %xmm11
.Ltmp5740:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovaps	%xmm5, %xmm6
.Ltmp5741:
	.loc	39 71 9
	vmulss	400(%rsp), %xmm5, %xmm11
.Ltmp5742:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5743:
	.loc	39 71 9
	vmulss	456(%rsp), %xmm5, %xmm11
.Ltmp5744:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5745:
	.loc	39 71 9
	vmulss	448(%rsp), %xmm5, %xmm11
.Ltmp5746:
	.loc	39 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp5747:
	.loc	39 71 9
	vmulss	440(%rsp), %xmm5, %xmm11
.Ltmp5748:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovss	96(%rsp), %xmm5
.Ltmp5749:
	.loc	39 71 9
	vmulss	20(%rsp), %xmm5, %xmm11
.Ltmp5750:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5751:
	.loc	39 71 9
	vmulss	16(%rsp), %xmm5, %xmm11
.Ltmp5752:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5753:
	.loc	39 71 9
	vmulss	432(%rsp), %xmm5, %xmm11
.Ltmp5754:
	.loc	39 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp5755:
	.loc	39 71 9
	vmulss	424(%rsp), %xmm5, %xmm11
.Ltmp5756:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovss	304(%rsp), %xmm9
.Ltmp5757:
	.loc	39 71 9
	vmulss	232(%rsp), %xmm9, %xmm11
.Ltmp5758:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5759:
	.loc	39 71 9
	vmulss	228(%rsp), %xmm9, %xmm11
.Ltmp5760:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5761:
	.loc	39 71 9
	vmulss	224(%rsp), %xmm9, %xmm11
.Ltmp5762:
	.loc	39 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp5763:
	.loc	39 71 9
	vmulss	220(%rsp), %xmm9, %xmm11
.Ltmp5764:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovss	240(%rsp), %xmm14
.Ltmp5765:
	.loc	39 71 9
	vmulss	216(%rsp), %xmm14, %xmm11
.Ltmp5766:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5767:
	.loc	39 71 9
	vmulss	212(%rsp), %xmm14, %xmm11
.Ltmp5768:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5769:
	.loc	39 71 9
	vmulss	208(%rsp), %xmm14, %xmm11
.Ltmp5770:
	.loc	39 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp5771:
	.loc	39 71 9
	vmulss	204(%rsp), %xmm14, %xmm11
.Ltmp5772:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovaps	%xmm12, %xmm11
.Ltmp5773:
	.loc	39 103 24
	vandps	%xmm1, %xmm12, %xmm1
	vmovaps	176(%rsp), %xmm12
.Ltmp5774:
	.loc	39 161 24
	vmaxss	%xmm1, %xmm12, %xmm1
	vmovss	296(%rsp), %xmm12
.Ltmp5775:
	.loc	39 103 24
	vandps	%xmm3, %xmm11, %xmm3
.Ltmp5776:
	.loc	39 161 24
	vmaxss	%xmm3, %xmm1, %xmm1
.Ltmp5777:
	.loc	39 103 24
	vandps	%xmm0, %xmm11, %xmm3
.Ltmp5778:
	.loc	39 161 24
	vmaxss	%xmm3, %xmm1, %xmm1
.Ltmp5779:
	.loc	39 103 24
	vandps	%xmm11, %xmm8, %xmm3
.Ltmp5780:
	.loc	39 161 24
	vmaxss	%xmm3, %xmm1, %xmm1
.Ltmp5781:
	.loc	39 56 9
	vmovss	%xmm1, 2184(%rsp,%rdx,4)
.Ltmp5782:
	.loc	15 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm9, 176(%rsp)
	vmovss	%xmm5, 136(%rsp)
	vmovss	%xmm6, 40(%rsp)
	vmovss	%xmm15, 24(%rsp)
	vmovss	%xmm7, 80(%rsp)
	vmovaps	%xmm13, %xmm1
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm2, %xmm11
	vmovaps	%xmm4, %xmm8
	vmovaps	%xmm12, %xmm3
.Ltmp5783:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp5784:
	.loc	11 900 12
	jne	.LBB34_261
.Ltmp5785:
.LBB34_264:
	.loc	11 0 12 is_stmt 0
	vmovss	56(%rsp), %xmm0
	.loc	15 1618 5 is_stmt 1
	vmovss	%xmm0, 628(%rsp)
	vmovss	%xmm12, 632(%rsp)
	vmovss	%xmm4, 636(%rsp)
	vmovss	%xmm2, 640(%rsp)
	vmovss	%xmm10, 644(%rsp)
	vmovss	%xmm13, 648(%rsp)
	vmovss	%xmm7, 652(%rsp)
	vmovss	%xmm15, 656(%rsp)
	vmovss	%xmm6, 660(%rsp)
	vmovss	%xmm5, 664(%rsp)
	vmovss	%xmm9, 668(%rsp)
	vmovss	%xmm14, 672(%rsp)
.Ltmp5786:
	.loc	15 1612 23
	vmovss	720(%rsp), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	724(%rsp), %xmm3
	vmovss	728(%rsp), %xmm4
	vmovss	732(%rsp), %xmm2
	vmovss	736(%rsp), %xmm10
	vmovss	740(%rsp), %xmm13
	vmovss	744(%rsp), %xmm7
	vmovss	748(%rsp), %xmm15
	vmovss	752(%rsp), %xmm6
	vmovss	756(%rsp), %xmm5
	vmovss	760(%rsp), %xmm9
.Ltmp5787:
	.loc	10 1916 50
	cmpq	%r15, %rbx
.Ltmp5788:
	.loc	11 900 12
	jne	.LBB34_268
.Ltmp5789:
	.loc	15 0 0 is_stmt 0
	vmovss	764(%rsp), %xmm14
	vmovaps	%xmm3, %xmm12
.Ltmp5790:
	.loc	11 900 12
	jmp	.LBB34_272
.Ltmp5791:
.LBB34_268:
	.loc	11 0 12
	vmovss	584(%r13), %xmm1
	vmovss	%xmm1, 416(%rsp)
	vmovss	588(%r13), %xmm1
	vmovss	%xmm1, 12(%rsp)
	vmovss	592(%r13), %xmm1
	vmovss	%xmm1, 64(%rsp)
	vmovss	596(%r13), %xmm1
	vmovss	%xmm1, 248(%rsp)
	vmovss	600(%r13), %xmm1
	vmovss	%xmm1, 52(%rsp)
	vmovss	604(%r13), %xmm1
	vmovss	%xmm1, 76(%rsp)
	movq	1112(%rsp), %rcx
.Ltmp5792:
	.loc	37 568 12 is_stmt 1
	addq	%r15, %rcx
	xorl	%edx, %edx
	vmovss	%xmm9, 176(%rsp)
	vmovss	%xmm5, 136(%rsp)
	vmovss	%xmm6, 40(%rsp)
	vmovss	%xmm15, 24(%rsp)
	vmovss	%xmm7, 80(%rsp)
	vmovaps	%xmm13, %xmm1
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm2, %xmm11
	vmovaps	%xmm4, %xmm8
	vmovss	608(%r13), %xmm2
	vmovss	%xmm2, 288(%rsp)
	vmovss	612(%r13), %xmm2
	vmovss	%xmm2, 384(%rsp)
	vmovss	616(%r13), %xmm2
	vmovss	%xmm2, 376(%rsp)
	vmovss	620(%r13), %xmm2
	vmovss	%xmm2, 280(%rsp)
	vmovss	624(%r13), %xmm2
	vmovss	%xmm2, 272(%rsp)
	vmovss	628(%r13), %xmm2
	vmovss	%xmm2, 368(%rsp)
	vmovss	632(%r13), %xmm2
	vmovss	%xmm2, 360(%rsp)
	vmovss	636(%r13), %xmm2
	vmovss	%xmm2, 132(%rsp)
	vmovss	640(%r13), %xmm2
	vmovss	%xmm2, 352(%rsp)
	vmovss	644(%r13), %xmm2
	vmovss	%xmm2, 344(%rsp)
	vmovss	648(%r13), %xmm2
	vmovss	%xmm2, 264(%rsp)
	vmovss	652(%r13), %xmm2
	vmovss	%xmm2, 336(%rsp)
	vmovss	656(%r13), %xmm2
	vmovss	%xmm2, 328(%rsp)
	vmovss	660(%r13), %xmm2
	vmovss	%xmm2, 496(%rsp)
	vmovss	664(%r13), %xmm2
	vmovss	%xmm2, 488(%rsp)
	vmovss	668(%r13), %xmm2
	vmovss	%xmm2, 480(%rsp)
	vmovss	672(%r13), %xmm2
	vmovss	%xmm2, 408(%rsp)
	vmovss	676(%r13), %xmm2
	vmovss	%xmm2, 472(%rsp)
	vmovss	680(%r13), %xmm2
	vmovss	%xmm2, 164(%rsp)
	vmovss	684(%r13), %xmm2
	vmovss	%xmm2, 256(%rsp)
	vmovss	688(%r13), %xmm2
	vmovss	%xmm2, 160(%rsp)
	vmovss	692(%r13), %xmm2
	vmovss	%xmm2, 156(%rsp)
	vmovss	696(%r13), %xmm2
	vmovss	%xmm2, 152(%rsp)
	vmovss	700(%r13), %xmm2
	vmovss	%xmm2, 148(%rsp)
	vmovss	704(%r13), %xmm2
	vmovss	%xmm2, 320(%rsp)
	vmovss	708(%r13), %xmm2
	vmovss	%xmm2, 464(%rsp)
	vmovss	712(%r13), %xmm2
	vmovss	%xmm2, 400(%rsp)
	vmovss	716(%r13), %xmm2
	vmovss	%xmm2, 456(%rsp)
	vmovss	720(%r13), %xmm2
	vmovss	%xmm2, 448(%rsp)
	vmovss	724(%r13), %xmm2
	vmovss	%xmm2, 440(%rsp)
	vmovss	728(%r13), %xmm2
	vmovss	%xmm2, 20(%rsp)
	vmovss	732(%r13), %xmm2
	vmovss	%xmm2, 16(%rsp)
	vmovss	736(%r13), %xmm2
	vmovss	%xmm2, 432(%rsp)
	vmovss	740(%r13), %xmm2
	vmovss	%xmm2, 424(%rsp)
	vmovss	744(%r13), %xmm2
	vmovss	%xmm2, 232(%rsp)
	vmovss	748(%r13), %xmm2
	vmovss	%xmm2, 228(%rsp)
	vmovss	752(%r13), %xmm2
	vmovss	%xmm2, 224(%rsp)
	vmovss	756(%r13), %xmm2
	vmovss	%xmm2, 220(%rsp)
	vmovss	760(%r13), %xmm2
	vmovss	%xmm2, 216(%rsp)
	vmovss	764(%r13), %xmm2
	vmovss	%xmm2, 212(%rsp)
	vmovss	768(%r13), %xmm2
	vmovss	%xmm2, 208(%rsp)
	vmovss	772(%r13), %xmm2
	vmovss	%xmm2, 204(%rsp)
	.loc	37 0 12 is_stmt 0
.Ltmp5793:
	.p2align	4
.LBB34_269:
	vmovss	56(%rsp), %xmm10
	vmovaps	%xmm1, %xmm7
	vmovss	80(%rsp), %xmm15
	vmovss	24(%rsp), %xmm1
	vmovss	40(%rsp), %xmm2
	vmovss	136(%rsp), %xmm4
	vmovss	176(%rsp), %xmm5
	.loc	37 568 12 is_stmt 1
	leaq	(%r15,%rdx), %rdi
	cmpq	%rbp, %rdi
	ja	.LBB34_277
.Ltmp5794:
	.loc	37 0 12 is_stmt 0
	vmovss	%xmm5, 240(%rsp)
	vmovss	%xmm4, 304(%rsp)
	vmovss	%xmm2, 96(%rsp)
	.loc	15 1615 0 is_stmt 1
	movq	%rcx, %rsi
	addq	%rdx, %rsi
.Ltmp5795:
	.loc	39 51 9
	je	.LBB34_76
	.loc	39 0 9 is_stmt 0
	vmovaps	%xmm1, %xmm5
	vmovaps	%xmm0, %xmm13
	vmovaps	%xmm11, %xmm0
	vmovaps	%xmm8, %xmm2
	vmovaps	%xmm3, %xmm4
	.loc	39 51 9
	vmovss	(%r10,%rdx,4), %xmm8
	vbroadcastss	.LCPI34_0(%rip), %xmm12
.Ltmp5796:
	.loc	39 103 24 is_stmt 1
	vandps	%xmm7, %xmm12, %xmm1
	vmovaps	%xmm1, 176(%rsp)
.Ltmp5797:
	.loc	39 71 9
	vmulss	416(%rsp), %xmm8, %xmm1
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp5798:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5799:
	.loc	39 71 9
	vmulss	12(%rsp), %xmm8, %xmm3
.Ltmp5800:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5801:
	.loc	39 71 9
	vmulss	64(%rsp), %xmm8, %xmm6
.Ltmp5802:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
	vmovss	%xmm8, 56(%rsp)
.Ltmp5803:
	.loc	39 71 9
	vmulss	248(%rsp), %xmm8, %xmm8
.Ltmp5804:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovss	%xmm10, 296(%rsp)
.Ltmp5805:
	.loc	39 71 9
	vmulss	52(%rsp), %xmm10, %xmm11
.Ltmp5806:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5807:
	.loc	39 71 9
	vmulss	76(%rsp), %xmm10, %xmm11
.Ltmp5808:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5809:
	.loc	39 71 9
	vmulss	288(%rsp), %xmm10, %xmm11
.Ltmp5810:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5811:
	.loc	39 71 9
	vmulss	384(%rsp), %xmm10, %xmm11
.Ltmp5812:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5813:
	.loc	39 71 9
	vmulss	376(%rsp), %xmm4, %xmm11
.Ltmp5814:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5815:
	.loc	39 71 9
	vmulss	280(%rsp), %xmm4, %xmm11
.Ltmp5816:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5817:
	.loc	39 71 9
	vmulss	272(%rsp), %xmm4, %xmm11
.Ltmp5818:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5819:
	.loc	39 71 9
	vmulss	368(%rsp), %xmm4, %xmm11
.Ltmp5820:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5821:
	.loc	39 71 9
	vmulss	360(%rsp), %xmm2, %xmm11
.Ltmp5822:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5823:
	.loc	39 71 9
	vmulss	132(%rsp), %xmm2, %xmm11
.Ltmp5824:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5825:
	.loc	39 71 9
	vmulss	352(%rsp), %xmm2, %xmm11
.Ltmp5826:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5827:
	.loc	39 71 9
	vmulss	344(%rsp), %xmm2, %xmm11
.Ltmp5828:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovaps	%xmm0, %xmm10
.Ltmp5829:
	.loc	39 71 9
	vmulss	264(%rsp), %xmm0, %xmm11
.Ltmp5830:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5831:
	.loc	39 71 9
	vmulss	336(%rsp), %xmm0, %xmm11
.Ltmp5832:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5833:
	.loc	39 71 9
	vmulss	328(%rsp), %xmm0, %xmm11
.Ltmp5834:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5835:
	.loc	39 71 9
	vmulss	496(%rsp), %xmm0, %xmm11
.Ltmp5836:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5837:
	.loc	39 71 9
	vmulss	488(%rsp), %xmm13, %xmm11
.Ltmp5838:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5839:
	.loc	39 71 9
	vmulss	480(%rsp), %xmm13, %xmm11
.Ltmp5840:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5841:
	.loc	39 71 9
	vmulss	408(%rsp), %xmm13, %xmm11
.Ltmp5842:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5843:
	.loc	39 71 9
	vmulss	472(%rsp), %xmm13, %xmm11
.Ltmp5844:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5845:
	.loc	39 71 9
	vmulss	164(%rsp), %xmm7, %xmm11
.Ltmp5846:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5847:
	.loc	39 71 9
	vmulss	256(%rsp), %xmm7, %xmm11
.Ltmp5848:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5849:
	.loc	39 71 9
	vmulss	160(%rsp), %xmm7, %xmm11
.Ltmp5850:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp5851:
	.loc	39 71 9
	vmulss	156(%rsp), %xmm7, %xmm11
.Ltmp5852:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp5853:
	.loc	39 71 9
	vmulss	152(%rsp), %xmm15, %xmm11
.Ltmp5854:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5855:
	.loc	39 71 9
	vmulss	148(%rsp), %xmm15, %xmm11
.Ltmp5856:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5857:
	.loc	39 71 9
	vmulss	320(%rsp), %xmm15, %xmm11
.Ltmp5858:
	.loc	39 61 9
	vaddss	%xmm6, %xmm11, %xmm0
.Ltmp5859:
	.loc	39 71 9
	vmulss	464(%rsp), %xmm15, %xmm11
.Ltmp5860:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovaps	%xmm5, %xmm6
.Ltmp5861:
	.loc	39 71 9
	vmulss	400(%rsp), %xmm5, %xmm11
.Ltmp5862:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5863:
	.loc	39 71 9
	vmulss	456(%rsp), %xmm5, %xmm11
.Ltmp5864:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5865:
	.loc	39 71 9
	vmulss	448(%rsp), %xmm5, %xmm11
.Ltmp5866:
	.loc	39 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp5867:
	.loc	39 71 9
	vmulss	440(%rsp), %xmm5, %xmm11
.Ltmp5868:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovss	96(%rsp), %xmm5
.Ltmp5869:
	.loc	39 71 9
	vmulss	20(%rsp), %xmm5, %xmm11
.Ltmp5870:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5871:
	.loc	39 71 9
	vmulss	16(%rsp), %xmm5, %xmm11
.Ltmp5872:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5873:
	.loc	39 71 9
	vmulss	432(%rsp), %xmm5, %xmm11
.Ltmp5874:
	.loc	39 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp5875:
	.loc	39 71 9
	vmulss	424(%rsp), %xmm5, %xmm11
.Ltmp5876:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovss	304(%rsp), %xmm9
.Ltmp5877:
	.loc	39 71 9
	vmulss	232(%rsp), %xmm9, %xmm11
.Ltmp5878:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5879:
	.loc	39 71 9
	vmulss	228(%rsp), %xmm9, %xmm11
.Ltmp5880:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5881:
	.loc	39 71 9
	vmulss	224(%rsp), %xmm9, %xmm11
.Ltmp5882:
	.loc	39 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp5883:
	.loc	39 71 9
	vmulss	220(%rsp), %xmm9, %xmm11
.Ltmp5884:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovss	240(%rsp), %xmm14
.Ltmp5885:
	.loc	39 71 9
	vmulss	216(%rsp), %xmm14, %xmm11
.Ltmp5886:
	.loc	39 61 9
	vaddss	%xmm1, %xmm11, %xmm1
.Ltmp5887:
	.loc	39 71 9
	vmulss	212(%rsp), %xmm14, %xmm11
.Ltmp5888:
	.loc	39 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5889:
	.loc	39 71 9
	vmulss	208(%rsp), %xmm14, %xmm11
.Ltmp5890:
	.loc	39 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp5891:
	.loc	39 71 9
	vmulss	204(%rsp), %xmm14, %xmm11
.Ltmp5892:
	.loc	39 61 9
	vaddss	%xmm11, %xmm8, %xmm8
	vmovaps	%xmm12, %xmm11
.Ltmp5893:
	.loc	39 103 24
	vandps	%xmm1, %xmm12, %xmm1
	vmovaps	176(%rsp), %xmm12
.Ltmp5894:
	.loc	39 161 24
	vmaxss	%xmm1, %xmm12, %xmm1
	vmovss	296(%rsp), %xmm12
.Ltmp5895:
	.loc	39 103 24
	vandps	%xmm3, %xmm11, %xmm3
.Ltmp5896:
	.loc	39 161 24
	vmaxss	%xmm3, %xmm1, %xmm1
.Ltmp5897:
	.loc	39 103 24
	vandps	%xmm0, %xmm11, %xmm3
.Ltmp5898:
	.loc	39 161 24
	vmaxss	%xmm3, %xmm1, %xmm1
.Ltmp5899:
	.loc	39 103 24
	vandps	%xmm11, %xmm8, %xmm3
.Ltmp5900:
	.loc	39 161 24
	vmaxss	%xmm3, %xmm1, %xmm1
.Ltmp5901:
	.loc	39 56 9
	vmovss	%xmm1, 1152(%rsp,%rdx,4)
.Ltmp5902:
	.loc	15 0 0 is_stmt 0
	leaq	1(%rdx), %rdx
	vmovss	%xmm9, 176(%rsp)
	vmovss	%xmm5, 136(%rsp)
	vmovss	%xmm6, 40(%rsp)
	vmovss	%xmm15, 24(%rsp)
	vmovss	%xmm7, 80(%rsp)
	vmovaps	%xmm13, %xmm1
	vmovaps	%xmm10, %xmm0
	vmovaps	%xmm2, %xmm11
	vmovaps	%xmm4, %xmm8
	vmovaps	%xmm12, %xmm3
.Ltmp5903:
	.loc	10 1916 50 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp5904:
	.loc	11 900 12
	jne	.LBB34_269
.Ltmp5905:
.LBB34_272:
	.loc	11 0 12 is_stmt 0
	vmovss	56(%rsp), %xmm0
	.loc	15 1618 5 is_stmt 1
	vmovss	%xmm0, 720(%rsp)
	vmovss	%xmm12, 724(%rsp)
	vmovss	%xmm4, 728(%rsp)
	vmovss	%xmm2, 732(%rsp)
	vmovss	%xmm10, 736(%rsp)
	vmovss	%xmm13, 740(%rsp)
	vmovss	%xmm7, 744(%rsp)
	vmovss	%xmm15, 748(%rsp)
	vmovss	%xmm6, 752(%rsp)
	vmovss	%xmm5, 756(%rsp)
	vmovss	%xmm9, 760(%rsp)
	vmovss	%xmm14, 764(%rsp)
.Ltmp5906:
	.loc	10 1916 50
	cmpq	%r15, %rbx
	jne	.LBB34_273
.Ltmp5907:
.LBB34_257:
	.loc	15 0 0 is_stmt 0
	addq	$32, %r15
	decq	%r8
	movq	1128(%rsp), %rcx
.Ltmp5908:
	.loc	40 446 20 is_stmt 1
	addq	$-32, %rcx
	subq	$-128, 616(%rsp)
	subq	$-128, %r10
	testq	%r8, %r8
	jne	.LBB34_258
	jmp	.LBB34_334
.Ltmp5909:
.LBB34_273:
	.loc	40 0 20 is_stmt 0
	movq	%r15, 400(%rsp)
	movq	%r10, 424(%rsp)
	movq	%r8, 432(%rsp)
	movq	568(%rsp), %rdi
	movq	576(%rsp), %rbx
	movq	1064(%rsp), %r15
	movq	1072(%rsp), %rsi
	movq	512(%rsp), %r13
	movq	520(%rsp), %rdx
	movq	560(%rsp), %rcx
	movq	528(%rsp), %rax
	movq	%rax, 344(%rsp)
	movq	536(%rsp), %rax
	movq	%rax, 272(%rsp)
	movq	552(%rsp), %rax
	movq	%rax, 264(%rsp)
	movq	544(%rsp), %rax
	movq	%rax, 336(%rsp)
	movq	1008(%rsp), %rbp
	movq	1056(%rsp), %rax
	vmovss	584(%rsp), %xmm2
	vmovss	1080(%rsp), %xmm0
.Ltmp5910:
	.loc	15 1865 19 is_stmt 1
	cmpq	$1, %rcx
	movq	%rcx, 368(%rsp)
	adcq	$0, %rcx
	movq	%rcx, 360(%rsp)
	cmpq	$1, %rax
	movq	%rax, 496(%rsp)
	adcq	$0, %rax
	movq	%rax, 488(%rsp)
	vmovaps	%xmm0, %xmm10
	vmovaps	%xmm2, %xmm8
	movq	1016(%rsp), %rax
	movq	1024(%rsp), %rcx
	movq	%rcx, 480(%rsp)
	movq	1032(%rsp), %rcx
	movq	%rcx, 328(%rsp)
	movq	1048(%rsp), %rcx
	movq	%rcx, 408(%rsp)
	movq	1040(%rsp), %rcx
	movq	%rcx, 472(%rsp)
	movl	588(%rsp), %ecx
	movq	%rcx, 56(%rsp)
	movl	1084(%rsp), %ecx
	movq	%rcx, 320(%rsp)
	xorl	%ecx, %ecx
	movq	120(%rsp), %r8
	xorl	%r10d, %r10d
	vmovss	.LCPI34_2(%rip), %xmm12
	movq	1104(%rsp), %r12
	movq	392(%rsp), %r11
	movq	%rdi, 448(%rsp)
	movq	%rbx, 440(%rsp)
	movq	%r15, 280(%rsp)
	movq	%rsi, 456(%rsp)
.LBB34_274:
	.loc	15 0 19 is_stmt 0
	movq	%rcx, 256(%rsp)
	.loc	15 1870 21 is_stmt 1
	subq	%rcx, %r12
	movq	920(%rsp), %rcx
.Ltmp5911:
	.loc	15 1470 16
	movq	544(%rcx), %r9
.Ltmp5912:
	.loc	15 1471 16
	movq	552(%rcx), %r14
.Ltmp5913:
	.loc	15 1472 25
	leaq	1(%r11), %rsi
.Ltmp5914:
	.loc	15 1043 8
	cmpq	%r9, %rsi
	movq	%r9, %rsi
	cmovbq	%r10, %rsi
	negq	%rsi
.Ltmp5915:
	.loc	15 1473 28
	leaq	(%rdi,%r11), %rcx
.Ltmp5916:
	.loc	15 1043 8
	cmpq	%r9, %rcx
	movq	%r9, %rdi
	cmovbq	%r10, %rdi
	subq	%rdi, %rcx
	movq	%rcx, 240(%rsp)
.Ltmp5917:
	.loc	15 1474 29
	leaq	(%r15,%r11), %rcx
.Ltmp5918:
	.loc	15 1043 8
	cmpq	%r9, %rcx
	movq	%r9, %rdi
	cmovbq	%r10, %rdi
	subq	%rdi, %rcx
	movq	%rcx, 416(%rsp)
.Ltmp5919:
	.loc	15 1475 33
	leaq	(%rbx,%r11), %rcx
.Ltmp5920:
	.loc	15 1043 8
	cmpq	%r9, %rcx
	movq	%r9, %rdi
	cmovbq	%r10, %rdi
	subq	%rdi, %rcx
	movq	%rcx, 296(%rsp)
	movq	456(%rsp), %rcx
.Ltmp5921:
	.loc	15 1476 34
	addq	%r11, %rcx
.Ltmp5922:
	.loc	15 1043 8
	cmpq	%r9, %rcx
	movq	%r9, %rdi
	cmovbq	%r10, %rdi
.Ltmp5923:
	.loc	15 1043 8 is_stmt 0
	addq	%r11, %rsi
	incq	%rsi
.Ltmp5924:
	.loc	15 1043 8
	subq	%rdi, %rcx
	movq	%rcx, %rdi
.Ltmp5925:
	.loc	15 1478 14 is_stmt 1
	movq	%r9, %r10
	movq	%r11, 392(%rsp)
	subq	%r11, %r10
.Ltmp5926:
	.loc	10 1078 5
	cmpq	%r12, %r10
	cmovbq	%r10, %r12
	movq	%r8, 120(%rsp)
.Ltmp5927:
	.loc	15 1479 14
	subq	%r8, %r14
.Ltmp5928:
	.loc	10 1078 5
	cmpq	%r12, %r14
	cmovbq	%r14, %r12
.Ltmp5929:
	.loc	15 1480 14
	movq	%r9, %rbx
	movq	%rsi, 248(%rsp)
	subq	%rsi, %rbx
.Ltmp5930:
	.loc	10 1078 5
	cmpq	%r12, %rbx
	cmovbq	%rbx, %r12
.Ltmp5931:
	.loc	15 1481 14
	movq	%r9, %r15
	movq	240(%rsp), %rcx
	subq	%rcx, %r15
.Ltmp5932:
	.loc	10 1078 5
	cmpq	%r12, %r15
	cmovbq	%r15, %r12
.Ltmp5933:
	.loc	15 1482 14
	movq	%r9, %r11
	movq	416(%rsp), %rcx
	subq	%rcx, %r11
.Ltmp5934:
	.loc	10 1078 5
	cmpq	%r12, %r11
	cmovbq	%r11, %r12
.Ltmp5935:
	.loc	15 1483 14
	movq	%r9, %r8
	movq	296(%rsp), %rcx
	subq	%rcx, %r8
.Ltmp5936:
	.loc	10 1078 5
	cmpq	%r12, %r8
	cmovbq	%r8, %r12
	movq	%rdi, 288(%rsp)
.Ltmp5937:
	.loc	15 1484 14
	subq	%rdi, %r9
.Ltmp5938:
	.loc	10 1078 5
	cmpq	%r12, %r9
	cmovbq	%r9, %r12
	movq	400(%rsp), %rcx
	movq	256(%rsp), %rsi
.Ltmp5939:
	.loc	15 1876 28
	leaq	(%rsi,%rcx), %rdi
.Ltmp5940:
	.loc	15 1878 55
	movq	%r12, %rsi
	addq	%rdi, %rsi
.Ltmp5941:
	.loc	14 1050 16
	jb	.LBB34_276
	cmpq	912(%rsp), %rsi
	ja	.LBB34_276
.Ltmp5942:
	.loc	37 451 16
	cmpq	904(%rsp), %rsi
	ja	.LBB34_427
.Ltmp5943:
	.loc	37 0 16 is_stmt 0
	movq	%r12, 464(%rsp)
.Ltmp5944:
	.loc	32 304 12 is_stmt 1
	testq	%r12, %r12
	movq	320(%rsp), %r12
	je	.LBB34_281
	.loc	32 0 12 is_stmt 0
	vmovss	%xmm2, 16(%rsp)
	vmovss	%xmm0, 20(%rsp)
	movq	504(%rsp), %rcx
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 176(%rsp)
	movq	608(%rsp), %rcx
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 136(%rsp)
	movq	256(%rsp), %rcx
	leaq	2184(%rsp,%rcx,4), %rsi
	movq	%rsi, 384(%rsp)
	leaq	1152(%rsp,%rcx,4), %rsi
	movq	%rsi, 376(%rsp)
	vmovss	676(%rsp), %xmm11
	vmovss	680(%rsp), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	696(%rsp), %xmm0
	vmovss	%xmm0, 156(%rsp)
	vmovss	768(%rsp), %xmm9
	vmovss	772(%rsp), %xmm0
	vmovss	%xmm0, 152(%rsp)
	vmovss	788(%rsp), %xmm0
	vmovss	%xmm0, 148(%rsp)
	vmovss	716(%rsp), %xmm2
	vmovss	808(%rsp), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	688(%rsp), %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	684(%rsp), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovss	704(%rsp), %xmm0
	vmovss	%xmm0, 64(%rsp)
	.loc	32 304 12
	cmpq	%r11, %r15
	cmovbq	%r15, %r11
	vmovss	692(%rsp), %xmm0
	vmovss	%xmm0, 40(%rsp)
	cmpq	%r8, %r11
	cmovaeq	%r8, %r11
	vmovss	700(%rsp), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	cmpq	%r9, %r11
	cmovaeq	%r9, %r11
	vmovss	780(%rsp), %xmm0
	vmovss	%xmm0, 76(%rsp)
	cmpq	%rbx, %r11
	cmovaeq	%rbx, %r11
	vmovss	776(%rsp), %xmm0
	vmovaps	%xmm0, 96(%rsp)
	cmpq	%r10, %r11
	cmovaeq	%r10, %r11
	cmpq	%r14, %r11
	cmovaeq	%r14, %r11
	movq	1144(%rsp), %rsi
	subq	%rcx, %rsi
	cmpq	%rsi, %r11
	cmovbq	%r11, %rsi
	movq	%rsi, 352(%rsp)
	vmovss	796(%rsp), %xmm0
	vmovss	%xmm0, 12(%rsp)
	xorl	%ebx, %ebx
	vmovss	784(%rsp), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	792(%rsp), %xmm7
	vmovss	712(%rsp), %xmm15
	vmovss	708(%rsp), %xmm4
	vmovss	804(%rsp), %xmm0
	vmovss	800(%rsp), %xmm3
	vmovss	%xmm2, 164(%rsp)
.Ltmp5945:
	.loc	32 0 12
.Ltmp5946:
	.p2align	4
.LBB34_283:
	.loc	15 1890 85 is_stmt 1
	cmpb	$0, 992(%rsp)
	movq	392(%rsp), %rsi
	je	.LBB34_287
	.loc	15 0 85 is_stmt 0
	movl	172(%rsp), %edi
	movq	272(%rsp), %r8
	.loc	15 1890 85
	jmp	.LBB34_300
	.loc	15 0 85
.Ltmp5947:
	.p2align	4
.LBB34_287:
	vmovss	52(%rsp), %xmm1
.Ltmp5948:
	.loc	39 66 9 is_stmt 1
	vaddss	.LCPI34_1(%rip), %xmm1, %xmm5
.Ltmp5949:
	.loc	39 124 14
	vxorps	%xmm1, %xmm1, %xmm1
	vucomiss	%xmm1, %xmm5
	movl	172(%rsp), %edi
	movq	272(%rsp), %r8
.Ltmp5950:
	.loc	39 161 24
	ja	.LBB34_288
.Ltmp5951:
	.loc	39 0 24 is_stmt 0
	vmovss	160(%rsp), %xmm11
	jmp	.LBB34_290
	.p2align	4
.LBB34_288:
	.loc	15 748 0 is_stmt 1
	vaddss	80(%rsp), %xmm11, %xmm11
.Ltmp5952:
.LBB34_290:
	.loc	15 0 0 is_stmt 0
	vmovss	64(%rsp), %xmm1
	vmovaps	304(%rsp), %xmm2
.Ltmp5953:
	.loc	39 66 9 is_stmt 1
	vaddss	.LCPI34_1(%rip), %xmm1, %xmm1
.Ltmp5954:
	.loc	39 124 14
	vxorps	%xmm6, %xmm6, %xmm6
	vucomiss	%xmm6, %xmm1
.Ltmp5955:
	.loc	39 161 24
	ja	.LBB34_291
.Ltmp5956:
	.loc	39 0 24 is_stmt 0
	vmovss	156(%rsp), %xmm2
	vmovss	%xmm2, 40(%rsp)
	vmovaps	304(%rsp), %xmm2
	jmp	.LBB34_293
	.p2align	4
.LBB34_291:
	vmovss	40(%rsp), %xmm6
	.loc	15 748 0 is_stmt 1
	vaddss	%xmm2, %xmm6, %xmm6
	vmovss	%xmm6, 40(%rsp)
.LBB34_293:
	.loc	15 0 0 is_stmt 0
	vxorps	%xmm14, %xmm14, %xmm14
.Ltmp5957:
	.loc	39 161 24 is_stmt 1
	vcmpltss	%xmm1, %xmm14, %xmm6
	vandps	%xmm2, %xmm6, %xmm2
.Ltmp5958:
	.loc	15 749 9
	vmovss	%xmm2, 700(%rsp)
	vmovss	76(%rsp), %xmm6
.Ltmp5959:
	.loc	39 66 9
	vaddss	.LCPI34_1(%rip), %xmm6, %xmm6
	vxorps	%xmm13, %xmm13, %xmm13
.Ltmp5960:
	.loc	39 124 14
	vucomiss	%xmm14, %xmm6
.Ltmp5961:
	.loc	39 161 24
	ja	.LBB34_294
.Ltmp5962:
	.loc	39 0 24 is_stmt 0
	vmovss	152(%rsp), %xmm9
	jmp	.LBB34_296
	.p2align	4
.LBB34_294:
	.loc	15 748 0 is_stmt 1
	vaddss	96(%rsp), %xmm9, %xmm9
.Ltmp5963:
.LBB34_296:
	.loc	15 0 0 is_stmt 0
	vmovss	12(%rsp), %xmm14
.Ltmp5964:
	.loc	39 66 9 is_stmt 1
	vaddss	.LCPI34_1(%rip), %xmm14, %xmm14
.Ltmp5965:
	.loc	39 124 14
	vucomiss	%xmm13, %xmm14
	vmovaps	%xmm2, 304(%rsp)
.Ltmp5966:
	.loc	39 161 24
	ja	.LBB34_297
.Ltmp5967:
	.loc	39 0 24 is_stmt 0
	vmovss	148(%rsp), %xmm2
	vmovss	%xmm2, 24(%rsp)
	jmp	.LBB34_299
	.p2align	4
.LBB34_297:
	vmovss	24(%rsp), %xmm13
	.loc	15 748 0 is_stmt 1
	vaddss	%xmm7, %xmm13, %xmm13
	vmovss	%xmm13, 24(%rsp)
.Ltmp5968:
.LBB34_299:
	.loc	15 0 0 is_stmt 0
	vxorps	%xmm13, %xmm13, %xmm13
.Ltmp5969:
	vmaxss	%xmm13, %xmm5, %xmm2
	vmovss	%xmm2, 52(%rsp)
	vcmpltss	%xmm5, %xmm13, %xmm5
	vmovaps	80(%rsp), %xmm2
	vandps	%xmm2, %xmm5, %xmm2
	vmovaps	%xmm2, 80(%rsp)
	vmaxss	%xmm13, %xmm1, %xmm1
	vmovss	%xmm1, 64(%rsp)
	vmaxss	%xmm13, %xmm6, %xmm1
	vmovss	%xmm1, 76(%rsp)
	vcmpltss	%xmm6, %xmm13, %xmm1
	vmovaps	96(%rsp), %xmm2
	vandps	%xmm2, %xmm1, %xmm2
	vmovaps	%xmm2, 96(%rsp)
.Ltmp5970:
	vmaxss	%xmm13, %xmm14, %xmm1
	vmovss	%xmm1, 12(%rsp)
.Ltmp5971:
	.loc	39 161 24 is_stmt 1
	vcmpltss	%xmm14, %xmm13, %xmm1
	vandps	%xmm7, %xmm1, %xmm7
.Ltmp5972:
	.loc	15 749 9
	vmovss	%xmm7, 792(%rsp)
	vmovss	164(%rsp), %xmm2
.Ltmp5973:
.LBB34_300:
	.loc	15 0 9 is_stmt 0
	movq	384(%rsp), %rcx
.Ltmp5974:
	.loc	39 51 9 is_stmt 1
	vmovss	(%rcx,%rbx,4), %xmm1
	movq	376(%rsp), %rcx
.Ltmp5975:
	.loc	39 51 9 is_stmt 0
	vmovss	(%rcx,%rbx,4), %xmm5
.Ltmp5976:
	.loc	39 124 14 is_stmt 1
	vucomiss	%xmm1, %xmm5
.Ltmp5977:
	.loc	19 1244 18
	vmovd	%xmm5, %r9d
.Ltmp5978:
	.loc	19 1244 18 is_stmt 0
	vmovd	%xmm1, %ecx
.Ltmp5979:
	.loc	39 161 24 is_stmt 1
	movl	%r9d, %r15d
	cmovbel	%ecx, %r15d
.Ltmp5980:
	.loc	15 1397 26
	leaq	(%rbx,%rsi), %r10
.Ltmp5981:
	.loc	14 1050 16
	cmpq	%rdx, %r10
	jae	.LBB34_301
.Ltmp5982:
	.loc	15 0 0 is_stmt 0
	andl	984(%rsp), %r15d
.Ltmp5983:
	.loc	39 161 44 is_stmt 1
	andl	%edi, %ecx
	.loc	39 161 24 is_stmt 0
	orl	%r15d, %ecx
.Ltmp5984:
	.loc	19 1291 18 is_stmt 1
	vmovd	%ecx, %xmm5
.Ltmp5985:
	.loc	39 124 14
	vucomiss	%xmm11, %xmm5
	vmovaps	%xmm12, %xmm1
.Ltmp5986:
	.loc	39 161 24
	jbe	.LBB34_304
.Ltmp5987:
	.loc	39 76 9
	vdivss	%xmm5, %xmm11, %xmm1
.Ltmp5988:
.LBB34_304:
	.loc	39 0 9 is_stmt 0
	movq	176(%rsp), %rcx
	vmovss	(%rcx,%rbx,4), %xmm13
	movq	136(%rsp), %rcx
	vmovss	(%rcx,%rbx,4), %xmm6
	movq	240(%rsp), %rcx
	leaq	(%rbx,%rcx), %rdi
.Ltmp5989:
	.loc	39 56 9 is_stmt 1
	vmovss	%xmm1, (%r13,%r10,4)
.Ltmp5990:
	.loc	14 1050 16
	cmpq	%rdx, %rdi
	jae	.LBB34_428
.Ltmp5991:
	.loc	14 0 16 is_stmt 0
	movq	%r12, %r11
.Ltmp5992:
	.loc	39 51 9 is_stmt 1
	vmovss	(%r13,%rdi,4), %xmm14
	vmovaps	%xmm14, %xmm1
.Ltmp5993:
	.loc	15 1101 22
	cmpl	$0, 56(%rsp)
	je	.LBB34_307
.Ltmp5994:
	.loc	15 0 22 is_stmt 0
	vminss	%xmm14, %xmm8, %xmm1
.LBB34_307:
	movq	248(%rsp), %rcx
	leaq	(%rbx,%rcx), %r14
	movq	56(%rsp), %r12
.Ltmp5995:
	movl	%r12d, %r12d
.Ltmp5996:
	.loc	15 1107 20 is_stmt 1
	incq	%r12
	movq	360(%rsp), %rcx
	movq	%r12, 56(%rsp)
	cmpq	368(%rsp), %r12
	movq	%r11, %r12
	movq	%r8, %r11
.Ltmp5997:
	.loc	15 1108 22
	jne	.LBB34_308
	.loc	15 0 22 is_stmt 0
.Ltmp5998:
	.p2align	4
.LBB34_310:
.Ltmp5999:
	.loc	14 1050 16 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB34_428
.Ltmp6000:
	.loc	39 161 24
	vminss	(%r13,%rdi,4), %xmm14, %xmm14
.Ltmp6001:
	.loc	39 56 9
	vmovss	%xmm14, (%r13,%rdi,4)
.Ltmp6002:
	.loc	15 1119 16
	testq	%rdi, %rdi
	cmoveq	32(%rsp), %rdi
	.loc	15 1122 13
	decq	%rdi
.Ltmp6003:
	.loc	10 1916 50
	decq	%rcx
.Ltmp6004:
	.loc	11 900 12
	jne	.LBB34_310
.Ltmp6005:
	.loc	11 0 12 is_stmt 0
	movq	$0, 56(%rsp)
	vmovaps	%xmm1, %xmm8
	jmp	.LBB34_313
	.p2align	4
.LBB34_308:
.Ltmp6006:
	.loc	14 1050 16 is_stmt 1
	cmpq	%rdx, %r14
	jae	.LBB34_429
.Ltmp6007:
	.loc	39 51 9
	vmovss	(%r13,%r14,4), %xmm8
.Ltmp6008:
	.loc	39 161 24
	vminss	%xmm1, %xmm8, %xmm8
.Ltmp6009:
.LBB34_313:
	.loc	39 0 24 is_stmt 0
	movq	296(%rsp), %rcx
	leaq	(%rbx,%rcx), %rdi
.Ltmp6010:
	.loc	39 71 9 is_stmt 1
	vmulss	.LCPI34_3(%rip), %xmm8, %xmm8
.Ltmp6011:
	.loc	19 1783 9
	vroundss	$9, %xmm8, %xmm8, %xmm8
.Ltmp6012:
	.loc	39 71 9
	vmulss	.LCPI34_4(%rip), %xmm8, %xmm8
.Ltmp6013:
	.loc	14 1050 16
	cmpq	%r11, %rdi
	jae	.LBB34_430
.Ltmp6014:
	.loc	15 0 0 is_stmt 0
	addq	%rbx, %rsi
	incq	%rsi
.Ltmp6015:
	.loc	39 61 9 is_stmt 1
	vaddss	%xmm15, %xmm8, %xmm14
	movq	344(%rsp), %rcx
.Ltmp6016:
	.loc	39 66 9
	vsubss	(%rcx,%rdi,4), %xmm14, %xmm15
.Ltmp6017:
	.loc	15 1549 5
	vmovss	%xmm15, 712(%rsp)
	cmpq	%r11, %rsi
.Ltmp6018:
	.loc	14 1050 16
	ja	.LBB34_431
.Ltmp6019:
	.loc	14 0 16 is_stmt 0
	movq	%rcx, %rdi
	movq	120(%rsp), %rcx
	leaq	(%rbx,%rcx), %r11
.Ltmp6020:
	.loc	39 56 9 is_stmt 1
	vmovss	%xmm8, (%rdi,%r10,4)
.Ltmp6021:
	.loc	39 76 9
	vdivss	%xmm2, %xmm15, %xmm8
.Ltmp6022:
	.loc	39 66 9
	vsubss	%xmm8, %xmm12, %xmm8
.Ltmp6023:
	.loc	39 66 9 is_stmt 0
	vsubss	%xmm4, %xmm8, %xmm14
.Ltmp6024:
	.loc	39 92 9 is_stmt 1
	vmulss	40(%rsp), %xmm14, %xmm14
	vaddss	%xmm4, %xmm14, %xmm4
.Ltmp6025:
	.loc	39 161 24
	vmaxss	%xmm4, %xmm8, %xmm4
.Ltmp6026:
	.loc	39 103 24
	vbroadcastss	.LCPI34_0(%rip), %xmm8
	vandps	%xmm4, %xmm8, %xmm14
.Ltmp6027:
	.loc	39 166 24
	vcmpnltss	.LCPI34_5(%rip), %xmm14, %xmm14
	vandps	%xmm4, %xmm14, %xmm4
.Ltmp6028:
	.loc	15 1555 5
	vmovss	%xmm4, 708(%rsp)
.Ltmp6029:
	.loc	14 1050 16
	cmpq	264(%rsp), %r11
	jae	.LBB34_432
.Ltmp6030:
	.loc	39 66 9
	vsubss	%xmm4, %xmm12, %xmm14
	vmovaps	%xmm12, %xmm5
	movq	336(%rsp), %rcx
.Ltmp6031:
	.loc	39 51 9
	vmovss	(%rcx,%r11,4), %xmm12
.Ltmp6032:
	.loc	39 56 9
	vmovss	%xmm13, (%rcx,%r11,4)
.Ltmp6033:
	.loc	39 71 9
	vmulss	%xmm12, %xmm14, %xmm13
.Ltmp6034:
	.loc	19 1244 18
	vmovd	%xmm12, %edi
.Ltmp6035:
	.loc	39 161 24
	andl	168(%rsp), %edi
.Ltmp6036:
	.loc	19 1244 18
	vmovd	%xmm13, %ecx
.Ltmp6037:
	.loc	39 161 44
	andl	236(%rsp), %ecx
	.loc	39 161 24 is_stmt 0
	orl	%edi, %ecx
	movq	176(%rsp), %rdi
.Ltmp6038:
	.loc	39 56 9 is_stmt 1
	movl	%ecx, (%rdi,%rbx,4)
	cmpq	%rax, %rsi
.Ltmp6039:
	.loc	14 1050 16
	ja	.LBB34_433
.Ltmp6040:
	.loc	15 0 0 is_stmt 0
	andl	172(%rsp), %r9d
	orl	%r9d, %r15d
	vmovd	%r15d, %xmm14
.Ltmp6041:
	.loc	39 124 14 is_stmt 1
	vucomiss	%xmm9, %xmm14
	vmovaps	%xmm5, %xmm13
	vmovaps	%xmm5, %xmm12
.Ltmp6042:
	.loc	39 161 24
	jbe	.LBB34_319
.Ltmp6043:
	.loc	39 76 9
	vdivss	%xmm14, %xmm9, %xmm13
.Ltmp6044:
.LBB34_319:
	.loc	39 0 9 is_stmt 0
	movq	416(%rsp), %rcx
	leaq	(%rbx,%rcx), %rdi
.Ltmp6045:
	.loc	39 56 9 is_stmt 1
	vmovss	%xmm13, (%rbp,%r10,4)
.Ltmp6046:
	.loc	14 1050 16
	cmpq	%rax, %rdi
	movq	280(%rsp), %r15
	movq	328(%rsp), %rcx
	jae	.LBB34_434
.Ltmp6047:
	.loc	39 51 9
	vmovss	(%rbp,%rdi,4), %xmm14
	vmovaps	%xmm14, %xmm5
.Ltmp6048:
	.loc	15 1101 22
	testl	%r12d, %r12d
	je	.LBB34_322
.Ltmp6049:
	.loc	15 0 22 is_stmt 0
	vminss	%xmm14, %xmm10, %xmm5
.LBB34_322:
	movl	%r12d, %r12d
.Ltmp6050:
	.loc	15 1107 20 is_stmt 1
	incq	%r12
	movq	488(%rsp), %r9
	cmpq	496(%rsp), %r12
.Ltmp6051:
	.loc	15 1108 22
	jne	.LBB34_323
	.loc	15 0 22 is_stmt 0
.Ltmp6052:
	.p2align	4
.LBB34_325:
.Ltmp6053:
	.loc	14 1050 16 is_stmt 1
	cmpq	%rax, %rdi
	jae	.LBB34_434
.Ltmp6054:
	.loc	39 161 24
	vminss	(%rbp,%rdi,4), %xmm14, %xmm14
.Ltmp6055:
	.loc	39 56 9
	vmovss	%xmm14, (%rbp,%rdi,4)
.Ltmp6056:
	.loc	15 1119 16
	testq	%rdi, %rdi
	cmoveq	32(%rsp), %rdi
	.loc	15 1122 13
	decq	%rdi
.Ltmp6057:
	.loc	10 1916 50
	decq	%r9
.Ltmp6058:
	.loc	11 900 12
	jne	.LBB34_325
.Ltmp6059:
	.loc	11 0 12 is_stmt 0
	xorl	%r12d, %r12d
	vmovaps	%xmm5, %xmm10
	jmp	.LBB34_328
	.p2align	4
.LBB34_323:
.Ltmp6060:
	.loc	14 1050 16 is_stmt 1
	cmpq	%rax, %r14
	jae	.LBB34_435
.Ltmp6061:
	.loc	39 51 9
	vmovss	(%rbp,%r14,4), %xmm10
.Ltmp6062:
	.loc	39 161 24
	vminss	%xmm5, %xmm10, %xmm10
.Ltmp6063:
.LBB34_328:
	.loc	39 0 24 is_stmt 0
	movq	288(%rsp), %rdi
	addq	%rbx, %rdi
.Ltmp6064:
	.loc	39 71 9 is_stmt 1
	vmulss	.LCPI34_3(%rip), %xmm10, %xmm10
.Ltmp6065:
	.loc	19 1783 9
	vroundss	$9, %xmm10, %xmm10, %xmm10
.Ltmp6066:
	.loc	39 71 9
	vmulss	.LCPI34_4(%rip), %xmm10, %xmm10
.Ltmp6067:
	.loc	14 1050 16
	cmpq	%rcx, %rdi
	movq	480(%rsp), %r8
	jae	.LBB34_436
.Ltmp6068:
	.loc	39 61 9
	vaddss	%xmm0, %xmm10, %xmm0
.Ltmp6069:
	.loc	39 66 9
	vsubss	(%r8,%rdi,4), %xmm0, %xmm0
.Ltmp6070:
	.loc	15 1549 5
	vmovss	%xmm0, 804(%rsp)
	cmpq	%rcx, %rsi
.Ltmp6071:
	.loc	14 1050 16
	ja	.LBB34_330
.Ltmp6072:
	.loc	15 0 0 is_stmt 0
	leaq	1(%r11), %rsi
.Ltmp6073:
	.loc	39 56 9 is_stmt 1
	vmovss	%xmm10, (%r8,%r10,4)
.Ltmp6074:
	.loc	39 76 9
	vdivss	132(%rsp), %xmm0, %xmm10
.Ltmp6075:
	.loc	39 66 9
	vsubss	%xmm10, %xmm12, %xmm10
	vmovaps	%xmm12, %xmm14
.Ltmp6076:
	.loc	39 66 9 is_stmt 0
	vsubss	%xmm3, %xmm10, %xmm12
.Ltmp6077:
	.loc	39 92 9 is_stmt 1
	vmulss	24(%rsp), %xmm12, %xmm12
	vaddss	%xmm3, %xmm12, %xmm3
.Ltmp6078:
	.loc	39 161 24
	vmaxss	%xmm3, %xmm10, %xmm3
.Ltmp6079:
	.loc	39 103 24
	vandps	%xmm3, %xmm8, %xmm8
.Ltmp6080:
	.loc	39 166 24
	vcmpnltss	.LCPI34_5(%rip), %xmm8, %xmm8
	vandps	%xmm3, %xmm8, %xmm3
.Ltmp6081:
	.loc	15 1555 5
	vmovss	%xmm3, 800(%rsp)
	cmpq	408(%rsp), %rsi
.Ltmp6082:
	.loc	14 1050 16
	ja	.LBB34_437
.Ltmp6083:
	.loc	39 66 9
	vsubss	%xmm3, %xmm14, %xmm8
	movq	472(%rsp), %rcx
.Ltmp6084:
	.loc	39 51 9
	vmovss	(%rcx,%r11,4), %xmm10
.Ltmp6085:
	.loc	39 56 9
	vmovss	%xmm6, (%rcx,%r11,4)
.Ltmp6086:
	.loc	39 71 9
	vmulss	%xmm10, %xmm8, %xmm6
.Ltmp6087:
	.loc	19 1244 18
	vmovd	%xmm10, %ecx
.Ltmp6088:
	.loc	39 161 24
	andl	168(%rsp), %ecx
.Ltmp6089:
	.loc	19 1244 18
	vmovd	%xmm6, %esi
.Ltmp6090:
	.loc	39 161 44
	andl	236(%rsp), %esi
	.loc	39 161 24 is_stmt 0
	orl	%ecx, %esi
	movq	136(%rsp), %rcx
.Ltmp6091:
	.loc	39 56 9 is_stmt 1
	movl	%esi, (%rcx,%rbx,4)
.Ltmp6092:
	.loc	15 0 0 is_stmt 0
	incq	%rbx
	vmovaps	%xmm5, %xmm10
	vmovaps	%xmm1, %xmm8
.Ltmp6093:
	.loc	32 304 12 is_stmt 1
	cmpq	352(%rsp), %rbx
	vmovaps	%xmm14, %xmm12
	jne	.LBB34_283
.Ltmp6094:
	.loc	32 0 12 is_stmt 0
	movq	%r12, 320(%rsp)
.Ltmp6095:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6096:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6097:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6098:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6099:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6100:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
	vmovss	52(%rsp), %xmm0
.Ltmp6101:
	.loc	15 746 0
	vmovss	%xmm0, 688(%rsp)
	vmovaps	80(%rsp), %xmm0
.Ltmp6102:
	.loc	15 0 0 is_stmt 0
	vmovss	%xmm0, 684(%rsp)
	vmovss	76(%rsp), %xmm0
.Ltmp6103:
	.loc	15 746 0
	vmovss	%xmm0, 780(%rsp)
	vmovaps	96(%rsp), %xmm0
.Ltmp6104:
	.loc	15 0 0
	vmovss	%xmm0, 776(%rsp)
	vmovaps	%xmm5, %xmm0
	vmovaps	%xmm1, %xmm2
	vmovaps	%xmm5, %xmm10
	vmovaps	%xmm1, %xmm8
	movq	392(%rsp), %r11
	jmp	.LBB34_286
.Ltmp6105:
.LBB34_281:
	movq	392(%rsp), %r11
	movq	280(%rsp), %r15
.LBB34_286:
	movq	256(%rsp), %rcx
	movq	464(%rsp), %rdi
	addq	%rdi, %rcx
.Ltmp6106:
	.loc	15 1942 39 is_stmt 1
	addq	%rdi, %r11
	movq	32(%rsp), %rsi
.Ltmp6107:
	.loc	15 1043 8
	cmpq	%rsi, %r11
	movl	$0, %r10d
	cmovbq	%r10, %rsi
	subq	%rsi, %r11
	movq	120(%rsp), %r8
.Ltmp6108:
	.loc	15 1943 39
	addq	%rdi, %r8
	movq	1136(%rsp), %rsi
.Ltmp6109:
	.loc	15 1043 8
	cmpq	%rsi, %r8
	cmovbq	%r10, %rsi
	subq	%rsi, %r8
	movq	1104(%rsp), %r12
.Ltmp6110:
	.loc	15 1865 19
	cmpq	%r12, %rcx
	movq	448(%rsp), %rdi
	movq	440(%rsp), %rbx
	jb	.LBB34_274
	.loc	15 0 19 is_stmt 0
	movq	%r11, 392(%rsp)
	movq	%r8, 120(%rsp)
.Ltmp6111:
	vmovss	%xmm2, 584(%rsp)
.Ltmp6112:
	vmovss	%xmm0, 1080(%rsp)
	movq	56(%rsp), %rax
	movl	%eax, 588(%rsp)
	movq	320(%rsp), %rax
	movl	%eax, 1084(%rsp)
	movq	920(%rsp), %r13
	movq	976(%rsp), %rbx
	movq	912(%rsp), %r12
	movq	904(%rsp), %rbp
	movq	432(%rsp), %r8
	movl	$32, %r9d
	movq	424(%rsp), %r10
	movq	400(%rsp), %r15
	jmp	.LBB34_257
.Ltmp6113:
.LBB34_334:
	movq	392(%rsp), %r14
.Ltmp6114:
	.loc	39 56 9 is_stmt 1
	cmpq	$0, 224(%r13)
	je	.LBB34_335
.Ltmp6115:
	.loc	15 0 0 is_stmt 0
	vmovss	584(%rsp), %xmm1
	movl	588(%rsp), %ecx
	vmovss	1080(%rsp), %xmm0
	movl	1084(%rsp), %eax
.Ltmp6116:
	.loc	15 1958 0 is_stmt 1
	movq	216(%r13), %rdx
.Ltmp6117:
	.loc	39 56 9
	vmovss	%xmm1, (%rdx)
.Ltmp6118:
	.loc	15 1959 5
	movq	256(%r13), %rdx
.Ltmp6119:
	.loc	16 1714 9
	testq	%rdx, %rdx
.Ltmp6120:
	.loc	17 180 28
	je	.LBB34_339
.Ltmp6121:
	.loc	17 0 28 is_stmt 0
	movq	248(%r13), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB34_338:
.Ltmp6122:
	.loc	31 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp6123:
	.loc	16 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp6124:
	.loc	17 180 28
	jne	.LBB34_338
.Ltmp6125:
.LBB34_339:
	.loc	39 56 9
	cmpq	$0, 424(%r13)
	je	.LBB34_335
.Ltmp6126:
	.loc	15 1960 0
	movq	416(%r13), %rcx
.Ltmp6127:
	.loc	39 56 9
	vmovss	%xmm0, (%rcx)
.Ltmp6128:
	.loc	15 1961 5
	movq	456(%r13), %rcx
.Ltmp6129:
	.loc	16 1714 9
	testq	%rcx, %rcx
.Ltmp6130:
	.loc	17 180 28
	je	.LBB34_343
.Ltmp6131:
	.loc	17 0 28 is_stmt 0
	movq	448(%r13), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB34_342:
.Ltmp6132:
	.loc	31 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp6133:
	.loc	16 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp6134:
	.loc	17 180 28
	jne	.LBB34_342
.Ltmp6135:
.LBB34_343:
	.loc	17 0 28 is_stmt 0
	leaq	628(%rsp), %rdi
	movq	968(%rsp), %rsi
	.loc	15 1963 14 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	leaq	720(%rsp), %rdi
	movq	960(%rsp), %rsi
	.loc	15 1964 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
	movq	120(%rsp), %rax
	.loc	15 1965 5
	movl	%eax, 560(%r13)
	.loc	15 1966 5
	movl	%r14d, 564(%r13)
.Ltmp6136:
	.loc	15 2141 35
	cmpb	$0, 1004(%rsp)
	jne	.LBB34_398
.LBB34_345:
	.loc	15 0 35 is_stmt 0
	xorl	%eax, %eax
.LBB34_346:
	leaq	536(%r13), %rcx
	.loc	15 2141 9 is_stmt 1
	movb	%al, 780(%r13)
	.loc	15 2146 30
	movzbl	96(%r13), %eax
	.loc	15 2146 9 is_stmt 0
	movb	%al, 781(%r13)
	.loc	15 2147 21 is_stmt 1
	movq	16(%rcx), %rax
	movq	%rax, 1168(%rsp)
	vmovups	(%rcx), %xmm0
	vmovaps	%xmm0, 1152(%rsp)
.Ltmp6137:
	.loc	15 2148 20
	movl	72(%r13), %r8d
.Ltmp6138:
	.loc	15 2154 64
	movq	104(%r13), %r14
	movq	112(%r13), %r10
	movq	120(%r13), %rsi
	movq	128(%r13), %r9
.Ltmp6139:
	.loc	30 2155 12
	testq	%r12, %r12
	movq	504(%rsp), %rdi
	movq	608(%rsp), %rbx
	je	.LBB34_350
.Ltmp6140:
	.loc	30 0 12 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI34_0(%rip), %xmm0
	vmovss	.LCPI34_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB34_348:
.Ltmp6141:
	.loc	39 103 24 is_stmt 1
	vmovss	(%rdi,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp6142:
	.loc	39 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp6143:
	.loc	39 139 9
	cmovbel	%ecx, %eax
.Ltmp6144:
	.loc	30 2155 12
	incq	%rdx
	cmpq	%rdx, %r12
	jne	.LBB34_348
.Ltmp6145:
	.loc	39 154 9
	cmpl	$-1, %eax
.Ltmp6146:
	.loc	34 208 8
	jne	.LBB34_413
.LBB34_350:
.Ltmp6147:
	.loc	30 2155 12
	testq	%rbp, %rbp
	je	.LBB34_397
.Ltmp6148:
	.loc	30 0 12 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI34_0(%rip), %xmm0
	vmovss	.LCPI34_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB34_352:
.Ltmp6149:
	.loc	39 103 24 is_stmt 1
	vmovss	(%rbx,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp6150:
	.loc	39 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp6151:
	.loc	39 139 9
	cmovbel	%ecx, %eax
.Ltmp6152:
	.loc	30 2155 12
	incq	%rdx
	cmpq	%rdx, %rbp
	jne	.LBB34_352
.Ltmp6153:
	.loc	39 154 9
	cmpl	$-1, %eax
.Ltmp6154:
	.loc	34 208 34
	je	.LBB34_397
.Ltmp6155:
	.loc	30 2155 12
	testq	%r12, %r12
.Ltmp6156:
	.loc	30 2155 12 is_stmt 0
	je	.LBB34_355
.Ltmp6157:
.LBB34_413:
	.loc	30 0 12
	movl	$-1, %ecx
	xorl	%eax, %eax
	vbroadcastss	.LCPI34_0(%rip), %xmm0
	vmovss	.LCPI34_6(%rip), %xmm1
	xorl	%edx, %edx
	.p2align	4
.LBB34_414:
.Ltmp6158:
	.loc	39 103 24 is_stmt 1
	vmovss	(%rdi,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp6159:
	.loc	39 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp6160:
	.loc	39 139 9
	cmovbel	%eax, %ecx
.Ltmp6161:
	.loc	30 2155 12
	incq	%rdx
	cmpq	%rdx, %r12
	jne	.LBB34_414
.Ltmp6162:
	.loc	34 185 12
	notl	%ecx
	xorl	%eax, %eax
	testl	$1065353216, %ecx
	setne	%al
.Ltmp6163:
	.loc	30 2155 12
	testq	%rbp, %rbp
	movq	%rsi, 32(%rsp)
	jne	.LBB34_356
.Ltmp6164:
	.loc	30 0 12 is_stmt 0
	movq	%r10, 136(%rsp)
	movq	%r9, 56(%rsp)
	movl	%r8d, 176(%rsp)
	.loc	34 211 5 is_stmt 1
	movl	%eax, 576(%r13)
	.loc	34 212 31
	movq	568(%r13), %rax
.Ltmp6165:
	.loc	14 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp6166:
	.loc	34 212 5
	movq	%rcx, 568(%r13)
	jmp	.LBB34_417
.LBB34_355:
	.loc	34 0 5 is_stmt 0
	movq	%rsi, 32(%rsp)
	xorl	%eax, %eax
.LBB34_356:
	movl	$-1, %ecx
	xorl	%edx, %edx
	vbroadcastss	.LCPI34_0(%rip), %xmm0
	vmovss	.LCPI34_6(%rip), %xmm1
	xorl	%esi, %esi
	.p2align	4
.LBB34_357:
.Ltmp6167:
	.loc	39 103 24 is_stmt 1
	vmovss	(%rbx,%rsi,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp6168:
	.loc	39 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp6169:
	.loc	39 139 9
	cmovbel	%edx, %ecx
.Ltmp6170:
	.loc	30 2155 12
	incq	%rsi
	cmpq	%rsi, %rbp
	jne	.LBB34_357
.Ltmp6171:
	.loc	30 0 12 is_stmt 0
	movq	%r10, 136(%rsp)
	movq	%r9, 56(%rsp)
	movl	%r8d, 176(%rsp)
.Ltmp6172:
	.loc	34 185 12 is_stmt 1
	notl	%ecx
	xorl	%edx, %edx
	testl	$1065353216, %ecx
	setne	%dl
.Ltmp6173:
	.loc	34 211 5
	orl	%edx, %eax
	.loc	34 212 31
	movq	568(%r13), %rcx
.Ltmp6174:
	.loc	14 2428 13
	incq	%rcx
	movq	$-1, %rdx
	cmovneq	%rcx, %rdx
.Ltmp6175:
	.loc	34 211 5
	movl	%eax, 576(%r13)
	.loc	34 212 5
	movq	%rdx, 568(%r13)
.Ltmp6176:
	.loc	30 2155 12
	testq	%r12, %r12
	je	.LBB34_360
.Ltmp6177:
	.loc	12 961 18
	shlq	$2, %r12
.Ltmp6178:
	.loc	31 25 13
	xorl	%esi, %esi
	movq	%r12, %rdx
	callq	*memset@GOTPCREL(%rip)
.Ltmp6179:
.LBB34_360:
	.loc	31 0 13 is_stmt 0
	movq	%rbp, %r12
	movq	%rbx, %rdi
.LBB34_417:
.Ltmp6180:
	.loc	12 961 18 is_stmt 1
	shlq	$2, %r12
.Ltmp6181:
	.loc	31 25 13
	xorl	%esi, %esi
	movq	%r12, %rdx
	callq	*memset@GOTPCREL(%rip)
.Ltmp6182:
	.loc	15 2155 18
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %rbx
	leaq	1152(%rsp), %r15
	movq	968(%rsp), %rdi
	movq	%r15, %rsi
	movq	%r14, %rdx
	movq	136(%rsp), %rcx
	movl	176(%rsp), %ebp
	movl	%ebp, %r8d
	callq	*%rbx
	movq	960(%rsp), %rdi
	.loc	15 2156 19
	movq	%r15, %rsi
	movq	32(%rsp), %rdx
	movq	56(%rsp), %rcx
	movl	%ebp, %r8d
	callq	*%rbx
	.loc	15 2157 13
	movq	$0, 560(%r13)
.Ltmp6183:
.LBB34_397:
	.loc	15 2159 6 epilogue_begin
	addq	$3208, %rsp
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
.LBB34_428:
	.cfi_def_cfa_offset 3264
	.loc	15 0 6 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6184:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6185:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6186:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6187:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6188:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6189:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6190:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6191:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6192:
	.loc	15 1026 25
	leaq	1(%rdi), %rsi
.Ltmp6193:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6194:
.LBB34_434:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6195:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6196:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6197:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6198:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6199:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6200:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6201:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6202:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6203:
	.loc	15 1026 25
	leaq	1(%rdi), %rsi
.Ltmp6204:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6205:
.LBB34_361:
	.loc	37 0 13 is_stmt 0
	movb	$1, %dil
	.loc	15 2096 12 is_stmt 1
	cmpb	$0, 780(%r13)
	je	.LBB34_30
.Ltmp6206:
	.loc	15 660 31
	movq	256(%r13), %rsi
	.loc	15 660 57 is_stmt 0
	movq	320(%r13), %rax
.Ltmp6207:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rsi, %rax
	cmovbq	%rax, %rsi
.Ltmp6208:
	.loc	32 304 12
	testq	%rsi, %rsi
	je	.LBB34_372
.Ltmp6209:
	.loc	32 0 12 is_stmt 0
	movq	248(%r13), %rdi
	movq	312(%r13), %r8
	xorl	%ebx, %ebx
	movq	%r9, %r10
	shrq	$32, %r10
	jmp	.LBB34_364
.LBB34_370:
.Ltmp6210:
	.loc	15 662 22 is_stmt 1
	movq	%rcx, %rax
	xorl	%edx, %edx
	divq	%r11
.LBB34_371:
	.loc	15 662 13 is_stmt 0
	movl	%edx, (%rdi,%rbx,4)
.Ltmp6211:
	.loc	15 0 0
	incq	%rbx
.Ltmp6212:
	.loc	32 304 12 is_stmt 1
	addq	$12, %r8
	cmpq	%rbx, %rsi
	je	.LBB34_372
.Ltmp6213:
.LBB34_364:
	.loc	15 661 26
	movl	(%r8), %r11d
	testq	%r11, %r11
.Ltmp6214:
	.loc	15 662 42
	je	.LBB34_438
	.loc	15 662 24 is_stmt 0
	movl	(%rdi,%rbx,4), %ecx
	.loc	15 662 42
	testq	%r10, %r10
	je	.LBB34_366
	movq	%r9, %rax
	xorl	%edx, %edx
	divq	%r11
	.loc	15 662 23
	addq	%rdx, %rcx
	.loc	15 662 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB34_370
	jmp	.LBB34_369
.LBB34_366:
	.loc	15 662 42
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%r11d
	.loc	15 662 23
	addq	%rdx, %rcx
	.loc	15 662 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB34_370
.LBB34_369:
	movl	%ecx, %eax
	xorl	%edx, %edx
	divl	%r11d
	jmp	.LBB34_371
.Ltmp6215:
.LBB34_372:
	.loc	15 660 31 is_stmt 1
	movq	456(%r13), %rsi
	.loc	15 660 57 is_stmt 0
	movq	520(%r13), %rax
.Ltmp6216:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rsi, %rax
	cmovbq	%rax, %rsi
.Ltmp6217:
	.loc	32 304 12
	testq	%rsi, %rsi
	je	.LBB34_382
.Ltmp6218:
	.loc	32 0 12 is_stmt 0
	movq	448(%r13), %rdi
	movq	512(%r13), %r8
	xorl	%ebx, %ebx
	movq	%r9, %r10
	shrq	$32, %r10
	jmp	.LBB34_374
.LBB34_380:
.Ltmp6219:
	.loc	15 662 22 is_stmt 1
	movq	%rcx, %rax
	xorl	%edx, %edx
	divq	%r11
.LBB34_381:
	.loc	15 662 13 is_stmt 0
	movl	%edx, (%rdi,%rbx,4)
.Ltmp6220:
	.loc	15 0 0
	incq	%rbx
.Ltmp6221:
	.loc	32 304 12 is_stmt 1
	addq	$12, %r8
	cmpq	%rbx, %rsi
	je	.LBB34_382
.Ltmp6222:
.LBB34_374:
	.loc	15 661 26
	movl	(%r8), %r11d
	testq	%r11, %r11
.Ltmp6223:
	.loc	15 662 42
	je	.LBB34_438
	.loc	15 662 24 is_stmt 0
	movl	(%rdi,%rbx,4), %ecx
	.loc	15 662 42
	testq	%r10, %r10
	je	.LBB34_376
	movq	%r9, %rax
	xorl	%edx, %edx
	divq	%r11
	.loc	15 662 23
	addq	%rdx, %rcx
	.loc	15 662 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB34_380
	jmp	.LBB34_379
.LBB34_376:
	.loc	15 662 42
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%r11d
	.loc	15 662 23
	addq	%rdx, %rcx
	.loc	15 662 22
	movq	%rcx, %rax
	shrq	$32, %rax
	jne	.LBB34_380
.LBB34_379:
	movl	%ecx, %eax
	xorl	%edx, %edx
	divl	%r11d
	jmp	.LBB34_381
.Ltmp6224:
.LBB34_382:
	.loc	15 2113 26 is_stmt 1
	movq	552(%r13), %rsi
.Ltmp6225:
	.loc	15 452 44
	testq	%rsi, %rsi
	je	.LBB34_439
.Ltmp6226:
	.loc	15 2113 26
	movq	544(%r13), %rcx
.Ltmp6227:
	.loc	15 452 23
	movl	560(%r13), %edi
	.loc	15 452 44 is_stmt 0
	movq	%r9, %rax
	orq	%rsi, %rax
	shrq	$32, %rax
	je	.LBB34_384
	movq	%r9, %rax
	xorl	%edx, %edx
	divq	%rsi
	movq	%rdx, %rax
	jmp	.LBB34_386
.Ltmp6228:
.LBB34_301:
	.loc	15 0 44
	vmovss	16(%rsp), %xmm0
.Ltmp6229:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6230:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6231:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6232:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6233:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6234:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6235:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6236:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6237:
	.loc	32 304 12
	cmpq	%rdx, %rsi
	cmovbeq	%rdx, %rsi
	incq	%rsi
.Ltmp6238:
	.loc	37 456 13
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	%r10, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6239:
.LBB34_430:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6240:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6241:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6242:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6243:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6244:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6245:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6246:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6247:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6248:
	.loc	15 1026 25
	leaq	1(%rdi), %rsi
.Ltmp6249:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6250:
.LBB34_431:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6251:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6252:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6253:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6254:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6255:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6256:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6257:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6258:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6259:
	.loc	37 456 13
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	%r10, %rdi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6260:
.LBB34_432:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6261:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6262:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6263:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6264:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6265:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6266:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6267:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6268:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
	movq	120(%rsp), %rsi
	movq	264(%rsp), %rdx
.Ltmp6269:
	.loc	32 304 12
	cmpq	%rdx, %rsi
	cmovbeq	%rdx, %rsi
	incq	%rsi
.Ltmp6270:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r11, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6271:
.LBB34_433:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6272:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6273:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6274:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6275:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6276:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6277:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6278:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6279:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6280:
	.loc	37 456 13
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	%r10, %rdi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6281:
.LBB34_436:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6282:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6283:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6284:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6285:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6286:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6287:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6288:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6289:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6290:
	.loc	15 1026 25
	leaq	1(%rdi), %rsi
	movq	%rcx, %rdx
.Ltmp6291:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6292:
.LBB34_330:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6293:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6294:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6295:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6296:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6297:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6298:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6299:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6300:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
	movq	%rcx, %rdx
.Ltmp6301:
	.loc	37 456 13
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	%r10, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6302:
.LBB34_437:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6303:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6304:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6305:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6306:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6307:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6308:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6309:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6310:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6311:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r11, %rdi
	movq	408(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6312:
.LBB34_384:
	.loc	15 452 44
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB34_386:
	.loc	15 452 22 is_stmt 0
	addq	%rdi, %rax
	.loc	15 452 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB34_387
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB34_389
.LBB34_387:
	xorl	%edx, %edx
	divl	%esi
.LBB34_389:
	.loc	15 452 9
	movl	%edx, 560(%r13)
	.loc	15 453 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB34_440
	.loc	15 453 23 is_stmt 0
	movl	564(%r13), %esi
	.loc	15 453 44
	movq	%r9, %rax
	orq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB34_391
	movq	%r9, %rax
	xorl	%edx, %edx
	divq	%rcx
	movq	%rdx, %rax
	jmp	.LBB34_393
.LBB34_391:
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%ecx
	movl	%edx, %eax
.LBB34_393:
	.loc	15 453 22
	addq	%rsi, %rax
	.loc	15 453 21
	movq	%rax, %rdx
	orq	%rcx, %rdx
	shrq	$32, %rdx
	je	.LBB34_394
	xorl	%edx, %edx
	divq	%rcx
	jmp	.LBB34_396
.LBB34_394:
	xorl	%edx, %edx
	divl	%ecx
.LBB34_396:
	.loc	15 453 9
	movl	%edx, 564(%r13)
	jmp	.LBB34_397
.Ltmp6313:
.LBB34_266:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_e0592aef22128a0ac53753b9632a8183(%rip), %rcx
.Ltmp6314:
	.loc	15 0 0 is_stmt 0
	movq	%r12, %rsi
	movq	%r12, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_277:
.Ltmp6315:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_e0592aef22128a0ac53753b9632a8183(%rip), %rcx
.Ltmp6316:
	.loc	15 0 0 is_stmt 0
	movq	%rbp, %rsi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_201:
	vmovss	80(%rsp), %xmm0
.Ltmp6317:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6318:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6319:
	.loc	15 1158 21 is_stmt 1
	leaq	.Lalloc_077370d5cece7380867993336836eb69(%rip), %rdx
	movq	56(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6320:
.LBB34_206:
	.loc	15 0 21 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp6321:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6322:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6323:
	.loc	15 1175 9 is_stmt 1
	leaq	.Lalloc_067dce244605df4a33956fbd4d726027(%rip), %rdx
	movq	%r9, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6324:
.LBB34_202:
	.loc	15 0 9 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp6325:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6326:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6327:
	.loc	15 1168 22 is_stmt 1
	leaq	.Lalloc_5cae9ea88ed6362f62dbad10291edbd4(%rip), %rdx
	movq	%r14, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6328:
.LBB34_203:
	.loc	15 0 22 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp6329:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6330:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6331:
	.loc	15 1169 24 is_stmt 1
	leaq	.Lalloc_9fdd00ed7abe4ccfd73f0d145a7b741b(%rip), %rdx
	movq	176(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6332:
.LBB34_429:
	.loc	15 0 24 is_stmt 0
	vmovss	16(%rsp), %xmm0
.Ltmp6333:
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6334:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6335:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6336:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6337:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6338:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6339:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6340:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6341:
	.loc	15 1026 25
	leaq	1(%r14), %rsi
.Ltmp6342:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r14, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6343:
.LBB34_435:
	.loc	37 0 13 is_stmt 0
	vmovss	16(%rsp), %xmm0
	vmovss	%xmm0, 584(%rsp)
	vmovss	20(%rsp), %xmm0
.Ltmp6344:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6345:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm11, 676(%rsp)
	vmovss	64(%rsp), %xmm0
.Ltmp6346:
	.loc	15 746 9
	vmovss	%xmm0, 704(%rsp)
	vmovss	40(%rsp), %xmm0
.Ltmp6347:
	.loc	15 748 9
	vmovss	%xmm0, 692(%rsp)
.Ltmp6348:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm9, 768(%rsp)
	vmovss	12(%rsp), %xmm0
.Ltmp6349:
	.loc	15 746 9 is_stmt 1
	vmovss	%xmm0, 796(%rsp)
	vmovss	24(%rsp), %xmm0
.Ltmp6350:
	.loc	15 748 9
	vmovss	%xmm0, 784(%rsp)
.Ltmp6351:
	.loc	15 1026 25
	leaq	1(%r14), %rsi
.Ltmp6352:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r14, %rdi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6353:
.LBB34_204:
	.loc	37 0 13 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp6354:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6355:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6356:
	.loc	15 1173 24 is_stmt 1
	leaq	.Lalloc_697dc8945f5e5b040d7e9a0b92cb249f(%rip), %rdx
	movq	%r11, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6357:
.LBB34_423:
	.loc	15 0 24 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp6358:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6359:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6360:
	.loc	15 1180 24 is_stmt 1
	leaq	.Lalloc_d5ea38731a1cb6311efef2f919234d06(%rip), %rdx
	movq	%r14, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6361:
.LBB34_109:
	.loc	15 0 24 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp6362:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6363:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6364:
	.loc	15 0 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_104:
	vmovss	80(%rsp), %xmm0
.Ltmp6365:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6366:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6367:
.LBB34_76:
	.loc	15 0 0
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_276:
.Ltmp6368:
	vmovss	%xmm2, 584(%rsp)
.Ltmp6369:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6370:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_14e3d3ba493695bccf4dcde9be821bfc(%rip), %rcx
	movq	912(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6371:
.LBB34_234:
	.loc	37 0 13 is_stmt 0
	vmovss	80(%rsp), %xmm0
.Ltmp6372:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6373:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6374:
	.loc	15 1302 25 is_stmt 1
	leaq	.Lalloc_aca25255cb99dc5ba482e692667f5878(%rip), %rdx
	movq	%r8, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6375:
.LBB34_427:
	.loc	15 0 0 is_stmt 0
	vmovss	%xmm2, 584(%rsp)
.Ltmp6376:
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6377:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_8706d23ef0096dbeb31c5991bb003668(%rip), %rcx
	movq	904(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6378:
.LBB34_441:
	.loc	37 443 13
	leaq	.Lalloc_df34220a19b474a957e74caf6cc3e38d(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbx, %rsi
	movq	%r12, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6379:
.LBB34_129:
	.loc	37 0 13 is_stmt 0
	movl	$1, %eax
	jmp	.LBB34_130
.LBB34_419:
.Ltmp6380:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_449c9992b9e28dda1c528a8de6026d01(%rip), %rcx
	xorl	%edi, %edi
	movq	%r9, %rsi
	movq	%r12, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6381:
.LBB34_177:
	.loc	37 0 13 is_stmt 0
	movl	$2, %eax
	jmp	.LBB34_130
.LBB34_418:
.Ltmp6382:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_983c0eafc7ebd40f6695894c941b193b(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbx, %rsi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6383:
.LBB34_181:
	.loc	37 0 13 is_stmt 0
	movl	$3, %eax
	jmp	.LBB34_130
.LBB34_420:
.Ltmp6384:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_69177cbbe364d953a90a8930cefca3fe(%rip), %rcx
	xorl	%edi, %edi
	movq	%r9, %rsi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6385:
.LBB34_185:
	.loc	37 0 13 is_stmt 0
	movl	$4, %eax
	jmp	.LBB34_130
.LBB34_189:
	movl	$5, %eax
	jmp	.LBB34_130
.LBB34_193:
	movl	$6, %eax
	jmp	.LBB34_130
.LBB34_197:
	movl	$7, %eax
.LBB34_130:
	movq	%rax, 120(%rsp)
.LBB34_131:
	vmovss	80(%rsp), %xmm0
.Ltmp6386:
	.loc	15 748 9 is_stmt 1
	vmovss	%xmm0, 860(%rsp)
	vmovss	96(%rsp), %xmm0
.Ltmp6387:
	.loc	15 748 9 is_stmt 0
	vmovss	%xmm0, 560(%rsp)
.Ltmp6388:
	.loc	15 1298 42 is_stmt 1
	leaq	.Lalloc_891dd683d7367e0a1ec5a22f9c2a2aa8(%rip), %rdx
	movq	120(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6389:
.LBB34_438:
	.loc	15 662 42
	leaq	.Lalloc_1781ea1b97b96e9885c590e9b4440a45(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp6390:
.LBB34_335:
	.loc	15 0 0 is_stmt 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6391:
.LBB34_439:
	.loc	15 452 44 is_stmt 1
	leaq	.Lalloc_6b076e9a9e313bc2481504f4da644a5c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB34_440:
	.loc	15 453 44
	leaq	.Lalloc_f370b9a38141751c9788be81259bacff(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp6392:
.Lfunc_end34:
	.size	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_, .Lfunc_end34-_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_
