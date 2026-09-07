_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_:
.Lfunc_begin30:
	.loc	1 1462 0 is_stmt 1
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	.loc	1 1465 20 prologue_end
	cmpq	$0, 184(%rsi)
	je	.LBB30_6
.Ltmp4398:
	.loc	1 1467 29
	movq	88(%rsi), %rax
.Ltmp4399:
	.loc	5 438 16
	cmpq	$7, %rax
	jbe	.LBB30_13
.Ltmp4400:
	.loc	1 1468 20
	cmpq	$0, 120(%rsi)
	je	.LBB30_8
.Ltmp4401:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rdx
.Ltmp4402:
	.loc	1 1470 33 is_stmt 1
	movq	40(%rsi), %rax
	cmpq	%rax, %rdx
.Ltmp4403:
	.loc	4 1050 16
	ja	.LBB30_4
.Ltmp4404:
	.loc	1 1471 28
	movq	56(%rsi), %rax
.Ltmp4405:
	.loc	5 451 16
	cmpq	%rax, %rdx
	ja	.LBB30_10
.Ltmp4406:
	.loc	1 0 0 is_stmt 0
	shlq	$3, %rcx
.Ltmp4407:
	.loc	1 1472 29 is_stmt 1
	movq	24(%rsi), %rax
	cmpq	%rax, %rcx
.Ltmp4408:
	.loc	4 1050 16
	ja	.LBB30_12
.Ltmp4409:
	.loc	1 0 0 is_stmt 0
	movq	176(%rsi), %rax
	movl	(%rax), %r8d
	movl	4(%rax), %r9d
	movl	8(%rax), %eax
.Ltmp4410:
	movq	80(%rsi), %r10
	movq	112(%rsi), %r11
	movl	(%r11), %r11d
	movq	32(%rsi), %rbx
	movq	48(%rsi), %r14
	.loc	1 1472 29 is_stmt 1
	movq	16(%rsi), %rsi
	.loc	1 1466 9
	vmovups	(%r10), %ymm0
	vmovups	%ymm0, (%rdi)
	movl	%r11d, 104(%rdi)
	movq	%r8, 80(%rdi)
	movq	%r9, 88(%rdi)
	movq	%rax, 96(%rdi)
	movq	%rbx, 32(%rdi)
	movq	%rdx, 40(%rdi)
	movq	%r14, 48(%rdi)
	movq	%rdx, 56(%rdi)
	movq	%rsi, 64(%rdi)
	movq	%rcx, 72(%rdi)
.Ltmp4411:
	.loc	1 1474 6 epilogue_begin
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB30_4:
	.cfi_def_cfa_offset 32
.Ltmp4412:
	.loc	5 456 13
	leaq	.Lalloc_c8124f7a6de0156cfd71e0a65b4b69a2(%rip), %rcx
.Ltmp4413:
	.loc	5 456 13 is_stmt 0
	xorl	%edi, %edi
	movq	%rdx, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4414:
.LBB30_12:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_6eb85161aaa18d5606c3983debc5991b(%rip), %r8
	xorl	%edi, %edi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	movq	%r8, %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4415:
.LBB30_13:
	.loc	5 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4416:
.LBB30_10:
	.loc	5 456 13
	leaq	.Lalloc_624ccde1cab09599ea02385287f48f78(%rip), %rcx
.Ltmp4417:
	.loc	5 456 13 is_stmt 0
	xorl	%edi, %edi
	movq	%rdx, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4418:
.LBB30_6:
	.loc	1 1465 20 is_stmt 1
	leaq	.Lalloc_4552947ca1e665baf4644d94967030f9(%rip), %rdx
	.loc	1 0 0 is_stmt 0
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB30_8:
.Ltmp4419:
	.loc	1 1468 20 is_stmt 1
	leaq	.Lalloc_823a10f588a6cbdf8b85bd154459b44e(%rip), %rdx
.Ltmp4420:
	.loc	1 0 0 is_stmt 0
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4421:
.Lfunc_end30:
	.size	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_, .Lfunc_end30-_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
