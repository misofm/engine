_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb0_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_:
.Lfunc_begin32:
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
	subq	$440, %rsp
	.cfi_def_cfa_offset 496
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r10
	movq	%rsi, %r14
	movq	%rdi, 400(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%xmm0, 416(%rsp)
	movq	$0, 360(%rsp)
	movq	$0, 328(%rsp)
.Ltmp5027:
	.loc	6 997 31 prologue_end
	movq	32(%rdx), %rbx
	movq	40(%rdx), %rax
	.loc	6 997 49 is_stmt 0
	movq	80(%rdx), %rdx
	movl	$0, 208(%rsp)
	movl	$0, 216(%rsp)
	movl	$0, 224(%rsp)
	movl	$0, 232(%rsp)
	movl	$0, 240(%rsp)
	movl	$0, 248(%rsp)
	movl	$0, 256(%rsp)
	movl	$0, 264(%rsp)
.Ltmp5028:
	.loc	38 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5029:
	.loc	19 180 28
	je	.LBB32_1
.Ltmp5030:
	.loc	19 0 28 is_stmt 0
	leaq	240(%rsp), %rsi
	leaq	(%rax,%rax,4), %rax
	leaq	(%rbx,%rax,8), %rdi
	movl	92(%r14), %r9d
	movb	$1, %al
	movl	%eax, 40(%rsp)
	xorl	%r12d, %r12d
	leaq	208(%rsp), %r11
	xorl	%eax, %eax
	xorl	%r13d, %r13d
.LBB32_8:
	movq	%rax, %r8
	jmp	.LBB32_9
	.p2align	4
.LBB32_103:
	addq	$40, %rbx
.Ltmp5031:
	.loc	15 2428 13 is_stmt 1
	incq	%r8
	movq	$-1, %rax
	cmoveq	%rax, %r8
.Ltmp5032:
	.loc	34 82 9
	incq	%r13
	movq	%r8, %r12
.Ltmp5033:
	.loc	38 1714 9
	cmpq	%rdi, %rbx
.Ltmp5034:
	.loc	19 180 28
	je	.LBB32_2
.Ltmp5035:
.LBB32_9:
	.loc	6 613 33
	movl	32(%rbx), %eax
	.loc	6 613 27 is_stmt 0
	cmpl	$1, %eax
	je	.LBB32_10
	cmpl	$2, %eax
	jne	.LBB32_103
	.loc	6 0 27
	movl	$1, %eax
	movq	%rsi, %r15
	jmp	.LBB32_91
	.p2align	4
.LBB32_10:
	xorl	%eax, %eax
	movq	%r11, %r15
.LBB32_91:
.Ltmp5036:
	.loc	6 621 35 is_stmt 1
	movl	16(%rbx), %ebp
.Ltmp5037:
	.loc	15 3178 26
	testl	%ebp, %ebp
.Ltmp5038:
	.file	46 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/intrinsics/mod.rs"
	.loc	46 459 8
	js	.LBB32_103
.Ltmp5039:
	.loc	6 630 25
	cmpq	%r9, %r13
	jae	.LBB32_103
	cmpl	$3, %ebp
	ja	.LBB32_103
	.loc	6 632 20
	cmpl	$1, 28(%rbx)
	jne	.LBB32_103
	.loc	6 633 20
	cmpq	%rdx, (%rbx)
	jne	.LBB32_103
	.loc	6 634 20
	cmpq	%rdx, 8(%rbx)
	jne	.LBB32_103
	.loc	6 635 20
	vmovd	20(%rbx), %xmm0
.Ltmp5040:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5041:
	.loc	6 635 20
	cmpl	%ecx, 24(%rbx)
	jne	.LBB32_103
.Ltmp5042:
	.loc	6 0 0 is_stmt 0
	leal	(%rax,%rbp,2), %eax
	movl	%eax, 16(%rsp)
.Ltmp5043:
	.loc	6 636 42 is_stmt 1
	leaq	(,%rbp,4), %rax
	addq	%rbp, %rax
	movq	%rdi, 8(%rsp)
	leaq	.Lalloc_d0058eaee52f1ba8b2e5577cef0c3c7f(%rip), %rcx
	leaq	(%rcx,%rax,8), %rdi
	movq	%r8, 304(%rsp)
	movq	%r10, 32(%rsp)
	movq	%rdx, 64(%rsp)
	movq	%r9, 56(%rsp)
	vmovdqa	%xmm0, 128(%rsp)
	.loc	6 636 20 is_stmt 0
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	128(%rsp), %xmm1
	leaq	208(%rsp), %r11
	movq	56(%rsp), %r9
	movq	8(%rsp), %rdi
	leaq	240(%rsp), %rsi
	movq	64(%rsp), %rdx
	movq	32(%rsp), %r10
	movq	304(%rsp), %r8
	movl	16(%rsp), %ecx
	cmpl	48(%rsp), %ecx
	seta	%cl
	testb	%al, %al
	je	.LBB32_103
	orb	40(%rsp), %cl
	testb	$1, %cl
	je	.LBB32_103
.Ltmp5044:
	.loc	6 639 17 is_stmt 1
	cmpb	$0, (%r15,%rbp,8)
	jne	.LBB32_103
.Ltmp5045:
	.loc	12 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
.Ltmp5046:
	.loc	6 644 13
	movl	$1, (%r15,%rbp,8)
	vmovss	%xmm0, 4(%r15,%rbp,8)
.Ltmp5047:
	.loc	38 1714 9
	addq	$40, %rbx
.Ltmp5048:
	.loc	19 180 28
	incq	%r13
	movl	$0, 40(%rsp)
	movq	%r8, %rax
	movl	16(%rsp), %ecx
	movl	%ecx, 48(%rsp)
	movq	%r12, %r8
.Ltmp5049:
	.loc	38 1714 9
	cmpq	%rdi, %rbx
.Ltmp5050:
	.loc	19 180 28
	jne	.LBB32_8
	jmp	.LBB32_2
.Ltmp5051:
.LBB32_1:
	.loc	19 0 28 is_stmt 0
	xorl	%r8d, %r8d
.LBB32_2:
.Ltmp5052:
	.loc	6 648 21 is_stmt 1
	cmpl	$1, 208(%rsp)
	je	.LBB32_3
	cmpl	$1, 216(%rsp)
	je	.LBB32_64
.LBB32_72:
	cmpl	$1, 224(%rsp)
	je	.LBB32_73
.LBB32_81:
	cmpl	$1, 232(%rsp)
	je	.LBB32_82
.LBB32_11:
	cmpl	$1, 240(%rsp)
	je	.LBB32_12
.LBB32_20:
	cmpl	$1, 248(%rsp)
	je	.LBB32_21
.LBB32_29:
	cmpl	$1, 256(%rsp)
	je	.LBB32_30
.LBB32_38:
	cmpb	$0, 264(%rsp)
	je	.LBB32_39
.LBB32_51:
	.loc	6 648 26 is_stmt 0
	vmovd	268(%rsp), %xmm0
.Ltmp5053:
	.loc	6 651 39 is_stmt 1
	vmovd	916(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5054:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5055:
	.file	47 "/home/bl/misofm/engine-gate-detector-access" "crates/effect-runtime/src/ramp.rs"
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
	jne	.LBB32_53
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB32_53:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB32_54
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB32_57
	jmp	.LBB32_58
.Ltmp5056:
.LBB32_3:
	.loc	6 648 26 is_stmt 1
	vmovd	212(%rsp), %xmm0
.Ltmp5057:
	.loc	6 651 39
	vmovd	792(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5058:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5059:
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
	jne	.LBB32_5
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB32_5:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB32_6
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB32_61
	jmp	.LBB32_62
.LBB32_54:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB32_58
.Ltmp5060:
.LBB32_57:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_58:
.Ltmp5061:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 916(%r14)
.Ltmp5062:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 920(%r14)
.Ltmp5063:
	.loc	6 398 5
	vmovss	%xmm1, 924(%r14)
.Ltmp5064:
	.loc	6 398 5
	vmovss	%xmm3, 928(%r14)
.Ltmp5065:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r14)
	movl	$64, %ecx
.Ltmp5066:
	.loc	6 647 13
	jmp	.LBB32_40
.LBB32_6:
	.loc	6 0 13 is_stmt 0
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
.Ltmp5067:
	.loc	47 112 9 is_stmt 1
	je	.LBB32_62
.Ltmp5068:
.LBB32_61:
	.loc	47 0 9 is_stmt 0
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_62:
.Ltmp5069:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 792(%r14)
.Ltmp5070:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 796(%r14)
.Ltmp5071:
	.loc	6 398 5
	vmovss	%xmm1, 800(%r14)
.Ltmp5072:
	.loc	6 398 5
	vmovss	%xmm3, 804(%r14)
.Ltmp5073:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5074:
	.loc	6 648 21
	cmpl	$1, 216(%rsp)
	jne	.LBB32_72
.LBB32_64:
	.loc	6 648 26 is_stmt 0
	vmovd	220(%rsp), %xmm0
.Ltmp5075:
	.loc	6 651 39 is_stmt 1
	vmovd	808(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5076:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5077:
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
	jne	.LBB32_66
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB32_66:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB32_67
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB32_70
	jmp	.LBB32_71
.LBB32_67:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB32_71
.Ltmp5078:
.LBB32_70:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_71:
.Ltmp5079:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 808(%r14)
.Ltmp5080:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 812(%r14)
.Ltmp5081:
	.loc	6 398 5
	vmovss	%xmm1, 816(%r14)
.Ltmp5082:
	.loc	6 398 5
	vmovss	%xmm3, 820(%r14)
.Ltmp5083:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5084:
	.loc	6 648 21
	cmpl	$1, 224(%rsp)
	jne	.LBB32_81
.LBB32_73:
	.loc	6 648 26 is_stmt 0
	vmovd	228(%rsp), %xmm0
.Ltmp5085:
	.loc	6 651 39 is_stmt 1
	vmovd	824(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5086:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5087:
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
	jne	.LBB32_75
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB32_75:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB32_76
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB32_79
	jmp	.LBB32_80
.LBB32_76:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB32_80
.Ltmp5088:
.LBB32_79:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_80:
.Ltmp5089:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 824(%r14)
.Ltmp5090:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 828(%r14)
.Ltmp5091:
	.loc	6 398 5
	vmovss	%xmm1, 832(%r14)
.Ltmp5092:
	.loc	6 398 5
	vmovss	%xmm3, 836(%r14)
.Ltmp5093:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5094:
	.loc	6 648 21
	cmpl	$1, 232(%rsp)
	jne	.LBB32_11
.LBB32_82:
	.loc	6 648 26 is_stmt 0
	vmovd	236(%rsp), %xmm0
.Ltmp5095:
	.loc	6 651 39 is_stmt 1
	vmovd	840(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5096:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5097:
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
	jne	.LBB32_84
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB32_84:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB32_85
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB32_88
	jmp	.LBB32_89
.LBB32_85:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB32_89
.Ltmp5098:
.LBB32_88:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_89:
.Ltmp5099:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 840(%r14)
.Ltmp5100:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 844(%r14)
.Ltmp5101:
	.loc	6 398 5
	vmovss	%xmm1, 848(%r14)
.Ltmp5102:
	.loc	6 398 5
	vmovss	%xmm3, 852(%r14)
.Ltmp5103:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5104:
	.loc	6 648 21
	cmpl	$1, 240(%rsp)
	jne	.LBB32_20
.LBB32_12:
	.loc	6 648 26 is_stmt 0
	vmovd	244(%rsp), %xmm0
.Ltmp5105:
	.loc	6 651 39 is_stmt 1
	vmovd	868(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5106:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5107:
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
	jne	.LBB32_14
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB32_14:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB32_15
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB32_18
	jmp	.LBB32_19
.LBB32_15:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB32_19
.Ltmp5108:
.LBB32_18:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_19:
.Ltmp5109:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 868(%r14)
.Ltmp5110:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 872(%r14)
.Ltmp5111:
	.loc	6 398 5
	vmovss	%xmm1, 876(%r14)
.Ltmp5112:
	.loc	6 398 5
	vmovss	%xmm3, 880(%r14)
.Ltmp5113:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5114:
	.loc	6 648 21
	cmpl	$1, 248(%rsp)
	jne	.LBB32_29
.LBB32_21:
	.loc	6 648 26 is_stmt 0
	vmovd	252(%rsp), %xmm0
.Ltmp5115:
	.loc	6 651 39 is_stmt 1
	vmovd	884(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5116:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5117:
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
	jne	.LBB32_23
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB32_23:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB32_24
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB32_27
	jmp	.LBB32_28
.LBB32_24:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB32_28
.Ltmp5118:
.LBB32_27:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_28:
.Ltmp5119:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 884(%r14)
.Ltmp5120:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 888(%r14)
.Ltmp5121:
	.loc	6 398 5
	vmovss	%xmm1, 892(%r14)
.Ltmp5122:
	.loc	6 398 5
	vmovss	%xmm3, 896(%r14)
.Ltmp5123:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5124:
	.loc	6 648 21
	cmpl	$1, 256(%rsp)
	jne	.LBB32_38
.LBB32_30:
	.loc	6 648 26 is_stmt 0
	vmovd	260(%rsp), %xmm0
.Ltmp5125:
	.loc	6 651 39 is_stmt 1
	vmovd	900(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5126:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5127:
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
	jne	.LBB32_32
	.loc	47 0 9 is_stmt 0
	vmovdqa	%xmm0, %xmm2
.LBB32_32:
	vxorps	%xmm3, %xmm3, %xmm3
	.loc	47 112 9 is_stmt 1
	jne	.LBB32_33
	.loc	47 0 9 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	.loc	47 112 9
	jne	.LBB32_36
	jmp	.LBB32_37
.LBB32_33:
	.loc	47 0 9
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
	.loc	47 112 9
	je	.LBB32_37
.Ltmp5128:
.LBB32_36:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_37:
.Ltmp5129:
	.loc	6 398 5 is_stmt 1
	vmovd	%xmm2, 900(%r14)
.Ltmp5130:
	.loc	6 398 5 is_stmt 0
	vmovss	%xmm0, 904(%r14)
.Ltmp5131:
	.loc	6 398 5
	vmovss	%xmm1, 908(%r14)
.Ltmp5132:
	.loc	6 398 5
	vmovss	%xmm3, 912(%r14)
.Ltmp5133:
	.loc	6 661 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5134:
	.loc	6 648 21
	cmpb	$0, 264(%rsp)
	jne	.LBB32_51
.Ltmp5135:
.LBB32_39:
	.loc	6 682 25
	movl	1220(%r14), %ecx
.Ltmp5136:
.LBB32_40:
	.loc	20 1095 9
	movq	(%r10), %r9
	movq	8(%r10), %r11
.Ltmp5137:
	.loc	6 1000 36
	movq	16(%r10), %rax
	movq	%rax, 40(%rsp)
	movq	24(%r10), %rdx
.Ltmp5138:
	.loc	8 1078 5
	cmpq	%rcx, %r11
	movq	%rcx, %rsi
	cmovbq	%r11, %rsi
.Ltmp5139:
	.loc	6 683 12
	testq	%rsi, %rsi
	movq	%r8, 304(%rsp)
	movq	%r11, 72(%rsp)
	movq	%r9, 64(%rsp)
	movq	%rdx, 352(%rsp)
	je	.LBB32_41
.Ltmp5140:
	.loc	25 451 16
	cmpq	%rdx, %rsi
	ja	.LBB32_377
.Ltmp5141:
	.loc	25 0 16 is_stmt 0
	movq	%rcx, 168(%rsp)
.Ltmp5142:
	.loc	6 730 27 is_stmt 1
	movq	104(%r14), %r15
	movq	112(%r14), %rax
	.loc	6 735 27
	movq	168(%r14), %r11
	movq	176(%r14), %r10
	.loc	6 741 24
	movl	1212(%r14), %edx
	.loc	6 742 20
	movl	1216(%r14), %ecx
.Ltmp5143:
	.loc	21 238 16
	movl	1208(%r14), %ebx
	movl	136(%r14), %r8d
	movl	200(%r14), %edi
	vmovss	760(%r14), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	764(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	752(%r14), %xmm0
	vmovss	%xmm0, 8(%rsp)
	movl	744(%r14), %ebp
	movl	%ebp, 56(%rsp)
	movl	748(%r14), %ebp
	movl	%ebp, 128(%rsp)
	vmovss	756(%r14), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	784(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	788(%r14), %xmm0
	vmovss	%xmm0, 4(%rsp)
	vmovss	776(%r14), %xmm0
	vmovss	%xmm0, 192(%rsp)
	movl	768(%r14), %ebp
	movl	%ebp, 28(%rsp)
	movl	772(%r14), %ebp
	movl	%ebp, 24(%rsp)
	vmovss	780(%r14), %xmm0
	vmovss	%xmm0, 336(%rsp)
.Ltmp5144:
	.loc	25 451 16
	movl	%ebx, %r12d
	subl	%edi, %r12d
	cmpq	%r10, %rax
	jbe	.LBB32_106
	.loc	25 0 16 is_stmt 0
	movq	%r12, 88(%rsp)
	.loc	25 451 16
	movl	%ebx, %edi
	subl	%r8d, %edi
	movq	%rdi, 272(%rsp)
	movl	%ebx, %r12d
	subl	%ecx, %r12d
	xorl	%r13d, %r13d
	vmovss	.LCPI32_2(%rip), %xmm8
	vmovss	.LCPI32_3(%rip), %xmm9
	xorl	%ecx, %ecx
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB32_191
.Ltmp5145:
	.loc	25 0 16
.Ltmp5146:
	.p2align	4
.LBB32_274:
	.loc	21 392 36 is_stmt 1
	vmovss	940(%r14), %xmm5
.Ltmp5147:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	movl	28(%rsp), %edi
.Ltmp5148:
	.loc	26 161 24
	cmovbel	24(%rsp), %edi
.Ltmp5149:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp5150:
	.loc	26 66 9
	vsubss	%xmm5, %xmm2, %xmm2
.Ltmp5151:
	.loc	26 92 9
	vmulss	%xmm6, %xmm2, %xmm2
	vaddss	%xmm2, %xmm5, %xmm2
.Ltmp5152:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm5
	vandps	%xmm5, %xmm2, %xmm5
.Ltmp5153:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm5, %xmm5
	vandps	%xmm2, %xmm5, %xmm2
	vmovss	.LCPI32_23(%rip), %xmm6
.Ltmp5154:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm5
	vmovss	.LCPI32_24(%rip), %xmm7
.Ltmp5155:
	.loc	26 61 9
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp5156:
	.loc	26 71 9
	vmulss	%xmm5, %xmm12, %xmm5
	vmovss	.LCPI32_25(%rip), %xmm10
.Ltmp5157:
	.loc	26 61 9
	vaddss	%xmm5, %xmm10, %xmm5
.Ltmp5158:
	.loc	26 71 9
	vmulss	%xmm5, %xmm12, %xmm5
	vmovss	.LCPI32_26(%rip), %xmm11
.Ltmp5159:
	.loc	26 61 9
	vaddss	%xmm5, %xmm11, %xmm5
.Ltmp5160:
	.loc	26 71 9
	vmulss	%xmm5, %xmm12, %xmm5
	vmovss	.LCPI32_27(%rip), %xmm13
.Ltmp5161:
	.loc	26 61 9
	vaddss	%xmm5, %xmm13, %xmm5
.Ltmp5162:
	.loc	26 71 9
	vmulss	%xmm5, %xmm12, %xmm5
.Ltmp5163:
	.loc	26 61 9
	vaddss	%xmm5, %xmm8, %xmm5
	vmovss	.LCPI32_28(%rip), %xmm12
.Ltmp5164:
	.loc	26 178 22
	vaddss	%xmm3, %xmm12, %xmm3
.Ltmp5165:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5166:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5167:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5168:
	.loc	26 71 9
	vmulss	%xmm3, %xmm5, %xmm3
.Ltmp5169:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm14, %xmm5
.Ltmp5170:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI32_21(%rip), %xmm5, %xmm5
.Ltmp5171:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm5, %xmm5
.Ltmp5172:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp5173:
	.loc	26 161 24
	vcmpneqss	%xmm4, %xmm15, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm1, %xmm3
	vcmpnltss	48(%rsp), %xmm15, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm1, %xmm1
.Ltmp5174:
	.loc	7 1783 9
	vroundss	$9, %xmm5, %xmm5, %xmm3
.Ltmp5175:
	.loc	26 66 9
	vsubss	%xmm3, %xmm5, %xmm4
.Ltmp5176:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm5
.Ltmp5177:
	.loc	26 61 9
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp5178:
	.loc	26 71 9
	vmulss	%xmm5, %xmm4, %xmm5
.Ltmp5179:
	.loc	26 61 9
	vaddss	%xmm5, %xmm10, %xmm5
.Ltmp5180:
	.loc	26 71 9
	vmulss	%xmm5, %xmm4, %xmm5
.Ltmp5181:
	.loc	26 61 9
	vaddss	%xmm5, %xmm11, %xmm5
.Ltmp5182:
	.loc	26 71 9
	vmulss	%xmm5, %xmm4, %xmm5
.Ltmp5183:
	.loc	26 61 9
	vaddss	%xmm5, %xmm13, %xmm5
.Ltmp5184:
	.loc	26 71 9
	vmulss	%xmm5, %xmm4, %xmm4
.Ltmp5185:
	.loc	26 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp5186:
	.loc	26 178 22
	vaddss	%xmm3, %xmm12, %xmm3
.Ltmp5187:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5188:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5189:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5190:
	.loc	26 71 9
	vmulss	%xmm3, %xmm4, %xmm3
.Ltmp5191:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp5192:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm2, %xmm15, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm0, %xmm3
	vcmpnltss	336(%rsp), %xmm15, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm0, %xmm0
.Ltmp5193:
	.loc	21 394 5
	vmovss	%xmm2, 940(%r14)
.Ltmp5194:
	.loc	26 56 9
	vmovss	%xmm1, (%r9,%rcx,4)
	movq	40(%rsp), %rdi
.Ltmp5195:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm0, (%rdi,%rcx,4)
	movq	%rbp, %rcx
.Ltmp5196:
	.loc	8 1916 50 is_stmt 1
	cmpq	%rbp, %rsi
.Ltmp5197:
	.loc	11 900 12
	je	.LBB32_189
.Ltmp5198:
.LBB32_191:
	.loc	21 246 22
	leal	(%rbx,%rcx), %edi
	andl	%edx, %edi
.Ltmp5199:
	.loc	25 451 16
	cmpq	%rdi, %rax
	jbe	.LBB32_305
.Ltmp5200:
	.loc	26 51 9
	vmovss	(%r9,%rcx,4), %xmm0
.Ltmp5201:
	.loc	26 56 9
	vmovss	%xmm0, (%r15,%rdi,4)
.Ltmp5202:
	.loc	25 451 16
	cmpq	%rdi, %r10
	jbe	.LBB32_275
.Ltmp5203:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %r8
.Ltmp5204:
	.loc	26 51 9 is_stmt 1
	vmovss	(%r8,%rcx,4), %xmm0
.Ltmp5205:
	.loc	26 56 9
	vmovss	%xmm0, (%r11,%rdi,4)
.Ltmp5206:
	.loc	21 255 21
	leal	(%r12,%rcx), %edi
	andl	%edx, %edi
.Ltmp5207:
	.loc	25 438 16
	cmpq	%rdi, %rax
	jbe	.LBB32_381
.Ltmp5208:
	.loc	25 438 16 is_stmt 0
	cmpq	%rdi, %r10
	jbe	.LBB32_276
.Ltmp5209:
	.loc	25 0 16
	movq	272(%rsp), %r8
	addl	%ecx, %r8d
	andl	%edx, %r8d
	cmpq	%r8, %rax
	jbe	.LBB32_280
	cmpq	%r8, %r10
.Ltmp5210:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB32_279
.Ltmp5211:
	.loc	21 0 29 is_stmt 0
	movq	88(%rsp), %r9
	addl	%ecx, %r9d
	andl	%edx, %r9d
	cmpq	%r9, %r10
	jbe	.LBB32_278
.Ltmp5212:
	.loc	21 323 26 is_stmt 1
	vmovss	804(%r14), %xmm5
.Ltmp5213:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm5
	leaq	1(%rcx), %rbp
.Ltmp5214:
	.loc	21 325 44
	vmovss	800(%r14), %xmm6
	.loc	21 325 27 is_stmt 0
	vmovss	792(%r14), %xmm4
.Ltmp5215:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm4, %xmm0
.Ltmp5216:
	.loc	26 161 24
	jne	.LBB32_201
	jp	.LBB32_201
.Ltmp5217:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%r14), %xmm0
.LBB32_201:
.Ltmp5218:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_204
	jp	.LBB32_204
.Ltmp5219:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm6, %xmm6, %xmm6
.LBB32_204:
.Ltmp5220:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm5
.Ltmp5221:
	.loc	26 161 24
	jbe	.LBB32_206
.Ltmp5222:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm0, %xmm4
.LBB32_206:
	vmovss	(%r15,%rdi,4), %xmm1
	vmovss	(%r11,%rdi,4), %xmm0
.Ltmp5223:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r15,%r8,4), %xmm7
.Ltmp5224:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r11,%r8,4), %xmm12
.Ltmp5225:
	.loc	26 103 24
	vmovss	(%r11,%r9,4), %xmm2
	vmovaps	%xmm2, 288(%rsp)
.Ltmp5226:
	.loc	26 103 24
	vmovss	(%r15,%r9,4), %xmm3
.Ltmp5227:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm4, 792(%r14)
	.loc	21 331 13
	vmovss	%xmm6, 800(%r14)
.Ltmp5228:
	.loc	26 66 9
	vaddss	%xmm5, %xmm9, %xmm6
.Ltmp5229:
	.loc	26 161 24
	vcmpnltss	%xmm5, %xmm15, %xmm10
	vblendvps	%xmm10, %xmm5, %xmm6, %xmm5
.Ltmp5230:
	.loc	21 332 13
	vmovss	%xmm5, 804(%r14)
.Ltmp5231:
	.loc	21 323 26
	vmovss	820(%r14), %xmm6
.Ltmp5232:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm6
.Ltmp5233:
	.loc	21 325 27
	vmovss	808(%r14), %xmm5
	.loc	21 325 44 is_stmt 0
	vmovss	816(%r14), %xmm11
.Ltmp5234:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm5, %xmm11, %xmm13
.Ltmp5235:
	.loc	26 161 24
	jne	.LBB32_209
	jp	.LBB32_209
.Ltmp5236:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%r14), %xmm13
.LBB32_209:
.Ltmp5237:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_212
	jp	.LBB32_212
.Ltmp5238:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm11, %xmm11, %xmm11
.LBB32_212:
.Ltmp5239:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm6
.Ltmp5240:
	.loc	26 161 24
	jbe	.LBB32_214
.Ltmp5241:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm5
.LBB32_214:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm5, 808(%r14)
	.loc	21 331 13
	vmovss	%xmm11, 816(%r14)
.Ltmp5242:
	.loc	26 66 9
	vaddss	%xmm6, %xmm9, %xmm10
.Ltmp5243:
	.loc	26 161 24
	vcmpnltss	%xmm6, %xmm15, %xmm11
	vblendvps	%xmm11, %xmm6, %xmm10, %xmm6
.Ltmp5244:
	.loc	21 332 13
	vmovss	%xmm6, 820(%r14)
.Ltmp5245:
	.loc	21 323 26
	vmovss	836(%r14), %xmm11
.Ltmp5246:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm11
.Ltmp5247:
	.loc	21 325 44
	vmovss	832(%r14), %xmm13
	.loc	21 325 27 is_stmt 0
	vmovss	824(%r14), %xmm6
.Ltmp5248:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm13, %xmm14
.Ltmp5249:
	.loc	26 161 24
	jne	.LBB32_217
	jp	.LBB32_217
.Ltmp5250:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%r14), %xmm14
.LBB32_217:
.Ltmp5251:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_220
	jp	.LBB32_220
.Ltmp5252:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm13, %xmm13, %xmm13
.LBB32_220:
.Ltmp5253:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp5254:
	.loc	26 161 24
	jbe	.LBB32_222
.Ltmp5255:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm6
.LBB32_222:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm6, 824(%r14)
	.loc	21 331 13
	vmovss	%xmm13, 832(%r14)
.Ltmp5256:
	.loc	26 66 9
	vaddss	%xmm9, %xmm11, %xmm10
.Ltmp5257:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm13
	vblendvps	%xmm13, %xmm11, %xmm10, %xmm10
.Ltmp5258:
	.loc	21 332 13
	vmovss	%xmm10, 836(%r14)
.Ltmp5259:
	.loc	21 323 26
	vmovss	852(%r14), %xmm11
.Ltmp5260:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm11
.Ltmp5261:
	.loc	21 325 44
	vmovss	848(%r14), %xmm14
	.loc	21 325 27 is_stmt 0
	vmovss	840(%r14), %xmm13
.Ltmp5262:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm14, %xmm13, %xmm10
.Ltmp5263:
	.loc	26 161 24
	jne	.LBB32_225
	jp	.LBB32_225
.Ltmp5264:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%r14), %xmm10
.LBB32_225:
.Ltmp5265:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_228
	jp	.LBB32_228
.Ltmp5266:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm14, %xmm14, %xmm14
.LBB32_228:
.Ltmp5267:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp5268:
	.loc	26 161 24
	jbe	.LBB32_230
.Ltmp5269:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm10, %xmm13
.LBB32_230:
	vbroadcastss	.LCPI32_4(%rip), %xmm2
.Ltmp5270:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm2, %xmm7, %xmm10
.Ltmp5271:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm12, %xmm12
.Ltmp5272:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm12, %xmm10
.Ltmp5273:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp5274:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm12, %r8d
.Ltmp5275:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r8d
.Ltmp5276:
	.loc	26 66 9
	vaddss	%xmm9, %xmm11, %xmm7
.Ltmp5277:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm2
	vblendvps	%xmm2, %xmm11, %xmm7, %xmm7
	vmovss	16(%rsp), %xmm2
	vucomiss	%xmm15, %xmm2
.Ltmp5278:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r8d
	vmovss	.LCPI32_5(%rip), %xmm11
.Ltmp5279:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm11, %xmm10, %xmm2
.Ltmp5280:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm11, %xmm12, %xmm10
.Ltmp5281:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm10, %xmm2
	vmovss	32(%rsp), %xmm10
	vucomiss	%xmm15, %xmm10
.Ltmp5282:
	.loc	7 1244 18
	vmovd	%xmm2, %edi
.Ltmp5283:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp5284:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5285:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm2
.Ltmp5286:
	.loc	26 161 24
	movl	$841731191, %r8d
	cmovbel	%r8d, %edi
.Ltmp5287:
	.loc	21 326 13
	vmovss	%xmm13, 840(%r14)
.Ltmp5288:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5289:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm2
.Ltmp5290:
	.loc	21 331 13
	vmovss	%xmm14, 848(%r14)
.Ltmp5291:
	.loc	26 161 24
	movl	$8388608, %r8d
	cmovbel	%r8d, %edi
.Ltmp5292:
	.loc	26 185 42
	movl	%edi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp5293:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp5294:
	.loc	26 66 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp5295:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm2, %xmm10
.Ltmp5296:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm11
	vsubss	%xmm10, %xmm11, %xmm10
.Ltmp5297:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm10
.Ltmp5298:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm10, %xmm10
.Ltmp5299:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm10
.Ltmp5300:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm10, %xmm10
.Ltmp5301:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm10
.Ltmp5302:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm10, %xmm10
.Ltmp5303:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm10
.Ltmp5304:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm10, %xmm10
.Ltmp5305:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp5306:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm2
.Ltmp5307:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp5308:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm10, %xmm10
.Ltmp5309:
	.loc	26 61 9
	vaddss	%xmm2, %xmm10, %xmm2
.Ltmp5310:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm2, %xmm2
.Ltmp5311:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm2, %xmm2
.Ltmp5312:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm2, %xmm12
.Ltmp5313:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm4, %xmm2
.Ltmp5314:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm12
.Ltmp5315:
	.loc	26 28 5
	movl	$0, %r9d
	adcl	$-1, %r9d
.Ltmp5316:
	.loc	26 129 14
	vucomiss	%xmm4, %xmm12
.Ltmp5317:
	.loc	21 332 13
	vmovss	%xmm7, 852(%r14)
.Ltmp5318:
	.loc	26 144 9
	movl	$0, %r8d
	adcl	$-1, %r8d
.Ltmp5319:
	.loc	26 149 9
	movl	%r9d, %edi
.Ltmp5320:
	.loc	21 370 47
	vmovss	860(%r14), %xmm7
.Ltmp5321:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm7
.Ltmp5322:
	.loc	26 149 9
	notl	%edi
.Ltmp5323:
	.loc	26 139 9
	cmovbel	%r13d, %edi
.Ltmp5324:
	.loc	21 362 20
	vmovss	856(%r14), %xmm2
.Ltmp5325:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp5326:
	.loc	26 144 9
	cmoval	%r9d, %r8d
.Ltmp5327:
	.loc	26 139 9
	cmovbel	%r13d, %edi
.Ltmp5328:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB32_232
.Ltmp5329:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm7, %xmm9, %xmm7
.LBB32_232:
	vmovss	8(%rsp), %xmm10
.Ltmp5330:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r8b
	jne	.LBB32_234
.Ltmp5331:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm7, %xmm10
.LBB32_234:
	orl	%r8d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm2
.Ltmp5332:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm10, 860(%r14)
	.loc	21 382 5
	movl	%edi, 856(%r14)
.Ltmp5333:
	.loc	26 66 9
	vaddss	%xmm5, %xmm9, %xmm5
.Ltmp5334:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm4, %xmm12, %xmm4
.Ltmp5335:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm4, %xmm5, %xmm4
.Ltmp5336:
	.loc	26 98 24
	vbroadcastss	.LCPI32_18(%rip), %xmm5
	vxorps	%xmm5, %xmm6, %xmm5
.Ltmp5337:
	.loc	26 161 24
	vmaxss	%xmm5, %xmm4, %xmm4
.Ltmp5338:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm2, %xmm15, %xmm2
	vcmpltss	%xmm15, %xmm4, %xmm5
	vandps	%xmm5, %xmm2, %xmm2
	vmovd	%xmm2, %edi
	testb	$1, %dil
	jne	.LBB32_236
.Ltmp5339:
	.loc	26 0 44
	vxorps	%xmm4, %xmm4, %xmm4
.LBB32_236:
.Ltmp5340:
	.loc	21 392 36 is_stmt 1
	vmovss	864(%r14), %xmm2
.Ltmp5341:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm4
	movl	56(%rsp), %edi
.Ltmp5342:
	.loc	26 161 24
	cmovbel	128(%rsp), %edi
.Ltmp5343:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp5344:
	.loc	26 66 9
	vsubss	%xmm2, %xmm4, %xmm4
.Ltmp5345:
	.loc	26 92 9
	vmulss	%xmm5, %xmm4, %xmm4
	vaddss	%xmm4, %xmm2, %xmm2
.Ltmp5346:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm4
	vandps	%xmm4, %xmm2, %xmm4
.Ltmp5347:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm4, %xmm4
	vandps	%xmm2, %xmm4, %xmm4
.Ltmp5348:
	.loc	21 394 5
	vmovss	%xmm4, 864(%r14)
.Ltmp5349:
	.loc	21 323 26
	vmovss	880(%r14), %xmm6
.Ltmp5350:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm6
.Ltmp5351:
	.loc	21 325 44
	vmovss	876(%r14), %xmm7
	.loc	21 325 27 is_stmt 0
	vmovss	868(%r14), %xmm5
.Ltmp5352:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm7, %xmm5, %xmm10
.Ltmp5353:
	.loc	26 161 24
	jne	.LBB32_239
	jp	.LBB32_239
.Ltmp5354:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%r14), %xmm10
.LBB32_239:
.Ltmp5355:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_242
	jp	.LBB32_242
.Ltmp5356:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm7, %xmm7, %xmm7
.LBB32_242:
.Ltmp5357:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm6
.Ltmp5358:
	.loc	26 161 24
	jbe	.LBB32_244
.Ltmp5359:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm10, %xmm5
.LBB32_244:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm5, 868(%r14)
	.loc	21 331 13
	vmovss	%xmm7, 876(%r14)
.Ltmp5360:
	.loc	26 66 9
	vaddss	%xmm6, %xmm9, %xmm2
.Ltmp5361:
	.loc	26 161 24
	vcmpnltss	%xmm6, %xmm15, %xmm7
	vblendvps	%xmm7, %xmm6, %xmm2, %xmm2
.Ltmp5362:
	.loc	21 332 13
	vmovss	%xmm2, 880(%r14)
.Ltmp5363:
	.loc	21 323 26
	vmovss	896(%r14), %xmm7
.Ltmp5364:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm7
.Ltmp5365:
	.loc	21 325 44
	vmovss	892(%r14), %xmm11
	.loc	21 325 27 is_stmt 0
	vmovss	884(%r14), %xmm6
.Ltmp5366:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm11, %xmm10
.Ltmp5367:
	.loc	26 161 24
	jne	.LBB32_247
	jp	.LBB32_247
.Ltmp5368:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%r14), %xmm10
.LBB32_247:
.Ltmp5369:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_250
	jp	.LBB32_250
.Ltmp5370:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm11, %xmm11, %xmm11
.LBB32_250:
.Ltmp5371:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm7
.Ltmp5372:
	.loc	26 161 24
	jbe	.LBB32_252
.Ltmp5373:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm10, %xmm6
.LBB32_252:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm6, 884(%r14)
	.loc	21 331 13
	vmovss	%xmm11, 892(%r14)
.Ltmp5374:
	.loc	26 66 9
	vaddss	%xmm7, %xmm9, %xmm2
.Ltmp5375:
	.loc	26 161 24
	vcmpnltss	%xmm7, %xmm15, %xmm10
	vblendvps	%xmm10, %xmm7, %xmm2, %xmm2
.Ltmp5376:
	.loc	21 332 13
	vmovss	%xmm2, 896(%r14)
.Ltmp5377:
	.loc	21 323 26
	vmovss	912(%r14), %xmm11
.Ltmp5378:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm11
.Ltmp5379:
	.loc	21 325 44
	vmovss	908(%r14), %xmm12
	.loc	21 325 27 is_stmt 0
	vmovss	900(%r14), %xmm7
.Ltmp5380:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm7, %xmm12, %xmm10
.Ltmp5381:
	.loc	26 161 24
	jne	.LBB32_255
	jp	.LBB32_255
.Ltmp5382:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%r14), %xmm10
.LBB32_255:
.Ltmp5383:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_258
	jp	.LBB32_258
.Ltmp5384:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm12, %xmm12, %xmm12
.LBB32_258:
.Ltmp5385:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp5386:
	.loc	26 161 24
	jbe	.LBB32_260
.Ltmp5387:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm10, %xmm7
.LBB32_260:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm7, 900(%r14)
	.loc	21 331 13
	vmovss	%xmm12, 908(%r14)
.Ltmp5388:
	.loc	26 66 9
	vaddss	%xmm9, %xmm11, %xmm2
.Ltmp5389:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm10
	vblendvps	%xmm10, %xmm11, %xmm2, %xmm2
.Ltmp5390:
	.loc	21 332 13
	vmovss	%xmm2, 912(%r14)
.Ltmp5391:
	.loc	21 323 26
	vmovss	928(%r14), %xmm11
.Ltmp5392:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm11
.Ltmp5393:
	.loc	21 325 44
	vmovss	924(%r14), %xmm13
	.loc	21 325 27 is_stmt 0
	vmovss	916(%r14), %xmm12
.Ltmp5394:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm13, %xmm12, %xmm10
.Ltmp5395:
	.loc	26 161 24
	jne	.LBB32_263
	jp	.LBB32_263
.Ltmp5396:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%r14), %xmm10
.LBB32_263:
.Ltmp5397:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_266
	jp	.LBB32_266
