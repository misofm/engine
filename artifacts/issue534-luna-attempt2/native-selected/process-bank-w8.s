_RNvXse_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank:
.Lfunc_begin46:
	.loc	6 1077 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
.Ltmp8283:
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	andq	$-32, %rsp
	subq	$1440, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.loc	6 1078 29 prologue_end
	movzbl	2624(%rsi), %eax
.Ltmp8284:
	.loc	18 966 15
	cmpb	$2, %al
	.loc	18 966 9 is_stmt 0
	je	.LBB46_287
.Ltmp8285:
	.loc	20 186 45 is_stmt 1
	cmpb	%al, 108(%rdx)
.Ltmp8286:
	.loc	6 1083 20
	jne	.LBB46_60
	cmpq	$0, 64(%rdx)
	jne	.LBB46_60
	.loc	6 0 20 is_stmt 0
	movq	%rsi, %r8
	movb	%al, 119(%rsp)
	movq	%rdi, 1072(%rsp)
	.loc	6 1088 35 is_stmt 1
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqa	%ymm0, 1376(%rsp)
	vmovdqa	%ymm0, 1344(%rsp)
	vmovdqa	%ymm0, 1312(%rsp)
	vmovdqa	%ymm0, 1280(%rsp)
	vmovdqa	%ymm0, 1248(%rsp)
	vmovdqa	%ymm0, 1216(%rsp)
	vmovdqa	%ymm0, 1184(%rsp)
	vmovdqa	%ymm0, 1152(%rsp)
	vmovdqa	%ymm0, 1120(%rsp)
	vmovdqa	%ymm0, 1088(%rsp)
	movq	48(%rdx), %r11
	movq	56(%rdx), %r9
	movq	32(%rdx), %rax
	movq	%rax, 384(%rsp)
	movq	40(%rdx), %rax
	movq	%rax, 96(%rsp)
	movq	%rdx, 128(%rsp)
	movq	96(%rdx), %rax
	movq	%rax, 88(%rsp)
	leaq	1280(%rsi), %rax
	movq	%rax, 760(%rsp)
.Ltmp8287:
	.loc	11 900 12
	cmpq	$1, %r9
	movq	%r9, %rax
	adcq	$-1, %rax
	movq	%rax, 104(%rsp)
	xorl	%ebx, %ebx
	vmovss	.LCPI46_0(%rip), %xmm5
	vmovss	.LCPI46_1(%rip), %xmm6
	movq	%rsi, 168(%rsp)
	movq	%r9, 144(%rsp)
	movq	%r11, 192(%rsp)
	jmp	.LBB46_5
	.loc	11 0 12 is_stmt 0
.Ltmp8288:
	.p2align	4
.LBB46_4:
	movq	16(%rsp), %rax
	movq	%rax, %rbx
.Ltmp8289:
	.loc	8 1916 50 is_stmt 1
	cmpq	$8, %rax
.Ltmp8290:
	.loc	11 900 12
	je	.LBB46_61
.Ltmp8291:
.LBB46_5:
	.loc	6 1090 33
	cmpq	%r9, %rbx
	je	.LBB46_292
.Ltmp8292:
	.loc	6 0 0 is_stmt 0
	leaq	1(%rbx), %rax
.Ltmp8293:
	.loc	6 1091 31 is_stmt 1
	cmpq	104(%rsp), %rbx
	je	.LBB46_293
	.loc	6 0 31 is_stmt 0
	movl	(%r11,%rbx,4), %edi
	.loc	6 1091 31
	movl	(%r11,%rax,4), %esi
.Ltmp8294:
	.loc	15 1050 16 is_stmt 1
	cmpl	%edi, %esi
	jb	.LBB46_273
	cmpq	%rsi, 96(%rsp)
	jb	.LBB46_273
.Ltmp8295:
	.loc	15 0 16 is_stmt 0
	movq	%rax, 16(%rsp)
	movl	$0, 608(%rsp)
	movl	$0, 616(%rsp)
	movl	$0, 624(%rsp)
	movl	$0, 632(%rsp)
	movl	$0, 640(%rsp)
	movl	$0, 648(%rsp)
	movl	$0, 656(%rsp)
	movl	$0, 664(%rsp)
.Ltmp8296:
	.loc	38 1714 9 is_stmt 1
	cmpl	%edi, %esi
.Ltmp8297:
	.loc	19 180 28
	jne	.LBB46_44
.Ltmp8298:
.LBB46_10:
	.loc	19 0 28 is_stmt 0
	movq	760(%rsp), %rax
	xorl	%ecx, %ecx
.Ltmp8299:
	.loc	19 180 28
	jmp	.LBB46_13
.Ltmp8300:
	.loc	19 0 28
.Ltmp8301:
	.p2align	4
.LBB46_11:
	.loc	6 399 5 is_stmt 1
	vmovaps	384(%rax), %ymm4
	vmovaps	%ymm4, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, 384(%rax)
