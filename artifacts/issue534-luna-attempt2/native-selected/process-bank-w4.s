_RNvXsd_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank:
.Lfunc_begin40:
	.loc	6 1077 0
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
	subq	$1192, %rsp
	.cfi_def_cfa_offset 1248
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	.loc	6 1078 29 prologue_end
	movzbl	1824(%rsi), %eax
.Ltmp7152:
	.loc	18 966 15
	cmpb	$2, %al
	.loc	18 966 9 is_stmt 0
	je	.LBB40_272
.Ltmp7153:
	.loc	20 1032 9 is_stmt 1
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqu	%ymm0, 1152(%rsp)
	vmovdqu	%ymm0, 1120(%rsp)
	vmovdqu	%ymm0, 1088(%rsp)
	vmovdqu	%ymm0, 1056(%rsp)
	vmovdqu	%ymm0, 1024(%rsp)
	vmovdqu	%ymm0, 992(%rsp)
	vmovdqu	%ymm0, 960(%rsp)
	vmovdqu	%ymm0, 928(%rsp)
	vmovdqu	%ymm0, 896(%rsp)
	vmovdqu	%ymm0, 864(%rsp)
.Ltmp7154:
	.loc	20 186 45
	cmpb	%al, 108(%rdx)
.Ltmp7155:
	.loc	6 1083 20
	jne	.LBB40_3
	cmpq	$0, 64(%rdx)
	jne	.LBB40_3
	.loc	6 0 20 is_stmt 0
	movq	%rsi, %r10
	movb	%al, 71(%rsp)
	movq	%rdi, 520(%rsp)
	.loc	6 1088 35 is_stmt 1
	vmovdqu	%ymm0, 832(%rsp)
	vmovdqu	%ymm0, 800(%rsp)
	vmovdqu	%ymm0, 768(%rsp)
	vmovdqu	%ymm0, 736(%rsp)
	vmovdqu	%ymm0, 704(%rsp)
	vmovdqu	%ymm0, 672(%rsp)
	vmovdqu	%ymm0, 640(%rsp)
	vmovdqu	%ymm0, 608(%rsp)
	vmovdqu	%ymm0, 576(%rsp)
	vmovdqu	%ymm0, 544(%rsp)
	movq	48(%rdx), %rax
	movq	%rax, 352(%rsp)
	movq	56(%rdx), %r9
	movq	32(%rdx), %rax
	movq	%rax, 208(%rsp)
	movq	40(%rdx), %rax
	movq	%rax, 72(%rsp)
	movq	%rdx, 160(%rsp)
	movq	96(%rdx), %rax
	movq	%rax, 40(%rsp)
	leaq	1088(%rsi), %rax
	movq	%rax, 536(%rsp)
.Ltmp7156:
	.loc	11 900 12
	cmpq	$1, %r9
	movq	%r9, %rax
	adcq	$-1, %rax
	movq	%rax, 288(%rsp)
	leaq	1328(%rsi), %rax
	movq	%rax, 256(%rsp)
	xorl	%ebp, %ebp
	vmovss	.LCPI40_0(%rip), %xmm5
	vmovss	.LCPI40_1(%rip), %xmm6
	movq	%rsi, 48(%rsp)
	movq	%r9, 192(%rsp)
	jmp	.LBB40_215
	.loc	11 0 12 is_stmt 0
.Ltmp7157:
	.p2align	4
.LBB40_5:
	movq	56(%rsp), %rax
	movq	%rax, %rbp
.Ltmp7158:
	.loc	8 1916 50 is_stmt 1
	cmpq	$4, %rax
.Ltmp7159:
	.loc	11 900 12
	je	.LBB40_6
.Ltmp7160:
.LBB40_215:
	.loc	6 1090 33
	cmpq	%r9, %rbp
	je	.LBB40_288
.Ltmp7161:
	.loc	6 0 0 is_stmt 0
	leaq	1(%rbp), %rcx
.Ltmp7162:
	.loc	6 1091 31 is_stmt 1
	cmpq	288(%rsp), %rbp
	je	.LBB40_289
	.loc	6 0 31 is_stmt 0
	movq	352(%rsp), %rax
	movl	(%rax,%rbp,4), %edi
	.loc	6 1091 31
	movl	(%rax,%rcx,4), %esi
.Ltmp7163:
	.loc	15 1050 16 is_stmt 1
	cmpl	%edi, %esi
	jb	.LBB40_290
	cmpq	%rsi, 72(%rsp)
	jb	.LBB40_290
.Ltmp7164:
	.loc	15 0 16 is_stmt 0
	movq	%rcx, 56(%rsp)
	movl	$0, 424(%rsp)
	movl	$0, 432(%rsp)
	movl	$0, 440(%rsp)
	movl	$0, 448(%rsp)
	movl	$0, 456(%rsp)
	movl	$0, 464(%rsp)
	movl	$0, 472(%rsp)
	movl	$0, 480(%rsp)
.Ltmp7165:
	.loc	38 1714 9 is_stmt 1
	cmpl	%edi, %esi
.Ltmp7166:
	.loc	19 180 28
	jne	.LBB40_220
.Ltmp7167:
.LBB40_271:
	.loc	19 0 28 is_stmt 0
	movq	256(%rsp), %rax
	xorl	%ecx, %ecx
.Ltmp7168:
	.loc	19 180 28
	jmp	.LBB40_224
.Ltmp7169:
	.loc	19 0 28
.Ltmp7170:
	.p2align	4
.LBB40_255:
	.loc	6 399 5 is_stmt 1
	vmovaps	-48(%rax), %xmm4
	vmovaps	%xmm4, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, -48(%rax)