.Ltmp5398:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm13, %xmm13, %xmm13
.LBB32_266:
.Ltmp5399:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp5400:
	.loc	26 161 24
	jbe	.LBB32_268
.Ltmp5401:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm10, %xmm12
.LBB32_268:
	vbroadcastss	.LCPI32_4(%rip), %xmm2
.Ltmp5402:
	.loc	26 103 24 is_stmt 1
	vandps	288(%rsp), %xmm2, %xmm10
.Ltmp5403:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm3, %xmm3
.Ltmp5404:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm3, %xmm10
.Ltmp5405:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp5406:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm3, %r8d
.Ltmp5407:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r8d
.Ltmp5408:
	.loc	26 66 9
	vaddss	%xmm9, %xmm11, %xmm2
.Ltmp5409:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm14
	vblendvps	%xmm14, %xmm11, %xmm2, %xmm2
	vmovss	80(%rsp), %xmm11
	vucomiss	%xmm15, %xmm11
.Ltmp5410:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r8d
	vmovss	.LCPI32_5(%rip), %xmm11
.Ltmp5411:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm11, %xmm10, %xmm10
.Ltmp5412:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp5413:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm3
	vmovss	4(%rsp), %xmm10
	vucomiss	%xmm15, %xmm10
.Ltmp5414:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5415:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp5416:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5417:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm3
.Ltmp5418:
	.loc	26 161 24
	movl	$841731191, %r8d
	cmovbel	%r8d, %edi
