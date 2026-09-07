_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_:
.Lfunc_begin38:
	.loc	6 1003 0 is_stmt 1
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
.Ltmp7039:
	.loc	6 1005 31 prologue_end
	movq	32(%rdx), %r12
	movq	40(%rdx), %rax
	.loc	6 1005 49 is_stmt 0
	movq	80(%rdx), %rdx
	movl	$0, 24(%rsp)
	movl	$0, 32(%rsp)
	movl	$0, 40(%rsp)
	movl	$0, 48(%rsp)
	movl	$0, 56(%rsp)
	movl	$0, 64(%rsp)
	movl	$0, 72(%rsp)
	movl	$0, 80(%rsp)
.Ltmp7040:
	.loc	38 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7041:
	.loc	19 180 28
	je	.LBB38_42
.Ltmp7042:
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
.Ltmp7043:
	.loc	15 2428 13 is_stmt 1
	incq	%rbp
	movq	$-1, %rax
	cmoveq	%rax, %rbp
.Ltmp7044:
	.loc	34 82 9
	incq	%r14
.Ltmp7045:
	.loc	38 1714 9
	cmpq	%rbx, %r12
.Ltmp7046:
	.loc	19 180 28
	je	.LBB38_41
.Ltmp7047:
.LBB38_2:
	.loc	6 616 33
	movl	32(%r12), %eax
	.loc	6 616 27 is_stmt 0
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
.Ltmp7048:
	.loc	6 624 35 is_stmt 1
	movl	16(%r12), %r15d
.Ltmp7049:
	.loc	15 3178 26
	testl	%r15d, %r15d
.Ltmp7050:
	.loc	46 459 8
	js	.LBB38_91
.Ltmp7051:
	.loc	6 633 25
	cmpq	%rdi, %r14
	jae	.LBB38_91
	cmpl	$3, %r15d
	ja	.LBB38_91
	.loc	6 635 20
	cmpl	$1, 28(%r12)
	jne	.LBB38_91
	.loc	6 636 20
	cmpq	%rdx, (%r12)
	jne	.LBB38_91
	.loc	6 637 20
	cmpq	%rdx, 8(%r12)
	jne	.LBB38_91
	.loc	6 638 20
	vmovd	20(%r12), %xmm0
.Ltmp7052:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7053:
	.loc	6 638 20
	cmpl	%ecx, 24(%r12)
	jne	.LBB38_91
.Ltmp7054:
	.loc	6 0 0 is_stmt 0
	leal	(%rax,%r15,2), %eax
	movl	%eax, 12(%rsp)
.Ltmp7055:
	.loc	6 639 42 is_stmt 1
	leaq	(%r15,%r15,4), %rax
	movq	%rdi, 96(%rsp)
	leaq	(%r11,%rax,8), %rdi
	movq	%r10, 120(%rsp)
	movq	%r8, 112(%rsp)
	movq	%rdx, 104(%rsp)
	vmovdqa	%xmm0, 128(%rsp)
	.loc	6 639 20 is_stmt 0
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
.Ltmp7056:
	.loc	6 642 17 is_stmt 1
	cmpb	$0, (%r13,%r15,8)
	jne	.LBB38_91
.Ltmp7057:
	.loc	6 0 0 is_stmt 0
	movq	%rbp, 160(%rsp)
.Ltmp7058:
	.loc	12 110 8 is_stmt 1
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
.Ltmp7059:
	.loc	6 647 13
	movl	$1, (%r13,%r15,8)
	vmovss	%xmm0, 4(%r13,%r15,8)
.Ltmp7060:
	.loc	38 1714 9
	addq	$40, %r12
.Ltmp7061:
	.loc	19 180 28
	incq	%r14
	movl	$0, 16(%rsp)
	movl	12(%rsp), %eax
	movl	%eax, 20(%rsp)
.Ltmp7062:
	.loc	38 1714 9
	cmpq	%rbx, %r12
	movq	88(%rsp), %r15
.Ltmp7063:
	.loc	19 180 28
	jne	.LBB38_2
	jmp	.LBB38_42
.Ltmp7064:
.LBB38_41:
	.loc	6 0 0 is_stmt 0
	movq	%rbp, 160(%rsp)
	movq	88(%rsp), %r15
.Ltmp7065:
.LBB38_42:
	.loc	6 651 21 is_stmt 1
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
	.loc	6 651 26 is_stmt 0
	vmovd	28(%rsp), %xmm0
