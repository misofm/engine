_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState13clear_runtime:
.Lfunc_begin14:
	.loc	1 604 0
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movq	%rdi, %rbx
.Ltmp3158:
	.loc	1 606 9 prologue_end
	movq	8(%rdi), %rdx
.Ltmp3159:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp3160:
	.loc	6 180 28
	je	.LBB14_2
.Ltmp3161:
	.loc	16 961 18
	shlq	$2, %rdx
.Ltmp3162:
	.loc	1 606 9
	movq	(%rbx), %rdi
.Ltmp3163:
	.loc	20 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp3164:
.LBB14_2:
	.loc	1 607 9
	movq	24(%rbx), %rdx
.Ltmp3165:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp3166:
	.loc	6 180 28
	je	.LBB14_4
.Ltmp3167:
	.loc	16 961 18
	shlq	$2, %rdx
.Ltmp3168:
	.loc	1 607 9
	movq	16(%rbx), %rdi
.Ltmp3169:
	.loc	20 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp3170:
.LBB14_4:
	.loc	1 608 9
	movq	40(%rbx), %rax
.Ltmp3171:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp3172:
	.loc	6 180 28
	je	.LBB14_7
.Ltmp3173:
	.loc	6 0 28 is_stmt 0
	movq	32(%rbx), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB14_6:
.Ltmp3174:
	.loc	20 25 13 is_stmt 1
	movl	$1065353216, (%rcx,%rdx)
.Ltmp3175:
	.loc	7 1714 9
	addq	$4, %rdx
	cmpq	%rdx, %rax
.Ltmp3176:
	.loc	6 180 28
	jne	.LBB14_6
.Ltmp3177:
.LBB14_7:
	.loc	1 609 9
	movq	56(%rbx), %rax
.Ltmp3178:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp3179:
	.loc	6 180 28
	je	.LBB14_10
.Ltmp3180:
	.loc	6 0 28 is_stmt 0
	movq	48(%rbx), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB14_9:
.Ltmp3181:
	.loc	20 25 13 is_stmt 1
	movl	$1065353216, (%rcx,%rdx)
.Ltmp3182:
	.loc	7 1714 9
	addq	$4, %rdx
	cmpq	%rdx, %rax
.Ltmp3183:
	.loc	6 180 28
	jne	.LBB14_9
.Ltmp3184:
.LBB14_10:
	.loc	1 610 9
	movq	72(%rbx), %rdx
.Ltmp3185:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp3186:
	.loc	6 180 28
	je	.LBB14_12
.Ltmp3187:
	.loc	16 961 18
	shlq	$2, %rdx
.Ltmp3188:
	.loc	1 610 9
	movq	64(%rbx), %rdi
.Ltmp3189:
	.loc	20 25 13
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp3190:
.LBB14_12:
	.loc	1 611 9
	movq	88(%rbx), %rax
.Ltmp3191:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp3192:
	.loc	6 180 28
	je	.LBB14_15
.Ltmp3193:
	.loc	6 0 28 is_stmt 0
	movq	80(%rbx), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB14_14:
.Ltmp3194:
	.loc	20 25 13 is_stmt 1
	movl	$1065353216, (%rcx,%rdx)
.Ltmp3195:
	.loc	7 1714 9
	addq	$4, %rdx
	cmpq	%rdx, %rax
.Ltmp3196:
	.loc	6 180 28
	jne	.LBB14_14
.Ltmp3197:
.LBB14_15:
	.loc	1 612 9
	movq	112(%rbx), %rdi
	movq	120(%rbx), %rdx
.Ltmp3198:
	.loc	20 60 29
	shlq	$2, %rdx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
.Ltmp3199:
	.loc	1 613 29
	movq	104(%rbx), %rax
	.loc	1 613 57 is_stmt 0
	movq	184(%rbx), %rcx
.Ltmp3200:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rax, %rcx
	cmovbq	%rcx, %rax
.Ltmp3201:
	.loc	14 304 12
	testq	%rax, %rax
	je	.LBB14_18
.Ltmp3202:
	.loc	14 0 12 is_stmt 0
	movq	96(%rbx), %rcx
	movq	176(%rbx), %rdx
	xorl	%esi, %esi
	.p2align	4
.LBB14_17:
.Ltmp3203:
	.loc	1 614 20 is_stmt 1
	movl	(%rdx), %edi
	.loc	1 614 13 is_stmt 0
	vcvtsi2ss	%rdi, %xmm15, %xmm0
	vmovss	%xmm0, (%rcx,%rsi,4)
.Ltmp3204:
	.loc	14 308 13 is_stmt 1
	incq	%rsi
.Ltmp3205:
	.loc	14 304 12
	addq	$12, %rdx
	cmpq	%rsi, %rax
	jne	.LBB14_17
.Ltmp3206:
.LBB14_18:
	.loc	1 616 6 epilogue_begin
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Ltmp3207:
.Lfunc_end14:
	.size	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState13clear_runtime, .Lfunc_end14-_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState13clear_runtime