.Ltmp5419:
	.loc	21 326 13
	vmovss	%xmm12, 916(%r14)
.Ltmp5420:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5421:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm3
.Ltmp5422:
	.loc	21 331 13
	vmovss	%xmm13, 924(%r14)
.Ltmp5423:
	.loc	26 161 24
	movl	$8388608, %r8d
	cmovbel	%r8d, %edi
.Ltmp5424:
	.loc	26 185 42
	movl	%edi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp5425:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp5426:
	.loc	26 66 9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp5427:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm3, %xmm10
.Ltmp5428:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm11
	vsubss	%xmm10, %xmm11, %xmm10
.Ltmp5429:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm10
.Ltmp5430:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm10, %xmm10
.Ltmp5431:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm10
.Ltmp5432:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm10, %xmm10
.Ltmp5433:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm10
.Ltmp5434:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm10, %xmm10
.Ltmp5435:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm10
.Ltmp5436:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm10, %xmm10
.Ltmp5437:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp5438:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm3
.Ltmp5439:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp5440:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm10, %xmm10
.Ltmp5441:
	.loc	26 61 9
	vaddss	%xmm3, %xmm10, %xmm3
.Ltmp5442:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp5443:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
.Ltmp5444:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm3, %xmm13
.Ltmp5445:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm12, %xmm5, %xmm3
.Ltmp5446:
	.loc	26 129 14
	vucomiss	%xmm3, %xmm13
.Ltmp5447:
	.loc	26 28 5
	movl	$0, %r9d
	adcl	$-1, %r9d
.Ltmp5448:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm13
.Ltmp5449:
	.loc	21 332 13
	vmovss	%xmm2, 928(%r14)
.Ltmp5450:
	.loc	26 144 9
	movl	$0, %r8d
	adcl	$-1, %r8d
.Ltmp5451:
	.loc	26 149 9
	movl	%r9d, %edi
.Ltmp5452:
	.loc	21 370 47
	vmovss	936(%r14), %xmm2
.Ltmp5453:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp5454:
	.loc	26 149 9
	notl	%edi
.Ltmp5455:
	.loc	26 139 9
	cmovbel	%r13d, %edi
.Ltmp5456:
	.loc	21 362 20
	vmovss	932(%r14), %xmm3
.Ltmp5457:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm3
.Ltmp5458:
	.loc	26 144 9
	cmoval	%r9d, %r8d
.Ltmp5459:
	.loc	26 139 9
	cmovbel	%r13d, %edi
.Ltmp5460:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB32_270
.Ltmp5461:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm2, %xmm9, %xmm2
.LBB32_270:
	movq	64(%rsp), %r9
	vmovss	.LCPI32_20(%rip), %xmm14
	vmovss	192(%rsp), %xmm10
.Ltmp5462:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r8b
	jne	.LBB32_272
.Ltmp5463:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB32_272:
.Ltmp5464:
	vmulss	%xmm4, %xmm14, %xmm2
	vmaxss	.LCPI32_21(%rip), %xmm2, %xmm2
	vminss	.LCPI32_22(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm3
	vsubss	%xmm3, %xmm2, %xmm12
.Ltmp5465:
	orl	%r8d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm11
.Ltmp5466:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm10, 936(%r14)
	.loc	21 382 5
	movl	%edi, 932(%r14)
.Ltmp5467:
	.loc	26 66 9
	vaddss	%xmm6, %xmm9, %xmm2
.Ltmp5468:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm5, %xmm13, %xmm5
.Ltmp5469:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm2, %xmm5, %xmm2
.Ltmp5470:
	.loc	26 98 24
	vbroadcastss	.LCPI32_18(%rip), %xmm5
	vxorps	%xmm5, %xmm7, %xmm5
.Ltmp5471:
	.loc	26 161 24
	vmaxss	%xmm5, %xmm2, %xmm2
.Ltmp5472:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm11, %xmm15, %xmm5
	vcmpltss	%xmm15, %xmm2, %xmm6
	vandps	%xmm6, %xmm5, %xmm5
	vmovd	%xmm5, %edi
	testb	$1, %dil
	jne	.LBB32_274
.Ltmp5473:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB32_274
.LBB32_106:
.Ltmp5474:
	.loc	25 451 16 is_stmt 1
	movl	%ebx, %edi
	subl	%r8d, %edi
	movq	%rdi, 176(%rsp)
	movl	%ebx, %edi
	subl	%ecx, %edi
	movq	%rdi, 88(%rsp)
	xorl	%r13d, %r13d
	vmovss	.LCPI32_2(%rip), %xmm8
	vmovss	.LCPI32_3(%rip), %xmm9
	vbroadcastss	.LCPI32_4(%rip), %xmm14
	xorl	%ebp, %ebp
	vxorps	%xmm6, %xmm6, %xmm6
	jmp	.LBB32_107
.Ltmp5475:
	.loc	25 0 16 is_stmt 0
.Ltmp5476:
	.p2align	4
.LBB32_188:
	.loc	21 392 36 is_stmt 1
	vmovss	940(%r14), %xmm3
.Ltmp5477:
	.loc	26 124 14
	vucomiss	%xmm3, %xmm1
	movl	28(%rsp), %edi
.Ltmp5478:
	.loc	26 161 24
	cmovbel	24(%rsp), %edi
.Ltmp5479:
	.loc	7 1291 18
	vmovd	%edi, %xmm4
.Ltmp5480:
	.loc	26 66 9
	vsubss	%xmm3, %xmm1, %xmm1
.Ltmp5481:
	.loc	26 92 9
	vmulss	%xmm4, %xmm1, %xmm1
	vaddss	%xmm1, %xmm3, %xmm1
	vmovaps	%xmm12, %xmm14
.Ltmp5482:
	.loc	26 103 24
	vandps	%xmm1, %xmm12, %xmm3
.Ltmp5483:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm3, %xmm3
	vandps	%xmm1, %xmm3, %xmm1
	vmovss	.LCPI32_23(%rip), %xmm4
.Ltmp5484:
	.loc	26 71 9
	vmulss	%xmm4, %xmm5, %xmm3
	vmovss	.LCPI32_24(%rip), %xmm7
.Ltmp5485:
	.loc	26 61 9
	vaddss	%xmm7, %xmm3, %xmm3
.Ltmp5486:
	.loc	26 71 9
	vmulss	%xmm3, %xmm5, %xmm3
	vmovss	.LCPI32_25(%rip), %xmm10
.Ltmp5487:
	.loc	26 61 9
	vaddss	%xmm3, %xmm10, %xmm3
.Ltmp5488:
	.loc	26 71 9
	vmulss	%xmm3, %xmm5, %xmm3
	vmovss	.LCPI32_26(%rip), %xmm12
.Ltmp5489:
	.loc	26 61 9
	vaddss	%xmm3, %xmm12, %xmm3
.Ltmp5490:
	.loc	26 71 9
	vmulss	%xmm3, %xmm5, %xmm3
	vmovss	.LCPI32_27(%rip), %xmm13
.Ltmp5491:
	.loc	26 61 9
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp5492:
	.loc	26 71 9
	vmulss	%xmm3, %xmm5, %xmm3
.Ltmp5493:
	.loc	26 61 9
	vaddss	%xmm3, %xmm8, %xmm3
	vmovss	.LCPI32_28(%rip), %xmm5
.Ltmp5494:
	.loc	26 178 22
	vaddss	%xmm5, %xmm2, %xmm2
.Ltmp5495:
	.loc	7 1244 18
	vmovd	%xmm2, %edi
.Ltmp5496:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5497:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5498:
	.loc	26 71 9
	vmulss	%xmm2, %xmm3, %xmm2
.Ltmp5499:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm11, %xmm3
.Ltmp5500:
	.loc	26 161 24 is_stmt 1
	vmaxss	%xmm15, %xmm3, %xmm3
.Ltmp5501:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm3, %xmm3
	vmovaps	272(%rsp), %xmm11
.Ltmp5502:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm2, %xmm11, %xmm2
.Ltmp5503:
	.loc	26 161 24
	vcmpneqss	%xmm6, %xmm0, %xmm0
	vblendvps	%xmm0, %xmm2, %xmm11, %xmm0
	vcmpnltss	48(%rsp), %xmm6, %xmm2
	vblendvps	%xmm2, %xmm0, %xmm11, %xmm0
.Ltmp5504:
	.loc	7 1783 9
	vroundss	$9, %xmm3, %xmm3, %xmm2
.Ltmp5505:
	.loc	26 66 9
	vsubss	%xmm2, %xmm3, %xmm3
.Ltmp5506:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm4
.Ltmp5507:
	.loc	26 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp5508:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm4
.Ltmp5509:
	.loc	26 61 9
	vaddss	%xmm4, %xmm10, %xmm4
.Ltmp5510:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm4
.Ltmp5511:
	.loc	26 61 9
	vaddss	%xmm4, %xmm12, %xmm4
.Ltmp5512:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm4
.Ltmp5513:
	.loc	26 61 9
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp5514:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp5515:
	.loc	26 61 9
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp5516:
	.loc	26 178 22
	vaddss	%xmm5, %xmm2, %xmm2
.Ltmp5517:
	.loc	7 1244 18
	vmovd	%xmm2, %edi
.Ltmp5518:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5519:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5520:
	.loc	26 71 9
	vmulss	%xmm2, %xmm3, %xmm2
	vmovaps	288(%rsp), %xmm4
.Ltmp5521:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm4, %xmm2
.Ltmp5522:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm6, %xmm1, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
	vcmpnltss	336(%rsp), %xmm6, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
.Ltmp5523:
	.loc	21 394 5
	vmovss	%xmm1, 940(%r14)
.Ltmp5524:
	.loc	26 56 9
	vmovss	%xmm0, (%r9,%rbp,4)
	movq	40(%rsp), %rdi
.Ltmp5525:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm2, (%rdi,%rbp,4)
	movq	%rcx, %rbp
.Ltmp5526:
	.loc	8 1916 50 is_stmt 1
	cmpq	%rcx, %rsi
.Ltmp5527:
	.loc	11 900 12
	je	.LBB32_189
.Ltmp5528:
.LBB32_107:
	.loc	21 246 22
	leal	(%rbx,%rbp), %edi
	andl	%edx, %edi
.Ltmp5529:
	.loc	25 451 16
	cmpq	%rdi, %rax
	jbe	.LBB32_305
.Ltmp5530:
	.loc	26 51 9
	vmovss	(%r9,%rbp,4), %xmm0
.Ltmp5531:
	.loc	26 56 9
	vmovss	%xmm0, (%r15,%rdi,4)
	movq	40(%rsp), %rcx
.Ltmp5532:
	.loc	26 51 9
	vmovss	(%rcx,%rbp,4), %xmm0
.Ltmp5533:
	.loc	26 56 9
	vmovss	%xmm0, (%r11,%rdi,4)
	movq	88(%rsp), %rcx
.Ltmp5534:
	.loc	21 255 21
	leal	(%rcx,%rbp), %edi
	andl	%edx, %edi
.Ltmp5535:
	.loc	25 438 16
	cmpq	%rdi, %rax
	jbe	.LBB32_381
.Ltmp5536:
	.loc	25 0 16 is_stmt 0
	movq	176(%rsp), %rcx
	leal	(%rcx,%rbp), %r8d
	andl	%edx, %r8d
	cmpq	%r8, %rax
	jbe	.LBB32_280
	leal	(%r12,%rbp), %r9d
	andl	%edx, %r9d
	cmpq	%r9, %r10
	jbe	.LBB32_278
	cmpq	%r9, %rax
.Ltmp5537:
	.loc	21 277 29 is_stmt 1
	jbe	.LBB32_277
.Ltmp5538:
	.loc	21 323 26
	vmovss	804(%r14), %xmm1
.Ltmp5539:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm1
	leaq	1(%rbp), %rcx
.Ltmp5540:
	.loc	21 325 44
	vmovss	800(%r14), %xmm2
	.loc	21 325 27 is_stmt 0
	vmovss	792(%r14), %xmm0
.Ltmp5541:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm0, %xmm3
.Ltmp5542:
	.loc	26 161 24
	jne	.LBB32_115
	jp	.LBB32_115
.Ltmp5543:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%r14), %xmm3
.LBB32_115:
.Ltmp5544:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_118
	jp	.LBB32_118
.Ltmp5545:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm2, %xmm2, %xmm2
.LBB32_118:
.Ltmp5546:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm1
.Ltmp5547:
	.loc	26 161 24
	jbe	.LBB32_120
.Ltmp5548:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm0
.LBB32_120:
	vmovss	(%r15,%rdi,4), %xmm3
	vmovaps	%xmm3, 272(%rsp)
	vmovss	(%r11,%rdi,4), %xmm3
	vmovaps	%xmm3, 288(%rsp)
	vmovss	(%r11,%r8,4), %xmm4
.Ltmp5549:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r15,%r8,4), %xmm5
.Ltmp5550:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r11,%r9,4), %xmm13
.Ltmp5551:
	.loc	26 103 24
	vmovss	(%r15,%r9,4), %xmm15
.Ltmp5552:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm0, 792(%r14)
	.loc	21 331 13
	vmovss	%xmm2, 800(%r14)
.Ltmp5553:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp5554:
	.loc	26 161 24
	vcmpnltss	%xmm1, %xmm6, %xmm3
	vblendvps	%xmm3, %xmm1, %xmm2, %xmm1
.Ltmp5555:
	.loc	21 332 13
	vmovss	%xmm1, 804(%r14)
.Ltmp5556:
	.loc	21 323 26
	vmovss	820(%r14), %xmm2
.Ltmp5557:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm2
.Ltmp5558:
	.loc	21 325 27
	vmovss	808(%r14), %xmm1
	.loc	21 325 44 is_stmt 0
	vmovss	816(%r14), %xmm3
.Ltmp5559:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm1, %xmm7
.Ltmp5560:
	.loc	26 161 24
	jne	.LBB32_123
	jp	.LBB32_123
.Ltmp5561:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%r14), %xmm7
.LBB32_123:
.Ltmp5562:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_126
	jp	.LBB32_126
.Ltmp5563:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_126:
.Ltmp5564:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm2
.Ltmp5565:
	.loc	26 161 24
	jbe	.LBB32_128
.Ltmp5566:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm7, %xmm1
.LBB32_128:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm1, 808(%r14)
	.loc	21 331 13
	vmovss	%xmm3, 816(%r14)
.Ltmp5567:
	.loc	26 66 9
	vaddss	%xmm2, %xmm9, %xmm3
.Ltmp5568:
	.loc	26 161 24
	vcmpnltss	%xmm2, %xmm6, %xmm7
	vblendvps	%xmm7, %xmm2, %xmm3, %xmm2
.Ltmp5569:
	.loc	21 332 13
	vmovss	%xmm2, 820(%r14)
.Ltmp5570:
	.loc	21 323 26
	vmovss	836(%r14), %xmm2
.Ltmp5571:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm2
.Ltmp5572:
	.loc	21 325 44
	vmovss	832(%r14), %xmm7
	.loc	21 325 27 is_stmt 0
	vmovss	824(%r14), %xmm3
.Ltmp5573:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm7, %xmm3, %xmm11
.Ltmp5574:
	.loc	26 161 24
	jne	.LBB32_131
	jp	.LBB32_131
.Ltmp5575:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%r14), %xmm11
.LBB32_131:
.Ltmp5576:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_134
	jp	.LBB32_134
.Ltmp5577:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm7, %xmm7, %xmm7
.LBB32_134:
.Ltmp5578:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm2
.Ltmp5579:
	.loc	26 161 24
	jbe	.LBB32_136
.Ltmp5580:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm3
.LBB32_136:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm3, 824(%r14)
	.loc	21 331 13
	vmovss	%xmm7, 832(%r14)
.Ltmp5581:
	.loc	26 66 9
	vaddss	%xmm2, %xmm9, %xmm7
.Ltmp5582:
	.loc	26 161 24
	vcmpnltss	%xmm2, %xmm6, %xmm10
	vblendvps	%xmm10, %xmm2, %xmm7, %xmm2
.Ltmp5583:
	.loc	21 332 13
	vmovss	%xmm2, 836(%r14)
.Ltmp5584:
	.loc	21 323 26
	vmovss	852(%r14), %xmm11
.Ltmp5585:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm11
.Ltmp5586:
	.loc	21 325 44
	vmovss	848(%r14), %xmm2
	.loc	21 325 27 is_stmt 0
	vmovss	840(%r14), %xmm7