.Ltmp7171:
	.loc	6 399 5
	vmovaps	-32(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	vmovss	%xmm0, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -32(%rax)
.Ltmp7172:
	.loc	6 399 5
	vmovaps	-16(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm2, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -16(%rax)
.Ltmp7173:
	.loc	6 399 5
	vmovaps	(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm3, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovdqa	(%rsp), %xmm0
	vmovdqa	%xmm0, (%rax)
.Ltmp7174:
	.loc	6 664 17
	movl	$64, 1820(%r10)
.Ltmp7175:
.LBB40_223:
	.loc	6 0 0 is_stmt 0
	addq	$32, %rcx
.Ltmp7176:
	.loc	38 1714 9 is_stmt 1
	addq	$304, %rax
	cmpq	$64, %rcx
.Ltmp7177:
	.loc	19 180 28
	je	.LBB40_5
.Ltmp7178:
.LBB40_224:
	.loc	6 651 21
	cmpl	$1, 424(%rsp,%rcx)
	je	.LBB40_225
	cmpl	$1, 432(%rsp,%rcx)
	je	.LBB40_233
.LBB40_240:
	cmpl	$1, 440(%rsp,%rcx)
	je	.LBB40_241
.LBB40_248:
	cmpl	$1, 448(%rsp,%rcx)
	jne	.LBB40_223
	jmp	.LBB40_249
	.loc	6 0 21 is_stmt 0
.Ltmp7179:
	.p2align	4
.LBB40_225:
	.loc	6 651 0
	leaq	-240(%rax), %rdx
	.loc	6 651 26
	vmovd	428(%rsp,%rcx), %xmm0
.Ltmp7180:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rdx), %xmm1
	vmovaps	%xmm1, (%rsp)
.Ltmp7181:
	.loc	6 393 5
	vmovd	(%rsp,%rbp,4), %xmm1
	vmovd	%xmm1, %esi
.Ltmp7182:
	.loc	7 1244 18
	vmovd	%xmm0, %edi
.Ltmp7183:
	.loc	47 112 9
	cmpl	%edi, %esi
	setne	%dil
	movl	%esi, %r8d
	negl	%r8d
	seto	%r8b
	orb	%dil, %r8b
	andl	$2147483647, %esi
	cmpl	$2139095040, %esi
	setge	%sil
	orb	%r8b, %sil
	vsubss	%xmm1, %xmm0, %xmm2
	vmulss	%xmm5, %xmm2, %xmm2
	testb	%sil, %sil
	je	.LBB40_226
	je	.LBB40_228
.LBB40_229:
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_231
.Ltmp7184:
.LBB40_230:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB40_231:
.Ltmp7185:
	.loc	6 399 5 is_stmt 1
	vmovaps	(%rdx), %xmm4
	vmovaps	%xmm4, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, (%rdx)
.Ltmp7186:
	.loc	6 399 5
	vmovaps	-224(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	vmovss	%xmm0, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -224(%rax)
.Ltmp7187:
	.loc	6 399 5
	vmovaps	-208(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm2, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -208(%rax)
.Ltmp7188:
	.loc	6 399 5
	vmovaps	-192(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm3, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovdqa	(%rsp), %xmm0
	vmovdqa	%xmm0, -192(%rax)
.Ltmp7189:
	.loc	6 664 17
	movl	$64, 1820(%r10)
.Ltmp7190:
	.loc	6 651 21
	cmpl	$1, 432(%rsp,%rcx)
	jne	.LBB40_240
.LBB40_233:
	.loc	6 651 26 is_stmt 0
	vmovd	436(%rsp,%rcx), %xmm0
.Ltmp7191:
	.loc	1 551 14 is_stmt 1
	vmovups	-176(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
.Ltmp7192:
	.loc	6 393 5
	vmovd	(%rsp,%rbp,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp7193:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp7194:
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
	je	.LBB40_234
	je	.LBB40_236
.LBB40_237:
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_239
.Ltmp7195:
.LBB40_238:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB40_239:
.Ltmp7196:
	.loc	6 399 5 is_stmt 1
	vmovaps	-176(%rax), %xmm4
	vmovaps	%xmm4, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, -176(%rax)
.Ltmp7197:
	.loc	6 399 5
	vmovaps	-160(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	vmovss	%xmm0, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -160(%rax)
.Ltmp7198:
	.loc	6 399 5
	vmovaps	-144(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm2, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -144(%rax)
.Ltmp7199:
	.loc	6 399 5
	vmovaps	-128(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm3, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovdqa	(%rsp), %xmm0
	vmovdqa	%xmm0, -128(%rax)
.Ltmp7200:
	.loc	6 664 17
	movl	$64, 1820(%r10)
.Ltmp7201:
	.loc	6 651 21
	cmpl	$1, 440(%rsp,%rcx)
	jne	.LBB40_248
.LBB40_241:
	.loc	6 651 26 is_stmt 0
	vmovd	444(%rsp,%rcx), %xmm0
.Ltmp7202:
	.loc	1 551 14 is_stmt 1
	vmovups	-112(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
.Ltmp7203:
	.loc	6 393 5
	vmovd	(%rsp,%rbp,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp7204:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp7205:
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
	je	.LBB40_242
	je	.LBB40_244
.LBB40_245:
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_247
.Ltmp7206:
.LBB40_246:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB40_247:
.Ltmp7207:
	.loc	6 399 5 is_stmt 1
	vmovaps	-112(%rax), %xmm4
	vmovaps	%xmm4, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, -112(%rax)
.Ltmp7208:
	.loc	6 399 5
	vmovaps	-96(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	vmovss	%xmm0, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -96(%rax)
.Ltmp7209:
	.loc	6 399 5
	vmovaps	-80(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm2, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -80(%rax)
.Ltmp7210:
	.loc	6 399 5
	vmovaps	-64(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm3, (%rsp,%rbp,4)
	.loc	6 401 5
	vmovdqa	(%rsp), %xmm0
	vmovdqa	%xmm0, -64(%rax)
.Ltmp7211:
	.loc	6 664 17
	movl	$64, 1820(%r10)
.Ltmp7212:
	.loc	6 651 21
	cmpl	$1, 448(%rsp,%rcx)
	jne	.LBB40_223
.LBB40_249:
	.loc	6 651 26 is_stmt 0
	vmovd	452(%rsp,%rcx), %xmm0
.Ltmp7213:
	.loc	1 551 14 is_stmt 1
	vmovups	-48(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
.Ltmp7214:
	.loc	6 393 5
	vmovd	(%rsp,%rbp,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp7215:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp7216:
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
	je	.LBB40_250
	je	.LBB40_252
.LBB40_253:
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_255
	jmp	.LBB40_254
	.loc	47 0 9
.Ltmp7217:
	.p2align	4
.LBB40_226:
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	jne	.LBB40_229
.LBB40_228:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB40_230
	jmp	.LBB40_231
	.loc	47 0 9
.Ltmp7218:
	.p2align	4
.LBB40_234:
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	jne	.LBB40_237
.LBB40_236:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB40_238
	jmp	.LBB40_239
	.loc	47 0 9
.Ltmp7219:
	.p2align	4
.LBB40_242:
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	jne	.LBB40_245
.LBB40_244:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB40_246
	jmp	.LBB40_247
	.loc	47 0 9
.Ltmp7220:
	.p2align	4
.LBB40_250:
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	jne	.LBB40_253
.LBB40_252:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_255
.Ltmp7221:
.LBB40_254:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
	jmp	.LBB40_255
	.p2align	4
.LBB40_220:
	subq	%rdi, %rsi
	leaq	(%rdi,%rdi,4), %rax
	movq	208(%rsp), %rcx
	leaq	(%rcx,%rax,8), %r14
	leaq	(%rsi,%rsi,4), %rax
	leaq	(%r14,%rax,8), %rdx
	.loc	6 1096 25 is_stmt 1
	leaq	(%rbp,%rbp,4), %rax
	leaq	544(%rsp,%rax,8), %rsi
	movl	1796(%r10), %edi
	movq	560(%rsp,%rax,8), %r13
	movb	$1, %r8b
	xorl	%r12d, %r12d
	.loc	6 0 25 is_stmt 0
.Ltmp7222:
	.p2align	4
.LBB40_221:
.Ltmp7223:
	.loc	6 616 33 is_stmt 1
	movl	32(%r14), %eax
	.loc	6 616 27 is_stmt 0
	cmpl	$1, %eax
	je	.LBB40_222
	cmpl	$2, %eax
	jne	.LBB40_269
	.loc	6 0 27
	movl	$1, %eax
	leaq	456(%rsp), %rbx
.Ltmp7224:
	.loc	6 624 35 is_stmt 1
	movl	16(%r14), %r15d
.Ltmp7225:
	.loc	15 3178 26
	testl	%r15d, %r15d
.Ltmp7226:
	.loc	46 459 8
	jns	.LBB40_258
	jmp	.LBB40_269
.Ltmp7227:
	.loc	46 0 8 is_stmt 0
.Ltmp7228:
	.p2align	4
.LBB40_222:
	xorl	%eax, %eax
	leaq	424(%rsp), %rbx
	.loc	6 624 35 is_stmt 1
	movl	16(%r14), %r15d
.Ltmp7229:
	.loc	15 3178 26
	testl	%r15d, %r15d
.Ltmp7230:
	.loc	46 459 8
	js	.LBB40_269
.Ltmp7231:
.LBB40_258:
	.loc	6 633 25
	cmpq	%rdi, %r12
	jae	.LBB40_269
	cmpl	$3, %r15d
	ja	.LBB40_269
	.loc	6 635 20
	cmpl	$1, 28(%r14)
	jne	.LBB40_269
	.loc	6 0 20 is_stmt 0
	movq	40(%rsp), %rcx
	.loc	6 636 20 is_stmt 1
	cmpq	%rcx, (%r14)
	jne	.LBB40_269
	.loc	6 0 20 is_stmt 0
	movq	40(%rsp), %rcx
	.loc	6 637 20 is_stmt 1
	cmpq	%rcx, 8(%r14)
	jne	.LBB40_269
	.loc	6 638 20
	vmovd	20(%r14), %xmm0
.Ltmp7232:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp7233:
	.loc	6 638 20
	cmpl	%ecx, 24(%r14)
	jne	.LBB40_269
.Ltmp7234:
	.loc	6 0 0 is_stmt 0
	leal	(%rax,%r15,2), %eax
	movl	%eax, 112(%rsp)
.Ltmp7235:
	.loc	6 639 42 is_stmt 1
	leaq	(%r15,%r15,4), %rax
	leaq	.Lalloc_d0058eaee52f1ba8b2e5577cef0c3c7f(%rip), %rcx
	movq	%rdi, 128(%rsp)
	leaq	(%rcx,%rax,8), %rdi
	movq	%rdx, 240(%rsp)
	movq	%rsi, 144(%rsp)
	movl	%r8d, 224(%rsp)
	vmovdqa	%xmm0, 400(%rsp)
	.loc	6 639 20 is_stmt 0
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	400(%rsp), %xmm1
	movl	224(%rsp), %r8d
	movq	128(%rsp), %rdi
	movq	144(%rsp), %rsi
	movq	240(%rsp), %rdx
	vmovss	.LCPI40_1(%rip), %xmm6
	vmovss	.LCPI40_0(%rip), %xmm5
	movq	192(%rsp), %r9
	movq	48(%rsp), %r10
	movl	112(%rsp), %ecx
	movl	%ecx, %r11d
	cmpl	176(%rsp), %ecx
	seta	%cl
	testb	%al, %al
	je	.LBB40_269
	orb	%r8b, %cl
	testb	$1, %cl
	je	.LBB40_269
.Ltmp7236:
	.loc	6 642 17 is_stmt 1
	cmpb	$0, (%rbx,%r15,8)
	je	.LBB40_267
.Ltmp7237:
	.loc	6 0 17 is_stmt 0
.Ltmp7238:
	.p2align	4
.LBB40_269:
	addq	$40, %r14
.Ltmp7239:
	.loc	15 2428 13 is_stmt 1
	incq	%r13
	movq	$-1, %rax
	cmoveq	%rax, %r13
.Ltmp7240:
	.loc	6 0 0 is_stmt 0
	movq	%r13, 16(%rsi)
.Ltmp7241:
	.loc	34 82 9 is_stmt 1
	incq	%r12
.Ltmp7242:
	.loc	38 1714 9
	cmpq	%rdx, %r14
.Ltmp7243:
	.loc	19 180 28
	jne	.LBB40_221
	jmp	.LBB40_271
.Ltmp7244:
.LBB40_267:
	.loc	12 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
.Ltmp7245:
	.loc	6 647 13
	movl	$1, (%rbx,%r15,8)
	vmovss	%xmm0, 4(%rbx,%r15,8)
.Ltmp7246:
	.loc	38 1714 9
	addq	$40, %r14
.Ltmp7247:
	.loc	19 180 28
	incq	%r12
	xorl	%r8d, %r8d
	movl	%r11d, 176(%rsp)
.Ltmp7248:
	.loc	38 1714 9
	cmpq	%rdx, %r14
.Ltmp7249:
	.loc	19 180 28
	jne	.LBB40_221
	jmp	.LBB40_271
.Ltmp7250:
.LBB40_3:
	.loc	6 1084 28
	vmovdqu	%ymm0, 288(%rdi)
	vmovdqu	%ymm0, 256(%rdi)
	vmovdqu	%ymm0, 224(%rdi)
	vmovdqu	%ymm0, 192(%rdi)
	vmovdqu	%ymm0, 160(%rdi)
	vmovdqu	%ymm0, 128(%rdi)
	vmovdqu	%ymm0, 96(%rdi)
	vmovdqu	%ymm0, 64(%rdi)
	vmovdqu	%ymm0, 32(%rdi)
	vmovdqu	%ymm0, (%rdi)
	movb	%al, 320(%rdi)
	jmp	.LBB40_4
.LBB40_6:
	.loc	6 0 28 is_stmt 0
	movq	160(%rsp), %rsi
.Ltmp7251:
	.loc	6 1099 30 is_stmt 1
	movl	104(%rsi), %ecx
.Ltmp7252:
	.loc	6 1100 32
	movq	(%rsi), %rax
	movq	%rax, 376(%rsp)
	movq	8(%rsi), %rdx
	.loc	6 1100 44 is_stmt 0
	movq	16(%rsi), %rax
	movq	%rax, 368(%rsp)
	movq	24(%rsi), %r8
.Ltmp7253:
	.loc	6 685 25 is_stmt 1
	movl	1820(%r10), %edi
.Ltmp7254:
	.loc	8 1078 5
	cmpl	%edi, %ecx
	movl	%edi, %r11d
	cmovbl	%ecx, %r11d
.Ltmp7255:
	.loc	6 686 12
	testl	%r11d, %r11d
	movq	%rcx, 400(%rsp)
	movq	%r11, 280(%rsp)
	movq	%rdx, 496(%rsp)
	movq	%r8, 488(%rsp)
	je	.LBB40_7
	.loc	6 687 25
	leaq	(,%r11,4), %rsi
	cmpq	%rdx, %rsi
.Ltmp7256:
	.loc	15 1050 16
	ja	.LBB40_273
.Ltmp7257:
	.loc	25 451 16
	cmpq	%r8, %rsi
	ja	.LBB40_274
.Ltmp7258:
	.loc	21 61 8
	cmpl	$1, 1772(%r10)
	movl	%edi, 388(%rsp)
	movq	%rsi, 512(%rsp)
	jne	.LBB40_20
	.loc	21 0 8 is_stmt 0
	xorl	%r14d, %r14d
	.loc	21 61 8
	jmp	.LBB40_24
.LBB40_20:
.Ltmp7259:
	.file	48 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/iter/traits/iterator.rs"
	.loc	48 2494 21 is_stmt 1
	movl	800(%r10), %eax
	movb	$2, %r14b
.Ltmp7260:
	.loc	8 1878 54
	cmpl	864(%r10), %eax
.Ltmp7261:
	.loc	48 2494 21
	jne	.LBB40_24
	movl	804(%r10), %eax
.Ltmp7262:
	.loc	8 1878 54
	cmpl	868(%r10), %eax
.Ltmp7263:
	.loc	48 2494 21
	jne	.LBB40_24
	movl	808(%r10), %eax
.Ltmp7264:
	.loc	8 1878 54
	cmpl	872(%r10), %eax
.Ltmp7265:
	.loc	48 2494 21
	jne	.LBB40_24
	movl	812(%r10), %eax
.Ltmp7266:
	.loc	8 1878 54
	cmpl	876(%r10), %eax
	sete	%al
	movb	$2, %r14b
.Ltmp7267:
	.loc	48 2494 21
	subb	%al, %r14b
.Ltmp7268:
.LBB40_24:
	.loc	6 736 31
	movq	768(%r10), %r8
	movq	776(%r10), %rdx
	.loc	6 741 31
	movq	832(%r10), %rbx
	movq	840(%r10), %r15
	.loc	6 747 28
	movl	1812(%r10), %ebp
.Ltmp7269:
	.loc	21 353 16
	movl	1808(%r10), %r13d
.Ltmp7270:
	.loc	11 900 12
	movl	%r13d, %eax
	subl	1816(%r10), %eax
	movq	%rax, 528(%rsp)
	xorl	%eax, %eax
	movzbl	%r14b, %ecx
	movl	%ecx, 392(%rsp)
	xorl	%r9d, %r9d
	movq	%rdx, 80(%rsp)
	movq	%r15, 40(%rsp)
	movl	%r14d, 396(%rsp)
	movq	%r13, 504(%rsp)
	jmp	.LBB40_25
.Ltmp7271:
	.loc	11 0 12 is_stmt 0
.Ltmp7272:
	.p2align	4
.LBB40_100:
	vmovss	(%r8,%r12,4), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	(%rbx,%r12,4), %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vmovss	(%rbx,%rcx,4), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	(%r8,%rcx,4), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	(%r8,%r11,4), %xmm0
	vmovaps	%xmm0, 96(%rsp)
	vmovss	(%rbx,%r11,4), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	(%rbx,%r14,4), %xmm0
	vmovaps	%xmm0, 112(%rsp)
	vmovss	(%r8,%r14,4), %xmm0
	vmovaps	%xmm0, 128(%rsp)
	vmovss	(%r8,%r13,4), %xmm0
	vmovaps	%xmm0, 336(%rsp)
	vmovss	(%rbx,%r13,4), %xmm6
	vmovss	(%rbx,%rsi,4), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovss	(%r8,%rsi,4), %xmm0
	vmovaps	%xmm0, 208(%rsp)
.Ltmp7273:
	vmovss	(%r8,%r10,4), %xmm0
	vmovaps	%xmm0, 320(%rsp)
	vmovss	(%rbx,%r10,4), %xmm7
	.loc	21 268 33 is_stmt 1
	vmovss	(%rbx,%rdi,4), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	.loc	21 269 33
	vmovss	(%r8,%rdi,4), %xmm0
	vmovaps	%xmm0, 192(%rsp)
.Ltmp7274:
.LBB40_101:
	.loc	21 0 33 is_stmt 0
	movq	48(%rsp), %r10
.Ltmp7275:
	.loc	21 441 27 is_stmt 1
	vmovaps	1088(%r10), %xmm1
.Ltmp7276:
	.loc	21 439 26
	vmovaps	1136(%r10), %xmm2
.Ltmp7277:
	.file	49 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs"
	.loc	49 36 14
	vaddps	1120(%r10), %xmm1, %xmm8
	vbroadcastss	.LCPI40_2(%rip), %xmm11
.Ltmp7278:
	.loc	49 504 14
	vcmpeqps	%xmm2, %xmm11, %xmm10
.Ltmp7279:
	.file	50 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/sse41.rs"
	.loc	50 131 19
	vblendvps	%xmm10, 1104(%r10), %xmm8, %xmm8
.Ltmp7280:
	.loc	21 441 27
	vmovaps	1152(%r10), %xmm4
.Ltmp7281:
	.loc	21 439 26
	vmovaps	1200(%r10), %xmm3
	vxorps	%xmm12, %xmm12, %xmm12
.Ltmp7282:
	.loc	49 544 14
	vcmpltps	%xmm2, %xmm12, %xmm9
.Ltmp7283:
	.loc	21 442 13
	vmaskmovps	%xmm8, %xmm9, 1088(%r10)
	.loc	21 447 13
	vmaskmovps	%xmm12, %xmm10, 1120(%r10)
	vbroadcastss	.LCPI40_3(%rip), %xmm5
.Ltmp7284:
	.loc	49 62 14
	vaddps	%xmm5, %xmm2, %xmm2
.Ltmp7285:
	.loc	21 448 13
	vmaskmovps	%xmm2, %xmm9, 1136(%r10)
	vmovaps	%xmm11, %xmm0
.Ltmp7286:
	.loc	49 504 14
	vcmpeqps	%xmm3, %xmm11, %xmm2
.Ltmp7287:
	.loc	49 36 14
	vaddps	1184(%r10), %xmm4, %xmm10
.Ltmp7288:
	.loc	50 131 19
	vblendvps	%xmm2, 1168(%r10), %xmm10, %xmm11
.Ltmp7289:
	.loc	49 544 14
	vcmpltps	%xmm3, %xmm12, %xmm15
.Ltmp7290:
	.loc	21 442 13
	vmaskmovps	%xmm11, %xmm15, 1152(%r10)
	.loc	21 447 13
	vmaskmovps	%xmm12, %xmm2, 1184(%r10)
.Ltmp7291:
	.loc	49 62 14
	vaddps	%xmm5, %xmm3, %xmm2
.Ltmp7292:
	.loc	21 448 13
	vmaskmovps	%xmm2, %xmm15, 1200(%r10)
.Ltmp7293:
	.loc	21 439 26
	vmovaps	1264(%r10), %xmm2
.Ltmp7294:
	.loc	49 504 14
	vcmpeqps	%xmm0, %xmm2, %xmm3
.Ltmp7295:
	.loc	21 441 27
	vmovaps	1216(%r10), %xmm14
.Ltmp7296:
	.loc	49 36 14
	vaddps	1248(%r10), %xmm14, %xmm10
.Ltmp7297:
	.loc	50 131 19
	vblendvps	%xmm3, 1232(%r10), %xmm10, %xmm13
	vxorps	%xmm10, %xmm10, %xmm10
.Ltmp7298:
	.loc	49 544 14
	vcmpltps	%xmm2, %xmm10, %xmm12
.Ltmp7299:
	.loc	21 442 13
	vmaskmovps	%xmm13, %xmm12, 1216(%r10)
	.loc	21 447 13
	vmaskmovps	%xmm10, %xmm3, 1248(%r10)
.Ltmp7300:
	.loc	50 131 19
	vblendvps	%xmm9, %xmm8, %xmm1, %xmm10
.Ltmp7301:
	.loc	49 62 14
	vaddps	%xmm5, %xmm2, %xmm1
.Ltmp7302:
	.loc	21 448 13
	vmaskmovps	%xmm1, %xmm12, 1264(%r10)
.Ltmp7303:
	.loc	50 131 19
	vblendvps	%xmm15, %xmm11, %xmm4, %xmm1
.Ltmp7304:
	.loc	21 441 27
	vmovaps	1280(%r10), %xmm3
.Ltmp7305:
	.loc	21 439 26
	vmovaps	1328(%r10), %xmm4
.Ltmp7306:
	.loc	49 504 14
	vcmpeqps	%xmm0, %xmm4, %xmm2
.Ltmp7307:
	.loc	49 36 14
	vaddps	1312(%r10), %xmm3, %xmm8
.Ltmp7308:
	.loc	50 131 19
	vblendvps	%xmm2, 1296(%r10), %xmm8, %xmm8
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp7309:
	.loc	49 544 14
	vcmpltps	%xmm4, %xmm0, %xmm9
.Ltmp7310:
	.loc	21 442 13
	vmaskmovps	%xmm8, %xmm9, 1280(%r10)
	.loc	21 447 13
	vmaskmovps	%xmm0, %xmm2, 1312(%r10)
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp7311:
	.loc	50 131 19
	vblendvps	%xmm12, %xmm13, %xmm14, %xmm12
.Ltmp7312:
	.loc	49 62 14
	vaddps	%xmm5, %xmm4, %xmm4
	vmovaps	%xmm5, %xmm11
.Ltmp7313:
	.loc	21 448 13
	vmaskmovps	%xmm4, %xmm9, 1328(%r10)
.Ltmp7314:
	.loc	50 131 19
	vblendvps	%xmm9, %xmm8, %xmm3, %xmm8
	vmovaps	176(%rsp), %xmm0
.Ltmp7315:
	.loc	1 551 14
	vinsertps	$16, 304(%rsp), %xmm0, %xmm3
	vinsertps	$32, %xmm6, %xmm3, %xmm3
	vinsertps	$48, %xmm7, %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm0
.Ltmp7316:
	.loc	1 551 14 is_stmt 0
	vinsertps	$16, 96(%rsp), %xmm0, %xmm4
	vinsertps	$32, 336(%rsp), %xmm4, %xmm4
	vinsertps	$48, 320(%rsp), %xmm4, %xmm4
	vbroadcastss	.LCPI40_36(%rip), %xmm13
.Ltmp7317:
	.loc	49 257 24 is_stmt 1
	vandps	%xmm4, %xmm13, %xmm4
.Ltmp7318:
	.loc	49 257 24 is_stmt 0
	vandps	%xmm3, %xmm13, %xmm3
.Ltmp7319:
	.loc	49 544 14 is_stmt 1
	vcmpltps	960(%r10), %xmm2, %xmm5
.Ltmp7320:
	.loc	49 238 14
	vmaxps	%xmm3, %xmm4, %xmm6
.Ltmp7321:
	.loc	50 131 19
	vblendvps	%xmm5, %xmm6, %xmm4, %xmm5
	vbroadcastss	.LCPI40_5(%rip), %xmm14
.Ltmp7322:
	.loc	49 88 14
	vmulps	%xmm4, %xmm14, %xmm4
.Ltmp7323:
	.loc	49 88 14 is_stmt 0
	vmulps	%xmm3, %xmm14, %xmm3
.Ltmp7324:
	.loc	49 36 14 is_stmt 1
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7325:
	.loc	49 544 14
	vcmpltps	976(%r10), %xmm2, %xmm4
.Ltmp7326:
	.loc	50 131 19
	vblendvps	%xmm4, %xmm3, %xmm5, %xmm3
	vbroadcastss	.LCPI40_6(%rip), %xmm15
.Ltmp7327:
	.loc	49 238 14
	vmaxps	%xmm15, %xmm3, %xmm3
	vbroadcastss	.LCPI40_7(%rip), %xmm4
.Ltmp7328:
	.loc	49 238 14 is_stmt 0
	vmaxps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI40_37(%rip), %xmm4
.Ltmp7329:
	.file	51 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs"
	.loc	51 863 14 is_stmt 1
	vandps	%xmm4, %xmm3, %xmm4
	vbroadcastss	.LCPI40_2(%rip), %xmm5
.Ltmp7330:
	.loc	51 889 14
	vorps	%xmm5, %xmm4, %xmm4
.Ltmp7331:
	.loc	49 62 14
	vaddps	%xmm4, %xmm11, %xmm4
	vbroadcastss	.LCPI40_10(%rip), %xmm5
.Ltmp7332:
	.loc	49 88 14
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI40_11(%rip), %xmm6
.Ltmp7333:
	.loc	49 36 14
	vsubps	%xmm5, %xmm6, %xmm5
.Ltmp7334:
	.loc	49 88 14
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI40_12(%rip), %xmm6
.Ltmp7335:
	.loc	49 36 14
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7336:
	.loc	49 88 14
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI40_13(%rip), %xmm6
.Ltmp7337:
	.loc	49 36 14
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7338:
	.loc	49 88 14
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI40_14(%rip), %xmm6
.Ltmp7339:
	.loc	49 36 14
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7340:
	.loc	49 88 14
	vmulps	%xmm5, %xmm4, %xmm5
	vbroadcastss	.LCPI40_15(%rip), %xmm6
.Ltmp7341:
	.loc	49 36 14
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7342:
	.loc	49 88 14
	vmulps	%xmm5, %xmm4, %xmm4
.Ltmp7343:
	.loc	51 817 24
	vpsrld	$23, %xmm3, %xmm3
	vmovdqa	.LCPI40_16(%rip), %xmm15
.Ltmp7344:
	.loc	51 889 14
	vpor	%xmm3, %xmm15, %xmm3
	vbroadcastss	.LCPI40_17(%rip), %xmm5
.Ltmp7345:
	.loc	49 62 14
	vaddps	%xmm5, %xmm3, %xmm3
.Ltmp7346:
	.loc	49 36 14
	vaddps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI40_18(%rip), %xmm4
.Ltmp7347:
	.loc	49 88 14
	vmulps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI40_19(%rip), %xmm4
.Ltmp7348:
	.loc	49 212 14
	vminps	%xmm4, %xmm3, %xmm3
	vbroadcastss	.LCPI40_20(%rip), %xmm4
.Ltmp7349:
	.loc	49 238 14
	vmaxps	%xmm4, %xmm3, %xmm3
.Ltmp7350:
	.loc	49 544 14
	vcmpltps	1344(%r10), %xmm2, %xmm4
.Ltmp7351:
	.loc	49 62 14
	vsubps	%xmm8, %xmm10, %xmm5
.Ltmp7352:
	.loc	49 558 14
	vcmpleps	%xmm3, %xmm5, %xmm5
.Ltmp7353:
	.loc	49 558 14 is_stmt 0
	vcmpleps	%xmm3, %xmm10, %xmm6
.Ltmp7354:
	.loc	49 257 24 is_stmt 1
	vandnps	%xmm6, %xmm4, %xmm6
.Ltmp7355:
	.loc	49 257 24 is_stmt 0
	vandps	%xmm4, %xmm5, %xmm7
.Ltmp7356:
	.loc	49 302 24 is_stmt 1
	vorps	%xmm6, %xmm7, %xmm6
.Ltmp7357:
	.loc	21 486 47
	vmovaps	1360(%r10), %xmm7
.Ltmp7358:
	.loc	49 544 14
	vcmpltps	%xmm7, %xmm2, %xmm8
.Ltmp7359:
	.loc	49 257 24
	vandnps	%xmm8, %xmm5, %xmm5
	vandps	%xmm4, %xmm5, %xmm4
.Ltmp7360:
	.loc	49 62 14
	vaddps	%xmm7, %xmm11, %xmm5
.Ltmp7361:
	.loc	50 131 19
	vblendvps	%xmm4, %xmm5, %xmm7, %xmm5
.Ltmp7362:
	.loc	49 302 24
	vorps	%xmm6, %xmm4, %xmm4
.Ltmp7363:
	.loc	50 131 19
	vblendvps	%xmm6, 928(%r10), %xmm5, %xmm6
.Ltmp7364:
	.loc	21 508 65
	vmovaps	912(%r10), %xmm7
	.loc	21 508 36 is_stmt 0
	vmovaps	1376(%r10), %xmm8
.Ltmp7365:
	.loc	21 441 27 is_stmt 1
	vmovaps	1392(%r10), %xmm5
.Ltmp7366:
	.loc	21 489 5
	vmovaps	%xmm6, 1360(%r10)
.Ltmp7367:
	.loc	49 62 14
	vsubps	%xmm10, %xmm3, %xmm3
.Ltmp7368:
	.loc	49 62 14 is_stmt 0
	vaddps	%xmm1, %xmm11, %xmm1
	vmovaps	%xmm11, %xmm14
.Ltmp7369:
	.loc	49 88 14 is_stmt 1
	vmulps	%xmm3, %xmm1, %xmm1
.Ltmp7370:
	.loc	50 131 19
	vpcmpgtd	%xmm4, %xmm2, %xmm3
	vpbroadcastd	.LCPI40_2(%rip), %xmm4
	vpand	%xmm4, %xmm3, %xmm3
.Ltmp7371:
	.loc	21 498 5
	vmovdqa	%xmm3, 1344(%r10)
	vbroadcastss	.LCPI40_38(%rip), %xmm15
.Ltmp7372:
	.loc	49 323 24
	vxorps	%xmm15, %xmm12, %xmm2
.Ltmp7373:
	.loc	49 238 14
	vmaxps	%xmm2, %xmm1, %xmm1
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp7374:
	.loc	49 212 14
	vminps	%xmm0, %xmm1, %xmm1
.Ltmp7375:
	.loc	49 544 14
	vcmpltps	%xmm3, %xmm0, %xmm2
.Ltmp7376:
	.loc	50 131 19
	vpcmpgtd	%xmm2, %xmm0, %xmm2
	vpandn	%xmm1, %xmm2, %xmm1
.Ltmp7377:
	.loc	49 544 14
	vcmpltps	%xmm1, %xmm8, %xmm2
.Ltmp7378:
	.loc	50 131 19
	vblendvps	%xmm2, 896(%r10), %xmm7, %xmm2
.Ltmp7379:
	.loc	49 62 14
	vsubps	%xmm8, %xmm1, %xmm1
.Ltmp7380:
	.loc	49 88 14
	vmulps	%xmm2, %xmm1, %xmm1
.Ltmp7381:
	.loc	49 36 14
	vaddps	%xmm1, %xmm8, %xmm1
.Ltmp7382:
	.loc	49 257 24
	vandps	%xmm1, %xmm13, %xmm2
	vbroadcastss	.LCPI40_22(%rip), %xmm3
.Ltmp7383:
	.loc	49 517 14
	vcmpltps	%xmm3, %xmm2, %xmm2
.Ltmp7384:
	.loc	49 257 24
	vandnps	%xmm1, %xmm2, %xmm6
.Ltmp7385:
	.loc	21 510 5
	vmovaps	%xmm6, 1376(%r10)
.Ltmp7386:
	.loc	21 439 26
	vmovaps	1440(%r10), %xmm1
.Ltmp7387:
	.loc	49 36 14
	vaddps	1424(%r10), %xmm5, %xmm2
	vmovdqa	%xmm4, %xmm13
.Ltmp7388:
	.loc	49 504 14
	vcmpeqps	%xmm4, %xmm1, %xmm3
.Ltmp7389:
	.loc	50 131 19
	vblendvps	%xmm3, 1408(%r10), %xmm2, %xmm4
.Ltmp7390:
	.loc	49 544 14
	vcmpltps	%xmm1, %xmm0, %xmm7
.Ltmp7391:
	.loc	49 544 14 is_stmt 0
	vcmpltps	944(%r10), %xmm0, %xmm2
.Ltmp7392:
	.loc	21 442 13 is_stmt 1
	vmaskmovps	%xmm4, %xmm7, 1392(%r10)
	.loc	21 447 13
	vmaskmovps	%xmm0, %xmm3, 1424(%r10)
.Ltmp7393:
	.loc	49 62 14
	vaddps	%xmm1, %xmm11, %xmm1
.Ltmp7394:
	.loc	21 448 13
	vmaskmovps	%xmm1, %xmm7, 1440(%r10)
.Ltmp7395:
	.loc	50 131 19
	vblendvps	%xmm7, %xmm4, %xmm5, %xmm5
.Ltmp7396:
	.loc	21 441 27
	vmovaps	1456(%r10), %xmm1
.Ltmp7397:
	.loc	21 439 26
	vmovaps	1504(%r10), %xmm3
.Ltmp7398:
	.loc	49 504 14
	vcmpeqps	%xmm3, %xmm13, %xmm4
.Ltmp7399:
	.loc	49 36 14
	vaddps	1488(%r10), %xmm1, %xmm7
.Ltmp7400:
	.loc	50 131 19
	vblendvps	%xmm4, 1472(%r10), %xmm7, %xmm7
.Ltmp7401:
	.loc	49 544 14
	vcmpltps	%xmm3, %xmm0, %xmm9
.Ltmp7402:
	.loc	21 442 13
	vmaskmovps	%xmm7, %xmm9, 1456(%r10)
	.loc	21 447 13
	vmaskmovps	%xmm0, %xmm4, 1488(%r10)
	vmovaps	256(%rsp), %xmm4
.Ltmp7403:
	.loc	1 551 14
	vinsertps	$16, 128(%rsp), %xmm4, %xmm4
	vinsertps	$32, 208(%rsp), %xmm4, %xmm4
	vinsertps	$48, 192(%rsp), %xmm4, %xmm8
.Ltmp7404:
	.loc	49 62 14
	vaddps	%xmm3, %xmm11, %xmm3
.Ltmp7405:
	.loc	21 448 13
	vmaskmovps	%xmm3, %xmm9, 1504(%r10)
	vmovaps	224(%rsp), %xmm3
.Ltmp7406:
	.loc	1 551 14
	vinsertps	$16, 112(%rsp), %xmm3, %xmm3
.Ltmp7407:
	.loc	50 131 19
	vblendvps	%xmm9, %xmm7, %xmm1, %xmm1
.Ltmp7408:
	.loc	21 441 27
	vmovaps	1520(%r10), %xmm4
.Ltmp7409:
	.loc	21 439 26
	vmovaps	1568(%r10), %xmm7
.Ltmp7410:
	.loc	49 544 14
	vcmpltps	%xmm7, %xmm0, %xmm9
.Ltmp7411:
	.loc	49 36 14
	vaddps	1552(%r10), %xmm4, %xmm10
.Ltmp7412:
	.loc	49 504 14
	vcmpeqps	%xmm7, %xmm13, %xmm11
.Ltmp7413:
	.loc	50 131 19
	vblendvps	%xmm11, 1536(%r10), %xmm10, %xmm10
.Ltmp7414:
	.loc	21 442 13
	vmaskmovps	%xmm10, %xmm9, 1520(%r10)
	.loc	21 447 13
	vmaskmovps	%xmm0, %xmm11, 1552(%r10)
.Ltmp7415:
	.loc	1 551 14
	vinsertps	$32, 144(%rsp), %xmm3, %xmm3
.Ltmp7416:
	.loc	50 131 19
	vblendvps	%xmm9, %xmm10, %xmm4, %xmm4
.Ltmp7417:
	.loc	49 62 14
	vaddps	%xmm7, %xmm14, %xmm7
.Ltmp7418:
	.loc	21 448 13
	vmaskmovps	%xmm7, %xmm9, 1568(%r10)
.Ltmp7419:
	.loc	21 441 27
	vmovaps	1584(%r10), %xmm7
.Ltmp7420:
	.loc	21 439 26
	vmovaps	1632(%r10), %xmm9
.Ltmp7421:
	.loc	49 36 14
	vaddps	1616(%r10), %xmm7, %xmm10
.Ltmp7422:
	.loc	49 504 14
	vcmpeqps	%xmm13, %xmm9, %xmm11
.Ltmp7423:
	.loc	50 131 19
	vblendvps	%xmm11, 1600(%r10), %xmm10, %xmm10
.Ltmp7424:
	.loc	1 551 14
	vinsertps	$48, 240(%rsp), %xmm3, %xmm12
.Ltmp7425:
	.loc	49 544 14
	vcmpltps	%xmm9, %xmm0, %xmm3
.Ltmp7426:
	.loc	50 131 19
	vblendvps	%xmm3, %xmm10, %xmm7, %xmm7
.Ltmp7427:
	.loc	21 442 13
	vmaskmovps	%xmm10, %xmm3, 1584(%r10)
	.loc	21 447 13
	vmaskmovps	%xmm0, %xmm11, 1616(%r10)
	vbroadcastss	.LCPI40_23(%rip), %xmm15
.Ltmp7428:
	.loc	49 88 14
	vmulps	%xmm6, %xmm15, %xmm10
	vbroadcastss	.LCPI40_24(%rip), %xmm11
.Ltmp7429:
	.loc	49 238 14
	vmaxps	%xmm11, %xmm10, %xmm10
	vbroadcastss	.LCPI40_25(%rip), %xmm11
.Ltmp7430:
	.loc	49 212 14
	vminps	%xmm11, %xmm10, %xmm10
.Ltmp7431:
	.loc	49 62 14
	vaddps	%xmm14, %xmm9, %xmm9
.Ltmp7432:
	.loc	21 448 13
	vmaskmovps	%xmm9, %xmm3, 1632(%r10)
.Ltmp7433:
	.loc	50 724 14
	vroundps	$9, %xmm10, %xmm3
.Ltmp7434:
	.loc	49 62 14
	vsubps	%xmm3, %xmm10, %xmm9
	vbroadcastss	.LCPI40_26(%rip), %xmm15
.Ltmp7435:
	.loc	49 88 14
	vmulps	%xmm15, %xmm9, %xmm10
	vbroadcastss	.LCPI40_27(%rip), %xmm11
.Ltmp7436:
	.loc	49 36 14
	vaddps	%xmm11, %xmm10, %xmm10
.Ltmp7437:
	.loc	49 88 14
	vmulps	%xmm10, %xmm9, %xmm10
	vbroadcastss	.LCPI40_28(%rip), %xmm11
.Ltmp7438:
	.loc	49 36 14
	vaddps	%xmm11, %xmm10, %xmm10
.Ltmp7439:
	.loc	49 88 14
	vmulps	%xmm10, %xmm9, %xmm10
	vbroadcastss	.LCPI40_29(%rip), %xmm11
.Ltmp7440:
	.loc	49 36 14
	vaddps	%xmm11, %xmm10, %xmm10
.Ltmp7441:
	.loc	49 88 14
	vmulps	%xmm10, %xmm9, %xmm10
	vbroadcastss	.LCPI40_30(%rip), %xmm11
.Ltmp7442:
	.loc	49 36 14
	vaddps	%xmm11, %xmm10, %xmm10
.Ltmp7443:
	.loc	49 88 14
	vmulps	%xmm10, %xmm9, %xmm9
.Ltmp7444:
	.loc	49 36 14
	vaddps	%xmm13, %xmm9, %xmm9
	vbroadcastss	.LCPI40_31(%rip), %xmm10
.Ltmp7445:
	.loc	49 36 14 is_stmt 0
	vaddps	%xmm3, %xmm10, %xmm3
.Ltmp7446:
	.loc	51 613 24 is_stmt 1
	vpslld	$23, %xmm3, %xmm3
.Ltmp7447:
	.loc	49 88 14
	vmulps	%xmm3, %xmm9, %xmm3
.Ltmp7448:
	.loc	49 504 14
	vcmpeqps	%xmm0, %xmm6, %xmm6
.Ltmp7449:
	.loc	49 302 24
	vorps	%xmm6, %xmm2, %xmm2
	vmovaps	352(%rsp), %xmm6
.Ltmp7450:
	.loc	49 88 14
	vmulps	%xmm3, %xmm6, %xmm3
.Ltmp7451:
	.loc	50 131 19
	vblendvps	%xmm2, %xmm6, %xmm3, %xmm3
.Ltmp7452:
	.loc	49 544 14
	vcmpltps	1072(%r10), %xmm0, %xmm2
	vbroadcastss	.LCPI40_36(%rip), %xmm10
.Ltmp7453:
	.loc	49 257 24
	vandps	%xmm10, %xmm12, %xmm6
	vbroadcastss	.LCPI40_5(%rip), %xmm11
.Ltmp7454:
	.loc	49 88 14
	vmulps	%xmm6, %xmm11, %xmm9
.Ltmp7455:
	.loc	49 257 24
	vandps	%xmm10, %xmm8, %xmm8
	vmovaps	%xmm10, %xmm12
.Ltmp7456:
	.loc	49 88 14
	vmulps	%xmm11, %xmm8, %xmm10
.Ltmp7457:
	.loc	49 544 14
	vcmpltps	1056(%r10), %xmm0, %xmm11
.Ltmp7458:
	.loc	49 36 14
	vaddps	%xmm9, %xmm10, %xmm9
.Ltmp7459:
	.loc	49 238 14
	vmaxps	%xmm8, %xmm6, %xmm8
.Ltmp7460:
	.loc	50 131 19
	vblendvps	%xmm11, %xmm8, %xmm6, %xmm6
.Ltmp7461:
	.loc	50 131 19 is_stmt 0
	vblendvps	%xmm2, %xmm9, %xmm6, %xmm2
.Ltmp7462:
	.loc	49 238 14 is_stmt 1
	vbroadcastss	.LCPI40_6(%rip), %xmm6
	vmaxps	%xmm6, %xmm2, %xmm2
.Ltmp7463:
	.loc	49 238 14 is_stmt 0
	vbroadcastss	.LCPI40_7(%rip), %xmm6
	vmaxps	%xmm6, %xmm2, %xmm2
.Ltmp7464:
	.loc	51 863 14 is_stmt 1
	vandps	.LCPI40_8(%rip), %xmm2, %xmm6
.Ltmp7465:
	.loc	51 889 14
	vorps	.LCPI40_9(%rip), %xmm6, %xmm6
.Ltmp7466:
	.loc	49 62 14
	vaddps	%xmm6, %xmm14, %xmm6
.Ltmp7467:
	.loc	49 88 14
	vbroadcastss	.LCPI40_10(%rip), %xmm8
	vmulps	%xmm6, %xmm8, %xmm8
.Ltmp7468:
	.loc	49 36 14
	vbroadcastss	.LCPI40_11(%rip), %xmm9
	vsubps	%xmm8, %xmm9, %xmm8
.Ltmp7469:
	.loc	49 88 14
	vmulps	%xmm6, %xmm8, %xmm8
.Ltmp7470:
	.loc	49 36 14
	vbroadcastss	.LCPI40_12(%rip), %xmm9
	vaddps	%xmm9, %xmm8, %xmm8
.Ltmp7471:
	.loc	49 88 14
	vmulps	%xmm6, %xmm8, %xmm8
.Ltmp7472:
	.loc	49 36 14
	vbroadcastss	.LCPI40_13(%rip), %xmm9
	vaddps	%xmm9, %xmm8, %xmm8
.Ltmp7473:
	.loc	49 88 14
	vmulps	%xmm6, %xmm8, %xmm8
.Ltmp7474:
	.loc	49 36 14
	vbroadcastss	.LCPI40_14(%rip), %xmm9
	vaddps	%xmm9, %xmm8, %xmm8
.Ltmp7475:
	.loc	49 88 14
	vmulps	%xmm6, %xmm8, %xmm8
.Ltmp7476:
	.loc	49 36 14
	vbroadcastss	.LCPI40_15(%rip), %xmm9
	vaddps	%xmm9, %xmm8, %xmm8
.Ltmp7477:
	.loc	51 817 24
	vpsrld	$23, %xmm2, %xmm2
.Ltmp7478:
	.loc	51 889 14
	vpor	.LCPI40_16(%rip), %xmm2, %xmm2
.Ltmp7479:
	.loc	49 62 14
	vbroadcastss	.LCPI40_17(%rip), %xmm9
	vaddps	%xmm2, %xmm9, %xmm2
.Ltmp7480:
	.loc	49 88 14
	vmulps	%xmm6, %xmm8, %xmm6
.Ltmp7481:
	.loc	49 36 14
	vaddps	%xmm6, %xmm2, %xmm2
.Ltmp7482:
	.loc	49 88 14
	vbroadcastss	.LCPI40_18(%rip), %xmm6
	vmulps	%xmm6, %xmm2, %xmm2
.Ltmp7483:
	.loc	49 212 14
	vbroadcastss	.LCPI40_19(%rip), %xmm6
	vminps	%xmm6, %xmm2, %xmm2
.Ltmp7484:
	.loc	49 238 14
	vbroadcastss	.LCPI40_20(%rip), %xmm6
	vmaxps	%xmm6, %xmm2, %xmm2
.Ltmp7485:
	.loc	49 544 14
	vcmpltps	1648(%r10), %xmm0, %xmm6
.Ltmp7486:
	.loc	49 558 14
	vcmpleps	%xmm2, %xmm5, %xmm8
.Ltmp7487:
	.loc	49 62 14
	vsubps	%xmm7, %xmm5, %xmm7
.Ltmp7488:
	.loc	49 558 14
	vcmpleps	%xmm2, %xmm7, %xmm7
.Ltmp7489:
	.loc	49 257 24
	vandnps	%xmm8, %xmm6, %xmm8
.Ltmp7490:
	.loc	49 257 24 is_stmt 0
	vandps	%xmm6, %xmm7, %xmm9
.Ltmp7491:
	.loc	49 302 24 is_stmt 1
	vorps	%xmm8, %xmm9, %xmm8
.Ltmp7492:
	.loc	21 486 47
	vmovaps	1664(%r10), %xmm9
.Ltmp7493:
	.loc	49 544 14
	vcmpltps	%xmm9, %xmm0, %xmm10
.Ltmp7494:
	.loc	49 257 24
	vandnps	%xmm10, %xmm7, %xmm7
	vandps	%xmm6, %xmm7, %xmm6
.Ltmp7495:
	.loc	49 62 14
	vaddps	%xmm14, %xmm9, %xmm7
.Ltmp7496:
	.loc	50 131 19
	vblendvps	%xmm6, %xmm7, %xmm9, %xmm7
.Ltmp7497:
	.loc	49 302 24
	vorps	%xmm6, %xmm8, %xmm6
.Ltmp7498:
	.loc	50 131 19
	vblendvps	%xmm8, 1024(%r10), %xmm7, %xmm7
.Ltmp7499:
	.loc	21 489 5
	vmovaps	%xmm7, 1664(%r10)
.Ltmp7500:
	.loc	49 62 14
	vsubps	%xmm5, %xmm2, %xmm2
.Ltmp7501:
	.loc	50 131 19
	vpcmpgtd	%xmm6, %xmm0, %xmm5
	vpand	%xmm5, %xmm13, %xmm5
.Ltmp7502:
	.loc	21 498 5
	vmovdqa	%xmm5, 1648(%r10)
.Ltmp7503:
	.loc	49 62 14
	vaddps	%xmm1, %xmm14, %xmm1
.Ltmp7504:
	.loc	49 88 14
	vmulps	%xmm2, %xmm1, %xmm1
.Ltmp7505:
	.loc	21 508 36
	vmovaps	1680(%r10), %xmm2
.Ltmp7506:
	.loc	49 323 24
	vxorps	.LCPI40_21(%rip), %xmm4, %xmm4
.Ltmp7507:
	.loc	49 238 14
	vmaxps	%xmm4, %xmm1, %xmm1
.Ltmp7508:
	.loc	21 508 65
	vmovaps	1008(%r10), %xmm4
.Ltmp7509:
	.loc	49 212 14
	vminps	%xmm0, %xmm1, %xmm1
.Ltmp7510:
	.loc	49 544 14
	vcmpltps	%xmm5, %xmm0, %xmm5
.Ltmp7511:
	.loc	50 131 19
	vpcmpgtd	%xmm5, %xmm0, %xmm5
	vpandn	%xmm1, %xmm5, %xmm1
.Ltmp7512:
	.loc	49 544 14
	vcmpltps	%xmm1, %xmm2, %xmm5
.Ltmp7513:
	.loc	50 131 19
	vblendvps	%xmm5, 992(%r10), %xmm4, %xmm4
.Ltmp7514:
	.loc	49 62 14
	vsubps	%xmm2, %xmm1, %xmm1
.Ltmp7515:
	.loc	49 88 14
	vmulps	%xmm4, %xmm1, %xmm1
.Ltmp7516:
	.loc	49 36 14
	vaddps	%xmm1, %xmm2, %xmm1
.Ltmp7517:
	.loc	49 257 24
	vandps	%xmm1, %xmm12, %xmm2
.Ltmp7518:
	.loc	49 517 14
	vbroadcastss	.LCPI40_22(%rip), %xmm4
	vcmpltps	%xmm4, %xmm2, %xmm2
.Ltmp7519:
	.loc	49 257 24
	vandnps	%xmm1, %xmm2, %xmm1
.Ltmp7520:
	.loc	49 88 14
	vbroadcastss	.LCPI40_23(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm2
.Ltmp7521:
	.loc	49 238 14
	vbroadcastss	.LCPI40_24(%rip), %xmm4
	vmaxps	%xmm4, %xmm2, %xmm2
.Ltmp7522:
	.loc	49 212 14
	vbroadcastss	.LCPI40_25(%rip), %xmm4
	vminps	%xmm4, %xmm2, %xmm2
.Ltmp7523:
	.loc	50 724 14
	vroundps	$9, %xmm2, %xmm4
.Ltmp7524:
	.loc	49 62 14
	vsubps	%xmm4, %xmm2, %xmm2
.Ltmp7525:
	.loc	49 88 14
	vmulps	%xmm2, %xmm15, %xmm5
.Ltmp7526:
	.loc	49 36 14
	vbroadcastss	.LCPI40_27(%rip), %xmm6
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7527:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm5
.Ltmp7528:
	.loc	49 36 14
	vbroadcastss	.LCPI40_28(%rip), %xmm6
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7529:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm5
.Ltmp7530:
	.loc	49 36 14
	vbroadcastss	.LCPI40_29(%rip), %xmm6
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7531:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm5
.Ltmp7532:
	.loc	49 36 14
	vbroadcastss	.LCPI40_30(%rip), %xmm6
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7533:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm2
.Ltmp7534:
	.loc	21 510 5
	vmovaps	%xmm1, 1680(%r10)
.Ltmp7535:
	.loc	49 36 14
	vaddps	%xmm2, %xmm13, %xmm2
.Ltmp7536:
	.loc	49 36 14 is_stmt 0
	vbroadcastss	.LCPI40_31(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
.Ltmp7537:
	.loc	51 613 24 is_stmt 1
	vpslld	$23, %xmm4, %xmm4
.Ltmp7538:
	.loc	49 88 14
	vmulps	%xmm4, %xmm2, %xmm2
.Ltmp7539:
	.loc	49 504 14
	vcmpeqps	%xmm0, %xmm1, %xmm1
.Ltmp7540:
	.loc	49 544 14
	vcmpltps	1040(%r10), %xmm0, %xmm4
.Ltmp7541:
	.loc	49 302 24
	vorps	%xmm1, %xmm4, %xmm1
	vmovaps	288(%rsp), %xmm0
.Ltmp7542:
	.loc	49 88 14
	vmulps	%xmm2, %xmm0, %xmm2
.Ltmp7543:
	.loc	50 131 19
	vblendvps	%xmm1, %xmm0, %xmm2, %xmm1
	movq	56(%rsp), %rcx
.Ltmp7544:
	.loc	1 551 14
	vmovups	%xmm3, (%rcx)
	movq	72(%rsp), %rcx
.Ltmp7545:
	.loc	1 551 14 is_stmt 0
	vmovups	%xmm1, (%rcx)
.Ltmp7546:
	.loc	21 0 0
	incq	%r9
.Ltmp7547:
	.loc	8 1916 50 is_stmt 1
	addq	$4, %rax
	movq	280(%rsp), %r11
	cmpq	%r9, %r11
	movl	396(%rsp), %r14d
	movq	504(%rsp), %r13
.Ltmp7548:
	.loc	11 900 12
	je	.LBB40_102
.Ltmp7549:
.LBB40_25:
	.loc	15 1050 16
	cmpq	%r9, %r11
	je	.LBB40_275
.Ltmp7550:
	.loc	21 0 0 is_stmt 0
	leal	(%r9,%r13), %r10d
	andl	%ebp, %r10d
	shlq	$2, %r10
.Ltmp7551:
	.loc	21 362 77 is_stmt 1
	leaq	4(%r10), %rsi
.Ltmp7552:
	.loc	25 451 16
	cmpq	%rdx, %rsi
	ja	.LBB40_276
.Ltmp7553:
	.loc	25 0 16 is_stmt 0
	movq	376(%rsp), %rcx
	.loc	21 362 0 is_stmt 1
	leaq	(%rcx,%rax,4), %rdi
.Ltmp7554:
	.loc	1 551 14
	vmovups	(%rdi), %xmm1
	vmovups	%xmm1, (%r8,%r10,4)
.Ltmp7555:
	.loc	25 451 16
	cmpq	%r15, %rsi
	ja	.LBB40_277
.Ltmp7556:
	.loc	25 0 16 is_stmt 0
	movq	368(%rsp), %rcx
	.loc	21 363 0 is_stmt 1
	leaq	(%rcx,%rax,4), %r12
.Ltmp7557:
	.loc	1 551 14
	vmovups	(%r12), %xmm1
	vmovups	%xmm1, (%rbx,%r10,4)
	movq	528(%rsp), %rcx
.Ltmp7558:
	.loc	21 370 21
	leal	(%rcx,%r9), %r11d
	andl	%ebp, %r11d
.Ltmp7559:
	.loc	21 371 54
	leaq	4(,%r11,4), %rsi
.Ltmp7560:
	.loc	21 370 20
	shlq	$2, %r11
.Ltmp7561:
	.loc	25 438 16
	cmpq	%rdx, %rsi
	ja	.LBB40_278
.Ltmp7562:
	.loc	25 438 16 is_stmt 0
	cmpq	%r15, %rsi
	ja	.LBB40_279
.Ltmp7563:
	.loc	25 0 16
	movq	%r12, 72(%rsp)
.Ltmp7564:
	.loc	1 551 14 is_stmt 1
	vmovups	(%r8,%r11,4), %xmm1
.Ltmp7565:
	.loc	21 0 0 is_stmt 0
	leal	(%r9,%r13), %ecx
	movl	%ecx, %r12d
	movq	48(%rsp), %r10
	subl	800(%r10), %r12d
.Ltmp7566:
	.loc	1 551 14
	vmovups	(%rbx,%r11,4), %xmm0
.Ltmp7567:
	.loc	21 0 0
	andl	%ebp, %r12d
	subl	864(%r10), %ecx
	shlq	$2, %r12
	andl	%ebp, %ecx
	shlq	$2, %rcx
	.loc	21 229 5 is_stmt 1
	testb	%r14b, %r14b
	movq	%rdi, 56(%rsp)
	vmovaps	%xmm0, 288(%rsp)
	vmovaps	%xmm1, 352(%rsp)
	je	.LBB40_44
	cmpl	$1, 392(%rsp)
	jne	.LBB40_32
	.loc	21 0 0 is_stmt 0
	cmpq	%rdx, %r12
.Ltmp7568:
	.loc	21 251 32 is_stmt 1
	jae	.LBB40_43
.Ltmp7569:
	.loc	21 252 33
	cmpq	%r15, %rcx
	jae	.LBB40_65
.Ltmp7570:
	.loc	21 248 29
	leal	(%r9,%r13), %esi
	movl	%esi, %edi
	subl	804(%r10), %edi
	andl	%ebp, %edi
	.loc	21 248 28 is_stmt 0
	leaq	1(,%rdi,4), %r11
.Ltmp7571:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %r11
	jae	.LBB40_42
.Ltmp7572:
	.loc	21 0 0 is_stmt 0
	subl	868(%r10), %esi
	andl	%ebp, %esi
	leaq	1(,%rsi,4), %r14
.Ltmp7573:
	.loc	21 252 33 is_stmt 1
	cmpq	%r15, %r14
	jae	.LBB40_64
.Ltmp7574:
	.loc	21 248 29
	leal	(%r9,%r13), %edi
	movl	%edi, %esi
	subl	808(%r10), %esi
	andl	%ebp, %esi
	.loc	21 248 28 is_stmt 0
	leaq	2(,%rsi,4), %rsi
.Ltmp7575:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB40_67
.Ltmp7576:
	.loc	21 0 0 is_stmt 0
	subl	872(%r10), %edi
	andl	%ebp, %edi
	movq	%r10, %r15
	leaq	2(,%rdi,4), %r10
.Ltmp7577:
	.loc	21 252 33 is_stmt 1
	cmpq	40(%rsp), %r10
	jae	.LBB40_69
.Ltmp7578:
	.loc	21 248 29
	addl	%r9d, %r13d
	movl	%r13d, %edi
	subl	812(%r15), %edi
	andl	%ebp, %edi
	.loc	21 248 28 is_stmt 0
	leaq	3(,%rdi,4), %rdi
.Ltmp7579:
	.loc	21 251 32 is_stmt 1
	cmpq	80(%rsp), %rdi
	jae	.LBB40_71
.Ltmp7580:
	.loc	21 0 0 is_stmt 0
	subl	876(%r15), %r13d
	andl	%ebp, %r13d
	leaq	3(,%r13,4), %r13
	movq	40(%rsp), %r15
.Ltmp7581:
	.loc	21 252 33 is_stmt 1
	cmpq	%r15, %r13
	jae	.LBB40_73
.Ltmp7582:
	.loc	21 0 0 is_stmt 0
	vmovss	(%r8,%r12,4), %xmm0
	vmovss	(%rbx,%rcx,4), %xmm2
	vmovss	(%r8,%r11,4), %xmm1
	vmovss	(%rbx,%r14,4), %xmm5
	vmovss	(%r8,%rsi,4), %xmm3
	vmovss	(%rbx,%r10,4), %xmm6
.Ltmp7583:
	.loc	21 251 32 is_stmt 1
	vmovss	(%r8,%rdi,4), %xmm4
.Ltmp7584:
	.loc	21 252 33
	vmovss	(%rbx,%r13,4), %xmm7
	vmovaps	%xmm7, 240(%rsp)
	vmovaps	%xmm6, 144(%rsp)
	vmovaps	%xmm5, 112(%rsp)
	vmovaps	%xmm5, 304(%rsp)
	vmovaps	%xmm2, 224(%rsp)
	vmovaps	%xmm2, 176(%rsp)
	vmovaps	%xmm4, 192(%rsp)
	vmovaps	%xmm4, 320(%rsp)
	vmovaps	%xmm3, 208(%rsp)
	vmovaps	%xmm3, 336(%rsp)
	vmovaps	%xmm1, 128(%rsp)
	vmovaps	%xmm1, 96(%rsp)
	vmovaps	%xmm0, 256(%rsp)
	vmovaps	%xmm0, 160(%rsp)
	movq	80(%rsp), %rdx
.Ltmp7585:
	.loc	17 149 21
	jmp	.LBB40_101
.Ltmp7586:
	.loc	17 0 21 is_stmt 0
.Ltmp7587:
	.p2align	4
.LBB40_44:
	cmpq	%rdx, %r12
.Ltmp7588:
	.loc	21 236 32 is_stmt 1
	jae	.LBB40_48
.Ltmp7589:
	.loc	21 237 33
	cmpq	%r15, %rcx
	jae	.LBB40_46
.Ltmp7590:
	.loc	21 233 29
	leal	(%r9,%r13), %esi
	movl	%esi, %edi
	subl	804(%r10), %edi
	andl	%ebp, %edi
	.loc	21 233 28 is_stmt 0
	leaq	1(,%rdi,4), %r11
.Ltmp7591:
	.loc	21 236 32 is_stmt 1
	cmpq	%rdx, %r11
	jae	.LBB40_51
.Ltmp7592:
	.loc	21 0 0 is_stmt 0
	subl	868(%r10), %esi
	andl	%ebp, %esi
	leaq	1(,%rsi,4), %r14
.Ltmp7593:
	.loc	21 237 33 is_stmt 1
	cmpq	%r15, %r14
	jae	.LBB40_53
.Ltmp7594:
	.loc	21 233 29
	leal	(%r9,%r13), %edi
	movl	%edi, %esi
	subl	808(%r10), %esi
	andl	%ebp, %esi
	.loc	21 233 28 is_stmt 0
	leaq	2(,%rsi,4), %rsi
.Ltmp7595:
	.loc	21 236 32 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB40_55
.Ltmp7596:
	.loc	21 0 0 is_stmt 0
	subl	872(%r10), %edi
	andl	%ebp, %edi
	leaq	2(,%rdi,4), %r15
.Ltmp7597:
	.loc	21 237 33 is_stmt 1
	cmpq	40(%rsp), %r15
	jae	.LBB40_57
.Ltmp7598:
	.loc	21 233 29
	addl	%r9d, %r13d
	movl	%r13d, %edi
	subl	812(%r10), %edi
	andl	%ebp, %edi
	.loc	21 233 28 is_stmt 0
	leaq	3(,%rdi,4), %rdi
.Ltmp7599:
	.loc	21 236 32 is_stmt 1
	cmpq	80(%rsp), %rdi
	jae	.LBB40_59
.Ltmp7600:
	.loc	21 0 0 is_stmt 0
	subl	876(%r10), %r13d
	andl	%ebp, %r13d
	leaq	3(,%r13,4), %r13
.Ltmp7601:
	.loc	21 237 33 is_stmt 1
	cmpq	40(%rsp), %r13
	jae	.LBB40_61
.Ltmp7602:
	.loc	21 0 0 is_stmt 0
	vmovss	(%r8,%r12,4), %xmm4
	vmovss	(%rbx,%rcx,4), %xmm0
	vmovss	(%r8,%r11,4), %xmm5
	vmovss	(%rbx,%r14,4), %xmm1
	vmovss	(%r8,%rsi,4), %xmm6
	vmovss	(%rbx,%r15,4), %xmm2
.Ltmp7603:
	.loc	21 236 32 is_stmt 1
	vmovss	(%r8,%rdi,4), %xmm7
.Ltmp7604:
	.loc	21 237 33
	vmovss	(%rbx,%r13,4), %xmm3
	vmovaps	%xmm3, 192(%rsp)
	vmovaps	%xmm3, 240(%rsp)
	vmovaps	%xmm2, 208(%rsp)
	vmovaps	%xmm2, 144(%rsp)
	vmovaps	%xmm1, 128(%rsp)
	vmovaps	%xmm1, 112(%rsp)
	vmovaps	%xmm0, 256(%rsp)
	vmovaps	%xmm0, 224(%rsp)
	vmovaps	%xmm7, 320(%rsp)
	vmovaps	%xmm6, 336(%rsp)
	vmovaps	%xmm5, 304(%rsp)
	vmovaps	%xmm5, 96(%rsp)
	vmovaps	%xmm4, 176(%rsp)
	vmovaps	%xmm4, 160(%rsp)
	movq	80(%rsp), %rdx
	movq	40(%rsp), %r15
.Ltmp7605:
	.loc	17 149 21
	jmp	.LBB40_101
.Ltmp7606:
	.loc	17 0 21 is_stmt 0
.Ltmp7607:
	.p2align	4
.LBB40_32:
	cmpq	%rdx, %r12
.Ltmp7608:
	.loc	21 266 33 is_stmt 1
	jae	.LBB40_38
	.loc	21 267 33
	cmpq	%r15, %r12
	jae	.LBB40_77
	.loc	21 268 33
	cmpq	%r15, %rcx
	jae	.LBB40_80
	.loc	21 269 33
	cmpq	%rdx, %rcx
	jae	.LBB40_83
.Ltmp7609:
	.loc	21 263 29
	leal	(%r9,%r13), %esi
	movl	%esi, %edi
	subl	804(%r10), %edi
	andl	%ebp, %edi
	.loc	21 263 28 is_stmt 0
	leaq	1(,%rdi,4), %r11
.Ltmp7610:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r11
	jae	.LBB40_37
	.loc	21 267 33
	cmpq	%r15, %r11
	jae	.LBB40_76
.Ltmp7611:
	.loc	21 0 0 is_stmt 0
	subl	868(%r10), %esi
	andl	%ebp, %esi
	leaq	1(,%rsi,4), %r14
.Ltmp7612:
	.loc	21 268 33 is_stmt 1
	cmpq	%r15, %r14
	jae	.LBB40_79
	.loc	21 269 33
	cmpq	%rdx, %r14
	jae	.LBB40_82
.Ltmp7613:
	.loc	21 263 29
	leal	(%r9,%r13), %esi
	movl	%esi, %edi
	subl	808(%r10), %edi
	andl	%ebp, %edi
	.loc	21 263 28 is_stmt 0
	leaq	2(,%rdi,4), %r13
.Ltmp7614:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r13
	jae	.LBB40_85
	.loc	21 267 33
	cmpq	%r15, %r13
	jae	.LBB40_87
.Ltmp7615:
	.loc	21 0 0 is_stmt 0
	subl	872(%r10), %esi
	andl	%ebp, %esi
	leaq	2(,%rsi,4), %rsi
.Ltmp7616:
	.loc	21 268 33 is_stmt 1
	cmpq	%r15, %rsi
	jae	.LBB40_89
	.loc	21 269 33
	cmpq	%rdx, %rsi
	jae	.LBB40_91
.Ltmp7617:
	.loc	21 0 33 is_stmt 0
	movq	504(%rsp), %rdi
	.loc	21 263 29 is_stmt 1
	addl	%r9d, %edi
	movl	%edi, %r10d
	movq	48(%rsp), %r15
	subl	812(%r15), %r10d
	andl	%ebp, %r10d
	.loc	21 263 28 is_stmt 0
	leaq	3(,%r10,4), %r10
.Ltmp7618:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r10
	jae	.LBB40_93
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 267 33 is_stmt 1
	cmpq	%rdx, %r10
	jae	.LBB40_95
.Ltmp7619:
	.loc	21 0 33 is_stmt 0
	movq	48(%rsp), %r15
	subl	876(%r15), %edi
	andl	%ebp, %edi
	leaq	3(,%rdi,4), %rdi
.Ltmp7620:
	.loc	21 268 33 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB40_97
	.loc	21 0 33 is_stmt 0
	movq	%rdx, %r15
	movq	80(%rsp), %rdx
	.loc	21 269 33 is_stmt 1
	cmpq	%rdx, %rdi
	jb	.LBB40_100
	.loc	21 0 33 is_stmt 0
	movq	%rdi, %rcx
	.loc	21 269 33
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%rcx, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp7621:
.LBB40_102:
	.loc	15 2584 13 is_stmt 1
	leal	(%r11,%r13), %eax
.Ltmp7622:
	.loc	21 413 5
	movl	%eax, 1808(%r10)
	movq	400(%rsp), %rcx
	movq	496(%rsp), %rdx
	movq	488(%rsp), %r8
	movl	388(%rsp), %edi
.Ltmp7623:
.LBB40_7:
	.loc	6 691 12
	cmpl	%edi, %ecx
	jbe	.LBB40_8
	.loc	6 692 25
	leaq	(,%r11,4), %rdi
.Ltmp7624:
	.loc	25 580 12
	movq	%rdx, %rsi
	subq	%rdi, %rsi
	jb	.LBB40_281
.Ltmp7625:
	.loc	25 580 12 is_stmt 0
	movq	%r8, %r9
	subq	%rdi, %r9
	jb	.LBB40_213
.Ltmp7626:
	.loc	21 61 8 is_stmt 1
	cmpl	$1, 1772(%r10)
	jne	.LBB40_129
	.loc	21 0 8 is_stmt 0
	xorl	%r13d, %r13d
	.loc	21 61 8
	jmp	.LBB40_133
.LBB40_129:
.Ltmp7627:
	.loc	48 2494 21 is_stmt 1
	movl	800(%r10), %eax
	movb	$2, %r13b
.Ltmp7628:
	.loc	8 1878 54
	cmpl	864(%r10), %eax
.Ltmp7629:
	.loc	48 2494 21
	jne	.LBB40_133
	movl	804(%r10), %eax
.Ltmp7630:
	.loc	8 1878 54
	cmpl	868(%r10), %eax
.Ltmp7631:
	.loc	48 2494 21
	jne	.LBB40_133
	movl	808(%r10), %eax
.Ltmp7632:
	.loc	8 1878 54
	cmpl	872(%r10), %eax
.Ltmp7633:
	.loc	48 2494 21
	jne	.LBB40_133
	movl	812(%r10), %eax
.Ltmp7634:
	.loc	8 1878 54
	cmpl	876(%r10), %eax
	sete	%al
	movb	$2, %r13b
.Ltmp7635:
	.loc	48 2494 21
	subb	%al, %r13b
.Ltmp7636:
.LBB40_133:
	.loc	48 0 21 is_stmt 0
	movq	376(%rsp), %rax
	leaq	(%rax,%rdi,4), %rax
	movq	%rax, 192(%rsp)
	movq	368(%rsp), %rax
	leaq	(%rax,%rdi,4), %rax
	movq	%rax, 160(%rsp)
	subq	%r11, %rcx
	movq	%rcx, 112(%rsp)
.Ltmp7637:
	.loc	6 736 31 is_stmt 1
	movq	768(%r10), %rbp
	movq	776(%r10), %rdx
	movq	%r10, %rax
	.loc	6 741 31
	movq	832(%r10), %r10
	movq	840(%rax), %r12
	.loc	6 747 28
	movl	1812(%rax), %r8d
.Ltmp7638:
	.loc	21 353 16
	movl	1808(%rax), %ecx
	movq	%r9, 80(%rsp)
.Ltmp7639:
	.loc	11 900 12
	movq	%r9, %rdi
	movq	%rcx, %r9
	shrq	$2, %rdi
	movq	%rdi, 176(%rsp)
	movq	%rsi, 304(%rsp)
	shrq	$2, %rsi
	movq	%rsi, 208(%rsp)
	subl	1816(%rax), %ecx
	movq	%rcx, 96(%rsp)
	xorl	%eax, %eax
	movzbl	%r13b, %ecx
	movl	%ecx, 320(%rsp)
	xorl	%ebx, %ebx
	movq	%rdx, 56(%rsp)
	movq	%r12, 40(%rsp)
	movl	%r13d, 336(%rsp)
	movq	%r9, 72(%rsp)
	vxorps	%xmm9, %xmm9, %xmm9
	jmp	.LBB40_134
.Ltmp7640:
	.loc	11 0 12 is_stmt 0
.Ltmp7641:
	.p2align	4
.LBB40_184:
	vmovss	(%rbp,%rcx,4), %xmm14
	vmovss	(%r10,%r14,4), %xmm15
	vmovss	(%rbp,%rdi,4), %xmm13
	vmovss	(%r10,%r15,4), %xmm4
	vmovss	(%rbp,%rsi,4), %xmm10
	vmovss	(%r10,%r11,4), %xmm2
.Ltmp7642:
	.loc	21 251 32 is_stmt 1
	vmovss	(%rbp,%r9,4), %xmm11
.Ltmp7643:
	.loc	21 252 33
	vmovss	(%r10,%r13,4), %xmm1
	vmovaps	%xmm1, 128(%rsp)
	vmovaps	%xmm2, 144(%rsp)
	vmovaps	%xmm4, %xmm8
	vmovaps	%xmm15, %xmm7
	vmovaps	%xmm11, %xmm6
	vmovaps	%xmm10, %xmm3
	vmovaps	%xmm13, %xmm5
	vmovaps	%xmm14, %xmm12
.Ltmp7644:
.LBB40_211:
	.loc	1 551 14
	vinsertps	$16, %xmm13, %xmm14, %xmm13
	vinsertps	$32, %xmm10, %xmm13, %xmm10
	vinsertps	$48, %xmm11, %xmm10, %xmm0
	vmovaps	%xmm0, 224(%rsp)
.Ltmp7645:
	.loc	1 551 14 is_stmt 0
	vinsertps	$16, %xmm4, %xmm15, %xmm13
.Ltmp7646:
	.loc	1 551 14
	vinsertps	$16, %xmm8, %xmm7, %xmm4
	vinsertps	$32, %xmm2, %xmm4, %xmm2
	vinsertps	$48, %xmm1, %xmm2, %xmm1
.Ltmp7647:
	.loc	1 551 14
	vinsertps	$16, %xmm5, %xmm12, %xmm2
	vinsertps	$32, %xmm3, %xmm2, %xmm2
	vinsertps	$48, %xmm6, %xmm2, %xmm2
	movq	48(%rsp), %rcx
.Ltmp7648:
	.loc	49 544 14 is_stmt 1
	vcmpltps	976(%rcx), %xmm9, %xmm3
	vbroadcastss	.LCPI40_36(%rip), %xmm8
.Ltmp7649:
	.loc	49 257 24
	vandps	%xmm2, %xmm8, %xmm2
	vbroadcastss	.LCPI40_5(%rip), %xmm11
.Ltmp7650:
	.loc	49 88 14
	vmulps	%xmm2, %xmm11, %xmm4
.Ltmp7651:
	.loc	49 257 24
	vandps	%xmm1, %xmm8, %xmm1
.Ltmp7652:
	.loc	49 544 14
	vcmpltps	960(%rcx), %xmm9, %xmm5
.Ltmp7653:
	.loc	49 88 14
	vmulps	%xmm1, %xmm11, %xmm6
.Ltmp7654:
	.loc	49 238 14
	vmaxps	%xmm1, %xmm2, %xmm1
.Ltmp7655:
	.loc	50 131 19
	vblendvps	%xmm5, %xmm1, %xmm2, %xmm1
.Ltmp7656:
	.loc	49 36 14
	vaddps	%xmm4, %xmm6, %xmm2
.Ltmp7657:
	.loc	50 131 19
	vblendvps	%xmm3, %xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI40_6(%rip), %xmm12
.Ltmp7658:
	.loc	49 238 14
	vmaxps	%xmm12, %xmm1, %xmm1
	vbroadcastss	.LCPI40_7(%rip), %xmm14
.Ltmp7659:
	.loc	49 238 14 is_stmt 0
	vmaxps	%xmm14, %xmm1, %xmm1
	vbroadcastss	.LCPI40_37(%rip), %xmm15
.Ltmp7660:
	.loc	51 863 14 is_stmt 1
	vandps	%xmm1, %xmm15, %xmm2
	vbroadcastss	.LCPI40_2(%rip), %xmm3
.Ltmp7661:
	.loc	51 889 14
	vorps	%xmm3, %xmm2, %xmm2
	vbroadcastss	.LCPI40_3(%rip), %xmm0
.Ltmp7662:
	.loc	49 62 14
	vaddps	%xmm0, %xmm2, %xmm2
	vbroadcastss	.LCPI40_10(%rip), %xmm3
.Ltmp7663:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI40_11(%rip), %xmm4
.Ltmp7664:
	.loc	49 36 14
	vsubps	%xmm3, %xmm4, %xmm3
.Ltmp7665:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI40_12(%rip), %xmm4
.Ltmp7666:
	.loc	49 36 14
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7667:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI40_13(%rip), %xmm4
.Ltmp7668:
	.loc	49 36 14
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7669:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI40_14(%rip), %xmm4
.Ltmp7670:
	.loc	49 36 14
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7671:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
	vbroadcastss	.LCPI40_15(%rip), %xmm4
.Ltmp7672:
	.loc	49 36 14
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7673:
	.loc	51 817 24
	vpsrld	$23, %xmm1, %xmm1
	vmovdqa	.LCPI40_16(%rip), %xmm4
.Ltmp7674:
	.loc	51 889 14
	vpor	%xmm4, %xmm1, %xmm1
	vbroadcastss	.LCPI40_17(%rip), %xmm4
.Ltmp7675:
	.loc	49 62 14
	vaddps	%xmm4, %xmm1, %xmm1
.Ltmp7676:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm2
.Ltmp7677:
	.loc	49 36 14
	vaddps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI40_18(%rip), %xmm2
.Ltmp7678:
	.loc	49 88 14
	vmulps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI40_19(%rip), %xmm2
.Ltmp7679:
	.loc	49 212 14
	vminps	%xmm2, %xmm1, %xmm1
	vbroadcastss	.LCPI40_20(%rip), %xmm2
.Ltmp7680:
	.loc	49 238 14
	vmaxps	%xmm2, %xmm1, %xmm1
.Ltmp7681:
	.loc	49 544 14
	vcmpltps	1344(%rcx), %xmm9, %xmm2
.Ltmp7682:
	.loc	21 451 21
	vmovaps	1088(%rcx), %xmm3
.Ltmp7683:
	.loc	21 486 47
	vmovaps	1360(%rcx), %xmm4
.Ltmp7684:
	.loc	49 62 14
	vsubps	1280(%rcx), %xmm3, %xmm5
.Ltmp7685:
	.loc	49 558 14
	vcmpleps	%xmm1, %xmm3, %xmm6
.Ltmp7686:
	.loc	49 558 14 is_stmt 0
	vcmpleps	%xmm1, %xmm5, %xmm5
.Ltmp7687:
	.loc	49 257 24 is_stmt 1
	vandnps	%xmm6, %xmm2, %xmm6
.Ltmp7688:
	.loc	49 257 24 is_stmt 0
	vandps	%xmm2, %xmm5, %xmm7
.Ltmp7689:
	.loc	49 302 24 is_stmt 1
	vorps	%xmm6, %xmm7, %xmm6
.Ltmp7690:
	.loc	49 544 14
	vcmpltps	%xmm4, %xmm9, %xmm7
.Ltmp7691:
	.loc	49 257 24
	vandnps	%xmm7, %xmm5, %xmm5
	vandps	%xmm2, %xmm5, %xmm2
.Ltmp7692:
	.loc	49 62 14
	vaddps	%xmm0, %xmm4, %xmm5
.Ltmp7693:
	.loc	50 131 19
	vblendvps	%xmm2, %xmm5, %xmm4, %xmm4
.Ltmp7694:
	.loc	21 508 65
	vmovaps	912(%rcx), %xmm5
.Ltmp7695:
	.loc	49 302 24
	vorps	%xmm6, %xmm2, %xmm2
.Ltmp7696:
	.loc	50 131 19
	vblendvps	%xmm6, 928(%rcx), %xmm4, %xmm4
.Ltmp7697:
	.loc	21 508 36
	vmovaps	1376(%rcx), %xmm6
.Ltmp7698:
	.loc	21 489 5
	vmovaps	%xmm4, 1360(%rcx)
.Ltmp7699:
	.loc	50 131 19
	vpcmpgtd	%xmm2, %xmm9, %xmm2
	vpbroadcastd	.LCPI40_2(%rip), %xmm10
	vpand	%xmm2, %xmm10, %xmm2
.Ltmp7700:
	.loc	21 498 5
	vmovdqa	%xmm2, 1344(%rcx)
.Ltmp7701:
	.loc	49 62 14
	vsubps	%xmm3, %xmm1, %xmm1
.Ltmp7702:
	.loc	49 62 14 is_stmt 0
	vaddps	1152(%rcx), %xmm0, %xmm3
.Ltmp7703:
	.loc	49 88 14 is_stmt 1
	vmulps	%xmm1, %xmm3, %xmm1
	vbroadcastss	.LCPI40_38(%rip), %xmm12
.Ltmp7704:
	.loc	49 323 24
	vxorps	1216(%rcx), %xmm12, %xmm3
.Ltmp7705:
	.loc	49 238 14
	vmaxps	%xmm3, %xmm1, %xmm1
.Ltmp7706:
	.loc	49 212 14
	vminps	%xmm9, %xmm1, %xmm1
.Ltmp7707:
	.loc	49 544 14
	vcmpltps	%xmm2, %xmm9, %xmm2
.Ltmp7708:
	.loc	50 131 19
	vpcmpgtd	%xmm2, %xmm9, %xmm2
	vpandn	%xmm1, %xmm2, %xmm1
.Ltmp7709:
	.loc	49 544 14
	vcmpltps	%xmm1, %xmm6, %xmm2
.Ltmp7710:
	.loc	50 131 19
	vblendvps	%xmm2, 896(%rcx), %xmm5, %xmm2
.Ltmp7711:
	.loc	1 551 14
	vinsertps	$32, 144(%rsp), %xmm13, %xmm3
	vinsertps	$48, 128(%rsp), %xmm3, %xmm3
.Ltmp7712:
	.loc	49 62 14
	vsubps	%xmm6, %xmm1, %xmm1
.Ltmp7713:
	.loc	49 88 14
	vmulps	%xmm2, %xmm1, %xmm1
.Ltmp7714:
	.loc	49 36 14
	vaddps	%xmm1, %xmm6, %xmm1
.Ltmp7715:
	.loc	49 257 24
	vandps	%xmm1, %xmm8, %xmm2
	vbroadcastss	.LCPI40_22(%rip), %xmm14
.Ltmp7716:
	.loc	49 517 14
	vcmpltps	%xmm14, %xmm2, %xmm2
.Ltmp7717:
	.loc	49 257 24
	vandnps	%xmm1, %xmm2, %xmm1
.Ltmp7718:
	.loc	21 510 5
	vmovaps	%xmm1, 1376(%rcx)
	vbroadcastss	.LCPI40_23(%rip), %xmm15
.Ltmp7719:
	.loc	49 88 14
	vmulps	%xmm1, %xmm15, %xmm2
	vbroadcastss	.LCPI40_24(%rip), %xmm4
.Ltmp7720:
	.loc	49 238 14
	vmaxps	%xmm4, %xmm2, %xmm2
	vbroadcastss	.LCPI40_25(%rip), %xmm4
.Ltmp7721:
	.loc	49 212 14
	vminps	%xmm4, %xmm2, %xmm2
.Ltmp7722:
	.loc	50 724 14
	vroundps	$9, %xmm2, %xmm4
.Ltmp7723:
	.loc	49 62 14
	vsubps	%xmm4, %xmm2, %xmm2
	vbroadcastss	.LCPI40_26(%rip), %xmm5
.Ltmp7724:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI40_27(%rip), %xmm6
.Ltmp7725:
	.loc	49 36 14
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7726:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI40_28(%rip), %xmm6
.Ltmp7727:
	.loc	49 36 14
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7728:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI40_29(%rip), %xmm6
.Ltmp7729:
	.loc	49 36 14
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7730:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm5
	vbroadcastss	.LCPI40_30(%rip), %xmm6
.Ltmp7731:
	.loc	49 36 14
	vaddps	%xmm6, %xmm5, %xmm5
.Ltmp7732:
	.loc	49 88 14
	vmulps	%xmm5, %xmm2, %xmm2
.Ltmp7733:
	.loc	49 36 14
	vaddps	%xmm2, %xmm10, %xmm2
	vbroadcastss	.LCPI40_31(%rip), %xmm5
.Ltmp7734:
	.loc	49 36 14 is_stmt 0
	vaddps	%xmm5, %xmm4, %xmm4
.Ltmp7735:
	.loc	51 613 24 is_stmt 1
	vpslld	$23, %xmm4, %xmm4
.Ltmp7736:
	.loc	49 88 14
	vmulps	%xmm4, %xmm2, %xmm2
.Ltmp7737:
	.loc	49 504 14
	vcmpeqps	%xmm1, %xmm9, %xmm1
.Ltmp7738:
	.loc	49 544 14
	vcmpltps	944(%rcx), %xmm9, %xmm4
.Ltmp7739:
	.loc	49 302 24
	vorps	%xmm1, %xmm4, %xmm1
	vmovaps	240(%rsp), %xmm4
.Ltmp7740:
	.loc	49 88 14
	vmulps	%xmm2, %xmm4, %xmm2
.Ltmp7741:
	.loc	50 131 19
	vblendvps	%xmm1, %xmm4, %xmm2, %xmm13
.Ltmp7742:
	.loc	49 544 14
	vcmpltps	1072(%rcx), %xmm9, %xmm1
.Ltmp7743:
	.loc	49 257 24
	vandps	%xmm3, %xmm8, %xmm2
.Ltmp7744:
	.loc	49 88 14
	vmulps	%xmm2, %xmm11, %xmm3
.Ltmp7745:
	.loc	49 257 24
	vandps	224(%rsp), %xmm8, %xmm4
.Ltmp7746:
	.loc	49 88 14
	vmulps	%xmm4, %xmm11, %xmm5
.Ltmp7747:
	.loc	49 36 14
	vaddps	%xmm3, %xmm5, %xmm3
.Ltmp7748:
	.loc	49 544 14
	vcmpltps	1056(%rcx), %xmm9, %xmm5
.Ltmp7749:
	.loc	49 238 14
	vmaxps	%xmm4, %xmm2, %xmm4
.Ltmp7750:
	.loc	50 131 19
	vblendvps	%xmm5, %xmm4, %xmm2, %xmm2
.Ltmp7751:
	.loc	50 131 19 is_stmt 0
	vblendvps	%xmm1, %xmm3, %xmm2, %xmm1
.Ltmp7752:
	.loc	49 238 14 is_stmt 1
	vbroadcastss	.LCPI40_6(%rip), %xmm2
	vmaxps	%xmm2, %xmm1, %xmm1
.Ltmp7753:
	.loc	49 238 14 is_stmt 0
	vbroadcastss	.LCPI40_7(%rip), %xmm2
	vmaxps	%xmm2, %xmm1, %xmm1
.Ltmp7754:
	.loc	51 863 14 is_stmt 1
	vandps	.LCPI40_8(%rip), %xmm1, %xmm2
.Ltmp7755:
	.loc	51 889 14
	vorps	.LCPI40_9(%rip), %xmm2, %xmm2
.Ltmp7756:
	.loc	49 62 14
	vaddps	%xmm0, %xmm2, %xmm2
.Ltmp7757:
	.loc	49 88 14
	vbroadcastss	.LCPI40_10(%rip), %xmm3
	vmulps	%xmm3, %xmm2, %xmm3
.Ltmp7758:
	.loc	49 36 14
	vbroadcastss	.LCPI40_11(%rip), %xmm4
	vsubps	%xmm3, %xmm4, %xmm3
.Ltmp7759:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
.Ltmp7760:
	.loc	49 36 14
	vbroadcastss	.LCPI40_12(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7761:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
.Ltmp7762:
	.loc	49 36 14
	vbroadcastss	.LCPI40_13(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7763:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
.Ltmp7764:
	.loc	49 36 14
	vbroadcastss	.LCPI40_14(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7765:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm3
.Ltmp7766:
	.loc	49 36 14
	vbroadcastss	.LCPI40_15(%rip), %xmm4
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7767:
	.loc	51 817 24
	vpsrld	$23, %xmm1, %xmm1
.Ltmp7768:
	.loc	51 889 14
	vpor	.LCPI40_16(%rip), %xmm1, %xmm1
.Ltmp7769:
	.loc	49 62 14
	vbroadcastss	.LCPI40_17(%rip), %xmm4
	vaddps	%xmm4, %xmm1, %xmm1
.Ltmp7770:
	.loc	21 451 21
	vmovaps	1392(%rcx), %xmm4
.Ltmp7771:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm2
.Ltmp7772:
	.loc	49 36 14
	vaddps	%xmm2, %xmm1, %xmm1
.Ltmp7773:
	.loc	49 88 14
	vbroadcastss	.LCPI40_18(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm1
.Ltmp7774:
	.loc	49 212 14
	vbroadcastss	.LCPI40_19(%rip), %xmm2
	vminps	%xmm2, %xmm1, %xmm1
.Ltmp7775:
	.loc	49 238 14
	vbroadcastss	.LCPI40_20(%rip), %xmm2
	vmaxps	%xmm2, %xmm1, %xmm1
.Ltmp7776:
	.loc	49 544 14
	vcmpltps	1648(%rcx), %xmm9, %xmm2
.Ltmp7777:
	.loc	49 558 14
	vcmpleps	%xmm1, %xmm4, %xmm3
.Ltmp7778:
	.loc	49 62 14
	vsubps	1584(%rcx), %xmm4, %xmm5
.Ltmp7779:
	.loc	49 558 14
	vcmpleps	%xmm1, %xmm5, %xmm5
.Ltmp7780:
	.loc	49 257 24
	vandnps	%xmm3, %xmm2, %xmm3
.Ltmp7781:
	.loc	21 486 47
	vmovaps	1664(%rcx), %xmm6
.Ltmp7782:
	.loc	49 544 14
	vcmpltps	%xmm6, %xmm9, %xmm7
.Ltmp7783:
	.loc	49 257 24
	vandnps	%xmm7, %xmm5, %xmm7
.Ltmp7784:
	.loc	49 257 24 is_stmt 0
	vandps	%xmm2, %xmm5, %xmm5
.Ltmp7785:
	.loc	49 257 24
	vandps	%xmm2, %xmm7, %xmm2
.Ltmp7786:
	.loc	49 62 14 is_stmt 1
	vaddps	%xmm0, %xmm6, %xmm7
.Ltmp7787:
	.loc	50 131 19
	vblendvps	%xmm2, %xmm7, %xmm6, %xmm6
.Ltmp7788:
	.loc	49 302 24
	vorps	%xmm3, %xmm5, %xmm3
.Ltmp7789:
	.loc	50 131 19
	vblendvps	%xmm3, 1024(%rcx), %xmm6, %xmm5
.Ltmp7790:
	.loc	49 302 24
	vorps	%xmm3, %xmm2, %xmm2
.Ltmp7791:
	.loc	49 62 14
	vaddps	1456(%rcx), %xmm0, %xmm3
.Ltmp7792:
	.loc	49 323 24
	vxorps	1520(%rcx), %xmm12, %xmm6
.Ltmp7793:
	.loc	21 489 5
	vmovaps	%xmm5, 1664(%rcx)
.Ltmp7794:
	.loc	49 62 14
	vsubps	%xmm4, %xmm1, %xmm1
.Ltmp7795:
	.loc	50 131 19
	vpcmpgtd	%xmm2, %xmm9, %xmm2
	vpand	%xmm2, %xmm10, %xmm2
.Ltmp7796:
	.loc	21 498 5
	vmovdqa	%xmm2, 1648(%rcx)
.Ltmp7797:
	.loc	49 88 14
	vmulps	%xmm1, %xmm3, %xmm1
.Ltmp7798:
	.loc	21 508 36
	vmovaps	1680(%rcx), %xmm3
.Ltmp7799:
	.loc	49 238 14
	vmaxps	%xmm6, %xmm1, %xmm1
.Ltmp7800:
	.loc	21 508 65
	vmovaps	1008(%rcx), %xmm4
.Ltmp7801:
	.loc	49 212 14
	vminps	%xmm9, %xmm1, %xmm1
.Ltmp7802:
	.loc	49 544 14
	vcmpltps	%xmm2, %xmm9, %xmm2
.Ltmp7803:
	.loc	50 131 19
	vpcmpgtd	%xmm2, %xmm9, %xmm2
	vpandn	%xmm1, %xmm2, %xmm1
.Ltmp7804:
	.loc	49 544 14
	vcmpltps	%xmm1, %xmm3, %xmm2
.Ltmp7805:
	.loc	50 131 19
	vblendvps	%xmm2, 992(%rcx), %xmm4, %xmm2
.Ltmp7806:
	.loc	49 62 14
	vsubps	%xmm3, %xmm1, %xmm1
.Ltmp7807:
	.loc	49 88 14
	vmulps	%xmm2, %xmm1, %xmm1
.Ltmp7808:
	.loc	49 36 14
	vaddps	%xmm1, %xmm3, %xmm1
.Ltmp7809:
	.loc	49 257 24
	vandps	%xmm1, %xmm8, %xmm2
.Ltmp7810:
	.loc	49 517 14
	vcmpltps	%xmm14, %xmm2, %xmm2
.Ltmp7811:
	.loc	49 257 24
	vandnps	%xmm1, %xmm2, %xmm1
.Ltmp7812:
	.loc	49 88 14
	vmulps	%xmm1, %xmm15, %xmm2
.Ltmp7813:
	.loc	49 238 14
	vbroadcastss	.LCPI40_24(%rip), %xmm0
	vmaxps	%xmm0, %xmm2, %xmm2
.Ltmp7814:
	.loc	49 212 14
	vbroadcastss	.LCPI40_25(%rip), %xmm0
	vminps	%xmm0, %xmm2, %xmm2
.Ltmp7815:
	.loc	50 724 14
	vroundps	$9, %xmm2, %xmm3
.Ltmp7816:
	.loc	49 62 14
	vsubps	%xmm3, %xmm2, %xmm2
.Ltmp7817:
	.loc	49 88 14
	vbroadcastss	.LCPI40_26(%rip), %xmm0
	vmulps	%xmm0, %xmm2, %xmm4
.Ltmp7818:
	.loc	49 36 14
	vbroadcastss	.LCPI40_27(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
.Ltmp7819:
	.loc	49 88 14
	vmulps	%xmm4, %xmm2, %xmm4
.Ltmp7820:
	.loc	49 36 14
	vbroadcastss	.LCPI40_28(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
.Ltmp7821:
	.loc	49 88 14
	vmulps	%xmm4, %xmm2, %xmm4
.Ltmp7822:
	.loc	49 36 14
	vbroadcastss	.LCPI40_29(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
.Ltmp7823:
	.loc	49 88 14
	vmulps	%xmm4, %xmm2, %xmm4
.Ltmp7824:
	.loc	49 36 14
	vbroadcastss	.LCPI40_30(%rip), %xmm0
	vaddps	%xmm0, %xmm4, %xmm4
.Ltmp7825:
	.loc	49 88 14
	vmulps	%xmm4, %xmm2, %xmm2
.Ltmp7826:
	.loc	21 510 5
	vmovaps	%xmm1, 1680(%rcx)
.Ltmp7827:
	.loc	49 36 14
	vaddps	%xmm2, %xmm10, %xmm2
.Ltmp7828:
	.loc	49 36 14 is_stmt 0
	vbroadcastss	.LCPI40_31(%rip), %xmm0
	vaddps	%xmm0, %xmm3, %xmm3
.Ltmp7829:
	.loc	51 613 24 is_stmt 1
	vpslld	$23, %xmm3, %xmm3
.Ltmp7830:
	.loc	49 88 14
	vmulps	%xmm3, %xmm2, %xmm2
.Ltmp7831:
	.loc	49 504 14
	vcmpeqps	%xmm1, %xmm9, %xmm1
.Ltmp7832:
	.loc	49 544 14
	vcmpltps	1040(%rcx), %xmm9, %xmm3
.Ltmp7833:
	.loc	49 302 24
	vorps	%xmm1, %xmm3, %xmm1
	vmovaps	256(%rsp), %xmm0
.Ltmp7834:
	.loc	49 88 14
	vmulps	%xmm2, %xmm0, %xmm2
.Ltmp7835:
	.loc	50 131 19
	vblendvps	%xmm1, %xmm0, %xmm2, %xmm1
	movq	288(%rsp), %rcx
.Ltmp7836:
	.loc	1 551 14
	vmovups	%xmm13, (%rcx)
	movq	352(%rsp), %rcx
.Ltmp7837:
	.loc	1 551 14 is_stmt 0
	vmovups	%xmm1, (%rcx)
.Ltmp7838:
	.loc	21 0 0
	incq	%rbx
.Ltmp7839:
	.loc	8 1916 50 is_stmt 1
	addq	$4, %rax
	cmpq	%rbx, 112(%rsp)
	movq	280(%rsp), %r11
	movq	56(%rsp), %rdx
	movl	336(%rsp), %r13d
	movq	72(%rsp), %r9
.Ltmp7840:
	.loc	11 900 12
	je	.LBB40_212
.Ltmp7841:
.LBB40_134:
	.loc	15 1050 16
	cmpq	%rbx, 208(%rsp)
	je	.LBB40_282
.Ltmp7842:
	.loc	21 0 0 is_stmt 0
	leal	(%r9,%rbx), %r11d
	andl	%r8d, %r11d
	shlq	$2, %r11
.Ltmp7843:
	.loc	21 362 77 is_stmt 1
	leaq	4(%r11), %rsi
.Ltmp7844:
	.loc	25 451 16
	cmpq	%rdx, %rsi
	ja	.LBB40_283
.Ltmp7845:
	.loc	25 0 16 is_stmt 0
	movq	192(%rsp), %rcx
	.loc	21 362 0 is_stmt 1
	leaq	(%rcx,%rax,4), %rdi
.Ltmp7846:
	.loc	1 551 14
	vmovups	(%rdi), %xmm1
	vmovups	%xmm1, (%rbp,%r11,4)
.Ltmp7847:
	.loc	25 438 16
	cmpq	%rbx, 176(%rsp)
	je	.LBB40_284
.Ltmp7848:
	.loc	25 451 16
	cmpq	%r12, %rsi
	ja	.LBB40_285
.Ltmp7849:
	.loc	25 0 16 is_stmt 0
	movq	160(%rsp), %rcx
	.loc	21 363 0 is_stmt 1
	leaq	(%rcx,%rax,4), %r14
.Ltmp7850:
	.loc	1 551 14
	vmovups	(%r14), %xmm1
	vmovups	%xmm1, (%r10,%r11,4)
	movq	96(%rsp), %rcx
.Ltmp7851:
	.loc	21 370 21
	leal	(%rcx,%rbx), %r15d
	andl	%r8d, %r15d
.Ltmp7852:
	.loc	21 371 54
	leaq	4(,%r15,4), %rsi
.Ltmp7853:
	.loc	21 370 20
	shlq	$2, %r15
.Ltmp7854:
	.loc	25 438 16
	cmpq	%rdx, %rsi
	ja	.LBB40_286
.Ltmp7855:
	.loc	25 438 16 is_stmt 0
	cmpq	%r12, %rsi
	ja	.LBB40_287
.Ltmp7856:
	.loc	25 0 16
	movq	%r14, 352(%rsp)
.Ltmp7857:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rbp,%r15,4), %xmm1
.Ltmp7858:
	.loc	21 0 0 is_stmt 0
	leal	(%r9,%rbx), %r14d
	movl	%r14d, %ecx
	movq	48(%rsp), %rsi
	subl	800(%rsi), %ecx
.Ltmp7859:
	.loc	1 551 14
	vmovups	(%r10,%r15,4), %xmm0
.Ltmp7860:
	.loc	21 0 0
	andl	%r8d, %ecx
	subl	864(%rsi), %r14d
	shlq	$2, %rcx
	andl	%r8d, %r14d
	shlq	$2, %r14
	.loc	21 229 5 is_stmt 1
	testb	%r13b, %r13b
	movq	%rdi, 288(%rsp)
	vmovaps	%xmm0, 256(%rsp)
	vmovaps	%xmm1, 240(%rsp)
	je	.LBB40_154
	cmpl	$1, 320(%rsp)
	jne	.LBB40_142
	.loc	21 0 0 is_stmt 0
	cmpq	%rdx, %rcx
.Ltmp7861:
	.loc	21 251 32 is_stmt 1
	jae	.LBB40_153
.Ltmp7862:
	.loc	21 252 33
	cmpq	%r12, %r14
	jae	.LBB40_175
.Ltmp7863:
	.loc	21 248 29
	leal	(%r9,%rbx), %esi
	movl	%esi, %edi
	movq	48(%rsp), %r12
	subl	804(%r12), %edi
	andl	%r8d, %edi
	.loc	21 248 28 is_stmt 0
	leaq	1(,%rdi,4), %rdi
.Ltmp7864:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB40_152
.Ltmp7865:
	.loc	21 0 0 is_stmt 0
	subl	868(%r12), %esi
	andl	%r8d, %esi
	leaq	1(,%rsi,4), %r15
.Ltmp7866:
	.loc	21 252 33 is_stmt 1
	cmpq	40(%rsp), %r15
	jae	.LBB40_174
.Ltmp7867:
	.loc	21 248 29
	addl	%ebx, %r9d
	movl	%r9d, %esi
	subl	808(%r12), %esi
	andl	%r8d, %esi
	.loc	21 248 28 is_stmt 0
	leaq	2(,%rsi,4), %rsi
.Ltmp7868:
	.loc	21 251 32 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB40_177
.Ltmp7869:
	.loc	21 0 0 is_stmt 0
	subl	872(%r12), %r9d
	andl	%r8d, %r9d
	leaq	2(,%r9,4), %r11
.Ltmp7870:
	.loc	21 252 33 is_stmt 1
	cmpq	40(%rsp), %r11
	jae	.LBB40_179
.Ltmp7871:
	.loc	21 0 33 is_stmt 0
	movq	72(%rsp), %rdx
	.loc	21 248 29 is_stmt 1
	leal	(%rdx,%rbx), %r13d
	movl	%r13d, %r9d
	subl	812(%r12), %r9d
	andl	%r8d, %r9d
	.loc	21 248 28 is_stmt 0
	leaq	3(,%r9,4), %r9
.Ltmp7872:
	.loc	21 251 32 is_stmt 1
	cmpq	56(%rsp), %r9
	jae	.LBB40_181
.Ltmp7873:
	.loc	21 0 0 is_stmt 0
	subl	876(%r12), %r13d
	andl	%r8d, %r13d
	leaq	3(,%r13,4), %r13
	movq	40(%rsp), %r12
.Ltmp7874:
	.loc	21 252 33 is_stmt 1
	cmpq	%r12, %r13
	jb	.LBB40_184
	jmp	.LBB40_183
.Ltmp7875:
	.loc	21 0 33 is_stmt 0
.Ltmp7876:
	.p2align	4
.LBB40_154:
	cmpq	%rdx, %rcx
.Ltmp7877:
	.loc	21 236 32 is_stmt 1
	jae	.LBB40_158
.Ltmp7878:
	.loc	21 237 33
	cmpq	%r12, %r14
	jae	.LBB40_156
.Ltmp7879:
	.loc	21 233 29
	leal	(%r9,%rbx), %esi
	movl	%esi, %edi
	movq	48(%rsp), %r12
	subl	804(%r12), %edi
	andl	%r8d, %edi
	.loc	21 233 28 is_stmt 0
	leaq	1(,%rdi,4), %rdi
.Ltmp7880:
	.loc	21 236 32 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB40_161
.Ltmp7881:
	.loc	21 0 0 is_stmt 0
	subl	868(%r12), %esi
	andl	%r8d, %esi
	leaq	1(,%rsi,4), %r15
.Ltmp7882:
	.loc	21 237 33 is_stmt 1
	cmpq	40(%rsp), %r15
	jae	.LBB40_163
.Ltmp7883:
	.loc	21 233 29
	addl	%ebx, %r9d
	movl	%r9d, %esi
	subl	808(%r12), %esi
	andl	%r8d, %esi
	.loc	21 233 28 is_stmt 0
	leaq	2(,%rsi,4), %rsi
.Ltmp7884:
	.loc	21 236 32 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB40_165
.Ltmp7885:
	.loc	21 0 0 is_stmt 0
	subl	872(%r12), %r9d
	andl	%r8d, %r9d
	leaq	2(,%r9,4), %r11
.Ltmp7886:
	.loc	21 237 33 is_stmt 1
	cmpq	40(%rsp), %r11
	jae	.LBB40_167
.Ltmp7887:
	.loc	21 0 33 is_stmt 0
	movq	72(%rsp), %rdx
	.loc	21 233 29 is_stmt 1
	leal	(%rdx,%rbx), %r13d
	movl	%r13d, %r9d
	subl	812(%r12), %r9d
	andl	%r8d, %r9d
	.loc	21 233 28 is_stmt 0
	leaq	3(,%r9,4), %r9
.Ltmp7888:
	.loc	21 236 32 is_stmt 1
	cmpq	56(%rsp), %r9
	jae	.LBB40_169
.Ltmp7889:
	.loc	21 0 0 is_stmt 0
	subl	876(%r12), %r13d
	andl	%r8d, %r13d
	leaq	3(,%r13,4), %r13
	movq	40(%rsp), %r12
.Ltmp7890:
	.loc	21 237 33 is_stmt 1
	cmpq	%r12, %r13
	jae	.LBB40_171
.Ltmp7891:
	.loc	21 0 0 is_stmt 0
	vmovss	(%rbp,%rcx,4), %xmm7
	vmovss	(%r10,%r14,4), %xmm14
	vmovss	(%rbp,%rdi,4), %xmm8
	vmovss	(%r10,%r15,4), %xmm13
	vmovss	(%rbp,%rsi,4), %xmm2
	vmovss	(%r10,%r11,4), %xmm10
.Ltmp7892:
	.loc	21 236 32 is_stmt 1
	vmovss	(%rbp,%r9,4), %xmm1
.Ltmp7893:
	.loc	21 237 33
	vmovss	(%r10,%r13,4), %xmm11
	vmovaps	%xmm11, 128(%rsp)
	vmovaps	%xmm10, 144(%rsp)
	vmovaps	%xmm13, %xmm4
	vmovaps	%xmm14, %xmm15
	vmovaps	%xmm1, %xmm6
	vmovaps	%xmm2, %xmm3
	vmovaps	%xmm8, %xmm5
	vmovaps	%xmm7, %xmm12
.Ltmp7894:
	.loc	17 149 21
	jmp	.LBB40_211
.Ltmp7895:
	.loc	17 0 21 is_stmt 0
.Ltmp7896:
	.p2align	4
.LBB40_142:
	cmpq	%rdx, %rcx
.Ltmp7897:
	.loc	21 266 33 is_stmt 1
	jae	.LBB40_148
	.loc	21 267 33
	cmpq	%r12, %rcx
	jae	.LBB40_187
	.loc	21 268 33
	cmpq	%r12, %r14
	jae	.LBB40_190
	.loc	21 269 33
	cmpq	%rdx, %r14
	jae	.LBB40_193
.Ltmp7898:
	.loc	21 263 29
	leal	(%r9,%rbx), %esi
	movl	%esi, %edi
	movq	48(%rsp), %r9
	subl	804(%r9), %edi
	andl	%r8d, %edi
	.loc	21 263 28 is_stmt 0
	leaq	1(,%rdi,4), %r9
.Ltmp7899:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r9
	jae	.LBB40_147
	.loc	21 267 33
	cmpq	%r12, %r9
	jae	.LBB40_186
.Ltmp7900:
	.loc	21 0 33 is_stmt 0
	movq	48(%rsp), %rdi
	subl	868(%rdi), %esi
	andl	%r8d, %esi
	leaq	1(,%rsi,4), %r15
.Ltmp7901:
	.loc	21 268 33 is_stmt 1
	cmpq	%r12, %r15
	jae	.LBB40_189
	.loc	21 269 33
	cmpq	%rdx, %r15
	jae	.LBB40_192
.Ltmp7902:
	.loc	21 0 33 is_stmt 0
	movq	72(%rsp), %rsi
	.loc	21 263 29 is_stmt 1
	addl	%ebx, %esi
	movl	%esi, %edi
	movq	48(%rsp), %r11
	subl	808(%r11), %edi
	andl	%r8d, %edi
	.loc	21 263 28 is_stmt 0
	leaq	2(,%rdi,4), %r13
.Ltmp7903:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r13
	jae	.LBB40_195
	.loc	21 267 33
	cmpq	%r12, %r13
	jae	.LBB40_197
.Ltmp7904:
	.loc	21 0 33 is_stmt 0
	movq	48(%rsp), %rdi
	subl	872(%rdi), %esi
	andl	%r8d, %esi
	leaq	2(,%rsi,4), %rsi
.Ltmp7905:
	.loc	21 268 33 is_stmt 1
	cmpq	%r12, %rsi
	jae	.LBB40_199
	.loc	21 0 33 is_stmt 0
	movq	56(%rsp), %rdx
	.loc	21 269 33 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB40_201
.Ltmp7906:
	.loc	21 0 33 is_stmt 0
	movq	72(%rsp), %rdi
	.loc	21 263 29 is_stmt 1
	addl	%ebx, %edi
	movl	%edi, %r11d
	movq	48(%rsp), %r12
	subl	812(%r12), %r11d
	andl	%r8d, %r11d
	.loc	21 263 28 is_stmt 0
	leaq	3(,%r11,4), %r11
.Ltmp7907:
	.loc	21 266 33 is_stmt 1
	cmpq	%rdx, %r11
	jae	.LBB40_203
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 267 33 is_stmt 1
	cmpq	%rdx, %r11
	jae	.LBB40_205
.Ltmp7908:
	.loc	21 0 33 is_stmt 0
	movq	48(%rsp), %r12
	subl	876(%r12), %edi
	andl	%r8d, %edi
	leaq	3(,%rdi,4), %rdi
.Ltmp7909:
	.loc	21 268 33 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB40_207
	.loc	21 269 33
	cmpq	56(%rsp), %rdi
	jae	.LBB40_209
.Ltmp7910:
	.loc	21 0 33 is_stmt 0
	movq	%rdx, %r12
	vmovss	(%rbp,%rcx,4), %xmm12
	vmovss	(%r10,%rcx,4), %xmm7
	vmovss	(%r10,%r14,4), %xmm15
	vmovss	(%rbp,%r14,4), %xmm14
	vmovss	(%rbp,%r9,4), %xmm5
	vmovss	(%r10,%r9,4), %xmm8
	vmovss	(%r10,%r15,4), %xmm4
	vmovss	(%rbp,%r15,4), %xmm13
	vmovss	(%rbp,%r13,4), %xmm3
	vmovss	(%r10,%r13,4), %xmm2
	vmovss	(%r10,%rsi,4), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovss	(%rbp,%rsi,4), %xmm10
.Ltmp7911:
	vmovss	(%rbp,%r11,4), %xmm6
	vmovss	(%r10,%r11,4), %xmm1
	.loc	21 268 33 is_stmt 1
	vmovss	(%r10,%rdi,4), %xmm0
	vmovaps	%xmm0, 128(%rsp)
	.loc	21 269 33
	vmovss	(%rbp,%rdi,4), %xmm11
	jmp	.LBB40_211
.Ltmp7912:
.LBB40_212:
	.loc	21 0 33 is_stmt 0
	movq	112(%rsp), %rax
.Ltmp7913:
	.loc	15 2584 13 is_stmt 1
	addl	%r9d, %eax
	movq	48(%rsp), %r10
.Ltmp7914:
	.loc	21 413 5
	movl	%eax, 1808(%r10)
	movq	400(%rsp), %rcx
	movq	496(%rsp), %rdx
	movq	488(%rsp), %r8
.Ltmp7915:
.LBB40_8:
	.loc	6 701 9
	subl	%r11d, 1820(%r10)
.Ltmp7916:
	.loc	6 773 33
	movq	$0, 424(%rsp)
	movq	$2, 432(%rsp)
	movq	376(%rsp), %rax
	movq	%rax, 440(%rsp)
	movq	%rdx, 448(%rsp)
	movq	368(%rsp), %rax
	movq	%rax, 456(%rsp)
	movq	%r8, 464(%rsp)
	movq	$0, 472(%rsp)
	leaq	768(%r10), %rax
	movq	%rax, 320(%rsp)
	leaq	512(%r10), %rax
	movq	%rax, 304(%rsp)
	leaq	896(%r10), %rax
	movq	%rax, 280(%rsp)
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqps	%xmm0, %xmm0, %xmm0
.Ltmp7917:
	.loc	27 131 12
	leaq	(,%rcx,4), %r9
	movl	$24, %eax
	vbroadcastss	.LCPI40_36(%rip), %xmm4
	vbroadcastss	.LCPI40_2(%rip), %xmm1
	vmovaps	%xmm1, 80(%rsp)
	vbroadcastss	.LCPI40_32(%rip), %xmm5
	vmovsd	.LCPI40_33(%rip), %xmm6
	vmovsd	.LCPI40_34(%rip), %xmm7
	vmovsd	.LCPI40_35(%rip), %xmm8
	xorl	%ecx, %ecx
	xorl	%esi, %esi
	vmovaps	%xmm0, 96(%rsp)
	movq	%r9, 112(%rsp)
	jmp	.LBB40_9
	.loc	27 0 12 is_stmt 0
.Ltmp7918:
	.p2align	4
.LBB40_14:
	movl	$1, %esi
	movl	$32, %eax
	.loc	27 131 12 is_stmt 1
	testb	$1, 336(%rsp)
	movb	$1, %cl
	vmovaps	96(%rsp), %xmm0
	jne	.LBB40_15
.Ltmp7919:
.LBB40_9:
	.loc	27 0 12 is_stmt 0
	movq	%rcx, 336(%rsp)
.Ltmp7920:
	.loc	25 253 13 is_stmt 1
	movq	%rsi, %rdx
	shlq	$4, %rdx
	leaq	440(%rsp), %rcx
.Ltmp7921:
	.loc	1 1733 9
	movq	(%rcx,%rdx), %rbp
	movq	%rdx, 56(%rsp)
	movq	8(%rcx,%rdx), %r15
.Ltmp7922:
	.loc	16 2155 12
	movq	%r15, %rcx
	andq	$-4, %rcx
	je	.LBB40_12
.Ltmp7923:
	.loc	16 0 12 is_stmt 0
	xorl	%edx, %edx
	vmovaps	96(%rsp), %xmm0
	.p2align	4
.LBB40_11:
.Ltmp7924:
	.loc	49 257 24 is_stmt 1
	vandps	(%rbp,%rdx,4), %xmm4, %xmm1
.Ltmp7925:
	.loc	49 517 14
	vcmpltps	%xmm5, %xmm1, %xmm1
.Ltmp7926:
	.loc	49 257 24
	vandps	%xmm1, %xmm0, %xmm0
.Ltmp7927:
	.loc	16 2155 12
	addq	$4, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB40_11
.Ltmp7928:
.LBB40_12:
	.loc	16 0 12 is_stmt 0
	movq	%rsi, %r14
	imulq	$304, %rsi, %rcx
.Ltmp7929:
	.file	52 "/home/bl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wide-1.6.1" "src/f32x4_.rs"
	.loc	52 285 9 is_stmt 1
	vpcmpeqd	%xmm1, %xmm1, %xmm1
	vtestps	%xmm1, %xmm0
	vandps	1376(%r10,%rcx), %xmm4, %xmm0
.Ltmp7930:
	.loc	6 775 16
	jae	.LBB40_103
.Ltmp7931:
	.loc	49 517 14
	vcmpltps	%xmm5, %xmm0, %xmm1
.Ltmp7932:
	.loc	49 257 24
	vandps	96(%rsp), %xmm1, %xmm1
.Ltmp7933:
	.loc	52 285 9
	vpcmpeqd	%xmm2, %xmm2, %xmm2
	vtestps	%xmm2, %xmm1
.Ltmp7934:
	.loc	6 775 43
	jb	.LBB40_14
.LBB40_103:
.Ltmp7935:
	.loc	16 2155 12
	movq	%r15, %rdx
	vmovaps	96(%rsp), %xmm2
	vmovaps	%xmm2, %xmm1
	movabsq	$2305843009213693948, %rsi
	andq	%rsi, %rdx
	je	.LBB40_107
.Ltmp7936:
	.loc	16 0 12 is_stmt 0
	xorl	%esi, %esi
	vmovaps	%xmm2, %xmm1
	.p2align	4
.LBB40_105:
.Ltmp7937:
	.loc	49 257 24 is_stmt 1
	vandps	(%rbp,%rsi,4), %xmm4, %xmm2
.Ltmp7938:
	.loc	49 517 14
	vcmpltps	%xmm5, %xmm2, %xmm2
.Ltmp7939:
	.loc	49 257 24
	vandps	%xmm2, %xmm1, %xmm1
.Ltmp7940:
	.loc	16 2155 12
	addq	$4, %rsi
	cmpq	%rsi, %rdx
	jne	.LBB40_105
.Ltmp7941:
	.loc	16 0 12 is_stmt 0
	vmovaps	96(%rsp), %xmm2
.LBB40_107:
.Ltmp7942:
	.loc	49 517 14 is_stmt 1
	vcmpltps	%xmm5, %xmm0, %xmm0
.Ltmp7943:
	.loc	49 257 24
	vandps	%xmm2, %xmm0, %xmm0
.Ltmp7944:
	.loc	50 131 19
	vpsrad	$31, %xmm1, %xmm1
	vmovdqa	80(%rsp), %xmm2
	vpandn	%xmm2, %xmm1, %xmm1
.Ltmp7945:
	.loc	28 185 12
	vmovd	%xmm1, %esi
	xorl	%edx, %edx
	testl	%esi, %esi
	vpextrd	$1, %xmm1, %esi
	setne	%dl
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	vpextrd	$2, %xmm1, %esi
	xorl	%edi, %edi
	testl	%esi, %esi
	setne	%dil
	vpextrd	$3, %xmm1, %r9d
	xorl	%esi, %esi
	testl	%r9d, %r9d
	setne	%sil
.Ltmp7946:
	.loc	50 131 19
	vpsrad	$31, %xmm0, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
.Ltmp7947:
	.loc	28 185 12
	vmovd	%xmm0, %r10d
	xorl	%r9d, %r9d
	testl	%r10d, %r10d
	setne	%r9b
	vpextrd	$1, %xmm0, %r10d
	xorl	%r12d, %r12d
	testl	%r10d, %r10d
	setne	%r12b
	addl	%r12d, %r12d
	vpextrd	$2, %xmm0, %r11d
	xorl	%r10d, %r10d
	testl	%r11d, %r11d
	setne	%r10b
	shll	$2, %r10d
	vpextrd	$3, %xmm0, %r11d
	xorl	%ebx, %ebx
	testl	%r11d, %r11d
	setne	%bl
	shll	$3, %ebx
	leal	(%rdx,%r8,2), %edx
	leal	(%rdx,%rdi,4), %edx
	leal	(%rdx,%rsi,8), %edx
.Ltmp7948:
	.loc	28 185 12 is_stmt 0
	orl	%r9d, %r12d
	orl	%r10d, %r12d
	orl	%edx, %r12d
.Ltmp7949:
	.loc	6 779 17 is_stmt 1
	orl	%ebx, %r12d
	movl	%r12d, 40(%rsp)
	movq	%r14, %rdi
.Ltmp7950:
	.loc	6 780 13
	movq	%r14, %rdx
	shlq	$5, %rdx
	movq	48(%rsp), %r10
	leaq	(%r10,%rdx), %r14
	shlq	$6, %rdi
	movq	320(%rsp), %rsi
	leaq	(%rsi,%rdi), %r13
	movq	56(%rsp), %rsi
.Ltmp7951:
	.loc	11 900 12
	addq	304(%rsp), %rsi
	movq	%rsi, 56(%rsp)
	addq	%r10, %rdi
	movq	%rdi, 208(%rsp)
	leaq	(%rdx,%rdx,2), %rdx
	movq	280(%rsp), %rsi
	leaq	(%rsi,%rdx), %rdi
	movq	%rdi, 192(%rsp)
	leaq	32(%rsi,%rdx), %rsi
	movq	%rsi, 288(%rsp)
	leaq	928(%r10,%rdx), %rdx
	movq	%rdx, 176(%rsp)
	movq	536(%rsp), %rdx
	leaq	(%rdx,%rcx), %rsi
	movq	%rsi, 160(%rsp)
	leaq	(%rdx,%rcx), %rbx
	addq	$288, %rbx
	addq	%rsp, %rax
	addq	$544, %rax
	movq	%rax, 72(%rsp)
	xorl	%r12d, %r12d
	movq	400(%rsp), %r8
	movq	112(%rsp), %r9
	movabsq	$9223372036854775807, %r11
	jmp	.LBB40_108
.Ltmp7952:
	.loc	11 0 12 is_stmt 0
.Ltmp7953:
	.p2align	4
.LBB40_122:
	.loc	6 789 34 is_stmt 1
	leaq	(%r12,%r12,4), %rax
	movq	72(%rsp), %rsi
	movq	(%rsi,%rax,8), %rcx
.Ltmp7954:
	.loc	15 2428 13
	addq	%r8, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp7955:
	.loc	6 794 17
	movq	%rcx, (%rsi,%rax,8)
.Ltmp7956:
.LBB40_123:
	.loc	11 0 0 is_stmt 0
	incq	%r12
	.loc	11 900 12 is_stmt 1
	addq	$4, %rbp
.Ltmp7957:
	.loc	8 1916 50
	cmpq	$4, %r12
.Ltmp7958:
	.loc	11 900 12
	je	.LBB40_14
.Ltmp7959:
.LBB40_108:
	.loc	11 0 12 is_stmt 0
	movl	40(%rsp), %eax
.Ltmp7960:
	.loc	6 781 20 is_stmt 1
	btl	%r12d, %eax
	jae	.LBB40_123
	.loc	6 0 20 is_stmt 0
	testl	%r8d, %r8d
.Ltmp7961:
	.loc	11 900 12 is_stmt 1
	je	.LBB40_113
.Ltmp7962:
	.loc	11 0 12 is_stmt 0
	xorl	%eax, %eax
	.p2align	4
.LBB40_111:
.Ltmp7963:
	.loc	6 785 21 is_stmt 1
	leaq	(%r12,%rax), %rdi
	cmpq	%r15, %rdi
	jae	.LBB40_124
	movl	$0, (%rbp,%rax,4)
.Ltmp7964:
	.loc	8 1916 50
	addq	$4, %rax
	cmpq	%rax, %r9
.Ltmp7965:
	.loc	11 900 12
	jne	.LBB40_111
.Ltmp7966:
.LBB40_113:
	.loc	6 562 21
	movq	1696(%r10), %rax
.Ltmp7967:
	.loc	8 1916 50
	testq	%rax, %rax
.Ltmp7968:
	.loc	11 900 12
	je	.LBB40_117
.Ltmp7969:
	.loc	11 0 12 is_stmt 0
	movq	8(%r13), %rsi
	movq	%r12, %rdi
	.p2align	4
.LBB40_115:
.Ltmp7970:
	.loc	6 565 13 is_stmt 1
	cmpq	%rsi, %rdi
	jae	.LBB40_280
	movq	(%r13), %rcx
	movl	$0, (%rcx,%rdi,4)
.Ltmp7971:
	.loc	8 1916 50
	addq	$4, %rdi
	decq	%rax
.Ltmp7972:
	.loc	11 900 12
	jne	.LBB40_115
.Ltmp7973:
.LBB40_117:
	.loc	6 509 22
	movq	%r12, %rax
	shlq	$6, %rax
	vmovss	(%r14,%rax), %xmm12
	vmovss	4(%r14,%rax), %xmm11
	vmovss	8(%r14,%rax), %xmm10
	vmovss	12(%r14,%rax), %xmm9
	vmovss	16(%r14,%rax), %xmm0
	vmovss	20(%r14,%rax), %xmm2
	vmovss	24(%r14,%rax), %xmm13
	vmovss	28(%r14,%rax), %xmm1
.Ltmp7974:
	.loc	6 510 9
	movq	%r12, %rax
	shlq	$5, %rax
	movq	56(%rsp), %rcx
	vmovss	%xmm1, (%rcx,%rax)
	vmovss	%xmm0, 4(%rcx,%rax)
	vmovss	%xmm2, 8(%rcx,%rax)
	vmovss	%xmm13, 12(%rcx,%rax)
.Ltmp7975:
	.loc	6 533 27
	movl	1776(%r10), %eax
.Ltmp7976:
	.loc	9 82 17
	vcvtss2sd	%xmm1, %xmm1, %xmm1
.Ltmp7977:
	.loc	9 82 17 is_stmt 0
	vcvtsi2sd	%rax, %xmm15, %xmm3
.Ltmp7978:
	.loc	6 406 20 is_stmt 1
	vmulsd	%xmm3, %xmm1, %xmm1
	vdivsd	%xmm6, %xmm1, %xmm1
	.loc	6 406 19 is_stmt 0
	vaddsd	%xmm7, %xmm1, %xmm1
.Ltmp7979:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm1, %xmm1, %xmm1
	vmovq	%xmm1, %rcx
.Ltmp7980:
	.loc	6 407 9
	testq	%rcx, %rcx
	sets	%dl
	andq	%r11, %rcx
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
	jne	.LBB40_122
	vucomisd	%xmm8, %xmm1
	ja	.LBB40_122
.Ltmp7981:
	.loc	9 82 17
	vcvtss2sd	%xmm2, %xmm2, %xmm2
.Ltmp7982:
	.loc	6 406 20
	vmulsd	%xmm3, %xmm2, %xmm2
	vdivsd	%xmm6, %xmm2, %xmm2
	.loc	6 406 19 is_stmt 0
	vaddsd	%xmm7, %xmm2, %xmm2
.Ltmp7983:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm2, %xmm2, %xmm2
	vmovq	%xmm2, %rcx
.Ltmp7984:
	.loc	6 407 9
	testq	%rcx, %rcx
	sets	%dl
	andq	%r11, %rcx
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
	jne	.LBB40_122
	vucomisd	%xmm8, %xmm2
	ja	.LBB40_122
.Ltmp7985:
	.loc	6 534 80
	movl	1816(%r10), %ecx
	vxorpd	%xmm3, %xmm3, %xmm3
.Ltmp7986:
	.loc	6 410 10
	vmaxsd	%xmm1, %xmm3, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rdx
.Ltmp7987:
	.loc	6 410 10 is_stmt 0
	vmaxsd	%xmm2, %xmm3, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rsi
.Ltmp7988:
	.loc	6 536 9 is_stmt 1
	subl	%edx, %ecx
	movl	$0, %edx
	cmovbl	%edx, %ecx
	movq	208(%rsp), %rdx
	movl	%ecx, 800(%rdx,%r12,4)
	.loc	6 537 62
	movl	%esi, %ecx
	vcvtsi2ss	%rcx, %xmm15, %xmm1
	movq	288(%rsp), %rcx
.Ltmp7989:
	.loc	6 399 5
	vmovaps	(%rcx), %xmm2
	vmovaps	%xmm2, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, (%rcx)
	movq	%rax, %rdi
	movq	%rax, 128(%rsp)
	vmovss	%xmm9, 352(%rsp)
	vmovss	%xmm10, 256(%rsp)
	vmovss	%xmm11, 240(%rsp)
	vmovss	%xmm12, 144(%rsp)
	vmovss	%xmm13, 224(%rsp)
.Ltmp7990:
	.loc	6 541 13
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movq	192(%rsp), %rax
.Ltmp7991:
	.loc	6 399 5
	vmovaps	(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, (%rax)
	vmovss	224(%rsp), %xmm0
	movq	128(%rsp), %rdi
.Ltmp7992:
	.loc	6 546 13
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	vmovsd	.LCPI40_35(%rip), %xmm8
	movabsq	$9223372036854775807, %r11
	vmovsd	.LCPI40_34(%rip), %xmm7
	vmovsd	.LCPI40_33(%rip), %xmm6
	vbroadcastss	.LCPI40_32(%rip), %xmm5
	vbroadcastss	.LCPI40_36(%rip), %xmm4
	movq	112(%rsp), %r9
	movq	400(%rsp), %r8
	movq	48(%rsp), %r10
	movq	288(%rsp), %rax
.Ltmp7993:
	.loc	6 399 5
	vmovaps	-16(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -16(%rax)
	movq	176(%rsp), %rax
.Ltmp7994:
	.loc	6 517 29
	vmovaps	(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
.Ltmp7995:
	.loc	6 393 5
	vmovss	(%rsp,%r12,4), %xmm0
.Ltmp7996:
	.loc	6 399 5
	vmovaps	(%rbx), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, (%rbx)
.Ltmp7997:
	.loc	6 399 5
	vmovaps	-32(%rbx), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	movl	$1065353216, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, -32(%rbx)
.Ltmp7998:
	.loc	6 399 5
	vmovaps	-16(%rbx), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 400 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -16(%rbx)
	movq	160(%rsp), %rax
.Ltmp7999:
	.loc	6 399 5
	vmovaps	(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovss	144(%rsp), %xmm1
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, (%rax)
.Ltmp8000:
	.loc	6 399 5
	vmovaps	-272(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -272(%rbx)
.Ltmp8001:
	.loc	6 399 5
	vmovaps	-256(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -256(%rbx)
.Ltmp8002:
	.loc	6 399 5
	vmovaps	-240(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -240(%rbx)
.Ltmp8003:
	.loc	6 399 5
	vmovaps	-224(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovss	240(%rsp), %xmm1
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -224(%rbx)
.Ltmp8004:
	.loc	6 399 5
	vmovaps	-208(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -208(%rbx)
.Ltmp8005:
	.loc	6 399 5
	vmovaps	-192(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -192(%rbx)
.Ltmp8006:
	.loc	6 399 5
	vmovaps	-176(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -176(%rbx)
.Ltmp8007:
	.loc	6 399 5
	vmovaps	-160(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovss	256(%rsp), %xmm1
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -160(%rbx)
.Ltmp8008:
	.loc	6 399 5
	vmovaps	-144(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -144(%rbx)
.Ltmp8009:
	.loc	6 399 5
	vmovaps	-128(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -128(%rbx)
.Ltmp8010:
	.loc	6 399 5
	vmovaps	-112(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -112(%rbx)
.Ltmp8011:
	.loc	6 399 5
	vmovaps	-96(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovss	352(%rsp), %xmm1
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -96(%rbx)
.Ltmp8012:
	.loc	6 399 5
	vmovaps	-80(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -80(%rbx)
.Ltmp8013:
	.loc	6 399 5
	vmovaps	-64(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -64(%rbx)
.Ltmp8014:
	.loc	6 399 5
	vmovaps	-48(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 400 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 401 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -48(%rbx)
	jmp	.LBB40_122
.Ltmp8015:
.LBB40_15:
	.loc	1 551 14
	vmovups	544(%rsp), %ymm0
	vmovups	576(%rsp), %ymm1
	vmovups	608(%rsp), %ymm2
	vmovups	640(%rsp), %ymm3
	vmovups	%ymm0, 864(%rsp)
	vmovups	%ymm1, 896(%rsp)
	vmovups	%ymm2, 928(%rsp)
	vmovups	%ymm3, 960(%rsp)
	vmovdqu	672(%rsp), %ymm0
	vmovdqu	%ymm0, 992(%rsp)
	leaq	864(%rsp), %rsi
.Ltmp8016:
	.loc	6 1102 17
	movl	$320, %edx
	movq	520(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	%rbx, %rdi
	movzbl	71(%rsp), %eax
	movb	%al, 320(%rbx)
.Ltmp8017:
.LBB40_4:
	.loc	6 1103 14
	movq	%rdi, %rax
	.loc	6 1103 14 epilogue_begin is_stmt 0
	addq	$1192, %rsp
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
.LBB40_290:
	.cfi_def_cfa_offset 1248
.Ltmp8018:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_99ab90c4f654517a4481f4facf03e67c(%rip), %rcx
	movq	72(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8019:
.LBB40_275:
	.loc	25 443 13
	leaq	4(%rax), %rsi
	leaq	.Lalloc_d0dbd696a058609bd94bbe1fa75d0723(%rip), %rcx
	movq	%rax, %rdi
	movq	512(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8020:
.LBB40_282:
	.loc	25 443 13
	leaq	4(%rax), %rsi
	leaq	.Lalloc_d0dbd696a058609bd94bbe1fa75d0723(%rip), %rcx
	movq	%rax, %rdi
	movq	304(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8021:
.LBB40_124:
	.loc	6 785 21
	leaq	.Lalloc_6797264598a169e4722ae66c7bc497b8(%rip), %rdx
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8022:
.LBB40_280:
	.loc	6 565 13
	leaq	.Lalloc_835aafef72e8508601474e7b1f4172a9(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8023:
.LBB40_276:
	.loc	25 456 13
	leaq	.Lalloc_1eeef3195352adc58f4bfdb015316fef(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8024:
.LBB40_277:
	.loc	25 456 13
	leaq	.Lalloc_b9d56679ca30f2caa500ddf2e89d00cb(%rip), %rcx
	movq	%r10, %rdi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8025:
.LBB40_278:
	.loc	25 443 13
	leaq	.Lalloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7(%rip), %rcx
	movq	%r11, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8026:
.LBB40_279:
	.loc	25 443 13
	leaq	.Lalloc_f198228fd40f4c04a48a745576c5c5ee(%rip), %rcx
	movq	%r11, %rdi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8027:
.LBB40_283:
	.loc	25 456 13
	leaq	.Lalloc_1eeef3195352adc58f4bfdb015316fef(%rip), %rcx
	movq	%r11, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8028:
.LBB40_284:
	.loc	25 0 13 is_stmt 0
	movabsq	$2305843009213693948, %rsi
	movq	80(%rsp), %rdx
.Ltmp8029:
	.loc	11 900 12 is_stmt 1
	andq	%rdx, %rsi
	addq	$4, %rsi
.Ltmp8030:
	.loc	25 443 13
	leaq	.Lalloc_d9529ff5ddc99dd60299cff5ff3cd676(%rip), %rcx
	movq	%rax, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8031:
.LBB40_285:
	.loc	25 456 13
	leaq	.Lalloc_b9d56679ca30f2caa500ddf2e89d00cb(%rip), %rcx
	movq	%r11, %rdi
	movq	%r12, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8032:
.LBB40_286:
	.loc	25 443 13
	leaq	.Lalloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7(%rip), %rcx
	movq	%r15, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8033:
.LBB40_287:
	.loc	25 443 13
	leaq	.Lalloc_f198228fd40f4c04a48a745576c5c5ee(%rip), %rcx
	movq	%r15, %rdi
	movq	%r12, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8034:
.LBB40_272:
	.loc	18 968 21
	leaq	.Lalloc_376120b9c5efdf3d59386c16952a74b7(%rip), %rdi
	leaq	.Lalloc_99ab90c4f654517a4481f4facf03e67c(%rip), %rdx
	movl	$31, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core6option13expect_failed@GOTPCREL(%rip)
.Ltmp8035:
.LBB40_273:
	.loc	25 456 13
	leaq	.Lalloc_dd5f55065f566218c9f31cb2a4357231(%rip), %rcx
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8036:
.LBB40_274:
	.loc	25 456 13
	leaq	.Lalloc_1d7cc6e40c752396aa7def7556a6c433(%rip), %rcx
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8037:
.LBB40_281:
	.loc	25 581 13
	leaq	.Lalloc_1e79f4c3c2f015f90ab54e70b61044ab(%rip), %rcx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8038:
.LBB40_213:
	.loc	25 581 13
	leaq	.Lalloc_a6d4388bd1c2ee005f6a969a0e3ca0f4(%rip), %rcx
	movq	%r8, %rsi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8039:
.LBB40_288:
	.loc	6 1090 33
	leaq	.Lalloc_99ab90c4f654517a4481f4facf03e67c(%rip), %rdx
	movq	%r9, %rdi
	movq	%r9, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_289:
.Ltmp8040:
	.loc	6 1091 31
	leaq	.Lalloc_99ab90c4f654517a4481f4facf03e67c(%rip), %rdx
	movq	%rcx, %rdi
	movq	%r9, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8041:
.LBB40_89:
	.loc	6 0 31 is_stmt 0
	movq	%rsi, %rcx
.Ltmp8042:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp8043:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_61:
	movq	%r13, %rcx
.Ltmp8044:
	.loc	21 237 33 is_stmt 1
	jmp	.LBB40_46
.Ltmp8045:
.LBB40_79:
	.loc	21 0 33 is_stmt 0
	movq	%r14, %rcx
.LBB40_80:
.Ltmp8046:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp8047:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_53:
	movq	%r14, %rcx
.Ltmp8048:
	.loc	21 237 33 is_stmt 1
	jmp	.LBB40_46
.Ltmp8049:
.LBB40_91:
	.loc	21 0 33 is_stmt 0
	movq	%rsi, %rcx
.Ltmp8050:
	.loc	21 269 33 is_stmt 1
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%rcx, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8051:
.LBB40_67:
	.loc	21 0 33 is_stmt 0
	movq	%rsi, %r12
.Ltmp8052:
	.loc	21 251 32 is_stmt 1
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
.Ltmp8053:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_55:
	movq	%rsi, %r12
.Ltmp8054:
	.loc	21 236 32 is_stmt 1
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
.Ltmp8055:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_93:
	movq	%r10, %r12
.Ltmp8056:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
.Ltmp8057:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_69:
	movq	%r10, %rcx
.Ltmp8058:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp8059:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_82:
	movq	%r14, %rcx
.LBB40_83:
.Ltmp8060:
	.loc	21 269 33 is_stmt 1
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%rcx, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8061:
.LBB40_42:
	.loc	21 0 33 is_stmt 0
	movq	%r11, %r12
.LBB40_43:
.Ltmp8062:
	.loc	21 251 32 is_stmt 1
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
.Ltmp8063:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_57:
	movq	%r15, %rcx
.LBB40_46:
.Ltmp8064:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
.Ltmp8065:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_37:
	movq	%r11, %r12
.LBB40_38:
.Ltmp8066:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
.Ltmp8067:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_95:
	movq	%r10, %r12
.Ltmp8068:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	%r12, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8069:
.LBB40_71:
	.loc	21 0 33 is_stmt 0
	movq	%rdi, %r12
.Ltmp8070:
	.loc	21 251 32 is_stmt 1
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
.Ltmp8071:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_85:
	movq	%r13, %r12
.Ltmp8072:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
.Ltmp8073:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_59:
	movq	%rdi, %r12
.Ltmp8074:
	.loc	21 236 32 is_stmt 1
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
.Ltmp8075:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_73:
	movq	%r13, %rcx
.Ltmp8076:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp8077:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_87:
	movq	%r13, %r12
.Ltmp8078:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	%r12, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8079:
.LBB40_64:
	.loc	21 0 33 is_stmt 0
	movq	%r14, %rcx
.LBB40_65:
.Ltmp8080:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp8081:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_76:
	movq	%r11, %r12
.LBB40_77:
.Ltmp8082:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	%r12, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_97:
	.loc	21 0 33 is_stmt 0
	movq	%rdi, %rcx
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp8083:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_51:
	movq	%r11, %r12
.LBB40_48:
.Ltmp8084:
	.loc	21 236 32 is_stmt 1
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
.Ltmp8085:
	.loc	21 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	80(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8086:
.LBB40_181:
	movq	%r9, %rcx
.Ltmp8087:
	.loc	21 251 32 is_stmt 1
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
.Ltmp8088:
	.loc	21 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_203:
	movq	%r11, %rcx
.Ltmp8089:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
.Ltmp8090:
	.loc	21 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_147:
	movq	%r9, %rcx
.LBB40_148:
.Ltmp8091:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
.Ltmp8092:
	.loc	21 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_169:
	movq	%r9, %rcx
.Ltmp8093:
	.loc	21 236 32 is_stmt 1
	jmp	.LBB40_158
.Ltmp8094:
.LBB40_192:
	.loc	21 0 32 is_stmt 0
	movq	%r15, %r14
.Ltmp8095:
	.loc	21 269 33 is_stmt 1
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%r14, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8096:
.LBB40_183:
	.loc	21 0 33 is_stmt 0
	movq	%r13, %r14
.Ltmp8097:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp8098:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_205:
	movq	%r11, %rcx
.Ltmp8099:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
.Ltmp8100:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_174:
	movq	%r15, %r14
.LBB40_175:
.Ltmp8101:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp8102:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_195:
	movq	%r13, %rcx
.Ltmp8103:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
.Ltmp8104:
	.loc	21 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_161:
	movq	%rdi, %rcx
.Ltmp8105:
	.loc	21 236 32 is_stmt 1
	jmp	.LBB40_158
.Ltmp8106:
.LBB40_207:
	.loc	21 0 32 is_stmt 0
	movq	%rdi, %r14
.Ltmp8107:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp8108:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_186:
	movq	%r9, %rcx
.LBB40_187:
.Ltmp8109:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
.Ltmp8110:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_171:
	movq	%r13, %r14
.Ltmp8111:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
.Ltmp8112:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_197:
	movq	%r13, %rcx
.Ltmp8113:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
.Ltmp8114:
	.loc	6 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_163:
	movq	%r15, %r14
.Ltmp8115:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
.Ltmp8116:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_209:
	movq	%rdi, %r14
.Ltmp8117:
	.loc	21 269 33 is_stmt 1
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%r14, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8118:
.LBB40_177:
	.loc	21 0 33 is_stmt 0
	movq	%rsi, %rcx
.Ltmp8119:
	.loc	21 251 32 is_stmt 1
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
.Ltmp8120:
	.loc	21 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_199:
	movq	%rsi, %r14
.Ltmp8121:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp8122:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_165:
	movq	%rsi, %rcx
.LBB40_158:
.Ltmp8123:
	.loc	21 236 32 is_stmt 1
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
.Ltmp8124:
	.loc	21 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_189:
	movq	%r15, %r14
.LBB40_190:
.Ltmp8125:
	.loc	21 268 33 is_stmt 1
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp8126:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_179:
	movq	%r11, %r14
.Ltmp8127:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp8128:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_201:
	movq	%rsi, %r14
.LBB40_193:
.Ltmp8129:
	.loc	21 269 33 is_stmt 1
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%r14, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8130:
.LBB40_167:
	.loc	21 0 33 is_stmt 0
	movq	%r11, %r14
.LBB40_156:
.Ltmp8131:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
.Ltmp8132:
	.loc	21 0 0 is_stmt 0
	movq	%r14, %rdi
	movq	40(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_152:
	movq	%rdi, %rcx
.LBB40_153:
.Ltmp8133:
	.loc	21 251 32 is_stmt 1
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
.Ltmp8134:
	.loc	21 0 0 is_stmt 0
	movq	%rcx, %rdi
	movq	56(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8135:
.Lfunc_end40:
	.size	_RNvXsd_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank, .Lfunc_end40-_RNvXsd_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank
