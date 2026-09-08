_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_:
.Lfunc_begin1:
	.loc	1 3120 0
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
	subq	$1632, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, 3224(%rsp)
.Ltmp1329:
	.loc	1 3133 13 prologue_end
	movq	40(%rdx), %rax
	movq	%rax, 32(%rsp)
	testq	%rax, %rax
	je	.LBB1_2
	.loc	1 3134 13
	movb	$0, 2152(%rsi)
.LBB1_2:
	.loc	1 0 0 is_stmt 0
	movq	32(%rdx), %rax
	movq	%rax, 96(%rsp)
	movq	%rsi, 24(%rsp)
	.loc	1 3136 51 is_stmt 1
	movzbl	2288(%rsi), %eax
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 4576(%rsp)
	vmovaps	%ymm0, 4544(%rsp)
	vmovaps	%ymm0, 4512(%rsp)
	vmovaps	%ymm0, 4480(%rsp)
	vmovaps	%ymm0, 4448(%rsp)
	vmovaps	%ymm0, 4416(%rsp)
	vmovaps	%ymm0, 4384(%rsp)
	vmovaps	%ymm0, 4352(%rsp)
	vmovaps	%ymm0, 4320(%rsp)
	vmovaps	%ymm0, 4288(%rsp)
.Ltmp1330:
	.loc	2 1032 9
	movb	%al, 4608(%rsp)
	movq	%rdx, 224(%rsp)
	movq	56(%rdx), %rbx
.Ltmp1331:
	.loc	3 900 12
	cmpq	$1, %rbx
	movq	%rbx, %r12
	adcq	$-1, %r12
.Ltmp1332:
	.loc	1 3138 25
	testq	%rbx, %rbx
	je	.LBB1_408
.Ltmp1333:
	.loc	1 3139 23
	cmpq	$1, %rbx
	je	.LBB1_415
	.loc	1 0 23 is_stmt 0
	movq	224(%rsp), %rcx
	movq	48(%rcx), %r15
	movl	(%r15), %edi
	.loc	1 3139 23
	movl	4(%r15), %esi
.Ltmp1334:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	32(%rsp), %rdx
	jb	.LBB1_366
	cmpq	%rsi, %rdx
	jb	.LBB1_366
	.loc	4 0 16 is_stmt 0
	movq	24(%rsp), %rax
	leaq	1848(%rax), %r9
	leaq	1648(%rax), %r8
	movq	96(%rcx), %r13
	leaq	2048(%rax), %rdx
	.loc	4 1054 31 is_stmt 1
	subq	%rdi, %rsi