.Ltmp5587:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm7, %xmm10
.Ltmp5588:
	.loc	26 161 24
	jne	.LBB32_139
	jp	.LBB32_139
.Ltmp5589:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%r14), %xmm10
.LBB32_139:
.Ltmp5590:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_142
	jp	.LBB32_142
.Ltmp5591:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm2, %xmm2, %xmm2
.LBB32_142:
.Ltmp5592:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm11
.Ltmp5593:
	.loc	26 161 24
	jbe	.LBB32_144
.Ltmp5594:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm10, %xmm7
.LBB32_144:
.Ltmp5595:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm5, %xmm14, %xmm5
	vmovaps	%xmm14, %xmm12
.Ltmp5596:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm4, %xmm14, %xmm10
.Ltmp5597:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm5
.Ltmp5598:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp5599:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm10, %r8d
.Ltmp5600:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r8d
.Ltmp5601:
	.loc	26 66 9
	vaddss	%xmm9, %xmm11, %xmm4
.Ltmp5602:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm6, %xmm14
	vblendvps	%xmm14, %xmm11, %xmm4, %xmm4
	vmovss	16(%rsp), %xmm11
	vucomiss	%xmm6, %xmm11
.Ltmp5603:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r8d
	vmovss	.LCPI32_5(%rip), %xmm11
.Ltmp5604:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm11, %xmm5
.Ltmp5605:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm11, %xmm10, %xmm10
.Ltmp5606:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm5, %xmm10, %xmm5
	vmovss	32(%rsp), %xmm10
	vucomiss	%xmm6, %xmm10
.Ltmp5607:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp5608:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp5609:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp5610:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm5
.Ltmp5611:
	.loc	26 161 24
	movl	$841731191, %r8d
	cmovbel	%r8d, %edi
.Ltmp5612:
	.loc	21 326 13
	vmovss	%xmm7, 840(%r14)
.Ltmp5613:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp5614:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm5
.Ltmp5615:
	.loc	21 331 13
	vmovss	%xmm2, 848(%r14)
.Ltmp5616:
	.loc	26 161 24
	movl	$8388608, %r8d
	cmovbel	%r8d, %edi
.Ltmp5617:
	.loc	26 185 42
	movl	%edi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp5618:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp5619:
	.loc	26 66 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp5620:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm2, %xmm5
.Ltmp5621:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm10
	vsubss	%xmm5, %xmm10, %xmm5
.Ltmp5622:
	.loc	26 71 9
	vmulss	%xmm5, %xmm2, %xmm5
.Ltmp5623:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm5, %xmm5
.Ltmp5624:
	.loc	26 71 9
	vmulss	%xmm5, %xmm2, %xmm5
.Ltmp5625:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm5, %xmm5
.Ltmp5626:
	.loc	26 71 9
	vmulss	%xmm5, %xmm2, %xmm5
.Ltmp5627:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm5, %xmm5
.Ltmp5628:
	.loc	26 71 9
	vmulss	%xmm5, %xmm2, %xmm5
.Ltmp5629:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm5, %xmm5
.Ltmp5630:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp5631:
	.loc	26 71 9
	vmulss	%xmm5, %xmm2, %xmm2
.Ltmp5632:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp5633:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm5, %xmm5
.Ltmp5634:
	.loc	26 61 9
	vaddss	%xmm2, %xmm5, %xmm2
.Ltmp5635:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm2, %xmm2
.Ltmp5636:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm2, %xmm2
.Ltmp5637:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm2, %xmm2
.Ltmp5638:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm7, %xmm0, %xmm5
.Ltmp5639:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm2
.Ltmp5640:
	.loc	26 28 5
	movl	$0, %r9d
	adcl	$-1, %r9d
.Ltmp5641:
	.loc	26 129 14
	vucomiss	%xmm0, %xmm2
.Ltmp5642:
	.loc	21 332 13
	vmovss	%xmm4, 852(%r14)
.Ltmp5643:
	.loc	26 144 9
	movl	$0, %r8d
	adcl	$-1, %r8d
.Ltmp5644:
	.loc	26 149 9
	movl	%r9d, %edi
.Ltmp5645:
	.loc	21 370 47
	vmovss	860(%r14), %xmm4
.Ltmp5646:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm4
.Ltmp5647:
	.loc	26 149 9
	notl	%edi
.Ltmp5648:
	.loc	26 139 9
	cmovbel	%r13d, %edi
.Ltmp5649:
	.loc	21 362 20
	vmovss	856(%r14), %xmm5
.Ltmp5650:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm5
.Ltmp5651:
	.loc	26 144 9
	cmoval	%r9d, %r8d
.Ltmp5652:
	.loc	26 139 9
	cmovbel	%r13d, %edi
.Ltmp5653:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB32_146
.Ltmp5654:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm4, %xmm9, %xmm4
.LBB32_146:
	vmovss	8(%rsp), %xmm5
.Ltmp5655:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r8b
	vmovaps	%xmm12, %xmm11
	jne	.LBB32_148
.Ltmp5656:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm5
.LBB32_148:
	orl	%r8d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm4
.Ltmp5657:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm5, 860(%r14)
	.loc	21 382 5
	movl	%edi, 856(%r14)
.Ltmp5658:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm1
.Ltmp5659:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm0, %xmm2, %xmm0
.Ltmp5660:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp5661:
	.loc	26 98 24
	vbroadcastss	.LCPI32_18(%rip), %xmm1
	vxorps	%xmm1, %xmm3, %xmm1
.Ltmp5662:
	.loc	26 161 24
	vmaxss	%xmm1, %xmm0, %xmm0
.Ltmp5663:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm4, %xmm6, %xmm1
	vcmpltss	%xmm6, %xmm0, %xmm2
	vandps	%xmm2, %xmm1, %xmm1
	vmovd	%xmm1, %edi
	testb	$1, %dil
	jne	.LBB32_150
.Ltmp5664:
	.loc	26 0 44
	vxorps	%xmm0, %xmm0, %xmm0
.LBB32_150:
.Ltmp5665:
	.loc	21 392 36 is_stmt 1
	vmovss	864(%r14), %xmm1
.Ltmp5666:
	.loc	26 124 14
	vucomiss	%xmm1, %xmm0
	movl	56(%rsp), %edi
.Ltmp5667:
	.loc	26 161 24
	cmovbel	128(%rsp), %edi
.Ltmp5668:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5669:
	.loc	26 66 9
	vsubss	%xmm1, %xmm0, %xmm0
.Ltmp5670:
	.loc	26 92 9
	vmulss	%xmm2, %xmm0, %xmm0
	vaddss	%xmm0, %xmm1, %xmm0
.Ltmp5671:
	.loc	26 103 24
	vandps	%xmm0, %xmm11, %xmm1
.Ltmp5672:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm1, %xmm1
	vandps	%xmm0, %xmm1, %xmm0
.Ltmp5673:
	.loc	21 394 5
	vmovss	%xmm0, 864(%r14)
.Ltmp5674:
	.loc	21 323 26
	vmovss	880(%r14), %xmm2
.Ltmp5675:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm2
.Ltmp5676:
	.loc	21 325 44
	vmovss	876(%r14), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	868(%r14), %xmm1
.Ltmp5677:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm1, %xmm4
.Ltmp5678:
	.loc	26 161 24
	jne	.LBB32_153
	jp	.LBB32_153
.Ltmp5679:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%r14), %xmm4
.LBB32_153:
.Ltmp5680:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_156
	jp	.LBB32_156
.Ltmp5681:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_156:
.Ltmp5682:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm2
.Ltmp5683:
	.loc	26 161 24
	jbe	.LBB32_158
.Ltmp5684:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm1
.LBB32_158:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm1, 868(%r14)
	.loc	21 331 13
	vmovss	%xmm3, 876(%r14)
.Ltmp5685:
	.loc	26 66 9
	vaddss	%xmm2, %xmm9, %xmm3
.Ltmp5686:
	.loc	26 161 24
	vcmpnltss	%xmm2, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm2, %xmm3, %xmm2
.Ltmp5687:
	.loc	21 332 13
	vmovss	%xmm2, 880(%r14)
.Ltmp5688:
	.loc	21 323 26
	vmovss	896(%r14), %xmm2
.Ltmp5689:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm2
.Ltmp5690:
	.loc	21 325 44
	vmovss	892(%r14), %xmm4
	.loc	21 325 27 is_stmt 0
	vmovss	884(%r14), %xmm3
.Ltmp5691:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm3, %xmm5
.Ltmp5692:
	.loc	26 161 24
	jne	.LBB32_161
	jp	.LBB32_161
.Ltmp5693:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%r14), %xmm5
.LBB32_161:
.Ltmp5694:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_164
	jp	.LBB32_164
.Ltmp5695:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm4, %xmm4, %xmm4
.LBB32_164:
.Ltmp5696:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm2
.Ltmp5697:
	.loc	26 161 24
	jbe	.LBB32_166
.Ltmp5698:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm5, %xmm3
.LBB32_166:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm3, 884(%r14)
	.loc	21 331 13
	vmovss	%xmm4, 892(%r14)
.Ltmp5699:
	.loc	26 66 9
	vaddss	%xmm2, %xmm9, %xmm4
.Ltmp5700:
	.loc	26 161 24
	vcmpnltss	%xmm2, %xmm6, %xmm5
	vblendvps	%xmm5, %xmm2, %xmm4, %xmm2
.Ltmp5701:
	.loc	21 332 13
	vmovss	%xmm2, 896(%r14)
.Ltmp5702:
	.loc	21 323 26
	vmovss	912(%r14), %xmm2
.Ltmp5703:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm2
.Ltmp5704:
	.loc	21 325 44
	vmovss	908(%r14), %xmm5
	.loc	21 325 27 is_stmt 0
	vmovss	900(%r14), %xmm4
.Ltmp5705:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm5, %xmm4, %xmm7
.Ltmp5706:
	.loc	26 161 24
	jne	.LBB32_169
	jp	.LBB32_169
.Ltmp5707:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%r14), %xmm7
.LBB32_169:
.Ltmp5708:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_172
	jp	.LBB32_172
.Ltmp5709:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm5, %xmm5, %xmm5
.LBB32_172:
.Ltmp5710:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm2
.Ltmp5711:
	.loc	26 161 24
	jbe	.LBB32_174
.Ltmp5712:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm7, %xmm4
.LBB32_174:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm4, 900(%r14)
	.loc	21 331 13
	vmovss	%xmm5, 908(%r14)
.Ltmp5713:
	.loc	26 66 9
	vaddss	%xmm2, %xmm9, %xmm5
.Ltmp5714:
	.loc	26 161 24
	vcmpnltss	%xmm2, %xmm6, %xmm7
	vblendvps	%xmm7, %xmm2, %xmm5, %xmm2
.Ltmp5715:
	.loc	21 332 13
	vmovss	%xmm2, 912(%r14)
.Ltmp5716:
	.loc	21 323 26
	vmovss	928(%r14), %xmm7
.Ltmp5717:
	.loc	26 134 14
	vucomiss	%xmm8, %xmm7
.Ltmp5718:
	.loc	21 325 44
	vmovss	924(%r14), %xmm2
	.loc	21 325 27 is_stmt 0
	vmovss	916(%r14), %xmm5
.Ltmp5719:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm5, %xmm10
.Ltmp5720:
	.loc	26 161 24
	jne	.LBB32_177
	jp	.LBB32_177
.Ltmp5721:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%r14), %xmm10
.LBB32_177:
.Ltmp5722:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_180
	jp	.LBB32_180
.Ltmp5723:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm2, %xmm2, %xmm2
.LBB32_180:
.Ltmp5724:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm7
.Ltmp5725:
	.loc	26 161 24
	jbe	.LBB32_182
.Ltmp5726:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm10, %xmm5
.LBB32_182:
.Ltmp5727:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm11, %xmm13, %xmm10
.Ltmp5728:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm11, %xmm15, %xmm11
.Ltmp5729:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm11, %xmm10
.Ltmp5730:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp5731:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm11, %r8d
.Ltmp5732:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r8d
.Ltmp5733:
	.loc	26 66 9
	vaddss	%xmm7, %xmm9, %xmm14
.Ltmp5734:
	.loc	26 161 24
	vcmpnltss	%xmm7, %xmm6, %xmm15
	vblendvps	%xmm15, %xmm7, %xmm14, %xmm7
	vmovss	80(%rsp), %xmm14
	vucomiss	%xmm6, %xmm14
.Ltmp5735:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r8d
	vmovss	.LCPI32_5(%rip), %xmm14
.Ltmp5736:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm14, %xmm10, %xmm10
.Ltmp5737:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm14, %xmm11, %xmm11
.Ltmp5738:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm11, %xmm10
	vmovss	4(%rsp), %xmm11
	vucomiss	%xmm6, %xmm11
.Ltmp5739:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp5740:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp5741:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp5742:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm10
.Ltmp5743:
	.loc	26 161 24
	movl	$841731191, %r8d
	cmovbel	%r8d, %edi
.Ltmp5744:
	.loc	21 326 13
	vmovss	%xmm5, 916(%r14)
.Ltmp5745:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp5746:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm10
.Ltmp5747:
	.loc	21 331 13
	vmovss	%xmm2, 924(%r14)
.Ltmp5748:
	.loc	26 161 24
	movl	$8388608, %r8d
	cmovbel	%r8d, %edi
.Ltmp5749:
	.loc	26 185 42
	movl	%edi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp5750:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp5751:
	.loc	26 66 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp5752:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm2, %xmm10
.Ltmp5753:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm11
	vsubss	%xmm10, %xmm11, %xmm10
.Ltmp5754:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm10
.Ltmp5755:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm10, %xmm10
.Ltmp5756:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm10
.Ltmp5757:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm10, %xmm10
.Ltmp5758:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm10
.Ltmp5759:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm10, %xmm10
.Ltmp5760:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm10
.Ltmp5761:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm10, %xmm10
.Ltmp5762:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp5763:
	.loc	26 71 9
	vmulss	%xmm2, %xmm10, %xmm2
.Ltmp5764:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp5765:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm10, %xmm10
.Ltmp5766:
	.loc	26 61 9
	vaddss	%xmm2, %xmm10, %xmm2
.Ltmp5767:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm2, %xmm2
.Ltmp5768:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm2, %xmm2
.Ltmp5769:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm2, %xmm14
.Ltmp5770:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm5, %xmm1, %xmm2
.Ltmp5771:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp5772:
	.loc	26 28 5
	movl	$0, %r9d
	adcl	$-1, %r9d
.Ltmp5773:
	.loc	26 129 14
	vucomiss	%xmm1, %xmm14
.Ltmp5774:
	.loc	21 332 13
	vmovss	%xmm7, 928(%r14)
.Ltmp5775:
	.loc	26 144 9
	movl	$0, %r8d
	adcl	$-1, %r8d
.Ltmp5776:
	.loc	26 149 9
	movl	%r9d, %edi
.Ltmp5777:
	.loc	21 370 47
	vmovss	936(%r14), %xmm2
.Ltmp5778:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm2
.Ltmp5779:
	.loc	26 149 9
	notl	%edi
.Ltmp5780:
	.loc	26 139 9
	cmovbel	%r13d, %edi
.Ltmp5781:
	.loc	21 362 20
	vmovss	932(%r14), %xmm5
.Ltmp5782:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm5
.Ltmp5783:
	.loc	26 144 9
	cmoval	%r9d, %r8d
.Ltmp5784:
	.loc	26 139 9
	cmovbel	%r13d, %edi
.Ltmp5785:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB32_184
.Ltmp5786:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm2, %xmm9, %xmm2
.LBB32_184:
	movq	64(%rsp), %r9
	vmovss	.LCPI32_20(%rip), %xmm11
	vmovss	.LCPI32_21(%rip), %xmm15
	vmovss	192(%rsp), %xmm7
.Ltmp5787:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r8b
	jne	.LBB32_186
.Ltmp5788:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm7
.LBB32_186:
.Ltmp5789:
	vmulss	%xmm0, %xmm11, %xmm2
	vmaxss	%xmm15, %xmm2, %xmm2
	vminss	.LCPI32_22(%rip), %xmm2, %xmm5
	vroundss	$9, %xmm5, %xmm5, %xmm2
	vsubss	%xmm2, %xmm5, %xmm5
.Ltmp5790:
	orl	%r8d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp5791:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm7, 936(%r14)
	.loc	21 382 5
	movl	%edi, 932(%r14)
.Ltmp5792:
	.loc	26 66 9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp5793:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm1, %xmm14, %xmm1
.Ltmp5794:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp5795:
	.loc	26 98 24
	vbroadcastss	.LCPI32_18(%rip), %xmm3
	vxorps	%xmm3, %xmm4, %xmm3
.Ltmp5796:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm1, %xmm1
.Ltmp5797:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm10, %xmm6, %xmm3
	vcmpltss	%xmm6, %xmm1, %xmm4
	vandps	%xmm4, %xmm3, %xmm3
	vmovd	%xmm3, %edi
	testb	$1, %dil
	jne	.LBB32_188
.Ltmp5798:
	.loc	26 0 44
	vxorps	%xmm1, %xmm1, %xmm1
	jmp	.LBB32_188
.LBB32_189:
.Ltmp5799:
	.loc	15 2584 13 is_stmt 1
	leal	(%rsi,%rbx), %eax
.Ltmp5800:
	.loc	21 297 5
	movl	%eax, 1208(%r14)
	movq	72(%rsp), %r11
	movq	352(%rsp), %rdx
	movq	168(%rsp), %rcx
.Ltmp5801:
.LBB32_41:
	.loc	6 688 12
	cmpq	%rcx, %r11
	jbe	.LBB32_42
.Ltmp5802:
	.loc	25 580 12
	movq	%rsi, %rax
	subq	%rdx, %rax
	movq	%rax, 160(%rsp)
	ja	.LBB32_376
.Ltmp5803:
	.loc	25 101 24
	leaq	(%r9,%rsi,4), %r12
.Ltmp5804:
	.loc	25 585 27
	movq	%r11, %rax
	subq	%rsi, %rax
	movq	%rax, 320(%rsp)
	movq	40(%rsp), %rax
.Ltmp5805:
	.loc	25 101 24
	leaq	(%rax,%rsi,4), %r13
.Ltmp5806:
	.loc	6 730 27
	movq	104(%r14), %r8
	movq	112(%r14), %rax
	.loc	6 735 27
	movq	168(%r14), %rbp
	movq	176(%r14), %r15
	.loc	6 741 24
	movl	1212(%r14), %ebx
	.loc	6 742 20
	movl	1216(%r14), %ecx
	movl	%ecx, 188(%rsp)
