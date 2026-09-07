_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process:
.Lfunc_begin32:
	.loc	1 1823 0
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
	subq	$952, %rsp
	.cfi_def_cfa_offset 1008
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rsi, %r14
.Ltmp1188:
	.loc	1 1827 13 prologue_end
	movq	32(%rdx), %r12
	movq	40(%rdx), %rax
	.loc	1 1828 13
	movl	92(%rsi), %esi
	movq	%rdx, 56(%rsp)
	.loc	1 1829 13
	movq	80(%rdx), %rdx
	movl	$0, 712(%rsp)
	movl	$0, 720(%rsp)
	movl	$0, 728(%rsp)
	movl	$0, 736(%rsp)
	movl	$0, 744(%rsp)
	movl	$0, 752(%rsp)
	movl	$0, 760(%rsp)
	movl	$0, 768(%rsp)
	movl	$0, 776(%rsp)
	movl	$0, 784(%rsp)
	movl	$0, 792(%rsp)
	movl	$0, 800(%rsp)
	movl	$0, 808(%rsp)
	movl	$0, 816(%rsp)
	movl	$0, 824(%rsp)
	movl	$0, 832(%rsp)
	movl	$0, 840(%rsp)
	movl	$0, 848(%rsp)
	movl	$0, 856(%rsp)
	movl	$0, 864(%rsp)
.Ltmp1189:
	.loc	32 1714 9
	testq	%rax, %rax
	movq	%rdi, 872(%rsp)
.Ltmp1190:
	.loc	33 180 28
	je	.LBB32_1
.Ltmp1191:
	.loc	33 0 28 is_stmt 0
	leaq	792(%rsp), %r9
	leaq	(%rax,%rax,4), %rax
	leaq	(%r12,%rax,8), %rbx
	movb	$1, %al
	movl	%eax, 20(%rsp)
	xorl	%ebp, %ebp
	leaq	712(%rsp), %r11
	xorl	%eax, %eax
	xorl	%r13d, %r13d
.LBB32_3:
	movq	%rax, %r8
	jmp	.LBB32_4
	.p2align	4
.LBB32_94:
	addq	$40, %r12
.Ltmp1192:
	.loc	38 2428 13 is_stmt 1
	incq	%r8
	movq	$-1, %rax
	cmoveq	%rax, %r8
.Ltmp1193:
	.loc	4 82 9
	incq	%r13
	movq	%r8, %rbp
.Ltmp1194:
	.loc	32 1714 9
	cmpq	%rbx, %r12
.Ltmp1195:
	.loc	33 180 28
	je	.LBB32_95
.Ltmp1196:
.LBB32_4:
	.loc	1 1354 30
	movl	32(%r12), %eax
	.loc	1 1354 24 is_stmt 0
	cmpl	$1, %eax
	je	.LBB32_5
	cmpl	$2, %eax
	jne	.LBB32_94
	.loc	1 0 24
	movl	$1, %eax
	movq	%r9, %r15
.Ltmp1197:
	.loc	1 1362 29 is_stmt 1
	movl	16(%r12), %edi
	cmpq	$2, %rdi
.Ltmp1198:
	.loc	38 1050 16
	jae	.LBB32_80
.Ltmp1199:
.LBB32_79:
	.loc	38 0 16 is_stmt 0
	xorl	%ecx, %ecx
	cmpq	%rsi, %r13
.Ltmp1200:
	.loc	1 1370 25 is_stmt 1
	jb	.LBB32_82
	jmp	.LBB32_94
.Ltmp1201:
	.loc	1 0 25 is_stmt 0
.Ltmp1202:
	.p2align	4
.LBB32_5:
	xorl	%eax, %eax
	movq	%r11, %r15
	.loc	1 1362 29 is_stmt 1
	movl	16(%r12), %edi
	cmpq	$2, %rdi
.Ltmp1203:
	.loc	38 1050 16
	jb	.LBB32_79
.LBB32_80:
	.loc	38 1054 31
	leaq	-2(%rdi), %rcx
	movq	%rcx, 304(%rsp)
.Ltmp1204:
	.loc	28 1580 16
	xorl	%ecx, %ecx
	cmpl	$12, %edi
	setb	%cl
	cmpq	%rsi, %r13
.Ltmp1205:
	.loc	1 1370 25
	jae	.LBB32_94
.LBB32_82:
	cmpq	$1, %rcx
	jne	.LBB32_94
	.loc	1 1372 20
	cmpl	$1, 28(%r12)
	jne	.LBB32_94
	.loc	1 1373 20
	cmpq	%rdx, (%r12)
	jne	.LBB32_94
	.loc	1 1374 20
	cmpq	%rdx, 8(%r12)
	jne	.LBB32_94
	.loc	1 1375 20
	vmovd	20(%r12), %xmm0
.Ltmp1206:
	.loc	23 1244 18
	vmovd	%xmm0, %ecx
.Ltmp1207:
	.loc	1 1375 20
	cmpl	%ecx, 24(%r12)
	jne	.LBB32_94
	.loc	1 0 20 is_stmt 0
	movq	%rdx, 72(%rsp)
	movq	%rsi, 80(%rsp)
	movq	%r8, 704(%rsp)
	.loc	1 1376 43 is_stmt 1
	cmpl	$11, %edi
	ja	.LBB32_606
	.loc	1 0 43 is_stmt 0
	leal	(%rax,%rdi,2), %eax
	movl	%eax, 32(%rsp)
	.loc	1 1376 42
	leaq	(%rdi,%rdi,4), %rax
	leaq	.Lalloc_cc33a3b9cd8c16d253f2168b5461d31d(%rip), %rcx
	leaq	(%rcx,%rax,8), %rdi
	vmovdqa	%xmm0, 432(%rsp)
	.loc	1 1376 20
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	432(%rsp), %xmm1
	movl	32(%rsp), %r10d
	cmpl	28(%rsp), %r10d
	seta	%cl
	testb	%al, %al
	movq	704(%rsp), %r8
	movq	80(%rsp), %rsi
	movq	72(%rsp), %rdx
	leaq	792(%rsp), %r9
	leaq	712(%rsp), %r11
	je	.LBB32_94
	orb	20(%rsp), %cl
	testb	$1, %cl
	je	.LBB32_94
	.loc	1 0 20
	movq	304(%rsp), %rdi
.Ltmp1208:
	.loc	1 1378 45 is_stmt 1
	cmpq	$9, %rdi
	ja	.LBB32_607
.Ltmp1209:
	.file	47 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/macros/mod.rs"
	.loc	47 430 9
	cmpl	$0, (%r15,%rdi,8)
.Ltmp1210:
	.loc	1 1383 17
	jne	.LBB32_94
.Ltmp1211:
	.loc	31 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
	movq	304(%rsp), %rax
.Ltmp1212:
	.loc	1 1388 13
	movl	$1, (%r15,%rax,8)
	vmovss	%xmm0, 4(%r15,%rax,8)
.Ltmp1213:
	.loc	32 1714 9
	addq	$40, %r12
.Ltmp1214:
	.loc	33 180 28
	incq	%r13
	movl	$0, 20(%rsp)
	movq	%r8, %rax
	movl	%r10d, 28(%rsp)
	movq	%rbp, %r8
.Ltmp1215:
	.loc	32 1714 9
	cmpq	%rbx, %r12
.Ltmp1216:
	.loc	33 180 28
	jne	.LBB32_3
	jmp	.LBB32_95
.Ltmp1217:
.LBB32_1:
	.loc	33 0 28 is_stmt 0
	xorl	%r8d, %r8d
.LBB32_95:
.Ltmp1218:
	.loc	33 180 28 is_stmt 1
	leaq	348(%r14), %rax
	movl	$76, %ecx
	vmovss	.LCPI32_0(%rip), %xmm0
	jmp	.LBB32_9
.Ltmp1219:
	.loc	33 0 28 is_stmt 0
.Ltmp1220:
	.p2align	4
.LBB32_6:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_7:
	.loc	36 0 0
	vmovd	%xmm1, -4(%rax)
	movl	%edx, (%rax)
.Ltmp1221:
.LBB32_8:
	.loc	32 1714 9 is_stmt 1
	addq	$80, %rcx
	addq	$360, %rax
	cmpq	$236, %rcx
.Ltmp1222:
	.loc	33 180 28
	je	.LBB32_96
.Ltmp1223:
.LBB32_9:
	.loc	1 1392 24
	cmpl	$1, 636(%rsp,%rcx)
	je	.LBB32_10
	cmpl	$1, 644(%rsp,%rcx)
	je	.LBB32_17
.LBB32_23:
	cmpl	$1, 652(%rsp,%rcx)
	je	.LBB32_24
.LBB32_30:
	cmpl	$1, 660(%rsp,%rcx)
	je	.LBB32_31
.LBB32_37:
	cmpl	$1, 668(%rsp,%rcx)
	je	.LBB32_38
.LBB32_44:
	cmpl	$1, 676(%rsp,%rcx)
	je	.LBB32_45
.LBB32_51:
	cmpl	$1, 684(%rsp,%rcx)
	je	.LBB32_52
.LBB32_58:
	cmpl	$1, 692(%rsp,%rcx)
	je	.LBB32_59
.LBB32_65:
	cmpl	$1, 700(%rsp,%rcx)
	je	.LBB32_66
.LBB32_72:
	cmpl	$1, 708(%rsp,%rcx)
	jne	.LBB32_8
	jmp	.LBB32_73
	.loc	1 0 24 is_stmt 0
.Ltmp1224:
	.p2align	4
.LBB32_10:
	.loc	1 1392 29
	vmovd	640(%rsp,%rcx), %xmm1
