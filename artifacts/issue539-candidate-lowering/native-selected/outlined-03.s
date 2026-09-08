_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_:
.Lfunc_begin28:
	.loc	1 1338 0 is_stmt 1
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
.Ltmp4238:
	.loc	1 970 23 prologue_end
	movq	8(%rsi), %rdx
.Ltmp4239:
	.loc	52 56 9
	testq	%rdx, %rdx
	je	.LBB28_44
.Ltmp4240:
	.loc	1 0 0 is_stmt 0
	movq	192(%rsi), %rax
.Ltmp4241:
	movq	(%rsi), %rcx
.Ltmp4242:
	.loc	1 971 0 is_stmt 1
	vmovss	(%rdi), %xmm0
.Ltmp4243:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx)
.Ltmp4244:
	.loc	5 580 12
	cmpq	%rdx, %rax
	ja	.LBB28_49
.Ltmp4245:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4246:
	.loc	1 972 0
	vmovss	4(%rdi), %xmm0
.Ltmp4247:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%rax,4)
.Ltmp4248:
	.loc	1 970 77
	leaq	(%rax,%rax), %r11
.Ltmp4249:
	.loc	5 580 12
	cmpq	%rdx, %r11
	ja	.LBB28_45
.Ltmp4250:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4251:
	.loc	1 973 0
	vmovss	8(%rdi), %xmm0
.Ltmp4252:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r11,4)
.Ltmp4253:
	.loc	1 970 77
	leaq	(%rax,%rax,2), %r8
.Ltmp4254:
	.loc	5 580 12
	cmpq	%rdx, %r8
	ja	.LBB28_46
.Ltmp4255:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4256:
	.loc	1 974 0
	vmovss	12(%rdi), %xmm0
.Ltmp4257:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r8,4)
.Ltmp4258:
	.loc	1 970 77
	leaq	(,%rax,4), %r8
.Ltmp4259:
	.loc	5 580 12
	cmpq	%rdx, %r8
	ja	.LBB28_46
.Ltmp4260:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4261:
	.loc	1 975 0
	vmovss	16(%rdi), %xmm0
.Ltmp4262:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r8,4)
.Ltmp4263:
	.loc	1 970 77
	leaq	(%rax,%rax,4), %r8
.Ltmp4264:
	.loc	5 580 12
	cmpq	%rdx, %r8
	ja	.LBB28_46
.Ltmp4265:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4266:
	.loc	1 976 0
	vmovss	20(%rdi), %xmm0
.Ltmp4267:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r8,4)
.Ltmp4268:
	.loc	1 970 77
	leaq	(%r11,%r11,2), %r9
.Ltmp4269:
	.loc	5 580 12
	cmpq	%rdx, %r9
	ja	.LBB28_48
.Ltmp4270:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4271:
	.loc	1 977 0
	vmovss	24(%rdi), %xmm0
.Ltmp4272:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r9,4)
.Ltmp4273:
	.loc	1 970 77
	leaq	(,%rax,8), %r9
	movq	%r9, %r10
	subq	%rax, %r10
.Ltmp4274:
	.loc	5 580 12
	cmpq	%rdx, %r10
	ja	.LBB28_47
.Ltmp4275:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4276:
	.loc	1 978 0
	vmovss	28(%rdi), %xmm0
.Ltmp4277:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r10,4)
.Ltmp4278:
	.loc	5 580 12
	cmpq	%rdx, %r9
	ja	.LBB28_48
.Ltmp4279:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4280:
	.loc	1 979 0
	vmovss	32(%rdi), %xmm0
.Ltmp4281:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r9,4)
.Ltmp4282:
	.loc	1 970 77
	leaq	(%rax,%rax,8), %r9
.Ltmp4283:
	.loc	5 580 12
	cmpq	%rdx, %r9
	ja	.LBB28_48
.Ltmp4284:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4285:
	.loc	1 980 0
	vmovss	36(%rdi), %xmm0
.Ltmp4286:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r9,4)
.Ltmp4287:
	.loc	1 970 77
	leaq	(%r11,%r11,4), %r9
.Ltmp4288:
	.loc	5 580 12
	cmpq	%rdx, %r9
	ja	.LBB28_48
.Ltmp4289:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4290:
	.loc	1 981 0
	vmovss	40(%rdi), %xmm0
.Ltmp4291:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%r9,4)
.Ltmp4292:
	.loc	1 970 77
	leaq	(%rax,%r8,2), %rax
.Ltmp4293:
	.loc	5 580 12
	cmpq	%rdx, %rax
	ja	.LBB28_49
.Ltmp4294:
	.loc	52 56 9
	je	.LBB28_44
