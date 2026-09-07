_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process:
.Lfunc_begin32:
	.loc	1 1926 0
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
	subq	$936, %rsp
	.cfi_def_cfa_offset 992
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rsi, %r14
.Ltmp1188:
	.loc	1 1930 13 prologue_end
	movq	32(%rdx), %r12
	movq	40(%rdx), %rax
	.loc	1 1931 13
	movl	92(%rsi), %esi
	movq	%rdx, 56(%rsp)
	.loc	1 1932 13
	movq	80(%rdx), %rdx
	movl	$0, 696(%rsp)
	movl	$0, 704(%rsp)
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
.Ltmp1189:
	.loc	32 1714 9
	testq	%rax, %rax
	movq	%rdi, 856(%rsp)
.Ltmp1190:
	.loc	33 180 28
	je	.LBB32_21
.Ltmp1191:
	.loc	33 0 28 is_stmt 0
	leaq	776(%rsp), %r9
	leaq	(%rax,%rax,4), %rax
	leaq	(%r12,%rax,8), %rbx
	movb	$1, %al
	movl	%eax, 16(%rsp)
	xorl	%ebp, %ebp
	leaq	696(%rsp), %r11
	xorl	%eax, %eax
	xorl	%r13d, %r13d
.LBB32_2:
	movq	%rax, %r8
	jmp	.LBB32_4
	.p2align	4
.LBB32_3:
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
	je	.LBB32_22
.Ltmp1196:
.LBB32_4:
	.loc	1 1457 30
	movl	32(%r12), %eax
	.loc	1 1457 24 is_stmt 0
	cmpl	$1, %eax
	je	.LBB32_8
	cmpl	$2, %eax
	jne	.LBB32_3
	.loc	1 0 24
	movl	$1, %eax
	movq	%r9, %r15
.Ltmp1197:
	.loc	1 1465 29 is_stmt 1
	movl	16(%r12), %edi
	cmpq	$2, %rdi
.Ltmp1198:
	.loc	38 1050 16
	jae	.LBB32_9
.Ltmp1199:
.LBB32_7:
	.loc	38 0 16 is_stmt 0
	xorl	%ecx, %ecx
	cmpq	%rsi, %r13
.Ltmp1200:
	.loc	1 1473 25 is_stmt 1
	jb	.LBB32_10
	jmp	.LBB32_3
.Ltmp1201:
	.loc	1 0 25 is_stmt 0
.Ltmp1202:
	.p2align	4
.LBB32_8:
	xorl	%eax, %eax
	movq	%r11, %r15
	.loc	1 1465 29 is_stmt 1
	movl	16(%r12), %edi
	cmpq	$2, %rdi
.Ltmp1203:
	.loc	38 1050 16
	jb	.LBB32_7
.LBB32_9:
	.loc	38 1054 31
	leaq	-2(%rdi), %rcx
	movq	%rcx, 288(%rsp)
.Ltmp1204:
	.loc	28 1580 16
	xorl	%ecx, %ecx
	cmpl	$12, %edi
	setb	%cl
	cmpq	%rsi, %r13
.Ltmp1205:
	.loc	1 1473 25
	jae	.LBB32_3
.LBB32_10:
	cmpq	$1, %rcx
	jne	.LBB32_3
	.loc	1 1475 20
	cmpl	$1, 28(%r12)
	jne	.LBB32_3
	.loc	1 1476 20
	cmpq	%rdx, (%r12)
	jne	.LBB32_3
	.loc	1 1477 20
	cmpq	%rdx, 8(%r12)
	jne	.LBB32_3
	.loc	1 1478 20
	vmovd	20(%r12), %xmm0
.Ltmp1206:
	.loc	23 1244 18
	vmovd	%xmm0, %ecx
.Ltmp1207:
	.loc	1 1478 20
	cmpl	%ecx, 24(%r12)
	jne	.LBB32_3
	.loc	1 0 20 is_stmt 0
	movq	%rdx, 72(%rsp)
	movq	%rsi, 80(%rsp)
	movq	%r8, 688(%rsp)
	.loc	1 1479 43 is_stmt 1
	cmpl	$11, %edi
	ja	.LBB32_650
	.loc	1 0 43 is_stmt 0
	leal	(%rax,%rdi,2), %eax
	movl	%eax, 28(%rsp)
	.loc	1 1479 42
	leaq	(%rdi,%rdi,4), %rax
	leaq	.Lalloc_cc33a3b9cd8c16d253f2168b5461d31d(%rip), %rcx
	leaq	(%rcx,%rax,8), %rdi
	vmovdqa	%xmm0, 416(%rsp)
	.loc	1 1479 20
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	416(%rsp), %xmm1
	movl	28(%rsp), %r10d
	cmpl	24(%rsp), %r10d
	seta	%cl
	testb	%al, %al
	movq	688(%rsp), %r8
	movq	80(%rsp), %rsi
	movq	72(%rsp), %rdx
	leaq	776(%rsp), %r9
	leaq	696(%rsp), %r11
	je	.LBB32_3
	orb	16(%rsp), %cl
	testb	$1, %cl
	je	.LBB32_3
	.loc	1 0 20
	movq	288(%rsp), %rdi
.Ltmp1208:
	.loc	1 1481 45 is_stmt 1
	cmpq	$9, %rdi
	ja	.LBB32_651
.Ltmp1209:
	.file	47 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/macros/mod.rs"
	.loc	47 430 9
	cmpl	$0, (%r15,%rdi,8)
.Ltmp1210:
	.loc	1 1486 17
	jne	.LBB32_3
.Ltmp1211:
	.loc	31 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
	movq	288(%rsp), %rax
.Ltmp1212:
	.loc	1 1491 13
	movl	$1, (%r15,%rax,8)
	vmovss	%xmm0, 4(%r15,%rax,8)
.Ltmp1213:
	.loc	32 1714 9
	addq	$40, %r12
.Ltmp1214:
	.loc	33 180 28
	incq	%r13
	movl	$0, 16(%rsp)
	movq	%r8, %rax
	movl	%r10d, 24(%rsp)
	movq	%rbp, %r8
.Ltmp1215:
	.loc	32 1714 9
	cmpq	%rbx, %r12
.Ltmp1216:
	.loc	33 180 28
	jne	.LBB32_2
	jmp	.LBB32_22
.Ltmp1217:
.LBB32_21:
	.loc	33 0 28 is_stmt 0
	xorl	%r8d, %r8d
.LBB32_22:
.Ltmp1218:
	.loc	33 180 28 is_stmt 1
	leaq	348(%r14), %rax
	movl	$76, %ecx
	vmovss	.LCPI32_0(%rip), %xmm0
	jmp	.LBB32_26
.Ltmp1219:
	.loc	33 0 28 is_stmt 0
.Ltmp1220:
	.p2align	4
.LBB32_23:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
.LBB32_24:
	.loc	36 0 0
	vmovd	%xmm1, -4(%rax)
	movl	%edx, (%rax)
.Ltmp1221:
.LBB32_25:
	.loc	32 1714 9 is_stmt 1
	addq	$80, %rcx
	addq	$360, %rax
	cmpq	$236, %rcx
.Ltmp1222:
	.loc	33 180 28
	je	.LBB32_94
.Ltmp1223:
.LBB32_26:
	.loc	1 1495 24
	cmpl	$1, 620(%rsp,%rcx)
	jne	.LBB32_27
	.loc	1 1495 29 is_stmt 0
	vmovd	624(%rsp,%rcx), %xmm1