.Ltmp7066:
	.loc	6 654 39 is_stmt 1
	vmovd	792(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp7067:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7068:
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
.Ltmp7069:
.LBB38_49:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_50:
.Ltmp7070:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 792(%r10)
.Ltmp7071:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 796(%r10)
.Ltmp7072:
	.loc	6 401 5
	vmovss	%xmm1, 800(%r10)
.Ltmp7073:
	.loc	6 401 5
	vmovss	%xmm3, 804(%r10)
.Ltmp7074:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp7075:
	.loc	6 651 21
	cmpl	$1, 32(%rsp)
	jne	.LBB38_60
.LBB38_52:
	.loc	6 651 26 is_stmt 0
	vmovd	36(%rsp), %xmm0
.Ltmp7076:
	.loc	6 654 39 is_stmt 1
	vmovd	808(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp7077:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7078:
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
.Ltmp7079:
.LBB38_58:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_59:
.Ltmp7080:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 808(%r10)
.Ltmp7081:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 812(%r10)
.Ltmp7082:
	.loc	6 401 5
	vmovss	%xmm1, 816(%r10)
.Ltmp7083:
	.loc	6 401 5
	vmovss	%xmm3, 820(%r10)
.Ltmp7084:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp7085:
	.loc	6 651 21
	cmpl	$1, 40(%rsp)
	jne	.LBB38_69
.LBB38_61:
	.loc	6 651 26 is_stmt 0
	vmovd	44(%rsp), %xmm0
.Ltmp7086:
	.loc	6 654 39 is_stmt 1
	vmovd	824(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp7087:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7088:
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
.Ltmp7089:
.LBB38_67:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_68:
.Ltmp7090:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 824(%r10)
.Ltmp7091:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 828(%r10)
.Ltmp7092:
	.loc	6 401 5
	vmovss	%xmm1, 832(%r10)
.Ltmp7093:
	.loc	6 401 5
	vmovss	%xmm3, 836(%r10)
.Ltmp7094:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp7095:
	.loc	6 651 21
	cmpl	$1, 48(%rsp)
	jne	.LBB38_4
.LBB38_70:
	.loc	6 651 26 is_stmt 0
	vmovd	52(%rsp), %xmm0
.Ltmp7096:
	.loc	6 654 39 is_stmt 1
	vmovd	840(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp7097:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7098:
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
.Ltmp7099:
.LBB38_76:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_77:
.Ltmp7100:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 840(%r10)
.Ltmp7101:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 844(%r10)
.Ltmp7102:
	.loc	6 401 5
	vmovss	%xmm1, 848(%r10)
.Ltmp7103:
	.loc	6 401 5
	vmovss	%xmm3, 852(%r10)
.Ltmp7104:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp7105:
	.loc	6 651 21
	cmpl	$1, 56(%rsp)
	jne	.LBB38_13
.LBB38_5:
	.loc	6 651 26 is_stmt 0
	vmovd	60(%rsp), %xmm0
.Ltmp7106:
	.loc	6 654 39 is_stmt 1
	vmovd	868(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp7107:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7108:
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
.Ltmp7109:
.LBB38_11:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_12:
.Ltmp7110:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 868(%r10)
.Ltmp7111:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 872(%r10)
.Ltmp7112:
	.loc	6 401 5
	vmovss	%xmm1, 876(%r10)
.Ltmp7113:
	.loc	6 401 5
	vmovss	%xmm3, 880(%r10)
.Ltmp7114:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp7115:
	.loc	6 651 21
	cmpl	$1, 64(%rsp)
	jne	.LBB38_22
.LBB38_14:
	.loc	6 651 26 is_stmt 0
	vmovd	68(%rsp), %xmm0
.Ltmp7116:
	.loc	6 654 39 is_stmt 1
	vmovd	884(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp7117:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7118:
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
.Ltmp7119:
.LBB38_20:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_21:
.Ltmp7120:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 884(%r10)
.Ltmp7121:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 888(%r10)
.Ltmp7122:
	.loc	6 401 5
	vmovss	%xmm1, 892(%r10)
.Ltmp7123:
	.loc	6 401 5
	vmovss	%xmm3, 896(%r10)
.Ltmp7124:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp7125:
	.loc	6 651 21
	cmpl	$1, 72(%rsp)
	jne	.LBB38_31
.LBB38_23:
	.loc	6 651 26 is_stmt 0
	vmovd	76(%rsp), %xmm0
.Ltmp7126:
	.loc	6 654 39 is_stmt 1
	vmovd	900(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp7127:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7128:
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
.Ltmp7129:
.LBB38_29:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_30:
.Ltmp7130:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 900(%r10)
.Ltmp7131:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 904(%r10)
.Ltmp7132:
	.loc	6 401 5
	vmovss	%xmm1, 908(%r10)
.Ltmp7133:
	.loc	6 401 5
	vmovss	%xmm3, 912(%r10)
.Ltmp7134:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp7135:
	.loc	6 651 21
	cmpl	$1, 80(%rsp)
	jne	.LBB38_40
.LBB38_32:
	.loc	6 651 26 is_stmt 0
	vmovd	84(%rsp), %xmm0
.Ltmp7136:
	.loc	6 654 39 is_stmt 1
	vmovd	916(%r10), %xmm1
	vmovd	%xmm1, %eax
.Ltmp7137:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7138:
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
.Ltmp7139:
.LBB38_38:
	.loc	47 0 9
	vmovss	.LCPI38_1(%rip), %xmm3
.LBB38_39:
.Ltmp7140:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 916(%r10)
.Ltmp7141:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 920(%r10)
.Ltmp7142:
	.loc	6 401 5
	vmovss	%xmm1, 924(%r10)
.Ltmp7143:
	.loc	6 401 5
	vmovss	%xmm3, 928(%r10)
.Ltmp7144:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r10)
.Ltmp7145:
.LBB38_40:
	.loc	20 1095 9
	movq	(%r8), %rsi
	movq	8(%r8), %rdx
.Ltmp7146:
	.loc	6 1007 40
	leaq	48(%r8), %r9
.Ltmp7147:
	.loc	6 1008 36
	movq	16(%r8), %rcx
	movq	24(%r8), %r8
	leaq	144(%rsp), %rax
	.loc	6 1008 14 is_stmt 0
	movq	%r10, %rdi
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	%rdx
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_
	addq	$16, %rsp
	.cfi_adjust_cfa_offset -16
	.loc	6 1009 9 is_stmt 1
	movq	176(%rsp), %rax
	movq	%rax, 32(%r15)
	vmovups	144(%rsp), %ymm0
	vmovups	%ymm0, (%r15)
.Ltmp7148:
	.loc	6 1010 6
	movq	%r15, %rax
	.loc	6 1010 6 epilogue_begin is_stmt 0
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
.Ltmp7149:
.Lfunc_end38:
	.size	_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_, .Lfunc_end38-_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_