.Ltmp4295:
	.loc	1 982 0
	vmovss	44(%rdi), %xmm0
.Ltmp4296:
	.loc	52 56 9
	vmovss	%xmm0, (%rcx,%rax,4)
.Ltmp4297:
	.loc	52 56 9 is_stmt 0
	cmpq	$0, 72(%rsi)
	je	.LBB28_44
.Ltmp4298:
	.loc	1 1340 0 is_stmt 1
	vmovss	80(%rdi), %xmm0
	movq	64(%rsi), %rax
.Ltmp4299:
	.loc	52 56 9
	vmovss	%xmm0, (%rax)
.Ltmp4300:
	.loc	52 56 9 is_stmt 0
	cmpq	$0, 104(%rsi)
	je	.LBB28_44
.Ltmp4301:
	.loc	1 1341 0 is_stmt 1
	vmovss	84(%rdi), %xmm0
	movq	96(%rsi), %rax
.Ltmp4302:
	.loc	52 56 9
	vmovss	%xmm0, (%rax)
.Ltmp4303:
	.loc	1 1342 28
	movq	136(%rsi), %rax
.Ltmp4304:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp4305:
	.loc	6 180 28
	je	.LBB28_34
.Ltmp4306:
	.loc	1 1342 0
	vmovss	48(%rdi), %xmm0
	vmovss	60(%rdi), %xmm1
	xorl	%edx, %edx
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp4307:
	.loc	1 840 13
	vucomiss	%xmm2, %xmm1
.Ltmp4308:
	.loc	1 1342 0
	vmovss	56(%rdi), %xmm2
.Ltmp4309:
	.loc	1 840 13
	vcvttss2si	%xmm1, %rcx
	cmovael	%ecx, %edx
.Ltmp4310:
	.loc	1 1342 0
	movq	128(%rsi), %rcx
.Ltmp4311:
	.loc	1 840 13
	vucomiss	.LCPI28_0(%rip), %xmm1
	movl	$-1, %r8d
	cmovbel	%edx, %r8d
	.loc	1 838 13
	vmovss	%xmm0, (%rcx)
	.loc	1 839 13
	vmovss	%xmm2, 8(%rcx)
	.loc	1 840 13
	movl	%r8d, 12(%rcx)
.Ltmp4312:
	.loc	7 1714 9
	cmpq	$1, %rax
.Ltmp4313:
	.loc	6 180 28
	je	.LBB28_34
.Ltmp4314:
	.loc	1 838 13
	movl	$0, 16(%rcx)
	.loc	1 839 13
	movq	$0, 24(%rcx)
.Ltmp4315:
	.loc	7 1714 9
	cmpq	$2, %rax
.Ltmp4316:
	.loc	6 180 28
	je	.LBB28_34
.Ltmp4317:
	.loc	1 838 13
	movl	$0, 32(%rcx)
	.loc	1 839 13
	movq	$0, 40(%rcx)
.Ltmp4318:
	.loc	7 1714 9
	cmpq	$3, %rax
.Ltmp4319:
	.loc	6 180 28
	je	.LBB28_34
.Ltmp4320:
	.loc	1 838 13
	movl	$0, 48(%rcx)
	.loc	1 839 13
	movq	$0, 56(%rcx)
.Ltmp4321:
	.loc	7 1714 9
	cmpq	$4, %rax
.Ltmp4322:
	.loc	6 180 28
	je	.LBB28_34
.Ltmp4323:
	.loc	1 838 13
	movl	$0, 64(%rcx)
	.loc	1 839 13
	movq	$0, 72(%rcx)
.Ltmp4324:
	.loc	7 1714 9
	cmpq	$5, %rax
.Ltmp4325:
	.loc	6 180 28
	je	.LBB28_34
.Ltmp4326:
	.loc	1 838 13
	movl	$0, 80(%rcx)
	.loc	1 839 13
	movq	$0, 88(%rcx)
.Ltmp4327:
	.loc	7 1714 9
	cmpq	$6, %rax
.Ltmp4328:
	.loc	6 180 28
	je	.LBB28_34
.Ltmp4329:
	.loc	1 838 13
	movl	$0, 96(%rcx)
	.loc	1 839 13
	movq	$0, 104(%rcx)
.Ltmp4330:
	.loc	7 1714 9
	cmpq	$7, %rax
.Ltmp4331:
	.loc	6 180 28
	je	.LBB28_34
.Ltmp4332:
	.loc	1 838 13
	movl	$0, 112(%rcx)
	.loc	1 839 13
	movq	$0, 120(%rcx)
