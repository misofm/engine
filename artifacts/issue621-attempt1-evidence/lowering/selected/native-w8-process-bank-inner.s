_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_:
.Lfunc_begin1:
	.loc	1 3113 0
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
	subq	$1312, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, 2968(%rsp)
.Ltmp1308:
	.loc	1 3126 13 prologue_end
	movq	40(%rdx), %rax
	movq	%rax, 128(%rsp)
	testq	%rax, %rax
	je	.LBB1_2
	.loc	1 3127 13
	movb	$0, 2152(%rsi)
.LBB1_2:
	.loc	1 0 0 is_stmt 0
	movq	32(%rdx), %rax
	movq	%rax, 32(%rsp)
	movq	%rsi, 88(%rsp)
	.loc	1 3129 51 is_stmt 1
	movzbl	2288(%rsi), %eax
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 4256(%rsp)
	vmovaps	%ymm0, 4224(%rsp)
	vmovaps	%ymm0, 4192(%rsp)
	vmovaps	%ymm0, 4160(%rsp)
	vmovaps	%ymm0, 4128(%rsp)
	vmovaps	%ymm0, 4096(%rsp)
	vmovaps	%ymm0, 4064(%rsp)
	vmovaps	%ymm0, 4032(%rsp)
	vmovaps	%ymm0, 4000(%rsp)
	vmovaps	%ymm0, 3968(%rsp)
.Ltmp1309:
	.loc	2 1050 9
	movb	%al, 4288(%rsp)
	movq	%rdx, 416(%rsp)
	movq	56(%rdx), %rbx
.Ltmp1310:
	.loc	3 900 12
	cmpq	$1, %rbx
	movq	%rbx, %r12
	adcq	$-1, %r12
.Ltmp1311:
	.loc	1 3131 25
	testq	%rbx, %rbx
	je	.LBB1_401
.Ltmp1312:
	.loc	1 3132 23
	cmpq	$1, %rbx
	je	.LBB1_406
	.loc	1 0 23 is_stmt 0
	movq	416(%rsp), %rcx
	movq	48(%rcx), %r15
	movl	(%r15), %edi
	.loc	1 3132 23
	movl	4(%r15), %esi
.Ltmp1313:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	128(%rsp), %rdx
	jb	.LBB1_336
	cmpq	%rsi, %rdx
	jb	.LBB1_336
	.loc	4 0 16 is_stmt 0
	movq	88(%rsp), %rax
	leaq	1848(%rax), %r9
	leaq	1648(%rax), %r8
	movq	96(%rcx), %r13
	leaq	2048(%rax), %rdx
	.loc	4 1054 31 is_stmt 1
	subq	%rdi, %rsi
.Ltmp1314:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
	leaq	3968(%rsp), %rax
	movq	%rdx, %r14
.Ltmp1315:
	.loc	1 3133 13
	movq	%r13, %rcx
	movq	%r8, 16(%rsp)
	movq	%r9, 184(%rsp)
	pushq	%rax
	pushq	$0
	vzeroupper
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1316:
	.loc	1 3132 23
	cmpq	$1, %r12
	je	.LBB1_407
	.loc	1 0 23 is_stmt 0
	movl	4(%r15), %edi
	.loc	1 3132 23
	movl	8(%r15), %esi
.Ltmp1317:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	128(%rsp), %rdx
	jb	.LBB1_336
	cmpq	%rsi, %rdx
	jb	.LBB1_336
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1318:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1319:
	.loc	1 3140 17
	leaq	4008(%rsp), %rax
	.loc	1 3133 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	184(%rsp), %r9
	pushq	%rax
	pushq	$1
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1320:
	.loc	1 3131 25
	cmpq	$2, %rbx
	je	.LBB1_401
.Ltmp1321:
	.loc	1 3132 23
	cmpq	$2, %r12
	je	.LBB1_408
	.loc	1 0 23 is_stmt 0
	movl	8(%r15), %edi
	.loc	1 3132 23
	movl	12(%r15), %esi
.Ltmp1322:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	128(%rsp), %rdx
	jb	.LBB1_336
	cmpq	%rsi, %rdx
	jb	.LBB1_336
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1323:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1324:
	.loc	1 3140 17
	leaq	4048(%rsp), %rax
	.loc	1 3133 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	184(%rsp), %r9
	pushq	%rax
	pushq	$2
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1325:
	.loc	1 3131 25
	cmpq	$3, %rbx
	je	.LBB1_401
.Ltmp1326:
	.loc	1 3132 23
	cmpq	$3, %r12
	je	.LBB1_409
	.loc	1 0 23 is_stmt 0
	movl	12(%r15), %edi
	.loc	1 3132 23
	movl	16(%r15), %esi
.Ltmp1327:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	128(%rsp), %rdx
	jb	.LBB1_336
	cmpq	%rsi, %rdx
	jb	.LBB1_336
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1328:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1329:
	.loc	1 3140 17
	leaq	4088(%rsp), %rax
	.loc	1 3133 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	184(%rsp), %r9
	pushq	%rax
	pushq	$3
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1330:
	.loc	1 3131 25
	cmpq	$4, %rbx
	je	.LBB1_401
.Ltmp1331:
	.loc	1 3132 23
	cmpq	$4, %r12
	je	.LBB1_410
	.loc	1 0 23 is_stmt 0
	movl	16(%r15), %edi
	.loc	1 3132 23
	movl	20(%r15), %esi
.Ltmp1332:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	128(%rsp), %rdx
	jb	.LBB1_336
	cmpq	%rsi, %rdx
	jb	.LBB1_336
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1333:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1334:
	.loc	1 3140 17
	leaq	4128(%rsp), %rax
	.loc	1 3133 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	184(%rsp), %r9
	pushq	%rax
	pushq	$4
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1335:
	.loc	1 3131 25
	cmpq	$5, %rbx
	je	.LBB1_401
.Ltmp1336:
	.loc	1 3132 23
	cmpq	$5, %r12
	je	.LBB1_411
	.loc	1 0 23 is_stmt 0
	movl	20(%r15), %edi
	.loc	1 3132 23
	movl	24(%r15), %esi
.Ltmp1337:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	128(%rsp), %rdx
	jb	.LBB1_336
	cmpq	%rsi, %rdx
	jb	.LBB1_336
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1338:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1339:
	.loc	1 3140 17
	leaq	4168(%rsp), %rax
	.loc	1 3133 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	184(%rsp), %r9
	pushq	%rax
	pushq	$5
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1340:
	.loc	1 3131 25
	cmpq	$6, %rbx
	je	.LBB1_401
.Ltmp1341:
	.loc	1 3132 23
	cmpq	$6, %r12
	je	.LBB1_412
	.loc	1 0 23 is_stmt 0
	movl	24(%r15), %edi
	.loc	1 3132 23
	movl	28(%r15), %esi
.Ltmp1342:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	128(%rsp), %rdx
	jb	.LBB1_336
	cmpq	%rsi, %rdx
	jb	.LBB1_336
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1343:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1344:
	.loc	1 3140 17
	leaq	4208(%rsp), %rax
	.loc	1 3133 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	184(%rsp), %r9
	pushq	%rax
	pushq	$6
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp1345:
	.loc	1 3131 25
	cmpq	$7, %rbx
	je	.LBB1_401
.Ltmp1346:
	.loc	1 3132 23
	cmpq	$7, %r12
	je	.LBB1_413
	.loc	1 0 23 is_stmt 0
	movl	28(%r15), %edi
	.loc	1 3132 23
	movl	32(%r15), %esi
.Ltmp1347:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	128(%rsp), %rdx
	jb	.LBB1_336
	cmpq	%rsi, %rdx
	jb	.LBB1_336
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp1348:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	32(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp1349:
	.loc	1 3140 17
	leaq	4248(%rsp), %rax
	.loc	1 3133 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	184(%rsp), %r9
	pushq	%rax
	pushq	$7
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
	movq	416(%rsp), %rax
.Ltmp1350:
	.loc	1 3145 37
	movq	(%rax), %r11
	movq	8(%rax), %r15
	.loc	1 3145 49 is_stmt 0
	movl	104(%rax), %eax
	movq	%rax, 8(%rsp)
.Ltmp1351:
	.loc	1 2304 21 is_stmt 1
	leaq	(,%rax,8), %rax
	movq	%rax, 2952(%rsp)
	movq	88(%rsp), %r13
.Ltmp1352:
	.loc	1 2305 21
	movzbl	2153(%r13), %ecx
	.loc	1 0 0 is_stmt 0
	movq	1784(%r13), %rax
	.loc	1 2305 21
	cmpb	2144(%r13), %cl
	movq	%r15, 80(%rsp)
	movq	%r11, 24(%rsp)
	jne	.LBB1_38
	.loc	1 2306 37 is_stmt 1
	movq	1776(%r13), %rcx
.Ltmp1353:
	.loc	6 314 17
	movq	%rax, %rdx
	shlq	$4, %rdx
	movq	%rcx, %rsi
	.loc	6 0 17 is_stmt 0
.Ltmp1354:
	.p2align	4
.LBB1_35:
.Ltmp1355:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp1356:
	.loc	6 180 28
	je	.LBB1_211
.Ltmp1357:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp1358:
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
.Ltmp1359:
.LBB1_38:
	.loc	1 715 26 is_stmt 1
	movq	1776(%r13), %rcx
.Ltmp1360:
.LBB1_39:
	.loc	1 0 26 is_stmt 0
	xorl	%r9d, %r9d
.LBB1_40:
	.loc	1 2322 13 is_stmt 1
	leaq	1616(%r13), %r8
.Ltmp1361:
	.loc	6 314 17
	shlq	$4, %rax
	movq	%r8, 2960(%rsp)
	movl	%r9d, 604(%rsp)
	.loc	6 0 17 is_stmt 0
.Ltmp1362:
	.p2align	4
.LBB1_41:
.Ltmp1363:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp1364:
	.loc	6 180 28
	je	.LBB1_50
.Ltmp1365:
	.loc	1 690 21
	cmpl	$0, 12(%rcx)
.Ltmp1366:
	.loc	6 315 25
	jne	.LBB1_44
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB1_41
.Ltmp1367:
.LBB1_44:
	.loc	1 1116 5 is_stmt 1
	movq	1832(%r13), %rcx
	testq	%rcx, %rcx
	je	.LBB1_136
	.loc	1 0 5 is_stmt 0
	movq	1824(%r13), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rdx
	.p2align	4
.LBB1_46:
.Ltmp1368:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp1369:
	.loc	6 180 28
	je	.LBB1_136
.Ltmp1370:
	.loc	1 390 34
	movl	(%rdx), %esi
	cmpl	(%rax), %esi
	jne	.LBB1_140
	movl	4(%rdx), %esi
	cmpl	4(%rax), %esi
	jne	.LBB1_140
	movl	8(%rdx), %esi
.Ltmp1371:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rdx
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp1372:
	.loc	1 390 34
	cmpl	8(%rax), %esi
.Ltmp1373:
	.loc	6 315 25
	je	.LBB1_46
	jmp	.LBB1_140
.Ltmp1374:
.LBB1_50:
	.loc	1 715 63
	movq	1792(%r13), %rcx
	movq	1800(%r13), %rdx
.Ltmp1375:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp1376:
	.p2align	4
.LBB1_51:
	movq	%rdx, %rax
.Ltmp1377:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp1378:
	.loc	6 180 28
	je	.LBB1_54
.Ltmp1379:
	.loc	1 690 21
	cmpl	$0, 12(%rcx)
.Ltmp1380:
	.loc	6 315 25
	jne	.LBB1_54
	leaq	-16(%rax), %rdx
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB1_51
.Ltmp1381:
.LBB1_54:
	.loc	1 1116 5 is_stmt 1
	movq	1832(%r13), %rdx
	testq	%rdx, %rdx
	je	.LBB1_60
	.loc	1 0 5 is_stmt 0
	movq	1824(%r13), %rcx
	shlq	$2, %rdx
	leaq	(%rdx,%rdx,2), %rdx
	movq	%rcx, %rsi
	.p2align	4
.LBB1_56:
.Ltmp1382:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp1383:
	.loc	6 180 28
	je	.LBB1_60
.Ltmp1384:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rcx), %edi
	jne	.LBB1_64
	movl	4(%rsi), %edi
	cmpl	4(%rcx), %edi
	jne	.LBB1_64
	movl	8(%rsi), %edi
.Ltmp1385:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rdx
.Ltmp1386:
	.loc	1 390 34
	cmpl	8(%rcx), %edi
.Ltmp1387:
	.loc	6 315 25
	je	.LBB1_56
	jmp	.LBB1_64
.Ltmp1388:
.LBB1_60:
	.loc	1 1117 12
	movq	1768(%r13), %rcx
	testq	%rcx, %rcx
	je	.LBB1_216
	.loc	1 0 12 is_stmt 0
	movq	1760(%r13), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB1_62:
.Ltmp1389:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rcx
.Ltmp1390:
	.loc	6 180 28
	je	.LBB1_216
.Ltmp1391:
	.loc	6 315 25
	movl	(%rdx,%rsi), %edi
	addq	$4, %rsi
.Ltmp1392:
	.loc	1 1117 43
	cmpl	(%rdx), %edi
.Ltmp1393:
	.loc	6 315 25
	je	.LBB1_62
