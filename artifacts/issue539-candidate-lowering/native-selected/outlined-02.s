_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_:
.Lfunc_begin27:
	.loc	1 1320 0
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
	subq	$40, %rsp
	.cfi_def_cfa_offset 96
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	.loc	1 949 19 prologue_end
	movq	8(%rsi), %rdx
.Ltmp4165:
	.file	52 "/home/bl/misofm/engine-limiter-stationary-dispatch" "crates/lane/src/scalar.rs"
	.loc	52 51 9
	testq	%rdx, %rdx
	je	.LBB27_39
.Ltmp4166:
	.loc	1 0 0 is_stmt 0
	movq	192(%rsi), %rax
.Ltmp4167:
	.loc	5 568 12 is_stmt 1
	cmpq	%rdx, %rax
	ja	.LBB27_40
.Ltmp4168:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4169:
	.loc	1 949 57
	leaq	(%rax,%rax), %rbp
.Ltmp4170:
	.loc	5 568 12
	cmpq	%rdx, %rbp
	ja	.LBB27_41
.Ltmp4171:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4172:
	.loc	1 949 57
	leaq	(%rax,%rax,2), %r8
.Ltmp4173:
	.loc	5 568 12
	cmpq	%rdx, %r8
	ja	.LBB27_42
.Ltmp4174:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4175:
	.loc	1 949 57
	leaq	(,%rax,4), %r9
.Ltmp4176:
	.loc	5 568 12
	cmpq	%rdx, %r9
	ja	.LBB27_43
.Ltmp4177:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4178:
	.loc	1 949 57
	leaq	(%rax,%rax,4), %r10
.Ltmp4179:
	.loc	5 568 12
	cmpq	%rdx, %r10
	ja	.LBB27_44
.Ltmp4180:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4181:
	.loc	1 949 57
	leaq	(,%rbp,2), %r11
	addq	%rbp, %r11
.Ltmp4182:
	.loc	5 568 12
	cmpq	%rdx, %r11
	ja	.LBB27_45
.Ltmp4183:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4184:
	.loc	1 949 57
	leaq	(,%rax,8), %rbx
	movq	%rbx, %r14
	subq	%rax, %r14
.Ltmp4185:
	.loc	5 568 12
	cmpq	%rdx, %r14
	ja	.LBB27_46
.Ltmp4186:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4187:
	.loc	5 568 12
	cmpq	%rdx, %rbx
	ja	.LBB27_47
.Ltmp4188:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4189:
	.loc	1 949 57
	leaq	(%rax,%rax,8), %r15
.Ltmp4190:
	.loc	5 568 12
	cmpq	%rdx, %r15
	ja	.LBB27_48
.Ltmp4191:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4192:
	.loc	1 949 57
	leaq	(,%rbp,4), %r12
	addq	%rbp, %r12
.Ltmp4193:
	.loc	5 568 12
	cmpq	%rdx, %r12
	ja	.LBB27_49
.Ltmp4194:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4195:
	.loc	1 949 57
	leaq	(%rax,%r10,2), %r13
.Ltmp4196:
	.loc	5 568 12
	cmpq	%rdx, %r13
	ja	.LBB27_50
.Ltmp4197:
	.loc	52 51 9
	je	.LBB27_39
.Ltmp4198:
	.loc	1 0 0 is_stmt 0
	movq	(%rsi), %rcx
.Ltmp4199:
	.loc	52 51 9
	vmovss	(%rcx), %xmm4
	vmovss	(%rcx,%rax,4), %xmm5
	vmovss	(%rcx,%rbp,4), %xmm6
	vmovss	(%rcx,%r8,4), %xmm7
	vmovss	(%rcx,%r9,4), %xmm8
	vmovss	(%rcx,%r10,4), %xmm9
	vmovss	(%rcx,%r11,4), %xmm10
	vmovss	(%rcx,%r14,4), %xmm11
	vmovss	(%rcx,%rbx,4), %xmm12
	vmovss	(%rcx,%r15,4), %xmm13
	vmovss	(%rcx,%r12,4), %xmm0
.Ltmp4200:
	.loc	52 51 9
	vmovss	(%rcx,%r13,4), %xmm1
.Ltmp4201:
	.loc	1 1323 30 is_stmt 1
	movq	184(%rsi), %rax