.Ltmp1224:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -152(%rax)
	.loc	36 81 48
	vmovd	-156(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1225:
	.loc	36 112 9
	jg	.LBB32_40
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_40
	negl	%edx
	jo	.LBB32_40
.Ltmp1226:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -156(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -148(%rax)
	movl	%edx, -144(%rax)
.Ltmp1227:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 628(%rsp,%rcx)
	je	.LBB32_42
	.loc	1 0 24 is_stmt 0
.Ltmp1228:
	.p2align	4
.LBB32_28:
	.loc	1 1495 24
	cmpl	$1, 636(%rsp,%rcx)
	jne	.LBB32_29
.LBB32_48:
	.loc	1 1495 29
	vmovd	640(%rsp,%rcx), %xmm1
.Ltmp1229:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -120(%rax)
	.loc	36 81 48
	vmovd	-124(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1230:
	.loc	36 112 9
	jg	.LBB32_52
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_52
	negl	%edx
	jo	.LBB32_52
.Ltmp1231:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -124(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -116(%rax)
	movl	%edx, -112(%rax)
.Ltmp1232:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 644(%rsp,%rcx)
	je	.LBB32_54
	.loc	1 0 24 is_stmt 0
.Ltmp1233:
	.p2align	4
.LBB32_30:
	.loc	1 1495 24
	cmpl	$1, 652(%rsp,%rcx)
	jne	.LBB32_31
.LBB32_60:
	.loc	1 1495 29
	vmovd	656(%rsp,%rcx), %xmm1
.Ltmp1234:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -88(%rax)
	.loc	36 81 48
	vmovd	-92(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1235:
	.loc	36 112 9
	jg	.LBB32_64
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_64
	negl	%edx
	jo	.LBB32_64
.Ltmp1236:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -92(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -84(%rax)
	movl	%edx, -80(%rax)
.Ltmp1237:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 660(%rsp,%rcx)
	je	.LBB32_66
	.loc	1 0 24 is_stmt 0
.Ltmp1238:
	.p2align	4
.LBB32_32:
	.loc	1 1495 24
	cmpl	$1, 668(%rsp,%rcx)
	jne	.LBB32_33
.LBB32_72:
	.loc	1 1495 29
	vmovd	672(%rsp,%rcx), %xmm1
.Ltmp1239:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -56(%rax)
	.loc	36 81 48
	vmovd	-60(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1240:
	.loc	36 112 9
	jg	.LBB32_76
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_76
	negl	%edx
	jo	.LBB32_76
.Ltmp1241:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -60(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -52(%rax)
	movl	%edx, -48(%rax)
.Ltmp1242:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 676(%rsp,%rcx)
	je	.LBB32_78
	.loc	1 0 24 is_stmt 0
.Ltmp1243:
	.p2align	4
.LBB32_34:
	.loc	1 1495 24
	cmpl	$1, 684(%rsp,%rcx)
	jne	.LBB32_35
.LBB32_84:
	.loc	1 1495 29
	vmovd	688(%rsp,%rcx), %xmm1
.Ltmp1244:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -24(%rax)
	.loc	36 81 48
	vmovd	-28(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1245:
	.loc	36 112 9
	jg	.LBB32_88
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_88
	negl	%edx
	jo	.LBB32_88
.Ltmp1246:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -28(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -20(%rax)
	movl	%edx, -16(%rax)
.Ltmp1247:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 692(%rsp,%rcx)
	jne	.LBB32_25
	jmp	.LBB32_90
	.loc	1 0 24 is_stmt 0
.Ltmp1248:
	.p2align	4
.LBB32_27:
	.loc	1 1495 24
	cmpl	$1, 628(%rsp,%rcx)
	jne	.LBB32_28
.LBB32_42:
	.loc	1 1495 29
	vmovd	632(%rsp,%rcx), %xmm1
.Ltmp1249:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -136(%rax)
	.loc	36 81 48
	vmovd	-140(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1250:
	.loc	36 112 9
	jg	.LBB32_46
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_46
	negl	%edx
	jo	.LBB32_46
.Ltmp1251:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -140(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -132(%rax)
	movl	%edx, -128(%rax)
.Ltmp1252:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 636(%rsp,%rcx)
	jne	.LBB32_29
	jmp	.LBB32_48
	.loc	1 0 24 is_stmt 0
.Ltmp1253:
	.p2align	4
.LBB32_40:
.Ltmp1254:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -148(%rax)
	movl	%edx, -144(%rax)
.Ltmp1255:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 628(%rsp,%rcx)
	jne	.LBB32_28
	jmp	.LBB32_42
	.loc	1 0 24 is_stmt 0
.Ltmp1256:
	.p2align	4
.LBB32_52:
.Ltmp1257:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -116(%rax)
	movl	%edx, -112(%rax)
.Ltmp1258:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 644(%rsp,%rcx)
	jne	.LBB32_30
	jmp	.LBB32_54
	.loc	1 0 24 is_stmt 0
.Ltmp1259:
	.p2align	4
.LBB32_64:
.Ltmp1260:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -84(%rax)
	movl	%edx, -80(%rax)
.Ltmp1261:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 660(%rsp,%rcx)
	jne	.LBB32_32
	jmp	.LBB32_66
	.loc	1 0 24 is_stmt 0
.Ltmp1262:
	.p2align	4
.LBB32_76:
.Ltmp1263:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -52(%rax)
	movl	%edx, -48(%rax)
.Ltmp1264:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 676(%rsp,%rcx)
	jne	.LBB32_34
	jmp	.LBB32_78
	.loc	1 0 24 is_stmt 0
.Ltmp1265:
	.p2align	4
.LBB32_88:
.Ltmp1266:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -20(%rax)
	movl	%edx, -16(%rax)
.Ltmp1267:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 692(%rsp,%rcx)
	jne	.LBB32_25
	jmp	.LBB32_90
	.loc	1 0 24 is_stmt 0
.Ltmp1268:
	.p2align	4
.LBB32_46:
.Ltmp1269:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -132(%rax)
	movl	%edx, -128(%rax)
.Ltmp1270:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 636(%rsp,%rcx)
	je	.LBB32_48
	.loc	1 0 24 is_stmt 0
.Ltmp1271:
	.p2align	4
.LBB32_29:
	.loc	1 1495 24
	cmpl	$1, 644(%rsp,%rcx)
	jne	.LBB32_30
.LBB32_54:
	.loc	1 1495 29
	vmovd	648(%rsp,%rcx), %xmm1
.Ltmp1272:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -104(%rax)
	.loc	36 81 48
	vmovd	-108(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1273:
	.loc	36 112 9
	jg	.LBB32_58
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_58
	negl	%edx
	jo	.LBB32_58
.Ltmp1274:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -108(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -100(%rax)
	movl	%edx, -96(%rax)
.Ltmp1275:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 652(%rsp,%rcx)
	jne	.LBB32_31
	jmp	.LBB32_60
	.loc	1 0 24 is_stmt 0
.Ltmp1276:
	.p2align	4
.LBB32_58:
.Ltmp1277:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -100(%rax)
	movl	%edx, -96(%rax)
.Ltmp1278:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 652(%rsp,%rcx)
	je	.LBB32_60
	.loc	1 0 24 is_stmt 0
.Ltmp1279:
	.p2align	4
.LBB32_31:
	.loc	1 1495 24
	cmpl	$1, 660(%rsp,%rcx)
	jne	.LBB32_32
.LBB32_66:
	.loc	1 1495 29
	vmovd	664(%rsp,%rcx), %xmm1
.Ltmp1280:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -72(%rax)
	.loc	36 81 48
	vmovd	-76(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1281:
	.loc	36 112 9
	jg	.LBB32_70
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_70
	negl	%edx
	jo	.LBB32_70
.Ltmp1282:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -76(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -68(%rax)
	movl	%edx, -64(%rax)
.Ltmp1283:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 668(%rsp,%rcx)
	jne	.LBB32_33
	jmp	.LBB32_72
	.loc	1 0 24 is_stmt 0
.Ltmp1284:
	.p2align	4
.LBB32_70:
.Ltmp1285:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -68(%rax)
	movl	%edx, -64(%rax)
.Ltmp1286:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 668(%rsp,%rcx)
	je	.LBB32_72
	.loc	1 0 24 is_stmt 0
.Ltmp1287:
	.p2align	4
.LBB32_33:
	.loc	1 1495 24
	cmpl	$1, 676(%rsp,%rcx)
	jne	.LBB32_34
.LBB32_78:
	.loc	1 1495 29
	vmovd	680(%rsp,%rcx), %xmm1
.Ltmp1288:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -40(%rax)
	.loc	36 81 48
	vmovd	-44(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1289:
	.loc	36 112 9
	jg	.LBB32_82
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_82
	negl	%edx
	jo	.LBB32_82
.Ltmp1290:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -44(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 0 0 is_stmt 0
	vmovd	%xmm1, -36(%rax)
	movl	%edx, -32(%rax)
.Ltmp1291:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 684(%rsp,%rcx)
	jne	.LBB32_35
	jmp	.LBB32_84
	.loc	1 0 24 is_stmt 0
.Ltmp1292:
	.p2align	4
.LBB32_82:
.Ltmp1293:
	.loc	36 87 21 is_stmt 1
	vsubss	%xmm2, %xmm1, %xmm1
	.loc	36 87 9 is_stmt 0
	vmulss	%xmm0, %xmm1, %xmm1
	movl	$64, %edx
	.loc	36 0 0
	vmovss	%xmm1, -36(%rax)
	movl	%edx, -32(%rax)
.Ltmp1294:
	.loc	1 1495 24 is_stmt 1
	cmpl	$1, 684(%rsp,%rcx)
	je	.LBB32_84
	.loc	1 0 24 is_stmt 0
.Ltmp1295:
	.p2align	4
.LBB32_35:
	.loc	1 1495 24
	cmpl	$1, 692(%rsp,%rcx)
	jne	.LBB32_25
.LBB32_90:
	.loc	1 1495 29
	vmovd	696(%rsp,%rcx), %xmm1
.Ltmp1296:
	.loc	36 80 9 is_stmt 1
	vmovd	%xmm1, -8(%rax)
	.loc	36 81 48
	vmovd	-12(%rax), %xmm2
	vmovd	%xmm2, %edx
	movl	%edx, %esi
	andl	$2147483647, %esi
	cmpl	$2139095039, %esi
.Ltmp1297:
	.loc	36 112 9
	jg	.LBB32_23
	.loc	36 112 0 is_stmt 0
	vmovd	%xmm1, %esi
	.loc	36 112 9
	cmpl	%esi, %edx
	jne	.LBB32_23
	negl	%edx
	jo	.LBB32_23
.Ltmp1298:
	.loc	36 82 13 is_stmt 1
	vmovd	%xmm1, -12(%rax)
	xorl	%edx, %edx
	vpxor	%xmm1, %xmm1, %xmm1
	.loc	36 89 6
	jmp	.LBB32_24
.Ltmp1299:
.LBB32_94:
	.loc	36 0 6 is_stmt 0
	movq	56(%rsp), %rax
	.loc	1 1935 22 is_stmt 1
	movq	8(%rax), %r9
.Ltmp1300:
	.loc	1 1936 12
	testq	%r9, %r9
	je	.LBB32_100
	.loc	1 0 12 is_stmt 0
	movq	56(%rsp), %rax
	.loc	1 1936 12
	cmpq	%r9, 24(%rax)
	jne	.LBB32_100
	.loc	1 0 12
	movq	56(%rsp), %rax
	movq	(%rax), %rdx
	.loc	1 1936 27
	movq	16(%rax), %rax
	movq	%rax, 408(%rsp)
.Ltmp1301:
	.loc	1 1245 12 is_stmt 1
	movl	860(%r14), %ecx
	.loc	1 1245 27 is_stmt 0
	movzbl	864(%r14), %eax
	.loc	1 1245 5
	cmpl	$3, %ecx
	movq	%r9, 152(%rsp)
	movq	%r8, 688(%rsp)
	movq	%rdx, 440(%rsp)
	je	.LBB32_101
	cmpl	$2, %ecx
	jne	.LBB32_103
	testb	%al, %al
	jne	.LBB32_104
.Ltmp1302:
	.loc	1 1186 23 is_stmt 1
	movl	856(%r14), %eax
	movl	%eax, 476(%rsp)
.Ltmp1303:
	.loc	1 1187 20
	movq	840(%r14), %r8
	xorl	%ebp, %ebp
	movq	%r8, 448(%rsp)
	jmp	.LBB32_144
.Ltmp1304:
.LBB32_100:
	.loc	1 1937 20
	vxorps	%xmm0, %xmm0, %xmm0
	movq	856(%rsp), %rax
	vmovups	%xmm0, (%rax)
	movq	%r8, 16(%rax)
	vmovups	%xmm0, 24(%rax)
.Ltmp1305:
	.loc	1 1948 6
	jmp	.LBB32_604
.LBB32_101:
.Ltmp1306:
	.loc	1 1245 5
	testb	%al, %al
	jne	.LBB32_104
.Ltmp1307:
	.loc	1 1186 23
	movl	856(%r14), %eax
	movl	%eax, 476(%rsp)
.Ltmp1308:
	.loc	1 1187 20
	movq	840(%r14), %r8
	xorl	%ebp, %ebp
	movq	%r8, 448(%rsp)
	jmp	.LBB32_286
.Ltmp1309:
.LBB32_103:
	.loc	1 1245 5
	testb	%al, %al
	je	.LBB32_425
.LBB32_104:
.Ltmp1310:
	.loc	1 1186 23
	movl	856(%r14), %eax
	movl	%eax, 68(%rsp)
.Ltmp1311:
	.loc	1 1187 20
	movq	840(%r14), %rbx
	leaq	120(%r14), %r15
	leaq	480(%r14), %r13
	xorl	%r12d, %r12d
	jmp	.LBB32_108
.LBB32_105:
	.loc	1 0 20 is_stmt 0
	movq	%rax, %rdi
	leaq	120(%r14), %r15
	vmovss	40(%rsp), %xmm2
	vmovss	36(%rsp), %xmm3
	vmovss	32(%rsp), %xmm4
	vmovss	512(%rsp), %xmm5
	vmovss	624(%rsp), %xmm6
	vmovss	608(%rsp), %xmm7
	vmovss	592(%rsp), %xmm8
	vmovss	576(%rsp), %xmm9
	vmovss	560(%rsp), %xmm10
	vmovss	544(%rsp), %xmm11
	vmovss	528(%rsp), %xmm12
	vmovss	448(%rsp), %xmm13
	vmovss	320(%rsp), %xmm14
	vmovss	148(%rsp), %xmm15
.LBB32_106:
.Ltmp1312:
	.loc	1 1160 5 is_stmt 1
	vmovaps	864(%rsp), %xmm0
	leaq	168(%r14), %rax
	vmovups	%xmm0, (%rax)
	.loc	1 1161 5
	vmovaps	880(%rsp), %xmm0
	leaq	528(%r14), %rax
	vmovups	%xmm0, (%rax)
	.loc	1 1162 5
	movq	%rcx, 184(%r14)
	.loc	1 1163 5
	movq	%r8, 544(%r14)
	.loc	1 1164 5
	movq	%rdi, 848(%r14)
	vmovss	416(%rsp), %xmm0
.Ltmp1313:
	.loc	1 1300 17
	vmovss	%xmm0, 192(%r14)
	.loc	1 1301 34
	movl	204(%r14), %eax
.Ltmp1314:
	.loc	38 2472 13
	subl	%ebp, %eax
	movl	$0, %edx
	cmovbl	%edx, %eax
.Ltmp1315:
	.loc	1 1301 34
	movl	220(%r14), %ecx
	.loc	1 1301 17 is_stmt 0
	movl	%eax, 204(%r14)
	vmovss	72(%rsp), %xmm0
	.loc	1 1300 17 is_stmt 1
	vmovss	%xmm0, 208(%r14)
.Ltmp1316:
	.loc	38 2472 13
	subl	%ebp, %ecx
	cmovbl	%edx, %ecx
.Ltmp1317:
	.loc	1 1301 17
	movl	%ecx, 220(%r14)
	vmovss	80(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 224(%r14)
	.loc	1 1301 34
	movl	236(%r14), %eax
.Ltmp1318:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1319:
	.loc	1 1301 17
	movl	%eax, 236(%r14)
	vmovss	16(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 240(%r14)
	.loc	1 1301 34
	movl	252(%r14), %eax
.Ltmp1320:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1321:
	.loc	1 1301 17
	movl	%eax, 252(%r14)
	vmovss	56(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 256(%r14)
	.loc	1 1301 34
	movl	268(%r14), %eax
.Ltmp1322:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1323:
	.loc	1 1301 17
	movl	%eax, 268(%r14)
	vmovss	288(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 272(%r14)
	.loc	1 1301 34
	movl	284(%r14), %eax
.Ltmp1324:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1325:
	.loc	1 1301 17
	movl	%eax, 284(%r14)
	.loc	1 1300 17
	vmovss	%xmm15, 288(%r14)
	.loc	1 1301 34
	movl	300(%r14), %eax
.Ltmp1326:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1327:
	.loc	1 1301 17
	movl	%eax, 300(%r14)
	.loc	1 1300 17
	vmovss	%xmm14, 304(%r14)
	.loc	1 1301 34
	movl	316(%r14), %eax
.Ltmp1328:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1329:
	.loc	1 1301 17
	movl	%eax, 316(%r14)
	.loc	1 1300 17
	vmovss	%xmm13, 320(%r14)
	.loc	1 1301 34
	movl	332(%r14), %eax
.Ltmp1330:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1331:
	.loc	1 1301 17
	movl	%eax, 332(%r14)
	.loc	1 1300 17
	vmovss	%xmm12, 336(%r14)
	.loc	1 1301 34
	movl	348(%r14), %eax
.Ltmp1332:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1333:
	.loc	1 1301 17
	movl	%eax, 348(%r14)
.Ltmp1334:
	.loc	1 1300 17
	vmovss	%xmm11, 552(%r14)
	.loc	1 1301 34
	movl	564(%r14), %eax
.Ltmp1335:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1336:
	.loc	1 1301 17
	movl	%eax, 564(%r14)
	.loc	1 1300 17
	vmovss	%xmm10, 568(%r14)
	.loc	1 1301 34
	movl	580(%r14), %eax
.Ltmp1337:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1338:
	.loc	1 1301 17
	movl	%eax, 580(%r14)
	.loc	1 1300 17
	vmovss	%xmm9, 584(%r14)
	.loc	1 1301 34
	movl	596(%r14), %eax
.Ltmp1339:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1340:
	.loc	1 1301 17
	movl	%eax, 596(%r14)
	.loc	1 1300 17
	vmovss	%xmm8, 600(%r14)
	.loc	1 1301 34
	movl	612(%r14), %eax
.Ltmp1341:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1342:
	.loc	1 1301 17
	movl	%eax, 612(%r14)
	.loc	1 1300 17
	vmovss	%xmm7, 616(%r14)
	.loc	1 1301 34
	movl	628(%r14), %eax
.Ltmp1343:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1344:
	.loc	1 1301 17
	movl	%eax, 628(%r14)
	.loc	1 1300 17
	vmovss	%xmm6, 632(%r14)
	.loc	1 1301 34
	movl	644(%r14), %eax
.Ltmp1345:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1346:
	.loc	1 1301 17
	movl	%eax, 644(%r14)
	.loc	1 1300 17
	vmovss	%xmm5, 648(%r14)
	.loc	1 1301 34
	movl	660(%r14), %eax
.Ltmp1347:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1348:
	.loc	1 1301 17
	movl	%eax, 660(%r14)
	.loc	1 1300 17
	vmovss	%xmm4, 664(%r14)
	.loc	1 1301 34
	movl	676(%r14), %eax
.Ltmp1349:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1350:
	.loc	1 1301 17
	movl	%eax, 676(%r14)
	.loc	1 1300 17
	vmovss	%xmm3, 680(%r14)
	.loc	1 1301 34
	movl	692(%r14), %eax
.Ltmp1351:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1352:
	.loc	1 1301 17
	movl	%eax, 692(%r14)
	.loc	1 1300 17
	vmovss	%xmm2, 696(%r14)
	.loc	1 1301 34
	movl	708(%r14), %eax
.Ltmp1353:
	.loc	38 2472 13
	subl	%ebp, %eax
	cmovbl	%edx, %eax
.Ltmp1354:
	.loc	1 1301 17
	movl	%eax, 708(%r14)
	movq	%rsi, %r12
	movq	152(%rsp), %r9
.Ltmp1355:
	.loc	1 1189 11
	cmpq	%r9, %rsi
	jae	.LBB32_584
.LBB32_108:
	.loc	1 1190 42
	movq	%r9, %rsi
	subq	%r12, %rsi
	.loc	1 1190 29 is_stmt 0
	leaq	104(%r14), %rdi
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_
	movb	%dl, 44(%rsp)
	movq	%rax, %rbp
	vmovss	192(%r14), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	200(%r14), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	208(%r14), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	216(%r14), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	224(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	232(%r14), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	240(%r14), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	248(%r14), %xmm0
	vmovss	%xmm0, 256(%rsp)
	vmovss	256(%r14), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	264(%r14), %xmm0
	vmovss	%xmm0, 240(%rsp)
	vmovss	272(%r14), %xmm0
	vmovss	%xmm0, 288(%rsp)
	vmovss	280(%r14), %xmm0
	vmovss	%xmm0, 224(%rsp)
	vmovss	288(%r14), %xmm0
	vmovss	%xmm0, 148(%rsp)
	vmovss	296(%r14), %xmm0
	vmovss	%xmm0, 208(%rsp)
	vmovss	304(%r14), %xmm0
	vmovss	%xmm0, 320(%rsp)
	vmovss	312(%r14), %xmm0
	vmovss	%xmm0, 192(%rsp)
	vmovss	320(%r14), %xmm0
	vmovss	%xmm0, 448(%rsp)
	vmovss	328(%r14), %xmm0
	vmovss	%xmm0, 176(%rsp)
	vmovss	336(%r14), %xmm0
	vmovss	%xmm0, 528(%rsp)
	vmovss	344(%r14), %xmm0
	vmovss	%xmm0, 160(%rsp)
	vmovss	552(%r14), %xmm0
	vmovss	%xmm0, 544(%rsp)
	vmovss	560(%r14), %xmm0
	vmovss	%xmm0, 8(%rsp)
	vmovss	568(%r14), %xmm0
	vmovss	%xmm0, 560(%rsp)
	vmovss	576(%r14), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	584(%r14), %xmm0
	vmovss	%xmm0, 576(%rsp)
	vmovss	592(%r14), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	600(%r14), %xmm0
	vmovss	%xmm0, 592(%rsp)
	vmovss	608(%r14), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	616(%r14), %xmm0
	vmovss	%xmm0, 608(%rsp)
	vmovss	624(%r14), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	632(%r14), %xmm0
	vmovss	%xmm0, 624(%rsp)
	vmovss	640(%r14), %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	648(%r14), %xmm0
	vmovss	%xmm0, 512(%rsp)
	vmovss	656(%r14), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	664(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	672(%r14), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	680(%r14), %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	688(%r14), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	696(%r14), %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	704(%r14), %xmm0
	vmovss	%xmm0, 88(%rsp)
.Ltmp1356:
	.loc	1 1194 31 is_stmt 1
	leaq	480(%rsp), %rdi
	movq	%r15, %rsi
	movl	68(%rsp), %r15d
	movl	%r15d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	696(%rsp), %rdi
	movq	%r13, %rsi
	movl	%r15d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	movq	152(%rsp), %rdx
.Ltmp1357:
	.loc	1 0 0 is_stmt 0
	movq	%rbp, %rsi
	addq	%r12, %rsi
	setb	%cl
	cmpq	%rdx, %rsi
	seta	%al
.Ltmp1358:
	.loc	38 1050 16 is_stmt 1
	orb	%cl, %al
.Ltmp1359:
	.loc	1 1197 12
	testb	$1, 44(%rsp)
	je	.LBB32_124
.Ltmp1360:
	.loc	38 1050 16
	testb	%al, %al
	movq	408(%rsp), %rdi
	jne	.LBB32_641
.Ltmp1361:
	.loc	1 1053 27
	leaq	168(%r14), %rax
	vmovups	(%rax), %xmm0
	vmovaps	%xmm0, 864(%rsp)
.Ltmp1362:
	.loc	1 1054 26
	leaq	528(%r14), %rax
	vmovups	(%rax), %xmm0
	vmovaps	%xmm0, 880(%rsp)
.Ltmp1363:
	.loc	1 1055 25
	movq	184(%r14), %rcx
.Ltmp1364:
	.loc	1 1056 24
	movq	544(%r14), %r8
.Ltmp1365:
	.loc	1 1057 24
	movq	848(%r14), %rax
.Ltmp1366:
	.loc	2 1916 50
	testq	%rbp, %rbp
.Ltmp1367:
	.loc	3 900 12
	je	.LBB32_105
.Ltmp1368:
	.loc	3 0 12 is_stmt 0
	movq	440(%rsp), %rdx
	leaq	(%rdx,%r12,4), %r9
	leaq	(%rdi,%r12,4), %r10
	xorl	%r11d, %r11d
	leaq	120(%r14), %r15
	vmovss	40(%rsp), %xmm2
	vmovss	36(%rsp), %xmm3
	vmovss	32(%rsp), %xmm4
	vmovss	512(%rsp), %xmm5
	vmovss	624(%rsp), %xmm6
	vmovss	608(%rsp), %xmm7
	vmovss	592(%rsp), %xmm8
	vmovss	576(%rsp), %xmm9
	vmovss	560(%rsp), %xmm10
	vmovss	544(%rsp), %xmm11
	vmovss	528(%rsp), %xmm12
	vmovss	448(%rsp), %xmm13
	vmovss	320(%rsp), %xmm14
	vmovss	148(%rsp), %xmm15
	.p2align	4
.LBB32_112:
.Ltmp1369:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%rax), %rdx
.Ltmp1370:
	.loc	1 857 8
	cmpq	%rbx, %rdx
	jb	.LBB32_114
.Ltmp1371:
	.loc	1 0 8 is_stmt 0
	movq	%rbx, %rdi
	jmp	.LBB32_115
	.p2align	4
.LBB32_114:
	xorl	%edi, %edi
.LBB32_115:
.Ltmp1372:
	.loc	1 1074 35 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp1373:
	.file	48 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/slice/index.rs"
	.loc	48 580 12
	cmpq	%rdx, %rax
	ja	.LBB32_607
.Ltmp1374:
	.file	49 "/home/bl/misofm/engine-multiband-detector-access" "crates/lane/src/scalar.rs"
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1375:
	.loc	1 0 0 is_stmt 0
	vmovss	(%r9,%r11,4), %xmm1
.Ltmp1376:
	vmovss	(%r10,%r11,4), %xmm0
.Ltmp1377:
	.loc	1 1074 0 is_stmt 1
	movq	(%r15), %rdx
.Ltmp1378:
	.loc	49 56 9
	vmovss	%xmm1, (%rdx,%rax,4)
.Ltmp1379:
	.loc	1 1075 34
	movq	488(%r14), %rdx
.Ltmp1380:
	.loc	48 580 12
	cmpq	%rdx, %rax
	ja	.LBB32_609
.Ltmp1381:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1382:
	.loc	1 0 0 is_stmt 0
	negq	%rdi
	addq	%rax, %rdi
	incq	%rdi
.Ltmp1383:
	.loc	1 1075 0 is_stmt 1
	movq	(%r13), %rdx
.Ltmp1384:
	.loc	49 56 9
	vmovss	%xmm0, (%rdx,%rax,4)
.Ltmp1385:
	.loc	1 1076 22
	movq	128(%r14), %rdx
.Ltmp1386:
	.loc	48 568 12
	cmpq	%rdx, %rdi
	ja	.LBB32_610
.Ltmp1387:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1388:
	.loc	1 1076 0
	movq	(%r15), %rax
.Ltmp1389:
	.loc	49 51 9
	vmovss	(%rax,%rdi,4), %xmm0
.Ltmp1390:
	.loc	49 56 9
	vmovss	%xmm0, (%r9,%r11,4)
.Ltmp1391:
	.loc	1 1077 22
	movq	488(%r14), %rdx
.Ltmp1392:
	.loc	48 568 12
	cmpq	%rdx, %rdi
	ja	.LBB32_613
.Ltmp1393:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1394:
	.loc	1 1077 0
	movq	(%r13), %rax
.Ltmp1395:
	.loc	49 51 9
	vmovss	(%rax,%rdi,4), %xmm0
.Ltmp1396:
	.loc	49 56 9
	vmovss	%xmm0, (%r10,%r11,4)
	vmovss	416(%rsp), %xmm0
.Ltmp1397:
	.loc	1 0 0 is_stmt 0
	vaddss	28(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 416(%rsp)
	vaddss	8(%rsp), %xmm11, %xmm11
	vmovss	72(%rsp), %xmm0
	vaddss	24(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 72(%rsp)
	vaddss	48(%rsp), %xmm10, %xmm10
	vmovss	80(%rsp), %xmm0
	vaddss	304(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 80(%rsp)
	vaddss	128(%rsp), %xmm9, %xmm9
	vmovss	16(%rsp), %xmm0
	vaddss	256(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 16(%rsp)
	vaddss	20(%rsp), %xmm8, %xmm8
	vmovss	56(%rsp), %xmm0
	vaddss	240(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 56(%rsp)
	vaddss	12(%rsp), %xmm7, %xmm7
	vmovss	288(%rsp), %xmm0
	vaddss	224(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 288(%rsp)
	vaddss	120(%rsp), %xmm6, %xmm6
	vaddss	208(%rsp), %xmm15, %xmm15
	vaddss	112(%rsp), %xmm5, %xmm5
	vaddss	192(%rsp), %xmm14, %xmm14
	vaddss	104(%rsp), %xmm4, %xmm4
	vaddss	176(%rsp), %xmm13, %xmm13
	vaddss	96(%rsp), %xmm3, %xmm3
	vaddss	160(%rsp), %xmm12, %xmm12
	vaddss	88(%rsp), %xmm2, %xmm2
	incq	%r11
	movq	%rdi, %rax
.Ltmp1398:
	.loc	2 1916 50 is_stmt 1
	cmpq	%r11, %rbp
.Ltmp1399:
	.loc	3 900 12
	jne	.LBB32_112
	jmp	.LBB32_106
.Ltmp1400:
	.loc	3 0 12 is_stmt 0
.Ltmp1401:
	.p2align	4
.LBB32_124:
	.loc	38 1050 16 is_stmt 1
	testb	%al, %al
	jne	.LBB32_640
.Ltmp1402:
	.loc	1 1053 27
	leaq	168(%r14), %rax
	vmovups	(%rax), %xmm0
	vmovaps	%xmm0, 896(%rsp)
.Ltmp1403:
	.loc	1 1054 26
	leaq	528(%r14), %rax
	vmovups	(%rax), %xmm0
	vmovaps	%xmm0, 912(%rsp)
.Ltmp1404:
	.loc	1 1055 25
	movq	184(%r14), %rcx
.Ltmp1405:
	.loc	1 1056 24
	movq	544(%r14), %r8
.Ltmp1406:
	.loc	1 1057 24
	movq	848(%r14), %rdi
.Ltmp1407:
	.loc	2 1916 50
	testq	%rbp, %rbp
.Ltmp1408:
	.loc	3 900 12
	je	.LBB32_139
.Ltmp1409:
	.loc	3 0 12 is_stmt 0
	movq	440(%rsp), %rax
	leaq	(%rax,%r12,4), %r9
	movq	408(%rsp), %rax
	leaq	(%rax,%r12,4), %r10
	xorl	%r11d, %r11d
	leaq	120(%r14), %r15
	.p2align	4
.LBB32_127:
.Ltmp1410:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp1411:
	.loc	1 857 8
	cmpq	%rbx, %rax
	jb	.LBB32_129
.Ltmp1412:
	.loc	1 0 8 is_stmt 0
	movq	%rbx, %rax
	jmp	.LBB32_130
	.p2align	4
.LBB32_129:
	xorl	%eax, %eax
.LBB32_130:
.Ltmp1413:
	.loc	1 1074 35 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp1414:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_606
.Ltmp1415:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1416:
	.loc	1 0 0 is_stmt 0
	vmovss	(%r9,%r11,4), %xmm1
.Ltmp1417:
	vmovss	(%r10,%r11,4), %xmm0
.Ltmp1418:
	.loc	1 1074 0 is_stmt 1
	movq	(%r15), %rdx
.Ltmp1419:
	.loc	49 56 9
	vmovss	%xmm1, (%rdx,%rdi,4)
.Ltmp1420:
	.loc	1 1075 34
	movq	488(%r14), %rdx
.Ltmp1421:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_608
.Ltmp1422:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1423:
	.loc	1 0 0 is_stmt 0
	negq	%rax
	addq	%rdi, %rax
	incq	%rax
.Ltmp1424:
	.loc	1 1075 0 is_stmt 1
	movq	(%r13), %rdx
.Ltmp1425:
	.loc	49 56 9
	vmovss	%xmm0, (%rdx,%rdi,4)
.Ltmp1426:
	.loc	1 1076 22
	movq	128(%r14), %rdx
.Ltmp1427:
	.loc	48 568 12
	cmpq	%rdx, %rax
	ja	.LBB32_611
.Ltmp1428:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1429:
	.loc	1 1076 0
	movq	(%r15), %rdx
.Ltmp1430:
	.loc	49 51 9
	vmovss	(%rdx,%rax,4), %xmm0
.Ltmp1431:
	.loc	49 56 9
	vmovss	%xmm0, (%r9,%r11,4)
.Ltmp1432:
	.loc	1 1077 22
	movq	488(%r14), %rdx
.Ltmp1433:
	.loc	48 568 12
	cmpq	%rdx, %rax
	ja	.LBB32_612
.Ltmp1434:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_644
	.loc	1 1077 0 is_stmt 1
	movq	(%r13), %rdx
.Ltmp1435:
	.loc	49 51 9
	vmovss	(%rdx,%rax,4), %xmm0
.Ltmp1436:
	.loc	49 56 9
	vmovss	%xmm0, (%r10,%r11,4)
.Ltmp1437:
	.loc	1 0 0 is_stmt 0
	incq	%r11
	movq	%rax, %rdi
.Ltmp1438:
	.loc	2 1916 50 is_stmt 1
	cmpq	%r11, %rbp
.Ltmp1439:
	.loc	3 900 12
	jne	.LBB32_127
	jmp	.LBB32_140
.Ltmp1440:
.LBB32_139:
	.loc	3 0 12 is_stmt 0
	movq	%rdi, %rax
	leaq	120(%r14), %r15
.LBB32_140:
	.loc	1 1160 5 is_stmt 1
	vmovaps	896(%rsp), %xmm0
	leaq	168(%r14), %rdx
	vmovups	%xmm0, (%rdx)
	.loc	1 1161 5
	vmovaps	912(%rsp), %xmm0
	leaq	528(%r14), %rdx
	vmovups	%xmm0, (%rdx)
	.loc	1 1162 5
	movq	%rcx, 184(%r14)
	.loc	1 1163 5
	movq	%r8, 544(%r14)
	.loc	1 1164 5
	movq	%rax, 848(%r14)
	movq	%rsi, %r12
	movq	152(%rsp), %r9
.Ltmp1441:
	.loc	1 1189 11
	cmpq	%r9, %rsi
	jb	.LBB32_108
.Ltmp1442:
.LBB32_584:
	.loc	1 0 11 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI32_1(%rip), %xmm0
	vmovss	.LCPI32_31(%rip), %xmm1
	xorl	%edx, %edx
	movq	440(%rsp), %rdi
	.p2align	4
.LBB32_585:
.Ltmp1443:
	.loc	49 103 24 is_stmt 1
	vmovss	(%rdi,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp1444:
	.loc	49 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp1445:
	.loc	49 139 9
	cmovbel	%ecx, %eax
.Ltmp1446:
	.loc	10 2155 12
	incq	%rdx
	cmpq	%rdx, %r9
	jne	.LBB32_585
.Ltmp1447:
	.loc	49 154 9
	cmpl	$-1, %eax
	movq	688(%rsp), %r12
.Ltmp1448:
	.file	50 "/home/bl/misofm/engine-multiband-detector-access" "crates/effect-runtime/src/bank.rs"
	.loc	50 208 8
	jne	.LBB32_590
	.loc	50 0 8 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	xorl	%edx, %edx
	movq	408(%rsp), %rsi
	.p2align	4
.LBB32_588:
.Ltmp1449:
	.loc	49 103 24 is_stmt 1
	vmovss	(%rsi,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp1450:
	.loc	49 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp1451:
	.loc	49 139 9
	cmovbel	%ecx, %eax
.Ltmp1452:
	.loc	10 2155 12
	incq	%rdx
	cmpq	%rdx, %r9
	jne	.LBB32_588
.Ltmp1453:
	.loc	49 154 9
	cmpl	$-1, %eax
.Ltmp1454:
	.loc	50 208 34
	je	.LBB32_605
.LBB32_590:
	.loc	50 0 34 is_stmt 0
	movl	$-1, %eax
	xorl	%ecx, %ecx
	xorl	%edx, %edx
	.p2align	4
.LBB32_591:
.Ltmp1455:
	.loc	49 103 24 is_stmt 1
	vmovss	(%rdi,%rdx,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp1456:
	.loc	49 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp1457:
	.loc	49 139 9
	cmovbel	%ecx, %eax
.Ltmp1458:
	.loc	10 2155 12
	incq	%rdx
	cmpq	%rdx, %r9
	jne	.LBB32_591
.Ltmp1459:
	.loc	10 0 12 is_stmt 0
	movl	$-1, %ecx
	xorl	%edx, %edx
	xorl	%esi, %esi
	movq	408(%rsp), %r13
	.p2align	4
.LBB32_593:
.Ltmp1460:
	.loc	49 103 24 is_stmt 1
	vmovss	(%r13,%rsi,4), %xmm2
	vandps	%xmm0, %xmm2, %xmm2
.Ltmp1461:
	.loc	49 114 14
	vucomiss	%xmm2, %xmm1
.Ltmp1462:
	.loc	49 139 9
	cmovbel	%edx, %ecx
.Ltmp1463:
	.loc	10 2155 12
	incq	%rsi
	cmpq	%rsi, %r9
	jne	.LBB32_593
.Ltmp1464:
	.loc	50 211 5
	andl	%ecx, %eax
	notl	%eax
	xorl	%ecx, %ecx
	testl	$1065353216, %eax
	setne	%cl
	movl	%ecx, 112(%r14)
	.loc	50 212 31
	movq	104(%r14), %rax
.Ltmp1465:
	.loc	38 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp1466:
	.loc	50 212 5
	movq	%rcx, 104(%r14)
.Ltmp1467:
	.loc	34 961 18
	leaq	(,%r9,4), %rbx
	movq	memset@GOTPCREL(%rip), %r15
.Ltmp1468:
	.loc	35 25 13
	xorl	%esi, %esi
	movq	%rbx, %rdx
	callq	*%r15
.Ltmp1469:
	.loc	35 25 13 is_stmt 0
	movq	%r13, %rdi
	xorl	%esi, %esi
	movq	%rbx, %rdx
	callq	*%r15
.Ltmp1470:
	.loc	1 1422 13 is_stmt 1
	movq	$0, 848(%r14)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%xmm0, 168(%r14)
	movq	$0, 184(%r14)
.Ltmp1471:
	.loc	1 746 9
	movq	128(%r14), %rdx
.Ltmp1472:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp1473:
	.loc	33 180 28
	je	.LBB32_596
.Ltmp1474:
	.loc	1 746 9
	movq	120(%r14), %rdi
.Ltmp1475:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp1476:
	.loc	35 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp1477:
.LBB32_596:
	.loc	1 747 9
	movq	144(%r14), %rdx
.Ltmp1478:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp1479:
	.loc	33 180 28
	je	.LBB32_598
.Ltmp1480:
	.loc	1 747 9
	movq	136(%r14), %rdi
.Ltmp1481:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp1482:
	.loc	35 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp1483:
.LBB32_598:
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
.Ltmp1484:
	.loc	1 744 9
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%xmm0, 528(%r14)
	movq	$0, 544(%r14)
	.loc	1 746 9
	movq	488(%r14), %rdx
.Ltmp1485:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp1486:
	.loc	33 180 28
	je	.LBB32_600
.Ltmp1487:
	.loc	1 746 9
	movq	480(%r14), %rdi
.Ltmp1488:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp1489:
	.loc	35 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp1490:
.LBB32_600:
	.loc	1 747 9
	movq	504(%r14), %rdx
.Ltmp1491:
	.loc	32 1714 9
	testq	%rdx, %rdx
.Ltmp1492:
	.loc	33 180 28
	je	.LBB32_602
.Ltmp1493:
	.loc	1 747 9
	movq	496(%r14), %rdi
.Ltmp1494:
	.loc	34 961 18
	shlq	$2, %rdx
.Ltmp1495:
	.loc	35 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp1496:
.LBB32_602:
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
.Ltmp1497:
	.loc	1 1433 16
	testb	$1, 112(%r14)
	movq	152(%rsp), %rcx
	cmoveq	%rax, %rcx
.Ltmp1498:
.LBB32_603:
	.loc	1 1947 9
	vxorps	%xmm0, %xmm0, %xmm0
	movq	856(%rsp), %rax
	vmovups	%xmm0, (%rax)
	movq	%r12, 16(%rax)
	movq	%rcx, 24(%rax)
	movq	%rcx, 32(%rax)
.Ltmp1499:
.LBB32_604:
	.loc	1 1948 6 epilogue_begin
	addq	$936, %rsp
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
.LBB32_141:
	.cfi_def_cfa_offset 992
	.loc	1 0 6 is_stmt 0
	movl	%ecx, %ebp
.LBB32_142:
	vmovaps	256(%rsp), %xmm0
.Ltmp1500:
	.loc	1 1160 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	240(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	304(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	208(%rsp), %xmm0
	.loc	1 1161 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	224(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	176(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	8(%rsp), %eax
	.loc	1 1162 5
	movl	%eax, 184(%r14)
	movl	%ebp, 188(%r14)
	.loc	1 1163 5
	movl	%r13d, 544(%r14)
	movl	%ebx, 548(%r14)
	.loc	1 1164 5
	movq	%rdi, 848(%r14)
.Ltmp1501:
.LBB32_143:
	.loc	1 0 5 is_stmt 0
	movq	648(%rsp), %rax
	movq	%rax, %rbp
	movq	152(%rsp), %r9
	.loc	1 1189 11 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB32_584
.LBB32_144:
	.loc	1 1190 42
	movq	%r9, %rsi
	subq	%rbp, %rsi
	.loc	1 1190 29 is_stmt 0
	leaq	104(%r14), %rdi
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r15
	vmovss	192(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	200(%r14), %xmm0
	vmovss	%xmm0, 404(%rsp)
	vmovss	208(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	216(%r14), %xmm0
	vmovss	%xmm0, 388(%rsp)
	vmovss	224(%r14), %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	232(%r14), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	240(%r14), %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	248(%r14), %xmm0
	vmovss	%xmm0, 380(%rsp)
	vmovss	256(%r14), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	264(%r14), %xmm0
	vmovss	%xmm0, 376(%rsp)
	vmovss	272(%r14), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	280(%r14), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	288(%r14), %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	296(%r14), %xmm0
	vmovss	%xmm0, 372(%rsp)
	vmovss	304(%r14), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	312(%r14), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	320(%r14), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	328(%r14), %xmm0
	vmovss	%xmm0, 364(%rsp)
	vmovss	336(%r14), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	344(%r14), %xmm0
	vmovss	%xmm0, 360(%rsp)
	vmovss	552(%r14), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	560(%r14), %xmm0
	vmovss	%xmm0, 396(%rsp)
	vmovss	568(%r14), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	576(%r14), %xmm0
	vmovss	%xmm0, 356(%rsp)
	vmovss	584(%r14), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	592(%r14), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	600(%r14), %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	608(%r14), %xmm0
	vmovss	%xmm0, 348(%rsp)
	vmovss	616(%r14), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	624(%r14), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	632(%r14), %xmm0
	vmovss	%xmm0, 288(%rsp)
	vmovss	640(%r14), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	648(%r14), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	656(%r14), %xmm0
	vmovss	%xmm0, 340(%rsp)
	vmovss	664(%r14), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	672(%r14), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	680(%r14), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	688(%r14), %xmm0
	vmovss	%xmm0, 332(%rsp)
	vmovss	696(%r14), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	704(%r14), %xmm0
	vmovss	%xmm0, 664(%rsp)
.Ltmp1502:
	.loc	1 1194 31 is_stmt 1
	leaq	480(%rsp), %rdi
	leaq	120(%r14), %rsi
	movl	476(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	696(%rsp), %rdi
	leaq	480(%r14), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1193 28
	vmovss	480(%rsp), %xmm0
	vmovss	%xmm0, 144(%rsp)
	vmovss	484(%rsp), %xmm0
	vmovaps	%xmm0, 512(%rsp)
	vmovss	488(%rsp), %xmm0
	vmovaps	%xmm0, 624(%rsp)
	vmovss	492(%rsp), %xmm0
	vmovss	%xmm0, 140(%rsp)
	vmovss	496(%rsp), %xmm0
	vmovaps	%xmm0, 608(%rsp)
	vmovss	500(%rsp), %xmm0
	vmovaps	%xmm0, 592(%rsp)
	vmovss	696(%rsp), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	700(%rsp), %xmm0
	vmovaps	%xmm0, 576(%rsp)
	vmovss	704(%rsp), %xmm0
	vmovaps	%xmm0, 560(%rsp)
	vmovss	708(%rsp), %xmm0
	vmovss	%xmm0, 284(%rsp)
	vmovss	712(%rsp), %xmm0
	vmovaps	%xmm0, 544(%rsp)
	vmovss	716(%rsp), %xmm0
	vmovaps	%xmm0, 528(%rsp)
	movq	%r15, 48(%rsp)
.Ltmp1503:
	.loc	1 0 0 is_stmt 0
	addq	%rbp, %r15
	setb	%cl
	cmpq	152(%rsp), %r15
	seta	%al
.Ltmp1504:
	.loc	38 1050 16 is_stmt 1
	orb	%cl, %al
.Ltmp1505:
	.loc	1 1197 12
	testb	$1, %r12b
	movq	%r15, 648(%rsp)
	je	.LBB32_214
.Ltmp1506:
	.loc	38 1050 16
	testb	%al, %al
	jne	.LBB32_642
.Ltmp1507:
	.loc	1 1053 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
.Ltmp1508:
	.loc	1 1054 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp1509:
	.loc	1 1055 25
	movl	184(%r14), %eax
	movl	%eax, 8(%rsp)
	movl	188(%r14), %ecx
.Ltmp1510:
	.loc	1 1056 24
	movl	544(%r14), %r12d
	movl	548(%r14), %r15d
.Ltmp1511:
	.loc	1 1057 24
	movq	848(%r14), %rdi
.Ltmp1512:
	.loc	2 1916 50
	cmpq	$0, 48(%rsp)
.Ltmp1513:
	.loc	3 900 12
	je	.LBB32_281
.Ltmp1514:
	.loc	3 0 12 is_stmt 0
	movq	440(%rsp), %rax
	leaq	(%rax,%rbp,4), %rbx
	movq	408(%rsp), %rax
	leaq	(%rax,%rbp,4), %r10
	xorl	%esi, %esi
	movq	448(%rsp), %r8
	movl	%ecx, %ebp
	movq	%r10, 656(%rsp)
	movq	%rbx, 464(%rsp)
	.p2align	4
.LBB32_148:
.Ltmp1515:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp1516:
	.loc	1 857 8
	cmpq	%r8, %rax
	jb	.LBB32_150
.Ltmp1517:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %r13
	jmp	.LBB32_151
	.p2align	4
.LBB32_150:
	xorl	%r13d, %r13d
.LBB32_151:
.Ltmp1518:
	.loc	1 1083 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp1519:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_614
.Ltmp1520:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1521:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rbx,%rsi,4), %xmm0
.Ltmp1522:
	vmovss	(%r10,%rsi,4), %xmm3
.Ltmp1523:
	vmovss	152(%r14), %xmm6
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm4
	vmovaps	240(%rsp), %xmm9
	vsubss	%xmm9, %xmm0, %xmm1
	vmulss	%xmm7, %xmm1, %xmm5
	vmovaps	256(%rsp), %xmm2
	vmovss	%xmm6, 148(%rsp)
	vmulss	%xmm6, %xmm2, %xmm6
	vaddss	%xmm5, %xmm6, %xmm5
	vaddss	%xmm5, %xmm2, %xmm6
	vmulss	%xmm7, %xmm2, %xmm8
	vmulss	%xmm4, %xmm1, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	164(%r14), %xmm6, %xmm15
	vmovss	%xmm1, 320(%rsp)
	vaddss	%xmm1, %xmm9, %xmm6
	vmovaps	192(%rsp), %xmm1
	vsubss	%xmm1, %xmm6, %xmm10
	vmulss	304(%rsp), %xmm7, %xmm6
	vmulss	%xmm4, %xmm10, %xmm4
	vaddss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm1, %xmm6
.Ltmp1524:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm12
	vmovss	520(%r14), %xmm13
	vmovaps	224(%rsp), %xmm1
	vsubss	%xmm1, %xmm3, %xmm14
	vmulss	%xmm12, %xmm14, %xmm8
	vmovaps	208(%rsp), %xmm2
	vmulss	%xmm2, %xmm11, %xmm9
	vaddss	%xmm8, %xmm9, %xmm8
	vaddss	%xmm2, %xmm8, %xmm9
	vmulss	524(%r14), %xmm9, %xmm9
.Ltmp1525:
	.loc	1 1083 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp1526:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp1527:
	.loc	1 1084 30
	movq	144(%r14), %rdx
.Ltmp1528:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_615
.Ltmp1529:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1530:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm0, %xmm15, %xmm0
	vsubss	%xmm6, %xmm0, %xmm0
.Ltmp1531:
	.loc	1 1084 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp1532:
	.loc	49 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp1533:
	.loc	1 1085 28
	movq	488(%r14), %rdx
.Ltmp1534:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_616
.Ltmp1535:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1536:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm2, %xmm12, %xmm0
	vmulss	%xmm13, %xmm14, %xmm6
	vaddss	%xmm6, %xmm0, %xmm14
	vaddss	%xmm1, %xmm14, %xmm0
	vmovaps	176(%rsp), %xmm1
	vsubss	%xmm1, %xmm0, %xmm15
	vmulss	160(%rsp), %xmm12, %xmm0
	vmulss	%xmm15, %xmm13, %xmm6
	vaddss	%xmm6, %xmm0, %xmm0
	vaddss	%xmm0, %xmm1, %xmm6
.Ltmp1537:
	.loc	1 1085 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp1538:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp1539:
	.loc	1 1086 29
	movq	504(%r14), %rdx
.Ltmp1540:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp1541:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1542:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm9, %xmm3
	vsubss	%xmm6, %xmm3, %xmm3
.Ltmp1543:
	.loc	1 1086 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp1544:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r9
.Ltmp1545:
	.loc	1 947 24
	addq	%rdi, %r9
.Ltmp1546:
	.loc	1 857 8
	cmpq	%r8, %r9
	jb	.LBB32_161
.Ltmp1547:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %rax
	jmp	.LBB32_162
	.p2align	4
.LBB32_161:
	xorl	%eax, %eax
.LBB32_162:
	movq	128(%r14), %rdx
.Ltmp1548:
	.loc	1 857 8 is_stmt 1
	subq	%rax, %r9
.Ltmp1549:
	.loc	48 568 12
	cmpq	%rdx, %r9
	ja	.LBB32_622
.Ltmp1550:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1551:
	.loc	1 1096 13
	movq	144(%r14), %rax
.Ltmp1552:
	.loc	48 568 12
	cmpq	%rax, %r9
	ja	.LBB32_623
.Ltmp1553:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1554:
	.loc	49 0 9 is_stmt 0
	movq	760(%r14), %r10
.Ltmp1555:
	.loc	1 947 24 is_stmt 1
	addq	%rdi, %r10
.Ltmp1556:
	.loc	1 857 8
	cmpq	%r8, %r10
	movq	%rsi, 680(%rsp)
	jb	.LBB32_168
.Ltmp1557:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %rcx
	jmp	.LBB32_169
	.p2align	4
.LBB32_168:
	xorl	%ecx, %ecx
.LBB32_169:
	movq	488(%r14), %r11
.Ltmp1558:
	.loc	1 857 8 is_stmt 1
	subq	%rcx, %r10
.Ltmp1559:
	.loc	48 568 12
	cmpq	%r11, %r10
	ja	.LBB32_624
.Ltmp1560:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1561:
	.loc	1 1110 13
	movq	504(%r14), %r8
.Ltmp1562:
	.loc	48 568 12
	cmpq	%r8, %r10
	ja	.LBB32_625
.Ltmp1563:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1564:
	.loc	49 0 9 is_stmt 0
	movq	%r13, %rsi
	movl	%ebp, %ecx
	movl	%r12d, %r13d
	movl	%r15d, 68(%rsp)
	movq	120(%r14), %rbx
	movq	%rbx, 504(%rsp)
	vmovss	(%rbx,%r9,4), %xmm3
.Ltmp1565:
	movq	480(%r14), %rbx
	movq	%rbx, 672(%rsp)
	vmovss	(%rbx,%r10,4), %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1566:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp1567:
	.loc	49 103 24 is_stmt 0
	vandps	%xmm1, %xmm6, %xmm6
.Ltmp1568:
	.loc	49 161 24 is_stmt 1
	vmaxss	%xmm6, %xmm3, %xmm3
.Ltmp1569:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r12d
	movl	$841731191, %ebp
.Ltmp1570:
	.loc	49 161 24
	jbe	.LBB32_175
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %ebp
.LBB32_175:
.Ltmp1571:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebp, %xmm3
.Ltmp1572:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %r15d
	movl	$8388608, %ebx
.Ltmp1573:
	.loc	49 161 24
	jbe	.LBB32_177
.Ltmp1574:
	.loc	49 0 24 is_stmt 0
	movl	%ebp, %ebx
.LBB32_177:
	vmovss	80(%rsp), %xmm1
	vaddss	404(%rsp), %xmm1, %xmm1
.Ltmp1575:
	.loc	49 185 42 is_stmt 1
	movl	%ebx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1576:
	.loc	23 1291 18
	vmovd	%ebp, %xmm3
.Ltmp1577:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp1578:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm6
.Ltmp1579:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm6, %xmm6
.Ltmp1580:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1581:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm6, %xmm6
.Ltmp1582:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1583:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm6, %xmm6
.Ltmp1584:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1585:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm6, %xmm6
.Ltmp1586:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1587:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm6, %xmm6
.Ltmp1588:
	.loc	49 187 28
	shrl	$23, %ebx
	orl	$1258291200, %ebx
.Ltmp1589:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1590:
	.loc	23 1291 18
	vmovd	%ebx, %xmm6
.Ltmp1591:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm6, %xmm6
.Ltmp1592:
	.loc	49 61 9
	vaddss	%xmm3, %xmm6, %xmm3
.Ltmp1593:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp1594:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp1595:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm1, 80(%rsp)
.Ltmp1596:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm3, %xmm6
.Ltmp1597:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm6, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_179
.Ltmp1598:
	.loc	49 0 44
	xorl	%ebx, %ebx
	jmp	.LBB32_180
	.p2align	4
.LBB32_179:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm6, %xmm9
.Ltmp1599:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp1600:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm1, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	144(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %ebx
.Ltmp1601:
.LBB32_180:
	.loc	49 0 0
	movl	%ecx, %ebp
.Ltmp1602:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebx, %xmm6
.Ltmp1603:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp1604:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vminss	%xmm1, %xmm6, %xmm6
.Ltmp1605:
	.loc	1 1005 9 is_stmt 1
	vmovss	8(%rsp), %xmm9
.Ltmp1606:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	512(%rsp), %xmm1
	vblendvps	%xmm13, 624(%rsp), %xmm1, %xmm13
.Ltmp1607:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp1608:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1609:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm9
.Ltmp1610:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm1
	vucomiss	%xmm9, %xmm1
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_182
	.loc	49 0 24
	movl	$0, 8(%rsp)
	jmp	.LBB32_183
	.p2align	4
.LBB32_182:
	.loc	49 166 0
	vmovss	%xmm6, 8(%rsp)
.Ltmp1611:
.LBB32_183:
	.loc	1 0 0
	movq	136(%r14), %rbx
	vmovss	(%rbx,%r9,4), %xmm6
.Ltmp1612:
	movq	496(%r14), %r9
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1613:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm6, %xmm6
.Ltmp1614:
	.loc	49 103 24 is_stmt 0
	vmovss	(%r9,%r10,4), %xmm9
	vandps	%xmm1, %xmm9, %xmm9
.Ltmp1615:
	.loc	49 161 24 is_stmt 1
	vmaxss	%xmm9, %xmm6, %xmm6
.Ltmp1616:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
.Ltmp1617:
	.loc	49 161 24
	jbe	.LBB32_185
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %r12d
.LBB32_185:
.Ltmp1618:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r12d, %xmm6
.Ltmp1619:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
.Ltmp1620:
	.loc	49 161 24
	jbe	.LBB32_187
.Ltmp1621:
	.loc	49 0 24 is_stmt 0
	movl	%r12d, %r15d
.LBB32_187:
	vmovss	16(%rsp), %xmm1
	vaddss	400(%rsp), %xmm1, %xmm1
.Ltmp1622:
	.loc	49 185 42 is_stmt 1
	movl	%r15d, %r10d
	andl	$8388607, %r10d
	orl	$1065353216, %r10d
.Ltmp1623:
	.loc	23 1291 18
	vmovd	%r10d, %xmm6
.Ltmp1624:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp1625:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp1626:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp1627:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1628:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp1629:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1630:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp1631:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1632:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp1633:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp1634:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp1635:
	.loc	49 187 28
	shrl	$23, %r15d
	orl	$1258291200, %r15d
.Ltmp1636:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp1637:
	.loc	23 1291 18
	vmovd	%r15d, %xmm9
.Ltmp1638:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp1639:
	.loc	49 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1640:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp1641:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp1642:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm9
	vmovss	%xmm1, 16(%rsp)
.Ltmp1643:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm9, %xmm6
.Ltmp1644:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm6, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_189
.Ltmp1645:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_190
	.p2align	4
.LBB32_189:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm6, %xmm13
.Ltmp1646:
	vmulss	%xmm13, %xmm13, %xmm13
	vmulss	.LCPI32_18(%rip), %xmm13, %xmm13
.Ltmp1647:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm1, %xmm1
	vblendvps	%xmm1, %xmm13, %xmm6, %xmm1
	vmulss	140(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp1648:
.LBB32_190:
	.loc	49 0 0
	movl	68(%rsp), %ecx
.Ltmp1649:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp1650:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1651:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm6, %xmm6, %xmm6
	vminss	%xmm6, %xmm1, %xmm1
.Ltmp1652:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ebp, %xmm6
.Ltmp1653:
	.loc	49 161 24
	vcmpnltss	%xmm6, %xmm1, %xmm13
	vmovaps	608(%rsp), %xmm2
	vblendvps	%xmm13, 592(%rsp), %xmm2, %xmm13
.Ltmp1654:
	.loc	49 66 9
	vsubss	%xmm1, %xmm6, %xmm6
.Ltmp1655:
	.loc	49 92 9
	vmulss	%xmm6, %xmm13, %xmm6
	vaddss	%xmm6, %xmm1, %xmm6
.Ltmp1656:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm1
.Ltmp1657:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_192
	.loc	49 0 24
	xorl	%ebp, %ebp
	jmp	.LBB32_193
	.p2align	4
.LBB32_192:
	.loc	49 166 0
	vmovd	%xmm6, %ebp
.Ltmp1658:
.LBB32_193:
	.loc	49 0 0
	vmovss	56(%rsp), %xmm1
	vaddss	396(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 56(%rsp)
.Ltmp1659:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1660:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_195
.Ltmp1661:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_196
	.p2align	4
.LBB32_195:
	vmovss	.LCPI32_17(%rip), %xmm6
	vaddss	%xmm6, %xmm3, %xmm1
.Ltmp1662:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1663:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm6, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vmulss	136(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp1664:
.LBB32_196:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp1665:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1666:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1667:
	.loc	1 1005 9 is_stmt 1
	vmovd	%r13d, %xmm3
.Ltmp1668:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm6
	vmovaps	576(%rsp), %xmm2
	vblendvps	%xmm6, 560(%rsp), %xmm2, %xmm6
.Ltmp1669:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1670:
	.loc	49 92 9
	vmulss	%xmm6, %xmm3, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1671:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1672:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm6
	vucomiss	%xmm1, %xmm6
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_198
	.loc	49 0 24
	xorl	%r12d, %r12d
	jmp	.LBB32_199
	.p2align	4
.LBB32_198:
	.loc	49 166 0
	vmovd	%xmm3, %r12d
.Ltmp1673:
.LBB32_199:
	.loc	49 0 0
	vmovss	288(%rsp), %xmm1
	vaddss	392(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 288(%rsp)
.Ltmp1674:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm9, %xmm3
.Ltmp1675:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_201
.Ltmp1676:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_202
	.p2align	4
.LBB32_201:
	vmovss	.LCPI32_17(%rip), %xmm6
	vaddss	%xmm6, %xmm3, %xmm1
.Ltmp1677:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1678:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm6, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vmulss	284(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp1679:
.LBB32_202:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp1680:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1681:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1682:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp1683:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm6
	vmovaps	544(%rsp), %xmm2
	vblendvps	%xmm6, 528(%rsp), %xmm2, %xmm6
.Ltmp1684:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1685:
	.loc	49 92 9
	vmulss	%xmm6, %xmm3, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1686:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1687:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm6
	vucomiss	%xmm1, %xmm6
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_204
	.loc	49 0 24
	xorl	%r15d, %r15d
	jmp	.LBB32_205
	.p2align	4
.LBB32_204:
	.loc	49 166 0
	vmovd	%xmm3, %r15d
.Ltmp1688:
.LBB32_205:
	.loc	1 0 0
	negq	%rsi
	addq	%rsi, %rdi
	incq	%rdi
.Ltmp1689:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp1690:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1691:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_619
.Ltmp1692:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1693:
	.loc	48 568 12
	cmpq	%r11, %rdi
	ja	.LBB32_620
.Ltmp1694:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1695:
	.loc	48 568 12
	cmpq	%r8, %rdi
	movq	656(%rsp), %r10
	movq	680(%rsp), %rcx
	ja	.LBB32_621
.Ltmp1696:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1697:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm1
	vmovaps	304(%rsp), %xmm13
	vmulss	148(%rsp), %xmm13, %xmm2
	vaddss	%xmm1, %xmm2, %xmm1
	vmulss	%xmm15, %xmm12, %xmm2
	vmovaps	160(%rsp), %xmm12
	vmulss	%xmm11, %xmm12, %xmm3
	vaddss	%xmm2, %xmm3, %xmm2
	vaddss	%xmm5, %xmm5, %xmm3
	vaddss	256(%rsp), %xmm3, %xmm7
	vmovss	320(%rsp), %xmm3
	vaddss	%xmm3, %xmm3, %xmm3
	vaddss	240(%rsp), %xmm3, %xmm5
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	%xmm4, %xmm4, %xmm3
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm14, %xmm14, %xmm6
	vaddss	%xmm2, %xmm2, %xmm9
	vaddss	%xmm0, %xmm0, %xmm10
	vaddss	%xmm1, %xmm13, %xmm8
	vaddss	192(%rsp), %xmm3, %xmm0
	vaddss	208(%rsp), %xmm4, %xmm1
	vaddss	224(%rsp), %xmm6, %xmm2
	vaddss	%xmm9, %xmm12, %xmm3
	vaddss	176(%rsp), %xmm10, %xmm4
	vmovss	24(%rsp), %xmm9
	vaddss	376(%rsp), %xmm9, %xmm9
	vmovss	8(%rsp), %xmm6
	vmovss	%xmm9, 24(%rsp)
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
	vmovaps	%xmm7, 256(%rsp)
	vandps	%xmm2, %xmm14, %xmm7
	vcmpnltss	%xmm15, %xmm10, %xmm9
	vandps	%xmm5, %xmm9, %xmm5
	vmovaps	%xmm5, 240(%rsp)
	vandps	%xmm3, %xmm14, %xmm5
	vcmpnltss	%xmm15, %xmm11, %xmm9
	vandps	%xmm8, %xmm9, %xmm8
	vmovaps	%xmm8, 304(%rsp)
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
	vmovaps	%xmm0, 192(%rsp)
	vandps	%xmm1, %xmm10, %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vandps	%xmm2, %xmm7, %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vandps	%xmm3, %xmm5, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vandps	%xmm4, %xmm8, %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm11
	vminss	%xmm11, %xmm6, %xmm0
	vroundss	$9, %xmm0, %xmm0, %xmm2
	vmovss	416(%rsp), %xmm5
	vaddss	344(%rsp), %xmm5, %xmm5
	vmovss	28(%rsp), %xmm1
	vaddss	360(%rsp), %xmm1, %xmm1
	vsubss	%xmm2, %xmm0, %xmm3
	vmovd	%ebp, %xmm0
	vmovss	%xmm1, 28(%rsp)
	vaddss	%xmm0, %xmm1, %xmm0
	vmulss	%xmm0, %xmm14, %xmm0
	vmaxss	%xmm12, %xmm0, %xmm0
	vminss	%xmm11, %xmm0, %xmm1
	vroundss	$9, %xmm1, %xmm1, %xmm7
	vsubss	%xmm7, %xmm1, %xmm1
	vmovd	%r12d, %xmm4
	vmovss	%xmm5, 416(%rsp)
	vaddss	%xmm4, %xmm5, %xmm4
	vmulss	%xmm4, %xmm14, %xmm4
	vmaxss	%xmm12, %xmm4, %xmm4
	vminss	%xmm11, %xmm4, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm6
.Ltmp1698:
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
.Ltmp1699:
	vroundss	$9, %xmm4, %xmm4, %xmm3
	vsubss	%xmm3, %xmm4, %xmm4
	vmovss	72(%rsp), %xmm0
	vaddss	664(%rsp), %xmm0, %xmm0
	vmovss	.LCPI32_29(%rip), %xmm14
.Ltmp1700:
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
.Ltmp1701:
	vmulss	%xmm5, %xmm1, %xmm1
.Ltmp1702:
	vmovd	%r15d, %xmm5
	vmovss	%xmm0, 72(%rsp)
	vaddss	%xmm5, %xmm0, %xmm5
	vmulss	.LCPI32_21(%rip), %xmm5, %xmm5
	vmaxss	%xmm12, %xmm5, %xmm5
	vminss	%xmm11, %xmm5, %xmm5
.Ltmp1703:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm7, %xmm15, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp1704:
	vmulss	%xmm6, %xmm4, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm9, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm10, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp1705:
	vmulss	%xmm1, %xmm4, %xmm1
.Ltmp1706:
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
.Ltmp1707:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm3, %xmm15, %xmm3
	vmovd	%xmm3, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm3
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp1708:
	vmulss	%xmm6, %xmm5, %xmm3
	vaddss	%xmm3, %xmm8, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm9, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp1709:
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm4, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	504(%rsp), %rax
.Ltmp1710:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm2
	vmulss	(%rbx,%rdi,4), %xmm0, %xmm0
.Ltmp1711:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp1712:
	vaddss	%xmm0, %xmm2, %xmm0
	movq	672(%rsp), %rax
.Ltmp1713:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
.Ltmp1714:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r9,%rdi,4), %xmm3, %xmm2
.Ltmp1715:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	464(%rsp), %rbx
.Ltmp1716:
	.loc	49 56 9
	vmovss	%xmm0, (%rbx,%rcx,4)
.Ltmp1717:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r10,%rcx,4)
	vmovss	32(%rsp), %xmm0
.Ltmp1718:
	.loc	1 0 0
	vaddss	388(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	104(%rsp), %xmm0
	vaddss	356(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	36(%rsp), %xmm0
	vaddss	384(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	112(%rsp), %xmm0
	vaddss	352(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	40(%rsp), %xmm0
	vaddss	380(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	120(%rsp), %xmm0
	vaddss	348(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	44(%rsp), %xmm0
	vaddss	372(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	12(%rsp), %xmm0
	vaddss	340(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	88(%rsp), %xmm0
	vaddss	368(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	20(%rsp), %xmm0
	vaddss	336(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	96(%rsp), %xmm0
	vaddss	364(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	128(%rsp), %xmm0
	vaddss	332(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 128(%rsp)
	leaq	1(%rcx), %rsi
.Ltmp1719:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rsi, 48(%rsp)
	movq	448(%rsp), %r8
.Ltmp1720:
	.loc	3 900 12
	jne	.LBB32_148
	jmp	.LBB32_282
.Ltmp1721:
	.loc	3 0 12 is_stmt 0
.Ltmp1722:
	.p2align	4
.LBB32_214:
	.loc	38 1050 16 is_stmt 1
	testb	%al, %al
	jne	.LBB32_643
.Ltmp1723:
	.loc	1 1053 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
.Ltmp1724:
	.loc	1 1054 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp1725:
	.loc	1 1055 25
	movl	184(%r14), %eax
	movl	%eax, 8(%rsp)
	movl	188(%r14), %ecx
.Ltmp1726:
	.loc	1 1056 24
	movl	544(%r14), %r13d
	movl	548(%r14), %ebx
.Ltmp1727:
	.loc	1 1057 24
	movq	848(%r14), %rdi
.Ltmp1728:
	.loc	2 1916 50
	cmpq	$0, 48(%rsp)
.Ltmp1729:
	.loc	3 900 12
	je	.LBB32_141
.Ltmp1730:
	.loc	3 0 12 is_stmt 0
	movq	440(%rsp), %rax
	leaq	(%rax,%rbp,4), %rsi
	movq	408(%rsp), %rax
	leaq	(%rax,%rbp,4), %r10
	xorl	%r12d, %r12d
	movq	448(%rsp), %r8
	movl	%ecx, %ebp
	movq	%r10, 96(%rsp)
	movq	%rsi, 88(%rsp)
	.p2align	4
.LBB32_217:
.Ltmp1731:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp1732:
	.loc	1 857 8
	cmpq	%r8, %rax
	jb	.LBB32_219
.Ltmp1733:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %r15
	jmp	.LBB32_220
	.p2align	4
.LBB32_219:
	xorl	%r15d, %r15d
.LBB32_220:
.Ltmp1734:
	.loc	1 1083 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp1735:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_614
.Ltmp1736:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1737:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rsi,%r12,4), %xmm6
.Ltmp1738:
	vmovss	(%r10,%r12,4), %xmm13
.Ltmp1739:
	vmovss	152(%r14), %xmm4
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm0
	vmovaps	240(%rsp), %xmm8
	vsubss	%xmm8, %xmm6, %xmm1
	vmulss	%xmm7, %xmm1, %xmm3
	vmovaps	256(%rsp), %xmm2
	vmovss	%xmm4, 20(%rsp)
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm3, %xmm4, %xmm5
	vaddss	%xmm5, %xmm2, %xmm3
	vmulss	%xmm7, %xmm2, %xmm4
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	%xmm1, %xmm4, %xmm1
	vmulss	164(%r14), %xmm3, %xmm3
	vmovss	%xmm1, 128(%rsp)
	vaddss	%xmm1, %xmm8, %xmm4
	vmovaps	192(%rsp), %xmm1
	vsubss	%xmm1, %xmm4, %xmm10
	vmulss	304(%rsp), %xmm7, %xmm4
	vmulss	%xmm0, %xmm10, %xmm0
	vaddss	%xmm0, %xmm4, %xmm4
	vaddss	%xmm4, %xmm1, %xmm14
.Ltmp1740:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm0
	vmovss	520(%r14), %xmm12
	vmovaps	224(%rsp), %xmm1
	vsubss	%xmm1, %xmm13, %xmm9
	vmulss	%xmm0, %xmm9, %xmm8
	vmovaps	208(%rsp), %xmm2
	vmulss	%xmm2, %xmm11, %xmm15
	vaddss	%xmm8, %xmm15, %xmm8
	vaddss	%xmm2, %xmm8, %xmm15
	vmulss	524(%r14), %xmm15, %xmm15
.Ltmp1741:
	.loc	1 1083 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp1742:
	.loc	49 56 9
	vmovss	%xmm14, (%rax,%rdi,4)
.Ltmp1743:
	.loc	1 1084 30
	movq	144(%r14), %rdx
.Ltmp1744:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_615
.Ltmp1745:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1746:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm6, %xmm3
	vsubss	%xmm14, %xmm3, %xmm3
.Ltmp1747:
	.loc	1 1084 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp1748:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp1749:
	.loc	1 1085 28
	movq	488(%r14), %rdx
.Ltmp1750:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_616
.Ltmp1751:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1752:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm0, %xmm2, %xmm3
	vmulss	%xmm12, %xmm9, %xmm6
	vaddss	%xmm6, %xmm3, %xmm6
	vaddss	%xmm6, %xmm1, %xmm3
	vmovaps	176(%rsp), %xmm1
	vsubss	%xmm1, %xmm3, %xmm9
	vmulss	160(%rsp), %xmm0, %xmm3
	vmulss	%xmm9, %xmm12, %xmm12
	vaddss	%xmm3, %xmm12, %xmm12
	vaddss	%xmm1, %xmm12, %xmm3
.Ltmp1753:
	.loc	1 1085 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp1754:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp1755:
	.loc	1 1086 29
	movq	504(%r14), %rdx
.Ltmp1756:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp1757:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp1758:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm15, %xmm13, %xmm13
	vsubss	%xmm3, %xmm13, %xmm3
.Ltmp1759:
	.loc	1 1086 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp1760:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r9
.Ltmp1761:
	.loc	1 947 24
	addq	%rdi, %r9
.Ltmp1762:
	.loc	1 857 8
	cmpq	%r8, %r9
	jb	.LBB32_230
.Ltmp1763:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %rax
	jmp	.LBB32_231
	.p2align	4
.LBB32_230:
	xorl	%eax, %eax
.LBB32_231:
	movq	128(%r14), %rdx
.Ltmp1764:
	.loc	1 857 8 is_stmt 1
	subq	%rax, %r9
.Ltmp1765:
	.loc	48 568 12
	cmpq	%rdx, %r9
	ja	.LBB32_622
.Ltmp1766:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1767:
	.loc	1 1096 13
	movq	144(%r14), %rax
.Ltmp1768:
	.loc	48 568 12
	cmpq	%rax, %r9
	ja	.LBB32_623
.Ltmp1769:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1770:
	.loc	49 0 9 is_stmt 0
	movq	760(%r14), %r10
.Ltmp1771:
	.loc	1 947 24 is_stmt 1
	addq	%rdi, %r10
.Ltmp1772:
	.loc	1 857 8
	cmpq	%r8, %r10
	movq	%r12, 120(%rsp)
	jb	.LBB32_237
.Ltmp1773:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %rcx
	jmp	.LBB32_238
	.p2align	4
.LBB32_237:
	xorl	%ecx, %ecx
.LBB32_238:
	movq	488(%r14), %r11
.Ltmp1774:
	.loc	1 857 8 is_stmt 1
	subq	%rcx, %r10
.Ltmp1775:
	.loc	48 568 12
	cmpq	%r11, %r10
	ja	.LBB32_624
.Ltmp1776:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1777:
	.loc	1 1110 13
	movq	504(%r14), %r8
.Ltmp1778:
	.loc	48 568 12
	cmpq	%r8, %r10
	ja	.LBB32_625
.Ltmp1779:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1780:
	.loc	49 0 9 is_stmt 0
	movq	%r15, %r12
	movl	%ebp, %ecx
	movl	%r13d, %r15d
	movl	%ebx, 12(%rsp)
	movq	120(%r14), %rsi
	movq	%rsi, 104(%rsp)
	vmovss	(%rsi,%r9,4), %xmm3
.Ltmp1781:
	movq	480(%r14), %rsi
	movq	%rsi, 112(%rsp)
	vmovss	(%rsi,%r10,4), %xmm13
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1782:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp1783:
	.loc	49 103 24 is_stmt 0
	vandps	%xmm1, %xmm13, %xmm13
.Ltmp1784:
	.loc	49 161 24 is_stmt 1
	vmaxss	%xmm13, %xmm3, %xmm3
.Ltmp1785:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r13d
	movl	$841731191, %ebp
.Ltmp1786:
	.loc	49 161 24
	jbe	.LBB32_244
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %ebp
.LBB32_244:
.Ltmp1787:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebp, %xmm3
.Ltmp1788:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %ebx
	movl	$8388608, %esi
.Ltmp1789:
	.loc	49 161 24
	jbe	.LBB32_246
.Ltmp1790:
	.loc	49 0 24 is_stmt 0
	movl	%ebp, %esi
.LBB32_246:
.Ltmp1791:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1792:
	.loc	23 1291 18
	vmovd	%ebp, %xmm3
.Ltmp1793:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp1794:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm13
.Ltmp1795:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm13, %xmm13
.Ltmp1796:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp1797:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm13, %xmm13
.Ltmp1798:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp1799:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm13, %xmm13
.Ltmp1800:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp1801:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm13, %xmm13
.Ltmp1802:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp1803:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm13, %xmm13
.Ltmp1804:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp1805:
	.loc	23 1291 18
	vmovd	%esi, %xmm14
.Ltmp1806:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm14, %xmm14
.Ltmp1807:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm3
.Ltmp1808:
	.loc	49 61 9
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp1809:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp1810:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp1811:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm13
.Ltmp1812:
	.loc	49 66 9 is_stmt 1
	vsubss	80(%rsp), %xmm13, %xmm3
.Ltmp1813:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_248
.Ltmp1814:
	.loc	49 0 44
	xorl	%esi, %esi
	jmp	.LBB32_249
	.p2align	4
.LBB32_248:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm3, %xmm14
.Ltmp1815:
	vmulss	%xmm14, %xmm14, %xmm14
	vmulss	.LCPI32_18(%rip), %xmm14, %xmm14
.Ltmp1816:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm1, %xmm15
	vblendvps	%xmm15, %xmm14, %xmm3, %xmm3
	vmulss	144(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %esi
.Ltmp1817:
.LBB32_249:
	.loc	49 0 0
	movl	%ecx, %ebp
.Ltmp1818:
	.loc	23 1291 18 is_stmt 1
	vmovd	%esi, %xmm3
.Ltmp1819:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp1820:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vminss	%xmm1, %xmm3, %xmm3
.Ltmp1821:
	.loc	1 1005 9 is_stmt 1
	vmovss	8(%rsp), %xmm14
.Ltmp1822:
	.loc	49 161 24
	vcmpnltss	%xmm14, %xmm3, %xmm15
	vmovaps	512(%rsp), %xmm1
	vblendvps	%xmm15, 624(%rsp), %xmm1, %xmm15
.Ltmp1823:
	.loc	49 66 9
	vsubss	%xmm3, %xmm14, %xmm14
.Ltmp1824:
	.loc	49 92 9
	vmulss	%xmm15, %xmm14, %xmm14
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp1825:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm14
.Ltmp1826:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm1
	vucomiss	%xmm14, %xmm1
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_251
	.loc	49 0 24
	movl	$0, 8(%rsp)
	jmp	.LBB32_252
	.p2align	4
.LBB32_251:
	.loc	49 166 0
	vmovss	%xmm3, 8(%rsp)
.Ltmp1827:
.LBB32_252:
	.loc	1 0 0
	movq	136(%r14), %rsi
	vmovss	(%rsi,%r9,4), %xmm3
.Ltmp1828:
	movq	496(%r14), %r9
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp1829:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp1830:
	.loc	49 103 24 is_stmt 0
	vmovss	(%r9,%r10,4), %xmm14
	vandps	%xmm1, %xmm14, %xmm14
.Ltmp1831:
	.loc	49 161 24 is_stmt 1
	vmaxss	%xmm14, %xmm3, %xmm3
.Ltmp1832:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
.Ltmp1833:
	.loc	49 161 24
	jbe	.LBB32_254
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %r13d
.LBB32_254:
.Ltmp1834:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r13d, %xmm3
.Ltmp1835:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
.Ltmp1836:
	.loc	49 161 24
	jbe	.LBB32_256
.Ltmp1837:
	.loc	49 0 24 is_stmt 0
	movl	%r13d, %ebx
.LBB32_256:
.Ltmp1838:
	.loc	49 185 42 is_stmt 1
	movl	%ebx, %r10d
	andl	$8388607, %r10d
	orl	$1065353216, %r10d
.Ltmp1839:
	.loc	23 1291 18
	vmovd	%r10d, %xmm3
.Ltmp1840:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp1841:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm14
.Ltmp1842:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm14, %xmm14
.Ltmp1843:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp1844:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm14, %xmm14
.Ltmp1845:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp1846:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm14, %xmm14
.Ltmp1847:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp1848:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm14, %xmm14
.Ltmp1849:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp1850:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm14, %xmm14
.Ltmp1851:
	.loc	49 187 28
	shrl	$23, %ebx
	orl	$1258291200, %ebx
.Ltmp1852:
	.loc	23 1291 18
	vmovd	%ebx, %xmm15
.Ltmp1853:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm15, %xmm15
.Ltmp1854:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm3
.Ltmp1855:
	.loc	49 61 9
	vaddss	%xmm3, %xmm15, %xmm3
.Ltmp1856:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp1857:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp1858:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm15
.Ltmp1859:
	.loc	49 66 9 is_stmt 1
	vsubss	16(%rsp), %xmm15, %xmm3
.Ltmp1860:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_258
.Ltmp1861:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_259
	.p2align	4
.LBB32_258:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm3, %xmm14
.Ltmp1862:
	vmulss	%xmm14, %xmm14, %xmm14
	vmulss	.LCPI32_18(%rip), %xmm14, %xmm14
.Ltmp1863:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm1, %xmm1
	vblendvps	%xmm1, %xmm14, %xmm3, %xmm1
	vmulss	140(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp1864:
.LBB32_259:
	.loc	49 0 0
	movl	12(%rsp), %ecx
.Ltmp1865:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp1866:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1867:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1868:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ebp, %xmm3
.Ltmp1869:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm14
	vmovaps	608(%rsp), %xmm2
	vblendvps	%xmm14, 592(%rsp), %xmm2, %xmm14
.Ltmp1870:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1871:
	.loc	49 92 9
	vmulss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1872:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1873:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm1, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_262
.Ltmp1874:
	.loc	49 0 24
	xorl	%ebp, %ebp
.Ltmp1875:
	.loc	49 66 9 is_stmt 1
	vsubss	56(%rsp), %xmm13, %xmm3
.Ltmp1876:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jae	.LBB32_263
.Ltmp1877:
.LBB32_261:
	.loc	49 0 44
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm1
.Ltmp1878:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1879:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm1, %xmm3, %xmm1
	vmulss	136(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp1880:
	.loc	49 161 44
	jmp	.LBB32_264
.Ltmp1881:
	.loc	49 0 44
.Ltmp1882:
	.p2align	4
.LBB32_262:
	.loc	49 166 0 is_stmt 1
	vmovd	%xmm3, %ebp
.Ltmp1883:
	.loc	49 66 9
	vsubss	56(%rsp), %xmm13, %xmm3
.Ltmp1884:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_261
.LBB32_263:
	.loc	49 0 44
	xorl	%r10d, %r10d
.LBB32_264:
.Ltmp1885:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp1886:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1887:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1888:
	.loc	1 1005 9 is_stmt 1
	vmovd	%r15d, %xmm3
.Ltmp1889:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm13
	vmovaps	576(%rsp), %xmm2
	vblendvps	%xmm13, 560(%rsp), %xmm2, %xmm13
.Ltmp1890:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1891:
	.loc	49 92 9
	vmulss	%xmm3, %xmm13, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1892:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1893:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_267
.Ltmp1894:
	.loc	49 0 24
	xorl	%r13d, %r13d
.Ltmp1895:
	.loc	49 66 9 is_stmt 1
	vsubss	288(%rsp), %xmm15, %xmm3
.Ltmp1896:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jae	.LBB32_268
.Ltmp1897:
.LBB32_266:
	.loc	49 0 44
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm1
.Ltmp1898:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp1899:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm1, %xmm3, %xmm1
	vmulss	284(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp1900:
	.loc	49 161 44
	jmp	.LBB32_269
.Ltmp1901:
	.loc	49 0 44
.Ltmp1902:
	.p2align	4
.LBB32_267:
	.loc	49 166 0 is_stmt 1
	vmovd	%xmm3, %r13d
.Ltmp1903:
	.loc	49 66 9
	vsubss	288(%rsp), %xmm15, %xmm3
.Ltmp1904:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_266
.LBB32_268:
	.loc	49 0 44
	xorl	%r10d, %r10d
.LBB32_269:
.Ltmp1905:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp1906:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp1907:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp1908:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp1909:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm13
	vmovaps	544(%rsp), %xmm2
	vblendvps	%xmm13, 528(%rsp), %xmm2, %xmm13
.Ltmp1910:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp1911:
	.loc	49 92 9
	vmulss	%xmm3, %xmm13, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp1912:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp1913:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_271
	.loc	49 0 24
	xorl	%ebx, %ebx
	jmp	.LBB32_272
	.p2align	4
.LBB32_271:
	.loc	49 166 0
	vmovd	%xmm3, %ebx
.Ltmp1914:
.LBB32_272:
	.loc	1 0 0
	negq	%r12
	addq	%r12, %rdi
	incq	%rdi
.Ltmp1915:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp1916:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1917:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_619
.Ltmp1918:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1919:
	.loc	48 568 12
	cmpq	%r11, %rdi
	ja	.LBB32_620
.Ltmp1920:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1921:
	.loc	48 568 12
	cmpq	%r8, %rdi
	movq	96(%rsp), %r10
	movq	120(%rsp), %rcx
	ja	.LBB32_621
.Ltmp1922:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp1923:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm1
	vmovaps	304(%rsp), %xmm10
	vmulss	20(%rsp), %xmm10, %xmm2
	vaddss	%xmm1, %xmm2, %xmm1
	vmulss	%xmm0, %xmm9, %xmm0
	vmovaps	160(%rsp), %xmm13
	vmulss	%xmm11, %xmm13, %xmm2
	vaddss	%xmm0, %xmm2, %xmm0
	vaddss	%xmm5, %xmm5, %xmm2
	vaddss	256(%rsp), %xmm2, %xmm3
	vmovss	128(%rsp), %xmm2
	vaddss	%xmm2, %xmm2, %xmm2
	vaddss	240(%rsp), %xmm2, %xmm7
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	%xmm4, %xmm4, %xmm2
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm6, %xmm6, %xmm5
	vaddss	%xmm0, %xmm0, %xmm6
	vaddss	%xmm12, %xmm12, %xmm8
	vaddss	%xmm1, %xmm10, %xmm9
	vaddss	192(%rsp), %xmm2, %xmm0
	vaddss	208(%rsp), %xmm4, %xmm1
	vaddss	224(%rsp), %xmm5, %xmm2
	vaddss	%xmm6, %xmm13, %xmm4
	vaddss	176(%rsp), %xmm8, %xmm5
	vmovss	8(%rsp), %xmm6
	vaddss	24(%rsp), %xmm6, %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm14
	vandps	%xmm3, %xmm14, %xmm8
	vandps	%xmm7, %xmm14, %xmm10
	vandps	%xmm14, %xmm9, %xmm11
	vandps	%xmm0, %xmm14, %xmm12
	vandps	%xmm1, %xmm14, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm15
	vcmpnltss	%xmm15, %xmm8, %xmm8
	vandps	%xmm3, %xmm8, %xmm3
	vmovaps	%xmm3, 256(%rsp)
	vandps	%xmm2, %xmm14, %xmm3
	vcmpnltss	%xmm15, %xmm10, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovaps	%xmm7, 240(%rsp)
	vandps	%xmm4, %xmm14, %xmm7
	vcmpnltss	%xmm15, %xmm11, %xmm8
	vandps	%xmm9, %xmm8, %xmm8
	vmovaps	%xmm8, 304(%rsp)
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
	vmovaps	%xmm0, 192(%rsp)
	vandps	%xmm1, %xmm10, %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vandps	%xmm2, %xmm3, %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vandps	%xmm4, %xmm7, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vandps	%xmm5, %xmm8, %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm7
	vminss	%xmm7, %xmm6, %xmm0
	vroundss	$9, %xmm0, %xmm0, %xmm2
	vsubss	%xmm2, %xmm0, %xmm4
	vmovd	%ebp, %xmm0
	vaddss	28(%rsp), %xmm0, %xmm0
	vmulss	%xmm0, %xmm11, %xmm0
	vmaxss	%xmm12, %xmm0, %xmm0
	vminss	%xmm7, %xmm0, %xmm1
	vroundss	$9, %xmm1, %xmm1, %xmm0
	vsubss	%xmm0, %xmm1, %xmm1
	vmovd	%r13d, %xmm3
	vaddss	416(%rsp), %xmm3, %xmm3
	vmulss	%xmm3, %xmm11, %xmm3
	vmaxss	%xmm12, %xmm3, %xmm3
	vminss	%xmm7, %xmm3, %xmm5
	vmovss	.LCPI32_24(%rip), %xmm6
.Ltmp1924:
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
.Ltmp1925:
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
	vmovss	.LCPI32_29(%rip), %xmm14
.Ltmp1926:
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
.Ltmp1927:
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp1928:
	vmovd	%ebx, %xmm3
	vaddss	72(%rsp), %xmm3, %xmm3
	vmulss	%xmm3, %xmm11, %xmm3
	vmaxss	%xmm12, %xmm3, %xmm3
	vminss	%xmm7, %xmm3, %xmm3
.Ltmp1929:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm0, %xmm15, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp1930:
	vmulss	%xmm6, %xmm5, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm9, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm10, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp1931:
	vmulss	%xmm1, %xmm5, %xmm1
.Ltmp1932:
	vroundss	$9, %xmm3, %xmm3, %xmm5
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp1933:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm4, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	vmulss	%xmm4, %xmm1, %xmm1
.Ltmp1934:
	vmulss	%xmm6, %xmm3, %xmm4
	vaddss	%xmm4, %xmm8, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm9, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm10, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp1935:
	vmulss	%xmm4, %xmm3, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm5, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	104(%rsp), %rax
.Ltmp1936:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm2
	vmulss	(%rsi,%rdi,4), %xmm0, %xmm0
.Ltmp1937:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp1938:
	vaddss	%xmm0, %xmm2, %xmm0
	movq	112(%rsp), %rax
.Ltmp1939:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
.Ltmp1940:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r9,%rdi,4), %xmm3, %xmm2
.Ltmp1941:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	88(%rsp), %rsi
.Ltmp1942:
	.loc	49 56 9
	vmovss	%xmm0, (%rsi,%rcx,4)
.Ltmp1943:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r10,%rcx,4)
.Ltmp1944:
	.loc	1 0 0
	leaq	1(%rcx), %r12
.Ltmp1945:
	.loc	2 1916 50 is_stmt 1
	cmpq	%r12, 48(%rsp)
	movq	448(%rsp), %r8
.Ltmp1946:
	.loc	3 900 12
	jne	.LBB32_217
	jmp	.LBB32_142
.Ltmp1947:
.LBB32_281:
	.loc	3 0 12 is_stmt 0
	movl	%ecx, %ebp
.LBB32_282:
	vmovaps	256(%rsp), %xmm0
.Ltmp1948:
	.loc	1 1160 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	240(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	304(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	208(%rsp), %xmm0
	.loc	1 1161 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	224(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	176(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	8(%rsp), %eax
	.loc	1 1162 5
	movl	%eax, 184(%r14)
	movl	%ebp, 188(%r14)
	.loc	1 1163 5
	movl	%r12d, 544(%r14)
	movl	%r15d, 548(%r14)
	.loc	1 1164 5
	movq	%rdi, 848(%r14)
	vmovss	80(%rsp), %xmm0
.Ltmp1949:
	.loc	1 1300 17
	vmovss	%xmm0, 192(%r14)
	.loc	1 1301 34
	movl	204(%r14), %eax
	movq	48(%rsp), %rsi
.Ltmp1950:
	.loc	38 2472 13
	subl	%esi, %eax
	movl	$0, %edx
	cmovbl	%edx, %eax
.Ltmp1951:
	.loc	1 1301 34
	movl	220(%r14), %ecx
	.loc	1 1301 17 is_stmt 0
	movl	%eax, 204(%r14)
	vmovss	32(%rsp), %xmm0
	.loc	1 1300 17 is_stmt 1
	vmovss	%xmm0, 208(%r14)
.Ltmp1952:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp1953:
	.loc	1 1301 17
	movl	%ecx, 220(%r14)
	vmovss	36(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 224(%r14)
	.loc	1 1301 34
	movl	236(%r14), %eax
.Ltmp1954:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1955:
	.loc	1 1301 34
	movl	252(%r14), %ecx
	.loc	1 1301 17 is_stmt 0
	movl	%eax, 236(%r14)
	vmovss	40(%rsp), %xmm0
	.loc	1 1300 17 is_stmt 1
	vmovss	%xmm0, 240(%r14)
.Ltmp1956:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp1957:
	.loc	1 1301 17
	movl	%ecx, 252(%r14)
	vmovss	24(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 256(%r14)
	.loc	1 1301 34
	movl	268(%r14), %eax
.Ltmp1958:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1959:
	.loc	1 1301 17
	movl	%eax, 268(%r14)
	vmovss	16(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 272(%r14)
	.loc	1 1301 34
	movl	284(%r14), %eax
.Ltmp1960:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1961:
	.loc	1 1301 17
	movl	%eax, 284(%r14)
	vmovss	44(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 288(%r14)
	.loc	1 1301 34
	movl	300(%r14), %eax
.Ltmp1962:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1963:
	.loc	1 1301 17
	movl	%eax, 300(%r14)
	vmovss	88(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 304(%r14)
	.loc	1 1301 34
	movl	316(%r14), %eax
.Ltmp1964:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1965:
	.loc	1 1301 17
	movl	%eax, 316(%r14)
	vmovss	96(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 320(%r14)
	.loc	1 1301 34
	movl	332(%r14), %eax
.Ltmp1966:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1967:
	.loc	1 1301 17
	movl	%eax, 332(%r14)
	vmovss	28(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 336(%r14)
	.loc	1 1301 34
	movl	348(%r14), %eax
.Ltmp1968:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1969:
	.loc	1 1301 17
	movl	%eax, 348(%r14)
	vmovss	56(%rsp), %xmm0
.Ltmp1970:
	.loc	1 1300 17
	vmovss	%xmm0, 552(%r14)
	.loc	1 1301 34
	movl	564(%r14), %eax
.Ltmp1971:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1972:
	.loc	1 1301 17
	movl	%eax, 564(%r14)
	vmovss	104(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 568(%r14)
	.loc	1 1301 34
	movl	580(%r14), %eax
.Ltmp1973:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1974:
	.loc	1 1301 17
	movl	%eax, 580(%r14)
	vmovss	112(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 584(%r14)
	.loc	1 1301 34
	movl	596(%r14), %eax
.Ltmp1975:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1976:
	.loc	1 1301 17
	movl	%eax, 596(%r14)
	vmovss	120(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 600(%r14)
	.loc	1 1301 34
	movl	612(%r14), %eax
.Ltmp1977:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1978:
	.loc	1 1301 17
	movl	%eax, 612(%r14)
	vmovss	416(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 616(%r14)
	.loc	1 1301 34
	movl	628(%r14), %eax
.Ltmp1979:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1980:
	.loc	1 1301 17
	movl	%eax, 628(%r14)
	vmovss	288(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 632(%r14)
	.loc	1 1301 34
	movl	644(%r14), %eax
.Ltmp1981:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1982:
	.loc	1 1301 17
	movl	%eax, 644(%r14)
	vmovss	12(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 648(%r14)
	.loc	1 1301 34
	movl	660(%r14), %eax
.Ltmp1983:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1984:
	.loc	1 1301 17
	movl	%eax, 660(%r14)
	vmovss	20(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 664(%r14)
	.loc	1 1301 34
	movl	676(%r14), %eax
.Ltmp1985:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1986:
	.loc	1 1301 17
	movl	%eax, 676(%r14)
	vmovss	128(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 680(%r14)
	.loc	1 1301 34
	movl	692(%r14), %eax
.Ltmp1987:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1988:
	.loc	1 1301 17
	movl	%eax, 692(%r14)
	vmovss	72(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 696(%r14)
	.loc	1 1301 34
	movl	708(%r14), %eax
.Ltmp1989:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp1990:
	.loc	1 1301 17
	movl	%eax, 708(%r14)
.Ltmp1991:
	.loc	1 1197 9
	jmp	.LBB32_143
.Ltmp1992:
.LBB32_283:
	.loc	1 0 9 is_stmt 0
	movl	%ecx, %ebp
.LBB32_284:
	vmovaps	256(%rsp), %xmm0
.Ltmp1993:
	.loc	1 1160 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	240(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	304(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	208(%rsp), %xmm0
	.loc	1 1161 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	224(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	176(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	8(%rsp), %eax
	.loc	1 1162 5
	movl	%eax, 184(%r14)
	movl	%ebp, 188(%r14)
	.loc	1 1163 5
	movl	%r12d, 544(%r14)
	movl	%r15d, 548(%r14)
	.loc	1 1164 5
	movq	%rdi, 848(%r14)
	vmovss	80(%rsp), %xmm0
.Ltmp1994:
	.loc	1 1300 17
	vmovss	%xmm0, 192(%r14)
	.loc	1 1301 34
	movl	204(%r14), %eax
	movq	48(%rsp), %rsi
.Ltmp1995:
	.loc	38 2472 13
	subl	%esi, %eax
	movl	$0, %edx
	cmovbl	%edx, %eax
.Ltmp1996:
	.loc	1 1301 34
	movl	220(%r14), %ecx
	.loc	1 1301 17 is_stmt 0
	movl	%eax, 204(%r14)
	vmovss	32(%rsp), %xmm0
	.loc	1 1300 17 is_stmt 1
	vmovss	%xmm0, 208(%r14)
.Ltmp1997:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp1998:
	.loc	1 1301 17
	movl	%ecx, 220(%r14)
	vmovss	36(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 224(%r14)
	.loc	1 1301 34
	movl	236(%r14), %eax
.Ltmp1999:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2000:
	.loc	1 1301 34
	movl	252(%r14), %ecx
	.loc	1 1301 17 is_stmt 0
	movl	%eax, 236(%r14)
	vmovss	40(%rsp), %xmm0
	.loc	1 1300 17 is_stmt 1
	vmovss	%xmm0, 240(%r14)
.Ltmp2001:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp2002:
	.loc	1 1301 17
	movl	%ecx, 252(%r14)
	vmovss	24(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 256(%r14)
	.loc	1 1301 34
	movl	268(%r14), %eax
.Ltmp2003:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2004:
	.loc	1 1301 17
	movl	%eax, 268(%r14)
	vmovss	16(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 272(%r14)
	.loc	1 1301 34
	movl	284(%r14), %eax
.Ltmp2005:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2006:
	.loc	1 1301 17
	movl	%eax, 284(%r14)
	vmovss	44(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 288(%r14)
	.loc	1 1301 34
	movl	300(%r14), %eax
.Ltmp2007:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2008:
	.loc	1 1301 17
	movl	%eax, 300(%r14)
	vmovss	88(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 304(%r14)
	.loc	1 1301 34
	movl	316(%r14), %eax
.Ltmp2009:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2010:
	.loc	1 1301 17
	movl	%eax, 316(%r14)
	vmovss	96(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 320(%r14)
	.loc	1 1301 34
	movl	332(%r14), %eax
.Ltmp2011:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2012:
	.loc	1 1301 17
	movl	%eax, 332(%r14)
	vmovss	28(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 336(%r14)
	.loc	1 1301 34
	movl	348(%r14), %eax
.Ltmp2013:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2014:
	.loc	1 1301 17
	movl	%eax, 348(%r14)
	vmovss	56(%rsp), %xmm0
.Ltmp2015:
	.loc	1 1300 17
	vmovss	%xmm0, 552(%r14)
	.loc	1 1301 34
	movl	564(%r14), %eax
.Ltmp2016:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2017:
	.loc	1 1301 17
	movl	%eax, 564(%r14)
	vmovss	104(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 568(%r14)
	.loc	1 1301 34
	movl	580(%r14), %eax
.Ltmp2018:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2019:
	.loc	1 1301 17
	movl	%eax, 580(%r14)
	vmovss	112(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 584(%r14)
	.loc	1 1301 34
	movl	596(%r14), %eax
.Ltmp2020:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2021:
	.loc	1 1301 17
	movl	%eax, 596(%r14)
	vmovss	120(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 600(%r14)
	.loc	1 1301 34
	movl	612(%r14), %eax
.Ltmp2022:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2023:
	.loc	1 1301 17
	movl	%eax, 612(%r14)
	vmovss	416(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 616(%r14)
	.loc	1 1301 34
	movl	628(%r14), %eax
.Ltmp2024:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2025:
	.loc	1 1301 17
	movl	%eax, 628(%r14)
	vmovss	288(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 632(%r14)
	.loc	1 1301 34
	movl	644(%r14), %eax
.Ltmp2026:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2027:
	.loc	1 1301 17
	movl	%eax, 644(%r14)
	vmovss	12(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 648(%r14)
	.loc	1 1301 34
	movl	660(%r14), %eax
.Ltmp2028:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2029:
	.loc	1 1301 17
	movl	%eax, 660(%r14)
	vmovss	20(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 664(%r14)
	.loc	1 1301 34
	movl	676(%r14), %eax
.Ltmp2030:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2031:
	.loc	1 1301 17
	movl	%eax, 676(%r14)
	vmovss	128(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 680(%r14)
	.loc	1 1301 34
	movl	692(%r14), %eax
.Ltmp2032:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2033:
	.loc	1 1301 17
	movl	%eax, 692(%r14)
	vmovss	72(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 696(%r14)
	.loc	1 1301 34
	movl	708(%r14), %eax
.Ltmp2034:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp2035:
	.loc	1 1301 17
	movl	%eax, 708(%r14)
.Ltmp2036:
.LBB32_285:
	.loc	1 0 17 is_stmt 0
	movq	648(%rsp), %rax
	movq	%rax, %rbp
	movq	152(%rsp), %r9
	.loc	1 1189 11 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB32_584
.LBB32_286:
	.loc	1 1190 42
	movq	%r9, %rsi
	subq	%rbp, %rsi
	.loc	1 1190 29 is_stmt 0
	leaq	104(%r14), %rdi
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_
	movl	%edx, %r12d
	movq	%rax, %r15
	vmovss	192(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	200(%r14), %xmm0
	vmovss	%xmm0, 404(%rsp)
	vmovss	208(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	216(%r14), %xmm0
	vmovss	%xmm0, 388(%rsp)
	vmovss	224(%r14), %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	232(%r14), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	240(%r14), %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	248(%r14), %xmm0
	vmovss	%xmm0, 380(%rsp)
	vmovss	256(%r14), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	264(%r14), %xmm0
	vmovss	%xmm0, 376(%rsp)
	vmovss	272(%r14), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	280(%r14), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	288(%r14), %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	296(%r14), %xmm0
	vmovss	%xmm0, 372(%rsp)
	vmovss	304(%r14), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	312(%r14), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	320(%r14), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	328(%r14), %xmm0
	vmovss	%xmm0, 364(%rsp)
	vmovss	336(%r14), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	344(%r14), %xmm0
	vmovss	%xmm0, 360(%rsp)
	vmovss	552(%r14), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	560(%r14), %xmm0
	vmovss	%xmm0, 396(%rsp)
	vmovss	568(%r14), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	576(%r14), %xmm0
	vmovss	%xmm0, 356(%rsp)
	vmovss	584(%r14), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	592(%r14), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	600(%r14), %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	608(%r14), %xmm0
	vmovss	%xmm0, 348(%rsp)
	vmovss	616(%r14), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	624(%r14), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	632(%r14), %xmm0
	vmovss	%xmm0, 288(%rsp)
	vmovss	640(%r14), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	648(%r14), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	656(%r14), %xmm0
	vmovss	%xmm0, 340(%rsp)
	vmovss	664(%r14), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	672(%r14), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	680(%r14), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	688(%r14), %xmm0
	vmovss	%xmm0, 332(%rsp)
	vmovss	696(%r14), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	704(%r14), %xmm0
	vmovss	%xmm0, 664(%rsp)
.Ltmp2037:
	.loc	1 1194 31 is_stmt 1
	leaq	480(%rsp), %rdi
	leaq	120(%r14), %rsi
	movl	476(%rsp), %ebx
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	696(%rsp), %rdi
	leaq	480(%r14), %rsi
	movl	%ebx, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1193 28
	vmovss	480(%rsp), %xmm0
	vmovss	%xmm0, 144(%rsp)
	vmovss	484(%rsp), %xmm0
	vmovaps	%xmm0, 512(%rsp)
	vmovss	488(%rsp), %xmm0
	vmovaps	%xmm0, 624(%rsp)
	vmovss	492(%rsp), %xmm0
	vmovss	%xmm0, 140(%rsp)
	vmovss	496(%rsp), %xmm0
	vmovaps	%xmm0, 608(%rsp)
	vmovss	500(%rsp), %xmm0
	vmovaps	%xmm0, 592(%rsp)
	vmovss	696(%rsp), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	700(%rsp), %xmm0
	vmovaps	%xmm0, 576(%rsp)
	vmovss	704(%rsp), %xmm0
	vmovaps	%xmm0, 560(%rsp)
	vmovss	708(%rsp), %xmm0
	vmovss	%xmm0, 284(%rsp)
	vmovss	712(%rsp), %xmm0
	vmovaps	%xmm0, 544(%rsp)
	vmovss	716(%rsp), %xmm0
	vmovaps	%xmm0, 528(%rsp)
	movq	%r15, 48(%rsp)
.Ltmp2038:
	.loc	1 0 0 is_stmt 0
	addq	%rbp, %r15
	setb	%cl
	cmpq	152(%rsp), %r15
	seta	%al
.Ltmp2039:
	.loc	38 1050 16 is_stmt 1
	orb	%cl, %al
.Ltmp2040:
	.loc	1 1197 12
	testb	$1, %r12b
	movq	%r15, 648(%rsp)
	je	.LBB32_356
.Ltmp2041:
	.loc	38 1050 16
	testb	%al, %al
	jne	.LBB32_642
.Ltmp2042:
	.loc	1 1053 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
.Ltmp2043:
	.loc	1 1054 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp2044:
	.loc	1 1055 25
	movl	184(%r14), %eax
	movl	%eax, 8(%rsp)
	movl	188(%r14), %ecx
.Ltmp2045:
	.loc	1 1056 24
	movl	544(%r14), %r12d
	movl	548(%r14), %r15d
.Ltmp2046:
	.loc	1 1057 24
	movq	848(%r14), %rdi
.Ltmp2047:
	.loc	2 1916 50
	cmpq	$0, 48(%rsp)
.Ltmp2048:
	.loc	3 900 12
	je	.LBB32_283
.Ltmp2049:
	.loc	3 0 12 is_stmt 0
	movq	440(%rsp), %rax
	leaq	(%rax,%rbp,4), %rbx
	movq	408(%rsp), %rax
	leaq	(%rax,%rbp,4), %r10
	xorl	%esi, %esi
	movq	448(%rsp), %r8
	movl	%ecx, %ebp
	movq	%r10, 656(%rsp)
	movq	%rbx, 464(%rsp)
	.p2align	4
.LBB32_290:
.Ltmp2050:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp2051:
	.loc	1 857 8
	cmpq	%r8, %rax
	jb	.LBB32_292
.Ltmp2052:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %r13
	jmp	.LBB32_293
	.p2align	4
.LBB32_292:
	xorl	%r13d, %r13d
.LBB32_293:
.Ltmp2053:
	.loc	1 1083 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp2054:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_614
.Ltmp2055:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2056:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rbx,%rsi,4), %xmm0
.Ltmp2057:
	vmovss	(%r10,%rsi,4), %xmm3
.Ltmp2058:
	vmovss	152(%r14), %xmm6
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm4
	vmovaps	240(%rsp), %xmm9
	vsubss	%xmm9, %xmm0, %xmm1
	vmulss	%xmm7, %xmm1, %xmm5
	vmovaps	256(%rsp), %xmm2
	vmovss	%xmm6, 148(%rsp)
	vmulss	%xmm6, %xmm2, %xmm6
	vaddss	%xmm5, %xmm6, %xmm5
	vaddss	%xmm5, %xmm2, %xmm6
	vmulss	%xmm7, %xmm2, %xmm8
	vmulss	%xmm4, %xmm1, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	164(%r14), %xmm6, %xmm15
	vmovss	%xmm1, 320(%rsp)
	vaddss	%xmm1, %xmm9, %xmm6
	vmovaps	192(%rsp), %xmm1
	vsubss	%xmm1, %xmm6, %xmm10
	vmulss	304(%rsp), %xmm7, %xmm6
	vmulss	%xmm4, %xmm10, %xmm4
	vaddss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm1, %xmm6
.Ltmp2059:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm12
	vmovss	520(%r14), %xmm13
	vmovaps	224(%rsp), %xmm1
	vsubss	%xmm1, %xmm3, %xmm14
	vmulss	%xmm12, %xmm14, %xmm8
	vmovaps	208(%rsp), %xmm2
	vmulss	%xmm2, %xmm11, %xmm9
	vaddss	%xmm8, %xmm9, %xmm8
	vaddss	%xmm2, %xmm8, %xmm9
	vmulss	524(%r14), %xmm9, %xmm9
.Ltmp2060:
	.loc	1 1083 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp2061:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp2062:
	.loc	1 1084 30
	movq	144(%r14), %rdx
.Ltmp2063:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_615
.Ltmp2064:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2065:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm0, %xmm15, %xmm0
	vsubss	%xmm6, %xmm0, %xmm0
.Ltmp2066:
	.loc	1 1084 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp2067:
	.loc	49 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp2068:
	.loc	1 1085 28
	movq	488(%r14), %rdx
.Ltmp2069:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_616
.Ltmp2070:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2071:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm2, %xmm12, %xmm0
	vmulss	%xmm13, %xmm14, %xmm6
	vaddss	%xmm6, %xmm0, %xmm14
	vaddss	%xmm1, %xmm14, %xmm0
	vmovaps	176(%rsp), %xmm1
	vsubss	%xmm1, %xmm0, %xmm15
	vmulss	160(%rsp), %xmm12, %xmm0
	vmulss	%xmm15, %xmm13, %xmm6
	vaddss	%xmm6, %xmm0, %xmm0
	vaddss	%xmm0, %xmm1, %xmm6
.Ltmp2072:
	.loc	1 1085 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp2073:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp2074:
	.loc	1 1086 29
	movq	504(%r14), %rdx
.Ltmp2075:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp2076:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2077:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm9, %xmm3
	vsubss	%xmm6, %xmm3, %xmm3
.Ltmp2078:
	.loc	1 1086 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp2079:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r9
.Ltmp2080:
	.loc	1 947 24
	addq	%rdi, %r9
.Ltmp2081:
	.loc	1 857 8
	cmpq	%r8, %r9
	jb	.LBB32_303
.Ltmp2082:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %rax
	jmp	.LBB32_304
	.p2align	4
.LBB32_303:
	xorl	%eax, %eax
.LBB32_304:
	movq	128(%r14), %rdx
.Ltmp2083:
	.loc	1 857 8 is_stmt 1
	subq	%rax, %r9
.Ltmp2084:
	.loc	48 568 12
	cmpq	%rdx, %r9
	ja	.LBB32_622
.Ltmp2085:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2086:
	.loc	1 1096 13
	movq	144(%r14), %rax
.Ltmp2087:
	.loc	48 568 12
	cmpq	%rax, %r9
	ja	.LBB32_623
.Ltmp2088:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2089:
	.loc	49 0 9 is_stmt 0
	movq	760(%r14), %r10
.Ltmp2090:
	.loc	1 947 24 is_stmt 1
	addq	%rdi, %r10
.Ltmp2091:
	.loc	1 857 8
	cmpq	%r8, %r10
	movq	%rsi, 680(%rsp)
	jb	.LBB32_310
.Ltmp2092:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %rcx
	jmp	.LBB32_311
	.p2align	4
.LBB32_310:
	xorl	%ecx, %ecx
.LBB32_311:
	movq	488(%r14), %r11
.Ltmp2093:
	.loc	1 857 8 is_stmt 1
	subq	%rcx, %r10
.Ltmp2094:
	.loc	48 568 12
	cmpq	%r11, %r10
	ja	.LBB32_624
.Ltmp2095:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2096:
	.loc	1 1110 13
	movq	504(%r14), %r8
.Ltmp2097:
	.loc	48 568 12
	cmpq	%r8, %r10
	ja	.LBB32_625
.Ltmp2098:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2099:
	.loc	49 0 9 is_stmt 0
	movq	%r13, %rsi
	movl	%ebp, %ecx
	movl	%r12d, %r13d
	movl	%r15d, 68(%rsp)
	movq	120(%r14), %rbx
	movq	%rbx, 504(%rsp)
	vmovss	(%rbx,%r9,4), %xmm3
.Ltmp2100:
	movq	480(%r14), %rbx
	movq	%rbx, 672(%rsp)
	vmovss	(%rbx,%r10,4), %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp2101:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp2102:
	.loc	49 103 24 is_stmt 0
	vandps	%xmm1, %xmm6, %xmm6
	vmovss	.LCPI32_3(%rip), %xmm1
.Ltmp2103:
	.loc	49 71 9 is_stmt 1
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp2104:
	.loc	49 71 9 is_stmt 0
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp2105:
	.loc	49 61 9 is_stmt 1
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp2106:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r12d
	movl	$841731191, %ebp
.Ltmp2107:
	.loc	49 161 24
	jbe	.LBB32_317
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %ebp
.LBB32_317:
.Ltmp2108:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebp, %xmm3
.Ltmp2109:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %r15d
	movl	$8388608, %ebx
.Ltmp2110:
	.loc	49 161 24
	jbe	.LBB32_319
.Ltmp2111:
	.loc	49 0 24 is_stmt 0
	movl	%ebp, %ebx
.LBB32_319:
	vmovss	80(%rsp), %xmm1
	vaddss	404(%rsp), %xmm1, %xmm1
.Ltmp2112:
	.loc	49 185 42 is_stmt 1
	movl	%ebx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2113:
	.loc	23 1291 18
	vmovd	%ebp, %xmm3
.Ltmp2114:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2115:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm6
.Ltmp2116:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm6, %xmm6
.Ltmp2117:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2118:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm6, %xmm6
.Ltmp2119:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2120:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm6, %xmm6
.Ltmp2121:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2122:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm6, %xmm6
.Ltmp2123:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2124:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm6, %xmm6
.Ltmp2125:
	.loc	49 187 28
	shrl	$23, %ebx
	orl	$1258291200, %ebx
.Ltmp2126:
	.loc	49 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp2127:
	.loc	23 1291 18
	vmovd	%ebx, %xmm6
.Ltmp2128:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm6, %xmm6
.Ltmp2129:
	.loc	49 61 9
	vaddss	%xmm3, %xmm6, %xmm3
.Ltmp2130:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2131:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2132:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm1, 80(%rsp)
.Ltmp2133:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm3, %xmm6
.Ltmp2134:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm6, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_321
.Ltmp2135:
	.loc	49 0 44
	xorl	%ebx, %ebx
	jmp	.LBB32_322
	.p2align	4
.LBB32_321:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm6, %xmm9
.Ltmp2136:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2137:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm1, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	144(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %ebx
.Ltmp2138:
.LBB32_322:
	.loc	49 0 0
	movl	%ecx, %ebp
.Ltmp2139:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebx, %xmm6
.Ltmp2140:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp2141:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vminss	%xmm1, %xmm6, %xmm6
.Ltmp2142:
	.loc	1 1005 9 is_stmt 1
	vmovss	8(%rsp), %xmm9
.Ltmp2143:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	512(%rsp), %xmm1
	vblendvps	%xmm13, 624(%rsp), %xmm1, %xmm13
.Ltmp2144:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp2145:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2146:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm9
.Ltmp2147:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm1
	vucomiss	%xmm9, %xmm1
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_324
	.loc	49 0 24
	movl	$0, 8(%rsp)
	jmp	.LBB32_325
	.p2align	4
.LBB32_324:
	.loc	49 166 0
	vmovss	%xmm6, 8(%rsp)
.Ltmp2148:
.LBB32_325:
	.loc	1 0 0
	movq	136(%r14), %rbx
	vmovss	(%rbx,%r9,4), %xmm6
.Ltmp2149:
	movq	496(%r14), %r9
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp2150:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm6, %xmm6
.Ltmp2151:
	.loc	49 103 24 is_stmt 0
	vmovss	(%r9,%r10,4), %xmm9
	vandps	%xmm1, %xmm9, %xmm9
	vmovss	.LCPI32_3(%rip), %xmm1
.Ltmp2152:
	.loc	49 71 9 is_stmt 1
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp2153:
	.loc	49 71 9 is_stmt 0
	vmulss	%xmm1, %xmm9, %xmm9
.Ltmp2154:
	.loc	49 61 9 is_stmt 1
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2155:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
.Ltmp2156:
	.loc	49 161 24
	jbe	.LBB32_327
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %r12d
.LBB32_327:
.Ltmp2157:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r12d, %xmm6
.Ltmp2158:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
.Ltmp2159:
	.loc	49 161 24
	jbe	.LBB32_329
.Ltmp2160:
	.loc	49 0 24 is_stmt 0
	movl	%r12d, %r15d
.LBB32_329:
	vmovss	16(%rsp), %xmm1
	vaddss	400(%rsp), %xmm1, %xmm1
.Ltmp2161:
	.loc	49 185 42 is_stmt 1
	movl	%r15d, %r10d
	andl	$8388607, %r10d
	orl	$1065353216, %r10d
.Ltmp2162:
	.loc	23 1291 18
	vmovd	%r10d, %xmm6
.Ltmp2163:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2164:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2165:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2166:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2167:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2168:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2169:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2170:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2171:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2172:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2173:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2174:
	.loc	49 187 28
	shrl	$23, %r15d
	orl	$1258291200, %r15d
.Ltmp2175:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2176:
	.loc	23 1291 18
	vmovd	%r15d, %xmm9
.Ltmp2177:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2178:
	.loc	49 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2179:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2180:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2181:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm9
	vmovss	%xmm1, 16(%rsp)
.Ltmp2182:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm9, %xmm6
.Ltmp2183:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm6, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_331
.Ltmp2184:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_332
	.p2align	4
.LBB32_331:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm6, %xmm13
.Ltmp2185:
	vmulss	%xmm13, %xmm13, %xmm13
	vmulss	.LCPI32_18(%rip), %xmm13, %xmm13
.Ltmp2186:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm1, %xmm1
	vblendvps	%xmm1, %xmm13, %xmm6, %xmm1
	vmulss	140(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp2187:
.LBB32_332:
	.loc	49 0 0
	movl	68(%rsp), %ecx
.Ltmp2188:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp2189:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2190:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm6, %xmm6, %xmm6
	vminss	%xmm6, %xmm1, %xmm1
.Ltmp2191:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ebp, %xmm6
.Ltmp2192:
	.loc	49 161 24
	vcmpnltss	%xmm6, %xmm1, %xmm13
	vmovaps	608(%rsp), %xmm2
	vblendvps	%xmm13, 592(%rsp), %xmm2, %xmm13
.Ltmp2193:
	.loc	49 66 9
	vsubss	%xmm1, %xmm6, %xmm6
.Ltmp2194:
	.loc	49 92 9
	vmulss	%xmm6, %xmm13, %xmm6
	vaddss	%xmm6, %xmm1, %xmm6
.Ltmp2195:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm6, %xmm1
.Ltmp2196:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_334
	.loc	49 0 24
	xorl	%ebp, %ebp
	jmp	.LBB32_335
	.p2align	4
.LBB32_334:
	.loc	49 166 0
	vmovd	%xmm6, %ebp
.Ltmp2197:
.LBB32_335:
	.loc	49 0 0
	vmovss	56(%rsp), %xmm1
	vaddss	396(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 56(%rsp)
.Ltmp2198:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2199:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_337
.Ltmp2200:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_338
	.p2align	4
.LBB32_337:
	vmovss	.LCPI32_17(%rip), %xmm6
	vaddss	%xmm6, %xmm3, %xmm1
.Ltmp2201:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp2202:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm6, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vmulss	136(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp2203:
.LBB32_338:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp2204:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2205:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp2206:
	.loc	1 1005 9 is_stmt 1
	vmovd	%r13d, %xmm3
.Ltmp2207:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm6
	vmovaps	576(%rsp), %xmm2
	vblendvps	%xmm6, 560(%rsp), %xmm2, %xmm6
.Ltmp2208:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2209:
	.loc	49 92 9
	vmulss	%xmm6, %xmm3, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp2210:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp2211:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm6
	vucomiss	%xmm1, %xmm6
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_340
	.loc	49 0 24
	xorl	%r12d, %r12d
	jmp	.LBB32_341
	.p2align	4
.LBB32_340:
	.loc	49 166 0
	vmovd	%xmm3, %r12d
.Ltmp2212:
.LBB32_341:
	.loc	49 0 0
	vmovss	288(%rsp), %xmm1
	vaddss	392(%rsp), %xmm1, %xmm1
	vmovss	%xmm1, 288(%rsp)
.Ltmp2213:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm1, %xmm9, %xmm3
.Ltmp2214:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_343
.Ltmp2215:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_344
	.p2align	4
.LBB32_343:
	vmovss	.LCPI32_17(%rip), %xmm6
	vaddss	%xmm6, %xmm3, %xmm1
.Ltmp2216:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp2217:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm6, %xmm6
	vblendvps	%xmm6, %xmm1, %xmm3, %xmm1
	vmulss	284(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp2218:
.LBB32_344:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp2219:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2220:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp2221:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp2222:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm6
	vmovaps	544(%rsp), %xmm2
	vblendvps	%xmm6, 528(%rsp), %xmm2, %xmm6
.Ltmp2223:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2224:
	.loc	49 92 9
	vmulss	%xmm6, %xmm3, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp2225:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp2226:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm6
	vucomiss	%xmm1, %xmm6
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_346
	.loc	49 0 24
	xorl	%r15d, %r15d
	jmp	.LBB32_347
	.p2align	4
.LBB32_346:
	.loc	49 166 0
	vmovd	%xmm3, %r15d
.Ltmp2227:
.LBB32_347:
	.loc	1 0 0
	negq	%rsi
	addq	%rsi, %rdi
	incq	%rdi
.Ltmp2228:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp2229:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2230:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_619
.Ltmp2231:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2232:
	.loc	48 568 12
	cmpq	%r11, %rdi
	ja	.LBB32_620
.Ltmp2233:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2234:
	.loc	48 568 12
	cmpq	%r8, %rdi
	movq	656(%rsp), %r10
	movq	680(%rsp), %rcx
	ja	.LBB32_621
.Ltmp2235:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2236:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm1
	vmovaps	304(%rsp), %xmm13
	vmulss	148(%rsp), %xmm13, %xmm2
	vaddss	%xmm1, %xmm2, %xmm1
	vmulss	%xmm15, %xmm12, %xmm2
	vmovaps	160(%rsp), %xmm12
	vmulss	%xmm11, %xmm12, %xmm3
	vaddss	%xmm2, %xmm3, %xmm2
	vaddss	%xmm5, %xmm5, %xmm3
	vaddss	256(%rsp), %xmm3, %xmm7
	vmovss	320(%rsp), %xmm3
	vaddss	%xmm3, %xmm3, %xmm3
	vaddss	240(%rsp), %xmm3, %xmm5
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	%xmm4, %xmm4, %xmm3
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm14, %xmm14, %xmm6
	vaddss	%xmm2, %xmm2, %xmm9
	vaddss	%xmm0, %xmm0, %xmm10
	vaddss	%xmm1, %xmm13, %xmm8
	vaddss	192(%rsp), %xmm3, %xmm0
	vaddss	208(%rsp), %xmm4, %xmm1
	vaddss	224(%rsp), %xmm6, %xmm2
	vaddss	%xmm9, %xmm12, %xmm3
	vaddss	176(%rsp), %xmm10, %xmm4
	vmovss	24(%rsp), %xmm9
	vaddss	376(%rsp), %xmm9, %xmm9
	vmovss	8(%rsp), %xmm6
	vmovss	%xmm9, 24(%rsp)
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
	vmovaps	%xmm7, 256(%rsp)
	vandps	%xmm2, %xmm14, %xmm7
	vcmpnltss	%xmm15, %xmm10, %xmm9
	vandps	%xmm5, %xmm9, %xmm5
	vmovaps	%xmm5, 240(%rsp)
	vandps	%xmm3, %xmm14, %xmm5
	vcmpnltss	%xmm15, %xmm11, %xmm9
	vandps	%xmm8, %xmm9, %xmm8
	vmovaps	%xmm8, 304(%rsp)
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
	vmovaps	%xmm0, 192(%rsp)
	vandps	%xmm1, %xmm10, %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vandps	%xmm2, %xmm7, %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vandps	%xmm3, %xmm5, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vandps	%xmm4, %xmm8, %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm11
	vminss	%xmm11, %xmm6, %xmm0
	vroundss	$9, %xmm0, %xmm0, %xmm2
	vmovss	416(%rsp), %xmm5
	vaddss	344(%rsp), %xmm5, %xmm5
	vmovss	28(%rsp), %xmm1
	vaddss	360(%rsp), %xmm1, %xmm1
	vsubss	%xmm2, %xmm0, %xmm3
	vmovd	%ebp, %xmm0
	vmovss	%xmm1, 28(%rsp)
	vaddss	%xmm0, %xmm1, %xmm0
	vmulss	%xmm0, %xmm14, %xmm0
	vmaxss	%xmm12, %xmm0, %xmm0
	vminss	%xmm11, %xmm0, %xmm1
	vroundss	$9, %xmm1, %xmm1, %xmm7
	vsubss	%xmm7, %xmm1, %xmm1
	vmovd	%r12d, %xmm4
	vmovss	%xmm5, 416(%rsp)
	vaddss	%xmm4, %xmm5, %xmm4
	vmulss	%xmm4, %xmm14, %xmm4
	vmaxss	%xmm12, %xmm4, %xmm4
	vminss	%xmm11, %xmm4, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm6
.Ltmp2237:
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
.Ltmp2238:
	vroundss	$9, %xmm4, %xmm4, %xmm3
	vsubss	%xmm3, %xmm4, %xmm4
	vmovss	72(%rsp), %xmm0
	vaddss	664(%rsp), %xmm0, %xmm0
	vmovss	.LCPI32_29(%rip), %xmm14
.Ltmp2239:
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
.Ltmp2240:
	vmulss	%xmm5, %xmm1, %xmm1
.Ltmp2241:
	vmovd	%r15d, %xmm5
	vmovss	%xmm0, 72(%rsp)
	vaddss	%xmm5, %xmm0, %xmm5
	vmulss	.LCPI32_21(%rip), %xmm5, %xmm5
	vmaxss	%xmm12, %xmm5, %xmm5
	vminss	%xmm11, %xmm5, %xmm5
.Ltmp2242:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm7, %xmm15, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp2243:
	vmulss	%xmm6, %xmm4, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm9, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm10, %xmm1
	vmulss	%xmm1, %xmm4, %xmm1
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp2244:
	vmulss	%xmm1, %xmm4, %xmm1
.Ltmp2245:
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
.Ltmp2246:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm3, %xmm15, %xmm3
	vmovd	%xmm3, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm3
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp2247:
	vmulss	%xmm6, %xmm5, %xmm3
	vaddss	%xmm3, %xmm8, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm9, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp2248:
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm4, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	504(%rsp), %rax
.Ltmp2249:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm2
	vmulss	(%rbx,%rdi,4), %xmm0, %xmm0
.Ltmp2250:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp2251:
	vaddss	%xmm0, %xmm2, %xmm0
	movq	672(%rsp), %rax
.Ltmp2252:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
.Ltmp2253:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r9,%rdi,4), %xmm3, %xmm2
.Ltmp2254:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	464(%rsp), %rbx
.Ltmp2255:
	.loc	49 56 9
	vmovss	%xmm0, (%rbx,%rcx,4)
.Ltmp2256:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r10,%rcx,4)
	vmovss	32(%rsp), %xmm0
.Ltmp2257:
	.loc	1 0 0
	vaddss	388(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	104(%rsp), %xmm0
	vaddss	356(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	36(%rsp), %xmm0
	vaddss	384(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	112(%rsp), %xmm0
	vaddss	352(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	40(%rsp), %xmm0
	vaddss	380(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	120(%rsp), %xmm0
	vaddss	348(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	44(%rsp), %xmm0
	vaddss	372(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	12(%rsp), %xmm0
	vaddss	340(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	88(%rsp), %xmm0
	vaddss	368(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	20(%rsp), %xmm0
	vaddss	336(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	96(%rsp), %xmm0
	vaddss	364(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	128(%rsp), %xmm0
	vaddss	332(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 128(%rsp)
	leaq	1(%rcx), %rsi
.Ltmp2258:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rsi, 48(%rsp)
	movq	448(%rsp), %r8
.Ltmp2259:
	.loc	3 900 12
	jne	.LBB32_290
	jmp	.LBB32_284
.Ltmp2260:
	.loc	3 0 12 is_stmt 0
.Ltmp2261:
	.p2align	4
.LBB32_356:
	.loc	38 1050 16 is_stmt 1
	testb	%al, %al
	jne	.LBB32_643
.Ltmp2262:
	.loc	1 1053 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
.Ltmp2263:
	.loc	1 1054 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp2264:
	.loc	1 1055 25
	movl	184(%r14), %eax
	movl	%eax, 8(%rsp)
	movl	188(%r14), %ecx
.Ltmp2265:
	.loc	1 1056 24
	movl	544(%r14), %r13d
	movl	548(%r14), %ebx
.Ltmp2266:
	.loc	1 1057 24
	movq	848(%r14), %rdi
.Ltmp2267:
	.loc	2 1916 50
	cmpq	$0, 48(%rsp)
.Ltmp2268:
	.loc	3 900 12
	je	.LBB32_423
.Ltmp2269:
	.loc	3 0 12 is_stmt 0
	movq	440(%rsp), %rax
	leaq	(%rax,%rbp,4), %rsi
	movq	408(%rsp), %rax
	leaq	(%rax,%rbp,4), %r10
	xorl	%r12d, %r12d
	movq	448(%rsp), %r8
	movl	%ecx, %ebp
	movq	%r10, 96(%rsp)
	movq	%rsi, 88(%rsp)
	.p2align	4
.LBB32_359:
.Ltmp2270:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp2271:
	.loc	1 857 8
	cmpq	%r8, %rax
	jb	.LBB32_361
.Ltmp2272:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %r15
	jmp	.LBB32_362
	.p2align	4
.LBB32_361:
	xorl	%r15d, %r15d
.LBB32_362:
.Ltmp2273:
	.loc	1 1083 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp2274:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_614
.Ltmp2275:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2276:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rsi,%r12,4), %xmm6
.Ltmp2277:
	vmovss	(%r10,%r12,4), %xmm13
.Ltmp2278:
	vmovss	152(%r14), %xmm4
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm0
	vmovaps	240(%rsp), %xmm8
	vsubss	%xmm8, %xmm6, %xmm1
	vmulss	%xmm7, %xmm1, %xmm3
	vmovaps	256(%rsp), %xmm2
	vmovss	%xmm4, 20(%rsp)
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm3, %xmm4, %xmm5
	vaddss	%xmm5, %xmm2, %xmm3
	vmulss	%xmm7, %xmm2, %xmm4
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	%xmm1, %xmm4, %xmm1
	vmulss	164(%r14), %xmm3, %xmm3
	vmovss	%xmm1, 128(%rsp)
	vaddss	%xmm1, %xmm8, %xmm4
	vmovaps	192(%rsp), %xmm1
	vsubss	%xmm1, %xmm4, %xmm10
	vmulss	304(%rsp), %xmm7, %xmm4
	vmulss	%xmm0, %xmm10, %xmm0
	vaddss	%xmm0, %xmm4, %xmm4
	vaddss	%xmm4, %xmm1, %xmm14
.Ltmp2279:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm0
	vmovss	520(%r14), %xmm12
	vmovaps	224(%rsp), %xmm1
	vsubss	%xmm1, %xmm13, %xmm9
	vmulss	%xmm0, %xmm9, %xmm8
	vmovaps	208(%rsp), %xmm2
	vmulss	%xmm2, %xmm11, %xmm15
	vaddss	%xmm8, %xmm15, %xmm8
	vaddss	%xmm2, %xmm8, %xmm15
	vmulss	524(%r14), %xmm15, %xmm15
.Ltmp2280:
	.loc	1 1083 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp2281:
	.loc	49 56 9
	vmovss	%xmm14, (%rax,%rdi,4)
.Ltmp2282:
	.loc	1 1084 30
	movq	144(%r14), %rdx
.Ltmp2283:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_615
.Ltmp2284:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2285:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm6, %xmm3
	vsubss	%xmm14, %xmm3, %xmm3
.Ltmp2286:
	.loc	1 1084 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp2287:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp2288:
	.loc	1 1085 28
	movq	488(%r14), %rdx
.Ltmp2289:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_616
.Ltmp2290:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2291:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm0, %xmm2, %xmm3
	vmulss	%xmm12, %xmm9, %xmm6
	vaddss	%xmm6, %xmm3, %xmm6
	vaddss	%xmm6, %xmm1, %xmm3
	vmovaps	176(%rsp), %xmm1
	vsubss	%xmm1, %xmm3, %xmm9
	vmulss	160(%rsp), %xmm0, %xmm3
	vmulss	%xmm9, %xmm12, %xmm12
	vaddss	%xmm3, %xmm12, %xmm12
	vaddss	%xmm1, %xmm12, %xmm3
.Ltmp2292:
	.loc	1 1085 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp2293:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp2294:
	.loc	1 1086 29
	movq	504(%r14), %rdx
.Ltmp2295:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp2296:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2297:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm15, %xmm13, %xmm13
	vsubss	%xmm3, %xmm13, %xmm3
.Ltmp2298:
	.loc	1 1086 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp2299:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r9
.Ltmp2300:
	.loc	1 947 24
	addq	%rdi, %r9
.Ltmp2301:
	.loc	1 857 8
	cmpq	%r8, %r9
	jb	.LBB32_372
.Ltmp2302:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %rax
	jmp	.LBB32_373
	.p2align	4
.LBB32_372:
	xorl	%eax, %eax
.LBB32_373:
	movq	128(%r14), %rdx
.Ltmp2303:
	.loc	1 857 8 is_stmt 1
	subq	%rax, %r9
.Ltmp2304:
	.loc	48 568 12
	cmpq	%rdx, %r9
	ja	.LBB32_622
.Ltmp2305:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2306:
	.loc	1 1096 13
	movq	144(%r14), %rax
.Ltmp2307:
	.loc	48 568 12
	cmpq	%rax, %r9
	ja	.LBB32_623
.Ltmp2308:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2309:
	.loc	49 0 9 is_stmt 0
	movq	760(%r14), %r10
.Ltmp2310:
	.loc	1 947 24 is_stmt 1
	addq	%rdi, %r10
.Ltmp2311:
	.loc	1 857 8
	cmpq	%r8, %r10
	movq	%r12, 120(%rsp)
	jb	.LBB32_379
.Ltmp2312:
	.loc	1 0 8 is_stmt 0
	movq	%r8, %rcx
	jmp	.LBB32_380
	.p2align	4
.LBB32_379:
	xorl	%ecx, %ecx
.LBB32_380:
	movq	488(%r14), %r11
.Ltmp2313:
	.loc	1 857 8 is_stmt 1
	subq	%rcx, %r10
.Ltmp2314:
	.loc	48 568 12
	cmpq	%r11, %r10
	ja	.LBB32_624
.Ltmp2315:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2316:
	.loc	1 1110 13
	movq	504(%r14), %r8
.Ltmp2317:
	.loc	48 568 12
	cmpq	%r8, %r10
	ja	.LBB32_625
.Ltmp2318:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2319:
	.loc	49 0 9 is_stmt 0
	movq	%r15, %r12
	movl	%ebp, %ecx
	movl	%r13d, %r15d
	movl	%ebx, 12(%rsp)
	movq	120(%r14), %rsi
	movq	%rsi, 104(%rsp)
	vmovss	(%rsi,%r9,4), %xmm3
.Ltmp2320:
	movq	480(%r14), %rsi
	movq	%rsi, 112(%rsp)
	vmovss	(%rsi,%r10,4), %xmm13
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp2321:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp2322:
	.loc	49 103 24 is_stmt 0
	vandps	%xmm1, %xmm13, %xmm13
	vmovss	.LCPI32_3(%rip), %xmm1
.Ltmp2323:
	.loc	49 71 9 is_stmt 1
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp2324:
	.loc	49 71 9 is_stmt 0
	vmulss	%xmm1, %xmm13, %xmm13
.Ltmp2325:
	.loc	49 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp2326:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r13d
	movl	$841731191, %ebp
.Ltmp2327:
	.loc	49 161 24
	jbe	.LBB32_386
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %ebp
.LBB32_386:
.Ltmp2328:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebp, %xmm3
.Ltmp2329:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %ebx
	movl	$8388608, %esi
.Ltmp2330:
	.loc	49 161 24
	jbe	.LBB32_388
.Ltmp2331:
	.loc	49 0 24 is_stmt 0
	movl	%ebp, %esi
.LBB32_388:
.Ltmp2332:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2333:
	.loc	23 1291 18
	vmovd	%ebp, %xmm3
.Ltmp2334:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2335:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm13
.Ltmp2336:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm13, %xmm13
.Ltmp2337:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp2338:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm13, %xmm13
.Ltmp2339:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp2340:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm13, %xmm13
.Ltmp2341:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp2342:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm13, %xmm13
.Ltmp2343:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm13
.Ltmp2344:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm13, %xmm13
.Ltmp2345:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp2346:
	.loc	23 1291 18
	vmovd	%esi, %xmm14
.Ltmp2347:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm14, %xmm14
.Ltmp2348:
	.loc	49 71 9
	vmulss	%xmm3, %xmm13, %xmm3
.Ltmp2349:
	.loc	49 61 9
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2350:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2351:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2352:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm13
.Ltmp2353:
	.loc	49 66 9 is_stmt 1
	vsubss	80(%rsp), %xmm13, %xmm3
.Ltmp2354:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_390
.Ltmp2355:
	.loc	49 0 44
	xorl	%esi, %esi
	jmp	.LBB32_391
	.p2align	4
.LBB32_390:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm3, %xmm14
.Ltmp2356:
	vmulss	%xmm14, %xmm14, %xmm14
	vmulss	.LCPI32_18(%rip), %xmm14, %xmm14
.Ltmp2357:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm1, %xmm15
	vblendvps	%xmm15, %xmm14, %xmm3, %xmm3
	vmulss	144(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %esi
.Ltmp2358:
.LBB32_391:
	.loc	49 0 0
	movl	%ecx, %ebp
.Ltmp2359:
	.loc	23 1291 18 is_stmt 1
	vmovd	%esi, %xmm3
.Ltmp2360:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2361:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vminss	%xmm1, %xmm3, %xmm3
.Ltmp2362:
	.loc	1 1005 9 is_stmt 1
	vmovss	8(%rsp), %xmm14
.Ltmp2363:
	.loc	49 161 24
	vcmpnltss	%xmm14, %xmm3, %xmm15
	vmovaps	512(%rsp), %xmm1
	vblendvps	%xmm15, 624(%rsp), %xmm1, %xmm15
.Ltmp2364:
	.loc	49 66 9
	vsubss	%xmm3, %xmm14, %xmm14
.Ltmp2365:
	.loc	49 92 9
	vmulss	%xmm15, %xmm14, %xmm14
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2366:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm14
.Ltmp2367:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm1
	vucomiss	%xmm14, %xmm1
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_393
	.loc	49 0 24
	movl	$0, 8(%rsp)
	jmp	.LBB32_394
	.p2align	4
.LBB32_393:
	.loc	49 166 0
	vmovss	%xmm3, 8(%rsp)
.Ltmp2368:
.LBB32_394:
	.loc	1 0 0
	movq	136(%r14), %rsi
	vmovss	(%rsi,%r9,4), %xmm3
.Ltmp2369:
	movq	496(%r14), %r9
	vbroadcastss	.LCPI32_1(%rip), %xmm1
.Ltmp2370:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm1, %xmm3, %xmm3
.Ltmp2371:
	.loc	49 103 24 is_stmt 0
	vmovss	(%r9,%r10,4), %xmm14
	vandps	%xmm1, %xmm14, %xmm14
	vmovss	.LCPI32_3(%rip), %xmm1
.Ltmp2372:
	.loc	49 71 9 is_stmt 1
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp2373:
	.loc	49 71 9 is_stmt 0
	vmulss	%xmm1, %xmm14, %xmm14
.Ltmp2374:
	.loc	49 61 9 is_stmt 1
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2375:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
.Ltmp2376:
	.loc	49 161 24
	jbe	.LBB32_396
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %r13d
.LBB32_396:
.Ltmp2377:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r13d, %xmm3
.Ltmp2378:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
.Ltmp2379:
	.loc	49 161 24
	jbe	.LBB32_398
.Ltmp2380:
	.loc	49 0 24 is_stmt 0
	movl	%r13d, %ebx
.LBB32_398:
.Ltmp2381:
	.loc	49 185 42 is_stmt 1
	movl	%ebx, %r10d
	andl	$8388607, %r10d
	orl	$1065353216, %r10d
.Ltmp2382:
	.loc	23 1291 18
	vmovd	%r10d, %xmm3
.Ltmp2383:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2384:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm14
.Ltmp2385:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm14, %xmm14
.Ltmp2386:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp2387:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm14, %xmm14
.Ltmp2388:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp2389:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm14, %xmm14
.Ltmp2390:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp2391:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm14, %xmm14
.Ltmp2392:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm14
.Ltmp2393:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm14, %xmm14
.Ltmp2394:
	.loc	49 187 28
	shrl	$23, %ebx
	orl	$1258291200, %ebx
.Ltmp2395:
	.loc	23 1291 18
	vmovd	%ebx, %xmm15
.Ltmp2396:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm15, %xmm15
.Ltmp2397:
	.loc	49 71 9
	vmulss	%xmm3, %xmm14, %xmm3
.Ltmp2398:
	.loc	49 61 9
	vaddss	%xmm3, %xmm15, %xmm3
.Ltmp2399:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2400:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2401:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm15
.Ltmp2402:
	.loc	49 66 9 is_stmt 1
	vsubss	16(%rsp), %xmm15, %xmm3
.Ltmp2403:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_400
.Ltmp2404:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_401
	.p2align	4
.LBB32_400:
	vmovss	.LCPI32_17(%rip), %xmm1
	vaddss	%xmm1, %xmm3, %xmm14
.Ltmp2405:
	vmulss	%xmm14, %xmm14, %xmm14
	vmulss	.LCPI32_18(%rip), %xmm14, %xmm14
.Ltmp2406:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm1, %xmm1
	vblendvps	%xmm1, %xmm14, %xmm3, %xmm1
	vmulss	140(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp2407:
.LBB32_401:
	.loc	49 0 0
	movl	12(%rsp), %ecx
.Ltmp2408:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp2409:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2410:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp2411:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ebp, %xmm3
.Ltmp2412:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm14
	vmovaps	608(%rsp), %xmm2
	vblendvps	%xmm14, 592(%rsp), %xmm2, %xmm14
.Ltmp2413:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2414:
	.loc	49 92 9
	vmulss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp2415:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp2416:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm1, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_404
.Ltmp2417:
	.loc	49 0 24
	xorl	%ebp, %ebp
.Ltmp2418:
	.loc	49 66 9 is_stmt 1
	vsubss	56(%rsp), %xmm13, %xmm3
.Ltmp2419:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jae	.LBB32_405
.Ltmp2420:
.LBB32_403:
	.loc	49 0 44
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm1
.Ltmp2421:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp2422:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm1, %xmm3, %xmm1
	vmulss	136(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp2423:
	.loc	49 161 44
	jmp	.LBB32_406
.Ltmp2424:
	.loc	49 0 44
.Ltmp2425:
	.p2align	4
.LBB32_404:
	.loc	49 166 0 is_stmt 1
	vmovd	%xmm3, %ebp
.Ltmp2426:
	.loc	49 66 9
	vsubss	56(%rsp), %xmm13, %xmm3
.Ltmp2427:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_403
.LBB32_405:
	.loc	49 0 44
	xorl	%r10d, %r10d
.LBB32_406:
.Ltmp2428:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp2429:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2430:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp2431:
	.loc	1 1005 9 is_stmt 1
	vmovd	%r15d, %xmm3
.Ltmp2432:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm13
	vmovaps	576(%rsp), %xmm2
	vblendvps	%xmm13, 560(%rsp), %xmm2, %xmm13
.Ltmp2433:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2434:
	.loc	49 92 9
	vmulss	%xmm3, %xmm13, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp2435:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp2436:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_409
.Ltmp2437:
	.loc	49 0 24
	xorl	%r13d, %r13d
.Ltmp2438:
	.loc	49 66 9 is_stmt 1
	vsubss	288(%rsp), %xmm15, %xmm3
.Ltmp2439:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jae	.LBB32_410
.Ltmp2440:
.LBB32_408:
	.loc	49 0 44
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm3, %xmm13, %xmm1
.Ltmp2441:
	vmulss	%xmm1, %xmm1, %xmm1
	vmulss	.LCPI32_18(%rip), %xmm1, %xmm1
.Ltmp2442:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm1, %xmm3, %xmm1
	vmulss	284(%rsp), %xmm1, %xmm1
	vmovd	%xmm1, %r10d
.Ltmp2443:
	.loc	49 161 44
	jmp	.LBB32_411
.Ltmp2444:
	.loc	49 0 44
.Ltmp2445:
	.p2align	4
.LBB32_409:
	.loc	49 166 0 is_stmt 1
	vmovd	%xmm3, %r13d
.Ltmp2446:
	.loc	49 66 9
	vsubss	288(%rsp), %xmm15, %xmm3
.Ltmp2447:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm1
	vucomiss	%xmm3, %xmm1
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_408
.LBB32_410:
	.loc	49 0 44
	xorl	%r10d, %r10d
.LBB32_411:
.Ltmp2448:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm1
.Ltmp2449:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm1, %xmm1
.Ltmp2450:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
	vminss	%xmm3, %xmm1, %xmm1
.Ltmp2451:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp2452:
	.loc	49 161 24
	vcmpnltss	%xmm3, %xmm1, %xmm13
	vmovaps	544(%rsp), %xmm2
	vblendvps	%xmm13, 528(%rsp), %xmm2, %xmm13
.Ltmp2453:
	.loc	49 66 9
	vsubss	%xmm1, %xmm3, %xmm3
.Ltmp2454:
	.loc	49 92 9
	vmulss	%xmm3, %xmm13, %xmm3
	vaddss	%xmm3, %xmm1, %xmm3
.Ltmp2455:
	.loc	49 103 24
	vbroadcastss	.LCPI32_1(%rip), %xmm1
	vandps	%xmm1, %xmm3, %xmm1
.Ltmp2456:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm1, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_413
	.loc	49 0 24
	xorl	%ebx, %ebx
	jmp	.LBB32_414
	.p2align	4
.LBB32_413:
	.loc	49 166 0
	vmovd	%xmm3, %ebx
.Ltmp2457:
.LBB32_414:
	.loc	1 0 0
	negq	%r12
	addq	%r12, %rdi
	incq	%rdi
.Ltmp2458:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp2459:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2460:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_619
.Ltmp2461:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2462:
	.loc	48 568 12
	cmpq	%r11, %rdi
	ja	.LBB32_620
.Ltmp2463:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2464:
	.loc	48 568 12
	cmpq	%r8, %rdi
	movq	96(%rsp), %r10
	movq	120(%rsp), %rcx
	ja	.LBB32_621
.Ltmp2465:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2466:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm1
	vmovaps	304(%rsp), %xmm10
	vmulss	20(%rsp), %xmm10, %xmm2
	vaddss	%xmm1, %xmm2, %xmm1
	vmulss	%xmm0, %xmm9, %xmm0
	vmovaps	160(%rsp), %xmm13
	vmulss	%xmm11, %xmm13, %xmm2
	vaddss	%xmm0, %xmm2, %xmm0
	vaddss	%xmm5, %xmm5, %xmm2
	vaddss	256(%rsp), %xmm2, %xmm3
	vmovss	128(%rsp), %xmm2
	vaddss	%xmm2, %xmm2, %xmm2
	vaddss	240(%rsp), %xmm2, %xmm7
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	%xmm4, %xmm4, %xmm2
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm6, %xmm6, %xmm5
	vaddss	%xmm0, %xmm0, %xmm6
	vaddss	%xmm12, %xmm12, %xmm8
	vaddss	%xmm1, %xmm10, %xmm9
	vaddss	192(%rsp), %xmm2, %xmm0
	vaddss	208(%rsp), %xmm4, %xmm1
	vaddss	224(%rsp), %xmm5, %xmm2
	vaddss	%xmm6, %xmm13, %xmm4
	vaddss	176(%rsp), %xmm8, %xmm5
	vmovss	8(%rsp), %xmm6
	vaddss	24(%rsp), %xmm6, %xmm6
	vbroadcastss	.LCPI32_1(%rip), %xmm14
	vandps	%xmm3, %xmm14, %xmm8
	vandps	%xmm7, %xmm14, %xmm10
	vandps	%xmm14, %xmm9, %xmm11
	vandps	%xmm0, %xmm14, %xmm12
	vandps	%xmm1, %xmm14, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm15
	vcmpnltss	%xmm15, %xmm8, %xmm8
	vandps	%xmm3, %xmm8, %xmm3
	vmovaps	%xmm3, 256(%rsp)
	vandps	%xmm2, %xmm14, %xmm3
	vcmpnltss	%xmm15, %xmm10, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovaps	%xmm7, 240(%rsp)
	vandps	%xmm4, %xmm14, %xmm7
	vcmpnltss	%xmm15, %xmm11, %xmm8
	vandps	%xmm9, %xmm8, %xmm8
	vmovaps	%xmm8, 304(%rsp)
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
	vmovaps	%xmm0, 192(%rsp)
	vandps	%xmm1, %xmm10, %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vandps	%xmm2, %xmm3, %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vandps	%xmm4, %xmm7, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vandps	%xmm5, %xmm8, %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm7
	vminss	%xmm7, %xmm6, %xmm0
	vroundss	$9, %xmm0, %xmm0, %xmm2
	vsubss	%xmm2, %xmm0, %xmm4
	vmovd	%ebp, %xmm0
	vaddss	28(%rsp), %xmm0, %xmm0
	vmulss	%xmm0, %xmm11, %xmm0
	vmaxss	%xmm12, %xmm0, %xmm0
	vminss	%xmm7, %xmm0, %xmm1
	vroundss	$9, %xmm1, %xmm1, %xmm0
	vsubss	%xmm0, %xmm1, %xmm1
	vmovd	%r13d, %xmm3
	vaddss	416(%rsp), %xmm3, %xmm3
	vmulss	%xmm3, %xmm11, %xmm3
	vmaxss	%xmm12, %xmm3, %xmm3
	vminss	%xmm7, %xmm3, %xmm5
	vmovss	.LCPI32_24(%rip), %xmm6
.Ltmp2467:
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
.Ltmp2468:
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
	vmovss	.LCPI32_29(%rip), %xmm14
.Ltmp2469:
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
.Ltmp2470:
	vmulss	%xmm3, %xmm1, %xmm1
.Ltmp2471:
	vmovd	%ebx, %xmm3
	vaddss	72(%rsp), %xmm3, %xmm3
	vmulss	%xmm3, %xmm11, %xmm3
	vmaxss	%xmm12, %xmm3, %xmm3
	vminss	%xmm7, %xmm3, %xmm3
.Ltmp2472:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm0, %xmm15, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm1, %xmm0
.Ltmp2473:
	vmulss	%xmm6, %xmm5, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm9, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm10, %xmm1
	vmulss	%xmm1, %xmm5, %xmm1
	vaddss	%xmm1, %xmm13, %xmm1
.Ltmp2474:
	vmulss	%xmm1, %xmm5, %xmm1
.Ltmp2475:
	vroundss	$9, %xmm3, %xmm3, %xmm5
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp2476:
	vaddss	%xmm1, %xmm14, %xmm1
	vaddss	%xmm4, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	vmulss	%xmm4, %xmm1, %xmm1
.Ltmp2477:
	vmulss	%xmm6, %xmm3, %xmm4
	vaddss	%xmm4, %xmm8, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm9, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm10, %xmm4
	vmulss	%xmm4, %xmm3, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
.Ltmp2478:
	vmulss	%xmm4, %xmm3, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm5, %xmm15, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	104(%rsp), %rax
.Ltmp2479:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm2
	vmulss	(%rsi,%rdi,4), %xmm0, %xmm0
.Ltmp2480:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp2481:
	vaddss	%xmm0, %xmm2, %xmm0
	movq	112(%rsp), %rax
.Ltmp2482:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
.Ltmp2483:
	.loc	49 71 9 is_stmt 1
	vmulss	(%r9,%rdi,4), %xmm3, %xmm2
.Ltmp2484:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	88(%rsp), %rsi
.Ltmp2485:
	.loc	49 56 9
	vmovss	%xmm0, (%rsi,%rcx,4)
.Ltmp2486:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r10,%rcx,4)
.Ltmp2487:
	.loc	1 0 0
	leaq	1(%rcx), %r12
.Ltmp2488:
	.loc	2 1916 50 is_stmt 1
	cmpq	%r12, 48(%rsp)
	movq	448(%rsp), %r8
.Ltmp2489:
	.loc	3 900 12
	jne	.LBB32_359
	jmp	.LBB32_424
.Ltmp2490:
.LBB32_423:
	.loc	3 0 12 is_stmt 0
	movl	%ecx, %ebp
.LBB32_424:
	vmovaps	256(%rsp), %xmm0
	.loc	1 1160 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	240(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	304(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	208(%rsp), %xmm0
	.loc	1 1161 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	224(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	176(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	8(%rsp), %eax
	.loc	1 1162 5
	movl	%eax, 184(%r14)
	movl	%ebp, 188(%r14)
	.loc	1 1163 5
	movl	%r13d, 544(%r14)
	movl	%ebx, 548(%r14)
	.loc	1 1164 5
	movq	%rdi, 848(%r14)
	jmp	.LBB32_285
.Ltmp2491:
.LBB32_425:
	.loc	1 1186 23
	movl	856(%r14), %eax
	movl	%eax, 648(%rsp)
.Ltmp2492:
	.loc	1 1187 20
	movq	840(%r14), %r12
	xorl	%ebp, %ebp
	movq	%r12, 320(%rsp)
	jmp	.LBB32_429
.LBB32_426:
	.loc	1 0 20 is_stmt 0
	movl	%ecx, %ebp
.LBB32_427:
	vmovaps	224(%rsp), %xmm0
.Ltmp2493:
	.loc	1 1160 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	256(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	240(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	208(%rsp), %xmm0
	.loc	1 1161 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	288(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	176(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	48(%rsp), %eax
	.loc	1 1162 5
	movl	%eax, 184(%r14)
	movl	8(%rsp), %eax
	movl	%eax, 188(%r14)
	.loc	1 1163 5
	movl	%ebp, 544(%r14)
	movl	%esi, 548(%r14)
	.loc	1 1164 5
	movq	%rdi, 848(%r14)
.Ltmp2494:
.LBB32_428:
	.loc	1 0 5 is_stmt 0
	movq	464(%rsp), %rax
	movq	%rax, %rbp
	movq	152(%rsp), %r9
	.loc	1 1189 11 is_stmt 1
	cmpq	%r9, %rax
	jae	.LBB32_584
.LBB32_429:
	.loc	1 1190 42
	movq	%r9, %rsi
	subq	%rbp, %rsi
	.loc	1 1190 29 is_stmt 0
	leaq	104(%r14), %rdi
	callq	_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_
	movl	%edx, %ebx
	movq	%rax, %r13
	vmovss	192(%r14), %xmm0
	vmovss	%xmm0, 72(%rsp)
	vmovss	200(%r14), %xmm0
	vmovss	%xmm0, 284(%rsp)
	vmovss	208(%r14), %xmm0
	vmovss	%xmm0, 512(%rsp)
	vmovss	216(%r14), %xmm0
	vmovss	%xmm0, 392(%rsp)
	vmovss	224(%r14), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	232(%r14), %xmm0
	vmovss	%xmm0, 388(%rsp)
	vmovss	240(%r14), %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	248(%r14), %xmm0
	vmovss	%xmm0, 384(%rsp)
	vmovss	256(%r14), %xmm0
	vmovss	%xmm0, 304(%rsp)
	vmovss	264(%r14), %xmm0
	vmovss	%xmm0, 380(%rsp)
	vmovss	272(%r14), %xmm0
	vmovss	%xmm0, 80(%rsp)
	vmovss	280(%r14), %xmm0
	vmovss	%xmm0, 404(%rsp)
	vmovss	288(%r14), %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	296(%r14), %xmm0
	vmovss	%xmm0, 376(%rsp)
	vmovss	304(%r14), %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	312(%r14), %xmm0
	vmovss	%xmm0, 372(%rsp)
	vmovss	320(%r14), %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	328(%r14), %xmm0
	vmovss	%xmm0, 368(%rsp)
	vmovss	336(%r14), %xmm0
	vmovss	%xmm0, 24(%rsp)
	vmovss	344(%r14), %xmm0
	vmovss	%xmm0, 364(%rsp)
	vmovss	552(%r14), %xmm0
	vmovss	%xmm0, 16(%rsp)
	vmovss	560(%r14), %xmm0
	vmovss	%xmm0, 400(%rsp)
	vmovss	568(%r14), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	576(%r14), %xmm0
	vmovss	%xmm0, 360(%rsp)
	vmovss	584(%r14), %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	592(%r14), %xmm0
	vmovss	%xmm0, 356(%rsp)
	vmovss	600(%r14), %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	608(%r14), %xmm0
	vmovss	%xmm0, 352(%rsp)
	vmovss	616(%r14), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	624(%r14), %xmm0
	vmovss	%xmm0, 348(%rsp)
	vmovss	632(%r14), %xmm0
	vmovss	%xmm0, 56(%rsp)
	vmovss	640(%r14), %xmm0
	vmovss	%xmm0, 396(%rsp)
	vmovss	648(%r14), %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	656(%r14), %xmm0
	vmovss	%xmm0, 344(%rsp)
	vmovss	664(%r14), %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	672(%r14), %xmm0
	vmovss	%xmm0, 340(%rsp)
	vmovss	680(%r14), %xmm0
	vmovss	%xmm0, 20(%rsp)
	vmovss	688(%r14), %xmm0
	vmovss	%xmm0, 336(%rsp)
	vmovss	696(%r14), %xmm0
	vmovss	%xmm0, 416(%rsp)
	vmovss	704(%r14), %xmm0
	vmovss	%xmm0, 332(%rsp)
.Ltmp2495:
	.loc	1 1194 31 is_stmt 1
	leaq	480(%rsp), %rdi
	leaq	120(%r14), %rsi
	movl	648(%rsp), %r15d
	movl	%r15d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1195 31
	leaq	696(%rsp), %rdi
	leaq	480(%r14), %rsi
	movl	%r15d, %edx
	callq	_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_
	.loc	1 1193 28
	vmovss	480(%rsp), %xmm0
	vmovss	%xmm0, 504(%rsp)
	vmovss	484(%rsp), %xmm0
	vmovaps	%xmm0, 624(%rsp)
	vmovss	488(%rsp), %xmm0
	vmovaps	%xmm0, 608(%rsp)
	vmovss	492(%rsp), %xmm0
	vmovss	%xmm0, 144(%rsp)
	vmovss	496(%rsp), %xmm0
	vmovaps	%xmm0, 592(%rsp)
	vmovss	500(%rsp), %xmm0
	vmovaps	%xmm0, 576(%rsp)
	vmovss	696(%rsp), %xmm0
	vmovss	%xmm0, 140(%rsp)
	vmovss	700(%rsp), %xmm0
	vmovaps	%xmm0, 560(%rsp)
	vmovss	704(%rsp), %xmm0
	vmovaps	%xmm0, 544(%rsp)
	vmovss	708(%rsp), %xmm0
	vmovss	%xmm0, 136(%rsp)
	vmovss	712(%rsp), %xmm0
	vmovaps	%xmm0, 528(%rsp)
	vmovss	716(%rsp), %xmm0
	vmovaps	%xmm0, 448(%rsp)
	movq	%r13, 128(%rsp)
.Ltmp2496:
	.loc	1 0 0 is_stmt 0
	addq	%rbp, %r13
	setb	%cl
	cmpq	152(%rsp), %r13
	seta	%al
.Ltmp2497:
	.loc	38 1050 16 is_stmt 1
	orb	%cl, %al
.Ltmp2498:
	.loc	1 1197 12
	testb	$1, %bl
	movq	%r13, 464(%rsp)
	je	.LBB32_506
.Ltmp2499:
	.loc	38 1050 16
	testb	%al, %al
	jne	.LBB32_645
.Ltmp2500:
	.loc	1 1053 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
.Ltmp2501:
	.loc	1 1054 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 288(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp2502:
	.loc	1 1055 25
	movl	184(%r14), %eax
	movl	%eax, 48(%rsp)
	movl	188(%r14), %ecx
.Ltmp2503:
	.loc	1 1056 24
	movl	544(%r14), %eax
	movl	%eax, 8(%rsp)
	movl	548(%r14), %r13d
.Ltmp2504:
	.loc	1 1057 24
	movq	848(%r14), %rdi
.Ltmp2505:
	.loc	2 1916 50
	cmpq	$0, 128(%rsp)
.Ltmp2506:
	.loc	3 900 12
	je	.LBB32_582
.Ltmp2507:
	.loc	3 0 12 is_stmt 0
	movq	440(%rsp), %rax
	leaq	(%rax,%rbp,4), %rbx
	movq	408(%rsp), %rax
	leaq	(%rax,%rbp,4), %r8
	xorl	%esi, %esi
	movl	%ecx, %ebp
	movq	%rbx, 664(%rsp)
	movq	%r8, 656(%rsp)
	.p2align	4
.LBB32_433:
.Ltmp2508:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp2509:
	.loc	1 857 8
	cmpq	%r12, %rax
	jb	.LBB32_435
.Ltmp2510:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %r15
	jmp	.LBB32_436
	.p2align	4
.LBB32_435:
	xorl	%r15d, %r15d
.LBB32_436:
.Ltmp2511:
	.loc	1 1083 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp2512:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_614
.Ltmp2513:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2514:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rbx,%rsi,4), %xmm0
.Ltmp2515:
	vmovss	(%r8,%rsi,4), %xmm3
.Ltmp2516:
	vmovss	152(%r14), %xmm2
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm4
	vmovaps	192(%rsp), %xmm9
	vsubss	%xmm9, %xmm0, %xmm1
	vmulss	%xmm7, %xmm1, %xmm5
	vmovaps	224(%rsp), %xmm8
	vmulss	%xmm2, %xmm8, %xmm6
	vaddss	%xmm5, %xmm6, %xmm5
	vaddss	%xmm5, %xmm8, %xmm6
	vmulss	%xmm7, %xmm8, %xmm8
	vmulss	%xmm4, %xmm1, %xmm1
	vaddss	%xmm1, %xmm8, %xmm1
	vmulss	164(%r14), %xmm6, %xmm13
	vmovss	%xmm1, 148(%rsp)
	vaddss	%xmm1, %xmm9, %xmm6
	vmovaps	240(%rsp), %xmm1
	vsubss	%xmm1, %xmm6, %xmm10
	vmulss	256(%rsp), %xmm7, %xmm6
	vmulss	%xmm4, %xmm10, %xmm4
	vaddss	%xmm4, %xmm6, %xmm4
	vaddss	%xmm4, %xmm1, %xmm15
.Ltmp2517:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm12
	vmovss	520(%r14), %xmm14
	vsubss	288(%rsp), %xmm3, %xmm6
	vmulss	%xmm6, %xmm12, %xmm8
	vmovaps	208(%rsp), %xmm1
	vmulss	%xmm1, %xmm11, %xmm9
	vaddss	%xmm8, %xmm9, %xmm8
	vaddss	%xmm1, %xmm8, %xmm9
	vmulss	524(%r14), %xmm9, %xmm9
.Ltmp2518:
	.loc	1 1083 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp2519:
	.loc	49 56 9
	vmovss	%xmm15, (%rax,%rdi,4)
.Ltmp2520:
	.loc	1 1084 30
	movq	144(%r14), %rdx
.Ltmp2521:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_615
.Ltmp2522:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2523:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm0, %xmm13, %xmm0
	vsubss	%xmm15, %xmm0, %xmm0
.Ltmp2524:
	.loc	1 1084 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp2525:
	.loc	49 56 9
	vmovss	%xmm0, (%rax,%rdi,4)
.Ltmp2526:
	.loc	1 1085 28
	movq	488(%r14), %rdx
.Ltmp2527:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_616
.Ltmp2528:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2529:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm1, %xmm12, %xmm0
	vmulss	%xmm6, %xmm14, %xmm6
	vaddss	%xmm6, %xmm0, %xmm13
	vaddss	288(%rsp), %xmm13, %xmm0
	vmovaps	176(%rsp), %xmm1
	vsubss	%xmm1, %xmm0, %xmm6
	vmulss	160(%rsp), %xmm12, %xmm0
	vmulss	%xmm6, %xmm14, %xmm14
	vaddss	%xmm0, %xmm14, %xmm0
	vaddss	%xmm0, %xmm1, %xmm14
.Ltmp2530:
	.loc	1 1085 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp2531:
	.loc	49 56 9
	vmovss	%xmm14, (%rax,%rdi,4)
.Ltmp2532:
	.loc	1 1086 29
	movq	504(%r14), %rdx
.Ltmp2533:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	vbroadcastss	.LCPI32_1(%rip), %xmm15
	ja	.LBB32_617
.Ltmp2534:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2535:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm3, %xmm9, %xmm3
	vsubss	%xmm14, %xmm3, %xmm3
.Ltmp2536:
	.loc	1 1086 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp2537:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
	movq	400(%r14), %r10
.Ltmp2538:
	.loc	1 947 24
	addq	%rdi, %r10
.Ltmp2539:
	.loc	1 857 8
	cmpq	%r12, %r10
	jb	.LBB32_446
.Ltmp2540:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rax
	jmp	.LBB32_447
	.p2align	4
.LBB32_446:
	xorl	%eax, %eax
.LBB32_447:
	movq	128(%r14), %rdx
.Ltmp2541:
	.loc	1 857 8 is_stmt 1
	subq	%rax, %r10
.Ltmp2542:
	.loc	48 568 12
	cmpq	%rdx, %r10
	ja	.LBB32_626
.Ltmp2543:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2544:
	.loc	1 1096 13
	movq	144(%r14), %rax
.Ltmp2545:
	.loc	48 568 12
	cmpq	%rax, %r10
	ja	.LBB32_627
.Ltmp2546:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2547:
	.loc	49 0 9 is_stmt 0
	movq	760(%r14), %r9
.Ltmp2548:
	.loc	1 947 24 is_stmt 1
	addq	%rdi, %r9
.Ltmp2549:
	.loc	1 857 8
	cmpq	%r12, %r9
	jb	.LBB32_453
.Ltmp2550:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rcx
	jmp	.LBB32_454
	.p2align	4
.LBB32_453:
	xorl	%ecx, %ecx
.LBB32_454:
	movq	488(%r14), %r11
.Ltmp2551:
	.loc	1 857 8 is_stmt 1
	subq	%rcx, %r9
.Ltmp2552:
	.loc	48 568 12
	cmpq	%r11, %r9
	ja	.LBB32_628
.Ltmp2553:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2554:
	.loc	1 1110 13
	movq	504(%r14), %r8
.Ltmp2555:
	.loc	48 568 12
	cmpq	%r8, %r9
	ja	.LBB32_629
.Ltmp2556:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2557:
	.loc	49 0 9 is_stmt 0
	movl	%r13d, 68(%rsp)
	movq	120(%r14), %rcx
	movq	%rcx, 680(%rsp)
	vmovss	(%rcx,%r10,4), %xmm3
.Ltmp2558:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2559:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %r12d
	movl	$841731191, %ebx
.Ltmp2560:
	.loc	49 161 24
	jbe	.LBB32_460
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %ebx
.LBB32_460:
.Ltmp2561:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebx, %xmm3
.Ltmp2562:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %r13d
	movl	$8388608, %ecx
.Ltmp2563:
	.loc	49 161 24
	jbe	.LBB32_462
.Ltmp2564:
	.loc	49 0 24 is_stmt 0
	movl	%ebx, %ecx
.LBB32_462:
.Ltmp2565:
	.loc	49 185 42 is_stmt 1
	movl	%ecx, %ebx
	andl	$8388607, %ebx
	orl	$1065353216, %ebx
.Ltmp2566:
	.loc	23 1291 18
	vmovd	%ebx, %xmm3
.Ltmp2567:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2568:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm9
.Ltmp2569:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2570:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2571:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2572:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2573:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2574:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2575:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2576:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2577:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2578:
	.loc	49 187 28
	shrl	$23, %ecx
	orl	$1258291200, %ecx
.Ltmp2579:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm3
.Ltmp2580:
	.loc	23 1291 18
	vmovd	%ecx, %xmm9
.Ltmp2581:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2582:
	.loc	49 61 9
	vaddss	%xmm3, %xmm9, %xmm3
	vmovss	72(%rsp), %xmm9
.Ltmp2583:
	.loc	1 0 0 is_stmt 0
	vaddss	284(%rsp), %xmm9, %xmm9
.Ltmp2584:
	.loc	49 71 9 is_stmt 1
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2585:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2586:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm9, 72(%rsp)
.Ltmp2587:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2588:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm3, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_464
.Ltmp2589:
	.loc	49 0 44
	xorl	%ecx, %ecx
	jmp	.LBB32_465
	.p2align	4
.LBB32_464:
	vmovss	.LCPI32_17(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm9
.Ltmp2590:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2591:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm14, %xmm14
	vblendvps	%xmm14, %xmm9, %xmm3, %xmm3
	vmulss	504(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %ecx
.Ltmp2592:
.LBB32_465:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ecx, %xmm3
.Ltmp2593:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2594:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm3, %xmm3
.Ltmp2595:
	.loc	1 1005 9 is_stmt 1
	vmovss	48(%rsp), %xmm9
.Ltmp2596:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm3, %xmm14
	vmovaps	624(%rsp), %xmm1
	vblendvps	%xmm14, 608(%rsp), %xmm1, %xmm14
.Ltmp2597:
	.loc	49 66 9
	vsubss	%xmm3, %xmm9, %xmm9
.Ltmp2598:
	.loc	49 92 9
	vmulss	%xmm14, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp2599:
	.loc	49 103 24
	vandps	%xmm3, %xmm15, %xmm9
.Ltmp2600:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm9, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_467
	.loc	49 0 24
	movl	$0, 48(%rsp)
	jmp	.LBB32_468
	.p2align	4
.LBB32_467:
	.loc	49 166 0
	vmovss	%xmm3, 48(%rsp)
.Ltmp2601:
.LBB32_468:
	.loc	1 0 0
	movq	136(%r14), %rcx
	movq	%rcx, 672(%rsp)
	vmovss	(%rcx,%r10,4), %xmm3
.Ltmp2602:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2603:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %ebx
.Ltmp2604:
	.loc	49 161 24
	jbe	.LBB32_470
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %ebx
.LBB32_470:
.Ltmp2605:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebx, %xmm3
.Ltmp2606:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %r10d
.Ltmp2607:
	.loc	49 161 24
	jbe	.LBB32_472
.Ltmp2608:
	.loc	49 0 24 is_stmt 0
	movl	%ebx, %r10d
.LBB32_472:
.Ltmp2609:
	.loc	49 185 42 is_stmt 1
	movl	%r10d, %ebx
	andl	$8388607, %ebx
	orl	$1065353216, %ebx
.Ltmp2610:
	.loc	23 1291 18
	vmovd	%ebx, %xmm3
.Ltmp2611:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2612:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm9
.Ltmp2613:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2614:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2615:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2616:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2617:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2618:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2619:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2620:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2621:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2622:
	.loc	49 187 28
	shrl	$23, %r10d
	orl	$1258291200, %r10d
.Ltmp2623:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm3
.Ltmp2624:
	.loc	23 1291 18
	vmovd	%r10d, %xmm9
.Ltmp2625:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2626:
	.loc	49 61 9
	vaddss	%xmm3, %xmm9, %xmm3
	vmovss	80(%rsp), %xmm9
.Ltmp2627:
	.loc	1 0 0 is_stmt 0
	vaddss	404(%rsp), %xmm9, %xmm9
.Ltmp2628:
	.loc	49 71 9 is_stmt 1
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2629:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2630:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm9, 80(%rsp)
.Ltmp2631:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2632:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm3, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_474
.Ltmp2633:
	.loc	49 0 44
	xorl	%r10d, %r10d
	jmp	.LBB32_475
	.p2align	4
.LBB32_474:
	vmovss	.LCPI32_17(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm9
.Ltmp2634:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2635:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm14, %xmm14
	vblendvps	%xmm14, %xmm9, %xmm3, %xmm3
	vmulss	144(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %r10d
.Ltmp2636:
.LBB32_475:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm3
.Ltmp2637:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2638:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm3, %xmm3
.Ltmp2639:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ebp, %xmm9
.Ltmp2640:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm3, %xmm14
	vmovaps	592(%rsp), %xmm1
	vblendvps	%xmm14, 576(%rsp), %xmm1, %xmm14
.Ltmp2641:
	.loc	49 66 9
	vsubss	%xmm3, %xmm9, %xmm9
.Ltmp2642:
	.loc	49 92 9
	vmulss	%xmm14, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp2643:
	.loc	49 103 24
	vandps	%xmm3, %xmm15, %xmm9
.Ltmp2644:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm9, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_477
	.loc	49 0 24
	xorl	%ecx, %ecx
	jmp	.LBB32_478
	.p2align	4
.LBB32_477:
	.loc	49 166 0
	vmovd	%xmm3, %ecx
.Ltmp2645:
.LBB32_478:
	.loc	1 0 0
	movq	480(%r14), %r10
	vmovss	(%r10,%r9,4), %xmm3
.Ltmp2646:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2647:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
	movl	$841731191, %ebp
.Ltmp2648:
	.loc	49 161 24
	jbe	.LBB32_480
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %ebp
.LBB32_480:
.Ltmp2649:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebp, %xmm3
.Ltmp2650:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
	movl	$8388608, %ebx
.Ltmp2651:
	.loc	49 161 24
	jbe	.LBB32_482
.Ltmp2652:
	.loc	49 0 24 is_stmt 0
	movl	%ebp, %ebx
.LBB32_482:
.Ltmp2653:
	.loc	49 185 42 is_stmt 1
	movl	%ebx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2654:
	.loc	23 1291 18
	vmovd	%ebp, %xmm3
.Ltmp2655:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2656:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm9
.Ltmp2657:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2658:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2659:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2660:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2661:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2662:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2663:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2664:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2665:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2666:
	.loc	49 187 28
	shrl	$23, %ebx
	orl	$1258291200, %ebx
.Ltmp2667:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm3
.Ltmp2668:
	.loc	23 1291 18
	vmovd	%ebx, %xmm9
.Ltmp2669:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2670:
	.loc	49 61 9
	vaddss	%xmm3, %xmm9, %xmm3
	vmovss	16(%rsp), %xmm9
.Ltmp2671:
	.loc	1 0 0 is_stmt 0
	vaddss	400(%rsp), %xmm9, %xmm9
.Ltmp2672:
	movq	496(%r14), %rbx
.Ltmp2673:
	.loc	49 71 9 is_stmt 1
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2674:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2675:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm9, 16(%rsp)
.Ltmp2676:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2677:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm3, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_484
.Ltmp2678:
	.loc	49 0 44
	xorl	%ebp, %ebp
	jmp	.LBB32_485
	.p2align	4
.LBB32_484:
	vmovss	.LCPI32_17(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm9
.Ltmp2679:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2680:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm14, %xmm14
	vblendvps	%xmm14, %xmm9, %xmm3, %xmm3
	vmulss	140(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %ebp
.Ltmp2681:
.LBB32_485:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebp, %xmm3
.Ltmp2682:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2683:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm3, %xmm3
.Ltmp2684:
	.loc	1 1005 9 is_stmt 1
	vmovss	8(%rsp), %xmm9
.Ltmp2685:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm3, %xmm14
	vmovaps	560(%rsp), %xmm1
	vblendvps	%xmm14, 544(%rsp), %xmm1, %xmm14
.Ltmp2686:
	.loc	49 66 9
	vsubss	%xmm3, %xmm9, %xmm9
.Ltmp2687:
	.loc	49 92 9
	vmulss	%xmm14, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp2688:
	.loc	49 103 24
	vandps	%xmm3, %xmm15, %xmm9
.Ltmp2689:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm9, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_487
.Ltmp2690:
	.loc	49 0 24
	movl	$0, 8(%rsp)
.Ltmp2691:
	.loc	49 103 24 is_stmt 1
	vmovss	(%rbx,%r9,4), %xmm3
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2692:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
.Ltmp2693:
	.loc	49 161 24
	ja	.LBB32_488
	jmp	.LBB32_489
.Ltmp2694:
	.loc	49 0 24 is_stmt 0
.Ltmp2695:
	.p2align	4
.LBB32_487:
	.loc	49 166 0 is_stmt 1
	vmovss	%xmm3, 8(%rsp)
.Ltmp2696:
	.loc	49 103 24
	vmovss	(%rbx,%r9,4), %xmm3
	vandps	%xmm3, %xmm15, %xmm3
.Ltmp2697:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm3
.Ltmp2698:
	.loc	49 161 24
	jbe	.LBB32_489
.LBB32_488:
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm3, %r12d
.LBB32_489:
.Ltmp2699:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r12d, %xmm3
.Ltmp2700:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm3
.Ltmp2701:
	.loc	49 161 24
	jbe	.LBB32_491
.Ltmp2702:
	.loc	49 0 24 is_stmt 0
	movl	%r12d, %r13d
.LBB32_491:
.Ltmp2703:
	.loc	49 185 42 is_stmt 1
	movl	%r13d, %r9d
	andl	$8388607, %r9d
	orl	$1065353216, %r9d
.Ltmp2704:
	.loc	23 1291 18
	vmovd	%r9d, %xmm3
.Ltmp2705:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm3, %xmm3
.Ltmp2706:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm3, %xmm9
.Ltmp2707:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2708:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2709:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2710:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2711:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2712:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2713:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2714:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp2715:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2716:
	.loc	49 187 28
	shrl	$23, %r13d
	orl	$1258291200, %r13d
.Ltmp2717:
	.loc	49 71 9
	vmulss	%xmm3, %xmm9, %xmm3
.Ltmp2718:
	.loc	23 1291 18
	vmovd	%r13d, %xmm9
.Ltmp2719:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm9, %xmm9
.Ltmp2720:
	.loc	49 61 9
	vaddss	%xmm3, %xmm9, %xmm3
	vmovss	56(%rsp), %xmm9
.Ltmp2721:
	.loc	1 0 0 is_stmt 0
	vaddss	396(%rsp), %xmm9, %xmm9
.Ltmp2722:
	.loc	49 71 9 is_stmt 1
	vmulss	.LCPI32_14(%rip), %xmm3, %xmm3
.Ltmp2723:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm3, %xmm3
.Ltmp2724:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm3, %xmm3
	vmovss	%xmm9, 56(%rsp)
.Ltmp2725:
	.loc	49 66 9 is_stmt 1
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2726:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm3, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_493
.Ltmp2727:
	.loc	49 0 44
	xorl	%r9d, %r9d
	jmp	.LBB32_494
	.p2align	4
.LBB32_493:
	vmovss	.LCPI32_17(%rip), %xmm14
	vaddss	%xmm3, %xmm14, %xmm9
.Ltmp2728:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2729:
	.loc	49 161 0
	vcmpnltss	%xmm3, %xmm14, %xmm14
	vblendvps	%xmm14, %xmm9, %xmm3, %xmm3
	vmulss	136(%rsp), %xmm3, %xmm3
	vmovd	%xmm3, %r9d
.Ltmp2730:
.LBB32_494:
	.loc	49 0 0
	movq	320(%rsp), %r12
	movl	68(%rsp), %ebp
.Ltmp2731:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r9d, %xmm3
.Ltmp2732:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm3, %xmm3
.Ltmp2733:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm3, %xmm3
.Ltmp2734:
	.loc	1 1005 9 is_stmt 1
	vmovd	%ebp, %xmm9
.Ltmp2735:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm3, %xmm14
	vmovaps	528(%rsp), %xmm1
	vblendvps	%xmm14, 448(%rsp), %xmm1, %xmm14
.Ltmp2736:
	.loc	49 66 9
	vsubss	%xmm3, %xmm9, %xmm9
.Ltmp2737:
	.loc	49 92 9
	vmulss	%xmm14, %xmm9, %xmm9
	vaddss	%xmm3, %xmm9, %xmm3
.Ltmp2738:
	.loc	49 103 24
	vandps	%xmm3, %xmm15, %xmm9
.Ltmp2739:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm14
	vucomiss	%xmm9, %xmm14
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_496
	.loc	49 0 24
	xorl	%r13d, %r13d
	jmp	.LBB32_497
	.p2align	4
.LBB32_496:
	.loc	49 166 0
	vmovd	%xmm3, %r13d
.Ltmp2740:
.LBB32_497:
	.loc	49 0 0
	movl	%ecx, %ebp
	negq	%r15
	addq	%r15, %rdi
	incq	%rdi
.Ltmp2741:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp2742:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2743:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_619
.Ltmp2744:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2745:
	.loc	48 568 12
	cmpq	%r11, %rdi
	ja	.LBB32_620
.Ltmp2746:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2747:
	.loc	48 568 12
	cmpq	%r8, %rdi
	ja	.LBB32_621
.Ltmp2748:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2749:
	.loc	1 0 0 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm3
	vmovaps	256(%rsp), %xmm7
	vmulss	%xmm2, %xmm7, %xmm2
	vaddss	%xmm3, %xmm2, %xmm2
	vmulss	%xmm6, %xmm12, %xmm3
	vmovaps	160(%rsp), %xmm12
	vmulss	%xmm11, %xmm12, %xmm6
	vaddss	%xmm3, %xmm6, %xmm6
	vaddss	%xmm5, %xmm5, %xmm3
	vaddss	224(%rsp), %xmm3, %xmm5
	vmovss	148(%rsp), %xmm1
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	192(%rsp), %xmm1, %xmm3
	vaddss	%xmm2, %xmm2, %xmm1
	vaddss	%xmm4, %xmm4, %xmm2
	vaddss	%xmm8, %xmm8, %xmm4
	vaddss	%xmm13, %xmm13, %xmm8
	vaddss	%xmm6, %xmm6, %xmm9
	vaddss	%xmm0, %xmm0, %xmm10
	vaddss	%xmm1, %xmm7, %xmm7
	vaddss	240(%rsp), %xmm2, %xmm6
	vaddss	208(%rsp), %xmm4, %xmm4
	vaddss	288(%rsp), %xmm8, %xmm0
	vaddss	%xmm9, %xmm12, %xmm1
	vaddss	176(%rsp), %xmm10, %xmm2
	vmovss	304(%rsp), %xmm9
	vaddss	380(%rsp), %xmm9, %xmm9
	vmovss	48(%rsp), %xmm8
	vmovss	%xmm9, 304(%rsp)
	vaddss	%xmm8, %xmm9, %xmm8
	vmovss	24(%rsp), %xmm10
	vaddss	364(%rsp), %xmm10, %xmm10
	vmovd	%ebp, %xmm9
	vmovss	%xmm10, 24(%rsp)
	vaddss	%xmm9, %xmm10, %xmm9
	vmovss	28(%rsp), %xmm11
	vaddss	348(%rsp), %xmm11, %xmm11
	vmovss	8(%rsp), %xmm10
	vmovss	%xmm11, 28(%rsp)
	vaddss	%xmm10, %xmm11, %xmm10
	vandps	%xmm5, %xmm15, %xmm11
	vandps	%xmm3, %xmm15, %xmm12
	vandps	%xmm7, %xmm15, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm14
	vcmpnltss	%xmm14, %xmm11, %xmm11
	vandps	%xmm5, %xmm11, %xmm5
	vmovaps	%xmm5, 224(%rsp)
	vandps	%xmm6, %xmm15, %xmm5
	vcmpnltss	%xmm14, %xmm12, %xmm11
	vandps	%xmm3, %xmm11, %xmm3
	vmovaps	%xmm3, 192(%rsp)
	vandps	%xmm4, %xmm15, %xmm3
	vcmpnltss	%xmm14, %xmm13, %xmm11
	vandps	%xmm7, %xmm11, %xmm7
	vmovaps	%xmm7, 256(%rsp)
	vandps	%xmm0, %xmm15, %xmm7
	vcmpnltss	%xmm14, %xmm5, %xmm5
	vandps	%xmm6, %xmm5, %xmm5
	vmovaps	%xmm5, 240(%rsp)
	vandps	%xmm1, %xmm15, %xmm5
	vcmpnltss	%xmm14, %xmm3, %xmm3
	vandps	%xmm4, %xmm3, %xmm3
	vmovaps	%xmm3, 208(%rsp)
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
	vmovaps	%xmm0, 288(%rsp)
	vandps	%xmm1, %xmm5, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vandps	%xmm2, %xmm3, %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm11
	vminss	%xmm11, %xmm4, %xmm3
	vminss	%xmm11, %xmm6, %xmm2
	vminss	%xmm11, %xmm8, %xmm4
	vroundss	$9, %xmm3, %xmm3, %xmm1
	vroundss	$9, %xmm2, %xmm2, %xmm9
	vsubss	%xmm1, %xmm3, %xmm3
	vmovss	.LCPI32_24(%rip), %xmm8
.Ltmp2750:
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
.Ltmp2751:
	vroundss	$9, %xmm4, %xmm4, %xmm3
	vsubss	%xmm9, %xmm2, %xmm6
	vsubss	%xmm3, %xmm4, %xmm2
	vmovss	416(%rsp), %xmm0
	vaddss	332(%rsp), %xmm0, %xmm0
	vmovss	.LCPI32_29(%rip), %xmm15
.Ltmp2752:
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
.Ltmp2753:
	vmulss	%xmm4, %xmm6, %xmm4
.Ltmp2754:
	vmovd	%r13d, %xmm5
	vmovss	%xmm0, 416(%rsp)
	vaddss	%xmm5, %xmm0, %xmm5
	vmulss	.LCPI32_21(%rip), %xmm5, %xmm5
	vmaxss	.LCPI32_22(%rip), %xmm5, %xmm5
	vminss	%xmm11, %xmm5, %xmm5
.Ltmp2755:
	vaddss	%xmm4, %xmm15, %xmm4
	vaddss	%xmm7, %xmm9, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm4, %xmm0
.Ltmp2756:
	vmulss	%xmm2, %xmm8, %xmm4
	vaddss	%xmm4, %xmm10, %xmm4
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm4, %xmm12, %xmm4
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
	vmulss	%xmm4, %xmm2, %xmm4
	vaddss	%xmm4, %xmm14, %xmm4
.Ltmp2757:
	vmulss	%xmm4, %xmm2, %xmm2
.Ltmp2758:
	vroundss	$9, %xmm5, %xmm5, %xmm4
	vsubss	%xmm4, %xmm5, %xmm5
.Ltmp2759:
	vaddss	%xmm2, %xmm15, %xmm2
	vaddss	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm3
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2760:
	vmulss	%xmm5, %xmm8, %xmm3
	vaddss	%xmm3, %xmm10, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm12, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp2761:
	vmulss	%xmm3, %xmm5, %xmm3
	vaddss	%xmm3, %xmm15, %xmm3
	vaddss	%xmm7, %xmm4, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	movq	680(%rsp), %rax
.Ltmp2762:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
	movq	672(%rsp), %rax
	vmulss	(%rax,%rdi,4), %xmm0, %xmm0
.Ltmp2763:
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp2764:
	vaddss	%xmm0, %xmm1, %xmm0
.Ltmp2765:
	vmulss	(%r10,%rdi,4), %xmm2, %xmm1
.Ltmp2766:
	.loc	49 71 9 is_stmt 1
	vmulss	(%rbx,%rdi,4), %xmm3, %xmm2
.Ltmp2767:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	664(%rsp), %rbx
.Ltmp2768:
	.loc	49 56 9
	vmovss	%xmm0, (%rbx,%rsi,4)
	movq	656(%rsp), %r8
.Ltmp2769:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r8,%rsi,4)
	vmovss	512(%rsp), %xmm0
.Ltmp2770:
	.loc	1 0 0
	vaddss	392(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 512(%rsp)
	vmovss	96(%rsp), %xmm0
	vaddss	360(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	32(%rsp), %xmm0
	vaddss	388(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	104(%rsp), %xmm0
	vaddss	356(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 104(%rsp)
	vmovss	36(%rsp), %xmm0
	vaddss	384(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 36(%rsp)
	vmovss	112(%rsp), %xmm0
	vaddss	352(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	40(%rsp), %xmm0
	vaddss	376(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	120(%rsp), %xmm0
	vaddss	344(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	44(%rsp), %xmm0
	vaddss	372(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	12(%rsp), %xmm0
	vaddss	340(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 12(%rsp)
	vmovss	88(%rsp), %xmm0
	vaddss	368(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 88(%rsp)
	vmovss	20(%rsp), %xmm0
	vaddss	336(%rsp), %xmm0, %xmm0
	vmovss	%xmm0, 20(%rsp)
	incq	%rsi
.Ltmp2771:
	.loc	2 1916 50 is_stmt 1
	cmpq	%rsi, 128(%rsp)
.Ltmp2772:
	.loc	3 900 12
	jne	.LBB32_433
	jmp	.LBB32_583
.Ltmp2773:
	.loc	3 0 12 is_stmt 0
.Ltmp2774:
	.p2align	4
.LBB32_506:
	.loc	38 1050 16 is_stmt 1
	testb	%al, %al
	jne	.LBB32_646
.Ltmp2775:
	.loc	1 1053 27
	vmovss	168(%r14), %xmm0
	vmovaps	%xmm0, 224(%rsp)
	vmovss	172(%r14), %xmm0
	vmovaps	%xmm0, 192(%rsp)
	vmovss	176(%r14), %xmm0
	vmovaps	%xmm0, 256(%rsp)
	vmovss	180(%r14), %xmm0
	vmovaps	%xmm0, 240(%rsp)
.Ltmp2776:
	.loc	1 1054 26
	vmovss	528(%r14), %xmm0
	vmovaps	%xmm0, 208(%rsp)
	vmovss	532(%r14), %xmm0
	vmovaps	%xmm0, 288(%rsp)
	vmovss	536(%r14), %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vmovss	540(%r14), %xmm0
	vmovaps	%xmm0, 176(%rsp)
.Ltmp2777:
	.loc	1 1055 25
	movl	184(%r14), %eax
	movl	%eax, 48(%rsp)
	movl	188(%r14), %eax
	movl	%eax, 8(%rsp)
.Ltmp2778:
	.loc	1 1056 24
	movl	544(%r14), %ecx
	movl	548(%r14), %esi
.Ltmp2779:
	.loc	1 1057 24
	movq	848(%r14), %rdi
.Ltmp2780:
	.loc	2 1916 50
	cmpq	$0, 128(%rsp)
.Ltmp2781:
	.loc	3 900 12
	je	.LBB32_426
.Ltmp2782:
	.loc	3 0 12 is_stmt 0
	movq	440(%rsp), %rax
	leaq	(%rax,%rbp,4), %r8
	movq	408(%rsp), %rax
	leaq	(%rax,%rbp,4), %r9
	xorl	%r13d, %r13d
	movl	%ecx, %ebp
	movq	%r9, 96(%rsp)
	movq	%r8, 88(%rsp)
	.p2align	4
.LBB32_509:
.Ltmp2783:
	.loc	1 1070 28 is_stmt 1
	leaq	1(%rdi), %rax
.Ltmp2784:
	.loc	1 857 8
	cmpq	%r12, %rax
	jb	.LBB32_511
.Ltmp2785:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rbx
	jmp	.LBB32_512
	.p2align	4
.LBB32_511:
	xorl	%ebx, %ebx
.LBB32_512:
.Ltmp2786:
	.loc	1 1083 29 is_stmt 1
	movq	128(%r14), %rdx
.Ltmp2787:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_614
.Ltmp2788:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2789:
	.loc	1 0 0 is_stmt 0
	vmovss	(%r8,%r13,4), %xmm3
.Ltmp2790:
	vmovss	(%r9,%r13,4), %xmm13
.Ltmp2791:
	vmovss	152(%r14), %xmm2
	vmovss	156(%r14), %xmm7
	vmovss	160(%r14), %xmm0
	vmovaps	192(%rsp), %xmm8
	vsubss	%xmm8, %xmm3, %xmm1
	vmulss	%xmm7, %xmm1, %xmm4
	vmovaps	224(%rsp), %xmm6
	vmulss	%xmm2, %xmm6, %xmm5
	vaddss	%xmm4, %xmm5, %xmm5
	vaddss	%xmm5, %xmm6, %xmm4
	vmulss	%xmm7, %xmm6, %xmm6
	vmulss	%xmm0, %xmm1, %xmm1
	vaddss	%xmm1, %xmm6, %xmm1
	vmulss	164(%r14), %xmm4, %xmm6
	vmovss	%xmm1, 20(%rsp)
	vaddss	%xmm1, %xmm8, %xmm4
	vmovaps	240(%rsp), %xmm1
	vsubss	%xmm1, %xmm4, %xmm10
	vmulss	256(%rsp), %xmm7, %xmm4
	vmulss	%xmm0, %xmm10, %xmm0
	vaddss	%xmm0, %xmm4, %xmm4
	vaddss	%xmm4, %xmm1, %xmm9
.Ltmp2792:
	vmovss	512(%r14), %xmm11
	vmovss	516(%r14), %xmm0
	vmovss	520(%r14), %xmm12
	vsubss	288(%rsp), %xmm13, %xmm14
	vmulss	%xmm0, %xmm14, %xmm8
	vmovaps	208(%rsp), %xmm1
	vmulss	%xmm1, %xmm11, %xmm15
	vaddss	%xmm8, %xmm15, %xmm8
	vaddss	%xmm1, %xmm8, %xmm15
	vmulss	524(%r14), %xmm15, %xmm15
.Ltmp2793:
	.loc	1 1083 0 is_stmt 1
	leaq	120(%r14), %rax
	movq	(%rax), %rax
.Ltmp2794:
	.loc	49 56 9
	vmovss	%xmm9, (%rax,%rdi,4)
.Ltmp2795:
	.loc	1 1084 30
	movq	144(%r14), %rdx
.Ltmp2796:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_615
.Ltmp2797:
	.loc	49 56 9
	je	.LBB32_647
.Ltmp2798:
	.loc	1 0 0 is_stmt 0
	vaddss	%xmm6, %xmm3, %xmm3
	vsubss	%xmm9, %xmm3, %xmm3
.Ltmp2799:
	.loc	1 1084 0 is_stmt 1
	movq	136(%r14), %rax
.Ltmp2800:
	.loc	49 56 9
	vmovss	%xmm3, (%rax,%rdi,4)
.Ltmp2801:
	.loc	1 1085 28
	movq	488(%r14), %rdx
.Ltmp2802:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_616
.Ltmp2803:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_647
	vmulss	%xmm0, %xmm1, %xmm3
	vmulss	%xmm12, %xmm14, %xmm6
	vaddss	%xmm6, %xmm3, %xmm3
	vaddss	288(%rsp), %xmm3, %xmm6
	vmovaps	176(%rsp), %xmm1
	vsubss	%xmm1, %xmm6, %xmm14
	vmulss	160(%rsp), %xmm0, %xmm6
	vmulss	%xmm14, %xmm12, %xmm9
	vaddss	%xmm6, %xmm9, %xmm12
	vaddss	%xmm1, %xmm12, %xmm6
.Ltmp2804:
	.loc	1 1085 0 is_stmt 1
	leaq	480(%r14), %rax
	movq	(%rax), %rax
.Ltmp2805:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
.Ltmp2806:
	.loc	1 1086 29
	movq	504(%r14), %rdx
.Ltmp2807:
	.loc	48 580 12
	cmpq	%rdx, %rdi
	ja	.LBB32_617
.Ltmp2808:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_647
	vaddss	%xmm15, %xmm13, %xmm9
	vsubss	%xmm6, %xmm9, %xmm6
.Ltmp2809:
	.loc	1 1086 0 is_stmt 1
	movq	496(%r14), %rax
.Ltmp2810:
	.loc	49 56 9
	vmovss	%xmm6, (%rax,%rdi,4)
	movq	400(%r14), %r10
.Ltmp2811:
	.loc	1 947 24
	addq	%rdi, %r10
.Ltmp2812:
	.loc	1 857 8
	cmpq	%r12, %r10
	jb	.LBB32_522
.Ltmp2813:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rax
	jmp	.LBB32_523
	.p2align	4
.LBB32_522:
	xorl	%eax, %eax
.LBB32_523:
	vbroadcastss	.LCPI32_1(%rip), %xmm15
	movq	128(%r14), %rdx
.Ltmp2814:
	.loc	1 857 8 is_stmt 1
	subq	%rax, %r10
.Ltmp2815:
	.loc	48 568 12
	cmpq	%rdx, %r10
	ja	.LBB32_626
.Ltmp2816:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2817:
	.loc	1 1096 13
	movq	144(%r14), %rax
.Ltmp2818:
	.loc	48 568 12
	cmpq	%rax, %r10
	ja	.LBB32_627
.Ltmp2819:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2820:
	.loc	49 0 9 is_stmt 0
	movq	760(%r14), %r9
.Ltmp2821:
	.loc	1 947 24 is_stmt 1
	addq	%rdi, %r9
.Ltmp2822:
	.loc	1 857 8
	cmpq	%r12, %r9
	jb	.LBB32_529
.Ltmp2823:
	.loc	1 0 8 is_stmt 0
	movq	%r12, %rcx
	jmp	.LBB32_530
	.p2align	4
.LBB32_529:
	xorl	%ecx, %ecx
.LBB32_530:
	movq	488(%r14), %r11
.Ltmp2824:
	.loc	1 857 8 is_stmt 1
	subq	%rcx, %r9
.Ltmp2825:
	.loc	48 568 12
	cmpq	%r11, %r9
	ja	.LBB32_628
.Ltmp2826:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2827:
	.loc	1 1110 13
	movq	504(%r14), %r8
.Ltmp2828:
	.loc	48 568 12
	cmpq	%r8, %r9
	ja	.LBB32_629
.Ltmp2829:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp2830:
	.loc	49 0 9 is_stmt 0
	movl	%esi, 12(%rsp)
	movq	120(%r14), %rcx
	movq	%rcx, 120(%rsp)
	vmovss	(%rcx,%r10,4), %xmm6
.Ltmp2831:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2832:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
	movl	$841731191, %r12d
	movl	$841731191, %esi
.Ltmp2833:
	.loc	49 161 24
	jbe	.LBB32_536
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %esi
.LBB32_536:
.Ltmp2834:
	.loc	23 1291 18 is_stmt 1
	vmovd	%esi, %xmm6
.Ltmp2835:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
	movl	$8388608, %r15d
	movl	$8388608, %ecx
.Ltmp2836:
	.loc	49 161 24
	jbe	.LBB32_538
.Ltmp2837:
	.loc	49 0 24 is_stmt 0
	movl	%esi, %ecx
.LBB32_538:
.Ltmp2838:
	.loc	49 185 42 is_stmt 1
	movl	%ecx, %esi
	andl	$8388607, %esi
	orl	$1065353216, %esi
.Ltmp2839:
	.loc	23 1291 18
	vmovd	%esi, %xmm6
.Ltmp2840:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2841:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2842:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2843:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2844:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2845:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2846:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2847:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2848:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2849:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2850:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2851:
	.loc	49 187 28
	shrl	$23, %ecx
	orl	$1258291200, %ecx
.Ltmp2852:
	.loc	23 1291 18
	vmovd	%ecx, %xmm13
.Ltmp2853:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm13, %xmm13
.Ltmp2854:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2855:
	.loc	49 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2856:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2857:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2858:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm6
.Ltmp2859:
	.loc	49 66 9 is_stmt 1
	vsubss	72(%rsp), %xmm6, %xmm6
.Ltmp2860:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm6, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_540
.Ltmp2861:
	.loc	49 0 44
	xorl	%ecx, %ecx
	jmp	.LBB32_541
	.p2align	4
.LBB32_540:
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm6, %xmm13, %xmm9
.Ltmp2862:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2863:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	504(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %ecx
.Ltmp2864:
.LBB32_541:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ecx, %xmm6
.Ltmp2865:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp2866:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm6, %xmm6
.Ltmp2867:
	.loc	1 1005 9 is_stmt 1
	vmovss	48(%rsp), %xmm9
.Ltmp2868:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	624(%rsp), %xmm1
	vblendvps	%xmm13, 608(%rsp), %xmm1, %xmm13
.Ltmp2869:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp2870:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2871:
	.loc	49 103 24
	vandps	%xmm6, %xmm15, %xmm9
.Ltmp2872:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm9, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_543
	.loc	49 0 24
	movl	$0, 48(%rsp)
	jmp	.LBB32_544
	.p2align	4
.LBB32_543:
	.loc	49 166 0
	vmovss	%xmm6, 48(%rsp)
.Ltmp2873:
.LBB32_544:
	.loc	1 0 0
	movq	136(%r14), %rcx
	movq	%rcx, 112(%rsp)
	vmovss	(%rcx,%r10,4), %xmm6
.Ltmp2874:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2875:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
	movl	$841731191, %r10d
.Ltmp2876:
	.loc	49 161 24
	jbe	.LBB32_546
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %r10d
.LBB32_546:
.Ltmp2877:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r10d, %xmm6
.Ltmp2878:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
	movl	$8388608, %esi
.Ltmp2879:
	.loc	49 161 24
	jbe	.LBB32_548
.Ltmp2880:
	.loc	49 0 24 is_stmt 0
	movl	%r10d, %esi
.LBB32_548:
.Ltmp2881:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %r10d
	andl	$8388607, %r10d
	orl	$1065353216, %r10d
.Ltmp2882:
	.loc	23 1291 18
	vmovd	%r10d, %xmm6
.Ltmp2883:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2884:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2885:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2886:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2887:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2888:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2889:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2890:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2891:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2892:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2893:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2894:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp2895:
	.loc	23 1291 18
	vmovd	%esi, %xmm13
.Ltmp2896:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm13, %xmm13
.Ltmp2897:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2898:
	.loc	49 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2899:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2900:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2901:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm6
.Ltmp2902:
	.loc	49 66 9 is_stmt 1
	vsubss	80(%rsp), %xmm6, %xmm6
.Ltmp2903:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm6, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_550
.Ltmp2904:
	.loc	49 0 44
	xorl	%esi, %esi
	jmp	.LBB32_551
	.p2align	4
.LBB32_550:
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm6, %xmm13, %xmm9
.Ltmp2905:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2906:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	144(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %esi
.Ltmp2907:
.LBB32_551:
	.loc	23 1291 18 is_stmt 1
	vmovd	%esi, %xmm6
.Ltmp2908:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp2909:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm6, %xmm6
.Ltmp2910:
	.loc	1 1005 9 is_stmt 1
	vmovss	8(%rsp), %xmm9
.Ltmp2911:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	592(%rsp), %xmm1
	vblendvps	%xmm13, 576(%rsp), %xmm1, %xmm13
.Ltmp2912:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp2913:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2914:
	.loc	49 103 24
	vandps	%xmm6, %xmm15, %xmm9
.Ltmp2915:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	movl	%ebp, %r10d
	vucomiss	%xmm9, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_553
	.loc	49 0 24
	movl	$0, 8(%rsp)
	jmp	.LBB32_554
	.p2align	4
.LBB32_553:
	.loc	49 166 0
	vmovss	%xmm6, 8(%rsp)
.Ltmp2916:
.LBB32_554:
	.loc	1 0 0
	movq	480(%r14), %rcx
	movq	%rcx, 104(%rsp)
	vmovss	(%rcx,%r9,4), %xmm6
.Ltmp2917:
	.loc	49 103 24 is_stmt 1
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2918:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
	movl	$841731191, %ebp
.Ltmp2919:
	.loc	49 161 24
	jbe	.LBB32_556
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %ebp
.LBB32_556:
.Ltmp2920:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebp, %xmm6
.Ltmp2921:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
	movl	$8388608, %esi
.Ltmp2922:
	.loc	49 161 24
	jbe	.LBB32_558
.Ltmp2923:
	.loc	49 0 24 is_stmt 0
	movl	%ebp, %esi
.LBB32_558:
	movq	496(%r14), %rcx
.Ltmp2924:
	.loc	49 185 42 is_stmt 1
	movl	%esi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2925:
	.loc	23 1291 18
	vmovd	%ebp, %xmm6
.Ltmp2926:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2927:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2928:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2929:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2930:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2931:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2932:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2933:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2934:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2935:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2936:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2937:
	.loc	49 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp2938:
	.loc	23 1291 18
	vmovd	%esi, %xmm13
.Ltmp2939:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm13, %xmm13
.Ltmp2940:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2941:
	.loc	49 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2942:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2943:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2944:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm6
.Ltmp2945:
	.loc	49 66 9 is_stmt 1
	vsubss	16(%rsp), %xmm6, %xmm6
.Ltmp2946:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm6, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_560
.Ltmp2947:
	.loc	49 0 44
	xorl	%ebp, %ebp
	jmp	.LBB32_561
	.p2align	4
.LBB32_560:
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm6, %xmm13, %xmm9
.Ltmp2948:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2949:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	140(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %ebp
.Ltmp2950:
.LBB32_561:
	.loc	23 1291 18 is_stmt 1
	vmovd	%ebp, %xmm6
.Ltmp2951:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp2952:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm6, %xmm6
.Ltmp2953:
	.loc	1 1005 9 is_stmt 1
	vmovd	%r10d, %xmm9
.Ltmp2954:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	560(%rsp), %xmm1
	vblendvps	%xmm13, 544(%rsp), %xmm1, %xmm13
.Ltmp2955:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp2956:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2957:
	.loc	49 103 24
	vandps	%xmm6, %xmm15, %xmm9
.Ltmp2958:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm9, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_563
.Ltmp2959:
	.loc	49 0 24
	xorl	%ebp, %ebp
.Ltmp2960:
	.loc	49 103 24 is_stmt 1
	vmovss	(%rcx,%r9,4), %xmm6
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2961:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
.Ltmp2962:
	.loc	49 161 24
	ja	.LBB32_564
	jmp	.LBB32_565
.Ltmp2963:
	.loc	49 0 24 is_stmt 0
.Ltmp2964:
	.p2align	4
.LBB32_563:
	.loc	49 166 0 is_stmt 1
	vmovd	%xmm6, %ebp
.Ltmp2965:
	.loc	49 103 24
	vmovss	(%rcx,%r9,4), %xmm6
	vandps	%xmm6, %xmm15, %xmm6
.Ltmp2966:
	.loc	49 124 14
	vucomiss	.LCPI32_4(%rip), %xmm6
.Ltmp2967:
	.loc	49 161 24
	jbe	.LBB32_565
.LBB32_564:
	.loc	49 0 24 is_stmt 0
	vmovd	%xmm6, %r12d
.LBB32_565:
.Ltmp2968:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r12d, %xmm6
.Ltmp2969:
	.loc	49 124 14
	vucomiss	.LCPI32_5(%rip), %xmm6
.Ltmp2970:
	.loc	49 161 24
	jbe	.LBB32_567
.Ltmp2971:
	.loc	49 0 24 is_stmt 0
	movl	%r12d, %r15d
.LBB32_567:
.Ltmp2972:
	.loc	49 185 42 is_stmt 1
	movl	%r15d, %esi
	andl	$8388607, %esi
	orl	$1065353216, %esi
.Ltmp2973:
	.loc	23 1291 18
	vmovd	%esi, %xmm6
.Ltmp2974:
	.loc	49 66 9
	vaddss	.LCPI32_6(%rip), %xmm6, %xmm6
.Ltmp2975:
	.loc	49 71 9
	vmulss	.LCPI32_7(%rip), %xmm6, %xmm9
.Ltmp2976:
	.loc	49 61 9
	vaddss	.LCPI32_8(%rip), %xmm9, %xmm9
.Ltmp2977:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2978:
	.loc	49 61 9
	vaddss	.LCPI32_9(%rip), %xmm9, %xmm9
.Ltmp2979:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2980:
	.loc	49 61 9
	vaddss	.LCPI32_10(%rip), %xmm9, %xmm9
.Ltmp2981:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2982:
	.loc	49 61 9
	vaddss	.LCPI32_11(%rip), %xmm9, %xmm9
.Ltmp2983:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm9
.Ltmp2984:
	.loc	49 61 9
	vaddss	.LCPI32_12(%rip), %xmm9, %xmm9
.Ltmp2985:
	.loc	49 187 28
	shrl	$23, %r15d
	orl	$1258291200, %r15d
.Ltmp2986:
	.loc	23 1291 18
	vmovd	%r15d, %xmm13
.Ltmp2987:
	.loc	49 187 13
	vaddss	.LCPI32_13(%rip), %xmm13, %xmm13
.Ltmp2988:
	.loc	49 71 9
	vmulss	%xmm6, %xmm9, %xmm6
.Ltmp2989:
	.loc	49 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2990:
	.loc	49 71 9
	vmulss	.LCPI32_14(%rip), %xmm6, %xmm6
.Ltmp2991:
	.loc	49 161 24
	vmaxss	.LCPI32_15(%rip), %xmm6, %xmm6
.Ltmp2992:
	.loc	49 161 24 is_stmt 0
	vminss	.LCPI32_16(%rip), %xmm6, %xmm6
.Ltmp2993:
	.loc	49 66 9 is_stmt 1
	vsubss	56(%rsp), %xmm6, %xmm6
.Ltmp2994:
	.loc	49 161 59
	vmovss	.LCPI32_19(%rip), %xmm9
	vucomiss	%xmm6, %xmm9
	.loc	49 161 44 is_stmt 0
	jb	.LBB32_569
.Ltmp2995:
	.loc	49 0 44
	xorl	%r9d, %r9d
	jmp	.LBB32_570
	.p2align	4
.LBB32_569:
	vmovss	.LCPI32_17(%rip), %xmm13
	vaddss	%xmm6, %xmm13, %xmm9
.Ltmp2996:
	vmulss	%xmm9, %xmm9, %xmm9
	vmulss	.LCPI32_18(%rip), %xmm9, %xmm9
.Ltmp2997:
	.loc	49 161 0
	vcmpnltss	%xmm6, %xmm13, %xmm13
	vblendvps	%xmm13, %xmm9, %xmm6, %xmm6
	vmulss	136(%rsp), %xmm6, %xmm6
	vmovd	%xmm6, %r9d
.Ltmp2998:
.LBB32_570:
	.loc	49 0 0
	movq	320(%rsp), %r12
	movl	12(%rsp), %esi
.Ltmp2999:
	.loc	23 1291 18 is_stmt 1
	vmovd	%r9d, %xmm6
.Ltmp3000:
	.loc	49 161 24
	vmaxss	.LCPI32_20(%rip), %xmm6, %xmm6
.Ltmp3001:
	.loc	49 161 24 is_stmt 0
	vxorps	%xmm9, %xmm9, %xmm9
	vminss	%xmm9, %xmm6, %xmm6
.Ltmp3002:
	.loc	1 1005 9 is_stmt 1
	vmovd	%esi, %xmm9
.Ltmp3003:
	.loc	49 161 24
	vcmpnltss	%xmm9, %xmm6, %xmm13
	vmovaps	528(%rsp), %xmm1
	vblendvps	%xmm13, 448(%rsp), %xmm1, %xmm13
.Ltmp3004:
	.loc	49 66 9
	vsubss	%xmm6, %xmm9, %xmm9
.Ltmp3005:
	.loc	49 92 9
	vmulss	%xmm13, %xmm9, %xmm9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp3006:
	.loc	49 103 24
	vandps	%xmm6, %xmm15, %xmm9
.Ltmp3007:
	.loc	49 166 41
	vmovss	.LCPI32_2(%rip), %xmm13
	vucomiss	%xmm9, %xmm13
	.loc	49 166 24 is_stmt 0
	jbe	.LBB32_572
	.loc	49 0 24
	xorl	%esi, %esi
	jmp	.LBB32_573
	.p2align	4
.LBB32_572:
	.loc	49 166 0
	vmovd	%xmm6, %esi
.Ltmp3008:
.LBB32_573:
	.loc	1 0 0
	negq	%rbx
	addq	%rbx, %rdi
	incq	%rdi
.Ltmp3009:
	.loc	48 568 12 is_stmt 1
	cmpq	%rdx, %rdi
	ja	.LBB32_618
.Ltmp3010:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp3011:
	.loc	48 568 12
	cmpq	%rax, %rdi
	ja	.LBB32_619
.Ltmp3012:
	.loc	49 51 9
	je	.LBB32_644
.Ltmp3013:
	.loc	48 568 12
	cmpq	%r11, %rdi
	ja	.LBB32_620
.Ltmp3014:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_644
.Ltmp3015:
	.loc	48 568 12 is_stmt 1
	cmpq	%r8, %rdi
	movq	96(%rsp), %r9
	ja	.LBB32_621
.Ltmp3016:
	.loc	48 0 12 is_stmt 0
	je	.LBB32_644
	vmulss	%xmm7, %xmm10, %xmm6
	vmovaps	256(%rsp), %xmm13
	vmulss	%xmm2, %xmm13, %xmm2
	vaddss	%xmm6, %xmm2, %xmm2
	vmulss	%xmm0, %xmm14, %xmm0
	vmovaps	160(%rsp), %xmm14
	vmulss	%xmm11, %xmm14, %xmm6
	vaddss	%xmm0, %xmm6, %xmm0
	vaddss	%xmm5, %xmm5, %xmm5
	vaddss	224(%rsp), %xmm5, %xmm6
	vmovss	20(%rsp), %xmm1
	vaddss	%xmm1, %xmm1, %xmm1
	vaddss	192(%rsp), %xmm1, %xmm5
	vaddss	%xmm2, %xmm2, %xmm1
	vaddss	%xmm4, %xmm4, %xmm2
	vaddss	%xmm8, %xmm8, %xmm7
	vaddss	%xmm3, %xmm3, %xmm9
	vaddss	%xmm0, %xmm0, %xmm10
	vaddss	%xmm12, %xmm12, %xmm11
	vaddss	%xmm1, %xmm13, %xmm8
	vaddss	240(%rsp), %xmm2, %xmm4
	vaddss	208(%rsp), %xmm7, %xmm3
	vaddss	288(%rsp), %xmm9, %xmm0
	vaddss	%xmm10, %xmm14, %xmm1
	vaddss	176(%rsp), %xmm11, %xmm2
	vmovss	48(%rsp), %xmm7
	vaddss	304(%rsp), %xmm7, %xmm7
	vmovss	8(%rsp), %xmm9
	vaddss	24(%rsp), %xmm9, %xmm10
	vmovd	%ebp, %xmm9
	vaddss	28(%rsp), %xmm9, %xmm11
	vandps	%xmm6, %xmm15, %xmm9
	vandps	%xmm5, %xmm15, %xmm12
	vandps	%xmm15, %xmm8, %xmm13
	vmovss	.LCPI32_2(%rip), %xmm14
	vcmpnltss	%xmm14, %xmm9, %xmm9
	vandps	%xmm6, %xmm9, %xmm6
	vmovaps	%xmm6, 224(%rsp)
	vandps	%xmm4, %xmm15, %xmm6
	vcmpnltss	%xmm14, %xmm12, %xmm9
	vandps	%xmm5, %xmm9, %xmm5
	vmovaps	%xmm5, 192(%rsp)
	vandps	%xmm3, %xmm15, %xmm5
	vcmpnltss	%xmm14, %xmm13, %xmm9
	vandps	%xmm8, %xmm9, %xmm8
	vmovaps	%xmm8, 256(%rsp)
	vandps	%xmm0, %xmm15, %xmm8
	vcmpnltss	%xmm14, %xmm6, %xmm6
	vandps	%xmm4, %xmm6, %xmm4
	vmovaps	%xmm4, 240(%rsp)
	vandps	%xmm1, %xmm15, %xmm4
	vcmpnltss	%xmm14, %xmm5, %xmm5
	vandps	%xmm3, %xmm5, %xmm3
	vmovaps	%xmm3, 208(%rsp)
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
	vmovaps	%xmm0, 288(%rsp)
	vandps	%xmm1, %xmm4, %xmm0
	vmovaps	%xmm0, 160(%rsp)
	vandps	%xmm2, %xmm3, %xmm0
	vmovaps	%xmm0, 176(%rsp)
	vmovss	.LCPI32_23(%rip), %xmm9
	vminss	%xmm9, %xmm5, %xmm4
	vminss	%xmm9, %xmm6, %xmm2
	vminss	%xmm9, %xmm7, %xmm3
	vroundss	$9, %xmm4, %xmm4, %xmm1
	vroundss	$9, %xmm2, %xmm2, %xmm0
	vsubss	%xmm1, %xmm4, %xmm4
	vmovss	.LCPI32_24(%rip), %xmm8
.Ltmp3017:
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
.Ltmp3018:
	vroundss	$9, %xmm3, %xmm3, %xmm4
	vsubss	%xmm0, %xmm2, %xmm6
	vsubss	%xmm4, %xmm3, %xmm2
	vmovss	.LCPI32_29(%rip), %xmm15
.Ltmp3019:
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
.Ltmp3020:
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp3021:
	vmovd	%esi, %xmm5
	vaddss	416(%rsp), %xmm5, %xmm5
	vmulss	.LCPI32_21(%rip), %xmm5, %xmm5
	vmaxss	%xmm10, %xmm5, %xmm5
	vminss	%xmm9, %xmm5, %xmm5
.Ltmp3022:
	vaddss	%xmm3, %xmm15, %xmm3
	vaddss	%xmm7, %xmm0, %xmm0
	vmovd	%xmm0, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm0
	vmulss	%xmm0, %xmm3, %xmm0
.Ltmp3023:
	vmulss	%xmm2, %xmm8, %xmm3
	vaddss	%xmm3, %xmm11, %xmm3
	vmulss	%xmm3, %xmm2, %xmm3
	vaddss	%xmm3, %xmm12, %xmm3
	vmulss	%xmm3, %xmm2, %xmm3
	vaddss	%xmm3, %xmm13, %xmm3
	vmulss	%xmm3, %xmm2, %xmm3
	vaddss	%xmm3, %xmm14, %xmm3
.Ltmp3024:
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp3025:
	vroundss	$9, %xmm5, %xmm5, %xmm3
	vsubss	%xmm3, %xmm5, %xmm5
.Ltmp3026:
	vaddss	%xmm2, %xmm15, %xmm2
	vaddss	%xmm7, %xmm4, %xmm4
	vmovd	%xmm4, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm4
	vmulss	%xmm4, %xmm2, %xmm2
.Ltmp3027:
	vmulss	%xmm5, %xmm8, %xmm4
	vaddss	%xmm4, %xmm11, %xmm4
	vmulss	%xmm4, %xmm5, %xmm4
	vaddss	%xmm4, %xmm12, %xmm4
	vmulss	%xmm4, %xmm5, %xmm4
	vaddss	%xmm4, %xmm13, %xmm4
	vmulss	%xmm4, %xmm5, %xmm4
	vaddss	%xmm4, %xmm14, %xmm4
.Ltmp3028:
	vmulss	%xmm4, %xmm5, %xmm4
	vaddss	%xmm4, %xmm15, %xmm4
	vaddss	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %eax
	shll	$23, %eax
	vmovd	%eax, %xmm3
	movq	120(%rsp), %rax
.Ltmp3029:
	vmulss	(%rax,%rdi,4), %xmm1, %xmm1
	movq	112(%rsp), %rax
	vmulss	(%rax,%rdi,4), %xmm0, %xmm0
.Ltmp3030:
	vmulss	%xmm3, %xmm4, %xmm3
.Ltmp3031:
	vaddss	%xmm0, %xmm1, %xmm0
	movq	104(%rsp), %rax
.Ltmp3032:
	vmulss	(%rax,%rdi,4), %xmm2, %xmm1
.Ltmp3033:
	.loc	49 71 9 is_stmt 1
	vmulss	(%rcx,%rdi,4), %xmm3, %xmm2
.Ltmp3034:
	.loc	49 61 9
	vaddss	%xmm2, %xmm1, %xmm1
	movq	88(%rsp), %r8
.Ltmp3035:
	.loc	49 56 9
	vmovss	%xmm0, (%r8,%r13,4)
.Ltmp3036:
	.loc	49 56 9 is_stmt 0
	vmovss	%xmm1, (%r9,%r13,4)
.Ltmp3037:
	.loc	1 0 0
	incq	%r13
.Ltmp3038:
	.loc	2 1916 50 is_stmt 1
	cmpq	%r13, 128(%rsp)
.Ltmp3039:
	.loc	3 900 12
	jne	.LBB32_509
	jmp	.LBB32_427
.Ltmp3040:
.LBB32_582:
	.loc	3 0 12 is_stmt 0
	movl	%ecx, %ebp
.LBB32_583:
	vmovaps	224(%rsp), %xmm0
.Ltmp3041:
	.loc	1 1160 5 is_stmt 1
	vmovss	%xmm0, 168(%r14)
	vmovaps	192(%rsp), %xmm0
	vmovss	%xmm0, 172(%r14)
	vmovaps	256(%rsp), %xmm0
	vmovss	%xmm0, 176(%r14)
	vmovaps	240(%rsp), %xmm0
	vmovss	%xmm0, 180(%r14)
	vmovaps	208(%rsp), %xmm0
	.loc	1 1161 5
	vmovss	%xmm0, 528(%r14)
	vmovaps	288(%rsp), %xmm0
	vmovss	%xmm0, 532(%r14)
	vmovaps	160(%rsp), %xmm0
	vmovss	%xmm0, 536(%r14)
	vmovaps	176(%rsp), %xmm0
	vmovss	%xmm0, 540(%r14)
	movl	48(%rsp), %eax
	.loc	1 1162 5
	movl	%eax, 184(%r14)
	movl	%ebp, 188(%r14)
	movl	8(%rsp), %eax
	.loc	1 1163 5
	movl	%eax, 544(%r14)
	movl	%r13d, 548(%r14)
	.loc	1 1164 5
	movq	%rdi, 848(%r14)
	vmovss	72(%rsp), %xmm0
.Ltmp3042:
	.loc	1 1300 17
	vmovss	%xmm0, 192(%r14)
	.loc	1 1301 34
	movl	204(%r14), %eax
	movq	128(%rsp), %rsi
.Ltmp3043:
	.loc	38 2472 13
	subl	%esi, %eax
	movl	$0, %edx
	cmovbl	%edx, %eax
.Ltmp3044:
	.loc	1 1301 34
	movl	220(%r14), %ecx
	.loc	1 1301 17 is_stmt 0
	movl	%eax, 204(%r14)
	vmovss	512(%rsp), %xmm0
	.loc	1 1300 17 is_stmt 1
	vmovss	%xmm0, 208(%r14)
.Ltmp3045:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp3046:
	.loc	1 1301 17
	movl	%ecx, 220(%r14)
	vmovss	32(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 224(%r14)
	.loc	1 1301 34
	movl	236(%r14), %eax
.Ltmp3047:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3048:
	.loc	1 1301 34
	movl	252(%r14), %ecx
	.loc	1 1301 17 is_stmt 0
	movl	%eax, 236(%r14)
	vmovss	36(%rsp), %xmm0
	.loc	1 1300 17 is_stmt 1
	vmovss	%xmm0, 240(%r14)
.Ltmp3049:
	.loc	38 2472 13
	subl	%esi, %ecx
	cmovbl	%edx, %ecx
.Ltmp3050:
	.loc	1 1301 17
	movl	%ecx, 252(%r14)
	vmovss	304(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 256(%r14)
	.loc	1 1301 34
	movl	268(%r14), %eax
.Ltmp3051:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3052:
	.loc	1 1301 17
	movl	%eax, 268(%r14)
	vmovss	80(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 272(%r14)
	.loc	1 1301 34
	movl	284(%r14), %eax
.Ltmp3053:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3054:
	.loc	1 1301 17
	movl	%eax, 284(%r14)
	vmovss	40(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 288(%r14)
	.loc	1 1301 34
	movl	300(%r14), %eax
.Ltmp3055:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3056:
	.loc	1 1301 17
	movl	%eax, 300(%r14)
	vmovss	44(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 304(%r14)
	.loc	1 1301 34
	movl	316(%r14), %eax
.Ltmp3057:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3058:
	.loc	1 1301 17
	movl	%eax, 316(%r14)
	vmovss	88(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 320(%r14)
	.loc	1 1301 34
	movl	332(%r14), %eax
.Ltmp3059:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3060:
	.loc	1 1301 17
	movl	%eax, 332(%r14)
	vmovss	24(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 336(%r14)
	.loc	1 1301 34
	movl	348(%r14), %eax
.Ltmp3061:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3062:
	.loc	1 1301 17
	movl	%eax, 348(%r14)
	vmovss	16(%rsp), %xmm0
.Ltmp3063:
	.loc	1 1300 17
	vmovss	%xmm0, 552(%r14)
	.loc	1 1301 34
	movl	564(%r14), %eax
.Ltmp3064:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3065:
	.loc	1 1301 17
	movl	%eax, 564(%r14)
	vmovss	96(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 568(%r14)
	.loc	1 1301 34
	movl	580(%r14), %eax
.Ltmp3066:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3067:
	.loc	1 1301 17
	movl	%eax, 580(%r14)
	vmovss	104(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 584(%r14)
	.loc	1 1301 34
	movl	596(%r14), %eax
.Ltmp3068:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3069:
	.loc	1 1301 17
	movl	%eax, 596(%r14)
	vmovss	112(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 600(%r14)
	.loc	1 1301 34
	movl	612(%r14), %eax
.Ltmp3070:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3071:
	.loc	1 1301 17
	movl	%eax, 612(%r14)
	vmovss	28(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 616(%r14)
	.loc	1 1301 34
	movl	628(%r14), %eax
.Ltmp3072:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3073:
	.loc	1 1301 17
	movl	%eax, 628(%r14)
	vmovss	56(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 632(%r14)
	.loc	1 1301 34
	movl	644(%r14), %eax
.Ltmp3074:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3075:
	.loc	1 1301 17
	movl	%eax, 644(%r14)
	vmovss	120(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 648(%r14)
	.loc	1 1301 34
	movl	660(%r14), %eax
.Ltmp3076:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3077:
	.loc	1 1301 17
	movl	%eax, 660(%r14)
	vmovss	12(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 664(%r14)
	.loc	1 1301 34
	movl	676(%r14), %eax
.Ltmp3078:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3079:
	.loc	1 1301 17
	movl	%eax, 676(%r14)
	vmovss	20(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 680(%r14)
	.loc	1 1301 34
	movl	692(%r14), %eax
.Ltmp3080:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3081:
	.loc	1 1301 17
	movl	%eax, 692(%r14)
	vmovss	416(%rsp), %xmm0
	.loc	1 1300 17
	vmovss	%xmm0, 696(%r14)
	.loc	1 1301 34
	movl	708(%r14), %eax
.Ltmp3082:
	.loc	38 2472 13
	subl	%esi, %eax
	cmovbl	%edx, %eax
.Ltmp3083:
	.loc	1 1301 17
	movl	%eax, 708(%r14)
.Ltmp3084:
	.loc	1 1197 9
	jmp	.LBB32_428
.Ltmp3085:
.LBB32_605:
	.loc	1 0 9 is_stmt 0
	xorl	%ecx, %ecx
.Ltmp3086:
	.loc	50 208 34 is_stmt 1
	jmp	.LBB32_603
.Ltmp3087:
.LBB32_606:
	.loc	48 581 13
	leaq	.Lalloc_1f724420e117514d5eeca145f523ec40(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3088:
.LBB32_607:
	.loc	48 581 13
	leaq	.Lalloc_1f724420e117514d5eeca145f523ec40(%rip), %rcx
.Ltmp3089:
	.loc	1 0 0 is_stmt 0
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_608:
.Ltmp3090:
	.loc	48 581 13 is_stmt 1
	leaq	.Lalloc_ebd89d0b93275e5686fc2b41f2e9561d(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3091:
.LBB32_609:
	.loc	48 581 13
	leaq	.Lalloc_ebd89d0b93275e5686fc2b41f2e9561d(%rip), %rcx
.Ltmp3092:
	.loc	1 0 0 is_stmt 0
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_610:
.Ltmp3093:
	.loc	48 569 13 is_stmt 1
	leaq	.Lalloc_89de14a4db9a7a5e199f0b3432909247(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3094:
.LBB32_611:
	.loc	48 569 13
	leaq	.Lalloc_89de14a4db9a7a5e199f0b3432909247(%rip), %rcx
.Ltmp3095:
	.loc	1 0 0 is_stmt 0
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_612:
.Ltmp3096:
	.loc	48 569 13 is_stmt 1
	leaq	.Lalloc_e4c416c2b2b3df4fd71341a77882ada1(%rip), %rcx
.Ltmp3097:
	.loc	1 0 0 is_stmt 0
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_613:
.Ltmp3098:
	.loc	48 569 13 is_stmt 1
	leaq	.Lalloc_e4c416c2b2b3df4fd71341a77882ada1(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3099:
.LBB32_614:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_7515f6adc8a5521b72f04a3abb372e72(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_615:
	leaq	.Lalloc_8c2aade3368450b16e7e4997ad3fca81(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_616:
	leaq	.Lalloc_a4c3da5a99763e9452b49d42852d67e3(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_617:
	leaq	.Lalloc_e11ba0c4c1124bbcae4603a2ae121338(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_618:
	leaq	.Lalloc_379b93204ee4659b79f4b07b6520076a(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_619:
	leaq	.Lalloc_2bb8eb0542a889f29ef069491eb97a17(%rip), %rcx
	movq	%rax, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_620:
	leaq	.Lalloc_cd96798e807157d7db308788cb4dd125(%rip), %rcx
	movq	%r11, %rsi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_621:
	leaq	.Lalloc_164adf6876ccf79975d46e129e248ca5(%rip), %rcx
	movq	%r8, %rsi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_622:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r9, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_623:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r9, %rdi
	movq	%rax, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_624:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r10, %rdi
	movq	%r11, %rsi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_625:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r10, %rdi
	movq	%r8, %rsi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_626:
.Ltmp3100:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r10, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_627:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r10, %rdi
.Ltmp3101:
	movq	%rax, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_628:
.Ltmp3102:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r9, %rdi
.Ltmp3103:
	movq	%r11, %rsi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_629:
.Ltmp3104:
	leaq	.Lalloc_a843826d471182f1ef404bfcc8b3fde6(%rip), %rcx
	movq	%r9, %rdi
.Ltmp3105:
	movq	%r8, %rsi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_640:
.Ltmp3106:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_7ac5156198d2516c0e2a17140923b083(%rip), %rcx
	movq	%r12, %rdi
	movq	152(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3107:
.LBB32_641:
	.loc	48 456 13
	leaq	.Lalloc_683f9160cdc5ee4596b2ecf709188a9a(%rip), %rcx
	movq	%r12, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3108:
.LBB32_642:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_683f9160cdc5ee4596b2ecf709188a9a(%rip), %rcx
	movq	%rbp, %rdi
	movq	%r15, %rsi
	movq	152(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_643:
	leaq	.Lalloc_7ac5156198d2516c0e2a17140923b083(%rip), %rcx
	movq	%rbp, %rdi
	movq	%r15, %rsi
	movq	152(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_644:
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_645:
.Ltmp3109:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_683f9160cdc5ee4596b2ecf709188a9a(%rip), %rcx
.Ltmp3110:
	.loc	48 456 13 is_stmt 0
	movq	%rbp, %rdi
	movq	464(%rsp), %rsi
	movq	152(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3111:
.LBB32_646:
	.loc	48 456 13 is_stmt 1
	leaq	.Lalloc_7ac5156198d2516c0e2a17140923b083(%rip), %rcx
.Ltmp3112:
	.loc	48 456 13 is_stmt 0
	movq	%rbp, %rdi
	movq	464(%rsp), %rsi
	movq	152(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3113:
.LBB32_647:
	.loc	1 0 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp3114:
.LBB32_650:
	.loc	1 1479 43 is_stmt 1
	leaq	.Lalloc_618452d81e57e47d125c0afad7fc5006(%rip), %rdx
	movl	$12, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB32_651:
.Ltmp3115:
	.loc	1 1481 45
	leaq	.Lalloc_97e1583d528f48ca4578bc0e35112ce6(%rip), %rdx
	movl	$10, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp3116:
.Lfunc_end32:
	.size	_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process, .Lfunc_end32-_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process
