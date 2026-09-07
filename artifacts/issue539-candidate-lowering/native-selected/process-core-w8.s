_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_:
.Lfunc_begin34:
	.loc	1 2191 0
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
	subq	$4096, %rsp
	movq	$0, (%rsp)
	subq	$3168, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%r8, %r13
	movq	%rcx, 408(%rsp)
	movq	%rdx, 24(%rsp)
	movq	%rsi, 344(%rsp)
	movq	%rdi, %r11
	movq	%r9, 16(%rsp)
.Ltmp7177:
	.loc	1 2192 21 prologue_end
	leaq	(,%r9,8), %rax
	movq	%rax, 4336(%rsp)
.Ltmp7178:
	.loc	1 2210 21
	movzbl	2153(%rdi), %ecx
	.loc	1 0 0 is_stmt 0
	movq	1784(%rdi), %rax
	.loc	1 2210 21
	cmpb	2144(%rdi), %cl
	jne	.LBB34_1
	.loc	1 2211 37 is_stmt 1
	movq	1776(%r11), %rcx
.Ltmp7179:
	.loc	6 314 17
	movq	%rax, %rdx
	shlq	$4, %rdx
	movq	%rcx, %rsi
	.loc	6 0 17 is_stmt 0
.Ltmp7180:
	.p2align	4
.LBB34_3:
.Ltmp7181:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp7182:
	.loc	6 180 28
	je	.LBB34_6
.Ltmp7183:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp7184:
	.loc	6 315 25
	jne	.LBB34_30
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB34_3
	jmp	.LBB34_30
.Ltmp7185:
.LBB34_1:
	.loc	1 707 26 is_stmt 1
	movq	1776(%r11), %rcx
.Ltmp7186:
.LBB34_30:
	.loc	1 0 26 is_stmt 0
	xorl	%r9d, %r9d
.LBB34_31:
	.loc	1 2246 13 is_stmt 1
	leaq	1616(%r11), %r8
	.loc	1 2247 13
	leaq	1648(%r11), %rdx
	movq	%rdx, 536(%rsp)
	.loc	1 2248 13
	leaq	1848(%r11), %rdx
	movq	%rdx, 528(%rsp)
.Ltmp7187:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp7188:
	.p2align	4
.LBB34_32:
.Ltmp7189:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7190:
	.loc	6 180 28
	je	.LBB34_37
.Ltmp7191:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp7192:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
	jne	.LBB34_50
.Ltmp7193:
	.loc	6 315 25
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB34_32
	jmp	.LBB34_50
.Ltmp7194:
.LBB34_37:
	.loc	1 708 33 is_stmt 1
	movq	1792(%r11), %rcx
	movq	1800(%r11), %rax
.Ltmp7195:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp7196:
	.p2align	4
.LBB34_38:
.Ltmp7197:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7198:
	.loc	6 180 28
	je	.LBB34_41
.Ltmp7199:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp7200:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7201:
	.loc	6 315 25
	jne	.LBB34_50
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB34_38
	jmp	.LBB34_50
.Ltmp7202:
.LBB34_41:
	.loc	1 709 33 is_stmt 1
	movq	1976(%r11), %rcx
	movq	1984(%r11), %rax
.Ltmp7203:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp7204:
	.p2align	4
.LBB34_42:
.Ltmp7205:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7206:
	.loc	6 180 28
	je	.LBB34_45
.Ltmp7207:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp7208:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7209:
	.loc	6 315 25
	jne	.LBB34_50
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB34_42
	jmp	.LBB34_50
.Ltmp7210:
.LBB34_6:
	.loc	1 2212 37 is_stmt 1
	movq	1792(%r11), %rsi
	movq	1800(%r11), %rdx
.Ltmp7211:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp7212:
	.p2align	4
.LBB34_7:
.Ltmp7213:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp7214:
	.loc	6 180 28
	je	.LBB34_10
.Ltmp7215:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp7216:
	.loc	6 315 25
	jne	.LBB34_30
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB34_7
	jmp	.LBB34_30
.Ltmp7217:
.LBB34_45:
	.loc	1 710 33 is_stmt 1
	movq	1992(%r11), %rcx
	movq	2000(%r11), %rax
.Ltmp7218:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp7219:
	.p2align	4
.LBB34_46:
.Ltmp7220:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7221:
	.loc	6 180 28
	je	.LBB34_47
.Ltmp7222:
	.loc	6 0 28 is_stmt 0
	xorl	%edx, %edx
.Ltmp7223:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7224:
	.loc	6 315 25
	jne	.LBB34_50
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB34_46
	jmp	.LBB34_50
.Ltmp7225:
.LBB34_10:
	.loc	1 2213 37 is_stmt 1
	movq	1976(%r11), %rsi
	movq	1984(%r11), %rdx
.Ltmp7226:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp7227:
	.p2align	4
.LBB34_11:
.Ltmp7228:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp7229:
	.loc	6 180 28
	je	.LBB34_14
.Ltmp7230:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp7231:
	.loc	6 315 25
	jne	.LBB34_30
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB34_11
	jmp	.LBB34_30
.Ltmp7232:
.LBB34_47:
	.loc	6 0 25
	movb	$1, %dl
.LBB34_50:
.Ltmp7233:
	.loc	1 1118 5 is_stmt 1
	movq	1832(%r11), %rcx
	testq	%rcx, %rcx
	movq	%r11, 8(%rsp)
	movq	%r13, 248(%rsp)
	movq	%r8, 4344(%rsp)
	movl	%r9d, 2588(%rsp)
	je	.LBB34_56
	.loc	1 0 5 is_stmt 0
	movq	1824(%r11), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rsi
	.p2align	4
.LBB34_52:
.Ltmp7234:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp7235:
	.loc	6 180 28
	je	.LBB34_56
.Ltmp7236:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rax), %edi
	jne	.LBB34_70
	movl	4(%rsi), %edi
	cmpl	4(%rax), %edi
	jne	.LBB34_70
	movl	8(%rsi), %edi
.Ltmp7237:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp7238:
	.loc	1 390 34
	cmpl	8(%rax), %edi
.Ltmp7239:
	.loc	6 315 25
	je	.LBB34_52
	jmp	.LBB34_70
.Ltmp7240:
.LBB34_56:
	.loc	1 1119 12
	movq	1768(%r11), %rax
	testq	%rax, %rax
	je	.LBB34_60
	.loc	1 0 12 is_stmt 0
	movq	1760(%r11), %rcx
	shlq	$2, %rax
	xorl	%esi, %esi
	.p2align	4
.LBB34_58:
.Ltmp7241:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rax
.Ltmp7242:
	.loc	6 180 28
	je	.LBB34_60
.Ltmp7243:
	.loc	6 315 25
	movl	(%rcx,%rsi), %edi
	addq	$4, %rsi
.Ltmp7244:
	.loc	1 1119 43
	cmpl	(%rcx), %edi
.Ltmp7245:
	.loc	6 315 25
	je	.LBB34_58
	jmp	.LBB34_70
.Ltmp7246:
.LBB34_60:
	.loc	1 1118 5
	movq	2032(%r11), %rcx
	testq	%rcx, %rcx
	je	.LBB34_66
	.loc	1 0 5 is_stmt 0
	movq	2024(%r11), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rsi
	.p2align	4
.LBB34_62:
.Ltmp7247:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp7248:
	.loc	6 180 28
	je	.LBB34_66
.Ltmp7249:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rax), %edi
	jne	.LBB34_70
	movl	4(%rsi), %edi
	cmpl	4(%rax), %edi
	jne	.LBB34_70
	movl	8(%rsi), %edi
.Ltmp7250:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp7251:
	.loc	1 390 34
	cmpl	8(%rax), %edi
.Ltmp7252:
	.loc	6 315 25
	je	.LBB34_62
	jmp	.LBB34_70
.Ltmp7253:
.LBB34_66:
	.loc	1 1119 12
	movq	1968(%r11), %rax
	testq	%rax, %rax
	je	.LBB34_397
	.loc	1 0 12 is_stmt 0
	movq	1960(%r11), %rcx
	shlq	$2, %rax
	xorl	%esi, %esi
	.p2align	4
.LBB34_68:
.Ltmp7254:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rax
.Ltmp7255:
	.loc	6 180 28
	je	.LBB34_397
.Ltmp7256:
	.loc	6 315 25
	movl	(%rcx,%rsi), %edi
	addq	$4, %rsi
.Ltmp7257:
	.loc	1 1119 43
	cmpl	(%rcx), %edi
.Ltmp7258:
	.loc	6 315 25
	je	.LBB34_68
.Ltmp7259:
.LBB34_70:
	.loc	6 0 25 is_stmt 0
	movq	16(%rsp), %rax
	leaq	31(%rax), %r12
	shrq	$5, %r12
	.loc	1 1708 12 is_stmt 1
	testb	%dl, %dl
	je	.LBB34_71
	.loc	1 0 12 is_stmt 0
	leaq	7776(%rsp), %rdi
	movq	536(%rsp), %rsi
.Ltmp7260:
	.loc	1 1797 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	8512(%rsp), %rdi
	movq	528(%rsp), %rsi
.Ltmp7261:
	.loc	1 1798 25
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	movq	8(%rsp), %rax
.Ltmp7262:
	.loc	1 1803 19
	movzbl	1536(%rax), %r14d
.Ltmp7263:
	.loc	1 1804 21
	movzbl	1537(%rax), %ebx
.Ltmp7264:
	.loc	1 1805 27
	movl	1640(%rax), %ecx
	movq	%rcx, 64(%rsp)