.Ltmp4202:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp4203:
	.loc	6 180 28
	je	.LBB27_24
	cmpq	$8, %rax
	ja	.LBB27_38
.Ltmp4204:
	.loc	6 0 28 is_stmt 0
	movq	176(%rsi), %rax
	movl	(%rax), %eax
	vcvtsi2ss	%rax, %xmm15, %xmm14
.Ltmp4205:
	.loc	52 51 9 is_stmt 1
	cmpq	$0, 72(%rsi)
	jne	.LBB27_28
	jmp	.LBB27_39
.LBB27_24:
	.loc	52 0 9 is_stmt 0
	vxorps	%xmm14, %xmm14, %xmm14
	.loc	52 51 9 is_stmt 1
	cmpq	$0, 72(%rsi)
	je	.LBB27_39
.Ltmp4206:
.LBB27_28:
	.loc	52 51 9 is_stmt 0
	cmpq	$0, 104(%rsi)
	je	.LBB27_39
.Ltmp4207:
	.loc	1 0 0
	movq	64(%rsi), %rax
	vmovss	(%rax), %xmm15
	.loc	1 1329 0 is_stmt 1
	movq	96(%rsi), %rax
.Ltmp4208:
	.loc	52 51 9
	vmovss	(%rax), %xmm2
	vmovss	%xmm2, 32(%rsp)
.Ltmp4209:
	.loc	1 1331 38
	movq	136(%rsi), %rax
	vxorps	%xmm2, %xmm2, %xmm2
	vmovss	%xmm2, 8(%rsp)
	vmovss	%xmm2, 16(%rsp)
	vmovss	%xmm2, 12(%rsp)
	vxorps	%xmm3, %xmm3, %xmm3
.Ltmp4210:
	.loc	7 1714 9
	testq	%rax, %rax
	vmovss	%xmm14, 36(%rsp)
.Ltmp4211:
	.loc	6 180 28
	je	.LBB27_32
	cmpq	$9, %rax
	jae	.LBB27_36
.Ltmp4212:
	.loc	6 0 28 is_stmt 0
	vmovaps	%xmm1, %xmm14
	vmovaps	%xmm0, %xmm1
	vmovaps	%xmm13, %xmm0
	vmovaps	%xmm12, %xmm13
	vmovaps	%xmm11, %xmm12
	vmovaps	%xmm10, %xmm11
	vmovaps	%xmm9, %xmm10
	vmovaps	%xmm8, %xmm9
	vmovaps	%xmm7, %xmm8
	vmovaps	%xmm6, %xmm7
	vmovaps	%xmm5, %xmm6
	vmovaps	%xmm4, %xmm5
	.loc	1 1331 0 is_stmt 1
	movq	128(%rsi), %rax
.Ltmp4213:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rax), %xmm3
	vmovss	4(%rax), %xmm2
	vmovss	8(%rax), %xmm4
	vmovss	%xmm4, 12(%rsp)
	movzwl	12(%rax), %eax
	vcvtsi2ss	%eax, %xmm15, %xmm4
	vmovss	%xmm4, 16(%rsp)
	vmovaps	%xmm5, %xmm4
	vmovaps	%xmm6, %xmm5
	vmovaps	%xmm7, %xmm6
	vmovaps	%xmm8, %xmm7
	vmovaps	%xmm9, %xmm8
	vmovaps	%xmm10, %xmm9
	vmovaps	%xmm11, %xmm10
	vmovaps	%xmm12, %xmm11
	vmovaps	%xmm13, %xmm12
	vmovaps	%xmm0, %xmm13
	vmovaps	%xmm1, %xmm0
	vmovaps	%xmm14, %xmm1
.Ltmp4214:
.LBB27_32:
	vmovss	%xmm15, 24(%rsp)
	vmovss	%xmm1, 28(%rsp)
	vmovss	%xmm0, 20(%rsp)
	vmovaps	%xmm13, %xmm0
	vmovaps	%xmm12, %xmm15
	vmovaps	%xmm11, %xmm14
	vmovaps	%xmm10, %xmm13
	vmovaps	%xmm9, %xmm12
	vmovaps	%xmm8, %xmm11
	vmovaps	%xmm7, %xmm10
	vmovaps	%xmm6, %xmm9
	vmovaps	%xmm5, %xmm8
	vmovaps	%xmm4, %xmm7
	.loc	1 1332 40 is_stmt 1
	movq	152(%rsi), %rax
	vxorps	%xmm4, %xmm4, %xmm4
	vxorps	%xmm5, %xmm5, %xmm5
	vxorps	%xmm6, %xmm6, %xmm6