.Ltmp5807:
	.loc	21 238 16
	movl	1208(%r14), %r10d
	movl	136(%r14), %ecx
	movl	200(%r14), %r9d
	vmovss	792(%r14), %xmm0
	vmovss	760(%r14), %xmm1
	vmovss	%xmm1, 80(%rsp)
	vmovss	764(%r14), %xmm1
	vmovss	%xmm1, 4(%rsp)
	vsubss	840(%r14), %xmm0, %xmm1
	vmovss	%xmm1, 192(%rsp)
	vmovss	752(%r14), %xmm1
	vmovss	%xmm1, 28(%rsp)
	vmovss	.LCPI32_3(%rip), %xmm14
	vaddss	808(%r14), %xmm14, %xmm1
	vmovss	%xmm1, 24(%rsp)
	vmovss	824(%r14), %xmm1
	vbroadcastss	.LCPI32_18(%rip), %xmm2
	vxorps	%xmm2, %xmm1, %xmm1
	vmovaps	%xmm1, 336(%rsp)
	movl	744(%r14), %edi
	movl	%edi, 288(%rsp)
	movl	748(%r14), %edi
	movl	%edi, 272(%rsp)
	vmovss	756(%r14), %xmm1
	vmovss	%xmm1, 88(%rsp)
	vmovss	868(%r14), %xmm9
	vmovss	784(%r14), %xmm1
	vmovss	%xmm1, 176(%rsp)
	vmovss	788(%r14), %xmm1
	vmovss	%xmm1, 168(%rsp)
	vsubss	916(%r14), %xmm9, %xmm1
	vmovss	%xmm1, 124(%rsp)
	vmovss	776(%r14), %xmm1
	vmovss	%xmm1, 120(%rsp)
	vmovss	900(%r14), %xmm1
	vxorps	%xmm2, %xmm1, %xmm1
	vmovaps	%xmm1, 368(%rsp)
	vaddss	884(%r14), %xmm14, %xmm1
	vmovss	%xmm1, 116(%rsp)
	movl	768(%r14), %edi
	movl	%edi, 112(%rsp)
	movl	772(%r14), %edi
	movl	%edi, 108(%rsp)
	vmovss	780(%r14), %xmm1
	vmovss	%xmm1, 104(%rsp)
	vmovss	860(%r14), %xmm4
	vmovss	864(%r14), %xmm7
	vmovss	936(%r14), %xmm13
	vmovss	940(%r14), %xmm2
	cmpq	%rdx, %r11
	movq	%rsi, 8(%rsp)
	movq	%r12, 56(%rsp)
	movq	%r13, 128(%rsp)
	movq	%r10, 48(%rsp)
	vmovss	%xmm0, 100(%rsp)
	vmovss	%xmm9, 16(%rsp)
	jbe	.LBB32_306
	.loc	21 0 16 is_stmt 0
	subq	%rsi, %rdx
	movq	%rdx, 392(%rsp)
.Ltmp5808:
	.loc	25 451 16 is_stmt 1
	movl	%r10d, %edx
	subl	%r9d, %edx
	movq	%rdx, 408(%rsp)
	movl	%r10d, %edx
	subl	%ecx, %edx
	movq	%rdx, 312(%rsp)
	movl	%r10d, %ecx
	subl	188(%rsp), %ecx
	movq	%rcx, 152(%rsp)
	movq	%rsi, %rcx
	subq	%r11, %rcx
	movl	$1, %r9d
	vmovss	.LCPI32_5(%rip), %xmm15
	vmovss	.LCPI32_6(%rip), %xmm6
	xorl	%edx, %edx
	vmovss	.LCPI32_27(%rip), %xmm10
	jmp	.LBB32_304
.Ltmp5809:
	.loc	25 0 16 is_stmt 0
.Ltmp5810:
	.p2align	4
.LBB32_371:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm15
	movl	112(%rsp), %edi
.Ltmp5811:
	.loc	26 161 24
	cmovbel	108(%rsp), %edi
.Ltmp5812:
	.loc	7 1291 18
	vmovd	%edi, %xmm1
.Ltmp5813:
	.loc	26 66 9
	vsubss	%xmm2, %xmm15, %xmm6
.Ltmp5814:
	.loc	26 92 9
	vmulss	%xmm1, %xmm6, %xmm1
	vaddss	%xmm1, %xmm2, %xmm1
.Ltmp5815:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm0
	vandps	%xmm0, %xmm1, %xmm2
.Ltmp5816:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm2, %xmm2
	vandps	%xmm1, %xmm2, %xmm2
	vmovss	.LCPI32_23(%rip), %xmm15
.Ltmp5817:
	.loc	26 71 9
	vmulss	%xmm15, %xmm14, %xmm1
	vmovss	.LCPI32_24(%rip), %xmm0
.Ltmp5818:
	.loc	26 61 9
	vaddss	%xmm0, %xmm1, %xmm1
.Ltmp5819:
	.loc	26 71 9
	vmulss	%xmm1, %xmm14, %xmm1
	vmovss	.LCPI32_25(%rip), %xmm4
.Ltmp5820:
	.loc	26 61 9
	vaddss	%xmm4, %xmm1, %xmm1
.Ltmp5821:
	.loc	26 71 9
	vmulss	%xmm1, %xmm14, %xmm1
	vmovss	.LCPI32_26(%rip), %xmm7
.Ltmp5822:
	.loc	26 61 9
	vaddss	%xmm7, %xmm1, %xmm1
.Ltmp5823:
	.loc	26 71 9
	vmulss	%xmm1, %xmm14, %xmm1
.Ltmp5824:
	.loc	26 61 9
	vaddss	%xmm1, %xmm10, %xmm1
.Ltmp5825:
	.loc	26 71 9
	vmulss	%xmm1, %xmm14, %xmm1
.Ltmp5826:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI32_20(%rip), %xmm2, %xmm6
.Ltmp5827:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI32_21(%rip), %xmm6, %xmm6
.Ltmp5828:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm6, %xmm6
	vmovss	.LCPI32_2(%rip), %xmm14
.Ltmp5829:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm1
	vmovss	.LCPI32_28(%rip), %xmm8
.Ltmp5830:
	.loc	26 178 22
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp5831:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5832:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5833:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5834:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp5835:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm12, %xmm1
.Ltmp5836:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm9, %xmm3
	vblendvps	%xmm3, %xmm1, %xmm12, %xmm1
	vcmpnltss	88(%rsp), %xmm5, %xmm3
	vblendvps	%xmm3, %xmm1, %xmm12, %xmm1
.Ltmp5837:
	.loc	7 1783 9
	vroundss	$9, %xmm6, %xmm6, %xmm3
.Ltmp5838:
	.loc	26 66 9
	vsubss	%xmm3, %xmm6, %xmm6
.Ltmp5839:
	.loc	26 71 9
	vmulss	%xmm6, %xmm15, %xmm12
.Ltmp5840:
	.loc	26 61 9
	vaddss	%xmm0, %xmm12, %xmm12
.Ltmp5841:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm12
.Ltmp5842:
	.loc	26 61 9
	vaddss	%xmm4, %xmm12, %xmm12
.Ltmp5843:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm12
.Ltmp5844:
	.loc	26 61 9
	vaddss	%xmm7, %xmm12, %xmm12
	vmovaps	%xmm9, %xmm7
.Ltmp5845:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm12
.Ltmp5846:
	.loc	26 61 9
	vaddss	%xmm10, %xmm12, %xmm12
.Ltmp5847:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm6
.Ltmp5848:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp5849:
	.loc	26 178 22
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp5850:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5851:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5852:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5853:
	.loc	26 71 9
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp5854:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp5855:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm2, %xmm6
	vblendvps	%xmm6, %xmm3, %xmm11, %xmm3
	vcmpnltss	104(%rsp), %xmm5, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm11, %xmm3
.Ltmp5856:
	.loc	21 394 5
	vmovss	%xmm2, 940(%r14)
	movq	56(%rsp), %r12
.Ltmp5857:
	.loc	26 56 9
	vmovss	%xmm1, -4(%r12,%r9,4)
.Ltmp5858:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%r13,%r9,4)
.Ltmp5859:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rcx,%r9), %rdi
	incq	%rdi
	incq	%r9
	cmpq	$1, %rdi
	movq	48(%rsp), %r10
	vmovss	.LCPI32_3(%rip), %xmm14
	vmovss	.LCPI32_6(%rip), %xmm6
	vmovss	16(%rsp), %xmm9
	vmovss	.LCPI32_5(%rip), %xmm15
	vmovss	100(%rsp), %xmm0
	vmovss	32(%rsp), %xmm4
.Ltmp5860:
	.loc	11 900 12
	je	.LBB32_327
.Ltmp5861:
.LBB32_304:
	.loc	21 246 22
	leal	(%r10,%r9), %edi
	decl	%edi
	andl	%ebx, %edi
.Ltmp5862:
	.loc	25 451 16
	cmpq	%rdi, %rax
	jbe	.LBB32_305
.Ltmp5863:
	.loc	25 0 16 is_stmt 0
	movq	160(%rsp), %rsi
	leaq	(%rsi,%r9), %r10
.Ltmp5864:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r9,4), %xmm3
.Ltmp5865:
	.loc	26 56 9
	vmovss	%xmm3, (%r8,%rdi,4)
.Ltmp5866:
	.loc	25 438 16
	cmpq	$1, %r10
	je	.LBB32_379
.Ltmp5867:
	.loc	25 451 16
	cmpq	%rdi, %r15
	jbe	.LBB32_380
.Ltmp5868:
	.loc	26 51 9
	vmovss	-4(%r13,%r9,4), %xmm3
.Ltmp5869:
	.loc	26 56 9
	vmovss	%xmm3, (%rbp,%rdi,4)
	movq	152(%rsp), %rsi
.Ltmp5870:
	.loc	21 255 21
	leal	(%rsi,%r9), %edi
	decl	%edi
	andl	%ebx, %edi
.Ltmp5871:
	.loc	25 438 16
	cmpq	%rdi, %rax
	jbe	.LBB32_381
.Ltmp5872:
	.loc	25 438 16 is_stmt 0
	cmpq	%rdi, %r15
	jbe	.LBB32_382
.Ltmp5873:
	.loc	25 0 16
	movq	312(%rsp), %rsi
	leal	(%rsi,%r9), %r11d
	decl	%r11d
	andl	%ebx, %r11d
	cmpq	%r11, %rax
	jbe	.LBB32_375
	cmpq	%r11, %r15
.Ltmp5874:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB32_374
	.loc	21 0 29 is_stmt 0
	movq	408(%rsp), %rsi
	leal	(%rsi,%r9), %r10d
	decl	%r10d
	andl	%ebx, %r10d
	cmpq	%r10, %r15
	jbe	.LBB32_373
	cmpq	%r10, %rax
	.loc	21 277 29 is_stmt 1
	jbe	.LBB32_372
.Ltmp5875:
	.loc	21 0 29 is_stmt 0
	vmovss	(%rbp,%r11,4), %xmm3
.Ltmp5876:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r8,%r11,4), %xmm5
	vbroadcastss	.LCPI32_4(%rip), %xmm1
	vandps	%xmm1, %xmm5, %xmm11
.Ltmp5877:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp5878:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm3, %xmm11
.Ltmp5879:
	.loc	7 1244 18
	vmovd	%xmm11, %r11d
.Ltmp5880:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm3, %r12d
.Ltmp5881:
	.loc	26 161 24 is_stmt 1
	cmoval	%r11d, %r12d
	vxorps	%xmm5, %xmm5, %xmm5
	vmovss	80(%rsp), %xmm1
	vucomiss	%xmm5, %xmm1
.Ltmp5882:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r11d, %r12d
	vmovss	4(%rsp), %xmm1
	vucomiss	%xmm5, %xmm1
.Ltmp5883:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm15, %xmm11, %xmm11
.Ltmp5884:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm15, %xmm3
.Ltmp5885:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5886:
	.loc	7 1244 18
	vmovd	%xmm3, %r11d
.Ltmp5887:
	.loc	26 161 24
	cmovbel	%r12d, %r11d
.Ltmp5888:
	.loc	7 1291 18
	vmovd	%r11d, %xmm3
.Ltmp5889:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm3
.Ltmp5890:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %r11d
.Ltmp5891:
	.loc	7 1291 18
	vmovd	%r11d, %xmm3
.Ltmp5892:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm3
.Ltmp5893:
	.loc	26 161 24
	movl	$8388608, %esi
	cmovbel	%esi, %r11d
.Ltmp5894:
	.loc	26 185 42
	movl	%r11d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp5895:
	.loc	7 1291 18
	vmovd	%r12d, %xmm3
.Ltmp5896:
	.loc	26 66 9
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp5897:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm3, %xmm11
.Ltmp5898:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm1
	vsubss	%xmm11, %xmm1, %xmm11
.Ltmp5899:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm11
.Ltmp5900:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm11, %xmm11
.Ltmp5901:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm11
.Ltmp5902:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm11, %xmm11
.Ltmp5903:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm11
.Ltmp5904:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm11, %xmm11
.Ltmp5905:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm11
.Ltmp5906:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm11, %xmm11
.Ltmp5907:
	.loc	26 187 28
	shrl	$23, %r11d
	orl	$1258291200, %r11d
.Ltmp5908:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp5909:
	.loc	7 1291 18
	vmovd	%r11d, %xmm11
.Ltmp5910:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm11, %xmm11
.Ltmp5911:
	.loc	26 61 9
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp5912:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp5913:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
.Ltmp5914:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm3, %xmm3
.Ltmp5915:
	.loc	26 129 14 is_stmt 1
	vucomiss	192(%rsp), %xmm3
.Ltmp5916:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp5917:
	.loc	26 129 14
	vucomiss	%xmm0, %xmm3
.Ltmp5918:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp5919:
	.loc	26 149 9
	movl	%r13d, %r11d
.Ltmp5920:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm4
.Ltmp5921:
	.loc	26 149 9
	notl	%r11d
.Ltmp5922:
	.loc	26 139 9
	cmovbel	%edx, %r11d
.Ltmp5923:
	.loc	21 362 20
	vmovss	856(%r14), %xmm11
.Ltmp5924:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm11
.Ltmp5925:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp5926:
	.loc	26 139 9
	cmovbel	%edx, %r11d
.Ltmp5927:
	.loc	26 161 24
	testb	$1, %r11b
	jne	.LBB32_359
.Ltmp5928:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm11
	vmovss	28(%rsp), %xmm4
.Ltmp5929:
	.loc	26 161 24
	testb	$1, %r12b
	je	.LBB32_362
	jmp	.LBB32_363
.Ltmp5930:
	.loc	26 0 24
.Ltmp5931:
	.p2align	4
.LBB32_359:
	vaddss	%xmm4, %xmm14, %xmm11
	vmovss	28(%rsp), %xmm4
.Ltmp5932:
	.loc	26 161 24
	testb	$1, %r12b
	jne	.LBB32_363
.Ltmp5933:
.LBB32_362:
	.loc	26 0 24
	vmovaps	%xmm11, %xmm4
.LBB32_363:
	orl	%r12d, %r11d
	andl	$1065353216, %r11d
	vmovd	%r11d, %xmm11
.Ltmp5934:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm0, %xmm3, %xmm3
.Ltmp5935:
	.loc	26 71 9
	vmulss	24(%rsp), %xmm3, %xmm3
.Ltmp5936:
	.loc	26 161 24
	vmaxss	336(%rsp), %xmm3, %xmm3
.Ltmp5937:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm11, %xmm5, %xmm11
	vcmpltss	%xmm5, %xmm3, %xmm12
	vandps	%xmm12, %xmm11, %xmm11
	vmovd	%xmm11, %r12d
	testb	$1, %r12b
	jne	.LBB32_365
.Ltmp5938:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_365:
.Ltmp5939:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
	movl	288(%rsp), %r12d
.Ltmp5940:
	.loc	26 161 24
	cmovbel	272(%rsp), %r12d
.Ltmp5941:
	.loc	26 103 24
	vmovss	(%rbp,%r10,4), %xmm11
	vbroadcastss	.LCPI32_4(%rip), %xmm8
	vandps	%xmm8, %xmm11, %xmm11
.Ltmp5942:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r8,%r10,4), %xmm12
	vandps	%xmm8, %xmm12, %xmm12
.Ltmp5943:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm12, %xmm11
.Ltmp5944:
	.loc	7 1244 18
	vmovd	%xmm11, %r10d
.Ltmp5945:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm12, %r13d
.Ltmp5946:
	.loc	26 161 24 is_stmt 1
	cmoval	%r10d, %r13d
	vmovss	176(%rsp), %xmm1
	vucomiss	%xmm5, %xmm1
.Ltmp5947:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r10d, %r13d
	vmovss	168(%rsp), %xmm1
	vucomiss	%xmm5, %xmm1
.Ltmp5948:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm15, %xmm11, %xmm11
.Ltmp5949:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm15, %xmm12, %xmm12
.Ltmp5950:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm11, %xmm12, %xmm11
.Ltmp5951:
	.loc	7 1244 18
	vmovd	%xmm11, %r10d
.Ltmp5952:
	.loc	26 161 24
	cmovbel	%r13d, %r10d
.Ltmp5953:
	.loc	7 1291 18
	vmovd	%r10d, %xmm11
.Ltmp5954:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm11
.Ltmp5955:
	.loc	7 1291 18
	vmovd	%r12d, %xmm11
.Ltmp5956:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %r10d
.Ltmp5957:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp5958:
	.loc	7 1291 18
	vmovd	%r10d, %xmm12
.Ltmp5959:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm12
.Ltmp5960:
	.loc	26 92 9
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp5961:
	.loc	26 161 24
	movl	$8388608, %esi
	cmovbel	%esi, %r10d
.Ltmp5962:
	.loc	26 185 42
	movl	%r10d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp5963:
	.loc	7 1291 18
	vmovd	%r12d, %xmm11
.Ltmp5964:
	.loc	26 66 9
	vaddss	%xmm14, %xmm11, %xmm11
.Ltmp5965:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm11, %xmm12
.Ltmp5966:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm1
	vsubss	%xmm12, %xmm1, %xmm12
.Ltmp5967:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp5968:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm12, %xmm12
.Ltmp5969:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp5970:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm12, %xmm12
.Ltmp5971:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp5972:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm12, %xmm12
.Ltmp5973:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp5974:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm12, %xmm12
.Ltmp5975:
	.loc	26 187 28
	shrl	$23, %r10d
	orl	$1258291200, %r10d
.Ltmp5976:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm11
.Ltmp5977:
	.loc	7 1291 18
	vmovd	%r10d, %xmm12
.Ltmp5978:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm12, %xmm12
.Ltmp5979:
	.loc	26 61 9
	vaddss	%xmm11, %xmm12, %xmm11
	vmovss	(%r8,%rdi,4), %xmm12
.Ltmp5980:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm11, %xmm11
.Ltmp5981:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm11, %xmm11
.Ltmp5982:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm11, %xmm15
.Ltmp5983:
	.loc	26 129 14 is_stmt 1
	vucomiss	124(%rsp), %xmm15
.Ltmp5984:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp5985:
	.loc	26 129 14
	vucomiss	%xmm9, %xmm15
.Ltmp5986:
	.loc	26 92 9
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp5987:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp5988:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%r14), %xmm7
.Ltmp5989:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r10d
.Ltmp5990:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm13
.Ltmp5991:
	.loc	26 149 9
	notl	%r10d
.Ltmp5992:
	.loc	26 139 9
	cmovbel	%edx, %r10d
.Ltmp5993:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm7
.Ltmp5994:
	.loc	26 103 24
	vandps	%xmm3, %xmm8, %xmm7
.Ltmp5995:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm7, %xmm7
	vandps	%xmm3, %xmm7, %xmm7
	vmovss	(%rbp,%rdi,4), %xmm11
.Ltmp5996:
	.loc	21 373 5
	vmovss	%xmm4, 860(%r14)
	.loc	21 382 5
	movl	%r11d, 856(%r14)
.Ltmp5997:
	.loc	21 394 5
	vmovss	%xmm7, 864(%r14)
.Ltmp5998:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp5999:
	.loc	26 139 9
	cmovbel	%edx, %r10d