.Ltmp1335:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	96(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
	leaq	4288(%rsp), %rax
	movq	%rdx, %r14
.Ltmp1336:
	.loc	1 3140 13
	movq	%r13, %rcx
	movq	%r8, 16(%rsp)
	movq	%r9, 208(%rsp)
	pushq	%rax
	pushq	$0
	vzeroupper
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1337:
	.loc	1 3139 23
	cmpq	$1, %r12
	je	.LBB1_416
	.loc	1 0 23 is_stmt 0
	movl	4(%r15), %edi
	.loc	1 3139 23
	movl	8(%r15), %esi
.Ltmp1338:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	32(%rsp), %rdx
	jb	.LBB1_366
	cmpq	%rsi, %rdx
	jb	.LBB1_366
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1339:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	96(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1340:
	.loc	1 3147 17
	leaq	4328(%rsp), %rax
	.loc	1 3140 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	208(%rsp), %r9
	pushq	%rax
	pushq	$1
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1341:
	.loc	1 3138 25
	cmpq	$2, %rbx
	je	.LBB1_408
.Ltmp1342:
	.loc	1 3139 23
	cmpq	$2, %r12
	je	.LBB1_417
	.loc	1 0 23 is_stmt 0
	movl	8(%r15), %edi
	.loc	1 3139 23
	movl	12(%r15), %esi
.Ltmp1343:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	32(%rsp), %rdx
	jb	.LBB1_366
	cmpq	%rsi, %rdx
	jb	.LBB1_366
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1344:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	96(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1345:
	.loc	1 3147 17
	leaq	4368(%rsp), %rax
	.loc	1 3140 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	208(%rsp), %r9
	pushq	%rax
	pushq	$2
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1346:
	.loc	1 3138 25
	cmpq	$3, %rbx
	je	.LBB1_408
.Ltmp1347:
	.loc	1 3139 23
	cmpq	$3, %r12
	je	.LBB1_418
	.loc	1 0 23 is_stmt 0
	movl	12(%r15), %edi
	.loc	1 3139 23
	movl	16(%r15), %esi
.Ltmp1348:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	32(%rsp), %rdx
	jb	.LBB1_366
	cmpq	%rsi, %rdx
	jb	.LBB1_366
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1349:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	96(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1350:
	.loc	1 3147 17
	leaq	4408(%rsp), %rax
	.loc	1 3140 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	208(%rsp), %r9
	pushq	%rax
	pushq	$3
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1351:
	.loc	1 3138 25
	cmpq	$4, %rbx
	je	.LBB1_408
.Ltmp1352:
	.loc	1 3139 23
	cmpq	$4, %r12
	je	.LBB1_419
	.loc	1 0 23 is_stmt 0
	movl	16(%r15), %edi
	.loc	1 3139 23
	movl	20(%r15), %esi
.Ltmp1353:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	32(%rsp), %rdx
	jb	.LBB1_366
	cmpq	%rsi, %rdx
	jb	.LBB1_366
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1354:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	96(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1355:
	.loc	1 3147 17
	leaq	4448(%rsp), %rax
	.loc	1 3140 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	208(%rsp), %r9
	pushq	%rax
	pushq	$4
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1356:
	.loc	1 3138 25
	cmpq	$5, %rbx
	je	.LBB1_408
.Ltmp1357:
	.loc	1 3139 23
	cmpq	$5, %r12
	je	.LBB1_420
	.loc	1 0 23 is_stmt 0
	movl	20(%r15), %edi
	.loc	1 3139 23
	movl	24(%r15), %esi
.Ltmp1358:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	32(%rsp), %rdx
	jb	.LBB1_366
	cmpq	%rsi, %rdx
	jb	.LBB1_366
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1359:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	96(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1360:
	.loc	1 3147 17
	leaq	4488(%rsp), %rax
	.loc	1 3140 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	208(%rsp), %r9
	pushq	%rax
	pushq	$5
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1361:
	.loc	1 3138 25
	cmpq	$6, %rbx
	je	.LBB1_408
.Ltmp1362:
	.loc	1 3139 23
	cmpq	$6, %r12
	je	.LBB1_421
	.loc	1 0 23 is_stmt 0
	movl	24(%r15), %edi
	.loc	1 3139 23
	movl	28(%r15), %esi
.Ltmp1363:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	32(%rsp), %rdx
	jb	.LBB1_366
	cmpq	%rsi, %rdx
	jb	.LBB1_366
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1364:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	96(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1365:
	.loc	1 3147 17
	leaq	4528(%rsp), %rax
	.loc	1 3140 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	208(%rsp), %r9
	pushq	%rax
	pushq	$6
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1366:
	.loc	1 3138 25
	cmpq	$7, %rbx
	je	.LBB1_408
.Ltmp1367:
	.loc	1 3139 23
	cmpq	$7, %r12
	je	.LBB1_422
	.loc	1 0 23 is_stmt 0
	movl	28(%r15), %edi
	.loc	1 3139 23
	movl	32(%r15), %esi
.Ltmp1368:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	32(%rsp), %rdx
	jb	.LBB1_366
	cmpq	%rsi, %rdx
	jb	.LBB1_366
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1369:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	96(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1370:
	.loc	1 3147 17
	leaq	4568(%rsp), %rax
	.loc	1 3140 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	208(%rsp), %r9
	pushq	%rax
	pushq	$7
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
	movq	224(%rsp), %rax
.Ltmp1371:
	.loc	1 3152 37
	movq	(%rax), %r13
	movq	8(%rax), %r15
	.loc	1 3152 49 is_stmt 0
	movl	104(%rax), %eax
	movq	%rax, 8(%rsp)
.Ltmp1372:
	.loc	1 2311 21 is_stmt 1
	leaq	(,%rax,8), %r11
	movq	24(%rsp), %rbx
.Ltmp1373:
	.loc	1 2312 21
	movzbl	2153(%rbx), %ecx
	.loc	1 0 0 is_stmt 0
	movq	1784(%rbx), %rax
	.loc	1 2312 21
	cmpb	2144(%rbx), %cl
	movq	%r15, 88(%rsp)
	movq	%r13, 216(%rsp)
	movq	%r11, 3208(%rsp)
	jne	.LBB1_38
	.loc	1 2313 37 is_stmt 1
	movq	1776(%rbx), %rcx
.Ltmp1374:
	.loc	6 314 17
	movq	%rax, %rdx
	shlq	$4, %rdx
	movq	%rcx, %rsi
	.loc	6 0 17 is_stmt 0
.Ltmp1375:
	.p2align	4
.LBB1_35:
.Ltmp1376:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp1377:
	.loc	6 180 28
	je	.LBB1_211
.Ltmp1378:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp1379:
	.loc	6 315 25
	jne	.LBB1_39
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB1_35
	jmp	.LBB1_39
.Ltmp1380:
.LBB1_38:
	.loc	1 715 26 is_stmt 1
	movq	1776(%rbx), %rcx
.Ltmp1381:
.LBB1_39:
	.loc	1 0 26 is_stmt 0
	xorl	%r8d, %r8d
.LBB1_40:
	.loc	1 2329 13 is_stmt 1
	leaq	1616(%rbx), %rdx
	movq	%rdx, 3216(%rsp)
.Ltmp1382:
	.loc	6 314 17
	shlq	$4, %rax
	movl	%r8d, 764(%rsp)
	.loc	6 0 17 is_stmt 0
.Ltmp1383:
	.p2align	4
.LBB1_41:
.Ltmp1384:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp1385:
	.loc	6 180 28
	je	.LBB1_50
.Ltmp1386:
	.loc	1 690 21
	cmpl	$0, 12(%rcx)
.Ltmp1387:
	.loc	6 315 25
	jne	.LBB1_44
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB1_41
.Ltmp1388:
.LBB1_44:
	.loc	1 1118 5 is_stmt 1
	movq	1832(%rbx), %rcx
	testq	%rcx, %rcx
	je	.LBB1_136
	.loc	1 0 5 is_stmt 0
	movq	1824(%rbx), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rdx
	.p2align	4
.LBB1_46:
.Ltmp1389:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp1390:
	.loc	6 180 28
	je	.LBB1_136
.Ltmp1391:
	.loc	1 390 34
	movl	(%rdx), %esi
	cmpl	(%rax), %esi
	jne	.LBB1_140
	movl	4(%rdx), %esi
	cmpl	4(%rax), %esi
	jne	.LBB1_140
	movl	8(%rdx), %esi
.Ltmp1392:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rdx
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp1393:
	.loc	1 390 34
	cmpl	8(%rax), %esi
.Ltmp1394:
	.loc	6 315 25
	je	.LBB1_46
	jmp	.LBB1_140
.Ltmp1395:
.LBB1_50:
	.loc	1 715 63
	movq	1792(%rbx), %rcx
	movq	1800(%rbx), %rdx
.Ltmp1396:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp1397:
	.p2align	4
.LBB1_51:
	movq	%rdx, %rax
.Ltmp1398:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp1399:
	.loc	6 180 28
	je	.LBB1_54
.Ltmp1400:
	.loc	1 690 21
	cmpl	$0, 12(%rcx)
.Ltmp1401:
	.loc	6 315 25
	jne	.LBB1_54
	leaq	-16(%rax), %rdx
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB1_51
.Ltmp1402:
.LBB1_54:
	.loc	1 1118 5 is_stmt 1
	movq	1832(%rbx), %rdx
	testq	%rdx, %rdx
	je	.LBB1_60
	.loc	1 0 5 is_stmt 0
	movq	1824(%rbx), %rcx
	shlq	$2, %rdx
	leaq	(%rdx,%rdx,2), %rdx
	movq	%rcx, %rsi
	.p2align	4
.LBB1_56:
.Ltmp1403:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp1404:
	.loc	6 180 28
	je	.LBB1_60
.Ltmp1405:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rcx), %edi
	jne	.LBB1_64
	movl	4(%rsi), %edi
	cmpl	4(%rcx), %edi
	jne	.LBB1_64
	movl	8(%rsi), %edi
.Ltmp1406:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rdx
.Ltmp1407:
	.loc	1 390 34
	cmpl	8(%rcx), %edi
.Ltmp1408:
	.loc	6 315 25
	je	.LBB1_56
	jmp	.LBB1_64
.Ltmp1409:
.LBB1_60:
	.loc	1 1119 12
	movq	1768(%rbx), %rcx
	testq	%rcx, %rcx
	je	.LBB1_216
	.loc	1 0 12 is_stmt 0
	movq	1760(%rbx), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB1_62:
.Ltmp1410:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rcx
.Ltmp1411:
	.loc	6 180 28
	je	.LBB1_216
.Ltmp1412:
	.loc	6 315 25
	movl	(%rdx,%rsi), %edi
	addq	$4, %rsi
.Ltmp1413:
	.loc	1 1119 43
	cmpl	(%rdx), %edi
.Ltmp1414:
	.loc	6 315 25
	je	.LBB1_62
.Ltmp1415:
.LBB1_64:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp1416:
	.loc	1 3204 12
	jne	.LBB1_140
	.loc	1 0 12 is_stmt 0
	leaq	1376(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp1417:
	.loc	1 3254 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp1418:
	.loc	1 3259 19
	movzbl	1536(%rbx), %r14d
	movq	%rbx, %rax
.Ltmp1419:
	.loc	1 3260 21
	movzbl	1537(%rbx), %ebx
.Ltmp1420:
	.loc	1 3261 27
	movl	1640(%rax), %r12d
.Ltmp1421:
	.loc	1 3262 27
	movl	1644(%rax), %eax
	movq	%rax, 96(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 3264(%rsp)
	leaq	4640(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
	movq	8(%rsp), %rax
.Ltmp1422:
	.loc	4 3758 16
	leaq	31(%rax), %rsi
	shrq	$5, %rsi
	vmovaps	1376(%rsp), %ymm13
.Ltmp1423:
	.loc	8 446 20
	je	.LBB1_316
.Ltmp1424:
	.file	29 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs"
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm11
.Ltmp1425:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm11, %ymm0
	vmovaps	%ymm11, %ymm12
	testb	%r14b, %r14b
	jne	.LBB1_68
.Ltmp1426:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, %ymm12
.LBB1_68:
	vmovaps	%ymm13, 1216(%rsp)
	testb	%bl, %bl
	movq	%r12, %r8
	jne	.LBB1_70
	vmovaps	%ymm0, %ymm11
.LBB1_70:
.Ltmp1427:
	.loc	1 1761 23 is_stmt 1
	vmovaps	1408(%rsp), %ymm4
	vmovaps	1440(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	1472(%rsp), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	1504(%rsp), %ymm6
	vmovaps	1536(%rsp), %ymm8
	vmovaps	1568(%rsp), %ymm13
	vmovaps	1600(%rsp), %ymm3
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovaps	1664(%rsp), %ymm10
	vmovaps	1696(%rsp), %ymm7
	vmovaps	1728(%rsp), %ymm0
	vmovaps	%ymm0, 1120(%rsp)
	vmovaps	1760(%rsp), %ymm5
	vmovaps	1888(%rsp), %ymm14
	vmovaps	2080(%rsp), %ymm15
	movq	$0, 640(%rsp)
	movl	$32, %r9d
	movq	%r13, %r10
	movq	%r15, %r11
	xorl	%r14d, %r14d
	movq	%rax, %rcx
	xorl	%r12d, %r12d
	vmovaps	2048(%rsp), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	movq	24(%rsp), %rbx
	vmovaps	%ymm11, 1184(%rsp)
	vmovaps	%ymm12, 2336(%rsp)
	vmovaps	%ymm5, 2304(%rsp)
	vmovaps	%ymm14, 2272(%rsp)
	vmovaps	%ymm15, 608(%rsp)
.Ltmp1428:
	.loc	8 446 20
	jmp	.LBB1_73
.Ltmp1429:
	.loc	8 0 20 is_stmt 0
.Ltmp1430:
	.p2align	4
.LBB1_71:
	vmovaps	%ymm0, %ymm8
.LBB1_72:
	addq	$32, %r12
	decq	%rsi
	movq	2432(%rsp), %rcx
.Ltmp1431:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	addq	$256, %r14
	addq	$-256, %r11
	addq	$1024, %r10
	testq	%rsi, %rsi
	je	.LBB1_334
.LBB1_73:
.Ltmp1432:
	.loc	4 2584 13
	cmpq	$1, %rcx
	movq	%rcx, 2432(%rsp)
	adcq	$0, %rcx
	cmpq	$32, %rcx
	cmovaeq	%r9, %rcx
	movq	%r12, 2368(%rsp)
.Ltmp1433:
	.loc	10 1916 50
	cmpq	%rax, %r12
	movq	%rcx, 2464(%rsp)
.Ltmp1434:
	.loc	3 900 12
	jne	.LBB1_75
	.loc	3 0 12 is_stmt 0
	vmovaps	%ymm13, %ymm1
	vmovaps	%ymm8, 224(%rsp)
	vmovaps	%ymm6, 32(%rsp)
	vmovaps	1216(%rsp), %ymm13
	vmovaps	1280(%rsp), %ymm6
	vmovaps	1248(%rsp), %ymm15
	vmovaps	%ymm3, %ymm8
	vmovaps	%ymm4, %ymm3
	vmovaps	%ymm10, %ymm2
	vmovaps	672(%rsp), %ymm10
	vmovaps	%ymm7, %ymm9
	.loc	3 900 12
	jmp	.LBB1_79
.Ltmp1435:
	.loc	3 0 12
.Ltmp1436:
	.p2align	4
.LBB1_75:
	leal	(,%rcx,8), %eax
	vmovaps	(%rbx), %ymm0
	vmovaps	%ymm0, 352(%rsp)
	vmovaps	32(%rbx), %ymm0
	vmovaps	%ymm0, 512(%rsp)
	vmovaps	64(%rbx), %ymm0
	vmovaps	%ymm0, 704(%rsp)
	vmovaps	96(%rbx), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	128(%rbx), %ymm0
	vmovaps	%ymm0, 1152(%rsp)
	vmovaps	160(%rbx), %ymm0
	vmovaps	%ymm0, 864(%rsp)
	vmovaps	192(%rbx), %ymm0
	vmovaps	%ymm0, 832(%rsp)
	vmovaps	224(%rbx), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	256(%rbx), %ymm0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	288(%rbx), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	320(%rbx), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	movq	%r11, %rdx
	xorl	%ecx, %ecx
	vmovaps	%ymm7, %ymm0
	vmovaps	%ymm10, %ymm14
	vmovaps	672(%rsp), %ymm12
	vmovaps	%ymm3, %ymm11
	vmovaps	352(%rbx), %ymm2
	vmovaps	%ymm2, 672(%rsp)
	vmovaps	384(%rbx), %ymm2
	vmovaps	%ymm2, 1088(%rsp)
	vmovaps	416(%rbx), %ymm2
	vmovaps	%ymm2, 480(%rsp)
	vmovaps	448(%rbx), %ymm2
	vmovaps	%ymm2, 384(%rsp)
	vmovaps	480(%rbx), %ymm2
	vmovaps	%ymm2, 288(%rsp)
	vmovaps	512(%rbx), %ymm2
	vmovaps	%ymm2, 256(%rsp)
	vmovaps	544(%rbx), %ymm2
	vmovaps	%ymm2, 448(%rsp)
	vmovaps	576(%rbx), %ymm2
	vmovaps	%ymm2, 1056(%rsp)
	vmovaps	608(%rbx), %ymm2
	vmovaps	%ymm2, 1024(%rsp)
	vmovaps	640(%rbx), %ymm2
	vmovaps	%ymm2, 992(%rsp)
	vmovaps	672(%rbx), %ymm2
	vmovaps	%ymm2, 960(%rsp)
	vmovaps	704(%rbx), %ymm2
	vmovaps	%ymm2, 2208(%rsp)
	vmovaps	736(%rbx), %ymm2
	vmovaps	%ymm2, 2176(%rsp)
	vmovaps	768(%rbx), %ymm2
	vmovaps	%ymm2, 2144(%rsp)
	vmovaps	800(%rbx), %ymm2
	vmovaps	%ymm2, 2112(%rsp)
	vmovaps	832(%rbx), %ymm2
	vmovaps	%ymm2, 3168(%rsp)
	vmovaps	864(%rbx), %ymm2
	vmovaps	%ymm2, 3136(%rsp)
	vmovaps	896(%rbx), %ymm2
	vmovaps	%ymm2, 3104(%rsp)
	vmovaps	928(%rbx), %ymm2
	vmovaps	%ymm2, 3072(%rsp)
	vmovaps	960(%rbx), %ymm2
	vmovaps	%ymm2, 3040(%rsp)
	vmovaps	992(%rbx), %ymm2
	vmovaps	%ymm2, 3008(%rsp)
	vmovaps	1024(%rbx), %ymm2
	vmovaps	%ymm2, 2976(%rsp)
	vmovaps	1056(%rbx), %ymm2
	vmovaps	%ymm2, 2944(%rsp)
	vmovaps	1088(%rbx), %ymm2
	vmovaps	%ymm2, 2912(%rsp)
	vmovaps	1120(%rbx), %ymm2
	vmovaps	%ymm2, 2880(%rsp)
	vmovaps	1152(%rbx), %ymm2
	vmovaps	%ymm2, 2848(%rsp)
	vmovaps	1184(%rbx), %ymm2
	vmovaps	%ymm2, 2816(%rsp)
	vmovaps	1216(%rbx), %ymm2
	vmovaps	%ymm2, 2784(%rsp)
	vmovaps	1248(%rbx), %ymm2
	vmovaps	%ymm2, 2752(%rsp)
	vmovaps	1280(%rbx), %ymm2
	vmovaps	%ymm2, 2720(%rsp)
	vmovaps	1312(%rbx), %ymm2
	vmovaps	%ymm2, 2688(%rsp)
	vmovaps	1344(%rbx), %ymm2
	vmovaps	%ymm2, 2656(%rsp)
	vmovaps	1376(%rbx), %ymm2
	vmovaps	%ymm2, 2624(%rsp)
	vmovaps	1408(%rbx), %ymm2
	vmovaps	%ymm2, 2592(%rsp)
	vmovaps	1440(%rbx), %ymm2
	vmovaps	%ymm2, 2560(%rsp)
	vmovaps	1472(%rbx), %ymm2
	vmovaps	%ymm2, 2528(%rsp)
	vmovaps	1504(%rbx), %ymm2
	vmovaps	%ymm2, 2496(%rsp)
	vmovaps	%ymm6, 928(%rsp)
	vmovaps	%ymm6, 32(%rsp)
	vmovaps	%ymm8, 128(%rsp)
	vmovaps	%ymm8, 224(%rsp)
	vmovaps	%ymm13, 160(%rsp)
	vmovaps	%ymm13, %ymm1
	vmovaps	1216(%rsp), %ymm13
	vmovaps	1280(%rsp), %ymm6
	vmovaps	1248(%rsp), %ymm15
	.p2align	4
.LBB1_76:
	vmovaps	%ymm1, %ymm3
	vmovaps	224(%rsp), %ymm1
	vmovaps	32(%rsp), %ymm2
.Ltmp1437:
	.loc	5 568 12 is_stmt 1
	leaq	(%r14,%rcx), %rdi
	cmpq	%r15, %rdi
	ja	.LBB1_372
.Ltmp1438:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_374
.Ltmp1439:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm13, %ymm7
	vmovaps	%ymm0, %ymm5
	vmovaps	%ymm6, %ymm0
	vmovaps	%ymm15, %ymm6
.Ltmp1440:
	.loc	11 551 14 is_stmt 1
	vmovups	(%r10,%rcx,4), %ymm13
	vmovaps	%ymm4, %ymm15
.Ltmp1441:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm4
	vmovaps	%ymm1, 544(%rsp)
	vmovaps	%ymm0, %ymm1
.Ltmp1442:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm13, %ymm0
	vxorps	%xmm8, %xmm8, %xmm8
.Ltmp1443:
	.loc	29 48 14
	vaddps	%ymm0, %ymm8, %ymm0
	vmovaps	%ymm12, 416(%rsp)
	vmovaps	%ymm11, %ymm10
.Ltmp1444:
	.loc	29 283 14
	vmulps	512(%rsp), %ymm13, %ymm11
.Ltmp1445:
	.loc	29 48 14
	vaddps	%ymm8, %ymm11, %ymm11
.Ltmp1446:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm13, %ymm12
.Ltmp1447:
	.loc	29 48 14
	vaddps	%ymm8, %ymm12, %ymm12
	vmovaps	%ymm14, %ymm9
.Ltmp1448:
	.loc	29 283 14
	vmulps	896(%rsp), %ymm13, %ymm14
.Ltmp1449:
	.loc	29 48 14
	vaddps	%ymm8, %ymm14, %ymm14
	vmovaps	%ymm3, %ymm8
	vmovaps	%ymm7, %ymm3
.Ltmp1450:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm7, %ymm7
.Ltmp1451:
	.loc	29 48 14
	vaddps	%ymm0, %ymm7, %ymm0
.Ltmp1452:
	.loc	29 283 14
	vmulps	864(%rsp), %ymm3, %ymm7
.Ltmp1453:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm7
.Ltmp1454:
	.loc	29 283 14
	vmulps	832(%rsp), %ymm3, %ymm11
.Ltmp1455:
	.loc	29 48 14
	vaddps	%ymm12, %ymm11, %ymm11
.Ltmp1456:
	.loc	29 283 14
	vmulps	800(%rsp), %ymm3, %ymm12
.Ltmp1457:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp1458:
	.loc	29 283 14
	vmulps	1344(%rsp), %ymm15, %ymm14
.Ltmp1459:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1460:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm15, %ymm14
.Ltmp1461:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1462:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm15, %ymm14
.Ltmp1463:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp1464:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm15, %ymm14
.Ltmp1465:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1466:
	.loc	29 283 14
	vmulps	1088(%rsp), %ymm6, %ymm14
.Ltmp1467:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1468:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm6, %ymm14
.Ltmp1469:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1470:
	.loc	29 283 14
	vmulps	384(%rsp), %ymm6, %ymm14
.Ltmp1471:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp1472:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm6, %ymm14
.Ltmp1473:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1474:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm1, %ymm14
.Ltmp1475:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1476:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm1, %ymm14
.Ltmp1477:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1478:
	.loc	29 283 14
	vmulps	1056(%rsp), %ymm1, %ymm14
.Ltmp1479:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
	vmovaps	%ymm1, 32(%rsp)
.Ltmp1480:
	.loc	29 283 14
	vmulps	1024(%rsp), %ymm1, %ymm14
	vmovaps	544(%rsp), %ymm1
.Ltmp1481:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1482:
	.loc	29 283 14
	vmulps	992(%rsp), %ymm2, %ymm14
.Ltmp1483:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1484:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm2, %ymm14
.Ltmp1485:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1486:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm2, %ymm14
.Ltmp1487:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
	vmovaps	%ymm2, 224(%rsp)
.Ltmp1488:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm2, %ymm14
	vmovaps	416(%rsp), %ymm2
.Ltmp1489:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1490:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm1, %ymm14
.Ltmp1491:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1492:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm1, %ymm14
.Ltmp1493:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1494:
	.loc	29 283 14
	vmulps	3168(%rsp), %ymm1, %ymm14
.Ltmp1495:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp1496:
	.loc	29 283 14
	vmulps	3136(%rsp), %ymm1, %ymm14
.Ltmp1497:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1498:
	.loc	29 283 14
	vmulps	3104(%rsp), %ymm8, %ymm14
.Ltmp1499:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1500:
	.loc	29 283 14
	vmulps	3072(%rsp), %ymm8, %ymm14
.Ltmp1501:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1502:
	.loc	29 283 14
	vmulps	3040(%rsp), %ymm8, %ymm14
.Ltmp1503:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp1504:
	.loc	29 283 14
	vmulps	3008(%rsp), %ymm8, %ymm14
.Ltmp1505:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1506:
	.loc	29 283 14
	vmulps	2976(%rsp), %ymm10, %ymm14
.Ltmp1507:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1508:
	.loc	29 283 14
	vmulps	2944(%rsp), %ymm10, %ymm14
.Ltmp1509:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1510:
	.loc	29 283 14
	vmulps	2912(%rsp), %ymm10, %ymm14
.Ltmp1511:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp1512:
	.loc	29 283 14
	vmulps	2880(%rsp), %ymm10, %ymm14
.Ltmp1513:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1514:
	.loc	29 283 14
	vmulps	2848(%rsp), %ymm2, %ymm14
.Ltmp1515:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1516:
	.loc	29 283 14
	vmulps	2816(%rsp), %ymm2, %ymm14
.Ltmp1517:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1518:
	.loc	29 283 14
	vmulps	2784(%rsp), %ymm2, %ymm14
.Ltmp1519:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp1520:
	.loc	29 283 14
	vmulps	2752(%rsp), %ymm2, %ymm14
.Ltmp1521:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1522:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm9, %ymm14
.Ltmp1523:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1524:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm9, %ymm14
.Ltmp1525:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1526:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm9, %ymm14
.Ltmp1527:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp1528:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm9, %ymm14
.Ltmp1529:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1530:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm5, %ymm14
.Ltmp1531:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1532:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm5, %ymm14
.Ltmp1533:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1534:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm5, %ymm14
.Ltmp1535:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
	vmovaps	%ymm5, 1120(%rsp)
.Ltmp1536:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm5, %ymm14
.Ltmp1537:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1538:
	.loc	29 82 19
	vandps	%ymm4, %ymm1, %ymm14
.Ltmp1539:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm4, %ymm0, %ymm0
.Ltmp1540:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm14, %ymm0
.Ltmp1541:
	.loc	29 82 19
	vandps	%ymm4, %ymm7, %ymm7
.Ltmp1542:
	.loc	29 233 14
	vmaxps	%ymm7, %ymm0, %ymm0
.Ltmp1543:
	.loc	29 82 19
	vandps	%ymm4, %ymm11, %ymm7
.Ltmp1544:
	.loc	29 233 14
	vmaxps	%ymm7, %ymm0, %ymm0
.Ltmp1545:
	.loc	29 82 19
	vandps	%ymm4, %ymm12, %ymm4
.Ltmp1546:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm0, %ymm0
.Ltmp1547:
	.loc	11 551 14
	vmovups	%ymm0, 4640(%rsp,%rcx,4)
.Ltmp1548:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm3, %ymm4
	vmovaps	%ymm9, %ymm0
	vmovaps	%ymm2, %ymm14
	vmovaps	%ymm10, %ymm12
	vmovaps	%ymm8, %ymm11
	cmpq	%rcx, %rax
.Ltmp1549:
	.loc	3 900 12
	jne	.LBB1_76
.Ltmp1550:
.LBB1_79:
	.loc	3 0 12 is_stmt 0
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	%ymm3, %ymm4
	vmovaps	%ymm15, 1248(%rsp)
	vmovaps	%ymm6, 1280(%rsp)
	vmovaps	32(%rsp), %ymm6
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm1, %ymm13
.Ltmp1551:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm8, 1600(%rsp)
	vmovaps	%ymm10, 672(%rsp)
	vmovaps	%ymm10, 1632(%rsp)
	vmovaps	%ymm2, 1664(%rsp)
	vmovaps	%ymm9, %ymm7
	vmovaps	%ymm9, 1696(%rsp)
	vmovaps	1120(%rsp), %ymm1
	vmovaps	%ymm1, 1728(%rsp)
	movq	8(%rsp), %rax
	movq	2368(%rsp), %r12
.Ltmp1552:
	.loc	10 1916 50
	cmpq	%rax, %r12
	vmovaps	%ymm8, %ymm3
	vmovaps	%ymm2, %ymm10
.Ltmp1553:
	.loc	3 900 12
	je	.LBB1_71
.Ltmp1554:
	.loc	3 0 12 is_stmt 0
	vmovaps	%ymm3, 768(%rsp)
	movq	%r14, 1312(%rsp)
	movq	%r11, 1344(%rsp)
	movq	%r10, 800(%rsp)
	movq	%rsi, 832(%rsp)
	movq	1624(%rbx), %r14
	movq	1632(%rbx), %rax
	movq	%rax, 864(%rsp)
	xorl	%r9d, %r9d
	vmovaps	1184(%rsp), %ymm11
	vmovaps	2336(%rsp), %ymm12
	vmovaps	2304(%rsp), %ymm5
	vmovaps	2272(%rsp), %ymm14
	vmovaps	608(%rsp), %ymm15
	vbroadcastss	.LCPI1_4(%rip), %ymm3
	vmovaps	%ymm0, %ymm8
	.p2align	4
.LBB1_81:
.Ltmp1555:
	.loc	1 3278 24 is_stmt 1
	leaq	(%r9,%r12), %rdi
	shlq	$3, %rdi
.Ltmp1556:
	.loc	5 568 12
	movq	%r15, %rdx
	subq	%rdi, %rdx
	jb	.LBB1_375
.Ltmp1557:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_357
.Ltmp1558:
	.loc	1 1394 25
	movq	1688(%rbx), %rsi
.Ltmp1559:
	.loc	1 1390 17
	movq	1840(%rbx), %rax
.Ltmp1560:
	.loc	1 1394 45
	movq	%rax, %r10
	imulq	96(%rsp), %r10
.Ltmp1561:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r10, %rdx
	jb	.LBB1_376
.Ltmp1562:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_357
.Ltmp1563:
	.loc	5 0 16 is_stmt 0
	movq	%rax, 352(%rsp)
	movq	%rdi, 1152(%rsp)
	movq	%r8, 704(%rsp)
	movq	%r9, 896(%rsp)
	movq	%r9, %rax
	shlq	$5, %rax
	vmovups	4640(%rsp,%rax), %ymm0
.Ltmp1564:
	vmaxps	%ymm0, %ymm0, %ymm1
.Ltmp1565:
	vblendvps	%ymm12, %ymm1, %ymm0, %ymm0
.Ltmp1566:
	vdivps	%ymm0, %ymm5, %ymm1
	vcmpgt_oqps	%ymm5, %ymm0, %ymm0
	vbroadcastss	.LCPI1_2(%rip), %ymm2
	vblendvps	%ymm0, %ymm1, %ymm2, %ymm0
.Ltmp1567:
	.loc	1 1394 25 is_stmt 1
	movq	1680(%rbx), %rax
	movq	%r10, 512(%rsp)
.Ltmp1568:
	.loc	11 551 14
	vmovups	%ymm0, (%rax,%r10,4)
.Ltmp1569:
	.loc	1 1261 17
	movq	1840(%rbx), %rdx
.Ltmp1570:
	.loc	12 37 12
	testq	%rdx, %rdx
	je	.LBB1_104
.Ltmp1571:
	.loc	12 0 12 is_stmt 0
	movq	24(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 224(%rsp)
	movq	96(%rsp), %rax
	leaq	1(%rax), %r9
	cmpq	%r14, %r9
	movq	%r14, %rax
	movl	$0, %ecx
	cmovbq	%rcx, %rax
	movq	1824(%rdi), %rcx
	subq	%rax, %r9
	movq	1680(%rdi), %rax
	movq	1688(%rdi), %rsi
	movq	1768(%rdi), %r8
	movq	%r8, 544(%rsp)
	movq	1760(%rdi), %r15
	movq	1736(%rdi), %r8
	movq	%r8, 32(%rsp)
	movq	1728(%rdi), %rbx
	imulq	%rdx, %r9
	movq	%r9, 416(%rsp)
	movq	%rdx, %r13
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB1_89
	.p2align	4
.LBB1_87:
	xorl	%r9d, %r9d
.LBB1_88:
	decq	%r13
	addq	$4, %rdi
.Ltmp1572:
	movl	%r9d, (%r15,%r11,4)
.Ltmp1573:
	incq	%r11
.Ltmp1574:
	.loc	12 37 12 is_stmt 1
	testq	%r13, %r13
	je	.LBB1_104
.LBB1_89:
.Ltmp1575:
	.loc	7 1714 9
	cmpq	$32, %rdi
.Ltmp1576:
	.loc	6 180 28
	je	.LBB1_104
.Ltmp1577:
	.loc	1 1265 21
	cmpq	224(%rsp), %r11
	je	.LBB1_398
	leaq	(%r11,%r11,2), %r9
	movl	4(%rcx,%r9,4), %r8d
.Ltmp1578:
	.loc	1 1267 23
	addq	96(%rsp), %r8
.Ltmp1579:
	.loc	1 1268 12
	cmpq	%r14, %r8
	movl	$0, %r10d
	cmovaeq	%r14, %r10
	subq	%r10, %r8
.Ltmp1580:
	.loc	1 1275 42
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1275 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_400
.Ltmp1581:
	.loc	1 1276 24 is_stmt 1
	cmpq	544(%rsp), %r11
	je	.LBB1_401
.Ltmp1582:
	.loc	1 0 0 is_stmt 0
	movl	(%rcx,%r9,4), %r10d
.Ltmp1583:
	vmovss	(%rax,%r12,4), %xmm0
.Ltmp1584:
	.loc	1 1276 24
	movl	(%r15,%r11,4), %r9d
	testq	%r9, %r9
	je	.LBB1_97
.Ltmp1585:
	.loc	1 1280 24 is_stmt 1
	cmpq	32(%rsp), %r11
	jae	.LBB1_402
	vmovss	(%rbx,%r11,4), %xmm1
.Ltmp1586:
	.loc	1 905 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB1_97
.Ltmp1587:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB1_97:
.Ltmp1588:
	.loc	1 1282 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB1_399
	vmovss	%xmm0, (%rbx,%r11,4)
	.loc	1 1283 24
	incq	%r9
	cmpq	%r10, %r9
.Ltmp1589:
	.loc	1 1284 23
	jne	.LBB1_102
	.loc	1 1284 9 is_stmt 0
	vmovss	%xmm0, 3264(%rsp,%rdi)
	.loc	1 1290 30 is_stmt 1
	vmovss	(%rax,%r12,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp1590:
	.p2align	4
.LBB1_100:
.Ltmp1591:
	.loc	1 1293 65 is_stmt 1
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1293 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_384
.Ltmp1592:
	.loc	1 905 8 is_stmt 1
	vminss	(%rax,%r12,4), %xmm0, %xmm0
.Ltmp1593:
	.loc	1 1294 17
	vmovss	%xmm0, (%rax,%r12,4)
	.loc	1 1295 20
	testq	%r8, %r8
	cmoveq	%r14, %r8
	.loc	1 1298 17
	decq	%r8
.Ltmp1594:
	.loc	10 1916 50
	decq	%r10
.Ltmp1595:
	.loc	3 900 12
	jne	.LBB1_100
	jmp	.LBB1_87
.Ltmp1596:
	.loc	3 0 12 is_stmt 0
.Ltmp1597:
	.p2align	4
.LBB1_102:
	movq	416(%rsp), %r8
	.loc	1 1287 44 is_stmt 1
	leaq	(%r11,%r8), %r12
	.loc	1 1287 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_404
	vmovss	(%rax,%r12,4), %xmm1
.Ltmp1598:
	.loc	1 905 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp1599:
	.loc	1 1284 9
	vmovss	%xmm0, 3264(%rsp,%rdi)
	jmp	.LBB1_88
.Ltmp1600:
	.loc	1 0 9 is_stmt 0
.Ltmp1601:
	.p2align	4
.LBB1_104:
	.loc	11 551 14 is_stmt 1
	vmovaps	3264(%rsp), %ymm2
	movq	24(%rsp), %rbx
.Ltmp1602:
	.loc	1 1414 26
	movq	1704(%rbx), %rsi
	vmovaps	%ymm2, %ymm0
	movq	352(%rsp), %rdi
.Ltmp1603:
	.loc	12 37 12
	testq	%rdi, %rdi
	movq	216(%rsp), %r13
	movq	512(%rsp), %r11
	je	.LBB1_130
.Ltmp1604:
	.loc	12 0 12 is_stmt 0
	movq	1832(%rbx), %r9
.Ltmp1605:
	.loc	1 1405 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB1_410
	.loc	1 0 42 is_stmt 0
	movq	1824(%rbx), %r10
	.loc	1 1405 42
	movl	8(%r10), %r8d
	.loc	1 1405 28
	addq	96(%rsp), %r8
.Ltmp1606:
	.loc	1 1406 16 is_stmt 1
	cmpq	%r14, %r8
	movl	$0, %eax
	cmovaeq	%r14, %rax
	subq	%rax, %r8
	.loc	1 1409 40
	imulq	%rdi, %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_407
	.loc	1 0 25
	movq	1696(%rbx), %rdx
	.loc	1 1409 25
	vmovss	(%rdx,%r8,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 3264(%rsp)
.Ltmp1607:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB1_129
.Ltmp1608:
	.loc	1 1405 42
	cmpq	$1, %r9
	je	.LBB1_412
	movl	20(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1609:
	.loc	1 1406 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_407
	vmovss	4(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 3268(%rsp)
.Ltmp1610:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB1_129
.Ltmp1611:
	.loc	1 1405 42
	cmpq	$2, %r9
	je	.LBB1_414
	movl	32(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1612:
	.loc	1 1406 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_407
	vmovss	8(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 3272(%rsp)
.Ltmp1613:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB1_129
.Ltmp1614:
	.loc	1 1405 42
	cmpq	$3, %r9
	je	.LBB1_424
	movl	44(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1615:
	.loc	1 1406 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_407
	vmovss	12(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 3276(%rsp)
.Ltmp1616:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB1_129
.Ltmp1617:
	.loc	1 1405 42
	cmpq	$4, %r9
	je	.LBB1_427
	movl	56(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1618:
	.loc	1 1406 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_407
	vmovss	16(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 3280(%rsp)
.Ltmp1619:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB1_129
.Ltmp1620:
	.loc	1 1405 42
	cmpq	$5, %r9
	je	.LBB1_431
	movl	68(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1621:
	.loc	1 1406 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_407
	vmovss	20(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 3284(%rsp)
.Ltmp1622:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB1_129
.Ltmp1623:
	.loc	1 1405 42
	cmpq	$6, %r9
	je	.LBB1_432
	movl	80(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1624:
	.loc	1 1406 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_407
	vmovss	24(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 3288(%rsp)
.Ltmp1625:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB1_129
.Ltmp1626:
	.loc	1 1405 42
	cmpq	$7, %r9
	je	.LBB1_433
	movl	92(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1627:
	.loc	1 1406 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_407
	vmovss	28(%rdx,%rax,4), %xmm0
	.loc	1 1409 13
	vmovss	%xmm0, 3292(%rsp)
.Ltmp1628:
	.loc	1 0 13
.Ltmp1629:
	.p2align	4
.LBB1_129:
	.loc	11 551 14 is_stmt 1
	vmovaps	3264(%rsp), %ymm0
.Ltmp1630:
.LBB1_130:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r11, %rdx
	movq	88(%rsp), %r15
	movq	2368(%rsp), %r12
	jb	.LBB1_377
.Ltmp1631:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_357
.Ltmp1632:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI1_3(%rip), %ymm1
	vmulps	%ymm1, %ymm2, %ymm1
	vroundps	$9, %ymm1, %ymm1
	vmulps	%ymm3, %ymm1, %ymm1
.Ltmp1633:
	vaddps	320(%rsp), %ymm1, %ymm2
	vsubps	%ymm0, %ymm2, %ymm0
	.loc	1 1414 26 is_stmt 1
	movq	1696(%rbx), %rax
.Ltmp1634:
	.loc	11 551 14
	vmovups	%ymm1, (%rax,%r11,4)
	vmovaps	%ymm0, 320(%rsp)
.Ltmp1635:
	.loc	29 360 14
	vdivps	%ymm15, %ymm0, %ymm0
.Ltmp1636:
	.loc	1 1418 43
	vmovaps	2016(%rsp), %ymm1
.Ltmp1637:
	.loc	29 347 14
	vbroadcastss	.LCPI1_2(%rip), %ymm2
	vsubps	%ymm0, %ymm2, %ymm0
.Ltmp1638:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm1, %ymm0, %ymm2
.Ltmp1639:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm2, %ymm14, %ymm2
.Ltmp1640:
	.loc	29 48 14
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp1641:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp1642:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm1
	vandps	%ymm1, %ymm0, %ymm1
.Ltmp1643:
	.loc	29 871 14
	vbroadcastss	.LCPI1_5(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp1644:
	.loc	29 82 19
	vandnps	%ymm0, %ymm1, %ymm0
.Ltmp1645:
	.loc	1 1419 5
	vmovaps	%ymm0, 2016(%rsp)
.Ltmp1646:
	.loc	1 1422 28
	movq	1672(%rbx), %rsi
	movq	704(%rsp), %r8
	.loc	1 1422 44 is_stmt 0
	imulq	%r8, %rdi
.Ltmp1647:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB1_378
.Ltmp1648:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_357
.Ltmp1649:
	.loc	5 0 16 is_stmt 0
	movq	896(%rsp), %r9
	incq	%r9
	movq	1152(%rsp), %rax
.Ltmp1650:
	leaq	(,%rax,4), %rax
	addq	%r13, %rax
.Ltmp1651:
	vbroadcastss	.LCPI1_2(%rip), %ymm1
	vsubps	%ymm0, %ymm1, %ymm0
.Ltmp1652:
	.loc	1 1422 28 is_stmt 1
	movq	1664(%rbx), %rcx
.Ltmp1653:
	.loc	11 551 14
	vmovups	(%rcx,%rdi,4), %ymm1
.Ltmp1654:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rax), %ymm2
	vmovups	%ymm2, (%rcx,%rdi,4)
.Ltmp1655:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp1656:
	.loc	29 585 19
	vblendvps	%ymm11, %ymm1, %ymm0, %ymm0
.Ltmp1657:
	.loc	11 551 14
	vmovups	%ymm0, (%rax)
.Ltmp1658:
	.loc	1 3302 13
	incq	%r8
	.loc	1 3303 16
	cmpq	864(%rsp), %r8
	movl	$0, %edx
	cmoveq	%rdx, %r8
	movq	96(%rsp), %rax
	.loc	1 3306 13
	incq	%rax
	.loc	1 3307 16
	cmpq	%r14, %rax
	movl	$0, %ecx
	movq	%rcx, 640(%rsp)
	cmoveq	%rdx, %rax
	movq	%rax, 96(%rsp)
.Ltmp1659:
	.loc	10 1916 50
	cmpq	2464(%rsp), %r9
.Ltmp1660:
	.loc	3 900 12
	jne	.LBB1_81
.Ltmp1661:
	.loc	3 0 12 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp1662:
	.loc	1 1413 0 is_stmt 1
	vmovaps	%ymm0, 2048(%rsp)
	movq	8(%rsp), %rax
	movq	832(%rsp), %rsi
	movl	$32, %r9d
	movq	800(%rsp), %r10
	movq	1344(%rsp), %r11
	movq	1312(%rsp), %r14
	vmovaps	768(%rsp), %ymm3
	jmp	.LBB1_72
.Ltmp1663:
.LBB1_136:
	.loc	1 1119 12
	movq	1768(%rbx), %rax
	testq	%rax, %rax
	je	.LBB1_217
	.loc	1 0 12 is_stmt 0
	movq	1760(%rbx), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB1_138:
.Ltmp1664:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp1665:
	.loc	6 180 28
	je	.LBB1_217
.Ltmp1666:
	.loc	6 315 25
	movl	(%rcx,%rdx), %esi
	addq	$4, %rdx
.Ltmp1667:
	.loc	1 1119 43
	cmpl	(%rcx), %esi
.Ltmp1668:
	.loc	6 315 25
	je	.LBB1_138
.Ltmp1669:
.LBB1_140:
	.loc	6 0 25 is_stmt 0
	leaq	3264(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp1670:
	.loc	1 3254 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp1671:
	.loc	1 3259 19
	movzbl	1536(%rbx), %r14d
	movq	%rbx, %rax
.Ltmp1672:
	.loc	1 3260 21
	movzbl	1537(%rbx), %ebx
.Ltmp1673:
	.loc	1 3261 27
	movl	1640(%rax), %r12d
.Ltmp1674:
	.loc	1 3262 27
	movl	1644(%rax), %eax
	movq	%rax, 96(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 576(%rsp)
	leaq	4640(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
	movq	8(%rsp), %rax
.Ltmp1675:
	.loc	4 3758 16
	leaq	31(%rax), %rcx
	shrq	$5, %rcx
.Ltmp1676:
	.loc	8 446 20
	je	.LBB1_215
.Ltmp1677:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm15
.Ltmp1678:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm15, %ymm0
	vmovaps	%ymm15, 1184(%rsp)
	testb	%r14b, %r14b
	jne	.LBB1_143
.Ltmp1679:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 1184(%rsp)
.LBB1_143:
	testb	%bl, %bl
	movq	%r12, %r11
	jne	.LBB1_145
	vmovaps	%ymm0, %ymm15
.LBB1_145:
	vmovaps	3680(%rsp), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	3808(%rsp), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	3968(%rsp), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	3264(%rsp), %ymm10
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	3328(%rsp), %ymm0
	vmovaps	%ymm0, 160(%rsp)
	vmovaps	3360(%rsp), %ymm0
	vmovaps	%ymm0, 704(%rsp)
	vmovaps	3392(%rsp), %ymm1
	vmovaps	3424(%rsp), %ymm0
	vmovaps	%ymm0, 512(%rsp)
	vmovaps	3456(%rsp), %ymm0
	vmovaps	%ymm0, 928(%rsp)
	vmovaps	3488(%rsp), %ymm4
	vmovaps	3520(%rsp), %ymm5
	vmovaps	3552(%rsp), %ymm3
	vmovaps	3584(%rsp), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	3616(%rsp), %ymm0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	3744(%rsp), %ymm9
	movq	$0, 608(%rsp)
	movq	%r13, 768(%rsp)
	movq	%r15, %rdx
	movq	$0, 1312(%rsp)
	movq	%rax, %r14
	xorl	%r9d, %r9d
	vmovaps	3648(%rsp), %ymm2
	vmovaps	%ymm2, 896(%rsp)
	vmovaps	3712(%rsp), %ymm8
	vmovaps	3872(%rsp), %ymm0
	vmovaps	3776(%rsp), %ymm2
	vmovaps	%ymm2, 1152(%rsp)
	vmovaps	3840(%rsp), %ymm13
	vmovaps	3936(%rsp), %ymm2
	vmovaps	%ymm2, 1120(%rsp)
	movq	24(%rsp), %rbx
	vmovaps	%ymm15, 4000(%rsp)
.Ltmp1680:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB1_148
.Ltmp1681:
	.loc	8 0 20 is_stmt 0
.Ltmp1682:
	.p2align	4
.LBB1_146:
	vmovaps	2400(%rsp), %ymm9
	vmovaps	%ymm8, %ymm10
	vmovaps	3232(%rsp), %ymm8
	vmovaps	2240(%rsp), %ymm0
.LBB1_147:
	addq	$32, %r9
	movq	4064(%rsp), %rcx
	decq	%rcx
.Ltmp1683:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %r14
	addq	$256, 1312(%rsp)
	movq	4032(%rsp), %rdx
	addq	$-256, %rdx
	addq	$1024, 768(%rsp)
	testq	%rcx, %rcx
	je	.LBB1_271
.LBB1_148:
	.loc	8 0 20 is_stmt 0
	vmovaps	%ymm13, 4096(%rsp)
	vmovaps	%ymm10, %ymm11
	vmovaps	%ymm9, 2400(%rsp)
	vmovaps	%ymm1, %ymm9
	movq	%rcx, 4064(%rsp)
.Ltmp1684:
	.loc	4 2584 13 is_stmt 1
	cmpq	$1, %r14
	movq	%r14, %rsi
	adcq	$0, %rsi
	cmpq	$32, %rsi
	movl	$32, %ecx
	cmovaeq	%rcx, %rsi
.Ltmp1685:
	.loc	10 1916 50
	cmpq	%rax, %r9
	movq	%rsi, 1248(%rsp)
	movq	%rdx, 4032(%rsp)
	vmovaps	%ymm8, 3232(%rsp)
	vmovaps	%ymm0, 2240(%rsp)
.Ltmp1686:
	.loc	3 900 12
	jne	.LBB1_150
	.loc	3 0 12 is_stmt 0
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	vmovaps	128(%rsp), %ymm2
	vmovaps	512(%rsp), %ymm1
	vmovaps	%ymm11, %ymm8
	vmovaps	256(%rsp), %ymm7
	vmovaps	928(%rsp), %ymm10
	.loc	3 900 12
	jmp	.LBB1_154
.Ltmp1687:
	.loc	3 0 12
.Ltmp1688:
	.p2align	4
.LBB1_150:
	leal	(,%rsi,8), %ecx
	vmovaps	(%rbx), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	32(%rbx), %ymm0
	vmovaps	%ymm0, 416(%rsp)
	vmovaps	64(%rbx), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	96(%rbx), %ymm0
	vmovaps	%ymm0, 352(%rsp)
	vmovaps	128(%rbx), %ymm0
	vmovaps	%ymm0, 864(%rsp)
	vmovaps	160(%rbx), %ymm0
	vmovaps	%ymm0, 832(%rsp)
	vmovaps	192(%rbx), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	224(%rbx), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovaps	256(%rbx), %ymm0
	vmovaps	%ymm0, 1088(%rsp)
	vmovaps	288(%rbx), %ymm0
	vmovaps	%ymm0, 480(%rsp)
	vmovaps	320(%rbx), %ymm0
	vmovaps	%ymm0, 384(%rsp)
	vmovaps	352(%rbx), %ymm0
	vmovaps	%ymm0, 288(%rsp)
	xorl	%esi, %esi
	vmovaps	256(%rsp), %ymm6
	vmovaps	%ymm3, %ymm15
	vmovaps	%ymm5, %ymm14
	vmovaps	%ymm4, %ymm13
	vmovaps	384(%rbx), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	416(%rbx), %ymm0
	vmovaps	%ymm0, 448(%rsp)
	vmovaps	448(%rbx), %ymm0
	vmovaps	%ymm0, 1056(%rsp)
	vmovaps	480(%rbx), %ymm0
	vmovaps	%ymm0, 1024(%rsp)
	vmovaps	512(%rbx), %ymm0
	vmovaps	%ymm0, 992(%rsp)
	vmovaps	544(%rbx), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	576(%rbx), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	608(%rbx), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	640(%rbx), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	672(%rbx), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	704(%rbx), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
	vmovaps	736(%rbx), %ymm0
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	768(%rbx), %ymm0
	vmovaps	%ymm0, 3104(%rsp)
	vmovaps	800(%rbx), %ymm0
	vmovaps	%ymm0, 3072(%rsp)
	vmovaps	832(%rbx), %ymm0
	vmovaps	%ymm0, 3040(%rsp)
	vmovaps	864(%rbx), %ymm0
	vmovaps	%ymm0, 3008(%rsp)
	vmovaps	896(%rbx), %ymm0
	vmovaps	%ymm0, 2976(%rsp)
	vmovaps	928(%rbx), %ymm0
	vmovaps	%ymm0, 2944(%rsp)
	vmovaps	960(%rbx), %ymm0
	vmovaps	%ymm0, 2912(%rsp)
	vmovaps	992(%rbx), %ymm0
	vmovaps	%ymm0, 2880(%rsp)
	vmovaps	1024(%rbx), %ymm0
	vmovaps	%ymm0, 2848(%rsp)
	vmovaps	1056(%rbx), %ymm0
	vmovaps	%ymm0, 2816(%rsp)
	vmovaps	1088(%rbx), %ymm0
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	1120(%rbx), %ymm0
	vmovaps	%ymm0, 2752(%rsp)
	vmovaps	1152(%rbx), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	1184(%rbx), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	1216(%rbx), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	1248(%rbx), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
	vmovaps	1280(%rbx), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	1312(%rbx), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	1344(%rbx), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	1376(%rbx), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1408(%rbx), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1440(%rbx), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1472(%rbx), %ymm0
	vmovaps	%ymm0, 640(%rsp)
	vmovaps	1504(%rbx), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	128(%rsp), %ymm12
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	vmovaps	512(%rsp), %ymm1
	vmovaps	%ymm11, %ymm8
	vmovaps	928(%rsp), %ymm10
	.p2align	4
.LBB1_151:
	vmovaps	%ymm10, %ymm4
	vmovaps	%ymm14, 224(%rsp)
	vmovaps	%ymm1, %ymm10
	vmovaps	%ymm9, %ymm2
	vmovaps	704(%rsp), %ymm9
	vmovaps	32(%rsp), %ymm1
	movq	1312(%rsp), %rax
.Ltmp1689:
	.loc	5 568 12 is_stmt 1
	addq	%rsi, %rax
	cmpq	%r15, %rax
	ja	.LBB1_329
.Ltmp1690:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_269
.Ltmp1691:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm8, %ymm11
	vmovaps	%ymm13, %ymm3
	vmovaps	%ymm6, %ymm0
	vmovaps	%ymm2, 512(%rsp)
	movq	768(%rsp), %rax
.Ltmp1692:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rax,%rsi,4), %ymm8
.Ltmp1693:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm6
	vmovaps	%ymm12, %ymm5
.Ltmp1694:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm8, %ymm12
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp1695:
	.loc	29 48 14
	vaddps	%ymm2, %ymm12, %ymm12
.Ltmp1696:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm8, %ymm13
.Ltmp1697:
	.loc	29 48 14
	vaddps	%ymm2, %ymm13, %ymm13
.Ltmp1698:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm8, %ymm14
.Ltmp1699:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm14
	vmovaps	%ymm15, %ymm7
.Ltmp1700:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm8, %ymm15
.Ltmp1701:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm15
.Ltmp1702:
	.loc	29 283 14
	vmulps	864(%rsp), %ymm11, %ymm2
.Ltmp1703:
	.loc	29 48 14
	vaddps	%ymm2, %ymm12, %ymm2
.Ltmp1704:
	.loc	29 283 14
	vmulps	832(%rsp), %ymm11, %ymm12
.Ltmp1705:
	.loc	29 48 14
	vaddps	%ymm13, %ymm12, %ymm12
.Ltmp1706:
	.loc	29 283 14
	vmulps	800(%rsp), %ymm11, %ymm13
.Ltmp1707:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp1708:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm11, %ymm14
.Ltmp1709:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp1710:
	.loc	29 283 14
	vmulps	1088(%rsp), %ymm5, %ymm15
.Ltmp1711:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1712:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm5, %ymm15
.Ltmp1713:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1714:
	.loc	29 283 14
	vmulps	384(%rsp), %ymm5, %ymm15
.Ltmp1715:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
	vmovaps	%ymm5, 32(%rsp)
.Ltmp1716:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm5, %ymm15
.Ltmp1717:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1718:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm1, %ymm15
.Ltmp1719:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1720:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm1, %ymm15
.Ltmp1721:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1722:
	.loc	29 283 14
	vmulps	1056(%rsp), %ymm1, %ymm15
.Ltmp1723:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
	vmovaps	%ymm1, 704(%rsp)
.Ltmp1724:
	.loc	29 283 14
	vmulps	1024(%rsp), %ymm1, %ymm15
	vmovaps	512(%rsp), %ymm1
.Ltmp1725:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1726:
	.loc	29 283 14
	vmulps	992(%rsp), %ymm9, %ymm15
.Ltmp1727:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1728:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm9, %ymm15
.Ltmp1729:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1730:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm9, %ymm15
.Ltmp1731:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1732:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm9, %ymm15
.Ltmp1733:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1734:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm1, %ymm15
.Ltmp1735:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1736:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm1, %ymm15
.Ltmp1737:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1738:
	.loc	29 283 14
	vmulps	3168(%rsp), %ymm1, %ymm15
.Ltmp1739:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1740:
	.loc	29 283 14
	vmulps	3136(%rsp), %ymm1, %ymm15
.Ltmp1741:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1742:
	.loc	29 283 14
	vmulps	3104(%rsp), %ymm10, %ymm15
.Ltmp1743:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1744:
	.loc	29 283 14
	vmulps	3072(%rsp), %ymm10, %ymm15
.Ltmp1745:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1746:
	.loc	29 283 14
	vmulps	3040(%rsp), %ymm10, %ymm15
.Ltmp1747:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1748:
	.loc	29 283 14
	vmulps	3008(%rsp), %ymm10, %ymm15
.Ltmp1749:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1750:
	.loc	29 283 14
	vmulps	2976(%rsp), %ymm4, %ymm15
.Ltmp1751:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1752:
	.loc	29 283 14
	vmulps	2944(%rsp), %ymm4, %ymm15
.Ltmp1753:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1754:
	.loc	29 283 14
	vmulps	2912(%rsp), %ymm4, %ymm15
.Ltmp1755:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1756:
	.loc	29 283 14
	vmulps	2880(%rsp), %ymm4, %ymm15
.Ltmp1757:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
	vmovaps	%ymm3, %ymm5
.Ltmp1758:
	.loc	29 283 14
	vmulps	2848(%rsp), %ymm3, %ymm15
.Ltmp1759:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1760:
	.loc	29 283 14
	vmulps	2816(%rsp), %ymm3, %ymm15
.Ltmp1761:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1762:
	.loc	29 283 14
	vmulps	2784(%rsp), %ymm3, %ymm15
.Ltmp1763:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1764:
	.loc	29 283 14
	vmulps	2752(%rsp), %ymm3, %ymm15
.Ltmp1765:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
	vmovaps	224(%rsp), %ymm3
.Ltmp1766:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm3, %ymm15
.Ltmp1767:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1768:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm3, %ymm15
.Ltmp1769:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1770:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm3, %ymm15
.Ltmp1771:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1772:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm3, %ymm15
.Ltmp1773:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1774:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm7, %ymm15
.Ltmp1775:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1776:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm7, %ymm15
.Ltmp1777:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1778:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm7, %ymm15
.Ltmp1779:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1780:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm7, %ymm15
.Ltmp1781:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1782:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm0, %ymm15
.Ltmp1783:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp1784:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm0, %ymm15
.Ltmp1785:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1786:
	.loc	29 283 14
	vmulps	640(%rsp), %ymm0, %ymm15
.Ltmp1787:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
	vmovaps	%ymm0, 1344(%rsp)
.Ltmp1788:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm0, %ymm15
.Ltmp1789:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1790:
	.loc	29 82 19
	vandps	%ymm6, %ymm10, %ymm15
.Ltmp1791:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm6, %ymm2, %ymm2
.Ltmp1792:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm2, %ymm15, %ymm2
.Ltmp1793:
	.loc	29 82 19
	vandps	%ymm6, %ymm12, %ymm12
.Ltmp1794:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm2, %ymm2
.Ltmp1795:
	.loc	29 82 19
	vandps	%ymm6, %ymm13, %ymm12
.Ltmp1796:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm2, %ymm2
.Ltmp1797:
	.loc	29 82 19
	vandps	%ymm6, %ymm14, %ymm6
.Ltmp1798:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm2, %ymm2
.Ltmp1799:
	.loc	11 551 14
	vmovups	%ymm2, 4640(%rsp,%rsi,4)
	vmovaps	%ymm11, %ymm2
.Ltmp1800:
	.loc	10 1916 50
	addq	$8, %rsi
	addq	$-8, %rdx
	vmovaps	%ymm11, %ymm12
	vmovaps	%ymm7, %ymm6
	vmovaps	%ymm3, %ymm15
	vmovaps	%ymm5, %ymm14
	vmovaps	%ymm4, %ymm13
	cmpq	%rsi, %rcx
.Ltmp1801:
	.loc	3 900 12
	jne	.LBB1_151
.Ltmp1802:
.LBB1_154:
	.loc	3 0 12 is_stmt 0
	vmovaps	%ymm2, 128(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 160(%rsp)
	vmovaps	%ymm1, %ymm2
	vmovaps	704(%rsp), %ymm1
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm1, 3360(%rsp)
	vmovaps	%ymm9, 3392(%rsp)
	vmovaps	%ymm2, 512(%rsp)
	vmovaps	%ymm2, 3424(%rsp)
	vmovaps	%ymm10, 928(%rsp)
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	%ymm4, 3488(%rsp)
	vmovaps	%ymm5, 3520(%rsp)
	vmovaps	%ymm3, 3552(%rsp)
	vmovaps	%ymm7, 256(%rsp)
	vmovaps	%ymm7, 3584(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 3616(%rsp)
	movq	8(%rsp), %rax
.Ltmp1803:
	.loc	10 1916 50
	cmpq	%rax, %r9
	vmovaps	%ymm9, %ymm1
	vmovaps	4096(%rsp), %ymm13
.Ltmp1804:
	.loc	3 900 12
	je	.LBB1_146
.Ltmp1805:
	.loc	3 0 12 is_stmt 0
	movq	1624(%rbx), %rcx
	movq	1632(%rbx), %rax
	movq	%rax, 672(%rsp)
	xorl	%eax, %eax
	vmovaps	4000(%rsp), %ymm15
	vmovaps	2400(%rsp), %ymm9
	vmovaps	%ymm8, %ymm10
	vmovaps	3232(%rsp), %ymm8
	vmovaps	2240(%rsp), %ymm0
	movq	%r14, 1216(%rsp)
	movq	%r9, 2432(%rsp)
	.p2align	4
.LBB1_156:
.Ltmp1806:
	.loc	1 3278 24 is_stmt 1
	leaq	(%rax,%r9), %rdi
	shlq	$3, %rdi
.Ltmp1807:
	.loc	5 568 12
	movq	%r15, %rdx
	subq	%rdi, %rdx
	jb	.LBB1_330
.Ltmp1808:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_269
.Ltmp1809:
	.loc	5 0 16 is_stmt 0
	movq	%rdi, 832(%rsp)
.Ltmp1810:
	.loc	1 1394 25 is_stmt 1
	movq	1688(%rbx), %rsi
.Ltmp1811:
	.loc	1 1390 17
	movq	1840(%rbx), %rdi
.Ltmp1812:
	.loc	1 1394 45
	movq	%rdi, %r8
	imulq	96(%rsp), %r8
.Ltmp1813:
	.loc	5 580 12
	movq	%rsi, %rdx
	movq	%r8, 320(%rsp)
	subq	%r8, %rdx
	jb	.LBB1_331
.Ltmp1814:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_269
.Ltmp1815:
	.loc	5 0 16 is_stmt 0
	movq	%rdi, 352(%rsp)
	movq	%r11, 864(%rsp)
	vbroadcastss	.LCPI1_1(%rip), %ymm2
	vaddps	%ymm2, %ymm9, %ymm2
	vxorps	%xmm6, %xmm6, %xmm6
	vmaxps	%ymm6, %ymm2, %ymm9
	vcmpgt_oqps	%ymm6, %ymm9, %ymm7
	vaddps	896(%rsp), %ymm8, %ymm2
	vmovaps	2336(%rsp), %ymm6
	vblendvps	%ymm7, %ymm2, %ymm6, %ymm12
	movq	%rax, 800(%rsp)
.Ltmp1816:
	shlq	$5, %rax
	vmovups	4640(%rsp,%rax), %ymm2
.Ltmp1817:
	vmaxps	%ymm2, %ymm2, %ymm6
	vmovaps	1184(%rsp), %ymm11
.Ltmp1818:
	vblendvps	%ymm11, %ymm6, %ymm2, %ymm2
.Ltmp1819:
	vdivps	%ymm2, %ymm12, %ymm6
	vmovaps	%ymm12, 896(%rsp)
	vcmpgt_oqps	%ymm12, %ymm2, %ymm2
	vbroadcastss	.LCPI1_2(%rip), %ymm11
	vblendvps	%ymm2, %ymm6, %ymm11, %ymm2
.Ltmp1820:
	.loc	1 1394 25 is_stmt 1
	movq	1680(%rbx), %rax
	movq	320(%rsp), %rdx
.Ltmp1821:
	.loc	11 551 14
	vmovups	%ymm2, (%rax,%rdx,4)
.Ltmp1822:
	.loc	1 1261 17
	movq	1840(%rbx), %rdx
.Ltmp1823:
	.loc	12 37 12
	testq	%rdx, %rdx
	je	.LBB1_179
.Ltmp1824:
	.loc	12 0 12 is_stmt 0
	movq	24(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 224(%rsp)
	movq	96(%rsp), %rax
	leaq	1(%rax), %r9
	cmpq	%rcx, %r9
	movq	%rcx, %rax
	movl	$0, %esi
	cmovbq	%rsi, %rax
	movq	1824(%rdi), %r14
	subq	%rax, %r9
	movq	1680(%rdi), %rax
	movq	1688(%rdi), %rsi
	movq	1768(%rdi), %r8
	movq	%r8, 544(%rsp)
	movq	1760(%rdi), %r13
	movq	1736(%rdi), %r8
	movq	%r8, 32(%rsp)
	movq	1728(%rdi), %r15
	imulq	%rdx, %r9
	movq	%r9, 416(%rsp)
	movq	%rdx, %rbx
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB1_164
	.p2align	4
.LBB1_162:
	xorl	%r9d, %r9d
.LBB1_163:
	decq	%rbx
	addq	$4, %rdi
.Ltmp1825:
	movl	%r9d, (%r13,%r11,4)
.Ltmp1826:
	incq	%r11
.Ltmp1827:
	.loc	12 37 12 is_stmt 1
	testq	%rbx, %rbx
	je	.LBB1_179
.LBB1_164:
.Ltmp1828:
	.loc	7 1714 9
	cmpq	$32, %rdi
.Ltmp1829:
	.loc	6 180 28
	je	.LBB1_179
.Ltmp1830:
	.loc	1 1265 21
	cmpq	224(%rsp), %r11
	je	.LBB1_389
	leaq	(%r11,%r11,2), %r9
	movl	4(%r14,%r9,4), %r8d
.Ltmp1831:
	.loc	1 1267 23
	addq	96(%rsp), %r8
.Ltmp1832:
	.loc	1 1268 12
	cmpq	%rcx, %r8
	movl	$0, %r10d
	cmovaeq	%rcx, %r10
	subq	%r10, %r8
.Ltmp1833:
	.loc	1 1275 42
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1275 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_391
.Ltmp1834:
	.loc	1 1276 24 is_stmt 1
	cmpq	544(%rsp), %r11
	je	.LBB1_392
.Ltmp1835:
	.loc	1 0 0 is_stmt 0
	movl	(%r14,%r9,4), %r10d
.Ltmp1836:
	vmovss	(%rax,%r12,4), %xmm6
.Ltmp1837:
	.loc	1 1276 24
	movl	(%r13,%r11,4), %r9d
	testq	%r9, %r9
.Ltmp1838:
	.loc	1 1277 26 is_stmt 1
	je	.LBB1_172
	.loc	1 1280 24
	cmpq	32(%rsp), %r11
	jae	.LBB1_393
	vmovss	(%r15,%r11,4), %xmm11
.Ltmp1839:
	.loc	1 905 8
	vucomiss	%xmm11, %xmm6
	jbe	.LBB1_172
.Ltmp1840:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm11, %xmm6
.LBB1_172:
.Ltmp1841:
	.loc	1 1282 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB1_390
	vmovss	%xmm6, (%r15,%r11,4)
	.loc	1 1283 24
	incq	%r9
	cmpq	%r10, %r9
.Ltmp1842:
	.loc	1 1284 23
	jne	.LBB1_177
	.loc	1 1284 9 is_stmt 0
	vmovss	%xmm6, 576(%rsp,%rdi)
	.loc	1 1290 30 is_stmt 1
	vmovss	(%rax,%r12,4), %xmm6
	.loc	1 0 30 is_stmt 0
.Ltmp1843:
	.p2align	4
.LBB1_175:
.Ltmp1844:
	.loc	1 1293 65 is_stmt 1
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1293 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_371
.Ltmp1845:
	.loc	1 905 8 is_stmt 1
	vminss	(%rax,%r12,4), %xmm6, %xmm6
.Ltmp1846:
	.loc	1 1294 17
	vmovss	%xmm6, (%rax,%r12,4)
	.loc	1 1295 20
	testq	%r8, %r8
	cmoveq	%rcx, %r8
	.loc	1 1298 17
	decq	%r8
.Ltmp1847:
	.loc	10 1916 50
	decq	%r10
.Ltmp1848:
	.loc	3 900 12
	jne	.LBB1_175
	jmp	.LBB1_162
.Ltmp1849:
	.loc	3 0 12 is_stmt 0
.Ltmp1850:
	.p2align	4
.LBB1_177:
	movq	416(%rsp), %r8
	.loc	1 1287 44 is_stmt 1
	leaq	(%r11,%r8), %r12
	.loc	1 1287 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_394
	vmovss	(%rax,%r12,4), %xmm2
.Ltmp1851:
	.loc	1 905 8 is_stmt 1
	vminss	%xmm6, %xmm2, %xmm2
.Ltmp1852:
	.loc	1 1284 9
	vmovss	%xmm2, 576(%rsp,%rdi)
	jmp	.LBB1_163
.Ltmp1853:
	.loc	1 0 9 is_stmt 0
.Ltmp1854:
	.p2align	4
.LBB1_179:
	.loc	11 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm6
	movq	24(%rsp), %rbx
.Ltmp1855:
	.loc	1 1414 26
	movq	1704(%rbx), %rsi
	vmovaps	%ymm6, %ymm12
	movq	352(%rsp), %rdi
.Ltmp1856:
	.loc	12 37 12
	testq	%rdi, %rdi
	movq	216(%rsp), %r13
	movq	864(%rsp), %r11
	movq	1216(%rsp), %r14
	je	.LBB1_205
.Ltmp1857:
	.loc	12 0 12 is_stmt 0
	movq	1832(%rbx), %r9
.Ltmp1858:
	.loc	1 1405 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB1_405
	.loc	1 0 42 is_stmt 0
	movq	1824(%rbx), %r10
	.loc	1 1405 42
	movl	8(%r10), %r8d
	.loc	1 1405 28
	addq	96(%rsp), %r8
.Ltmp1859:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	.loc	1 1409 40
	imulq	%rdi, %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_403
	.loc	1 0 25
	movq	1696(%rbx), %rdx
	.loc	1 1409 25
	vmovss	(%rdx,%r8,4), %xmm2
	.loc	1 1409 13
	vmovss	%xmm2, 576(%rsp)
.Ltmp1860:
	.loc	12 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB1_204
.Ltmp1861:
	.loc	1 1405 42
	cmpq	$1, %r9
	je	.LBB1_406
	movl	20(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1862:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	352(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_403
	vmovss	4(%rdx,%rax,4), %xmm2
	.loc	1 1409 13
	vmovss	%xmm2, 580(%rsp)
.Ltmp1863:
	.loc	12 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB1_204
.Ltmp1864:
	.loc	1 1405 42
	cmpq	$2, %r9
	je	.LBB1_409
	movl	32(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1865:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	352(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_403
	vmovss	8(%rdx,%rax,4), %xmm2
	.loc	1 1409 13
	vmovss	%xmm2, 584(%rsp)
.Ltmp1866:
	.loc	12 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB1_204
.Ltmp1867:
	.loc	1 1405 42
	cmpq	$3, %r9
	je	.LBB1_411
	movl	44(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1868:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	352(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_403
	vmovss	12(%rdx,%rax,4), %xmm2
	.loc	1 1409 13
	vmovss	%xmm2, 588(%rsp)
.Ltmp1869:
	.loc	12 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB1_204
.Ltmp1870:
	.loc	1 1405 42
	cmpq	$4, %r9
	je	.LBB1_413
	movl	56(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1871:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	352(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_403
	vmovss	16(%rdx,%rax,4), %xmm2
	.loc	1 1409 13
	vmovss	%xmm2, 592(%rsp)
.Ltmp1872:
	.loc	12 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB1_204
.Ltmp1873:
	.loc	1 1405 42
	cmpq	$5, %r9
	je	.LBB1_423
	movl	68(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1874:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	352(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_403
	vmovss	20(%rdx,%rax,4), %xmm2
	.loc	1 1409 13
	vmovss	%xmm2, 596(%rsp)
.Ltmp1875:
	.loc	12 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB1_204
.Ltmp1876:
	.loc	1 1405 42
	cmpq	$6, %r9
	je	.LBB1_426
	movl	80(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1877:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	352(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_403
	vmovss	24(%rdx,%rax,4), %xmm2
	.loc	1 1409 13
	vmovss	%xmm2, 600(%rsp)
.Ltmp1878:
	.loc	12 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB1_204
.Ltmp1879:
	.loc	1 1405 42
	cmpq	$7, %r9
	je	.LBB1_428
	movl	92(%r10), %eax
	.loc	1 1405 28 is_stmt 0
	addq	96(%rsp), %rax
.Ltmp1880:
	.loc	1 1406 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	352(%rsp), %rdi
	.loc	1 1409 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1409 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_403
	vmovss	28(%rdx,%rax,4), %xmm2
	.loc	1 1409 13
	vmovss	%xmm2, 604(%rsp)
.Ltmp1881:
	.loc	1 0 13
.Ltmp1882:
	.p2align	4
.LBB1_204:
	.loc	11 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm12
.Ltmp1883:
.LBB1_205:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	320(%rsp), %rdx
	movq	88(%rsp), %r15
	movq	2432(%rsp), %r9
	jb	.LBB1_332
.Ltmp1884:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_269
.Ltmp1885:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI1_1(%rip), %ymm2
	vaddps	%ymm2, %ymm0, %ymm2
	vxorps	%xmm11, %xmm11, %xmm11
	vmaxps	%ymm11, %ymm2, %ymm0
	vcmpgt_oqps	%ymm11, %ymm0, %ymm11
	vmovaps	%ymm13, %ymm14
	vaddps	1152(%rsp), %ymm13, %ymm2
	vmovaps	2304(%rsp), %ymm13
	vblendvps	%ymm11, %ymm2, %ymm13, %ymm13
.Ltmp1886:
	vbroadcastss	.LCPI1_3(%rip), %ymm2
	vmulps	%ymm2, %ymm6, %ymm2
	vroundps	$9, %ymm2, %ymm2
	vbroadcastss	.LCPI1_4(%rip), %ymm6
	vmulps	%ymm6, %ymm2, %ymm2
.Ltmp1887:
	vaddps	1120(%rsp), %ymm2, %ymm6
	vsubps	%ymm12, %ymm6, %ymm6
	.loc	1 1414 26 is_stmt 1
	movq	1696(%rbx), %rax
	movq	320(%rsp), %rdx
.Ltmp1888:
	.loc	11 551 14
	vmovups	%ymm2, (%rax,%rdx,4)
	vmovaps	%ymm6, 1120(%rsp)
.Ltmp1889:
	.loc	29 360 14
	vdivps	2272(%rsp), %ymm6, %ymm2
.Ltmp1890:
	.loc	1 1418 43
	vmovaps	3904(%rsp), %ymm6
.Ltmp1891:
	.loc	29 347 14
	vbroadcastss	.LCPI1_2(%rip), %ymm12
	vsubps	%ymm2, %ymm12, %ymm2
.Ltmp1892:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm6, %ymm2, %ymm12
	vmovaps	%ymm13, 1152(%rsp)
.Ltmp1893:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm12, %ymm13, %ymm12
.Ltmp1894:
	.loc	29 48 14
	vaddps	%ymm6, %ymm12, %ymm6
.Ltmp1895:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm2, %ymm2
.Ltmp1896:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm6
	vandps	%ymm6, %ymm2, %ymm6
.Ltmp1897:
	.loc	29 871 14
	vbroadcastss	.LCPI1_5(%rip), %ymm12
	vcmplt_oqps	%ymm12, %ymm6, %ymm6
.Ltmp1898:
	.loc	29 82 19
	vandnps	%ymm2, %ymm6, %ymm6
.Ltmp1899:
	.loc	1 1419 5
	vmovaps	%ymm6, 3904(%rsp)
.Ltmp1900:
	.loc	1 1422 28
	movq	1672(%rbx), %rsi
	.loc	1 1422 44 is_stmt 0
	imulq	%r11, %rdi
.Ltmp1901:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB1_333
.Ltmp1902:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_269
.Ltmp1903:
	.loc	5 0 16 is_stmt 0
	vxorps	%xmm2, %xmm2, %xmm2
	vblendvps	%ymm7, %ymm8, %ymm2, %ymm8
	vblendvps	%ymm11, %ymm14, %ymm2, %ymm13
	movq	800(%rsp), %r8
.Ltmp1904:
	incq	%r8
	movq	832(%rsp), %rax
.Ltmp1905:
	leaq	(,%rax,4), %rax
	addq	%r13, %rax
.Ltmp1906:
	vbroadcastss	.LCPI1_2(%rip), %ymm2
	vsubps	%ymm6, %ymm2, %ymm2
.Ltmp1907:
	.loc	1 1422 28 is_stmt 1
	movq	1664(%rbx), %rdx
.Ltmp1908:
	.loc	11 551 14
	vmovups	(%rdx,%rdi,4), %ymm6
.Ltmp1909:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rax), %ymm7
	vmovups	%ymm7, (%rdx,%rdi,4)
.Ltmp1910:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm6, %ymm2, %ymm2
.Ltmp1911:
	.loc	29 585 19
	vblendvps	%ymm15, %ymm6, %ymm2, %ymm2
.Ltmp1912:
	.loc	11 551 14
	vmovups	%ymm2, (%rax)
.Ltmp1913:
	.loc	1 3302 13
	incq	%r11
	.loc	1 3303 16
	cmpq	672(%rsp), %r11
	movl	$0, %esi
	cmoveq	%rsi, %r11
	movq	96(%rsp), %rax
	.loc	1 3306 13
	incq	%rax
	.loc	1 3307 16
	cmpq	%rcx, %rax
	movl	$0, %edx
	movq	%rdx, 608(%rsp)
	cmoveq	%rsi, %rax
	movq	%rax, 96(%rsp)
	movq	%r8, %rax
.Ltmp1914:
	.loc	10 1916 50
	cmpq	1248(%rsp), %r8
.Ltmp1915:
	.loc	3 900 12
	jne	.LBB1_156
.Ltmp1916:
	.loc	3 0 12 is_stmt 0
	vmovaps	1120(%rsp), %ymm2
.Ltmp1917:
	.loc	1 1413 5 is_stmt 1
	vmovaps	%ymm2, 3936(%rsp)
.Ltmp1918:
	.loc	1 853 0
	vmovaps	%ymm9, 3744(%rsp)
	vmovaps	896(%rsp), %ymm2
.Ltmp1919:
	.loc	1 855 0
	vmovaps	%ymm2, 3648(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 3712(%rsp)
.Ltmp1920:
	.loc	1 853 0 is_stmt 1
	vmovaps	%ymm0, 3872(%rsp)
	vmovaps	1152(%rsp), %ymm2
.Ltmp1921:
	.loc	1 855 0
	vmovaps	%ymm2, 3776(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 3840(%rsp)
	movq	8(%rsp), %rax
	jmp	.LBB1_147
.Ltmp1922:
.LBB1_211:
	.loc	1 2314 37 is_stmt 1
	movq	1792(%rbx), %rsi
	movq	1800(%rbx), %rdx
.Ltmp1923:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp1924:
	.p2align	4
.LBB1_212:
.Ltmp1925:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp1926:
	.loc	6 180 28
	je	.LBB1_255
.Ltmp1927:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp1928:
	.loc	6 315 25
	jne	.LBB1_39
	addq	$-16, %rdx
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB1_212
	jmp	.LBB1_39
.Ltmp1929:
.LBB1_215:
	.loc	6 0 25
	movq	%r15, %r14
	movq	24(%rsp), %rbx
.Ltmp1930:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB1_272
.Ltmp1931:
.LBB1_216:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp1932:
	.loc	1 3194 12
	je	.LBB1_273
.LBB1_217:
	.loc	1 0 12 is_stmt 0
	leaq	1376(%rsp), %rdi
	movq	16(%rsp), %r15
.Ltmp1933:
	.loc	1 3334 24 is_stmt 1
	movq	%r15, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp1934:
	.loc	1 3339 19
	movzbl	1536(%rbx), %r14d
	movq	%rbx, %rax
.Ltmp1935:
	.loc	1 3340 21
	movzbl	1537(%rbx), %ecx
	movb	%cl, 32(%rsp)
.Ltmp1936:
	.loc	1 3341 16
	movq	1624(%rbx), %r12
.Ltmp1937:
	.loc	1 3342 16
	movq	1632(%rbx), %rbx
.Ltmp1938:
	.loc	1 3343 27
	movl	1640(%rax), %ecx
	movq	%rcx, 320(%rsp)
.Ltmp1939:
	.loc	1 3344 27
	movl	1644(%rax), %eax
	movq	%rax, 416(%rsp)
	leaq	4640(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	3264(%rsp), %rdi
.Ltmp1940:
	.loc	1 3348 32
	movq	%r15, %rsi
	movq	%r12, %rdx
	movq	%rbx, 160(%rsp)
	movq	%rbx, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	8(%rsp), %rax
.Ltmp1941:
	.loc	4 3758 16
	leaq	31(%rax), %rdi
	shrq	$5, %rdi
.Ltmp1942:
	.loc	8 446 20
	je	.LBB1_262
.Ltmp1943:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp1944:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, %ymm6
	vmovaps	%ymm1, 512(%rsp)
	testb	%r14b, %r14b
	movq	88(%rsp), %r15
	jne	.LBB1_220
.Ltmp1945:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 512(%rsp)
.LBB1_220:
	cmpb	$0, 32(%rsp)
	jne	.LBB1_222
	vmovaps	%ymm0, %ymm6
.LBB1_222:
	movq	3352(%rsp), %rax
	movq	%rax, 2304(%rsp)
	movq	3360(%rsp), %rax
	movq	%rax, 2272(%rsp)
	movq	3296(%rsp), %r13
	movq	3304(%rsp), %rdx
	movq	3344(%rsp), %rax
	movq	%rax, 1152(%rsp)
	movq	3312(%rsp), %rax
	movq	%rax, 1344(%rsp)
	movq	3320(%rsp), %r11
	movq	3336(%rsp), %rax
	movq	%rax, 864(%rsp)
	movq	3328(%rsp), %rax
	movq	%rax, 1312(%rsp)
	vmovaps	3264(%rsp), %ymm14
	movl	3368(%rsp), %r10d
.Ltmp1946:
	.loc	1 1761 23 is_stmt 1
	vmovaps	1376(%rsp), %ymm4
	vmovaps	1408(%rsp), %ymm7
	vmovaps	1440(%rsp), %ymm11
	vmovaps	1472(%rsp), %ymm12
	vmovaps	1504(%rsp), %ymm15
	vmovaps	1536(%rsp), %ymm3
	vmovaps	1568(%rsp), %ymm0
	vmovaps	%ymm0, 224(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	1632(%rsp), %ymm9
	vmovaps	1664(%rsp), %ymm8
	vmovaps	1696(%rsp), %ymm10
	vmovaps	1728(%rsp), %ymm1
	movl	$32, %esi
	movq	216(%rsp), %r14
	movq	%r15, %r8
	movl	$0, %eax
	movq	%rax, 672(%rsp)
	movq	8(%rsp), %rcx
	movq	%rcx, %rax
	xorl	%r9d, %r9d
	movq	24(%rsp), %rbx
	vmovaps	%ymm6, 2240(%rsp)
	movq	%r11, 1120(%rsp)
.Ltmp1947:
.LBB1_223:
	.loc	1 0 23 is_stmt 0
	movq	%rdi, 928(%rsp)
	vmovaps	%ymm14, 128(%rsp)
.Ltmp1948:
	.loc	4 2584 13 is_stmt 1
	cmpq	$32, %rax
	movl	$32, %edi
	movq	%rax, 3232(%rsp)
	cmovbq	%rax, %rdi
	cmpq	$1, %rdi
	movq	%rdi, 608(%rsp)
	movq	%rdi, %rax
	adcq	$0, %rax
.Ltmp1949:
	.loc	10 1916 50
	movq	%rcx, %rdi
	subq	%r9, %rdi
.Ltmp1950:
	.loc	10 1078 5
	cmpq	$32, %rdi
	cmovaeq	%rsi, %rdi
	movq	%rdi, 2336(%rsp)
	movq	%r9, 1184(%rsp)
.Ltmp1951:
	.loc	10 1916 50
	cmpq	%r9, %rcx
	movq	%r8, 2400(%rsp)
.Ltmp1952:
	.loc	3 900 12
	jne	.LBB1_225
	.loc	3 0 12 is_stmt 0
	vmovaps	%ymm1, 768(%rsp)
	.loc	3 900 12
	jmp	.LBB1_229
.Ltmp1953:
.LBB1_225:
	.loc	3 0 12
	shll	$3, %eax
	vmovaps	(%rbx), %ymm1
	vmovaps	%ymm1, 352(%rsp)
	vmovaps	32(%rbx), %ymm1
	vmovaps	%ymm1, 704(%rsp)
	vmovaps	64(%rbx), %ymm1
	vmovaps	%ymm1, 896(%rsp)
	vmovaps	96(%rbx), %ymm1
	vmovaps	%ymm1, 832(%rsp)
	vmovaps	128(%rbx), %ymm1
	vmovaps	%ymm1, 800(%rsp)
	vmovaps	160(%rbx), %ymm1
	vmovaps	%ymm1, 1088(%rsp)
	vmovaps	192(%rbx), %ymm1
	vmovaps	%ymm1, 480(%rsp)
	vmovaps	224(%rbx), %ymm1
	vmovaps	%ymm1, 384(%rsp)
	vmovaps	256(%rbx), %ymm1
	vmovaps	%ymm1, 288(%rsp)
	vmovaps	288(%rbx), %ymm1
	vmovaps	%ymm1, 256(%rsp)
	vmovaps	320(%rbx), %ymm1
	vmovaps	%ymm1, 448(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm7, %ymm2
	vmovaps	%ymm10, %ymm1
	vmovaps	%ymm8, %ymm14
	vmovaps	%ymm9, %ymm13
	vmovaps	544(%rsp), %ymm5
	vmovaps	352(%rbx), %ymm0
	vmovaps	%ymm0, 1056(%rsp)
	vmovaps	384(%rbx), %ymm0
	vmovaps	%ymm0, 1024(%rsp)
	vmovaps	416(%rbx), %ymm0
	vmovaps	%ymm0, 992(%rsp)
	vmovaps	448(%rbx), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	480(%rbx), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	512(%rbx), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	544(%rbx), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	576(%rbx), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	608(%rbx), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
	vmovaps	640(%rbx), %ymm0
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	672(%rbx), %ymm0
	vmovaps	%ymm0, 3104(%rsp)
	vmovaps	704(%rbx), %ymm0
	vmovaps	%ymm0, 3072(%rsp)
	vmovaps	736(%rbx), %ymm0
	vmovaps	%ymm0, 3040(%rsp)
	vmovaps	768(%rbx), %ymm0
	vmovaps	%ymm0, 3008(%rsp)
	vmovaps	800(%rbx), %ymm0
	vmovaps	%ymm0, 2976(%rsp)
	vmovaps	832(%rbx), %ymm0
	vmovaps	%ymm0, 2944(%rsp)
	vmovaps	864(%rbx), %ymm0
	vmovaps	%ymm0, 2912(%rsp)
	vmovaps	896(%rbx), %ymm0
	vmovaps	%ymm0, 2880(%rsp)
	vmovaps	928(%rbx), %ymm0
	vmovaps	%ymm0, 2848(%rsp)
	vmovaps	960(%rbx), %ymm0
	vmovaps	%ymm0, 2816(%rsp)
	vmovaps	992(%rbx), %ymm0
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	1024(%rbx), %ymm0
	vmovaps	%ymm0, 2752(%rsp)
	vmovaps	1056(%rbx), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	1088(%rbx), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	1120(%rbx), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	1152(%rbx), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
	vmovaps	1184(%rbx), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	1216(%rbx), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	1248(%rbx), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	1280(%rbx), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1312(%rbx), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1344(%rbx), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1376(%rbx), %ymm0
	vmovaps	%ymm0, 640(%rsp)
	vmovaps	1408(%rbx), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	1440(%rbx), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	1472(%rbx), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	vmovaps	1504(%rbx), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	.p2align	4
.LBB1_226:
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	%ymm3, 224(%rsp)
	movq	672(%rsp), %rdi
.Ltmp1954:
	.loc	5 568 12 is_stmt 1
	addq	%rcx, %rdi
	cmpq	%r15, %rdi
	ja	.LBB1_373
.Ltmp1955:
	.loc	5 438 16
	cmpq	$7, %r8
	jbe	.LBB1_387
.Ltmp1956:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm14, %ymm10
	vmovaps	%ymm15, %ymm3
	vmovaps	%ymm12, %ymm15
	vmovaps	%ymm11, %ymm12
	vmovaps	%ymm2, %ymm7
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	%ymm4, 32(%rsp)
.Ltmp1957:
	.loc	11 551 14 is_stmt 1
	vmovups	(%r14,%rcx,4), %ymm9
.Ltmp1958:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm11
.Ltmp1959:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm9, %ymm1
	vxorps	%xmm4, %xmm4, %xmm4
.Ltmp1960:
	.loc	29 48 14
	vaddps	%ymm4, %ymm1, %ymm1
.Ltmp1961:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm9, %ymm5
.Ltmp1962:
	.loc	29 48 14
	vaddps	%ymm4, %ymm5, %ymm5
	vmovaps	%ymm13, %ymm8
.Ltmp1963:
	.loc	29 283 14
	vmulps	896(%rsp), %ymm9, %ymm13
.Ltmp1964:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm13
.Ltmp1965:
	.loc	29 283 14
	vmulps	832(%rsp), %ymm9, %ymm14
.Ltmp1966:
	.loc	29 48 14
	vaddps	%ymm4, %ymm14, %ymm14
	vmovaps	%ymm3, %ymm4
	vmovaps	%ymm15, %ymm3
	vmovaps	%ymm12, %ymm15
	vmovaps	%ymm7, %ymm12
	vmovaps	32(%rsp), %ymm0
.Ltmp1967:
	.loc	29 283 14
	vmulps	800(%rsp), %ymm0, %ymm7
.Ltmp1968:
	.loc	29 48 14
	vaddps	%ymm1, %ymm7, %ymm1
	vmovaps	32(%rsp), %ymm0
.Ltmp1969:
	.loc	29 283 14
	vmulps	1088(%rsp), %ymm0, %ymm7
.Ltmp1970:
	.loc	29 48 14
	vaddps	%ymm5, %ymm7, %ymm5
	vmovaps	32(%rsp), %ymm0
.Ltmp1971:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm0, %ymm7
.Ltmp1972:
	.loc	29 48 14
	vaddps	%ymm7, %ymm13, %ymm7
	vmovaps	32(%rsp), %ymm0
.Ltmp1973:
	.loc	29 283 14
	vmulps	384(%rsp), %ymm0, %ymm13
.Ltmp1974:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp1975:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm12, %ymm14
.Ltmp1976:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp1977:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm12, %ymm14
.Ltmp1978:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp1979:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm12, %ymm14
.Ltmp1980:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1981:
	.loc	29 283 14
	vmulps	1056(%rsp), %ymm12, %ymm14
.Ltmp1982:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp1983:
	.loc	29 283 14
	vmulps	1024(%rsp), %ymm15, %ymm14
.Ltmp1984:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp1985:
	.loc	29 283 14
	vmulps	992(%rsp), %ymm15, %ymm14
.Ltmp1986:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp1987:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm15, %ymm14
.Ltmp1988:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1989:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm15, %ymm14
.Ltmp1990:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp1991:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm3, %ymm14
.Ltmp1992:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp1993:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm3, %ymm14
.Ltmp1994:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp1995:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm3, %ymm14
.Ltmp1996:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp1997:
	.loc	29 283 14
	vmulps	3168(%rsp), %ymm3, %ymm14
.Ltmp1998:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp1999:
	.loc	29 283 14
	vmulps	3136(%rsp), %ymm4, %ymm14
.Ltmp2000:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp2001:
	.loc	29 283 14
	vmulps	3104(%rsp), %ymm4, %ymm14
.Ltmp2002:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2003:
	.loc	29 283 14
	vmulps	3072(%rsp), %ymm4, %ymm14
.Ltmp2004:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp2005:
	.loc	29 283 14
	vmulps	3040(%rsp), %ymm4, %ymm14
.Ltmp2006:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	224(%rsp), %ymm0
.Ltmp2007:
	.loc	29 283 14
	vmulps	3008(%rsp), %ymm0, %ymm14
.Ltmp2008:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp2009:
	.loc	29 283 14
	vmulps	2976(%rsp), %ymm0, %ymm14
.Ltmp2010:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2011:
	.loc	29 283 14
	vmulps	2944(%rsp), %ymm0, %ymm14
.Ltmp2012:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp2013:
	.loc	29 283 14
	vmulps	2912(%rsp), %ymm0, %ymm14
.Ltmp2014:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	544(%rsp), %ymm6
.Ltmp2015:
	.loc	29 283 14
	vmulps	2880(%rsp), %ymm6, %ymm14
.Ltmp2016:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp2017:
	.loc	29 283 14
	vmulps	2848(%rsp), %ymm6, %ymm14
.Ltmp2018:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2019:
	.loc	29 283 14
	vmulps	2816(%rsp), %ymm6, %ymm14
.Ltmp2020:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp2021:
	.loc	29 283 14
	vmulps	2784(%rsp), %ymm6, %ymm14
.Ltmp2022:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	96(%rsp), %ymm14
.Ltmp2023:
	.loc	29 283 14
	vmulps	2752(%rsp), %ymm14, %ymm14
.Ltmp2024:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
	vmovaps	96(%rsp), %ymm14
.Ltmp2025:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm14, %ymm14
.Ltmp2026:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
	vmovaps	96(%rsp), %ymm14
.Ltmp2027:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm14, %ymm14
.Ltmp2028:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
	vmovaps	96(%rsp), %ymm14
.Ltmp2029:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm14, %ymm14
.Ltmp2030:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2031:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm8, %ymm14
.Ltmp2032:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp2033:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm8, %ymm14
.Ltmp2034:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2035:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm8, %ymm14
.Ltmp2036:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp2037:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm8, %ymm14
.Ltmp2038:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2039:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm10, %ymm14
.Ltmp2040:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp2041:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm10, %ymm14
.Ltmp2042:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2043:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm10, %ymm14
.Ltmp2044:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp2045:
	.loc	29 283 14
	vmulps	640(%rsp), %ymm10, %ymm14
.Ltmp2046:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2047:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm2, %ymm14
.Ltmp2048:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp2049:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm2, %ymm14
.Ltmp2050:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2051:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm2, %ymm14
.Ltmp2052:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
	vmovaps	%ymm2, 768(%rsp)
.Ltmp2053:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm2, %ymm14
.Ltmp2054:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2055:
	.loc	29 82 19
	vandps	%ymm0, %ymm11, %ymm14
.Ltmp2056:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm1, %ymm11, %ymm1
.Ltmp2057:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm1, %ymm14, %ymm1
.Ltmp2058:
	.loc	29 82 19
	vandps	%ymm5, %ymm11, %ymm5
.Ltmp2059:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm1, %ymm1
.Ltmp2060:
	.loc	29 82 19
	vandps	%ymm7, %ymm11, %ymm5
	vmovaps	%ymm12, %ymm0
	vmovaps	%ymm15, %ymm12
	vmovaps	%ymm3, %ymm15
	vmovaps	%ymm4, %ymm3
	vmovaps	%ymm9, %ymm4
	vmovaps	32(%rsp), %ymm7
	vmovaps	96(%rsp), %ymm9
.Ltmp2061:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm1, %ymm1
.Ltmp2062:
	.loc	29 82 19
	vandps	%ymm11, %ymm13, %ymm2
	vmovaps	%ymm0, %ymm11
.Ltmp2063:
	.loc	29 233 14
	vmaxps	%ymm2, %ymm1, %ymm1
.Ltmp2064:
	.loc	11 551 14
	vmovups	%ymm1, 4640(%rsp,%rcx,4)
.Ltmp2065:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %r8
	vmovaps	%ymm7, %ymm2
	vmovaps	%ymm10, %ymm1
	vmovaps	%ymm8, %ymm14
	vmovaps	%ymm9, %ymm13
	vmovaps	%ymm6, %ymm5
	cmpq	%rcx, %rax
.Ltmp2066:
	.loc	3 900 12
	jne	.LBB1_226
.Ltmp2067:
.LBB1_229:
	.loc	1 1767 5
	vmovaps	%ymm4, 1376(%rsp)
	vmovaps	%ymm7, 1408(%rsp)
	vmovaps	%ymm11, 1440(%rsp)
	vmovaps	%ymm12, 1472(%rsp)
	vmovaps	%ymm15, 1504(%rsp)
	vmovaps	%ymm3, 1536(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
	vmovaps	544(%rsp), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	%ymm9, 1632(%rsp)
	vmovaps	%ymm8, 1664(%rsp)
	vmovaps	%ymm10, 1696(%rsp)
	vmovaps	768(%rsp), %ymm1
	vmovaps	%ymm1, 1728(%rsp)
	movq	1184(%rsp), %r9
.Ltmp2068:
	.loc	10 1916 50
	cmpq	%r9, 8(%rsp)
	vmovaps	2240(%rsp), %ymm6
.Ltmp2069:
	.loc	1 3362 19
	jne	.LBB1_232
.Ltmp2070:
	.loc	1 0 19 is_stmt 0
	vmovaps	128(%rsp), %ymm14
	movq	928(%rsp), %rdi
.LBB1_231:
	addq	$32, %r9
	decq	%rdi
	movq	3232(%rsp), %rax
.Ltmp2071:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rax
	addq	$256, 672(%rsp)
	movq	2400(%rsp), %r8
	addq	$-256, %r8
	addq	$1024, %r14
	testq	%rdi, %rdi
	movq	8(%rsp), %rcx
	movl	$32, %esi
	jne	.LBB1_223
	jmp	.LBB1_370
.Ltmp2072:
.LBB1_232:
	.loc	8 0 20 is_stmt 0
	vmovaps	%ymm10, 2112(%rsp)
	vmovaps	%ymm8, 2144(%rsp)
	vmovaps	%ymm9, 96(%rsp)
	vmovaps	%ymm7, 32(%rsp)
	vmovaps	%ymm4, 2176(%rsp)
	movq	%r14, 2208(%rsp)
	vmovaps	%ymm3, 960(%rsp)
	vmovaps	%ymm15, 992(%rsp)
	vmovaps	%ymm12, 1024(%rsp)
	vmovaps	%ymm11, 1056(%rsp)
	vmovaps	1760(%rsp), %ymm8
	vmovaps	1792(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	1824(%rsp), %ymm0
	vmovaps	1856(%rsp), %ymm11
	vmovaps	1920(%rsp), %ymm1
	vmovaps	%ymm1, 832(%rsp)
	vmovaps	2080(%rsp), %ymm1
	vmovaps	%ymm1, 800(%rsp)
	vmovaps	1984(%rsp), %ymm15
	vmovaps	1888(%rsp), %ymm12
	vmovaps	1952(%rsp), %ymm7
	vmovaps	2048(%rsp), %ymm5
	vmovaps	%ymm5, 448(%rsp)
	vmovaps	%ymm7, 384(%rsp)
	vmovaps	%ymm12, 288(%rsp)
	vmovaps	%ymm15, 256(%rsp)
	xorl	%ecx, %ecx
	movq	320(%rsp), %r9
	movq	416(%rsp), %r15
	xorl	%edi, %edi
	vxorps	%xmm13, %xmm13, %xmm13
	vmovaps	128(%rsp), %ymm14
	movq	2336(%rsp), %r14
	jmp	.LBB1_235
	.p2align	4
.LBB1_233:
	vmovaps	384(%rsp), %ymm9
	vmovaps	288(%rsp), %ymm2
	vmovaps	256(%rsp), %ymm4
	vmovaps	448(%rsp), %ymm3
.LBB1_234:
	movq	480(%rsp), %rcx
.Ltmp2073:
	addq	%r14, %rcx
	movq	416(%rsp), %r15
.Ltmp2074:
	.loc	1 3413 39 is_stmt 1
	addq	%r14, %r15
.Ltmp2075:
	.loc	1 1150 8
	cmpq	%r12, %r15
	movq	%r12, %rax
	movl	$0, %edi
	cmovbq	%rdi, %rax
	subq	%rax, %r15
	movq	320(%rsp), %r9
.Ltmp2076:
	.loc	1 3414 39
	addq	%r14, %r9
	movq	160(%rsp), %rax
.Ltmp2077:
	.loc	1 1150 8
	cmpq	%rax, %r9
	cmovbq	%rdi, %rax
	subq	%rax, %r9
	vmovaps	%ymm9, 384(%rsp)
	vmovaps	%ymm2, 288(%rsp)
	vmovaps	%ymm4, 256(%rsp)
	vmovaps	%ymm3, 448(%rsp)
	movq	2336(%rsp), %r14
.Ltmp2078:
	.loc	1 3362 19
	cmpq	%r14, %rcx
	movq	24(%rsp), %rbx
	jae	.LBB1_253
.LBB1_235:
	.loc	1 0 19 is_stmt 0
	movq	%rcx, 480(%rsp)
	.loc	1 3370 21 is_stmt 1
	subq	%rcx, %r14
.Ltmp2079:
	.loc	1 1577 16
	movq	1624(%rbx), %rcx
.Ltmp2080:
	.loc	1 1578 16
	movq	1632(%rbx), %r8
.Ltmp2081:
	.loc	1 1579 25
	leaq	1(%r15), %rax
.Ltmp2082:
	.loc	1 1150 8
	cmpq	%rcx, %rax
	movq	%rcx, %rsi
	cmovbq	%rdi, %rsi
	negq	%rsi
	movq	2304(%rsp), %rax
.Ltmp2083:
	.loc	1 1580 28
	leaq	(%r15,%rax), %rbx
.Ltmp2084:
	.loc	1 1150 8
	cmpq	%rcx, %rbx
	movq	%rcx, %rax
	cmovbq	%rdi, %rax
	subq	%rax, %rbx
	movq	2272(%rsp), %rax
.Ltmp2085:
	.loc	1 1582 33
	addq	%r15, %rax
.Ltmp2086:
	.loc	1 1150 8
	cmpq	%rcx, %rax
	movl	$0, %r11d
	movq	%rcx, %rdi
	cmovbq	%r11, %rdi
	subq	%rdi, %rax
.Ltmp2087:
	.loc	1 1585 14
	movq	%rcx, %r11
	subq	%r15, %r11
.Ltmp2088:
	.loc	10 1078 5
	cmpq	%r14, %r11
	cmovbq	%r11, %r14
	movq	%r9, 320(%rsp)
.Ltmp2089:
	.loc	1 1586 14
	subq	%r9, %r8
.Ltmp2090:
	.loc	10 1078 5
	cmpq	%r14, %r8
	cmovbq	%r8, %r14
	movq	%r15, 416(%rsp)
.Ltmp2091:
	.loc	1 1150 8
	leaq	(%r15,%rsi), %rdi
	incq	%rdi
.Ltmp2092:
	.loc	1 1587 14
	movq	%rcx, %r9
	movq	%rdi, 1088(%rsp)
	subq	%rdi, %r9
.Ltmp2093:
	.loc	10 1078 5
	cmpq	%r14, %r9
	cmovbq	%r9, %r14
.Ltmp2094:
	.loc	1 1588 14
	movq	%rcx, %r15
	subq	%rbx, %r15
.Ltmp2095:
	.loc	10 1078 5
	cmpq	%r14, %r15
	cmovbq	%r15, %r14
.Ltmp2096:
	.loc	1 1590 14
	subq	%rax, %rcx
.Ltmp2097:
	.loc	10 1078 5
	cmpq	%r14, %rcx
	cmovbq	%rcx, %r14
	movq	1184(%rsp), %rsi
	movq	480(%rsp), %rdi
.Ltmp2098:
	.loc	1 3376 28
	addq	%rsi, %rdi
.Ltmp2099:
	.loc	1 3378 55
	leaq	(%r14,%rdi), %rsi
.Ltmp2100:
	.loc	1 3376 28
	shlq	$3, %rdi
.Ltmp2101:
	.loc	1 3378 55
	shlq	$3, %rsi
.Ltmp2102:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB1_388
	cmpq	88(%rsp), %rsi
	ja	.LBB1_388
.Ltmp2103:
	.loc	14 304 12
	testq	%r14, %r14
	je	.LBB1_233
.Ltmp2104:
	.loc	14 0 12 is_stmt 0
	movq	480(%rsp), %rsi
	shlq	$5, %rsi
.Ltmp2105:
	.loc	14 304 12
	cmpq	%rcx, %r15
	cmovbq	%r15, %rcx
	cmpq	%r9, %rcx
	cmovaeq	%r9, %rcx
	movq	216(%rsp), %r9
.Ltmp2106:
	.loc	1 0 0
	leaq	(%r9,%rdi,4), %rdi
	movq	%rdi, 704(%rsp)
.Ltmp2107:
	.loc	14 304 12
	cmpq	%r11, %rcx
	cmovaeq	%r11, %rcx
	cmpq	%r8, %rcx
	cmovaeq	%r8, %rcx
	movq	608(%rsp), %r15
	subq	480(%rsp), %r15
	cmpq	%r15, %rcx
	cmovbq	%rcx, %r15
.Ltmp2108:
	.loc	1 0 0
	leaq	(%rsp,%rsi), %rcx
	addq	$4640, %rcx
	movq	%rcx, 352(%rsp)
.Ltmp2109:
	.loc	14 304 12
	movabsq	$2305843009213693951, %rcx
	andq	%rcx, %r15
	vmovaps	%ymm5, %ymm3
	vmovaps	%ymm7, %ymm9
	vmovaps	%ymm12, %ymm2
	vmovaps	%ymm15, %ymm4
	xorl	%ecx, %ecx
	vmovaps	%ymm14, %ymm1
.Ltmp2110:
	.loc	14 0 12
.Ltmp2111:
	.p2align	4
.LBB1_239:
	movq	416(%rsp), %rsi
.Ltmp2112:
	.loc	1 1504 26 is_stmt 1
	leaq	(%rcx,%rsi), %r8
.Ltmp2113:
	.loc	1 1139 16
	leaq	(,%r8,8), %rdi
.Ltmp2114:
	.loc	1 1140 33
	leaq	8(,%r8,8), %rsi
.Ltmp2115:
	.loc	4 1050 16
	leaq	7(,%r8,8), %r8
	cmpq	%rdx, %r8
	jae	.LBB1_365
.Ltmp2116:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rcx,8), %r8
	vbroadcastss	.LCPI1_1(%rip), %ymm15
.Ltmp2117:
	vaddps	%ymm15, %ymm11, %ymm5
	vmaxps	%ymm13, %ymm5, %ymm11
	vcmpgt_oqps	%ymm13, %ymm11, %ymm5
	vaddps	%ymm0, %ymm8, %ymm7
	vmovaps	896(%rsp), %ymm8
	vblendvps	%ymm5, %ymm7, %ymm8, %ymm8
	movq	352(%rsp), %r9
.Ltmp2118:
	.loc	11 551 14 is_stmt 1
	vmovups	(%r9,%r8,4), %ymm7
.Ltmp2119:
	.loc	29 233 14
	vmaxps	%ymm7, %ymm7, %ymm12
	vmovaps	512(%rsp), %ymm13
.Ltmp2120:
	.loc	29 585 19
	vblendvps	%ymm13, %ymm12, %ymm7, %ymm7
.Ltmp2121:
	.loc	29 360 14
	vdivps	%ymm7, %ymm8, %ymm12
.Ltmp2122:
	.loc	1 0 0 is_stmt 0
	leaq	(%rcx,%rbx), %r9
.Ltmp2123:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm8, %ymm7, %ymm7
.Ltmp2124:
	.loc	29 585 19
	vbroadcastss	.LCPI1_2(%rip), %ymm13
	vblendvps	%ymm7, %ymm12, %ymm13, %ymm7
.Ltmp2125:
	.loc	11 551 14
	vmovups	%ymm7, (%r13,%rdi,4)
.Ltmp2126:
	.loc	1 1132 16
	leaq	(,%r9,8), %r8
.Ltmp2127:
	.loc	4 1050 16
	leaq	7(,%r9,8), %r11
	cmpq	%rdx, %r11
	jae	.LBB1_254
.Ltmp2128:
	.loc	11 551 14
	vmovups	(%r13,%r8,4), %ymm7
	vmovaps	%ymm7, %ymm13
.Ltmp2129:
	.loc	1 1208 22
	testl	%r10d, %r10d
	je	.LBB1_243
.Ltmp2130:
	.loc	29 257 14
	vminps	%ymm7, %ymm1, %ymm13
.Ltmp2131:
.LBB1_243:
	.loc	1 0 0 is_stmt 0
	movl	%r10d, %r10d
.Ltmp2132:
	.loc	1 1214 20 is_stmt 1
	incq	%r10
	movq	1152(%rsp), %r8
	movq	%r8, %r11
	cmpq	%r8, %r10
	vxorps	%xmm12, %xmm12, %xmm12
.Ltmp2133:
	.loc	1 1215 22
	jne	.LBB1_247
	.loc	1 0 22 is_stmt 0
.Ltmp2134:
	.p2align	4
.LBB1_244:
.Ltmp2135:
	.loc	1 1132 16 is_stmt 1
	leaq	(,%r9,8), %r8
.Ltmp2136:
	.loc	4 1050 16
	leaq	7(,%r9,8), %r10
	cmpq	%rdx, %r10
	jae	.LBB1_254
.Ltmp2137:
	.loc	29 257 14
	vminps	(%r13,%r8,4), %ymm7, %ymm7
.Ltmp2138:
	.loc	11 551 14
	vmovups	%ymm7, (%r13,%r8,4)
.Ltmp2139:
	.loc	1 1226 16
	testq	%r9, %r9
	cmoveq	%r12, %r9
	.loc	1 1229 13
	decq	%r9
.Ltmp2140:
	.loc	10 1916 50
	decq	%r11
.Ltmp2141:
	.loc	3 900 12
	jne	.LBB1_244
.Ltmp2142:
	.loc	3 0 12 is_stmt 0
	xorl	%r10d, %r10d
	vmovaps	%ymm13, %ymm7
	jmp	.LBB1_249
	.p2align	4
.LBB1_247:
	movq	1088(%rsp), %r8
	addq	%rcx, %r8
.Ltmp2143:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r8,8), %r9
.Ltmp2144:
	.loc	1 1132 16
	shlq	$3, %r8
.Ltmp2145:
	.loc	4 1050 16
	cmpq	%rdx, %r9
	jae	.LBB1_254
.Ltmp2146:
	.loc	11 551 14
	vmovups	(%r13,%r8,4), %ymm1
.Ltmp2147:
	.loc	29 257 14
	vminps	%ymm13, %ymm1, %ymm7
.Ltmp2148:
.LBB1_249:
	.loc	1 0 0 is_stmt 0
	leaq	(%rcx,%rax), %r8
.Ltmp2149:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r8,8), %r9
.Ltmp2150:
	.loc	1 1132 16
	shlq	$3, %r8
	movq	1120(%rsp), %r11
.Ltmp2151:
	.loc	4 1050 16
	cmpq	%r11, %r9
	jae	.LBB1_367
.Ltmp2152:
	.loc	4 0 16 is_stmt 0
	cmpq	%r11, %rsi
.Ltmp2153:
	.loc	4 1050 16
	ja	.LBB1_368
.Ltmp2154:
	.loc	4 0 16
	vmovaps	%ymm13, %ymm10
	vaddps	%ymm4, %ymm15, %ymm1
	vmaxps	%ymm12, %ymm1, %ymm4
	vcmpgt_oqps	%ymm12, %ymm4, %ymm1
	vaddps	%ymm2, %ymm9, %ymm2
	vmovaps	832(%rsp), %ymm12
	vblendvps	%ymm1, %ymm2, %ymm12, %ymm2
.Ltmp2155:
	vbroadcastss	.LCPI1_3(%rip), %ymm12
	vmulps	%ymm7, %ymm12, %ymm7
	vroundps	$9, %ymm7, %ymm7
	vbroadcastss	.LCPI1_4(%rip), %ymm12
	vmulps	%ymm7, %ymm12, %ymm7
.Ltmp2156:
	vaddps	%ymm7, %ymm3, %ymm3
	movq	1344(%rsp), %r9
	vsubps	(%r9,%r8,4), %ymm3, %ymm3
	movq	320(%rsp), %rsi
.Ltmp2157:
	leaq	(%rcx,%rsi), %r8
.Ltmp2158:
	.loc	29 360 14 is_stmt 1
	vdivps	800(%rsp), %ymm3, %ymm12
.Ltmp2159:
	.loc	11 551 14
	vmovups	%ymm7, (%r9,%rdi,4)
.Ltmp2160:
	.loc	1 1661 43
	vmovaps	2016(%rsp), %ymm7
	vbroadcastss	.LCPI1_2(%rip), %ymm15
.Ltmp2161:
	.loc	29 347 14
	vsubps	%ymm12, %ymm15, %ymm12
	vxorps	%xmm14, %xmm14, %xmm14
.Ltmp2162:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm7, %ymm12, %ymm13
.Ltmp2163:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm2, %ymm13, %ymm13
.Ltmp2164:
	.loc	29 48 14
	vaddps	%ymm7, %ymm13, %ymm7
.Ltmp2165:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm13
.Ltmp2166:
	.loc	29 233 14
	vmaxps	%ymm7, %ymm12, %ymm7
.Ltmp2167:
	.loc	29 82 19
	vandps	%ymm7, %ymm13, %ymm12
.Ltmp2168:
	.loc	29 871 14
	vbroadcastss	.LCPI1_5(%rip), %ymm13
	vcmplt_oqps	%ymm13, %ymm12, %ymm12
.Ltmp2169:
	.loc	29 82 19
	vandnps	%ymm7, %ymm12, %ymm7
.Ltmp2170:
	.loc	1 1662 5
	vmovaps	%ymm7, 2016(%rsp)
.Ltmp2171:
	.loc	4 1050 16
	leaq	7(,%r8,8), %rsi
.Ltmp2172:
	.loc	1 1132 16
	shlq	$3, %r8
.Ltmp2173:
	.loc	4 1050 16
	cmpq	864(%rsp), %rsi
	jae	.LBB1_369
.Ltmp2174:
	.loc	1 0 0 is_stmt 0
	movq	%rcx, %rsi
	shlq	$5, %rsi
	addq	704(%rsp), %rsi
	incq	%rcx
.Ltmp2175:
	vblendvps	%ymm5, %ymm0, %ymm14, %ymm0
	vblendvps	%ymm1, %ymm9, %ymm14, %ymm9
.Ltmp2176:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm7, %ymm15, %ymm1
	movq	1312(%rsp), %rdi
.Ltmp2177:
	.loc	11 551 14
	vmovups	(%rdi,%r8,4), %ymm5
.Ltmp2178:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rsi), %ymm7
.Ltmp2179:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm5, %ymm1, %ymm1
.Ltmp2180:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm5, %ymm1, %ymm1
.Ltmp2181:
	.loc	11 551 14
	vmovups	%ymm7, (%rdi,%r8,4)
.Ltmp2182:
	.loc	11 551 14 is_stmt 0
	vmovups	%ymm1, (%rsi)
	vmovaps	%ymm10, %ymm14
	vmovaps	%ymm10, %ymm1
	vmovaps	%ymm3, %ymm5
	vmovaps	%ymm9, %ymm7
	vmovaps	%ymm2, %ymm12
	vmovaps	%ymm4, %ymm15
.Ltmp2183:
	.loc	14 304 12 is_stmt 1
	cmpq	%r15, %rcx
	vxorps	%xmm13, %xmm13, %xmm13
	jne	.LBB1_239
	jmp	.LBB1_234
.Ltmp2184:
.LBB1_253:
	.loc	14 0 12 is_stmt 0
	movq	%r15, 416(%rsp)
	movq	%r9, 320(%rsp)
.Ltmp2185:
	.loc	1 1656 5 is_stmt 1
	vmovaps	%ymm3, 2048(%rsp)
.Ltmp2186:
	.loc	1 853 0
	vmovaps	%ymm4, 1984(%rsp)
.Ltmp2187:
	.loc	1 855 0
	vmovaps	%ymm2, 1888(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm9, 1952(%rsp)
	vmovaps	%ymm11, 1856(%rsp)
	vmovaps	%ymm8, 1760(%rsp)
	vmovaps	%ymm0, 1824(%rsp)
	movq	88(%rsp), %r15
	movq	928(%rsp), %rdi
	vmovaps	1056(%rsp), %ymm11
	vmovaps	1024(%rsp), %ymm12
	vmovaps	992(%rsp), %ymm15
	vmovaps	960(%rsp), %ymm3
	movq	2208(%rsp), %r14
	vmovaps	2176(%rsp), %ymm4
	movq	1184(%rsp), %r9
	vmovaps	32(%rsp), %ymm7
	vmovaps	96(%rsp), %ymm9
	vmovaps	2144(%rsp), %ymm8
	vmovaps	2112(%rsp), %ymm10
	vmovaps	768(%rsp), %ymm1
	jmp	.LBB1_231
.Ltmp2188:
.LBB1_254:
	vmovaps	448(%rsp), %ymm0
.Ltmp2189:
	.loc	1 1656 5 is_stmt 1
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2190:
	.loc	1 853 0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp2191:
	.loc	1 855 0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	384(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp2192:
	leaq	8(%r8), %rsi
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2193:
.LBB1_255:
	cmpq	%r15, %r11
.Ltmp2194:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB1_396
.Ltmp2195:
	.loc	4 0 16 is_stmt 0
	movq	%r11, %rdx
	movq	%r13, %rsi
	.p2align	4
.LBB1_257:
.Ltmp2196:
	.loc	15 1504 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB1_308
.Ltmp2197:
	.loc	10 1078 5
	cmpq	$32, %rdx
	movl	$32, %edi
	cmovbq	%rdx, %rdi
.Ltmp2198:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp2199:
	.loc	16 0 18 is_stmt 0
.Ltmp2200:
	.p2align	4
.LBB1_259:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp2201:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp2202:
	.loc	6 180 28
	jne	.LBB1_259
.Ltmp2203:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp2204:
	.loc	19 2054 74
	subq	%rdi, %rdx
.Ltmp2205:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB1_257
	jmp	.LBB1_39
.Ltmp2206:
.LBB1_262:
	.loc	1 3418 31
	movl	3368(%rsp), %r10d
	movq	24(%rsp), %rbx
	movq	88(%rsp), %r15
.LBB1_263:
	.loc	1 3418 10
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
.Ltmp2207:
	.loc	1 3421 23
	movq	1736(%rbx), %rdx
.Ltmp2208:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_270
.Ltmp2209:
	.loc	1 3421 23
	movq	1728(%rbx), %rax
.Ltmp2210:
	.loc	11 551 14
	vmovaps	576(%rsp), %ymm0
	vmovups	%ymm0, (%rax)
.Ltmp2211:
	.loc	1 3422 5
	movq	1768(%rbx), %rax
.Ltmp2212:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp2213:
	.loc	6 180 28
	je	.LBB1_267
.Ltmp2214:
	.loc	6 0 28 is_stmt 0
	movq	1760(%rbx), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB1_266:
.Ltmp2215:
	.loc	20 66 21 is_stmt 1
	movl	%r10d, (%rcx,%rdx)
.Ltmp2216:
	.loc	7 1714 9
	addq	$4, %rdx
	cmpq	%rdx, %rax
.Ltmp2217:
	.loc	6 180 28
	jne	.LBB1_266
.Ltmp2218:
.LBB1_267:
	.loc	6 0 28 is_stmt 0
	leaq	1376(%rsp), %rdi
	movq	16(%rsp), %rsi
	.loc	1 3424 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	320(%rsp), %rax
	.loc	1 3425 5
	movl	%eax, 1640(%rbx)
.Ltmp2219:
.LBB1_268:
	.loc	1 0 5 is_stmt 0
	movq	416(%rsp), %rax
	movl	%eax, 1644(%rbx)
.Ltmp2220:
	.loc	1 2334 13 is_stmt 1
	cmpb	$0, 764(%rsp)
	je	.LBB1_345
	jmp	.LBB1_337
.LBB1_269:
	.loc	1 0 13 is_stmt 0
	vmovaps	128(%rsp), %ymm0
.Ltmp2221:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2222:
.LBB1_270:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_271:
	movq	%r11, %r12
	movq	%r15, %r14
	vmovaps	128(%rsp), %ymm0
.Ltmp2223:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2224:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 3264(%rsp)
.Ltmp2225:
.LBB1_272:
	leaq	1376(%rsp), %r15
	leaq	3264(%rsp), %rsi
	.loc	1 3313 5 is_stmt 1
	movl	$736, %edx
	movq	%r15, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	.loc	1 3313 14 is_stmt 0
	movq	%r15, %rdi
	movq	16(%rsp), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	.loc	1 3314 5 is_stmt 1
	movl	%r12d, 1640(%rbx)
	movq	96(%rsp), %rax
	.loc	1 3315 5
	movl	%eax, 1644(%rbx)
	movq	%r14, %r15
.Ltmp2226:
	.loc	1 2334 13
	cmpb	$0, 764(%rsp)
	je	.LBB1_345
.LBB1_337:
	.loc	1 0 13 is_stmt 0
	movq	16(%rsp), %rdi
	.loc	1 2334 32
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2334 22
	testb	%al, %al
	je	.LBB1_345
	.loc	1 0 22
	movq	3208(%rsp), %rsi
	cmpq	%r15, %rsi
.Ltmp2227:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB1_397
.Ltmp2228:
	.loc	4 0 16 is_stmt 0
	movq	%r13, %rax
	.p2align	4
.LBB1_340:
.Ltmp2229:
	.loc	17 131 18 is_stmt 1
	movq	%rsi, %rcx
.Ltmp2230:
	.loc	15 1504 12
	testq	%rsi, %rsi
	je	.LBB1_344
.Ltmp2231:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp2232:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%r9d, %r9d
	xorl	%r8d, %r8d
.Ltmp2233:
	.loc	16 0 18 is_stmt 0
.Ltmp2234:
	.p2align	4
.LBB1_342:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r8), %r9d
.Ltmp2235:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp2236:
	.loc	6 180 28
	jne	.LBB1_342
.Ltmp2237:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp2238:
	.loc	19 2054 74
	movq	%rcx, %rsi
	subq	%rdx, %rsi
.Ltmp2239:
	.loc	17 136 12
	testl	%r9d, %r9d
	je	.LBB1_340
.Ltmp2240:
.LBB1_344:
	.loc	15 1504 12
	testq	%rcx, %rcx
	sete	%al
	jmp	.LBB1_346
.Ltmp2241:
.LBB1_273:
	.loc	15 0 12 is_stmt 0
	leaq	1376(%rsp), %rdi
	movq	16(%rsp), %r15
.Ltmp2242:
	.loc	1 3334 24 is_stmt 1
	movq	%r15, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp2243:
	.loc	1 3339 19
	movzbl	1536(%rbx), %eax
	movb	%al, 224(%rsp)
	movq	%rbx, %rax
.Ltmp2244:
	.loc	1 3340 21
	movzbl	1537(%rbx), %ecx
	movb	%cl, 32(%rsp)
.Ltmp2245:
	.loc	1 3341 16
	movq	1624(%rbx), %r12
.Ltmp2246:
	.loc	1 3342 16
	movq	1632(%rbx), %rbx
.Ltmp2247:
	.loc	1 3343 27
	movl	1640(%rax), %r14d
.Ltmp2248:
	.loc	1 3344 27
	movl	1644(%rax), %eax
	movq	%rax, 416(%rsp)
	leaq	4640(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	3264(%rsp), %rdi
.Ltmp2249:
	.loc	1 3348 32
	movq	%r15, %rsi
	movq	%r12, %rdx
	movq	%rbx, 928(%rsp)
	movq	%rbx, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	8(%rsp), %rax
.Ltmp2250:
	.loc	4 3758 16
	leaq	31(%rax), %rsi
	shrq	$5, %rsi
	vmovaps	1376(%rsp), %ymm0
	vmovaps	3264(%rsp), %ymm4
.Ltmp2251:
	.loc	8 446 20
	je	.LBB1_323
.Ltmp2252:
	.loc	8 0 20 is_stmt 0
	vmovaps	%ymm0, 96(%rsp)
.Ltmp2253:
	.loc	29 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm13
.Ltmp2254:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm13, %ymm0
	vmovaps	%ymm13, %ymm14
	cmpb	$0, 224(%rsp)
	movq	88(%rsp), %r15
	jne	.LBB1_276
.Ltmp2255:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, %ymm14
.LBB1_276:
	cmpb	$0, 32(%rsp)
	movq	%r14, %r10
	jne	.LBB1_278
	vmovaps	%ymm0, %ymm13
.LBB1_278:
	vmovaps	1408(%rsp), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	1440(%rsp), %ymm10
	vmovaps	1472(%rsp), %ymm11
	vmovaps	1504(%rsp), %ymm12
	vmovaps	1536(%rsp), %ymm1
	vmovaps	1568(%rsp), %ymm3
	vmovaps	1600(%rsp), %ymm5
	vmovaps	1632(%rsp), %ymm15
	vmovaps	1664(%rsp), %ymm6
	vmovaps	1696(%rsp), %ymm7
	vmovaps	1728(%rsp), %ymm9
	movl	3368(%rsp), %r11d
	vmovaps	2048(%rsp), %ymm0
	movq	3352(%rsp), %rax
	movq	%rax, 608(%rsp)
	movq	3360(%rsp), %rax
	movq	%rax, 160(%rsp)
	vmovaps	1760(%rsp), %ymm2
	vmovaps	%ymm2, 896(%rsp)
	vmovaps	1888(%rsp), %ymm2
	vmovaps	%ymm2, 832(%rsp)
	movq	3296(%rsp), %r13
	movq	3304(%rsp), %r14
	movq	3344(%rsp), %rax
	movq	%rax, 1152(%rsp)
	movq	3312(%rsp), %rax
	movq	%rax, 800(%rsp)
	movq	3320(%rsp), %rax
	movq	%rax, 1120(%rsp)
	vmovaps	2080(%rsp), %ymm2
	vmovaps	%ymm2, 1344(%rsp)
	movq	3336(%rsp), %rax
	movq	%rax, 864(%rsp)
	movq	3328(%rsp), %rax
	movq	%rax, 1312(%rsp)
	xorl	%r9d, %r9d
	movl	$32, %ecx
	movq	216(%rsp), %rax
	movq	%rax, 672(%rsp)
	movq	%r15, %r8
	movq	$0, 768(%rsp)
	movq	8(%rsp), %rdi
	movq	%rdi, %rdx
	xorl	%eax, %eax
	movq	24(%rsp), %rbx
	vmovaps	%ymm13, 4032(%rsp)
	vmovaps	%ymm14, 4000(%rsp)
.LBB1_279:
	movq	%rsi, 2400(%rsp)
.Ltmp2256:
	.loc	4 2584 13 is_stmt 1
	cmpq	$32, %rdx
	movl	$32, %esi
	movq	%rdx, 3232(%rsp)
	cmovbq	%rdx, %rsi
	cmpq	$1, %rsi
	movq	%rsi, 128(%rsp)
	adcq	$0, %rsi
.Ltmp2257:
	.loc	10 1916 50
	movq	%rdi, %rdx
	subq	%rax, %rdx
.Ltmp2258:
	.loc	10 1078 5
	cmpq	$32, %rdx
	cmovaeq	%rcx, %rdx
	movq	%rdx, 2304(%rsp)
.Ltmp2259:
	.loc	10 1916 50
	cmpq	%rax, %rdi
.Ltmp2260:
	.loc	3 900 12
	jne	.LBB1_282
.Ltmp2261:
	.loc	3 0 12 is_stmt 0
	movq	2400(%rsp), %rsi
.LBB1_281:
.Ltmp2262:
	addq	$32, %rax
	decq	%rsi
	movq	3232(%rsp), %rdx
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rdx
	addq	$256, 768(%rsp)
	addq	$-256, %r8
	addq	$1024, 672(%rsp)
	testq	%rsi, %rsi
	movq	8(%rsp), %rdi
	movl	$32, %ecx
	jne	.LBB1_279
	jmp	.LBB1_383
.Ltmp2263:
.LBB1_282:
	.loc	8 0 20 is_stmt 0
	vmovaps	%ymm9, 4128(%rsp)
	movq	%rax, 2272(%rsp)
	vmovaps	%ymm0, 4064(%rsp)
	vmovaps	%ymm4, 4096(%rsp)
	shll	$3, %esi
	vmovaps	(%rbx), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	32(%rbx), %ymm0
	vmovaps	%ymm0, 352(%rsp)
	vmovaps	64(%rbx), %ymm0
	vmovaps	%ymm0, 512(%rsp)
	vmovaps	96(%rbx), %ymm0
	vmovaps	%ymm0, 704(%rsp)
	vmovaps	128(%rbx), %ymm0
	vmovaps	%ymm0, 1088(%rsp)
	vmovaps	160(%rbx), %ymm0
	vmovaps	%ymm0, 480(%rsp)
	vmovaps	192(%rbx), %ymm0
	vmovaps	%ymm0, 384(%rsp)
	vmovaps	224(%rbx), %ymm0
	vmovaps	%ymm0, 288(%rsp)
	vmovaps	256(%rbx), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	288(%rbx), %ymm0
	vmovaps	%ymm0, 448(%rsp)
	vmovaps	320(%rbx), %ymm0
	vmovaps	%ymm0, 1056(%rsp)
	movq	%r8, 3200(%rsp)
	xorl	%eax, %eax
	vmovaps	%ymm3, %ymm14
	vmovaps	%ymm1, %ymm13
	vmovaps	352(%rbx), %ymm0
	vmovaps	%ymm0, 1024(%rsp)
	vmovaps	384(%rbx), %ymm0
	vmovaps	%ymm0, 992(%rsp)
	vmovaps	416(%rbx), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	448(%rbx), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	480(%rbx), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	512(%rbx), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	544(%rbx), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	576(%rbx), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
	vmovaps	608(%rbx), %ymm0
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	640(%rbx), %ymm0
	vmovaps	%ymm0, 3104(%rsp)
	vmovaps	672(%rbx), %ymm0
	vmovaps	%ymm0, 3072(%rsp)
	vmovaps	704(%rbx), %ymm0
	vmovaps	%ymm0, 3040(%rsp)
	vmovaps	736(%rbx), %ymm0
	vmovaps	%ymm0, 3008(%rsp)
	vmovaps	768(%rbx), %ymm0
	vmovaps	%ymm0, 2976(%rsp)
	vmovaps	800(%rbx), %ymm0
	vmovaps	%ymm0, 2944(%rsp)
	vmovaps	832(%rbx), %ymm0
	vmovaps	%ymm0, 2912(%rsp)
	vmovaps	864(%rbx), %ymm0
	vmovaps	%ymm0, 2880(%rsp)
	vmovaps	896(%rbx), %ymm0
	vmovaps	%ymm0, 2848(%rsp)
	vmovaps	928(%rbx), %ymm0
	vmovaps	%ymm0, 2816(%rsp)
	vmovaps	960(%rbx), %ymm0
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	992(%rbx), %ymm0
	vmovaps	%ymm0, 2752(%rsp)
	vmovaps	1024(%rbx), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	1056(%rbx), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	1088(%rbx), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	1120(%rbx), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
	vmovaps	1152(%rbx), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	1184(%rbx), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	1216(%rbx), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	1248(%rbx), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1280(%rbx), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1312(%rbx), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1344(%rbx), %ymm0
	vmovaps	%ymm0, 640(%rsp)
	vmovaps	1376(%rbx), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	1408(%rbx), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	1440(%rbx), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	vmovaps	1472(%rbx), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	1504(%rbx), %ymm0
	vmovaps	%ymm0, 1184(%rsp)
	vmovaps	%ymm5, 4256(%rsp)
	vmovaps	%ymm5, 32(%rsp)
	vmovaps	%ymm15, 4224(%rsp)
	vmovaps	%ymm15, %ymm1
	vmovaps	%ymm6, 4192(%rsp)
	vmovaps	%ymm6, %ymm3
	vmovaps	%ymm7, 4160(%rsp)
	vmovaps	2240(%rsp), %ymm0
	.p2align	4
.LBB1_283:
	vmovaps	%ymm7, %ymm8
	vmovaps	%ymm3, %ymm7
	vmovaps	%ymm1, %ymm3
	vmovaps	32(%rsp), %ymm15
	vmovaps	%ymm14, %ymm1
	vmovaps	96(%rsp), %ymm14
	movq	768(%rsp), %rdi
.Ltmp2264:
	.loc	5 568 12 is_stmt 1
	addq	%rax, %rdi
	cmpq	%r15, %rdi
	ja	.LBB1_385
.Ltmp2265:
	.loc	5 438 16
	cmpq	$7, %r8
	jbe	.LBB1_386
.Ltmp2266:
	.loc	5 0 16 is_stmt 0
	movq	672(%rsp), %rdi
.Ltmp2267:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rdi,%rax,4), %ymm4
	vmovaps	%ymm0, %ymm5
	vmovaps	%ymm15, 224(%rsp)
	vmovaps	%ymm10, %ymm15
	vmovaps	%ymm11, %ymm6
	vmovaps	%ymm12, %ymm9
	vmovaps	%ymm13, %ymm2
.Ltmp2268:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm0
	vmovaps	%ymm0, 544(%rsp)
.Ltmp2269:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm4, %ymm10
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp2270:
	.loc	29 48 14
	vaddps	%ymm0, %ymm10, %ymm10
.Ltmp2271:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm4, %ymm11
.Ltmp2272:
	.loc	29 48 14
	vaddps	%ymm0, %ymm11, %ymm11
.Ltmp2273:
	.loc	29 283 14
	vmulps	512(%rsp), %ymm4, %ymm12
.Ltmp2274:
	.loc	29 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm4, 96(%rsp)
.Ltmp2275:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm4, %ymm13
.Ltmp2276:
	.loc	29 48 14
	vaddps	%ymm0, %ymm13, %ymm13
	vmovaps	%ymm14, %ymm0
.Ltmp2277:
	.loc	29 283 14
	vmulps	1088(%rsp), %ymm14, %ymm14
.Ltmp2278:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2279:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm0, %ymm14
.Ltmp2280:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2281:
	.loc	29 283 14
	vmulps	384(%rsp), %ymm0, %ymm14
.Ltmp2282:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2283:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm0, %ymm14
.Ltmp2284:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2285:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm5, %ymm14
.Ltmp2286:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2287:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm5, %ymm14
.Ltmp2288:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2289:
	.loc	29 283 14
	vmulps	1056(%rsp), %ymm5, %ymm14
.Ltmp2290:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2291:
	.loc	29 283 14
	vmulps	1024(%rsp), %ymm5, %ymm14
.Ltmp2292:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2293:
	.loc	29 283 14
	vmulps	992(%rsp), %ymm15, %ymm14
.Ltmp2294:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2295:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm15, %ymm14
.Ltmp2296:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2297:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm15, %ymm14
.Ltmp2298:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2299:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm15, %ymm14
.Ltmp2300:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2301:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm6, %ymm14
.Ltmp2302:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2303:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm6, %ymm14
.Ltmp2304:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2305:
	.loc	29 283 14
	vmulps	3168(%rsp), %ymm6, %ymm14
.Ltmp2306:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2307:
	.loc	29 283 14
	vmulps	3136(%rsp), %ymm6, %ymm14
.Ltmp2308:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2309:
	.loc	29 283 14
	vmulps	3104(%rsp), %ymm9, %ymm14
.Ltmp2310:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2311:
	.loc	29 283 14
	vmulps	3072(%rsp), %ymm9, %ymm14
.Ltmp2312:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2313:
	.loc	29 283 14
	vmulps	3040(%rsp), %ymm9, %ymm14
.Ltmp2314:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2315:
	.loc	29 283 14
	vmulps	3008(%rsp), %ymm9, %ymm14
.Ltmp2316:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2317:
	.loc	29 283 14
	vmulps	2976(%rsp), %ymm2, %ymm14
.Ltmp2318:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2319:
	.loc	29 283 14
	vmulps	2944(%rsp), %ymm2, %ymm14
.Ltmp2320:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2321:
	.loc	29 283 14
	vmulps	2912(%rsp), %ymm2, %ymm14
.Ltmp2322:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2323:
	.loc	29 283 14
	vmulps	2880(%rsp), %ymm2, %ymm14
.Ltmp2324:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2325:
	.loc	29 283 14
	vmulps	2848(%rsp), %ymm1, %ymm14
.Ltmp2326:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2327:
	.loc	29 283 14
	vmulps	2816(%rsp), %ymm1, %ymm14
.Ltmp2328:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2329:
	.loc	29 283 14
	vmulps	2784(%rsp), %ymm1, %ymm14
.Ltmp2330:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
	vmovaps	%ymm1, 32(%rsp)
.Ltmp2331:
	.loc	29 283 14
	vmulps	2752(%rsp), %ymm1, %ymm14
	vmovaps	224(%rsp), %ymm1
.Ltmp2332:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2333:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm1, %ymm14
.Ltmp2334:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2335:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm1, %ymm14
.Ltmp2336:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2337:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm1, %ymm14
.Ltmp2338:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2339:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm1, %ymm14
.Ltmp2340:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2341:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm3, %ymm14
.Ltmp2342:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2343:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm3, %ymm14
.Ltmp2344:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2345:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm3, %ymm14
.Ltmp2346:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2347:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm3, %ymm14
.Ltmp2348:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2349:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm7, %ymm14
.Ltmp2350:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2351:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm7, %ymm14
.Ltmp2352:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2353:
	.loc	29 283 14
	vmulps	640(%rsp), %ymm7, %ymm14
.Ltmp2354:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2355:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm7, %ymm14
.Ltmp2356:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2357:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm8, %ymm14
.Ltmp2358:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2359:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm8, %ymm14
.Ltmp2360:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2361:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm8, %ymm14
.Ltmp2362:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
	vmovaps	%ymm8, 2336(%rsp)
.Ltmp2363:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm8, %ymm14
	vmovaps	544(%rsp), %ymm8
.Ltmp2364:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp2365:
	.loc	29 82 19
	vandps	%ymm2, %ymm8, %ymm14
.Ltmp2366:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm8, %ymm10, %ymm10
.Ltmp2367:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm10, %ymm14, %ymm10
.Ltmp2368:
	.loc	29 82 19
	vandps	%ymm8, %ymm11, %ymm11
.Ltmp2369:
	.loc	29 233 14
	vmaxps	%ymm11, %ymm10, %ymm10
.Ltmp2370:
	.loc	29 82 19
	vandps	%ymm8, %ymm12, %ymm11
.Ltmp2371:
	.loc	29 233 14
	vmaxps	%ymm11, %ymm10, %ymm10
.Ltmp2372:
	.loc	29 82 19
	vandps	%ymm8, %ymm13, %ymm11
.Ltmp2373:
	.loc	29 233 14
	vmaxps	%ymm11, %ymm10, %ymm10
.Ltmp2374:
	.loc	11 551 14
	vmovups	%ymm10, 4640(%rsp,%rax,4)
.Ltmp2375:
	.loc	10 1916 50
	addq	$8, %rax
	addq	$-8, %r8
	vmovaps	%ymm2, %ymm14
	vmovaps	%ymm9, %ymm13
	vmovaps	%ymm6, %ymm12
	vmovaps	%ymm15, %ymm11
	vmovaps	%ymm5, %ymm10
	cmpq	%rax, %rsi
.Ltmp2376:
	.loc	3 900 12
	jne	.LBB1_283
.Ltmp2377:
	.loc	3 0 12 is_stmt 0
	vmovaps	%ymm3, 256(%rsp)
	vmovaps	%ymm7, 288(%rsp)
	vmovaps	%ymm2, 960(%rsp)
	vmovaps	%ymm9, 992(%rsp)
	vmovaps	%ymm6, 1024(%rsp)
	vmovaps	%ymm15, 1056(%rsp)
	vmovaps	%ymm5, 448(%rsp)
	vmovaps	%ymm0, 2240(%rsp)
	xorl	%ecx, %ecx
	movq	416(%rsp), %rdi
	vmovaps	4096(%rsp), %ymm4
	vmovaps	4032(%rsp), %ymm13
	vmovaps	4000(%rsp), %ymm14
	vmovaps	4064(%rsp), %ymm0
	vbroadcastss	.LCPI1_2(%rip), %ymm1
	vbroadcastss	.LCPI1_3(%rip), %ymm2
	vbroadcastss	.LCPI1_4(%rip), %ymm5
	vbroadcastss	.LCPI1_5(%rip), %ymm6
	movq	2304(%rsp), %r15
	vmovaps	2336(%rsp), %ymm9
	vmovaps	%ymm8, %ymm3
	jmp	.LBB1_289
.LBB1_287:
	movq	320(%rsp), %r10
	movq	384(%rsp), %r11
.LBB1_288:
	movq	480(%rsp), %rcx
.Ltmp2378:
	addq	%r15, %rcx
	movq	416(%rsp), %rdi
.Ltmp2379:
	.loc	1 3413 39 is_stmt 1
	addq	%r15, %rdi
.Ltmp2380:
	.loc	1 1150 8
	cmpq	%r12, %rdi
	movq	%r12, %rax
	movl	$0, %r9d
	cmovbq	%r9, %rax
	subq	%rax, %rdi
.Ltmp2381:
	.loc	1 3414 39
	addq	%r15, %r10
	movq	928(%rsp), %rax
.Ltmp2382:
	.loc	1 1150 8
	cmpq	%rax, %r10
	cmovbq	%r9, %rax
	subq	%rax, %r10
	movq	2304(%rsp), %r15
.Ltmp2383:
	.loc	1 3362 19
	cmpq	%r15, %rcx
	movq	24(%rsp), %rbx
	jae	.LBB1_307
.LBB1_289:
	.loc	1 0 19 is_stmt 0
	movq	%rcx, 480(%rsp)
	movq	%r11, 384(%rsp)
	.loc	1 3370 21 is_stmt 1
	subq	%rcx, %r15
.Ltmp2384:
	.loc	1 1577 16
	movq	1624(%rbx), %r8
.Ltmp2385:
	.loc	1 1578 16
	movq	1632(%rbx), %rcx
.Ltmp2386:
	.loc	1 1579 25
	leaq	1(%rdi), %rax
.Ltmp2387:
	.loc	1 1150 8
	cmpq	%r8, %rax
	movq	%r8, %rsi
	cmovbq	%r9, %rsi
	negq	%rsi
	movq	%rdi, %rbx
	movq	608(%rsp), %rax
.Ltmp2388:
	.loc	1 1580 28
	leaq	(%rdi,%rax), %r11
.Ltmp2389:
	.loc	1 1150 8
	cmpq	%r8, %r11
	movq	%r8, %rax
	cmovbq	%r9, %rax
	subq	%rax, %r11
	movq	160(%rsp), %rax
.Ltmp2390:
	.loc	1 1582 33
	addq	%rdi, %rax
.Ltmp2391:
	.loc	1 1150 8
	cmpq	%r8, %rax
	movq	%r8, %rdi
	cmovbq	%r9, %rdi
	subq	%rdi, %rax
.Ltmp2392:
	.loc	1 1585 14
	movq	%r8, %r9
	subq	%rbx, %r9
.Ltmp2393:
	.loc	10 1078 5
	cmpq	%r15, %r9
	cmovbq	%r9, %r15
	movq	%r10, 320(%rsp)
.Ltmp2394:
	.loc	1 1586 14
	subq	%r10, %rcx
.Ltmp2395:
	.loc	10 1078 5
	cmpq	%r15, %rcx
	cmovbq	%rcx, %r15
	movq	%rbx, 416(%rsp)
.Ltmp2396:
	.loc	1 1150 8
	addq	%rbx, %rsi
	incq	%rsi
.Ltmp2397:
	.loc	1 1587 14
	movq	%r8, %r10
	movq	%rsi, 1088(%rsp)
	subq	%rsi, %r10
.Ltmp2398:
	.loc	10 1078 5
	cmpq	%r15, %r10
	cmovbq	%r10, %r15
.Ltmp2399:
	.loc	1 1588 14
	movq	%r8, %rbx
	movq	%r11, 352(%rsp)
	subq	%r11, %rbx
.Ltmp2400:
	.loc	10 1078 5
	cmpq	%r15, %rbx
	cmovbq	%rbx, %r15
.Ltmp2401:
	.loc	1 1590 14
	subq	%rax, %r8
.Ltmp2402:
	.loc	10 1078 5
	cmpq	%r15, %r8
	cmovbq	%r8, %r15
	movq	2272(%rsp), %rdx
	movq	480(%rsp), %rsi
.Ltmp2403:
	.loc	1 3376 28
	leaq	(%rsi,%rdx), %rdi
.Ltmp2404:
	.loc	1 3378 55
	leaq	(%r15,%rdi), %rsi
.Ltmp2405:
	.loc	1 3376 28
	shlq	$3, %rdi
.Ltmp2406:
	.loc	1 3378 55
	shlq	$3, %rsi
.Ltmp2407:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB1_395
	cmpq	88(%rsp), %rsi
	ja	.LBB1_395
.Ltmp2408:
	.loc	14 304 12
	testq	%r15, %r15
	je	.LBB1_287
.Ltmp2409:
	.loc	14 0 12 is_stmt 0
	movq	480(%rsp), %rdx
	movq	%rdx, %rsi
	shlq	$5, %rsi
	cmpq	%r8, %rbx
	cmovbq	%rbx, %r8
	cmpq	%r10, %r8
	cmovaeq	%r10, %r8
	movq	216(%rsp), %r10
	leaq	(%r10,%rdi,4), %rdi
	movq	%rdi, 704(%rsp)
	cmpq	%r9, %r8
	cmovaeq	%r9, %r8
	cmpq	%rcx, %r8
	cmovaeq	%rcx, %r8
	movq	128(%rsp), %rcx
	subq	%rdx, %rcx
	cmpq	%rcx, %r8
	cmovbq	%r8, %rcx
	leaq	(%rsp,%rsi), %rdx
	addq	$4640, %rdx
	movq	%rdx, 512(%rsp)
	movabsq	$2305843009213693951, %rsi
	andq	%rsi, %rcx
	xorl	%ebx, %ebx
	vmovaps	%ymm4, %ymm10
	movq	384(%rsp), %r11
	.p2align	4
.LBB1_293:
	movq	416(%rsp), %rsi
.Ltmp2410:
	.loc	1 1504 26 is_stmt 1
	leaq	(%rbx,%rsi), %r8
.Ltmp2411:
	.loc	1 1139 16
	leaq	(,%r8,8), %rdi
.Ltmp2412:
	.loc	1 1140 33
	leaq	8(,%r8,8), %rsi
.Ltmp2413:
	.loc	4 1050 16
	leaq	7(,%r8,8), %r8
	cmpq	%r14, %r8
	jae	.LBB1_379
.Ltmp2414:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rbx,8), %r8
	movq	512(%rsp), %rdx
.Ltmp2415:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rdx,%r8,4), %ymm11
.Ltmp2416:
	.loc	29 233 14
	vmaxps	%ymm11, %ymm11, %ymm12
.Ltmp2417:
	.loc	29 585 19
	vblendvps	%ymm14, %ymm12, %ymm11, %ymm11
	vmovaps	896(%rsp), %ymm4
.Ltmp2418:
	.loc	29 360 14
	vdivps	%ymm11, %ymm4, %ymm12
	movq	352(%rsp), %rdx
.Ltmp2419:
	.loc	1 0 0 is_stmt 0
	leaq	(%rbx,%rdx), %r9
.Ltmp2420:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm4, %ymm11, %ymm11
.Ltmp2421:
	.loc	29 585 19
	vblendvps	%ymm11, %ymm12, %ymm1, %ymm11
.Ltmp2422:
	.loc	11 551 14
	vmovups	%ymm11, (%r13,%rdi,4)
.Ltmp2423:
	.loc	1 1132 16
	leaq	(,%r9,8), %r8
.Ltmp2424:
	.loc	4 1050 16
	leaq	7(,%r9,8), %r10
	cmpq	%r14, %r10
	jae	.LBB1_322
.Ltmp2425:
	.loc	11 551 14
	vmovups	(%r13,%r8,4), %ymm11
	vmovaps	%ymm11, %ymm4
.Ltmp2426:
	.loc	1 1208 22
	testl	%r11d, %r11d
	je	.LBB1_297
.Ltmp2427:
	.loc	29 257 14
	vminps	%ymm11, %ymm10, %ymm4
.Ltmp2428:
.LBB1_297:
	.loc	1 0 0 is_stmt 0
	movl	%r11d, %r11d
.Ltmp2429:
	.loc	1 1214 20 is_stmt 1
	incq	%r11
	movq	1152(%rsp), %rdx
	movq	%rdx, %r10
	cmpq	%rdx, %r11
.Ltmp2430:
	.loc	1 1215 22
	jne	.LBB1_301
	.loc	1 0 22 is_stmt 0
.Ltmp2431:
	.p2align	4
.LBB1_298:
.Ltmp2432:
	.loc	1 1132 16 is_stmt 1
	leaq	(,%r9,8), %r8
.Ltmp2433:
	.loc	4 1050 16
	leaq	7(,%r9,8), %r11
	cmpq	%r14, %r11
	jae	.LBB1_322
.Ltmp2434:
	.loc	29 257 14
	vminps	(%r13,%r8,4), %ymm11, %ymm11
.Ltmp2435:
	.loc	11 551 14
	vmovups	%ymm11, (%r13,%r8,4)
.Ltmp2436:
	.loc	1 1226 16
	testq	%r9, %r9
	cmoveq	%r12, %r9
	.loc	1 1229 13
	decq	%r9
.Ltmp2437:
	.loc	10 1916 50
	decq	%r10
.Ltmp2438:
	.loc	3 900 12
	jne	.LBB1_298
.Ltmp2439:
	.loc	3 0 12 is_stmt 0
	xorl	%r11d, %r11d
	vmovaps	%ymm4, %ymm10
	jmp	.LBB1_303
	.p2align	4
.LBB1_301:
	movq	1088(%rsp), %rdx
	leaq	(%rbx,%rdx), %r8
.Ltmp2440:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r8,8), %r9
.Ltmp2441:
	.loc	1 1132 16
	shlq	$3, %r8
.Ltmp2442:
	.loc	4 1050 16
	cmpq	%r14, %r9
	jae	.LBB1_322
.Ltmp2443:
	.loc	11 551 14
	vmovups	(%r13,%r8,4), %ymm10
.Ltmp2444:
	.loc	29 257 14
	vminps	%ymm4, %ymm10, %ymm10
.Ltmp2445:
.LBB1_303:
	.loc	1 0 0 is_stmt 0
	leaq	(%rbx,%rax), %r8
.Ltmp2446:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r8,8), %r9
.Ltmp2447:
	.loc	1 1132 16
	shlq	$3, %r8
	movq	1120(%rsp), %rdx
.Ltmp2448:
	.loc	4 1050 16
	cmpq	%rdx, %r9
	movq	320(%rsp), %r10
	jae	.LBB1_380
.Ltmp2449:
	.loc	4 0 16 is_stmt 0
	cmpq	%rdx, %rsi
.Ltmp2450:
	.loc	4 1050 16
	ja	.LBB1_381
.Ltmp2451:
	.loc	1 0 0
	leaq	(%rbx,%r10), %r9
.Ltmp2452:
	vmulps	%ymm2, %ymm10, %ymm10
	vroundps	$9, %ymm10, %ymm10
	vmulps	%ymm5, %ymm10, %ymm10
.Ltmp2453:
	vaddps	%ymm0, %ymm10, %ymm11
	movq	800(%rsp), %rdx
	vsubps	(%rdx,%r8,4), %ymm11, %ymm0
.Ltmp2454:
	.loc	11 551 14 is_stmt 1
	vmovups	%ymm10, (%rdx,%rdi,4)
.Ltmp2455:
	.loc	1 1661 43
	vmovaps	2016(%rsp), %ymm10
.Ltmp2456:
	.loc	29 360 14
	vdivps	1344(%rsp), %ymm0, %ymm11
.Ltmp2457:
	.loc	29 347 14
	vsubps	%ymm11, %ymm1, %ymm11
.Ltmp2458:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm10, %ymm11, %ymm12
.Ltmp2459:
	.loc	29 283 14 is_stmt 1
	vmulps	832(%rsp), %ymm12, %ymm12
.Ltmp2460:
	.loc	29 48 14
	vaddps	%ymm12, %ymm10, %ymm10
.Ltmp2461:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm11, %ymm10
.Ltmp2462:
	.loc	29 82 19
	vandps	%ymm3, %ymm10, %ymm11
.Ltmp2463:
	.loc	29 871 14
	vcmplt_oqps	%ymm6, %ymm11, %ymm11
.Ltmp2464:
	.loc	29 82 19
	vandnps	%ymm10, %ymm11, %ymm10
.Ltmp2465:
	.loc	1 1662 5
	vmovaps	%ymm10, 2016(%rsp)
.Ltmp2466:
	.loc	4 1050 16
	leaq	7(,%r9,8), %rsi
.Ltmp2467:
	.loc	1 1132 16
	shlq	$3, %r9
.Ltmp2468:
	.loc	4 1050 16
	cmpq	864(%rsp), %rsi
	jae	.LBB1_382
.Ltmp2469:
	.loc	1 0 0 is_stmt 0
	movq	%rbx, %rsi
	shlq	$5, %rsi
	addq	704(%rsp), %rsi
	incq	%rbx
.Ltmp2470:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm10, %ymm1, %ymm10
	movq	1312(%rsp), %rdx
.Ltmp2471:
	.loc	11 551 14
	vmovups	(%rdx,%r9,4), %ymm11
.Ltmp2472:
	.loc	11 551 14 is_stmt 0
	vmovups	(%rsi), %ymm12
.Ltmp2473:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm11, %ymm10, %ymm10
.Ltmp2474:
	.loc	29 585 19
	vblendvps	%ymm13, %ymm11, %ymm10, %ymm10
.Ltmp2475:
	.loc	11 551 14
	vmovups	%ymm12, (%rdx,%r9,4)
.Ltmp2476:
	.loc	11 551 14 is_stmt 0
	vmovups	%ymm10, (%rsi)
	vmovaps	%ymm4, %ymm10
.Ltmp2477:
	.loc	14 304 12 is_stmt 1
	cmpq	%rcx, %rbx
	jne	.LBB1_293
	jmp	.LBB1_288
.Ltmp2478:
.LBB1_307:
	.loc	14 0 12 is_stmt 0
	movq	%rdi, 416(%rsp)
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	32(%rsp), %ymm5
	vmovaps	224(%rsp), %ymm15
	vmovaps	256(%rsp), %ymm6
	vmovaps	288(%rsp), %ymm7
	movq	88(%rsp), %r15
	movq	2400(%rsp), %rsi
	vmovaps	448(%rsp), %ymm10
	movq	3200(%rsp), %r8
	vmovaps	1056(%rsp), %ymm11
	vmovaps	1024(%rsp), %ymm12
	vmovaps	992(%rsp), %ymm1
	vmovaps	960(%rsp), %ymm3
	movq	2272(%rsp), %rax
	jmp	.LBB1_281
.LBB1_308:
	movb	$1, %r8b
	.loc	1 2316 12 is_stmt 1
	cmpb	$0, 2152(%rbx)
	je	.LBB1_40
.Ltmp2479:
	.loc	1 663 31
	movq	1768(%rbx), %rcx
	.loc	1 663 57 is_stmt 0
	movq	1832(%rbx), %rax
.Ltmp2480:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp2481:
	.loc	14 304 12
	testq	%rcx, %rcx
	je	.LBB1_317
.Ltmp2482:
	.loc	14 0 12 is_stmt 0
	movq	1760(%rbx), %rsi
	movq	1824(%rbx), %rdi
	xorl	%r8d, %r8d
	jmp	.LBB1_313
	.p2align	4
.LBB1_311:
.Ltmp2483:
	.loc	1 665 22 is_stmt 1
	xorl	%edx, %edx
	divq	%r9
.LBB1_312:
	.loc	1 665 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp2484:
	.loc	1 0 0
	incq	%r8
.Ltmp2485:
	.loc	14 304 12 is_stmt 1
	addq	$12, %rdi
	cmpq	%r8, %rcx
	je	.LBB1_317
.Ltmp2486:
.LBB1_313:
	.loc	1 664 26
	movl	(%rdi), %r9d
	testq	%r9, %r9
	movq	8(%rsp), %rax
.Ltmp2487:
	.loc	1 665 42
	je	.LBB1_425
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
	jb	.LBB1_311
	xorl	%edx, %edx
	divl	%r9d
	jmp	.LBB1_312
.Ltmp2488:
.LBB1_316:
	.loc	1 0 22
	movq	24(%rsp), %rbx
.Ltmp2489:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB1_335
.Ltmp2490:
.LBB1_317:
	.loc	1 2318 26
	movq	1632(%rbx), %rsi
.Ltmp2491:
	.loc	1 455 44
	testq	%rsi, %rsi
	movq	8(%rsp), %rdx
	je	.LBB1_436
.Ltmp2492:
	.loc	1 2318 26
	movq	1624(%rbx), %rcx
.Ltmp2493:
	.loc	1 455 23
	movl	1640(%rbx), %edi
	movq	%rdx, %rax
	.loc	1 455 44 is_stmt 0
	cmpq	%rsi, %rdx
	jb	.LBB1_320
	.loc	1 0 44
	movq	8(%rsp), %rax
	.loc	1 455 44
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB1_320:
	.loc	1 455 22
	addq	%rdi, %rax
	.loc	1 455 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB1_358
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB1_359
.Ltmp2494:
.LBB1_322:
	.loc	1 0 21
	vmovaps	32(%rsp), %ymm0
.Ltmp2495:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	256(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	288(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	%ymm9, 1728(%rsp)
.Ltmp2496:
	.loc	1 1133 25
	leaq	8(%r8), %rsi
.Ltmp2497:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r8, %rdi
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2498:
.LBB1_323:
	.loc	1 3418 31
	movl	3368(%rsp), %r11d
	movq	24(%rsp), %rbx
	movq	88(%rsp), %r15
	movq	%r14, %r10
.LBB1_324:
.Ltmp2499:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	%ymm4, 3264(%rsp)
.Ltmp2500:
	.loc	1 3418 10 is_stmt 1
	vmovaps	3264(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
.Ltmp2501:
	.loc	1 3421 23
	movq	1736(%rbx), %rdx
.Ltmp2502:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_270
.Ltmp2503:
	.loc	5 0 16 is_stmt 0
	movq	%r10, %r14
	.loc	1 3421 23 is_stmt 1
	movq	1728(%rbx), %rax
.Ltmp2504:
	.loc	11 551 14
	vmovaps	576(%rsp), %ymm0
	vmovups	%ymm0, (%rax)
.Ltmp2505:
	.loc	1 3422 5
	movq	1768(%rbx), %rax
.Ltmp2506:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp2507:
	.loc	6 180 28
	je	.LBB1_328
.Ltmp2508:
	.loc	6 0 28 is_stmt 0
	movq	1760(%rbx), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB1_327:
.Ltmp2509:
	.loc	20 66 21 is_stmt 1
	movl	%r11d, (%rcx,%rdx)
.Ltmp2510:
	.loc	7 1714 9
	addq	$4, %rdx
	cmpq	%rdx, %rax
.Ltmp2511:
	.loc	6 180 28
	jne	.LBB1_327
.Ltmp2512:
.LBB1_328:
	.loc	6 0 28 is_stmt 0
	leaq	1376(%rsp), %rdi
	movq	16(%rsp), %rsi
	.loc	1 3424 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	.loc	1 3425 5
	movl	%r14d, 1640(%rbx)
	jmp	.LBB1_268
.Ltmp2513:
.LBB1_329:
	.loc	1 0 5 is_stmt 0
	vmovaps	128(%rsp), %ymm0
.Ltmp2514:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2515:
	.loc	5 569 13
	leaq	.Lalloc_fc26f793d85338b5649d38df0c19e7e0(%rip), %rcx
	movq	%rax, %rdi
.Ltmp2516:
	.loc	1 0 0 is_stmt 0
	movq	%r15, %rsi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_330:
	vmovaps	128(%rsp), %ymm0
.Ltmp2517:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2518:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_beae6f21d5ab5b7a10c8cf24995b1244(%rip), %rcx
	movq	%r15, %rsi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_331:
	vmovaps	128(%rsp), %ymm0
.Ltmp2519:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2520:
	.loc	5 581 13
	leaq	.Lalloc_913d17a5751fc2956adecdab98dac09f(%rip), %rcx
	movq	320(%rsp), %rdi
.Ltmp2521:
	.loc	5 581 13 is_stmt 0
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2522:
.LBB1_332:
	.loc	5 0 13
	vmovaps	128(%rsp), %ymm0
.Ltmp2523:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2524:
	.loc	5 581 13
	leaq	.Lalloc_8a0dcf875eae79f6708bcf79c3cbff55(%rip), %rcx
	movq	320(%rsp), %rdi
.Ltmp2525:
	.loc	5 581 13 is_stmt 0
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2526:
.LBB1_333:
	.loc	5 0 13
	vmovaps	128(%rsp), %ymm0
.Ltmp2527:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2528:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_e5e0b8406fbb9ac3f5f26ac6469c25f7(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_334:
	movq	%r8, %r12
.Ltmp2529:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
	vmovaps	%ymm4, 1408(%rsp)
	vmovaps	1248(%rsp), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	1280(%rsp), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	1216(%rsp), %ymm13
.Ltmp2530:
.LBB1_335:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 1376(%rsp)
	leaq	1376(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp2531:
	.loc	1 3313 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	.loc	1 3314 5
	movl	%r12d, 1640(%rbx)
	movq	96(%rsp), %rax
.Ltmp2532:
	.loc	1 0 0 is_stmt 0
	movl	%eax, 1644(%rbx)
.Ltmp2533:
	.loc	1 2334 13 is_stmt 1
	cmpb	$0, 764(%rsp)
	jne	.LBB1_337
.LBB1_345:
	.loc	1 0 13 is_stmt 0
	xorl	%eax, %eax
.LBB1_346:
	.loc	1 2333 9 is_stmt 1
	movb	%al, 2152(%rbx)
	.loc	1 2335 30
	movzbl	2144(%rbx), %eax
	.loc	1 2335 9 is_stmt 0
	movb	%al, 2153(%rbx)
.Ltmp2534:
	.loc	29 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
	movabsq	$2305843009213693951, %rax
.Ltmp2535:
	.loc	15 1851 23
	addq	$-7, %rax
.Ltmp2536:
	.loc	19 2155 12
	andq	%r15, %rax
	je	.LBB1_352
.Ltmp2537:
	.loc	19 0 12 is_stmt 0
	vbroadcastss	.LCPI1_0(%rip), %ymm1
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI1_6(%rip), %ymm2
	vmovaps	%ymm0, %ymm3
	.p2align	4
.LBB1_348:
.Ltmp2538:
	.loc	29 82 19 is_stmt 1
	vandps	(%r13,%rcx,4), %ymm1, %ymm4
.Ltmp2539:
	.loc	29 871 14
	vcmplt_oqps	%ymm2, %ymm4, %ymm4
.Ltmp2540:
	.loc	29 82 19
	vandps	%ymm4, %ymm3, %ymm3
.Ltmp2541:
	.loc	19 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB1_348
.Ltmp2542:
	.file	30 "/home/bl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wide-1.6.1" "src/f32x8_.rs"
	.loc	30 176 9
	vpcmpeqd	%ymm4, %ymm4, %ymm4
	vtestps	%ymm4, %ymm3
.Ltmp2543:
	.loc	1 2336 12
	jb	.LBB1_356
	.loc	1 0 12 is_stmt 0
	xorl	%ecx, %ecx
	.p2align	4
.LBB1_351:
.Ltmp2544:
	.loc	29 82 19 is_stmt 1
	vandps	(%r13,%rcx,4), %ymm1, %ymm3
.Ltmp2545:
	.loc	29 871 14
	vcmplt_oqps	%ymm2, %ymm3, %ymm3
.Ltmp2546:
	.loc	29 82 19
	vandps	%ymm3, %ymm0, %ymm0
.Ltmp2547:
	.loc	19 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB1_351
	jmp	.LBB1_353
.Ltmp2548:
.LBB1_352:
	.loc	30 176 9
	vpcmpeqd	%ymm1, %ymm1, %ymm1
	vtestps	%ymm1, %ymm0
.Ltmp2549:
	.loc	1 2336 12
	jb	.LBB1_356
.LBB1_353:
.Ltmp2550:
	.loc	29 585 19
	vpbroadcastd	.LCPI1_2(%rip), %ymm1
	vpsrad	$31, %ymm0, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
.Ltmp2551:
	.loc	17 185 12
	vmovd	%xmm0, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	vpextrd	$1, %xmm0, %eax
	xorl	%edx, %edx
	testl	%eax, %eax
	setne	%dl
	leal	(%rcx,%rdx,2), %eax
	vpextrd	$2, %xmm0, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	leal	(%rax,%rdx,4), %eax
	vpextrd	$3, %xmm0, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	leal	(%rax,%rdx,8), %ecx
	vextracti128	$1, %ymm0, %xmm0
	vmovd	%xmm0, %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	shll	$4, %eax
	vpextrd	$1, %xmm0, %edx
	orl	%ecx, %eax
	xorl	%ecx, %ecx
	testl	%edx, %edx
	setne	%cl
	shll	$5, %ecx
	vpextrd	$2, %xmm0, %edx
	xorl	%esi, %esi
	testl	%edx, %edx
	setne	%sil
	shll	$6, %esi
	orl	%ecx, %esi
	vpextrd	$3, %xmm0, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	shll	$7, %edx
	orl	%esi, %edx
	orl	%eax, %edx
.Ltmp2552:
	.loc	1 2339 9
	movl	%edx, 1576(%rbx)
	.loc	1 2340 40
	movq	1568(%rbx), %rax
.Ltmp2553:
	.loc	4 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp2554:
	.loc	1 2340 9
	movq	%rcx, 1568(%rbx)
.Ltmp2555:
	.loc	7 1714 9
	testq	%r15, %r15
.Ltmp2556:
	.loc	6 180 28
	je	.LBB1_355
.Ltmp2557:
	.loc	16 961 18
	shlq	$2, %r15
.Ltmp2558:
	.loc	20 25 13
	movq	%r13, %rdi
	xorl	%esi, %esi
	movq	%r15, %rdx
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp2559:
.LBB1_355:
	.loc	20 0 13 is_stmt 0
	movq	3216(%rsp), %rcx
	.loc	1 2342 21 is_stmt 1
	movq	16(%rcx), %rax
	movq	%rax, 4656(%rsp)
	vmovups	(%rcx), %xmm0
	vmovaps	%xmm0, 4640(%rsp)
	movq	%rbx, %r12
.Ltmp2560:
	.loc	1 2343 20
	movl	2120(%rbx), %ebx
.Ltmp2561:
	.loc	1 2345 40
	movq	1584(%r12), %rdx
	movq	1592(%r12), %rcx
	.loc	1 2345 14 is_stmt 0
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %r14
	leaq	4640(%rsp), %r15
	movq	16(%rsp), %rdi
	movq	%r15, %rsi
	movl	%ebx, %r8d
	vzeroupper
	callq	*%r14
	.loc	1 2347 40 is_stmt 1
	movq	1600(%r12), %rdx
	movq	1608(%r12), %rcx
	movq	208(%rsp), %rdi
	.loc	1 2347 14 is_stmt 0
	movq	%r15, %rsi
	movl	%ebx, %r8d
	callq	*%r14
	.loc	1 2348 9 is_stmt 1
	movq	$0, 1640(%r12)
.Ltmp2562:
.LBB1_356:
	.loc	1 0 9 is_stmt 0
	leaq	4288(%rsp), %rsi
	.loc	1 3157 9 is_stmt 1
	movl	$328, %edx
	movq	3224(%rsp), %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp2563:
	.loc	1 3158 6
	leaq	-40(%rbp), %rsp
	.loc	1 3158 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB1_357:
	.cfi_def_cfa %rbp, 16
.Ltmp2564:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2565:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2566:
.LBB1_358:
	.loc	1 455 21 is_stmt 1
	xorl	%edx, %edx
	divl	%esi
.LBB1_359:
	.loc	1 455 9 is_stmt 0
	movl	%edx, 1640(%rbx)
	.loc	1 456 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB1_437
	.loc	1 456 23 is_stmt 0
	movl	1644(%rbx), %esi
	movq	8(%rsp), %rdx
	.loc	1 456 44
	cmpq	%rcx, %rdx
	jb	.LBB1_362
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ecx
.LBB1_362:
	.loc	1 456 22
	addq	%rsi, %rdx
	.loc	1 456 21
	movq	%rdx, %rax
	orq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB1_364
	movq	%rdx, %rax
	xorl	%edx, %edx
	divq	%rcx
	.loc	1 456 9
	movl	%edx, 1644(%rbx)
.Ltmp2567:
	.loc	1 0 0
	jmp	.LBB1_356
.LBB1_364:
.Ltmp2568:
	.loc	1 456 21
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ecx
	.loc	1 456 9
	movl	%edx, 1644(%rbx)
.Ltmp2569:
	.loc	1 0 0
	jmp	.LBB1_356
.LBB1_365:
	vmovaps	448(%rsp), %ymm0
.Ltmp2570:
	.loc	1 1656 5 is_stmt 1
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2571:
	.loc	1 853 0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp2572:
	.loc	1 855 0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	384(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp2573:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2574:
.LBB1_366:
	.loc	5 443 13
	leaq	.Lalloc_cdadbf7cf6c6abfb3682542d463ff330(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2575:
.LBB1_367:
	.loc	5 0 13 is_stmt 0
	vmovaps	448(%rsp), %ymm0
.Ltmp2576:
	.loc	1 1656 5 is_stmt 1
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2577:
	.loc	1 853 0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp2578:
	.loc	1 855 0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	384(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp2579:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%r8), %rsi
.Ltmp2580:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r8, %rdi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2581:
.LBB1_368:
	.loc	5 0 13 is_stmt 0
	vmovaps	448(%rsp), %ymm0
.Ltmp2582:
	.loc	1 1656 5 is_stmt 1
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2583:
	.loc	1 853 0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp2584:
	.loc	1 855 0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	384(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp2585:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2586:
.LBB1_369:
	.loc	5 0 13 is_stmt 0
	vmovaps	448(%rsp), %ymm0
	.loc	1 1656 5 is_stmt 1
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2587:
	.loc	1 853 0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp2588:
	.loc	1 855 0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	384(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp2589:
	.loc	1 1133 25 is_stmt 1
	leaq	8(%r8), %rsi
.Ltmp2590:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r8, %rdi
	movq	864(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2591:
.LBB1_370:
	.loc	5 0 13 is_stmt 0
	vmovaps	%ymm14, 3264(%rsp)
	movq	216(%rsp), %r13
	jmp	.LBB1_263
.LBB1_371:
	vmovaps	128(%rsp), %ymm0
.Ltmp2592:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2593:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_b22b5c926aed02a79660ac772e5ad40d(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_372:
	vmovaps	928(%rsp), %ymm0
.Ltmp2594:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	128(%rsp), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp2595:
.LBB1_373:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_fc26f793d85338b5649d38df0c19e7e0(%rip), %rcx
	movq	%r15, %rsi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_374:
	vmovaps	928(%rsp), %ymm0
.Ltmp2596:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	128(%rsp), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp2597:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_375:
.Ltmp2598:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2599:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_beae6f21d5ab5b7a10c8cf24995b1244(%rip), %rcx
	movq	%r15, %rsi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_376:
.Ltmp2600:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2601:
	.loc	5 581 13
	leaq	.Lalloc_913d17a5751fc2956adecdab98dac09f(%rip), %rcx
	movq	%r10, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2602:
.LBB1_377:
	.loc	1 1767 5
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2603:
	.loc	5 581 13
	leaq	.Lalloc_8a0dcf875eae79f6708bcf79c3cbff55(%rip), %rcx
	movq	%r11, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2604:
.LBB1_378:
	.loc	1 1767 5
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2605:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_e5e0b8406fbb9ac3f5f26ac6469c25f7(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_379:
	vmovaps	32(%rsp), %ymm0
.Ltmp2606:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	256(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	288(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	%ymm9, 1728(%rsp)
.Ltmp2607:
	.loc	5 456 13
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2608:
.LBB1_380:
	.loc	5 0 13 is_stmt 0
	vmovaps	32(%rsp), %ymm0
.Ltmp2609:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	256(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	288(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	%ymm9, 1728(%rsp)
.Ltmp2610:
	.loc	1 0 0 is_stmt 0
	leaq	8(%r8), %rsi
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_381:
	vmovaps	32(%rsp), %ymm0
.Ltmp2611:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	256(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	288(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	%ymm9, 1728(%rsp)
.Ltmp2612:
	.loc	5 456 13
	leaq	.Lalloc_8421dc8ec9e43c41e5b11981eccec9a3(%rip), %rcx
	movq	1120(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2613:
.LBB1_382:
	.loc	5 0 13 is_stmt 0
	vmovaps	32(%rsp), %ymm0
.Ltmp2614:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	256(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	288(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	%ymm9, 1728(%rsp)
.Ltmp2615:
	.loc	1 1133 25
	leaq	8(%r9), %rsi
.Ltmp2616:
	.loc	5 443 13
	leaq	.Lalloc_6ed527a45f08d183090a42adb6b2bde3(%rip), %rcx
	movq	%r9, %rdi
	movq	864(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2617:
.LBB1_383:
	.loc	1 1767 5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	%ymm15, 1632(%rsp)
	vmovaps	%ymm6, 1664(%rsp)
	vmovaps	%ymm7, 1696(%rsp)
	vmovaps	%ymm9, 1728(%rsp)
	vmovaps	%ymm10, 1440(%rsp)
	vmovaps	%ymm11, 1472(%rsp)
	vmovaps	%ymm12, 1504(%rsp)
	vmovaps	%ymm1, 1536(%rsp)
	vmovaps	%ymm3, 1568(%rsp)
	vmovaps	2240(%rsp), %ymm0
.Ltmp2618:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1408(%rsp)
	movq	216(%rsp), %r13
	vmovaps	96(%rsp), %ymm0
	jmp	.LBB1_324
.Ltmp2619:
.LBB1_384:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2620:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_b22b5c926aed02a79660ac772e5ad40d(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_385:
	vmovaps	4256(%rsp), %ymm0
.Ltmp2621:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	4224(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	4192(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	4160(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	4128(%rsp), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
.Ltmp2622:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_fc26f793d85338b5649d38df0c19e7e0(%rip), %rcx
	movq	%r15, %rsi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_386:
	vmovaps	4256(%rsp), %ymm0
.Ltmp2623:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	4224(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	4192(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	4160(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	4128(%rsp), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
.Ltmp2624:
.LBB1_387:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_388:
	vmovaps	448(%rsp), %ymm0
.Ltmp2625:
	.loc	1 1656 5 is_stmt 1
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2626:
	.loc	1 853 0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp2627:
	.loc	1 855 0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	384(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1952(%rsp)
.Ltmp2628:
	leaq	.Lalloc_2cc46ad3620fe9ec5bf0f181ee950489(%rip), %rcx
	movq	88(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_389:
	vmovaps	128(%rsp), %ymm0
.Ltmp2629:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2630:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_8013bf8450ffb218032f1e27d334efa7(%rip), %rdx
	movq	224(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_390:
	vmovaps	128(%rsp), %ymm0
.Ltmp2631:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2632:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_2d2a28b8cb03afaaebfea48ffa77c925(%rip), %rdx
	movq	32(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_391:
	vmovaps	128(%rsp), %ymm0
.Ltmp2633:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2634:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_d297cbdfce2474defb1d7f11394e8cb6(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_392:
	vmovaps	128(%rsp), %ymm0
.Ltmp2635:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2636:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_ac016620dad255f0d323022a5b7f5883(%rip), %rdx
	movq	544(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_393:
	vmovaps	128(%rsp), %ymm0
.Ltmp2637:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2638:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_e5355958980a78279123102c3494b10a(%rip), %rdx
	movq	%r11, %rdi
	movq	32(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_394:
	vmovaps	128(%rsp), %ymm0
.Ltmp2639:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2640:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_53d3a5c3ecf31ea5f14c0a7f8bf40921(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_395:
	vmovaps	32(%rsp), %ymm0
.Ltmp2641:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	256(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	288(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	%ymm9, 1728(%rsp)
.Ltmp2642:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_2cc46ad3620fe9ec5bf0f181ee950489(%rip), %rcx
	movq	88(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2643:
.LBB1_396:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_b63e7acf20e2df0969a284a5c2277b6c(%rip), %rcx
	xorl	%edi, %edi
	movq	%r11, %rsi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2644:
.LBB1_397:
	.loc	5 443 13
	leaq	.Lalloc_dceebfa5c8edf99352a4c26a81a69889(%rip), %rcx
	xorl	%edi, %edi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2645:
.LBB1_398:
	.loc	1 1767 5
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2646:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_8013bf8450ffb218032f1e27d334efa7(%rip), %rdx
	movq	224(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_399:
.Ltmp2647:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2648:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_2d2a28b8cb03afaaebfea48ffa77c925(%rip), %rdx
	movq	32(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_400:
.Ltmp2649:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2650:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_d297cbdfce2474defb1d7f11394e8cb6(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_401:
.Ltmp2651:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2652:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_ac016620dad255f0d323022a5b7f5883(%rip), %rdx
	movq	544(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_402:
.Ltmp2653:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2654:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_e5355958980a78279123102c3494b10a(%rip), %rdx
	movq	%r11, %rdi
	movq	32(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_403:
	vmovaps	128(%rsp), %ymm0
.Ltmp2655:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	160(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
.Ltmp2656:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_7d7f2b4ff3f37cf08b84adcf6762221c(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_404:
.Ltmp2657:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2658:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_53d3a5c3ecf31ea5f14c0a7f8bf40921(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_405:
	vmovaps	128(%rsp), %ymm14
	vmovaps	160(%rsp), %ymm13
	jmp	.LBB1_430
.LBB1_406:
	vmovaps	128(%rsp), %ymm14
	vmovaps	160(%rsp), %ymm13
	movl	$1, %eax
	jmp	.LBB1_429
.LBB1_407:
.Ltmp2659:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm6, 1504(%rsp)
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2660:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_7d7f2b4ff3f37cf08b84adcf6762221c(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2661:
.LBB1_408:
	.loc	1 3138 25 is_stmt 1
	leaq	.Lalloc_8790d798703aa1c903d88be7cb1df9ea(%rip), %rdx
	movq	%rbx, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_409:
	.loc	1 0 25 is_stmt 0
	vmovaps	128(%rsp), %ymm14
	vmovaps	160(%rsp), %ymm13
	movl	$2, %eax
	jmp	.LBB1_429
.LBB1_410:
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm8, %ymm6
	jmp	.LBB1_435
.LBB1_411:
	vmovaps	128(%rsp), %ymm14
	vmovaps	160(%rsp), %ymm13
	movl	$3, %eax
	jmp	.LBB1_429
.LBB1_412:
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm8, %ymm6
	movl	$1, %eax
	jmp	.LBB1_434
.LBB1_413:
	vmovaps	128(%rsp), %ymm14
	vmovaps	160(%rsp), %ymm13
	movl	$4, %eax
	jmp	.LBB1_429
.LBB1_414:
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm8, %ymm6
	movl	$2, %eax
	jmp	.LBB1_434
.LBB1_415:
	movl	$1, %edi
.Ltmp2662:
	.loc	1 3139 23 is_stmt 1
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_416:
	.loc	1 0 23 is_stmt 0
	movl	$2, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_417:
	.loc	1 0 23
	movl	$3, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_418:
	.loc	1 0 23
	movl	$4, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_419:
	.loc	1 0 23
	movl	$5, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_420:
	.loc	1 0 23
	movl	$6, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_421:
	.loc	1 0 23
	movl	$7, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_422:
	.loc	1 0 23
	movl	$8, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2663:
.LBB1_423:
	.loc	1 0 23
	vmovaps	128(%rsp), %ymm14
	vmovaps	160(%rsp), %ymm13
	movl	$5, %eax
	jmp	.LBB1_429
.LBB1_424:
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm8, %ymm6
	movl	$3, %eax
	jmp	.LBB1_434
.LBB1_425:
.Ltmp2664:
	.loc	1 665 42 is_stmt 1
	leaq	.Lalloc_bac57976a2bdbfad4a3a85d5d1c7648c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp2665:
.LBB1_426:
	.loc	1 0 42 is_stmt 0
	vmovaps	128(%rsp), %ymm14
	vmovaps	160(%rsp), %ymm13
	movl	$6, %eax
	jmp	.LBB1_429
.LBB1_427:
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm8, %ymm6
	movl	$4, %eax
	jmp	.LBB1_434
.LBB1_428:
	vmovaps	128(%rsp), %ymm14
	vmovaps	160(%rsp), %ymm13
	movl	$7, %eax
.LBB1_429:
	movq	%rax, 608(%rsp)
.LBB1_430:
.Ltmp2666:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm14, 3296(%rsp)
	vmovaps	%ymm13, 3328(%rsp)
.Ltmp2667:
	.loc	1 1405 42
	leaq	.Lalloc_7ff2ed40a8d2df224a47a35441e5e2fe(%rip), %rdx
	movq	608(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2668:
.LBB1_431:
	.loc	1 0 42 is_stmt 0
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm8, %ymm6
	movl	$5, %eax
	jmp	.LBB1_434
.LBB1_432:
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm8, %ymm6
	movl	$6, %eax
	jmp	.LBB1_434
.LBB1_433:
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm8, %ymm6
	movl	$7, %eax
.LBB1_434:
	movq	%rax, 640(%rsp)
.LBB1_435:
.Ltmp2669:
	.loc	1 1767 5 is_stmt 1
	vmovaps	%ymm7, 1504(%rsp)
	vmovaps	%ymm6, 1536(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
.Ltmp2670:
	.loc	1 1405 42
	leaq	.Lalloc_7ff2ed40a8d2df224a47a35441e5e2fe(%rip), %rdx
	movq	640(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2671:
.LBB1_436:
	.loc	1 455 44
	leaq	.Lalloc_f0ee36f67d9a332211aa5518dd2ebfd5(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB1_437:
	.loc	1 456 44
	leaq	.Lalloc_33d4d33e0a850133578789055882dcf9(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp2672:
.Lfunc_end1:
	.size	_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_, .Lfunc_end1-_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_
