_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_:
.Lfunc_begin31:
	.loc	1 1462 0 is_stmt 1
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	.loc	1 1465 20 prologue_end
	cmpq	$0, 184(%rsi)
	je	.LBB31_4
.Ltmp4422:
	.loc	52 51 9
	cmpq	$0, 88(%rsi)
	je	.LBB31_2
.Ltmp4423:
	.loc	1 1468 20
	cmpq	$0, 120(%rsi)
	je	.LBB31_13
	.loc	1 1470 33
	movq	40(%rsi), %rax
	cmpq	%rax, %rdx
.Ltmp4424:
	.loc	4 1050 16
	ja	.LBB31_7
.Ltmp4425:
	.loc	1 1471 28
	movq	56(%rsi), %rax
.Ltmp4426:
	.loc	5 451 16
	cmpq	%rax, %rdx
	ja	.LBB31_10
.Ltmp4427:
	.loc	1 1472 29
	movq	24(%rsi), %rax
	cmpq	%rax, %rcx
.Ltmp4428:
	.loc	4 1050 16
	ja	.LBB31_12
.Ltmp4429:
	.loc	1 0 0 is_stmt 0
	movq	80(%rsi), %rax
	movq	176(%rsi), %r8
	movl	(%r8), %r9d
	movl	4(%r8), %r10d
	movl	8(%r8), %r8d
.Ltmp4430:
	vmovss	(%rax), %xmm0
	movq	112(%rsi), %rax
	movl	(%rax), %eax
	movq	32(%rsi), %r11
	movq	48(%rsi), %rbx
	.loc	1 1472 29 is_stmt 1
	movq	16(%rsi), %rsi
	.loc	1 1466 9
	vmovss	%xmm0, 72(%rdi)
	movl	%eax, 76(%rdi)
	movq	%r9, 48(%rdi)
	movq	%r10, 56(%rdi)
	movq	%r8, 64(%rdi)
	movq	%r11, (%rdi)
	movq	%rdx, 8(%rdi)
	movq	%rbx, 16(%rdi)
	movq	%rdx, 24(%rdi)
	movq	%rsi, 32(%rdi)
	movq	%rcx, 40(%rdi)
.Ltmp4431:
	.loc	1 1474 6 epilogue_begin
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.LBB31_7:
	.cfi_def_cfa_offset 16
.Ltmp4432:
	.loc	5 456 13
	leaq	.Lalloc_c8124f7a6de0156cfd71e0a65b4b69a2(%rip), %rcx
.Ltmp4433:
	.loc	5 456 13 is_stmt 0
	xorl	%edi, %edi
	movq	%rdx, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4434:
.LBB31_12:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_6eb85161aaa18d5606c3983debc5991b(%rip), %r8
	xorl	%edi, %edi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	movq	%r8, %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4435:
.LBB31_10:
	.loc	5 456 13
	leaq	.Lalloc_624ccde1cab09599ea02385287f48f78(%rip), %rcx
.Ltmp4436:
	.loc	5 456 13 is_stmt 0
	xorl	%edi, %edi
	movq	%rdx, %rsi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4437:
.LBB31_4:
	.loc	1 1465 20 is_stmt 1
	leaq	.Lalloc_4552947ca1e665baf4644d94967030f9(%rip), %rdx
	.loc	1 0 0 is_stmt 0
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB31_2:
.Ltmp4438:
	.loc	52 51 9 is_stmt 1
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
.Ltmp4439:
	.loc	1 0 0 is_stmt 0
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB31_13:
.Ltmp4440:
	.loc	1 1468 20 is_stmt 1
	leaq	.Lalloc_823a10f588a6cbdf8b85bd154459b44e(%rip), %rdx
.Ltmp4441:
	.loc	1 0 0 is_stmt 0
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4442:
.Lfunc_end31:
	.size	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_, .Lfunc_end31-_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_