.Ltmp6000:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_367
.Ltmp6001:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm14, %xmm13, %xmm13
.LBB32_367:
	movq	72(%rsp), %r11
	movq	8(%rsp), %rsi
	movq	128(%rsp), %r13
	vmovss	120(%rsp), %xmm1
.Ltmp6002:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r12b
	jne	.LBB32_369
.Ltmp6003:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm1
.LBB32_369:
	vmovaps	%xmm1, %xmm6
	vmovss	%xmm4, 32(%rsp)
	vmovaps	%xmm7, %xmm9
.Ltmp6004:
	vmulss	.LCPI32_20(%rip), %xmm7, %xmm3
	vmaxss	.LCPI32_21(%rip), %xmm3, %xmm3
	vminss	.LCPI32_22(%rip), %xmm3, %xmm14
	vroundss	$9, %xmm14, %xmm14, %xmm3
	vsubss	%xmm3, %xmm14, %xmm14
.Ltmp6005:
	orl	%r12d, %r10d
	andl	$1065353216, %r10d
	vmovd	%r10d, %xmm1
	vmovaps	%xmm6, %xmm13
.Ltmp6006:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm6, 936(%r14)
	.loc	21 382 5
	movl	%r10d, 932(%r14)
.Ltmp6007:
	.loc	26 66 9
	vsubss	16(%rsp), %xmm15, %xmm15
.Ltmp6008:
	.loc	26 71 9
	vmulss	116(%rsp), %xmm15, %xmm15
.Ltmp6009:
	.loc	26 161 24
	vmaxss	368(%rsp), %xmm15, %xmm15
.Ltmp6010:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm1, %xmm5, %xmm1
	vcmpltss	%xmm5, %xmm15, %xmm6
	vandps	%xmm6, %xmm1, %xmm1
	vmovd	%xmm1, %edi
	testb	$1, %dil
	jne	.LBB32_371
.Ltmp6011:
	.loc	26 0 44
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB32_371
.LBB32_306:
	movl	188(%rsp), %esi
.Ltmp6012:
	.loc	25 451 16 is_stmt 1
	movl	%r10d, %edx
	subl	%r9d, %edx
	cmpq	%r15, %rax
	jbe	.LBB32_307
	.loc	25 0 16 is_stmt 0
	movq	%rdx, 312(%rsp)
	.loc	25 451 16
	movl	%r10d, %edx
	subl	%ecx, %edx
	movq	%rdx, 152(%rsp)
	movl	%r10d, %r9d
	subl	%esi, %r9d
	xorl	%edx, %edx
	vmovss	.LCPI32_5(%rip), %xmm10
	vmovss	.LCPI32_6(%rip), %xmm6
	vmovss	.LCPI32_7(%rip), %xmm15
	xorl	%ecx, %ecx
	vxorps	%xmm12, %xmm12, %xmm12
	jmp	.LBB32_329
.Ltmp6013:
	.loc	25 0 16
.Ltmp6014:
	.p2align	4
.LBB32_349:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm1
	movl	112(%rsp), %edi
.Ltmp6015:
	.loc	26 161 24
	cmovbel	108(%rsp), %edi
.Ltmp6016:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp6017:
	.loc	26 66 9
	vsubss	%xmm2, %xmm1, %xmm1
.Ltmp6018:
	.loc	26 92 9
	vmulss	%xmm5, %xmm1, %xmm1
	vaddss	%xmm1, %xmm2, %xmm1
.Ltmp6019:
	.loc	26 103 24
	vandps	%xmm1, %xmm8, %xmm2
.Ltmp6020:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm2, %xmm2
	vandps	%xmm1, %xmm2, %xmm2
	vmovss	.LCPI32_23(%rip), %xmm8
.Ltmp6021:
	.loc	26 71 9
	vmulss	%xmm8, %xmm15, %xmm1
	vmovss	.LCPI32_24(%rip), %xmm9
.Ltmp6022:
	.loc	26 61 9
	vaddss	%xmm1, %xmm9, %xmm1
.Ltmp6023:
	.loc	26 71 9
	vmulss	%xmm1, %xmm15, %xmm1
	vmovss	.LCPI32_25(%rip), %xmm0
.Ltmp6024:
	.loc	26 61 9
	vaddss	%xmm0, %xmm1, %xmm1
.Ltmp6025:
	.loc	26 71 9
	vmulss	%xmm1, %xmm15, %xmm1
	vmovss	.LCPI32_26(%rip), %xmm10
.Ltmp6026:
	.loc	26 61 9
	vaddss	%xmm1, %xmm10, %xmm1
.Ltmp6027:
	.loc	26 71 9
	vmulss	%xmm1, %xmm15, %xmm1
	vmovss	.LCPI32_27(%rip), %xmm13
.Ltmp6028:
	.loc	26 61 9
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp6029:
	.loc	26 71 9
	vmulss	%xmm1, %xmm15, %xmm1
.Ltmp6030:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI32_20(%rip), %xmm2, %xmm5
.Ltmp6031:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI32_21(%rip), %xmm5, %xmm5
.Ltmp6032:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm5, %xmm5
	vmovss	.LCPI32_2(%rip), %xmm15
.Ltmp6033:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm1, %xmm15, %xmm1
	vmovss	.LCPI32_28(%rip), %xmm4
.Ltmp6034:
	.loc	26 178 22
	vaddss	%xmm4, %xmm14, %xmm6
.Ltmp6035:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp6036:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp6037:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp6038:
	.loc	26 71 9
	vmulss	%xmm6, %xmm1, %xmm1
.Ltmp6039:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm3, %xmm1
.Ltmp6040:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm12, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vcmpnltss	88(%rsp), %xmm12, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
.Ltmp6041:
	.loc	7 1783 9
	vroundss	$9, %xmm5, %xmm5, %xmm3
.Ltmp6042:
	.loc	26 66 9
	vsubss	%xmm3, %xmm5, %xmm5
.Ltmp6043:
	.loc	26 71 9
	vmulss	%xmm5, %xmm8, %xmm6
.Ltmp6044:
	.loc	26 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp6045:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp6046:
	.loc	26 61 9
	vaddss	%xmm0, %xmm6, %xmm6
.Ltmp6047:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp6048:
	.loc	26 61 9
	vaddss	%xmm6, %xmm10, %xmm6
.Ltmp6049:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp6050:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp6051:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm5
.Ltmp6052:
	.loc	26 61 9
	vaddss	%xmm5, %xmm15, %xmm5
.Ltmp6053:
	.loc	26 178 22
	vaddss	%xmm4, %xmm3, %xmm3
.Ltmp6054:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp6055:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp6056:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp6057:
	.loc	26 71 9
	vmulss	%xmm3, %xmm5, %xmm3
.Ltmp6058:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp6059:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm2, %xmm12, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm11, %xmm3
	vcmpnltss	104(%rsp), %xmm12, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm11, %xmm3
.Ltmp6060:
	.loc	21 394 5
	vmovss	%xmm2, 940(%r14)
	movq	56(%rsp), %r12
.Ltmp6061:
	.loc	26 56 9
	vmovss	%xmm1, (%r12,%rcx,4)
.Ltmp6062:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, (%r13,%rcx,4)
	incq	%rcx
.Ltmp6063:
	.loc	8 1916 50 is_stmt 1
	cmpq	%rcx, 320(%rsp)
	movq	48(%rsp), %r10
	vmovss	.LCPI32_3(%rip), %xmm14
	vmovss	.LCPI32_6(%rip), %xmm6
	vmovss	.LCPI32_5(%rip), %xmm10
	vmovss	.LCPI32_7(%rip), %xmm15
	vmovss	100(%rsp), %xmm0
	vmovss	16(%rsp), %xmm9
	vmovss	160(%rsp), %xmm13
	vmovss	32(%rsp), %xmm4
.Ltmp6064:
	.loc	11 900 12
	je	.LBB32_327
.Ltmp6065:
.LBB32_329:
	.loc	21 246 22
	leal	(%r10,%rcx), %edi
	andl	%ebx, %edi
.Ltmp6066:
	.loc	25 451 16
	cmpq	%rdi, %rax
	jbe	.LBB32_305
.Ltmp6067:
	.loc	26 51 9
	vmovss	(%r12,%rcx,4), %xmm1
.Ltmp6068:
	.loc	26 56 9
	vmovss	%xmm1, (%r8,%rdi,4)
.Ltmp6069:
	.loc	25 451 16
	cmpq	%rdi, %r15
	jbe	.LBB32_380
.Ltmp6070:
	.loc	26 51 9
	vmovss	(%r13,%rcx,4), %xmm1
.Ltmp6071:
	.loc	26 56 9
	vmovss	%xmm1, (%rbp,%rdi,4)
.Ltmp6072:
	.loc	21 255 21
	leal	(%r9,%rcx), %edi
	andl	%ebx, %edi
.Ltmp6073:
	.loc	25 438 16
	cmpq	%rdi, %rax
	jbe	.LBB32_381
.Ltmp6074:
	.loc	25 438 16 is_stmt 0
	cmpq	%rdi, %r15
	jbe	.LBB32_382
.Ltmp6075:
	.loc	25 0 16
	movq	152(%rsp), %rsi
	leal	(%rsi,%rcx), %r11d
	andl	%ebx, %r11d
	cmpq	%r11, %rax
	jbe	.LBB32_375
	cmpq	%r11, %r15
	vbroadcastss	.LCPI32_4(%rip), %xmm8
.Ltmp6076:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB32_374
.Ltmp6077:
	.loc	21 0 29 is_stmt 0
	movq	312(%rsp), %rsi
	leal	(%rsi,%rcx), %r10d
	andl	%ebx, %r10d
	cmpq	%r10, %r15
	jbe	.LBB32_373
.Ltmp6078:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r8,%r11,4), %xmm1
	vandps	%xmm1, %xmm8, %xmm1
.Ltmp6079:
	.loc	26 103 24 is_stmt 0
	vmovss	(%rbp,%r11,4), %xmm3
	vandps	%xmm3, %xmm8, %xmm3
.Ltmp6080:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm3, %xmm1
.Ltmp6081:
	.loc	7 1244 18
	vmovd	%xmm1, %r11d
.Ltmp6082:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm3, %r12d
.Ltmp6083:
	.loc	26 161 24 is_stmt 1
	cmoval	%r11d, %r12d
	vmovss	80(%rsp), %xmm5
	vucomiss	%xmm12, %xmm5
.Ltmp6084:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r11d, %r12d
	vmovss	4(%rsp), %xmm5
	vucomiss	%xmm12, %xmm5
.Ltmp6085:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm10, %xmm1
.Ltmp6086:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm10, %xmm3
.Ltmp6087:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm1, %xmm3, %xmm1
.Ltmp6088:
	.loc	7 1244 18
	vmovd	%xmm1, %r11d
.Ltmp6089:
	.loc	26 161 24
	cmovbel	%r12d, %r11d
.Ltmp6090:
	.loc	7 1291 18
	vmovd	%r11d, %xmm1
.Ltmp6091:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm1
.Ltmp6092:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %r11d
.Ltmp6093:
	.loc	7 1291 18
	vmovd	%r11d, %xmm1
.Ltmp6094:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm1
.Ltmp6095:
	.loc	26 161 24
	movl	$8388608, %esi
	cmovbel	%esi, %r11d
.Ltmp6096:
	.loc	26 185 42
	movl	%r11d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp6097:
	.loc	7 1291 18
	vmovd	%r12d, %xmm1
.Ltmp6098:
	.loc	26 66 9
	vaddss	%xmm1, %xmm14, %xmm1
.Ltmp6099:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm1, %xmm3
.Ltmp6100:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm5
	vsubss	%xmm3, %xmm5, %xmm3
.Ltmp6101:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp6102:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm3, %xmm3
.Ltmp6103:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp6104:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm3, %xmm3
.Ltmp6105:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp6106:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm3, %xmm3
.Ltmp6107:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp6108:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm3, %xmm3
.Ltmp6109:
	.loc	26 187 28
	shrl	$23, %r11d
	orl	$1258291200, %r11d
.Ltmp6110:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp6111:
	.loc	7 1291 18
	vmovd	%r11d, %xmm3
.Ltmp6112:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp6113:
	.loc	26 61 9
	vaddss	%xmm1, %xmm3, %xmm1
.Ltmp6114:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm1, %xmm1
.Ltmp6115:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm1, %xmm1
.Ltmp6116:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm1, %xmm1
.Ltmp6117:
	.loc	26 129 14 is_stmt 1
	vucomiss	192(%rsp), %xmm1
.Ltmp6118:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp6119:
	.loc	26 129 14
	vucomiss	%xmm0, %xmm1
.Ltmp6120:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6121:
	.loc	26 149 9
	movl	%r13d, %r11d
.Ltmp6122:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm4
.Ltmp6123:
	.loc	26 149 9
	notl	%r11d
.Ltmp6124:
	.loc	26 139 9
	cmovbel	%edx, %r11d
.Ltmp6125:
	.loc	21 362 20
	vmovss	856(%r14), %xmm3
.Ltmp6126:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm3
.Ltmp6127:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp6128:
	.loc	26 139 9
	cmovbel	%edx, %r11d
.Ltmp6129:
	.loc	26 161 24
	testb	$1, %r11b
	jne	.LBB32_337
.Ltmp6130:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm3
	vmovss	28(%rsp), %xmm4
.Ltmp6131:
	.loc	26 161 24
	testb	$1, %r12b
	je	.LBB32_340
	jmp	.LBB32_341
.Ltmp6132:
	.loc	26 0 24
.Ltmp6133:
	.p2align	4
.LBB32_337:
	vaddss	%xmm4, %xmm14, %xmm3
	vmovss	28(%rsp), %xmm4
.Ltmp6134:
	.loc	26 161 24
	testb	$1, %r12b
	jne	.LBB32_341
.Ltmp6135:
.LBB32_340:
	.loc	26 0 24
	vmovaps	%xmm3, %xmm4
.LBB32_341:
	orl	%r12d, %r11d
	andl	$1065353216, %r11d
	vmovd	%r11d, %xmm3
.Ltmp6136:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm0, %xmm1, %xmm1
.Ltmp6137:
	.loc	26 71 9
	vmulss	24(%rsp), %xmm1, %xmm1
.Ltmp6138:
	.loc	26 161 24
	vmaxss	336(%rsp), %xmm1, %xmm1
.Ltmp6139:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm3, %xmm12, %xmm3
	vcmpltss	%xmm12, %xmm1, %xmm11
	vandps	%xmm3, %xmm11, %xmm3
	vmovd	%xmm3, %r12d
	testb	$1, %r12b
	jne	.LBB32_343
.Ltmp6140:
	.loc	26 0 44
	vxorps	%xmm1, %xmm1, %xmm1
.LBB32_343:
.Ltmp6141:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm1
	movl	288(%rsp), %r12d
.Ltmp6142:
	.loc	26 161 24
	cmovbel	272(%rsp), %r12d
.Ltmp6143:
	.loc	26 103 24
	vmovss	(%rbp,%r10,4), %xmm3
	vandps	%xmm3, %xmm8, %xmm3
.Ltmp6144:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r8,%r10,4), %xmm11
	vandps	%xmm8, %xmm11, %xmm11
.Ltmp6145:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm11, %xmm3
.Ltmp6146:
	.loc	7 1244 18
	vmovd	%xmm3, %r10d
.Ltmp6147:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm11, %r13d
.Ltmp6148:
	.loc	26 161 24 is_stmt 1
	cmoval	%r10d, %r13d
	vmovss	176(%rsp), %xmm5
	vucomiss	%xmm12, %xmm5
.Ltmp6149:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r10d, %r13d
	vmovss	168(%rsp), %xmm5
	vucomiss	%xmm12, %xmm5
.Ltmp6150:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm10, %xmm3
.Ltmp6151:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm10, %xmm11, %xmm11
.Ltmp6152:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm3
.Ltmp6153:
	.loc	7 1244 18
	vmovd	%xmm3, %r10d
.Ltmp6154:
	.loc	26 161 24
	cmovbel	%r13d, %r10d
.Ltmp6155:
	.loc	7 1291 18
	vmovd	%r10d, %xmm3
.Ltmp6156:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm3
.Ltmp6157:
	.loc	7 1291 18
	vmovd	%r12d, %xmm3
.Ltmp6158:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %r10d
.Ltmp6159:
	.loc	26 66 9
	vsubss	%xmm7, %xmm1, %xmm1
.Ltmp6160:
	.loc	7 1291 18
	vmovd	%r10d, %xmm11
.Ltmp6161:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm11
.Ltmp6162:
	.loc	26 92 9
	vmulss	%xmm3, %xmm1, %xmm11
.Ltmp6163:
	.loc	26 161 24
	movl	$8388608, %esi
	cmovbel	%esi, %r10d
.Ltmp6164:
	.loc	26 185 42
	movl	%r10d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp6165:
	.loc	7 1291 18
	vmovd	%r12d, %xmm1
.Ltmp6166:
	.loc	26 66 9
	vaddss	%xmm1, %xmm14, %xmm1
.Ltmp6167:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm1, %xmm3
.Ltmp6168:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm0
	vsubss	%xmm3, %xmm0, %xmm3
.Ltmp6169:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp6170:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm3, %xmm3
.Ltmp6171:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp6172:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm3, %xmm3
.Ltmp6173:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp6174:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm3, %xmm3
.Ltmp6175:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp6176:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm3, %xmm3
.Ltmp6177:
	.loc	26 187 28
	shrl	$23, %r10d
	orl	$1258291200, %r10d
.Ltmp6178:
	.loc	26 71 9
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp6179:
	.loc	7 1291 18
	vmovd	%r10d, %xmm3
.Ltmp6180:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp6181:
	.loc	26 61 9
	vaddss	%xmm1, %xmm3, %xmm1
	vmovss	(%r8,%rdi,4), %xmm3
.Ltmp6182:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm1, %xmm1
.Ltmp6183:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm1, %xmm1
.Ltmp6184:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm1, %xmm1
.Ltmp6185:
	.loc	26 129 14 is_stmt 1
	vucomiss	124(%rsp), %xmm1
.Ltmp6186:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp6187:
	.loc	26 129 14
	vucomiss	%xmm9, %xmm1
.Ltmp6188:
	.loc	26 92 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp6189:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6190:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%r14), %xmm11
.Ltmp6191:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r10d
.Ltmp6192:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm13
.Ltmp6193:
	.loc	26 149 9
	notl	%r10d
.Ltmp6194:
	.loc	26 139 9
	cmovbel	%edx, %r10d
.Ltmp6195:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm11
.Ltmp6196:
	.loc	26 103 24
	vandps	%xmm7, %xmm8, %xmm11
.Ltmp6197:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm11, %xmm11
	vandps	%xmm7, %xmm11, %xmm7
	vmovss	(%rbp,%rdi,4), %xmm11
.Ltmp6198:
	.loc	21 373 5
	vmovss	%xmm4, 860(%r14)
	.loc	21 382 5
	movl	%r11d, 856(%r14)
.Ltmp6199:
	.loc	21 394 5
	vmovss	%xmm7, 864(%r14)
.Ltmp6200:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp6201:
	.loc	26 139 9
	cmovbel	%edx, %r10d
.Ltmp6202:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_345
.Ltmp6203:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm14, %xmm13, %xmm13
.LBB32_345:
	movq	72(%rsp), %r11
	movq	8(%rsp), %rsi
	movq	128(%rsp), %r13
	vmovss	120(%rsp), %xmm5