.Ltmp4215:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp4216:
	.loc	6 180 28
	je	.LBB27_35
	cmpq	$9, %rax
	jae	.LBB27_36
.Ltmp4217:
	.loc	1 1332 0
	movq	144(%rsi), %rax
.Ltmp4218:
	.loc	1 0 0 is_stmt 0
	vmovss	(%rax), %xmm6
	vmovss	4(%rax), %xmm5
	vmovss	8(%rax), %xmm4
	movzwl	12(%rax), %eax
	vcvtsi2ss	%eax, %xmm2, %xmm1
	vmovss	%xmm1, 8(%rsp)
.Ltmp4219:
.LBB27_35:
	.loc	1 1326 9 is_stmt 1
	vmovss	%xmm7, (%rdi)
	vmovss	%xmm8, 4(%rdi)
	vmovss	%xmm9, 8(%rdi)
	vmovss	%xmm10, 12(%rdi)
	vmovss	%xmm11, 16(%rdi)
	vmovss	%xmm12, 20(%rdi)
	vmovss	%xmm13, 24(%rdi)
	vmovss	%xmm14, 28(%rdi)
	vmovss	%xmm15, 32(%rdi)
	vmovss	%xmm0, 36(%rdi)
	vmovss	20(%rsp), %xmm0
	vmovss	%xmm0, 40(%rdi)
	vmovss	28(%rsp), %xmm0
	vmovss	%xmm0, 44(%rdi)
	vmovss	24(%rsp), %xmm0
	vmovss	%xmm0, 80(%rdi)
	vmovss	32(%rsp), %xmm0
	vmovss	%xmm0, 84(%rdi)
	vmovss	36(%rsp), %xmm0
	vmovss	%xmm0, 88(%rdi)
	vmovss	%xmm3, 48(%rdi)
	vmovss	%xmm2, 52(%rdi)
	vmovss	12(%rsp), %xmm0
	vmovss	%xmm0, 56(%rdi)
	vmovss	16(%rsp), %xmm0
	vmovss	%xmm0, 60(%rdi)
	vmovss	%xmm6, 64(%rdi)
	vmovss	%xmm5, 68(%rdi)
	vmovss	%xmm4, 72(%rdi)
	vmovss	8(%rsp), %xmm0
	vmovss	%xmm0, 76(%rdi)
.Ltmp4220:
	.loc	1 1334 6 epilogue_begin
	addq	$40, %rsp
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
.LBB27_40:
	.cfi_def_cfa_offset 96
.Ltmp4221:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4222:
.LBB27_41:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%rbp, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4223:
.LBB27_42:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r8, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4224:
.LBB27_43:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r9, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4225:
.LBB27_44:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r10, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4226:
.LBB27_45:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r11, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4227:
.LBB27_46:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r14, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4228:
.LBB27_47:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%rbx, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4229:
.LBB27_48:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r15, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4230:
.LBB27_49:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r12, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4231:
.LBB27_50:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r13, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4232:
.LBB27_39:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_448b96be3252b33bba65fff6359cc55f(%rip), %rdx
	xorl	%edi, %edi
	xorl	%esi, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB27_36:
.Ltmp4233:
	.loc	1 815 13 is_stmt 1
	leaq	.Lalloc_34ec9deef3d1165c325897b5845d9fe1(%rip), %rdx
.Ltmp4234:
	.loc	1 0 0 is_stmt 0
	movl	$8, %edi
	movl	$8, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB27_38:
.Ltmp4235:
	.loc	1 1324 13 is_stmt 1
	leaq	.Lalloc_3862c97910d0689d8b59a4255cb98beb(%rip), %rdx
.Ltmp4236:
	.loc	1 0 0 is_stmt 0
	movl	$8, %edi
	movl	$8, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4237:
.Lfunc_end27:
	.size	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_, .Lfunc_end27-_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_