.Ltmp7265:
	.loc	1 1806 27
	movl	1644(%rax), %eax
	movq	%rax, 192(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 544(%rsp)
	leaq	10280(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%r15
	leaq	9248(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
.Ltmp7266:
	.loc	1 0 0 is_stmt 0
	testq	%r12, %r12
.Ltmp7267:
	.loc	8 446 20 is_stmt 1
	je	.LBB34_241
.Ltmp7268:
	.loc	8 0 20 is_stmt 0
	movq	%r12, 2688(%rsp)
.Ltmp7269:
	.loc	29 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm6
.Ltmp7270:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm6, %ymm0
	vmovaps	%ymm6, 704(%rsp)
	testb	%r14b, %r14b
	jne	.LBB34_245
.Ltmp7271:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 704(%rsp)
.LBB34_245:
	testb	%bl, %bl
	movq	8(%rsp), %r11
	movq	24(%rsp), %rax
	movq	16(%rsp), %rdx
	jne	.LBB34_247
	vmovaps	%ymm0, %ymm6
.LBB34_247:
	movq	$0, 864(%rsp)
	movq	408(%rsp), %rcx
	movq	%rcx, 256(%rsp)
	movq	%r13, 2624(%rsp)
	movq	344(%rsp), %rcx
	movq	%rcx, 288(%rsp)
	movq	%rax, 832(%rsp)
	xorl	%r10d, %r10d
	movq	%rdx, %rax
	xorl	%r8d, %r8d
	movq	64(%rsp), %rbx
	vmovaps	%ymm6, 2592(%rsp)
.Ltmp7272:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB34_250
.Ltmp7273:
	.loc	8 0 20 is_stmt 0
.Ltmp7274:
	.p2align	4
.LBB34_249:
	addq	$32, %r8
	movq	2688(%rsp), %rax
	decq	%rax
	movq	4256(%rsp), %rcx
.Ltmp7275:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	addq	$256, %r10
	addq	$-256, 832(%rsp)
	addq	$1024, 288(%rsp)
	addq	$-256, 2624(%rsp)
	addq	$1024, 256(%rsp)
	movq	%rax, 2688(%rsp)
	testq	%rax, %rax
	movq	%rcx, %rax
	je	.LBB34_242
.LBB34_250:
.Ltmp7276:
	.loc	4 2584 13
	cmpq	$1, %rax
	movq	%rax, 4256(%rsp)
	movq	%rax, %rcx
	adcq	$0, %rcx
	cmpq	$32, %rcx
	movl	$32, %eax
	cmovaeq	%rax, %rcx
	movq	%rcx, 2656(%rsp)
	leal	(,%rcx,8), %ecx
.Ltmp7277:
	.loc	1 1761 23
	vmovaps	7776(%rsp), %ymm0
	vmovaps	%ymm0, 160(%rsp)
	vmovaps	7808(%rsp), %ymm8
	vmovaps	7840(%rsp), %ymm4
	vmovaps	7872(%rsp), %ymm5
	vmovaps	7904(%rsp), %ymm10
	vmovaps	7936(%rsp), %ymm11
	vmovaps	7968(%rsp), %ymm7
	vmovaps	8000(%rsp), %ymm6
	vmovaps	8032(%rsp), %ymm14
	vmovaps	8064(%rsp), %ymm0
	vmovaps	8096(%rsp), %ymm1
.Ltmp7278:
	.loc	10 1916 50
	cmpq	%r8, %rdx
.Ltmp7279:
	.loc	3 900 12
	jne	.LBB34_252
.Ltmp7280:
	.loc	1 0 0 is_stmt 0
	vmovaps	8128(%rsp), %ymm2
	vmovaps	%ymm2, 352(%rsp)
.Ltmp7281:
	.loc	3 900 12
	jmp	.LBB34_256
.Ltmp7282:
	.loc	3 0 12
.Ltmp7283:
	.p2align	4
.LBB34_252:
	vmovaps	%ymm0, %ymm15
	vmovaps	(%r11), %ymm0
	vmovaps	%ymm0, 448(%rsp)
	vmovaps	32(%r11), %ymm0
	vmovaps	%ymm0, 736(%rsp)
	vmovaps	64(%r11), %ymm0
	vmovaps	%ymm0, 416(%rsp)
	vmovaps	96(%r11), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	128(%r11), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	vmovaps	160(%r11), %ymm0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	192(%r11), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	224(%r11), %ymm0
	vmovaps	%ymm0, 480(%rsp)
	vmovaps	256(%r11), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	288(%r11), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	320(%r11), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	movq	832(%rsp), %rdx
	xorl	%esi, %esi
	vmovaps	%ymm8, %ymm13
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm14, %ymm12
	vmovaps	%ymm6, %ymm3
	vmovaps	352(%r11), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	384(%r11), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	416(%r11), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	448(%r11), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	480(%r11), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	512(%r11), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	544(%r11), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	576(%r11), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	608(%r11), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	640(%r11), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	672(%r11), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	704(%r11), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	736(%r11), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	768(%r11), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	800(%r11), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	832(%r11), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	864(%r11), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	896(%r11), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	928(%r11), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	960(%r11), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	992(%r11), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	1024(%r11), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	1056(%r11), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	1088(%r11), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	1120(%r11), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	1152(%r11), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	1184(%r11), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	1216(%r11), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1248(%r11), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	1280(%r11), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1312(%r11), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovaps	1344(%r11), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1376(%r11), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1408(%r11), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1440(%r11), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	1472(%r11), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1504(%r11), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	.p2align	4
.LBB34_253:
	vmovaps	160(%rsp), %ymm8
.Ltmp7284:
	.loc	5 568 12 is_stmt 1
	leaq	(%r10,%rsi), %rax
	cmpq	24(%rsp), %rax
	ja	.LBB34_629
.Ltmp7285:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7286:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm3, %ymm14
	vmovaps	%ymm12, %ymm9
	vmovaps	%ymm15, %ymm1
	vmovaps	%ymm13, %ymm3
	movq	288(%rsp), %rax
	vmovaps	%ymm7, 32(%rsp)
.Ltmp7287:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rax,%rsi,4), %ymm15
	vmovaps	%ymm15, 160(%rsp)
.Ltmp7288:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm13
	vmovaps	%ymm2, %ymm6
.Ltmp7289:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm15, %ymm2
	vmovaps	%ymm11, %ymm7
	vmovaps	%ymm10, %ymm0
	vmovaps	%ymm5, %ymm11
	vmovaps	%ymm4, %ymm5
	vmovaps	%ymm3, %ymm4
	vxorps	%xmm3, %xmm3, %xmm3
.Ltmp7290:
	.loc	29 48 14
	vaddps	%ymm3, %ymm2, %ymm2
.Ltmp7291:
	.loc	29 283 14
	vmulps	736(%rsp), %ymm15, %ymm10
.Ltmp7292:
	.loc	29 48 14
	vaddps	%ymm3, %ymm10, %ymm10
.Ltmp7293:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm15, %ymm12
.Ltmp7294:
	.loc	29 48 14
	vaddps	%ymm3, %ymm12, %ymm12
.Ltmp7295:
	.loc	29 283 14
	vmulps	64(%rsp), %ymm15, %ymm15
.Ltmp7296:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm15
.Ltmp7297:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm8, %ymm3
.Ltmp7298:
	.loc	29 48 14
	vaddps	%ymm3, %ymm2, %ymm2
.Ltmp7299:
	.loc	29 283 14
	vmulps	1344(%rsp), %ymm8, %ymm3
.Ltmp7300:
	.loc	29 48 14
	vaddps	%ymm3, %ymm10, %ymm3
.Ltmp7301:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm8, %ymm10
.Ltmp7302:
	.loc	29 48 14
	vaddps	%ymm10, %ymm12, %ymm10
.Ltmp7303:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm8, %ymm12
.Ltmp7304:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp7305:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm4, %ymm15
.Ltmp7306:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7307:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm4, %ymm15
.Ltmp7308:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7309:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm4, %ymm15
.Ltmp7310:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7311:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm4, %ymm15
.Ltmp7312:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7313:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm5, %ymm15
.Ltmp7314:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7315:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm5, %ymm15
.Ltmp7316:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7317:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm5, %ymm15
.Ltmp7318:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7319:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm5, %ymm15
.Ltmp7320:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7321:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm11, %ymm15
.Ltmp7322:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7323:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm11, %ymm15
.Ltmp7324:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7325:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm11, %ymm15
.Ltmp7326:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7327:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm11, %ymm15
.Ltmp7328:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7329:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm0, %ymm15
.Ltmp7330:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7331:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm0, %ymm15
.Ltmp7332:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7333:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm0, %ymm15
.Ltmp7334:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7335:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm0, %ymm15
.Ltmp7336:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7337:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm7, %ymm15
.Ltmp7338:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7339:
	.loc	29 283 14
	vmulps	1632(%rsp), %ymm7, %ymm15
.Ltmp7340:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7341:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm7, %ymm15
.Ltmp7342:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7343:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm7, %ymm15
.Ltmp7344:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	1600(%rsp), %ymm15
.Ltmp7345:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp7346:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
	vmovaps	2016(%rsp), %ymm15
.Ltmp7347:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp7348:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
	vmovaps	1984(%rsp), %ymm15
.Ltmp7349:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp7350:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
	vmovaps	1952(%rsp), %ymm15
.Ltmp7351:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp7352:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7353:
	.loc	29 283 14
	vmulps	1568(%rsp), %ymm14, %ymm15
.Ltmp7354:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7355:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm14, %ymm15
.Ltmp7356:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7357:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm14, %ymm15
.Ltmp7358:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7359:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm14, %ymm15
.Ltmp7360:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7361:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm9, %ymm15
.Ltmp7362:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7363:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm9, %ymm15
.Ltmp7364:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7365:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm9, %ymm15
.Ltmp7366:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7367:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm9, %ymm15
.Ltmp7368:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7369:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm1, %ymm15
.Ltmp7370:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7371:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm1, %ymm15
.Ltmp7372:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7373:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm1, %ymm15
.Ltmp7374:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7375:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm1, %ymm15
.Ltmp7376:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7377:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm6, %ymm15
.Ltmp7378:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7379:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm6, %ymm15
.Ltmp7380:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7381:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm6, %ymm15
.Ltmp7382:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
	vmovaps	%ymm6, 352(%rsp)
.Ltmp7383:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm6, %ymm15
.Ltmp7384:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7385:
	.loc	29 82 19
	vandps	%ymm7, %ymm13, %ymm15
.Ltmp7386:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm2, %ymm13, %ymm2
.Ltmp7387:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm2, %ymm15, %ymm2
.Ltmp7388:
	.loc	29 82 19
	vandps	%ymm3, %ymm13, %ymm3
.Ltmp7389:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7390:
	.loc	29 82 19
	vandps	%ymm13, %ymm10, %ymm3
.Ltmp7391:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7392:
	.loc	29 82 19
	vandps	%ymm13, %ymm12, %ymm3
.Ltmp7393:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
	vmovaps	%ymm11, %ymm10
	vmovaps	%ymm0, %ymm11
	vmovaps	32(%rsp), %ymm6
	vmovaps	%ymm9, %ymm0
.Ltmp7394:
	.loc	11 551 14
	vmovups	%ymm2, 10280(%rsp,%rsi,4)
.Ltmp7395:
	.loc	10 1916 50
	addq	$8, %rsi
	addq	$-8, %rdx
	vmovaps	%ymm8, %ymm13
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm9, %ymm15
	vmovaps	%ymm14, %ymm12
	vmovaps	%ymm6, %ymm3
	cmpq	%rsi, %rcx
.Ltmp7396:
	.loc	3 900 12
	jne	.LBB34_253
.Ltmp7397:
.LBB34_256:
	.loc	3 0 12 is_stmt 0
	vmovaps	160(%rsp), %ymm2
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm2, 7776(%rsp)
	vmovaps	%ymm8, 7808(%rsp)
	vmovaps	%ymm4, 7840(%rsp)
	vmovaps	%ymm5, 7872(%rsp)
	vmovaps	%ymm10, 7904(%rsp)
	vmovaps	%ymm11, 7936(%rsp)
	vmovaps	%ymm7, 7968(%rsp)
	vmovaps	%ymm6, 8000(%rsp)
	vmovaps	%ymm14, 8032(%rsp)
	vmovaps	%ymm0, 8064(%rsp)
	vmovaps	%ymm1, 8096(%rsp)
	vmovaps	352(%rsp), %ymm0
	vmovaps	%ymm0, 8128(%rsp)
.Ltmp7398:
	.loc	1 1761 23
	vmovaps	8512(%rsp), %ymm0
	vmovaps	%ymm0, 160(%rsp)
	vmovaps	8544(%rsp), %ymm8
	vmovaps	8576(%rsp), %ymm4
	vmovaps	8608(%rsp), %ymm5
	vmovaps	8640(%rsp), %ymm10
	vmovaps	8672(%rsp), %ymm11
	vmovaps	8704(%rsp), %ymm7
	vmovaps	8736(%rsp), %ymm6
	vmovaps	8768(%rsp), %ymm14
	vmovaps	8800(%rsp), %ymm0
	vmovaps	8832(%rsp), %ymm1
.Ltmp7399:
	.loc	10 1916 50
	cmpq	%r8, 16(%rsp)
.Ltmp7400:
	.loc	3 900 12
	jne	.LBB34_258
.Ltmp7401:
	.loc	1 0 0 is_stmt 0
	vmovaps	8864(%rsp), %ymm2
	vmovaps	%ymm2, 352(%rsp)
.Ltmp7402:
	.loc	3 900 12
	jmp	.LBB34_262
.Ltmp7403:
	.loc	3 0 12
.Ltmp7404:
	.p2align	4
.LBB34_258:
	vmovaps	%ymm0, %ymm15
	vmovaps	(%r11), %ymm0
	vmovaps	%ymm0, 448(%rsp)
	vmovaps	32(%r11), %ymm0
	vmovaps	%ymm0, 736(%rsp)
	vmovaps	64(%r11), %ymm0
	vmovaps	%ymm0, 416(%rsp)
	vmovaps	96(%r11), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	128(%r11), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	vmovaps	160(%r11), %ymm0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	192(%r11), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	224(%r11), %ymm0
	vmovaps	%ymm0, 480(%rsp)
	vmovaps	256(%r11), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	288(%r11), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	320(%r11), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	movq	2624(%rsp), %rdx
	xorl	%esi, %esi
	vmovaps	%ymm8, %ymm13
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm14, %ymm12
	vmovaps	%ymm6, %ymm3
	vmovaps	352(%r11), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	384(%r11), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	416(%r11), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	448(%r11), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	480(%r11), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	512(%r11), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	544(%r11), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	576(%r11), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	608(%r11), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	640(%r11), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	672(%r11), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	704(%r11), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	736(%r11), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	768(%r11), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	800(%r11), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	832(%r11), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	864(%r11), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	896(%r11), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	928(%r11), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	960(%r11), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	992(%r11), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	1024(%r11), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	1056(%r11), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	1088(%r11), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	1120(%r11), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	1152(%r11), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	1184(%r11), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	1216(%r11), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1248(%r11), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	1280(%r11), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1312(%r11), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovaps	1344(%r11), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1376(%r11), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1408(%r11), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1440(%r11), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	1472(%r11), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1504(%r11), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	.p2align	4
.LBB34_259:
	vmovaps	160(%rsp), %ymm8
.Ltmp7405:
	.loc	5 568 12 is_stmt 1
	leaq	(%r10,%rsi), %rax
	cmpq	%r13, %rax
	ja	.LBB34_630
.Ltmp7406:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7407:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm3, %ymm14
	vmovaps	%ymm12, %ymm9
	vmovaps	%ymm15, %ymm1
	vmovaps	%ymm13, %ymm3
	movq	256(%rsp), %rax
	vmovaps	%ymm7, 32(%rsp)
.Ltmp7408:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rax,%rsi,4), %ymm15
	vmovaps	%ymm15, 160(%rsp)
.Ltmp7409:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm13
	vmovaps	%ymm2, %ymm6
.Ltmp7410:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm15, %ymm2
	vmovaps	%ymm11, %ymm7
	vmovaps	%ymm10, %ymm0
	vmovaps	%ymm5, %ymm11
	vmovaps	%ymm4, %ymm5
	vmovaps	%ymm3, %ymm4
	vxorps	%xmm3, %xmm3, %xmm3
.Ltmp7411:
	.loc	29 48 14
	vaddps	%ymm3, %ymm2, %ymm2
.Ltmp7412:
	.loc	29 283 14
	vmulps	736(%rsp), %ymm15, %ymm10
.Ltmp7413:
	.loc	29 48 14
	vaddps	%ymm3, %ymm10, %ymm10
.Ltmp7414:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm15, %ymm12
.Ltmp7415:
	.loc	29 48 14
	vaddps	%ymm3, %ymm12, %ymm12
.Ltmp7416:
	.loc	29 283 14
	vmulps	64(%rsp), %ymm15, %ymm15
.Ltmp7417:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm15
.Ltmp7418:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm8, %ymm3
.Ltmp7419:
	.loc	29 48 14
	vaddps	%ymm3, %ymm2, %ymm2
.Ltmp7420:
	.loc	29 283 14
	vmulps	1344(%rsp), %ymm8, %ymm3
.Ltmp7421:
	.loc	29 48 14
	vaddps	%ymm3, %ymm10, %ymm3
.Ltmp7422:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm8, %ymm10
.Ltmp7423:
	.loc	29 48 14
	vaddps	%ymm10, %ymm12, %ymm10
.Ltmp7424:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm8, %ymm12
.Ltmp7425:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp7426:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm4, %ymm15
.Ltmp7427:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7428:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm4, %ymm15
.Ltmp7429:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7430:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm4, %ymm15
.Ltmp7431:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7432:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm4, %ymm15
.Ltmp7433:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7434:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm5, %ymm15
.Ltmp7435:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7436:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm5, %ymm15
.Ltmp7437:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7438:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm5, %ymm15
.Ltmp7439:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7440:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm5, %ymm15
.Ltmp7441:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7442:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm11, %ymm15
.Ltmp7443:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7444:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm11, %ymm15
.Ltmp7445:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7446:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm11, %ymm15
.Ltmp7447:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7448:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm11, %ymm15
.Ltmp7449:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7450:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm0, %ymm15
.Ltmp7451:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7452:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm0, %ymm15
.Ltmp7453:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7454:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm0, %ymm15
.Ltmp7455:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7456:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm0, %ymm15
.Ltmp7457:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7458:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm7, %ymm15
.Ltmp7459:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7460:
	.loc	29 283 14
	vmulps	1632(%rsp), %ymm7, %ymm15
.Ltmp7461:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7462:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm7, %ymm15
.Ltmp7463:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7464:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm7, %ymm15
.Ltmp7465:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	1600(%rsp), %ymm15
.Ltmp7466:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp7467:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
	vmovaps	2016(%rsp), %ymm15
.Ltmp7468:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp7469:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
	vmovaps	1984(%rsp), %ymm15
.Ltmp7470:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp7471:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
	vmovaps	1952(%rsp), %ymm15
.Ltmp7472:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp7473:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7474:
	.loc	29 283 14
	vmulps	1568(%rsp), %ymm14, %ymm15
.Ltmp7475:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7476:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm14, %ymm15
.Ltmp7477:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7478:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm14, %ymm15
.Ltmp7479:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7480:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm14, %ymm15
.Ltmp7481:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7482:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm9, %ymm15
.Ltmp7483:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7484:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm9, %ymm15
.Ltmp7485:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7486:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm9, %ymm15
.Ltmp7487:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7488:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm9, %ymm15
.Ltmp7489:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7490:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm1, %ymm15
.Ltmp7491:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7492:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm1, %ymm15
.Ltmp7493:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7494:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm1, %ymm15
.Ltmp7495:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp7496:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm1, %ymm15
.Ltmp7497:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7498:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm6, %ymm15
.Ltmp7499:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7500:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm6, %ymm15
.Ltmp7501:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7502:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm6, %ymm15
.Ltmp7503:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
	vmovaps	%ymm6, 352(%rsp)
.Ltmp7504:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm6, %ymm15
.Ltmp7505:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7506:
	.loc	29 82 19
	vandps	%ymm7, %ymm13, %ymm15
.Ltmp7507:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm2, %ymm13, %ymm2
.Ltmp7508:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm2, %ymm15, %ymm2
.Ltmp7509:
	.loc	29 82 19
	vandps	%ymm3, %ymm13, %ymm3
.Ltmp7510:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7511:
	.loc	29 82 19
	vandps	%ymm13, %ymm10, %ymm3
.Ltmp7512:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7513:
	.loc	29 82 19
	vandps	%ymm13, %ymm12, %ymm3
.Ltmp7514:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
	vmovaps	%ymm11, %ymm10
	vmovaps	%ymm0, %ymm11
	vmovaps	32(%rsp), %ymm6
	vmovaps	%ymm9, %ymm0
.Ltmp7515:
	.loc	11 551 14
	vmovups	%ymm2, 9248(%rsp,%rsi,4)
.Ltmp7516:
	.loc	10 1916 50
	addq	$8, %rsi
	addq	$-8, %rdx
	vmovaps	%ymm8, %ymm13
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm9, %ymm15
	vmovaps	%ymm14, %ymm12
	vmovaps	%ymm6, %ymm3
	cmpq	%rsi, %rcx
.Ltmp7517:
	.loc	3 900 12
	jne	.LBB34_259
.Ltmp7518:
.LBB34_262:
	.loc	3 0 12 is_stmt 0
	vmovaps	160(%rsp), %ymm2
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm2, 8512(%rsp)
	vmovaps	%ymm8, 8544(%rsp)
	vmovaps	%ymm4, 8576(%rsp)
	vmovaps	%ymm5, 8608(%rsp)
	vmovaps	%ymm10, 8640(%rsp)
	vmovaps	%ymm11, 8672(%rsp)
	vmovaps	%ymm7, 8704(%rsp)
	vmovaps	%ymm6, 8736(%rsp)
	vmovaps	%ymm14, 8768(%rsp)
	vmovaps	%ymm0, 8800(%rsp)
	vmovaps	%ymm1, 8832(%rsp)
	vmovaps	352(%rsp), %ymm0
	vmovaps	%ymm0, 8864(%rsp)
	movq	16(%rsp), %rdx
.Ltmp7519:
	.loc	10 1916 50
	cmpq	%r8, %rdx
.Ltmp7520:
	.loc	3 900 12
	je	.LBB34_249
.Ltmp7521:
	.loc	1 872 9
	vmovaps	8160(%rsp), %ymm7
.Ltmp7522:
	.loc	1 872 9 is_stmt 0
	vmovaps	8288(%rsp), %ymm8
	vmovaps	8448(%rsp), %ymm9
	vmovaps	8480(%rsp), %ymm10
.Ltmp7523:
	.loc	1 872 9
	vmovaps	8896(%rsp), %ymm11
.Ltmp7524:
	.loc	1 872 9
	vmovaps	9024(%rsp), %ymm13
	vmovaps	9184(%rsp), %ymm12
	vmovaps	9216(%rsp), %ymm14
	xorl	%r9d, %r9d
	movq	24(%rsp), %rax
	vmovaps	2592(%rsp), %ymm6
	movq	%r10, 2336(%rsp)
	movq	%r8, 2720(%rsp)
.Ltmp7525:
	.loc	1 0 9
.Ltmp7526:
	.p2align	4
.LBB34_264:
	.loc	1 1837 24 is_stmt 1
	leaq	(%r9,%r8), %rsi
	shlq	$3, %rsi
.Ltmp7527:
	.loc	5 568 12
	movq	%rax, %rdx
	movq	%rsi, 416(%rsp)
	subq	%rsi, %rdx
	jb	.LBB34_387
.Ltmp7528:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7529:
	.loc	1 1394 25
	movq	1688(%r11), %rsi
.Ltmp7530:
	.loc	1 1390 17
	movq	1840(%r11), %rdi
	movq	%rdi, 96(%rsp)
.Ltmp7531:
	.loc	1 1394 45
	imulq	192(%rsp), %rdi
.Ltmp7532:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_302
.Ltmp7533:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7534:
	.loc	5 0 16 is_stmt 0
	movq	%rbx, 64(%rsp)
	movq	%r9, 128(%rsp)
	movq	%r9, %rax
	shlq	$5, %rax
	vmovups	10280(%rsp,%rax), %ymm0
.Ltmp7535:
	vmovups	9248(%rsp,%rax), %ymm15
.Ltmp7536:
	vmaxps	%ymm0, %ymm15, %ymm3
	vmovaps	704(%rsp), %ymm1
.Ltmp7537:
	vblendvps	%ymm1, %ymm3, %ymm0, %ymm0
.Ltmp7538:
	vdivps	%ymm0, %ymm7, %ymm1
.Ltmp7539:
	movq	1624(%r11), %rcx
.Ltmp7540:
	vcmpgt_oqps	%ymm7, %ymm0, %ymm0
	vbroadcastss	.LCPI34_2(%rip), %ymm2
	vblendvps	%ymm0, %ymm1, %ymm2, %ymm0
.Ltmp7541:
	.loc	1 1394 25 is_stmt 1
	movq	1680(%r11), %rax
	movq	%rdi, 1344(%rsp)
.Ltmp7542:
	.loc	11 551 14
	vmovups	%ymm0, (%rax,%rdi,4)
.Ltmp7543:
	.loc	1 1261 17
	movq	1840(%r11), %rdx
.Ltmp7544:
	.loc	12 37 12
	testq	%rdx, %rdx
	je	.LBB34_289
.Ltmp7545:
	.loc	12 0 12 is_stmt 0
	movq	8(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 352(%rsp)
	movq	192(%rsp), %rax
	leaq	1(%rax), %r8
	cmpq	%rcx, %r8
	movq	%rcx, %rsi
	movl	$0, %eax
	cmovbq	%rax, %rsi
	movq	1824(%rdi), %rax
	subq	%rsi, %r8
	movq	1680(%rdi), %rbx
	movq	1688(%rdi), %rsi
	movq	1768(%rdi), %r9
	movq	%r9, 448(%rsp)
	movq	1760(%rdi), %r13
	movq	1736(%rdi), %r9
	movq	%r9, 32(%rsp)
	movq	1728(%rdi), %rdi
	movq	%rdi, 160(%rsp)
	imulq	%rdx, %r8
	movq	%r8, 736(%rsp)
	movq	%rdx, %r10
	xorl	%r11d, %r11d
	xorl	%r8d, %r8d
	jmp	.LBB34_270
	.p2align	4
.LBB34_300:
	xorl	%r15d, %r15d
.LBB34_288:
	decq	%r10
	addq	$4, %r8
.Ltmp7546:
	movl	%r15d, (%r13,%r11,4)
.Ltmp7547:
	incq	%r11
.Ltmp7548:
	.loc	12 37 12 is_stmt 1
	testq	%r10, %r10
	je	.LBB34_289
.LBB34_270:
.Ltmp7549:
	.loc	7 1714 9
	cmpq	$32, %r8
.Ltmp7550:
	.loc	6 180 28
	je	.LBB34_289
.Ltmp7551:
	.loc	1 1265 21
	cmpq	352(%rsp), %r11
	je	.LBB34_282
	leaq	(%r11,%r11,2), %r9
	movl	4(%rax,%r9,4), %r12d
.Ltmp7552:
	.loc	1 1267 23
	addq	192(%rsp), %r12
.Ltmp7553:
	.loc	1 1268 12
	cmpq	%rcx, %r12
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %r12
.Ltmp7554:
	.loc	1 1275 42
	movq	%r12, %rdi
	imulq	%rdx, %rdi
	addq	%r11, %rdi
	.loc	1 1275 22 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB34_388
.Ltmp7555:
	.loc	1 1276 24 is_stmt 1
	cmpq	448(%rsp), %r11
	je	.LBB34_285
.Ltmp7556:
	.loc	1 0 0 is_stmt 0
	movl	(%rax,%r9,4), %r9d
.Ltmp7557:
	vmovss	(%rbx,%rdi,4), %xmm0
.Ltmp7558:
	.loc	1 1276 24
	movl	(%r13,%r11,4), %r15d
	testq	%r15, %r15
	je	.LBB34_277
.Ltmp7559:
	.loc	1 1280 24 is_stmt 1
	cmpq	32(%rsp), %r11
	jae	.LBB34_117
	.loc	1 0 24 is_stmt 0
	movq	160(%rsp), %r14
	.loc	1 1280 24
	vmovss	(%r14,%r11,4), %xmm1
.Ltmp7560:
	.loc	1 905 8 is_stmt 1
	vucomiss	%xmm1, %xmm0
	jbe	.LBB34_277
.Ltmp7561:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB34_277:
.Ltmp7562:
	.loc	1 1282 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB34_121
	.loc	1 0 9 is_stmt 0
	movq	160(%rsp), %r14
	.loc	1 1282 9
	vmovss	%xmm0, (%r14,%r11,4)
	.loc	1 1283 24 is_stmt 1
	incq	%r15
	cmpq	%r9, %r15
.Ltmp7563:
	.loc	1 1284 23
	jne	.LBB34_279
	.loc	1 1284 9 is_stmt 0
	vmovss	%xmm0, 544(%rsp,%r8)
	.loc	1 1290 30 is_stmt 1
	vmovss	(%rbx,%rdi,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp7564:
	.p2align	4
.LBB34_298:
.Ltmp7565:
	.loc	1 1293 65 is_stmt 1
	movq	%r12, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1293 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_232
.Ltmp7566:
	.loc	1 905 8 is_stmt 1
	vminss	(%rbx,%r14,4), %xmm0, %xmm0
.Ltmp7567:
	.loc	1 1294 17
	vmovss	%xmm0, (%rbx,%r14,4)
	.loc	1 1295 20
	testq	%r12, %r12
	cmoveq	%rcx, %r12
	.loc	1 1298 17
	decq	%r12
.Ltmp7568:
	.loc	10 1916 50
	decq	%r9
.Ltmp7569:
	.loc	3 900 12
	jne	.LBB34_298
	jmp	.LBB34_300
.Ltmp7570:
	.loc	3 0 12 is_stmt 0
.Ltmp7571:
	.p2align	4
.LBB34_279:
	movq	736(%rsp), %rdi
	.loc	1 1287 44 is_stmt 1
	leaq	(%r11,%rdi), %r14
	.loc	1 1287 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_280
	vmovss	(%rbx,%r14,4), %xmm1
.Ltmp7572:
	.loc	1 905 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp7573:
	.loc	1 1284 9
	vmovss	%xmm0, 544(%rsp,%r8)
	jmp	.LBB34_288
.Ltmp7574:
	.loc	1 0 9 is_stmt 0
.Ltmp7575:
	.p2align	4
.LBB34_289:
	.loc	11 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm0
	movq	8(%rsp), %r11
.Ltmp7576:
	.loc	1 1414 26
	movq	1704(%r11), %rsi
	vmovaps	%ymm0, %ymm5
	movq	96(%rsp), %rdi
.Ltmp7577:
	.loc	12 37 12
	testq	%rdi, %rdi
	movq	248(%rsp), %r13
	movq	64(%rsp), %rbx
	je	.LBB34_304
.Ltmp7578:
	.loc	12 0 12 is_stmt 0
	movq	1832(%r11), %r9
.Ltmp7579:
	.loc	1 1405 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB34_296
	.loc	1 0 42 is_stmt 0
	movq	1824(%r11), %r10
	.loc	1 1405 42
	movl	8(%r10), %r8d
	.loc	1 1405 28
	addq	192(%rsp), %r8
.Ltmp7580:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	movq	96(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	.loc	1 0 25
	movq	1696(%r11), %rdx
	.loc	1 1409 25
	vmovss	(%rdx,%r8,4), %xmm1
	.loc	1 1409 13
	vmovss	%xmm1, 544(%rsp)
.Ltmp7581:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB34_303
.Ltmp7582:
	.loc	1 1405 42
	cmpq	$1, %r9
	je	.LBB34_294
	movl	20(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7583:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	4(%rdx,%rax,4), %xmm1
	.loc	1 1409 13
	vmovss	%xmm1, 548(%rsp)
.Ltmp7584:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB34_303
.Ltmp7585:
	.loc	1 1405 42
	cmpq	$2, %r9
	je	.LBB34_364
	movl	32(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7586:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	8(%rdx,%rax,4), %xmm1
	.loc	1 1409 13
	vmovss	%xmm1, 552(%rsp)
.Ltmp7587:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB34_303
.Ltmp7588:
	.loc	1 1405 42
	cmpq	$3, %r9
	je	.LBB34_368
	movl	44(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7589:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	12(%rdx,%rax,4), %xmm1
	.loc	1 1409 13
	vmovss	%xmm1, 556(%rsp)
.Ltmp7590:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB34_303
.Ltmp7591:
	.loc	1 1405 42
	cmpq	$4, %r9
	je	.LBB34_372
	movl	56(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7592:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	16(%rdx,%rax,4), %xmm1
	.loc	1 1409 13
	vmovss	%xmm1, 560(%rsp)
.Ltmp7593:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB34_303
.Ltmp7594:
	.loc	1 1405 42
	cmpq	$5, %r9
	je	.LBB34_376
	movl	68(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7595:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	20(%rdx,%rax,4), %xmm1
	.loc	1 1409 13
	vmovss	%xmm1, 564(%rsp)
.Ltmp7596:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB34_303
.Ltmp7597:
	.loc	1 1405 42
	cmpq	$6, %r9
	je	.LBB34_380
	movl	80(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7598:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	24(%rdx,%rax,4), %xmm1
	.loc	1 1409 13
	vmovss	%xmm1, 568(%rsp)
.Ltmp7599:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB34_303
.Ltmp7600:
	.loc	1 1405 42
	cmpq	$7, %r9
	je	.LBB34_384
	movl	92(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7601:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	28(%rdx,%rax,4), %xmm1
	.loc	1 1409 13
	vmovss	%xmm1, 572(%rsp)
.Ltmp7602:
	.loc	1 0 13
.Ltmp7603:
	.p2align	4
.LBB34_303:
	.loc	11 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm5
.Ltmp7604:
.LBB34_304:
	.loc	5 580 12
	movq	%rsi, %rdx
	movq	1344(%rsp), %r8
	subq	%r8, %rdx
	jb	.LBB34_359
.Ltmp7605:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7606:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm0
	vbroadcastss	.LCPI34_4(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp7607:
	vaddps	%ymm0, %ymm9, %ymm1
	vsubps	%ymm5, %ymm1, %ymm9
	.loc	1 1414 26 is_stmt 1
	movq	1696(%r11), %rax
.Ltmp7608:
	.loc	11 551 14
	vmovups	%ymm0, (%rax,%r8,4)
.Ltmp7609:
	.loc	29 360 14
	vdivps	%ymm10, %ymm9, %ymm0
.Ltmp7610:
	.loc	1 1418 43
	vmovaps	8416(%rsp), %ymm1
.Ltmp7611:
	.loc	29 347 14
	vbroadcastss	.LCPI34_2(%rip), %ymm2
	vsubps	%ymm0, %ymm2, %ymm0
.Ltmp7612:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm1, %ymm0, %ymm2
.Ltmp7613:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm2, %ymm8, %ymm2
.Ltmp7614:
	.loc	29 48 14
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp7615:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp7616:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm4
	vandps	%ymm4, %ymm0, %ymm1
.Ltmp7617:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp7618:
	.loc	29 82 19
	vandnps	%ymm0, %ymm1, %ymm0
.Ltmp7619:
	.loc	1 1419 5
	vmovaps	%ymm0, 8416(%rsp)
.Ltmp7620:
	.loc	1 1422 28
	movq	1672(%r11), %rsi
	.loc	1 1422 44 is_stmt 0
	imulq	%rbx, %rdi
.Ltmp7621:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_146
.Ltmp7622:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7623:
	.loc	5 0 16 is_stmt 0
	movq	344(%rsp), %rax
	movq	416(%rsp), %rsi
	leaq	(%rax,%rsi,4), %rax
.Ltmp7624:
	vbroadcastss	.LCPI34_2(%rip), %ymm1
	vsubps	%ymm0, %ymm1, %ymm0
.Ltmp7625:
	.loc	1 1422 28 is_stmt 1
	movq	1664(%r11), %rdx
.Ltmp7626:
	.loc	11 551 14
	vmovups	(%rdx,%rdi,4), %ymm1
.Ltmp7627:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rax), %ymm2
.Ltmp7628:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp7629:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm1, %ymm0, %ymm0
.Ltmp7630:
	.loc	11 551 14
	vmovups	%ymm2, (%rdx,%rdi,4)
.Ltmp7631:
	.loc	11 551 14 is_stmt 0
	vmovups	%ymm0, (%rax)
.Ltmp7632:
	.loc	5 568 12 is_stmt 1
	movq	%r13, %rdx
	subq	%rsi, %rdx
	jb	.LBB34_396
.Ltmp7633:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7634:
	.loc	1 1394 25
	movq	1888(%r11), %rsi
.Ltmp7635:
	.loc	1 1390 17
	movq	2040(%r11), %rdi
.Ltmp7636:
	.loc	1 1394 45
	movq	%rdi, %r8
	imulq	192(%rsp), %r8
.Ltmp7637:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r8, %rdx
	jb	.LBB34_395
.Ltmp7638:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7639:
	.loc	5 0 16 is_stmt 0
	vmovaps	704(%rsp), %ymm0
	vblendvps	%ymm0, %ymm3, %ymm15, %ymm0
.Ltmp7640:
	vdivps	%ymm0, %ymm11, %ymm1
	vcmpgt_oqps	%ymm11, %ymm0, %ymm0
	vbroadcastss	.LCPI34_2(%rip), %ymm2
	vblendvps	%ymm0, %ymm1, %ymm2, %ymm0
.Ltmp7641:
	.loc	1 1394 25 is_stmt 1
	movq	1880(%r11), %rax
.Ltmp7642:
	.loc	11 551 14
	vmovups	%ymm0, (%rax,%r8,4)
.Ltmp7643:
	.loc	1 1261 17
	movq	2040(%r11), %rdx
.Ltmp7644:
	.loc	12 37 12
	testq	%rdx, %rdx
	je	.LBB34_327
.Ltmp7645:
	.loc	12 0 12 is_stmt 0
	movq	%r8, 1344(%rsp)
	movq	%rdi, 96(%rsp)
	movq	2032(%r11), %rax
	movq	%rax, 352(%rsp)
	movq	192(%rsp), %rax
	leaq	1(%rax), %rdi
	cmpq	%rcx, %rdi
	movq	%rcx, %rsi
	movl	$0, %eax
	cmovbq	%rax, %rsi
	movq	2024(%r11), %rax
	subq	%rsi, %rdi
	movq	1880(%r11), %rbx
	movq	1888(%r11), %rsi
	movq	1968(%r11), %r8
	movq	%r8, 448(%rsp)
	movq	1960(%r11), %r13
	movq	1936(%r11), %r8
	movq	%r8, 32(%rsp)
	movq	1928(%r11), %r8
	movq	%r8, 160(%rsp)
	imulq	%rdx, %rdi
	movq	%rdi, 736(%rsp)
	movq	%rdx, %r10
	xorl	%r11d, %r11d
	xorl	%r8d, %r8d
	jmp	.LBB34_314
	.p2align	4
.LBB34_393:
	xorl	%r15d, %r15d
.LBB34_325:
	decq	%r10
	addq	$4, %r8
.Ltmp7646:
	movl	%r15d, (%r13,%r11,4)
.Ltmp7647:
	incq	%r11
.Ltmp7648:
	.loc	12 37 12 is_stmt 1
	testq	%r10, %r10
	je	.LBB34_326
.LBB34_314:
.Ltmp7649:
	.loc	7 1714 9
	cmpq	$32, %r8
.Ltmp7650:
	.loc	6 180 28
	je	.LBB34_326
.Ltmp7651:
	.loc	1 1265 21
	cmpq	352(%rsp), %r11
	je	.LBB34_282
	leaq	(%r11,%r11,2), %r9
	movl	4(%rax,%r9,4), %r12d
.Ltmp7652:
	.loc	1 1267 23
	addq	192(%rsp), %r12
.Ltmp7653:
	.loc	1 1268 12
	cmpq	%rcx, %r12
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %r12
.Ltmp7654:
	.loc	1 1275 42
	movq	%r12, %rdi
	imulq	%rdx, %rdi
	addq	%r11, %rdi
	.loc	1 1275 22 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB34_388
.Ltmp7655:
	.loc	1 1276 24 is_stmt 1
	cmpq	448(%rsp), %r11
	je	.LBB34_285
.Ltmp7656:
	.loc	1 0 0 is_stmt 0
	movl	(%rax,%r9,4), %r9d
.Ltmp7657:
	vmovss	(%rbx,%rdi,4), %xmm0
.Ltmp7658:
	.loc	1 1276 24
	movl	(%r13,%r11,4), %r15d
	testq	%r15, %r15
	je	.LBB34_321
.Ltmp7659:
	.loc	1 1280 24 is_stmt 1
	cmpq	32(%rsp), %r11
	jae	.LBB34_117
	.loc	1 0 24 is_stmt 0
	movq	160(%rsp), %r14
	.loc	1 1280 24
	vmovss	(%r14,%r11,4), %xmm1
.Ltmp7660:
	.loc	1 905 8 is_stmt 1
	vucomiss	%xmm1, %xmm0
	jbe	.LBB34_321
.Ltmp7661:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB34_321:
.Ltmp7662:
	.loc	1 1282 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB34_121
	.loc	1 0 9 is_stmt 0
	movq	160(%rsp), %r14
	.loc	1 1282 9
	vmovss	%xmm0, (%r14,%r11,4)
	.loc	1 1283 24 is_stmt 1
	incq	%r15
	cmpq	%r9, %r15
.Ltmp7663:
	.loc	1 1284 23
	jne	.LBB34_323
	.loc	1 1284 9 is_stmt 0
	vmovss	%xmm0, 544(%rsp,%r8)
	.loc	1 1290 30 is_stmt 1
	vmovss	(%rbx,%rdi,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp7664:
	.p2align	4
.LBB34_391:
.Ltmp7665:
	.loc	1 1293 65 is_stmt 1
	movq	%r12, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1293 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_232
.Ltmp7666:
	.loc	1 905 8 is_stmt 1
	vminss	(%rbx,%r14,4), %xmm0, %xmm0
.Ltmp7667:
	.loc	1 1294 17
	vmovss	%xmm0, (%rbx,%r14,4)
	.loc	1 1295 20
	testq	%r12, %r12
	cmoveq	%rcx, %r12
	.loc	1 1298 17
	decq	%r12
.Ltmp7668:
	.loc	10 1916 50
	decq	%r9
.Ltmp7669:
	.loc	3 900 12
	jne	.LBB34_391
	jmp	.LBB34_393
.Ltmp7670:
	.loc	3 0 12 is_stmt 0
.Ltmp7671:
	.p2align	4
.LBB34_323:
	movq	736(%rsp), %rdi
	.loc	1 1287 44 is_stmt 1
	leaq	(%r11,%rdi), %r14
	.loc	1 1287 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_280
	vmovss	(%rbx,%r14,4), %xmm1
.Ltmp7672:
	.loc	1 905 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp7673:
	.loc	1 1284 9
	vmovss	%xmm0, 544(%rsp,%r8)
	jmp	.LBB34_325
.Ltmp7674:
	.loc	1 0 9 is_stmt 0
.Ltmp7675:
	.p2align	4
.LBB34_326:
	.loc	11 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm5
	movq	8(%rsp), %r11
	movq	248(%rsp), %r13
	movq	64(%rsp), %rbx
	movq	96(%rsp), %rdi
	movq	1344(%rsp), %r8
.Ltmp7676:
.LBB34_327:
	.loc	1 1414 26
	movq	1904(%r11), %rsi
	vmovaps	%ymm5, %ymm0
.Ltmp7677:
	.loc	12 37 12
	testq	%rdi, %rdi
	je	.LBB34_353
.Ltmp7678:
	.loc	12 0 12 is_stmt 0
	movq	2032(%r11), %r9
.Ltmp7679:
	.loc	1 1405 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB34_296
	.loc	1 0 42 is_stmt 0
	movq	%r8, %r14
	movq	2024(%r11), %r10
	.loc	1 1405 42
	movl	8(%r10), %r8d
	.loc	1 1405 28
	addq	192(%rsp), %r8
.Ltmp7680:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	.loc	1 1409 40
	imulq	%rdi, %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	.loc	1 0 25
	movq	1896(%r11), %rdx
	.loc	1 1409 25
	vmovss	(%rdx,%r8,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 544(%rsp)
.Ltmp7681:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB34_352
.Ltmp7682:
	.loc	1 1405 42
	cmpq	$1, %r9
	je	.LBB34_294
	movl	20(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7683:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	4(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 548(%rsp)
.Ltmp7684:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB34_352
.Ltmp7685:
	.loc	1 1405 42
	cmpq	$2, %r9
	je	.LBB34_364
	movl	32(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7686:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	8(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 552(%rsp)
.Ltmp7687:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB34_352
.Ltmp7688:
	.loc	1 1405 42
	cmpq	$3, %r9
	je	.LBB34_368
	movl	44(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7689:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	12(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 556(%rsp)
.Ltmp7690:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB34_352
.Ltmp7691:
	.loc	1 1405 42
	cmpq	$4, %r9
	je	.LBB34_372
	movl	56(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7692:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	16(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 560(%rsp)
.Ltmp7693:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB34_352
.Ltmp7694:
	.loc	1 1405 42
	cmpq	$5, %r9
	je	.LBB34_376
	movl	68(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7695:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	20(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 564(%rsp)
.Ltmp7696:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB34_352
.Ltmp7697:
	.loc	1 1405 42
	cmpq	$6, %r9
	je	.LBB34_380
	movl	80(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7698:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	24(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 568(%rsp)
.Ltmp7699:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB34_352
.Ltmp7700:
	.loc	1 1405 42
	cmpq	$7, %r9
	je	.LBB34_384
	movl	92(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp7701:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_175
	vmovss	28(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 572(%rsp)
.Ltmp7702:
	.loc	1 0 13
.Ltmp7703:
	.p2align	4
.LBB34_352:
	.loc	11 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm0
	movq	%r14, %r8
.Ltmp7704:
.LBB34_353:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r8, %rdx
	jb	.LBB34_359
.Ltmp7705:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7706:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm1
	vmulps	%ymm1, %ymm5, %ymm1
	vroundps	$9, %ymm1, %ymm1
	vbroadcastss	.LCPI34_4(%rip), %ymm2
	vmulps	%ymm2, %ymm1, %ymm1
.Ltmp7707:
	vaddps	%ymm1, %ymm12, %ymm2
	vsubps	%ymm0, %ymm2, %ymm12
	.loc	1 1414 26 is_stmt 1
	movq	1896(%r11), %rax
.Ltmp7708:
	.loc	11 551 14
	vmovups	%ymm1, (%rax,%r8,4)
.Ltmp7709:
	.loc	1 1418 43
	vmovaps	9152(%rsp), %ymm0
.Ltmp7710:
	.loc	29 360 14
	vdivps	%ymm14, %ymm12, %ymm1
.Ltmp7711:
	.loc	29 347 14
	vbroadcastss	.LCPI34_2(%rip), %ymm2
	vsubps	%ymm1, %ymm2, %ymm1
.Ltmp7712:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm0, %ymm1, %ymm2
.Ltmp7713:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm2, %ymm13, %ymm2
.Ltmp7714:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp7715:
	.loc	29 233 14
	vmaxps	%ymm0, %ymm1, %ymm0
.Ltmp7716:
	.loc	29 82 19
	vandps	%ymm4, %ymm0, %ymm1
.Ltmp7717:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp7718:
	.loc	29 82 19
	vandnps	%ymm0, %ymm1, %ymm0
.Ltmp7719:
	.loc	1 1419 5
	vmovaps	%ymm0, 9152(%rsp)
.Ltmp7720:
	.loc	1 1422 28
	movq	1872(%r11), %rsi
	.loc	1 1422 44 is_stmt 0
	imulq	%rbx, %rdi
.Ltmp7721:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_146
.Ltmp7722:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7723:
	.loc	5 0 16 is_stmt 0
	movq	128(%rsp), %r9
	incq	%r9
	movq	408(%rsp), %rax
	movq	416(%rsp), %rdx
.Ltmp7724:
	leaq	(%rax,%rdx,4), %rax
.Ltmp7725:
	vbroadcastss	.LCPI34_2(%rip), %ymm1
	vsubps	%ymm0, %ymm1, %ymm0
.Ltmp7726:
	.loc	1 1422 28 is_stmt 1
	movq	1864(%r11), %rdx
.Ltmp7727:
	.loc	11 551 14
	vmovups	(%rdx,%rdi,4), %ymm1
.Ltmp7728:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rax), %ymm2
.Ltmp7729:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp7730:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm1, %ymm0, %ymm0
.Ltmp7731:
	.loc	11 551 14
	vmovups	%ymm2, (%rdx,%rdi,4)
.Ltmp7732:
	.loc	1 1883 13
	incq	%rbx
	.loc	1 1884 16
	cmpq	1632(%r11), %rbx
.Ltmp7733:
	.loc	11 551 14
	vmovups	%ymm0, (%rax)
	movl	$0, %edx
.Ltmp7734:
	.loc	1 1884 16
	cmoveq	%rdx, %rbx
	movq	192(%rsp), %rax
	.loc	1 1887 13
	incq	%rax
	.loc	1 1888 16
	cmpq	%rcx, %rax
	movl	$0, %ecx
	movq	%rcx, 864(%rsp)
	cmoveq	%rdx, %rax
	movq	%rax, 192(%rsp)
.Ltmp7735:
	.loc	10 1916 50
	cmpq	2656(%rsp), %r9
	movq	24(%rsp), %rax
	movq	2336(%rsp), %r10
	movq	2720(%rsp), %r8
.Ltmp7736:
	.loc	3 900 12
	jne	.LBB34_264
.Ltmp7737:
	.loc	1 1413 0
	vmovaps	%ymm9, 8448(%rsp)
.Ltmp7738:
	.loc	1 1413 0 is_stmt 0
	vmovaps	%ymm12, 9184(%rsp)
	movq	16(%rsp), %rdx
	jmp	.LBB34_249
.Ltmp7739:
.LBB34_71:
	.loc	1 0 0
	leaq	2784(%rsp), %rdi
	movq	536(%rsp), %rsi
.Ltmp7740:
	.loc	1 1797 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	3520(%rsp), %rdi
	movq	528(%rsp), %rsi
.Ltmp7741:
	.loc	1 1798 25
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	movq	8(%rsp), %rax
.Ltmp7742:
	.loc	1 1803 19
	movzbl	1536(%rax), %r14d
.Ltmp7743:
	.loc	1 1804 21
	movzbl	1537(%rax), %ecx
	movb	%cl, 32(%rsp)
.Ltmp7744:
	.loc	1 1805 27
	movl	1640(%rax), %ebx
.Ltmp7745:
	.loc	1 1806 27
	movl	1644(%rax), %eax
	movq	%rax, 192(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 544(%rsp)
	leaq	10280(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%r15
	leaq	9248(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
.Ltmp7746:
	.loc	1 0 0 is_stmt 0
	testq	%r12, %r12
.Ltmp7747:
	.loc	8 446 20 is_stmt 1
	je	.LBB34_238
.Ltmp7748:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp7749:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 2432(%rsp)
	vmovaps	%ymm1, 1728(%rsp)
	testb	%r14b, %r14b
	jne	.LBB34_74
.Ltmp7750:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 1728(%rsp)
.LBB34_74:
	cmpb	$0, 32(%rsp)
	movq	8(%rsp), %r11
	movq	24(%rsp), %rsi
	movq	16(%rsp), %rcx
	jne	.LBB34_76
	vmovaps	%ymm0, 2432(%rsp)
.LBB34_76:
	movq	$0, 832(%rsp)
	movq	408(%rsp), %rax
	movq	%rax, 2272(%rsp)
	movq	%r13, 2592(%rsp)
	movq	344(%rsp), %rax
	movq	%rax, 1504(%rsp)
	movq	%rsi, %rdx
	xorl	%r14d, %r14d
	movq	%rcx, %r15
	xorl	%eax, %eax
.Ltmp7751:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB34_79
.Ltmp7752:
	.loc	8 0 20 is_stmt 0
.Ltmp7753:
	.p2align	4
.LBB34_77:
	vmovaps	480(%rsp), %ymm0
.Ltmp7754:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp7755:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp7756:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp7757:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp7758:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp7759:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp7760:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp7761:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
	movq	16(%rsp), %rcx
.Ltmp7762:
.LBB34_78:
	addq	$32, %rax
	movq	2688(%rsp), %r12
	decq	%r12
.Ltmp7763:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %r15
	addq	$256, %r14
	movq	2624(%rsp), %rdx
	addq	$-256, %rdx
	addq	$1024, 1504(%rsp)
	addq	$-256, 2592(%rsp)
	addq	$1024, 2272(%rsp)
	testq	%r12, %r12
	je	.LBB34_238
.LBB34_79:
	.loc	8 0 20 is_stmt 0
	movq	%r12, 2688(%rsp)
.Ltmp7764:
	.loc	4 2584 13 is_stmt 1
	cmpq	$1, %r15
	movq	%r15, %r8
	adcq	$0, %r8
	cmpq	$32, %r8
	movq	%rax, %rdi
	movl	$32, %eax
	cmovaeq	%rax, %r8
	movq	%r8, 2720(%rsp)
	leal	(,%r8,8), %eax
.Ltmp7765:
	.loc	1 1761 23
	vmovaps	2784(%rsp), %ymm4
	vmovaps	2816(%rsp), %ymm5
	vmovaps	2848(%rsp), %ymm0
	vmovaps	2880(%rsp), %ymm1
	vmovaps	2912(%rsp), %ymm2
	vmovaps	2944(%rsp), %ymm10
	vmovaps	2976(%rsp), %ymm3
	vmovaps	3008(%rsp), %ymm13
	vmovaps	3040(%rsp), %ymm7
	vmovaps	3072(%rsp), %ymm8
	vmovaps	3104(%rsp), %ymm11
	movq	%rdi, 2336(%rsp)
.Ltmp7766:
	.loc	10 1916 50
	cmpq	%rdi, %rcx
	movq	%rdx, 2624(%rsp)
.Ltmp7767:
	.loc	3 900 12
	jne	.LBB34_81
.Ltmp7768:
	.loc	1 0 0 is_stmt 0
	vmovaps	3136(%rsp), %ymm6
	vmovaps	%ymm6, 352(%rsp)
.Ltmp7769:
	.loc	3 900 12
	jmp	.LBB34_85
.Ltmp7770:
	.loc	3 0 12
.Ltmp7771:
	.p2align	4
.LBB34_81:
	vmovaps	(%r11), %ymm6
	vmovaps	%ymm6, 448(%rsp)
	vmovaps	32(%r11), %ymm6
	vmovaps	%ymm6, 736(%rsp)
	vmovaps	64(%r11), %ymm6
	vmovaps	%ymm6, 416(%rsp)
	vmovaps	%ymm5, %ymm14
	vmovaps	96(%r11), %ymm5
	vmovaps	%ymm5, 64(%rsp)
	vmovaps	128(%r11), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	160(%r11), %ymm5
	vmovaps	%ymm5, 704(%rsp)
	vmovaps	192(%r11), %ymm5
	vmovaps	%ymm5, 1344(%rsp)
	vmovaps	224(%r11), %ymm5
	vmovaps	%ymm5, 128(%rsp)
	vmovaps	256(%r11), %ymm5
	vmovaps	%ymm5, 288(%rsp)
	vmovaps	288(%r11), %ymm5
	vmovaps	%ymm5, 256(%rsp)
	vmovaps	320(%r11), %ymm5
	vmovaps	%ymm5, 480(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm11, %ymm15
	vmovaps	%ymm8, %ymm12
	vmovaps	%ymm7, %ymm9
	vmovaps	%ymm13, %ymm6
	vmovaps	352(%r11), %ymm5
	vmovaps	%ymm5, 768(%rsp)
	vmovaps	384(%r11), %ymm5
	vmovaps	%ymm5, 1312(%rsp)
	vmovaps	416(%r11), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	448(%r11), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	480(%r11), %ymm5
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	512(%r11), %ymm5
	vmovaps	%ymm5, 2240(%rsp)
	vmovaps	544(%r11), %ymm5
	vmovaps	%ymm5, 1280(%rsp)
	vmovaps	576(%r11), %ymm5
	vmovaps	%ymm5, 2208(%rsp)
	vmovaps	608(%r11), %ymm5
	vmovaps	%ymm5, 1696(%rsp)
	vmovaps	640(%r11), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	672(%r11), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	vmovaps	704(%r11), %ymm5
	vmovaps	%ymm5, 2112(%rsp)
	vmovaps	736(%r11), %ymm5
	vmovaps	%ymm5, 1664(%rsp)
	vmovaps	768(%r11), %ymm5
	vmovaps	%ymm5, 1632(%rsp)
	vmovaps	800(%r11), %ymm5
	vmovaps	%ymm5, 2080(%rsp)
	vmovaps	832(%r11), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	864(%r11), %ymm5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	896(%r11), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	928(%r11), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	960(%r11), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	992(%r11), %ymm5
	vmovaps	%ymm5, 1568(%rsp)
	vmovaps	1024(%r11), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	1056(%r11), %ymm5
	vmovaps	%ymm5, 1536(%rsp)
	vmovaps	1088(%r11), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	1120(%r11), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	1152(%r11), %ymm5
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	1184(%r11), %ymm5
	vmovaps	%ymm5, 1376(%rsp)
	vmovaps	1216(%r11), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	1248(%r11), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	1280(%r11), %ymm5
	vmovaps	%ymm5, 672(%rsp)
	vmovaps	1312(%r11), %ymm5
	vmovaps	%ymm5, 1760(%rsp)
	vmovaps	1344(%r11), %ymm5
	vmovaps	%ymm5, 2400(%rsp)
	vmovaps	1376(%r11), %ymm5
	vmovaps	%ymm5, 2368(%rsp)
	vmovaps	1408(%r11), %ymm5
	vmovaps	%ymm5, 2528(%rsp)
	vmovaps	1440(%r11), %ymm5
	vmovaps	%ymm5, 2496(%rsp)
	vmovaps	1472(%r11), %ymm5
	vmovaps	%ymm5, 2464(%rsp)
	vmovaps	1504(%r11), %ymm5
	vmovaps	%ymm5, 2656(%rsp)
	.p2align	4
.LBB34_82:
.Ltmp7772:
	.loc	5 568 12 is_stmt 1
	leaq	(%r14,%rcx), %rdi
	cmpq	%rsi, %rdi
	ja	.LBB34_493
.Ltmp7773:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7774:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm4, %ymm5
	vmovaps	%ymm9, %ymm4
	vmovaps	%ymm2, %ymm9
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm14, %ymm0
	movq	1504(%rsp), %rdi
	vmovaps	%ymm12, %ymm11
	vmovaps	%ymm5, 32(%rsp)
	vmovaps	%ymm4, 160(%rsp)
	vmovaps	%ymm6, %ymm8
	vmovaps	%ymm3, %ymm7
.Ltmp7775:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rdi,%rcx,4), %ymm13
.Ltmp7776:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm14
.Ltmp7777:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm13, %ymm6
	vmovaps	%ymm10, %ymm3
	vmovaps	%ymm9, %ymm5
	vmovaps	%ymm2, %ymm10
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm4
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp7778:
	.loc	29 48 14
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp7779:
	.loc	29 283 14
	vmulps	736(%rsp), %ymm13, %ymm9
.Ltmp7780:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp7781:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm13, %ymm12
.Ltmp7782:
	.loc	29 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm15, %ymm1
.Ltmp7783:
	.loc	29 283 14
	vmulps	64(%rsp), %ymm13, %ymm15
.Ltmp7784:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	32(%rsp), %ymm0
.Ltmp7785:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm0, %ymm0
.Ltmp7786:
	.loc	29 48 14
	vaddps	%ymm0, %ymm6, %ymm0
	vmovaps	32(%rsp), %ymm6
.Ltmp7787:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm6, %ymm6
.Ltmp7788:
	.loc	29 48 14
	vaddps	%ymm6, %ymm9, %ymm6
	vmovaps	1344(%rsp), %ymm9
.Ltmp7789:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm9, %ymm9
.Ltmp7790:
	.loc	29 48 14
	vaddps	%ymm9, %ymm12, %ymm9
	vmovaps	128(%rsp), %ymm12
.Ltmp7791:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm12, %ymm12
.Ltmp7792:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp7793:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm4, %ymm15
.Ltmp7794:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7795:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm4, %ymm15
.Ltmp7796:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7797:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm4, %ymm15
.Ltmp7798:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7799:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm4, %ymm15
.Ltmp7800:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7801:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm2, %ymm15
.Ltmp7802:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7803:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm2, %ymm15
.Ltmp7804:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7805:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm2, %ymm15
.Ltmp7806:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7807:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm2, %ymm15
.Ltmp7808:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7809:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm10, %ymm15
.Ltmp7810:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7811:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm10, %ymm15
.Ltmp7812:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7813:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm10, %ymm15
.Ltmp7814:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7815:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm10, %ymm15
.Ltmp7816:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7817:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm5, %ymm15
.Ltmp7818:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7819:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm5, %ymm15
.Ltmp7820:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7821:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm5, %ymm15
.Ltmp7822:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7823:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm5, %ymm15
.Ltmp7824:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7825:
	.loc	29 283 14
	vmulps	1632(%rsp), %ymm3, %ymm15
.Ltmp7826:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7827:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm3, %ymm15
.Ltmp7828:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7829:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm3, %ymm15
.Ltmp7830:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7831:
	.loc	29 283 14
	vmulps	1600(%rsp), %ymm3, %ymm15
.Ltmp7832:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7833:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm7, %ymm15
.Ltmp7834:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7835:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm7, %ymm15
.Ltmp7836:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7837:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm7, %ymm15
.Ltmp7838:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7839:
	.loc	29 283 14
	vmulps	1568(%rsp), %ymm7, %ymm15
.Ltmp7840:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7841:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm8, %ymm15
.Ltmp7842:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7843:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm8, %ymm15
.Ltmp7844:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7845:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm8, %ymm15
.Ltmp7846:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7847:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm8, %ymm15
.Ltmp7848:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	160(%rsp), %ymm15
.Ltmp7849:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm15, %ymm15
.Ltmp7850:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
	vmovaps	160(%rsp), %ymm15
.Ltmp7851:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm15, %ymm15
.Ltmp7852:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	160(%rsp), %ymm15
.Ltmp7853:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm15, %ymm15
.Ltmp7854:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	160(%rsp), %ymm15
.Ltmp7855:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm15, %ymm15
.Ltmp7856:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7857:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm11, %ymm15
.Ltmp7858:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7859:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm11, %ymm15
.Ltmp7860:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7861:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm11, %ymm15
.Ltmp7862:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7863:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm11, %ymm15
.Ltmp7864:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7865:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm1, %ymm15
.Ltmp7866:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7867:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm1, %ymm15
.Ltmp7868:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7869:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm1, %ymm15
.Ltmp7870:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	%ymm1, 352(%rsp)
.Ltmp7871:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm1, %ymm15
.Ltmp7872:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7873:
	.loc	29 82 19
	vandps	%ymm3, %ymm14, %ymm15
.Ltmp7874:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm0, %ymm14, %ymm0
.Ltmp7875:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm15, %ymm0
.Ltmp7876:
	.loc	29 82 19
	vandps	%ymm6, %ymm14, %ymm6
.Ltmp7877:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp7878:
	.loc	29 82 19
	vandps	%ymm14, %ymm9, %ymm6
.Ltmp7879:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp7880:
	.loc	29 82 19
	vandps	%ymm14, %ymm12, %ymm6
.Ltmp7881:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp7882:
	.loc	11 551 14
	vmovups	%ymm0, 10280(%rsp,%rcx,4)
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm2, %ymm1
	vmovaps	%ymm10, %ymm2
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm13, %ymm4
	vmovaps	%ymm7, %ymm13
	vmovaps	%ymm8, %ymm7
	vmovaps	160(%rsp), %ymm8
	vmovaps	32(%rsp), %ymm5
.Ltmp7883:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm5, %ymm14
	vmovaps	%ymm11, %ymm15
	vmovaps	%ymm8, %ymm12
	vmovaps	%ymm7, %ymm9
	vmovaps	%ymm13, %ymm6
	cmpq	%rcx, %rax
.Ltmp7884:
	.loc	3 900 12
	jne	.LBB34_82
.Ltmp7885:
.LBB34_85:
	.loc	1 1767 5
	vmovaps	%ymm4, 2784(%rsp)
	vmovaps	%ymm5, 2816(%rsp)
	vmovaps	%ymm0, 2848(%rsp)
	vmovaps	%ymm1, 2880(%rsp)
	vmovaps	%ymm2, 2912(%rsp)
	vmovaps	%ymm10, 2944(%rsp)
	vmovaps	%ymm3, 2976(%rsp)
	vmovaps	%ymm13, 3008(%rsp)
	vmovaps	%ymm7, 3040(%rsp)
	vmovaps	%ymm8, 3072(%rsp)
	vmovaps	%ymm11, 3104(%rsp)
	vmovaps	352(%rsp), %ymm0
	vmovaps	%ymm0, 3136(%rsp)
.Ltmp7886:
	.loc	1 1761 23
	vmovaps	3520(%rsp), %ymm4
	vmovaps	3552(%rsp), %ymm5
	vmovaps	3584(%rsp), %ymm0
	vmovaps	3616(%rsp), %ymm1
	vmovaps	3648(%rsp), %ymm2
	vmovaps	3680(%rsp), %ymm10
	vmovaps	3712(%rsp), %ymm3
	vmovaps	3744(%rsp), %ymm13
	vmovaps	3776(%rsp), %ymm7
	vmovaps	3808(%rsp), %ymm8
	vmovaps	3840(%rsp), %ymm11
	movq	2336(%rsp), %rcx
.Ltmp7887:
	.loc	10 1916 50
	cmpq	%rcx, 16(%rsp)
.Ltmp7888:
	.loc	3 900 12
	jne	.LBB34_87
.Ltmp7889:
	.loc	1 0 0 is_stmt 0
	vmovaps	3872(%rsp), %ymm6
	vmovaps	%ymm6, 352(%rsp)
.Ltmp7890:
	.loc	3 900 12
	jmp	.LBB34_91
.Ltmp7891:
	.loc	3 0 12
.Ltmp7892:
	.p2align	4
.LBB34_87:
	vmovaps	(%r11), %ymm6
	vmovaps	%ymm6, 448(%rsp)
	vmovaps	32(%r11), %ymm6
	vmovaps	%ymm6, 736(%rsp)
	vmovaps	64(%r11), %ymm6
	vmovaps	%ymm6, 416(%rsp)
	vmovaps	%ymm5, %ymm14
	vmovaps	96(%r11), %ymm5
	vmovaps	%ymm5, 64(%rsp)
	vmovaps	128(%r11), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	160(%r11), %ymm5
	vmovaps	%ymm5, 704(%rsp)
	vmovaps	192(%r11), %ymm5
	vmovaps	%ymm5, 1344(%rsp)
	vmovaps	224(%r11), %ymm5
	vmovaps	%ymm5, 128(%rsp)
	vmovaps	256(%r11), %ymm5
	vmovaps	%ymm5, 288(%rsp)
	vmovaps	288(%r11), %ymm5
	vmovaps	%ymm5, 256(%rsp)
	vmovaps	320(%r11), %ymm5
	vmovaps	%ymm5, 480(%rsp)
	movq	2592(%rsp), %rdx
	xorl	%ecx, %ecx
	vmovaps	%ymm11, %ymm15
	vmovaps	%ymm8, %ymm12
	vmovaps	%ymm7, %ymm9
	vmovaps	%ymm13, %ymm6
	vmovaps	352(%r11), %ymm5
	vmovaps	%ymm5, 768(%rsp)
	vmovaps	384(%r11), %ymm5
	vmovaps	%ymm5, 1312(%rsp)
	vmovaps	416(%r11), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	448(%r11), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	480(%r11), %ymm5
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	512(%r11), %ymm5
	vmovaps	%ymm5, 2240(%rsp)
	vmovaps	544(%r11), %ymm5
	vmovaps	%ymm5, 1280(%rsp)
	vmovaps	576(%r11), %ymm5
	vmovaps	%ymm5, 2208(%rsp)
	vmovaps	608(%r11), %ymm5
	vmovaps	%ymm5, 1696(%rsp)
	vmovaps	640(%r11), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	672(%r11), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	vmovaps	704(%r11), %ymm5
	vmovaps	%ymm5, 2112(%rsp)
	vmovaps	736(%r11), %ymm5
	vmovaps	%ymm5, 1664(%rsp)
	vmovaps	768(%r11), %ymm5
	vmovaps	%ymm5, 1632(%rsp)
	vmovaps	800(%r11), %ymm5
	vmovaps	%ymm5, 2080(%rsp)
	vmovaps	832(%r11), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	864(%r11), %ymm5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	896(%r11), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	928(%r11), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	960(%r11), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	992(%r11), %ymm5
	vmovaps	%ymm5, 1568(%rsp)
	vmovaps	1024(%r11), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	1056(%r11), %ymm5
	vmovaps	%ymm5, 1536(%rsp)
	vmovaps	1088(%r11), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	1120(%r11), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	1152(%r11), %ymm5
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	1184(%r11), %ymm5
	vmovaps	%ymm5, 1376(%rsp)
	vmovaps	1216(%r11), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	1248(%r11), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	1280(%r11), %ymm5
	vmovaps	%ymm5, 672(%rsp)
	vmovaps	1312(%r11), %ymm5
	vmovaps	%ymm5, 1760(%rsp)
	vmovaps	1344(%r11), %ymm5
	vmovaps	%ymm5, 2400(%rsp)
	vmovaps	1376(%r11), %ymm5
	vmovaps	%ymm5, 2368(%rsp)
	vmovaps	1408(%r11), %ymm5
	vmovaps	%ymm5, 2528(%rsp)
	vmovaps	1440(%r11), %ymm5
	vmovaps	%ymm5, 2496(%rsp)
	vmovaps	1472(%r11), %ymm5
	vmovaps	%ymm5, 2464(%rsp)
	vmovaps	1504(%r11), %ymm5
	vmovaps	%ymm5, 2656(%rsp)
	.p2align	4
.LBB34_88:
.Ltmp7893:
	.loc	5 568 12 is_stmt 1
	leaq	(%r14,%rcx), %rdi
	cmpq	%r13, %rdi
	ja	.LBB34_98
.Ltmp7894:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp7895:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm4, %ymm5
	vmovaps	%ymm9, %ymm4
	vmovaps	%ymm2, %ymm9
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm14, %ymm0
	movq	2272(%rsp), %rdi
	vmovaps	%ymm12, %ymm11
	vmovaps	%ymm5, 32(%rsp)
	vmovaps	%ymm4, 160(%rsp)
	vmovaps	%ymm6, %ymm8
	vmovaps	%ymm3, %ymm7
.Ltmp7896:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rdi,%rcx,4), %ymm13
.Ltmp7897:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm14
.Ltmp7898:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm13, %ymm6
	vmovaps	%ymm10, %ymm3
	vmovaps	%ymm9, %ymm5
	vmovaps	%ymm2, %ymm10
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm4
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp7899:
	.loc	29 48 14
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp7900:
	.loc	29 283 14
	vmulps	736(%rsp), %ymm13, %ymm9
.Ltmp7901:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp7902:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm13, %ymm12
.Ltmp7903:
	.loc	29 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm15, %ymm1
.Ltmp7904:
	.loc	29 283 14
	vmulps	64(%rsp), %ymm13, %ymm15
.Ltmp7905:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	32(%rsp), %ymm0
.Ltmp7906:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm0, %ymm0
.Ltmp7907:
	.loc	29 48 14
	vaddps	%ymm0, %ymm6, %ymm0
	vmovaps	32(%rsp), %ymm6
.Ltmp7908:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm6, %ymm6
.Ltmp7909:
	.loc	29 48 14
	vaddps	%ymm6, %ymm9, %ymm6
	vmovaps	1344(%rsp), %ymm9
.Ltmp7910:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm9, %ymm9
.Ltmp7911:
	.loc	29 48 14
	vaddps	%ymm9, %ymm12, %ymm9
	vmovaps	128(%rsp), %ymm12
.Ltmp7912:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm12, %ymm12
.Ltmp7913:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp7914:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm4, %ymm15
.Ltmp7915:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7916:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm4, %ymm15
.Ltmp7917:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7918:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm4, %ymm15
.Ltmp7919:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7920:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm4, %ymm15
.Ltmp7921:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7922:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm2, %ymm15
.Ltmp7923:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7924:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm2, %ymm15
.Ltmp7925:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7926:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm2, %ymm15
.Ltmp7927:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7928:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm2, %ymm15
.Ltmp7929:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7930:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm10, %ymm15
.Ltmp7931:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7932:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm10, %ymm15
.Ltmp7933:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7934:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm10, %ymm15
.Ltmp7935:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7936:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm10, %ymm15
.Ltmp7937:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7938:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm5, %ymm15
.Ltmp7939:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7940:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm5, %ymm15
.Ltmp7941:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7942:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm5, %ymm15
.Ltmp7943:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7944:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm5, %ymm15
.Ltmp7945:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7946:
	.loc	29 283 14
	vmulps	1632(%rsp), %ymm3, %ymm15
.Ltmp7947:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7948:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm3, %ymm15
.Ltmp7949:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7950:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm3, %ymm15
.Ltmp7951:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7952:
	.loc	29 283 14
	vmulps	1600(%rsp), %ymm3, %ymm15
.Ltmp7953:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7954:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm7, %ymm15
.Ltmp7955:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7956:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm7, %ymm15
.Ltmp7957:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7958:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm7, %ymm15
.Ltmp7959:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7960:
	.loc	29 283 14
	vmulps	1568(%rsp), %ymm7, %ymm15
.Ltmp7961:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7962:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm8, %ymm15
.Ltmp7963:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7964:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm8, %ymm15
.Ltmp7965:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7966:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm8, %ymm15
.Ltmp7967:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7968:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm8, %ymm15
.Ltmp7969:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	160(%rsp), %ymm15
.Ltmp7970:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm15, %ymm15
.Ltmp7971:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
	vmovaps	160(%rsp), %ymm15
.Ltmp7972:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm15, %ymm15
.Ltmp7973:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	160(%rsp), %ymm15
.Ltmp7974:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm15, %ymm15
.Ltmp7975:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	160(%rsp), %ymm15
.Ltmp7976:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm15, %ymm15
.Ltmp7977:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7978:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm11, %ymm15
.Ltmp7979:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7980:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm11, %ymm15
.Ltmp7981:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7982:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm11, %ymm15
.Ltmp7983:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7984:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm11, %ymm15
.Ltmp7985:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7986:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm1, %ymm15
.Ltmp7987:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7988:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm1, %ymm15
.Ltmp7989:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7990:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm1, %ymm15
.Ltmp7991:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	%ymm1, 352(%rsp)
.Ltmp7992:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm1, %ymm15
.Ltmp7993:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp7994:
	.loc	29 82 19
	vandps	%ymm3, %ymm14, %ymm15
.Ltmp7995:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm0, %ymm14, %ymm0
.Ltmp7996:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm15, %ymm0
.Ltmp7997:
	.loc	29 82 19
	vandps	%ymm6, %ymm14, %ymm6
.Ltmp7998:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp7999:
	.loc	29 82 19
	vandps	%ymm14, %ymm9, %ymm6
.Ltmp8000:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp8001:
	.loc	29 82 19
	vandps	%ymm14, %ymm12, %ymm6
.Ltmp8002:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp8003:
	.loc	11 551 14
	vmovups	%ymm0, 9248(%rsp,%rcx,4)
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm2, %ymm1
	vmovaps	%ymm10, %ymm2
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm13, %ymm4
	vmovaps	%ymm7, %ymm13
	vmovaps	%ymm8, %ymm7
	vmovaps	160(%rsp), %ymm8
	vmovaps	32(%rsp), %ymm5
.Ltmp8004:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm5, %ymm14
	vmovaps	%ymm11, %ymm15
	vmovaps	%ymm8, %ymm12
	vmovaps	%ymm7, %ymm9
	vmovaps	%ymm13, %ymm6
	cmpq	%rcx, %rax
.Ltmp8005:
	.loc	3 900 12
	jne	.LBB34_88
.Ltmp8006:
.LBB34_91:
	.loc	1 1767 5
	vmovaps	%ymm4, 3520(%rsp)
	vmovaps	%ymm5, 3552(%rsp)
	vmovaps	%ymm0, 3584(%rsp)
	vmovaps	%ymm1, 3616(%rsp)
	vmovaps	%ymm2, 3648(%rsp)
	vmovaps	%ymm10, 3680(%rsp)
	vmovaps	%ymm3, 3712(%rsp)
	vmovaps	%ymm13, 3744(%rsp)
	vmovaps	%ymm7, 3776(%rsp)
	vmovaps	%ymm8, 3808(%rsp)
	vmovaps	%ymm11, 3840(%rsp)
	vmovaps	352(%rsp), %ymm0
	vmovaps	%ymm0, 3872(%rsp)
	movq	16(%rsp), %rcx
	movq	2336(%rsp), %rax
.Ltmp8007:
	.loc	10 1916 50
	cmpq	%rax, %rcx
	je	.LBB34_78
.Ltmp8008:
	.loc	1 855 44
	vmovaps	3168(%rsp), %ymm5
	.loc	1 855 73 is_stmt 0
	vmovaps	3200(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	.loc	1 855 61
	vmovaps	3232(%rsp), %ymm6
.Ltmp8009:
	.loc	1 853 26 is_stmt 1
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 480(%rsp)
.Ltmp8010:
	.loc	1 853 26 is_stmt 0
	vmovaps	3392(%rsp), %ymm0
	vmovaps	%ymm0, 256(%rsp)
.Ltmp8011:
	.loc	1 855 44 is_stmt 1
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	.loc	1 855 61 is_stmt 0
	vmovaps	3360(%rsp), %ymm14
	.loc	1 855 73
	vmovaps	3328(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
.Ltmp8012:
	.loc	1 855 44
	vmovaps	3904(%rsp), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	.loc	1 855 73
	vmovaps	3936(%rsp), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	.loc	1 855 61
	vmovaps	3968(%rsp), %ymm11
.Ltmp8013:
	.loc	1 853 26 is_stmt 1
	vmovaps	4000(%rsp), %ymm0
	vmovaps	%ymm0, 288(%rsp)
.Ltmp8014:
	.loc	1 853 26 is_stmt 0
	vmovaps	4128(%rsp), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	4032(%rsp), %ymm15
	vmovaps	4096(%rsp), %ymm8
	vmovaps	3456(%rsp), %ymm1
	xorl	%ecx, %ecx
	vmovaps	4192(%rsp), %ymm0
	vbroadcastss	.LCPI34_2(%rip), %ymm12
	movq	%r14, 864(%rsp)
	movq	%r15, 4256(%rsp)
.Ltmp8015:
	.loc	1 0 26
.Ltmp8016:
	.p2align	4
.LBB34_93:
	vbroadcastss	.LCPI34_1(%rip), %ymm4
.Ltmp8017:
	.loc	29 347 14 is_stmt 1
	vaddps	480(%rsp), %ymm4, %ymm2
	vxorps	%xmm10, %xmm10, %xmm10
.Ltmp8018:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm2, %ymm2
	vmovaps	%ymm2, 480(%rsp)
.Ltmp8019:
	.loc	29 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm3
.Ltmp8020:
	.loc	29 48 14
	vaddps	%ymm6, %ymm5, %ymm5
	vmovaps	1472(%rsp), %ymm2
.Ltmp8021:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm5, %ymm2, %ymm5
.Ltmp8022:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm6, %ymm10, %ymm6
.Ltmp8023:
	.loc	29 347 14 is_stmt 1
	vaddps	256(%rsp), %ymm4, %ymm3
.Ltmp8024:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm3, %ymm2
	vmovaps	%ymm2, 256(%rsp)
.Ltmp8025:
	.loc	29 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm3
.Ltmp8026:
	.loc	29 48 14
	vaddps	64(%rsp), %ymm14, %ymm7
	vmovaps	1440(%rsp), %ymm2
.Ltmp8027:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm7, %ymm2, %ymm2
	vmovaps	%ymm2, 64(%rsp)
.Ltmp8028:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm14, %ymm10, %ymm14
.Ltmp8029:
	.loc	29 347 14 is_stmt 1
	vaddps	288(%rsp), %ymm4, %ymm3
.Ltmp8030:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm3, %ymm2
	vmovaps	%ymm2, 288(%rsp)
.Ltmp8031:
	.loc	29 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm3
.Ltmp8032:
	.loc	29 48 14
	vaddps	96(%rsp), %ymm11, %ymm7
	vmovaps	1408(%rsp), %ymm2
.Ltmp8033:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm7, %ymm2, %ymm2
	vmovaps	%ymm2, 96(%rsp)
.Ltmp8034:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm11, %ymm10, %ymm11
.Ltmp8035:
	.loc	29 347 14 is_stmt 1
	vaddps	128(%rsp), %ymm4, %ymm3
.Ltmp8036:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm3, %ymm2
	vmovaps	%ymm2, 128(%rsp)
.Ltmp8037:
	.loc	29 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm3
.Ltmp8038:
	.loc	29 48 14
	vaddps	%ymm8, %ymm15, %ymm7
.Ltmp8039:
	.loc	1 855 73
	vmovaps	4064(%rsp), %ymm9
.Ltmp8040:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm7, %ymm9, %ymm2
.Ltmp8041:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm8, %ymm10, %ymm8
.Ltmp8042:
	.loc	1 1837 24 is_stmt 1
	leaq	(%rcx,%rax), %rdi
	shlq	$3, %rdi
.Ltmp8043:
	.loc	5 568 12
	movq	%rsi, %rdx
	movq	%rdi, 704(%rsp)
	subq	%rdi, %rdx
	jb	.LBB34_176
.Ltmp8044:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_95
.Ltmp8045:
	.loc	1 1394 25
	movq	1688(%r11), %rsi
.Ltmp8046:
	.loc	1 1390 17
	movq	1840(%r11), %rdi
	movq	%rdi, 416(%rsp)
.Ltmp8047:
	.loc	1 1394 45
	imulq	192(%rsp), %rdi
.Ltmp8048:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_301
.Ltmp8049:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_95
.Ltmp8050:
	.loc	5 0 16 is_stmt 0
	movq	%rbx, 1344(%rsp)
	movq	%rcx, 768(%rsp)
	movq	%rcx, %rax
	shlq	$5, %rax
	vmovups	10280(%rsp,%rax), %ymm7
.Ltmp8051:
	vmovups	9248(%rsp,%rax), %ymm3
.Ltmp8052:
	vmaxps	%ymm7, %ymm3, %ymm10
	vmovaps	1728(%rsp), %ymm4
.Ltmp8053:
	vblendvps	%ymm4, %ymm10, %ymm7, %ymm7
.Ltmp8054:
	vdivps	%ymm7, %ymm5, %ymm9
.Ltmp8055:
	movq	1624(%r11), %rcx
.Ltmp8056:
	vcmpgt_oqps	%ymm5, %ymm7, %ymm7
	vblendvps	%ymm7, %ymm9, %ymm12, %ymm7
.Ltmp8057:
	.loc	1 1394 25 is_stmt 1
	movq	1680(%r11), %rax
	movq	%rdi, 736(%rsp)
.Ltmp8058:
	.loc	11 551 14
	vmovups	%ymm7, (%rax,%rdi,4)
.Ltmp8059:
	.loc	1 1261 17
	movq	1840(%r11), %rdx
.Ltmp8060:
	.loc	12 37 12
	testq	%rdx, %rdx
	je	.LBB34_124
.Ltmp8061:
	.loc	12 0 12 is_stmt 0
	movq	8(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 160(%rsp)
	movq	192(%rsp), %rax
	leaq	1(%rax), %r9
	cmpq	%rcx, %r9
	movq	%rcx, %rax
	movl	$0, %esi
	cmovbq	%rsi, %rax
	movq	1824(%rdi), %rbx
	subq	%rax, %r9
	movq	1680(%rdi), %rax
	movq	1688(%rdi), %rsi
	movq	1768(%rdi), %r8
	movq	%r8, 352(%rsp)
	movq	1760(%rdi), %r13
	movq	1736(%rdi), %r8
	movq	%r8, 32(%rsp)
	movq	1728(%rdi), %r14
	imulq	%rdx, %r9
	movq	%r9, 448(%rsp)
	movq	%rdx, %r15
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB34_104
	.p2align	4
.LBB34_135:
	xorl	%r8d, %r8d
.LBB34_123:
	decq	%r15
	addq	$4, %rdi
.Ltmp8062:
	movl	%r8d, (%r13,%r11,4)
.Ltmp8063:
	incq	%r11
.Ltmp8064:
	.loc	12 37 12 is_stmt 1
	testq	%r15, %r15
	je	.LBB34_124
.LBB34_104:
.Ltmp8065:
	.loc	7 1714 9
	cmpq	$32, %rdi
.Ltmp8066:
	.loc	6 180 28
	je	.LBB34_124
.Ltmp8067:
	.loc	1 1265 21
	cmpq	160(%rsp), %r11
	je	.LBB34_623
	leaq	(%r11,%r11,2), %r8
	movl	4(%rbx,%r8,4), %r9d
.Ltmp8068:
	.loc	1 1267 23
	addq	192(%rsp), %r9
.Ltmp8069:
	.loc	1 1268 12
	cmpq	%rcx, %r9
	movl	$0, %r10d
	cmovaeq	%rcx, %r10
	subq	%r10, %r9
.Ltmp8070:
	.loc	1 1275 42
	movq	%r9, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1275 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_624
.Ltmp8071:
	.loc	1 1276 24 is_stmt 1
	cmpq	352(%rsp), %r11
	je	.LBB34_625
.Ltmp8072:
	.loc	1 0 0 is_stmt 0
	movl	(%rbx,%r8,4), %r10d
.Ltmp8073:
	vmovss	(%rax,%r12,4), %xmm7
.Ltmp8074:
	.loc	1 1276 24
	movl	(%r13,%r11,4), %r8d
	testq	%r8, %r8
.Ltmp8075:
	.loc	1 1277 26 is_stmt 1
	je	.LBB34_111
	.loc	1 1280 24
	cmpq	32(%rsp), %r11
	jae	.LBB34_115
	vmovss	(%r14,%r11,4), %xmm9
.Ltmp8076:
	.loc	1 905 8
	vucomiss	%xmm9, %xmm7
	jbe	.LBB34_111
.Ltmp8077:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm9, %xmm7
.LBB34_111:
.Ltmp8078:
	.loc	1 1282 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB34_119
	vmovss	%xmm7, (%r14,%r11,4)
	.loc	1 1283 24
	incq	%r8
	cmpq	%r10, %r8
.Ltmp8079:
	.loc	1 1284 23
	jne	.LBB34_113
	.loc	1 1284 9 is_stmt 0
	vmovss	%xmm7, 544(%rsp,%rdi)
	.loc	1 1290 30 is_stmt 1
	vmovss	(%rax,%r12,4), %xmm7
	.loc	1 0 30 is_stmt 0
.Ltmp8080:
	.p2align	4
.LBB34_133:
.Ltmp8081:
	.loc	1 1293 65 is_stmt 1
	movq	%r9, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1293 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_626
.Ltmp8082:
	.loc	1 905 8 is_stmt 1
	vminss	(%rax,%r12,4), %xmm7, %xmm7
.Ltmp8083:
	.loc	1 1294 17
	vmovss	%xmm7, (%rax,%r12,4)
	.loc	1 1295 20
	testq	%r9, %r9
	cmoveq	%rcx, %r9
	.loc	1 1298 17
	decq	%r9
.Ltmp8084:
	.loc	10 1916 50
	decq	%r10
.Ltmp8085:
	.loc	3 900 12
	jne	.LBB34_133
	jmp	.LBB34_135
.Ltmp8086:
	.loc	3 0 12 is_stmt 0
.Ltmp8087:
	.p2align	4
.LBB34_113:
	movq	448(%rsp), %r9
	.loc	1 1287 44 is_stmt 1
	leaq	(%r11,%r9), %r12
	.loc	1 1287 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB34_114
	vmovss	(%rax,%r12,4), %xmm9
.Ltmp8088:
	.loc	1 905 8 is_stmt 1
	vminss	%xmm7, %xmm9, %xmm7
.Ltmp8089:
	.loc	1 1284 9
	vmovss	%xmm7, 544(%rsp,%rdi)
	jmp	.LBB34_123
.Ltmp8090:
	.loc	1 0 9 is_stmt 0
.Ltmp8091:
	.p2align	4
.LBB34_124:
	.loc	11 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm9
	movq	8(%rsp), %r11
.Ltmp8092:
	.loc	1 1414 26
	movq	1704(%r11), %rsi
	vmovaps	%ymm9, %ymm13
	movq	416(%rsp), %rdi
.Ltmp8093:
	.loc	12 37 12
	testq	%rdi, %rdi
	movq	248(%rsp), %r13
	movq	1344(%rsp), %rbx
	je	.LBB34_137
.Ltmp8094:
	.loc	12 0 12 is_stmt 0
	movq	1832(%r11), %r9
.Ltmp8095:
	.loc	1 1405 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB34_131
	.loc	1 0 42 is_stmt 0
	movq	1824(%r11), %r10
	.loc	1 1405 42
	movl	8(%r10), %r8d
	.loc	1 1405 28
	addq	192(%rsp), %r8
.Ltmp8096:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	.loc	1 1409 40
	imulq	%rdi, %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_173
	.loc	1 0 25
	movq	1696(%r11), %rdx
	.loc	1 1409 25
	vmovss	(%rdx,%r8,4), %xmm7
	.loc	1 1409 13
	vmovss	%xmm7, 544(%rsp)
.Ltmp8097:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB34_136
.Ltmp8098:
	.loc	1 1405 42
	cmpq	$1, %r9
	je	.LBB34_129
	movl	20(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8099:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_173
	vmovss	4(%rdx,%rax,4), %xmm7
	.loc	1 1409 13
	vmovss	%xmm7, 548(%rsp)
.Ltmp8100:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB34_136
.Ltmp8101:
	.loc	1 1405 42
	cmpq	$2, %r9
	je	.LBB34_150
	movl	32(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8102:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_173
	vmovss	8(%rdx,%rax,4), %xmm7
	.loc	1 1409 13
	vmovss	%xmm7, 552(%rsp)
.Ltmp8103:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB34_136
.Ltmp8104:
	.loc	1 1405 42
	cmpq	$3, %r9
	je	.LBB34_154
	movl	44(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8105:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_173
	vmovss	12(%rdx,%rax,4), %xmm7
	.loc	1 1409 13
	vmovss	%xmm7, 556(%rsp)
.Ltmp8106:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB34_136
.Ltmp8107:
	.loc	1 1405 42
	cmpq	$4, %r9
	je	.LBB34_158
	movl	56(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8108:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_173
	vmovss	16(%rdx,%rax,4), %xmm7
	.loc	1 1409 13
	vmovss	%xmm7, 560(%rsp)
.Ltmp8109:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB34_136
.Ltmp8110:
	.loc	1 1405 42
	cmpq	$5, %r9
	je	.LBB34_162
	movl	68(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8111:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_173
	vmovss	20(%rdx,%rax,4), %xmm7
	.loc	1 1409 13
	vmovss	%xmm7, 564(%rsp)
.Ltmp8112:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB34_136
.Ltmp8113:
	.loc	1 1405 42
	cmpq	$6, %r9
	je	.LBB34_166
	movl	80(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8114:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_173
	vmovss	24(%rdx,%rax,4), %xmm7
	.loc	1 1409 13
	vmovss	%xmm7, 568(%rsp)
.Ltmp8115:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB34_136
.Ltmp8116:
	.loc	1 1405 42
	cmpq	$7, %r9
	je	.LBB34_170
	movl	92(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8117:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %r8d
	cmovaeq	%rcx, %r8
	subq	%r8, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_173
	vmovss	28(%rdx,%rax,4), %xmm7
	.loc	1 1409 13
	vmovss	%xmm7, 572(%rsp)
.Ltmp8118:
	.loc	1 0 13
.Ltmp8119:
	.p2align	4
.LBB34_136:
	.loc	11 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm13
.Ltmp8120:
.LBB34_137:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm4
	vmulps	%ymm4, %ymm9, %ymm7
	vroundps	$9, %ymm7, %ymm7
	vbroadcastss	.LCPI34_4(%rip), %ymm4
	vmulps	%ymm4, %ymm7, %ymm7
.Ltmp8121:
	.loc	29 48 14 is_stmt 1
	vaddps	%ymm1, %ymm7, %ymm1
.Ltmp8122:
	.loc	29 347 14
	vsubps	%ymm13, %ymm1, %ymm1
.Ltmp8123:
	.loc	1 1413 5
	vmovaps	%ymm1, 3456(%rsp)
.Ltmp8124:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	736(%rsp), %rdx
	jb	.LBB34_627
.Ltmp8125:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_95
.Ltmp8126:
	.loc	1 1414 26
	movq	1696(%r11), %rax
	movq	736(%rsp), %rdx
.Ltmp8127:
	.loc	11 551 14
	vmovups	%ymm7, (%rax,%rdx,4)
.Ltmp8128:
	.loc	29 360 14
	vdivps	3488(%rsp), %ymm1, %ymm7
.Ltmp8129:
	.loc	1 1418 43
	vmovaps	3424(%rsp), %ymm9
.Ltmp8130:
	.loc	29 347 14
	vsubps	%ymm7, %ymm12, %ymm7
.Ltmp8131:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm9, %ymm7, %ymm15
.Ltmp8132:
	.loc	29 283 14 is_stmt 1
	vmulps	64(%rsp), %ymm15, %ymm15
.Ltmp8133:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8134:
	.loc	29 233 14
	vmaxps	%ymm9, %ymm7, %ymm7
.Ltmp8135:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm9
	vandps	%ymm7, %ymm9, %ymm15
.Ltmp8136:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm4
	vcmplt_oqps	%ymm4, %ymm15, %ymm15
.Ltmp8137:
	.loc	29 82 19
	vandnps	%ymm7, %ymm15, %ymm7
.Ltmp8138:
	.loc	1 1419 5
	vmovaps	%ymm7, 3424(%rsp)
.Ltmp8139:
	.loc	1 1422 28
	movq	1672(%r11), %rsi
	.loc	1 1422 44 is_stmt 0
	imulq	%rbx, %rdi
.Ltmp8140:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_144
.Ltmp8141:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_95
.Ltmp8142:
	.loc	5 0 16 is_stmt 0
	movq	344(%rsp), %rax
	movq	704(%rsp), %rsi
	leaq	(%rax,%rsi,4), %rax
.Ltmp8143:
	vsubps	%ymm7, %ymm12, %ymm7
.Ltmp8144:
	.loc	1 1422 28 is_stmt 1
	movq	1664(%r11), %rdx
.Ltmp8145:
	.loc	11 551 14
	vmovups	(%rdx,%rdi,4), %ymm15
.Ltmp8146:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rax), %ymm4
.Ltmp8147:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm7, %ymm15, %ymm7
	vmovaps	2432(%rsp), %ymm12
.Ltmp8148:
	.loc	29 585 19
	vblendvps	%ymm12, %ymm15, %ymm7, %ymm7
.Ltmp8149:
	.loc	11 551 14
	vmovups	%ymm4, (%rdx,%rdi,4)
.Ltmp8150:
	.loc	11 551 14 is_stmt 0
	vmovups	%ymm7, (%rax)
.Ltmp8151:
	.loc	5 568 12 is_stmt 1
	movq	%r13, %rdx
	subq	%rsi, %rdx
	jb	.LBB34_237
.Ltmp8152:
	.loc	5 438 16
	cmpq	$7, %rdx
	vmovaps	%ymm2, %ymm15
	jbe	.LBB34_143
.Ltmp8153:
	.loc	1 1394 25
	movq	1888(%r11), %rsi
.Ltmp8154:
	.loc	1 1390 17
	movq	2040(%r11), %rdi
.Ltmp8155:
	.loc	1 1394 45
	movq	%rdi, %r8
	imulq	192(%rsp), %r8
.Ltmp8156:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r8, %rdx
	jb	.LBB34_394
.Ltmp8157:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_143
.Ltmp8158:
	.loc	5 0 16 is_stmt 0
	vmovaps	1728(%rsp), %ymm4
	vblendvps	%ymm4, %ymm10, %ymm3, %ymm3
	vmovaps	96(%rsp), %ymm7
.Ltmp8159:
	vdivps	%ymm3, %ymm7, %ymm4
	vcmpgt_oqps	%ymm7, %ymm3, %ymm3
	vbroadcastss	.LCPI34_2(%rip), %ymm12
	vblendvps	%ymm3, %ymm4, %ymm12, %ymm3
.Ltmp8160:
	.loc	1 1394 25 is_stmt 1
	movq	1880(%r11), %rax
.Ltmp8161:
	.loc	11 551 14
	vmovups	%ymm3, (%rax,%r8,4)
.Ltmp8162:
	.loc	1 1261 17
	movq	2040(%r11), %rdx
.Ltmp8163:
	.loc	12 37 12
	testq	%rdx, %rdx
	movq	%rdi, 416(%rsp)
	je	.LBB34_198
.Ltmp8164:
	.loc	12 0 12 is_stmt 0
	movq	%r8, 1312(%rsp)
	movq	2032(%r11), %rax
	movq	%rax, 352(%rsp)
	movq	192(%rsp), %rax
	leaq	1(%rax), %r8
	cmpq	%rcx, %r8
	movq	%rcx, %rsi
	movl	$0, %eax
	cmovbq	%rax, %rsi
	movq	2024(%r11), %rax
	subq	%rsi, %r8
	movq	1880(%r11), %r13
	movq	1888(%r11), %rsi
	movq	1968(%r11), %rdi
	movq	%rdi, 448(%rsp)
	movq	1960(%r11), %r15
	movq	1936(%r11), %rdi
	movq	%rdi, 32(%rsp)
	movq	1928(%r11), %rdi
	movq	%rdi, 160(%rsp)
	imulq	%rdx, %r8
	movq	%r8, 736(%rsp)
	movq	%rdx, %r10
	xorl	%r11d, %r11d
	xorl	%r9d, %r9d
	jmp	.LBB34_181
	.p2align	4
.LBB34_234:
	xorl	%ebx, %ebx
.LBB34_196:
	decq	%r10
	addq	$4, %r9
.Ltmp8165:
	movl	%ebx, (%r15,%r11,4)
.Ltmp8166:
	incq	%r11
.Ltmp8167:
	.loc	12 37 12 is_stmt 1
	testq	%r10, %r10
	je	.LBB34_197
.LBB34_181:
.Ltmp8168:
	.loc	7 1714 9
	cmpq	$32, %r9
.Ltmp8169:
	.loc	6 180 28
	je	.LBB34_197
.Ltmp8170:
	.loc	1 1265 21
	cmpq	352(%rsp), %r11
	je	.LBB34_281
	leaq	(%r11,%r11,2), %r8
	movl	4(%rax,%r8,4), %r12d
.Ltmp8171:
	.loc	1 1267 23
	addq	192(%rsp), %r12
.Ltmp8172:
	.loc	1 1268 12
	cmpq	%rcx, %r12
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %r12
.Ltmp8173:
	.loc	1 1275 42
	movq	%r12, %rdi
	imulq	%rdx, %rdi
	addq	%r11, %rdi
	.loc	1 1275 22 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB34_628
.Ltmp8174:
	.loc	1 1276 24 is_stmt 1
	cmpq	448(%rsp), %r11
	je	.LBB34_284
.Ltmp8175:
	.loc	1 0 0 is_stmt 0
	movl	(%rax,%r8,4), %r8d
.Ltmp8176:
	vmovss	(%r13,%rdi,4), %xmm3
.Ltmp8177:
	.loc	1 1276 24
	movl	(%r15,%r11,4), %ebx
	testq	%rbx, %rbx
.Ltmp8178:
	.loc	1 1277 26 is_stmt 1
	je	.LBB34_188
	.loc	1 1280 24
	cmpq	32(%rsp), %r11
	jae	.LBB34_192
	.loc	1 0 24 is_stmt 0
	movq	160(%rsp), %r14
	.loc	1 1280 24
	vmovss	(%r14,%r11,4), %xmm7
.Ltmp8179:
	.loc	1 905 8 is_stmt 1
	vucomiss	%xmm7, %xmm3
	jbe	.LBB34_188
.Ltmp8180:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm7, %xmm3
.LBB34_188:
.Ltmp8181:
	.loc	1 1282 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB34_194
	.loc	1 0 9 is_stmt 0
	movq	160(%rsp), %r14
	.loc	1 1282 9
	vmovss	%xmm3, (%r14,%r11,4)
	.loc	1 1283 24 is_stmt 1
	incq	%rbx
	cmpq	%r8, %rbx
.Ltmp8182:
	.loc	1 1284 23
	jne	.LBB34_190
	.loc	1 1284 9 is_stmt 0
	vmovss	%xmm3, 544(%rsp,%r9)
	.loc	1 1290 30 is_stmt 1
	vmovss	(%r13,%rdi,4), %xmm3
	.loc	1 0 30 is_stmt 0
.Ltmp8183:
	.p2align	4
.LBB34_230:
.Ltmp8184:
	.loc	1 1293 65 is_stmt 1
	movq	%r12, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1293 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_231
.Ltmp8185:
	.loc	1 905 8 is_stmt 1
	vminss	(%r13,%r14,4), %xmm3, %xmm3
.Ltmp8186:
	.loc	1 1294 17
	vmovss	%xmm3, (%r13,%r14,4)
	.loc	1 1295 20
	testq	%r12, %r12
	cmoveq	%rcx, %r12
	.loc	1 1298 17
	decq	%r12
.Ltmp8187:
	.loc	10 1916 50
	decq	%r8
.Ltmp8188:
	.loc	3 900 12
	jne	.LBB34_230
	jmp	.LBB34_234
.Ltmp8189:
	.loc	3 0 12 is_stmt 0
.Ltmp8190:
	.p2align	4
.LBB34_190:
	movq	736(%rsp), %rdi
	.loc	1 1287 44 is_stmt 1
	leaq	(%r11,%rdi), %r14
	.loc	1 1287 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_191
	vmovss	(%r13,%r14,4), %xmm4
.Ltmp8191:
	.loc	1 905 8 is_stmt 1
	vminss	%xmm3, %xmm4, %xmm3
.Ltmp8192:
	.loc	1 1284 9
	vmovss	%xmm3, 544(%rsp,%r9)
	jmp	.LBB34_196
.Ltmp8193:
	.loc	1 0 9 is_stmt 0
.Ltmp8194:
	.p2align	4
.LBB34_197:
	.loc	11 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm13
	movq	8(%rsp), %r11
	movq	248(%rsp), %r13
	movq	1344(%rsp), %rbx
	movq	416(%rsp), %rdi
	movq	1312(%rsp), %r8
.Ltmp8195:
.LBB34_198:
	.loc	1 1414 26
	movq	1904(%r11), %rsi
	vmovaps	%ymm13, %ymm7
.Ltmp8196:
	.loc	12 37 12
	testq	%rdi, %rdi
	movq	864(%rsp), %r14
	movq	4256(%rsp), %r15
	je	.LBB34_224
.Ltmp8197:
	.loc	12 0 12 is_stmt 0
	movq	2032(%r11), %r9
.Ltmp8198:
	.loc	1 1405 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB34_131
	.loc	1 0 42 is_stmt 0
	movq	%r8, %r12
	movq	2024(%r11), %r10
	.loc	1 1405 42
	movl	8(%r10), %r8d
	.loc	1 1405 28
	addq	192(%rsp), %r8
.Ltmp8199:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	movq	416(%rsp), %rax
	.loc	1 1409 40
	imulq	%rax, %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_236
	.loc	1 0 25
	movq	1896(%r11), %rdx
	.loc	1 1409 25
	vmovss	(%rdx,%r8,4), %xmm3
	.loc	1 1409 13
	vmovss	%xmm3, 544(%rsp)
.Ltmp8200:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %rax
	je	.LBB34_223
.Ltmp8201:
	.loc	1 1405 42
	cmpq	$1, %r9
	je	.LBB34_129
	movl	20(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8202:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	416(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_236
	vmovss	4(%rdx,%rax,4), %xmm3
	.loc	1 1409 13
	vmovss	%xmm3, 548(%rsp)
.Ltmp8203:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB34_223
.Ltmp8204:
	.loc	1 1405 42
	cmpq	$2, %r9
	je	.LBB34_150
	movl	32(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8205:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	416(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_236
	vmovss	8(%rdx,%rax,4), %xmm3
	.loc	1 1409 13
	vmovss	%xmm3, 552(%rsp)
.Ltmp8206:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB34_223
.Ltmp8207:
	.loc	1 1405 42
	cmpq	$3, %r9
	je	.LBB34_154
	movl	44(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8208:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	416(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_236
	vmovss	12(%rdx,%rax,4), %xmm3
	.loc	1 1409 13
	vmovss	%xmm3, 556(%rsp)
.Ltmp8209:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB34_223
.Ltmp8210:
	.loc	1 1405 42
	cmpq	$4, %r9
	je	.LBB34_158
	movl	56(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8211:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	416(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_236
	vmovss	16(%rdx,%rax,4), %xmm3
	.loc	1 1409 13
	vmovss	%xmm3, 560(%rsp)
.Ltmp8212:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB34_223
.Ltmp8213:
	.loc	1 1405 42
	cmpq	$5, %r9
	je	.LBB34_162
	movl	68(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8214:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	416(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_236
	vmovss	20(%rdx,%rax,4), %xmm3
	.loc	1 1409 13
	vmovss	%xmm3, 564(%rsp)
.Ltmp8215:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB34_223
.Ltmp8216:
	.loc	1 1405 42
	cmpq	$6, %r9
	je	.LBB34_166
	movl	80(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8217:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	416(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_236
	vmovss	24(%rdx,%rax,4), %xmm3
	.loc	1 1409 13
	vmovss	%xmm3, 568(%rsp)
.Ltmp8218:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB34_223
.Ltmp8219:
	.loc	1 1405 42
	cmpq	$7, %r9
	je	.LBB34_170
	movl	92(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	192(%rsp), %rax
.Ltmp8220:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	.loc	1 1409 40
	imulq	416(%rsp), %rax
	leaq	7(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_236
	vmovss	28(%rdx,%rax,4), %xmm3
	.loc	1 1409 13
	vmovss	%xmm3, 572(%rsp)
.Ltmp8221:
	.loc	1 0 13
.Ltmp8222:
	.p2align	4
.LBB34_223:
	.loc	11 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm7
	movq	416(%rsp), %rdi
	movq	%r12, %r8
.Ltmp8223:
.LBB34_224:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm3
	vmulps	%ymm3, %ymm13, %ymm3
	vroundps	$9, %ymm3, %ymm3
	vbroadcastss	.LCPI34_4(%rip), %ymm4
	vmulps	%ymm4, %ymm3, %ymm3
.Ltmp8224:
	.loc	29 48 14 is_stmt 1
	vaddps	%ymm0, %ymm3, %ymm0
.Ltmp8225:
	.loc	29 347 14
	vsubps	%ymm7, %ymm0, %ymm0
.Ltmp8226:
	.loc	1 1413 5
	vmovaps	%ymm0, 4192(%rsp)
.Ltmp8227:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r8, %rdx
	jb	.LBB34_358
.Ltmp8228:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_143
.Ltmp8229:
	.loc	1 1414 26
	movq	1896(%r11), %rax
.Ltmp8230:
	.loc	11 551 14
	vmovups	%ymm3, (%rax,%r8,4)
.Ltmp8231:
	.loc	29 360 14
	vdivps	4224(%rsp), %ymm0, %ymm3
.Ltmp8232:
	.loc	1 1418 43
	vmovaps	4160(%rsp), %ymm4
.Ltmp8233:
	.loc	29 347 14
	vsubps	%ymm3, %ymm12, %ymm3
.Ltmp8234:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm4, %ymm3, %ymm7
.Ltmp8235:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm7, %ymm15, %ymm7
.Ltmp8236:
	.loc	29 48 14
	vaddps	%ymm7, %ymm4, %ymm4
.Ltmp8237:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm3, %ymm3
.Ltmp8238:
	.loc	29 82 19
	vandps	%ymm3, %ymm9, %ymm4
.Ltmp8239:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm7
	vcmplt_oqps	%ymm7, %ymm4, %ymm4
.Ltmp8240:
	.loc	29 82 19
	vandnps	%ymm3, %ymm4, %ymm3
.Ltmp8241:
	.loc	1 1419 5
	vmovaps	%ymm3, 4160(%rsp)
.Ltmp8242:
	.loc	1 1422 28
	movq	1872(%r11), %rsi
	.loc	1 1422 44 is_stmt 0
	imulq	%rbx, %rdi
.Ltmp8243:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_235
.Ltmp8244:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_143
.Ltmp8245:
	.loc	5 0 16 is_stmt 0
	movq	768(%rsp), %r8
	incq	%r8
	movq	408(%rsp), %rax
	movq	704(%rsp), %rdx
.Ltmp8246:
	leaq	(%rax,%rdx,4), %rax
.Ltmp8247:
	vsubps	%ymm3, %ymm12, %ymm3
.Ltmp8248:
	.loc	1 1422 28 is_stmt 1
	movq	1864(%r11), %rdx
.Ltmp8249:
	.loc	11 551 14
	vmovups	(%rdx,%rdi,4), %ymm4
.Ltmp8250:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rax), %ymm7
.Ltmp8251:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm4, %ymm3, %ymm3
	vmovaps	2432(%rsp), %ymm9
.Ltmp8252:
	.loc	29 585 19
	vblendvps	%ymm9, %ymm4, %ymm3, %ymm3
.Ltmp8253:
	.loc	11 551 14
	vmovups	%ymm7, (%rdx,%rdi,4)
.Ltmp8254:
	.loc	1 1883 13
	incq	%rbx
	.loc	1 1884 16
	cmpq	1632(%r11), %rbx
.Ltmp8255:
	.loc	11 551 14
	vmovups	%ymm3, (%rax)
	movl	$0, %edx
.Ltmp8256:
	.loc	1 1884 16
	cmoveq	%rdx, %rbx
	movq	192(%rsp), %rsi
	.loc	1 1887 13
	incq	%rsi
	.loc	1 1888 16
	cmpq	%rcx, %rsi
	movq	%r8, %rcx
	movl	$0, %eax
	movq	%rax, 832(%rsp)
	cmoveq	%rdx, %rsi
	movq	%rsi, 192(%rsp)
.Ltmp8257:
	.loc	10 1916 50
	cmpq	2720(%rsp), %r8
	movq	24(%rsp), %rsi
	movq	2336(%rsp), %rax
.Ltmp8258:
	.loc	3 900 12
	jne	.LBB34_93
	jmp	.LBB34_77
.Ltmp8259:
.LBB34_238:
	.loc	3 0 12 is_stmt 0
	leaq	2784(%rsp), %rdi
	movq	536(%rsp), %rsi
	.loc	1 1894 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	3520(%rsp), %rdi
	jmp	.LBB34_239
.Ltmp8260:
.LBB34_241:
	.loc	1 0 14 is_stmt 0
	movq	64(%rsp), %rbx
.LBB34_242:
	leaq	7776(%rsp), %rdi
	movq	536(%rsp), %rsi
.Ltmp8261:
	.loc	1 1894 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	8512(%rsp), %rdi
.Ltmp8262:
.LBB34_239:
	.loc	1 0 14 is_stmt 0
	movq	528(%rsp), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	8(%rsp), %r9
	movl	%ebx, 1640(%r9)
	movq	192(%rsp), %rax
.LBB34_554:
	movl	%eax, 1644(%r9)
.Ltmp8263:
	.loc	1 2262 35 is_stmt 1
	cmpb	$0, 2588(%rsp)
	je	.LBB34_555
	.loc	1 0 35 is_stmt 0
	movq	536(%rsp), %rdi
	.loc	1 2263 26 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2263 16 is_stmt 0
	testb	%al, %al
	je	.LBB34_587
	.loc	1 0 16
	movq	528(%rsp), %rdi
	.loc	1 2264 27 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2264 16 is_stmt 0
	testb	%al, %al
	je	.LBB34_587
	.loc	1 0 16
	movq	24(%rsp), %rdx
	movq	4336(%rsp), %rsi
	cmpq	%rdx, %rsi
	movq	344(%rsp), %r10
.Ltmp8264:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB34_654
.Ltmp8265:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rax
	movq	%r10, %rcx
	movq	8(%rsp), %r9
	.p2align	4
.LBB34_591:
.Ltmp8266:
	.loc	15 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB34_615
.Ltmp8267:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp8268:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%r11d, %r11d
	xorl	%r8d, %r8d
.Ltmp8269:
	.loc	16 0 18 is_stmt 0
.Ltmp8270:
	.p2align	4
.LBB34_593:
	.loc	17 134 13 is_stmt 1
	orl	(%rcx,%r8), %r11d
.Ltmp8271:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp8272:
	.loc	6 180 28
	jne	.LBB34_593
.Ltmp8273:
	.loc	18 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp8274:
	.loc	19 2054 74
	subq	%rdx, %rax
.Ltmp8275:
	.loc	17 136 12
	testl	%r11d, %r11d
	movq	24(%rsp), %rdx
	je	.LBB34_591
.Ltmp8276:
	.loc	17 0 12 is_stmt 0
	xorl	%ecx, %ecx
	jmp	.LBB34_596
.LBB34_555:
	xorl	%ecx, %ecx
	jmp	.LBB34_556
.LBB34_587:
	xorl	%ecx, %ecx
	movq	8(%rsp), %r9
.LBB34_556:
	movq	24(%rsp), %rdx
	movq	344(%rsp), %r10
.LBB34_596:
	movabsq	$2305843009213693944, %rax
	.loc	1 2262 9 is_stmt 1
	movb	%cl, 2152(%r9)
	.loc	1 2267 30
	movzbl	2144(%r9), %ecx
	.loc	1 2267 9 is_stmt 0
	movb	%cl, 2153(%r9)
	movq	4344(%rsp), %rsi
	.loc	1 2268 21 is_stmt 1
	movq	16(%rsi), %rcx
	movq	%rcx, 9264(%rsp)
	vmovups	(%rsi), %xmm0
	vmovaps	%xmm0, 9248(%rsp)
.Ltmp8277:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
.Ltmp8278:
	.loc	19 2155 12
	movq	%rdx, %rcx
	vmovaps	%ymm0, %ymm1
	andq	%rax, %rcx
	je	.LBB34_599
.Ltmp8279:
	.loc	19 0 12 is_stmt 0
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	xorl	%edx, %edx
	vbroadcastss	.LCPI34_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB34_598:
.Ltmp8280:
	.loc	29 82 19 is_stmt 1
	vandps	(%r10,%rdx,4), %ymm2, %ymm4
.Ltmp8281:
	.loc	29 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp8282:
	.loc	29 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp8283:
	.loc	19 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB34_598
.Ltmp8284:
.LBB34_599:
	.loc	1 0 0 is_stmt 0
	movl	2120(%r9), %r8d
.Ltmp8285:
	.loc	1 2275 0 is_stmt 1
	movq	1584(%r9), %rbx
	movq	1592(%r9), %r14
	movq	1600(%r9), %r11
	movq	1608(%r9), %r9
.Ltmp8286:
	.loc	30 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp8287:
	.loc	17 208 8
	jae	.LBB34_604
.Ltmp8288:
	.loc	19 2155 12
	movq	%r13, %rdx
	vmovaps	%ymm0, %ymm1
	andq	%rax, %rdx
	movq	408(%rsp), %rdi
	je	.LBB34_603
.Ltmp8289:
	.loc	19 0 12 is_stmt 0
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	xorl	%esi, %esi
	vbroadcastss	.LCPI34_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB34_602:
.Ltmp8290:
	.loc	29 82 19 is_stmt 1
	vandps	(%rdi,%rsi,4), %ymm2, %ymm4
.Ltmp8291:
	.loc	29 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp8292:
	.loc	29 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp8293:
	.loc	19 2155 12
	addq	$8, %rsi
	cmpq	%rsi, %rdx
	jne	.LBB34_602
.Ltmp8294:
.LBB34_603:
	.loc	30 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp8295:
	.loc	17 208 34
	jb	.LBB34_585
.LBB34_604:
	.loc	17 0 34 is_stmt 0
	movq	%r9, 160(%rsp)
	movq	%r11, 32(%rsp)
	vmovaps	%ymm0, %ymm1
.Ltmp8296:
	.loc	19 2155 12 is_stmt 1
	testq	%rcx, %rcx
.Ltmp8297:
	.loc	19 2155 12 is_stmt 0
	je	.LBB34_607
.Ltmp8298:
	.loc	19 0 12
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	xorl	%edx, %edx
	vbroadcastss	.LCPI34_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB34_606:
.Ltmp8299:
	.loc	29 82 19 is_stmt 1
	vandps	(%r10,%rdx,4), %ymm2, %ymm4
.Ltmp8300:
	.loc	29 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp8301:
	.loc	29 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp8302:
	.loc	19 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB34_606
.Ltmp8303:
.LBB34_607:
	.loc	29 585 19
	vpsrad	$31, %ymm1, %ymm2
	vpbroadcastd	.LCPI34_2(%rip), %ymm1
	vpandn	%ymm1, %ymm2, %ymm2
.Ltmp8304:
	.loc	17 185 12
	vmovd	%xmm2, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	movl	%edx, 736(%rsp)
	vpextrd	$1, %xmm2, %edx
	xorl	%ecx, %ecx
	testl	%edx, %edx
	setne	%cl
	vpextrd	$2, %xmm2, %edx
	addl	%ecx, %ecx
	xorl	%esi, %esi
	testl	%edx, %edx
	setne	%sil
	shll	$2, %esi
	movl	%esi, 416(%rsp)
	vpextrd	$3, %xmm2, %edx
	xorl	%esi, %esi
	testl	%edx, %edx
	setne	%sil
	shll	$3, %esi
	vextracti128	$1, %ymm2, %xmm2
	vmovd	%xmm2, %edx
	xorl	%edi, %edi
	testl	%edx, %edx
	setne	%dil
	shll	$4, %edi
	movl	%edi, 64(%rsp)
	vpextrd	$1, %xmm2, %edx
	xorl	%r10d, %r10d
	testl	%edx, %edx
	setne	%r10b
	vpextrd	$2, %xmm2, %edx
	shll	$5, %r10d
	xorl	%r9d, %r9d
	testl	%edx, %edx
	setne	%r9b
	shll	$6, %r9d
	vpextrd	$3, %xmm2, %edx
	xorl	%r11d, %r11d
	testl	%edx, %edx
	setne	%r11b
	shll	$7, %r11d
.Ltmp8305:
	.loc	19 2155 12
	andq	%r13, %rax
	movl	%r8d, 192(%rsp)
	movq	%rbx, 352(%rsp)
	movq	%r14, 448(%rsp)
	je	.LBB34_610
.Ltmp8306:
	.loc	19 0 12 is_stmt 0
	xorl	%edx, %edx
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	vbroadcastss	.LCPI34_6(%rip), %ymm3
	movq	408(%rsp), %rdi
	.p2align	4
.LBB34_609:
.Ltmp8307:
	.loc	29 82 19 is_stmt 1
	vandps	(%rdi,%rdx,4), %ymm2, %ymm4
.Ltmp8308:
	.loc	29 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp8309:
	.loc	29 82 19
	vandps	%ymm4, %ymm0, %ymm0
.Ltmp8310:
	.loc	19 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rax
	jne	.LBB34_609
.Ltmp8311:
.LBB34_610:
	.loc	29 585 19
	vpsrad	$31, %ymm0, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
.Ltmp8312:
	.loc	17 185 12
	vmovd	%xmm0, %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	vpextrd	$1, %xmm0, %edx
	setne	%al
	xorl	%ebx, %ebx
	testl	%edx, %edx
	setne	%bl
	addl	%ebx, %ebx
	vpextrd	$2, %xmm0, %edx
	xorl	%r14d, %r14d
	testl	%edx, %edx
	setne	%r14b
	vpextrd	$3, %xmm0, %edx
	shll	$2, %r14d
	xorl	%r15d, %r15d
	testl	%edx, %edx
	setne	%r15b
	shll	$3, %r15d
	vextracti128	$1, %ymm0, %xmm0
	vmovd	%xmm0, %edx
	xorl	%r13d, %r13d
	testl	%edx, %edx
	setne	%r13b
	vpextrd	$1, %xmm0, %edx
	shll	$4, %r13d
	xorl	%r12d, %r12d
	testl	%edx, %edx
	setne	%r12b
	shll	$5, %r12d
	vpextrd	$2, %xmm0, %edx
	xorl	%edi, %edi
	testl	%edx, %edx
	setne	%dil
	vpextrd	$3, %xmm0, %r8d
	shll	$6, %edi
	xorl	%edx, %edx
	testl	%r8d, %r8d
	setne	%dl
	shll	$7, %edx
.Ltmp8313:
	.loc	17 185 12 is_stmt 0
	orl	%edi, %edx
.Ltmp8314:
	.loc	17 185 12
	orl	736(%rsp), %ecx
	orl	416(%rsp), %ecx
	orl	64(%rsp), %esi
	orl	%r10d, %esi
	orl	%ecx, %esi
	orl	%r11d, %r9d
	orl	%esi, %r9d
.Ltmp8315:
	.loc	17 185 12
	orl	%eax, %r9d
	orl	%ebx, %r9d
	orl	%r14d, %r15d
	orl	%r9d, %r15d
	orl	%r13d, %r12d
	orl	%r15d, %r12d
.Ltmp8316:
	.loc	17 211 5 is_stmt 1
	orl	%edx, %r12d
	movq	8(%rsp), %rdx
	movl	%r12d, 1576(%rdx)
	.loc	17 212 31
	movq	1568(%rdx), %rax
.Ltmp8317:
	.loc	4 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp8318:
	.loc	17 212 5
	movq	%rcx, 1568(%rdx)
	movq	24(%rsp), %rdx
.Ltmp8319:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp8320:
	.loc	6 180 28
	je	.LBB34_612
.Ltmp8321:
	.loc	16 961 18
	shlq	$2, %rdx
	movq	344(%rsp), %rdi
.Ltmp8322:
	.loc	20 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp8323:
.LBB34_612:
	.loc	20 0 13 is_stmt 0
	movq	248(%rsp), %rdx
.Ltmp8324:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
	movq	408(%rsp), %rdi
.Ltmp8325:
	.loc	6 180 28
	je	.LBB34_614
.Ltmp8326:
	.loc	16 961 18
	shlq	$2, %rdx
.Ltmp8327:
	.loc	20 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp8328:
.LBB34_614:
	.loc	1 2276 18
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %rbx
	leaq	9248(%rsp), %r14
	movq	536(%rsp), %rdi
	movq	%r14, %rsi
	movq	352(%rsp), %rdx
	movq	448(%rsp), %rcx
	movl	192(%rsp), %r15d
	movl	%r15d, %r8d
	vzeroupper
	callq	*%rbx
	movq	528(%rsp), %rdi
	.loc	1 2277 19
	movq	%r14, %rsi
	movq	32(%rsp), %rdx
	movq	160(%rsp), %rcx
	movl	%r15d, %r8d
	callq	*%rbx
	movq	8(%rsp), %rax
	.loc	1 2278 13
	movq	$0, 1640(%rax)
.Ltmp8329:
.LBB34_585:
	.loc	1 2280 6
	leaq	-40(%rbp), %rsp
	.loc	1 2280 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	vzeroupper
	retq
.LBB34_397:
	.cfi_def_cfa %rbp, 16
.Ltmp8330:
	.loc	1 1698 12 is_stmt 1
	testb	%dl, %dl
	je	.LBB34_398
	.loc	1 0 12 is_stmt 0
	leaq	5824(%rsp), %rdi
	movq	536(%rsp), %r14
.Ltmp8331:
	.loc	1 1948 24 is_stmt 1
	movq	%r14, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	6560(%rsp), %rdi
	movq	528(%rsp), %rsi
.Ltmp8332:
	.loc	1 1949 25
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	movq	8(%rsp), %r13
.Ltmp8333:
	.loc	1 1954 19
	movzbl	1536(%r13), %eax
	movb	%al, 32(%rsp)
.Ltmp8334:
	.loc	1 1955 21
	movzbl	1537(%r13), %eax
	movb	%al, 192(%rsp)
.Ltmp8335:
	.loc	1 1956 16
	movq	1624(%r13), %r12
.Ltmp8336:
	.loc	1 1957 16
	movq	1632(%r13), %r15
.Ltmp8337:
	.loc	1 1958 27
	movl	1640(%r13), %eax
	movq	%rax, 768(%rsp)
.Ltmp8338:
	.loc	1 1959 27
	movl	1644(%r13), %eax
	movq	%rax, 2592(%rsp)
	leaq	10280(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %rbx
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	leaq	9248(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	leaq	896(%rsp), %rdi
.Ltmp8339:
	.loc	1 1966 32
	movq	%r14, %rsi
	movq	%r12, %rdx
	movq	%r15, 4320(%rsp)
	movq	%r15, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
.Ltmp8340:
	.loc	1 1967 33
	movq	1624(%r13), %rdx
	movq	1632(%r13), %rcx
	leaq	544(%rsp), %rdi
	movq	528(%rsp), %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	16(%rsp), %rcx
.Ltmp8341:
	.loc	4 3758 16
	leaq	31(%rcx), %r9
	shrq	$5, %r9
.Ltmp8342:
	.loc	8 446 20
	je	.LBB34_477
.Ltmp8343:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp8344:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 704(%rsp)
	vmovaps	%ymm1, 96(%rsp)
	cmpb	$0, 32(%rsp)
	movq	248(%rsp), %r13
	movq	24(%rsp), %rsi
	jne	.LBB34_480
.Ltmp8345:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 96(%rsp)
.LBB34_480:
	cmpb	$0, 192(%rsp)
	movq	8(%rsp), %r11
	movq	768(%rsp), %r14
	jne	.LBB34_482
	vmovaps	%ymm0, 704(%rsp)
.LBB34_482:
	vmovaps	5824(%rsp), %ymm0
	vmovaps	%ymm0, 1056(%rsp)
	vmovaps	5856(%rsp), %ymm0
	vmovaps	%ymm0, 1088(%rsp)
	vmovaps	5888(%rsp), %ymm11
	vmovaps	5920(%rsp), %ymm0
	vmovaps	%ymm0, 7424(%rsp)
	vmovaps	5952(%rsp), %ymm3
	vmovaps	5984(%rsp), %ymm1
	vmovaps	%ymm1, 7392(%rsp)
	vmovaps	6016(%rsp), %ymm4
	vmovaps	6048(%rsp), %ymm13
	vmovaps	6080(%rsp), %ymm0
	vmovaps	%ymm0, 448(%rsp)
	vmovaps	6112(%rsp), %ymm0
	vmovaps	%ymm0, 352(%rsp)
	vmovaps	6144(%rsp), %ymm1
	vmovaps	%ymm1, 7360(%rsp)
	vmovaps	6176(%rsp), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	6560(%rsp), %ymm10
	vmovaps	6592(%rsp), %ymm1
	vmovaps	%ymm1, 1024(%rsp)
	vmovaps	6624(%rsp), %ymm6
	vmovaps	6656(%rsp), %ymm5
	movl	1000(%rsp), %r10d
	movl	648(%rsp), %eax
	movl	%eax, 4(%rsp)
	movl	$32, %r15d
	movabsq	$2305843009213693944, %rax
	addq	$7, %rax
	movq	%rax, 4296(%rsp)
	movq	408(%rsp), %rax
	movq	%rax, 2432(%rsp)
	movq	%r13, 2768(%rsp)
	movq	344(%rsp), %rax
	movq	%rax, 1728(%rsp)
	movq	%rsi, %rdx
	movl	$0, %eax
	movq	%rax, 416(%rsp)
	movq	%rcx, %rax
	xorl	%r8d, %r8d
	vmovaps	6688(%rsp), %ymm8
	vmovaps	6720(%rsp), %ymm9
	vmovaps	6752(%rsp), %ymm7
	vmovaps	6784(%rsp), %ymm14
	vmovaps	6816(%rsp), %ymm0
	vmovaps	%ymm0, 7328(%rsp)
	vmovaps	6848(%rsp), %ymm0
	vmovaps	6880(%rsp), %ymm1
	vmovaps	%ymm1, 7296(%rsp)
	vmovaps	6912(%rsp), %ymm1
	vmovaps	%ymm1, 480(%rsp)
	vmovaps	896(%rsp), %ymm1
	vmovaps	%ymm1, 1152(%rsp)
	vmovaps	544(%rsp), %ymm1
	vmovaps	%ymm1, 1184(%rsp)
	vmovaps	6496(%rsp), %ymm2
	vmovaps	7232(%rsp), %ymm1
	vmovaps	%ymm1, 1344(%rsp)
.LBB34_485:
	vmovaps	%ymm7, 7520(%rsp)
	vmovaps	%ymm9, 7552(%rsp)
	vmovaps	%ymm8, 7584(%rsp)
	vmovaps	%ymm2, 7488(%rsp)
	vmovaps	%ymm0, 736(%rsp)
	vmovaps	%ymm14, 32(%rsp)
	vmovaps	%ymm5, 2624(%rsp)
	vmovaps	%ymm10, 1120(%rsp)
	vmovaps	%ymm6, 800(%rsp)
.Ltmp8346:
	.loc	4 2584 13 is_stmt 1
	cmpq	$32, %rax
	movl	$32, %edi
	movq	%rax, 4304(%rsp)
	cmovbq	%rax, %rdi
	cmpq	$1, %rdi
	movq	%rdi, 4328(%rsp)
	movq	%rdi, %rax
	adcq	$0, %rax
	shll	$3, %eax
.Ltmp8347:
	.loc	10 1916 50
	movq	%rcx, %rdi
	subq	%r8, %rdi
.Ltmp8348:
	.loc	10 1078 5
	cmpq	$32, %rdi
	cmovaeq	%r15, %rdi
	movq	%rdi, 2776(%rsp)
	movq	%r8, 2328(%rsp)
.Ltmp8349:
	.loc	10 1916 50
	cmpq	%r8, %rcx
	movq	%rdx, 4312(%rsp)
	vmovaps	%ymm3, 160(%rsp)
.Ltmp8350:
	.loc	3 900 12
	jne	.LBB34_487
	.loc	3 0 12 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm11, %ymm7
	vmovaps	1088(%rsp), %ymm11
	vmovaps	7424(%rsp), %ymm3
	vmovaps	%ymm13, %ymm2
	vmovaps	448(%rsp), %ymm8
	vmovaps	352(%rsp), %ymm9
	vmovaps	7392(%rsp), %ymm6
	vmovaps	%ymm4, %ymm13
	vmovaps	7360(%rsp), %ymm5
	.loc	3 900 12
	jmp	.LBB34_495
.Ltmp8351:
.LBB34_487:
	.loc	3 0 12
	vmovaps	(%r11), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	32(%r11), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	64(%r11), %ymm0
	vmovaps	%ymm0, 288(%rsp)
	vmovaps	96(%r11), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	vmovaps	128(%r11), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	160(%r11), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	192(%r11), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	224(%r11), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	256(%r11), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	288(%r11), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	320(%r11), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	352(%r11), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	xorl	%ecx, %ecx
	vmovaps	7360(%rsp), %ymm12
	vmovaps	352(%rsp), %ymm1
	vmovaps	448(%rsp), %ymm15
	vmovaps	%ymm13, %ymm14
	vmovaps	384(%r11), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	416(%r11), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	448(%r11), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	480(%r11), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	512(%r11), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	544(%r11), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	576(%r11), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	608(%r11), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	640(%r11), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	672(%r11), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	704(%r11), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	736(%r11), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	768(%r11), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	800(%r11), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	832(%r11), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	864(%r11), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	896(%r11), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	928(%r11), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	960(%r11), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	992(%r11), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1024(%r11), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	1056(%r11), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1088(%r11), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovaps	1120(%r11), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1152(%r11), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1184(%r11), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1216(%r11), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	1248(%r11), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1280(%r11), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1312(%r11), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	1344(%r11), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	1376(%r11), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	1408(%r11), %ymm0
	vmovaps	%ymm0, 864(%rsp)
	vmovaps	1440(%r11), %ymm0
	vmovaps	%ymm0, 4256(%rsp)
	vmovaps	1472(%r11), %ymm0
	vmovaps	%ymm0, 832(%rsp)
	vmovaps	1504(%r11), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	1088(%rsp), %ymm10
	vmovaps	%ymm11, 1216(%rsp)
	vmovaps	%ymm11, %ymm7
	vmovaps	1056(%rsp), %ymm0
	vmovaps	7424(%rsp), %ymm3
	vmovaps	7392(%rsp), %ymm6
	vmovaps	%ymm4, %ymm13
	.p2align	4
.LBB34_488:
	vmovaps	%ymm14, 448(%rsp)
	vmovaps	%ymm15, 352(%rsp)
	vmovaps	160(%rsp), %ymm4
	movq	416(%rsp), %rdi
.Ltmp8352:
	.loc	5 568 12 is_stmt 1
	addq	%rcx, %rdi
	cmpq	%rsi, %rdi
	ja	.LBB34_492
.Ltmp8353:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_490
.Ltmp8354:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm0, %ymm11
	vmovaps	%ymm13, %ymm2
	vmovaps	%ymm1, %ymm5
	vmovaps	%ymm3, %ymm0
	movq	1728(%rsp), %rdi
.Ltmp8355:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rdi,%rcx,4), %ymm8
.Ltmp8356:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm3
	vmovaps	%ymm3, 192(%rsp)
	vmovaps	%ymm7, %ymm3
.Ltmp8357:
	.loc	29 283 14
	vmulps	64(%rsp), %ymm8, %ymm1
	vxorps	%xmm9, %xmm9, %xmm9
.Ltmp8358:
	.loc	29 48 14
	vaddps	%ymm1, %ymm9, %ymm1
	vmovaps	%ymm10, %ymm7
.Ltmp8359:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm8, %ymm10
.Ltmp8360:
	.loc	29 48 14
	vaddps	%ymm9, %ymm10, %ymm10
	vmovaps	%ymm6, %ymm13
	vmovaps	%ymm4, %ymm6
.Ltmp8361:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm8, %ymm14
.Ltmp8362:
	.loc	29 48 14
	vaddps	%ymm9, %ymm14, %ymm14
.Ltmp8363:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm8, %ymm15
.Ltmp8364:
	.loc	29 48 14
	vaddps	%ymm9, %ymm15, %ymm15
.Ltmp8365:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm11, %ymm4
.Ltmp8366:
	.loc	29 48 14
	vaddps	%ymm4, %ymm1, %ymm1
.Ltmp8367:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm11, %ymm4
.Ltmp8368:
	.loc	29 48 14
	vaddps	%ymm4, %ymm10, %ymm4
.Ltmp8369:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm11, %ymm10
.Ltmp8370:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8371:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm11, %ymm14
.Ltmp8372:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp8373:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm7, %ymm15
.Ltmp8374:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8375:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm7, %ymm15
.Ltmp8376:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8377:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm7, %ymm15
.Ltmp8378:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp8379:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm7, %ymm15
.Ltmp8380:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8381:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm3, %ymm15
.Ltmp8382:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8383:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm3, %ymm15
.Ltmp8384:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8385:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm3, %ymm15
.Ltmp8386:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp8387:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm3, %ymm15
.Ltmp8388:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8389:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm0, %ymm15
.Ltmp8390:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8391:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm0, %ymm15
.Ltmp8392:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8393:
	.loc	29 283 14
	vmulps	1632(%rsp), %ymm0, %ymm15
.Ltmp8394:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
	vmovaps	%ymm0, 160(%rsp)
.Ltmp8395:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm0, %ymm15
	vmovaps	%ymm8, %ymm0
.Ltmp8396:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8397:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm6, %ymm15
.Ltmp8398:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8399:
	.loc	29 283 14
	vmulps	1600(%rsp), %ymm6, %ymm15
.Ltmp8400:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8401:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm6, %ymm15
.Ltmp8402:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp8403:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm6, %ymm15
.Ltmp8404:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8405:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm13, %ymm15
.Ltmp8406:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8407:
	.loc	29 283 14
	vmulps	1568(%rsp), %ymm13, %ymm15
.Ltmp8408:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8409:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm13, %ymm15
.Ltmp8410:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp8411:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm13, %ymm15
.Ltmp8412:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8413:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm2, %ymm15
.Ltmp8414:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8415:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm2, %ymm15
.Ltmp8416:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8417:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm2, %ymm15
.Ltmp8418:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp8419:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm2, %ymm15
.Ltmp8420:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
	vmovaps	448(%rsp), %ymm8
.Ltmp8421:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm8, %ymm15
.Ltmp8422:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8423:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm8, %ymm15
.Ltmp8424:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8425:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm8, %ymm15
.Ltmp8426:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp8427:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm8, %ymm15
.Ltmp8428:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
	vmovaps	352(%rsp), %ymm9
.Ltmp8429:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm9, %ymm15
.Ltmp8430:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8431:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm9, %ymm15
.Ltmp8432:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8433:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm9, %ymm15
.Ltmp8434:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp8435:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm9, %ymm15
.Ltmp8436:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8437:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm5, %ymm15
.Ltmp8438:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8439:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm5, %ymm15
.Ltmp8440:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8441:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm5, %ymm15
.Ltmp8442:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
.Ltmp8443:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm5, %ymm15
.Ltmp8444:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8445:
	.loc	29 283 14
	vmulps	864(%rsp), %ymm12, %ymm15
.Ltmp8446:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8447:
	.loc	29 283 14
	vmulps	4256(%rsp), %ymm12, %ymm15
.Ltmp8448:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8449:
	.loc	29 283 14
	vmulps	832(%rsp), %ymm12, %ymm15
.Ltmp8450:
	.loc	29 48 14
	vaddps	%ymm15, %ymm10, %ymm10
	vmovaps	%ymm12, 256(%rsp)
.Ltmp8451:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm12, %ymm15
.Ltmp8452:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
	vmovaps	192(%rsp), %ymm12
.Ltmp8453:
	.loc	29 82 19
	vandps	%ymm12, %ymm13, %ymm15
.Ltmp8454:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm1, %ymm12, %ymm1
.Ltmp8455:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm1, %ymm15, %ymm1
.Ltmp8456:
	.loc	29 82 19
	vandps	%ymm4, %ymm12, %ymm4
.Ltmp8457:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm1, %ymm1
.Ltmp8458:
	.loc	29 82 19
	vandps	%ymm12, %ymm10, %ymm4
.Ltmp8459:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm1, %ymm1
.Ltmp8460:
	.loc	29 82 19
	vandps	%ymm12, %ymm14, %ymm4
.Ltmp8461:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm1, %ymm1
.Ltmp8462:
	.loc	11 551 14
	vmovups	%ymm1, 10280(%rsp,%rcx,4)
.Ltmp8463:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm11, %ymm10
	vmovaps	%ymm5, %ymm12
	vmovaps	%ymm9, %ymm1
	vmovaps	%ymm8, %ymm15
	vmovaps	%ymm2, %ymm14
	cmpq	%rcx, %rax
.Ltmp8464:
	.loc	3 900 12
	jne	.LBB34_488
.Ltmp8465:
.LBB34_495:
	.loc	3 0 12 is_stmt 0
	vmovaps	%ymm13, 7616(%rsp)
	vmovaps	%ymm9, 352(%rsp)
	vmovaps	%ymm8, 448(%rsp)
	vmovaps	%ymm2, 7456(%rsp)
	vmovaps	%ymm11, 1088(%rsp)
	vmovaps	%ymm7, 1216(%rsp)
	vmovaps	%ymm0, 1056(%rsp)
	vmovaps	%ymm3, 7424(%rsp)
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm3, 5920(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 5952(%rsp)
	vmovaps	%ymm6, 7392(%rsp)
	vmovaps	%ymm6, 5984(%rsp)
	vmovaps	%ymm13, 6016(%rsp)
	vmovaps	%ymm2, 6048(%rsp)
	vmovaps	%ymm8, 6080(%rsp)
	vmovaps	%ymm9, 6112(%rsp)
	vmovaps	%ymm5, 7360(%rsp)
	vmovaps	%ymm5, 6144(%rsp)
	vmovaps	256(%rsp), %ymm0
	vmovaps	%ymm0, 6176(%rsp)
	movq	2328(%rsp), %rcx
.Ltmp8466:
	.loc	10 1916 50
	cmpq	%rcx, 16(%rsp)
.Ltmp8467:
	.loc	3 900 12
	jne	.LBB34_497
	.loc	3 0 12 is_stmt 0
	vmovaps	1024(%rsp), %ymm12
	vmovaps	800(%rsp), %ymm10
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	2624(%rsp), %ymm11
	vmovaps	32(%rsp), %ymm13
	vmovaps	7328(%rsp), %ymm3
	vmovaps	7296(%rsp), %ymm2
	vmovaps	7584(%rsp), %ymm8
	vmovaps	7552(%rsp), %ymm9
	vmovaps	7520(%rsp), %ymm7
	.loc	3 900 12
	jmp	.LBB34_501
.Ltmp8468:
.LBB34_497:
	.loc	3 0 12
	vmovaps	(%r11), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	32(%r11), %ymm0
	vmovaps	%ymm0, 288(%rsp)
	vmovaps	64(%r11), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	vmovaps	96(%r11), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	128(%r11), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	160(%r11), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	192(%r11), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	224(%r11), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	256(%r11), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	288(%r11), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	320(%r11), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	352(%r11), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	movq	2768(%rsp), %rdx
	xorl	%ecx, %ecx
	vmovaps	7520(%rsp), %ymm5
	vmovaps	7552(%rsp), %ymm1
	vmovaps	7584(%rsp), %ymm15
	vmovaps	2624(%rsp), %ymm14
	vmovaps	384(%r11), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	416(%r11), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	448(%r11), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	480(%r11), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	512(%r11), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	544(%r11), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	576(%r11), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	608(%r11), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	640(%r11), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	672(%r11), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	704(%r11), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	736(%r11), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	768(%r11), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	800(%r11), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	832(%r11), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	864(%r11), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	896(%r11), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	928(%r11), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	960(%r11), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	992(%r11), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	1024(%r11), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1056(%r11), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovaps	1088(%r11), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1120(%r11), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1152(%r11), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1184(%r11), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	1216(%r11), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1248(%r11), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1280(%r11), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	1312(%r11), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	1344(%r11), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	1376(%r11), %ymm0
	vmovaps	%ymm0, 864(%rsp)
	vmovaps	1408(%r11), %ymm0
	vmovaps	%ymm0, 4256(%rsp)
	vmovaps	1440(%r11), %ymm0
	vmovaps	%ymm0, 832(%rsp)
	vmovaps	1472(%r11), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	1504(%r11), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	800(%rsp), %ymm6
	vmovaps	1024(%rsp), %ymm12
	vmovaps	32(%rsp), %ymm13
	vmovaps	7328(%rsp), %ymm3
	vmovaps	7296(%rsp), %ymm2
	.p2align	4
.LBB34_498:
	vmovaps	%ymm12, %ymm10
	vmovaps	%ymm15, %ymm9
	vmovaps	736(%rsp), %ymm15
	vmovaps	%ymm5, 32(%rsp)
	vmovaps	192(%rsp), %ymm12
	movq	416(%rsp), %rdi
.Ltmp8469:
	.loc	5 568 12 is_stmt 1
	addq	%rcx, %rdi
	cmpq	%r13, %rdi
	ja	.LBB34_97
.Ltmp8470:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_490
.Ltmp8471:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm6, %ymm11
	vmovaps	%ymm14, %ymm8
	vmovaps	%ymm1, %ymm7
	vmovaps	%ymm13, %ymm14
	movq	2432(%rsp), %rdi
.Ltmp8472:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rdi,%rcx,4), %ymm13
.Ltmp8473:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm1
	vmovaps	%ymm1, 64(%rsp)
.Ltmp8474:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm13, %ymm1
	vmovaps	%ymm2, %ymm5
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp8475:
	.loc	29 48 14
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp8476:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm13, %ymm4
.Ltmp8477:
	.loc	29 48 14
	vaddps	%ymm2, %ymm4, %ymm4
.Ltmp8478:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm13, %ymm6
.Ltmp8479:
	.loc	29 48 14
	vaddps	%ymm2, %ymm6, %ymm6
	vmovaps	%ymm3, %ymm0
	vmovaps	%ymm14, %ymm3
	vmovaps	%ymm13, 192(%rsp)
.Ltmp8480:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm13, %ymm14
.Ltmp8481:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm14
	vmovaps	%ymm15, %ymm2
.Ltmp8482:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm12, %ymm15
.Ltmp8483:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8484:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm12, %ymm15
.Ltmp8485:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8486:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm12, %ymm15
.Ltmp8487:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8488:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm12, %ymm15
.Ltmp8489:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8490:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm10, %ymm15
.Ltmp8491:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8492:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm10, %ymm15
.Ltmp8493:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8494:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm10, %ymm15
.Ltmp8495:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8496:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm10, %ymm15
.Ltmp8497:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8498:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm11, %ymm15
.Ltmp8499:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8500:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm11, %ymm15
.Ltmp8501:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8502:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm11, %ymm15
.Ltmp8503:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8504:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm11, %ymm15
.Ltmp8505:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8506:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm8, %ymm15
.Ltmp8507:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8508:
	.loc	29 283 14
	vmulps	1632(%rsp), %ymm8, %ymm15
.Ltmp8509:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8510:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm8, %ymm15
.Ltmp8511:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8512:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm8, %ymm15
.Ltmp8513:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8514:
	.loc	29 283 14
	vmulps	1600(%rsp), %ymm9, %ymm15
.Ltmp8515:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8516:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm9, %ymm15
.Ltmp8517:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8518:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm9, %ymm15
.Ltmp8519:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8520:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm9, %ymm15
.Ltmp8521:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8522:
	.loc	29 283 14
	vmulps	1568(%rsp), %ymm7, %ymm15
.Ltmp8523:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8524:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm7, %ymm15
.Ltmp8525:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8526:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm7, %ymm15
.Ltmp8527:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8528:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm7, %ymm15
.Ltmp8529:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
	vmovaps	32(%rsp), %ymm13
.Ltmp8530:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm13, %ymm15
.Ltmp8531:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8532:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm13, %ymm15
.Ltmp8533:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8534:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm13, %ymm15
.Ltmp8535:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8536:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm13, %ymm15
.Ltmp8537:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8538:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm3, %ymm15
.Ltmp8539:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8540:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm3, %ymm15
.Ltmp8541:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8542:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm3, %ymm15
.Ltmp8543:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8544:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm3, %ymm15
.Ltmp8545:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8546:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm0, %ymm15
.Ltmp8547:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8548:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm0, %ymm15
.Ltmp8549:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8550:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm0, %ymm15
.Ltmp8551:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	%ymm0, 736(%rsp)
.Ltmp8552:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm0, %ymm15
.Ltmp8553:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8554:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm2, %ymm15
.Ltmp8555:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8556:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm2, %ymm15
.Ltmp8557:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8558:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm2, %ymm15
.Ltmp8559:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8560:
	.loc	29 283 14
	vmulps	864(%rsp), %ymm2, %ymm15
.Ltmp8561:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp8562:
	.loc	29 283 14
	vmulps	4256(%rsp), %ymm5, %ymm15
.Ltmp8563:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp8564:
	.loc	29 283 14
	vmulps	832(%rsp), %ymm5, %ymm15
.Ltmp8565:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp8566:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm5, %ymm15
.Ltmp8567:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	%ymm5, 480(%rsp)
.Ltmp8568:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm5, %ymm15
.Ltmp8569:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
	vmovaps	64(%rsp), %ymm0
.Ltmp8570:
	.loc	29 82 19
	vandps	%ymm0, %ymm7, %ymm15
.Ltmp8571:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm0, %ymm1, %ymm1
.Ltmp8572:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm1, %ymm15, %ymm1
.Ltmp8573:
	.loc	29 82 19
	vandps	%ymm0, %ymm4, %ymm4
.Ltmp8574:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm1, %ymm1
.Ltmp8575:
	.loc	29 82 19
	vandps	%ymm0, %ymm6, %ymm4
.Ltmp8576:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm1, %ymm1
.Ltmp8577:
	.loc	29 82 19
	vandps	%ymm0, %ymm14, %ymm4
.Ltmp8578:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm1, %ymm1
.Ltmp8579:
	.loc	11 551 14
	vmovups	%ymm1, 9248(%rsp,%rcx,4)
.Ltmp8580:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm7, %ymm5
	vmovaps	%ymm9, %ymm1
	vmovaps	%ymm8, %ymm15
	vmovaps	%ymm11, %ymm14
	vmovaps	%ymm10, %ymm6
	cmpq	%rcx, %rax
.Ltmp8581:
	.loc	3 900 12
	jne	.LBB34_498
.Ltmp8582:
.LBB34_501:
	.loc	3 0 12 is_stmt 0
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1120(%rsp)
	vmovaps	%ymm10, 800(%rsp)
	vmovaps	%ymm12, 1024(%rsp)
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm11, 6656(%rsp)
	vmovaps	%ymm8, 6688(%rsp)
	vmovaps	%ymm9, 6720(%rsp)
	vmovaps	%ymm7, 6752(%rsp)
	vmovaps	%ymm13, 6784(%rsp)
	vmovaps	%ymm3, 7328(%rsp)
	vmovaps	%ymm3, 6816(%rsp)
	vmovaps	736(%rsp), %ymm0
	vmovaps	%ymm0, 6848(%rsp)
	vmovaps	%ymm2, 7296(%rsp)
	vmovaps	%ymm2, 6880(%rsp)
	vmovaps	480(%rsp), %ymm0
	vmovaps	%ymm0, 6912(%rsp)
	movq	16(%rsp), %rcx
	movq	2328(%rsp), %rdi
.Ltmp8583:
	.loc	10 1916 50
	cmpq	%rdi, %rcx
	vmovaps	%ymm11, %ymm5
	vmovaps	%ymm13, %ymm14
.Ltmp8584:
	.loc	1 1991 19
	jne	.LBB34_503
.Ltmp8585:
	.loc	1 0 19 is_stmt 0
	vmovaps	7488(%rsp), %ymm2
	vmovaps	7456(%rsp), %ymm13
	vmovaps	800(%rsp), %ymm6
.LBB34_484:
	addq	$32, %rdi
	decq	%r9
	movq	%rdi, %r8
	movq	4304(%rsp), %rax
.Ltmp8586:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rax
	addq	$256, 416(%rsp)
	movq	4312(%rsp), %rdx
	addq	$-256, %rdx
	addq	$1024, 1728(%rsp)
	addq	$-256, 2768(%rsp)
	addq	$1024, 2432(%rsp)
	testq	%r9, %r9
	vmovaps	1216(%rsp), %ymm11
	vmovaps	160(%rsp), %ymm3
	vmovaps	7616(%rsp), %ymm4
	vmovaps	1120(%rsp), %ymm10
	vmovaps	736(%rsp), %ymm0
	jne	.LBB34_485
	jmp	.LBB34_544
.Ltmp8587:
.LBB34_503:
	.loc	8 0 20 is_stmt 0
	movq	%r9, 1760(%rsp)
.Ltmp8588:
	.loc	1 1997 21 is_stmt 1
	movq	984(%rsp), %rdi
	movq	992(%rsp), %rax
	movq	%rax, 1376(%rsp)
	.loc	1 1998 21
	movq	632(%rsp), %r13
	xorl	%ebx, %ebx
	movq	640(%rsp), %rax
	movq	%rax, 1824(%rsp)
	vmovaps	6208(%rsp), %ymm15
	vmovaps	6336(%rsp), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	6528(%rsp), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	6944(%rsp), %ymm0
	vmovaps	7072(%rsp), %ymm1
	vmovaps	%ymm1, 1920(%rsp)
	vmovaps	7264(%rsp), %ymm1
	vmovaps	%ymm1, 1536(%rsp)
	movq	928(%rsp), %r15
	movq	936(%rsp), %rdx
	movq	976(%rsp), %rax
	movq	%rax, 2176(%rsp)
	movq	944(%rsp), %rax
	movq	%rax, 1600(%rsp)
	movq	952(%rsp), %rax
	movq	%rax, 1664(%rsp)
	movq	968(%rsp), %rax
	movq	%rax, 1632(%rsp)
	movq	960(%rsp), %rax
	movq	%rax, 2016(%rsp)
	movq	576(%rsp), %r9
	movq	584(%rsp), %rax
	vmovaps	1152(%rsp), %ymm4
	vmovaps	1184(%rsp), %ymm11
	movq	624(%rsp), %rcx
	movq	%rcx, 1984(%rsp)
	movq	592(%rsp), %rcx
	movq	%rcx, 1888(%rsp)
	movq	600(%rsp), %rcx
	movq	%rcx, 1952(%rsp)
	movq	616(%rsp), %rcx
	movq	%rcx, 1568(%rsp)
	movq	608(%rsp), %rcx
	movq	%rcx, 1856(%rsp)
	movl	4(%rsp), %ecx
	movq	%rcx, 192(%rsp)
	movl	%r10d, 672(%rsp)
	movl	%r10d, %ecx
	movq	%rcx, 64(%rsp)
	xorl	%r8d, %r8d
	movq	2592(%rsp), %rsi
	vmovaps	7488(%rsp), %ymm2
	vbroadcastss	.LCPI34_2(%rip), %ymm3
	movq	2776(%rsp), %r10
	vmovaps	7456(%rsp), %ymm13
	vmovaps	800(%rsp), %ymm6
	movq	%rdi, 1792(%rsp)
	movq	%r13, 1696(%rsp)
	vmovaps	%ymm14, 32(%rsp)
	vmovaps	%ymm5, 2624(%rsp)
.LBB34_504:
	.loc	1 0 21 is_stmt 0
	movq	%r14, 768(%rsp)
	movq	%r8, 288(%rsp)
	.loc	1 1996 21 is_stmt 1
	subq	%r8, %r10
	movq	%r11, %rcx
.Ltmp8589:
	.loc	1 1577 16
	movq	1624(%r11), %r11
.Ltmp8590:
	.loc	1 1578 16
	movq	1632(%rcx), %rcx
	movq	%rcx, 128(%rsp)
.Ltmp8591:
	.loc	1 1579 25
	leaq	1(%rsi), %rcx
.Ltmp8592:
	.loc	1 1150 8
	cmpq	%r11, %rcx
	movq	%r11, %rcx
	cmovbq	%rbx, %rcx
	negq	%rcx
	addq	%rsi, %rcx
	incq	%rcx
	movq	%rcx, 2272(%rsp)
.Ltmp8593:
	.loc	1 1580 28
	leaq	(%rdi,%rsi), %r14
.Ltmp8594:
	.loc	1 1150 8
	cmpq	%r11, %r14
	movq	%r11, %rcx
	cmovbq	%rbx, %rcx
	subq	%rcx, %r14
.Ltmp8595:
	.loc	1 1581 29
	leaq	(%rsi,%r13), %r8
.Ltmp8596:
	.loc	1 1150 8
	cmpq	%r11, %r8
	movq	%r11, %rcx
	cmovbq	%rbx, %rcx
	subq	%rcx, %r8
	movq	1376(%rsp), %rcx
.Ltmp8597:
	.loc	1 1582 33
	leaq	(%rcx,%rsi), %rdi
.Ltmp8598:
	.loc	1 1150 8
	cmpq	%r11, %rdi
	movq	%r11, %rcx
	cmovbq	%rbx, %rcx
	subq	%rcx, %rdi
	movq	%rdi, 1312(%rsp)
	movq	1824(%rsp), %rcx
.Ltmp8599:
	.loc	1 1583 34
	leaq	(%rcx,%rsi), %rdi
.Ltmp8600:
	.loc	1 1150 8
	cmpq	%r11, %rdi
	movq	%r11, %rcx
	cmovbq	%rbx, %rcx
	subq	%rcx, %rdi
.Ltmp8601:
	.loc	1 1585 14
	movq	%r11, %rbx
	movq	%rsi, 2592(%rsp)
	subq	%rsi, %rbx
.Ltmp8602:
	.loc	10 1078 5
	cmpq	%r10, %rbx
	cmovbq	%rbx, %r10
	movq	768(%rsp), %rsi
	movq	128(%rsp), %rcx
.Ltmp8603:
	.loc	1 1586 14
	subq	%rsi, %rcx
.Ltmp8604:
	.loc	10 1078 5
	cmpq	%r10, %rcx
	movq	%rcx, 128(%rsp)
	cmovbq	%rcx, %r10
.Ltmp8605:
	.loc	1 1587 14
	movq	%r11, %r13
	movq	2272(%rsp), %rcx
	subq	%rcx, %r13
.Ltmp8606:
	.loc	10 1078 5
	cmpq	%r10, %r13
	cmovbq	%r13, %r10
.Ltmp8607:
	.loc	1 1588 14
	movq	%r11, %rcx
	movq	%r14, 1472(%rsp)
	subq	%r14, %rcx
.Ltmp8608:
	.loc	10 1078 5
	cmpq	%r10, %rcx
	cmovbq	%rcx, %r10
.Ltmp8609:
	.loc	1 1589 14
	movq	%r11, %r14
	movq	%r8, 1440(%rsp)
	subq	%r8, %r14
.Ltmp8610:
	.loc	10 1078 5
	cmpq	%r10, %r14
	cmovbq	%r14, %r10
.Ltmp8611:
	.loc	1 1590 14
	movq	%r11, %r8
	movq	1312(%rsp), %rsi
	subq	%rsi, %r8
.Ltmp8612:
	.loc	10 1078 5
	cmpq	%r10, %r8
	cmovbq	%r8, %r10
	movq	%rdi, 1408(%rsp)
.Ltmp8613:
	.loc	1 1591 14
	subq	%rdi, %r11
.Ltmp8614:
	.loc	10 1078 5
	cmpq	%r10, %r11
	cmovbq	%r11, %r10
	movq	2328(%rsp), %rsi
	movq	288(%rsp), %rdi
.Ltmp8615:
	.loc	1 2002 28
	addq	%rsi, %rdi
	movq	%r10, 1504(%rsp)
.Ltmp8616:
	.loc	1 2004 55
	leaq	(%r10,%rdi), %rsi
.Ltmp8617:
	.loc	1 2002 28
	shlq	$3, %rdi
.Ltmp8618:
	.loc	1 2004 55
	shlq	$3, %rsi
.Ltmp8619:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB34_506
	cmpq	24(%rsp), %rsi
	ja	.LBB34_506
.Ltmp8620:
	.loc	5 451 16
	cmpq	248(%rsp), %rsi
	ja	.LBB34_508
.Ltmp8621:
	.loc	5 0 16 is_stmt 0
	movq	288(%rsp), %rsi
.Ltmp8622:
	.loc	1 2006 46 is_stmt 1
	leaq	(,%rsi,8), %r10
	movq	%r10, 1280(%rsp)
	movq	1504(%rsp), %r10
	.loc	1 2006 61 is_stmt 0
	addq	%r10, %rsi
	movq	%rsi, 1248(%rsp)
	cmpq	$33, %rsi
.Ltmp8623:
	.loc	4 1050 16 is_stmt 1
	jae	.LBB34_641
.Ltmp8624:
	.loc	14 304 12
	testq	%r10, %r10
	je	.LBB34_511
.Ltmp8625:
	.loc	14 0 12 is_stmt 0
	cmpq	%r14, %rcx
	cmovbq	%rcx, %r14
	cmpq	%r8, %r14
	cmovaeq	%r8, %r14
	cmpq	%r11, %r14
	cmovaeq	%r11, %r14
	cmpq	%r13, %r14
	cmovaeq	%r13, %r14
	cmpq	%rbx, %r14
	cmovaeq	%rbx, %r14
	movq	344(%rsp), %rcx
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 2208(%rsp)
	movq	128(%rsp), %rcx
	cmpq	%rcx, %r14
	cmovaeq	%rcx, %r14
	movq	408(%rsp), %rcx
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 2144(%rsp)
	movq	4328(%rsp), %rcx
	subq	288(%rsp), %rcx
	movq	1280(%rsp), %rdi
	leaq	(%rsp,%rdi,4), %rsi
	addq	$10280, %rsi
	movq	%rsi, 2240(%rsp)
	leaq	(%rsp,%rdi,4), %rsi
	addq	$9248, %rsi
	movq	%rsi, 1280(%rsp)
	cmpq	%rcx, %r14
	cmovbq	%r14, %rcx
	andq	4296(%rsp), %rcx
	movq	%rcx, 2112(%rsp)
	xorl	%r14d, %r14d
	vmovaps	%ymm4, %ymm1
	vmovaps	%ymm11, %ymm10
	.p2align	4
.LBB34_513:
	movq	2592(%rsp), %rcx
.Ltmp8626:
	.loc	1 1504 26 is_stmt 1
	addq	%r14, %rcx
.Ltmp8627:
	.loc	1 1139 16
	leaq	(,%rcx,8), %r8
.Ltmp8628:
	.loc	1 1140 33
	leaq	8(,%rcx,8), %r11
.Ltmp8629:
	.loc	4 1050 16
	leaq	7(,%rcx,8), %rcx
	cmpq	%rdx, %rcx
	jae	.LBB34_642
.Ltmp8630:
	.loc	1 0 0 is_stmt 0
	movq	%r14, %rbx
	shlq	$5, %rbx
	movq	2240(%rsp), %rcx
	vmovups	(%rcx,%rbx), %ymm4
	movq	1280(%rsp), %rcx
	vmovups	(%rcx,%rbx), %ymm11
.Ltmp8631:
	vmaxps	%ymm4, %ymm11, %ymm12
	vmovaps	96(%rsp), %ymm5
.Ltmp8632:
	.loc	29 585 19 is_stmt 1
	vblendvps	%ymm5, %ymm12, %ymm4, %ymm4
.Ltmp8633:
	.loc	29 360 14
	vdivps	%ymm4, %ymm15, %ymm5
	movq	1472(%rsp), %rcx
.Ltmp8634:
	.loc	1 0 0 is_stmt 0
	leaq	(%r14,%rcx), %rsi
.Ltmp8635:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm15, %ymm4, %ymm4
.Ltmp8636:
	.loc	29 585 19
	vblendvps	%ymm4, %ymm5, %ymm3, %ymm4
.Ltmp8637:
	.loc	11 551 14
	vmovups	%ymm4, (%r15,%r8,4)
.Ltmp8638:
	.loc	1 1132 16
	leaq	(,%rsi,8), %rdi
.Ltmp8639:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %rcx
	cmpq	%rdx, %rcx
	jae	.LBB34_515
.Ltmp8640:
	.loc	11 551 14
	vmovups	(%r15,%rdi,4), %ymm5
	vmovaps	%ymm5, %ymm14
	movq	64(%rsp), %r13
.Ltmp8641:
	.loc	1 1208 22
	testl	%r13d, %r13d
	je	.LBB34_518
.Ltmp8642:
	.loc	29 257 14
	vminps	%ymm5, %ymm1, %ymm14
.Ltmp8643:
.LBB34_518:
	.loc	29 0 14 is_stmt 0
	movq	2272(%rsp), %rcx
	leaq	(%r14,%rcx), %r10
.Ltmp8644:
	movl	%r13d, %r13d
.Ltmp8645:
	.loc	1 1214 20 is_stmt 1
	incq	%r13
	movq	2176(%rsp), %rdi
	movq	%rdi, %rcx
	cmpq	%rdi, %r13
.Ltmp8646:
	.loc	1 1215 22
	jne	.LBB34_519
	.loc	1 0 22 is_stmt 0
.Ltmp8647:
	.p2align	4
.LBB34_521:
.Ltmp8648:
	.loc	1 1132 16 is_stmt 1
	leaq	(,%rsi,8), %rdi
.Ltmp8649:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %r13
	cmpq	%rdx, %r13
	jae	.LBB34_515
.Ltmp8650:
	.loc	29 257 14
	vminps	(%r15,%rdi,4), %ymm5, %ymm5
.Ltmp8651:
	.loc	11 551 14
	vmovups	%ymm5, (%r15,%rdi,4)
.Ltmp8652:
	.loc	1 1226 16
	testq	%rsi, %rsi
	cmoveq	%r12, %rsi
	.loc	1 1229 13
	decq	%rsi
.Ltmp8653:
	.loc	10 1916 50
	decq	%rcx
.Ltmp8654:
	.loc	3 900 12
	jne	.LBB34_521
.Ltmp8655:
	.loc	3 0 12 is_stmt 0
	xorl	%r13d, %r13d
	vmovaps	%ymm14, %ymm1
	jmp	.LBB34_524
	.p2align	4
.LBB34_519:
.Ltmp8656:
	.loc	1 1132 16 is_stmt 1
	leaq	(,%r10,8), %rdi
.Ltmp8657:
	.loc	4 1050 16
	leaq	7(,%r10,8), %rcx
	cmpq	%rdx, %rcx
	jae	.LBB34_515
.Ltmp8658:
	.loc	11 551 14
	vmovups	(%r15,%rdi,4), %ymm1
.Ltmp8659:
	.loc	29 257 14
	vminps	%ymm14, %ymm1, %ymm1
.Ltmp8660:
.LBB34_524:
	.loc	29 0 14 is_stmt 0
	movq	1312(%rsp), %rcx
	leaq	(%r14,%rcx), %rdi
.Ltmp8661:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%rdi,8), %rcx
.Ltmp8662:
	.loc	1 1132 16
	shlq	$3, %rdi
	movq	1664(%rsp), %rsi
.Ltmp8663:
	.loc	4 1050 16
	cmpq	%rsi, %rcx
	movq	%rsi, %rcx
	jae	.LBB34_643
.Ltmp8664:
	.loc	4 0 16 is_stmt 0
	cmpq	%rcx, %r11
.Ltmp8665:
	.loc	4 1050 16
	ja	.LBB34_644
.Ltmp8666:
	.loc	4 0 16
	movq	768(%rsp), %rcx
	addq	%r14, %rcx
.Ltmp8667:
	vbroadcastss	.LCPI34_3(%rip), %ymm4
	vmulps	%ymm4, %ymm1, %ymm1
	vroundps	$9, %ymm1, %ymm1
	vbroadcastss	.LCPI34_4(%rip), %ymm4
	vmulps	%ymm4, %ymm1, %ymm1
.Ltmp8668:
	vaddps	%ymm2, %ymm1, %ymm4
	movq	1600(%rsp), %rsi
	vsubps	(%rsi,%rdi,4), %ymm4, %ymm2
.Ltmp8669:
	.loc	11 551 14 is_stmt 1
	vmovups	%ymm1, (%rsi,%r8,4)
.Ltmp8670:
	.loc	29 360 14
	vdivps	2048(%rsp), %ymm2, %ymm1
.Ltmp8671:
	.loc	1 1661 43
	vmovaps	6464(%rsp), %ymm4
.Ltmp8672:
	.loc	29 347 14
	vsubps	%ymm1, %ymm3, %ymm1
.Ltmp8673:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm4, %ymm1, %ymm5
.Ltmp8674:
	.loc	29 283 14 is_stmt 1
	vmulps	2080(%rsp), %ymm5, %ymm5
.Ltmp8675:
	.loc	29 48 14
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8676:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm1, %ymm1
.Ltmp8677:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm5
	vandps	%ymm5, %ymm1, %ymm4
.Ltmp8678:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm6
	vcmplt_oqps	%ymm6, %ymm4, %ymm4
.Ltmp8679:
	.loc	29 82 19
	vandnps	%ymm1, %ymm4, %ymm1
.Ltmp8680:
	.loc	1 1662 5
	vmovaps	%ymm1, 6464(%rsp)
.Ltmp8681:
	.loc	1 1132 16
	leaq	(,%rcx,8), %rdi
.Ltmp8682:
	.loc	1 1133 25
	leaq	8(,%rcx,8), %rsi
.Ltmp8683:
	.loc	4 1050 16
	leaq	7(,%rcx,8), %rcx
	cmpq	1632(%rsp), %rcx
	jae	.LBB34_645
.Ltmp8684:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, 128(%rsp)
	movq	2208(%rsp), %rcx
	addq	%rbx, %rcx
.Ltmp8685:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm1, %ymm3, %ymm1
	movq	%r11, %rsi
	movq	2016(%rsp), %r11
.Ltmp8686:
	.loc	11 551 14
	vmovups	(%r11,%rdi,4), %ymm4
.Ltmp8687:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rcx), %ymm6
	vmovups	%ymm6, (%r11,%rdi,4)
.Ltmp8688:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm4, %ymm1, %ymm1
	vmovaps	704(%rsp), %ymm6
.Ltmp8689:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm4, %ymm1, %ymm1
.Ltmp8690:
	.loc	11 551 14
	vmovups	%ymm1, (%rcx)
	cmpq	%rax, %rsi
.Ltmp8691:
	.loc	4 1050 16
	ja	.LBB34_646
.Ltmp8692:
	.loc	4 0 16 is_stmt 0
	movq	%r13, 64(%rsp)
	vmovaps	96(%rsp), %ymm1
	vblendvps	%ymm1, %ymm12, %ymm11, %ymm1
.Ltmp8693:
	.loc	29 360 14 is_stmt 1
	vdivps	%ymm1, %ymm0, %ymm4
	movq	1440(%rsp), %rcx
.Ltmp8694:
	.loc	1 0 0 is_stmt 0
	addq	%r14, %rcx
.Ltmp8695:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm0, %ymm1, %ymm1
.Ltmp8696:
	.loc	29 585 19
	vblendvps	%ymm1, %ymm4, %ymm3, %ymm1
.Ltmp8697:
	.loc	11 551 14
	vmovups	%ymm1, (%r9,%r8,4)
.Ltmp8698:
	.loc	1 1132 16
	leaq	(,%rcx,8), %r13
.Ltmp8699:
	.loc	4 1050 16
	leaq	7(,%rcx,8), %r11
	cmpq	%rax, %r11
	jae	.LBB34_647
.Ltmp8700:
	.loc	4 0 16 is_stmt 0
	movq	%r8, 288(%rsp)
	movq	%r12, %r8
.Ltmp8701:
	.loc	11 551 14 is_stmt 1
	vmovups	(%r9,%r13,4), %ymm1
	vmovaps	%ymm1, %ymm11
.Ltmp8702:
	.loc	1 1208 22
	cmpl	$0, 192(%rsp)
	je	.LBB34_531
.Ltmp8703:
	.loc	29 257 14
	vminps	%ymm1, %ymm10, %ymm11
.Ltmp8704:
.LBB34_531:
	.loc	29 0 14 is_stmt 0
	movq	192(%rsp), %r12
	movl	%r12d, %r12d
.Ltmp8705:
	.loc	1 1214 20 is_stmt 1
	incq	%r12
	movq	1984(%rsp), %r11
	movq	%r11, %r13
	movq	%r12, 192(%rsp)
	cmpq	%r11, %r12
	movq	%r8, %r12
	movq	1504(%rsp), %r8
.Ltmp8706:
	.loc	1 1215 22
	jne	.LBB34_532
	.loc	1 0 22 is_stmt 0
.Ltmp8707:
	.p2align	4
.LBB34_534:
.Ltmp8708:
	.loc	1 1132 16 is_stmt 1
	leaq	(,%rcx,8), %r10
.Ltmp8709:
	.loc	4 1050 16
	leaq	7(,%rcx,8), %r11
	cmpq	%rax, %r11
	jae	.LBB34_648
.Ltmp8710:
	.loc	29 257 14
	vminps	(%r9,%r10,4), %ymm1, %ymm1
.Ltmp8711:
	.loc	11 551 14
	vmovups	%ymm1, (%r9,%r10,4)
.Ltmp8712:
	.loc	1 1226 16
	testq	%rcx, %rcx
	cmoveq	%r12, %rcx
	.loc	1 1229 13
	decq	%rcx
.Ltmp8713:
	.loc	10 1916 50
	decq	%r13
.Ltmp8714:
	.loc	3 900 12
	jne	.LBB34_534
.Ltmp8715:
	.loc	3 0 12 is_stmt 0
	movq	$0, 192(%rsp)
	vmovaps	%ymm11, %ymm1
	jmp	.LBB34_537
	.p2align	4
.LBB34_532:
.Ltmp8716:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r10,8), %rcx
.Ltmp8717:
	.loc	1 1132 16
	shlq	$3, %r10
.Ltmp8718:
	.loc	4 1050 16
	cmpq	%rax, %rcx
	jae	.LBB34_648
.Ltmp8719:
	.loc	11 551 14
	vmovups	(%r9,%r10,4), %ymm1
.Ltmp8720:
	.loc	29 257 14
	vminps	%ymm11, %ymm1, %ymm1
.Ltmp8721:
.LBB34_537:
	.loc	29 0 14 is_stmt 0
	movq	1408(%rsp), %rcx
	leaq	(%r14,%rcx), %r10
.Ltmp8722:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r10,8), %rcx
.Ltmp8723:
	.loc	1 1132 16
	shlq	$3, %r10
	movq	1952(%rsp), %r11
.Ltmp8724:
	.loc	4 1050 16
	cmpq	%r11, %rcx
	movq	%r11, %rcx
	movq	8(%rsp), %r11
	movq	1696(%rsp), %r13
	jae	.LBB34_538
.Ltmp8725:
	.loc	4 0 16 is_stmt 0
	cmpq	%rcx, %rsi
.Ltmp8726:
	.loc	4 1050 16
	ja	.LBB34_649
.Ltmp8727:
	.loc	1 0 0
	vbroadcastss	.LCPI34_3(%rip), %ymm4
	vmulps	%ymm4, %ymm1, %ymm1
	vroundps	$9, %ymm1, %ymm1
	vbroadcastss	.LCPI34_4(%rip), %ymm4
	vmulps	%ymm4, %ymm1, %ymm1
.Ltmp8728:
	vaddps	1344(%rsp), %ymm1, %ymm4
	movq	1888(%rsp), %rcx
	vsubps	(%rcx,%r10,4), %ymm4, %ymm4
	vmovaps	%ymm4, 1344(%rsp)
.Ltmp8729:
	.loc	29 360 14 is_stmt 1
	vdivps	1536(%rsp), %ymm4, %ymm4
	movq	288(%rsp), %r10
.Ltmp8730:
	.loc	11 551 14
	vmovups	%ymm1, (%rcx,%r10,4)
.Ltmp8731:
	.loc	1 1661 43
	vmovaps	7200(%rsp), %ymm1
.Ltmp8732:
	.loc	29 347 14
	vsubps	%ymm4, %ymm3, %ymm4
.Ltmp8733:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm1, %ymm4, %ymm6
.Ltmp8734:
	.loc	29 283 14 is_stmt 1
	vmulps	1920(%rsp), %ymm6, %ymm6
.Ltmp8735:
	.loc	29 48 14
	vaddps	%ymm6, %ymm1, %ymm1
.Ltmp8736:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm4, %ymm1
.Ltmp8737:
	.loc	29 82 19
	vandps	%ymm5, %ymm1, %ymm4
.Ltmp8738:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm5
	vcmplt_oqps	%ymm5, %ymm4, %ymm4
.Ltmp8739:
	.loc	29 82 19
	vandnps	%ymm1, %ymm4, %ymm1
.Ltmp8740:
	.loc	1 1662 5
	vmovaps	%ymm1, 7200(%rsp)
	movq	128(%rsp), %rsi
	cmpq	1568(%rsp), %rsi
.Ltmp8741:
	.loc	4 1050 16
	ja	.LBB34_650
.Ltmp8742:
	.loc	1 0 0 is_stmt 0
	addq	2144(%rsp), %rbx
	incq	%r14
.Ltmp8743:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm1, %ymm3, %ymm1
	movq	1856(%rsp), %rcx
.Ltmp8744:
	.loc	11 551 14
	vmovups	(%rcx,%rdi,4), %ymm4
.Ltmp8745:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rbx), %ymm5
	vmovups	%ymm5, (%rcx,%rdi,4)
.Ltmp8746:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm4, %ymm1, %ymm1
	vmovaps	704(%rsp), %ymm5
.Ltmp8747:
	.loc	29 585 19
	vblendvps	%ymm5, %ymm4, %ymm1, %ymm1
.Ltmp8748:
	.loc	11 551 14
	vmovups	%ymm1, (%rbx)
	vmovaps	%ymm14, %ymm4
	vmovaps	%ymm14, %ymm1
	vmovaps	%ymm11, %ymm10
.Ltmp8749:
	.loc	14 304 12
	cmpq	2112(%rsp), %r14
	jne	.LBB34_513
	jmp	.LBB34_543
.Ltmp8750:
.LBB34_511:
	.loc	14 0 12 is_stmt 0
	movq	8(%rsp), %r11
	movq	1696(%rsp), %r13
	movq	1504(%rsp), %r8
.LBB34_543:
	movq	2592(%rsp), %rsi
	.loc	1 2063 39 is_stmt 1
	addq	%r8, %rsi
.Ltmp8751:
	.loc	1 1150 8
	cmpq	%r12, %rsi
	movq	%r12, %rcx
	movl	$0, %ebx
	cmovbq	%rbx, %rcx
	subq	%rcx, %rsi
	movq	768(%rsp), %r14
.Ltmp8752:
	.loc	1 2064 39
	addq	%r8, %r14
	movq	4320(%rsp), %rcx
.Ltmp8753:
	.loc	1 1150 8
	cmpq	%rcx, %r14
	cmovbq	%rbx, %rcx
	subq	%rcx, %r14
	movq	1248(%rsp), %rdi
	movq	%rdi, %r8
	movq	2776(%rsp), %r10
.Ltmp8754:
	.loc	1 1991 19
	cmpq	%r10, %rdi
	vmovaps	2624(%rsp), %ymm5
	vmovaps	32(%rsp), %ymm14
	vmovaps	800(%rsp), %ymm6
	movq	1792(%rsp), %rdi
	jb	.LBB34_504
.Ltmp8755:
	.loc	1 0 19 is_stmt 0
	movq	%rsi, 2592(%rsp)
	vmovaps	%ymm2, 6496(%rsp)
	vmovaps	1344(%rsp), %ymm1
	vmovaps	%ymm1, 7232(%rsp)
	movq	192(%rsp), %rax
	movl	%eax, 4(%rsp)
	movq	64(%rsp), %rax
	movl	%eax, %r10d
	vmovaps	%ymm11, 1184(%rsp)
	vmovaps	%ymm4, 1152(%rsp)
	movq	248(%rsp), %r13
	movq	24(%rsp), %rsi
	movq	16(%rsp), %rcx
	movq	1760(%rsp), %r9
	movl	$32, %r15d
	movq	2328(%rsp), %rdi
	jmp	.LBB34_484
.LBB34_398:
	leaq	4352(%rsp), %rdi
	movq	536(%rsp), %r14
.Ltmp8756:
	.loc	1 1948 24 is_stmt 1
	movq	%r14, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	5088(%rsp), %rdi
	movq	528(%rsp), %rsi
.Ltmp8757:
	.loc	1 1949 25
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	movq	8(%rsp), %r13
.Ltmp8758:
	.loc	1 1954 19
	movzbl	1536(%r13), %eax
	movb	%al, 160(%rsp)
.Ltmp8759:
	.loc	1 1955 21
	movzbl	1537(%r13), %eax
	movb	%al, 32(%rsp)
.Ltmp8760:
	.loc	1 1956 16
	movq	1624(%r13), %r12
.Ltmp8761:
	.loc	1 1957 16
	movq	1632(%r13), %r15
.Ltmp8762:
	.loc	1 1958 27
	movl	1640(%r13), %eax
	movq	%rax, 1728(%rsp)
.Ltmp8763:
	.loc	1 1959 27
	movl	1644(%r13), %eax
	movq	%rax, 2336(%rsp)
	leaq	10280(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %rbx
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	leaq	9248(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%rbx
	leaq	896(%rsp), %rdi
.Ltmp8764:
	.loc	1 1966 32
	movq	%r14, %rsi
	movq	%r12, 192(%rsp)
	movq	%r12, %rdx
	movq	%r15, 832(%rsp)
	movq	%r15, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
.Ltmp8765:
	.loc	1 1967 33
	movq	1624(%r13), %rdx
	movq	1632(%r13), %rcx
	leaq	544(%rsp), %rdi
	movq	528(%rsp), %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	16(%rsp), %r10
.Ltmp8766:
	.loc	4 3758 16
	leaq	31(%r10), %rax
	shrq	$5, %rax
.Ltmp8767:
	.loc	8 446 20
	je	.LBB34_399
.Ltmp8768:
	.loc	8 0 20 is_stmt 0
	movq	%rax, 2688(%rsp)
.Ltmp8769:
	.loc	29 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp8770:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 288(%rsp)
	vmovaps	%ymm1, 1344(%rsp)
	cmpb	$0, 160(%rsp)
	movq	248(%rsp), %r13
	movq	24(%rsp), %rsi
	movq	344(%rsp), %rax
	jne	.LBB34_411
.Ltmp8771:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
.LBB34_411:
	cmpb	$0, 32(%rsp)
	movq	8(%rsp), %r9
	movq	1728(%rsp), %rbx
	jne	.LBB34_413
	vmovaps	%ymm0, 288(%rsp)
.LBB34_413:
	movl	$32, %r15d
	movabsq	$2305843009213693944, %rdx
	addq	$7, %rdx
	movq	%rdx, 800(%rsp)
	movq	408(%rsp), %r11
	movq	%r13, %r8
	movq	%rax, 480(%rsp)
	movq	%rsi, %rdx
	movq	$0, 736(%rsp)
	movq	%r10, %rcx
	xorl	%edi, %edi
.LBB34_416:
.Ltmp8772:
	.loc	4 2584 13 is_stmt 1
	cmpq	$32, %rcx
	movl	$32, %eax
	movq	%rcx, 2592(%rsp)
	cmovbq	%rcx, %rax
	cmpq	$1, %rax
	movq	%rax, 4256(%rsp)
	adcq	$0, %rax
	shll	$3, %eax
.Ltmp8773:
	.loc	10 1916 50
	movq	%r10, %rcx
	subq	%rdi, %rcx
.Ltmp8774:
	.loc	10 1078 5
	cmpq	$32, %rcx
	cmovaeq	%r15, %rcx
	movq	%rcx, 864(%rsp)
.Ltmp8775:
	.loc	1 1761 23
	vmovaps	4352(%rsp), %ymm4
	vmovaps	4384(%rsp), %ymm5
	vmovaps	4416(%rsp), %ymm0
	vmovaps	4448(%rsp), %ymm1
	vmovaps	4480(%rsp), %ymm2
	vmovaps	4512(%rsp), %ymm10
	vmovaps	4544(%rsp), %ymm3
	vmovaps	4576(%rsp), %ymm11
	vmovaps	4608(%rsp), %ymm8
	vmovaps	4640(%rsp), %ymm13
	vmovaps	4672(%rsp), %ymm14
	movq	%rdi, 2720(%rsp)
.Ltmp8776:
	.loc	10 1916 50
	cmpq	%rdi, %r10
	movq	%rdx, 2624(%rsp)
.Ltmp8777:
	.loc	3 900 12
	jne	.LBB34_418
.Ltmp8778:
	.loc	1 0 0 is_stmt 0
	vmovaps	4704(%rsp), %ymm6
	vmovaps	%ymm6, 352(%rsp)
.Ltmp8779:
	.loc	3 900 12
	jmp	.LBB34_422
.Ltmp8780:
.LBB34_418:
	.loc	3 0 12
	vmovaps	(%r9), %ymm6
	vmovaps	%ymm6, 448(%rsp)
	vmovaps	32(%r9), %ymm6
	vmovaps	%ymm6, 416(%rsp)
	vmovaps	64(%r9), %ymm6
	vmovaps	%ymm6, 64(%rsp)
	vmovaps	%ymm5, %ymm7
	vmovaps	96(%r9), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	128(%r9), %ymm5
	vmovaps	%ymm5, 704(%rsp)
	vmovaps	160(%r9), %ymm5
	vmovaps	%ymm5, 128(%rsp)
	vmovaps	192(%r9), %ymm5
	vmovaps	%ymm5, 256(%rsp)
	vmovaps	224(%r9), %ymm5
	vmovaps	%ymm5, 1728(%rsp)
	vmovaps	256(%r9), %ymm5
	vmovaps	%ymm5, 2432(%rsp)
	vmovaps	288(%r9), %ymm5
	vmovaps	%ymm5, 768(%rsp)
	vmovaps	320(%r9), %ymm5
	vmovaps	%ymm5, 1504(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm14, %ymm15
	vmovaps	%ymm13, %ymm12
	vmovaps	%ymm8, %ymm9
	vmovaps	%ymm11, %ymm6
	vmovaps	352(%r9), %ymm5
	vmovaps	%ymm5, 2272(%rsp)
	vmovaps	384(%r9), %ymm5
	vmovaps	%ymm5, 1312(%rsp)
	vmovaps	416(%r9), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	448(%r9), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	480(%r9), %ymm5
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	512(%r9), %ymm5
	vmovaps	%ymm5, 2240(%rsp)
	vmovaps	544(%r9), %ymm5
	vmovaps	%ymm5, 1280(%rsp)
	vmovaps	576(%r9), %ymm5
	vmovaps	%ymm5, 2208(%rsp)
	vmovaps	608(%r9), %ymm5
	vmovaps	%ymm5, 1696(%rsp)
	vmovaps	640(%r9), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	672(%r9), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	vmovaps	704(%r9), %ymm5
	vmovaps	%ymm5, 2112(%rsp)
	vmovaps	736(%r9), %ymm5
	vmovaps	%ymm5, 1664(%rsp)
	vmovaps	768(%r9), %ymm5
	vmovaps	%ymm5, 1632(%rsp)
	vmovaps	800(%r9), %ymm5
	vmovaps	%ymm5, 2080(%rsp)
	vmovaps	832(%r9), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	864(%r9), %ymm5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	896(%r9), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	928(%r9), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	960(%r9), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	992(%r9), %ymm5
	vmovaps	%ymm5, 1568(%rsp)
	vmovaps	1024(%r9), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	1056(%r9), %ymm5
	vmovaps	%ymm5, 1536(%rsp)
	vmovaps	1088(%r9), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	1120(%r9), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	1152(%r9), %ymm5
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	1184(%r9), %ymm5
	vmovaps	%ymm5, 1376(%rsp)
	vmovaps	1216(%r9), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	1248(%r9), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	1280(%r9), %ymm5
	vmovaps	%ymm5, 672(%rsp)
	vmovaps	1312(%r9), %ymm5
	vmovaps	%ymm5, 1760(%rsp)
	vmovaps	1344(%r9), %ymm5
	vmovaps	%ymm5, 2400(%rsp)
	vmovaps	1376(%r9), %ymm5
	vmovaps	%ymm5, 2368(%rsp)
	vmovaps	1408(%r9), %ymm5
	vmovaps	%ymm5, 2528(%rsp)
	vmovaps	1440(%r9), %ymm5
	vmovaps	%ymm5, 2496(%rsp)
	vmovaps	1472(%r9), %ymm5
	vmovaps	%ymm5, 2464(%rsp)
	vmovaps	1504(%r9), %ymm5
	vmovaps	%ymm5, 2656(%rsp)
	.p2align	4
.LBB34_419:
	movq	736(%rsp), %rdi
.Ltmp8781:
	.loc	5 568 12 is_stmt 1
	addq	%rcx, %rdi
	cmpq	%rsi, %rdi
	ja	.LBB34_493
.Ltmp8782:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp8783:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm4, %ymm5
	vmovaps	%ymm9, %ymm4
	vmovaps	%ymm2, %ymm9
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm7, %ymm0
	movq	480(%rsp), %rdi
	vmovaps	%ymm12, %ymm14
	vmovaps	%ymm5, 32(%rsp)
	vmovaps	%ymm4, 160(%rsp)
	vmovaps	%ymm6, %ymm13
	vmovaps	%ymm3, %ymm8
.Ltmp8784:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rdi,%rcx,4), %ymm11
.Ltmp8785:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm7
.Ltmp8786:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm11, %ymm6
	vmovaps	%ymm10, %ymm3
	vmovaps	%ymm9, %ymm5
	vmovaps	%ymm2, %ymm10
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm4
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp8787:
	.loc	29 48 14
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp8788:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm11, %ymm9
.Ltmp8789:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp8790:
	.loc	29 283 14
	vmulps	64(%rsp), %ymm11, %ymm12
.Ltmp8791:
	.loc	29 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm15, %ymm1
.Ltmp8792:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm11, %ymm15
.Ltmp8793:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	32(%rsp), %ymm0
.Ltmp8794:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm0, %ymm0
.Ltmp8795:
	.loc	29 48 14
	vaddps	%ymm0, %ymm6, %ymm0
	vmovaps	32(%rsp), %ymm6
.Ltmp8796:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm6, %ymm6
.Ltmp8797:
	.loc	29 48 14
	vaddps	%ymm6, %ymm9, %ymm6
	vmovaps	256(%rsp), %ymm9
.Ltmp8798:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm9, %ymm9
.Ltmp8799:
	.loc	29 48 14
	vaddps	%ymm9, %ymm12, %ymm9
	vmovaps	1728(%rsp), %ymm12
.Ltmp8800:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm12, %ymm12
.Ltmp8801:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp8802:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm4, %ymm15
.Ltmp8803:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8804:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm4, %ymm15
.Ltmp8805:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8806:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm4, %ymm15
.Ltmp8807:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8808:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm4, %ymm15
.Ltmp8809:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8810:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm2, %ymm15
.Ltmp8811:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8812:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm2, %ymm15
.Ltmp8813:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8814:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm2, %ymm15
.Ltmp8815:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8816:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm2, %ymm15
.Ltmp8817:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8818:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm10, %ymm15
.Ltmp8819:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8820:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm10, %ymm15
.Ltmp8821:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8822:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm10, %ymm15
.Ltmp8823:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8824:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm10, %ymm15
.Ltmp8825:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8826:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm5, %ymm15
.Ltmp8827:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8828:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm5, %ymm15
.Ltmp8829:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8830:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm5, %ymm15
.Ltmp8831:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8832:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm5, %ymm15
.Ltmp8833:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8834:
	.loc	29 283 14
	vmulps	1632(%rsp), %ymm3, %ymm15
.Ltmp8835:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8836:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm3, %ymm15
.Ltmp8837:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8838:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm3, %ymm15
.Ltmp8839:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8840:
	.loc	29 283 14
	vmulps	1600(%rsp), %ymm3, %ymm15
.Ltmp8841:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8842:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm8, %ymm15
.Ltmp8843:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8844:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm8, %ymm15
.Ltmp8845:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8846:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm8, %ymm15
.Ltmp8847:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8848:
	.loc	29 283 14
	vmulps	1568(%rsp), %ymm8, %ymm15
.Ltmp8849:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8850:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm13, %ymm15
.Ltmp8851:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8852:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm13, %ymm15
.Ltmp8853:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8854:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm13, %ymm15
.Ltmp8855:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8856:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm13, %ymm15
.Ltmp8857:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	160(%rsp), %ymm15
.Ltmp8858:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm15, %ymm15
.Ltmp8859:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
	vmovaps	160(%rsp), %ymm15
.Ltmp8860:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm15, %ymm15
.Ltmp8861:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	160(%rsp), %ymm15
.Ltmp8862:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm15, %ymm15
.Ltmp8863:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	160(%rsp), %ymm15
.Ltmp8864:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm15, %ymm15
.Ltmp8865:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8866:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm14, %ymm15
.Ltmp8867:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8868:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm14, %ymm15
.Ltmp8869:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8870:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm14, %ymm15
.Ltmp8871:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8872:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm14, %ymm15
.Ltmp8873:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8874:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm1, %ymm15
.Ltmp8875:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8876:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm1, %ymm15
.Ltmp8877:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8878:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm1, %ymm15
.Ltmp8879:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	%ymm1, 352(%rsp)
.Ltmp8880:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm1, %ymm15
.Ltmp8881:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8882:
	.loc	29 82 19
	vandps	%ymm7, %ymm3, %ymm15
.Ltmp8883:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm7, %ymm0, %ymm0
.Ltmp8884:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm15, %ymm0
.Ltmp8885:
	.loc	29 82 19
	vandps	%ymm7, %ymm6, %ymm6
.Ltmp8886:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp8887:
	.loc	29 82 19
	vandps	%ymm7, %ymm9, %ymm6
.Ltmp8888:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp8889:
	.loc	29 82 19
	vandps	%ymm7, %ymm12, %ymm6
.Ltmp8890:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp8891:
	.loc	11 551 14
	vmovups	%ymm0, 10280(%rsp,%rcx,4)
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm2, %ymm1
	vmovaps	%ymm10, %ymm2
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm11, %ymm4
	vmovaps	%ymm8, %ymm11
	vmovaps	%ymm13, %ymm8
	vmovaps	160(%rsp), %ymm13
	vmovaps	32(%rsp), %ymm5
.Ltmp8892:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm5, %ymm7
	vmovaps	%ymm14, %ymm15
	vmovaps	%ymm13, %ymm12
	vmovaps	%ymm8, %ymm9
	vmovaps	%ymm11, %ymm6
	cmpq	%rcx, %rax
.Ltmp8893:
	.loc	3 900 12
	jne	.LBB34_419
.Ltmp8894:
.LBB34_422:
	.loc	1 1767 5
	vmovaps	%ymm4, 4352(%rsp)
	vmovaps	%ymm5, 4384(%rsp)
	vmovaps	%ymm0, 4416(%rsp)
	vmovaps	%ymm1, 4448(%rsp)
	vmovaps	%ymm2, 4480(%rsp)
	vmovaps	%ymm10, 4512(%rsp)
	vmovaps	%ymm3, 4544(%rsp)
	vmovaps	%ymm11, 4576(%rsp)
	vmovaps	%ymm8, 4608(%rsp)
	vmovaps	%ymm13, 4640(%rsp)
	vmovaps	%ymm14, 4672(%rsp)
	vmovaps	352(%rsp), %ymm0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp8895:
	.loc	1 1761 23
	vmovaps	5088(%rsp), %ymm4
	vmovaps	5120(%rsp), %ymm5
	vmovaps	5152(%rsp), %ymm0
	vmovaps	5184(%rsp), %ymm1
	vmovaps	5216(%rsp), %ymm2
	vmovaps	5248(%rsp), %ymm10
	vmovaps	5280(%rsp), %ymm3
	vmovaps	5312(%rsp), %ymm11
	vmovaps	5344(%rsp), %ymm8
	vmovaps	5376(%rsp), %ymm13
	vmovaps	5408(%rsp), %ymm14
	movq	2720(%rsp), %rcx
.Ltmp8896:
	.loc	10 1916 50
	cmpq	%rcx, 16(%rsp)
.Ltmp8897:
	.loc	3 900 12
	jne	.LBB34_424
.Ltmp8898:
	.loc	1 0 0 is_stmt 0
	vmovaps	5440(%rsp), %ymm6
	vmovaps	%ymm6, 352(%rsp)
.Ltmp8899:
	.loc	3 900 12
	jmp	.LBB34_428
.Ltmp8900:
.LBB34_424:
	.loc	3 0 12
	vmovaps	(%r9), %ymm6
	vmovaps	%ymm6, 448(%rsp)
	vmovaps	32(%r9), %ymm6
	vmovaps	%ymm6, 416(%rsp)
	vmovaps	64(%r9), %ymm6
	vmovaps	%ymm6, 64(%rsp)
	vmovaps	%ymm14, %ymm15
	vmovaps	%ymm5, %ymm7
	vmovaps	96(%r9), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	128(%r9), %ymm5
	vmovaps	%ymm5, 704(%rsp)
	vmovaps	160(%r9), %ymm5
	vmovaps	%ymm5, 128(%rsp)
	vmovaps	192(%r9), %ymm5
	vmovaps	%ymm5, 256(%rsp)
	vmovaps	224(%r9), %ymm5
	vmovaps	%ymm5, 1728(%rsp)
	vmovaps	256(%r9), %ymm5
	vmovaps	%ymm5, 2432(%rsp)
	vmovaps	288(%r9), %ymm5
	vmovaps	%ymm5, 768(%rsp)
	vmovaps	320(%r9), %ymm5
	vmovaps	%ymm5, 1504(%rsp)
	movq	%r8, %rdx
	xorl	%ecx, %ecx
	vmovaps	%ymm13, %ymm12
	vmovaps	%ymm8, %ymm9
	vmovaps	%ymm11, %ymm6
	vmovaps	352(%r9), %ymm5
	vmovaps	%ymm5, 2272(%rsp)
	vmovaps	384(%r9), %ymm5
	vmovaps	%ymm5, 1312(%rsp)
	vmovaps	416(%r9), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	448(%r9), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	480(%r9), %ymm5
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	512(%r9), %ymm5
	vmovaps	%ymm5, 2240(%rsp)
	vmovaps	544(%r9), %ymm5
	vmovaps	%ymm5, 1280(%rsp)
	vmovaps	576(%r9), %ymm5
	vmovaps	%ymm5, 2208(%rsp)
	vmovaps	608(%r9), %ymm5
	vmovaps	%ymm5, 1696(%rsp)
	vmovaps	640(%r9), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	672(%r9), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	vmovaps	704(%r9), %ymm5
	vmovaps	%ymm5, 2112(%rsp)
	vmovaps	736(%r9), %ymm5
	vmovaps	%ymm5, 1664(%rsp)
	vmovaps	768(%r9), %ymm5
	vmovaps	%ymm5, 1632(%rsp)
	vmovaps	800(%r9), %ymm5
	vmovaps	%ymm5, 2080(%rsp)
	vmovaps	832(%r9), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	864(%r9), %ymm5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	896(%r9), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	928(%r9), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	960(%r9), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	992(%r9), %ymm5
	vmovaps	%ymm5, 1568(%rsp)
	vmovaps	1024(%r9), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	1056(%r9), %ymm5
	vmovaps	%ymm5, 1536(%rsp)
	vmovaps	1088(%r9), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	1120(%r9), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	1152(%r9), %ymm5
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	1184(%r9), %ymm5
	vmovaps	%ymm5, 1376(%rsp)
	vmovaps	1216(%r9), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	1248(%r9), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	1280(%r9), %ymm5
	vmovaps	%ymm5, 672(%rsp)
	vmovaps	1312(%r9), %ymm5
	vmovaps	%ymm5, 1760(%rsp)
	vmovaps	1344(%r9), %ymm5
	vmovaps	%ymm5, 2400(%rsp)
	vmovaps	1376(%r9), %ymm5
	vmovaps	%ymm5, 2368(%rsp)
	vmovaps	1408(%r9), %ymm5
	vmovaps	%ymm5, 2528(%rsp)
	vmovaps	1440(%r9), %ymm5
	vmovaps	%ymm5, 2496(%rsp)
	vmovaps	1472(%r9), %ymm5
	vmovaps	%ymm5, 2464(%rsp)
	vmovaps	1504(%r9), %ymm5
	vmovaps	%ymm5, 2656(%rsp)
	.p2align	4
.LBB34_425:
	movq	736(%rsp), %rdi
.Ltmp8901:
	.loc	5 568 12 is_stmt 1
	addq	%rcx, %rdi
	cmpq	%r13, %rdi
	ja	.LBB34_98
.Ltmp8902:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp8903:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm9, %ymm14
	vmovaps	%ymm2, %ymm9
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm7, %ymm0
	vmovaps	%ymm12, 32(%rsp)
	vmovaps	%ymm4, 160(%rsp)
	vmovaps	%ymm6, %ymm13
	vmovaps	%ymm3, %ymm8
.Ltmp8904:
	.loc	11 551 14 is_stmt 1
	vmovups	(%r11,%rcx,4), %ymm11
.Ltmp8905:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm7
.Ltmp8906:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm11, %ymm6
	vmovaps	%ymm10, %ymm3
	vmovaps	%ymm9, %ymm5
	vmovaps	%ymm2, %ymm10
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm4
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp8907:
	.loc	29 48 14
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp8908:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm11, %ymm9
.Ltmp8909:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp8910:
	.loc	29 283 14
	vmulps	64(%rsp), %ymm11, %ymm12
.Ltmp8911:
	.loc	29 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm15, %ymm1
.Ltmp8912:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm11, %ymm15
.Ltmp8913:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	160(%rsp), %ymm0
.Ltmp8914:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm0, %ymm0
.Ltmp8915:
	.loc	29 48 14
	vaddps	%ymm0, %ymm6, %ymm0
	vmovaps	160(%rsp), %ymm6
.Ltmp8916:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm6, %ymm6
.Ltmp8917:
	.loc	29 48 14
	vaddps	%ymm6, %ymm9, %ymm6
	vmovaps	160(%rsp), %ymm9
.Ltmp8918:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm9, %ymm9
.Ltmp8919:
	.loc	29 48 14
	vaddps	%ymm9, %ymm12, %ymm9
	vmovaps	160(%rsp), %ymm12
.Ltmp8920:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm12, %ymm12
.Ltmp8921:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp8922:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm4, %ymm15
.Ltmp8923:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8924:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm4, %ymm15
.Ltmp8925:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8926:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm4, %ymm15
.Ltmp8927:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8928:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm4, %ymm15
.Ltmp8929:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8930:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm2, %ymm15
.Ltmp8931:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8932:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm2, %ymm15
.Ltmp8933:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8934:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm2, %ymm15
.Ltmp8935:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8936:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm2, %ymm15
.Ltmp8937:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8938:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm10, %ymm15
.Ltmp8939:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8940:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm10, %ymm15
.Ltmp8941:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8942:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm10, %ymm15
.Ltmp8943:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8944:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm10, %ymm15
.Ltmp8945:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8946:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm5, %ymm15
.Ltmp8947:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8948:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm5, %ymm15
.Ltmp8949:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8950:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm5, %ymm15
.Ltmp8951:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8952:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm5, %ymm15
.Ltmp8953:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8954:
	.loc	29 283 14
	vmulps	1632(%rsp), %ymm3, %ymm15
.Ltmp8955:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8956:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm3, %ymm15
.Ltmp8957:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8958:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm3, %ymm15
.Ltmp8959:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8960:
	.loc	29 283 14
	vmulps	1600(%rsp), %ymm3, %ymm15
.Ltmp8961:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8962:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm8, %ymm15
.Ltmp8963:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8964:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm8, %ymm15
.Ltmp8965:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8966:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm8, %ymm15
.Ltmp8967:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8968:
	.loc	29 283 14
	vmulps	1568(%rsp), %ymm8, %ymm15
.Ltmp8969:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8970:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm13, %ymm15
.Ltmp8971:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8972:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm13, %ymm15
.Ltmp8973:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8974:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm13, %ymm15
.Ltmp8975:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8976:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm13, %ymm15
.Ltmp8977:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8978:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm14, %ymm15
.Ltmp8979:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8980:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm14, %ymm15
.Ltmp8981:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8982:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm14, %ymm15
.Ltmp8983:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8984:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm14, %ymm15
.Ltmp8985:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	672(%rsp), %ymm15
.Ltmp8986:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp8987:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
	vmovaps	1760(%rsp), %ymm15
.Ltmp8988:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp8989:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	2400(%rsp), %ymm15
.Ltmp8990:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp8991:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	2368(%rsp), %ymm15
.Ltmp8992:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp8993:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp8994:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm1, %ymm15
.Ltmp8995:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp8996:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm1, %ymm15
.Ltmp8997:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp8998:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm1, %ymm15
.Ltmp8999:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	%ymm1, 352(%rsp)
.Ltmp9000:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm1, %ymm15
.Ltmp9001:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp9002:
	.loc	29 82 19
	vandps	%ymm7, %ymm3, %ymm15
.Ltmp9003:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm7, %ymm0, %ymm0
.Ltmp9004:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm15, %ymm0
.Ltmp9005:
	.loc	29 82 19
	vandps	%ymm7, %ymm6, %ymm6
.Ltmp9006:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp9007:
	.loc	29 82 19
	vandps	%ymm7, %ymm9, %ymm6
.Ltmp9008:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp9009:
	.loc	29 82 19
	vandps	%ymm7, %ymm12, %ymm6
.Ltmp9010:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp9011:
	.loc	11 551 14
	vmovups	%ymm0, 9248(%rsp,%rcx,4)
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm2, %ymm1
	vmovaps	%ymm10, %ymm2
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm11, %ymm4
	vmovaps	%ymm8, %ymm11
	vmovaps	%ymm13, %ymm8
	vmovaps	%ymm14, %ymm13
	vmovaps	160(%rsp), %ymm5
	vmovaps	32(%rsp), %ymm14
.Ltmp9012:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm5, %ymm7
	vmovaps	%ymm14, %ymm15
	vmovaps	%ymm13, %ymm12
	vmovaps	%ymm8, %ymm9
	vmovaps	%ymm11, %ymm6
	cmpq	%rcx, %rax
.Ltmp9013:
	.loc	3 900 12
	jne	.LBB34_425
.Ltmp9014:
.LBB34_428:
	.loc	1 1767 5
	vmovaps	%ymm4, 5088(%rsp)
	vmovaps	%ymm5, 5120(%rsp)
	vmovaps	%ymm0, 5152(%rsp)
	vmovaps	%ymm1, 5184(%rsp)
	vmovaps	%ymm2, 5216(%rsp)
	vmovaps	%ymm10, 5248(%rsp)
	vmovaps	%ymm3, 5280(%rsp)
	vmovaps	%ymm11, 5312(%rsp)
	vmovaps	%ymm8, 5344(%rsp)
	vmovaps	%ymm13, 5376(%rsp)
	vmovaps	%ymm14, 5408(%rsp)
	vmovaps	352(%rsp), %ymm0
	vmovaps	%ymm0, 5440(%rsp)
	movq	16(%rsp), %r10
	movq	2720(%rsp), %rdi
.Ltmp9015:
	.loc	10 1916 50
	cmpq	%rdi, %r10
.Ltmp9016:
	.loc	1 1991 19
	jne	.LBB34_430
.Ltmp9017:
	.loc	1 0 19 is_stmt 0
	movq	2688(%rsp), %rax
.LBB34_415:
	addq	$32, %rdi
	decq	%rax
	movq	2592(%rsp), %rcx
.Ltmp9018:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	addq	$256, 736(%rsp)
	movq	2624(%rsp), %rdx
	addq	$-256, %rdx
	addq	$1024, 480(%rsp)
	addq	$-256, %r8
	addq	$1024, %r11
	movq	%rax, 2688(%rsp)
	testq	%rax, %rax
	movl	$32, %r15d
	jne	.LBB34_416
	jmp	.LBB34_400
.Ltmp9019:
.LBB34_430:
	.loc	8 0 20 is_stmt 0
	movq	%r8, 2368(%rsp)
	movq	%r11, 2400(%rsp)
	movq	984(%rsp), %rdi
	movq	%rbx, %r15
	movq	992(%rsp), %r12
	movq	632(%rsp), %rbx
	movq	640(%rsp), %r10
	movq	928(%rsp), %rcx
	movq	936(%rsp), %rdx
	movq	976(%rsp), %rax
	movq	%rax, 2144(%rsp)
	movq	944(%rsp), %rax
	movq	%rax, 2048(%rsp)
	movq	952(%rsp), %rax
	movq	%rax, 2080(%rsp)
	movq	968(%rsp), %rax
	movq	%rax, 1600(%rsp)
	movq	960(%rsp), %rax
	movq	%rax, 2016(%rsp)
	movq	576(%rsp), %r8
	movq	584(%rsp), %rax
	movq	%rax, 1984(%rsp)
	vmovaps	896(%rsp), %ymm11
	vmovaps	544(%rsp), %ymm4
	movq	624(%rsp), %rsi
	movq	%rsi, 1952(%rsp)
	movq	592(%rsp), %rsi
	movq	%rsi, 1568(%rsp)
	movq	600(%rsp), %rsi
	movq	%rsi, 1920(%rsp)
	movq	616(%rsp), %rsi
	movq	%rsi, 1536(%rsp)
	movq	608(%rsp), %rsi
	movq	%rsi, 1888(%rsp)
	movl	1000(%rsp), %esi
	movl	%esi, 128(%rsp)
	movl	648(%rsp), %esi
	movl	%esi, 448(%rsp)
	xorl	%r11d, %r11d
	movq	2336(%rsp), %r13
	vbroadcastss	.LCPI34_2(%rip), %ymm14
	movq	864(%rsp), %rsi
	xorl	%r14d, %r14d
	movq	%rdi, 1792(%rsp)
	movq	%r12, 672(%rsp)
	movq	%rbx, 1760(%rsp)
	movq	%r10, 1824(%rsp)
	movq	%rdx, 1856(%rsp)
.LBB34_431:
	movq	%r11, 32(%rsp)
.Ltmp9020:
	.loc	1 1996 21 is_stmt 1
	movq	%rsi, %r10
	subq	%r11, %r10
.Ltmp9021:
	.loc	1 1577 16
	movq	1624(%r9), %r11
	movq	%r12, %rax
.Ltmp9022:
	.loc	1 1578 16
	movq	1632(%r9), %r12
.Ltmp9023:
	.loc	1 1579 25
	leaq	1(%r13), %rsi
.Ltmp9024:
	.loc	1 1150 8
	cmpq	%r11, %rsi
	movq	%r11, %rsi
	cmovbq	%r14, %rsi
	negq	%rsi
	addq	%r13, %rsi
	incq	%rsi
	movq	%rsi, 768(%rsp)
.Ltmp9025:
	.loc	1 1580 28
	addq	%r13, %rdi
.Ltmp9026:
	.loc	1 1150 8
	cmpq	%r11, %rdi
	movq	%r11, %rsi
	cmovbq	%r14, %rsi
	subq	%rsi, %rdi
	movq	%rdi, 2432(%rsp)
.Ltmp9027:
	.loc	1 1581 29
	leaq	(%rbx,%r13), %r9
.Ltmp9028:
	.loc	1 1150 8
	cmpq	%r11, %r9
	movq	%r11, %rsi
	cmovbq	%r14, %rsi
	subq	%rsi, %r9
.Ltmp9029:
	.loc	1 1582 33
	addq	%r13, %rax
.Ltmp9030:
	.loc	1 1150 8
	cmpq	%r11, %rax
	movq	%r11, %rsi
	cmovbq	%r14, %rsi
	subq	%rsi, %rax
	movq	%rax, 1504(%rsp)
	movq	1824(%rsp), %rax
.Ltmp9031:
	.loc	1 1583 34
	leaq	(%rax,%r13), %rdi
.Ltmp9032:
	.loc	1 1150 8
	cmpq	%r11, %rdi
	movq	%r11, %rsi
	cmovbq	%r14, %rsi
	subq	%rsi, %rdi
.Ltmp9033:
	.loc	1 1585 14
	movq	%r11, %rbx
	movq	%r13, 2336(%rsp)
	subq	%r13, %rbx
.Ltmp9034:
	.loc	10 1078 5
	cmpq	%r10, %rbx
	cmovbq	%rbx, %r10
	movq	%r15, 1728(%rsp)
.Ltmp9035:
	.loc	1 1586 14
	subq	%r15, %r12
.Ltmp9036:
	.loc	10 1078 5
	cmpq	%r10, %r12
	movq	%r12, 160(%rsp)
	cmovbq	%r12, %r10
	movq	32(%rsp), %rax
.Ltmp9037:
	.loc	1 1587 14
	movq	%r11, %r13
	movq	768(%rsp), %rsi
	subq	%rsi, %r13
.Ltmp9038:
	.loc	10 1078 5
	cmpq	%r10, %r13
	cmovbq	%r13, %r10
.Ltmp9039:
	.loc	1 1588 14
	movq	%r11, %r14
	movq	2432(%rsp), %rsi
	subq	%rsi, %r14
.Ltmp9040:
	.loc	10 1078 5
	cmpq	%r10, %r14
	cmovbq	%r14, %r10
.Ltmp9041:
	.loc	1 1589 14
	movq	%r11, %r15
	movq	%r9, 2272(%rsp)
	subq	%r9, %r15
.Ltmp9042:
	.loc	10 1078 5
	cmpq	%r10, %r15
	cmovbq	%r15, %r10
.Ltmp9043:
	.loc	1 1590 14
	movq	%r11, %r9
	movq	1504(%rsp), %rsi
	subq	%rsi, %r9
.Ltmp9044:
	.loc	10 1078 5
	cmpq	%r10, %r9
	cmovbq	%r9, %r10
	movq	%rdi, 1312(%rsp)
.Ltmp9045:
	.loc	1 1591 14
	subq	%rdi, %r11
.Ltmp9046:
	.loc	10 1078 5
	cmpq	%r10, %r11
	cmovbq	%r11, %r10
	movq	2720(%rsp), %rsi
.Ltmp9047:
	.loc	1 2002 28
	leaq	(%rax,%rsi), %rdi
	movq	%r10, 1248(%rsp)
.Ltmp9048:
	.loc	1 2004 55
	leaq	(%r10,%rdi), %rsi
.Ltmp9049:
	.loc	1 2002 28
	shlq	$3, %rdi
.Ltmp9050:
	.loc	1 2004 55
	shlq	$3, %rsi
.Ltmp9051:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB34_433
	cmpq	24(%rsp), %rsi
	ja	.LBB34_433
.Ltmp9052:
	.loc	5 451 16
	cmpq	248(%rsp), %rsi
	ja	.LBB34_436
.Ltmp9053:
	.loc	1 2006 46
	leaq	(,%rax,8), %r12
	movq	1248(%rsp), %r10
	.loc	1 2006 61 is_stmt 0
	leaq	(%r10,%rax), %rsi
	movq	%rsi, 1376(%rsp)
	cmpq	$33, %rsi
.Ltmp9054:
	.loc	4 1050 16 is_stmt 1
	jae	.LBB34_631
.Ltmp9055:
	.loc	14 304 12
	testq	%r10, %r10
	je	.LBB34_440
.Ltmp9056:
	.loc	14 0 12 is_stmt 0
	cmpq	%r15, %r14
	cmovbq	%r14, %r15
	cmpq	%r9, %r15
	cmovaeq	%r9, %r15
	cmpq	%r11, %r15
	cmovaeq	%r11, %r15
	cmpq	%r13, %r15
	cmovaeq	%r13, %r15
	cmpq	%rbx, %r15
	cmovaeq	%rbx, %r15
	movq	344(%rsp), %r9
	leaq	(%r9,%rdi,4), %rsi
	movq	%rsi, 2176(%rsp)
	movq	408(%rsp), %r9
	leaq	(%r9,%rdi,4), %rsi
	movq	%rsi, 1664(%rsp)
	movq	160(%rsp), %rsi
	cmpq	%rsi, %r15
	cmovaeq	%rsi, %r15
	movq	4256(%rsp), %rsi
	subq	%rax, %rsi
	leaq	(%rsp,%r12,4), %rdi
	addq	$10280, %rdi
	movq	%rdi, 1280(%rsp)
	leaq	(%rsp,%r12,4), %rdi
	addq	$9248, %rdi
	movq	%rdi, 2208(%rsp)
	cmpq	%rsi, %r15
	cmovbq	%r15, %rsi
	andq	800(%rsp), %rsi
	movq	%rsi, 1632(%rsp)
.Ltmp9057:
	.loc	1 855 44 is_stmt 1
	vmovaps	4864(%rsp), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	1 855 61 is_stmt 0
	vmovaps	4928(%rsp), %ymm15
.Ltmp9058:
	.loc	1 855 44
	vmovaps	5472(%rsp), %ymm0
	vmovaps	%ymm0, 160(%rsp)
	.loc	1 855 73
	vmovaps	5504(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	.loc	1 855 61
	vmovaps	5536(%rsp), %ymm0
	vmovaps	%ymm0, 64(%rsp)
.Ltmp9059:
	.loc	1 853 26 is_stmt 1
	vmovaps	5568(%rsp), %ymm0
	vmovaps	%ymm0, 416(%rsp)
.Ltmp9060:
	.loc	1 853 26 is_stmt 0
	vmovaps	5696(%rsp), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	vmovaps	4736(%rsp), %ymm1
	vmovaps	4768(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	4800(%rsp), %ymm2
	vmovaps	4832(%rsp), %ymm0
	vmovaps	4896(%rsp), %ymm3
	vmovaps	%ymm3, 1408(%rsp)
	vmovaps	5632(%rsp), %ymm3
	vmovaps	%ymm3, 2240(%rsp)
	vmovaps	5056(%rsp), %ymm3
	vmovaps	%ymm3, 1696(%rsp)
	vmovaps	5792(%rsp), %ymm3
	vmovaps	%ymm3, 2112(%rsp)
	vmovaps	4960(%rsp), %ymm10
	xorl	%r15d, %r15d
	vmovaps	5600(%rsp), %ymm3
	vmovaps	%ymm3, 352(%rsp)
	vmovaps	5664(%rsp), %ymm3
	vmovaps	%ymm3, 704(%rsp)
	vmovaps	5024(%rsp), %ymm8
	vmovaps	5760(%rsp), %ymm3
	movq	2336(%rsp), %r13
.Ltmp9061:
	.loc	1 0 26
.Ltmp9062:
	.p2align	4
.LBB34_442:
	vmovaps	%ymm4, 256(%rsp)
	vbroadcastss	.LCPI34_1(%rip), %ymm9
.Ltmp9063:
	.loc	29 347 14 is_stmt 1
	vaddps	%ymm0, %ymm9, %ymm0
	vxorps	%xmm12, %xmm12, %xmm12
.Ltmp9064:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm0, %ymm0
.Ltmp9065:
	.loc	29 871 14
	vcmpgt_oqps	%ymm12, %ymm0, %ymm6
.Ltmp9066:
	.loc	29 48 14
	vaddps	%ymm2, %ymm1, %ymm1
	vmovaps	1440(%rsp), %ymm4
.Ltmp9067:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm1, %ymm4, %ymm1
.Ltmp9068:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm6, %ymm2, %ymm12, %ymm2
.Ltmp9069:
	.loc	29 347 14 is_stmt 1
	vaddps	%ymm9, %ymm10, %ymm5
.Ltmp9070:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm5, %ymm10
.Ltmp9071:
	.loc	29 871 14
	vcmpgt_oqps	%ymm12, %ymm10, %ymm6
.Ltmp9072:
	.loc	29 48 14
	vaddps	32(%rsp), %ymm15, %ymm4
	vmovaps	1408(%rsp), %ymm5
.Ltmp9073:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm4, %ymm5, %ymm4
	vmovaps	%ymm4, 32(%rsp)
.Ltmp9074:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm6, %ymm15, %ymm12, %ymm15
.Ltmp9075:
	.loc	29 347 14 is_stmt 1
	vaddps	416(%rsp), %ymm9, %ymm6
.Ltmp9076:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm6, %ymm4
	vmovaps	%ymm4, 416(%rsp)
.Ltmp9077:
	.loc	29 871 14
	vcmpgt_oqps	%ymm12, %ymm4, %ymm6
	vmovaps	64(%rsp), %ymm5
.Ltmp9078:
	.loc	29 48 14
	vaddps	160(%rsp), %ymm5, %ymm7
	vmovaps	1472(%rsp), %ymm4
.Ltmp9079:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm7, %ymm4, %ymm4
	vmovaps	%ymm4, 160(%rsp)
.Ltmp9080:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm6, %ymm5, %ymm12, %ymm5
	vmovaps	%ymm5, 64(%rsp)
.Ltmp9081:
	.loc	29 347 14 is_stmt 1
	vaddps	96(%rsp), %ymm9, %ymm6
.Ltmp9082:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm6, %ymm4
.Ltmp9083:
	.loc	29 871 14
	vcmpgt_oqps	%ymm12, %ymm4, %ymm6
	vmovaps	704(%rsp), %ymm13
.Ltmp9084:
	.loc	29 48 14
	vaddps	352(%rsp), %ymm13, %ymm7
	vmovaps	2240(%rsp), %ymm5
.Ltmp9085:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm7, %ymm5, %ymm5
	vmovaps	%ymm5, 352(%rsp)
.Ltmp9086:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm6, %ymm13, %ymm12, %ymm13
.Ltmp9087:
	.loc	1 1504 26 is_stmt 1
	leaq	(%r15,%r13), %rsi
.Ltmp9088:
	.loc	1 1139 16
	leaq	(,%rsi,8), %r9
.Ltmp9089:
	.loc	1 1140 33
	leaq	8(,%rsi,8), %r12
.Ltmp9090:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %rsi
	cmpq	%rdx, %rsi
	jae	.LBB34_632
.Ltmp9091:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm4, 96(%rsp)
	movq	%r15, %rbx
	shlq	$5, %rbx
	movq	1280(%rsp), %rsi
	vmovups	(%rsi,%rbx), %ymm9
	movq	2208(%rsp), %rsi
	vmovups	(%rsi,%rbx), %ymm7
.Ltmp9092:
	vmaxps	%ymm9, %ymm7, %ymm6
	vmovaps	1344(%rsp), %ymm4
.Ltmp9093:
	.loc	29 585 19 is_stmt 1
	vblendvps	%ymm4, %ymm6, %ymm9, %ymm9
.Ltmp9094:
	.loc	29 360 14
	vdivps	%ymm9, %ymm1, %ymm12
	movq	2432(%rsp), %rsi
.Ltmp9095:
	.loc	1 0 0 is_stmt 0
	addq	%r15, %rsi
.Ltmp9096:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm1, %ymm9, %ymm9
.Ltmp9097:
	.loc	29 585 19
	vblendvps	%ymm9, %ymm12, %ymm14, %ymm9
.Ltmp9098:
	.loc	11 551 14
	vmovups	%ymm9, (%rcx,%r9,4)
.Ltmp9099:
	.loc	1 1132 16
	leaq	(,%rsi,8), %rdi
.Ltmp9100:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %r10
	cmpq	%rdx, %r10
	movl	128(%rsp), %r13d
	jae	.LBB34_444
.Ltmp9101:
	.loc	11 551 14
	vmovups	(%rcx,%rdi,4), %ymm9
	vmovaps	%ymm9, %ymm5
.Ltmp9102:
	.loc	1 1208 22
	testl	%r13d, %r13d
	je	.LBB34_449
.Ltmp9103:
	.loc	29 257 14
	vminps	%ymm9, %ymm11, %ymm5
.Ltmp9104:
.LBB34_449:
	.loc	29 0 14 is_stmt 0
	movq	768(%rsp), %rdi
	leaq	(%r15,%rdi), %r10
.Ltmp9105:
	movl	%r13d, %r11d
.Ltmp9106:
	.loc	1 1214 20 is_stmt 1
	incq	%r11
	movq	2144(%rsp), %rdi
	movq	%rdi, %r14
	cmpq	%rdi, %r11
.Ltmp9107:
	.loc	1 1215 22
	jne	.LBB34_450
	.loc	1 0 22 is_stmt 0
.Ltmp9108:
	.p2align	4
.LBB34_453:
.Ltmp9109:
	.loc	1 1132 16 is_stmt 1
	leaq	(,%rsi,8), %rdi
.Ltmp9110:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %r11
	cmpq	%rdx, %r11
	jae	.LBB34_451
.Ltmp9111:
	.loc	29 257 14
	vminps	(%rcx,%rdi,4), %ymm9, %ymm9
.Ltmp9112:
	.loc	11 551 14
	vmovups	%ymm9, (%rcx,%rdi,4)
.Ltmp9113:
	.loc	1 1226 16
	testq	%rsi, %rsi
	cmoveq	192(%rsp), %rsi
	.loc	1 1229 13
	decq	%rsi
.Ltmp9114:
	.loc	10 1916 50
	decq	%r14
.Ltmp9115:
	.loc	3 900 12
	jne	.LBB34_453
.Ltmp9116:
	.loc	3 0 12 is_stmt 0
	xorl	%r14d, %r14d
	vmovaps	%ymm5, %ymm9
	jmp	.LBB34_456
	.p2align	4
.LBB34_450:
.Ltmp9117:
	.loc	1 1132 16 is_stmt 1
	leaq	(,%r10,8), %rdi
.Ltmp9118:
	.loc	4 1050 16
	leaq	7(,%r10,8), %rsi
	cmpq	%rdx, %rsi
	jae	.LBB34_451
.Ltmp9119:
	.loc	11 551 14
	vmovups	(%rcx,%rdi,4), %ymm9
.Ltmp9120:
	.loc	29 257 14
	vminps	%ymm5, %ymm9, %ymm9
	movl	%r11d, %r14d
.Ltmp9121:
.LBB34_456:
	.loc	29 0 14 is_stmt 0
	movq	1504(%rsp), %rsi
	leaq	(%r15,%rsi), %rdi
.Ltmp9122:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%rdi,8), %rsi
.Ltmp9123:
	.loc	1 1132 16
	shlq	$3, %rdi
	movq	2080(%rsp), %r11
.Ltmp9124:
	.loc	4 1050 16
	cmpq	%r11, %rsi
	movq	2048(%rsp), %r13
	jae	.LBB34_633
.Ltmp9125:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm4
	vmulps	%ymm4, %ymm9, %ymm9
	vroundps	$9, %ymm9, %ymm9
	vbroadcastss	.LCPI34_4(%rip), %ymm4
	vmulps	%ymm4, %ymm9, %ymm9
.Ltmp9126:
	.loc	29 48 14 is_stmt 1
	vaddps	%ymm8, %ymm9, %ymm8
.Ltmp9127:
	.loc	29 347 14
	vsubps	(%r13,%rdi,4), %ymm8, %ymm8
.Ltmp9128:
	.loc	1 1656 5
	vmovaps	%ymm8, 5024(%rsp)
	cmpq	%r11, %r12
.Ltmp9129:
	.loc	4 1050 16
	ja	.LBB34_634
.Ltmp9130:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm13, 704(%rsp)
	vmovaps	%ymm5, %ymm13
	movq	1728(%rsp), %rsi
	leaq	(%r15,%rsi), %r11
.Ltmp9131:
	.loc	11 551 14 is_stmt 1
	vmovups	%ymm9, (%r13,%r9,4)
.Ltmp9132:
	.loc	29 360 14
	vdivps	1696(%rsp), %ymm8, %ymm9
.Ltmp9133:
	.loc	1 1661 43
	vmovaps	4992(%rsp), %ymm12
.Ltmp9134:
	.loc	29 347 14
	vsubps	%ymm9, %ymm14, %ymm9
	vmovaps	%ymm14, %ymm5
.Ltmp9135:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm12, %ymm9, %ymm14
.Ltmp9136:
	.loc	29 283 14 is_stmt 1
	vmulps	32(%rsp), %ymm14, %ymm14
.Ltmp9137:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp9138:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm9, %ymm12
.Ltmp9139:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm9
	vandps	%ymm9, %ymm12, %ymm14
.Ltmp9140:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm4
	vcmplt_oqps	%ymm4, %ymm14, %ymm14
.Ltmp9141:
	.loc	29 82 19
	vandnps	%ymm12, %ymm14, %ymm12
.Ltmp9142:
	.loc	1 1662 5
	vmovaps	%ymm12, 4992(%rsp)
.Ltmp9143:
	.loc	1 1132 16
	leaq	(,%r11,8), %rdi
.Ltmp9144:
	.loc	1 1133 25
	leaq	8(,%r11,8), %rsi
.Ltmp9145:
	.loc	4 1050 16
	leaq	7(,%r11,8), %r11
	cmpq	1600(%rsp), %r11
	jae	.LBB34_635
.Ltmp9146:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	%r12, %rsi
	movq	%r9, %r12
	movq	2176(%rsp), %r9
	leaq	(%r9,%rbx), %r11
.Ltmp9147:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm12, %ymm5, %ymm12
	movq	2016(%rsp), %r13
.Ltmp9148:
	.loc	11 551 14
	vmovups	(%r13,%rdi,4), %ymm14
.Ltmp9149:
	.loc	11 551 14 is_stmt 0
	vmovups	(%r11), %ymm4
	vmovups	%ymm4, (%r13,%rdi,4)
.Ltmp9150:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm14, %ymm12, %ymm4
	vmovaps	288(%rsp), %ymm12
.Ltmp9151:
	.loc	29 585 19
	vblendvps	%ymm12, %ymm14, %ymm4, %ymm4
.Ltmp9152:
	.loc	11 551 14
	vmovups	%ymm4, (%r11)
	movq	%rsi, %rax
	movq	1984(%rsp), %r9
	cmpq	%r9, %rsi
	movq	%r9, %rsi
.Ltmp9153:
	.loc	4 1050 16
	ja	.LBB34_636
.Ltmp9154:
	.loc	4 0 16 is_stmt 0
	movl	%r14d, 128(%rsp)
	vmovaps	1344(%rsp), %ymm4
	vblendvps	%ymm4, %ymm6, %ymm7, %ymm4
	vmovaps	160(%rsp), %ymm7
.Ltmp9155:
	.loc	29 360 14 is_stmt 1
	vdivps	%ymm4, %ymm7, %ymm6
	movq	2272(%rsp), %r9
.Ltmp9156:
	.loc	1 0 0 is_stmt 0
	leaq	(%r15,%r9), %r14
.Ltmp9157:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm7, %ymm4, %ymm4
.Ltmp9158:
	.loc	29 585 19
	vblendvps	%ymm4, %ymm6, %ymm5, %ymm4
.Ltmp9159:
	.loc	11 551 14
	vmovups	%ymm4, (%r8,%r12,4)
.Ltmp9160:
	.loc	1 1132 16
	leaq	(,%r14,8), %r13
.Ltmp9161:
	.loc	4 1050 16
	leaq	7(,%r14,8), %r11
	cmpq	%rsi, %r11
	vmovaps	%ymm13, %ymm12
	jae	.LBB34_637
.Ltmp9162:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm5, %ymm14
.Ltmp9163:
	.loc	11 551 14 is_stmt 1
	vmovups	(%r8,%r13,4), %ymm6
	vmovaps	%ymm6, %ymm7
.Ltmp9164:
	.loc	1 1208 22
	cmpl	$0, 448(%rsp)
	je	.LBB34_463
	.loc	1 0 22 is_stmt 0
	vmovaps	256(%rsp), %ymm4
.Ltmp9165:
	.loc	29 257 14 is_stmt 1
	vminps	%ymm6, %ymm4, %ymm7
.Ltmp9166:
.LBB34_463:
	.loc	1 0 0 is_stmt 0
	movl	448(%rsp), %r11d
.Ltmp9167:
	.loc	1 1214 20 is_stmt 1
	incq	%r11
	movq	1952(%rsp), %r9
	movq	%r9, %r13
	cmpq	%r9, %r11
	movq	1568(%rsp), %r9
.Ltmp9168:
	.loc	1 1215 22
	jne	.LBB34_464
	.loc	1 0 22 is_stmt 0
.Ltmp9169:
	.p2align	4
.LBB34_467:
.Ltmp9170:
	.loc	1 1132 16 is_stmt 1
	leaq	(,%r14,8), %r10
.Ltmp9171:
	.loc	4 1050 16
	leaq	7(,%r14,8), %r11
	cmpq	%rsi, %r11
	jae	.LBB34_465
.Ltmp9172:
	.loc	29 257 14
	vminps	(%r8,%r10,4), %ymm6, %ymm6
.Ltmp9173:
	.loc	11 551 14
	vmovups	%ymm6, (%r8,%r10,4)
.Ltmp9174:
	.loc	1 1226 16
	testq	%r14, %r14
	cmoveq	192(%rsp), %r14
	.loc	1 1229 13
	decq	%r14
.Ltmp9175:
	.loc	10 1916 50
	decq	%r13
.Ltmp9176:
	.loc	3 900 12
	jne	.LBB34_467
.Ltmp9177:
	.loc	3 0 12 is_stmt 0
	movl	$0, 448(%rsp)
	vmovaps	%ymm7, %ymm6
	jmp	.LBB34_470
	.p2align	4
.LBB34_464:
.Ltmp9178:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r10,8), %r14
.Ltmp9179:
	.loc	1 1132 16
	shlq	$3, %r10
.Ltmp9180:
	.loc	4 1050 16
	cmpq	%rsi, %r14
	jae	.LBB34_465
.Ltmp9181:
	.loc	11 551 14
	vmovups	(%r8,%r10,4), %ymm4
.Ltmp9182:
	.loc	29 257 14
	vminps	%ymm7, %ymm4, %ymm6
	movl	%r11d, 448(%rsp)
.Ltmp9183:
.LBB34_470:
	.loc	29 0 14 is_stmt 0
	movq	1312(%rsp), %r10
	addq	%r15, %r10
.Ltmp9184:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r10,8), %r11
.Ltmp9185:
	.loc	1 1132 16
	shlq	$3, %r10
	movq	1920(%rsp), %r14
.Ltmp9186:
	.loc	4 1050 16
	cmpq	%r14, %r11
	movq	2336(%rsp), %r13
	jae	.LBB34_638
.Ltmp9187:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm4
	vmulps	%ymm4, %ymm6, %ymm4
	vroundps	$9, %ymm4, %ymm4
	vbroadcastss	.LCPI34_4(%rip), %ymm5
	vmulps	%ymm5, %ymm4, %ymm6
.Ltmp9188:
	.loc	29 48 14 is_stmt 1
	vaddps	%ymm3, %ymm6, %ymm3
.Ltmp9189:
	.loc	29 347 14
	vsubps	(%r9,%r10,4), %ymm3, %ymm3
.Ltmp9190:
	.loc	1 1656 5
	vmovaps	%ymm3, 5760(%rsp)
	cmpq	%r14, %rax
.Ltmp9191:
	.loc	4 1050 16
	ja	.LBB34_639
.Ltmp9192:
	.loc	29 360 14
	vdivps	2112(%rsp), %ymm3, %ymm4
.Ltmp9193:
	.loc	11 551 14
	vmovups	%ymm6, (%r9,%r12,4)
.Ltmp9194:
	.loc	1 1661 43
	vmovaps	5728(%rsp), %ymm6
.Ltmp9195:
	.loc	29 347 14
	vsubps	%ymm4, %ymm14, %ymm4
.Ltmp9196:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm6, %ymm4, %ymm12
.Ltmp9197:
	.loc	29 283 14 is_stmt 1
	vmulps	352(%rsp), %ymm12, %ymm12
.Ltmp9198:
	.loc	29 48 14
	vaddps	%ymm6, %ymm12, %ymm6
.Ltmp9199:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm4, %ymm4
.Ltmp9200:
	.loc	29 82 19
	vandps	%ymm4, %ymm9, %ymm6
.Ltmp9201:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm5
	vcmplt_oqps	%ymm5, %ymm6, %ymm6
.Ltmp9202:
	.loc	29 82 19
	vandnps	%ymm4, %ymm6, %ymm6
.Ltmp9203:
	.loc	1 1662 5
	vmovaps	%ymm6, 5728(%rsp)
	cmpq	1536(%rsp), %rdx
.Ltmp9204:
	.loc	4 1050 16
	ja	.LBB34_640
.Ltmp9205:
	.loc	1 0 0 is_stmt 0
	addq	1664(%rsp), %rbx
	incq	%r15
.Ltmp9206:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm6, %ymm14, %ymm4
	movq	1888(%rsp), %rsi
.Ltmp9207:
	.loc	11 551 14
	vmovups	(%rsi,%rdi,4), %ymm6
.Ltmp9208:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rbx), %ymm9
	vmovups	%ymm9, (%rsi,%rdi,4)
.Ltmp9209:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm6, %ymm4, %ymm4
	vmovaps	288(%rsp), %ymm9
.Ltmp9210:
	.loc	29 585 19
	vblendvps	%ymm9, %ymm6, %ymm4, %ymm4
.Ltmp9211:
	.loc	11 551 14
	vmovups	%ymm4, (%rbx)
	vmovaps	%ymm7, %ymm4
	vmovaps	%ymm13, %ymm11
.Ltmp9212:
	.loc	14 304 12
	cmpq	1632(%rsp), %r15
	movq	1856(%rsp), %rdx
	jne	.LBB34_442
.Ltmp9213:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9214:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9215:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9216:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9217:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9218:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9219:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9220:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	vmovaps	704(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 5664(%rsp)
	vmovaps	%ymm7, %ymm4
	vmovaps	%ymm13, %ymm11
	movq	8(%rsp), %r9
	jmp	.LBB34_475
.Ltmp9221:
.LBB34_440:
	movq	8(%rsp), %r9
	movq	2336(%rsp), %r13
.LBB34_475:
	xorl	%r10d, %r10d
	movq	1248(%rsp), %r11
	movq	1376(%rsp), %rdi
	.loc	1 2063 39 is_stmt 1
	addq	%r11, %r13
	movq	192(%rsp), %rsi
.Ltmp9222:
	.loc	1 1150 8
	cmpq	%rsi, %r13
	cmovbq	%r10, %rsi
	subq	%rsi, %r13
	movq	1728(%rsp), %r15
.Ltmp9223:
	.loc	1 2064 39
	addq	%r11, %r15
	movq	832(%rsp), %rsi
.Ltmp9224:
	.loc	1 1150 8
	cmpq	%rsi, %r15
	cmovbq	%r10, %rsi
	subq	%rsi, %r15
	movq	%rdi, %r11
	movq	864(%rsp), %rsi
.Ltmp9225:
	.loc	1 1991 19
	cmpq	%rsi, %rdi
	movq	1792(%rsp), %rdi
	movq	672(%rsp), %r12
	movl	$0, %r14d
	movq	1760(%rsp), %rbx
	jb	.LBB34_431
.Ltmp9226:
	.loc	1 0 19 is_stmt 0
	movq	%r13, 2336(%rsp)
	movl	128(%rsp), %eax
	movl	%eax, 1000(%rsp)
	vmovaps	%ymm11, 896(%rsp)
	movl	448(%rsp), %eax
	movl	%eax, 648(%rsp)
	vmovaps	%ymm4, 544(%rsp)
	movq	248(%rsp), %r13
	movq	24(%rsp), %rsi
	movq	16(%rsp), %r10
	movq	2688(%rsp), %rax
	movq	2400(%rsp), %r11
	movq	2368(%rsp), %r8
	movq	%r15, %rbx
	movq	2720(%rsp), %rdi
	jmp	.LBB34_415
.LBB34_14:
	.loc	1 2214 37 is_stmt 1
	movq	1992(%r11), %rsi
	movq	2000(%r11), %rdx
.Ltmp9227:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp9228:
	.p2align	4
.LBB34_15:
.Ltmp9229:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp9230:
	.loc	6 180 28
	je	.LBB34_18
.Ltmp9231:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp9232:
	.loc	6 315 25
	jne	.LBB34_30
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB34_15
	jmp	.LBB34_30
.Ltmp9233:
.LBB34_95:
	.loc	6 0 25
	vmovaps	480(%rsp), %ymm0
.Ltmp9234:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9235:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9236:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9237:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9238:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9239:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9240:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9241:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9242:
.LBB34_491:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_143:
	vmovaps	480(%rsp), %ymm0
.Ltmp9243:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9244:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9245:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9246:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9247:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9248:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9249:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9250:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9251:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9252:
.LBB34_18:
	movq	16(%rsp), %rdx
	leaq	(,%rdx,8), %rsi
	cmpq	24(%rsp), %rsi
.Ltmp9253:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB34_622
.Ltmp9254:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	344(%rsp), %rsi
	.p2align	4
.LBB34_20:
.Ltmp9255:
	.loc	15 1504 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB34_24
.Ltmp9256:
	.loc	10 1078 5
	cmpq	$32, %rdx
	movl	$32, %edi
	cmovbq	%rdx, %rdi
.Ltmp9257:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp9258:
	.loc	16 0 18 is_stmt 0
.Ltmp9259:
	.p2align	4
.LBB34_22:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp9260:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp9261:
	.loc	6 180 28
	jne	.LBB34_22
.Ltmp9262:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp9263:
	.loc	19 2054 74
	subq	%rdi, %rdx
.Ltmp9264:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB34_20
	jmp	.LBB34_30
.Ltmp9265:
.LBB34_615:
	.loc	5 438 16
	cmpq	%r13, %rsi
	ja	.LBB34_655
.Ltmp9266:
	.loc	5 0 16 is_stmt 0
	movq	408(%rsp), %rax
	.p2align	4
.LBB34_617:
.Ltmp9267:
	.loc	17 131 18 is_stmt 1
	movq	%rsi, %rcx
.Ltmp9268:
	.loc	15 1504 12
	testq	%rsi, %rsi
	je	.LBB34_621
.Ltmp9269:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp9270:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%r11d, %r11d
	xorl	%r8d, %r8d
.Ltmp9271:
	.loc	16 0 18 is_stmt 0
.Ltmp9272:
	.p2align	4
.LBB34_619:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r8), %r11d
.Ltmp9273:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp9274:
	.loc	6 180 28
	jne	.LBB34_619
.Ltmp9275:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp9276:
	.loc	19 2054 74
	movq	%rcx, %rsi
	subq	%rdx, %rsi
.Ltmp9277:
	.loc	17 136 12
	testl	%r11d, %r11d
	je	.LBB34_617
.Ltmp9278:
.LBB34_621:
	.loc	15 1504 12
	testq	%rcx, %rcx
	sete	%cl
	movq	24(%rsp), %rdx
.Ltmp9279:
	.loc	1 2262 35
	jmp	.LBB34_596
.LBB34_477:
	.loc	1 0 35 is_stmt 0
	movq	8(%rsp), %r11
	movq	248(%rsp), %r13
	movq	768(%rsp), %r14
.LBB34_545:
.Ltmp9280:
	.loc	1 2069 13 is_stmt 1
	vmovaps	896(%rsp), %ymm0
	vmovaps	%ymm0, 7648(%rsp)
	.loc	1 2070 13
	movl	1000(%rsp), %ecx
	.loc	1 2071 13
	vmovaps	544(%rsp), %ymm0
	vmovaps	%ymm0, 7680(%rsp)
.Ltmp9281:
	.loc	1 2079 23
	movq	1736(%r11), %rdx
.Ltmp9282:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp9283:
	.loc	1 0 0 is_stmt 0
	movl	648(%rsp), %eax
.Ltmp9284:
	.loc	1 2079 23 is_stmt 1
	movq	1728(%r11), %rdx
.Ltmp9285:
	.loc	11 551 14
	vmovaps	7648(%rsp), %ymm0
	vmovups	%ymm0, (%rdx)
.Ltmp9286:
	.loc	1 2080 5
	movq	1768(%r11), %rdx
.Ltmp9287:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp9288:
	.loc	6 180 28
	je	.LBB34_549
.Ltmp9289:
	.loc	6 0 28 is_stmt 0
	movq	1760(%r11), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB34_548:
.Ltmp9290:
	.loc	20 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp9291:
	.loc	7 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp9292:
	.loc	6 180 28
	jne	.LBB34_548
.Ltmp9293:
.LBB34_549:
	.loc	1 2081 24
	movq	1936(%r11), %rdx
.Ltmp9294:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp9295:
	.loc	1 2081 24
	movq	1928(%r11), %rcx
.Ltmp9296:
	.loc	11 551 14
	vmovaps	7680(%rsp), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp9297:
	.loc	1 2082 5
	movq	1968(%r11), %rcx
.Ltmp9298:
	.loc	7 1714 9
	testq	%rcx, %rcx
.Ltmp9299:
	.loc	6 180 28
	je	.LBB34_553
.Ltmp9300:
	.loc	6 0 28 is_stmt 0
	movq	1960(%r11), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB34_552:
.Ltmp9301:
	.loc	20 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp9302:
	.loc	7 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp9303:
	.loc	6 180 28
	jne	.LBB34_552
.Ltmp9304:
.LBB34_553:
	.loc	6 0 28 is_stmt 0
	leaq	5824(%rsp), %rdi
	movq	536(%rsp), %rsi
	.loc	1 2084 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	6560(%rsp), %rdi
	movq	528(%rsp), %rsi
	.loc	1 2085 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	8(%rsp), %r9
	.loc	1 2086 5
	movl	%r14d, 1640(%r9)
	movq	2592(%rsp), %rax
	jmp	.LBB34_554
.Ltmp9305:
.LBB34_399:
	.loc	1 0 5 is_stmt 0
	movq	8(%rsp), %r9
	movq	248(%rsp), %r13
	movq	1728(%rsp), %rbx
.LBB34_400:
.Ltmp9306:
	.loc	1 2069 13 is_stmt 1
	vmovaps	896(%rsp), %ymm0
	vmovaps	%ymm0, 7712(%rsp)
	.loc	1 2070 13
	movl	1000(%rsp), %ecx
	.loc	1 2071 13
	vmovaps	544(%rsp), %ymm0
	vmovaps	%ymm0, 7744(%rsp)
.Ltmp9307:
	.loc	1 2079 23
	movq	1736(%r9), %rdx
.Ltmp9308:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp9309:
	.loc	1 0 0 is_stmt 0
	movl	648(%rsp), %eax
.Ltmp9310:
	.loc	1 2079 23 is_stmt 1
	movq	1728(%r9), %rdx
.Ltmp9311:
	.loc	11 551 14
	vmovaps	7712(%rsp), %ymm0
	vmovups	%ymm0, (%rdx)
.Ltmp9312:
	.loc	1 2080 5
	movq	1768(%r9), %rdx
.Ltmp9313:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp9314:
	.loc	6 180 28
	je	.LBB34_404
.Ltmp9315:
	.loc	6 0 28 is_stmt 0
	movq	1760(%r9), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB34_403:
.Ltmp9316:
	.loc	20 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp9317:
	.loc	7 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp9318:
	.loc	6 180 28
	jne	.LBB34_403
.Ltmp9319:
.LBB34_404:
	.loc	1 2081 24
	movq	1936(%r9), %rdx
.Ltmp9320:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_491
.Ltmp9321:
	.loc	1 2081 24
	movq	1928(%r9), %rcx
.Ltmp9322:
	.loc	11 551 14
	vmovaps	7744(%rsp), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp9323:
	.loc	1 2082 5
	movq	1968(%r9), %rcx
.Ltmp9324:
	.loc	7 1714 9
	testq	%rcx, %rcx
.Ltmp9325:
	.loc	6 180 28
	je	.LBB34_408
.Ltmp9326:
	.loc	6 0 28 is_stmt 0
	movq	1960(%r9), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB34_407:
.Ltmp9327:
	.loc	20 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp9328:
	.loc	7 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp9329:
	.loc	6 180 28
	jne	.LBB34_407
.Ltmp9330:
.LBB34_408:
	.loc	6 0 28 is_stmt 0
	leaq	4352(%rsp), %rdi
	movq	536(%rsp), %rsi
	.loc	1 2084 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	5088(%rsp), %rdi
	movq	528(%rsp), %rsi
	.loc	1 2085 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	8(%rsp), %r9
	.loc	1 2086 5
	movl	%ebx, 1640(%r9)
	movq	2336(%rsp), %rax
	.loc	1 2087 5
	jmp	.LBB34_554
.Ltmp9331:
.LBB34_24:
	.loc	1 0 5 is_stmt 0
	movq	16(%rsp), %rdx
	leaq	(,%rdx,8), %rsi
.Ltmp9332:
	.loc	5 438 16 is_stmt 1
	cmpq	%r13, %rsi
	ja	.LBB34_35
.Ltmp9333:
	.loc	5 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	408(%rsp), %rsi
	.p2align	4
.LBB34_26:
.Ltmp9334:
	.loc	15 1504 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB34_557
.Ltmp9335:
	.loc	10 1078 5
	cmpq	$32, %rdx
	movl	$32, %edi
	cmovbq	%rdx, %rdi
.Ltmp9336:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp9337:
	.loc	16 0 18 is_stmt 0
.Ltmp9338:
	.p2align	4
.LBB34_28:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp9339:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp9340:
	.loc	6 180 28
	jne	.LBB34_28
.Ltmp9341:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp9342:
	.loc	19 2054 74
	subq	%rdi, %rdx
.Ltmp9343:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB34_26
	jmp	.LBB34_30
.Ltmp9344:
.LBB34_515:
	.loc	17 0 12 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9345:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9346:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9347:
	leaq	8(%rdi), %rsi
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_451:
.Ltmp9348:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9349:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9350:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9351:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9352:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9353:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9354:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9355:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 5664(%rsp)
.Ltmp9356:
	movl	%r13d, 1000(%rsp)
.Ltmp9357:
	vmovaps	%ymm5, 896(%rsp)
.Ltmp9358:
.LBB34_445:
	movl	448(%rsp), %eax
.Ltmp9359:
	movl	%eax, 648(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9360:
	vmovaps	%ymm0, 544(%rsp)
.Ltmp9361:
	leaq	8(%rdi), %rsi
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_465:
.Ltmp9362:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9363:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9364:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9365:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9366:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9367:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9368:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9369:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	vmovaps	704(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 5664(%rsp)
	movl	128(%rsp), %ecx
.Ltmp9370:
	movl	%ecx, 1000(%rsp)
.Ltmp9371:
	vmovaps	%ymm12, 896(%rsp)
	movl	448(%rsp), %ecx
.Ltmp9372:
	movl	%ecx, 648(%rsp)
.Ltmp9373:
	vmovaps	%ymm7, 544(%rsp)
	movq	%rsi, %rdx
.Ltmp9374:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp9375:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_648:
	vmovaps	1056(%rsp), %ymm0
.Ltmp9376:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9377:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %ecx
	movl	%ecx, 1000(%rsp)
	movl	4(%rsp), %ecx
	movl	%ecx, 648(%rsp)
.Ltmp9378:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp9379:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r10, %rdi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9380:
.LBB34_629:
	.loc	5 569 13
	leaq	.Lalloc_fc26f793d85338b5649d38df0c19e7e0(%rip), %rcx
	movq	%rax, %rdi
	movq	24(%rsp), %rdx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9381:
.LBB34_630:
	.loc	5 569 13
	leaq	.Lalloc_fc26f793d85338b5649d38df0c19e7e0(%rip), %rcx
	movq	%rax, %rdi
.Ltmp9382:
	.loc	1 0 0 is_stmt 0
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_176:
	vmovaps	480(%rsp), %ymm0
.Ltmp9383:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9384:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9385:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9386:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9387:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9388:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9389:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9390:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9391:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_82ebe3a409d1fcceb2641bd874c3a328(%rip), %rcx
	movq	704(%rsp), %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9392:
.LBB34_387:
	.loc	5 569 13
	leaq	.Lalloc_82ebe3a409d1fcceb2641bd874c3a328(%rip), %rcx
	movq	416(%rsp), %rdi
	movq	%rax, %rdx
	movq	%rax, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9393:
.LBB34_301:
	.loc	5 0 13 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9394:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9395:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9396:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9397:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9398:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9399:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9400:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9401:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9402:
.LBB34_302:
	leaq	.Lalloc_913d17a5751fc2956adecdab98dac09f(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_627:
	vmovaps	480(%rsp), %ymm0
.Ltmp9403:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9404:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9405:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9406:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9407:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9408:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9409:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9410:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9411:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_8a0dcf875eae79f6708bcf79c3cbff55(%rip), %rcx
	movq	736(%rsp), %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9412:
.LBB34_144:
	.loc	5 0 13 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9413:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9414:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9415:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9416:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9417:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9418:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9419:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9420:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	jmp	.LBB34_145
.Ltmp9421:
.LBB34_237:
	.loc	1 0 0 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9422:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9423:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9424:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9425:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9426:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9427:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9428:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9429:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9430:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_77ae5a1ade955db4654bdc779dedbc90(%rip), %rcx
	movq	704(%rsp), %rdi
.Ltmp9431:
	.loc	1 0 0 is_stmt 0
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_396:
.Ltmp9432:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_77ae5a1ade955db4654bdc779dedbc90(%rip), %rcx
	movq	416(%rsp), %rdi
.Ltmp9433:
	.loc	1 0 0 is_stmt 0
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_394:
	vmovaps	480(%rsp), %ymm0
.Ltmp9434:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9435:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9436:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9437:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9438:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9439:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9440:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9441:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9442:
.LBB34_395:
	leaq	.Lalloc_913d17a5751fc2956adecdab98dac09f(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_358:
	vmovaps	480(%rsp), %ymm0
.Ltmp9443:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9444:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9445:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9446:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9447:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9448:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9449:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9450:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9451:
.LBB34_359:
	leaq	.Lalloc_8a0dcf875eae79f6708bcf79c3cbff55(%rip), %rcx
	movq	%r8, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_235:
	vmovaps	480(%rsp), %ymm0
.Ltmp9452:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9453:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9454:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9455:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9456:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9457:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9458:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9459:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
.LBB34_145:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9460:
.LBB34_146:
	leaq	.Lalloc_e5e0b8406fbb9ac3f5f26ac6469c25f7(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9461:
.LBB34_557:
	movb	$1, %r9b
	.loc	1 2217 12 is_stmt 1
	cmpb	$0, 2152(%r11)
	je	.LBB34_31
.Ltmp9462:
	.loc	1 663 31
	movq	1768(%r11), %rcx
	.loc	1 663 57 is_stmt 0
	movq	1832(%r11), %rax
.Ltmp9463:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp9464:
	.loc	14 304 12
	testq	%rcx, %rcx
	je	.LBB34_565
.Ltmp9465:
	.loc	14 0 12 is_stmt 0
	movq	1760(%r11), %rsi
	movq	1824(%r11), %rdi
	xorl	%r8d, %r8d
	jmp	.LBB34_560
.LBB34_563:
.Ltmp9466:
	.loc	1 665 22 is_stmt 1
	xorl	%edx, %edx
	divq	%r9
.LBB34_564:
	.loc	1 665 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp9467:
	.loc	1 0 0
	incq	%r8
.Ltmp9468:
	.loc	14 304 12 is_stmt 1
	addq	$12, %rdi
	cmpq	%r8, %rcx
	je	.LBB34_565
.Ltmp9469:
.LBB34_560:
	.loc	1 664 26
	movl	(%rdi), %r9d
	testq	%r9, %r9
	movq	16(%rsp), %rax
.Ltmp9470:
	.loc	1 665 42
	je	.LBB34_651
	.loc	1 665 24 is_stmt 0
	movl	(%rsi,%r8,4), %r10d
	.loc	1 665 42
	xorl	%edx, %edx
	divl	%r9d
	movl	%edx, %eax
	.loc	1 665 23
	addq	%r10, %rax
	.loc	1 665 22
	btq	$32, %rax
	jb	.LBB34_563
	xorl	%edx, %edx
	divl	%r9d
	jmp	.LBB34_564
.Ltmp9471:
.LBB34_565:
	.loc	1 663 31 is_stmt 1
	movq	1968(%r11), %rcx
	.loc	1 663 57 is_stmt 0
	movq	2032(%r11), %rax
.Ltmp9472:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp9473:
	.loc	14 304 12
	testq	%rcx, %rcx
	movq	16(%rsp), %rax
	je	.LBB34_572
.Ltmp9474:
	.loc	14 0 12 is_stmt 0
	movq	1960(%r11), %rsi
	movq	2024(%r11), %rdi
	xorl	%r8d, %r8d
	jmp	.LBB34_567
.LBB34_570:
.Ltmp9475:
	.loc	1 665 22 is_stmt 1
	movq	%rdx, %rax
	xorl	%edx, %edx
	divq	%r9
.LBB34_571:
	.loc	1 665 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp9476:
	.loc	1 0 0
	incq	%r8
.Ltmp9477:
	.loc	14 304 12 is_stmt 1
	addq	$12, %rdi
	cmpq	%r8, %rcx
	movq	16(%rsp), %rax
	je	.LBB34_572
.Ltmp9478:
.LBB34_567:
	.loc	1 664 26
	movl	(%rdi), %r9d
	testq	%r9, %r9
.Ltmp9479:
	.loc	1 665 42
	je	.LBB34_651
	.loc	1 665 24 is_stmt 0
	movl	(%rsi,%r8,4), %r10d
	.loc	1 665 42
	xorl	%edx, %edx
	divl	%r9d
	.loc	1 665 23
	addq	%r10, %rdx
	.loc	1 665 22
	btq	$32, %rdx
	jb	.LBB34_570
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%r9d
	jmp	.LBB34_571
.Ltmp9480:
.LBB34_572:
	.loc	1 2234 26 is_stmt 1
	movq	1632(%r11), %rsi
.Ltmp9481:
	.loc	1 455 44
	testq	%rsi, %rsi
	je	.LBB34_652
.Ltmp9482:
	.loc	1 2234 26
	movq	1624(%r11), %rcx
.Ltmp9483:
	.loc	1 455 23
	movl	1640(%r11), %edi
	.loc	1 455 44 is_stmt 0
	cmpq	%rsi, %rax
	jb	.LBB34_575
	.loc	1 0 44
	movq	16(%rsp), %rax
	.loc	1 455 44
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB34_575:
	.loc	1 455 22
	addq	%rdi, %rax
	.loc	1 455 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB34_576
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB34_578
.Ltmp9484:
.LBB34_626:
	.loc	1 0 21
	vmovaps	480(%rsp), %ymm0
.Ltmp9485:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9486:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9487:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9488:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9489:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9490:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9491:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9492:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9493:
	.loc	1 1293 45 is_stmt 1
	leaq	.Lalloc_b22b5c926aed02a79660ac772e5ad40d(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9494:
.LBB34_231:
	.loc	1 0 45 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9495:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9496:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9497:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9498:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9499:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9500:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9501:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9502:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9503:
.LBB34_232:
	leaq	.Lalloc_b22b5c926aed02a79660ac772e5ad40d(%rip), %rdx
	movq	%r14, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9504:
.LBB34_576:
	.loc	1 455 21 is_stmt 1
	xorl	%edx, %edx
	divl	%esi
.LBB34_578:
	.loc	1 455 9 is_stmt 0
	movl	%edx, 1640(%r11)
	.loc	1 456 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB34_653
	.loc	1 456 23 is_stmt 0
	movl	1644(%r11), %esi
	movq	16(%rsp), %rdx
	.loc	1 456 44
	cmpq	%rcx, %rdx
	jb	.LBB34_581
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ecx
.LBB34_581:
	.loc	1 456 22
	addq	%rsi, %rdx
	.loc	1 456 21
	movq	%rdx, %rax
	orq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB34_582
	movq	%rdx, %rax
	xorl	%edx, %edx
	divq	%rcx
	jmp	.LBB34_584
.LBB34_582:
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ecx
.LBB34_584:
	.loc	1 456 9
	movl	%edx, 1644(%r11)
	jmp	.LBB34_585
.Ltmp9505:
.LBB34_544:
	.loc	1 0 9
	vmovaps	1056(%rsp), %ymm0
.Ltmp9506:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	%ymm11, 5888(%rsp)
.Ltmp9507:
	vmovaps	%ymm10, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	%ymm6, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	%r10d, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
	jmp	.LBB34_545
.Ltmp9508:
.LBB34_490:
	vmovaps	1056(%rsp), %ymm0
.Ltmp9509:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9510:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	%r10d, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9511:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_642:
	vmovaps	1056(%rsp), %ymm0
.Ltmp9512:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9513:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9514:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r8, %rdi
	movq	%r11, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9515:
.LBB34_632:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9516:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9517:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9518:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9519:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9520:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
.Ltmp9521:
	.loc	1 853 9
	vmovaps	%ymm4, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9522:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 5664(%rsp)
	movl	128(%rsp), %eax
.Ltmp9523:
	movl	%eax, 1000(%rsp)
.Ltmp9524:
	vmovaps	%ymm11, 896(%rsp)
	movl	448(%rsp), %eax
.Ltmp9525:
	movl	%eax, 648(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9526:
	vmovaps	%ymm0, 544(%rsp)
.Ltmp9527:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r9, %rdi
	movq	%r12, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9528:
.LBB34_444:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9529:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9530:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9531:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9532:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9533:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9534:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9535:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 5664(%rsp)
.Ltmp9536:
	movl	%r13d, 1000(%rsp)
.Ltmp9537:
	vmovaps	%ymm11, 896(%rsp)
	jmp	.LBB34_445
.Ltmp9538:
.LBB34_643:
	vmovaps	1056(%rsp), %ymm0
.Ltmp9539:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9540:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9541:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%rdi), %rsi
	movq	%rcx, %rdx
.Ltmp9542:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9543:
.LBB34_633:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9544:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9545:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9546:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9547:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9548:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9549:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9550:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 5664(%rsp)
.Ltmp9551:
	movl	%r14d, 1000(%rsp)
.Ltmp9552:
	vmovaps	%ymm5, 896(%rsp)
	movl	448(%rsp), %eax
.Ltmp9553:
	movl	%eax, 648(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9554:
	vmovaps	%ymm0, 544(%rsp)
.Ltmp9555:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%rdi), %rsi
.Ltmp9556:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9557:
.LBB34_644:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9558:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9559:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9560:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r8, %rdi
	movq	%r11, %rsi
	movq	1664(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9561:
.LBB34_634:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9562:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9563:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9564:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9565:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9566:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9567:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9568:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 5664(%rsp)
.Ltmp9569:
	movl	%r14d, 1000(%rsp)
.Ltmp9570:
	vmovaps	%ymm5, 896(%rsp)
	movl	448(%rsp), %eax
.Ltmp9571:
	movl	%eax, 648(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9572:
	vmovaps	%ymm0, 544(%rsp)
.Ltmp9573:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r9, %rdi
	movq	%r12, %rsi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9574:
.LBB34_635:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9575:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9576:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9577:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9578:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9579:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9580:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9581:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	vmovaps	704(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 5664(%rsp)
.Ltmp9582:
	movl	%r14d, 1000(%rsp)
.Ltmp9583:
	vmovaps	%ymm13, 896(%rsp)
	movl	448(%rsp), %eax
.Ltmp9584:
	movl	%eax, 648(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9585:
	vmovaps	%ymm0, 544(%rsp)
.Ltmp9586:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	1600(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9587:
.LBB34_645:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9588:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9589:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9590:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	1632(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9591:
.LBB34_646:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9592:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9593:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %ecx
	movl	%ecx, 1000(%rsp)
	movl	4(%rsp), %ecx
	movl	%ecx, 648(%rsp)
.Ltmp9594:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r8, %rdi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9595:
.LBB34_636:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9596:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9597:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9598:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9599:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9600:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9601:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9602:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	vmovaps	704(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 5664(%rsp)
.Ltmp9603:
	movl	%r14d, 1000(%rsp)
.Ltmp9604:
	vmovaps	%ymm13, 896(%rsp)
	movl	448(%rsp), %ecx
.Ltmp9605:
	movl	%ecx, 648(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9606:
	vmovaps	%ymm0, 544(%rsp)
.Ltmp9607:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r12, %rdi
	movq	%rsi, %rdx
	movq	%rax, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9608:
.LBB34_647:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9609:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9610:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %ecx
	movl	%ecx, 1000(%rsp)
	movl	4(%rsp), %ecx
	movl	%ecx, 648(%rsp)
.Ltmp9611:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%r13), %rsi
.Ltmp9612:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r13, %rdi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9613:
.LBB34_637:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9614:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9615:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9616:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9617:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9618:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9619:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9620:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	vmovaps	704(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 5664(%rsp)
	movl	128(%rsp), %ecx
.Ltmp9621:
	movl	%ecx, 1000(%rsp)
.Ltmp9622:
	vmovaps	%ymm12, 896(%rsp)
	movl	448(%rsp), %ecx
.Ltmp9623:
	movl	%ecx, 648(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9624:
	vmovaps	%ymm0, 544(%rsp)
	movq	%rsi, %rdx
.Ltmp9625:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%r13), %rsi
.Ltmp9626:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r13, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9627:
.LBB34_638:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9628:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9629:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9630:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9631:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9632:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9633:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9634:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	vmovaps	704(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 5664(%rsp)
	movl	128(%rsp), %eax
.Ltmp9635:
	movl	%eax, 1000(%rsp)
.Ltmp9636:
	vmovaps	%ymm12, 896(%rsp)
	movl	448(%rsp), %eax
.Ltmp9637:
	movl	%eax, 648(%rsp)
.Ltmp9638:
	vmovaps	%ymm7, 544(%rsp)
.Ltmp9639:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp9640:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r10, %rdi
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9641:
.LBB34_538:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9642:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9643:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9644:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%r10), %rsi
	movq	%rcx, %rdx
.Ltmp9645:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_649:
	vmovaps	1056(%rsp), %ymm0
.Ltmp9646:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9647:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
	movq	%rcx, %rdx
.Ltmp9648:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	288(%rsp), %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9649:
.LBB34_639:
	.loc	5 0 13 is_stmt 0
	movq	%rax, %rsi
.Ltmp9650:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9651:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9652:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9653:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9654:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9655:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9656:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9657:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	vmovaps	704(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 5664(%rsp)
	movl	128(%rsp), %eax
.Ltmp9658:
	movl	%eax, 1000(%rsp)
.Ltmp9659:
	vmovaps	%ymm12, 896(%rsp)
	movl	448(%rsp), %eax
.Ltmp9660:
	movl	%eax, 648(%rsp)
.Ltmp9661:
	vmovaps	%ymm7, 544(%rsp)
.Ltmp9662:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r12, %rdi
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9663:
.LBB34_640:
	.loc	1 853 0
	vmovaps	%ymm0, 4832(%rsp)
.Ltmp9664:
	.loc	1 855 0
	vmovaps	%ymm1, 4736(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4800(%rsp)
.Ltmp9665:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm10, 4960(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9666:
	.loc	1 855 9
	vmovaps	%ymm0, 4864(%rsp)
	.loc	1 856 9
	vmovaps	%ymm15, 4928(%rsp)
	vmovaps	416(%rsp), %ymm0
.Ltmp9667:
	.loc	1 853 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	160(%rsp), %ymm0
.Ltmp9668:
	.loc	1 855 9
	vmovaps	%ymm0, 5472(%rsp)
	vmovaps	64(%rsp), %ymm0
	.loc	1 856 9
	vmovaps	%ymm0, 5536(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9669:
	.loc	1 853 9
	vmovaps	%ymm0, 5696(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9670:
	.loc	1 855 0
	vmovaps	%ymm0, 5600(%rsp)
	vmovaps	704(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 5664(%rsp)
	movl	128(%rsp), %eax
.Ltmp9671:
	movl	%eax, 1000(%rsp)
.Ltmp9672:
	vmovaps	%ymm13, 896(%rsp)
	movl	448(%rsp), %eax
.Ltmp9673:
	movl	%eax, 648(%rsp)
.Ltmp9674:
	vmovaps	%ymm7, 544(%rsp)
.Ltmp9675:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%rdx, %rsi
	movq	1536(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9676:
.LBB34_650:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9677:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9678:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9679:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	1568(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9680:
.LBB34_492:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9681:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9682:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	%r10d, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9683:
.LBB34_493:
	leaq	.Lalloc_fc26f793d85338b5649d38df0c19e7e0(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_97:
	vmovaps	1056(%rsp), %ymm0
.Ltmp9684:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9685:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	%r10d, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9686:
.LBB34_98:
	leaq	.Lalloc_fc26f793d85338b5649d38df0c19e7e0(%rip), %rcx
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_388:
.Ltmp9687:
	.loc	1 1275 22 is_stmt 1
	leaq	.Lalloc_d297cbdfce2474defb1d7f11394e8cb6(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9688:
.LBB34_623:
	.loc	1 0 22 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9689:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9690:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9691:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9692:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9693:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9694:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9695:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9696:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9697:
	.loc	1 1265 21 is_stmt 1
	leaq	.Lalloc_8013bf8450ffb218032f1e27d334efa7(%rip), %rdx
	movq	160(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9698:
.LBB34_119:
	.loc	1 0 21 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9699:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9700:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9701:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9702:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9703:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9704:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9705:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9706:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	jmp	.LBB34_120
.Ltmp9707:
.LBB34_624:
	.loc	1 0 0 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9708:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9709:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9710:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9711:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9712:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9713:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9714:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9715:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9716:
	.loc	1 1275 22 is_stmt 1
	leaq	.Lalloc_d297cbdfce2474defb1d7f11394e8cb6(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9717:
.LBB34_625:
	.loc	1 0 22 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9718:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9719:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9720:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9721:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9722:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9723:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9724:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9725:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9726:
	.loc	1 1276 24 is_stmt 1
	leaq	.Lalloc_ac016620dad255f0d323022a5b7f5883(%rip), %rdx
	movq	352(%rsp), %rdi
.Ltmp9727:
	.loc	1 0 0 is_stmt 0
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_194:
	vmovaps	480(%rsp), %ymm0
.Ltmp9728:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9729:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9730:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9731:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9732:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9733:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9734:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9735:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
.LBB34_120:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9736:
.LBB34_121:
	leaq	.Lalloc_2d2a28b8cb03afaaebfea48ffa77c925(%rip), %rdx
	movq	32(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_281:
	vmovaps	480(%rsp), %ymm0
.Ltmp9737:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9738:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9739:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9740:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9741:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9742:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9743:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9744:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9745:
.LBB34_282:
	leaq	.Lalloc_8013bf8450ffb218032f1e27d334efa7(%rip), %rdx
	movq	352(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_628:
	vmovaps	480(%rsp), %ymm0
.Ltmp9746:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9747:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9748:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9749:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9750:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9751:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9752:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9753:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9754:
	.loc	1 1275 22 is_stmt 1
	leaq	.Lalloc_d297cbdfce2474defb1d7f11394e8cb6(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9755:
.LBB34_284:
	.loc	1 0 22 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9756:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9757:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9758:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9759:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9760:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9761:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9762:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9763:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9764:
.LBB34_285:
	leaq	.Lalloc_ac016620dad255f0d323022a5b7f5883(%rip), %rdx
	movq	448(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_115:
	vmovaps	480(%rsp), %ymm0
.Ltmp9765:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9766:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9767:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9768:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9769:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9770:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9771:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9772:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	jmp	.LBB34_116
.Ltmp9773:
.LBB34_192:
	.loc	1 0 0 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9774:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9775:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9776:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9777:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9778:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9779:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9780:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9781:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
.LBB34_116:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9782:
.LBB34_117:
	leaq	.Lalloc_e5355958980a78279123102c3494b10a(%rip), %rdx
	movq	%r11, %rdi
	movq	32(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_506:
	vmovaps	1056(%rsp), %ymm0
.Ltmp9783:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9784:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	%ymm6, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9785:
	leaq	.Lalloc_1f23704f6ca4e855056ec66ed6f2587d(%rip), %rcx
	movq	24(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_433:
	movl	128(%rsp), %eax
	movl	%eax, 1000(%rsp)
	vmovaps	%ymm11, 896(%rsp)
	movl	448(%rsp), %eax
	movl	%eax, 648(%rsp)
	vmovaps	%ymm4, 544(%rsp)
	leaq	.Lalloc_1f23704f6ca4e855056ec66ed6f2587d(%rip), %rcx
	movq	24(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_631:
	movl	128(%rsp), %eax
	movl	%eax, 1000(%rsp)
	vmovaps	%ymm11, 896(%rsp)
	movl	448(%rsp), %eax
	movl	%eax, 648(%rsp)
	vmovaps	%ymm4, 544(%rsp)
	movq	1376(%rsp), %rsi
.Ltmp9786:
	.loc	1 2006 61 is_stmt 1
	shlq	$3, %rsi
.Ltmp9787:
	.loc	5 443 13
	leaq	.Lalloc_fa989b02c58b19a323a01f96ca250d1c(%rip), %rcx
	movl	$256, %edx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9788:
.LBB34_641:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9789:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9790:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	%ymm6, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
	movq	1248(%rsp), %rsi
.Ltmp9791:
	.loc	1 2006 61 is_stmt 1
	shlq	$3, %rsi
.Ltmp9792:
	.loc	5 443 13
	leaq	.Lalloc_fa989b02c58b19a323a01f96ca250d1c(%rip), %rcx
	movl	$256, %edx
	movq	1280(%rsp), %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9793:
.LBB34_114:
	.loc	5 0 13 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9794:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9795:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9796:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9797:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9798:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9799:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9800:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9801:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9802:
	.loc	1 1287 24 is_stmt 1
	leaq	.Lalloc_53d3a5c3ecf31ea5f14c0a7f8bf40921(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9803:
.LBB34_191:
	.loc	1 0 24 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9804:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9805:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9806:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9807:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9808:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9809:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9810:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9811:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9812:
.LBB34_280:
	leaq	.Lalloc_53d3a5c3ecf31ea5f14c0a7f8bf40921(%rip), %rdx
	movq	%r14, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9813:
.LBB34_654:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_84c34a80eea34d71b95e4ffce11a67a7(%rip), %rcx
	xorl	%edi, %edi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9814:
.LBB34_508:
	.loc	5 0 13 is_stmt 0
	vmovaps	1056(%rsp), %ymm0
.Ltmp9815:
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	1216(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	1120(%rsp), %ymm0
.Ltmp9816:
	vmovaps	%ymm0, 6560(%rsp)
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 6592(%rsp)
	vmovaps	%ymm6, 6624(%rsp)
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1184(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movl	672(%rsp), %eax
	movl	%eax, 1000(%rsp)
	movl	4(%rsp), %eax
	movl	%eax, 648(%rsp)
.Ltmp9817:
	leaq	.Lalloc_5a76e74e04cb182b892fe72abb75dc84(%rip), %rcx
	movq	248(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_436:
	movl	128(%rsp), %eax
	movl	%eax, 1000(%rsp)
	vmovaps	%ymm11, 896(%rsp)
	movl	448(%rsp), %eax
	movl	%eax, 648(%rsp)
	vmovaps	%ymm4, 544(%rsp)
	leaq	.Lalloc_5a76e74e04cb182b892fe72abb75dc84(%rip), %rcx
	movq	248(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9818:
.LBB34_622:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_76fe406493636db615b3ae498abf3aec(%rip), %rcx
	xorl	%edi, %edi
	movq	24(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9819:
.LBB34_173:
	.loc	5 0 13 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9820:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9821:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9822:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9823:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9824:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9825:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9826:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9827:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	jmp	.LBB34_174
.Ltmp9828:
.LBB34_236:
	.loc	1 0 0 is_stmt 0
	vmovaps	480(%rsp), %ymm0
.Ltmp9829:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9830:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9831:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9832:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9833:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9834:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9835:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9836:
	.loc	1 855 0
	vmovaps	%ymm15, 4032(%rsp)
.LBB34_174:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9837:
.LBB34_175:
	leaq	.Lalloc_7d7f2b4ff3f37cf08b84adcf6762221c(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9838:
.LBB34_655:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_1f13fdd6e9523d10fc4ba543bd6c2386(%rip), %rcx
.Ltmp9839:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9840:
.LBB34_129:
	.loc	5 0 13
	movl	$1, %eax
	jmp	.LBB34_130
.LBB34_294:
	movl	$1, %eax
	jmp	.LBB34_295
.LBB34_35:
.Ltmp9841:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_a95969ba91c39576d44da837974b8fd5(%rip), %rcx
.Ltmp9842:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9843:
.LBB34_364:
	.loc	5 0 13
	movl	$2, %eax
	jmp	.LBB34_295
.LBB34_150:
	movl	$2, %eax
	jmp	.LBB34_130
.LBB34_154:
	movl	$3, %eax
	jmp	.LBB34_130
.LBB34_368:
	movl	$3, %eax
	jmp	.LBB34_295
.LBB34_372:
	movl	$4, %eax
	jmp	.LBB34_295
.LBB34_158:
	movl	$4, %eax
	jmp	.LBB34_130
.LBB34_376:
	movl	$5, %eax
	jmp	.LBB34_295
.LBB34_162:
	movl	$5, %eax
	jmp	.LBB34_130
.LBB34_380:
	movl	$6, %eax
	jmp	.LBB34_295
.LBB34_166:
	movl	$6, %eax
	jmp	.LBB34_130
.LBB34_384:
	movl	$7, %eax
.LBB34_295:
	movq	%rax, 864(%rsp)
.LBB34_296:
.Ltmp9844:
	.loc	1 1405 42 is_stmt 1
	leaq	.Lalloc_7ff2ed40a8d2df224a47a35441e5e2fe(%rip), %rdx
	movq	864(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9845:
.LBB34_170:
	.loc	1 0 42 is_stmt 0
	movl	$7, %eax
.LBB34_130:
	movq	%rax, 832(%rsp)
.LBB34_131:
	vmovaps	480(%rsp), %ymm0
.Ltmp9846:
	.loc	1 853 9 is_stmt 1
	vmovaps	%ymm0, 3264(%rsp)
.Ltmp9847:
	.loc	1 855 9
	vmovaps	%ymm5, 3168(%rsp)
	.loc	1 856 9
	vmovaps	%ymm6, 3232(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9848:
	.loc	1 853 9
	vmovaps	%ymm0, 3392(%rsp)
	vmovaps	64(%rsp), %ymm0
.Ltmp9849:
	.loc	1 855 9
	vmovaps	%ymm0, 3296(%rsp)
	.loc	1 856 9
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp9850:
	.loc	1 853 9
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9851:
	.loc	1 855 9
	vmovaps	%ymm0, 3904(%rsp)
	.loc	1 856 9
	vmovaps	%ymm11, 3968(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9852:
	.loc	1 853 9
	vmovaps	%ymm0, 4128(%rsp)
.Ltmp9853:
	.loc	1 855 0
	vmovaps	%ymm2, 4032(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4096(%rsp)
.Ltmp9854:
	.loc	1 1405 42 is_stmt 1
	leaq	.Lalloc_7ff2ed40a8d2df224a47a35441e5e2fe(%rip), %rdx
	movq	832(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9855:
.LBB34_651:
	.loc	1 665 42
	leaq	.Lalloc_bac57976a2bdbfad4a3a85d5d1c7648c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp9856:
.LBB34_652:
	.loc	1 455 44
	leaq	.Lalloc_f0ee36f67d9a332211aa5518dd2ebfd5(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB34_653:
	.loc	1 456 44
	leaq	.Lalloc_33d4d33e0a850133578789055882dcf9(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp9857:
.Lfunc_end34:
	.size	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_, .Lfunc_end34-_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_