.Ltmp1394:
.LBB1_64:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp1395:
	.loc	1 3197 12
	jne	.LBB1_140
	.loc	1 0 12 is_stmt 0
	leaq	992(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp1396:
	.loc	1 3247 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp1397:
	.loc	1 3252 19
	movzbl	1536(%r13), %r14d
.Ltmp1398:
	.loc	1 3253 21
	movzbl	1537(%r13), %ebx
.Ltmp1399:
	.loc	1 3254 27
	movl	1640(%r13), %r12d
.Ltmp1400:
	.loc	1 3255 27
	movl	1644(%r13), %eax
	movq	%rax, 32(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 3104(%rsp)
	leaq	4320(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
	movq	8(%rsp), %rax
.Ltmp1401:
	.loc	4 3758 16
	leaq	31(%rax), %rdx
	shrq	$5, %rdx
	vmovaps	992(%rsp), %ymm3
.Ltmp1402:
	.loc	8 446 20
	je	.LBB1_286
.Ltmp1403:
	.file	29 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs"
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm12
.Ltmp1404:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm12, %ymm0
	vmovaps	%ymm12, %ymm5
	testb	%r14b, %r14b
	jne	.LBB1_68
.Ltmp1405:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, %ymm5
.LBB1_68:
	vmovaps	%ymm3, 960(%rsp)
	testb	%bl, %bl
	movq	%r12, %r8
	jne	.LBB1_70
	vmovaps	%ymm0, %ymm12
.LBB1_70:
	vmovaps	1376(%rsp), %ymm7
	vmovaps	1504(%rsp), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	1696(%rsp), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	1024(%rsp), %ymm13
	vmovaps	1056(%rsp), %ymm0
	vmovaps	1088(%rsp), %ymm9
	vmovaps	1120(%rsp), %ymm6
	vmovaps	1152(%rsp), %ymm3
	vmovaps	1184(%rsp), %ymm2
	vmovaps	1216(%rsp), %ymm1
	vmovaps	1248(%rsp), %ymm15
	vmovaps	1280(%rsp), %ymm11
	vmovaps	1312(%rsp), %ymm14
	vmovaps	1344(%rsp), %ymm4
	movq	$0, 544(%rsp)
	movl	$32, %r9d
	movq	24(%rsp), %rbx
	movq	%rbx, %r10
	movq	8(%rsp), %r11
	xorl	%r14d, %r14d
	vmovaps	1664(%rsp), %ymm8
	vmovaps	%ymm8, 224(%rsp)
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	%ymm12, 928(%rsp)
	vmovaps	%ymm7, 896(%rsp)
.Ltmp1406:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB1_73
.Ltmp1407:
	.loc	8 0 20 is_stmt 0
.Ltmp1408:
	.p2align	4
.LBB1_71:
	vmovaps	%ymm10, %ymm3
.LBB1_72:
	addq	$32, %r14
	decq	%rdx
.Ltmp1409:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %r11
	addq	$1024, %r10
	testq	%rdx, %rdx
	vmovaps	768(%rsp), %ymm0
	vmovaps	288(%rsp), %ymm13
	je	.LBB1_301
.LBB1_73:
.Ltmp1410:
	.loc	4 2584 13
	cmpq	$1, %r11
	movq	%r11, %rax
	adcq	$0, %rax
	cmpq	$32, %rax
	cmovaeq	%r9, %rax
	movq	%rax, 1792(%rsp)
	movq	8(%rsp), %rsi
.Ltmp1411:
	.loc	1 3260 51
	subq	%r14, %rsi
.Ltmp1412:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%r9, %rsi
.Ltmp1413:
	.loc	1 3261 27
	leaq	(,%r14,8), %rdi
.Ltmp1414:
	.loc	1 3265 35
	addq	%r14, %rsi
	shlq	$3, %rsi
.Ltmp1415:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB1_378
	cmpq	%r15, %rsi
	ja	.LBB1_378
.Ltmp1416:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm0, 768(%rsp)
	vmovaps	%ymm9, %ymm8
.Ltmp1417:
	.loc	11 304 12 is_stmt 1
	cmpq	%r14, 8(%rsp)
	vmovaps	%ymm6, 448(%rsp)
	vmovaps	%ymm3, 384(%rsp)
	jne	.LBB1_77
	.loc	11 0 12 is_stmt 0
	vmovaps	960(%rsp), %ymm3
	vmovaps	%ymm8, %ymm12
	vmovaps	768(%rsp), %ymm6
	vmovaps	%ymm13, %ymm7
	.loc	11 304 12
	jmp	.LBB1_79
.Ltmp1418:
	.loc	11 0 12
.Ltmp1419:
	.p2align	4
.LBB1_77:
	movq	1792(%rsp), %rax
	shll	$5, %eax
	vmovaps	(%r13), %ymm0
	vmovaps	%ymm0, 416(%rsp)
	vmovaps	32(%r13), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	64(%r13), %ymm0
	vmovaps	%ymm0, 352(%rsp)
	vmovaps	96(%r13), %ymm0
	vmovaps	%ymm0, 736(%rsp)
	vmovaps	128(%r13), %ymm0
	vmovaps	%ymm0, 704(%rsp)
	vmovaps	160(%r13), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovaps	192(%r13), %ymm0
	vmovaps	%ymm0, 864(%rsp)
	vmovaps	224(%r13), %ymm0
	vmovaps	%ymm0, 832(%rsp)
	vmovaps	256(%r13), %ymm0
	vmovaps	%ymm0, 608(%rsp)
	vmovaps	288(%r13), %ymm0
	vmovaps	%ymm0, 480(%rsp)
	vmovaps	320(%r13), %ymm0
	vmovaps	%ymm0, 288(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm14, %ymm10
	vmovaps	%ymm11, %ymm9
	vmovaps	%ymm15, %ymm5
	vmovaps	%ymm1, %ymm0
	vmovaps	352(%r13), %ymm1
	vmovaps	%ymm1, 256(%rsp)
	vmovaps	384(%r13), %ymm1
	vmovaps	%ymm1, 192(%rsp)
	vmovaps	416(%r13), %ymm1
	vmovaps	%ymm1, 96(%rsp)
	vmovaps	448(%r13), %ymm1
	vmovaps	%ymm1, 640(%rsp)
	vmovaps	480(%r13), %ymm1
	vmovaps	%ymm1, 800(%rsp)
	vmovaps	512(%r13), %ymm1
	vmovaps	%ymm1, 2912(%rsp)
	vmovaps	544(%r13), %ymm1
	vmovaps	%ymm1, 2880(%rsp)
	vmovaps	576(%r13), %ymm1
	vmovaps	%ymm1, 2848(%rsp)
	vmovaps	608(%r13), %ymm1
	vmovaps	%ymm1, 2816(%rsp)
	vmovaps	640(%r13), %ymm1
	vmovaps	%ymm1, 2784(%rsp)
	vmovaps	672(%r13), %ymm1
	vmovaps	%ymm1, 2752(%rsp)
	vmovaps	704(%r13), %ymm1
	vmovaps	%ymm1, 2720(%rsp)
	vmovaps	736(%r13), %ymm1
	vmovaps	%ymm1, 2688(%rsp)
	vmovaps	768(%r13), %ymm1
	vmovaps	%ymm1, 2656(%rsp)
	vmovaps	800(%r13), %ymm1
	vmovaps	%ymm1, 2624(%rsp)
	vmovaps	832(%r13), %ymm1
	vmovaps	%ymm1, 2592(%rsp)
	vmovaps	864(%r13), %ymm1
	vmovaps	%ymm1, 2560(%rsp)
	vmovaps	896(%r13), %ymm1
	vmovaps	%ymm1, 2528(%rsp)
	vmovaps	928(%r13), %ymm1
	vmovaps	%ymm1, 2496(%rsp)
	vmovaps	960(%r13), %ymm1
	vmovaps	%ymm1, 2464(%rsp)
	vmovaps	992(%r13), %ymm1
	vmovaps	%ymm1, 2432(%rsp)
	vmovaps	1024(%r13), %ymm1
	vmovaps	%ymm1, 2400(%rsp)
	vmovaps	1056(%r13), %ymm1
	vmovaps	%ymm1, 2368(%rsp)
	vmovaps	1088(%r13), %ymm1
	vmovaps	%ymm1, 2336(%rsp)
	vmovaps	1120(%r13), %ymm1
	vmovaps	%ymm1, 2304(%rsp)
	vmovaps	1152(%r13), %ymm1
	vmovaps	%ymm1, 2272(%rsp)
	vmovaps	1184(%r13), %ymm1
	vmovaps	%ymm1, 2240(%rsp)
	vmovaps	1216(%r13), %ymm1
	vmovaps	%ymm1, 2208(%rsp)
	vmovaps	1248(%r13), %ymm1
	vmovaps	%ymm1, 2176(%rsp)
	vmovaps	1280(%r13), %ymm1
	vmovaps	%ymm1, 2144(%rsp)
	vmovaps	1312(%r13), %ymm1
	vmovaps	%ymm1, 2112(%rsp)
	vmovaps	1344(%r13), %ymm1
	vmovaps	%ymm1, 2080(%rsp)
	vmovaps	1376(%r13), %ymm1
	vmovaps	%ymm1, 2048(%rsp)
	vmovaps	1408(%r13), %ymm1
	vmovaps	%ymm1, 2016(%rsp)
	vmovaps	1440(%r13), %ymm1
	vmovaps	%ymm1, 1984(%rsp)
	vmovaps	1472(%r13), %ymm1
	vmovaps	%ymm1, 1952(%rsp)
	vmovaps	1504(%r13), %ymm1
	vmovaps	%ymm1, 1920(%rsp)
	vmovaps	960(%rsp), %ymm3
	vmovaps	%ymm8, %ymm12
	vmovaps	768(%rsp), %ymm6
	.p2align	4
.LBB1_78:
	vmovaps	%ymm10, 128(%rsp)
	vmovaps	%ymm3, %ymm7
.Ltmp1420:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r10,%rcx), %ymm3
.Ltmp1421:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm3, %ymm1
	vxorps	%xmm8, %xmm8, %xmm8
.Ltmp1422:
	.loc	29 48 14
	vaddps	%ymm1, %ymm8, %ymm1
.Ltmp1423:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm3, %ymm4
.Ltmp1424:
	.loc	29 48 14
	vaddps	%ymm4, %ymm8, %ymm4
.Ltmp1425:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm3, %ymm11
.Ltmp1426:
	.loc	29 48 14
	vaddps	%ymm8, %ymm11, %ymm11
.Ltmp1427:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm7, %ymm14
.Ltmp1428:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm14
.Ltmp1429:
	.loc	29 283 14
	vmulps	672(%rsp), %ymm7, %ymm1
.Ltmp1430:
	.loc	29 48 14
	vaddps	%ymm4, %ymm1, %ymm4
.Ltmp1431:
	.loc	29 283 14
	vmulps	864(%rsp), %ymm7, %ymm1
.Ltmp1432:
	.loc	29 48 14
	vaddps	%ymm1, %ymm11, %ymm11
.Ltmp1433:
	.loc	29 283 14
	vmulps	736(%rsp), %ymm3, %ymm1
.Ltmp1434:
	.loc	29 48 14
	vaddps	%ymm1, %ymm8, %ymm1
.Ltmp1435:
	.loc	29 283 14
	vmulps	832(%rsp), %ymm7, %ymm15
.Ltmp1436:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm15
	vmovaps	%ymm2, %ymm10
	vmovaps	384(%rsp), %ymm2
	vmovaps	448(%rsp), %ymm1
	vmovaps	%ymm12, %ymm8
	vmovaps	%ymm6, %ymm12
	vmovaps	%ymm13, %ymm6
.Ltmp1437:
	.loc	29 283 14
	vmulps	608(%rsp), %ymm13, %ymm13
.Ltmp1438:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp1439:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm6, %ymm14
.Ltmp1440:
	.loc	29 48 14
	vaddps	%ymm4, %ymm14, %ymm4
.Ltmp1441:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm6, %ymm14
.Ltmp1442:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp1443:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm6, %ymm14
.Ltmp1444:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp1445:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm12, %ymm15
.Ltmp1446:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1447:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm12, %ymm15
.Ltmp1448:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp1449:
	.loc	29 283 14
	vmulps	640(%rsp), %ymm12, %ymm15
.Ltmp1450:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
.Ltmp1451:
	.loc	29 283 14
	vmulps	800(%rsp), %ymm12, %ymm15
.Ltmp1452:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1453:
	.loc	29 283 14
	vmulps	2912(%rsp), %ymm8, %ymm15
.Ltmp1454:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1455:
	.loc	29 283 14
	vmulps	2880(%rsp), %ymm8, %ymm15
.Ltmp1456:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp1457:
	.loc	29 283 14
	vmulps	2848(%rsp), %ymm8, %ymm15
.Ltmp1458:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
	vmovaps	%ymm8, 448(%rsp)
.Ltmp1459:
	.loc	29 283 14
	vmulps	2816(%rsp), %ymm8, %ymm15
.Ltmp1460:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1461:
	.loc	29 283 14
	vmulps	2784(%rsp), %ymm1, %ymm15
.Ltmp1462:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1463:
	.loc	29 283 14
	vmulps	2752(%rsp), %ymm1, %ymm15
.Ltmp1464:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp1465:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm1, %ymm15
.Ltmp1466:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
	vmovaps	%ymm1, 384(%rsp)
.Ltmp1467:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm1, %ymm15
	vmovaps	%ymm10, %ymm1
.Ltmp1468:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1469:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm2, %ymm15
.Ltmp1470:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1471:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm2, %ymm15
.Ltmp1472:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp1473:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm2, %ymm15
.Ltmp1474:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
.Ltmp1475:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm2, %ymm15
.Ltmp1476:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1477:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm10, %ymm15
.Ltmp1478:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp1479:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm10, %ymm15
.Ltmp1480:
	.loc	29 48 14
	vaddps	%ymm4, %ymm15, %ymm4
.Ltmp1481:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm10, %ymm15
.Ltmp1482:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
.Ltmp1483:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm10, %ymm15
.Ltmp1484:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
	vmovaps	%ymm0, %ymm15
.Ltmp1485:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm0, %ymm0
.Ltmp1486:
	.loc	29 48 14
	vaddps	%ymm0, %ymm13, %ymm0
.Ltmp1487:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm15, %ymm13
.Ltmp1488:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm4
.Ltmp1489:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm15, %ymm13
.Ltmp1490:
	.loc	29 48 14
	vaddps	%ymm11, %ymm13, %ymm13
.Ltmp1491:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm15, %ymm11
.Ltmp1492:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm14
	vmovaps	%ymm5, %ymm11
.Ltmp1493:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm5, %ymm5
.Ltmp1494:
	.loc	29 48 14
	vaddps	%ymm0, %ymm5, %ymm0
.Ltmp1495:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm11, %ymm5
.Ltmp1496:
	.loc	29 48 14
	vaddps	%ymm4, %ymm5, %ymm4
.Ltmp1497:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm11, %ymm5
.Ltmp1498:
	.loc	29 48 14
	vaddps	%ymm5, %ymm13, %ymm5
.Ltmp1499:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm11, %ymm13
.Ltmp1500:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
	vmovaps	%ymm9, %ymm14
.Ltmp1501:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm9, %ymm9
.Ltmp1502:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm0
.Ltmp1503:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm14, %ymm9
.Ltmp1504:
	.loc	29 48 14
	vaddps	%ymm4, %ymm9, %ymm9
.Ltmp1505:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm14, %ymm4
.Ltmp1506:
	.loc	29 48 14
	vaddps	%ymm5, %ymm4, %ymm5
.Ltmp1507:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm14, %ymm4
.Ltmp1508:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm13
	vmovaps	128(%rsp), %ymm4
.Ltmp1509:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm4, %ymm10
.Ltmp1510:
	.loc	29 48 14
	vaddps	%ymm0, %ymm10, %ymm0
.Ltmp1511:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm4, %ymm10
.Ltmp1512:
	.loc	29 48 14
	vaddps	%ymm9, %ymm10, %ymm9
.Ltmp1513:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm4, %ymm10
.Ltmp1514:
	.loc	29 48 14
	vaddps	%ymm5, %ymm10, %ymm5
.Ltmp1515:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm4, %ymm10
.Ltmp1516:
	.loc	29 48 14
	vaddps	%ymm13, %ymm10, %ymm10
.Ltmp1517:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm13
.Ltmp1518:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm0, %ymm13, %ymm0
.Ltmp1519:
	.loc	29 82 19
	vandps	%ymm2, %ymm13, %ymm8
.Ltmp1520:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm8, %ymm0
.Ltmp1521:
	.loc	29 82 19
	vandps	%ymm13, %ymm9, %ymm8
.Ltmp1522:
	.loc	29 233 14
	vmaxps	%ymm8, %ymm0, %ymm0
.Ltmp1523:
	.loc	29 82 19
	vandps	%ymm5, %ymm13, %ymm5
.Ltmp1524:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm0, %ymm0
.Ltmp1525:
	.loc	29 82 19
	vandps	%ymm13, %ymm10, %ymm5
.Ltmp1526:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm0, %ymm0
.Ltmp1527:
	.loc	12 551 14
	vmovups	%ymm0, 4320(%rsp,%rcx)
.Ltmp1528:
	.loc	11 304 12
	addq	$32, %rcx
	vmovaps	%ymm7, %ymm13
	vmovaps	%ymm14, %ymm10
	vmovaps	%ymm11, %ymm9
	vmovaps	%ymm15, %ymm5
	vmovaps	%ymm1, %ymm0
	cmpq	%rcx, %rax
	jne	.LBB1_78
.Ltmp1529:
.LBB1_79:
	.loc	11 0 12 is_stmt 0
	vmovaps	%ymm7, 288(%rsp)
	vmovaps	%ymm6, 768(%rsp)
	vmovaps	%ymm3, 960(%rsp)
	vmovaps	%ymm12, %ymm9
	vmovaps	448(%rsp), %ymm6
	vmovaps	384(%rsp), %ymm10
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm1, 1216(%rsp)
	vmovaps	%ymm15, 1248(%rsp)
	vmovaps	%ymm11, 1280(%rsp)
	vmovaps	%ymm14, 1312(%rsp)
	vmovaps	%ymm4, 1344(%rsp)
.Ltmp1530:
	.loc	11 304 12
	cmpq	%r14, 8(%rsp)
	vmovaps	1824(%rsp), %ymm13
.Ltmp1531:
	.loc	3 900 12
	je	.LBB1_71
.Ltmp1532:
	.loc	3 0 12 is_stmt 0
	movq	%r14, 608(%rsp)
	movq	%r11, 96(%rsp)
	movq	%r10, 192(%rsp)
	movq	%rdx, 256(%rsp)
	movq	1624(%r13), %r14
	movq	1632(%r13), %rax
	movq	%rax, 480(%rsp)
	xorl	%r9d, %r9d
	vmovaps	%ymm10, %ymm3
	vmovaps	928(%rsp), %ymm12
	vmovaps	896(%rsp), %ymm7
	.p2align	4
.LBB1_81:
	movq	608(%rsp), %rax
.Ltmp1533:
	.loc	1 3271 24 is_stmt 1
	leaq	(%r9,%rax), %rdi
	shlq	$3, %rdi
.Ltmp1534:
	.loc	5 568 12
	movq	%r15, %rdx
	subq	%rdi, %rdx
	jb	.LBB1_339
.Ltmp1535:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_324
.Ltmp1536:
	.loc	1 1392 25
	movq	1688(%r13), %rsi
.Ltmp1537:
	.loc	1 1388 17
	movq	1840(%r13), %rax
.Ltmp1538:
	.loc	1 1392 45
	movq	%rax, %r10
	imulq	32(%rsp), %r10
.Ltmp1539:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r10, %rdx
	jb	.LBB1_340
.Ltmp1540:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_324
.Ltmp1541:
	.loc	5 0 16 is_stmt 0
	movq	%rax, 736(%rsp)
	movq	%rdi, 832(%rsp)
	movq	%r8, 672(%rsp)
	movq	%r9, 864(%rsp)
	movq	%r9, %rax
	shlq	$5, %rax
	vmovups	4320(%rsp,%rax), %ymm0
.Ltmp1542:
	vmaxps	%ymm0, %ymm0, %ymm5
.Ltmp1543:
	vblendvps	%ymm13, %ymm5, %ymm0, %ymm0
.Ltmp1544:
	vdivps	%ymm0, %ymm7, %ymm5
	vcmpgt_oqps	%ymm7, %ymm0, %ymm0
	vbroadcastss	.LCPI1_2(%rip), %ymm8
	vblendvps	%ymm0, %ymm5, %ymm8, %ymm0
.Ltmp1545:
	.loc	1 1392 25 is_stmt 1
	movq	1680(%r13), %rax
	movq	%r10, 704(%rsp)
.Ltmp1546:
	.loc	12 551 14
	vmovups	%ymm0, (%rax,%r10,4)
.Ltmp1547:
	.loc	1 1259 17
	movq	1840(%r13), %rdx
.Ltmp1548:
	.loc	13 37 12
	testq	%rdx, %rdx
	je	.LBB1_104
.Ltmp1549:
	.loc	13 0 12 is_stmt 0
	movq	88(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 416(%rsp)
	movq	32(%rsp), %rax
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
	movq	%r8, 320(%rsp)
	movq	1760(%rdi), %r15
	movq	1736(%rdi), %r8
	movq	%r8, 128(%rsp)
	movq	1728(%rdi), %r13
	imulq	%rdx, %r9
	movq	%r9, 352(%rsp)
	movq	%rdx, %rbx
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB1_89
	.p2align	4
.LBB1_87:
	xorl	%r9d, %r9d
.LBB1_88:
	decq	%rbx
	addq	$4, %rdi
.Ltmp1550:
	movl	%r9d, (%r15,%r11,4)
.Ltmp1551:
	incq	%r11
.Ltmp1552:
	.loc	13 37 12 is_stmt 1
	testq	%rbx, %rbx
	je	.LBB1_104
.LBB1_89:
.Ltmp1553:
	.loc	7 1714 9
	cmpq	$32, %rdi
.Ltmp1554:
	.loc	6 180 28
	je	.LBB1_104
.Ltmp1555:
	.loc	1 1263 21
	cmpq	416(%rsp), %r11
	je	.LBB1_389
	leaq	(%r11,%r11,2), %r9
	movl	4(%rcx,%r9,4), %r8d
.Ltmp1556:
	.loc	1 1265 23
	addq	32(%rsp), %r8
.Ltmp1557:
	.loc	1 1266 12
	cmpq	%r14, %r8
	movl	$0, %r10d
	cmovaeq	%r14, %r10
	subq	%r10, %r8
.Ltmp1558:
	.loc	1 1273 42
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_391
.Ltmp1559:
	.loc	1 1274 24 is_stmt 1
	cmpq	320(%rsp), %r11
	je	.LBB1_392
.Ltmp1560:
	.loc	1 0 0 is_stmt 0
	movl	(%rcx,%r9,4), %r10d
.Ltmp1561:
	vmovss	(%rax,%r12,4), %xmm0
.Ltmp1562:
	.loc	1 1274 24
	movl	(%r15,%r11,4), %r9d
	testq	%r9, %r9
	je	.LBB1_97
.Ltmp1563:
	.loc	1 1278 24 is_stmt 1
	cmpq	128(%rsp), %r11
	jae	.LBB1_395
	vmovss	(%r13,%r11,4), %xmm5
.Ltmp1564:
	.loc	1 903 8
	vucomiss	%xmm5, %xmm0
	jbe	.LBB1_97
.Ltmp1565:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm5, %xmm0
.LBB1_97:
.Ltmp1566:
	.loc	1 1280 9 is_stmt 1
	cmpq	128(%rsp), %r11
	je	.LBB1_390
	vmovss	%xmm0, (%r13,%r11,4)
	.loc	1 1281 24
	incq	%r9
	cmpq	%r10, %r9
.Ltmp1567:
	.loc	1 1282 23
	jne	.LBB1_102
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 3104(%rsp,%rdi)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rax,%r12,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp1568:
	.p2align	4
.LBB1_100:
.Ltmp1569:
	.loc	1 1291 65 is_stmt 1
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_376
.Ltmp1570:
	.loc	1 903 8 is_stmt 1
	vminss	(%rax,%r12,4), %xmm0, %xmm0
.Ltmp1571:
	.loc	1 1292 17
	vmovss	%xmm0, (%rax,%r12,4)
	.loc	1 1293 20
	testq	%r8, %r8
	cmoveq	%r14, %r8
	.loc	1 1296 17
	decq	%r8
.Ltmp1572:
	.loc	10 1916 50
	decq	%r10
.Ltmp1573:
	.loc	3 900 12
	jne	.LBB1_100
	jmp	.LBB1_87
.Ltmp1574:
	.loc	3 0 12 is_stmt 0
.Ltmp1575:
	.p2align	4
.LBB1_102:
	movq	352(%rsp), %r8
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%r8), %r12
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_397
	vmovss	(%rax,%r12,4), %xmm5
.Ltmp1576:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm5, %xmm0
.Ltmp1577:
	.loc	1 1282 9
	vmovss	%xmm0, 3104(%rsp,%rdi)
	jmp	.LBB1_88
.Ltmp1578:
	.loc	1 0 9 is_stmt 0
.Ltmp1579:
	.p2align	4
.LBB1_104:
	.loc	12 551 14 is_stmt 1
	vmovaps	3104(%rsp), %ymm8
	movq	88(%rsp), %r13
.Ltmp1580:
	.loc	1 1412 26
	movq	1704(%r13), %rsi
	vmovaps	%ymm8, %ymm0
	movq	736(%rsp), %rdi
.Ltmp1581:
	.loc	13 37 12
	testq	%rdi, %rdi
	movq	24(%rsp), %rbx
	movq	704(%rsp), %r11
	je	.LBB1_130
.Ltmp1582:
	.loc	13 0 12 is_stmt 0
	movq	1832(%r13), %r9
.Ltmp1583:
	.loc	1 1403 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB1_426
	.loc	1 0 42 is_stmt 0
	movq	1824(%r13), %r10
	.loc	1 1403 42
	movl	8(%r10), %r8d
	.loc	1 1403 28
	addq	32(%rsp), %r8
.Ltmp1584:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r14, %r8
	movl	$0, %eax
	cmovaeq	%r14, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%rdi, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_399
	.loc	1 0 25
	movq	1696(%r13), %rdx
	.loc	1 1407 25
	vmovss	(%rdx,%r8,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 3104(%rsp)
.Ltmp1585:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB1_129
.Ltmp1586:
	.loc	1 1403 42
	cmpq	$1, %r9
	je	.LBB1_403
	movl	20(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1587:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_399
	vmovss	4(%rdx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 3108(%rsp)
.Ltmp1588:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB1_129
.Ltmp1589:
	.loc	1 1403 42
	cmpq	$2, %r9
	je	.LBB1_405
	movl	32(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1590:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_399
	vmovss	8(%rdx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 3112(%rsp)
.Ltmp1591:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB1_129
.Ltmp1592:
	.loc	1 1403 42
	cmpq	$3, %r9
	je	.LBB1_415
	movl	44(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1593:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_399
	vmovss	12(%rdx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 3116(%rsp)
.Ltmp1594:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB1_129
.Ltmp1595:
	.loc	1 1403 42
	cmpq	$4, %r9
	je	.LBB1_418
	movl	56(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1596:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_399
	vmovss	16(%rdx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 3120(%rsp)
.Ltmp1597:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB1_129
.Ltmp1598:
	.loc	1 1403 42
	cmpq	$5, %r9
	je	.LBB1_422
	movl	68(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1599:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_399
	vmovss	20(%rdx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 3124(%rsp)
.Ltmp1600:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB1_129
.Ltmp1601:
	.loc	1 1403 42
	cmpq	$6, %r9
	je	.LBB1_423
	movl	80(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1602:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_399
	vmovss	24(%rdx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 3128(%rsp)
.Ltmp1603:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB1_129
.Ltmp1604:
	.loc	1 1403 42
	cmpq	$7, %r9
	je	.LBB1_424
	movl	92(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1605:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r14, %rax
	movl	$0, %ecx
	cmovaeq	%r14, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_399
	vmovss	28(%rdx,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 3132(%rsp)
.Ltmp1606:
	.loc	1 0 13
.Ltmp1607:
	.p2align	4
.LBB1_129:
	.loc	12 551 14 is_stmt 1
	vmovaps	3104(%rsp), %ymm0
.Ltmp1608:
.LBB1_130:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r11, %rdx
	movq	80(%rsp), %r15
	jb	.LBB1_341
.Ltmp1609:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_324
.Ltmp1610:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI1_3(%rip), %ymm5
	vmulps	%ymm5, %ymm8, %ymm5
	vroundps	$9, %ymm5, %ymm5
	vbroadcastss	.LCPI1_4(%rip), %ymm8
	vmulps	%ymm5, %ymm8, %ymm5
.Ltmp1611:
	vaddps	224(%rsp), %ymm5, %ymm8
	vsubps	%ymm0, %ymm8, %ymm0
	.loc	1 1412 26 is_stmt 1
	movq	1696(%r13), %rax
.Ltmp1612:
	.loc	12 551 14
	vmovups	%ymm5, (%rax,%r11,4)
	vmovaps	%ymm0, 224(%rsp)
.Ltmp1613:
	.loc	29 360 14
	vdivps	1856(%rsp), %ymm0, %ymm0
.Ltmp1614:
	.loc	1 1416 43
	vmovaps	1632(%rsp), %ymm5
.Ltmp1615:
	.loc	29 347 14
	vbroadcastss	.LCPI1_2(%rip), %ymm8
	vsubps	%ymm0, %ymm8, %ymm0
.Ltmp1616:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm5, %ymm0, %ymm8
.Ltmp1617:
	.loc	29 283 14 is_stmt 1
	vmulps	1888(%rsp), %ymm8, %ymm8
.Ltmp1618:
	.loc	29 48 14
	vaddps	%ymm5, %ymm8, %ymm5
.Ltmp1619:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm0, %ymm0
.Ltmp1620:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm5
	vandps	%ymm5, %ymm0, %ymm5
.Ltmp1621:
	.loc	29 871 14
	vbroadcastss	.LCPI1_5(%rip), %ymm8
	vcmplt_oqps	%ymm8, %ymm5, %ymm5
.Ltmp1622:
	.loc	29 82 19
	vandnps	%ymm0, %ymm5, %ymm0
.Ltmp1623:
	.loc	1 1417 5
	vmovaps	%ymm0, 1632(%rsp)
.Ltmp1624:
	.loc	1 1420 28
	movq	1672(%r13), %rsi
	movq	672(%rsp), %r8
	.loc	1 1420 44 is_stmt 0
	imulq	%r8, %rdi
.Ltmp1625:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB1_342
.Ltmp1626:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_324
.Ltmp1627:
	.loc	5 0 16 is_stmt 0
	movq	864(%rsp), %r9
	incq	%r9
	movq	832(%rsp), %rax
.Ltmp1628:
	leaq	(%rbx,%rax,4), %rax
.Ltmp1629:
	vbroadcastss	.LCPI1_2(%rip), %ymm5
	vsubps	%ymm0, %ymm5, %ymm0
.Ltmp1630:
	.loc	1 1420 28 is_stmt 1
	movq	1664(%r13), %rcx
.Ltmp1631:
	.loc	12 551 14
	vmovups	(%rcx,%rdi,4), %ymm5
.Ltmp1632:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rax), %ymm8
	vmovups	%ymm8, (%rcx,%rdi,4)
.Ltmp1633:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm5, %ymm0, %ymm0
.Ltmp1634:
	.loc	29 585 19
	vblendvps	%ymm12, %ymm5, %ymm0, %ymm0
.Ltmp1635:
	.loc	12 551 14
	vmovups	%ymm0, (%rax)
.Ltmp1636:
	.loc	1 3295 13
	incq	%r8
	.loc	1 3296 16
	cmpq	480(%rsp), %r8
	movl	$0, %edx
	cmoveq	%rdx, %r8
	movq	32(%rsp), %rax
	.loc	1 3299 13
	incq	%rax
	.loc	1 3300 16
	cmpq	%r14, %rax
	movl	$0, %ecx
	movq	%rcx, 544(%rsp)
	cmoveq	%rdx, %rax
	movq	%rax, 32(%rsp)
.Ltmp1637:
	.loc	10 1916 50
	cmpq	1792(%rsp), %r9
.Ltmp1638:
	.loc	3 900 12
	jne	.LBB1_81
.Ltmp1639:
	.loc	3 0 12 is_stmt 0
	vmovaps	224(%rsp), %ymm0
.Ltmp1640:
	.loc	1 1411 0 is_stmt 1
	vmovaps	%ymm0, 1664(%rsp)
	movq	256(%rsp), %rdx
	movl	$32, %r9d
	movq	192(%rsp), %r10
	movq	96(%rsp), %r11
	movq	608(%rsp), %r14
	jmp	.LBB1_72
.Ltmp1641:
.LBB1_136:
	.loc	1 1117 12
	movq	1768(%r13), %rax
	testq	%rax, %rax
	je	.LBB1_217
	.loc	1 0 12 is_stmt 0
	movq	1760(%r13), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB1_138:
.Ltmp1642:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp1643:
	.loc	6 180 28
	je	.LBB1_217
.Ltmp1644:
	.loc	6 315 25
	movl	(%rcx,%rdx), %esi
	addq	$4, %rdx
.Ltmp1645:
	.loc	1 1117 43
	cmpl	(%rcx), %esi
.Ltmp1646:
	.loc	6 315 25
	je	.LBB1_138
.Ltmp1647:
.LBB1_140:
	.loc	6 0 25 is_stmt 0
	leaq	3104(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp1648:
	.loc	1 3247 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp1649:
	.loc	1 3252 19
	movzbl	1536(%r13), %r14d
.Ltmp1650:
	.loc	1 3253 21
	movzbl	1537(%r13), %ebx
.Ltmp1651:
	.loc	1 3254 27
	movl	1640(%r13), %r12d
.Ltmp1652:
	.loc	1 3255 27
	movl	1644(%r13), %eax
	movq	%rax, 32(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 512(%rsp)
	leaq	4320(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
	movq	8(%rsp), %rax
.Ltmp1653:
	.loc	4 3758 16
	leaq	31(%rax), %rdx
	shrq	$5, %rdx
.Ltmp1654:
	.loc	8 446 20
	je	.LBB1_215
.Ltmp1655:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp1656:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 896(%rsp)
	vmovaps	%ymm1, 928(%rsp)
	testb	%r14b, %r14b
	jne	.LBB1_143
.Ltmp1657:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 928(%rsp)
.LBB1_143:
	testb	%bl, %bl
	movq	%r12, %r14
	jne	.LBB1_145
	vmovaps	%ymm0, 896(%rsp)
.LBB1_145:
	vmovaps	3520(%rsp), %ymm10
	vmovaps	3648(%rsp), %ymm0
	vmovaps	%ymm0, 3040(%rsp)
	vmovaps	3808(%rsp), %ymm0
	vmovaps	%ymm0, 3072(%rsp)
	vmovaps	3104(%rsp), %ymm6
	vmovaps	3136(%rsp), %ymm3
	vmovaps	3168(%rsp), %ymm0
	vmovaps	%ymm0, 224(%rsp)
	vmovaps	3200(%rsp), %ymm1
	vmovaps	3232(%rsp), %ymm8
	vmovaps	3264(%rsp), %ymm11
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	3328(%rsp), %ymm0
	vmovaps	3360(%rsp), %ymm2
	vmovaps	3392(%rsp), %ymm13
	vmovaps	3424(%rsp), %ymm4
	vmovaps	3456(%rsp), %ymm5
	vmovaps	3584(%rsp), %ymm7
	vmovaps	%ymm7, 704(%rsp)
	movq	$0, 72(%rsp)
	movq	24(%rsp), %rbx
	movq	8(%rsp), %rcx
	xorl	%r9d, %r9d
	vmovaps	3488(%rsp), %ymm7
	vmovaps	%ymm7, 672(%rsp)
	vmovaps	3552(%rsp), %ymm12
	vmovaps	3712(%rsp), %ymm7
	vmovaps	%ymm7, 864(%rsp)
	vmovaps	3616(%rsp), %ymm7
	vmovaps	%ymm7, 832(%rsp)
	vmovaps	3680(%rsp), %ymm14
	vmovaps	3776(%rsp), %ymm7
	vmovaps	%ymm7, 608(%rsp)
	vmovaps	%ymm10, 3872(%rsp)
.Ltmp1658:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB1_148
.Ltmp1659:
	.loc	8 0 20 is_stmt 0
.Ltmp1660:
	.p2align	4
.LBB1_146:
	movq	3840(%rsp), %rcx
	vmovaps	3008(%rsp), %ymm12
	vmovaps	2976(%rsp), %ymm14
.LBB1_147:
	addq	$32, %r9
	decq	%rdx
.Ltmp1661:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	addq	$1024, %rbx
	testq	%rdx, %rdx
	vmovaps	96(%rsp), %ymm3
	vmovaps	736(%rsp), %ymm1
	vmovaps	1760(%rsp), %ymm6
	vmovaps	%ymm7, %ymm8
	vmovaps	640(%rsp), %ymm11
	je	.LBB1_268
.LBB1_148:
.Ltmp1662:
	.loc	4 2584 13
	cmpq	$1, %rcx
	movq	%rcx, %rsi
	adcq	$0, %rsi
	cmpq	$32, %rsi
	movl	$32, %eax
	cmovaeq	%rax, %rsi
	movq	%rsi, 768(%rsp)
	movq	8(%rsp), %rsi
.Ltmp1663:
	.loc	1 3260 51
	subq	%r9, %rsi
.Ltmp1664:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%rax, %rsi
.Ltmp1665:
	.loc	1 3261 27
	leaq	(,%r9,8), %rax
.Ltmp1666:
	.loc	1 3265 35
	addq	%r9, %rsi
	shlq	$3, %rsi
.Ltmp1667:
	.loc	4 1050 16
	cmpq	%rax, %rsi
	jb	.LBB1_371
	cmpq	%r15, %rsi
	ja	.LBB1_371
.Ltmp1668:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm14, 2976(%rsp)
	vmovaps	%ymm12, 3008(%rsp)
	movq	%rcx, 3840(%rsp)
	vmovaps	%ymm6, 1760(%rsp)
	vmovaps	224(%rsp), %ymm7
.Ltmp1669:
	.loc	11 304 12 is_stmt 1
	cmpq	%r9, 8(%rsp)
	vmovaps	%ymm1, 736(%rsp)
	vmovaps	%ymm7, 224(%rsp)
	jne	.LBB1_152
	.loc	11 0 12 is_stmt 0
	vmovaps	%ymm3, %ymm10
	vmovaps	%ymm8, %ymm9
	vmovaps	%ymm11, %ymm1
	vmovaps	1760(%rsp), %ymm11
	vmovaps	1728(%rsp), %ymm7
	.loc	11 304 12
	jmp	.LBB1_154
.Ltmp1670:
	.loc	11 0 12
.Ltmp1671:
	.p2align	4
.LBB1_152:
	movq	768(%rsp), %rax
	shll	$5, %eax
	vmovaps	(%r13), %ymm5
	vmovaps	%ymm5, 416(%rsp)
	vmovaps	32(%r13), %ymm5
	vmovaps	%ymm5, 320(%rsp)
	vmovaps	64(%r13), %ymm5
	vmovaps	%ymm5, 352(%rsp)
	vmovaps	96(%r13), %ymm5
	vmovaps	%ymm5, 448(%rsp)
	vmovaps	128(%r13), %ymm5
	vmovaps	%ymm5, 384(%rsp)
	vmovaps	160(%r13), %ymm5
	vmovaps	%ymm5, 480(%rsp)
	vmovaps	192(%r13), %ymm5
	vmovaps	%ymm5, 288(%rsp)
	vmovaps	224(%r13), %ymm5
	vmovaps	%ymm5, 256(%rsp)
	vmovaps	256(%r13), %ymm5
	vmovaps	%ymm5, 192(%rsp)
	vmovaps	288(%r13), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	320(%r13), %ymm5
	vmovaps	%ymm5, 640(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm4, %ymm15
	vmovaps	%ymm13, %ymm14
	vmovaps	%ymm2, %ymm12
	vmovaps	%ymm0, %ymm6
	vmovaps	352(%r13), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	384(%r13), %ymm0
	vmovaps	%ymm0, 2912(%rsp)
	vmovaps	416(%r13), %ymm0
	vmovaps	%ymm0, 2880(%rsp)
	vmovaps	448(%r13), %ymm0
	vmovaps	%ymm0, 2848(%rsp)
	vmovaps	480(%r13), %ymm0
	vmovaps	%ymm0, 2816(%rsp)
	vmovaps	512(%r13), %ymm0
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	544(%r13), %ymm0
	vmovaps	%ymm0, 2752(%rsp)
	vmovaps	576(%r13), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	608(%r13), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	640(%r13), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	672(%r13), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
	vmovaps	704(%r13), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	736(%r13), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	768(%r13), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	800(%r13), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	832(%r13), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	864(%r13), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	896(%r13), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	928(%r13), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	960(%r13), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	992(%r13), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	1024(%r13), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	1056(%r13), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	1088(%r13), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	1120(%r13), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	1152(%r13), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	1184(%r13), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	1216(%r13), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	1248(%r13), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	1280(%r13), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	1312(%r13), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	1344(%r13), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	1376(%r13), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	1408(%r13), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1440(%r13), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	1472(%r13), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	1504(%r13), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	%ymm8, %ymm9
	vmovaps	%ymm11, %ymm1
	vmovaps	1760(%rsp), %ymm11
	vmovaps	1728(%rsp), %ymm7
	.p2align	4
.LBB1_153:
	vmovaps	%ymm15, 128(%rsp)
	vmovaps	%ymm11, %ymm10
.Ltmp1672:
	.loc	12 551 14 is_stmt 1
	vmovups	(%rbx,%rcx), %ymm11
.Ltmp1673:
	.loc	29 283 14
	vmulps	416(%rsp), %ymm11, %ymm0
	vxorps	%xmm8, %xmm8, %xmm8
.Ltmp1674:
	.loc	29 48 14
	vaddps	%ymm0, %ymm8, %ymm0
.Ltmp1675:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm11, %ymm2
.Ltmp1676:
	.loc	29 48 14
	vaddps	%ymm2, %ymm8, %ymm2
.Ltmp1677:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm11, %ymm4
.Ltmp1678:
	.loc	29 48 14
	vaddps	%ymm4, %ymm8, %ymm4
.Ltmp1679:
	.loc	29 283 14
	vmulps	384(%rsp), %ymm10, %ymm5
.Ltmp1680:
	.loc	29 48 14
	vaddps	%ymm0, %ymm5, %ymm5
.Ltmp1681:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm10, %ymm0
.Ltmp1682:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm2
.Ltmp1683:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm10, %ymm0
.Ltmp1684:
	.loc	29 48 14
	vaddps	%ymm4, %ymm0, %ymm4
.Ltmp1685:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm11, %ymm0
.Ltmp1686:
	.loc	29 48 14
	vaddps	%ymm0, %ymm8, %ymm0
.Ltmp1687:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm10, %ymm13
.Ltmp1688:
	.loc	29 48 14
	vaddps	%ymm0, %ymm13, %ymm13
	vmovaps	%ymm7, %ymm8
	vmovaps	%ymm1, %ymm7
	vmovaps	%ymm9, %ymm0
	vmovaps	736(%rsp), %ymm9
	vmovaps	224(%rsp), %ymm1
	vmovaps	%ymm3, %ymm15
.Ltmp1689:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm3, %ymm3
.Ltmp1690:
	.loc	29 48 14
	vaddps	%ymm5, %ymm3, %ymm3
.Ltmp1691:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm15, %ymm5
.Ltmp1692:
	.loc	29 48 14
	vaddps	%ymm2, %ymm5, %ymm2
.Ltmp1693:
	.loc	29 283 14
	vmulps	640(%rsp), %ymm15, %ymm5
.Ltmp1694:
	.loc	29 48 14
	vaddps	%ymm4, %ymm5, %ymm4
	vmovaps	%ymm15, 224(%rsp)
.Ltmp1695:
	.loc	29 283 14
	vmulps	800(%rsp), %ymm15, %ymm5
.Ltmp1696:
	.loc	29 48 14
	vaddps	%ymm5, %ymm13, %ymm5
.Ltmp1697:
	.loc	29 283 14
	vmulps	2912(%rsp), %ymm1, %ymm13
.Ltmp1698:
	.loc	29 48 14
	vaddps	%ymm3, %ymm13, %ymm3
.Ltmp1699:
	.loc	29 283 14
	vmulps	2880(%rsp), %ymm1, %ymm13
.Ltmp1700:
	.loc	29 48 14
	vaddps	%ymm2, %ymm13, %ymm2
.Ltmp1701:
	.loc	29 283 14
	vmulps	2848(%rsp), %ymm1, %ymm13
.Ltmp1702:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm4
	vmovaps	%ymm1, 736(%rsp)
.Ltmp1703:
	.loc	29 283 14
	vmulps	2816(%rsp), %ymm1, %ymm13
.Ltmp1704:
	.loc	29 48 14
	vaddps	%ymm5, %ymm13, %ymm5
.Ltmp1705:
	.loc	29 283 14
	vmulps	2784(%rsp), %ymm9, %ymm13
.Ltmp1706:
	.loc	29 48 14
	vaddps	%ymm3, %ymm13, %ymm3
.Ltmp1707:
	.loc	29 283 14
	vmulps	2752(%rsp), %ymm9, %ymm13
.Ltmp1708:
	.loc	29 48 14
	vaddps	%ymm2, %ymm13, %ymm2
.Ltmp1709:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm9, %ymm13
.Ltmp1710:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm4
.Ltmp1711:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm9, %ymm13
.Ltmp1712:
	.loc	29 48 14
	vaddps	%ymm5, %ymm13, %ymm5
.Ltmp1713:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm0, %ymm13
.Ltmp1714:
	.loc	29 48 14
	vaddps	%ymm3, %ymm13, %ymm3
.Ltmp1715:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm0, %ymm13
.Ltmp1716:
	.loc	29 48 14
	vaddps	%ymm2, %ymm13, %ymm2
.Ltmp1717:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm0, %ymm13
.Ltmp1718:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm4
.Ltmp1719:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm0, %ymm13
.Ltmp1720:
	.loc	29 48 14
	vaddps	%ymm5, %ymm13, %ymm5
.Ltmp1721:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm7, %ymm13
.Ltmp1722:
	.loc	29 48 14
	vaddps	%ymm3, %ymm13, %ymm3
.Ltmp1723:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm7, %ymm13
.Ltmp1724:
	.loc	29 48 14
	vaddps	%ymm2, %ymm13, %ymm2
.Ltmp1725:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm7, %ymm13
.Ltmp1726:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm4
.Ltmp1727:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm7, %ymm13
.Ltmp1728:
	.loc	29 48 14
	vaddps	%ymm5, %ymm13, %ymm5
.Ltmp1729:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm8, %ymm13
.Ltmp1730:
	.loc	29 48 14
	vaddps	%ymm3, %ymm13, %ymm3
.Ltmp1731:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm8, %ymm13
.Ltmp1732:
	.loc	29 48 14
	vaddps	%ymm2, %ymm13, %ymm13
.Ltmp1733:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm8, %ymm2
.Ltmp1734:
	.loc	29 48 14
	vaddps	%ymm4, %ymm2, %ymm4
.Ltmp1735:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm8, %ymm2
.Ltmp1736:
	.loc	29 48 14
	vaddps	%ymm5, %ymm2, %ymm5
	vmovaps	%ymm6, %ymm2
.Ltmp1737:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm6, %ymm6
.Ltmp1738:
	.loc	29 48 14
	vaddps	%ymm3, %ymm6, %ymm3
.Ltmp1739:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm2, %ymm6
.Ltmp1740:
	.loc	29 48 14
	vaddps	%ymm6, %ymm13, %ymm6
.Ltmp1741:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm2, %ymm13
.Ltmp1742:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm4
.Ltmp1743:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm2, %ymm13
.Ltmp1744:
	.loc	29 48 14
	vaddps	%ymm5, %ymm13, %ymm5
	vmovaps	%ymm12, %ymm13
.Ltmp1745:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm12, %ymm12
.Ltmp1746:
	.loc	29 48 14
	vaddps	%ymm3, %ymm12, %ymm3
.Ltmp1747:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm13, %ymm12
.Ltmp1748:
	.loc	29 48 14
	vaddps	%ymm6, %ymm12, %ymm6
.Ltmp1749:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm13, %ymm12
.Ltmp1750:
	.loc	29 48 14
	vaddps	%ymm4, %ymm12, %ymm12
.Ltmp1751:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm13, %ymm4
.Ltmp1752:
	.loc	29 48 14
	vaddps	%ymm5, %ymm4, %ymm5
	vmovaps	%ymm14, %ymm4
.Ltmp1753:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm14, %ymm14
.Ltmp1754:
	.loc	29 48 14
	vaddps	%ymm3, %ymm14, %ymm3
.Ltmp1755:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm4, %ymm14
.Ltmp1756:
	.loc	29 48 14
	vaddps	%ymm6, %ymm14, %ymm6
.Ltmp1757:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm4, %ymm14
.Ltmp1758:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1759:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm4, %ymm14
.Ltmp1760:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm14
	vmovaps	128(%rsp), %ymm5
.Ltmp1761:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm5, %ymm15
.Ltmp1762:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp1763:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm5, %ymm15
.Ltmp1764:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp1765:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm5, %ymm15
.Ltmp1766:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp1767:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm5, %ymm15
.Ltmp1768:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp1769:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm15
.Ltmp1770:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm3, %ymm15, %ymm3
.Ltmp1771:
	.loc	29 82 19
	vandps	%ymm7, %ymm15, %ymm1
.Ltmp1772:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp1773:
	.loc	29 82 19
	vandps	%ymm6, %ymm15, %ymm3
.Ltmp1774:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp1775:
	.loc	29 82 19
	vandps	%ymm15, %ymm12, %ymm3
.Ltmp1776:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp1777:
	.loc	29 82 19
	vandps	%ymm15, %ymm14, %ymm3
.Ltmp1778:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp1779:
	.loc	12 551 14
	vmovups	%ymm1, 4320(%rsp,%rcx)
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm8, %ymm0
.Ltmp1780:
	.loc	11 304 12
	addq	$32, %rcx
	vmovaps	%ymm10, %ymm3
	vmovaps	%ymm4, %ymm15
	vmovaps	%ymm13, %ymm14
	vmovaps	%ymm2, %ymm12
	vmovaps	%ymm8, %ymm6
	cmpq	%rcx, %rax
	jne	.LBB1_153
.Ltmp1781:
.LBB1_154:
	.loc	11 0 12 is_stmt 0
	vmovaps	%ymm7, 1728(%rsp)
	vmovaps	%ymm1, 640(%rsp)
	vmovaps	%ymm11, 1760(%rsp)
	vmovaps	%ymm10, 96(%rsp)
	vmovaps	224(%rsp), %ymm11
	vmovaps	736(%rsp), %ymm3
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm3, 3200(%rsp)
	vmovaps	%ymm9, 3232(%rsp)
	vmovaps	%ymm1, 3264(%rsp)
	vmovaps	%ymm7, 3296(%rsp)
	vmovaps	%ymm0, 3328(%rsp)
	vmovaps	%ymm2, 3360(%rsp)
	vmovaps	%ymm13, 3392(%rsp)
	vmovaps	%ymm4, 3424(%rsp)
	vmovaps	%ymm5, 3456(%rsp)
.Ltmp1782:
	.loc	11 304 12
	cmpq	%r9, 8(%rsp)
	vmovaps	%ymm9, %ymm7
.Ltmp1783:
	.loc	3 900 12
	je	.LBB1_146
.Ltmp1784:
	.loc	3 0 12 is_stmt 0
	movq	%rdx, 800(%rsp)
	movq	1624(%r13), %rcx
	movq	1632(%r13), %rax
	movq	%rax, 192(%rsp)
	xorl	%eax, %eax
	vmovaps	3872(%rsp), %ymm10
	vmovaps	3008(%rsp), %ymm12
	vmovaps	2976(%rsp), %ymm14
	vxorps	%xmm8, %xmm8, %xmm8
	movq	%rbx, 960(%rsp)
	movq	%r9, 1824(%rsp)
	.p2align	4
.LBB1_156:
.Ltmp1785:
	.loc	1 3271 24 is_stmt 1
	leaq	(%rax,%r9), %rdi
	shlq	$3, %rdi
.Ltmp1786:
	.loc	5 568 12
	movq	%r15, %rdx
	subq	%rdi, %rdx
	jb	.LBB1_297
.Ltmp1787:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_270
.Ltmp1788:
	.loc	5 0 16 is_stmt 0
	movq	%rdi, 288(%rsp)
.Ltmp1789:
	.loc	1 1392 25 is_stmt 1
	movq	1688(%r13), %rsi
.Ltmp1790:
	.loc	1 1388 17
	movq	1840(%r13), %rdi
.Ltmp1791:
	.loc	1 1392 45
	movq	%rdi, %r8
	imulq	32(%rsp), %r8
.Ltmp1792:
	.loc	5 580 12
	movq	%rsi, %rdx
	movq	%r8, 448(%rsp)
	subq	%r8, %rdx
	jb	.LBB1_298
.Ltmp1793:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_270
.Ltmp1794:
	.loc	5 0 16 is_stmt 0
	movq	%rdi, 384(%rsp)
	movq	%r14, 480(%rsp)
	vbroadcastss	.LCPI1_1(%rip), %ymm1
	vaddps	704(%rsp), %ymm1, %ymm1
	vmaxps	%ymm8, %ymm1, %ymm1
	vmovaps	%ymm1, 704(%rsp)
	vcmpgt_oqps	%ymm8, %ymm1, %ymm15
	vaddps	672(%rsp), %ymm12, %ymm1
	vblendvps	%ymm15, %ymm1, %ymm10, %ymm9
	movq	%rax, 256(%rsp)
.Ltmp1795:
	shlq	$5, %rax
	vmovups	4320(%rsp,%rax), %ymm1
.Ltmp1796:
	vmaxps	%ymm1, %ymm1, %ymm3
	vmovaps	928(%rsp), %ymm6
.Ltmp1797:
	vblendvps	%ymm6, %ymm3, %ymm1, %ymm1
.Ltmp1798:
	vdivps	%ymm1, %ymm9, %ymm3
	vmovaps	%ymm9, 672(%rsp)
	vcmpgt_oqps	%ymm9, %ymm1, %ymm1
	vbroadcastss	.LCPI1_2(%rip), %ymm6
	vblendvps	%ymm1, %ymm3, %ymm6, %ymm1
.Ltmp1799:
	.loc	1 1392 25 is_stmt 1
	movq	1680(%r13), %rax
	movq	448(%rsp), %rdx
.Ltmp1800:
	.loc	12 551 14
	vmovups	%ymm1, (%rax,%rdx,4)
.Ltmp1801:
	.loc	1 1259 17
	movq	1840(%r13), %rdx
.Ltmp1802:
	.loc	13 37 12
	testq	%rdx, %rdx
	je	.LBB1_179
.Ltmp1803:
	.loc	13 0 12 is_stmt 0
	movq	88(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 416(%rsp)
	movq	32(%rsp), %rax
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
	movq	%r8, 320(%rsp)
	movq	1760(%rdi), %rbx
	movq	1736(%rdi), %r8
	movq	%r8, 128(%rsp)
	movq	1728(%rdi), %r15
	imulq	%rdx, %r9
	movq	%r9, 352(%rsp)
	movq	%rdx, %r13
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB1_164
	.p2align	4
.LBB1_162:
	xorl	%r9d, %r9d
.LBB1_163:
	decq	%r13
	addq	$4, %rdi
.Ltmp1804:
	movl	%r9d, (%rbx,%r11,4)
.Ltmp1805:
	incq	%r11
.Ltmp1806:
	.loc	13 37 12 is_stmt 1
	testq	%r13, %r13
	je	.LBB1_179
.LBB1_164:
.Ltmp1807:
	.loc	7 1714 9
	cmpq	$32, %rdi
.Ltmp1808:
	.loc	6 180 28
	je	.LBB1_179
.Ltmp1809:
	.loc	1 1263 21
	cmpq	416(%rsp), %r11
	je	.LBB1_380
	leaq	(%r11,%r11,2), %r9
	movl	4(%r14,%r9,4), %r8d
.Ltmp1810:
	.loc	1 1265 23
	addq	32(%rsp), %r8
.Ltmp1811:
	.loc	1 1266 12
	cmpq	%rcx, %r8
	movl	$0, %r10d
	cmovaeq	%rcx, %r10
	subq	%r10, %r8
.Ltmp1812:
	.loc	1 1273 42
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_382
.Ltmp1813:
	.loc	1 1274 24 is_stmt 1
	cmpq	320(%rsp), %r11
	je	.LBB1_383
.Ltmp1814:
	.loc	1 0 0 is_stmt 0
	movl	(%r14,%r9,4), %r10d
.Ltmp1815:
	vmovss	(%rax,%r12,4), %xmm1
.Ltmp1816:
	.loc	1 1274 24
	movl	(%rbx,%r11,4), %r9d
	testq	%r9, %r9
.Ltmp1817:
	.loc	1 1275 26 is_stmt 1
	je	.LBB1_172
	.loc	1 1278 24
	cmpq	128(%rsp), %r11
	jae	.LBB1_384
	vmovss	(%r15,%r11,4), %xmm3
.Ltmp1818:
	.loc	1 903 8
	vucomiss	%xmm3, %xmm1
	jbe	.LBB1_172
.Ltmp1819:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm3, %xmm1
.LBB1_172:
.Ltmp1820:
	.loc	1 1280 9 is_stmt 1
	cmpq	128(%rsp), %r11
	je	.LBB1_381
	vmovss	%xmm1, (%r15,%r11,4)
	.loc	1 1281 24
	incq	%r9
	cmpq	%r10, %r9
.Ltmp1821:
	.loc	1 1282 23
	jne	.LBB1_177
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm1, 512(%rsp,%rdi)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rax,%r12,4), %xmm1
	.loc	1 0 30 is_stmt 0
.Ltmp1822:
	.p2align	4
.LBB1_175:
.Ltmp1823:
	.loc	1 1291 65 is_stmt 1
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_337
.Ltmp1824:
	.loc	1 903 8 is_stmt 1
	vminss	(%rax,%r12,4), %xmm1, %xmm1
.Ltmp1825:
	.loc	1 1292 17
	vmovss	%xmm1, (%rax,%r12,4)
	.loc	1 1293 20
	testq	%r8, %r8
	cmoveq	%rcx, %r8
	.loc	1 1296 17
	decq	%r8
.Ltmp1826:
	.loc	10 1916 50
	decq	%r10
.Ltmp1827:
	.loc	3 900 12
	jne	.LBB1_175
	jmp	.LBB1_162
.Ltmp1828:
	.loc	3 0 12 is_stmt 0
.Ltmp1829:
	.p2align	4
.LBB1_177:
	movq	352(%rsp), %r8
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%r8), %r12
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB1_385
	vmovss	(%rax,%r12,4), %xmm3
.Ltmp1830:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm1, %xmm3, %xmm1
.Ltmp1831:
	.loc	1 1282 9
	vmovss	%xmm1, 512(%rsp,%rdi)
	jmp	.LBB1_163
.Ltmp1832:
	.loc	1 0 9 is_stmt 0
.Ltmp1833:
	.p2align	4
.LBB1_179:
	.loc	12 551 14 is_stmt 1
	vmovaps	512(%rsp), %ymm1
	movq	88(%rsp), %r13
.Ltmp1834:
	.loc	1 1412 26
	movq	1704(%r13), %rsi
	vmovaps	%ymm1, %ymm6
	movq	384(%rsp), %rdi
.Ltmp1835:
	.loc	13 37 12
	testq	%rdi, %rdi
	movq	24(%rsp), %r11
	movq	480(%rsp), %r14
	movq	960(%rsp), %rbx
	je	.LBB1_205
.Ltmp1836:
	.loc	13 0 12 is_stmt 0
	movq	1832(%r13), %r9
.Ltmp1837:
	.loc	1 1403 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB1_421
	.loc	1 0 42 is_stmt 0
	movq	1824(%r13), %r10
	.loc	1 1403 42
	movl	8(%r10), %r8d
	.loc	1 1403 28
	addq	32(%rsp), %r8
.Ltmp1838:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%rdi, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_396
	.loc	1 0 25
	movq	1696(%r13), %rdx
	.loc	1 1407 25
	vmovss	(%rdx,%r8,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 512(%rsp)
.Ltmp1839:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB1_204
.Ltmp1840:
	.loc	1 1403 42
	cmpq	$1, %r9
	je	.LBB1_398
	movl	20(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1841:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	384(%rsp), %rdi
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_396
	vmovss	4(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 516(%rsp)
.Ltmp1842:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB1_204
.Ltmp1843:
	.loc	1 1403 42
	cmpq	$2, %r9
	je	.LBB1_400
	movl	32(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1844:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	384(%rsp), %rdi
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_396
	vmovss	8(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 520(%rsp)
.Ltmp1845:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB1_204
.Ltmp1846:
	.loc	1 1403 42
	cmpq	$3, %r9
	je	.LBB1_402
	movl	44(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1847:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	384(%rsp), %rdi
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_396
	vmovss	12(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 524(%rsp)
.Ltmp1848:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB1_204
.Ltmp1849:
	.loc	1 1403 42
	cmpq	$4, %r9
	je	.LBB1_404
	movl	56(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1850:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	384(%rsp), %rdi
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_396
	vmovss	16(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 528(%rsp)
.Ltmp1851:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB1_204
.Ltmp1852:
	.loc	1 1403 42
	cmpq	$5, %r9
	je	.LBB1_414
	movl	68(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1853:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	384(%rsp), %rdi
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_396
	vmovss	20(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 532(%rsp)
.Ltmp1854:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB1_204
.Ltmp1855:
	.loc	1 1403 42
	cmpq	$6, %r9
	je	.LBB1_417
	movl	80(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1856:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	384(%rsp), %rdi
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_396
	vmovss	24(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 536(%rsp)
.Ltmp1857:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB1_204
.Ltmp1858:
	.loc	1 1403 42
	cmpq	$7, %r9
	je	.LBB1_419
	movl	92(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp1859:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	384(%rsp), %rdi
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB1_396
	vmovss	28(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 540(%rsp)
.Ltmp1860:
	.loc	1 0 13
.Ltmp1861:
	.p2align	4
.LBB1_204:
	.loc	12 551 14 is_stmt 1
	vmovaps	512(%rsp), %ymm6
.Ltmp1862:
.LBB1_205:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	448(%rsp), %rdx
	movq	80(%rsp), %r15
	movq	1824(%rsp), %r9
	jb	.LBB1_299
.Ltmp1863:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_270
.Ltmp1864:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm12, %ymm9
	vbroadcastss	.LCPI1_1(%rip), %ymm3
	vaddps	864(%rsp), %ymm3, %ymm3
	vmaxps	%ymm8, %ymm3, %ymm3
	vmovaps	%ymm3, 864(%rsp)
	vcmpgt_oqps	%ymm8, %ymm3, %ymm3
	vmovaps	%ymm14, %ymm8
	vaddps	832(%rsp), %ymm14, %ymm12
	vmovaps	3040(%rsp), %ymm14
	vblendvps	%ymm3, %ymm12, %ymm14, %ymm14
.Ltmp1865:
	vbroadcastss	.LCPI1_3(%rip), %ymm12
	vmulps	%ymm1, %ymm12, %ymm1
	vroundps	$9, %ymm1, %ymm1
	vbroadcastss	.LCPI1_4(%rip), %ymm12
	vmulps	%ymm1, %ymm12, %ymm1
.Ltmp1866:
	vaddps	608(%rsp), %ymm1, %ymm12
	vsubps	%ymm6, %ymm12, %ymm6
	.loc	1 1412 26 is_stmt 1
	movq	1696(%r13), %rax
	movq	448(%rsp), %rdx
.Ltmp1867:
	.loc	12 551 14
	vmovups	%ymm1, (%rax,%rdx,4)
	vmovaps	%ymm6, 608(%rsp)
.Ltmp1868:
	.loc	29 360 14
	vdivps	3072(%rsp), %ymm6, %ymm1
.Ltmp1869:
	.loc	1 1416 43
	vmovaps	3744(%rsp), %ymm6
.Ltmp1870:
	.loc	29 347 14
	vbroadcastss	.LCPI1_2(%rip), %ymm12
	vsubps	%ymm1, %ymm12, %ymm1
.Ltmp1871:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm6, %ymm1, %ymm12
	vmovaps	%ymm14, 832(%rsp)
.Ltmp1872:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm12, %ymm14, %ymm12
.Ltmp1873:
	.loc	29 48 14
	vaddps	%ymm6, %ymm12, %ymm6
.Ltmp1874:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm1, %ymm1
.Ltmp1875:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm6
	vandps	%ymm6, %ymm1, %ymm6
.Ltmp1876:
	.loc	29 871 14
	vbroadcastss	.LCPI1_5(%rip), %ymm12
	vcmplt_oqps	%ymm12, %ymm6, %ymm6
.Ltmp1877:
	.loc	29 82 19
	vandnps	%ymm1, %ymm6, %ymm1
.Ltmp1878:
	.loc	1 1417 5
	vmovaps	%ymm1, 3744(%rsp)
.Ltmp1879:
	.loc	1 1420 28
	movq	1672(%r13), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	%r14, %rdi
.Ltmp1880:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB1_300
.Ltmp1881:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB1_270
.Ltmp1882:
	.loc	5 0 16 is_stmt 0
	vxorps	%xmm6, %xmm6, %xmm6
	vblendvps	%ymm15, %ymm9, %ymm6, %ymm12
	vmovaps	%ymm8, %ymm14
	vxorps	%xmm8, %xmm8, %xmm8
	vblendvps	%ymm3, %ymm14, %ymm8, %ymm14
	movq	256(%rsp), %r8
.Ltmp1883:
	incq	%r8
	movq	288(%rsp), %rax
.Ltmp1884:
	leaq	(%r11,%rax,4), %rax
.Ltmp1885:
	vbroadcastss	.LCPI1_2(%rip), %ymm3
	vsubps	%ymm1, %ymm3, %ymm1
.Ltmp1886:
	.loc	1 1420 28 is_stmt 1
	movq	1664(%r13), %rdx
.Ltmp1887:
	.loc	12 551 14
	vmovups	(%rdx,%rdi,4), %ymm3
.Ltmp1888:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rax), %ymm6
	vmovups	%ymm6, (%rdx,%rdi,4)
.Ltmp1889:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm3, %ymm1, %ymm1
	vmovaps	896(%rsp), %ymm6
.Ltmp1890:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm3, %ymm1, %ymm1
.Ltmp1891:
	.loc	12 551 14
	vmovups	%ymm1, (%rax)
.Ltmp1892:
	.loc	1 3295 13
	incq	%r14
	.loc	1 3296 16
	cmpq	192(%rsp), %r14
	movl	$0, %esi
	cmoveq	%rsi, %r14
	movq	32(%rsp), %rax
	.loc	1 3299 13
	incq	%rax
	.loc	1 3300 16
	cmpq	%rcx, %rax
	movl	$0, %edx
	movq	%rdx, 72(%rsp)
	cmoveq	%rsi, %rax
	movq	%rax, 32(%rsp)
	movq	%r8, %rax
.Ltmp1893:
	.loc	10 1916 50
	cmpq	768(%rsp), %r8
.Ltmp1894:
	.loc	3 900 12
	jne	.LBB1_156
.Ltmp1895:
	.loc	3 0 12 is_stmt 0
	vmovaps	608(%rsp), %ymm1
.Ltmp1896:
	.loc	1 1411 5 is_stmt 1
	vmovaps	%ymm1, 3776(%rsp)
	vmovaps	704(%rsp), %ymm1
.Ltmp1897:
	.loc	1 851 0
	vmovaps	%ymm1, 3584(%rsp)
	vmovaps	672(%rsp), %ymm1
.Ltmp1898:
	.loc	1 853 0
	vmovaps	%ymm1, 3488(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm12, 3552(%rsp)
	vmovaps	864(%rsp), %ymm1
.Ltmp1899:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm1, 3712(%rsp)
	vmovaps	832(%rsp), %ymm1
.Ltmp1900:
	.loc	1 853 0
	vmovaps	%ymm1, 3616(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm14, 3680(%rsp)
	movq	800(%rsp), %rdx
	movq	3840(%rsp), %rcx
	jmp	.LBB1_147
.Ltmp1901:
.LBB1_211:
	.loc	1 2307 37 is_stmt 1
	movq	1792(%r13), %rsi
	movq	1800(%r13), %rdx
.Ltmp1902:
	.loc	6 314 17
	shlq	$4, %rdx
	.loc	6 0 17 is_stmt 0
.Ltmp1903:
	.p2align	4
.LBB1_212:
.Ltmp1904:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
.Ltmp1905:
	.loc	6 180 28
	je	.LBB1_255
.Ltmp1906:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp1907:
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
.Ltmp1908:
.LBB1_215:
	.loc	6 0 25
	movq	%r15, %rbx
	movq	%r12, %r14
.Ltmp1909:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB1_269
.Ltmp1910:
.LBB1_216:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp1911:
	.loc	1 3187 12
	je	.LBB1_272
.LBB1_217:
	.loc	1 0 12 is_stmt 0
	leaq	992(%rsp), %rdi
	movq	16(%rsp), %r15
.Ltmp1912:
	.loc	1 3327 24 is_stmt 1
	movq	%r15, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp1913:
	.loc	1 3332 19
	movzbl	1536(%r13), %r14d
.Ltmp1914:
	.loc	1 3333 21
	movzbl	1537(%r13), %eax
	movb	%al, 128(%rsp)
.Ltmp1915:
	.loc	1 3334 16
	movq	1624(%r13), %r12
.Ltmp1916:
	.loc	1 3335 16
	movq	1632(%r13), %rbx
.Ltmp1917:
	.loc	1 3336 27
	movl	1640(%r13), %eax
	movq	%rax, 352(%rsp)
.Ltmp1918:
	.loc	1 3337 27
	movl	1644(%r13), %eax
	movq	%rax, 320(%rsp)
	leaq	4320(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	3104(%rsp), %rdi
.Ltmp1919:
	.loc	1 3341 32
	movq	%r15, %rsi
	movq	%r12, %rdx
	movq	%rbx, 1760(%rsp)
	movq	%rbx, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	8(%rsp), %rax
.Ltmp1920:
	.loc	4 3758 16
	leaq	31(%rax), %r9
	shrq	$5, %r9
.Ltmp1921:
	.loc	8 446 20
	je	.LBB1_262
.Ltmp1922:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp1923:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 224(%rsp)
	vmovaps	%ymm1, 448(%rsp)
	testb	%r14b, %r14b
	movq	80(%rsp), %r15
	jne	.LBB1_220
.Ltmp1924:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 448(%rsp)
.LBB1_220:
	cmpb	$0, 128(%rsp)
	jne	.LBB1_222
	vmovaps	%ymm0, 224(%rsp)
.LBB1_222:
	movq	3192(%rsp), %rax
	movq	%rax, 3040(%rsp)
	movq	3200(%rsp), %rax
	movq	%rax, 3072(%rsp)
	movq	3136(%rsp), %r13
	movq	3144(%rsp), %rdx
	movq	3184(%rsp), %rax
	movq	%rax, 384(%rsp)
	movq	3152(%rsp), %rax
	movq	%rax, 864(%rsp)
	movq	3160(%rsp), %r11
	movq	3176(%rsp), %rax
	movq	%rax, 672(%rsp)
	movq	3168(%rsp), %rax
	movq	%rax, 832(%rsp)
	vmovaps	3104(%rsp), %ymm13
	movl	3208(%rsp), %r10d
	movl	$32, %r14d
	movq	24(%rsp), %rbx
	movq	%rbx, %rcx
	movq	8(%rsp), %rax
	movq	%rax, %rsi
	xorl	%edi, %edi
	movq	%r11, 736(%rsp)
.LBB1_223:
.Ltmp1925:
	.loc	4 2584 13 is_stmt 1
	cmpq	$32, %rsi
	movl	$32, %r8d
	movq	%rsi, 3008(%rsp)
	cmovbq	%rsi, %r8
	cmpq	$1, %r8
	movq	%r8, 72(%rsp)
	adcq	$0, %r8
.Ltmp1926:
	.loc	1 3344 55
	movq	%rax, %rsi
	subq	%rdi, %rsi
.Ltmp1927:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%r14, %rsi
	movq	%rdi, %rax
.Ltmp1928:
	.loc	1 3345 31
	leaq	(,%rdi,8), %rdi
	movq	%rax, 928(%rsp)
	movq	%rsi, 896(%rsp)
.Ltmp1929:
	.loc	1 3349 39
	addq	%rax, %rsi
	shlq	$3, %rsi
.Ltmp1930:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB1_394
	cmpq	%r15, %rsi
	ja	.LBB1_394
.Ltmp1931:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm13, 1728(%rsp)
.Ltmp1932:
	.loc	1 1759 23 is_stmt 1
	vmovaps	992(%rsp), %ymm7
	vmovaps	1024(%rsp), %ymm9
	vmovaps	1056(%rsp), %ymm5
	vmovaps	1088(%rsp), %ymm10
	vmovaps	1120(%rsp), %ymm6
	vmovaps	1152(%rsp), %ymm1
	vmovaps	1184(%rsp), %ymm11
	vmovaps	1216(%rsp), %ymm4
	vmovaps	1248(%rsp), %ymm2
	vmovaps	1280(%rsp), %ymm12
	vmovaps	1312(%rsp), %ymm0
	movq	928(%rsp), %rdi
.Ltmp1933:
	.loc	11 304 12
	cmpq	%rdi, 8(%rsp)
	vmovaps	%ymm5, 128(%rsp)
	jne	.LBB1_227
.Ltmp1934:
	.loc	1 0 0 is_stmt 0
	vmovaps	1344(%rsp), %ymm13
.Ltmp1935:
	.loc	11 304 12
	jmp	.LBB1_229
.Ltmp1936:
.LBB1_227:
	.loc	11 0 12
	shll	$5, %r8d
	movq	%rcx, %rsi
	movq	88(%rsp), %rcx
	vmovaps	(%rcx), %ymm3
	vmovaps	%ymm3, 704(%rsp)
	vmovaps	32(%rcx), %ymm3
	vmovaps	%ymm3, 608(%rsp)
	vmovaps	64(%rcx), %ymm3
	vmovaps	%ymm3, 480(%rsp)
	vmovaps	96(%rcx), %ymm3
	vmovaps	%ymm3, 288(%rsp)
	vmovaps	128(%rcx), %ymm3
	vmovaps	%ymm3, 256(%rsp)
	vmovaps	160(%rcx), %ymm3
	vmovaps	%ymm3, 192(%rsp)
	vmovaps	192(%rcx), %ymm3
	vmovaps	%ymm3, 96(%rsp)
	vmovaps	224(%rcx), %ymm3
	vmovaps	%ymm3, 640(%rsp)
	vmovaps	256(%rcx), %ymm3
	vmovaps	%ymm3, 800(%rsp)
	vmovaps	288(%rcx), %ymm3
	vmovaps	%ymm3, 2912(%rsp)
	vmovaps	320(%rcx), %ymm3
	vmovaps	%ymm3, 2880(%rsp)
	xorl	%eax, %eax
	vmovaps	%ymm9, %ymm14
	vmovaps	%ymm0, 32(%rsp)
	vmovaps	%ymm12, %ymm3
	vmovaps	%ymm2, %ymm15
	vmovaps	%ymm4, %ymm8
	vmovaps	352(%rcx), %ymm0
	vmovaps	%ymm0, 2848(%rsp)
	vmovaps	384(%rcx), %ymm0
	vmovaps	%ymm0, 2816(%rsp)
	vmovaps	416(%rcx), %ymm0
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	448(%rcx), %ymm0
	vmovaps	%ymm0, 2752(%rsp)
	vmovaps	480(%rcx), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	512(%rcx), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	544(%rcx), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	576(%rcx), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
	vmovaps	608(%rcx), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	640(%rcx), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	672(%rcx), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	704(%rcx), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	736(%rcx), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	768(%rcx), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	800(%rcx), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	832(%rcx), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	864(%rcx), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	896(%rcx), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	928(%rcx), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	960(%rcx), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	992(%rcx), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	1024(%rcx), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	1056(%rcx), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	1088(%rcx), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	1120(%rcx), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	1152(%rcx), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	1184(%rcx), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	1216(%rcx), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	1248(%rcx), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	1280(%rcx), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	1312(%rcx), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1344(%rcx), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	1376(%rcx), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	1408(%rcx), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	1440(%rcx), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	vmovaps	1472(%rcx), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	1504(%rcx), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	movq	%rsi, %rcx
	.p2align	4
.LBB1_228:
	vmovaps	%ymm7, %ymm9
.Ltmp1937:
	.loc	12 551 14 is_stmt 1
	vmovups	(%rcx,%rax), %ymm5
.Ltmp1938:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm5, %ymm0
	vxorps	%xmm7, %xmm7, %xmm7
.Ltmp1939:
	.loc	29 48 14
	vaddps	%ymm7, %ymm0, %ymm0
.Ltmp1940:
	.loc	29 283 14
	vmulps	608(%rsp), %ymm5, %ymm2
.Ltmp1941:
	.loc	29 48 14
	vaddps	%ymm7, %ymm2, %ymm2
.Ltmp1942:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm5, %ymm4
.Ltmp1943:
	.loc	29 48 14
	vaddps	%ymm7, %ymm4, %ymm4
	vmovaps	%ymm9, 416(%rsp)
.Ltmp1944:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm9, %ymm12
.Ltmp1945:
	.loc	29 48 14
	vaddps	%ymm0, %ymm12, %ymm0
.Ltmp1946:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm9, %ymm12
.Ltmp1947:
	.loc	29 48 14
	vaddps	%ymm2, %ymm12, %ymm2
.Ltmp1948:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm9, %ymm12
.Ltmp1949:
	.loc	29 48 14
	vaddps	%ymm4, %ymm12, %ymm12
.Ltmp1950:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm5, %ymm4
.Ltmp1951:
	.loc	29 48 14
	vaddps	%ymm7, %ymm4, %ymm4
.Ltmp1952:
	.loc	29 283 14
	vmulps	640(%rsp), %ymm9, %ymm13
.Ltmp1953:
	.loc	29 48 14
	vaddps	%ymm4, %ymm13, %ymm13
	vmovaps	%ymm11, %ymm4
	vmovaps	%ymm1, %ymm11
	vmovaps	%ymm6, %ymm9
	vmovaps	%ymm10, %ymm7
	vmovaps	128(%rsp), %ymm10
	vmovaps	%ymm14, %ymm1
.Ltmp1954:
	.loc	29 283 14
	vmulps	800(%rsp), %ymm14, %ymm14
.Ltmp1955:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1956:
	.loc	29 283 14
	vmulps	2912(%rsp), %ymm1, %ymm14
.Ltmp1957:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp1958:
	.loc	29 283 14
	vmulps	2880(%rsp), %ymm1, %ymm14
.Ltmp1959:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
	vmovaps	%ymm1, 128(%rsp)
.Ltmp1960:
	.loc	29 283 14
	vmulps	2848(%rsp), %ymm1, %ymm14
.Ltmp1961:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp1962:
	.loc	29 283 14
	vmulps	2816(%rsp), %ymm10, %ymm14
.Ltmp1963:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1964:
	.loc	29 283 14
	vmulps	2784(%rsp), %ymm10, %ymm14
.Ltmp1965:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp1966:
	.loc	29 283 14
	vmulps	2752(%rsp), %ymm10, %ymm14
.Ltmp1967:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1968:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm10, %ymm14
.Ltmp1969:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp1970:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm7, %ymm14
.Ltmp1971:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1972:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm7, %ymm14
.Ltmp1973:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp1974:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm7, %ymm14
.Ltmp1975:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1976:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm7, %ymm14
.Ltmp1977:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp1978:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm6, %ymm14
.Ltmp1979:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1980:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm6, %ymm14
.Ltmp1981:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp1982:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm6, %ymm14
.Ltmp1983:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1984:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm6, %ymm14
.Ltmp1985:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp1986:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm11, %ymm14
.Ltmp1987:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1988:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm11, %ymm14
.Ltmp1989:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp1990:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm11, %ymm14
.Ltmp1991:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp1992:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm11, %ymm14
.Ltmp1993:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp1994:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm4, %ymm14
.Ltmp1995:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp1996:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm4, %ymm14
.Ltmp1997:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm14
.Ltmp1998:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm4, %ymm2
.Ltmp1999:
	.loc	29 48 14
	vaddps	%ymm2, %ymm12, %ymm12
.Ltmp2000:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm4, %ymm2
.Ltmp2001:
	.loc	29 48 14
	vaddps	%ymm2, %ymm13, %ymm13
	vmovaps	%ymm8, %ymm2
.Ltmp2002:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm8, %ymm8
.Ltmp2003:
	.loc	29 48 14
	vaddps	%ymm0, %ymm8, %ymm0
.Ltmp2004:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm2, %ymm8
.Ltmp2005:
	.loc	29 48 14
	vaddps	%ymm14, %ymm8, %ymm8
.Ltmp2006:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm2, %ymm14
.Ltmp2007:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm14
.Ltmp2008:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm2, %ymm12
.Ltmp2009:
	.loc	29 48 14
	vaddps	%ymm13, %ymm12, %ymm13
	vmovaps	%ymm15, %ymm12
.Ltmp2010:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm15, %ymm15
.Ltmp2011:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
.Ltmp2012:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm12, %ymm0
.Ltmp2013:
	.loc	29 48 14
	vaddps	%ymm0, %ymm8, %ymm8
.Ltmp2014:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm12, %ymm0
.Ltmp2015:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm14
.Ltmp2016:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm12, %ymm0
.Ltmp2017:
	.loc	29 48 14
	vaddps	%ymm0, %ymm13, %ymm13
	vmovaps	%ymm3, %ymm0
.Ltmp2018:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm3, %ymm3
.Ltmp2019:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp2020:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm0, %ymm15
.Ltmp2021:
	.loc	29 48 14
	vaddps	%ymm8, %ymm15, %ymm8
.Ltmp2022:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm0, %ymm15
.Ltmp2023:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp2024:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm0, %ymm15
.Ltmp2025:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm15
	vmovaps	32(%rsp), %ymm13
.Ltmp2026:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm13, %ymm1
.Ltmp2027:
	.loc	29 48 14
	vaddps	%ymm3, %ymm1, %ymm1
.Ltmp2028:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm13, %ymm3
.Ltmp2029:
	.loc	29 48 14
	vaddps	%ymm3, %ymm8, %ymm3
.Ltmp2030:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm13, %ymm8
.Ltmp2031:
	.loc	29 48 14
	vaddps	%ymm14, %ymm8, %ymm8
.Ltmp2032:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm13, %ymm14
.Ltmp2033:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp2034:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm15
.Ltmp2035:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm1, %ymm15, %ymm1
.Ltmp2036:
	.loc	29 82 19
	vandps	%ymm15, %ymm11, %ymm6
.Ltmp2037:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm1, %ymm6, %ymm1
	vmovaps	%ymm7, %ymm6
	vmovaps	%ymm5, %ymm7
.Ltmp2038:
	.loc	29 82 19
	vandps	%ymm3, %ymm15, %ymm3
.Ltmp2039:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp2040:
	.loc	29 82 19
	vandps	%ymm15, %ymm8, %ymm3
.Ltmp2041:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp2042:
	.loc	29 82 19
	vandps	%ymm15, %ymm14, %ymm3
.Ltmp2043:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm1, %ymm1
.Ltmp2044:
	.loc	12 551 14
	vmovups	%ymm1, 4320(%rsp,%rax)
	vmovaps	%ymm9, %ymm1
	vmovaps	416(%rsp), %ymm9
.Ltmp2045:
	.loc	11 304 12
	addq	$32, %rax
	vmovaps	%ymm9, %ymm14
	vmovaps	%ymm0, 32(%rsp)
	vmovaps	%ymm12, %ymm3
	vmovaps	%ymm2, %ymm15
	vmovaps	%ymm4, %ymm8
	cmpq	%rax, %r8
	jne	.LBB1_228
.Ltmp2046:
.LBB1_229:
	.loc	1 1764 5
	vmovaps	%ymm7, 992(%rsp)
	vmovaps	%ymm9, 1024(%rsp)
	vmovaps	128(%rsp), %ymm3
	vmovaps	%ymm3, 1056(%rsp)
	vmovaps	%ymm10, 1088(%rsp)
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm1, 1152(%rsp)
	vmovaps	%ymm11, 1184(%rsp)
	vmovaps	%ymm4, 1216(%rsp)
	vmovaps	%ymm2, 1248(%rsp)
	vmovaps	%ymm12, 1280(%rsp)
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	%ymm13, 1344(%rsp)
.Ltmp2047:
	.loc	11 304 12
	cmpq	%rdi, 8(%rsp)
.Ltmp2048:
	.loc	1 3355 19
	jne	.LBB1_232
.Ltmp2049:
	.loc	1 0 19 is_stmt 0
	vmovaps	1728(%rsp), %ymm13
.LBB1_231:
	addq	$32, %rdi
	decq	%r9
	movq	3008(%rsp), %rsi
.Ltmp2050:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rsi
	addq	$1024, %rcx
	testq	%r9, %r9
	movq	8(%rsp), %rax
	jne	.LBB1_223
	jmp	.LBB1_338
.Ltmp2051:
.LBB1_232:
	.loc	8 0 20 is_stmt 0
	movq	%rcx, 800(%rsp)
	movq	%r9, 640(%rsp)
	vmovaps	1376(%rsp), %ymm10
	vmovaps	1408(%rsp), %ymm12
	vmovaps	1440(%rsp), %ymm9
	vmovaps	1472(%rsp), %ymm11
	vmovaps	1536(%rsp), %ymm0
	vmovaps	%ymm0, 416(%rsp)
	vmovaps	1696(%rsp), %ymm0
	vmovaps	%ymm0, 704(%rsp)
	vmovaps	1600(%rsp), %ymm3
	vmovaps	1504(%rsp), %ymm2
	vmovaps	1568(%rsp), %ymm1
	vmovaps	1664(%rsp), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	vmovaps	%ymm1, 288(%rsp)
	vmovaps	%ymm2, 256(%rsp)
	vmovaps	%ymm3, 192(%rsp)
	xorl	%ecx, %ecx
	movq	352(%rsp), %r9
	movq	320(%rsp), %r15
	xorl	%edi, %edi
	vxorps	%xmm8, %xmm8, %xmm8
	vmovaps	1728(%rsp), %ymm13
	movq	896(%rsp), %r14
	jmp	.LBB1_235
	.p2align	4
.LBB1_233:
	vmovaps	288(%rsp), %ymm15
	vmovaps	256(%rsp), %ymm4
	vmovaps	192(%rsp), %ymm5
	vmovaps	96(%rsp), %ymm6
.LBB1_234:
	movq	480(%rsp), %rcx
.Ltmp2052:
	addq	%r14, %rcx
	movq	320(%rsp), %r15
.Ltmp2053:
	.loc	1 3406 39 is_stmt 1
	addq	%r14, %r15
.Ltmp2054:
	.loc	1 1148 8
	cmpq	%r12, %r15
	movq	%r12, %rax
	movl	$0, %edi
	cmovbq	%rdi, %rax
	subq	%rax, %r15
	movq	352(%rsp), %r9
.Ltmp2055:
	.loc	1 3407 39
	addq	%r14, %r9
	movq	1760(%rsp), %rax
.Ltmp2056:
	.loc	1 1148 8
	cmpq	%rax, %r9
	cmovbq	%rdi, %rax
	subq	%rax, %r9
	vmovaps	%ymm15, 288(%rsp)
	vmovaps	%ymm4, 256(%rsp)
	vmovaps	%ymm5, 192(%rsp)
	vmovaps	%ymm6, 96(%rsp)
	movq	896(%rsp), %r14
.Ltmp2057:
	.loc	1 3355 19
	cmpq	%r14, %rcx
	jae	.LBB1_253
.LBB1_235:
	.loc	1 0 19 is_stmt 0
	movq	%rcx, 480(%rsp)
	.loc	1 3363 21 is_stmt 1
	subq	%rcx, %r14
	movq	88(%rsp), %rax
.Ltmp2058:
	.loc	1 1575 16
	movq	1624(%rax), %rcx
.Ltmp2059:
	.loc	1 1576 16
	movq	1632(%rax), %r8
.Ltmp2060:
	.loc	1 1577 25
	leaq	1(%r15), %rax
.Ltmp2061:
	.loc	1 1148 8
	cmpq	%rcx, %rax
	movq	%rcx, %rsi
	cmovbq	%rdi, %rsi
	negq	%rsi
	movq	3040(%rsp), %rax
	xorl	%r11d, %r11d
.Ltmp2062:
	.loc	1 1578 28
	leaq	(%r15,%rax), %rbx
.Ltmp2063:
	.loc	1 1148 8
	cmpq	%rcx, %rbx
	movq	%rcx, %rax
	cmovbq	%r11, %rax
	subq	%rax, %rbx
	movq	3072(%rsp), %rax
.Ltmp2064:
	.loc	1 1580 33
	addq	%r15, %rax
.Ltmp2065:
	.loc	1 1148 8
	cmpq	%rcx, %rax
	movq	%rcx, %rdi
	cmovbq	%r11, %rdi
	subq	%rdi, %rax
.Ltmp2066:
	.loc	1 1583 14
	movq	%rcx, %r11
	subq	%r15, %r11
.Ltmp2067:
	.loc	10 1078 5
	cmpq	%r14, %r11
	cmovbq	%r11, %r14
	movq	%r9, 352(%rsp)
.Ltmp2068:
	.loc	1 1584 14
	subq	%r9, %r8
.Ltmp2069:
	.loc	10 1078 5
	cmpq	%r14, %r8
	cmovbq	%r8, %r14
	movq	%r15, 320(%rsp)
.Ltmp2070:
	.loc	1 1148 8
	leaq	(%r15,%rsi), %rdi
	incq	%rdi
.Ltmp2071:
	.loc	1 1585 14
	movq	%rcx, %r9
	movq	%rdi, 608(%rsp)
	subq	%rdi, %r9
.Ltmp2072:
	.loc	10 1078 5
	cmpq	%r14, %r9
	cmovbq	%r9, %r14
.Ltmp2073:
	.loc	1 1586 14
	movq	%rcx, %r15
	subq	%rbx, %r15
.Ltmp2074:
	.loc	10 1078 5
	cmpq	%r14, %r15
	cmovbq	%r15, %r14
.Ltmp2075:
	.loc	1 1588 14
	subq	%rax, %rcx
.Ltmp2076:
	.loc	10 1078 5
	cmpq	%r14, %rcx
	cmovbq	%rcx, %r14
	movq	928(%rsp), %rsi
	movq	480(%rsp), %rdi
.Ltmp2077:
	.loc	1 3369 28
	addq	%rsi, %rdi
.Ltmp2078:
	.loc	1 3371 55
	leaq	(%r14,%rdi), %rsi
.Ltmp2079:
	.loc	1 3369 28
	shlq	$3, %rdi
.Ltmp2080:
	.loc	1 3371 55
	shlq	$3, %rsi
.Ltmp2081:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB1_379
	cmpq	80(%rsp), %rsi
	ja	.LBB1_379
.Ltmp2082:
	.loc	11 304 12
	testq	%r14, %r14
	je	.LBB1_233
.Ltmp2083:
	.loc	11 0 12 is_stmt 0
	movq	480(%rsp), %rsi
	shlq	$5, %rsi
.Ltmp2084:
	.loc	11 304 12
	cmpq	%rcx, %r15
	cmovbq	%r15, %rcx
	cmpq	%r9, %rcx
	cmovaeq	%r9, %rcx
	movq	24(%rsp), %r9
.Ltmp2085:
	.loc	1 0 0
	leaq	(%r9,%rdi,4), %rdi
	movq	%rdi, 32(%rsp)
.Ltmp2086:
	.loc	11 304 12
	cmpq	%r11, %rcx
	cmovaeq	%r11, %rcx
	cmpq	%r8, %rcx
	cmovaeq	%r8, %rcx
	movq	72(%rsp), %r15
	subq	480(%rsp), %r15
	cmpq	%r15, %rcx
	cmovbq	%rcx, %r15
.Ltmp2087:
	.loc	1 0 0
	leaq	(%rsp,%rsi), %rcx
	addq	$4320, %rcx
	movq	%rcx, 128(%rsp)
.Ltmp2088:
	.loc	11 304 12
	movabsq	$2305843009213693951, %rcx
	andq	%rcx, %r15
	vmovaps	%ymm0, %ymm6
	vmovaps	%ymm1, %ymm15
	vmovaps	%ymm2, %ymm4
	vmovaps	%ymm3, %ymm5
	xorl	%ecx, %ecx
	vmovaps	%ymm13, %ymm7
.Ltmp2089:
	.loc	11 0 12
.Ltmp2090:
	.p2align	4
.LBB1_239:
	movq	320(%rsp), %rsi
.Ltmp2091:
	.loc	1 1502 26 is_stmt 1
	leaq	(%rcx,%rsi), %r8
.Ltmp2092:
	.loc	1 1137 16
	leaq	(,%r8,8), %rdi
.Ltmp2093:
	.loc	1 1138 33
	leaq	8(,%r8,8), %rsi
.Ltmp2094:
	.loc	4 1050 16
	leaq	7(,%r8,8), %r8
	cmpq	%rdx, %r8
	jae	.LBB1_332
.Ltmp2095:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rcx,8), %r8
.Ltmp2096:
	vbroadcastss	.LCPI1_1(%rip), %ymm0
	vaddps	%ymm0, %ymm11, %ymm0
	vmaxps	%ymm8, %ymm0, %ymm11
	vcmpgt_oqps	%ymm8, %ymm11, %ymm0
	vaddps	%ymm10, %ymm9, %ymm1
	vblendvps	%ymm0, %ymm1, %ymm12, %ymm10
	movq	128(%rsp), %r9
.Ltmp2097:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r9,%r8,4), %ymm1
.Ltmp2098:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm1, %ymm2
	vmovaps	448(%rsp), %ymm3
.Ltmp2099:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm2, %ymm1, %ymm1
.Ltmp2100:
	.loc	29 360 14
	vdivps	%ymm1, %ymm10, %ymm2
.Ltmp2101:
	.loc	1 0 0 is_stmt 0
	leaq	(%rcx,%rbx), %r9
.Ltmp2102:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm10, %ymm1, %ymm1
	vbroadcastss	.LCPI1_2(%rip), %ymm3
.Ltmp2103:
	.loc	29 585 19
	vblendvps	%ymm1, %ymm2, %ymm3, %ymm1
.Ltmp2104:
	.loc	12 551 14
	vmovups	%ymm1, (%r13,%rdi,4)
.Ltmp2105:
	.loc	1 1130 16
	leaq	(,%r9,8), %r8
.Ltmp2106:
	.loc	4 1050 16
	leaq	7(,%r9,8), %r11
	cmpq	%rdx, %r11
	jae	.LBB1_254
.Ltmp2107:
	.loc	12 551 14
	vmovups	(%r13,%r8,4), %ymm1
	vmovaps	%ymm1, %ymm13
.Ltmp2108:
	.loc	1 1206 22
	testl	%r10d, %r10d
	je	.LBB1_243
.Ltmp2109:
	.loc	29 257 14
	vminps	%ymm1, %ymm7, %ymm13
.Ltmp2110:
.LBB1_243:
	.loc	1 0 0 is_stmt 0
	movl	%r10d, %r10d
.Ltmp2111:
	.loc	1 1212 20 is_stmt 1
	incq	%r10
	movq	384(%rsp), %r8
	movq	%r8, %r11
	cmpq	%r8, %r10
.Ltmp2112:
	.loc	1 1213 22
	jne	.LBB1_247
	.loc	1 0 22 is_stmt 0
.Ltmp2113:
	.p2align	4
.LBB1_244:
.Ltmp2114:
	.loc	1 1130 16 is_stmt 1
	leaq	(,%r9,8), %r8
.Ltmp2115:
	.loc	4 1050 16
	leaq	7(,%r9,8), %r10
	cmpq	%rdx, %r10
	jae	.LBB1_254
.Ltmp2116:
	.loc	29 257 14
	vminps	(%r13,%r8,4), %ymm1, %ymm1
.Ltmp2117:
	.loc	12 551 14
	vmovups	%ymm1, (%r13,%r8,4)
.Ltmp2118:
	.loc	1 1224 16
	testq	%r9, %r9
	cmoveq	%r12, %r9
	.loc	1 1227 13
	decq	%r9
.Ltmp2119:
	.loc	10 1916 50
	decq	%r11
.Ltmp2120:
	.loc	3 900 12
	jne	.LBB1_244
.Ltmp2121:
	.loc	3 0 12 is_stmt 0
	xorl	%r10d, %r10d
	vmovaps	%ymm13, %ymm2
	jmp	.LBB1_249
	.p2align	4
.LBB1_247:
	movq	608(%rsp), %r8
	addq	%rcx, %r8
.Ltmp2122:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r8,8), %r9
.Ltmp2123:
	.loc	1 1130 16
	shlq	$3, %r8
.Ltmp2124:
	.loc	4 1050 16
	cmpq	%rdx, %r9
	jae	.LBB1_254
.Ltmp2125:
	.loc	12 551 14
	vmovups	(%r13,%r8,4), %ymm1
.Ltmp2126:
	.loc	29 257 14
	vminps	%ymm13, %ymm1, %ymm2
.Ltmp2127:
.LBB1_249:
	.loc	1 0 0 is_stmt 0
	leaq	(%rcx,%rax), %r8
.Ltmp2128:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r8,8), %r9
.Ltmp2129:
	.loc	1 1130 16
	shlq	$3, %r8
	movq	736(%rsp), %r11
.Ltmp2130:
	.loc	4 1050 16
	cmpq	%r11, %r9
	jae	.LBB1_333
.Ltmp2131:
	.loc	4 0 16 is_stmt 0
	cmpq	%r11, %rsi
.Ltmp2132:
	.loc	4 1050 16
	ja	.LBB1_334
.Ltmp2133:
	.loc	1 0 0
	vbroadcastss	.LCPI1_1(%rip), %ymm1
	vaddps	%ymm1, %ymm5, %ymm1
	vmaxps	%ymm8, %ymm1, %ymm5
	vcmpgt_oqps	%ymm8, %ymm5, %ymm1
	vxorps	%xmm14, %xmm14, %xmm14
	vmovaps	%ymm3, %ymm8
	vaddps	%ymm4, %ymm15, %ymm3
	vmovaps	416(%rsp), %ymm4
	vblendvps	%ymm1, %ymm3, %ymm4, %ymm4
.Ltmp2134:
	vbroadcastss	.LCPI1_3(%rip), %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm2
	vbroadcastss	.LCPI1_4(%rip), %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
.Ltmp2135:
	vaddps	%ymm2, %ymm6, %ymm3
	movq	864(%rsp), %r9
	vsubps	(%r9,%r8,4), %ymm3, %ymm6
	movq	352(%rsp), %rsi
.Ltmp2136:
	leaq	(%rcx,%rsi), %r8
.Ltmp2137:
	.loc	29 360 14 is_stmt 1
	vdivps	704(%rsp), %ymm6, %ymm3
.Ltmp2138:
	.loc	12 551 14
	vmovups	%ymm2, (%r9,%rdi,4)
.Ltmp2139:
	.loc	1 1659 43
	vmovaps	1632(%rsp), %ymm2
.Ltmp2140:
	.loc	29 347 14
	vsubps	%ymm3, %ymm8, %ymm3
.Ltmp2141:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm2, %ymm3, %ymm7
.Ltmp2142:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm7, %ymm4, %ymm7
.Ltmp2143:
	.loc	29 48 14
	vaddps	%ymm7, %ymm2, %ymm2
.Ltmp2144:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm7
.Ltmp2145:
	.loc	29 233 14
	vmaxps	%ymm2, %ymm3, %ymm2
.Ltmp2146:
	.loc	29 82 19
	vandps	%ymm7, %ymm2, %ymm3
.Ltmp2147:
	.loc	29 871 14
	vbroadcastss	.LCPI1_5(%rip), %ymm7
	vcmplt_oqps	%ymm7, %ymm3, %ymm3
.Ltmp2148:
	.loc	29 82 19
	vandnps	%ymm2, %ymm3, %ymm2
.Ltmp2149:
	.loc	1 1660 5
	vmovaps	%ymm2, 1632(%rsp)
.Ltmp2150:
	.loc	4 1050 16
	leaq	7(,%r8,8), %rsi
.Ltmp2151:
	.loc	1 1130 16
	shlq	$3, %r8
.Ltmp2152:
	.loc	4 1050 16
	cmpq	672(%rsp), %rsi
	jae	.LBB1_335
.Ltmp2153:
	.loc	1 0 0 is_stmt 0
	movq	%rcx, %rsi
	shlq	$5, %rsi
	addq	32(%rsp), %rsi
	incq	%rcx
.Ltmp2154:
	vblendvps	%ymm0, %ymm9, %ymm14, %ymm9
	vblendvps	%ymm1, %ymm15, %ymm14, %ymm15
.Ltmp2155:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm2, %ymm8, %ymm0
	movq	832(%rsp), %rdi
.Ltmp2156:
	.loc	12 551 14
	vmovups	(%rdi,%r8,4), %ymm1
.Ltmp2157:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rsi), %ymm2
.Ltmp2158:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm1, %ymm0, %ymm0
	vmovaps	224(%rsp), %ymm3
.Ltmp2159:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm1, %ymm0, %ymm0
.Ltmp2160:
	.loc	12 551 14
	vmovups	%ymm2, (%rdi,%r8,4)
.Ltmp2161:
	.loc	12 551 14 is_stmt 0
	vmovups	%ymm0, (%rsi)
	vmovaps	%ymm13, %ymm7
	vmovaps	%ymm6, %ymm0
	vmovaps	%ymm15, %ymm1
	vmovaps	%ymm4, %ymm2
	vmovaps	%ymm5, %ymm3
.Ltmp2162:
	.loc	11 304 12 is_stmt 1
	cmpq	%r15, %rcx
	vxorps	%xmm8, %xmm8, %xmm8
	jne	.LBB1_239
	jmp	.LBB1_234
.Ltmp2163:
.LBB1_253:
	.loc	11 0 12 is_stmt 0
	movq	%r15, 320(%rsp)
	movq	%r9, 352(%rsp)
.Ltmp2164:
	.loc	1 1654 5 is_stmt 1
	vmovaps	%ymm6, 1664(%rsp)
.Ltmp2165:
	.loc	1 851 0
	vmovaps	%ymm5, 1600(%rsp)
.Ltmp2166:
	.loc	1 853 0
	vmovaps	%ymm4, 1504(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm15, 1568(%rsp)
	vmovaps	%ymm11, 1472(%rsp)
	vmovaps	%ymm10, 1376(%rsp)
	vmovaps	%ymm9, 1440(%rsp)
	movq	80(%rsp), %r15
	movq	24(%rsp), %rbx
	movq	640(%rsp), %r9
	movl	$32, %r14d
	movq	800(%rsp), %rcx
	movq	928(%rsp), %rdi
	jmp	.LBB1_231
.Ltmp2167:
.LBB1_254:
	vmovaps	96(%rsp), %ymm0
.Ltmp2168:
	.loc	1 1654 5 is_stmt 1
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp2169:
	.loc	1 851 0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2170:
	.loc	1 853 0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	288(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp2171:
	leaq	8(%r8), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2172:
.LBB1_255:
	movq	8(%rsp), %rdx
	leaq	(,%rdx,8), %rsi
	cmpq	%r15, %rsi
.Ltmp2173:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB1_387
.Ltmp2174:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	%r11, %rsi
	.p2align	4
.LBB1_257:
.Ltmp2175:
	.loc	15 1504 12 is_stmt 1
	testq	%rdx, %rdx
	je	.LBB1_278
.Ltmp2176:
	.loc	10 1078 5
	cmpq	$32, %rdx
	movl	$32, %edi
	cmovbq	%rdx, %rdi
.Ltmp2177:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp2178:
	.loc	16 0 18 is_stmt 0
.Ltmp2179:
	.p2align	4
.LBB1_259:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp2180:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp2181:
	.loc	6 180 28
	jne	.LBB1_259
.Ltmp2182:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp2183:
	.loc	19 2054 74
	subq	%rdi, %rdx
.Ltmp2184:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB1_257
	jmp	.LBB1_39
.Ltmp2185:
.LBB1_262:
	.loc	1 3411 31
	movl	3208(%rsp), %r10d
	movq	80(%rsp), %r15
	movq	24(%rsp), %rbx
.LBB1_263:
	.loc	1 3411 10
	vmovaps	3104(%rsp), %ymm0
	vmovaps	%ymm0, 512(%rsp)
.Ltmp2186:
	.loc	1 3414 23
	movq	1736(%r13), %rdx
.Ltmp2187:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_271
.Ltmp2188:
	.loc	1 3414 23
	movq	1728(%r13), %rax
.Ltmp2189:
	.loc	12 551 14
	vmovaps	512(%rsp), %ymm0
	vmovups	%ymm0, (%rax)
.Ltmp2190:
	.loc	1 3415 5
	movq	1768(%r13), %rax
.Ltmp2191:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp2192:
	.loc	6 180 28
	je	.LBB1_267
.Ltmp2193:
	.loc	6 0 28 is_stmt 0
	movq	1760(%r13), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB1_266:
.Ltmp2194:
	.loc	20 66 21 is_stmt 1
	movl	%r10d, (%rcx,%rdx)
.Ltmp2195:
	.loc	7 1714 9
	addq	$4, %rdx
	cmpq	%rdx, %rax
.Ltmp2196:
	.loc	6 180 28
	jne	.LBB1_266
.Ltmp2197:
.LBB1_267:
	.loc	6 0 28 is_stmt 0
	leaq	992(%rsp), %rdi
	movq	16(%rsp), %rsi
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	352(%rsp), %rax
	movl	%eax, 1640(%r13)
	movq	320(%rsp), %rax
	movl	%eax, 1644(%r13)
.Ltmp2198:
	.loc	1 2327 13 is_stmt 1
	cmpb	$0, 604(%rsp)
	je	.LBB1_312
	jmp	.LBB1_304
.LBB1_268:
	.loc	1 0 13 is_stmt 0
	movq	%r15, %rbx
.Ltmp2199:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm3, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2200:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm6, 3104(%rsp)
.Ltmp2201:
.LBB1_269:
	leaq	992(%rsp), %r15
	leaq	3104(%rsp), %rsi
	.loc	1 3306 5 is_stmt 1
	movl	$736, %edx
	movq	%r15, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	.loc	1 3306 14 is_stmt 0
	movq	%r15, %rdi
	movq	16(%rsp), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	.loc	1 3307 5 is_stmt 1
	movl	%r14d, 1640(%r13)
	movq	32(%rsp), %rax
	.loc	1 3308 5
	movl	%eax, 1644(%r13)
	movq	%rbx, %r15
	movq	24(%rsp), %rbx
.Ltmp2202:
	.loc	1 2327 13
	cmpb	$0, 604(%rsp)
	je	.LBB1_312
.LBB1_304:
	.loc	1 0 13 is_stmt 0
	movq	16(%rsp), %rdi
	.loc	1 2327 32
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2327 22
	testb	%al, %al
	je	.LBB1_312
	.loc	1 0 22
	movq	2952(%rsp), %rsi
	cmpq	%r15, %rsi
.Ltmp2203:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB1_388
.Ltmp2204:
	.loc	4 0 16 is_stmt 0
	movq	%rbx, %rax
	.p2align	4
.LBB1_307:
.Ltmp2205:
	.loc	17 131 18 is_stmt 1
	movq	%rsi, %rcx
.Ltmp2206:
	.loc	15 1504 12
	testq	%rsi, %rsi
	je	.LBB1_311
.Ltmp2207:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp2208:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%r9d, %r9d
	xorl	%r8d, %r8d
.Ltmp2209:
	.loc	16 0 18 is_stmt 0
.Ltmp2210:
	.p2align	4
.LBB1_309:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r8), %r9d
.Ltmp2211:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp2212:
	.loc	6 180 28
	jne	.LBB1_309
.Ltmp2213:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp2214:
	.loc	19 2054 74
	movq	%rcx, %rsi
	subq	%rdx, %rsi
.Ltmp2215:
	.loc	17 136 12
	testl	%r9d, %r9d
	je	.LBB1_307
.Ltmp2216:
.LBB1_311:
	.loc	15 1504 12
	testq	%rcx, %rcx
	sete	%al
	jmp	.LBB1_313
.Ltmp2217:
.LBB1_270:
	.loc	15 0 12 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp2218:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2219:
.LBB1_271:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_272:
	leaq	992(%rsp), %rdi
	movq	16(%rsp), %r15
.Ltmp2220:
	.loc	1 3327 24 is_stmt 1
	movq	%r15, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp2221:
	.loc	1 3332 19
	movzbl	1536(%r13), %r14d
.Ltmp2222:
	.loc	1 3333 21
	movzbl	1537(%r13), %eax
	movb	%al, 128(%rsp)
.Ltmp2223:
	.loc	1 3334 16
	movq	1624(%r13), %r12
.Ltmp2224:
	.loc	1 3335 16
	movq	1632(%r13), %rbx
.Ltmp2225:
	.loc	1 3336 27
	movl	1640(%r13), %eax
	movq	%rax, 352(%rsp)
.Ltmp2226:
	.loc	1 3337 27
	movl	1644(%r13), %eax
	movq	%rax, 320(%rsp)
	leaq	4320(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	3104(%rsp), %rdi
.Ltmp2227:
	.loc	1 3341 32
	movq	%r15, %rsi
	movq	%r12, %rdx
	movq	%rbx, 1728(%rsp)
	movq	%rbx, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	8(%rsp), %rax
.Ltmp2228:
	.loc	4 3758 16
	leaq	31(%rax), %rcx
	shrq	$5, %rcx
	vmovaps	992(%rsp), %ymm13
	vmovaps	3104(%rsp), %ymm12
.Ltmp2229:
	.loc	8 446 20
	je	.LBB1_292
.Ltmp2230:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm6
.Ltmp2231:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm6, %ymm0
	vmovaps	%ymm6, %ymm7
	testb	%r14b, %r14b
	movq	80(%rsp), %r15
	jne	.LBB1_275
.Ltmp2232:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, %ymm7
.LBB1_275:
	vmovaps	%ymm13, 2976(%rsp)
	cmpb	$0, 128(%rsp)
	jne	.LBB1_277
	vmovaps	%ymm0, %ymm6
.LBB1_277:
	vmovaps	1024(%rsp), %ymm15
	vmovaps	1056(%rsp), %ymm4
	vmovaps	1088(%rsp), %ymm0
	vmovaps	1120(%rsp), %ymm9
	vmovaps	1152(%rsp), %ymm2
	vmovaps	1184(%rsp), %ymm10
	vmovaps	1216(%rsp), %ymm13
	vmovaps	1248(%rsp), %ymm5
	vmovaps	1280(%rsp), %ymm14
	vmovaps	1312(%rsp), %ymm1
	vmovaps	%ymm1, 32(%rsp)
	vmovaps	1344(%rsp), %ymm8
	movl	3208(%rsp), %r11d
	vmovaps	1664(%rsp), %ymm3
	movq	3192(%rsp), %rax
	movq	%rax, 3072(%rsp)
	movq	3200(%rsp), %rax
	movq	%rax, 72(%rsp)
	vmovaps	1376(%rsp), %ymm1
	vmovaps	%ymm1, 224(%rsp)
	vmovaps	1504(%rsp), %ymm1
	vmovaps	%ymm1, 864(%rsp)
	movq	3136(%rsp), %r13
	movq	3144(%rsp), %rdx
	movq	3184(%rsp), %rax
	movq	%rax, 736(%rsp)
	movq	3152(%rsp), %rax
	movq	%rax, 832(%rsp)
	movq	3160(%rsp), %rax
	movq	%rax, 704(%rsp)
	vmovaps	1696(%rsp), %ymm1
	vmovaps	%ymm1, 608(%rsp)
	vmovaps	%ymm4, %ymm1
	movq	3176(%rsp), %rax
	movq	%rax, 672(%rsp)
	movq	3168(%rsp), %rax
	movq	%rax, 480(%rsp)
	movl	$32, %r10d
	movq	24(%rsp), %rbx
	movq	%rbx, %r14
	movq	8(%rsp), %rax
	movq	%rax, %rsi
	xorl	%r9d, %r9d
	vmovaps	%ymm6, 3936(%rsp)
	vmovaps	%ymm7, 3904(%rsp)
.Ltmp2233:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB1_345
.Ltmp2234:
.LBB1_278:
	.loc	8 0 20 is_stmt 0
	movb	$1, %r9b
	.loc	1 2309 12 is_stmt 1
	cmpb	$0, 2152(%r13)
	je	.LBB1_40
.Ltmp2235:
	.loc	1 663 31
	movq	1768(%r13), %rcx
	.loc	1 663 57 is_stmt 0
	movq	1832(%r13), %rax
.Ltmp2236:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp2237:
	.loc	11 304 12
	testq	%rcx, %rcx
	movq	8(%rsp), %r11
	je	.LBB1_287
.Ltmp2238:
	.loc	11 0 12 is_stmt 0
	movq	1760(%r13), %rsi
	movq	1824(%r13), %rdi
	xorl	%r8d, %r8d
	jmp	.LBB1_283
	.p2align	4
.LBB1_281:
.Ltmp2239:
	.loc	1 665 22 is_stmt 1
	xorl	%edx, %edx
	divq	%r9
.LBB1_282:
	.loc	1 665 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp2240:
	.loc	1 0 0
	incq	%r8
.Ltmp2241:
	.loc	11 304 12 is_stmt 1
	addq	$12, %rdi
	cmpq	%r8, %rcx
	je	.LBB1_287
.Ltmp2242:
.LBB1_283:
	.loc	1 664 26
	movl	(%rdi), %r9d
	testq	%r9, %r9
.Ltmp2243:
	.loc	1 665 42
	je	.LBB1_416
	.loc	1 665 24 is_stmt 0
	movl	(%rsi,%r8,4), %r10d
	.loc	1 665 42
	movl	%r11d, %eax
	xorl	%edx, %edx
	divl	%r9d
	movl	%edx, %eax
	.loc	1 665 23
	addq	%r10, %rax
	.loc	1 665 22
	btq	$32, %rax
	jb	.LBB1_281
	xorl	%edx, %edx
	divl	%r9d
	jmp	.LBB1_282
.Ltmp2244:
.LBB1_286:
	.loc	1 0 22
	movq	24(%rsp), %rbx
.Ltmp2245:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB1_302
.Ltmp2246:
.LBB1_287:
	.loc	1 2311 26
	movq	1632(%r13), %rsi
.Ltmp2247:
	.loc	1 455 44
	testq	%rsi, %rsi
	je	.LBB1_427
.Ltmp2248:
	.loc	1 2311 26
	movq	1624(%r13), %rcx
.Ltmp2249:
	.loc	1 455 23
	movl	1640(%r13), %edi
	movq	%r11, %rax
	.loc	1 455 44 is_stmt 0
	cmpq	%rsi, %r11
	jb	.LBB1_290
	movl	%r11d, %eax
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB1_290:
	.loc	1 455 22
	addq	%rdi, %rax
	.loc	1 455 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB1_325
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB1_326
.Ltmp2250:
.LBB1_292:
	.loc	1 3411 31 is_stmt 1
	movl	3208(%rsp), %r11d
	movq	80(%rsp), %r15
	movq	24(%rsp), %rbx
.LBB1_293:
.Ltmp2251:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 992(%rsp)
	vmovaps	%ymm12, 3104(%rsp)
.Ltmp2252:
	.loc	1 3411 10 is_stmt 1
	vmovaps	3104(%rsp), %ymm0
	vmovaps	%ymm0, 512(%rsp)
.Ltmp2253:
	.loc	1 3414 23
	movq	1736(%r13), %rdx
.Ltmp2254:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB1_271
.Ltmp2255:
	.loc	1 3414 23
	movq	1728(%r13), %rax
.Ltmp2256:
	.loc	12 551 14
	vmovaps	512(%rsp), %ymm0
	vmovups	%ymm0, (%rax)
.Ltmp2257:
	.loc	1 3415 5
	movq	1768(%r13), %rax
.Ltmp2258:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp2259:
	.loc	6 180 28
	je	.LBB1_267
.Ltmp2260:
	.loc	6 0 28 is_stmt 0
	movq	1760(%r13), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB1_296:
.Ltmp2261:
	.loc	20 66 21 is_stmt 1
	movl	%r11d, (%rcx,%rdx)
.Ltmp2262:
	.loc	7 1714 9
	addq	$4, %rdx
	cmpq	%rdx, %rax
.Ltmp2263:
	.loc	6 180 28
	jne	.LBB1_296
	jmp	.LBB1_267
.Ltmp2264:
.LBB1_297:
	.loc	6 0 28 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp2265:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2266:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_754b353a9c273ca837480913d0661c80(%rip), %rcx
	movq	%r15, %rsi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_298:
	vmovaps	96(%rsp), %ymm0
.Ltmp2267:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2268:
	.loc	5 581 13
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
	movq	448(%rsp), %rdi
.Ltmp2269:
	.loc	5 581 13 is_stmt 0
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2270:
.LBB1_299:
	.loc	5 0 13
	vmovaps	96(%rsp), %ymm0
.Ltmp2271:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2272:
	.loc	5 581 13
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	448(%rsp), %rdi
.Ltmp2273:
	.loc	5 581 13 is_stmt 0
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2274:
.LBB1_300:
	.loc	5 0 13
	vmovaps	96(%rsp), %ymm0
.Ltmp2275:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2276:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_301:
	movq	%r8, %r12
.Ltmp2277:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2278:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm13, 1024(%rsp)
	vmovaps	%ymm0, 1056(%rsp)
	vmovaps	%ymm9, 1088(%rsp)
	vmovaps	960(%rsp), %ymm3
.LBB1_302:
	vmovaps	%ymm3, 992(%rsp)
	leaq	992(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp2279:
	.loc	1 3306 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	.loc	1 3307 5
	movl	%r12d, 1640(%r13)
	movq	32(%rsp), %rax
.Ltmp2280:
	.loc	1 0 0 is_stmt 0
	movl	%eax, 1644(%r13)
.Ltmp2281:
	.loc	1 2327 13 is_stmt 1
	cmpb	$0, 604(%rsp)
	jne	.LBB1_304
.LBB1_312:
	.loc	1 0 13 is_stmt 0
	xorl	%eax, %eax
.LBB1_313:
	.loc	1 2326 9 is_stmt 1
	movb	%al, 2152(%r13)
	.loc	1 2328 30
	movzbl	2144(%r13), %eax
	.loc	1 2328 9 is_stmt 0
	movb	%al, 2153(%r13)
.Ltmp2282:
	.loc	29 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
	movabsq	$2305843009213693951, %rax
.Ltmp2283:
	.loc	15 1851 23
	addq	$-7, %rax
.Ltmp2284:
	.loc	19 2155 12
	andq	%r15, %rax
	je	.LBB1_319
.Ltmp2285:
	.loc	19 0 12 is_stmt 0
	vbroadcastss	.LCPI1_0(%rip), %ymm1
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI1_6(%rip), %ymm2
	vmovaps	%ymm0, %ymm3
	.p2align	4
.LBB1_315:
.Ltmp2286:
	.loc	29 82 19 is_stmt 1
	vandps	(%rbx,%rcx,4), %ymm1, %ymm4
.Ltmp2287:
	.loc	29 871 14
	vcmplt_oqps	%ymm2, %ymm4, %ymm4
.Ltmp2288:
	.loc	29 82 19
	vandps	%ymm4, %ymm3, %ymm3
.Ltmp2289:
	.loc	19 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB1_315
.Ltmp2290:
	.file	30 "/home/bl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wide-1.6.1" "src/f32x8_.rs"
	.loc	30 176 9
	vpcmpeqd	%ymm4, %ymm4, %ymm4
	vtestps	%ymm4, %ymm3
.Ltmp2291:
	.loc	1 2329 12
	jb	.LBB1_323
	.loc	1 0 12 is_stmt 0
	xorl	%ecx, %ecx
	.p2align	4
.LBB1_318:
.Ltmp2292:
	.loc	29 82 19 is_stmt 1
	vandps	(%rbx,%rcx,4), %ymm1, %ymm3
.Ltmp2293:
	.loc	29 871 14
	vcmplt_oqps	%ymm2, %ymm3, %ymm3
.Ltmp2294:
	.loc	29 82 19
	vandps	%ymm3, %ymm0, %ymm0
.Ltmp2295:
	.loc	19 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB1_318
	jmp	.LBB1_320
.Ltmp2296:
.LBB1_319:
	.loc	30 176 9
	vpcmpeqd	%ymm1, %ymm1, %ymm1
	vtestps	%ymm1, %ymm0
.Ltmp2297:
	.loc	1 2329 12
	jb	.LBB1_323
.LBB1_320:
.Ltmp2298:
	.loc	29 585 19
	vpbroadcastd	.LCPI1_2(%rip), %ymm1
	vpsrad	$31, %ymm0, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
.Ltmp2299:
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
.Ltmp2300:
	.loc	1 2332 9
	movl	%edx, 1576(%r13)
	.loc	1 2333 40
	movq	1568(%r13), %rax
.Ltmp2301:
	.loc	4 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp2302:
	.loc	1 2333 9
	movq	%rcx, 1568(%r13)
.Ltmp2303:
	.loc	7 1714 9
	testq	%r15, %r15
.Ltmp2304:
	.loc	6 180 28
	je	.LBB1_322
.Ltmp2305:
	.loc	16 961 18
	shlq	$2, %r15
.Ltmp2306:
	.loc	20 25 13
	movq	%rbx, %rdi
	xorl	%esi, %esi
	movq	%r15, %rdx
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp2307:
.LBB1_322:
	.loc	20 0 13 is_stmt 0
	movq	2960(%rsp), %rcx
	.loc	1 2335 21 is_stmt 1
	movq	16(%rcx), %rax
	movq	%rax, 4336(%rsp)
	vmovups	(%rcx), %xmm0
	vmovaps	%xmm0, 4320(%rsp)
.Ltmp2308:
	.loc	1 2336 20
	movl	2120(%r13), %ebx
.Ltmp2309:
	.loc	1 2338 40
	movq	1584(%r13), %rdx
	movq	1592(%r13), %rcx
	.loc	1 2338 14 is_stmt 0
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %r14
	leaq	4320(%rsp), %r15
	movq	16(%rsp), %rdi
	movq	%r15, %rsi
	movl	%ebx, %r8d
	vzeroupper
	callq	*%r14
	.loc	1 2340 40 is_stmt 1
	movq	1600(%r13), %rdx
	movq	1608(%r13), %rcx
	movq	184(%rsp), %rdi
	.loc	1 2340 14 is_stmt 0
	movq	%r15, %rsi
	movl	%ebx, %r8d
	callq	*%r14
	.loc	1 2341 9 is_stmt 1
	movq	$0, 1640(%r13)
.Ltmp2310:
.LBB1_323:
	.loc	1 0 9 is_stmt 0
	leaq	3968(%rsp), %rsi
	.loc	1 3150 9 is_stmt 1
	movl	$328, %edx
	movq	2968(%rsp), %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp2311:
	.loc	1 3151 6
	leaq	-40(%rbp), %rsp
	.loc	1 3151 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB1_324:
	.cfi_def_cfa %rbp, 16
.Ltmp2312:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2313:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2314:
.LBB1_325:
	.loc	1 455 21 is_stmt 1
	xorl	%edx, %edx
	divl	%esi
.LBB1_326:
	.loc	1 455 9 is_stmt 0
	movl	%edx, 1640(%r13)
	.loc	1 456 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB1_428
	.loc	1 456 23 is_stmt 0
	movl	1644(%r13), %esi
	.loc	1 456 44
	cmpq	%rcx, %r11
	jb	.LBB1_329
	movl	%r11d, %eax
	xorl	%edx, %edx
	divl	%ecx
	movl	%edx, %r11d
.LBB1_329:
	.loc	1 456 22
	addq	%rsi, %r11
	.loc	1 456 21
	movq	%r11, %rax
	orq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB1_331
	movq	%r11, %rax
	xorl	%edx, %edx
	divq	%rcx
	.loc	1 456 9
	movl	%edx, 1644(%r13)
.Ltmp2315:
	.loc	1 0 0
	jmp	.LBB1_323
.LBB1_331:
.Ltmp2316:
	.loc	1 456 21
	movl	%r11d, %eax
	xorl	%edx, %edx
	divl	%ecx
	.loc	1 456 9
	movl	%edx, 1644(%r13)
.Ltmp2317:
	.loc	1 0 0
	jmp	.LBB1_323
.LBB1_332:
	vmovaps	96(%rsp), %ymm0
.Ltmp2318:
	.loc	1 1654 5 is_stmt 1
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp2319:
	.loc	1 851 0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2320:
	.loc	1 853 0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	288(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp2321:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2322:
.LBB1_333:
	.loc	5 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp2323:
	.loc	1 1654 5 is_stmt 1
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp2324:
	.loc	1 851 0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2325:
	.loc	1 853 0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	288(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp2326:
	.loc	1 1131 25 is_stmt 1
	leaq	8(%r8), %rsi
.Ltmp2327:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r8, %rdi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2328:
.LBB1_334:
	.loc	5 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp2329:
	.loc	1 1654 5 is_stmt 1
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp2330:
	.loc	1 851 0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2331:
	.loc	1 853 0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	288(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp2332:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2333:
.LBB1_335:
	.loc	5 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
	.loc	1 1654 5 is_stmt 1
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp2334:
	.loc	1 851 0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2335:
	.loc	1 853 0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	288(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp2336:
	.loc	1 1131 25 is_stmt 1
	leaq	8(%r8), %rsi
.Ltmp2337:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r8, %rdi
	movq	672(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2338:
.LBB1_336:
	.loc	5 443 13
	leaq	.Lalloc_2db6ebe421ea47d9786daaa42d69aa61(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2339:
.LBB1_337:
	.loc	5 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp2340:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2341:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_91a4c6b9b17ebf4d863f9a70b6dc929a(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_338:
	vmovaps	%ymm13, 3104(%rsp)
	movq	88(%rsp), %r13
	jmp	.LBB1_263
.LBB1_339:
.Ltmp2342:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2343:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_754b353a9c273ca837480913d0661c80(%rip), %rcx
	movq	%r15, %rsi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_340:
.Ltmp2344:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2345:
	.loc	5 581 13
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
	movq	%r10, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2346:
.LBB1_341:
	.loc	1 1764 5
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2347:
	.loc	5 581 13
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	%r11, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2348:
.LBB1_342:
	.loc	1 1764 5
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2349:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_343:
	movq	%rbx, 320(%rsp)
	movq	%r10, 352(%rsp)
	vmovaps	%ymm3, 1664(%rsp)
	movq	80(%rsp), %r15
	movq	24(%rsp), %rbx
	movq	2944(%rsp), %rcx
	vmovaps	192(%rsp), %ymm5
	vmovaps	416(%rsp), %ymm15
	movl	$32, %r10d
	vmovaps	96(%rsp), %ymm1
	movq	640(%rsp), %r14
	movq	3040(%rsp), %r9
.LBB1_344:
.Ltmp2350:
	addq	$32, %r9
	decq	%rcx
	movq	3008(%rsp), %rsi
.Ltmp2351:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rsi
	addq	$1024, %r14
	testq	%rcx, %rcx
	movq	8(%rsp), %rax
	je	.LBB1_377
.LBB1_345:
.Ltmp2352:
	.loc	4 2584 13
	cmpq	$32, %rsi
	movl	$32, %r8d
	movq	%rsi, 3008(%rsp)
	cmovbq	%rsi, %r8
	cmpq	$1, %r8
	movq	%r8, 1760(%rsp)
	adcq	$0, %r8
.Ltmp2353:
	.loc	1 3344 55
	subq	%r9, %rax
.Ltmp2354:
	.loc	10 1078 5
	cmpq	$32, %rax
	cmovaeq	%r10, %rax
.Ltmp2355:
	.loc	1 3345 31
	leaq	(,%r9,8), %rdi
.Ltmp2356:
	.loc	1 3349 39
	leaq	(%rax,%r9), %rsi
	shlq	$3, %rsi
.Ltmp2357:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB1_393
	cmpq	%r15, %rsi
	ja	.LBB1_393
.Ltmp2358:
	.loc	11 304 12
	cmpq	%r9, 8(%rsp)
	je	.LBB1_344
.Ltmp2359:
	.loc	11 0 12 is_stmt 0
	movq	%rax, 896(%rsp)
	movq	%r9, 3040(%rsp)
	vmovaps	%ymm3, 3872(%rsp)
	vmovaps	%ymm12, 3840(%rsp)
	movq	%rcx, 2944(%rsp)
	shll	$5, %r8d
	movq	88(%rsp), %rcx
	vmovaps	(%rcx), %ymm3
	vmovaps	%ymm3, 448(%rsp)
	vmovaps	32(%rcx), %ymm3
	vmovaps	%ymm3, 384(%rsp)
	vmovaps	64(%rcx), %ymm3
	vmovaps	%ymm3, 288(%rsp)
	vmovaps	96(%rcx), %ymm3
	vmovaps	%ymm3, 256(%rsp)
	vmovaps	128(%rcx), %ymm3
	vmovaps	%ymm3, 192(%rsp)
	vmovaps	160(%rcx), %ymm3
	vmovaps	%ymm3, 96(%rsp)
	vmovaps	192(%rcx), %ymm3
	vmovaps	%ymm3, 640(%rsp)
	vmovaps	224(%rcx), %ymm3
	vmovaps	%ymm3, 800(%rsp)
	vmovaps	256(%rcx), %ymm3
	vmovaps	%ymm3, 2912(%rsp)
	vmovaps	288(%rcx), %ymm3
	vmovaps	%ymm3, 2880(%rsp)
	vmovaps	320(%rcx), %ymm3
	vmovaps	%ymm3, 2848(%rsp)
	xorl	%eax, %eax
	vmovaps	%ymm15, 416(%rsp)
	vmovaps	%ymm10, %ymm4
	vmovaps	%ymm13, 128(%rsp)
	vmovaps	%ymm2, %ymm12
	vmovaps	%ymm9, %ymm11
	vmovaps	%ymm5, %ymm3
	vmovaps	%ymm0, %ymm5
	vmovaps	%ymm14, %ymm7
	vmovaps	%ymm1, %ymm14
	vmovaps	352(%rcx), %ymm0
	vmovaps	%ymm0, 2816(%rsp)
	vmovaps	384(%rcx), %ymm0
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	416(%rcx), %ymm0
	vmovaps	%ymm0, 2752(%rsp)
	vmovaps	448(%rcx), %ymm0
	vmovaps	%ymm0, 2720(%rsp)
	vmovaps	480(%rcx), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
	vmovaps	512(%rcx), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	544(%rcx), %ymm0
	vmovaps	%ymm0, 2624(%rsp)
	vmovaps	576(%rcx), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	608(%rcx), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	640(%rcx), %ymm0
	vmovaps	%ymm0, 2528(%rsp)
	vmovaps	672(%rcx), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	704(%rcx), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	736(%rcx), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	768(%rcx), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	800(%rcx), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	832(%rcx), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	864(%rcx), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	896(%rcx), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	928(%rcx), %ymm0
	vmovaps	%ymm0, 2240(%rsp)
	vmovaps	960(%rcx), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	992(%rcx), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	1024(%rcx), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	1056(%rcx), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	1088(%rcx), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	1120(%rcx), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	1152(%rcx), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	1184(%rcx), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	1216(%rcx), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	1248(%rcx), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	1280(%rcx), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1312(%rcx), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	1344(%rcx), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	1376(%rcx), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	1408(%rcx), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	vmovaps	1440(%rcx), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	1472(%rcx), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	1504(%rcx), %ymm0
	vmovaps	%ymm0, 928(%rsp)
	vmovaps	2976(%rsp), %ymm13
	.p2align	4
.LBB1_349:
	vmovaps	416(%rsp), %ymm15
	vmovaps	%ymm13, %ymm8
.Ltmp2360:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r14,%rax), %ymm13
.Ltmp2361:
	.loc	29 283 14
	vmulps	448(%rsp), %ymm13, %ymm0
	vxorps	%xmm6, %xmm6, %xmm6
.Ltmp2362:
	.loc	29 48 14
	vaddps	%ymm6, %ymm0, %ymm0
.Ltmp2363:
	.loc	29 283 14
	vmulps	384(%rsp), %ymm13, %ymm1
.Ltmp2364:
	.loc	29 48 14
	vaddps	%ymm6, %ymm1, %ymm1
.Ltmp2365:
	.loc	29 283 14
	vmulps	288(%rsp), %ymm13, %ymm2
.Ltmp2366:
	.loc	29 48 14
	vaddps	%ymm6, %ymm2, %ymm2
.Ltmp2367:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm8, %ymm9
.Ltmp2368:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm0
.Ltmp2369:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm8, %ymm9
.Ltmp2370:
	.loc	29 48 14
	vaddps	%ymm1, %ymm9, %ymm1
.Ltmp2371:
	.loc	29 283 14
	vmulps	640(%rsp), %ymm8, %ymm9
.Ltmp2372:
	.loc	29 48 14
	vaddps	%ymm2, %ymm9, %ymm2
.Ltmp2373:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm13, %ymm9
.Ltmp2374:
	.loc	29 48 14
	vaddps	%ymm6, %ymm9, %ymm9
	vmovaps	%ymm8, 416(%rsp)
.Ltmp2375:
	.loc	29 283 14
	vmulps	800(%rsp), %ymm8, %ymm10
.Ltmp2376:
	.loc	29 48 14
	vaddps	%ymm9, %ymm10, %ymm9
.Ltmp2377:
	.loc	29 283 14
	vmulps	2912(%rsp), %ymm15, %ymm10
.Ltmp2378:
	.loc	29 48 14
	vaddps	%ymm0, %ymm10, %ymm10
.Ltmp2379:
	.loc	29 283 14
	vmulps	2880(%rsp), %ymm15, %ymm0
.Ltmp2380:
	.loc	29 48 14
	vaddps	%ymm1, %ymm0, %ymm1
.Ltmp2381:
	.loc	29 283 14
	vmulps	2848(%rsp), %ymm15, %ymm0
.Ltmp2382:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm2
.Ltmp2383:
	.loc	29 283 14
	vmulps	2816(%rsp), %ymm15, %ymm0
.Ltmp2384:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm9
	vmovaps	%ymm14, %ymm0
.Ltmp2385:
	.loc	29 283 14
	vmulps	2784(%rsp), %ymm14, %ymm14
.Ltmp2386:
	.loc	29 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp2387:
	.loc	29 283 14
	vmulps	2752(%rsp), %ymm0, %ymm14
.Ltmp2388:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp2389:
	.loc	29 283 14
	vmulps	2720(%rsp), %ymm0, %ymm14
.Ltmp2390:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp2391:
	.loc	29 283 14
	vmulps	2688(%rsp), %ymm0, %ymm14
.Ltmp2392:
	.loc	29 48 14
	vaddps	%ymm9, %ymm14, %ymm14
	vmovaps	%ymm5, %ymm9
.Ltmp2393:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm5, %ymm5
.Ltmp2394:
	.loc	29 48 14
	vaddps	%ymm5, %ymm10, %ymm5
.Ltmp2395:
	.loc	29 283 14
	vmulps	2624(%rsp), %ymm9, %ymm10
.Ltmp2396:
	.loc	29 48 14
	vaddps	%ymm1, %ymm10, %ymm1
.Ltmp2397:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm9, %ymm10
.Ltmp2398:
	.loc	29 48 14
	vaddps	%ymm2, %ymm10, %ymm10
.Ltmp2399:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm9, %ymm2
.Ltmp2400:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm14
	vmovaps	%ymm11, %ymm2
.Ltmp2401:
	.loc	29 283 14
	vmulps	2528(%rsp), %ymm11, %ymm11
.Ltmp2402:
	.loc	29 48 14
	vaddps	%ymm5, %ymm11, %ymm5
.Ltmp2403:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm2, %ymm11
.Ltmp2404:
	.loc	29 48 14
	vaddps	%ymm1, %ymm11, %ymm1
.Ltmp2405:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm2, %ymm11
.Ltmp2406:
	.loc	29 48 14
	vaddps	%ymm10, %ymm11, %ymm11
.Ltmp2407:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm2, %ymm10
.Ltmp2408:
	.loc	29 48 14
	vaddps	%ymm14, %ymm10, %ymm14
	vmovaps	%ymm12, %ymm10
.Ltmp2409:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm12, %ymm12
.Ltmp2410:
	.loc	29 48 14
	vaddps	%ymm5, %ymm12, %ymm5
.Ltmp2411:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm10, %ymm12
.Ltmp2412:
	.loc	29 48 14
	vaddps	%ymm1, %ymm12, %ymm12
.Ltmp2413:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm10, %ymm1
.Ltmp2414:
	.loc	29 48 14
	vaddps	%ymm1, %ymm11, %ymm11
.Ltmp2415:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm10, %ymm1
.Ltmp2416:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm14
	vmovaps	32(%rsp), %ymm8
	vmovaps	%ymm3, %ymm6
	vmovaps	128(%rsp), %ymm3
	vmovaps	%ymm4, %ymm1
.Ltmp2417:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm4, %ymm4
.Ltmp2418:
	.loc	29 48 14
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp2419:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm1, %ymm5
.Ltmp2420:
	.loc	29 48 14
	vaddps	%ymm5, %ymm12, %ymm5
.Ltmp2421:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm1, %ymm12
.Ltmp2422:
	.loc	29 48 14
	vaddps	%ymm11, %ymm12, %ymm11
	vmovaps	%ymm1, 128(%rsp)
.Ltmp2423:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm1, %ymm12
.Ltmp2424:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp2425:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm3, %ymm14
.Ltmp2426:
	.loc	29 48 14
	vaddps	%ymm4, %ymm14, %ymm4
.Ltmp2427:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm3, %ymm14
.Ltmp2428:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2429:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm3, %ymm14
.Ltmp2430:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2431:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm3, %ymm14
.Ltmp2432:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2433:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm6, %ymm14
.Ltmp2434:
	.loc	29 48 14
	vaddps	%ymm4, %ymm14, %ymm4
.Ltmp2435:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm6, %ymm14
.Ltmp2436:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2437:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm6, %ymm14
.Ltmp2438:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2439:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm6, %ymm14
.Ltmp2440:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2441:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm7, %ymm14
.Ltmp2442:
	.loc	29 48 14
	vaddps	%ymm4, %ymm14, %ymm4
.Ltmp2443:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm7, %ymm14
.Ltmp2444:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2445:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm7, %ymm14
.Ltmp2446:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
	vmovaps	%ymm7, 32(%rsp)
.Ltmp2447:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm7, %ymm14
	vmovaps	%ymm6, %ymm7
.Ltmp2448:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2449:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm8, %ymm14
.Ltmp2450:
	.loc	29 48 14
	vaddps	%ymm4, %ymm14, %ymm4
.Ltmp2451:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm8, %ymm14
.Ltmp2452:
	.loc	29 48 14
	vaddps	%ymm5, %ymm14, %ymm5
.Ltmp2453:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm8, %ymm14
.Ltmp2454:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm11
.Ltmp2455:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm8, %ymm14
.Ltmp2456:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp2457:
	.loc	29 82 19
	vbroadcastss	.LCPI1_0(%rip), %ymm1
.Ltmp2458:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm1, %ymm4, %ymm4
.Ltmp2459:
	.loc	29 82 19
	vandps	%ymm1, %ymm10, %ymm14
.Ltmp2460:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm4, %ymm14, %ymm4
.Ltmp2461:
	.loc	29 82 19
	vandps	%ymm1, %ymm5, %ymm5
.Ltmp2462:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm4, %ymm4
.Ltmp2463:
	.loc	29 82 19
	vandps	%ymm1, %ymm11, %ymm5
.Ltmp2464:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm4, %ymm4
.Ltmp2465:
	.loc	29 82 19
	vandps	%ymm1, %ymm12, %ymm5
.Ltmp2466:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm4, %ymm4
.Ltmp2467:
	.loc	12 551 14
	vmovups	%ymm4, 4320(%rsp,%rax)
.Ltmp2468:
	.loc	11 304 12
	addq	$32, %rax
	vmovaps	%ymm10, %ymm4
	vmovaps	%ymm2, %ymm12
	vmovaps	%ymm9, %ymm11
	vmovaps	%ymm0, %ymm5
	vmovaps	%ymm15, %ymm14
	cmpq	%rax, %r8
	jne	.LBB1_349
.Ltmp2469:
	.loc	11 0 12 is_stmt 0
	movq	%r14, 640(%rsp)
	vmovaps	%ymm15, 96(%rsp)
	vmovaps	%ymm3, 192(%rsp)
	vmovaps	%ymm13, 2976(%rsp)
	xorl	%ecx, %ecx
	movq	352(%rsp), %r10
	movq	320(%rsp), %rbx
	vmovaps	3840(%rsp), %ymm12
	vmovaps	%ymm7, %ymm14
	vmovaps	3936(%rsp), %ymm6
	vmovaps	3904(%rsp), %ymm7
	vmovaps	128(%rsp), %ymm13
	vmovaps	3872(%rsp), %ymm3
	xorl	%r9d, %r9d
	movq	896(%rsp), %r14
	jmp	.LBB1_352
.LBB1_351:
	movq	256(%rsp), %rcx
.Ltmp2470:
	addq	%r14, %rcx
	movq	320(%rsp), %rbx
.Ltmp2471:
	.loc	1 3406 39 is_stmt 1
	addq	%r14, %rbx
.Ltmp2472:
	.loc	1 1148 8
	cmpq	%r12, %rbx
	movq	%r12, %rax
	movl	$0, %r9d
	cmovbq	%r9, %rax
	subq	%rax, %rbx
	movq	352(%rsp), %r10
.Ltmp2473:
	.loc	1 3407 39
	addq	%r14, %r10
	movq	1728(%rsp), %rax
.Ltmp2474:
	.loc	1 1148 8
	cmpq	%rax, %r10
	cmovbq	%r9, %rax
	subq	%rax, %r10
	movq	896(%rsp), %r14
.Ltmp2475:
	.loc	1 3355 19
	cmpq	%r14, %rcx
	jae	.LBB1_343
.LBB1_352:
	.loc	1 0 19 is_stmt 0
	movq	%rcx, 256(%rsp)
	.loc	1 3363 21 is_stmt 1
	subq	%rcx, %r14
	movq	88(%rsp), %rax
.Ltmp2476:
	.loc	1 1575 16
	movq	1624(%rax), %r8
.Ltmp2477:
	.loc	1 1576 16
	movq	1632(%rax), %rcx
.Ltmp2478:
	.loc	1 1577 25
	leaq	1(%rbx), %rax
.Ltmp2479:
	.loc	1 1148 8
	cmpq	%r8, %rax
	movq	%r8, %rsi
	cmovbq	%r9, %rsi
	negq	%rsi
	movq	3072(%rsp), %rax
.Ltmp2480:
	.loc	1 1578 28
	leaq	(%rbx,%rax), %r15
.Ltmp2481:
	.loc	1 1148 8
	cmpq	%r8, %r15
	movq	%r8, %rax
	cmovbq	%r9, %rax
	subq	%rax, %r15
	movq	72(%rsp), %rax
.Ltmp2482:
	.loc	1 1580 33
	addq	%rbx, %rax
.Ltmp2483:
	.loc	1 1148 8
	cmpq	%r8, %rax
	movq	%r8, %rdi
	cmovbq	%r9, %rdi
	subq	%rdi, %rax
.Ltmp2484:
	.loc	1 1583 14
	movq	%r8, %r9
	subq	%rbx, %r9
.Ltmp2485:
	.loc	10 1078 5
	cmpq	%r14, %r9
	cmovbq	%r9, %r14
	movq	%r10, 352(%rsp)
.Ltmp2486:
	.loc	1 1584 14
	subq	%r10, %rcx
.Ltmp2487:
	.loc	10 1078 5
	cmpq	%r14, %rcx
	cmovbq	%rcx, %r14
	movq	%rbx, 320(%rsp)
.Ltmp2488:
	.loc	1 1148 8
	leaq	(%rbx,%rsi), %rdi
	incq	%rdi
.Ltmp2489:
	.loc	1 1585 14
	movq	%r8, %r10
	movq	%rdi, 288(%rsp)
	subq	%rdi, %r10
.Ltmp2490:
	.loc	10 1078 5
	cmpq	%r14, %r10
	cmovbq	%r10, %r14
.Ltmp2491:
	.loc	1 1586 14
	movq	%r8, %rbx
	subq	%r15, %rbx
.Ltmp2492:
	.loc	10 1078 5
	cmpq	%r14, %rbx
	cmovbq	%rbx, %r14
.Ltmp2493:
	.loc	1 1588 14
	subq	%rax, %r8
.Ltmp2494:
	.loc	10 1078 5
	cmpq	%r14, %r8
	cmovbq	%r8, %r14
	movq	3040(%rsp), %rsi
	movq	256(%rsp), %rdi
.Ltmp2495:
	.loc	1 3369 28
	addq	%rsi, %rdi
.Ltmp2496:
	.loc	1 3371 55
	leaq	(%r14,%rdi), %rsi
.Ltmp2497:
	.loc	1 3369 28
	shlq	$3, %rdi
.Ltmp2498:
	.loc	1 3371 55
	shlq	$3, %rsi
.Ltmp2499:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB1_386
	cmpq	80(%rsp), %rsi
	ja	.LBB1_386
.Ltmp2500:
	.loc	11 304 12
	testq	%r14, %r14
	je	.LBB1_351
.Ltmp2501:
	.loc	11 0 12 is_stmt 0
	movq	256(%rsp), %rsi
	shlq	$5, %rsi
	cmpq	%r8, %rbx
	cmovbq	%rbx, %r8
	cmpq	%r10, %r8
	cmovaeq	%r10, %r8
	movq	24(%rsp), %r10
	leaq	(%r10,%rdi,4), %rdi
	movq	%rdi, 384(%rsp)
	cmpq	%r9, %r8
	cmovaeq	%r9, %r8
	cmpq	%rcx, %r8
	cmovaeq	%rcx, %r8
	movq	1760(%rsp), %rcx
	subq	256(%rsp), %rcx
	cmpq	%rcx, %r8
	cmovbq	%r8, %rcx
	addq	%rsp, %rsi
	addq	$4320, %rsi
	movq	%rsi, 448(%rsp)
	movabsq	$2305843009213693951, %rsi
	andq	%rsi, %rcx
	xorl	%ebx, %ebx
	vmovaps	%ymm12, %ymm4
	.p2align	4
.LBB1_356:
	movq	320(%rsp), %rsi
.Ltmp2502:
	.loc	1 1502 26 is_stmt 1
	leaq	(%rbx,%rsi), %r8
.Ltmp2503:
	.loc	1 1137 16
	leaq	(,%r8,8), %rdi
.Ltmp2504:
	.loc	1 1138 33
	leaq	8(,%r8,8), %rsi
.Ltmp2505:
	.loc	4 1050 16
	leaq	7(,%r8,8), %r8
	cmpq	%rdx, %r8
	jae	.LBB1_372
.Ltmp2506:
	.loc	1 0 0 is_stmt 0
	leaq	(,%rbx,8), %r8
	movq	448(%rsp), %r9
.Ltmp2507:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r9,%r8,4), %ymm5
.Ltmp2508:
	.loc	29 233 14
	vmaxps	%ymm5, %ymm5, %ymm11
.Ltmp2509:
	.loc	29 585 19
	vblendvps	%ymm7, %ymm11, %ymm5, %ymm5
	vmovaps	224(%rsp), %ymm12
.Ltmp2510:
	.loc	29 360 14
	vdivps	%ymm5, %ymm12, %ymm11
.Ltmp2511:
	.loc	1 0 0 is_stmt 0
	leaq	(%rbx,%r15), %r9
.Ltmp2512:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm12, %ymm5, %ymm5
	vbroadcastss	.LCPI1_2(%rip), %ymm15
.Ltmp2513:
	.loc	29 585 19
	vblendvps	%ymm5, %ymm11, %ymm15, %ymm5
.Ltmp2514:
	.loc	12 551 14
	vmovups	%ymm5, (%r13,%rdi,4)
.Ltmp2515:
	.loc	1 1130 16
	leaq	(,%r9,8), %r8
.Ltmp2516:
	.loc	4 1050 16
	leaq	7(,%r9,8), %r10
	cmpq	%rdx, %r10
	jae	.LBB1_370
.Ltmp2517:
	.loc	12 551 14
	vmovups	(%r13,%r8,4), %ymm5
	vmovaps	%ymm5, %ymm12
.Ltmp2518:
	.loc	1 1206 22
	testl	%r11d, %r11d
	je	.LBB1_360
.Ltmp2519:
	.loc	29 257 14
	vminps	%ymm5, %ymm4, %ymm12
.Ltmp2520:
.LBB1_360:
	.loc	1 0 0 is_stmt 0
	movl	%r11d, %r11d
.Ltmp2521:
	.loc	1 1212 20 is_stmt 1
	incq	%r11
	movq	736(%rsp), %r8
	movq	%r8, %r10
	cmpq	%r8, %r11
.Ltmp2522:
	.loc	1 1213 22
	jne	.LBB1_364
	.loc	1 0 22 is_stmt 0
.Ltmp2523:
	.p2align	4
.LBB1_361:
.Ltmp2524:
	.loc	1 1130 16 is_stmt 1
	leaq	(,%r9,8), %r8
.Ltmp2525:
	.loc	4 1050 16
	leaq	7(,%r9,8), %r11
	cmpq	%rdx, %r11
	jae	.LBB1_370
.Ltmp2526:
	.loc	29 257 14
	vminps	(%r13,%r8,4), %ymm5, %ymm5
.Ltmp2527:
	.loc	12 551 14
	vmovups	%ymm5, (%r13,%r8,4)
.Ltmp2528:
	.loc	1 1224 16
	testq	%r9, %r9
	cmoveq	%r12, %r9
	.loc	1 1227 13
	decq	%r9
.Ltmp2529:
	.loc	10 1916 50
	decq	%r10
.Ltmp2530:
	.loc	3 900 12
	jne	.LBB1_361
.Ltmp2531:
	.loc	3 0 12 is_stmt 0
	xorl	%r11d, %r11d
	vmovaps	%ymm12, %ymm4
	jmp	.LBB1_366
	.p2align	4
.LBB1_364:
	movq	288(%rsp), %r8
	addq	%rbx, %r8
.Ltmp2532:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r8,8), %r9
.Ltmp2533:
	.loc	1 1130 16
	shlq	$3, %r8
.Ltmp2534:
	.loc	4 1050 16
	cmpq	%rdx, %r9
	jae	.LBB1_370
.Ltmp2535:
	.loc	12 551 14
	vmovups	(%r13,%r8,4), %ymm4
.Ltmp2536:
	.loc	29 257 14
	vminps	%ymm12, %ymm4, %ymm4
.Ltmp2537:
.LBB1_366:
	.loc	1 0 0 is_stmt 0
	leaq	(%rbx,%rax), %r8
.Ltmp2538:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r8,8), %r9
.Ltmp2539:
	.loc	1 1130 16
	shlq	$3, %r8
	movq	704(%rsp), %r10
.Ltmp2540:
	.loc	4 1050 16
	cmpq	%r10, %r9
	jae	.LBB1_373
.Ltmp2541:
	.loc	4 0 16 is_stmt 0
	cmpq	%r10, %rsi
.Ltmp2542:
	.loc	4 1050 16
	ja	.LBB1_374
.Ltmp2543:
	.loc	4 0 16
	movq	352(%rsp), %rsi
	leaq	(%rbx,%rsi), %r9
.Ltmp2544:
	vbroadcastss	.LCPI1_3(%rip), %ymm5
	vmulps	%ymm5, %ymm4, %ymm4
	vroundps	$9, %ymm4, %ymm4
	vbroadcastss	.LCPI1_4(%rip), %ymm5
	vmulps	%ymm5, %ymm4, %ymm4
.Ltmp2545:
	vaddps	%ymm4, %ymm3, %ymm5
	movq	832(%rsp), %rsi
	vsubps	(%rsi,%r8,4), %ymm5, %ymm3
.Ltmp2546:
	.loc	12 551 14 is_stmt 1
	vmovups	%ymm4, (%rsi,%rdi,4)
.Ltmp2547:
	.loc	1 1659 43
	vmovaps	1632(%rsp), %ymm4
.Ltmp2548:
	.loc	29 360 14
	vdivps	608(%rsp), %ymm3, %ymm5
.Ltmp2549:
	.loc	29 347 14
	vsubps	%ymm5, %ymm15, %ymm5
.Ltmp2550:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm4, %ymm5, %ymm11
.Ltmp2551:
	.loc	29 283 14 is_stmt 1
	vmulps	864(%rsp), %ymm11, %ymm11
.Ltmp2552:
	.loc	29 48 14
	vaddps	%ymm4, %ymm11, %ymm4
.Ltmp2553:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm5, %ymm4
.Ltmp2554:
	.loc	29 82 19
	vandps	%ymm1, %ymm4, %ymm5
.Ltmp2555:
	.loc	29 871 14
	vbroadcastss	.LCPI1_5(%rip), %ymm11
	vcmplt_oqps	%ymm11, %ymm5, %ymm5
.Ltmp2556:
	.loc	29 82 19
	vandnps	%ymm4, %ymm5, %ymm4
.Ltmp2557:
	.loc	1 1660 5
	vmovaps	%ymm4, 1632(%rsp)
.Ltmp2558:
	.loc	4 1050 16
	leaq	7(,%r9,8), %rsi
.Ltmp2559:
	.loc	1 1130 16
	shlq	$3, %r9
.Ltmp2560:
	.loc	4 1050 16
	cmpq	672(%rsp), %rsi
	jae	.LBB1_375
.Ltmp2561:
	.loc	1 0 0 is_stmt 0
	movq	%rbx, %rsi
	shlq	$5, %rsi
	addq	384(%rsp), %rsi
	incq	%rbx
.Ltmp2562:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm4, %ymm15, %ymm4
	movq	480(%rsp), %rdi
.Ltmp2563:
	.loc	12 551 14
	vmovups	(%rdi,%r9,4), %ymm5
.Ltmp2564:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rsi), %ymm11
.Ltmp2565:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm5, %ymm4, %ymm4
.Ltmp2566:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm5, %ymm4, %ymm4
.Ltmp2567:
	.loc	12 551 14
	vmovups	%ymm11, (%rdi,%r9,4)
.Ltmp2568:
	.loc	12 551 14 is_stmt 0
	vmovups	%ymm4, (%rsi)
	vmovaps	%ymm12, %ymm4
.Ltmp2569:
	.loc	11 304 12 is_stmt 1
	cmpq	%rcx, %rbx
	jne	.LBB1_356
	jmp	.LBB1_351
.Ltmp2570:
.LBB1_370:
	.loc	1 1764 5
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	%ymm14, 1280(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	%ymm8, 1344(%rsp)
.Ltmp2571:
	.loc	1 0 0 is_stmt 0
	leaq	8(%r8), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_371:
.Ltmp2572:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm3, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2573:
	.loc	5 443 13
	leaq	.Lalloc_1c9f1dd5a676136ffd1db4032fbd14da(%rip), %rcx
	movq	%rax, %rdi
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2574:
.LBB1_372:
	.loc	1 1764 5
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	%ymm14, 1280(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	%ymm8, 1344(%rsp)
.Ltmp2575:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2576:
.LBB1_373:
	.loc	1 1764 5
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	%ymm14, 1280(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	%ymm8, 1344(%rsp)
.Ltmp2577:
	.loc	1 1131 25
	leaq	8(%r8), %rsi
.Ltmp2578:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r8, %rdi
	movq	%r10, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2579:
.LBB1_374:
	.loc	1 1764 5
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	%ymm14, 1280(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	%ymm8, 1344(%rsp)
.Ltmp2580:
	.loc	5 456 13
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r10, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2581:
.LBB1_375:
	.loc	1 1764 5
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	%ymm14, 1280(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	%ymm8, 1344(%rsp)
.Ltmp2582:
	.loc	1 1131 25
	leaq	8(%r9), %rsi
.Ltmp2583:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r9, %rdi
	movq	672(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2584:
.LBB1_376:
	.loc	1 1764 5
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2585:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_91a4c6b9b17ebf4d863f9a70b6dc929a(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_377:
.Ltmp2586:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	%ymm14, 1280(%rsp)
	vmovaps	32(%rsp), %ymm3
	vmovaps	%ymm3, 1312(%rsp)
	vmovaps	%ymm8, 1344(%rsp)
	vmovaps	%ymm1, 1056(%rsp)
	vmovaps	%ymm0, 1088(%rsp)
	vmovaps	%ymm9, 1120(%rsp)
	vmovaps	%ymm2, 1152(%rsp)
	vmovaps	%ymm10, 1184(%rsp)
.Ltmp2587:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm15, 1024(%rsp)
	movq	88(%rsp), %r13
	vmovaps	2976(%rsp), %ymm13
	jmp	.LBB1_293
.Ltmp2588:
.LBB1_378:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2589:
	.loc	5 443 13
	leaq	.Lalloc_1c9f1dd5a676136ffd1db4032fbd14da(%rip), %rcx
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2590:
.LBB1_379:
	.loc	5 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp2591:
	.loc	1 1654 5 is_stmt 1
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp2592:
	.loc	1 851 0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp2593:
	.loc	1 853 0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	288(%rsp), %ymm0
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm0, 1568(%rsp)
.Ltmp2594:
	leaq	.Lalloc_e76d4292d58481de1418049881299651(%rip), %rcx
	movq	80(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_380:
	vmovaps	96(%rsp), %ymm0
.Ltmp2595:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2596:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_4a8785a681d008a9bfd0cd82628ea9cb(%rip), %rdx
	movq	416(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_381:
	vmovaps	96(%rsp), %ymm0
.Ltmp2597:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2598:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_2b690e2c7763f11809942906fc2ca813(%rip), %rdx
	movq	128(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_382:
	vmovaps	96(%rsp), %ymm0
.Ltmp2599:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2600:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cbce7773ac40979e4ba2385da3aec116(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_383:
	vmovaps	96(%rsp), %ymm0
.Ltmp2601:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2602:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_ec0d48f73ebfc2755df5cedaa60b5c0a(%rip), %rdx
	movq	320(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_384:
	vmovaps	96(%rsp), %ymm0
.Ltmp2603:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2604:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_2c461872bb652d4796cdcf89c28c82c8(%rip), %rdx
	movq	%r11, %rdi
	movq	128(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_385:
	vmovaps	96(%rsp), %ymm0
.Ltmp2605:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2606:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_9fa421ae81817f58fcfc4a3243223891(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_386:
.Ltmp2607:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	%ymm14, 1280(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	%ymm8, 1344(%rsp)
.Ltmp2608:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_e76d4292d58481de1418049881299651(%rip), %rcx
	movq	80(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2609:
.LBB1_387:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_618bfb3c8de0a6cf4e1facf86d98b625(%rip), %rcx
.Ltmp2610:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2611:
.LBB1_388:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_54e75a1a7c7cc9b6c609a5a2db89b4ce(%rip), %rcx
.Ltmp2612:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2613:
.LBB1_389:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2614:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_4a8785a681d008a9bfd0cd82628ea9cb(%rip), %rdx
	movq	416(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_390:
.Ltmp2615:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2616:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_2b690e2c7763f11809942906fc2ca813(%rip), %rdx
	movq	128(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_391:
.Ltmp2617:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2618:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cbce7773ac40979e4ba2385da3aec116(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_392:
.Ltmp2619:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2620:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_ec0d48f73ebfc2755df5cedaa60b5c0a(%rip), %rdx
	movq	320(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_393:
.Ltmp2621:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm13, 1216(%rsp)
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	%ymm14, 1280(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	%ymm8, 1344(%rsp)
.Ltmp2622:
.LBB1_394:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cf577da48352b38fc6c5372e02febe14(%rip), %rcx
	movq	%r15, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB1_395:
.Ltmp2623:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2624:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_2c461872bb652d4796cdcf89c28c82c8(%rip), %rdx
	movq	%r11, %rdi
	movq	128(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_396:
	vmovaps	96(%rsp), %ymm0
.Ltmp2625:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	224(%rsp), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp2626:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_b305c1483509cfb31fdec21ff8752674(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_397:
.Ltmp2627:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2628:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_9fa421ae81817f58fcfc4a3243223891(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_398:
	movl	$1, %eax
	jmp	.LBB1_420
.LBB1_399:
.Ltmp2629:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2630:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_b305c1483509cfb31fdec21ff8752674(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2631:
.LBB1_400:
	movl	$2, %eax
	jmp	.LBB1_420
.LBB1_401:
.Ltmp2632:
	.loc	1 3131 25 is_stmt 1
	leaq	.Lalloc_d412f54e6343f84b390d7eb959721e64(%rip), %rdx
	movq	%rbx, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_402:
	.loc	1 0 25 is_stmt 0
	movl	$3, %eax
	jmp	.LBB1_420
.LBB1_403:
	movl	$1, %eax
	jmp	.LBB1_425
.LBB1_404:
	movl	$4, %eax
	jmp	.LBB1_420
.LBB1_405:
	movl	$2, %eax
	jmp	.LBB1_425
.LBB1_406:
	movl	$1, %edi
.Ltmp2633:
	.loc	1 3132 23 is_stmt 1
	leaq	.Lalloc_bc6430efe4e05da3532b83a8ba1c0c16(%rip), %rdx
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_407:
	.loc	1 0 23 is_stmt 0
	movl	$2, %edi
	.loc	1 3132 23
	leaq	.Lalloc_bc6430efe4e05da3532b83a8ba1c0c16(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_408:
	.loc	1 0 23
	movl	$3, %edi
	.loc	1 3132 23
	leaq	.Lalloc_bc6430efe4e05da3532b83a8ba1c0c16(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_409:
	.loc	1 0 23
	movl	$4, %edi
	.loc	1 3132 23
	leaq	.Lalloc_bc6430efe4e05da3532b83a8ba1c0c16(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_410:
	.loc	1 0 23
	movl	$5, %edi
	.loc	1 3132 23
	leaq	.Lalloc_bc6430efe4e05da3532b83a8ba1c0c16(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_411:
	.loc	1 0 23
	movl	$6, %edi
	.loc	1 3132 23
	leaq	.Lalloc_bc6430efe4e05da3532b83a8ba1c0c16(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_412:
	.loc	1 0 23
	movl	$7, %edi
	.loc	1 3132 23
	leaq	.Lalloc_bc6430efe4e05da3532b83a8ba1c0c16(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB1_413:
	.loc	1 0 23
	movl	$8, %edi
	.loc	1 3132 23
	leaq	.Lalloc_bc6430efe4e05da3532b83a8ba1c0c16(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2634:
.LBB1_414:
	.loc	1 0 23
	movl	$5, %eax
	jmp	.LBB1_420
.LBB1_415:
	movl	$3, %eax
	jmp	.LBB1_425
.LBB1_416:
.Ltmp2635:
	.loc	1 665 42 is_stmt 1
	leaq	.Lalloc_bac57976a2bdbfad4a3a85d5d1c7648c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp2636:
.LBB1_417:
	.loc	1 0 42 is_stmt 0
	movl	$6, %eax
	jmp	.LBB1_420
.LBB1_418:
	movl	$4, %eax
	jmp	.LBB1_425
.LBB1_419:
	movl	$7, %eax
.LBB1_420:
	movq	%rax, 72(%rsp)
.LBB1_421:
	vmovaps	96(%rsp), %ymm0
.Ltmp2637:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	%ymm11, 3168(%rsp)
.Ltmp2638:
	.loc	1 1403 42
	leaq	.Lalloc_a6feb40b34112df5f214f84424dd2c7c(%rip), %rdx
	movq	72(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2639:
.LBB1_422:
	.loc	1 0 42 is_stmt 0
	movl	$5, %eax
	jmp	.LBB1_425
.LBB1_423:
	movl	$6, %eax
	jmp	.LBB1_425
.LBB1_424:
	movl	$7, %eax
.LBB1_425:
	movq	%rax, 544(%rsp)
.LBB1_426:
.Ltmp2640:
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm6, 1120(%rsp)
	vmovaps	%ymm10, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
.Ltmp2641:
	.loc	1 1403 42
	leaq	.Lalloc_a6feb40b34112df5f214f84424dd2c7c(%rip), %rdx
	movq	544(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2642:
.LBB1_427:
	.loc	1 455 44
	leaq	.Lalloc_f0ee36f67d9a332211aa5518dd2ebfd5(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB1_428:
	.loc	1 456 44
	leaq	.Lalloc_33d4d33e0a850133578789055882dcf9(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp2643:
.Lfunc_end1:
	.size	_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_, .Lfunc_end1-_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_