.Ltmp1225:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -152(%rax)
	.loc	36 81 48
	vmovd	-156(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1226:
	.loc	36 112 9
	jg	.LBB32_14
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_14
	negl	%edx
	jo	.LBB32_14
.Ltmp1227:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -156(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_15
	.loc	36 0 6 is_stmt 0
.Ltmp1228:
	.p2align	4
.LBB32_14:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_15:
	.loc	36 0 0
	vmovd	%xmm1, -148(%rax)
	movl	%edx, -144(%rax)
.Ltmp1229:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 644(%rsp,%rcx)
	jne	.LBB32_23
.LBB32_17:
	.loc	1 1392 29 is_stmt 0
	vmovd	648(%rsp,%rcx), %xmm1
.Ltmp1230:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -136(%rax)
	.loc	36 81 48
	vmovd	-140(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1231:
	.loc	36 112 9
	jg	.LBB32_21
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_21
	negl	%edx
	jo	.LBB32_21
.Ltmp1232:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -140(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_22
	.loc	36 0 6 is_stmt 0
.Ltmp1233:
	.p2align	4
.LBB32_21:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_22:
	.loc	36 0 0
	vmovd	%xmm1, -132(%rax)
	movl	%edx, -128(%rax)
.Ltmp1234:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 652(%rsp,%rcx)
	jne	.LBB32_30
.LBB32_24:
	.loc	1 1392 29 is_stmt 0
	vmovd	656(%rsp,%rcx), %xmm1
.Ltmp1235:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -120(%rax)
	.loc	36 81 48
	vmovd	-124(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1236:
	.loc	36 112 9
	jg	.LBB32_28
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_28
	negl	%edx
	jo	.LBB32_28
.Ltmp1237:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -124(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_29
	.loc	36 0 6 is_stmt 0
.Ltmp1238:
	.p2align	4
.LBB32_28:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_29:
	.loc	36 0 0
	vmovd	%xmm1, -116(%rax)
	movl	%edx, -112(%rax)
.Ltmp1239:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 660(%rsp,%rcx)
	jne	.LBB32_37
.LBB32_31:
	.loc	1 1392 29 is_stmt 0
	vmovd	664(%rsp,%rcx), %xmm1
.Ltmp1240:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -104(%rax)
	.loc	36 81 48
	vmovd	-108(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1241:
	.loc	36 112 9
	jg	.LBB32_35
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_35
	negl	%edx
	jo	.LBB32_35
.Ltmp1242:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -108(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_36
	.loc	36 0 6 is_stmt 0
.Ltmp1243:
	.p2align	4
.LBB32_35:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_36:
	.loc	36 0 0
	vmovd	%xmm1, -100(%rax)
	movl	%edx, -96(%rax)
.Ltmp1244:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 668(%rsp,%rcx)
	jne	.LBB32_44
.LBB32_38:
	.loc	1 1392 29 is_stmt 0
	vmovd	672(%rsp,%rcx), %xmm1
.Ltmp1245:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -88(%rax)
	.loc	36 81 48
	vmovd	-92(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1246:
	.loc	36 112 9
	jg	.LBB32_42
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_42
	negl	%edx
	jo	.LBB32_42
.Ltmp1247:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -92(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_43
	.loc	36 0 6 is_stmt 0
.Ltmp1248:
	.p2align	4
.LBB32_42:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_43:
	.loc	36 0 0
	vmovd	%xmm1, -84(%rax)
	movl	%edx, -80(%rax)
.Ltmp1249:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 676(%rsp,%rcx)
	jne	.LBB32_51
.LBB32_45:
	.loc	1 1392 29 is_stmt 0
	vmovd	680(%rsp,%rcx), %xmm1
.Ltmp1250:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -72(%rax)
	.loc	36 81 48
	vmovd	-76(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1251:
	.loc	36 112 9
	jg	.LBB32_49
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_49
	negl	%edx
	jo	.LBB32_49
.Ltmp1252:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -76(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_50
	.loc	36 0 6 is_stmt 0
.Ltmp1253:
	.p2align	4
.LBB32_49:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_50:
	.loc	36 0 0
	vmovd	%xmm1, -68(%rax)
	movl	%edx, -64(%rax)
.Ltmp1254:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 684(%rsp,%rcx)
	jne	.LBB32_58
.LBB32_52:
	.loc	1 1392 29 is_stmt 0
	vmovd	688(%rsp,%rcx), %xmm1
.Ltmp1255:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -56(%rax)
	.loc	36 81 48
	vmovd	-60(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1256:
	.loc	36 112 9
	jg	.LBB32_56
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_56
	negl	%edx
	jo	.LBB32_56
.Ltmp1257:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -60(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_57
	.loc	36 0 6 is_stmt 0
.Ltmp1258:
	.p2align	4
.LBB32_56:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_57:
	.loc	36 0 0
	vmovd	%xmm1, -52(%rax)
	movl	%edx, -48(%rax)
.Ltmp1259:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 692(%rsp,%rcx)
	jne	.LBB32_65
.LBB32_59:
	.loc	1 1392 29 is_stmt 0
	vmovd	696(%rsp,%rcx), %xmm1
.Ltmp1260:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -40(%rax)
	.loc	36 81 48
	vmovd	-44(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1261:
	.loc	36 112 9
	jg	.LBB32_63
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_63
	negl	%edx
	jo	.LBB32_63
.Ltmp1262:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -44(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_64
	.loc	36 0 6 is_stmt 0
.Ltmp1263:
	.p2align	4
.LBB32_63:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_64:
	.loc	36 0 0
	vmovd	%xmm1, -36(%rax)
	movl	%edx, -32(%rax)
.Ltmp1264:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 700(%rsp,%rcx)
	jne	.LBB32_72
.LBB32_66:
	.loc	1 1392 29 is_stmt 0
	vmovd	704(%rsp,%rcx), %xmm1
.Ltmp1265:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -24(%rax)
	.loc	36 81 48
	vmovd	-28(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1266:
	.loc	36 112 9
	jg	.LBB32_70
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_70
	negl	%edx
	jo	.LBB32_70
.Ltmp1267:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -28(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_71
	.loc	36 0 6 is_stmt 0
.Ltmp1268:
	.p2align	4
.LBB32_70:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_71:
	.loc	36 0 0
	vmovd	%xmm1, -20(%rax)
	movl	%edx, -16(%rax)
.Ltmp1269:
	.loc	1 1392 24 is_stmt 1
	cmpl	$1, 708(%rsp,%rcx)
	jne	.LBB32_8
.LBB32_73:
	.loc	1 1392 29 is_stmt 0
	vmovd	712(%rsp,%rcx), %xmm1
.Ltmp1270:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -8(%rax)
	.loc	36 81 48
	vmovd	-12(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1271:
	.loc	36 112 9
	jg	.LBB32_6
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_6
	negl	%edx
	jo	.LBB32_6
.Ltmp1272:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -12(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_7
.Ltmp1273:
.LBB32_96:
	.loc	36 0 6 is_stmt 0
	movq	56(%rsp), %rax
	.loc	1 1832 22 is_stmt 1
	movq	8(%rax), %r9
.Ltmp1274:
	.loc	1 1833 12
	testq	%r9, %r9
	je	.LBB32_98
	.loc	1 0 12 is_stmt 0
	movq	56(%rsp), %rax
	.loc	1 1833 12
	cmpq	%r9, 24(%rax)
	jne	.LBB32_98
	.loc	1 0 12
	movq	56(%rsp), %rax
	movq	(%rax), %rsi
	.loc	1 1833 27
	movq	16(%rax), %rdx
.Ltmp1275:
	.loc	1 1142 12 is_stmt 1
	movl	860(%r14), %ecx
	.loc	1 1142 27 is_stmt 0
	movzbl	864(%r14), %eax
	.loc	1 1142 5
	cmpl	$3, %ecx
	movq	%r9, 184(%rsp)
	movq	%r8, 704(%rsp)
	movq	%rdx, 472(%rsp)
	movq	%rsi, 464(%rsp)
	je	.LBB32_442
	cmpl	$2, %ecx
	jne	.LBB32_101
	testb	%al, %al
	jne	.LBB32_263
.Ltmp1276:
	.loc	1 1083 23 is_stmt 1
	movl	856(%r14), %eax
	movl	%eax, 516(%rsp)
.Ltmp1277:
	.loc	1 1084 20
	movq	840(%r14), %r15
	xorl	%ebp, %ebp
	movq	%r15, 480(%rsp)
	jmp	.LBB32_306
.Ltmp1278:
.LBB32_98:
	.loc	1 1834 20
	vxorps	%xmm0, %xmm0, %xmm0
	movq	872(%rsp), %rax
	vmovups	%xmm0, (%rax)
	movq	%r8, 16(%rax)
	vmovups	%xmm0, 24(%rax)
.Ltmp1279:
	.loc	1 1845 6
	jmp	.LBB32_605
	.loc	1 0 6 is_stmt 0
.Ltmp1280:
	.p2align	4
.LBB32_379:
	vmovaps	144(%rsp), %xmm0
.Ltmp1281:
	.loc	1 1057 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	272(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	224(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	240(%rsp), %xmm0
	.loc	1 1058 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	256(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	208(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	12(%rsp), %eax
	.loc	1 1059 5
	movl	%eax, 184(%r14)
	movl	16(%rsp), %eax
	movl	%eax, 188(%r14)
	.loc	1 1060 5
	movl	%ebx, 544(%r14)
	movl	%r13d, 548(%r14)
	.loc	1 1061 5
	movq	%rdi, 848(%r14)
.Ltmp1282:
.LBB32_380:
	.loc	1 0 5 is_stmt 0
	movq	456(%rsp), %rax
	movq	%rax, %rbp
	movq	184(%rsp), %r9
	.loc	1 1086 11 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB32_517
.LBB32_306:
	.loc	1 1087 42
	movq	%r9, %rsi
	subq	%rbp, %rsi
	.loc	1 1087 29 is_stmt 0
	leaq	104(%r14), %rdi
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r13
	vmovss	192(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	200(%r14), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	208(%r14), %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	216(%r14), %xmm0
	vmovss	%xmm0, 412(%rsp)
	vmovss	224(%r14), %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	232(%r14), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	240(%r14), %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	248(%r14), %xmm0
	vmovss	%xmm0, 404(%rsp)
	vmovss	256(%r14), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	264(%r14), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	272(%r14), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	280(%r14), %xmm0
	vmovss	%xmm0, 420(%rsp)
	vmovss	288(%r14), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	296(%r14), %xmm0
	vmovss	%xmm0, 396(%rsp)
	vmovss	304(%r14), %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	312(%r14), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	320(%r14), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	328(%r14), %xmm0
	vmovss	%xmm0, 388(%rsp)
	vmovss	336(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	344(%r14), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	552(%r14), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	560(%r14), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	568(%r14), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	576(%r14), %xmm0
	vmovss	%xmm0, 380(%rsp)
	vmovss	584(%r14), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	592(%r14), %xmm0
	vmovss	%xmm0, 376(%rsp)
	vmovss	600(%r14), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	608(%r14), %xmm0
	vmovss	%xmm0, 372(%rsp)
	vmovss	616(%r14), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	624(%r14), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	632(%r14), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	640(%r14), %xmm0
	vmovss	%xmm0, 332(%rsp)
	vmovss	648(%r14), %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	656(%r14), %xmm0
	vmovss	%xmm0, 364(%rsp)
	vmovss	664(%r14), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	672(%r14), %xmm0
	vmovss	%xmm0, 360(%rsp)
	vmovss	680(%r14), %xmm0
	vmovss	%xmm0, 64(%rsp)
	vmovss	688(%r14), %xmm0
	vmovss	%xmm0, 356(%rsp)
	vmovss	696(%r14), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	704(%r14), %xmm0
	vmovss	%xmm0, 680(%rsp)
.Ltmp1283:
	.loc	1 1091 31 is_stmt 1
	leaq	520(%rsp), %rdi
	leaq	120(%r14), %rsi
	movl	516(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	712(%rsp), %rdi
	leaq	480(%r14), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovss	520(%rsp), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	524(%rsp), %xmm0
	vmovaps	%xmm0, 544(%rsp)
	vmovss	528(%rsp), %xmm0
	vmovaps	%xmm0, 656(%rsp)
	vmovss	532(%rsp), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	536(%rsp), %xmm0
	vmovaps	%xmm0, 640(%rsp)
	vmovss	540(%rsp), %xmm0
	vmovaps	%xmm0, 624(%rsp)
	vmovss	712(%rsp), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	716(%rsp), %xmm0
	vmovaps	%xmm0, 608(%rsp)
	vmovss	720(%rsp), %xmm0
	vmovaps	%xmm0, 592(%rsp)
	vmovss	724(%rsp), %xmm0
	vmovss	%xmm0, 300(%rsp)
	vmovss	728(%rsp), %xmm0
	vmovaps	%xmm0, 576(%rsp)
	vmovss	732(%rsp), %xmm0
	vmovaps	%xmm0, 560(%rsp)
	movq	%r13, 336(%rsp)
.Ltmp1284:
	.loc	1 0 0 is_stmt 0
	addq	%rbp, %r13
	setb	%cl
	movq	%r13, 456(%rsp)
	cmpq	184(%rsp), %r13
	seta	%al
.Ltmp1285:
	.loc	38 1050 16 is_stmt 1
	orb	%cl, %al
.Ltmp1286:
	.loc	1 1094 12
	testb	$1, %r12b
	je	.LBB32_312
.Ltmp1287:
	.loc	38 1050 16
	testb	%al, %al
	jne	.LBB32_524
.Ltmp1288:
	.loc	1 972 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 272(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
.Ltmp1289:
	.loc	1 973 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
.Ltmp1290:
	.loc	1 974 25
	movl	184(%r14), %eax
	movl	%eax, 12(%rsp)
	movl	188(%r14), %eax
	movl	%eax, 16(%rsp)
.Ltmp1291:
	.loc	1 975 24
	movl	544(%r14), %r13d
	movl	548(%r14), %ebx
.Ltmp1292:
	.loc	1 976 24
	movq	848(%r14), %rdi
.Ltmp1293:
	.loc	2 1916 50
	cmpq	$0, 336(%rsp)
	je	.LBB32_441
.Ltmp1294:
	.loc	2 0 50 is_stmt 0
	movq	464(%rsp), %rax
	leaq	(%rax,%rbp,4), %rsi
	movq	472(%rsp), %rax
	leaq	(%rax,%rbp,4), %r8
	xorl	%ebp, %ebp
	movq	%rsi, 672(%rsp)
	movq	%r8, 504(%rsp)
	.p2align	4
.LBB32_310:
.Ltmp1295:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp1296:
	.loc	1 857 8
	cmpq	%r15, %rax
	jb	.LBB32_311
.Ltmp1297:
	.loc	1 0 8 is_stmt 0
	movq	%r15, %rcx
	jmp	.LBB32_382
	.p2align	4
.LBB32_311:
	xorl	%ecx, %ecx
.LBB32_382:
.Ltmp1298:
	.loc	1 1000 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp1299:
	.file	48 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/slice/index.rs"
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp1300:
	.file	49 "/home/bl/misofm/engine-multiband-detector-access" "crates/lane/src/scalar.rs"
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1301:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rsi,%rbp,4), %xmm0
.Ltmp1302:
	vmovss	(%r8,%rbp,4), %xmm3
.Ltmp1303:
	vmovss	152(%r14), %xmm6
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm4
	vmovaps	272(%rsp), %xmm9
	vsubss	%xmm9, %xmm0, %xmm1
	vmulss	%xmm7, %xmm1, %xmm5
	vmovaps	144(%rsp), %xmm2
	vmovss	%xmm6, 428(%rsp)
	vmulss	%xmm6, %xmm2, %xmm6
	vaddss	%xmm5, %xmm6, %xmm5
	vaddss	%xmm5, %xmm2, %xmm6
	vmulss	%xmm7, %xmm2, %xmm8
	vmulss	%xmm4, %xmm1, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	164(%r14), %xmm6, %xmm15
	vmovss	%xmm1, 140(%rsp)
	vaddss	%xmm1, %xmm9, %xmm6
	vmovaps	224(%rsp), %xmm1
	vsubss	%xmm1, %xmm6, %xmm10
	vmulss	160(%rsp), %xmm7, %xmm6
	vmulss	%xmm4, %xmm10, %xmm4
	vaddss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm1, %xmm6
.Ltmp1304:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm12
	vmovss	520(%r14), %xmm13
	vmovaps	256(%rsp), %xmm1
	vsubss	%xmm1, %xmm3, %xmm14
	vmulss	%xmm12, %xmm14, %xmm8
	vmovaps	240(%rsp), %xmm2
	vmulss	%xmm2, %xmm11, %xmm9
	vaddss	%xmm8, %xmm9, %xmm8
	vaddss	%xmm2, %xmm8, %xmm9
	vmulss	524(%r14), %xmm9, %xmm9
.Ltmp1305:
	.loc	1 1000 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp1306:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp1307:
	.loc	1 1001 30
	movq	144(%r14), %rdx
.Ltmp1308:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_129
.Ltmp1309:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1310:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm0, %xmm15, %xmm0
	vsubss	%xmm6, %xmm0, %xmm0
.Ltmp1311:
	.loc	1 1001 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp1312:
	.loc	49 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp1313:
	.loc	1 1002 28
	movq	488(%r14), %rdx
.Ltmp1314:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp1315:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1316:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm2, %xmm12, %xmm0
	vmulss	%xmm13, %xmm14, %xmm6
	vaddss	%xmm6, %xmm0, %xmm14
	vaddss	%xmm1, %xmm14, %xmm0
	vmovaps	208(%rsp), %xmm1
	vsubss	%xmm1, %xmm0, %xmm15
	vmulss	192(%rsp), %xmm12, %xmm0
	vmulss	%xmm15, %xmm13, %xmm6
	vaddss	%xmm6, %xmm0, %xmm0
	vaddss	%xmm0, %xmm1, %xmm6
.Ltmp1317:
	.loc	1 1002 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp1318:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp1319:
	.loc	1 1003 29
	movq	504(%r14), %rdx
.Ltmp1320:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_587
.Ltmp1321:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1322:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm9, %xmm3
	vsubss	%xmm6, %xmm3, %xmm3
.Ltmp1323:
	.loc	1 1003 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp1324:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r9
	addq	%rdi, %r9
	cmpq	%r15, %r9
	jb	.LBB32_391
.Ltmp1325:
	.loc	49 0 9 is_stmt 0
	movq	%r15, %rax
	jmp	.LBB32_393
	.p2align	4
.LBB32_391:
	xorl	%eax, %eax
.LBB32_393:
	movq	128(%r14), %r11
	subq	%rax, %r9
	cmpq	%r11, %r9
	jae	.LBB32_619
.Ltmp1326:
	.loc	1 1008 34 is_stmt 1
	movq	144(%r14), %rax
	cmpq	%rax, %r9
	jae	.LBB32_620
	.loc	1 0 34 is_stmt 0
	movq	%rcx, 320(%rsp)
	movq	760(%r14), %r10
	addq	%rdi, %r10
	cmpq	%r15, %r10
	jb	.LBB32_396
	movq	%r15, %rcx
	jmp	.LBB32_398
	.p2align	4
.LBB32_396:
	xorl	%ecx, %ecx
.LBB32_398:
	movq	488(%r14), %r12
	subq	%rcx, %r10
	cmpq	%r12, %r10
	jae	.LBB32_621
.Ltmp1327:
	.loc	1 1012 34 is_stmt 1
	movq	504(%r14), %r8
	cmpq	%r8, %r10
	jae	.LBB32_586
	.loc	1 0 34 is_stmt 0
	movl	%ebx, %ecx
	movq	120(%r14), %rdx
	movq	%rdx, 688(%rsp)
	vmovss	(%rdx,%r9,4), %xmm3
	movq	480(%r14), %rdx
	movq	%rdx, 696(%rsp)
	vmovss	(%rdx,%r10,4), %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1328:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp1329:
	.loc	49 103 24 is_stmt 0
	vandps	%xmm1, %xmm6, %xmm6
.Ltmp1330:
	.loc	49 161 24 is_stmt 1
	vmaxss	%xmm6, %xmm3, %xmm3
.Ltmp1331:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r15d
	movl	$841731191, %edx
.Ltmp1332:
	.loc	49 161 24
	jbe	.LBB32_402
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %edx
.LBB32_402:
.Ltmp1333:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp1334:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %ebx
	movl	$8388608, %esi
.Ltmp1335:
	.loc	49 161 24
	jbe	.LBB32_404
.Ltmp1336:
	.loc	49 0 24 is_stmt 0
	movl	%edx, %esi
.LBB32_404:
	vmovss	80(%rsp), %xmm1
	vaddss	424(%rsp), %xmm1, %xmm1
.Ltmp1337:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp1338:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp1339:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp1340:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm6
.Ltmp1341:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm6, %xmm6
.Ltmp1342:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1343:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm6, %xmm6
.Ltmp1344:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1345:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm6, %xmm6
.Ltmp1346:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1347:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm6, %xmm6
.Ltmp1348:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1349:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm6, %xmm6
.Ltmp1350:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp1351:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1352:
	.loc	23 1291 18
	vmovd	%esi, %xmm6
.Ltmp1353:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm6, %xmm6
.Ltmp1354:
	.loc	49 61 9
	vaddss	%xmm3, %xmm6, %xmm3
.Ltmp1355:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp1356:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp1357:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm1, 80(%rsp)
.Ltmp1358:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm3, %xmm6
.Ltmp1359:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm6, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_405
.Ltmp1360:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_407
	.p2align	4
.LBB32_405:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm6, %xmm9
.Ltmp1361:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp1362:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm1, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	136(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %edx
.Ltmp1363:
.LBB32_407:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm6
.Ltmp1364:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp1365:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vminss	%xmm1, %xmm6, %xmm6
.Ltmp1366:
	.loc	1 924 9 is_stmt 1
	vmovss	12(%rsp), %xmm9
.Ltmp1367:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	544(%rsp), %xmm1
	vblendvps	%xmm13, 656(%rsp), %xmm1, %xmm13
.Ltmp1368:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp1369:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1370:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm9
.Ltmp1371:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm1
	vucomiss	%xmm9, %xmm1
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_408
	.loc	49 0 24
	movl	$0, 12(%rsp)
	jmp	.LBB32_410
	.p2align	4
.LBB32_408:
	.loc	49 166 0
	vmovss	%xmm6, 12(%rsp)
.Ltmp1372:
.LBB32_410:
	.loc	49 0 0
	movq	136(%r14), %rsi
	vmovss	(%rsi,%r9,4), %xmm6
	movq	496(%r14), %r9
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1373:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm6, %xmm6
.Ltmp1374:
	.loc	49 103 24 is_stmt 0
	vmovss	(%r9,%r10,4), %xmm9
	vandps	%xmm1, %xmm9, %xmm9
.Ltmp1375:
	.loc	49 161 24 is_stmt 1
	vmaxss	%xmm9, %xmm6, %xmm6
.Ltmp1376:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
.Ltmp1377:
	.loc	49 161 24
	jbe	.LBB32_412
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %r15d
.LBB32_412:
.Ltmp1378:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r15d, %xmm6
.Ltmp1379:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
.Ltmp1380:
	.loc	49 161 24
	jbe	.LBB32_414
.Ltmp1381:
	.loc	49 0 24 is_stmt 0
	movl	%r15d, %ebx
.LBB32_414:
	vmovss	20(%rsp), %xmm1
	vaddss	420(%rsp), %xmm1, %xmm1
.Ltmp1382:
	.loc	49 185 42 is_stmt 1
	movl	%ebx, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp1383:
	.loc	23 1291 18
	vmovd	%edx, %xmm6
.Ltmp1384:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp1385:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp1386:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp1387:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1388:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp1389:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1390:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp1391:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1392:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp1393:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1394:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp1395:
	.loc	49 187 28
	shrl	$23, %ebx
	orl	$1258291200, %ebx
.Ltmp1396:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp1397:
	.loc	23 1291 18
	vmovd	%ebx, %xmm9
.Ltmp1398:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp1399:
	.loc	49 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1400:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp1401:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp1402:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm9
	vmovss	%xmm1, 20(%rsp)
.Ltmp1403:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm9, %xmm6
.Ltmp1404:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm6, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_415
.Ltmp1405:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_417
	.p2align	4
.LBB32_415:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm6, %xmm13
.Ltmp1406:
	vmulss	%xmm13, %xmm13, %xmm13
	vmulss	.LCPI32_18(%rip), %xmm13, %xmm13
.Ltmp1407:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm1, %xmm1
	vblendvps	%xmm1, %xmm13, %xmm6, %xmm1
	vmulss	132(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1408:
.LBB32_417:
	.loc	49 0 0
	movq	480(%rsp), %r15
.Ltmp1409:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1410:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1411:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm6, %xmm6, %xmm6
	vminss	%xmm6, %xmm1, %xmm1
.Ltmp1412:
	.loc	1 924 9 is_stmt 1
	vmovss	16(%rsp), %xmm6
.Ltmp1413:
	.loc	49 161 24
	vcmpnltss	%xmm6, %xmm1, %xmm13
	vmovaps	640(%rsp), %xmm2
	vblendvps	%xmm13, 624(%rsp), %xmm2, %xmm13
.Ltmp1414:
	.loc	49 66 9
	vsubss	%xmm1, %xmm6, %xmm6
.Ltmp1415:
	.loc	49 92 9
	vmulss	%xmm6, %xmm13, %xmm6
	vaddss	%xmm6, %xmm1, %xmm6
.Ltmp1416:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm1
.Ltmp1417:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_418
	.loc	49 0 24
	movl	$0, 16(%rsp)
	jmp	.LBB32_420
	.p2align	4
.LBB32_418:
	.loc	49 166 0
	vmovss	%xmm6, 16(%rsp)
.Ltmp1418:
.LBB32_420:
	.loc	49 0 0
	vmovss	56(%rsp), %xmm1
	vaddss	416(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 56(%rsp)
.Ltmp1419:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1420:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_421
.Ltmp1421:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_423
	.p2align	4
.LBB32_421:
	vmovss	.LCPI32_17(%rip), %xmm6
	vaddss	%xmm6, %xmm3, %xmm1
.Ltmp1422:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1423:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm6, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vmulss	128(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1424:
.LBB32_423:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1425:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1426:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1427:
	.loc	1 924 9 is_stmt 1
	vmovd	%r13d, %xmm3
.Ltmp1428:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm6
	vmovaps	608(%rsp), %xmm2
	vblendvps	%xmm6, 592(%rsp), %xmm2, %xmm6
.Ltmp1429:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1430:
	.loc	49 92 9
	vmulss	%xmm6, %xmm3, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1431:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1432:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm6
	vucomiss	%xmm1, %xmm6
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_424
	.loc	49 0 24
	xorl	%r13d, %r13d
	jmp	.LBB32_426
	.p2align	4
.LBB32_424:
	.loc	49 166 0
	vmovd	%xmm3, %r13d
.Ltmp1433:
.LBB32_426:
	.loc	49 0 0
	vmovss	304(%rsp), %xmm1
	vaddss	332(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 304(%rsp)
.Ltmp1434:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm9, %xmm3
.Ltmp1435:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_427
.Ltmp1436:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_429
	.p2align	4
.LBB32_427:
	vmovss	.LCPI32_17(%rip), %xmm6
	vaddss	%xmm6, %xmm3, %xmm1
.Ltmp1437:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1438:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm6, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vmulss	300(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1439:
.LBB32_429:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1440:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1441:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1442:
	.loc	1 924 9 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp1443:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm6
	vmovaps	576(%rsp), %xmm2
	vblendvps	%xmm6, 560(%rsp), %xmm2, %xmm6
.Ltmp1444:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1445:
	.loc	49 92 9
	vmulss	%xmm6, %xmm3, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1446:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1447:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm6
	vucomiss	%xmm1, %xmm6
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_430
	.loc	49 0 24
	xorl	%ebx, %ebx
	jmp	.LBB32_432
	.p2align	4
.LBB32_430:
	.loc	49 166 0
	vmovd	%xmm3, %ebx
.Ltmp1448:
.LBB32_432:
	.loc	49 0 0
	movq	320(%rsp), %rcx
	negq	%rcx
	addq	%rcx, %rdi
	incq	%rdi
.Ltmp1449:
	.loc	48 568 12 is_stmt 1
	cmpq	%r11, %rdi
	ja	.LBB32_588
.Ltmp1450:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp1451:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_190
.Ltmp1452:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp1453:
	.loc	48 568 12
	cmpq	%r12, %rdi
	ja	.LBB32_589
.Ltmp1454:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp1455:
	.loc	48 568 12
	cmpq	%r8, %rdi
	ja	.LBB32_590
.Ltmp1456:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp1457:
	.loc	49 0 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm1
	vmovaps	160(%rsp), %xmm13
	vmulss	428(%rsp), %xmm13, %xmm2
	vaddss	%xmm1, %xmm2, %xmm1
	vmulss	%xmm15, %xmm12, %xmm2
	vmovaps	192(%rsp), %xmm12
	vmulss	%xmm11, %xmm12, %xmm3
	vaddss	%xmm2, %xmm3, %xmm2
	vaddss	%xmm5, %xmm5, %xmm3
	vaddss	144(%rsp), %xmm3, %xmm7
	vmovss	140(%rsp), %xmm3
	vaddss	%xmm3, %xmm3, %xmm3
	vaddss	272(%rsp), %xmm3, %xmm5
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	%xmm4, %xmm4, %xmm3
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm14, %xmm14, %xmm6
	vaddss	%xmm2, %xmm2, %xmm9
	vaddss	%xmm0, %xmm0, %xmm10
	vaddss	%xmm1, %xmm13, %xmm8
	vaddss	224(%rsp), %xmm3, %xmm0
	vaddss	240(%rsp), %xmm4, %xmm1
	vaddss	256(%rsp), %xmm6, %xmm2
	vaddss	%xmm9, %xmm12, %xmm3
	vaddss	208(%rsp), %xmm10, %xmm4
	vmovss	28(%rsp), %xmm9
	vaddss	400(%rsp), %xmm9, %xmm9
	vmovss	12(%rsp), %xmm6
	vmovss	%xmm9, 28(%rsp)
	vaddss	%xmm6, %xmm9, %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm14
	vandps	%xmm7, %xmm14, %xmm9
	vandps	%xmm5, %xmm14, %xmm10
	vandps	%xmm14, %xmm8, %xmm11
	vandps	%xmm0, %xmm14, %xmm12
	vandps	%xmm1, %xmm14, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm15
	vcmpnltss	%xmm15, %xmm9, %xmm9
	vandps	%xmm7, %xmm9, %xmm7
	vmovaps	%xmm7, 144(%rsp)
	vandps	%xmm2, %xmm14, %xmm7
	vcmpnltss	%xmm15, %xmm10, %xmm9
	vandps	%xmm5, %xmm9, %xmm5
	vmovaps	%xmm5, 272(%rsp)
	vandps	%xmm3, %xmm14, %xmm5
	vcmpnltss	%xmm15, %xmm11, %xmm9
	vandps	%xmm8, %xmm9, %xmm8
	vmovaps	%xmm8, 160(%rsp)
	vandps	%xmm4, %xmm14, %xmm8
	vmovss	.LCPI32_21(%rip), %xmm14
	vmulss	%xmm6, %xmm14, %xmm6
	vcmpnltss	%xmm15, %xmm12, %xmm9
	vcmpnltss	%xmm15, %xmm13, %xmm10
	vcmpnltss	%xmm15, %xmm7, %xmm7
	vcmpnltss	%xmm15, %xmm5, %xmm5
	vcmpnltss	%xmm15, %xmm8, %xmm8
	vmovss	.LCPI32_22(%rip), %xmm12
	vmaxss	%xmm12, %xmm6, %xmm6
	vandps	%xmm0, %xmm9, %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vandps	%xmm1, %xmm10, %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vandps	%xmm2, %xmm7, %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vandps	%xmm3, %xmm5, %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vandps	%xmm4, %xmm8, %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm11
	vminss	%xmm11, %xmm6, %xmm0
	vroundss	$9, %xmm0, %xmm0, %xmm2
	vmovss	432(%rsp), %xmm5
	vaddss	368(%rsp), %xmm5, %xmm5
	vmovss	32(%rsp), %xmm1
	vaddss	384(%rsp), %xmm1, %xmm1
	vsubss	%xmm2, %xmm0, %xmm3
	vmovss	16(%rsp), %xmm0
	vmovss	%xmm1, 32(%rsp)
	vaddss	%xmm0, %xmm1, %xmm0
	vmulss	%xmm0, %xmm14, %xmm0
	vmaxss	%xmm12, %xmm0, %xmm0
	vminss	%xmm11, %xmm0, %xmm1
	vroundss	$9, %xmm1, %xmm1, %xmm7
	vsubss	%xmm7, %xmm1, %xmm1
	vmovd	%r13d, %xmm4
	vmovss	%xmm5, 432(%rsp)
	vaddss	%xmm4, %xmm5, %xmm4
	vmulss	%xmm4, %xmm14, %xmm4
	vmaxss	%xmm12, %xmm4, %xmm4
	vminss	%xmm11, %xmm4, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm6
	vmulss	%xmm6, %xmm3, %xmm5
	vmovss	.LCPI32_25(%rip), %xmm8
	vaddss	%xmm5, %xmm8, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_26(%rip), %xmm9
	vaddss	%xmm5, %xmm9, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_27(%rip), %xmm10
	vaddss	%xmm5, %xmm10, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_28(%rip), %xmm13
	vaddss	%xmm5, %xmm13, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vroundss	$9, %xmm4, %xmm4, %xmm3
	vsubss	%xmm3, %xmm4, %xmm4
	vmovss	72(%rsp), %xmm0
	vaddss	680(%rsp), %xmm0, %xmm0
	vmovss	.LCPI32_29(%rip), %xmm14
	vaddss	%xmm5, %xmm14, %xmm5
	vmovss	.LCPI32_30(%rip), %xmm15
	vaddss	%xmm2, %xmm15, %xmm2
	vmovd	%xmm2, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm2
	vmulss	%xmm2, %xmm5, %xmm2
	vmulss	%xmm6, %xmm1, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
	vmulss	%xmm5, %xmm1, %xmm5
	vaddss	%xmm5, %xmm9, %xmm5
	vmulss	%xmm5, %xmm1, %xmm5
	vaddss	%xmm5, %xmm10, %xmm5
	vmulss	%xmm5, %xmm1, %xmm5
	vaddss	%xmm5, %xmm13, %xmm5
.Ltmp1458:
	vmulss	%xmm5, %xmm1, %xmm1
.Ltmp1459:
	vmovd	%ebx, %xmm5
	vmovss	%xmm0, 72(%rsp)
	vaddss	%xmm5, %xmm0, %xmm5
	vmulss	.LCPI32_21(%rip), %xmm5, %xmm5
	vmaxss	%xmm12, %xmm5, %xmm5
	vminss	%xmm11, %xmm5, %xmm5
.Ltmp1460:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm7, %xmm15, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp1461:
	vmulss	%xmm6, %xmm4, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm9, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm10, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp1462:
	vmulss	%xmm1, %xmm4, %xmm1
.Ltmp1463:
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
.Ltmp1464:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm3, %xmm15, %xmm3
	vmovd	%xmm3, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm3
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp1465:
	vmulss	%xmm6, %xmm5, %xmm3
	vaddss	%xmm3, %xmm8, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm9, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp1466:
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm4, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	688(%rsp), %rax
.Ltmp1467:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm2
	vmulss	(%rsi,%rdi,4), %xmm0, %xmm0
.Ltmp1468:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp1469:
	vaddss	%xmm0, %xmm2, %xmm0
	movq	696(%rsp), %rax
.Ltmp1470:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
.Ltmp1471:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r9,%rdi,4), %xmm3, %xmm2
.Ltmp1472:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	672(%rsp), %rsi
.Ltmp1473:
	.loc	49 56 9
	vmovss	%xmm0, (%rsi,%rbp,4)
	movq	504(%rsp), %r8
.Ltmp1474:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r8,%rbp,4)
	vmovss	36(%rsp), %xmm0
	vaddss	412(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	96(%rsp), %xmm0
	vaddss	380(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	40(%rsp), %xmm0
	vaddss	408(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	104(%rsp), %xmm0
	vaddss	376(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	44(%rsp), %xmm0
	vaddss	404(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	112(%rsp), %xmm0
	vaddss	372(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	48(%rsp), %xmm0
	vaddss	396(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	120(%rsp), %xmm0
	vaddss	364(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	52(%rsp), %xmm0
	vaddss	392(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	24(%rsp), %xmm0
	vaddss	360(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	88(%rsp), %xmm0
	vaddss	388(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	64(%rsp), %xmm0
	vaddss	356(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 64(%rsp)
	incq	%rbp
.Ltmp1475:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rbp, 336(%rsp)
.Ltmp1476:
	.loc	3 900 12
	jne	.LBB32_310
.Ltmp1477:
.LBB32_441:
	.loc	3 0 12 is_stmt 0
	vmovaps	144(%rsp), %xmm0
	.loc	1 1057 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	272(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	224(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	240(%rsp), %xmm0
	.loc	1 1058 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	256(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	208(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	12(%rsp), %eax
	.loc	1 1059 5
	movl	%eax, 184(%r14)
	movl	16(%rsp), %eax
	movl	%eax, 188(%r14)
	.loc	1 1060 5
	movl	%r13d, 544(%r14)
	movl	%ebx, 548(%r14)
	.loc	1 1061 5
	movq	%rdi, 848(%r14)
	vmovss	80(%rsp), %xmm0
.Ltmp1478:
	.loc	1 1197 17
	vmovss	%xmm0, 192(%r14)
	.loc	1 1198 34
	movl	204(%r14), %eax
	movq	336(%rsp), %rsi
.Ltmp1479:
	.loc	38 2472 13
	subl	%esi, %eax
	movl	$0, %edx
	cmovbl	%edx, %eax
.Ltmp1480:
	.loc	1 1198 34
	movl	220(%r14), %ecx
	.loc	1 1198 17 is_stmt 0
	movl	%eax, 204(%r14)
	vmovss	36(%rsp), %xmm0
	.loc	1 1197 17 is_stmt 1
	vmovss	%xmm0, 208(%r14)
.Ltmp1481:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp1482:
	.loc	1 1198 17
	movl	%ecx, 220(%r14)
	vmovss	40(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 224(%r14)
	.loc	1 1198 34
	movl	236(%r14), %eax
.Ltmp1483:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1484:
	.loc	1 1198 34
	movl	252(%r14), %ecx
	.loc	1 1198 17 is_stmt 0
	movl	%eax, 236(%r14)
	vmovss	44(%rsp), %xmm0
	.loc	1 1197 17 is_stmt 1
	vmovss	%xmm0, 240(%r14)
.Ltmp1485:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp1486:
	.loc	1 1198 17
	movl	%ecx, 252(%r14)
	vmovss	28(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 256(%r14)
	.loc	1 1198 34
	movl	268(%r14), %eax
.Ltmp1487:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1488:
	.loc	1 1198 17
	movl	%eax, 268(%r14)
	vmovss	20(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 272(%r14)
	.loc	1 1198 34
	movl	284(%r14), %eax
.Ltmp1489:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1490:
	.loc	1 1198 17
	movl	%eax, 284(%r14)
	vmovss	48(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 288(%r14)
	.loc	1 1198 34
	movl	300(%r14), %eax
.Ltmp1491:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1492:
	.loc	1 1198 17
	movl	%eax, 300(%r14)
	vmovss	52(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 304(%r14)
	.loc	1 1198 34
	movl	316(%r14), %eax
.Ltmp1493:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1494:
	.loc	1 1198 17
	movl	%eax, 316(%r14)
	vmovss	88(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 320(%r14)
	.loc	1 1198 34
	movl	332(%r14), %eax
.Ltmp1495:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1496:
	.loc	1 1198 17
	movl	%eax, 332(%r14)
	vmovss	32(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 336(%r14)
	.loc	1 1198 34
	movl	348(%r14), %eax
.Ltmp1497:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1498:
	.loc	1 1198 17
	movl	%eax, 348(%r14)
	vmovss	56(%rsp), %xmm0
.Ltmp1499:
	.loc	1 1197 17
	vmovss	%xmm0, 552(%r14)
	.loc	1 1198 34
	movl	564(%r14), %eax
.Ltmp1500:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1501:
	.loc	1 1198 17
	movl	%eax, 564(%r14)
	vmovss	96(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 568(%r14)
	.loc	1 1198 34
	movl	580(%r14), %eax
.Ltmp1502:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1503:
	.loc	1 1198 17
	movl	%eax, 580(%r14)
	vmovss	104(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 584(%r14)
	.loc	1 1198 34
	movl	596(%r14), %eax
.Ltmp1504:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1505:
	.loc	1 1198 17
	movl	%eax, 596(%r14)
	vmovss	112(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 600(%r14)
	.loc	1 1198 34
	movl	612(%r14), %eax
.Ltmp1506:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1507:
	.loc	1 1198 17
	movl	%eax, 612(%r14)
	vmovss	432(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 616(%r14)
	.loc	1 1198 34
	movl	628(%r14), %eax
.Ltmp1508:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1509:
	.loc	1 1198 17
	movl	%eax, 628(%r14)
	vmovss	304(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 632(%r14)
	.loc	1 1198 34
	movl	644(%r14), %eax
.Ltmp1510:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1511:
	.loc	1 1198 17
	movl	%eax, 644(%r14)
	vmovss	120(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 648(%r14)
	.loc	1 1198 34
	movl	660(%r14), %eax
.Ltmp1512:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1513:
	.loc	1 1198 17
	movl	%eax, 660(%r14)
	vmovss	24(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 664(%r14)
	.loc	1 1198 34
	movl	676(%r14), %eax
.Ltmp1514:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1515:
	.loc	1 1198 17
	movl	%eax, 676(%r14)
	vmovss	64(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 680(%r14)
	.loc	1 1198 34
	movl	692(%r14), %eax
.Ltmp1516:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1517:
	.loc	1 1198 17
	movl	%eax, 692(%r14)
	vmovss	72(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 696(%r14)
	.loc	1 1198 34
	movl	708(%r14), %eax
.Ltmp1518:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1519:
	.loc	1 1198 17
	movl	%eax, 708(%r14)
.Ltmp1520:
	.loc	1 1094 9
	jmp	.LBB32_380
	.loc	1 0 9 is_stmt 0
.Ltmp1521:
	.p2align	4
.LBB32_312:
.Ltmp1522:
	.loc	38 1050 16 is_stmt 1
	testb	%al, %al
	jne	.LBB32_317
.Ltmp1523:
	.loc	1 972 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 272(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
.Ltmp1524:
	.loc	1 973 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
.Ltmp1525:
	.loc	1 974 25
	movl	184(%r14), %eax
	movl	%eax, 12(%rsp)
	movl	188(%r14), %eax
	movl	%eax, 16(%rsp)
.Ltmp1526:
	.loc	1 975 24
	movl	544(%r14), %ebx
	movl	548(%r14), %r13d
.Ltmp1527:
	.loc	1 976 24
	movq	848(%r14), %rdi
.Ltmp1528:
	.loc	2 1916 50
	cmpq	$0, 336(%rsp)
.Ltmp1529:
	.loc	3 900 12
	je	.LBB32_379
.Ltmp1530:
	.loc	3 0 12 is_stmt 0
	movq	464(%rsp), %rax
	leaq	(%rax,%rbp,4), %rsi
	movq	472(%rsp), %rax
	leaq	(%rax,%rbp,4), %r8
	xorl	%ebp, %ebp
	movq	%rsi, 96(%rsp)
	movq	%r8, 88(%rsp)
	.p2align	4
.LBB32_315:
.Ltmp1531:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp1532:
	.loc	1 857 8
	cmpq	%r15, %rax
	jb	.LBB32_316
.Ltmp1533:
	.loc	1 0 8 is_stmt 0
	movq	%r15, %rcx
	jmp	.LBB32_320
	.p2align	4
.LBB32_316:
	xorl	%ecx, %ecx
.LBB32_320:
.Ltmp1534:
	.loc	1 1000 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp1535:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp1536:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1537:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rsi,%rbp,4), %xmm6
.Ltmp1538:
	vmovss	(%r8,%rbp,4), %xmm13
.Ltmp1539:
	vmovss	152(%r14), %xmm4
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm0
	vmovaps	272(%rsp), %xmm8
	vsubss	%xmm8, %xmm6, %xmm1
	vmulss	%xmm7, %xmm1, %xmm3
	vmovaps	144(%rsp), %xmm2
	vmovss	%xmm4, 120(%rsp)
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm3, %xmm4, %xmm5
	vaddss	%xmm5, %xmm2, %xmm3
	vmulss	%xmm7, %xmm2, %xmm4
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	%xmm1, %xmm4, %xmm1
	vmulss	164(%r14), %xmm3, %xmm3
	vmovss	%xmm1, 24(%rsp)
	vaddss	%xmm1, %xmm8, %xmm4
	vmovaps	224(%rsp), %xmm1
	vsubss	%xmm1, %xmm4, %xmm10
	vmulss	160(%rsp), %xmm7, %xmm4
	vmulss	%xmm0, %xmm10, %xmm0
	vaddss	%xmm0, %xmm4, %xmm4
	vaddss	%xmm4, %xmm1, %xmm14
.Ltmp1540:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm0
	vmovss	520(%r14), %xmm12
	vmovaps	256(%rsp), %xmm1
	vsubss	%xmm1, %xmm13, %xmm9
	vmulss	%xmm0, %xmm9, %xmm8
	vmovaps	240(%rsp), %xmm2
	vmulss	%xmm2, %xmm11, %xmm15
	vaddss	%xmm8, %xmm15, %xmm8
	vaddss	%xmm2, %xmm8, %xmm15
	vmulss	524(%r14), %xmm15, %xmm15
.Ltmp1541:
	.loc	1 1000 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp1542:
	.loc	49 56 9
	vmovss	%xmm14, (%rax,%rdi,4)
.Ltmp1543:
	.loc	1 1001 30
	movq	144(%r14), %rdx
.Ltmp1544:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_129
.Ltmp1545:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1546:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm6, %xmm3
	vsubss	%xmm14, %xmm3, %xmm3
.Ltmp1547:
	.loc	1 1001 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp1548:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp1549:
	.loc	1 1002 28
	movq	488(%r14), %rdx
.Ltmp1550:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp1551:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1552:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm0, %xmm2, %xmm3
	vmulss	%xmm12, %xmm9, %xmm6
	vaddss	%xmm6, %xmm3, %xmm6
	vaddss	%xmm6, %xmm1, %xmm3
	vmovaps	208(%rsp), %xmm1
	vsubss	%xmm1, %xmm3, %xmm9
	vmulss	192(%rsp), %xmm0, %xmm3
	vmulss	%xmm9, %xmm12, %xmm12
	vaddss	%xmm3, %xmm12, %xmm12
	vaddss	%xmm1, %xmm12, %xmm3
.Ltmp1553:
	.loc	1 1002 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp1554:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp1555:
	.loc	1 1003 29
	movq	504(%r14), %rdx
.Ltmp1556:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_587
.Ltmp1557:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1558:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm15, %xmm13, %xmm13
	vsubss	%xmm3, %xmm13, %xmm3
.Ltmp1559:
	.loc	1 1003 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp1560:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r9
	addq	%rdi, %r9
	cmpq	%r15, %r9
	jb	.LBB32_329
.Ltmp1561:
	.loc	49 0 9 is_stmt 0
	movq	%r15, %rax
	jmp	.LBB32_331
	.p2align	4
.LBB32_329:
	xorl	%eax, %eax
.LBB32_331:
	movq	128(%r14), %r11
	subq	%rax, %r9
	cmpq	%r11, %r9
	jae	.LBB32_619
.Ltmp1562:
	.loc	1 1008 34 is_stmt 1
	movq	144(%r14), %rax
	cmpq	%rax, %r9
	jae	.LBB32_620
	.loc	1 0 34 is_stmt 0
	movq	%rcx, 64(%rsp)
	movq	760(%r14), %r10
	addq	%rdi, %r10
	cmpq	%r15, %r10
	jb	.LBB32_334
	movq	%r15, %rcx
	jmp	.LBB32_336
	.p2align	4
.LBB32_334:
	xorl	%ecx, %ecx
.LBB32_336:
	movq	488(%r14), %r12
	subq	%rcx, %r10
	cmpq	%r12, %r10
	jae	.LBB32_621
.Ltmp1563:
	.loc	1 1012 34 is_stmt 1
	movq	504(%r14), %r8
	cmpq	%r8, %r10
	jae	.LBB32_586
	.loc	1 0 34 is_stmt 0
	movl	%r13d, %ecx
	movq	120(%r14), %rdx
	movq	%rdx, 104(%rsp)
	vmovss	(%rdx,%r9,4), %xmm3
	movq	480(%r14), %rdx
	movq	%rdx, 112(%rsp)
	vmovss	(%rdx,%r10,4), %xmm13
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1564:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp1565:
	.loc	49 103 24 is_stmt 0
	vandps	%xmm1, %xmm13, %xmm13
.Ltmp1566:
	.loc	49 161 24 is_stmt 1
	vmaxss	%xmm13, %xmm3, %xmm3
.Ltmp1567:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r15d
	movl	$841731191, %edx
.Ltmp1568:
	.loc	49 161 24
	jbe	.LBB32_340
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %edx
.LBB32_340:
.Ltmp1569:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp1570:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %r13d
	movl	$8388608, %esi
.Ltmp1571:
	.loc	49 161 24
	jbe	.LBB32_342
.Ltmp1572:
	.loc	49 0 24 is_stmt 0
	movl	%edx, %esi
.LBB32_342:
.Ltmp1573:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp1574:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp1575:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp1576:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm13
.Ltmp1577:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm13, %xmm13
.Ltmp1578:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp1579:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm13, %xmm13
.Ltmp1580:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp1581:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm13, %xmm13
.Ltmp1582:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp1583:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm13, %xmm13
.Ltmp1584:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp1585:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm13, %xmm13
.Ltmp1586:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp1587:
	.loc	23 1291 18
	vmovd	%esi, %xmm14
.Ltmp1588:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm14, %xmm14
.Ltmp1589:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm3
.Ltmp1590:
	.loc	49 61 9
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp1591:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp1592:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp1593:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm13
.Ltmp1594:
	.loc	49 66 9 is_stmt 1
	vsubss	80(%rsp), %xmm13, %xmm3
.Ltmp1595:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_343
.Ltmp1596:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_345
	.p2align	4
.LBB32_343:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm3, %xmm14
.Ltmp1597:
	vmulss	%xmm14, %xmm14, %xmm14
	vmulss	.LCPI32_18(%rip), %xmm14, %xmm14
.Ltmp1598:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm1, %xmm15
	vblendvps	%xmm15, %xmm14, %xmm3, %xmm3
	vmulss	136(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %edx
.Ltmp1599:
.LBB32_345:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp1600:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp1601:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vminss	%xmm1, %xmm3, %xmm3
.Ltmp1602:
	.loc	1 924 9 is_stmt 1
	vmovss	12(%rsp), %xmm14
.Ltmp1603:
	.loc	49 161 24
	vcmpnltss	%xmm14, %xmm3, %xmm15
	vmovaps	544(%rsp), %xmm1
	vblendvps	%xmm15, 656(%rsp), %xmm1, %xmm15
.Ltmp1604:
	.loc	49 66 9
	vsubss	%xmm3, %xmm14, %xmm14
.Ltmp1605:
	.loc	49 92 9
	vmulss	%xmm15, %xmm14, %xmm14
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp1606:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm14
.Ltmp1607:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm1
	vucomiss	%xmm14, %xmm1
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_346
	.loc	49 0 24
	movl	$0, 12(%rsp)
	jmp	.LBB32_348
	.p2align	4
.LBB32_346:
	.loc	49 166 0
	vmovss	%xmm3, 12(%rsp)
.Ltmp1608:
.LBB32_348:
	.loc	49 0 0
	movq	136(%r14), %rsi
	vmovss	(%rsi,%r9,4), %xmm3
	movq	496(%r14), %r9
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1609:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp1610:
	.loc	49 103 24 is_stmt 0
	vmovss	(%r9,%r10,4), %xmm14
	vandps	%xmm1, %xmm14, %xmm14
.Ltmp1611:
	.loc	49 161 24 is_stmt 1
	vmaxss	%xmm14, %xmm3, %xmm3
.Ltmp1612:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
.Ltmp1613:
	.loc	49 161 24
	jbe	.LBB32_350
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %r15d
.LBB32_350:
.Ltmp1614:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r15d, %xmm3
.Ltmp1615:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
.Ltmp1616:
	.loc	49 161 24
	jbe	.LBB32_352
.Ltmp1617:
	.loc	49 0 24 is_stmt 0
	movl	%r15d, %r13d
.LBB32_352:
.Ltmp1618:
	.loc	49 185 42 is_stmt 1
	movl	%r13d, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp1619:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp1620:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp1621:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm14
.Ltmp1622:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm14, %xmm14
.Ltmp1623:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp1624:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm14, %xmm14
.Ltmp1625:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp1626:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm14, %xmm14
.Ltmp1627:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp1628:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm14, %xmm14
.Ltmp1629:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp1630:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm14, %xmm14
.Ltmp1631:
	.loc	49 187 28
	shrl	$23, %r13d
	orl	$1258291200, %r13d
.Ltmp1632:
	.loc	23 1291 18
	vmovd	%r13d, %xmm15
.Ltmp1633:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm15, %xmm15
.Ltmp1634:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm3
.Ltmp1635:
	.loc	49 61 9
	vaddss	%xmm3, %xmm15, %xmm3
.Ltmp1636:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp1637:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp1638:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm15
.Ltmp1639:
	.loc	49 66 9 is_stmt 1
	vsubss	20(%rsp), %xmm15, %xmm3
.Ltmp1640:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_353
.Ltmp1641:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_355
	.p2align	4
.LBB32_353:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm3, %xmm14
.Ltmp1642:
	vmulss	%xmm14, %xmm14, %xmm14
	vmulss	.LCPI32_18(%rip), %xmm14, %xmm14
.Ltmp1643:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm1, %xmm1
	vblendvps	%xmm1, %xmm14, %xmm3, %xmm1
	vmulss	132(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1644:
.LBB32_355:
	.loc	49 0 0
	movq	480(%rsp), %r15
.Ltmp1645:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1646:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1647:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1648:
	.loc	1 924 9 is_stmt 1
	vmovss	16(%rsp), %xmm3
.Ltmp1649:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm14
	vmovaps	640(%rsp), %xmm2
	vblendvps	%xmm14, 624(%rsp), %xmm2, %xmm14
.Ltmp1650:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1651:
	.loc	49 92 9
	vmulss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1652:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1653:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm1, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_356
.Ltmp1654:
	.loc	49 0 24
	movl	$0, 16(%rsp)
.Ltmp1655:
	.loc	49 66 9 is_stmt 1
	vsubss	56(%rsp), %xmm13, %xmm3
.Ltmp1656:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jae	.LBB32_360
.Ltmp1657:
.LBB32_359:
	.loc	49 0 44
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm1
.Ltmp1658:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1659:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm1, %xmm3, %xmm1
	vmulss	128(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1660:
	.loc	49 161 44
	jmp	.LBB32_361
.Ltmp1661:
	.loc	49 0 44
.Ltmp1662:
	.p2align	4
.LBB32_356:
	.loc	49 166 0 is_stmt 1
	vmovss	%xmm3, 16(%rsp)
.Ltmp1663:
	.loc	49 66 9
	vsubss	56(%rsp), %xmm13, %xmm3
.Ltmp1664:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_359
.LBB32_360:
	.loc	49 0 44
	xorl	%edx, %edx
.LBB32_361:
.Ltmp1665:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1666:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1667:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1668:
	.loc	1 924 9 is_stmt 1
	vmovd	%ebx, %xmm3
.Ltmp1669:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm13
	vmovaps	608(%rsp), %xmm2
	vblendvps	%xmm13, 592(%rsp), %xmm2, %xmm13
.Ltmp1670:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1671:
	.loc	49 92 9
	vmulss	%xmm3, %xmm13, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1672:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1673:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_362
.Ltmp1674:
	.loc	49 0 24
	xorl	%ebx, %ebx
.Ltmp1675:
	.loc	49 66 9 is_stmt 1
	vsubss	304(%rsp), %xmm15, %xmm3
.Ltmp1676:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jae	.LBB32_366
.Ltmp1677:
.LBB32_365:
	.loc	49 0 44
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm1
.Ltmp1678:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1679:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm1, %xmm3, %xmm1
	vmulss	300(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1680:
	.loc	49 161 44
	jmp	.LBB32_367
.Ltmp1681:
	.loc	49 0 44
.Ltmp1682:
	.p2align	4
.LBB32_362:
	.loc	49 166 0 is_stmt 1
	vmovd	%xmm3, %ebx
.Ltmp1683:
	.loc	49 66 9
	vsubss	304(%rsp), %xmm15, %xmm3
.Ltmp1684:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_365
.LBB32_366:
	.loc	49 0 44
	xorl	%edx, %edx
.LBB32_367:
.Ltmp1685:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1686:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1687:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1688:
	.loc	1 924 9 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp1689:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm13
	vmovaps	576(%rsp), %xmm2
	vblendvps	%xmm13, 560(%rsp), %xmm2, %xmm13
.Ltmp1690:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1691:
	.loc	49 92 9
	vmulss	%xmm3, %xmm13, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1692:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1693:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_368
	.loc	49 0 24
	xorl	%r13d, %r13d
	jmp	.LBB32_370
	.p2align	4
.LBB32_368:
	.loc	49 166 0
	vmovd	%xmm3, %r13d
.Ltmp1694:
.LBB32_370:
	.loc	49 0 0
	movq	64(%rsp), %rcx
	negq	%rcx
	addq	%rcx, %rdi
	incq	%rdi
.Ltmp1695:
	.loc	48 568 12 is_stmt 1
	cmpq	%r11, %rdi
	ja	.LBB32_588
.Ltmp1696:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp1697:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_190
.Ltmp1698:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp1699:
	.loc	48 568 12
	cmpq	%r12, %rdi
	ja	.LBB32_589
.Ltmp1700:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp1701:
	.loc	48 568 12
	cmpq	%r8, %rdi
	ja	.LBB32_590
.Ltmp1702:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp1703:
	.loc	49 0 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm1
	vmovaps	160(%rsp), %xmm10
	vmulss	120(%rsp), %xmm10, %xmm2
	vaddss	%xmm1, %xmm2, %xmm1
	vmulss	%xmm0, %xmm9, %xmm0
	vmovaps	192(%rsp), %xmm13
	vmulss	%xmm11, %xmm13, %xmm2
	vaddss	%xmm0, %xmm2, %xmm0
	vaddss	%xmm5, %xmm5, %xmm2
	vaddss	144(%rsp), %xmm2, %xmm3
	vmovss	24(%rsp), %xmm2
	vaddss	%xmm2, %xmm2, %xmm2
	vaddss	272(%rsp), %xmm2, %xmm7
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	%xmm4, %xmm4, %xmm2
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm6, %xmm6, %xmm5
	vaddss	%xmm0, %xmm0, %xmm6
	vaddss	%xmm12, %xmm12, %xmm8
	vaddss	%xmm1, %xmm10, %xmm9
	vaddss	224(%rsp), %xmm2, %xmm0
	vaddss	240(%rsp), %xmm4, %xmm1
	vaddss	256(%rsp), %xmm5, %xmm2
	vaddss	%xmm6, %xmm13, %xmm4
	vaddss	208(%rsp), %xmm8, %xmm5
	vmovss	12(%rsp), %xmm6
	vaddss	28(%rsp), %xmm6, %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm14
	vandps	%xmm3, %xmm14, %xmm8
	vandps	%xmm7, %xmm14, %xmm10
	vandps	%xmm14, %xmm9, %xmm11
	vandps	%xmm0, %xmm14, %xmm12
	vandps	%xmm1, %xmm14, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm15
	vcmpnltss	%xmm15, %xmm8, %xmm8
	vandps	%xmm3, %xmm8, %xmm3
	vmovaps	%xmm3, 144(%rsp)
	vandps	%xmm2, %xmm14, %xmm3
	vcmpnltss	%xmm15, %xmm10, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovaps	%xmm7, 272(%rsp)
	vandps	%xmm4, %xmm14, %xmm7
	vcmpnltss	%xmm15, %xmm11, %xmm8
	vandps	%xmm9, %xmm8, %xmm8
	vmovaps	%xmm8, 160(%rsp)
	vandps	%xmm5, %xmm14, %xmm8
	vmovss	.LCPI32_21(%rip), %xmm11
	vmulss	%xmm6, %xmm11, %xmm6
	vcmpnltss	%xmm15, %xmm12, %xmm9
	vcmpnltss	%xmm15, %xmm13, %xmm10
	vcmpnltss	%xmm15, %xmm3, %xmm3
	vcmpnltss	%xmm15, %xmm7, %xmm7
	vcmpnltss	%xmm15, %xmm8, %xmm8
	vmovss	.LCPI32_22(%rip), %xmm12
	vmaxss	%xmm12, %xmm6, %xmm6
	vandps	%xmm0, %xmm9, %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vandps	%xmm1, %xmm10, %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vandps	%xmm2, %xmm3, %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vandps	%xmm4, %xmm7, %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vandps	%xmm5, %xmm8, %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm7
	vminss	%xmm7, %xmm6, %xmm0
	vroundss	$9, %xmm0, %xmm0, %xmm2
	vsubss	%xmm2, %xmm0, %xmm4
	vmovss	16(%rsp), %xmm0
	vaddss	32(%rsp), %xmm0, %xmm0
	vmulss	%xmm0, %xmm11, %xmm0
	vmaxss	%xmm12, %xmm0, %xmm0
	vminss	%xmm7, %xmm0, %xmm1
	vroundss	$9, %xmm1, %xmm1, %xmm0
	vsubss	%xmm0, %xmm1, %xmm1
	vmovd	%ebx, %xmm3
	vaddss	432(%rsp), %xmm3, %xmm3
	vmulss	%xmm3, %xmm11, %xmm3
	vmaxss	%xmm12, %xmm3, %xmm3
	vminss	%xmm7, %xmm3, %xmm5
	vmovss	.LCPI32_24(%rip), %xmm6
	vmulss	%xmm6, %xmm4, %xmm3
	vmovss	.LCPI32_25(%rip), %xmm8
	vaddss	%xmm3, %xmm8, %xmm3
	vmulss	%xmm3, %xmm4, %xmm3
	vmovss	.LCPI32_26(%rip), %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
	vmulss	%xmm3, %xmm4, %xmm3
	vmovss	.LCPI32_27(%rip), %xmm10
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm4, %xmm3
	vmovss	.LCPI32_28(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm3
	vmulss	%xmm3, %xmm4, %xmm3
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
	vmovss	.LCPI32_29(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm3
	vmovss	.LCPI32_30(%rip), %xmm15
	vaddss	%xmm2, %xmm15, %xmm2
	vmovd	%xmm2, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm2
	vmulss	%xmm2, %xmm3, %xmm2
	vmulss	%xmm6, %xmm1, %xmm3
	vaddss	%xmm3, %xmm8, %xmm3
	vmulss	%xmm3, %xmm1, %xmm3
	vaddss	%xmm3, %xmm9, %xmm3
	vmulss	%xmm3, %xmm1, %xmm3
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm1, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp1704:
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp1705:
	vmovd	%r13d, %xmm3
	vaddss	72(%rsp), %xmm3, %xmm3
	vmulss	%xmm3, %xmm11, %xmm3
	vmaxss	%xmm12, %xmm3, %xmm3
	vminss	%xmm7, %xmm3, %xmm3
.Ltmp1706:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm0, %xmm15, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp1707:
	vmulss	%xmm6, %xmm5, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm9, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm10, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp1708:
	vmulss	%xmm1, %xmm5, %xmm1
.Ltmp1709:
	vroundss	$9, %xmm3, %xmm3, %xmm5
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp1710:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm4, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	vmulss	%xmm4, %xmm1, %xmm1
.Ltmp1711:
	vmulss	%xmm6, %xmm3, %xmm4
	vaddss	%xmm4, %xmm8, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm9, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm10, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp1712:
	vmulss	%xmm4, %xmm3, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm5, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	104(%rsp), %rax
.Ltmp1713:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm2
	vmulss	(%rsi,%rdi,4), %xmm0, %xmm0
.Ltmp1714:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp1715:
	vaddss	%xmm0, %xmm2, %xmm0
	movq	112(%rsp), %rax
.Ltmp1716:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
.Ltmp1717:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r9,%rdi,4), %xmm3, %xmm2
.Ltmp1718:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	96(%rsp), %rsi
.Ltmp1719:
	.loc	49 56 9
	vmovss	%xmm0, (%rsi,%rbp,4)
	movq	88(%rsp), %r8
.Ltmp1720:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r8,%rbp,4)
	incq	%rbp
.Ltmp1721:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rbp, 336(%rsp)
.Ltmp1722:
	.loc	3 900 12
	jne	.LBB32_315
	jmp	.LBB32_379
.Ltmp1723:
.LBB32_442:
	.loc	1 1142 5
	testb	%al, %al
	jne	.LBB32_263
.Ltmp1724:
	.loc	1 1083 23
	movl	856(%r14), %eax
	movl	%eax, 516(%rsp)
.Ltmp1725:
	.loc	1 1084 20
	movq	840(%r14), %r15
	xorl	%ebp, %ebp
	movq	%r15, 480(%rsp)
	jmp	.LBB32_444
	.loc	1 0 20 is_stmt 0
.Ltmp1726:
	.p2align	4
.LBB32_515:
	vmovaps	144(%rsp), %xmm0
.Ltmp1727:
	.loc	1 1057 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	272(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	224(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	240(%rsp), %xmm0
	.loc	1 1058 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	256(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	208(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	12(%rsp), %eax
	.loc	1 1059 5
	movl	%eax, 184(%r14)
	movl	16(%rsp), %eax
	movl	%eax, 188(%r14)
	.loc	1 1060 5
	movl	%ebx, 544(%r14)
	movl	%r13d, 548(%r14)
	.loc	1 1061 5
	movq	%rdi, 848(%r14)
.Ltmp1728:
.LBB32_516:
	.loc	1 0 5 is_stmt 0
	movq	456(%rsp), %rax
	movq	%rax, %rbp
	movq	184(%rsp), %r9
	.loc	1 1086 11 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB32_517
.LBB32_444:
	.loc	1 1087 42
	movq	%r9, %rsi
	subq	%rbp, %rsi
	.loc	1 1087 29 is_stmt 0
	leaq	104(%r14), %rdi
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r13
	vmovss	192(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	200(%r14), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	208(%r14), %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	216(%r14), %xmm0
	vmovss	%xmm0, 412(%rsp)
	vmovss	224(%r14), %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	232(%r14), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	240(%r14), %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	248(%r14), %xmm0
	vmovss	%xmm0, 404(%rsp)
	vmovss	256(%r14), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	264(%r14), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	272(%r14), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	280(%r14), %xmm0
	vmovss	%xmm0, 420(%rsp)
	vmovss	288(%r14), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	296(%r14), %xmm0
	vmovss	%xmm0, 396(%rsp)
	vmovss	304(%r14), %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	312(%r14), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	320(%r14), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	328(%r14), %xmm0
	vmovss	%xmm0, 388(%rsp)
	vmovss	336(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	344(%r14), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	552(%r14), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	560(%r14), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	568(%r14), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	576(%r14), %xmm0
	vmovss	%xmm0, 380(%rsp)
	vmovss	584(%r14), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	592(%r14), %xmm0
	vmovss	%xmm0, 376(%rsp)
	vmovss	600(%r14), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	608(%r14), %xmm0
	vmovss	%xmm0, 372(%rsp)
	vmovss	616(%r14), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	624(%r14), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	632(%r14), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	640(%r14), %xmm0
	vmovss	%xmm0, 332(%rsp)
	vmovss	648(%r14), %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	656(%r14), %xmm0
	vmovss	%xmm0, 364(%rsp)
	vmovss	664(%r14), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	672(%r14), %xmm0
	vmovss	%xmm0, 360(%rsp)
	vmovss	680(%r14), %xmm0
	vmovss	%xmm0, 64(%rsp)
	vmovss	688(%r14), %xmm0
	vmovss	%xmm0, 356(%rsp)
	vmovss	696(%r14), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	704(%r14), %xmm0
	vmovss	%xmm0, 680(%rsp)
.Ltmp1729:
	.loc	1 1091 31 is_stmt 1
	leaq	520(%rsp), %rdi
	leaq	120(%r14), %rsi
	movl	516(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	712(%rsp), %rdi
	leaq	480(%r14), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovss	520(%rsp), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	524(%rsp), %xmm0
	vmovaps	%xmm0, 544(%rsp)
	vmovss	528(%rsp), %xmm0
	vmovaps	%xmm0, 656(%rsp)
	vmovss	532(%rsp), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	536(%rsp), %xmm0
	vmovaps	%xmm0, 640(%rsp)
	vmovss	540(%rsp), %xmm0
	vmovaps	%xmm0, 624(%rsp)
	vmovss	712(%rsp), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	716(%rsp), %xmm0
	vmovaps	%xmm0, 608(%rsp)
	vmovss	720(%rsp), %xmm0
	vmovaps	%xmm0, 592(%rsp)
	vmovss	724(%rsp), %xmm0
	vmovss	%xmm0, 300(%rsp)
	vmovss	728(%rsp), %xmm0
	vmovaps	%xmm0, 576(%rsp)
	vmovss	732(%rsp), %xmm0
	vmovaps	%xmm0, 560(%rsp)
	movq	%r13, 336(%rsp)
.Ltmp1730:
	.loc	1 0 0 is_stmt 0
	addq	%rbp, %r13
	setb	%cl
	cmpq	184(%rsp), %r13
	seta	%al
.Ltmp1731:
	.loc	38 1050 16 is_stmt 1
	orb	%cl, %al
.Ltmp1732:
	.loc	1 1094 12
	testb	$1, %r12b
	movq	%r13, 456(%rsp)
	je	.LBB32_450
.Ltmp1733:
	.loc	38 1050 16
	testb	%al, %al
	jne	.LBB32_524
.Ltmp1734:
	.loc	1 972 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 272(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
.Ltmp1735:
	.loc	1 973 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
.Ltmp1736:
	.loc	1 974 25
	movl	184(%r14), %eax
	movl	%eax, 12(%rsp)
	movl	188(%r14), %eax
	movl	%eax, 16(%rsp)
.Ltmp1737:
	.loc	1 975 24
	movl	544(%r14), %r13d
	movl	548(%r14), %ebx
.Ltmp1738:
	.loc	1 976 24
	movq	848(%r14), %rdi
.Ltmp1739:
	.loc	2 1916 50
	cmpq	$0, 336(%rsp)
	je	.LBB32_585
.Ltmp1740:
	.loc	2 0 50 is_stmt 0
	movq	464(%rsp), %rax
	leaq	(%rax,%rbp,4), %rsi
	movq	472(%rsp), %rax
	leaq	(%rax,%rbp,4), %r8
	xorl	%ebp, %ebp
	movq	%rsi, 672(%rsp)
	movq	%r8, 504(%rsp)
	.p2align	4
.LBB32_448:
.Ltmp1741:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp1742:
	.loc	1 857 8
	cmpq	%r15, %rax
	jb	.LBB32_449
.Ltmp1743:
	.loc	1 0 8 is_stmt 0
	movq	%r15, %rcx
	jmp	.LBB32_526
	.p2align	4
.LBB32_449:
	xorl	%ecx, %ecx
.LBB32_526:
.Ltmp1744:
	.loc	1 1000 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp1745:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp1746:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1747:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rsi,%rbp,4), %xmm0
.Ltmp1748:
	vmovss	(%r8,%rbp,4), %xmm3
.Ltmp1749:
	vmovss	152(%r14), %xmm6
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm4
	vmovaps	272(%rsp), %xmm9
	vsubss	%xmm9, %xmm0, %xmm1
	vmulss	%xmm7, %xmm1, %xmm5
	vmovaps	144(%rsp), %xmm2
	vmovss	%xmm6, 428(%rsp)
	vmulss	%xmm6, %xmm2, %xmm6
	vaddss	%xmm5, %xmm6, %xmm5
	vaddss	%xmm5, %xmm2, %xmm6
	vmulss	%xmm7, %xmm2, %xmm8
	vmulss	%xmm4, %xmm1, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	164(%r14), %xmm6, %xmm15
	vmovss	%xmm1, 140(%rsp)
	vaddss	%xmm1, %xmm9, %xmm6
	vmovaps	224(%rsp), %xmm1
	vsubss	%xmm1, %xmm6, %xmm10
	vmulss	160(%rsp), %xmm7, %xmm6
	vmulss	%xmm4, %xmm10, %xmm4
	vaddss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm1, %xmm6
.Ltmp1750:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm12
	vmovss	520(%r14), %xmm13
	vmovaps	256(%rsp), %xmm1
	vsubss	%xmm1, %xmm3, %xmm14
	vmulss	%xmm12, %xmm14, %xmm8
	vmovaps	240(%rsp), %xmm2
	vmulss	%xmm2, %xmm11, %xmm9
	vaddss	%xmm8, %xmm9, %xmm8
	vaddss	%xmm2, %xmm8, %xmm9
	vmulss	524(%r14), %xmm9, %xmm9
.Ltmp1751:
	.loc	1 1000 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp1752:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp1753:
	.loc	1 1001 30
	movq	144(%r14), %rdx
.Ltmp1754:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_129
.Ltmp1755:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1756:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm0, %xmm15, %xmm0
	vsubss	%xmm6, %xmm0, %xmm0
.Ltmp1757:
	.loc	1 1001 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp1758:
	.loc	49 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp1759:
	.loc	1 1002 28
	movq	488(%r14), %rdx
.Ltmp1760:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp1761:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1762:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm2, %xmm12, %xmm0
	vmulss	%xmm13, %xmm14, %xmm6
	vaddss	%xmm6, %xmm0, %xmm14
	vaddss	%xmm1, %xmm14, %xmm0
	vmovaps	208(%rsp), %xmm1
	vsubss	%xmm1, %xmm0, %xmm15
	vmulss	192(%rsp), %xmm12, %xmm0
	vmulss	%xmm15, %xmm13, %xmm6
	vaddss	%xmm6, %xmm0, %xmm0
	vaddss	%xmm0, %xmm1, %xmm6
.Ltmp1763:
	.loc	1 1002 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp1764:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp1765:
	.loc	1 1003 29
	movq	504(%r14), %rdx
.Ltmp1766:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_587
.Ltmp1767:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1768:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm9, %xmm3
	vsubss	%xmm6, %xmm3, %xmm3
.Ltmp1769:
	.loc	1 1003 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp1770:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r9
	addq	%rdi, %r9
	cmpq	%r15, %r9
	jb	.LBB32_535
.Ltmp1771:
	.loc	49 0 9 is_stmt 0
	movq	%r15, %rax
	jmp	.LBB32_537
	.p2align	4
.LBB32_535:
	xorl	%eax, %eax
.LBB32_537:
	movq	128(%r14), %r11
	subq	%rax, %r9
	cmpq	%r11, %r9
	jae	.LBB32_619
.Ltmp1772:
	.loc	1 1008 34 is_stmt 1
	movq	144(%r14), %rax
	cmpq	%rax, %r9
	jae	.LBB32_620
	.loc	1 0 34 is_stmt 0
	movq	%rcx, 320(%rsp)
	movq	760(%r14), %r10
	addq	%rdi, %r10
	cmpq	%r15, %r10
	jb	.LBB32_540
	movq	%r15, %rcx
	jmp	.LBB32_542
	.p2align	4
.LBB32_540:
	xorl	%ecx, %ecx
.LBB32_542:
	movq	488(%r14), %r12
	subq	%rcx, %r10
	cmpq	%r12, %r10
	jae	.LBB32_621
.Ltmp1773:
	.loc	1 1012 34 is_stmt 1
	movq	504(%r14), %r8
	cmpq	%r8, %r10
	jae	.LBB32_586
	.loc	1 0 34 is_stmt 0
	movl	%ebx, %ecx
	movq	120(%r14), %rdx
	movq	%rdx, 688(%rsp)
	vmovss	(%rdx,%r9,4), %xmm3
	movq	480(%r14), %rdx
	movq	%rdx, 696(%rsp)
	vmovss	(%rdx,%r10,4), %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1774:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp1775:
	.loc	49 103 24 is_stmt 0
	vandps	%xmm1, %xmm6, %xmm6
	vmovss	.LCPI32_3(%rip), %xmm1
.Ltmp1776:
	.loc	49 71 9 is_stmt 1
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp1777:
	.loc	49 71 9 is_stmt 0
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp1778:
	.loc	49 61 9 is_stmt 1
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp1779:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r15d
	movl	$841731191, %edx
.Ltmp1780:
	.loc	49 161 24
	jbe	.LBB32_546
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %edx
.LBB32_546:
.Ltmp1781:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp1782:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %ebx
	movl	$8388608, %esi
.Ltmp1783:
	.loc	49 161 24
	jbe	.LBB32_548
.Ltmp1784:
	.loc	49 0 24 is_stmt 0
	movl	%edx, %esi
.LBB32_548:
	vmovss	80(%rsp), %xmm1
	vaddss	424(%rsp), %xmm1, %xmm1
.Ltmp1785:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp1786:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp1787:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp1788:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm6
.Ltmp1789:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm6, %xmm6
.Ltmp1790:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1791:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm6, %xmm6
.Ltmp1792:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1793:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm6, %xmm6
.Ltmp1794:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1795:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm6, %xmm6
.Ltmp1796:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1797:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm6, %xmm6
.Ltmp1798:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp1799:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1800:
	.loc	23 1291 18
	vmovd	%esi, %xmm6
.Ltmp1801:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm6, %xmm6
.Ltmp1802:
	.loc	49 61 9
	vaddss	%xmm3, %xmm6, %xmm3
.Ltmp1803:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp1804:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp1805:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm1, 80(%rsp)
.Ltmp1806:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm3, %xmm6
.Ltmp1807:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm6, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_549
.Ltmp1808:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_551
	.p2align	4
.LBB32_549:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm6, %xmm9
.Ltmp1809:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp1810:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm1, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	136(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %edx
.Ltmp1811:
.LBB32_551:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm6
.Ltmp1812:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp1813:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vminss	%xmm1, %xmm6, %xmm6
.Ltmp1814:
	.loc	1 924 9 is_stmt 1
	vmovss	12(%rsp), %xmm9
.Ltmp1815:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	544(%rsp), %xmm1
	vblendvps	%xmm13, 656(%rsp), %xmm1, %xmm13
.Ltmp1816:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp1817:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1818:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm9
.Ltmp1819:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm1
	vucomiss	%xmm9, %xmm1
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_552
	.loc	49 0 24
	movl	$0, 12(%rsp)
	jmp	.LBB32_554
	.p2align	4
.LBB32_552:
	.loc	49 166 0
	vmovss	%xmm6, 12(%rsp)
.Ltmp1820:
.LBB32_554:
	.loc	49 0 0
	movq	136(%r14), %rsi
	vmovss	(%rsi,%r9,4), %xmm6
	movq	496(%r14), %r9
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1821:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm6, %xmm6
.Ltmp1822:
	.loc	49 103 24 is_stmt 0
	vmovss	(%r9,%r10,4), %xmm9
	vandps	%xmm1, %xmm9, %xmm9
	vmovss	.LCPI32_3(%rip), %xmm1
.Ltmp1823:
	.loc	49 71 9 is_stmt 1
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp1824:
	.loc	49 71 9 is_stmt 0
	vmulss	%xmm1, %xmm9, %xmm9
.Ltmp1825:
	.loc	49 61 9 is_stmt 1
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1826:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
.Ltmp1827:
	.loc	49 161 24
	jbe	.LBB32_556
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %r15d
.LBB32_556:
.Ltmp1828:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r15d, %xmm6
.Ltmp1829:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
.Ltmp1830:
	.loc	49 161 24
	jbe	.LBB32_558
.Ltmp1831:
	.loc	49 0 24 is_stmt 0
	movl	%r15d, %ebx
.LBB32_558:
	vmovss	20(%rsp), %xmm1
	vaddss	420(%rsp), %xmm1, %xmm1
.Ltmp1832:
	.loc	49 185 42 is_stmt 1
	movl	%ebx, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp1833:
	.loc	23 1291 18
	vmovd	%edx, %xmm6
.Ltmp1834:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp1835:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp1836:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp1837:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1838:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp1839:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1840:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp1841:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1842:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp1843:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1844:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp1845:
	.loc	49 187 28
	shrl	$23, %ebx
	orl	$1258291200, %ebx
.Ltmp1846:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp1847:
	.loc	23 1291 18
	vmovd	%ebx, %xmm9
.Ltmp1848:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp1849:
	.loc	49 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1850:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp1851:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp1852:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm9
	vmovss	%xmm1, 20(%rsp)
.Ltmp1853:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm9, %xmm6
.Ltmp1854:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm6, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_559
.Ltmp1855:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_561
	.p2align	4
.LBB32_559:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm6, %xmm13
.Ltmp1856:
	vmulss	%xmm13, %xmm13, %xmm13
	vmulss	.LCPI32_18(%rip), %xmm13, %xmm13
.Ltmp1857:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm1, %xmm1
	vblendvps	%xmm1, %xmm13, %xmm6, %xmm1
	vmulss	132(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1858:
.LBB32_561:
	.loc	49 0 0
	movq	480(%rsp), %r15
.Ltmp1859:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1860:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1861:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm6, %xmm6, %xmm6
	vminss	%xmm6, %xmm1, %xmm1
.Ltmp1862:
	.loc	1 924 9 is_stmt 1
	vmovss	16(%rsp), %xmm6
.Ltmp1863:
	.loc	49 161 24
	vcmpnltss	%xmm6, %xmm1, %xmm13
	vmovaps	640(%rsp), %xmm2
	vblendvps	%xmm13, 624(%rsp), %xmm2, %xmm13
.Ltmp1864:
	.loc	49 66 9
	vsubss	%xmm1, %xmm6, %xmm6
.Ltmp1865:
	.loc	49 92 9
	vmulss	%xmm6, %xmm13, %xmm6
	vaddss	%xmm6, %xmm1, %xmm6
.Ltmp1866:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm1
.Ltmp1867:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_562
	.loc	49 0 24
	movl	$0, 16(%rsp)
	jmp	.LBB32_564
	.p2align	4
.LBB32_562:
	.loc	49 166 0
	vmovss	%xmm6, 16(%rsp)
.Ltmp1868:
.LBB32_564:
	.loc	49 0 0
	vmovss	56(%rsp), %xmm1
	vaddss	416(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 56(%rsp)
.Ltmp1869:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1870:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_565
.Ltmp1871:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_567
	.p2align	4
.LBB32_565:
	vmovss	.LCPI32_17(%rip), %xmm6
	vaddss	%xmm6, %xmm3, %xmm1
.Ltmp1872:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1873:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm6, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vmulss	128(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1874:
.LBB32_567:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1875:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1876:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1877:
	.loc	1 924 9 is_stmt 1
	vmovd	%r13d, %xmm3
.Ltmp1878:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm6
	vmovaps	608(%rsp), %xmm2
	vblendvps	%xmm6, 592(%rsp), %xmm2, %xmm6
.Ltmp1879:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1880:
	.loc	49 92 9
	vmulss	%xmm6, %xmm3, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1881:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1882:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm6
	vucomiss	%xmm1, %xmm6
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_568
	.loc	49 0 24
	xorl	%r13d, %r13d
	jmp	.LBB32_570
	.p2align	4
.LBB32_568:
	.loc	49 166 0
	vmovd	%xmm3, %r13d
.Ltmp1883:
.LBB32_570:
	.loc	49 0 0
	vmovss	304(%rsp), %xmm1
	vaddss	332(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 304(%rsp)
.Ltmp1884:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm9, %xmm3
.Ltmp1885:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_571
.Ltmp1886:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_573
	.p2align	4
.LBB32_571:
	vmovss	.LCPI32_17(%rip), %xmm6
	vaddss	%xmm6, %xmm3, %xmm1
.Ltmp1887:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1888:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm6, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vmulss	300(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp1889:
.LBB32_573:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp1890:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1891:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1892:
	.loc	1 924 9 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp1893:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm6
	vmovaps	576(%rsp), %xmm2
	vblendvps	%xmm6, 560(%rsp), %xmm2, %xmm6
.Ltmp1894:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1895:
	.loc	49 92 9
	vmulss	%xmm6, %xmm3, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1896:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1897:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm6
	vucomiss	%xmm1, %xmm6
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_574
	.loc	49 0 24
	xorl	%ebx, %ebx
	jmp	.LBB32_576
	.p2align	4
.LBB32_574:
	.loc	49 166 0
	vmovd	%xmm3, %ebx
.Ltmp1898:
.LBB32_576:
	.loc	49 0 0
	movq	320(%rsp), %rcx
	negq	%rcx
	addq	%rcx, %rdi
	incq	%rdi
.Ltmp1899:
	.loc	48 568 12 is_stmt 1
	cmpq	%r11, %rdi
	ja	.LBB32_588
.Ltmp1900:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
.Ltmp1901:
	.loc	48 568 12 is_stmt 1
	cmpq	%rax, %rdi
	ja	.LBB32_190
.Ltmp1902:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
.Ltmp1903:
	.loc	48 568 12 is_stmt 1
	cmpq	%r12, %rdi
	ja	.LBB32_589
.Ltmp1904:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
.Ltmp1905:
	.loc	48 568 12 is_stmt 1
	cmpq	%r8, %rdi
	ja	.LBB32_590
.Ltmp1906:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
	vmulss	%xmm7, %xmm10, %xmm1
	vmovaps	160(%rsp), %xmm13
	vmulss	428(%rsp), %xmm13, %xmm2
	vaddss	%xmm1, %xmm2, %xmm1
	vmulss	%xmm15, %xmm12, %xmm2
	vmovaps	192(%rsp), %xmm12
	vmulss	%xmm11, %xmm12, %xmm3
	vaddss	%xmm2, %xmm3, %xmm2
	vaddss	%xmm5, %xmm5, %xmm3
	vaddss	144(%rsp), %xmm3, %xmm7
	vmovss	140(%rsp), %xmm3
	vaddss	%xmm3, %xmm3, %xmm3
	vaddss	272(%rsp), %xmm3, %xmm5
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	%xmm4, %xmm4, %xmm3
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm14, %xmm14, %xmm6
	vaddss	%xmm2, %xmm2, %xmm9
	vaddss	%xmm0, %xmm0, %xmm10
	vaddss	%xmm1, %xmm13, %xmm8
	vaddss	224(%rsp), %xmm3, %xmm0
	vaddss	240(%rsp), %xmm4, %xmm1
	vaddss	256(%rsp), %xmm6, %xmm2
	vaddss	%xmm9, %xmm12, %xmm3
	vaddss	208(%rsp), %xmm10, %xmm4
	vmovss	28(%rsp), %xmm9
	vaddss	400(%rsp), %xmm9, %xmm9
	vmovss	12(%rsp), %xmm6
	vmovss	%xmm9, 28(%rsp)
	vaddss	%xmm6, %xmm9, %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm14
	vandps	%xmm7, %xmm14, %xmm9
	vandps	%xmm5, %xmm14, %xmm10
	vandps	%xmm14, %xmm8, %xmm11
	vandps	%xmm0, %xmm14, %xmm12
	vandps	%xmm1, %xmm14, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm15
	vcmpnltss	%xmm15, %xmm9, %xmm9
	vandps	%xmm7, %xmm9, %xmm7
	vmovaps	%xmm7, 144(%rsp)
	vandps	%xmm2, %xmm14, %xmm7
	vcmpnltss	%xmm15, %xmm10, %xmm9
	vandps	%xmm5, %xmm9, %xmm5
	vmovaps	%xmm5, 272(%rsp)
	vandps	%xmm3, %xmm14, %xmm5
	vcmpnltss	%xmm15, %xmm11, %xmm9
	vandps	%xmm8, %xmm9, %xmm8
	vmovaps	%xmm8, 160(%rsp)
	vandps	%xmm4, %xmm14, %xmm8
	vmovss	.LCPI32_21(%rip), %xmm14
	vmulss	%xmm6, %xmm14, %xmm6
	vcmpnltss	%xmm15, %xmm12, %xmm9
	vcmpnltss	%xmm15, %xmm13, %xmm10
	vcmpnltss	%xmm15, %xmm7, %xmm7
	vcmpnltss	%xmm15, %xmm5, %xmm5
	vcmpnltss	%xmm15, %xmm8, %xmm8
	vmovss	.LCPI32_22(%rip), %xmm12
	vmaxss	%xmm12, %xmm6, %xmm6
	vandps	%xmm0, %xmm9, %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vandps	%xmm1, %xmm10, %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vandps	%xmm2, %xmm7, %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vandps	%xmm3, %xmm5, %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vandps	%xmm4, %xmm8, %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm11
	vminss	%xmm11, %xmm6, %xmm0
	vroundss	$9, %xmm0, %xmm0, %xmm2
	vmovss	432(%rsp), %xmm5
	vaddss	368(%rsp), %xmm5, %xmm5
	vmovss	32(%rsp), %xmm1
	vaddss	384(%rsp), %xmm1, %xmm1
	vsubss	%xmm2, %xmm0, %xmm3
	vmovss	16(%rsp), %xmm0
	vmovss	%xmm1, 32(%rsp)
	vaddss	%xmm0, %xmm1, %xmm0
	vmulss	%xmm0, %xmm14, %xmm0
	vmaxss	%xmm12, %xmm0, %xmm0
	vminss	%xmm11, %xmm0, %xmm1
	vroundss	$9, %xmm1, %xmm1, %xmm7
	vsubss	%xmm7, %xmm1, %xmm1
	vmovd	%r13d, %xmm4
	vmovss	%xmm5, 432(%rsp)
	vaddss	%xmm4, %xmm5, %xmm4
	vmulss	%xmm4, %xmm14, %xmm4
	vmaxss	%xmm12, %xmm4, %xmm4
	vminss	%xmm11, %xmm4, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm6
	vmulss	%xmm6, %xmm3, %xmm5
	vmovss	.LCPI32_25(%rip), %xmm8
	vaddss	%xmm5, %xmm8, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_26(%rip), %xmm9
	vaddss	%xmm5, %xmm9, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_27(%rip), %xmm10
	vaddss	%xmm5, %xmm10, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_28(%rip), %xmm13
	vaddss	%xmm5, %xmm13, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vroundss	$9, %xmm4, %xmm4, %xmm3
	vsubss	%xmm3, %xmm4, %xmm4
	vmovss	72(%rsp), %xmm0
	vaddss	680(%rsp), %xmm0, %xmm0
	vmovss	.LCPI32_29(%rip), %xmm14
	vaddss	%xmm5, %xmm14, %xmm5
	vmovss	.LCPI32_30(%rip), %xmm15
	vaddss	%xmm2, %xmm15, %xmm2
	vmovd	%xmm2, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm2
	vmulss	%xmm2, %xmm5, %xmm2
	vmulss	%xmm6, %xmm1, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
	vmulss	%xmm5, %xmm1, %xmm5
	vaddss	%xmm5, %xmm9, %xmm5
	vmulss	%xmm5, %xmm1, %xmm5
	vaddss	%xmm5, %xmm10, %xmm5
	vmulss	%xmm5, %xmm1, %xmm5
	vaddss	%xmm5, %xmm13, %xmm5
.Ltmp1907:
	vmulss	%xmm5, %xmm1, %xmm1
.Ltmp1908:
	vmovd	%ebx, %xmm5
	vmovss	%xmm0, 72(%rsp)
	vaddss	%xmm5, %xmm0, %xmm5
	vmulss	.LCPI32_21(%rip), %xmm5, %xmm5
	vmaxss	%xmm12, %xmm5, %xmm5
	vminss	%xmm11, %xmm5, %xmm5
.Ltmp1909:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm7, %xmm15, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp1910:
	vmulss	%xmm6, %xmm4, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm9, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm10, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp1911:
	vmulss	%xmm1, %xmm4, %xmm1
.Ltmp1912:
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
.Ltmp1913:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm3, %xmm15, %xmm3
	vmovd	%xmm3, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm3
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp1914:
	vmulss	%xmm6, %xmm5, %xmm3
	vaddss	%xmm3, %xmm8, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm9, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp1915:
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm4, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	688(%rsp), %rax
.Ltmp1916:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm2
	vmulss	(%rsi,%rdi,4), %xmm0, %xmm0
.Ltmp1917:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp1918:
	vaddss	%xmm0, %xmm2, %xmm0
	movq	696(%rsp), %rax
.Ltmp1919:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
.Ltmp1920:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r9,%rdi,4), %xmm3, %xmm2
.Ltmp1921:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	672(%rsp), %rsi
.Ltmp1922:
	.loc	49 56 9
	vmovss	%xmm0, (%rsi,%rbp,4)
	movq	504(%rsp), %r8
.Ltmp1923:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r8,%rbp,4)
	vmovss	36(%rsp), %xmm0
	vaddss	412(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	96(%rsp), %xmm0
	vaddss	380(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	40(%rsp), %xmm0
	vaddss	408(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	104(%rsp), %xmm0
	vaddss	376(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	44(%rsp), %xmm0
	vaddss	404(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	112(%rsp), %xmm0
	vaddss	372(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	48(%rsp), %xmm0
	vaddss	396(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	120(%rsp), %xmm0
	vaddss	364(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	52(%rsp), %xmm0
	vaddss	392(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	24(%rsp), %xmm0
	vaddss	360(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	88(%rsp), %xmm0
	vaddss	388(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	64(%rsp), %xmm0
	vaddss	356(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 64(%rsp)
	incq	%rbp
.Ltmp1924:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rbp, 336(%rsp)
.Ltmp1925:
	.loc	3 900 12
	jne	.LBB32_448
.Ltmp1926:
.LBB32_585:
	.loc	3 0 12 is_stmt 0
	vmovaps	144(%rsp), %xmm0
	.loc	1 1057 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	272(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	224(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	240(%rsp), %xmm0
	.loc	1 1058 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	256(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	208(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	12(%rsp), %eax
	.loc	1 1059 5
	movl	%eax, 184(%r14)
	movl	16(%rsp), %eax
	movl	%eax, 188(%r14)
	.loc	1 1060 5
	movl	%r13d, 544(%r14)
	movl	%ebx, 548(%r14)
	.loc	1 1061 5
	movq	%rdi, 848(%r14)
	vmovss	80(%rsp), %xmm0
.Ltmp1927:
	.loc	1 1197 17
	vmovss	%xmm0, 192(%r14)
	.loc	1 1198 34
	movl	204(%r14), %eax
	movq	336(%rsp), %rsi
.Ltmp1928:
	.loc	38 2472 13
	subl	%esi, %eax
	movl	$0, %edx
	cmovbl	%edx, %eax
.Ltmp1929:
	.loc	1 1198 34
	movl	220(%r14), %ecx
	.loc	1 1198 17 is_stmt 0
	movl	%eax, 204(%r14)
	vmovss	36(%rsp), %xmm0
	.loc	1 1197 17 is_stmt 1
	vmovss	%xmm0, 208(%r14)
.Ltmp1930:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp1931:
	.loc	1 1198 17
	movl	%ecx, 220(%r14)
	vmovss	40(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 224(%r14)
	.loc	1 1198 34
	movl	236(%r14), %eax
.Ltmp1932:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1933:
	.loc	1 1198 34
	movl	252(%r14), %ecx
	.loc	1 1198 17 is_stmt 0
	movl	%eax, 236(%r14)
	vmovss	44(%rsp), %xmm0
	.loc	1 1197 17 is_stmt 1
	vmovss	%xmm0, 240(%r14)
.Ltmp1934:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp1935:
	.loc	1 1198 17
	movl	%ecx, 252(%r14)
	vmovss	28(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 256(%r14)
	.loc	1 1198 34
	movl	268(%r14), %eax
.Ltmp1936:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1937:
	.loc	1 1198 17
	movl	%eax, 268(%r14)
	vmovss	20(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 272(%r14)
	.loc	1 1198 34
	movl	284(%r14), %eax
.Ltmp1938:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1939:
	.loc	1 1198 17
	movl	%eax, 284(%r14)
	vmovss	48(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 288(%r14)
	.loc	1 1198 34
	movl	300(%r14), %eax
.Ltmp1940:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1941:
	.loc	1 1198 17
	movl	%eax, 300(%r14)
	vmovss	52(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 304(%r14)
	.loc	1 1198 34
	movl	316(%r14), %eax
.Ltmp1942:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1943:
	.loc	1 1198 17
	movl	%eax, 316(%r14)
	vmovss	88(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 320(%r14)
	.loc	1 1198 34
	movl	332(%r14), %eax
.Ltmp1944:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1945:
	.loc	1 1198 17
	movl	%eax, 332(%r14)
	vmovss	32(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 336(%r14)
	.loc	1 1198 34
	movl	348(%r14), %eax
.Ltmp1946:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1947:
	.loc	1 1198 17
	movl	%eax, 348(%r14)
	vmovss	56(%rsp), %xmm0
.Ltmp1948:
	.loc	1 1197 17
	vmovss	%xmm0, 552(%r14)
	.loc	1 1198 34
	movl	564(%r14), %eax
.Ltmp1949:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1950:
	.loc	1 1198 17
	movl	%eax, 564(%r14)
	vmovss	96(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 568(%r14)
	.loc	1 1198 34
	movl	580(%r14), %eax
.Ltmp1951:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1952:
	.loc	1 1198 17
	movl	%eax, 580(%r14)
	vmovss	104(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 584(%r14)
	.loc	1 1198 34
	movl	596(%r14), %eax
.Ltmp1953:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1954:
	.loc	1 1198 17
	movl	%eax, 596(%r14)
	vmovss	112(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 600(%r14)
	.loc	1 1198 34
	movl	612(%r14), %eax
.Ltmp1955:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1956:
	.loc	1 1198 17
	movl	%eax, 612(%r14)
	vmovss	432(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 616(%r14)
	.loc	1 1198 34
	movl	628(%r14), %eax
.Ltmp1957:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1958:
	.loc	1 1198 17
	movl	%eax, 628(%r14)
	vmovss	304(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 632(%r14)
	.loc	1 1198 34
	movl	644(%r14), %eax
.Ltmp1959:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1960:
	.loc	1 1198 17
	movl	%eax, 644(%r14)
	vmovss	120(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 648(%r14)
	.loc	1 1198 34
	movl	660(%r14), %eax
.Ltmp1961:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1962:
	.loc	1 1198 17
	movl	%eax, 660(%r14)
	vmovss	24(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 664(%r14)
	.loc	1 1198 34
	movl	676(%r14), %eax
.Ltmp1963:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1964:
	.loc	1 1198 17
	movl	%eax, 676(%r14)
	vmovss	64(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 680(%r14)
	.loc	1 1198 34
	movl	692(%r14), %eax
.Ltmp1965:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1966:
	.loc	1 1198 17
	movl	%eax, 692(%r14)
	vmovss	72(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 696(%r14)
	.loc	1 1198 34
	movl	708(%r14), %eax
.Ltmp1967:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1968:
	.loc	1 1198 17
	movl	%eax, 708(%r14)
.Ltmp1969:
	.loc	1 1094 9
	jmp	.LBB32_516
	.loc	1 0 9 is_stmt 0
.Ltmp1970:
	.p2align	4
.LBB32_450:
.Ltmp1971:
	.loc	38 1050 16 is_stmt 1
	testb	%al, %al
	jne	.LBB32_317
.Ltmp1972:
	.loc	1 972 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 272(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
.Ltmp1973:
	.loc	1 973 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
.Ltmp1974:
	.loc	1 974 25
	movl	184(%r14), %eax
	movl	%eax, 12(%rsp)
	movl	188(%r14), %eax
	movl	%eax, 16(%rsp)
.Ltmp1975:
	.loc	1 975 24
	movl	544(%r14), %ebx
	movl	548(%r14), %r13d
.Ltmp1976:
	.loc	1 976 24
	movq	848(%r14), %rdi
.Ltmp1977:
	.loc	2 1916 50
	cmpq	$0, 336(%rsp)
.Ltmp1978:
	.loc	3 900 12
	je	.LBB32_515
.Ltmp1979:
	.loc	3 0 12 is_stmt 0
	movq	464(%rsp), %rax
	leaq	(%rax,%rbp,4), %rsi
	movq	472(%rsp), %rax
	leaq	(%rax,%rbp,4), %r8
	xorl	%ebp, %ebp
	movq	%rsi, 96(%rsp)
	movq	%r8, 88(%rsp)
	.p2align	4
.LBB32_453:
.Ltmp1980:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp1981:
	.loc	1 857 8
	cmpq	%r15, %rax
	jb	.LBB32_454
.Ltmp1982:
	.loc	1 0 8 is_stmt 0
	movq	%r15, %rcx
	jmp	.LBB32_456
	.p2align	4
.LBB32_454:
	xorl	%ecx, %ecx
.LBB32_456:
.Ltmp1983:
	.loc	1 1000 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp1984:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp1985:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1986:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rsi,%rbp,4), %xmm6
.Ltmp1987:
	vmovss	(%r8,%rbp,4), %xmm13
.Ltmp1988:
	vmovss	152(%r14), %xmm4
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm0
	vmovaps	272(%rsp), %xmm8
	vsubss	%xmm8, %xmm6, %xmm1
	vmulss	%xmm7, %xmm1, %xmm3
	vmovaps	144(%rsp), %xmm2
	vmovss	%xmm4, 120(%rsp)
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm3, %xmm4, %xmm5
	vaddss	%xmm5, %xmm2, %xmm3
	vmulss	%xmm7, %xmm2, %xmm4
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	%xmm1, %xmm4, %xmm1
	vmulss	164(%r14), %xmm3, %xmm3
	vmovss	%xmm1, 24(%rsp)
	vaddss	%xmm1, %xmm8, %xmm4
	vmovaps	224(%rsp), %xmm1
	vsubss	%xmm1, %xmm4, %xmm10
	vmulss	160(%rsp), %xmm7, %xmm4
	vmulss	%xmm0, %xmm10, %xmm0
	vaddss	%xmm0, %xmm4, %xmm4
	vaddss	%xmm4, %xmm1, %xmm14
.Ltmp1989:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm0
	vmovss	520(%r14), %xmm12
	vmovaps	256(%rsp), %xmm1
	vsubss	%xmm1, %xmm13, %xmm9
	vmulss	%xmm0, %xmm9, %xmm8
	vmovaps	240(%rsp), %xmm2
	vmulss	%xmm2, %xmm11, %xmm15
	vaddss	%xmm8, %xmm15, %xmm8
	vaddss	%xmm2, %xmm8, %xmm15
	vmulss	524(%r14), %xmm15, %xmm15
.Ltmp1990:
	.loc	1 1000 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp1991:
	.loc	49 56 9
	vmovss	%xmm14, (%rax,%rdi,4)
.Ltmp1992:
	.loc	1 1001 30
	movq	144(%r14), %rdx
.Ltmp1993:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_129
.Ltmp1994:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp1995:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm6, %xmm3
	vsubss	%xmm14, %xmm3, %xmm3
.Ltmp1996:
	.loc	1 1001 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp1997:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp1998:
	.loc	1 1002 28
	movq	488(%r14), %rdx
.Ltmp1999:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp2000:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2001:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm0, %xmm2, %xmm3
	vmulss	%xmm12, %xmm9, %xmm6
	vaddss	%xmm6, %xmm3, %xmm6
	vaddss	%xmm6, %xmm1, %xmm3
	vmovaps	208(%rsp), %xmm1
	vsubss	%xmm1, %xmm3, %xmm9
	vmulss	192(%rsp), %xmm0, %xmm3
	vmulss	%xmm9, %xmm12, %xmm12
	vaddss	%xmm3, %xmm12, %xmm12
	vaddss	%xmm1, %xmm12, %xmm3
.Ltmp2002:
	.loc	1 1002 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp2003:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp2004:
	.loc	1 1003 29
	movq	504(%r14), %rdx
.Ltmp2005:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_587
.Ltmp2006:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2007:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm15, %xmm13, %xmm13
	vsubss	%xmm3, %xmm13, %xmm3
.Ltmp2008:
	.loc	1 1003 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp2009:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r9
	addq	%rdi, %r9
	cmpq	%r15, %r9
	jb	.LBB32_465
.Ltmp2010:
	.loc	49 0 9 is_stmt 0
	movq	%r15, %rax
	jmp	.LBB32_467
	.p2align	4
.LBB32_465:
	xorl	%eax, %eax
.LBB32_467:
	movq	128(%r14), %r11
	subq	%rax, %r9
	cmpq	%r11, %r9
	jae	.LBB32_619
.Ltmp2011:
	.loc	1 1008 34 is_stmt 1
	movq	144(%r14), %rax
	cmpq	%rax, %r9
	jae	.LBB32_620
	.loc	1 0 34 is_stmt 0
	movq	%rcx, 64(%rsp)
	movq	760(%r14), %r10
	addq	%rdi, %r10
	cmpq	%r15, %r10
	jb	.LBB32_470
	movq	%r15, %rcx
	jmp	.LBB32_472
	.p2align	4
.LBB32_470:
	xorl	%ecx, %ecx
.LBB32_472:
	movq	488(%r14), %r12
	subq	%rcx, %r10
	cmpq	%r12, %r10
	jae	.LBB32_621
.Ltmp2012:
	.loc	1 1012 34 is_stmt 1
	movq	504(%r14), %r8
	cmpq	%r8, %r10
	jae	.LBB32_586
	.loc	1 0 34 is_stmt 0
	movl	%r13d, %ecx
	movq	120(%r14), %rdx
	movq	%rdx, 104(%rsp)
	vmovss	(%rdx,%r9,4), %xmm3
	movq	480(%r14), %rdx
	movq	%rdx, 112(%rsp)
	vmovss	(%rdx,%r10,4), %xmm13
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp2013:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp2014:
	.loc	49 103 24 is_stmt 0
	vandps	%xmm1, %xmm13, %xmm13
	vmovss	.LCPI32_3(%rip), %xmm1
.Ltmp2015:
	.loc	49 71 9 is_stmt 1
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp2016:
	.loc	49 71 9 is_stmt 0
	vmulss	%xmm1, %xmm13, %xmm13
.Ltmp2017:
	.loc	49 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp2018:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r15d
	movl	$841731191, %edx
.Ltmp2019:
	.loc	49 161 24
	jbe	.LBB32_476
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %edx
.LBB32_476:
.Ltmp2020:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp2021:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %r13d
	movl	$8388608, %esi
.Ltmp2022:
	.loc	49 161 24
	jbe	.LBB32_478
.Ltmp2023:
	.loc	49 0 24 is_stmt 0
	movl	%edx, %esi
.LBB32_478:
.Ltmp2024:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2025:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp2026:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2027:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm13
.Ltmp2028:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm13, %xmm13
.Ltmp2029:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp2030:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm13, %xmm13
.Ltmp2031:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp2032:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm13, %xmm13
.Ltmp2033:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp2034:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm13, %xmm13
.Ltmp2035:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp2036:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm13, %xmm13
.Ltmp2037:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp2038:
	.loc	23 1291 18
	vmovd	%esi, %xmm14
.Ltmp2039:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm14, %xmm14
.Ltmp2040:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm3
.Ltmp2041:
	.loc	49 61 9
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2042:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2043:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2044:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm13
.Ltmp2045:
	.loc	49 66 9 is_stmt 1
	vsubss	80(%rsp), %xmm13, %xmm3
.Ltmp2046:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_479
.Ltmp2047:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_481
	.p2align	4
.LBB32_479:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm3, %xmm14
.Ltmp2048:
	vmulss	%xmm14, %xmm14, %xmm14
	vmulss	.LCPI32_18(%rip), %xmm14, %xmm14
.Ltmp2049:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm1, %xmm15
	vblendvps	%xmm15, %xmm14, %xmm3, %xmm3
	vmulss	136(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %edx
.Ltmp2050:
.LBB32_481:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp2051:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2052:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vminss	%xmm1, %xmm3, %xmm3
.Ltmp2053:
	.loc	1 924 9 is_stmt 1
	vmovss	12(%rsp), %xmm14
.Ltmp2054:
	.loc	49 161 24
	vcmpnltss	%xmm14, %xmm3, %xmm15
	vmovaps	544(%rsp), %xmm1
	vblendvps	%xmm15, 656(%rsp), %xmm1, %xmm15
.Ltmp2055:
	.loc	49 66 9
	vsubss	%xmm3, %xmm14, %xmm14
.Ltmp2056:
	.loc	49 92 9
	vmulss	%xmm15, %xmm14, %xmm14
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2057:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm14
.Ltmp2058:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm1
	vucomiss	%xmm14, %xmm1
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_482
	.loc	49 0 24
	movl	$0, 12(%rsp)
	jmp	.LBB32_484
	.p2align	4
.LBB32_482:
	.loc	49 166 0
	vmovss	%xmm3, 12(%rsp)
.Ltmp2059:
.LBB32_484:
	.loc	49 0 0
	movq	136(%r14), %rsi
	vmovss	(%rsi,%r9,4), %xmm3
	movq	496(%r14), %r9
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp2060:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp2061:
	.loc	49 103 24 is_stmt 0
	vmovss	(%r9,%r10,4), %xmm14
	vandps	%xmm1, %xmm14, %xmm14
	vmovss	.LCPI32_3(%rip), %xmm1
.Ltmp2062:
	.loc	49 71 9 is_stmt 1
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp2063:
	.loc	49 71 9 is_stmt 0
	vmulss	%xmm1, %xmm14, %xmm14
.Ltmp2064:
	.loc	49 61 9 is_stmt 1
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2065:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
.Ltmp2066:
	.loc	49 161 24
	jbe	.LBB32_486
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %r15d
.LBB32_486:
.Ltmp2067:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r15d, %xmm3
.Ltmp2068:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
.Ltmp2069:
	.loc	49 161 24
	jbe	.LBB32_488
.Ltmp2070:
	.loc	49 0 24 is_stmt 0
	movl	%r15d, %r13d
.LBB32_488:
.Ltmp2071:
	.loc	49 185 42 is_stmt 1
	movl	%r13d, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2072:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp2073:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2074:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm14
.Ltmp2075:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm14, %xmm14
.Ltmp2076:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp2077:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm14, %xmm14
.Ltmp2078:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp2079:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm14, %xmm14
.Ltmp2080:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp2081:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm14, %xmm14
.Ltmp2082:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp2083:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm14, %xmm14
.Ltmp2084:
	.loc	49 187 28
	shrl	$23, %r13d
	orl	$1258291200, %r13d
.Ltmp2085:
	.loc	23 1291 18
	vmovd	%r13d, %xmm15
.Ltmp2086:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm15, %xmm15
.Ltmp2087:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm3
.Ltmp2088:
	.loc	49 61 9
	vaddss	%xmm3, %xmm15, %xmm3
.Ltmp2089:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2090:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2091:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm15
.Ltmp2092:
	.loc	49 66 9 is_stmt 1
	vsubss	20(%rsp), %xmm15, %xmm3
.Ltmp2093:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_489
.Ltmp2094:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_491
	.p2align	4
.LBB32_489:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm3, %xmm14
.Ltmp2095:
	vmulss	%xmm14, %xmm14, %xmm14
	vmulss	.LCPI32_18(%rip), %xmm14, %xmm14
.Ltmp2096:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm1, %xmm1
	vblendvps	%xmm1, %xmm14, %xmm3, %xmm1
	vmulss	132(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp2097:
.LBB32_491:
	.loc	49 0 0
	movq	480(%rsp), %r15
.Ltmp2098:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp2099:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2100:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp2101:
	.loc	1 924 9 is_stmt 1
	vmovss	16(%rsp), %xmm3
.Ltmp2102:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm14
	vmovaps	640(%rsp), %xmm2
	vblendvps	%xmm14, 624(%rsp), %xmm2, %xmm14
.Ltmp2103:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2104:
	.loc	49 92 9
	vmulss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp2105:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp2106:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm1, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_492
.Ltmp2107:
	.loc	49 0 24
	movl	$0, 16(%rsp)
.Ltmp2108:
	.loc	49 66 9 is_stmt 1
	vsubss	56(%rsp), %xmm13, %xmm3
.Ltmp2109:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jae	.LBB32_496
.Ltmp2110:
.LBB32_495:
	.loc	49 0 44
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm1
.Ltmp2111:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp2112:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm1, %xmm3, %xmm1
	vmulss	128(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp2113:
	.loc	49 161 44
	jmp	.LBB32_497
.Ltmp2114:
	.loc	49 0 44
.Ltmp2115:
	.p2align	4
.LBB32_492:
	.loc	49 166 0 is_stmt 1
	vmovss	%xmm3, 16(%rsp)
.Ltmp2116:
	.loc	49 66 9
	vsubss	56(%rsp), %xmm13, %xmm3
.Ltmp2117:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_495
.LBB32_496:
	.loc	49 0 44
	xorl	%edx, %edx
.LBB32_497:
.Ltmp2118:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp2119:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2120:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp2121:
	.loc	1 924 9 is_stmt 1
	vmovd	%ebx, %xmm3
.Ltmp2122:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm13
	vmovaps	608(%rsp), %xmm2
	vblendvps	%xmm13, 592(%rsp), %xmm2, %xmm13
.Ltmp2123:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2124:
	.loc	49 92 9
	vmulss	%xmm3, %xmm13, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp2125:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp2126:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_498
.Ltmp2127:
	.loc	49 0 24
	xorl	%ebx, %ebx
.Ltmp2128:
	.loc	49 66 9 is_stmt 1
	vsubss	304(%rsp), %xmm15, %xmm3
.Ltmp2129:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jae	.LBB32_502
.Ltmp2130:
.LBB32_501:
	.loc	49 0 44
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm1
.Ltmp2131:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp2132:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm1, %xmm3, %xmm1
	vmulss	300(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %edx
.Ltmp2133:
	.loc	49 161 44
	jmp	.LBB32_503
.Ltmp2134:
	.loc	49 0 44
.Ltmp2135:
	.p2align	4
.LBB32_498:
	.loc	49 166 0 is_stmt 1
	vmovd	%xmm3, %ebx
.Ltmp2136:
	.loc	49 66 9
	vsubss	304(%rsp), %xmm15, %xmm3
.Ltmp2137:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_501
.LBB32_502:
	.loc	49 0 44
	xorl	%edx, %edx
.LBB32_503:
.Ltmp2138:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm1
.Ltmp2139:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2140:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp2141:
	.loc	1 924 9 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp2142:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm13
	vmovaps	576(%rsp), %xmm2
	vblendvps	%xmm13, 560(%rsp), %xmm2, %xmm13
.Ltmp2143:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2144:
	.loc	49 92 9
	vmulss	%xmm3, %xmm13, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp2145:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp2146:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_504
	.loc	49 0 24
	xorl	%r13d, %r13d
	jmp	.LBB32_506
	.p2align	4
.LBB32_504:
	.loc	49 166 0
	vmovd	%xmm3, %r13d
.Ltmp2147:
.LBB32_506:
	.loc	49 0 0
	movq	64(%rsp), %rcx
	negq	%rcx
	addq	%rcx, %rdi
	incq	%rdi
.Ltmp2148:
	.loc	48 568 12 is_stmt 1
	cmpq	%r11, %rdi
	ja	.LBB32_588
.Ltmp2149:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2150:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_190
.Ltmp2151:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2152:
	.loc	48 568 12
	cmpq	%r12, %rdi
	ja	.LBB32_589
.Ltmp2153:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2154:
	.loc	48 568 12
	cmpq	%r8, %rdi
	ja	.LBB32_590
.Ltmp2155:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2156:
	.loc	49 0 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm1
	vmovaps	160(%rsp), %xmm10
	vmulss	120(%rsp), %xmm10, %xmm2
	vaddss	%xmm1, %xmm2, %xmm1
	vmulss	%xmm0, %xmm9, %xmm0
	vmovaps	192(%rsp), %xmm13
	vmulss	%xmm11, %xmm13, %xmm2
	vaddss	%xmm0, %xmm2, %xmm0
	vaddss	%xmm5, %xmm5, %xmm2
	vaddss	144(%rsp), %xmm2, %xmm3
	vmovss	24(%rsp), %xmm2
	vaddss	%xmm2, %xmm2, %xmm2
	vaddss	272(%rsp), %xmm2, %xmm7
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	%xmm4, %xmm4, %xmm2
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm6, %xmm6, %xmm5
	vaddss	%xmm0, %xmm0, %xmm6
	vaddss	%xmm12, %xmm12, %xmm8
	vaddss	%xmm1, %xmm10, %xmm9
	vaddss	224(%rsp), %xmm2, %xmm0
	vaddss	240(%rsp), %xmm4, %xmm1
	vaddss	256(%rsp), %xmm5, %xmm2
	vaddss	%xmm6, %xmm13, %xmm4
	vaddss	208(%rsp), %xmm8, %xmm5
	vmovss	12(%rsp), %xmm6
	vaddss	28(%rsp), %xmm6, %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm14
	vandps	%xmm3, %xmm14, %xmm8
	vandps	%xmm7, %xmm14, %xmm10
	vandps	%xmm14, %xmm9, %xmm11
	vandps	%xmm0, %xmm14, %xmm12
	vandps	%xmm1, %xmm14, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm15
	vcmpnltss	%xmm15, %xmm8, %xmm8
	vandps	%xmm3, %xmm8, %xmm3
	vmovaps	%xmm3, 144(%rsp)
	vandps	%xmm2, %xmm14, %xmm3
	vcmpnltss	%xmm15, %xmm10, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovaps	%xmm7, 272(%rsp)
	vandps	%xmm4, %xmm14, %xmm7
	vcmpnltss	%xmm15, %xmm11, %xmm8
	vandps	%xmm9, %xmm8, %xmm8
	vmovaps	%xmm8, 160(%rsp)
	vandps	%xmm5, %xmm14, %xmm8
	vmovss	.LCPI32_21(%rip), %xmm11
	vmulss	%xmm6, %xmm11, %xmm6
	vcmpnltss	%xmm15, %xmm12, %xmm9
	vcmpnltss	%xmm15, %xmm13, %xmm10
	vcmpnltss	%xmm15, %xmm3, %xmm3
	vcmpnltss	%xmm15, %xmm7, %xmm7
	vcmpnltss	%xmm15, %xmm8, %xmm8
	vmovss	.LCPI32_22(%rip), %xmm12
	vmaxss	%xmm12, %xmm6, %xmm6
	vandps	%xmm0, %xmm9, %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vandps	%xmm1, %xmm10, %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vandps	%xmm2, %xmm3, %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vandps	%xmm4, %xmm7, %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vandps	%xmm5, %xmm8, %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm7
	vminss	%xmm7, %xmm6, %xmm0
	vroundss	$9, %xmm0, %xmm0, %xmm2
	vsubss	%xmm2, %xmm0, %xmm4
	vmovss	16(%rsp), %xmm0
	vaddss	32(%rsp), %xmm0, %xmm0
	vmulss	%xmm0, %xmm11, %xmm0
	vmaxss	%xmm12, %xmm0, %xmm0
	vminss	%xmm7, %xmm0, %xmm1
	vroundss	$9, %xmm1, %xmm1, %xmm0
	vsubss	%xmm0, %xmm1, %xmm1
	vmovd	%ebx, %xmm3
	vaddss	432(%rsp), %xmm3, %xmm3
	vmulss	%xmm3, %xmm11, %xmm3
	vmaxss	%xmm12, %xmm3, %xmm3
	vminss	%xmm7, %xmm3, %xmm5
	vmovss	.LCPI32_24(%rip), %xmm6
	vmulss	%xmm6, %xmm4, %xmm3
	vmovss	.LCPI32_25(%rip), %xmm8
	vaddss	%xmm3, %xmm8, %xmm3
	vmulss	%xmm3, %xmm4, %xmm3
	vmovss	.LCPI32_26(%rip), %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
	vmulss	%xmm3, %xmm4, %xmm3
	vmovss	.LCPI32_27(%rip), %xmm10
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm4, %xmm3
	vmovss	.LCPI32_28(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm3
	vmulss	%xmm3, %xmm4, %xmm3
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
	vmovss	.LCPI32_29(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm3
	vmovss	.LCPI32_30(%rip), %xmm15
	vaddss	%xmm2, %xmm15, %xmm2
	vmovd	%xmm2, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm2
	vmulss	%xmm2, %xmm3, %xmm2
	vmulss	%xmm6, %xmm1, %xmm3
	vaddss	%xmm3, %xmm8, %xmm3
	vmulss	%xmm3, %xmm1, %xmm3
	vaddss	%xmm3, %xmm9, %xmm3
	vmulss	%xmm3, %xmm1, %xmm3
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm1, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp2157:
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp2158:
	vmovd	%r13d, %xmm3
	vaddss	72(%rsp), %xmm3, %xmm3
	vmulss	%xmm3, %xmm11, %xmm3
	vmaxss	%xmm12, %xmm3, %xmm3
	vminss	%xmm7, %xmm3, %xmm3
.Ltmp2159:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm0, %xmm15, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp2160:
	vmulss	%xmm6, %xmm5, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm9, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm10, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp2161:
	vmulss	%xmm1, %xmm5, %xmm1
.Ltmp2162:
	vroundss	$9, %xmm3, %xmm3, %xmm5
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp2163:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm4, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	vmulss	%xmm4, %xmm1, %xmm1
.Ltmp2164:
	vmulss	%xmm6, %xmm3, %xmm4
	vaddss	%xmm4, %xmm8, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm9, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm10, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp2165:
	vmulss	%xmm4, %xmm3, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm5, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	104(%rsp), %rax
.Ltmp2166:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm2
	vmulss	(%rsi,%rdi,4), %xmm0, %xmm0
.Ltmp2167:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp2168:
	vaddss	%xmm0, %xmm2, %xmm0
	movq	112(%rsp), %rax
.Ltmp2169:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
.Ltmp2170:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r9,%rdi,4), %xmm3, %xmm2
.Ltmp2171:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	96(%rsp), %rsi
.Ltmp2172:
	.loc	49 56 9
	vmovss	%xmm0, (%rsi,%rbp,4)
	movq	88(%rsp), %r8
.Ltmp2173:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r8,%rbp,4)
	incq	%rbp
.Ltmp2174:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rbp, 336(%rsp)
.Ltmp2175:
	.loc	3 900 12
	jne	.LBB32_453
	jmp	.LBB32_515
.Ltmp2176:
.LBB32_101:
	.loc	1 1142 5
	testb	%al, %al
	je	.LBB32_102
.LBB32_263:
.Ltmp2177:
	.loc	1 1083 23
	movl	856(%r14), %eax
	movl	%eax, 140(%rsp)
.Ltmp2178:
	.loc	1 1084 20
	movq	840(%r14), %rbx
	leaq	120(%r14), %r15
	leaq	480(%r14), %r13
	xorl	%r12d, %r12d
	jmp	.LBB32_264
.LBB32_267:
	.loc	1 0 20 is_stmt 0
	movq	%rax, %rdi
	leaq	120(%r14), %r15
	vmovss	48(%rsp), %xmm2
	vmovss	44(%rsp), %xmm3
	vmovss	40(%rsp), %xmm4
	vmovss	36(%rsp), %xmm5
	vmovss	544(%rsp), %xmm6
	vmovss	656(%rsp), %xmm7
	vmovss	640(%rsp), %xmm8
	vmovss	624(%rsp), %xmm9
	vmovss	608(%rsp), %xmm10
	vmovss	592(%rsp), %xmm11
	vmovss	576(%rsp), %xmm12
	vmovss	560(%rsp), %xmm13
	vmovss	480(%rsp), %xmm14
	vmovss	320(%rsp), %xmm15
.LBB32_304:
.Ltmp2179:
	.loc	1 1057 5 is_stmt 1
	vmovaps	880(%rsp), %xmm0
	leaq	168(%r14), %rax
	vmovups	%xmm0, (%rax)
	.loc	1 1058 5
	vmovaps	896(%rsp), %xmm0
	leaq	528(%r14), %rax
	vmovups	%xmm0, (%rax)
	.loc	1 1059 5
	movq	%rcx, 184(%r14)
	.loc	1 1060 5
	movq	%r8, 544(%r14)
	.loc	1 1061 5
	movq	%rdi, 848(%r14)
	vmovss	432(%rsp), %xmm0
.Ltmp2180:
	.loc	1 1197 17
	vmovss	%xmm0, 192(%r14)
	.loc	1 1198 34
	movl	204(%r14), %eax
.Ltmp2181:
	.loc	38 2472 13
	subl	%ebp, %eax
	movl	$0, %edx
	cmovbl	%edx, %eax
.Ltmp2182:
	.loc	1 1198 34
	movl	220(%r14), %ecx
	.loc	1 1198 17 is_stmt 0
	movl	%eax, 204(%r14)
	vmovss	72(%rsp), %xmm0
	.loc	1 1197 17 is_stmt 1
	vmovss	%xmm0, 208(%r14)
.Ltmp2183:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edx, %ecx
.Ltmp2184:
	.loc	1 1198 17
	movl	%ecx, 220(%r14)
	vmovss	80(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 224(%r14)
	.loc	1 1198 34
	movl	236(%r14), %eax
.Ltmp2185:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2186:
	.loc	1 1198 17
	movl	%eax, 236(%r14)
	vmovss	20(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 240(%r14)
	.loc	1 1198 34
	movl	252(%r14), %eax
.Ltmp2187:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2188:
	.loc	1 1198 17
	movl	%eax, 252(%r14)
	vmovss	56(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 256(%r14)
	.loc	1 1198 34
	movl	268(%r14), %eax
.Ltmp2189:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2190:
	.loc	1 1198 17
	movl	%eax, 268(%r14)
	vmovss	304(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 272(%r14)
	.loc	1 1198 34
	movl	284(%r14), %eax
.Ltmp2191:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2192:
	.loc	1 1198 17
	movl	%eax, 284(%r14)
	.loc	1 1197 17
	vmovss	%xmm15, 288(%r14)
	.loc	1 1198 34
	movl	300(%r14), %eax
.Ltmp2193:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2194:
	.loc	1 1198 17
	movl	%eax, 300(%r14)
	.loc	1 1197 17
	vmovss	%xmm14, 304(%r14)
	.loc	1 1198 34
	movl	316(%r14), %eax
.Ltmp2195:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2196:
	.loc	1 1198 17
	movl	%eax, 316(%r14)
	.loc	1 1197 17
	vmovss	%xmm13, 320(%r14)
	.loc	1 1198 34
	movl	332(%r14), %eax
.Ltmp2197:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2198:
	.loc	1 1198 17
	movl	%eax, 332(%r14)
	.loc	1 1197 17
	vmovss	%xmm12, 336(%r14)
	.loc	1 1198 34
	movl	348(%r14), %eax
.Ltmp2199:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2200:
	.loc	1 1198 17
	movl	%eax, 348(%r14)
.Ltmp2201:
	.loc	1 1197 17
	vmovss	%xmm11, 552(%r14)
	.loc	1 1198 34
	movl	564(%r14), %eax
.Ltmp2202:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2203:
	.loc	1 1198 17
	movl	%eax, 564(%r14)
	.loc	1 1197 17
	vmovss	%xmm10, 568(%r14)
	.loc	1 1198 34
	movl	580(%r14), %eax
.Ltmp2204:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2205:
	.loc	1 1198 17
	movl	%eax, 580(%r14)
	.loc	1 1197 17
	vmovss	%xmm9, 584(%r14)
	.loc	1 1198 34
	movl	596(%r14), %eax
.Ltmp2206:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2207:
	.loc	1 1198 17
	movl	%eax, 596(%r14)
	.loc	1 1197 17
	vmovss	%xmm8, 600(%r14)
	.loc	1 1198 34
	movl	612(%r14), %eax
.Ltmp2208:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2209:
	.loc	1 1198 17
	movl	%eax, 612(%r14)
	.loc	1 1197 17
	vmovss	%xmm7, 616(%r14)
	.loc	1 1198 34
	movl	628(%r14), %eax
.Ltmp2210:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2211:
	.loc	1 1198 17
	movl	%eax, 628(%r14)
	.loc	1 1197 17
	vmovss	%xmm6, 632(%r14)
	.loc	1 1198 34
	movl	644(%r14), %eax
.Ltmp2212:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2213:
	.loc	1 1198 17
	movl	%eax, 644(%r14)
	.loc	1 1197 17
	vmovss	%xmm5, 648(%r14)
	.loc	1 1198 34
	movl	660(%r14), %eax
.Ltmp2214:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2215:
	.loc	1 1198 17
	movl	%eax, 660(%r14)
	.loc	1 1197 17
	vmovss	%xmm4, 664(%r14)
	.loc	1 1198 34
	movl	676(%r14), %eax
.Ltmp2216:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2217:
	.loc	1 1198 17
	movl	%eax, 676(%r14)
	.loc	1 1197 17
	vmovss	%xmm3, 680(%r14)
	.loc	1 1198 34
	movl	692(%r14), %eax
.Ltmp2218:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2219:
	.loc	1 1198 17
	movl	%eax, 692(%r14)
	.loc	1 1197 17
	vmovss	%xmm2, 696(%r14)
	.loc	1 1198 34
	movl	708(%r14), %eax
.Ltmp2220:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp2221:
	.loc	1 1198 17
	movl	%eax, 708(%r14)
.Ltmp2222:
.LBB32_286:
	.loc	1 0 17 is_stmt 0
	movq	%rsi, %r12
	movq	184(%rsp), %r9
	.loc	1 1086 11 is_stmt 1
	cmpq	%r9, %rsi
	jae	.LBB32_517
.LBB32_264:
	.loc	1 1087 42
	movq	%r9, %rsi
	subq	%r12, %rsi
	.loc	1 1087 29 is_stmt 0
	leaq	104(%r14), %rdi
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_
	movb	%dl, 52(%rsp)
	movq	%rax, %rbp
	vmovss	192(%r14), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	200(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	208(%r14), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	216(%r14), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	224(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	232(%r14), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	240(%r14), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	248(%r14), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	256(%r14), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	264(%r14), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	272(%r14), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	280(%r14), %xmm0
	vmovss	%xmm0, 144(%rsp)
	vmovss	288(%r14), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	296(%r14), %xmm0
	vmovss	%xmm0, 272(%rsp)
	vmovss	304(%r14), %xmm0
	vmovss	%xmm0, 480(%rsp)
	vmovss	312(%r14), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	320(%r14), %xmm0
	vmovss	%xmm0, 560(%rsp)
	vmovss	328(%r14), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	336(%r14), %xmm0
	vmovss	%xmm0, 576(%rsp)
	vmovss	344(%r14), %xmm0
	vmovss	%xmm0, 224(%rsp)
	vmovss	552(%r14), %xmm0
	vmovss	%xmm0, 592(%rsp)
	vmovss	560(%r14), %xmm0
	vmovss	%xmm0, 208(%rsp)
	vmovss	568(%r14), %xmm0
	vmovss	%xmm0, 608(%rsp)
	vmovss	576(%r14), %xmm0
	vmovss	%xmm0, 192(%rsp)
	vmovss	584(%r14), %xmm0
	vmovss	%xmm0, 624(%rsp)
	vmovss	592(%r14), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	600(%r14), %xmm0
	vmovss	%xmm0, 640(%rsp)
	vmovss	608(%r14), %xmm0
	vmovss	%xmm0, 64(%rsp)
	vmovss	616(%r14), %xmm0
	vmovss	%xmm0, 656(%rsp)
	vmovss	624(%r14), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	632(%r14), %xmm0
	vmovss	%xmm0, 544(%rsp)
	vmovss	640(%r14), %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	648(%r14), %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	656(%r14), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	664(%r14), %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	672(%r14), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	680(%r14), %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	688(%r14), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	696(%r14), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	704(%r14), %xmm0
	vmovss	%xmm0, 88(%rsp)
.Ltmp2223:
	.loc	1 1091 31 is_stmt 1
	leaq	520(%rsp), %rdi
	movq	%r15, %rsi
	movl	140(%rsp), %r15d
	movl	%r15d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	712(%rsp), %rdi
	movq	%r13, %rsi
	movl	%r15d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	movq	184(%rsp), %rdx
.Ltmp2224:
	.loc	1 0 0 is_stmt 0
	movq	%rbp, %rsi
	addq	%r12, %rsi
	setb	%cl
	cmpq	%rdx, %rsi
	seta	%al
.Ltmp2225:
	.loc	38 1050 16 is_stmt 1
	orb	%cl, %al
.Ltmp2226:
	.loc	1 1094 12
	testb	$1, 52(%rsp)
	je	.LBB32_268
.Ltmp2227:
	.loc	38 1050 16
	testb	%al, %al
	jne	.LBB32_290
.Ltmp2228:
	.loc	1 972 27
	leaq	168(%r14), %rax
	vmovups	(%rax), %xmm0
	vmovaps	%xmm0, 880(%rsp)
.Ltmp2229:
	.loc	1 973 26
	leaq	528(%r14), %rax
	vmovups	(%rax), %xmm0
	vmovaps	%xmm0, 896(%rsp)
.Ltmp2230:
	.loc	1 974 25
	movq	184(%r14), %rcx
.Ltmp2231:
	.loc	1 975 24
	movq	544(%r14), %r8
.Ltmp2232:
	.loc	1 976 24
	movq	848(%r14), %rax
.Ltmp2233:
	.loc	2 1916 50
	testq	%rbp, %rbp
.Ltmp2234:
	.loc	3 900 12
	je	.LBB32_267
.Ltmp2235:
	.loc	3 0 12 is_stmt 0
	movq	464(%rsp), %rdx
	leaq	(%rdx,%r12,4), %r9
	movq	472(%rsp), %rdx
	leaq	(%rdx,%r12,4), %r10
	xorl	%r11d, %r11d
	leaq	120(%r14), %r15
	vmovss	48(%rsp), %xmm2
	vmovss	44(%rsp), %xmm3
	vmovss	40(%rsp), %xmm4
	vmovss	36(%rsp), %xmm5
	vmovss	544(%rsp), %xmm6
	vmovss	656(%rsp), %xmm7
	vmovss	640(%rsp), %xmm8
	vmovss	624(%rsp), %xmm9
	vmovss	608(%rsp), %xmm10
	vmovss	592(%rsp), %xmm11
	vmovss	576(%rsp), %xmm12
	vmovss	560(%rsp), %xmm13
	vmovss	480(%rsp), %xmm14
	vmovss	320(%rsp), %xmm15
	.p2align	4
.LBB32_292:
.Ltmp2236:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rax), %rdx
.Ltmp2237:
	.loc	1 857 8
	cmpq	%rbx, %rdx
	jb	.LBB32_293
.Ltmp2238:
	.loc	1 0 8 is_stmt 0
	movq	%rbx, %rdi
	jmp	.LBB32_295
	.p2align	4
.LBB32_293:
	xorl	%edi, %edi
.LBB32_295:
.Ltmp2239:
	.loc	1 991 35 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp2240:
	.loc	48 580 12
	cmpq	%rdx, %rax
	ja	.LBB32_613
.Ltmp2241:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2242:
	.loc	1 0 0 is_stmt 0
	vmovss	(%r9,%r11,4), %xmm1
.Ltmp2243:
	vmovss	(%r10,%r11,4), %xmm0
.Ltmp2244:
	.loc	1 991 0 is_stmt 1
	movq	(%r15), %rdx
.Ltmp2245:
	.loc	49 56 9
	vmovss	%xmm1, (%rdx,%rax,4)
.Ltmp2246:
	.loc	1 992 34
	movq	488(%r14), %rdx
.Ltmp2247:
	.loc	48 580 12
	cmpq	%rdx, %rax
	ja	.LBB32_614
.Ltmp2248:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2249:
	.loc	1 0 0 is_stmt 0
	negq	%rdi
	addq	%rax, %rdi
	incq	%rdi
.Ltmp2250:
	.loc	1 992 0 is_stmt 1
	movq	(%r13), %rdx
.Ltmp2251:
	.loc	49 56 9
	vmovss	%xmm0, (%rdx,%rax,4)
.Ltmp2252:
	.loc	1 993 22
	movq	128(%r14), %rdx
.Ltmp2253:
	.loc	48 568 12
	cmpq	%rdx, %rdi
	ja	.LBB32_615
.Ltmp2254:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2255:
	.loc	1 993 0
	movq	(%r15), %rax
.Ltmp2256:
	.loc	49 51 9
	vmovss	(%rax,%rdi,4), %xmm0
.Ltmp2257:
	.loc	49 56 9
	vmovss	%xmm0, (%r9,%r11,4)
.Ltmp2258:
	.loc	1 994 22
	movq	488(%r14), %rdx
.Ltmp2259:
	.loc	48 568 12
	cmpq	%rdx, %rdi
	ja	.LBB32_616
.Ltmp2260:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2261:
	.loc	1 994 0
	movq	(%r13), %rax
.Ltmp2262:
	.loc	49 51 9
	vmovss	(%rax,%rdi,4), %xmm0
.Ltmp2263:
	.loc	49 56 9
	vmovss	%xmm0, (%r10,%r11,4)
	vmovss	432(%rsp), %xmm0
.Ltmp2264:
	.loc	1 0 0 is_stmt 0
	vaddss	32(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 432(%rsp)
	vaddss	208(%rsp), %xmm11, %xmm11
	vmovss	72(%rsp), %xmm0
	vaddss	28(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 72(%rsp)
	vaddss	192(%rsp), %xmm10, %xmm10
	vmovss	80(%rsp), %xmm0
	vaddss	16(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 80(%rsp)
	vaddss	336(%rsp), %xmm9, %xmm9
	vmovss	20(%rsp), %xmm0
	vaddss	12(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 20(%rsp)
	vaddss	64(%rsp), %xmm8, %xmm8
	vmovss	56(%rsp), %xmm0
	vaddss	160(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 56(%rsp)
	vaddss	24(%rsp), %xmm7, %xmm7
	vmovss	304(%rsp), %xmm0
	vaddss	144(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 304(%rsp)
	vaddss	120(%rsp), %xmm6, %xmm6
	vaddss	272(%rsp), %xmm15, %xmm15
	vaddss	112(%rsp), %xmm5, %xmm5
	vaddss	256(%rsp), %xmm14, %xmm14
	vaddss	104(%rsp), %xmm4, %xmm4
	vaddss	240(%rsp), %xmm13, %xmm13
	vaddss	96(%rsp), %xmm3, %xmm3
	vaddss	224(%rsp), %xmm12, %xmm12
	vaddss	88(%rsp), %xmm2, %xmm2
	incq	%r11
	movq	%rdi, %rax
.Ltmp2265:
	.loc	2 1916 50 is_stmt 1
	cmpq	%r11, %rbp
.Ltmp2266:
	.loc	3 900 12
	jne	.LBB32_292
	jmp	.LBB32_304
.Ltmp2267:
	.loc	3 0 12 is_stmt 0
.Ltmp2268:
	.p2align	4
.LBB32_268:
	.loc	38 1050 16 is_stmt 1
	testb	%al, %al
	jne	.LBB32_271
.Ltmp2269:
	.loc	1 972 27
	leaq	168(%r14), %rax
	vmovups	(%rax), %xmm0
	vmovaps	%xmm0, 912(%rsp)
.Ltmp2270:
	.loc	1 973 26
	leaq	528(%r14), %rax
	vmovups	(%rax), %xmm0
	vmovaps	%xmm0, 928(%rsp)
.Ltmp2271:
	.loc	1 974 25
	movq	184(%r14), %rcx
.Ltmp2272:
	.loc	1 975 24
	movq	544(%r14), %r8
.Ltmp2273:
	.loc	1 976 24
	movq	848(%r14), %rdi
.Ltmp2274:
	.loc	2 1916 50
	testq	%rbp, %rbp
.Ltmp2275:
	.loc	3 900 12
	je	.LBB32_270
.Ltmp2276:
	.loc	3 0 12 is_stmt 0
	movq	464(%rsp), %rax
	leaq	(%rax,%r12,4), %r9
	movq	472(%rsp), %rax
	leaq	(%rax,%r12,4), %r10
	xorl	%r11d, %r11d
	leaq	120(%r14), %r15
	.p2align	4
.LBB32_273:
.Ltmp2277:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp2278:
	.loc	1 857 8
	cmpq	%rbx, %rax
	jb	.LBB32_274
.Ltmp2279:
	.loc	1 0 8 is_stmt 0
	movq	%rbx, %rax
	jmp	.LBB32_276
	.p2align	4
.LBB32_274:
	xorl	%eax, %eax
.LBB32_276:
.Ltmp2280:
	.loc	1 991 35 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp2281:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_611
.Ltmp2282:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2283:
	.loc	1 0 0 is_stmt 0
	vmovss	(%r9,%r11,4), %xmm1
.Ltmp2284:
	vmovss	(%r10,%r11,4), %xmm0
.Ltmp2285:
	.loc	1 991 0 is_stmt 1
	movq	(%r15), %rdx
.Ltmp2286:
	.loc	49 56 9
	vmovss	%xmm1, (%rdx,%rdi,4)
.Ltmp2287:
	.loc	1 992 34
	movq	488(%r14), %rdx
.Ltmp2288:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_612
.Ltmp2289:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2290:
	.loc	1 0 0 is_stmt 0
	negq	%rax
	addq	%rdi, %rax
	incq	%rax
.Ltmp2291:
	.loc	1 992 0 is_stmt 1
	movq	(%r13), %rdx
.Ltmp2292:
	.loc	49 56 9
	vmovss	%xmm0, (%rdx,%rdi,4)
.Ltmp2293:
	.loc	1 993 22
	movq	128(%r14), %rdx
.Ltmp2294:
	.loc	48 568 12
	cmpq	%rdx, %rax
	ja	.LBB32_287
.Ltmp2295:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2296:
	.loc	1 993 0
	movq	(%r15), %rdx
.Ltmp2297:
	.loc	49 51 9
	vmovss	(%rdx,%rax,4), %xmm0
.Ltmp2298:
	.loc	49 56 9
	vmovss	%xmm0, (%r9,%r11,4)
.Ltmp2299:
	.loc	1 994 22
	movq	488(%r14), %rdx
.Ltmp2300:
	.loc	48 568 12
	cmpq	%rdx, %rax
	ja	.LBB32_289
.Ltmp2301:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
	.loc	1 994 0 is_stmt 1
	movq	(%r13), %rdx
.Ltmp2302:
	.loc	49 51 9
	vmovss	(%rdx,%rax,4), %xmm0
.Ltmp2303:
	.loc	49 56 9
	vmovss	%xmm0, (%r10,%r11,4)
.Ltmp2304:
	.loc	1 0 0 is_stmt 0
	incq	%r11
	movq	%rax, %rdi
.Ltmp2305:
	.loc	2 1916 50 is_stmt 1
	cmpq	%r11, %rbp
.Ltmp2306:
	.loc	3 900 12
	jne	.LBB32_273
	jmp	.LBB32_285
.Ltmp2307:
.LBB32_270:
	.loc	3 0 12 is_stmt 0
	movq	%rdi, %rax
	leaq	120(%r14), %r15
.LBB32_285:
	.loc	1 1057 5 is_stmt 1
	vmovaps	912(%rsp), %xmm0
	leaq	168(%r14), %rdx
	vmovups	%xmm0, (%rdx)
	.loc	1 1058 5
	vmovaps	928(%rsp), %xmm0
	leaq	528(%r14), %rdx
	vmovups	%xmm0, (%rdx)
	.loc	1 1059 5
	movq	%rcx, 184(%r14)
	.loc	1 1060 5
	movq	%r8, 544(%r14)
	.loc	1 1061 5
	movq	%rax, 848(%r14)
	jmp	.LBB32_286
.Ltmp2308:
.LBB32_102:
	.loc	1 1083 23
	movl	856(%r14), %eax
	movl	%eax, 456(%rsp)
.Ltmp2309:
	.loc	1 1084 20
	movq	840(%r14), %r13
	xorl	%ebp, %ebp
	movq	%r13, 320(%rsp)
	jmp	.LBB32_103
	.loc	1 0 20 is_stmt 0
.Ltmp2310:
	.p2align	4
.LBB32_260:
	vmovaps	240(%rsp), %xmm0
.Ltmp2311:
	.loc	1 1057 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	208(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	272(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	256(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	224(%rsp), %xmm0
	.loc	1 1058 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	304(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	336(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	144(%rsp), %eax
	.loc	1 1059 5
	movl	%eax, 184(%r14)
	movl	160(%rsp), %eax
	movl	%eax, 188(%r14)
	movl	12(%rsp), %eax
	.loc	1 1060 5
	movl	%eax, 544(%r14)
	movl	%ebx, 548(%r14)
	.loc	1 1061 5
	movq	%rdi, 848(%r14)
	vmovss	72(%rsp), %xmm0
.Ltmp2312:
	.loc	1 1197 17
	vmovss	%xmm0, 192(%r14)
	.loc	1 1198 34
	movl	204(%r14), %eax
	movq	64(%rsp), %rsi
.Ltmp2313:
	.loc	38 2472 13
	subl	%esi, %eax
	movl	$0, %edx
	cmovbl	%edx, %eax
.Ltmp2314:
	.loc	1 1198 34
	movl	220(%r14), %ecx
	.loc	1 1198 17 is_stmt 0
	movl	%eax, 204(%r14)
	vmovss	544(%rsp), %xmm0
	.loc	1 1197 17 is_stmt 1
	vmovss	%xmm0, 208(%r14)
.Ltmp2315:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp2316:
	.loc	1 1198 17
	movl	%ecx, 220(%r14)
	vmovss	36(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 224(%r14)
	.loc	1 1198 34
	movl	236(%r14), %eax
.Ltmp2317:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2318:
	.loc	1 1198 34
	movl	252(%r14), %ecx
	.loc	1 1198 17 is_stmt 0
	movl	%eax, 236(%r14)
	vmovss	40(%rsp), %xmm0
	.loc	1 1197 17 is_stmt 1
	vmovss	%xmm0, 240(%r14)
.Ltmp2319:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp2320:
	.loc	1 1198 17
	movl	%ecx, 252(%r14)
	vmovss	16(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 256(%r14)
	.loc	1 1198 34
	movl	268(%r14), %eax
.Ltmp2321:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2322:
	.loc	1 1198 17
	movl	%eax, 268(%r14)
	vmovss	80(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 272(%r14)
	.loc	1 1198 34
	movl	284(%r14), %eax
.Ltmp2323:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2324:
	.loc	1 1198 17
	movl	%eax, 284(%r14)
	vmovss	44(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 288(%r14)
	.loc	1 1198 34
	movl	300(%r14), %eax
.Ltmp2325:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2326:
	.loc	1 1198 17
	movl	%eax, 300(%r14)
	vmovss	48(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 304(%r14)
	.loc	1 1198 34
	movl	316(%r14), %eax
.Ltmp2327:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2328:
	.loc	1 1198 17
	movl	%eax, 316(%r14)
	vmovss	52(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 320(%r14)
	.loc	1 1198 34
	movl	332(%r14), %eax
.Ltmp2329:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2330:
	.loc	1 1198 17
	movl	%eax, 332(%r14)
	vmovss	28(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 336(%r14)
	.loc	1 1198 34
	movl	348(%r14), %eax
.Ltmp2331:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2332:
	.loc	1 1198 17
	movl	%eax, 348(%r14)
	vmovss	20(%rsp), %xmm0
.Ltmp2333:
	.loc	1 1197 17
	vmovss	%xmm0, 552(%r14)
	.loc	1 1198 34
	movl	564(%r14), %eax
.Ltmp2334:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2335:
	.loc	1 1198 17
	movl	%eax, 564(%r14)
	vmovss	88(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 568(%r14)
	.loc	1 1198 34
	movl	580(%r14), %eax
.Ltmp2336:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2337:
	.loc	1 1198 17
	movl	%eax, 580(%r14)
	vmovss	96(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 584(%r14)
	.loc	1 1198 34
	movl	596(%r14), %eax
.Ltmp2338:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2339:
	.loc	1 1198 17
	movl	%eax, 596(%r14)
	vmovss	104(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 600(%r14)
	.loc	1 1198 34
	movl	612(%r14), %eax
.Ltmp2340:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2341:
	.loc	1 1198 17
	movl	%eax, 612(%r14)
	vmovss	32(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 616(%r14)
	.loc	1 1198 34
	movl	628(%r14), %eax
.Ltmp2342:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2343:
	.loc	1 1198 17
	movl	%eax, 628(%r14)
	vmovss	56(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 632(%r14)
	.loc	1 1198 34
	movl	644(%r14), %eax
.Ltmp2344:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2345:
	.loc	1 1198 17
	movl	%eax, 644(%r14)
	vmovss	112(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 648(%r14)
	.loc	1 1198 34
	movl	660(%r14), %eax
.Ltmp2346:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2347:
	.loc	1 1198 17
	movl	%eax, 660(%r14)
	vmovss	120(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 664(%r14)
	.loc	1 1198 34
	movl	676(%r14), %eax
.Ltmp2348:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2349:
	.loc	1 1198 17
	movl	%eax, 676(%r14)
	vmovss	24(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 680(%r14)
	.loc	1 1198 34
	movl	692(%r14), %eax
.Ltmp2350:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2351:
	.loc	1 1198 17
	movl	%eax, 692(%r14)
	vmovss	432(%rsp), %xmm0
	.loc	1 1197 17
	vmovss	%xmm0, 696(%r14)
	.loc	1 1198 34
	movl	708(%r14), %eax
.Ltmp2352:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2353:
	.loc	1 1198 17
	movl	%eax, 708(%r14)
.Ltmp2354:
.LBB32_188:
	.loc	1 0 17 is_stmt 0
	movq	504(%rsp), %rax
	movq	%rax, %rbp
	movq	184(%rsp), %r9
	.loc	1 1086 11 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB32_517
.LBB32_103:
	.loc	1 1087 42
	movq	%r9, %rsi
	subq	%rbp, %rsi
	.loc	1 1087 29 is_stmt 0
	leaq	104(%r14), %rdi
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r15
	vmovss	192(%r14), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	200(%r14), %xmm0
	vmovss	%xmm0, 688(%rsp)
	vmovss	208(%r14), %xmm0
	vmovss	%xmm0, 544(%rsp)
	vmovss	216(%r14), %xmm0
	vmovss	%xmm0, 300(%rsp)
	vmovss	224(%r14), %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	232(%r14), %xmm0
	vmovss	%xmm0, 412(%rsp)
	vmovss	240(%r14), %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	248(%r14), %xmm0
	vmovss	%xmm0, 408(%rsp)
	vmovss	256(%r14), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	264(%r14), %xmm0
	vmovss	%xmm0, 404(%rsp)
	vmovss	272(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	280(%r14), %xmm0
	vmovss	%xmm0, 424(%rsp)
	vmovss	288(%r14), %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	296(%r14), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	304(%r14), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	312(%r14), %xmm0
	vmovss	%xmm0, 396(%rsp)
	vmovss	320(%r14), %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	328(%r14), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	336(%r14), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	344(%r14), %xmm0
	vmovss	%xmm0, 388(%rsp)
	vmovss	552(%r14), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	560(%r14), %xmm0
	vmovss	%xmm0, 420(%rsp)
	vmovss	568(%r14), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	576(%r14), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	584(%r14), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	592(%r14), %xmm0
	vmovss	%xmm0, 380(%rsp)
	vmovss	600(%r14), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	608(%r14), %xmm0
	vmovss	%xmm0, 376(%rsp)
	vmovss	616(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	624(%r14), %xmm0
	vmovss	%xmm0, 372(%rsp)
	vmovss	632(%r14), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	640(%r14), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	648(%r14), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	656(%r14), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	664(%r14), %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	672(%r14), %xmm0
	vmovss	%xmm0, 364(%rsp)
	vmovss	680(%r14), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	688(%r14), %xmm0
	vmovss	%xmm0, 360(%rsp)
	vmovss	696(%r14), %xmm0
	vmovss	%xmm0, 432(%rsp)
	vmovss	704(%r14), %xmm0
	vmovss	%xmm0, 356(%rsp)
.Ltmp2355:
	.loc	1 1091 31 is_stmt 1
	leaq	520(%rsp), %rdi
	leaq	120(%r14), %rsi
	movl	456(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1092 31
	leaq	712(%rsp), %rdi
	leaq	480(%r14), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1090 28
	vmovss	520(%rsp), %xmm0
	vmovss	%xmm0, 332(%rsp)
	vmovss	524(%rsp), %xmm0
	vmovaps	%xmm0, 656(%rsp)
	vmovss	528(%rsp), %xmm0
	vmovaps	%xmm0, 640(%rsp)
	vmovss	532(%rsp), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	536(%rsp), %xmm0
	vmovaps	%xmm0, 624(%rsp)
	vmovss	540(%rsp), %xmm0
	vmovaps	%xmm0, 608(%rsp)
	vmovss	712(%rsp), %xmm0
	vmovss	%xmm0, 132(%rsp)
	vmovss	716(%rsp), %xmm0
	vmovaps	%xmm0, 592(%rsp)
	vmovss	720(%rsp), %xmm0
	vmovaps	%xmm0, 576(%rsp)
	vmovss	724(%rsp), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	728(%rsp), %xmm0
	vmovaps	%xmm0, 560(%rsp)
	vmovss	732(%rsp), %xmm0
	vmovaps	%xmm0, 480(%rsp)
	movq	%r15, 64(%rsp)
.Ltmp2356:
	.loc	1 0 0 is_stmt 0
	addq	%rbp, %r15
	setb	%cl
	movq	%r15, 504(%rsp)
	cmpq	184(%rsp), %r15
	seta	%al
.Ltmp2357:
	.loc	38 1050 16 is_stmt 1
	orb	%cl, %al
.Ltmp2358:
	.loc	1 1094 12
	testb	$1, %r12b
	je	.LBB32_109
.Ltmp2359:
	.loc	38 1050 16
	testb	%al, %al
	jne	.LBB32_191
.Ltmp2360:
	.loc	1 972 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 272(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
.Ltmp2361:
	.loc	1 973 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 336(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
.Ltmp2362:
	.loc	1 974 25
	movl	184(%r14), %eax
	movl	%eax, 144(%rsp)
	movl	188(%r14), %eax
	movl	%eax, 160(%rsp)
.Ltmp2363:
	.loc	1 975 24
	movl	544(%r14), %eax
	movl	%eax, 12(%rsp)
	movl	548(%r14), %ebx
.Ltmp2364:
	.loc	1 976 24
	movq	848(%r14), %rdi
.Ltmp2365:
	.loc	2 1916 50
	cmpq	$0, 64(%rsp)
	je	.LBB32_260
.Ltmp2366:
	.loc	2 0 50 is_stmt 0
	movq	464(%rsp), %rax
	leaq	(%rax,%rbp,4), %rsi
	movq	472(%rsp), %rax
	leaq	(%rax,%rbp,4), %rcx
	xorl	%ebp, %ebp
	movq	%rsi, 680(%rsp)
	movq	%rcx, 672(%rsp)
	.p2align	4
.LBB32_107:
.Ltmp2367:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp2368:
	.loc	1 857 8
	cmpq	%r13, %rax
	jb	.LBB32_108
.Ltmp2369:
	.loc	1 0 8 is_stmt 0
	movq	%r13, %r15
	jmp	.LBB32_193
	.p2align	4
.LBB32_108:
	xorl	%r15d, %r15d
.LBB32_193:
.Ltmp2370:
	.loc	1 1000 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp2371:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp2372:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2373:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rsi,%rbp,4), %xmm0
.Ltmp2374:
	vmovss	(%rcx,%rbp,4), %xmm3
.Ltmp2375:
	vmovss	152(%r14), %xmm2
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm4
	vmovaps	208(%rsp), %xmm9
	vsubss	%xmm9, %xmm0, %xmm1
	vmulss	%xmm7, %xmm1, %xmm5
	vmovaps	240(%rsp), %xmm8
	vmulss	%xmm2, %xmm8, %xmm6
	vaddss	%xmm5, %xmm6, %xmm5
	vaddss	%xmm5, %xmm8, %xmm6
	vmulss	%xmm7, %xmm8, %xmm8
	vmulss	%xmm4, %xmm1, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	164(%r14), %xmm6, %xmm13
	vmovss	%xmm1, 428(%rsp)
	vaddss	%xmm1, %xmm9, %xmm6
	vmovaps	256(%rsp), %xmm1
	vsubss	%xmm1, %xmm6, %xmm10
	vmulss	272(%rsp), %xmm7, %xmm6
	vmulss	%xmm4, %xmm10, %xmm4
	vaddss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm1, %xmm15
.Ltmp2376:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm12
	vmovss	520(%r14), %xmm14
	vsubss	304(%rsp), %xmm3, %xmm6
	vmulss	%xmm6, %xmm12, %xmm8
	vmovaps	224(%rsp), %xmm1
	vmulss	%xmm1, %xmm11, %xmm9
	vaddss	%xmm8, %xmm9, %xmm8
	vaddss	%xmm1, %xmm8, %xmm9
	vmulss	524(%r14), %xmm9, %xmm9
.Ltmp2377:
	.loc	1 1000 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp2378:
	.loc	49 56 9
	vmovss	%xmm15, (%rax,%rdi,4)
.Ltmp2379:
	.loc	1 1001 30
	movq	144(%r14), %rdx
.Ltmp2380:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_129
.Ltmp2381:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2382:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm0, %xmm13, %xmm0
	vsubss	%xmm15, %xmm0, %xmm0
.Ltmp2383:
	.loc	1 1001 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp2384:
	.loc	49 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp2385:
	.loc	1 1002 28
	movq	488(%r14), %rdx
.Ltmp2386:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp2387:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2388:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm1, %xmm12, %xmm0
	vmulss	%xmm6, %xmm14, %xmm6
	vaddss	%xmm6, %xmm0, %xmm13
	vaddss	304(%rsp), %xmm13, %xmm0
	vmovaps	192(%rsp), %xmm1
	vsubss	%xmm1, %xmm0, %xmm6
	vmulss	336(%rsp), %xmm12, %xmm0
	vmulss	%xmm6, %xmm14, %xmm14
	vaddss	%xmm0, %xmm14, %xmm0
	vaddss	%xmm0, %xmm1, %xmm14
.Ltmp2389:
	.loc	1 1002 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp2390:
	.loc	49 56 9
	vmovss	%xmm14, (%rax,%rdi,4)
.Ltmp2391:
	.loc	1 1003 29
	movq	504(%r14), %rdx
.Ltmp2392:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	vbroadcastss	.LCPI32_1(%rip), %xmm15
	ja	.LBB32_587
.Ltmp2393:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2394:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm9, %xmm3
	vsubss	%xmm14, %xmm3, %xmm3
.Ltmp2395:
	.loc	1 1003 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp2396:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r10
	addq	%rdi, %r10
	cmpq	%r13, %r10
	jb	.LBB32_202
.Ltmp2397:
	.loc	49 0 9 is_stmt 0
	movq	%r13, %rax
	jmp	.LBB32_204
	.p2align	4
.LBB32_202:
	xorl	%eax, %eax
.LBB32_204:
	movq	128(%r14), %r11
	subq	%rax, %r10
	cmpq	%r11, %r10
	jae	.LBB32_608
.Ltmp2398:
	.loc	1 1008 34 is_stmt 1
	movq	144(%r14), %rax
	cmpq	%rax, %r10
	jae	.LBB32_609
	.loc	1 0 34 is_stmt 0
	movq	760(%r14), %r9
	addq	%rdi, %r9
	cmpq	%r13, %r9
	jb	.LBB32_207
	movq	%r13, %rcx
	jmp	.LBB32_209
	.p2align	4
.LBB32_207:
	xorl	%ecx, %ecx
.LBB32_209:
	movq	488(%r14), %r12
	subq	%rcx, %r9
	cmpq	%r12, %r9
	jae	.LBB32_610
.Ltmp2399:
	.loc	1 1012 34 is_stmt 1
	movq	504(%r14), %r8
	cmpq	%r8, %r9
	jae	.LBB32_261
	.loc	1 0 34 is_stmt 0
	movl	%ebx, 140(%rsp)
	movq	120(%r14), %rcx
	movq	%rcx, 696(%rsp)
	vmovss	(%rcx,%r10,4), %xmm3
.Ltmp2400:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2401:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r13d
	movl	$841731191, %esi
.Ltmp2402:
	.loc	49 161 24
	jbe	.LBB32_213
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %esi
.LBB32_213:
.Ltmp2403:
	.loc	23 1291 18 is_stmt 1
	vmovd	%esi, %xmm3
.Ltmp2404:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %ebx
	movl	$8388608, %ecx
.Ltmp2405:
	.loc	49 161 24
	jbe	.LBB32_215
.Ltmp2406:
	.loc	49 0 24 is_stmt 0
	movl	%esi, %ecx
.LBB32_215:
.Ltmp2407:
	.loc	49 185 42 is_stmt 1
	movl	%ecx, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2408:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp2409:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2410:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm9
.Ltmp2411:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2412:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2413:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2414:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2415:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2416:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2417:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2418:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2419:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2420:
	.loc	49 187 28
	shrl	$23, %ecx
	orl	$1258291200, %ecx
.Ltmp2421:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm3
.Ltmp2422:
	.loc	23 1291 18
	vmovd	%ecx, %xmm9
.Ltmp2423:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2424:
	.loc	49 61 9
	vaddss	%xmm3, %xmm9, %xmm3
	vmovss	72(%rsp), %xmm9
	vaddss	688(%rsp), %xmm9, %xmm9
.Ltmp2425:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2426:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2427:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm9, 72(%rsp)
.Ltmp2428:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2429:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm3, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_216
.Ltmp2430:
	.loc	49 0 44
	xorl	%ecx, %ecx
	jmp	.LBB32_218
	.p2align	4
.LBB32_216:
	vmovss	.LCPI32_17(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm9
.Ltmp2431:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2432:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm14, %xmm14
	vblendvps	%xmm14, %xmm9, %xmm3, %xmm3
	vmulss	332(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %ecx
.Ltmp2433:
.LBB32_218:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp2434:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2435:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm3, %xmm3
.Ltmp2436:
	.loc	1 924 9 is_stmt 1
	vmovss	144(%rsp), %xmm9
.Ltmp2437:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm3, %xmm14
	vmovaps	656(%rsp), %xmm1
	vblendvps	%xmm14, 640(%rsp), %xmm1, %xmm14
.Ltmp2438:
	.loc	49 66 9
	vsubss	%xmm3, %xmm9, %xmm9
.Ltmp2439:
	.loc	49 92 9
	vmulss	%xmm14, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp2440:
	.loc	49 103 24
	vandps	%xmm3, %xmm15, %xmm9
.Ltmp2441:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm9, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_219
	.loc	49 0 24
	movl	$0, 144(%rsp)
	jmp	.LBB32_221
	.p2align	4
.LBB32_219:
	.loc	49 166 0
	vmovss	%xmm3, 144(%rsp)
.Ltmp2442:
.LBB32_221:
	.loc	49 0 0
	movq	136(%r14), %rcx
	vmovss	(%rcx,%r10,4), %xmm3
.Ltmp2443:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2444:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r10d
.Ltmp2445:
	.loc	49 161 24
	jbe	.LBB32_223
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %r10d
.LBB32_223:
.Ltmp2446:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm3
.Ltmp2447:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %esi
.Ltmp2448:
	.loc	49 161 24
	jbe	.LBB32_225
.Ltmp2449:
	.loc	49 0 24 is_stmt 0
	movl	%r10d, %esi
.LBB32_225:
.Ltmp2450:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2451:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp2452:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2453:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm9
.Ltmp2454:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2455:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2456:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2457:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2458:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2459:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2460:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2461:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2462:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2463:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp2464:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm3
.Ltmp2465:
	.loc	23 1291 18
	vmovd	%esi, %xmm9
.Ltmp2466:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2467:
	.loc	49 61 9
	vaddss	%xmm3, %xmm9, %xmm3
	vmovss	80(%rsp), %xmm9
	vaddss	424(%rsp), %xmm9, %xmm9
.Ltmp2468:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2469:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2470:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm9, 80(%rsp)
.Ltmp2471:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2472:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm3, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_226
.Ltmp2473:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_228
	.p2align	4
.LBB32_226:
	vmovss	.LCPI32_17(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm9
.Ltmp2474:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2475:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm14, %xmm14
	vblendvps	%xmm14, %xmm9, %xmm3, %xmm3
	vmulss	136(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %edx
.Ltmp2476:
.LBB32_228:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp2477:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2478:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm3, %xmm3
.Ltmp2479:
	.loc	1 924 9 is_stmt 1
	vmovss	160(%rsp), %xmm9
.Ltmp2480:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm3, %xmm14
	vmovaps	624(%rsp), %xmm1
	vblendvps	%xmm14, 608(%rsp), %xmm1, %xmm14
.Ltmp2481:
	.loc	49 66 9
	vsubss	%xmm3, %xmm9, %xmm9
.Ltmp2482:
	.loc	49 92 9
	vmulss	%xmm14, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp2483:
	.loc	49 103 24
	vandps	%xmm3, %xmm15, %xmm9
.Ltmp2484:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm9, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_229
	.loc	49 0 24
	movl	$0, 160(%rsp)
	jmp	.LBB32_231
	.p2align	4
.LBB32_229:
	.loc	49 166 0
	vmovss	%xmm3, 160(%rsp)
.Ltmp2485:
.LBB32_231:
	.loc	49 0 0
	movq	480(%r14), %r10
	vmovss	(%r10,%r9,4), %xmm3
.Ltmp2486:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2487:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %edx
.Ltmp2488:
	.loc	49 161 24
	jbe	.LBB32_233
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %edx
.LBB32_233:
.Ltmp2489:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp2490:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %esi
.Ltmp2491:
	.loc	49 161 24
	jbe	.LBB32_235
.Ltmp2492:
	.loc	49 0 24 is_stmt 0
	movl	%edx, %esi
.LBB32_235:
.Ltmp2493:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2494:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp2495:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2496:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm9
.Ltmp2497:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2498:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2499:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2500:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2501:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2502:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2503:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2504:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2505:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2506:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp2507:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm3
.Ltmp2508:
	.loc	23 1291 18
	vmovd	%esi, %xmm9
.Ltmp2509:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2510:
	.loc	49 61 9
	vaddss	%xmm3, %xmm9, %xmm3
	vmovss	20(%rsp), %xmm9
	vaddss	420(%rsp), %xmm9, %xmm9
.Ltmp2511:
	.loc	1 0 0 is_stmt 0
	movq	496(%r14), %rsi
.Ltmp2512:
	.loc	49 71 9 is_stmt 1
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2513:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2514:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm9, 20(%rsp)
.Ltmp2515:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2516:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm3, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_236
.Ltmp2517:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_238
	.p2align	4
.LBB32_236:
	vmovss	.LCPI32_17(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm9
.Ltmp2518:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2519:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm14, %xmm14
	vblendvps	%xmm14, %xmm9, %xmm3, %xmm3
	vmulss	132(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %edx
.Ltmp2520:
.LBB32_238:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp2521:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2522:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm3, %xmm3
.Ltmp2523:
	.loc	1 924 9 is_stmt 1
	vmovss	12(%rsp), %xmm9
.Ltmp2524:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm3, %xmm14
	vmovaps	592(%rsp), %xmm1
	vblendvps	%xmm14, 576(%rsp), %xmm1, %xmm14
.Ltmp2525:
	.loc	49 66 9
	vsubss	%xmm3, %xmm9, %xmm9
.Ltmp2526:
	.loc	49 92 9
	vmulss	%xmm14, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp2527:
	.loc	49 103 24
	vandps	%xmm3, %xmm15, %xmm9
.Ltmp2528:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm9, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_239
.Ltmp2529:
	.loc	49 0 24
	movl	$0, 12(%rsp)
.Ltmp2530:
	.loc	49 103 24 is_stmt 1
	vmovss	(%rsi,%r9,4), %xmm3
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2531:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
.Ltmp2532:
	.loc	49 161 24
	ja	.LBB32_242
	jmp	.LBB32_243
.Ltmp2533:
	.loc	49 0 24 is_stmt 0
.Ltmp2534:
	.p2align	4
.LBB32_239:
	.loc	49 166 0 is_stmt 1
	vmovss	%xmm3, 12(%rsp)
.Ltmp2535:
	.loc	49 103 24
	vmovss	(%rsi,%r9,4), %xmm3
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2536:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
.Ltmp2537:
	.loc	49 161 24
	jbe	.LBB32_243
.LBB32_242:
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %r13d
.LBB32_243:
.Ltmp2538:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r13d, %xmm3
.Ltmp2539:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
.Ltmp2540:
	.loc	49 161 24
	jbe	.LBB32_245
.Ltmp2541:
	.loc	49 0 24 is_stmt 0
	movl	%r13d, %ebx
.LBB32_245:
.Ltmp2542:
	.loc	49 185 42 is_stmt 1
	movl	%ebx, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2543:
	.loc	23 1291 18
	vmovd	%edx, %xmm3
.Ltmp2544:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2545:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm9
.Ltmp2546:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2547:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2548:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2549:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2550:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2551:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2552:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2553:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2554:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2555:
	.loc	49 187 28
	shrl	$23, %ebx
	orl	$1258291200, %ebx
.Ltmp2556:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm3
.Ltmp2557:
	.loc	23 1291 18
	vmovd	%ebx, %xmm9
.Ltmp2558:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2559:
	.loc	49 61 9
	vaddss	%xmm3, %xmm9, %xmm3
	vmovss	56(%rsp), %xmm9
	vaddss	416(%rsp), %xmm9, %xmm9
.Ltmp2560:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2561:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2562:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm9, 56(%rsp)
.Ltmp2563:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2564:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm3, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_246
.Ltmp2565:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_248
	.p2align	4
.LBB32_246:
	vmovss	.LCPI32_17(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm9
.Ltmp2566:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2567:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm14, %xmm14
	vblendvps	%xmm14, %xmm9, %xmm3, %xmm3
	vmulss	128(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %edx
.Ltmp2568:
.LBB32_248:
	.loc	49 0 0
	movq	320(%rsp), %r13
	movl	140(%rsp), %r9d
.Ltmp2569:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm3
.Ltmp2570:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2571:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm3, %xmm3
.Ltmp2572:
	.loc	1 924 9 is_stmt 1
	vmovd	%r9d, %xmm9
.Ltmp2573:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm3, %xmm14
	vmovaps	560(%rsp), %xmm1
	vblendvps	%xmm14, 480(%rsp), %xmm1, %xmm14
.Ltmp2574:
	.loc	49 66 9
	vsubss	%xmm3, %xmm9, %xmm9
.Ltmp2575:
	.loc	49 92 9
	vmulss	%xmm14, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp2576:
	.loc	49 103 24
	vandps	%xmm3, %xmm15, %xmm9
.Ltmp2577:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm9, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_249
	.loc	49 0 24
	xorl	%ebx, %ebx
	jmp	.LBB32_251
	.p2align	4
.LBB32_249:
	.loc	49 166 0
	vmovd	%xmm3, %ebx
.Ltmp2578:
.LBB32_251:
	.loc	49 0 0
	negq	%r15
	addq	%r15, %rdi
	incq	%rdi
.Ltmp2579:
	.loc	48 568 12 is_stmt 1
	cmpq	%r11, %rdi
	ja	.LBB32_588
.Ltmp2580:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
.Ltmp2581:
	.loc	48 568 12 is_stmt 1
	cmpq	%rax, %rdi
	ja	.LBB32_190
.Ltmp2582:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
.Ltmp2583:
	.loc	48 568 12 is_stmt 1
	cmpq	%r12, %rdi
	ja	.LBB32_589
.Ltmp2584:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
.Ltmp2585:
	.loc	48 568 12 is_stmt 1
	cmpq	%r8, %rdi
	ja	.LBB32_590
.Ltmp2586:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
	vmulss	%xmm7, %xmm10, %xmm3
	vmovaps	272(%rsp), %xmm7
	vmulss	%xmm2, %xmm7, %xmm2
	vaddss	%xmm3, %xmm2, %xmm2
	vmulss	%xmm6, %xmm12, %xmm3
	vmovaps	336(%rsp), %xmm12
	vmulss	%xmm11, %xmm12, %xmm6
	vaddss	%xmm3, %xmm6, %xmm6
	vaddss	%xmm5, %xmm5, %xmm3
	vaddss	240(%rsp), %xmm3, %xmm5
	vmovss	428(%rsp), %xmm1
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	208(%rsp), %xmm1, %xmm3
	vaddss	%xmm2, %xmm2, %xmm1
	vaddss	%xmm4, %xmm4, %xmm2
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm13, %xmm13, %xmm8
	vaddss	%xmm6, %xmm6, %xmm9
	vaddss	%xmm0, %xmm0, %xmm10
	vaddss	%xmm1, %xmm7, %xmm7
	vaddss	256(%rsp), %xmm2, %xmm6
	vaddss	224(%rsp), %xmm4, %xmm4
	vaddss	304(%rsp), %xmm8, %xmm0
	vaddss	%xmm9, %xmm12, %xmm1
	vaddss	192(%rsp), %xmm10, %xmm2
	vmovss	16(%rsp), %xmm9
	vaddss	404(%rsp), %xmm9, %xmm9
	vmovss	144(%rsp), %xmm8
	vmovss	%xmm9, 16(%rsp)
	vaddss	%xmm8, %xmm9, %xmm8
	vmovss	28(%rsp), %xmm10
	vaddss	388(%rsp), %xmm10, %xmm10
	vmovss	160(%rsp), %xmm9
	vmovss	%xmm10, 28(%rsp)
	vaddss	%xmm9, %xmm10, %xmm9
	vmovss	32(%rsp), %xmm11
	vaddss	372(%rsp), %xmm11, %xmm11
	vmovss	12(%rsp), %xmm10
	vmovss	%xmm11, 32(%rsp)
	vaddss	%xmm10, %xmm11, %xmm10
	vandps	%xmm5, %xmm15, %xmm11
	vandps	%xmm3, %xmm15, %xmm12
	vandps	%xmm7, %xmm15, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm14
	vcmpnltss	%xmm14, %xmm11, %xmm11
	vandps	%xmm5, %xmm11, %xmm5
	vmovaps	%xmm5, 240(%rsp)
	vandps	%xmm6, %xmm15, %xmm5
	vcmpnltss	%xmm14, %xmm12, %xmm11
	vandps	%xmm3, %xmm11, %xmm3
	vmovaps	%xmm3, 208(%rsp)
	vandps	%xmm4, %xmm15, %xmm3
	vcmpnltss	%xmm14, %xmm13, %xmm11
	vandps	%xmm7, %xmm11, %xmm7
	vmovaps	%xmm7, 272(%rsp)
	vandps	%xmm0, %xmm15, %xmm7
	vcmpnltss	%xmm14, %xmm5, %xmm5
	vandps	%xmm6, %xmm5, %xmm5
	vmovaps	%xmm5, 256(%rsp)
	vandps	%xmm1, %xmm15, %xmm5
	vcmpnltss	%xmm14, %xmm3, %xmm3
	vandps	%xmm4, %xmm3, %xmm3
	vmovaps	%xmm3, 224(%rsp)
	vandps	%xmm2, %xmm15, %xmm3
	vmovss	.LCPI32_21(%rip), %xmm11
	vmulss	%xmm11, %xmm8, %xmm4
	vmulss	%xmm11, %xmm9, %xmm6
	vmulss	%xmm11, %xmm10, %xmm8
	vcmpnltss	%xmm14, %xmm7, %xmm7
	vcmpnltss	%xmm14, %xmm5, %xmm5
	vcmpnltss	%xmm14, %xmm3, %xmm3
	vmovss	.LCPI32_22(%rip), %xmm9
	vmaxss	%xmm9, %xmm4, %xmm4
	vmaxss	%xmm9, %xmm6, %xmm6
	vmaxss	%xmm9, %xmm8, %xmm8
	vandps	%xmm0, %xmm7, %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vandps	%xmm1, %xmm5, %xmm0
	vmovaps	%xmm0, 336(%rsp)
	vandps	%xmm2, %xmm3, %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm11
	vminss	%xmm11, %xmm4, %xmm3
	vminss	%xmm11, %xmm6, %xmm2
	vminss	%xmm11, %xmm8, %xmm4
	vroundss	$9, %xmm3, %xmm3, %xmm1
	vroundss	$9, %xmm2, %xmm2, %xmm9
	vsubss	%xmm1, %xmm3, %xmm3
	vmovss	.LCPI32_24(%rip), %xmm8
	vmulss	%xmm3, %xmm8, %xmm5
	vmovss	.LCPI32_25(%rip), %xmm10
	vaddss	%xmm5, %xmm10, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_26(%rip), %xmm12
	vaddss	%xmm5, %xmm12, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_27(%rip), %xmm13
	vaddss	%xmm5, %xmm13, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vmovss	.LCPI32_28(%rip), %xmm14
	vaddss	%xmm5, %xmm14, %xmm5
	vmulss	%xmm5, %xmm3, %xmm5
	vroundss	$9, %xmm4, %xmm4, %xmm3
	vsubss	%xmm9, %xmm2, %xmm6
	vsubss	%xmm3, %xmm4, %xmm2
	vmovss	432(%rsp), %xmm0
	vaddss	356(%rsp), %xmm0, %xmm0
	vmovss	.LCPI32_29(%rip), %xmm15
	vaddss	%xmm5, %xmm15, %xmm4
	vmovss	.LCPI32_30(%rip), %xmm7
	vaddss	%xmm7, %xmm1, %xmm1
	vmovd	%xmm1, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vmulss	%xmm6, %xmm8, %xmm4
	vaddss	%xmm4, %xmm10, %xmm4
	vmulss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm12, %xmm4
	vmulss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
	vmulss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm14, %xmm4
.Ltmp2587:
	vmulss	%xmm4, %xmm6, %xmm4
.Ltmp2588:
	vmovd	%ebx, %xmm5
	vmovss	%xmm0, 432(%rsp)
	vaddss	%xmm5, %xmm0, %xmm5
	vmulss	.LCPI32_21(%rip), %xmm5, %xmm5
	vmaxss	.LCPI32_22(%rip), %xmm5, %xmm5
	vminss	%xmm11, %xmm5, %xmm5
.Ltmp2589:
	vaddss	%xmm4, %xmm15, %xmm4
	vaddss	%xmm7, %xmm9, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm4, %xmm0
.Ltmp2590:
	vmulss	%xmm2, %xmm8, %xmm4
	vaddss	%xmm4, %xmm10, %xmm4
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm4, %xmm12, %xmm4
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm4, %xmm14, %xmm4
.Ltmp2591:
	vmulss	%xmm4, %xmm2, %xmm2
.Ltmp2592:
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
.Ltmp2593:
	vaddss	%xmm2, %xmm15, %xmm2
	vaddss	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm3
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2594:
	vmulss	%xmm5, %xmm8, %xmm3
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm12, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2595:
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm15, %xmm3
	vaddss	%xmm7, %xmm4, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	696(%rsp), %rax
.Ltmp2596:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
	vmulss	(%rcx,%rdi,4), %xmm0, %xmm0
.Ltmp2597:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp2598:
	vaddss	%xmm0, %xmm1, %xmm0
.Ltmp2599:
	vmulss	(%r10,%rdi,4), %xmm2, %xmm1
.Ltmp2600:
	.loc	49 71 9 is_stmt 1
	vmulss	(%rsi,%rdi,4), %xmm3, %xmm2
.Ltmp2601:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	680(%rsp), %rsi
.Ltmp2602:
	.loc	49 56 9
	vmovss	%xmm0, (%rsi,%rbp,4)
	movq	672(%rsp), %rcx
.Ltmp2603:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%rcx,%rbp,4)
	vmovss	544(%rsp), %xmm0
	vaddss	300(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 544(%rsp)
	vmovss	88(%rsp), %xmm0
	vaddss	384(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	36(%rsp), %xmm0
	vaddss	412(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	96(%rsp), %xmm0
	vaddss	380(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	40(%rsp), %xmm0
	vaddss	408(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	104(%rsp), %xmm0
	vaddss	376(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	44(%rsp), %xmm0
	vaddss	400(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	112(%rsp), %xmm0
	vaddss	368(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	48(%rsp), %xmm0
	vaddss	396(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	120(%rsp), %xmm0
	vaddss	364(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	52(%rsp), %xmm0
	vaddss	392(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 52(%rsp)
	vmovss	24(%rsp), %xmm0
	vaddss	360(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 24(%rsp)
	incq	%rbp
.Ltmp2604:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rbp, 64(%rsp)
.Ltmp2605:
	.loc	3 900 12
	jne	.LBB32_107
	jmp	.LBB32_260
.Ltmp2606:
	.loc	3 0 12 is_stmt 0
.Ltmp2607:
	.p2align	4
.LBB32_109:
	.loc	38 1050 16 is_stmt 1
	testb	%al, %al
	jne	.LBB32_114
.Ltmp2608:
	.loc	1 972 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 272(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
.Ltmp2609:
	.loc	1 973 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 336(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
.Ltmp2610:
	.loc	1 974 25
	movl	184(%r14), %eax
	movl	%eax, 144(%rsp)
	movl	188(%r14), %eax
	movl	%eax, 160(%rsp)
.Ltmp2611:
	.loc	1 975 24
	movl	544(%r14), %eax
	movl	%eax, 12(%rsp)
	movl	548(%r14), %esi
.Ltmp2612:
	.loc	1 976 24
	movq	848(%r14), %rdi
.Ltmp2613:
	.loc	2 1916 50
	cmpq	$0, 64(%rsp)
.Ltmp2614:
	.loc	3 900 12
	je	.LBB32_187
.Ltmp2615:
	.loc	3 0 12 is_stmt 0
	movq	464(%rsp), %rax
	leaq	(%rax,%rbp,4), %r8
	movq	472(%rsp), %rax
	leaq	(%rax,%rbp,4), %rcx
	xorl	%ebp, %ebp
	movq	%r8, 96(%rsp)
	movq	%rcx, 88(%rsp)
	.p2align	4
.LBB32_112:
.Ltmp2616:
	.loc	1 987 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp2617:
	.loc	1 857 8
	cmpq	%r13, %rax
	jb	.LBB32_113
.Ltmp2618:
	.loc	1 0 8 is_stmt 0
	movq	%r13, %rbx
	jmp	.LBB32_117
	.p2align	4
.LBB32_113:
	xorl	%ebx, %ebx
.LBB32_117:
.Ltmp2619:
	.loc	1 1000 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp2620:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp2621:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2622:
	.loc	1 0 0 is_stmt 0
	vmovss	(%r8,%rbp,4), %xmm3
.Ltmp2623:
	vmovss	(%rcx,%rbp,4), %xmm13
.Ltmp2624:
	vmovss	152(%r14), %xmm2
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm0
	vmovaps	208(%rsp), %xmm8
	vsubss	%xmm8, %xmm3, %xmm1
	vmulss	%xmm7, %xmm1, %xmm4
	vmovaps	240(%rsp), %xmm6
	vmulss	%xmm2, %xmm6, %xmm5
	vaddss	%xmm4, %xmm5, %xmm5
	vaddss	%xmm5, %xmm6, %xmm4
	vmulss	%xmm7, %xmm6, %xmm6
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	%xmm1, %xmm6, %xmm1
	vmulss	164(%r14), %xmm4, %xmm6
	vmovss	%xmm1, 24(%rsp)
	vaddss	%xmm1, %xmm8, %xmm4
	vmovaps	256(%rsp), %xmm1
	vsubss	%xmm1, %xmm4, %xmm10
	vmulss	272(%rsp), %xmm7, %xmm4
	vmulss	%xmm0, %xmm10, %xmm0
	vaddss	%xmm0, %xmm4, %xmm4
	vaddss	%xmm4, %xmm1, %xmm9
.Ltmp2625:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm0
	vmovss	520(%r14), %xmm12
	vsubss	304(%rsp), %xmm13, %xmm14
	vmulss	%xmm0, %xmm14, %xmm8
	vmovaps	224(%rsp), %xmm1
	vmulss	%xmm1, %xmm11, %xmm15
	vaddss	%xmm8, %xmm15, %xmm8
	vaddss	%xmm1, %xmm8, %xmm15
	vmulss	524(%r14), %xmm15, %xmm15
.Ltmp2626:
	.loc	1 1000 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp2627:
	.loc	49 56 9
	vmovss	%xmm9, (%rax,%rdi,4)
.Ltmp2628:
	.loc	1 1001 30
	movq	144(%r14), %rdx
.Ltmp2629:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_129
.Ltmp2630:
	.loc	49 56 9
	je	.LBB32_119
.Ltmp2631:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm6, %xmm3, %xmm3
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2632:
	.loc	1 1001 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp2633:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp2634:
	.loc	1 1002 28
	movq	488(%r14), %rdx
.Ltmp2635:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp2636:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_119
	vmulss	%xmm0, %xmm1, %xmm3
	vmulss	%xmm12, %xmm14, %xmm6
	vaddss	%xmm6, %xmm3, %xmm3
	vaddss	304(%rsp), %xmm3, %xmm6
	vmovaps	192(%rsp), %xmm1
	vsubss	%xmm1, %xmm6, %xmm14
	vmulss	336(%rsp), %xmm0, %xmm6
	vmulss	%xmm14, %xmm12, %xmm9
	vaddss	%xmm6, %xmm9, %xmm12
	vaddss	%xmm1, %xmm12, %xmm6
.Ltmp2637:
	.loc	1 1002 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp2638:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp2639:
	.loc	1 1003 29
	movq	504(%r14), %rdx
.Ltmp2640:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_587
.Ltmp2641:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_119
	vaddss	%xmm15, %xmm13, %xmm9
	vsubss	%xmm6, %xmm9, %xmm6
.Ltmp2642:
	.loc	1 1003 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp2643:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
	movq	400(%r14), %r10
	addq	%rdi, %r10
	cmpq	%r13, %r10
	jb	.LBB32_127
.Ltmp2644:
	.loc	49 0 9 is_stmt 0
	movq	%r13, %rax
	jmp	.LBB32_131
	.p2align	4
.LBB32_127:
	xorl	%eax, %eax
.LBB32_131:
	vbroadcastss	.LCPI32_1(%rip), %xmm15
	movq	128(%r14), %r11
	subq	%rax, %r10
	cmpq	%r11, %r10
	jae	.LBB32_608
.Ltmp2645:
	.loc	1 1008 34 is_stmt 1
	movq	144(%r14), %rax
	cmpq	%rax, %r10
	jae	.LBB32_609
	.loc	1 0 34 is_stmt 0
	movq	760(%r14), %r9
	addq	%rdi, %r9
	cmpq	%r13, %r9
	jb	.LBB32_134
	movq	%r13, %rcx
	jmp	.LBB32_136
	.p2align	4
.LBB32_134:
	xorl	%ecx, %ecx
.LBB32_136:
	movq	488(%r14), %r12
	subq	%rcx, %r9
	cmpq	%r12, %r9
	jae	.LBB32_610
.Ltmp2646:
	.loc	1 1012 34 is_stmt 1
	movq	504(%r14), %r8
	cmpq	%r8, %r9
	jae	.LBB32_261
	.loc	1 0 34 is_stmt 0
	movq	120(%r14), %rcx
	movq	%rcx, 120(%rsp)
	vmovss	(%rcx,%r10,4), %xmm6
.Ltmp2647:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2648:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
	movl	$841731191, %r13d
	movl	$841731191, %edx
.Ltmp2649:
	.loc	49 161 24
	jbe	.LBB32_140
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %edx
.LBB32_140:
.Ltmp2650:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm6
.Ltmp2651:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
	movl	$8388608, %r15d
	movl	$8388608, %ecx
.Ltmp2652:
	.loc	49 161 24
	jbe	.LBB32_142
.Ltmp2653:
	.loc	49 0 24 is_stmt 0
	movl	%edx, %ecx
.LBB32_142:
.Ltmp2654:
	.loc	49 185 42 is_stmt 1
	movl	%ecx, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2655:
	.loc	23 1291 18
	vmovd	%edx, %xmm6
.Ltmp2656:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2657:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2658:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2659:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2660:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2661:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2662:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2663:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2664:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2665:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2666:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2667:
	.loc	49 187 28
	shrl	$23, %ecx
	orl	$1258291200, %ecx
.Ltmp2668:
	.loc	23 1291 18
	vmovd	%ecx, %xmm13
.Ltmp2669:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm13, %xmm13
.Ltmp2670:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2671:
	.loc	49 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2672:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2673:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2674:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm6
.Ltmp2675:
	.loc	49 66 9 is_stmt 1
	vsubss	72(%rsp), %xmm6, %xmm6
.Ltmp2676:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm6, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_143
.Ltmp2677:
	.loc	49 0 44
	xorl	%ecx, %ecx
	jmp	.LBB32_145
	.p2align	4
.LBB32_143:
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm6, %xmm13, %xmm9
.Ltmp2678:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2679:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	332(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %ecx
.Ltmp2680:
.LBB32_145:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ecx, %xmm6
.Ltmp2681:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp2682:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm6, %xmm6
.Ltmp2683:
	.loc	1 924 9 is_stmt 1
	vmovss	144(%rsp), %xmm9
.Ltmp2684:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	656(%rsp), %xmm1
	vblendvps	%xmm13, 640(%rsp), %xmm1, %xmm13
.Ltmp2685:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp2686:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2687:
	.loc	49 103 24
	vandps	%xmm6, %xmm15, %xmm9
.Ltmp2688:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	movl	%esi, %ecx
	vucomiss	%xmm9, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_146
	.loc	49 0 24
	movl	$0, 144(%rsp)
	jmp	.LBB32_148
	.p2align	4
.LBB32_146:
	.loc	49 166 0
	vmovss	%xmm6, 144(%rsp)
.Ltmp2689:
.LBB32_148:
	.loc	49 0 0
	movq	136(%r14), %rdx
	movq	%rdx, 112(%rsp)
	vmovss	(%rdx,%r10,4), %xmm6
.Ltmp2690:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2691:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
	movl	$841731191, %esi
.Ltmp2692:
	.loc	49 161 24
	jbe	.LBB32_150
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %esi
.LBB32_150:
.Ltmp2693:
	.loc	23 1291 18 is_stmt 1
	vmovd	%esi, %xmm6
.Ltmp2694:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
	movl	$8388608, %edx
.Ltmp2695:
	.loc	49 161 24
	jbe	.LBB32_152
.Ltmp2696:
	.loc	49 0 24 is_stmt 0
	movl	%esi, %edx
.LBB32_152:
.Ltmp2697:
	.loc	49 185 42 is_stmt 1
	movl	%edx, %esi
	andl	$8388607, %esi
	orl	$1065353216, %esi
.Ltmp2698:
	.loc	23 1291 18
	vmovd	%esi, %xmm6
.Ltmp2699:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2700:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2701:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2702:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2703:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2704:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2705:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2706:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2707:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2708:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2709:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2710:
	.loc	49 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp2711:
	.loc	23 1291 18
	vmovd	%edx, %xmm13
.Ltmp2712:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm13, %xmm13
.Ltmp2713:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2714:
	.loc	49 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2715:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2716:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2717:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm6
.Ltmp2718:
	.loc	49 66 9 is_stmt 1
	vsubss	80(%rsp), %xmm6, %xmm6
.Ltmp2719:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm6, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_153
.Ltmp2720:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_155
	.p2align	4
.LBB32_153:
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm6, %xmm13, %xmm9
.Ltmp2721:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2722:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	136(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %edx
.Ltmp2723:
.LBB32_155:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm6
.Ltmp2724:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp2725:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm6, %xmm6
.Ltmp2726:
	.loc	1 924 9 is_stmt 1
	vmovss	160(%rsp), %xmm9
.Ltmp2727:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	624(%rsp), %xmm1
	vblendvps	%xmm13, 608(%rsp), %xmm1, %xmm13
.Ltmp2728:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp2729:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2730:
	.loc	49 103 24
	vandps	%xmm6, %xmm15, %xmm9
.Ltmp2731:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm9, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_156
	.loc	49 0 24
	movl	$0, 160(%rsp)
	jmp	.LBB32_158
	.p2align	4
.LBB32_156:
	.loc	49 166 0
	vmovss	%xmm6, 160(%rsp)
.Ltmp2732:
.LBB32_158:
	.loc	49 0 0
	movq	480(%r14), %rdx
	movq	%rdx, 104(%rsp)
	vmovss	(%rdx,%r9,4), %xmm6
.Ltmp2733:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2734:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
	movl	$841731191, %edx
.Ltmp2735:
	.loc	49 161 24
	jbe	.LBB32_160
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %edx
.LBB32_160:
.Ltmp2736:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm6
.Ltmp2737:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
	movl	$8388608, %esi
.Ltmp2738:
	.loc	49 161 24
	jbe	.LBB32_162
.Ltmp2739:
	.loc	49 0 24 is_stmt 0
	movl	%edx, %esi
.LBB32_162:
	movq	496(%r14), %r10
.Ltmp2740:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2741:
	.loc	23 1291 18
	vmovd	%edx, %xmm6
.Ltmp2742:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2743:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2744:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2745:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2746:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2747:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2748:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2749:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2750:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2751:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2752:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2753:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp2754:
	.loc	23 1291 18
	vmovd	%esi, %xmm13
.Ltmp2755:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm13, %xmm13
.Ltmp2756:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2757:
	.loc	49 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2758:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2759:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2760:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm6
.Ltmp2761:
	.loc	49 66 9 is_stmt 1
	vsubss	20(%rsp), %xmm6, %xmm6
.Ltmp2762:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm6, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_163
.Ltmp2763:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_165
	.p2align	4
.LBB32_163:
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm6, %xmm13, %xmm9
.Ltmp2764:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2765:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	132(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %edx
.Ltmp2766:
.LBB32_165:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm6
.Ltmp2767:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp2768:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm6, %xmm6
.Ltmp2769:
	.loc	1 924 9 is_stmt 1
	vmovss	12(%rsp), %xmm9
.Ltmp2770:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	592(%rsp), %xmm1
	vblendvps	%xmm13, 576(%rsp), %xmm1, %xmm13
.Ltmp2771:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp2772:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2773:
	.loc	49 103 24
	vandps	%xmm6, %xmm15, %xmm9
.Ltmp2774:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm9, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_166
.Ltmp2775:
	.loc	49 0 24
	movl	$0, 12(%rsp)
.Ltmp2776:
	.loc	49 103 24 is_stmt 1
	vmovss	(%r10,%r9,4), %xmm6
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2777:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
.Ltmp2778:
	.loc	49 161 24
	ja	.LBB32_169
	jmp	.LBB32_170
.Ltmp2779:
	.loc	49 0 24 is_stmt 0
.Ltmp2780:
	.p2align	4
.LBB32_166:
	.loc	49 166 0 is_stmt 1
	vmovss	%xmm6, 12(%rsp)
.Ltmp2781:
	.loc	49 103 24
	vmovss	(%r10,%r9,4), %xmm6
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2782:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
.Ltmp2783:
	.loc	49 161 24
	jbe	.LBB32_170
.LBB32_169:
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %r13d
.LBB32_170:
.Ltmp2784:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r13d, %xmm6
.Ltmp2785:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
.Ltmp2786:
	.loc	49 161 24
	jbe	.LBB32_172
.Ltmp2787:
	.loc	49 0 24 is_stmt 0
	movl	%r13d, %r15d
.LBB32_172:
.Ltmp2788:
	.loc	49 185 42 is_stmt 1
	movl	%r15d, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp2789:
	.loc	23 1291 18
	vmovd	%edx, %xmm6
.Ltmp2790:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2791:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2792:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2793:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2794:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2795:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2796:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2797:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2798:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2799:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2800:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2801:
	.loc	49 187 28
	shrl	$23, %r15d
	orl	$1258291200, %r15d
.Ltmp2802:
	.loc	23 1291 18
	vmovd	%r15d, %xmm13
.Ltmp2803:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm13, %xmm13
.Ltmp2804:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2805:
	.loc	49 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2806:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2807:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2808:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm6
.Ltmp2809:
	.loc	49 66 9 is_stmt 1
	vsubss	56(%rsp), %xmm6, %xmm6
.Ltmp2810:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm6, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_173
.Ltmp2811:
	.loc	49 0 44
	xorl	%edx, %edx
	jmp	.LBB32_175
	.p2align	4
.LBB32_173:
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm6, %xmm13, %xmm9
.Ltmp2812:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2813:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	128(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %edx
.Ltmp2814:
.LBB32_175:
	.loc	49 0 0
	movq	320(%rsp), %r13
.Ltmp2815:
	.loc	23 1291 18 is_stmt 1
	vmovd	%edx, %xmm6
.Ltmp2816:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp2817:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm6, %xmm6
.Ltmp2818:
	.loc	1 924 9 is_stmt 1
	vmovd	%ecx, %xmm9
.Ltmp2819:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	560(%rsp), %xmm1
	vblendvps	%xmm13, 480(%rsp), %xmm1, %xmm13
.Ltmp2820:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp2821:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2822:
	.loc	49 103 24
	vandps	%xmm6, %xmm15, %xmm9
.Ltmp2823:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm9, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_176
	.loc	49 0 24
	xorl	%esi, %esi
	jmp	.LBB32_178
	.p2align	4
.LBB32_176:
	.loc	49 166 0
	vmovd	%xmm6, %esi
.Ltmp2824:
.LBB32_178:
	.loc	49 0 0
	negq	%rbx
	addq	%rbx, %rdi
	incq	%rdi
.Ltmp2825:
	.loc	48 568 12 is_stmt 1
	cmpq	%r11, %rdi
	ja	.LBB32_588
.Ltmp2826:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2827:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_190
.Ltmp2828:
	.loc	49 51 9
	je	.LBB32_180
.Ltmp2829:
	.loc	48 568 12
	cmpq	%r12, %rdi
	ja	.LBB32_589
.Ltmp2830:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
.Ltmp2831:
	.loc	48 568 12 is_stmt 1
	cmpq	%r8, %rdi
	ja	.LBB32_590
.Ltmp2832:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_180
	vmulss	%xmm7, %xmm10, %xmm6
	vmovaps	272(%rsp), %xmm13
	vmulss	%xmm2, %xmm13, %xmm2
	vaddss	%xmm6, %xmm2, %xmm2
	vmulss	%xmm0, %xmm14, %xmm0
	vmovaps	336(%rsp), %xmm14
	vmulss	%xmm11, %xmm14, %xmm6
	vaddss	%xmm0, %xmm6, %xmm0
	vaddss	%xmm5, %xmm5, %xmm5
	vaddss	240(%rsp), %xmm5, %xmm6
	vmovss	24(%rsp), %xmm1
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	208(%rsp), %xmm1, %xmm5
	vaddss	%xmm2, %xmm2, %xmm1
	vaddss	%xmm4, %xmm4, %xmm2
	vaddss	%xmm8, %xmm8, %xmm7
	vaddss	%xmm3, %xmm3, %xmm9
	vaddss	%xmm0, %xmm0, %xmm10
	vaddss	%xmm12, %xmm12, %xmm11
	vaddss	%xmm1, %xmm13, %xmm8
	vaddss	256(%rsp), %xmm2, %xmm4
	vaddss	224(%rsp), %xmm7, %xmm3
	vaddss	304(%rsp), %xmm9, %xmm0
	vaddss	%xmm10, %xmm14, %xmm1
	vaddss	192(%rsp), %xmm11, %xmm2
	vmovss	144(%rsp), %xmm7
	vaddss	16(%rsp), %xmm7, %xmm7
	vmovss	160(%rsp), %xmm9
	vaddss	28(%rsp), %xmm9, %xmm10
	vmovss	12(%rsp), %xmm9
	vaddss	32(%rsp), %xmm9, %xmm11
	vandps	%xmm6, %xmm15, %xmm9
	vandps	%xmm5, %xmm15, %xmm12
	vandps	%xmm15, %xmm8, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm14
	vcmpnltss	%xmm14, %xmm9, %xmm9
	vandps	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm6, 240(%rsp)
	vandps	%xmm4, %xmm15, %xmm6
	vcmpnltss	%xmm14, %xmm12, %xmm9
	vandps	%xmm5, %xmm9, %xmm5
	vmovaps	%xmm5, 208(%rsp)
	vandps	%xmm3, %xmm15, %xmm5
	vcmpnltss	%xmm14, %xmm13, %xmm9
	vandps	%xmm8, %xmm9, %xmm8
	vmovaps	%xmm8, 272(%rsp)
	vandps	%xmm0, %xmm15, %xmm8
	vcmpnltss	%xmm14, %xmm6, %xmm6
	vandps	%xmm4, %xmm6, %xmm4
	vmovaps	%xmm4, 256(%rsp)
	vandps	%xmm1, %xmm15, %xmm4
	vcmpnltss	%xmm14, %xmm5, %xmm5
	vandps	%xmm3, %xmm5, %xmm3
	vmovaps	%xmm3, 224(%rsp)
	vandps	%xmm2, %xmm15, %xmm3
	vmovss	.LCPI32_21(%rip), %xmm9
	vmulss	%xmm7, %xmm9, %xmm5
	vmulss	%xmm9, %xmm10, %xmm6
	vmulss	%xmm9, %xmm11, %xmm7
	vcmpnltss	%xmm14, %xmm8, %xmm8
	vcmpnltss	%xmm14, %xmm4, %xmm4
	vcmpnltss	%xmm14, %xmm3, %xmm3
	vmovss	.LCPI32_22(%rip), %xmm10
	vmaxss	%xmm10, %xmm5, %xmm5
	vmaxss	%xmm10, %xmm6, %xmm6
	vmaxss	%xmm10, %xmm7, %xmm7
	vandps	%xmm0, %xmm8, %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vandps	%xmm1, %xmm4, %xmm0
	vmovaps	%xmm0, 336(%rsp)
	vandps	%xmm2, %xmm3, %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm9
	vminss	%xmm9, %xmm5, %xmm4
	vminss	%xmm9, %xmm6, %xmm2
	vminss	%xmm9, %xmm7, %xmm3
	vroundss	$9, %xmm4, %xmm4, %xmm1
	vroundss	$9, %xmm2, %xmm2, %xmm0
	vsubss	%xmm1, %xmm4, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm8
	vmulss	%xmm4, %xmm8, %xmm5
	vmovss	.LCPI32_25(%rip), %xmm11
	vaddss	%xmm5, %xmm11, %xmm5
	vmulss	%xmm5, %xmm4, %xmm5
	vmovss	.LCPI32_26(%rip), %xmm12
	vaddss	%xmm5, %xmm12, %xmm5
	vmulss	%xmm5, %xmm4, %xmm5
	vmovss	.LCPI32_27(%rip), %xmm13
	vaddss	%xmm5, %xmm13, %xmm5
	vmulss	%xmm5, %xmm4, %xmm5
	vmovss	.LCPI32_28(%rip), %xmm14
	vaddss	%xmm5, %xmm14, %xmm5
	vmulss	%xmm5, %xmm4, %xmm5
	vroundss	$9, %xmm3, %xmm3, %xmm4
	vsubss	%xmm0, %xmm2, %xmm6
	vsubss	%xmm4, %xmm3, %xmm2
	vmovss	.LCPI32_29(%rip), %xmm15
	vaddss	%xmm5, %xmm15, %xmm3
	vmovss	.LCPI32_30(%rip), %xmm7
	vaddss	%xmm7, %xmm1, %xmm1
	vmovd	%xmm1, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm1
	vmulss	%xmm1, %xmm3, %xmm1
	vmulss	%xmm6, %xmm8, %xmm3
	vaddss	%xmm3, %xmm11, %xmm3
	vmulss	%xmm3, %xmm6, %xmm3
	vaddss	%xmm3, %xmm12, %xmm3
	vmulss	%xmm3, %xmm6, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
	vmulss	%xmm3, %xmm6, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2833:
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp2834:
	vmovd	%esi, %xmm5
	vaddss	432(%rsp), %xmm5, %xmm5
	vmulss	.LCPI32_21(%rip), %xmm5, %xmm5
	vmaxss	%xmm10, %xmm5, %xmm5
	vminss	%xmm9, %xmm5, %xmm5
.Ltmp2835:
	vaddss	%xmm3, %xmm15, %xmm3
	vaddss	%xmm7, %xmm0, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm3, %xmm0
.Ltmp2836:
	vmulss	%xmm2, %xmm8, %xmm3
	vaddss	%xmm3, %xmm11, %xmm3
	vmulss	%xmm3, %xmm2, %xmm3
	vaddss	%xmm3, %xmm12, %xmm3
	vmulss	%xmm3, %xmm2, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
	vmulss	%xmm3, %xmm2, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2837:
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2838:
	vroundss	$9, %xmm5, %xmm5, %xmm3
	vsubss	%xmm3, %xmm5, %xmm5
.Ltmp2839:
	vaddss	%xmm2, %xmm15, %xmm2
	vaddss	%xmm7, %xmm4, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	vmulss	%xmm4, %xmm2, %xmm2
.Ltmp2840:
	vmulss	%xmm5, %xmm8, %xmm4
	vaddss	%xmm4, %xmm11, %xmm4
	vmulss	%xmm4, %xmm5, %xmm4
	vaddss	%xmm4, %xmm12, %xmm4
	vmulss	%xmm4, %xmm5, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
	vmulss	%xmm4, %xmm5, %xmm4
	vaddss	%xmm4, %xmm14, %xmm4
.Ltmp2841:
	vmulss	%xmm4, %xmm5, %xmm4
	vaddss	%xmm4, %xmm15, %xmm4
	vaddss	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm3
	movq	120(%rsp), %rax
.Ltmp2842:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
	movq	112(%rsp), %rax
	vmulss	(%rax,%rdi,4), %xmm0, %xmm0
.Ltmp2843:
	vmulss	%xmm3, %xmm4, %xmm3
.Ltmp2844:
	vaddss	%xmm0, %xmm1, %xmm0
	movq	104(%rsp), %rax
.Ltmp2845:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm1
.Ltmp2846:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r10,%rdi,4), %xmm3, %xmm2
.Ltmp2847:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	96(%rsp), %r8
.Ltmp2848:
	.loc	49 56 9
	vmovss	%xmm0, (%r8,%rbp,4)
	movq	88(%rsp), %rcx
.Ltmp2849:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%rcx,%rbp,4)
	incq	%rbp
.Ltmp2850:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rbp, 64(%rsp)
.Ltmp2851:
	.loc	3 900 12
	jne	.LBB32_112
.Ltmp2852:
.LBB32_187:
	.loc	3 0 12 is_stmt 0
	vmovaps	240(%rsp), %xmm0
	.loc	1 1057 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	208(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	272(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	256(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	224(%rsp), %xmm0
	.loc	1 1058 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	304(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	336(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	144(%rsp), %eax
	.loc	1 1059 5
	movl	%eax, 184(%r14)
	movl	160(%rsp), %eax
	movl	%eax, 188(%r14)
	movl	12(%rsp), %eax
	.loc	1 1060 5
	movl	%eax, 544(%r14)
	movl	%esi, 548(%r14)
	.loc	1 1061 5
	movq	%rdi, 848(%r14)
	jmp	.LBB32_188
.Ltmp2853:
.LBB32_517:
	.loc	1 0 5 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI32_1(%rip), %xmm0
	vmovss	.LCPI32_31(%rip), %xmm1
	xorl	%edx, %edx
	movq	464(%rsp), %rdi
	.p2align	4
.LBB32_518:
.Ltmp2854:
	.loc	49 103 24 is_stmt 1
	vmovss	(%rdi,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp2855:
	.loc	49 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp2856:
	.loc	49 139 9
	cmovbel	%ecx, %eax
.Ltmp2857:
	.loc	10 2155 12
	incq	%rdx
	cmpq	%rdx, %r9
	jne	.LBB32_518
.Ltmp2858:
	.loc	49 154 9
	cmpl	$-1, %eax
	movq	704(%rsp), %r12
	movq	472(%rsp), %r13
.Ltmp2859:
	.file	50 "/home/bl/misofm/engine-multiband-detector-access" "crates/effect-runtime/src/bank.rs"
	.loc	50 208 8
	jne	.LBB32_591
	.loc	50 0 8 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	xorl	%edx, %edx
	.p2align	4
.LBB32_521:
.Ltmp2860:
	.loc	49 103 24 is_stmt 1
	vmovss	(%r13,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp2861:
	.loc	49 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp2862:
	.loc	49 139 9
	cmovbel	%ecx, %eax
.Ltmp2863:
	.loc	10 2155 12
	incq	%rdx
	cmpq	%rdx, %r9
	jne	.LBB32_521
.Ltmp2864:
	.loc	49 154 9
	cmpl	$-1, %eax
.Ltmp2865:
	.loc	50 208 34
	je	.LBB32_523
.LBB32_591:
	.loc	50 0 34 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	xorl	%edx, %edx
	.p2align	4
.LBB32_592:
.Ltmp2866:
	.loc	49 103 24 is_stmt 1
	vmovss	(%rdi,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp2867:
	.loc	49 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp2868:
	.loc	49 139 9
	cmovbel	%ecx, %eax
.Ltmp2869:
	.loc	10 2155 12
	incq	%rdx
	cmpq	%rdx, %r9
	jne	.LBB32_592
.Ltmp2870:
	.loc	10 0 12 is_stmt 0
	movl	$-1, %ecx
	xorl	%edx, %edx
	xorl	%esi, %esi
	.p2align	4
.LBB32_594:
.Ltmp2871:
	.loc	49 103 24 is_stmt 1
	vmovss	(%r13,%rsi,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp2872:
	.loc	49 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp2873:
	.loc	49 139 9
	cmovbel	%edx, %ecx
.Ltmp2874:
	.loc	10 2155 12
	incq	%rsi
	cmpq	%rsi, %r9
	jne	.LBB32_594
.Ltmp2875:
	.loc	50 211 5
	andl	%ecx, %eax
	notl	%eax
	xorl	%ecx, %ecx
	testl	$1065353216, %eax
	setne	%cl
	movl	%ecx, 112(%r14)
	.loc	50 212 31
	movq	104(%r14), %rax
.Ltmp2876:
	.loc	38 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp2877:
	.loc	50 212 5
	movq	%rcx, 104(%r14)
.Ltmp2878:
	.loc	34 961 18
	leaq	(,%r9,4), %rbx
	movq	memset@GOTPCREL(%rip), %r15
.Ltmp2879:
	.loc	35 25 13
	xorl	%esi, %esi
	movq	%rbx, %rdx
	callq	*%r15
.Ltmp2880:
	.loc	35 25 13 is_stmt 0
	movq	%r13, %rdi
	xorl	%esi, %esi
	movq	%rbx, %rdx
	callq	*%r15
.Ltmp2881:
	.loc	1 1319 13 is_stmt 1
	movq	$0, 848(%r14)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%xmm0, 168(%r14)
	movq	$0, 184(%r14)
.Ltmp2882:
	.loc	1 746 9
	movq	128(%r14), %rdx
.Ltmp2883:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp2884:
	.loc	33 180 28
	je	.LBB32_597
.Ltmp2885:
	.loc	1 746 9
	movq	120(%r14), %rdi
.Ltmp2886:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp2887:
	.loc	35 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp2888:
.LBB32_597:
	.loc	1 747 9
	movq	144(%r14), %rdx
.Ltmp2889:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp2890:
	.loc	33 180 28
	je	.LBB32_599
.Ltmp2891:
	.loc	1 747 9
	movq	136(%r14), %rdi
.Ltmp2892:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp2893:
	.loc	35 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp2894:
.LBB32_599:
	.loc	36 117 24
	vmovss	196(%r14), %xmm0
	vmovss	212(%r14), %xmm1
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 192(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 200(%r14)
	.loc	36 117 9
	vmovss	%xmm1, 208(%r14)
	.loc	36 118 9
	movq	$0, 216(%r14)
	.loc	36 117 24
	vmovss	228(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 224(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 232(%r14)
	.loc	36 117 24
	vmovss	244(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 240(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 248(%r14)
	.loc	36 117 24
	vmovss	260(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 256(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 264(%r14)
	.loc	36 117 24
	vmovss	276(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 272(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 280(%r14)
	.loc	36 117 24
	vmovss	292(%r14), %xmm0
	vmovss	308(%r14), %xmm1
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 288(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 296(%r14)
	.loc	36 117 9
	vmovss	%xmm1, 304(%r14)
	.loc	36 118 9
	movq	$0, 312(%r14)
	.loc	36 117 24
	vmovss	324(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 320(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 328(%r14)
	.loc	36 117 24
	vmovss	340(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 336(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 344(%r14)
.Ltmp2895:
	.loc	1 744 9
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%xmm0, 528(%r14)
	movq	$0, 544(%r14)
	.loc	1 746 9
	movq	488(%r14), %rdx
.Ltmp2896:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp2897:
	.loc	33 180 28
	je	.LBB32_601
.Ltmp2898:
	.loc	1 746 9
	movq	480(%r14), %rdi
.Ltmp2899:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp2900:
	.loc	35 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp2901:
.LBB32_601:
	.loc	1 747 9
	movq	504(%r14), %rdx
.Ltmp2902:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp2903:
	.loc	33 180 28
	je	.LBB32_603
.Ltmp2904:
	.loc	1 747 9
	movq	496(%r14), %rdi
.Ltmp2905:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp2906:
	.loc	35 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp2907:
.LBB32_603:
	.loc	36 117 24
	vmovss	556(%r14), %xmm0
	vmovss	572(%r14), %xmm1
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 552(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 560(%r14)
	.loc	36 117 9
	vmovss	%xmm1, 568(%r14)
	.loc	36 118 9
	movq	$0, 576(%r14)
	.loc	36 117 24
	vmovss	588(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 584(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 592(%r14)
	.loc	36 117 24
	vmovss	604(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 600(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 608(%r14)
	.loc	36 117 24
	vmovss	620(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 616(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 624(%r14)
	.loc	36 117 24
	vmovss	636(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 632(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 640(%r14)
	.loc	36 117 24
	vmovss	652(%r14), %xmm0
	vmovss	668(%r14), %xmm1
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 648(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 656(%r14)
	.loc	36 117 9
	vmovss	%xmm1, 664(%r14)
	.loc	36 118 9
	movq	$0, 672(%r14)
	.loc	36 117 24
	vmovss	684(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 680(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 688(%r14)
	.loc	36 117 24
	vmovss	700(%r14), %xmm0
	.loc	36 117 9 is_stmt 0
	vmovss	%xmm0, 696(%r14)
	.loc	36 118 9 is_stmt 1
	movq	$0, 704(%r14)
	xorl	%eax, %eax
.Ltmp2908:
	.loc	1 1330 16
	testb	$1, 112(%r14)
	movq	184(%rsp), %rcx
	cmoveq	%rax, %rcx
.Ltmp2909:
.LBB32_604:
	.loc	1 1844 9
	vxorps	%xmm0, %xmm0, %xmm0
	movq	872(%rsp), %rax
	vmovups	%xmm0, (%rax)
	movq	%r12, 16(%rax)
	movq	%rcx, 24(%rax)
	movq	%rcx, 32(%rax)
.Ltmp2910:
.LBB32_605:
	.loc	1 1845 6 epilogue_begin
	addq	$952, %rsp
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
.LBB32_523:
	.cfi_def_cfa_offset 1008
	.loc	1 0 6 is_stmt 0
	xorl	%ecx, %ecx
.Ltmp2911:
	.loc	50 208 34 is_stmt 1
	jmp	.LBB32_604
.Ltmp2912:
.LBB32_617:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_f7aeb6b0a3ba8e73c50a5abef6c30558(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_129:
	leaq	.Lalloc_a8e669d1bed0fb747f8a7bff920a6571(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_618:
	leaq	.Lalloc_1e81c2bc19b75441ce2fb90ce8f9eb70(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_587:
	leaq	.Lalloc_007bf1cfdcf9f845db5ef166fc9a1790(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_588:
	leaq	.Lalloc_a45b75cd2d07085fe69fe186ba115725(%rip), %rcx
	movq	%r11, %rsi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_190:
	leaq	.Lalloc_36be93341d7083c8a492c530428430ea(%rip), %rcx
	movq	%rax, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_589:
	leaq	.Lalloc_8a6f1b2a44d3e33eba5c708677237c81(%rip), %rcx
	movq	%r12, %rsi
	movq	%r12, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_590:
	leaq	.Lalloc_4aa2eaec3d1833a4a887fe1d76c05ca7(%rip), %rcx
	movq	%r8, %rsi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_611:
.Ltmp2913:
	.loc	48 581 13 is_stmt 1
	leaq	.Lalloc_a5fe42438bf1cd51848d461e43168e01(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2914:
.LBB32_613:
	.loc	48 581 13
	leaq	.Lalloc_a5fe42438bf1cd51848d461e43168e01(%rip), %rcx
.Ltmp2915:
	.loc	1 0 0 is_stmt 0
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_612:
.Ltmp2916:
	.loc	48 581 13 is_stmt 1
	leaq	.Lalloc_947a12da53e0da5ef683a81874315ce0(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2917:
.LBB32_614:
	.loc	48 581 13
	leaq	.Lalloc_947a12da53e0da5ef683a81874315ce0(%rip), %rcx
.Ltmp2918:
	.loc	1 0 0 is_stmt 0
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_615:
.Ltmp2919:
	.loc	48 569 13 is_stmt 1
	leaq	.Lalloc_3c433846b75a8609e2aa4275946b39b4(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2920:
.LBB32_287:
	.loc	48 569 13
	leaq	.Lalloc_3c433846b75a8609e2aa4275946b39b4(%rip), %rcx
.Ltmp2921:
	.loc	1 0 0 is_stmt 0
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_289:
.Ltmp2922:
	.loc	48 569 13 is_stmt 1
	leaq	.Lalloc_f7cdc51debee5c62c8dce89684c9c22f(%rip), %rcx
.Ltmp2923:
	.loc	1 0 0 is_stmt 0
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_616:
.Ltmp2924:
	.loc	48 569 13 is_stmt 1
	leaq	.Lalloc_f7cdc51debee5c62c8dce89684c9c22f(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2925:
.LBB32_271:
	.loc	48 456 13
	leaq	.Lalloc_a05c61cb172b1016332ce1d3ce81e461(%rip), %rcx
	movq	%r12, %rdi
	movq	184(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2926:
.LBB32_290:
	.loc	48 456 13
	leaq	.Lalloc_61a2f59006034c74bb8ab3eed52140c6(%rip), %rcx
	movq	%r12, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2927:
.LBB32_317:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_a05c61cb172b1016332ce1d3ce81e461(%rip), %rcx
	movq	%rbp, %rdi
	movq	456(%rsp), %rsi
	movq	184(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_524:
	leaq	.Lalloc_61a2f59006034c74bb8ab3eed52140c6(%rip), %rcx
	movq	%rbp, %rdi
	movq	456(%rsp), %rsi
	movq	184(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_191:
.Ltmp2928:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_61a2f59006034c74bb8ab3eed52140c6(%rip), %rcx
.Ltmp2929:
	.loc	48 456 13 is_stmt 0
	movq	%rbp, %rdi
	movq	504(%rsp), %rsi
	movq	184(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2930:
.LBB32_114:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_a05c61cb172b1016332ce1d3ce81e461(%rip), %rcx
.Ltmp2931:
	.loc	48 456 13 is_stmt 0
	movq	%rbp, %rdi
	movq	504(%rsp), %rsi
	movq	184(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2932:
.LBB32_119:
	.loc	1 0 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_180:
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_619:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r9, %rdi
	movq	%r11, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_621:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r10, %rdi
	movq	%r12, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_586:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r10, %rdi
	movq	%r8, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_620:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r9, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_608:
.Ltmp2933:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r10, %rdi
	movq	%r11, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_610:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r9, %rdi
	movq	%r12, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_261:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r9, %rdi
	movq	%r8, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_609:
	leaq	.Lalloc_996d872dbf8039f736508df84ee27faf(%rip), %rdx
	movq	%r10, %rdi
	movq	%rax, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2934:
.LBB32_606:
	.loc	1 1376 43 is_stmt 1
	leaq	.Lalloc_26decc7ea6b284c3e2e2a8674b90acb8(%rip), %rdx
	movl	$12, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_607:
.Ltmp2935:
	.loc	1 1378 45
	leaq	.Lalloc_229cdd12ec7f4a0b614787777e6def50(%rip), %rdx
	movl	$10, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2936:
.Lfunc_end32:
	.size	_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process, .Lfunc_end32-_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process
