_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_:
.Lfunc_begin40:
	.loc	1 1986 0
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
	subq	$320, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, 2776(%rsp)
.Ltmp6141:
	.loc	1 1987 51 prologue_end
	movzbl	5424(%rsi), %eax
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqa	%ymm0, 3072(%rsp)
	vmovdqa	%ymm0, 3040(%rsp)
	vmovdqa	%ymm0, 3008(%rsp)
	vmovdqa	%ymm0, 2976(%rsp)
	vmovdqa	%ymm0, 2944(%rsp)
	vmovdqa	%ymm0, 2912(%rsp)
	vmovdqa	%ymm0, 2880(%rsp)
	vmovdqa	%ymm0, 2848(%rsp)
	vmovdqa	%ymm0, 2816(%rsp)
	vmovdqa	%ymm0, 2784(%rsp)
.Ltmp6142:
	.loc	43 1032 9
	movb	%al, 3104(%rsp)
.Ltmp6143:
	.loc	43 186 45
	cmpb	%al, 108(%rdx)
.Ltmp6144:
	.loc	1 1991 12
	jne	.LBB40_622
	cmpq	$0, 64(%rdx)
	jne	.LBB40_622
	.loc	1 0 12 is_stmt 0
	movq	%rsi, %r12
	movq	48(%rdx), %r10
	movq	56(%rdx), %r8
	movq	32(%rdx), %rax
	movq	%rax, 2560(%rsp)
	movq	40(%rdx), %rax
	movq	%rax, 1600(%rsp)
	movq	%rdx, 2400(%rsp)
	movq	96(%rdx), %r11
.Ltmp6145:
	.loc	3 900 12 is_stmt 1
	cmpq	$1, %r8
	movq	%r8, %rax
	adcq	$-1, %rax
	movq	%rax, 1728(%rsp)
	leaq	156(%rsi), %rbx
	xorl	%eax, %eax
	vmovss	.LCPI40_0(%rip), %xmm2
	movq	%rsi, 1720(%rsp)
	movq	%r8, 2528(%rsp)
	movq	%r10, 2496(%rsp)
	movq	%r11, 2464(%rsp)
	jmp	.LBB40_4
	.loc	3 0 12 is_stmt 0
.Ltmp6146:
	.p2align	4
.LBB40_3:
	.loc	3 900 12 is_stmt 1
	addq	$160, %rbx
	movq	1568(%rsp), %rcx
	movq	%rcx, %rax
.Ltmp6147:
	.loc	2 1916 50
	cmpq	$8, %rcx
	movq	1720(%rsp), %r12
.Ltmp6148:
	.loc	3 900 12
	je	.LBB40_100
.Ltmp6149:
.LBB40_4:
	.loc	1 1995 25
	cmpq	%r8, %rax
	je	.LBB40_662
.Ltmp6150:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rax), %rcx
.Ltmp6151:
	.loc	1 1996 23 is_stmt 1
	cmpq	1728(%rsp), %rax
	je	.LBB40_663
	.loc	1 0 23 is_stmt 0
	movl	(%r10,%rax,4), %edi
	.loc	1 1996 23
	movl	(%r10,%rcx,4), %esi
.Ltmp6152:
	.loc	38 1050 16 is_stmt 1
	cmpl	%edi, %esi
	jb	.LBB40_632
	cmpq	%rsi, 1600(%rsp)
	jb	.LBB40_632
.Ltmp6153:
	.loc	38 0 16 is_stmt 0
	movq	%rcx, 1568(%rsp)
	movq	%rbx, 192(%rsp)
	.loc	1 2000 17 is_stmt 1
	movl	5524(%r12), %edx
	movl	$0, 256(%rsp)
	movl	$0, 264(%rsp)
	movl	$0, 272(%rsp)
	movl	$0, 280(%rsp)
	movl	$0, 288(%rsp)
	movl	$0, 296(%rsp)
	movl	$0, 304(%rsp)
	movl	$0, 312(%rsp)
	movl	$0, 320(%rsp)
	movl	$0, 328(%rsp)
	movl	$0, 336(%rsp)
	movl	$0, 344(%rsp)
	movl	$0, 352(%rsp)
	movl	$0, 360(%rsp)
	movl	$0, 368(%rsp)
	movl	$0, 376(%rsp)
	movl	$0, 384(%rsp)
	movl	$0, 392(%rsp)
	movl	$0, 400(%rsp)
	movl	$0, 408(%rsp)
.Ltmp6154:
	.loc	32 1714 9
	cmpl	%edi, %esi
.Ltmp6155:
	.loc	33 180 28
	jne	.LBB40_81
.Ltmp6156:
.LBB40_9:
	.loc	33 0 28 is_stmt 0
	movl	$76, %eax
	movq	192(%rsp), %rbx
	movq	%rbx, %rcx
.Ltmp6157:
	.loc	33 180 28
	jmp	.LBB40_13
.Ltmp6158:
	.loc	33 0 28
.Ltmp6159:
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
.Ltmp6160:
.LBB40_12:
	.loc	32 1714 9 is_stmt 1
	addq	$80, %rax
	addq	$2624, %rcx
	cmpq	$236, %rax
.Ltmp6161:
	.loc	33 180 28
	je	.LBB40_3
.Ltmp6162:
.LBB40_13:
	.loc	1 1495 24
	cmpl	$1, 180(%rsp,%rax)
	jne	.LBB40_14
	.loc	1 1495 29 is_stmt 0
	vmovd	184(%rsp,%rax), %xmm0
.Ltmp6163:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -152(%rcx)
	.loc	36 81 48
	vmovd	-156(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6164:
	.loc	36 112 9
	jg	.LBB40_27
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_27
	negl	%edx
	jo	.LBB40_27
.Ltmp6165:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -156(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -148(%rcx)
	movl	%edx, -144(%rcx)
.Ltmp6166:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 188(%rsp,%rax)
	je	.LBB40_29
	.loc	1 0 24 is_stmt 0
.Ltmp6167:
	.p2align	4
.LBB40_15:
	.loc	1 1495 24
	cmpl	$1, 196(%rsp,%rax)
	jne	.LBB40_16
.LBB40_35:
	.loc	1 1495 29
	vmovd	200(%rsp,%rax), %xmm0
.Ltmp6168:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -120(%rcx)
	.loc	36 81 48
	vmovd	-124(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6169:
	.loc	36 112 9
	jg	.LBB40_39
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_39
	negl	%edx
	jo	.LBB40_39
.Ltmp6170:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -124(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -116(%rcx)
	movl	%edx, -112(%rcx)
.Ltmp6171:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 204(%rsp,%rax)
	je	.LBB40_41
	.loc	1 0 24 is_stmt 0
.Ltmp6172:
	.p2align	4
.LBB40_17:
	.loc	1 1495 24
	cmpl	$1, 212(%rsp,%rax)
	jne	.LBB40_18
.LBB40_47:
	.loc	1 1495 29
	vmovd	216(%rsp,%rax), %xmm0
.Ltmp6173:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -88(%rcx)
	.loc	36 81 48
	vmovd	-92(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6174:
	.loc	36 112 9
	jg	.LBB40_51
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_51
	negl	%edx
	jo	.LBB40_51
.Ltmp6175:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -92(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -84(%rcx)
	movl	%edx, -80(%rcx)
.Ltmp6176:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 220(%rsp,%rax)
	je	.LBB40_53
	.loc	1 0 24 is_stmt 0
.Ltmp6177:
	.p2align	4
.LBB40_19:
	.loc	1 1495 24
	cmpl	$1, 228(%rsp,%rax)
	jne	.LBB40_20
.LBB40_59:
	.loc	1 1495 29
	vmovd	232(%rsp,%rax), %xmm0
.Ltmp6178:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -56(%rcx)
	.loc	36 81 48
	vmovd	-60(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6179:
	.loc	36 112 9
	jg	.LBB40_63
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_63
	negl	%edx
	jo	.LBB40_63
.Ltmp6180:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -60(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -52(%rcx)
	movl	%edx, -48(%rcx)
.Ltmp6181:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 236(%rsp,%rax)
	je	.LBB40_65
	.loc	1 0 24 is_stmt 0
.Ltmp6182:
	.p2align	4
.LBB40_21:
	.loc	1 1495 24
	cmpl	$1, 244(%rsp,%rax)
	jne	.LBB40_22
.LBB40_71:
	.loc	1 1495 29
	vmovd	248(%rsp,%rax), %xmm0
.Ltmp6183:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -24(%rcx)
	.loc	36 81 48
	vmovd	-28(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6184:
	.loc	36 112 9
	jg	.LBB40_75
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_75
	negl	%edx
	jo	.LBB40_75
.Ltmp6185:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -28(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -20(%rcx)
	movl	%edx, -16(%rcx)
.Ltmp6186:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 252(%rsp,%rax)
	jne	.LBB40_12
	jmp	.LBB40_77
	.loc	1 0 24 is_stmt 0
.Ltmp6187:
	.p2align	4
.LBB40_14:
	.loc	1 1495 24
	cmpl	$1, 188(%rsp,%rax)
	jne	.LBB40_15
.LBB40_29:
	.loc	1 1495 29
	vmovd	192(%rsp,%rax), %xmm0
.Ltmp6188:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -136(%rcx)
	.loc	36 81 48
	vmovd	-140(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6189:
	.loc	36 112 9
	jg	.LBB40_33
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_33
	negl	%edx
	jo	.LBB40_33
.Ltmp6190:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -140(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -132(%rcx)
	movl	%edx, -128(%rcx)
.Ltmp6191:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 196(%rsp,%rax)
	jne	.LBB40_16
	jmp	.LBB40_35
	.loc	1 0 24 is_stmt 0
.Ltmp6192:
	.p2align	4
.LBB40_27:
.Ltmp6193:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -148(%rcx)
	movl	%edx, -144(%rcx)
.Ltmp6194:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 188(%rsp,%rax)
	jne	.LBB40_15
	jmp	.LBB40_29
	.loc	1 0 24 is_stmt 0
.Ltmp6195:
	.p2align	4
.LBB40_39:
.Ltmp6196:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -116(%rcx)
	movl	%edx, -112(%rcx)
.Ltmp6197:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 204(%rsp,%rax)
	jne	.LBB40_17
	jmp	.LBB40_41
	.loc	1 0 24 is_stmt 0
.Ltmp6198:
	.p2align	4
.LBB40_51:
.Ltmp6199:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -84(%rcx)
	movl	%edx, -80(%rcx)
.Ltmp6200:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 220(%rsp,%rax)
	jne	.LBB40_19
	jmp	.LBB40_53
	.loc	1 0 24 is_stmt 0
.Ltmp6201:
	.p2align	4
.LBB40_63:
.Ltmp6202:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -52(%rcx)
	movl	%edx, -48(%rcx)
.Ltmp6203:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 236(%rsp,%rax)
	jne	.LBB40_21
	jmp	.LBB40_65
	.loc	1 0 24 is_stmt 0
.Ltmp6204:
	.p2align	4
.LBB40_75:
.Ltmp6205:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -20(%rcx)
	movl	%edx, -16(%rcx)
.Ltmp6206:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 252(%rsp,%rax)
	jne	.LBB40_12
	jmp	.LBB40_77
	.loc	1 0 24 is_stmt 0
.Ltmp6207:
	.p2align	4
.LBB40_33:
.Ltmp6208:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -132(%rcx)
	movl	%edx, -128(%rcx)
.Ltmp6209:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 196(%rsp,%rax)
	je	.LBB40_35
	.loc	1 0 24 is_stmt 0
.Ltmp6210:
	.p2align	4
.LBB40_16:
	.loc	1 1495 24
	cmpl	$1, 204(%rsp,%rax)
	jne	.LBB40_17
.LBB40_41:
	.loc	1 1495 29
	vmovd	208(%rsp,%rax), %xmm0
.Ltmp6211:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -104(%rcx)
	.loc	36 81 48
	vmovd	-108(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6212:
	.loc	36 112 9
	jg	.LBB40_45
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_45
	negl	%edx
	jo	.LBB40_45
.Ltmp6213:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -108(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -100(%rcx)
	movl	%edx, -96(%rcx)
.Ltmp6214:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 212(%rsp,%rax)
	jne	.LBB40_18
	jmp	.LBB40_47
	.loc	1 0 24 is_stmt 0
.Ltmp6215:
	.p2align	4
.LBB40_45:
.Ltmp6216:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -100(%rcx)
	movl	%edx, -96(%rcx)
.Ltmp6217:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 212(%rsp,%rax)
	je	.LBB40_47
	.loc	1 0 24 is_stmt 0
.Ltmp6218:
	.p2align	4
.LBB40_18:
	.loc	1 1495 24
	cmpl	$1, 220(%rsp,%rax)
	jne	.LBB40_19
.LBB40_53:
	.loc	1 1495 29
	vmovd	224(%rsp,%rax), %xmm0
.Ltmp6219:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -72(%rcx)
	.loc	36 81 48
	vmovd	-76(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6220:
	.loc	36 112 9
	jg	.LBB40_57
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_57
	negl	%edx
	jo	.LBB40_57
.Ltmp6221:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -76(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -68(%rcx)
	movl	%edx, -64(%rcx)
.Ltmp6222:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 228(%rsp,%rax)
	jne	.LBB40_20
	jmp	.LBB40_59
	.loc	1 0 24 is_stmt 0
.Ltmp6223:
	.p2align	4
.LBB40_57:
.Ltmp6224:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -68(%rcx)
	movl	%edx, -64(%rcx)
.Ltmp6225:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 228(%rsp,%rax)
	je	.LBB40_59
	.loc	1 0 24 is_stmt 0
.Ltmp6226:
	.p2align	4
.LBB40_20:
	.loc	1 1495 24
	cmpl	$1, 236(%rsp,%rax)
	jne	.LBB40_21
.LBB40_65:
	.loc	1 1495 29
	vmovd	240(%rsp,%rax), %xmm0
.Ltmp6227:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -40(%rcx)
	.loc	36 81 48
	vmovd	-44(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6228:
	.loc	36 112 9
	jg	.LBB40_69
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_69
	negl	%edx
	jo	.LBB40_69
.Ltmp6229:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -44(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -36(%rcx)
	movl	%edx, -32(%rcx)
.Ltmp6230:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 244(%rsp,%rax)
	jne	.LBB40_22
	jmp	.LBB40_71
	.loc	1 0 24 is_stmt 0
.Ltmp6231:
	.p2align	4
.LBB40_69:
.Ltmp6232:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -36(%rcx)
	movl	%edx, -32(%rcx)
.Ltmp6233:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 244(%rsp,%rax)
	je	.LBB40_71
	.loc	1 0 24 is_stmt 0
.Ltmp6234:
	.p2align	4
.LBB40_22:
	.loc	1 1495 24
	cmpl	$1, 252(%rsp,%rax)
	jne	.LBB40_12
.LBB40_77:
	.loc	1 1495 29
	vmovd	256(%rsp,%rax), %xmm0
.Ltmp6235:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -8(%rcx)
	.loc	36 81 48
	vmovd	-12(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp6236:
	.loc	36 112 9
	jg	.LBB40_10
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB40_10
	negl	%edx
	jo	.LBB40_10
.Ltmp6237:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -12(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 89 6
	jmp	.LBB40_11
.Ltmp6238:
	.loc	36 0 6 is_stmt 0
.Ltmp6239:
	.p2align	4
.LBB40_81:
	subq	%rdi, %rsi
	leaq	(%rdi,%rdi,4), %rcx
	movq	%rdx, 1632(%rsp)
	movq	2560(%rsp), %rdx
	leaq	(%rdx,%rcx,8), %r12
	movq	1632(%rsp), %rdx
	leaq	(%rsi,%rsi,4), %rcx
	leaq	(%r12,%rcx,8), %rsi
	.loc	1 2002 17 is_stmt 1
	leaq	(%rax,%rax,4), %rax
	leaq	2784(%rsp,%rax,8), %r14
	movq	2800(%rsp,%rax,8), %rbx
	movb	$1, %al
	movl	%eax, 1760(%rsp)
	xorl	%r13d, %r13d
	jmp	.LBB40_82
	.loc	1 0 17 is_stmt 0
.Ltmp6240:
	.p2align	4
.LBB40_98:
.Ltmp6241:
	addq	$40, %r12
.Ltmp6242:
	.loc	38 2428 13 is_stmt 1
	incq	%rbx
	movq	$-1, %rax
	cmoveq	%rax, %rbx
.Ltmp6243:
	.loc	1 0 0 is_stmt 0
	movq	%rbx, 16(%r14)
.Ltmp6244:
	.loc	4 82 9 is_stmt 1
	incq	%r13
.Ltmp6245:
	.loc	32 1714 9
	cmpq	%rsi, %r12
.Ltmp6246:
	.loc	33 180 28
	je	.LBB40_9
.Ltmp6247:
.LBB40_82:
	.loc	1 1457 30
	movl	32(%r12), %eax
	.loc	1 1457 24 is_stmt 0
	cmpl	$1, %eax
	je	.LBB40_86
	cmpl	$2, %eax
	jne	.LBB40_98
	.loc	1 0 24
	movl	$1, %eax
	leaq	336(%rsp), %r15
.Ltmp6248:
	.loc	1 1465 29 is_stmt 1
	movl	16(%r12), %edi
	cmpq	$2, %rdi
.Ltmp6249:
	.loc	38 1050 16
	jae	.LBB40_87
.Ltmp6250:
.LBB40_85:
	.loc	38 0 16 is_stmt 0
	xorl	%ecx, %ecx
	cmpq	%rdx, %r13
.Ltmp6251:
	.loc	1 1473 25 is_stmt 1
	jb	.LBB40_88
	jmp	.LBB40_98
.Ltmp6252:
	.loc	1 0 25 is_stmt 0
.Ltmp6253:
	.p2align	4
.LBB40_86:
	xorl	%eax, %eax
	leaq	256(%rsp), %r15
	.loc	1 1465 29 is_stmt 1
	movl	16(%r12), %edi
	cmpq	$2, %rdi
.Ltmp6254:
	.loc	38 1050 16
	jb	.LBB40_85
.LBB40_87:
	.loc	38 1054 31
	leaq	-2(%rdi), %rcx
	movq	%rcx, 64(%rsp)
.Ltmp6255:
	.loc	28 1580 16
	xorl	%ecx, %ecx
	cmpl	$12, %edi
	setb	%cl
	cmpq	%rdx, %r13
.Ltmp6256:
	.loc	1 1473 25
	jae	.LBB40_98
.LBB40_88:
	cmpq	$1, %rcx
	jne	.LBB40_98
	.loc	1 1475 20
	cmpl	$1, 28(%r12)
	jne	.LBB40_98
	.loc	1 1476 20
	cmpq	%r11, (%r12)
	jne	.LBB40_98
	.loc	1 1477 20
	cmpq	%r11, 8(%r12)
	jne	.LBB40_98
	.loc	1 1478 20
	vmovd	20(%r12), %xmm0
.Ltmp6257:
	.loc	23 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6258:
	.loc	1 1478 20
	cmpl	%ecx, 24(%r12)
	jne	.LBB40_98
	.loc	1 0 20 is_stmt 0
	movq	%rsi, 2144(%rsp)
	.loc	1 1479 43 is_stmt 1
	cmpl	$11, %edi
	ja	.LBB40_664
	.loc	1 0 43 is_stmt 0
	leal	(%rax,%rdi,2), %eax
	movl	%eax, 2592(%rsp)
	.loc	1 1479 42
	leaq	(%rdi,%rdi,4), %rax
	leaq	.Lalloc_cc33a3b9cd8c16d253f2168b5461d31d(%rip), %rcx
	leaq	(%rcx,%rax,8), %rdi
	vmovdqa	%xmm0, 2336(%rsp)
	.loc	1 1479 20
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	2336(%rsp), %xmm1
	movl	2592(%rsp), %r9d
	cmpl	2432(%rsp), %r9d
	seta	%cl
	testb	%al, %al
	movq	2528(%rsp), %r8
	movq	2496(%rsp), %r10
	movq	2464(%rsp), %r11
	vmovss	.LCPI40_0(%rip), %xmm2
	movq	1632(%rsp), %rdx
	movq	2144(%rsp), %rsi
	je	.LBB40_98
	orb	1760(%rsp), %cl
	testb	$1, %cl
	je	.LBB40_98
	.loc	1 0 20
	movq	64(%rsp), %rdi
.Ltmp6259:
	.loc	1 1481 45 is_stmt 1
	cmpq	$9, %rdi
	ja	.LBB40_667
.Ltmp6260:
	.loc	47 430 9
	cmpl	$0, (%r15,%rdi,8)
.Ltmp6261:
	.loc	1 1486 17
	jne	.LBB40_98
.Ltmp6262:
	.loc	31 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
	movq	64(%rsp), %rax
.Ltmp6263:
	.loc	1 1491 13
	movl	$1, (%r15,%rax,8)
	vmovss	%xmm0, 4(%r15,%rax,8)
.Ltmp6264:
	.loc	32 1714 9
	addq	$40, %r12
.Ltmp6265:
	.loc	33 180 28
	incq	%r13
	movl	$0, 1760(%rsp)
	movl	%r9d, 2432(%rsp)
.Ltmp6266:
	.loc	32 1714 9
	cmpq	%rsi, %r12
.Ltmp6267:
	.loc	33 180 28
	jne	.LBB40_82
	jmp	.LBB40_9
.Ltmp6268:
.LBB40_100:
	.loc	33 0 28 is_stmt 0
	movq	2400(%rsp), %rcx
	.loc	1 2007 13 is_stmt 1
	movq	(%rcx), %rax
	movq	%rax, 240(%rsp)
	movq	8(%rcx), %rax
	movq	%rax, 128(%rsp)
	.loc	1 2008 13
	movq	16(%rcx), %rax
	movq	%rax, 248(%rsp)
	movq	24(%rcx), %rax
	movq	%rax, 120(%rsp)
	.loc	1 2009 13
	movl	104(%rcx), %eax
	movq	%rax, 144(%rsp)
.Ltmp6269:
	.loc	1 1245 12
	movl	5284(%r12), %ecx
	.loc	1 1245 27 is_stmt 0
	movzbl	5288(%r12), %eax
	.loc	1 1245 5
	cmpl	$1, %ecx
	je	.LBB40_105
	cmpl	$2, %ecx
	jne	.LBB40_108
	testb	%al, %al
	jne	.LBB40_109
	.loc	1 0 5
	movq	144(%rsp), %rsi
.Ltmp6270:
	.loc	1 1189 11 is_stmt 1
	testq	%rsi, %rsi
	je	.LBB40_595
.Ltmp6271:
	.loc	1 0 11 is_stmt 0
	movl	5280(%r12), %eax
	movl	%eax, 1820(%rsp)
	movq	5264(%r12), %rbx
	leaq	2624(%r12), %rax
	movq	%rax, 2328(%rsp)
	xorl	%r13d, %r13d
	movq	%rbx, 136(%rsp)
	jmp	.LBB40_301
.LBB40_105:
	.loc	1 1245 5 is_stmt 1
	testb	%al, %al
	jne	.LBB40_109
	.loc	1 0 5 is_stmt 0
	movq	144(%rsp), %rsi
.Ltmp6272:
	.loc	1 1189 11 is_stmt 1
	testq	%rsi, %rsi
	je	.LBB40_595
.Ltmp6273:
	.loc	1 0 11 is_stmt 0
	movl	5280(%r12), %eax
	movl	%eax, 1560(%rsp)
	movq	5264(%r12), %rbx
	leaq	2624(%r12), %rax
	movq	%rax, 2040(%rsp)
	xorl	%r15d, %r15d
	xorl	%eax, %eax
	movq	%rbx, 2240(%rsp)
	jmp	.LBB40_154
.LBB40_108:
	.loc	1 1245 5 is_stmt 1
	testb	%al, %al
	je	.LBB40_446
.LBB40_109:
	.loc	1 0 5 is_stmt 0
	movq	144(%rsp), %rsi
.Ltmp6274:
	.loc	1 1189 11 is_stmt 1
	testq	%rsi, %rsi
	je	.LBB40_595
	.loc	1 0 11 is_stmt 0
	movl	5280(%r12), %eax
	movl	%eax, 2144(%rsp)
	movq	5264(%r12), %rax
	movq	%rax, 64(%rsp)
	leaq	2624(%r12), %rax
	movq	%rax, 2336(%rsp)
	leaq	1408(%r12), %rax
	movq	%rax, 192(%rsp)
	leaq	4032(%r12), %rax
	movq	%rax, 1568(%rsp)
	leaq	2304(%r12), %rax
	movq	%rax, 1632(%rsp)
	leaq	4928(%r12), %rax
	movq	%rax, 1600(%rsp)
	xorl	%ebx, %ebx
	xorl	%r13d, %r13d
	jmp	.LBB40_113
	.p2align	4
.LBB40_111:
.Ltmp6275:
	.loc	1 1160 5 is_stmt 1
	vmovaps	3936(%rsp), %ymm0
	vmovaps	3968(%rsp), %ymm1
	vmovaps	4000(%rsp), %ymm2
	vmovaps	4032(%rsp), %ymm3
	movq	192(%rsp), %rax
	vmovaps	%ymm3, 96(%rax)
	vmovaps	%ymm2, 64(%rax)
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1161 5
	vmovaps	4064(%rsp), %ymm0
	vmovaps	4096(%rsp), %ymm1
	vmovaps	4128(%rsp), %ymm2
	vmovaps	4160(%rsp), %ymm3
	movq	1568(%rsp), %rax
	vmovaps	%ymm3, 96(%rax)
	vmovaps	%ymm2, 64(%rax)
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1162 5
	vmovaps	4192(%rsp), %ymm0
	vmovaps	4224(%rsp), %ymm1
	movq	1632(%rsp), %rax
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1163 5
	vmovdqa	4256(%rsp), %ymm0
	vmovaps	4288(%rsp), %ymm1
	movq	1600(%rsp), %rax
	vmovaps	%ymm1, 32(%rax)
	vmovdqa	%ymm0, (%rax)
	.loc	1 1164 5
	movq	%r9, 5272(%r12)
.Ltmp6276:
.LBB40_112:
	.loc	1 0 5 is_stmt 0
	movq	144(%rsp), %rsi
	movq	1760(%rsp), %r13
	.loc	1 1189 11 is_stmt 1
	cmpq	%rsi, %r13
	jae	.LBB40_595
.LBB40_113:
	.loc	1 1190 42
	subq	%r13, %rsi
	.loc	1 1190 29 is_stmt 0
	movq	%r12, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E12plan_segmentB5_
	movb	%dl, 1728(%rsp)
	movq	%rax, %r15
.Ltmp6277:
	.loc	1 1279 33 is_stmt 1
	vmovss	(%r12), %xmm0
.Ltmp6278:
	.loc	1 1192 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp6279:
	.loc	1 1279 33
	vmovss	160(%r12), %xmm0
.Ltmp6280:
	.loc	1 1192 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp6281:
	.loc	1 1279 33
	vmovss	320(%r12), %xmm0
.Ltmp6282:
	.loc	1 1192 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp6283:
	.loc	1 1279 33
	vmovss	480(%r12), %xmm0
.Ltmp6284:
	.loc	1 1192 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp6285:
	.loc	1 1279 33
	vmovss	640(%r12), %xmm0
.Ltmp6286:
	.loc	1 1192 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp6287:
	.loc	1 1279 33
	vmovss	800(%r12), %xmm0
.Ltmp6288:
	.loc	1 1192 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp6289:
	.loc	1 1279 33
	vmovss	960(%r12), %xmm0
.Ltmp6290:
	.loc	1 1192 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp6291:
	.loc	1 1279 33
	vmovss	1120(%r12), %xmm0
.Ltmp6292:
	.loc	1 1192 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp6293:
	.loc	1 1279 33
	vmovss	16(%r12), %xmm0
.Ltmp6294:
	.loc	1 1192 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp6295:
	.loc	1 1279 33
	vmovss	176(%r12), %xmm0
.Ltmp6296:
	.loc	1 1192 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp6297:
	.loc	1 1279 33
	vmovss	336(%r12), %xmm0
.Ltmp6298:
	.loc	1 1192 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp6299:
	.loc	1 1279 33
	vmovss	496(%r12), %xmm0
.Ltmp6300:
	.loc	1 1192 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp6301:
	.loc	1 1279 33
	vmovss	656(%r12), %xmm0
.Ltmp6302:
	.loc	1 1192 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp6303:
	.loc	1 1279 33
	vmovss	816(%r12), %xmm0
.Ltmp6304:
	.loc	1 1192 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp6305:
	.loc	1 1279 33
	vmovss	976(%r12), %xmm0
.Ltmp6306:
	.loc	1 1192 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp6307:
	.loc	1 1279 33
	vmovss	1136(%r12), %xmm0
.Ltmp6308:
	.loc	1 1192 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp6309:
	.loc	1 1279 33
	vmovss	32(%r12), %xmm0
.Ltmp6310:
	.loc	1 1192 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp6311:
	.loc	1 1279 33
	vmovss	192(%r12), %xmm0
.Ltmp6312:
	.loc	1 1192 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp6313:
	.loc	1 1279 33
	vmovss	352(%r12), %xmm0
.Ltmp6314:
	.loc	1 1192 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp6315:
	.loc	1 1279 33
	vmovss	512(%r12), %xmm0
.Ltmp6316:
	.loc	1 1192 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp6317:
	.loc	1 1279 33
	vmovss	672(%r12), %xmm0
.Ltmp6318:
	.loc	1 1192 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp6319:
	.loc	1 1279 33
	vmovss	832(%r12), %xmm0
.Ltmp6320:
	.loc	1 1192 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp6321:
	.loc	1 1279 33
	vmovss	992(%r12), %xmm0
.Ltmp6322:
	.loc	1 1192 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp6323:
	.loc	1 1279 33
	vmovss	1152(%r12), %xmm0
.Ltmp6324:
	.loc	1 1192 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp6325:
	.loc	1 1279 33
	vmovss	48(%r12), %xmm0
.Ltmp6326:
	.loc	1 1192 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp6327:
	.loc	1 1279 33
	vmovss	208(%r12), %xmm0
.Ltmp6328:
	.loc	1 1192 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp6329:
	.loc	1 1279 33
	vmovss	368(%r12), %xmm0
.Ltmp6330:
	.loc	1 1192 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp6331:
	.loc	1 1279 33
	vmovss	528(%r12), %xmm0
.Ltmp6332:
	.loc	1 1192 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp6333:
	.loc	1 1279 33
	vmovss	688(%r12), %xmm0
.Ltmp6334:
	.loc	1 1192 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp6335:
	.loc	1 1279 33
	vmovss	848(%r12), %xmm0
.Ltmp6336:
	.loc	1 1192 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp6337:
	.loc	1 1279 33
	vmovss	1008(%r12), %xmm0
.Ltmp6338:
	.loc	1 1192 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp6339:
	.loc	1 1279 33
	vmovss	1168(%r12), %xmm0
.Ltmp6340:
	.loc	1 1192 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp6341:
	.loc	1 1279 33
	vmovss	64(%r12), %xmm0
.Ltmp6342:
	.loc	1 1192 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp6343:
	.loc	1 1279 33
	vmovss	224(%r12), %xmm0
.Ltmp6344:
	.loc	1 1192 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp6345:
	.loc	1 1279 33
	vmovss	384(%r12), %xmm0
.Ltmp6346:
	.loc	1 1192 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp6347:
	.loc	1 1279 33
	vmovss	544(%r12), %xmm0
.Ltmp6348:
	.loc	1 1192 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp6349:
	.loc	1 1279 33
	vmovss	704(%r12), %xmm0
.Ltmp6350:
	.loc	1 1192 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp6351:
	.loc	1 1279 33
	vmovss	864(%r12), %xmm0
.Ltmp6352:
	.loc	1 1192 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp6353:
	.loc	1 1279 33
	vmovss	1024(%r12), %xmm0
.Ltmp6354:
	.loc	1 1192 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp6355:
	.loc	1 1279 33
	vmovss	1184(%r12), %xmm0
.Ltmp6356:
	.loc	1 1192 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp6357:
	.loc	1 1279 33
	vmovss	80(%r12), %xmm0
.Ltmp6358:
	.loc	1 1192 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp6359:
	.loc	1 1279 33
	vmovss	240(%r12), %xmm0
.Ltmp6360:
	.loc	1 1192 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp6361:
	.loc	1 1279 33
	vmovss	400(%r12), %xmm0
.Ltmp6362:
	.loc	1 1192 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp6363:
	.loc	1 1279 33
	vmovss	560(%r12), %xmm0
.Ltmp6364:
	.loc	1 1192 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp6365:
	.loc	1 1279 33
	vmovss	720(%r12), %xmm0
.Ltmp6366:
	.loc	1 1192 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp6367:
	.loc	1 1279 33
	vmovss	880(%r12), %xmm0
.Ltmp6368:
	.loc	1 1192 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp6369:
	.loc	1 1279 33
	vmovss	1040(%r12), %xmm0
.Ltmp6370:
	.loc	1 1192 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp6371:
	.loc	1 1279 33
	vmovss	1200(%r12), %xmm0
.Ltmp6372:
	.loc	1 1192 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp6373:
	.loc	1 1279 33
	vmovss	96(%r12), %xmm0
.Ltmp6374:
	.loc	1 1192 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp6375:
	.loc	1 1279 33
	vmovss	256(%r12), %xmm0
.Ltmp6376:
	.loc	1 1192 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp6377:
	.loc	1 1279 33
	vmovss	416(%r12), %xmm0
.Ltmp6378:
	.loc	1 1192 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp6379:
	.loc	1 1279 33
	vmovss	576(%r12), %xmm0
.Ltmp6380:
	.loc	1 1192 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp6381:
	.loc	1 1279 33
	vmovss	736(%r12), %xmm0
.Ltmp6382:
	.loc	1 1192 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp6383:
	.loc	1 1279 33
	vmovss	896(%r12), %xmm0
.Ltmp6384:
	.loc	1 1192 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp6385:
	.loc	1 1279 33
	vmovss	1056(%r12), %xmm0
.Ltmp6386:
	.loc	1 1192 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp6387:
	.loc	1 1279 33
	vmovss	1216(%r12), %xmm0
.Ltmp6388:
	.loc	1 1192 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp6389:
	.loc	1 1279 33
	vmovss	112(%r12), %xmm0
.Ltmp6390:
	.loc	1 1192 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp6391:
	.loc	1 1279 33
	vmovss	272(%r12), %xmm0
.Ltmp6392:
	.loc	1 1192 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp6393:
	.loc	1 1279 33
	vmovss	432(%r12), %xmm0
.Ltmp6394:
	.loc	1 1192 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp6395:
	.loc	1 1279 33
	vmovss	592(%r12), %xmm0
.Ltmp6396:
	.loc	1 1192 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp6397:
	.loc	1 1279 33
	vmovss	752(%r12), %xmm0
.Ltmp6398:
	.loc	1 1192 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp6399:
	.loc	1 1279 33
	vmovss	912(%r12), %xmm0
.Ltmp6400:
	.loc	1 1192 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp6401:
	.loc	1 1279 33
	vmovss	1072(%r12), %xmm0
.Ltmp6402:
	.loc	1 1192 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp6403:
	.loc	1 1279 33
	vmovss	1232(%r12), %xmm0
.Ltmp6404:
	.loc	1 1192 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp6405:
	.loc	1 1279 33
	vmovss	128(%r12), %xmm0
.Ltmp6406:
	.loc	1 1192 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp6407:
	.loc	1 1279 33
	vmovss	288(%r12), %xmm0
.Ltmp6408:
	.loc	1 1192 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp6409:
	.loc	1 1279 33
	vmovss	448(%r12), %xmm0
.Ltmp6410:
	.loc	1 1192 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp6411:
	.loc	1 1279 33
	vmovss	608(%r12), %xmm0
.Ltmp6412:
	.loc	1 1192 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp6413:
	.loc	1 1279 33
	vmovss	768(%r12), %xmm0
.Ltmp6414:
	.loc	1 1192 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp6415:
	.loc	1 1279 33
	vmovss	928(%r12), %xmm0
.Ltmp6416:
	.loc	1 1192 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp6417:
	.loc	1 1279 33
	vmovss	1088(%r12), %xmm0
.Ltmp6418:
	.loc	1 1192 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp6419:
	.loc	1 1279 33
	vmovss	1248(%r12), %xmm0
.Ltmp6420:
	.loc	1 1192 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp6421:
	.loc	1 1279 33
	vmovss	144(%r12), %xmm0
.Ltmp6422:
	.loc	1 1192 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp6423:
	.loc	1 1279 33
	vmovss	304(%r12), %xmm0
.Ltmp6424:
	.loc	1 1192 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp6425:
	.loc	1 1279 33
	vmovss	464(%r12), %xmm0
.Ltmp6426:
	.loc	1 1192 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp6427:
	.loc	1 1279 33
	vmovss	624(%r12), %xmm0
.Ltmp6428:
	.loc	1 1192 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp6429:
	.loc	1 1279 33
	vmovss	784(%r12), %xmm0
.Ltmp6430:
	.loc	1 1192 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp6431:
	.loc	1 1279 33
	vmovss	944(%r12), %xmm0
.Ltmp6432:
	.loc	1 1192 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp6433:
	.loc	1 1279 33
	vmovss	1104(%r12), %xmm0
.Ltmp6434:
	.loc	1 1192 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp6435:
	.loc	1 1279 33
	vmovss	1264(%r12), %xmm0
.Ltmp6436:
	.loc	1 1192 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp6437:
	.loc	1 1280 32
	vmovss	8(%r12), %xmm0
.Ltmp6438:
	.loc	1 1192 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp6439:
	.loc	1 1280 32
	vmovss	168(%r12), %xmm0
.Ltmp6440:
	.loc	1 1192 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp6441:
	.loc	1 1280 32
	vmovss	328(%r12), %xmm0
.Ltmp6442:
	.loc	1 1192 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp6443:
	.loc	1 1280 32
	vmovss	488(%r12), %xmm0
.Ltmp6444:
	.loc	1 1192 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp6445:
	.loc	1 1280 32
	vmovss	648(%r12), %xmm0
.Ltmp6446:
	.loc	1 1192 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp6447:
	.loc	1 1280 32
	vmovss	808(%r12), %xmm0
.Ltmp6448:
	.loc	1 1192 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp6449:
	.loc	1 1280 32
	vmovss	968(%r12), %xmm0
.Ltmp6450:
	.loc	1 1192 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp6451:
	.loc	1 1280 32
	vmovss	1128(%r12), %xmm0
.Ltmp6452:
	.loc	1 1192 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp6453:
	.loc	1 1280 32
	vmovss	24(%r12), %xmm0
.Ltmp6454:
	.loc	1 1192 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp6455:
	.loc	1 1280 32
	vmovss	184(%r12), %xmm0
.Ltmp6456:
	.loc	1 1192 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp6457:
	.loc	1 1280 32
	vmovss	344(%r12), %xmm0
.Ltmp6458:
	.loc	1 1192 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp6459:
	.loc	1 1280 32
	vmovss	504(%r12), %xmm0
.Ltmp6460:
	.loc	1 1192 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp6461:
	.loc	1 1280 32
	vmovss	664(%r12), %xmm0
.Ltmp6462:
	.loc	1 1192 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp6463:
	.loc	1 1280 32
	vmovss	824(%r12), %xmm0
.Ltmp6464:
	.loc	1 1192 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp6465:
	.loc	1 1280 32
	vmovss	984(%r12), %xmm0
.Ltmp6466:
	.loc	1 1192 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp6467:
	.loc	1 1280 32
	vmovss	1144(%r12), %xmm0
.Ltmp6468:
	.loc	1 1192 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp6469:
	.loc	1 1280 32
	vmovss	40(%r12), %xmm0
.Ltmp6470:
	.loc	1 1192 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp6471:
	.loc	1 1280 32
	vmovss	200(%r12), %xmm0
.Ltmp6472:
	.loc	1 1192 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp6473:
	.loc	1 1280 32
	vmovss	360(%r12), %xmm0
.Ltmp6474:
	.loc	1 1192 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp6475:
	.loc	1 1280 32
	vmovss	520(%r12), %xmm0
.Ltmp6476:
	.loc	1 1192 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp6477:
	.loc	1 1280 32
	vmovss	680(%r12), %xmm0
.Ltmp6478:
	.loc	1 1192 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp6479:
	.loc	1 1280 32
	vmovss	840(%r12), %xmm0
.Ltmp6480:
	.loc	1 1192 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp6481:
	.loc	1 1280 32
	vmovss	1000(%r12), %xmm0
.Ltmp6482:
	.loc	1 1192 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp6483:
	.loc	1 1280 32
	vmovss	1160(%r12), %xmm0
.Ltmp6484:
	.loc	1 1192 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp6485:
	.loc	1 1280 32
	vmovss	56(%r12), %xmm0
.Ltmp6486:
	.loc	1 1192 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp6487:
	.loc	1 1280 32
	vmovss	216(%r12), %xmm0
.Ltmp6488:
	.loc	1 1192 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp6489:
	.loc	1 1280 32
	vmovss	376(%r12), %xmm0
.Ltmp6490:
	.loc	1 1192 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp6491:
	.loc	1 1280 32
	vmovss	536(%r12), %xmm0
.Ltmp6492:
	.loc	1 1192 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp6493:
	.loc	1 1280 32
	vmovss	696(%r12), %xmm0
.Ltmp6494:
	.loc	1 1192 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp6495:
	.loc	1 1280 32
	vmovss	856(%r12), %xmm0
.Ltmp6496:
	.loc	1 1192 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp6497:
	.loc	1 1280 32
	vmovss	1016(%r12), %xmm0
.Ltmp6498:
	.loc	1 1192 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp6499:
	.loc	1 1280 32
	vmovss	1176(%r12), %xmm0
.Ltmp6500:
	.loc	1 1192 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp6501:
	.loc	1 1280 32
	vmovss	72(%r12), %xmm0
.Ltmp6502:
	.loc	1 1192 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp6503:
	.loc	1 1280 32
	vmovss	232(%r12), %xmm0
.Ltmp6504:
	.loc	1 1192 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp6505:
	.loc	1 1280 32
	vmovss	392(%r12), %xmm0
.Ltmp6506:
	.loc	1 1192 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp6507:
	.loc	1 1280 32
	vmovss	552(%r12), %xmm0
.Ltmp6508:
	.loc	1 1192 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp6509:
	.loc	1 1280 32
	vmovss	712(%r12), %xmm0
.Ltmp6510:
	.loc	1 1192 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp6511:
	.loc	1 1280 32
	vmovss	872(%r12), %xmm0
.Ltmp6512:
	.loc	1 1192 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp6513:
	.loc	1 1280 32
	vmovss	1032(%r12), %xmm0
.Ltmp6514:
	.loc	1 1192 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp6515:
	.loc	1 1280 32
	vmovss	1192(%r12), %xmm0
.Ltmp6516:
	.loc	1 1192 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp6517:
	.loc	1 1280 32
	vmovss	88(%r12), %xmm0
.Ltmp6518:
	.loc	1 1192 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp6519:
	.loc	1 1280 32
	vmovss	248(%r12), %xmm0
.Ltmp6520:
	.loc	1 1192 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp6521:
	.loc	1 1280 32
	vmovss	408(%r12), %xmm0
.Ltmp6522:
	.loc	1 1192 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp6523:
	.loc	1 1280 32
	vmovss	568(%r12), %xmm0
.Ltmp6524:
	.loc	1 1192 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp6525:
	.loc	1 1280 32
	vmovss	728(%r12), %xmm0
.Ltmp6526:
	.loc	1 1192 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp6527:
	.loc	1 1280 32
	vmovss	888(%r12), %xmm0
.Ltmp6528:
	.loc	1 1192 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp6529:
	.loc	1 1280 32
	vmovss	1048(%r12), %xmm0
.Ltmp6530:
	.loc	1 1192 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp6531:
	.loc	1 1280 32
	vmovss	1208(%r12), %xmm0
.Ltmp6532:
	.loc	1 1192 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp6533:
	.loc	1 1280 32
	vmovss	104(%r12), %xmm0
.Ltmp6534:
	.loc	1 1192 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp6535:
	.loc	1 1280 32
	vmovss	264(%r12), %xmm0
.Ltmp6536:
	.loc	1 1192 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp6537:
	.loc	1 1280 32
	vmovss	424(%r12), %xmm0
.Ltmp6538:
	.loc	1 1192 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp6539:
	.loc	1 1280 32
	vmovss	584(%r12), %xmm0
.Ltmp6540:
	.loc	1 1192 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp6541:
	.loc	1 1280 32
	vmovss	744(%r12), %xmm0
.Ltmp6542:
	.loc	1 1192 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp6543:
	.loc	1 1280 32
	vmovss	904(%r12), %xmm0
.Ltmp6544:
	.loc	1 1192 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp6545:
	.loc	1 1280 32
	vmovss	1064(%r12), %xmm0
.Ltmp6546:
	.loc	1 1192 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp6547:
	.loc	1 1280 32
	vmovss	1224(%r12), %xmm0
.Ltmp6548:
	.loc	1 1192 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp6549:
	.loc	1 1280 32
	vmovss	120(%r12), %xmm0
.Ltmp6550:
	.loc	1 1192 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp6551:
	.loc	1 1280 32
	vmovss	280(%r12), %xmm0
.Ltmp6552:
	.loc	1 1192 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp6553:
	.loc	1 1280 32
	vmovss	440(%r12), %xmm0
.Ltmp6554:
	.loc	1 1192 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp6555:
	.loc	1 1280 32
	vmovss	600(%r12), %xmm0
.Ltmp6556:
	.loc	1 1192 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp6557:
	.loc	1 1280 32
	vmovss	760(%r12), %xmm0
.Ltmp6558:
	.loc	1 1192 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp6559:
	.loc	1 1280 32
	vmovss	920(%r12), %xmm0
.Ltmp6560:
	.loc	1 1192 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp6561:
	.loc	1 1280 32
	vmovss	1080(%r12), %xmm0
.Ltmp6562:
	.loc	1 1192 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp6563:
	.loc	1 1280 32
	vmovss	1240(%r12), %xmm0
.Ltmp6564:
	.loc	1 1192 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp6565:
	.loc	1 1280 32
	vmovss	136(%r12), %xmm0
.Ltmp6566:
	.loc	1 1192 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp6567:
	.loc	1 1280 32
	vmovss	296(%r12), %xmm0
.Ltmp6568:
	.loc	1 1192 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp6569:
	.loc	1 1280 32
	vmovss	456(%r12), %xmm0
.Ltmp6570:
	.loc	1 1192 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp6571:
	.loc	1 1280 32
	vmovss	616(%r12), %xmm0
.Ltmp6572:
	.loc	1 1192 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp6573:
	.loc	1 1280 32
	vmovss	776(%r12), %xmm0
.Ltmp6574:
	.loc	1 1192 28
	vmovss	%xmm0, 848(%rsp)
.Ltmp6575:
	.loc	1 1280 32
	vmovss	936(%r12), %xmm0
.Ltmp6576:
	.loc	1 1192 28
	vmovss	%xmm0, 852(%rsp)
.Ltmp6577:
	.loc	1 1280 32
	vmovss	1096(%r12), %xmm0
.Ltmp6578:
	.loc	1 1192 28
	vmovss	%xmm0, 856(%rsp)
.Ltmp6579:
	.loc	1 1280 32
	vmovss	1256(%r12), %xmm0
.Ltmp6580:
	.loc	1 1192 28
	vmovss	%xmm0, 860(%rsp)
.Ltmp6581:
	.loc	1 1280 32
	vmovss	152(%r12), %xmm0
.Ltmp6582:
	.loc	1 1192 28
	vmovss	%xmm0, 864(%rsp)
.Ltmp6583:
	.loc	1 1280 32
	vmovss	312(%r12), %xmm0
.Ltmp6584:
	.loc	1 1192 28
	vmovss	%xmm0, 868(%rsp)
.Ltmp6585:
	.loc	1 1280 32
	vmovss	472(%r12), %xmm0
.Ltmp6586:
	.loc	1 1192 28
	vmovss	%xmm0, 872(%rsp)
.Ltmp6587:
	.loc	1 1280 32
	vmovss	632(%r12), %xmm0
.Ltmp6588:
	.loc	1 1192 28
	vmovss	%xmm0, 876(%rsp)
.Ltmp6589:
	.loc	1 1280 32
	vmovss	792(%r12), %xmm0
.Ltmp6590:
	.loc	1 1192 28
	vmovss	%xmm0, 880(%rsp)
.Ltmp6591:
	.loc	1 1280 32
	vmovss	952(%r12), %xmm0
.Ltmp6592:
	.loc	1 1192 28
	vmovss	%xmm0, 884(%rsp)
.Ltmp6593:
	.loc	1 1280 32
	vmovss	1112(%r12), %xmm0
.Ltmp6594:
	.loc	1 1192 28
	vmovss	%xmm0, 888(%rsp)
.Ltmp6595:
	.loc	1 1280 32
	vmovss	1272(%r12), %xmm0
.Ltmp6596:
	.loc	1 1192 28
	vmovss	%xmm0, 892(%rsp)
.Ltmp6597:
	.loc	1 1279 33
	vmovss	2624(%r12), %xmm0
.Ltmp6598:
	.loc	1 1192 28
	vmovss	%xmm0, 896(%rsp)
.Ltmp6599:
	.loc	1 1279 33
	vmovss	2784(%r12), %xmm0
.Ltmp6600:
	.loc	1 1192 28
	vmovss	%xmm0, 900(%rsp)
.Ltmp6601:
	.loc	1 1279 33
	vmovss	2944(%r12), %xmm0
.Ltmp6602:
	.loc	1 1192 28
	vmovss	%xmm0, 904(%rsp)
.Ltmp6603:
	.loc	1 1279 33
	vmovss	3104(%r12), %xmm0
.Ltmp6604:
	.loc	1 1192 28
	vmovss	%xmm0, 908(%rsp)
.Ltmp6605:
	.loc	1 1279 33
	vmovss	3264(%r12), %xmm0
.Ltmp6606:
	.loc	1 1192 28
	vmovss	%xmm0, 912(%rsp)
.Ltmp6607:
	.loc	1 1279 33
	vmovss	3424(%r12), %xmm0
.Ltmp6608:
	.loc	1 1192 28
	vmovss	%xmm0, 916(%rsp)
.Ltmp6609:
	.loc	1 1279 33
	vmovss	3584(%r12), %xmm0
.Ltmp6610:
	.loc	1 1192 28
	vmovss	%xmm0, 920(%rsp)
.Ltmp6611:
	.loc	1 1279 33
	vmovss	3744(%r12), %xmm0
.Ltmp6612:
	.loc	1 1192 28
	vmovss	%xmm0, 924(%rsp)
.Ltmp6613:
	.loc	1 1279 33
	vmovss	2640(%r12), %xmm0
.Ltmp6614:
	.loc	1 1192 28
	vmovss	%xmm0, 928(%rsp)
.Ltmp6615:
	.loc	1 1279 33
	vmovss	2800(%r12), %xmm0
.Ltmp6616:
	.loc	1 1192 28
	vmovss	%xmm0, 932(%rsp)
.Ltmp6617:
	.loc	1 1279 33
	vmovss	2960(%r12), %xmm0
.Ltmp6618:
	.loc	1 1192 28
	vmovss	%xmm0, 936(%rsp)
.Ltmp6619:
	.loc	1 1279 33
	vmovss	3120(%r12), %xmm0
.Ltmp6620:
	.loc	1 1192 28
	vmovss	%xmm0, 940(%rsp)
.Ltmp6621:
	.loc	1 1279 33
	vmovss	3280(%r12), %xmm0
.Ltmp6622:
	.loc	1 1192 28
	vmovss	%xmm0, 944(%rsp)
.Ltmp6623:
	.loc	1 1279 33
	vmovss	3440(%r12), %xmm0
.Ltmp6624:
	.loc	1 1192 28
	vmovss	%xmm0, 948(%rsp)
.Ltmp6625:
	.loc	1 1279 33
	vmovss	3600(%r12), %xmm0
.Ltmp6626:
	.loc	1 1192 28
	vmovss	%xmm0, 952(%rsp)
.Ltmp6627:
	.loc	1 1279 33
	vmovss	3760(%r12), %xmm0
.Ltmp6628:
	.loc	1 1192 28
	vmovss	%xmm0, 956(%rsp)
.Ltmp6629:
	.loc	1 1279 33
	vmovss	2656(%r12), %xmm0
.Ltmp6630:
	.loc	1 1192 28
	vmovss	%xmm0, 960(%rsp)
.Ltmp6631:
	.loc	1 1279 33
	vmovss	2816(%r12), %xmm0
.Ltmp6632:
	.loc	1 1192 28
	vmovss	%xmm0, 964(%rsp)
.Ltmp6633:
	.loc	1 1279 33
	vmovss	2976(%r12), %xmm0
.Ltmp6634:
	.loc	1 1192 28
	vmovss	%xmm0, 968(%rsp)
.Ltmp6635:
	.loc	1 1279 33
	vmovss	3136(%r12), %xmm0
.Ltmp6636:
	.loc	1 1192 28
	vmovss	%xmm0, 972(%rsp)
.Ltmp6637:
	.loc	1 1279 33
	vmovss	3296(%r12), %xmm0
.Ltmp6638:
	.loc	1 1192 28
	vmovss	%xmm0, 976(%rsp)
.Ltmp6639:
	.loc	1 1279 33
	vmovss	3456(%r12), %xmm0
.Ltmp6640:
	.loc	1 1192 28
	vmovss	%xmm0, 980(%rsp)
.Ltmp6641:
	.loc	1 1279 33
	vmovss	3616(%r12), %xmm0
.Ltmp6642:
	.loc	1 1192 28
	vmovss	%xmm0, 984(%rsp)
.Ltmp6643:
	.loc	1 1279 33
	vmovss	3776(%r12), %xmm0
.Ltmp6644:
	.loc	1 1192 28
	vmovss	%xmm0, 988(%rsp)
.Ltmp6645:
	.loc	1 1279 33
	vmovss	2672(%r12), %xmm0
.Ltmp6646:
	.loc	1 1192 28
	vmovss	%xmm0, 992(%rsp)
.Ltmp6647:
	.loc	1 1279 33
	vmovss	2832(%r12), %xmm0
.Ltmp6648:
	.loc	1 1192 28
	vmovss	%xmm0, 996(%rsp)
.Ltmp6649:
	.loc	1 1279 33
	vmovss	2992(%r12), %xmm0
.Ltmp6650:
	.loc	1 1192 28
	vmovss	%xmm0, 1000(%rsp)
.Ltmp6651:
	.loc	1 1279 33
	vmovss	3152(%r12), %xmm0
.Ltmp6652:
	.loc	1 1192 28
	vmovss	%xmm0, 1004(%rsp)
.Ltmp6653:
	.loc	1 1279 33
	vmovss	3312(%r12), %xmm0
.Ltmp6654:
	.loc	1 1192 28
	vmovss	%xmm0, 1008(%rsp)
.Ltmp6655:
	.loc	1 1279 33
	vmovss	3472(%r12), %xmm0
.Ltmp6656:
	.loc	1 1192 28
	vmovss	%xmm0, 1012(%rsp)
.Ltmp6657:
	.loc	1 1279 33
	vmovss	3632(%r12), %xmm0
.Ltmp6658:
	.loc	1 1192 28
	vmovss	%xmm0, 1016(%rsp)
.Ltmp6659:
	.loc	1 1279 33
	vmovss	3792(%r12), %xmm0
.Ltmp6660:
	.loc	1 1192 28
	vmovss	%xmm0, 1020(%rsp)
.Ltmp6661:
	.loc	1 1279 33
	vmovss	2688(%r12), %xmm0
.Ltmp6662:
	.loc	1 1192 28
	vmovss	%xmm0, 1024(%rsp)
.Ltmp6663:
	.loc	1 1279 33
	vmovss	2848(%r12), %xmm0
.Ltmp6664:
	.loc	1 1192 28
	vmovss	%xmm0, 1028(%rsp)
.Ltmp6665:
	.loc	1 1279 33
	vmovss	3008(%r12), %xmm0
.Ltmp6666:
	.loc	1 1192 28
	vmovss	%xmm0, 1032(%rsp)
.Ltmp6667:
	.loc	1 1279 33
	vmovss	3168(%r12), %xmm0
.Ltmp6668:
	.loc	1 1192 28
	vmovss	%xmm0, 1036(%rsp)
.Ltmp6669:
	.loc	1 1279 33
	vmovss	3328(%r12), %xmm0
.Ltmp6670:
	.loc	1 1192 28
	vmovss	%xmm0, 1040(%rsp)
.Ltmp6671:
	.loc	1 1279 33
	vmovss	3488(%r12), %xmm0
.Ltmp6672:
	.loc	1 1192 28
	vmovss	%xmm0, 1044(%rsp)
.Ltmp6673:
	.loc	1 1279 33
	vmovss	3648(%r12), %xmm0
.Ltmp6674:
	.loc	1 1192 28
	vmovss	%xmm0, 1048(%rsp)
.Ltmp6675:
	.loc	1 1279 33
	vmovss	3808(%r12), %xmm0
.Ltmp6676:
	.loc	1 1192 28
	vmovss	%xmm0, 1052(%rsp)
.Ltmp6677:
	.loc	1 1279 33
	vmovss	2704(%r12), %xmm0
.Ltmp6678:
	.loc	1 1192 28
	vmovss	%xmm0, 1056(%rsp)
.Ltmp6679:
	.loc	1 1279 33
	vmovss	2864(%r12), %xmm0
.Ltmp6680:
	.loc	1 1192 28
	vmovss	%xmm0, 1060(%rsp)
.Ltmp6681:
	.loc	1 1279 33
	vmovss	3024(%r12), %xmm0
.Ltmp6682:
	.loc	1 1192 28
	vmovss	%xmm0, 1064(%rsp)
.Ltmp6683:
	.loc	1 1279 33
	vmovss	3184(%r12), %xmm0
.Ltmp6684:
	.loc	1 1192 28
	vmovss	%xmm0, 1068(%rsp)
.Ltmp6685:
	.loc	1 1279 33
	vmovss	3344(%r12), %xmm0
.Ltmp6686:
	.loc	1 1192 28
	vmovss	%xmm0, 1072(%rsp)
.Ltmp6687:
	.loc	1 1279 33
	vmovss	3504(%r12), %xmm0
.Ltmp6688:
	.loc	1 1192 28
	vmovss	%xmm0, 1076(%rsp)
.Ltmp6689:
	.loc	1 1279 33
	vmovss	3664(%r12), %xmm0
.Ltmp6690:
	.loc	1 1192 28
	vmovss	%xmm0, 1080(%rsp)
.Ltmp6691:
	.loc	1 1279 33
	vmovss	3824(%r12), %xmm0
.Ltmp6692:
	.loc	1 1192 28
	vmovss	%xmm0, 1084(%rsp)
.Ltmp6693:
	.loc	1 1279 33
	vmovss	2720(%r12), %xmm0
.Ltmp6694:
	.loc	1 1192 28
	vmovss	%xmm0, 1088(%rsp)
.Ltmp6695:
	.loc	1 1279 33
	vmovss	2880(%r12), %xmm0
.Ltmp6696:
	.loc	1 1192 28
	vmovss	%xmm0, 1092(%rsp)
.Ltmp6697:
	.loc	1 1279 33
	vmovss	3040(%r12), %xmm0
.Ltmp6698:
	.loc	1 1192 28
	vmovss	%xmm0, 1096(%rsp)
.Ltmp6699:
	.loc	1 1279 33
	vmovss	3200(%r12), %xmm0
.Ltmp6700:
	.loc	1 1192 28
	vmovss	%xmm0, 1100(%rsp)
.Ltmp6701:
	.loc	1 1279 33
	vmovss	3360(%r12), %xmm0
.Ltmp6702:
	.loc	1 1192 28
	vmovss	%xmm0, 1104(%rsp)
.Ltmp6703:
	.loc	1 1279 33
	vmovss	3520(%r12), %xmm0
.Ltmp6704:
	.loc	1 1192 28
	vmovss	%xmm0, 1108(%rsp)
.Ltmp6705:
	.loc	1 1279 33
	vmovss	3680(%r12), %xmm0
.Ltmp6706:
	.loc	1 1192 28
	vmovss	%xmm0, 1112(%rsp)
.Ltmp6707:
	.loc	1 1279 33
	vmovss	3840(%r12), %xmm0
.Ltmp6708:
	.loc	1 1192 28
	vmovss	%xmm0, 1116(%rsp)
.Ltmp6709:
	.loc	1 1279 33
	vmovss	2736(%r12), %xmm0
.Ltmp6710:
	.loc	1 1192 28
	vmovss	%xmm0, 1120(%rsp)
.Ltmp6711:
	.loc	1 1279 33
	vmovss	2896(%r12), %xmm0
.Ltmp6712:
	.loc	1 1192 28
	vmovss	%xmm0, 1124(%rsp)
.Ltmp6713:
	.loc	1 1279 33
	vmovss	3056(%r12), %xmm0
.Ltmp6714:
	.loc	1 1192 28
	vmovss	%xmm0, 1128(%rsp)
.Ltmp6715:
	.loc	1 1279 33
	vmovss	3216(%r12), %xmm0
.Ltmp6716:
	.loc	1 1192 28
	vmovss	%xmm0, 1132(%rsp)
.Ltmp6717:
	.loc	1 1279 33
	vmovss	3376(%r12), %xmm0
.Ltmp6718:
	.loc	1 1192 28
	vmovss	%xmm0, 1136(%rsp)
.Ltmp6719:
	.loc	1 1279 33
	vmovss	3536(%r12), %xmm0
.Ltmp6720:
	.loc	1 1192 28
	vmovss	%xmm0, 1140(%rsp)
.Ltmp6721:
	.loc	1 1279 33
	vmovss	3696(%r12), %xmm0
.Ltmp6722:
	.loc	1 1192 28
	vmovss	%xmm0, 1144(%rsp)
.Ltmp6723:
	.loc	1 1279 33
	vmovss	3856(%r12), %xmm0
.Ltmp6724:
	.loc	1 1192 28
	vmovss	%xmm0, 1148(%rsp)
.Ltmp6725:
	.loc	1 1279 33
	vmovss	2752(%r12), %xmm0
.Ltmp6726:
	.loc	1 1192 28
	vmovss	%xmm0, 1152(%rsp)
.Ltmp6727:
	.loc	1 1279 33
	vmovss	2912(%r12), %xmm0
.Ltmp6728:
	.loc	1 1192 28
	vmovss	%xmm0, 1156(%rsp)
.Ltmp6729:
	.loc	1 1279 33
	vmovss	3072(%r12), %xmm0
.Ltmp6730:
	.loc	1 1192 28
	vmovss	%xmm0, 1160(%rsp)
.Ltmp6731:
	.loc	1 1279 33
	vmovss	3232(%r12), %xmm0
.Ltmp6732:
	.loc	1 1192 28
	vmovss	%xmm0, 1164(%rsp)
.Ltmp6733:
	.loc	1 1279 33
	vmovss	3392(%r12), %xmm0
.Ltmp6734:
	.loc	1 1192 28
	vmovss	%xmm0, 1168(%rsp)
.Ltmp6735:
	.loc	1 1279 33
	vmovss	3552(%r12), %xmm0
.Ltmp6736:
	.loc	1 1192 28
	vmovss	%xmm0, 1172(%rsp)
.Ltmp6737:
	.loc	1 1279 33
	vmovss	3712(%r12), %xmm0
.Ltmp6738:
	.loc	1 1192 28
	vmovss	%xmm0, 1176(%rsp)
.Ltmp6739:
	.loc	1 1279 33
	vmovss	3872(%r12), %xmm0
.Ltmp6740:
	.loc	1 1192 28
	vmovss	%xmm0, 1180(%rsp)
.Ltmp6741:
	.loc	1 1279 33
	vmovss	2768(%r12), %xmm0
.Ltmp6742:
	.loc	1 1192 28
	vmovss	%xmm0, 1184(%rsp)
.Ltmp6743:
	.loc	1 1279 33
	vmovss	2928(%r12), %xmm0
.Ltmp6744:
	.loc	1 1192 28
	vmovss	%xmm0, 1188(%rsp)
.Ltmp6745:
	.loc	1 1279 33
	vmovss	3088(%r12), %xmm0
.Ltmp6746:
	.loc	1 1192 28
	vmovss	%xmm0, 1192(%rsp)
.Ltmp6747:
	.loc	1 1279 33
	vmovss	3248(%r12), %xmm0
.Ltmp6748:
	.loc	1 1192 28
	vmovss	%xmm0, 1196(%rsp)
.Ltmp6749:
	.loc	1 1279 33
	vmovss	3408(%r12), %xmm0
.Ltmp6750:
	.loc	1 1192 28
	vmovss	%xmm0, 1200(%rsp)
.Ltmp6751:
	.loc	1 1279 33
	vmovss	3568(%r12), %xmm0
.Ltmp6752:
	.loc	1 1192 28
	vmovss	%xmm0, 1204(%rsp)
.Ltmp6753:
	.loc	1 1279 33
	vmovss	3728(%r12), %xmm0
.Ltmp6754:
	.loc	1 1192 28
	vmovss	%xmm0, 1208(%rsp)
.Ltmp6755:
	.loc	1 1279 33
	vmovss	3888(%r12), %xmm0
.Ltmp6756:
	.loc	1 1192 28
	vmovss	%xmm0, 1212(%rsp)
.Ltmp6757:
	.loc	1 1280 32
	vmovss	2632(%r12), %xmm0
.Ltmp6758:
	.loc	1 1192 28
	vmovss	%xmm0, 1216(%rsp)
.Ltmp6759:
	.loc	1 1280 32
	vmovss	2792(%r12), %xmm0
.Ltmp6760:
	.loc	1 1192 28
	vmovss	%xmm0, 1220(%rsp)
.Ltmp6761:
	.loc	1 1280 32
	vmovss	2952(%r12), %xmm0
.Ltmp6762:
	.loc	1 1192 28
	vmovss	%xmm0, 1224(%rsp)
.Ltmp6763:
	.loc	1 1280 32
	vmovss	3112(%r12), %xmm0
.Ltmp6764:
	.loc	1 1192 28
	vmovss	%xmm0, 1228(%rsp)
.Ltmp6765:
	.loc	1 1280 32
	vmovss	3272(%r12), %xmm0
.Ltmp6766:
	.loc	1 1192 28
	vmovss	%xmm0, 1232(%rsp)
.Ltmp6767:
	.loc	1 1280 32
	vmovss	3432(%r12), %xmm0
.Ltmp6768:
	.loc	1 1192 28
	vmovss	%xmm0, 1236(%rsp)
.Ltmp6769:
	.loc	1 1280 32
	vmovss	3592(%r12), %xmm0
.Ltmp6770:
	.loc	1 1192 28
	vmovss	%xmm0, 1240(%rsp)
.Ltmp6771:
	.loc	1 1280 32
	vmovss	3752(%r12), %xmm0
.Ltmp6772:
	.loc	1 1192 28
	vmovss	%xmm0, 1244(%rsp)
.Ltmp6773:
	.loc	1 1280 32
	vmovss	2648(%r12), %xmm0
.Ltmp6774:
	.loc	1 1192 28
	vmovss	%xmm0, 1248(%rsp)
.Ltmp6775:
	.loc	1 1280 32
	vmovss	2808(%r12), %xmm0
.Ltmp6776:
	.loc	1 1192 28
	vmovss	%xmm0, 1252(%rsp)
.Ltmp6777:
	.loc	1 1280 32
	vmovss	2968(%r12), %xmm0
.Ltmp6778:
	.loc	1 1192 28
	vmovss	%xmm0, 1256(%rsp)
.Ltmp6779:
	.loc	1 1280 32
	vmovss	3128(%r12), %xmm0
.Ltmp6780:
	.loc	1 1192 28
	vmovss	%xmm0, 1260(%rsp)
.Ltmp6781:
	.loc	1 1280 32
	vmovss	3288(%r12), %xmm0
.Ltmp6782:
	.loc	1 1192 28
	vmovss	%xmm0, 1264(%rsp)
.Ltmp6783:
	.loc	1 1280 32
	vmovss	3448(%r12), %xmm0
.Ltmp6784:
	.loc	1 1192 28
	vmovss	%xmm0, 1268(%rsp)
.Ltmp6785:
	.loc	1 1280 32
	vmovss	3608(%r12), %xmm0
.Ltmp6786:
	.loc	1 1192 28
	vmovss	%xmm0, 1272(%rsp)
.Ltmp6787:
	.loc	1 1280 32
	vmovss	3768(%r12), %xmm0
.Ltmp6788:
	.loc	1 1192 28
	vmovss	%xmm0, 1276(%rsp)
.Ltmp6789:
	.loc	1 1280 32
	vmovss	2664(%r12), %xmm0
.Ltmp6790:
	.loc	1 1192 28
	vmovss	%xmm0, 1280(%rsp)
.Ltmp6791:
	.loc	1 1280 32
	vmovss	2824(%r12), %xmm0
.Ltmp6792:
	.loc	1 1192 28
	vmovss	%xmm0, 1284(%rsp)
.Ltmp6793:
	.loc	1 1280 32
	vmovss	2984(%r12), %xmm0
.Ltmp6794:
	.loc	1 1192 28
	vmovss	%xmm0, 1288(%rsp)
.Ltmp6795:
	.loc	1 1280 32
	vmovss	3144(%r12), %xmm0
.Ltmp6796:
	.loc	1 1192 28
	vmovss	%xmm0, 1292(%rsp)
.Ltmp6797:
	.loc	1 1280 32
	vmovss	3304(%r12), %xmm0
.Ltmp6798:
	.loc	1 1192 28
	vmovss	%xmm0, 1296(%rsp)
.Ltmp6799:
	.loc	1 1280 32
	vmovss	3464(%r12), %xmm0
.Ltmp6800:
	.loc	1 1192 28
	vmovss	%xmm0, 1300(%rsp)
.Ltmp6801:
	.loc	1 1280 32
	vmovss	3624(%r12), %xmm0
.Ltmp6802:
	.loc	1 1192 28
	vmovss	%xmm0, 1304(%rsp)
.Ltmp6803:
	.loc	1 1280 32
	vmovss	3784(%r12), %xmm0
.Ltmp6804:
	.loc	1 1192 28
	vmovss	%xmm0, 1308(%rsp)
.Ltmp6805:
	.loc	1 1280 32
	vmovss	2680(%r12), %xmm0
.Ltmp6806:
	.loc	1 1192 28
	vmovss	%xmm0, 1312(%rsp)
.Ltmp6807:
	.loc	1 1280 32
	vmovss	2840(%r12), %xmm0
.Ltmp6808:
	.loc	1 1192 28
	vmovss	%xmm0, 1316(%rsp)
.Ltmp6809:
	.loc	1 1280 32
	vmovss	3000(%r12), %xmm0
.Ltmp6810:
	.loc	1 1192 28
	vmovss	%xmm0, 1320(%rsp)
.Ltmp6811:
	.loc	1 1280 32
	vmovss	3160(%r12), %xmm0
.Ltmp6812:
	.loc	1 1192 28
	vmovss	%xmm0, 1324(%rsp)
.Ltmp6813:
	.loc	1 1280 32
	vmovss	3320(%r12), %xmm0
.Ltmp6814:
	.loc	1 1192 28
	vmovss	%xmm0, 1328(%rsp)
.Ltmp6815:
	.loc	1 1280 32
	vmovss	3480(%r12), %xmm0
.Ltmp6816:
	.loc	1 1192 28
	vmovss	%xmm0, 1332(%rsp)
.Ltmp6817:
	.loc	1 1280 32
	vmovss	3640(%r12), %xmm0
.Ltmp6818:
	.loc	1 1192 28
	vmovss	%xmm0, 1336(%rsp)
.Ltmp6819:
	.loc	1 1280 32
	vmovss	3800(%r12), %xmm0
.Ltmp6820:
	.loc	1 1192 28
	vmovss	%xmm0, 1340(%rsp)
.Ltmp6821:
	.loc	1 1280 32
	vmovss	2696(%r12), %xmm0
.Ltmp6822:
	.loc	1 1192 28
	vmovss	%xmm0, 1344(%rsp)
.Ltmp6823:
	.loc	1 1280 32
	vmovss	2856(%r12), %xmm0
.Ltmp6824:
	.loc	1 1192 28
	vmovss	%xmm0, 1348(%rsp)
.Ltmp6825:
	.loc	1 1280 32
	vmovss	3016(%r12), %xmm0
.Ltmp6826:
	.loc	1 1192 28
	vmovss	%xmm0, 1352(%rsp)
.Ltmp6827:
	.loc	1 1280 32
	vmovss	3176(%r12), %xmm0
.Ltmp6828:
	.loc	1 1192 28
	vmovss	%xmm0, 1356(%rsp)
.Ltmp6829:
	.loc	1 1280 32
	vmovss	3336(%r12), %xmm0
.Ltmp6830:
	.loc	1 1192 28
	vmovss	%xmm0, 1360(%rsp)
.Ltmp6831:
	.loc	1 1280 32
	vmovss	3496(%r12), %xmm0
.Ltmp6832:
	.loc	1 1192 28
	vmovss	%xmm0, 1364(%rsp)
.Ltmp6833:
	.loc	1 1280 32
	vmovss	3656(%r12), %xmm0
.Ltmp6834:
	.loc	1 1192 28
	vmovss	%xmm0, 1368(%rsp)
.Ltmp6835:
	.loc	1 1280 32
	vmovss	3816(%r12), %xmm0
.Ltmp6836:
	.loc	1 1192 28
	vmovss	%xmm0, 1372(%rsp)
.Ltmp6837:
	.loc	1 1280 32
	vmovss	2712(%r12), %xmm0
.Ltmp6838:
	.loc	1 1192 28
	vmovss	%xmm0, 1376(%rsp)
.Ltmp6839:
	.loc	1 1280 32
	vmovss	2872(%r12), %xmm0
.Ltmp6840:
	.loc	1 1192 28
	vmovss	%xmm0, 1380(%rsp)
.Ltmp6841:
	.loc	1 1280 32
	vmovss	3032(%r12), %xmm0
.Ltmp6842:
	.loc	1 1192 28
	vmovss	%xmm0, 1384(%rsp)
.Ltmp6843:
	.loc	1 1280 32
	vmovss	3192(%r12), %xmm0
.Ltmp6844:
	.loc	1 1192 28
	vmovss	%xmm0, 1388(%rsp)
.Ltmp6845:
	.loc	1 1280 32
	vmovss	3352(%r12), %xmm0
.Ltmp6846:
	.loc	1 1192 28
	vmovss	%xmm0, 1392(%rsp)
.Ltmp6847:
	.loc	1 1280 32
	vmovss	3512(%r12), %xmm0
.Ltmp6848:
	.loc	1 1192 28
	vmovss	%xmm0, 1396(%rsp)
.Ltmp6849:
	.loc	1 1280 32
	vmovss	3672(%r12), %xmm0
.Ltmp6850:
	.loc	1 1192 28
	vmovss	%xmm0, 1400(%rsp)
.Ltmp6851:
	.loc	1 1280 32
	vmovss	3832(%r12), %xmm0
.Ltmp6852:
	.loc	1 1192 28
	vmovss	%xmm0, 1404(%rsp)
.Ltmp6853:
	.loc	1 1280 32
	vmovss	2728(%r12), %xmm0
.Ltmp6854:
	.loc	1 1192 28
	vmovss	%xmm0, 1408(%rsp)
.Ltmp6855:
	.loc	1 1280 32
	vmovss	2888(%r12), %xmm0
.Ltmp6856:
	.loc	1 1192 28
	vmovss	%xmm0, 1412(%rsp)
.Ltmp6857:
	.loc	1 1280 32
	vmovss	3048(%r12), %xmm0
.Ltmp6858:
	.loc	1 1192 28
	vmovss	%xmm0, 1416(%rsp)
.Ltmp6859:
	.loc	1 1280 32
	vmovss	3208(%r12), %xmm0
.Ltmp6860:
	.loc	1 1192 28
	vmovss	%xmm0, 1420(%rsp)
.Ltmp6861:
	.loc	1 1280 32
	vmovss	3368(%r12), %xmm0
.Ltmp6862:
	.loc	1 1192 28
	vmovss	%xmm0, 1424(%rsp)
.Ltmp6863:
	.loc	1 1280 32
	vmovss	3528(%r12), %xmm0
.Ltmp6864:
	.loc	1 1192 28
	vmovss	%xmm0, 1428(%rsp)
.Ltmp6865:
	.loc	1 1280 32
	vmovss	3688(%r12), %xmm0
.Ltmp6866:
	.loc	1 1192 28
	vmovss	%xmm0, 1432(%rsp)
.Ltmp6867:
	.loc	1 1280 32
	vmovss	3848(%r12), %xmm0
.Ltmp6868:
	.loc	1 1192 28
	vmovss	%xmm0, 1436(%rsp)
.Ltmp6869:
	.loc	1 1280 32
	vmovss	2744(%r12), %xmm0
.Ltmp6870:
	.loc	1 1192 28
	vmovss	%xmm0, 1440(%rsp)
.Ltmp6871:
	.loc	1 1280 32
	vmovss	2904(%r12), %xmm0
.Ltmp6872:
	.loc	1 1192 28
	vmovss	%xmm0, 1444(%rsp)
.Ltmp6873:
	.loc	1 1280 32
	vmovss	3064(%r12), %xmm0
.Ltmp6874:
	.loc	1 1192 28
	vmovss	%xmm0, 1448(%rsp)
.Ltmp6875:
	.loc	1 1280 32
	vmovss	3224(%r12), %xmm0
.Ltmp6876:
	.loc	1 1192 28
	vmovss	%xmm0, 1452(%rsp)
.Ltmp6877:
	.loc	1 1280 32
	vmovss	3384(%r12), %xmm0
.Ltmp6878:
	.loc	1 1192 28
	vmovss	%xmm0, 1456(%rsp)
.Ltmp6879:
	.loc	1 1280 32
	vmovss	3544(%r12), %xmm0
.Ltmp6880:
	.loc	1 1192 28
	vmovss	%xmm0, 1460(%rsp)
.Ltmp6881:
	.loc	1 1280 32
	vmovss	3704(%r12), %xmm0
.Ltmp6882:
	.loc	1 1192 28
	vmovss	%xmm0, 1464(%rsp)
.Ltmp6883:
	.loc	1 1280 32
	vmovss	3864(%r12), %xmm0
.Ltmp6884:
	.loc	1 1192 28
	vmovss	%xmm0, 1468(%rsp)
.Ltmp6885:
	.loc	1 1280 32
	vmovss	2760(%r12), %xmm0
.Ltmp6886:
	.loc	1 1192 28
	vmovss	%xmm0, 1472(%rsp)
.Ltmp6887:
	.loc	1 1280 32
	vmovss	2920(%r12), %xmm0
.Ltmp6888:
	.loc	1 1192 28
	vmovss	%xmm0, 1476(%rsp)
.Ltmp6889:
	.loc	1 1280 32
	vmovss	3080(%r12), %xmm0
.Ltmp6890:
	.loc	1 1192 28
	vmovss	%xmm0, 1480(%rsp)
.Ltmp6891:
	.loc	1 1280 32
	vmovss	3240(%r12), %xmm0
.Ltmp6892:
	.loc	1 1192 28
	vmovss	%xmm0, 1484(%rsp)
.Ltmp6893:
	.loc	1 1280 32
	vmovss	3400(%r12), %xmm0
.Ltmp6894:
	.loc	1 1192 28
	vmovss	%xmm0, 1488(%rsp)
.Ltmp6895:
	.loc	1 1280 32
	vmovss	3560(%r12), %xmm0
.Ltmp6896:
	.loc	1 1192 28
	vmovss	%xmm0, 1492(%rsp)
.Ltmp6897:
	.loc	1 1280 32
	vmovss	3720(%r12), %xmm0
.Ltmp6898:
	.loc	1 1192 28
	vmovss	%xmm0, 1496(%rsp)
.Ltmp6899:
	.loc	1 1280 32
	vmovss	3880(%r12), %xmm0
.Ltmp6900:
	.loc	1 1192 28
	vmovss	%xmm0, 1500(%rsp)
.Ltmp6901:
	.loc	1 1280 32
	vmovss	2776(%r12), %xmm0
.Ltmp6902:
	.loc	1 1192 28
	vmovss	%xmm0, 1504(%rsp)
.Ltmp6903:
	.loc	1 1280 32
	vmovss	2936(%r12), %xmm0
.Ltmp6904:
	.loc	1 1192 28
	vmovss	%xmm0, 1508(%rsp)
.Ltmp6905:
	.loc	1 1280 32
	vmovss	3096(%r12), %xmm0
.Ltmp6906:
	.loc	1 1192 28
	vmovss	%xmm0, 1512(%rsp)
.Ltmp6907:
	.loc	1 1280 32
	vmovss	3256(%r12), %xmm0
.Ltmp6908:
	.loc	1 1192 28
	vmovss	%xmm0, 1516(%rsp)
.Ltmp6909:
	.loc	1 1280 32
	vmovss	3416(%r12), %xmm0
.Ltmp6910:
	.loc	1 1192 28
	vmovss	%xmm0, 1520(%rsp)
.Ltmp6911:
	.loc	1 1280 32
	vmovss	3576(%r12), %xmm0
.Ltmp6912:
	.loc	1 1192 28
	vmovss	%xmm0, 1524(%rsp)
.Ltmp6913:
	.loc	1 1280 32
	vmovss	3736(%r12), %xmm0
.Ltmp6914:
	.loc	1 1192 28
	vmovss	%xmm0, 1528(%rsp)
.Ltmp6915:
	.loc	1 1280 32
	vmovd	3896(%r12), %xmm0
.Ltmp6916:
	.loc	1 1192 28
	vmovd	%xmm0, 1532(%rsp)
.Ltmp6917:
	.loc	1 1194 31
	leaq	3136(%rsp), %rdi
	movq	%r12, %rsi
	movl	2144(%rsp), %r14d
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	3328(%rsp), %rdi
	movq	2336(%rsp), %rsi
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
.Ltmp6918:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%r13), %rax
	shlq	$3, %r13
	leaq	(,%rax,8), %rsi
	.loc	1 1197 12 is_stmt 1
	testb	$1, 1728(%rsp)
	movq	%rax, 1760(%rsp)
	je	.LBB40_134
.Ltmp6919:
	.loc	38 1050 16
	cmpq	%r13, %rsi
.Ltmp6920:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_657
.Ltmp6921:
	.loc	48 451 16 is_stmt 1
	cmpq	128(%rsp), %rsi
	ja	.LBB40_657
.Ltmp6922:
	.loc	48 451 16 is_stmt 0
	cmpq	120(%rsp), %rsi
	ja	.LBB40_658
.Ltmp6923:
	.loc	48 0 16
	movq	192(%rsp), %rax
.Ltmp6924:
	.loc	1 1053 27 is_stmt 1
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	64(%rax), %ymm2
	vmovaps	96(%rax), %ymm3
	vmovaps	%ymm3, 3616(%rsp)
	vmovaps	%ymm2, 3584(%rsp)
	vmovaps	%ymm1, 3552(%rsp)
	vmovaps	%ymm0, 3520(%rsp)
	movq	1568(%rsp), %rax
.Ltmp6925:
	.loc	1 1054 26
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	64(%rax), %ymm2
	vmovaps	96(%rax), %ymm3
	vmovaps	%ymm3, 3744(%rsp)
	vmovaps	%ymm2, 3712(%rsp)
	vmovaps	%ymm1, 3680(%rsp)
	vmovaps	%ymm0, 3648(%rsp)
	movq	1632(%rsp), %rax
.Ltmp6926:
	.loc	1 1055 25
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 3808(%rsp)
	vmovaps	%ymm0, 3776(%rsp)
	movq	1600(%rsp), %rax
.Ltmp6927:
	.loc	1 1056 24
	vmovdqa	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 3872(%rsp)
	vmovdqa	%ymm0, 3840(%rsp)
.Ltmp6928:
	.loc	1 1057 24
	movq	5272(%r12), %r9
.Ltmp6929:
	.loc	2 1916 50
	testq	%r15, %r15
	je	.LBB40_130
.Ltmp6930:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,8), %rdx
	movq	240(%rsp), %rax
	leaq	(%rax,%r13,4), %r10
	movq	248(%rsp), %rax
	leaq	(%rax,%r13,4), %r11
.Ltmp6931:
	.loc	3 900 12 is_stmt 1
	movq	%r15, %r14
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r14
	xorl	%edi, %edi
	xorl	%r13d, %r13d
.Ltmp6932:
	.loc	3 0 12 is_stmt 0
.Ltmp6933:
	.p2align	4
.LBB40_119:
	.loc	1 1064 21 is_stmt 1
	vmovaps	256(%rsp), %ymm0
	vmovaps	288(%rsp), %ymm1
	vmovaps	320(%rsp), %ymm2
.Ltmp6934:
	.loc	14 48 14
	vaddps	576(%rsp), %ymm0, %ymm0
.Ltmp6935:
	.loc	1 1063 17
	vmovaps	%ymm0, 256(%rsp)
	.loc	1 1066 21
	vmovaps	896(%rsp), %ymm0
.Ltmp6936:
	.loc	14 48 14
	vaddps	1216(%rsp), %ymm0, %ymm0
.Ltmp6937:
	.loc	1 1065 17
	vmovaps	%ymm0, 896(%rsp)
.Ltmp6938:
	.loc	14 48 14
	vaddps	608(%rsp), %ymm1, %ymm0
.Ltmp6939:
	.loc	1 1063 17
	vmovaps	%ymm0, 288(%rsp)
	.loc	1 1066 21
	vmovaps	928(%rsp), %ymm0
.Ltmp6940:
	.loc	14 48 14
	vaddps	1248(%rsp), %ymm0, %ymm0
.Ltmp6941:
	.loc	1 1065 17
	vmovaps	%ymm0, 928(%rsp)
.Ltmp6942:
	.loc	14 48 14
	vaddps	640(%rsp), %ymm2, %ymm0
.Ltmp6943:
	.loc	1 1063 17
	vmovaps	%ymm0, 320(%rsp)
	.loc	1 1066 21
	vmovaps	960(%rsp), %ymm0
.Ltmp6944:
	.loc	14 48 14
	vaddps	1280(%rsp), %ymm0, %ymm0
.Ltmp6945:
	.loc	1 1065 17
	vmovaps	%ymm0, 960(%rsp)
	.loc	1 1064 21
	vmovaps	352(%rsp), %ymm0
.Ltmp6946:
	.loc	14 48 14
	vaddps	672(%rsp), %ymm0, %ymm0
.Ltmp6947:
	.loc	1 1063 17
	vmovaps	%ymm0, 352(%rsp)
	.loc	1 1066 21
	vmovaps	992(%rsp), %ymm0
.Ltmp6948:
	.loc	14 48 14
	vaddps	1312(%rsp), %ymm0, %ymm0
.Ltmp6949:
	.loc	1 1065 17
	vmovaps	%ymm0, 992(%rsp)
	.loc	1 1064 21
	vmovaps	384(%rsp), %ymm0
.Ltmp6950:
	.loc	14 48 14
	vaddps	704(%rsp), %ymm0, %ymm0
.Ltmp6951:
	.loc	1 1063 17
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 1066 21
	vmovaps	1024(%rsp), %ymm0
.Ltmp6952:
	.loc	14 48 14
	vaddps	1344(%rsp), %ymm0, %ymm0
.Ltmp6953:
	.loc	1 1065 17
	vmovaps	%ymm0, 1024(%rsp)
	.loc	1 1064 21
	vmovaps	416(%rsp), %ymm0
.Ltmp6954:
	.loc	14 48 14
	vaddps	736(%rsp), %ymm0, %ymm0
.Ltmp6955:
	.loc	1 1063 17
	vmovaps	%ymm0, 416(%rsp)
	.loc	1 1066 21
	vmovaps	1056(%rsp), %ymm0
.Ltmp6956:
	.loc	14 48 14
	vaddps	1376(%rsp), %ymm0, %ymm0
.Ltmp6957:
	.loc	1 1065 17
	vmovaps	%ymm0, 1056(%rsp)
	.loc	1 1064 21
	vmovaps	448(%rsp), %ymm0
.Ltmp6958:
	.loc	14 48 14
	vaddps	768(%rsp), %ymm0, %ymm0
.Ltmp6959:
	.loc	1 1063 17
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 1066 21
	vmovaps	1088(%rsp), %ymm0
.Ltmp6960:
	.loc	14 48 14
	vaddps	1408(%rsp), %ymm0, %ymm0
.Ltmp6961:
	.loc	1 1065 17
	vmovaps	%ymm0, 1088(%rsp)
	.loc	1 1064 21
	vmovaps	480(%rsp), %ymm0
.Ltmp6962:
	.loc	14 48 14
	vaddps	800(%rsp), %ymm0, %ymm0
.Ltmp6963:
	.loc	1 1063 17
	vmovaps	%ymm0, 480(%rsp)
	.loc	1 1066 21
	vmovaps	1120(%rsp), %ymm0
.Ltmp6964:
	.loc	14 48 14
	vaddps	1440(%rsp), %ymm0, %ymm0
.Ltmp6965:
	.loc	1 1065 17
	vmovaps	%ymm0, 1120(%rsp)
	.loc	1 1064 21
	vmovaps	512(%rsp), %ymm0
.Ltmp6966:
	.loc	14 48 14
	vaddps	832(%rsp), %ymm0, %ymm0
.Ltmp6967:
	.loc	1 1063 17
	vmovaps	%ymm0, 512(%rsp)
	.loc	1 1066 21
	vmovaps	1152(%rsp), %ymm0
.Ltmp6968:
	.loc	14 48 14
	vaddps	1472(%rsp), %ymm0, %ymm0
.Ltmp6969:
	.loc	1 1065 17
	vmovaps	%ymm0, 1152(%rsp)
	.loc	1 1064 21
	vmovaps	544(%rsp), %ymm0
.Ltmp6970:
	.loc	14 48 14
	vaddps	864(%rsp), %ymm0, %ymm0
.Ltmp6971:
	.loc	1 1063 17
	vmovaps	%ymm0, 544(%rsp)
	.loc	1 1066 21
	vmovaps	1184(%rsp), %ymm0
.Ltmp6972:
	.loc	14 48 14
	vaddps	1504(%rsp), %ymm0, %ymm0
.Ltmp6973:
	.loc	1 1065 17
	vmovaps	%ymm0, 1184(%rsp)
.Ltmp6974:
	.loc	1 1070 28
	leaq	1(%r9), %rax
	movq	64(%rsp), %rcx
.Ltmp6975:
	.loc	1 857 8
	cmpq	%rcx, %rax
	movl	$0, %r12d
	cmovaeq	%rcx, %r12
.Ltmp6976:
	.loc	48 568 12
	cmpq	%rdx, %rdi
	ja	.LBB40_633
.Ltmp6977:
	.loc	48 438 16
	cmpq	%r13, %r14
	je	.LBB40_641
.Ltmp6978:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r9,8), %rax
.Ltmp6979:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%r11,%rdi,4), %ymm0
	vmovdqa	%ymm0, 3904(%rsp)
	movq	1720(%rsp), %rcx
.Ltmp6980:
	.loc	1 1074 35
	movq	2600(%rcx), %rsi
.Ltmp6981:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_634
.Ltmp6982:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp6983:
	.loc	1 0 0 is_stmt 0
	leaq	(%r10,%rdi,4), %rcx
	movq	1720(%rsp), %r8
.Ltmp6984:
	.loc	1 1074 35 is_stmt 1
	movq	2592(%r8), %rsi
.Ltmp6985:
	.loc	8 551 14
	vmovdqu	(%rcx), %ymm0
	vmovdqu	%ymm0, (%rsi,%rax,4)
.Ltmp6986:
	.loc	1 1075 34
	movq	5224(%r8), %rsi
.Ltmp6987:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_635
.Ltmp6988:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp6989:
	.loc	1 0 0 is_stmt 0
	negq	%r12
	addq	%r12, %r9
	incq	%r9
	leaq	(,%r9,8), %r8
	movq	1720(%rsp), %r12
.Ltmp6990:
	.loc	1 1075 34 is_stmt 1
	movq	5216(%r12), %rsi
.Ltmp6991:
	.loc	8 551 14
	vmovaps	3904(%rsp), %ymm0
	vmovups	%ymm0, (%rsi,%rax,4)
.Ltmp6992:
	.loc	1 1076 22
	movq	2600(%r12), %rsi
.Ltmp6993:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB40_636
.Ltmp6994:
	.loc	48 438 16
	cmpq	$7, %rax
	jbe	.LBB40_630
.Ltmp6995:
	.loc	1 1076 22
	movq	2592(%r12), %rax
.Ltmp6996:
	.loc	8 551 14
	vmovups	(%rax,%r8,4), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp6997:
	.loc	1 1077 22
	movq	5224(%r12), %rsi
.Ltmp6998:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB40_637
.Ltmp6999:
	.loc	48 438 16
	cmpq	$7, %rax
	jbe	.LBB40_630
.Ltmp7000:
	.loc	1 0 0 is_stmt 0
	incq	%r13
.Ltmp7001:
	leaq	(%r11,%rdi,4), %rax
.Ltmp7002:
	.loc	1 1077 22 is_stmt 1
	movq	5216(%r12), %rcx
.Ltmp7003:
	.loc	8 551 14
	vmovdqu	(%rcx,%r8,4), %ymm0
	vmovdqu	%ymm0, (%rax)
.Ltmp7004:
	.loc	2 1916 50
	addq	$8, %rdi
	cmpq	%r13, %r15
.Ltmp7005:
	.loc	3 900 12
	jne	.LBB40_119
.Ltmp7006:
.LBB40_130:
	.loc	1 1160 5
	vmovaps	3520(%rsp), %ymm0
	vmovaps	3552(%rsp), %ymm1
	vmovaps	3584(%rsp), %ymm2
	vmovaps	3616(%rsp), %ymm3
	movq	192(%rsp), %rax
	vmovaps	%ymm3, 96(%rax)
	vmovaps	%ymm2, 64(%rax)
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1161 5
	vmovaps	3648(%rsp), %ymm0
	vmovaps	3680(%rsp), %ymm1
	vmovaps	3712(%rsp), %ymm2
	vmovaps	3744(%rsp), %ymm3
	movq	1568(%rsp), %rax
	vmovaps	%ymm3, 96(%rax)
	vmovaps	%ymm2, 64(%rax)
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1162 5
	vmovaps	3776(%rsp), %ymm0
	vmovaps	3808(%rsp), %ymm1
	movq	1632(%rsp), %rax
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1163 5
	vmovaps	3840(%rsp), %ymm0
	vmovaps	3872(%rsp), %ymm1
	movq	1600(%rsp), %rax
	vmovaps	%ymm1, 32(%rax)
	vmovaps	%ymm0, (%rax)
	.loc	1 1164 5
	movq	%r9, 5272(%r12)
	xorl	%eax, %eax
.Ltmp7007:
	.loc	1 0 5 is_stmt 0
.Ltmp7008:
	.p2align	4
.LBB40_131:
	.loc	1 1297 13 is_stmt 1
	vmovss	256(%rsp,%rax,2), %xmm3
	vmovss	260(%rsp,%rax,2), %xmm4
	vmovss	264(%rsp,%rax,2), %xmm5
	vmovss	268(%rsp,%rax,2), %xmm6
	vmovss	272(%rsp,%rax,2), %xmm7
	vmovss	276(%rsp,%rax,2), %xmm2
	vmovss	280(%rsp,%rax,2), %xmm1
	vmovd	284(%rsp,%rax,2), %xmm0
.Ltmp7009:
	.loc	1 1300 17
	vmovss	%xmm3, (%r12,%rax)
	.loc	1 1301 34
	movl	12(%r12,%rax), %ecx
	movl	172(%r12,%rax), %edx
.Ltmp7010:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7011:
	.loc	1 1301 17
	movl	%ecx, 12(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm4, 160(%r12,%rax)
.Ltmp7012:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%ebx, %edx
.Ltmp7013:
	.loc	1 1301 17
	movl	%edx, 172(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm5, 320(%r12,%rax)
	.loc	1 1301 34
	movl	332(%r12,%rax), %ecx
.Ltmp7014:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7015:
	.loc	1 1301 17
	movl	%ecx, 332(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm6, 480(%r12,%rax)
	.loc	1 1301 34
	movl	492(%r12,%rax), %ecx
.Ltmp7016:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7017:
	.loc	1 1301 17
	movl	%ecx, 492(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm7, 640(%r12,%rax)
	.loc	1 1301 34
	movl	652(%r12,%rax), %ecx
.Ltmp7018:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7019:
	.loc	1 1301 17
	movl	%ecx, 652(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 800(%r12,%rax)
	.loc	1 1301 34
	movl	812(%r12,%rax), %ecx
.Ltmp7020:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7021:
	.loc	1 1301 17
	movl	%ecx, 812(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 960(%r12,%rax)
	.loc	1 1301 34
	movl	972(%r12,%rax), %ecx
.Ltmp7022:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7023:
	.loc	1 1301 17
	movl	%ecx, 972(%r12,%rax)
	.loc	1 1300 17
	vmovd	%xmm0, 1120(%r12,%rax)
	.loc	1 1301 34
	movl	1132(%r12,%rax), %ecx
.Ltmp7024:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7025:
	.loc	1 1301 17
	movl	%ecx, 1132(%r12,%rax)
.Ltmp7026:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp7027:
	.loc	3 900 12
	jne	.LBB40_131
.Ltmp7028:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	.p2align	4
.LBB40_133:
.Ltmp7029:
	.loc	1 1297 13 is_stmt 1
	vmovss	896(%rsp,%rax,2), %xmm3
	vmovss	900(%rsp,%rax,2), %xmm4
	vmovss	904(%rsp,%rax,2), %xmm5
	vmovss	908(%rsp,%rax,2), %xmm6
	vmovss	912(%rsp,%rax,2), %xmm7
	vmovss	916(%rsp,%rax,2), %xmm2
	vmovss	920(%rsp,%rax,2), %xmm1
	vmovd	924(%rsp,%rax,2), %xmm0
.Ltmp7030:
	.loc	1 1300 17
	vmovss	%xmm3, 2624(%r12,%rax)
	.loc	1 1301 34
	movl	2636(%r12,%rax), %ecx
	movl	2796(%r12,%rax), %edx
.Ltmp7031:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7032:
	.loc	1 1301 17
	movl	%ecx, 2636(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm4, 2784(%r12,%rax)
.Ltmp7033:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%ebx, %edx
.Ltmp7034:
	.loc	1 1301 17
	movl	%edx, 2796(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm5, 2944(%r12,%rax)
	.loc	1 1301 34
	movl	2956(%r12,%rax), %ecx
.Ltmp7035:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7036:
	.loc	1 1301 17
	movl	%ecx, 2956(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm6, 3104(%r12,%rax)
	.loc	1 1301 34
	movl	3116(%r12,%rax), %ecx
.Ltmp7037:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7038:
	.loc	1 1301 17
	movl	%ecx, 3116(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm7, 3264(%r12,%rax)
	.loc	1 1301 34
	movl	3276(%r12,%rax), %ecx
.Ltmp7039:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7040:
	.loc	1 1301 17
	movl	%ecx, 3276(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 3424(%r12,%rax)
	.loc	1 1301 34
	movl	3436(%r12,%rax), %ecx
.Ltmp7041:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7042:
	.loc	1 1301 17
	movl	%ecx, 3436(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 3584(%r12,%rax)
	.loc	1 1301 34
	movl	3596(%r12,%rax), %ecx
.Ltmp7043:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7044:
	.loc	1 1301 17
	movl	%ecx, 3596(%r12,%rax)
	.loc	1 1300 17
	vmovd	%xmm0, 3744(%r12,%rax)
	.loc	1 1301 34
	movl	3756(%r12,%rax), %ecx
.Ltmp7045:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebx, %ecx
.Ltmp7046:
	.loc	1 1301 17
	movl	%ecx, 3756(%r12,%rax)
.Ltmp7047:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp7048:
	.loc	3 900 12
	jne	.LBB40_133
	jmp	.LBB40_112
.Ltmp7049:
	.loc	3 0 12 is_stmt 0
.Ltmp7050:
	.p2align	4
.LBB40_134:
	.loc	38 1050 16 is_stmt 1
	cmpq	%r13, %rsi
.Ltmp7051:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_656
.Ltmp7052:
	.loc	48 451 16 is_stmt 1
	cmpq	128(%rsp), %rsi
	ja	.LBB40_656
.Ltmp7053:
	.loc	48 451 16 is_stmt 0
	cmpq	120(%rsp), %rsi
	ja	.LBB40_659
.Ltmp7054:
	.loc	48 0 16
	movq	192(%rsp), %rax
.Ltmp7055:
	.loc	1 1053 27 is_stmt 1
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	64(%rax), %ymm2
	vmovaps	96(%rax), %ymm3
	vmovaps	%ymm3, 4032(%rsp)
	vmovaps	%ymm2, 4000(%rsp)
	vmovaps	%ymm1, 3968(%rsp)
	vmovaps	%ymm0, 3936(%rsp)
	movq	1568(%rsp), %rax
.Ltmp7056:
	.loc	1 1054 26
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	64(%rax), %ymm2
	vmovaps	96(%rax), %ymm3
	vmovaps	%ymm3, 4160(%rsp)
	vmovaps	%ymm2, 4128(%rsp)
	vmovaps	%ymm1, 4096(%rsp)
	vmovaps	%ymm0, 4064(%rsp)
	movq	1632(%rsp), %rax
.Ltmp7057:
	.loc	1 1055 25
	vmovaps	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 4224(%rsp)
	vmovaps	%ymm0, 4192(%rsp)
	movq	1600(%rsp), %rax
.Ltmp7058:
	.loc	1 1056 24
	vmovdqa	(%rax), %ymm0
	vmovaps	32(%rax), %ymm1
	vmovaps	%ymm1, 4288(%rsp)
	vmovdqa	%ymm0, 4256(%rsp)
.Ltmp7059:
	.loc	1 1057 24
	movq	5272(%r12), %r9
.Ltmp7060:
	.loc	2 1916 50
	testq	%r15, %r15
	je	.LBB40_111
.Ltmp7061:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,8), %rdx
	movq	240(%rsp), %rax
	leaq	(%rax,%r13,4), %r10
	movq	248(%rsp), %rax
	leaq	(%rax,%r13,4), %r11
.Ltmp7062:
	.loc	48 568 12 is_stmt 1
	movq	%r15, %r14
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r14
	xorl	%edi, %edi
	xorl	%r13d, %r13d
.Ltmp7063:
	.loc	48 0 12 is_stmt 0
.Ltmp7064:
	.p2align	4
.LBB40_139:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%r9), %rax
	movq	64(%rsp), %r12
.Ltmp7065:
	.loc	1 857 8
	cmpq	%r12, %rax
	jae	.LBB40_141
.Ltmp7066:
	.loc	1 0 8 is_stmt 0
	xorl	%r12d, %r12d
.LBB40_141:
.Ltmp7067:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB40_633
.Ltmp7068:
	.loc	48 438 16
	cmpq	%r13, %r14
	je	.LBB40_641
.Ltmp7069:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r9,8), %rax
.Ltmp7070:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%r11,%rdi,4), %ymm0
	vmovdqa	%ymm0, 4320(%rsp)
	movq	1720(%rsp), %rcx
.Ltmp7071:
	.loc	1 1074 35
	movq	2600(%rcx), %rsi
.Ltmp7072:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_634
.Ltmp7073:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7074:
	.loc	1 0 0 is_stmt 0
	leaq	(%r10,%rdi,4), %rcx
	movq	1720(%rsp), %r8
.Ltmp7075:
	.loc	1 1074 35 is_stmt 1
	movq	2592(%r8), %rsi
.Ltmp7076:
	.loc	8 551 14
	vmovdqu	(%rcx), %ymm0
	vmovdqu	%ymm0, (%rsi,%rax,4)
.Ltmp7077:
	.loc	1 1075 34
	movq	5224(%r8), %rsi
.Ltmp7078:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_635
.Ltmp7079:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7080:
	.loc	1 0 0 is_stmt 0
	negq	%r12
	addq	%r12, %r9
	incq	%r9
	leaq	(,%r9,8), %r8
	movq	1720(%rsp), %r12
.Ltmp7081:
	.loc	1 1075 34 is_stmt 1
	movq	5216(%r12), %rsi
.Ltmp7082:
	.loc	8 551 14
	vmovaps	4320(%rsp), %ymm0
	vmovups	%ymm0, (%rsi,%rax,4)
.Ltmp7083:
	.loc	1 1076 22
	movq	2600(%r12), %rsi
.Ltmp7084:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB40_636
.Ltmp7085:
	.loc	48 438 16
	cmpq	$7, %rax
	jbe	.LBB40_630
.Ltmp7086:
	.loc	1 1076 22
	movq	2592(%r12), %rax
.Ltmp7087:
	.loc	8 551 14
	vmovups	(%rax,%r8,4), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp7088:
	.loc	1 1077 22
	movq	5224(%r12), %rsi
.Ltmp7089:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB40_637
.Ltmp7090:
	.loc	48 438 16
	cmpq	$7, %rax
	jbe	.LBB40_630
.Ltmp7091:
	.loc	1 0 0 is_stmt 0
	incq	%r13
.Ltmp7092:
	leaq	(%r11,%rdi,4), %rax
.Ltmp7093:
	.loc	1 1077 22 is_stmt 1
	movq	5216(%r12), %rcx
.Ltmp7094:
	.loc	8 551 14
	vmovdqu	(%rcx,%r8,4), %ymm0
	vmovdqu	%ymm0, (%rax)
.Ltmp7095:
	.loc	2 1916 50
	addq	$8, %rdi
	cmpq	%r13, %r15
.Ltmp7096:
	.loc	3 900 12
	jne	.LBB40_139
	jmp	.LBB40_111
.Ltmp7097:
.LBB40_152:
	.loc	3 0 12 is_stmt 0
	vmovaps	1728(%rsp), %ymm0
.Ltmp7098:
	.loc	1 1160 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r12)
	vmovaps	1568(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r12)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r12)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r12)
	vmovaps	64(%rsp), %ymm0
	.loc	1 1161 5
	vmovaps	%ymm0, 4032(%r12)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r12)
	vmovaps	2272(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r12)
	vmovaps	1760(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r12)
	vmovaps	1984(%rsp), %ymm0
	.loc	1 1162 5
	vmovaps	%ymm0, 2304(%r12)
	vmovdqa	1952(%rsp), %ymm0
	vmovdqa	%ymm0, 2336(%r12)
	.loc	1 1163 5
	vmovaps	%ymm9, 4928(%r12)
	vmovaps	%ymm3, 4960(%r12)
	.loc	1 1164 5
	movq	%r10, 5272(%r12)
.Ltmp7099:
.LBB40_153:
	.loc	1 0 5 is_stmt 0
	movq	144(%rsp), %rsi
	movq	160(%rsp), %rax
	.loc	1 1189 11 is_stmt 1
	cmpq	%rsi, %rax
	jae	.LBB40_595
.LBB40_154:
	.loc	1 0 11 is_stmt 0
	movq	%rax, 192(%rsp)
	.loc	1 1190 42 is_stmt 1
	subq	%rax, %rsi
	.loc	1 1190 29 is_stmt 0
	movq	%r12, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E12plan_segmentB5_
	movb	%dl, 64(%rsp)
	movq	%rax, %r13
.Ltmp7100:
	.loc	1 1279 33 is_stmt 1
	vmovss	(%r12), %xmm0
.Ltmp7101:
	.loc	1 1192 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp7102:
	.loc	1 1279 33
	vmovss	160(%r12), %xmm0
.Ltmp7103:
	.loc	1 1192 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp7104:
	.loc	1 1279 33
	vmovss	320(%r12), %xmm0
.Ltmp7105:
	.loc	1 1192 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp7106:
	.loc	1 1279 33
	vmovss	480(%r12), %xmm0
.Ltmp7107:
	.loc	1 1192 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp7108:
	.loc	1 1279 33
	vmovss	640(%r12), %xmm0
.Ltmp7109:
	.loc	1 1192 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp7110:
	.loc	1 1279 33
	vmovss	800(%r12), %xmm0
.Ltmp7111:
	.loc	1 1192 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp7112:
	.loc	1 1279 33
	vmovss	960(%r12), %xmm0
.Ltmp7113:
	.loc	1 1192 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp7114:
	.loc	1 1279 33
	vmovss	1120(%r12), %xmm0
.Ltmp7115:
	.loc	1 1192 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp7116:
	.loc	1 1279 33
	vmovss	16(%r12), %xmm0
.Ltmp7117:
	.loc	1 1192 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp7118:
	.loc	1 1279 33
	vmovss	176(%r12), %xmm0
.Ltmp7119:
	.loc	1 1192 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp7120:
	.loc	1 1279 33
	vmovss	336(%r12), %xmm0
.Ltmp7121:
	.loc	1 1192 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp7122:
	.loc	1 1279 33
	vmovss	496(%r12), %xmm0
.Ltmp7123:
	.loc	1 1192 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp7124:
	.loc	1 1279 33
	vmovss	656(%r12), %xmm0
.Ltmp7125:
	.loc	1 1192 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp7126:
	.loc	1 1279 33
	vmovss	816(%r12), %xmm0
.Ltmp7127:
	.loc	1 1192 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp7128:
	.loc	1 1279 33
	vmovss	976(%r12), %xmm0
.Ltmp7129:
	.loc	1 1192 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp7130:
	.loc	1 1279 33
	vmovss	1136(%r12), %xmm0
.Ltmp7131:
	.loc	1 1192 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp7132:
	.loc	1 1279 33
	vmovss	32(%r12), %xmm0
.Ltmp7133:
	.loc	1 1192 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp7134:
	.loc	1 1279 33
	vmovss	192(%r12), %xmm0
.Ltmp7135:
	.loc	1 1192 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp7136:
	.loc	1 1279 33
	vmovss	352(%r12), %xmm0
.Ltmp7137:
	.loc	1 1192 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp7138:
	.loc	1 1279 33
	vmovss	512(%r12), %xmm0
.Ltmp7139:
	.loc	1 1192 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp7140:
	.loc	1 1279 33
	vmovss	672(%r12), %xmm0
.Ltmp7141:
	.loc	1 1192 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp7142:
	.loc	1 1279 33
	vmovss	832(%r12), %xmm0
.Ltmp7143:
	.loc	1 1192 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp7144:
	.loc	1 1279 33
	vmovss	992(%r12), %xmm0
.Ltmp7145:
	.loc	1 1192 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp7146:
	.loc	1 1279 33
	vmovss	1152(%r12), %xmm0
.Ltmp7147:
	.loc	1 1192 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp7148:
	.loc	1 1279 33
	vmovss	48(%r12), %xmm0
.Ltmp7149:
	.loc	1 1192 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp7150:
	.loc	1 1279 33
	vmovss	208(%r12), %xmm0
.Ltmp7151:
	.loc	1 1192 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp7152:
	.loc	1 1279 33
	vmovss	368(%r12), %xmm0
.Ltmp7153:
	.loc	1 1192 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp7154:
	.loc	1 1279 33
	vmovss	528(%r12), %xmm0
.Ltmp7155:
	.loc	1 1192 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp7156:
	.loc	1 1279 33
	vmovss	688(%r12), %xmm0
.Ltmp7157:
	.loc	1 1192 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp7158:
	.loc	1 1279 33
	vmovss	848(%r12), %xmm0
.Ltmp7159:
	.loc	1 1192 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp7160:
	.loc	1 1279 33
	vmovss	1008(%r12), %xmm0
.Ltmp7161:
	.loc	1 1192 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp7162:
	.loc	1 1279 33
	vmovss	1168(%r12), %xmm0
.Ltmp7163:
	.loc	1 1192 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp7164:
	.loc	1 1279 33
	vmovss	64(%r12), %xmm0
.Ltmp7165:
	.loc	1 1192 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp7166:
	.loc	1 1279 33
	vmovss	224(%r12), %xmm0
.Ltmp7167:
	.loc	1 1192 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp7168:
	.loc	1 1279 33
	vmovss	384(%r12), %xmm0
.Ltmp7169:
	.loc	1 1192 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp7170:
	.loc	1 1279 33
	vmovss	544(%r12), %xmm0
.Ltmp7171:
	.loc	1 1192 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp7172:
	.loc	1 1279 33
	vmovss	704(%r12), %xmm0
.Ltmp7173:
	.loc	1 1192 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp7174:
	.loc	1 1279 33
	vmovss	864(%r12), %xmm0
.Ltmp7175:
	.loc	1 1192 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp7176:
	.loc	1 1279 33
	vmovss	1024(%r12), %xmm0
.Ltmp7177:
	.loc	1 1192 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp7178:
	.loc	1 1279 33
	vmovss	1184(%r12), %xmm0
.Ltmp7179:
	.loc	1 1192 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp7180:
	.loc	1 1279 33
	vmovss	80(%r12), %xmm0
.Ltmp7181:
	.loc	1 1192 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp7182:
	.loc	1 1279 33
	vmovss	240(%r12), %xmm0
.Ltmp7183:
	.loc	1 1192 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp7184:
	.loc	1 1279 33
	vmovss	400(%r12), %xmm0
.Ltmp7185:
	.loc	1 1192 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp7186:
	.loc	1 1279 33
	vmovss	560(%r12), %xmm0
.Ltmp7187:
	.loc	1 1192 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp7188:
	.loc	1 1279 33
	vmovss	720(%r12), %xmm0
.Ltmp7189:
	.loc	1 1192 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp7190:
	.loc	1 1279 33
	vmovss	880(%r12), %xmm0
.Ltmp7191:
	.loc	1 1192 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp7192:
	.loc	1 1279 33
	vmovss	1040(%r12), %xmm0
.Ltmp7193:
	.loc	1 1192 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp7194:
	.loc	1 1279 33
	vmovss	1200(%r12), %xmm0
.Ltmp7195:
	.loc	1 1192 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp7196:
	.loc	1 1279 33
	vmovss	96(%r12), %xmm0
.Ltmp7197:
	.loc	1 1192 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp7198:
	.loc	1 1279 33
	vmovss	256(%r12), %xmm0
.Ltmp7199:
	.loc	1 1192 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp7200:
	.loc	1 1279 33
	vmovss	416(%r12), %xmm0
.Ltmp7201:
	.loc	1 1192 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp7202:
	.loc	1 1279 33
	vmovss	576(%r12), %xmm0
.Ltmp7203:
	.loc	1 1192 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp7204:
	.loc	1 1279 33
	vmovss	736(%r12), %xmm0
.Ltmp7205:
	.loc	1 1192 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp7206:
	.loc	1 1279 33
	vmovss	896(%r12), %xmm0
.Ltmp7207:
	.loc	1 1192 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp7208:
	.loc	1 1279 33
	vmovss	1056(%r12), %xmm0
.Ltmp7209:
	.loc	1 1192 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp7210:
	.loc	1 1279 33
	vmovss	1216(%r12), %xmm0
.Ltmp7211:
	.loc	1 1192 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp7212:
	.loc	1 1279 33
	vmovss	112(%r12), %xmm0
.Ltmp7213:
	.loc	1 1192 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp7214:
	.loc	1 1279 33
	vmovss	272(%r12), %xmm0
.Ltmp7215:
	.loc	1 1192 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp7216:
	.loc	1 1279 33
	vmovss	432(%r12), %xmm0
.Ltmp7217:
	.loc	1 1192 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp7218:
	.loc	1 1279 33
	vmovss	592(%r12), %xmm0
.Ltmp7219:
	.loc	1 1192 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp7220:
	.loc	1 1279 33
	vmovss	752(%r12), %xmm0
.Ltmp7221:
	.loc	1 1192 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp7222:
	.loc	1 1279 33
	vmovss	912(%r12), %xmm0
.Ltmp7223:
	.loc	1 1192 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp7224:
	.loc	1 1279 33
	vmovss	1072(%r12), %xmm0
.Ltmp7225:
	.loc	1 1192 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp7226:
	.loc	1 1279 33
	vmovss	1232(%r12), %xmm0
.Ltmp7227:
	.loc	1 1192 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp7228:
	.loc	1 1279 33
	vmovss	128(%r12), %xmm0
.Ltmp7229:
	.loc	1 1192 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp7230:
	.loc	1 1279 33
	vmovss	288(%r12), %xmm0
.Ltmp7231:
	.loc	1 1192 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp7232:
	.loc	1 1279 33
	vmovss	448(%r12), %xmm0
.Ltmp7233:
	.loc	1 1192 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp7234:
	.loc	1 1279 33
	vmovss	608(%r12), %xmm0
.Ltmp7235:
	.loc	1 1192 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp7236:
	.loc	1 1279 33
	vmovss	768(%r12), %xmm0
.Ltmp7237:
	.loc	1 1192 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp7238:
	.loc	1 1279 33
	vmovss	928(%r12), %xmm0
.Ltmp7239:
	.loc	1 1192 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp7240:
	.loc	1 1279 33
	vmovss	1088(%r12), %xmm0
.Ltmp7241:
	.loc	1 1192 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp7242:
	.loc	1 1279 33
	vmovss	1248(%r12), %xmm0
.Ltmp7243:
	.loc	1 1192 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp7244:
	.loc	1 1279 33
	vmovss	144(%r12), %xmm0
.Ltmp7245:
	.loc	1 1192 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp7246:
	.loc	1 1279 33
	vmovss	304(%r12), %xmm0
.Ltmp7247:
	.loc	1 1192 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp7248:
	.loc	1 1279 33
	vmovss	464(%r12), %xmm0
.Ltmp7249:
	.loc	1 1192 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp7250:
	.loc	1 1279 33
	vmovss	624(%r12), %xmm0
.Ltmp7251:
	.loc	1 1192 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp7252:
	.loc	1 1279 33
	vmovss	784(%r12), %xmm0
.Ltmp7253:
	.loc	1 1192 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp7254:
	.loc	1 1279 33
	vmovss	944(%r12), %xmm0
.Ltmp7255:
	.loc	1 1192 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp7256:
	.loc	1 1279 33
	vmovss	1104(%r12), %xmm0
.Ltmp7257:
	.loc	1 1192 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp7258:
	.loc	1 1279 33
	vmovss	1264(%r12), %xmm0
.Ltmp7259:
	.loc	1 1192 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp7260:
	.loc	1 1280 32
	vmovss	8(%r12), %xmm0
.Ltmp7261:
	.loc	1 1192 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp7262:
	.loc	1 1280 32
	vmovss	168(%r12), %xmm0
.Ltmp7263:
	.loc	1 1192 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp7264:
	.loc	1 1280 32
	vmovss	328(%r12), %xmm0
.Ltmp7265:
	.loc	1 1192 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp7266:
	.loc	1 1280 32
	vmovss	488(%r12), %xmm0
.Ltmp7267:
	.loc	1 1192 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp7268:
	.loc	1 1280 32
	vmovss	648(%r12), %xmm0
.Ltmp7269:
	.loc	1 1192 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp7270:
	.loc	1 1280 32
	vmovss	808(%r12), %xmm0
.Ltmp7271:
	.loc	1 1192 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp7272:
	.loc	1 1280 32
	vmovss	968(%r12), %xmm0
.Ltmp7273:
	.loc	1 1192 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp7274:
	.loc	1 1280 32
	vmovss	1128(%r12), %xmm0
.Ltmp7275:
	.loc	1 1192 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp7276:
	.loc	1 1280 32
	vmovss	24(%r12), %xmm0
.Ltmp7277:
	.loc	1 1192 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp7278:
	.loc	1 1280 32
	vmovss	184(%r12), %xmm0
.Ltmp7279:
	.loc	1 1192 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp7280:
	.loc	1 1280 32
	vmovss	344(%r12), %xmm0
.Ltmp7281:
	.loc	1 1192 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp7282:
	.loc	1 1280 32
	vmovss	504(%r12), %xmm0
.Ltmp7283:
	.loc	1 1192 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp7284:
	.loc	1 1280 32
	vmovss	664(%r12), %xmm0
.Ltmp7285:
	.loc	1 1192 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp7286:
	.loc	1 1280 32
	vmovss	824(%r12), %xmm0
.Ltmp7287:
	.loc	1 1192 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp7288:
	.loc	1 1280 32
	vmovss	984(%r12), %xmm0
.Ltmp7289:
	.loc	1 1192 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp7290:
	.loc	1 1280 32
	vmovss	1144(%r12), %xmm0
.Ltmp7291:
	.loc	1 1192 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp7292:
	.loc	1 1280 32
	vmovss	40(%r12), %xmm0
.Ltmp7293:
	.loc	1 1192 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp7294:
	.loc	1 1280 32
	vmovss	200(%r12), %xmm0
.Ltmp7295:
	.loc	1 1192 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp7296:
	.loc	1 1280 32
	vmovss	360(%r12), %xmm0
.Ltmp7297:
	.loc	1 1192 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp7298:
	.loc	1 1280 32
	vmovss	520(%r12), %xmm0
.Ltmp7299:
	.loc	1 1192 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp7300:
	.loc	1 1280 32
	vmovss	680(%r12), %xmm0
.Ltmp7301:
	.loc	1 1192 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp7302:
	.loc	1 1280 32
	vmovss	840(%r12), %xmm0
.Ltmp7303:
	.loc	1 1192 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp7304:
	.loc	1 1280 32
	vmovss	1000(%r12), %xmm0
.Ltmp7305:
	.loc	1 1192 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp7306:
	.loc	1 1280 32
	vmovss	1160(%r12), %xmm0
.Ltmp7307:
	.loc	1 1192 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp7308:
	.loc	1 1280 32
	vmovss	56(%r12), %xmm0
.Ltmp7309:
	.loc	1 1192 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp7310:
	.loc	1 1280 32
	vmovss	216(%r12), %xmm0
.Ltmp7311:
	.loc	1 1192 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp7312:
	.loc	1 1280 32
	vmovss	376(%r12), %xmm0
.Ltmp7313:
	.loc	1 1192 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp7314:
	.loc	1 1280 32
	vmovss	536(%r12), %xmm0
.Ltmp7315:
	.loc	1 1192 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp7316:
	.loc	1 1280 32
	vmovss	696(%r12), %xmm0
.Ltmp7317:
	.loc	1 1192 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp7318:
	.loc	1 1280 32
	vmovss	856(%r12), %xmm0
.Ltmp7319:
	.loc	1 1192 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp7320:
	.loc	1 1280 32
	vmovss	1016(%r12), %xmm0
.Ltmp7321:
	.loc	1 1192 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp7322:
	.loc	1 1280 32
	vmovss	1176(%r12), %xmm0
.Ltmp7323:
	.loc	1 1192 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp7324:
	.loc	1 1280 32
	vmovss	72(%r12), %xmm0
.Ltmp7325:
	.loc	1 1192 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp7326:
	.loc	1 1280 32
	vmovss	232(%r12), %xmm0
.Ltmp7327:
	.loc	1 1192 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp7328:
	.loc	1 1280 32
	vmovss	392(%r12), %xmm0
.Ltmp7329:
	.loc	1 1192 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp7330:
	.loc	1 1280 32
	vmovss	552(%r12), %xmm0
.Ltmp7331:
	.loc	1 1192 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp7332:
	.loc	1 1280 32
	vmovss	712(%r12), %xmm0
.Ltmp7333:
	.loc	1 1192 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp7334:
	.loc	1 1280 32
	vmovss	872(%r12), %xmm0
.Ltmp7335:
	.loc	1 1192 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp7336:
	.loc	1 1280 32
	vmovss	1032(%r12), %xmm0
.Ltmp7337:
	.loc	1 1192 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp7338:
	.loc	1 1280 32
	vmovss	1192(%r12), %xmm0
.Ltmp7339:
	.loc	1 1192 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp7340:
	.loc	1 1280 32
	vmovss	88(%r12), %xmm0
.Ltmp7341:
	.loc	1 1192 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp7342:
	.loc	1 1280 32
	vmovss	248(%r12), %xmm0
.Ltmp7343:
	.loc	1 1192 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp7344:
	.loc	1 1280 32
	vmovss	408(%r12), %xmm0
.Ltmp7345:
	.loc	1 1192 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp7346:
	.loc	1 1280 32
	vmovss	568(%r12), %xmm0
.Ltmp7347:
	.loc	1 1192 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp7348:
	.loc	1 1280 32
	vmovss	728(%r12), %xmm0
.Ltmp7349:
	.loc	1 1192 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp7350:
	.loc	1 1280 32
	vmovss	888(%r12), %xmm0
.Ltmp7351:
	.loc	1 1192 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp7352:
	.loc	1 1280 32
	vmovss	1048(%r12), %xmm0
.Ltmp7353:
	.loc	1 1192 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp7354:
	.loc	1 1280 32
	vmovss	1208(%r12), %xmm0
.Ltmp7355:
	.loc	1 1192 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp7356:
	.loc	1 1280 32
	vmovss	104(%r12), %xmm0
.Ltmp7357:
	.loc	1 1192 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp7358:
	.loc	1 1280 32
	vmovss	264(%r12), %xmm0
.Ltmp7359:
	.loc	1 1192 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp7360:
	.loc	1 1280 32
	vmovss	424(%r12), %xmm0
.Ltmp7361:
	.loc	1 1192 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp7362:
	.loc	1 1280 32
	vmovss	584(%r12), %xmm0
.Ltmp7363:
	.loc	1 1192 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp7364:
	.loc	1 1280 32
	vmovss	744(%r12), %xmm0
.Ltmp7365:
	.loc	1 1192 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp7366:
	.loc	1 1280 32
	vmovss	904(%r12), %xmm0
.Ltmp7367:
	.loc	1 1192 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp7368:
	.loc	1 1280 32
	vmovss	1064(%r12), %xmm0
.Ltmp7369:
	.loc	1 1192 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp7370:
	.loc	1 1280 32
	vmovss	1224(%r12), %xmm0
.Ltmp7371:
	.loc	1 1192 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp7372:
	.loc	1 1280 32
	vmovss	120(%r12), %xmm0
.Ltmp7373:
	.loc	1 1192 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp7374:
	.loc	1 1280 32
	vmovss	280(%r12), %xmm0
.Ltmp7375:
	.loc	1 1192 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp7376:
	.loc	1 1280 32
	vmovss	440(%r12), %xmm0
.Ltmp7377:
	.loc	1 1192 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp7378:
	.loc	1 1280 32
	vmovss	600(%r12), %xmm0
.Ltmp7379:
	.loc	1 1192 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp7380:
	.loc	1 1280 32
	vmovss	760(%r12), %xmm0
.Ltmp7381:
	.loc	1 1192 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp7382:
	.loc	1 1280 32
	vmovss	920(%r12), %xmm0
.Ltmp7383:
	.loc	1 1192 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp7384:
	.loc	1 1280 32
	vmovss	1080(%r12), %xmm0
.Ltmp7385:
	.loc	1 1192 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp7386:
	.loc	1 1280 32
	vmovss	1240(%r12), %xmm0
.Ltmp7387:
	.loc	1 1192 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp7388:
	.loc	1 1280 32
	vmovss	136(%r12), %xmm0
.Ltmp7389:
	.loc	1 1192 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp7390:
	.loc	1 1280 32
	vmovss	296(%r12), %xmm0
.Ltmp7391:
	.loc	1 1192 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp7392:
	.loc	1 1280 32
	vmovss	456(%r12), %xmm0
.Ltmp7393:
	.loc	1 1192 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp7394:
	.loc	1 1280 32
	vmovss	616(%r12), %xmm0
.Ltmp7395:
	.loc	1 1192 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp7396:
	.loc	1 1280 32
	vmovss	776(%r12), %xmm0
.Ltmp7397:
	.loc	1 1192 28
	vmovss	%xmm0, 848(%rsp)
.Ltmp7398:
	.loc	1 1280 32
	vmovss	936(%r12), %xmm0
.Ltmp7399:
	.loc	1 1192 28
	vmovss	%xmm0, 852(%rsp)
.Ltmp7400:
	.loc	1 1280 32
	vmovss	1096(%r12), %xmm0
.Ltmp7401:
	.loc	1 1192 28
	vmovss	%xmm0, 856(%rsp)
.Ltmp7402:
	.loc	1 1280 32
	vmovss	1256(%r12), %xmm0
.Ltmp7403:
	.loc	1 1192 28
	vmovss	%xmm0, 860(%rsp)
.Ltmp7404:
	.loc	1 1280 32
	vmovss	152(%r12), %xmm0
.Ltmp7405:
	.loc	1 1192 28
	vmovss	%xmm0, 864(%rsp)
.Ltmp7406:
	.loc	1 1280 32
	vmovss	312(%r12), %xmm0
.Ltmp7407:
	.loc	1 1192 28
	vmovss	%xmm0, 868(%rsp)
.Ltmp7408:
	.loc	1 1280 32
	vmovss	472(%r12), %xmm0
.Ltmp7409:
	.loc	1 1192 28
	vmovss	%xmm0, 872(%rsp)
.Ltmp7410:
	.loc	1 1280 32
	vmovss	632(%r12), %xmm0
.Ltmp7411:
	.loc	1 1192 28
	vmovss	%xmm0, 876(%rsp)
.Ltmp7412:
	.loc	1 1280 32
	vmovss	792(%r12), %xmm0
.Ltmp7413:
	.loc	1 1192 28
	vmovss	%xmm0, 880(%rsp)
.Ltmp7414:
	.loc	1 1280 32
	vmovss	952(%r12), %xmm0
.Ltmp7415:
	.loc	1 1192 28
	vmovss	%xmm0, 884(%rsp)
.Ltmp7416:
	.loc	1 1280 32
	vmovss	1112(%r12), %xmm0
.Ltmp7417:
	.loc	1 1192 28
	vmovss	%xmm0, 888(%rsp)
.Ltmp7418:
	.loc	1 1280 32
	vmovss	1272(%r12), %xmm0
.Ltmp7419:
	.loc	1 1192 28
	vmovss	%xmm0, 892(%rsp)
.Ltmp7420:
	.loc	1 1279 33
	vmovss	2624(%r12), %xmm0
.Ltmp7421:
	.loc	1 1192 28
	vmovss	%xmm0, 896(%rsp)
.Ltmp7422:
	.loc	1 1279 33
	vmovss	2784(%r12), %xmm0
.Ltmp7423:
	.loc	1 1192 28
	vmovss	%xmm0, 900(%rsp)
.Ltmp7424:
	.loc	1 1279 33
	vmovss	2944(%r12), %xmm0
.Ltmp7425:
	.loc	1 1192 28
	vmovss	%xmm0, 904(%rsp)
.Ltmp7426:
	.loc	1 1279 33
	vmovss	3104(%r12), %xmm0
.Ltmp7427:
	.loc	1 1192 28
	vmovss	%xmm0, 908(%rsp)
.Ltmp7428:
	.loc	1 1279 33
	vmovss	3264(%r12), %xmm0
.Ltmp7429:
	.loc	1 1192 28
	vmovss	%xmm0, 912(%rsp)
.Ltmp7430:
	.loc	1 1279 33
	vmovss	3424(%r12), %xmm0
.Ltmp7431:
	.loc	1 1192 28
	vmovss	%xmm0, 916(%rsp)
.Ltmp7432:
	.loc	1 1279 33
	vmovss	3584(%r12), %xmm0
.Ltmp7433:
	.loc	1 1192 28
	vmovss	%xmm0, 920(%rsp)
.Ltmp7434:
	.loc	1 1279 33
	vmovss	3744(%r12), %xmm0
.Ltmp7435:
	.loc	1 1192 28
	vmovss	%xmm0, 924(%rsp)
.Ltmp7436:
	.loc	1 1279 33
	vmovss	2640(%r12), %xmm0
.Ltmp7437:
	.loc	1 1192 28
	vmovss	%xmm0, 928(%rsp)
.Ltmp7438:
	.loc	1 1279 33
	vmovss	2800(%r12), %xmm0
.Ltmp7439:
	.loc	1 1192 28
	vmovss	%xmm0, 932(%rsp)
.Ltmp7440:
	.loc	1 1279 33
	vmovss	2960(%r12), %xmm0
.Ltmp7441:
	.loc	1 1192 28
	vmovss	%xmm0, 936(%rsp)
.Ltmp7442:
	.loc	1 1279 33
	vmovss	3120(%r12), %xmm0
.Ltmp7443:
	.loc	1 1192 28
	vmovss	%xmm0, 940(%rsp)
.Ltmp7444:
	.loc	1 1279 33
	vmovss	3280(%r12), %xmm0
.Ltmp7445:
	.loc	1 1192 28
	vmovss	%xmm0, 944(%rsp)
.Ltmp7446:
	.loc	1 1279 33
	vmovss	3440(%r12), %xmm0
.Ltmp7447:
	.loc	1 1192 28
	vmovss	%xmm0, 948(%rsp)
.Ltmp7448:
	.loc	1 1279 33
	vmovss	3600(%r12), %xmm0
.Ltmp7449:
	.loc	1 1192 28
	vmovss	%xmm0, 952(%rsp)
.Ltmp7450:
	.loc	1 1279 33
	vmovss	3760(%r12), %xmm0
.Ltmp7451:
	.loc	1 1192 28
	vmovss	%xmm0, 956(%rsp)
.Ltmp7452:
	.loc	1 1279 33
	vmovss	2656(%r12), %xmm0
.Ltmp7453:
	.loc	1 1192 28
	vmovss	%xmm0, 960(%rsp)
.Ltmp7454:
	.loc	1 1279 33
	vmovss	2816(%r12), %xmm0
.Ltmp7455:
	.loc	1 1192 28
	vmovss	%xmm0, 964(%rsp)
.Ltmp7456:
	.loc	1 1279 33
	vmovss	2976(%r12), %xmm0
.Ltmp7457:
	.loc	1 1192 28
	vmovss	%xmm0, 968(%rsp)
.Ltmp7458:
	.loc	1 1279 33
	vmovss	3136(%r12), %xmm0
.Ltmp7459:
	.loc	1 1192 28
	vmovss	%xmm0, 972(%rsp)
.Ltmp7460:
	.loc	1 1279 33
	vmovss	3296(%r12), %xmm0
.Ltmp7461:
	.loc	1 1192 28
	vmovss	%xmm0, 976(%rsp)
.Ltmp7462:
	.loc	1 1279 33
	vmovss	3456(%r12), %xmm0
.Ltmp7463:
	.loc	1 1192 28
	vmovss	%xmm0, 980(%rsp)
.Ltmp7464:
	.loc	1 1279 33
	vmovss	3616(%r12), %xmm0
.Ltmp7465:
	.loc	1 1192 28
	vmovss	%xmm0, 984(%rsp)
.Ltmp7466:
	.loc	1 1279 33
	vmovss	3776(%r12), %xmm0
.Ltmp7467:
	.loc	1 1192 28
	vmovss	%xmm0, 988(%rsp)
.Ltmp7468:
	.loc	1 1279 33
	vmovss	2672(%r12), %xmm0
.Ltmp7469:
	.loc	1 1192 28
	vmovss	%xmm0, 992(%rsp)
.Ltmp7470:
	.loc	1 1279 33
	vmovss	2832(%r12), %xmm0
.Ltmp7471:
	.loc	1 1192 28
	vmovss	%xmm0, 996(%rsp)
.Ltmp7472:
	.loc	1 1279 33
	vmovss	2992(%r12), %xmm0
.Ltmp7473:
	.loc	1 1192 28
	vmovss	%xmm0, 1000(%rsp)
.Ltmp7474:
	.loc	1 1279 33
	vmovss	3152(%r12), %xmm0
.Ltmp7475:
	.loc	1 1192 28
	vmovss	%xmm0, 1004(%rsp)
.Ltmp7476:
	.loc	1 1279 33
	vmovss	3312(%r12), %xmm0
.Ltmp7477:
	.loc	1 1192 28
	vmovss	%xmm0, 1008(%rsp)
.Ltmp7478:
	.loc	1 1279 33
	vmovss	3472(%r12), %xmm0
.Ltmp7479:
	.loc	1 1192 28
	vmovss	%xmm0, 1012(%rsp)
.Ltmp7480:
	.loc	1 1279 33
	vmovss	3632(%r12), %xmm0
.Ltmp7481:
	.loc	1 1192 28
	vmovss	%xmm0, 1016(%rsp)
.Ltmp7482:
	.loc	1 1279 33
	vmovss	3792(%r12), %xmm0
.Ltmp7483:
	.loc	1 1192 28
	vmovss	%xmm0, 1020(%rsp)
.Ltmp7484:
	.loc	1 1279 33
	vmovss	2688(%r12), %xmm0
.Ltmp7485:
	.loc	1 1192 28
	vmovss	%xmm0, 1024(%rsp)
.Ltmp7486:
	.loc	1 1279 33
	vmovss	2848(%r12), %xmm0
.Ltmp7487:
	.loc	1 1192 28
	vmovss	%xmm0, 1028(%rsp)
.Ltmp7488:
	.loc	1 1279 33
	vmovss	3008(%r12), %xmm0
.Ltmp7489:
	.loc	1 1192 28
	vmovss	%xmm0, 1032(%rsp)
.Ltmp7490:
	.loc	1 1279 33
	vmovss	3168(%r12), %xmm0
.Ltmp7491:
	.loc	1 1192 28
	vmovss	%xmm0, 1036(%rsp)
.Ltmp7492:
	.loc	1 1279 33
	vmovss	3328(%r12), %xmm0
.Ltmp7493:
	.loc	1 1192 28
	vmovss	%xmm0, 1040(%rsp)
.Ltmp7494:
	.loc	1 1279 33
	vmovss	3488(%r12), %xmm0
.Ltmp7495:
	.loc	1 1192 28
	vmovss	%xmm0, 1044(%rsp)
.Ltmp7496:
	.loc	1 1279 33
	vmovss	3648(%r12), %xmm0
.Ltmp7497:
	.loc	1 1192 28
	vmovss	%xmm0, 1048(%rsp)
.Ltmp7498:
	.loc	1 1279 33
	vmovss	3808(%r12), %xmm0
.Ltmp7499:
	.loc	1 1192 28
	vmovss	%xmm0, 1052(%rsp)
.Ltmp7500:
	.loc	1 1279 33
	vmovss	2704(%r12), %xmm0
.Ltmp7501:
	.loc	1 1192 28
	vmovss	%xmm0, 1056(%rsp)
.Ltmp7502:
	.loc	1 1279 33
	vmovss	2864(%r12), %xmm0
.Ltmp7503:
	.loc	1 1192 28
	vmovss	%xmm0, 1060(%rsp)
.Ltmp7504:
	.loc	1 1279 33
	vmovss	3024(%r12), %xmm0
.Ltmp7505:
	.loc	1 1192 28
	vmovss	%xmm0, 1064(%rsp)
.Ltmp7506:
	.loc	1 1279 33
	vmovss	3184(%r12), %xmm0
.Ltmp7507:
	.loc	1 1192 28
	vmovss	%xmm0, 1068(%rsp)
.Ltmp7508:
	.loc	1 1279 33
	vmovss	3344(%r12), %xmm0
.Ltmp7509:
	.loc	1 1192 28
	vmovss	%xmm0, 1072(%rsp)
.Ltmp7510:
	.loc	1 1279 33
	vmovss	3504(%r12), %xmm0
.Ltmp7511:
	.loc	1 1192 28
	vmovss	%xmm0, 1076(%rsp)
.Ltmp7512:
	.loc	1 1279 33
	vmovss	3664(%r12), %xmm0
.Ltmp7513:
	.loc	1 1192 28
	vmovss	%xmm0, 1080(%rsp)
.Ltmp7514:
	.loc	1 1279 33
	vmovss	3824(%r12), %xmm0
.Ltmp7515:
	.loc	1 1192 28
	vmovss	%xmm0, 1084(%rsp)
.Ltmp7516:
	.loc	1 1279 33
	vmovss	2720(%r12), %xmm0
.Ltmp7517:
	.loc	1 1192 28
	vmovss	%xmm0, 1088(%rsp)
.Ltmp7518:
	.loc	1 1279 33
	vmovss	2880(%r12), %xmm0
.Ltmp7519:
	.loc	1 1192 28
	vmovss	%xmm0, 1092(%rsp)
.Ltmp7520:
	.loc	1 1279 33
	vmovss	3040(%r12), %xmm0
.Ltmp7521:
	.loc	1 1192 28
	vmovss	%xmm0, 1096(%rsp)
.Ltmp7522:
	.loc	1 1279 33
	vmovss	3200(%r12), %xmm0
.Ltmp7523:
	.loc	1 1192 28
	vmovss	%xmm0, 1100(%rsp)
.Ltmp7524:
	.loc	1 1279 33
	vmovss	3360(%r12), %xmm0
.Ltmp7525:
	.loc	1 1192 28
	vmovss	%xmm0, 1104(%rsp)
.Ltmp7526:
	.loc	1 1279 33
	vmovss	3520(%r12), %xmm0
.Ltmp7527:
	.loc	1 1192 28
	vmovss	%xmm0, 1108(%rsp)
.Ltmp7528:
	.loc	1 1279 33
	vmovss	3680(%r12), %xmm0
.Ltmp7529:
	.loc	1 1192 28
	vmovss	%xmm0, 1112(%rsp)
.Ltmp7530:
	.loc	1 1279 33
	vmovss	3840(%r12), %xmm0
.Ltmp7531:
	.loc	1 1192 28
	vmovss	%xmm0, 1116(%rsp)
.Ltmp7532:
	.loc	1 1279 33
	vmovss	2736(%r12), %xmm0
.Ltmp7533:
	.loc	1 1192 28
	vmovss	%xmm0, 1120(%rsp)
.Ltmp7534:
	.loc	1 1279 33
	vmovss	2896(%r12), %xmm0
.Ltmp7535:
	.loc	1 1192 28
	vmovss	%xmm0, 1124(%rsp)
.Ltmp7536:
	.loc	1 1279 33
	vmovss	3056(%r12), %xmm0
.Ltmp7537:
	.loc	1 1192 28
	vmovss	%xmm0, 1128(%rsp)
.Ltmp7538:
	.loc	1 1279 33
	vmovss	3216(%r12), %xmm0
.Ltmp7539:
	.loc	1 1192 28
	vmovss	%xmm0, 1132(%rsp)
.Ltmp7540:
	.loc	1 1279 33
	vmovss	3376(%r12), %xmm0
.Ltmp7541:
	.loc	1 1192 28
	vmovss	%xmm0, 1136(%rsp)
.Ltmp7542:
	.loc	1 1279 33
	vmovss	3536(%r12), %xmm0
.Ltmp7543:
	.loc	1 1192 28
	vmovss	%xmm0, 1140(%rsp)
.Ltmp7544:
	.loc	1 1279 33
	vmovss	3696(%r12), %xmm0
.Ltmp7545:
	.loc	1 1192 28
	vmovss	%xmm0, 1144(%rsp)
.Ltmp7546:
	.loc	1 1279 33
	vmovss	3856(%r12), %xmm0
.Ltmp7547:
	.loc	1 1192 28
	vmovss	%xmm0, 1148(%rsp)
.Ltmp7548:
	.loc	1 1279 33
	vmovss	2752(%r12), %xmm0
.Ltmp7549:
	.loc	1 1192 28
	vmovss	%xmm0, 1152(%rsp)
.Ltmp7550:
	.loc	1 1279 33
	vmovss	2912(%r12), %xmm0
.Ltmp7551:
	.loc	1 1192 28
	vmovss	%xmm0, 1156(%rsp)
.Ltmp7552:
	.loc	1 1279 33
	vmovss	3072(%r12), %xmm0
.Ltmp7553:
	.loc	1 1192 28
	vmovss	%xmm0, 1160(%rsp)
.Ltmp7554:
	.loc	1 1279 33
	vmovss	3232(%r12), %xmm0
.Ltmp7555:
	.loc	1 1192 28
	vmovss	%xmm0, 1164(%rsp)
.Ltmp7556:
	.loc	1 1279 33
	vmovss	3392(%r12), %xmm0
.Ltmp7557:
	.loc	1 1192 28
	vmovss	%xmm0, 1168(%rsp)
.Ltmp7558:
	.loc	1 1279 33
	vmovss	3552(%r12), %xmm0
.Ltmp7559:
	.loc	1 1192 28
	vmovss	%xmm0, 1172(%rsp)
.Ltmp7560:
	.loc	1 1279 33
	vmovss	3712(%r12), %xmm0
.Ltmp7561:
	.loc	1 1192 28
	vmovss	%xmm0, 1176(%rsp)
.Ltmp7562:
	.loc	1 1279 33
	vmovss	3872(%r12), %xmm0
.Ltmp7563:
	.loc	1 1192 28
	vmovss	%xmm0, 1180(%rsp)
.Ltmp7564:
	.loc	1 1279 33
	vmovss	2768(%r12), %xmm0
.Ltmp7565:
	.loc	1 1192 28
	vmovss	%xmm0, 1184(%rsp)
.Ltmp7566:
	.loc	1 1279 33
	vmovss	2928(%r12), %xmm0
.Ltmp7567:
	.loc	1 1192 28
	vmovss	%xmm0, 1188(%rsp)
.Ltmp7568:
	.loc	1 1279 33
	vmovss	3088(%r12), %xmm0
.Ltmp7569:
	.loc	1 1192 28
	vmovss	%xmm0, 1192(%rsp)
.Ltmp7570:
	.loc	1 1279 33
	vmovss	3248(%r12), %xmm0
.Ltmp7571:
	.loc	1 1192 28
	vmovss	%xmm0, 1196(%rsp)
.Ltmp7572:
	.loc	1 1279 33
	vmovss	3408(%r12), %xmm0
.Ltmp7573:
	.loc	1 1192 28
	vmovss	%xmm0, 1200(%rsp)
.Ltmp7574:
	.loc	1 1279 33
	vmovss	3568(%r12), %xmm0
.Ltmp7575:
	.loc	1 1192 28
	vmovss	%xmm0, 1204(%rsp)
.Ltmp7576:
	.loc	1 1279 33
	vmovss	3728(%r12), %xmm0
.Ltmp7577:
	.loc	1 1192 28
	vmovss	%xmm0, 1208(%rsp)
.Ltmp7578:
	.loc	1 1279 33
	vmovss	3888(%r12), %xmm0
.Ltmp7579:
	.loc	1 1192 28
	vmovss	%xmm0, 1212(%rsp)
.Ltmp7580:
	.loc	1 1280 32
	vmovss	2632(%r12), %xmm0
.Ltmp7581:
	.loc	1 1192 28
	vmovss	%xmm0, 1216(%rsp)
.Ltmp7582:
	.loc	1 1280 32
	vmovss	2792(%r12), %xmm0
.Ltmp7583:
	.loc	1 1192 28
	vmovss	%xmm0, 1220(%rsp)
.Ltmp7584:
	.loc	1 1280 32
	vmovss	2952(%r12), %xmm0
.Ltmp7585:
	.loc	1 1192 28
	vmovss	%xmm0, 1224(%rsp)
.Ltmp7586:
	.loc	1 1280 32
	vmovss	3112(%r12), %xmm0
.Ltmp7587:
	.loc	1 1192 28
	vmovss	%xmm0, 1228(%rsp)
.Ltmp7588:
	.loc	1 1280 32
	vmovss	3272(%r12), %xmm0
.Ltmp7589:
	.loc	1 1192 28
	vmovss	%xmm0, 1232(%rsp)
.Ltmp7590:
	.loc	1 1280 32
	vmovss	3432(%r12), %xmm0
.Ltmp7591:
	.loc	1 1192 28
	vmovss	%xmm0, 1236(%rsp)
.Ltmp7592:
	.loc	1 1280 32
	vmovss	3592(%r12), %xmm0
.Ltmp7593:
	.loc	1 1192 28
	vmovss	%xmm0, 1240(%rsp)
.Ltmp7594:
	.loc	1 1280 32
	vmovss	3752(%r12), %xmm0
.Ltmp7595:
	.loc	1 1192 28
	vmovss	%xmm0, 1244(%rsp)
.Ltmp7596:
	.loc	1 1280 32
	vmovss	2648(%r12), %xmm0
.Ltmp7597:
	.loc	1 1192 28
	vmovss	%xmm0, 1248(%rsp)
.Ltmp7598:
	.loc	1 1280 32
	vmovss	2808(%r12), %xmm0
.Ltmp7599:
	.loc	1 1192 28
	vmovss	%xmm0, 1252(%rsp)
.Ltmp7600:
	.loc	1 1280 32
	vmovss	2968(%r12), %xmm0
.Ltmp7601:
	.loc	1 1192 28
	vmovss	%xmm0, 1256(%rsp)
.Ltmp7602:
	.loc	1 1280 32
	vmovss	3128(%r12), %xmm0
.Ltmp7603:
	.loc	1 1192 28
	vmovss	%xmm0, 1260(%rsp)
.Ltmp7604:
	.loc	1 1280 32
	vmovss	3288(%r12), %xmm0
.Ltmp7605:
	.loc	1 1192 28
	vmovss	%xmm0, 1264(%rsp)
.Ltmp7606:
	.loc	1 1280 32
	vmovss	3448(%r12), %xmm0
.Ltmp7607:
	.loc	1 1192 28
	vmovss	%xmm0, 1268(%rsp)
.Ltmp7608:
	.loc	1 1280 32
	vmovss	3608(%r12), %xmm0
.Ltmp7609:
	.loc	1 1192 28
	vmovss	%xmm0, 1272(%rsp)
.Ltmp7610:
	.loc	1 1280 32
	vmovss	3768(%r12), %xmm0
.Ltmp7611:
	.loc	1 1192 28
	vmovss	%xmm0, 1276(%rsp)
.Ltmp7612:
	.loc	1 1280 32
	vmovss	2664(%r12), %xmm0
.Ltmp7613:
	.loc	1 1192 28
	vmovss	%xmm0, 1280(%rsp)
.Ltmp7614:
	.loc	1 1280 32
	vmovss	2824(%r12), %xmm0
.Ltmp7615:
	.loc	1 1192 28
	vmovss	%xmm0, 1284(%rsp)
.Ltmp7616:
	.loc	1 1280 32
	vmovss	2984(%r12), %xmm0
.Ltmp7617:
	.loc	1 1192 28
	vmovss	%xmm0, 1288(%rsp)
.Ltmp7618:
	.loc	1 1280 32
	vmovss	3144(%r12), %xmm0
.Ltmp7619:
	.loc	1 1192 28
	vmovss	%xmm0, 1292(%rsp)
.Ltmp7620:
	.loc	1 1280 32
	vmovss	3304(%r12), %xmm0
.Ltmp7621:
	.loc	1 1192 28
	vmovss	%xmm0, 1296(%rsp)
.Ltmp7622:
	.loc	1 1280 32
	vmovss	3464(%r12), %xmm0
.Ltmp7623:
	.loc	1 1192 28
	vmovss	%xmm0, 1300(%rsp)
.Ltmp7624:
	.loc	1 1280 32
	vmovss	3624(%r12), %xmm0
.Ltmp7625:
	.loc	1 1192 28
	vmovss	%xmm0, 1304(%rsp)
.Ltmp7626:
	.loc	1 1280 32
	vmovss	3784(%r12), %xmm0
.Ltmp7627:
	.loc	1 1192 28
	vmovss	%xmm0, 1308(%rsp)
.Ltmp7628:
	.loc	1 1280 32
	vmovss	2680(%r12), %xmm0
.Ltmp7629:
	.loc	1 1192 28
	vmovss	%xmm0, 1312(%rsp)
.Ltmp7630:
	.loc	1 1280 32
	vmovss	2840(%r12), %xmm0
.Ltmp7631:
	.loc	1 1192 28
	vmovss	%xmm0, 1316(%rsp)
.Ltmp7632:
	.loc	1 1280 32
	vmovss	3000(%r12), %xmm0
.Ltmp7633:
	.loc	1 1192 28
	vmovss	%xmm0, 1320(%rsp)
.Ltmp7634:
	.loc	1 1280 32
	vmovss	3160(%r12), %xmm0
.Ltmp7635:
	.loc	1 1192 28
	vmovss	%xmm0, 1324(%rsp)
.Ltmp7636:
	.loc	1 1280 32
	vmovss	3320(%r12), %xmm0
.Ltmp7637:
	.loc	1 1192 28
	vmovss	%xmm0, 1328(%rsp)
.Ltmp7638:
	.loc	1 1280 32
	vmovss	3480(%r12), %xmm0
.Ltmp7639:
	.loc	1 1192 28
	vmovss	%xmm0, 1332(%rsp)
.Ltmp7640:
	.loc	1 1280 32
	vmovss	3640(%r12), %xmm0
.Ltmp7641:
	.loc	1 1192 28
	vmovss	%xmm0, 1336(%rsp)
.Ltmp7642:
	.loc	1 1280 32
	vmovss	3800(%r12), %xmm0
.Ltmp7643:
	.loc	1 1192 28
	vmovss	%xmm0, 1340(%rsp)
.Ltmp7644:
	.loc	1 1280 32
	vmovss	2696(%r12), %xmm0
.Ltmp7645:
	.loc	1 1192 28
	vmovss	%xmm0, 1344(%rsp)
.Ltmp7646:
	.loc	1 1280 32
	vmovss	2856(%r12), %xmm0
.Ltmp7647:
	.loc	1 1192 28
	vmovss	%xmm0, 1348(%rsp)
.Ltmp7648:
	.loc	1 1280 32
	vmovss	3016(%r12), %xmm0
.Ltmp7649:
	.loc	1 1192 28
	vmovss	%xmm0, 1352(%rsp)
.Ltmp7650:
	.loc	1 1280 32
	vmovss	3176(%r12), %xmm0
.Ltmp7651:
	.loc	1 1192 28
	vmovss	%xmm0, 1356(%rsp)
.Ltmp7652:
	.loc	1 1280 32
	vmovss	3336(%r12), %xmm0
.Ltmp7653:
	.loc	1 1192 28
	vmovss	%xmm0, 1360(%rsp)
.Ltmp7654:
	.loc	1 1280 32
	vmovss	3496(%r12), %xmm0
.Ltmp7655:
	.loc	1 1192 28
	vmovss	%xmm0, 1364(%rsp)
.Ltmp7656:
	.loc	1 1280 32
	vmovss	3656(%r12), %xmm0
.Ltmp7657:
	.loc	1 1192 28
	vmovss	%xmm0, 1368(%rsp)
.Ltmp7658:
	.loc	1 1280 32
	vmovss	3816(%r12), %xmm0
.Ltmp7659:
	.loc	1 1192 28
	vmovss	%xmm0, 1372(%rsp)
.Ltmp7660:
	.loc	1 1280 32
	vmovss	2712(%r12), %xmm0
.Ltmp7661:
	.loc	1 1192 28
	vmovss	%xmm0, 1376(%rsp)
.Ltmp7662:
	.loc	1 1280 32
	vmovss	2872(%r12), %xmm0
.Ltmp7663:
	.loc	1 1192 28
	vmovss	%xmm0, 1380(%rsp)
.Ltmp7664:
	.loc	1 1280 32
	vmovss	3032(%r12), %xmm0
.Ltmp7665:
	.loc	1 1192 28
	vmovss	%xmm0, 1384(%rsp)
.Ltmp7666:
	.loc	1 1280 32
	vmovss	3192(%r12), %xmm0
.Ltmp7667:
	.loc	1 1192 28
	vmovss	%xmm0, 1388(%rsp)
.Ltmp7668:
	.loc	1 1280 32
	vmovss	3352(%r12), %xmm0
.Ltmp7669:
	.loc	1 1192 28
	vmovss	%xmm0, 1392(%rsp)
.Ltmp7670:
	.loc	1 1280 32
	vmovss	3512(%r12), %xmm0
.Ltmp7671:
	.loc	1 1192 28
	vmovss	%xmm0, 1396(%rsp)
.Ltmp7672:
	.loc	1 1280 32
	vmovss	3672(%r12), %xmm0
.Ltmp7673:
	.loc	1 1192 28
	vmovss	%xmm0, 1400(%rsp)
.Ltmp7674:
	.loc	1 1280 32
	vmovss	3832(%r12), %xmm0
.Ltmp7675:
	.loc	1 1192 28
	vmovss	%xmm0, 1404(%rsp)
.Ltmp7676:
	.loc	1 1280 32
	vmovss	2728(%r12), %xmm0
.Ltmp7677:
	.loc	1 1192 28
	vmovss	%xmm0, 1408(%rsp)
.Ltmp7678:
	.loc	1 1280 32
	vmovss	2888(%r12), %xmm0
.Ltmp7679:
	.loc	1 1192 28
	vmovss	%xmm0, 1412(%rsp)
.Ltmp7680:
	.loc	1 1280 32
	vmovss	3048(%r12), %xmm0
.Ltmp7681:
	.loc	1 1192 28
	vmovss	%xmm0, 1416(%rsp)
.Ltmp7682:
	.loc	1 1280 32
	vmovss	3208(%r12), %xmm0
.Ltmp7683:
	.loc	1 1192 28
	vmovss	%xmm0, 1420(%rsp)
.Ltmp7684:
	.loc	1 1280 32
	vmovss	3368(%r12), %xmm0
.Ltmp7685:
	.loc	1 1192 28
	vmovss	%xmm0, 1424(%rsp)
.Ltmp7686:
	.loc	1 1280 32
	vmovss	3528(%r12), %xmm0
.Ltmp7687:
	.loc	1 1192 28
	vmovss	%xmm0, 1428(%rsp)
.Ltmp7688:
	.loc	1 1280 32
	vmovss	3688(%r12), %xmm0
.Ltmp7689:
	.loc	1 1192 28
	vmovss	%xmm0, 1432(%rsp)
.Ltmp7690:
	.loc	1 1280 32
	vmovss	3848(%r12), %xmm0
.Ltmp7691:
	.loc	1 1192 28
	vmovss	%xmm0, 1436(%rsp)
.Ltmp7692:
	.loc	1 1280 32
	vmovss	2744(%r12), %xmm0
.Ltmp7693:
	.loc	1 1192 28
	vmovss	%xmm0, 1440(%rsp)
.Ltmp7694:
	.loc	1 1280 32
	vmovss	2904(%r12), %xmm0
.Ltmp7695:
	.loc	1 1192 28
	vmovss	%xmm0, 1444(%rsp)
.Ltmp7696:
	.loc	1 1280 32
	vmovss	3064(%r12), %xmm0
.Ltmp7697:
	.loc	1 1192 28
	vmovss	%xmm0, 1448(%rsp)
.Ltmp7698:
	.loc	1 1280 32
	vmovss	3224(%r12), %xmm0
.Ltmp7699:
	.loc	1 1192 28
	vmovss	%xmm0, 1452(%rsp)
.Ltmp7700:
	.loc	1 1280 32
	vmovss	3384(%r12), %xmm0
.Ltmp7701:
	.loc	1 1192 28
	vmovss	%xmm0, 1456(%rsp)
.Ltmp7702:
	.loc	1 1280 32
	vmovss	3544(%r12), %xmm0
.Ltmp7703:
	.loc	1 1192 28
	vmovss	%xmm0, 1460(%rsp)
.Ltmp7704:
	.loc	1 1280 32
	vmovss	3704(%r12), %xmm0
.Ltmp7705:
	.loc	1 1192 28
	vmovss	%xmm0, 1464(%rsp)
.Ltmp7706:
	.loc	1 1280 32
	vmovss	3864(%r12), %xmm0
.Ltmp7707:
	.loc	1 1192 28
	vmovss	%xmm0, 1468(%rsp)
.Ltmp7708:
	.loc	1 1280 32
	vmovss	2760(%r12), %xmm0
.Ltmp7709:
	.loc	1 1192 28
	vmovss	%xmm0, 1472(%rsp)
.Ltmp7710:
	.loc	1 1280 32
	vmovss	2920(%r12), %xmm0
.Ltmp7711:
	.loc	1 1192 28
	vmovss	%xmm0, 1476(%rsp)
.Ltmp7712:
	.loc	1 1280 32
	vmovss	3080(%r12), %xmm0
.Ltmp7713:
	.loc	1 1192 28
	vmovss	%xmm0, 1480(%rsp)
.Ltmp7714:
	.loc	1 1280 32
	vmovss	3240(%r12), %xmm0
.Ltmp7715:
	.loc	1 1192 28
	vmovss	%xmm0, 1484(%rsp)
.Ltmp7716:
	.loc	1 1280 32
	vmovss	3400(%r12), %xmm0
.Ltmp7717:
	.loc	1 1192 28
	vmovss	%xmm0, 1488(%rsp)
.Ltmp7718:
	.loc	1 1280 32
	vmovss	3560(%r12), %xmm0
.Ltmp7719:
	.loc	1 1192 28
	vmovss	%xmm0, 1492(%rsp)
.Ltmp7720:
	.loc	1 1280 32
	vmovss	3720(%r12), %xmm0
.Ltmp7721:
	.loc	1 1192 28
	vmovss	%xmm0, 1496(%rsp)
.Ltmp7722:
	.loc	1 1280 32
	vmovss	3880(%r12), %xmm0
.Ltmp7723:
	.loc	1 1192 28
	vmovss	%xmm0, 1500(%rsp)
.Ltmp7724:
	.loc	1 1280 32
	vmovss	2776(%r12), %xmm0
.Ltmp7725:
	.loc	1 1192 28
	vmovss	%xmm0, 1504(%rsp)
.Ltmp7726:
	.loc	1 1280 32
	vmovss	2936(%r12), %xmm0
.Ltmp7727:
	.loc	1 1192 28
	vmovss	%xmm0, 1508(%rsp)
.Ltmp7728:
	.loc	1 1280 32
	vmovss	3096(%r12), %xmm0
.Ltmp7729:
	.loc	1 1192 28
	vmovss	%xmm0, 1512(%rsp)
.Ltmp7730:
	.loc	1 1280 32
	vmovss	3256(%r12), %xmm0
.Ltmp7731:
	.loc	1 1192 28
	vmovss	%xmm0, 1516(%rsp)
.Ltmp7732:
	.loc	1 1280 32
	vmovss	3416(%r12), %xmm0
.Ltmp7733:
	.loc	1 1192 28
	vmovss	%xmm0, 1520(%rsp)
.Ltmp7734:
	.loc	1 1280 32
	vmovss	3576(%r12), %xmm0
.Ltmp7735:
	.loc	1 1192 28
	vmovss	%xmm0, 1524(%rsp)
.Ltmp7736:
	.loc	1 1280 32
	vmovss	3736(%r12), %xmm0
.Ltmp7737:
	.loc	1 1192 28
	vmovss	%xmm0, 1528(%rsp)
.Ltmp7738:
	.loc	1 1280 32
	vmovss	3896(%r12), %xmm0
.Ltmp7739:
	.loc	1 1192 28
	vmovss	%xmm0, 1532(%rsp)
.Ltmp7740:
	.loc	1 1194 31
	leaq	3136(%rsp), %rdi
	movq	%r12, %rsi
	movl	1560(%rsp), %r14d
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	3328(%rsp), %rdi
	movq	2040(%rsp), %rsi
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	movq	192(%rsp), %rdi
	.loc	1 1193 28
	vmovaps	3136(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	3168(%rsp), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	3200(%rsp), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	3232(%rsp), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	3328(%rsp), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	3360(%rsp), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	3392(%rsp), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	3424(%rsp), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	3456(%rsp), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	3488(%rsp), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
.Ltmp7741:
	.loc	1 0 0 is_stmt 0
	leaq	(%rdi,%r13), %rax
	shlq	$3, %rdi
	leaq	(,%rax,8), %rsi
	.loc	1 1197 12 is_stmt 1
	testb	$1, 64(%rsp)
	movq	%r13, 2144(%rsp)
	movq	%rax, 160(%rsp)
	je	.LBB40_229
.Ltmp7742:
	.loc	38 1050 16
	cmpq	%rdi, %rsi
.Ltmp7743:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_654
.Ltmp7744:
	.loc	48 451 16 is_stmt 1
	cmpq	128(%rsp), %rsi
	ja	.LBB40_654
.Ltmp7745:
	.loc	48 451 16 is_stmt 0
	cmpq	120(%rsp), %rsi
	ja	.LBB40_660
.Ltmp7746:
	.loc	1 1053 27 is_stmt 1
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
.Ltmp7747:
	.loc	1 1054 26
	vmovaps	4032(%r12), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	4064(%r12), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	4128(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
.Ltmp7748:
	.loc	1 1055 25
	vmovaps	2304(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	2336(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
.Ltmp7749:
	.loc	1 1056 24
	vmovaps	4928(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	4960(%r12), %ymm3
.Ltmp7750:
	.loc	1 1057 24
	movq	5272(%r12), %r10
.Ltmp7751:
	.loc	1 871 17
	movq	2368(%r12), %rax
	movq	2376(%r12), %rcx
.Ltmp7752:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2384(%r12), %rdx
	xorq	%rax, %rdx
	movq	2392(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2400(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	2408(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2416(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	orq	%rcx, %rdx
	xorq	2424(%r12), %rax
	orq	%rdx, %rax
	sete	176(%rsp)
.Ltmp7753:
	.loc	1 871 17
	movq	4992(%r12), %rax
	movq	5000(%r12), %rcx
.Ltmp7754:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	5008(%r12), %rdx
	xorq	%rax, %rdx
	movq	5016(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5024(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	5032(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5040(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	xorq	5048(%r12), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	184(%rsp)
.Ltmp7755:
	.loc	2 1916 50
	testq	%r13, %r13
	je	.LBB40_225
.Ltmp7756:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r13,8), %rax
	movq	%rax, 168(%rsp)
	movq	240(%rsp), %rax
	leaq	(%rax,%rdi,4), %rcx
	movq	248(%rsp), %rax
	leaq	(%rax,%rdi,4), %rdx
.Ltmp7757:
	.loc	3 900 12 is_stmt 1
	movq	%r13, %rsi
	movabsq	$2305843009213693951, %rax
	andq	%rax, %rsi
	movq	%rsi, 152(%rsp)
	xorl	%edi, %edi
	xorl	%r9d, %r9d
	movq	%rcx, 136(%rsp)
	movq	%rdx, 48(%rsp)
.Ltmp7758:
	.loc	3 0 12 is_stmt 0
.Ltmp7759:
	.p2align	4
.LBB40_160:
	.loc	1 1064 21 is_stmt 1
	vmovaps	256(%rsp), %ymm0
	vmovaps	288(%rsp), %ymm1
	vmovaps	320(%rsp), %ymm2
.Ltmp7760:
	.loc	14 48 14
	vaddps	576(%rsp), %ymm0, %ymm0
.Ltmp7761:
	.loc	1 1063 17
	vmovaps	%ymm0, 256(%rsp)
	.loc	1 1066 21
	vmovaps	896(%rsp), %ymm0
.Ltmp7762:
	.loc	14 48 14
	vaddps	1216(%rsp), %ymm0, %ymm0
.Ltmp7763:
	.loc	1 1065 17
	vmovaps	%ymm0, 896(%rsp)
.Ltmp7764:
	.loc	14 48 14
	vaddps	608(%rsp), %ymm1, %ymm0
.Ltmp7765:
	.loc	1 1063 17
	vmovaps	%ymm0, 288(%rsp)
	.loc	1 1066 21
	vmovaps	928(%rsp), %ymm0
.Ltmp7766:
	.loc	14 48 14
	vaddps	1248(%rsp), %ymm0, %ymm0
.Ltmp7767:
	.loc	1 1065 17
	vmovaps	%ymm0, 928(%rsp)
.Ltmp7768:
	.loc	14 48 14
	vaddps	640(%rsp), %ymm2, %ymm0
.Ltmp7769:
	.loc	1 1063 17
	vmovaps	%ymm0, 320(%rsp)
	.loc	1 1066 21
	vmovaps	960(%rsp), %ymm0
.Ltmp7770:
	.loc	14 48 14
	vaddps	1280(%rsp), %ymm0, %ymm0
.Ltmp7771:
	.loc	1 1065 17
	vmovaps	%ymm0, 960(%rsp)
	.loc	1 1064 21
	vmovaps	352(%rsp), %ymm0
.Ltmp7772:
	.loc	14 48 14
	vaddps	672(%rsp), %ymm0, %ymm0
.Ltmp7773:
	.loc	1 1063 17
	vmovaps	%ymm0, 352(%rsp)
	.loc	1 1066 21
	vmovaps	992(%rsp), %ymm0
.Ltmp7774:
	.loc	14 48 14
	vaddps	1312(%rsp), %ymm0, %ymm0
.Ltmp7775:
	.loc	1 1065 17
	vmovaps	%ymm0, 992(%rsp)
	.loc	1 1064 21
	vmovaps	384(%rsp), %ymm0
.Ltmp7776:
	.loc	14 48 14
	vaddps	704(%rsp), %ymm0, %ymm0
.Ltmp7777:
	.loc	1 1063 17
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 1066 21
	vmovaps	1024(%rsp), %ymm0
.Ltmp7778:
	.loc	14 48 14
	vaddps	1344(%rsp), %ymm0, %ymm0
.Ltmp7779:
	.loc	1 1065 17
	vmovaps	%ymm0, 1024(%rsp)
	.loc	1 1064 21
	vmovaps	416(%rsp), %ymm0
.Ltmp7780:
	.loc	14 48 14
	vaddps	736(%rsp), %ymm0, %ymm0
.Ltmp7781:
	.loc	1 1063 17
	vmovaps	%ymm0, 416(%rsp)
	.loc	1 1066 21
	vmovaps	1056(%rsp), %ymm0
.Ltmp7782:
	.loc	14 48 14
	vaddps	1376(%rsp), %ymm0, %ymm0
.Ltmp7783:
	.loc	1 1065 17
	vmovaps	%ymm0, 1056(%rsp)
	.loc	1 1064 21
	vmovaps	448(%rsp), %ymm0
.Ltmp7784:
	.loc	14 48 14
	vaddps	768(%rsp), %ymm0, %ymm0
.Ltmp7785:
	.loc	1 1063 17
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 1066 21
	vmovaps	1088(%rsp), %ymm0
.Ltmp7786:
	.loc	14 48 14
	vaddps	1408(%rsp), %ymm0, %ymm0
.Ltmp7787:
	.loc	1 1065 17
	vmovaps	%ymm0, 1088(%rsp)
	.loc	1 1064 21
	vmovaps	480(%rsp), %ymm0
.Ltmp7788:
	.loc	14 48 14
	vaddps	800(%rsp), %ymm0, %ymm0
.Ltmp7789:
	.loc	1 1063 17
	vmovaps	%ymm0, 480(%rsp)
	.loc	1 1066 21
	vmovaps	1120(%rsp), %ymm0
.Ltmp7790:
	.loc	14 48 14
	vaddps	1440(%rsp), %ymm0, %ymm0
.Ltmp7791:
	.loc	1 1065 17
	vmovaps	%ymm0, 1120(%rsp)
	.loc	1 1064 21
	vmovaps	512(%rsp), %ymm0
.Ltmp7792:
	.loc	14 48 14
	vaddps	832(%rsp), %ymm0, %ymm0
.Ltmp7793:
	.loc	1 1063 17
	vmovaps	%ymm0, 512(%rsp)
	.loc	1 1066 21
	vmovaps	1152(%rsp), %ymm0
.Ltmp7794:
	.loc	14 48 14
	vaddps	1472(%rsp), %ymm0, %ymm0
.Ltmp7795:
	.loc	1 1065 17
	vmovaps	%ymm0, 1152(%rsp)
	.loc	1 1064 21
	vmovaps	544(%rsp), %ymm0
.Ltmp7796:
	.loc	14 48 14
	vaddps	864(%rsp), %ymm0, %ymm0
.Ltmp7797:
	.loc	1 1063 17
	vmovaps	%ymm0, 544(%rsp)
	.loc	1 1066 21
	vmovaps	1184(%rsp), %ymm0
.Ltmp7798:
	.loc	14 48 14
	vaddps	1504(%rsp), %ymm0, %ymm0
.Ltmp7799:
	.loc	1 1065 17
	vmovaps	%ymm0, 1184(%rsp)
.Ltmp7800:
	.loc	1 1070 28
	leaq	1(%r10), %rax
.Ltmp7801:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %r15d
	cmovaeq	%rbx, %r15
.Ltmp7802:
	.loc	48 568 12
	cmpq	168(%rsp), %rdi
	ja	.LBB40_639
.Ltmp7803:
	.loc	48 438 16
	cmpq	%r9, 152(%rsp)
	je	.LBB40_641
.Ltmp7804:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,8), %rax
.Ltmp7805:
	.loc	1 1083 29 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp7806:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_642
.Ltmp7807:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7808:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm3, 2176(%rsp)
	vmovups	(%rcx,%rdi,4), %ymm1
.Ltmp7809:
	vmovups	(%rdx,%rdi,4), %ymm7
.Ltmp7810:
	vmovaps	1280(%r12), %ymm10
	vmovaps	1312(%r12), %ymm11
	vmovaps	1344(%r12), %ymm0
	vmovaps	3904(%r12), %ymm15
	vmovaps	3936(%r12), %ymm9
	vmovaps	3968(%r12), %ymm6
	vmovaps	1568(%rsp), %ymm8
	vsubps	%ymm8, %ymm1, %ymm2
	vmulps	%ymm2, %ymm11, %ymm3
	vmovaps	1728(%rsp), %ymm12
	vmulps	%ymm10, %ymm12, %ymm4
	vaddps	%ymm3, %ymm4, %ymm5
	vaddps	%ymm5, %ymm12, %ymm3
	vmulps	%ymm11, %ymm12, %ymm4
	vmulps	%ymm0, %ymm2, %ymm2
	vaddps	%ymm2, %ymm4, %ymm4
	vaddps	%ymm4, %ymm8, %ymm2
	vmulps	1376(%r12), %ymm3, %ymm12
	vmovaps	1600(%rsp), %ymm8
	vsubps	%ymm8, %ymm2, %ymm3
	vmulps	1632(%rsp), %ymm11, %ymm2
	vmulps	%ymm3, %ymm0, %ymm0
	vaddps	%ymm0, %ymm2, %ymm14
	vaddps	%ymm14, %ymm8, %ymm2
.Ltmp7811:
	vsubps	192(%rsp), %ymm7, %ymm13
	vmulps	%ymm9, %ymm13, %ymm0
	vmovaps	64(%rsp), %ymm8
	vmovaps	%ymm15, 1920(%rsp)
	vmulps	%ymm15, %ymm8, %ymm15
	vaddps	%ymm0, %ymm15, %ymm0
	vaddps	%ymm0, %ymm8, %ymm15
	vmulps	4000(%r12), %ymm15, %ymm15
.Ltmp7812:
	.loc	1 1083 29 is_stmt 1
	movq	2592(%r12), %rcx
.Ltmp7813:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp7814:
	.loc	1 1084 30
	movq	2616(%r12), %rsi
.Ltmp7815:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_643
.Ltmp7816:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7817:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm1, %ymm12, %ymm1
	vsubps	%ymm2, %ymm1, %ymm1
.Ltmp7818:
	.loc	1 1084 30 is_stmt 1
	movq	2608(%r12), %rcx
.Ltmp7819:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rax,4)
.Ltmp7820:
	.loc	1 1085 28
	movq	5224(%r12), %rsi
.Ltmp7821:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_644
.Ltmp7822:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7823:
	.loc	1 0 0 is_stmt 0
	vmulps	64(%rsp), %ymm9, %ymm1
	vmulps	%ymm6, %ymm13, %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
	vaddps	192(%rsp), %ymm1, %ymm2
	vmovaps	1760(%rsp), %ymm12
	vsubps	%ymm12, %ymm2, %ymm13
	vmovaps	2272(%rsp), %ymm8
	vmulps	%ymm9, %ymm8, %ymm2
	vmulps	%ymm6, %ymm13, %ymm6
	vaddps	%ymm6, %ymm2, %ymm6
	vaddps	%ymm6, %ymm12, %ymm2
.Ltmp7824:
	.loc	1 1085 28 is_stmt 1
	movq	5216(%r12), %rcx
.Ltmp7825:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp7826:
	.loc	1 1086 29
	movq	5240(%r12), %rsi
.Ltmp7827:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_645
.Ltmp7828:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7829:
	.loc	48 0 16 is_stmt 0
	movq	%r9, 2208(%rsp)
	movq	%rdi, 2368(%rsp)
	vaddps	%ymm7, %ymm15, %ymm7
	vsubps	%ymm2, %ymm7, %ymm2
.Ltmp7830:
	.loc	1 1086 29 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp7831:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp7832:
	.loc	1 1089 13
	movq	2592(%r12), %rdi
	movq	2600(%r12), %rsi
	movq	2368(%r12), %rax
.Ltmp7833:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp7834:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp7835:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 176(%rsp)
	movq	%r15, 2080(%rsp)
	je	.LBB40_176
.Ltmp7836:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB40_652
.Ltmp7837:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7838:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp7839:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
.Ltmp7840:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp7841:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp7842:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7843:
	.loc	1 0 0 is_stmt 0
	vmovups	(%rdi,%r9,4), %ymm2
	vmovaps	%ymm2, 1664(%rsp)
.Ltmp7844:
	movq	2608(%r12), %rcx
.Ltmp7845:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm2
.Ltmp7846:
	.loc	1 961 2
	jmp	.LBB40_193
.Ltmp7847:
	.loc	1 0 2 is_stmt 0
.Ltmp7848:
	.p2align	4
.LBB40_176:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB40_665
	.loc	1 0 25 is_stmt 0
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7849:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7850:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_695
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7851:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7852:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_694
	.loc	1 0 25
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7853:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7854:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_688
	.loc	1 0 25
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7855:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7856:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_677
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7857:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7858:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 1824(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_683
	.loc	1 0 25
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7859:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7860:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 1664(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_679
	.loc	1 0 25
	movq	2424(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7861:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7862:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_687
.Ltmp7863:
	.loc	1 0 25
	movq	%rcx, 1888(%rsp)
.Ltmp7864:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
.Ltmp7865:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp7866:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp7867:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7868:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7869:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 1856(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_668
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7870:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7871:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rdx
	movq	%rdx, 2048(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_678
	.loc	1 0 25
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7872:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7873:
	.loc	1 955 30
	leaq	3(,%rcx,8), %rdx
	movq	%rdx, 24(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_686
	.loc	1 0 25
	movq	%r13, 32(%rsp)
	movq	%r14, 16(%rsp)
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7874:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7875:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_690
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7876:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7877:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_689
	.loc	1 0 25
	movq	%r8, %r13
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7878:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movq	%r10, %r8
	movq	%r12, %r10
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %rcx
.Ltmp7879:
	.loc	1 955 30
	leaq	6(,%rcx,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	2424(%r10), %rcx
	movq	%r8, %r15
	.loc	1 955 35
	addq	%r8, %rcx
.Ltmp7880:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %r8d
	cmovaeq	%rbx, %r8
	subq	%r8, %rcx
.Ltmp7881:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
.Ltmp7882:
	.loc	1 0 25
	movq	32(%rsp), %rsi
	vmovd	(%rdi,%rsi,4), %xmm2
	movq	1824(%rsp), %rsi
	vpinsrd	$1, (%rdi,%rsi,4), %xmm2, %xmm2
	movq	1664(%rsp), %rsi
	vpinsrd	$2, (%rdi,%rsi,4), %xmm2, %xmm2
	movq	1888(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm2, %xmm2
	vmovd	(%rdi,%r9,4), %xmm7
	vpinsrd	$1, (%rdi,%r13,4), %xmm7, %xmm7
	vpinsrd	$2, (%rdi,%r11,4), %xmm7, %xmm7
	movq	16(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm7, %xmm7
.Ltmp7883:
	movq	2608(%r10), %rsi
.Ltmp7884:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm12
	vpinsrd	$1, (%rsi,%rdx,4), %xmm12, %xmm12
	vpinsrd	$2, (%rsi,%r12,4), %xmm12, %xmm12
	vpinsrd	$3, (%rsi,%rcx,4), %xmm12, %xmm12
	vmovd	(%rsi,%rax,4), %xmm15
	movq	1856(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm15, %xmm15
	movq	2048(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm15, %xmm15
	movq	24(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm15, %xmm15
.Ltmp7885:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm2, %ymm7, %ymm2
	vmovdqa	%ymm2, 1664(%rsp)
.Ltmp7886:
	.loc	8 551 14
	vinserti128	$1, %xmm12, %ymm15, %ymm2
	movq	%r10, %r12
	movq	2144(%rsp), %r13
	movq	%r15, %r10
	movq	2240(%rsp), %rbx
	movq	2080(%rsp), %r15
.Ltmp7887:
.LBB40_193:
	.loc	1 1103 13 is_stmt 1
	movq	5216(%r12), %r11
	movq	5224(%r12), %rsi
	movq	4992(%r12), %rax
.Ltmp7888:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp7889:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	movq	%rax, %rdi
	subq	%rcx, %rdi
.Ltmp7890:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rdi
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 184(%rsp)
	vmovdqa	%ymm2, 1824(%rsp)
	je	.LBB40_199
.Ltmp7891:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rdi, %r8
	jb	.LBB40_653
.Ltmp7892:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7893:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp7894:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
.Ltmp7895:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp7896:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp7897:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7898:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r11,%rdi,4), %ymm2
.Ltmp7899:
	movq	5232(%r12), %rcx
.Ltmp7900:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm7
.Ltmp7901:
	.loc	1 961 2
	jmp	.LBB40_216
.Ltmp7902:
	.loc	1 0 2 is_stmt 0
.Ltmp7903:
	.p2align	4
.LBB40_199:
	.loc	1 955 25 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB40_666
	.loc	1 0 25 is_stmt 0
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7904:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7905:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_697
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7906:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7907:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r9
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB40_665
	.loc	1 0 25
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7908:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7909:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_699
	.loc	1 0 25
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7910:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7911:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_704
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7912:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7913:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 1888(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_714
	.loc	1 0 25
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7914:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7915:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 1856(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_693
	.loc	1 0 25
	movq	5048(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7916:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7917:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_706
.Ltmp7918:
	.loc	1 0 25
	movq	%rcx, 2048(%rsp)
.Ltmp7919:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
.Ltmp7920:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp7921:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp7922:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7923:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7924:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 24(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_698
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7925:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7926:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rdx
	movq	%rdx, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_702
	.loc	1 0 25
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7927:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7928:
	.loc	1 955 30
	leaq	3(,%rcx,8), %rdx
	movq	%rdx, 32(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_691
	.loc	1 0 25
	movq	%r13, 56(%rsp)
	movq	%r14, 40(%rsp)
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7929:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7930:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_734
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7931:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp7932:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_710
	.loc	1 0 25
	movq	%rdi, %r13
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp7933:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movq	%r10, %rdi
	movq	%r12, %r10
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %rcx
.Ltmp7934:
	.loc	1 955 30
	leaq	6(,%rcx,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	5048(%r10), %rcx
	movq	%rdi, %r15
	.loc	1 955 35
	addq	%rdi, %rcx
.Ltmp7935:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rcx
.Ltmp7936:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
.Ltmp7937:
	.loc	1 0 25
	movq	56(%rsp), %rsi
	vmovd	(%r11,%rsi,4), %xmm2
	movq	1888(%rsp), %rsi
	vpinsrd	$1, (%r11,%rsi,4), %xmm2, %xmm2
	movq	1856(%rsp), %rsi
	vpinsrd	$2, (%r11,%rsi,4), %xmm2, %xmm2
	movq	2048(%rsp), %rsi
	vpinsrd	$3, (%r11,%rsi,4), %xmm2, %xmm2
	vmovd	(%r11,%r13,4), %xmm7
	vpinsrd	$1, (%r11,%r8,4), %xmm7, %xmm7
	vpinsrd	$2, (%r11,%r9,4), %xmm7, %xmm7
	movq	40(%rsp), %rsi
	vpinsrd	$3, (%r11,%rsi,4), %xmm7, %xmm7
.Ltmp7938:
	movq	5232(%r10), %rsi
.Ltmp7939:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm12
	vpinsrd	$1, (%rsi,%rdx,4), %xmm12, %xmm12
	vpinsrd	$2, (%rsi,%r12,4), %xmm12, %xmm12
	vpinsrd	$3, (%rsi,%rcx,4), %xmm12, %xmm12
	vmovd	(%rsi,%rax,4), %xmm15
	movq	24(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm15, %xmm15
	movq	16(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm15, %xmm15
	movq	32(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm15, %xmm15
.Ltmp7940:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm2, %ymm7, %ymm2
.Ltmp7941:
	.loc	8 551 14
	vinserti128	$1, %xmm12, %ymm15, %ymm7
	movq	%r10, %r12
	movq	2144(%rsp), %r13
	movq	%r15, %r10
	movq	2240(%rsp), %rbx
	movq	2080(%rsp), %r15
.Ltmp7942:
.LBB40_216:
	.loc	1 0 0
	negq	%r15
	addq	%r15, %r10
	incq	%r10
	leaq	(,%r10,8), %rax
.Ltmp7943:
	.loc	1 1150 36 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp7944:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movl	$0, %r15d
	movq	48(%rsp), %rdx
	movq	2368(%rsp), %rdi
	movq	2208(%rsp), %r9
	jb	.LBB40_647
.Ltmp7945:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7946:
	.loc	1 1152 27
	movq	2616(%r12), %rsi
.Ltmp7947:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_648
.Ltmp7948:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7949:
	.loc	1 1153 35
	movq	5224(%r12), %rsi
.Ltmp7950:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_649
.Ltmp7951:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7952:
	.loc	1 1155 27
	movq	5240(%r12), %rsi
.Ltmp7953:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_638
.Ltmp7954:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp7955:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm5, %ymm5, %ymm5
	vaddps	1728(%rsp), %ymm5, %ymm12
	vbroadcastss	.LCPI40_1(%rip), %ymm5
	vandps	%ymm5, %ymm12, %ymm15
	vmovdqa	%ymm7, 2080(%rsp)
	vbroadcastss	.LCPI40_2(%rip), %ymm7
	vcmplt_oqps	%ymm7, %ymm15, %ymm15
	vandnps	%ymm12, %ymm15, %ymm12
	vmovaps	%ymm12, 1728(%rsp)
	vaddps	%ymm4, %ymm4, %ymm4
	vaddps	1568(%rsp), %ymm4, %ymm4
	vandps	%ymm5, %ymm4, %ymm12
	vcmplt_oqps	%ymm7, %ymm12, %ymm12
	vandnps	%ymm4, %ymm12, %ymm4
	vmovaps	%ymm4, 1568(%rsp)
	vmulps	%ymm3, %ymm11, %ymm3
	vmovaps	1632(%rsp), %ymm11
	vmulps	%ymm10, %ymm11, %ymm4
	vaddps	%ymm3, %ymm4, %ymm3
	vaddps	%ymm3, %ymm3, %ymm3
	vaddps	%ymm3, %ymm11, %ymm3
	vandps	%ymm5, %ymm3, %ymm4
	vcmplt_oqps	%ymm7, %ymm4, %ymm4
	vandnps	%ymm3, %ymm4, %ymm3
	vmovaps	%ymm3, 1632(%rsp)
	vaddps	%ymm14, %ymm14, %ymm3
	vaddps	1600(%rsp), %ymm3, %ymm3
	vandps	%ymm5, %ymm3, %ymm4
	vcmplt_oqps	%ymm7, %ymm4, %ymm4
	vandnps	%ymm3, %ymm4, %ymm3
	vmovaps	%ymm3, 1600(%rsp)
.Ltmp7956:
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	64(%rsp), %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm3
	vcmplt_oqps	%ymm7, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vaddps	%ymm1, %ymm1, %ymm0
	vaddps	192(%rsp), %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm1
	vcmplt_oqps	%ymm7, %ymm1, %ymm1
	vmovaps	%ymm7, %ymm10
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmulps	%ymm13, %ymm9, %ymm0
	vmulps	1920(%rsp), %ymm8, %ymm1
	vaddps	%ymm0, %ymm1, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	%ymm0, %ymm8, %ymm0
	vandps	%ymm5, %ymm0, %ymm1
	vcmplt_oqps	%ymm7, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vaddps	%ymm6, %ymm6, %ymm0
	vaddps	1760(%rsp), %ymm0, %ymm4
.Ltmp7957:
	vandps	1664(%rsp), %ymm5, %ymm0
	vbroadcastss	.LCPI40_4(%rip), %ymm11
.Ltmp7958:
	vmaxps	%ymm11, %ymm0, %ymm1
	vbroadcastsd	.LCPI40_6(%rip), %ymm0
	vbroadcastss	.LCPI40_5(%rip), %ymm12
	vmaxps	%ymm12, %ymm1, %ymm3
	vandps	%ymm0, %ymm3, %ymm6
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm0, 1888(%rsp)
	vbroadcastsd	.LCPI40_7(%rip), %ymm12
	vorps	%ymm6, %ymm12, %ymm6
	vmovaps	%ymm12, 1856(%rsp)
	vbroadcastss	.LCPI40_8(%rip), %ymm13
	vaddps	%ymm6, %ymm13, %ymm6
	vbroadcastss	.LCPI40_9(%rip), %ymm14
	vmulps	%ymm6, %ymm14, %ymm8
	vmovdqa	%ymm2, 1664(%rsp)
	vmovaps	%ymm14, %ymm2
	vbroadcastss	.LCPI40_10(%rip), %ymm15
	vaddps	%ymm15, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_11(%rip), %ymm9
	vaddps	%ymm9, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_12(%rip), %ymm9
	vaddps	%ymm9, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_13(%rip), %ymm9
	vaddps	%ymm9, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_14(%rip), %ymm9
	vaddps	%ymm9, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm6
	vpsrld	$23, %ymm3, %ymm8
	vpbroadcastd	.LCPI40_15(%rip), %ymm3
	vpor	%ymm3, %ymm8, %ymm8
	vmovdqa	%ymm3, 2048(%rsp)
	vbroadcastss	.LCPI40_16(%rip), %ymm13
	vaddps	%ymm13, %ymm8, %ymm8
	vaddps	%ymm6, %ymm8, %ymm6
	vbroadcastss	.LCPI40_17(%rip), %ymm14
	vmulps	%ymm6, %ymm14, %ymm6
	vbroadcastss	.LCPI40_18(%rip), %ymm15
	vmaxps	%ymm15, %ymm6, %ymm6
	vbroadcastss	.LCPI40_19(%rip), %ymm0
	vminps	%ymm0, %ymm6, %ymm6
	vsubps	256(%rsp), %ymm6, %ymm6
	vbroadcastss	.LCPI40_20(%rip), %ymm11
	vaddps	%ymm6, %ymm11, %ymm8
	vmulps	%ymm8, %ymm8, %ymm8
	vbroadcastss	.LCPI40_22(%rip), %ymm7
	vmulps	%ymm7, %ymm8, %ymm8
	vcmpgt_oqps	%ymm11, %ymm6, %ymm9
	vblendvps	%ymm9, %ymm6, %ymm8, %ymm8
.Ltmp7959:
	vandps	%ymm5, %ymm4, %ymm9
	vcmplt_oqps	%ymm10, %ymm9, %ymm9
	vandnps	%ymm4, %ymm9, %ymm4
	vmovaps	%ymm4, 1760(%rsp)
	vbroadcastss	.LCPI40_21(%rip), %ymm13
.Ltmp7960:
	vcmple_oqps	%ymm13, %ymm6, %ymm4
	vmulps	2336(%rsp), %ymm8, %ymm6
	vxorps	%xmm8, %xmm8, %xmm8
	vpcmpgtd	%ymm4, %ymm8, %ymm4
	vpandn	%ymm6, %ymm4, %ymm4
	vbroadcastss	.LCPI40_23(%rip), %ymm14
	vmaxps	%ymm14, %ymm4, %ymm4
	vminps	%ymm8, %ymm4, %ymm4
	vmovaps	1952(%rsp), %ymm9
	vcmplt_oqps	%ymm9, %ymm4, %ymm6
	vmovaps	2560(%rsp), %ymm8
	vblendvps	%ymm6, 2592(%rsp), %ymm8, %ymm6
	vsubps	%ymm4, %ymm9, %ymm8
	vmulps	%ymm6, %ymm8, %ymm6
	vaddps	%ymm6, %ymm4, %ymm4
	vandps	%ymm5, %ymm4, %ymm6
	vcmplt_oqps	%ymm10, %ymm6, %ymm6
	vandnps	%ymm4, %ymm6, %ymm9
.Ltmp7961:
	vandps	1824(%rsp), %ymm5, %ymm4
.Ltmp7962:
	vbroadcastss	.LCPI40_4(%rip), %ymm6
	vmaxps	%ymm6, %ymm4, %ymm4
	vbroadcastss	.LCPI40_5(%rip), %ymm6
	vmaxps	%ymm6, %ymm4, %ymm4
	vandps	%ymm1, %ymm4, %ymm6
	vorps	%ymm6, %ymm12, %ymm6
	vbroadcastss	.LCPI40_8(%rip), %ymm1
	vaddps	%ymm1, %ymm6, %ymm6
	vmulps	%ymm2, %ymm6, %ymm8
	vbroadcastss	.LCPI40_10(%rip), %ymm1
	vaddps	%ymm1, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_11(%rip), %ymm1
	vaddps	%ymm1, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_12(%rip), %ymm1
	vaddps	%ymm1, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_13(%rip), %ymm1
	vaddps	%ymm1, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_14(%rip), %ymm1
	vaddps	%ymm1, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm6
	vpsrld	$23, %ymm4, %ymm4
	vpor	%ymm3, %ymm4, %ymm4
	vbroadcastss	.LCPI40_16(%rip), %ymm1
	vaddps	%ymm1, %ymm4, %ymm4
	vaddps	%ymm6, %ymm4, %ymm4
	vbroadcastss	.LCPI40_17(%rip), %ymm1
	vmulps	%ymm1, %ymm4, %ymm4
	vmaxps	%ymm15, %ymm4, %ymm4
	vminps	%ymm0, %ymm4, %ymm4
	vsubps	416(%rsp), %ymm4, %ymm4
	vaddps	%ymm4, %ymm11, %ymm6
	vmulps	%ymm6, %ymm6, %ymm6
	vmulps	%ymm7, %ymm6, %ymm6
	vcmpgt_oqps	%ymm11, %ymm4, %ymm8
	vblendvps	%ymm8, %ymm4, %ymm6, %ymm6
	vcmple_oqps	%ymm13, %ymm4, %ymm4
	vmulps	2528(%rsp), %ymm6, %ymm6
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm4, %ymm1, %ymm4
	vpandn	%ymm6, %ymm4, %ymm4
	vmovaps	%ymm9, 1952(%rsp)
.Ltmp7963:
	vaddps	384(%rsp), %ymm9, %ymm6
	vbroadcastss	.LCPI40_24(%rip), %ymm13
	vmulps	%ymm6, %ymm13, %ymm6
	vbroadcastss	.LCPI40_25(%rip), %ymm11
	vmaxps	%ymm11, %ymm6, %ymm6
	vbroadcastss	.LCPI40_26(%rip), %ymm15
	vminps	%ymm15, %ymm6, %ymm6
	vroundps	$9, %ymm6, %ymm8
	vsubps	%ymm8, %ymm6, %ymm6
	vbroadcastss	.LCPI40_27(%rip), %ymm12
	vmulps	%ymm6, %ymm12, %ymm9
	vbroadcastss	.LCPI40_28(%rip), %ymm0
	vaddps	%ymm0, %ymm9, %ymm9
	vmulps	%ymm6, %ymm9, %ymm9
	vbroadcastss	.LCPI40_29(%rip), %ymm7
	vaddps	%ymm7, %ymm9, %ymm9
	vmulps	%ymm6, %ymm9, %ymm9
	vbroadcastss	.LCPI40_30(%rip), %ymm2
	vaddps	%ymm2, %ymm9, %ymm9
	vmulps	%ymm6, %ymm9, %ymm9
	vbroadcastss	.LCPI40_31(%rip), %ymm3
	vaddps	%ymm3, %ymm9, %ymm9
	vbroadcastss	.LCPI40_23(%rip), %ymm10
.Ltmp7964:
	vmaxps	%ymm10, %ymm4, %ymm4
	vminps	%ymm1, %ymm4, %ymm4
	vmovaps	2112(%rsp), %ymm14
	vcmplt_oqps	%ymm14, %ymm4, %ymm10
	vmovaps	2464(%rsp), %ymm13
	vblendvps	%ymm10, 2496(%rsp), %ymm13, %ymm10
.Ltmp7965:
	vmulps	%ymm6, %ymm9, %ymm6
.Ltmp7966:
	vsubps	%ymm4, %ymm14, %ymm9
	vmulps	%ymm10, %ymm9, %ymm9
	vaddps	%ymm4, %ymm9, %ymm4
	vandps	%ymm5, %ymm4, %ymm9
	vbroadcastss	.LCPI40_2(%rip), %ymm1
	vcmplt_oqps	%ymm1, %ymm9, %ymm9
	vandnps	%ymm4, %ymm9, %ymm9
	vbroadcastss	.LCPI40_32(%rip), %ymm13
.Ltmp7967:
	vaddps	%ymm6, %ymm13, %ymm4
	vbroadcastss	.LCPI40_33(%rip), %ymm14
	vaddps	%ymm14, %ymm8, %ymm6
	vpslld	$23, %ymm6, %ymm6
	vmovaps	%ymm9, 2112(%rsp)
.Ltmp7968:
	vaddps	544(%rsp), %ymm9, %ymm8
.Ltmp7969:
	vmulps	%ymm6, %ymm4, %ymm4
	vmovaps	%ymm4, 1920(%rsp)
.Ltmp7970:
	vbroadcastss	.LCPI40_24(%rip), %ymm1
	vmulps	%ymm1, %ymm8, %ymm6
	vmaxps	%ymm11, %ymm6, %ymm6
	vminps	%ymm15, %ymm6, %ymm8
	vmovaps	%ymm15, %ymm14
	vroundps	$9, %ymm8, %ymm6
	vsubps	%ymm6, %ymm8, %ymm8
	vmulps	%ymm12, %ymm8, %ymm9
	vmovaps	%ymm12, %ymm13
	vaddps	%ymm0, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vaddps	%ymm7, %ymm9, %ymm9
	vmovaps	%ymm7, %ymm12
	vmulps	%ymm9, %ymm8, %ymm9
	vaddps	%ymm2, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm9
	vaddps	%ymm3, %ymm9, %ymm9
	vmulps	%ymm9, %ymm8, %ymm8
.Ltmp7971:
	vandps	1664(%rsp), %ymm5, %ymm2
	vbroadcastss	.LCPI40_4(%rip), %ymm4
.Ltmp7972:
	vmaxps	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_5(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm2
	vmovaps	1888(%rsp), %ymm11
	vandps	%ymm2, %ymm11, %ymm9
	vmovaps	1856(%rsp), %ymm7
	vorps	%ymm7, %ymm9, %ymm9
	vbroadcastss	.LCPI40_8(%rip), %ymm0
	vaddps	%ymm0, %ymm9, %ymm9
	vbroadcastss	.LCPI40_9(%rip), %ymm0
	vmulps	%ymm0, %ymm9, %ymm10
	vbroadcastss	.LCPI40_10(%rip), %ymm0
	vaddps	%ymm0, %ymm10, %ymm10
	vmulps	%ymm10, %ymm9, %ymm10
	vbroadcastss	.LCPI40_11(%rip), %ymm0
	vaddps	%ymm0, %ymm10, %ymm10
	vmulps	%ymm10, %ymm9, %ymm10
	vbroadcastss	.LCPI40_12(%rip), %ymm0
	vaddps	%ymm0, %ymm10, %ymm10
	vmulps	%ymm10, %ymm9, %ymm10
	vbroadcastss	.LCPI40_13(%rip), %ymm0
	vaddps	%ymm0, %ymm10, %ymm10
	vmulps	%ymm10, %ymm9, %ymm10
	vbroadcastss	.LCPI40_14(%rip), %ymm0
	vaddps	%ymm0, %ymm10, %ymm10
	vmulps	%ymm10, %ymm9, %ymm9
	vpsrld	$23, %ymm2, %ymm2
	vmovdqa	2048(%rsp), %ymm4
	vpor	%ymm4, %ymm2, %ymm2
	vbroadcastss	.LCPI40_16(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vaddps	%ymm2, %ymm9, %ymm2
	vbroadcastss	.LCPI40_32(%rip), %ymm0
.Ltmp7973:
	vaddps	%ymm0, %ymm8, %ymm8
	vbroadcastss	.LCPI40_33(%rip), %ymm1
	vaddps	%ymm1, %ymm6, %ymm6
	vpslld	$23, %ymm6, %ymm6
	vbroadcastss	.LCPI40_17(%rip), %ymm15
.Ltmp7974:
	vmulps	%ymm2, %ymm15, %ymm2
	vbroadcastss	.LCPI40_18(%rip), %ymm3
	vmaxps	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_19(%rip), %ymm3
	vminps	%ymm3, %ymm2, %ymm2
	vsubps	896(%rsp), %ymm2, %ymm2
.Ltmp7975:
	vmulps	%ymm6, %ymm8, %ymm3
	vmovaps	%ymm3, 1824(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm3
.Ltmp7976:
	vaddps	%ymm3, %ymm2, %ymm8
	vmulps	%ymm8, %ymm8, %ymm8
	vbroadcastss	.LCPI40_22(%rip), %ymm15
	vmulps	%ymm15, %ymm8, %ymm8
	vcmpgt_oqps	%ymm3, %ymm2, %ymm9
	vblendvps	%ymm9, %ymm2, %ymm8, %ymm8
	vbroadcastss	.LCPI40_21(%rip), %ymm6
	vcmple_oqps	%ymm6, %ymm2, %ymm2
	vmulps	2432(%rsp), %ymm8, %ymm8
	vxorps	%xmm6, %xmm6, %xmm6
	vpcmpgtd	%ymm2, %ymm6, %ymm2
	vpandn	%ymm8, %ymm2, %ymm2
	vbroadcastss	.LCPI40_23(%rip), %ymm8
	vmaxps	%ymm8, %ymm2, %ymm2
	vminps	%ymm6, %ymm2, %ymm2
	vmovaps	1984(%rsp), %ymm10
	vcmplt_oqps	%ymm10, %ymm2, %ymm8
	vmovaps	2720(%rsp), %ymm9
	vblendvps	%ymm8, 2400(%rsp), %ymm9, %ymm8
	vsubps	%ymm2, %ymm10, %ymm9
	vmulps	%ymm8, %ymm9, %ymm8
	vaddps	%ymm2, %ymm8, %ymm2
	vandps	%ymm5, %ymm2, %ymm8
	vbroadcastss	.LCPI40_2(%rip), %ymm6
	vcmplt_oqps	%ymm6, %ymm8, %ymm8
	vandnps	%ymm2, %ymm8, %ymm2
	vmovaps	%ymm2, 1984(%rsp)
	vaddps	1024(%rsp), %ymm2, %ymm2
	vbroadcastss	.LCPI40_24(%rip), %ymm6
	vmulps	%ymm6, %ymm2, %ymm2
	vbroadcastss	.LCPI40_25(%rip), %ymm10
	vmaxps	%ymm10, %ymm2, %ymm2
	vminps	%ymm14, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm8
	vsubps	%ymm8, %ymm2, %ymm2
	vmulps	%ymm2, %ymm13, %ymm9
	vbroadcastss	.LCPI40_28(%rip), %ymm6
	vaddps	%ymm6, %ymm9, %ymm9
	vmulps	%ymm2, %ymm9, %ymm9
	vaddps	%ymm12, %ymm9, %ymm9
	vmulps	%ymm2, %ymm9, %ymm9
	vbroadcastss	.LCPI40_30(%rip), %ymm6
	vaddps	%ymm6, %ymm9, %ymm9
	vmulps	%ymm2, %ymm9, %ymm9
	vbroadcastss	.LCPI40_31(%rip), %ymm6
	vaddps	%ymm6, %ymm9, %ymm9
	vmulps	%ymm2, %ymm9, %ymm2
	vaddps	%ymm0, %ymm2, %ymm2
	vmovaps	%ymm0, %ymm9
	vaddps	%ymm1, %ymm8, %ymm8
	vpslld	$23, %ymm8, %ymm8
	vmulps	%ymm2, %ymm8, %ymm8
.Ltmp7977:
	vandps	2080(%rsp), %ymm5, %ymm2
.Ltmp7978:
	vbroadcastss	.LCPI40_4(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_5(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm2
	vandps	%ymm2, %ymm11, %ymm0
	vorps	%ymm7, %ymm0, %ymm0
	vpsrld	$23, %ymm2, %ymm1
	vpor	%ymm4, %ymm1, %ymm1
	vbroadcastss	.LCPI40_8(%rip), %ymm2
	vaddps	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_9(%rip), %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_10(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_11(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_12(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_13(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_14(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
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
	vsubps	1056(%rsp), %ymm0, %ymm0
	vaddps	%ymm3, %ymm0, %ymm1
	vmulps	%ymm1, %ymm1, %ymm1
	vmulps	%ymm1, %ymm15, %ymm1
	vcmpgt_oqps	%ymm3, %ymm0, %ymm2
	vblendvps	%ymm2, %ymm0, %ymm1, %ymm1
	vbroadcastss	.LCPI40_21(%rip), %ymm2
	vcmple_oqps	%ymm2, %ymm0, %ymm0
	vmulps	2688(%rsp), %ymm1, %ymm1
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%ymm0, %ymm2, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vminps	%ymm2, %ymm0, %ymm0
	vmovaps	2176(%rsp), %ymm3
	vcmplt_oqps	%ymm3, %ymm0, %ymm1
	vmovaps	2624(%rsp), %ymm2
	vblendvps	%ymm1, 2656(%rsp), %ymm2, %ymm1
	vsubps	%ymm0, %ymm3, %ymm2
	vmulps	%ymm1, %ymm2, %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm1
	vbroadcastss	.LCPI40_2(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm3
	vaddps	1184(%rsp), %ymm3, %ymm0
	vbroadcastss	.LCPI40_24(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vmaxps	%ymm10, %ymm0, %ymm0
	vminps	%ymm14, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm1
	vsubps	%ymm1, %ymm0, %ymm0
	vmulps	%ymm0, %ymm13, %ymm2
	vbroadcastss	.LCPI40_28(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vaddps	%ymm2, %ymm12, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_30(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vaddps	%ymm6, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm0
	vaddps	%ymm0, %ymm9, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
	vpslld	$23, %ymm1, %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp7979:
	movq	2592(%r12), %rcx
	vmovaps	1920(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm1
	movq	2608(%r12), %rcx
	vmovaps	1824(%rsp), %ymm2
	vmulps	(%rcx,%rax,4), %ymm2, %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp7980:
	movq	5216(%r12), %rcx
	vmulps	(%rcx,%rax,4), %ymm8, %ymm2
	.loc	1 1155 27 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp7981:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
.Ltmp7982:
	.loc	14 48 14
	vaddps	%ymm0, %ymm2, %ymm0
	movq	136(%rsp), %rcx
.Ltmp7983:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rdi,4)
.Ltmp7984:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm0, (%rdx,%rdi,4)
.Ltmp7985:
	.loc	1 0 0
	incq	%r9
.Ltmp7986:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r9, %r13
.Ltmp7987:
	.loc	3 900 12
	jne	.LBB40_160
.Ltmp7988:
.LBB40_225:
	.loc	3 0 12 is_stmt 0
	vmovaps	1728(%rsp), %ymm0
	.loc	1 1160 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r12)
	vmovaps	1568(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r12)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r12)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r12)
	vmovaps	64(%rsp), %ymm0
	.loc	1 1161 5
	vmovaps	%ymm0, 4032(%r12)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r12)
	vmovaps	2272(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r12)
	vmovaps	1760(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r12)
	vmovaps	1952(%rsp), %ymm0
	.loc	1 1162 5
	vmovaps	%ymm0, 2304(%r12)
	vmovaps	2112(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r12)
	vmovaps	1984(%rsp), %ymm0
	.loc	1 1163 5
	vmovaps	%ymm0, 4928(%r12)
	vmovaps	%ymm3, 4960(%r12)
	.loc	1 1164 5
	movq	%r10, 5272(%r12)
	xorl	%eax, %eax
.Ltmp7989:
	.loc	1 0 5 is_stmt 0
.Ltmp7990:
	.p2align	4
.LBB40_226:
	.loc	1 1297 13 is_stmt 1
	vmovss	256(%rsp,%rax,2), %xmm3
	vmovss	260(%rsp,%rax,2), %xmm4
	vmovss	264(%rsp,%rax,2), %xmm5
	vmovss	268(%rsp,%rax,2), %xmm6
	vmovss	272(%rsp,%rax,2), %xmm7
	vmovss	276(%rsp,%rax,2), %xmm2
	vmovss	280(%rsp,%rax,2), %xmm1
	vmovd	284(%rsp,%rax,2), %xmm0
.Ltmp7991:
	.loc	1 1300 17
	vmovss	%xmm3, (%r12,%rax)
	.loc	1 1301 34
	movl	12(%r12,%rax), %ecx
	movl	172(%r12,%rax), %edx
.Ltmp7992:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp7993:
	.loc	1 1301 17
	movl	%ecx, 12(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm4, 160(%r12,%rax)
.Ltmp7994:
	.loc	38 2472 13
	subl	%r13d, %edx
	cmovbl	%r15d, %edx
.Ltmp7995:
	.loc	1 1301 17
	movl	%edx, 172(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm5, 320(%r12,%rax)
	.loc	1 1301 34
	movl	332(%r12,%rax), %ecx
.Ltmp7996:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp7997:
	.loc	1 1301 17
	movl	%ecx, 332(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm6, 480(%r12,%rax)
	.loc	1 1301 34
	movl	492(%r12,%rax), %ecx
.Ltmp7998:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp7999:
	.loc	1 1301 17
	movl	%ecx, 492(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm7, 640(%r12,%rax)
	.loc	1 1301 34
	movl	652(%r12,%rax), %ecx
.Ltmp8000:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8001:
	.loc	1 1301 17
	movl	%ecx, 652(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 800(%r12,%rax)
	.loc	1 1301 34
	movl	812(%r12,%rax), %ecx
.Ltmp8002:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8003:
	.loc	1 1301 17
	movl	%ecx, 812(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 960(%r12,%rax)
	.loc	1 1301 34
	movl	972(%r12,%rax), %ecx
.Ltmp8004:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8005:
	.loc	1 1301 17
	movl	%ecx, 972(%r12,%rax)
	.loc	1 1300 17
	vmovd	%xmm0, 1120(%r12,%rax)
	.loc	1 1301 34
	movl	1132(%r12,%rax), %ecx
.Ltmp8006:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8007:
	.loc	1 1301 17
	movl	%ecx, 1132(%r12,%rax)
.Ltmp8008:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp8009:
	.loc	3 900 12
	jne	.LBB40_226
.Ltmp8010:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	.p2align	4
.LBB40_228:
.Ltmp8011:
	.loc	1 1297 13 is_stmt 1
	vmovss	896(%rsp,%rax,2), %xmm3
	vmovss	900(%rsp,%rax,2), %xmm4
	vmovss	904(%rsp,%rax,2), %xmm5
	vmovss	908(%rsp,%rax,2), %xmm6
	vmovss	912(%rsp,%rax,2), %xmm7
	vmovss	916(%rsp,%rax,2), %xmm2
	vmovss	920(%rsp,%rax,2), %xmm1
	vmovd	924(%rsp,%rax,2), %xmm0
.Ltmp8012:
	.loc	1 1300 17
	vmovss	%xmm3, 2624(%r12,%rax)
	.loc	1 1301 34
	movl	2636(%r12,%rax), %ecx
	movl	2796(%r12,%rax), %edx
.Ltmp8013:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8014:
	.loc	1 1301 17
	movl	%ecx, 2636(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm4, 2784(%r12,%rax)
.Ltmp8015:
	.loc	38 2472 13
	subl	%r13d, %edx
	cmovbl	%r15d, %edx
.Ltmp8016:
	.loc	1 1301 17
	movl	%edx, 2796(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm5, 2944(%r12,%rax)
	.loc	1 1301 34
	movl	2956(%r12,%rax), %ecx
.Ltmp8017:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8018:
	.loc	1 1301 17
	movl	%ecx, 2956(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm6, 3104(%r12,%rax)
	.loc	1 1301 34
	movl	3116(%r12,%rax), %ecx
.Ltmp8019:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8020:
	.loc	1 1301 17
	movl	%ecx, 3116(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm7, 3264(%r12,%rax)
	.loc	1 1301 34
	movl	3276(%r12,%rax), %ecx
.Ltmp8021:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8022:
	.loc	1 1301 17
	movl	%ecx, 3276(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 3424(%r12,%rax)
	.loc	1 1301 34
	movl	3436(%r12,%rax), %ecx
.Ltmp8023:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8024:
	.loc	1 1301 17
	movl	%ecx, 3436(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 3584(%r12,%rax)
	.loc	1 1301 34
	movl	3596(%r12,%rax), %ecx
.Ltmp8025:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8026:
	.loc	1 1301 17
	movl	%ecx, 3596(%r12,%rax)
	.loc	1 1300 17
	vmovd	%xmm0, 3744(%r12,%rax)
	.loc	1 1301 34
	movl	3756(%r12,%rax), %ecx
.Ltmp8027:
	.loc	38 2472 13
	subl	%r13d, %ecx
	cmovbl	%r15d, %ecx
.Ltmp8028:
	.loc	1 1301 17
	movl	%ecx, 3756(%r12,%rax)
.Ltmp8029:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp8030:
	.loc	3 900 12
	jne	.LBB40_228
	jmp	.LBB40_153
.Ltmp8031:
.LBB40_229:
	.loc	38 1050 16
	cmpq	%rdi, %rsi
.Ltmp8032:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_655
.Ltmp8033:
	.loc	48 451 16 is_stmt 1
	cmpq	128(%rsp), %rsi
	ja	.LBB40_655
.Ltmp8034:
	.loc	48 451 16 is_stmt 0
	cmpq	120(%rsp), %rsi
	ja	.LBB40_661
.Ltmp8035:
	.loc	1 1053 27 is_stmt 1
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
.Ltmp8036:
	.loc	1 1054 26
	vmovaps	4032(%r12), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	4064(%r12), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	4128(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
.Ltmp8037:
	.loc	1 1055 25
	vmovaps	2304(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	2336(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp8038:
	.loc	1 1056 24
	vmovaps	4928(%r12), %ymm9
	vmovaps	4960(%r12), %ymm3
.Ltmp8039:
	.loc	1 1057 24
	movq	5272(%r12), %r10
.Ltmp8040:
	.loc	1 871 17
	movq	2368(%r12), %rax
	movq	2376(%r12), %rcx
.Ltmp8041:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2384(%r12), %rdx
	xorq	%rax, %rdx
	movq	2392(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2400(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	2408(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2416(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	orq	%rcx, %rdx
	xorq	2424(%r12), %rax
	orq	%rdx, %rax
	sete	176(%rsp)
.Ltmp8042:
	.loc	1 871 17
	movq	4992(%r12), %rax
	movq	5000(%r12), %rcx
.Ltmp8043:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	5008(%r12), %rdx
	xorq	%rax, %rdx
	movq	5016(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5024(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	5032(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5040(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	xorq	5048(%r12), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	184(%rsp)
.Ltmp8044:
	.loc	2 1916 50
	testq	%r13, %r13
.Ltmp8045:
	.loc	3 900 12
	je	.LBB40_152
.Ltmp8046:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r13,8), %rax
	movq	%rax, 168(%rsp)
	movq	240(%rsp), %rax
	leaq	(%rax,%rdi,4), %rcx
	movq	248(%rsp), %rax
	leaq	(%rax,%rdi,4), %rdx
.Ltmp8047:
	.loc	48 568 12 is_stmt 1
	movq	%r13, %rsi
	movabsq	$2305843009213693951, %rax
	andq	%rax, %rsi
	movq	%rsi, 152(%rsp)
	xorl	%edi, %edi
	xorl	%r9d, %r9d
	movq	%rcx, 136(%rsp)
	movq	%rdx, 48(%rsp)
.Ltmp8048:
	.loc	48 0 12 is_stmt 0
.Ltmp8049:
	.p2align	4
.LBB40_234:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp8050:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %r15d
	cmovaeq	%rbx, %r15
.Ltmp8051:
	.loc	48 568 12
	cmpq	168(%rsp), %rdi
	ja	.LBB40_639
.Ltmp8052:
	.loc	48 438 16
	cmpq	%r9, 152(%rsp)
	je	.LBB40_641
.Ltmp8053:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,8), %rax
.Ltmp8054:
	.loc	1 1083 29 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp8055:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_642
.Ltmp8056:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8057:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm3, 2208(%rsp)
	vmovaps	%ymm9, 2368(%rsp)
	vmovups	(%rcx,%rdi,4), %ymm1
.Ltmp8058:
	vmovups	(%rdx,%rdi,4), %ymm5
.Ltmp8059:
	vmovaps	1280(%r12), %ymm4
	vmovaps	1312(%r12), %ymm11
	vmovaps	1344(%r12), %ymm2
	vmovaps	3904(%r12), %ymm10
	vmovaps	3936(%r12), %ymm9
	vmovaps	3968(%r12), %ymm14
	vmovaps	1568(%rsp), %ymm6
	vsubps	%ymm6, %ymm1, %ymm0
	vmulps	%ymm0, %ymm11, %ymm3
	vmovaps	1728(%rsp), %ymm7
	vmovaps	%ymm4, 1920(%rsp)
	vmulps	%ymm4, %ymm7, %ymm4
	vaddps	%ymm3, %ymm4, %ymm13
	vaddps	%ymm7, %ymm13, %ymm3
	vmulps	%ymm7, %ymm11, %ymm4
	vmulps	%ymm2, %ymm0, %ymm0
	vaddps	%ymm0, %ymm4, %ymm12
	vaddps	%ymm6, %ymm12, %ymm4
	vmulps	1376(%r12), %ymm3, %ymm0
	vmovaps	1600(%rsp), %ymm6
	vsubps	%ymm6, %ymm4, %ymm7
	vmulps	1632(%rsp), %ymm11, %ymm3
	vmulps	%ymm7, %ymm2, %ymm2
	vaddps	%ymm2, %ymm3, %ymm2
	vaddps	%ymm2, %ymm6, %ymm6
.Ltmp8060:
	vsubps	192(%rsp), %ymm5, %ymm3
	vmulps	%ymm3, %ymm9, %ymm4
	vmovaps	64(%rsp), %ymm8
	vmovaps	%ymm10, 2176(%rsp)
	vmulps	%ymm10, %ymm8, %ymm15
	vaddps	%ymm4, %ymm15, %ymm4
	vaddps	%ymm4, %ymm8, %ymm15
	vmulps	4000(%r12), %ymm15, %ymm15
.Ltmp8061:
	.loc	1 1083 29 is_stmt 1
	movq	2592(%r12), %rcx
.Ltmp8062:
	.loc	8 551 14
	vmovups	%ymm6, (%rcx,%rax,4)
.Ltmp8063:
	.loc	1 1084 30
	movq	2616(%r12), %rsi
.Ltmp8064:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_643
.Ltmp8065:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8066:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm0, %ymm1, %ymm0
	vsubps	%ymm6, %ymm0, %ymm0
.Ltmp8067:
	.loc	1 1084 30 is_stmt 1
	movq	2608(%r12), %rcx
.Ltmp8068:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp8069:
	.loc	1 1085 28
	movq	5224(%r12), %rsi
.Ltmp8070:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_644
.Ltmp8071:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8072:
	.loc	1 0 0 is_stmt 0
	vmulps	64(%rsp), %ymm9, %ymm0
	vmulps	%ymm3, %ymm14, %ymm1
	vaddps	%ymm1, %ymm0, %ymm3
	vaddps	192(%rsp), %ymm3, %ymm0
	vmovaps	1760(%rsp), %ymm10
	vsubps	%ymm10, %ymm0, %ymm1
	vmovaps	2272(%rsp), %ymm8
	vmulps	%ymm9, %ymm8, %ymm0
	vmulps	%ymm1, %ymm14, %ymm6
	vaddps	%ymm6, %ymm0, %ymm14
	vaddps	%ymm14, %ymm10, %ymm0
.Ltmp8073:
	.loc	1 1085 28 is_stmt 1
	movq	5216(%r12), %rcx
.Ltmp8074:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp8075:
	.loc	1 1086 29
	movq	5240(%r12), %rsi
.Ltmp8076:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_645
.Ltmp8077:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8078:
	.loc	48 0 16 is_stmt 0
	movq	%r9, 2080(%rsp)
	movq	%rdi, 2112(%rsp)
	vaddps	%ymm5, %ymm15, %ymm5
	vsubps	%ymm0, %ymm5, %ymm0
.Ltmp8079:
	.loc	1 1086 29 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp8080:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp8081:
	.loc	1 1089 13
	movq	2592(%r12), %rdi
	movq	2600(%r12), %rsi
	movq	2368(%r12), %rax
.Ltmp8082:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp8083:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp8084:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 176(%rsp)
	je	.LBB40_250
.Ltmp8085:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB40_652
.Ltmp8086:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8087:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp8088:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
.Ltmp8089:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp8090:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp8091:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8092:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%rdi,%r9,4), %ymm10
.Ltmp8093:
	movq	2608(%r12), %rcx
.Ltmp8094:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm5
	vmovdqa	%ymm5, 1824(%rsp)
.Ltmp8095:
	.loc	1 961 2
	jmp	.LBB40_267
.Ltmp8096:
	.loc	1 0 2 is_stmt 0
.Ltmp8097:
	.p2align	4
.LBB40_250:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB40_665
	.loc	1 0 25 is_stmt 0
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8098:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8099:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_695
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8100:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8101:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_694
	.loc	1 0 25
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8102:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8103:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_688
	.loc	1 0 25
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8104:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8105:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_677
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8106:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8107:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 1824(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_683
	.loc	1 0 25
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8108:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8109:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 1664(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_679
	.loc	1 0 25
	movq	2424(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8110:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8111:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_687
.Ltmp8112:
	.loc	1 0 25
	movq	%rcx, 1888(%rsp)
.Ltmp8113:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
.Ltmp8114:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp8115:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp8116:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8117:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8118:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 1856(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_668
	.loc	1 0 25
	movq	%r13, 2048(%rsp)
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8119:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8120:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
	.loc	1 0 25
	movq	%r14, 16(%rsp)
	movq	%r11, 24(%rsp)
	movq	2392(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp8121:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp8122:
	.loc	1 955 30
	leaq	3(,%rdx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_684
	.loc	1 0 25
	movq	%r9, 32(%rsp)
	movq	2400(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp8123:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp8124:
	.loc	1 955 30
	leaq	4(,%rdx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_690
	.loc	1 0 25
	movq	%rcx, 40(%rsp)
	movq	2408(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp8125:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp8126:
	.loc	1 955 30
	leaq	5(,%rdx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_689
	.loc	1 0 25
	movq	%r8, %r9
	movq	2416(%r12), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp8127:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r11
	movq	%r10, %r8
	movq	%r12, %r10
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %r11
.Ltmp8128:
	.loc	1 955 30
	leaq	6(,%r11,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	%r15, %rcx
	movq	2424(%r10), %r11
	movq	%r8, %r15
	.loc	1 955 35
	addq	%r8, %r11
.Ltmp8129:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r11
	movl	$0, %r8d
	cmovaeq	%rbx, %r8
	subq	%r8, %r11
.Ltmp8130:
	.loc	1 955 30
	leaq	7(,%r11,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_682
.Ltmp8131:
	.loc	1 0 25
	movq	2048(%rsp), %rsi
	vmovd	(%rdi,%rsi,4), %xmm0
	movq	1824(%rsp), %rsi
	vpinsrd	$1, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	1664(%rsp), %rsi
	vpinsrd	$2, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	1888(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	32(%rsp), %rsi
	vmovd	(%rdi,%rsi,4), %xmm5
	vpinsrd	$1, (%rdi,%r9,4), %xmm5, %xmm5
	movq	24(%rsp), %rsi
	vpinsrd	$2, (%rdi,%rsi,4), %xmm5, %xmm5
	movq	16(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm5, %xmm5
.Ltmp8132:
	movq	2608(%r10), %rsi
.Ltmp8133:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm6
	vpinsrd	$1, (%rsi,%rdx,4), %xmm6, %xmm6
	vpinsrd	$2, (%rsi,%r12,4), %xmm6, %xmm6
	vpinsrd	$3, (%rsi,%r11,4), %xmm6, %xmm6
	vmovd	(%rsi,%rax,4), %xmm15
	movq	1856(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm15, %xmm15
	movq	40(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm15, %xmm15
	vpinsrd	$3, (%rsi,%r13,4), %xmm15, %xmm15
.Ltmp8134:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm5, %ymm10
.Ltmp8135:
	.loc	8 551 14
	vinserti128	$1, %xmm6, %ymm15, %ymm5
	vmovdqa	%ymm5, 1824(%rsp)
	movq	%r10, %r12
	movq	2144(%rsp), %r13
	movq	%r15, %r10
	movq	%rcx, %r15
	movq	2240(%rsp), %rbx
.Ltmp8136:
.LBB40_267:
	.loc	1 1103 13 is_stmt 1
	movq	5216(%r12), %r9
	movq	5224(%r12), %rsi
	movq	4992(%r12), %rax
.Ltmp8137:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp8138:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	movq	%rax, %rdi
	subq	%rcx, %rdi
.Ltmp8139:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rdi
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 184(%rsp)
	je	.LBB40_273
.Ltmp8140:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rdi, %r8
	jb	.LBB40_653
.Ltmp8141:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8142:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp8143:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
.Ltmp8144:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp8145:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp8146:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8147:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r9,%rdi,4), %ymm0
.Ltmp8148:
	movq	5232(%r12), %rcx
.Ltmp8149:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm5
.Ltmp8150:
	.loc	1 961 2
	jmp	.LBB40_290
.Ltmp8151:
	.loc	1 0 2 is_stmt 0
.Ltmp8152:
	.p2align	4
.LBB40_273:
	.loc	1 955 25 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB40_666
	.loc	1 0 25 is_stmt 0
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8153:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8154:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_697
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8155:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8156:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_699
	.loc	1 0 25
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8157:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8158:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_704
	.loc	1 0 25
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8159:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8160:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_730
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8161:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8162:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 1664(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_700
	.loc	1 0 25
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8163:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8164:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 1888(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_714
	.loc	1 0 25
	movq	5048(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8165:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8166:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_706
.Ltmp8167:
	.loc	1 0 25
	movq	%rcx, 1856(%rsp)
.Ltmp8168:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
.Ltmp8169:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp8170:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp8171:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8172:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8173:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 2048(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_725
	.loc	1 0 25
	movq	%r11, 24(%rsp)
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8174:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8175:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
	.loc	1 0 25
	movq	5016(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp8176:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp8177:
	.loc	1 955 30
	leaq	3(,%rdx,8), %r11
	movq	%r11, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_702
	.loc	1 0 25
	movq	%rcx, 56(%rsp)
	movq	%r13, 40(%rsp)
	movq	%r14, 32(%rsp)
	movq	5024(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp8178:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp8179:
	.loc	1 955 30
	leaq	4(,%rdx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_734
	.loc	1 0 25
	movq	5032(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp8180:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp8181:
	.loc	1 955 30
	leaq	5(,%rdx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_710
	.loc	1 0 25
	movq	%rdi, %r13
	movq	5040(%r12), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp8182:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r11
	movq	%r10, %rdi
	movq	%r12, %r10
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %r11
.Ltmp8183:
	.loc	1 955 30
	leaq	6(,%r11,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	%r9, %rcx
	movq	%r15, %r9
	movq	5048(%r10), %r11
	movq	%rdi, %r15
	.loc	1 955 35
	addq	%rdi, %r11
.Ltmp8184:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r11
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %r11
.Ltmp8185:
	.loc	1 955 30
	leaq	7(,%r11,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_746
.Ltmp8186:
	.loc	1 0 25
	movq	24(%rsp), %rsi
	vmovd	(%rcx,%rsi,4), %xmm0
	movq	1664(%rsp), %rsi
	vpinsrd	$1, (%rcx,%rsi,4), %xmm0, %xmm0
	movq	1888(%rsp), %rsi
	vpinsrd	$2, (%rcx,%rsi,4), %xmm0, %xmm0
	movq	1856(%rsp), %rsi
	vpinsrd	$3, (%rcx,%rsi,4), %xmm0, %xmm0
	vmovd	(%rcx,%r13,4), %xmm5
	vpinsrd	$1, (%rcx,%r8,4), %xmm5, %xmm5
	movq	32(%rsp), %rsi
	vpinsrd	$2, (%rcx,%rsi,4), %xmm5, %xmm5
	movq	40(%rsp), %rsi
	vpinsrd	$3, (%rcx,%rsi,4), %xmm5, %xmm5
.Ltmp8187:
	movq	5232(%r10), %rsi
.Ltmp8188:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm6
	vpinsrd	$1, (%rsi,%rdx,4), %xmm6, %xmm6
	vpinsrd	$2, (%rsi,%r12,4), %xmm6, %xmm6
	vpinsrd	$3, (%rsi,%r11,4), %xmm6, %xmm6
	vmovd	(%rsi,%rax,4), %xmm15
	movq	2048(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm15, %xmm15
	movq	56(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm15, %xmm15
	movq	16(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm15, %xmm15
.Ltmp8189:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm5, %ymm0
.Ltmp8190:
	.loc	8 551 14
	vinserti128	$1, %xmm6, %ymm15, %ymm5
	movq	%r10, %r12
	movq	2144(%rsp), %r13
	movq	%r15, %r10
	movq	%r9, %r15
	movq	2240(%rsp), %rbx
.Ltmp8191:
.LBB40_290:
	.loc	1 0 0
	negq	%r15
	addq	%r15, %r10
	incq	%r10
	leaq	(,%r10,8), %rax
.Ltmp8192:
	.loc	1 1150 36 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp8193:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movl	$0, %r15d
	movq	48(%rsp), %rdx
	movq	2112(%rsp), %rdi
	movq	2080(%rsp), %r9
	jb	.LBB40_647
.Ltmp8194:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8195:
	.loc	1 1152 27
	movq	2616(%r12), %rsi
.Ltmp8196:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_648
.Ltmp8197:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8198:
	.loc	1 1153 35
	movq	5224(%r12), %rsi
.Ltmp8199:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_649
.Ltmp8200:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8201:
	.loc	1 1155 27
	movq	5240(%r12), %rsi
.Ltmp8202:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_638
.Ltmp8203:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8204:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm13, %ymm13, %ymm6
	vaddps	1728(%rsp), %ymm6, %ymm6
	vbroadcastss	.LCPI40_1(%rip), %ymm13
	vandps	%ymm6, %ymm13, %ymm15
	vmovdqa	%ymm5, 1664(%rsp)
	vbroadcastss	.LCPI40_2(%rip), %ymm5
	vcmplt_oqps	%ymm5, %ymm15, %ymm15
	vandnps	%ymm6, %ymm15, %ymm6
	vmovaps	%ymm6, 1728(%rsp)
	vaddps	%ymm12, %ymm12, %ymm6
	vaddps	1568(%rsp), %ymm6, %ymm6
	vandps	%ymm6, %ymm13, %ymm12
	vcmplt_oqps	%ymm5, %ymm12, %ymm12
	vandnps	%ymm6, %ymm12, %ymm6
	vmovaps	%ymm6, 1568(%rsp)
	vmulps	%ymm7, %ymm11, %ymm6
	vmovaps	1632(%rsp), %ymm11
	vmulps	1920(%rsp), %ymm11, %ymm7
	vaddps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm6, %ymm6
	vaddps	%ymm6, %ymm11, %ymm6
	vandps	%ymm6, %ymm13, %ymm7
	vcmplt_oqps	%ymm5, %ymm7, %ymm7
	vandnps	%ymm6, %ymm7, %ymm6
	vmovaps	%ymm6, 1632(%rsp)
	vaddps	%ymm2, %ymm2, %ymm2
	vaddps	1600(%rsp), %ymm2, %ymm2
	vandps	%ymm2, %ymm13, %ymm6
	vcmplt_oqps	%ymm5, %ymm6, %ymm6
	vandnps	%ymm2, %ymm6, %ymm2
	vmovaps	%ymm2, 1600(%rsp)
.Ltmp8205:
	vaddps	%ymm4, %ymm4, %ymm2
	vaddps	64(%rsp), %ymm2, %ymm2
	vandps	%ymm2, %ymm13, %ymm4
	vcmplt_oqps	%ymm5, %ymm4, %ymm4
	vandnps	%ymm2, %ymm4, %ymm2
	vmovaps	%ymm2, 64(%rsp)
	vaddps	%ymm3, %ymm3, %ymm2
	vaddps	192(%rsp), %ymm2, %ymm2
	vandps	%ymm2, %ymm13, %ymm3
	vcmplt_oqps	%ymm5, %ymm3, %ymm3
	vmovaps	%ymm9, %ymm4
	vmovaps	%ymm5, %ymm9
	vandnps	%ymm2, %ymm3, %ymm2
	vmovaps	%ymm2, 192(%rsp)
	vmulps	%ymm1, %ymm4, %ymm1
	vmulps	2176(%rsp), %ymm8, %ymm2
	vaddps	%ymm1, %ymm2, %ymm1
	vaddps	%ymm1, %ymm1, %ymm1
	vaddps	%ymm1, %ymm8, %ymm1
	vandps	%ymm1, %ymm13, %ymm2
	vcmplt_oqps	%ymm5, %ymm2, %ymm2
	vandnps	%ymm1, %ymm2, %ymm1
	vmovaps	%ymm1, 2272(%rsp)
	vaddps	%ymm14, %ymm14, %ymm1
	vaddps	1760(%rsp), %ymm1, %ymm4
.Ltmp8206:
	vpand	%ymm13, %ymm10, %ymm1
	vbroadcastss	.LCPI40_4(%rip), %ymm10
.Ltmp8207:
	vmaxps	%ymm10, %ymm1, %ymm2
	vbroadcastsd	.LCPI40_6(%rip), %ymm1
	vbroadcastss	.LCPI40_5(%rip), %ymm11
	vmaxps	%ymm11, %ymm2, %ymm3
	vandps	%ymm1, %ymm3, %ymm6
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm1, 1888(%rsp)
	vbroadcastsd	.LCPI40_7(%rip), %ymm11
	vorps	%ymm6, %ymm11, %ymm6
	vmovaps	%ymm11, 1856(%rsp)
	vbroadcastss	.LCPI40_8(%rip), %ymm12
	vaddps	%ymm6, %ymm12, %ymm6
	vbroadcastss	.LCPI40_9(%rip), %ymm14
	vmulps	%ymm6, %ymm14, %ymm7
	vmovdqa	%ymm0, 1920(%rsp)
	vmovaps	%ymm14, %ymm0
	vbroadcastss	.LCPI40_10(%rip), %ymm15
	vaddps	%ymm7, %ymm15, %ymm7
	vmulps	%ymm7, %ymm6, %ymm7
	vbroadcastss	.LCPI40_11(%rip), %ymm8
	vaddps	%ymm7, %ymm8, %ymm7
	vmulps	%ymm7, %ymm6, %ymm7
	vbroadcastss	.LCPI40_12(%rip), %ymm8
	vaddps	%ymm7, %ymm8, %ymm7
	vmulps	%ymm7, %ymm6, %ymm7
	vbroadcastss	.LCPI40_13(%rip), %ymm8
	vaddps	%ymm7, %ymm8, %ymm7
	vmulps	%ymm7, %ymm6, %ymm7
	vbroadcastss	.LCPI40_14(%rip), %ymm8
	vaddps	%ymm7, %ymm8, %ymm7
	vmulps	%ymm7, %ymm6, %ymm6
	vpsrld	$23, %ymm3, %ymm7
	vpbroadcastd	.LCPI40_15(%rip), %ymm3
	vpor	%ymm3, %ymm7, %ymm7
	vmovdqa	%ymm3, 2048(%rsp)
	vbroadcastss	.LCPI40_16(%rip), %ymm12
	vaddps	%ymm7, %ymm12, %ymm7
	vaddps	%ymm6, %ymm7, %ymm6
	vbroadcastss	.LCPI40_17(%rip), %ymm14
	vmulps	%ymm6, %ymm14, %ymm6
	vbroadcastss	.LCPI40_18(%rip), %ymm15
	vmaxps	%ymm15, %ymm6, %ymm6
	vbroadcastss	.LCPI40_19(%rip), %ymm1
	vminps	%ymm1, %ymm6, %ymm6
	vsubps	256(%rsp), %ymm6, %ymm6
	vbroadcastss	.LCPI40_20(%rip), %ymm10
	vaddps	%ymm6, %ymm10, %ymm7
	vmulps	%ymm7, %ymm7, %ymm7
	vbroadcastss	.LCPI40_22(%rip), %ymm5
	vmulps	%ymm5, %ymm7, %ymm7
	vcmpgt_oqps	%ymm10, %ymm6, %ymm8
	vblendvps	%ymm8, %ymm6, %ymm7, %ymm7
.Ltmp8208:
	vandps	%ymm4, %ymm13, %ymm8
	vcmplt_oqps	%ymm9, %ymm8, %ymm8
	vandnps	%ymm4, %ymm8, %ymm4
	vmovaps	%ymm4, 1760(%rsp)
	vbroadcastss	.LCPI40_21(%rip), %ymm12
.Ltmp8209:
	vcmple_oqps	%ymm12, %ymm6, %ymm4
	vmulps	2336(%rsp), %ymm7, %ymm6
	vxorps	%xmm7, %xmm7, %xmm7
	vpcmpgtd	%ymm4, %ymm7, %ymm4
	vpandn	%ymm6, %ymm4, %ymm4
	vbroadcastss	.LCPI40_23(%rip), %ymm14
	vmaxps	%ymm14, %ymm4, %ymm4
	vminps	%ymm7, %ymm4, %ymm4
	vxorps	%xmm10, %xmm10, %xmm10
	vmovaps	1984(%rsp), %ymm8
	vcmplt_oqps	%ymm8, %ymm4, %ymm6
	vmovaps	2560(%rsp), %ymm7
	vblendvps	%ymm6, 2592(%rsp), %ymm7, %ymm6
	vsubps	%ymm4, %ymm8, %ymm7
	vmulps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm4, %ymm4
	vandps	%ymm4, %ymm13, %ymm6
	vcmplt_oqps	%ymm9, %ymm6, %ymm6
	vandnps	%ymm4, %ymm6, %ymm8
.Ltmp8210:
	vandps	1824(%rsp), %ymm13, %ymm4
.Ltmp8211:
	vbroadcastss	.LCPI40_4(%rip), %ymm6
	vmaxps	%ymm6, %ymm4, %ymm4
	vbroadcastss	.LCPI40_5(%rip), %ymm6
	vmaxps	%ymm6, %ymm4, %ymm4
	vandps	%ymm2, %ymm4, %ymm6
	vorps	%ymm6, %ymm11, %ymm6
	vbroadcastss	.LCPI40_8(%rip), %ymm2
	vaddps	%ymm2, %ymm6, %ymm6
	vmulps	%ymm0, %ymm6, %ymm7
	vbroadcastss	.LCPI40_10(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm6, %ymm7
	vbroadcastss	.LCPI40_11(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm6, %ymm7
	vbroadcastss	.LCPI40_12(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm6, %ymm7
	vbroadcastss	.LCPI40_13(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm6, %ymm7
	vbroadcastss	.LCPI40_14(%rip), %ymm0
	vaddps	%ymm0, %ymm7, %ymm7
	vmulps	%ymm7, %ymm6, %ymm6
	vpsrld	$23, %ymm4, %ymm4
	vpor	%ymm3, %ymm4, %ymm4
	vbroadcastss	.LCPI40_16(%rip), %ymm0
	vaddps	%ymm0, %ymm4, %ymm4
	vaddps	%ymm6, %ymm4, %ymm4
	vbroadcastss	.LCPI40_17(%rip), %ymm0
	vmulps	%ymm0, %ymm4, %ymm4
	vmaxps	%ymm15, %ymm4, %ymm4
	vminps	%ymm1, %ymm4, %ymm4
	vsubps	416(%rsp), %ymm4, %ymm4
	vbroadcastss	.LCPI40_20(%rip), %ymm0
	vaddps	%ymm0, %ymm4, %ymm6
	vmulps	%ymm6, %ymm6, %ymm6
	vmulps	%ymm5, %ymm6, %ymm6
	vcmpgt_oqps	%ymm0, %ymm4, %ymm7
	vblendvps	%ymm7, %ymm4, %ymm6, %ymm6
	vcmple_oqps	%ymm12, %ymm4, %ymm4
	vmulps	2528(%rsp), %ymm6, %ymm6
	vpcmpgtd	%ymm4, %ymm10, %ymm4
	vpandn	%ymm6, %ymm4, %ymm4
	vmovaps	%ymm8, 1984(%rsp)
.Ltmp8212:
	vaddps	384(%rsp), %ymm8, %ymm6
	vbroadcastss	.LCPI40_24(%rip), %ymm11
	vmulps	%ymm6, %ymm11, %ymm6
	vbroadcastss	.LCPI40_25(%rip), %ymm10
	vmaxps	%ymm10, %ymm6, %ymm6
	vbroadcastss	.LCPI40_26(%rip), %ymm15
	vminps	%ymm15, %ymm6, %ymm6
	vroundps	$9, %ymm6, %ymm7
	vsubps	%ymm7, %ymm6, %ymm6
	vbroadcastss	.LCPI40_27(%rip), %ymm5
	vmulps	%ymm5, %ymm6, %ymm8
	vbroadcastss	.LCPI40_28(%rip), %ymm0
	vaddps	%ymm0, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_29(%rip), %ymm1
	vaddps	%ymm1, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_30(%rip), %ymm2
	vaddps	%ymm2, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vbroadcastss	.LCPI40_31(%rip), %ymm3
	vaddps	%ymm3, %ymm8, %ymm8
	vbroadcastss	.LCPI40_23(%rip), %ymm9
.Ltmp8213:
	vmaxps	%ymm9, %ymm4, %ymm4
	vxorps	%xmm9, %xmm9, %xmm9
	vminps	%ymm9, %ymm4, %ymm4
	vmovaps	1952(%rsp), %ymm14
	vcmplt_oqps	%ymm14, %ymm4, %ymm9
	vmovaps	2464(%rsp), %ymm12
	vblendvps	%ymm9, 2496(%rsp), %ymm12, %ymm9
.Ltmp8214:
	vmulps	%ymm6, %ymm8, %ymm6
.Ltmp8215:
	vsubps	%ymm4, %ymm14, %ymm8
	vmulps	%ymm9, %ymm8, %ymm8
	vaddps	%ymm4, %ymm8, %ymm4
	vandps	%ymm4, %ymm13, %ymm8
	vbroadcastss	.LCPI40_2(%rip), %ymm9
	vcmplt_oqps	%ymm9, %ymm8, %ymm8
	vandnps	%ymm4, %ymm8, %ymm8
	vbroadcastss	.LCPI40_32(%rip), %ymm12
.Ltmp8216:
	vaddps	%ymm6, %ymm12, %ymm4
	vbroadcastss	.LCPI40_33(%rip), %ymm14
	vaddps	%ymm7, %ymm14, %ymm6
	vpslld	$23, %ymm6, %ymm6
	vmovaps	%ymm8, 1952(%rsp)
.Ltmp8217:
	vaddps	544(%rsp), %ymm8, %ymm7
.Ltmp8218:
	vmulps	%ymm6, %ymm4, %ymm4
	vmovaps	%ymm4, 2176(%rsp)
.Ltmp8219:
	vmulps	%ymm7, %ymm11, %ymm6
	vmaxps	%ymm10, %ymm6, %ymm6
	vminps	%ymm15, %ymm6, %ymm6
	vroundps	$9, %ymm6, %ymm7
	vsubps	%ymm7, %ymm6, %ymm6
	vmulps	%ymm5, %ymm6, %ymm8
	vmovaps	%ymm5, %ymm12
	vaddps	%ymm0, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vaddps	%ymm1, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vaddps	%ymm2, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm8
	vaddps	%ymm3, %ymm8, %ymm8
	vmulps	%ymm6, %ymm8, %ymm6
.Ltmp8220:
	vandps	1920(%rsp), %ymm13, %ymm0
	vbroadcastss	.LCPI40_4(%rip), %ymm4
.Ltmp8221:
	vmaxps	%ymm4, %ymm0, %ymm0
	vbroadcastss	.LCPI40_5(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vmovaps	1888(%rsp), %ymm5
	vandps	%ymm5, %ymm0, %ymm8
	vmovaps	1856(%rsp), %ymm4
	vorps	%ymm4, %ymm8, %ymm8
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
	vpsrld	$23, %ymm0, %ymm0
	vmovdqa	2048(%rsp), %ymm2
	vpor	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vaddps	%ymm0, %ymm8, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm1
.Ltmp8222:
	vaddps	%ymm1, %ymm6, %ymm6
	vaddps	%ymm7, %ymm14, %ymm7
	vpslld	$23, %ymm7, %ymm7
	vbroadcastss	.LCPI40_17(%rip), %ymm15
.Ltmp8223:
	vmulps	%ymm0, %ymm15, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm3
	vmaxps	%ymm3, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm3
	vminps	%ymm3, %ymm0, %ymm0
	vsubps	896(%rsp), %ymm0, %ymm0
.Ltmp8224:
	vmulps	%ymm7, %ymm6, %ymm3
	vmovaps	%ymm3, 1920(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm3
.Ltmp8225:
	vaddps	%ymm3, %ymm0, %ymm6
	vmulps	%ymm6, %ymm6, %ymm6
	vbroadcastss	.LCPI40_22(%rip), %ymm15
	vmulps	%ymm6, %ymm15, %ymm6
	vcmpgt_oqps	%ymm3, %ymm0, %ymm8
	vblendvps	%ymm8, %ymm0, %ymm6, %ymm6
	vbroadcastss	.LCPI40_21(%rip), %ymm7
	vcmple_oqps	%ymm7, %ymm0, %ymm0
	vmulps	2432(%rsp), %ymm6, %ymm6
	vxorps	%xmm7, %xmm7, %xmm7
	vpcmpgtd	%ymm0, %ymm7, %ymm0
	vpandn	%ymm6, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm6
	vmaxps	%ymm6, %ymm0, %ymm0
	vminps	%ymm7, %ymm0, %ymm0
	vmovaps	2368(%rsp), %ymm9
	vcmplt_oqps	%ymm9, %ymm0, %ymm6
	vmovaps	2720(%rsp), %ymm8
	vblendvps	%ymm6, 2400(%rsp), %ymm8, %ymm6
	vsubps	%ymm0, %ymm9, %ymm8
	vmulps	%ymm6, %ymm8, %ymm6
	vaddps	%ymm6, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm6
	vbroadcastss	.LCPI40_2(%rip), %ymm7
	vcmplt_oqps	%ymm7, %ymm6, %ymm6
	vandnps	%ymm0, %ymm6, %ymm9
	vaddps	1024(%rsp), %ymm9, %ymm0
	vmulps	%ymm0, %ymm11, %ymm0
	vmaxps	%ymm10, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm6
	vminps	%ymm6, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm6
	vsubps	%ymm6, %ymm0, %ymm0
	vmulps	%ymm0, %ymm12, %ymm8
	vbroadcastss	.LCPI40_28(%rip), %ymm7
	vaddps	%ymm7, %ymm8, %ymm8
	vmulps	%ymm0, %ymm8, %ymm8
	vbroadcastss	.LCPI40_29(%rip), %ymm10
	vaddps	%ymm10, %ymm8, %ymm8
	vmulps	%ymm0, %ymm8, %ymm8
	vbroadcastss	.LCPI40_30(%rip), %ymm7
	vaddps	%ymm7, %ymm8, %ymm8
	vmulps	%ymm0, %ymm8, %ymm8
	vbroadcastss	.LCPI40_31(%rip), %ymm7
	vaddps	%ymm7, %ymm8, %ymm8
	vmulps	%ymm0, %ymm8, %ymm0
	vaddps	%ymm1, %ymm0, %ymm0
	vaddps	%ymm6, %ymm14, %ymm6
	vpslld	$23, %ymm6, %ymm6
	vmulps	%ymm6, %ymm0, %ymm8
.Ltmp8226:
	vandps	1664(%rsp), %ymm13, %ymm0
.Ltmp8227:
	vbroadcastss	.LCPI40_4(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_5(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vandps	%ymm5, %ymm0, %ymm1
	vorps	%ymm4, %ymm1, %ymm1
	vpsrld	$23, %ymm0, %ymm0
	vpor	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_8(%rip), %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
	vbroadcastss	.LCPI40_9(%rip), %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_10(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_11(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_12(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_13(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_14(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
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
	vsubps	1056(%rsp), %ymm0, %ymm0
	vaddps	%ymm3, %ymm0, %ymm1
	vmulps	%ymm1, %ymm1, %ymm1
	vmulps	%ymm1, %ymm15, %ymm1
	vcmpgt_oqps	%ymm3, %ymm0, %ymm2
	vblendvps	%ymm2, %ymm0, %ymm1, %ymm1
	vbroadcastss	.LCPI40_21(%rip), %ymm2
	vcmple_oqps	%ymm2, %ymm0, %ymm0
	vmulps	2688(%rsp), %ymm1, %ymm1
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%ymm0, %ymm2, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vminps	%ymm2, %ymm0, %ymm0
	vmovaps	2208(%rsp), %ymm3
	vcmplt_oqps	%ymm3, %ymm0, %ymm1
	vmovaps	2624(%rsp), %ymm2
	vblendvps	%ymm1, 2656(%rsp), %ymm2, %ymm1
	vsubps	%ymm0, %ymm3, %ymm2
	vmulps	%ymm1, %ymm2, %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
	vandps	%ymm0, %ymm13, %ymm1
	vbroadcastss	.LCPI40_2(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm3
	vaddps	1184(%rsp), %ymm3, %ymm0
	vmulps	%ymm0, %ymm11, %ymm0
	vbroadcastss	.LCPI40_25(%rip), %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
	vbroadcastss	.LCPI40_26(%rip), %ymm1
	vminps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm1
	vsubps	%ymm1, %ymm0, %ymm0
	vmulps	%ymm0, %ymm12, %ymm2
	vbroadcastss	.LCPI40_28(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vaddps	%ymm2, %ymm10, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vbroadcastss	.LCPI40_30(%rip), %ymm4
	vaddps	%ymm4, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm2
	vaddps	%ymm7, %ymm2, %ymm2
	vmulps	%ymm2, %ymm0, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm2
	vaddps	%ymm2, %ymm0, %ymm0
	vaddps	%ymm1, %ymm14, %ymm1
	vpslld	$23, %ymm1, %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp8228:
	movq	2592(%r12), %rcx
	vmovaps	2176(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm1
	movq	2608(%r12), %rcx
	vmovaps	1920(%rsp), %ymm2
	vmulps	(%rcx,%rax,4), %ymm2, %ymm2
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp8229:
	movq	5216(%r12), %rcx
	vmulps	(%rcx,%rax,4), %ymm8, %ymm2
	.loc	1 1155 27 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp8230:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
.Ltmp8231:
	.loc	14 48 14
	vaddps	%ymm0, %ymm2, %ymm0
	movq	136(%rsp), %rcx
.Ltmp8232:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rdi,4)
.Ltmp8233:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm0, (%rdx,%rdi,4)
.Ltmp8234:
	.loc	1 0 0
	incq	%r9
.Ltmp8235:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r9, %r13
.Ltmp8236:
	.loc	3 900 12
	jne	.LBB40_234
	jmp	.LBB40_152
.Ltmp8237:
.LBB40_299:
	.loc	3 0 12 is_stmt 0
	vmovaps	1600(%rsp), %ymm0
.Ltmp8238:
	.loc	1 1160 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r12)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r12)
	vmovaps	1568(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r12)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r12)
	vmovaps	2272(%rsp), %ymm0
	.loc	1 1161 5
	vmovaps	%ymm0, 4032(%r12)
	vmovaps	1984(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r12)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r12)
	vmovaps	1760(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r12)
	vmovaps	2240(%rsp), %ymm0
	.loc	1 1162 5
	vmovaps	%ymm0, 2304(%r12)
	vmovaps	2112(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r12)
	vmovdqa	1952(%rsp), %ymm0
	.loc	1 1163 5
	vmovdqa	%ymm0, 4928(%r12)
	vmovaps	%ymm8, 4960(%r12)
	movq	64(%rsp), %rax
	.loc	1 1164 5
	movq	%rax, 5272(%r12)
.Ltmp8239:
.LBB40_300:
	.loc	1 0 5 is_stmt 0
	movq	144(%rsp), %rsi
	movq	2040(%rsp), %r13
	.loc	1 1189 11 is_stmt 1
	cmpq	%rsi, %r13
	jae	.LBB40_595
.LBB40_301:
	.loc	1 1190 42
	subq	%r13, %rsi
	.loc	1 1190 29 is_stmt 0
	movq	%r12, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E12plan_segmentB5_
	movb	%dl, 64(%rsp)
	movq	%rax, %r15
.Ltmp8240:
	.loc	1 1279 33 is_stmt 1
	vmovss	(%r12), %xmm0
.Ltmp8241:
	.loc	1 1192 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp8242:
	.loc	1 1279 33
	vmovss	160(%r12), %xmm0
.Ltmp8243:
	.loc	1 1192 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp8244:
	.loc	1 1279 33
	vmovss	320(%r12), %xmm0
.Ltmp8245:
	.loc	1 1192 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp8246:
	.loc	1 1279 33
	vmovss	480(%r12), %xmm0
.Ltmp8247:
	.loc	1 1192 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp8248:
	.loc	1 1279 33
	vmovss	640(%r12), %xmm0
.Ltmp8249:
	.loc	1 1192 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp8250:
	.loc	1 1279 33
	vmovss	800(%r12), %xmm0
.Ltmp8251:
	.loc	1 1192 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp8252:
	.loc	1 1279 33
	vmovss	960(%r12), %xmm0
.Ltmp8253:
	.loc	1 1192 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp8254:
	.loc	1 1279 33
	vmovss	1120(%r12), %xmm0
.Ltmp8255:
	.loc	1 1192 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp8256:
	.loc	1 1279 33
	vmovss	16(%r12), %xmm0
.Ltmp8257:
	.loc	1 1192 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp8258:
	.loc	1 1279 33
	vmovss	176(%r12), %xmm0
.Ltmp8259:
	.loc	1 1192 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp8260:
	.loc	1 1279 33
	vmovss	336(%r12), %xmm0
.Ltmp8261:
	.loc	1 1192 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp8262:
	.loc	1 1279 33
	vmovss	496(%r12), %xmm0
.Ltmp8263:
	.loc	1 1192 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp8264:
	.loc	1 1279 33
	vmovss	656(%r12), %xmm0
.Ltmp8265:
	.loc	1 1192 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp8266:
	.loc	1 1279 33
	vmovss	816(%r12), %xmm0
.Ltmp8267:
	.loc	1 1192 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp8268:
	.loc	1 1279 33
	vmovss	976(%r12), %xmm0
.Ltmp8269:
	.loc	1 1192 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp8270:
	.loc	1 1279 33
	vmovss	1136(%r12), %xmm0
.Ltmp8271:
	.loc	1 1192 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp8272:
	.loc	1 1279 33
	vmovss	32(%r12), %xmm0
.Ltmp8273:
	.loc	1 1192 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp8274:
	.loc	1 1279 33
	vmovss	192(%r12), %xmm0
.Ltmp8275:
	.loc	1 1192 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp8276:
	.loc	1 1279 33
	vmovss	352(%r12), %xmm0
.Ltmp8277:
	.loc	1 1192 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp8278:
	.loc	1 1279 33
	vmovss	512(%r12), %xmm0
.Ltmp8279:
	.loc	1 1192 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp8280:
	.loc	1 1279 33
	vmovss	672(%r12), %xmm0
.Ltmp8281:
	.loc	1 1192 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp8282:
	.loc	1 1279 33
	vmovss	832(%r12), %xmm0
.Ltmp8283:
	.loc	1 1192 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp8284:
	.loc	1 1279 33
	vmovss	992(%r12), %xmm0
.Ltmp8285:
	.loc	1 1192 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp8286:
	.loc	1 1279 33
	vmovss	1152(%r12), %xmm0
.Ltmp8287:
	.loc	1 1192 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp8288:
	.loc	1 1279 33
	vmovss	48(%r12), %xmm0
.Ltmp8289:
	.loc	1 1192 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp8290:
	.loc	1 1279 33
	vmovss	208(%r12), %xmm0
.Ltmp8291:
	.loc	1 1192 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp8292:
	.loc	1 1279 33
	vmovss	368(%r12), %xmm0
.Ltmp8293:
	.loc	1 1192 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp8294:
	.loc	1 1279 33
	vmovss	528(%r12), %xmm0
.Ltmp8295:
	.loc	1 1192 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp8296:
	.loc	1 1279 33
	vmovss	688(%r12), %xmm0
.Ltmp8297:
	.loc	1 1192 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp8298:
	.loc	1 1279 33
	vmovss	848(%r12), %xmm0
.Ltmp8299:
	.loc	1 1192 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp8300:
	.loc	1 1279 33
	vmovss	1008(%r12), %xmm0
.Ltmp8301:
	.loc	1 1192 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp8302:
	.loc	1 1279 33
	vmovss	1168(%r12), %xmm0
.Ltmp8303:
	.loc	1 1192 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp8304:
	.loc	1 1279 33
	vmovss	64(%r12), %xmm0
.Ltmp8305:
	.loc	1 1192 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp8306:
	.loc	1 1279 33
	vmovss	224(%r12), %xmm0
.Ltmp8307:
	.loc	1 1192 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp8308:
	.loc	1 1279 33
	vmovss	384(%r12), %xmm0
.Ltmp8309:
	.loc	1 1192 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp8310:
	.loc	1 1279 33
	vmovss	544(%r12), %xmm0
.Ltmp8311:
	.loc	1 1192 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp8312:
	.loc	1 1279 33
	vmovss	704(%r12), %xmm0
.Ltmp8313:
	.loc	1 1192 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp8314:
	.loc	1 1279 33
	vmovss	864(%r12), %xmm0
.Ltmp8315:
	.loc	1 1192 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp8316:
	.loc	1 1279 33
	vmovss	1024(%r12), %xmm0
.Ltmp8317:
	.loc	1 1192 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp8318:
	.loc	1 1279 33
	vmovss	1184(%r12), %xmm0
.Ltmp8319:
	.loc	1 1192 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp8320:
	.loc	1 1279 33
	vmovss	80(%r12), %xmm0
.Ltmp8321:
	.loc	1 1192 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp8322:
	.loc	1 1279 33
	vmovss	240(%r12), %xmm0
.Ltmp8323:
	.loc	1 1192 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp8324:
	.loc	1 1279 33
	vmovss	400(%r12), %xmm0
.Ltmp8325:
	.loc	1 1192 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp8326:
	.loc	1 1279 33
	vmovss	560(%r12), %xmm0
.Ltmp8327:
	.loc	1 1192 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp8328:
	.loc	1 1279 33
	vmovss	720(%r12), %xmm0
.Ltmp8329:
	.loc	1 1192 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp8330:
	.loc	1 1279 33
	vmovss	880(%r12), %xmm0
.Ltmp8331:
	.loc	1 1192 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp8332:
	.loc	1 1279 33
	vmovss	1040(%r12), %xmm0
.Ltmp8333:
	.loc	1 1192 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp8334:
	.loc	1 1279 33
	vmovss	1200(%r12), %xmm0
.Ltmp8335:
	.loc	1 1192 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp8336:
	.loc	1 1279 33
	vmovss	96(%r12), %xmm0
.Ltmp8337:
	.loc	1 1192 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp8338:
	.loc	1 1279 33
	vmovss	256(%r12), %xmm0
.Ltmp8339:
	.loc	1 1192 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp8340:
	.loc	1 1279 33
	vmovss	416(%r12), %xmm0
.Ltmp8341:
	.loc	1 1192 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp8342:
	.loc	1 1279 33
	vmovss	576(%r12), %xmm0
.Ltmp8343:
	.loc	1 1192 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp8344:
	.loc	1 1279 33
	vmovss	736(%r12), %xmm0
.Ltmp8345:
	.loc	1 1192 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp8346:
	.loc	1 1279 33
	vmovss	896(%r12), %xmm0
.Ltmp8347:
	.loc	1 1192 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp8348:
	.loc	1 1279 33
	vmovss	1056(%r12), %xmm0
.Ltmp8349:
	.loc	1 1192 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp8350:
	.loc	1 1279 33
	vmovss	1216(%r12), %xmm0
.Ltmp8351:
	.loc	1 1192 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp8352:
	.loc	1 1279 33
	vmovss	112(%r12), %xmm0
.Ltmp8353:
	.loc	1 1192 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp8354:
	.loc	1 1279 33
	vmovss	272(%r12), %xmm0
.Ltmp8355:
	.loc	1 1192 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp8356:
	.loc	1 1279 33
	vmovss	432(%r12), %xmm0
.Ltmp8357:
	.loc	1 1192 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp8358:
	.loc	1 1279 33
	vmovss	592(%r12), %xmm0
.Ltmp8359:
	.loc	1 1192 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp8360:
	.loc	1 1279 33
	vmovss	752(%r12), %xmm0
.Ltmp8361:
	.loc	1 1192 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp8362:
	.loc	1 1279 33
	vmovss	912(%r12), %xmm0
.Ltmp8363:
	.loc	1 1192 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp8364:
	.loc	1 1279 33
	vmovss	1072(%r12), %xmm0
.Ltmp8365:
	.loc	1 1192 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp8366:
	.loc	1 1279 33
	vmovss	1232(%r12), %xmm0
.Ltmp8367:
	.loc	1 1192 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp8368:
	.loc	1 1279 33
	vmovss	128(%r12), %xmm0
.Ltmp8369:
	.loc	1 1192 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp8370:
	.loc	1 1279 33
	vmovss	288(%r12), %xmm0
.Ltmp8371:
	.loc	1 1192 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp8372:
	.loc	1 1279 33
	vmovss	448(%r12), %xmm0
.Ltmp8373:
	.loc	1 1192 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp8374:
	.loc	1 1279 33
	vmovss	608(%r12), %xmm0
.Ltmp8375:
	.loc	1 1192 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp8376:
	.loc	1 1279 33
	vmovss	768(%r12), %xmm0
.Ltmp8377:
	.loc	1 1192 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp8378:
	.loc	1 1279 33
	vmovss	928(%r12), %xmm0
.Ltmp8379:
	.loc	1 1192 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp8380:
	.loc	1 1279 33
	vmovss	1088(%r12), %xmm0
.Ltmp8381:
	.loc	1 1192 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp8382:
	.loc	1 1279 33
	vmovss	1248(%r12), %xmm0
.Ltmp8383:
	.loc	1 1192 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp8384:
	.loc	1 1279 33
	vmovss	144(%r12), %xmm0
.Ltmp8385:
	.loc	1 1192 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp8386:
	.loc	1 1279 33
	vmovss	304(%r12), %xmm0
.Ltmp8387:
	.loc	1 1192 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp8388:
	.loc	1 1279 33
	vmovss	464(%r12), %xmm0
.Ltmp8389:
	.loc	1 1192 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp8390:
	.loc	1 1279 33
	vmovss	624(%r12), %xmm0
.Ltmp8391:
	.loc	1 1192 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp8392:
	.loc	1 1279 33
	vmovss	784(%r12), %xmm0
.Ltmp8393:
	.loc	1 1192 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp8394:
	.loc	1 1279 33
	vmovss	944(%r12), %xmm0
.Ltmp8395:
	.loc	1 1192 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp8396:
	.loc	1 1279 33
	vmovss	1104(%r12), %xmm0
.Ltmp8397:
	.loc	1 1192 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp8398:
	.loc	1 1279 33
	vmovss	1264(%r12), %xmm0
.Ltmp8399:
	.loc	1 1192 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp8400:
	.loc	1 1280 32
	vmovss	8(%r12), %xmm0
.Ltmp8401:
	.loc	1 1192 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp8402:
	.loc	1 1280 32
	vmovss	168(%r12), %xmm0
.Ltmp8403:
	.loc	1 1192 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp8404:
	.loc	1 1280 32
	vmovss	328(%r12), %xmm0
.Ltmp8405:
	.loc	1 1192 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp8406:
	.loc	1 1280 32
	vmovss	488(%r12), %xmm0
.Ltmp8407:
	.loc	1 1192 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp8408:
	.loc	1 1280 32
	vmovss	648(%r12), %xmm0
.Ltmp8409:
	.loc	1 1192 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp8410:
	.loc	1 1280 32
	vmovss	808(%r12), %xmm0
.Ltmp8411:
	.loc	1 1192 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp8412:
	.loc	1 1280 32
	vmovss	968(%r12), %xmm0
.Ltmp8413:
	.loc	1 1192 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp8414:
	.loc	1 1280 32
	vmovss	1128(%r12), %xmm0
.Ltmp8415:
	.loc	1 1192 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp8416:
	.loc	1 1280 32
	vmovss	24(%r12), %xmm0
.Ltmp8417:
	.loc	1 1192 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp8418:
	.loc	1 1280 32
	vmovss	184(%r12), %xmm0
.Ltmp8419:
	.loc	1 1192 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp8420:
	.loc	1 1280 32
	vmovss	344(%r12), %xmm0
.Ltmp8421:
	.loc	1 1192 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp8422:
	.loc	1 1280 32
	vmovss	504(%r12), %xmm0
.Ltmp8423:
	.loc	1 1192 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp8424:
	.loc	1 1280 32
	vmovss	664(%r12), %xmm0
.Ltmp8425:
	.loc	1 1192 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp8426:
	.loc	1 1280 32
	vmovss	824(%r12), %xmm0
.Ltmp8427:
	.loc	1 1192 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp8428:
	.loc	1 1280 32
	vmovss	984(%r12), %xmm0
.Ltmp8429:
	.loc	1 1192 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp8430:
	.loc	1 1280 32
	vmovss	1144(%r12), %xmm0
.Ltmp8431:
	.loc	1 1192 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp8432:
	.loc	1 1280 32
	vmovss	40(%r12), %xmm0
.Ltmp8433:
	.loc	1 1192 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp8434:
	.loc	1 1280 32
	vmovss	200(%r12), %xmm0
.Ltmp8435:
	.loc	1 1192 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp8436:
	.loc	1 1280 32
	vmovss	360(%r12), %xmm0
.Ltmp8437:
	.loc	1 1192 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp8438:
	.loc	1 1280 32
	vmovss	520(%r12), %xmm0
.Ltmp8439:
	.loc	1 1192 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp8440:
	.loc	1 1280 32
	vmovss	680(%r12), %xmm0
.Ltmp8441:
	.loc	1 1192 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp8442:
	.loc	1 1280 32
	vmovss	840(%r12), %xmm0
.Ltmp8443:
	.loc	1 1192 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp8444:
	.loc	1 1280 32
	vmovss	1000(%r12), %xmm0
.Ltmp8445:
	.loc	1 1192 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp8446:
	.loc	1 1280 32
	vmovss	1160(%r12), %xmm0
.Ltmp8447:
	.loc	1 1192 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp8448:
	.loc	1 1280 32
	vmovss	56(%r12), %xmm0
.Ltmp8449:
	.loc	1 1192 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp8450:
	.loc	1 1280 32
	vmovss	216(%r12), %xmm0
.Ltmp8451:
	.loc	1 1192 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp8452:
	.loc	1 1280 32
	vmovss	376(%r12), %xmm0
.Ltmp8453:
	.loc	1 1192 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp8454:
	.loc	1 1280 32
	vmovss	536(%r12), %xmm0
.Ltmp8455:
	.loc	1 1192 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp8456:
	.loc	1 1280 32
	vmovss	696(%r12), %xmm0
.Ltmp8457:
	.loc	1 1192 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp8458:
	.loc	1 1280 32
	vmovss	856(%r12), %xmm0
.Ltmp8459:
	.loc	1 1192 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp8460:
	.loc	1 1280 32
	vmovss	1016(%r12), %xmm0
.Ltmp8461:
	.loc	1 1192 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp8462:
	.loc	1 1280 32
	vmovss	1176(%r12), %xmm0
.Ltmp8463:
	.loc	1 1192 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp8464:
	.loc	1 1280 32
	vmovss	72(%r12), %xmm0
.Ltmp8465:
	.loc	1 1192 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp8466:
	.loc	1 1280 32
	vmovss	232(%r12), %xmm0
.Ltmp8467:
	.loc	1 1192 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp8468:
	.loc	1 1280 32
	vmovss	392(%r12), %xmm0
.Ltmp8469:
	.loc	1 1192 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp8470:
	.loc	1 1280 32
	vmovss	552(%r12), %xmm0
.Ltmp8471:
	.loc	1 1192 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp8472:
	.loc	1 1280 32
	vmovss	712(%r12), %xmm0
.Ltmp8473:
	.loc	1 1192 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp8474:
	.loc	1 1280 32
	vmovss	872(%r12), %xmm0
.Ltmp8475:
	.loc	1 1192 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp8476:
	.loc	1 1280 32
	vmovss	1032(%r12), %xmm0
.Ltmp8477:
	.loc	1 1192 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp8478:
	.loc	1 1280 32
	vmovss	1192(%r12), %xmm0
.Ltmp8479:
	.loc	1 1192 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp8480:
	.loc	1 1280 32
	vmovss	88(%r12), %xmm0
.Ltmp8481:
	.loc	1 1192 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp8482:
	.loc	1 1280 32
	vmovss	248(%r12), %xmm0
.Ltmp8483:
	.loc	1 1192 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp8484:
	.loc	1 1280 32
	vmovss	408(%r12), %xmm0
.Ltmp8485:
	.loc	1 1192 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp8486:
	.loc	1 1280 32
	vmovss	568(%r12), %xmm0
.Ltmp8487:
	.loc	1 1192 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp8488:
	.loc	1 1280 32
	vmovss	728(%r12), %xmm0
.Ltmp8489:
	.loc	1 1192 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp8490:
	.loc	1 1280 32
	vmovss	888(%r12), %xmm0
.Ltmp8491:
	.loc	1 1192 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp8492:
	.loc	1 1280 32
	vmovss	1048(%r12), %xmm0
.Ltmp8493:
	.loc	1 1192 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp8494:
	.loc	1 1280 32
	vmovss	1208(%r12), %xmm0
.Ltmp8495:
	.loc	1 1192 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp8496:
	.loc	1 1280 32
	vmovss	104(%r12), %xmm0
.Ltmp8497:
	.loc	1 1192 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp8498:
	.loc	1 1280 32
	vmovss	264(%r12), %xmm0
.Ltmp8499:
	.loc	1 1192 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp8500:
	.loc	1 1280 32
	vmovss	424(%r12), %xmm0
.Ltmp8501:
	.loc	1 1192 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp8502:
	.loc	1 1280 32
	vmovss	584(%r12), %xmm0
.Ltmp8503:
	.loc	1 1192 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp8504:
	.loc	1 1280 32
	vmovss	744(%r12), %xmm0
.Ltmp8505:
	.loc	1 1192 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp8506:
	.loc	1 1280 32
	vmovss	904(%r12), %xmm0
.Ltmp8507:
	.loc	1 1192 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp8508:
	.loc	1 1280 32
	vmovss	1064(%r12), %xmm0
.Ltmp8509:
	.loc	1 1192 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp8510:
	.loc	1 1280 32
	vmovss	1224(%r12), %xmm0
.Ltmp8511:
	.loc	1 1192 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp8512:
	.loc	1 1280 32
	vmovss	120(%r12), %xmm0
.Ltmp8513:
	.loc	1 1192 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp8514:
	.loc	1 1280 32
	vmovss	280(%r12), %xmm0
.Ltmp8515:
	.loc	1 1192 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp8516:
	.loc	1 1280 32
	vmovss	440(%r12), %xmm0
.Ltmp8517:
	.loc	1 1192 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp8518:
	.loc	1 1280 32
	vmovss	600(%r12), %xmm0
.Ltmp8519:
	.loc	1 1192 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp8520:
	.loc	1 1280 32
	vmovss	760(%r12), %xmm0
.Ltmp8521:
	.loc	1 1192 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp8522:
	.loc	1 1280 32
	vmovss	920(%r12), %xmm0
.Ltmp8523:
	.loc	1 1192 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp8524:
	.loc	1 1280 32
	vmovss	1080(%r12), %xmm0
.Ltmp8525:
	.loc	1 1192 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp8526:
	.loc	1 1280 32
	vmovss	1240(%r12), %xmm0
.Ltmp8527:
	.loc	1 1192 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp8528:
	.loc	1 1280 32
	vmovss	136(%r12), %xmm0
.Ltmp8529:
	.loc	1 1192 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp8530:
	.loc	1 1280 32
	vmovss	296(%r12), %xmm0
.Ltmp8531:
	.loc	1 1192 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp8532:
	.loc	1 1280 32
	vmovss	456(%r12), %xmm0
.Ltmp8533:
	.loc	1 1192 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp8534:
	.loc	1 1280 32
	vmovss	616(%r12), %xmm0
.Ltmp8535:
	.loc	1 1192 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp8536:
	.loc	1 1280 32
	vmovss	776(%r12), %xmm0
.Ltmp8537:
	.loc	1 1192 28
	vmovss	%xmm0, 848(%rsp)
.Ltmp8538:
	.loc	1 1280 32
	vmovss	936(%r12), %xmm0
.Ltmp8539:
	.loc	1 1192 28
	vmovss	%xmm0, 852(%rsp)
.Ltmp8540:
	.loc	1 1280 32
	vmovss	1096(%r12), %xmm0
.Ltmp8541:
	.loc	1 1192 28
	vmovss	%xmm0, 856(%rsp)
.Ltmp8542:
	.loc	1 1280 32
	vmovss	1256(%r12), %xmm0
.Ltmp8543:
	.loc	1 1192 28
	vmovss	%xmm0, 860(%rsp)
.Ltmp8544:
	.loc	1 1280 32
	vmovss	152(%r12), %xmm0
.Ltmp8545:
	.loc	1 1192 28
	vmovss	%xmm0, 864(%rsp)
.Ltmp8546:
	.loc	1 1280 32
	vmovss	312(%r12), %xmm0
.Ltmp8547:
	.loc	1 1192 28
	vmovss	%xmm0, 868(%rsp)
.Ltmp8548:
	.loc	1 1280 32
	vmovss	472(%r12), %xmm0
.Ltmp8549:
	.loc	1 1192 28
	vmovss	%xmm0, 872(%rsp)
.Ltmp8550:
	.loc	1 1280 32
	vmovss	632(%r12), %xmm0
.Ltmp8551:
	.loc	1 1192 28
	vmovss	%xmm0, 876(%rsp)
.Ltmp8552:
	.loc	1 1280 32
	vmovss	792(%r12), %xmm0
.Ltmp8553:
	.loc	1 1192 28
	vmovss	%xmm0, 880(%rsp)
.Ltmp8554:
	.loc	1 1280 32
	vmovss	952(%r12), %xmm0
.Ltmp8555:
	.loc	1 1192 28
	vmovss	%xmm0, 884(%rsp)
.Ltmp8556:
	.loc	1 1280 32
	vmovss	1112(%r12), %xmm0
.Ltmp8557:
	.loc	1 1192 28
	vmovss	%xmm0, 888(%rsp)
.Ltmp8558:
	.loc	1 1280 32
	vmovss	1272(%r12), %xmm0
.Ltmp8559:
	.loc	1 1192 28
	vmovss	%xmm0, 892(%rsp)
.Ltmp8560:
	.loc	1 1279 33
	vmovss	2624(%r12), %xmm0
.Ltmp8561:
	.loc	1 1192 28
	vmovss	%xmm0, 896(%rsp)
.Ltmp8562:
	.loc	1 1279 33
	vmovss	2784(%r12), %xmm0
.Ltmp8563:
	.loc	1 1192 28
	vmovss	%xmm0, 900(%rsp)
.Ltmp8564:
	.loc	1 1279 33
	vmovss	2944(%r12), %xmm0
.Ltmp8565:
	.loc	1 1192 28
	vmovss	%xmm0, 904(%rsp)
.Ltmp8566:
	.loc	1 1279 33
	vmovss	3104(%r12), %xmm0
.Ltmp8567:
	.loc	1 1192 28
	vmovss	%xmm0, 908(%rsp)
.Ltmp8568:
	.loc	1 1279 33
	vmovss	3264(%r12), %xmm0
.Ltmp8569:
	.loc	1 1192 28
	vmovss	%xmm0, 912(%rsp)
.Ltmp8570:
	.loc	1 1279 33
	vmovss	3424(%r12), %xmm0
.Ltmp8571:
	.loc	1 1192 28
	vmovss	%xmm0, 916(%rsp)
.Ltmp8572:
	.loc	1 1279 33
	vmovss	3584(%r12), %xmm0
.Ltmp8573:
	.loc	1 1192 28
	vmovss	%xmm0, 920(%rsp)
.Ltmp8574:
	.loc	1 1279 33
	vmovss	3744(%r12), %xmm0
.Ltmp8575:
	.loc	1 1192 28
	vmovss	%xmm0, 924(%rsp)
.Ltmp8576:
	.loc	1 1279 33
	vmovss	2640(%r12), %xmm0
.Ltmp8577:
	.loc	1 1192 28
	vmovss	%xmm0, 928(%rsp)
.Ltmp8578:
	.loc	1 1279 33
	vmovss	2800(%r12), %xmm0
.Ltmp8579:
	.loc	1 1192 28
	vmovss	%xmm0, 932(%rsp)
.Ltmp8580:
	.loc	1 1279 33
	vmovss	2960(%r12), %xmm0
.Ltmp8581:
	.loc	1 1192 28
	vmovss	%xmm0, 936(%rsp)
.Ltmp8582:
	.loc	1 1279 33
	vmovss	3120(%r12), %xmm0
.Ltmp8583:
	.loc	1 1192 28
	vmovss	%xmm0, 940(%rsp)
.Ltmp8584:
	.loc	1 1279 33
	vmovss	3280(%r12), %xmm0
.Ltmp8585:
	.loc	1 1192 28
	vmovss	%xmm0, 944(%rsp)
.Ltmp8586:
	.loc	1 1279 33
	vmovss	3440(%r12), %xmm0
.Ltmp8587:
	.loc	1 1192 28
	vmovss	%xmm0, 948(%rsp)
.Ltmp8588:
	.loc	1 1279 33
	vmovss	3600(%r12), %xmm0
.Ltmp8589:
	.loc	1 1192 28
	vmovss	%xmm0, 952(%rsp)
.Ltmp8590:
	.loc	1 1279 33
	vmovss	3760(%r12), %xmm0
.Ltmp8591:
	.loc	1 1192 28
	vmovss	%xmm0, 956(%rsp)
.Ltmp8592:
	.loc	1 1279 33
	vmovss	2656(%r12), %xmm0
.Ltmp8593:
	.loc	1 1192 28
	vmovss	%xmm0, 960(%rsp)
.Ltmp8594:
	.loc	1 1279 33
	vmovss	2816(%r12), %xmm0
.Ltmp8595:
	.loc	1 1192 28
	vmovss	%xmm0, 964(%rsp)
.Ltmp8596:
	.loc	1 1279 33
	vmovss	2976(%r12), %xmm0
.Ltmp8597:
	.loc	1 1192 28
	vmovss	%xmm0, 968(%rsp)
.Ltmp8598:
	.loc	1 1279 33
	vmovss	3136(%r12), %xmm0
.Ltmp8599:
	.loc	1 1192 28
	vmovss	%xmm0, 972(%rsp)
.Ltmp8600:
	.loc	1 1279 33
	vmovss	3296(%r12), %xmm0
.Ltmp8601:
	.loc	1 1192 28
	vmovss	%xmm0, 976(%rsp)
.Ltmp8602:
	.loc	1 1279 33
	vmovss	3456(%r12), %xmm0
.Ltmp8603:
	.loc	1 1192 28
	vmovss	%xmm0, 980(%rsp)
.Ltmp8604:
	.loc	1 1279 33
	vmovss	3616(%r12), %xmm0
.Ltmp8605:
	.loc	1 1192 28
	vmovss	%xmm0, 984(%rsp)
.Ltmp8606:
	.loc	1 1279 33
	vmovss	3776(%r12), %xmm0
.Ltmp8607:
	.loc	1 1192 28
	vmovss	%xmm0, 988(%rsp)
.Ltmp8608:
	.loc	1 1279 33
	vmovss	2672(%r12), %xmm0
.Ltmp8609:
	.loc	1 1192 28
	vmovss	%xmm0, 992(%rsp)
.Ltmp8610:
	.loc	1 1279 33
	vmovss	2832(%r12), %xmm0
.Ltmp8611:
	.loc	1 1192 28
	vmovss	%xmm0, 996(%rsp)
.Ltmp8612:
	.loc	1 1279 33
	vmovss	2992(%r12), %xmm0
.Ltmp8613:
	.loc	1 1192 28
	vmovss	%xmm0, 1000(%rsp)
.Ltmp8614:
	.loc	1 1279 33
	vmovss	3152(%r12), %xmm0
.Ltmp8615:
	.loc	1 1192 28
	vmovss	%xmm0, 1004(%rsp)
.Ltmp8616:
	.loc	1 1279 33
	vmovss	3312(%r12), %xmm0
.Ltmp8617:
	.loc	1 1192 28
	vmovss	%xmm0, 1008(%rsp)
.Ltmp8618:
	.loc	1 1279 33
	vmovss	3472(%r12), %xmm0
.Ltmp8619:
	.loc	1 1192 28
	vmovss	%xmm0, 1012(%rsp)
.Ltmp8620:
	.loc	1 1279 33
	vmovss	3632(%r12), %xmm0
.Ltmp8621:
	.loc	1 1192 28
	vmovss	%xmm0, 1016(%rsp)
.Ltmp8622:
	.loc	1 1279 33
	vmovss	3792(%r12), %xmm0
.Ltmp8623:
	.loc	1 1192 28
	vmovss	%xmm0, 1020(%rsp)
.Ltmp8624:
	.loc	1 1279 33
	vmovss	2688(%r12), %xmm0
.Ltmp8625:
	.loc	1 1192 28
	vmovss	%xmm0, 1024(%rsp)
.Ltmp8626:
	.loc	1 1279 33
	vmovss	2848(%r12), %xmm0
.Ltmp8627:
	.loc	1 1192 28
	vmovss	%xmm0, 1028(%rsp)
.Ltmp8628:
	.loc	1 1279 33
	vmovss	3008(%r12), %xmm0
.Ltmp8629:
	.loc	1 1192 28
	vmovss	%xmm0, 1032(%rsp)
.Ltmp8630:
	.loc	1 1279 33
	vmovss	3168(%r12), %xmm0
.Ltmp8631:
	.loc	1 1192 28
	vmovss	%xmm0, 1036(%rsp)
.Ltmp8632:
	.loc	1 1279 33
	vmovss	3328(%r12), %xmm0
.Ltmp8633:
	.loc	1 1192 28
	vmovss	%xmm0, 1040(%rsp)
.Ltmp8634:
	.loc	1 1279 33
	vmovss	3488(%r12), %xmm0
.Ltmp8635:
	.loc	1 1192 28
	vmovss	%xmm0, 1044(%rsp)
.Ltmp8636:
	.loc	1 1279 33
	vmovss	3648(%r12), %xmm0
.Ltmp8637:
	.loc	1 1192 28
	vmovss	%xmm0, 1048(%rsp)
.Ltmp8638:
	.loc	1 1279 33
	vmovss	3808(%r12), %xmm0
.Ltmp8639:
	.loc	1 1192 28
	vmovss	%xmm0, 1052(%rsp)
.Ltmp8640:
	.loc	1 1279 33
	vmovss	2704(%r12), %xmm0
.Ltmp8641:
	.loc	1 1192 28
	vmovss	%xmm0, 1056(%rsp)
.Ltmp8642:
	.loc	1 1279 33
	vmovss	2864(%r12), %xmm0
.Ltmp8643:
	.loc	1 1192 28
	vmovss	%xmm0, 1060(%rsp)
.Ltmp8644:
	.loc	1 1279 33
	vmovss	3024(%r12), %xmm0
.Ltmp8645:
	.loc	1 1192 28
	vmovss	%xmm0, 1064(%rsp)
.Ltmp8646:
	.loc	1 1279 33
	vmovss	3184(%r12), %xmm0
.Ltmp8647:
	.loc	1 1192 28
	vmovss	%xmm0, 1068(%rsp)
.Ltmp8648:
	.loc	1 1279 33
	vmovss	3344(%r12), %xmm0
.Ltmp8649:
	.loc	1 1192 28
	vmovss	%xmm0, 1072(%rsp)
.Ltmp8650:
	.loc	1 1279 33
	vmovss	3504(%r12), %xmm0
.Ltmp8651:
	.loc	1 1192 28
	vmovss	%xmm0, 1076(%rsp)
.Ltmp8652:
	.loc	1 1279 33
	vmovss	3664(%r12), %xmm0
.Ltmp8653:
	.loc	1 1192 28
	vmovss	%xmm0, 1080(%rsp)
.Ltmp8654:
	.loc	1 1279 33
	vmovss	3824(%r12), %xmm0
.Ltmp8655:
	.loc	1 1192 28
	vmovss	%xmm0, 1084(%rsp)
.Ltmp8656:
	.loc	1 1279 33
	vmovss	2720(%r12), %xmm0
.Ltmp8657:
	.loc	1 1192 28
	vmovss	%xmm0, 1088(%rsp)
.Ltmp8658:
	.loc	1 1279 33
	vmovss	2880(%r12), %xmm0
.Ltmp8659:
	.loc	1 1192 28
	vmovss	%xmm0, 1092(%rsp)
.Ltmp8660:
	.loc	1 1279 33
	vmovss	3040(%r12), %xmm0
.Ltmp8661:
	.loc	1 1192 28
	vmovss	%xmm0, 1096(%rsp)
.Ltmp8662:
	.loc	1 1279 33
	vmovss	3200(%r12), %xmm0
.Ltmp8663:
	.loc	1 1192 28
	vmovss	%xmm0, 1100(%rsp)
.Ltmp8664:
	.loc	1 1279 33
	vmovss	3360(%r12), %xmm0
.Ltmp8665:
	.loc	1 1192 28
	vmovss	%xmm0, 1104(%rsp)
.Ltmp8666:
	.loc	1 1279 33
	vmovss	3520(%r12), %xmm0
.Ltmp8667:
	.loc	1 1192 28
	vmovss	%xmm0, 1108(%rsp)
.Ltmp8668:
	.loc	1 1279 33
	vmovss	3680(%r12), %xmm0
.Ltmp8669:
	.loc	1 1192 28
	vmovss	%xmm0, 1112(%rsp)
.Ltmp8670:
	.loc	1 1279 33
	vmovss	3840(%r12), %xmm0
.Ltmp8671:
	.loc	1 1192 28
	vmovss	%xmm0, 1116(%rsp)
.Ltmp8672:
	.loc	1 1279 33
	vmovss	2736(%r12), %xmm0
.Ltmp8673:
	.loc	1 1192 28
	vmovss	%xmm0, 1120(%rsp)
.Ltmp8674:
	.loc	1 1279 33
	vmovss	2896(%r12), %xmm0
.Ltmp8675:
	.loc	1 1192 28
	vmovss	%xmm0, 1124(%rsp)
.Ltmp8676:
	.loc	1 1279 33
	vmovss	3056(%r12), %xmm0
.Ltmp8677:
	.loc	1 1192 28
	vmovss	%xmm0, 1128(%rsp)
.Ltmp8678:
	.loc	1 1279 33
	vmovss	3216(%r12), %xmm0
.Ltmp8679:
	.loc	1 1192 28
	vmovss	%xmm0, 1132(%rsp)
.Ltmp8680:
	.loc	1 1279 33
	vmovss	3376(%r12), %xmm0
.Ltmp8681:
	.loc	1 1192 28
	vmovss	%xmm0, 1136(%rsp)
.Ltmp8682:
	.loc	1 1279 33
	vmovss	3536(%r12), %xmm0
.Ltmp8683:
	.loc	1 1192 28
	vmovss	%xmm0, 1140(%rsp)
.Ltmp8684:
	.loc	1 1279 33
	vmovss	3696(%r12), %xmm0
.Ltmp8685:
	.loc	1 1192 28
	vmovss	%xmm0, 1144(%rsp)
.Ltmp8686:
	.loc	1 1279 33
	vmovss	3856(%r12), %xmm0
.Ltmp8687:
	.loc	1 1192 28
	vmovss	%xmm0, 1148(%rsp)
.Ltmp8688:
	.loc	1 1279 33
	vmovss	2752(%r12), %xmm0
.Ltmp8689:
	.loc	1 1192 28
	vmovss	%xmm0, 1152(%rsp)
.Ltmp8690:
	.loc	1 1279 33
	vmovss	2912(%r12), %xmm0
.Ltmp8691:
	.loc	1 1192 28
	vmovss	%xmm0, 1156(%rsp)
.Ltmp8692:
	.loc	1 1279 33
	vmovss	3072(%r12), %xmm0
.Ltmp8693:
	.loc	1 1192 28
	vmovss	%xmm0, 1160(%rsp)
.Ltmp8694:
	.loc	1 1279 33
	vmovss	3232(%r12), %xmm0
.Ltmp8695:
	.loc	1 1192 28
	vmovss	%xmm0, 1164(%rsp)
.Ltmp8696:
	.loc	1 1279 33
	vmovss	3392(%r12), %xmm0
.Ltmp8697:
	.loc	1 1192 28
	vmovss	%xmm0, 1168(%rsp)
.Ltmp8698:
	.loc	1 1279 33
	vmovss	3552(%r12), %xmm0
.Ltmp8699:
	.loc	1 1192 28
	vmovss	%xmm0, 1172(%rsp)
.Ltmp8700:
	.loc	1 1279 33
	vmovss	3712(%r12), %xmm0
.Ltmp8701:
	.loc	1 1192 28
	vmovss	%xmm0, 1176(%rsp)
.Ltmp8702:
	.loc	1 1279 33
	vmovss	3872(%r12), %xmm0
.Ltmp8703:
	.loc	1 1192 28
	vmovss	%xmm0, 1180(%rsp)
.Ltmp8704:
	.loc	1 1279 33
	vmovss	2768(%r12), %xmm0
.Ltmp8705:
	.loc	1 1192 28
	vmovss	%xmm0, 1184(%rsp)
.Ltmp8706:
	.loc	1 1279 33
	vmovss	2928(%r12), %xmm0
.Ltmp8707:
	.loc	1 1192 28
	vmovss	%xmm0, 1188(%rsp)
.Ltmp8708:
	.loc	1 1279 33
	vmovss	3088(%r12), %xmm0
.Ltmp8709:
	.loc	1 1192 28
	vmovss	%xmm0, 1192(%rsp)
.Ltmp8710:
	.loc	1 1279 33
	vmovss	3248(%r12), %xmm0
.Ltmp8711:
	.loc	1 1192 28
	vmovss	%xmm0, 1196(%rsp)
.Ltmp8712:
	.loc	1 1279 33
	vmovss	3408(%r12), %xmm0
.Ltmp8713:
	.loc	1 1192 28
	vmovss	%xmm0, 1200(%rsp)
.Ltmp8714:
	.loc	1 1279 33
	vmovss	3568(%r12), %xmm0
.Ltmp8715:
	.loc	1 1192 28
	vmovss	%xmm0, 1204(%rsp)
.Ltmp8716:
	.loc	1 1279 33
	vmovss	3728(%r12), %xmm0
.Ltmp8717:
	.loc	1 1192 28
	vmovss	%xmm0, 1208(%rsp)
.Ltmp8718:
	.loc	1 1279 33
	vmovss	3888(%r12), %xmm0
.Ltmp8719:
	.loc	1 1192 28
	vmovss	%xmm0, 1212(%rsp)
.Ltmp8720:
	.loc	1 1280 32
	vmovss	2632(%r12), %xmm0
.Ltmp8721:
	.loc	1 1192 28
	vmovss	%xmm0, 1216(%rsp)
.Ltmp8722:
	.loc	1 1280 32
	vmovss	2792(%r12), %xmm0
.Ltmp8723:
	.loc	1 1192 28
	vmovss	%xmm0, 1220(%rsp)
.Ltmp8724:
	.loc	1 1280 32
	vmovss	2952(%r12), %xmm0
.Ltmp8725:
	.loc	1 1192 28
	vmovss	%xmm0, 1224(%rsp)
.Ltmp8726:
	.loc	1 1280 32
	vmovss	3112(%r12), %xmm0
.Ltmp8727:
	.loc	1 1192 28
	vmovss	%xmm0, 1228(%rsp)
.Ltmp8728:
	.loc	1 1280 32
	vmovss	3272(%r12), %xmm0
.Ltmp8729:
	.loc	1 1192 28
	vmovss	%xmm0, 1232(%rsp)
.Ltmp8730:
	.loc	1 1280 32
	vmovss	3432(%r12), %xmm0
.Ltmp8731:
	.loc	1 1192 28
	vmovss	%xmm0, 1236(%rsp)
.Ltmp8732:
	.loc	1 1280 32
	vmovss	3592(%r12), %xmm0
.Ltmp8733:
	.loc	1 1192 28
	vmovss	%xmm0, 1240(%rsp)
.Ltmp8734:
	.loc	1 1280 32
	vmovss	3752(%r12), %xmm0
.Ltmp8735:
	.loc	1 1192 28
	vmovss	%xmm0, 1244(%rsp)
.Ltmp8736:
	.loc	1 1280 32
	vmovss	2648(%r12), %xmm0
.Ltmp8737:
	.loc	1 1192 28
	vmovss	%xmm0, 1248(%rsp)
.Ltmp8738:
	.loc	1 1280 32
	vmovss	2808(%r12), %xmm0
.Ltmp8739:
	.loc	1 1192 28
	vmovss	%xmm0, 1252(%rsp)
.Ltmp8740:
	.loc	1 1280 32
	vmovss	2968(%r12), %xmm0
.Ltmp8741:
	.loc	1 1192 28
	vmovss	%xmm0, 1256(%rsp)
.Ltmp8742:
	.loc	1 1280 32
	vmovss	3128(%r12), %xmm0
.Ltmp8743:
	.loc	1 1192 28
	vmovss	%xmm0, 1260(%rsp)
.Ltmp8744:
	.loc	1 1280 32
	vmovss	3288(%r12), %xmm0
.Ltmp8745:
	.loc	1 1192 28
	vmovss	%xmm0, 1264(%rsp)
.Ltmp8746:
	.loc	1 1280 32
	vmovss	3448(%r12), %xmm0
.Ltmp8747:
	.loc	1 1192 28
	vmovss	%xmm0, 1268(%rsp)
.Ltmp8748:
	.loc	1 1280 32
	vmovss	3608(%r12), %xmm0
.Ltmp8749:
	.loc	1 1192 28
	vmovss	%xmm0, 1272(%rsp)
.Ltmp8750:
	.loc	1 1280 32
	vmovss	3768(%r12), %xmm0
.Ltmp8751:
	.loc	1 1192 28
	vmovss	%xmm0, 1276(%rsp)
.Ltmp8752:
	.loc	1 1280 32
	vmovss	2664(%r12), %xmm0
.Ltmp8753:
	.loc	1 1192 28
	vmovss	%xmm0, 1280(%rsp)
.Ltmp8754:
	.loc	1 1280 32
	vmovss	2824(%r12), %xmm0
.Ltmp8755:
	.loc	1 1192 28
	vmovss	%xmm0, 1284(%rsp)
.Ltmp8756:
	.loc	1 1280 32
	vmovss	2984(%r12), %xmm0
.Ltmp8757:
	.loc	1 1192 28
	vmovss	%xmm0, 1288(%rsp)
.Ltmp8758:
	.loc	1 1280 32
	vmovss	3144(%r12), %xmm0
.Ltmp8759:
	.loc	1 1192 28
	vmovss	%xmm0, 1292(%rsp)
.Ltmp8760:
	.loc	1 1280 32
	vmovss	3304(%r12), %xmm0
.Ltmp8761:
	.loc	1 1192 28
	vmovss	%xmm0, 1296(%rsp)
.Ltmp8762:
	.loc	1 1280 32
	vmovss	3464(%r12), %xmm0
.Ltmp8763:
	.loc	1 1192 28
	vmovss	%xmm0, 1300(%rsp)
.Ltmp8764:
	.loc	1 1280 32
	vmovss	3624(%r12), %xmm0
.Ltmp8765:
	.loc	1 1192 28
	vmovss	%xmm0, 1304(%rsp)
.Ltmp8766:
	.loc	1 1280 32
	vmovss	3784(%r12), %xmm0
.Ltmp8767:
	.loc	1 1192 28
	vmovss	%xmm0, 1308(%rsp)
.Ltmp8768:
	.loc	1 1280 32
	vmovss	2680(%r12), %xmm0
.Ltmp8769:
	.loc	1 1192 28
	vmovss	%xmm0, 1312(%rsp)
.Ltmp8770:
	.loc	1 1280 32
	vmovss	2840(%r12), %xmm0
.Ltmp8771:
	.loc	1 1192 28
	vmovss	%xmm0, 1316(%rsp)
.Ltmp8772:
	.loc	1 1280 32
	vmovss	3000(%r12), %xmm0
.Ltmp8773:
	.loc	1 1192 28
	vmovss	%xmm0, 1320(%rsp)
.Ltmp8774:
	.loc	1 1280 32
	vmovss	3160(%r12), %xmm0
.Ltmp8775:
	.loc	1 1192 28
	vmovss	%xmm0, 1324(%rsp)
.Ltmp8776:
	.loc	1 1280 32
	vmovss	3320(%r12), %xmm0
.Ltmp8777:
	.loc	1 1192 28
	vmovss	%xmm0, 1328(%rsp)
.Ltmp8778:
	.loc	1 1280 32
	vmovss	3480(%r12), %xmm0
.Ltmp8779:
	.loc	1 1192 28
	vmovss	%xmm0, 1332(%rsp)
.Ltmp8780:
	.loc	1 1280 32
	vmovss	3640(%r12), %xmm0
.Ltmp8781:
	.loc	1 1192 28
	vmovss	%xmm0, 1336(%rsp)
.Ltmp8782:
	.loc	1 1280 32
	vmovss	3800(%r12), %xmm0
.Ltmp8783:
	.loc	1 1192 28
	vmovss	%xmm0, 1340(%rsp)
.Ltmp8784:
	.loc	1 1280 32
	vmovss	2696(%r12), %xmm0
.Ltmp8785:
	.loc	1 1192 28
	vmovss	%xmm0, 1344(%rsp)
.Ltmp8786:
	.loc	1 1280 32
	vmovss	2856(%r12), %xmm0
.Ltmp8787:
	.loc	1 1192 28
	vmovss	%xmm0, 1348(%rsp)
.Ltmp8788:
	.loc	1 1280 32
	vmovss	3016(%r12), %xmm0
.Ltmp8789:
	.loc	1 1192 28
	vmovss	%xmm0, 1352(%rsp)
.Ltmp8790:
	.loc	1 1280 32
	vmovss	3176(%r12), %xmm0
.Ltmp8791:
	.loc	1 1192 28
	vmovss	%xmm0, 1356(%rsp)
.Ltmp8792:
	.loc	1 1280 32
	vmovss	3336(%r12), %xmm0
.Ltmp8793:
	.loc	1 1192 28
	vmovss	%xmm0, 1360(%rsp)
.Ltmp8794:
	.loc	1 1280 32
	vmovss	3496(%r12), %xmm0
.Ltmp8795:
	.loc	1 1192 28
	vmovss	%xmm0, 1364(%rsp)
.Ltmp8796:
	.loc	1 1280 32
	vmovss	3656(%r12), %xmm0
.Ltmp8797:
	.loc	1 1192 28
	vmovss	%xmm0, 1368(%rsp)
.Ltmp8798:
	.loc	1 1280 32
	vmovss	3816(%r12), %xmm0
.Ltmp8799:
	.loc	1 1192 28
	vmovss	%xmm0, 1372(%rsp)
.Ltmp8800:
	.loc	1 1280 32
	vmovss	2712(%r12), %xmm0
.Ltmp8801:
	.loc	1 1192 28
	vmovss	%xmm0, 1376(%rsp)
.Ltmp8802:
	.loc	1 1280 32
	vmovss	2872(%r12), %xmm0
.Ltmp8803:
	.loc	1 1192 28
	vmovss	%xmm0, 1380(%rsp)
.Ltmp8804:
	.loc	1 1280 32
	vmovss	3032(%r12), %xmm0
.Ltmp8805:
	.loc	1 1192 28
	vmovss	%xmm0, 1384(%rsp)
.Ltmp8806:
	.loc	1 1280 32
	vmovss	3192(%r12), %xmm0
.Ltmp8807:
	.loc	1 1192 28
	vmovss	%xmm0, 1388(%rsp)
.Ltmp8808:
	.loc	1 1280 32
	vmovss	3352(%r12), %xmm0
.Ltmp8809:
	.loc	1 1192 28
	vmovss	%xmm0, 1392(%rsp)
.Ltmp8810:
	.loc	1 1280 32
	vmovss	3512(%r12), %xmm0
.Ltmp8811:
	.loc	1 1192 28
	vmovss	%xmm0, 1396(%rsp)
.Ltmp8812:
	.loc	1 1280 32
	vmovss	3672(%r12), %xmm0
.Ltmp8813:
	.loc	1 1192 28
	vmovss	%xmm0, 1400(%rsp)
.Ltmp8814:
	.loc	1 1280 32
	vmovss	3832(%r12), %xmm0
.Ltmp8815:
	.loc	1 1192 28
	vmovss	%xmm0, 1404(%rsp)
.Ltmp8816:
	.loc	1 1280 32
	vmovss	2728(%r12), %xmm0
.Ltmp8817:
	.loc	1 1192 28
	vmovss	%xmm0, 1408(%rsp)
.Ltmp8818:
	.loc	1 1280 32
	vmovss	2888(%r12), %xmm0
.Ltmp8819:
	.loc	1 1192 28
	vmovss	%xmm0, 1412(%rsp)
.Ltmp8820:
	.loc	1 1280 32
	vmovss	3048(%r12), %xmm0
.Ltmp8821:
	.loc	1 1192 28
	vmovss	%xmm0, 1416(%rsp)
.Ltmp8822:
	.loc	1 1280 32
	vmovss	3208(%r12), %xmm0
.Ltmp8823:
	.loc	1 1192 28
	vmovss	%xmm0, 1420(%rsp)
.Ltmp8824:
	.loc	1 1280 32
	vmovss	3368(%r12), %xmm0
.Ltmp8825:
	.loc	1 1192 28
	vmovss	%xmm0, 1424(%rsp)
.Ltmp8826:
	.loc	1 1280 32
	vmovss	3528(%r12), %xmm0
.Ltmp8827:
	.loc	1 1192 28
	vmovss	%xmm0, 1428(%rsp)
.Ltmp8828:
	.loc	1 1280 32
	vmovss	3688(%r12), %xmm0
.Ltmp8829:
	.loc	1 1192 28
	vmovss	%xmm0, 1432(%rsp)
.Ltmp8830:
	.loc	1 1280 32
	vmovss	3848(%r12), %xmm0
.Ltmp8831:
	.loc	1 1192 28
	vmovss	%xmm0, 1436(%rsp)
.Ltmp8832:
	.loc	1 1280 32
	vmovss	2744(%r12), %xmm0
.Ltmp8833:
	.loc	1 1192 28
	vmovss	%xmm0, 1440(%rsp)
.Ltmp8834:
	.loc	1 1280 32
	vmovss	2904(%r12), %xmm0
.Ltmp8835:
	.loc	1 1192 28
	vmovss	%xmm0, 1444(%rsp)
.Ltmp8836:
	.loc	1 1280 32
	vmovss	3064(%r12), %xmm0
.Ltmp8837:
	.loc	1 1192 28
	vmovss	%xmm0, 1448(%rsp)
.Ltmp8838:
	.loc	1 1280 32
	vmovss	3224(%r12), %xmm0
.Ltmp8839:
	.loc	1 1192 28
	vmovss	%xmm0, 1452(%rsp)
.Ltmp8840:
	.loc	1 1280 32
	vmovss	3384(%r12), %xmm0
.Ltmp8841:
	.loc	1 1192 28
	vmovss	%xmm0, 1456(%rsp)
.Ltmp8842:
	.loc	1 1280 32
	vmovss	3544(%r12), %xmm0
.Ltmp8843:
	.loc	1 1192 28
	vmovss	%xmm0, 1460(%rsp)
.Ltmp8844:
	.loc	1 1280 32
	vmovss	3704(%r12), %xmm0
.Ltmp8845:
	.loc	1 1192 28
	vmovss	%xmm0, 1464(%rsp)
.Ltmp8846:
	.loc	1 1280 32
	vmovss	3864(%r12), %xmm0
.Ltmp8847:
	.loc	1 1192 28
	vmovss	%xmm0, 1468(%rsp)
.Ltmp8848:
	.loc	1 1280 32
	vmovss	2760(%r12), %xmm0
.Ltmp8849:
	.loc	1 1192 28
	vmovss	%xmm0, 1472(%rsp)
.Ltmp8850:
	.loc	1 1280 32
	vmovss	2920(%r12), %xmm0
.Ltmp8851:
	.loc	1 1192 28
	vmovss	%xmm0, 1476(%rsp)
.Ltmp8852:
	.loc	1 1280 32
	vmovss	3080(%r12), %xmm0
.Ltmp8853:
	.loc	1 1192 28
	vmovss	%xmm0, 1480(%rsp)
.Ltmp8854:
	.loc	1 1280 32
	vmovss	3240(%r12), %xmm0
.Ltmp8855:
	.loc	1 1192 28
	vmovss	%xmm0, 1484(%rsp)
.Ltmp8856:
	.loc	1 1280 32
	vmovss	3400(%r12), %xmm0
.Ltmp8857:
	.loc	1 1192 28
	vmovss	%xmm0, 1488(%rsp)
.Ltmp8858:
	.loc	1 1280 32
	vmovss	3560(%r12), %xmm0
.Ltmp8859:
	.loc	1 1192 28
	vmovss	%xmm0, 1492(%rsp)
.Ltmp8860:
	.loc	1 1280 32
	vmovss	3720(%r12), %xmm0
.Ltmp8861:
	.loc	1 1192 28
	vmovss	%xmm0, 1496(%rsp)
.Ltmp8862:
	.loc	1 1280 32
	vmovss	3880(%r12), %xmm0
.Ltmp8863:
	.loc	1 1192 28
	vmovss	%xmm0, 1500(%rsp)
.Ltmp8864:
	.loc	1 1280 32
	vmovss	2776(%r12), %xmm0
.Ltmp8865:
	.loc	1 1192 28
	vmovss	%xmm0, 1504(%rsp)
.Ltmp8866:
	.loc	1 1280 32
	vmovss	2936(%r12), %xmm0
.Ltmp8867:
	.loc	1 1192 28
	vmovss	%xmm0, 1508(%rsp)
.Ltmp8868:
	.loc	1 1280 32
	vmovss	3096(%r12), %xmm0
.Ltmp8869:
	.loc	1 1192 28
	vmovss	%xmm0, 1512(%rsp)
.Ltmp8870:
	.loc	1 1280 32
	vmovss	3256(%r12), %xmm0
.Ltmp8871:
	.loc	1 1192 28
	vmovss	%xmm0, 1516(%rsp)
.Ltmp8872:
	.loc	1 1280 32
	vmovss	3416(%r12), %xmm0
.Ltmp8873:
	.loc	1 1192 28
	vmovss	%xmm0, 1520(%rsp)
.Ltmp8874:
	.loc	1 1280 32
	vmovss	3576(%r12), %xmm0
.Ltmp8875:
	.loc	1 1192 28
	vmovss	%xmm0, 1524(%rsp)
.Ltmp8876:
	.loc	1 1280 32
	vmovss	3736(%r12), %xmm0
.Ltmp8877:
	.loc	1 1192 28
	vmovss	%xmm0, 1528(%rsp)
.Ltmp8878:
	.loc	1 1280 32
	vmovss	3896(%r12), %xmm0
.Ltmp8879:
	.loc	1 1192 28
	vmovss	%xmm0, 1532(%rsp)
.Ltmp8880:
	.loc	1 1194 31
	leaq	3136(%rsp), %rdi
	movq	%r12, %rsi
	movl	1820(%rsp), %r14d
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	3328(%rsp), %rdi
	movq	2328(%rsp), %rsi
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1193 28
	vmovaps	3136(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	3168(%rsp), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	3200(%rsp), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	3232(%rsp), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	3328(%rsp), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	3360(%rsp), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	3392(%rsp), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	3424(%rsp), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	3456(%rsp), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	3488(%rsp), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
.Ltmp8881:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%r13), %rax
	shlq	$3, %r13
	leaq	(,%rax,8), %rsi
	.loc	1 1197 12 is_stmt 1
	testb	$1, 64(%rsp)
	movq	%r15, 2144(%rsp)
	movq	%rax, 2040(%rsp)
	je	.LBB40_376
.Ltmp8882:
	.loc	38 1050 16
	cmpq	%r13, %rsi
.Ltmp8883:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_657
.Ltmp8884:
	.loc	48 451 16 is_stmt 1
	cmpq	128(%rsp), %rsi
	ja	.LBB40_657
.Ltmp8885:
	.loc	48 451 16 is_stmt 0
	cmpq	120(%rsp), %rsi
	ja	.LBB40_658
.Ltmp8886:
	.loc	1 1053 27 is_stmt 1
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp8887:
	.loc	1 1054 26
	vmovaps	4032(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	4064(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	4096(%r12), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	4128(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
.Ltmp8888:
	.loc	1 1055 25
	vmovaps	2304(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	2336(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp8889:
	.loc	1 1056 24
	vmovaps	4928(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	4960(%r12), %ymm6
.Ltmp8890:
	.loc	1 1057 24
	movq	5272(%r12), %r10
.Ltmp8891:
	.loc	1 871 17
	movq	2368(%r12), %rax
	movq	2376(%r12), %rcx
.Ltmp8892:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2384(%r12), %rdx
	xorq	%rax, %rdx
	movq	2392(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2400(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	2408(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2416(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	orq	%rcx, %rdx
	xorq	2424(%r12), %rax
	orq	%rdx, %rax
	sete	1888(%rsp)
.Ltmp8893:
	.loc	1 871 17
	movq	4992(%r12), %rax
	movq	5000(%r12), %rcx
.Ltmp8894:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	5008(%r12), %rdx
	xorq	%rax, %rdx
	movq	5016(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5024(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	5032(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5040(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	xorq	5048(%r12), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	1856(%rsp)
.Ltmp8895:
	.loc	2 1916 50
	testq	%r15, %r15
	movl	$0, %r11d
	je	.LBB40_372
.Ltmp8896:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,8), %rax
	movq	%rax, 2048(%rsp)
	movq	240(%rsp), %rax
	leaq	(%rax,%r13,4), %rcx
	movq	248(%rsp), %rax
	leaq	(%rax,%r13,4), %rdx
.Ltmp8897:
	.loc	3 900 12 is_stmt 1
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r15
	movq	%r15, 176(%rsp)
	xorl	%edi, %edi
	xorl	%r9d, %r9d
	movq	%rcx, 168(%rsp)
	movq	%rdx, 184(%rsp)
.Ltmp8898:
	.loc	3 0 12 is_stmt 0
.Ltmp8899:
	.p2align	4
.LBB40_307:
	.loc	1 1064 21 is_stmt 1
	vmovaps	256(%rsp), %ymm0
	vmovaps	288(%rsp), %ymm1
	vmovaps	320(%rsp), %ymm2
.Ltmp8900:
	.loc	14 48 14
	vaddps	576(%rsp), %ymm0, %ymm0
.Ltmp8901:
	.loc	1 1063 17
	vmovaps	%ymm0, 256(%rsp)
	.loc	1 1066 21
	vmovaps	896(%rsp), %ymm0
.Ltmp8902:
	.loc	14 48 14
	vaddps	1216(%rsp), %ymm0, %ymm0
.Ltmp8903:
	.loc	1 1065 17
	vmovaps	%ymm0, 896(%rsp)
.Ltmp8904:
	.loc	14 48 14
	vaddps	608(%rsp), %ymm1, %ymm0
.Ltmp8905:
	.loc	1 1063 17
	vmovaps	%ymm0, 288(%rsp)
	.loc	1 1066 21
	vmovaps	928(%rsp), %ymm0
.Ltmp8906:
	.loc	14 48 14
	vaddps	1248(%rsp), %ymm0, %ymm0
.Ltmp8907:
	.loc	1 1065 17
	vmovaps	%ymm0, 928(%rsp)
.Ltmp8908:
	.loc	14 48 14
	vaddps	640(%rsp), %ymm2, %ymm0
.Ltmp8909:
	.loc	1 1063 17
	vmovaps	%ymm0, 320(%rsp)
	.loc	1 1066 21
	vmovaps	960(%rsp), %ymm0
.Ltmp8910:
	.loc	14 48 14
	vaddps	1280(%rsp), %ymm0, %ymm0
.Ltmp8911:
	.loc	1 1065 17
	vmovaps	%ymm0, 960(%rsp)
	.loc	1 1064 21
	vmovaps	352(%rsp), %ymm0
.Ltmp8912:
	.loc	14 48 14
	vaddps	672(%rsp), %ymm0, %ymm0
.Ltmp8913:
	.loc	1 1063 17
	vmovaps	%ymm0, 352(%rsp)
	.loc	1 1066 21
	vmovaps	992(%rsp), %ymm0
.Ltmp8914:
	.loc	14 48 14
	vaddps	1312(%rsp), %ymm0, %ymm0
.Ltmp8915:
	.loc	1 1065 17
	vmovaps	%ymm0, 992(%rsp)
	.loc	1 1064 21
	vmovaps	384(%rsp), %ymm0
.Ltmp8916:
	.loc	14 48 14
	vaddps	704(%rsp), %ymm0, %ymm0
.Ltmp8917:
	.loc	1 1063 17
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 1066 21
	vmovaps	1024(%rsp), %ymm0
.Ltmp8918:
	.loc	14 48 14
	vaddps	1344(%rsp), %ymm0, %ymm0
.Ltmp8919:
	.loc	1 1065 17
	vmovaps	%ymm0, 1024(%rsp)
	.loc	1 1064 21
	vmovaps	416(%rsp), %ymm0
.Ltmp8920:
	.loc	14 48 14
	vaddps	736(%rsp), %ymm0, %ymm0
.Ltmp8921:
	.loc	1 1063 17
	vmovaps	%ymm0, 416(%rsp)
	.loc	1 1066 21
	vmovaps	1056(%rsp), %ymm0
.Ltmp8922:
	.loc	14 48 14
	vaddps	1376(%rsp), %ymm0, %ymm0
.Ltmp8923:
	.loc	1 1065 17
	vmovaps	%ymm0, 1056(%rsp)
	.loc	1 1064 21
	vmovaps	448(%rsp), %ymm0
.Ltmp8924:
	.loc	14 48 14
	vaddps	768(%rsp), %ymm0, %ymm0
.Ltmp8925:
	.loc	1 1063 17
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 1066 21
	vmovaps	1088(%rsp), %ymm0
.Ltmp8926:
	.loc	14 48 14
	vaddps	1408(%rsp), %ymm0, %ymm0
.Ltmp8927:
	.loc	1 1065 17
	vmovaps	%ymm0, 1088(%rsp)
	.loc	1 1064 21
	vmovaps	480(%rsp), %ymm0
.Ltmp8928:
	.loc	14 48 14
	vaddps	800(%rsp), %ymm0, %ymm0
.Ltmp8929:
	.loc	1 1063 17
	vmovaps	%ymm0, 480(%rsp)
	.loc	1 1066 21
	vmovaps	1120(%rsp), %ymm0
.Ltmp8930:
	.loc	14 48 14
	vaddps	1440(%rsp), %ymm0, %ymm0
.Ltmp8931:
	.loc	1 1065 17
	vmovaps	%ymm0, 1120(%rsp)
	.loc	1 1064 21
	vmovaps	512(%rsp), %ymm0
.Ltmp8932:
	.loc	14 48 14
	vaddps	832(%rsp), %ymm0, %ymm0
.Ltmp8933:
	.loc	1 1063 17
	vmovaps	%ymm0, 512(%rsp)
	.loc	1 1066 21
	vmovaps	1152(%rsp), %ymm0
.Ltmp8934:
	.loc	14 48 14
	vaddps	1472(%rsp), %ymm0, %ymm0
.Ltmp8935:
	.loc	1 1065 17
	vmovaps	%ymm0, 1152(%rsp)
	.loc	1 1064 21
	vmovaps	544(%rsp), %ymm0
.Ltmp8936:
	.loc	14 48 14
	vaddps	864(%rsp), %ymm0, %ymm0
.Ltmp8937:
	.loc	1 1063 17
	vmovaps	%ymm0, 544(%rsp)
	.loc	1 1066 21
	vmovaps	1184(%rsp), %ymm0
.Ltmp8938:
	.loc	14 48 14
	vaddps	1504(%rsp), %ymm0, %ymm0
.Ltmp8939:
	.loc	1 1065 17
	vmovaps	%ymm0, 1184(%rsp)
.Ltmp8940:
	.loc	1 1070 28
	leaq	1(%r10), %rax
.Ltmp8941:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %r15d
	cmovaeq	%rbx, %r15
.Ltmp8942:
	.loc	48 568 12
	cmpq	2048(%rsp), %rdi
	ja	.LBB40_650
.Ltmp8943:
	.loc	48 438 16
	cmpq	%r9, 176(%rsp)
	je	.LBB40_641
.Ltmp8944:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,8), %rax
.Ltmp8945:
	.loc	1 1083 29 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp8946:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_642
.Ltmp8947:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8948:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm6, 2368(%rsp)
	vmovups	(%rcx,%rdi,4), %ymm7
.Ltmp8949:
	vmovups	(%rdx,%rdi,4), %ymm0
.Ltmp8950:
	vmovaps	1280(%r12), %ymm5
	vmovaps	1312(%r12), %ymm11
	vmovaps	1344(%r12), %ymm1
	vmovaps	3904(%r12), %ymm14
	vmovaps	3936(%r12), %ymm9
	vmovaps	3968(%r12), %ymm2
	vmovaps	64(%rsp), %ymm8
	vsubps	%ymm8, %ymm7, %ymm3
	vmulps	%ymm3, %ymm11, %ymm4
	vmovaps	1632(%rsp), %ymm10
	vmovaps	%ymm5, 2208(%rsp)
	vmulps	%ymm5, %ymm10, %ymm5
	vaddps	%ymm4, %ymm5, %ymm6
	vaddps	%ymm6, %ymm10, %ymm4
	vmulps	%ymm11, %ymm10, %ymm5
	vmulps	%ymm1, %ymm3, %ymm3
	vaddps	%ymm3, %ymm5, %ymm5
	vaddps	%ymm5, %ymm8, %ymm3
	vmulps	1376(%r12), %ymm4, %ymm15
	vmovaps	1568(%rsp), %ymm8
	vsubps	%ymm8, %ymm3, %ymm4
	vmulps	192(%rsp), %ymm11, %ymm3
	vmulps	%ymm4, %ymm1, %ymm1
	vaddps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 2176(%rsp)
	vaddps	%ymm1, %ymm8, %ymm3
	vmovaps	2272(%rsp), %ymm8
.Ltmp8951:
	vsubps	%ymm8, %ymm0, %ymm13
	vmulps	%ymm9, %ymm13, %ymm1
	vmovaps	1760(%rsp), %ymm10
	vmulps	%ymm14, %ymm10, %ymm12
	vaddps	%ymm1, %ymm12, %ymm1
	vaddps	%ymm1, %ymm10, %ymm12
	vmulps	4000(%r12), %ymm12, %ymm12
.Ltmp8952:
	.loc	1 1083 29 is_stmt 1
	movq	2592(%r12), %rcx
.Ltmp8953:
	.loc	8 551 14
	vmovups	%ymm3, (%rcx,%rax,4)
.Ltmp8954:
	.loc	1 1084 30
	movq	2616(%r12), %rsi
.Ltmp8955:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_643
.Ltmp8956:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8957:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm7, %ymm15, %ymm7
	vsubps	%ymm3, %ymm7, %ymm3
.Ltmp8958:
	.loc	1 1084 30 is_stmt 1
	movq	2608(%r12), %rcx
.Ltmp8959:
	.loc	8 551 14
	vmovups	%ymm3, (%rcx,%rax,4)
.Ltmp8960:
	.loc	1 1085 28
	movq	5224(%r12), %rsi
.Ltmp8961:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_644
.Ltmp8962:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8963:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm14, 1920(%rsp)
	vmulps	%ymm9, %ymm10, %ymm3
	vmulps	%ymm2, %ymm13, %ymm7
	vaddps	%ymm7, %ymm3, %ymm13
	vaddps	%ymm13, %ymm8, %ymm3
	vmovaps	1728(%rsp), %ymm14
	vsubps	%ymm14, %ymm3, %ymm15
	vmulps	1600(%rsp), %ymm9, %ymm3
	vmulps	%ymm2, %ymm15, %ymm2
	vaddps	%ymm2, %ymm3, %ymm7
	vaddps	%ymm7, %ymm14, %ymm2
.Ltmp8964:
	.loc	1 1085 28 is_stmt 1
	movq	5216(%r12), %rcx
.Ltmp8965:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp8966:
	.loc	1 1086 29
	movq	5240(%r12), %rsi
.Ltmp8967:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_645
.Ltmp8968:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8969:
	.loc	48 0 16 is_stmt 0
	movq	%r9, 2080(%rsp)
	movq	%rdi, 2240(%rsp)
	vaddps	%ymm0, %ymm12, %ymm0
	vsubps	%ymm2, %ymm0, %ymm0
.Ltmp8970:
	.loc	1 1086 29 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp8971:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp8972:
	.loc	1 1089 13
	movq	2592(%r12), %rdi
	movq	2600(%r12), %rsi
	movq	2368(%r12), %rax
.Ltmp8973:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp8974:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp8975:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 1888(%rsp)
	je	.LBB40_323
.Ltmp8976:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB40_652
.Ltmp8977:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8978:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp8979:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
.Ltmp8980:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp8981:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp8982:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp8983:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%rdi,%r9,4), %ymm14
.Ltmp8984:
	movq	2608(%r12), %rcx
.Ltmp8985:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm0
.Ltmp8986:
	.loc	1 961 2
	jmp	.LBB40_340
.Ltmp8987:
	.loc	1 0 2 is_stmt 0
.Ltmp8988:
	.p2align	4
.LBB40_323:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB40_665
	.loc	1 0 25 is_stmt 0
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8989:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8990:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_695
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8991:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8992:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_694
	.loc	1 0 25
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8993:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8994:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_688
	.loc	1 0 25
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8995:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8996:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_677
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8997:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp8998:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 1824(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_683
	.loc	1 0 25
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp8999:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9000:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 1664(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_679
	.loc	1 0 25
	movq	2424(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9001:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9002:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_687
.Ltmp9003:
	.loc	1 0 25
	movq	%rcx, 152(%rsp)
.Ltmp9004:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
.Ltmp9005:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp9006:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp9007:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9008:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9009:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 48(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_673
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9010:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9011:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rdx
	movq	%rdx, 24(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_686
	.loc	1 0 25
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9012:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9013:
	.loc	1 955 30
	leaq	3(,%rcx,8), %rdx
	movq	%rdx, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_681
	.loc	1 0 25
	movq	%r13, 40(%rsp)
	movq	%r14, 32(%rsp)
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9014:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9015:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_690
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9016:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9017:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_689
	.loc	1 0 25
	movq	%r11, %r13
	movq	%r8, %r11
	movq	%r15, 56(%rsp)
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9018:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movq	%r10, %r8
	movq	%r12, %r10
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %rcx
.Ltmp9019:
	.loc	1 955 30
	leaq	6(,%rcx,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	2424(%r10), %rcx
	movq	%r8, %r15
	.loc	1 955 35
	addq	%r8, %rcx
.Ltmp9020:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %r8d
	cmovaeq	%rbx, %r8
	subq	%r8, %rcx
.Ltmp9021:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
.Ltmp9022:
	.loc	1 0 25
	movq	40(%rsp), %rsi
	vmovd	(%rdi,%rsi,4), %xmm0
	movq	1824(%rsp), %rsi
	vpinsrd	$1, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	1664(%rsp), %rsi
	vpinsrd	$2, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	152(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm0, %xmm0
	vmovd	(%rdi,%r9,4), %xmm2
	vpinsrd	$1, (%rdi,%r11,4), %xmm2, %xmm2
	vpinsrd	$2, (%rdi,%r13,4), %xmm2, %xmm2
	movq	32(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm2, %xmm2
.Ltmp9023:
	movq	2608(%r10), %rsi
.Ltmp9024:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm3
	vpinsrd	$1, (%rsi,%rdx,4), %xmm3, %xmm3
	vpinsrd	$2, (%rsi,%r12,4), %xmm3, %xmm3
	vpinsrd	$3, (%rsi,%rcx,4), %xmm3, %xmm3
	vmovd	(%rsi,%rax,4), %xmm12
	movq	48(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm12, %xmm12
	movq	24(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm12, %xmm12
	movq	16(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm12, %xmm12
.Ltmp9025:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm2, %ymm14
.Ltmp9026:
	.loc	8 551 14
	vinserti128	$1, %xmm3, %ymm12, %ymm0
	movq	%r10, %r12
	xorl	%r11d, %r11d
	movq	%r15, %r10
	movq	56(%rsp), %r15
	movq	136(%rsp), %rbx
.Ltmp9027:
.LBB40_340:
	.loc	1 1103 13 is_stmt 1
	movq	5216(%r12), %r9
	movq	5224(%r12), %rsi
	movq	4992(%r12), %rax
.Ltmp9028:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp9029:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	movq	%rax, %rdi
	subq	%rcx, %rdi
.Ltmp9030:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rdi
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 1856(%rsp)
	vmovdqa	%ymm0, 1824(%rsp)
	je	.LBB40_346
.Ltmp9031:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rdi, %r8
	jb	.LBB40_653
.Ltmp9032:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9033:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp9034:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
.Ltmp9035:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp9036:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp9037:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9038:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r9,%rdi,4), %ymm0
.Ltmp9039:
	movq	5232(%r12), %rcx
.Ltmp9040:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm2
.Ltmp9041:
	.loc	1 961 2
	jmp	.LBB40_363
.Ltmp9042:
	.loc	1 0 2 is_stmt 0
.Ltmp9043:
	.p2align	4
.LBB40_346:
	.loc	1 955 25 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB40_666
	.loc	1 0 25 is_stmt 0
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9044:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9045:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_697
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9046:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9047:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_694
	.loc	1 0 25
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9048:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9049:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_699
	.loc	1 0 25
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9050:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9051:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_704
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9052:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9053:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 1664(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_700
	.loc	1 0 25
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9054:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9055:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 152(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_761
	.loc	1 0 25
	movq	5048(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9056:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9057:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_706
.Ltmp9058:
	.loc	1 0 25
	movq	%rcx, 48(%rsp)
.Ltmp9059:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
.Ltmp9060:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp9061:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp9062:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9063:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9064:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 24(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_698
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9065:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9066:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rdx
	movq	%rdx, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_702
	.loc	1 0 25
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9067:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9068:
	.loc	1 955 30
	leaq	3(,%rcx,8), %rdx
	movq	%rdx, 32(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_762
	.loc	1 0 25
	movq	%r13, 160(%rsp)
	movq	%r14, 56(%rsp)
	movq	%r11, 40(%rsp)
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9069:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9070:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_734
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9071:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9072:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_710
	.loc	1 0 25
	movq	%rdi, %r13
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp9073:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movq	%r10, %rdi
	movq	%r12, %r10
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %rcx
.Ltmp9074:
	.loc	1 955 30
	leaq	6(,%rcx,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_737
	.loc	1 0 25
	movq	%r9, %r11
	movq	%r15, %r9
	movq	5048(%r10), %rcx
	movq	%rdi, %r15
	.loc	1 955 35
	addq	%rdi, %rcx
.Ltmp9075:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rcx
.Ltmp9076:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_727
.Ltmp9077:
	.loc	1 0 25
	movq	160(%rsp), %rsi
	vmovd	(%r11,%rsi,4), %xmm0
	movq	1664(%rsp), %rsi
	vpinsrd	$1, (%r11,%rsi,4), %xmm0, %xmm0
	movq	152(%rsp), %rsi
	vpinsrd	$2, (%r11,%rsi,4), %xmm0, %xmm0
	movq	48(%rsp), %rsi
	vpinsrd	$3, (%r11,%rsi,4), %xmm0, %xmm0
	vmovd	(%r11,%r13,4), %xmm2
	vpinsrd	$1, (%r11,%r8,4), %xmm2, %xmm2
	movq	40(%rsp), %rsi
	vpinsrd	$2, (%r11,%rsi,4), %xmm2, %xmm2
	movq	56(%rsp), %rsi
	vpinsrd	$3, (%r11,%rsi,4), %xmm2, %xmm2
.Ltmp9078:
	movq	5232(%r10), %rsi
.Ltmp9079:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm3
	vpinsrd	$1, (%rsi,%rdx,4), %xmm3, %xmm3
	vpinsrd	$2, (%rsi,%r12,4), %xmm3, %xmm3
	vpinsrd	$3, (%rsi,%rcx,4), %xmm3, %xmm3
	vmovd	(%rsi,%rax,4), %xmm12
	movq	24(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm12, %xmm12
	movq	16(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm12, %xmm12
	movq	32(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm12, %xmm12
.Ltmp9080:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm2, %ymm0
.Ltmp9081:
	.loc	8 551 14
	vinserti128	$1, %xmm3, %ymm12, %ymm2
	movq	%r10, %r12
	xorl	%r11d, %r11d
	movq	%r15, %r10
	movq	%r9, %r15
	movq	136(%rsp), %rbx
.Ltmp9082:
.LBB40_363:
	.loc	1 0 0
	negq	%r15
	addq	%r15, %r10
	incq	%r10
	leaq	(,%r10,8), %rax
.Ltmp9083:
	.loc	1 1150 36 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp9084:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	2144(%rsp), %r15
	movq	184(%rsp), %rdx
	movq	2240(%rsp), %rdi
	movq	2080(%rsp), %r9
	jb	.LBB40_647
.Ltmp9085:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9086:
	.loc	1 1152 27
	movq	2616(%r12), %rsi
.Ltmp9087:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_648
.Ltmp9088:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9089:
	.loc	1 1153 35
	movq	5224(%r12), %rsi
.Ltmp9090:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_649
.Ltmp9091:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9092:
	.loc	1 1155 27
	movq	5240(%r12), %rsi
.Ltmp9093:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_638
.Ltmp9094:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9095:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm6, %ymm6, %ymm3
	vaddps	1632(%rsp), %ymm3, %ymm3
	vbroadcastss	.LCPI40_1(%rip), %ymm6
	vandps	%ymm6, %ymm3, %ymm12
	vmovdqa	%ymm2, 1664(%rsp)
	vmovdqa	%ymm14, %ymm2
	vmovaps	%ymm8, %ymm14
	vmovaps	%ymm10, %ymm8
	vbroadcastss	.LCPI40_2(%rip), %ymm10
	vcmplt_oqps	%ymm10, %ymm12, %ymm12
	vandnps	%ymm3, %ymm12, %ymm3
	vmovaps	%ymm3, 1632(%rsp)
	vaddps	%ymm5, %ymm5, %ymm3
	vaddps	64(%rsp), %ymm3, %ymm3
	vandps	%ymm6, %ymm3, %ymm5
	vcmplt_oqps	%ymm10, %ymm5, %ymm5
	vandnps	%ymm3, %ymm5, %ymm3
	vmovaps	%ymm3, 64(%rsp)
	vmulps	%ymm4, %ymm11, %ymm3
	vmovaps	192(%rsp), %ymm5
	vmulps	2208(%rsp), %ymm5, %ymm4
	vaddps	%ymm3, %ymm4, %ymm3
	vaddps	%ymm3, %ymm3, %ymm3
	vaddps	%ymm3, %ymm5, %ymm3
	vandps	%ymm6, %ymm3, %ymm4
	vcmplt_oqps	%ymm10, %ymm4, %ymm4
	vandnps	%ymm3, %ymm4, %ymm3
	vmovaps	%ymm3, 192(%rsp)
	vmovaps	2176(%rsp), %ymm3
	vaddps	%ymm3, %ymm3, %ymm3
	vaddps	1568(%rsp), %ymm3, %ymm3
	vandps	%ymm6, %ymm3, %ymm4
	vcmplt_oqps	%ymm10, %ymm4, %ymm4
	vandnps	%ymm3, %ymm4, %ymm3
	vmovaps	%ymm3, 1568(%rsp)
.Ltmp9096:
	vaddps	%ymm1, %ymm1, %ymm1
	vaddps	%ymm1, %ymm8, %ymm1
	vandps	%ymm6, %ymm1, %ymm3
	vcmplt_oqps	%ymm10, %ymm3, %ymm3
	vandnps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 1760(%rsp)
	vaddps	%ymm13, %ymm13, %ymm1
	vaddps	%ymm1, %ymm14, %ymm1
	vandps	%ymm6, %ymm1, %ymm3
	vcmplt_oqps	%ymm10, %ymm3, %ymm3
	vandnps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 2272(%rsp)
	vmulps	%ymm15, %ymm9, %ymm1
	vmovaps	1600(%rsp), %ymm4
	vmulps	1920(%rsp), %ymm4, %ymm3
	vaddps	%ymm1, %ymm3, %ymm1
	vaddps	%ymm1, %ymm1, %ymm1
	vaddps	%ymm1, %ymm4, %ymm1
	vandps	%ymm6, %ymm1, %ymm3
	vcmplt_oqps	%ymm10, %ymm3, %ymm3
	vandnps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 1600(%rsp)
	vaddps	%ymm7, %ymm7, %ymm1
	vaddps	1728(%rsp), %ymm1, %ymm1
	vandps	%ymm6, %ymm1, %ymm3
	vcmplt_oqps	%ymm10, %ymm3, %ymm3
	vandnps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 1728(%rsp)
.Ltmp9097:
	vpand	%ymm6, %ymm2, %ymm1
	vpand	%ymm6, %ymm0, %ymm0
	vmaxps	%ymm0, %ymm1, %ymm0
.Ltmp9098:
	vandps	1824(%rsp), %ymm6, %ymm1
	vandps	1664(%rsp), %ymm6, %ymm2
	vmaxps	%ymm2, %ymm1, %ymm1
	vbroadcastss	.LCPI40_4(%rip), %ymm9
.Ltmp9099:
	vmaxps	%ymm9, %ymm0, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm2
	vbroadcastss	.LCPI40_5(%rip), %ymm11
	vmaxps	%ymm11, %ymm0, %ymm0
	vandps	%ymm2, %ymm0, %ymm3
	vbroadcastsd	.LCPI40_7(%rip), %ymm4
	vorps	%ymm4, %ymm3, %ymm3
	vbroadcastss	.LCPI40_8(%rip), %ymm11
	vaddps	%ymm3, %ymm11, %ymm3
	vbroadcastss	.LCPI40_9(%rip), %ymm13
	vmulps	%ymm3, %ymm13, %ymm5
	vbroadcastss	.LCPI40_10(%rip), %ymm14
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_11(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_12(%rip), %ymm7
	vaddps	%ymm7, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_13(%rip), %ymm7
	vaddps	%ymm7, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_14(%rip), %ymm7
	vaddps	%ymm7, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm3
	vpsrld	$23, %ymm0, %ymm0
	vpbroadcastd	.LCPI40_15(%rip), %ymm5
	vpor	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm12
	vaddps	%ymm0, %ymm12, %ymm0
	vaddps	%ymm3, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm13
	vmulps	%ymm0, %ymm13, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm14
	vmaxps	%ymm14, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm15
	vminps	%ymm15, %ymm0, %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vsubps	256(%rsp), %ymm0, %ymm3
	vbroadcastss	.LCPI40_20(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm7
	vmulps	%ymm7, %ymm7, %ymm7
	vbroadcastss	.LCPI40_22(%rip), %ymm12
	vmulps	%ymm7, %ymm12, %ymm7
	vcmpgt_oqps	%ymm0, %ymm3, %ymm8
	vblendvps	%ymm8, %ymm3, %ymm7, %ymm7
	vbroadcastss	.LCPI40_21(%rip), %ymm9
	vcmple_oqps	%ymm9, %ymm3, %ymm3
	vmulps	2336(%rsp), %ymm7, %ymm7
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm3, %ymm0, %ymm3
	vpandn	%ymm7, %ymm3, %ymm3
	vbroadcastss	.LCPI40_23(%rip), %ymm7
	vmaxps	%ymm7, %ymm3, %ymm3
	vminps	%ymm0, %ymm3, %ymm3
	vmovaps	2112(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm3, %ymm7
	vmovaps	2560(%rsp), %ymm8
	vblendvps	%ymm7, 2592(%rsp), %ymm8, %ymm7
	vsubps	%ymm3, %ymm0, %ymm8
	vmulps	%ymm7, %ymm8, %ymm7
	vaddps	%ymm7, %ymm3, %ymm3
	vandps	%ymm6, %ymm3, %ymm7
	vcmplt_oqps	%ymm10, %ymm7, %ymm7
	vandnps	%ymm3, %ymm7, %ymm0
.Ltmp9100:
	vbroadcastss	.LCPI40_4(%rip), %ymm3
	vmaxps	%ymm3, %ymm1, %ymm1
	vbroadcastss	.LCPI40_5(%rip), %ymm3
	vmaxps	%ymm3, %ymm1, %ymm1
	vandps	%ymm2, %ymm1, %ymm2
	vorps	%ymm4, %ymm2, %ymm2
	vpsrld	$23, %ymm1, %ymm1
	vpor	%ymm5, %ymm1, %ymm1
	vaddps	%ymm2, %ymm11, %ymm2
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
	vbroadcastss	.LCPI40_16(%rip), %ymm3
	vaddps	%ymm3, %ymm1, %ymm1
	vaddps	%ymm2, %ymm1, %ymm1
	vmulps	%ymm1, %ymm13, %ymm1
	vmaxps	%ymm14, %ymm1, %ymm1
	vminps	%ymm15, %ymm1, %ymm1
	vmovaps	%ymm1, 1920(%rsp)
	vsubps	416(%rsp), %ymm1, %ymm1
	vbroadcastss	.LCPI40_20(%rip), %ymm2
	vaddps	%ymm2, %ymm1, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm12, %ymm3
	vcmpgt_oqps	%ymm2, %ymm1, %ymm4
	vblendvps	%ymm4, %ymm1, %ymm3, %ymm3
	vcmple_oqps	%ymm9, %ymm1, %ymm1
	vmulps	2528(%rsp), %ymm3, %ymm3
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%ymm1, %ymm2, %ymm1
	vpandn	%ymm3, %ymm1, %ymm1
	vmovaps	%ymm0, 2112(%rsp)
.Ltmp9101:
	vaddps	384(%rsp), %ymm0, %ymm3
	vbroadcastss	.LCPI40_24(%rip), %ymm8
	vmulps	%ymm3, %ymm8, %ymm3
	vbroadcastss	.LCPI40_25(%rip), %ymm12
	vmaxps	%ymm12, %ymm3, %ymm3
	vbroadcastss	.LCPI40_26(%rip), %ymm13
	vminps	%ymm13, %ymm3, %ymm3
	vroundps	$9, %ymm3, %ymm4
	vsubps	%ymm4, %ymm3, %ymm3
	vbroadcastss	.LCPI40_27(%rip), %ymm14
	vmulps	%ymm3, %ymm14, %ymm5
	vbroadcastss	.LCPI40_28(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_29(%rip), %ymm11
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_30(%rip), %ymm9
	vaddps	%ymm5, %ymm9, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm10
	vaddps	%ymm5, %ymm10, %ymm5
	vbroadcastss	.LCPI40_23(%rip), %ymm0
.Ltmp9102:
	vmaxps	%ymm0, %ymm1, %ymm1
	vminps	%ymm2, %ymm1, %ymm1
	vmovaps	1952(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm1, %ymm7
	vmovaps	2464(%rsp), %ymm8
	vblendvps	%ymm7, 2496(%rsp), %ymm8, %ymm7
.Ltmp9103:
	vmulps	%ymm5, %ymm3, %ymm3
.Ltmp9104:
	vsubps	%ymm1, %ymm0, %ymm5
	vmulps	%ymm7, %ymm5, %ymm5
	vaddps	%ymm5, %ymm1, %ymm1
	vandps	%ymm6, %ymm1, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm15
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm1, %ymm5, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm7
.Ltmp9105:
	vaddps	%ymm7, %ymm3, %ymm1
	vbroadcastss	.LCPI40_33(%rip), %ymm8
	vaddps	%ymm4, %ymm8, %ymm3
	vpslld	$23, %ymm3, %ymm3
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp9106:
	vaddps	544(%rsp), %ymm0, %ymm4
.Ltmp9107:
	vmulps	%ymm3, %ymm1, %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vbroadcastss	.LCPI40_24(%rip), %ymm2
.Ltmp9108:
	vmulps	%ymm2, %ymm4, %ymm3
	vmaxps	%ymm12, %ymm3, %ymm3
	vminps	%ymm13, %ymm3, %ymm3
	vroundps	$9, %ymm3, %ymm4
	vsubps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm14, %ymm5
	vbroadcastss	.LCPI40_28(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm9, %ymm5
	vmovaps	%ymm9, %ymm14
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm10, %ymm5
	vmulps	%ymm5, %ymm3, %ymm3
	vaddps	%ymm7, %ymm3, %ymm3
	vaddps	%ymm4, %ymm8, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmovaps	2208(%rsp), %ymm0
.Ltmp9109:
	vsubps	896(%rsp), %ymm0, %ymm5
.Ltmp9110:
	vmulps	%ymm4, %ymm3, %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm7
.Ltmp9111:
	vaddps	%ymm7, %ymm5, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vbroadcastss	.LCPI40_22(%rip), %ymm13
	vmulps	%ymm3, %ymm13, %ymm3
	vcmpgt_oqps	%ymm7, %ymm5, %ymm4
	vblendvps	%ymm4, %ymm5, %ymm3, %ymm3
	vbroadcastss	.LCPI40_21(%rip), %ymm12
	vcmple_oqps	%ymm12, %ymm5, %ymm4
	vmulps	2432(%rsp), %ymm3, %ymm3
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm4, %ymm0, %ymm4
	vpandn	%ymm3, %ymm4, %ymm3
	vbroadcastss	.LCPI40_23(%rip), %ymm4
	vmaxps	%ymm4, %ymm3, %ymm3
	vminps	%ymm0, %ymm3, %ymm3
	vxorps	%xmm9, %xmm9, %xmm9
	vmovaps	1984(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm3, %ymm4
	vmovaps	2720(%rsp), %ymm5
	vblendvps	%ymm4, 2400(%rsp), %ymm5, %ymm4
	vsubps	%ymm3, %ymm0, %ymm5
	vmulps	%ymm4, %ymm5, %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vandps	%ymm6, %ymm3, %ymm4
	vcmplt_oqps	%ymm15, %ymm4, %ymm4
	vandnps	%ymm3, %ymm4, %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vaddps	1024(%rsp), %ymm0, %ymm3
	vmulps	%ymm2, %ymm3, %ymm3
	vmovaps	%ymm2, %ymm15
	vbroadcastss	.LCPI40_25(%rip), %ymm8
	vmaxps	%ymm8, %ymm3, %ymm3
	vbroadcastss	.LCPI40_26(%rip), %ymm0
	vminps	%ymm0, %ymm3, %ymm3
	vroundps	$9, %ymm3, %ymm4
	vsubps	%ymm4, %ymm3, %ymm3
	vbroadcastss	.LCPI40_27(%rip), %ymm0
	vmulps	%ymm0, %ymm3, %ymm5
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm10, %ymm5
	vmulps	%ymm5, %ymm3, %ymm3
	vbroadcastss	.LCPI40_32(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm3
	vbroadcastss	.LCPI40_33(%rip), %ymm0
	vaddps	%ymm0, %ymm4, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmulps	%ymm4, %ymm3, %ymm4
	vmovaps	1920(%rsp), %ymm0
.Ltmp9112:
	vsubps	1056(%rsp), %ymm0, %ymm2
	vaddps	%ymm7, %ymm2, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm13, %ymm3
	vcmpgt_oqps	%ymm7, %ymm2, %ymm5
	vblendvps	%ymm5, %ymm2, %ymm3, %ymm3
	vcmple_oqps	%ymm12, %ymm2, %ymm2
	vmulps	2688(%rsp), %ymm3, %ymm3
	vpcmpgtd	%ymm2, %ymm9, %ymm2
	vpandn	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_23(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm2
	vminps	%ymm9, %ymm2, %ymm2
	vmovaps	2368(%rsp), %ymm5
	vcmplt_oqps	%ymm5, %ymm2, %ymm3
	vmovaps	2624(%rsp), %ymm0
	vblendvps	%ymm3, 2656(%rsp), %ymm0, %ymm3
	vsubps	%ymm2, %ymm5, %ymm5
	vmulps	%ymm3, %ymm5, %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vandps	%ymm6, %ymm2, %ymm3
	vbroadcastss	.LCPI40_2(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm3, %ymm3
	vandnps	%ymm2, %ymm3, %ymm6
	vaddps	1184(%rsp), %ymm6, %ymm2
	vmulps	%ymm2, %ymm15, %ymm2
	vmaxps	%ymm8, %ymm2, %ymm2
	vbroadcastss	.LCPI40_26(%rip), %ymm0
	vminps	%ymm0, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm3
	vsubps	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm0
	vmulps	%ymm0, %ymm2, %ymm5
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vaddps	%ymm5, %ymm10, %ymm5
	vmulps	%ymm5, %ymm2, %ymm2
	vbroadcastss	.LCPI40_32(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_33(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm3
	vpslld	$23, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
.Ltmp9113:
	movq	2592(%r12), %rcx
	vmovaps	2176(%rsp), %ymm0
	vmulps	(%rcx,%rax,4), %ymm0, %ymm1
	movq	2608(%r12), %rcx
	vmovaps	2208(%rsp), %ymm0
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
	vaddps	%ymm0, %ymm1, %ymm0
.Ltmp9114:
	movq	5216(%r12), %rcx
	vmulps	(%rcx,%rax,4), %ymm4, %ymm1
	.loc	1 1155 27 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp9115:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm2, %ymm2
.Ltmp9116:
	.loc	14 48 14
	vaddps	%ymm2, %ymm1, %ymm1
	movq	168(%rsp), %rcx
.Ltmp9117:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rdi,4)
.Ltmp9118:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm1, (%rdx,%rdi,4)
.Ltmp9119:
	.loc	1 0 0
	incq	%r9
.Ltmp9120:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r9, %r15
.Ltmp9121:
	.loc	3 900 12
	jne	.LBB40_307
.Ltmp9122:
.LBB40_372:
	.loc	3 0 12 is_stmt 0
	vmovaps	1632(%rsp), %ymm0
	.loc	1 1160 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r12)
	vmovaps	64(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r12)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r12)
	vmovaps	1568(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r12)
	vmovaps	1760(%rsp), %ymm0
	.loc	1 1161 5
	vmovaps	%ymm0, 4032(%r12)
	vmovaps	2272(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r12)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r12)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r12)
	vmovaps	2112(%rsp), %ymm0
	.loc	1 1162 5
	vmovaps	%ymm0, 2304(%r12)
	vmovaps	1952(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r12)
	vmovaps	1984(%rsp), %ymm0
	.loc	1 1163 5
	vmovaps	%ymm0, 4928(%r12)
	vmovaps	%ymm6, 4960(%r12)
	.loc	1 1164 5
	movq	%r10, 5272(%r12)
	xorl	%eax, %eax
.Ltmp9123:
	.loc	1 0 5 is_stmt 0
.Ltmp9124:
	.p2align	4
.LBB40_373:
	.loc	1 1297 13 is_stmt 1
	vmovss	256(%rsp,%rax,2), %xmm3
	vmovss	260(%rsp,%rax,2), %xmm4
	vmovss	264(%rsp,%rax,2), %xmm5
	vmovss	268(%rsp,%rax,2), %xmm6
	vmovss	272(%rsp,%rax,2), %xmm7
	vmovss	276(%rsp,%rax,2), %xmm2
	vmovss	280(%rsp,%rax,2), %xmm1
	vmovd	284(%rsp,%rax,2), %xmm0
.Ltmp9125:
	.loc	1 1300 17
	vmovss	%xmm3, (%r12,%rax)
	.loc	1 1301 34
	movl	12(%r12,%rax), %ecx
	movl	172(%r12,%rax), %edx
.Ltmp9126:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9127:
	.loc	1 1301 17
	movl	%ecx, 12(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm4, 160(%r12,%rax)
.Ltmp9128:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%r11d, %edx
.Ltmp9129:
	.loc	1 1301 17
	movl	%edx, 172(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm5, 320(%r12,%rax)
	.loc	1 1301 34
	movl	332(%r12,%rax), %ecx
.Ltmp9130:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9131:
	.loc	1 1301 17
	movl	%ecx, 332(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm6, 480(%r12,%rax)
	.loc	1 1301 34
	movl	492(%r12,%rax), %ecx
.Ltmp9132:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9133:
	.loc	1 1301 17
	movl	%ecx, 492(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm7, 640(%r12,%rax)
	.loc	1 1301 34
	movl	652(%r12,%rax), %ecx
.Ltmp9134:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9135:
	.loc	1 1301 17
	movl	%ecx, 652(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 800(%r12,%rax)
	.loc	1 1301 34
	movl	812(%r12,%rax), %ecx
.Ltmp9136:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9137:
	.loc	1 1301 17
	movl	%ecx, 812(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 960(%r12,%rax)
	.loc	1 1301 34
	movl	972(%r12,%rax), %ecx
.Ltmp9138:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9139:
	.loc	1 1301 17
	movl	%ecx, 972(%r12,%rax)
	.loc	1 1300 17
	vmovd	%xmm0, 1120(%r12,%rax)
	.loc	1 1301 34
	movl	1132(%r12,%rax), %ecx
.Ltmp9140:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9141:
	.loc	1 1301 17
	movl	%ecx, 1132(%r12,%rax)
.Ltmp9142:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp9143:
	.loc	3 900 12
	jne	.LBB40_373
.Ltmp9144:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	.p2align	4
.LBB40_375:
.Ltmp9145:
	.loc	1 1297 13 is_stmt 1
	vmovss	896(%rsp,%rax,2), %xmm3
	vmovss	900(%rsp,%rax,2), %xmm4
	vmovss	904(%rsp,%rax,2), %xmm5
	vmovss	908(%rsp,%rax,2), %xmm6
	vmovss	912(%rsp,%rax,2), %xmm7
	vmovss	916(%rsp,%rax,2), %xmm2
	vmovss	920(%rsp,%rax,2), %xmm1
	vmovd	924(%rsp,%rax,2), %xmm0
.Ltmp9146:
	.loc	1 1300 17
	vmovss	%xmm3, 2624(%r12,%rax)
	.loc	1 1301 34
	movl	2636(%r12,%rax), %ecx
	movl	2796(%r12,%rax), %edx
.Ltmp9147:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9148:
	.loc	1 1301 17
	movl	%ecx, 2636(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm4, 2784(%r12,%rax)
.Ltmp9149:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%r11d, %edx
.Ltmp9150:
	.loc	1 1301 17
	movl	%edx, 2796(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm5, 2944(%r12,%rax)
	.loc	1 1301 34
	movl	2956(%r12,%rax), %ecx
.Ltmp9151:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9152:
	.loc	1 1301 17
	movl	%ecx, 2956(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm6, 3104(%r12,%rax)
	.loc	1 1301 34
	movl	3116(%r12,%rax), %ecx
.Ltmp9153:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9154:
	.loc	1 1301 17
	movl	%ecx, 3116(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm7, 3264(%r12,%rax)
	.loc	1 1301 34
	movl	3276(%r12,%rax), %ecx
.Ltmp9155:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9156:
	.loc	1 1301 17
	movl	%ecx, 3276(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 3424(%r12,%rax)
	.loc	1 1301 34
	movl	3436(%r12,%rax), %ecx
.Ltmp9157:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9158:
	.loc	1 1301 17
	movl	%ecx, 3436(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 3584(%r12,%rax)
	.loc	1 1301 34
	movl	3596(%r12,%rax), %ecx
.Ltmp9159:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9160:
	.loc	1 1301 17
	movl	%ecx, 3596(%r12,%rax)
	.loc	1 1300 17
	vmovd	%xmm0, 3744(%r12,%rax)
	.loc	1 1301 34
	movl	3756(%r12,%rax), %ecx
.Ltmp9161:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp9162:
	.loc	1 1301 17
	movl	%ecx, 3756(%r12,%rax)
.Ltmp9163:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp9164:
	.loc	3 900 12
	jne	.LBB40_375
	jmp	.LBB40_300
.Ltmp9165:
.LBB40_376:
	.loc	38 1050 16
	cmpq	%r13, %rsi
.Ltmp9166:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_656
.Ltmp9167:
	.loc	48 451 16 is_stmt 1
	cmpq	128(%rsp), %rsi
	ja	.LBB40_656
.Ltmp9168:
	.loc	48 451 16 is_stmt 0
	cmpq	120(%rsp), %rsi
	ja	.LBB40_659
.Ltmp9169:
	.loc	1 1053 27 is_stmt 1
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
.Ltmp9170:
	.loc	1 1054 26
	vmovaps	4032(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	4064(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	4096(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	4128(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
.Ltmp9171:
	.loc	1 1055 25
	vmovaps	2304(%r12), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	2336(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
.Ltmp9172:
	.loc	1 1056 24
	vmovaps	4928(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	4960(%r12), %ymm8
.Ltmp9173:
	.loc	1 1057 24
	movq	5272(%r12), %rax
	movq	%rax, 64(%rsp)
.Ltmp9174:
	.loc	1 871 17
	movq	2368(%r12), %rax
	movq	2376(%r12), %rcx
.Ltmp9175:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2384(%r12), %rdx
	xorq	%rax, %rdx
	movq	2392(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2400(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	2408(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2416(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	orq	%rcx, %rdx
	xorq	2424(%r12), %rax
	orq	%rdx, %rax
	sete	2048(%rsp)
.Ltmp9176:
	.loc	1 871 17
	movq	4992(%r12), %rax
	movq	5000(%r12), %rcx
.Ltmp9177:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	5008(%r12), %rdx
	xorq	%rax, %rdx
	movq	5016(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5024(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	5032(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5040(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	xorq	5048(%r12), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	176(%rsp)
.Ltmp9178:
	.loc	2 1916 50
	testq	%r15, %r15
.Ltmp9179:
	.loc	3 900 12
	je	.LBB40_299
.Ltmp9180:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,8), %rax
	movq	%rax, 184(%rsp)
	movq	240(%rsp), %rax
	leaq	(%rax,%r13,4), %rcx
	movq	248(%rsp), %rax
	leaq	(%rax,%r13,4), %rdx
.Ltmp9181:
	.loc	48 568 12 is_stmt 1
	movq	%r15, %rsi
	movabsq	$2305843009213693951, %rax
	andq	%rax, %rsi
	movq	%rsi, 168(%rsp)
	xorl	%edi, %edi
	xorl	%r9d, %r9d
	movq	%rcx, 48(%rsp)
	movq	%rdx, 152(%rsp)
.Ltmp9182:
	.loc	48 0 12 is_stmt 0
.Ltmp9183:
	.p2align	4
.LBB40_381:
	movq	64(%rsp), %rax
	.loc	1 1070 28 is_stmt 1
	incq	%rax
.Ltmp9184:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
.Ltmp9185:
	.loc	48 568 12
	cmpq	184(%rsp), %rdi
	ja	.LBB40_651
.Ltmp9186:
	.loc	48 438 16
	cmpq	%r9, 168(%rsp)
	je	.LBB40_641
.Ltmp9187:
	.loc	48 0 16 is_stmt 0
	movq	64(%rsp), %rax
	leaq	(,%rax,8), %rax
.Ltmp9188:
	.loc	1 1083 29 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp9189:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_642
.Ltmp9190:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9191:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm8, 2176(%rsp)
	vmovups	(%rcx,%rdi,4), %ymm1
.Ltmp9192:
	vmovups	(%rdx,%rdi,4), %ymm4
.Ltmp9193:
	vmovaps	1280(%r12), %ymm5
	vmovaps	1312(%r12), %ymm11
	vmovaps	1344(%r12), %ymm0
	vmovaps	3904(%r12), %ymm12
	vmovaps	3936(%r12), %ymm9
	vmovaps	3968(%r12), %ymm6
	vmovaps	192(%rsp), %ymm7
	vsubps	%ymm7, %ymm1, %ymm2
	vmulps	%ymm2, %ymm11, %ymm3
	vmovaps	1600(%rsp), %ymm8
	vmovaps	%ymm5, 1824(%rsp)
	vmulps	%ymm5, %ymm8, %ymm5
	vaddps	%ymm3, %ymm5, %ymm14
	vaddps	%ymm14, %ymm8, %ymm3
	vmulps	%ymm11, %ymm8, %ymm5
	vmulps	%ymm0, %ymm2, %ymm2
	vaddps	%ymm2, %ymm5, %ymm13
	vaddps	%ymm7, %ymm13, %ymm2
	vmulps	1376(%r12), %ymm3, %ymm15
	vmovaps	1632(%rsp), %ymm3
	vsubps	%ymm3, %ymm2, %ymm5
	vmulps	1568(%rsp), %ymm11, %ymm2
	vmovaps	%ymm5, 1664(%rsp)
	vmulps	%ymm5, %ymm0, %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vaddps	%ymm2, %ymm3, %ymm7
	vmovaps	1984(%rsp), %ymm10
.Ltmp9194:
	vsubps	%ymm10, %ymm4, %ymm3
	vmulps	%ymm3, %ymm9, %ymm0
	vmovaps	2272(%rsp), %ymm8
	vmovaps	%ymm12, 1920(%rsp)
	vmulps	%ymm12, %ymm8, %ymm5
	vaddps	%ymm0, %ymm5, %ymm12
	vaddps	%ymm12, %ymm8, %ymm0
	vmulps	4000(%r12), %ymm0, %ymm0
.Ltmp9195:
	.loc	1 1083 29 is_stmt 1
	movq	2592(%r12), %rcx
.Ltmp9196:
	.loc	8 551 14
	vmovups	%ymm7, (%rcx,%rax,4)
.Ltmp9197:
	.loc	1 1084 30
	movq	2616(%r12), %rsi
.Ltmp9198:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_643
.Ltmp9199:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9200:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm1, %ymm15, %ymm1
	vsubps	%ymm7, %ymm1, %ymm1
.Ltmp9201:
	.loc	1 1084 30 is_stmt 1
	movq	2608(%r12), %rcx
.Ltmp9202:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rax,4)
.Ltmp9203:
	.loc	1 1085 28
	movq	5224(%r12), %rsi
.Ltmp9204:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_644
.Ltmp9205:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9206:
	.loc	1 0 0 is_stmt 0
	vmulps	%ymm9, %ymm8, %ymm1
	vmulps	%ymm6, %ymm3, %ymm3
	vaddps	%ymm3, %ymm1, %ymm1
	vaddps	%ymm1, %ymm10, %ymm3
	vmovaps	1760(%rsp), %ymm5
	vsubps	%ymm5, %ymm3, %ymm3
	vmulps	1728(%rsp), %ymm9, %ymm7
	vmulps	%ymm3, %ymm6, %ymm6
	vaddps	%ymm6, %ymm7, %ymm15
	vaddps	%ymm5, %ymm15, %ymm6
.Ltmp9207:
	.loc	1 1085 28 is_stmt 1
	movq	5216(%r12), %rcx
.Ltmp9208:
	.loc	8 551 14
	vmovups	%ymm6, (%rcx,%rax,4)
.Ltmp9209:
	.loc	1 1086 29
	movq	5240(%r12), %rsi
.Ltmp9210:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_645
.Ltmp9211:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9212:
	.loc	48 0 16 is_stmt 0
	movq	%r9, 2208(%rsp)
	movq	%rdi, 2368(%rsp)
	vaddps	%ymm0, %ymm4, %ymm0
	vsubps	%ymm6, %ymm0, %ymm0
.Ltmp9213:
	.loc	1 1086 29 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp9214:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp9215:
	.loc	1 1089 13
	movq	2592(%r12), %r14
	movq	2600(%r12), %rsi
	movq	2368(%r12), %rax
.Ltmp9216:
	.loc	1 0 0 is_stmt 0
	addq	64(%rsp), %rax
.Ltmp9217:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	movq	%rax, %rdi
	subq	%rcx, %rdi
.Ltmp9218:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rdi
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 2048(%rsp)
	movq	%r11, 2080(%rsp)
	je	.LBB40_397
.Ltmp9219:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rdi, %r8
	jb	.LBB40_653
.Ltmp9220:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9221:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp9222:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
.Ltmp9223:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp9224:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp9225:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9226:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r14,%rdi,4), %ymm5
.Ltmp9227:
	movq	2608(%r12), %rcx
.Ltmp9228:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm0
.Ltmp9229:
	.loc	1 961 2
	jmp	.LBB40_414
.Ltmp9230:
	.loc	1 0 2 is_stmt 0
.Ltmp9231:
	.p2align	4
.LBB40_397:
	.loc	1 955 25 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB40_666
	.loc	1 0 25 is_stmt 0
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9232:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9233:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_695
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9234:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9235:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r9
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r9
	jae	.LBB40_665
	.loc	1 0 25
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9236:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9237:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_680
	.loc	1 0 25
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9238:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9239:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_677
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9240:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9241:
	.loc	1 955 30
	leaq	5(,%rcx,8), %r10
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r10
	jae	.LBB40_671
	.loc	1 0 25
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9242:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9243:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 1888(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_674
	.loc	1 0 25
	movq	2424(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9244:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9245:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_687
.Ltmp9246:
	.loc	1 0 25
	movq	%rcx, 1856(%rsp)
.Ltmp9247:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
.Ltmp9248:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp9249:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp9250:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9251:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9252:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 24(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_686
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9253:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9254:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
	.loc	1 0 25
	movq	%r14, 16(%rsp)
	movq	2392(%r12), %rdx
	.loc	1 955 35
	addq	64(%rsp), %rdx
.Ltmp9255:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp9256:
	.loc	1 955 30
	leaq	3(,%rdx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_690
	.loc	1 0 25
	movq	%r10, 40(%rsp)
	movq	2400(%r12), %rdx
	.loc	1 955 35
	addq	64(%rsp), %rdx
.Ltmp9257:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp9258:
	.loc	1 955 30
	leaq	4(,%rdx,8), %r10
	movq	%r10, 32(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r10
	jae	.LBB40_691
	.loc	1 0 25
	movq	%r14, 160(%rsp)
	movq	%rcx, 56(%rsp)
	movq	2408(%r12), %rdx
	movq	64(%rsp), %r10
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp9259:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp9260:
	.loc	1 955 30
	leaq	5(,%rdx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_689
	.loc	1 0 25
	movq	%r15, 1560(%rsp)
	movq	2416(%r12), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp9261:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r11
	movq	%r12, %r15
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %r11
.Ltmp9262:
	.loc	1 955 30
	leaq	6(,%r11,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	%r8, %rcx
	movq	2424(%r15), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp9263:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r11
	movl	$0, %r8d
	cmovaeq	%rbx, %r8
	subq	%r8, %r11
.Ltmp9264:
	.loc	1 955 30
	leaq	7(,%r11,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_682
.Ltmp9265:
	.loc	1 0 25
	movq	%rdi, %r14
	movq	16(%rsp), %rdi
	vmovd	(%rdi,%r13,4), %xmm0
	movq	40(%rsp), %rsi
	vpinsrd	$1, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	1888(%rsp), %rsi
	vpinsrd	$2, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	1856(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm0, %xmm0
	vmovd	(%rdi,%r14,4), %xmm4
	vpinsrd	$1, (%rdi,%rcx,4), %xmm4, %xmm4
	vpinsrd	$2, (%rdi,%r9,4), %xmm4, %xmm4
	movq	1560(%rsp), %rcx
	vpinsrd	$3, (%rdi,%rcx,4), %xmm4, %xmm4
.Ltmp9266:
	movq	2608(%r15), %rsi
	movq	32(%rsp), %rcx
.Ltmp9267:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%rcx,4), %xmm6
	vpinsrd	$1, (%rsi,%rdx,4), %xmm6, %xmm6
	vpinsrd	$2, (%rsi,%r12,4), %xmm6, %xmm6
	vpinsrd	$3, (%rsi,%r11,4), %xmm6, %xmm6
	vmovd	(%rsi,%rax,4), %xmm7
	movq	24(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm7, %xmm7
	movq	56(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm7, %xmm7
	movq	160(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm7, %xmm7
.Ltmp9268:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm4, %ymm5
.Ltmp9269:
	.loc	8 551 14
	vinserti128	$1, %xmm6, %ymm7, %ymm0
	movq	%r15, %r12
	movq	2144(%rsp), %r15
	movq	2080(%rsp), %r11
.Ltmp9270:
.LBB40_414:
	.loc	1 1103 13 is_stmt 1
	movq	5216(%r12), %r9
	movq	5224(%r12), %rsi
	movq	4992(%r12), %rax
.Ltmp9271:
	.loc	1 0 0 is_stmt 0
	addq	64(%rsp), %rax
.Ltmp9272:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	movq	%rax, %rdi
	subq	%rcx, %rdi
.Ltmp9273:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rdi
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 176(%rsp)
	vmovdqa	%ymm0, 1888(%rsp)
	je	.LBB40_420
.Ltmp9274:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rdi, %r8
	jb	.LBB40_653
.Ltmp9275:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9276:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp9277:
	.loc	1 857 8
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
.Ltmp9278:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp9279:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp9280:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9281:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r9,%rdi,4), %ymm0
.Ltmp9282:
	movq	5232(%r12), %rcx
.Ltmp9283:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm4
.Ltmp9284:
	.loc	1 961 2
	jmp	.LBB40_437
.Ltmp9285:
	.loc	1 0 2 is_stmt 0
.Ltmp9286:
	.p2align	4
.LBB40_420:
	.loc	1 955 25 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB40_666
	.loc	1 0 25 is_stmt 0
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9287:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9288:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_697
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9289:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9290:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_703
	.loc	1 0 25
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9291:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9292:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r13
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r13
	jae	.LBB40_704
	.loc	1 0 25
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9293:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9294:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r10
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r10
	jae	.LBB40_731
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9295:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9296:
	.loc	1 955 30
	leaq	5(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_699
	.loc	1 0 25
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9297:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9298:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 1856(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_715
	.loc	1 0 25
	movq	5048(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9299:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9300:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_706
.Ltmp9301:
	.loc	1 0 25
	movq	%rcx, 24(%rsp)
.Ltmp9302:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
.Ltmp9303:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp9304:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp9305:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9306:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9307:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_702
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	64(%rsp), %rcx
.Ltmp9308:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp9309:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
	.loc	1 0 25
	movq	%r14, 40(%rsp)
	movq	5016(%r12), %rdx
	.loc	1 955 35
	addq	64(%rsp), %rdx
.Ltmp9310:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp9311:
	.loc	1 955 30
	leaq	3(,%rdx,8), %r11
	movq	%r11, 32(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_712
	.loc	1 0 25
	movq	%rcx, 56(%rsp)
	movq	5024(%r12), %rdx
	.loc	1 955 35
	addq	64(%rsp), %rdx
.Ltmp9312:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp9313:
	.loc	1 955 30
	leaq	4(,%rdx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_734
	.loc	1 0 25
	movq	%r13, 1560(%rsp)
	movq	%r15, 160(%rsp)
	movq	5032(%r12), %rdx
	.loc	1 955 35
	addq	64(%rsp), %rdx
.Ltmp9314:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rdx
	movl	$0, %r11d
	cmovaeq	%rbx, %r11
	subq	%r11, %rdx
.Ltmp9315:
	.loc	1 955 30
	leaq	5(,%rdx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_710
	.loc	1 0 25
	movq	5040(%r12), %r11
	.loc	1 955 35
	addq	64(%rsp), %r11
.Ltmp9316:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r11
	movq	%r12, %r15
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %r11
.Ltmp9317:
	.loc	1 955 30
	leaq	6(,%r11,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	%rdi, %r13
	movq	5048(%r15), %r11
	movq	64(%rsp), %rdi
	.loc	1 955 35
	addq	%rdi, %r11
.Ltmp9318:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %r11
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %r11
.Ltmp9319:
	.loc	1 955 30
	leaq	7(,%r11,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_746
.Ltmp9320:
	.loc	1 0 25
	movq	%r8, %rcx
	movq	%r9, %r8
	vmovd	(%r8,%r10,4), %xmm0
	movq	40(%rsp), %rsi
	vpinsrd	$1, (%r8,%rsi,4), %xmm0, %xmm0
	movq	1856(%rsp), %rsi
	vpinsrd	$2, (%r8,%rsi,4), %xmm0, %xmm0
	movq	24(%rsp), %rsi
	vpinsrd	$3, (%r8,%rsi,4), %xmm0, %xmm0
	vmovd	(%r8,%r13,4), %xmm4
	vpinsrd	$1, (%r8,%rcx,4), %xmm4, %xmm4
	movq	160(%rsp), %rcx
	vpinsrd	$2, (%r8,%rcx,4), %xmm4, %xmm4
	movq	1560(%rsp), %rcx
	vpinsrd	$3, (%r8,%rcx,4), %xmm4, %xmm4
.Ltmp9321:
	movq	5232(%r15), %rsi
.Ltmp9322:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm6
	vpinsrd	$1, (%rsi,%rdx,4), %xmm6, %xmm6
	vpinsrd	$2, (%rsi,%r12,4), %xmm6, %xmm6
	vpinsrd	$3, (%rsi,%r11,4), %xmm6, %xmm6
	vmovd	(%rsi,%rax,4), %xmm7
	movq	16(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm7, %xmm7
	movq	56(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm7, %xmm7
	movq	32(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm7, %xmm7
.Ltmp9323:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm4, %ymm0
.Ltmp9324:
	.loc	8 551 14
	vinserti128	$1, %xmm6, %ymm7, %ymm4
	movq	%r15, %r12
	movq	2144(%rsp), %r15
	movq	2080(%rsp), %r11
.Ltmp9325:
.LBB40_437:
	.loc	1 0 0
	negq	%r11
	movq	64(%rsp), %rax
	addq	%r11, %rax
	incq	%rax
	movq	%rax, 64(%rsp)
	leaq	(,%rax,8), %rax
.Ltmp9326:
	.loc	1 1150 36 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp9327:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	152(%rsp), %rdx
	movq	2368(%rsp), %rdi
	movq	2208(%rsp), %r9
	jb	.LBB40_647
.Ltmp9328:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9329:
	.loc	1 1152 27
	movq	2616(%r12), %rsi
.Ltmp9330:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_648
.Ltmp9331:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9332:
	.loc	1 1153 35
	movq	5224(%r12), %rsi
.Ltmp9333:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_649
.Ltmp9334:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9335:
	.loc	1 1155 27
	movq	5240(%r12), %rsi
.Ltmp9336:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_638
.Ltmp9337:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp9338:
	.loc	48 0 16 is_stmt 0
	vmovdqa	%ymm0, 2080(%rsp)
	vaddps	%ymm14, %ymm14, %ymm0
	vaddps	1600(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_1(%rip), %ymm14
	vandps	%ymm0, %ymm14, %ymm7
	vmovdqa	%ymm4, 1856(%rsp)
	vmovdqa	%ymm5, %ymm4
	vmovaps	%ymm10, %ymm5
	vmovaps	%ymm8, %ymm10
	vbroadcastss	.LCPI40_2(%rip), %ymm8
	vmovaps	%ymm1, %ymm6
	vmovaps	%ymm8, %ymm1
	vcmplt_oqps	%ymm8, %ymm7, %ymm7
	vandnps	%ymm0, %ymm7, %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vaddps	%ymm13, %ymm13, %ymm0
	vaddps	192(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm7
	vcmplt_oqps	%ymm8, %ymm7, %ymm7
	vandnps	%ymm0, %ymm7, %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmulps	1664(%rsp), %ymm11, %ymm0
	vmovaps	1568(%rsp), %ymm8
	vmulps	1824(%rsp), %ymm8, %ymm7
	vaddps	%ymm0, %ymm7, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	%ymm0, %ymm8, %ymm0
	vandps	%ymm0, %ymm14, %ymm7
	vcmplt_oqps	%ymm1, %ymm7, %ymm7
	vandnps	%ymm0, %ymm7, %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vaddps	%ymm2, %ymm2, %ymm0
	vaddps	1632(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm2
	vcmplt_oqps	%ymm1, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 1632(%rsp)
.Ltmp9339:
	vaddps	%ymm12, %ymm12, %ymm0
	vaddps	%ymm0, %ymm10, %ymm0
	vandps	%ymm0, %ymm14, %ymm2
	vcmplt_oqps	%ymm1, %ymm2, %ymm2
	vmovaps	%ymm1, %ymm8
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vaddps	%ymm6, %ymm6, %ymm0
	vaddps	%ymm0, %ymm5, %ymm0
	vandps	%ymm0, %ymm14, %ymm1
	vcmplt_oqps	%ymm8, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmulps	%ymm3, %ymm9, %ymm0
	vmovaps	1728(%rsp), %ymm2
	vmulps	1920(%rsp), %ymm2, %ymm1
	vaddps	%ymm0, %ymm1, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	%ymm0, %ymm2, %ymm0
	vandps	%ymm0, %ymm14, %ymm1
	vcmplt_oqps	%ymm8, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vaddps	%ymm15, %ymm15, %ymm0
	vaddps	1760(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm1
	vcmplt_oqps	%ymm8, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 1760(%rsp)
.Ltmp9340:
	vpand	%ymm4, %ymm14, %ymm0
	vandps	2080(%rsp), %ymm14, %ymm1
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp9341:
	vandps	1888(%rsp), %ymm14, %ymm1
	vandps	1856(%rsp), %ymm14, %ymm2
	vmaxps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_4(%rip), %ymm9
.Ltmp9342:
	vmaxps	%ymm9, %ymm0, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm3
	vbroadcastss	.LCPI40_5(%rip), %ymm10
	vmaxps	%ymm10, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm1
	vbroadcastsd	.LCPI40_7(%rip), %ymm4
	vorps	%ymm4, %ymm1, %ymm1
	vbroadcastss	.LCPI40_8(%rip), %ymm11
	vaddps	%ymm1, %ymm11, %ymm1
	vbroadcastss	.LCPI40_9(%rip), %ymm12
	vmulps	%ymm1, %ymm12, %ymm5
	vbroadcastss	.LCPI40_10(%rip), %ymm13
	vaddps	%ymm5, %ymm13, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_11(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
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
	vbroadcastss	.LCPI40_17(%rip), %ymm12
	vmulps	%ymm0, %ymm12, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm13
	vmaxps	%ymm13, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm15
	vminps	%ymm15, %ymm0, %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vsubps	256(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_20(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm6
	vmulps	%ymm6, %ymm6, %ymm6
	vbroadcastss	.LCPI40_22(%rip), %ymm12
	vmulps	%ymm6, %ymm12, %ymm6
	vcmpgt_oqps	%ymm1, %ymm0, %ymm7
	vblendvps	%ymm7, %ymm0, %ymm6, %ymm6
	vbroadcastss	.LCPI40_21(%rip), %ymm11
	vcmple_oqps	%ymm11, %ymm0, %ymm0
	vmulps	2336(%rsp), %ymm6, %ymm6
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm0, %ymm1, %ymm0
	vpandn	%ymm6, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm6
	vmaxps	%ymm6, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	2240(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm6
	vmovaps	2560(%rsp), %ymm7
	vblendvps	%ymm6, 2592(%rsp), %ymm7, %ymm6
	vsubps	%ymm0, %ymm1, %ymm7
	vmulps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm6
	vcmplt_oqps	%ymm8, %ymm6, %ymm6
	vandnps	%ymm0, %ymm6, %ymm6
.Ltmp9343:
	vbroadcastss	.LCPI40_4(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm0
	vmaxps	%ymm10, %ymm0, %ymm0
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
	vbroadcastss	.LCPI40_17(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vmaxps	%ymm13, %ymm0, %ymm0
	vminps	%ymm15, %ymm0, %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vsubps	416(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_20(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm2
	vmulps	%ymm2, %ymm2, %ymm2
	vmulps	%ymm2, %ymm12, %ymm2
	vcmpgt_oqps	%ymm1, %ymm0, %ymm4
	vblendvps	%ymm4, %ymm0, %ymm2, %ymm2
	vcmple_oqps	%ymm11, %ymm0, %ymm0
	vmulps	2528(%rsp), %ymm2, %ymm2
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm0, %ymm1, %ymm0
	vpandn	%ymm2, %ymm0, %ymm0
	vmovaps	%ymm6, 2240(%rsp)
.Ltmp9344:
	vaddps	384(%rsp), %ymm6, %ymm2
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
	vbroadcastss	.LCPI40_28(%rip), %ymm13
	vaddps	%ymm5, %ymm13, %ymm5
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
.Ltmp9345:
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	2112(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm6
	vmovaps	2464(%rsp), %ymm7
	vblendvps	%ymm6, 2496(%rsp), %ymm7, %ymm6
.Ltmp9346:
	vmulps	%ymm5, %ymm2, %ymm2
.Ltmp9347:
	vsubps	%ymm0, %ymm1, %ymm5
	vmulps	%ymm6, %ymm5, %ymm5
	vaddps	%ymm5, %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm11
	vcmplt_oqps	%ymm11, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm1
	vbroadcastss	.LCPI40_32(%rip), %ymm6
.Ltmp9348:
	vaddps	%ymm6, %ymm2, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm7
	vaddps	%ymm7, %ymm4, %ymm2
	vpslld	$23, %ymm2, %ymm2
	vmovaps	%ymm1, 2112(%rsp)
.Ltmp9349:
	vaddps	544(%rsp), %ymm1, %ymm4
.Ltmp9350:
	vmulps	%ymm2, %ymm0, %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vbroadcastss	.LCPI40_24(%rip), %ymm2
.Ltmp9351:
	vmulps	%ymm2, %ymm4, %ymm0
	vmaxps	%ymm8, %ymm0, %ymm0
	vmovaps	%ymm8, %ymm13
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
	vmovaps	2080(%rsp), %ymm1
.Ltmp9352:
	vsubps	896(%rsp), %ymm1, %ymm5
.Ltmp9353:
	vmulps	%ymm4, %ymm0, %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm6
.Ltmp9354:
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
	vmovaps	1952(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm4
	vmovaps	2720(%rsp), %ymm5
	vblendvps	%ymm4, 2400(%rsp), %ymm5, %ymm4
	vsubps	%ymm0, %ymm1, %ymm5
	vmulps	%ymm4, %ymm5, %ymm4
	vaddps	%ymm4, %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm4
	vcmplt_oqps	%ymm11, %ymm4, %ymm4
	vandnps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vaddps	1024(%rsp), %ymm0, %ymm0
	vmulps	%ymm2, %ymm0, %ymm0
	vmaxps	%ymm13, %ymm0, %ymm0
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
	vmovaps	1824(%rsp), %ymm0
.Ltmp9355:
	vsubps	1056(%rsp), %ymm0, %ymm0
	vaddps	%ymm6, %ymm0, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm9, %ymm3
	vcmpgt_oqps	%ymm6, %ymm0, %ymm5
	vblendvps	%ymm5, %ymm0, %ymm3, %ymm3
	vcmple_oqps	%ymm10, %ymm0, %ymm0
	vmulps	2688(%rsp), %ymm3, %ymm3
	vpcmpgtd	%ymm0, %ymm7, %ymm0
	vpandn	%ymm3, %ymm0, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm7, %ymm0, %ymm0
	vmovaps	2176(%rsp), %ymm5
	vcmplt_oqps	%ymm5, %ymm0, %ymm3
	vmovaps	2624(%rsp), %ymm1
	vblendvps	%ymm3, 2656(%rsp), %ymm1, %ymm3
	vsubps	%ymm0, %ymm5, %ymm5
	vmulps	%ymm3, %ymm5, %ymm3
	vaddps	%ymm3, %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm3
	vbroadcastss	.LCPI40_2(%rip), %ymm1
	vcmplt_oqps	%ymm1, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm8
	vaddps	1184(%rsp), %ymm8, %ymm0
	vmulps	%ymm2, %ymm0, %ymm0
	vmaxps	%ymm13, %ymm0, %ymm0
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
.Ltmp9356:
	movq	2592(%r12), %rcx
	vmovaps	1920(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm2
	movq	2608(%r12), %rcx
	vmovaps	2080(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm1
	vaddps	%ymm1, %ymm2, %ymm1
.Ltmp9357:
	movq	5216(%r12), %rcx
	vmulps	(%rcx,%rax,4), %ymm4, %ymm2
	.loc	1 1155 27 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp9358:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
.Ltmp9359:
	.loc	14 48 14
	vaddps	%ymm0, %ymm2, %ymm0
	movq	48(%rsp), %rcx
.Ltmp9360:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rdi,4)
.Ltmp9361:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm0, (%rdx,%rdi,4)
.Ltmp9362:
	.loc	1 0 0
	incq	%r9
.Ltmp9363:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r9, %r15
.Ltmp9364:
	.loc	3 900 12
	jne	.LBB40_381
	jmp	.LBB40_299
.Ltmp9365:
.LBB40_446:
	.loc	3 0 12 is_stmt 0
	movq	144(%rsp), %rsi
.Ltmp9366:
	.loc	1 1189 11 is_stmt 1
	testq	%rsi, %rsi
	je	.LBB40_595
	.loc	1 0 11 is_stmt 0
	movl	5280(%r12), %eax
	movl	%eax, 1820(%rsp)
	movq	5264(%r12), %r13
	leaq	2624(%r12), %rax
	movq	%rax, 2328(%rsp)
	xorl	%eax, %eax
	movq	%r13, 1888(%rsp)
	jmp	.LBB40_450
.LBB40_448:
	vmovaps	1728(%rsp), %ymm0
.Ltmp9367:
	.loc	1 1160 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r12)
	vmovaps	1568(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r12)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r12)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r12)
	vmovaps	64(%rsp), %ymm0
	.loc	1 1161 5
	vmovaps	%ymm0, 4032(%r12)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r12)
	vmovaps	1760(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r12)
	vmovaps	2144(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r12)
	vmovaps	2112(%rsp), %ymm0
	.loc	1 1162 5
	vmovaps	%ymm0, 2304(%r12)
	vmovaps	1952(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r12)
	vmovdqa	1984(%rsp), %ymm0
	.loc	1 1163 5
	vmovdqa	%ymm0, 4928(%r12)
	vmovaps	%ymm8, 4960(%r12)
	.loc	1 1164 5
	movq	%r10, 5272(%r12)
.Ltmp9368:
.LBB40_449:
	.loc	1 0 5 is_stmt 0
	movq	144(%rsp), %rsi
	movq	2040(%rsp), %rax
	.loc	1 1189 11 is_stmt 1
	cmpq	%rsi, %rax
	jae	.LBB40_595
.LBB40_450:
	.loc	1 0 11 is_stmt 0
	movq	%rax, %r13
	.loc	1 1190 42 is_stmt 1
	subq	%rax, %rsi
	.loc	1 1190 29 is_stmt 0
	movq	%r12, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E12plan_segmentB5_
	movb	%dl, 64(%rsp)
	movq	%rax, %r15
.Ltmp9369:
	.loc	1 1279 33 is_stmt 1
	vmovss	(%r12), %xmm0
.Ltmp9370:
	.loc	1 1192 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp9371:
	.loc	1 1279 33
	vmovss	160(%r12), %xmm0
.Ltmp9372:
	.loc	1 1192 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp9373:
	.loc	1 1279 33
	vmovss	320(%r12), %xmm0
.Ltmp9374:
	.loc	1 1192 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp9375:
	.loc	1 1279 33
	vmovss	480(%r12), %xmm0
.Ltmp9376:
	.loc	1 1192 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp9377:
	.loc	1 1279 33
	vmovss	640(%r12), %xmm0
.Ltmp9378:
	.loc	1 1192 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp9379:
	.loc	1 1279 33
	vmovss	800(%r12), %xmm0
.Ltmp9380:
	.loc	1 1192 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp9381:
	.loc	1 1279 33
	vmovss	960(%r12), %xmm0
.Ltmp9382:
	.loc	1 1192 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp9383:
	.loc	1 1279 33
	vmovss	1120(%r12), %xmm0
.Ltmp9384:
	.loc	1 1192 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp9385:
	.loc	1 1279 33
	vmovss	16(%r12), %xmm0
.Ltmp9386:
	.loc	1 1192 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp9387:
	.loc	1 1279 33
	vmovss	176(%r12), %xmm0
.Ltmp9388:
	.loc	1 1192 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp9389:
	.loc	1 1279 33
	vmovss	336(%r12), %xmm0
.Ltmp9390:
	.loc	1 1192 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp9391:
	.loc	1 1279 33
	vmovss	496(%r12), %xmm0
.Ltmp9392:
	.loc	1 1192 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp9393:
	.loc	1 1279 33
	vmovss	656(%r12), %xmm0
.Ltmp9394:
	.loc	1 1192 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp9395:
	.loc	1 1279 33
	vmovss	816(%r12), %xmm0
.Ltmp9396:
	.loc	1 1192 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp9397:
	.loc	1 1279 33
	vmovss	976(%r12), %xmm0
.Ltmp9398:
	.loc	1 1192 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp9399:
	.loc	1 1279 33
	vmovss	1136(%r12), %xmm0
.Ltmp9400:
	.loc	1 1192 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp9401:
	.loc	1 1279 33
	vmovss	32(%r12), %xmm0
.Ltmp9402:
	.loc	1 1192 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp9403:
	.loc	1 1279 33
	vmovss	192(%r12), %xmm0
.Ltmp9404:
	.loc	1 1192 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp9405:
	.loc	1 1279 33
	vmovss	352(%r12), %xmm0
.Ltmp9406:
	.loc	1 1192 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp9407:
	.loc	1 1279 33
	vmovss	512(%r12), %xmm0
.Ltmp9408:
	.loc	1 1192 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp9409:
	.loc	1 1279 33
	vmovss	672(%r12), %xmm0
.Ltmp9410:
	.loc	1 1192 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp9411:
	.loc	1 1279 33
	vmovss	832(%r12), %xmm0
.Ltmp9412:
	.loc	1 1192 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp9413:
	.loc	1 1279 33
	vmovss	992(%r12), %xmm0
.Ltmp9414:
	.loc	1 1192 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp9415:
	.loc	1 1279 33
	vmovss	1152(%r12), %xmm0
.Ltmp9416:
	.loc	1 1192 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp9417:
	.loc	1 1279 33
	vmovss	48(%r12), %xmm0
.Ltmp9418:
	.loc	1 1192 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp9419:
	.loc	1 1279 33
	vmovss	208(%r12), %xmm0
.Ltmp9420:
	.loc	1 1192 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp9421:
	.loc	1 1279 33
	vmovss	368(%r12), %xmm0
.Ltmp9422:
	.loc	1 1192 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp9423:
	.loc	1 1279 33
	vmovss	528(%r12), %xmm0
.Ltmp9424:
	.loc	1 1192 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp9425:
	.loc	1 1279 33
	vmovss	688(%r12), %xmm0
.Ltmp9426:
	.loc	1 1192 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp9427:
	.loc	1 1279 33
	vmovss	848(%r12), %xmm0
.Ltmp9428:
	.loc	1 1192 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp9429:
	.loc	1 1279 33
	vmovss	1008(%r12), %xmm0
.Ltmp9430:
	.loc	1 1192 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp9431:
	.loc	1 1279 33
	vmovss	1168(%r12), %xmm0
.Ltmp9432:
	.loc	1 1192 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp9433:
	.loc	1 1279 33
	vmovss	64(%r12), %xmm0
.Ltmp9434:
	.loc	1 1192 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp9435:
	.loc	1 1279 33
	vmovss	224(%r12), %xmm0
.Ltmp9436:
	.loc	1 1192 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp9437:
	.loc	1 1279 33
	vmovss	384(%r12), %xmm0
.Ltmp9438:
	.loc	1 1192 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp9439:
	.loc	1 1279 33
	vmovss	544(%r12), %xmm0
.Ltmp9440:
	.loc	1 1192 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp9441:
	.loc	1 1279 33
	vmovss	704(%r12), %xmm0
.Ltmp9442:
	.loc	1 1192 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp9443:
	.loc	1 1279 33
	vmovss	864(%r12), %xmm0
.Ltmp9444:
	.loc	1 1192 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp9445:
	.loc	1 1279 33
	vmovss	1024(%r12), %xmm0
.Ltmp9446:
	.loc	1 1192 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp9447:
	.loc	1 1279 33
	vmovss	1184(%r12), %xmm0
.Ltmp9448:
	.loc	1 1192 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp9449:
	.loc	1 1279 33
	vmovss	80(%r12), %xmm0
.Ltmp9450:
	.loc	1 1192 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp9451:
	.loc	1 1279 33
	vmovss	240(%r12), %xmm0
.Ltmp9452:
	.loc	1 1192 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp9453:
	.loc	1 1279 33
	vmovss	400(%r12), %xmm0
.Ltmp9454:
	.loc	1 1192 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp9455:
	.loc	1 1279 33
	vmovss	560(%r12), %xmm0
.Ltmp9456:
	.loc	1 1192 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp9457:
	.loc	1 1279 33
	vmovss	720(%r12), %xmm0
.Ltmp9458:
	.loc	1 1192 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp9459:
	.loc	1 1279 33
	vmovss	880(%r12), %xmm0
.Ltmp9460:
	.loc	1 1192 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp9461:
	.loc	1 1279 33
	vmovss	1040(%r12), %xmm0
.Ltmp9462:
	.loc	1 1192 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp9463:
	.loc	1 1279 33
	vmovss	1200(%r12), %xmm0
.Ltmp9464:
	.loc	1 1192 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp9465:
	.loc	1 1279 33
	vmovss	96(%r12), %xmm0
.Ltmp9466:
	.loc	1 1192 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp9467:
	.loc	1 1279 33
	vmovss	256(%r12), %xmm0
.Ltmp9468:
	.loc	1 1192 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp9469:
	.loc	1 1279 33
	vmovss	416(%r12), %xmm0
.Ltmp9470:
	.loc	1 1192 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp9471:
	.loc	1 1279 33
	vmovss	576(%r12), %xmm0
.Ltmp9472:
	.loc	1 1192 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp9473:
	.loc	1 1279 33
	vmovss	736(%r12), %xmm0
.Ltmp9474:
	.loc	1 1192 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp9475:
	.loc	1 1279 33
	vmovss	896(%r12), %xmm0
.Ltmp9476:
	.loc	1 1192 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp9477:
	.loc	1 1279 33
	vmovss	1056(%r12), %xmm0
.Ltmp9478:
	.loc	1 1192 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp9479:
	.loc	1 1279 33
	vmovss	1216(%r12), %xmm0
.Ltmp9480:
	.loc	1 1192 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp9481:
	.loc	1 1279 33
	vmovss	112(%r12), %xmm0
.Ltmp9482:
	.loc	1 1192 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp9483:
	.loc	1 1279 33
	vmovss	272(%r12), %xmm0
.Ltmp9484:
	.loc	1 1192 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp9485:
	.loc	1 1279 33
	vmovss	432(%r12), %xmm0
.Ltmp9486:
	.loc	1 1192 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp9487:
	.loc	1 1279 33
	vmovss	592(%r12), %xmm0
.Ltmp9488:
	.loc	1 1192 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp9489:
	.loc	1 1279 33
	vmovss	752(%r12), %xmm0
.Ltmp9490:
	.loc	1 1192 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp9491:
	.loc	1 1279 33
	vmovss	912(%r12), %xmm0
.Ltmp9492:
	.loc	1 1192 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp9493:
	.loc	1 1279 33
	vmovss	1072(%r12), %xmm0
.Ltmp9494:
	.loc	1 1192 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp9495:
	.loc	1 1279 33
	vmovss	1232(%r12), %xmm0
.Ltmp9496:
	.loc	1 1192 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp9497:
	.loc	1 1279 33
	vmovss	128(%r12), %xmm0
.Ltmp9498:
	.loc	1 1192 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp9499:
	.loc	1 1279 33
	vmovss	288(%r12), %xmm0
.Ltmp9500:
	.loc	1 1192 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp9501:
	.loc	1 1279 33
	vmovss	448(%r12), %xmm0
.Ltmp9502:
	.loc	1 1192 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp9503:
	.loc	1 1279 33
	vmovss	608(%r12), %xmm0
.Ltmp9504:
	.loc	1 1192 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp9505:
	.loc	1 1279 33
	vmovss	768(%r12), %xmm0
.Ltmp9506:
	.loc	1 1192 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp9507:
	.loc	1 1279 33
	vmovss	928(%r12), %xmm0
.Ltmp9508:
	.loc	1 1192 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp9509:
	.loc	1 1279 33
	vmovss	1088(%r12), %xmm0
.Ltmp9510:
	.loc	1 1192 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp9511:
	.loc	1 1279 33
	vmovss	1248(%r12), %xmm0
.Ltmp9512:
	.loc	1 1192 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp9513:
	.loc	1 1279 33
	vmovss	144(%r12), %xmm0
.Ltmp9514:
	.loc	1 1192 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp9515:
	.loc	1 1279 33
	vmovss	304(%r12), %xmm0
.Ltmp9516:
	.loc	1 1192 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp9517:
	.loc	1 1279 33
	vmovss	464(%r12), %xmm0
.Ltmp9518:
	.loc	1 1192 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp9519:
	.loc	1 1279 33
	vmovss	624(%r12), %xmm0
.Ltmp9520:
	.loc	1 1192 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp9521:
	.loc	1 1279 33
	vmovss	784(%r12), %xmm0
.Ltmp9522:
	.loc	1 1192 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp9523:
	.loc	1 1279 33
	vmovss	944(%r12), %xmm0
.Ltmp9524:
	.loc	1 1192 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp9525:
	.loc	1 1279 33
	vmovss	1104(%r12), %xmm0
.Ltmp9526:
	.loc	1 1192 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp9527:
	.loc	1 1279 33
	vmovss	1264(%r12), %xmm0
.Ltmp9528:
	.loc	1 1192 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp9529:
	.loc	1 1280 32
	vmovss	8(%r12), %xmm0
.Ltmp9530:
	.loc	1 1192 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp9531:
	.loc	1 1280 32
	vmovss	168(%r12), %xmm0
.Ltmp9532:
	.loc	1 1192 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp9533:
	.loc	1 1280 32
	vmovss	328(%r12), %xmm0
.Ltmp9534:
	.loc	1 1192 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp9535:
	.loc	1 1280 32
	vmovss	488(%r12), %xmm0
.Ltmp9536:
	.loc	1 1192 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp9537:
	.loc	1 1280 32
	vmovss	648(%r12), %xmm0
.Ltmp9538:
	.loc	1 1192 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp9539:
	.loc	1 1280 32
	vmovss	808(%r12), %xmm0
.Ltmp9540:
	.loc	1 1192 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp9541:
	.loc	1 1280 32
	vmovss	968(%r12), %xmm0
.Ltmp9542:
	.loc	1 1192 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp9543:
	.loc	1 1280 32
	vmovss	1128(%r12), %xmm0
.Ltmp9544:
	.loc	1 1192 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp9545:
	.loc	1 1280 32
	vmovss	24(%r12), %xmm0
.Ltmp9546:
	.loc	1 1192 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp9547:
	.loc	1 1280 32
	vmovss	184(%r12), %xmm0
.Ltmp9548:
	.loc	1 1192 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp9549:
	.loc	1 1280 32
	vmovss	344(%r12), %xmm0
.Ltmp9550:
	.loc	1 1192 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp9551:
	.loc	1 1280 32
	vmovss	504(%r12), %xmm0
.Ltmp9552:
	.loc	1 1192 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp9553:
	.loc	1 1280 32
	vmovss	664(%r12), %xmm0
.Ltmp9554:
	.loc	1 1192 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp9555:
	.loc	1 1280 32
	vmovss	824(%r12), %xmm0
.Ltmp9556:
	.loc	1 1192 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp9557:
	.loc	1 1280 32
	vmovss	984(%r12), %xmm0
.Ltmp9558:
	.loc	1 1192 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp9559:
	.loc	1 1280 32
	vmovss	1144(%r12), %xmm0
.Ltmp9560:
	.loc	1 1192 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp9561:
	.loc	1 1280 32
	vmovss	40(%r12), %xmm0
.Ltmp9562:
	.loc	1 1192 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp9563:
	.loc	1 1280 32
	vmovss	200(%r12), %xmm0
.Ltmp9564:
	.loc	1 1192 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp9565:
	.loc	1 1280 32
	vmovss	360(%r12), %xmm0
.Ltmp9566:
	.loc	1 1192 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp9567:
	.loc	1 1280 32
	vmovss	520(%r12), %xmm0
.Ltmp9568:
	.loc	1 1192 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp9569:
	.loc	1 1280 32
	vmovss	680(%r12), %xmm0
.Ltmp9570:
	.loc	1 1192 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp9571:
	.loc	1 1280 32
	vmovss	840(%r12), %xmm0
.Ltmp9572:
	.loc	1 1192 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp9573:
	.loc	1 1280 32
	vmovss	1000(%r12), %xmm0
.Ltmp9574:
	.loc	1 1192 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp9575:
	.loc	1 1280 32
	vmovss	1160(%r12), %xmm0
.Ltmp9576:
	.loc	1 1192 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp9577:
	.loc	1 1280 32
	vmovss	56(%r12), %xmm0
.Ltmp9578:
	.loc	1 1192 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp9579:
	.loc	1 1280 32
	vmovss	216(%r12), %xmm0
.Ltmp9580:
	.loc	1 1192 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp9581:
	.loc	1 1280 32
	vmovss	376(%r12), %xmm0
.Ltmp9582:
	.loc	1 1192 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp9583:
	.loc	1 1280 32
	vmovss	536(%r12), %xmm0
.Ltmp9584:
	.loc	1 1192 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp9585:
	.loc	1 1280 32
	vmovss	696(%r12), %xmm0
.Ltmp9586:
	.loc	1 1192 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp9587:
	.loc	1 1280 32
	vmovss	856(%r12), %xmm0
.Ltmp9588:
	.loc	1 1192 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp9589:
	.loc	1 1280 32
	vmovss	1016(%r12), %xmm0
.Ltmp9590:
	.loc	1 1192 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp9591:
	.loc	1 1280 32
	vmovss	1176(%r12), %xmm0
.Ltmp9592:
	.loc	1 1192 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp9593:
	.loc	1 1280 32
	vmovss	72(%r12), %xmm0
.Ltmp9594:
	.loc	1 1192 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp9595:
	.loc	1 1280 32
	vmovss	232(%r12), %xmm0
.Ltmp9596:
	.loc	1 1192 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp9597:
	.loc	1 1280 32
	vmovss	392(%r12), %xmm0
.Ltmp9598:
	.loc	1 1192 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp9599:
	.loc	1 1280 32
	vmovss	552(%r12), %xmm0
.Ltmp9600:
	.loc	1 1192 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp9601:
	.loc	1 1280 32
	vmovss	712(%r12), %xmm0
.Ltmp9602:
	.loc	1 1192 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp9603:
	.loc	1 1280 32
	vmovss	872(%r12), %xmm0
.Ltmp9604:
	.loc	1 1192 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp9605:
	.loc	1 1280 32
	vmovss	1032(%r12), %xmm0
.Ltmp9606:
	.loc	1 1192 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp9607:
	.loc	1 1280 32
	vmovss	1192(%r12), %xmm0
.Ltmp9608:
	.loc	1 1192 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp9609:
	.loc	1 1280 32
	vmovss	88(%r12), %xmm0
.Ltmp9610:
	.loc	1 1192 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp9611:
	.loc	1 1280 32
	vmovss	248(%r12), %xmm0
.Ltmp9612:
	.loc	1 1192 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp9613:
	.loc	1 1280 32
	vmovss	408(%r12), %xmm0
.Ltmp9614:
	.loc	1 1192 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp9615:
	.loc	1 1280 32
	vmovss	568(%r12), %xmm0
.Ltmp9616:
	.loc	1 1192 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp9617:
	.loc	1 1280 32
	vmovss	728(%r12), %xmm0
.Ltmp9618:
	.loc	1 1192 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp9619:
	.loc	1 1280 32
	vmovss	888(%r12), %xmm0
.Ltmp9620:
	.loc	1 1192 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp9621:
	.loc	1 1280 32
	vmovss	1048(%r12), %xmm0
.Ltmp9622:
	.loc	1 1192 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp9623:
	.loc	1 1280 32
	vmovss	1208(%r12), %xmm0
.Ltmp9624:
	.loc	1 1192 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp9625:
	.loc	1 1280 32
	vmovss	104(%r12), %xmm0
.Ltmp9626:
	.loc	1 1192 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp9627:
	.loc	1 1280 32
	vmovss	264(%r12), %xmm0
.Ltmp9628:
	.loc	1 1192 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp9629:
	.loc	1 1280 32
	vmovss	424(%r12), %xmm0
.Ltmp9630:
	.loc	1 1192 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp9631:
	.loc	1 1280 32
	vmovss	584(%r12), %xmm0
.Ltmp9632:
	.loc	1 1192 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp9633:
	.loc	1 1280 32
	vmovss	744(%r12), %xmm0
.Ltmp9634:
	.loc	1 1192 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp9635:
	.loc	1 1280 32
	vmovss	904(%r12), %xmm0
.Ltmp9636:
	.loc	1 1192 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp9637:
	.loc	1 1280 32
	vmovss	1064(%r12), %xmm0
.Ltmp9638:
	.loc	1 1192 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp9639:
	.loc	1 1280 32
	vmovss	1224(%r12), %xmm0
.Ltmp9640:
	.loc	1 1192 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp9641:
	.loc	1 1280 32
	vmovss	120(%r12), %xmm0
.Ltmp9642:
	.loc	1 1192 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp9643:
	.loc	1 1280 32
	vmovss	280(%r12), %xmm0
.Ltmp9644:
	.loc	1 1192 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp9645:
	.loc	1 1280 32
	vmovss	440(%r12), %xmm0
.Ltmp9646:
	.loc	1 1192 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp9647:
	.loc	1 1280 32
	vmovss	600(%r12), %xmm0
.Ltmp9648:
	.loc	1 1192 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp9649:
	.loc	1 1280 32
	vmovss	760(%r12), %xmm0
.Ltmp9650:
	.loc	1 1192 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp9651:
	.loc	1 1280 32
	vmovss	920(%r12), %xmm0
.Ltmp9652:
	.loc	1 1192 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp9653:
	.loc	1 1280 32
	vmovss	1080(%r12), %xmm0
.Ltmp9654:
	.loc	1 1192 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp9655:
	.loc	1 1280 32
	vmovss	1240(%r12), %xmm0
.Ltmp9656:
	.loc	1 1192 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp9657:
	.loc	1 1280 32
	vmovss	136(%r12), %xmm0
.Ltmp9658:
	.loc	1 1192 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp9659:
	.loc	1 1280 32
	vmovss	296(%r12), %xmm0
.Ltmp9660:
	.loc	1 1192 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp9661:
	.loc	1 1280 32
	vmovss	456(%r12), %xmm0
.Ltmp9662:
	.loc	1 1192 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp9663:
	.loc	1 1280 32
	vmovss	616(%r12), %xmm0
.Ltmp9664:
	.loc	1 1192 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp9665:
	.loc	1 1280 32
	vmovss	776(%r12), %xmm0
.Ltmp9666:
	.loc	1 1192 28
	vmovss	%xmm0, 848(%rsp)
.Ltmp9667:
	.loc	1 1280 32
	vmovss	936(%r12), %xmm0
.Ltmp9668:
	.loc	1 1192 28
	vmovss	%xmm0, 852(%rsp)
.Ltmp9669:
	.loc	1 1280 32
	vmovss	1096(%r12), %xmm0
.Ltmp9670:
	.loc	1 1192 28
	vmovss	%xmm0, 856(%rsp)
.Ltmp9671:
	.loc	1 1280 32
	vmovss	1256(%r12), %xmm0
.Ltmp9672:
	.loc	1 1192 28
	vmovss	%xmm0, 860(%rsp)
.Ltmp9673:
	.loc	1 1280 32
	vmovss	152(%r12), %xmm0
.Ltmp9674:
	.loc	1 1192 28
	vmovss	%xmm0, 864(%rsp)
.Ltmp9675:
	.loc	1 1280 32
	vmovss	312(%r12), %xmm0
.Ltmp9676:
	.loc	1 1192 28
	vmovss	%xmm0, 868(%rsp)
.Ltmp9677:
	.loc	1 1280 32
	vmovss	472(%r12), %xmm0
.Ltmp9678:
	.loc	1 1192 28
	vmovss	%xmm0, 872(%rsp)
.Ltmp9679:
	.loc	1 1280 32
	vmovss	632(%r12), %xmm0
.Ltmp9680:
	.loc	1 1192 28
	vmovss	%xmm0, 876(%rsp)
.Ltmp9681:
	.loc	1 1280 32
	vmovss	792(%r12), %xmm0
.Ltmp9682:
	.loc	1 1192 28
	vmovss	%xmm0, 880(%rsp)
.Ltmp9683:
	.loc	1 1280 32
	vmovss	952(%r12), %xmm0
.Ltmp9684:
	.loc	1 1192 28
	vmovss	%xmm0, 884(%rsp)
.Ltmp9685:
	.loc	1 1280 32
	vmovss	1112(%r12), %xmm0
.Ltmp9686:
	.loc	1 1192 28
	vmovss	%xmm0, 888(%rsp)
.Ltmp9687:
	.loc	1 1280 32
	vmovss	1272(%r12), %xmm0
.Ltmp9688:
	.loc	1 1192 28
	vmovss	%xmm0, 892(%rsp)
.Ltmp9689:
	.loc	1 1279 33
	vmovss	2624(%r12), %xmm0
.Ltmp9690:
	.loc	1 1192 28
	vmovss	%xmm0, 896(%rsp)
.Ltmp9691:
	.loc	1 1279 33
	vmovss	2784(%r12), %xmm0
.Ltmp9692:
	.loc	1 1192 28
	vmovss	%xmm0, 900(%rsp)
.Ltmp9693:
	.loc	1 1279 33
	vmovss	2944(%r12), %xmm0
.Ltmp9694:
	.loc	1 1192 28
	vmovss	%xmm0, 904(%rsp)
.Ltmp9695:
	.loc	1 1279 33
	vmovss	3104(%r12), %xmm0
.Ltmp9696:
	.loc	1 1192 28
	vmovss	%xmm0, 908(%rsp)
.Ltmp9697:
	.loc	1 1279 33
	vmovss	3264(%r12), %xmm0
.Ltmp9698:
	.loc	1 1192 28
	vmovss	%xmm0, 912(%rsp)
.Ltmp9699:
	.loc	1 1279 33
	vmovss	3424(%r12), %xmm0
.Ltmp9700:
	.loc	1 1192 28
	vmovss	%xmm0, 916(%rsp)
.Ltmp9701:
	.loc	1 1279 33
	vmovss	3584(%r12), %xmm0
.Ltmp9702:
	.loc	1 1192 28
	vmovss	%xmm0, 920(%rsp)
.Ltmp9703:
	.loc	1 1279 33
	vmovss	3744(%r12), %xmm0
.Ltmp9704:
	.loc	1 1192 28
	vmovss	%xmm0, 924(%rsp)
.Ltmp9705:
	.loc	1 1279 33
	vmovss	2640(%r12), %xmm0
.Ltmp9706:
	.loc	1 1192 28
	vmovss	%xmm0, 928(%rsp)
.Ltmp9707:
	.loc	1 1279 33
	vmovss	2800(%r12), %xmm0
.Ltmp9708:
	.loc	1 1192 28
	vmovss	%xmm0, 932(%rsp)
.Ltmp9709:
	.loc	1 1279 33
	vmovss	2960(%r12), %xmm0
.Ltmp9710:
	.loc	1 1192 28
	vmovss	%xmm0, 936(%rsp)
.Ltmp9711:
	.loc	1 1279 33
	vmovss	3120(%r12), %xmm0
.Ltmp9712:
	.loc	1 1192 28
	vmovss	%xmm0, 940(%rsp)
.Ltmp9713:
	.loc	1 1279 33
	vmovss	3280(%r12), %xmm0
.Ltmp9714:
	.loc	1 1192 28
	vmovss	%xmm0, 944(%rsp)
.Ltmp9715:
	.loc	1 1279 33
	vmovss	3440(%r12), %xmm0
.Ltmp9716:
	.loc	1 1192 28
	vmovss	%xmm0, 948(%rsp)
.Ltmp9717:
	.loc	1 1279 33
	vmovss	3600(%r12), %xmm0
.Ltmp9718:
	.loc	1 1192 28
	vmovss	%xmm0, 952(%rsp)
.Ltmp9719:
	.loc	1 1279 33
	vmovss	3760(%r12), %xmm0
.Ltmp9720:
	.loc	1 1192 28
	vmovss	%xmm0, 956(%rsp)
.Ltmp9721:
	.loc	1 1279 33
	vmovss	2656(%r12), %xmm0
.Ltmp9722:
	.loc	1 1192 28
	vmovss	%xmm0, 960(%rsp)
.Ltmp9723:
	.loc	1 1279 33
	vmovss	2816(%r12), %xmm0
.Ltmp9724:
	.loc	1 1192 28
	vmovss	%xmm0, 964(%rsp)
.Ltmp9725:
	.loc	1 1279 33
	vmovss	2976(%r12), %xmm0
.Ltmp9726:
	.loc	1 1192 28
	vmovss	%xmm0, 968(%rsp)
.Ltmp9727:
	.loc	1 1279 33
	vmovss	3136(%r12), %xmm0
.Ltmp9728:
	.loc	1 1192 28
	vmovss	%xmm0, 972(%rsp)
.Ltmp9729:
	.loc	1 1279 33
	vmovss	3296(%r12), %xmm0
.Ltmp9730:
	.loc	1 1192 28
	vmovss	%xmm0, 976(%rsp)
.Ltmp9731:
	.loc	1 1279 33
	vmovss	3456(%r12), %xmm0
.Ltmp9732:
	.loc	1 1192 28
	vmovss	%xmm0, 980(%rsp)
.Ltmp9733:
	.loc	1 1279 33
	vmovss	3616(%r12), %xmm0
.Ltmp9734:
	.loc	1 1192 28
	vmovss	%xmm0, 984(%rsp)
.Ltmp9735:
	.loc	1 1279 33
	vmovss	3776(%r12), %xmm0
.Ltmp9736:
	.loc	1 1192 28
	vmovss	%xmm0, 988(%rsp)
.Ltmp9737:
	.loc	1 1279 33
	vmovss	2672(%r12), %xmm0
.Ltmp9738:
	.loc	1 1192 28
	vmovss	%xmm0, 992(%rsp)
.Ltmp9739:
	.loc	1 1279 33
	vmovss	2832(%r12), %xmm0
.Ltmp9740:
	.loc	1 1192 28
	vmovss	%xmm0, 996(%rsp)
.Ltmp9741:
	.loc	1 1279 33
	vmovss	2992(%r12), %xmm0
.Ltmp9742:
	.loc	1 1192 28
	vmovss	%xmm0, 1000(%rsp)
.Ltmp9743:
	.loc	1 1279 33
	vmovss	3152(%r12), %xmm0
.Ltmp9744:
	.loc	1 1192 28
	vmovss	%xmm0, 1004(%rsp)
.Ltmp9745:
	.loc	1 1279 33
	vmovss	3312(%r12), %xmm0
.Ltmp9746:
	.loc	1 1192 28
	vmovss	%xmm0, 1008(%rsp)
.Ltmp9747:
	.loc	1 1279 33
	vmovss	3472(%r12), %xmm0
.Ltmp9748:
	.loc	1 1192 28
	vmovss	%xmm0, 1012(%rsp)
.Ltmp9749:
	.loc	1 1279 33
	vmovss	3632(%r12), %xmm0
.Ltmp9750:
	.loc	1 1192 28
	vmovss	%xmm0, 1016(%rsp)
.Ltmp9751:
	.loc	1 1279 33
	vmovss	3792(%r12), %xmm0
.Ltmp9752:
	.loc	1 1192 28
	vmovss	%xmm0, 1020(%rsp)
.Ltmp9753:
	.loc	1 1279 33
	vmovss	2688(%r12), %xmm0
.Ltmp9754:
	.loc	1 1192 28
	vmovss	%xmm0, 1024(%rsp)
.Ltmp9755:
	.loc	1 1279 33
	vmovss	2848(%r12), %xmm0
.Ltmp9756:
	.loc	1 1192 28
	vmovss	%xmm0, 1028(%rsp)
.Ltmp9757:
	.loc	1 1279 33
	vmovss	3008(%r12), %xmm0
.Ltmp9758:
	.loc	1 1192 28
	vmovss	%xmm0, 1032(%rsp)
.Ltmp9759:
	.loc	1 1279 33
	vmovss	3168(%r12), %xmm0
.Ltmp9760:
	.loc	1 1192 28
	vmovss	%xmm0, 1036(%rsp)
.Ltmp9761:
	.loc	1 1279 33
	vmovss	3328(%r12), %xmm0
.Ltmp9762:
	.loc	1 1192 28
	vmovss	%xmm0, 1040(%rsp)
.Ltmp9763:
	.loc	1 1279 33
	vmovss	3488(%r12), %xmm0
.Ltmp9764:
	.loc	1 1192 28
	vmovss	%xmm0, 1044(%rsp)
.Ltmp9765:
	.loc	1 1279 33
	vmovss	3648(%r12), %xmm0
.Ltmp9766:
	.loc	1 1192 28
	vmovss	%xmm0, 1048(%rsp)
.Ltmp9767:
	.loc	1 1279 33
	vmovss	3808(%r12), %xmm0
.Ltmp9768:
	.loc	1 1192 28
	vmovss	%xmm0, 1052(%rsp)
.Ltmp9769:
	.loc	1 1279 33
	vmovss	2704(%r12), %xmm0
.Ltmp9770:
	.loc	1 1192 28
	vmovss	%xmm0, 1056(%rsp)
.Ltmp9771:
	.loc	1 1279 33
	vmovss	2864(%r12), %xmm0
.Ltmp9772:
	.loc	1 1192 28
	vmovss	%xmm0, 1060(%rsp)
.Ltmp9773:
	.loc	1 1279 33
	vmovss	3024(%r12), %xmm0
.Ltmp9774:
	.loc	1 1192 28
	vmovss	%xmm0, 1064(%rsp)
.Ltmp9775:
	.loc	1 1279 33
	vmovss	3184(%r12), %xmm0
.Ltmp9776:
	.loc	1 1192 28
	vmovss	%xmm0, 1068(%rsp)
.Ltmp9777:
	.loc	1 1279 33
	vmovss	3344(%r12), %xmm0
.Ltmp9778:
	.loc	1 1192 28
	vmovss	%xmm0, 1072(%rsp)
.Ltmp9779:
	.loc	1 1279 33
	vmovss	3504(%r12), %xmm0
.Ltmp9780:
	.loc	1 1192 28
	vmovss	%xmm0, 1076(%rsp)
.Ltmp9781:
	.loc	1 1279 33
	vmovss	3664(%r12), %xmm0
.Ltmp9782:
	.loc	1 1192 28
	vmovss	%xmm0, 1080(%rsp)
.Ltmp9783:
	.loc	1 1279 33
	vmovss	3824(%r12), %xmm0
.Ltmp9784:
	.loc	1 1192 28
	vmovss	%xmm0, 1084(%rsp)
.Ltmp9785:
	.loc	1 1279 33
	vmovss	2720(%r12), %xmm0
.Ltmp9786:
	.loc	1 1192 28
	vmovss	%xmm0, 1088(%rsp)
.Ltmp9787:
	.loc	1 1279 33
	vmovss	2880(%r12), %xmm0
.Ltmp9788:
	.loc	1 1192 28
	vmovss	%xmm0, 1092(%rsp)
.Ltmp9789:
	.loc	1 1279 33
	vmovss	3040(%r12), %xmm0
.Ltmp9790:
	.loc	1 1192 28
	vmovss	%xmm0, 1096(%rsp)
.Ltmp9791:
	.loc	1 1279 33
	vmovss	3200(%r12), %xmm0
.Ltmp9792:
	.loc	1 1192 28
	vmovss	%xmm0, 1100(%rsp)
.Ltmp9793:
	.loc	1 1279 33
	vmovss	3360(%r12), %xmm0
.Ltmp9794:
	.loc	1 1192 28
	vmovss	%xmm0, 1104(%rsp)
.Ltmp9795:
	.loc	1 1279 33
	vmovss	3520(%r12), %xmm0
.Ltmp9796:
	.loc	1 1192 28
	vmovss	%xmm0, 1108(%rsp)
.Ltmp9797:
	.loc	1 1279 33
	vmovss	3680(%r12), %xmm0
.Ltmp9798:
	.loc	1 1192 28
	vmovss	%xmm0, 1112(%rsp)
.Ltmp9799:
	.loc	1 1279 33
	vmovss	3840(%r12), %xmm0
.Ltmp9800:
	.loc	1 1192 28
	vmovss	%xmm0, 1116(%rsp)
.Ltmp9801:
	.loc	1 1279 33
	vmovss	2736(%r12), %xmm0
.Ltmp9802:
	.loc	1 1192 28
	vmovss	%xmm0, 1120(%rsp)
.Ltmp9803:
	.loc	1 1279 33
	vmovss	2896(%r12), %xmm0
.Ltmp9804:
	.loc	1 1192 28
	vmovss	%xmm0, 1124(%rsp)
.Ltmp9805:
	.loc	1 1279 33
	vmovss	3056(%r12), %xmm0
.Ltmp9806:
	.loc	1 1192 28
	vmovss	%xmm0, 1128(%rsp)
.Ltmp9807:
	.loc	1 1279 33
	vmovss	3216(%r12), %xmm0
.Ltmp9808:
	.loc	1 1192 28
	vmovss	%xmm0, 1132(%rsp)
.Ltmp9809:
	.loc	1 1279 33
	vmovss	3376(%r12), %xmm0
.Ltmp9810:
	.loc	1 1192 28
	vmovss	%xmm0, 1136(%rsp)
.Ltmp9811:
	.loc	1 1279 33
	vmovss	3536(%r12), %xmm0
.Ltmp9812:
	.loc	1 1192 28
	vmovss	%xmm0, 1140(%rsp)
.Ltmp9813:
	.loc	1 1279 33
	vmovss	3696(%r12), %xmm0
.Ltmp9814:
	.loc	1 1192 28
	vmovss	%xmm0, 1144(%rsp)
.Ltmp9815:
	.loc	1 1279 33
	vmovss	3856(%r12), %xmm0
.Ltmp9816:
	.loc	1 1192 28
	vmovss	%xmm0, 1148(%rsp)
.Ltmp9817:
	.loc	1 1279 33
	vmovss	2752(%r12), %xmm0
.Ltmp9818:
	.loc	1 1192 28
	vmovss	%xmm0, 1152(%rsp)
.Ltmp9819:
	.loc	1 1279 33
	vmovss	2912(%r12), %xmm0
.Ltmp9820:
	.loc	1 1192 28
	vmovss	%xmm0, 1156(%rsp)
.Ltmp9821:
	.loc	1 1279 33
	vmovss	3072(%r12), %xmm0
.Ltmp9822:
	.loc	1 1192 28
	vmovss	%xmm0, 1160(%rsp)
.Ltmp9823:
	.loc	1 1279 33
	vmovss	3232(%r12), %xmm0
.Ltmp9824:
	.loc	1 1192 28
	vmovss	%xmm0, 1164(%rsp)
.Ltmp9825:
	.loc	1 1279 33
	vmovss	3392(%r12), %xmm0
.Ltmp9826:
	.loc	1 1192 28
	vmovss	%xmm0, 1168(%rsp)
.Ltmp9827:
	.loc	1 1279 33
	vmovss	3552(%r12), %xmm0
.Ltmp9828:
	.loc	1 1192 28
	vmovss	%xmm0, 1172(%rsp)
.Ltmp9829:
	.loc	1 1279 33
	vmovss	3712(%r12), %xmm0
.Ltmp9830:
	.loc	1 1192 28
	vmovss	%xmm0, 1176(%rsp)
.Ltmp9831:
	.loc	1 1279 33
	vmovss	3872(%r12), %xmm0
.Ltmp9832:
	.loc	1 1192 28
	vmovss	%xmm0, 1180(%rsp)
.Ltmp9833:
	.loc	1 1279 33
	vmovss	2768(%r12), %xmm0
.Ltmp9834:
	.loc	1 1192 28
	vmovss	%xmm0, 1184(%rsp)
.Ltmp9835:
	.loc	1 1279 33
	vmovss	2928(%r12), %xmm0
.Ltmp9836:
	.loc	1 1192 28
	vmovss	%xmm0, 1188(%rsp)
.Ltmp9837:
	.loc	1 1279 33
	vmovss	3088(%r12), %xmm0
.Ltmp9838:
	.loc	1 1192 28
	vmovss	%xmm0, 1192(%rsp)
.Ltmp9839:
	.loc	1 1279 33
	vmovss	3248(%r12), %xmm0
.Ltmp9840:
	.loc	1 1192 28
	vmovss	%xmm0, 1196(%rsp)
.Ltmp9841:
	.loc	1 1279 33
	vmovss	3408(%r12), %xmm0
.Ltmp9842:
	.loc	1 1192 28
	vmovss	%xmm0, 1200(%rsp)
.Ltmp9843:
	.loc	1 1279 33
	vmovss	3568(%r12), %xmm0
.Ltmp9844:
	.loc	1 1192 28
	vmovss	%xmm0, 1204(%rsp)
.Ltmp9845:
	.loc	1 1279 33
	vmovss	3728(%r12), %xmm0
.Ltmp9846:
	.loc	1 1192 28
	vmovss	%xmm0, 1208(%rsp)
.Ltmp9847:
	.loc	1 1279 33
	vmovss	3888(%r12), %xmm0
.Ltmp9848:
	.loc	1 1192 28
	vmovss	%xmm0, 1212(%rsp)
.Ltmp9849:
	.loc	1 1280 32
	vmovss	2632(%r12), %xmm0
.Ltmp9850:
	.loc	1 1192 28
	vmovss	%xmm0, 1216(%rsp)
.Ltmp9851:
	.loc	1 1280 32
	vmovss	2792(%r12), %xmm0
.Ltmp9852:
	.loc	1 1192 28
	vmovss	%xmm0, 1220(%rsp)
.Ltmp9853:
	.loc	1 1280 32
	vmovss	2952(%r12), %xmm0
.Ltmp9854:
	.loc	1 1192 28
	vmovss	%xmm0, 1224(%rsp)
.Ltmp9855:
	.loc	1 1280 32
	vmovss	3112(%r12), %xmm0
.Ltmp9856:
	.loc	1 1192 28
	vmovss	%xmm0, 1228(%rsp)
.Ltmp9857:
	.loc	1 1280 32
	vmovss	3272(%r12), %xmm0
.Ltmp9858:
	.loc	1 1192 28
	vmovss	%xmm0, 1232(%rsp)
.Ltmp9859:
	.loc	1 1280 32
	vmovss	3432(%r12), %xmm0
.Ltmp9860:
	.loc	1 1192 28
	vmovss	%xmm0, 1236(%rsp)
.Ltmp9861:
	.loc	1 1280 32
	vmovss	3592(%r12), %xmm0
.Ltmp9862:
	.loc	1 1192 28
	vmovss	%xmm0, 1240(%rsp)
.Ltmp9863:
	.loc	1 1280 32
	vmovss	3752(%r12), %xmm0
.Ltmp9864:
	.loc	1 1192 28
	vmovss	%xmm0, 1244(%rsp)
.Ltmp9865:
	.loc	1 1280 32
	vmovss	2648(%r12), %xmm0
.Ltmp9866:
	.loc	1 1192 28
	vmovss	%xmm0, 1248(%rsp)
.Ltmp9867:
	.loc	1 1280 32
	vmovss	2808(%r12), %xmm0
.Ltmp9868:
	.loc	1 1192 28
	vmovss	%xmm0, 1252(%rsp)
.Ltmp9869:
	.loc	1 1280 32
	vmovss	2968(%r12), %xmm0
.Ltmp9870:
	.loc	1 1192 28
	vmovss	%xmm0, 1256(%rsp)
.Ltmp9871:
	.loc	1 1280 32
	vmovss	3128(%r12), %xmm0
.Ltmp9872:
	.loc	1 1192 28
	vmovss	%xmm0, 1260(%rsp)
.Ltmp9873:
	.loc	1 1280 32
	vmovss	3288(%r12), %xmm0
.Ltmp9874:
	.loc	1 1192 28
	vmovss	%xmm0, 1264(%rsp)
.Ltmp9875:
	.loc	1 1280 32
	vmovss	3448(%r12), %xmm0
.Ltmp9876:
	.loc	1 1192 28
	vmovss	%xmm0, 1268(%rsp)
.Ltmp9877:
	.loc	1 1280 32
	vmovss	3608(%r12), %xmm0
.Ltmp9878:
	.loc	1 1192 28
	vmovss	%xmm0, 1272(%rsp)
.Ltmp9879:
	.loc	1 1280 32
	vmovss	3768(%r12), %xmm0
.Ltmp9880:
	.loc	1 1192 28
	vmovss	%xmm0, 1276(%rsp)
.Ltmp9881:
	.loc	1 1280 32
	vmovss	2664(%r12), %xmm0
.Ltmp9882:
	.loc	1 1192 28
	vmovss	%xmm0, 1280(%rsp)
.Ltmp9883:
	.loc	1 1280 32
	vmovss	2824(%r12), %xmm0
.Ltmp9884:
	.loc	1 1192 28
	vmovss	%xmm0, 1284(%rsp)
.Ltmp9885:
	.loc	1 1280 32
	vmovss	2984(%r12), %xmm0
.Ltmp9886:
	.loc	1 1192 28
	vmovss	%xmm0, 1288(%rsp)
.Ltmp9887:
	.loc	1 1280 32
	vmovss	3144(%r12), %xmm0
.Ltmp9888:
	.loc	1 1192 28
	vmovss	%xmm0, 1292(%rsp)
.Ltmp9889:
	.loc	1 1280 32
	vmovss	3304(%r12), %xmm0
.Ltmp9890:
	.loc	1 1192 28
	vmovss	%xmm0, 1296(%rsp)
.Ltmp9891:
	.loc	1 1280 32
	vmovss	3464(%r12), %xmm0
.Ltmp9892:
	.loc	1 1192 28
	vmovss	%xmm0, 1300(%rsp)
.Ltmp9893:
	.loc	1 1280 32
	vmovss	3624(%r12), %xmm0
.Ltmp9894:
	.loc	1 1192 28
	vmovss	%xmm0, 1304(%rsp)
.Ltmp9895:
	.loc	1 1280 32
	vmovss	3784(%r12), %xmm0
.Ltmp9896:
	.loc	1 1192 28
	vmovss	%xmm0, 1308(%rsp)
.Ltmp9897:
	.loc	1 1280 32
	vmovss	2680(%r12), %xmm0
.Ltmp9898:
	.loc	1 1192 28
	vmovss	%xmm0, 1312(%rsp)
.Ltmp9899:
	.loc	1 1280 32
	vmovss	2840(%r12), %xmm0
.Ltmp9900:
	.loc	1 1192 28
	vmovss	%xmm0, 1316(%rsp)
.Ltmp9901:
	.loc	1 1280 32
	vmovss	3000(%r12), %xmm0
.Ltmp9902:
	.loc	1 1192 28
	vmovss	%xmm0, 1320(%rsp)
.Ltmp9903:
	.loc	1 1280 32
	vmovss	3160(%r12), %xmm0
.Ltmp9904:
	.loc	1 1192 28
	vmovss	%xmm0, 1324(%rsp)
.Ltmp9905:
	.loc	1 1280 32
	vmovss	3320(%r12), %xmm0
.Ltmp9906:
	.loc	1 1192 28
	vmovss	%xmm0, 1328(%rsp)
.Ltmp9907:
	.loc	1 1280 32
	vmovss	3480(%r12), %xmm0
.Ltmp9908:
	.loc	1 1192 28
	vmovss	%xmm0, 1332(%rsp)
.Ltmp9909:
	.loc	1 1280 32
	vmovss	3640(%r12), %xmm0
.Ltmp9910:
	.loc	1 1192 28
	vmovss	%xmm0, 1336(%rsp)
.Ltmp9911:
	.loc	1 1280 32
	vmovss	3800(%r12), %xmm0
.Ltmp9912:
	.loc	1 1192 28
	vmovss	%xmm0, 1340(%rsp)
.Ltmp9913:
	.loc	1 1280 32
	vmovss	2696(%r12), %xmm0
.Ltmp9914:
	.loc	1 1192 28
	vmovss	%xmm0, 1344(%rsp)
.Ltmp9915:
	.loc	1 1280 32
	vmovss	2856(%r12), %xmm0
.Ltmp9916:
	.loc	1 1192 28
	vmovss	%xmm0, 1348(%rsp)
.Ltmp9917:
	.loc	1 1280 32
	vmovss	3016(%r12), %xmm0
.Ltmp9918:
	.loc	1 1192 28
	vmovss	%xmm0, 1352(%rsp)
.Ltmp9919:
	.loc	1 1280 32
	vmovss	3176(%r12), %xmm0
.Ltmp9920:
	.loc	1 1192 28
	vmovss	%xmm0, 1356(%rsp)
.Ltmp9921:
	.loc	1 1280 32
	vmovss	3336(%r12), %xmm0
.Ltmp9922:
	.loc	1 1192 28
	vmovss	%xmm0, 1360(%rsp)
.Ltmp9923:
	.loc	1 1280 32
	vmovss	3496(%r12), %xmm0
.Ltmp9924:
	.loc	1 1192 28
	vmovss	%xmm0, 1364(%rsp)
.Ltmp9925:
	.loc	1 1280 32
	vmovss	3656(%r12), %xmm0
.Ltmp9926:
	.loc	1 1192 28
	vmovss	%xmm0, 1368(%rsp)
.Ltmp9927:
	.loc	1 1280 32
	vmovss	3816(%r12), %xmm0
.Ltmp9928:
	.loc	1 1192 28
	vmovss	%xmm0, 1372(%rsp)
.Ltmp9929:
	.loc	1 1280 32
	vmovss	2712(%r12), %xmm0
.Ltmp9930:
	.loc	1 1192 28
	vmovss	%xmm0, 1376(%rsp)
.Ltmp9931:
	.loc	1 1280 32
	vmovss	2872(%r12), %xmm0
.Ltmp9932:
	.loc	1 1192 28
	vmovss	%xmm0, 1380(%rsp)
.Ltmp9933:
	.loc	1 1280 32
	vmovss	3032(%r12), %xmm0
.Ltmp9934:
	.loc	1 1192 28
	vmovss	%xmm0, 1384(%rsp)
.Ltmp9935:
	.loc	1 1280 32
	vmovss	3192(%r12), %xmm0
.Ltmp9936:
	.loc	1 1192 28
	vmovss	%xmm0, 1388(%rsp)
.Ltmp9937:
	.loc	1 1280 32
	vmovss	3352(%r12), %xmm0
.Ltmp9938:
	.loc	1 1192 28
	vmovss	%xmm0, 1392(%rsp)
.Ltmp9939:
	.loc	1 1280 32
	vmovss	3512(%r12), %xmm0
.Ltmp9940:
	.loc	1 1192 28
	vmovss	%xmm0, 1396(%rsp)
.Ltmp9941:
	.loc	1 1280 32
	vmovss	3672(%r12), %xmm0
.Ltmp9942:
	.loc	1 1192 28
	vmovss	%xmm0, 1400(%rsp)
.Ltmp9943:
	.loc	1 1280 32
	vmovss	3832(%r12), %xmm0
.Ltmp9944:
	.loc	1 1192 28
	vmovss	%xmm0, 1404(%rsp)
.Ltmp9945:
	.loc	1 1280 32
	vmovss	2728(%r12), %xmm0
.Ltmp9946:
	.loc	1 1192 28
	vmovss	%xmm0, 1408(%rsp)
.Ltmp9947:
	.loc	1 1280 32
	vmovss	2888(%r12), %xmm0
.Ltmp9948:
	.loc	1 1192 28
	vmovss	%xmm0, 1412(%rsp)
.Ltmp9949:
	.loc	1 1280 32
	vmovss	3048(%r12), %xmm0
.Ltmp9950:
	.loc	1 1192 28
	vmovss	%xmm0, 1416(%rsp)
.Ltmp9951:
	.loc	1 1280 32
	vmovss	3208(%r12), %xmm0
.Ltmp9952:
	.loc	1 1192 28
	vmovss	%xmm0, 1420(%rsp)
.Ltmp9953:
	.loc	1 1280 32
	vmovss	3368(%r12), %xmm0
.Ltmp9954:
	.loc	1 1192 28
	vmovss	%xmm0, 1424(%rsp)
.Ltmp9955:
	.loc	1 1280 32
	vmovss	3528(%r12), %xmm0
.Ltmp9956:
	.loc	1 1192 28
	vmovss	%xmm0, 1428(%rsp)
.Ltmp9957:
	.loc	1 1280 32
	vmovss	3688(%r12), %xmm0
.Ltmp9958:
	.loc	1 1192 28
	vmovss	%xmm0, 1432(%rsp)
.Ltmp9959:
	.loc	1 1280 32
	vmovss	3848(%r12), %xmm0
.Ltmp9960:
	.loc	1 1192 28
	vmovss	%xmm0, 1436(%rsp)
.Ltmp9961:
	.loc	1 1280 32
	vmovss	2744(%r12), %xmm0
.Ltmp9962:
	.loc	1 1192 28
	vmovss	%xmm0, 1440(%rsp)
.Ltmp9963:
	.loc	1 1280 32
	vmovss	2904(%r12), %xmm0
.Ltmp9964:
	.loc	1 1192 28
	vmovss	%xmm0, 1444(%rsp)
.Ltmp9965:
	.loc	1 1280 32
	vmovss	3064(%r12), %xmm0
.Ltmp9966:
	.loc	1 1192 28
	vmovss	%xmm0, 1448(%rsp)
.Ltmp9967:
	.loc	1 1280 32
	vmovss	3224(%r12), %xmm0
.Ltmp9968:
	.loc	1 1192 28
	vmovss	%xmm0, 1452(%rsp)
.Ltmp9969:
	.loc	1 1280 32
	vmovss	3384(%r12), %xmm0
.Ltmp9970:
	.loc	1 1192 28
	vmovss	%xmm0, 1456(%rsp)
.Ltmp9971:
	.loc	1 1280 32
	vmovss	3544(%r12), %xmm0
.Ltmp9972:
	.loc	1 1192 28
	vmovss	%xmm0, 1460(%rsp)
.Ltmp9973:
	.loc	1 1280 32
	vmovss	3704(%r12), %xmm0
.Ltmp9974:
	.loc	1 1192 28
	vmovss	%xmm0, 1464(%rsp)
.Ltmp9975:
	.loc	1 1280 32
	vmovss	3864(%r12), %xmm0
.Ltmp9976:
	.loc	1 1192 28
	vmovss	%xmm0, 1468(%rsp)
.Ltmp9977:
	.loc	1 1280 32
	vmovss	2760(%r12), %xmm0
.Ltmp9978:
	.loc	1 1192 28
	vmovss	%xmm0, 1472(%rsp)
.Ltmp9979:
	.loc	1 1280 32
	vmovss	2920(%r12), %xmm0
.Ltmp9980:
	.loc	1 1192 28
	vmovss	%xmm0, 1476(%rsp)
.Ltmp9981:
	.loc	1 1280 32
	vmovss	3080(%r12), %xmm0
.Ltmp9982:
	.loc	1 1192 28
	vmovss	%xmm0, 1480(%rsp)
.Ltmp9983:
	.loc	1 1280 32
	vmovss	3240(%r12), %xmm0
.Ltmp9984:
	.loc	1 1192 28
	vmovss	%xmm0, 1484(%rsp)
.Ltmp9985:
	.loc	1 1280 32
	vmovss	3400(%r12), %xmm0
.Ltmp9986:
	.loc	1 1192 28
	vmovss	%xmm0, 1488(%rsp)
.Ltmp9987:
	.loc	1 1280 32
	vmovss	3560(%r12), %xmm0
.Ltmp9988:
	.loc	1 1192 28
	vmovss	%xmm0, 1492(%rsp)
.Ltmp9989:
	.loc	1 1280 32
	vmovss	3720(%r12), %xmm0
.Ltmp9990:
	.loc	1 1192 28
	vmovss	%xmm0, 1496(%rsp)
.Ltmp9991:
	.loc	1 1280 32
	vmovss	3880(%r12), %xmm0
.Ltmp9992:
	.loc	1 1192 28
	vmovss	%xmm0, 1500(%rsp)
.Ltmp9993:
	.loc	1 1280 32
	vmovss	2776(%r12), %xmm0
.Ltmp9994:
	.loc	1 1192 28
	vmovss	%xmm0, 1504(%rsp)
.Ltmp9995:
	.loc	1 1280 32
	vmovss	2936(%r12), %xmm0
.Ltmp9996:
	.loc	1 1192 28
	vmovss	%xmm0, 1508(%rsp)
.Ltmp9997:
	.loc	1 1280 32
	vmovss	3096(%r12), %xmm0
.Ltmp9998:
	.loc	1 1192 28
	vmovss	%xmm0, 1512(%rsp)
.Ltmp9999:
	.loc	1 1280 32
	vmovss	3256(%r12), %xmm0
.Ltmp10000:
	.loc	1 1192 28
	vmovss	%xmm0, 1516(%rsp)
.Ltmp10001:
	.loc	1 1280 32
	vmovss	3416(%r12), %xmm0
.Ltmp10002:
	.loc	1 1192 28
	vmovss	%xmm0, 1520(%rsp)
.Ltmp10003:
	.loc	1 1280 32
	vmovss	3576(%r12), %xmm0
.Ltmp10004:
	.loc	1 1192 28
	vmovss	%xmm0, 1524(%rsp)
.Ltmp10005:
	.loc	1 1280 32
	vmovss	3736(%r12), %xmm0
.Ltmp10006:
	.loc	1 1192 28
	vmovss	%xmm0, 1528(%rsp)
.Ltmp10007:
	.loc	1 1280 32
	vmovss	3896(%r12), %xmm0
.Ltmp10008:
	.loc	1 1192 28
	vmovss	%xmm0, 1532(%rsp)
.Ltmp10009:
	.loc	1 1194 31
	leaq	3136(%rsp), %rdi
	movq	%r12, %rsi
	movl	1820(%rsp), %r14d
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	3328(%rsp), %rdi
	movq	2328(%rsp), %rsi
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E17band_coefficientsB5_
	.loc	1 1193 28
	vmovaps	3136(%rsp), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	3168(%rsp), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	3200(%rsp), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	3232(%rsp), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	3328(%rsp), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	3360(%rsp), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	3392(%rsp), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	3424(%rsp), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	3456(%rsp), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
	vmovaps	3488(%rsp), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
.Ltmp10010:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%r13), %rax
	shlq	$3, %r13
	leaq	(,%rax,8), %rsi
	.loc	1 1197 12 is_stmt 1
	testb	$1, 64(%rsp)
	movq	%r15, 2336(%rsp)
	movq	%rax, 2040(%rsp)
	je	.LBB40_525
.Ltmp10011:
	.loc	38 1050 16
	cmpq	%r13, %rsi
.Ltmp10012:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_657
.Ltmp10013:
	.loc	48 451 16 is_stmt 1
	cmpq	128(%rsp), %rsi
	ja	.LBB40_657
.Ltmp10014:
	.loc	48 451 16 is_stmt 0
	cmpq	120(%rsp), %rsi
	ja	.LBB40_658
.Ltmp10015:
	.loc	1 1053 27 is_stmt 1
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp10016:
	.loc	1 1054 26
	vmovaps	4032(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	4064(%r12), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	4096(%r12), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	4128(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
.Ltmp10017:
	.loc	1 1055 25
	vmovaps	2304(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	2336(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp10018:
	.loc	1 1056 24
	vmovaps	4928(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	4960(%r12), %ymm6
.Ltmp10019:
	.loc	1 1057 24
	movq	5272(%r12), %r10
.Ltmp10020:
	.loc	1 871 17
	movq	2368(%r12), %rax
	movq	2376(%r12), %rcx
.Ltmp10021:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2384(%r12), %rdx
	xorq	%rax, %rdx
	movq	2392(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2400(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	2408(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2416(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	orq	%rcx, %rdx
	xorq	2424(%r12), %rax
	orq	%rdx, %rax
	sete	1856(%rsp)
.Ltmp10022:
	.loc	1 871 17
	movq	4992(%r12), %rax
	movq	5000(%r12), %rcx
.Ltmp10023:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	5008(%r12), %rdx
	xorq	%rax, %rdx
	movq	5016(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5024(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	5032(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5040(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	xorq	5048(%r12), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	2048(%rsp)
.Ltmp10024:
	.loc	2 1916 50
	testq	%r15, %r15
	movl	$0, %r11d
	je	.LBB40_521
.Ltmp10025:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,8), %rax
	movq	%rax, 176(%rsp)
	movq	240(%rsp), %rax
	leaq	(%rax,%r13,4), %rcx
	movq	248(%rsp), %rax
	leaq	(%rax,%r13,4), %rdx
.Ltmp10026:
	.loc	3 900 12 is_stmt 1
	movabsq	$2305843009213693951, %rax
	andq	%rax, %r15
	movq	%r15, 184(%rsp)
	xorl	%edi, %edi
	xorl	%r9d, %r9d
	movq	1888(%rsp), %r13
	movq	%rcx, 152(%rsp)
	movq	%rdx, 168(%rsp)
.Ltmp10027:
	.loc	3 0 12 is_stmt 0
.Ltmp10028:
	.p2align	4
.LBB40_456:
	.loc	1 1064 21 is_stmt 1
	vmovaps	256(%rsp), %ymm0
	vmovaps	288(%rsp), %ymm1
	vmovaps	320(%rsp), %ymm2
.Ltmp10029:
	.loc	14 48 14
	vaddps	576(%rsp), %ymm0, %ymm0
.Ltmp10030:
	.loc	1 1063 17
	vmovaps	%ymm0, 256(%rsp)
	.loc	1 1066 21
	vmovaps	896(%rsp), %ymm0
.Ltmp10031:
	.loc	14 48 14
	vaddps	1216(%rsp), %ymm0, %ymm0
.Ltmp10032:
	.loc	1 1065 17
	vmovaps	%ymm0, 896(%rsp)
.Ltmp10033:
	.loc	14 48 14
	vaddps	608(%rsp), %ymm1, %ymm0
.Ltmp10034:
	.loc	1 1063 17
	vmovaps	%ymm0, 288(%rsp)
	.loc	1 1066 21
	vmovaps	928(%rsp), %ymm0
.Ltmp10035:
	.loc	14 48 14
	vaddps	1248(%rsp), %ymm0, %ymm0
.Ltmp10036:
	.loc	1 1065 17
	vmovaps	%ymm0, 928(%rsp)
.Ltmp10037:
	.loc	14 48 14
	vaddps	640(%rsp), %ymm2, %ymm0
.Ltmp10038:
	.loc	1 1063 17
	vmovaps	%ymm0, 320(%rsp)
	.loc	1 1066 21
	vmovaps	960(%rsp), %ymm0
.Ltmp10039:
	.loc	14 48 14
	vaddps	1280(%rsp), %ymm0, %ymm0
.Ltmp10040:
	.loc	1 1065 17
	vmovaps	%ymm0, 960(%rsp)
	.loc	1 1064 21
	vmovaps	352(%rsp), %ymm0
.Ltmp10041:
	.loc	14 48 14
	vaddps	672(%rsp), %ymm0, %ymm0
.Ltmp10042:
	.loc	1 1063 17
	vmovaps	%ymm0, 352(%rsp)
	.loc	1 1066 21
	vmovaps	992(%rsp), %ymm0
.Ltmp10043:
	.loc	14 48 14
	vaddps	1312(%rsp), %ymm0, %ymm0
.Ltmp10044:
	.loc	1 1065 17
	vmovaps	%ymm0, 992(%rsp)
	.loc	1 1064 21
	vmovaps	384(%rsp), %ymm0
.Ltmp10045:
	.loc	14 48 14
	vaddps	704(%rsp), %ymm0, %ymm0
.Ltmp10046:
	.loc	1 1063 17
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 1066 21
	vmovaps	1024(%rsp), %ymm0
.Ltmp10047:
	.loc	14 48 14
	vaddps	1344(%rsp), %ymm0, %ymm0
.Ltmp10048:
	.loc	1 1065 17
	vmovaps	%ymm0, 1024(%rsp)
	.loc	1 1064 21
	vmovaps	416(%rsp), %ymm0
.Ltmp10049:
	.loc	14 48 14
	vaddps	736(%rsp), %ymm0, %ymm0
.Ltmp10050:
	.loc	1 1063 17
	vmovaps	%ymm0, 416(%rsp)
	.loc	1 1066 21
	vmovaps	1056(%rsp), %ymm0
.Ltmp10051:
	.loc	14 48 14
	vaddps	1376(%rsp), %ymm0, %ymm0
.Ltmp10052:
	.loc	1 1065 17
	vmovaps	%ymm0, 1056(%rsp)
	.loc	1 1064 21
	vmovaps	448(%rsp), %ymm0
.Ltmp10053:
	.loc	14 48 14
	vaddps	768(%rsp), %ymm0, %ymm0
.Ltmp10054:
	.loc	1 1063 17
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 1066 21
	vmovaps	1088(%rsp), %ymm0
.Ltmp10055:
	.loc	14 48 14
	vaddps	1408(%rsp), %ymm0, %ymm0
.Ltmp10056:
	.loc	1 1065 17
	vmovaps	%ymm0, 1088(%rsp)
	.loc	1 1064 21
	vmovaps	480(%rsp), %ymm0
.Ltmp10057:
	.loc	14 48 14
	vaddps	800(%rsp), %ymm0, %ymm0
.Ltmp10058:
	.loc	1 1063 17
	vmovaps	%ymm0, 480(%rsp)
	.loc	1 1066 21
	vmovaps	1120(%rsp), %ymm0
.Ltmp10059:
	.loc	14 48 14
	vaddps	1440(%rsp), %ymm0, %ymm0
.Ltmp10060:
	.loc	1 1065 17
	vmovaps	%ymm0, 1120(%rsp)
	.loc	1 1064 21
	vmovaps	512(%rsp), %ymm0
.Ltmp10061:
	.loc	14 48 14
	vaddps	832(%rsp), %ymm0, %ymm0
.Ltmp10062:
	.loc	1 1063 17
	vmovaps	%ymm0, 512(%rsp)
	.loc	1 1066 21
	vmovaps	1152(%rsp), %ymm0
.Ltmp10063:
	.loc	14 48 14
	vaddps	1472(%rsp), %ymm0, %ymm0
.Ltmp10064:
	.loc	1 1065 17
	vmovaps	%ymm0, 1152(%rsp)
	.loc	1 1064 21
	vmovaps	544(%rsp), %ymm0
.Ltmp10065:
	.loc	14 48 14
	vaddps	864(%rsp), %ymm0, %ymm0
.Ltmp10066:
	.loc	1 1063 17
	vmovaps	%ymm0, 544(%rsp)
	.loc	1 1066 21
	vmovaps	1184(%rsp), %ymm0
.Ltmp10067:
	.loc	14 48 14
	vaddps	1504(%rsp), %ymm0, %ymm0
.Ltmp10068:
	.loc	1 1065 17
	vmovaps	%ymm0, 1184(%rsp)
.Ltmp10069:
	.loc	1 1070 28
	leaq	1(%r10), %rax
.Ltmp10070:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %r15d
	cmovaeq	%r13, %r15
.Ltmp10071:
	.loc	48 568 12
	cmpq	176(%rsp), %rdi
	ja	.LBB40_640
.Ltmp10072:
	.loc	48 438 16
	cmpq	%r9, 184(%rsp)
	je	.LBB40_641
.Ltmp10073:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,8), %rax
.Ltmp10074:
	.loc	1 1083 29 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp10075:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_642
.Ltmp10076:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10077:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm6, 2368(%rsp)
	vmovups	(%rcx,%rdi,4), %ymm7
.Ltmp10078:
	vmovups	(%rdx,%rdi,4), %ymm0
.Ltmp10079:
	vmovaps	1280(%r12), %ymm5
	vmovaps	1312(%r12), %ymm11
	vmovaps	1344(%r12), %ymm1
	vmovaps	3904(%r12), %ymm14
	vmovaps	3936(%r12), %ymm9
	vmovaps	3968(%r12), %ymm2
	vmovaps	64(%rsp), %ymm8
	vsubps	%ymm8, %ymm7, %ymm3
	vmulps	%ymm3, %ymm11, %ymm4
	vmovaps	1632(%rsp), %ymm10
	vmovaps	%ymm5, 2208(%rsp)
	vmulps	%ymm5, %ymm10, %ymm5
	vaddps	%ymm4, %ymm5, %ymm6
	vaddps	%ymm6, %ymm10, %ymm4
	vmulps	%ymm11, %ymm10, %ymm5
	vmulps	%ymm1, %ymm3, %ymm3
	vaddps	%ymm3, %ymm5, %ymm5
	vaddps	%ymm5, %ymm8, %ymm3
	vmulps	1376(%r12), %ymm4, %ymm15
	vmovaps	1568(%rsp), %ymm8
	vsubps	%ymm8, %ymm3, %ymm4
	vmulps	192(%rsp), %ymm11, %ymm3
	vmulps	%ymm4, %ymm1, %ymm1
	vaddps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 2176(%rsp)
	vaddps	%ymm1, %ymm8, %ymm3
	vmovaps	2144(%rsp), %ymm8
.Ltmp10080:
	vsubps	%ymm8, %ymm0, %ymm13
	vmulps	%ymm9, %ymm13, %ymm1
	vmovaps	1760(%rsp), %ymm10
	vmulps	%ymm14, %ymm10, %ymm12
	vaddps	%ymm1, %ymm12, %ymm1
	vaddps	%ymm1, %ymm10, %ymm12
	vmulps	4000(%r12), %ymm12, %ymm12
.Ltmp10081:
	.loc	1 1083 29 is_stmt 1
	movq	2592(%r12), %rcx
.Ltmp10082:
	.loc	8 551 14
	vmovups	%ymm3, (%rcx,%rax,4)
.Ltmp10083:
	.loc	1 1084 30
	movq	2616(%r12), %rsi
.Ltmp10084:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_643
.Ltmp10085:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10086:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm7, %ymm15, %ymm7
	vsubps	%ymm3, %ymm7, %ymm3
.Ltmp10087:
	.loc	1 1084 30 is_stmt 1
	movq	2608(%r12), %rcx
.Ltmp10088:
	.loc	8 551 14
	vmovups	%ymm3, (%rcx,%rax,4)
.Ltmp10089:
	.loc	1 1085 28
	movq	5224(%r12), %rsi
.Ltmp10090:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_644
.Ltmp10091:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10092:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm14, 1920(%rsp)
	vmulps	%ymm9, %ymm10, %ymm3
	vmulps	%ymm2, %ymm13, %ymm7
	vaddps	%ymm7, %ymm3, %ymm13
	vaddps	%ymm13, %ymm8, %ymm3
	vmovaps	1728(%rsp), %ymm14
	vsubps	%ymm14, %ymm3, %ymm15
	vmulps	1600(%rsp), %ymm9, %ymm3
	vmulps	%ymm2, %ymm15, %ymm2
	vaddps	%ymm2, %ymm3, %ymm7
	vaddps	%ymm7, %ymm14, %ymm2
.Ltmp10093:
	.loc	1 1085 28 is_stmt 1
	movq	5216(%r12), %rcx
.Ltmp10094:
	.loc	8 551 14
	vmovups	%ymm2, (%rcx,%rax,4)
.Ltmp10095:
	.loc	1 1086 29
	movq	5240(%r12), %rsi
.Ltmp10096:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_645
.Ltmp10097:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10098:
	.loc	48 0 16 is_stmt 0
	movq	%r9, 2080(%rsp)
	movq	%rdi, 2240(%rsp)
	vaddps	%ymm0, %ymm12, %ymm0
	vsubps	%ymm2, %ymm0, %ymm0
.Ltmp10099:
	.loc	1 1086 29 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp10100:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp10101:
	.loc	1 1089 13
	movq	2592(%r12), %rdi
	movq	2600(%r12), %rsi
	movq	2368(%r12), %rax
.Ltmp10102:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp10103:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp10104:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 1856(%rsp)
	je	.LBB40_472
.Ltmp10105:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB40_652
.Ltmp10106:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10107:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp10108:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
.Ltmp10109:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp10110:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp10111:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10112:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%rdi,%r9,4), %ymm0
.Ltmp10113:
	movq	2608(%r12), %rcx
.Ltmp10114:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm14
.Ltmp10115:
	.loc	1 961 2
	jmp	.LBB40_489
.Ltmp10116:
	.loc	1 0 2 is_stmt 0
.Ltmp10117:
	.p2align	4
.LBB40_472:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB40_665
	.loc	1 0 25 is_stmt 0
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10118:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10119:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_695
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10120:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10121:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_694
	.loc	1 0 25
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10122:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10123:
	.loc	1 955 30
	leaq	3(,%rcx,8), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB40_676
	.loc	1 0 25
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10124:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10125:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_688
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10126:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10127:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 1824(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_683
	.loc	1 0 25
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10128:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10129:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 1664(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_679
	.loc	1 0 25
	movq	2424(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10130:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10131:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_687
.Ltmp10132:
	.loc	1 0 25
	movq	%rcx, 48(%rsp)
.Ltmp10133:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
.Ltmp10134:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp10135:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp10136:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10137:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10138:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 136(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_685
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10139:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10140:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rdx
	movq	%rdx, 24(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_686
	.loc	1 0 25
	movq	%r14, 32(%rsp)
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10141:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10142:
	.loc	1 955 30
	leaq	3(,%rcx,8), %rdx
	movq	%rdx, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_681
	.loc	1 0 25
	movq	%rbx, 56(%rsp)
	movq	%r11, 40(%rsp)
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10143:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10144:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_690
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10145:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10146:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_689
	.loc	1 0 25
	movq	%r8, %r11
	movq	%r13, %rbx
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10147:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movq	%r10, %r8
	movq	%r12, %r10
	movl	$0, %r12d
	cmovaeq	%r13, %r12
	subq	%r12, %rcx
.Ltmp10148:
	.loc	1 955 30
	leaq	6(,%rcx,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	%r15, %r13
	movq	2424(%r10), %rcx
	movq	%r8, %r15
	.loc	1 955 35
	addq	%r8, %rcx
.Ltmp10149:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %r8d
	cmovaeq	%rbx, %r8
	subq	%r8, %rcx
.Ltmp10150:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
.Ltmp10151:
	.loc	1 0 25
	movq	32(%rsp), %rsi
	vmovd	(%rdi,%rsi,4), %xmm0
	movq	1824(%rsp), %rsi
	vpinsrd	$1, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	1664(%rsp), %rsi
	vpinsrd	$2, (%rdi,%rsi,4), %xmm0, %xmm0
	movq	48(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm0, %xmm0
	vmovd	(%rdi,%r9,4), %xmm2
	vpinsrd	$1, (%rdi,%r11,4), %xmm2, %xmm2
	movq	40(%rsp), %rsi
	vpinsrd	$2, (%rdi,%rsi,4), %xmm2, %xmm2
	movq	56(%rsp), %rsi
	vpinsrd	$3, (%rdi,%rsi,4), %xmm2, %xmm2
.Ltmp10152:
	movq	2608(%r10), %rsi
.Ltmp10153:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm3
	vpinsrd	$1, (%rsi,%rdx,4), %xmm3, %xmm3
	vpinsrd	$2, (%rsi,%r12,4), %xmm3, %xmm3
	vpinsrd	$3, (%rsi,%rcx,4), %xmm3, %xmm3
	vmovd	(%rsi,%rax,4), %xmm12
	movq	136(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm12, %xmm12
	movq	24(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm12, %xmm12
	movq	16(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm12, %xmm12
.Ltmp10154:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm2, %ymm0
.Ltmp10155:
	.loc	8 551 14
	vinserti128	$1, %xmm3, %ymm12, %ymm14
	movq	%r10, %r12
	xorl	%r11d, %r11d
	movq	%r15, %r10
	movq	%r13, %r15
	movq	1888(%rsp), %r13
.Ltmp10156:
.LBB40_489:
	.loc	1 1103 13 is_stmt 1
	movq	5216(%r12), %r9
	movq	5224(%r12), %rsi
	movq	4992(%r12), %rax
.Ltmp10157:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp10158:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	movq	%rax, %rdi
	subq	%rcx, %rdi
.Ltmp10159:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rdi
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 2048(%rsp)
	vmovdqa	%ymm0, 1824(%rsp)
	je	.LBB40_495
.Ltmp10160:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rdi, %r8
	jb	.LBB40_653
.Ltmp10161:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10162:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp10163:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
.Ltmp10164:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp10165:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp10166:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10167:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r9,%rdi,4), %ymm2
.Ltmp10168:
	movq	5232(%r12), %rcx
.Ltmp10169:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm0
.Ltmp10170:
	.loc	1 961 2
	jmp	.LBB40_512
.Ltmp10171:
	.loc	1 0 2 is_stmt 0
.Ltmp10172:
	.p2align	4
.LBB40_495:
	.loc	1 955 25 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB40_666
	.loc	1 0 25 is_stmt 0
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10173:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10174:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_695
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10175:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10176:
	.loc	1 955 30
	leaq	2(,%rcx,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_730
	.loc	1 0 25
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10177:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10178:
	.loc	1 955 30
	leaq	3(,%rcx,8), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB40_708
	.loc	1 0 25
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10179:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10180:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_699
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10181:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10182:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 1664(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_700
	.loc	1 0 25
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10183:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10184:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 48(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_729
	.loc	1 0 25
	movq	5048(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10185:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10186:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_706
.Ltmp10187:
	.loc	1 0 25
	movq	%rcx, 136(%rsp)
.Ltmp10188:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
.Ltmp10189:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp10190:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp10191:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10192:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10193:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 24(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_698
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10194:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10195:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rdx
	movq	%rdx, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_681
	.loc	1 0 25
	movq	%r14, 40(%rsp)
	movq	%rbx, 32(%rsp)
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10196:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10197:
	.loc	1 955 30
	leaq	3(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
	.loc	1 0 25
	movq	%r13, %rbx
	movq	%rcx, 160(%rsp)
	movq	%r11, 56(%rsp)
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10198:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10199:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_734
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10200:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edx
	cmovaeq	%rbx, %rdx
	subq	%rdx, %rcx
.Ltmp10201:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_710
	.loc	1 0 25
	movq	%rdi, %r13
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10202:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movq	%r10, %rdi
	movq	%r12, %r10
	movl	$0, %r12d
	cmovaeq	%rbx, %r12
	subq	%r12, %rcx
.Ltmp10203:
	.loc	1 955 30
	leaq	6(,%rcx,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	%r9, %r11
	movq	%r15, %r9
	movq	5048(%r10), %rcx
	movq	%rdi, %r15
	.loc	1 955 35
	addq	%rdi, %rcx
.Ltmp10204:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbx, %rcx
	movl	$0, %edi
	cmovaeq	%rbx, %rdi
	subq	%rdi, %rcx
.Ltmp10205:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
.Ltmp10206:
	.loc	1 0 25
	movq	40(%rsp), %rsi
	vmovd	(%r11,%rsi,4), %xmm0
	movq	1664(%rsp), %rsi
	vpinsrd	$1, (%r11,%rsi,4), %xmm0, %xmm0
	movq	48(%rsp), %rsi
	vpinsrd	$2, (%r11,%rsi,4), %xmm0, %xmm0
	movq	136(%rsp), %rsi
	vpinsrd	$3, (%r11,%rsi,4), %xmm0, %xmm0
	vmovd	(%r11,%r13,4), %xmm2
	vpinsrd	$1, (%r11,%r8,4), %xmm2, %xmm2
	movq	56(%rsp), %rsi
	vpinsrd	$2, (%r11,%rsi,4), %xmm2, %xmm2
	movq	32(%rsp), %rsi
	vpinsrd	$3, (%r11,%rsi,4), %xmm2, %xmm2
.Ltmp10207:
	movq	5232(%r10), %rsi
.Ltmp10208:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm3
	vpinsrd	$1, (%rsi,%rdx,4), %xmm3, %xmm3
	vpinsrd	$2, (%rsi,%r12,4), %xmm3, %xmm3
	vpinsrd	$3, (%rsi,%rcx,4), %xmm3, %xmm3
	vmovd	(%rsi,%rax,4), %xmm12
	movq	24(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm12, %xmm12
	movq	16(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm12, %xmm12
	movq	160(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm12, %xmm12
.Ltmp10209:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm2, %ymm2
.Ltmp10210:
	.loc	8 551 14
	vinserti128	$1, %xmm3, %ymm12, %ymm0
	movq	%r10, %r12
	xorl	%r11d, %r11d
	movq	%r15, %r10
	movq	%r9, %r15
	movq	1888(%rsp), %r13
.Ltmp10211:
.LBB40_512:
	.loc	1 0 0
	negq	%r15
	addq	%r15, %r10
	incq	%r10
	leaq	(,%r10,8), %rax
.Ltmp10212:
	.loc	1 1150 36 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp10213:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	2336(%rsp), %r15
	movq	168(%rsp), %rdx
	movq	2240(%rsp), %rdi
	movq	2080(%rsp), %r9
	jb	.LBB40_647
.Ltmp10214:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10215:
	.loc	1 1152 27
	movq	2616(%r12), %rsi
.Ltmp10216:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_648
.Ltmp10217:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10218:
	.loc	1 1153 35
	movq	5224(%r12), %rsi
.Ltmp10219:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_649
.Ltmp10220:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10221:
	.loc	1 1155 27
	movq	5240(%r12), %rsi
.Ltmp10222:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_638
.Ltmp10223:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10224:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm6, %ymm6, %ymm3
	vaddps	1632(%rsp), %ymm3, %ymm3
	vbroadcastss	.LCPI40_1(%rip), %ymm6
	vandps	%ymm6, %ymm3, %ymm12
	vmovdqa	%ymm14, 1664(%rsp)
	vmovaps	%ymm8, %ymm14
	vmovaps	%ymm10, %ymm8
	vbroadcastss	.LCPI40_2(%rip), %ymm10
	vcmplt_oqps	%ymm10, %ymm12, %ymm12
	vandnps	%ymm3, %ymm12, %ymm3
	vmovaps	%ymm3, 1632(%rsp)
	vaddps	%ymm5, %ymm5, %ymm3
	vaddps	64(%rsp), %ymm3, %ymm3
	vandps	%ymm6, %ymm3, %ymm5
	vcmplt_oqps	%ymm10, %ymm5, %ymm5
	vandnps	%ymm3, %ymm5, %ymm3
	vmovaps	%ymm3, 64(%rsp)
	vmulps	%ymm4, %ymm11, %ymm3
	vmovaps	192(%rsp), %ymm5
	vmulps	2208(%rsp), %ymm5, %ymm4
	vaddps	%ymm3, %ymm4, %ymm3
	vaddps	%ymm3, %ymm3, %ymm3
	vaddps	%ymm3, %ymm5, %ymm3
	vandps	%ymm6, %ymm3, %ymm4
	vcmplt_oqps	%ymm10, %ymm4, %ymm4
	vandnps	%ymm3, %ymm4, %ymm3
	vmovaps	%ymm3, 192(%rsp)
	vmovaps	2176(%rsp), %ymm3
	vaddps	%ymm3, %ymm3, %ymm3
	vaddps	1568(%rsp), %ymm3, %ymm3
	vandps	%ymm6, %ymm3, %ymm4
	vcmplt_oqps	%ymm10, %ymm4, %ymm4
	vandnps	%ymm3, %ymm4, %ymm3
	vmovaps	%ymm3, 1568(%rsp)
.Ltmp10225:
	vaddps	%ymm1, %ymm1, %ymm1
	vaddps	%ymm1, %ymm8, %ymm1
	vandps	%ymm6, %ymm1, %ymm3
	vcmplt_oqps	%ymm10, %ymm3, %ymm3
	vandnps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 1760(%rsp)
	vaddps	%ymm13, %ymm13, %ymm1
	vaddps	%ymm1, %ymm14, %ymm1
	vandps	%ymm6, %ymm1, %ymm3
	vcmplt_oqps	%ymm10, %ymm3, %ymm3
	vandnps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 2144(%rsp)
	vmulps	%ymm15, %ymm9, %ymm1
	vmovaps	1600(%rsp), %ymm4
	vmulps	1920(%rsp), %ymm4, %ymm3
	vaddps	%ymm1, %ymm3, %ymm1
	vaddps	%ymm1, %ymm1, %ymm1
	vaddps	%ymm1, %ymm4, %ymm1
	vandps	%ymm6, %ymm1, %ymm3
	vcmplt_oqps	%ymm10, %ymm3, %ymm3
	vandnps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 1600(%rsp)
	vaddps	%ymm7, %ymm7, %ymm1
	vaddps	1728(%rsp), %ymm1, %ymm1
	vandps	%ymm6, %ymm1, %ymm3
	vcmplt_oqps	%ymm10, %ymm3, %ymm3
	vandnps	%ymm1, %ymm3, %ymm1
	vmovaps	%ymm1, 1728(%rsp)
.Ltmp10226:
	vandps	1824(%rsp), %ymm6, %ymm1
	vpand	%ymm6, %ymm2, %ymm2
	vbroadcastss	.LCPI40_3(%rip), %ymm3
	vmulps	%ymm3, %ymm1, %ymm1
	vmulps	%ymm3, %ymm2, %ymm2
	vaddps	%ymm2, %ymm1, %ymm2
.Ltmp10227:
	vandps	1664(%rsp), %ymm6, %ymm1
	vpand	%ymm6, %ymm0, %ymm0
	vmulps	%ymm3, %ymm1, %ymm1
	vmulps	%ymm3, %ymm0, %ymm0
	vaddps	%ymm0, %ymm1, %ymm1
	vbroadcastss	.LCPI40_4(%rip), %ymm9
.Ltmp10228:
	vmaxps	%ymm9, %ymm2, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm2
	vbroadcastss	.LCPI40_5(%rip), %ymm11
	vmaxps	%ymm11, %ymm0, %ymm0
	vandps	%ymm2, %ymm0, %ymm3
	vbroadcastsd	.LCPI40_7(%rip), %ymm4
	vorps	%ymm4, %ymm3, %ymm3
	vbroadcastss	.LCPI40_8(%rip), %ymm11
	vaddps	%ymm3, %ymm11, %ymm3
	vbroadcastss	.LCPI40_9(%rip), %ymm13
	vmulps	%ymm3, %ymm13, %ymm5
	vbroadcastss	.LCPI40_10(%rip), %ymm14
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_11(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_12(%rip), %ymm7
	vaddps	%ymm7, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_13(%rip), %ymm7
	vaddps	%ymm7, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_14(%rip), %ymm7
	vaddps	%ymm7, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm3
	vpsrld	$23, %ymm0, %ymm0
	vpbroadcastd	.LCPI40_15(%rip), %ymm5
	vpor	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_16(%rip), %ymm12
	vaddps	%ymm0, %ymm12, %ymm0
	vaddps	%ymm3, %ymm0, %ymm0
	vbroadcastss	.LCPI40_17(%rip), %ymm13
	vmulps	%ymm0, %ymm13, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm14
	vmaxps	%ymm14, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm15
	vminps	%ymm15, %ymm0, %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vsubps	256(%rsp), %ymm0, %ymm3
	vbroadcastss	.LCPI40_20(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm7
	vmulps	%ymm7, %ymm7, %ymm7
	vbroadcastss	.LCPI40_22(%rip), %ymm12
	vmulps	%ymm7, %ymm12, %ymm7
	vcmpgt_oqps	%ymm0, %ymm3, %ymm8
	vblendvps	%ymm8, %ymm3, %ymm7, %ymm7
	vbroadcastss	.LCPI40_21(%rip), %ymm9
	vcmple_oqps	%ymm9, %ymm3, %ymm3
	vmulps	2592(%rsp), %ymm7, %ymm7
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm3, %ymm0, %ymm3
	vpandn	%ymm7, %ymm3, %ymm3
	vbroadcastss	.LCPI40_23(%rip), %ymm7
	vmaxps	%ymm7, %ymm3, %ymm3
	vminps	%ymm0, %ymm3, %ymm3
	vmovaps	2112(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm3, %ymm7
	vmovaps	2528(%rsp), %ymm8
	vblendvps	%ymm7, 2560(%rsp), %ymm8, %ymm7
	vsubps	%ymm3, %ymm0, %ymm8
	vmulps	%ymm7, %ymm8, %ymm7
	vaddps	%ymm7, %ymm3, %ymm3
	vandps	%ymm6, %ymm3, %ymm7
	vcmplt_oqps	%ymm10, %ymm7, %ymm7
	vandnps	%ymm3, %ymm7, %ymm0
.Ltmp10229:
	vbroadcastss	.LCPI40_4(%rip), %ymm3
	vmaxps	%ymm3, %ymm1, %ymm1
	vbroadcastss	.LCPI40_5(%rip), %ymm3
	vmaxps	%ymm3, %ymm1, %ymm1
	vandps	%ymm2, %ymm1, %ymm2
	vorps	%ymm4, %ymm2, %ymm2
	vpsrld	$23, %ymm1, %ymm1
	vpor	%ymm5, %ymm1, %ymm1
	vaddps	%ymm2, %ymm11, %ymm2
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
	vbroadcastss	.LCPI40_16(%rip), %ymm3
	vaddps	%ymm3, %ymm1, %ymm1
	vaddps	%ymm2, %ymm1, %ymm1
	vmulps	%ymm1, %ymm13, %ymm1
	vmaxps	%ymm14, %ymm1, %ymm1
	vminps	%ymm15, %ymm1, %ymm1
	vmovaps	%ymm1, 1920(%rsp)
	vsubps	416(%rsp), %ymm1, %ymm1
	vbroadcastss	.LCPI40_20(%rip), %ymm2
	vaddps	%ymm2, %ymm1, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm12, %ymm3
	vcmpgt_oqps	%ymm2, %ymm1, %ymm4
	vblendvps	%ymm4, %ymm1, %ymm3, %ymm3
	vcmple_oqps	%ymm9, %ymm1, %ymm1
	vmulps	2496(%rsp), %ymm3, %ymm3
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%ymm1, %ymm2, %ymm1
	vpandn	%ymm3, %ymm1, %ymm1
	vmovaps	%ymm0, 2112(%rsp)
.Ltmp10230:
	vaddps	384(%rsp), %ymm0, %ymm3
	vbroadcastss	.LCPI40_24(%rip), %ymm8
	vmulps	%ymm3, %ymm8, %ymm3
	vbroadcastss	.LCPI40_25(%rip), %ymm12
	vmaxps	%ymm12, %ymm3, %ymm3
	vbroadcastss	.LCPI40_26(%rip), %ymm13
	vminps	%ymm13, %ymm3, %ymm3
	vroundps	$9, %ymm3, %ymm4
	vsubps	%ymm4, %ymm3, %ymm3
	vbroadcastss	.LCPI40_27(%rip), %ymm14
	vmulps	%ymm3, %ymm14, %ymm5
	vbroadcastss	.LCPI40_28(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_29(%rip), %ymm11
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_30(%rip), %ymm9
	vaddps	%ymm5, %ymm9, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vbroadcastss	.LCPI40_31(%rip), %ymm10
	vaddps	%ymm5, %ymm10, %ymm5
	vbroadcastss	.LCPI40_23(%rip), %ymm0
.Ltmp10231:
	vmaxps	%ymm0, %ymm1, %ymm1
	vminps	%ymm2, %ymm1, %ymm1
	vmovaps	1952(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm1, %ymm7
	vmovaps	2432(%rsp), %ymm8
	vblendvps	%ymm7, 2464(%rsp), %ymm8, %ymm7
.Ltmp10232:
	vmulps	%ymm5, %ymm3, %ymm3
.Ltmp10233:
	vsubps	%ymm1, %ymm0, %ymm5
	vmulps	%ymm7, %ymm5, %ymm5
	vaddps	%ymm5, %ymm1, %ymm1
	vandps	%ymm6, %ymm1, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm15
	vcmplt_oqps	%ymm15, %ymm5, %ymm5
	vandnps	%ymm1, %ymm5, %ymm0
	vbroadcastss	.LCPI40_32(%rip), %ymm7
.Ltmp10234:
	vaddps	%ymm7, %ymm3, %ymm1
	vbroadcastss	.LCPI40_33(%rip), %ymm8
	vaddps	%ymm4, %ymm8, %ymm3
	vpslld	$23, %ymm3, %ymm3
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp10235:
	vaddps	544(%rsp), %ymm0, %ymm4
.Ltmp10236:
	vmulps	%ymm3, %ymm1, %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vbroadcastss	.LCPI40_24(%rip), %ymm2
.Ltmp10237:
	vmulps	%ymm2, %ymm4, %ymm3
	vmaxps	%ymm12, %ymm3, %ymm3
	vminps	%ymm13, %ymm3, %ymm3
	vroundps	$9, %ymm3, %ymm4
	vsubps	%ymm4, %ymm3, %ymm3
	vmulps	%ymm3, %ymm14, %ymm5
	vbroadcastss	.LCPI40_28(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm9, %ymm5
	vmovaps	%ymm9, %ymm14
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm10, %ymm5
	vmulps	%ymm5, %ymm3, %ymm3
	vaddps	%ymm7, %ymm3, %ymm3
	vaddps	%ymm4, %ymm8, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmovaps	2208(%rsp), %ymm0
.Ltmp10238:
	vsubps	896(%rsp), %ymm0, %ymm5
.Ltmp10239:
	vmulps	%ymm4, %ymm3, %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm7
.Ltmp10240:
	vaddps	%ymm7, %ymm5, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vbroadcastss	.LCPI40_22(%rip), %ymm13
	vmulps	%ymm3, %ymm13, %ymm3
	vcmpgt_oqps	%ymm7, %ymm5, %ymm4
	vblendvps	%ymm4, %ymm5, %ymm3, %ymm3
	vbroadcastss	.LCPI40_21(%rip), %ymm12
	vcmple_oqps	%ymm12, %ymm5, %ymm4
	vmulps	2400(%rsp), %ymm3, %ymm3
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%ymm4, %ymm0, %ymm4
	vpandn	%ymm3, %ymm4, %ymm3
	vbroadcastss	.LCPI40_23(%rip), %ymm4
	vmaxps	%ymm4, %ymm3, %ymm3
	vminps	%ymm0, %ymm3, %ymm3
	vxorps	%xmm9, %xmm9, %xmm9
	vmovaps	1984(%rsp), %ymm0
	vcmplt_oqps	%ymm0, %ymm3, %ymm4
	vmovaps	2688(%rsp), %ymm5
	vblendvps	%ymm4, 2720(%rsp), %ymm5, %ymm4
	vsubps	%ymm3, %ymm0, %ymm5
	vmulps	%ymm4, %ymm5, %ymm4
	vaddps	%ymm4, %ymm3, %ymm3
	vandps	%ymm6, %ymm3, %ymm4
	vcmplt_oqps	%ymm15, %ymm4, %ymm4
	vandnps	%ymm3, %ymm4, %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vaddps	1024(%rsp), %ymm0, %ymm3
	vmulps	%ymm2, %ymm3, %ymm3
	vmovaps	%ymm2, %ymm15
	vbroadcastss	.LCPI40_25(%rip), %ymm8
	vmaxps	%ymm8, %ymm3, %ymm3
	vbroadcastss	.LCPI40_26(%rip), %ymm0
	vminps	%ymm0, %ymm3, %ymm3
	vroundps	$9, %ymm3, %ymm4
	vsubps	%ymm4, %ymm3, %ymm3
	vbroadcastss	.LCPI40_27(%rip), %ymm0
	vmulps	%ymm0, %ymm3, %ymm5
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm3, %ymm5
	vaddps	%ymm5, %ymm10, %ymm5
	vmulps	%ymm5, %ymm3, %ymm3
	vbroadcastss	.LCPI40_32(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm3
	vbroadcastss	.LCPI40_33(%rip), %ymm0
	vaddps	%ymm0, %ymm4, %ymm4
	vpslld	$23, %ymm4, %ymm4
	vmulps	%ymm4, %ymm3, %ymm4
	vmovaps	1920(%rsp), %ymm0
.Ltmp10241:
	vsubps	1056(%rsp), %ymm0, %ymm2
	vaddps	%ymm7, %ymm2, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm13, %ymm3
	vcmpgt_oqps	%ymm7, %ymm2, %ymm5
	vblendvps	%ymm5, %ymm2, %ymm3, %ymm3
	vcmple_oqps	%ymm12, %ymm2, %ymm2
	vmulps	2656(%rsp), %ymm3, %ymm3
	vpcmpgtd	%ymm2, %ymm9, %ymm2
	vpandn	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_23(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm2
	vminps	%ymm9, %ymm2, %ymm2
	vmovaps	2368(%rsp), %ymm5
	vcmplt_oqps	%ymm5, %ymm2, %ymm3
	vmovaps	2272(%rsp), %ymm0
	vblendvps	%ymm3, 2624(%rsp), %ymm0, %ymm3
	vsubps	%ymm2, %ymm5, %ymm5
	vmulps	%ymm3, %ymm5, %ymm3
	vaddps	%ymm3, %ymm2, %ymm2
	vandps	%ymm6, %ymm2, %ymm3
	vbroadcastss	.LCPI40_2(%rip), %ymm0
	vcmplt_oqps	%ymm0, %ymm3, %ymm3
	vandnps	%ymm2, %ymm3, %ymm6
	vaddps	1184(%rsp), %ymm6, %ymm2
	vmulps	%ymm2, %ymm15, %ymm2
	vmaxps	%ymm8, %ymm2, %ymm2
	vbroadcastss	.LCPI40_26(%rip), %ymm0
	vminps	%ymm0, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm3
	vsubps	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI40_27(%rip), %ymm0
	vmulps	%ymm0, %ymm2, %ymm5
	vaddps	%ymm1, %ymm5, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vaddps	%ymm5, %ymm11, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vaddps	%ymm5, %ymm14, %ymm5
	vmulps	%ymm5, %ymm2, %ymm5
	vaddps	%ymm5, %ymm10, %ymm5
	vmulps	%ymm5, %ymm2, %ymm2
	vbroadcastss	.LCPI40_32(%rip), %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vbroadcastss	.LCPI40_33(%rip), %ymm0
	vaddps	%ymm0, %ymm3, %ymm3
	vpslld	$23, %ymm3, %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
.Ltmp10242:
	movq	2592(%r12), %rcx
	vmovaps	2176(%rsp), %ymm0
	vmulps	(%rcx,%rax,4), %ymm0, %ymm1
	movq	2608(%r12), %rcx
	vmovaps	2208(%rsp), %ymm0
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
	vaddps	%ymm0, %ymm1, %ymm0
.Ltmp10243:
	movq	5216(%r12), %rcx
	vmulps	(%rcx,%rax,4), %ymm4, %ymm1
	.loc	1 1155 27 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp10244:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm2, %ymm2
.Ltmp10245:
	.loc	14 48 14
	vaddps	%ymm2, %ymm1, %ymm1
	movq	152(%rsp), %rcx
.Ltmp10246:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rdi,4)
.Ltmp10247:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm1, (%rdx,%rdi,4)
.Ltmp10248:
	.loc	1 0 0
	incq	%r9
.Ltmp10249:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r9, %r15
.Ltmp10250:
	.loc	3 900 12
	jne	.LBB40_456
.Ltmp10251:
.LBB40_521:
	.loc	3 0 12 is_stmt 0
	vmovaps	1632(%rsp), %ymm0
	.loc	1 1160 5 is_stmt 1
	vmovaps	%ymm0, 1408(%r12)
	vmovaps	64(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%r12)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%r12)
	vmovaps	1568(%rsp), %ymm0
	vmovaps	%ymm0, 1504(%r12)
	vmovaps	1760(%rsp), %ymm0
	.loc	1 1161 5
	vmovaps	%ymm0, 4032(%r12)
	vmovaps	2144(%rsp), %ymm0
	vmovaps	%ymm0, 4064(%r12)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 4096(%r12)
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 4128(%r12)
	vmovaps	2112(%rsp), %ymm0
	.loc	1 1162 5
	vmovaps	%ymm0, 2304(%r12)
	vmovaps	1952(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%r12)
	vmovaps	1984(%rsp), %ymm0
	.loc	1 1163 5
	vmovaps	%ymm0, 4928(%r12)
	vmovaps	%ymm6, 4960(%r12)
	.loc	1 1164 5
	movq	%r10, 5272(%r12)
	xorl	%eax, %eax
.Ltmp10252:
	.loc	1 0 5 is_stmt 0
.Ltmp10253:
	.p2align	4
.LBB40_522:
	.loc	1 1297 13 is_stmt 1
	vmovss	256(%rsp,%rax,2), %xmm3
	vmovss	260(%rsp,%rax,2), %xmm4
	vmovss	264(%rsp,%rax,2), %xmm5
	vmovss	268(%rsp,%rax,2), %xmm6
	vmovss	272(%rsp,%rax,2), %xmm7
	vmovss	276(%rsp,%rax,2), %xmm2
	vmovss	280(%rsp,%rax,2), %xmm1
	vmovd	284(%rsp,%rax,2), %xmm0
.Ltmp10254:
	.loc	1 1300 17
	vmovss	%xmm3, (%r12,%rax)
	.loc	1 1301 34
	movl	12(%r12,%rax), %ecx
	movl	172(%r12,%rax), %edx
.Ltmp10255:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10256:
	.loc	1 1301 17
	movl	%ecx, 12(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm4, 160(%r12,%rax)
.Ltmp10257:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%r11d, %edx
.Ltmp10258:
	.loc	1 1301 17
	movl	%edx, 172(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm5, 320(%r12,%rax)
	.loc	1 1301 34
	movl	332(%r12,%rax), %ecx
.Ltmp10259:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10260:
	.loc	1 1301 17
	movl	%ecx, 332(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm6, 480(%r12,%rax)
	.loc	1 1301 34
	movl	492(%r12,%rax), %ecx
.Ltmp10261:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10262:
	.loc	1 1301 17
	movl	%ecx, 492(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm7, 640(%r12,%rax)
	.loc	1 1301 34
	movl	652(%r12,%rax), %ecx
.Ltmp10263:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10264:
	.loc	1 1301 17
	movl	%ecx, 652(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 800(%r12,%rax)
	.loc	1 1301 34
	movl	812(%r12,%rax), %ecx
.Ltmp10265:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10266:
	.loc	1 1301 17
	movl	%ecx, 812(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 960(%r12,%rax)
	.loc	1 1301 34
	movl	972(%r12,%rax), %ecx
.Ltmp10267:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10268:
	.loc	1 1301 17
	movl	%ecx, 972(%r12,%rax)
	.loc	1 1300 17
	vmovd	%xmm0, 1120(%r12,%rax)
	.loc	1 1301 34
	movl	1132(%r12,%rax), %ecx
.Ltmp10269:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10270:
	.loc	1 1301 17
	movl	%ecx, 1132(%r12,%rax)
.Ltmp10271:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp10272:
	.loc	3 900 12
	jne	.LBB40_522
.Ltmp10273:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	.p2align	4
.LBB40_524:
.Ltmp10274:
	.loc	1 1297 13 is_stmt 1
	vmovss	896(%rsp,%rax,2), %xmm3
	vmovss	900(%rsp,%rax,2), %xmm4
	vmovss	904(%rsp,%rax,2), %xmm5
	vmovss	908(%rsp,%rax,2), %xmm6
	vmovss	912(%rsp,%rax,2), %xmm7
	vmovss	916(%rsp,%rax,2), %xmm2
	vmovss	920(%rsp,%rax,2), %xmm1
	vmovd	924(%rsp,%rax,2), %xmm0
.Ltmp10275:
	.loc	1 1300 17
	vmovss	%xmm3, 2624(%r12,%rax)
	.loc	1 1301 34
	movl	2636(%r12,%rax), %ecx
	movl	2796(%r12,%rax), %edx
.Ltmp10276:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10277:
	.loc	1 1301 17
	movl	%ecx, 2636(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm4, 2784(%r12,%rax)
.Ltmp10278:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%r11d, %edx
.Ltmp10279:
	.loc	1 1301 17
	movl	%edx, 2796(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm5, 2944(%r12,%rax)
	.loc	1 1301 34
	movl	2956(%r12,%rax), %ecx
.Ltmp10280:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10281:
	.loc	1 1301 17
	movl	%ecx, 2956(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm6, 3104(%r12,%rax)
	.loc	1 1301 34
	movl	3116(%r12,%rax), %ecx
.Ltmp10282:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10283:
	.loc	1 1301 17
	movl	%ecx, 3116(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm7, 3264(%r12,%rax)
	.loc	1 1301 34
	movl	3276(%r12,%rax), %ecx
.Ltmp10284:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10285:
	.loc	1 1301 17
	movl	%ecx, 3276(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 3424(%r12,%rax)
	.loc	1 1301 34
	movl	3436(%r12,%rax), %ecx
.Ltmp10286:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10287:
	.loc	1 1301 17
	movl	%ecx, 3436(%r12,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 3584(%r12,%rax)
	.loc	1 1301 34
	movl	3596(%r12,%rax), %ecx
.Ltmp10288:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10289:
	.loc	1 1301 17
	movl	%ecx, 3596(%r12,%rax)
	.loc	1 1300 17
	vmovd	%xmm0, 3744(%r12,%rax)
	.loc	1 1301 34
	movl	3756(%r12,%rax), %ecx
.Ltmp10290:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%r11d, %ecx
.Ltmp10291:
	.loc	1 1301 17
	movl	%ecx, 3756(%r12,%rax)
.Ltmp10292:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp10293:
	.loc	3 900 12
	jne	.LBB40_524
	jmp	.LBB40_449
.Ltmp10294:
.LBB40_525:
	.loc	38 1050 16
	cmpq	%r13, %rsi
.Ltmp10295:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB40_656
.Ltmp10296:
	.loc	48 451 16 is_stmt 1
	cmpq	128(%rsp), %rsi
	ja	.LBB40_656
.Ltmp10297:
	.loc	48 451 16 is_stmt 0
	cmpq	120(%rsp), %rsi
	ja	.LBB40_659
.Ltmp10298:
	.loc	1 1053 27 is_stmt 1
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
.Ltmp10299:
	.loc	1 1054 26
	vmovaps	4032(%r12), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	4064(%r12), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	4096(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	4128(%r12), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
.Ltmp10300:
	.loc	1 1055 25
	vmovaps	2304(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	2336(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp10301:
	.loc	1 1056 24
	vmovaps	4928(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	4960(%r12), %ymm8
.Ltmp10302:
	.loc	1 1057 24
	movq	5272(%r12), %r10
.Ltmp10303:
	.loc	1 871 17
	movq	2368(%r12), %rax
	movq	2376(%r12), %rcx
.Ltmp10304:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2384(%r12), %rdx
	xorq	%rax, %rdx
	movq	2392(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2400(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	2408(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	2416(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	orq	%rcx, %rdx
	xorq	2424(%r12), %rax
	orq	%rdx, %rax
	sete	1856(%rsp)
.Ltmp10305:
	.loc	1 871 17
	movq	4992(%r12), %rax
	movq	5000(%r12), %rcx
.Ltmp10306:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	5008(%r12), %rdx
	xorq	%rax, %rdx
	movq	5016(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5024(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	movq	5032(%r12), %rsi
	xorq	%rax, %rsi
	orq	%rdx, %rsi
	movq	5040(%r12), %rdx
	xorq	%rax, %rdx
	orq	%rsi, %rdx
	xorq	5048(%r12), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	2048(%rsp)
.Ltmp10307:
	.loc	2 1916 50
	testq	%r15, %r15
.Ltmp10308:
	.loc	3 900 12
	je	.LBB40_448
.Ltmp10309:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,8), %rax
	movq	%rax, 176(%rsp)
	movq	240(%rsp), %rax
	leaq	(%rax,%r13,4), %rcx
	movq	248(%rsp), %rax
	leaq	(%rax,%r13,4), %rdx
.Ltmp10310:
	.loc	48 568 12 is_stmt 1
	movq	%r15, %rsi
	movabsq	$2305843009213693951, %rax
	andq	%rax, %rsi
	movq	%rsi, 184(%rsp)
	xorl	%edi, %edi
	xorl	%r9d, %r9d
	movq	1888(%rsp), %r13
	movq	%rcx, 152(%rsp)
	movq	%rdx, 168(%rsp)
.Ltmp10311:
	.loc	48 0 12 is_stmt 0
.Ltmp10312:
	.p2align	4
.LBB40_530:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp10313:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %r11d
	cmovaeq	%r13, %r11
.Ltmp10314:
	.loc	48 568 12
	cmpq	176(%rsp), %rdi
	ja	.LBB40_640
.Ltmp10315:
	.loc	48 438 16
	cmpq	%r9, 184(%rsp)
	je	.LBB40_641
.Ltmp10316:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,8), %rax
.Ltmp10317:
	.loc	1 1083 29 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp10318:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_642
.Ltmp10319:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10320:
	.loc	48 0 16 is_stmt 0
	vmovaps	%ymm8, 2208(%rsp)
	vmovups	(%rcx,%rdi,4), %ymm1
.Ltmp10321:
	vmovups	(%rdx,%rdi,4), %ymm4
.Ltmp10322:
	vmovaps	1280(%r12), %ymm5
	vmovaps	1312(%r12), %ymm11
	vmovaps	1344(%r12), %ymm0
	vmovaps	3904(%r12), %ymm10
	vmovaps	3936(%r12), %ymm9
	vmovaps	3968(%r12), %ymm6
	vmovaps	1568(%rsp), %ymm7
	vsubps	%ymm7, %ymm1, %ymm2
	vmulps	%ymm2, %ymm11, %ymm3
	vmovaps	1728(%rsp), %ymm8
	vmovaps	%ymm5, 1920(%rsp)
	vmulps	%ymm5, %ymm8, %ymm5
	vaddps	%ymm3, %ymm5, %ymm14
	vaddps	%ymm14, %ymm8, %ymm3
	vmulps	%ymm11, %ymm8, %ymm5
	vmulps	%ymm0, %ymm2, %ymm2
	vaddps	%ymm2, %ymm5, %ymm13
	vaddps	%ymm7, %ymm13, %ymm2
	vmulps	1376(%r12), %ymm3, %ymm15
	vmovaps	1600(%rsp), %ymm3
	vsubps	%ymm3, %ymm2, %ymm12
	vmulps	1632(%rsp), %ymm11, %ymm2
	vmulps	%ymm0, %ymm12, %ymm0
	vaddps	%ymm0, %ymm2, %ymm2
	vaddps	%ymm2, %ymm3, %ymm7
.Ltmp10323:
	vsubps	192(%rsp), %ymm4, %ymm3
	vmulps	%ymm3, %ymm9, %ymm0
	vmovaps	64(%rsp), %ymm8
	vmovaps	%ymm10, 2176(%rsp)
	vmulps	%ymm10, %ymm8, %ymm5
	vaddps	%ymm0, %ymm5, %ymm5
	vaddps	%ymm5, %ymm8, %ymm0
	vmulps	4000(%r12), %ymm0, %ymm0
.Ltmp10324:
	.loc	1 1083 29 is_stmt 1
	movq	2592(%r12), %rcx
.Ltmp10325:
	.loc	8 551 14
	vmovups	%ymm7, (%rcx,%rax,4)
.Ltmp10326:
	.loc	1 1084 30
	movq	2616(%r12), %rsi
.Ltmp10327:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_643
.Ltmp10328:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10329:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm1, %ymm15, %ymm1
	vsubps	%ymm7, %ymm1, %ymm1
.Ltmp10330:
	.loc	1 1084 30 is_stmt 1
	movq	2608(%r12), %rcx
.Ltmp10331:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rax,4)
.Ltmp10332:
	.loc	1 1085 28
	movq	5224(%r12), %rsi
.Ltmp10333:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_644
.Ltmp10334:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10335:
	.loc	1 0 0 is_stmt 0
	vmulps	64(%rsp), %ymm9, %ymm1
	vmulps	%ymm6, %ymm3, %ymm3
	vaddps	%ymm3, %ymm1, %ymm1
	vaddps	192(%rsp), %ymm1, %ymm3
	vmovaps	2144(%rsp), %ymm8
	vsubps	%ymm8, %ymm3, %ymm3
	vmulps	1760(%rsp), %ymm9, %ymm7
	vmulps	%ymm3, %ymm6, %ymm6
	vaddps	%ymm6, %ymm7, %ymm10
	vaddps	%ymm10, %ymm8, %ymm6
.Ltmp10336:
	.loc	1 1085 28 is_stmt 1
	movq	5216(%r12), %rcx
.Ltmp10337:
	.loc	8 551 14
	vmovups	%ymm6, (%rcx,%rax,4)
.Ltmp10338:
	.loc	1 1086 29
	movq	5240(%r12), %rsi
.Ltmp10339:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_645
.Ltmp10340:
	.loc	48 451 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10341:
	.loc	48 0 16 is_stmt 0
	movq	%r9, 2368(%rsp)
	movq	%rdi, 2080(%rsp)
	vaddps	%ymm0, %ymm4, %ymm0
	vsubps	%ymm6, %ymm0, %ymm0
.Ltmp10342:
	.loc	1 1086 29 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp10343:
	.loc	8 551 14
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp10344:
	.loc	1 1089 13
	movq	2592(%r12), %r14
	movq	2600(%r12), %rsi
	movq	2368(%r12), %rax
.Ltmp10345:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp10346:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp10347:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 1856(%rsp)
	movq	%r11, 2240(%rsp)
	je	.LBB40_546
.Ltmp10348:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB40_652
.Ltmp10349:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10350:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp10351:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
.Ltmp10352:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp10353:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp10354:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10355:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r14,%r9,4), %ymm0
.Ltmp10356:
	movq	2608(%r12), %rcx
.Ltmp10357:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm15
.Ltmp10358:
	.loc	1 961 2
	jmp	.LBB40_563
.Ltmp10359:
	.loc	1 0 2 is_stmt 0
.Ltmp10360:
	.p2align	4
.LBB40_546:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB40_665
	.loc	1 0 25 is_stmt 0
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10361:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10362:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdi
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB40_666
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10363:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10364:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB40_676
	.loc	1 0 25
	movq	2392(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10365:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10366:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_680
	.loc	1 0 25
	movq	2400(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10367:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10368:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_695
	.loc	1 0 25
	movq	2408(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10369:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10370:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 48(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_672
	.loc	1 0 25
	movq	2416(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10371:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10372:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 136(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_675
	.loc	1 0 25
	movq	2424(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10373:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10374:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_687
.Ltmp10375:
	.loc	1 0 25
	movq	%rcx, 24(%rsp)
.Ltmp10376:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
.Ltmp10377:
	.loc	1 1096 13
	movq	2616(%r12), %rsi
.Ltmp10378:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp10379:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	%r8, 32(%rsp)
	movq	2376(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10380:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10381:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_681
	.loc	1 0 25
	movq	2384(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10382:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10383:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_692
	.loc	1 0 25
	movq	%r10, %r8
	movq	2392(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp10384:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rdx
	movl	$0, %r11d
	cmovaeq	%r13, %r11
	subq	%r11, %rdx
.Ltmp10385:
	.loc	1 955 30
	leaq	3(,%rdx,8), %r11
	movq	%r11, 40(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_670
	.loc	1 0 25
	movq	%r14, %r10
	movq	2400(%r12), %rdx
	.loc	1 955 35
	addq	%r8, %rdx
.Ltmp10386:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rdx
	movl	$0, %r11d
	cmovaeq	%r13, %r11
	subq	%r11, %rdx
.Ltmp10387:
	.loc	1 955 30
	leaq	4(,%rdx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_690
	.loc	1 0 25
	movq	%r15, 160(%rsp)
	movq	%rbx, 56(%rsp)
	movq	2408(%r12), %rdx
	.loc	1 955 35
	addq	%r8, %rdx
.Ltmp10388:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rdx
	movl	$0, %r11d
	cmovaeq	%r13, %r11
	subq	%r11, %rdx
.Ltmp10389:
	.loc	1 955 30
	leaq	5(,%rdx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_689
	.loc	1 0 25
	movq	%rcx, 1560(%rsp)
	movq	2416(%r12), %r11
	.loc	1 955 35
	addq	%r8, %r11
.Ltmp10390:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %r11
	movq	%r12, %r15
	movl	$0, %r12d
	cmovaeq	%r13, %r12
	subq	%r12, %r11
.Ltmp10391:
	.loc	1 955 30
	leaq	6(,%r11,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_696
	.loc	1 0 25
	movq	%r13, %rbx
	movq	2424(%r15), %r11
	movq	%r8, %rcx
	.loc	1 955 35
	addq	%r8, %r11
.Ltmp10392:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %r11
	movl	$0, %r8d
	cmovaeq	%r13, %r8
	subq	%r8, %r11
.Ltmp10393:
	.loc	1 955 30
	leaq	7(,%r11,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_682
.Ltmp10394:
	.loc	1 0 25
	movq	%rbx, %r13
	movq	32(%rsp), %rsi
	vmovd	(%r10,%rsi,4), %xmm0
	movq	48(%rsp), %rsi
	vpinsrd	$1, (%r10,%rsi,4), %xmm0, %xmm0
	movq	136(%rsp), %rsi
	vpinsrd	$2, (%r10,%rsi,4), %xmm0, %xmm0
	movq	24(%rsp), %rsi
	vpinsrd	$3, (%r10,%rsi,4), %xmm0, %xmm0
	vmovd	(%r10,%r9,4), %xmm4
	vpinsrd	$1, (%r10,%rdi,4), %xmm4, %xmm4
	movq	56(%rsp), %rsi
	vpinsrd	$2, (%r10,%rsi,4), %xmm4, %xmm4
	movq	160(%rsp), %rsi
	vpinsrd	$3, (%r10,%rsi,4), %xmm4, %xmm4
.Ltmp10395:
	movq	2608(%r15), %rsi
.Ltmp10396:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm6
	vpinsrd	$1, (%rsi,%rdx,4), %xmm6, %xmm6
	vpinsrd	$2, (%rsi,%r12,4), %xmm6, %xmm6
	vpinsrd	$3, (%rsi,%r11,4), %xmm6, %xmm6
	vmovd	(%rsi,%rax,4), %xmm7
	movq	16(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm7, %xmm7
	movq	1560(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm7, %xmm7
	movq	40(%rsp), %rax
	vpinsrd	$3, (%rsi,%rax,4), %xmm7, %xmm7
.Ltmp10397:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm4, %ymm0
.Ltmp10398:
	.loc	8 551 14
	vinserti128	$1, %xmm6, %ymm7, %ymm15
	movq	%r15, %r12
	movq	2336(%rsp), %r15
	movq	2240(%rsp), %r11
	movq	%rcx, %r10
.Ltmp10399:
.LBB40_563:
	.loc	1 1103 13 is_stmt 1
	movq	5216(%r12), %r9
	movq	5224(%r12), %rsi
	movq	4992(%r12), %rax
.Ltmp10400:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp10401:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	movq	%rax, %rdi
	subq	%rcx, %rdi
.Ltmp10402:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rdi
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 2048(%rsp)
	vmovaps	%ymm9, 1824(%rsp)
	vmovaps	%ymm10, 1664(%rsp)
	je	.LBB40_569
.Ltmp10403:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rdi, %r8
	jb	.LBB40_653
.Ltmp10404:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10405:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp10406:
	.loc	1 857 8
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
.Ltmp10407:
	.loc	1 948 35
	shlq	$3, %rax
.Ltmp10408:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_646
.Ltmp10409:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10410:
	.loc	48 0 16 is_stmt 0
	vmovdqa	%ymm0, %ymm10
	vmovdqu	(%r9,%rdi,4), %ymm9
.Ltmp10411:
	movq	5232(%r12), %rcx
.Ltmp10412:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %ymm6
.Ltmp10413:
	.loc	1 961 2
	jmp	.LBB40_586
.Ltmp10414:
	.loc	1 0 2 is_stmt 0
.Ltmp10415:
	.p2align	4
.LBB40_569:
	.loc	1 955 25 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB40_666
	.loc	1 0 25 is_stmt 0
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10416:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10417:
	.loc	1 955 30
	leaq	1(,%rcx,8), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB40_697
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10418:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10419:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB40_708
	.loc	1 0 25
	movq	5016(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10420:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10421:
	.loc	1 955 30
	leaq	3(,%rcx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_699
	.loc	1 0 25
	movq	5024(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10422:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10423:
	.loc	1 955 30
	leaq	4(,%rcx,8), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB40_703
	.loc	1 0 25
	movq	5032(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10424:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10425:
	.loc	1 955 30
	leaq	5(,%rcx,8), %rdx
	movq	%rdx, 48(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_729
	.loc	1 0 25
	movq	5040(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10426:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10427:
	.loc	1 955 30
	leaq	6(,%rcx,8), %rdx
	movq	%rdx, 136(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_756
	.loc	1 0 25
	movq	5048(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10428:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10429:
	.loc	1 955 30
	leaq	7(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_706
.Ltmp10430:
	.loc	1 0 25
	movq	%rcx, 24(%rsp)
.Ltmp10431:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
.Ltmp10432:
	.loc	1 1110 13
	movq	5240(%r12), %rsi
.Ltmp10433:
	.loc	1 857 8
	subq	%rcx, %rax
.Ltmp10434:
	.loc	1 955 30
	shlq	$3, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB40_763
	.loc	1 0 25
	movq	5000(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10435:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10436:
	.loc	1 955 30
	leaq	1(,%rcx,8), %rdx
	movq	%rdx, 16(%rsp)
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_681
	.loc	1 0 25
	movq	5008(%r12), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp10437:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rcx
	movl	$0, %edx
	cmovaeq	%r13, %rdx
	subq	%rdx, %rcx
.Ltmp10438:
	.loc	1 955 30
	leaq	2(,%rcx,8), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB40_727
	.loc	1 0 25
	movq	%rcx, 160(%rsp)
	movq	%r15, 56(%rsp)
	movq	%r14, 40(%rsp)
	movq	%rbx, 32(%rsp)
	movq	5016(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp10439:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rdx
	movl	$0, %r11d
	cmovaeq	%r13, %r11
	subq	%r11, %rdx
.Ltmp10440:
	.loc	1 955 30
	leaq	3(,%rdx,8), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB40_724
	.loc	1 0 25
	movq	%rdi, 1560(%rsp)
	movq	5024(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp10441:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rdx
	movl	$0, %r11d
	cmovaeq	%r13, %r11
	subq	%r11, %rdx
.Ltmp10442:
	.loc	1 955 30
	leaq	4(,%rdx,8), %r14
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB40_690
	.loc	1 0 25
	movq	%r10, %rcx
	movq	5032(%r12), %rdx
	.loc	1 955 35
	addq	%r10, %rdx
.Ltmp10443:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %rdx
	movl	$0, %r11d
	cmovaeq	%r13, %r11
	subq	%r11, %rdx
.Ltmp10444:
	.loc	1 955 30
	leaq	5(,%rdx,8), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB40_710
	.loc	1 0 25
	movq	%r9, %r15
	movq	%r8, %r9
	movq	%r15, %r8
	movq	5040(%r12), %r11
	.loc	1 955 35
	addq	%rcx, %r11
.Ltmp10445:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %r11
	movq	%r12, %r15
	movl	$0, %r12d
	cmovaeq	%r13, %r12
	subq	%r12, %r11
.Ltmp10446:
	.loc	1 955 30
	leaq	6(,%r11,8), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB40_737
	.loc	1 0 25
	movq	5048(%r15), %r11
	.loc	1 955 35
	addq	%rcx, %r11
.Ltmp10447:
	.loc	1 857 8 is_stmt 1
	cmpq	%r13, %r11
	movl	$0, %edi
	cmovaeq	%r13, %rdi
	subq	%rdi, %r11
.Ltmp10448:
	.loc	1 955 30
	leaq	7(,%r11,8), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB40_764
.Ltmp10449:
	.loc	1 0 25
	vmovdqa	%ymm0, %ymm10
	movq	%rcx, %r10
	movq	56(%rsp), %rsi
	vmovd	(%r8,%rsi,4), %xmm0
	movq	48(%rsp), %rsi
	vpinsrd	$1, (%r8,%rsi,4), %xmm0, %xmm0
	movq	136(%rsp), %rsi
	vpinsrd	$2, (%r8,%rsi,4), %xmm0, %xmm0
	movq	24(%rsp), %rsi
	vpinsrd	$3, (%r8,%rsi,4), %xmm0, %xmm0
	movq	1560(%rsp), %rsi
	vmovd	(%r8,%rsi,4), %xmm4
	vpinsrd	$1, (%r8,%r9,4), %xmm4, %xmm4
	movq	32(%rsp), %rsi
	vpinsrd	$2, (%r8,%rsi,4), %xmm4, %xmm4
	movq	40(%rsp), %rsi
	vpinsrd	$3, (%r8,%rsi,4), %xmm4, %xmm4
.Ltmp10450:
	movq	5232(%r15), %rsi
.Ltmp10451:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rsi,%r14,4), %xmm6
	vpinsrd	$1, (%rsi,%rdx,4), %xmm6, %xmm6
	vpinsrd	$2, (%rsi,%r12,4), %xmm6, %xmm6
	vpinsrd	$3, (%rsi,%r11,4), %xmm6, %xmm7
	vmovd	(%rsi,%rax,4), %xmm6
	movq	16(%rsp), %rax
	vpinsrd	$1, (%rsi,%rax,4), %xmm6, %xmm6
	movq	160(%rsp), %rax
	vpinsrd	$2, (%rsi,%rax,4), %xmm6, %xmm6
	vpinsrd	$3, (%rsi,%rbx,4), %xmm6, %xmm8
.Ltmp10452:
	.loc	1 0 0 is_stmt 0
	vinserti128	$1, %xmm0, %ymm4, %ymm9
.Ltmp10453:
	.loc	8 551 14
	vinserti128	$1, %xmm7, %ymm8, %ymm6
	movq	%r15, %r12
	movq	2336(%rsp), %r15
	movq	2240(%rsp), %r11
.Ltmp10454:
.LBB40_586:
	.loc	1 0 0
	negq	%r11
	addq	%r11, %r10
	incq	%r10
	leaq	(,%r10,8), %rax
.Ltmp10455:
	.loc	1 1150 36 is_stmt 1
	movq	2600(%r12), %rsi
.Ltmp10456:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	168(%rsp), %rdx
	movq	2080(%rsp), %rdi
	movq	2368(%rsp), %r9
	jb	.LBB40_647
.Ltmp10457:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10458:
	.loc	1 1152 27
	movq	2616(%r12), %rsi
.Ltmp10459:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_648
.Ltmp10460:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10461:
	.loc	1 1153 35
	movq	5224(%r12), %rsi
.Ltmp10462:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_649
.Ltmp10463:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10464:
	.loc	1 1155 27
	movq	5240(%r12), %rsi
.Ltmp10465:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB40_638
.Ltmp10466:
	.loc	48 438 16
	cmpq	$7, %r8
	jbe	.LBB40_631
.Ltmp10467:
	.loc	1 0 0 is_stmt 0
	vaddps	%ymm14, %ymm14, %ymm0
	vaddps	1728(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_1(%rip), %ymm14
	vandps	%ymm0, %ymm14, %ymm7
	vbroadcastss	.LCPI40_2(%rip), %ymm8
	vmovaps	%ymm1, %ymm4
	vmovaps	%ymm8, %ymm1
	vcmplt_oqps	%ymm8, %ymm7, %ymm7
	vandnps	%ymm0, %ymm7, %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vaddps	%ymm13, %ymm13, %ymm0
	vaddps	1568(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm7
	vcmplt_oqps	%ymm8, %ymm7, %ymm7
	vandnps	%ymm0, %ymm7, %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmulps	%ymm12, %ymm11, %ymm0
	vmovaps	1632(%rsp), %ymm8
	vmulps	1920(%rsp), %ymm8, %ymm7
	vaddps	%ymm0, %ymm7, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	%ymm0, %ymm8, %ymm0
	vandps	%ymm0, %ymm14, %ymm7
	vcmplt_oqps	%ymm1, %ymm7, %ymm7
	vandnps	%ymm0, %ymm7, %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vaddps	%ymm2, %ymm2, %ymm0
	vaddps	1600(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm2
	vmovaps	%ymm1, %ymm8
	vcmplt_oqps	%ymm1, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 1600(%rsp)
.Ltmp10468:
	vaddps	%ymm5, %ymm5, %ymm0
	vaddps	64(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm2
	vcmplt_oqps	%ymm1, %ymm2, %ymm2
	vandnps	%ymm0, %ymm2, %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vaddps	%ymm4, %ymm4, %ymm0
	vaddps	192(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm1
	vcmplt_oqps	%ymm8, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmulps	1824(%rsp), %ymm3, %ymm0
	vmovaps	1760(%rsp), %ymm2
	vmulps	2176(%rsp), %ymm2, %ymm1
	vaddps	%ymm0, %ymm1, %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	%ymm0, %ymm2, %ymm0
	vandps	%ymm0, %ymm14, %ymm1
	vcmplt_oqps	%ymm8, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1664(%rsp), %ymm0
	vaddps	%ymm0, %ymm0, %ymm0
	vaddps	2144(%rsp), %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm1
	vcmplt_oqps	%ymm8, %ymm1, %ymm1
	vandnps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm0, 2144(%rsp)
.Ltmp10469:
	vpand	%ymm14, %ymm10, %ymm0
	vpand	%ymm14, %ymm9, %ymm1
	vbroadcastss	.LCPI40_3(%rip), %ymm3
	vmulps	%ymm3, %ymm0, %ymm0
	vmulps	%ymm3, %ymm1, %ymm1
	vaddps	%ymm1, %ymm0, %ymm0
.Ltmp10470:
	vpand	%ymm14, %ymm15, %ymm1
	vpand	%ymm6, %ymm14, %ymm2
	vmulps	%ymm3, %ymm1, %ymm1
	vmulps	%ymm3, %ymm2, %ymm2
	vaddps	%ymm2, %ymm1, %ymm2
	vbroadcastss	.LCPI40_4(%rip), %ymm9
.Ltmp10471:
	vmaxps	%ymm9, %ymm0, %ymm0
	vbroadcastsd	.LCPI40_6(%rip), %ymm3
	vbroadcastss	.LCPI40_5(%rip), %ymm10
	vmaxps	%ymm10, %ymm0, %ymm0
	vandps	%ymm3, %ymm0, %ymm1
	vbroadcastsd	.LCPI40_7(%rip), %ymm4
	vorps	%ymm4, %ymm1, %ymm1
	vbroadcastss	.LCPI40_8(%rip), %ymm11
	vaddps	%ymm1, %ymm11, %ymm1
	vbroadcastss	.LCPI40_9(%rip), %ymm12
	vmulps	%ymm1, %ymm12, %ymm5
	vbroadcastss	.LCPI40_10(%rip), %ymm13
	vaddps	%ymm5, %ymm13, %ymm5
	vmulps	%ymm5, %ymm1, %ymm5
	vbroadcastss	.LCPI40_11(%rip), %ymm15
	vaddps	%ymm5, %ymm15, %ymm5
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
	vbroadcastss	.LCPI40_17(%rip), %ymm12
	vmulps	%ymm0, %ymm12, %ymm0
	vbroadcastss	.LCPI40_18(%rip), %ymm13
	vmaxps	%ymm13, %ymm0, %ymm0
	vbroadcastss	.LCPI40_19(%rip), %ymm15
	vminps	%ymm15, %ymm0, %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vsubps	256(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_20(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm6
	vmulps	%ymm6, %ymm6, %ymm6
	vbroadcastss	.LCPI40_22(%rip), %ymm12
	vmulps	%ymm6, %ymm12, %ymm6
	vcmpgt_oqps	%ymm1, %ymm0, %ymm7
	vblendvps	%ymm7, %ymm0, %ymm6, %ymm6
	vbroadcastss	.LCPI40_21(%rip), %ymm11
	vcmple_oqps	%ymm11, %ymm0, %ymm0
	vmulps	2592(%rsp), %ymm6, %ymm6
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm0, %ymm1, %ymm0
	vpandn	%ymm6, %ymm0, %ymm0
	vbroadcastss	.LCPI40_23(%rip), %ymm6
	vmaxps	%ymm6, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	2112(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm6
	vmovaps	2528(%rsp), %ymm7
	vblendvps	%ymm6, 2560(%rsp), %ymm7, %ymm6
	vsubps	%ymm0, %ymm1, %ymm7
	vmulps	%ymm6, %ymm7, %ymm6
	vaddps	%ymm6, %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm6
	vcmplt_oqps	%ymm8, %ymm6, %ymm6
	vandnps	%ymm0, %ymm6, %ymm6
.Ltmp10472:
	vbroadcastss	.LCPI40_4(%rip), %ymm0
	vmaxps	%ymm0, %ymm2, %ymm0
	vmaxps	%ymm10, %ymm0, %ymm0
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
	vbroadcastss	.LCPI40_17(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vmaxps	%ymm13, %ymm0, %ymm0
	vminps	%ymm15, %ymm0, %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vsubps	416(%rsp), %ymm0, %ymm0
	vbroadcastss	.LCPI40_20(%rip), %ymm1
	vaddps	%ymm1, %ymm0, %ymm2
	vmulps	%ymm2, %ymm2, %ymm2
	vmulps	%ymm2, %ymm12, %ymm2
	vcmpgt_oqps	%ymm1, %ymm0, %ymm4
	vblendvps	%ymm4, %ymm0, %ymm2, %ymm2
	vcmple_oqps	%ymm11, %ymm0, %ymm0
	vmulps	2496(%rsp), %ymm2, %ymm2
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm0, %ymm1, %ymm0
	vpandn	%ymm2, %ymm0, %ymm0
	vmovaps	%ymm6, 2112(%rsp)
.Ltmp10473:
	vaddps	384(%rsp), %ymm6, %ymm2
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
	vbroadcastss	.LCPI40_28(%rip), %ymm13
	vaddps	%ymm5, %ymm13, %ymm5
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
.Ltmp10474:
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vmovaps	1952(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm6
	vmovaps	2432(%rsp), %ymm7
	vblendvps	%ymm6, 2464(%rsp), %ymm7, %ymm6
.Ltmp10475:
	vmulps	%ymm5, %ymm2, %ymm2
.Ltmp10476:
	vsubps	%ymm0, %ymm1, %ymm5
	vmulps	%ymm6, %ymm5, %ymm5
	vaddps	%ymm5, %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm5
	vbroadcastss	.LCPI40_2(%rip), %ymm11
	vcmplt_oqps	%ymm11, %ymm5, %ymm5
	vandnps	%ymm0, %ymm5, %ymm1
	vbroadcastss	.LCPI40_32(%rip), %ymm6
.Ltmp10477:
	vaddps	%ymm6, %ymm2, %ymm0
	vbroadcastss	.LCPI40_33(%rip), %ymm7
	vaddps	%ymm7, %ymm4, %ymm2
	vpslld	$23, %ymm2, %ymm2
	vmovaps	%ymm1, 1952(%rsp)
.Ltmp10478:
	vaddps	544(%rsp), %ymm1, %ymm4
.Ltmp10479:
	vmulps	%ymm2, %ymm0, %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vbroadcastss	.LCPI40_24(%rip), %ymm2
.Ltmp10480:
	vmulps	%ymm2, %ymm4, %ymm0
	vmaxps	%ymm8, %ymm0, %ymm0
	vmovaps	%ymm8, %ymm13
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
	vmovaps	2240(%rsp), %ymm1
.Ltmp10481:
	vsubps	896(%rsp), %ymm1, %ymm5
.Ltmp10482:
	vmulps	%ymm4, %ymm0, %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vbroadcastss	.LCPI40_20(%rip), %ymm6
.Ltmp10483:
	vaddps	%ymm6, %ymm5, %ymm0
	vmulps	%ymm0, %ymm0, %ymm0
	vbroadcastss	.LCPI40_22(%rip), %ymm9
	vmulps	%ymm0, %ymm9, %ymm0
	vcmpgt_oqps	%ymm6, %ymm5, %ymm4
	vblendvps	%ymm4, %ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI40_21(%rip), %ymm10
	vcmple_oqps	%ymm10, %ymm5, %ymm4
	vmulps	2400(%rsp), %ymm0, %ymm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%ymm4, %ymm1, %ymm4
	vpandn	%ymm0, %ymm4, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm1, %ymm0, %ymm0
	vxorps	%xmm7, %xmm7, %xmm7
	vmovaps	1984(%rsp), %ymm1
	vcmplt_oqps	%ymm1, %ymm0, %ymm4
	vmovaps	2688(%rsp), %ymm5
	vblendvps	%ymm4, 2720(%rsp), %ymm5, %ymm4
	vsubps	%ymm0, %ymm1, %ymm5
	vmulps	%ymm4, %ymm5, %ymm4
	vaddps	%ymm4, %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm4
	vcmplt_oqps	%ymm11, %ymm4, %ymm4
	vandnps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vaddps	1024(%rsp), %ymm0, %ymm0
	vmulps	%ymm2, %ymm0, %ymm0
	vmaxps	%ymm13, %ymm0, %ymm0
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
	vmovaps	1920(%rsp), %ymm0
.Ltmp10484:
	vsubps	1056(%rsp), %ymm0, %ymm0
	vaddps	%ymm6, %ymm0, %ymm3
	vmulps	%ymm3, %ymm3, %ymm3
	vmulps	%ymm3, %ymm9, %ymm3
	vcmpgt_oqps	%ymm6, %ymm0, %ymm5
	vblendvps	%ymm5, %ymm0, %ymm3, %ymm3
	vcmple_oqps	%ymm10, %ymm0, %ymm0
	vmulps	2656(%rsp), %ymm3, %ymm3
	vpcmpgtd	%ymm0, %ymm7, %ymm0
	vpandn	%ymm3, %ymm0, %ymm0
	vmaxps	%ymm12, %ymm0, %ymm0
	vminps	%ymm7, %ymm0, %ymm0
	vmovaps	2208(%rsp), %ymm5
	vcmplt_oqps	%ymm5, %ymm0, %ymm3
	vmovaps	2272(%rsp), %ymm1
	vblendvps	%ymm3, 2624(%rsp), %ymm1, %ymm3
	vsubps	%ymm0, %ymm5, %ymm5
	vmulps	%ymm3, %ymm5, %ymm3
	vaddps	%ymm3, %ymm0, %ymm0
	vandps	%ymm0, %ymm14, %ymm3
	vbroadcastss	.LCPI40_2(%rip), %ymm1
	vcmplt_oqps	%ymm1, %ymm3, %ymm3
	vandnps	%ymm0, %ymm3, %ymm8
	vaddps	1184(%rsp), %ymm8, %ymm0
	vmulps	%ymm2, %ymm0, %ymm0
	vmaxps	%ymm13, %ymm0, %ymm0
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
.Ltmp10485:
	movq	2592(%r12), %rcx
	vmovaps	2176(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm2
	movq	2608(%r12), %rcx
	vmovaps	2240(%rsp), %ymm1
	vmulps	(%rcx,%rax,4), %ymm1, %ymm1
	vaddps	%ymm1, %ymm2, %ymm1
.Ltmp10486:
	movq	5216(%r12), %rcx
	vmulps	(%rcx,%rax,4), %ymm4, %ymm2
	.loc	1 1155 27 is_stmt 1
	movq	5232(%r12), %rcx
.Ltmp10487:
	.loc	14 283 14
	vmulps	(%rcx,%rax,4), %ymm0, %ymm0
.Ltmp10488:
	.loc	14 48 14
	vaddps	%ymm0, %ymm2, %ymm0
	movq	152(%rsp), %rcx
.Ltmp10489:
	.loc	8 551 14
	vmovups	%ymm1, (%rcx,%rdi,4)
.Ltmp10490:
	.loc	8 551 14 is_stmt 0
	vmovups	%ymm0, (%rdx,%rdi,4)
.Ltmp10491:
	.loc	1 0 0
	incq	%r9
.Ltmp10492:
	.loc	2 1916 50 is_stmt 1
	addq	$8, %rdi
	cmpq	%r9, %r15
.Ltmp10493:
	.loc	3 900 12
	jne	.LBB40_530
	jmp	.LBB40_448
.Ltmp10494:
.LBB40_595:
	.loc	14 871 14
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
	movabsq	$2305843009213693951, %r11
.Ltmp10495:
	.loc	37 1851 23
	addq	$-7, %r11
	movq	128(%rsp), %rax
	vmovaps	%ymm0, %ymm1
.Ltmp10496:
	.loc	10 2155 12
	andq	%r11, %rax
	movq	240(%rsp), %r8
	je	.LBB40_598
.Ltmp10497:
	.loc	10 0 12 is_stmt 0
	vbroadcastss	.LCPI40_1(%rip), %ymm2
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI40_34(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB40_597:
.Ltmp10498:
	.loc	14 82 19 is_stmt 1
	vandps	(%r8,%rcx,4), %ymm2, %ymm4
.Ltmp10499:
	.loc	14 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp10500:
	.loc	14 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp10501:
	.loc	10 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB40_597
.Ltmp10502:
.LBB40_598:
	.loc	16 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
	movq	120(%rsp), %rsi
	movq	248(%rsp), %rdi
.Ltmp10503:
	.loc	50 208 8
	jae	.LBB40_603
.Ltmp10504:
	.loc	10 2155 12
	movq	%rsi, %rcx
	vmovaps	%ymm0, %ymm1
	andq	%r11, %rcx
	je	.LBB40_602
.Ltmp10505:
	.loc	10 0 12 is_stmt 0
	vbroadcastss	.LCPI40_1(%rip), %ymm2
	xorl	%edx, %edx
	vbroadcastss	.LCPI40_34(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB40_601:
.Ltmp10506:
	.loc	14 82 19 is_stmt 1
	vandps	(%rdi,%rdx,4), %ymm2, %ymm4
.Ltmp10507:
	.loc	14 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp10508:
	.loc	14 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp10509:
	.loc	10 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB40_601
.Ltmp10510:
.LBB40_602:
	.loc	16 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp10511:
	.loc	50 208 34
	jb	.LBB40_622
.LBB40_603:
	.loc	50 0 34 is_stmt 0
	vmovaps	%ymm0, %ymm1
.Ltmp10512:
	.loc	10 2155 12 is_stmt 1
	testq	%rax, %rax
.Ltmp10513:
	.loc	10 2155 12 is_stmt 0
	je	.LBB40_606
.Ltmp10514:
	.loc	10 0 12
	vbroadcastss	.LCPI40_1(%rip), %ymm2
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI40_34(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB40_605:
.Ltmp10515:
	.loc	14 82 19 is_stmt 1
	vandps	(%r8,%rcx,4), %ymm2, %ymm4
.Ltmp10516:
	.loc	14 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp10517:
	.loc	14 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp10518:
	.loc	10 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB40_605
.Ltmp10519:
.LBB40_606:
	.loc	14 585 19
	vpsrad	$31, %ymm1, %ymm2
	vpbroadcastd	.LCPI40_32(%rip), %ymm1
	vpandn	%ymm1, %ymm2, %ymm2
.Ltmp10520:
	.loc	50 185 12
	vmovd	%xmm2, %eax
	xorl	%ebx, %ebx
	testl	%eax, %eax
	setne	%bl
	vpextrd	$1, %xmm2, %ecx
	xorl	%eax, %eax
	testl	%ecx, %ecx
	setne	%al
	vpextrd	$2, %xmm2, %ecx
	addl	%eax, %eax
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	setne	%r14b
	shll	$2, %r14d
	vpextrd	$3, %xmm2, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	shll	$3, %edx
	vextracti128	$1, %ymm2, %xmm2
	vmovd	%xmm2, %ecx
	xorl	%r15d, %r15d
	testl	%ecx, %ecx
	setne	%r15b
	shll	$4, %r15d
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
.Ltmp10521:
	.loc	10 2155 12
	andq	%rsi, %r11
	movl	%ebx, 64(%rsp)
	movl	%r14d, 192(%rsp)
	movl	%r15d, 1568(%rsp)
	je	.LBB40_609
.Ltmp10522:
	.loc	10 0 12 is_stmt 0
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI40_1(%rip), %ymm2
	vbroadcastss	.LCPI40_34(%rip), %ymm3
	.p2align	4
.LBB40_608:
.Ltmp10523:
	.loc	14 82 19 is_stmt 1
	vandps	(%rdi,%rcx,4), %ymm2, %ymm4
.Ltmp10524:
	.loc	14 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp10525:
	.loc	14 82 19
	vandps	%ymm4, %ymm0, %ymm0
.Ltmp10526:
	.loc	10 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %r11
	jne	.LBB40_608
.Ltmp10527:
.LBB40_609:
	.loc	14 585 19
	vpsrad	$31, %ymm0, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
.Ltmp10528:
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
.Ltmp10529:
	.loc	50 185 12 is_stmt 0
	orl	%esi, %ecx
.Ltmp10530:
	.loc	50 185 12
	orl	64(%rsp), %eax
	orl	192(%rsp), %eax
	orl	1568(%rsp), %edx
	orl	%r9d, %edx
	orl	%eax, %edx
	orl	%r10d, %r8d
	orl	%edx, %r8d
.Ltmp10531:
	.loc	50 185 12
	orl	%r11d, %r8d
	orl	%ebx, %r8d
	orl	%r14d, %r15d
	orl	%r8d, %r15d
	orl	%r12d, %r13d
	orl	%r15d, %r13d
.Ltmp10532:
	.loc	50 211 5 is_stmt 1
	orl	%ecx, %r13d
	movq	1720(%rsp), %rbx
	movl	%r13d, 5256(%rbx)
	.loc	50 212 31
	movq	5248(%rbx), %rax
.Ltmp10533:
	.loc	38 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp10534:
	.loc	50 212 5
	movq	%rcx, 5248(%rbx)
	movq	128(%rsp), %rdx
.Ltmp10535:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp10536:
	.loc	33 180 28
	je	.LBB40_611
.Ltmp10537:
	.loc	34 961 18
	shlq	$2, %rdx
	movq	240(%rsp), %rdi
.Ltmp10538:
	.loc	35 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp10539:
.LBB40_611:
	.loc	35 0 13 is_stmt 0
	movq	120(%rsp), %rdx
.Ltmp10540:
	.loc	32 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp10541:
	.loc	33 180 28
	je	.LBB40_613
.Ltmp10542:
	.loc	34 961 18
	shlq	$2, %rdx
	movq	248(%rsp), %rdi
.Ltmp10543:
	.loc	35 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp10544:
.LBB40_613:
	.loc	1 1422 13
	movq	$0, 5272(%rbx)
.Ltmp10545:
	.loc	1 1424 22
	movq	%rbx, %rdi
	vzeroupper
	callq	_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E19discontinuity_resetB5_
.Ltmp10546:
	.loc	32 1714 9
	leaq	2624(%rbx), %rdi
.Ltmp10547:
	.loc	1 1424 22
	callq	_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_E19discontinuity_resetB5_
.Ltmp10548:
	.loc	1 1430 20
	movl	5256(%rbx), %eax
.Ltmp10549:
	.loc	1 1433 16
	testb	$1, %al
	jne	.LBB40_623
	testb	$2, %al
	jne	.LBB40_624
.LBB40_615:
	testb	$4, %al
	jne	.LBB40_625
.LBB40_616:
	testb	$8, %al
	jne	.LBB40_626
.LBB40_617:
	testb	$16, %al
	jne	.LBB40_627
.LBB40_618:
	testb	$32, %al
	jne	.LBB40_628
.LBB40_619:
	testb	$64, %al
	jne	.LBB40_629
.LBB40_620:
	testb	%al, %al
	jns	.LBB40_622
.LBB40_621:
	.loc	1 0 16 is_stmt 0
	movq	3088(%rsp), %rax
	movq	144(%rsp), %rdx
.Ltmp10550:
	.loc	38 2428 13 is_stmt 1
	addq	%rdx, %rax
	movq	$-1, %rcx
	cmovbq	%rcx, %rax
.Ltmp10551:
	.loc	1 1434 17
	movq	%rax, 3088(%rsp)
.Ltmp10552:
	.loc	38 2428 13
	addq	3096(%rsp), %rdx
	cmovbq	%rcx, %rdx
.Ltmp10553:
	.loc	1 1435 17
	movq	%rdx, 3096(%rsp)
.Ltmp10554:
.LBB40_622:
	.loc	1 0 17 is_stmt 0
	leaq	2784(%rsp), %rsi
	movl	$328, %edx
	movq	2776(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp10555:
	.loc	1 2013 6 is_stmt 1
	movq	%rbx, %rax
	leaq	-40(%rbp), %rsp
	.loc	1 2013 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB40_623:
	.cfi_def_cfa %rbp, 16
	.loc	1 0 6
	movq	2808(%rsp), %rcx
	movq	144(%rsp), %rsi
.Ltmp10556:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp10557:
	.loc	1 1434 17
	movq	%rcx, 2808(%rsp)
	movq	2816(%rsp), %rcx
.Ltmp10558:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp10559:
	.loc	1 1435 17
	movq	%rcx, 2816(%rsp)
	.loc	1 1433 16
	testb	$2, %al
	je	.LBB40_615
.LBB40_624:
	.loc	1 0 16 is_stmt 0
	movq	2848(%rsp), %rcx
	movq	144(%rsp), %rsi
.Ltmp10560:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp10561:
	.loc	1 1434 17
	movq	%rcx, 2848(%rsp)
	movq	2856(%rsp), %rcx
.Ltmp10562:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp10563:
	.loc	1 1435 17
	movq	%rcx, 2856(%rsp)
	.loc	1 1433 16
	testb	$4, %al
	je	.LBB40_616
.LBB40_625:
	.loc	1 0 16 is_stmt 0
	movq	2888(%rsp), %rcx
	movq	144(%rsp), %rsi
.Ltmp10564:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp10565:
	.loc	1 1434 17
	movq	%rcx, 2888(%rsp)
	movq	2896(%rsp), %rcx
.Ltmp10566:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp10567:
	.loc	1 1435 17
	movq	%rcx, 2896(%rsp)
	.loc	1 1433 16
	testb	$8, %al
	je	.LBB40_617
.LBB40_626:
	.loc	1 0 16 is_stmt 0
	movq	2928(%rsp), %rcx
	movq	144(%rsp), %rsi
.Ltmp10568:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp10569:
	.loc	1 1434 17
	movq	%rcx, 2928(%rsp)
	movq	2936(%rsp), %rcx
.Ltmp10570:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp10571:
	.loc	1 1435 17
	movq	%rcx, 2936(%rsp)
	.loc	1 1433 16
	testb	$16, %al
	je	.LBB40_618
.LBB40_627:
	.loc	1 0 16 is_stmt 0
	movq	2968(%rsp), %rcx
	movq	144(%rsp), %rsi
.Ltmp10572:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp10573:
	.loc	1 1434 17
	movq	%rcx, 2968(%rsp)
	movq	2976(%rsp), %rcx
.Ltmp10574:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp10575:
	.loc	1 1435 17
	movq	%rcx, 2976(%rsp)
	.loc	1 1433 16
	testb	$32, %al
	je	.LBB40_619
.LBB40_628:
	.loc	1 0 16 is_stmt 0
	movq	3008(%rsp), %rcx
	movq	144(%rsp), %rsi
.Ltmp10576:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp10577:
	.loc	1 1434 17
	movq	%rcx, 3008(%rsp)
	movq	3016(%rsp), %rcx
.Ltmp10578:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp10579:
	.loc	1 1435 17
	movq	%rcx, 3016(%rsp)
	.loc	1 1433 16
	testb	$64, %al
	je	.LBB40_620
.LBB40_629:
	.loc	1 0 16 is_stmt 0
	movq	3048(%rsp), %rcx
	movq	144(%rsp), %rsi
.Ltmp10580:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp10581:
	.loc	1 1434 17
	movq	%rcx, 3048(%rsp)
	movq	3056(%rsp), %rcx
.Ltmp10582:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp10583:
	.loc	1 1435 17
	movq	%rcx, 3056(%rsp)
	.loc	1 1433 16
	testb	%al, %al
	js	.LBB40_621
	jmp	.LBB40_622
.Ltmp10584:
.LBB40_631:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_630:
.Ltmp10585:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10586:
.LBB40_641:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	xorl	%edx, %edx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10587:
.LBB40_632:
	.loc	48 443 13 is_stmt 1
	leaq	.Lalloc_b02e35ee2c207cc88671eb1977dbcd5b(%rip), %rcx
	movq	1600(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10588:
.LBB40_633:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_634:
	leaq	.Lalloc_1f724420e117514d5eeca145f523ec40(%rip), %rcx
.Ltmp10589:
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_635:
.Ltmp10590:
	leaq	.Lalloc_ebd89d0b93275e5686fc2b41f2e9561d(%rip), %rcx
.Ltmp10591:
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_636:
.Ltmp10592:
	leaq	.Lalloc_89de14a4db9a7a5e199f0b3432909247(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_637:
	leaq	.Lalloc_e4c416c2b2b3df4fd71341a77882ada1(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10593:
.LBB40_642:
	leaq	.Lalloc_7515f6adc8a5521b72f04a3abb372e72(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_643:
	leaq	.Lalloc_8c2aade3368450b16e7e4997ad3fca81(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_644:
	leaq	.Lalloc_a4c3da5a99763e9452b49d42852d67e3(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_645:
	leaq	.Lalloc_e11ba0c4c1124bbcae4603a2ae121338(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_646:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_647:
	leaq	.Lalloc_379b93204ee4659b79f4b07b6520076a(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_648:
	leaq	.Lalloc_2bb8eb0542a889f29ef069491eb97a17(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_649:
	leaq	.Lalloc_cd96798e807157d7db308788cb4dd125(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_638:
	leaq	.Lalloc_164adf6876ccf79975d46e129e248ca5(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_653:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_652:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r9, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_639:
.Ltmp10594:
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	168(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10595:
.LBB40_640:
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	176(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10596:
.LBB40_650:
	.loc	48 569 13 is_stmt 1
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	2048(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10597:
.LBB40_651:
	.loc	48 569 13
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	184(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10598:
.LBB40_656:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_7ac5156198d2516c0e2a17140923b083(%rip), %rcx
	movq	%r13, %rdi
	movq	128(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_657:
	leaq	.Lalloc_683f9160cdc5ee4596b2ecf709188a9a(%rip), %rcx
	movq	%r13, %rdi
	movq	128(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_659:
	leaq	.Lalloc_94bf6a39a1d7f90452d19d4598bc6c91(%rip), %rcx
	movq	%r13, %rdi
	movq	120(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_658:
	leaq	.Lalloc_5337a4cbd2266e89524aaf9670b5f666(%rip), %rcx
	movq	%r13, %rdi
	movq	120(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB40_654:
.Ltmp10599:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_683f9160cdc5ee4596b2ecf709188a9a(%rip), %rcx
	movq	128(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10600:
.LBB40_655:
	.loc	48 456 13
	leaq	.Lalloc_7ac5156198d2516c0e2a17140923b083(%rip), %rcx
	movq	128(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10601:
.LBB40_660:
	.loc	48 456 13
	leaq	.Lalloc_5337a4cbd2266e89524aaf9670b5f666(%rip), %rcx
	movq	120(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10602:
.LBB40_661:
	.loc	48 456 13
	leaq	.Lalloc_94bf6a39a1d7f90452d19d4598bc6c91(%rip), %rcx
	movq	120(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp10603:
.LBB40_662:
	.loc	1 1995 25
	leaq	.Lalloc_e163651fd3b09506777efc3802c70e08(%rip), %rdx
	movq	%r8, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_663:
.Ltmp10604:
	.loc	1 1996 23
	leaq	.Lalloc_4833734f0e99b6856ca3d83ba3e6f90c(%rip), %rdx
	movq	%rcx, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10605:
.LBB40_692:
	.loc	1 0 23 is_stmt 0
	movq	%rcx, %rax
.Ltmp10606:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_696:
	movq	%r12, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_695:
	movq	%r8, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_665:
	movq	%r9, %rdi
.LBB40_666:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_690:
	movq	%r14, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_689:
	movq	%rdx, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_687:
	movq	%rcx, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_699:
	movq	%r14, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_710:
	movq	%rdx, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_706:
	movq	%rcx, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10607:
.LBB40_664:
	.loc	1 1479 43 is_stmt 1
	leaq	.Lalloc_618452d81e57e47d125c0afad7fc5006(%rip), %rdx
	movl	$12, %esi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10608:
.LBB40_694:
	.loc	1 0 43 is_stmt 0
	movq	%r11, %rdi
.Ltmp10609:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_681:
	movq	16(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_697:
	movq	%r8, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_734:
	movq	%r14, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_677:
	movq	%r13, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_683:
	movq	1824(%rsp), %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_688:
	movq	%r14, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_679:
	movq	1664(%rsp), %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_686:
	movq	24(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_704:
	movq	%r13, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_702:
	movq	16(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_682:
	movq	%r11, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_698:
	movq	24(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_700:
	movq	1664(%rsp), %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10610:
.LBB40_667:
	.loc	1 1481 45 is_stmt 1
	leaq	.Lalloc_97e1583d528f48ca4578bc0e35112ce6(%rip), %rdx
	movl	$10, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10611:
.LBB40_676:
	.loc	1 0 45 is_stmt 0
	movq	%rbx, %rdi
.Ltmp10612:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10613:
.LBB40_668:
	movq	1856(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_680:
	movq	%r15, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_691:
	movq	32(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_703:
	movq	%r15, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_714:
	movq	1888(%rsp), %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_708:
	movq	%rbx, %rdi
.Ltmp10614:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10615:
.LBB40_727:
	movq	%rcx, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_737:
	movq	%r12, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_729:
	movq	48(%rsp), %rdi
.Ltmp10616:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10617:
.LBB40_730:
	movq	%r11, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_746:
	movq	%r11, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_670:
	movq	40(%rsp), %rax
.Ltmp10618:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10619:
.LBB40_671:
	movq	%r10, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_672:
	movq	48(%rsp), %rdi
.Ltmp10620:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10621:
.LBB40_673:
	movq	48(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_674:
	movq	1888(%rsp), %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_675:
	movq	136(%rsp), %rdi
.Ltmp10622:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10623:
.LBB40_678:
	movq	2048(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_684:
	movq	%r13, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_685:
	movq	136(%rsp), %rax
.Ltmp10624:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10625:
.LBB40_693:
	movq	1856(%rsp), %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_712:
	movq	32(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_715:
	movq	1856(%rsp), %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_724:
	movq	%rbx, %rax
.Ltmp10626:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10627:
.LBB40_725:
	movq	2048(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_731:
	movq	%r10, %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_756:
	movq	136(%rsp), %rdi
.Ltmp10628:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10629:
.LBB40_761:
	movq	152(%rsp), %rdi
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_762:
	movq	32(%rsp), %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_764:
	movq	%r11, %rax
.LBB40_763:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp10630:
.Lfunc_end40:
	.size	_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_, .Lfunc_end40-_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kj8_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_