.Ltmp6204:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r12b
	jne	.LBB32_347
.Ltmp6205:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm5
.LBB32_347:
	vmovss	%xmm4, 32(%rsp)
.Ltmp6206:
	vmulss	.LCPI32_20(%rip), %xmm7, %xmm14
	vmaxss	.LCPI32_21(%rip), %xmm14, %xmm14
	vminss	.LCPI32_22(%rip), %xmm14, %xmm15
	vroundss	$9, %xmm15, %xmm15, %xmm14
	vsubss	%xmm14, %xmm15, %xmm15
.Ltmp6207:
	orl	%r12d, %r10d
	andl	$1065353216, %r10d
	vmovd	%r10d, %xmm6
	vmovss	%xmm5, 160(%rsp)
.Ltmp6208:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm5, 936(%r14)
	.loc	21 382 5
	movl	%r10d, 932(%r14)
.Ltmp6209:
	.loc	26 66 9
	vsubss	%xmm9, %xmm1, %xmm1
.Ltmp6210:
	.loc	26 71 9
	vmulss	116(%rsp), %xmm1, %xmm1
.Ltmp6211:
	.loc	26 161 24
	vmaxss	368(%rsp), %xmm1, %xmm1
.Ltmp6212:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm6, %xmm12, %xmm6
	vcmpltss	%xmm12, %xmm1, %xmm5
	vandps	%xmm5, %xmm6, %xmm5
	vmovd	%xmm5, %edi
	testb	$1, %dil
	jne	.LBB32_349
.Ltmp6213:
	.loc	26 0 44
	vxorps	%xmm1, %xmm1, %xmm1
	jmp	.LBB32_349
.LBB32_307:
	movq	%rdx, 152(%rsp)
.Ltmp6214:
	.loc	25 451 16 is_stmt 1
	movl	%r10d, %edx
	subl	%ecx, %edx
	movq	%rdx, 32(%rsp)
	movl	%r10d, %r9d
	subl	%esi, %r9d
	xorl	%edx, %edx
	vmovss	.LCPI32_5(%rip), %xmm10
	vmovss	.LCPI32_6(%rip), %xmm12
	vmovss	.LCPI32_7(%rip), %xmm15
	xorl	%ecx, %ecx
	jmp	.LBB32_308
.Ltmp6215:
	.loc	25 0 16 is_stmt 0
.Ltmp6216:
	.p2align	4
.LBB32_326:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm14
	movl	112(%rsp), %edi
.Ltmp6217:
	.loc	26 161 24
	cmovbel	108(%rsp), %edi
.Ltmp6218:
	.loc	7 1291 18
	vmovd	%edi, %xmm1
.Ltmp6219:
	.loc	26 66 9
	vsubss	%xmm2, %xmm14, %xmm14
.Ltmp6220:
	.loc	26 92 9
	vmulss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm1, %xmm2, %xmm1
.Ltmp6221:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm2
	vandps	%xmm2, %xmm1, %xmm2
.Ltmp6222:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm2, %xmm2
	vandps	%xmm1, %xmm2, %xmm2
	vmovss	.LCPI32_23(%rip), %xmm14
.Ltmp6223:
	.loc	26 71 9
	vmulss	%xmm3, %xmm14, %xmm1
	vmovss	.LCPI32_24(%rip), %xmm15
.Ltmp6224:
	.loc	26 61 9
	vaddss	%xmm1, %xmm15, %xmm1
.Ltmp6225:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm1
	vmovss	.LCPI32_25(%rip), %xmm9
.Ltmp6226:
	.loc	26 61 9
	vaddss	%xmm1, %xmm9, %xmm1
.Ltmp6227:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm1
	vmovss	.LCPI32_26(%rip), %xmm10
.Ltmp6228:
	.loc	26 61 9
	vaddss	%xmm1, %xmm10, %xmm1
.Ltmp6229:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm1
	vmovss	.LCPI32_27(%rip), %xmm8
.Ltmp6230:
	.loc	26 61 9
	vaddss	%xmm1, %xmm8, %xmm1
.Ltmp6231:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm1
.Ltmp6232:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI32_20(%rip), %xmm2, %xmm3
.Ltmp6233:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI32_21(%rip), %xmm3, %xmm3
.Ltmp6234:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm3, %xmm3
	vmovss	.LCPI32_2(%rip), %xmm0
.Ltmp6235:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm0, %xmm1, %xmm1
	vmovss	.LCPI32_28(%rip), %xmm7
.Ltmp6236:
	.loc	26 178 22
	vaddss	%xmm7, %xmm12, %xmm12
.Ltmp6237:
	.loc	7 1244 18
	vmovd	%xmm12, %edi
.Ltmp6238:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp6239:
	.loc	7 1291 18
	vmovd	%edi, %xmm12
.Ltmp6240:
	.loc	26 71 9
	vmulss	%xmm1, %xmm12, %xmm1
.Ltmp6241:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm11, %xmm1
.Ltmp6242:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm6, %xmm13, %xmm12
	vblendvps	%xmm12, %xmm1, %xmm11, %xmm1
	vcmpnltss	88(%rsp), %xmm6, %xmm12
	vblendvps	%xmm12, %xmm1, %xmm11, %xmm1
.Ltmp6243:
	.loc	7 1783 9
	vroundss	$9, %xmm3, %xmm3, %xmm11
.Ltmp6244:
	.loc	26 66 9
	vsubss	%xmm11, %xmm3, %xmm3
.Ltmp6245:
	.loc	26 71 9
	vmulss	%xmm3, %xmm14, %xmm12
.Ltmp6246:
	.loc	26 61 9
	vaddss	%xmm15, %xmm12, %xmm12
.Ltmp6247:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm12
.Ltmp6248:
	.loc	26 61 9
	vaddss	%xmm9, %xmm12, %xmm12
.Ltmp6249:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm12
.Ltmp6250:
	.loc	26 61 9
	vaddss	%xmm10, %xmm12, %xmm12
.Ltmp6251:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm12
.Ltmp6252:
	.loc	26 61 9
	vaddss	%xmm8, %xmm12, %xmm12
.Ltmp6253:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
.Ltmp6254:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp6255:
	.loc	26 178 22
	vaddss	%xmm7, %xmm11, %xmm11
	vmovaps	%xmm13, %xmm7
.Ltmp6256:
	.loc	7 1244 18
	vmovd	%xmm11, %edi
.Ltmp6257:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp6258:
	.loc	7 1291 18
	vmovd	%edi, %xmm11
.Ltmp6259:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp6260:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm5, %xmm3
.Ltmp6261:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm6, %xmm2, %xmm11
	vblendvps	%xmm11, %xmm3, %xmm5, %xmm3
	vcmpnltss	104(%rsp), %xmm6, %xmm6
	vblendvps	%xmm6, %xmm3, %xmm5, %xmm3
.Ltmp6262:
	.loc	21 394 5
	vmovss	%xmm2, 940(%r14)
	movq	56(%rsp), %r12
.Ltmp6263:
	.loc	26 56 9
	vmovss	%xmm1, (%r12,%rcx,4)
.Ltmp6264:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, (%r13,%rcx,4)
	incq	%rcx
.Ltmp6265:
	.loc	8 1916 50 is_stmt 1
	cmpq	%rcx, 320(%rsp)
	movq	48(%rsp), %r10
	vmovss	.LCPI32_3(%rip), %xmm14
	vmovss	.LCPI32_6(%rip), %xmm12
	vmovss	.LCPI32_5(%rip), %xmm10
	vmovss	.LCPI32_7(%rip), %xmm15
	vmovss	16(%rsp), %xmm9
	vmovss	100(%rsp), %xmm0
	vmovss	160(%rsp), %xmm13
.Ltmp6266:
	.loc	11 900 12
	je	.LBB32_327
.Ltmp6267:
.LBB32_308:
	.loc	21 246 22
	leal	(%r10,%rcx), %edi
	andl	%ebx, %edi
.Ltmp6268:
	.loc	25 451 16
	cmpq	%rdi, %rax
	jbe	.LBB32_305
.Ltmp6269:
	.loc	26 51 9
	vmovss	(%r12,%rcx,4), %xmm3
.Ltmp6270:
	.loc	26 56 9
	vmovss	%xmm3, (%r8,%rdi,4)
.Ltmp6271:
	.loc	26 51 9
	vmovss	(%r13,%rcx,4), %xmm3
.Ltmp6272:
	.loc	26 56 9
	vmovss	%xmm3, (%rbp,%rdi,4)
.Ltmp6273:
	.loc	21 255 21
	leal	(%r9,%rcx), %edi
	andl	%ebx, %edi
.Ltmp6274:
	.loc	25 438 16
	cmpq	%rdi, %rax
	jbe	.LBB32_381
.Ltmp6275:
	.loc	25 0 16 is_stmt 0
	movq	32(%rsp), %rsi
	leal	(%rsi,%rcx), %r11d
	andl	%ebx, %r11d
	cmpq	%r11, %rax
	jbe	.LBB32_375
	movq	152(%rsp), %rsi
	leal	(%rsi,%rcx), %r10d
	andl	%ebx, %r10d
	cmpq	%r10, %r15
	jbe	.LBB32_373
	cmpq	%r10, %rax
.Ltmp6276:
	.loc	21 277 29 is_stmt 1
	jbe	.LBB32_372
.Ltmp6277:
	.loc	21 0 29 is_stmt 0
	vmovss	(%rbp,%r11,4), %xmm3
.Ltmp6278:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r8,%r11,4), %xmm5
	vbroadcastss	.LCPI32_4(%rip), %xmm1
	vandps	%xmm1, %xmm5, %xmm5
.Ltmp6279:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp6280:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm3, %xmm5
.Ltmp6281:
	.loc	7 1244 18
	vmovd	%xmm5, %r11d
.Ltmp6282:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm3, %r12d
.Ltmp6283:
	.loc	26 161 24 is_stmt 1
	cmoval	%r11d, %r12d
	vxorps	%xmm6, %xmm6, %xmm6
	vmovss	80(%rsp), %xmm1
	vucomiss	%xmm6, %xmm1
.Ltmp6284:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r11d, %r12d
	vmovss	4(%rsp), %xmm1
	vucomiss	%xmm6, %xmm1
.Ltmp6285:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm10, %xmm5
.Ltmp6286:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm10, %xmm3
.Ltmp6287:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp6288:
	.loc	7 1244 18
	vmovd	%xmm3, %r11d
.Ltmp6289:
	.loc	26 161 24
	cmovbel	%r12d, %r11d
.Ltmp6290:
	.loc	7 1291 18
	vmovd	%r11d, %xmm3
.Ltmp6291:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm3
.Ltmp6292:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %r11d
.Ltmp6293:
	.loc	7 1291 18
	vmovd	%r11d, %xmm3
.Ltmp6294:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm3
.Ltmp6295:
	.loc	26 161 24
	movl	$8388608, %esi
	cmovbel	%esi, %r11d
.Ltmp6296:
	.loc	26 185 42
	movl	%r11d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp6297:
	.loc	7 1291 18
	vmovd	%r12d, %xmm3
.Ltmp6298:
	.loc	26 66 9
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp6299:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm3, %xmm5
.Ltmp6300:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm1
	vsubss	%xmm5, %xmm1, %xmm5
.Ltmp6301:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm5
.Ltmp6302:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm5, %xmm5
.Ltmp6303:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm5
.Ltmp6304:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm5, %xmm5
.Ltmp6305:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm5
.Ltmp6306:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm5, %xmm5
.Ltmp6307:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm5
.Ltmp6308:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm5, %xmm5
.Ltmp6309:
	.loc	26 187 28
	shrl	$23, %r11d
	orl	$1258291200, %r11d
.Ltmp6310:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm3
.Ltmp6311:
	.loc	7 1291 18
	vmovd	%r11d, %xmm5
.Ltmp6312:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm5, %xmm5
.Ltmp6313:
	.loc	26 61 9
	vaddss	%xmm3, %xmm5, %xmm3
.Ltmp6314:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp6315:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
.Ltmp6316:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm3, %xmm3
.Ltmp6317:
	.loc	26 129 14 is_stmt 1
	vucomiss	192(%rsp), %xmm3
.Ltmp6318:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp6319:
	.loc	26 129 14
	vucomiss	%xmm0, %xmm3
.Ltmp6320:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6321:
	.loc	26 149 9
	movl	%r13d, %r11d
.Ltmp6322:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm4
.Ltmp6323:
	.loc	26 149 9
	notl	%r11d
.Ltmp6324:
	.loc	26 139 9
	cmovbel	%edx, %r11d
.Ltmp6325:
	.loc	21 362 20
	vmovss	856(%r14), %xmm5
.Ltmp6326:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm5
.Ltmp6327:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp6328:
	.loc	26 139 9
	cmovbel	%edx, %r11d
.Ltmp6329:
	.loc	26 161 24
	testb	$1, %r11b
	jne	.LBB32_314
.Ltmp6330:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm5
	vmovss	28(%rsp), %xmm4
.Ltmp6331:
	.loc	26 161 24
	testb	$1, %r12b
	je	.LBB32_317
	jmp	.LBB32_318
.Ltmp6332:
	.loc	26 0 24
.Ltmp6333:
	.p2align	4
.LBB32_314:
	vaddss	%xmm4, %xmm14, %xmm5
	vmovss	28(%rsp), %xmm4
.Ltmp6334:
	.loc	26 161 24
	testb	$1, %r12b
	jne	.LBB32_318
.Ltmp6335:
.LBB32_317:
	.loc	26 0 24
	vmovaps	%xmm5, %xmm4
.LBB32_318:
	orl	%r12d, %r11d
	andl	$1065353216, %r11d
	vmovd	%r11d, %xmm5
.Ltmp6336:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm0, %xmm3, %xmm3
.Ltmp6337:
	.loc	26 71 9
	vmulss	24(%rsp), %xmm3, %xmm3
.Ltmp6338:
	.loc	26 161 24
	vmaxss	336(%rsp), %xmm3, %xmm3
.Ltmp6339:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm5, %xmm6, %xmm5
	vcmpltss	%xmm6, %xmm3, %xmm11
	vandps	%xmm5, %xmm11, %xmm5
	vmovd	%xmm5, %r12d
	testb	$1, %r12b
	jne	.LBB32_320
.Ltmp6340:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_320:
.Ltmp6341:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
	movl	288(%rsp), %r12d
.Ltmp6342:
	.loc	26 161 24
	cmovbel	272(%rsp), %r12d
.Ltmp6343:
	.loc	26 103 24
	vmovss	(%rbp,%r10,4), %xmm5
	vbroadcastss	.LCPI32_4(%rip), %xmm8
	vandps	%xmm5, %xmm8, %xmm5
.Ltmp6344:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r8,%r10,4), %xmm11
	vandps	%xmm8, %xmm11, %xmm11
.Ltmp6345:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm11, %xmm5
.Ltmp6346:
	.loc	7 1244 18
	vmovd	%xmm5, %r10d
.Ltmp6347:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm11, %r13d
.Ltmp6348:
	.loc	26 161 24 is_stmt 1
	cmoval	%r10d, %r13d
	vmovss	176(%rsp), %xmm1
	vucomiss	%xmm6, %xmm1
.Ltmp6349:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r10d, %r13d
	vmovss	168(%rsp), %xmm1
	vucomiss	%xmm6, %xmm1
.Ltmp6350:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm10, %xmm5
.Ltmp6351:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm10, %xmm11, %xmm11
.Ltmp6352:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm5, %xmm11, %xmm5
.Ltmp6353:
	.loc	7 1244 18
	vmovd	%xmm5, %r10d
.Ltmp6354:
	.loc	26 161 24
	cmovbel	%r13d, %r10d
.Ltmp6355:
	.loc	7 1291 18
	vmovd	%r10d, %xmm5
.Ltmp6356:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm5
.Ltmp6357:
	.loc	7 1291 18
	vmovd	%r12d, %xmm5
.Ltmp6358:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %r10d
.Ltmp6359:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp6360:
	.loc	7 1291 18
	vmovd	%r10d, %xmm11
.Ltmp6361:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm11
.Ltmp6362:
	.loc	26 92 9
	vmulss	%xmm5, %xmm3, %xmm3
.Ltmp6363:
	.loc	26 161 24
	movl	$8388608, %esi
	cmovbel	%esi, %r10d
.Ltmp6364:
	.loc	26 185 42
	movl	%r10d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp6365:
	.loc	7 1291 18
	vmovd	%r12d, %xmm5
.Ltmp6366:
	.loc	26 66 9
	vaddss	%xmm5, %xmm14, %xmm5
.Ltmp6367:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm5, %xmm11
.Ltmp6368:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm1
	vsubss	%xmm11, %xmm1, %xmm11
.Ltmp6369:
	.loc	26 71 9
	vmulss	%xmm5, %xmm11, %xmm11
.Ltmp6370:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm11, %xmm11
.Ltmp6371:
	.loc	26 71 9
	vmulss	%xmm5, %xmm11, %xmm11
.Ltmp6372:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm11, %xmm11
.Ltmp6373:
	.loc	26 71 9
	vmulss	%xmm5, %xmm11, %xmm11
.Ltmp6374:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm11, %xmm11
.Ltmp6375:
	.loc	26 71 9
	vmulss	%xmm5, %xmm11, %xmm11
.Ltmp6376:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm11, %xmm11
.Ltmp6377:
	.loc	26 187 28
	shrl	$23, %r10d
	orl	$1258291200, %r10d
.Ltmp6378:
	.loc	26 71 9
	vmulss	%xmm5, %xmm11, %xmm5
.Ltmp6379:
	.loc	7 1291 18
	vmovd	%r10d, %xmm11
.Ltmp6380:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm11, %xmm11
.Ltmp6381:
	.loc	26 61 9
	vaddss	%xmm5, %xmm11, %xmm5
	vmovss	(%r8,%rdi,4), %xmm11
.Ltmp6382:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm5, %xmm5
.Ltmp6383:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm5, %xmm5
	vmovaps	%xmm14, %xmm1
.Ltmp6384:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm5, %xmm14
.Ltmp6385:
	.loc	26 129 14 is_stmt 1
	vucomiss	124(%rsp), %xmm14
.Ltmp6386:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp6387:
	.loc	26 129 14
	vucomiss	%xmm9, %xmm14
.Ltmp6388:
	.loc	26 92 9
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp6389:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6390:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%r14), %xmm5
.Ltmp6391:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r10d
.Ltmp6392:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm13
.Ltmp6393:
	.loc	26 149 9
	notl	%r10d
.Ltmp6394:
	.loc	26 139 9
	cmovbel	%edx, %r10d
.Ltmp6395:
	.loc	26 124 14
	vucomiss	%xmm6, %xmm5
.Ltmp6396:
	.loc	26 103 24
	vandps	%xmm3, %xmm8, %xmm5
.Ltmp6397:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm5, %xmm5
	vandps	%xmm3, %xmm5, %xmm7
	vmovss	(%rbp,%rdi,4), %xmm5
.Ltmp6398:
	.loc	21 373 5
	vmovss	%xmm4, 860(%r14)
	.loc	21 382 5
	movl	%r11d, 856(%r14)
.Ltmp6399:
	.loc	21 394 5
	vmovss	%xmm7, 864(%r14)
.Ltmp6400:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp6401:
	.loc	26 139 9
	cmovbel	%edx, %r10d
