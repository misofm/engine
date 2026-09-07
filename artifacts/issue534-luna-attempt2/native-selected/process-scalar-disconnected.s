_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb0_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_:
.Lfunc_begin32:
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
	subq	$408, %rsp
	.cfi_def_cfa_offset 464
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r9
	movq	%rsi, %r14
	movq	%rdi, 352(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%xmm0, 384(%rsp)
	movq	$0, 312(%rsp)
	movq	$0, 248(%rsp)
.Ltmp5274:
	.loc	6 1005 31 prologue_end
	movq	32(%rdx), %rbx
	movq	40(%rdx), %rax
	.loc	6 1005 49 is_stmt 0
	movq	80(%rdx), %rdx
	movl	$0, 160(%rsp)
	movl	$0, 168(%rsp)
	movl	$0, 176(%rsp)
	movl	$0, 184(%rsp)
	movl	$0, 192(%rsp)
	movl	$0, 200(%rsp)
	movl	$0, 208(%rsp)
	movl	$0, 216(%rsp)
.Ltmp5275:
	.loc	38 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp5276:
	.loc	19 180 28
	je	.LBB32_1
.Ltmp5277:
	.loc	19 0 28 is_stmt 0
	leaq	192(%rsp), %rsi
	leaq	(%rax,%rax,4), %rax
	leaq	(%rbx,%rax,8), %r8
	movl	92(%r14), %r10d
	movb	$1, %al
	movl	%eax, (%rsp)
	xorl	%r12d, %r12d
	leaq	160(%rsp), %r11
	xorl	%eax, %eax
	xorl	%r13d, %r13d
.LBB32_8:
	movq	%rax, %rdi
	jmp	.LBB32_9
	.p2align	4
.LBB32_103:
	addq	$40, %rbx
.Ltmp5278:
	.loc	15 2428 13 is_stmt 1
	incq	%rdi
	movq	$-1, %rax
	cmoveq	%rax, %rdi
.Ltmp5279:
	.loc	34 82 9
	incq	%r13
	movq	%rdi, %r12
.Ltmp5280:
	.loc	38 1714 9
	cmpq	%r8, %rbx
.Ltmp5281:
	.loc	19 180 28
	je	.LBB32_2
.Ltmp5282:
.LBB32_9:
	.loc	6 616 33
	movl	32(%rbx), %eax
	.loc	6 616 27 is_stmt 0
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
.Ltmp5283:
	.loc	6 624 35 is_stmt 1
	movl	16(%rbx), %ebp
.Ltmp5284:
	.loc	15 3178 26
	testl	%ebp, %ebp
.Ltmp5285:
	.file	46 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/intrinsics/mod.rs"
	.loc	46 459 8
	js	.LBB32_103
.Ltmp5286:
	.loc	6 633 25
	cmpq	%r10, %r13
	jae	.LBB32_103
	cmpl	$3, %ebp
	ja	.LBB32_103
	.loc	6 635 20
	cmpl	$1, 28(%rbx)
	jne	.LBB32_103
	.loc	6 636 20
	cmpq	%rdx, (%rbx)
	jne	.LBB32_103
	.loc	6 637 20
	cmpq	%rdx, 8(%rbx)
	jne	.LBB32_103
	.loc	6 638 20
	vmovd	20(%rbx), %xmm0
.Ltmp5287:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5288:
	.loc	6 638 20
	cmpl	%ecx, 24(%rbx)
	jne	.LBB32_103
.Ltmp5289:
	.loc	6 0 0 is_stmt 0
	leal	(%rax,%rbp,2), %eax
	movl	%eax, 8(%rsp)
.Ltmp5290:
	.loc	6 639 42 is_stmt 1
	leaq	(,%rbp,4), %rax
	addq	%rbp, %rax
	movq	%rdi, 240(%rsp)
	leaq	.Lalloc_d0058eaee52f1ba8b2e5577cef0c3c7f(%rip), %rcx
	leaq	(%rcx,%rax,8), %rdi
	movq	%r9, 112(%rsp)
	movq	%rdx, 96(%rsp)
	movq	%r8, 32(%rsp)
	movq	%r10, 24(%rsp)
	vmovdqa	%xmm0, 224(%rsp)
	.loc	6 639 20 is_stmt 0
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	224(%rsp), %xmm1
	leaq	160(%rsp), %r11
	movq	24(%rsp), %r10
	movq	32(%rsp), %r8
	leaq	192(%rsp), %rsi
	movq	96(%rsp), %rdx
	movq	112(%rsp), %r9
	movq	240(%rsp), %rdi
	movl	8(%rsp), %ecx
	cmpl	16(%rsp), %ecx
	seta	%cl
	testb	%al, %al
	je	.LBB32_103
	orb	(%rsp), %cl
	testb	$1, %cl
	je	.LBB32_103
.Ltmp5291:
	.loc	6 642 17 is_stmt 1
	cmpb	$0, (%r15,%rbp,8)
	jne	.LBB32_103
.Ltmp5292:
	.loc	12 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
.Ltmp5293:
	.loc	6 647 13
	movl	$1, (%r15,%rbp,8)
	vmovss	%xmm0, 4(%r15,%rbp,8)
.Ltmp5294:
	.loc	38 1714 9
	addq	$40, %rbx
.Ltmp5295:
	.loc	19 180 28
	incq	%r13
	movl	$0, (%rsp)
	movq	%rdi, %rax
	movl	8(%rsp), %ecx
	movl	%ecx, 16(%rsp)
	movq	%r12, %rdi
.Ltmp5296:
	.loc	38 1714 9
	cmpq	%r8, %rbx
.Ltmp5297:
	.loc	19 180 28
	jne	.LBB32_8
	jmp	.LBB32_2
.Ltmp5298:
.LBB32_1:
	.loc	19 0 28 is_stmt 0
	xorl	%edi, %edi
.LBB32_2:
.Ltmp5299:
	.loc	6 651 21 is_stmt 1
	cmpl	$1, 160(%rsp)
	je	.LBB32_3
	cmpl	$1, 168(%rsp)
	je	.LBB32_64
.LBB32_72:
	cmpl	$1, 176(%rsp)
	je	.LBB32_73
.LBB32_81:
	cmpl	$1, 184(%rsp)
	je	.LBB32_82
.LBB32_11:
	cmpl	$1, 192(%rsp)
	je	.LBB32_12
.LBB32_20:
	cmpl	$1, 200(%rsp)
	je	.LBB32_21
.LBB32_29:
	cmpl	$1, 208(%rsp)
	je	.LBB32_30
.LBB32_38:
	cmpb	$0, 216(%rsp)
	je	.LBB32_39
.LBB32_51:
	.loc	6 651 26 is_stmt 0
	vmovd	220(%rsp), %xmm0
.Ltmp5300:
	.loc	6 654 39 is_stmt 1
	vmovd	916(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5301:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5302:
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
.Ltmp5303:
.LBB32_3:
	.loc	6 651 26 is_stmt 1
	vmovd	164(%rsp), %xmm0
.Ltmp5304:
	.loc	6 654 39
	vmovd	792(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5305:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5306:
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
.Ltmp5307:
.LBB32_57:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_58:
.Ltmp5308:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 916(%r14)
.Ltmp5309:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 920(%r14)
.Ltmp5310:
	.loc	6 401 5
	vmovss	%xmm1, 924(%r14)
.Ltmp5311:
	.loc	6 401 5
	vmovss	%xmm3, 928(%r14)
.Ltmp5312:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r14)
	movl	$64, %r8d
.Ltmp5313:
	.loc	6 650 13
	jmp	.LBB32_40
.LBB32_6:
	.loc	6 0 13 is_stmt 0
	vsubss	%xmm1, %xmm0, %xmm1
	vmulss	.LCPI32_0(%rip), %xmm1, %xmm1
.Ltmp5314:
	.loc	47 112 9 is_stmt 1
	je	.LBB32_62
.Ltmp5315:
.LBB32_61:
	.loc	47 0 9 is_stmt 0
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_62:
.Ltmp5316:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 792(%r14)
.Ltmp5317:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 796(%r14)
.Ltmp5318:
	.loc	6 401 5
	vmovss	%xmm1, 800(%r14)
.Ltmp5319:
	.loc	6 401 5
	vmovss	%xmm3, 804(%r14)
.Ltmp5320:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5321:
	.loc	6 651 21
	cmpl	$1, 168(%rsp)
	jne	.LBB32_72
.LBB32_64:
	.loc	6 651 26 is_stmt 0
	vmovd	172(%rsp), %xmm0
.Ltmp5322:
	.loc	6 654 39 is_stmt 1
	vmovd	808(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5323:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5324:
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
.Ltmp5325:
.LBB32_70:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_71:
.Ltmp5326:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 808(%r14)
.Ltmp5327:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 812(%r14)
.Ltmp5328:
	.loc	6 401 5
	vmovss	%xmm1, 816(%r14)
.Ltmp5329:
	.loc	6 401 5
	vmovss	%xmm3, 820(%r14)
.Ltmp5330:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5331:
	.loc	6 651 21
	cmpl	$1, 176(%rsp)
	jne	.LBB32_81
.LBB32_73:
	.loc	6 651 26 is_stmt 0
	vmovd	180(%rsp), %xmm0
.Ltmp5332:
	.loc	6 654 39 is_stmt 1
	vmovd	824(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5333:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5334:
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
.Ltmp5335:
.LBB32_79:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_80:
.Ltmp5336:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 824(%r14)
.Ltmp5337:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 828(%r14)
.Ltmp5338:
	.loc	6 401 5
	vmovss	%xmm1, 832(%r14)
.Ltmp5339:
	.loc	6 401 5
	vmovss	%xmm3, 836(%r14)
.Ltmp5340:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5341:
	.loc	6 651 21
	cmpl	$1, 184(%rsp)
	jne	.LBB32_11
.LBB32_82:
	.loc	6 651 26 is_stmt 0
	vmovd	188(%rsp), %xmm0
.Ltmp5342:
	.loc	6 654 39 is_stmt 1
	vmovd	840(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5343:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5344:
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
.Ltmp5345:
.LBB32_88:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_89:
.Ltmp5346:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 840(%r14)
.Ltmp5347:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 844(%r14)
.Ltmp5348:
	.loc	6 401 5
	vmovss	%xmm1, 848(%r14)
.Ltmp5349:
	.loc	6 401 5
	vmovss	%xmm3, 852(%r14)
.Ltmp5350:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5351:
	.loc	6 651 21
	cmpl	$1, 192(%rsp)
	jne	.LBB32_20
.LBB32_12:
	.loc	6 651 26 is_stmt 0
	vmovd	196(%rsp), %xmm0
.Ltmp5352:
	.loc	6 654 39 is_stmt 1
	vmovd	868(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5353:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5354:
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
.Ltmp5355:
.LBB32_18:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_19:
.Ltmp5356:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 868(%r14)
.Ltmp5357:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 872(%r14)
.Ltmp5358:
	.loc	6 401 5
	vmovss	%xmm1, 876(%r14)
.Ltmp5359:
	.loc	6 401 5
	vmovss	%xmm3, 880(%r14)
.Ltmp5360:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5361:
	.loc	6 651 21
	cmpl	$1, 200(%rsp)
	jne	.LBB32_29
.LBB32_21:
	.loc	6 651 26 is_stmt 0
	vmovd	204(%rsp), %xmm0
.Ltmp5362:
	.loc	6 654 39 is_stmt 1
	vmovd	884(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5363:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5364:
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
.Ltmp5365:
.LBB32_27:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_28:
.Ltmp5366:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 884(%r14)
.Ltmp5367:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 888(%r14)
.Ltmp5368:
	.loc	6 401 5
	vmovss	%xmm1, 892(%r14)
.Ltmp5369:
	.loc	6 401 5
	vmovss	%xmm3, 896(%r14)
.Ltmp5370:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5371:
	.loc	6 651 21
	cmpl	$1, 208(%rsp)
	jne	.LBB32_38
.LBB32_30:
	.loc	6 651 26 is_stmt 0
	vmovd	212(%rsp), %xmm0
.Ltmp5372:
	.loc	6 654 39 is_stmt 1
	vmovd	900(%r14), %xmm1
	vmovd	%xmm1, %eax
.Ltmp5373:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp5374:
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
.Ltmp5375:
.LBB32_36:
	.loc	47 0 9
	vmovss	.LCPI32_1(%rip), %xmm3
.LBB32_37:
.Ltmp5376:
	.loc	6 401 5 is_stmt 1
	vmovd	%xmm2, 900(%r14)
.Ltmp5377:
	.loc	6 401 5 is_stmt 0
	vmovss	%xmm0, 904(%r14)
.Ltmp5378:
	.loc	6 401 5
	vmovss	%xmm1, 908(%r14)
.Ltmp5379:
	.loc	6 401 5
	vmovss	%xmm3, 912(%r14)
.Ltmp5380:
	.loc	6 664 17 is_stmt 1
	movl	$64, 1220(%r14)
.Ltmp5381:
	.loc	6 651 21
	cmpb	$0, 216(%rsp)
	jne	.LBB32_51
.Ltmp5382:
.LBB32_39:
	.loc	6 685 25
	movl	1220(%r14), %r8d
.Ltmp5383:
.LBB32_40:
	.loc	20 1095 9
	movq	(%r9), %rcx
	movq	8(%r9), %r10
.Ltmp5384:
	.loc	6 1008 36
	movq	16(%r9), %rax
	movq	%rax, (%rsp)
	movq	24(%r9), %rdx
.Ltmp5385:
	.loc	8 1078 5
	cmpq	%r8, %r10
	movq	%r8, %rsi
	cmovbq	%r10, %rsi
.Ltmp5386:
	.loc	6 686 12
	testq	%rsi, %rsi
	movq	%rdi, 240(%rsp)
	movq	%r10, 144(%rsp)
	movq	%rdx, 304(%rsp)
	movq	%rsi, 136(%rsp)
	je	.LBB32_41
.Ltmp5387:
	.loc	25 451 16
	cmpq	%rdx, %rsi
	ja	.LBB32_438
.Ltmp5388:
	.loc	25 0 16 is_stmt 0
	movq	%r8, 32(%rsp)
.Ltmp5389:
	.loc	21 61 8 is_stmt 1
	movl	136(%r14), %eax
	cmpl	200(%r14), %eax
	sete	%al
	movb	$2, %dl
	subb	%al, %dl
	xorl	%r11d, %r11d
	cmpl	$1, 68(%r14)
	movzbl	%dl, %edi
	cmovel	%r11d, %edi
.Ltmp5390:
	.loc	6 736 31
	movq	104(%r14), %rbx
	movq	112(%r14), %rax
	.loc	6 741 31
	movq	168(%r14), %rdx
	movq	176(%r14), %r9
	.loc	6 747 28
	movl	1212(%r14), %r10d
	.loc	6 748 24
	movl	1216(%r14), %r13d
.Ltmp5391:
	.loc	21 353 16
	movl	1208(%r14), %r8d
	movl	%edi, 8(%rsp)
	movzbl	%dil, %edi
	cmpq	%r9, %rax
	movl	%r8d, 24(%rsp)
	jbe	.LBB32_106
.Ltmp5392:
	.loc	25 451 16
	negl	%r13d
	leaq	(,%rsi,4), %rsi
	movq	%rsi, 96(%rsp)
	vmovss	.LCPI32_2(%rip), %xmm0
	vmovss	.LCPI32_3(%rip), %xmm1
	movl	%r8d, %ebp
	xorl	%r15d, %r15d
	movl	%edi, %esi
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB32_203
.Ltmp5393:
	.loc	25 0 16 is_stmt 0
.Ltmp5394:
	.p2align	4
.LBB32_292:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%r14), %xmm3
.Ltmp5395:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm3, %xmm2
	setbe	%dil
.Ltmp5396:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp5397:
	.loc	26 92 9
	vmulss	768(%r14,%rdi,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5398:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp5399:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI32_23(%rip), %xmm11
.Ltmp5400:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI32_24(%rip), %xmm12
.Ltmp5401:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp5402:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI32_25(%rip), %xmm13
.Ltmp5403:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp5404:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI32_26(%rip), %xmm14
.Ltmp5405:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp5406:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI32_27(%rip), %xmm15
.Ltmp5407:
	.loc	26 61 9
	vaddss	%xmm2, %xmm15, %xmm2
.Ltmp5408:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp5409:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI32_28(%rip), %xmm7
.Ltmp5410:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp5411:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5412:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5413:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5414:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp5415:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm4, %xmm2
.Ltmp5416:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
.Ltmp5417:
	.loc	26 71 9
	vmulss	.LCPI32_20(%rip), %xmm10, %xmm3
.Ltmp5418:
	.loc	26 161 24
	vmaxss	.LCPI32_21(%rip), %xmm3, %xmm3
.Ltmp5419:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm3, %xmm3
.Ltmp5420:
	.loc	26 161 24
	vblendvps	%xmm9, %xmm2, %xmm4, %xmm2
.Ltmp5421:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm4
.Ltmp5422:
	.loc	26 66 9
	vsubss	%xmm4, %xmm3, %xmm3
.Ltmp5423:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp5424:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp5425:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp5426:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp5427:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp5428:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp5429:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp5430:
	.loc	26 61 9
	vaddss	%xmm6, %xmm15, %xmm6
.Ltmp5431:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp5432:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp5433:
	.loc	26 178 22
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp5434:
	.loc	7 1244 18
	vmovd	%xmm4, %edi
.Ltmp5435:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5436:
	.loc	7 1291 18
	vmovd	%edi, %xmm4
.Ltmp5437:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp5438:
	.loc	21 510 5
	vmovss	%xmm10, 940(%r14)
	vmovaps	112(%rsp), %xmm6
.Ltmp5439:
	.loc	26 71 9
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp5440:
	.loc	26 161 24
	vcmpneqss	%xmm5, %xmm10, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	vcmpnltss	780(%r14), %xmm5, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
.Ltmp5441:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%r15)
	movq	(%rsp), %rdi
.Ltmp5442:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, (%rdi,%r15)
.Ltmp5443:
	.loc	8 1916 50 is_stmt 1
	addq	$4, %r15
	incl	%ebp
	cmpq	%r15, 96(%rsp)
.Ltmp5444:
	.loc	11 900 12
	je	.LBB32_195
.Ltmp5445:
.LBB32_203:
	.loc	21 361 22
	movl	%ebp, %edi
	andl	%r10d, %edi
.Ltmp5446:
	.loc	25 451 16
	cmpq	%rdi, %rax
	jbe	.LBB32_296
.Ltmp5447:
	.loc	26 51 9
	vmovss	(%rcx,%r15), %xmm2
.Ltmp5448:
	.loc	26 56 9
	vmovss	%xmm2, (%rbx,%rdi,4)
.Ltmp5449:
	.loc	25 451 16
	cmpq	%rdi, %r9
	jbe	.LBB32_297
.Ltmp5450:
	.loc	25 0 16 is_stmt 0
	movq	(%rsp), %r8
.Ltmp5451:
	.loc	26 51 9 is_stmt 1
	vmovd	(%r8,%r15), %xmm2
.Ltmp5452:
	.loc	26 56 9
	vmovd	%xmm2, (%rdx,%rdi,4)
.Ltmp5453:
	.loc	21 370 21
	leal	(%r13,%rbp), %edi
	andl	%r10d, %edi
.Ltmp5454:
	.loc	25 438 16
	cmpq	%rdi, %rax
	jbe	.LBB32_298
.Ltmp5455:
	.loc	25 438 16 is_stmt 0
	cmpq	%rdi, %r9
	jbe	.LBB32_299
.Ltmp5456:
	.loc	26 51 9 is_stmt 1
	vmovss	(%rbx,%rdi,4), %xmm4
.Ltmp5457:
	.loc	26 51 9 is_stmt 0
	vmovd	(%rdx,%rdi,4), %xmm2
.Ltmp5458:
	.loc	21 0 0
	movl	%ebp, %edi
	subl	136(%r14), %edi
	andl	%r10d, %edi
	movl	%ebp, %r8d
	subl	200(%r14), %r8d
	andl	%r10d, %r8d
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 8(%rsp)
	vmovdqa	%xmm2, 112(%rsp)
	je	.LBB32_293
	cmpl	$1, %esi
	jne	.LBB32_212
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, %rax
	jbe	.LBB32_112
.Ltmp5459:
	.loc	21 252 33 is_stmt 1
	cmpq	%r8, %r9
	jbe	.LBB32_302
.Ltmp5460:
	.loc	21 251 32
	vmovss	(%rbx,%rdi,4), %xmm6
.Ltmp5461:
	.loc	21 252 33
	vmovss	(%rdx,%r8,4), %xmm15
	vmovaps	%xmm15, %xmm10
	vmovaps	%xmm6, %xmm12
	jmp	.LBB32_216
.Ltmp5462:
	.loc	21 0 33 is_stmt 0
.Ltmp5463:
	.p2align	4
.LBB32_293:
	cmpq	%rdi, %rax
.Ltmp5464:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB32_199
.Ltmp5465:
	.loc	21 237 33
	cmpq	%r8, %r9
	jbe	.LBB32_300
.Ltmp5466:
	.loc	21 236 32
	vmovss	(%rbx,%rdi,4), %xmm10
.Ltmp5467:
	.loc	21 237 33
	vmovss	(%rdx,%r8,4), %xmm6
	vmovaps	%xmm6, %xmm15
	vmovaps	%xmm10, %xmm12
.Ltmp5468:
	.loc	26 51 9
	jmp	.LBB32_216
.Ltmp5469:
	.loc	26 0 9 is_stmt 0
.Ltmp5470:
	.p2align	4
.LBB32_212:
	cmpq	%rdi, %rax
.Ltmp5471:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB32_201
	.loc	21 267 33
	cmpq	%rdi, %r9
	jbe	.LBB32_303
	.loc	21 268 33
	cmpq	%r8, %r9
	jbe	.LBB32_304
	.loc	21 0 0 is_stmt 0
	vmovss	(%rbx,%rdi,4), %xmm12
	.loc	21 267 33 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm10
	.loc	21 268 33
	vmovss	(%rdx,%r8,4), %xmm15
	.loc	21 269 33
	vmovss	(%rbx,%r8,4), %xmm6
.Ltmp5472:
.LBB32_216:
	.loc	21 439 26
	vmovss	804(%r14), %xmm9
.Ltmp5473:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp5474:
	.loc	21 441 44
	vmovss	800(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	792(%r14), %xmm8
.Ltmp5475:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm8, %xmm11
.Ltmp5476:
	.loc	26 161 24
	jne	.LBB32_219
	jp	.LBB32_219
.Ltmp5477:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%r14), %xmm11
.LBB32_219:
.Ltmp5478:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_222
	jp	.LBB32_222
.Ltmp5479:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_222:
.Ltmp5480:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm9
.Ltmp5481:
	.loc	26 161 24
	jbe	.LBB32_224
.Ltmp5482:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm8
.LBB32_224:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm8, 792(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 800(%r14)
.Ltmp5483:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp5484:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp5485:
	.loc	21 448 13
	vmovss	%xmm2, 804(%r14)
.Ltmp5486:
	.loc	21 439 26
	vmovss	820(%r14), %xmm11
.Ltmp5487:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp5488:
	.loc	21 441 27
	vmovss	808(%r14), %xmm9
	.loc	21 441 44 is_stmt 0
	vmovss	816(%r14), %xmm3
.Ltmp5489:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp5490:
	.loc	26 161 24
	jne	.LBB32_227
	jp	.LBB32_227
.Ltmp5491:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%r14), %xmm13
.LBB32_227:
.Ltmp5492:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_230
	jp	.LBB32_230
.Ltmp5493:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_230:
.Ltmp5494:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp5495:
	.loc	26 161 24
	jbe	.LBB32_232
.Ltmp5496:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB32_232:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm9, 808(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 816(%r14)
.Ltmp5497:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp5498:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp5499:
	.loc	21 448 13
	vmovss	%xmm2, 820(%r14)
.Ltmp5500:
	.loc	21 439 26
	vmovss	836(%r14), %xmm13
.Ltmp5501:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp5502:
	.loc	21 441 27
	vmovss	824(%r14), %xmm11
	.loc	21 441 44 is_stmt 0
	vmovss	832(%r14), %xmm3
.Ltmp5503:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm14
.Ltmp5504:
	.loc	26 161 24
	jne	.LBB32_235
	jp	.LBB32_235
.Ltmp5505:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%r14), %xmm14
.LBB32_235:
.Ltmp5506:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_238
	jp	.LBB32_238
.Ltmp5507:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_238:
.Ltmp5508:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp5509:
	.loc	26 161 24
	jbe	.LBB32_240
.Ltmp5510:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm11
.LBB32_240:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 824(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 832(%r14)
.Ltmp5511:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp5512:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp5513:
	.loc	21 448 13
	vmovss	%xmm2, 836(%r14)
.Ltmp5514:
	.loc	21 439 26
	vmovss	852(%r14), %xmm14
.Ltmp5515:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp5516:
	.loc	21 441 44
	vmovss	848(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	840(%r14), %xmm13
.Ltmp5517:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp5518:
	.loc	26 161 24
	jne	.LBB32_243
	jp	.LBB32_243
.Ltmp5519:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%r14), %xmm2
.LBB32_243:
.Ltmp5520:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_246
	jp	.LBB32_246
.Ltmp5521:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_246:
.Ltmp5522:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp5523:
	.loc	26 161 24
	jbe	.LBB32_248
.Ltmp5524:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB32_248:
.Ltmp5525:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp5526:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp5527:
	.loc	21 442 13
	vmovss	%xmm13, 840(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 848(%r14)
	vbroadcastss	.LCPI32_4(%rip), %xmm7
.Ltmp5528:
	.loc	26 103 24
	vandps	%xmm7, %xmm12, %xmm3
.Ltmp5529:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm10, %xmm7
.Ltmp5530:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp5531:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5532:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %r8d
.Ltmp5533:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r8d
.Ltmp5534:
	.loc	21 448 13
	vmovss	%xmm2, 852(%r14)
.Ltmp5535:
	.loc	21 459 23
	vmovss	760(%r14), %xmm2
.Ltmp5536:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp5537:
	.loc	26 161 24
	cmovbel	%edi, %r8d
.Ltmp5538:
	.loc	21 461 9
	vmovss	764(%r14), %xmm2
.Ltmp5539:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI32_5(%rip), %xmm10
.Ltmp5540:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm2
.Ltmp5541:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm3
.Ltmp5542:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5543:
	.loc	7 1244 18
	vmovd	%xmm2, %edi
.Ltmp5544:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp5545:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5546:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm2
.Ltmp5547:
	.loc	26 161 24
	movl	$841731191, %r8d
	cmovbel	%r8d, %edi
.Ltmp5548:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5549:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm2
.Ltmp5550:
	.loc	26 161 24
	movl	$8388608, %r8d
	cmovbel	%r8d, %edi
.Ltmp5551:
	.loc	26 185 42
	movl	%edi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp5552:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp5553:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp5554:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm2, %xmm3
.Ltmp5555:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm7
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp5556:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5557:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm3, %xmm3
.Ltmp5558:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5559:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm3, %xmm3
.Ltmp5560:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5561:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm3, %xmm3
.Ltmp5562:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5563:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm3, %xmm3
.Ltmp5564:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp5565:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp5566:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5567:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp5568:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5569:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm2, %xmm2
.Ltmp5570:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm2, %xmm2
.Ltmp5571:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm2, %xmm10
.Ltmp5572:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp5573:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm10
.Ltmp5574:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp5575:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm10
.Ltmp5576:
	.loc	26 144 9
	movl	$0, %r8d
	adcl	$-1, %r8d
.Ltmp5577:
	.loc	26 149 9
	movl	%r12d, %edi
.Ltmp5578:
	.loc	21 486 47
	vmovss	860(%r14), %xmm3
.Ltmp5579:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm3
.Ltmp5580:
	.loc	26 149 9
	notl	%edi
.Ltmp5581:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp5582:
	.loc	21 478 20
	vmovss	856(%r14), %xmm2
.Ltmp5583:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp5584:
	.loc	21 491 9
	vmovss	752(%r14), %xmm12
.Ltmp5585:
	.loc	26 144 9
	cmoval	%r12d, %r8d
.Ltmp5586:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp5587:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB32_250
.Ltmp5588:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB32_250:
.Ltmp5589:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r8b
	jne	.LBB32_252
.Ltmp5590:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB32_252:
	orl	%r8d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm3
.Ltmp5591:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm12, 860(%r14)
	.loc	21 498 5
	movl	%edi, 856(%r14)
.Ltmp5592:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp5593:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm10, %xmm7
.Ltmp5594:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm7, %xmm2, %xmm2
.Ltmp5595:
	.loc	26 98 24
	vbroadcastss	.LCPI32_18(%rip), %xmm7
	vxorps	%xmm7, %xmm11, %xmm7
.Ltmp5596:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm2, %xmm2
.Ltmp5597:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm7
	vandps	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %edi
	testb	$1, %dil
	jne	.LBB32_254
.Ltmp5598:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB32_254:
.Ltmp5599:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%r14), %xmm3
.Ltmp5600:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm3, %xmm2
	setbe	%dil
.Ltmp5601:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp5602:
	.loc	26 92 9
	vmulss	744(%r14,%rdi,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5603:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp5604:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp5605:
	.loc	21 510 5
	vmovss	%xmm8, 864(%r14)
.Ltmp5606:
	.loc	21 439 26
	vmovss	880(%r14), %xmm11
.Ltmp5607:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp5608:
	.loc	21 441 44
	vmovss	876(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	868(%r14), %xmm10
.Ltmp5609:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp5610:
	.loc	26 161 24
	jne	.LBB32_257
	jp	.LBB32_257
.Ltmp5611:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%r14), %xmm2
.LBB32_257:
.Ltmp5612:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_260
	jp	.LBB32_260
.Ltmp5613:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_260:
.Ltmp5614:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp5615:
	.loc	26 161 24
	jbe	.LBB32_262
.Ltmp5616:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB32_262:
.Ltmp5617:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%r14), %xmm5, %xmm9
.Ltmp5618:
	.loc	21 442 13
	vmovss	%xmm10, 868(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 876(%r14)
.Ltmp5619:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp5620:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp5621:
	.loc	21 448 13
	vmovss	%xmm2, 880(%r14)
.Ltmp5622:
	.loc	21 439 26
	vmovss	896(%r14), %xmm12
.Ltmp5623:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp5624:
	.loc	21 441 44
	vmovss	892(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	884(%r14), %xmm11
.Ltmp5625:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp5626:
	.loc	26 161 24
	jne	.LBB32_265
	jp	.LBB32_265
.Ltmp5627:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%r14), %xmm2
.LBB32_265:
.Ltmp5628:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_268
	jp	.LBB32_268
.Ltmp5629:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_268:
.Ltmp5630:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm12
.Ltmp5631:
	.loc	26 161 24
	jbe	.LBB32_270
.Ltmp5632:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB32_270:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 884(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 892(%r14)
.Ltmp5633:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp5634:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp5635:
	.loc	21 448 13
	vmovss	%xmm2, 896(%r14)
.Ltmp5636:
	.loc	21 439 26
	vmovss	912(%r14), %xmm13
.Ltmp5637:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp5638:
	.loc	21 441 44
	vmovss	908(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	900(%r14), %xmm12
.Ltmp5639:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp5640:
	.loc	26 161 24
	jne	.LBB32_273
	jp	.LBB32_273
.Ltmp5641:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%r14), %xmm2
.LBB32_273:
.Ltmp5642:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_276
	jp	.LBB32_276
.Ltmp5643:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_276:
.Ltmp5644:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp5645:
	.loc	26 161 24
	jbe	.LBB32_278
.Ltmp5646:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB32_278:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm12, 900(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 908(%r14)
.Ltmp5647:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp5648:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp5649:
	.loc	21 448 13
	vmovss	%xmm2, 912(%r14)
.Ltmp5650:
	.loc	21 439 26
	vmovss	928(%r14), %xmm14
.Ltmp5651:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp5652:
	.loc	21 441 44
	vmovss	924(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	916(%r14), %xmm13
.Ltmp5653:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp5654:
	.loc	26 161 24
	jne	.LBB32_281
	jp	.LBB32_281
.Ltmp5655:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%r14), %xmm2
.LBB32_281:
.Ltmp5656:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_284
	jp	.LBB32_284
.Ltmp5657:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_284:
.Ltmp5658:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp5659:
	.loc	26 161 24
	jbe	.LBB32_286
.Ltmp5660:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB32_286:
.Ltmp5661:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp5662:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp5663:
	.loc	21 442 13
	vmovss	%xmm13, 916(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 924(%r14)
	vbroadcastss	.LCPI32_4(%rip), %xmm7
.Ltmp5664:
	.loc	26 103 24
	vandps	%xmm7, %xmm15, %xmm3
.Ltmp5665:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm6, %xmm6
.Ltmp5666:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp5667:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5668:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %r8d
.Ltmp5669:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r8d
.Ltmp5670:
	.loc	21 448 13
	vmovss	%xmm2, 928(%r14)
.Ltmp5671:
	.loc	21 459 23
	vmovss	784(%r14), %xmm2
.Ltmp5672:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp5673:
	.loc	26 161 24
	cmovbel	%edi, %r8d
.Ltmp5674:
	.loc	21 461 9
	vmovss	788(%r14), %xmm2
.Ltmp5675:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI32_5(%rip), %xmm7
.Ltmp5676:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp5677:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp5678:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5679:
	.loc	7 1244 18
	vmovd	%xmm2, %edi
.Ltmp5680:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp5681:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5682:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm2
.Ltmp5683:
	.loc	26 161 24
	movl	$841731191, %r8d
	cmovbel	%r8d, %edi
.Ltmp5684:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5685:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm2
.Ltmp5686:
	.loc	26 161 24
	movl	$8388608, %r8d
	cmovbel	%r8d, %edi
.Ltmp5687:
	.loc	26 185 42
	movl	%edi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp5688:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp5689:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp5690:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm2, %xmm3
.Ltmp5691:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp5692:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5693:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm3, %xmm3
.Ltmp5694:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5695:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm3, %xmm3
.Ltmp5696:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5697:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm3, %xmm3
.Ltmp5698:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5699:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm3, %xmm3
.Ltmp5700:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp5701:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp5702:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5703:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp5704:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5705:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm2, %xmm2
.Ltmp5706:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm2, %xmm2
.Ltmp5707:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm2, %xmm14
.Ltmp5708:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp5709:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp5710:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp5711:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp5712:
	.loc	26 144 9
	movl	$0, %r8d
	adcl	$-1, %r8d
.Ltmp5713:
	.loc	26 149 9
	movl	%r12d, %edi
.Ltmp5714:
	.loc	21 486 47
	vmovss	936(%r14), %xmm6
.Ltmp5715:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm6
.Ltmp5716:
	.loc	26 149 9
	notl	%edi
.Ltmp5717:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp5718:
	.loc	21 478 20
	vmovss	932(%r14), %xmm2
.Ltmp5719:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp5720:
	.loc	21 491 9
	vmovss	776(%r14), %xmm3
.Ltmp5721:
	.loc	26 144 9
	cmoval	%r12d, %r8d
.Ltmp5722:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp5723:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB32_288
.Ltmp5724:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB32_288:
.Ltmp5725:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r8b
	jne	.LBB32_290
.Ltmp5726:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB32_290:
.Ltmp5727:
	vmulss	.LCPI32_20(%rip), %xmm8, %xmm2
	vmaxss	.LCPI32_21(%rip), %xmm2, %xmm2
	vminss	.LCPI32_22(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp5728:
	orl	%r8d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm13
.Ltmp5729:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm3, 936(%r14)
	.loc	21 498 5
	movl	%edi, 932(%r14)
.Ltmp5730:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp5731:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp5732:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp5733:
	.loc	26 98 24
	vbroadcastss	.LCPI32_18(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp5734:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp5735:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %edi
	testb	$1, %dil
	jne	.LBB32_292
.Ltmp5736:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB32_292
.LBB32_106:
.Ltmp5737:
	.loc	25 451 16 is_stmt 1
	negl	%r13d
	leaq	(,%rsi,4), %rsi
	movq	%rsi, 96(%rsp)
	vmovss	.LCPI32_2(%rip), %xmm0
	vmovss	.LCPI32_3(%rip), %xmm1
	movl	%r8d, %ebp
	xorl	%r15d, %r15d
	movl	%edi, %esi
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB32_107
.Ltmp5738:
	.loc	25 0 16 is_stmt 0
.Ltmp5739:
	.p2align	4
.LBB32_194:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%r14), %xmm3
.Ltmp5740:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm3, %xmm2
	setbe	%dil
.Ltmp5741:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp5742:
	.loc	26 92 9
	vmulss	768(%r14,%rdi,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5743:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp5744:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI32_23(%rip), %xmm11
.Ltmp5745:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI32_24(%rip), %xmm12
.Ltmp5746:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp5747:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI32_25(%rip), %xmm13
.Ltmp5748:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp5749:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI32_26(%rip), %xmm14
.Ltmp5750:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp5751:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI32_27(%rip), %xmm15
.Ltmp5752:
	.loc	26 61 9
	vaddss	%xmm2, %xmm15, %xmm2
.Ltmp5753:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp5754:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI32_28(%rip), %xmm7
.Ltmp5755:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp5756:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5757:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5758:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5759:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp5760:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm4, %xmm2
.Ltmp5761:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
.Ltmp5762:
	.loc	26 71 9
	vmulss	.LCPI32_20(%rip), %xmm10, %xmm3
.Ltmp5763:
	.loc	26 161 24
	vmaxss	.LCPI32_21(%rip), %xmm3, %xmm3
.Ltmp5764:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm3, %xmm3
.Ltmp5765:
	.loc	26 161 24
	vblendvps	%xmm9, %xmm2, %xmm4, %xmm2
.Ltmp5766:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm4
.Ltmp5767:
	.loc	26 66 9
	vsubss	%xmm4, %xmm3, %xmm3
.Ltmp5768:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp5769:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp5770:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp5771:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp5772:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp5773:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp5774:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp5775:
	.loc	26 61 9
	vaddss	%xmm6, %xmm15, %xmm6
.Ltmp5776:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp5777:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp5778:
	.loc	26 178 22
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp5779:
	.loc	7 1244 18
	vmovd	%xmm4, %edi
.Ltmp5780:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp5781:
	.loc	7 1291 18
	vmovd	%edi, %xmm4
.Ltmp5782:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp5783:
	.loc	21 510 5
	vmovss	%xmm10, 940(%r14)
	vmovaps	112(%rsp), %xmm6
.Ltmp5784:
	.loc	26 71 9
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp5785:
	.loc	26 161 24
	vcmpneqss	%xmm5, %xmm10, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	vcmpnltss	780(%r14), %xmm5, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
.Ltmp5786:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%r15)
	movq	(%rsp), %rdi
.Ltmp5787:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, (%rdi,%r15)
.Ltmp5788:
	.loc	8 1916 50 is_stmt 1
	addq	$4, %r15
	incl	%ebp
	cmpq	%r15, 96(%rsp)
.Ltmp5789:
	.loc	11 900 12
	je	.LBB32_195
.Ltmp5790:
.LBB32_107:
	.loc	21 361 22
	movl	%ebp, %edi
	andl	%r10d, %edi
.Ltmp5791:
	.loc	25 451 16
	cmpq	%rdi, %rax
	jbe	.LBB32_296
.Ltmp5792:
	.loc	26 51 9
	vmovss	(%rcx,%r15), %xmm2
.Ltmp5793:
	.loc	26 56 9
	vmovss	%xmm2, (%rbx,%rdi,4)
	movq	(%rsp), %r8
.Ltmp5794:
	.loc	26 51 9
	vmovss	(%r8,%r15), %xmm2
.Ltmp5795:
	.loc	26 56 9
	vmovss	%xmm2, (%rdx,%rdi,4)
.Ltmp5796:
	.loc	21 370 21
	leal	(%r13,%rbp), %edi
	andl	%r10d, %edi
.Ltmp5797:
	.loc	25 438 16
	cmpq	%rdi, %rax
	jbe	.LBB32_298
.Ltmp5798:
	.loc	26 51 9
	vmovss	(%rbx,%rdi,4), %xmm4
.Ltmp5799:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm2
.Ltmp5800:
	.loc	21 0 0
	movl	%ebp, %edi
	subl	136(%r14), %edi
	andl	%r10d, %edi
	movl	%ebp, %r8d
	subl	200(%r14), %r8d
	andl	%r10d, %r8d
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 8(%rsp)
	vmovaps	%xmm2, 112(%rsp)
	je	.LBB32_198
	cmpl	$1, %esi
	jne	.LBB32_200
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, %rax
.Ltmp5801:
	.loc	21 251 32 is_stmt 1
	jbe	.LBB32_112
.Ltmp5802:
	.loc	21 252 33
	cmpq	%r8, %r9
	jbe	.LBB32_302
.Ltmp5803:
	.loc	21 251 32
	vmovss	(%rbx,%rdi,4), %xmm6
.Ltmp5804:
	.loc	21 252 33
	vmovss	(%rdx,%r8,4), %xmm15
	vmovaps	%xmm15, %xmm10
	vmovaps	%xmm6, %xmm12
	jmp	.LBB32_118
.Ltmp5805:
	.loc	21 0 33 is_stmt 0
.Ltmp5806:
	.p2align	4
.LBB32_198:
	cmpq	%rdi, %rax
.Ltmp5807:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB32_199
.Ltmp5808:
	.loc	21 237 33
	cmpq	%r8, %r9
	jbe	.LBB32_300
.Ltmp5809:
	.loc	21 236 32
	vmovss	(%rbx,%rdi,4), %xmm10
.Ltmp5810:
	.loc	21 237 33
	vmovss	(%rdx,%r8,4), %xmm6
	vmovaps	%xmm6, %xmm15
	vmovaps	%xmm10, %xmm12
.Ltmp5811:
	.loc	26 51 9
	jmp	.LBB32_118
.Ltmp5812:
	.loc	26 0 9 is_stmt 0
.Ltmp5813:
	.p2align	4
.LBB32_200:
	cmpq	%rdi, %rax
.Ltmp5814:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB32_201
	.loc	21 268 33
	cmpq	%r8, %r9
	jbe	.LBB32_304
	.loc	21 269 33
	cmpq	%r8, %rax
	jbe	.LBB32_305
	.loc	21 0 0 is_stmt 0
	vmovss	(%rbx,%rdi,4), %xmm12
	vmovss	(%rdx,%rdi,4), %xmm10
	.loc	21 268 33 is_stmt 1
	vmovss	(%rdx,%r8,4), %xmm15
	.loc	21 269 33
	vmovss	(%rbx,%r8,4), %xmm6
.Ltmp5815:
.LBB32_118:
	.loc	21 439 26
	vmovss	804(%r14), %xmm9
.Ltmp5816:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp5817:
	.loc	21 441 44
	vmovss	800(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	792(%r14), %xmm8
.Ltmp5818:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm8, %xmm11
.Ltmp5819:
	.loc	26 161 24
	jne	.LBB32_121
	jp	.LBB32_121
.Ltmp5820:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%r14), %xmm11
.LBB32_121:
.Ltmp5821:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_124
	jp	.LBB32_124
.Ltmp5822:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_124:
.Ltmp5823:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm9
.Ltmp5824:
	.loc	26 161 24
	jbe	.LBB32_126
.Ltmp5825:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm8
.LBB32_126:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm8, 792(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 800(%r14)
.Ltmp5826:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp5827:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp5828:
	.loc	21 448 13
	vmovss	%xmm2, 804(%r14)
.Ltmp5829:
	.loc	21 439 26
	vmovss	820(%r14), %xmm11
.Ltmp5830:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp5831:
	.loc	21 441 27
	vmovss	808(%r14), %xmm9
	.loc	21 441 44 is_stmt 0
	vmovss	816(%r14), %xmm3
.Ltmp5832:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp5833:
	.loc	26 161 24
	jne	.LBB32_129
	jp	.LBB32_129
.Ltmp5834:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%r14), %xmm13
.LBB32_129:
.Ltmp5835:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_132
	jp	.LBB32_132
.Ltmp5836:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_132:
.Ltmp5837:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp5838:
	.loc	26 161 24
	jbe	.LBB32_134
.Ltmp5839:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB32_134:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm9, 808(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 816(%r14)
.Ltmp5840:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp5841:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp5842:
	.loc	21 448 13
	vmovss	%xmm2, 820(%r14)
.Ltmp5843:
	.loc	21 439 26
	vmovss	836(%r14), %xmm13
.Ltmp5844:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp5845:
	.loc	21 441 27
	vmovss	824(%r14), %xmm11
	.loc	21 441 44 is_stmt 0
	vmovss	832(%r14), %xmm3
.Ltmp5846:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm14
.Ltmp5847:
	.loc	26 161 24
	jne	.LBB32_137
	jp	.LBB32_137
.Ltmp5848:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%r14), %xmm14
.LBB32_137:
.Ltmp5849:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_140
	jp	.LBB32_140
.Ltmp5850:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_140:
.Ltmp5851:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp5852:
	.loc	26 161 24
	jbe	.LBB32_142
.Ltmp5853:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm11
.LBB32_142:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 824(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 832(%r14)
.Ltmp5854:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp5855:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp5856:
	.loc	21 448 13
	vmovss	%xmm2, 836(%r14)
.Ltmp5857:
	.loc	21 439 26
	vmovss	852(%r14), %xmm14
.Ltmp5858:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp5859:
	.loc	21 441 44
	vmovss	848(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	840(%r14), %xmm13
.Ltmp5860:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp5861:
	.loc	26 161 24
	jne	.LBB32_145
	jp	.LBB32_145
.Ltmp5862:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%r14), %xmm2
.LBB32_145:
.Ltmp5863:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_148
	jp	.LBB32_148
.Ltmp5864:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_148:
.Ltmp5865:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp5866:
	.loc	26 161 24
	jbe	.LBB32_150
.Ltmp5867:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB32_150:
.Ltmp5868:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp5869:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp5870:
	.loc	21 442 13
	vmovss	%xmm13, 840(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 848(%r14)
	vbroadcastss	.LCPI32_4(%rip), %xmm7
.Ltmp5871:
	.loc	26 103 24
	vandps	%xmm7, %xmm12, %xmm3
.Ltmp5872:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm10, %xmm7
.Ltmp5873:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp5874:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp5875:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %r8d
.Ltmp5876:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r8d
.Ltmp5877:
	.loc	21 448 13
	vmovss	%xmm2, 852(%r14)
.Ltmp5878:
	.loc	21 459 23
	vmovss	760(%r14), %xmm2
.Ltmp5879:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp5880:
	.loc	26 161 24
	cmovbel	%edi, %r8d
.Ltmp5881:
	.loc	21 461 9
	vmovss	764(%r14), %xmm2
.Ltmp5882:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI32_5(%rip), %xmm10
.Ltmp5883:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm2
.Ltmp5884:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm3
.Ltmp5885:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5886:
	.loc	7 1244 18
	vmovd	%xmm2, %edi
.Ltmp5887:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp5888:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5889:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm2
.Ltmp5890:
	.loc	26 161 24
	movl	$841731191, %r8d
	cmovbel	%r8d, %edi
.Ltmp5891:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp5892:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm2
.Ltmp5893:
	.loc	26 161 24
	movl	$8388608, %r8d
	cmovbel	%r8d, %edi
.Ltmp5894:
	.loc	26 185 42
	movl	%edi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp5895:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp5896:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp5897:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm2, %xmm3
.Ltmp5898:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm7
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp5899:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5900:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm3, %xmm3
.Ltmp5901:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5902:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm3, %xmm3
.Ltmp5903:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5904:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm3, %xmm3
.Ltmp5905:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp5906:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm3, %xmm3
.Ltmp5907:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp5908:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp5909:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp5910:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp5911:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5912:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm2, %xmm2
.Ltmp5913:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm2, %xmm2
.Ltmp5914:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm2, %xmm10
.Ltmp5915:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp5916:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm10
.Ltmp5917:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp5918:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm10
.Ltmp5919:
	.loc	26 144 9
	movl	$0, %r8d
	adcl	$-1, %r8d
.Ltmp5920:
	.loc	26 149 9
	movl	%r12d, %edi
.Ltmp5921:
	.loc	21 486 47
	vmovss	860(%r14), %xmm3
.Ltmp5922:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm3
.Ltmp5923:
	.loc	26 149 9
	notl	%edi
.Ltmp5924:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp5925:
	.loc	21 478 20
	vmovss	856(%r14), %xmm2
.Ltmp5926:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp5927:
	.loc	21 491 9
	vmovss	752(%r14), %xmm12
.Ltmp5928:
	.loc	26 144 9
	cmoval	%r12d, %r8d
.Ltmp5929:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp5930:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB32_152
.Ltmp5931:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB32_152:
.Ltmp5932:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r8b
	jne	.LBB32_154
.Ltmp5933:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB32_154:
	orl	%r8d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm3
.Ltmp5934:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm12, 860(%r14)
	.loc	21 498 5
	movl	%edi, 856(%r14)
.Ltmp5935:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp5936:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm10, %xmm7
.Ltmp5937:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm7, %xmm2, %xmm2
.Ltmp5938:
	.loc	26 98 24
	vbroadcastss	.LCPI32_18(%rip), %xmm7
	vxorps	%xmm7, %xmm11, %xmm7
.Ltmp5939:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm2, %xmm2
.Ltmp5940:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm7
	vandps	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %edi
	testb	$1, %dil
	jne	.LBB32_156
.Ltmp5941:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB32_156:
.Ltmp5942:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%r14), %xmm3
.Ltmp5943:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm3, %xmm2
	setbe	%dil
.Ltmp5944:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp5945:
	.loc	26 92 9
	vmulss	744(%r14,%rdi,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp5946:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp5947:
	.loc	26 166 24
	vcmpnltss	.LCPI32_19(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp5948:
	.loc	21 510 5
	vmovss	%xmm8, 864(%r14)
.Ltmp5949:
	.loc	21 439 26
	vmovss	880(%r14), %xmm11
.Ltmp5950:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp5951:
	.loc	21 441 44
	vmovss	876(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	868(%r14), %xmm10
.Ltmp5952:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp5953:
	.loc	26 161 24
	jne	.LBB32_159
	jp	.LBB32_159
.Ltmp5954:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%r14), %xmm2
.LBB32_159:
.Ltmp5955:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_162
	jp	.LBB32_162
.Ltmp5956:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_162:
.Ltmp5957:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp5958:
	.loc	26 161 24
	jbe	.LBB32_164
.Ltmp5959:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB32_164:
.Ltmp5960:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%r14), %xmm5, %xmm9
.Ltmp5961:
	.loc	21 442 13
	vmovss	%xmm10, 868(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 876(%r14)
.Ltmp5962:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp5963:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp5964:
	.loc	21 448 13
	vmovss	%xmm2, 880(%r14)
.Ltmp5965:
	.loc	21 439 26
	vmovss	896(%r14), %xmm12
.Ltmp5966:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp5967:
	.loc	21 441 44
	vmovss	892(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	884(%r14), %xmm11
.Ltmp5968:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp5969:
	.loc	26 161 24
	jne	.LBB32_167
	jp	.LBB32_167
.Ltmp5970:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%r14), %xmm2
.LBB32_167:
.Ltmp5971:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_170
	jp	.LBB32_170
.Ltmp5972:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_170:
.Ltmp5973:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm12
.Ltmp5974:
	.loc	26 161 24
	jbe	.LBB32_172
.Ltmp5975:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB32_172:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 884(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 892(%r14)
.Ltmp5976:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp5977:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp5978:
	.loc	21 448 13
	vmovss	%xmm2, 896(%r14)
.Ltmp5979:
	.loc	21 439 26
	vmovss	912(%r14), %xmm13
.Ltmp5980:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp5981:
	.loc	21 441 44
	vmovss	908(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	900(%r14), %xmm12
.Ltmp5982:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp5983:
	.loc	26 161 24
	jne	.LBB32_175
	jp	.LBB32_175
.Ltmp5984:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%r14), %xmm2
.LBB32_175:
.Ltmp5985:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_178
	jp	.LBB32_178
.Ltmp5986:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_178:
.Ltmp5987:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp5988:
	.loc	26 161 24
	jbe	.LBB32_180
.Ltmp5989:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB32_180:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm12, 900(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 908(%r14)
.Ltmp5990:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp5991:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp5992:
	.loc	21 448 13
	vmovss	%xmm2, 912(%r14)
.Ltmp5993:
	.loc	21 439 26
	vmovss	928(%r14), %xmm14
.Ltmp5994:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp5995:
	.loc	21 441 44
	vmovss	924(%r14), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	916(%r14), %xmm13
.Ltmp5996:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp5997:
	.loc	26 161 24
	jne	.LBB32_183
	jp	.LBB32_183
.Ltmp5998:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%r14), %xmm2
.LBB32_183:
.Ltmp5999:
	.loc	26 161 44 is_stmt 1
	jne	.LBB32_186
	jp	.LBB32_186
.Ltmp6000:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB32_186:
.Ltmp6001:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp6002:
	.loc	26 161 24
	jbe	.LBB32_188
.Ltmp6003:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB32_188:
.Ltmp6004:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp6005:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp6006:
	.loc	21 442 13
	vmovss	%xmm13, 916(%r14)
	.loc	21 447 13
	vmovss	%xmm3, 924(%r14)
	vbroadcastss	.LCPI32_4(%rip), %xmm7
.Ltmp6007:
	.loc	26 103 24
	vandps	%xmm7, %xmm15, %xmm3
.Ltmp6008:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm6, %xmm6
.Ltmp6009:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp6010:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp6011:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %r8d
.Ltmp6012:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r8d
.Ltmp6013:
	.loc	21 448 13
	vmovss	%xmm2, 928(%r14)
.Ltmp6014:
	.loc	21 459 23
	vmovss	784(%r14), %xmm2
.Ltmp6015:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp6016:
	.loc	26 161 24
	cmovbel	%edi, %r8d
.Ltmp6017:
	.loc	21 461 9
	vmovss	788(%r14), %xmm2
.Ltmp6018:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI32_5(%rip), %xmm7
.Ltmp6019:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp6020:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp6021:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp6022:
	.loc	7 1244 18
	vmovd	%xmm2, %edi
.Ltmp6023:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp6024:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp6025:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm2
.Ltmp6026:
	.loc	26 161 24
	movl	$841731191, %r8d
	cmovbel	%r8d, %edi
.Ltmp6027:
	.loc	7 1291 18
	vmovd	%edi, %xmm2
.Ltmp6028:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm2
.Ltmp6029:
	.loc	26 161 24
	movl	$8388608, %r8d
	cmovbel	%r8d, %edi
.Ltmp6030:
	.loc	26 185 42
	movl	%edi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp6031:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp6032:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp6033:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm2, %xmm3
.Ltmp6034:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp6035:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp6036:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm3, %xmm3
.Ltmp6037:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp6038:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm3, %xmm3
.Ltmp6039:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp6040:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm3, %xmm3
.Ltmp6041:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp6042:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm3, %xmm3
.Ltmp6043:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp6044:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp6045:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp6046:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp6047:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp6048:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm2, %xmm2
.Ltmp6049:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm2, %xmm2
.Ltmp6050:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm2, %xmm14
.Ltmp6051:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp6052:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp6053:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6054:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp6055:
	.loc	26 144 9
	movl	$0, %r8d
	adcl	$-1, %r8d
.Ltmp6056:
	.loc	26 149 9
	movl	%r12d, %edi
.Ltmp6057:
	.loc	21 486 47
	vmovss	936(%r14), %xmm6
.Ltmp6058:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm6
.Ltmp6059:
	.loc	26 149 9
	notl	%edi
.Ltmp6060:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp6061:
	.loc	21 478 20
	vmovss	932(%r14), %xmm2
.Ltmp6062:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp6063:
	.loc	21 491 9
	vmovss	776(%r14), %xmm3
.Ltmp6064:
	.loc	26 144 9
	cmoval	%r12d, %r8d
.Ltmp6065:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp6066:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB32_190
.Ltmp6067:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB32_190:
.Ltmp6068:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r8b
	jne	.LBB32_192
.Ltmp6069:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB32_192:
.Ltmp6070:
	vmulss	.LCPI32_20(%rip), %xmm8, %xmm2
	vmaxss	.LCPI32_21(%rip), %xmm2, %xmm2
	vminss	.LCPI32_22(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp6071:
	orl	%r8d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm13
.Ltmp6072:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm3, 936(%r14)
	.loc	21 498 5
	movl	%edi, 932(%r14)
.Ltmp6073:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp6074:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp6075:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp6076:
	.loc	26 98 24
	vbroadcastss	.LCPI32_18(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp6077:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp6078:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %edi
	testb	$1, %dil
	jne	.LBB32_194
.Ltmp6079:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB32_194
.LBB32_195:
	movl	24(%rsp), %eax
	movq	136(%rsp), %rsi
.Ltmp6080:
	.loc	15 2584 13 is_stmt 1
	addl	%esi, %eax
.Ltmp6081:
	.loc	21 413 5
	movl	%eax, 1208(%r14)
	movq	144(%rsp), %r10
	movq	304(%rsp), %rdx
	movq	32(%rsp), %r8
.Ltmp6082:
.LBB32_41:
	.loc	6 691 12
	cmpq	%r8, %r10
	jbe	.LBB32_42
.Ltmp6083:
	.loc	25 580 12
	movq	%rsi, %rax
	subq	%rdx, %rax
	movq	%rax, 376(%rsp)
	ja	.LBB32_437
.Ltmp6084:
	.loc	21 61 8
	movl	136(%r14), %eax
	cmpl	200(%r14), %eax
	sete	%al
	movb	$2, %dil
	subb	%al, %dil
	xorl	%esi, %esi
	cmpl	$1, 68(%r14)
	movzbl	%dil, %edi
	cmovel	%esi, %edi
.Ltmp6085:
	.loc	6 736 31
	movq	104(%r14), %rbx
	movq	112(%r14), %rax
	.loc	6 741 31
	movq	168(%r14), %r15
	movq	176(%r14), %r11
	.loc	6 747 28
	movl	1212(%r14), %ebp
	.loc	6 748 24
	movl	1216(%r14), %r12d
.Ltmp6086:
	.loc	21 353 16
	movl	1208(%r14), %r9d
	vmovss	792(%r14), %xmm0
	vmovss	760(%r14), %xmm1
	vmovss	%xmm1, 24(%rsp)
	vmovss	764(%r14), %xmm1
	vmovss	%xmm1, 224(%rsp)
	vmovss	%xmm0, 8(%rsp)
	vsubss	840(%r14), %xmm0, %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	752(%r14), %xmm0
	vmovss	%xmm0, 152(%rsp)
	vmovss	.LCPI32_3(%rip), %xmm2
	vaddss	808(%r14), %xmm2, %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	824(%r14), %xmm0
	vbroadcastss	.LCPI32_18(%rip), %xmm1
	vxorps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 288(%rsp)
	movl	744(%r14), %r8d
	movl	%r8d, 92(%rsp)
	movl	748(%r14), %r8d
	movl	%r8d, 88(%rsp)
	vmovss	756(%r14), %xmm0
	vmovss	%xmm0, 84(%rsp)
	vmovss	868(%r14), %xmm9
	vmovss	784(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	788(%r14), %xmm0
	vmovss	%xmm0, 76(%rsp)
	vsubss	916(%r14), %xmm9, %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	776(%r14), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	900(%r14), %xmm0
	vxorps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 320(%rsp)
	vaddss	884(%r14), %xmm2, %xmm0
	vmovss	%xmm0, 64(%rsp)
	movl	768(%r14), %r8d
	movl	%r8d, 60(%rsp)
	movl	772(%r14), %r8d
	movl	%r8d, 56(%rsp)
	movl	%edi, 32(%rsp)
	movzbl	%dil, %edi
	movl	%edi, 48(%rsp)
	vmovss	780(%r14), %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	860(%r14), %xmm0
	vmovss	864(%r14), %xmm7
	vmovss	936(%r14), %xmm13
	vmovss	940(%r14), %xmm2
	cmpq	%rdx, %r10
	jbe	.LBB32_332
.Ltmp6087:
	.loc	21 0 16 is_stmt 0
	movq	136(%rsp), %rdi
	leaq	(%rcx,%rdi,4), %r13
	subq	%rdi, %rdx
	movq	%rdx, 344(%rsp)
	movq	(%rsp), %rdx
	leaq	(%rdx,%rdi,4), %rdx
	movq	%rdx, 40(%rsp)
.Ltmp6088:
	.loc	25 451 16 is_stmt 1
	movl	%r9d, %edx
	subl	%r12d, %edx
	movq	%rdx, 368(%rsp)
	subq	%r10, %rdi
	movq	%rdi, 360(%rsp)
	movl	$1, %r8d
	vmovss	.LCPI32_5(%rip), %xmm3
	movq	%r9, %rdx
	vmovss	.LCPI32_28(%rip), %xmm5
	jmp	.LBB32_329
.Ltmp6089:
	.loc	25 0 16 is_stmt 0
.Ltmp6090:
	.p2align	4
.LBB32_435:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm6
	movl	60(%rsp), %r9d
.Ltmp6091:
	.loc	26 161 24
	cmovbel	56(%rsp), %r9d
.Ltmp6092:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6093:
	.loc	26 66 9
	vsubss	%xmm2, %xmm6, %xmm6
.Ltmp6094:
	.loc	26 92 9
	vmulss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm2, %xmm2
.Ltmp6095:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm4
	vandps	%xmm4, %xmm2, %xmm4
.Ltmp6096:
	.loc	26 166 24
	vcmpnltss	%xmm12, %xmm4, %xmm4
	vandps	%xmm2, %xmm4, %xmm2
	vmovss	.LCPI32_23(%rip), %xmm11
.Ltmp6097:
	.loc	26 71 9
	vmulss	%xmm11, %xmm15, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm12
.Ltmp6098:
	.loc	26 61 9
	vaddss	%xmm4, %xmm12, %xmm4
.Ltmp6099:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm4
	vmovss	.LCPI32_25(%rip), %xmm9
.Ltmp6100:
	.loc	26 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp6101:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm4
	vmovss	.LCPI32_26(%rip), %xmm0
.Ltmp6102:
	.loc	26 61 9
	vaddss	%xmm0, %xmm4, %xmm4
.Ltmp6103:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm4
	vmovss	.LCPI32_27(%rip), %xmm7
.Ltmp6104:
	.loc	26 61 9
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp6105:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm4
.Ltmp6106:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI32_20(%rip), %xmm2, %xmm6
.Ltmp6107:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI32_21(%rip), %xmm6, %xmm6
.Ltmp6108:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm6, %xmm6
	vmovss	.LCPI32_2(%rip), %xmm8
.Ltmp6109:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp6110:
	.loc	26 178 22
	vaddss	%xmm5, %xmm1, %xmm1
.Ltmp6111:
	.loc	7 1244 18
	vmovd	%xmm1, %r9d
.Ltmp6112:
	.loc	26 179 24
	shll	$23, %r9d
.Ltmp6113:
	.loc	7 1291 18
	vmovd	%r9d, %xmm1
.Ltmp6114:
	.loc	26 71 9
	vmulss	%xmm1, %xmm4, %xmm1
	vmovaps	256(%rsp), %xmm13
.Ltmp6115:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm13, %xmm1
	vmovaps	%xmm3, %xmm15
.Ltmp6116:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm3, %xmm14, %xmm4
	vblendvps	%xmm4, %xmm1, %xmm13, %xmm1
	vcmpnltss	84(%rsp), %xmm14, %xmm4
	vblendvps	%xmm4, %xmm1, %xmm13, %xmm1
.Ltmp6117:
	.loc	7 1783 9
	vroundss	$9, %xmm6, %xmm6, %xmm3
.Ltmp6118:
	.loc	26 66 9
	vsubss	%xmm3, %xmm6, %xmm4
.Ltmp6119:
	.loc	26 71 9
	vmulss	%xmm4, %xmm11, %xmm6
.Ltmp6120:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp6121:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6122:
	.loc	26 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp6123:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6124:
	.loc	26 61 9
	vaddss	%xmm0, %xmm6, %xmm6
.Ltmp6125:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6126:
	.loc	26 61 9
	vaddss	%xmm7, %xmm6, %xmm6
	vmovaps	%xmm15, %xmm7
.Ltmp6127:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm4
.Ltmp6128:
	.loc	26 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp6129:
	.loc	26 178 22
	vaddss	%xmm5, %xmm3, %xmm3
.Ltmp6130:
	.loc	7 1244 18
	vmovd	%xmm3, %r9d
.Ltmp6131:
	.loc	26 179 24
	shll	$23, %r9d
.Ltmp6132:
	.loc	7 1291 18
	vmovd	%r9d, %xmm3
.Ltmp6133:
	.loc	26 71 9
	vmulss	%xmm3, %xmm4, %xmm3
	vmovaps	272(%rsp), %xmm0
.Ltmp6134:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp6135:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm2, %xmm14, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm0, %xmm3
	vcmpnltss	52(%rsp), %xmm14, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm0, %xmm3
.Ltmp6136:
	.loc	21 510 5
	vmovss	%xmm2, 940(%r14)
.Ltmp6137:
	.loc	26 56 9
	vmovss	%xmm1, -4(%r13,%r8,4)
	movq	40(%rsp), %rdi
.Ltmp6138:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rdi,%r8,4)
	movq	360(%rsp), %rdi
.Ltmp6139:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rdi,%r8), %r9
	incq	%r9
	incq	%r8
	cmpq	$1, %r9
	vmovaps	%xmm10, %xmm9
	vmovss	.LCPI32_5(%rip), %xmm3
	vmovss	96(%rsp), %xmm0
	vmovss	112(%rsp), %xmm13
.Ltmp6140:
	.loc	11 900 12
	je	.LBB32_436
.Ltmp6141:
.LBB32_329:
	.loc	21 361 22
	leal	(%rdx,%r8), %r9d
	decl	%r9d
	andl	%ebp, %r9d
.Ltmp6142:
	.loc	25 451 16
	cmpq	%r9, %rax
	jbe	.LBB32_330
.Ltmp6143:
	.loc	25 0 16 is_stmt 0
	movq	376(%rsp), %r10
	addq	%r8, %r10
.Ltmp6144:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r13,%r8,4), %xmm1
.Ltmp6145:
	.loc	26 56 9
	vmovss	%xmm1, (%rbx,%r9,4)
.Ltmp6146:
	.loc	25 438 16
	cmpq	$1, %r10
	je	.LBB32_440
.Ltmp6147:
	.loc	25 451 16
	cmpq	%r9, %r11
	jbe	.LBB32_398
.Ltmp6148:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %rdi
.Ltmp6149:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rdi,%r8,4), %xmm1
.Ltmp6150:
	.loc	26 56 9
	vmovss	%xmm1, (%r15,%r9,4)
	movq	368(%rsp), %rdi
.Ltmp6151:
	.loc	21 370 21
	leal	(%rdi,%r8), %r9d
	decl	%r9d
	andl	%ebp, %r9d
.Ltmp6152:
	.loc	25 438 16
	cmpq	%r9, %rax
	jbe	.LBB32_401
.Ltmp6153:
	.loc	25 438 16 is_stmt 0
	cmpq	%r9, %r11
	jbe	.LBB32_403
.Ltmp6154:
	.loc	26 51 9 is_stmt 1
	vmovss	(%rbx,%r9,4), %xmm4
.Ltmp6155:
	.loc	26 51 9 is_stmt 0
	vmovss	(%r15,%r9,4), %xmm1
.Ltmp6156:
	.loc	21 0 0
	leal	(%rdx,%r8), %r12d
	movl	136(%r14), %r10d
	movl	200(%r14), %r9d
	notl	%r10d
	addl	%r12d, %r10d
	andl	%ebp, %r10d
	notl	%r9d
	addl	%r12d, %r9d
	andl	%ebp, %r9d
	.loc	21 229 5 is_stmt 1
	cmpb	$2, 32(%rsp)
	vmovaps	%xmm1, 272(%rsp)
	vmovaps	%xmm4, 256(%rsp)
	je	.LBB32_415
	cmpl	$1, 48(%rsp)
	jne	.LBB32_406
	.loc	21 0 0 is_stmt 0
	cmpq	%r10, %rax
.Ltmp6157:
	.loc	21 251 32 is_stmt 1
	jbe	.LBB32_339
.Ltmp6158:
	.loc	21 252 33
	cmpq	%r9, %r11
	jbe	.LBB32_413
.Ltmp6159:
	.loc	21 251 32
	vmovss	(%rbx,%r10,4), %xmm15
.Ltmp6160:
	.loc	21 252 33
	vmovss	(%r15,%r9,4), %xmm1
	vmovaps	%xmm1, %xmm6
	vmovaps	%xmm15, %xmm14
.Ltmp6161:
	.loc	26 51 9
	jmp	.LBB32_422
.Ltmp6162:
	.loc	26 0 9 is_stmt 0
.Ltmp6163:
	.p2align	4
.LBB32_415:
	cmpq	%r10, %rax
.Ltmp6164:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB32_416
	.loc	21 267 33
	cmpq	%r10, %r11
	jbe	.LBB32_441
	.loc	21 268 33
	cmpq	%r9, %r11
	jbe	.LBB32_419
	.loc	21 269 33
	cmpq	%r9, %rax
	jbe	.LBB32_442
	.loc	21 0 0 is_stmt 0
	vmovss	(%rbx,%r10,4), %xmm14
	vmovss	(%r15,%r10,4), %xmm6
	.loc	21 268 33 is_stmt 1
	vmovss	(%r15,%r9,4), %xmm1
	.loc	21 269 33
	vmovss	(%rbx,%r9,4), %xmm15
	jmp	.LBB32_422
.Ltmp6165:
	.loc	21 0 33 is_stmt 0
.Ltmp6166:
	.p2align	4
.LBB32_406:
	cmpq	%r10, %rax
.Ltmp6167:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB32_363
.Ltmp6168:
	.loc	21 237 33
	cmpq	%r9, %r11
	jbe	.LBB32_408
.Ltmp6169:
	.loc	21 236 32
	vmovss	(%rbx,%r10,4), %xmm6
.Ltmp6170:
	.loc	21 237 33
	vmovss	(%r15,%r9,4), %xmm15
	vmovaps	%xmm15, %xmm1
	vmovaps	%xmm6, %xmm14
.Ltmp6171:
.LBB32_422:
	.loc	21 0 33 is_stmt 0
	vbroadcastss	.LCPI32_4(%rip), %xmm8
.Ltmp6172:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm8, %xmm14, %xmm4
.Ltmp6173:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm8, %xmm6
.Ltmp6174:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm4
.Ltmp6175:
	.loc	7 1244 18
	vmovd	%xmm4, %r9d
.Ltmp6176:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %r10d
.Ltmp6177:
	.loc	26 161 24 is_stmt 1
	cmoval	%r9d, %r10d
	vxorps	%xmm14, %xmm14, %xmm14
	vmovss	24(%rsp), %xmm11
	vucomiss	%xmm14, %xmm11
.Ltmp6178:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r9d, %r10d
	vmovss	224(%rsp), %xmm11
	vucomiss	%xmm14, %xmm11
.Ltmp6179:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm4, %xmm4
.Ltmp6180:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm6, %xmm6
.Ltmp6181:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm6, %xmm4
.Ltmp6182:
	.loc	7 1244 18
	vmovd	%xmm4, %r9d
.Ltmp6183:
	.loc	26 161 24
	cmovbel	%r10d, %r9d
.Ltmp6184:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6185:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm4
.Ltmp6186:
	.loc	26 161 24
	movl	$841731191, %edi
	cmovbel	%edi, %r9d
.Ltmp6187:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6188:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm4
.Ltmp6189:
	.loc	26 161 24
	movl	$8388608, %edi
	cmovbel	%edi, %r9d
.Ltmp6190:
	.loc	26 185 42
	movl	%r9d, %r10d
	andl	$8388607, %r10d
	orl	$1065353216, %r10d
.Ltmp6191:
	.loc	7 1291 18
	vmovd	%r10d, %xmm4
	vmovss	.LCPI32_3(%rip), %xmm11
.Ltmp6192:
	.loc	26 66 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp6193:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm4, %xmm6
.Ltmp6194:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm12
	vsubss	%xmm6, %xmm12, %xmm6
.Ltmp6195:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
	vmovss	.LCPI32_10(%rip), %xmm12
.Ltmp6196:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp6197:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6198:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm6, %xmm6
.Ltmp6199:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6200:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm6, %xmm6
.Ltmp6201:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6202:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm6, %xmm6
.Ltmp6203:
	.loc	26 187 28
	shrl	$23, %r9d
	orl	$1258291200, %r9d
.Ltmp6204:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm4
.Ltmp6205:
	.loc	7 1291 18
	vmovd	%r9d, %xmm6
.Ltmp6206:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp6207:
	.loc	26 61 9
	vaddss	%xmm4, %xmm6, %xmm4
.Ltmp6208:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm4, %xmm4
.Ltmp6209:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm4, %xmm4
.Ltmp6210:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm4, %xmm6
.Ltmp6211:
	.loc	26 129 14 is_stmt 1
	vucomiss	16(%rsp), %xmm6
.Ltmp6212:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6213:
	.loc	26 129 14
	vucomiss	8(%rsp), %xmm6
.Ltmp6214:
	.loc	26 144 9
	movl	$0, %r10d
	adcl	$-1, %r10d
.Ltmp6215:
	.loc	26 149 9
	movl	%r12d, %r9d
.Ltmp6216:
	.loc	26 124 14
	vucomiss	%xmm14, %xmm0
.Ltmp6217:
	.loc	26 149 9
	notl	%r9d
.Ltmp6218:
	.loc	26 139 9
	cmovbel	%esi, %r9d
.Ltmp6219:
	.loc	21 478 20
	vmovss	856(%r14), %xmm4
.Ltmp6220:
	.loc	26 124 14
	vucomiss	%xmm14, %xmm4
.Ltmp6221:
	.loc	26 144 9
	cmoval	%r12d, %r10d
.Ltmp6222:
	.loc	26 139 9
	cmovbel	%esi, %r9d
.Ltmp6223:
	.loc	26 161 24
	testb	$1, %r9b
	jne	.LBB32_423
.Ltmp6224:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm0, %xmm4
	vmovss	152(%rsp), %xmm0
.Ltmp6225:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_426
	jmp	.LBB32_427
.Ltmp6226:
	.loc	26 0 24
.Ltmp6227:
	.p2align	4
.LBB32_423:
	vaddss	%xmm0, %xmm11, %xmm4
	vmovss	152(%rsp), %xmm0
.Ltmp6228:
	.loc	26 161 24
	testb	$1, %r10b
	jne	.LBB32_427
.Ltmp6229:
.LBB32_426:
	.loc	26 0 24
	vmovaps	%xmm4, %xmm0
.LBB32_427:
	movq	%r13, %rdi
	orl	%r10d, %r9d
	andl	$1065353216, %r9d
	vmovd	%r9d, %xmm4
.Ltmp6230:
	.loc	26 66 9 is_stmt 1
	vsubss	8(%rsp), %xmm6, %xmm6
.Ltmp6231:
	.loc	26 71 9
	vmulss	20(%rsp), %xmm6, %xmm6
.Ltmp6232:
	.loc	26 161 24
	vmaxss	288(%rsp), %xmm6, %xmm6
.Ltmp6233:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm4, %xmm14, %xmm4
	vcmpltss	%xmm14, %xmm6, %xmm11
	vandps	%xmm4, %xmm11, %xmm4
	vmovd	%xmm4, %r10d
	testb	$1, %r10b
	jne	.LBB32_429
.Ltmp6234:
	.loc	26 0 44
	vxorps	%xmm6, %xmm6, %xmm6
.LBB32_429:
.Ltmp6235:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm6
	movl	92(%rsp), %r12d
.Ltmp6236:
	.loc	26 161 24
	cmovbel	88(%rsp), %r12d
	vbroadcastss	.LCPI32_4(%rip), %xmm8
.Ltmp6237:
	.loc	26 103 24
	vandps	%xmm1, %xmm8, %xmm1
.Ltmp6238:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm8, %xmm15, %xmm4
.Ltmp6239:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm4, %xmm1
.Ltmp6240:
	.loc	7 1244 18
	vmovd	%xmm1, %r10d
.Ltmp6241:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm4, %r13d
.Ltmp6242:
	.loc	26 161 24 is_stmt 1
	cmoval	%r10d, %r13d
	vmovss	80(%rsp), %xmm11
	vucomiss	%xmm14, %xmm11
.Ltmp6243:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r10d, %r13d
.Ltmp6244:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp6245:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm4, %xmm4
.Ltmp6246:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm1, %xmm4, %xmm1
	vmovss	76(%rsp), %xmm4
	vucomiss	%xmm14, %xmm4
.Ltmp6247:
	.loc	7 1244 18
	vmovd	%xmm1, %r10d
.Ltmp6248:
	.loc	26 161 24
	cmovbel	%r13d, %r10d
.Ltmp6249:
	.loc	7 1291 18
	vmovd	%r10d, %xmm1
.Ltmp6250:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm1
.Ltmp6251:
	.loc	7 1291 18
	vmovd	%r12d, %xmm1
.Ltmp6252:
	.loc	26 66 9
	vsubss	%xmm7, %xmm6, %xmm4
.Ltmp6253:
	.loc	26 92 9
	vmulss	%xmm1, %xmm4, %xmm1
.Ltmp6254:
	.loc	26 161 24
	movl	$841731191, %r12d
	cmovbel	%r12d, %r10d
.Ltmp6255:
	.loc	26 92 9
	vaddss	%xmm1, %xmm7, %xmm1
.Ltmp6256:
	.loc	7 1291 18
	vmovd	%r10d, %xmm4
.Ltmp6257:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm4
.Ltmp6258:
	.loc	26 161 24
	movl	$8388608, %r12d
	cmovbel	%r12d, %r10d
.Ltmp6259:
	.loc	26 185 42
	movl	%r10d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp6260:
	.loc	7 1291 18
	vmovd	%r12d, %xmm4
	vmovss	.LCPI32_3(%rip), %xmm11
.Ltmp6261:
	.loc	26 66 9
	vaddss	%xmm4, %xmm11, %xmm4
.Ltmp6262:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm4, %xmm6
.Ltmp6263:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm7
	vsubss	%xmm6, %xmm7, %xmm6
.Ltmp6264:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6265:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp6266:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6267:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm6, %xmm6
.Ltmp6268:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6269:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm6, %xmm6
.Ltmp6270:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6271:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm6, %xmm6
.Ltmp6272:
	.loc	26 187 28
	shrl	$23, %r10d
	orl	$1258291200, %r10d
.Ltmp6273:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm4
.Ltmp6274:
	.loc	7 1291 18
	vmovd	%r10d, %xmm6
.Ltmp6275:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp6276:
	.loc	26 61 9
	vaddss	%xmm4, %xmm6, %xmm4
.Ltmp6277:
	.loc	26 103 24
	vandps	%xmm1, %xmm8, %xmm6
	vmovss	.LCPI32_19(%rip), %xmm12
.Ltmp6278:
	.loc	26 166 24
	vcmpnltss	%xmm12, %xmm6, %xmm7
.Ltmp6279:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm4, %xmm4
.Ltmp6280:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm4, %xmm4
.Ltmp6281:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm4, %xmm6
.Ltmp6282:
	.loc	26 129 14 is_stmt 1
	vucomiss	72(%rsp), %xmm6
.Ltmp6283:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp6284:
	.loc	26 129 14
	vucomiss	%xmm9, %xmm6
.Ltmp6285:
	.loc	26 166 24
	vandps	%xmm1, %xmm7, %xmm7
.Ltmp6286:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6287:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%r14), %xmm1
.Ltmp6288:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r10d
.Ltmp6289:
	.loc	26 124 14
	vucomiss	%xmm14, %xmm13
.Ltmp6290:
	.loc	26 149 9
	notl	%r10d
.Ltmp6291:
	.loc	26 139 9
	cmovbel	%esi, %r10d
.Ltmp6292:
	.loc	26 124 14
	vucomiss	%xmm14, %xmm1
.Ltmp6293:
	.loc	21 489 5
	vmovss	%xmm0, 860(%r14)
	.loc	21 498 5
	movl	%r9d, 856(%r14)
.Ltmp6294:
	.loc	21 510 5
	vmovss	%xmm7, 864(%r14)
.Ltmp6295:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp6296:
	.loc	26 139 9
	cmovbel	%esi, %r10d
.Ltmp6297:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_431
.Ltmp6298:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm13, %xmm13
.LBB32_431:
	movq	%rdi, %r13
	vmovss	68(%rsp), %xmm4
.Ltmp6299:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r12b
	jne	.LBB32_433
.Ltmp6300:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm4
.LBB32_433:
	vmovaps	%xmm4, %xmm10
	vmovss	%xmm0, 96(%rsp)
	vmovaps	%xmm7, %xmm3
.Ltmp6301:
	vmulss	.LCPI32_20(%rip), %xmm7, %xmm1
	vmaxss	.LCPI32_21(%rip), %xmm1, %xmm1
	vminss	.LCPI32_22(%rip), %xmm1, %xmm4
	vroundss	$9, %xmm4, %xmm4, %xmm1
	vsubss	%xmm1, %xmm4, %xmm15
.Ltmp6302:
	orl	%r12d, %r10d
	andl	$1065353216, %r10d
	vmovd	%r10d, %xmm4
	vmovss	%xmm10, 112(%rsp)
.Ltmp6303:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm10, 936(%r14)
	.loc	21 498 5
	movl	%r10d, 932(%r14)
	vmovaps	%xmm9, %xmm10
.Ltmp6304:
	.loc	26 66 9
	vsubss	%xmm9, %xmm6, %xmm6
.Ltmp6305:
	.loc	26 71 9
	vmulss	64(%rsp), %xmm6, %xmm6
.Ltmp6306:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm6, %xmm6
.Ltmp6307:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm4, %xmm14, %xmm4
	vcmpltss	%xmm14, %xmm6, %xmm11
	vandps	%xmm4, %xmm11, %xmm4
	vmovd	%xmm4, %r9d
	testb	$1, %r9b
	jne	.LBB32_435
.Ltmp6308:
	.loc	26 0 44
	vxorps	%xmm6, %xmm6, %xmm6
	jmp	.LBB32_435
.LBB32_332:
	movq	%r9, 40(%rsp)
.Ltmp6309:
	.loc	25 451 16 is_stmt 1
	negl	%r12d
	cmpq	%r11, %rax
	jbe	.LBB32_333
.Ltmp6310:
	.loc	25 0 16 is_stmt 0
	vmovss	.LCPI32_5(%rip), %xmm5
	xorl	%esi, %esi
	movq	40(%rsp), %rdi
	movq	136(%rsp), %r8
	movq	%r12, 272(%rsp)
	vmovss	.LCPI32_25(%rip), %xmm8
	jmp	.LBB32_366
	.p2align	4
.LBB32_392:
.Ltmp6311:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm15
	movl	60(%rsp), %r9d
.Ltmp6312:
	.loc	26 161 24
	cmovbel	56(%rsp), %r9d
.Ltmp6313:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6314:
	.loc	26 66 9
	vsubss	%xmm2, %xmm15, %xmm10
.Ltmp6315:
	.loc	26 92 9
	vmulss	%xmm4, %xmm10, %xmm4
	vaddss	%xmm4, %xmm2, %xmm2
.Ltmp6316:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm4
	vandps	%xmm4, %xmm2, %xmm4
.Ltmp6317:
	.loc	26 166 24
	vcmpnltss	%xmm1, %xmm4, %xmm4
	vandps	%xmm2, %xmm4, %xmm2
	vmovss	.LCPI32_23(%rip), %xmm1
.Ltmp6318:
	.loc	26 71 9
	vmulss	%xmm1, %xmm14, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm15
.Ltmp6319:
	.loc	26 61 9
	vaddss	%xmm4, %xmm15, %xmm4
.Ltmp6320:
	.loc	26 71 9
	vmulss	%xmm4, %xmm14, %xmm4
.Ltmp6321:
	.loc	26 61 9
	vaddss	%xmm4, %xmm8, %xmm4
.Ltmp6322:
	.loc	26 71 9
	vmulss	%xmm4, %xmm14, %xmm4
	vmovss	.LCPI32_26(%rip), %xmm5
.Ltmp6323:
	.loc	26 61 9
	vaddss	%xmm5, %xmm4, %xmm4
.Ltmp6324:
	.loc	26 71 9
	vmulss	%xmm4, %xmm14, %xmm4
	vmovss	.LCPI32_27(%rip), %xmm9
.Ltmp6325:
	.loc	26 61 9
	vaddss	%xmm4, %xmm9, %xmm4
.Ltmp6326:
	.loc	26 71 9
	vmulss	%xmm4, %xmm14, %xmm4
.Ltmp6327:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI32_20(%rip), %xmm2, %xmm10
.Ltmp6328:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI32_21(%rip), %xmm10, %xmm10
.Ltmp6329:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm10, %xmm10
	vmovss	.LCPI32_2(%rip), %xmm14
.Ltmp6330:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm14, %xmm4
	vmovss	.LCPI32_28(%rip), %xmm0
.Ltmp6331:
	.loc	26 178 22
	vaddss	%xmm0, %xmm12, %xmm12
.Ltmp6332:
	.loc	7 1244 18
	vmovd	%xmm12, %r9d
.Ltmp6333:
	.loc	26 179 24
	shll	$23, %r9d
.Ltmp6334:
	.loc	7 1291 18
	vmovd	%r9d, %xmm12
.Ltmp6335:
	.loc	26 71 9
	vmulss	%xmm4, %xmm12, %xmm4
	vmovaps	256(%rsp), %xmm6
.Ltmp6336:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm4, %xmm6, %xmm4
.Ltmp6337:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm11, %xmm12
	vblendvps	%xmm12, %xmm4, %xmm6, %xmm4
	vcmpnltss	84(%rsp), %xmm11, %xmm12
	vblendvps	%xmm12, %xmm4, %xmm6, %xmm4
.Ltmp6338:
	.loc	7 1783 9
	vroundss	$9, %xmm10, %xmm10, %xmm6
.Ltmp6339:
	.loc	26 66 9
	vsubss	%xmm6, %xmm10, %xmm10
.Ltmp6340:
	.loc	26 71 9
	vmulss	%xmm1, %xmm10, %xmm12
.Ltmp6341:
	.loc	26 61 9
	vaddss	%xmm15, %xmm12, %xmm12
.Ltmp6342:
	.loc	26 71 9
	vmulss	%xmm12, %xmm10, %xmm12
.Ltmp6343:
	.loc	26 61 9
	vaddss	%xmm8, %xmm12, %xmm12
.Ltmp6344:
	.loc	26 71 9
	vmulss	%xmm12, %xmm10, %xmm12
.Ltmp6345:
	.loc	26 61 9
	vaddss	%xmm5, %xmm12, %xmm12
.Ltmp6346:
	.loc	26 71 9
	vmulss	%xmm12, %xmm10, %xmm12
.Ltmp6347:
	.loc	26 61 9
	vaddss	%xmm9, %xmm12, %xmm12
.Ltmp6348:
	.loc	26 71 9
	vmulss	%xmm12, %xmm10, %xmm10
.Ltmp6349:
	.loc	26 61 9
	vaddss	%xmm14, %xmm10, %xmm10
.Ltmp6350:
	.loc	26 178 22
	vaddss	%xmm0, %xmm6, %xmm6
.Ltmp6351:
	.loc	7 1244 18
	vmovd	%xmm6, %r9d
.Ltmp6352:
	.loc	26 179 24
	shll	$23, %r9d
.Ltmp6353:
	.loc	7 1291 18
	vmovd	%r9d, %xmm6
.Ltmp6354:
	.loc	26 71 9
	vmulss	%xmm6, %xmm10, %xmm6
	vmovaps	112(%rsp), %xmm0
.Ltmp6355:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm6, %xmm0, %xmm6
.Ltmp6356:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm2, %xmm11, %xmm10
	vblendvps	%xmm10, %xmm6, %xmm0, %xmm6
	vcmpnltss	52(%rsp), %xmm11, %xmm10
	vblendvps	%xmm10, %xmm6, %xmm0, %xmm5
.Ltmp6357:
	.loc	21 510 5
	vmovss	%xmm2, 940(%r14)
.Ltmp6358:
	.loc	26 56 9
	vmovss	%xmm4, (%rcx,%r8,4)
	movq	(%rsp), %rdx
.Ltmp6359:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm5, (%rdx,%r8,4)
.Ltmp6360:
	.loc	8 1916 50 is_stmt 1
	incq	%r8
	incl	%edi
	cmpq	%r8, 144(%rsp)
	movq	40(%rsp), %rdx
	movq	272(%rsp), %r12
	vmovss	.LCPI32_5(%rip), %xmm5
	vmovaps	%xmm3, %xmm9
	vmovss	96(%rsp), %xmm0
.Ltmp6361:
	.loc	11 900 12
	je	.LBB32_436
.Ltmp6362:
.LBB32_366:
	.loc	21 361 22
	movl	%edi, %r9d
	andl	%ebp, %r9d
.Ltmp6363:
	.loc	25 451 16
	cmpq	%r9, %rax
	jbe	.LBB32_330
.Ltmp6364:
	.loc	26 51 9
	vmovss	(%rcx,%r8,4), %xmm4
.Ltmp6365:
	.loc	26 56 9
	vmovss	%xmm4, (%rbx,%r9,4)
.Ltmp6366:
	.loc	25 451 16
	cmpq	%r9, %r11
	jbe	.LBB32_398
.Ltmp6367:
	.loc	25 0 16 is_stmt 0
	movq	(%rsp), %rdx
.Ltmp6368:
	.loc	26 51 9 is_stmt 1
	vmovss	(%rdx,%r8,4), %xmm4
.Ltmp6369:
	.loc	26 56 9
	vmovss	%xmm4, (%r15,%r9,4)
.Ltmp6370:
	.loc	21 370 21
	leal	(%r12,%rdi), %r9d
	andl	%ebp, %r9d
.Ltmp6371:
	.loc	25 438 16
	cmpq	%r9, %rax
	jbe	.LBB32_401
.Ltmp6372:
	.loc	25 438 16 is_stmt 0
	cmpq	%r9, %r11
	jbe	.LBB32_403
.Ltmp6373:
	.loc	26 51 9 is_stmt 1
	vmovss	(%rbx,%r9,4), %xmm3
.Ltmp6374:
	.loc	26 51 9 is_stmt 0
	vmovss	(%r15,%r9,4), %xmm1
.Ltmp6375:
	.loc	21 0 0
	movl	%edi, %r10d
	subl	136(%r14), %r10d
	andl	%ebp, %r10d
	movl	%edi, %r9d
	subl	200(%r14), %r9d
	andl	%ebp, %r9d
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 32(%rsp)
	vmovaps	%xmm1, 112(%rsp)
	vmovaps	%xmm3, 256(%rsp)
	je	.LBB32_393
	cmpl	$1, 48(%rsp)
	jne	.LBB32_375
	.loc	21 0 0 is_stmt 0
	cmpq	%r10, %rax
	jbe	.LBB32_339
.Ltmp6376:
	.loc	21 252 33 is_stmt 1
	cmpq	%r9, %r11
	jbe	.LBB32_413
.Ltmp6377:
	.loc	21 251 32
	vmovss	(%rbx,%r10,4), %xmm12
.Ltmp6378:
	.loc	21 252 33
	vmovss	(%r15,%r9,4), %xmm14
	vmovaps	%xmm14, %xmm11
	vmovaps	%xmm12, %xmm15
	jmp	.LBB32_379
.Ltmp6379:
	.loc	21 0 33 is_stmt 0
.Ltmp6380:
	.p2align	4
.LBB32_393:
	cmpq	%r10, %rax
.Ltmp6381:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB32_363
.Ltmp6382:
	.loc	21 237 33
	cmpq	%r9, %r11
	jbe	.LBB32_408
.Ltmp6383:
	.loc	21 236 32
	vmovss	(%rbx,%r10,4), %xmm11
.Ltmp6384:
	.loc	21 237 33
	vmovss	(%r15,%r9,4), %xmm12
	vmovaps	%xmm12, %xmm14
	vmovaps	%xmm11, %xmm15
.Ltmp6385:
	.loc	26 51 9
	jmp	.LBB32_379
.Ltmp6386:
	.loc	26 0 9 is_stmt 0
.Ltmp6387:
	.p2align	4
.LBB32_375:
	cmpq	%r10, %rax
.Ltmp6388:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB32_416
	.loc	21 267 33
	cmpq	%r10, %r11
	jbe	.LBB32_441
	.loc	21 268 33
	cmpq	%r9, %r11
	jbe	.LBB32_419
	.loc	21 0 0 is_stmt 0
	vmovss	(%rbx,%r10,4), %xmm15
	.loc	21 267 33 is_stmt 1
	vmovss	(%r15,%r10,4), %xmm11
	.loc	21 268 33
	vmovss	(%r15,%r9,4), %xmm14
	.loc	21 269 33
	vmovss	(%rbx,%r9,4), %xmm12
.Ltmp6389:
.LBB32_379:
	.loc	21 0 33 is_stmt 0
	vbroadcastss	.LCPI32_4(%rip), %xmm1
.Ltmp6390:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm1, %xmm15, %xmm4
.Ltmp6391:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm1, %xmm11, %xmm15
.Ltmp6392:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm4
.Ltmp6393:
	.loc	7 1244 18
	vmovd	%xmm4, %r9d
.Ltmp6394:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm15, %r10d
.Ltmp6395:
	.loc	26 161 24 is_stmt 1
	cmoval	%r9d, %r10d
	vxorps	%xmm11, %xmm11, %xmm11
	vmovss	24(%rsp), %xmm10
	vucomiss	%xmm11, %xmm10
.Ltmp6396:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r9d, %r10d
	vmovss	224(%rsp), %xmm10
	vucomiss	%xmm11, %xmm10
.Ltmp6397:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm4, %xmm4
.Ltmp6398:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm5, %xmm15, %xmm15
.Ltmp6399:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm15, %xmm4
.Ltmp6400:
	.loc	7 1244 18
	vmovd	%xmm4, %r9d
.Ltmp6401:
	.loc	26 161 24
	cmovbel	%r10d, %r9d
.Ltmp6402:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6403:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm4
.Ltmp6404:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %r9d
.Ltmp6405:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6406:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm4
.Ltmp6407:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %r9d
.Ltmp6408:
	.loc	26 185 42
	movl	%r9d, %r10d
	andl	$8388607, %r10d
	orl	$1065353216, %r10d
.Ltmp6409:
	.loc	7 1291 18
	vmovd	%r10d, %xmm4
	vmovss	.LCPI32_3(%rip), %xmm10
.Ltmp6410:
	.loc	26 66 9
	vaddss	%xmm4, %xmm10, %xmm4
.Ltmp6411:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm4, %xmm15
.Ltmp6412:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm1
	vsubss	%xmm15, %xmm1, %xmm15
.Ltmp6413:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm15
.Ltmp6414:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm15, %xmm15
.Ltmp6415:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm15
.Ltmp6416:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm15, %xmm15
.Ltmp6417:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm15
.Ltmp6418:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm15, %xmm15
.Ltmp6419:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm15
.Ltmp6420:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm15, %xmm15
.Ltmp6421:
	.loc	26 187 28
	shrl	$23, %r9d
	orl	$1258291200, %r9d
.Ltmp6422:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm4
.Ltmp6423:
	.loc	7 1291 18
	vmovd	%r9d, %xmm15
.Ltmp6424:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm15, %xmm15
.Ltmp6425:
	.loc	26 61 9
	vaddss	%xmm4, %xmm15, %xmm4
.Ltmp6426:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm4, %xmm4
.Ltmp6427:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm4, %xmm4
.Ltmp6428:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm4, %xmm15
.Ltmp6429:
	.loc	26 129 14 is_stmt 1
	vucomiss	16(%rsp), %xmm15
.Ltmp6430:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6431:
	.loc	26 129 14
	vucomiss	8(%rsp), %xmm15
.Ltmp6432:
	.loc	26 144 9
	movl	$0, %r10d
	adcl	$-1, %r10d
.Ltmp6433:
	.loc	26 149 9
	movl	%r12d, %r9d
.Ltmp6434:
	.loc	26 124 14
	vucomiss	%xmm11, %xmm0
.Ltmp6435:
	.loc	26 149 9
	notl	%r9d
.Ltmp6436:
	.loc	26 139 9
	cmovbel	%esi, %r9d
.Ltmp6437:
	.loc	21 478 20
	vmovss	856(%r14), %xmm4
.Ltmp6438:
	.loc	26 124 14
	vucomiss	%xmm11, %xmm4
.Ltmp6439:
	.loc	26 144 9
	cmoval	%r12d, %r10d
.Ltmp6440:
	.loc	26 139 9
	cmovbel	%esi, %r9d
.Ltmp6441:
	.loc	26 161 24
	testb	$1, %r9b
	jne	.LBB32_380
.Ltmp6442:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm0, %xmm4
	vmovss	152(%rsp), %xmm0
.Ltmp6443:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_383
	jmp	.LBB32_384
.Ltmp6444:
	.loc	26 0 24
.Ltmp6445:
	.p2align	4
.LBB32_380:
	vaddss	%xmm0, %xmm10, %xmm4
	vmovss	152(%rsp), %xmm0
.Ltmp6446:
	.loc	26 161 24
	testb	$1, %r10b
	jne	.LBB32_384
.Ltmp6447:
.LBB32_383:
	.loc	26 0 24
	vmovaps	%xmm4, %xmm0
.LBB32_384:
	orl	%r10d, %r9d
	andl	$1065353216, %r9d
	vmovd	%r9d, %xmm4
.Ltmp6448:
	.loc	26 66 9 is_stmt 1
	vsubss	8(%rsp), %xmm15, %xmm15
.Ltmp6449:
	.loc	26 71 9
	vmulss	20(%rsp), %xmm15, %xmm15
.Ltmp6450:
	.loc	26 161 24
	vmaxss	288(%rsp), %xmm15, %xmm15
.Ltmp6451:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm4, %xmm11, %xmm4
	vcmpltss	%xmm11, %xmm15, %xmm10
	vandps	%xmm4, %xmm10, %xmm4
	vmovd	%xmm4, %r10d
	testb	$1, %r10b
	jne	.LBB32_386
.Ltmp6452:
	.loc	26 0 44
	vxorps	%xmm15, %xmm15, %xmm15
.LBB32_386:
.Ltmp6453:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm15
	movl	92(%rsp), %r12d
.Ltmp6454:
	.loc	26 161 24
	cmovbel	88(%rsp), %r12d
	vbroadcastss	.LCPI32_4(%rip), %xmm3
.Ltmp6455:
	.loc	26 103 24
	vandps	%xmm3, %xmm14, %xmm4
.Ltmp6456:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm3, %xmm12, %xmm10
.Ltmp6457:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm4
.Ltmp6458:
	.loc	7 1244 18
	vmovd	%xmm4, %r10d
.Ltmp6459:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm10, %r13d
.Ltmp6460:
	.loc	26 161 24 is_stmt 1
	cmoval	%r10d, %r13d
	vmovss	80(%rsp), %xmm12
	vucomiss	%xmm11, %xmm12
.Ltmp6461:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r10d, %r13d
.Ltmp6462:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm4, %xmm4
.Ltmp6463:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm5, %xmm10, %xmm10
.Ltmp6464:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm10, %xmm4
	vmovss	76(%rsp), %xmm10
	vucomiss	%xmm11, %xmm10
.Ltmp6465:
	.loc	7 1244 18
	vmovd	%xmm4, %r10d
.Ltmp6466:
	.loc	26 161 24
	cmovbel	%r13d, %r10d
.Ltmp6467:
	.loc	7 1291 18
	vmovd	%r10d, %xmm4
.Ltmp6468:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm4
.Ltmp6469:
	.loc	7 1291 18
	vmovd	%r12d, %xmm4
.Ltmp6470:
	.loc	26 66 9
	vsubss	%xmm7, %xmm15, %xmm10
.Ltmp6471:
	.loc	26 92 9
	vmulss	%xmm4, %xmm10, %xmm4
.Ltmp6472:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %r10d
.Ltmp6473:
	.loc	26 92 9
	vaddss	%xmm4, %xmm7, %xmm7
.Ltmp6474:
	.loc	7 1291 18
	vmovd	%r10d, %xmm4
.Ltmp6475:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm4
.Ltmp6476:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %r10d
.Ltmp6477:
	.loc	26 185 42
	movl	%r10d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp6478:
	.loc	7 1291 18
	vmovd	%r12d, %xmm4
	vmovss	.LCPI32_3(%rip), %xmm12
.Ltmp6479:
	.loc	26 66 9
	vaddss	%xmm4, %xmm12, %xmm4
.Ltmp6480:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm4, %xmm10
.Ltmp6481:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm1
	vsubss	%xmm10, %xmm1, %xmm10
.Ltmp6482:
	.loc	26 71 9
	vmulss	%xmm4, %xmm10, %xmm10
.Ltmp6483:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm10, %xmm10
.Ltmp6484:
	.loc	26 71 9
	vmulss	%xmm4, %xmm10, %xmm10
.Ltmp6485:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm10, %xmm10
.Ltmp6486:
	.loc	26 71 9
	vmulss	%xmm4, %xmm10, %xmm10
.Ltmp6487:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm10, %xmm10
.Ltmp6488:
	.loc	26 71 9
	vmulss	%xmm4, %xmm10, %xmm10
.Ltmp6489:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm10, %xmm10
.Ltmp6490:
	.loc	26 187 28
	shrl	$23, %r10d
	orl	$1258291200, %r10d
.Ltmp6491:
	.loc	26 71 9
	vmulss	%xmm4, %xmm10, %xmm4
.Ltmp6492:
	.loc	7 1291 18
	vmovd	%r10d, %xmm10
.Ltmp6493:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm10, %xmm10
.Ltmp6494:
	.loc	26 61 9
	vaddss	%xmm4, %xmm10, %xmm4
.Ltmp6495:
	.loc	26 103 24
	vandps	%xmm3, %xmm7, %xmm10
	vmovss	.LCPI32_19(%rip), %xmm1
.Ltmp6496:
	.loc	26 166 24
	vcmpnltss	%xmm1, %xmm10, %xmm10
.Ltmp6497:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm4, %xmm4
.Ltmp6498:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm4, %xmm4
.Ltmp6499:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm4, %xmm15
.Ltmp6500:
	.loc	26 129 14 is_stmt 1
	vucomiss	72(%rsp), %xmm15
.Ltmp6501:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp6502:
	.loc	26 129 14
	vucomiss	%xmm9, %xmm15
.Ltmp6503:
	.loc	26 166 24
	vandps	%xmm7, %xmm10, %xmm7
.Ltmp6504:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6505:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%r14), %xmm4
.Ltmp6506:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r10d
.Ltmp6507:
	.loc	26 124 14
	vucomiss	%xmm11, %xmm13
.Ltmp6508:
	.loc	26 149 9
	notl	%r10d
.Ltmp6509:
	.loc	26 139 9
	cmovbel	%esi, %r10d
.Ltmp6510:
	.loc	26 124 14
	vucomiss	%xmm11, %xmm4
.Ltmp6511:
	.loc	21 489 5
	vmovss	%xmm0, 860(%r14)
	.loc	21 498 5
	movl	%r9d, 856(%r14)
.Ltmp6512:
	.loc	21 510 5
	vmovss	%xmm7, 864(%r14)
.Ltmp6513:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp6514:
	.loc	26 139 9
	cmovbel	%esi, %r10d
.Ltmp6515:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_388
.Ltmp6516:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm12, %xmm13, %xmm13
.LBB32_388:
	vmovss	68(%rsp), %xmm3
.Ltmp6517:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r12b
	jne	.LBB32_390
.Ltmp6518:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm3
.LBB32_390:
	vmovss	%xmm0, 96(%rsp)
.Ltmp6519:
	vmulss	.LCPI32_20(%rip), %xmm7, %xmm4
	vmaxss	.LCPI32_21(%rip), %xmm4, %xmm4
	vminss	.LCPI32_22(%rip), %xmm4, %xmm4
	vroundss	$9, %xmm4, %xmm4, %xmm12
	vsubss	%xmm12, %xmm4, %xmm14
.Ltmp6520:
	orl	%r12d, %r10d
	andl	$1065353216, %r10d
	vmovd	%r10d, %xmm4
	vmovaps	%xmm3, %xmm13
.Ltmp6521:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm3, 936(%r14)
	.loc	21 498 5
	movl	%r10d, 932(%r14)
	vmovaps	%xmm9, %xmm3
.Ltmp6522:
	.loc	26 66 9
	vsubss	%xmm9, %xmm15, %xmm10
.Ltmp6523:
	.loc	26 71 9
	vmulss	64(%rsp), %xmm10, %xmm10
.Ltmp6524:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm10, %xmm15
.Ltmp6525:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm4, %xmm11, %xmm4
	vcmpltss	%xmm11, %xmm15, %xmm10
	vandps	%xmm4, %xmm10, %xmm4
	vmovd	%xmm4, %r9d
	testb	$1, %r9b
	jne	.LBB32_392
.Ltmp6526:
	.loc	26 0 44
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB32_392
.LBB32_333:
	vmovss	.LCPI32_5(%rip), %xmm3
	xorl	%esi, %esi
	movq	40(%rsp), %r8
	movq	136(%rsp), %rdi
	movq	%r12, 272(%rsp)
	jmp	.LBB32_334
	.p2align	4
.LBB32_359:
.Ltmp6527:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm15
	movl	60(%rsp), %r9d
.Ltmp6528:
	.loc	26 161 24
	cmovbel	56(%rsp), %r9d
.Ltmp6529:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6530:
	.loc	26 66 9
	vsubss	%xmm2, %xmm15, %xmm6
.Ltmp6531:
	.loc	26 92 9
	vmulss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm2, %xmm2
.Ltmp6532:
	.loc	26 103 24
	vbroadcastss	.LCPI32_4(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm4
.Ltmp6533:
	.loc	26 166 24
	vcmpnltss	%xmm8, %xmm4, %xmm4
	vandps	%xmm2, %xmm4, %xmm2
	vmovss	.LCPI32_23(%rip), %xmm8
.Ltmp6534:
	.loc	26 71 9
	vmulss	%xmm8, %xmm14, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm15
.Ltmp6535:
	.loc	26 61 9
	vaddss	%xmm4, %xmm15, %xmm4
.Ltmp6536:
	.loc	26 71 9
	vmulss	%xmm4, %xmm14, %xmm4
	vmovss	.LCPI32_25(%rip), %xmm3
.Ltmp6537:
	.loc	26 61 9
	vaddss	%xmm3, %xmm4, %xmm4
.Ltmp6538:
	.loc	26 71 9
	vmulss	%xmm4, %xmm14, %xmm4
	vmovss	.LCPI32_26(%rip), %xmm5
.Ltmp6539:
	.loc	26 61 9
	vaddss	%xmm5, %xmm4, %xmm4
.Ltmp6540:
	.loc	26 71 9
	vmulss	%xmm4, %xmm14, %xmm4
	vmovss	.LCPI32_27(%rip), %xmm1
.Ltmp6541:
	.loc	26 61 9
	vaddss	%xmm1, %xmm4, %xmm4
.Ltmp6542:
	.loc	26 71 9
	vmulss	%xmm4, %xmm14, %xmm4
.Ltmp6543:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI32_20(%rip), %xmm2, %xmm6
.Ltmp6544:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI32_21(%rip), %xmm6, %xmm6
.Ltmp6545:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI32_22(%rip), %xmm6, %xmm6
	vmovss	.LCPI32_2(%rip), %xmm14
.Ltmp6546:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm14, %xmm4
	vmovss	.LCPI32_28(%rip), %xmm9
.Ltmp6547:
	.loc	26 178 22
	vaddss	%xmm9, %xmm12, %xmm12
.Ltmp6548:
	.loc	7 1244 18
	vmovd	%xmm12, %r9d
.Ltmp6549:
	.loc	26 179 24
	shll	$23, %r9d
.Ltmp6550:
	.loc	7 1291 18
	vmovd	%r9d, %xmm12
.Ltmp6551:
	.loc	26 71 9
	vmulss	%xmm4, %xmm12, %xmm4
	vmovaps	256(%rsp), %xmm10
.Ltmp6552:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm4, %xmm10, %xmm4
.Ltmp6553:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm11, %xmm12
	vblendvps	%xmm12, %xmm4, %xmm10, %xmm4
	vcmpnltss	84(%rsp), %xmm11, %xmm12
	vblendvps	%xmm12, %xmm4, %xmm10, %xmm4
.Ltmp6554:
	.loc	7 1783 9
	vroundss	$9, %xmm6, %xmm6, %xmm10
.Ltmp6555:
	.loc	26 66 9
	vsubss	%xmm10, %xmm6, %xmm6
.Ltmp6556:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm12
.Ltmp6557:
	.loc	26 61 9
	vaddss	%xmm15, %xmm12, %xmm12
.Ltmp6558:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm12
.Ltmp6559:
	.loc	26 61 9
	vaddss	%xmm3, %xmm12, %xmm12
.Ltmp6560:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm12
.Ltmp6561:
	.loc	26 61 9
	vaddss	%xmm5, %xmm12, %xmm12
.Ltmp6562:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm12
.Ltmp6563:
	.loc	26 61 9
	vaddss	%xmm1, %xmm12, %xmm12
.Ltmp6564:
	.loc	26 71 9
	vmulss	%xmm6, %xmm12, %xmm6
.Ltmp6565:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp6566:
	.loc	26 178 22
	vaddss	%xmm9, %xmm10, %xmm10
.Ltmp6567:
	.loc	7 1244 18
	vmovd	%xmm10, %r9d
.Ltmp6568:
	.loc	26 179 24
	shll	$23, %r9d
.Ltmp6569:
	.loc	7 1291 18
	vmovd	%r9d, %xmm10
.Ltmp6570:
	.loc	26 71 9
	vmulss	%xmm6, %xmm10, %xmm6
	vmovaps	96(%rsp), %xmm1
.Ltmp6571:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm6, %xmm1, %xmm6
.Ltmp6572:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm2, %xmm11, %xmm10
	vblendvps	%xmm10, %xmm6, %xmm1, %xmm6
	vcmpnltss	52(%rsp), %xmm11, %xmm10
	vblendvps	%xmm10, %xmm6, %xmm1, %xmm5
.Ltmp6573:
	.loc	21 510 5
	vmovss	%xmm2, 940(%r14)
.Ltmp6574:
	.loc	26 56 9
	vmovss	%xmm4, (%rcx,%rdi,4)
	movq	(%rsp), %rdx
.Ltmp6575:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm5, (%rdx,%rdi,4)
.Ltmp6576:
	.loc	8 1916 50 is_stmt 1
	incq	%rdi
	incl	%r8d
	cmpq	%rdi, 144(%rsp)
	movq	40(%rsp), %rdx
	movq	272(%rsp), %r12
	vmovss	.LCPI32_5(%rip), %xmm3
	vmovaps	%xmm13, %xmm9
	vmovss	112(%rsp), %xmm13
.Ltmp6577:
	.loc	11 900 12
	je	.LBB32_436
.Ltmp6578:
.LBB32_334:
	.loc	21 361 22
	movl	%r8d, %r9d
	andl	%ebp, %r9d
.Ltmp6579:
	.loc	25 451 16
	cmpq	%r9, %rax
	jbe	.LBB32_330
.Ltmp6580:
	.loc	26 51 9
	vmovss	(%rcx,%rdi,4), %xmm4
.Ltmp6581:
	.loc	26 56 9
	vmovss	%xmm4, (%rbx,%r9,4)
	movq	(%rsp), %rdx
.Ltmp6582:
	.loc	26 51 9
	vmovss	(%rdx,%rdi,4), %xmm4
.Ltmp6583:
	.loc	26 56 9
	vmovss	%xmm4, (%r15,%r9,4)
.Ltmp6584:
	.loc	21 370 21
	leal	(%r12,%r8), %r9d
	andl	%ebp, %r9d
.Ltmp6585:
	.loc	25 438 16
	cmpq	%r9, %rax
	jbe	.LBB32_401
.Ltmp6586:
	.loc	26 51 9
	vmovss	(%rbx,%r9,4), %xmm4
.Ltmp6587:
	.loc	26 51 9 is_stmt 0
	vmovss	(%r15,%r9,4), %xmm1
.Ltmp6588:
	.loc	21 0 0
	movl	%r8d, %r10d
	subl	136(%r14), %r10d
	andl	%ebp, %r10d
	movl	%r8d, %r9d
	subl	200(%r14), %r9d
	andl	%ebp, %r9d
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 32(%rsp)
	vmovaps	%xmm1, 96(%rsp)
	vmovaps	%xmm4, 256(%rsp)
	je	.LBB32_362
	cmpl	$1, 48(%rsp)
	jne	.LBB32_340
	.loc	21 0 0 is_stmt 0
	cmpq	%r10, %rax
.Ltmp6589:
	.loc	21 251 32 is_stmt 1
	jbe	.LBB32_339
.Ltmp6590:
	.loc	21 252 33
	cmpq	%r9, %r11
	jbe	.LBB32_413
.Ltmp6591:
	.loc	21 251 32
	vmovss	(%rbx,%r10,4), %xmm12
.Ltmp6592:
	.loc	21 252 33
	vmovss	(%r15,%r9,4), %xmm14
	vmovaps	%xmm14, %xmm11
	vmovaps	%xmm12, %xmm15
	jmp	.LBB32_346
.Ltmp6593:
	.loc	21 0 33 is_stmt 0
.Ltmp6594:
	.p2align	4
.LBB32_362:
	cmpq	%r10, %rax
.Ltmp6595:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB32_363
.Ltmp6596:
	.loc	21 237 33
	cmpq	%r9, %r11
	jbe	.LBB32_408
.Ltmp6597:
	.loc	21 236 32
	vmovss	(%rbx,%r10,4), %xmm11
.Ltmp6598:
	.loc	21 237 33
	vmovss	(%r15,%r9,4), %xmm12
	vmovaps	%xmm12, %xmm14
	vmovaps	%xmm11, %xmm15
.Ltmp6599:
	.loc	26 51 9
	jmp	.LBB32_346
.Ltmp6600:
	.loc	26 0 9 is_stmt 0
.Ltmp6601:
	.p2align	4
.LBB32_340:
	cmpq	%r10, %rax
.Ltmp6602:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB32_416
	.loc	21 268 33
	cmpq	%r9, %r11
	jbe	.LBB32_419
	.loc	21 269 33
	cmpq	%r9, %rax
	jbe	.LBB32_442
	.loc	21 0 0 is_stmt 0
	vmovss	(%rbx,%r10,4), %xmm15
	vmovss	(%r15,%r10,4), %xmm11
	.loc	21 268 33 is_stmt 1
	vmovss	(%r15,%r9,4), %xmm14
	.loc	21 269 33
	vmovss	(%rbx,%r9,4), %xmm12
.Ltmp6603:
.LBB32_346:
	.loc	21 0 33 is_stmt 0
	vbroadcastss	.LCPI32_4(%rip), %xmm1
.Ltmp6604:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm1, %xmm15, %xmm4
.Ltmp6605:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm1, %xmm11, %xmm15
.Ltmp6606:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm4
.Ltmp6607:
	.loc	7 1244 18
	vmovd	%xmm4, %r9d
.Ltmp6608:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm15, %r10d
.Ltmp6609:
	.loc	26 161 24 is_stmt 1
	cmoval	%r9d, %r10d
	vxorps	%xmm11, %xmm11, %xmm11
	vmovss	24(%rsp), %xmm6
	vucomiss	%xmm11, %xmm6
.Ltmp6610:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r9d, %r10d
	vmovss	224(%rsp), %xmm6
	vucomiss	%xmm11, %xmm6
.Ltmp6611:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm4, %xmm4
.Ltmp6612:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm15, %xmm15
.Ltmp6613:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm15, %xmm4
.Ltmp6614:
	.loc	7 1244 18
	vmovd	%xmm4, %r9d
.Ltmp6615:
	.loc	26 161 24
	cmovbel	%r10d, %r9d
.Ltmp6616:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6617:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm4
.Ltmp6618:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %r9d
.Ltmp6619:
	.loc	7 1291 18
	vmovd	%r9d, %xmm4
.Ltmp6620:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm4
.Ltmp6621:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %r9d
.Ltmp6622:
	.loc	26 185 42
	movl	%r9d, %r10d
	andl	$8388607, %r10d
	orl	$1065353216, %r10d
.Ltmp6623:
	.loc	7 1291 18
	vmovd	%r10d, %xmm4
	vmovss	.LCPI32_3(%rip), %xmm6
.Ltmp6624:
	.loc	26 66 9
	vaddss	%xmm6, %xmm4, %xmm4
.Ltmp6625:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm4, %xmm15
.Ltmp6626:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm8
	vsubss	%xmm15, %xmm8, %xmm15
.Ltmp6627:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm15
.Ltmp6628:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm15, %xmm15
.Ltmp6629:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm15
.Ltmp6630:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm15, %xmm15
.Ltmp6631:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm15
.Ltmp6632:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm15, %xmm15
.Ltmp6633:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm15
.Ltmp6634:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm15, %xmm15
.Ltmp6635:
	.loc	26 187 28
	shrl	$23, %r9d
	orl	$1258291200, %r9d
.Ltmp6636:
	.loc	26 71 9
	vmulss	%xmm4, %xmm15, %xmm4
.Ltmp6637:
	.loc	7 1291 18
	vmovd	%r9d, %xmm15
.Ltmp6638:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm15, %xmm15
.Ltmp6639:
	.loc	26 61 9
	vaddss	%xmm4, %xmm15, %xmm4
.Ltmp6640:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm4, %xmm4
.Ltmp6641:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm4, %xmm4
.Ltmp6642:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm4, %xmm15
.Ltmp6643:
	.loc	26 129 14 is_stmt 1
	vucomiss	16(%rsp), %xmm15
.Ltmp6644:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6645:
	.loc	26 129 14
	vucomiss	8(%rsp), %xmm15
.Ltmp6646:
	.loc	26 144 9
	movl	$0, %r10d
	adcl	$-1, %r10d
.Ltmp6647:
	.loc	26 149 9
	movl	%r12d, %r9d
.Ltmp6648:
	.loc	26 124 14
	vucomiss	%xmm11, %xmm0
.Ltmp6649:
	.loc	26 149 9
	notl	%r9d
.Ltmp6650:
	.loc	26 139 9
	cmovbel	%esi, %r9d
.Ltmp6651:
	.loc	21 478 20
	vmovss	856(%r14), %xmm4
.Ltmp6652:
	.loc	26 124 14
	vucomiss	%xmm11, %xmm4
.Ltmp6653:
	.loc	26 144 9
	cmoval	%r12d, %r10d
.Ltmp6654:
	.loc	26 139 9
	cmovbel	%esi, %r9d
.Ltmp6655:
	.loc	26 161 24
	testb	$1, %r9b
	jne	.LBB32_347
.Ltmp6656:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm0, %xmm4
	vmovss	152(%rsp), %xmm0
.Ltmp6657:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_350
	jmp	.LBB32_351
.Ltmp6658:
	.loc	26 0 24
.Ltmp6659:
	.p2align	4
.LBB32_347:
	vaddss	%xmm6, %xmm0, %xmm4
	vmovss	152(%rsp), %xmm0
.Ltmp6660:
	.loc	26 161 24
	testb	$1, %r10b
	jne	.LBB32_351
.Ltmp6661:
.LBB32_350:
	.loc	26 0 24
	vmovaps	%xmm4, %xmm0
.LBB32_351:
	orl	%r10d, %r9d
	andl	$1065353216, %r9d
	vmovd	%r9d, %xmm4
.Ltmp6662:
	.loc	26 66 9 is_stmt 1
	vsubss	8(%rsp), %xmm15, %xmm15
.Ltmp6663:
	.loc	26 71 9
	vmulss	20(%rsp), %xmm15, %xmm15
.Ltmp6664:
	.loc	26 161 24
	vmaxss	288(%rsp), %xmm15, %xmm15
.Ltmp6665:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm4, %xmm11, %xmm4
	vcmpltss	%xmm11, %xmm15, %xmm6
	vandps	%xmm6, %xmm4, %xmm4
	vmovd	%xmm4, %r10d
	testb	$1, %r10b
	jne	.LBB32_353
.Ltmp6666:
	.loc	26 0 44
	vxorps	%xmm15, %xmm15, %xmm15
.LBB32_353:
.Ltmp6667:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm15
	movl	92(%rsp), %r12d
.Ltmp6668:
	.loc	26 161 24
	cmovbel	88(%rsp), %r12d
	vbroadcastss	.LCPI32_4(%rip), %xmm1
.Ltmp6669:
	.loc	26 103 24
	vandps	%xmm1, %xmm14, %xmm4
.Ltmp6670:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm1, %xmm12, %xmm6
.Ltmp6671:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm4
.Ltmp6672:
	.loc	7 1244 18
	vmovd	%xmm4, %r10d
.Ltmp6673:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %r13d
.Ltmp6674:
	.loc	26 161 24 is_stmt 1
	cmoval	%r10d, %r13d
	vmovss	80(%rsp), %xmm12
	vucomiss	%xmm11, %xmm12
.Ltmp6675:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r10d, %r13d
.Ltmp6676:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm4, %xmm4
.Ltmp6677:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm6, %xmm6
.Ltmp6678:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm6, %xmm4
	vmovss	76(%rsp), %xmm6
	vucomiss	%xmm11, %xmm6
.Ltmp6679:
	.loc	7 1244 18
	vmovd	%xmm4, %r10d
.Ltmp6680:
	.loc	26 161 24
	cmovbel	%r13d, %r10d
.Ltmp6681:
	.loc	7 1291 18
	vmovd	%r10d, %xmm4
.Ltmp6682:
	.loc	26 124 14
	vucomiss	.LCPI32_6(%rip), %xmm4
.Ltmp6683:
	.loc	7 1291 18
	vmovd	%r12d, %xmm4
.Ltmp6684:
	.loc	26 66 9
	vsubss	%xmm7, %xmm15, %xmm6
.Ltmp6685:
	.loc	26 92 9
	vmulss	%xmm4, %xmm6, %xmm4
.Ltmp6686:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %r10d
.Ltmp6687:
	.loc	26 92 9
	vaddss	%xmm4, %xmm7, %xmm7
.Ltmp6688:
	.loc	7 1291 18
	vmovd	%r10d, %xmm4
.Ltmp6689:
	.loc	26 124 14
	vucomiss	.LCPI32_7(%rip), %xmm4
.Ltmp6690:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %r10d
.Ltmp6691:
	.loc	26 185 42
	movl	%r10d, %r12d
	andl	$8388607, %r12d
	orl	$1065353216, %r12d
.Ltmp6692:
	.loc	7 1291 18
	vmovd	%r12d, %xmm4
	vmovss	.LCPI32_3(%rip), %xmm12
.Ltmp6693:
	.loc	26 66 9
	vaddss	%xmm4, %xmm12, %xmm4
.Ltmp6694:
	.loc	26 71 9
	vmulss	.LCPI32_8(%rip), %xmm4, %xmm6
.Ltmp6695:
	.loc	26 61 9
	vmovss	.LCPI32_9(%rip), %xmm8
	vsubss	%xmm6, %xmm8, %xmm6
.Ltmp6696:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6697:
	.loc	26 61 9
	vaddss	.LCPI32_10(%rip), %xmm6, %xmm6
.Ltmp6698:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6699:
	.loc	26 61 9
	vaddss	.LCPI32_11(%rip), %xmm6, %xmm6
.Ltmp6700:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6701:
	.loc	26 61 9
	vaddss	.LCPI32_12(%rip), %xmm6, %xmm6
.Ltmp6702:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp6703:
	.loc	26 61 9
	vaddss	.LCPI32_13(%rip), %xmm6, %xmm6
.Ltmp6704:
	.loc	26 187 28
	shrl	$23, %r10d
	orl	$1258291200, %r10d
.Ltmp6705:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm4
.Ltmp6706:
	.loc	7 1291 18
	vmovd	%r10d, %xmm6
.Ltmp6707:
	.loc	26 187 13
	vaddss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp6708:
	.loc	26 61 9
	vaddss	%xmm4, %xmm6, %xmm4
.Ltmp6709:
	.loc	26 103 24
	vandps	%xmm1, %xmm7, %xmm6
	vmovss	.LCPI32_19(%rip), %xmm8
.Ltmp6710:
	.loc	26 166 24
	vcmpnltss	%xmm8, %xmm6, %xmm6
.Ltmp6711:
	.loc	26 71 9
	vmulss	.LCPI32_15(%rip), %xmm4, %xmm4
.Ltmp6712:
	.loc	26 161 24
	vminss	.LCPI32_16(%rip), %xmm4, %xmm4
.Ltmp6713:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI32_17(%rip), %xmm4, %xmm15
.Ltmp6714:
	.loc	26 129 14 is_stmt 1
	vucomiss	72(%rsp), %xmm15
.Ltmp6715:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp6716:
	.loc	26 129 14
	vucomiss	%xmm9, %xmm15
.Ltmp6717:
	.loc	26 166 24
	vandps	%xmm7, %xmm6, %xmm7
.Ltmp6718:
	.loc	26 144 9
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp6719:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%r14), %xmm4
.Ltmp6720:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r10d
.Ltmp6721:
	.loc	26 124 14
	vucomiss	%xmm11, %xmm13
.Ltmp6722:
	.loc	26 149 9
	notl	%r10d
.Ltmp6723:
	.loc	26 139 9
	cmovbel	%esi, %r10d
.Ltmp6724:
	.loc	26 124 14
	vucomiss	%xmm11, %xmm4
.Ltmp6725:
	.loc	21 489 5
	vmovss	%xmm0, 860(%r14)
	.loc	21 498 5
	movl	%r9d, 856(%r14)
.Ltmp6726:
	.loc	21 510 5
	vmovss	%xmm7, 864(%r14)
.Ltmp6727:
	.loc	26 144 9
	cmoval	%r13d, %r12d
.Ltmp6728:
	.loc	26 139 9
	cmovbel	%esi, %r10d
.Ltmp6729:
	.loc	26 161 24
	testb	$1, %r10b
	je	.LBB32_355
.Ltmp6730:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm12, %xmm13, %xmm13
.LBB32_355:
	vmovss	68(%rsp), %xmm1
.Ltmp6731:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r12b
	jne	.LBB32_357
.Ltmp6732:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm1
.LBB32_357:
.Ltmp6733:
	vmulss	.LCPI32_20(%rip), %xmm7, %xmm4
	vmaxss	.LCPI32_21(%rip), %xmm4, %xmm4
	vminss	.LCPI32_22(%rip), %xmm4, %xmm4
	vroundss	$9, %xmm4, %xmm4, %xmm12
	vsubss	%xmm12, %xmm4, %xmm14
.Ltmp6734:
	orl	%r12d, %r10d
	andl	$1065353216, %r10d
	vmovd	%r10d, %xmm4
	vmovss	%xmm1, 112(%rsp)
.Ltmp6735:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm1, 936(%r14)
	.loc	21 498 5
	movl	%r10d, 932(%r14)
	vmovaps	%xmm9, %xmm13
.Ltmp6736:
	.loc	26 66 9
	vsubss	%xmm9, %xmm15, %xmm6
.Ltmp6737:
	.loc	26 71 9
	vmulss	64(%rsp), %xmm6, %xmm6
.Ltmp6738:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm6, %xmm15
.Ltmp6739:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm4, %xmm11, %xmm4
	vcmpltss	%xmm11, %xmm15, %xmm6
	vandps	%xmm6, %xmm4, %xmm4
	vmovd	%xmm4, %r9d
	testb	$1, %r9b
	jne	.LBB32_359
.Ltmp6740:
	.loc	26 0 44
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB32_359
.LBB32_436:
	movq	144(%rsp), %r10
	movq	%r10, %rax
	movq	136(%rsp), %rsi
	subq	%rsi, %rax
.Ltmp6741:
	.loc	15 2584 13 is_stmt 1
	addl	%eax, %edx
.Ltmp6742:
	.loc	21 413 5
	movl	%edx, 1208(%r14)
	movq	304(%rsp), %rdx
.Ltmp6743:
.LBB32_42:
	.loc	6 701 9
	subl	%esi, 1220(%r14)
.Ltmp6744:
	.loc	6 773 33
	leaq	176(%rsp), %r11
	movq	%rcx, 176(%rsp)
	movq	%r10, 184(%rsp)
	movq	(%rsp), %rax
	movq	%rax, 192(%rsp)
	movq	%rdx, 200(%rsp)
	movl	72(%r14), %eax
	movq	%rax, 8(%rsp)
	vxorps	%xmm11, %xmm11, %xmm11
	vcvtsi2sd	%rax, %xmm11, %xmm3
	movq	1200(%r14), %rbp
	movl	1216(%r14), %eax
	movl	%eax, 20(%rsp)
	leaq	312(%rsp), %r13
	xorl	%r15d, %r15d
	vbroadcastss	.LCPI32_4(%rip), %xmm4
	vmovss	.LCPI32_29(%rip), %xmm5
	leaq	248(%rsp), %r9
	vmovsd	.LCPI32_30(%rip), %xmm6
	vmovsd	.LCPI32_31(%rip), %xmm7
	vmovsd	.LCPI32_32(%rip), %xmm8
	xorl	%r12d, %r12d
	xorl	%eax, %eax
	vmovsd	%xmm3, 152(%rsp)
	vmovaps	%xmm4, 288(%rsp)
	jmp	.LBB32_43
	.loc	6 0 33 is_stmt 0
.Ltmp6745:
	.p2align	4
.LBB32_322:
	movq	(%r13), %rax
.Ltmp6746:
	.loc	15 2428 13 is_stmt 1
	addq	%r10, %rax
	movq	$-1, %rcx
	cmovbq	%rcx, %rax
.Ltmp6747:
	.loc	6 794 17
	movq	%rax, (%r13)
	leaq	248(%rsp), %r9
.Ltmp6748:
.LBB32_323:
	.loc	6 0 17 is_stmt 0
	movl	$1, %eax
	movq	%r9, %r13
.Ltmp6749:
	.loc	27 131 12 is_stmt 1
	testb	$1, %r12b
	movb	$1, %r12b
	jne	.LBB32_324
.Ltmp6750:
.LBB32_43:
	.loc	25 253 13
	movq	%rax, %rcx
	shlq	$4, %rcx
.Ltmp6751:
	.loc	1 1733 9
	movq	(%r11,%rcx), %rdx
	movq	8(%r11,%rcx), %rdi
.Ltmp6752:
	.loc	6 774 13
	imulq	$76, %rax, %rbx
	vmovss	864(%r14,%rbx), %xmm0
.Ltmp6753:
	.loc	16 2155 12
	testq	%rdi, %rdi
	je	.LBB32_306
.Ltmp6754:
	.loc	16 0 12 is_stmt 0
	movl	$-1, %esi
	xorl	%r8d, %r8d
	.p2align	4
.LBB32_45:
.Ltmp6755:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rdx,%r8,4), %xmm1
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp6756:
	.loc	26 114 14
	vucomiss	%xmm1, %xmm5
.Ltmp6757:
	.loc	26 139 9
	cmovbel	%r15d, %esi
.Ltmp6758:
	.loc	16 2155 12
	incq	%r8
	cmpq	%r8, %rdi
	jne	.LBB32_45
.Ltmp6759:
	.loc	16 0 12 is_stmt 0
	vandps	%xmm4, %xmm0, %xmm0
	vucomiss	%xmm0, %xmm5
	.loc	6 775 16 is_stmt 1
	jbe	.LBB32_48
	cmpl	$-1, %esi
	je	.LBB32_323
.LBB32_48:
	.loc	6 0 16 is_stmt 0
	movl	$-1, %esi
	xorl	%r8d, %r8d
	.p2align	4
.LBB32_49:
.Ltmp6760:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rdx,%r8,4), %xmm1
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp6761:
	.loc	26 114 14
	vucomiss	%xmm1, %xmm5
.Ltmp6762:
	.loc	26 139 9
	cmovbel	%r15d, %esi
.Ltmp6763:
	.loc	16 2155 12
	incq	%r8
	cmpq	%r8, %rdi
	jne	.LBB32_49
.Ltmp6764:
	.loc	28 185 12
	notl	%esi
	xorl	%r8d, %r8d
	testl	$1065353216, %esi
	setne	%r8b
	jmp	.LBB32_308
.Ltmp6765:
	.loc	28 0 12 is_stmt 0
.Ltmp6766:
	.p2align	4
.LBB32_306:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm4, %xmm0, %xmm0
.Ltmp6767:
	.loc	26 114 14
	vucomiss	%xmm0, %xmm5
.Ltmp6768:
	.loc	6 775 43
	ja	.LBB32_323
	.loc	6 0 43 is_stmt 0
	xorl	%r8d, %r8d
.LBB32_308:
.Ltmp6769:
	.loc	26 114 14 is_stmt 1
	xorl	%esi, %esi
	vucomiss	%xmm0, %xmm5
	setbe	%sil
	orl	%r8d, %esi
	je	.LBB32_323
.Ltmp6770:
	.loc	26 0 14 is_stmt 0
	testq	%r10, %r10
.Ltmp6771:
	.loc	11 900 12 is_stmt 1
	je	.LBB32_313
.Ltmp6772:
	.loc	11 0 12 is_stmt 0
	xorl	%esi, %esi
	.p2align	4
.LBB32_311:
.Ltmp6773:
	.loc	6 785 21 is_stmt 1
	cmpq	%rsi, %rdi
	je	.LBB32_325
	movl	$0, (%rdx,%rsi,4)
.Ltmp6774:
	.loc	15 971 17
	incq	%rsi
.Ltmp6775:
	.loc	8 1916 50
	cmpq	%rsi, %r10
.Ltmp6776:
	.loc	11 900 12
	jne	.LBB32_311
.Ltmp6777:
.LBB32_313:
	.loc	11 0 12 is_stmt 0
	movq	%rax, %rdx
	shlq	$6, %rdx
	testq	%rbp, %rbp
.Ltmp6778:
	.loc	11 900 12
	je	.LBB32_317
.Ltmp6779:
	.loc	11 0 12
	leaq	104(%r14), %rsi
	addq	%rdx, %rsi
	movq	8(%rsi), %rdi
	xorl	%r8d, %r8d
	.p2align	4
.LBB32_315:
.Ltmp6780:
	.loc	6 565 13 is_stmt 1
	cmpq	%r8, %rdi
	je	.LBB32_439
	movq	(%rsi), %r9
	movl	$0, (%r9,%r8,4)
.Ltmp6781:
	.loc	15 971 17
	incq	%r8
.Ltmp6782:
	.loc	8 1916 50
	cmpq	%r8, %rbp
.Ltmp6783:
	.loc	11 900 12
	jne	.LBB32_315
.Ltmp6784:
.LBB32_317:
	.loc	11 0 12 is_stmt 0
	movq	%rax, %rsi
	shlq	$5, %rsi
	leaq	232(%r14), %rdi
	addq	%rdi, %rsi
	leaq	944(%r14), %rdi
	addq	%rdi, %rcx
.Ltmp6785:
	.loc	6 509 22 is_stmt 1
	vmovss	(%rsi), %xmm12
	vmovss	4(%rsi), %xmm11
	vmovss	8(%rsi), %xmm10
	vmovss	12(%rsi), %xmm9
	vmovss	16(%rsi), %xmm0
	vmovss	20(%rsi), %xmm2
	vmovss	24(%rsi), %xmm13
	vmovss	28(%rsi), %xmm1
.Ltmp6786:
	.loc	6 510 9
	vmovss	%xmm1, (%rcx)
	vmovss	%xmm0, 4(%rcx)
	vmovss	%xmm2, 8(%rcx)
	vmovss	%xmm13, 12(%rcx)
.Ltmp6787:
	.loc	9 82 17
	vcvtss2sd	%xmm1, %xmm1, %xmm1
.Ltmp6788:
	.loc	6 406 20
	vmulsd	%xmm1, %xmm3, %xmm1
	vdivsd	%xmm6, %xmm1, %xmm1
	.loc	6 406 19 is_stmt 0
	vaddsd	%xmm7, %xmm1, %xmm1
.Ltmp6789:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm1, %xmm1, %xmm1
	vmovq	%xmm1, %rcx
.Ltmp6790:
	.loc	6 407 9
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
	jne	.LBB32_322
	vucomisd	%xmm8, %xmm1
	ja	.LBB32_322
.Ltmp6791:
	.loc	9 82 17
	vcvtss2sd	%xmm2, %xmm2, %xmm2
.Ltmp6792:
	.loc	6 406 20
	vmulsd	%xmm2, %xmm3, %xmm2
	vdivsd	%xmm6, %xmm2, %xmm2
	.loc	6 406 19 is_stmt 0
	vaddsd	%xmm7, %xmm2, %xmm2
.Ltmp6793:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm2, %xmm2, %xmm2
	vmovq	%xmm2, %rcx
.Ltmp6794:
	.loc	6 407 9
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
	jne	.LBB32_322
	vucomisd	%xmm8, %xmm2
	ja	.LBB32_322
.Ltmp6795:
	.loc	6 0 9 is_stmt 0
	addq	%r14, %rdx
	leaq	792(%r14), %rcx
	addq	%rcx, %rbx
	leaq	(%rax,%rax,2), %rax
	leaq	744(%r14), %rcx
	leaq	(%rcx,%rax,8), %rdi
	movq	%rdi, (%rsp)
	vxorps	%xmm5, %xmm5, %xmm5
.Ltmp6796:
	.loc	6 410 10 is_stmt 1
	vmaxsd	%xmm1, %xmm5, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rax
.Ltmp6797:
	.loc	6 410 10 is_stmt 0
	vmaxsd	%xmm2, %xmm5, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rcx
	movl	20(%rsp), %esi
.Ltmp6798:
	.loc	6 536 9 is_stmt 1
	subl	%eax, %esi
	cmovbl	%r15d, %esi
	movl	%esi, 136(%rdx)
	.loc	6 537 62
	movl	%ecx, %eax
	vcvtsi2ss	%rax, %xmm15, %xmm1
	vmovss	%xmm1, 32(%rsp)
.Ltmp6799:
	.loc	6 401 5
	vmovss	%xmm1, 8(%rdi)
	movq	8(%rsp), %rdi
	vmovss	%xmm9, 112(%rsp)
	vmovss	%xmm10, 96(%rsp)
	vmovss	%xmm11, 24(%rsp)
	vmovss	%xmm12, 224(%rsp)
	vmovss	%xmm13, 16(%rsp)
.Ltmp6800:
	.loc	6 541 13
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movq	(%rsp), %rax
.Ltmp6801:
	.loc	6 401 5
	vmovss	%xmm0, (%rax)
	vmovss	16(%rsp), %xmm0
	movq	8(%rsp), %rdi
.Ltmp6802:
	.loc	6 546 13
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	vmovsd	.LCPI32_32(%rip), %xmm8
	vmovsd	.LCPI32_31(%rip), %xmm7
	vmovsd	.LCPI32_30(%rip), %xmm6
	vmovss	.LCPI32_29(%rip), %xmm5
	vmovaps	288(%rsp), %xmm4
	vmovsd	152(%rsp), %xmm3
	leaq	176(%rsp), %r11
	movq	144(%rsp), %r10
	movq	(%rsp), %rax
.Ltmp6803:
	.loc	6 401 5
	vmovss	%xmm0, 4(%rax)
.Ltmp6804:
	.loc	6 401 5 is_stmt 0
	movl	$0, 72(%rbx)
.Ltmp6805:
	.loc	6 401 5
	movl	$1065353216, 64(%rbx)
	vmovss	32(%rsp), %xmm0
.Ltmp6806:
	.loc	6 401 5
	vmovss	%xmm0, 68(%rbx)
	vmovss	224(%rsp), %xmm0
.Ltmp6807:
	.loc	6 401 5
	vmovss	%xmm0, (%rbx)
.Ltmp6808:
	.loc	6 401 5
	vmovss	%xmm0, 4(%rbx)
.Ltmp6809:
	.loc	6 401 5
	movq	$0, 8(%rbx)
	vmovss	24(%rsp), %xmm0
.Ltmp6810:
	.loc	6 401 5
	vmovss	%xmm0, 16(%rbx)
.Ltmp6811:
	.loc	6 401 5
	vmovss	%xmm0, 20(%rbx)
.Ltmp6812:
	.loc	6 401 5
	movq	$0, 24(%rbx)
	vmovss	96(%rsp), %xmm0
.Ltmp6813:
	.loc	6 401 5
	vmovss	%xmm0, 32(%rbx)
.Ltmp6814:
	.loc	6 401 5
	vmovss	%xmm0, 36(%rbx)
.Ltmp6815:
	.loc	6 401 5
	movq	$0, 40(%rbx)
	vmovss	112(%rsp), %xmm0
.Ltmp6816:
	.loc	6 401 5
	vmovss	%xmm0, 48(%rbx)
.Ltmp6817:
	.loc	6 401 5
	vmovss	%xmm0, 52(%rbx)
.Ltmp6818:
	.loc	6 401 5
	movq	$0, 56(%rbx)
	jmp	.LBB32_322
.Ltmp6819:
.LBB32_324:
	.loc	6 1009 9 is_stmt 1
	vmovaps	384(%rsp), %xmm0
	movq	352(%rsp), %rax
	vmovups	%xmm0, (%rax)
	movq	240(%rsp), %rcx
	movq	%rcx, 16(%rax)
	movq	312(%rsp), %rcx
	movq	%rcx, 24(%rax)
	movq	248(%rsp), %rcx
	movq	%rcx, 32(%rax)
.Ltmp6820:
	.loc	6 1010 6 epilogue_begin
	addq	$408, %rsp
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
.LBB32_296:
	.cfi_def_cfa_offset 464
	.loc	6 0 6 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp6821:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_1eeef3195352adc58f4bfdb015316fef(%rip), %rcx
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6822:
.LBB32_298:
	.loc	25 0 13 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp6823:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7(%rip), %rcx
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6824:
.LBB32_330:
	.loc	25 0 13 is_stmt 0
	leaq	1(%r9), %rsi
.Ltmp6825:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_1eeef3195352adc58f4bfdb015316fef(%rip), %rcx
.Ltmp6826:
	.loc	21 0 0 is_stmt 0
	movq	%r9, %rdi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_401:
	leaq	1(%r9), %rsi
.Ltmp6827:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7(%rip), %rcx
.Ltmp6828:
	.loc	21 0 0 is_stmt 0
	movq	%r9, %rdi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_398:
	leaq	1(%r9), %rsi
.Ltmp6829:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_b9d56679ca30f2caa500ddf2e89d00cb(%rip), %rcx
.Ltmp6830:
	.loc	21 0 0 is_stmt 0
	movq	%r9, %rdi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_403:
	leaq	1(%r9), %rsi
.Ltmp6831:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_f198228fd40f4c04a48a745576c5c5ee(%rip), %rcx
.Ltmp6832:
	.loc	21 0 0 is_stmt 0
	movq	%r9, %rdi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6833:
.LBB32_297:
	leaq	1(%rdi), %rsi
.Ltmp6834:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_b9d56679ca30f2caa500ddf2e89d00cb(%rip), %rcx
	movq	%r9, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6835:
.LBB32_299:
	.loc	25 0 13 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp6836:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_f198228fd40f4c04a48a745576c5c5ee(%rip), %rcx
	movq	%r9, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6837:
.LBB32_440:
	.loc	25 443 13
	leaq	.Lalloc_d9529ff5ddc99dd60299cff5ff3cd676(%rip), %rcx
	movq	344(%rsp), %rdi
	movq	%r8, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6838:
.LBB32_438:
	.loc	25 456 13
	leaq	.Lalloc_1d7cc6e40c752396aa7def7556a6c433(%rip), %rcx
	xorl	%edi, %edi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6839:
.LBB32_437:
	.loc	25 581 13
	leaq	.Lalloc_a6d4388bd1c2ee005f6a969a0e3ca0f4(%rip), %rcx
	movq	%rsi, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp6840:
.LBB32_325:
	.loc	6 785 21
	leaq	.Lalloc_6797264598a169e4722ae66c7bc497b8(%rip), %rdx
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6841:
.LBB32_439:
	.loc	6 565 13
	leaq	.Lalloc_835aafef72e8508601474e7b1f4172a9(%rip), %rdx
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6842:
.LBB32_304:
	.loc	21 268 33
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp6843:
	.loc	21 0 0 is_stmt 0
	movq	%r8, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_201:
.Ltmp6844:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6845:
.LBB32_112:
	.loc	21 251 32
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6846:
.LBB32_300:
	.loc	21 237 33
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
.Ltmp6847:
	.loc	21 0 0 is_stmt 0
	movq	%r8, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_302:
.Ltmp6848:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp6849:
	.loc	21 0 0 is_stmt 0
	movq	%r8, %rdi
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_199:
.Ltmp6850:
	.loc	21 236 32 is_stmt 1
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6851:
.LBB32_339:
	.loc	21 251 32
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
.Ltmp6852:
	.loc	21 0 0 is_stmt 0
	movq	%r10, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_408:
.Ltmp6853:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
.Ltmp6854:
	.loc	21 0 0 is_stmt 0
	movq	%r9, %rdi
	movq	%r11, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_413:
.Ltmp6855:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp6856:
	.loc	21 0 0 is_stmt 0
	movq	%r9, %rdi
	movq	%r11, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_419:
.Ltmp6857:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp6858:
	.loc	21 0 0 is_stmt 0
	movq	%r9, %rdi
	movq	%r11, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_416:
.Ltmp6859:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
.Ltmp6860:
	.loc	21 0 0 is_stmt 0
	movq	%r10, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_363:
.Ltmp6861:
	.loc	21 236 32 is_stmt 1
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
.Ltmp6862:
	.loc	21 0 0 is_stmt 0
	movq	%r10, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_442:
.Ltmp6863:
	.loc	21 269 33 is_stmt 1
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%r9, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_441:
	.loc	21 267 33
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	%r10, %rdi
	movq	%r11, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6864:
.LBB32_305:
	.loc	21 269 33
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%r8, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_303:
	.loc	21 267 33
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	%r9, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6865:
.Lfunc_end32:
	.size	_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb0_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_, .Lfunc_end32-_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb0_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_