.Ltmp4333:
	.loc	7 1714 9
	cmpq	$8, %rax
.Ltmp4334:
	.loc	6 180 28
	jne	.LBB28_50
.Ltmp4335:
.LBB28_34:
	.loc	1 1343 30
	movq	152(%rsi), %rax
.Ltmp4336:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp4337:
	.loc	6 180 28
	je	.LBB28_43
.Ltmp4338:
	.loc	1 1343 0
	vmovss	64(%rdi), %xmm0
	vmovss	76(%rdi), %xmm1
	xorl	%edx, %edx
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp4339:
	.loc	1 840 13
	vucomiss	%xmm2, %xmm1
.Ltmp4340:
	.loc	1 1343 0
	vmovss	72(%rdi), %xmm2
.Ltmp4341:
	.loc	1 840 13
	vcvttss2si	%xmm1, %rcx
	cmovael	%ecx, %edx
.Ltmp4342:
	.loc	1 1343 0
	movq	144(%rsi), %rcx
.Ltmp4343:
	.loc	1 840 13
	vucomiss	.LCPI28_0(%rip), %xmm1
	movl	$-1, %esi
	cmovbel	%edx, %esi
	.loc	1 838 13
	vmovss	%xmm0, (%rcx)
	.loc	1 839 13
	vmovss	%xmm2, 8(%rcx)
	.loc	1 840 13
	movl	%esi, 12(%rcx)
.Ltmp4344:
	.loc	7 1714 9
	cmpq	$1, %rax
.Ltmp4345:
	.loc	6 180 28
	je	.LBB28_43
.Ltmp4346:
	.loc	1 838 13
	movl	$0, 16(%rcx)
	.loc	1 839 13
	movq	$0, 24(%rcx)
.Ltmp4347:
	.loc	7 1714 9
	cmpq	$2, %rax
.Ltmp4348:
	.loc	6 180 28
	je	.LBB28_43
.Ltmp4349:
	.loc	1 838 13
	movl	$0, 32(%rcx)
	.loc	1 839 13
	movq	$0, 40(%rcx)
.Ltmp4350:
	.loc	7 1714 9
	cmpq	$3, %rax
.Ltmp4351:
	.loc	6 180 28
	je	.LBB28_43
.Ltmp4352:
	.loc	1 838 13
	movl	$0, 48(%rcx)
	.loc	1 839 13
	movq	$0, 56(%rcx)
.Ltmp4353:
	.loc	7 1714 9
	cmpq	$4, %rax
.Ltmp4354:
	.loc	6 180 28
	je	.LBB28_43
.Ltmp4355:
	.loc	1 838 13
	movl	$0, 64(%rcx)
	.loc	1 839 13
	movq	$0, 72(%rcx)
.Ltmp4356:
	.loc	7 1714 9
	cmpq	$5, %rax
.Ltmp4357:
	.loc	6 180 28
	je	.LBB28_43
.Ltmp4358:
	.loc	1 838 13
	movl	$0, 80(%rcx)
	.loc	1 839 13
	movq	$0, 88(%rcx)
.Ltmp4359:
	.loc	7 1714 9
	cmpq	$6, %rax
.Ltmp4360:
	.loc	6 180 28
	je	.LBB28_43
.Ltmp4361:
	.loc	1 838 13
	movl	$0, 96(%rcx)
	.loc	1 839 13
	movq	$0, 104(%rcx)
.Ltmp4362:
	.loc	7 1714 9
	cmpq	$7, %rax
.Ltmp4363:
	.loc	6 180 28
	je	.LBB28_43
.Ltmp4364:
	.loc	1 838 13
	movl	$0, 112(%rcx)
	.loc	1 839 13
	movq	$0, 120(%rcx)
.Ltmp4365:
	.loc	7 1714 9
	cmpq	$8, %rax
.Ltmp4366:
	.loc	6 180 28
	jne	.LBB28_50
.Ltmp4367:
.LBB28_43:
	.loc	1 1344 6 epilogue_begin
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.LBB28_48:
	.cfi_def_cfa_offset 16
.Ltmp4368:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%r9, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB28_46:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%r8, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB28_49:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4369:
.LBB28_45:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%r11, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4370:
.LBB28_47:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%r10, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4371:
.LBB28_44:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_b60532d7446335d93b6150c47af294d0(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB28_50:
.Ltmp4372:
	.loc	1 838 28 is_stmt 1
	leaq	.Lalloc_4e7cf9526cbf9c5bb2e965885a8a18fa(%rip), %rdx
	movl	$8, %edi
	movl	$8, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4373:
.Lfunc_end28:
	.size	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_, .Lfunc_end28-_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_