.Ltmp6402:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_322
.Ltmp6403:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm13, %xmm13
.LBB32_322:
	movq	72(%rsp), %r11
	movq	8(%rsp), %rsi
	movq	128(%rsp), %r13
	vmovss	120(%rsp), %xmm1
.Ltmp6404:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r12b
	jne	.LBB32_324
.Ltmp6405:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm1
.LBB32_324:
	vmovaps	%xmm7, %xmm13
.Ltmp6406:
	vmulss	.LCPI32_20(%rip), %xmm7, %xmm3
	vmaxss	.LCPI32_21(%rip), %xmm3, %xmm3
	vminss	.LCPI32_22(%rip), %xmm3, %xmm3
	vroundss	$9, %xmm3, %xmm3, %xmm12
	vsubss	%xmm12, %xmm3, %xmm3
.Ltmp6407:
	orl	%r12d, %r10d
	andl	$1065353216, %r10d
	vmovd	%r10d, %xmm15
	vmovss	%xmm1, 160(%rsp)
.Ltmp6408:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm1, 936(%r14)
	.loc	21 382 5
	movl	%r10d, 932(%r14)
.Ltmp6409:
	.loc	26 66 9
	vsubss	%xmm9, %xmm14, %xmm14
.Ltmp6410:
	.loc	26 71 9
	vmulss	116(%rsp), %xmm14, %xmm14
.Ltmp6411:
	.loc	26 161 24
	vmaxss	368(%rsp), %xmm14, %xmm14
.Ltmp6412:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm15, %xmm6, %xmm15
	vcmpltss	%xmm6, %xmm14, %xmm1
	vandps	%xmm1, %xmm15, %xmm1
	vmovd	%xmm1, %edi
	testb	$1, %dil
	jne	.LBB32_326
.Ltmp6413:
	.loc	26 0 44
	vxorps	%xmm14, %xmm14, %xmm14
	jmp	.LBB32_326
.LBB32_327:
	movq	320(%rsp), %rax
.Ltmp6414:
	.loc	15 2584 13 is_stmt 1
	addl	%r10d, %eax
.Ltmp6415:
	.loc	21 297 5
	movl	%eax, 1208(%r14)
	movq	352(%rsp), %rdx
	movq	64(%rsp), %r9
.Ltmp6416:
.LBB32_42:
	.loc	6 698 9
	subl	%esi, 1220(%r14)
.Ltmp6417:
	.loc	6 765 33
	leaq	224(%rsp), %r10
	movq	%r9, 224(%rsp)
	movq	%r11, 232(%rsp)
	movq	40(%rsp), %rax
	movq	%rax, 240(%rsp)
	movq	%rdx, 248(%rsp)
	movl	72(%r14), %eax
	movq	%rax, 16(%rsp)
	vxorps	%xmm8, %xmm8, %xmm8
	vcvtsi2sd	%rax, %xmm8, %xmm3
	movq	1200(%r14), %rbp
	movl	1216(%r14), %eax
	movl	%eax, 4(%rsp)
	leaq	360(%rsp), %r13
	xorl	%r15d, %r15d
	vbroadcastss	.LCPI32_4(%rip), %xmm4
	vmovss	.LCPI32_29(%rip), %xmm5
	leaq	328(%rsp), %r9
	vmovsd	.LCPI32_30(%rip), %xmm6
	vmovsd	.LCPI32_31(%rip), %xmm7
	vmovsd	.LCPI32_32(%rip), %xmm8
	xorl	%r12d, %r12d
	xorl	%eax, %eax
	vmovsd	%xmm3, 80(%rsp)
	vmovaps	%xmm4, 192(%rsp)
	jmp	.LBB32_43
	.loc	6 0 33 is_stmt 0
.Ltmp6418:
	.p2align	4
.LBB32_297:
	movq	(%r13), %rax
.Ltmp6419:
	.loc	15 2428 13 is_stmt 1
	addq	%r11, %rax
	movq	$-1, %rcx
	cmovbq	%rcx, %rax
.Ltmp6420:
	.loc	6 786 17
	movq	%rax, (%r13)
	leaq	328(%rsp), %r9
.Ltmp6421:
.LBB32_298:
	.loc	6 0 17 is_stmt 0
	movl	$1, %eax
	movq	%r9, %r13
.Ltmp6422:
	.loc	27 131 12 is_stmt 1
	testb	$1, %r12b
	movb	$1, %r12b
	jne	.LBB32_299
.Ltmp6423:
.LBB32_43:
	.loc	25 253 13
	movq	%rax, %rcx
	shlq	$4, %rcx
.Ltmp6424:
	.loc	1 1733 9
	movq	(%r10,%rcx), %rdx
	movq	8(%r10,%rcx), %rdi
.Ltmp6425:
	.loc	6 766 13
	imulq	$76, %rax, %rbx
	vmovss	864(%r14,%rbx), %xmm0
.Ltmp6426:
	.loc	16 2155 12
	testq	%rdi, %rdi
	je	.LBB32_281
.Ltmp6427:
	.loc	16 0 12 is_stmt 0
	movl	$-1, %esi
	xorl	%r8d, %r8d
	.p2align	4
.LBB32_45:
.Ltmp6428:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rdx,%r8,4), %xmm1
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp6429:
	.loc	26 114 14
	vucomiss	%xmm1, %xmm5
.Ltmp6430:
	.loc	26 139 9
	cmovbel	%r15d, %esi
.Ltmp6431:
	.loc	16 2155 12
	incq	%r8
	cmpq	%r8, %rdi
	jne	.LBB32_45
.Ltmp6432:
	.loc	16 0 12 is_stmt 0
	vandps	%xmm4, %xmm0, %xmm0
	vucomiss	%xmm0, %xmm5
	.loc	6 767 16 is_stmt 1
	jbe	.LBB32_48
	cmpl	$-1, %esi
	je	.LBB32_298
.LBB32_48:
	.loc	6 0 16 is_stmt 0
	movl	$-1, %esi
	xorl	%r8d, %r8d
	.p2align	4
.LBB32_49:
.Ltmp6433:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rdx,%r8,4), %xmm1
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp6434:
	.loc	26 114 14
	vucomiss	%xmm1, %xmm5
.Ltmp6435:
	.loc	26 139 9
	cmovbel	%r15d, %esi
.Ltmp6436:
	.loc	16 2155 12
	incq	%r8
	cmpq	%r8, %rdi
	jne	.LBB32_49
.Ltmp6437:
	.loc	28 185 12
	notl	%esi
	xorl	%r8d, %r8d
	testl	$1065353216, %esi
	setne	%r8b
	jmp	.LBB32_283
.Ltmp6438:
	.loc	28 0 12 is_stmt 0
.Ltmp6439:
	.p2align	4
.LBB32_281:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm4, %xmm0, %xmm0
.Ltmp6440:
	.loc	26 114 14
	vucomiss	%xmm0, %xmm5
.Ltmp6441:
	.loc	6 767 43
	ja	.LBB32_298
	.loc	6 0 43 is_stmt 0
	xorl	%r8d, %r8d
.LBB32_283:
.Ltmp6442:
	.loc	26 114 14 is_stmt 1
	xorl	%esi, %esi
	vucomiss	%xmm0, %xmm5
	setbe	%sil
	orl	%r8d, %esi
	je	.LBB32_298
.Ltmp6443:
	.loc	26 0 14 is_stmt 0
	testq	%r11, %r11
.Ltmp6444:
	.loc	11 900 12 is_stmt 1
	je	.LBB32_288
.Ltmp6445:
	.loc	11 0 12 is_stmt 0
	xorl	%esi, %esi
	.p2align	4
.LBB32_286:
.Ltmp6446:
	.loc	6 777 21 is_stmt 1
	cmpq	%rsi, %rdi
	je	.LBB32_300
	movl	$0, (%rdx,%rsi,4)
.Ltmp6447:
	.loc	15 971 17
	incq	%rsi
.Ltmp6448:
	.loc	8 1916 50
	cmpq	%rsi, %r11
.Ltmp6449:
	.loc	11 900 12
	jne	.LBB32_286
.Ltmp6450:
.LBB32_288:
	.loc	11 0 12 is_stmt 0
	movq	%rax, %rdx
	shlq	$6, %rdx
	testq	%rbp, %rbp
.Ltmp6451:
	.loc	11 900 12
	je	.LBB32_292
.Ltmp6452:
	.loc	11 0 12
	leaq	104(%r14), %rsi
	addq	%rdx, %rsi
	movq	8(%rsi), %rdi
	xorl	%r8d, %r8d
	.p2align	4
.LBB32_290:
.Ltmp6453:
	.loc	6 562 13 is_stmt 1
	cmpq	%r8, %rdi
	je	.LBB32_378
	movq	(%rsi), %r9
	movl	$0, (%r9,%r8,4)
.Ltmp6454:
	.loc	15 971 17
	incq	%r8
.Ltmp6455:
	.loc	8 1916 50
	cmpq	%r8, %rbp
.Ltmp6456:
	.loc	11 900 12
	jne	.LBB32_290
.Ltmp6457:
.LBB32_292:
	.loc	11 0 12 is_stmt 0
	movq	%rax, %rsi
	shlq	$5, %rsi
	leaq	232(%r14), %rdi
	addq	%rdi, %rsi
	leaq	944(%r14), %rdi
	addq	%rdi, %rcx
.Ltmp6458:
	.loc	6 506 22 is_stmt 1
	vmovss	(%rsi), %xmm12
	vmovss	4(%rsi), %xmm11
	vmovss	8(%rsi), %xmm10
	vmovss	12(%rsi), %xmm9
	vmovss	16(%rsi), %xmm0
	vmovss	20(%rsi), %xmm2
	vmovss	24(%rsi), %xmm13
	vmovss	28(%rsi), %xmm1
.Ltmp6459:
	.loc	6 507 9
	vmovss	%xmm1, (%rcx)
	vmovss	%xmm0, 4(%rcx)
	vmovss	%xmm2, 8(%rcx)
	vmovss	%xmm13, 12(%rcx)
.Ltmp6460:
	.loc	9 82 17
	vcvtss2sd	%xmm1, %xmm1, %xmm1
.Ltmp6461:
	.loc	6 403 20
	vmulsd	%xmm1, %xmm3, %xmm1
	vdivsd	%xmm6, %xmm1, %xmm1
	.loc	6 403 19 is_stmt 0
	vaddsd	%xmm7, %xmm1, %xmm1
.Ltmp6462:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm1, %xmm1, %xmm1
	vmovq	%xmm1, %rcx
.Ltmp6463:
	.loc	6 404 9
	testq	%rcx, %rcx
	sets	%sil
	movabsq	$9223372036854775807, %rdi
	andq	%rdi, %rcx
	movabsq	$-4503599627370496, %rdi
	addq	%rcx, %rdi
	shrq	$53, %rdi
	cmpl	$1023, %edi
	setb	%dil
	andb	%sil, %dil
	movabsq	$9218868437227405311, %rsi
	cmpq	%rsi, %rcx
	setg	%cl
	orb	%dil, %cl
	jne	.LBB32_297
	vucomisd	%xmm8, %xmm1
	ja	.LBB32_297
.Ltmp6464:
	.loc	9 82 17
	vcvtss2sd	%xmm2, %xmm2, %xmm2
.Ltmp6465:
	.loc	6 403 20
	vmulsd	%xmm2, %xmm3, %xmm2
	vdivsd	%xmm6, %xmm2, %xmm2
	.loc	6 403 19 is_stmt 0
	vaddsd	%xmm7, %xmm2, %xmm2
.Ltmp6466:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm2, %xmm2, %xmm2
	vmovq	%xmm2, %rcx
.Ltmp6467:
	.loc	6 404 9
	testq	%rcx, %rcx
	sets	%sil
	movabsq	$9223372036854775807, %rdi
	andq	%rdi, %rcx
	movabsq	$-4503599627370496, %rdi
	addq	%rcx, %rdi
	shrq	$53, %rdi
	cmpl	$1023, %edi
	setb	%dil
	andb	%sil, %dil
	movabsq	$9218868437227405311, %rsi
	cmpq	%rsi, %rcx
	setg	%cl
	orb	%dil, %cl
	jne	.LBB32_297
	vucomisd	%xmm8, %xmm2
	ja	.LBB32_297
.Ltmp6468:
	.loc	6 0 9 is_stmt 0
	addq	%r14, %rdx
	leaq	792(%r14), %rcx
	addq	%rcx, %rbx
	leaq	(%rax,%rax,2), %rax
	leaq	744(%r14), %rcx
	leaq	(%rcx,%rax,8), %rdi
	movq	%rdi, 40(%rsp)
	vxorps	%xmm5, %xmm5, %xmm5
.Ltmp6469:
	.loc	6 407 10 is_stmt 1
	vmaxsd	%xmm1, %xmm5, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rax
.Ltmp6470:
	.loc	6 407 10 is_stmt 0
	vmaxsd	%xmm2, %xmm5, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rcx
	movl	4(%rsp), %esi
.Ltmp6471:
	.loc	6 533 9 is_stmt 1
	subl	%eax, %esi
	cmovbl	%r15d, %esi
	movl	%esi, 136(%rdx)
	.loc	6 534 62
	movl	%ecx, %eax
	vcvtsi2ss	%rax, %xmm15, %xmm1
	vmovss	%xmm1, 8(%rsp)
.Ltmp6472:
	.loc	6 398 5
	vmovss	%xmm1, 8(%rdi)
	movq	16(%rsp), %rdi
	vmovss	%xmm9, 32(%rsp)
	vmovss	%xmm10, 64(%rsp)
	vmovss	%xmm11, 56(%rsp)
	vmovss	%xmm12, 128(%rsp)
	vmovss	%xmm13, 48(%rsp)
.Ltmp6473:
	.loc	6 538 13
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movq	40(%rsp), %rax
.Ltmp6474:
	.loc	6 398 5
	vmovss	%xmm0, (%rax)
	vmovss	48(%rsp), %xmm0
	movq	16(%rsp), %rdi
.Ltmp6475:
	.loc	6 543 13
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	vmovsd	.LCPI32_32(%rip), %xmm8
	vmovsd	.LCPI32_31(%rip), %xmm7
	vmovsd	.LCPI32_30(%rip), %xmm6
	vmovss	.LCPI32_29(%rip), %xmm5
	vmovaps	192(%rsp), %xmm4
	vmovsd	80(%rsp), %xmm3
	leaq	224(%rsp), %r10
	movq	72(%rsp), %r11
	movq	40(%rsp), %rax
.Ltmp6476:
	.loc	6 398 5
	vmovss	%xmm0, 4(%rax)
.Ltmp6477:
	.loc	6 398 5 is_stmt 0
	movl	$0, 72(%rbx)
.Ltmp6478:
	.loc	6 398 5
	movl	$1065353216, 64(%rbx)
	vmovss	8(%rsp), %xmm0
.Ltmp6479:
	.loc	6 398 5
	vmovss	%xmm0, 68(%rbx)
	vmovss	128(%rsp), %xmm0
.Ltmp6480:
	.loc	6 398 5
	vmovss	%xmm0, (%rbx)
.Ltmp6481:
	.loc	6 398 5
	vmovss	%xmm0, 4(%rbx)
.Ltmp6482:
	.loc	6 398 5
	movq	$0, 8(%rbx)
	vmovss	56(%rsp), %xmm0
.Ltmp6483:
	.loc	6 398 5
	vmovss	%xmm0, 16(%rbx)
.Ltmp6484:
	.loc	6 398 5
	vmovss	%xmm0, 20(%rbx)
.Ltmp6485:
	.loc	6 398 5
	movq	$0, 24(%rbx)
	vmovss	64(%rsp), %xmm0
.Ltmp6486:
	.loc	6 398 5
	vmovss	%xmm0, 32(%rbx)
.Ltmp6487:
	.loc	6 398 5
	vmovss	%xmm0, 36(%rbx)
.Ltmp6488:
	.loc	6 398 5
	movq	$0, 40(%rbx)
	vmovss	32(%rsp), %xmm0
.Ltmp6489:
	.loc	6 398 5
	vmovss	%xmm0, 48(%rbx)
.Ltmp6490:
	.loc	6 398 5
	vmovss	%xmm0, 52(%rbx)
.Ltmp6491:
	.loc	6 398 5
	movq	$0, 56(%rbx)
	jmp	.LBB32_297
.Ltmp6492:
.LBB32_299:
	.loc	6 1001 9 is_stmt 1
	vmovaps	416(%rsp), %xmm0
	movq	400(%rsp), %rax
	vmovups	%xmm0, (%rax)
	movq	304(%rsp), %rcx
	movq	%rcx, 16(%rax)
	movq	360(%rsp), %rcx
	movq	%rcx, 24(%rax)
	movq	328(%rsp), %rcx
	movq	%rcx, 32(%rax)
.Ltmp6493:
	.loc	6 1002 6 epilogue_begin
	addq	$440, %rsp
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
.LBB32_305:
	.cfi_def_cfa_offset 496
	.loc	6 0 6 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp6494:
	leaq	.Lalloc_cad5a7b24766ffcdda9cbfe066eb4078(%rip), %rcx
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_381:
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_ba99eeeb3482270ebe1c674b4013dd6a(%rip), %rcx
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_380:
	leaq	1(%rdi), %rsi
.Ltmp6495:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_72e24c5578221343e8a784545a3910d2(%rip), %rcx
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6496:
.LBB32_382:
	.loc	25 0 13 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp6497:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_0f4c27429e4885e1b619f1e4a3587acc(%rip), %rcx
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6498:
.LBB32_275:
	.loc	25 0 13 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp6499:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_72e24c5578221343e8a784545a3910d2(%rip), %rcx
	movq	%r10, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6500:
.LBB32_276:
	.loc	25 0 13 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp6501:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_0f4c27429e4885e1b619f1e4a3587acc(%rip), %rcx
	movq	%r10, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6502:
.LBB32_379:
	.loc	25 443 13
	leaq	.Lalloc_aebeae245abdbd708a3d39b73262e641(%rip), %rcx
	movq	392(%rsp), %rdi
	movq	%r9, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6503:
.LBB32_377:
	.loc	25 456 13
	leaq	.Lalloc_a0e50f2d670fa7dbeabdb68abcb7ac10(%rip), %rcx
	xorl	%edi, %edi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6504:
.LBB32_376:
	.loc	25 581 13
	leaq	.Lalloc_9c634077742a23e4340a846355ecc484(%rip), %rcx
	movq	%rsi, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6505:
.LBB32_300:
	.loc	6 777 21
	leaq	.Lalloc_30d570589940b68d7f77af9df77eb2ee(%rip), %rdx
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6506:
.LBB32_378:
	.loc	6 562 13
	leaq	.Lalloc_f8f0512af3f0ba047152c3c522a44b59(%rip), %rdx
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6507:
.LBB32_280:
	.loc	21 274 29
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r8, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_278:
	.loc	21 276 29
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%r9, %rdi
	movq	%r10, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6508:
.LBB32_375:
	.loc	21 274 29
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_373:
	.loc	21 276 29
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%r10, %rdi
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_372:
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%r10, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_374:
	.loc	21 275 29
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6509:
.LBB32_277:
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%r9, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_279:
	.loc	21 275 29
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r8, %rdi
	movq	%r10, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6510:
.Lfunc_end32:
	.size	_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb0_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_, .Lfunc_end32-_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb0_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_
