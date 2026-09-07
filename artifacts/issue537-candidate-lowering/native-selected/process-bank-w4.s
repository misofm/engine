_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_:
.Lfunc_begin34:
	.loc	1 1986 0
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
	subq	$2280, %rsp
	.cfi_def_cfa_offset 2336
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdi, 1320(%rsp)
.Ltmp3119:
	.loc	1 1987 51 prologue_end
	movzbl	2816(%rsi), %eax
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqu	%ymm0, 1840(%rsp)
	vmovdqu	%ymm0, 1808(%rsp)
	vmovdqu	%ymm0, 1776(%rsp)
	vmovdqu	%ymm0, 1744(%rsp)
	vmovdqu	%ymm0, 1712(%rsp)
	vmovdqu	%ymm0, 1680(%rsp)
	vmovdqu	%ymm0, 1648(%rsp)
	vmovdqu	%ymm0, 1616(%rsp)
	vmovdqu	%ymm0, 1584(%rsp)
	vmovdqu	%ymm0, 1552(%rsp)
.Ltmp3120:
	.loc	43 1032 9
	movb	%al, 1872(%rsp)
.Ltmp3121:
	.loc	43 186 45
	cmpb	%al, 108(%rdx)
.Ltmp3122:
	.loc	1 1991 12
	jne	.LBB34_526
	cmpq	$0, 64(%rdx)
	jne	.LBB34_526
	.loc	1 0 12 is_stmt 0
	movq	%rsi, %rbx
	movq	48(%rdx), %r10
	movq	56(%rdx), %r8
	movq	32(%rdx), %rax
	movq	%rax, 1200(%rsp)
	movq	40(%rdx), %rax
	movq	%rax, 48(%rsp)
	movq	%rdx, 1120(%rsp)
	movq	96(%rdx), %r11
.Ltmp3123:
	.loc	3 900 12 is_stmt 1
	cmpq	$1, %r8
	movq	%r8, %rax
	adcq	$-1, %rax
	movq	%rax, 128(%rsp)
	leaq	348(%rsi), %r14
	xorl	%eax, %eax
	vmovss	.LCPI34_0(%rip), %xmm2
	movq	%rsi, 1080(%rsp)
	movq	%r8, 1184(%rsp)
	movq	%r10, 1168(%rsp)
	movq	%r11, 1152(%rsp)
	jmp	.LBB34_4
	.loc	3 0 12 is_stmt 0
.Ltmp3124:
	.p2align	4
.LBB34_3:
	.loc	3 900 12 is_stmt 1
	addq	$160, %r14
	movq	80(%rsp), %rcx
	movq	%rcx, %rax
.Ltmp3125:
	.loc	2 1916 50
	cmpq	$4, %rcx
	movq	1080(%rsp), %rbx
.Ltmp3126:
	.loc	3 900 12
	je	.LBB34_100
.Ltmp3127:
.LBB34_4:
	.loc	1 1995 25
	cmpq	%r8, %rax
	je	.LBB34_578
.Ltmp3128:
	.loc	1 0 0 is_stmt 0
	leaq	1(%rax), %rcx
.Ltmp3129:
	.loc	1 1996 23 is_stmt 1
	cmpq	128(%rsp), %rax
	je	.LBB34_579
	.loc	1 0 23 is_stmt 0
	movl	(%r10,%rax,4), %edi
	.loc	1 1996 23
	movl	(%r10,%rcx,4), %esi
.Ltmp3130:
	.loc	38 1050 16 is_stmt 1
	cmpl	%edi, %esi
	jb	.LBB34_533
	cmpq	%rsi, 48(%rsp)
	jb	.LBB34_533
.Ltmp3131:
	.loc	38 0 16 is_stmt 0
	movq	%rcx, 80(%rsp)
	movq	%r14, 16(%rsp)
	.loc	1 2000 17 is_stmt 1
	movl	2916(%rbx), %edx
	movl	$0, 208(%rsp)
	movl	$0, 216(%rsp)
	movl	$0, 224(%rsp)
	movl	$0, 232(%rsp)
	movl	$0, 240(%rsp)
	movl	$0, 248(%rsp)
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
.Ltmp3132:
	.loc	32 1714 9
	cmpl	%edi, %esi
.Ltmp3133:
	.loc	33 180 28
	jne	.LBB34_81
.Ltmp3134:
.LBB34_9:
	.loc	33 0 28 is_stmt 0
	movl	$76, %eax
	movq	16(%rsp), %r14
	movq	%r14, %rcx
.Ltmp3135:
	.loc	33 180 28
	jmp	.LBB34_13
.Ltmp3136:
	.loc	33 0 28
.Ltmp3137:
	.p2align	4
.LBB34_10:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
.LBB34_11:
	.loc	36 0 0
	vmovd	%xmm0, -4(%rcx)
	movl	%edx, (%rcx)
.Ltmp3138:
.LBB34_12:
	.loc	32 1714 9 is_stmt 1
	addq	$80, %rax
	addq	$1328, %rcx
	cmpq	$236, %rax
.Ltmp3139:
	.loc	33 180 28
	je	.LBB34_3
.Ltmp3140:
.LBB34_13:
	.loc	1 1495 24
	cmpl	$1, 132(%rsp,%rax)
	jne	.LBB34_14
	.loc	1 1495 29 is_stmt 0
	vmovd	136(%rsp,%rax), %xmm0
.Ltmp3141:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -152(%rcx)
	.loc	36 81 48
	vmovd	-156(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3142:
	.loc	36 112 9
	jg	.LBB34_27
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_27
	negl	%edx
	jo	.LBB34_27
.Ltmp3143:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -156(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -148(%rcx)
	movl	%edx, -144(%rcx)
.Ltmp3144:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 140(%rsp,%rax)
	je	.LBB34_29
	.loc	1 0 24 is_stmt 0
.Ltmp3145:
	.p2align	4
.LBB34_15:
	.loc	1 1495 24
	cmpl	$1, 148(%rsp,%rax)
	jne	.LBB34_16
.LBB34_35:
	.loc	1 1495 29
	vmovd	152(%rsp,%rax), %xmm0
.Ltmp3146:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -120(%rcx)
	.loc	36 81 48
	vmovd	-124(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3147:
	.loc	36 112 9
	jg	.LBB34_39
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_39
	negl	%edx
	jo	.LBB34_39
.Ltmp3148:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -124(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -116(%rcx)
	movl	%edx, -112(%rcx)
.Ltmp3149:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 156(%rsp,%rax)
	je	.LBB34_41
	.loc	1 0 24 is_stmt 0
.Ltmp3150:
	.p2align	4
.LBB34_17:
	.loc	1 1495 24
	cmpl	$1, 164(%rsp,%rax)
	jne	.LBB34_18
.LBB34_47:
	.loc	1 1495 29
	vmovd	168(%rsp,%rax), %xmm0
.Ltmp3151:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -88(%rcx)
	.loc	36 81 48
	vmovd	-92(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3152:
	.loc	36 112 9
	jg	.LBB34_51
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_51
	negl	%edx
	jo	.LBB34_51
.Ltmp3153:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -92(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -84(%rcx)
	movl	%edx, -80(%rcx)
.Ltmp3154:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 172(%rsp,%rax)
	je	.LBB34_53
	.loc	1 0 24 is_stmt 0
.Ltmp3155:
	.p2align	4
.LBB34_19:
	.loc	1 1495 24
	cmpl	$1, 180(%rsp,%rax)
	jne	.LBB34_20
.LBB34_59:
	.loc	1 1495 29
	vmovd	184(%rsp,%rax), %xmm0
.Ltmp3156:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -56(%rcx)
	.loc	36 81 48
	vmovd	-60(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3157:
	.loc	36 112 9
	jg	.LBB34_63
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_63
	negl	%edx
	jo	.LBB34_63
.Ltmp3158:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -60(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -52(%rcx)
	movl	%edx, -48(%rcx)
.Ltmp3159:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 188(%rsp,%rax)
	je	.LBB34_65
	.loc	1 0 24 is_stmt 0
.Ltmp3160:
	.p2align	4
.LBB34_21:
	.loc	1 1495 24
	cmpl	$1, 196(%rsp,%rax)
	jne	.LBB34_22
.LBB34_71:
	.loc	1 1495 29
	vmovd	200(%rsp,%rax), %xmm0
.Ltmp3161:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -24(%rcx)
	.loc	36 81 48
	vmovd	-28(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3162:
	.loc	36 112 9
	jg	.LBB34_75
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_75
	negl	%edx
	jo	.LBB34_75
.Ltmp3163:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -28(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -20(%rcx)
	movl	%edx, -16(%rcx)
.Ltmp3164:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 204(%rsp,%rax)
	jne	.LBB34_12
	jmp	.LBB34_77
	.loc	1 0 24 is_stmt 0
.Ltmp3165:
	.p2align	4
.LBB34_14:
	.loc	1 1495 24
	cmpl	$1, 140(%rsp,%rax)
	jne	.LBB34_15
.LBB34_29:
	.loc	1 1495 29
	vmovd	144(%rsp,%rax), %xmm0
.Ltmp3166:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -136(%rcx)
	.loc	36 81 48
	vmovd	-140(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3167:
	.loc	36 112 9
	jg	.LBB34_33
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_33
	negl	%edx
	jo	.LBB34_33
.Ltmp3168:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -140(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -132(%rcx)
	movl	%edx, -128(%rcx)
.Ltmp3169:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 148(%rsp,%rax)
	jne	.LBB34_16
	jmp	.LBB34_35
	.loc	1 0 24 is_stmt 0
.Ltmp3170:
	.p2align	4
.LBB34_27:
.Ltmp3171:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -148(%rcx)
	movl	%edx, -144(%rcx)
.Ltmp3172:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 140(%rsp,%rax)
	jne	.LBB34_15
	jmp	.LBB34_29
	.loc	1 0 24 is_stmt 0
.Ltmp3173:
	.p2align	4
.LBB34_39:
.Ltmp3174:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -116(%rcx)
	movl	%edx, -112(%rcx)
.Ltmp3175:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 156(%rsp,%rax)
	jne	.LBB34_17
	jmp	.LBB34_41
	.loc	1 0 24 is_stmt 0
.Ltmp3176:
	.p2align	4
.LBB34_51:
.Ltmp3177:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -84(%rcx)
	movl	%edx, -80(%rcx)
.Ltmp3178:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 172(%rsp,%rax)
	jne	.LBB34_19
	jmp	.LBB34_53
	.loc	1 0 24 is_stmt 0
.Ltmp3179:
	.p2align	4
.LBB34_63:
.Ltmp3180:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -52(%rcx)
	movl	%edx, -48(%rcx)
.Ltmp3181:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 188(%rsp,%rax)
	jne	.LBB34_21
	jmp	.LBB34_65
	.loc	1 0 24 is_stmt 0
.Ltmp3182:
	.p2align	4
.LBB34_75:
.Ltmp3183:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -20(%rcx)
	movl	%edx, -16(%rcx)
.Ltmp3184:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 204(%rsp,%rax)
	jne	.LBB34_12
	jmp	.LBB34_77
	.loc	1 0 24 is_stmt 0
.Ltmp3185:
	.p2align	4
.LBB34_33:
.Ltmp3186:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -132(%rcx)
	movl	%edx, -128(%rcx)
.Ltmp3187:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 148(%rsp,%rax)
	je	.LBB34_35
	.loc	1 0 24 is_stmt 0
.Ltmp3188:
	.p2align	4
.LBB34_16:
	.loc	1 1495 24
	cmpl	$1, 156(%rsp,%rax)
	jne	.LBB34_17
.LBB34_41:
	.loc	1 1495 29
	vmovd	160(%rsp,%rax), %xmm0
.Ltmp3189:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -104(%rcx)
	.loc	36 81 48
	vmovd	-108(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3190:
	.loc	36 112 9
	jg	.LBB34_45
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_45
	negl	%edx
	jo	.LBB34_45
.Ltmp3191:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -108(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -100(%rcx)
	movl	%edx, -96(%rcx)
.Ltmp3192:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 164(%rsp,%rax)
	jne	.LBB34_18
	jmp	.LBB34_47
	.loc	1 0 24 is_stmt 0
.Ltmp3193:
	.p2align	4
.LBB34_45:
.Ltmp3194:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -100(%rcx)
	movl	%edx, -96(%rcx)
.Ltmp3195:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 164(%rsp,%rax)
	je	.LBB34_47
	.loc	1 0 24 is_stmt 0
.Ltmp3196:
	.p2align	4
.LBB34_18:
	.loc	1 1495 24
	cmpl	$1, 172(%rsp,%rax)
	jne	.LBB34_19
.LBB34_53:
	.loc	1 1495 29
	vmovd	176(%rsp,%rax), %xmm0
.Ltmp3197:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -72(%rcx)
	.loc	36 81 48
	vmovd	-76(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3198:
	.loc	36 112 9
	jg	.LBB34_57
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_57
	negl	%edx
	jo	.LBB34_57
.Ltmp3199:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -76(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -68(%rcx)
	movl	%edx, -64(%rcx)
.Ltmp3200:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 180(%rsp,%rax)
	jne	.LBB34_20
	jmp	.LBB34_59
	.loc	1 0 24 is_stmt 0
.Ltmp3201:
	.p2align	4
.LBB34_57:
.Ltmp3202:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -68(%rcx)
	movl	%edx, -64(%rcx)
.Ltmp3203:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 180(%rsp,%rax)
	je	.LBB34_59
	.loc	1 0 24 is_stmt 0
.Ltmp3204:
	.p2align	4
.LBB34_20:
	.loc	1 1495 24
	cmpl	$1, 188(%rsp,%rax)
	jne	.LBB34_21
.LBB34_65:
	.loc	1 1495 29
	vmovd	192(%rsp,%rax), %xmm0
.Ltmp3205:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -40(%rcx)
	.loc	36 81 48
	vmovd	-44(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3206:
	.loc	36 112 9
	jg	.LBB34_69
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_69
	negl	%edx
	jo	.LBB34_69
.Ltmp3207:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -44(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm0, -36(%rcx)
	movl	%edx, -32(%rcx)
.Ltmp3208:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 196(%rsp,%rax)
	jne	.LBB34_22
	jmp	.LBB34_71
	.loc	1 0 24 is_stmt 0
.Ltmp3209:
	.p2align	4
.LBB34_69:
.Ltmp3210:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm0, -36(%rcx)
	movl	%edx, -32(%rcx)
.Ltmp3211:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 196(%rsp,%rax)
	je	.LBB34_71
	.loc	1 0 24 is_stmt 0
.Ltmp3212:
	.p2align	4
.LBB34_22:
	.loc	1 1495 24
	cmpl	$1, 204(%rsp,%rax)
	jne	.LBB34_12
.LBB34_77:
	.loc	1 1495 29
	vmovd	208(%rsp,%rax), %xmm0
.Ltmp3213:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm0, -8(%rcx)
	.loc	36 81 48
	vmovd	-12(%rcx), %xmm1
	vmovd	%xmm1, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp3214:
	.loc	36 112 9
	jg	.LBB34_10
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm0, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB34_10
	negl	%edx
	jo	.LBB34_10
.Ltmp3215:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm0, -12(%rcx)
	xorl	%edx, %edx
	vpxor	%xmm0, %xmm0, %xmm0
	.loc	36 89 6
	jmp	.LBB34_11
.Ltmp3216:
	.loc	36 0 6 is_stmt 0
.Ltmp3217:
	.p2align	4
.LBB34_81:
	subq	%rdi, %rsi
	leaq	(%rdi,%rdi,4), %rcx
	movq	%rdx, 64(%rsp)
	movq	1200(%rsp), %rdx
	leaq	(%rdx,%rcx,8), %r13
	movq	64(%rsp), %rdx
	leaq	(%rsi,%rsi,4), %rcx
	leaq	(%r13,%rcx,8), %rbx
	.loc	1 2002 17 is_stmt 1
	leaq	(%rax,%rax,4), %rax
	leaq	1552(%rsp,%rax,8), %r14
	movq	1568(%rsp,%rax,8), %r15
	movb	$1, %sil
	xorl	%ebp, %ebp
	jmp	.LBB34_82
	.loc	1 0 17 is_stmt 0
.Ltmp3218:
	.p2align	4
.LBB34_98:
.Ltmp3219:
	addq	$40, %r13
.Ltmp3220:
	.loc	38 2428 13 is_stmt 1
	incq	%r15
	movq	$-1, %rax
	cmoveq	%rax, %r15
.Ltmp3221:
	.loc	1 0 0 is_stmt 0
	movq	%r15, 16(%r14)
.Ltmp3222:
	.loc	4 82 9 is_stmt 1
	incq	%rbp
.Ltmp3223:
	.loc	32 1714 9
	cmpq	%rbx, %r13
.Ltmp3224:
	.loc	33 180 28
	je	.LBB34_9
.Ltmp3225:
.LBB34_82:
	.loc	1 1457 30
	movl	32(%r13), %eax
	.loc	1 1457 24 is_stmt 0
	cmpl	$1, %eax
	je	.LBB34_86
	cmpl	$2, %eax
	jne	.LBB34_98
	.loc	1 0 24
	movl	$1, %eax
	leaq	288(%rsp), %r12
.Ltmp3226:
	.loc	1 1465 29 is_stmt 1
	movl	16(%r13), %edi
	cmpq	$2, %rdi
.Ltmp3227:
	.loc	38 1050 16
	jae	.LBB34_87
.Ltmp3228:
.LBB34_85:
	.loc	38 0 16 is_stmt 0
	xorl	%ecx, %ecx
	cmpq	%rdx, %rbp
.Ltmp3229:
	.loc	1 1473 25 is_stmt 1
	jb	.LBB34_88
	jmp	.LBB34_98
.Ltmp3230:
	.loc	1 0 25 is_stmt 0
.Ltmp3231:
	.p2align	4
.LBB34_86:
	xorl	%eax, %eax
	leaq	208(%rsp), %r12
	.loc	1 1465 29 is_stmt 1
	movl	16(%r13), %edi
	cmpq	$2, %rdi
.Ltmp3232:
	.loc	38 1050 16
	jb	.LBB34_85
.LBB34_87:
	.loc	38 1054 31
	leaq	-2(%rdi), %rcx
	movq	%rcx, 32(%rsp)
.Ltmp3233:
	.loc	28 1580 16
	xorl	%ecx, %ecx
	cmpl	$12, %edi
	setb	%cl
	cmpq	%rdx, %rbp
.Ltmp3234:
	.loc	1 1473 25
	jae	.LBB34_98
.LBB34_88:
	cmpq	$1, %rcx
	jne	.LBB34_98
	.loc	1 1475 20
	cmpl	$1, 28(%r13)
	jne	.LBB34_98
	.loc	1 1476 20
	cmpq	%r11, (%r13)
	jne	.LBB34_98
	.loc	1 1477 20
	cmpq	%r11, 8(%r13)
	jne	.LBB34_98
	.loc	1 1478 20
	vmovd	20(%r13), %xmm0
.Ltmp3235:
	.loc	23 1244 18
	vmovd	%xmm0, %ecx
.Ltmp3236:
	.loc	1 1478 20
	cmpl	%ecx, 24(%r13)
	jne	.LBB34_98
	.loc	1 0 20 is_stmt 0
	movl	%esi, 176(%rsp)
	.loc	1 1479 43 is_stmt 1
	cmpl	$11, %edi
	ja	.LBB34_584
	.loc	1 0 43 is_stmt 0
	leal	(%rax,%rdi,2), %eax
	movl	%eax, 1216(%rsp)
	.loc	1 1479 42
	leaq	(%rdi,%rdi,4), %rax
	leaq	.Lalloc_cc33a3b9cd8c16d253f2168b5461d31d(%rip), %rcx
	leaq	(%rcx,%rax,8), %rdi
	vmovdqa	%xmm0, 1024(%rsp)
	.loc	1 1479 20
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	1024(%rsp), %xmm1
	movl	1216(%rsp), %r9d
	cmpl	1136(%rsp), %r9d
	seta	%cl
	testb	%al, %al
	movq	1184(%rsp), %r8
	movq	1168(%rsp), %r10
	movq	1152(%rsp), %r11
	vmovss	.LCPI34_0(%rip), %xmm2
	movq	64(%rsp), %rdx
	movl	176(%rsp), %esi
	je	.LBB34_98
	orb	%sil, %cl
	testb	$1, %cl
	je	.LBB34_98
	.loc	1 0 20
	movq	32(%rsp), %rdi
.Ltmp3237:
	.loc	1 1481 45 is_stmt 1
	cmpq	$9, %rdi
	ja	.LBB34_587
.Ltmp3238:
	.loc	47 430 9
	cmpl	$0, (%r12,%rdi,8)
.Ltmp3239:
	.loc	1 1486 17
	jne	.LBB34_98
.Ltmp3240:
	.loc	31 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
	movq	32(%rsp), %rax
.Ltmp3241:
	.loc	1 1491 13
	movl	$1, (%r12,%rax,8)
	vmovss	%xmm0, 4(%r12,%rax,8)
.Ltmp3242:
	.loc	32 1714 9
	addq	$40, %r13
.Ltmp3243:
	.loc	33 180 28
	incq	%rbp
	xorl	%esi, %esi
	movl	%r9d, 1136(%rsp)
.Ltmp3244:
	.loc	32 1714 9
	cmpq	%rbx, %r13
.Ltmp3245:
	.loc	33 180 28
	jne	.LBB34_82
	jmp	.LBB34_9
.Ltmp3246:
.LBB34_100:
	.loc	33 0 28 is_stmt 0
	movq	1120(%rsp), %rax
	.loc	1 2007 13 is_stmt 1
	movq	(%rax), %r8
	movq	8(%rax), %rdi
	.loc	1 2008 13
	movq	16(%rax), %r9
	movq	24(%rax), %rdx
	.loc	1 2009 13
	movl	104(%rax), %esi
.Ltmp3247:
	.loc	1 1245 12
	movl	2692(%rbx), %ecx
	.loc	1 1245 27 is_stmt 0
	movzbl	2696(%rbx), %eax
	.loc	1 1245 5
	cmpl	$1, %ecx
	movq	%rsi, 984(%rsp)
	movq	%r9, 120(%rsp)
	movq	%rdx, 104(%rsp)
	movq	%rdi, 112(%rsp)
	movq	%r8, 968(%rsp)
	je	.LBB34_105
	cmpl	$2, %ecx
	jne	.LBB34_108
	testb	%al, %al
	jne	.LBB34_109
.Ltmp3248:
	.loc	1 1189 11 is_stmt 1
	testq	%rsi, %rsi
	je	.LBB34_503
.Ltmp3249:
	.loc	1 0 11 is_stmt 0
	movl	2688(%rbx), %eax
	movl	%eax, 1104(%rsp)
	movq	2672(%rbx), %rbp
	leaq	1328(%rbx), %rax
	movq	%rax, 1312(%rsp)
	xorl	%r15d, %r15d
	movq	%rbp, 176(%rsp)
	jmp	.LBB34_273
.LBB34_105:
	.loc	1 1245 5 is_stmt 1
	testb	%al, %al
	jne	.LBB34_109
.Ltmp3250:
	.loc	1 1189 11
	testq	%rsi, %rsi
	je	.LBB34_503
.Ltmp3251:
	.loc	1 0 11 is_stmt 0
	movl	2688(%rbx), %eax
	movl	%eax, 1112(%rsp)
	movq	2672(%rbx), %rbp
	leaq	1328(%rbx), %rax
	movq	%rax, 1104(%rsp)
	xorl	%r15d, %r15d
	movq	%rbp, 1024(%rsp)
	jmp	.LBB34_158
.LBB34_108:
	.loc	1 1245 5 is_stmt 1
	testb	%al, %al
	je	.LBB34_386
.LBB34_109:
.Ltmp3252:
	.loc	1 1189 11
	testq	%rsi, %rsi
	je	.LBB34_503
	.loc	1 0 11 is_stmt 0
	movl	2688(%rbx), %eax
	movl	%eax, 1024(%rsp)
	movq	2672(%rbx), %r12
	leaq	1328(%rbx), %rax
	movq	%rax, 32(%rsp)
	leaq	96(%rbx), %rax
	movq	%rax, 16(%rsp)
	leaq	1424(%rbx), %rax
	movq	%rax, 80(%rsp)
	leaq	160(%rbx), %rax
	movq	%rax, 64(%rsp)
	leaq	1488(%rbx), %rax
	movq	%rax, 48(%rsp)
	xorl	%ebp, %ebp
	xorl	%r13d, %r13d
	jmp	.LBB34_113
	.p2align	4
.LBB34_111:
.Ltmp3253:
	.loc	1 1160 5 is_stmt 1
	vmovups	2080(%rsp), %ymm0
	vmovups	2112(%rsp), %ymm1
	movq	16(%rsp), %rax
	vmovups	%ymm1, 32(%rax)
	vmovups	%ymm0, (%rax)
	.loc	1 1161 5
	vmovups	2144(%rsp), %ymm0
	vmovups	2176(%rsp), %ymm1
	movq	80(%rsp), %rax
	vmovups	%ymm1, 32(%rax)
	vmovups	%ymm0, (%rax)
	.loc	1 1162 5
	vmovups	2208(%rsp), %ymm0
	movq	64(%rsp), %rax
	vmovups	%ymm0, (%rax)
	.loc	1 1163 5
	vmovdqu	2240(%rsp), %ymm0
	movq	48(%rsp), %rax
	vmovdqu	%ymm0, (%rax)
	.loc	1 1164 5
	movq	%r9, 2680(%rbx)
	movq	984(%rsp), %rsi
	movq	120(%rsp), %r9
.Ltmp3254:
.LBB34_112:
	.loc	1 0 5 is_stmt 0
	movq	176(%rsp), %r13
	.loc	1 1189 11 is_stmt 1
	cmpq	%rsi, %r13
	jae	.LBB34_503
.LBB34_113:
	.loc	1 1190 42
	subq	%r13, %rsi
	.loc	1 1190 29 is_stmt 0
	movq	%rbx, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E12plan_segmentB5_
	movb	%dl, 128(%rsp)
	movq	%rax, %r15
.Ltmp3255:
	.loc	1 1279 33 is_stmt 1
	vmovss	192(%rbx), %xmm0
.Ltmp3256:
	.loc	1 1192 28
	vmovss	%xmm0, 208(%rsp)
.Ltmp3257:
	.loc	1 1279 33
	vmovss	352(%rbx), %xmm0
.Ltmp3258:
	.loc	1 1192 28
	vmovss	%xmm0, 212(%rsp)
.Ltmp3259:
	.loc	1 1279 33
	vmovss	512(%rbx), %xmm0
.Ltmp3260:
	.loc	1 1192 28
	vmovss	%xmm0, 216(%rsp)
.Ltmp3261:
	.loc	1 1279 33
	vmovss	672(%rbx), %xmm0
.Ltmp3262:
	.loc	1 1192 28
	vmovss	%xmm0, 220(%rsp)
.Ltmp3263:
	.loc	1 1279 33
	vmovss	208(%rbx), %xmm0
.Ltmp3264:
	.loc	1 1192 28
	vmovss	%xmm0, 224(%rsp)
.Ltmp3265:
	.loc	1 1279 33
	vmovss	368(%rbx), %xmm0
.Ltmp3266:
	.loc	1 1192 28
	vmovss	%xmm0, 228(%rsp)
.Ltmp3267:
	.loc	1 1279 33
	vmovss	528(%rbx), %xmm0
.Ltmp3268:
	.loc	1 1192 28
	vmovss	%xmm0, 232(%rsp)
.Ltmp3269:
	.loc	1 1279 33
	vmovss	688(%rbx), %xmm0
.Ltmp3270:
	.loc	1 1192 28
	vmovss	%xmm0, 236(%rsp)
.Ltmp3271:
	.loc	1 1279 33
	vmovss	224(%rbx), %xmm0
.Ltmp3272:
	.loc	1 1192 28
	vmovss	%xmm0, 240(%rsp)
.Ltmp3273:
	.loc	1 1279 33
	vmovss	384(%rbx), %xmm0
.Ltmp3274:
	.loc	1 1192 28
	vmovss	%xmm0, 244(%rsp)
.Ltmp3275:
	.loc	1 1279 33
	vmovss	544(%rbx), %xmm0
.Ltmp3276:
	.loc	1 1192 28
	vmovss	%xmm0, 248(%rsp)
.Ltmp3277:
	.loc	1 1279 33
	vmovss	704(%rbx), %xmm0
.Ltmp3278:
	.loc	1 1192 28
	vmovss	%xmm0, 252(%rsp)
.Ltmp3279:
	.loc	1 1279 33
	vmovss	240(%rbx), %xmm0
.Ltmp3280:
	.loc	1 1192 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp3281:
	.loc	1 1279 33
	vmovss	400(%rbx), %xmm0
.Ltmp3282:
	.loc	1 1192 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp3283:
	.loc	1 1279 33
	vmovss	560(%rbx), %xmm0
.Ltmp3284:
	.loc	1 1192 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp3285:
	.loc	1 1279 33
	vmovss	720(%rbx), %xmm0
.Ltmp3286:
	.loc	1 1192 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp3287:
	.loc	1 1279 33
	vmovss	256(%rbx), %xmm0
.Ltmp3288:
	.loc	1 1192 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp3289:
	.loc	1 1279 33
	vmovss	416(%rbx), %xmm0
.Ltmp3290:
	.loc	1 1192 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp3291:
	.loc	1 1279 33
	vmovss	576(%rbx), %xmm0
.Ltmp3292:
	.loc	1 1192 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp3293:
	.loc	1 1279 33
	vmovss	736(%rbx), %xmm0
.Ltmp3294:
	.loc	1 1192 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp3295:
	.loc	1 1279 33
	vmovss	272(%rbx), %xmm0
.Ltmp3296:
	.loc	1 1192 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp3297:
	.loc	1 1279 33
	vmovss	432(%rbx), %xmm0
.Ltmp3298:
	.loc	1 1192 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp3299:
	.loc	1 1279 33
	vmovss	592(%rbx), %xmm0
.Ltmp3300:
	.loc	1 1192 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp3301:
	.loc	1 1279 33
	vmovss	752(%rbx), %xmm0
.Ltmp3302:
	.loc	1 1192 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp3303:
	.loc	1 1279 33
	vmovss	288(%rbx), %xmm0
.Ltmp3304:
	.loc	1 1192 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp3305:
	.loc	1 1279 33
	vmovss	448(%rbx), %xmm0
.Ltmp3306:
	.loc	1 1192 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp3307:
	.loc	1 1279 33
	vmovss	608(%rbx), %xmm0
.Ltmp3308:
	.loc	1 1192 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp3309:
	.loc	1 1279 33
	vmovss	768(%rbx), %xmm0
.Ltmp3310:
	.loc	1 1192 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp3311:
	.loc	1 1279 33
	vmovss	304(%rbx), %xmm0
.Ltmp3312:
	.loc	1 1192 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp3313:
	.loc	1 1279 33
	vmovss	464(%rbx), %xmm0
.Ltmp3314:
	.loc	1 1192 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp3315:
	.loc	1 1279 33
	vmovss	624(%rbx), %xmm0
.Ltmp3316:
	.loc	1 1192 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp3317:
	.loc	1 1279 33
	vmovss	784(%rbx), %xmm0
.Ltmp3318:
	.loc	1 1192 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp3319:
	.loc	1 1279 33
	vmovss	320(%rbx), %xmm0
.Ltmp3320:
	.loc	1 1192 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp3321:
	.loc	1 1279 33
	vmovss	480(%rbx), %xmm0
.Ltmp3322:
	.loc	1 1192 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp3323:
	.loc	1 1279 33
	vmovss	640(%rbx), %xmm0
.Ltmp3324:
	.loc	1 1192 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp3325:
	.loc	1 1279 33
	vmovss	800(%rbx), %xmm0
.Ltmp3326:
	.loc	1 1192 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp3327:
	.loc	1 1279 33
	vmovss	336(%rbx), %xmm0
.Ltmp3328:
	.loc	1 1192 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp3329:
	.loc	1 1279 33
	vmovss	496(%rbx), %xmm0
.Ltmp3330:
	.loc	1 1192 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp3331:
	.loc	1 1279 33
	vmovss	656(%rbx), %xmm0
.Ltmp3332:
	.loc	1 1192 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp3333:
	.loc	1 1279 33
	vmovss	816(%rbx), %xmm0
.Ltmp3334:
	.loc	1 1192 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp3335:
	.loc	1 1280 32
	vmovss	200(%rbx), %xmm0
.Ltmp3336:
	.loc	1 1192 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp3337:
	.loc	1 1280 32
	vmovss	360(%rbx), %xmm0
.Ltmp3338:
	.loc	1 1192 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp3339:
	.loc	1 1280 32
	vmovss	520(%rbx), %xmm0
.Ltmp3340:
	.loc	1 1192 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp3341:
	.loc	1 1280 32
	vmovss	680(%rbx), %xmm0
.Ltmp3342:
	.loc	1 1192 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp3343:
	.loc	1 1280 32
	vmovss	216(%rbx), %xmm0
.Ltmp3344:
	.loc	1 1192 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp3345:
	.loc	1 1280 32
	vmovss	376(%rbx), %xmm0
.Ltmp3346:
	.loc	1 1192 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp3347:
	.loc	1 1280 32
	vmovss	536(%rbx), %xmm0
.Ltmp3348:
	.loc	1 1192 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp3349:
	.loc	1 1280 32
	vmovss	696(%rbx), %xmm0
.Ltmp3350:
	.loc	1 1192 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp3351:
	.loc	1 1280 32
	vmovss	232(%rbx), %xmm0
.Ltmp3352:
	.loc	1 1192 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp3353:
	.loc	1 1280 32
	vmovss	392(%rbx), %xmm0
.Ltmp3354:
	.loc	1 1192 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp3355:
	.loc	1 1280 32
	vmovss	552(%rbx), %xmm0
.Ltmp3356:
	.loc	1 1192 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp3357:
	.loc	1 1280 32
	vmovss	712(%rbx), %xmm0
.Ltmp3358:
	.loc	1 1192 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp3359:
	.loc	1 1280 32
	vmovss	248(%rbx), %xmm0
.Ltmp3360:
	.loc	1 1192 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp3361:
	.loc	1 1280 32
	vmovss	408(%rbx), %xmm0
.Ltmp3362:
	.loc	1 1192 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp3363:
	.loc	1 1280 32
	vmovss	568(%rbx), %xmm0
.Ltmp3364:
	.loc	1 1192 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp3365:
	.loc	1 1280 32
	vmovss	728(%rbx), %xmm0
.Ltmp3366:
	.loc	1 1192 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp3367:
	.loc	1 1280 32
	vmovss	264(%rbx), %xmm0
.Ltmp3368:
	.loc	1 1192 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp3369:
	.loc	1 1280 32
	vmovss	424(%rbx), %xmm0
.Ltmp3370:
	.loc	1 1192 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp3371:
	.loc	1 1280 32
	vmovss	584(%rbx), %xmm0
.Ltmp3372:
	.loc	1 1192 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp3373:
	.loc	1 1280 32
	vmovss	744(%rbx), %xmm0
.Ltmp3374:
	.loc	1 1192 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp3375:
	.loc	1 1280 32
	vmovss	280(%rbx), %xmm0
.Ltmp3376:
	.loc	1 1192 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp3377:
	.loc	1 1280 32
	vmovss	440(%rbx), %xmm0
.Ltmp3378:
	.loc	1 1192 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp3379:
	.loc	1 1280 32
	vmovss	600(%rbx), %xmm0
.Ltmp3380:
	.loc	1 1192 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp3381:
	.loc	1 1280 32
	vmovss	760(%rbx), %xmm0
.Ltmp3382:
	.loc	1 1192 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp3383:
	.loc	1 1280 32
	vmovss	296(%rbx), %xmm0
.Ltmp3384:
	.loc	1 1192 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp3385:
	.loc	1 1280 32
	vmovss	456(%rbx), %xmm0
.Ltmp3386:
	.loc	1 1192 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp3387:
	.loc	1 1280 32
	vmovss	616(%rbx), %xmm0
.Ltmp3388:
	.loc	1 1192 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp3389:
	.loc	1 1280 32
	vmovss	776(%rbx), %xmm0
.Ltmp3390:
	.loc	1 1192 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp3391:
	.loc	1 1280 32
	vmovss	312(%rbx), %xmm0
.Ltmp3392:
	.loc	1 1192 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp3393:
	.loc	1 1280 32
	vmovss	472(%rbx), %xmm0
.Ltmp3394:
	.loc	1 1192 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp3395:
	.loc	1 1280 32
	vmovss	632(%rbx), %xmm0
.Ltmp3396:
	.loc	1 1192 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp3397:
	.loc	1 1280 32
	vmovss	792(%rbx), %xmm0
.Ltmp3398:
	.loc	1 1192 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp3399:
	.loc	1 1280 32
	vmovss	328(%rbx), %xmm0
.Ltmp3400:
	.loc	1 1192 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp3401:
	.loc	1 1280 32
	vmovss	488(%rbx), %xmm0
.Ltmp3402:
	.loc	1 1192 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp3403:
	.loc	1 1280 32
	vmovss	648(%rbx), %xmm0
.Ltmp3404:
	.loc	1 1192 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp3405:
	.loc	1 1280 32
	vmovss	808(%rbx), %xmm0
.Ltmp3406:
	.loc	1 1192 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp3407:
	.loc	1 1280 32
	vmovss	344(%rbx), %xmm0
.Ltmp3408:
	.loc	1 1192 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp3409:
	.loc	1 1280 32
	vmovss	504(%rbx), %xmm0
.Ltmp3410:
	.loc	1 1192 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp3411:
	.loc	1 1280 32
	vmovss	664(%rbx), %xmm0
.Ltmp3412:
	.loc	1 1192 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp3413:
	.loc	1 1280 32
	vmovss	824(%rbx), %xmm0
.Ltmp3414:
	.loc	1 1192 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp3415:
	.loc	1 1279 33
	vmovss	1520(%rbx), %xmm0
.Ltmp3416:
	.loc	1 1192 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp3417:
	.loc	1 1279 33
	vmovss	1680(%rbx), %xmm0
.Ltmp3418:
	.loc	1 1192 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp3419:
	.loc	1 1279 33
	vmovss	1840(%rbx), %xmm0
.Ltmp3420:
	.loc	1 1192 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp3421:
	.loc	1 1279 33
	vmovss	2000(%rbx), %xmm0
.Ltmp3422:
	.loc	1 1192 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp3423:
	.loc	1 1279 33
	vmovss	1536(%rbx), %xmm0
.Ltmp3424:
	.loc	1 1192 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp3425:
	.loc	1 1279 33
	vmovss	1696(%rbx), %xmm0
.Ltmp3426:
	.loc	1 1192 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp3427:
	.loc	1 1279 33
	vmovss	1856(%rbx), %xmm0
.Ltmp3428:
	.loc	1 1192 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp3429:
	.loc	1 1279 33
	vmovss	2016(%rbx), %xmm0
.Ltmp3430:
	.loc	1 1192 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp3431:
	.loc	1 1279 33
	vmovss	1552(%rbx), %xmm0
.Ltmp3432:
	.loc	1 1192 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp3433:
	.loc	1 1279 33
	vmovss	1712(%rbx), %xmm0
.Ltmp3434:
	.loc	1 1192 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp3435:
	.loc	1 1279 33
	vmovss	1872(%rbx), %xmm0
.Ltmp3436:
	.loc	1 1192 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp3437:
	.loc	1 1279 33
	vmovss	2032(%rbx), %xmm0
.Ltmp3438:
	.loc	1 1192 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp3439:
	.loc	1 1279 33
	vmovss	1568(%rbx), %xmm0
.Ltmp3440:
	.loc	1 1192 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp3441:
	.loc	1 1279 33
	vmovss	1728(%rbx), %xmm0
.Ltmp3442:
	.loc	1 1192 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp3443:
	.loc	1 1279 33
	vmovss	1888(%rbx), %xmm0
.Ltmp3444:
	.loc	1 1192 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp3445:
	.loc	1 1279 33
	vmovss	2048(%rbx), %xmm0
.Ltmp3446:
	.loc	1 1192 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp3447:
	.loc	1 1279 33
	vmovss	1584(%rbx), %xmm0
.Ltmp3448:
	.loc	1 1192 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp3449:
	.loc	1 1279 33
	vmovss	1744(%rbx), %xmm0
.Ltmp3450:
	.loc	1 1192 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp3451:
	.loc	1 1279 33
	vmovss	1904(%rbx), %xmm0
.Ltmp3452:
	.loc	1 1192 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp3453:
	.loc	1 1279 33
	vmovss	2064(%rbx), %xmm0
.Ltmp3454:
	.loc	1 1192 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp3455:
	.loc	1 1279 33
	vmovss	1600(%rbx), %xmm0
.Ltmp3456:
	.loc	1 1192 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp3457:
	.loc	1 1279 33
	vmovss	1760(%rbx), %xmm0
.Ltmp3458:
	.loc	1 1192 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp3459:
	.loc	1 1279 33
	vmovss	1920(%rbx), %xmm0
.Ltmp3460:
	.loc	1 1192 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp3461:
	.loc	1 1279 33
	vmovss	2080(%rbx), %xmm0
.Ltmp3462:
	.loc	1 1192 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp3463:
	.loc	1 1279 33
	vmovss	1616(%rbx), %xmm0
.Ltmp3464:
	.loc	1 1192 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp3465:
	.loc	1 1279 33
	vmovss	1776(%rbx), %xmm0
.Ltmp3466:
	.loc	1 1192 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp3467:
	.loc	1 1279 33
	vmovss	1936(%rbx), %xmm0
.Ltmp3468:
	.loc	1 1192 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp3469:
	.loc	1 1279 33
	vmovss	2096(%rbx), %xmm0
.Ltmp3470:
	.loc	1 1192 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp3471:
	.loc	1 1279 33
	vmovss	1632(%rbx), %xmm0
.Ltmp3472:
	.loc	1 1192 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp3473:
	.loc	1 1279 33
	vmovss	1792(%rbx), %xmm0
.Ltmp3474:
	.loc	1 1192 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp3475:
	.loc	1 1279 33
	vmovss	1952(%rbx), %xmm0
.Ltmp3476:
	.loc	1 1192 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp3477:
	.loc	1 1279 33
	vmovss	2112(%rbx), %xmm0
.Ltmp3478:
	.loc	1 1192 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp3479:
	.loc	1 1279 33
	vmovss	1648(%rbx), %xmm0
.Ltmp3480:
	.loc	1 1192 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp3481:
	.loc	1 1279 33
	vmovss	1808(%rbx), %xmm0
.Ltmp3482:
	.loc	1 1192 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp3483:
	.loc	1 1279 33
	vmovss	1968(%rbx), %xmm0
.Ltmp3484:
	.loc	1 1192 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp3485:
	.loc	1 1279 33
	vmovss	2128(%rbx), %xmm0
.Ltmp3486:
	.loc	1 1192 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp3487:
	.loc	1 1279 33
	vmovss	1664(%rbx), %xmm0
.Ltmp3488:
	.loc	1 1192 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp3489:
	.loc	1 1279 33
	vmovss	1824(%rbx), %xmm0
.Ltmp3490:
	.loc	1 1192 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp3491:
	.loc	1 1279 33
	vmovss	1984(%rbx), %xmm0
.Ltmp3492:
	.loc	1 1192 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp3493:
	.loc	1 1279 33
	vmovss	2144(%rbx), %xmm0
.Ltmp3494:
	.loc	1 1192 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp3495:
	.loc	1 1280 32
	vmovss	1528(%rbx), %xmm0
.Ltmp3496:
	.loc	1 1192 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp3497:
	.loc	1 1280 32
	vmovss	1688(%rbx), %xmm0
.Ltmp3498:
	.loc	1 1192 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp3499:
	.loc	1 1280 32
	vmovss	1848(%rbx), %xmm0
.Ltmp3500:
	.loc	1 1192 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp3501:
	.loc	1 1280 32
	vmovss	2008(%rbx), %xmm0
.Ltmp3502:
	.loc	1 1192 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp3503:
	.loc	1 1280 32
	vmovss	1544(%rbx), %xmm0
.Ltmp3504:
	.loc	1 1192 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp3505:
	.loc	1 1280 32
	vmovss	1704(%rbx), %xmm0
.Ltmp3506:
	.loc	1 1192 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp3507:
	.loc	1 1280 32
	vmovss	1864(%rbx), %xmm0
.Ltmp3508:
	.loc	1 1192 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp3509:
	.loc	1 1280 32
	vmovss	2024(%rbx), %xmm0
.Ltmp3510:
	.loc	1 1192 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp3511:
	.loc	1 1280 32
	vmovss	1560(%rbx), %xmm0
.Ltmp3512:
	.loc	1 1192 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp3513:
	.loc	1 1280 32
	vmovss	1720(%rbx), %xmm0
.Ltmp3514:
	.loc	1 1192 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp3515:
	.loc	1 1280 32
	vmovss	1880(%rbx), %xmm0
.Ltmp3516:
	.loc	1 1192 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp3517:
	.loc	1 1280 32
	vmovss	2040(%rbx), %xmm0
.Ltmp3518:
	.loc	1 1192 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp3519:
	.loc	1 1280 32
	vmovss	1576(%rbx), %xmm0
.Ltmp3520:
	.loc	1 1192 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp3521:
	.loc	1 1280 32
	vmovss	1736(%rbx), %xmm0
.Ltmp3522:
	.loc	1 1192 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp3523:
	.loc	1 1280 32
	vmovss	1896(%rbx), %xmm0
.Ltmp3524:
	.loc	1 1192 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp3525:
	.loc	1 1280 32
	vmovss	2056(%rbx), %xmm0
.Ltmp3526:
	.loc	1 1192 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp3527:
	.loc	1 1280 32
	vmovss	1592(%rbx), %xmm0
.Ltmp3528:
	.loc	1 1192 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp3529:
	.loc	1 1280 32
	vmovss	1752(%rbx), %xmm0
.Ltmp3530:
	.loc	1 1192 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp3531:
	.loc	1 1280 32
	vmovss	1912(%rbx), %xmm0
.Ltmp3532:
	.loc	1 1192 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp3533:
	.loc	1 1280 32
	vmovss	2072(%rbx), %xmm0
.Ltmp3534:
	.loc	1 1192 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp3535:
	.loc	1 1280 32
	vmovss	1608(%rbx), %xmm0
.Ltmp3536:
	.loc	1 1192 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp3537:
	.loc	1 1280 32
	vmovss	1768(%rbx), %xmm0
.Ltmp3538:
	.loc	1 1192 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp3539:
	.loc	1 1280 32
	vmovss	1928(%rbx), %xmm0
.Ltmp3540:
	.loc	1 1192 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp3541:
	.loc	1 1280 32
	vmovss	2088(%rbx), %xmm0
.Ltmp3542:
	.loc	1 1192 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp3543:
	.loc	1 1280 32
	vmovss	1624(%rbx), %xmm0
.Ltmp3544:
	.loc	1 1192 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp3545:
	.loc	1 1280 32
	vmovss	1784(%rbx), %xmm0
.Ltmp3546:
	.loc	1 1192 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp3547:
	.loc	1 1280 32
	vmovss	1944(%rbx), %xmm0
.Ltmp3548:
	.loc	1 1192 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp3549:
	.loc	1 1280 32
	vmovss	2104(%rbx), %xmm0
.Ltmp3550:
	.loc	1 1192 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp3551:
	.loc	1 1280 32
	vmovss	1640(%rbx), %xmm0
.Ltmp3552:
	.loc	1 1192 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp3553:
	.loc	1 1280 32
	vmovss	1800(%rbx), %xmm0
.Ltmp3554:
	.loc	1 1192 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp3555:
	.loc	1 1280 32
	vmovss	1960(%rbx), %xmm0
.Ltmp3556:
	.loc	1 1192 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp3557:
	.loc	1 1280 32
	vmovss	2120(%rbx), %xmm0
.Ltmp3558:
	.loc	1 1192 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp3559:
	.loc	1 1280 32
	vmovss	1656(%rbx), %xmm0
.Ltmp3560:
	.loc	1 1192 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp3561:
	.loc	1 1280 32
	vmovss	1816(%rbx), %xmm0
.Ltmp3562:
	.loc	1 1192 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp3563:
	.loc	1 1280 32
	vmovss	1976(%rbx), %xmm0
.Ltmp3564:
	.loc	1 1192 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp3565:
	.loc	1 1280 32
	vmovss	2136(%rbx), %xmm0
.Ltmp3566:
	.loc	1 1192 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp3567:
	.loc	1 1280 32
	vmovss	1672(%rbx), %xmm0
.Ltmp3568:
	.loc	1 1192 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp3569:
	.loc	1 1280 32
	vmovss	1832(%rbx), %xmm0
.Ltmp3570:
	.loc	1 1192 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp3571:
	.loc	1 1280 32
	vmovss	1992(%rbx), %xmm0
.Ltmp3572:
	.loc	1 1192 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp3573:
	.loc	1 1280 32
	vmovss	2152(%rbx), %xmm0
.Ltmp3574:
	.loc	1 1192 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp3575:
	.loc	1 1194 31
	leaq	1328(%rsp), %rdi
	movq	%rbx, %rsi
	movl	1024(%rsp), %r14d
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	1424(%rsp), %rdi
	movq	32(%rsp), %rsi
	movl	%r14d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
.Ltmp3576:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%r13), %rax
	shlq	$2, %r13
	leaq	(,%rax,4), %rsi
	.loc	1 1197 12 is_stmt 1
	testb	$1, 128(%rsp)
	movq	%rax, 176(%rsp)
	je	.LBB34_137
.Ltmp3577:
	.loc	38 1050 16
	cmpq	%r13, %rsi
.Ltmp3578:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_567
.Ltmp3579:
	.loc	48 451 16 is_stmt 1
	cmpq	112(%rsp), %rsi
	ja	.LBB34_567
.Ltmp3580:
	.loc	48 451 16 is_stmt 0
	cmpq	104(%rsp), %rsi
	ja	.LBB34_569
.Ltmp3581:
	.loc	48 0 16
	movq	16(%rsp), %rax
.Ltmp3582:
	.loc	1 1053 27 is_stmt 1
	vmovups	(%rax), %ymm0
	vmovups	32(%rax), %ymm1
	vmovups	%ymm1, 1920(%rsp)
	vmovups	%ymm0, 1888(%rsp)
	movq	80(%rsp), %rax
.Ltmp3583:
	.loc	1 1054 26
	vmovups	(%rax), %ymm0
	vmovups	32(%rax), %ymm1
	vmovups	%ymm1, 1984(%rsp)
	vmovups	%ymm0, 1952(%rsp)
	movq	64(%rsp), %rax
.Ltmp3584:
	.loc	1 1055 25
	vmovups	(%rax), %ymm0
	vmovups	%ymm0, 2016(%rsp)
	movq	48(%rsp), %rax
.Ltmp3585:
	.loc	1 1056 24
	vmovdqu	(%rax), %ymm0
	vmovdqu	%ymm0, 2048(%rsp)
.Ltmp3586:
	.loc	1 1057 24
	movq	2680(%rbx), %r9
.Ltmp3587:
	.loc	2 1916 50
	testq	%r15, %r15
	je	.LBB34_133
.Ltmp3588:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,4), %rdx
	movq	968(%rsp), %rax
	leaq	(%rax,%r13,4), %r10
	movq	120(%rsp), %rax
	leaq	(%rax,%r13,4), %r11
.Ltmp3589:
	.loc	3 900 12 is_stmt 1
	movq	%r15, %r14
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	xorl	%edi, %edi
	xorl	%r13d, %r13d
.Ltmp3590:
	.loc	3 0 12 is_stmt 0
.Ltmp3591:
	.p2align	4
.LBB34_119:
	.loc	1 1064 21 is_stmt 1
	vmovaps	208(%rsp), %xmm0
	vmovaps	224(%rsp), %xmm1
	vmovaps	240(%rsp), %xmm2
.Ltmp3592:
	.loc	9 36 14
	vaddps	368(%rsp), %xmm0, %xmm0
.Ltmp3593:
	.loc	1 1066 21
	vmovaps	528(%rsp), %xmm3
	.loc	1 1063 17
	vmovaps	%xmm0, 208(%rsp)
.Ltmp3594:
	.loc	9 36 14
	vaddps	688(%rsp), %xmm3, %xmm0
.Ltmp3595:
	.loc	1 1065 17
	vmovaps	%xmm0, 528(%rsp)
.Ltmp3596:
	.loc	9 36 14
	vaddps	384(%rsp), %xmm1, %xmm0
.Ltmp3597:
	.loc	1 1063 17
	vmovaps	%xmm0, 224(%rsp)
	.loc	1 1066 21
	vmovaps	544(%rsp), %xmm0
.Ltmp3598:
	.loc	9 36 14
	vaddps	704(%rsp), %xmm0, %xmm0
.Ltmp3599:
	.loc	1 1065 17
	vmovaps	%xmm0, 544(%rsp)
.Ltmp3600:
	.loc	9 36 14
	vaddps	400(%rsp), %xmm2, %xmm0
.Ltmp3601:
	.loc	1 1063 17
	vmovaps	%xmm0, 240(%rsp)
	.loc	1 1066 21
	vmovaps	560(%rsp), %xmm0
.Ltmp3602:
	.loc	9 36 14
	vaddps	720(%rsp), %xmm0, %xmm0
.Ltmp3603:
	.loc	1 1065 17
	vmovaps	%xmm0, 560(%rsp)
	.loc	1 1064 21
	vmovaps	256(%rsp), %xmm0
.Ltmp3604:
	.loc	9 36 14
	vaddps	416(%rsp), %xmm0, %xmm0
.Ltmp3605:
	.loc	1 1063 17
	vmovaps	%xmm0, 256(%rsp)
	.loc	1 1066 21
	vmovaps	576(%rsp), %xmm0
.Ltmp3606:
	.loc	9 36 14
	vaddps	736(%rsp), %xmm0, %xmm0
.Ltmp3607:
	.loc	1 1065 17
	vmovaps	%xmm0, 576(%rsp)
	.loc	1 1064 21
	vmovaps	272(%rsp), %xmm0
.Ltmp3608:
	.loc	9 36 14
	vaddps	432(%rsp), %xmm0, %xmm0
.Ltmp3609:
	.loc	1 1063 17
	vmovaps	%xmm0, 272(%rsp)
	.loc	1 1066 21
	vmovaps	592(%rsp), %xmm0
.Ltmp3610:
	.loc	9 36 14
	vaddps	752(%rsp), %xmm0, %xmm0
.Ltmp3611:
	.loc	1 1065 17
	vmovaps	%xmm0, 592(%rsp)
	.loc	1 1064 21
	vmovaps	288(%rsp), %xmm0
.Ltmp3612:
	.loc	9 36 14
	vaddps	448(%rsp), %xmm0, %xmm0
.Ltmp3613:
	.loc	1 1063 17
	vmovaps	%xmm0, 288(%rsp)
	.loc	1 1066 21
	vmovaps	608(%rsp), %xmm0
.Ltmp3614:
	.loc	9 36 14
	vaddps	768(%rsp), %xmm0, %xmm0
.Ltmp3615:
	.loc	1 1065 17
	vmovaps	%xmm0, 608(%rsp)
	.loc	1 1064 21
	vmovaps	304(%rsp), %xmm0
.Ltmp3616:
	.loc	9 36 14
	vaddps	464(%rsp), %xmm0, %xmm0
.Ltmp3617:
	.loc	1 1063 17
	vmovaps	%xmm0, 304(%rsp)
	.loc	1 1066 21
	vmovaps	624(%rsp), %xmm0
.Ltmp3618:
	.loc	9 36 14
	vaddps	784(%rsp), %xmm0, %xmm0
.Ltmp3619:
	.loc	1 1065 17
	vmovaps	%xmm0, 624(%rsp)
	.loc	1 1064 21
	vmovaps	320(%rsp), %xmm0
.Ltmp3620:
	.loc	9 36 14
	vaddps	480(%rsp), %xmm0, %xmm0
.Ltmp3621:
	.loc	1 1063 17
	vmovaps	%xmm0, 320(%rsp)
	.loc	1 1066 21
	vmovaps	640(%rsp), %xmm0
.Ltmp3622:
	.loc	9 36 14
	vaddps	800(%rsp), %xmm0, %xmm0
.Ltmp3623:
	.loc	1 1065 17
	vmovaps	%xmm0, 640(%rsp)
	.loc	1 1064 21
	vmovaps	336(%rsp), %xmm0
.Ltmp3624:
	.loc	9 36 14
	vaddps	496(%rsp), %xmm0, %xmm0
.Ltmp3625:
	.loc	1 1063 17
	vmovaps	%xmm0, 336(%rsp)
	.loc	1 1066 21
	vmovaps	656(%rsp), %xmm0
.Ltmp3626:
	.loc	9 36 14
	vaddps	816(%rsp), %xmm0, %xmm0
.Ltmp3627:
	.loc	1 1065 17
	vmovaps	%xmm0, 656(%rsp)
	.loc	1 1064 21
	vmovaps	352(%rsp), %xmm0
.Ltmp3628:
	.loc	9 36 14
	vaddps	512(%rsp), %xmm0, %xmm0
.Ltmp3629:
	.loc	1 1063 17
	vmovaps	%xmm0, 352(%rsp)
	.loc	1 1066 21
	vmovaps	672(%rsp), %xmm0
.Ltmp3630:
	.loc	9 36 14
	vaddps	832(%rsp), %xmm0, %xmm0
.Ltmp3631:
	.loc	1 1065 17
	vmovaps	%xmm0, 672(%rsp)
.Ltmp3632:
	.loc	1 1070 28
	leaq	1(%r9), %rax
.Ltmp3633:
	.loc	1 857 8
	cmpq	%r12, %rax
	jb	.LBB34_121
.Ltmp3634:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rbx
	jmp	.LBB34_122
	.p2align	4
.LBB34_121:
	xorl	%ebx, %ebx
.LBB34_122:
.Ltmp3635:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB34_535
.Ltmp3636:
	.loc	48 438 16
	cmpq	%r13, %r14
	je	.LBB34_534
.Ltmp3637:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r9,4), %rax
.Ltmp3638:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%r11,%rdi,4), %xmm0
	vmovdqa	%xmm0, 1520(%rsp)
	movq	1080(%rsp), %rcx
.Ltmp3639:
	.loc	1 1074 35
	movq	8(%rcx), %rsi
.Ltmp3640:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_536
.Ltmp3641:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp3642:
	.loc	1 0 0 is_stmt 0
	leaq	(%r10,%rdi,4), %rcx
	movq	1080(%rsp), %r8
.Ltmp3643:
	.loc	1 1074 35 is_stmt 1
	movq	(%r8), %rsi
.Ltmp3644:
	.loc	8 551 14
	vmovdqu	(%rcx), %xmm0
	vmovdqu	%xmm0, (%rsi,%rax,4)
.Ltmp3645:
	.loc	1 1075 34
	movq	1336(%r8), %rsi
.Ltmp3646:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_537
.Ltmp3647:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp3648:
	.loc	1 0 0 is_stmt 0
	negq	%rbx
	addq	%rbx, %r9
	incq	%r9
	leaq	(,%r9,4), %r8
	movq	1080(%rsp), %rbx
.Ltmp3649:
	.loc	1 1075 34 is_stmt 1
	movq	1328(%rbx), %rsi
.Ltmp3650:
	.loc	8 551 14
	vmovaps	1520(%rsp), %xmm0
	vmovups	%xmm0, (%rsi,%rax,4)
.Ltmp3651:
	.loc	1 1076 22
	movq	8(%rbx), %rsi
.Ltmp3652:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB34_538
.Ltmp3653:
	.loc	48 438 16
	cmpq	$3, %rax
	jbe	.LBB34_531
.Ltmp3654:
	.loc	1 1076 22
	movq	(%rbx), %rax
.Ltmp3655:
	.loc	8 551 14
	vmovups	(%rax,%r8,4), %xmm0
	vmovups	%xmm0, (%rcx)
.Ltmp3656:
	.loc	1 1077 22
	movq	1336(%rbx), %rsi
.Ltmp3657:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB34_539
.Ltmp3658:
	.loc	48 438 16
	cmpq	$3, %rax
	jbe	.LBB34_531
.Ltmp3659:
	.loc	1 0 0 is_stmt 0
	incq	%r13
.Ltmp3660:
	leaq	(%r11,%rdi,4), %rax
	movq	32(%rsp), %rcx
.Ltmp3661:
	.loc	1 1077 22 is_stmt 1
	movq	(%rcx), %rcx
.Ltmp3662:
	.loc	8 551 14
	vmovdqu	(%rcx,%r8,4), %xmm0
	vmovdqu	%xmm0, (%rax)
.Ltmp3663:
	.loc	2 1916 50
	addq	$4, %rdi
	cmpq	%r13, %r15
.Ltmp3664:
	.loc	3 900 12
	jne	.LBB34_119
.Ltmp3665:
.LBB34_133:
	.loc	1 1160 5
	vmovups	1888(%rsp), %ymm0
	vmovups	1920(%rsp), %ymm1
	movq	16(%rsp), %rax
	vmovups	%ymm1, 32(%rax)
	vmovups	%ymm0, (%rax)
	.loc	1 1161 5
	vmovups	1952(%rsp), %ymm0
	vmovups	1984(%rsp), %ymm1
	movq	80(%rsp), %rax
	vmovups	%ymm1, 32(%rax)
	vmovups	%ymm0, (%rax)
	.loc	1 1162 5
	vmovups	2016(%rsp), %ymm0
	movq	64(%rsp), %rax
	vmovups	%ymm0, (%rax)
	.loc	1 1163 5
	vmovups	2048(%rsp), %ymm0
	movq	48(%rsp), %rax
	vmovups	%ymm0, (%rax)
	.loc	1 1164 5
	movq	%r9, 2680(%rbx)
	xorl	%eax, %eax
.Ltmp3666:
	.loc	1 0 5 is_stmt 0
.Ltmp3667:
	.p2align	4
.LBB34_134:
	.loc	1 1297 13 is_stmt 1
	vmovd	208(%rsp,%rax), %xmm0
	vmovss	212(%rsp,%rax), %xmm1
	vmovss	216(%rsp,%rax), %xmm2
	vmovss	220(%rsp,%rax), %xmm3
.Ltmp3668:
	.loc	1 1300 17
	vmovd	%xmm0, 192(%rbx,%rax)
	.loc	1 1301 34
	movl	204(%rbx,%rax), %ecx
	movl	364(%rbx,%rax), %edx
.Ltmp3669:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3670:
	.loc	1 1301 17
	movl	%ecx, 204(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 352(%rbx,%rax)
.Ltmp3671:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%ebp, %edx
.Ltmp3672:
	.loc	1 1301 17
	movl	%edx, 364(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 512(%rbx,%rax)
	.loc	1 1301 34
	movl	524(%rbx,%rax), %ecx
.Ltmp3673:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3674:
	.loc	1 1301 17
	movl	%ecx, 524(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm3, 672(%rbx,%rax)
	.loc	1 1301 34
	movl	684(%rbx,%rax), %ecx
.Ltmp3675:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3676:
	.loc	1 1301 17
	movl	%ecx, 684(%rbx,%rax)
.Ltmp3677:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp3678:
	.loc	3 900 12
	jne	.LBB34_134
.Ltmp3679:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	984(%rsp), %rsi
	movq	120(%rsp), %r9
	.p2align	4
.LBB34_136:
.Ltmp3680:
	.loc	1 1297 13 is_stmt 1
	vmovd	528(%rsp,%rax), %xmm0
	vmovss	532(%rsp,%rax), %xmm1
	vmovss	536(%rsp,%rax), %xmm2
	vmovss	540(%rsp,%rax), %xmm3
.Ltmp3681:
	.loc	1 1300 17
	vmovd	%xmm0, 1520(%rbx,%rax)
	.loc	1 1301 34
	movl	1532(%rbx,%rax), %ecx
	movl	1692(%rbx,%rax), %edx
.Ltmp3682:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3683:
	.loc	1 1301 17
	movl	%ecx, 1532(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 1680(%rbx,%rax)
.Ltmp3684:
	.loc	38 2472 13
	subl	%r15d, %edx
	cmovbl	%ebp, %edx
.Ltmp3685:
	.loc	1 1301 17
	movl	%edx, 1692(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 1840(%rbx,%rax)
	.loc	1 1301 34
	movl	1852(%rbx,%rax), %ecx
.Ltmp3686:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3687:
	.loc	1 1301 17
	movl	%ecx, 1852(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm3, 2000(%rbx,%rax)
	.loc	1 1301 34
	movl	2012(%rbx,%rax), %ecx
.Ltmp3688:
	.loc	38 2472 13
	subl	%r15d, %ecx
	cmovbl	%ebp, %ecx
.Ltmp3689:
	.loc	1 1301 17
	movl	%ecx, 2012(%rbx,%rax)
.Ltmp3690:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp3691:
	.loc	3 900 12
	jne	.LBB34_136
	jmp	.LBB34_112
.Ltmp3692:
	.loc	3 0 12 is_stmt 0
.Ltmp3693:
	.p2align	4
.LBB34_137:
	.loc	38 1050 16 is_stmt 1
	cmpq	%r13, %rsi
.Ltmp3694:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_566
.Ltmp3695:
	.loc	48 451 16 is_stmt 1
	cmpq	112(%rsp), %rsi
	ja	.LBB34_566
.Ltmp3696:
	.loc	48 451 16 is_stmt 0
	cmpq	104(%rsp), %rsi
	ja	.LBB34_568
.Ltmp3697:
	.loc	48 0 16
	movq	16(%rsp), %rax
.Ltmp3698:
	.loc	1 1053 27 is_stmt 1
	vmovups	(%rax), %ymm0
	vmovups	32(%rax), %ymm1
	vmovups	%ymm1, 2112(%rsp)
	vmovups	%ymm0, 2080(%rsp)
	movq	80(%rsp), %rax
.Ltmp3699:
	.loc	1 1054 26
	vmovups	(%rax), %ymm0
	vmovups	32(%rax), %ymm1
	vmovups	%ymm1, 2176(%rsp)
	vmovups	%ymm0, 2144(%rsp)
	movq	64(%rsp), %rax
.Ltmp3700:
	.loc	1 1055 25
	vmovups	(%rax), %ymm0
	vmovups	%ymm0, 2208(%rsp)
	movq	48(%rsp), %rax
.Ltmp3701:
	.loc	1 1056 24
	vmovdqu	(%rax), %ymm0
	vmovdqu	%ymm0, 2240(%rsp)
.Ltmp3702:
	.loc	1 1057 24
	movq	2680(%rbx), %r9
.Ltmp3703:
	.loc	2 1916 50
	testq	%r15, %r15
	je	.LBB34_111
.Ltmp3704:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r15,4), %rdx
	movq	968(%rsp), %rax
	leaq	(%rax,%r13,4), %r10
	movq	120(%rsp), %rax
	leaq	(%rax,%r13,4), %r11
.Ltmp3705:
	.loc	48 568 12 is_stmt 1
	movq	%r15, %r14
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	xorl	%edi, %edi
	xorl	%r13d, %r13d
.Ltmp3706:
	.loc	48 0 12 is_stmt 0
.Ltmp3707:
	.p2align	4
.LBB34_142:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%r9), %rax
.Ltmp3708:
	.loc	1 857 8
	cmpq	%r12, %rax
	jb	.LBB34_144
.Ltmp3709:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rbx
	jmp	.LBB34_145
	.p2align	4
.LBB34_144:
	xorl	%ebx, %ebx
.LBB34_145:
.Ltmp3710:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB34_535
.Ltmp3711:
	.loc	48 438 16
	cmpq	%r13, %r14
	je	.LBB34_534
.Ltmp3712:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r9,4), %rax
.Ltmp3713:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%r11,%rdi,4), %xmm0
	vmovdqa	%xmm0, 1536(%rsp)
	movq	1080(%rsp), %rcx
.Ltmp3714:
	.loc	1 1074 35
	movq	8(%rcx), %rsi
.Ltmp3715:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_536
.Ltmp3716:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp3717:
	.loc	1 0 0 is_stmt 0
	leaq	(%r10,%rdi,4), %rcx
	movq	1080(%rsp), %r8
.Ltmp3718:
	.loc	1 1074 35 is_stmt 1
	movq	(%r8), %rsi
.Ltmp3719:
	.loc	8 551 14
	vmovdqu	(%rcx), %xmm0
	vmovdqu	%xmm0, (%rsi,%rax,4)
.Ltmp3720:
	.loc	1 1075 34
	movq	1336(%r8), %rsi
.Ltmp3721:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_537
.Ltmp3722:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp3723:
	.loc	1 0 0 is_stmt 0
	negq	%rbx
	addq	%rbx, %r9
	incq	%r9
	leaq	(,%r9,4), %r8
	movq	1080(%rsp), %rbx
.Ltmp3724:
	.loc	1 1075 34 is_stmt 1
	movq	1328(%rbx), %rsi
.Ltmp3725:
	.loc	8 551 14
	vmovaps	1536(%rsp), %xmm0
	vmovups	%xmm0, (%rsi,%rax,4)
.Ltmp3726:
	.loc	1 1076 22
	movq	8(%rbx), %rsi
.Ltmp3727:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB34_538
.Ltmp3728:
	.loc	48 438 16
	cmpq	$3, %rax
	jbe	.LBB34_531
.Ltmp3729:
	.loc	1 1076 22
	movq	(%rbx), %rax
.Ltmp3730:
	.loc	8 551 14
	vmovups	(%rax,%r8,4), %xmm0
	vmovups	%xmm0, (%rcx)
.Ltmp3731:
	.loc	1 1077 22
	movq	1336(%rbx), %rsi
.Ltmp3732:
	.loc	48 568 12
	movq	%rsi, %rax
	subq	%r8, %rax
	jb	.LBB34_539
.Ltmp3733:
	.loc	48 438 16
	cmpq	$3, %rax
	jbe	.LBB34_531
.Ltmp3734:
	.loc	1 0 0 is_stmt 0
	incq	%r13
.Ltmp3735:
	leaq	(%r11,%rdi,4), %rax
	movq	32(%rsp), %rcx
.Ltmp3736:
	.loc	1 1077 22 is_stmt 1
	movq	(%rcx), %rcx
.Ltmp3737:
	.loc	8 551 14
	vmovdqu	(%rcx,%r8,4), %xmm0
	vmovdqu	%xmm0, (%rax)
.Ltmp3738:
	.loc	2 1916 50
	addq	$4, %rdi
	cmpq	%r13, %r15
.Ltmp3739:
	.loc	3 900 12
	jne	.LBB34_142
	jmp	.LBB34_111
.Ltmp3740:
.LBB34_156:
	.loc	3 0 12 is_stmt 0
	vmovaps	80(%rsp), %xmm0
.Ltmp3741:
	.loc	1 1160 5 is_stmt 1
	vmovaps	%xmm0, 96(%rbx)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 112(%rbx)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 128(%rbx)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 144(%rbx)
	vmovaps	32(%rsp), %xmm0
	.loc	1 1161 5
	vmovaps	%xmm0, 1424(%rbx)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%rbx)
	vmovaps	912(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%rbx)
	vmovaps	176(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%rbx)
	vmovaps	864(%rsp), %xmm0
	.loc	1 1162 5
	vmovaps	%xmm0, 160(%rbx)
	vmovaps	896(%rsp), %xmm0
	vmovaps	%xmm0, 176(%rbx)
	vmovdqa	880(%rsp), %xmm0
	.loc	1 1163 5
	vmovdqa	%xmm0, 1488(%rbx)
	vmovaps	%xmm6, 1504(%rbx)
	.loc	1 1164 5
	movq	%r10, 2680(%rbx)
	movq	984(%rsp), %rsi
	movq	120(%rsp), %r9
.Ltmp3742:
.LBB34_157:
	.loc	1 0 5 is_stmt 0
	movq	976(%rsp), %r15
	.loc	1 1189 11 is_stmt 1
	cmpq	%rsi, %r15
	jae	.LBB34_503
.LBB34_158:
	.loc	1 1190 42
	subq	%r15, %rsi
	.loc	1 1190 29 is_stmt 0
	movq	%rbx, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r14
.Ltmp3743:
	.loc	1 1279 33 is_stmt 1
	vmovss	192(%rbx), %xmm0
.Ltmp3744:
	.loc	1 1192 28
	vmovss	%xmm0, 208(%rsp)
.Ltmp3745:
	.loc	1 1279 33
	vmovss	352(%rbx), %xmm0
.Ltmp3746:
	.loc	1 1192 28
	vmovss	%xmm0, 212(%rsp)
.Ltmp3747:
	.loc	1 1279 33
	vmovss	512(%rbx), %xmm0
.Ltmp3748:
	.loc	1 1192 28
	vmovss	%xmm0, 216(%rsp)
.Ltmp3749:
	.loc	1 1279 33
	vmovss	672(%rbx), %xmm0
.Ltmp3750:
	.loc	1 1192 28
	vmovss	%xmm0, 220(%rsp)
.Ltmp3751:
	.loc	1 1279 33
	vmovss	208(%rbx), %xmm0
.Ltmp3752:
	.loc	1 1192 28
	vmovss	%xmm0, 224(%rsp)
.Ltmp3753:
	.loc	1 1279 33
	vmovss	368(%rbx), %xmm0
.Ltmp3754:
	.loc	1 1192 28
	vmovss	%xmm0, 228(%rsp)
.Ltmp3755:
	.loc	1 1279 33
	vmovss	528(%rbx), %xmm0
.Ltmp3756:
	.loc	1 1192 28
	vmovss	%xmm0, 232(%rsp)
.Ltmp3757:
	.loc	1 1279 33
	vmovss	688(%rbx), %xmm0
.Ltmp3758:
	.loc	1 1192 28
	vmovss	%xmm0, 236(%rsp)
.Ltmp3759:
	.loc	1 1279 33
	vmovss	224(%rbx), %xmm0
.Ltmp3760:
	.loc	1 1192 28
	vmovss	%xmm0, 240(%rsp)
.Ltmp3761:
	.loc	1 1279 33
	vmovss	384(%rbx), %xmm0
.Ltmp3762:
	.loc	1 1192 28
	vmovss	%xmm0, 244(%rsp)
.Ltmp3763:
	.loc	1 1279 33
	vmovss	544(%rbx), %xmm0
.Ltmp3764:
	.loc	1 1192 28
	vmovss	%xmm0, 248(%rsp)
.Ltmp3765:
	.loc	1 1279 33
	vmovss	704(%rbx), %xmm0
.Ltmp3766:
	.loc	1 1192 28
	vmovss	%xmm0, 252(%rsp)
.Ltmp3767:
	.loc	1 1279 33
	vmovss	240(%rbx), %xmm0
.Ltmp3768:
	.loc	1 1192 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp3769:
	.loc	1 1279 33
	vmovss	400(%rbx), %xmm0
.Ltmp3770:
	.loc	1 1192 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp3771:
	.loc	1 1279 33
	vmovss	560(%rbx), %xmm0
.Ltmp3772:
	.loc	1 1192 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp3773:
	.loc	1 1279 33
	vmovss	720(%rbx), %xmm0
.Ltmp3774:
	.loc	1 1192 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp3775:
	.loc	1 1279 33
	vmovss	256(%rbx), %xmm0
.Ltmp3776:
	.loc	1 1192 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp3777:
	.loc	1 1279 33
	vmovss	416(%rbx), %xmm0
.Ltmp3778:
	.loc	1 1192 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp3779:
	.loc	1 1279 33
	vmovss	576(%rbx), %xmm0
.Ltmp3780:
	.loc	1 1192 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp3781:
	.loc	1 1279 33
	vmovss	736(%rbx), %xmm0
.Ltmp3782:
	.loc	1 1192 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp3783:
	.loc	1 1279 33
	vmovss	272(%rbx), %xmm0
.Ltmp3784:
	.loc	1 1192 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp3785:
	.loc	1 1279 33
	vmovss	432(%rbx), %xmm0
.Ltmp3786:
	.loc	1 1192 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp3787:
	.loc	1 1279 33
	vmovss	592(%rbx), %xmm0
.Ltmp3788:
	.loc	1 1192 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp3789:
	.loc	1 1279 33
	vmovss	752(%rbx), %xmm0
.Ltmp3790:
	.loc	1 1192 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp3791:
	.loc	1 1279 33
	vmovss	288(%rbx), %xmm0
.Ltmp3792:
	.loc	1 1192 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp3793:
	.loc	1 1279 33
	vmovss	448(%rbx), %xmm0
.Ltmp3794:
	.loc	1 1192 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp3795:
	.loc	1 1279 33
	vmovss	608(%rbx), %xmm0
.Ltmp3796:
	.loc	1 1192 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp3797:
	.loc	1 1279 33
	vmovss	768(%rbx), %xmm0
.Ltmp3798:
	.loc	1 1192 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp3799:
	.loc	1 1279 33
	vmovss	304(%rbx), %xmm0
.Ltmp3800:
	.loc	1 1192 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp3801:
	.loc	1 1279 33
	vmovss	464(%rbx), %xmm0
.Ltmp3802:
	.loc	1 1192 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp3803:
	.loc	1 1279 33
	vmovss	624(%rbx), %xmm0
.Ltmp3804:
	.loc	1 1192 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp3805:
	.loc	1 1279 33
	vmovss	784(%rbx), %xmm0
.Ltmp3806:
	.loc	1 1192 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp3807:
	.loc	1 1279 33
	vmovss	320(%rbx), %xmm0
.Ltmp3808:
	.loc	1 1192 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp3809:
	.loc	1 1279 33
	vmovss	480(%rbx), %xmm0
.Ltmp3810:
	.loc	1 1192 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp3811:
	.loc	1 1279 33
	vmovss	640(%rbx), %xmm0
.Ltmp3812:
	.loc	1 1192 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp3813:
	.loc	1 1279 33
	vmovss	800(%rbx), %xmm0
.Ltmp3814:
	.loc	1 1192 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp3815:
	.loc	1 1279 33
	vmovss	336(%rbx), %xmm0
.Ltmp3816:
	.loc	1 1192 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp3817:
	.loc	1 1279 33
	vmovss	496(%rbx), %xmm0
.Ltmp3818:
	.loc	1 1192 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp3819:
	.loc	1 1279 33
	vmovss	656(%rbx), %xmm0
.Ltmp3820:
	.loc	1 1192 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp3821:
	.loc	1 1279 33
	vmovss	816(%rbx), %xmm0
.Ltmp3822:
	.loc	1 1192 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp3823:
	.loc	1 1280 32
	vmovss	200(%rbx), %xmm0
.Ltmp3824:
	.loc	1 1192 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp3825:
	.loc	1 1280 32
	vmovss	360(%rbx), %xmm0
.Ltmp3826:
	.loc	1 1192 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp3827:
	.loc	1 1280 32
	vmovss	520(%rbx), %xmm0
.Ltmp3828:
	.loc	1 1192 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp3829:
	.loc	1 1280 32
	vmovss	680(%rbx), %xmm0
.Ltmp3830:
	.loc	1 1192 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp3831:
	.loc	1 1280 32
	vmovss	216(%rbx), %xmm0
.Ltmp3832:
	.loc	1 1192 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp3833:
	.loc	1 1280 32
	vmovss	376(%rbx), %xmm0
.Ltmp3834:
	.loc	1 1192 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp3835:
	.loc	1 1280 32
	vmovss	536(%rbx), %xmm0
.Ltmp3836:
	.loc	1 1192 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp3837:
	.loc	1 1280 32
	vmovss	696(%rbx), %xmm0
.Ltmp3838:
	.loc	1 1192 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp3839:
	.loc	1 1280 32
	vmovss	232(%rbx), %xmm0
.Ltmp3840:
	.loc	1 1192 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp3841:
	.loc	1 1280 32
	vmovss	392(%rbx), %xmm0
.Ltmp3842:
	.loc	1 1192 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp3843:
	.loc	1 1280 32
	vmovss	552(%rbx), %xmm0
.Ltmp3844:
	.loc	1 1192 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp3845:
	.loc	1 1280 32
	vmovss	712(%rbx), %xmm0
.Ltmp3846:
	.loc	1 1192 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp3847:
	.loc	1 1280 32
	vmovss	248(%rbx), %xmm0
.Ltmp3848:
	.loc	1 1192 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp3849:
	.loc	1 1280 32
	vmovss	408(%rbx), %xmm0
.Ltmp3850:
	.loc	1 1192 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp3851:
	.loc	1 1280 32
	vmovss	568(%rbx), %xmm0
.Ltmp3852:
	.loc	1 1192 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp3853:
	.loc	1 1280 32
	vmovss	728(%rbx), %xmm0
.Ltmp3854:
	.loc	1 1192 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp3855:
	.loc	1 1280 32
	vmovss	264(%rbx), %xmm0
.Ltmp3856:
	.loc	1 1192 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp3857:
	.loc	1 1280 32
	vmovss	424(%rbx), %xmm0
.Ltmp3858:
	.loc	1 1192 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp3859:
	.loc	1 1280 32
	vmovss	584(%rbx), %xmm0
.Ltmp3860:
	.loc	1 1192 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp3861:
	.loc	1 1280 32
	vmovss	744(%rbx), %xmm0
.Ltmp3862:
	.loc	1 1192 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp3863:
	.loc	1 1280 32
	vmovss	280(%rbx), %xmm0
.Ltmp3864:
	.loc	1 1192 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp3865:
	.loc	1 1280 32
	vmovss	440(%rbx), %xmm0
.Ltmp3866:
	.loc	1 1192 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp3867:
	.loc	1 1280 32
	vmovss	600(%rbx), %xmm0
.Ltmp3868:
	.loc	1 1192 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp3869:
	.loc	1 1280 32
	vmovss	760(%rbx), %xmm0
.Ltmp3870:
	.loc	1 1192 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp3871:
	.loc	1 1280 32
	vmovss	296(%rbx), %xmm0
.Ltmp3872:
	.loc	1 1192 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp3873:
	.loc	1 1280 32
	vmovss	456(%rbx), %xmm0
.Ltmp3874:
	.loc	1 1192 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp3875:
	.loc	1 1280 32
	vmovss	616(%rbx), %xmm0
.Ltmp3876:
	.loc	1 1192 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp3877:
	.loc	1 1280 32
	vmovss	776(%rbx), %xmm0
.Ltmp3878:
	.loc	1 1192 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp3879:
	.loc	1 1280 32
	vmovss	312(%rbx), %xmm0
.Ltmp3880:
	.loc	1 1192 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp3881:
	.loc	1 1280 32
	vmovss	472(%rbx), %xmm0
.Ltmp3882:
	.loc	1 1192 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp3883:
	.loc	1 1280 32
	vmovss	632(%rbx), %xmm0
.Ltmp3884:
	.loc	1 1192 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp3885:
	.loc	1 1280 32
	vmovss	792(%rbx), %xmm0
.Ltmp3886:
	.loc	1 1192 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp3887:
	.loc	1 1280 32
	vmovss	328(%rbx), %xmm0
.Ltmp3888:
	.loc	1 1192 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp3889:
	.loc	1 1280 32
	vmovss	488(%rbx), %xmm0
.Ltmp3890:
	.loc	1 1192 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp3891:
	.loc	1 1280 32
	vmovss	648(%rbx), %xmm0
.Ltmp3892:
	.loc	1 1192 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp3893:
	.loc	1 1280 32
	vmovss	808(%rbx), %xmm0
.Ltmp3894:
	.loc	1 1192 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp3895:
	.loc	1 1280 32
	vmovss	344(%rbx), %xmm0
.Ltmp3896:
	.loc	1 1192 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp3897:
	.loc	1 1280 32
	vmovss	504(%rbx), %xmm0
.Ltmp3898:
	.loc	1 1192 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp3899:
	.loc	1 1280 32
	vmovss	664(%rbx), %xmm0
.Ltmp3900:
	.loc	1 1192 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp3901:
	.loc	1 1280 32
	vmovss	824(%rbx), %xmm0
.Ltmp3902:
	.loc	1 1192 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp3903:
	.loc	1 1279 33
	vmovss	1520(%rbx), %xmm0
.Ltmp3904:
	.loc	1 1192 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp3905:
	.loc	1 1279 33
	vmovss	1680(%rbx), %xmm0
.Ltmp3906:
	.loc	1 1192 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp3907:
	.loc	1 1279 33
	vmovss	1840(%rbx), %xmm0
.Ltmp3908:
	.loc	1 1192 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp3909:
	.loc	1 1279 33
	vmovss	2000(%rbx), %xmm0
.Ltmp3910:
	.loc	1 1192 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp3911:
	.loc	1 1279 33
	vmovss	1536(%rbx), %xmm0
.Ltmp3912:
	.loc	1 1192 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp3913:
	.loc	1 1279 33
	vmovss	1696(%rbx), %xmm0
.Ltmp3914:
	.loc	1 1192 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp3915:
	.loc	1 1279 33
	vmovss	1856(%rbx), %xmm0
.Ltmp3916:
	.loc	1 1192 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp3917:
	.loc	1 1279 33
	vmovss	2016(%rbx), %xmm0
.Ltmp3918:
	.loc	1 1192 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp3919:
	.loc	1 1279 33
	vmovss	1552(%rbx), %xmm0
.Ltmp3920:
	.loc	1 1192 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp3921:
	.loc	1 1279 33
	vmovss	1712(%rbx), %xmm0
.Ltmp3922:
	.loc	1 1192 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp3923:
	.loc	1 1279 33
	vmovss	1872(%rbx), %xmm0
.Ltmp3924:
	.loc	1 1192 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp3925:
	.loc	1 1279 33
	vmovss	2032(%rbx), %xmm0
.Ltmp3926:
	.loc	1 1192 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp3927:
	.loc	1 1279 33
	vmovss	1568(%rbx), %xmm0
.Ltmp3928:
	.loc	1 1192 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp3929:
	.loc	1 1279 33
	vmovss	1728(%rbx), %xmm0
.Ltmp3930:
	.loc	1 1192 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp3931:
	.loc	1 1279 33
	vmovss	1888(%rbx), %xmm0
.Ltmp3932:
	.loc	1 1192 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp3933:
	.loc	1 1279 33
	vmovss	2048(%rbx), %xmm0
.Ltmp3934:
	.loc	1 1192 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp3935:
	.loc	1 1279 33
	vmovss	1584(%rbx), %xmm0
.Ltmp3936:
	.loc	1 1192 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp3937:
	.loc	1 1279 33
	vmovss	1744(%rbx), %xmm0
.Ltmp3938:
	.loc	1 1192 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp3939:
	.loc	1 1279 33
	vmovss	1904(%rbx), %xmm0
.Ltmp3940:
	.loc	1 1192 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp3941:
	.loc	1 1279 33
	vmovss	2064(%rbx), %xmm0
.Ltmp3942:
	.loc	1 1192 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp3943:
	.loc	1 1279 33
	vmovss	1600(%rbx), %xmm0
.Ltmp3944:
	.loc	1 1192 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp3945:
	.loc	1 1279 33
	vmovss	1760(%rbx), %xmm0
.Ltmp3946:
	.loc	1 1192 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp3947:
	.loc	1 1279 33
	vmovss	1920(%rbx), %xmm0
.Ltmp3948:
	.loc	1 1192 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp3949:
	.loc	1 1279 33
	vmovss	2080(%rbx), %xmm0
.Ltmp3950:
	.loc	1 1192 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp3951:
	.loc	1 1279 33
	vmovss	1616(%rbx), %xmm0
.Ltmp3952:
	.loc	1 1192 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp3953:
	.loc	1 1279 33
	vmovss	1776(%rbx), %xmm0
.Ltmp3954:
	.loc	1 1192 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp3955:
	.loc	1 1279 33
	vmovss	1936(%rbx), %xmm0
.Ltmp3956:
	.loc	1 1192 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp3957:
	.loc	1 1279 33
	vmovss	2096(%rbx), %xmm0
.Ltmp3958:
	.loc	1 1192 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp3959:
	.loc	1 1279 33
	vmovss	1632(%rbx), %xmm0
.Ltmp3960:
	.loc	1 1192 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp3961:
	.loc	1 1279 33
	vmovss	1792(%rbx), %xmm0
.Ltmp3962:
	.loc	1 1192 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp3963:
	.loc	1 1279 33
	vmovss	1952(%rbx), %xmm0
.Ltmp3964:
	.loc	1 1192 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp3965:
	.loc	1 1279 33
	vmovss	2112(%rbx), %xmm0
.Ltmp3966:
	.loc	1 1192 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp3967:
	.loc	1 1279 33
	vmovss	1648(%rbx), %xmm0
.Ltmp3968:
	.loc	1 1192 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp3969:
	.loc	1 1279 33
	vmovss	1808(%rbx), %xmm0
.Ltmp3970:
	.loc	1 1192 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp3971:
	.loc	1 1279 33
	vmovss	1968(%rbx), %xmm0
.Ltmp3972:
	.loc	1 1192 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp3973:
	.loc	1 1279 33
	vmovss	2128(%rbx), %xmm0
.Ltmp3974:
	.loc	1 1192 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp3975:
	.loc	1 1279 33
	vmovss	1664(%rbx), %xmm0
.Ltmp3976:
	.loc	1 1192 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp3977:
	.loc	1 1279 33
	vmovss	1824(%rbx), %xmm0
.Ltmp3978:
	.loc	1 1192 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp3979:
	.loc	1 1279 33
	vmovss	1984(%rbx), %xmm0
.Ltmp3980:
	.loc	1 1192 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp3981:
	.loc	1 1279 33
	vmovss	2144(%rbx), %xmm0
.Ltmp3982:
	.loc	1 1192 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp3983:
	.loc	1 1280 32
	vmovss	1528(%rbx), %xmm0
.Ltmp3984:
	.loc	1 1192 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp3985:
	.loc	1 1280 32
	vmovss	1688(%rbx), %xmm0
.Ltmp3986:
	.loc	1 1192 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp3987:
	.loc	1 1280 32
	vmovss	1848(%rbx), %xmm0
.Ltmp3988:
	.loc	1 1192 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp3989:
	.loc	1 1280 32
	vmovss	2008(%rbx), %xmm0
.Ltmp3990:
	.loc	1 1192 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp3991:
	.loc	1 1280 32
	vmovss	1544(%rbx), %xmm0
.Ltmp3992:
	.loc	1 1192 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp3993:
	.loc	1 1280 32
	vmovss	1704(%rbx), %xmm0
.Ltmp3994:
	.loc	1 1192 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp3995:
	.loc	1 1280 32
	vmovss	1864(%rbx), %xmm0
.Ltmp3996:
	.loc	1 1192 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp3997:
	.loc	1 1280 32
	vmovss	2024(%rbx), %xmm0
.Ltmp3998:
	.loc	1 1192 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp3999:
	.loc	1 1280 32
	vmovss	1560(%rbx), %xmm0
.Ltmp4000:
	.loc	1 1192 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp4001:
	.loc	1 1280 32
	vmovss	1720(%rbx), %xmm0
.Ltmp4002:
	.loc	1 1192 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp4003:
	.loc	1 1280 32
	vmovss	1880(%rbx), %xmm0
.Ltmp4004:
	.loc	1 1192 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp4005:
	.loc	1 1280 32
	vmovss	2040(%rbx), %xmm0
.Ltmp4006:
	.loc	1 1192 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp4007:
	.loc	1 1280 32
	vmovss	1576(%rbx), %xmm0
.Ltmp4008:
	.loc	1 1192 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp4009:
	.loc	1 1280 32
	vmovss	1736(%rbx), %xmm0
.Ltmp4010:
	.loc	1 1192 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp4011:
	.loc	1 1280 32
	vmovss	1896(%rbx), %xmm0
.Ltmp4012:
	.loc	1 1192 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp4013:
	.loc	1 1280 32
	vmovss	2056(%rbx), %xmm0
.Ltmp4014:
	.loc	1 1192 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp4015:
	.loc	1 1280 32
	vmovss	1592(%rbx), %xmm0
.Ltmp4016:
	.loc	1 1192 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp4017:
	.loc	1 1280 32
	vmovss	1752(%rbx), %xmm0
.Ltmp4018:
	.loc	1 1192 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp4019:
	.loc	1 1280 32
	vmovss	1912(%rbx), %xmm0
.Ltmp4020:
	.loc	1 1192 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp4021:
	.loc	1 1280 32
	vmovss	2072(%rbx), %xmm0
.Ltmp4022:
	.loc	1 1192 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp4023:
	.loc	1 1280 32
	vmovss	1608(%rbx), %xmm0
.Ltmp4024:
	.loc	1 1192 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp4025:
	.loc	1 1280 32
	vmovss	1768(%rbx), %xmm0
.Ltmp4026:
	.loc	1 1192 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp4027:
	.loc	1 1280 32
	vmovss	1928(%rbx), %xmm0
.Ltmp4028:
	.loc	1 1192 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp4029:
	.loc	1 1280 32
	vmovss	2088(%rbx), %xmm0
.Ltmp4030:
	.loc	1 1192 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp4031:
	.loc	1 1280 32
	vmovss	1624(%rbx), %xmm0
.Ltmp4032:
	.loc	1 1192 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp4033:
	.loc	1 1280 32
	vmovss	1784(%rbx), %xmm0
.Ltmp4034:
	.loc	1 1192 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp4035:
	.loc	1 1280 32
	vmovss	1944(%rbx), %xmm0
.Ltmp4036:
	.loc	1 1192 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp4037:
	.loc	1 1280 32
	vmovss	2104(%rbx), %xmm0
.Ltmp4038:
	.loc	1 1192 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp4039:
	.loc	1 1280 32
	vmovss	1640(%rbx), %xmm0
.Ltmp4040:
	.loc	1 1192 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp4041:
	.loc	1 1280 32
	vmovss	1800(%rbx), %xmm0
.Ltmp4042:
	.loc	1 1192 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp4043:
	.loc	1 1280 32
	vmovss	1960(%rbx), %xmm0
.Ltmp4044:
	.loc	1 1192 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp4045:
	.loc	1 1280 32
	vmovss	2120(%rbx), %xmm0
.Ltmp4046:
	.loc	1 1192 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp4047:
	.loc	1 1280 32
	vmovss	1656(%rbx), %xmm0
.Ltmp4048:
	.loc	1 1192 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp4049:
	.loc	1 1280 32
	vmovss	1816(%rbx), %xmm0
.Ltmp4050:
	.loc	1 1192 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp4051:
	.loc	1 1280 32
	vmovss	1976(%rbx), %xmm0
.Ltmp4052:
	.loc	1 1192 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp4053:
	.loc	1 1280 32
	vmovss	2136(%rbx), %xmm0
.Ltmp4054:
	.loc	1 1192 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp4055:
	.loc	1 1280 32
	vmovss	1672(%rbx), %xmm0
.Ltmp4056:
	.loc	1 1192 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp4057:
	.loc	1 1280 32
	vmovss	1832(%rbx), %xmm0
.Ltmp4058:
	.loc	1 1192 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp4059:
	.loc	1 1280 32
	vmovss	1992(%rbx), %xmm0
.Ltmp4060:
	.loc	1 1192 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp4061:
	.loc	1 1280 32
	vmovss	2152(%rbx), %xmm0
.Ltmp4062:
	.loc	1 1192 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp4063:
	.loc	1 1194 31
	leaq	1328(%rsp), %rdi
	movq	%rbx, %rsi
	movl	1112(%rsp), %ebp
	movl	%ebp, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	1424(%rsp), %rdi
	movq	1104(%rsp), %rsi
	movl	%ebp, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1193 28
	vmovaps	1328(%rsp), %xmm0
	vmovaps	%xmm0, 1200(%rsp)
	vmovaps	1344(%rsp), %xmm0
	vmovaps	%xmm0, 1184(%rsp)
	vmovaps	1360(%rsp), %xmm0
	vmovaps	%xmm0, 1168(%rsp)
	vmovaps	1376(%rsp), %xmm0
	vmovaps	%xmm0, 1152(%rsp)
	vmovaps	1392(%rsp), %xmm0
	vmovaps	%xmm0, 1136(%rsp)
	vmovaps	1408(%rsp), %xmm0
	vmovaps	%xmm0, 1120(%rsp)
	vmovaps	1424(%rsp), %xmm0
	vmovaps	%xmm0, 1296(%rsp)
	vmovaps	1440(%rsp), %xmm0
	vmovaps	%xmm0, 1280(%rsp)
	vmovaps	1456(%rsp), %xmm0
	vmovaps	%xmm0, 1264(%rsp)
	vmovaps	1472(%rsp), %xmm0
	vmovaps	%xmm0, 1248(%rsp)
	vmovaps	1488(%rsp), %xmm0
	vmovaps	%xmm0, 1232(%rsp)
	vmovdqa	1504(%rsp), %xmm0
	vmovdqa	%xmm0, 992(%rsp)
.Ltmp4064:
	.loc	1 0 0 is_stmt 0
	leaq	(%r14,%r15), %rax
	shlq	$2, %r15
	leaq	(,%rax,4), %rsi
	.loc	1 1197 12 is_stmt 1
	testb	$1, %r12b
	movq	%r14, 1216(%rsp)
	movq	%rax, 976(%rsp)
	je	.LBB34_217
.Ltmp4065:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp4066:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_570
.Ltmp4067:
	.loc	48 451 16 is_stmt 1
	cmpq	112(%rsp), %rsi
	ja	.LBB34_570
.Ltmp4068:
	.loc	48 451 16 is_stmt 0
	cmpq	104(%rsp), %rsi
	ja	.LBB34_577
.Ltmp4069:
	.loc	1 1053 27 is_stmt 1
	vmovaps	96(%rbx), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	112(%rbx), %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmovaps	128(%rbx), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	144(%rbx), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4070:
	.loc	1 1054 26
	vmovaps	1424(%rbx), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	1440(%rbx), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	1456(%rbx), %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmovaps	1472(%rbx), %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp4071:
	.loc	1 1055 25
	vmovaps	160(%rbx), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	176(%rbx), %xmm0
	vmovaps	%xmm0, 896(%rsp)
.Ltmp4072:
	.loc	1 1056 24
	vmovaps	1488(%rbx), %xmm0
	vmovaps	%xmm0, 880(%rsp)
	vmovaps	1504(%rbx), %xmm6
.Ltmp4073:
	.loc	1 1057 24
	movq	2680(%rbx), %r12
.Ltmp4074:
	.loc	1 871 17
	movq	1104(%rbx), %rax
	movq	1112(%rbx), %rcx
.Ltmp4075:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	1120(%rbx), %rdx
	xorq	%rax, %rdx
	orq	%rcx, %rdx
	xorq	1128(%rbx), %rax
	orq	%rdx, %rax
	sete	1040(%rsp)
.Ltmp4076:
	.loc	1 871 17
	movq	2432(%rbx), %rax
	movq	2440(%rbx), %rcx
.Ltmp4077:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2448(%rbx), %rdx
	xorq	%rax, %rdx
	xorq	2456(%rbx), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	15(%rsp)
.Ltmp4078:
	.loc	2 1916 50
	testq	%r14, %r14
	movq	1024(%rsp), %rbp
	je	.LBB34_213
.Ltmp4079:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r14,4), %rax
	movq	%rax, 192(%rsp)
	movq	968(%rsp), %rax
	leaq	(%rax,%r15,4), %rcx
	movq	120(%rsp), %rax
	leaq	(%rax,%r15,4), %r9
.Ltmp4080:
	.loc	3 900 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	movq	%r14, 152(%rsp)
	xorl	%edi, %edi
	xorl	%esi, %esi
	movq	%rcx, 928(%rsp)
	movq	%r9, 936(%rsp)
.Ltmp4081:
	.loc	3 0 12 is_stmt 0
.Ltmp4082:
	.p2align	4
.LBB34_164:
	.loc	1 1064 21 is_stmt 1
	vmovaps	208(%rsp), %xmm0
	vmovaps	224(%rsp), %xmm1
	vmovaps	240(%rsp), %xmm2
.Ltmp4083:
	.loc	9 36 14
	vaddps	368(%rsp), %xmm0, %xmm0
.Ltmp4084:
	.loc	1 1066 21
	vmovaps	528(%rsp), %xmm3
	.loc	1 1063 17
	vmovaps	%xmm0, 208(%rsp)
.Ltmp4085:
	.loc	9 36 14
	vaddps	688(%rsp), %xmm3, %xmm0
.Ltmp4086:
	.loc	1 1065 17
	vmovaps	%xmm0, 528(%rsp)
.Ltmp4087:
	.loc	9 36 14
	vaddps	384(%rsp), %xmm1, %xmm0
.Ltmp4088:
	.loc	1 1063 17
	vmovaps	%xmm0, 224(%rsp)
	.loc	1 1066 21
	vmovaps	544(%rsp), %xmm0
.Ltmp4089:
	.loc	9 36 14
	vaddps	704(%rsp), %xmm0, %xmm0
.Ltmp4090:
	.loc	1 1065 17
	vmovaps	%xmm0, 544(%rsp)
.Ltmp4091:
	.loc	9 36 14
	vaddps	400(%rsp), %xmm2, %xmm0
.Ltmp4092:
	.loc	1 1063 17
	vmovaps	%xmm0, 240(%rsp)
	.loc	1 1066 21
	vmovaps	560(%rsp), %xmm0
.Ltmp4093:
	.loc	9 36 14
	vaddps	720(%rsp), %xmm0, %xmm0
.Ltmp4094:
	.loc	1 1065 17
	vmovaps	%xmm0, 560(%rsp)
	.loc	1 1064 21
	vmovaps	256(%rsp), %xmm0
.Ltmp4095:
	.loc	9 36 14
	vaddps	416(%rsp), %xmm0, %xmm0
.Ltmp4096:
	.loc	1 1063 17
	vmovaps	%xmm0, 256(%rsp)
	.loc	1 1066 21
	vmovaps	576(%rsp), %xmm0
.Ltmp4097:
	.loc	9 36 14
	vaddps	736(%rsp), %xmm0, %xmm0
.Ltmp4098:
	.loc	1 1065 17
	vmovaps	%xmm0, 576(%rsp)
	.loc	1 1064 21
	vmovaps	272(%rsp), %xmm0
.Ltmp4099:
	.loc	9 36 14
	vaddps	432(%rsp), %xmm0, %xmm0
.Ltmp4100:
	.loc	1 1063 17
	vmovaps	%xmm0, 272(%rsp)
	.loc	1 1066 21
	vmovaps	592(%rsp), %xmm0
.Ltmp4101:
	.loc	9 36 14
	vaddps	752(%rsp), %xmm0, %xmm0
.Ltmp4102:
	.loc	1 1065 17
	vmovaps	%xmm0, 592(%rsp)
	.loc	1 1064 21
	vmovaps	288(%rsp), %xmm0
.Ltmp4103:
	.loc	9 36 14
	vaddps	448(%rsp), %xmm0, %xmm0
.Ltmp4104:
	.loc	1 1063 17
	vmovaps	%xmm0, 288(%rsp)
	.loc	1 1066 21
	vmovaps	608(%rsp), %xmm0
.Ltmp4105:
	.loc	9 36 14
	vaddps	768(%rsp), %xmm0, %xmm0
.Ltmp4106:
	.loc	1 1065 17
	vmovaps	%xmm0, 608(%rsp)
	.loc	1 1064 21
	vmovaps	304(%rsp), %xmm0
.Ltmp4107:
	.loc	9 36 14
	vaddps	464(%rsp), %xmm0, %xmm0
.Ltmp4108:
	.loc	1 1063 17
	vmovaps	%xmm0, 304(%rsp)
	.loc	1 1066 21
	vmovaps	624(%rsp), %xmm0
.Ltmp4109:
	.loc	9 36 14
	vaddps	784(%rsp), %xmm0, %xmm0
.Ltmp4110:
	.loc	1 1065 17
	vmovaps	%xmm0, 624(%rsp)
	.loc	1 1064 21
	vmovaps	320(%rsp), %xmm0
.Ltmp4111:
	.loc	9 36 14
	vaddps	480(%rsp), %xmm0, %xmm0
.Ltmp4112:
	.loc	1 1063 17
	vmovaps	%xmm0, 320(%rsp)
	.loc	1 1066 21
	vmovaps	640(%rsp), %xmm0
.Ltmp4113:
	.loc	9 36 14
	vaddps	800(%rsp), %xmm0, %xmm0
.Ltmp4114:
	.loc	1 1065 17
	vmovaps	%xmm0, 640(%rsp)
	.loc	1 1064 21
	vmovaps	336(%rsp), %xmm0
.Ltmp4115:
	.loc	9 36 14
	vaddps	496(%rsp), %xmm0, %xmm0
.Ltmp4116:
	.loc	1 1063 17
	vmovaps	%xmm0, 336(%rsp)
	.loc	1 1066 21
	vmovaps	656(%rsp), %xmm0
.Ltmp4117:
	.loc	9 36 14
	vaddps	816(%rsp), %xmm0, %xmm0
.Ltmp4118:
	.loc	1 1065 17
	vmovaps	%xmm0, 656(%rsp)
	.loc	1 1064 21
	vmovaps	352(%rsp), %xmm0
.Ltmp4119:
	.loc	9 36 14
	vaddps	512(%rsp), %xmm0, %xmm0
.Ltmp4120:
	.loc	1 1063 17
	vmovaps	%xmm0, 352(%rsp)
	.loc	1 1066 21
	vmovaps	672(%rsp), %xmm0
.Ltmp4121:
	.loc	9 36 14
	vaddps	832(%rsp), %xmm0, %xmm0
.Ltmp4122:
	.loc	1 1065 17
	vmovaps	%xmm0, 672(%rsp)
.Ltmp4123:
	.loc	1 1070 28
	leaq	1(%r12), %rax
.Ltmp4124:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %edx
	cmovaeq	%rbp, %rdx
.Ltmp4125:
	.loc	48 568 12
	cmpq	192(%rsp), %rdi
	ja	.LBB34_551
.Ltmp4126:
	.loc	48 438 16
	cmpq	%rsi, 152(%rsp)
	je	.LBB34_534
.Ltmp4127:
	.loc	48 0 16 is_stmt 0
	movq	%rsi, 1088(%rsp)
	leaq	(,%r12,4), %rax
.Ltmp4128:
	.loc	1 1083 29 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp4129:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_540
.Ltmp4130:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4131:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm6, 160(%rsp)
	vmovups	(%rcx,%rdi,4), %xmm0
.Ltmp4132:
	vmovups	(%r9,%rdi,4), %xmm2
.Ltmp4133:
	vmovaps	32(%rbx), %xmm5
	vmovaps	48(%rbx), %xmm11
	vmovaps	64(%rbx), %xmm1
	vmovaps	1360(%rbx), %xmm10
	vmovaps	1376(%rbx), %xmm9
	vmovaps	1392(%rbx), %xmm7
	vmovaps	64(%rsp), %xmm12
	vsubps	%xmm12, %xmm0, %xmm3
	vmulps	%xmm3, %xmm11, %xmm4
	vmovaps	80(%rsp), %xmm8
	vmovaps	%xmm5, 944(%rsp)
	vmulps	%xmm5, %xmm8, %xmm5
	vaddps	%xmm4, %xmm5, %xmm5
	vaddps	%xmm5, %xmm8, %xmm6
	vmulps	%xmm11, %xmm8, %xmm4
	vmulps	%xmm1, %xmm3, %xmm3
	vaddps	%xmm3, %xmm4, %xmm4
	vaddps	%xmm4, %xmm12, %xmm3
	vmulps	80(%rbx), %xmm6, %xmm13
	vmovaps	128(%rsp), %xmm8
	vsubps	%xmm8, %xmm3, %xmm3
	vmulps	48(%rsp), %xmm11, %xmm6
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm6, %xmm14
	vaddps	%xmm14, %xmm8, %xmm15
.Ltmp4134:
	vsubps	16(%rsp), %xmm2, %xmm6
	vmulps	%xmm6, %xmm9, %xmm1
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm10, 848(%rsp)
	vmulps	%xmm10, %xmm8, %xmm12
	vaddps	%xmm1, %xmm12, %xmm1
	vaddps	%xmm1, %xmm8, %xmm12
	vmulps	1408(%rbx), %xmm12, %xmm12
.Ltmp4135:
	.loc	1 1083 29 is_stmt 1
	movq	(%rbx), %rcx
.Ltmp4136:
	.loc	8 551 14
	vmovups	%xmm15, (%rcx,%rax,4)
.Ltmp4137:
	.loc	1 1084 30
	movq	24(%rbx), %rsi
.Ltmp4138:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_541
.Ltmp4139:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4140:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm0, %xmm13, %xmm0
	vsubps	%xmm15, %xmm0, %xmm0
.Ltmp4141:
	.loc	1 1084 30 is_stmt 1
	movq	16(%rbx), %rcx
.Ltmp4142:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4143:
	.loc	1 1085 28
	movq	1336(%rbx), %rsi
.Ltmp4144:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_542
.Ltmp4145:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4146:
	.loc	1 0 0 is_stmt 0
	vmulps	32(%rsp), %xmm9, %xmm0
	vmulps	%xmm7, %xmm6, %xmm6
	vaddps	%xmm6, %xmm0, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm6
	vmovaps	176(%rsp), %xmm15
	vsubps	%xmm15, %xmm6, %xmm6
	vmovaps	912(%rsp), %xmm8
	vmulps	%xmm9, %xmm8, %xmm13
	vmulps	%xmm6, %xmm7, %xmm7
	vaddps	%xmm7, %xmm13, %xmm13
	vaddps	%xmm13, %xmm15, %xmm7
.Ltmp4147:
	.loc	1 1085 28 is_stmt 1
	movq	1328(%rbx), %rcx
.Ltmp4148:
	.loc	8 551 14
	vmovups	%xmm7, (%rcx,%rax,4)
.Ltmp4149:
	.loc	1 1086 29
	movq	1352(%rbx), %rsi
.Ltmp4150:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_543
.Ltmp4151:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4152:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm2, %xmm12, %xmm2
	vsubps	%xmm7, %xmm2, %xmm2
.Ltmp4153:
	.loc	1 1086 29 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp4154:
	.loc	8 551 14
	vmovups	%xmm2, (%rcx,%rax,4)
.Ltmp4155:
	.loc	1 1089 13
	movq	(%rbx), %r10
	movq	8(%rbx), %rsi
	movq	1104(%rbx), %rax
.Ltmp4156:
	.loc	1 0 0 is_stmt 0
	addq	%r12, %rax
.Ltmp4157:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp4158:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 1040(%rsp)
	movq	%rdx, 1008(%rsp)
	movq	%rdi, 200(%rsp)
	je	.LBB34_180
.Ltmp4159:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp4160:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4161:
	.loc	1 1096 13
	movq	24(%rbx), %rsi
.Ltmp4162:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp4163:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp4164:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp4165:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4166:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r10,%r9,4), %xmm2
.Ltmp4167:
	movq	16(%rbx), %rcx
.Ltmp4168:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm10
.Ltmp4169:
	.loc	1 961 2
	jmp	.LBB34_189
.Ltmp4170:
	.loc	1 0 2 is_stmt 0
.Ltmp4171:
	.p2align	4
.LBB34_180:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	1112(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4172:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp4173:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_582
	.loc	1 0 25
	movq	1120(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4174:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp4175:
	.loc	1 955 30
	leaq	2(,%rcx,4), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_601
	.loc	1 0 25
	movq	1128(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4176:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp4177:
	.loc	1 955 30
	leaq	3(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
.Ltmp4178:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp4179:
	.loc	1 1096 13
	movq	24(%r13), %rsi
.Ltmp4180:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp4181:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	1112(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4182:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp4183:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_580
	.loc	1 0 25
	movq	%rcx, %rdx
	movq	%r11, %rcx
	movq	%r8, %r11
	movq	%r10, %r8
	movq	1120(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4184:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%rbp, %r10
	movl	$0, %ebp
	cmovaeq	%r10, %rbp
	subq	%rbp, %rbx
.Ltmp4185:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	1128(%r13), %rbx
	movq	%r12, %r14
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4186:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r12d
	cmovaeq	%r10, %r12
	subq	%r12, %rbx
.Ltmp4187:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp4188:
	.loc	1 0 0
	vmovd	(%r8,%r9,4), %xmm2
	vpinsrd	$1, (%r8,%r11,4), %xmm2, %xmm2
	vpinsrd	$2, (%r8,%rcx,4), %xmm2, %xmm2
	vpinsrd	$3, (%r8,%rdx,4), %xmm2, %xmm2
.Ltmp4189:
	movq	16(%r13), %rcx
.Ltmp4190:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm7
	vpinsrd	$1, (%rcx,%r15,4), %xmm7, %xmm7
	vpinsrd	$2, (%rcx,%rbp,4), %xmm7, %xmm7
	vpinsrd	$3, (%rcx,%rbx,4), %xmm7, %xmm10
	movq	%r13, %rbx
	movq	1024(%rsp), %rbp
	movq	200(%rsp), %rdi
	movq	%r14, %r12
	movq	1008(%rsp), %rdx
.Ltmp4191:
.LBB34_189:
	.loc	1 1103 13
	movq	1328(%rbx), %r10
	movq	1336(%rbx), %rsi
	movq	2432(%rbx), %rax
.Ltmp4192:
	.loc	1 0 0 is_stmt 0
	addq	%r12, %rax
.Ltmp4193:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp4194:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 15(%rsp)
	je	.LBB34_195
.Ltmp4195:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp4196:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4197:
	.loc	1 1110 13
	movq	1352(%rbx), %rsi
.Ltmp4198:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp4199:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp4200:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp4201:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4202:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r10,%r9,4), %xmm15
.Ltmp4203:
	movq	1344(%rbx), %rcx
.Ltmp4204:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm7
.Ltmp4205:
	.loc	1 961 2
	jmp	.LBB34_204
.Ltmp4206:
	.loc	1 0 2 is_stmt 0
.Ltmp4207:
	.p2align	4
.LBB34_195:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	2440(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4208:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp4209:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_582
	.loc	1 0 25
	movq	2448(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4210:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp4211:
	.loc	1 955 30
	leaq	2(,%rcx,4), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_601
	.loc	1 0 25
	movq	2456(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4212:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp4213:
	.loc	1 955 30
	leaq	3(,%rcx,4), %rdi
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB34_625
.Ltmp4214:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp4215:
	.loc	1 1110 13
	movq	1352(%r13), %rsi
.Ltmp4216:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp4217:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	2440(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4218:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp4219:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_580
	.loc	1 0 25
	movq	%r11, %rcx
	movq	%r10, %r11
	movq	2448(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4220:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%r12, %r14
	movl	$0, %r12d
	cmovaeq	%rbp, %r12
	subq	%r12, %rbx
	movq	%rbp, %r10
.Ltmp4221:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	2456(%r13), %rbx
	.loc	1 955 35
	addq	%r14, %rbx
.Ltmp4222:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r12d
	cmovaeq	%r10, %r12
	subq	%r12, %rbx
.Ltmp4223:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp4224:
	.loc	1 0 25
	movq	%r8, %rdx
	vmovd	(%r11,%r9,4), %xmm7
	vpinsrd	$1, (%r11,%rdx,4), %xmm7, %xmm7
	vpinsrd	$2, (%r11,%rcx,4), %xmm7, %xmm7
	vpinsrd	$3, (%r11,%rdi,4), %xmm7, %xmm15
.Ltmp4225:
	movq	1344(%r13), %rcx
.Ltmp4226:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm12
	vpinsrd	$1, (%rcx,%r15,4), %xmm12, %xmm12
	vpinsrd	$2, (%rcx,%rbp,4), %xmm12, %xmm12
	vpinsrd	$3, (%rcx,%rbx,4), %xmm12, %xmm7
	movq	%r13, %rbx
	movq	1024(%rsp), %rbp
	movq	200(%rsp), %rdi
	movq	%r14, %r12
	movq	1008(%rsp), %rdx
.Ltmp4227:
.LBB34_204:
	.loc	1 0 0 is_stmt 0
	negq	%rdx
	addq	%rdx, %r12
	incq	%r12
	leaq	(,%r12,4), %rax
.Ltmp4228:
	.loc	1 1150 36 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp4229:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	1216(%rsp), %r14
	movq	936(%rsp), %r9
	jb	.LBB34_546
.Ltmp4230:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4231:
	.loc	1 1152 27
	movq	24(%rbx), %rsi
.Ltmp4232:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_547
.Ltmp4233:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4234:
	.loc	1 1153 35
	movq	1336(%rbx), %rsi
.Ltmp4235:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_548
.Ltmp4236:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4237:
	.loc	1 1155 27
	movq	1352(%rbx), %rsi
.Ltmp4238:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_550
.Ltmp4239:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4240:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm5, %xmm5, %xmm5
	vaddps	80(%rsp), %xmm5, %xmm5
	vmovdqa	%xmm15, 1056(%rsp)
	vbroadcastss	.LCPI34_35(%rip), %xmm15
	vandps	%xmm5, %xmm15, %xmm12
	vmovdqa	%xmm7, 1008(%rsp)
	vbroadcastss	.LCPI34_2(%rip), %xmm7
	vcmpltps	%xmm7, %xmm12, %xmm12
	vandnps	%xmm5, %xmm12, %xmm5
	vmovaps	%xmm5, 80(%rsp)
	vaddps	%xmm4, %xmm4, %xmm4
	vaddps	64(%rsp), %xmm4, %xmm4
	vandps	%xmm4, %xmm15, %xmm5
	vcmpltps	%xmm7, %xmm5, %xmm5
	vandnps	%xmm4, %xmm5, %xmm4
	vmovaps	%xmm4, 64(%rsp)
	vmulps	%xmm3, %xmm11, %xmm3
	vmovaps	48(%rsp), %xmm5
	vmulps	944(%rsp), %xmm5, %xmm4
	vaddps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm3, %xmm3
	vaddps	%xmm3, %xmm5, %xmm3
	vandps	%xmm3, %xmm15, %xmm4
	vcmpltps	%xmm7, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm3
	vmovaps	%xmm3, 48(%rsp)
	vaddps	%xmm14, %xmm14, %xmm3
	vaddps	128(%rsp), %xmm3, %xmm3
	vandps	%xmm3, %xmm15, %xmm4
	vcmpltps	%xmm7, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm3
	vmovaps	%xmm3, 128(%rsp)
.Ltmp4241:
	vaddps	%xmm1, %xmm1, %xmm1
	vaddps	32(%rsp), %xmm1, %xmm1
	vandps	%xmm1, %xmm15, %xmm3
	vcmpltps	%xmm7, %xmm3, %xmm3
	vandnps	%xmm1, %xmm3, %xmm1
	vmovaps	%xmm1, 32(%rsp)
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm15, %xmm1
	vcmpltps	%xmm7, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmulps	%xmm6, %xmm9, %xmm0
	vmulps	848(%rsp), %xmm8, %xmm1
	vaddps	%xmm0, %xmm1, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm8, %xmm0
	vandps	%xmm0, %xmm15, %xmm1
	vcmpltps	%xmm7, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	176(%rsp), %xmm0, %xmm0
.Ltmp4242:
	vpand	%xmm2, %xmm15, %xmm1
	vbroadcastss	.LCPI34_4(%rip), %xmm6
.Ltmp4243:
	vmaxps	%xmm6, %xmm1, %xmm1
	vbroadcastss	.LCPI34_5(%rip), %xmm8
	vmaxps	%xmm8, %xmm1, %xmm1
	vbroadcastss	.LCPI34_36(%rip), %xmm9
	vandps	%xmm1, %xmm9, %xmm2
	vmovdqa	%xmm10, %xmm5
	vbroadcastss	.LCPI34_32(%rip), %xmm10
	vorps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_8(%rip), %xmm11
	vaddps	%xmm2, %xmm11, %xmm2
	vbroadcastss	.LCPI34_9(%rip), %xmm12
	vmulps	%xmm2, %xmm12, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm14
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm9
	vaddps	%xmm3, %xmm9, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm11
	vaddps	%xmm3, %xmm11, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm2
	vpsrld	$23, %xmm1, %xmm1
	vmovdqa	.LCPI34_15(%rip), %xmm3
	vpor	%xmm3, %xmm1, %xmm1
	vbroadcastss	.LCPI34_16(%rip), %xmm3
	vaddps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_19(%rip), %xmm12
	vminps	%xmm12, %xmm1, %xmm1
	vsubps	208(%rsp), %xmm1, %xmm1
	vbroadcastss	.LCPI34_20(%rip), %xmm6
	vaddps	%xmm6, %xmm1, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vbroadcastss	.LCPI34_22(%rip), %xmm3
	vmulps	%xmm3, %xmm2, %xmm2
	vcmpltps	%xmm1, %xmm6, %xmm3
	vblendvps	%xmm3, %xmm1, %xmm2, %xmm2
.Ltmp4244:
	vandps	%xmm0, %xmm15, %xmm3
	vcmpltps	%xmm7, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vbroadcastss	.LCPI34_21(%rip), %xmm8
.Ltmp4245:
	vcmpleps	%xmm8, %xmm1, %xmm0
	vmulps	1200(%rsp), %xmm2, %xmm1
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%xmm0, %xmm2, %xmm0
	vpandn	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm10
	vmaxps	%xmm10, %xmm0, %xmm0
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	864(%rsp), %xmm3
	vcmpltps	%xmm3, %xmm0, %xmm1
	vmovaps	1168(%rsp), %xmm2
	vblendvps	%xmm1, 1184(%rsp), %xmm2, %xmm1
	vsubps	%xmm0, %xmm3, %xmm2
	vmulps	%xmm1, %xmm2, %xmm1
.Ltmp4246:
	vpand	%xmm5, %xmm15, %xmm2
.Ltmp4247:
	vaddps	%xmm1, %xmm0, %xmm0
	vandps	%xmm0, %xmm15, %xmm1
	vcmpltps	%xmm7, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm3
.Ltmp4248:
	vbroadcastss	.LCPI34_4(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm1
	vorps	.LCPI34_7(%rip), %xmm1, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm2
	vaddps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm2, %xmm13, %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm2, %xmm14, %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm2, %xmm9, %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm2, %xmm11, %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm4, %xmm2, %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm12, %xmm0, %xmm0
	vsubps	288(%rsp), %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm1
	vmulps	%xmm1, %xmm1, %xmm1
	vbroadcastss	.LCPI34_22(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vcmpltps	%xmm0, %xmm6, %xmm2
	vblendvps	%xmm2, %xmm0, %xmm1, %xmm1
	vcmpleps	%xmm8, %xmm0, %xmm0
	vmulps	1152(%rsp), %xmm1, %xmm1
	vpxor	%xmm5, %xmm5, %xmm5
	vpcmpgtd	%xmm0, %xmm5, %xmm0
	vpandn	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm3, 864(%rsp)
.Ltmp4249:
	vaddps	272(%rsp), %xmm3, %xmm1
	vbroadcastss	.LCPI34_24(%rip), %xmm15
	vmulps	%xmm1, %xmm15, %xmm1
	vbroadcastss	.LCPI34_25(%rip), %xmm9
	vmaxps	%xmm9, %xmm1, %xmm1
	vbroadcastss	.LCPI34_26(%rip), %xmm11
	vminps	%xmm11, %xmm1, %xmm1
	vroundps	$9, %xmm1, %xmm2
	vsubps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_27(%rip), %xmm12
	vmulps	%xmm1, %xmm12, %xmm3
	vbroadcastss	.LCPI34_28(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_29(%rip), %xmm14
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_30(%rip), %xmm6
	vaddps	%xmm6, %xmm3, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_31(%rip), %xmm7
	vaddps	%xmm7, %xmm3, %xmm3
	vbroadcastss	.LCPI34_23(%rip), %xmm4
.Ltmp4250:
	vmaxps	%xmm4, %xmm0, %xmm0
	vminps	%xmm5, %xmm0, %xmm0
	vmovaps	896(%rsp), %xmm8
	vcmpltps	%xmm8, %xmm0, %xmm4
	vmovaps	1120(%rsp), %xmm5
	vblendvps	%xmm4, 1136(%rsp), %xmm5, %xmm4
.Ltmp4251:
	vmulps	%xmm3, %xmm1, %xmm1
.Ltmp4252:
	vsubps	%xmm0, %xmm8, %xmm3
	vmulps	%xmm4, %xmm3, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm5
	vandps	%xmm5, %xmm0, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm4
	vcmpltps	%xmm4, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_32(%rip), %xmm8
.Ltmp4253:
	vaddps	%xmm1, %xmm8, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm10
	vaddps	%xmm2, %xmm10, %xmm1
	vpslld	$23, %xmm1, %xmm1
	vmovaps	%xmm3, 896(%rsp)
.Ltmp4254:
	vaddps	352(%rsp), %xmm3, %xmm2
.Ltmp4255:
	vmulps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 848(%rsp)
.Ltmp4256:
	vmulps	%xmm2, %xmm15, %xmm1
	vmaxps	%xmm9, %xmm1, %xmm1
	vminps	%xmm11, %xmm1, %xmm2
	vroundps	$9, %xmm2, %xmm1
	vsubps	%xmm1, %xmm2, %xmm2
	vmulps	%xmm2, %xmm12, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm6, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm7, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm2
.Ltmp4257:
	vandps	1056(%rsp), %xmm5, %xmm3
	vmovaps	%xmm5, %xmm9
	vbroadcastss	.LCPI34_4(%rip), %xmm13
.Ltmp4258:
	vmaxps	%xmm13, %xmm3, %xmm3
	vbroadcastss	.LCPI34_5(%rip), %xmm0
	vmaxps	%xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_36(%rip), %xmm0
	vandps	%xmm0, %xmm3, %xmm4
	vbroadcastss	.LCPI34_32(%rip), %xmm0
	vorps	%xmm0, %xmm4, %xmm4
	vbroadcastss	.LCPI34_8(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm4, %xmm11, %xmm5
	vbroadcastss	.LCPI34_10(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_11(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_12(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_13(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_14(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm4
	vpsrld	$23, %xmm3, %xmm3
	vpor	.LCPI34_15(%rip), %xmm3, %xmm3
	vbroadcastss	.LCPI34_16(%rip), %xmm0
	vaddps	%xmm0, %xmm3, %xmm3
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp4259:
	vaddps	%xmm2, %xmm8, %xmm2
	vaddps	%xmm1, %xmm10, %xmm1
	vpslld	$23, %xmm1, %xmm1
.Ltmp4260:
	vbroadcastss	.LCPI34_17(%rip), %xmm0
	vmulps	%xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_18(%rip), %xmm0
	vmaxps	%xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_19(%rip), %xmm0
	vminps	%xmm0, %xmm3, %xmm3
	vsubps	528(%rsp), %xmm3, %xmm3
.Ltmp4261:
	vmulps	%xmm1, %xmm2, %xmm0
	vmovaps	%xmm0, 944(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm0
.Ltmp4262:
	vaddps	%xmm0, %xmm3, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vbroadcastss	.LCPI34_22(%rip), %xmm1
	vmulps	%xmm1, %xmm2, %xmm2
	vcmpltps	%xmm3, %xmm0, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm2, %xmm2
	vbroadcastss	.LCPI34_21(%rip), %xmm6
	vcmpleps	%xmm6, %xmm3, %xmm3
	vmulps	1296(%rsp), %xmm2, %xmm2
	vxorps	%xmm4, %xmm4, %xmm4
	vpcmpgtd	%xmm3, %xmm4, %xmm3
	vpandn	%xmm2, %xmm3, %xmm2
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm2, %xmm2
	vminps	%xmm4, %xmm2, %xmm2
	vmovaps	880(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm2, %xmm3
	vmovaps	1264(%rsp), %xmm4
	vblendvps	%xmm3, 1280(%rsp), %xmm4, %xmm3
	vsubps	%xmm2, %xmm5, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm2, %xmm2
	vandps	%xmm2, %xmm9, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm4
	vcmpltps	%xmm4, %xmm3, %xmm3
	vandnps	%xmm2, %xmm3, %xmm2
	vmovaps	%xmm2, 880(%rsp)
	vaddps	592(%rsp), %xmm2, %xmm2
	vmovaps	%xmm15, %xmm7
	vmulps	%xmm2, %xmm15, %xmm2
	vbroadcastss	.LCPI34_25(%rip), %xmm3
	vmaxps	%xmm3, %xmm2, %xmm2
	vbroadcastss	.LCPI34_26(%rip), %xmm12
	vminps	%xmm12, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm3
	vsubps	%xmm3, %xmm2, %xmm2
	vbroadcastss	.LCPI34_27(%rip), %xmm4
	vmulps	%xmm4, %xmm2, %xmm4
	vbroadcastss	.LCPI34_28(%rip), %xmm13
	vaddps	%xmm4, %xmm13, %xmm4
	vmulps	%xmm4, %xmm2, %xmm4
	vaddps	%xmm4, %xmm14, %xmm4
	vmulps	%xmm4, %xmm2, %xmm4
	vbroadcastss	.LCPI34_30(%rip), %xmm15
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm2, %xmm4
	vbroadcastss	.LCPI34_31(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm2, %xmm2
	vaddps	%xmm2, %xmm8, %xmm2
	vaddps	%xmm3, %xmm10, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm2
.Ltmp4263:
	vandps	1008(%rsp), %xmm9, %xmm3
.Ltmp4264:
	vbroadcastss	.LCPI34_4(%rip), %xmm4
	vmaxps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_5(%rip), %xmm4
	vmaxps	%xmm4, %xmm3, %xmm3
	vandps	.LCPI34_6(%rip), %xmm3, %xmm4
	vorps	.LCPI34_7(%rip), %xmm4, %xmm4
	vbroadcastss	.LCPI34_8(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm11, %xmm5
	vbroadcastss	.LCPI34_10(%rip), %xmm11
	vaddps	%xmm5, %xmm11, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_11(%rip), %xmm11
	vaddps	%xmm5, %xmm11, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_12(%rip), %xmm11
	vaddps	%xmm5, %xmm11, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_13(%rip), %xmm11
	vaddps	%xmm5, %xmm11, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_14(%rip), %xmm11
	vaddps	%xmm5, %xmm11, %xmm5
	vmulps	%xmm5, %xmm4, %xmm4
	vpsrld	$23, %xmm3, %xmm3
	vpor	.LCPI34_15(%rip), %xmm3, %xmm3
	vbroadcastss	.LCPI34_16(%rip), %xmm5
	vaddps	%xmm5, %xmm3, %xmm3
	vaddps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_17(%rip), %xmm4
	vmulps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_18(%rip), %xmm4
	vmaxps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_19(%rip), %xmm4
	vminps	%xmm4, %xmm3, %xmm3
	vsubps	608(%rsp), %xmm3, %xmm3
	vaddps	%xmm0, %xmm3, %xmm4
	vmulps	%xmm4, %xmm4, %xmm4
	vmulps	%xmm1, %xmm4, %xmm4
	vcmpltps	%xmm3, %xmm0, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm4
	vcmpleps	%xmm6, %xmm3, %xmm3
	vmulps	1248(%rsp), %xmm4, %xmm4
	vxorps	%xmm0, %xmm0, %xmm0
	vpcmpgtd	%xmm3, %xmm0, %xmm3
	vpandn	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_23(%rip), %xmm1
	vmaxps	%xmm1, %xmm3, %xmm3
	vminps	%xmm0, %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm6
	vcmpltps	%xmm6, %xmm3, %xmm4
	vmovaps	992(%rsp), %xmm5
	vblendvps	%xmm4, 1232(%rsp), %xmm5, %xmm4
	vsubps	%xmm3, %xmm6, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vandps	%xmm3, %xmm9, %xmm4
	vbroadcastss	.LCPI34_2(%rip), %xmm0
	vcmpltps	%xmm0, %xmm4, %xmm4
	vandnps	%xmm3, %xmm4, %xmm6
	vaddps	672(%rsp), %xmm6, %xmm3
	vmulps	%xmm7, %xmm3, %xmm3
	vbroadcastss	.LCPI34_25(%rip), %xmm0
	vmaxps	%xmm0, %xmm3, %xmm3
	vminps	%xmm12, %xmm3, %xmm3
	vroundps	$9, %xmm3, %xmm4
	vsubps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI34_27(%rip), %xmm0
	vmulps	%xmm0, %xmm3, %xmm5
	vaddps	%xmm5, %xmm13, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm14, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm0
	vaddps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vaddps	%xmm3, %xmm8, %xmm3
	vaddps	%xmm4, %xmm10, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm3, %xmm3
.Ltmp4265:
	movq	(%rbx), %rcx
	vmovaps	848(%rsp), %xmm0
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
	movq	16(%rbx), %rcx
	vmovaps	944(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
.Ltmp4266:
	movq	1328(%rbx), %rcx
	vmulps	(%rcx,%rax,4), %xmm2, %xmm1
	.loc	1 1155 27 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp4267:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm3, %xmm2
.Ltmp4268:
	.loc	9 36 14
	vaddps	%xmm2, %xmm1, %xmm1
	movq	928(%rsp), %rcx
.Ltmp4269:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rdi,4)
.Ltmp4270:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm1, (%r9,%rdi,4)
	movq	1088(%rsp), %rsi
.Ltmp4271:
	.loc	1 0 0
	incq	%rsi
.Ltmp4272:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	cmpq	%rsi, %r14
.Ltmp4273:
	.loc	3 900 12
	jne	.LBB34_164
.Ltmp4274:
.LBB34_213:
	.loc	3 0 12 is_stmt 0
	vmovaps	80(%rsp), %xmm0
	.loc	1 1160 5 is_stmt 1
	vmovaps	%xmm0, 96(%rbx)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 112(%rbx)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 128(%rbx)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 144(%rbx)
	vmovaps	32(%rsp), %xmm0
	.loc	1 1161 5
	vmovaps	%xmm0, 1424(%rbx)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%rbx)
	vmovaps	912(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%rbx)
	vmovaps	176(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%rbx)
	vmovaps	864(%rsp), %xmm0
	.loc	1 1162 5
	vmovaps	%xmm0, 160(%rbx)
	vmovaps	896(%rsp), %xmm0
	vmovaps	%xmm0, 176(%rbx)
	vmovaps	880(%rsp), %xmm0
	.loc	1 1163 5
	vmovaps	%xmm0, 1488(%rbx)
	vmovaps	%xmm6, 1504(%rbx)
	.loc	1 1164 5
	movq	%r12, 2680(%rbx)
	xorl	%eax, %eax
	xorl	%edi, %edi
.Ltmp4275:
	.loc	1 0 5 is_stmt 0
.Ltmp4276:
	.p2align	4
.LBB34_214:
	.loc	1 1297 13 is_stmt 1
	vmovd	208(%rsp,%rax), %xmm0
	vmovss	212(%rsp,%rax), %xmm1
	vmovss	216(%rsp,%rax), %xmm2
	vmovss	220(%rsp,%rax), %xmm3
.Ltmp4277:
	.loc	1 1300 17
	vmovd	%xmm0, 192(%rbx,%rax)
	.loc	1 1301 34
	movl	204(%rbx,%rax), %ecx
	movl	364(%rbx,%rax), %edx
.Ltmp4278:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp4279:
	.loc	1 1301 17
	movl	%ecx, 204(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 352(%rbx,%rax)
.Ltmp4280:
	.loc	38 2472 13
	subl	%r14d, %edx
	cmovbl	%edi, %edx
.Ltmp4281:
	.loc	1 1301 17
	movl	%edx, 364(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 512(%rbx,%rax)
	.loc	1 1301 34
	movl	524(%rbx,%rax), %ecx
.Ltmp4282:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp4283:
	.loc	1 1301 17
	movl	%ecx, 524(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm3, 672(%rbx,%rax)
	.loc	1 1301 34
	movl	684(%rbx,%rax), %ecx
.Ltmp4284:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp4285:
	.loc	1 1301 17
	movl	%ecx, 684(%rbx,%rax)
.Ltmp4286:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp4287:
	.loc	3 900 12
	jne	.LBB34_214
.Ltmp4288:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	984(%rsp), %rsi
	movq	120(%rsp), %r9
	.p2align	4
.LBB34_216:
.Ltmp4289:
	.loc	1 1297 13 is_stmt 1
	vmovd	528(%rsp,%rax), %xmm0
	vmovss	532(%rsp,%rax), %xmm1
	vmovss	536(%rsp,%rax), %xmm2
	vmovss	540(%rsp,%rax), %xmm3
.Ltmp4290:
	.loc	1 1300 17
	vmovd	%xmm0, 1520(%rbx,%rax)
	.loc	1 1301 34
	movl	1532(%rbx,%rax), %ecx
	movl	1692(%rbx,%rax), %edx
.Ltmp4291:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp4292:
	.loc	1 1301 17
	movl	%ecx, 1532(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 1680(%rbx,%rax)
.Ltmp4293:
	.loc	38 2472 13
	subl	%r14d, %edx
	cmovbl	%edi, %edx
.Ltmp4294:
	.loc	1 1301 17
	movl	%edx, 1692(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 1840(%rbx,%rax)
	.loc	1 1301 34
	movl	1852(%rbx,%rax), %ecx
.Ltmp4295:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp4296:
	.loc	1 1301 17
	movl	%ecx, 1852(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm3, 2000(%rbx,%rax)
	.loc	1 1301 34
	movl	2012(%rbx,%rax), %ecx
.Ltmp4297:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp4298:
	.loc	1 1301 17
	movl	%ecx, 2012(%rbx,%rax)
.Ltmp4299:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp4300:
	.loc	3 900 12
	jne	.LBB34_216
	jmp	.LBB34_157
.Ltmp4301:
.LBB34_217:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp4302:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_571
.Ltmp4303:
	.loc	48 451 16 is_stmt 1
	cmpq	112(%rsp), %rsi
	ja	.LBB34_571
.Ltmp4304:
	.loc	48 451 16 is_stmt 0
	cmpq	104(%rsp), %rsi
	ja	.LBB34_576
.Ltmp4305:
	.loc	1 1053 27 is_stmt 1
	vmovaps	96(%rbx), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	112(%rbx), %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmovaps	128(%rbx), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	144(%rbx), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4306:
	.loc	1 1054 26
	vmovaps	1424(%rbx), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	1440(%rbx), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	1456(%rbx), %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmovaps	1472(%rbx), %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp4307:
	.loc	1 1055 25
	vmovaps	160(%rbx), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	176(%rbx), %xmm0
	vmovaps	%xmm0, 896(%rsp)
.Ltmp4308:
	.loc	1 1056 24
	vmovaps	1488(%rbx), %xmm0
	vmovaps	%xmm0, 880(%rsp)
	vmovaps	1504(%rbx), %xmm6
.Ltmp4309:
	.loc	1 1057 24
	movq	2680(%rbx), %r10
.Ltmp4310:
	.loc	1 871 17
	movq	1104(%rbx), %rax
	movq	1112(%rbx), %rcx
.Ltmp4311:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	1120(%rbx), %rdx
	xorq	%rax, %rdx
	orq	%rcx, %rdx
	xorq	1128(%rbx), %rax
	orq	%rdx, %rax
	sete	1040(%rsp)
.Ltmp4312:
	.loc	1 871 17
	movq	2432(%rbx), %rax
	movq	2440(%rbx), %rcx
.Ltmp4313:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2448(%rbx), %rdx
	xorq	%rax, %rdx
	xorq	2456(%rbx), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	15(%rsp)
.Ltmp4314:
	.loc	2 1916 50
	testq	%r14, %r14
	movq	1024(%rsp), %rbp
	je	.LBB34_156
.Ltmp4315:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r14,4), %rax
	movq	%rax, 192(%rsp)
	movq	968(%rsp), %rax
	leaq	(%rax,%r15,4), %rcx
	movq	120(%rsp), %rax
	leaq	(%rax,%r15,4), %r9
.Ltmp4316:
	.loc	48 568 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	movq	%r14, 152(%rsp)
	xorl	%edi, %edi
	xorl	%r11d, %r11d
	movq	%rcx, 928(%rsp)
	movq	%r9, 936(%rsp)
.Ltmp4317:
	.loc	48 0 12 is_stmt 0
.Ltmp4318:
	.p2align	4
.LBB34_222:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp4319:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %edx
	cmovaeq	%rbp, %rdx
.Ltmp4320:
	.loc	48 568 12
	cmpq	192(%rsp), %rdi
	ja	.LBB34_551
.Ltmp4321:
	.loc	48 438 16
	cmpq	%r11, 152(%rsp)
	je	.LBB34_534
.Ltmp4322:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,4), %rax
.Ltmp4323:
	.loc	1 1083 29 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp4324:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_540
.Ltmp4325:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4326:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm6, 1088(%rsp)
	vmovups	(%rcx,%rdi,4), %xmm4
.Ltmp4327:
	vmovups	(%r9,%rdi,4), %xmm14
.Ltmp4328:
	vmovaps	32(%rbx), %xmm5
	vmovaps	48(%rbx), %xmm11
	vmovaps	64(%rbx), %xmm2
	vmovaps	1360(%rbx), %xmm15
	vmovaps	1376(%rbx), %xmm9
	vmovaps	1392(%rbx), %xmm1
	vmovaps	64(%rsp), %xmm7
	vsubps	%xmm7, %xmm4, %xmm0
	vmulps	%xmm0, %xmm11, %xmm3
	vmovaps	80(%rsp), %xmm6
	vmovaps	%xmm5, 848(%rsp)
	vmulps	%xmm5, %xmm6, %xmm5
	vaddps	%xmm3, %xmm5, %xmm13
	vaddps	%xmm6, %xmm13, %xmm3
	vmulps	%xmm6, %xmm11, %xmm5
	vmulps	%xmm2, %xmm0, %xmm0
	vaddps	%xmm0, %xmm5, %xmm12
	vaddps	%xmm7, %xmm12, %xmm5
	vmulps	80(%rbx), %xmm3, %xmm0
	vmovaps	128(%rsp), %xmm6
	vsubps	%xmm6, %xmm5, %xmm7
	vmulps	48(%rsp), %xmm11, %xmm3
	vmulps	%xmm7, %xmm2, %xmm2
	vaddps	%xmm2, %xmm3, %xmm2
	vaddps	%xmm2, %xmm6, %xmm3
.Ltmp4329:
	vsubps	16(%rsp), %xmm14, %xmm6
	vmulps	%xmm6, %xmm9, %xmm5
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm15, 160(%rsp)
	vmulps	%xmm15, %xmm8, %xmm15
	vaddps	%xmm5, %xmm15, %xmm10
	vaddps	%xmm10, %xmm8, %xmm15
	vmulps	1408(%rbx), %xmm15, %xmm15
.Ltmp4330:
	.loc	1 1083 29 is_stmt 1
	movq	(%rbx), %rcx
.Ltmp4331:
	.loc	8 551 14
	vmovups	%xmm3, (%rcx,%rax,4)
.Ltmp4332:
	.loc	1 1084 30
	movq	24(%rbx), %rsi
.Ltmp4333:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_541
.Ltmp4334:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4335:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm0, %xmm4, %xmm0
	vsubps	%xmm3, %xmm0, %xmm0
.Ltmp4336:
	.loc	1 1084 30 is_stmt 1
	movq	16(%rbx), %rcx
.Ltmp4337:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4338:
	.loc	1 1085 28
	movq	1336(%rbx), %rsi
.Ltmp4339:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_542
.Ltmp4340:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4341:
	.loc	1 0 0 is_stmt 0
	vmulps	32(%rsp), %xmm9, %xmm0
	vmulps	%xmm1, %xmm6, %xmm3
	vaddps	%xmm3, %xmm0, %xmm6
	vaddps	16(%rsp), %xmm6, %xmm0
	vmovaps	176(%rsp), %xmm3
	vsubps	%xmm3, %xmm0, %xmm4
	vmovaps	912(%rsp), %xmm8
	vmulps	%xmm9, %xmm8, %xmm0
	vmulps	%xmm4, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm1
	vaddps	%xmm1, %xmm3, %xmm0
.Ltmp4342:
	.loc	1 1085 28 is_stmt 1
	movq	1328(%rbx), %rcx
.Ltmp4343:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4344:
	.loc	1 1086 29
	movq	1352(%rbx), %rsi
.Ltmp4345:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_543
.Ltmp4346:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4347:
	.loc	48 0 16 is_stmt 0
	movq	%rdi, 1008(%rsp)
	vaddps	%xmm15, %xmm14, %xmm3
	vsubps	%xmm0, %xmm3, %xmm0
.Ltmp4348:
	.loc	1 1086 29 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp4349:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4350:
	.loc	1 1089 13
	movq	(%rbx), %rdi
	movq	8(%rbx), %rsi
	movq	1104(%rbx), %rax
.Ltmp4351:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp4352:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp4353:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 1040(%rsp)
	movq	%r11, 200(%rsp)
	je	.LBB34_238
.Ltmp4354:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp4355:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4356:
	.loc	1 1096 13
	movq	24(%rbx), %rsi
.Ltmp4357:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp4358:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp4359:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp4360:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4361:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%rdi,%r9,4), %xmm15
.Ltmp4362:
	movq	16(%rbx), %rcx
.Ltmp4363:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm3
.Ltmp4364:
	.loc	1 961 2
	jmp	.LBB34_247
.Ltmp4365:
	.loc	1 0 2 is_stmt 0
.Ltmp4366:
	.p2align	4
.LBB34_238:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	1112(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp4367:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp4368:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_582
	.loc	1 0 25
	movq	1120(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp4369:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp4370:
	.loc	1 955 30
	leaq	2(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
	.loc	1 0 25
	movq	1128(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp4371:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp4372:
	.loc	1 955 30
	leaq	3(,%rbx,4), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_609
.Ltmp4373:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp4374:
	.loc	1 1096 13
	movq	24(%r13), %rsi
.Ltmp4375:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp4376:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	1112(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp4377:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp4378:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_580
	.loc	1 0 25
	movq	1120(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp4379:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%r10, %r11
	movq	%rbp, %r10
	movl	$0, %ebp
	cmovaeq	%r10, %rbp
	subq	%rbp, %rbx
.Ltmp4380:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	1128(%r13), %rbx
	movq	%r11, %r14
	.loc	1 955 35
	addq	%r11, %rbx
.Ltmp4381:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r11d
	cmovaeq	%r10, %r11
	subq	%r11, %rbx
.Ltmp4382:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp4383:
	.loc	1 0 0
	vmovd	(%rdi,%r9,4), %xmm0
	vpinsrd	$1, (%rdi,%r8,4), %xmm0, %xmm0
	vpinsrd	$2, (%rdi,%rcx,4), %xmm0, %xmm0
	vpinsrd	$3, (%rdi,%r12,4), %xmm0, %xmm15
.Ltmp4384:
	movq	16(%r13), %rcx
.Ltmp4385:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm3
	movq	%r13, %rbx
	movq	1024(%rsp), %rbp
	movq	%r14, %r10
.Ltmp4386:
.LBB34_247:
	.loc	1 1103 13
	movq	1328(%rbx), %r14
	movq	1336(%rbx), %rsi
	movq	2432(%rbx), %rax
.Ltmp4387:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp4388:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp4389:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 15(%rsp)
	je	.LBB34_253
.Ltmp4390:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp4391:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4392:
	.loc	1 1110 13
	movq	1352(%rbx), %rsi
.Ltmp4393:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp4394:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp4395:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp4396:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4397:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r14,%r9,4), %xmm5
.Ltmp4398:
	movq	1344(%rbx), %rcx
.Ltmp4399:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm0
.Ltmp4400:
	.loc	1 961 2
	jmp	.LBB34_262
.Ltmp4401:
	.loc	1 0 2 is_stmt 0
.Ltmp4402:
	.p2align	4
.LBB34_253:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	2440(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp4403:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp4404:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_582
	.loc	1 0 25
	movq	2448(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp4405:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp4406:
	.loc	1 955 30
	leaq	2(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
	.loc	1 0 25
	movq	2456(%rbx), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp4407:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movq	%rbx, %r15
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
.Ltmp4408:
	.loc	1 955 30
	leaq	3(,%r11,4), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_609
.Ltmp4409:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
.Ltmp4410:
	.loc	1 1110 13
	movq	1352(%r15), %rsi
.Ltmp4411:
	.loc	1 857 8
	subq	%r11, %rax
.Ltmp4412:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	2440(%r15), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp4413:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movq	%r15, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
.Ltmp4414:
	.loc	1 955 30
	leaq	1(,%r11,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_580
	.loc	1 0 25
	movq	2448(%r13), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp4415:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
	movq	%r10, %rbx
	movq	%rbp, %r10
.Ltmp4416:
	.loc	1 955 30
	leaq	2(,%r11,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	2456(%r13), %r11
	movq	%rbx, %rdi
	.loc	1 955 35
	addq	%rbx, %r11
.Ltmp4417:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %r11
	movl	$0, %ebx
	cmovaeq	%r10, %rbx
	subq	%rbx, %r11
.Ltmp4418:
	.loc	1 955 30
	leaq	3(,%r11,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp4419:
	.loc	1 0 0
	vmovd	(%r14,%r9,4), %xmm0
	vpinsrd	$1, (%r14,%r8,4), %xmm0, %xmm0
	vpinsrd	$2, (%r14,%rcx,4), %xmm0, %xmm0
	vpinsrd	$3, (%r14,%r12,4), %xmm0, %xmm5
.Ltmp4420:
	movq	1344(%r13), %rcx
.Ltmp4421:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm0
	movq	%r13, %rbx
	movq	1024(%rsp), %rbp
	movq	%rdi, %r10
.Ltmp4422:
.LBB34_262:
	.loc	1 0 0 is_stmt 0
	negq	%rdx
	addq	%rdx, %r10
	incq	%r10
	leaq	(,%r10,4), %rax
.Ltmp4423:
	.loc	1 1150 36 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp4424:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	1216(%rsp), %rdx
	movq	936(%rsp), %r9
	movq	1008(%rsp), %rdi
	movq	200(%rsp), %r11
	jb	.LBB34_546
.Ltmp4425:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4426:
	.loc	1 1152 27
	movq	24(%rbx), %rsi
.Ltmp4427:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_547
.Ltmp4428:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4429:
	.loc	1 1153 35
	movq	1336(%rbx), %rsi
.Ltmp4430:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_548
.Ltmp4431:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4432:
	.loc	1 1155 27
	movq	1352(%rbx), %rsi
.Ltmp4433:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_550
.Ltmp4434:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4435:
	.loc	48 0 16 is_stmt 0
	vmovdqa	%xmm0, 944(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	80(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm13
	vmovdqa	%xmm5, 1056(%rsp)
	vmovdqa	%xmm3, %xmm5
	vandps	%xmm0, %xmm13, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm14
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vaddps	%xmm12, %xmm12, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm13, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 64(%rsp)
	vmulps	%xmm7, %xmm11, %xmm0
	vmovaps	48(%rsp), %xmm7
	vmulps	848(%rsp), %xmm7, %xmm3
	vaddps	%xmm0, %xmm3, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm7, %xmm0
	vandps	%xmm0, %xmm13, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm2, %xmm2, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm13, %xmm2
	vcmpltps	%xmm14, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4436:
	vaddps	%xmm10, %xmm10, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm13, %xmm2
	vcmpltps	%xmm14, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm6, %xmm6, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm13, %xmm2
	vcmpltps	%xmm14, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmulps	%xmm4, %xmm9, %xmm0
	vmulps	160(%rsp), %xmm8, %xmm2
	vaddps	%xmm0, %xmm2, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm8, %xmm0
	vandps	%xmm0, %xmm13, %xmm2
	vcmpltps	%xmm14, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vaddps	%xmm1, %xmm1, %xmm0
	vaddps	176(%rsp), %xmm0, %xmm1
.Ltmp4437:
	vpand	%xmm13, %xmm15, %xmm0
	vbroadcastss	.LCPI34_4(%rip), %xmm6
.Ltmp4438:
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm7
	vmaxps	%xmm7, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm8
	vandps	%xmm0, %xmm8, %xmm2
	vbroadcastss	.LCPI34_32(%rip), %xmm9
	vorps	%xmm2, %xmm9, %xmm2
	vbroadcastss	.LCPI34_8(%rip), %xmm10
	vaddps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm2, %xmm11, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm12
	vaddps	%xmm3, %xmm12, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm8
	vaddps	%xmm3, %xmm8, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm10
	vaddps	%xmm3, %xmm10, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm2
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm3
	vpor	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm11
	vminps	%xmm11, %xmm0, %xmm0
	vsubps	208(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm6
	vaddps	%xmm6, %xmm0, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vbroadcastss	.LCPI34_22(%rip), %xmm3
	vmulps	%xmm3, %xmm2, %xmm2
	vcmpltps	%xmm0, %xmm6, %xmm3
	vblendvps	%xmm3, %xmm0, %xmm2, %xmm2
.Ltmp4439:
	vandps	%xmm1, %xmm13, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm1, %xmm3, %xmm1
	vmovaps	%xmm1, 176(%rsp)
	vbroadcastss	.LCPI34_21(%rip), %xmm7
.Ltmp4440:
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1200(%rsp), %xmm2, %xmm1
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%xmm0, %xmm2, %xmm0
	vpandn	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm9
	vmaxps	%xmm9, %xmm0, %xmm0
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	864(%rsp), %xmm3
	vcmpltps	%xmm3, %xmm0, %xmm1
	vmovaps	1168(%rsp), %xmm2
	vblendvps	%xmm1, 1184(%rsp), %xmm2, %xmm1
	vsubps	%xmm0, %xmm3, %xmm2
	vmulps	%xmm1, %xmm2, %xmm1
.Ltmp4441:
	vpand	%xmm5, %xmm13, %xmm2
.Ltmp4442:
	vaddps	%xmm1, %xmm0, %xmm0
	vandps	%xmm0, %xmm13, %xmm1
	vcmpltps	%xmm14, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm3
.Ltmp4443:
	vbroadcastss	.LCPI34_4(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm1
	vorps	.LCPI34_7(%rip), %xmm1, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm2
	vaddps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm2, %xmm12, %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm2, %xmm15, %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm2, %xmm8, %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm2, %xmm10, %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
	vaddps	%xmm4, %xmm2, %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vsubps	288(%rsp), %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm1
	vmulps	%xmm1, %xmm1, %xmm1
	vbroadcastss	.LCPI34_22(%rip), %xmm4
	vmulps	%xmm4, %xmm1, %xmm1
	vcmpltps	%xmm0, %xmm6, %xmm2
	vblendvps	%xmm2, %xmm0, %xmm1, %xmm1
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1152(%rsp), %xmm1, %xmm1
	vpxor	%xmm5, %xmm5, %xmm5
	vpcmpgtd	%xmm0, %xmm5, %xmm0
	vpandn	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm3, 864(%rsp)
.Ltmp4444:
	vaddps	272(%rsp), %xmm3, %xmm1
	vbroadcastss	.LCPI34_24(%rip), %xmm14
	vmulps	%xmm1, %xmm14, %xmm1
	vbroadcastss	.LCPI34_25(%rip), %xmm13
	vmaxps	%xmm13, %xmm1, %xmm1
	vbroadcastss	.LCPI34_26(%rip), %xmm10
	vminps	%xmm10, %xmm1, %xmm1
	vroundps	$9, %xmm1, %xmm2
	vsubps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI34_27(%rip), %xmm11
	vmulps	%xmm1, %xmm11, %xmm3
	vbroadcastss	.LCPI34_28(%rip), %xmm12
	vaddps	%xmm3, %xmm12, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_29(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_30(%rip), %xmm6
	vaddps	%xmm6, %xmm3, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_31(%rip), %xmm8
	vaddps	%xmm3, %xmm8, %xmm3
	vbroadcastss	.LCPI34_23(%rip), %xmm4
.Ltmp4445:
	vmaxps	%xmm4, %xmm0, %xmm0
	vminps	%xmm5, %xmm0, %xmm0
	vmovaps	896(%rsp), %xmm7
	vcmpltps	%xmm7, %xmm0, %xmm4
	vmovaps	1120(%rsp), %xmm5
	vblendvps	%xmm4, 1136(%rsp), %xmm5, %xmm4
.Ltmp4446:
	vmulps	%xmm3, %xmm1, %xmm1
.Ltmp4447:
	vsubps	%xmm0, %xmm7, %xmm3
	vmulps	%xmm4, %xmm3, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm5
	vandps	%xmm5, %xmm0, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm4
	vcmpltps	%xmm4, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_32(%rip), %xmm7
.Ltmp4448:
	vaddps	%xmm7, %xmm1, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm9
	vaddps	%xmm2, %xmm9, %xmm1
	vpslld	$23, %xmm1, %xmm1
	vmovaps	%xmm3, 896(%rsp)
.Ltmp4449:
	vaddps	352(%rsp), %xmm3, %xmm2
.Ltmp4450:
	vmulps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
.Ltmp4451:
	vmulps	%xmm2, %xmm14, %xmm0
	vmaxps	%xmm13, %xmm0, %xmm0
	vminps	%xmm10, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm2
	vsubps	%xmm2, %xmm0, %xmm0
	vmulps	%xmm0, %xmm11, %xmm3
	vaddps	%xmm3, %xmm12, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vaddps	%xmm6, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm3
	vaddps	%xmm3, %xmm8, %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
.Ltmp4452:
	vandps	1056(%rsp), %xmm5, %xmm3
	vmovaps	%xmm5, %xmm12
	vbroadcastss	.LCPI34_4(%rip), %xmm1
.Ltmp4453:
	vmaxps	%xmm1, %xmm3, %xmm3
	vbroadcastss	.LCPI34_5(%rip), %xmm1
	vmaxps	%xmm1, %xmm3, %xmm3
	vbroadcastss	.LCPI34_36(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm4
	vbroadcastss	.LCPI34_32(%rip), %xmm15
	vorps	%xmm4, %xmm15, %xmm4
	vbroadcastss	.LCPI34_8(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vbroadcastss	.LCPI34_9(%rip), %xmm1
	vmulps	%xmm1, %xmm4, %xmm5
	vbroadcastss	.LCPI34_10(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_11(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_12(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_13(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI34_14(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm4, %xmm4
	vpsrld	$23, %xmm3, %xmm3
	vpor	.LCPI34_15(%rip), %xmm3, %xmm3
	vbroadcastss	.LCPI34_16(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp4454:
	vaddps	%xmm7, %xmm0, %xmm0
	vaddps	%xmm2, %xmm9, %xmm2
	vpslld	$23, %xmm2, %xmm2
.Ltmp4455:
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm3, %xmm3
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm3, %xmm3
	vbroadcastss	.LCPI34_19(%rip), %xmm1
	vminps	%xmm1, %xmm3, %xmm3
	vsubps	528(%rsp), %xmm3, %xmm3
.Ltmp4456:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm2
.Ltmp4457:
	vaddps	%xmm2, %xmm3, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm6
	vmulps	%xmm6, %xmm0, %xmm0
	vcmpltps	%xmm3, %xmm2, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm1
	vcmpleps	%xmm1, %xmm3, %xmm3
	vmulps	1296(%rsp), %xmm0, %xmm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm3, %xmm1, %xmm3
	vpandn	%xmm0, %xmm3, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	880(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm0, %xmm3
	vmovaps	1264(%rsp), %xmm4
	vblendvps	%xmm3, 1280(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm5, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm0, 880(%rsp)
	vaddps	592(%rsp), %xmm0, %xmm0
	vmovaps	%xmm14, %xmm1
	vmulps	%xmm0, %xmm14, %xmm0
	vmovaps	%xmm13, %xmm8
	vmaxps	%xmm13, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm11
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_28(%rip), %xmm13
	vaddps	%xmm4, %xmm13, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_29(%rip), %xmm14
	vaddps	%xmm4, %xmm14, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_30(%rip), %xmm15
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_31(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
	vaddps	%xmm7, %xmm0, %xmm0
	vaddps	%xmm3, %xmm9, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm4
.Ltmp4458:
	vandps	944(%rsp), %xmm12, %xmm0
.Ltmp4459:
	vbroadcastss	.LCPI34_4(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm3
	vorps	.LCPI34_7(%rip), %xmm3, %xmm3
	vbroadcastss	.LCPI34_8(%rip), %xmm5
	vaddps	%xmm5, %xmm3, %xmm3
	vbroadcastss	.LCPI34_9(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vaddps	%xmm5, %xmm10, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_11(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_12(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_13(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vmulps	%xmm5, %xmm3, %xmm5
	vbroadcastss	.LCPI34_14(%rip), %xmm10
	vaddps	%xmm5, %xmm10, %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm3
	vminps	%xmm3, %xmm0, %xmm0
	vsubps	608(%rsp), %xmm0, %xmm0
	vaddps	%xmm2, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm6, %xmm3, %xmm3
	vcmpltps	%xmm0, %xmm2, %xmm5
	vblendvps	%xmm5, %xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm2
	vcmpleps	%xmm2, %xmm0, %xmm0
	vmulps	1248(%rsp), %xmm3, %xmm3
	vxorps	%xmm2, %xmm2, %xmm2
	vpcmpgtd	%xmm0, %xmm2, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	1088(%rsp), %xmm6
	vcmpltps	%xmm6, %xmm0, %xmm3
	vmovaps	992(%rsp), %xmm5
	vblendvps	%xmm3, 1232(%rsp), %xmm5, %xmm3
	vsubps	%xmm0, %xmm6, %xmm5
	vmulps	%xmm3, %xmm5, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm2
	vcmpltps	%xmm2, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm6
	vaddps	672(%rsp), %xmm6, %xmm0
	vmulps	%xmm1, %xmm0, %xmm0
	vmaxps	%xmm8, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm5
	vaddps	%xmm5, %xmm13, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm14, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm7, %xmm0, %xmm0
	vaddps	%xmm3, %xmm9, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
.Ltmp4460:
	movq	(%rbx), %rcx
	vmovaps	160(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm1
	movq	16(%rbx), %rcx
	vmovaps	848(%rsp), %xmm2
	vmulps	(%rcx,%rax,4), %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm1
.Ltmp4461:
	movq	1328(%rbx), %rcx
	vmulps	(%rcx,%rax,4), %xmm4, %xmm2
	.loc	1 1155 27 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp4462:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
.Ltmp4463:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	928(%rsp), %rcx
.Ltmp4464:
	.loc	8 551 14
	vmovups	%xmm1, (%rcx,%rdi,4)
.Ltmp4465:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r9,%rdi,4)
.Ltmp4466:
	.loc	1 0 0
	incq	%r11
.Ltmp4467:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	cmpq	%r11, %rdx
.Ltmp4468:
	.loc	3 900 12
	jne	.LBB34_222
	jmp	.LBB34_156
.Ltmp4469:
.LBB34_271:
	.loc	3 0 12 is_stmt 0
	vmovaps	32(%rsp), %xmm0
.Ltmp4470:
	.loc	1 1160 5 is_stmt 1
	vmovaps	%xmm0, 96(%rbx)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 112(%rbx)
	vmovaps	80(%rsp), %xmm0
	vmovaps	%xmm0, 128(%rbx)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 144(%rbx)
	vmovaps	992(%rsp), %xmm0
	.loc	1 1161 5
	vmovaps	%xmm0, 1424(%rbx)
	vmovaps	912(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%rbx)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%rbx)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%rbx)
	vmovaps	864(%rsp), %xmm0
	.loc	1 1162 5
	vmovaps	%xmm0, 160(%rbx)
	vmovaps	880(%rsp), %xmm0
	vmovaps	%xmm0, 176(%rbx)
	vmovdqa	896(%rsp), %xmm0
	.loc	1 1163 5
	vmovdqa	%xmm0, 1488(%rbx)
	vmovaps	%xmm7, 1504(%rbx)
	.loc	1 1164 5
	movq	%r10, 2680(%rbx)
	movq	984(%rsp), %rsi
	movq	120(%rsp), %r9
.Ltmp4471:
.LBB34_272:
	.loc	1 0 5 is_stmt 0
	movq	1112(%rsp), %r15
	.loc	1 1189 11 is_stmt 1
	cmpq	%rsi, %r15
	jae	.LBB34_503
.LBB34_273:
	.loc	1 1190 42
	subq	%r15, %rsi
	.loc	1 1190 29 is_stmt 0
	movq	%rbx, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r14
.Ltmp4472:
	.loc	1 1279 33 is_stmt 1
	vmovss	192(%rbx), %xmm0
.Ltmp4473:
	.loc	1 1192 28
	vmovss	%xmm0, 208(%rsp)
.Ltmp4474:
	.loc	1 1279 33
	vmovss	352(%rbx), %xmm0
.Ltmp4475:
	.loc	1 1192 28
	vmovss	%xmm0, 212(%rsp)
.Ltmp4476:
	.loc	1 1279 33
	vmovss	512(%rbx), %xmm0
.Ltmp4477:
	.loc	1 1192 28
	vmovss	%xmm0, 216(%rsp)
.Ltmp4478:
	.loc	1 1279 33
	vmovss	672(%rbx), %xmm0
.Ltmp4479:
	.loc	1 1192 28
	vmovss	%xmm0, 220(%rsp)
.Ltmp4480:
	.loc	1 1279 33
	vmovss	208(%rbx), %xmm0
.Ltmp4481:
	.loc	1 1192 28
	vmovss	%xmm0, 224(%rsp)
.Ltmp4482:
	.loc	1 1279 33
	vmovss	368(%rbx), %xmm0
.Ltmp4483:
	.loc	1 1192 28
	vmovss	%xmm0, 228(%rsp)
.Ltmp4484:
	.loc	1 1279 33
	vmovss	528(%rbx), %xmm0
.Ltmp4485:
	.loc	1 1192 28
	vmovss	%xmm0, 232(%rsp)
.Ltmp4486:
	.loc	1 1279 33
	vmovss	688(%rbx), %xmm0
.Ltmp4487:
	.loc	1 1192 28
	vmovss	%xmm0, 236(%rsp)
.Ltmp4488:
	.loc	1 1279 33
	vmovss	224(%rbx), %xmm0
.Ltmp4489:
	.loc	1 1192 28
	vmovss	%xmm0, 240(%rsp)
.Ltmp4490:
	.loc	1 1279 33
	vmovss	384(%rbx), %xmm0
.Ltmp4491:
	.loc	1 1192 28
	vmovss	%xmm0, 244(%rsp)
.Ltmp4492:
	.loc	1 1279 33
	vmovss	544(%rbx), %xmm0
.Ltmp4493:
	.loc	1 1192 28
	vmovss	%xmm0, 248(%rsp)
.Ltmp4494:
	.loc	1 1279 33
	vmovss	704(%rbx), %xmm0
.Ltmp4495:
	.loc	1 1192 28
	vmovss	%xmm0, 252(%rsp)
.Ltmp4496:
	.loc	1 1279 33
	vmovss	240(%rbx), %xmm0
.Ltmp4497:
	.loc	1 1192 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp4498:
	.loc	1 1279 33
	vmovss	400(%rbx), %xmm0
.Ltmp4499:
	.loc	1 1192 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp4500:
	.loc	1 1279 33
	vmovss	560(%rbx), %xmm0
.Ltmp4501:
	.loc	1 1192 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp4502:
	.loc	1 1279 33
	vmovss	720(%rbx), %xmm0
.Ltmp4503:
	.loc	1 1192 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp4504:
	.loc	1 1279 33
	vmovss	256(%rbx), %xmm0
.Ltmp4505:
	.loc	1 1192 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp4506:
	.loc	1 1279 33
	vmovss	416(%rbx), %xmm0
.Ltmp4507:
	.loc	1 1192 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp4508:
	.loc	1 1279 33
	vmovss	576(%rbx), %xmm0
.Ltmp4509:
	.loc	1 1192 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp4510:
	.loc	1 1279 33
	vmovss	736(%rbx), %xmm0
.Ltmp4511:
	.loc	1 1192 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp4512:
	.loc	1 1279 33
	vmovss	272(%rbx), %xmm0
.Ltmp4513:
	.loc	1 1192 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp4514:
	.loc	1 1279 33
	vmovss	432(%rbx), %xmm0
.Ltmp4515:
	.loc	1 1192 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp4516:
	.loc	1 1279 33
	vmovss	592(%rbx), %xmm0
.Ltmp4517:
	.loc	1 1192 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp4518:
	.loc	1 1279 33
	vmovss	752(%rbx), %xmm0
.Ltmp4519:
	.loc	1 1192 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp4520:
	.loc	1 1279 33
	vmovss	288(%rbx), %xmm0
.Ltmp4521:
	.loc	1 1192 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp4522:
	.loc	1 1279 33
	vmovss	448(%rbx), %xmm0
.Ltmp4523:
	.loc	1 1192 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp4524:
	.loc	1 1279 33
	vmovss	608(%rbx), %xmm0
.Ltmp4525:
	.loc	1 1192 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp4526:
	.loc	1 1279 33
	vmovss	768(%rbx), %xmm0
.Ltmp4527:
	.loc	1 1192 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp4528:
	.loc	1 1279 33
	vmovss	304(%rbx), %xmm0
.Ltmp4529:
	.loc	1 1192 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp4530:
	.loc	1 1279 33
	vmovss	464(%rbx), %xmm0
.Ltmp4531:
	.loc	1 1192 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp4532:
	.loc	1 1279 33
	vmovss	624(%rbx), %xmm0
.Ltmp4533:
	.loc	1 1192 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp4534:
	.loc	1 1279 33
	vmovss	784(%rbx), %xmm0
.Ltmp4535:
	.loc	1 1192 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp4536:
	.loc	1 1279 33
	vmovss	320(%rbx), %xmm0
.Ltmp4537:
	.loc	1 1192 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp4538:
	.loc	1 1279 33
	vmovss	480(%rbx), %xmm0
.Ltmp4539:
	.loc	1 1192 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp4540:
	.loc	1 1279 33
	vmovss	640(%rbx), %xmm0
.Ltmp4541:
	.loc	1 1192 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp4542:
	.loc	1 1279 33
	vmovss	800(%rbx), %xmm0
.Ltmp4543:
	.loc	1 1192 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp4544:
	.loc	1 1279 33
	vmovss	336(%rbx), %xmm0
.Ltmp4545:
	.loc	1 1192 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp4546:
	.loc	1 1279 33
	vmovss	496(%rbx), %xmm0
.Ltmp4547:
	.loc	1 1192 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp4548:
	.loc	1 1279 33
	vmovss	656(%rbx), %xmm0
.Ltmp4549:
	.loc	1 1192 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp4550:
	.loc	1 1279 33
	vmovss	816(%rbx), %xmm0
.Ltmp4551:
	.loc	1 1192 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp4552:
	.loc	1 1280 32
	vmovss	200(%rbx), %xmm0
.Ltmp4553:
	.loc	1 1192 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp4554:
	.loc	1 1280 32
	vmovss	360(%rbx), %xmm0
.Ltmp4555:
	.loc	1 1192 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp4556:
	.loc	1 1280 32
	vmovss	520(%rbx), %xmm0
.Ltmp4557:
	.loc	1 1192 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp4558:
	.loc	1 1280 32
	vmovss	680(%rbx), %xmm0
.Ltmp4559:
	.loc	1 1192 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp4560:
	.loc	1 1280 32
	vmovss	216(%rbx), %xmm0
.Ltmp4561:
	.loc	1 1192 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp4562:
	.loc	1 1280 32
	vmovss	376(%rbx), %xmm0
.Ltmp4563:
	.loc	1 1192 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp4564:
	.loc	1 1280 32
	vmovss	536(%rbx), %xmm0
.Ltmp4565:
	.loc	1 1192 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp4566:
	.loc	1 1280 32
	vmovss	696(%rbx), %xmm0
.Ltmp4567:
	.loc	1 1192 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp4568:
	.loc	1 1280 32
	vmovss	232(%rbx), %xmm0
.Ltmp4569:
	.loc	1 1192 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp4570:
	.loc	1 1280 32
	vmovss	392(%rbx), %xmm0
.Ltmp4571:
	.loc	1 1192 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp4572:
	.loc	1 1280 32
	vmovss	552(%rbx), %xmm0
.Ltmp4573:
	.loc	1 1192 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp4574:
	.loc	1 1280 32
	vmovss	712(%rbx), %xmm0
.Ltmp4575:
	.loc	1 1192 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp4576:
	.loc	1 1280 32
	vmovss	248(%rbx), %xmm0
.Ltmp4577:
	.loc	1 1192 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp4578:
	.loc	1 1280 32
	vmovss	408(%rbx), %xmm0
.Ltmp4579:
	.loc	1 1192 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp4580:
	.loc	1 1280 32
	vmovss	568(%rbx), %xmm0
.Ltmp4581:
	.loc	1 1192 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp4582:
	.loc	1 1280 32
	vmovss	728(%rbx), %xmm0
.Ltmp4583:
	.loc	1 1192 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp4584:
	.loc	1 1280 32
	vmovss	264(%rbx), %xmm0
.Ltmp4585:
	.loc	1 1192 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp4586:
	.loc	1 1280 32
	vmovss	424(%rbx), %xmm0
.Ltmp4587:
	.loc	1 1192 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp4588:
	.loc	1 1280 32
	vmovss	584(%rbx), %xmm0
.Ltmp4589:
	.loc	1 1192 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp4590:
	.loc	1 1280 32
	vmovss	744(%rbx), %xmm0
.Ltmp4591:
	.loc	1 1192 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp4592:
	.loc	1 1280 32
	vmovss	280(%rbx), %xmm0
.Ltmp4593:
	.loc	1 1192 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp4594:
	.loc	1 1280 32
	vmovss	440(%rbx), %xmm0
.Ltmp4595:
	.loc	1 1192 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp4596:
	.loc	1 1280 32
	vmovss	600(%rbx), %xmm0
.Ltmp4597:
	.loc	1 1192 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp4598:
	.loc	1 1280 32
	vmovss	760(%rbx), %xmm0
.Ltmp4599:
	.loc	1 1192 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp4600:
	.loc	1 1280 32
	vmovss	296(%rbx), %xmm0
.Ltmp4601:
	.loc	1 1192 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp4602:
	.loc	1 1280 32
	vmovss	456(%rbx), %xmm0
.Ltmp4603:
	.loc	1 1192 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp4604:
	.loc	1 1280 32
	vmovss	616(%rbx), %xmm0
.Ltmp4605:
	.loc	1 1192 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp4606:
	.loc	1 1280 32
	vmovss	776(%rbx), %xmm0
.Ltmp4607:
	.loc	1 1192 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp4608:
	.loc	1 1280 32
	vmovss	312(%rbx), %xmm0
.Ltmp4609:
	.loc	1 1192 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp4610:
	.loc	1 1280 32
	vmovss	472(%rbx), %xmm0
.Ltmp4611:
	.loc	1 1192 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp4612:
	.loc	1 1280 32
	vmovss	632(%rbx), %xmm0
.Ltmp4613:
	.loc	1 1192 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp4614:
	.loc	1 1280 32
	vmovss	792(%rbx), %xmm0
.Ltmp4615:
	.loc	1 1192 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp4616:
	.loc	1 1280 32
	vmovss	328(%rbx), %xmm0
.Ltmp4617:
	.loc	1 1192 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp4618:
	.loc	1 1280 32
	vmovss	488(%rbx), %xmm0
.Ltmp4619:
	.loc	1 1192 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp4620:
	.loc	1 1280 32
	vmovss	648(%rbx), %xmm0
.Ltmp4621:
	.loc	1 1192 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp4622:
	.loc	1 1280 32
	vmovss	808(%rbx), %xmm0
.Ltmp4623:
	.loc	1 1192 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp4624:
	.loc	1 1280 32
	vmovss	344(%rbx), %xmm0
.Ltmp4625:
	.loc	1 1192 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp4626:
	.loc	1 1280 32
	vmovss	504(%rbx), %xmm0
.Ltmp4627:
	.loc	1 1192 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp4628:
	.loc	1 1280 32
	vmovss	664(%rbx), %xmm0
.Ltmp4629:
	.loc	1 1192 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp4630:
	.loc	1 1280 32
	vmovss	824(%rbx), %xmm0
.Ltmp4631:
	.loc	1 1192 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp4632:
	.loc	1 1279 33
	vmovss	1520(%rbx), %xmm0
.Ltmp4633:
	.loc	1 1192 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp4634:
	.loc	1 1279 33
	vmovss	1680(%rbx), %xmm0
.Ltmp4635:
	.loc	1 1192 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp4636:
	.loc	1 1279 33
	vmovss	1840(%rbx), %xmm0
.Ltmp4637:
	.loc	1 1192 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp4638:
	.loc	1 1279 33
	vmovss	2000(%rbx), %xmm0
.Ltmp4639:
	.loc	1 1192 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp4640:
	.loc	1 1279 33
	vmovss	1536(%rbx), %xmm0
.Ltmp4641:
	.loc	1 1192 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp4642:
	.loc	1 1279 33
	vmovss	1696(%rbx), %xmm0
.Ltmp4643:
	.loc	1 1192 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp4644:
	.loc	1 1279 33
	vmovss	1856(%rbx), %xmm0
.Ltmp4645:
	.loc	1 1192 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp4646:
	.loc	1 1279 33
	vmovss	2016(%rbx), %xmm0
.Ltmp4647:
	.loc	1 1192 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp4648:
	.loc	1 1279 33
	vmovss	1552(%rbx), %xmm0
.Ltmp4649:
	.loc	1 1192 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp4650:
	.loc	1 1279 33
	vmovss	1712(%rbx), %xmm0
.Ltmp4651:
	.loc	1 1192 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp4652:
	.loc	1 1279 33
	vmovss	1872(%rbx), %xmm0
.Ltmp4653:
	.loc	1 1192 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp4654:
	.loc	1 1279 33
	vmovss	2032(%rbx), %xmm0
.Ltmp4655:
	.loc	1 1192 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp4656:
	.loc	1 1279 33
	vmovss	1568(%rbx), %xmm0
.Ltmp4657:
	.loc	1 1192 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp4658:
	.loc	1 1279 33
	vmovss	1728(%rbx), %xmm0
.Ltmp4659:
	.loc	1 1192 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp4660:
	.loc	1 1279 33
	vmovss	1888(%rbx), %xmm0
.Ltmp4661:
	.loc	1 1192 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp4662:
	.loc	1 1279 33
	vmovss	2048(%rbx), %xmm0
.Ltmp4663:
	.loc	1 1192 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp4664:
	.loc	1 1279 33
	vmovss	1584(%rbx), %xmm0
.Ltmp4665:
	.loc	1 1192 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp4666:
	.loc	1 1279 33
	vmovss	1744(%rbx), %xmm0
.Ltmp4667:
	.loc	1 1192 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp4668:
	.loc	1 1279 33
	vmovss	1904(%rbx), %xmm0
.Ltmp4669:
	.loc	1 1192 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp4670:
	.loc	1 1279 33
	vmovss	2064(%rbx), %xmm0
.Ltmp4671:
	.loc	1 1192 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp4672:
	.loc	1 1279 33
	vmovss	1600(%rbx), %xmm0
.Ltmp4673:
	.loc	1 1192 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp4674:
	.loc	1 1279 33
	vmovss	1760(%rbx), %xmm0
.Ltmp4675:
	.loc	1 1192 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp4676:
	.loc	1 1279 33
	vmovss	1920(%rbx), %xmm0
.Ltmp4677:
	.loc	1 1192 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp4678:
	.loc	1 1279 33
	vmovss	2080(%rbx), %xmm0
.Ltmp4679:
	.loc	1 1192 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp4680:
	.loc	1 1279 33
	vmovss	1616(%rbx), %xmm0
.Ltmp4681:
	.loc	1 1192 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp4682:
	.loc	1 1279 33
	vmovss	1776(%rbx), %xmm0
.Ltmp4683:
	.loc	1 1192 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp4684:
	.loc	1 1279 33
	vmovss	1936(%rbx), %xmm0
.Ltmp4685:
	.loc	1 1192 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp4686:
	.loc	1 1279 33
	vmovss	2096(%rbx), %xmm0
.Ltmp4687:
	.loc	1 1192 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp4688:
	.loc	1 1279 33
	vmovss	1632(%rbx), %xmm0
.Ltmp4689:
	.loc	1 1192 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp4690:
	.loc	1 1279 33
	vmovss	1792(%rbx), %xmm0
.Ltmp4691:
	.loc	1 1192 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp4692:
	.loc	1 1279 33
	vmovss	1952(%rbx), %xmm0
.Ltmp4693:
	.loc	1 1192 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp4694:
	.loc	1 1279 33
	vmovss	2112(%rbx), %xmm0
.Ltmp4695:
	.loc	1 1192 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp4696:
	.loc	1 1279 33
	vmovss	1648(%rbx), %xmm0
.Ltmp4697:
	.loc	1 1192 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp4698:
	.loc	1 1279 33
	vmovss	1808(%rbx), %xmm0
.Ltmp4699:
	.loc	1 1192 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp4700:
	.loc	1 1279 33
	vmovss	1968(%rbx), %xmm0
.Ltmp4701:
	.loc	1 1192 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp4702:
	.loc	1 1279 33
	vmovss	2128(%rbx), %xmm0
.Ltmp4703:
	.loc	1 1192 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp4704:
	.loc	1 1279 33
	vmovss	1664(%rbx), %xmm0
.Ltmp4705:
	.loc	1 1192 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp4706:
	.loc	1 1279 33
	vmovss	1824(%rbx), %xmm0
.Ltmp4707:
	.loc	1 1192 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp4708:
	.loc	1 1279 33
	vmovss	1984(%rbx), %xmm0
.Ltmp4709:
	.loc	1 1192 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp4710:
	.loc	1 1279 33
	vmovss	2144(%rbx), %xmm0
.Ltmp4711:
	.loc	1 1192 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp4712:
	.loc	1 1280 32
	vmovss	1528(%rbx), %xmm0
.Ltmp4713:
	.loc	1 1192 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp4714:
	.loc	1 1280 32
	vmovss	1688(%rbx), %xmm0
.Ltmp4715:
	.loc	1 1192 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp4716:
	.loc	1 1280 32
	vmovss	1848(%rbx), %xmm0
.Ltmp4717:
	.loc	1 1192 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp4718:
	.loc	1 1280 32
	vmovss	2008(%rbx), %xmm0
.Ltmp4719:
	.loc	1 1192 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp4720:
	.loc	1 1280 32
	vmovss	1544(%rbx), %xmm0
.Ltmp4721:
	.loc	1 1192 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp4722:
	.loc	1 1280 32
	vmovss	1704(%rbx), %xmm0
.Ltmp4723:
	.loc	1 1192 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp4724:
	.loc	1 1280 32
	vmovss	1864(%rbx), %xmm0
.Ltmp4725:
	.loc	1 1192 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp4726:
	.loc	1 1280 32
	vmovss	2024(%rbx), %xmm0
.Ltmp4727:
	.loc	1 1192 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp4728:
	.loc	1 1280 32
	vmovss	1560(%rbx), %xmm0
.Ltmp4729:
	.loc	1 1192 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp4730:
	.loc	1 1280 32
	vmovss	1720(%rbx), %xmm0
.Ltmp4731:
	.loc	1 1192 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp4732:
	.loc	1 1280 32
	vmovss	1880(%rbx), %xmm0
.Ltmp4733:
	.loc	1 1192 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp4734:
	.loc	1 1280 32
	vmovss	2040(%rbx), %xmm0
.Ltmp4735:
	.loc	1 1192 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp4736:
	.loc	1 1280 32
	vmovss	1576(%rbx), %xmm0
.Ltmp4737:
	.loc	1 1192 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp4738:
	.loc	1 1280 32
	vmovss	1736(%rbx), %xmm0
.Ltmp4739:
	.loc	1 1192 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp4740:
	.loc	1 1280 32
	vmovss	1896(%rbx), %xmm0
.Ltmp4741:
	.loc	1 1192 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp4742:
	.loc	1 1280 32
	vmovss	2056(%rbx), %xmm0
.Ltmp4743:
	.loc	1 1192 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp4744:
	.loc	1 1280 32
	vmovss	1592(%rbx), %xmm0
.Ltmp4745:
	.loc	1 1192 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp4746:
	.loc	1 1280 32
	vmovss	1752(%rbx), %xmm0
.Ltmp4747:
	.loc	1 1192 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp4748:
	.loc	1 1280 32
	vmovss	1912(%rbx), %xmm0
.Ltmp4749:
	.loc	1 1192 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp4750:
	.loc	1 1280 32
	vmovss	2072(%rbx), %xmm0
.Ltmp4751:
	.loc	1 1192 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp4752:
	.loc	1 1280 32
	vmovss	1608(%rbx), %xmm0
.Ltmp4753:
	.loc	1 1192 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp4754:
	.loc	1 1280 32
	vmovss	1768(%rbx), %xmm0
.Ltmp4755:
	.loc	1 1192 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp4756:
	.loc	1 1280 32
	vmovss	1928(%rbx), %xmm0
.Ltmp4757:
	.loc	1 1192 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp4758:
	.loc	1 1280 32
	vmovss	2088(%rbx), %xmm0
.Ltmp4759:
	.loc	1 1192 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp4760:
	.loc	1 1280 32
	vmovss	1624(%rbx), %xmm0
.Ltmp4761:
	.loc	1 1192 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp4762:
	.loc	1 1280 32
	vmovss	1784(%rbx), %xmm0
.Ltmp4763:
	.loc	1 1192 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp4764:
	.loc	1 1280 32
	vmovss	1944(%rbx), %xmm0
.Ltmp4765:
	.loc	1 1192 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp4766:
	.loc	1 1280 32
	vmovss	2104(%rbx), %xmm0
.Ltmp4767:
	.loc	1 1192 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp4768:
	.loc	1 1280 32
	vmovss	1640(%rbx), %xmm0
.Ltmp4769:
	.loc	1 1192 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp4770:
	.loc	1 1280 32
	vmovss	1800(%rbx), %xmm0
.Ltmp4771:
	.loc	1 1192 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp4772:
	.loc	1 1280 32
	vmovss	1960(%rbx), %xmm0
.Ltmp4773:
	.loc	1 1192 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp4774:
	.loc	1 1280 32
	vmovss	2120(%rbx), %xmm0
.Ltmp4775:
	.loc	1 1192 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp4776:
	.loc	1 1280 32
	vmovss	1656(%rbx), %xmm0
.Ltmp4777:
	.loc	1 1192 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp4778:
	.loc	1 1280 32
	vmovss	1816(%rbx), %xmm0
.Ltmp4779:
	.loc	1 1192 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp4780:
	.loc	1 1280 32
	vmovss	1976(%rbx), %xmm0
.Ltmp4781:
	.loc	1 1192 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp4782:
	.loc	1 1280 32
	vmovss	2136(%rbx), %xmm0
.Ltmp4783:
	.loc	1 1192 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp4784:
	.loc	1 1280 32
	vmovss	1672(%rbx), %xmm0
.Ltmp4785:
	.loc	1 1192 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp4786:
	.loc	1 1280 32
	vmovss	1832(%rbx), %xmm0
.Ltmp4787:
	.loc	1 1192 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp4788:
	.loc	1 1280 32
	vmovss	1992(%rbx), %xmm0
.Ltmp4789:
	.loc	1 1192 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp4790:
	.loc	1 1280 32
	vmovss	2152(%rbx), %xmm0
.Ltmp4791:
	.loc	1 1192 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp4792:
	.loc	1 1194 31
	leaq	1328(%rsp), %rdi
	movq	%rbx, %rsi
	movl	1104(%rsp), %ebp
	movl	%ebp, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	1424(%rsp), %rdi
	movq	1312(%rsp), %rsi
	movl	%ebp, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1193 28
	vmovaps	1328(%rsp), %xmm0
	vmovaps	%xmm0, 1216(%rsp)
	vmovaps	1344(%rsp), %xmm0
	vmovaps	%xmm0, 1200(%rsp)
	vmovaps	1360(%rsp), %xmm0
	vmovaps	%xmm0, 1184(%rsp)
	vmovaps	1376(%rsp), %xmm0
	vmovaps	%xmm0, 1168(%rsp)
	vmovaps	1392(%rsp), %xmm0
	vmovaps	%xmm0, 1152(%rsp)
	vmovaps	1408(%rsp), %xmm0
	vmovaps	%xmm0, 1136(%rsp)
	vmovaps	1424(%rsp), %xmm0
	vmovaps	%xmm0, 1120(%rsp)
	vmovaps	1440(%rsp), %xmm0
	vmovaps	%xmm0, 1296(%rsp)
	vmovaps	1456(%rsp), %xmm0
	vmovaps	%xmm0, 1280(%rsp)
	vmovaps	1472(%rsp), %xmm0
	vmovaps	%xmm0, 1264(%rsp)
	vmovaps	1488(%rsp), %xmm0
	vmovaps	%xmm0, 1248(%rsp)
	vmovaps	1504(%rsp), %xmm0
	vmovaps	%xmm0, 1232(%rsp)
.Ltmp4793:
	.loc	1 0 0 is_stmt 0
	leaq	(%r14,%r15), %rax
	shlq	$2, %r15
	leaq	(,%rax,4), %rsi
	.loc	1 1197 12 is_stmt 1
	testb	$1, %r12b
	movq	%r14, 1024(%rsp)
	movq	%rax, 1112(%rsp)
	je	.LBB34_332
.Ltmp4794:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp4795:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_570
.Ltmp4796:
	.loc	48 451 16 is_stmt 1
	cmpq	112(%rsp), %rsi
	ja	.LBB34_570
.Ltmp4797:
	.loc	48 451 16 is_stmt 0
	cmpq	104(%rsp), %rsi
	ja	.LBB34_577
.Ltmp4798:
	.loc	1 1053 27 is_stmt 1
	vmovaps	96(%rbx), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	112(%rbx), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	128(%rbx), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	144(%rbx), %xmm0
	vmovaps	%xmm0, 64(%rsp)
.Ltmp4799:
	.loc	1 1054 26
	vmovaps	1424(%rbx), %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vmovaps	1440(%rbx), %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmovaps	1456(%rbx), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	1472(%rbx), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4800:
	.loc	1 1055 25
	vmovaps	160(%rbx), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	176(%rbx), %xmm0
	vmovaps	%xmm0, 880(%rsp)
.Ltmp4801:
	.loc	1 1056 24
	vmovaps	1488(%rbx), %xmm0
	vmovaps	%xmm0, 896(%rsp)
	vmovaps	1504(%rbx), %xmm7
.Ltmp4802:
	.loc	1 1057 24
	movq	2680(%rbx), %r12
.Ltmp4803:
	.loc	1 871 17
	movq	1104(%rbx), %rax
	movq	1112(%rbx), %rcx
.Ltmp4804:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	1120(%rbx), %rdx
	xorq	%rax, %rdx
	orq	%rcx, %rdx
	xorq	1128(%rbx), %rax
	orq	%rdx, %rax
	sete	15(%rsp)
.Ltmp4805:
	.loc	1 871 17
	movq	2432(%rbx), %rax
	movq	2440(%rbx), %rcx
.Ltmp4806:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2448(%rbx), %rdx
	xorq	%rax, %rdx
	xorq	2456(%rbx), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	192(%rsp)
.Ltmp4807:
	.loc	2 1916 50
	testq	%r14, %r14
	movq	176(%rsp), %rbp
	je	.LBB34_328
.Ltmp4808:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r14,4), %rax
	movq	%rax, 152(%rsp)
	movq	968(%rsp), %rax
	leaq	(%rax,%r15,4), %rcx
	movq	120(%rsp), %rax
	leaq	(%rax,%r15,4), %r9
.Ltmp4809:
	.loc	3 900 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	movq	%r14, 936(%rsp)
	xorl	%r10d, %r10d
	xorl	%esi, %esi
	movq	%rcx, 976(%rsp)
	movq	%r9, 928(%rsp)
.Ltmp4810:
	.loc	3 0 12 is_stmt 0
.Ltmp4811:
	.p2align	4
.LBB34_279:
	.loc	1 1064 21 is_stmt 1
	vmovaps	208(%rsp), %xmm0
	vmovaps	224(%rsp), %xmm1
	vmovaps	240(%rsp), %xmm2
.Ltmp4812:
	.loc	9 36 14
	vaddps	368(%rsp), %xmm0, %xmm0
.Ltmp4813:
	.loc	1 1066 21
	vmovaps	528(%rsp), %xmm3
	.loc	1 1063 17
	vmovaps	%xmm0, 208(%rsp)
.Ltmp4814:
	.loc	9 36 14
	vaddps	688(%rsp), %xmm3, %xmm0
.Ltmp4815:
	.loc	1 1065 17
	vmovaps	%xmm0, 528(%rsp)
.Ltmp4816:
	.loc	9 36 14
	vaddps	384(%rsp), %xmm1, %xmm0
.Ltmp4817:
	.loc	1 1063 17
	vmovaps	%xmm0, 224(%rsp)
	.loc	1 1066 21
	vmovaps	544(%rsp), %xmm0
.Ltmp4818:
	.loc	9 36 14
	vaddps	704(%rsp), %xmm0, %xmm0
.Ltmp4819:
	.loc	1 1065 17
	vmovaps	%xmm0, 544(%rsp)
.Ltmp4820:
	.loc	9 36 14
	vaddps	400(%rsp), %xmm2, %xmm0
.Ltmp4821:
	.loc	1 1063 17
	vmovaps	%xmm0, 240(%rsp)
	.loc	1 1066 21
	vmovaps	560(%rsp), %xmm0
.Ltmp4822:
	.loc	9 36 14
	vaddps	720(%rsp), %xmm0, %xmm0
.Ltmp4823:
	.loc	1 1065 17
	vmovaps	%xmm0, 560(%rsp)
	.loc	1 1064 21
	vmovaps	256(%rsp), %xmm0
.Ltmp4824:
	.loc	9 36 14
	vaddps	416(%rsp), %xmm0, %xmm0
.Ltmp4825:
	.loc	1 1063 17
	vmovaps	%xmm0, 256(%rsp)
	.loc	1 1066 21
	vmovaps	576(%rsp), %xmm0
.Ltmp4826:
	.loc	9 36 14
	vaddps	736(%rsp), %xmm0, %xmm0
.Ltmp4827:
	.loc	1 1065 17
	vmovaps	%xmm0, 576(%rsp)
	.loc	1 1064 21
	vmovaps	272(%rsp), %xmm0
.Ltmp4828:
	.loc	9 36 14
	vaddps	432(%rsp), %xmm0, %xmm0
.Ltmp4829:
	.loc	1 1063 17
	vmovaps	%xmm0, 272(%rsp)
	.loc	1 1066 21
	vmovaps	592(%rsp), %xmm0
.Ltmp4830:
	.loc	9 36 14
	vaddps	752(%rsp), %xmm0, %xmm0
.Ltmp4831:
	.loc	1 1065 17
	vmovaps	%xmm0, 592(%rsp)
	.loc	1 1064 21
	vmovaps	288(%rsp), %xmm0
.Ltmp4832:
	.loc	9 36 14
	vaddps	448(%rsp), %xmm0, %xmm0
.Ltmp4833:
	.loc	1 1063 17
	vmovaps	%xmm0, 288(%rsp)
	.loc	1 1066 21
	vmovaps	608(%rsp), %xmm0
.Ltmp4834:
	.loc	9 36 14
	vaddps	768(%rsp), %xmm0, %xmm0
.Ltmp4835:
	.loc	1 1065 17
	vmovaps	%xmm0, 608(%rsp)
	.loc	1 1064 21
	vmovaps	304(%rsp), %xmm0
.Ltmp4836:
	.loc	9 36 14
	vaddps	464(%rsp), %xmm0, %xmm0
.Ltmp4837:
	.loc	1 1063 17
	vmovaps	%xmm0, 304(%rsp)
	.loc	1 1066 21
	vmovaps	624(%rsp), %xmm0
.Ltmp4838:
	.loc	9 36 14
	vaddps	784(%rsp), %xmm0, %xmm0
.Ltmp4839:
	.loc	1 1065 17
	vmovaps	%xmm0, 624(%rsp)
	.loc	1 1064 21
	vmovaps	320(%rsp), %xmm0
.Ltmp4840:
	.loc	9 36 14
	vaddps	480(%rsp), %xmm0, %xmm0
.Ltmp4841:
	.loc	1 1063 17
	vmovaps	%xmm0, 320(%rsp)
	.loc	1 1066 21
	vmovaps	640(%rsp), %xmm0
.Ltmp4842:
	.loc	9 36 14
	vaddps	800(%rsp), %xmm0, %xmm0
.Ltmp4843:
	.loc	1 1065 17
	vmovaps	%xmm0, 640(%rsp)
	.loc	1 1064 21
	vmovaps	336(%rsp), %xmm0
.Ltmp4844:
	.loc	9 36 14
	vaddps	496(%rsp), %xmm0, %xmm0
.Ltmp4845:
	.loc	1 1063 17
	vmovaps	%xmm0, 336(%rsp)
	.loc	1 1066 21
	vmovaps	656(%rsp), %xmm0
.Ltmp4846:
	.loc	9 36 14
	vaddps	816(%rsp), %xmm0, %xmm0
.Ltmp4847:
	.loc	1 1065 17
	vmovaps	%xmm0, 656(%rsp)
	.loc	1 1064 21
	vmovaps	352(%rsp), %xmm0
.Ltmp4848:
	.loc	9 36 14
	vaddps	512(%rsp), %xmm0, %xmm0
.Ltmp4849:
	.loc	1 1063 17
	vmovaps	%xmm0, 352(%rsp)
	.loc	1 1066 21
	vmovaps	672(%rsp), %xmm0
.Ltmp4850:
	.loc	9 36 14
	vaddps	832(%rsp), %xmm0, %xmm0
.Ltmp4851:
	.loc	1 1065 17
	vmovaps	%xmm0, 672(%rsp)
.Ltmp4852:
	.loc	1 1070 28
	leaq	1(%r12), %rax
.Ltmp4853:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %edi
	cmovaeq	%rbp, %rdi
.Ltmp4854:
	.loc	48 568 12
	cmpq	152(%rsp), %r10
	ja	.LBB34_565
.Ltmp4855:
	.loc	48 438 16
	cmpq	%rsi, 936(%rsp)
	je	.LBB34_534
.Ltmp4856:
	.loc	48 0 16 is_stmt 0
	movq	%rsi, 200(%rsp)
	leaq	(,%r12,4), %rax
.Ltmp4857:
	.loc	1 1083 29 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp4858:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_540
.Ltmp4859:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4860:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm7, 1088(%rsp)
	vmovups	(%rcx,%r10,4), %xmm7
.Ltmp4861:
	vmovups	(%r9,%r10,4), %xmm1
.Ltmp4862:
	vmovaps	32(%rbx), %xmm5
	vmovaps	48(%rbx), %xmm11
	vmovaps	64(%rbx), %xmm0
	vmovaps	1360(%rbx), %xmm14
	vmovaps	1376(%rbx), %xmm9
	vmovaps	1392(%rbx), %xmm3
	vmovaps	16(%rsp), %xmm10
	vsubps	%xmm10, %xmm7, %xmm2
	vmulps	%xmm2, %xmm11, %xmm4
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm5, 160(%rsp)
	vmulps	%xmm5, %xmm8, %xmm5
	vaddps	%xmm4, %xmm5, %xmm6
	vaddps	%xmm6, %xmm8, %xmm4
	vmulps	%xmm11, %xmm8, %xmm5
	vmulps	%xmm0, %xmm2, %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vaddps	%xmm5, %xmm10, %xmm2
	vmulps	80(%rbx), %xmm4, %xmm15
	vmovaps	64(%rsp), %xmm8
	vsubps	%xmm8, %xmm2, %xmm4
	vmulps	80(%rsp), %xmm11, %xmm2
	vmulps	%xmm4, %xmm0, %xmm0
	vaddps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vaddps	%xmm0, %xmm8, %xmm0
	vmovaps	912(%rsp), %xmm10
.Ltmp4863:
	vsubps	%xmm10, %xmm1, %xmm13
	vmulps	%xmm9, %xmm13, %xmm2
	vmovaps	992(%rsp), %xmm8
	vmulps	%xmm14, %xmm8, %xmm12
	vaddps	%xmm2, %xmm12, %xmm2
	vaddps	%xmm2, %xmm8, %xmm12
	vmulps	1408(%rbx), %xmm12, %xmm12
.Ltmp4864:
	.loc	1 1083 29 is_stmt 1
	movq	(%rbx), %rcx
.Ltmp4865:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4866:
	.loc	1 1084 30
	movq	24(%rbx), %rsi
.Ltmp4867:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_541
.Ltmp4868:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4869:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm7, %xmm15, %xmm7
	vsubps	%xmm0, %xmm7, %xmm0
.Ltmp4870:
	.loc	1 1084 30 is_stmt 1
	movq	16(%rbx), %rcx
.Ltmp4871:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4872:
	.loc	1 1085 28
	movq	1336(%rbx), %rsi
.Ltmp4873:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_542
.Ltmp4874:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4875:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm14, 944(%rsp)
	vmulps	%xmm9, %xmm8, %xmm0
	vmulps	%xmm3, %xmm13, %xmm7
	vaddps	%xmm7, %xmm0, %xmm13
	vaddps	%xmm13, %xmm10, %xmm0
	vmovaps	128(%rsp), %xmm14
	vsubps	%xmm14, %xmm0, %xmm15
	vmulps	48(%rsp), %xmm9, %xmm0
	vmulps	%xmm3, %xmm15, %xmm3
	vaddps	%xmm3, %xmm0, %xmm7
	vaddps	%xmm7, %xmm14, %xmm0
.Ltmp4876:
	.loc	1 1085 28 is_stmt 1
	movq	1328(%rbx), %rcx
.Ltmp4877:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4878:
	.loc	1 1086 29
	movq	1352(%rbx), %rsi
.Ltmp4879:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_543
.Ltmp4880:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4881:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm1, %xmm12, %xmm1
	vsubps	%xmm0, %xmm1, %xmm0
.Ltmp4882:
	.loc	1 1086 29 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp4883:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp4884:
	.loc	1 1089 13
	movq	(%rbx), %r14
	movq	8(%rbx), %rsi
	movq	1104(%rbx), %rax
.Ltmp4885:
	.loc	1 0 0 is_stmt 0
	addq	%r12, %rax
.Ltmp4886:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp4887:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 15(%rsp)
	movq	%r10, 1008(%rsp)
	je	.LBB34_295
.Ltmp4888:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp4889:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4890:
	.loc	1 1096 13
	movq	24(%rbx), %rsi
.Ltmp4891:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp4892:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp4893:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp4894:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4895:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r14,%r9,4), %xmm12
.Ltmp4896:
	movq	16(%rbx), %rcx
.Ltmp4897:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm1
.Ltmp4898:
	.loc	1 961 2
	jmp	.LBB34_304
.Ltmp4899:
	.loc	1 0 2 is_stmt 0
.Ltmp4900:
	.p2align	4
.LBB34_295:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	1112(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4901:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp4902:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_582
	.loc	1 0 25
	movq	1120(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4903:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp4904:
	.loc	1 955 30
	leaq	2(,%rcx,4), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_601
	.loc	1 0 25
	movq	1128(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4905:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp4906:
	.loc	1 955 30
	leaq	3(,%rcx,4), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB34_607
.Ltmp4907:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp4908:
	.loc	1 1096 13
	movq	24(%r13), %rsi
.Ltmp4909:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp4910:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	1112(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4911:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp4912:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_580
	.loc	1 0 25
	movq	1120(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4913:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%rbp, %r10
	movl	$0, %ebp
	cmovaeq	%r10, %rbp
	subq	%rbp, %rbx
.Ltmp4914:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	%r11, %rcx
	movq	%r8, %r11
	movq	%r14, %r8
	movq	1128(%r13), %rbx
	movq	%r12, %r14
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4915:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r12d
	cmovaeq	%r10, %r12
	subq	%r12, %rbx
.Ltmp4916:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp4917:
	.loc	1 0 0
	vmovd	(%r8,%r9,4), %xmm0
	vpinsrd	$1, (%r8,%r11,4), %xmm0, %xmm0
	vpinsrd	$2, (%r8,%rcx,4), %xmm0, %xmm0
	vpinsrd	$3, (%r8,%rdx,4), %xmm0, %xmm12
.Ltmp4918:
	movq	16(%r13), %rcx
.Ltmp4919:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm1
	movq	%r13, %rbx
	movq	176(%rsp), %rbp
	movq	1008(%rsp), %r10
	movq	%r14, %r12
.Ltmp4920:
.LBB34_304:
	.loc	1 1103 13
	movq	1328(%rbx), %rdx
	movq	1336(%rbx), %rsi
	movq	2432(%rbx), %rax
.Ltmp4921:
	.loc	1 0 0 is_stmt 0
	addq	%r12, %rax
.Ltmp4922:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp4923:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 192(%rsp)
	je	.LBB34_310
.Ltmp4924:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp4925:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4926:
	.loc	1 1110 13
	movq	1352(%rbx), %rsi
.Ltmp4927:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp4928:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp4929:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp4930:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4931:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%rdx,%r9,4), %xmm3
.Ltmp4932:
	movq	1344(%rbx), %rcx
.Ltmp4933:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm0
.Ltmp4934:
	.loc	1 961 2
	jmp	.LBB34_319
.Ltmp4935:
	.loc	1 0 2 is_stmt 0
.Ltmp4936:
	.p2align	4
.LBB34_310:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	2440(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4937:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp4938:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_582
	.loc	1 0 25
	movq	2448(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4939:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp4940:
	.loc	1 955 30
	leaq	2(,%rcx,4), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_601
	.loc	1 0 25
	movq	2456(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp4941:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp4942:
	.loc	1 955 30
	leaq	3(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
.Ltmp4943:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp4944:
	.loc	1 1110 13
	movq	1352(%r13), %rsi
.Ltmp4945:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp4946:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	%rcx, 1040(%rsp)
	movq	%rdi, 1056(%rsp)
	movq	2440(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4947:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp4948:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_580
	.loc	1 0 25
	movq	2448(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp4949:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%r12, %r14
	movl	$0, %r12d
	cmovaeq	%rbp, %r12
	subq	%r12, %rbx
	movq	%rbp, %r10
.Ltmp4950:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	2456(%r13), %rbx
	.loc	1 955 35
	addq	%r14, %rbx
.Ltmp4951:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r12d
	cmovaeq	%r10, %r12
	subq	%r12, %rbx
.Ltmp4952:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp4953:
	.loc	1 0 25
	movq	%r11, %rcx
	movq	%rdx, %r11
	movq	%r8, %rdx
	vmovd	(%r11,%r9,4), %xmm0
	vpinsrd	$1, (%r11,%rdx,4), %xmm0, %xmm0
	vpinsrd	$2, (%r11,%rcx,4), %xmm0, %xmm0
	movq	1040(%rsp), %rcx
	vpinsrd	$3, (%r11,%rcx,4), %xmm0, %xmm3
.Ltmp4954:
	movq	1344(%r13), %rcx
.Ltmp4955:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm0
	movq	%r13, %rbx
	movq	176(%rsp), %rbp
	movq	1008(%rsp), %r10
	movq	%r14, %r12
	movq	1056(%rsp), %rdi
.Ltmp4956:
.LBB34_319:
	.loc	1 0 0 is_stmt 0
	negq	%rdi
	addq	%rdi, %r12
	incq	%r12
	leaq	(,%r12,4), %rax
.Ltmp4957:
	.loc	1 1150 36 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp4958:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	1024(%rsp), %r14
	movq	928(%rsp), %r9
	jb	.LBB34_546
.Ltmp4959:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4960:
	.loc	1 1152 27
	movq	24(%rbx), %rsi
.Ltmp4961:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_547
.Ltmp4962:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4963:
	.loc	1 1153 35
	movq	1336(%rbx), %rsi
.Ltmp4964:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_548
.Ltmp4965:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4966:
	.loc	1 1155 27
	movq	1352(%rbx), %rsi
.Ltmp4967:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_550
.Ltmp4968:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp4969:
	.loc	48 0 16 is_stmt 0
	vmovdqa	%xmm0, 1056(%rsp)
	vaddps	%xmm6, %xmm6, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vmovdqa	%xmm1, 1040(%rsp)
	vmovdqa	%xmm12, %xmm1
	vbroadcastss	.LCPI34_35(%rip), %xmm12
	vandps	%xmm0, %xmm12, %xmm6
	vmovaps	%xmm10, %xmm14
	vmovaps	%xmm8, %xmm10
	vbroadcastss	.LCPI34_2(%rip), %xmm8
	vcmpltps	%xmm8, %xmm6, %xmm6
	vandnps	%xmm0, %xmm6, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm5, %xmm5, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm5
	vcmpltps	%xmm8, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmulps	%xmm4, %xmm11, %xmm0
	vmovaps	80(%rsp), %xmm5
	vmulps	160(%rsp), %xmm5, %xmm4
	vaddps	%xmm0, %xmm4, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm5, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	848(%rsp), %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 64(%rsp)
.Ltmp4970:
	vaddps	%xmm2, %xmm2, %xmm0
	vaddps	%xmm0, %xmm10, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	%xmm0, %xmm14, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmulps	%xmm15, %xmm9, %xmm0
	vmovaps	48(%rsp), %xmm4
	vmulps	944(%rsp), %xmm4, %xmm2
	vaddps	%xmm0, %xmm2, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm4, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm7, %xmm7, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp4971:
	vpand	%xmm1, %xmm12, %xmm0
	vpand	%xmm3, %xmm12, %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
.Ltmp4972:
	vandps	1040(%rsp), %xmm12, %xmm1
	vandps	1056(%rsp), %xmm12, %xmm2
	vmaxps	%xmm2, %xmm1, %xmm2
	vbroadcastss	.LCPI34_4(%rip), %xmm5
.Ltmp4973:
	vmaxps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm6
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm7
	vandps	%xmm7, %xmm0, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm9
	vorps	%xmm1, %xmm9, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm10
	vaddps	%xmm1, %xmm10, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm1, %xmm11, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm14
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vsubps	208(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm0, %xmm1, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm7
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1216(%rsp), %xmm3, %xmm3
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm0, %xmm1, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	864(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm3
	vmovaps	1184(%rsp), %xmm4
	vblendvps	%xmm3, 1200(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm1, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm8, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm1
.Ltmp4974:
	vbroadcastss	.LCPI34_4(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm0
	vmaxps	%xmm6, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm2
	vorps	%xmm2, %xmm9, %xmm2
	vaddps	%xmm2, %xmm10, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 944(%rsp)
	vsubps	288(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm3
	vaddps	%xmm3, %xmm0, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vmulps	%xmm5, %xmm2, %xmm2
	vcmpltps	%xmm0, %xmm3, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm2, %xmm2
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1168(%rsp), %xmm2, %xmm2
	vxorps	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm3, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm1, 864(%rsp)
.Ltmp4975:
	vaddps	272(%rsp), %xmm1, %xmm2
	vbroadcastss	.LCPI34_24(%rip), %xmm10
	vmulps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_25(%rip), %xmm11
	vmaxps	%xmm11, %xmm2, %xmm2
	vbroadcastss	.LCPI34_26(%rip), %xmm11
	vminps	%xmm11, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm4
	vsubps	%xmm4, %xmm2, %xmm2
	vbroadcastss	.LCPI34_27(%rip), %xmm14
	vmulps	%xmm2, %xmm14, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm7
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm8
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm9
	vaddps	%xmm5, %xmm9, %xmm5
	vbroadcastss	.LCPI34_23(%rip), %xmm14
.Ltmp4976:
	vmaxps	%xmm14, %xmm0, %xmm0
	vminps	%xmm3, %xmm0, %xmm0
	vmovaps	880(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm6
	vmovaps	1136(%rsp), %xmm10
	vblendvps	%xmm6, 1152(%rsp), %xmm10, %xmm6
.Ltmp4977:
	vmulps	%xmm5, %xmm2, %xmm2
.Ltmp4978:
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm6, %xmm5, %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm5
	vbroadcastss	.LCPI34_2(%rip), %xmm13
	vcmpltps	%xmm13, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm6
.Ltmp4979:
	vaddps	%xmm6, %xmm2, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm10
	vaddps	%xmm4, %xmm10, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vmovaps	%xmm1, 880(%rsp)
.Ltmp4980:
	vaddps	352(%rsp), %xmm1, %xmm4
.Ltmp4981:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vbroadcastss	.LCPI34_24(%rip), %xmm2
.Ltmp4982:
	vmulps	%xmm2, %xmm4, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm0
	vaddps	%xmm4, %xmm10, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmovaps	160(%rsp), %xmm1
.Ltmp4983:
	vsubps	528(%rsp), %xmm1, %xmm5
.Ltmp4984:
	vmulps	%xmm4, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm6
.Ltmp4985:
	vaddps	%xmm6, %xmm5, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm11
	vmulps	%xmm0, %xmm11, %xmm0
	vcmpltps	%xmm5, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm10
	vcmpleps	%xmm10, %xmm5, %xmm4
	vmulps	1120(%rsp), %xmm0, %xmm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm4, %xmm1, %xmm4
	vpandn	%xmm0, %xmm4, %xmm0
	vmaxps	%xmm14, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vxorps	%xmm14, %xmm14, %xmm14
	vmovaps	896(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm4
	vmovaps	1280(%rsp), %xmm5
	vblendvps	%xmm4, 1296(%rsp), %xmm5, %xmm4
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm13, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 896(%rsp)
	vaddps	592(%rsp), %xmm0, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm13
	vmaxps	%xmm13, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vmovaps	944(%rsp), %xmm0
.Ltmp4986:
	vsubps	608(%rsp), %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm3, %xmm11, %xmm3
	vcmpltps	%xmm0, %xmm6, %xmm5
	vblendvps	%xmm5, %xmm0, %xmm3, %xmm3
	vcmpleps	%xmm10, %xmm0, %xmm0
	vmulps	1264(%rsp), %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm14, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm14, %xmm0, %xmm0
	vmovaps	1088(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm0, %xmm3
	vmovaps	1232(%rsp), %xmm1
	vblendvps	%xmm3, 1248(%rsp), %xmm1, %xmm3
	vsubps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm3, %xmm5, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm7
	vaddps	672(%rsp), %xmm7, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm13, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
.Ltmp4987:
	movq	(%rbx), %rcx
	vmovaps	848(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm2
	movq	16(%rbx), %rcx
	vmovaps	160(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm1
	vaddps	%xmm1, %xmm2, %xmm1
.Ltmp4988:
	movq	1328(%rbx), %rcx
	vmulps	(%rcx,%rax,4), %xmm4, %xmm2
	.loc	1 1155 27 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp4989:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
.Ltmp4990:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	976(%rsp), %rcx
.Ltmp4991:
	.loc	8 551 14
	vmovups	%xmm1, (%rcx,%r10,4)
.Ltmp4992:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r9,%r10,4)
	movq	200(%rsp), %rsi
.Ltmp4993:
	.loc	1 0 0
	incq	%rsi
.Ltmp4994:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %r10
	cmpq	%rsi, %r14
.Ltmp4995:
	.loc	3 900 12
	jne	.LBB34_279
.Ltmp4996:
.LBB34_328:
	.loc	3 0 12 is_stmt 0
	vmovaps	32(%rsp), %xmm0
	.loc	1 1160 5 is_stmt 1
	vmovaps	%xmm0, 96(%rbx)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 112(%rbx)
	vmovaps	80(%rsp), %xmm0
	vmovaps	%xmm0, 128(%rbx)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 144(%rbx)
	vmovaps	992(%rsp), %xmm0
	.loc	1 1161 5
	vmovaps	%xmm0, 1424(%rbx)
	vmovaps	912(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%rbx)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%rbx)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%rbx)
	vmovaps	864(%rsp), %xmm0
	.loc	1 1162 5
	vmovaps	%xmm0, 160(%rbx)
	vmovaps	880(%rsp), %xmm0
	vmovaps	%xmm0, 176(%rbx)
	vmovaps	896(%rsp), %xmm0
	.loc	1 1163 5
	vmovaps	%xmm0, 1488(%rbx)
	vmovaps	%xmm7, 1504(%rbx)
	.loc	1 1164 5
	movq	%r12, 2680(%rbx)
	xorl	%eax, %eax
	xorl	%edi, %edi
.Ltmp4997:
	.loc	1 0 5 is_stmt 0
.Ltmp4998:
	.p2align	4
.LBB34_329:
	.loc	1 1297 13 is_stmt 1
	vmovd	208(%rsp,%rax), %xmm0
	vmovss	212(%rsp,%rax), %xmm1
	vmovss	216(%rsp,%rax), %xmm2
	vmovss	220(%rsp,%rax), %xmm3
.Ltmp4999:
	.loc	1 1300 17
	vmovd	%xmm0, 192(%rbx,%rax)
	.loc	1 1301 34
	movl	204(%rbx,%rax), %ecx
	movl	364(%rbx,%rax), %edx
.Ltmp5000:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5001:
	.loc	1 1301 17
	movl	%ecx, 204(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 352(%rbx,%rax)
.Ltmp5002:
	.loc	38 2472 13
	subl	%r14d, %edx
	cmovbl	%edi, %edx
.Ltmp5003:
	.loc	1 1301 17
	movl	%edx, 364(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 512(%rbx,%rax)
	.loc	1 1301 34
	movl	524(%rbx,%rax), %ecx
.Ltmp5004:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5005:
	.loc	1 1301 17
	movl	%ecx, 524(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm3, 672(%rbx,%rax)
	.loc	1 1301 34
	movl	684(%rbx,%rax), %ecx
.Ltmp5006:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5007:
	.loc	1 1301 17
	movl	%ecx, 684(%rbx,%rax)
.Ltmp5008:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp5009:
	.loc	3 900 12
	jne	.LBB34_329
.Ltmp5010:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	984(%rsp), %rsi
	movq	120(%rsp), %r9
	.p2align	4
.LBB34_331:
.Ltmp5011:
	.loc	1 1297 13 is_stmt 1
	vmovd	528(%rsp,%rax), %xmm0
	vmovss	532(%rsp,%rax), %xmm1
	vmovss	536(%rsp,%rax), %xmm2
	vmovss	540(%rsp,%rax), %xmm3
.Ltmp5012:
	.loc	1 1300 17
	vmovd	%xmm0, 1520(%rbx,%rax)
	.loc	1 1301 34
	movl	1532(%rbx,%rax), %ecx
	movl	1692(%rbx,%rax), %edx
.Ltmp5013:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5014:
	.loc	1 1301 17
	movl	%ecx, 1532(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 1680(%rbx,%rax)
.Ltmp5015:
	.loc	38 2472 13
	subl	%r14d, %edx
	cmovbl	%edi, %edx
.Ltmp5016:
	.loc	1 1301 17
	movl	%edx, 1692(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 1840(%rbx,%rax)
	.loc	1 1301 34
	movl	1852(%rbx,%rax), %ecx
.Ltmp5017:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5018:
	.loc	1 1301 17
	movl	%ecx, 1852(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm3, 2000(%rbx,%rax)
	.loc	1 1301 34
	movl	2012(%rbx,%rax), %ecx
.Ltmp5019:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5020:
	.loc	1 1301 17
	movl	%ecx, 2012(%rbx,%rax)
.Ltmp5021:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp5022:
	.loc	3 900 12
	jne	.LBB34_331
	jmp	.LBB34_272
.Ltmp5023:
.LBB34_332:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp5024:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_571
.Ltmp5025:
	.loc	48 451 16 is_stmt 1
	cmpq	112(%rsp), %rsi
	ja	.LBB34_571
.Ltmp5026:
	.loc	48 451 16 is_stmt 0
	cmpq	104(%rsp), %rsi
	ja	.LBB34_576
.Ltmp5027:
	.loc	1 1053 27 is_stmt 1
	vmovaps	96(%rbx), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	112(%rbx), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	128(%rbx), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	144(%rbx), %xmm0
	vmovaps	%xmm0, 64(%rsp)
.Ltmp5028:
	.loc	1 1054 26
	vmovaps	1424(%rbx), %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vmovaps	1440(%rbx), %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmovaps	1456(%rbx), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	1472(%rbx), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5029:
	.loc	1 1055 25
	vmovaps	160(%rbx), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	176(%rbx), %xmm0
	vmovaps	%xmm0, 880(%rsp)
.Ltmp5030:
	.loc	1 1056 24
	vmovdqa	1488(%rbx), %xmm0
	vmovdqa	%xmm0, 896(%rsp)
	vmovaps	1504(%rbx), %xmm7
.Ltmp5031:
	.loc	1 1057 24
	movq	2680(%rbx), %r10
.Ltmp5032:
	.loc	1 871 17
	movq	1104(%rbx), %rax
	movq	1112(%rbx), %rcx
.Ltmp5033:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	1120(%rbx), %rdx
	xorq	%rax, %rdx
	orq	%rcx, %rdx
	xorq	1128(%rbx), %rax
	orq	%rdx, %rax
	sete	15(%rsp)
.Ltmp5034:
	.loc	1 871 17
	movq	2432(%rbx), %rax
	movq	2440(%rbx), %rcx
.Ltmp5035:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2448(%rbx), %rdx
	xorq	%rax, %rdx
	xorq	2456(%rbx), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	192(%rsp)
.Ltmp5036:
	.loc	2 1916 50
	testq	%r14, %r14
	movq	176(%rsp), %rbp
.Ltmp5037:
	.loc	3 900 12
	je	.LBB34_271
.Ltmp5038:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r14,4), %rax
	movq	%rax, 152(%rsp)
	movq	968(%rsp), %rax
	leaq	(%rax,%r15,4), %rcx
	movq	120(%rsp), %rax
	leaq	(%rax,%r15,4), %r9
.Ltmp5039:
	.loc	48 568 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	movq	%r14, 936(%rsp)
	xorl	%edi, %edi
	xorl	%r11d, %r11d
	movq	%rcx, 976(%rsp)
	movq	%r9, 928(%rsp)
.Ltmp5040:
	.loc	48 0 12 is_stmt 0
.Ltmp5041:
	.p2align	4
.LBB34_337:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp5042:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %edx
	cmovaeq	%rbp, %rdx
.Ltmp5043:
	.loc	48 568 12
	cmpq	152(%rsp), %rdi
	ja	.LBB34_563
.Ltmp5044:
	.loc	48 438 16
	cmpq	%r11, 936(%rsp)
	je	.LBB34_534
.Ltmp5045:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,4), %rax
.Ltmp5046:
	.loc	1 1083 29 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp5047:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_540
.Ltmp5048:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5049:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm7, 1088(%rsp)
	vmovups	(%rcx,%rdi,4), %xmm1
.Ltmp5050:
	vmovups	(%r9,%rdi,4), %xmm5
.Ltmp5051:
	vmovaps	32(%rbx), %xmm4
	vmovaps	48(%rbx), %xmm11
	vmovaps	64(%rbx), %xmm0
	vmovaps	1360(%rbx), %xmm12
	vmovaps	1376(%rbx), %xmm9
	vmovaps	1392(%rbx), %xmm7
	vmovaps	16(%rsp), %xmm8
	vsubps	%xmm8, %xmm1, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vmovaps	32(%rsp), %xmm6
	vmovaps	%xmm4, 848(%rsp)
	vmulps	%xmm4, %xmm6, %xmm4
	vaddps	%xmm3, %xmm4, %xmm14
	vaddps	%xmm6, %xmm14, %xmm3
	vmulps	%xmm6, %xmm11, %xmm4
	vmulps	%xmm0, %xmm2, %xmm2
	vaddps	%xmm2, %xmm4, %xmm13
	vaddps	%xmm13, %xmm8, %xmm2
	vmulps	80(%rbx), %xmm3, %xmm15
	vmovaps	64(%rsp), %xmm3
	vsubps	%xmm3, %xmm2, %xmm4
	vmulps	80(%rsp), %xmm11, %xmm2
	vmovaps	%xmm4, 944(%rsp)
	vmulps	%xmm4, %xmm0, %xmm0
	vaddps	%xmm0, %xmm2, %xmm2
	vaddps	%xmm2, %xmm3, %xmm4
	vmovaps	912(%rsp), %xmm10
.Ltmp5052:
	vsubps	%xmm10, %xmm5, %xmm3
	vmulps	%xmm3, %xmm9, %xmm0
	vmovaps	992(%rsp), %xmm8
	vmovaps	%xmm12, 160(%rsp)
	vmulps	%xmm12, %xmm8, %xmm6
	vaddps	%xmm0, %xmm6, %xmm12
	vaddps	%xmm12, %xmm8, %xmm0
	vmulps	1408(%rbx), %xmm0, %xmm0
.Ltmp5053:
	.loc	1 1083 29 is_stmt 1
	movq	(%rbx), %rcx
.Ltmp5054:
	.loc	8 551 14
	vmovups	%xmm4, (%rcx,%rax,4)
.Ltmp5055:
	.loc	1 1084 30
	movq	24(%rbx), %rsi
.Ltmp5056:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_541
.Ltmp5057:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5058:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm1, %xmm15, %xmm1
	vsubps	%xmm4, %xmm1, %xmm1
.Ltmp5059:
	.loc	1 1084 30 is_stmt 1
	movq	16(%rbx), %rcx
.Ltmp5060:
	.loc	8 551 14
	vmovups	%xmm1, (%rcx,%rax,4)
.Ltmp5061:
	.loc	1 1085 28
	movq	1336(%rbx), %rsi
.Ltmp5062:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_542
.Ltmp5063:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5064:
	.loc	1 0 0 is_stmt 0
	vmulps	%xmm9, %xmm8, %xmm1
	vmulps	%xmm7, %xmm3, %xmm3
	vaddps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm10, %xmm3
	vmovaps	128(%rsp), %xmm6
	vsubps	%xmm6, %xmm3, %xmm3
	vmulps	48(%rsp), %xmm9, %xmm4
	vmulps	%xmm3, %xmm7, %xmm7
	vaddps	%xmm7, %xmm4, %xmm15
	vaddps	%xmm6, %xmm15, %xmm4
.Ltmp5065:
	.loc	1 1085 28 is_stmt 1
	movq	1328(%rbx), %rcx
.Ltmp5066:
	.loc	8 551 14
	vmovups	%xmm4, (%rcx,%rax,4)
.Ltmp5067:
	.loc	1 1086 29
	movq	1352(%rbx), %rsi
.Ltmp5068:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_543
.Ltmp5069:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5070:
	.loc	48 0 16 is_stmt 0
	movq	%rdi, 1008(%rsp)
	vaddps	%xmm0, %xmm5, %xmm0
	vsubps	%xmm4, %xmm0, %xmm0
.Ltmp5071:
	.loc	1 1086 29 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp5072:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5073:
	.loc	1 1089 13
	movq	(%rbx), %rdi
	movq	8(%rbx), %rsi
	movq	1104(%rbx), %rax
.Ltmp5074:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp5075:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp5076:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 15(%rsp)
	movq	%r11, 200(%rsp)
	je	.LBB34_353
.Ltmp5077:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp5078:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5079:
	.loc	1 1096 13
	movq	24(%rbx), %rsi
.Ltmp5080:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp5081:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp5082:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp5083:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5084:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%rdi,%r9,4), %xmm7
.Ltmp5085:
	movq	16(%rbx), %rcx
.Ltmp5086:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm5
.Ltmp5087:
	.loc	1 961 2
	jmp	.LBB34_362
.Ltmp5088:
	.loc	1 0 2 is_stmt 0
.Ltmp5089:
	.p2align	4
.LBB34_353:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	1112(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp5090:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp5091:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_582
	.loc	1 0 25
	movq	1120(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp5092:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp5093:
	.loc	1 955 30
	leaq	2(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
	.loc	1 0 25
	movq	1128(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp5094:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp5095:
	.loc	1 955 30
	leaq	3(,%rbx,4), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_609
.Ltmp5096:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp5097:
	.loc	1 1096 13
	movq	24(%r13), %rsi
.Ltmp5098:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp5099:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	1112(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp5100:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp5101:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_580
	.loc	1 0 25
	movq	1120(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp5102:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%r10, %r11
	movq	%rbp, %r10
	movl	$0, %ebp
	cmovaeq	%r10, %rbp
	subq	%rbp, %rbx
.Ltmp5103:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	1128(%r13), %rbx
	movq	%r11, %r14
	.loc	1 955 35
	addq	%r11, %rbx
.Ltmp5104:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r11d
	cmovaeq	%r10, %r11
	subq	%r11, %rbx
.Ltmp5105:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp5106:
	.loc	1 0 0
	vmovd	(%rdi,%r9,4), %xmm0
	vpinsrd	$1, (%rdi,%r8,4), %xmm0, %xmm0
	vpinsrd	$2, (%rdi,%rcx,4), %xmm0, %xmm0
	vpinsrd	$3, (%rdi,%r12,4), %xmm0, %xmm7
.Ltmp5107:
	movq	16(%r13), %rcx
.Ltmp5108:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm5
	movq	%r13, %rbx
	movq	176(%rsp), %rbp
	movq	%r14, %r10
.Ltmp5109:
.LBB34_362:
	.loc	1 1103 13
	movq	1328(%rbx), %r14
	movq	1336(%rbx), %rsi
	movq	2432(%rbx), %rax
.Ltmp5110:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp5111:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp5112:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 192(%rsp)
	je	.LBB34_368
.Ltmp5113:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp5114:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5115:
	.loc	1 1110 13
	movq	1352(%rbx), %rsi
.Ltmp5116:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp5117:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp5118:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp5119:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5120:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r14,%r9,4), %xmm6
.Ltmp5121:
	movq	1344(%rbx), %rcx
.Ltmp5122:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm0
.Ltmp5123:
	.loc	1 961 2
	jmp	.LBB34_377
.Ltmp5124:
	.loc	1 0 2 is_stmt 0
.Ltmp5125:
	.p2align	4
.LBB34_368:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	2440(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp5126:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp5127:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_582
	.loc	1 0 25
	movq	2448(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp5128:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp5129:
	.loc	1 955 30
	leaq	2(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
	.loc	1 0 25
	movq	2456(%rbx), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp5130:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movq	%rbx, %r15
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
.Ltmp5131:
	.loc	1 955 30
	leaq	3(,%r11,4), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_609
.Ltmp5132:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
.Ltmp5133:
	.loc	1 1110 13
	movq	1352(%r15), %rsi
.Ltmp5134:
	.loc	1 857 8
	subq	%r11, %rax
.Ltmp5135:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	2440(%r15), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp5136:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movq	%r15, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
.Ltmp5137:
	.loc	1 955 30
	leaq	1(,%r11,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_580
	.loc	1 0 25
	movq	2448(%r13), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp5138:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
	movq	%r10, %rbx
	movq	%rbp, %r10
.Ltmp5139:
	.loc	1 955 30
	leaq	2(,%r11,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	2456(%r13), %r11
	movq	%rbx, %rdi
	.loc	1 955 35
	addq	%rbx, %r11
.Ltmp5140:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %r11
	movl	$0, %ebx
	cmovaeq	%r10, %rbx
	subq	%rbx, %r11
.Ltmp5141:
	.loc	1 955 30
	leaq	3(,%r11,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp5142:
	.loc	1 0 0
	vmovd	(%r14,%r9,4), %xmm0
	vpinsrd	$1, (%r14,%r8,4), %xmm0, %xmm0
	vpinsrd	$2, (%r14,%rcx,4), %xmm0, %xmm0
	vpinsrd	$3, (%r14,%r12,4), %xmm0, %xmm6
.Ltmp5143:
	movq	1344(%r13), %rcx
.Ltmp5144:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm0
	movq	%r13, %rbx
	movq	176(%rsp), %rbp
	movq	%rdi, %r10
.Ltmp5145:
.LBB34_377:
	.loc	1 0 0 is_stmt 0
	negq	%rdx
	addq	%rdx, %r10
	incq	%r10
	leaq	(,%r10,4), %rax
.Ltmp5146:
	.loc	1 1150 36 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp5147:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	1024(%rsp), %rdx
	movq	928(%rsp), %r9
	movq	1008(%rsp), %rdi
	movq	200(%rsp), %r11
	jb	.LBB34_546
.Ltmp5148:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5149:
	.loc	1 1152 27
	movq	24(%rbx), %rsi
.Ltmp5150:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_547
.Ltmp5151:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5152:
	.loc	1 1153 35
	movq	1336(%rbx), %rsi
.Ltmp5153:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_548
.Ltmp5154:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5155:
	.loc	1 1155 27
	movq	1352(%rbx), %rsi
.Ltmp5156:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_550
.Ltmp5157:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5158:
	.loc	48 0 16 is_stmt 0
	vmovdqa	%xmm0, 1056(%rsp)
	vaddps	%xmm14, %xmm14, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm14
	vmovdqa	%xmm5, 1040(%rsp)
	vandps	%xmm0, %xmm14, %xmm4
	vmovdqa	%xmm6, %xmm5
	vmovaps	%xmm10, %xmm6
	vmovaps	%xmm8, %xmm10
	vbroadcastss	.LCPI34_2(%rip), %xmm8
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm4
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmulps	944(%rsp), %xmm11, %xmm0
	vmovaps	80(%rsp), %xmm11
	vmulps	848(%rsp), %xmm11, %xmm4
	vaddps	%xmm0, %xmm4, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm11, %xmm0
	vandps	%xmm0, %xmm14, %xmm4
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vaddps	%xmm2, %xmm2, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 64(%rsp)
.Ltmp5159:
	vaddps	%xmm12, %xmm12, %xmm0
	vaddps	%xmm0, %xmm10, %xmm0
	vandps	%xmm0, %xmm14, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vaddps	%xmm1, %xmm1, %xmm0
	vaddps	%xmm0, %xmm6, %xmm0
	vandps	%xmm0, %xmm14, %xmm1
	vcmpltps	%xmm8, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmulps	%xmm3, %xmm9, %xmm0
	vmovaps	48(%rsp), %xmm2
	vmulps	160(%rsp), %xmm2, %xmm1
	vaddps	%xmm0, %xmm1, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm2, %xmm0
	vandps	%xmm0, %xmm14, %xmm1
	vcmpltps	%xmm8, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm15, %xmm15, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm1
	vcmpltps	%xmm8, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5160:
	vpand	%xmm7, %xmm14, %xmm0
	vpand	%xmm5, %xmm14, %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
.Ltmp5161:
	vandps	1040(%rsp), %xmm14, %xmm1
	vandps	1056(%rsp), %xmm14, %xmm2
	vmaxps	%xmm2, %xmm1, %xmm2
	vbroadcastss	.LCPI34_4(%rip), %xmm5
.Ltmp5162:
	vmaxps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm6
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm7
	vandps	%xmm7, %xmm0, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm9
	vorps	%xmm1, %xmm9, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm10
	vaddps	%xmm1, %xmm10, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm1, %xmm11, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm12
	vaddps	%xmm3, %xmm12, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vsubps	208(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm0, %xmm1, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm7
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1216(%rsp), %xmm3, %xmm3
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm0, %xmm1, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	864(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm3
	vmovaps	1184(%rsp), %xmm4
	vblendvps	%xmm3, 1200(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm1, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm3
	vcmpltps	%xmm8, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm1
.Ltmp5163:
	vbroadcastss	.LCPI34_4(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm0
	vmaxps	%xmm6, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm2
	vorps	%xmm2, %xmm9, %xmm2
	vaddps	%xmm2, %xmm10, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vaddps	%xmm3, %xmm12, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 944(%rsp)
	vsubps	288(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm3
	vaddps	%xmm3, %xmm0, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vmulps	%xmm5, %xmm2, %xmm2
	vcmpltps	%xmm0, %xmm3, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm2, %xmm2
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1168(%rsp), %xmm2, %xmm2
	vxorps	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm3, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm1, 864(%rsp)
.Ltmp5164:
	vaddps	272(%rsp), %xmm1, %xmm2
	vbroadcastss	.LCPI34_24(%rip), %xmm10
	vmulps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_25(%rip), %xmm11
	vmaxps	%xmm11, %xmm2, %xmm2
	vbroadcastss	.LCPI34_26(%rip), %xmm11
	vminps	%xmm11, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm4
	vsubps	%xmm4, %xmm2, %xmm2
	vbroadcastss	.LCPI34_27(%rip), %xmm13
	vmulps	%xmm2, %xmm13, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm7
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm8
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm9
	vaddps	%xmm5, %xmm9, %xmm5
	vbroadcastss	.LCPI34_23(%rip), %xmm13
.Ltmp5165:
	vmaxps	%xmm13, %xmm0, %xmm0
	vminps	%xmm3, %xmm0, %xmm0
	vmovaps	880(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm6
	vmovaps	1136(%rsp), %xmm10
	vblendvps	%xmm6, 1152(%rsp), %xmm10, %xmm6
.Ltmp5166:
	vmulps	%xmm5, %xmm2, %xmm2
.Ltmp5167:
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm6, %xmm5, %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm5
	vbroadcastss	.LCPI34_2(%rip), %xmm12
	vcmpltps	%xmm12, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm6
.Ltmp5168:
	vaddps	%xmm6, %xmm2, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm10
	vaddps	%xmm4, %xmm10, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vmovaps	%xmm1, 880(%rsp)
.Ltmp5169:
	vaddps	352(%rsp), %xmm1, %xmm4
.Ltmp5170:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vbroadcastss	.LCPI34_24(%rip), %xmm2
.Ltmp5171:
	vmulps	%xmm2, %xmm4, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm0
	vaddps	%xmm4, %xmm10, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmovaps	160(%rsp), %xmm1
.Ltmp5172:
	vsubps	528(%rsp), %xmm1, %xmm5
.Ltmp5173:
	vmulps	%xmm4, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm6
.Ltmp5174:
	vaddps	%xmm6, %xmm5, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm11
	vmulps	%xmm0, %xmm11, %xmm0
	vcmpltps	%xmm5, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm10
	vcmpleps	%xmm10, %xmm5, %xmm4
	vmulps	1120(%rsp), %xmm0, %xmm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm4, %xmm1, %xmm4
	vpandn	%xmm0, %xmm4, %xmm0
	vmaxps	%xmm13, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vxorps	%xmm13, %xmm13, %xmm13
	vmovaps	896(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm4
	vmovaps	1280(%rsp), %xmm5
	vblendvps	%xmm4, 1296(%rsp), %xmm5, %xmm4
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm4
	vcmpltps	%xmm12, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 896(%rsp)
	vaddps	592(%rsp), %xmm0, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm12
	vmaxps	%xmm12, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm5
	vmovaps	944(%rsp), %xmm0
.Ltmp5175:
	vsubps	608(%rsp), %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm3, %xmm11, %xmm3
	vcmpltps	%xmm0, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vcmpleps	%xmm10, %xmm0, %xmm0
	vmulps	1264(%rsp), %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm13, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm13, %xmm0, %xmm0
	vmovaps	1088(%rsp), %xmm4
	vcmpltps	%xmm4, %xmm0, %xmm3
	vmovaps	1232(%rsp), %xmm1
	vblendvps	%xmm3, 1248(%rsp), %xmm1, %xmm3
	vsubps	%xmm0, %xmm4, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm3
	vbroadcastss	.LCPI34_2(%rip), %xmm1
	vcmpltps	%xmm1, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm7
	vaddps	672(%rsp), %xmm7, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm12, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm4
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_29(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm8, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm9, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_32(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm1
	vaddps	%xmm1, %xmm3, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
.Ltmp5176:
	movq	(%rbx), %rcx
	vmovaps	848(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm2
	movq	16(%rbx), %rcx
	vmovaps	160(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm1
	vaddps	%xmm1, %xmm2, %xmm1
.Ltmp5177:
	movq	1328(%rbx), %rcx
	vmulps	(%rcx,%rax,4), %xmm5, %xmm2
	.loc	1 1155 27 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp5178:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
.Ltmp5179:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	976(%rsp), %rcx
.Ltmp5180:
	.loc	8 551 14
	vmovups	%xmm1, (%rcx,%rdi,4)
.Ltmp5181:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r9,%rdi,4)
.Ltmp5182:
	.loc	1 0 0
	incq	%r11
.Ltmp5183:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	cmpq	%r11, %rdx
.Ltmp5184:
	.loc	3 900 12
	jne	.LBB34_337
	jmp	.LBB34_271
.Ltmp5185:
.LBB34_386:
	.loc	1 1189 11
	testq	%rsi, %rsi
	je	.LBB34_503
	.loc	1 0 11 is_stmt 0
	movl	2688(%rbx), %eax
	movl	%eax, 1104(%rsp)
	movq	2672(%rbx), %rbp
	leaq	1328(%rbx), %rax
	movq	%rax, 1312(%rsp)
	xorl	%r15d, %r15d
	movq	%rbp, 176(%rsp)
	jmp	.LBB34_390
.LBB34_388:
	vmovaps	32(%rsp), %xmm0
.Ltmp5186:
	.loc	1 1160 5 is_stmt 1
	vmovaps	%xmm0, 96(%rbx)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 112(%rbx)
	vmovaps	80(%rsp), %xmm0
	vmovaps	%xmm0, 128(%rbx)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 144(%rbx)
	vmovaps	992(%rsp), %xmm0
	.loc	1 1161 5
	vmovaps	%xmm0, 1424(%rbx)
	vmovaps	912(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%rbx)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%rbx)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%rbx)
	vmovaps	864(%rsp), %xmm0
	.loc	1 1162 5
	vmovaps	%xmm0, 160(%rbx)
	vmovaps	880(%rsp), %xmm0
	vmovaps	%xmm0, 176(%rbx)
	vmovdqa	896(%rsp), %xmm0
	.loc	1 1163 5
	vmovdqa	%xmm0, 1488(%rbx)
	vmovaps	%xmm7, 1504(%rbx)
	.loc	1 1164 5
	movq	%r10, 2680(%rbx)
	movq	984(%rsp), %rsi
	movq	120(%rsp), %r9
.Ltmp5187:
.LBB34_389:
	.loc	1 0 5 is_stmt 0
	movq	1112(%rsp), %r15
	.loc	1 1189 11 is_stmt 1
	cmpq	%rsi, %r15
	jae	.LBB34_503
.LBB34_390:
	.loc	1 1190 42
	subq	%r15, %rsi
	.loc	1 1190 29 is_stmt 0
	movq	%rbx, %rdi
	vzeroupper
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r14
.Ltmp5188:
	.loc	1 1279 33 is_stmt 1
	vmovss	192(%rbx), %xmm0
.Ltmp5189:
	.loc	1 1192 28
	vmovss	%xmm0, 208(%rsp)
.Ltmp5190:
	.loc	1 1279 33
	vmovss	352(%rbx), %xmm0
.Ltmp5191:
	.loc	1 1192 28
	vmovss	%xmm0, 212(%rsp)
.Ltmp5192:
	.loc	1 1279 33
	vmovss	512(%rbx), %xmm0
.Ltmp5193:
	.loc	1 1192 28
	vmovss	%xmm0, 216(%rsp)
.Ltmp5194:
	.loc	1 1279 33
	vmovss	672(%rbx), %xmm0
.Ltmp5195:
	.loc	1 1192 28
	vmovss	%xmm0, 220(%rsp)
.Ltmp5196:
	.loc	1 1279 33
	vmovss	208(%rbx), %xmm0
.Ltmp5197:
	.loc	1 1192 28
	vmovss	%xmm0, 224(%rsp)
.Ltmp5198:
	.loc	1 1279 33
	vmovss	368(%rbx), %xmm0
.Ltmp5199:
	.loc	1 1192 28
	vmovss	%xmm0, 228(%rsp)
.Ltmp5200:
	.loc	1 1279 33
	vmovss	528(%rbx), %xmm0
.Ltmp5201:
	.loc	1 1192 28
	vmovss	%xmm0, 232(%rsp)
.Ltmp5202:
	.loc	1 1279 33
	vmovss	688(%rbx), %xmm0
.Ltmp5203:
	.loc	1 1192 28
	vmovss	%xmm0, 236(%rsp)
.Ltmp5204:
	.loc	1 1279 33
	vmovss	224(%rbx), %xmm0
.Ltmp5205:
	.loc	1 1192 28
	vmovss	%xmm0, 240(%rsp)
.Ltmp5206:
	.loc	1 1279 33
	vmovss	384(%rbx), %xmm0
.Ltmp5207:
	.loc	1 1192 28
	vmovss	%xmm0, 244(%rsp)
.Ltmp5208:
	.loc	1 1279 33
	vmovss	544(%rbx), %xmm0
.Ltmp5209:
	.loc	1 1192 28
	vmovss	%xmm0, 248(%rsp)
.Ltmp5210:
	.loc	1 1279 33
	vmovss	704(%rbx), %xmm0
.Ltmp5211:
	.loc	1 1192 28
	vmovss	%xmm0, 252(%rsp)
.Ltmp5212:
	.loc	1 1279 33
	vmovss	240(%rbx), %xmm0
.Ltmp5213:
	.loc	1 1192 28
	vmovss	%xmm0, 256(%rsp)
.Ltmp5214:
	.loc	1 1279 33
	vmovss	400(%rbx), %xmm0
.Ltmp5215:
	.loc	1 1192 28
	vmovss	%xmm0, 260(%rsp)
.Ltmp5216:
	.loc	1 1279 33
	vmovss	560(%rbx), %xmm0
.Ltmp5217:
	.loc	1 1192 28
	vmovss	%xmm0, 264(%rsp)
.Ltmp5218:
	.loc	1 1279 33
	vmovss	720(%rbx), %xmm0
.Ltmp5219:
	.loc	1 1192 28
	vmovss	%xmm0, 268(%rsp)
.Ltmp5220:
	.loc	1 1279 33
	vmovss	256(%rbx), %xmm0
.Ltmp5221:
	.loc	1 1192 28
	vmovss	%xmm0, 272(%rsp)
.Ltmp5222:
	.loc	1 1279 33
	vmovss	416(%rbx), %xmm0
.Ltmp5223:
	.loc	1 1192 28
	vmovss	%xmm0, 276(%rsp)
.Ltmp5224:
	.loc	1 1279 33
	vmovss	576(%rbx), %xmm0
.Ltmp5225:
	.loc	1 1192 28
	vmovss	%xmm0, 280(%rsp)
.Ltmp5226:
	.loc	1 1279 33
	vmovss	736(%rbx), %xmm0
.Ltmp5227:
	.loc	1 1192 28
	vmovss	%xmm0, 284(%rsp)
.Ltmp5228:
	.loc	1 1279 33
	vmovss	272(%rbx), %xmm0
.Ltmp5229:
	.loc	1 1192 28
	vmovss	%xmm0, 288(%rsp)
.Ltmp5230:
	.loc	1 1279 33
	vmovss	432(%rbx), %xmm0
.Ltmp5231:
	.loc	1 1192 28
	vmovss	%xmm0, 292(%rsp)
.Ltmp5232:
	.loc	1 1279 33
	vmovss	592(%rbx), %xmm0
.Ltmp5233:
	.loc	1 1192 28
	vmovss	%xmm0, 296(%rsp)
.Ltmp5234:
	.loc	1 1279 33
	vmovss	752(%rbx), %xmm0
.Ltmp5235:
	.loc	1 1192 28
	vmovss	%xmm0, 300(%rsp)
.Ltmp5236:
	.loc	1 1279 33
	vmovss	288(%rbx), %xmm0
.Ltmp5237:
	.loc	1 1192 28
	vmovss	%xmm0, 304(%rsp)
.Ltmp5238:
	.loc	1 1279 33
	vmovss	448(%rbx), %xmm0
.Ltmp5239:
	.loc	1 1192 28
	vmovss	%xmm0, 308(%rsp)
.Ltmp5240:
	.loc	1 1279 33
	vmovss	608(%rbx), %xmm0
.Ltmp5241:
	.loc	1 1192 28
	vmovss	%xmm0, 312(%rsp)
.Ltmp5242:
	.loc	1 1279 33
	vmovss	768(%rbx), %xmm0
.Ltmp5243:
	.loc	1 1192 28
	vmovss	%xmm0, 316(%rsp)
.Ltmp5244:
	.loc	1 1279 33
	vmovss	304(%rbx), %xmm0
.Ltmp5245:
	.loc	1 1192 28
	vmovss	%xmm0, 320(%rsp)
.Ltmp5246:
	.loc	1 1279 33
	vmovss	464(%rbx), %xmm0
.Ltmp5247:
	.loc	1 1192 28
	vmovss	%xmm0, 324(%rsp)
.Ltmp5248:
	.loc	1 1279 33
	vmovss	624(%rbx), %xmm0
.Ltmp5249:
	.loc	1 1192 28
	vmovss	%xmm0, 328(%rsp)
.Ltmp5250:
	.loc	1 1279 33
	vmovss	784(%rbx), %xmm0
.Ltmp5251:
	.loc	1 1192 28
	vmovss	%xmm0, 332(%rsp)
.Ltmp5252:
	.loc	1 1279 33
	vmovss	320(%rbx), %xmm0
.Ltmp5253:
	.loc	1 1192 28
	vmovss	%xmm0, 336(%rsp)
.Ltmp5254:
	.loc	1 1279 33
	vmovss	480(%rbx), %xmm0
.Ltmp5255:
	.loc	1 1192 28
	vmovss	%xmm0, 340(%rsp)
.Ltmp5256:
	.loc	1 1279 33
	vmovss	640(%rbx), %xmm0
.Ltmp5257:
	.loc	1 1192 28
	vmovss	%xmm0, 344(%rsp)
.Ltmp5258:
	.loc	1 1279 33
	vmovss	800(%rbx), %xmm0
.Ltmp5259:
	.loc	1 1192 28
	vmovss	%xmm0, 348(%rsp)
.Ltmp5260:
	.loc	1 1279 33
	vmovss	336(%rbx), %xmm0
.Ltmp5261:
	.loc	1 1192 28
	vmovss	%xmm0, 352(%rsp)
.Ltmp5262:
	.loc	1 1279 33
	vmovss	496(%rbx), %xmm0
.Ltmp5263:
	.loc	1 1192 28
	vmovss	%xmm0, 356(%rsp)
.Ltmp5264:
	.loc	1 1279 33
	vmovss	656(%rbx), %xmm0
.Ltmp5265:
	.loc	1 1192 28
	vmovss	%xmm0, 360(%rsp)
.Ltmp5266:
	.loc	1 1279 33
	vmovss	816(%rbx), %xmm0
.Ltmp5267:
	.loc	1 1192 28
	vmovss	%xmm0, 364(%rsp)
.Ltmp5268:
	.loc	1 1280 32
	vmovss	200(%rbx), %xmm0
.Ltmp5269:
	.loc	1 1192 28
	vmovss	%xmm0, 368(%rsp)
.Ltmp5270:
	.loc	1 1280 32
	vmovss	360(%rbx), %xmm0
.Ltmp5271:
	.loc	1 1192 28
	vmovss	%xmm0, 372(%rsp)
.Ltmp5272:
	.loc	1 1280 32
	vmovss	520(%rbx), %xmm0
.Ltmp5273:
	.loc	1 1192 28
	vmovss	%xmm0, 376(%rsp)
.Ltmp5274:
	.loc	1 1280 32
	vmovss	680(%rbx), %xmm0
.Ltmp5275:
	.loc	1 1192 28
	vmovss	%xmm0, 380(%rsp)
.Ltmp5276:
	.loc	1 1280 32
	vmovss	216(%rbx), %xmm0
.Ltmp5277:
	.loc	1 1192 28
	vmovss	%xmm0, 384(%rsp)
.Ltmp5278:
	.loc	1 1280 32
	vmovss	376(%rbx), %xmm0
.Ltmp5279:
	.loc	1 1192 28
	vmovss	%xmm0, 388(%rsp)
.Ltmp5280:
	.loc	1 1280 32
	vmovss	536(%rbx), %xmm0
.Ltmp5281:
	.loc	1 1192 28
	vmovss	%xmm0, 392(%rsp)
.Ltmp5282:
	.loc	1 1280 32
	vmovss	696(%rbx), %xmm0
.Ltmp5283:
	.loc	1 1192 28
	vmovss	%xmm0, 396(%rsp)
.Ltmp5284:
	.loc	1 1280 32
	vmovss	232(%rbx), %xmm0
.Ltmp5285:
	.loc	1 1192 28
	vmovss	%xmm0, 400(%rsp)
.Ltmp5286:
	.loc	1 1280 32
	vmovss	392(%rbx), %xmm0
.Ltmp5287:
	.loc	1 1192 28
	vmovss	%xmm0, 404(%rsp)
.Ltmp5288:
	.loc	1 1280 32
	vmovss	552(%rbx), %xmm0
.Ltmp5289:
	.loc	1 1192 28
	vmovss	%xmm0, 408(%rsp)
.Ltmp5290:
	.loc	1 1280 32
	vmovss	712(%rbx), %xmm0
.Ltmp5291:
	.loc	1 1192 28
	vmovss	%xmm0, 412(%rsp)
.Ltmp5292:
	.loc	1 1280 32
	vmovss	248(%rbx), %xmm0
.Ltmp5293:
	.loc	1 1192 28
	vmovss	%xmm0, 416(%rsp)
.Ltmp5294:
	.loc	1 1280 32
	vmovss	408(%rbx), %xmm0
.Ltmp5295:
	.loc	1 1192 28
	vmovss	%xmm0, 420(%rsp)
.Ltmp5296:
	.loc	1 1280 32
	vmovss	568(%rbx), %xmm0
.Ltmp5297:
	.loc	1 1192 28
	vmovss	%xmm0, 424(%rsp)
.Ltmp5298:
	.loc	1 1280 32
	vmovss	728(%rbx), %xmm0
.Ltmp5299:
	.loc	1 1192 28
	vmovss	%xmm0, 428(%rsp)
.Ltmp5300:
	.loc	1 1280 32
	vmovss	264(%rbx), %xmm0
.Ltmp5301:
	.loc	1 1192 28
	vmovss	%xmm0, 432(%rsp)
.Ltmp5302:
	.loc	1 1280 32
	vmovss	424(%rbx), %xmm0
.Ltmp5303:
	.loc	1 1192 28
	vmovss	%xmm0, 436(%rsp)
.Ltmp5304:
	.loc	1 1280 32
	vmovss	584(%rbx), %xmm0
.Ltmp5305:
	.loc	1 1192 28
	vmovss	%xmm0, 440(%rsp)
.Ltmp5306:
	.loc	1 1280 32
	vmovss	744(%rbx), %xmm0
.Ltmp5307:
	.loc	1 1192 28
	vmovss	%xmm0, 444(%rsp)
.Ltmp5308:
	.loc	1 1280 32
	vmovss	280(%rbx), %xmm0
.Ltmp5309:
	.loc	1 1192 28
	vmovss	%xmm0, 448(%rsp)
.Ltmp5310:
	.loc	1 1280 32
	vmovss	440(%rbx), %xmm0
.Ltmp5311:
	.loc	1 1192 28
	vmovss	%xmm0, 452(%rsp)
.Ltmp5312:
	.loc	1 1280 32
	vmovss	600(%rbx), %xmm0
.Ltmp5313:
	.loc	1 1192 28
	vmovss	%xmm0, 456(%rsp)
.Ltmp5314:
	.loc	1 1280 32
	vmovss	760(%rbx), %xmm0
.Ltmp5315:
	.loc	1 1192 28
	vmovss	%xmm0, 460(%rsp)
.Ltmp5316:
	.loc	1 1280 32
	vmovss	296(%rbx), %xmm0
.Ltmp5317:
	.loc	1 1192 28
	vmovss	%xmm0, 464(%rsp)
.Ltmp5318:
	.loc	1 1280 32
	vmovss	456(%rbx), %xmm0
.Ltmp5319:
	.loc	1 1192 28
	vmovss	%xmm0, 468(%rsp)
.Ltmp5320:
	.loc	1 1280 32
	vmovss	616(%rbx), %xmm0
.Ltmp5321:
	.loc	1 1192 28
	vmovss	%xmm0, 472(%rsp)
.Ltmp5322:
	.loc	1 1280 32
	vmovss	776(%rbx), %xmm0
.Ltmp5323:
	.loc	1 1192 28
	vmovss	%xmm0, 476(%rsp)
.Ltmp5324:
	.loc	1 1280 32
	vmovss	312(%rbx), %xmm0
.Ltmp5325:
	.loc	1 1192 28
	vmovss	%xmm0, 480(%rsp)
.Ltmp5326:
	.loc	1 1280 32
	vmovss	472(%rbx), %xmm0
.Ltmp5327:
	.loc	1 1192 28
	vmovss	%xmm0, 484(%rsp)
.Ltmp5328:
	.loc	1 1280 32
	vmovss	632(%rbx), %xmm0
.Ltmp5329:
	.loc	1 1192 28
	vmovss	%xmm0, 488(%rsp)
.Ltmp5330:
	.loc	1 1280 32
	vmovss	792(%rbx), %xmm0
.Ltmp5331:
	.loc	1 1192 28
	vmovss	%xmm0, 492(%rsp)
.Ltmp5332:
	.loc	1 1280 32
	vmovss	328(%rbx), %xmm0
.Ltmp5333:
	.loc	1 1192 28
	vmovss	%xmm0, 496(%rsp)
.Ltmp5334:
	.loc	1 1280 32
	vmovss	488(%rbx), %xmm0
.Ltmp5335:
	.loc	1 1192 28
	vmovss	%xmm0, 500(%rsp)
.Ltmp5336:
	.loc	1 1280 32
	vmovss	648(%rbx), %xmm0
.Ltmp5337:
	.loc	1 1192 28
	vmovss	%xmm0, 504(%rsp)
.Ltmp5338:
	.loc	1 1280 32
	vmovss	808(%rbx), %xmm0
.Ltmp5339:
	.loc	1 1192 28
	vmovss	%xmm0, 508(%rsp)
.Ltmp5340:
	.loc	1 1280 32
	vmovss	344(%rbx), %xmm0
.Ltmp5341:
	.loc	1 1192 28
	vmovss	%xmm0, 512(%rsp)
.Ltmp5342:
	.loc	1 1280 32
	vmovss	504(%rbx), %xmm0
.Ltmp5343:
	.loc	1 1192 28
	vmovss	%xmm0, 516(%rsp)
.Ltmp5344:
	.loc	1 1280 32
	vmovss	664(%rbx), %xmm0
.Ltmp5345:
	.loc	1 1192 28
	vmovss	%xmm0, 520(%rsp)
.Ltmp5346:
	.loc	1 1280 32
	vmovss	824(%rbx), %xmm0
.Ltmp5347:
	.loc	1 1192 28
	vmovss	%xmm0, 524(%rsp)
.Ltmp5348:
	.loc	1 1279 33
	vmovss	1520(%rbx), %xmm0
.Ltmp5349:
	.loc	1 1192 28
	vmovss	%xmm0, 528(%rsp)
.Ltmp5350:
	.loc	1 1279 33
	vmovss	1680(%rbx), %xmm0
.Ltmp5351:
	.loc	1 1192 28
	vmovss	%xmm0, 532(%rsp)
.Ltmp5352:
	.loc	1 1279 33
	vmovss	1840(%rbx), %xmm0
.Ltmp5353:
	.loc	1 1192 28
	vmovss	%xmm0, 536(%rsp)
.Ltmp5354:
	.loc	1 1279 33
	vmovss	2000(%rbx), %xmm0
.Ltmp5355:
	.loc	1 1192 28
	vmovss	%xmm0, 540(%rsp)
.Ltmp5356:
	.loc	1 1279 33
	vmovss	1536(%rbx), %xmm0
.Ltmp5357:
	.loc	1 1192 28
	vmovss	%xmm0, 544(%rsp)
.Ltmp5358:
	.loc	1 1279 33
	vmovss	1696(%rbx), %xmm0
.Ltmp5359:
	.loc	1 1192 28
	vmovss	%xmm0, 548(%rsp)
.Ltmp5360:
	.loc	1 1279 33
	vmovss	1856(%rbx), %xmm0
.Ltmp5361:
	.loc	1 1192 28
	vmovss	%xmm0, 552(%rsp)
.Ltmp5362:
	.loc	1 1279 33
	vmovss	2016(%rbx), %xmm0
.Ltmp5363:
	.loc	1 1192 28
	vmovss	%xmm0, 556(%rsp)
.Ltmp5364:
	.loc	1 1279 33
	vmovss	1552(%rbx), %xmm0
.Ltmp5365:
	.loc	1 1192 28
	vmovss	%xmm0, 560(%rsp)
.Ltmp5366:
	.loc	1 1279 33
	vmovss	1712(%rbx), %xmm0
.Ltmp5367:
	.loc	1 1192 28
	vmovss	%xmm0, 564(%rsp)
.Ltmp5368:
	.loc	1 1279 33
	vmovss	1872(%rbx), %xmm0
.Ltmp5369:
	.loc	1 1192 28
	vmovss	%xmm0, 568(%rsp)
.Ltmp5370:
	.loc	1 1279 33
	vmovss	2032(%rbx), %xmm0
.Ltmp5371:
	.loc	1 1192 28
	vmovss	%xmm0, 572(%rsp)
.Ltmp5372:
	.loc	1 1279 33
	vmovss	1568(%rbx), %xmm0
.Ltmp5373:
	.loc	1 1192 28
	vmovss	%xmm0, 576(%rsp)
.Ltmp5374:
	.loc	1 1279 33
	vmovss	1728(%rbx), %xmm0
.Ltmp5375:
	.loc	1 1192 28
	vmovss	%xmm0, 580(%rsp)
.Ltmp5376:
	.loc	1 1279 33
	vmovss	1888(%rbx), %xmm0
.Ltmp5377:
	.loc	1 1192 28
	vmovss	%xmm0, 584(%rsp)
.Ltmp5378:
	.loc	1 1279 33
	vmovss	2048(%rbx), %xmm0
.Ltmp5379:
	.loc	1 1192 28
	vmovss	%xmm0, 588(%rsp)
.Ltmp5380:
	.loc	1 1279 33
	vmovss	1584(%rbx), %xmm0
.Ltmp5381:
	.loc	1 1192 28
	vmovss	%xmm0, 592(%rsp)
.Ltmp5382:
	.loc	1 1279 33
	vmovss	1744(%rbx), %xmm0
.Ltmp5383:
	.loc	1 1192 28
	vmovss	%xmm0, 596(%rsp)
.Ltmp5384:
	.loc	1 1279 33
	vmovss	1904(%rbx), %xmm0
.Ltmp5385:
	.loc	1 1192 28
	vmovss	%xmm0, 600(%rsp)
.Ltmp5386:
	.loc	1 1279 33
	vmovss	2064(%rbx), %xmm0
.Ltmp5387:
	.loc	1 1192 28
	vmovss	%xmm0, 604(%rsp)
.Ltmp5388:
	.loc	1 1279 33
	vmovss	1600(%rbx), %xmm0
.Ltmp5389:
	.loc	1 1192 28
	vmovss	%xmm0, 608(%rsp)
.Ltmp5390:
	.loc	1 1279 33
	vmovss	1760(%rbx), %xmm0
.Ltmp5391:
	.loc	1 1192 28
	vmovss	%xmm0, 612(%rsp)
.Ltmp5392:
	.loc	1 1279 33
	vmovss	1920(%rbx), %xmm0
.Ltmp5393:
	.loc	1 1192 28
	vmovss	%xmm0, 616(%rsp)
.Ltmp5394:
	.loc	1 1279 33
	vmovss	2080(%rbx), %xmm0
.Ltmp5395:
	.loc	1 1192 28
	vmovss	%xmm0, 620(%rsp)
.Ltmp5396:
	.loc	1 1279 33
	vmovss	1616(%rbx), %xmm0
.Ltmp5397:
	.loc	1 1192 28
	vmovss	%xmm0, 624(%rsp)
.Ltmp5398:
	.loc	1 1279 33
	vmovss	1776(%rbx), %xmm0
.Ltmp5399:
	.loc	1 1192 28
	vmovss	%xmm0, 628(%rsp)
.Ltmp5400:
	.loc	1 1279 33
	vmovss	1936(%rbx), %xmm0
.Ltmp5401:
	.loc	1 1192 28
	vmovss	%xmm0, 632(%rsp)
.Ltmp5402:
	.loc	1 1279 33
	vmovss	2096(%rbx), %xmm0
.Ltmp5403:
	.loc	1 1192 28
	vmovss	%xmm0, 636(%rsp)
.Ltmp5404:
	.loc	1 1279 33
	vmovss	1632(%rbx), %xmm0
.Ltmp5405:
	.loc	1 1192 28
	vmovss	%xmm0, 640(%rsp)
.Ltmp5406:
	.loc	1 1279 33
	vmovss	1792(%rbx), %xmm0
.Ltmp5407:
	.loc	1 1192 28
	vmovss	%xmm0, 644(%rsp)
.Ltmp5408:
	.loc	1 1279 33
	vmovss	1952(%rbx), %xmm0
.Ltmp5409:
	.loc	1 1192 28
	vmovss	%xmm0, 648(%rsp)
.Ltmp5410:
	.loc	1 1279 33
	vmovss	2112(%rbx), %xmm0
.Ltmp5411:
	.loc	1 1192 28
	vmovss	%xmm0, 652(%rsp)
.Ltmp5412:
	.loc	1 1279 33
	vmovss	1648(%rbx), %xmm0
.Ltmp5413:
	.loc	1 1192 28
	vmovss	%xmm0, 656(%rsp)
.Ltmp5414:
	.loc	1 1279 33
	vmovss	1808(%rbx), %xmm0
.Ltmp5415:
	.loc	1 1192 28
	vmovss	%xmm0, 660(%rsp)
.Ltmp5416:
	.loc	1 1279 33
	vmovss	1968(%rbx), %xmm0
.Ltmp5417:
	.loc	1 1192 28
	vmovss	%xmm0, 664(%rsp)
.Ltmp5418:
	.loc	1 1279 33
	vmovss	2128(%rbx), %xmm0
.Ltmp5419:
	.loc	1 1192 28
	vmovss	%xmm0, 668(%rsp)
.Ltmp5420:
	.loc	1 1279 33
	vmovss	1664(%rbx), %xmm0
.Ltmp5421:
	.loc	1 1192 28
	vmovss	%xmm0, 672(%rsp)
.Ltmp5422:
	.loc	1 1279 33
	vmovss	1824(%rbx), %xmm0
.Ltmp5423:
	.loc	1 1192 28
	vmovss	%xmm0, 676(%rsp)
.Ltmp5424:
	.loc	1 1279 33
	vmovss	1984(%rbx), %xmm0
.Ltmp5425:
	.loc	1 1192 28
	vmovss	%xmm0, 680(%rsp)
.Ltmp5426:
	.loc	1 1279 33
	vmovss	2144(%rbx), %xmm0
.Ltmp5427:
	.loc	1 1192 28
	vmovss	%xmm0, 684(%rsp)
.Ltmp5428:
	.loc	1 1280 32
	vmovss	1528(%rbx), %xmm0
.Ltmp5429:
	.loc	1 1192 28
	vmovss	%xmm0, 688(%rsp)
.Ltmp5430:
	.loc	1 1280 32
	vmovss	1688(%rbx), %xmm0
.Ltmp5431:
	.loc	1 1192 28
	vmovss	%xmm0, 692(%rsp)
.Ltmp5432:
	.loc	1 1280 32
	vmovss	1848(%rbx), %xmm0
.Ltmp5433:
	.loc	1 1192 28
	vmovss	%xmm0, 696(%rsp)
.Ltmp5434:
	.loc	1 1280 32
	vmovss	2008(%rbx), %xmm0
.Ltmp5435:
	.loc	1 1192 28
	vmovss	%xmm0, 700(%rsp)
.Ltmp5436:
	.loc	1 1280 32
	vmovss	1544(%rbx), %xmm0
.Ltmp5437:
	.loc	1 1192 28
	vmovss	%xmm0, 704(%rsp)
.Ltmp5438:
	.loc	1 1280 32
	vmovss	1704(%rbx), %xmm0
.Ltmp5439:
	.loc	1 1192 28
	vmovss	%xmm0, 708(%rsp)
.Ltmp5440:
	.loc	1 1280 32
	vmovss	1864(%rbx), %xmm0
.Ltmp5441:
	.loc	1 1192 28
	vmovss	%xmm0, 712(%rsp)
.Ltmp5442:
	.loc	1 1280 32
	vmovss	2024(%rbx), %xmm0
.Ltmp5443:
	.loc	1 1192 28
	vmovss	%xmm0, 716(%rsp)
.Ltmp5444:
	.loc	1 1280 32
	vmovss	1560(%rbx), %xmm0
.Ltmp5445:
	.loc	1 1192 28
	vmovss	%xmm0, 720(%rsp)
.Ltmp5446:
	.loc	1 1280 32
	vmovss	1720(%rbx), %xmm0
.Ltmp5447:
	.loc	1 1192 28
	vmovss	%xmm0, 724(%rsp)
.Ltmp5448:
	.loc	1 1280 32
	vmovss	1880(%rbx), %xmm0
.Ltmp5449:
	.loc	1 1192 28
	vmovss	%xmm0, 728(%rsp)
.Ltmp5450:
	.loc	1 1280 32
	vmovss	2040(%rbx), %xmm0
.Ltmp5451:
	.loc	1 1192 28
	vmovss	%xmm0, 732(%rsp)
.Ltmp5452:
	.loc	1 1280 32
	vmovss	1576(%rbx), %xmm0
.Ltmp5453:
	.loc	1 1192 28
	vmovss	%xmm0, 736(%rsp)
.Ltmp5454:
	.loc	1 1280 32
	vmovss	1736(%rbx), %xmm0
.Ltmp5455:
	.loc	1 1192 28
	vmovss	%xmm0, 740(%rsp)
.Ltmp5456:
	.loc	1 1280 32
	vmovss	1896(%rbx), %xmm0
.Ltmp5457:
	.loc	1 1192 28
	vmovss	%xmm0, 744(%rsp)
.Ltmp5458:
	.loc	1 1280 32
	vmovss	2056(%rbx), %xmm0
.Ltmp5459:
	.loc	1 1192 28
	vmovss	%xmm0, 748(%rsp)
.Ltmp5460:
	.loc	1 1280 32
	vmovss	1592(%rbx), %xmm0
.Ltmp5461:
	.loc	1 1192 28
	vmovss	%xmm0, 752(%rsp)
.Ltmp5462:
	.loc	1 1280 32
	vmovss	1752(%rbx), %xmm0
.Ltmp5463:
	.loc	1 1192 28
	vmovss	%xmm0, 756(%rsp)
.Ltmp5464:
	.loc	1 1280 32
	vmovss	1912(%rbx), %xmm0
.Ltmp5465:
	.loc	1 1192 28
	vmovss	%xmm0, 760(%rsp)
.Ltmp5466:
	.loc	1 1280 32
	vmovss	2072(%rbx), %xmm0
.Ltmp5467:
	.loc	1 1192 28
	vmovss	%xmm0, 764(%rsp)
.Ltmp5468:
	.loc	1 1280 32
	vmovss	1608(%rbx), %xmm0
.Ltmp5469:
	.loc	1 1192 28
	vmovss	%xmm0, 768(%rsp)
.Ltmp5470:
	.loc	1 1280 32
	vmovss	1768(%rbx), %xmm0
.Ltmp5471:
	.loc	1 1192 28
	vmovss	%xmm0, 772(%rsp)
.Ltmp5472:
	.loc	1 1280 32
	vmovss	1928(%rbx), %xmm0
.Ltmp5473:
	.loc	1 1192 28
	vmovss	%xmm0, 776(%rsp)
.Ltmp5474:
	.loc	1 1280 32
	vmovss	2088(%rbx), %xmm0
.Ltmp5475:
	.loc	1 1192 28
	vmovss	%xmm0, 780(%rsp)
.Ltmp5476:
	.loc	1 1280 32
	vmovss	1624(%rbx), %xmm0
.Ltmp5477:
	.loc	1 1192 28
	vmovss	%xmm0, 784(%rsp)
.Ltmp5478:
	.loc	1 1280 32
	vmovss	1784(%rbx), %xmm0
.Ltmp5479:
	.loc	1 1192 28
	vmovss	%xmm0, 788(%rsp)
.Ltmp5480:
	.loc	1 1280 32
	vmovss	1944(%rbx), %xmm0
.Ltmp5481:
	.loc	1 1192 28
	vmovss	%xmm0, 792(%rsp)
.Ltmp5482:
	.loc	1 1280 32
	vmovss	2104(%rbx), %xmm0
.Ltmp5483:
	.loc	1 1192 28
	vmovss	%xmm0, 796(%rsp)
.Ltmp5484:
	.loc	1 1280 32
	vmovss	1640(%rbx), %xmm0
.Ltmp5485:
	.loc	1 1192 28
	vmovss	%xmm0, 800(%rsp)
.Ltmp5486:
	.loc	1 1280 32
	vmovss	1800(%rbx), %xmm0
.Ltmp5487:
	.loc	1 1192 28
	vmovss	%xmm0, 804(%rsp)
.Ltmp5488:
	.loc	1 1280 32
	vmovss	1960(%rbx), %xmm0
.Ltmp5489:
	.loc	1 1192 28
	vmovss	%xmm0, 808(%rsp)
.Ltmp5490:
	.loc	1 1280 32
	vmovss	2120(%rbx), %xmm0
.Ltmp5491:
	.loc	1 1192 28
	vmovss	%xmm0, 812(%rsp)
.Ltmp5492:
	.loc	1 1280 32
	vmovss	1656(%rbx), %xmm0
.Ltmp5493:
	.loc	1 1192 28
	vmovss	%xmm0, 816(%rsp)
.Ltmp5494:
	.loc	1 1280 32
	vmovss	1816(%rbx), %xmm0
.Ltmp5495:
	.loc	1 1192 28
	vmovss	%xmm0, 820(%rsp)
.Ltmp5496:
	.loc	1 1280 32
	vmovss	1976(%rbx), %xmm0
.Ltmp5497:
	.loc	1 1192 28
	vmovss	%xmm0, 824(%rsp)
.Ltmp5498:
	.loc	1 1280 32
	vmovss	2136(%rbx), %xmm0
.Ltmp5499:
	.loc	1 1192 28
	vmovss	%xmm0, 828(%rsp)
.Ltmp5500:
	.loc	1 1280 32
	vmovss	1672(%rbx), %xmm0
.Ltmp5501:
	.loc	1 1192 28
	vmovss	%xmm0, 832(%rsp)
.Ltmp5502:
	.loc	1 1280 32
	vmovss	1832(%rbx), %xmm0
.Ltmp5503:
	.loc	1 1192 28
	vmovss	%xmm0, 836(%rsp)
.Ltmp5504:
	.loc	1 1280 32
	vmovss	1992(%rbx), %xmm0
.Ltmp5505:
	.loc	1 1192 28
	vmovss	%xmm0, 840(%rsp)
.Ltmp5506:
	.loc	1 1280 32
	vmovss	2152(%rbx), %xmm0
.Ltmp5507:
	.loc	1 1192 28
	vmovss	%xmm0, 844(%rsp)
.Ltmp5508:
	.loc	1 1194 31
	leaq	1328(%rsp), %rdi
	movq	%rbx, %rsi
	movl	1104(%rsp), %ebp
	movl	%ebp, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	1424(%rsp), %rdi
	movq	1312(%rsp), %rsi
	movl	%ebp, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E17band_coefficientsB5_
	.loc	1 1193 28
	vmovaps	1328(%rsp), %xmm0
	vmovaps	%xmm0, 1216(%rsp)
	vmovaps	1344(%rsp), %xmm0
	vmovaps	%xmm0, 1200(%rsp)
	vmovaps	1360(%rsp), %xmm0
	vmovaps	%xmm0, 1184(%rsp)
	vmovaps	1376(%rsp), %xmm0
	vmovaps	%xmm0, 1168(%rsp)
	vmovaps	1392(%rsp), %xmm0
	vmovaps	%xmm0, 1152(%rsp)
	vmovaps	1408(%rsp), %xmm0
	vmovaps	%xmm0, 1136(%rsp)
	vmovaps	1424(%rsp), %xmm0
	vmovaps	%xmm0, 1120(%rsp)
	vmovaps	1440(%rsp), %xmm0
	vmovaps	%xmm0, 1296(%rsp)
	vmovaps	1456(%rsp), %xmm0
	vmovaps	%xmm0, 1280(%rsp)
	vmovaps	1472(%rsp), %xmm0
	vmovaps	%xmm0, 1264(%rsp)
	vmovaps	1488(%rsp), %xmm0
	vmovaps	%xmm0, 1248(%rsp)
	vmovaps	1504(%rsp), %xmm0
	vmovaps	%xmm0, 1232(%rsp)
.Ltmp5509:
	.loc	1 0 0 is_stmt 0
	leaq	(%r14,%r15), %rax
	shlq	$2, %r15
	leaq	(,%rax,4), %rsi
	.loc	1 1197 12 is_stmt 1
	testb	$1, %r12b
	movq	%r14, 1024(%rsp)
	movq	%rax, 1112(%rsp)
	je	.LBB34_449
.Ltmp5510:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp5511:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_570
.Ltmp5512:
	.loc	48 451 16 is_stmt 1
	cmpq	112(%rsp), %rsi
	ja	.LBB34_570
.Ltmp5513:
	.loc	48 451 16 is_stmt 0
	cmpq	104(%rsp), %rsi
	ja	.LBB34_577
.Ltmp5514:
	.loc	1 1053 27 is_stmt 1
	vmovaps	96(%rbx), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	112(%rbx), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	128(%rbx), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	144(%rbx), %xmm0
	vmovaps	%xmm0, 64(%rsp)
.Ltmp5515:
	.loc	1 1054 26
	vmovaps	1424(%rbx), %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vmovaps	1440(%rbx), %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmovaps	1456(%rbx), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	1472(%rbx), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5516:
	.loc	1 1055 25
	vmovaps	160(%rbx), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	176(%rbx), %xmm0
	vmovaps	%xmm0, 880(%rsp)
.Ltmp5517:
	.loc	1 1056 24
	vmovaps	1488(%rbx), %xmm0
	vmovaps	%xmm0, 896(%rsp)
	vmovaps	1504(%rbx), %xmm7
.Ltmp5518:
	.loc	1 1057 24
	movq	2680(%rbx), %r12
.Ltmp5519:
	.loc	1 871 17
	movq	1104(%rbx), %rax
	movq	1112(%rbx), %rcx
.Ltmp5520:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	1120(%rbx), %rdx
	xorq	%rax, %rdx
	orq	%rcx, %rdx
	xorq	1128(%rbx), %rax
	orq	%rdx, %rax
	sete	15(%rsp)
.Ltmp5521:
	.loc	1 871 17
	movq	2432(%rbx), %rax
	movq	2440(%rbx), %rcx
.Ltmp5522:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2448(%rbx), %rdx
	xorq	%rax, %rdx
	xorq	2456(%rbx), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	192(%rsp)
.Ltmp5523:
	.loc	2 1916 50
	testq	%r14, %r14
	movq	176(%rsp), %rbp
	je	.LBB34_445
.Ltmp5524:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r14,4), %rax
	movq	%rax, 152(%rsp)
	movq	968(%rsp), %rax
	leaq	(%rax,%r15,4), %rcx
	movq	120(%rsp), %rax
	leaq	(%rax,%r15,4), %r9
.Ltmp5525:
	.loc	3 900 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	movq	%r14, 936(%rsp)
	xorl	%r10d, %r10d
	xorl	%esi, %esi
	movq	%rcx, 976(%rsp)
	movq	%r9, 928(%rsp)
.Ltmp5526:
	.loc	3 0 12 is_stmt 0
.Ltmp5527:
	.p2align	4
.LBB34_396:
	.loc	1 1064 21 is_stmt 1
	vmovaps	208(%rsp), %xmm0
	vmovaps	224(%rsp), %xmm1
	vmovaps	240(%rsp), %xmm2
.Ltmp5528:
	.loc	9 36 14
	vaddps	368(%rsp), %xmm0, %xmm0
.Ltmp5529:
	.loc	1 1066 21
	vmovaps	528(%rsp), %xmm3
	.loc	1 1063 17
	vmovaps	%xmm0, 208(%rsp)
.Ltmp5530:
	.loc	9 36 14
	vaddps	688(%rsp), %xmm3, %xmm0
.Ltmp5531:
	.loc	1 1065 17
	vmovaps	%xmm0, 528(%rsp)
.Ltmp5532:
	.loc	9 36 14
	vaddps	384(%rsp), %xmm1, %xmm0
.Ltmp5533:
	.loc	1 1063 17
	vmovaps	%xmm0, 224(%rsp)
	.loc	1 1066 21
	vmovaps	544(%rsp), %xmm0
.Ltmp5534:
	.loc	9 36 14
	vaddps	704(%rsp), %xmm0, %xmm0
.Ltmp5535:
	.loc	1 1065 17
	vmovaps	%xmm0, 544(%rsp)
.Ltmp5536:
	.loc	9 36 14
	vaddps	400(%rsp), %xmm2, %xmm0
.Ltmp5537:
	.loc	1 1063 17
	vmovaps	%xmm0, 240(%rsp)
	.loc	1 1066 21
	vmovaps	560(%rsp), %xmm0
.Ltmp5538:
	.loc	9 36 14
	vaddps	720(%rsp), %xmm0, %xmm0
.Ltmp5539:
	.loc	1 1065 17
	vmovaps	%xmm0, 560(%rsp)
	.loc	1 1064 21
	vmovaps	256(%rsp), %xmm0
.Ltmp5540:
	.loc	9 36 14
	vaddps	416(%rsp), %xmm0, %xmm0
.Ltmp5541:
	.loc	1 1063 17
	vmovaps	%xmm0, 256(%rsp)
	.loc	1 1066 21
	vmovaps	576(%rsp), %xmm0
.Ltmp5542:
	.loc	9 36 14
	vaddps	736(%rsp), %xmm0, %xmm0
.Ltmp5543:
	.loc	1 1065 17
	vmovaps	%xmm0, 576(%rsp)
	.loc	1 1064 21
	vmovaps	272(%rsp), %xmm0
.Ltmp5544:
	.loc	9 36 14
	vaddps	432(%rsp), %xmm0, %xmm0
.Ltmp5545:
	.loc	1 1063 17
	vmovaps	%xmm0, 272(%rsp)
	.loc	1 1066 21
	vmovaps	592(%rsp), %xmm0
.Ltmp5546:
	.loc	9 36 14
	vaddps	752(%rsp), %xmm0, %xmm0
.Ltmp5547:
	.loc	1 1065 17
	vmovaps	%xmm0, 592(%rsp)
	.loc	1 1064 21
	vmovaps	288(%rsp), %xmm0
.Ltmp5548:
	.loc	9 36 14
	vaddps	448(%rsp), %xmm0, %xmm0
.Ltmp5549:
	.loc	1 1063 17
	vmovaps	%xmm0, 288(%rsp)
	.loc	1 1066 21
	vmovaps	608(%rsp), %xmm0
.Ltmp5550:
	.loc	9 36 14
	vaddps	768(%rsp), %xmm0, %xmm0
.Ltmp5551:
	.loc	1 1065 17
	vmovaps	%xmm0, 608(%rsp)
	.loc	1 1064 21
	vmovaps	304(%rsp), %xmm0
.Ltmp5552:
	.loc	9 36 14
	vaddps	464(%rsp), %xmm0, %xmm0
.Ltmp5553:
	.loc	1 1063 17
	vmovaps	%xmm0, 304(%rsp)
	.loc	1 1066 21
	vmovaps	624(%rsp), %xmm0
.Ltmp5554:
	.loc	9 36 14
	vaddps	784(%rsp), %xmm0, %xmm0
.Ltmp5555:
	.loc	1 1065 17
	vmovaps	%xmm0, 624(%rsp)
	.loc	1 1064 21
	vmovaps	320(%rsp), %xmm0
.Ltmp5556:
	.loc	9 36 14
	vaddps	480(%rsp), %xmm0, %xmm0
.Ltmp5557:
	.loc	1 1063 17
	vmovaps	%xmm0, 320(%rsp)
	.loc	1 1066 21
	vmovaps	640(%rsp), %xmm0
.Ltmp5558:
	.loc	9 36 14
	vaddps	800(%rsp), %xmm0, %xmm0
.Ltmp5559:
	.loc	1 1065 17
	vmovaps	%xmm0, 640(%rsp)
	.loc	1 1064 21
	vmovaps	336(%rsp), %xmm0
.Ltmp5560:
	.loc	9 36 14
	vaddps	496(%rsp), %xmm0, %xmm0
.Ltmp5561:
	.loc	1 1063 17
	vmovaps	%xmm0, 336(%rsp)
	.loc	1 1066 21
	vmovaps	656(%rsp), %xmm0
.Ltmp5562:
	.loc	9 36 14
	vaddps	816(%rsp), %xmm0, %xmm0
.Ltmp5563:
	.loc	1 1065 17
	vmovaps	%xmm0, 656(%rsp)
	.loc	1 1064 21
	vmovaps	352(%rsp), %xmm0
.Ltmp5564:
	.loc	9 36 14
	vaddps	512(%rsp), %xmm0, %xmm0
.Ltmp5565:
	.loc	1 1063 17
	vmovaps	%xmm0, 352(%rsp)
	.loc	1 1066 21
	vmovaps	672(%rsp), %xmm0
.Ltmp5566:
	.loc	9 36 14
	vaddps	832(%rsp), %xmm0, %xmm0
.Ltmp5567:
	.loc	1 1065 17
	vmovaps	%xmm0, 672(%rsp)
.Ltmp5568:
	.loc	1 1070 28
	leaq	1(%r12), %rax
.Ltmp5569:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %edi
	cmovaeq	%rbp, %rdi
.Ltmp5570:
	.loc	48 568 12
	cmpq	152(%rsp), %r10
	ja	.LBB34_565
.Ltmp5571:
	.loc	48 438 16
	cmpq	%rsi, 936(%rsp)
	je	.LBB34_534
.Ltmp5572:
	.loc	48 0 16 is_stmt 0
	movq	%rsi, 200(%rsp)
	leaq	(,%r12,4), %rax
.Ltmp5573:
	.loc	1 1083 29 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp5574:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_540
.Ltmp5575:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5576:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm7, 1088(%rsp)
	vmovups	(%rcx,%r10,4), %xmm7
.Ltmp5577:
	vmovups	(%r9,%r10,4), %xmm1
.Ltmp5578:
	vmovaps	32(%rbx), %xmm5
	vmovaps	48(%rbx), %xmm11
	vmovaps	64(%rbx), %xmm0
	vmovaps	1360(%rbx), %xmm14
	vmovaps	1376(%rbx), %xmm9
	vmovaps	1392(%rbx), %xmm3
	vmovaps	16(%rsp), %xmm10
	vsubps	%xmm10, %xmm7, %xmm2
	vmulps	%xmm2, %xmm11, %xmm4
	vmovaps	32(%rsp), %xmm8
	vmovaps	%xmm5, 160(%rsp)
	vmulps	%xmm5, %xmm8, %xmm5
	vaddps	%xmm4, %xmm5, %xmm6
	vaddps	%xmm6, %xmm8, %xmm4
	vmulps	%xmm11, %xmm8, %xmm5
	vmulps	%xmm0, %xmm2, %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
	vaddps	%xmm5, %xmm10, %xmm2
	vmulps	80(%rbx), %xmm4, %xmm15
	vmovaps	64(%rsp), %xmm8
	vsubps	%xmm8, %xmm2, %xmm4
	vmulps	80(%rsp), %xmm11, %xmm2
	vmulps	%xmm4, %xmm0, %xmm0
	vaddps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vaddps	%xmm0, %xmm8, %xmm0
	vmovaps	912(%rsp), %xmm10
.Ltmp5579:
	vsubps	%xmm10, %xmm1, %xmm13
	vmulps	%xmm9, %xmm13, %xmm2
	vmovaps	992(%rsp), %xmm8
	vmulps	%xmm14, %xmm8, %xmm12
	vaddps	%xmm2, %xmm12, %xmm2
	vaddps	%xmm2, %xmm8, %xmm12
	vmulps	1408(%rbx), %xmm12, %xmm12
.Ltmp5580:
	.loc	1 1083 29 is_stmt 1
	movq	(%rbx), %rcx
.Ltmp5581:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5582:
	.loc	1 1084 30
	movq	24(%rbx), %rsi
.Ltmp5583:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_541
.Ltmp5584:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5585:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm7, %xmm15, %xmm7
	vsubps	%xmm0, %xmm7, %xmm0
.Ltmp5586:
	.loc	1 1084 30 is_stmt 1
	movq	16(%rbx), %rcx
.Ltmp5587:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5588:
	.loc	1 1085 28
	movq	1336(%rbx), %rsi
.Ltmp5589:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_542
.Ltmp5590:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5591:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm14, 944(%rsp)
	vmulps	%xmm9, %xmm8, %xmm0
	vmulps	%xmm3, %xmm13, %xmm7
	vaddps	%xmm7, %xmm0, %xmm13
	vaddps	%xmm13, %xmm10, %xmm0
	vmovaps	128(%rsp), %xmm14
	vsubps	%xmm14, %xmm0, %xmm15
	vmulps	48(%rsp), %xmm9, %xmm0
	vmulps	%xmm3, %xmm15, %xmm3
	vaddps	%xmm3, %xmm0, %xmm7
	vaddps	%xmm7, %xmm14, %xmm0
.Ltmp5592:
	.loc	1 1085 28 is_stmt 1
	movq	1328(%rbx), %rcx
.Ltmp5593:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5594:
	.loc	1 1086 29
	movq	1352(%rbx), %rsi
.Ltmp5595:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_543
.Ltmp5596:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5597:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm1, %xmm12, %xmm1
	vsubps	%xmm0, %xmm1, %xmm0
.Ltmp5598:
	.loc	1 1086 29 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp5599:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5600:
	.loc	1 1089 13
	movq	(%rbx), %r14
	movq	8(%rbx), %rsi
	movq	1104(%rbx), %rax
.Ltmp5601:
	.loc	1 0 0 is_stmt 0
	addq	%r12, %rax
.Ltmp5602:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp5603:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 15(%rsp)
	movq	%r10, 1008(%rsp)
	je	.LBB34_412
.Ltmp5604:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp5605:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5606:
	.loc	1 1096 13
	movq	24(%rbx), %rsi
.Ltmp5607:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp5608:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp5609:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp5610:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5611:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r14,%r9,4), %xmm12
.Ltmp5612:
	movq	16(%rbx), %rcx
.Ltmp5613:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm1
.Ltmp5614:
	.loc	1 961 2
	jmp	.LBB34_421
.Ltmp5615:
	.loc	1 0 2 is_stmt 0
.Ltmp5616:
	.p2align	4
.LBB34_412:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	1112(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp5617:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp5618:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_586
	.loc	1 0 25
	movq	1120(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp5619:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp5620:
	.loc	1 955 30
	leaq	2(,%rcx,4), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_601
	.loc	1 0 25
	movq	1128(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp5621:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp5622:
	.loc	1 955 30
	leaq	3(,%rcx,4), %rdx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rdx
	jae	.LBB34_607
.Ltmp5623:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp5624:
	.loc	1 1096 13
	movq	24(%r13), %rsi
.Ltmp5625:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp5626:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	1112(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp5627:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp5628:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_585
	.loc	1 0 25
	movq	1120(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp5629:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%rbp, %r10
	movl	$0, %ebp
	cmovaeq	%r10, %rbp
	subq	%rbp, %rbx
.Ltmp5630:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	%r11, %rcx
	movq	%r8, %r11
	movq	%r14, %r8
	movq	1128(%r13), %rbx
	movq	%r12, %r14
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp5631:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r12d
	cmovaeq	%r10, %r12
	subq	%r12, %rbx
.Ltmp5632:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp5633:
	.loc	1 0 0
	vmovd	(%r8,%r9,4), %xmm0
	vpinsrd	$1, (%r8,%r11,4), %xmm0, %xmm0
	vpinsrd	$2, (%r8,%rcx,4), %xmm0, %xmm0
	vpinsrd	$3, (%r8,%rdx,4), %xmm0, %xmm12
.Ltmp5634:
	movq	16(%r13), %rcx
.Ltmp5635:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm1
	movq	%r13, %rbx
	movq	176(%rsp), %rbp
	movq	1008(%rsp), %r10
	movq	%r14, %r12
.Ltmp5636:
.LBB34_421:
	.loc	1 1103 13
	movq	1328(%rbx), %rdx
	movq	1336(%rbx), %rsi
	movq	2432(%rbx), %rax
.Ltmp5637:
	.loc	1 0 0 is_stmt 0
	addq	%r12, %rax
.Ltmp5638:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp5639:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 192(%rsp)
	je	.LBB34_427
.Ltmp5640:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp5641:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5642:
	.loc	1 1110 13
	movq	1352(%rbx), %rsi
.Ltmp5643:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp5644:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp5645:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp5646:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5647:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%rdx,%r9,4), %xmm3
.Ltmp5648:
	movq	1344(%rbx), %rcx
.Ltmp5649:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm0
.Ltmp5650:
	.loc	1 961 2
	jmp	.LBB34_436
.Ltmp5651:
	.loc	1 0 2 is_stmt 0
.Ltmp5652:
	.p2align	4
.LBB34_427:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	2440(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp5653:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp5654:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_586
	.loc	1 0 25
	movq	2448(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp5655:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp5656:
	.loc	1 955 30
	leaq	2(,%rcx,4), %r11
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r11
	jae	.LBB34_601
	.loc	1 0 25
	movq	2456(%rbx), %rcx
	.loc	1 955 35
	addq	%r12, %rcx
.Ltmp5657:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp5658:
	.loc	1 955 30
	leaq	3(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
.Ltmp5659:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp5660:
	.loc	1 1110 13
	movq	1352(%r13), %rsi
.Ltmp5661:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp5662:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	%rcx, 1040(%rsp)
	movq	%rdi, 1056(%rsp)
	movq	2440(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp5663:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp5664:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_585
	.loc	1 0 25
	movq	2448(%r13), %rbx
	.loc	1 955 35
	addq	%r12, %rbx
.Ltmp5665:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%r12, %r14
	movl	$0, %r12d
	cmovaeq	%rbp, %r12
	subq	%r12, %rbx
	movq	%rbp, %r10
.Ltmp5666:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_631
	.loc	1 0 25
	movq	2456(%r13), %rbx
	.loc	1 955 35
	addq	%r14, %rbx
.Ltmp5667:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r12d
	cmovaeq	%r10, %r12
	subq	%r12, %rbx
.Ltmp5668:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp5669:
	.loc	1 0 25
	movq	%r11, %rcx
	movq	%rdx, %r11
	movq	%r8, %rdx
	vmovd	(%r11,%r9,4), %xmm0
	vpinsrd	$1, (%r11,%rdx,4), %xmm0, %xmm0
	vpinsrd	$2, (%r11,%rcx,4), %xmm0, %xmm0
	movq	1040(%rsp), %rcx
	vpinsrd	$3, (%r11,%rcx,4), %xmm0, %xmm3
.Ltmp5670:
	movq	1344(%r13), %rcx
.Ltmp5671:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm0
	movq	%r13, %rbx
	movq	176(%rsp), %rbp
	movq	1008(%rsp), %r10
	movq	%r14, %r12
	movq	1056(%rsp), %rdi
.Ltmp5672:
.LBB34_436:
	.loc	1 0 0 is_stmt 0
	negq	%rdi
	addq	%rdi, %r12
	incq	%r12
	leaq	(,%r12,4), %rax
.Ltmp5673:
	.loc	1 1150 36 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp5674:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	1024(%rsp), %r14
	movq	928(%rsp), %r9
	jb	.LBB34_546
.Ltmp5675:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5676:
	.loc	1 1152 27
	movq	24(%rbx), %rsi
.Ltmp5677:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_547
.Ltmp5678:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5679:
	.loc	1 1153 35
	movq	1336(%rbx), %rsi
.Ltmp5680:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_548
.Ltmp5681:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5682:
	.loc	1 1155 27
	movq	1352(%rbx), %rsi
.Ltmp5683:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_550
.Ltmp5684:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5685:
	.loc	48 0 16 is_stmt 0
	vmovdqa	%xmm0, 1056(%rsp)
	vaddps	%xmm6, %xmm6, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vmovdqa	%xmm1, 1040(%rsp)
	vmovdqa	%xmm12, %xmm1
	vbroadcastss	.LCPI34_35(%rip), %xmm12
	vandps	%xmm0, %xmm12, %xmm6
	vmovaps	%xmm10, %xmm14
	vmovaps	%xmm8, %xmm10
	vbroadcastss	.LCPI34_2(%rip), %xmm8
	vcmpltps	%xmm8, %xmm6, %xmm6
	vandnps	%xmm0, %xmm6, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm5, %xmm5, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm5
	vcmpltps	%xmm8, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmulps	%xmm4, %xmm11, %xmm0
	vmovaps	80(%rsp), %xmm5
	vmulps	160(%rsp), %xmm5, %xmm4
	vaddps	%xmm0, %xmm4, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm5, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	848(%rsp), %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 64(%rsp)
.Ltmp5686:
	vaddps	%xmm2, %xmm2, %xmm0
	vaddps	%xmm0, %xmm10, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	%xmm0, %xmm14, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmulps	%xmm15, %xmm9, %xmm0
	vmovaps	48(%rsp), %xmm4
	vmulps	944(%rsp), %xmm4, %xmm2
	vaddps	%xmm0, %xmm2, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm4, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm7, %xmm7, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5687:
	vpand	%xmm1, %xmm12, %xmm0
	vpand	%xmm3, %xmm12, %xmm1
	vbroadcastss	.LCPI34_3(%rip), %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm4, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
.Ltmp5688:
	vandps	1040(%rsp), %xmm12, %xmm1
	vandps	1056(%rsp), %xmm12, %xmm2
	vmulps	%xmm4, %xmm1, %xmm1
	vmulps	%xmm4, %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm2
	vbroadcastss	.LCPI34_4(%rip), %xmm5
.Ltmp5689:
	vmaxps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm6
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm7
	vandps	%xmm7, %xmm0, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm9
	vorps	%xmm1, %xmm9, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm10
	vaddps	%xmm1, %xmm10, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm1, %xmm11, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm14
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vsubps	208(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm0, %xmm1, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm7
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1216(%rsp), %xmm3, %xmm3
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm0, %xmm1, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	864(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm3
	vmovaps	1184(%rsp), %xmm4
	vblendvps	%xmm3, 1200(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm1, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm8, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm1
.Ltmp5690:
	vbroadcastss	.LCPI34_4(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm0
	vmaxps	%xmm6, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm2
	vorps	%xmm2, %xmm9, %xmm2
	vaddps	%xmm2, %xmm10, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm14, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 944(%rsp)
	vsubps	288(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm3
	vaddps	%xmm3, %xmm0, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vmulps	%xmm5, %xmm2, %xmm2
	vcmpltps	%xmm0, %xmm3, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm2, %xmm2
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1168(%rsp), %xmm2, %xmm2
	vxorps	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm3, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm1, 864(%rsp)
.Ltmp5691:
	vaddps	272(%rsp), %xmm1, %xmm2
	vbroadcastss	.LCPI34_24(%rip), %xmm10
	vmulps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_25(%rip), %xmm11
	vmaxps	%xmm11, %xmm2, %xmm2
	vbroadcastss	.LCPI34_26(%rip), %xmm11
	vminps	%xmm11, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm4
	vsubps	%xmm4, %xmm2, %xmm2
	vbroadcastss	.LCPI34_27(%rip), %xmm14
	vmulps	%xmm2, %xmm14, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm7
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm8
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm9
	vaddps	%xmm5, %xmm9, %xmm5
	vbroadcastss	.LCPI34_23(%rip), %xmm14
.Ltmp5692:
	vmaxps	%xmm14, %xmm0, %xmm0
	vminps	%xmm3, %xmm0, %xmm0
	vmovaps	880(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm6
	vmovaps	1136(%rsp), %xmm10
	vblendvps	%xmm6, 1152(%rsp), %xmm10, %xmm6
.Ltmp5693:
	vmulps	%xmm5, %xmm2, %xmm2
.Ltmp5694:
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm6, %xmm5, %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm5
	vbroadcastss	.LCPI34_2(%rip), %xmm13
	vcmpltps	%xmm13, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm6
.Ltmp5695:
	vaddps	%xmm6, %xmm2, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm10
	vaddps	%xmm4, %xmm10, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vmovaps	%xmm1, 880(%rsp)
.Ltmp5696:
	vaddps	352(%rsp), %xmm1, %xmm4
.Ltmp5697:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vbroadcastss	.LCPI34_24(%rip), %xmm2
.Ltmp5698:
	vmulps	%xmm2, %xmm4, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmovaps	%xmm7, %xmm15
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm0
	vmovaps	%xmm6, %xmm8
	vaddps	%xmm4, %xmm10, %xmm4
	vmovaps	%xmm10, %xmm9
	vpslld	$23, %xmm4, %xmm4
	vmovaps	160(%rsp), %xmm1
.Ltmp5699:
	vsubps	528(%rsp), %xmm1, %xmm5
.Ltmp5700:
	vmulps	%xmm4, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm6
.Ltmp5701:
	vaddps	%xmm6, %xmm5, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm11
	vmulps	%xmm0, %xmm11, %xmm0
	vcmpltps	%xmm5, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm10
	vcmpleps	%xmm10, %xmm5, %xmm4
	vmulps	1120(%rsp), %xmm0, %xmm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm4, %xmm1, %xmm4
	vpandn	%xmm0, %xmm4, %xmm0
	vmaxps	%xmm14, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vxorps	%xmm7, %xmm7, %xmm7
	vmovaps	896(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm4
	vmovaps	1280(%rsp), %xmm5
	vblendvps	%xmm4, 1296(%rsp), %xmm5, %xmm4
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm4
	vcmpltps	%xmm13, %xmm4, %xmm4
	vmovaps	%xmm13, %xmm14
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 896(%rsp)
	vaddps	592(%rsp), %xmm0, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm13
	vmaxps	%xmm13, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm0, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm0, %xmm8, %xmm0
	vaddps	%xmm4, %xmm9, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vmovaps	944(%rsp), %xmm0
.Ltmp5702:
	vsubps	608(%rsp), %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm3, %xmm11, %xmm3
	vcmpltps	%xmm0, %xmm6, %xmm5
	vblendvps	%xmm5, %xmm0, %xmm3, %xmm3
	vcmpleps	%xmm10, %xmm0, %xmm0
	vmulps	1264(%rsp), %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm7, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm7, %xmm0, %xmm0
	vmovaps	1088(%rsp), %xmm5
	vcmpltps	%xmm5, %xmm0, %xmm3
	vmovaps	1232(%rsp), %xmm1
	vblendvps	%xmm3, 1248(%rsp), %xmm1, %xmm3
	vsubps	%xmm0, %xmm5, %xmm5
	vmulps	%xmm3, %xmm5, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm12, %xmm3
	vcmpltps	%xmm14, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm7
	vaddps	672(%rsp), %xmm7, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm13, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm0, %xmm8, %xmm0
	vaddps	%xmm3, %xmm9, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
.Ltmp5703:
	movq	(%rbx), %rcx
	vmovaps	848(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm2
	movq	16(%rbx), %rcx
	vmovaps	160(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm1
	vaddps	%xmm1, %xmm2, %xmm1
.Ltmp5704:
	movq	1328(%rbx), %rcx
	vmulps	(%rcx,%rax,4), %xmm4, %xmm2
	.loc	1 1155 27 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp5705:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
.Ltmp5706:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	976(%rsp), %rcx
.Ltmp5707:
	.loc	8 551 14
	vmovups	%xmm1, (%rcx,%r10,4)
.Ltmp5708:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r9,%r10,4)
	movq	200(%rsp), %rsi
.Ltmp5709:
	.loc	1 0 0
	incq	%rsi
.Ltmp5710:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %r10
	cmpq	%rsi, %r14
.Ltmp5711:
	.loc	3 900 12
	jne	.LBB34_396
.Ltmp5712:
.LBB34_445:
	.loc	3 0 12 is_stmt 0
	vmovaps	32(%rsp), %xmm0
	.loc	1 1160 5 is_stmt 1
	vmovaps	%xmm0, 96(%rbx)
	vmovaps	16(%rsp), %xmm0
	vmovaps	%xmm0, 112(%rbx)
	vmovaps	80(%rsp), %xmm0
	vmovaps	%xmm0, 128(%rbx)
	vmovaps	64(%rsp), %xmm0
	vmovaps	%xmm0, 144(%rbx)
	vmovaps	992(%rsp), %xmm0
	.loc	1 1161 5
	vmovaps	%xmm0, 1424(%rbx)
	vmovaps	912(%rsp), %xmm0
	vmovaps	%xmm0, 1440(%rbx)
	vmovaps	48(%rsp), %xmm0
	vmovaps	%xmm0, 1456(%rbx)
	vmovaps	128(%rsp), %xmm0
	vmovaps	%xmm0, 1472(%rbx)
	vmovaps	864(%rsp), %xmm0
	.loc	1 1162 5
	vmovaps	%xmm0, 160(%rbx)
	vmovaps	880(%rsp), %xmm0
	vmovaps	%xmm0, 176(%rbx)
	vmovaps	896(%rsp), %xmm0
	.loc	1 1163 5
	vmovaps	%xmm0, 1488(%rbx)
	vmovaps	%xmm7, 1504(%rbx)
	.loc	1 1164 5
	movq	%r12, 2680(%rbx)
	xorl	%eax, %eax
	xorl	%edi, %edi
.Ltmp5713:
	.loc	1 0 5 is_stmt 0
.Ltmp5714:
	.p2align	4
.LBB34_446:
	.loc	1 1297 13 is_stmt 1
	vmovd	208(%rsp,%rax), %xmm0
	vmovss	212(%rsp,%rax), %xmm1
	vmovss	216(%rsp,%rax), %xmm2
	vmovss	220(%rsp,%rax), %xmm3
.Ltmp5715:
	.loc	1 1300 17
	vmovd	%xmm0, 192(%rbx,%rax)
	.loc	1 1301 34
	movl	204(%rbx,%rax), %ecx
	movl	364(%rbx,%rax), %edx
.Ltmp5716:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5717:
	.loc	1 1301 17
	movl	%ecx, 204(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 352(%rbx,%rax)
.Ltmp5718:
	.loc	38 2472 13
	subl	%r14d, %edx
	cmovbl	%edi, %edx
.Ltmp5719:
	.loc	1 1301 17
	movl	%edx, 364(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 512(%rbx,%rax)
	.loc	1 1301 34
	movl	524(%rbx,%rax), %ecx
.Ltmp5720:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5721:
	.loc	1 1301 17
	movl	%ecx, 524(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm3, 672(%rbx,%rax)
	.loc	1 1301 34
	movl	684(%rbx,%rax), %ecx
.Ltmp5722:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5723:
	.loc	1 1301 17
	movl	%ecx, 684(%rbx,%rax)
.Ltmp5724:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp5725:
	.loc	3 900 12
	jne	.LBB34_446
.Ltmp5726:
	.loc	3 0 12 is_stmt 0
	xorl	%eax, %eax
	movq	984(%rsp), %rsi
	movq	120(%rsp), %r9
	.p2align	4
.LBB34_448:
.Ltmp5727:
	.loc	1 1297 13 is_stmt 1
	vmovd	528(%rsp,%rax), %xmm0
	vmovss	532(%rsp,%rax), %xmm1
	vmovss	536(%rsp,%rax), %xmm2
	vmovss	540(%rsp,%rax), %xmm3
.Ltmp5728:
	.loc	1 1300 17
	vmovd	%xmm0, 1520(%rbx,%rax)
	.loc	1 1301 34
	movl	1532(%rbx,%rax), %ecx
	movl	1692(%rbx,%rax), %edx
.Ltmp5729:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5730:
	.loc	1 1301 17
	movl	%ecx, 1532(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm1, 1680(%rbx,%rax)
.Ltmp5731:
	.loc	38 2472 13
	subl	%r14d, %edx
	cmovbl	%edi, %edx
.Ltmp5732:
	.loc	1 1301 17
	movl	%edx, 1692(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm2, 1840(%rbx,%rax)
	.loc	1 1301 34
	movl	1852(%rbx,%rax), %ecx
.Ltmp5733:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5734:
	.loc	1 1301 17
	movl	%ecx, 1852(%rbx,%rax)
	.loc	1 1300 17
	vmovss	%xmm3, 2000(%rbx,%rax)
	.loc	1 1301 34
	movl	2012(%rbx,%rax), %ecx
.Ltmp5735:
	.loc	38 2472 13
	subl	%r14d, %ecx
	cmovbl	%edi, %ecx
.Ltmp5736:
	.loc	1 1301 17
	movl	%ecx, 2012(%rbx,%rax)
.Ltmp5737:
	.loc	2 1916 50
	addq	$16, %rax
	cmpq	$160, %rax
.Ltmp5738:
	.loc	3 900 12
	jne	.LBB34_448
	jmp	.LBB34_389
.Ltmp5739:
.LBB34_449:
	.loc	38 1050 16
	cmpq	%r15, %rsi
.Ltmp5740:
	.loc	38 1050 16 is_stmt 0
	jb	.LBB34_571
.Ltmp5741:
	.loc	48 451 16 is_stmt 1
	cmpq	112(%rsp), %rsi
	ja	.LBB34_571
.Ltmp5742:
	.loc	48 451 16 is_stmt 0
	cmpq	104(%rsp), %rsi
	ja	.LBB34_576
.Ltmp5743:
	.loc	1 1053 27 is_stmt 1
	vmovaps	96(%rbx), %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vmovaps	112(%rbx), %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmovaps	128(%rbx), %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vmovaps	144(%rbx), %xmm0
	vmovaps	%xmm0, 64(%rsp)
.Ltmp5744:
	.loc	1 1054 26
	vmovaps	1424(%rbx), %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vmovaps	1440(%rbx), %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmovaps	1456(%rbx), %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vmovaps	1472(%rbx), %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5745:
	.loc	1 1055 25
	vmovaps	160(%rbx), %xmm0
	vmovaps	%xmm0, 864(%rsp)
	vmovaps	176(%rbx), %xmm0
	vmovaps	%xmm0, 880(%rsp)
.Ltmp5746:
	.loc	1 1056 24
	vmovaps	1488(%rbx), %xmm0
	vmovaps	%xmm0, 896(%rsp)
	vmovaps	1504(%rbx), %xmm7
.Ltmp5747:
	.loc	1 1057 24
	movq	2680(%rbx), %r10
.Ltmp5748:
	.loc	1 871 17
	movq	1104(%rbx), %rax
	movq	1112(%rbx), %rcx
.Ltmp5749:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	1120(%rbx), %rdx
	xorq	%rax, %rdx
	orq	%rcx, %rdx
	xorq	1128(%rbx), %rax
	orq	%rdx, %rax
	sete	15(%rsp)
.Ltmp5750:
	.loc	1 871 17
	movq	2432(%rbx), %rax
	movq	2440(%rbx), %rcx
.Ltmp5751:
	.loc	1 874 12
	xorq	%rax, %rcx
	movq	2448(%rbx), %rdx
	xorq	%rax, %rdx
	xorq	2456(%rbx), %rax
	orq	%rcx, %rdx
	orq	%rdx, %rax
	sete	192(%rsp)
.Ltmp5752:
	.loc	2 1916 50
	testq	%r14, %r14
	movq	176(%rsp), %rbp
.Ltmp5753:
	.loc	3 900 12
	je	.LBB34_388
.Ltmp5754:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r14,4), %rax
	movq	%rax, 152(%rsp)
	movq	968(%rsp), %rax
	leaq	(%rax,%r15,4), %rcx
	movq	120(%rsp), %rax
	leaq	(%rax,%r15,4), %r9
.Ltmp5755:
	.loc	48 568 12 is_stmt 1
	movabsq	$4611686018427387903, %rax
	andq	%rax, %r14
	movq	%r14, 936(%rsp)
	xorl	%edi, %edi
	xorl	%r11d, %r11d
	movq	%rcx, 976(%rsp)
	movq	%r9, 928(%rsp)
.Ltmp5756:
	.loc	48 0 12 is_stmt 0
.Ltmp5757:
	.p2align	4
.LBB34_454:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%r10), %rax
.Ltmp5758:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %edx
	cmovaeq	%rbp, %rdx
.Ltmp5759:
	.loc	48 568 12
	cmpq	152(%rsp), %rdi
	ja	.LBB34_563
.Ltmp5760:
	.loc	48 438 16
	cmpq	%r11, 936(%rsp)
	je	.LBB34_534
.Ltmp5761:
	.loc	1 0 0 is_stmt 0
	leaq	(,%r10,4), %rax
.Ltmp5762:
	.loc	1 1083 29 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp5763:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_540
.Ltmp5764:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5765:
	.loc	48 0 16 is_stmt 0
	vmovaps	%xmm7, 1088(%rsp)
	vmovups	(%rcx,%rdi,4), %xmm1
.Ltmp5766:
	vmovups	(%r9,%rdi,4), %xmm5
.Ltmp5767:
	vmovaps	32(%rbx), %xmm4
	vmovaps	48(%rbx), %xmm11
	vmovaps	64(%rbx), %xmm0
	vmovaps	1360(%rbx), %xmm12
	vmovaps	1376(%rbx), %xmm9
	vmovaps	1392(%rbx), %xmm7
	vmovaps	16(%rsp), %xmm8
	vsubps	%xmm8, %xmm1, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vmovaps	32(%rsp), %xmm6
	vmovaps	%xmm4, 848(%rsp)
	vmulps	%xmm4, %xmm6, %xmm4
	vaddps	%xmm3, %xmm4, %xmm14
	vaddps	%xmm6, %xmm14, %xmm3
	vmulps	%xmm6, %xmm11, %xmm4
	vmulps	%xmm0, %xmm2, %xmm2
	vaddps	%xmm2, %xmm4, %xmm13
	vaddps	%xmm13, %xmm8, %xmm2
	vmulps	80(%rbx), %xmm3, %xmm15
	vmovaps	64(%rsp), %xmm3
	vsubps	%xmm3, %xmm2, %xmm4
	vmulps	80(%rsp), %xmm11, %xmm2
	vmovaps	%xmm4, 944(%rsp)
	vmulps	%xmm4, %xmm0, %xmm0
	vaddps	%xmm0, %xmm2, %xmm2
	vaddps	%xmm2, %xmm3, %xmm4
	vmovaps	912(%rsp), %xmm10
.Ltmp5768:
	vsubps	%xmm10, %xmm5, %xmm3
	vmulps	%xmm3, %xmm9, %xmm0
	vmovaps	992(%rsp), %xmm8
	vmovaps	%xmm12, 160(%rsp)
	vmulps	%xmm12, %xmm8, %xmm6
	vaddps	%xmm0, %xmm6, %xmm12
	vaddps	%xmm12, %xmm8, %xmm0
	vmulps	1408(%rbx), %xmm0, %xmm0
.Ltmp5769:
	.loc	1 1083 29 is_stmt 1
	movq	(%rbx), %rcx
.Ltmp5770:
	.loc	8 551 14
	vmovups	%xmm4, (%rcx,%rax,4)
.Ltmp5771:
	.loc	1 1084 30
	movq	24(%rbx), %rsi
.Ltmp5772:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_541
.Ltmp5773:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5774:
	.loc	1 0 0 is_stmt 0
	vaddps	%xmm1, %xmm15, %xmm1
	vsubps	%xmm4, %xmm1, %xmm1
.Ltmp5775:
	.loc	1 1084 30 is_stmt 1
	movq	16(%rbx), %rcx
.Ltmp5776:
	.loc	8 551 14
	vmovups	%xmm1, (%rcx,%rax,4)
.Ltmp5777:
	.loc	1 1085 28
	movq	1336(%rbx), %rsi
.Ltmp5778:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_542
.Ltmp5779:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5780:
	.loc	1 0 0 is_stmt 0
	vmulps	%xmm9, %xmm8, %xmm1
	vmulps	%xmm7, %xmm3, %xmm3
	vaddps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm10, %xmm3
	vmovaps	128(%rsp), %xmm6
	vsubps	%xmm6, %xmm3, %xmm3
	vmulps	48(%rsp), %xmm9, %xmm4
	vmulps	%xmm3, %xmm7, %xmm7
	vaddps	%xmm7, %xmm4, %xmm15
	vaddps	%xmm6, %xmm15, %xmm4
.Ltmp5781:
	.loc	1 1085 28 is_stmt 1
	movq	1328(%rbx), %rcx
.Ltmp5782:
	.loc	8 551 14
	vmovups	%xmm4, (%rcx,%rax,4)
.Ltmp5783:
	.loc	1 1086 29
	movq	1352(%rbx), %rsi
.Ltmp5784:
	.loc	48 580 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_543
.Ltmp5785:
	.loc	48 451 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5786:
	.loc	48 0 16 is_stmt 0
	movq	%rdi, 1008(%rsp)
	vaddps	%xmm0, %xmm5, %xmm0
	vsubps	%xmm4, %xmm0, %xmm0
.Ltmp5787:
	.loc	1 1086 29 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp5788:
	.loc	8 551 14
	vmovups	%xmm0, (%rcx,%rax,4)
.Ltmp5789:
	.loc	1 1089 13
	movq	(%rbx), %rdi
	movq	8(%rbx), %rsi
	movq	1104(%rbx), %rax
.Ltmp5790:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp5791:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp5792:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 15(%rsp)
	movq	%r11, 200(%rsp)
	je	.LBB34_470
.Ltmp5793:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp5794:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5795:
	.loc	1 1096 13
	movq	24(%rbx), %rsi
.Ltmp5796:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp5797:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp5798:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp5799:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5800:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%rdi,%r9,4), %xmm7
.Ltmp5801:
	movq	16(%rbx), %rcx
.Ltmp5802:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm5
.Ltmp5803:
	.loc	1 961 2
	jmp	.LBB34_479
.Ltmp5804:
	.loc	1 0 2 is_stmt 0
.Ltmp5805:
	.p2align	4
.LBB34_470:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	1112(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp5806:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp5807:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_586
	.loc	1 0 25
	movq	1120(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp5808:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movq	%rbx, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %rcx
.Ltmp5809:
	.loc	1 955 30
	leaq	2(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
	.loc	1 0 25
	movq	1128(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp5810:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp5811:
	.loc	1 955 30
	leaq	3(,%rbx,4), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_609
.Ltmp5812:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
.Ltmp5813:
	.loc	1 1096 13
	movq	24(%r13), %rsi
.Ltmp5814:
	.loc	1 857 8
	subq	%rbx, %rax
.Ltmp5815:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	1112(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp5816:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movl	$0, %r15d
	cmovaeq	%rbp, %r15
	subq	%r15, %rbx
.Ltmp5817:
	.loc	1 955 30
	leaq	1(,%rbx,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_585
	.loc	1 0 25
	movq	1120(%r13), %rbx
	.loc	1 955 35
	addq	%r10, %rbx
.Ltmp5818:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rbx
	movq	%r10, %r11
	movq	%rbp, %r10
	movl	$0, %ebp
	cmovaeq	%r10, %rbp
	subq	%rbp, %rbx
.Ltmp5819:
	.loc	1 955 30
	leaq	2(,%rbx,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	1128(%r13), %rbx
	movq	%r11, %r14
	.loc	1 955 35
	addq	%r11, %rbx
.Ltmp5820:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %rbx
	movl	$0, %r11d
	cmovaeq	%r10, %r11
	subq	%r11, %rbx
.Ltmp5821:
	.loc	1 955 30
	leaq	3(,%rbx,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp5822:
	.loc	1 0 0
	vmovd	(%rdi,%r9,4), %xmm0
	vpinsrd	$1, (%rdi,%r8,4), %xmm0, %xmm0
	vpinsrd	$2, (%rdi,%rcx,4), %xmm0, %xmm0
	vpinsrd	$3, (%rdi,%r12,4), %xmm0, %xmm7
.Ltmp5823:
	movq	16(%r13), %rcx
.Ltmp5824:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm5
	movq	%r13, %rbx
	movq	176(%rsp), %rbp
	movq	%r14, %r10
.Ltmp5825:
.LBB34_479:
	.loc	1 1103 13
	movq	1328(%rbx), %r14
	movq	1336(%rbx), %rsi
	movq	2432(%rbx), %rax
.Ltmp5826:
	.loc	1 0 0 is_stmt 0
	addq	%r10, %rax
.Ltmp5827:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	movq	%rax, %r9
	subq	%rcx, %r9
.Ltmp5828:
	.loc	1 0 0 is_stmt 0
	shlq	$2, %r9
	.loc	1 946 8 is_stmt 1
	cmpb	$0, 192(%rsp)
	je	.LBB34_485
.Ltmp5829:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%r9, %r8
	jb	.LBB34_544
.Ltmp5830:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5831:
	.loc	1 1110 13
	movq	1352(%rbx), %rsi
.Ltmp5832:
	.loc	1 857 8
	cmpq	%rbp, %rax
	movl	$0, %ecx
	cmovaeq	%rbp, %rcx
	subq	%rcx, %rax
.Ltmp5833:
	.loc	1 948 35
	shlq	$2, %rax
.Ltmp5834:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_545
.Ltmp5835:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5836:
	.loc	1 0 0 is_stmt 0
	vmovdqu	(%r14,%r9,4), %xmm6
.Ltmp5837:
	movq	1344(%rbx), %rcx
.Ltmp5838:
	.loc	8 551 14 is_stmt 1
	vmovdqu	(%rcx,%rax,4), %xmm0
.Ltmp5839:
	.loc	1 961 2
	jmp	.LBB34_494
.Ltmp5840:
	.loc	1 0 2 is_stmt 0
.Ltmp5841:
	.p2align	4
.LBB34_485:
	.loc	1 955 25 is_stmt 1
	cmpq	%r9, %rsi
	jbe	.LBB34_583
	.loc	1 0 25 is_stmt 0
	movq	2440(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp5842:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r8d
	cmovaeq	%rbp, %r8
	subq	%r8, %rcx
.Ltmp5843:
	.loc	1 955 30
	leaq	1(,%rcx,4), %r8
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_586
	.loc	1 0 25
	movq	2448(%rbx), %rcx
	.loc	1 955 35
	addq	%r10, %rcx
.Ltmp5844:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rcx
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
	subq	%r11, %rcx
.Ltmp5845:
	.loc	1 955 30
	leaq	2(,%rcx,4), %rcx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rcx
	jae	.LBB34_593
	.loc	1 0 25
	movq	2456(%rbx), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp5846:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movq	%rbx, %r15
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
.Ltmp5847:
	.loc	1 955 30
	leaq	3(,%r11,4), %r12
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_635
.Ltmp5848:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %rax
	movl	$0, %r11d
	cmovaeq	%rbp, %r11
.Ltmp5849:
	.loc	1 1110 13
	movq	1352(%r15), %rsi
.Ltmp5850:
	.loc	1 857 8
	subq	%r11, %rax
.Ltmp5851:
	.loc	1 955 30
	shlq	$2, %rax
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rax
	jae	.LBB34_581
	.loc	1 0 25
	movq	2440(%r15), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp5852:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movq	%r15, %r13
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
.Ltmp5853:
	.loc	1 955 30
	leaq	1(,%r11,4), %r15
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB34_585
	.loc	1 0 25
	movq	2448(%r13), %r11
	.loc	1 955 35
	addq	%r10, %r11
.Ltmp5854:
	.loc	1 857 8 is_stmt 1
	cmpq	%rbp, %r11
	movl	$0, %ebx
	cmovaeq	%rbp, %rbx
	subq	%rbx, %r11
	movq	%r10, %rbx
	movq	%rbp, %r10
.Ltmp5855:
	.loc	1 955 30
	leaq	2(,%r11,4), %rbp
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbp
	jae	.LBB34_603
	.loc	1 0 25
	movq	2456(%r13), %r11
	movq	%rbx, %rdi
	.loc	1 955 35
	addq	%rbx, %r11
.Ltmp5856:
	.loc	1 857 8 is_stmt 1
	cmpq	%r10, %r11
	movl	$0, %ebx
	cmovaeq	%r10, %rbx
	subq	%rbx, %r11
.Ltmp5857:
	.loc	1 955 30
	leaq	3(,%r11,4), %rbx
	.loc	1 955 25 is_stmt 0
	cmpq	%rsi, %rbx
	jae	.LBB34_598
.Ltmp5858:
	.loc	1 0 0
	vmovd	(%r14,%r9,4), %xmm0
	vpinsrd	$1, (%r14,%r8,4), %xmm0, %xmm0
	vpinsrd	$2, (%r14,%rcx,4), %xmm0, %xmm0
	vpinsrd	$3, (%r14,%r12,4), %xmm0, %xmm6
.Ltmp5859:
	movq	1344(%r13), %rcx
.Ltmp5860:
	.loc	8 551 14 is_stmt 1
	vmovd	(%rcx,%rax,4), %xmm0
	vpinsrd	$1, (%rcx,%r15,4), %xmm0, %xmm0
	vpinsrd	$2, (%rcx,%rbp,4), %xmm0, %xmm0
	vpinsrd	$3, (%rcx,%rbx,4), %xmm0, %xmm0
	movq	%r13, %rbx
	movq	176(%rsp), %rbp
	movq	%rdi, %r10
.Ltmp5861:
.LBB34_494:
	.loc	1 0 0 is_stmt 0
	negq	%rdx
	addq	%rdx, %r10
	incq	%r10
	leaq	(,%r10,4), %rax
.Ltmp5862:
	.loc	1 1150 36 is_stmt 1
	movq	8(%rbx), %rsi
.Ltmp5863:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	movq	1024(%rsp), %rdx
	movq	928(%rsp), %r9
	movq	1008(%rsp), %rdi
	movq	200(%rsp), %r11
	jb	.LBB34_546
.Ltmp5864:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5865:
	.loc	1 1152 27
	movq	24(%rbx), %rsi
.Ltmp5866:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_547
.Ltmp5867:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5868:
	.loc	1 1153 35
	movq	1336(%rbx), %rsi
.Ltmp5869:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_548
.Ltmp5870:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5871:
	.loc	1 1155 27
	movq	1352(%rbx), %rsi
.Ltmp5872:
	.loc	48 568 12
	movq	%rsi, %r8
	subq	%rax, %r8
	jb	.LBB34_550
.Ltmp5873:
	.loc	48 438 16
	cmpq	$3, %r8
	jbe	.LBB34_530
.Ltmp5874:
	.loc	48 0 16 is_stmt 0
	vmovdqa	%xmm0, 1056(%rsp)
	vaddps	%xmm14, %xmm14, %xmm0
	vaddps	32(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_35(%rip), %xmm14
	vmovdqa	%xmm5, 1040(%rsp)
	vandps	%xmm0, %xmm14, %xmm4
	vmovdqa	%xmm6, %xmm5
	vmovaps	%xmm10, %xmm6
	vmovaps	%xmm8, %xmm10
	vbroadcastss	.LCPI34_2(%rip), %xmm8
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 32(%rsp)
	vaddps	%xmm13, %xmm13, %xmm0
	vaddps	16(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm4
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 16(%rsp)
	vmulps	944(%rsp), %xmm11, %xmm0
	vmovaps	80(%rsp), %xmm11
	vmulps	848(%rsp), %xmm11, %xmm4
	vaddps	%xmm0, %xmm4, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm11, %xmm0
	vandps	%xmm0, %xmm14, %xmm4
	vcmpltps	%xmm8, %xmm4, %xmm4
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 80(%rsp)
	vaddps	%xmm2, %xmm2, %xmm0
	vaddps	64(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 64(%rsp)
.Ltmp5875:
	vaddps	%xmm12, %xmm12, %xmm0
	vaddps	%xmm0, %xmm10, %xmm0
	vandps	%xmm0, %xmm14, %xmm2
	vcmpltps	%xmm8, %xmm2, %xmm2
	vandnps	%xmm0, %xmm2, %xmm0
	vmovaps	%xmm0, 992(%rsp)
	vaddps	%xmm1, %xmm1, %xmm0
	vaddps	%xmm0, %xmm6, %xmm0
	vandps	%xmm0, %xmm14, %xmm1
	vcmpltps	%xmm8, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 912(%rsp)
	vmulps	%xmm3, %xmm9, %xmm0
	vmovaps	48(%rsp), %xmm2
	vmulps	160(%rsp), %xmm2, %xmm1
	vaddps	%xmm0, %xmm1, %xmm0
	vaddps	%xmm0, %xmm0, %xmm0
	vaddps	%xmm0, %xmm2, %xmm0
	vandps	%xmm0, %xmm14, %xmm1
	vcmpltps	%xmm8, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 48(%rsp)
	vaddps	%xmm15, %xmm15, %xmm0
	vaddps	128(%rsp), %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm1
	vcmpltps	%xmm8, %xmm1, %xmm1
	vandnps	%xmm0, %xmm1, %xmm0
	vmovaps	%xmm0, 128(%rsp)
.Ltmp5876:
	vpand	%xmm7, %xmm14, %xmm0
	vpand	%xmm5, %xmm14, %xmm1
	vbroadcastss	.LCPI34_3(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
.Ltmp5877:
	vandps	1040(%rsp), %xmm14, %xmm1
	vandps	1056(%rsp), %xmm14, %xmm2
	vmulps	%xmm3, %xmm1, %xmm1
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm1, %xmm2
	vbroadcastss	.LCPI34_4(%rip), %xmm5
.Ltmp5878:
	vmaxps	%xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_5(%rip), %xmm6
	vmaxps	%xmm6, %xmm0, %xmm0
	vbroadcastss	.LCPI34_36(%rip), %xmm7
	vandps	%xmm7, %xmm0, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm9
	vorps	%xmm1, %xmm9, %xmm1
	vbroadcastss	.LCPI34_8(%rip), %xmm10
	vaddps	%xmm1, %xmm10, %xmm1
	vbroadcastss	.LCPI34_9(%rip), %xmm11
	vmulps	%xmm1, %xmm11, %xmm3
	vbroadcastss	.LCPI34_10(%rip), %xmm12
	vaddps	%xmm3, %xmm12, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_11(%rip), %xmm13
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_12(%rip), %xmm15
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI34_15(%rip), %xmm4
	vpor	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm1, %xmm1
	vaddps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vsubps	208(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm1
	vaddps	%xmm1, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vbroadcastss	.LCPI34_22(%rip), %xmm5
	vmulps	%xmm5, %xmm3, %xmm3
	vcmpltps	%xmm0, %xmm1, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vbroadcastss	.LCPI34_21(%rip), %xmm7
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1216(%rsp), %xmm3, %xmm3
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm0, %xmm1, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm3
	vmaxps	%xmm3, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vmovaps	864(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm3
	vmovaps	1184(%rsp), %xmm4
	vblendvps	%xmm3, 1200(%rsp), %xmm4, %xmm3
	vsubps	%xmm0, %xmm1, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm3
	vcmpltps	%xmm8, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm1
.Ltmp5879:
	vbroadcastss	.LCPI34_4(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm0
	vmaxps	%xmm6, %xmm0, %xmm0
	vandps	.LCPI34_6(%rip), %xmm0, %xmm2
	vorps	%xmm2, %xmm9, %xmm2
	vaddps	%xmm2, %xmm10, %xmm2
	vmulps	%xmm2, %xmm11, %xmm3
	vaddps	%xmm3, %xmm12, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm13, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vaddps	%xmm3, %xmm15, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI34_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
	vpsrld	$23, %xmm0, %xmm0
	vpor	.LCPI34_15(%rip), %xmm0, %xmm0
	vbroadcastss	.LCPI34_16(%rip), %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm2, %xmm2
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_17(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_18(%rip), %xmm2
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_19(%rip), %xmm2
	vminps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 944(%rsp)
	vsubps	288(%rsp), %xmm0, %xmm0
	vbroadcastss	.LCPI34_20(%rip), %xmm3
	vaddps	%xmm3, %xmm0, %xmm2
	vmulps	%xmm2, %xmm2, %xmm2
	vmulps	%xmm5, %xmm2, %xmm2
	vcmpltps	%xmm0, %xmm3, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm2, %xmm2
	vcmpleps	%xmm7, %xmm0, %xmm0
	vmulps	1168(%rsp), %xmm2, %xmm2
	vxorps	%xmm3, %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm3, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm1, 864(%rsp)
.Ltmp5880:
	vaddps	272(%rsp), %xmm1, %xmm2
	vbroadcastss	.LCPI34_24(%rip), %xmm10
	vmulps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI34_25(%rip), %xmm11
	vmaxps	%xmm11, %xmm2, %xmm2
	vbroadcastss	.LCPI34_26(%rip), %xmm11
	vminps	%xmm11, %xmm2, %xmm2
	vroundps	$9, %xmm2, %xmm4
	vsubps	%xmm4, %xmm2, %xmm2
	vbroadcastss	.LCPI34_27(%rip), %xmm13
	vmulps	%xmm2, %xmm13, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm15
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_29(%rip), %xmm7
	vaddps	%xmm7, %xmm5, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm8
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm9
	vaddps	%xmm5, %xmm9, %xmm5
	vbroadcastss	.LCPI34_23(%rip), %xmm13
.Ltmp5881:
	vmaxps	%xmm13, %xmm0, %xmm0
	vminps	%xmm3, %xmm0, %xmm0
	vmovaps	880(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm6
	vmovaps	1136(%rsp), %xmm10
	vblendvps	%xmm6, 1152(%rsp), %xmm10, %xmm6
.Ltmp5882:
	vmulps	%xmm5, %xmm2, %xmm2
.Ltmp5883:
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm6, %xmm5, %xmm5
	vaddps	%xmm5, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm5
	vbroadcastss	.LCPI34_2(%rip), %xmm12
	vcmpltps	%xmm12, %xmm5, %xmm5
	vandnps	%xmm0, %xmm5, %xmm1
	vbroadcastss	.LCPI34_32(%rip), %xmm6
.Ltmp5884:
	vaddps	%xmm6, %xmm2, %xmm0
	vbroadcastss	.LCPI34_33(%rip), %xmm10
	vaddps	%xmm4, %xmm10, %xmm2
	vpslld	$23, %xmm2, %xmm2
	vmovaps	%xmm1, 880(%rsp)
.Ltmp5885:
	vaddps	352(%rsp), %xmm1, %xmm4
.Ltmp5886:
	vmulps	%xmm2, %xmm0, %xmm0
	vmovaps	%xmm0, 848(%rsp)
	vbroadcastss	.LCPI34_24(%rip), %xmm2
.Ltmp5887:
	vmulps	%xmm2, %xmm4, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm11, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm3
	vmulps	%xmm3, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm7, %xmm5, %xmm5
	vmovaps	%xmm7, %xmm15
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm8, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm9, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm0
	vmovaps	%xmm6, %xmm8
	vaddps	%xmm4, %xmm10, %xmm4
	vmovaps	%xmm10, %xmm9
	vpslld	$23, %xmm4, %xmm4
	vmovaps	160(%rsp), %xmm1
.Ltmp5888:
	vsubps	528(%rsp), %xmm1, %xmm5
.Ltmp5889:
	vmulps	%xmm4, %xmm0, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vbroadcastss	.LCPI34_20(%rip), %xmm6
.Ltmp5890:
	vaddps	%xmm6, %xmm5, %xmm0
	vmulps	%xmm0, %xmm0, %xmm0
	vbroadcastss	.LCPI34_22(%rip), %xmm11
	vmulps	%xmm0, %xmm11, %xmm0
	vcmpltps	%xmm5, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm5, %xmm0, %xmm0
	vbroadcastss	.LCPI34_21(%rip), %xmm10
	vcmpleps	%xmm10, %xmm5, %xmm4
	vmulps	1120(%rsp), %xmm0, %xmm0
	vxorps	%xmm1, %xmm1, %xmm1
	vpcmpgtd	%xmm4, %xmm1, %xmm4
	vpandn	%xmm0, %xmm4, %xmm0
	vmaxps	%xmm13, %xmm0, %xmm0
	vminps	%xmm1, %xmm0, %xmm0
	vxorps	%xmm7, %xmm7, %xmm7
	vmovaps	896(%rsp), %xmm1
	vcmpltps	%xmm1, %xmm0, %xmm4
	vmovaps	1280(%rsp), %xmm5
	vblendvps	%xmm4, 1296(%rsp), %xmm5, %xmm4
	vsubps	%xmm0, %xmm1, %xmm5
	vmulps	%xmm4, %xmm5, %xmm4
	vaddps	%xmm4, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm4
	vcmpltps	%xmm12, %xmm4, %xmm4
	vmovaps	%xmm12, %xmm13
	vandnps	%xmm0, %xmm4, %xmm0
	vmovaps	%xmm0, 896(%rsp)
	vaddps	592(%rsp), %xmm0, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI34_25(%rip), %xmm12
	vmaxps	%xmm12, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm4
	vsubps	%xmm4, %xmm0, %xmm0
	vmulps	%xmm3, %xmm0, %xmm5
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vaddps	%xmm5, %xmm15, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_30(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm5
	vbroadcastss	.LCPI34_31(%rip), %xmm1
	vaddps	%xmm1, %xmm5, %xmm5
	vmulps	%xmm5, %xmm0, %xmm0
	vaddps	%xmm0, %xmm8, %xmm0
	vaddps	%xmm4, %xmm9, %xmm4
	vpslld	$23, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm5
	vmovaps	944(%rsp), %xmm0
.Ltmp5891:
	vsubps	608(%rsp), %xmm0, %xmm0
	vaddps	%xmm6, %xmm0, %xmm3
	vmulps	%xmm3, %xmm3, %xmm3
	vmulps	%xmm3, %xmm11, %xmm3
	vcmpltps	%xmm0, %xmm6, %xmm4
	vblendvps	%xmm4, %xmm0, %xmm3, %xmm3
	vcmpleps	%xmm10, %xmm0, %xmm0
	vmulps	1264(%rsp), %xmm3, %xmm3
	vpcmpgtd	%xmm0, %xmm7, %xmm0
	vpandn	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_23(%rip), %xmm1
	vmaxps	%xmm1, %xmm0, %xmm0
	vminps	%xmm7, %xmm0, %xmm0
	vmovaps	1088(%rsp), %xmm4
	vcmpltps	%xmm4, %xmm0, %xmm3
	vmovaps	1232(%rsp), %xmm1
	vblendvps	%xmm3, 1248(%rsp), %xmm1, %xmm3
	vsubps	%xmm0, %xmm4, %xmm4
	vmulps	%xmm3, %xmm4, %xmm3
	vaddps	%xmm3, %xmm0, %xmm0
	vandps	%xmm0, %xmm14, %xmm3
	vcmpltps	%xmm13, %xmm3, %xmm3
	vandnps	%xmm0, %xmm3, %xmm7
	vaddps	672(%rsp), %xmm7, %xmm0
	vmulps	%xmm2, %xmm0, %xmm0
	vmaxps	%xmm12, %xmm0, %xmm0
	vbroadcastss	.LCPI34_26(%rip), %xmm1
	vminps	%xmm1, %xmm0, %xmm0
	vroundps	$9, %xmm0, %xmm3
	vsubps	%xmm3, %xmm0, %xmm0
	vbroadcastss	.LCPI34_27(%rip), %xmm1
	vmulps	%xmm1, %xmm0, %xmm4
	vbroadcastss	.LCPI34_28(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vaddps	%xmm4, %xmm15, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_30(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm4
	vbroadcastss	.LCPI34_31(%rip), %xmm1
	vaddps	%xmm1, %xmm4, %xmm4
	vmulps	%xmm4, %xmm0, %xmm0
	vaddps	%xmm0, %xmm8, %xmm0
	vaddps	%xmm3, %xmm9, %xmm3
	vpslld	$23, %xmm3, %xmm3
	vmulps	%xmm3, %xmm0, %xmm0
.Ltmp5892:
	movq	(%rbx), %rcx
	vmovaps	848(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm2
	movq	16(%rbx), %rcx
	vmovaps	160(%rsp), %xmm1
	vmulps	(%rcx,%rax,4), %xmm1, %xmm1
	vaddps	%xmm1, %xmm2, %xmm1
.Ltmp5893:
	movq	1328(%rbx), %rcx
	vmulps	(%rcx,%rax,4), %xmm5, %xmm2
	.loc	1 1155 27 is_stmt 1
	movq	1344(%rbx), %rcx
.Ltmp5894:
	.loc	9 88 14
	vmulps	(%rcx,%rax,4), %xmm0, %xmm0
.Ltmp5895:
	.loc	9 36 14
	vaddps	%xmm0, %xmm2, %xmm0
	movq	976(%rsp), %rcx
.Ltmp5896:
	.loc	8 551 14
	vmovups	%xmm1, (%rcx,%rdi,4)
.Ltmp5897:
	.loc	8 551 14 is_stmt 0
	vmovups	%xmm0, (%r9,%rdi,4)
.Ltmp5898:
	.loc	1 0 0
	incq	%r11
.Ltmp5899:
	.loc	2 1916 50 is_stmt 1
	addq	$4, %rdi
	cmpq	%r11, %rdx
.Ltmp5900:
	.loc	3 900 12
	jne	.LBB34_454
	jmp	.LBB34_388
.Ltmp5901:
.LBB34_503:
	.loc	3 0 12 is_stmt 0
	movabsq	$2305843009213693948, %rax
.Ltmp5902:
	.loc	9 504 14 is_stmt 1
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqps	%xmm0, %xmm0, %xmm0
	movq	112(%rsp), %rdx
.Ltmp5903:
	.loc	10 2155 12
	movq	%rdx, %rcx
	vmovaps	%xmm0, %xmm1
	andq	%rax, %rcx
	movq	968(%rsp), %rdi
	je	.LBB34_506
.Ltmp5904:
	.loc	10 0 12 is_stmt 0
	xorl	%esi, %esi
	vbroadcastss	.LCPI34_35(%rip), %xmm2
	vbroadcastss	.LCPI34_34(%rip), %xmm3
	vmovaps	%xmm0, %xmm1
	.p2align	4
.LBB34_505:
.Ltmp5905:
	.loc	9 257 24 is_stmt 1
	vandps	(%rdi,%rsi,4), %xmm2, %xmm4
.Ltmp5906:
	.loc	9 517 14
	vcmpltps	%xmm3, %xmm4, %xmm4
.Ltmp5907:
	.loc	9 257 24
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp5908:
	.loc	10 2155 12
	addq	$4, %rsi
	cmpq	%rsi, %rcx
	jne	.LBB34_505
.Ltmp5909:
.LBB34_506:
	.loc	13 285 9
	vpcmpeqd	%xmm2, %xmm2, %xmm2
	vtestps	%xmm2, %xmm1
	movq	104(%rsp), %r14
.Ltmp5910:
	.loc	50 208 8
	jae	.LBB34_511
.Ltmp5911:
	.loc	10 2155 12
	movq	%r14, %r8
	vmovaps	%xmm0, %xmm1
	andq	%rax, %r8
	je	.LBB34_510
.Ltmp5912:
	.loc	10 0 12 is_stmt 0
	xorl	%esi, %esi
	vbroadcastss	.LCPI34_35(%rip), %xmm2
	vbroadcastss	.LCPI34_34(%rip), %xmm3
	vmovaps	%xmm0, %xmm1
	.p2align	4
.LBB34_509:
.Ltmp5913:
	.loc	9 257 24 is_stmt 1
	vandps	(%r9,%rsi,4), %xmm2, %xmm4
.Ltmp5914:
	.loc	9 517 14
	vcmpltps	%xmm3, %xmm4, %xmm4
.Ltmp5915:
	.loc	9 257 24
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp5916:
	.loc	10 2155 12
	addq	$4, %rsi
	cmpq	%rsi, %r8
	jne	.LBB34_509
.Ltmp5917:
.LBB34_510:
	.loc	13 285 9
	vpcmpeqd	%xmm2, %xmm2, %xmm2
	vtestps	%xmm2, %xmm1
.Ltmp5918:
	.loc	50 208 34
	jb	.LBB34_526
.LBB34_511:
	.loc	50 0 34 is_stmt 0
	vmovaps	%xmm0, %xmm1
.Ltmp5919:
	.loc	10 2155 12 is_stmt 1
	testq	%rcx, %rcx
.Ltmp5920:
	.loc	10 2155 12 is_stmt 0
	je	.LBB34_514
.Ltmp5921:
	.loc	10 0 12
	xorl	%esi, %esi
	vbroadcastss	.LCPI34_35(%rip), %xmm2
	vbroadcastss	.LCPI34_34(%rip), %xmm3
	vmovaps	%xmm0, %xmm1
	.p2align	4
.LBB34_513:
.Ltmp5922:
	.loc	9 257 24 is_stmt 1
	vandps	(%rdi,%rsi,4), %xmm2, %xmm4
.Ltmp5923:
	.loc	9 517 14
	vcmpltps	%xmm3, %xmm4, %xmm4
.Ltmp5924:
	.loc	9 257 24
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp5925:
	.loc	10 2155 12
	addq	$4, %rsi
	cmpq	%rsi, %rcx
	jne	.LBB34_513
.Ltmp5926:
.LBB34_514:
	.file	55 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/sse41.rs"
	.loc	55 131 19
	vpsrad	$31, %xmm1, %xmm2
	vpbroadcastd	.LCPI34_32(%rip), %xmm1
	vpandn	%xmm1, %xmm2, %xmm2
.Ltmp5927:
	.loc	50 185 12
	vmovd	%xmm2, %ecx
	xorl	%ebp, %ebp
	testl	%ecx, %ecx
	setne	%bpl
	vpextrd	$1, %xmm2, %esi
	xorl	%ecx, %ecx
	testl	%esi, %esi
	setne	%cl
	vpextrd	$2, %xmm2, %r8d
	addl	%ecx, %ecx
	xorl	%esi, %esi
	testl	%r8d, %r8d
	setne	%sil
	shll	$2, %esi
	vpextrd	$3, %xmm2, %r8d
	xorl	%r15d, %r15d
	testl	%r8d, %r8d
	setne	%r15b
	shll	$3, %r15d
.Ltmp5928:
	.loc	10 2155 12
	andq	%r14, %rax
	je	.LBB34_517
.Ltmp5929:
	.loc	10 0 12 is_stmt 0
	xorl	%r8d, %r8d
	vbroadcastss	.LCPI34_35(%rip), %xmm2
	vbroadcastss	.LCPI34_34(%rip), %xmm3
	.p2align	4
.LBB34_516:
.Ltmp5930:
	.loc	9 257 24 is_stmt 1
	vandps	(%r9,%r8,4), %xmm2, %xmm4
.Ltmp5931:
	.loc	9 517 14
	vcmpltps	%xmm3, %xmm4, %xmm4
.Ltmp5932:
	.loc	9 257 24
	vandps	%xmm4, %xmm0, %xmm0
.Ltmp5933:
	.loc	10 2155 12
	addq	$4, %r8
	cmpq	%r8, %rax
	jne	.LBB34_516
.Ltmp5934:
.LBB34_517:
	.loc	55 131 19
	vpsrad	$31, %xmm0, %xmm0
	vpandn	%xmm1, %xmm0, %xmm0
.Ltmp5935:
	.loc	50 185 12
	vmovd	%xmm0, %r8d
	xorl	%eax, %eax
	testl	%r8d, %r8d
	vpextrd	$1, %xmm0, %r9d
	setne	%al
	xorl	%r8d, %r8d
	testl	%r9d, %r9d
	setne	%r8b
	addl	%r8d, %r8d
	vpextrd	$2, %xmm0, %r9d
	xorl	%r10d, %r10d
	testl	%r9d, %r9d
	setne	%r10b
	vpextrd	$3, %xmm0, %r9d
	shll	$2, %r10d
	xorl	%r11d, %r11d
	testl	%r9d, %r9d
	setne	%r11b
	shll	$3, %r11d
.Ltmp5936:
	.loc	50 185 12 is_stmt 0
	orl	%r10d, %r11d
.Ltmp5937:
	.loc	50 185 12
	orl	%ebp, %ecx
	orl	%esi, %ecx
	orl	%r15d, %ecx
.Ltmp5938:
	.loc	50 185 12
	orl	%eax, %ecx
	orl	%r8d, %ecx
.Ltmp5939:
	.loc	50 211 5 is_stmt 1
	orl	%r11d, %ecx
	movl	%ecx, 2664(%rbx)
	.loc	50 212 31
	movq	2656(%rbx), %rax
.Ltmp5940:
	.loc	38 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp5941:
	.loc	50 212 5
	movq	%rcx, 2656(%rbx)
.Ltmp5942:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp5943:
	.loc	33 180 28
	je	.LBB34_519
.Ltmp5944:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp5945:
	.loc	35 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp5946:
.LBB34_519:
	.loc	32 1714 9
	testq	%r14, %r14
.Ltmp5947:
	.loc	33 180 28
	je	.LBB34_521
.Ltmp5948:
	.loc	34 961 18
	shlq	$2, %r14
	movq	120(%rsp), %rdi
.Ltmp5949:
	.loc	35 25 13
	xorl	%esi, %esi
	movq	%r14, %rdx
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp5950:
.LBB34_521:
	.loc	1 1422 13
	movq	$0, 2680(%rbx)
.Ltmp5951:
	.loc	1 1424 22
	movq	%rbx, %rdi
	vzeroupper
	callq	_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E19discontinuity_resetB5_
.Ltmp5952:
	.loc	32 1714 9
	leaq	1328(%rbx), %rdi
.Ltmp5953:
	.loc	1 1424 22
	callq	_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SideNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_E19discontinuity_resetB5_
.Ltmp5954:
	.loc	1 1430 20
	movl	2664(%rbx), %eax
.Ltmp5955:
	.loc	1 1433 16
	testb	$1, %al
	movq	984(%rsp), %rsi
	jne	.LBB34_527
	testb	$2, %al
	jne	.LBB34_528
.LBB34_523:
	testb	$4, %al
	jne	.LBB34_529
.LBB34_524:
	testb	$8, %al
	je	.LBB34_526
.LBB34_525:
	.loc	1 0 16 is_stmt 0
	movq	1696(%rsp), %rax
.Ltmp5956:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rax
	movq	$-1, %rcx
	cmovbq	%rcx, %rax
.Ltmp5957:
	.loc	1 1434 17
	movq	%rax, 1696(%rsp)
.Ltmp5958:
	.loc	38 2428 13
	addq	1704(%rsp), %rsi
	cmovbq	%rcx, %rsi
.Ltmp5959:
	.loc	1 1435 17
	movq	%rsi, 1704(%rsp)
.Ltmp5960:
.LBB34_526:
	.loc	1 0 17 is_stmt 0
	leaq	1552(%rsp), %rsi
	movl	$328, %edx
	movq	1320(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp5961:
	.loc	1 2013 6 is_stmt 1
	movq	%rbx, %rax
	.loc	1 2013 6 epilogue_begin is_stmt 0
	addq	$2280, %rsp
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
.LBB34_527:
	.cfi_def_cfa_offset 2336
	.loc	1 0 6
	movq	1576(%rsp), %rcx
.Ltmp5962:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp5963:
	.loc	1 1434 17
	movq	%rcx, 1576(%rsp)
	movq	1584(%rsp), %rcx
.Ltmp5964:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp5965:
	.loc	1 1435 17
	movq	%rcx, 1584(%rsp)
	.loc	1 1433 16
	testb	$2, %al
	je	.LBB34_523
.LBB34_528:
	.loc	1 0 16 is_stmt 0
	movq	1616(%rsp), %rcx
.Ltmp5966:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp5967:
	.loc	1 1434 17
	movq	%rcx, 1616(%rsp)
	movq	1624(%rsp), %rcx
.Ltmp5968:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp5969:
	.loc	1 1435 17
	movq	%rcx, 1624(%rsp)
	.loc	1 1433 16
	testb	$4, %al
	je	.LBB34_524
.LBB34_529:
	.loc	1 0 16 is_stmt 0
	movq	1656(%rsp), %rcx
.Ltmp5970:
	.loc	38 2428 13 is_stmt 1
	addq	%rsi, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp5971:
	.loc	1 1434 17
	movq	%rcx, 1656(%rsp)
	movq	1664(%rsp), %rcx
.Ltmp5972:
	.loc	38 2428 13
	addq	%rsi, %rcx
	cmovbq	%rdx, %rcx
.Ltmp5973:
	.loc	1 1435 17
	movq	%rcx, 1664(%rsp)
	.loc	1 1433 16
	testb	$8, %al
	jne	.LBB34_525
	jmp	.LBB34_526
.Ltmp5974:
.LBB34_530:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_9dcceffc0d89ad4d6a9a2e0685e8ac95(%rip), %rcx
	movl	$4, %esi
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_531:
.Ltmp5975:
	leaq	.Lalloc_9dcceffc0d89ad4d6a9a2e0685e8ac95(%rip), %rcx
	movl	$4, %esi
	xorl	%edi, %edi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5976:
.LBB34_534:
	leaq	.Lalloc_9dcceffc0d89ad4d6a9a2e0685e8ac95(%rip), %rcx
	movl	$4, %esi
	xorl	%edi, %edi
	xorl	%edx, %edx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5977:
.LBB34_533:
	.loc	48 443 13 is_stmt 1
	leaq	.Lalloc_b02e35ee2c207cc88671eb1977dbcd5b(%rip), %rcx
	movq	48(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5978:
.LBB34_535:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_536:
	leaq	.Lalloc_1f724420e117514d5eeca145f523ec40(%rip), %rcx
.Ltmp5979:
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_537:
.Ltmp5980:
	leaq	.Lalloc_ebd89d0b93275e5686fc2b41f2e9561d(%rip), %rcx
.Ltmp5981:
	movq	%rax, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_538:
.Ltmp5982:
	leaq	.Lalloc_89de14a4db9a7a5e199f0b3432909247(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_539:
	leaq	.Lalloc_e4c416c2b2b3df4fd71341a77882ada1(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5983:
.LBB34_540:
	leaq	.Lalloc_7515f6adc8a5521b72f04a3abb372e72(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_541:
	leaq	.Lalloc_8c2aade3368450b16e7e4997ad3fca81(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_542:
	leaq	.Lalloc_a4c3da5a99763e9452b49d42852d67e3(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_543:
	leaq	.Lalloc_e11ba0c4c1124bbcae4603a2ae121338(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_544:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r9, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_545:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_546:
	leaq	.Lalloc_379b93204ee4659b79f4b07b6520076a(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_547:
	leaq	.Lalloc_2bb8eb0542a889f29ef069491eb97a17(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_548:
	leaq	.Lalloc_cd96798e807157d7db308788cb4dd125(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_550:
	leaq	.Lalloc_164adf6876ccf79975d46e129e248ca5(%rip), %rcx
	movq	%rax, %rdi
	movq	%rsi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_563:
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	152(%rsp), %rdx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_551:
.Ltmp5984:
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	192(%rsp), %rdx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5985:
.LBB34_565:
	leaq	.Lalloc_bd312be13bed481ea06b89305a878dc2(%rip), %rcx
	movq	%r10, %rdi
	movq	152(%rsp), %rdx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_566:
.Ltmp5986:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_7ac5156198d2516c0e2a17140923b083(%rip), %rcx
.Ltmp5987:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	112(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5988:
.LBB34_567:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_683f9160cdc5ee4596b2ecf709188a9a(%rip), %rcx
.Ltmp5989:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	112(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5990:
.LBB34_570:
	.loc	1 0 0
	leaq	.Lalloc_683f9160cdc5ee4596b2ecf709188a9a(%rip), %rcx
	movq	%r15, %rdi
	movq	112(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_571:
	leaq	.Lalloc_7ac5156198d2516c0e2a17140923b083(%rip), %rcx
	movq	%r15, %rdi
	movq	112(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_568:
.Ltmp5991:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_94bf6a39a1d7f90452d19d4598bc6c91(%rip), %rcx
.Ltmp5992:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	104(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5993:
.LBB34_569:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_5337a4cbd2266e89524aaf9670b5f666(%rip), %rcx
.Ltmp5994:
	.loc	48 456 13 is_stmt 0
	movq	%r13, %rdi
	movq	104(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5995:
.LBB34_577:
	.loc	1 0 0
	leaq	.Lalloc_5337a4cbd2266e89524aaf9670b5f666(%rip), %rcx
	movq	%r15, %rdi
	movq	104(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_576:
	leaq	.Lalloc_94bf6a39a1d7f90452d19d4598bc6c91(%rip), %rcx
	movq	%r15, %rdi
	movq	104(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp5996:
.LBB34_578:
	.loc	1 1995 25 is_stmt 1
	leaq	.Lalloc_e163651fd3b09506777efc3802c70e08(%rip), %rdx
	movq	%r8, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_579:
.Ltmp5997:
	.loc	1 1996 23
	leaq	.Lalloc_4833734f0e99b6856ca3d83ba3e6f90c(%rip), %rdx
	movq	%rcx, %rdi
	movq	%r8, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp5998:
.LBB34_598:
	.loc	1 0 23 is_stmt 0
	movq	%rbx, %rax
.Ltmp5999:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_603:
	movq	%rbp, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_593:
	movq	%rcx, %r9
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_580:
	movq	%r15, %rax
.LBB34_581:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_582:
	movq	%r8, %r9
.LBB34_583:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_601:
	movq	%r11, %r9
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6000:
.LBB34_584:
	.loc	1 1479 43 is_stmt 1
	leaq	.Lalloc_618452d81e57e47d125c0afad7fc5006(%rip), %rdx
	movl	$12, %esi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6001:
.LBB34_609:
	.loc	1 0 43 is_stmt 0
	movq	%r12, %r9
.Ltmp6002:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_585:
	movq	%r15, %rax
.Ltmp6003:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_586:
	movq	%r8, %r9
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6004:
.LBB34_587:
	.loc	1 1481 45 is_stmt 1
	leaq	.Lalloc_97e1583d528f48ca4578bc0e35112ce6(%rip), %rdx
	movl	$10, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6005:
.LBB34_607:
	.loc	1 0 45 is_stmt 0
	movq	%rdx, %r9
.Ltmp6006:
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_625:
	movq	%rdi, %r9
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_631:
	movq	%rbp, %rax
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_635:
	movq	%r12, %r9
	leaq	.Lalloc_368cec9f5031d82ea4f868eab84dd17d(%rip), %rdx
	movq	%r9, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp6007:
.Lfunc_end34:
	.size	_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_, .Lfunc_end34-_RNvXs7_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_31PreparedMultibandCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kj4_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_