.Ltmp8302:
	.loc	6 399 5
	vmovaps	416(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm0, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 416(%rax)
.Ltmp8303:
	.loc	6 399 5
	vmovaps	448(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm2, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 448(%rax)
.Ltmp8304:
	.loc	6 399 5
	vmovaps	480(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm3, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovdqa	32(%rsp), %ymm0
	vmovdqa	%ymm0, 480(%rax)
.Ltmp8305:
	.loc	6 664 17
	movl	$64, 2620(%r8)
.Ltmp8306:
.LBB46_12:
	.loc	6 0 0 is_stmt 0
	addq	$32, %rcx
.Ltmp8307:
	.loc	38 1714 9 is_stmt 1
	addq	$608, %rax
	cmpq	$64, %rcx
.Ltmp8308:
	.loc	19 180 28
	je	.LBB46_4
.Ltmp8309:
.LBB46_13:
	.loc	6 651 21
	cmpl	$1, 608(%rsp,%rcx)
	jne	.LBB46_14
	.loc	6 651 26 is_stmt 0
	vmovd	612(%rsp,%rcx), %xmm0
.Ltmp8310:
	.loc	6 654 39 is_stmt 1
	vmovaps	(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
.Ltmp8311:
	.loc	6 393 5
	vmovd	32(%rsp,%rbx,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp8312:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp8313:
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
.Ltmp8314:
.LBB46_20:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB46_21:
.Ltmp8315:
	.loc	6 399 5 is_stmt 1
	vmovaps	(%rax), %ymm4
	vmovaps	%ymm4, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, (%rax)
.Ltmp8316:
	.loc	6 399 5
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm0, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 32(%rax)
.Ltmp8317:
	.loc	6 399 5
	vmovaps	64(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm2, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 64(%rax)
.Ltmp8318:
	.loc	6 399 5
	vmovaps	96(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm3, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovdqa	32(%rsp), %ymm0
	vmovdqa	%ymm0, 96(%rax)
.Ltmp8319:
	.loc	6 664 17
	movl	$64, 2620(%r8)
.Ltmp8320:
	.loc	6 651 21
	cmpl	$1, 616(%rsp,%rcx)
	je	.LBB46_22
.LBB46_15:
	cmpl	$1, 624(%rsp,%rcx)
	jne	.LBB46_16
.LBB46_27:
	.loc	6 651 26 is_stmt 0
	vmovd	628(%rsp,%rcx), %xmm0
.Ltmp8321:
	.loc	6 654 39 is_stmt 1
	vmovaps	256(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
.Ltmp8322:
	.loc	6 393 5
	vmovd	32(%rsp,%rbx,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp8323:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp8324:
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
.Ltmp8325:
.LBB46_30:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB46_31:
.Ltmp8326:
	.loc	6 399 5 is_stmt 1
	vmovaps	256(%rax), %ymm4
	vmovaps	%ymm4, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, 256(%rax)
.Ltmp8327:
	.loc	6 399 5
	vmovaps	288(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm0, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 288(%rax)
.Ltmp8328:
	.loc	6 399 5
	vmovaps	320(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm2, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 320(%rax)
.Ltmp8329:
	.loc	6 399 5
	vmovaps	352(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm3, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovdqa	32(%rsp), %ymm0
	vmovdqa	%ymm0, 352(%rax)
.Ltmp8330:
	.loc	6 664 17
	movl	$64, 2620(%r8)
.Ltmp8331:
	.loc	6 651 21
	cmpl	$1, 632(%rsp,%rcx)
	jne	.LBB46_12
	jmp	.LBB46_32
	.loc	6 0 21 is_stmt 0
.Ltmp8332:
	.p2align	4
.LBB46_14:
	.loc	6 651 21
	cmpl	$1, 616(%rsp,%rcx)
	jne	.LBB46_15
.LBB46_22:
	.loc	6 651 26
	vmovd	620(%rsp,%rcx), %xmm0
.Ltmp8333:
	.loc	6 654 39 is_stmt 1
	vmovaps	128(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
.Ltmp8334:
	.loc	6 393 5
	vmovd	32(%rsp,%rbx,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp8335:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp8336:
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
.Ltmp8337:
.LBB46_25:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB46_26:
.Ltmp8338:
	.loc	6 399 5 is_stmt 1
	vmovaps	128(%rax), %ymm4
	vmovaps	%ymm4, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, 128(%rax)
.Ltmp8339:
	.loc	6 399 5
	vmovaps	160(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm0, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 160(%rax)
.Ltmp8340:
	.loc	6 399 5
	vmovaps	192(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm2, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 192(%rax)
.Ltmp8341:
	.loc	6 399 5
	vmovaps	224(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm3, 32(%rsp,%rbx,4)
	.loc	6 401 5
	vmovdqa	32(%rsp), %ymm0
	vmovdqa	%ymm0, 224(%rax)
.Ltmp8342:
	.loc	6 664 17
	movl	$64, 2620(%r8)
.Ltmp8343:
	.loc	6 651 21
	cmpl	$1, 624(%rsp,%rcx)
	je	.LBB46_27
.LBB46_16:
	cmpl	$1, 632(%rsp,%rcx)
	jne	.LBB46_12
.LBB46_32:
	.loc	6 651 26 is_stmt 0
	vmovd	636(%rsp,%rcx), %xmm0
.Ltmp8344:
	.loc	6 654 39 is_stmt 1
	vmovaps	384(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
.Ltmp8345:
	.loc	6 393 5
	vmovd	32(%rsp,%rbx,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp8346:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp8347:
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
.Ltmp8348:
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
.Ltmp8349:
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
.Ltmp8350:
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
.Ltmp8351:
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
.Ltmp8352:
.LBB46_43:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
	jmp	.LBB46_11
	.p2align	4
.LBB46_44:
	subq	%rdi, %rsi
	leaq	(%rdi,%rdi,4), %rax
	movq	384(%rsp), %rcx
	leaq	(%rcx,%rax,8), %r14
	leaq	(%rsi,%rsi,4), %rax
	leaq	(%r14,%rax,8), %rdx
	.loc	6 1096 25 is_stmt 1
	leaq	(%rbx,%rbx,4), %rax
	leaq	1088(%rsp,%rax,8), %rsi
	movl	2596(%r8), %edi
	movq	1104(%rsp,%rax,8), %r13
	movb	$1, %al
	movl	%eax, 120(%rsp)
	xorl	%r15d, %r15d
	jmp	.LBB46_45
	.loc	6 0 25 is_stmt 0
.Ltmp8353:
	.p2align	4
.LBB46_58:
.Ltmp8354:
	addq	$40, %r14
.Ltmp8355:
	.loc	15 2428 13 is_stmt 1
	incq	%r13
	movq	$-1, %rax
	cmoveq	%rax, %r13
.Ltmp8356:
	.loc	6 0 0 is_stmt 0
	movq	%r13, 16(%rsi)
.Ltmp8357:
	.loc	34 82 9 is_stmt 1
	incq	%r15
.Ltmp8358:
	.loc	38 1714 9
	cmpq	%rdx, %r14
.Ltmp8359:
	.loc	19 180 28
	je	.LBB46_10
.Ltmp8360:
.LBB46_45:
	.loc	6 616 33
	movl	32(%r14), %eax
	.loc	6 616 27 is_stmt 0
	cmpl	$1, %eax
	je	.LBB46_48
	cmpl	$2, %eax
	jne	.LBB46_58
	.loc	6 0 27
	movl	$1, %eax
	leaq	640(%rsp), %rcx
	movq	%rcx, 24(%rsp)
.Ltmp8361:
	.loc	6 624 35 is_stmt 1
	movl	16(%r14), %r12d
.Ltmp8362:
	.loc	15 3178 26
	testl	%r12d, %r12d
.Ltmp8363:
	.loc	46 459 8
	jns	.LBB46_49
	jmp	.LBB46_58
.Ltmp8364:
	.loc	46 0 8 is_stmt 0
.Ltmp8365:
	.p2align	4
.LBB46_48:
	xorl	%eax, %eax
	leaq	608(%rsp), %rcx
	movq	%rcx, 24(%rsp)
	.loc	6 624 35 is_stmt 1
	movl	16(%r14), %r12d
.Ltmp8366:
	.loc	15 3178 26
	testl	%r12d, %r12d
.Ltmp8367:
	.loc	46 459 8
	js	.LBB46_58
.Ltmp8368:
.LBB46_49:
	.loc	6 633 25
	cmpq	%rdi, %r15
	jae	.LBB46_58
	cmpl	$3, %r12d
	ja	.LBB46_58
	.loc	6 635 20
	cmpl	$1, 28(%r14)
	jne	.LBB46_58
	.loc	6 0 20 is_stmt 0
	movq	88(%rsp), %rcx
	.loc	6 636 20 is_stmt 1
	cmpq	%rcx, (%r14)
	jne	.LBB46_58
	.loc	6 0 20 is_stmt 0
	movq	88(%rsp), %rcx
	.loc	6 637 20 is_stmt 1
	cmpq	%rcx, 8(%r14)
	jne	.LBB46_58
	.loc	6 638 20
	vmovd	20(%r14), %xmm0
.Ltmp8369:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp8370:
	.loc	6 638 20
	cmpl	%ecx, 24(%r14)
	jne	.LBB46_58
.Ltmp8371:
	.loc	6 0 0 is_stmt 0
	leal	(%rax,%r12,2), %eax
	movl	%eax, 248(%rsp)
.Ltmp8372:
	.loc	6 639 42 is_stmt 1
	leaq	(%r12,%r12,4), %rax
	leaq	.Lalloc_d0058eaee52f1ba8b2e5577cef0c3c7f(%rip), %rcx
	movq	%rdi, 832(%rsp)
	leaq	(%rcx,%rax,8), %rdi
	movq	%rdx, 216(%rsp)
	movq	%rsi, 864(%rsp)
	vmovdqa	%xmm0, 544(%rsp)
	.loc	6 639 20 is_stmt 0
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	544(%rsp), %xmm1
	movq	832(%rsp), %rdi
	movq	864(%rsp), %rsi
	movq	216(%rsp), %rdx
	vmovss	.LCPI46_1(%rip), %xmm6
	vmovss	.LCPI46_0(%rip), %xmm5
	movq	192(%rsp), %r11
	movq	144(%rsp), %r9
	movq	168(%rsp), %r8
	movl	248(%rsp), %ecx
	movl	%ecx, %r10d
	cmpl	176(%rsp), %ecx
	seta	%cl
	testb	%al, %al
	je	.LBB46_58
	orb	120(%rsp), %cl
	testb	$1, %cl
	je	.LBB46_58
	.loc	6 0 20
	movq	24(%rsp), %rax
.Ltmp8373:
	.loc	6 642 17 is_stmt 1
	cmpb	$0, (%rax,%r12,8)
	jne	.LBB46_58
.Ltmp8374:
	.loc	12 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
	movq	24(%rsp), %rax
.Ltmp8375:
	.loc	6 647 13
	movl	$1, (%rax,%r12,8)
	vmovss	%xmm0, 4(%rax,%r12,8)
.Ltmp8376:
	.loc	38 1714 9
	addq	$40, %r14
.Ltmp8377:
	.loc	19 180 28
	incq	%r15
	movl	$0, 120(%rsp)
	movl	%r10d, 176(%rsp)
.Ltmp8378:
	.loc	38 1714 9
	cmpq	%rdx, %r14
.Ltmp8379:
	.loc	19 180 28
	jne	.LBB46_45
	jmp	.LBB46_10
.Ltmp8380:
.LBB46_60:
	.loc	6 1084 28
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
	jmp	.LBB46_272
.LBB46_61:
	.loc	6 0 28 is_stmt 0
	movq	128(%rsp), %rcx
.Ltmp8381:
	.loc	6 1099 30 is_stmt 1
	movl	104(%rcx), %r9d
.Ltmp8382:
	.loc	6 1100 32
	movq	(%rcx), %rax
	movq	%rax, 488(%rsp)
	movq	8(%rcx), %rax
	movq	%rax, 264(%rsp)
	.loc	6 1100 44 is_stmt 0
	movq	16(%rcx), %rax
	movq	%rax, 480(%rsp)
	movq	24(%rcx), %rax
	movq	%rax, 256(%rsp)
.Ltmp8383:
	.loc	6 685 25 is_stmt 1
	movl	2620(%r8), %edx
.Ltmp8384:
	.loc	8 1078 5
	cmpl	%edx, %r9d
	movl	%edx, %ecx
	cmovbl	%r9d, %ecx
.Ltmp8385:
	.loc	6 686 12
	testl	%ecx, %ecx
	movq	%r9, 248(%rsp)
	movq	%rcx, 752(%rsp)
	je	.LBB46_152
	.loc	6 687 25
	leaq	(,%rcx,8), %rsi
	cmpq	264(%rsp), %rsi
.Ltmp8386:
	.loc	15 1050 16
	ja	.LBB46_288
.Ltmp8387:
	.loc	25 451 16
	cmpq	256(%rsp), %rsi
	ja	.LBB46_289
.Ltmp8388:
	.loc	21 61 8
	cmpl	$1, 2572(%r8)
	movl	%edx, 500(%rsp)
	movq	%rsi, 1064(%rsp)
	jne	.LBB46_66
	.loc	21 0 8 is_stmt 0
	xorl	%r14d, %r14d
	.loc	21 61 8
	jmp	.LBB46_74
.LBB46_66:
.Ltmp8389:
	.loc	48 2494 21 is_stmt 1
	movl	1184(%r8), %eax
	movb	$2, %r14b
.Ltmp8390:
	.loc	8 1878 54
	cmpl	1248(%r8), %eax
.Ltmp8391:
	.loc	48 2494 21
	jne	.LBB46_74
	movl	1188(%r8), %eax
.Ltmp8392:
	.loc	8 1878 54
	cmpl	1252(%r8), %eax
.Ltmp8393:
	.loc	48 2494 21
	jne	.LBB46_74
	movl	1192(%r8), %eax
.Ltmp8394:
	.loc	8 1878 54
	cmpl	1256(%r8), %eax
.Ltmp8395:
	.loc	48 2494 21
	jne	.LBB46_74
	movl	1196(%r8), %eax
.Ltmp8396:
	.loc	8 1878 54
	cmpl	1260(%r8), %eax
.Ltmp8397:
	.loc	48 2494 21
	jne	.LBB46_74
	movl	1200(%r8), %eax
.Ltmp8398:
	.loc	8 1878 54
	cmpl	1264(%r8), %eax
.Ltmp8399:
	.loc	48 2494 21
	jne	.LBB46_74
	movl	1204(%r8), %eax
.Ltmp8400:
	.loc	8 1878 54
	cmpl	1268(%r8), %eax
.Ltmp8401:
	.loc	48 2494 21
	jne	.LBB46_74
	movl	1208(%r8), %eax
.Ltmp8402:
	.loc	8 1878 54
	cmpl	1272(%r8), %eax
.Ltmp8403:
	.loc	48 2494 21
	jne	.LBB46_74
	movl	1212(%r8), %eax
.Ltmp8404:
	.loc	8 1878 54
	cmpl	1276(%r8), %eax
	sete	%al
	movb	$2, %r14b
.Ltmp8405:
	.loc	48 2494 21
	subb	%al, %r14b
.Ltmp8406:
.LBB46_74:
	.loc	6 736 31
	movq	1152(%r8), %rax
	movq	%rax, 96(%rsp)
	movq	1160(%r8), %rdx
	.loc	6 741 31
	movq	1216(%r8), %r11
	movq	1224(%r8), %rbx
	.loc	6 747 28
	movl	2612(%r8), %r15d
.Ltmp8407:
	.loc	21 353 16
	movl	2608(%r8), %r13d
.Ltmp8408:
	.loc	11 900 12
	movl	%r13d, %eax
	subl	2616(%r8), %eax
	movq	%rax, 1080(%rsp)
	xorl	%edi, %edi
	movzbl	%r14b, %eax
	movl	%eax, 504(%rsp)
	xorl	%r12d, %r12d
	movq	%rdx, 16(%rsp)
	movq	%rbx, 104(%rsp)
	movl	%r14d, 508(%rsp)
	movq	%r13, 120(%rsp)
	movq	%r11, 472(%rsp)
	movl	%r15d, 24(%rsp)
	jmp	.LBB46_77
.Ltmp8409:
	.loc	11 0 12 is_stmt 0
.Ltmp8410:
	.p2align	4
.LBB46_75:
	movq	%r15, %rbx
	movq	96(%rsp), %r10
	movq	192(%rsp), %rdx
.Ltmp8411:
	vmovss	(%r10,%rdx,4), %xmm0
	vmovaps	%xmm0, 416(%rsp)
	movq	472(%rsp), %r15
	vmovss	(%r15,%rdx,4), %xmm0
	vmovaps	%xmm0, 992(%rsp)
	movq	144(%rsp), %rdx
	vmovss	(%r15,%rdx,4), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	(%r10,%rdx,4), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovss	(%r10,%r11,4), %xmm0
	vmovaps	%xmm0, 720(%rsp)
	vmovss	(%r15,%r11,4), %xmm0
	vmovaps	%xmm0, 960(%rsp)
	movq	512(%rsp), %rdx
	vmovss	(%r15,%rdx,4), %xmm0
	vmovaps	%xmm0, 352(%rsp)
	vmovss	(%r10,%rdx,4), %xmm0
	vmovaps	%xmm0, 192(%rsp)
	movq	176(%rsp), %rdx
	vmovss	(%r10,%rdx,4), %xmm0
	vmovaps	%xmm0, 336(%rsp)
	vmovss	(%r15,%rdx,4), %xmm0
	vmovaps	%xmm0, 928(%rsp)
	vmovss	(%r15,%rbx,4), %xmm0
	vmovaps	%xmm0, 320(%rsp)
	vmovss	(%r10,%rbx,4), %xmm0
	vmovaps	%xmm0, 176(%rsp)
	movq	128(%rsp), %rdx
	vmovss	(%r10,%rdx,4), %xmm0
	vmovaps	%xmm0, 704(%rsp)
	vmovss	(%r15,%rdx,4), %xmm0
	vmovaps	%xmm0, 896(%rsp)
	vmovss	(%r15,%r9,4), %xmm0
	vmovaps	%xmm0, 288(%rsp)
	vmovss	(%r10,%r9,4), %xmm0
	vmovaps	%xmm0, 128(%rsp)
	vmovss	(%r10,%r14,4), %xmm0
	vmovaps	%xmm0, 688(%rsp)
	vmovss	(%r15,%r14,4), %xmm0
	vmovaps	%xmm0, 768(%rsp)
	vmovss	(%r15,%rsi,4), %xmm0
	vmovaps	%xmm0, 272(%rsp)
	vmovss	(%r10,%rsi,4), %xmm0
	vmovaps	%xmm0, 512(%rsp)
	vmovss	(%r10,%rcx,4), %xmm0
	vmovaps	%xmm0, 672(%rsp)
	vmovss	(%r15,%rcx,4), %xmm9
	movq	%r15, %r11
	vmovss	(%r15,%rdi,4), %xmm0
	vmovaps	%xmm0, 448(%rsp)
	vmovss	(%r10,%rdi,4), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	(%r10,%r13,4), %xmm0
	vmovaps	%xmm0, 1040(%rsp)
	vmovss	(%r15,%r13,4), %xmm1
	vmovss	(%r15,%r8,4), %xmm0
	vmovaps	%xmm0, 432(%rsp)
	vmovss	(%r10,%r8,4), %xmm0
	vmovaps	%xmm0, 800(%rsp)
.Ltmp8412:
	vmovss	(%r10,%r12,4), %xmm4
	vmovss	(%r15,%r12,4), %xmm3
	.loc	21 268 33 is_stmt 1
	vmovss	(%r15,%rax,4), %xmm0
	vmovaps	%xmm0, 736(%rsp)
	.loc	21 269 33
	vmovss	(%r10,%rax,4), %xmm0
	vmovaps	%xmm0, 592(%rsp)
.Ltmp8413:
.LBB46_76:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	movq	104(%rsp), %rbx
	movq	168(%rsp), %r8
.Ltmp8414:
	.loc	21 441 27 is_stmt 1
	vmovaps	1280(%r8), %ymm5
.Ltmp8415:
	.loc	21 439 26
	vmovaps	1376(%r8), %ymm6
.Ltmp8416:
	.file	58 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs"
	.loc	58 48 14
	vaddps	1344(%r8), %ymm5, %ymm8
	vbroadcastss	.LCPI46_2(%rip), %ymm7
.Ltmp8417:
	.loc	58 871 14
	vcmpeqps	%ymm7, %ymm6, %ymm12
.Ltmp8418:
	.loc	58 585 19
	vblendvps	%ymm12, 1312(%r8), %ymm8, %ymm8
.Ltmp8419:
	.loc	21 441 27
	vmovaps	1408(%r8), %ymm10
.Ltmp8420:
	.loc	21 439 26
	vmovaps	1504(%r8), %ymm2
	vxorps	%xmm14, %xmm14, %xmm14
.Ltmp8421:
	.loc	58 871 14
	vcmpgt_oqps	%ymm14, %ymm6, %ymm11
.Ltmp8422:
	.loc	21 442 13
	vmaskmovps	%ymm8, %ymm11, 1280(%r8)
	.loc	21 447 13
	vmaskmovps	%ymm14, %ymm12, 1344(%r8)
	vbroadcastss	.LCPI46_3(%rip), %ymm13
.Ltmp8423:
	.loc	58 347 14
	vaddps	%ymm6, %ymm13, %ymm6
.Ltmp8424:
	.loc	21 448 13
	vmaskmovps	%ymm6, %ymm11, 1376(%r8)
.Ltmp8425:
	.loc	58 871 14
	vcmpeqps	%ymm7, %ymm2, %ymm6
.Ltmp8426:
	.loc	58 48 14
	vaddps	1472(%r8), %ymm10, %ymm12
.Ltmp8427:
	.loc	58 585 19
	vblendvps	%ymm6, 1440(%r8), %ymm12, %ymm12
.Ltmp8428:
	.loc	58 871 14
	vcmpgt_oqps	%ymm14, %ymm2, %ymm15
.Ltmp8429:
	.loc	21 442 13
	vmaskmovps	%ymm12, %ymm15, 1408(%r8)
	.loc	21 447 13
	vmaskmovps	%ymm14, %ymm6, 1472(%r8)
.Ltmp8430:
	.loc	58 347 14
	vaddps	%ymm2, %ymm13, %ymm2
.Ltmp8431:
	.loc	21 448 13
	vmaskmovps	%ymm2, %ymm15, 1504(%r8)
.Ltmp8432:
	.loc	58 585 19
	vblendvps	%ymm11, %ymm8, %ymm5, %ymm5
.Ltmp8433:
	.loc	21 439 26
	vmovaps	1632(%r8), %ymm2
.Ltmp8434:
	.loc	58 871 14
	vcmpeqps	%ymm7, %ymm2, %ymm6
.Ltmp8435:
	.loc	21 441 27
	vmovaps	1536(%r8), %ymm8
.Ltmp8436:
	.loc	58 48 14
	vaddps	1600(%r8), %ymm8, %ymm11
.Ltmp8437:
	.loc	58 585 19
	vblendvps	%ymm6, 1568(%r8), %ymm11, %ymm11
.Ltmp8438:
	.loc	58 871 14
	vcmpgt_oqps	%ymm14, %ymm2, %ymm0
.Ltmp8439:
	.loc	21 442 13
	vmaskmovps	%ymm11, %ymm0, 1536(%r8)
	.loc	21 447 13
	vmaskmovps	%ymm14, %ymm6, 1600(%r8)
.Ltmp8440:
	.loc	58 585 19
	vblendvps	%ymm15, %ymm12, %ymm10, %ymm6
.Ltmp8441:
	.loc	58 347 14
	vaddps	%ymm2, %ymm13, %ymm2
.Ltmp8442:
	.loc	21 448 13
	vmaskmovps	%ymm2, %ymm0, 1632(%r8)
.Ltmp8443:
	.loc	58 585 19
	vblendvps	%ymm0, %ymm11, %ymm8, %ymm8
.Ltmp8444:
	.loc	21 441 27
	vmovaps	1664(%r8), %ymm0
.Ltmp8445:
	.loc	21 439 26
	vmovaps	1760(%r8), %ymm2
.Ltmp8446:
	.loc	58 871 14
	vcmpeqps	%ymm7, %ymm2, %ymm10
.Ltmp8447:
	.loc	58 48 14
	vaddps	1728(%r8), %ymm0, %ymm11
.Ltmp8448:
	.loc	58 585 19
	vblendvps	%ymm10, 1696(%r8), %ymm11, %ymm11
.Ltmp8449:
	.loc	58 871 14
	vcmpgt_oqps	%ymm14, %ymm2, %ymm15
.Ltmp8450:
	.loc	21 442 13
	vmaskmovps	%ymm11, %ymm15, 1664(%r8)
	.loc	21 447 13
	vmaskmovps	%ymm14, %ymm10, 1728(%r8)
.Ltmp8451:
	.loc	58 585 19
	vblendvps	%ymm15, %ymm11, %ymm0, %ymm12
.Ltmp8452:
	.loc	58 347 14
	vaddps	%ymm2, %ymm13, %ymm0
.Ltmp8453:
	.loc	21 448 13
	vmaskmovps	%ymm0, %ymm15, 1760(%r8)
	vmovaps	992(%rsp), %xmm0
.Ltmp8454:
	.loc	1 551 14
	vinsertps	$16, 960(%rsp), %xmm0, %xmm0
	vinsertps	$32, 928(%rsp), %xmm0, %xmm0
	vinsertps	$48, 896(%rsp), %xmm0, %xmm0
	vmovaps	768(%rsp), %xmm2
	vinsertps	$16, %xmm9, %xmm2, %xmm2
	vinsertps	$32, %xmm1, %xmm2, %xmm1
	vinsertps	$48, %xmm3, %xmm1, %xmm1
	vmovaps	416(%rsp), %xmm2
.Ltmp8455:
	.loc	1 551 14 is_stmt 0
	vinsertps	$16, 720(%rsp), %xmm2, %xmm2
	vinsertps	$32, 336(%rsp), %xmm2, %xmm2
	vinsertps	$48, 704(%rsp), %xmm2, %xmm2
	vmovaps	688(%rsp), %xmm3
	vinsertps	$16, 672(%rsp), %xmm3, %xmm3
	vinsertps	$32, 1040(%rsp), %xmm3, %xmm3
	vinsertps	$48, %xmm4, %xmm3, %xmm3
	vinsertf128	$1, %xmm3, %ymm2, %ymm2
.Ltmp8456:
	.loc	58 82 19 is_stmt 1
	vbroadcastss	.LCPI46_4(%rip), %ymm15
.Ltmp8457:
	.loc	1 551 14
	vinsertf128	$1, %xmm1, %ymm0, %ymm0
.Ltmp8458:
	.loc	58 82 19
	vandps	%ymm2, %ymm15, %ymm1
.Ltmp8459:
	.loc	58 82 19 is_stmt 0
	vandps	%ymm0, %ymm15, %ymm0
	vmovaps	%ymm15, 768(%rsp)
.Ltmp8460:
	.loc	21 459 23 is_stmt 1
	vmovaps	896(%r8), %ymm2
.Ltmp8461:
	.loc	58 871 14
	vcmpgt_oqps	%ymm14, %ymm2, %ymm2
.Ltmp8462:
	.loc	58 233 14
	vmaxps	%ymm0, %ymm1, %ymm3
.Ltmp8463:
	.loc	58 585 19
	vblendvps	%ymm2, %ymm3, %ymm1, %ymm2
	vbroadcastss	.LCPI46_5(%rip), %ymm3
.Ltmp8464:
	.loc	58 283 14
	vmulps	%ymm3, %ymm1, %ymm1
.Ltmp8465:
	.loc	58 283 14 is_stmt 0
	vmulps	%ymm3, %ymm0, %ymm0
.Ltmp8466:
	.loc	58 48 14 is_stmt 1
	vaddps	%ymm1, %ymm0, %ymm0
.Ltmp8467:
	.loc	21 461 9
	vmovaps	928(%r8), %ymm1
.Ltmp8468:
	.loc	58 871 14
	vcmpgt_oqps	%ymm14, %ymm1, %ymm1
.Ltmp8469:
	.loc	58 585 19
	vblendvps	%ymm1, %ymm0, %ymm2, %ymm0
	vbroadcastss	.LCPI46_6(%rip), %ymm1
.Ltmp8470:
	.loc	58 233 14
	vmaxps	%ymm1, %ymm0, %ymm1
.Ltmp8471:
	.file	59 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs"
	.loc	59 264 24
	vbroadcastsd	.LCPI46_8(%rip), %ymm3
	vmovaps	%ymm3, 992(%rsp)
	vbroadcastss	.LCPI46_7(%rip), %ymm0
.Ltmp8472:
	.loc	58 233 14
	vmaxps	%ymm0, %ymm1, %ymm2
.Ltmp8473:
	.loc	59 264 24
	vandps	%ymm3, %ymm2, %ymm3
.Ltmp8474:
	.loc	59 2315 14
	vbroadcastsd	.LCPI46_9(%rip), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vorps	%ymm0, %ymm3, %ymm3
.Ltmp8475:
	.loc	58 347 14
	vaddps	%ymm3, %ymm13, %ymm3
	vbroadcastss	.LCPI46_10(%rip), %ymm0
.Ltmp8476:
	.loc	58 283 14
	vmulps	%ymm0, %ymm3, %ymm4
	vbroadcastss	.LCPI46_11(%rip), %ymm0
.Ltmp8477:
	.loc	58 48 14
	vsubps	%ymm4, %ymm0, %ymm4
.Ltmp8478:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_12(%rip), %ymm0
.Ltmp8479:
	.loc	58 48 14
	vaddps	%ymm0, %ymm4, %ymm4
.Ltmp8480:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_13(%rip), %ymm0
.Ltmp8481:
	.loc	58 48 14
	vaddps	%ymm0, %ymm4, %ymm4
.Ltmp8482:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_14(%rip), %ymm0
.Ltmp8483:
	.loc	58 48 14
	vaddps	%ymm0, %ymm4, %ymm4
.Ltmp8484:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_15(%rip), %ymm0
.Ltmp8485:
	.loc	58 48 14
	vaddps	%ymm0, %ymm4, %ymm4
.Ltmp8486:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm3
.Ltmp8487:
	.loc	59 3217 24
	vpsrld	$23, %ymm2, %ymm4
.Ltmp8488:
	.loc	59 2315 24
	vpbroadcastd	.LCPI46_16(%rip), %ymm0
	vmovdqa	%ymm0, 928(%rsp)
	vpor	%ymm0, %ymm4, %ymm4
	vbroadcastss	.LCPI46_17(%rip), %ymm0
.Ltmp8489:
	.loc	58 347 14
	vaddps	%ymm0, %ymm4, %ymm4
.Ltmp8490:
	.loc	58 48 14
	vaddps	%ymm3, %ymm4, %ymm3
	vbroadcastss	.LCPI46_18(%rip), %ymm0
.Ltmp8491:
	.loc	58 283 14
	vmulps	%ymm0, %ymm3, %ymm3
	vbroadcastss	.LCPI46_19(%rip), %ymm0
.Ltmp8492:
	.loc	58 257 14
	vminps	%ymm0, %ymm3, %ymm3
	vbroadcastss	.LCPI46_20(%rip), %ymm0
.Ltmp8493:
	.loc	58 233 14
	vmaxps	%ymm0, %ymm3, %ymm3
.Ltmp8494:
	.loc	21 478 20
	vmovaps	1792(%r8), %ymm4
.Ltmp8495:
	.loc	58 871 14
	vcmpgt_oqps	%ymm14, %ymm4, %ymm4
.Ltmp8496:
	.loc	58 347 14
	vsubps	%ymm12, %ymm5, %ymm7
.Ltmp8497:
	.loc	58 871 14
	vcmpge_oqps	%ymm7, %ymm3, %ymm7
.Ltmp8498:
	.loc	58 871 14 is_stmt 0
	vcmpge_oqps	%ymm5, %ymm3, %ymm9
.Ltmp8499:
	.loc	58 82 19 is_stmt 1
	vandnps	%ymm9, %ymm4, %ymm9
.Ltmp8500:
	.loc	21 486 47
	vmovaps	1824(%r8), %ymm11
.Ltmp8501:
	.loc	58 871 14
	vcmpgt_oqps	%ymm14, %ymm11, %ymm12
.Ltmp8502:
	.loc	58 82 19
	vandnps	%ymm12, %ymm7, %ymm12
.Ltmp8503:
	.loc	58 82 19 is_stmt 0
	vandps	%ymm4, %ymm7, %ymm7
.Ltmp8504:
	.loc	58 82 19
	vandps	%ymm4, %ymm12, %ymm4
.Ltmp8505:
	.loc	58 347 14 is_stmt 1
	vaddps	%ymm13, %ymm11, %ymm12
.Ltmp8506:
	.loc	58 585 19
	vblendvps	%ymm4, %ymm12, %ymm11, %ymm11
.Ltmp8507:
	.loc	58 117 19
	vorps	%ymm7, %ymm9, %ymm7
.Ltmp8508:
	.loc	58 585 19
	vblendvps	%ymm7, 832(%r8), %ymm11, %ymm9
.Ltmp8509:
	.loc	58 117 19
	vorps	%ymm7, %ymm4, %ymm7
.Ltmp8510:
	.loc	21 508 65
	vmovaps	800(%r8), %ymm11
.Ltmp8511:
	.loc	21 514 49
	vmovaps	864(%r8), %ymm4
.Ltmp8512:
	.loc	21 489 5
	vmovaps	%ymm9, 1824(%r8)
.Ltmp8513:
	.loc	58 347 14
	vsubps	%ymm5, %ymm3, %ymm3
.Ltmp8514:
	.loc	58 347 14 is_stmt 0
	vaddps	%ymm6, %ymm13, %ymm5
.Ltmp8515:
	.loc	58 283 14 is_stmt 1
	vmulps	%ymm3, %ymm5, %ymm5
.Ltmp8516:
	.loc	58 713 19
	vbroadcastss	.LCPI46_21(%rip), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vxorps	%ymm0, %ymm8, %ymm6
.Ltmp8517:
	.loc	58 233 14
	vmaxps	%ymm6, %ymm5, %ymm5
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp8518:
	.loc	58 585 19
	vpcmpgtd	%ymm7, %ymm0, %ymm6
	vpbroadcastd	.LCPI46_2(%rip), %ymm2
	vpand	%ymm2, %ymm6, %ymm6
.Ltmp8519:
	.loc	21 498 5
	vmovdqa	%ymm6, 1792(%r8)
.Ltmp8520:
	.loc	58 257 14
	vminps	%ymm0, %ymm5, %ymm5
.Ltmp8521:
	.loc	58 871 14
	vcmpgt_oqps	%ymm0, %ymm6, %ymm6
.Ltmp8522:
	.loc	58 585 19
	vpcmpgtd	%ymm6, %ymm0, %ymm6
	vxorps	%xmm10, %xmm10, %xmm10
	vpandn	%ymm5, %ymm6, %ymm5
.Ltmp8523:
	.loc	21 508 36
	vmovaps	1856(%r8), %ymm6
.Ltmp8524:
	.loc	58 871 14
	vcmpgt_oqps	%ymm6, %ymm5, %ymm7
.Ltmp8525:
	.loc	58 585 19
	vblendvps	%ymm7, 768(%r8), %ymm11, %ymm7
.Ltmp8526:
	.loc	58 347 14
	vsubps	%ymm6, %ymm5, %ymm5
.Ltmp8527:
	.loc	58 283 14
	vmulps	%ymm7, %ymm5, %ymm5
.Ltmp8528:
	.loc	58 48 14
	vaddps	%ymm5, %ymm6, %ymm5
.Ltmp8529:
	.loc	58 82 19
	vandps	%ymm5, %ymm15, %ymm6
	vbroadcastss	.LCPI46_22(%rip), %ymm0
.Ltmp8530:
	.loc	58 871 14
	vcmplt_oqps	%ymm0, %ymm6, %ymm6
.Ltmp8531:
	.loc	58 82 19
	vandnps	%ymm5, %ymm6, %ymm9
.Ltmp8532:
	.loc	21 510 5
	vmovaps	%ymm9, 1856(%r8)
.Ltmp8533:
	.loc	21 439 26
	vmovaps	1984(%r8), %ymm5
.Ltmp8534:
	.loc	58 871 14
	vcmpeqps	%ymm2, %ymm5, %ymm6
.Ltmp8535:
	.loc	21 441 27
	vmovaps	1888(%r8), %ymm7
.Ltmp8536:
	.loc	58 48 14
	vaddps	1952(%r8), %ymm7, %ymm8
.Ltmp8537:
	.loc	58 585 19
	vblendvps	%ymm6, 1920(%r8), %ymm8, %ymm8
.Ltmp8538:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm5, %ymm11
.Ltmp8539:
	.loc	21 442 13
	vmaskmovps	%ymm8, %ymm11, 1888(%r8)
	.loc	21 447 13
	vmaskmovps	%ymm10, %ymm6, 1952(%r8)
	vmovaps	%ymm13, %ymm3
.Ltmp8540:
	.loc	58 347 14
	vaddps	%ymm5, %ymm13, %ymm5
.Ltmp8541:
	.loc	21 448 13
	vmaskmovps	%ymm5, %ymm11, 1984(%r8)
.Ltmp8542:
	.loc	21 441 27
	vmovaps	2016(%r8), %ymm6
.Ltmp8543:
	.loc	21 439 26
	vmovaps	2112(%r8), %ymm5
.Ltmp8544:
	.loc	58 871 14
	vcmpeqps	%ymm2, %ymm5, %ymm12
.Ltmp8545:
	.loc	58 48 14
	vaddps	2080(%r8), %ymm6, %ymm13
.Ltmp8546:
	.loc	58 585 19
	vblendvps	%ymm12, 2048(%r8), %ymm13, %ymm13
.Ltmp8547:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm5, %ymm15
.Ltmp8548:
	.loc	21 442 13
	vmaskmovps	%ymm13, %ymm15, 2016(%r8)
	.loc	21 447 13
	vmaskmovps	%ymm10, %ymm12, 2080(%r8)
.Ltmp8549:
	.loc	58 347 14
	vaddps	%ymm3, %ymm5, %ymm5
.Ltmp8550:
	.loc	21 448 13
	vmaskmovps	%ymm5, %ymm15, 2112(%r8)
.Ltmp8551:
	.loc	21 441 27
	vmovaps	2144(%r8), %ymm12
.Ltmp8552:
	.loc	21 439 26
	vmovaps	2240(%r8), %ymm0
.Ltmp8553:
	.loc	58 871 14
	vcmpeqps	%ymm2, %ymm0, %ymm5
.Ltmp8554:
	.loc	58 48 14
	vaddps	2208(%r8), %ymm12, %ymm1
.Ltmp8555:
	.loc	58 585 19
	vblendvps	%ymm5, 2176(%r8), %ymm1, %ymm1
.Ltmp8556:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm0, %ymm14
.Ltmp8557:
	.loc	21 442 13
	vmaskmovps	%ymm1, %ymm14, 2144(%r8)
	.loc	21 447 13
	vmaskmovps	%ymm10, %ymm5, 2208(%r8)
.Ltmp8558:
	.loc	58 585 19
	vblendvps	%ymm11, %ymm8, %ymm7, %ymm5
.Ltmp8559:
	.loc	58 347 14
	vaddps	%ymm3, %ymm0, %ymm0
.Ltmp8560:
	.loc	21 448 13
	vmaskmovps	%ymm0, %ymm14, 2240(%r8)
.Ltmp8561:
	.loc	58 585 19
	vblendvps	%ymm15, %ymm13, %ymm6, %ymm6
.Ltmp8562:
	.loc	21 441 27
	vmovaps	2272(%r8), %ymm0
.Ltmp8563:
	.loc	21 439 26
	vmovaps	2368(%r8), %ymm8
.Ltmp8564:
	.loc	58 871 14
	vcmpeqps	%ymm2, %ymm8, %ymm7
.Ltmp8565:
	.loc	58 48 14
	vaddps	2336(%r8), %ymm0, %ymm11
.Ltmp8566:
	.loc	58 585 19
	vblendvps	%ymm7, 2304(%r8), %ymm11, %ymm11
.Ltmp8567:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm8, %ymm13
.Ltmp8568:
	.loc	21 442 13
	vmaskmovps	%ymm11, %ymm13, 2272(%r8)
	.loc	21 447 13
	vmaskmovps	%ymm10, %ymm7, 2336(%r8)
.Ltmp8569:
	.loc	58 585 19
	vblendvps	%ymm14, %ymm1, %ymm12, %ymm7
.Ltmp8570:
	.loc	58 347 14
	vaddps	%ymm3, %ymm8, %ymm1
.Ltmp8571:
	.loc	21 448 13
	vmaskmovps	%ymm1, %ymm13, 2368(%r8)
.Ltmp8572:
	.loc	58 585 19
	vblendvps	%ymm13, %ymm11, %ymm0, %ymm8
	vbroadcastss	.LCPI46_23(%rip), %ymm12
.Ltmp8573:
	.loc	58 283 14
	vmulps	%ymm12, %ymm9, %ymm0
	vbroadcastss	.LCPI46_24(%rip), %ymm13
.Ltmp8574:
	.loc	58 233 14
	vmaxps	%ymm13, %ymm0, %ymm0
	vbroadcastss	.LCPI46_25(%rip), %ymm14
.Ltmp8575:
	.loc	58 257 14
	vminps	%ymm14, %ymm0, %ymm0
.Ltmp8576:
	.loc	58 471 14
	vroundps	$9, %ymm0, %ymm1
.Ltmp8577:
	.loc	58 347 14
	vsubps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI46_26(%rip), %ymm15
.Ltmp8578:
	.loc	58 283 14
	vmulps	%ymm0, %ymm15, %ymm11
	vbroadcastss	.LCPI46_27(%rip), %ymm12
.Ltmp8579:
	.loc	58 48 14
	vaddps	%ymm12, %ymm11, %ymm11
.Ltmp8580:
	.loc	58 283 14
	vmulps	%ymm0, %ymm11, %ymm11
	vbroadcastss	.LCPI46_28(%rip), %ymm12
.Ltmp8581:
	.loc	58 48 14
	vaddps	%ymm12, %ymm11, %ymm11
.Ltmp8582:
	.loc	58 283 14
	vmulps	%ymm0, %ymm11, %ymm11
	vbroadcastss	.LCPI46_29(%rip), %ymm12
.Ltmp8583:
	.loc	58 48 14
	vaddps	%ymm12, %ymm11, %ymm11
.Ltmp8584:
	.loc	58 283 14
	vmulps	%ymm0, %ymm11, %ymm11
	vbroadcastss	.LCPI46_30(%rip), %ymm12
.Ltmp8585:
	.loc	58 48 14
	vaddps	%ymm12, %ymm11, %ymm11
.Ltmp8586:
	.loc	58 283 14
	vmulps	%ymm0, %ymm11, %ymm0
.Ltmp8587:
	.loc	58 48 14
	vaddps	%ymm2, %ymm0, %ymm0
	vmovdqa	%ymm2, %ymm13
	vbroadcastss	.LCPI46_31(%rip), %ymm11
.Ltmp8588:
	.loc	58 48 14 is_stmt 0
	vaddps	%ymm1, %ymm11, %ymm1
.Ltmp8589:
	.loc	59 2798 24 is_stmt 1
	vpslld	$23, %ymm1, %ymm1
.Ltmp8590:
	.loc	58 283 14
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp8591:
	.loc	58 871 14
	vcmpeqps	%ymm10, %ymm9, %ymm1
.Ltmp8592:
	.loc	58 871 14 is_stmt 0
	vcmpgt_oqps	%ymm10, %ymm4, %ymm4
.Ltmp8593:
	.loc	58 117 19 is_stmt 1
	vorps	%ymm1, %ymm4, %ymm1
	vmovaps	384(%rsp), %ymm4
.Ltmp8594:
	.loc	58 283 14
	vmulps	%ymm0, %ymm4, %ymm0
.Ltmp8595:
	.loc	58 585 19
	vblendvps	%ymm1, %ymm4, %ymm0, %ymm12
	vmovaps	144(%rsp), %xmm0
.Ltmp8596:
	.loc	1 551 14
	vinsertps	$16, 192(%rsp), %xmm0, %xmm0
	vinsertps	$32, 176(%rsp), %xmm0, %xmm0
	vinsertps	$48, 128(%rsp), %xmm0, %xmm0
	vmovaps	512(%rsp), %xmm1
	vinsertps	$16, 304(%rsp), %xmm1, %xmm1
	vinsertps	$32, 800(%rsp), %xmm1, %xmm1
	vinsertps	$48, 592(%rsp), %xmm1, %xmm1
	vmovaps	224(%rsp), %xmm9
.Ltmp8597:
	.loc	1 551 14 is_stmt 0
	vinsertps	$16, 352(%rsp), %xmm9, %xmm9
	vinsertps	$32, 320(%rsp), %xmm9, %xmm9
	vinsertps	$48, 288(%rsp), %xmm9, %xmm9
	vmovaps	272(%rsp), %xmm11
	vinsertps	$16, 448(%rsp), %xmm11, %xmm11
	vinsertps	$32, 432(%rsp), %xmm11, %xmm11
	vinsertps	$48, 736(%rsp), %xmm11, %xmm11
	vinsertf128	$1, %xmm11, %ymm9, %ymm9
.Ltmp8598:
	.loc	21 459 23 is_stmt 1
	vmovaps	1088(%r8), %ymm11
	vmovaps	768(%rsp), %ymm4
.Ltmp8599:
	.loc	58 82 19
	vandps	%ymm4, %ymm9, %ymm9
.Ltmp8600:
	.loc	1 551 14
	vinsertf128	$1, %xmm1, %ymm0, %ymm0
.Ltmp8601:
	.loc	58 82 19
	vandps	%ymm4, %ymm0, %ymm0
.Ltmp8602:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm11, %ymm1
.Ltmp8603:
	.loc	58 233 14
	vmaxps	%ymm0, %ymm9, %ymm11
.Ltmp8604:
	.loc	58 585 19
	vblendvps	%ymm1, %ymm11, %ymm9, %ymm1
	vbroadcastss	.LCPI46_5(%rip), %ymm11
.Ltmp8605:
	.loc	58 283 14
	vmulps	%ymm11, %ymm9, %ymm9
.Ltmp8606:
	.loc	58 283 14 is_stmt 0
	vmulps	%ymm0, %ymm11, %ymm0
.Ltmp8607:
	.loc	58 48 14 is_stmt 1
	vaddps	%ymm0, %ymm9, %ymm0
.Ltmp8608:
	.loc	21 461 9
	vmovaps	1120(%r8), %ymm9
.Ltmp8609:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm9, %ymm9
.Ltmp8610:
	.loc	58 585 19
	vblendvps	%ymm9, %ymm0, %ymm1, %ymm0
.Ltmp8611:
	.loc	58 233 14
	vbroadcastss	.LCPI46_6(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8612:
	.loc	58 233 14 is_stmt 0
	vbroadcastss	.LCPI46_7(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8613:
	.loc	59 264 24 is_stmt 1
	vandps	992(%rsp), %ymm0, %ymm1
.Ltmp8614:
	.loc	59 2315 14
	vorps	960(%rsp), %ymm1, %ymm1
.Ltmp8615:
	.loc	58 347 14
	vaddps	%ymm3, %ymm1, %ymm1
.Ltmp8616:
	.loc	58 283 14
	vbroadcastss	.LCPI46_10(%rip), %ymm9
	vmulps	%ymm1, %ymm9, %ymm9
.Ltmp8617:
	.loc	58 48 14
	vbroadcastss	.LCPI46_11(%rip), %ymm2
	vsubps	%ymm9, %ymm2, %ymm9
.Ltmp8618:
	.loc	58 283 14
	vmulps	%ymm1, %ymm9, %ymm9
.Ltmp8619:
	.loc	58 48 14
	vbroadcastss	.LCPI46_12(%rip), %ymm2
	vaddps	%ymm2, %ymm9, %ymm9
.Ltmp8620:
	.loc	58 283 14
	vmulps	%ymm1, %ymm9, %ymm9
.Ltmp8621:
	.loc	58 48 14
	vbroadcastss	.LCPI46_13(%rip), %ymm2
	vaddps	%ymm2, %ymm9, %ymm9
.Ltmp8622:
	.loc	58 283 14
	vmulps	%ymm1, %ymm9, %ymm9
.Ltmp8623:
	.loc	58 48 14
	vbroadcastss	.LCPI46_14(%rip), %ymm2
	vaddps	%ymm2, %ymm9, %ymm9
.Ltmp8624:
	.loc	58 283 14
	vmulps	%ymm1, %ymm9, %ymm9
.Ltmp8625:
	.loc	58 48 14
	vbroadcastss	.LCPI46_15(%rip), %ymm2
	vaddps	%ymm2, %ymm9, %ymm9
.Ltmp8626:
	.loc	59 3217 24
	vpsrld	$23, %ymm0, %ymm0
.Ltmp8627:
	.loc	59 2315 24
	vpor	928(%rsp), %ymm0, %ymm0
.Ltmp8628:
	.loc	58 347 14
	vbroadcastss	.LCPI46_17(%rip), %ymm2
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp8629:
	.loc	58 283 14
	vmulps	%ymm1, %ymm9, %ymm1
.Ltmp8630:
	.loc	58 48 14
	vaddps	%ymm1, %ymm0, %ymm0
.Ltmp8631:
	.loc	58 283 14
	vbroadcastss	.LCPI46_18(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp8632:
	.loc	58 257 14
	vbroadcastss	.LCPI46_19(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
.Ltmp8633:
	.loc	58 233 14
	vbroadcastss	.LCPI46_20(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm1
.Ltmp8634:
	.loc	21 478 20
	vmovaps	2400(%r8), %ymm0
.Ltmp8635:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm0, %ymm0
.Ltmp8636:
	.loc	58 871 14 is_stmt 0
	vcmpge_oqps	%ymm5, %ymm1, %ymm2
.Ltmp8637:
	.loc	58 347 14 is_stmt 1
	vsubps	%ymm8, %ymm5, %ymm8
.Ltmp8638:
	.loc	58 871 14
	vcmpge_oqps	%ymm8, %ymm1, %ymm8
.Ltmp8639:
	.loc	58 82 19
	vandnps	%ymm2, %ymm0, %ymm2
.Ltmp8640:
	.loc	58 82 19 is_stmt 0
	vandps	%ymm0, %ymm8, %ymm9
.Ltmp8641:
	.loc	58 117 19 is_stmt 1
	vorps	%ymm2, %ymm9, %ymm2
.Ltmp8642:
	.loc	21 486 47
	vmovaps	2432(%r8), %ymm9
.Ltmp8643:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm9, %ymm11
.Ltmp8644:
	.loc	58 82 19
	vandnps	%ymm11, %ymm8, %ymm8
	vandps	%ymm0, %ymm8, %ymm0
.Ltmp8645:
	.loc	58 347 14
	vaddps	%ymm3, %ymm9, %ymm8
.Ltmp8646:
	.loc	58 585 19
	vblendvps	%ymm0, %ymm8, %ymm9, %ymm8
.Ltmp8647:
	.loc	21 508 65
	vmovaps	992(%r8), %ymm9
.Ltmp8648:
	.loc	58 585 19
	vblendvps	%ymm2, 1024(%r8), %ymm8, %ymm8
.Ltmp8649:
	.loc	58 117 19
	vorps	%ymm2, %ymm0, %ymm2
.Ltmp8650:
	.loc	21 514 49
	vmovaps	1056(%r8), %ymm0
.Ltmp8651:
	.loc	21 489 5
	vmovaps	%ymm8, 2432(%r8)
.Ltmp8652:
	.loc	58 347 14
	vsubps	%ymm5, %ymm1, %ymm1
.Ltmp8653:
	.loc	58 347 14 is_stmt 0
	vaddps	%ymm3, %ymm6, %ymm5
.Ltmp8654:
	.loc	58 283 14 is_stmt 1
	vmulps	%ymm1, %ymm5, %ymm1
.Ltmp8655:
	.loc	58 713 19
	vxorps	896(%rsp), %ymm7, %ymm3
.Ltmp8656:
	.loc	58 585 19
	vpcmpgtd	%ymm2, %ymm10, %ymm2
	vpand	%ymm2, %ymm13, %ymm2
.Ltmp8657:
	.loc	21 498 5
	vmovdqa	%ymm2, 2400(%r8)
.Ltmp8658:
	.loc	58 233 14
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp8659:
	.loc	21 508 36
	vmovaps	2464(%r8), %ymm3
.Ltmp8660:
	.loc	58 257 14
	vminps	%ymm10, %ymm1, %ymm1
.Ltmp8661:
	.loc	58 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm2
.Ltmp8662:
	.loc	58 585 19
	vpcmpgtd	%ymm2, %ymm10, %ymm2
	vpandn	%ymm1, %ymm2, %ymm1
.Ltmp8663:
	.loc	58 871 14
	vcmpgt_oqps	%ymm3, %ymm1, %ymm2
.Ltmp8664:
	.loc	58 585 19
	vblendvps	%ymm2, 960(%r8), %ymm9, %ymm2
.Ltmp8665:
	.loc	58 347 14
	vsubps	%ymm3, %ymm1, %ymm1
.Ltmp8666:
	.loc	58 283 14
	vmulps	%ymm2, %ymm1, %ymm1
.Ltmp8667:
	.loc	58 48 14
	vaddps	%ymm1, %ymm3, %ymm1
.Ltmp8668:
	.loc	58 82 19
	vandps	%ymm4, %ymm1, %ymm2
.Ltmp8669:
	.loc	58 871 14
	vbroadcastss	.LCPI46_22(%rip), %ymm3
	vcmplt_oqps	%ymm3, %ymm2, %ymm2
.Ltmp8670:
	.loc	58 82 19
	vandnps	%ymm1, %ymm2, %ymm1
.Ltmp8671:
	.loc	58 283 14
	vbroadcastss	.LCPI46_23(%rip), %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
.Ltmp8672:
	.loc	58 233 14
	vbroadcastss	.LCPI46_24(%rip), %ymm3
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp8673:
	.loc	58 257 14
	vminps	%ymm14, %ymm2, %ymm2
.Ltmp8674:
	.loc	58 471 14
	vroundps	$9, %ymm2, %ymm3
.Ltmp8675:
	.loc	58 347 14
	vsubps	%ymm3, %ymm2, %ymm2
.Ltmp8676:
	.loc	58 283 14
	vmulps	%ymm2, %ymm15, %ymm5
.Ltmp8677:
	.loc	58 48 14
	vbroadcastss	.LCPI46_27(%rip), %ymm4
	vaddps	%ymm4, %ymm5, %ymm5
.Ltmp8678:
	.loc	58 283 14
	vmulps	%ymm5, %ymm2, %ymm5
.Ltmp8679:
	.loc	58 48 14
	vbroadcastss	.LCPI46_28(%rip), %ymm4
	vaddps	%ymm4, %ymm5, %ymm5
.Ltmp8680:
	.loc	58 283 14
	vmulps	%ymm5, %ymm2, %ymm5
.Ltmp8681:
	.loc	58 48 14
	vbroadcastss	.LCPI46_29(%rip), %ymm4
	vaddps	%ymm4, %ymm5, %ymm5
.Ltmp8682:
	.loc	58 283 14
	vmulps	%ymm5, %ymm2, %ymm5
.Ltmp8683:
	.loc	58 48 14
	vbroadcastss	.LCPI46_30(%rip), %ymm4
	vaddps	%ymm4, %ymm5, %ymm5
.Ltmp8684:
	.loc	58 283 14
	vmulps	%ymm5, %ymm2, %ymm2
.Ltmp8685:
	.loc	58 48 14
	vaddps	%ymm2, %ymm13, %ymm2
.Ltmp8686:
	.loc	58 48 14 is_stmt 0
	vbroadcastss	.LCPI46_31(%rip), %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
.Ltmp8687:
	.loc	59 2798 24 is_stmt 1
	vpslld	$23, %ymm3, %ymm3
.Ltmp8688:
	.loc	58 283 14
	vmulps	%ymm3, %ymm2, %ymm2
.Ltmp8689:
	.loc	58 871 14
	vcmpeqps	%ymm1, %ymm10, %ymm3
.Ltmp8690:
	.loc	58 871 14 is_stmt 0
	vcmpgt_oqps	%ymm10, %ymm0, %ymm0
.Ltmp8691:
	.loc	58 117 19 is_stmt 1
	vorps	%ymm3, %ymm0, %ymm0
	vmovaps	544(%rsp), %ymm3
.Ltmp8692:
	.loc	58 283 14
	vmulps	%ymm2, %ymm3, %ymm2
.Ltmp8693:
	.loc	58 585 19
	vblendvps	%ymm0, %ymm3, %ymm2, %ymm0
.Ltmp8694:
	.loc	21 510 5
	vmovaps	%ymm1, 2464(%r8)
	movq	864(%rsp), %rax
.Ltmp8695:
	.loc	1 551 14
	vmovups	%ymm12, (%rax)
	movq	832(%rsp), %rax
.Ltmp8696:
	.loc	1 551 14 is_stmt 0
	vmovups	%ymm0, (%rax)
	movq	88(%rsp), %r12
.Ltmp8697:
	.loc	21 0 0
	incq	%r12
	movq	216(%rsp), %rdi
.Ltmp8698:
	.loc	8 1916 50 is_stmt 1
	addq	$8, %rdi
	movq	752(%rsp), %rcx
	cmpq	%r12, %rcx
	movq	248(%rsp), %r9
	movl	508(%rsp), %r14d
	movq	120(%rsp), %r13
	movl	24(%rsp), %r15d
.Ltmp8699:
	.loc	11 900 12
	je	.LBB46_151
.Ltmp8700:
.LBB46_77:
	.loc	15 1050 16
	cmpq	%r12, %rcx
	je	.LBB46_274
.Ltmp8701:
	.loc	21 0 0 is_stmt 0
	leal	(%r12,%r13), %r10d
	andl	%r15d, %r10d
	shlq	$3, %r10
.Ltmp8702:
	.loc	21 362 77 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp8703:
	.loc	25 451 16
	cmpq	%rdx, %rsi
	ja	.LBB46_278
.Ltmp8704:
	.loc	25 0 16 is_stmt 0
	movq	488(%rsp), %rax
	.loc	21 362 0 is_stmt 1
	leaq	(%rax,%rdi,4), %r9
.Ltmp8705:
	.loc	1 551 14
	vmovups	(%r9), %ymm0
	movq	96(%rsp), %rax
	vmovups	%ymm0, (%rax,%r10,4)
.Ltmp8706:
	.loc	25 451 16
	cmpq	%rbx, %rsi
	ja	.LBB46_279
.Ltmp8707:
	.loc	25 0 16 is_stmt 0
	movq	%rbx, %rdx
	movq	480(%rsp), %rax
	.loc	21 363 0 is_stmt 1
	leaq	(%rax,%rdi,4), %rcx
.Ltmp8708:
	.loc	1 551 14
	vmovups	(%rcx), %ymm0
	movq	%r11, %rbx
	vmovups	%ymm0, (%r11,%r10,4)
	movq	1080(%rsp), %rax
.Ltmp8709:
	.loc	21 370 21
	leal	(%rax,%r12), %r11d
	andl	%r15d, %r11d
.Ltmp8710:
	.loc	21 371 54
	leaq	8(,%r11,8), %rsi
.Ltmp8711:
	.loc	21 370 20
	shlq	$3, %r11
	movq	16(%rsp), %r10
.Ltmp8712:
	.loc	25 438 16
	cmpq	%r10, %rsi
	ja	.LBB46_280
.Ltmp8713:
	.loc	25 438 16 is_stmt 0
	cmpq	%rdx, %rsi
	ja	.LBB46_281
.Ltmp8714:
	.loc	25 0 16
	movq	96(%rsp), %rax
.Ltmp8715:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rax,%r11,4), %ymm1
.Ltmp8716:
	.loc	21 0 0 is_stmt 0
	leal	(%r12,%r13), %esi
	movl	%esi, %eax
	subl	1184(%r8), %eax
.Ltmp8717:
	.loc	1 551 14
	vmovups	(%rbx,%r11,4), %ymm0
.Ltmp8718:
	.loc	21 0 0
	andl	%r15d, %eax
	subl	1248(%r8), %esi
	shlq	$3, %rax
	andl	%r15d, %esi
	shlq	$3, %rsi
	.loc	21 229 5 is_stmt 1
	testb	%r14b, %r14b
	movq	%rdi, 216(%rsp)
	movq	%r12, 88(%rsp)
	movq	%r9, 864(%rsp)
	movq	%rcx, 832(%rsp)
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	%ymm1, 384(%rsp)
	movq	%rsi, 144(%rsp)
	movq	%rax, 192(%rsp)
	je	.LBB46_101
	cmpl	$1, 504(%rsp)
	movq	%rdx, %rbx
	jne	.LBB46_118
	.loc	21 0 0 is_stmt 0
	cmpq	%r10, %rax
.Ltmp8719:
	.loc	21 251 32 is_stmt 1
	jae	.LBB46_294
.Ltmp8720:
	.loc	21 252 33
	cmpq	%rbx, %rsi
	jae	.LBB46_297
.Ltmp8721:
	.loc	21 248 29
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1188(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 248 28 is_stmt 0
	leaq	1(,%rcx,8), %rdx
.Ltmp8722:
	.loc	21 251 32 is_stmt 1
	cmpq	%r10, %rdx
	jae	.LBB46_350
.Ltmp8723:
	.loc	21 0 0 is_stmt 0
	subl	1252(%r8), %eax
	andl	%r15d, %eax
	leaq	1(,%rax,8), %r11
.Ltmp8724:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbx, %r11
	jae	.LBB46_312
.Ltmp8725:
	.loc	21 248 29
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1192(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 248 28 is_stmt 0
	leaq	2(,%rcx,8), %r9
.Ltmp8726:
	.loc	21 251 32 is_stmt 1
	cmpq	%r10, %r9
	jae	.LBB46_322
.Ltmp8727:
	.loc	21 0 0 is_stmt 0
	subl	1256(%r8), %eax
	andl	%r15d, %eax
	leaq	2(,%rax,8), %rsi
.Ltmp8728:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbx, %rsi
	jae	.LBB46_297
.Ltmp8729:
	.loc	21 248 29
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1196(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 248 28 is_stmt 0
	leaq	3(,%rcx,8), %rdi
.Ltmp8730:
	.loc	21 251 32 is_stmt 1
	cmpq	%r10, %rdi
	jae	.LBB46_385
.Ltmp8731:
	.loc	21 0 0 is_stmt 0
	subl	1260(%r8), %eax
	andl	%r15d, %eax
	leaq	3(,%rax,8), %rax
.Ltmp8732:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbx, %rax
	jae	.LBB46_346
.Ltmp8733:
	.loc	21 0 33 is_stmt 0
	movq	%rax, 176(%rsp)
	.loc	21 248 29 is_stmt 1
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1200(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 248 28 is_stmt 0
	leaq	4(,%rcx,8), %r12
.Ltmp8734:
	.loc	21 251 32 is_stmt 1
	cmpq	%r10, %r12
	jae	.LBB46_308
.Ltmp8735:
	.loc	21 0 32 is_stmt 0
	movq	%r11, 128(%rsp)
	subl	1264(%r8), %eax
	andl	%r15d, %eax
	leaq	4(,%rax,8), %r14
.Ltmp8736:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbx, %r14
	jae	.LBB46_318
.Ltmp8737:
	.loc	21 0 33 is_stmt 0
	movq	88(%rsp), %rax
	.loc	21 248 29 is_stmt 1
	addl	%r13d, %eax
	movl	%eax, %ecx
	subl	1204(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 248 28 is_stmt 0
	leaq	5(,%rcx,8), %r11
.Ltmp8738:
	.loc	21 251 32 is_stmt 1
	cmpq	%r10, %r11
	jae	.LBB46_328
.Ltmp8739:
	.loc	21 0 32 is_stmt 0
	movq	%r12, 224(%rsp)
	subl	1268(%r8), %eax
	andl	%r15d, %eax
	leaq	5(,%rax,8), %r10
.Ltmp8740:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbx, %r10
	jae	.LBB46_335
.Ltmp8741:
	.loc	21 0 33 is_stmt 0
	movq	88(%rsp), %rax
	.loc	21 248 29 is_stmt 1
	leal	(%rax,%r13), %ecx
	movl	%ecx, %eax
	subl	1208(%r8), %eax
	andl	%r15d, %eax
	.loc	21 248 28 is_stmt 0
	leaq	6(,%rax,8), %rax
.Ltmp8742:
	.loc	21 251 32 is_stmt 1
	cmpq	16(%rsp), %rax
	jae	.LBB46_294
.Ltmp8743:
	.loc	21 0 0 is_stmt 0
	subl	1272(%r8), %ecx
	andl	%r15d, %ecx
	leaq	6(,%rcx,8), %r13
.Ltmp8744:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbx, %r13
	jae	.LBB46_342
.Ltmp8745:
	.loc	21 0 33 is_stmt 0
	movq	120(%rsp), %rcx
	movq	88(%rsp), %rbx
	.loc	21 248 29 is_stmt 1
	addl	%ebx, %ecx
	movl	%ecx, %ebx
	subl	1212(%r8), %ebx
	andl	%r15d, %ebx
	.loc	21 248 28 is_stmt 0
	leaq	7(,%rbx,8), %rbx
.Ltmp8746:
	.loc	21 251 32 is_stmt 1
	cmpq	16(%rsp), %rbx
	jae	.LBB46_304
.Ltmp8747:
	.loc	21 0 0 is_stmt 0
	subl	1276(%r8), %ecx
	andl	%r15d, %ecx
	leaq	7(,%rcx,8), %rcx
.Ltmp8748:
	.loc	21 252 33 is_stmt 1
	cmpq	104(%rsp), %rcx
	jae	.LBB46_314
.Ltmp8749:
	.loc	21 0 33 is_stmt 0
	movq	%rdi, %r12
	movq	%rsi, %rdi
	movq	%r9, %rsi
	movq	%rdx, %r9
	movq	96(%rsp), %rdx
	movq	192(%rsp), %r8
	vmovss	(%rdx,%r8,4), %xmm0
	movq	472(%rsp), %r8
	movq	144(%rsp), %r15
	vmovss	(%r8,%r15,4), %xmm4
	vmovss	(%rdx,%r9,4), %xmm2
	movq	128(%rsp), %r9
	vmovss	(%r8,%r9,4), %xmm7
	vmovss	(%rdx,%rsi,4), %xmm5
	vmovss	(%r8,%rdi,4), %xmm10
	vmovss	(%rdx,%r12,4), %xmm6
	movq	176(%rsp), %rsi
	vmovss	(%r8,%rsi,4), %xmm11
	movq	224(%rsp), %rsi
	vmovss	(%rdx,%rsi,4), %xmm8
	vmovss	(%r8,%r14,4), %xmm12
	vmovss	(%rdx,%r11,4), %xmm13
	movq	%r8, %r11
	vmovss	(%r8,%r10,4), %xmm9
	vmovss	(%rdx,%rax,4), %xmm14
	vmovss	(%r8,%r13,4), %xmm1
.Ltmp8750:
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%rbx,4), %xmm15
.Ltmp8751:
	.loc	21 252 33
	vmovss	(%r8,%rcx,4), %xmm3
	vmovaps	%xmm3, 736(%rsp)
	vmovaps	%xmm1, 432(%rsp)
	vmovaps	%xmm9, 448(%rsp)
	vmovaps	%xmm12, 272(%rsp)
	vmovaps	%xmm12, 768(%rsp)
	vmovaps	%xmm11, 288(%rsp)
	vmovaps	%xmm11, 896(%rsp)
	vmovaps	%xmm10, 320(%rsp)
	vmovaps	%xmm10, 928(%rsp)
	vmovaps	%xmm7, 352(%rsp)
	vmovaps	%xmm7, 960(%rsp)
	vmovaps	%xmm4, 224(%rsp)
	vmovaps	%xmm4, 992(%rsp)
	vmovaps	%xmm15, 592(%rsp)
	vmovaps	%xmm15, %xmm4
	vmovaps	%xmm14, 800(%rsp)
	vmovaps	%xmm14, 1040(%rsp)
	vmovaps	%xmm13, 304(%rsp)
	vmovaps	%xmm13, 672(%rsp)
	vmovaps	%xmm8, 512(%rsp)
	vmovaps	%xmm8, 688(%rsp)
	vmovaps	%xmm6, 128(%rsp)
	vmovaps	%xmm6, 704(%rsp)
	vmovaps	%xmm5, 176(%rsp)
	vmovaps	%xmm5, 336(%rsp)
	vmovaps	%xmm2, 192(%rsp)
	vmovaps	%xmm2, 720(%rsp)
	vmovaps	%xmm0, 144(%rsp)
	vmovaps	%xmm0, 416(%rsp)
	jmp	.LBB46_76
.Ltmp8752:
	.loc	21 0 33 is_stmt 0
.Ltmp8753:
	.p2align	4
.LBB46_101:
	cmpq	%r10, %rax
.Ltmp8754:
	.loc	21 236 32 is_stmt 1
	jae	.LBB46_296
.Ltmp8755:
	.loc	21 237 33
	cmpq	%rdx, %rsi
	jae	.LBB46_299
.Ltmp8756:
	.loc	21 233 29
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1188(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 233 28 is_stmt 0
	leaq	1(,%rcx,8), %r11
.Ltmp8757:
	.loc	21 236 32 is_stmt 1
	cmpq	%r10, %r11
	jae	.LBB46_324
.Ltmp8758:
	.loc	21 0 0 is_stmt 0
	subl	1252(%r8), %eax
	andl	%r15d, %eax
	leaq	1(,%rax,8), %rbx
.Ltmp8759:
	.loc	21 237 33 is_stmt 1
	cmpq	%rdx, %rbx
	jae	.LBB46_333
.Ltmp8760:
	.loc	21 233 29
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1192(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 233 28 is_stmt 0
	leaq	2(,%rcx,8), %r14
.Ltmp8761:
	.loc	21 236 32 is_stmt 1
	cmpq	%r10, %r14
	jae	.LBB46_339
.Ltmp8762:
	.loc	21 0 0 is_stmt 0
	subl	1256(%r8), %eax
	andl	%r15d, %eax
	leaq	2(,%rax,8), %rcx
	movq	%rcx, 176(%rsp)
.Ltmp8763:
	.loc	21 237 33 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_348
.Ltmp8764:
	.loc	21 233 29
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1196(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 233 28 is_stmt 0
	leaq	3(,%rcx,8), %r12
.Ltmp8765:
	.loc	21 236 32 is_stmt 1
	cmpq	%r10, %r12
	jae	.LBB46_310
.Ltmp8766:
	.loc	21 0 0 is_stmt 0
	subl	1260(%r8), %eax
	andl	%r15d, %eax
	leaq	3(,%rax,8), %rcx
	movq	%rcx, 128(%rsp)
.Ltmp8767:
	.loc	21 237 33 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_320
.Ltmp8768:
	.loc	21 0 33 is_stmt 0
	movq	88(%rsp), %rax
	.loc	21 233 29 is_stmt 1
	addl	%r13d, %eax
	movl	%eax, %ecx
	subl	1200(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 233 28 is_stmt 0
	leaq	4(,%rcx,8), %rcx
.Ltmp8769:
	.loc	21 236 32 is_stmt 1
	cmpq	%r10, %rcx
	jae	.LBB46_330
.Ltmp8770:
	.loc	21 0 0 is_stmt 0
	subl	1264(%r8), %eax
	andl	%r15d, %eax
	leaq	4(,%rax,8), %rsi
.Ltmp8771:
	.loc	21 237 33 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB46_299
.Ltmp8772:
	.loc	21 0 33 is_stmt 0
	movq	%r14, 224(%rsp)
	movq	88(%rsp), %rax
	.loc	21 233 29 is_stmt 1
	addl	%r13d, %eax
	movl	%eax, %r9d
	subl	1204(%r8), %r9d
	andl	%r15d, %r9d
	.loc	21 233 28 is_stmt 0
	leaq	5(,%r9,8), %r10
.Ltmp8773:
	.loc	21 236 32 is_stmt 1
	cmpq	16(%rsp), %r10
	jae	.LBB46_306
.Ltmp8774:
	.loc	21 0 32 is_stmt 0
	movq	%rsi, 512(%rsp)
	subl	1268(%r8), %eax
	andl	%r15d, %eax
	leaq	5(,%rax,8), %r14
.Ltmp8775:
	.loc	21 237 33 is_stmt 1
	cmpq	%rdx, %r14
	jae	.LBB46_316
.Ltmp8776:
	.loc	21 0 33 is_stmt 0
	movq	%rcx, 352(%rsp)
	movq	88(%rsp), %rax
	.loc	21 233 29 is_stmt 1
	leal	(%rax,%r13), %r9d
	movl	%r9d, %eax
	subl	1208(%r8), %eax
	andl	%r15d, %eax
	.loc	21 233 28 is_stmt 0
	leaq	6(,%rax,8), %rax
.Ltmp8777:
	.loc	21 236 32 is_stmt 1
	cmpq	16(%rsp), %rax
	jae	.LBB46_296
.Ltmp8778:
	.loc	21 0 0 is_stmt 0
	subl	1272(%r8), %r9d
	andl	%r15d, %r9d
	leaq	6(,%r9,8), %r13
.Ltmp8779:
	.loc	21 237 33 is_stmt 1
	cmpq	%rdx, %r13
	jae	.LBB46_326
.Ltmp8780:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, %rcx
	movq	120(%rsp), %rdx
	movq	88(%rsp), %rdi
	.loc	21 233 29 is_stmt 1
	leal	(%rdx,%rdi), %ebx
	movl	%ebx, %r9d
	subl	1212(%r8), %r9d
	andl	%r15d, %r9d
	.loc	21 233 28 is_stmt 0
	leaq	7(,%r9,8), %r9
.Ltmp8781:
	.loc	21 236 32 is_stmt 1
	cmpq	16(%rsp), %r9
	jae	.LBB46_334
.Ltmp8782:
	.loc	21 0 0 is_stmt 0
	subl	1276(%r8), %ebx
	andl	%r15d, %ebx
	leaq	7(,%rbx,8), %rbx
.Ltmp8783:
	.loc	21 237 33 is_stmt 1
	cmpq	104(%rsp), %rbx
	jae	.LBB46_333
.Ltmp8784:
	.loc	21 0 33 is_stmt 0
	movq	%r12, %rsi
	movq	%r11, %r12
	movq	96(%rsp), %rdx
	movq	192(%rsp), %rdi
	vmovss	(%rdx,%rdi,4), %xmm11
	movq	472(%rsp), %r11
	movq	144(%rsp), %rdi
	vmovss	(%r11,%rdi,4), %xmm0
	vmovss	(%rdx,%r12,4), %xmm12
	vmovss	(%r11,%rcx,4), %xmm2
	movq	224(%rsp), %rcx
	vmovss	(%rdx,%rcx,4), %xmm13
	movq	176(%rsp), %rcx
	vmovss	(%r11,%rcx,4), %xmm4
	vmovss	(%rdx,%rsi,4), %xmm14
	movq	128(%rsp), %rcx
	vmovss	(%r11,%rcx,4), %xmm5
	movq	352(%rsp), %rcx
	vmovss	(%rdx,%rcx,4), %xmm15
	movq	512(%rsp), %rcx
	vmovss	(%r11,%rcx,4), %xmm6
	vmovss	(%rdx,%r10,4), %xmm9
	vmovss	(%r11,%r14,4), %xmm7
	vmovss	(%rdx,%rax,4), %xmm1
	vmovss	(%r11,%r13,4), %xmm8
.Ltmp8785:
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%r9,4), %xmm3
.Ltmp8786:
	.loc	21 237 33
	vmovss	(%r11,%rbx,4), %xmm10
	vmovaps	%xmm10, 592(%rsp)
	vmovaps	%xmm10, 736(%rsp)
	vmovaps	%xmm8, 800(%rsp)
	vmovaps	%xmm8, 432(%rsp)
	vmovaps	%xmm7, 304(%rsp)
	vmovaps	%xmm7, 448(%rsp)
	vmovaps	%xmm6, 512(%rsp)
	vmovaps	%xmm6, 272(%rsp)
	vmovaps	%xmm5, 128(%rsp)
	vmovaps	%xmm5, 288(%rsp)
	vmovaps	%xmm4, 176(%rsp)
	vmovaps	%xmm4, 320(%rsp)
	vmovaps	%xmm2, 192(%rsp)
	vmovaps	%xmm2, 352(%rsp)
	vmovaps	%xmm0, 144(%rsp)
	vmovaps	%xmm0, 224(%rsp)
	vmovaps	%xmm3, %xmm4
	vmovaps	%xmm1, 1040(%rsp)
	vmovaps	%xmm9, 672(%rsp)
	vmovaps	%xmm15, 768(%rsp)
	vmovaps	%xmm15, 688(%rsp)
	vmovaps	%xmm14, 896(%rsp)
	vmovaps	%xmm14, 704(%rsp)
	vmovaps	%xmm13, 928(%rsp)
	vmovaps	%xmm13, 336(%rsp)
	vmovaps	%xmm12, 960(%rsp)
	vmovaps	%xmm12, 720(%rsp)
	vmovaps	%xmm11, 992(%rsp)
	vmovaps	%xmm11, 416(%rsp)
	jmp	.LBB46_76
.Ltmp8787:
	.loc	21 0 33 is_stmt 0
.Ltmp8788:
	.p2align	4
.LBB46_118:
	cmpq	%r10, %rax
.Ltmp8789:
	.loc	21 266 33 is_stmt 1
	jae	.LBB46_300
	.loc	21 267 33
	cmpq	%rbx, %rax
	jae	.LBB46_341
	.loc	21 268 33
	cmpq	%rbx, %rsi
	jae	.LBB46_298
	.loc	21 269 33
	cmpq	%r10, %rsi
	jae	.LBB46_295
.Ltmp8790:
	.loc	21 263 29
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1188(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 263 28 is_stmt 0
	leaq	1(,%rcx,8), %rdx
.Ltmp8791:
	.loc	21 266 33 is_stmt 1
	cmpq	%r10, %rdx
	jae	.LBB46_351
	.loc	21 267 33
	cmpq	%rbx, %rdx
	jae	.LBB46_313
.Ltmp8792:
	.loc	21 0 0 is_stmt 0
	subl	1252(%r8), %eax
	andl	%r15d, %eax
	leaq	1(,%rax,8), %r11
.Ltmp8793:
	.loc	21 268 33 is_stmt 1
	cmpq	%rbx, %r11
	jae	.LBB46_323
	.loc	21 269 33
	cmpq	%r10, %r11
	jae	.LBB46_332
.Ltmp8794:
	.loc	21 263 29
	leal	(%r12,%r13), %eax
	movl	%eax, %ecx
	subl	1192(%r8), %ecx
	andl	%r15d, %ecx
	.loc	21 263 28 is_stmt 0
	leaq	2(,%rcx,8), %rcx
.Ltmp8795:
	.loc	21 266 33 is_stmt 1
	cmpq	%r10, %rcx
	jae	.LBB46_338
	.loc	21 267 33
	cmpq	%rbx, %rcx
	jae	.LBB46_347
.Ltmp8796:
	.loc	21 0 0 is_stmt 0
	subl	1256(%r8), %eax
	andl	%r15d, %eax
	leaq	2(,%rax,8), %r15
.Ltmp8797:
	.loc	21 268 33 is_stmt 1
	cmpq	%rbx, %r15
	jae	.LBB46_309
	.loc	21 269 33
	cmpq	%r10, %r15
	jae	.LBB46_319
.Ltmp8798:
	.loc	21 263 29
	leal	(%r12,%r13), %eax
	movl	%eax, %esi
	subl	1196(%r8), %esi
	andl	24(%rsp), %esi
	.loc	21 263 28 is_stmt 0
	leaq	3(,%rsi,8), %rsi
.Ltmp8799:
	.loc	21 266 33 is_stmt 1
	cmpq	%r10, %rsi
	jae	.LBB46_329
	.loc	21 267 33
	cmpq	%rbx, %rsi
	jae	.LBB46_336
.Ltmp8800:
	.loc	21 0 0 is_stmt 0
	subl	1260(%r8), %eax
	andl	24(%rsp), %eax
	leaq	3(,%rax,8), %r9
.Ltmp8801:
	.loc	21 268 33 is_stmt 1
	cmpq	%rbx, %r9
	jae	.LBB46_343
	.loc	21 0 33 is_stmt 0
	movq	%rsi, 128(%rsp)
	.loc	21 269 33 is_stmt 1
	cmpq	%r10, %r9
	jae	.LBB46_305
.Ltmp8802:
	.loc	21 263 29
	leal	(%r12,%r13), %eax
	movl	%eax, %esi
	subl	1200(%r8), %esi
	andl	24(%rsp), %esi
	.loc	21 263 28 is_stmt 0
	leaq	4(,%rsi,8), %r14
.Ltmp8803:
	.loc	21 266 33 is_stmt 1
	cmpq	%r10, %r14
	jae	.LBB46_315
	.loc	21 267 33
	cmpq	%rbx, %r14
	jae	.LBB46_325
.Ltmp8804:
	.loc	21 0 0 is_stmt 0
	subl	1264(%r8), %eax
	andl	24(%rsp), %eax
	leaq	4(,%rax,8), %rsi
.Ltmp8805:
	.loc	21 268 33 is_stmt 1
	cmpq	%rbx, %rsi
	jae	.LBB46_298
	.loc	21 0 33 is_stmt 0
	movq	%rcx, 176(%rsp)
	.loc	21 269 33 is_stmt 1
	cmpq	%r10, %rsi
	jae	.LBB46_295
.Ltmp8806:
	.loc	21 263 29
	leal	(%r12,%r13), %eax
	movl	%eax, %edi
	subl	1204(%r8), %edi
	andl	24(%rsp), %edi
	.loc	21 263 28 is_stmt 0
	leaq	5(,%rdi,8), %rcx
.Ltmp8807:
	.loc	21 266 33 is_stmt 1
	cmpq	%r10, %rcx
	jae	.LBB46_338
	.loc	21 0 33 is_stmt 0
	movq	%r11, 512(%rsp)
	.loc	21 267 33 is_stmt 1
	cmpq	%rbx, %rcx
	jae	.LBB46_347
.Ltmp8808:
	.loc	21 0 0 is_stmt 0
	subl	1268(%r8), %eax
	andl	24(%rsp), %eax
	leaq	5(,%rax,8), %rdi
.Ltmp8809:
	.loc	21 268 33 is_stmt 1
	cmpq	%rbx, %rdi
	jae	.LBB46_344
	.loc	21 269 33
	cmpq	%r10, %rdi
	jae	.LBB46_396
.Ltmp8810:
	.loc	21 0 33 is_stmt 0
	movq	%rdx, %r11
	movq	%r10, %rdx
	.loc	21 263 29 is_stmt 1
	leal	(%r12,%r13), %eax
	movl	%eax, %r10d
	subl	1208(%r8), %r10d
	andl	24(%rsp), %r10d
	.loc	21 263 28 is_stmt 0
	leaq	6(,%r10,8), %r13
.Ltmp8811:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r13
	jae	.LBB46_321
	.loc	21 267 33
	cmpq	%rbx, %r13
	jae	.LBB46_331
.Ltmp8812:
	.loc	21 0 0 is_stmt 0
	subl	1272(%r8), %eax
	andl	24(%rsp), %eax
	leaq	6(,%rax,8), %r8
.Ltmp8813:
	.loc	21 268 33 is_stmt 1
	cmpq	%rbx, %r8
	jae	.LBB46_337
	.loc	21 269 33
	cmpq	%rdx, %r8
	jae	.LBB46_345
.Ltmp8814:
	.loc	21 0 33 is_stmt 0
	movq	120(%rsp), %rax
	.loc	21 263 29 is_stmt 1
	addl	%r12d, %eax
	movl	%eax, %ebx
	movq	168(%rsp), %r10
	subl	1212(%r10), %ebx
	andl	24(%rsp), %ebx
	.loc	21 263 28 is_stmt 0
	leaq	7(,%rbx,8), %r12
.Ltmp8815:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r12
	jae	.LBB46_307
	.loc	21 0 33 is_stmt 0
	movq	104(%rsp), %rdx
	.loc	21 267 33 is_stmt 1
	cmpq	%rdx, %r12
	jae	.LBB46_317
.Ltmp8816:
	.loc	21 0 33 is_stmt 0
	movq	168(%rsp), %r10
	subl	1276(%r10), %eax
	andl	24(%rsp), %eax
	leaq	7(,%rax,8), %rax
.Ltmp8817:
	.loc	21 268 33 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB46_327
	.loc	21 269 33
	cmpq	16(%rsp), %rax
	jb	.LBB46_75
.Ltmp8818:
	.loc	21 0 33 is_stmt 0
	movq	%rax, %rdi
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_151:
.Ltmp8819:
	.loc	15 2584 13 is_stmt 1
	leal	(%rcx,%r13), %eax
.Ltmp8820:
	.loc	21 413 5
	movl	%eax, 2608(%r8)
	movl	500(%rsp), %edx
.Ltmp8821:
.LBB46_152:
	.loc	6 691 12
	cmpl	%edx, %r9d
	jbe	.LBB46_244
	.loc	6 692 25
	leaq	(,%rcx,8), %rdi
	movq	264(%rsp), %rsi
.Ltmp8822:
	.loc	25 580 12
	subq	%rdi, %rsi
	jb	.LBB46_290
.Ltmp8823:
	.loc	25 0 12 is_stmt 0
	movq	256(%rsp), %r10
.Ltmp8824:
	.loc	25 580 12
	subq	%rdi, %r10
	jb	.LBB46_291
.Ltmp8825:
	.loc	21 61 8 is_stmt 1
	cmpl	$1, 2572(%r8)
	jne	.LBB46_157
	.loc	21 0 8 is_stmt 0
	xorl	%r14d, %r14d
	.loc	21 61 8
	jmp	.LBB46_165
.LBB46_157:
.Ltmp8826:
	.loc	48 2494 21 is_stmt 1
	movl	1184(%r8), %eax
	movb	$2, %r14b
.Ltmp8827:
	.loc	8 1878 54
	cmpl	1248(%r8), %eax
.Ltmp8828:
	.loc	48 2494 21
	jne	.LBB46_165
	movl	1188(%r8), %eax
.Ltmp8829:
	.loc	8 1878 54
	cmpl	1252(%r8), %eax
.Ltmp8830:
	.loc	48 2494 21
	jne	.LBB46_165
	movl	1192(%r8), %eax
.Ltmp8831:
	.loc	8 1878 54
	cmpl	1256(%r8), %eax
.Ltmp8832:
	.loc	48 2494 21
	jne	.LBB46_165
	movl	1196(%r8), %eax
.Ltmp8833:
	.loc	8 1878 54
	cmpl	1260(%r8), %eax
.Ltmp8834:
	.loc	48 2494 21
	jne	.LBB46_165
	movl	1200(%r8), %eax
.Ltmp8835:
	.loc	8 1878 54
	cmpl	1264(%r8), %eax
.Ltmp8836:
	.loc	48 2494 21
	jne	.LBB46_165
	movl	1204(%r8), %eax
.Ltmp8837:
	.loc	8 1878 54
	cmpl	1268(%r8), %eax
.Ltmp8838:
	.loc	48 2494 21
	jne	.LBB46_165
	movl	1208(%r8), %eax
.Ltmp8839:
	.loc	8 1878 54
	cmpl	1272(%r8), %eax
.Ltmp8840:
	.loc	48 2494 21
	jne	.LBB46_165
	movl	1212(%r8), %eax
.Ltmp8841:
	.loc	8 1878 54
	cmpl	1276(%r8), %eax
	sete	%al
	movb	$2, %r14b
.Ltmp8842:
	.loc	48 2494 21
	subb	%al, %r14b
.Ltmp8843:
.LBB46_165:
	.loc	48 0 21 is_stmt 0
	movq	488(%rsp), %rax
	leaq	(%rax,%rdi,4), %rax
	movq	%rax, 992(%rsp)
	movq	480(%rsp), %rax
	leaq	(%rax,%rdi,4), %rax
	movq	%rax, 928(%rsp)
	subq	%rcx, %r9
	movq	%r9, 592(%rsp)
.Ltmp8844:
	.loc	6 736 31 is_stmt 1
	movq	1152(%r8), %rcx
	movq	1160(%r8), %rdx
	.loc	6 741 31
	movq	1216(%r8), %r12
	movq	1224(%r8), %rax
	movq	%rax, 24(%rsp)
	.loc	6 747 28
	movl	2612(%r8), %r11d
.Ltmp8845:
	.loc	21 353 16
	movl	2608(%r8), %r15d
	movq	%r10, 672(%rsp)
.Ltmp8846:
	.loc	11 900 12
	shrq	$3, %r10
	movq	%r10, 960(%rsp)
	movq	%rsi, 688(%rsp)
	shrq	$3, %rsi
	movq	%rsi, 736(%rsp)
	movl	%r15d, %eax
	subl	2616(%r8), %eax
	movq	%rax, 896(%rsp)
	xorl	%edi, %edi
	movzbl	%r14b, %eax
	movl	%eax, 704(%rsp)
	xorl	%r13d, %r13d
	movq	%rdx, 16(%rsp)
	movl	%r14d, 720(%rsp)
	movq	%rcx, 768(%rsp)
	movq	%r15, 96(%rsp)
	movq	%r12, 416(%rsp)
	movl	%r11d, 88(%rsp)
	jmp	.LBB46_168
.Ltmp8847:
	.loc	11 0 12 is_stmt 0
.Ltmp8848:
	.p2align	4
.LBB46_166:
	movq	768(%rsp), %rdx
	movq	%r12, %rdi
.Ltmp8849:
	vmovss	(%rdx,%r12,4), %xmm9
	movq	416(%rsp), %r12
	vmovss	(%r12,%rdi,4), %xmm0
	vmovaps	%xmm0, 336(%rsp)
	movq	544(%rsp), %rdi
	vmovss	(%r12,%rdi,4), %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vmovss	(%rdx,%rdi,4), %xmm0
	vmovaps	%xmm0, 544(%rsp)
	movq	224(%rsp), %rdi
	vmovss	(%rdx,%rdi,4), %xmm10
	vmovss	(%r12,%rdi,4), %xmm5
	movq	384(%rsp), %rdi
	vmovss	(%r12,%rdi,4), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	(%rdx,%rdi,4), %xmm0
	vmovaps	%xmm0, 384(%rsp)
	movq	192(%rsp), %rdi
	vmovss	(%rdx,%rdi,4), %xmm12
	vmovss	(%r12,%rdi,4), %xmm6
	movq	144(%rsp), %rdi
	vmovss	(%r12,%rdi,4), %xmm0
	vmovaps	%xmm0, 512(%rsp)
	vmovss	(%rdx,%rdi,4), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	movq	128(%rsp), %rdi
	vmovss	(%rdx,%rdi,4), %xmm1
	vmovss	(%r12,%rdi,4), %xmm8
	vmovss	(%r12,%rax,4), %xmm0
	vmovaps	%xmm0, 320(%rsp)
	vmovss	(%rdx,%rax,4), %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	(%rdx,%rbx,4), %xmm4
	vmovss	(%r12,%rbx,4), %xmm11
	vmovss	(%r12,%rsi,4), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	(%rdx,%rsi,4), %xmm0
	vmovaps	%xmm0, 128(%rsp)
	vmovss	(%rdx,%r14,4), %xmm2
	vmovss	(%r12,%r14,4), %xmm15
	vmovss	(%r12,%r11,4), %xmm0
	vmovaps	%xmm0, 272(%rsp)
	vmovss	(%rdx,%r11,4), %xmm0
	vmovaps	%xmm0, 352(%rsp)
	vmovss	(%rdx,%r10,4), %xmm13
	vmovss	(%r12,%r10,4), %xmm14
	vmovss	(%r12,%r8,4), %xmm0
	vmovaps	%xmm0, 800(%rsp)
	vmovss	(%rdx,%r8,4), %xmm0
	vmovaps	%xmm0, 288(%rsp)
.Ltmp8850:
	vmovss	(%rdx,%rcx,4), %xmm7
	vmovss	(%r12,%rcx,4), %xmm0
	.loc	21 268 33 is_stmt 1
	vmovss	(%r12,%r15,4), %xmm3
	vmovaps	%xmm3, 432(%rsp)
	.loc	21 269 33
	vmovss	(%rdx,%r15,4), %xmm3
	vmovaps	%xmm3, 448(%rsp)
	vmovaps	336(%rsp), %xmm3
	movq	%rdx, %rcx
	movl	88(%rsp), %r11d
.Ltmp8851:
.LBB46_167:
	.loc	1 551 14
	vinsertps	$16, %xmm5, %xmm3, %xmm3
	vinsertps	$32, %xmm6, %xmm3, %xmm3
	vinsertps	$48, %xmm8, %xmm3, %xmm3
	vinsertps	$16, %xmm15, %xmm11, %xmm5
	vinsertps	$32, %xmm14, %xmm5, %xmm5
	vinsertps	$48, %xmm0, %xmm5, %xmm0
.Ltmp8852:
	.loc	1 551 14 is_stmt 0
	vinsertps	$16, %xmm10, %xmm9, %xmm5
	vinsertps	$32, %xmm12, %xmm5, %xmm5
	vinsertps	$48, %xmm1, %xmm5, %xmm1
	vinsertps	$16, %xmm2, %xmm4, %xmm2
	vinsertps	$32, %xmm13, %xmm2, %xmm2
	vinsertps	$48, %xmm7, %xmm2, %xmm2
	vinsertf128	$1, %xmm2, %ymm1, %ymm1
.Ltmp8853:
	.loc	1 551 14
	vinsertf128	$1, %xmm0, %ymm3, %ymm0
.Ltmp8854:
	.loc	58 82 19 is_stmt 1
	vbroadcastss	.LCPI46_4(%rip), %ymm9
	vandps	%ymm1, %ymm9, %ymm1
.Ltmp8855:
	.loc	58 82 19 is_stmt 0
	vandps	%ymm0, %ymm9, %ymm0
	movq	168(%rsp), %r8
.Ltmp8856:
	.loc	21 459 23 is_stmt 1
	vmovaps	896(%r8), %ymm2
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp8857:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm2, %ymm2
.Ltmp8858:
	.loc	58 233 14
	vmaxps	%ymm0, %ymm1, %ymm3
.Ltmp8859:
	.loc	58 585 19
	vblendvps	%ymm2, %ymm3, %ymm1, %ymm2
	vbroadcastss	.LCPI46_5(%rip), %ymm12
.Ltmp8860:
	.loc	58 283 14
	vmulps	%ymm1, %ymm12, %ymm1
.Ltmp8861:
	.loc	58 283 14 is_stmt 0
	vmulps	%ymm0, %ymm12, %ymm0
.Ltmp8862:
	.loc	58 48 14 is_stmt 1
	vaddps	%ymm1, %ymm0, %ymm0
.Ltmp8863:
	.loc	21 461 9
	vmovaps	928(%r8), %ymm1
.Ltmp8864:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm1, %ymm1
.Ltmp8865:
	.loc	58 585 19
	vblendvps	%ymm1, %ymm0, %ymm2, %ymm0
	vbroadcastss	.LCPI46_6(%rip), %ymm13
.Ltmp8866:
	.loc	58 233 14
	vmaxps	%ymm13, %ymm0, %ymm0
	vbroadcastss	.LCPI46_7(%rip), %ymm14
.Ltmp8867:
	.loc	58 233 14 is_stmt 0
	vmaxps	%ymm14, %ymm0, %ymm0
.Ltmp8868:
	.loc	59 264 24 is_stmt 1
	vbroadcastsd	.LCPI46_8(%rip), %ymm1
.Ltmp8869:
	.loc	59 2315 14
	vbroadcastsd	.LCPI46_9(%rip), %ymm2
.Ltmp8870:
	.loc	59 264 24
	vandps	%ymm1, %ymm0, %ymm3
.Ltmp8871:
	.loc	59 2315 14
	vorps	%ymm2, %ymm3, %ymm3
	vbroadcastss	.LCPI46_3(%rip), %ymm15
.Ltmp8872:
	.loc	58 347 14
	vaddps	%ymm3, %ymm15, %ymm3
	vbroadcastss	.LCPI46_10(%rip), %ymm13
.Ltmp8873:
	.loc	58 283 14
	vmulps	%ymm3, %ymm13, %ymm4
	vbroadcastss	.LCPI46_11(%rip), %ymm14
.Ltmp8874:
	.loc	58 48 14
	vsubps	%ymm4, %ymm14, %ymm4
.Ltmp8875:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_12(%rip), %ymm5
.Ltmp8876:
	.loc	58 48 14
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8877:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_13(%rip), %ymm5
.Ltmp8878:
	.loc	58 48 14
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8879:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_14(%rip), %ymm5
.Ltmp8880:
	.loc	58 48 14
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8881:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_15(%rip), %ymm5
.Ltmp8882:
	.loc	58 48 14
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8883:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm3
.Ltmp8884:
	.loc	59 2315 24
	vpbroadcastd	.LCPI46_16(%rip), %ymm7
.Ltmp8885:
	.loc	59 3217 24
	vpsrld	$23, %ymm0, %ymm0
.Ltmp8886:
	.loc	59 2315 24
	vpor	%ymm7, %ymm0, %ymm0
	vbroadcastss	.LCPI46_17(%rip), %ymm13
.Ltmp8887:
	.loc	58 347 14
	vaddps	%ymm0, %ymm13, %ymm0
.Ltmp8888:
	.loc	58 48 14
	vaddps	%ymm3, %ymm0, %ymm0
	vbroadcastss	.LCPI46_18(%rip), %ymm14
.Ltmp8889:
	.loc	58 283 14
	vmulps	%ymm0, %ymm14, %ymm0
	vbroadcastss	.LCPI46_19(%rip), %ymm3
.Ltmp8890:
	.loc	58 257 14
	vminps	%ymm3, %ymm0, %ymm0
	vbroadcastss	.LCPI46_20(%rip), %ymm3
.Ltmp8891:
	.loc	58 233 14
	vmaxps	%ymm3, %ymm0, %ymm3
.Ltmp8892:
	.loc	21 478 20
	vmovaps	1792(%r8), %ymm0
.Ltmp8893:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm0, %ymm0
.Ltmp8894:
	.loc	21 451 21
	vmovaps	1280(%r8), %ymm4
.Ltmp8895:
	.loc	58 347 14
	vsubps	1664(%r8), %ymm4, %ymm5
.Ltmp8896:
	.loc	58 871 14
	vcmpge_oqps	%ymm5, %ymm3, %ymm5
.Ltmp8897:
	.loc	58 871 14 is_stmt 0
	vcmpge_oqps	%ymm4, %ymm3, %ymm6
.Ltmp8898:
	.loc	58 82 19 is_stmt 1
	vandnps	%ymm6, %ymm0, %ymm6
.Ltmp8899:
	.loc	58 82 19 is_stmt 0
	vandps	%ymm0, %ymm5, %ymm8
.Ltmp8900:
	.loc	58 117 19 is_stmt 1
	vorps	%ymm6, %ymm8, %ymm6
.Ltmp8901:
	.loc	21 486 47
	vmovaps	1824(%r8), %ymm8
.Ltmp8902:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm8, %ymm10
.Ltmp8903:
	.loc	58 82 19
	vandnps	%ymm10, %ymm5, %ymm5
	vandps	%ymm0, %ymm5, %ymm0
.Ltmp8904:
	.loc	58 347 14
	vaddps	%ymm15, %ymm8, %ymm5
.Ltmp8905:
	.loc	58 585 19
	vblendvps	%ymm0, %ymm5, %ymm8, %ymm5
.Ltmp8906:
	.loc	58 117 19
	vorps	%ymm6, %ymm0, %ymm8
.Ltmp8907:
	.loc	58 585 19
	vblendvps	%ymm6, 832(%r8), %ymm5, %ymm5
.Ltmp8908:
	.loc	21 508 65
	vmovaps	800(%r8), %ymm6
.Ltmp8909:
	.loc	21 514 49
	vmovaps	864(%r8), %ymm0
.Ltmp8910:
	.loc	21 489 5
	vmovaps	%ymm5, 1824(%r8)
.Ltmp8911:
	.loc	58 585 19
	vpcmpgtd	%ymm8, %ymm11, %ymm5
	vpbroadcastd	.LCPI46_2(%rip), %ymm13
	vpand	%ymm5, %ymm13, %ymm5
.Ltmp8912:
	.loc	21 498 5
	vmovdqa	%ymm5, 1792(%r8)
.Ltmp8913:
	.loc	58 347 14
	vaddps	1408(%r8), %ymm15, %ymm8
.Ltmp8914:
	.loc	58 347 14 is_stmt 0
	vsubps	%ymm4, %ymm3, %ymm3
.Ltmp8915:
	.loc	58 283 14 is_stmt 1
	vmulps	%ymm3, %ymm8, %ymm3
.Ltmp8916:
	.loc	58 713 19
	vbroadcastss	.LCPI46_21(%rip), %ymm10
	vxorps	1536(%r8), %ymm10, %ymm4
.Ltmp8917:
	.loc	58 233 14
	vmaxps	%ymm4, %ymm3, %ymm3
.Ltmp8918:
	.loc	58 257 14
	vminps	%ymm11, %ymm3, %ymm3
.Ltmp8919:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm5, %ymm4
.Ltmp8920:
	.loc	58 585 19
	vpcmpgtd	%ymm4, %ymm11, %ymm4
	vpandn	%ymm3, %ymm4, %ymm3
.Ltmp8921:
	.loc	21 508 36
	vmovaps	1856(%r8), %ymm4
.Ltmp8922:
	.loc	58 871 14
	vcmpgt_oqps	%ymm4, %ymm3, %ymm5
.Ltmp8923:
	.loc	58 585 19
	vblendvps	%ymm5, 768(%r8), %ymm6, %ymm5
.Ltmp8924:
	.loc	58 347 14
	vsubps	%ymm4, %ymm3, %ymm3
.Ltmp8925:
	.loc	58 283 14
	vmulps	%ymm5, %ymm3, %ymm3
.Ltmp8926:
	.loc	58 48 14
	vaddps	%ymm3, %ymm4, %ymm3
.Ltmp8927:
	.loc	58 82 19
	vandps	%ymm3, %ymm9, %ymm4
	vbroadcastss	.LCPI46_22(%rip), %ymm5
.Ltmp8928:
	.loc	58 871 14
	vcmplt_oqps	%ymm5, %ymm4, %ymm4
.Ltmp8929:
	.loc	58 82 19
	vandnps	%ymm3, %ymm4, %ymm3
	vbroadcastss	.LCPI46_23(%rip), %ymm4
.Ltmp8930:
	.loc	58 283 14
	vmulps	%ymm4, %ymm3, %ymm4
	vbroadcastss	.LCPI46_24(%rip), %ymm5
.Ltmp8931:
	.loc	58 233 14
	vmaxps	%ymm5, %ymm4, %ymm4
	vbroadcastss	.LCPI46_25(%rip), %ymm5
.Ltmp8932:
	.loc	58 257 14
	vminps	%ymm5, %ymm4, %ymm4
.Ltmp8933:
	.loc	58 471 14
	vroundps	$9, %ymm4, %ymm5
.Ltmp8934:
	.loc	58 347 14
	vsubps	%ymm5, %ymm4, %ymm4
	vbroadcastss	.LCPI46_26(%rip), %ymm6
.Ltmp8935:
	.loc	58 283 14
	vmulps	%ymm6, %ymm4, %ymm6
	vbroadcastss	.LCPI46_27(%rip), %ymm8
.Ltmp8936:
	.loc	58 48 14
	vaddps	%ymm6, %ymm8, %ymm6
.Ltmp8937:
	.loc	58 283 14
	vmulps	%ymm6, %ymm4, %ymm6
	vbroadcastss	.LCPI46_28(%rip), %ymm8
.Ltmp8938:
	.loc	58 48 14
	vaddps	%ymm6, %ymm8, %ymm6
.Ltmp8939:
	.loc	58 283 14
	vmulps	%ymm6, %ymm4, %ymm6
	vbroadcastss	.LCPI46_29(%rip), %ymm8
.Ltmp8940:
	.loc	58 48 14
	vaddps	%ymm6, %ymm8, %ymm6
.Ltmp8941:
	.loc	58 283 14
	vmulps	%ymm6, %ymm4, %ymm6
	vbroadcastss	.LCPI46_30(%rip), %ymm8
.Ltmp8942:
	.loc	58 48 14
	vaddps	%ymm6, %ymm8, %ymm6
.Ltmp8943:
	.loc	58 283 14
	vmulps	%ymm6, %ymm4, %ymm4
.Ltmp8944:
	.loc	58 48 14
	vaddps	%ymm4, %ymm13, %ymm4
	vbroadcastss	.LCPI46_31(%rip), %ymm6
.Ltmp8945:
	.loc	58 48 14 is_stmt 0
	vaddps	%ymm6, %ymm5, %ymm5
.Ltmp8946:
	.loc	59 2798 24 is_stmt 1
	vpslld	$23, %ymm5, %ymm5
.Ltmp8947:
	.loc	58 283 14
	vmulps	%ymm5, %ymm4, %ymm4
.Ltmp8948:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm0, %ymm0
.Ltmp8949:
	.loc	58 871 14 is_stmt 0
	vcmpeqps	%ymm3, %ymm11, %ymm5
.Ltmp8950:
	.loc	58 117 19 is_stmt 1
	vorps	%ymm5, %ymm0, %ymm0
	vmovaps	832(%rsp), %ymm5
.Ltmp8951:
	.loc	58 283 14
	vmulps	%ymm4, %ymm5, %ymm4
.Ltmp8952:
	.loc	58 585 19
	vblendvps	%ymm0, %ymm5, %ymm4, %ymm4
	vmovaps	544(%rsp), %xmm0
.Ltmp8953:
	.loc	1 551 14
	vinsertps	$16, 384(%rsp), %xmm0, %xmm0
	vinsertps	$32, 144(%rsp), %xmm0, %xmm0
	vinsertps	$48, 192(%rsp), %xmm0, %xmm0
	vmovaps	128(%rsp), %xmm5
	vinsertps	$16, 352(%rsp), %xmm5, %xmm5
	vinsertps	$32, 288(%rsp), %xmm5, %xmm5
	vinsertps	$48, 448(%rsp), %xmm5, %xmm5
	vmovaps	176(%rsp), %xmm6
.Ltmp8954:
	.loc	1 551 14 is_stmt 0
	vinsertps	$16, 224(%rsp), %xmm6, %xmm6
	vinsertps	$32, 512(%rsp), %xmm6, %xmm6
	vinsertps	$48, 320(%rsp), %xmm6, %xmm6
	vmovaps	304(%rsp), %xmm8
	vinsertps	$16, 272(%rsp), %xmm8, %xmm8
	vinsertps	$32, 800(%rsp), %xmm8, %xmm8
	vinsertps	$48, 432(%rsp), %xmm8, %xmm8
.Ltmp8955:
	.loc	21 510 5 is_stmt 1
	vmovaps	%ymm3, 1856(%r8)
.Ltmp8956:
	.loc	1 551 14
	vinsertf128	$1, %xmm8, %ymm6, %ymm3
.Ltmp8957:
	.loc	58 82 19
	vandps	%ymm3, %ymm9, %ymm3
.Ltmp8958:
	.loc	1 551 14
	vinsertf128	$1, %xmm5, %ymm0, %ymm0
.Ltmp8959:
	.loc	58 82 19
	vandps	%ymm0, %ymm9, %ymm0
.Ltmp8960:
	.loc	21 459 23
	vmovaps	1088(%r8), %ymm5
.Ltmp8961:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm5, %ymm5
.Ltmp8962:
	.loc	58 233 14
	vmaxps	%ymm0, %ymm3, %ymm6
.Ltmp8963:
	.loc	58 585 19
	vblendvps	%ymm5, %ymm6, %ymm3, %ymm5
.Ltmp8964:
	.loc	21 461 9
	vmovaps	1120(%r8), %ymm6
.Ltmp8965:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm6, %ymm6
.Ltmp8966:
	.loc	58 283 14
	vmulps	%ymm3, %ymm12, %ymm3
.Ltmp8967:
	.loc	58 283 14 is_stmt 0
	vmulps	%ymm0, %ymm12, %ymm0
.Ltmp8968:
	.loc	58 48 14 is_stmt 1
	vaddps	%ymm3, %ymm0, %ymm0
.Ltmp8969:
	.loc	58 585 19
	vblendvps	%ymm6, %ymm0, %ymm5, %ymm0
.Ltmp8970:
	.loc	58 233 14
	vbroadcastss	.LCPI46_6(%rip), %ymm3
	vmaxps	%ymm3, %ymm0, %ymm0
.Ltmp8971:
	.loc	58 233 14 is_stmt 0
	vbroadcastss	.LCPI46_7(%rip), %ymm3
	vmaxps	%ymm3, %ymm0, %ymm0
.Ltmp8972:
	.loc	59 264 24 is_stmt 1
	vandps	%ymm1, %ymm0, %ymm1
.Ltmp8973:
	.loc	59 2315 14
	vorps	%ymm2, %ymm1, %ymm1
.Ltmp8974:
	.loc	58 347 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8975:
	.loc	58 283 14
	vbroadcastss	.LCPI46_10(%rip), %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
.Ltmp8976:
	.loc	58 48 14
	vbroadcastss	.LCPI46_11(%rip), %ymm3
	vsubps	%ymm2, %ymm3, %ymm2
.Ltmp8977:
	.loc	58 283 14
	vmulps	%ymm2, %ymm1, %ymm2
.Ltmp8978:
	.loc	58 48 14
	vbroadcastss	.LCPI46_12(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
.Ltmp8979:
	.loc	58 283 14
	vmulps	%ymm2, %ymm1, %ymm2
.Ltmp8980:
	.loc	58 48 14
	vbroadcastss	.LCPI46_13(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
.Ltmp8981:
	.loc	58 283 14
	vmulps	%ymm2, %ymm1, %ymm2
.Ltmp8982:
	.loc	58 48 14
	vbroadcastss	.LCPI46_14(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
.Ltmp8983:
	.loc	58 283 14
	vmulps	%ymm2, %ymm1, %ymm2
.Ltmp8984:
	.loc	58 48 14
	vbroadcastss	.LCPI46_15(%rip), %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
.Ltmp8985:
	.loc	59 3217 24
	vpsrld	$23, %ymm0, %ymm0
.Ltmp8986:
	.loc	59 2315 24
	vpor	%ymm7, %ymm0, %ymm0
.Ltmp8987:
	.loc	58 347 14
	vbroadcastss	.LCPI46_17(%rip), %ymm3
	vaddps	%ymm3, %ymm0, %ymm0
.Ltmp8988:
	.loc	21 451 21
	vmovaps	1888(%r8), %ymm3
.Ltmp8989:
	.loc	58 283 14
	vmulps	%ymm2, %ymm1, %ymm1
.Ltmp8990:
	.loc	58 48 14
	vaddps	%ymm1, %ymm0, %ymm0
.Ltmp8991:
	.loc	58 283 14
	vmulps	%ymm0, %ymm14, %ymm0
.Ltmp8992:
	.loc	58 257 14
	vbroadcastss	.LCPI46_19(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
.Ltmp8993:
	.loc	58 233 14
	vbroadcastss	.LCPI46_20(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8994:
	.loc	21 478 20
	vmovaps	2400(%r8), %ymm1
.Ltmp8995:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm1, %ymm1
.Ltmp8996:
	.loc	58 871 14 is_stmt 0
	vcmpge_oqps	%ymm3, %ymm0, %ymm2
.Ltmp8997:
	.loc	58 347 14 is_stmt 1
	vsubps	2272(%r8), %ymm3, %ymm5
.Ltmp8998:
	.loc	58 871 14
	vcmpge_oqps	%ymm5, %ymm0, %ymm5
.Ltmp8999:
	.loc	58 82 19
	vandnps	%ymm2, %ymm1, %ymm2
.Ltmp9000:
	.loc	21 486 47
	vmovaps	2432(%r8), %ymm6
.Ltmp9001:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm6, %ymm7
.Ltmp9002:
	.loc	58 82 19
	vandnps	%ymm7, %ymm5, %ymm7
.Ltmp9003:
	.loc	58 82 19 is_stmt 0
	vandps	%ymm1, %ymm5, %ymm5
.Ltmp9004:
	.loc	58 82 19
	vandps	%ymm1, %ymm7, %ymm1
.Ltmp9005:
	.loc	58 347 14 is_stmt 1
	vaddps	%ymm6, %ymm15, %ymm7
.Ltmp9006:
	.loc	58 585 19
	vblendvps	%ymm1, %ymm7, %ymm6, %ymm6
.Ltmp9007:
	.loc	58 117 19
	vorps	%ymm2, %ymm5, %ymm2
.Ltmp9008:
	.loc	58 585 19
	vblendvps	%ymm2, 1024(%r8), %ymm6, %ymm5
.Ltmp9009:
	.loc	58 117 19
	vorps	%ymm2, %ymm1, %ymm1
.Ltmp9010:
	.loc	58 347 14
	vaddps	2016(%r8), %ymm15, %ymm2
.Ltmp9011:
	.loc	58 713 19
	vxorps	2144(%r8), %ymm10, %ymm6
.Ltmp9012:
	.loc	21 489 5
	vmovaps	%ymm5, 2432(%r8)
.Ltmp9013:
	.loc	58 347 14
	vsubps	%ymm3, %ymm0, %ymm0
.Ltmp9014:
	.loc	58 585 19
	vpcmpgtd	%ymm1, %ymm11, %ymm1
	vpand	%ymm1, %ymm13, %ymm1
.Ltmp9015:
	.loc	21 498 5
	vmovdqa	%ymm1, 2400(%r8)
.Ltmp9016:
	.loc	58 283 14
	vmulps	%ymm0, %ymm2, %ymm0
.Ltmp9017:
	.loc	21 508 36
	vmovaps	2464(%r8), %ymm2
.Ltmp9018:
	.loc	58 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp9019:
	.loc	58 257 14
	vminps	%ymm11, %ymm0, %ymm0
.Ltmp9020:
	.loc	58 871 14
	vcmpgt_oqps	%ymm11, %ymm1, %ymm1
.Ltmp9021:
	.loc	58 585 19
	vpcmpgtd	%ymm1, %ymm11, %ymm1
	vpandn	%ymm0, %ymm1, %ymm0
.Ltmp9022:
	.loc	58 871 14
	vcmpgt_oqps	%ymm2, %ymm0, %ymm1
.Ltmp9023:
	.loc	21 508 65
	vmovaps	992(%r8), %ymm3
.Ltmp9024:
	.loc	58 585 19
	vblendvps	%ymm1, 960(%r8), %ymm3, %ymm1
.Ltmp9025:
	.loc	58 347 14
	vsubps	%ymm2, %ymm0, %ymm0
.Ltmp9026:
	.loc	58 283 14
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp9027:
	.loc	58 48 14
	vaddps	%ymm0, %ymm2, %ymm0
.Ltmp9028:
	.loc	58 82 19
	vandps	%ymm0, %ymm9, %ymm1
.Ltmp9029:
	.loc	58 871 14
	vbroadcastss	.LCPI46_22(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp9030:
	.loc	58 82 19
	vandnps	%ymm0, %ymm1, %ymm0
.Ltmp9031:
	.loc	58 283 14
	vbroadcastss	.LCPI46_23(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm1
.Ltmp9032:
	.loc	58 233 14
	vbroadcastss	.LCPI46_24(%rip), %ymm2
	vmaxps	%ymm2, %ymm1, %ymm1
.Ltmp9033:
	.loc	58 257 14
	vbroadcastss	.LCPI46_25(%rip), %ymm2
	vminps	%ymm2, %ymm1, %ymm1
.Ltmp9034:
	.loc	58 471 14
	vroundps	$9, %ymm1, %ymm2
.Ltmp9035:
	.loc	58 347 14
	vsubps	%ymm2, %ymm1, %ymm1
.Ltmp9036:
	.loc	58 283 14
	vbroadcastss	.LCPI46_26(%rip), %ymm3
	vmulps	%ymm3, %ymm1, %ymm3
.Ltmp9037:
	.loc	58 48 14
	vbroadcastss	.LCPI46_27(%rip), %ymm5
	vaddps	%ymm5, %ymm3, %ymm3
.Ltmp9038:
	.loc	58 283 14
	vmulps	%ymm3, %ymm1, %ymm3
.Ltmp9039:
	.loc	58 48 14
	vbroadcastss	.LCPI46_28(%rip), %ymm5
	vaddps	%ymm5, %ymm3, %ymm3
.Ltmp9040:
	.loc	58 283 14
	vmulps	%ymm3, %ymm1, %ymm3
.Ltmp9041:
	.loc	58 48 14
	vbroadcastss	.LCPI46_29(%rip), %ymm5
	vaddps	%ymm5, %ymm3, %ymm3
.Ltmp9042:
	.loc	58 283 14
	vmulps	%ymm3, %ymm1, %ymm3
.Ltmp9043:
	.loc	58 48 14
	vbroadcastss	.LCPI46_30(%rip), %ymm5
	vaddps	%ymm5, %ymm3, %ymm3
.Ltmp9044:
	.loc	58 283 14
	vmulps	%ymm3, %ymm1, %ymm1
.Ltmp9045:
	.loc	21 514 49
	vmovaps	1056(%r8), %ymm3
.Ltmp9046:
	.loc	58 48 14
	vaddps	%ymm1, %ymm13, %ymm1
.Ltmp9047:
	.loc	58 48 14 is_stmt 0
	vbroadcastss	.LCPI46_31(%rip), %ymm5
	vaddps	%ymm5, %ymm2, %ymm2
.Ltmp9048:
	.loc	59 2798 24 is_stmt 1
	vpslld	$23, %ymm2, %ymm2
.Ltmp9049:
	.loc	58 283 14
	vmulps	%ymm2, %ymm1, %ymm1
.Ltmp9050:
	.loc	58 871 14
	vcmpeqps	%ymm0, %ymm11, %ymm2
.Ltmp9051:
	.loc	58 871 14 is_stmt 0
	vcmpgt_oqps	%ymm11, %ymm3, %ymm3
.Ltmp9052:
	.loc	58 117 19 is_stmt 1
	vorps	%ymm2, %ymm3, %ymm2
	vmovaps	864(%rsp), %ymm3
.Ltmp9053:
	.loc	58 283 14
	vmulps	%ymm1, %ymm3, %ymm1
.Ltmp9054:
	.loc	58 585 19
	vblendvps	%ymm2, %ymm3, %ymm1, %ymm1
.Ltmp9055:
	.loc	21 510 5
	vmovaps	%ymm0, 2464(%r8)
	movq	216(%rsp), %rax
.Ltmp9056:
	.loc	1 551 14
	vmovups	%ymm4, (%rax)
	movq	104(%rsp), %rax
.Ltmp9057:
	.loc	1 551 14 is_stmt 0
	vmovups	%ymm1, (%rax)
.Ltmp9058:
	.loc	21 0 0
	incq	%r13
	movq	120(%rsp), %rdi
.Ltmp9059:
	.loc	8 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r13, 592(%rsp)
	movq	16(%rsp), %rdx
	movl	720(%rsp), %r14d
	movq	96(%rsp), %r15
.Ltmp9060:
	.loc	11 900 12
	je	.LBB46_243
.Ltmp9061:
.LBB46_168:
	.loc	15 1050 16
	cmpq	%r13, 736(%rsp)
	je	.LBB46_275
.Ltmp9062:
	.loc	15 0 16 is_stmt 0
	movl	%r11d, %eax
	leal	(%r15,%r13), %r11d
	andl	%eax, %r11d
	shlq	$3, %r11
.Ltmp9063:
	.loc	21 362 77 is_stmt 1
	leaq	8(%r11), %rsi
.Ltmp9064:
	.loc	25 451 16
	cmpq	%rdx, %rsi
	ja	.LBB46_282
.Ltmp9065:
	.loc	25 0 16 is_stmt 0
	movq	992(%rsp), %rax
	.loc	21 362 0 is_stmt 1
	leaq	(%rax,%rdi,4), %r10
.Ltmp9066:
	.loc	1 551 14
	vmovups	(%r10), %ymm0
	vmovups	%ymm0, (%rcx,%r11,4)
.Ltmp9067:
	.loc	25 438 16
	cmpq	%r13, 960(%rsp)
	je	.LBB46_283
.Ltmp9068:
	.loc	25 0 16 is_stmt 0
	movq	24(%rsp), %r9
.Ltmp9069:
	.loc	25 451 16 is_stmt 1
	cmpq	%r9, %rsi
	ja	.LBB46_284
.Ltmp9070:
	.loc	25 0 16 is_stmt 0
	movq	928(%rsp), %rax
	.loc	21 363 0 is_stmt 1
	leaq	(%rax,%rdi,4), %rax
	movq	%rax, 104(%rsp)
.Ltmp9071:
	.loc	1 551 14
	vmovups	(%rax), %ymm0
	vmovups	%ymm0, (%r12,%r11,4)
	movq	896(%rsp), %rax
.Ltmp9072:
	.loc	21 370 21
	leal	(%rax,%r13), %ebx
	movl	88(%rsp), %r11d
	andl	%r11d, %ebx
.Ltmp9073:
	.loc	21 371 54
	leaq	8(,%rbx,8), %rsi
.Ltmp9074:
	.loc	21 370 20
	shlq	$3, %rbx
.Ltmp9075:
	.loc	25 438 16
	cmpq	%rdx, %rsi
	ja	.LBB46_285
.Ltmp9076:
	.loc	25 438 16 is_stmt 0
	cmpq	%r9, %rsi
	ja	.LBB46_286
.Ltmp9077:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rcx,%rbx,4), %ymm1
.Ltmp9078:
	.loc	21 0 0 is_stmt 0
	leal	(%r15,%r13), %esi
	movl	%esi, %eax
	subl	1184(%r8), %eax
.Ltmp9079:
	.loc	1 551 14
	vmovdqu	(%r12,%rbx,4), %ymm0
.Ltmp9080:
	.loc	21 0 0
	andl	%r11d, %eax
	subl	1248(%r8), %esi
	shlq	$3, %rax
	andl	%r11d, %esi
	shlq	$3, %rsi
	.loc	21 229 5 is_stmt 1
	testb	%r14b, %r14b
	movq	%rdi, 120(%rsp)
	movq	%r10, 216(%rsp)
	vmovdqa	%ymm0, 864(%rsp)
	vmovaps	%ymm1, 832(%rsp)
	movq	%rsi, 544(%rsp)
	je	.LBB46_193
	cmpl	$1, 704(%rsp)
	jne	.LBB46_210
	.loc	21 0 0 is_stmt 0
	cmpq	%rdx, %rax
.Ltmp9081:
	.loc	21 251 32 is_stmt 1
	jae	.LBB46_294
	.loc	21 0 32 is_stmt 0
	movq	%rax, 384(%rsp)
.Ltmp9082:
	.loc	21 252 33 is_stmt 1
	cmpq	%r9, %rsi
	jae	.LBB46_303
.Ltmp9083:
	.loc	21 248 29
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1188(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 248 28 is_stmt 0
	leaq	1(,%rcx,8), %r14
.Ltmp9084:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %r14
	jae	.LBB46_366
.Ltmp9085:
	.loc	21 0 0 is_stmt 0
	subl	1252(%r8), %eax
	andl	%r11d, %eax
	leaq	1(,%rax,8), %r12
.Ltmp9086:
	.loc	21 252 33 is_stmt 1
	cmpq	%r9, %r12
	jae	.LBB46_376
.Ltmp9087:
	.loc	21 248 29
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1192(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 248 28 is_stmt 0
	leaq	2(,%rcx,8), %rbx
.Ltmp9088:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %rbx
	jae	.LBB46_384
.Ltmp9089:
	.loc	21 0 0 is_stmt 0
	subl	1256(%r8), %eax
	andl	%r11d, %eax
	leaq	2(,%rax,8), %rsi
.Ltmp9090:
	.loc	21 252 33 is_stmt 1
	cmpq	%r9, %rsi
	jae	.LBB46_303
.Ltmp9091:
	.loc	21 248 29
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1196(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 248 28 is_stmt 0
	leaq	3(,%rcx,8), %rcx
.Ltmp9092:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_360
.Ltmp9093:
	.loc	21 0 0 is_stmt 0
	subl	1260(%r8), %eax
	andl	%r11d, %eax
	leaq	3(,%rax,8), %r10
	movq	%r10, 144(%rsp)
.Ltmp9094:
	.loc	21 252 33 is_stmt 1
	cmpq	%r9, %r10
	jae	.LBB46_371
.Ltmp9095:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, 192(%rsp)
	.loc	21 248 29 is_stmt 1
	leal	(%r15,%r13), %r10d
	movl	%r10d, %eax
	subl	1200(%r8), %eax
	andl	%r11d, %eax
	.loc	21 248 28 is_stmt 0
	leaq	4(,%rax,8), %rax
.Ltmp9096:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB46_294
.Ltmp9097:
	.loc	21 0 0 is_stmt 0
	subl	1264(%r8), %r10d
	andl	%r11d, %r10d
	leaq	4(,%r10,8), %r10
.Ltmp9098:
	.loc	21 252 33 is_stmt 1
	cmpq	%r9, %r10
	jae	.LBB46_397
.Ltmp9099:
	.loc	21 0 33 is_stmt 0
	movq	%rcx, 128(%rsp)
	movq	%r12, 176(%rsp)
	.loc	21 248 29 is_stmt 1
	leal	(%r15,%r13), %edi
	movl	%edi, %r9d
	subl	1204(%r8), %r9d
	andl	%r11d, %r9d
	.loc	21 248 28 is_stmt 0
	leaq	5(,%r9,8), %r9
.Ltmp9100:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %r9
	jae	.LBB46_322
.Ltmp9101:
	.loc	21 0 0 is_stmt 0
	subl	1268(%r8), %edi
	andl	%r11d, %edi
	leaq	5(,%rdi,8), %r12
.Ltmp9102:
	.loc	21 252 33 is_stmt 1
	cmpq	24(%rsp), %r12
	jae	.LBB46_376
.Ltmp9103:
	.loc	21 0 33 is_stmt 0
	movq	%r14, %rcx
	movq	%r9, 224(%rsp)
	.loc	21 248 29 is_stmt 1
	leal	(%r15,%r13), %edi
	movl	%edi, %r9d
	subl	1208(%r8), %r9d
	andl	%r11d, %r9d
	.loc	21 248 28 is_stmt 0
	leaq	6(,%r9,8), %r14
.Ltmp9104:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %r14
	jae	.LBB46_366
.Ltmp9105:
	.loc	21 0 0 is_stmt 0
	subl	1272(%r8), %edi
	andl	%r11d, %edi
	leaq	6(,%rdi,8), %r15
.Ltmp9106:
	.loc	21 252 33 is_stmt 1
	cmpq	24(%rsp), %r15
	jae	.LBB46_383
.Ltmp9107:
	.loc	21 0 33 is_stmt 0
	movq	96(%rsp), %rdx
	.loc	21 248 29 is_stmt 1
	leal	(%rdx,%r13), %r9d
	movl	%r9d, %edi
	subl	1212(%r8), %edi
	andl	%r11d, %edi
	.loc	21 248 28 is_stmt 0
	leaq	7(,%rdi,8), %rdi
.Ltmp9108:
	.loc	21 251 32 is_stmt 1
	cmpq	16(%rsp), %rdi
	jae	.LBB46_385
.Ltmp9109:
	.loc	21 0 0 is_stmt 0
	subl	1276(%r8), %r9d
	andl	%r11d, %r9d
	leaq	7(,%r9,8), %r9
.Ltmp9110:
	.loc	21 252 33 is_stmt 1
	cmpq	24(%rsp), %r9
	jae	.LBB46_401
.Ltmp9111:
	.loc	21 0 33 is_stmt 0
	movq	%rax, %rbx
	movq	%rsi, %rax
	movq	768(%rsp), %rdx
	movq	384(%rsp), %rsi
	vmovss	(%rdx,%rsi,4), %xmm9
	movq	416(%rsp), %r8
	movq	544(%rsp), %rsi
	vmovss	(%r8,%rsi,4), %xmm3
	vmovss	(%rdx,%rcx,4), %xmm10
	movq	176(%rsp), %rcx
	vmovss	(%r8,%rcx,4), %xmm5
	movq	192(%rsp), %rcx
	vmovss	(%rdx,%rcx,4), %xmm12
	vmovss	(%r8,%rax,4), %xmm6
	movq	128(%rsp), %rax
	vmovss	(%rdx,%rax,4), %xmm1
	movq	144(%rsp), %rax
	vmovss	(%r8,%rax,4), %xmm8
	vmovss	(%rdx,%rbx,4), %xmm4
	vmovss	(%r8,%r10,4), %xmm11
	movq	224(%rsp), %rax
	vmovss	(%rdx,%rax,4), %xmm2
	vmovss	(%r8,%r12,4), %xmm15
	movq	%r8, %r12
	vmovss	(%rdx,%r14,4), %xmm13
	vmovss	(%r8,%r15,4), %xmm14
	movq	%rdx, %rcx
.Ltmp9112:
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm7
.Ltmp9113:
	.loc	21 252 33
	vmovss	(%r8,%r9,4), %xmm0
	vmovaps	%xmm0, 432(%rsp)
	vmovaps	%xmm14, 800(%rsp)
	vmovaps	%xmm15, 272(%rsp)
	vmovaps	%xmm11, 304(%rsp)
	vmovaps	%xmm8, 320(%rsp)
	vmovaps	%xmm6, 512(%rsp)
	vmovaps	%xmm5, 224(%rsp)
	vmovaps	%xmm3, 176(%rsp)
	vmovaps	%xmm7, 448(%rsp)
	vmovaps	%xmm13, 288(%rsp)
	vmovaps	%xmm2, 352(%rsp)
	vmovaps	%xmm4, 128(%rsp)
	vmovaps	%xmm1, 192(%rsp)
	vmovaps	%xmm12, 144(%rsp)
	vmovaps	%xmm10, 384(%rsp)
	vmovaps	%xmm9, 544(%rsp)
	movq	248(%rsp), %r9
.Ltmp9114:
	.loc	17 149 21
	jmp	.LBB46_167
.Ltmp9115:
	.loc	17 0 21 is_stmt 0
.Ltmp9116:
	.p2align	4
.LBB46_193:
	cmpq	%rdx, %rax
.Ltmp9117:
	.loc	21 236 32 is_stmt 1
	jae	.LBB46_296
	.loc	21 0 32 is_stmt 0
	movq	%rax, 384(%rsp)
.Ltmp9118:
	.loc	21 237 33 is_stmt 1
	cmpq	%r9, %rsi
	jae	.LBB46_301
.Ltmp9119:
	.loc	21 233 29
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1188(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 233 28 is_stmt 0
	leaq	1(,%rcx,8), %r14
.Ltmp9120:
	.loc	21 236 32 is_stmt 1
	cmpq	%rdx, %r14
	jae	.LBB46_339
.Ltmp9121:
	.loc	21 0 0 is_stmt 0
	subl	1252(%r8), %eax
	andl	%r11d, %eax
	leaq	1(,%rax,8), %r12
.Ltmp9122:
	.loc	21 237 33 is_stmt 1
	cmpq	%r9, %r12
	jae	.LBB46_379
.Ltmp9123:
	.loc	21 233 29
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1192(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 233 28 is_stmt 0
	leaq	2(,%rcx,8), %rbx
.Ltmp9124:
	.loc	21 236 32 is_stmt 1
	cmpq	%rdx, %rbx
	jae	.LBB46_388
.Ltmp9125:
	.loc	21 0 0 is_stmt 0
	subl	1256(%r8), %eax
	andl	%r11d, %eax
	leaq	2(,%rax,8), %rsi
.Ltmp9126:
	.loc	21 237 33 is_stmt 1
	cmpq	%r9, %rsi
	jae	.LBB46_301
.Ltmp9127:
	.loc	21 233 29
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1196(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 233 28 is_stmt 0
	leaq	3(,%rcx,8), %rcx
.Ltmp9128:
	.loc	21 236 32 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_330
.Ltmp9129:
	.loc	21 0 32 is_stmt 0
	movq	%rbx, 144(%rsp)
	subl	1260(%r8), %eax
	andl	%r11d, %eax
	leaq	3(,%rax,8), %rbx
.Ltmp9130:
	.loc	21 237 33 is_stmt 1
	cmpq	%r9, %rbx
	jae	.LBB46_373
.Ltmp9131:
	.loc	21 233 29
	leal	(%r15,%r13), %eax
	movl	%eax, %r10d
	subl	1200(%r8), %r10d
	andl	%r11d, %r10d
	.loc	21 233 28 is_stmt 0
	leaq	4(,%r10,8), %r15
.Ltmp9132:
	.loc	21 236 32 is_stmt 1
	cmpq	%rdx, %r15
	jae	.LBB46_381
.Ltmp9133:
	.loc	21 0 0 is_stmt 0
	subl	1264(%r8), %eax
	andl	%r11d, %eax
	leaq	4(,%rax,8), %r10
.Ltmp9134:
	.loc	21 237 33 is_stmt 1
	cmpq	%r9, %r10
	jae	.LBB46_391
.Ltmp9135:
	.loc	21 0 33 is_stmt 0
	movq	%rcx, 176(%rsp)
	movq	%r12, 192(%rsp)
	movq	96(%rsp), %rax
	.loc	21 233 29 is_stmt 1
	addl	%r13d, %eax
	movl	%eax, %edi
	subl	1204(%r8), %edi
	andl	%r11d, %edi
	.loc	21 233 28 is_stmt 0
	leaq	5(,%rdi,8), %rdi
.Ltmp9136:
	.loc	21 236 32 is_stmt 1
	cmpq	16(%rsp), %rdi
	jae	.LBB46_389
.Ltmp9137:
	.loc	21 0 0 is_stmt 0
	subl	1268(%r8), %eax
	andl	%r11d, %eax
	leaq	5(,%rax,8), %r12
.Ltmp9138:
	.loc	21 237 33 is_stmt 1
	cmpq	%r9, %r12
	jae	.LBB46_379
.Ltmp9139:
	.loc	21 0 33 is_stmt 0
	movq	%r14, %rcx
	movq	%rdi, 128(%rsp)
	movq	96(%rsp), %rax
	movq	%r13, 336(%rsp)
	.loc	21 233 29 is_stmt 1
	addl	%r13d, %eax
	movl	%eax, %edi
	subl	1208(%r8), %edi
	andl	%r11d, %edi
	.loc	21 233 28 is_stmt 0
	leaq	6(,%rdi,8), %r14
.Ltmp9140:
	.loc	21 236 32 is_stmt 1
	cmpq	16(%rsp), %r14
	jae	.LBB46_339
.Ltmp9141:
	.loc	21 0 0 is_stmt 0
	subl	1272(%r8), %eax
	andl	%r11d, %eax
	leaq	6(,%rax,8), %rax
.Ltmp9142:
	.loc	21 237 33 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB46_368
.Ltmp9143:
	.loc	21 0 33 is_stmt 0
	movq	96(%rsp), %rdx
	movq	336(%rsp), %rdi
	.loc	21 233 29 is_stmt 1
	leal	(%rdx,%rdi), %r9d
	movl	%r9d, %edi
	subl	1212(%r8), %edi
	andl	%r11d, %edi
	.loc	21 233 28 is_stmt 0
	leaq	7(,%rdi,8), %rdi
.Ltmp9144:
	.loc	21 236 32 is_stmt 1
	cmpq	16(%rsp), %rdi
	jae	.LBB46_389
.Ltmp9145:
	.loc	21 0 0 is_stmt 0
	subl	1276(%r8), %r9d
	andl	%r11d, %r9d
	leaq	7(,%r9,8), %r9
.Ltmp9146:
	.loc	21 237 33 is_stmt 1
	cmpq	24(%rsp), %r9
	jae	.LBB46_394
.Ltmp9147:
	.loc	21 0 33 is_stmt 0
	movq	%r15, %r13
	movq	%rbx, %r15
	movq	%rsi, %rbx
	movq	768(%rsp), %rdx
	movq	384(%rsp), %rsi
	vmovss	(%rdx,%rsi,4), %xmm13
	movq	416(%rsp), %r8
	movq	544(%rsp), %rsi
	vmovss	(%r8,%rsi,4), %xmm1
	vmovss	(%rdx,%rcx,4), %xmm5
	movq	192(%rsp), %rcx
	vmovss	(%r8,%rcx,4), %xmm2
	movq	144(%rsp), %rcx
	vmovss	(%rdx,%rcx,4), %xmm6
	vmovss	(%r8,%rbx,4), %xmm4
	movq	176(%rsp), %rcx
	vmovss	(%rdx,%rcx,4), %xmm8
	vmovss	(%r8,%r15,4), %xmm7
	vmovss	(%rdx,%r13,4), %xmm11
	vmovss	(%r8,%r10,4), %xmm9
	movq	%rdx, %rcx
	movq	128(%rsp), %rdx
	vmovss	(%rcx,%rdx,4), %xmm15
	vmovss	(%r8,%r12,4), %xmm10
	movq	%r8, %r12
	vmovss	(%rcx,%r14,4), %xmm14
	vmovss	(%r8,%rax,4), %xmm12
.Ltmp9148:
	.loc	21 236 32 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm0
.Ltmp9149:
	.loc	21 237 33
	vmovss	(%r8,%r9,4), %xmm3
	vmovaps	%xmm3, 448(%rsp)
	vmovaps	%xmm3, 432(%rsp)
	vmovaps	%xmm13, %xmm3
	vmovaps	%xmm12, 288(%rsp)
	vmovaps	%xmm12, 800(%rsp)
	vmovaps	%xmm10, 352(%rsp)
	vmovaps	%xmm10, 272(%rsp)
	vmovaps	%xmm9, 128(%rsp)
	vmovaps	%xmm9, 304(%rsp)
	vmovaps	%xmm7, 192(%rsp)
	vmovaps	%xmm7, 320(%rsp)
	vmovaps	%xmm4, 144(%rsp)
	vmovaps	%xmm4, 512(%rsp)
	vmovaps	%xmm2, 384(%rsp)
	vmovaps	%xmm2, 224(%rsp)
	vmovaps	%xmm1, 544(%rsp)
	vmovaps	%xmm1, 176(%rsp)
	vmovaps	%xmm0, %xmm7
	vmovaps	%xmm14, %xmm13
	vmovaps	%xmm15, %xmm2
	vmovaps	%xmm11, %xmm4
	vmovaps	%xmm8, %xmm1
	vmovaps	%xmm6, %xmm12
	vmovaps	%xmm5, %xmm10
	vmovaps	%xmm3, %xmm9
	movq	248(%rsp), %r9
	movq	336(%rsp), %r13
.Ltmp9150:
	.loc	17 149 21
	jmp	.LBB46_167
.Ltmp9151:
	.loc	17 0 21 is_stmt 0
.Ltmp9152:
	.p2align	4
.LBB46_210:
	cmpq	%rdx, %rax
.Ltmp9153:
	.loc	21 266 33 is_stmt 1
	jae	.LBB46_300
	.loc	21 267 33
	cmpq	%r9, %rax
	jae	.LBB46_352
	.loc	21 268 33
	cmpq	%r9, %rsi
	jae	.LBB46_302
	.loc	21 269 33
	cmpq	%rdx, %rsi
	jae	.LBB46_295
.Ltmp9154:
	.loc	21 0 33 is_stmt 0
	movq	%rax, %r12
	.loc	21 263 29 is_stmt 1
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1188(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 263 28 is_stmt 0
	leaq	1(,%rcx,8), %r14
.Ltmp9155:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r14
	jae	.LBB46_315
	.loc	21 267 33
	cmpq	%r9, %r14
	jae	.LBB46_356
.Ltmp9156:
	.loc	21 0 0 is_stmt 0
	subl	1252(%r8), %eax
	andl	%r11d, %eax
	leaq	1(,%rax,8), %rbx
.Ltmp9157:
	.loc	21 268 33 is_stmt 1
	cmpq	%r9, %rbx
	jae	.LBB46_364
	.loc	21 269 33
	cmpq	%rdx, %rbx
	jae	.LBB46_375
.Ltmp9158:
	.loc	21 263 29
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1192(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 263 28 is_stmt 0
	leaq	2(,%rcx,8), %rcx
.Ltmp9159:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_338
	.loc	21 267 33
	cmpq	%r9, %rcx
	jae	.LBB46_392
.Ltmp9160:
	.loc	21 0 0 is_stmt 0
	subl	1256(%r8), %eax
	andl	%r11d, %eax
	leaq	2(,%rax,8), %rax
.Ltmp9161:
	.loc	21 268 33 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB46_402
	.loc	21 0 33 is_stmt 0
	movq	%rcx, 192(%rsp)
	movq	%rax, 144(%rsp)
	.loc	21 269 33 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB46_359
.Ltmp9162:
	.loc	21 263 29
	leal	(%r15,%r13), %eax
	movl	%eax, %ecx
	subl	1196(%r8), %ecx
	andl	%r11d, %ecx
	.loc	21 263 28 is_stmt 0
	leaq	3(,%rcx,8), %rcx
.Ltmp9163:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_338
	.loc	21 267 33
	cmpq	%r9, %rcx
	jae	.LBB46_392
.Ltmp9164:
	.loc	21 0 0 is_stmt 0
	subl	1260(%r8), %eax
	andl	%r11d, %eax
	leaq	3(,%rax,8), %rax
.Ltmp9165:
	.loc	21 268 33 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB46_402
	.loc	21 0 33 is_stmt 0
	movq	%rcx, 128(%rsp)
	movq	%rbx, 384(%rsp)
	.loc	21 269 33 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB46_395
.Ltmp9166:
	.loc	21 263 29
	leal	(%r15,%r13), %ecx
	movl	%ecx, %esi
	subl	1200(%r8), %esi
	andl	%r11d, %esi
	.loc	21 263 28 is_stmt 0
	leaq	4(,%rsi,8), %rbx
.Ltmp9167:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %rbx
	jae	.LBB46_354
	.loc	21 267 33
	cmpq	%r9, %rbx
	jae	.LBB46_362
.Ltmp9168:
	.loc	21 0 0 is_stmt 0
	subl	1264(%r8), %ecx
	andl	%r11d, %ecx
	leaq	4(,%rcx,8), %rsi
.Ltmp9169:
	.loc	21 268 33 is_stmt 1
	cmpq	%r9, %rsi
	jae	.LBB46_302
	.loc	21 0 33 is_stmt 0
	movq	%r14, 224(%rsp)
	.loc	21 269 33 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB46_295
.Ltmp9170:
	.loc	21 263 29
	leal	(%r15,%r13), %ecx
	movl	%ecx, %r10d
	subl	1204(%r8), %r10d
	andl	%r11d, %r10d
	.loc	21 263 28 is_stmt 0
	leaq	5(,%r10,8), %r14
.Ltmp9171:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r14
	jae	.LBB46_315
	.loc	21 267 33
	cmpq	%r9, %r14
	jae	.LBB46_356
.Ltmp9172:
	.loc	21 0 0 is_stmt 0
	subl	1268(%r8), %ecx
	andl	%r11d, %ecx
	leaq	5(,%rcx,8), %r11
.Ltmp9173:
	.loc	21 268 33 is_stmt 1
	cmpq	%r9, %r11
	jae	.LBB46_377
	.loc	21 269 33
	cmpq	%rdx, %r11
	jae	.LBB46_332
.Ltmp9174:
	.loc	21 263 29
	leal	(%r15,%r13), %ecx
	movl	%ecx, %r8d
	movq	168(%rsp), %r9
	subl	1208(%r9), %r8d
	andl	88(%rsp), %r8d
	.loc	21 263 28 is_stmt 0
	leaq	6(,%r8,8), %r10
.Ltmp9175:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r10
	jae	.LBB46_393
	.loc	21 0 33 is_stmt 0
	movq	24(%rsp), %r9
	.loc	21 267 33 is_stmt 1
	cmpq	%r9, %r10
	jae	.LBB46_353
.Ltmp9176:
	.loc	21 0 33 is_stmt 0
	movq	168(%rsp), %r8
	subl	1272(%r8), %ecx
	andl	88(%rsp), %ecx
	leaq	6(,%rcx,8), %r8
.Ltmp9177:
	.loc	21 268 33 is_stmt 1
	cmpq	%r9, %r8
	jae	.LBB46_361
	.loc	21 269 33
	cmpq	%rdx, %r8
	jae	.LBB46_345
.Ltmp9178:
	.loc	21 263 29
	leal	(%r15,%r13), %r9d
	movl	%r9d, %ecx
	movq	168(%rsp), %r15
	subl	1212(%r15), %ecx
	andl	88(%rsp), %ecx
	.loc	21 263 28 is_stmt 0
	leaq	7(,%rcx,8), %rcx
.Ltmp9179:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_338
	.loc	21 0 33 is_stmt 0
	movq	24(%rsp), %rdx
	.loc	21 267 33 is_stmt 1
	cmpq	%rdx, %rcx
	jae	.LBB46_392
.Ltmp9180:
	.loc	21 0 33 is_stmt 0
	movq	168(%rsp), %r15
	subl	1276(%r15), %r9d
	andl	88(%rsp), %r9d
	leaq	7(,%r9,8), %r15
.Ltmp9181:
	.loc	21 268 33 is_stmt 1
	cmpq	%rdx, %r15
	jae	.LBB46_399
	.loc	21 269 33
	cmpq	16(%rsp), %r15
	movq	248(%rsp), %r9
	jb	.LBB46_166
.Ltmp9182:
.LBB46_319:
	.loc	21 0 33 is_stmt 0
	movq	%r15, %rdi
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_243:
	movq	592(%rsp), %rax
.Ltmp9183:
	.loc	15 2584 13 is_stmt 1
	addl	%r15d, %eax
.Ltmp9184:
	.loc	21 413 5
	movl	%eax, 2608(%r8)
	movq	752(%rsp), %rcx
.Ltmp9185:
.LBB46_244:
	.loc	6 701 9
	subl	%ecx, 2620(%r8)
.Ltmp9186:
	.loc	6 773 33
	movq	$0, 608(%rsp)
	movq	$2, 616(%rsp)
	movq	488(%rsp), %rax
	movq	%rax, 624(%rsp)
	movq	264(%rsp), %rax
	movq	%rax, 632(%rsp)
	movq	480(%rsp), %rax
	movq	%rax, 640(%rsp)
	movq	256(%rsp), %rax
	movq	%rax, 648(%rsp)
	movq	$0, 656(%rsp)
	leaq	1152(%r8), %rax
	movq	%rax, 304(%rsp)
	leaq	512(%r8), %rax
	movq	%rax, 288(%rsp)
	leaq	768(%r8), %rax
	movq	%rax, 272(%rsp)
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
	vmovaps	%ymm0, 352(%rsp)
.Ltmp9187:
	.loc	27 131 12
	leaq	(,%r9,8), %r10
	movl	$24, %edx
	vbroadcastss	.LCPI46_4(%rip), %ymm4
	vbroadcastss	.LCPI46_32(%rip), %ymm5
	vbroadcastss	.LCPI46_2(%rip), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovsd	.LCPI46_33(%rip), %xmm6
	vmovsd	.LCPI46_34(%rip), %xmm7
	vmovsd	.LCPI46_35(%rip), %xmm8
	xorl	%eax, %eax
	xorl	%esi, %esi
	movq	%r10, 384(%rsp)
	vmovaps	%ymm4, 512(%rsp)
	jmp	.LBB46_246
	.loc	27 0 12 is_stmt 0
.Ltmp9188:
	.p2align	4
.LBB46_245:
	movl	$1, %esi
	movl	$32, %edx
	.loc	27 131 12 is_stmt 1
	testb	$1, 320(%rsp)
	movb	$1, %al
	jne	.LBB46_271
.Ltmp9189:
.LBB46_246:
	.loc	27 0 12 is_stmt 0
	movq	%rax, 320(%rsp)
.Ltmp9190:
	.loc	25 253 13 is_stmt 1
	movq	%rsi, %rcx
	shlq	$4, %rcx
	leaq	624(%rsp), %rax
.Ltmp9191:
	.loc	1 1733 9
	movq	(%rax,%rcx), %r14
	movq	%rcx, 24(%rsp)
	movq	8(%rax,%rcx), %r15
.Ltmp9192:
	.loc	16 2155 12
	movq	%r15, %rax
	vmovaps	352(%rsp), %ymm0
	andq	$-8, %rax
	je	.LBB46_249
.Ltmp9193:
	.loc	16 0 12 is_stmt 0
	xorl	%ecx, %ecx
	vmovaps	352(%rsp), %ymm0
	.p2align	4
.LBB46_248:
.Ltmp9194:
	.loc	58 82 19 is_stmt 1
	vandps	(%r14,%rcx,4), %ymm4, %ymm1
.Ltmp9195:
	.loc	58 871 14
	vcmplt_oqps	%ymm5, %ymm1, %ymm1
.Ltmp9196:
	.loc	58 82 19
	vandps	%ymm1, %ymm0, %ymm0
.Ltmp9197:
	.loc	16 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB46_248
.Ltmp9198:
.LBB46_249:
	.loc	6 0 0 is_stmt 0
	imulq	$608, %rsi, %rax
.Ltmp9199:
	.file	60 "/home/bl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wide-1.6.1" "src/f32x8_.rs"
	.loc	60 176 9 is_stmt 1
	vpcmpeqd	%ymm1, %ymm1, %ymm1
	vtestps	%ymm1, %ymm0
	vandps	1856(%r8,%rax), %ymm4, %ymm0
.Ltmp9200:
	.loc	6 775 16
	jae	.LBB46_251
.Ltmp9201:
	.loc	58 871 14
	vcmplt_oqps	%ymm5, %ymm0, %ymm1
.Ltmp9202:
	.loc	58 82 19
	vandps	352(%rsp), %ymm1, %ymm1
.Ltmp9203:
	.loc	60 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp9204:
	.loc	6 775 43
	jb	.LBB46_245
.LBB46_251:
	.loc	6 0 43 is_stmt 0
	movq	%rax, 104(%rsp)
	movq	%rsi, 96(%rsp)
	movq	%rdx, 16(%rsp)
.Ltmp9205:
	.loc	16 2155 12 is_stmt 1
	movq	%r15, %rax
	vmovaps	352(%rsp), %ymm1
	movabsq	$2305843009213693944, %rcx
	andq	%rcx, %rax
	je	.LBB46_254
.Ltmp9206:
	.loc	16 0 12 is_stmt 0
	xorl	%ecx, %ecx
	vmovaps	352(%rsp), %ymm1
	.p2align	4
.LBB46_253:
.Ltmp9207:
	.loc	58 82 19 is_stmt 1
	vandps	(%r14,%rcx,4), %ymm4, %ymm2
.Ltmp9208:
	.loc	58 871 14
	vcmplt_oqps	%ymm5, %ymm2, %ymm2
.Ltmp9209:
	.loc	58 82 19
	vandps	%ymm2, %ymm1, %ymm1
.Ltmp9210:
	.loc	16 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB46_253
.Ltmp9211:
.LBB46_254:
	.loc	58 871 14
	vcmplt_oqps	%ymm5, %ymm0, %ymm0
.Ltmp9212:
	.loc	58 82 19
	vandps	352(%rsp), %ymm0, %ymm0
.Ltmp9213:
	.loc	58 585 19
	vpsrad	$31, %ymm1, %ymm1
	vmovdqa	800(%rsp), %ymm2
	vpandn	%ymm2, %ymm1, %ymm1
.Ltmp9214:
	.loc	28 185 12
	vmovd	%xmm1, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movq	%rcx, 88(%rsp)
	vpextrd	$1, %xmm1, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movq	%rcx, 864(%rsp)
	vpextrd	$2, %xmm1, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movq	%rcx, 216(%rsp)
	vpextrd	$3, %xmm1, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movq	%rcx, 120(%rsp)
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
.Ltmp9215:
	.loc	58 585 19
	vandnps	%ymm2, %ymm0, %ymm0
.Ltmp9216:
	.loc	28 185 12
	vmovd	%xmm0, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	movl	%ecx, 832(%rsp)
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
.Ltmp9217:
	.loc	28 185 12 is_stmt 0
	orl	%r8d, %edi
	movq	88(%rsp), %r8
	movq	864(%rsp), %r10
.Ltmp9218:
	.loc	28 185 12
	leal	(%r8,%r10,2), %r8d
	movq	216(%rsp), %r10
	leal	(%r8,%r10,4), %r8d
	movq	120(%rsp), %r10
	leal	(%r8,%r10,8), %r8d
	orl	%r13d, %r8d
	orl	%ebx, %r9d
	orl	%r12d, %r9d
	orl	%r8d, %r9d
.Ltmp9219:
	.loc	28 185 12
	orl	832(%rsp), %r11d
	orl	%eax, %r11d
	orl	%edx, %r11d
	orl	%ecx, %r11d
	orl	%esi, %r11d
	orl	%r9d, %r11d
.Ltmp9220:
	.loc	6 779 17 is_stmt 1
	orl	%edi, %r11d
	movq	96(%rsp), %rcx
.Ltmp9221:
	.loc	6 780 13
	movq	%rcx, %rax
	shlq	$5, %rax
	movq	168(%rsp), %r8
	addq	%r8, %rax
	movq	%rax, 88(%rsp)
	shlq	$6, %rcx
	movq	304(%rsp), %rax
	leaq	(%rax,%rcx), %r13
	movq	24(%rsp), %rax
.Ltmp9222:
	.loc	11 900 12
	addq	288(%rsp), %rax
	movq	%rax, 24(%rsp)
	leaq	(%r8,%rcx), %rax
	movq	%rax, 192(%rsp)
	leaq	(%rcx,%rcx,2), %rax
	movq	272(%rsp), %rcx
	leaq	(%rcx,%rax), %rdx
	movq	%rdx, 176(%rsp)
	leaq	64(%rcx,%rax), %rcx
	movq	%rcx, 96(%rsp)
	leaq	832(%r8,%rax), %rax
	movq	%rax, 128(%rsp)
	movq	760(%rsp), %rax
	movq	104(%rsp), %rdx
	leaq	(%rax,%rdx), %rcx
	movq	%rcx, 224(%rsp)
	leaq	576(%rax,%rdx), %rbx
	movq	16(%rsp), %rax
	addq	%rsp, %rax
	addq	$1088, %rax
	movq	%rax, 16(%rsp)
	xorl	%r12d, %r12d
	movq	248(%rsp), %r9
	movq	384(%rsp), %r10
	movl	%r11d, 144(%rsp)
	jmp	.LBB46_257
.Ltmp9223:
	.loc	11 0 12 is_stmt 0
.Ltmp9224:
	.p2align	4
.LBB46_255:
	.loc	6 789 34 is_stmt 1
	leaq	(%r12,%r12,4), %rax
	movq	16(%rsp), %rsi
	movq	(%rsi,%rax,8), %rcx
.Ltmp9225:
	.loc	15 2428 13
	addq	%r9, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp9226:
	.loc	6 794 17
	movq	%rcx, (%rsi,%rax,8)
.Ltmp9227:
.LBB46_256:
	.loc	11 0 0 is_stmt 0
	incq	%r12
	.loc	11 900 12 is_stmt 1
	addq	$4, %r14
.Ltmp9228:
	.loc	8 1916 50
	cmpq	$8, %r12
.Ltmp9229:
	.loc	11 900 12
	je	.LBB46_245
.Ltmp9230:
.LBB46_257:
	.loc	6 781 20
	btl	%r12d, %r11d
	jae	.LBB46_256
	.loc	6 0 20 is_stmt 0
	testl	%r9d, %r9d
.Ltmp9231:
	.loc	11 900 12 is_stmt 1
	je	.LBB46_262
.Ltmp9232:
	.loc	11 0 12 is_stmt 0
	xorl	%eax, %eax
	.p2align	4
.LBB46_260:
.Ltmp9233:
	.loc	6 785 21 is_stmt 1
	leaq	(%r12,%rax), %rdi
	cmpq	%r15, %rdi
	jae	.LBB46_276
	movl	$0, (%r14,%rax,4)
.Ltmp9234:
	.loc	8 1916 50
	addq	$8, %rax
	cmpq	%rax, %r10
.Ltmp9235:
	.loc	11 900 12
	jne	.LBB46_260
.Ltmp9236:
.LBB46_262:
	.loc	6 562 21
	movq	2496(%r8), %rax
.Ltmp9237:
	.loc	8 1916 50
	testq	%rax, %rax
.Ltmp9238:
	.loc	11 900 12
	je	.LBB46_266
.Ltmp9239:
	.loc	11 0 12 is_stmt 0
	movq	8(%r13), %rsi
	movq	%r12, %rdi
	.p2align	4
.LBB46_264:
.Ltmp9240:
	.loc	6 565 13 is_stmt 1
	cmpq	%rsi, %rdi
	jae	.LBB46_277
	movq	(%r13), %rcx
	movl	$0, (%rcx,%rdi,4)
.Ltmp9241:
	.loc	8 1916 50
	addq	$8, %rdi
	decq	%rax
.Ltmp9242:
	.loc	11 900 12
	jne	.LBB46_264
.Ltmp9243:
.LBB46_266:
	.loc	6 509 22
	movq	%r12, %rax
	shlq	$6, %rax
	movq	88(%rsp), %rcx
	vmovss	(%rcx,%rax), %xmm12
	vmovss	4(%rcx,%rax), %xmm11
	vmovss	8(%rcx,%rax), %xmm10
	vmovss	12(%rcx,%rax), %xmm9
	vmovss	16(%rcx,%rax), %xmm0
	vmovss	20(%rcx,%rax), %xmm2
	vmovss	24(%rcx,%rax), %xmm13
	vmovss	28(%rcx,%rax), %xmm1
.Ltmp9244:
	.loc	6 510 9
	movq	%r12, %rax
	shlq	$5, %rax
	movq	24(%rsp), %rcx
	vmovss	%xmm1, (%rcx,%rax)
	vmovss	%xmm0, 4(%rcx,%rax)
	vmovss	%xmm2, 8(%rcx,%rax)
	vmovss	%xmm13, 12(%rcx,%rax)
.Ltmp9245:
	.loc	6 533 27
	movl	2576(%r8), %eax
.Ltmp9246:
	.loc	9 82 17
	vcvtss2sd	%xmm1, %xmm1, %xmm1
.Ltmp9247:
	.loc	9 82 17 is_stmt 0
	vcvtsi2sd	%rax, %xmm15, %xmm3
.Ltmp9248:
	.loc	6 406 20 is_stmt 1
	vmulsd	%xmm3, %xmm1, %xmm1
	vdivsd	%xmm6, %xmm1, %xmm1
	.loc	6 406 19 is_stmt 0
	vaddsd	%xmm7, %xmm1, %xmm1
.Ltmp9249:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm1, %xmm1, %xmm1
	vmovq	%xmm1, %rcx
.Ltmp9250:
	.loc	6 407 9
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
	jne	.LBB46_255
	vucomisd	%xmm8, %xmm1
	ja	.LBB46_255
.Ltmp9251:
	.loc	9 82 17
	vcvtss2sd	%xmm2, %xmm2, %xmm2
.Ltmp9252:
	.loc	6 406 20
	vmulsd	%xmm3, %xmm2, %xmm2
	vdivsd	%xmm6, %xmm2, %xmm2
	.loc	6 406 19 is_stmt 0
	vaddsd	%xmm7, %xmm2, %xmm2
.Ltmp9253:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm2, %xmm2, %xmm2
	vmovq	%xmm2, %rcx
.Ltmp9254:
	.loc	6 407 9
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
	jne	.LBB46_255
	vucomisd	%xmm8, %xmm2
	ja	.LBB46_255
.Ltmp9255:
	.loc	6 534 80
	movl	2616(%r8), %ecx
	vxorpd	%xmm3, %xmm3, %xmm3
.Ltmp9256:
	.loc	6 410 10
	vmaxsd	%xmm1, %xmm3, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rdx
.Ltmp9257:
	.loc	6 410 10 is_stmt 0
	vmaxsd	%xmm2, %xmm3, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rsi
.Ltmp9258:
	.loc	6 536 9 is_stmt 1
	subl	%edx, %ecx
	movl	$0, %edx
	cmovbl	%edx, %ecx
	movq	192(%rsp), %rdx
	movl	%ecx, 1184(%rdx,%r12,4)
	.loc	6 537 62
	movl	%esi, %ecx
	vcvtsi2ss	%rcx, %xmm15, %xmm1
	movq	96(%rsp), %rcx
.Ltmp9259:
	.loc	6 399 5
	vmovaps	(%rcx), %ymm2
	vmovaps	%ymm2, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, (%rcx)
	movq	%rax, %rdi
	movq	%rax, 832(%rsp)
	vmovss	%xmm9, 104(%rsp)
	vmovss	%xmm10, 120(%rsp)
	vmovss	%xmm11, 216(%rsp)
	vmovss	%xmm12, 864(%rsp)
	vmovss	%xmm13, 544(%rsp)
.Ltmp9260:
	.loc	6 541 13
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movq	176(%rsp), %rax
.Ltmp9261:
	.loc	6 399 5
	vmovaps	(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, (%rax)
	vmovss	544(%rsp), %xmm0
	movq	832(%rsp), %rdi
.Ltmp9262:
	.loc	6 546 13
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movl	144(%rsp), %r11d
	vmovsd	.LCPI46_35(%rip), %xmm8
	vmovsd	.LCPI46_34(%rip), %xmm7
	vmovsd	.LCPI46_33(%rip), %xmm6
	vbroadcastss	.LCPI46_32(%rip), %ymm5
	vmovaps	512(%rsp), %ymm4
	movq	384(%rsp), %r10
	movq	248(%rsp), %r9
	movq	168(%rsp), %r8
	movq	96(%rsp), %rax
.Ltmp9263:
	.loc	6 399 5
	vmovaps	-32(%rax), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -32(%rax)
	movq	128(%rsp), %rax
.Ltmp9264:
	.loc	6 517 29
	vmovaps	(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
.Ltmp9265:
	.loc	6 393 5
	vmovss	32(%rsp,%r12,4), %xmm0
.Ltmp9266:
	.loc	6 399 5
	vmovaps	(%rbx), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, (%rbx)
.Ltmp9267:
	.loc	6 399 5
	vmovaps	-64(%rbx), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	movl	$1065353216, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, -64(%rbx)
.Ltmp9268:
	.loc	6 399 5
	vmovaps	-32(%rbx), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -32(%rbx)
	movq	224(%rsp), %rax
.Ltmp9269:
	.loc	6 399 5
	vmovaps	(%rax), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	vmovss	864(%rsp), %xmm1
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, (%rax)
.Ltmp9270:
	.loc	6 399 5
	vmovaps	-544(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -544(%rbx)
.Ltmp9271:
	.loc	6 399 5
	vmovaps	-512(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -512(%rbx)
.Ltmp9272:
	.loc	6 399 5
	vmovaps	-480(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -480(%rbx)
.Ltmp9273:
	.loc	6 399 5
	vmovaps	-448(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	vmovss	216(%rsp), %xmm1
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -448(%rbx)
.Ltmp9274:
	.loc	6 399 5
	vmovaps	-416(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -416(%rbx)
.Ltmp9275:
	.loc	6 399 5
	vmovaps	-384(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -384(%rbx)
.Ltmp9276:
	.loc	6 399 5
	vmovaps	-352(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -352(%rbx)
.Ltmp9277:
	.loc	6 399 5
	vmovaps	-320(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	vmovss	120(%rsp), %xmm1
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -320(%rbx)
.Ltmp9278:
	.loc	6 399 5
	vmovaps	-288(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -288(%rbx)
.Ltmp9279:
	.loc	6 399 5
	vmovaps	-256(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -256(%rbx)
.Ltmp9280:
	.loc	6 399 5
	vmovaps	-224(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -224(%rbx)
.Ltmp9281:
	.loc	6 399 5
	vmovaps	-192(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	vmovss	104(%rsp), %xmm1
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -192(%rbx)
.Ltmp9282:
	.loc	6 399 5
	vmovaps	-160(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	vmovss	%xmm1, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -160(%rbx)
.Ltmp9283:
	.loc	6 399 5
	vmovaps	-128(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -128(%rbx)
.Ltmp9284:
	.loc	6 399 5
	vmovaps	-96(%rbx), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	6 400 5
	movl	$0, 32(%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, -96(%rbx)
	jmp	.LBB46_255
.Ltmp9285:
.LBB46_271:
	.loc	6 0 5 is_stmt 0
	leaq	1088(%rsp), %rsi
	.loc	6 1102 17 is_stmt 1
	movl	$320, %edx
	movq	1072(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	%rbx, %rdi
	movzbl	119(%rsp), %eax
	movb	%al, 320(%rbx)
.Ltmp9286:
.LBB46_272:
	.loc	6 1103 14
	movq	%rdi, %rax
	leaq	-40(%rbp), %rsp
	.loc	6 1103 14 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	vzeroupper
	retq
.LBB46_273:
	.cfi_def_cfa %rbp, 16
.Ltmp9287:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_d54ae23796fd62146400a9560f58d006(%rip), %rcx
	movq	96(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9288:
.LBB46_274:
	.loc	25 443 13
	leaq	8(%rdi), %rsi
	leaq	.Lalloc_d0dbd696a058609bd94bbe1fa75d0723(%rip), %rcx
	movq	1064(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9289:
.LBB46_275:
	.loc	25 443 13
	leaq	8(%rdi), %rsi
	leaq	.Lalloc_d0dbd696a058609bd94bbe1fa75d0723(%rip), %rcx
	movq	688(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9290:
.LBB46_276:
	.loc	6 785 21
	leaq	.Lalloc_6797264598a169e4722ae66c7bc497b8(%rip), %rdx
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9291:
.LBB46_277:
	.loc	6 565 13
	leaq	.Lalloc_835aafef72e8508601474e7b1f4172a9(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9292:
.LBB46_278:
	.loc	25 456 13
	leaq	.Lalloc_1eeef3195352adc58f4bfdb015316fef(%rip), %rcx
	movq	%r10, %rdi
	movq	16(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9293:
.LBB46_279:
	.loc	25 456 13
	leaq	.Lalloc_b9d56679ca30f2caa500ddf2e89d00cb(%rip), %rcx
	movq	%r10, %rdi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9294:
.LBB46_280:
	.loc	25 443 13
	leaq	.Lalloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7(%rip), %rcx
	movq	%r11, %rdi
	movq	%r10, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9295:
.LBB46_281:
	.loc	25 443 13
	leaq	.Lalloc_f198228fd40f4c04a48a745576c5c5ee(%rip), %rcx
	movq	%r11, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9296:
.LBB46_282:
	.loc	25 456 13
	leaq	.Lalloc_1eeef3195352adc58f4bfdb015316fef(%rip), %rcx
	movq	%r11, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9297:
.LBB46_283:
	.loc	25 0 13 is_stmt 0
	movabsq	$2305843009213693944, %rsi
	movq	672(%rsp), %rdx
.Ltmp9298:
	.loc	11 900 12 is_stmt 1
	andq	%rdx, %rsi
	addq	$8, %rsi
.Ltmp9299:
	.loc	25 443 13
	leaq	.Lalloc_d9529ff5ddc99dd60299cff5ff3cd676(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9300:
.LBB46_284:
	.loc	25 456 13
	leaq	.Lalloc_b9d56679ca30f2caa500ddf2e89d00cb(%rip), %rcx
	movq	%r11, %rdi
	movq	%r9, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9301:
.LBB46_285:
	.loc	25 443 13
	leaq	.Lalloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7(%rip), %rcx
	movq	%rbx, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9302:
.LBB46_286:
	.loc	25 443 13
	leaq	.Lalloc_f198228fd40f4c04a48a745576c5c5ee(%rip), %rcx
	movq	%rbx, %rdi
	movq	%r9, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9303:
.LBB46_287:
	.loc	18 968 21
	leaq	.Lalloc_376120b9c5efdf3d59386c16952a74b7(%rip), %rdi
	leaq	.Lalloc_d54ae23796fd62146400a9560f58d006(%rip), %rdx
	movl	$31, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core6option13expect_failed@GOTPCREL(%rip)
.Ltmp9304:
.LBB46_288:
	.loc	25 456 13
	leaq	.Lalloc_dd5f55065f566218c9f31cb2a4357231(%rip), %rcx
	xorl	%edi, %edi
	movq	264(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9305:
.LBB46_289:
	.loc	25 456 13
	leaq	.Lalloc_1d7cc6e40c752396aa7def7556a6c433(%rip), %rcx
	xorl	%edi, %edi
	movq	256(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9306:
.LBB46_290:
	.loc	25 581 13
	leaq	.Lalloc_1e79f4c3c2f015f90ab54e70b61044ab(%rip), %rcx
	movq	264(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9307:
.LBB46_291:
	.loc	25 581 13
	leaq	.Lalloc_a6d4388bd1c2ee005f6a969a0e3ca0f4(%rip), %rcx
	movq	256(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9308:
.LBB46_292:
	.loc	6 1090 33
	leaq	.Lalloc_d54ae23796fd62146400a9560f58d006(%rip), %rdx
	movq	%r9, %rdi
	movq	%r9, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_293:
.Ltmp9309:
	.loc	6 1091 31
	leaq	.Lalloc_d54ae23796fd62146400a9560f58d006(%rip), %rdx
	movq	%rax, %rdi
	movq	%r9, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9310:
.LBB46_338:
	.loc	6 0 31 is_stmt 0
	movq	%rcx, %rdi
.Ltmp9311:
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_294:
	movq	%rax, %rdi
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_295:
	movq	%rsi, %rdi
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_296:
	movq	%rax, %rdi
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_315:
	movq	%r14, %rdi
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_339:
	movq	%r14, %rdi
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_392:
	movq	%rcx, %rdi
.Ltmp9312:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9313:
.LBB46_347:
	.loc	21 0 33 is_stmt 0
	movq	%rcx, %rdi
.Ltmp9314:
	.loc	21 267 33
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9315:
.LBB46_297:
	.loc	21 0 33
	movq	%rsi, %rdi
.Ltmp9316:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9317:
.LBB46_298:
	.loc	21 0 33 is_stmt 0
	movq	%rsi, %rdi
.Ltmp9318:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9319:
.LBB46_299:
	.loc	21 0 33 is_stmt 0
	movq	%rsi, %rdi
.Ltmp9320:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_333:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, %rdi
	.loc	21 237 33
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9321:
.LBB46_300:
	.loc	21 0 33
	movq	%rax, %rdi
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_322:
	movq	%r9, %rdi
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_330:
	movq	%rcx, %rdi
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_332:
	movq	%r11, %rdi
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_345:
	movq	%r8, %rdi
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_301:
	movq	%rsi, %rdi
.Ltmp9322:
	.loc	21 237 33
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9323:
.LBB46_302:
	.loc	21 0 33
	movq	%rsi, %rdi
.Ltmp9324:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9325:
.LBB46_303:
	.loc	21 0 33 is_stmt 0
	movq	%rsi, %rdi
.Ltmp9326:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9327:
.LBB46_356:
	.loc	21 0 33 is_stmt 0
	movq	%r14, %rdi
.Ltmp9328:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9329:
.LBB46_366:
	.loc	21 0 33 is_stmt 0
	movq	%r14, %rdi
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_376:
	movq	%r12, %rdi
.Ltmp9330:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9331:
.LBB46_379:
	.loc	21 0 33 is_stmt 0
	movq	%r12, %rdi
.Ltmp9332:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9333:
.LBB46_402:
	.loc	21 0 33 is_stmt 0
	movq	%rax, %rdi
.Ltmp9334:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9335:
.LBB46_304:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, %rdi
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_305:
	movq	%r9, %rdi
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_306:
	movq	%r10, %rdi
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_307:
	movq	%r12, %rdi
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_308:
	movq	%r12, %rdi
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_309:
	movq	%r15, %rdi
.Ltmp9336:
	.loc	21 268 33
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9337:
.LBB46_310:
	.loc	21 0 33
	movq	%r12, %rdi
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_312:
	movq	%r11, %rdi
.Ltmp9338:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9339:
.LBB46_313:
	.loc	21 0 33 is_stmt 0
	movq	%rdx, %rdi
.Ltmp9340:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9341:
.LBB46_314:
	.loc	21 0 33 is_stmt 0
	movq	%rcx, %rdi
.Ltmp9342:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9343:
.LBB46_316:
	.loc	21 0 33 is_stmt 0
	movq	%r14, %rdi
.Ltmp9344:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9345:
.LBB46_317:
	.loc	21 0 33 is_stmt 0
	movq	%r12, %rdi
.Ltmp9346:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9347:
.LBB46_318:
	.loc	21 0 33 is_stmt 0
	movq	%r14, %rdi
.Ltmp9348:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9349:
.LBB46_320:
	.loc	21 0 33 is_stmt 0
	movq	128(%rsp), %rdi
.Ltmp9350:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9351:
.LBB46_321:
	.loc	21 0 33 is_stmt 0
	movq	%r13, %rdi
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_323:
	movq	%r11, %rdi
.Ltmp9352:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9353:
.LBB46_324:
	.loc	21 0 33 is_stmt 0
	movq	%r11, %rdi
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_325:
	movq	%r14, %rdi
.Ltmp9354:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9355:
.LBB46_326:
	.loc	21 0 33 is_stmt 0
	movq	%r13, %rdi
.Ltmp9356:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9357:
.LBB46_327:
	.loc	21 0 33 is_stmt 0
	movq	%rax, %rdi
.Ltmp9358:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9359:
.LBB46_328:
	.loc	21 0 33 is_stmt 0
	movq	%r11, %rdi
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_329:
	movq	%rsi, %rdi
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_331:
	movq	%r13, %rdi
.Ltmp9360:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9361:
.LBB46_334:
	.loc	21 0 33 is_stmt 0
	movq	%r9, %rdi
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_335:
	movq	%r10, %rdi
.Ltmp9362:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9363:
.LBB46_336:
	.loc	21 0 33 is_stmt 0
	movq	%rsi, %rdi
.Ltmp9364:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_337:
	.loc	21 0 33 is_stmt 0
	movq	%r8, %rdi
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_341:
	.loc	21 0 33 is_stmt 0
	movq	%rax, %rdi
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9365:
.LBB46_342:
	.loc	21 0 33 is_stmt 0
	movq	%r13, %rdi
.Ltmp9366:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9367:
.LBB46_343:
	.loc	21 0 33 is_stmt 0
	movq	%r9, %rdi
.LBB46_344:
.Ltmp9368:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9369:
.LBB46_346:
	.loc	21 0 33 is_stmt 0
	movq	%rax, %rdi
.Ltmp9370:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9371:
.LBB46_348:
	.loc	21 0 33 is_stmt 0
	movq	176(%rsp), %rdi
.Ltmp9372:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	104(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9373:
.LBB46_350:
	.loc	21 0 33 is_stmt 0
	movq	%rdx, %rdi
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_351:
	movq	%rdx, %rdi
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_352:
	movq	%rax, %rdi
.Ltmp9374:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_353:
	.loc	21 0 33 is_stmt 0
	movq	%r10, %rdi
	.loc	21 267 33
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9375:
.LBB46_354:
	.loc	21 0 33
	movq	%rbx, %rdi
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_359:
	movq	144(%rsp), %rdi
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_360:
	movq	%rcx, %rdi
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_361:
	movq	%r8, %rdi
.Ltmp9376:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_362:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, %rdi
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_364:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, %rdi
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9377:
.LBB46_368:
	.loc	21 0 33 is_stmt 0
	movq	%rax, %rdi
.Ltmp9378:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9379:
.LBB46_371:
	.loc	21 0 33 is_stmt 0
	movq	144(%rsp), %rdi
.Ltmp9380:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9381:
.LBB46_373:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, %rdi
.Ltmp9382:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9383:
.LBB46_375:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, %rdi
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_377:
	movq	%r11, %rdi
.Ltmp9384:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9385:
.LBB46_381:
	.loc	21 0 33 is_stmt 0
	movq	%r15, %rdi
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_383:
	movq	%r15, %rdi
.Ltmp9386:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9387:
.LBB46_384:
	.loc	21 0 33 is_stmt 0
	movq	%rbx, %rdi
.LBB46_385:
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_388:
	movq	%rbx, %rdi
.LBB46_389:
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_391:
	movq	%r10, %rdi
.Ltmp9388:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9389:
.LBB46_393:
	.loc	21 0 33 is_stmt 0
	movq	%r10, %rdi
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_394:
	movq	%r9, %rdi
.Ltmp9390:
	.loc	21 237 33
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9391:
.LBB46_395:
	.loc	21 0 33
	movq	%rax, %rdi
.LBB46_396:
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	16(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB46_397:
	movq	%r10, %rdi
.Ltmp9392:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9393:
.LBB46_399:
	.loc	21 0 33 is_stmt 0
	movq	%r15, %rdi
.Ltmp9394:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9395:
.LBB46_401:
	.loc	21 0 33 is_stmt 0
	movq	%r9, %rdi
.Ltmp9396:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
	movq	24(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9397:
.Lfunc_end46:
	.size	_RNvXse_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank, .Lfunc_end46-_RNvXse_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank
