_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_:
.Lfunc_begin34:
	.loc	1 2184 0
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
	subq	$4096, %rsp
	movq	$0, (%rsp)
	subq	$3104, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%r8, 88(%rsp)
	movq	%rcx, 432(%rsp)
	movq	%rsi, %rbx
	movq	%rdi, %r12
	movq	%r9, 312(%rsp)
.Ltmp7133:
	.loc	1 2185 21 prologue_end
	leaq	(,%r9,8), %r9
.Ltmp7134:
	.loc	1 2203 21
	movzbl	2153(%rdi), %ecx
	.loc	1 0 0 is_stmt 0
	movq	1784(%rdi), %rax
	.loc	1 2203 21
	cmpb	2144(%rdi), %cl
	movq	%r9, 4216(%rsp)
	jne	.LBB34_1
	.loc	1 2204 37 is_stmt 1
	movq	1776(%r12), %rcx
.Ltmp7135:
	.loc	6 314 17
	movq	%rax, %r8
	shlq	$4, %r8
	movq	%rcx, %rsi
	.loc	6 0 17 is_stmt 0
.Ltmp7136:
	.p2align	4
.LBB34_3:
.Ltmp7137:
	.loc	7 1714 9 is_stmt 1
	testq	%r8, %r8
.Ltmp7138:
	.loc	6 180 28
	je	.LBB34_6
.Ltmp7139:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp7140:
	.loc	6 315 25
	jne	.LBB34_30
	addq	$-16, %r8
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB34_3
	jmp	.LBB34_30
.Ltmp7141:
.LBB34_1:
	.loc	1 707 26 is_stmt 1
	movq	1776(%r12), %rcx
.Ltmp7142:
.LBB34_30:
	.loc	1 0 26 is_stmt 0
	xorl	%esi, %esi
.LBB34_31:
	movl	%esi, 2556(%rsp)
	.loc	1 2239 13 is_stmt 1
	leaq	1616(%r12), %rsi
	movq	%rsi, 1048(%rsp)
	.loc	1 2240 13
	leaq	1648(%r12), %rsi
	movq	%rsi, 760(%rsp)
	.loc	1 2241 13
	leaq	1848(%r12), %rsi
	movq	%rsi, 752(%rsp)
.Ltmp7143:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp7144:
	.p2align	4
.LBB34_32:
.Ltmp7145:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7146:
	.loc	6 180 28
	je	.LBB34_37
.Ltmp7147:
	.loc	6 0 28 is_stmt 0
	xorl	%r8d, %r8d
.Ltmp7148:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
	jne	.LBB34_50
.Ltmp7149:
	.loc	6 315 25
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB34_32
	jmp	.LBB34_50
.Ltmp7150:
.LBB34_37:
	.loc	1 708 33 is_stmt 1
	movq	1792(%r12), %rcx
	movq	1800(%r12), %rax
.Ltmp7151:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp7152:
	.p2align	4
.LBB34_38:
.Ltmp7153:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7154:
	.loc	6 180 28
	je	.LBB34_41
.Ltmp7155:
	.loc	6 0 28 is_stmt 0
	xorl	%r8d, %r8d
.Ltmp7156:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7157:
	.loc	6 315 25
	jne	.LBB34_50
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB34_38
	jmp	.LBB34_50
.Ltmp7158:
.LBB34_41:
	.loc	1 709 33 is_stmt 1
	movq	1976(%r12), %rcx
	movq	1984(%r12), %rax
.Ltmp7159:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp7160:
	.p2align	4
.LBB34_42:
.Ltmp7161:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7162:
	.loc	6 180 28
	je	.LBB34_45
.Ltmp7163:
	.loc	6 0 28 is_stmt 0
	xorl	%r8d, %r8d
.Ltmp7164:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7165:
	.loc	6 315 25
	jne	.LBB34_50
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB34_42
	jmp	.LBB34_50
.Ltmp7166:
.LBB34_6:
	.loc	1 2205 37 is_stmt 1
	movq	1792(%r12), %rsi
	movq	1800(%r12), %r8
.Ltmp7167:
	.loc	6 314 17
	shlq	$4, %r8
	.loc	6 0 17 is_stmt 0
.Ltmp7168:
	.p2align	4
.LBB34_7:
.Ltmp7169:
	.loc	7 1714 9 is_stmt 1
	testq	%r8, %r8
.Ltmp7170:
	.loc	6 180 28
	je	.LBB34_10
.Ltmp7171:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp7172:
	.loc	6 315 25
	jne	.LBB34_30
	addq	$-16, %r8
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB34_7
	jmp	.LBB34_30
.Ltmp7173:
.LBB34_45:
	.loc	1 710 33 is_stmt 1
	movq	1992(%r12), %rcx
	movq	2000(%r12), %rax
.Ltmp7174:
	.loc	6 314 17
	shlq	$4, %rax
	.loc	6 0 17 is_stmt 0
.Ltmp7175:
	.p2align	4
.LBB34_46:
.Ltmp7176:
	.loc	7 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7177:
	.loc	6 180 28
	je	.LBB34_47
.Ltmp7178:
	.loc	6 0 28 is_stmt 0
	xorl	%r8d, %r8d
.Ltmp7179:
	.loc	1 690 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7180:
	.loc	6 315 25
	jne	.LBB34_50
	addq	$-16, %rax
	movl	4(%rcx), %esi
	cmpl	%esi, (%rcx)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	6 315 25
	je	.LBB34_46
	jmp	.LBB34_50
.Ltmp7181:
.LBB34_10:
	.loc	1 2206 37 is_stmt 1
	movq	1976(%r12), %rsi
	movq	1984(%r12), %r8
.Ltmp7182:
	.loc	6 314 17
	shlq	$4, %r8
	.loc	6 0 17 is_stmt 0
.Ltmp7183:
	.p2align	4
.LBB34_11:
.Ltmp7184:
	.loc	7 1714 9 is_stmt 1
	testq	%r8, %r8
.Ltmp7185:
	.loc	6 180 28
	je	.LBB34_14
.Ltmp7186:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp7187:
	.loc	6 315 25
	jne	.LBB34_30
	addq	$-16, %r8
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB34_11
	jmp	.LBB34_30
.Ltmp7188:
.LBB34_47:
	.loc	6 0 25
	movb	$1, %r8b
.LBB34_50:
.Ltmp7189:
	.loc	1 1116 5 is_stmt 1
	movq	1832(%r12), %rcx
	testq	%rcx, %rcx
	movq	%r12, 440(%rsp)
	movq	%rdx, 184(%rsp)
	movq	%rbx, 536(%rsp)
	movq	312(%rsp), %rdx
	je	.LBB34_56
	.loc	1 0 5 is_stmt 0
	movq	1824(%r12), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rsi
	.p2align	4
.LBB34_52:
.Ltmp7190:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp7191:
	.loc	6 180 28
	je	.LBB34_56
.Ltmp7192:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rax), %edi
	jne	.LBB34_70
	movl	4(%rsi), %edi
	cmpl	4(%rax), %edi
	jne	.LBB34_70
	movl	8(%rsi), %edi
.Ltmp7193:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp7194:
	.loc	1 390 34
	cmpl	8(%rax), %edi
.Ltmp7195:
	.loc	6 315 25
	je	.LBB34_52
	jmp	.LBB34_70
.Ltmp7196:
.LBB34_56:
	.loc	1 1117 12
	movq	1768(%r12), %rax
	testq	%rax, %rax
	je	.LBB34_60
	.loc	1 0 12 is_stmt 0
	movq	1760(%r12), %rcx
	shlq	$2, %rax
	xorl	%esi, %esi
	.p2align	4
.LBB34_58:
.Ltmp7197:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rax
.Ltmp7198:
	.loc	6 180 28
	je	.LBB34_60
.Ltmp7199:
	.loc	6 315 25
	movl	(%rcx,%rsi), %edi
	addq	$4, %rsi
.Ltmp7200:
	.loc	1 1117 43
	cmpl	(%rcx), %edi
.Ltmp7201:
	.loc	6 315 25
	je	.LBB34_58
	jmp	.LBB34_70
.Ltmp7202:
.LBB34_60:
	.loc	1 1116 5
	movq	2032(%r12), %rcx
	testq	%rcx, %rcx
	je	.LBB34_66
	.loc	1 0 5 is_stmt 0
	movq	2024(%r12), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rsi
	.p2align	4
.LBB34_62:
.Ltmp7203:
	.loc	7 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp7204:
	.loc	6 180 28
	je	.LBB34_66
.Ltmp7205:
	.loc	1 390 34
	movl	(%rsi), %edi
	cmpl	(%rax), %edi
	jne	.LBB34_70
	movl	4(%rsi), %edi
	cmpl	4(%rax), %edi
	jne	.LBB34_70
	movl	8(%rsi), %edi
.Ltmp7206:
	.loc	6 0 0 is_stmt 0
	addq	$12, %rsi
	.loc	6 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp7207:
	.loc	1 390 34
	cmpl	8(%rax), %edi
.Ltmp7208:
	.loc	6 315 25
	je	.LBB34_62
	jmp	.LBB34_70
.Ltmp7209:
.LBB34_66:
	.loc	1 1117 12
	movq	1968(%r12), %rax
	testq	%rax, %rax
	je	.LBB34_399
	.loc	1 0 12 is_stmt 0
	movq	1960(%r12), %rcx
	shlq	$2, %rax
	xorl	%esi, %esi
	.p2align	4
.LBB34_68:
.Ltmp7210:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rax
.Ltmp7211:
	.loc	6 180 28
	je	.LBB34_399
.Ltmp7212:
	.loc	6 315 25
	movl	(%rcx,%rsi), %edi
	addq	$4, %rsi
.Ltmp7213:
	.loc	1 1117 43
	cmpl	(%rcx), %edi
.Ltmp7214:
	.loc	6 315 25
	je	.LBB34_68
.Ltmp7215:
.LBB34_70:
	.loc	1 0 0 is_stmt 0
	leaq	31(%rdx), %r13
	shrq	$5, %r13
	.loc	1 1706 12 is_stmt 1
	testb	%r8b, %r8b
	je	.LBB34_71
	.loc	1 0 12 is_stmt 0
	leaq	7712(%rsp), %rdi
.Ltmp7216:
	.loc	1 1794 24 is_stmt 1
	leaq	1648(%r12), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	8448(%rsp), %rdi
.Ltmp7217:
	.loc	1 1795 25
	leaq	1848(%r12), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp7218:
	.loc	1 1800 19
	movzbl	1536(%r12), %ebx
.Ltmp7219:
	.loc	1 1801 21
	movzbl	1537(%r12), %eax
	movb	%al, 32(%rsp)
.Ltmp7220:
	.loc	1 1802 27
	movl	1640(%r12), %r14d
.Ltmp7221:
	.loc	1 1803 27
	movl	1644(%r12), %eax
	movq	%rax, 224(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 576(%rsp)
	leaq	10216(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%r15
	leaq	9184(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
.Ltmp7222:
	.loc	1 0 0 is_stmt 0
	testq	%r13, %r13
.Ltmp7223:
	.loc	8 446 20 is_stmt 1
	je	.LBB34_254
.Ltmp7224:
	.loc	8 0 20 is_stmt 0
	movq	%r13, 424(%rsp)
.Ltmp7225:
	.loc	29 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm6
.Ltmp7226:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm6, %ymm0
	vmovaps	%ymm6, 2240(%rsp)
	testb	%bl, %bl
	jne	.LBB34_258
.Ltmp7227:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 2240(%rsp)
.LBB34_258:
	cmpb	$0, 32(%rsp)
	movq	312(%rsp), %rdx
	jne	.LBB34_260
	vmovaps	%ymm0, %ymm6
.LBB34_260:
	movq	$0, 768(%rsp)
	vxorps	%xmm7, %xmm7, %xmm7
	movq	432(%rsp), %rdi
	movq	536(%rsp), %r15
	movq	%rdx, %rax
	xorl	%r11d, %r11d
	movq	184(%rsp), %rbx
	vmovaps	%ymm6, 1632(%rsp)
.Ltmp7228:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB34_263
.Ltmp7229:
	.loc	8 0 20 is_stmt 0
.Ltmp7230:
	.p2align	4
.LBB34_262:
	addq	$32, %r11
	movq	424(%rsp), %rax
	decq	%rax
	movq	704(%rsp), %rcx
.Ltmp7231:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	addq	$1024, %r15
	addq	$1024, %rdi
	movq	%rax, 424(%rsp)
	testq	%rax, %rax
	movq	%rcx, %rax
	je	.LBB34_255
.LBB34_263:
.Ltmp7232:
	.loc	4 2584 13
	cmpq	$1, %rax
	movq	%rax, 704(%rsp)
	movq	%rax, %rcx
	adcq	$0, %rcx
	cmpq	$32, %rcx
	movl	$32, %eax
	cmovaeq	%rax, %rcx
	movq	%rcx, 2560(%rsp)
.Ltmp7233:
	.loc	1 1815 51
	movq	%rdx, %rsi
	subq	%r11, %rsi
.Ltmp7234:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%rax, %rsi
.Ltmp7235:
	.loc	1 1816 27
	leaq	(,%r11,8), %rax
.Ltmp7236:
	.loc	1 1820 35
	addq	%r11, %rsi
	shlq	$3, %rsi
.Ltmp7237:
	.loc	4 1050 16
	cmpq	%rax, %rsi
	jb	.LBB34_623
	cmpq	%rbx, %rsi
	ja	.LBB34_623
.Ltmp7238:
	.loc	4 0 16 is_stmt 0
	movq	2560(%rsp), %rcx
	shll	$5, %ecx
.Ltmp7239:
	.loc	1 1759 23 is_stmt 1
	vmovaps	7712(%rsp), %ymm9
	vmovaps	7744(%rsp), %ymm2
	vmovaps	7776(%rsp), %ymm4
	vmovaps	7808(%rsp), %ymm8
	vmovaps	7840(%rsp), %ymm5
	vmovaps	7872(%rsp), %ymm13
	vmovaps	7904(%rsp), %ymm12
	vmovaps	7936(%rsp), %ymm3
	vmovaps	7968(%rsp), %ymm6
	vmovaps	8000(%rsp), %ymm14
	vmovaps	8032(%rsp), %ymm1
.Ltmp7240:
	.loc	11 304 12
	cmpq	%r11, %rdx
	vmovaps	%ymm4, 32(%rsp)
	jne	.LBB34_267
.Ltmp7241:
	.loc	1 0 0 is_stmt 0
	vmovaps	8064(%rsp), %ymm15
	vmovaps	%ymm2, %ymm4
.Ltmp7242:
	.loc	11 304 12
	jmp	.LBB34_269
.Ltmp7243:
	.loc	11 0 12
.Ltmp7244:
	.p2align	4
.LBB34_267:
	vmovaps	(%r12), %ymm0
	vmovaps	%ymm0, 480(%rsp)
	vmovaps	32(%r12), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	vmovaps	64(%r12), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	96(%r12), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	128(%r12), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	160(%r12), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	192(%r12), %ymm0
	vmovaps	%ymm0, 352(%rsp)
	vmovaps	224(%r12), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	256(%r12), %ymm0
	vmovaps	%ymm0, 992(%rsp)
	vmovaps	288(%r12), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	320(%r12), %ymm0
	vmovaps	%ymm0, 928(%rsp)
	xorl	%edx, %edx
	vmovaps	%ymm2, %ymm0
	vmovaps	%ymm1, 384(%rsp)
	vmovaps	%ymm14, %ymm10
	vmovaps	%ymm6, %ymm2
	vmovaps	%ymm3, %ymm11
	vmovaps	352(%r12), %ymm1
	vmovaps	%ymm1, 960(%rsp)
	vmovaps	384(%r12), %ymm1
	vmovaps	%ymm1, 1312(%rsp)
	vmovaps	416(%r12), %ymm1
	vmovaps	%ymm1, 1280(%rsp)
	vmovaps	448(%r12), %ymm1
	vmovaps	%ymm1, 1248(%rsp)
	vmovaps	480(%r12), %ymm1
	vmovaps	%ymm1, 1216(%rsp)
	vmovaps	512(%r12), %ymm1
	vmovaps	%ymm1, 2208(%rsp)
	vmovaps	544(%r12), %ymm1
	vmovaps	%ymm1, 2176(%rsp)
	vmovaps	576(%r12), %ymm1
	vmovaps	%ymm1, 2144(%rsp)
	vmovaps	608(%r12), %ymm1
	vmovaps	%ymm1, 2112(%rsp)
	vmovaps	640(%r12), %ymm1
	vmovaps	%ymm1, 2080(%rsp)
	vmovaps	672(%r12), %ymm1
	vmovaps	%ymm1, 1504(%rsp)
	vmovaps	704(%r12), %ymm1
	vmovaps	%ymm1, 2048(%rsp)
	vmovaps	736(%r12), %ymm1
	vmovaps	%ymm1, 2016(%rsp)
	vmovaps	768(%r12), %ymm1
	vmovaps	%ymm1, 1984(%rsp)
	vmovaps	800(%r12), %ymm1
	vmovaps	%ymm1, 1952(%rsp)
	vmovaps	832(%r12), %ymm1
	vmovaps	%ymm1, 1920(%rsp)
	vmovaps	864(%r12), %ymm1
	vmovaps	%ymm1, 1472(%rsp)
	vmovaps	896(%r12), %ymm1
	vmovaps	%ymm1, 1440(%rsp)
	vmovaps	928(%r12), %ymm1
	vmovaps	%ymm1, 1888(%rsp)
	vmovaps	960(%r12), %ymm1
	vmovaps	%ymm1, 1856(%rsp)
	vmovaps	992(%r12), %ymm1
	vmovaps	%ymm1, 1824(%rsp)
	vmovaps	1024(%r12), %ymm1
	vmovaps	%ymm1, 1792(%rsp)
	vmovaps	1056(%r12), %ymm1
	vmovaps	%ymm1, 1184(%rsp)
	vmovaps	1088(%r12), %ymm1
	vmovaps	%ymm1, 1408(%rsp)
	vmovaps	1120(%r12), %ymm1
	vmovaps	%ymm1, 1760(%rsp)
	vmovaps	1152(%r12), %ymm1
	vmovaps	%ymm1, 1152(%rsp)
	vmovaps	1184(%r12), %ymm1
	vmovaps	%ymm1, 1376(%rsp)
	vmovaps	1216(%r12), %ymm1
	vmovaps	%ymm1, 1728(%rsp)
	vmovaps	1248(%r12), %ymm1
	vmovaps	%ymm1, 1696(%rsp)
	vmovaps	1280(%r12), %ymm1
	vmovaps	%ymm1, 1664(%rsp)
	vmovaps	1312(%r12), %ymm1
	vmovaps	%ymm1, 2400(%rsp)
	vmovaps	1344(%r12), %ymm1
	vmovaps	%ymm1, 2368(%rsp)
	vmovaps	1376(%r12), %ymm1
	vmovaps	%ymm1, 2336(%rsp)
	vmovaps	1408(%r12), %ymm1
	vmovaps	%ymm1, 2304(%rsp)
	vmovaps	1440(%r12), %ymm1
	vmovaps	%ymm1, 2272(%rsp)
	vmovaps	1472(%r12), %ymm1
	vmovaps	%ymm1, 2496(%rsp)
	vmovaps	1504(%r12), %ymm1
	vmovaps	%ymm1, 2464(%rsp)
	.p2align	4
.LBB34_268:
.Ltmp7245:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r15,%rdx), %ymm4
.Ltmp7246:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm4, %ymm1
.Ltmp7247:
	.loc	29 48 14
	vaddps	%ymm7, %ymm1, %ymm1
.Ltmp7248:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm4, %ymm3
.Ltmp7249:
	.loc	29 48 14
	vaddps	%ymm7, %ymm3, %ymm3
.Ltmp7250:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm4, %ymm6
.Ltmp7251:
	.loc	29 48 14
	vaddps	%ymm7, %ymm6, %ymm6
	vmovaps	%ymm9, 448(%rsp)
.Ltmp7252:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm9, %ymm14
.Ltmp7253:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp7254:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm9, %ymm14
.Ltmp7255:
	.loc	29 48 14
	vaddps	%ymm3, %ymm14, %ymm14
.Ltmp7256:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm9, %ymm3
.Ltmp7257:
	.loc	29 48 14
	vaddps	%ymm3, %ymm6, %ymm6
.Ltmp7258:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm4, %ymm3
.Ltmp7259:
	.loc	29 48 14
	vaddps	%ymm7, %ymm3, %ymm3
.Ltmp7260:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm9, %ymm15
.Ltmp7261:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm15
	vmovaps	%ymm12, %ymm3
	vmovaps	%ymm5, %ymm9
	vmovaps	%ymm8, %ymm12
	vmovaps	32(%rsp), %ymm8
	vmovaps	%ymm0, %ymm5
.Ltmp7262:
	.loc	29 283 14
	vmulps	992(%rsp), %ymm0, %ymm0
.Ltmp7263:
	.loc	29 48 14
	vaddps	%ymm0, %ymm1, %ymm0
.Ltmp7264:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm5, %ymm1
.Ltmp7265:
	.loc	29 48 14
	vaddps	%ymm1, %ymm14, %ymm1
.Ltmp7266:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm5, %ymm14
.Ltmp7267:
	.loc	29 48 14
	vaddps	%ymm6, %ymm14, %ymm6
	vmovaps	%ymm5, 32(%rsp)
.Ltmp7268:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm5, %ymm14
.Ltmp7269:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp7270:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm8, %ymm15
.Ltmp7271:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7272:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm8, %ymm15
.Ltmp7273:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp7274:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm8, %ymm15
.Ltmp7275:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7276:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm8, %ymm15
.Ltmp7277:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7278:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm12, %ymm15
.Ltmp7279:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7280:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm12, %ymm15
.Ltmp7281:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp7282:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm12, %ymm15
.Ltmp7283:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7284:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm12, %ymm15
.Ltmp7285:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7286:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm9, %ymm15
.Ltmp7287:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7288:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm9, %ymm15
.Ltmp7289:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp7290:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm9, %ymm15
.Ltmp7291:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7292:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm9, %ymm15
.Ltmp7293:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7294:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm13, %ymm15
.Ltmp7295:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7296:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm13, %ymm15
.Ltmp7297:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp7298:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm13, %ymm15
.Ltmp7299:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp7300:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm13, %ymm15
.Ltmp7301:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7302:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm3, %ymm15
.Ltmp7303:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7304:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm3, %ymm15
.Ltmp7305:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm1
.Ltmp7306:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm3, %ymm15
.Ltmp7307:
	.loc	29 48 14
	vaddps	%ymm6, %ymm15, %ymm15
.Ltmp7308:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm3, %ymm6
.Ltmp7309:
	.loc	29 48 14
	vaddps	%ymm6, %ymm14, %ymm14
	vmovaps	%ymm11, %ymm6
.Ltmp7310:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm11, %ymm11
.Ltmp7311:
	.loc	29 48 14
	vaddps	%ymm0, %ymm11, %ymm0
.Ltmp7312:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm6, %ymm11
.Ltmp7313:
	.loc	29 48 14
	vaddps	%ymm1, %ymm11, %ymm1
.Ltmp7314:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm6, %ymm11
.Ltmp7315:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
.Ltmp7316:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm6, %ymm15
.Ltmp7317:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm15
	vmovaps	%ymm2, %ymm14
.Ltmp7318:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm2, %ymm2
.Ltmp7319:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp7320:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm14, %ymm2
.Ltmp7321:
	.loc	29 48 14
	vaddps	%ymm2, %ymm1, %ymm2
.Ltmp7322:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm14, %ymm1
.Ltmp7323:
	.loc	29 48 14
	vaddps	%ymm1, %ymm11, %ymm11
.Ltmp7324:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm14, %ymm1
.Ltmp7325:
	.loc	29 48 14
	vaddps	%ymm1, %ymm15, %ymm15
	vmovaps	%ymm10, %ymm1
.Ltmp7326:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm10, %ymm10
.Ltmp7327:
	.loc	29 48 14
	vaddps	%ymm0, %ymm10, %ymm0
.Ltmp7328:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm1, %ymm10
.Ltmp7329:
	.loc	29 48 14
	vaddps	%ymm2, %ymm10, %ymm2
.Ltmp7330:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm1, %ymm10
.Ltmp7331:
	.loc	29 48 14
	vaddps	%ymm10, %ymm11, %ymm10
.Ltmp7332:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm1, %ymm11
.Ltmp7333:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
	vmovaps	384(%rsp), %ymm15
.Ltmp7334:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm15, %ymm5
.Ltmp7335:
	.loc	29 48 14
	vaddps	%ymm5, %ymm0, %ymm0
.Ltmp7336:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm15, %ymm5
.Ltmp7337:
	.loc	29 48 14
	vaddps	%ymm5, %ymm2, %ymm2
.Ltmp7338:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm15, %ymm5
.Ltmp7339:
	.loc	29 48 14
	vaddps	%ymm5, %ymm10, %ymm5
.Ltmp7340:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm15, %ymm10
.Ltmp7341:
	.loc	29 48 14
	vaddps	%ymm10, %ymm11, %ymm10
.Ltmp7342:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm11
.Ltmp7343:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm0, %ymm11, %ymm0
.Ltmp7344:
	.loc	29 82 19
	vandps	%ymm11, %ymm13, %ymm7
.Ltmp7345:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm7, %ymm0
	vxorps	%xmm7, %xmm7, %xmm7
.Ltmp7346:
	.loc	29 82 19
	vandps	%ymm2, %ymm11, %ymm2
.Ltmp7347:
	.loc	29 233 14
	vmaxps	%ymm2, %ymm0, %ymm0
.Ltmp7348:
	.loc	29 82 19
	vandps	%ymm5, %ymm11, %ymm2
	vmovaps	%ymm12, %ymm5
	vmovaps	%ymm13, %ymm12
	vmovaps	%ymm9, %ymm13
	vmovaps	%ymm4, %ymm9
	vmovaps	448(%rsp), %ymm4
.Ltmp7349:
	.loc	29 233 14
	vmaxps	%ymm2, %ymm0, %ymm0
.Ltmp7350:
	.loc	29 82 19
	vandps	%ymm11, %ymm10, %ymm2
.Ltmp7351:
	.loc	29 233 14
	vmaxps	%ymm2, %ymm0, %ymm0
.Ltmp7352:
	.loc	12 551 14
	vmovups	%ymm0, 10216(%rsp,%rdx)
.Ltmp7353:
	.loc	11 304 12
	addq	$32, %rdx
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm1, 384(%rsp)
	vmovaps	%ymm14, %ymm10
	vmovaps	%ymm6, %ymm2
	vmovaps	%ymm3, %ymm11
	cmpq	%rdx, %rcx
	jne	.LBB34_268
.Ltmp7354:
.LBB34_269:
	.loc	1 1764 5
	vmovaps	%ymm9, 7712(%rsp)
	vmovaps	%ymm4, 7744(%rsp)
	vmovaps	32(%rsp), %ymm0
	vmovaps	%ymm0, 7776(%rsp)
	vmovaps	%ymm8, 7808(%rsp)
	vmovaps	%ymm5, 7840(%rsp)
	vmovaps	%ymm13, 7872(%rsp)
	vmovaps	%ymm12, 7904(%rsp)
	vmovaps	%ymm3, 7936(%rsp)
	vmovaps	%ymm6, 7968(%rsp)
	vmovaps	%ymm14, 8000(%rsp)
	vmovaps	%ymm1, 8032(%rsp)
	vmovaps	%ymm15, 8064(%rsp)
.Ltmp7355:
	.loc	5 438 16
	cmpq	88(%rsp), %rsi
	ja	.LBB34_624
.Ltmp7356:
	.loc	1 1759 23
	vmovaps	8448(%rsp), %ymm1
	vmovaps	8480(%rsp), %ymm9
	vmovaps	8512(%rsp), %ymm4
	vmovaps	8544(%rsp), %ymm8
	vmovaps	8576(%rsp), %ymm5
	vmovaps	8608(%rsp), %ymm13
	vmovaps	8640(%rsp), %ymm12
	vmovaps	8672(%rsp), %ymm3
	vmovaps	8704(%rsp), %ymm2
	vmovaps	8736(%rsp), %ymm14
	vmovaps	8768(%rsp), %ymm0
	movq	312(%rsp), %rdx
.Ltmp7357:
	.loc	11 304 12
	cmpq	%r11, %rdx
	vmovaps	%ymm4, 32(%rsp)
.Ltmp7358:
	.loc	11 304 12 is_stmt 0
	jne	.LBB34_272
.Ltmp7359:
	.loc	1 0 0
	vmovaps	8800(%rsp), %ymm15
	vmovaps	%ymm9, %ymm4
.Ltmp7360:
	.loc	11 304 12
	jmp	.LBB34_274
.Ltmp7361:
	.loc	11 0 12
.Ltmp7362:
	.p2align	4
.LBB34_272:
	vmovaps	(%r12), %ymm6
	vmovaps	%ymm6, 480(%rsp)
	vmovaps	32(%r12), %ymm6
	vmovaps	%ymm6, 96(%rsp)
	vmovaps	64(%r12), %ymm6
	vmovaps	%ymm6, 192(%rsp)
	vmovaps	96(%r12), %ymm6
	vmovaps	%ymm6, 544(%rsp)
	vmovaps	128(%r12), %ymm6
	vmovaps	%ymm6, 128(%rsp)
	vmovaps	160(%r12), %ymm6
	vmovaps	%ymm6, 256(%rsp)
	vmovaps	192(%r12), %ymm6
	vmovaps	%ymm6, 352(%rsp)
	vmovaps	224(%r12), %ymm6
	vmovaps	%ymm6, 320(%rsp)
	vmovaps	256(%r12), %ymm6
	vmovaps	%ymm6, 992(%rsp)
	vmovaps	288(%r12), %ymm6
	vmovaps	%ymm6, 1536(%rsp)
	vmovaps	320(%r12), %ymm6
	vmovaps	%ymm6, 928(%rsp)
	xorl	%eax, %eax
	vmovaps	%ymm0, 384(%rsp)
	vmovaps	%ymm14, %ymm6
	vmovaps	%ymm2, %ymm10
	vmovaps	%ymm3, %ymm11
	vmovaps	352(%r12), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	384(%r12), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	416(%r12), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	448(%r12), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	480(%r12), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	vmovaps	512(%r12), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	544(%r12), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	576(%r12), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	608(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	640(%r12), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	672(%r12), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	704(%r12), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	736(%r12), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	768(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	800(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	832(%r12), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	864(%r12), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	896(%r12), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	928(%r12), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	960(%r12), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	992(%r12), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	1024(%r12), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1056(%r12), %ymm0
	vmovaps	%ymm0, 1184(%rsp)
	vmovaps	1088(%r12), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	1120(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1152(%r12), %ymm0
	vmovaps	%ymm0, 1152(%rsp)
	vmovaps	1184(%r12), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1216(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1248(%r12), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	1280(%r12), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1312(%r12), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1344(%r12), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1376(%r12), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	.p2align	4
.LBB34_273:
.Ltmp7363:
	.loc	12 551 14 is_stmt 1
	vmovups	(%rdi,%rax), %ymm4
.Ltmp7364:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm4, %ymm0
.Ltmp7365:
	.loc	29 48 14
	vaddps	%ymm7, %ymm0, %ymm0
.Ltmp7366:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm4, %ymm2
.Ltmp7367:
	.loc	29 48 14
	vaddps	%ymm7, %ymm2, %ymm2
.Ltmp7368:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm4, %ymm3
.Ltmp7369:
	.loc	29 48 14
	vaddps	%ymm7, %ymm3, %ymm3
	vmovaps	%ymm1, 448(%rsp)
.Ltmp7370:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm1, %ymm14
.Ltmp7371:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp7372:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm1, %ymm14
.Ltmp7373:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp7374:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm1, %ymm14
.Ltmp7375:
	.loc	29 48 14
	vaddps	%ymm3, %ymm14, %ymm14
.Ltmp7376:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm4, %ymm3
.Ltmp7377:
	.loc	29 48 14
	vaddps	%ymm7, %ymm3, %ymm3
.Ltmp7378:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm1, %ymm15
.Ltmp7379:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm15
	vmovaps	%ymm12, %ymm3
	vmovaps	%ymm5, %ymm1
	vmovaps	%ymm8, %ymm12
	vmovaps	32(%rsp), %ymm8
	vmovaps	%ymm9, %ymm5
.Ltmp7380:
	.loc	29 283 14
	vmulps	992(%rsp), %ymm9, %ymm9
.Ltmp7381:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm0
.Ltmp7382:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm5, %ymm9
.Ltmp7383:
	.loc	29 48 14
	vaddps	%ymm2, %ymm9, %ymm2
.Ltmp7384:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm5, %ymm9
.Ltmp7385:
	.loc	29 48 14
	vaddps	%ymm9, %ymm14, %ymm9
	vmovaps	%ymm5, 32(%rsp)
.Ltmp7386:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm5, %ymm14
.Ltmp7387:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp7388:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm8, %ymm15
.Ltmp7389:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7390:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm8, %ymm15
.Ltmp7391:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7392:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm8, %ymm15
.Ltmp7393:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7394:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm8, %ymm15
.Ltmp7395:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7396:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm12, %ymm15
.Ltmp7397:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7398:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm12, %ymm15
.Ltmp7399:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7400:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm12, %ymm15
.Ltmp7401:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7402:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm12, %ymm15
.Ltmp7403:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7404:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm1, %ymm15
.Ltmp7405:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7406:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm1, %ymm15
.Ltmp7407:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7408:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm1, %ymm15
.Ltmp7409:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7410:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm1, %ymm15
.Ltmp7411:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7412:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm13, %ymm15
.Ltmp7413:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7414:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm13, %ymm15
.Ltmp7415:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm2
.Ltmp7416:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm13, %ymm15
.Ltmp7417:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7418:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm13, %ymm15
.Ltmp7419:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7420:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm3, %ymm15
.Ltmp7421:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp7422:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm3, %ymm15
.Ltmp7423:
	.loc	29 48 14
	vaddps	%ymm2, %ymm15, %ymm15
.Ltmp7424:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm3, %ymm2
.Ltmp7425:
	.loc	29 48 14
	vaddps	%ymm2, %ymm9, %ymm9
.Ltmp7426:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm3, %ymm2
.Ltmp7427:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm14
	vmovaps	%ymm11, %ymm2
.Ltmp7428:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm11, %ymm11
.Ltmp7429:
	.loc	29 48 14
	vaddps	%ymm0, %ymm11, %ymm0
.Ltmp7430:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm2, %ymm11
.Ltmp7431:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
.Ltmp7432:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm2, %ymm15
.Ltmp7433:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7434:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm2, %ymm15
.Ltmp7435:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm15
	vmovaps	%ymm10, %ymm14
.Ltmp7436:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm10, %ymm10
.Ltmp7437:
	.loc	29 48 14
	vaddps	%ymm0, %ymm10, %ymm10
.Ltmp7438:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm14, %ymm0
.Ltmp7439:
	.loc	29 48 14
	vaddps	%ymm0, %ymm11, %ymm11
.Ltmp7440:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm14, %ymm0
.Ltmp7441:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp7442:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm14, %ymm0
.Ltmp7443:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	%ymm6, %ymm0
.Ltmp7444:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm6, %ymm6
.Ltmp7445:
	.loc	29 48 14
	vaddps	%ymm6, %ymm10, %ymm6
.Ltmp7446:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm0, %ymm10
.Ltmp7447:
	.loc	29 48 14
	vaddps	%ymm10, %ymm11, %ymm10
.Ltmp7448:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm0, %ymm11
.Ltmp7449:
	.loc	29 48 14
	vaddps	%ymm11, %ymm9, %ymm9
.Ltmp7450:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm0, %ymm11
.Ltmp7451:
	.loc	29 48 14
	vaddps	%ymm11, %ymm15, %ymm11
	vmovaps	384(%rsp), %ymm15
.Ltmp7452:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm15, %ymm5
.Ltmp7453:
	.loc	29 48 14
	vaddps	%ymm5, %ymm6, %ymm5
.Ltmp7454:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm15, %ymm6
.Ltmp7455:
	.loc	29 48 14
	vaddps	%ymm6, %ymm10, %ymm6
.Ltmp7456:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm15, %ymm10
.Ltmp7457:
	.loc	29 48 14
	vaddps	%ymm10, %ymm9, %ymm9
.Ltmp7458:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm15, %ymm10
.Ltmp7459:
	.loc	29 48 14
	vaddps	%ymm10, %ymm11, %ymm10
.Ltmp7460:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm11
.Ltmp7461:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm5, %ymm11, %ymm5
.Ltmp7462:
	.loc	29 82 19
	vandps	%ymm11, %ymm13, %ymm7
.Ltmp7463:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm5, %ymm7, %ymm5
	vxorps	%xmm7, %xmm7, %xmm7
.Ltmp7464:
	.loc	29 82 19
	vandps	%ymm6, %ymm11, %ymm6
.Ltmp7465:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm5, %ymm5
.Ltmp7466:
	.loc	29 82 19
	vandps	%ymm11, %ymm9, %ymm6
.Ltmp7467:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm5, %ymm5
.Ltmp7468:
	.loc	29 82 19
	vandps	%ymm11, %ymm10, %ymm6
.Ltmp7469:
	.loc	29 233 14
	vmaxps	%ymm6, %ymm5, %ymm5
.Ltmp7470:
	.loc	12 551 14
	vmovups	%ymm5, 9184(%rsp,%rax)
	vmovaps	%ymm12, %ymm5
	vmovaps	%ymm13, %ymm12
	vmovaps	%ymm1, %ymm13
	vmovaps	%ymm4, %ymm1
	vmovaps	448(%rsp), %ymm4
.Ltmp7471:
	.loc	11 304 12
	addq	$32, %rax
	vmovaps	%ymm4, %ymm9
	vmovaps	%ymm0, 384(%rsp)
	vmovaps	%ymm14, %ymm6
	vmovaps	%ymm2, %ymm10
	vmovaps	%ymm3, %ymm11
	cmpq	%rax, %rcx
	jne	.LBB34_273
.Ltmp7472:
.LBB34_274:
	.loc	1 1764 5
	vmovaps	%ymm1, 8448(%rsp)
	vmovaps	%ymm4, 8480(%rsp)
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, 8512(%rsp)
	vmovaps	%ymm8, 8544(%rsp)
	vmovaps	%ymm5, 8576(%rsp)
	vmovaps	%ymm13, 8608(%rsp)
	vmovaps	%ymm12, 8640(%rsp)
	vmovaps	%ymm3, 8672(%rsp)
	vmovaps	%ymm2, 8704(%rsp)
	vmovaps	%ymm14, 8736(%rsp)
	vmovaps	%ymm0, 8768(%rsp)
	vmovaps	%ymm15, 8800(%rsp)
.Ltmp7473:
	.loc	11 304 12
	cmpq	%r11, %rdx
.Ltmp7474:
	.loc	3 900 12
	je	.LBB34_262
.Ltmp7475:
	.loc	1 870 9
	vmovaps	8096(%rsp), %ymm7
.Ltmp7476:
	.loc	1 870 9 is_stmt 0
	vmovaps	8224(%rsp), %ymm8
	vmovaps	8384(%rsp), %ymm9
	vmovaps	8416(%rsp), %ymm10
.Ltmp7477:
	.loc	1 870 9
	vmovaps	8832(%rsp), %ymm11
.Ltmp7478:
	.loc	1 870 9
	vmovaps	8960(%rsp), %ymm13
	vmovaps	9120(%rsp), %ymm12
	vmovaps	9152(%rsp), %ymm14
	xorl	%r13d, %r13d
	vmovaps	1632(%rsp), %ymm6
	movq	%rdi, 2432(%rsp)
	movq	%r15, 2592(%rsp)
	movq	%r11, 2656(%rsp)
.Ltmp7479:
	.loc	1 0 9
.Ltmp7480:
	.p2align	4
.LBB34_276:
	.loc	1 1832 24 is_stmt 1
	leaq	(%r11,%r13), %rcx
	shlq	$3, %rcx
.Ltmp7481:
	.loc	5 568 12
	movq	%rbx, %rdx
	movq	%rcx, 96(%rsp)
	subq	%rcx, %rdx
	jb	.LBB34_389
.Ltmp7482:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp7483:
	.loc	1 1392 25
	movq	1688(%r12), %rsi
.Ltmp7484:
	.loc	1 1388 17
	movq	1840(%r12), %r9
	movq	%r9, 544(%rsp)
.Ltmp7485:
	.loc	1 1392 45
	imulq	224(%rsp), %r9
.Ltmp7486:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r9, %rdx
	jb	.LBB34_306
.Ltmp7487:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp7488:
	.loc	5 0 16 is_stmt 0
	movq	%r14, 192(%rsp)
	movq	%r13, 256(%rsp)
	movq	%r13, %rax
	shlq	$5, %rax
	vmovups	10216(%rsp,%rax), %ymm0
.Ltmp7489:
	vmovups	9184(%rsp,%rax), %ymm15
.Ltmp7490:
	vmaxps	%ymm0, %ymm15, %ymm3
	vmovaps	2240(%rsp), %ymm1
.Ltmp7491:
	vblendvps	%ymm1, %ymm3, %ymm0, %ymm0
.Ltmp7492:
	vdivps	%ymm0, %ymm7, %ymm1
.Ltmp7493:
	movq	1624(%r12), %rbx
.Ltmp7494:
	vcmpgt_oqps	%ymm7, %ymm0, %ymm0
	vbroadcastss	.LCPI34_2(%rip), %ymm2
	vblendvps	%ymm0, %ymm1, %ymm2, %ymm0
.Ltmp7495:
	.loc	1 1392 25 is_stmt 1
	movq	1680(%r12), %rax
	movq	%r9, 128(%rsp)
.Ltmp7496:
	.loc	12 551 14
	vmovups	%ymm0, (%rax,%r9,4)
.Ltmp7497:
	.loc	1 1259 17
	movq	1840(%r12), %rdx
.Ltmp7498:
	.loc	13 37 12
	testq	%rdx, %rdx
	je	.LBB34_294
.Ltmp7499:
	.loc	13 0 12 is_stmt 0
	movq	440(%rsp), %rcx
	movq	1832(%rcx), %rax
	movq	%rax, 384(%rsp)
	movq	224(%rsp), %rax
	leaq	1(%rax), %r8
	cmpq	%rbx, %r8
	movq	%rbx, %rax
	movl	$0, %esi
	cmovbq	%rsi, %rax
	movq	1824(%rcx), %rdi
	subq	%rax, %r8
	movq	1680(%rcx), %rax
	movq	1688(%rcx), %rsi
	movq	1768(%rcx), %r9
	movq	%r9, 448(%rsp)
	movq	1760(%rcx), %r13
	movq	1736(%rcx), %r9
	movq	%r9, 32(%rsp)
	movq	1728(%rcx), %r15
	imulq	%rdx, %r8
	movq	%r8, 480(%rsp)
	movq	%rdx, %r12
	xorl	%r11d, %r11d
	xorl	%ecx, %ecx
	jmp	.LBB34_282
	.p2align	4
.LBB34_305:
	xorl	%r9d, %r9d
.LBB34_293:
	decq	%r12
	addq	$4, %rcx
.Ltmp7500:
	movl	%r9d, (%r13,%r11,4)
.Ltmp7501:
	incq	%r11
.Ltmp7502:
	.loc	13 37 12 is_stmt 1
	testq	%r12, %r12
	je	.LBB34_294
.LBB34_282:
.Ltmp7503:
	.loc	7 1714 9
	cmpq	$32, %rcx
.Ltmp7504:
	.loc	6 180 28
	je	.LBB34_294
.Ltmp7505:
	.loc	1 1263 21
	cmpq	384(%rsp), %r11
	je	.LBB34_119
	leaq	(%r11,%r11,2), %r9
	movl	4(%rdi,%r9,4), %r8d
.Ltmp7506:
	.loc	1 1265 23
	addq	224(%rsp), %r8
.Ltmp7507:
	.loc	1 1266 12
	cmpq	%rbx, %r8
	movl	$0, %r10d
	cmovaeq	%rbx, %r10
	subq	%r10, %r8
.Ltmp7508:
	.loc	1 1273 42
	movq	%r8, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_122
.Ltmp7509:
	.loc	1 1274 24 is_stmt 1
	cmpq	448(%rsp), %r11
	je	.LBB34_125
.Ltmp7510:
	.loc	1 0 0 is_stmt 0
	movl	(%rdi,%r9,4), %r10d
.Ltmp7511:
	vmovss	(%rax,%r14,4), %xmm0
.Ltmp7512:
	.loc	1 1274 24
	movl	(%r13,%r11,4), %r9d
	testq	%r9, %r9
.Ltmp7513:
	.loc	1 1275 26 is_stmt 1
	je	.LBB34_289
	.loc	1 1278 24
	cmpq	32(%rsp), %r11
	jae	.LBB34_128
	vmovss	(%r15,%r11,4), %xmm1
.Ltmp7514:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB34_289
.Ltmp7515:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB34_289:
.Ltmp7516:
	.loc	1 1280 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB34_132
	vmovss	%xmm0, (%r15,%r11,4)
	.loc	1 1281 24
	incq	%r9
	cmpq	%r10, %r9
.Ltmp7517:
	.loc	1 1282 23
	jne	.LBB34_291
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 576(%rsp,%rcx)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rax,%r14,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp7518:
	.p2align	4
.LBB34_303:
.Ltmp7519:
	.loc	1 1291 65 is_stmt 1
	movq	%r8, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_147
.Ltmp7520:
	.loc	1 903 8 is_stmt 1
	vminss	(%rax,%r14,4), %xmm0, %xmm0
.Ltmp7521:
	.loc	1 1292 17
	vmovss	%xmm0, (%rax,%r14,4)
	.loc	1 1293 20
	testq	%r8, %r8
	cmoveq	%rbx, %r8
	.loc	1 1296 17
	decq	%r8
.Ltmp7522:
	.loc	10 1916 50
	decq	%r10
.Ltmp7523:
	.loc	3 900 12
	jne	.LBB34_303
	jmp	.LBB34_305
.Ltmp7524:
	.loc	3 0 12 is_stmt 0
.Ltmp7525:
	.p2align	4
.LBB34_291:
	movq	480(%rsp), %r8
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%r8), %r14
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_116
	vmovss	(%rax,%r14,4), %xmm1
.Ltmp7526:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp7527:
	.loc	1 1282 9
	vmovss	%xmm0, 576(%rsp,%rcx)
	jmp	.LBB34_293
.Ltmp7528:
	.loc	1 0 9 is_stmt 0
.Ltmp7529:
	.p2align	4
.LBB34_294:
	.loc	12 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm0
	movq	440(%rsp), %r12
.Ltmp7530:
	.loc	1 1412 26
	movq	1704(%r12), %rsi
	vmovaps	%ymm0, %ymm5
	movq	544(%rsp), %rdi
.Ltmp7531:
	.loc	13 37 12
	testq	%rdi, %rdi
	movq	192(%rsp), %r14
	movq	128(%rsp), %r11
	je	.LBB34_309
.Ltmp7532:
	.loc	13 0 12 is_stmt 0
	movq	1832(%r12), %r9
.Ltmp7533:
	.loc	1 1403 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB34_301
	.loc	1 0 42 is_stmt 0
	movq	1824(%r12), %r10
	.loc	1 1403 42
	movl	8(%r10), %r8d
	.loc	1 1403 28
	addq	224(%rsp), %r8
.Ltmp7534:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %r8
	movl	$0, %eax
	cmovaeq	%rbx, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%rdi, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	1696(%r12), %rdx
	.loc	1 1407 25
	vmovss	(%rdx,%r8,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 576(%rsp)
.Ltmp7535:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB34_308
.Ltmp7536:
	.loc	1 1403 42
	cmpq	$1, %r9
	je	.LBB34_299
	movl	20(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7537:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	vmovss	4(%rdx,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 580(%rsp)
.Ltmp7538:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB34_308
.Ltmp7539:
	.loc	1 1403 42
	cmpq	$2, %r9
	je	.LBB34_366
	movl	32(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7540:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	vmovss	8(%rdx,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 584(%rsp)
.Ltmp7541:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB34_308
.Ltmp7542:
	.loc	1 1403 42
	cmpq	$3, %r9
	je	.LBB34_370
	movl	44(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7543:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	vmovss	12(%rdx,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 588(%rsp)
.Ltmp7544:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB34_308
.Ltmp7545:
	.loc	1 1403 42
	cmpq	$4, %r9
	je	.LBB34_374
	movl	56(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7546:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	vmovss	16(%rdx,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 592(%rsp)
.Ltmp7547:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB34_308
.Ltmp7548:
	.loc	1 1403 42
	cmpq	$5, %r9
	je	.LBB34_378
	movl	68(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7549:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	vmovss	20(%rdx,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 596(%rsp)
.Ltmp7550:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB34_308
.Ltmp7551:
	.loc	1 1403 42
	cmpq	$6, %r9
	je	.LBB34_382
	movl	80(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7552:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	vmovss	24(%rdx,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 600(%rsp)
.Ltmp7553:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB34_308
.Ltmp7554:
	.loc	1 1403 42
	cmpq	$7, %r9
	je	.LBB34_386
	movl	92(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7555:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	vmovss	28(%rdx,%rax,4), %xmm1
	.loc	1 1407 13
	vmovss	%xmm1, 604(%rsp)
.Ltmp7556:
	.loc	1 0 13
.Ltmp7557:
	.p2align	4
.LBB34_308:
	.loc	12 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm5
.Ltmp7558:
.LBB34_309:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r11, %rdx
	jb	.LBB34_631
.Ltmp7559:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp7560:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm0
	vbroadcastss	.LCPI34_4(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp7561:
	vaddps	%ymm0, %ymm9, %ymm1
	vsubps	%ymm5, %ymm1, %ymm9
	.loc	1 1412 26 is_stmt 1
	movq	1696(%r12), %rax
.Ltmp7562:
	.loc	12 551 14
	vmovups	%ymm0, (%rax,%r11,4)
.Ltmp7563:
	.loc	29 360 14
	vdivps	%ymm10, %ymm9, %ymm0
.Ltmp7564:
	.loc	1 1416 43
	vmovaps	8352(%rsp), %ymm1
.Ltmp7565:
	.loc	29 347 14
	vbroadcastss	.LCPI34_2(%rip), %ymm2
	vsubps	%ymm0, %ymm2, %ymm0
.Ltmp7566:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm1, %ymm0, %ymm2
.Ltmp7567:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm2, %ymm8, %ymm2
.Ltmp7568:
	.loc	29 48 14
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp7569:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp7570:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm4
	vandps	%ymm4, %ymm0, %ymm1
.Ltmp7571:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp7572:
	.loc	29 82 19
	vandnps	%ymm0, %ymm1, %ymm0
.Ltmp7573:
	.loc	1 1417 5
	vmovaps	%ymm0, 8352(%rsp)
.Ltmp7574:
	.loc	1 1420 28
	movq	1672(%r12), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	%r14, %rdi
.Ltmp7575:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_397
.Ltmp7576:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp7577:
	.loc	5 0 16 is_stmt 0
	movq	536(%rsp), %rax
	movq	96(%rsp), %r8
	leaq	(%rax,%r8,4), %rax
.Ltmp7578:
	vbroadcastss	.LCPI34_2(%rip), %ymm1
	vsubps	%ymm0, %ymm1, %ymm0
.Ltmp7579:
	.loc	1 1420 28 is_stmt 1
	movq	1664(%r12), %rcx
.Ltmp7580:
	.loc	12 551 14
	vmovups	(%rcx,%rdi,4), %ymm1
.Ltmp7581:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rax), %ymm2
.Ltmp7582:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp7583:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm1, %ymm0, %ymm0
.Ltmp7584:
	.loc	12 551 14
	vmovups	%ymm2, (%rcx,%rdi,4)
.Ltmp7585:
	.loc	12 551 14 is_stmt 0
	vmovups	%ymm0, (%rax)
	movq	88(%rsp), %rsi
.Ltmp7586:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%r8, %rdx
	jb	.LBB34_398
.Ltmp7587:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp7588:
	.loc	1 1392 25
	movq	1888(%r12), %rsi
.Ltmp7589:
	.loc	1 1388 17
	movq	2040(%r12), %rcx
.Ltmp7590:
	.loc	1 1392 45
	movq	%rcx, %r9
	imulq	224(%rsp), %r9
.Ltmp7591:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r9, %rdx
	jb	.LBB34_306
.Ltmp7592:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp7593:
	.loc	5 0 16 is_stmt 0
	vmovaps	2240(%rsp), %ymm0
	vblendvps	%ymm0, %ymm3, %ymm15, %ymm0
.Ltmp7594:
	vdivps	%ymm0, %ymm11, %ymm1
	vcmpgt_oqps	%ymm11, %ymm0, %ymm0
	vbroadcastss	.LCPI34_2(%rip), %ymm2
	vblendvps	%ymm0, %ymm1, %ymm2, %ymm0
.Ltmp7595:
	.loc	1 1392 25 is_stmt 1
	movq	1880(%r12), %rax
.Ltmp7596:
	.loc	12 551 14
	vmovups	%ymm0, (%rax,%r9,4)
.Ltmp7597:
	.loc	1 1259 17
	movq	2040(%r12), %rdx
.Ltmp7598:
	.loc	13 37 12
	testq	%rdx, %rdx
	movq	%rcx, 544(%rsp)
	je	.LBB34_332
.Ltmp7599:
	.loc	13 0 12 is_stmt 0
	movq	%r9, 128(%rsp)
	movq	2032(%r12), %rax
	movq	%rax, 384(%rsp)
	movq	224(%rsp), %rax
	leaq	1(%rax), %r8
	cmpq	%rbx, %r8
	movq	%rbx, %rax
	movl	$0, %ecx
	cmovbq	%rcx, %rax
	movq	2024(%r12), %rdi
	subq	%rax, %r8
	movq	1880(%r12), %rax
	movq	1888(%r12), %rsi
	movq	1968(%r12), %rcx
	movq	%rcx, 448(%rsp)
	movq	1960(%r12), %r13
	movq	1936(%r12), %rcx
	movq	%rcx, 32(%rsp)
	movq	1928(%r12), %r15
	imulq	%rdx, %r8
	movq	%r8, 480(%rsp)
	movq	%rdx, %r12
	xorl	%r11d, %r11d
	xorl	%ecx, %ecx
	jmp	.LBB34_319
	.p2align	4
.LBB34_395:
	xorl	%r9d, %r9d
.LBB34_330:
	decq	%r12
	addq	$4, %rcx
.Ltmp7600:
	movl	%r9d, (%r13,%r11,4)
.Ltmp7601:
	incq	%r11
.Ltmp7602:
	.loc	13 37 12 is_stmt 1
	testq	%r12, %r12
	je	.LBB34_331
.LBB34_319:
.Ltmp7603:
	.loc	7 1714 9
	cmpq	$32, %rcx
.Ltmp7604:
	.loc	6 180 28
	je	.LBB34_331
.Ltmp7605:
	.loc	1 1263 21
	cmpq	384(%rsp), %r11
	je	.LBB34_119
	leaq	(%r11,%r11,2), %r9
	movl	4(%rdi,%r9,4), %r8d
.Ltmp7606:
	.loc	1 1265 23
	addq	224(%rsp), %r8
.Ltmp7607:
	.loc	1 1266 12
	cmpq	%rbx, %r8
	movl	$0, %r10d
	cmovaeq	%rbx, %r10
	subq	%r10, %r8
.Ltmp7608:
	.loc	1 1273 42
	movq	%r8, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_122
.Ltmp7609:
	.loc	1 1274 24 is_stmt 1
	cmpq	448(%rsp), %r11
	je	.LBB34_125
.Ltmp7610:
	.loc	1 0 0 is_stmt 0
	movl	(%rdi,%r9,4), %r10d
.Ltmp7611:
	vmovss	(%rax,%r14,4), %xmm0
.Ltmp7612:
	.loc	1 1274 24
	movl	(%r13,%r11,4), %r9d
	testq	%r9, %r9
	je	.LBB34_326
.Ltmp7613:
	.loc	1 1278 24 is_stmt 1
	cmpq	32(%rsp), %r11
	jae	.LBB34_128
	vmovss	(%r15,%r11,4), %xmm1
.Ltmp7614:
	.loc	1 903 8
	vucomiss	%xmm1, %xmm0
	jbe	.LBB34_326
.Ltmp7615:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm1, %xmm0
.LBB34_326:
.Ltmp7616:
	.loc	1 1280 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB34_132
	vmovss	%xmm0, (%r15,%r11,4)
	.loc	1 1281 24
	incq	%r9
	cmpq	%r10, %r9
.Ltmp7617:
	.loc	1 1282 23
	jne	.LBB34_328
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm0, 576(%rsp,%rcx)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rax,%r14,4), %xmm0
	.loc	1 0 30 is_stmt 0
.Ltmp7618:
	.p2align	4
.LBB34_393:
.Ltmp7619:
	.loc	1 1291 65 is_stmt 1
	movq	%r8, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_147
.Ltmp7620:
	.loc	1 903 8 is_stmt 1
	vminss	(%rax,%r14,4), %xmm0, %xmm0
.Ltmp7621:
	.loc	1 1292 17
	vmovss	%xmm0, (%rax,%r14,4)
	.loc	1 1293 20
	testq	%r8, %r8
	cmoveq	%rbx, %r8
	.loc	1 1296 17
	decq	%r8
.Ltmp7622:
	.loc	10 1916 50
	decq	%r10
.Ltmp7623:
	.loc	3 900 12
	jne	.LBB34_393
	jmp	.LBB34_395
.Ltmp7624:
	.loc	3 0 12 is_stmt 0
.Ltmp7625:
	.p2align	4
.LBB34_328:
	movq	480(%rsp), %r8
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%r8), %r14
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_116
	vmovss	(%rax,%r14,4), %xmm1
.Ltmp7626:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm0, %xmm1, %xmm0
.Ltmp7627:
	.loc	1 1282 9
	vmovss	%xmm0, 576(%rsp,%rcx)
	jmp	.LBB34_330
.Ltmp7628:
	.loc	1 0 9 is_stmt 0
.Ltmp7629:
	.p2align	4
.LBB34_331:
	.loc	12 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm5
	movq	440(%rsp), %r12
	movq	192(%rsp), %r14
	movq	544(%rsp), %rcx
	movq	128(%rsp), %r9
.Ltmp7630:
.LBB34_332:
	.loc	1 1412 26
	movq	1904(%r12), %rsi
	vmovaps	%ymm5, %ymm0
.Ltmp7631:
	.loc	13 37 12
	testq	%rcx, %rcx
	movq	2432(%rsp), %rdi
	movq	2592(%rsp), %r15
	movq	2656(%rsp), %r11
	movq	256(%rsp), %r13
	je	.LBB34_358
.Ltmp7632:
	.loc	13 0 12 is_stmt 0
	movq	%r9, %rdx
	movq	2032(%r12), %r9
.Ltmp7633:
	.loc	1 1403 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB34_301
	.loc	1 0 42 is_stmt 0
	movq	2024(%r12), %r10
	.loc	1 1403 42
	movl	8(%r10), %r8d
	.loc	1 1403 28
	addq	224(%rsp), %r8
.Ltmp7634:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %r8
	movl	$0, %eax
	cmovaeq	%rbx, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%rcx, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	1896(%r12), %rax
	.loc	1 1407 25
	vmovss	(%rax,%r8,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 576(%rsp)
.Ltmp7635:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %rcx
	je	.LBB34_357
.Ltmp7636:
	.loc	13 0 12 is_stmt 0
	movq	%rax, 32(%rsp)
.Ltmp7637:
	.loc	1 1403 42 is_stmt 1
	cmpq	$1, %r9
	je	.LBB34_299
	movl	20(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7638:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	movq	544(%rsp), %rcx
	.loc	1 1407 40
	imulq	%rcx, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	32(%rsp), %r8
	.loc	1 1407 25
	vmovss	4(%r8,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 580(%rsp)
.Ltmp7639:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %rcx
	je	.LBB34_357
.Ltmp7640:
	.loc	1 1403 42
	cmpq	$2, %r9
	je	.LBB34_366
	movl	32(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7641:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	movq	544(%rsp), %rcx
	.loc	1 1407 40
	imulq	%rcx, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	32(%rsp), %r8
	.loc	1 1407 25
	vmovss	8(%r8,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 584(%rsp)
.Ltmp7642:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %rcx
	je	.LBB34_357
.Ltmp7643:
	.loc	1 1403 42
	cmpq	$3, %r9
	je	.LBB34_370
	movl	44(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7644:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	movq	544(%rsp), %rcx
	.loc	1 1407 40
	imulq	%rcx, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	32(%rsp), %r8
	.loc	1 1407 25
	vmovss	12(%r8,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 588(%rsp)
.Ltmp7645:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %rcx
	je	.LBB34_357
.Ltmp7646:
	.loc	1 1403 42
	cmpq	$4, %r9
	je	.LBB34_374
	movl	56(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7647:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	movq	544(%rsp), %rcx
	.loc	1 1407 40
	imulq	%rcx, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	32(%rsp), %r8
	.loc	1 1407 25
	vmovss	16(%r8,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 592(%rsp)
.Ltmp7648:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %rcx
	je	.LBB34_357
.Ltmp7649:
	.loc	1 1403 42
	cmpq	$5, %r9
	je	.LBB34_378
	movl	68(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7650:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	movq	544(%rsp), %rcx
	.loc	1 1407 40
	imulq	%rcx, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	32(%rsp), %r8
	.loc	1 1407 25
	vmovss	20(%r8,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 596(%rsp)
.Ltmp7651:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %rcx
	je	.LBB34_357
.Ltmp7652:
	.loc	1 1403 42
	cmpq	$6, %r9
	je	.LBB34_382
	movl	80(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7653:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	movq	544(%rsp), %rcx
	.loc	1 1407 40
	imulq	%rcx, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	32(%rsp), %r8
	.loc	1 1407 25
	vmovss	24(%r8,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 600(%rsp)
.Ltmp7654:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %rcx
	je	.LBB34_357
.Ltmp7655:
	.loc	1 1403 42
	cmpq	$7, %r9
	je	.LBB34_386
	movl	92(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp7656:
	.loc	1 1404 16 is_stmt 1
	cmpq	%rbx, %rax
	movl	$0, %ecx
	cmovaeq	%rbx, %rcx
	subq	%rcx, %rax
	movq	544(%rsp), %rcx
	.loc	1 1407 40
	imulq	%rcx, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_189
	.loc	1 0 25
	movq	32(%rsp), %r8
	.loc	1 1407 25
	vmovss	28(%r8,%rax,4), %xmm0
	.loc	1 1407 13
	vmovss	%xmm0, 604(%rsp)
.Ltmp7657:
	.loc	1 0 13
.Ltmp7658:
	.p2align	4
.LBB34_357:
	.loc	12 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm0
	movq	%rdx, %r9
.Ltmp7659:
.LBB34_358:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r9, %rdx
	jb	.LBB34_632
.Ltmp7660:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp7661:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm1
	vmulps	%ymm1, %ymm5, %ymm1
	vroundps	$9, %ymm1, %ymm1
	vbroadcastss	.LCPI34_4(%rip), %ymm2
	vmulps	%ymm2, %ymm1, %ymm1
.Ltmp7662:
	vaddps	%ymm1, %ymm12, %ymm2
	vsubps	%ymm0, %ymm2, %ymm12
	.loc	1 1412 26 is_stmt 1
	movq	1896(%r12), %rax
.Ltmp7663:
	.loc	12 551 14
	vmovups	%ymm1, (%rax,%r9,4)
.Ltmp7664:
	.loc	1 1416 43
	vmovaps	9088(%rsp), %ymm0
.Ltmp7665:
	.loc	29 360 14
	vdivps	%ymm14, %ymm12, %ymm1
.Ltmp7666:
	.loc	29 347 14
	vbroadcastss	.LCPI34_2(%rip), %ymm2
	vsubps	%ymm1, %ymm2, %ymm1
.Ltmp7667:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm0, %ymm1, %ymm2
.Ltmp7668:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm2, %ymm13, %ymm2
.Ltmp7669:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp7670:
	.loc	29 233 14
	vmaxps	%ymm0, %ymm1, %ymm0
.Ltmp7671:
	.loc	29 82 19
	vandps	%ymm4, %ymm0, %ymm1
.Ltmp7672:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp7673:
	.loc	29 82 19
	vandnps	%ymm0, %ymm1, %ymm0
.Ltmp7674:
	.loc	1 1417 5
	vmovaps	%ymm0, 9088(%rsp)
.Ltmp7675:
	.loc	1 1420 28
	movq	1872(%r12), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	%r14, %rcx
.Ltmp7676:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rcx, %rdx
	jb	.LBB34_396
.Ltmp7677:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp7678:
	.loc	1 0 0 is_stmt 0
	incq	%r13
	movq	432(%rsp), %rax
	movq	%rcx, %rdx
	movq	96(%rsp), %rcx
.Ltmp7679:
	leaq	(%rax,%rcx,4), %rax
.Ltmp7680:
	vbroadcastss	.LCPI34_2(%rip), %ymm1
	vsubps	%ymm0, %ymm1, %ymm0
.Ltmp7681:
	.loc	1 1420 28 is_stmt 1
	movq	1864(%r12), %rcx
.Ltmp7682:
	.loc	12 551 14
	vmovups	(%rcx,%rdx,4), %ymm1
.Ltmp7683:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rax), %ymm2
.Ltmp7684:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp7685:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm1, %ymm0, %ymm0
.Ltmp7686:
	.loc	12 551 14
	vmovups	%ymm2, (%rcx,%rdx,4)
.Ltmp7687:
	.loc	1 1878 13
	incq	%r14
	.loc	1 1879 16
	cmpq	1632(%r12), %r14
.Ltmp7688:
	.loc	12 551 14
	vmovups	%ymm0, (%rax)
	movl	$0, %edx
.Ltmp7689:
	.loc	1 1879 16
	cmoveq	%rdx, %r14
	movq	224(%rsp), %rax
	.loc	1 1882 13
	incq	%rax
	.loc	1 1883 16
	cmpq	%rbx, %rax
	movl	$0, %ecx
	movq	%rcx, 768(%rsp)
	cmoveq	%rdx, %rax
	movq	%rax, 224(%rsp)
.Ltmp7690:
	.loc	10 1916 50
	cmpq	2560(%rsp), %r13
	movq	184(%rsp), %rbx
.Ltmp7691:
	.loc	3 900 12
	jne	.LBB34_276
.Ltmp7692:
	.loc	1 1411 0
	vmovaps	%ymm9, 8384(%rsp)
.Ltmp7693:
	.loc	1 1411 0 is_stmt 0
	vmovaps	%ymm12, 9120(%rsp)
	movq	312(%rsp), %rdx
	vxorps	%xmm7, %xmm7, %xmm7
	jmp	.LBB34_262
.Ltmp7694:
.LBB34_71:
	.loc	1 0 0
	leaq	2688(%rsp), %rdi
.Ltmp7695:
	.loc	1 1794 24 is_stmt 1
	leaq	1648(%r12), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	3424(%rsp), %rdi
.Ltmp7696:
	.loc	1 1795 25
	leaq	1848(%r12), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp7697:
	.loc	1 1800 19
	movzbl	1536(%r12), %ebx
.Ltmp7698:
	.loc	1 1801 21
	movzbl	1537(%r12), %eax
	movb	%al, 32(%rsp)
.Ltmp7699:
	.loc	1 1802 27
	movl	1640(%r12), %r14d
.Ltmp7700:
	.loc	1 1803 27
	movl	1644(%r12), %eax
	movq	%rax, 224(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 576(%rsp)
	leaq	10216(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%r15
	leaq	9184(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
.Ltmp7701:
	.loc	1 0 0 is_stmt 0
	testq	%r13, %r13
.Ltmp7702:
	.loc	8 446 20 is_stmt 1
	je	.LBB34_72
.Ltmp7703:
	.loc	8 0 20 is_stmt 0
	movq	%r13, 424(%rsp)
.Ltmp7704:
	.loc	29 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp7705:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 1536(%rsp)
	vmovaps	%ymm1, 992(%rsp)
	testb	%bl, %bl
	jne	.LBB34_77
.Ltmp7706:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 992(%rsp)
.LBB34_77:
	cmpb	$0, 32(%rsp)
	movq	312(%rsp), %rdx
	jne	.LBB34_79
	vmovaps	%ymm0, 1536(%rsp)
.LBB34_79:
	movq	$0, 704(%rsp)
	vxorps	%xmm2, %xmm2, %xmm2
	movq	432(%rsp), %r10
	movq	536(%rsp), %r8
	movq	%rdx, %rax
	xorl	%r9d, %r9d
	movq	184(%rsp), %rbx
.Ltmp7707:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB34_82
.Ltmp7708:
	.loc	8 0 20 is_stmt 0
.Ltmp7709:
	.p2align	4
.LBB34_80:
	vmovaps	320(%rsp), %ymm0
.Ltmp7710:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp7711:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp7712:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp7713:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp7714:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp7715:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp7716:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp7717:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
	movq	312(%rsp), %rdx
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp7718:
.LBB34_81:
	addq	$32, %r9
	movq	424(%rsp), %rax
	decq	%rax
	movq	1632(%rsp), %rcx
.Ltmp7719:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	addq	$1024, %r8
	addq	$1024, %r10
	movq	%rax, 424(%rsp)
	testq	%rax, %rax
	movq	%rcx, %rax
	je	.LBB34_73
.LBB34_82:
.Ltmp7720:
	.loc	4 2584 13
	cmpq	$1, %rax
	movq	%rax, 1632(%rsp)
	movq	%rax, %rcx
	adcq	$0, %rcx
	cmpq	$32, %rcx
	movl	$32, %eax
	cmovaeq	%rax, %rcx
	movq	%rcx, 2432(%rsp)
.Ltmp7721:
	.loc	1 1815 51
	movq	%rdx, %rsi
	subq	%r9, %rsi
.Ltmp7722:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%rax, %rsi
.Ltmp7723:
	.loc	1 1816 27
	leaq	(,%r9,8), %rax
.Ltmp7724:
	.loc	1 1820 35
	addq	%r9, %rsi
	shlq	$3, %rsi
.Ltmp7725:
	.loc	4 1050 16
	cmpq	%rax, %rsi
	jb	.LBB34_623
	cmpq	%rbx, %rsi
	ja	.LBB34_623
.Ltmp7726:
	.loc	4 0 16 is_stmt 0
	movq	2432(%rsp), %rcx
	shll	$5, %ecx
.Ltmp7727:
	.loc	1 1759 23 is_stmt 1
	vmovaps	2688(%rsp), %ymm6
	vmovaps	2720(%rsp), %ymm14
	vmovaps	2752(%rsp), %ymm1
	vmovaps	2784(%rsp), %ymm5
	vmovaps	2816(%rsp), %ymm4
	vmovaps	2848(%rsp), %ymm13
	vmovaps	2880(%rsp), %ymm10
	vmovaps	2912(%rsp), %ymm0
	vmovaps	2944(%rsp), %ymm11
	vmovaps	2976(%rsp), %ymm15
	vmovaps	3008(%rsp), %ymm8
.Ltmp7728:
	.loc	11 304 12
	cmpq	%r9, %rdx
	vmovaps	%ymm1, 32(%rsp)
	jne	.LBB34_86
.Ltmp7729:
	.loc	1 0 0 is_stmt 0
	vmovaps	3040(%rsp), %ymm7
	vmovaps	%ymm14, %ymm1
.Ltmp7730:
	.loc	11 304 12
	jmp	.LBB34_88
.Ltmp7731:
	.loc	11 0 12
.Ltmp7732:
	.p2align	4
.LBB34_86:
	vmovaps	(%r12), %ymm3
	vmovaps	%ymm3, 480(%rsp)
	vmovaps	32(%r12), %ymm3
	vmovaps	%ymm3, 96(%rsp)
	vmovaps	64(%r12), %ymm3
	vmovaps	%ymm3, 192(%rsp)
	vmovaps	96(%r12), %ymm3
	vmovaps	%ymm3, 544(%rsp)
	vmovaps	128(%r12), %ymm3
	vmovaps	%ymm3, 2240(%rsp)
	vmovaps	160(%r12), %ymm3
	vmovaps	%ymm3, 128(%rsp)
	vmovaps	192(%r12), %ymm3
	vmovaps	%ymm3, 256(%rsp)
	vmovaps	224(%r12), %ymm3
	vmovaps	%ymm3, 352(%rsp)
	vmovaps	256(%r12), %ymm3
	vmovaps	%ymm3, 320(%rsp)
	vmovaps	288(%r12), %ymm3
	vmovaps	%ymm3, 928(%rsp)
	vmovaps	320(%r12), %ymm3
	vmovaps	%ymm3, 960(%rsp)
	xorl	%edx, %edx
	vmovaps	%ymm8, 384(%rsp)
	vmovaps	%ymm15, %ymm12
	vmovaps	%ymm11, %ymm9
	vmovaps	%ymm0, %ymm3
	vmovaps	352(%r12), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	384(%r12), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	416(%r12), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	448(%r12), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	vmovaps	480(%r12), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	512(%r12), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	544(%r12), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	576(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	608(%r12), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	640(%r12), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	672(%r12), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	704(%r12), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	736(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	768(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	800(%r12), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	832(%r12), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	864(%r12), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	896(%r12), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	928(%r12), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	960(%r12), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	992(%r12), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1024(%r12), %ymm0
	vmovaps	%ymm0, 1184(%rsp)
	vmovaps	1056(%r12), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	1088(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1120(%r12), %ymm0
	vmovaps	%ymm0, 1152(%rsp)
	vmovaps	1152(%r12), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1184(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1216(%r12), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	1248(%r12), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1280(%r12), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1312(%r12), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1344(%r12), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	1376(%r12), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	.p2align	4
.LBB34_87:
.Ltmp7733:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r8,%rdx), %ymm1
.Ltmp7734:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm1, %ymm0
.Ltmp7735:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp7736:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm1, %ymm7
.Ltmp7737:
	.loc	29 48 14
	vaddps	%ymm2, %ymm7, %ymm7
.Ltmp7738:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm1, %ymm8
.Ltmp7739:
	.loc	29 48 14
	vaddps	%ymm2, %ymm8, %ymm8
	vmovaps	%ymm6, 448(%rsp)
.Ltmp7740:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm6, %ymm11
.Ltmp7741:
	.loc	29 48 14
	vaddps	%ymm0, %ymm11, %ymm11
.Ltmp7742:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm6, %ymm0
.Ltmp7743:
	.loc	29 48 14
	vaddps	%ymm0, %ymm7, %ymm7
.Ltmp7744:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm6, %ymm0
.Ltmp7745:
	.loc	29 48 14
	vaddps	%ymm0, %ymm8, %ymm8
.Ltmp7746:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm1, %ymm0
.Ltmp7747:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp7748:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm6, %ymm15
.Ltmp7749:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	%ymm10, %ymm0
	vmovaps	%ymm4, %ymm6
	vmovaps	%ymm5, %ymm10
	vmovaps	32(%rsp), %ymm5
	vmovaps	%ymm14, %ymm2
.Ltmp7750:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm14, %ymm14
.Ltmp7751:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm11
.Ltmp7752:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm2, %ymm14
.Ltmp7753:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp7754:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm2, %ymm14
.Ltmp7755:
	.loc	29 48 14
	vaddps	%ymm14, %ymm8, %ymm8
	vmovaps	%ymm2, 32(%rsp)
.Ltmp7756:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm2, %ymm14
.Ltmp7757:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp7758:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm5, %ymm15
.Ltmp7759:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp7760:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm5, %ymm15
.Ltmp7761:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7762:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm5, %ymm15
.Ltmp7763:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7764:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm5, %ymm15
.Ltmp7765:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7766:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm10, %ymm15
.Ltmp7767:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp7768:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm10, %ymm15
.Ltmp7769:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7770:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm10, %ymm15
.Ltmp7771:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7772:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm10, %ymm15
.Ltmp7773:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7774:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm4, %ymm15
.Ltmp7775:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp7776:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm4, %ymm15
.Ltmp7777:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7778:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm4, %ymm15
.Ltmp7779:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7780:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm4, %ymm15
.Ltmp7781:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7782:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm13, %ymm15
.Ltmp7783:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp7784:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm13, %ymm15
.Ltmp7785:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7786:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm13, %ymm15
.Ltmp7787:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7788:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm13, %ymm15
.Ltmp7789:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp7790:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm0, %ymm15
.Ltmp7791:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm15
.Ltmp7792:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm0, %ymm11
.Ltmp7793:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm7
.Ltmp7794:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm0, %ymm11
.Ltmp7795:
	.loc	29 48 14
	vaddps	%ymm11, %ymm8, %ymm8
.Ltmp7796:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm0, %ymm11
.Ltmp7797:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm14
	vmovaps	%ymm3, %ymm11
.Ltmp7798:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm3, %ymm3
.Ltmp7799:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7800:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm11, %ymm15
.Ltmp7801:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7802:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm11, %ymm15
.Ltmp7803:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7804:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm11, %ymm15
.Ltmp7805:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
	vmovaps	%ymm9, %ymm15
.Ltmp7806:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm9, %ymm9
.Ltmp7807:
	.loc	29 48 14
	vaddps	%ymm3, %ymm9, %ymm3
.Ltmp7808:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm15, %ymm9
.Ltmp7809:
	.loc	29 48 14
	vaddps	%ymm7, %ymm9, %ymm7
.Ltmp7810:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm15, %ymm9
.Ltmp7811:
	.loc	29 48 14
	vaddps	%ymm9, %ymm8, %ymm9
.Ltmp7812:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm15, %ymm8
.Ltmp7813:
	.loc	29 48 14
	vaddps	%ymm8, %ymm14, %ymm14
	vmovaps	%ymm12, %ymm8
.Ltmp7814:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm12, %ymm12
.Ltmp7815:
	.loc	29 48 14
	vaddps	%ymm3, %ymm12, %ymm3
.Ltmp7816:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm8, %ymm12
.Ltmp7817:
	.loc	29 48 14
	vaddps	%ymm7, %ymm12, %ymm12
.Ltmp7818:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm8, %ymm7
.Ltmp7819:
	.loc	29 48 14
	vaddps	%ymm7, %ymm9, %ymm9
.Ltmp7820:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm8, %ymm7
.Ltmp7821:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm14
	vmovaps	384(%rsp), %ymm7
.Ltmp7822:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm7, %ymm2
.Ltmp7823:
	.loc	29 48 14
	vaddps	%ymm2, %ymm3, %ymm2
.Ltmp7824:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm7, %ymm3
.Ltmp7825:
	.loc	29 48 14
	vaddps	%ymm3, %ymm12, %ymm3
.Ltmp7826:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm7, %ymm12
.Ltmp7827:
	.loc	29 48 14
	vaddps	%ymm12, %ymm9, %ymm9
.Ltmp7828:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm7, %ymm12
.Ltmp7829:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp7830:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm14
.Ltmp7831:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm2, %ymm14, %ymm2
.Ltmp7832:
	.loc	29 82 19
	vandps	%ymm14, %ymm13, %ymm4
.Ltmp7833:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm2, %ymm4, %ymm2
	vmovaps	%ymm10, %ymm4
	vmovaps	%ymm13, %ymm10
	vmovaps	%ymm6, %ymm13
	vmovaps	%ymm1, %ymm6
	vmovaps	448(%rsp), %ymm1
.Ltmp7834:
	.loc	29 82 19
	vandps	%ymm3, %ymm14, %ymm3
.Ltmp7835:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7836:
	.loc	29 82 19
	vandps	%ymm14, %ymm9, %ymm3
.Ltmp7837:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7838:
	.loc	29 82 19
	vandps	%ymm14, %ymm12, %ymm3
.Ltmp7839:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7840:
	.loc	12 551 14
	vmovups	%ymm2, 10216(%rsp,%rdx)
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp7841:
	.loc	11 304 12
	addq	$32, %rdx
	vmovaps	%ymm1, %ymm14
	vmovaps	%ymm8, 384(%rsp)
	vmovaps	%ymm15, %ymm12
	vmovaps	%ymm11, %ymm9
	vmovaps	%ymm0, %ymm3
	cmpq	%rdx, %rcx
	jne	.LBB34_87
.Ltmp7842:
.LBB34_88:
	.loc	1 1764 5
	vmovaps	%ymm6, 2688(%rsp)
	vmovaps	%ymm1, 2720(%rsp)
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, 2752(%rsp)
	vmovaps	%ymm5, 2784(%rsp)
	vmovaps	%ymm4, 2816(%rsp)
	vmovaps	%ymm13, 2848(%rsp)
	vmovaps	%ymm10, 2880(%rsp)
	vmovaps	%ymm0, 2912(%rsp)
	vmovaps	%ymm11, 2944(%rsp)
	vmovaps	%ymm15, 2976(%rsp)
	vmovaps	%ymm8, 3008(%rsp)
	vmovaps	%ymm7, 3040(%rsp)
.Ltmp7843:
	.loc	5 438 16
	cmpq	88(%rsp), %rsi
	ja	.LBB34_624
.Ltmp7844:
	.loc	1 1759 23
	vmovaps	3424(%rsp), %ymm6
	vmovaps	3456(%rsp), %ymm13
	vmovaps	3488(%rsp), %ymm1
	vmovaps	3520(%rsp), %ymm5
	vmovaps	3552(%rsp), %ymm4
	vmovaps	3584(%rsp), %ymm14
	vmovaps	3616(%rsp), %ymm10
	vmovaps	3648(%rsp), %ymm0
	vmovaps	3680(%rsp), %ymm9
	vmovaps	3712(%rsp), %ymm15
	vmovaps	3744(%rsp), %ymm8
	movq	312(%rsp), %rdx
.Ltmp7845:
	.loc	11 304 12
	cmpq	%r9, %rdx
	vmovaps	%ymm1, 32(%rsp)
.Ltmp7846:
	.loc	11 304 12 is_stmt 0
	jne	.LBB34_91
.Ltmp7847:
	.loc	1 0 0
	vmovaps	3776(%rsp), %ymm7
	vmovaps	%ymm13, %ymm1
.Ltmp7848:
	.loc	11 304 12
	jmp	.LBB34_93
.Ltmp7849:
	.loc	11 0 12
.Ltmp7850:
	.p2align	4
.LBB34_91:
	vmovaps	(%r12), %ymm3
	vmovaps	%ymm3, 480(%rsp)
	vmovaps	32(%r12), %ymm3
	vmovaps	%ymm3, 96(%rsp)
	vmovaps	64(%r12), %ymm3
	vmovaps	%ymm3, 192(%rsp)
	vmovaps	96(%r12), %ymm3
	vmovaps	%ymm3, 544(%rsp)
	vmovaps	128(%r12), %ymm3
	vmovaps	%ymm3, 2240(%rsp)
	vmovaps	160(%r12), %ymm3
	vmovaps	%ymm3, 128(%rsp)
	vmovaps	192(%r12), %ymm3
	vmovaps	%ymm3, 256(%rsp)
	vmovaps	224(%r12), %ymm3
	vmovaps	%ymm3, 352(%rsp)
	vmovaps	256(%r12), %ymm3
	vmovaps	%ymm3, 320(%rsp)
	vmovaps	288(%r12), %ymm3
	vmovaps	%ymm3, 928(%rsp)
	vmovaps	320(%r12), %ymm3
	vmovaps	%ymm3, 960(%rsp)
	xorl	%eax, %eax
	vmovaps	%ymm8, 384(%rsp)
	vmovaps	%ymm15, %ymm11
	vmovaps	%ymm9, %ymm12
	vmovaps	%ymm0, %ymm3
	vmovaps	352(%r12), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	384(%r12), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	416(%r12), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	448(%r12), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	vmovaps	480(%r12), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	512(%r12), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	544(%r12), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	576(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	608(%r12), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	640(%r12), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	672(%r12), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	704(%r12), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	736(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	768(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	800(%r12), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	832(%r12), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	864(%r12), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	896(%r12), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	928(%r12), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	960(%r12), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	992(%r12), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1024(%r12), %ymm0
	vmovaps	%ymm0, 1184(%rsp)
	vmovaps	1056(%r12), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	1088(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1120(%r12), %ymm0
	vmovaps	%ymm0, 1152(%rsp)
	vmovaps	1152(%r12), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1184(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1216(%r12), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	1248(%r12), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1280(%r12), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1312(%r12), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1344(%r12), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	1376(%r12), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	.p2align	4
.LBB34_92:
.Ltmp7851:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r10,%rax), %ymm1
.Ltmp7852:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm1, %ymm0
.Ltmp7853:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp7854:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm1, %ymm7
.Ltmp7855:
	.loc	29 48 14
	vaddps	%ymm2, %ymm7, %ymm7
.Ltmp7856:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm1, %ymm8
.Ltmp7857:
	.loc	29 48 14
	vaddps	%ymm2, %ymm8, %ymm8
	vmovaps	%ymm6, 448(%rsp)
.Ltmp7858:
	.loc	29 283 14
	vmulps	2240(%rsp), %ymm6, %ymm9
.Ltmp7859:
	.loc	29 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp7860:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm6, %ymm0
.Ltmp7861:
	.loc	29 48 14
	vaddps	%ymm0, %ymm7, %ymm7
.Ltmp7862:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm6, %ymm0
.Ltmp7863:
	.loc	29 48 14
	vaddps	%ymm0, %ymm8, %ymm8
.Ltmp7864:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm1, %ymm0
.Ltmp7865:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp7866:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm6, %ymm15
.Ltmp7867:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	%ymm10, %ymm0
	vmovaps	%ymm4, %ymm6
	vmovaps	%ymm5, %ymm10
	vmovaps	32(%rsp), %ymm5
	vmovaps	%ymm13, %ymm2
.Ltmp7868:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm13, %ymm13
.Ltmp7869:
	.loc	29 48 14
	vaddps	%ymm13, %ymm9, %ymm9
.Ltmp7870:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm2, %ymm13
.Ltmp7871:
	.loc	29 48 14
	vaddps	%ymm7, %ymm13, %ymm7
.Ltmp7872:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm2, %ymm13
.Ltmp7873:
	.loc	29 48 14
	vaddps	%ymm13, %ymm8, %ymm8
	vmovaps	%ymm2, 32(%rsp)
.Ltmp7874:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm2, %ymm13
.Ltmp7875:
	.loc	29 48 14
	vaddps	%ymm13, %ymm15, %ymm13
.Ltmp7876:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm5, %ymm15
.Ltmp7877:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7878:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm5, %ymm15
.Ltmp7879:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7880:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm5, %ymm15
.Ltmp7881:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7882:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm5, %ymm15
.Ltmp7883:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp7884:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm10, %ymm15
.Ltmp7885:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7886:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm10, %ymm15
.Ltmp7887:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7888:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm10, %ymm15
.Ltmp7889:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7890:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm10, %ymm15
.Ltmp7891:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp7892:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm4, %ymm15
.Ltmp7893:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7894:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm4, %ymm15
.Ltmp7895:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7896:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm4, %ymm15
.Ltmp7897:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7898:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm4, %ymm15
.Ltmp7899:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp7900:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm14, %ymm15
.Ltmp7901:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp7902:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm14, %ymm15
.Ltmp7903:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7904:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm14, %ymm15
.Ltmp7905:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7906:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm14, %ymm15
.Ltmp7907:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp7908:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm0, %ymm15
.Ltmp7909:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm15
.Ltmp7910:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm0, %ymm9
.Ltmp7911:
	.loc	29 48 14
	vaddps	%ymm7, %ymm9, %ymm7
.Ltmp7912:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm0, %ymm9
.Ltmp7913:
	.loc	29 48 14
	vaddps	%ymm9, %ymm8, %ymm8
.Ltmp7914:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm0, %ymm9
.Ltmp7915:
	.loc	29 48 14
	vaddps	%ymm9, %ymm13, %ymm13
	vmovaps	%ymm3, %ymm9
.Ltmp7916:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm3, %ymm3
.Ltmp7917:
	.loc	29 48 14
	vaddps	%ymm3, %ymm15, %ymm3
.Ltmp7918:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm9, %ymm15
.Ltmp7919:
	.loc	29 48 14
	vaddps	%ymm7, %ymm15, %ymm7
.Ltmp7920:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm9, %ymm15
.Ltmp7921:
	.loc	29 48 14
	vaddps	%ymm15, %ymm8, %ymm8
.Ltmp7922:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm9, %ymm15
.Ltmp7923:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
	vmovaps	%ymm12, %ymm15
.Ltmp7924:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm12, %ymm12
.Ltmp7925:
	.loc	29 48 14
	vaddps	%ymm3, %ymm12, %ymm3
.Ltmp7926:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm15, %ymm12
.Ltmp7927:
	.loc	29 48 14
	vaddps	%ymm7, %ymm12, %ymm7
.Ltmp7928:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm15, %ymm12
.Ltmp7929:
	.loc	29 48 14
	vaddps	%ymm12, %ymm8, %ymm12
.Ltmp7930:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm15, %ymm8
.Ltmp7931:
	.loc	29 48 14
	vaddps	%ymm8, %ymm13, %ymm13
	vmovaps	%ymm11, %ymm8
.Ltmp7932:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm11, %ymm11
.Ltmp7933:
	.loc	29 48 14
	vaddps	%ymm3, %ymm11, %ymm3
.Ltmp7934:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm8, %ymm11
.Ltmp7935:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm11
.Ltmp7936:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm8, %ymm7
.Ltmp7937:
	.loc	29 48 14
	vaddps	%ymm7, %ymm12, %ymm12
.Ltmp7938:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm8, %ymm7
.Ltmp7939:
	.loc	29 48 14
	vaddps	%ymm7, %ymm13, %ymm13
	vmovaps	384(%rsp), %ymm7
.Ltmp7940:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm7, %ymm2
.Ltmp7941:
	.loc	29 48 14
	vaddps	%ymm2, %ymm3, %ymm2
.Ltmp7942:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm7, %ymm3
.Ltmp7943:
	.loc	29 48 14
	vaddps	%ymm3, %ymm11, %ymm3
.Ltmp7944:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm7, %ymm11
.Ltmp7945:
	.loc	29 48 14
	vaddps	%ymm11, %ymm12, %ymm11
.Ltmp7946:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm7, %ymm12
.Ltmp7947:
	.loc	29 48 14
	vaddps	%ymm12, %ymm13, %ymm12
.Ltmp7948:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm13
.Ltmp7949:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm2, %ymm13, %ymm2
.Ltmp7950:
	.loc	29 82 19
	vandps	%ymm13, %ymm14, %ymm4
.Ltmp7951:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm2, %ymm4, %ymm2
	vmovaps	%ymm10, %ymm4
	vmovaps	%ymm14, %ymm10
	vmovaps	%ymm6, %ymm14
	vmovaps	%ymm1, %ymm6
	vmovaps	448(%rsp), %ymm1
.Ltmp7952:
	.loc	29 82 19
	vandps	%ymm3, %ymm13, %ymm3
.Ltmp7953:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7954:
	.loc	29 82 19
	vandps	%ymm13, %ymm11, %ymm3
.Ltmp7955:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7956:
	.loc	29 82 19
	vandps	%ymm13, %ymm12, %ymm3
.Ltmp7957:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm2
.Ltmp7958:
	.loc	12 551 14
	vmovups	%ymm2, 9184(%rsp,%rax)
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp7959:
	.loc	11 304 12
	addq	$32, %rax
	vmovaps	%ymm1, %ymm13
	vmovaps	%ymm8, 384(%rsp)
	vmovaps	%ymm15, %ymm11
	vmovaps	%ymm9, %ymm12
	vmovaps	%ymm0, %ymm3
	cmpq	%rax, %rcx
	jne	.LBB34_92
.Ltmp7960:
.LBB34_93:
	.loc	1 1764 5
	vmovaps	%ymm6, 3424(%rsp)
	vmovaps	%ymm1, 3456(%rsp)
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, 3488(%rsp)
	vmovaps	%ymm5, 3520(%rsp)
	vmovaps	%ymm4, 3552(%rsp)
	vmovaps	%ymm14, 3584(%rsp)
	vmovaps	%ymm10, 3616(%rsp)
	vmovaps	%ymm0, 3648(%rsp)
	vmovaps	%ymm9, 3680(%rsp)
	vmovaps	%ymm15, 3712(%rsp)
	vmovaps	%ymm8, 3744(%rsp)
	vmovaps	%ymm7, 3776(%rsp)
.Ltmp7961:
	.loc	11 304 12
	cmpq	%r9, %rdx
.Ltmp7962:
	.loc	3 900 12
	je	.LBB34_81
.Ltmp7963:
	.loc	1 853 44
	vmovaps	3072(%rsp), %ymm6
	.loc	1 853 73 is_stmt 0
	vmovaps	3104(%rsp), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	.loc	1 853 61
	vmovaps	3136(%rsp), %ymm7
.Ltmp7964:
	.loc	1 851 26 is_stmt 1
	vmovaps	3168(%rsp), %ymm0
	vmovaps	%ymm0, 320(%rsp)
.Ltmp7965:
	.loc	1 851 26 is_stmt 0
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 352(%rsp)
.Ltmp7966:
	.loc	1 853 44 is_stmt 1
	vmovaps	3200(%rsp), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	.loc	1 853 61 is_stmt 0
	vmovaps	3264(%rsp), %ymm14
	.loc	1 853 73
	vmovaps	3232(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
.Ltmp7967:
	.loc	1 853 44
	vmovaps	3808(%rsp), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	.loc	1 853 73
	vmovaps	3840(%rsp), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	.loc	1 853 61
	vmovaps	3872(%rsp), %ymm11
.Ltmp7968:
	.loc	1 851 26 is_stmt 1
	vmovaps	3904(%rsp), %ymm0
	vmovaps	%ymm0, 256(%rsp)
.Ltmp7969:
	.loc	1 851 26 is_stmt 0
	vmovaps	4032(%rsp), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	3936(%rsp), %ymm15
	vmovaps	4000(%rsp), %ymm8
	vmovaps	3360(%rsp), %ymm1
	xorl	%r11d, %r11d
	vmovaps	4096(%rsp), %ymm0
	vbroadcastss	.LCPI34_2(%rip), %ymm12
	movq	%r10, 2592(%rsp)
	movq	%r8, 2656(%rsp)
	movq	%r9, 768(%rsp)
.Ltmp7970:
	.loc	1 0 26
.Ltmp7971:
	.p2align	4
.LBB34_95:
	vbroadcastss	.LCPI34_1(%rip), %ymm5
.Ltmp7972:
	.loc	29 347 14 is_stmt 1
	vaddps	320(%rsp), %ymm5, %ymm2
	vxorps	%xmm10, %xmm10, %xmm10
.Ltmp7973:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm2, %ymm2
	vmovaps	%ymm2, 320(%rsp)
.Ltmp7974:
	.loc	29 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm3
.Ltmp7975:
	.loc	29 48 14
	vaddps	%ymm7, %ymm6, %ymm4
	vmovaps	1280(%rsp), %ymm2
.Ltmp7976:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm4, %ymm2, %ymm6
.Ltmp7977:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm7, %ymm10, %ymm7
.Ltmp7978:
	.loc	29 347 14 is_stmt 1
	vaddps	352(%rsp), %ymm5, %ymm3
.Ltmp7979:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm3, %ymm2
	vmovaps	%ymm2, 352(%rsp)
.Ltmp7980:
	.loc	29 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm3
.Ltmp7981:
	.loc	29 48 14
	vaddps	96(%rsp), %ymm14, %ymm4
	vmovaps	1248(%rsp), %ymm2
.Ltmp7982:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm4, %ymm2, %ymm2
	vmovaps	%ymm2, 96(%rsp)
.Ltmp7983:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm14, %ymm10, %ymm14
.Ltmp7984:
	.loc	29 347 14 is_stmt 1
	vaddps	256(%rsp), %ymm5, %ymm3
.Ltmp7985:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm3, %ymm2
	vmovaps	%ymm2, 256(%rsp)
.Ltmp7986:
	.loc	29 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm3
.Ltmp7987:
	.loc	29 48 14
	vaddps	192(%rsp), %ymm11, %ymm4
	vmovaps	1216(%rsp), %ymm2
.Ltmp7988:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm4, %ymm2, %ymm2
	vmovaps	%ymm2, 192(%rsp)
.Ltmp7989:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm11, %ymm10, %ymm11
.Ltmp7990:
	.loc	29 347 14 is_stmt 1
	vaddps	128(%rsp), %ymm5, %ymm3
.Ltmp7991:
	.loc	29 233 14
	vmaxps	%ymm10, %ymm3, %ymm2
	vmovaps	%ymm2, 128(%rsp)
.Ltmp7992:
	.loc	29 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm3
.Ltmp7993:
	.loc	29 48 14
	vaddps	%ymm8, %ymm15, %ymm4
.Ltmp7994:
	.loc	1 853 73
	vmovaps	3968(%rsp), %ymm9
.Ltmp7995:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm4, %ymm9, %ymm2
.Ltmp7996:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm8, %ymm10, %ymm8
.Ltmp7997:
	.loc	1 1832 24 is_stmt 1
	leaq	(%r11,%r9), %rcx
	shlq	$3, %rcx
.Ltmp7998:
	.loc	5 568 12
	movq	%rbx, %rdx
	movq	%rcx, 544(%rsp)
	subq	%rcx, %rdx
	jb	.LBB34_190
.Ltmp7999:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_97
.Ltmp8000:
	.loc	1 1392 25
	movq	1688(%r12), %rsi
.Ltmp8001:
	.loc	1 1388 17
	movq	1840(%r12), %rax
.Ltmp8002:
	.loc	1 1392 45
	movq	%rax, %rdi
	imulq	224(%rsp), %rdi
.Ltmp8003:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_150
.Ltmp8004:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_97
.Ltmp8005:
	.loc	5 0 16 is_stmt 0
	movq	%rax, 928(%rsp)
	movq	%r14, 2240(%rsp)
	movq	%r11, 1312(%rsp)
	movq	%r11, %rax
	shlq	$5, %rax
	vmovups	10216(%rsp,%rax), %ymm3
.Ltmp8006:
	vmovups	9184(%rsp,%rax), %ymm4
.Ltmp8007:
	vmaxps	%ymm3, %ymm4, %ymm10
	vmovaps	992(%rsp), %ymm5
.Ltmp8008:
	vblendvps	%ymm5, %ymm10, %ymm3, %ymm3
.Ltmp8009:
	vdivps	%ymm3, %ymm6, %ymm9
.Ltmp8010:
	movq	1624(%r12), %r13
.Ltmp8011:
	vcmpgt_oqps	%ymm6, %ymm3, %ymm3
	vblendvps	%ymm3, %ymm9, %ymm12, %ymm3
.Ltmp8012:
	.loc	1 1392 25 is_stmt 1
	movq	1680(%r12), %rax
	movq	%rdi, 960(%rsp)
.Ltmp8013:
	.loc	12 551 14
	vmovups	%ymm3, (%rax,%rdi,4)
.Ltmp8014:
	.loc	1 1259 17
	movq	1840(%r12), %rdx
.Ltmp8015:
	.loc	13 37 12
	testq	%rdx, %rdx
	je	.LBB34_135
.Ltmp8016:
	.loc	13 0 12 is_stmt 0
	movq	440(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 384(%rsp)
	movq	224(%rsp), %rax
	leaq	1(%rax), %r9
	cmpq	%r13, %r9
	movq	%r13, %rax
	movl	$0, %ecx
	cmovbq	%rcx, %rax
	movq	1824(%rdi), %rcx
	subq	%rax, %r9
	movq	1680(%rdi), %rax
	movq	1688(%rdi), %rsi
	movq	1768(%rdi), %r8
	movq	%r8, 448(%rsp)
	movq	1760(%rdi), %r15
	movq	1736(%rdi), %r8
	movq	%r8, 32(%rsp)
	movq	1728(%rdi), %r12
	imulq	%rdx, %r9
	movq	%r9, 480(%rsp)
	movq	%rdx, %rbx
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB34_104
	.p2align	4
.LBB34_149:
	xorl	%r8d, %r8d
.LBB34_134:
	decq	%rbx
	addq	$4, %rdi
.Ltmp8017:
	movl	%r8d, (%r15,%r11,4)
.Ltmp8018:
	incq	%r11
.Ltmp8019:
	.loc	13 37 12 is_stmt 1
	testq	%rbx, %rbx
	je	.LBB34_135
.LBB34_104:
.Ltmp8020:
	.loc	7 1714 9
	cmpq	$32, %rdi
.Ltmp8021:
	.loc	6 180 28
	je	.LBB34_135
.Ltmp8022:
	.loc	1 1263 21
	cmpq	384(%rsp), %r11
	je	.LBB34_117
	leaq	(%r11,%r11,2), %r8
	movl	4(%rcx,%r8,4), %r9d
.Ltmp8023:
	.loc	1 1265 23
	addq	224(%rsp), %r9
.Ltmp8024:
	.loc	1 1266 12
	cmpq	%r13, %r9
	movl	$0, %r10d
	cmovaeq	%r13, %r10
	subq	%r10, %r9
.Ltmp8025:
	.loc	1 1273 42
	movq	%r9, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_120
.Ltmp8026:
	.loc	1 1274 24 is_stmt 1
	cmpq	448(%rsp), %r11
	je	.LBB34_123
.Ltmp8027:
	.loc	1 0 0 is_stmt 0
	movl	(%rcx,%r8,4), %r10d
.Ltmp8028:
	vmovss	(%rax,%r14,4), %xmm3
.Ltmp8029:
	.loc	1 1274 24
	movl	(%r15,%r11,4), %r8d
	testq	%r8, %r8
	je	.LBB34_111
.Ltmp8030:
	.loc	1 1278 24 is_stmt 1
	cmpq	32(%rsp), %r11
	jae	.LBB34_126
	vmovss	(%r12,%r11,4), %xmm9
.Ltmp8031:
	.loc	1 903 8
	vucomiss	%xmm9, %xmm3
	jbe	.LBB34_111
.Ltmp8032:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm9, %xmm3
.LBB34_111:
.Ltmp8033:
	.loc	1 1280 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB34_130
	vmovss	%xmm3, (%r12,%r11,4)
	.loc	1 1281 24
	incq	%r8
	cmpq	%r10, %r8
.Ltmp8034:
	.loc	1 1282 23
	jne	.LBB34_113
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm3, 576(%rsp,%rdi)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rax,%r14,4), %xmm3
	.loc	1 0 30 is_stmt 0
.Ltmp8035:
	.p2align	4
.LBB34_144:
.Ltmp8036:
	.loc	1 1291 65 is_stmt 1
	movq	%r9, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_145
.Ltmp8037:
	.loc	1 903 8 is_stmt 1
	vminss	(%rax,%r14,4), %xmm3, %xmm3
.Ltmp8038:
	.loc	1 1292 17
	vmovss	%xmm3, (%rax,%r14,4)
	.loc	1 1293 20
	testq	%r9, %r9
	cmoveq	%r13, %r9
	.loc	1 1296 17
	decq	%r9
.Ltmp8039:
	.loc	10 1916 50
	decq	%r10
.Ltmp8040:
	.loc	3 900 12
	jne	.LBB34_144
	jmp	.LBB34_149
.Ltmp8041:
	.loc	3 0 12 is_stmt 0
.Ltmp8042:
	.p2align	4
.LBB34_113:
	movq	480(%rsp), %r9
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%r9), %r14
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_114
	vmovss	(%rax,%r14,4), %xmm9
.Ltmp8043:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm3, %xmm9, %xmm3
.Ltmp8044:
	.loc	1 1282 9
	vmovss	%xmm3, 576(%rsp,%rdi)
	jmp	.LBB34_134
.Ltmp8045:
	.loc	1 0 9 is_stmt 0
.Ltmp8046:
	.p2align	4
.LBB34_135:
	.loc	12 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm9
	movq	440(%rsp), %r12
.Ltmp8047:
	.loc	1 1412 26
	movq	1704(%r12), %rsi
	vmovaps	%ymm9, %ymm13
	movq	928(%rsp), %rdi
.Ltmp8048:
	.loc	13 37 12
	testq	%rdi, %rdi
	movq	536(%rsp), %r11
	movq	2240(%rsp), %r14
	movq	960(%rsp), %rbx
	je	.LBB34_152
.Ltmp8049:
	.loc	13 0 12 is_stmt 0
	movq	1832(%r12), %r9
.Ltmp8050:
	.loc	1 1403 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB34_142
	.loc	1 0 42 is_stmt 0
	movq	1824(%r12), %r10
	.loc	1 1403 42
	movl	8(%r10), %r8d
	.loc	1 1403 28
	addq	224(%rsp), %r8
.Ltmp8051:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %r8
	movl	$0, %eax
	cmovaeq	%r13, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%rdi, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_187
	.loc	1 0 25
	movq	1696(%r12), %rdx
	.loc	1 1407 25
	vmovss	(%rdx,%r8,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 576(%rsp)
.Ltmp8052:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB34_151
.Ltmp8053:
	.loc	1 1403 42
	cmpq	$1, %r9
	je	.LBB34_140
	movl	20(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8054:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_187
	vmovss	4(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 580(%rsp)
.Ltmp8055:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB34_151
.Ltmp8056:
	.loc	1 1403 42
	cmpq	$2, %r9
	je	.LBB34_164
	movl	32(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8057:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_187
	vmovss	8(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 584(%rsp)
.Ltmp8058:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB34_151
.Ltmp8059:
	.loc	1 1403 42
	cmpq	$3, %r9
	je	.LBB34_168
	movl	44(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8060:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_187
	vmovss	12(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 588(%rsp)
.Ltmp8061:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB34_151
.Ltmp8062:
	.loc	1 1403 42
	cmpq	$4, %r9
	je	.LBB34_172
	movl	56(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8063:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_187
	vmovss	16(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 592(%rsp)
.Ltmp8064:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB34_151
.Ltmp8065:
	.loc	1 1403 42
	cmpq	$5, %r9
	je	.LBB34_176
	movl	68(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8066:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_187
	vmovss	20(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 596(%rsp)
.Ltmp8067:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB34_151
.Ltmp8068:
	.loc	1 1403 42
	cmpq	$6, %r9
	je	.LBB34_180
	movl	80(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8069:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_187
	vmovss	24(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 600(%rsp)
.Ltmp8070:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB34_151
.Ltmp8071:
	.loc	1 1403 42
	cmpq	$7, %r9
	je	.LBB34_184
	movl	92(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8072:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_187
	vmovss	28(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 604(%rsp)
.Ltmp8073:
	.loc	1 0 13
.Ltmp8074:
	.p2align	4
.LBB34_151:
	.loc	12 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm13
.Ltmp8075:
.LBB34_152:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm3
	vmulps	%ymm3, %ymm9, %ymm3
	vroundps	$9, %ymm3, %ymm3
	vbroadcastss	.LCPI34_4(%rip), %ymm5
	vmulps	%ymm5, %ymm3, %ymm3
.Ltmp8076:
	.loc	29 48 14 is_stmt 1
	vaddps	%ymm1, %ymm3, %ymm1
.Ltmp8077:
	.loc	29 347 14
	vsubps	%ymm13, %ymm1, %ymm1
.Ltmp8078:
	.loc	1 1411 5
	vmovaps	%ymm1, 3360(%rsp)
.Ltmp8079:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%rbx, %rdx
	jb	.LBB34_625
.Ltmp8080:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_97
.Ltmp8081:
	.loc	1 1412 26
	movq	1696(%r12), %rax
.Ltmp8082:
	.loc	12 551 14
	vmovups	%ymm3, (%rax,%rbx,4)
.Ltmp8083:
	.loc	29 360 14
	vdivps	3392(%rsp), %ymm1, %ymm3
.Ltmp8084:
	.loc	1 1416 43
	vmovaps	3328(%rsp), %ymm9
.Ltmp8085:
	.loc	29 347 14
	vsubps	%ymm3, %ymm12, %ymm3
.Ltmp8086:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm9, %ymm3, %ymm15
.Ltmp8087:
	.loc	29 283 14 is_stmt 1
	vmulps	96(%rsp), %ymm15, %ymm15
.Ltmp8088:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp8089:
	.loc	29 233 14
	vmaxps	%ymm9, %ymm3, %ymm3
.Ltmp8090:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm9
	vandps	%ymm3, %ymm9, %ymm15
.Ltmp8091:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm5
	vcmplt_oqps	%ymm5, %ymm15, %ymm15
.Ltmp8092:
	.loc	29 82 19
	vandnps	%ymm3, %ymm15, %ymm3
.Ltmp8093:
	.loc	1 1417 5
	vmovaps	%ymm3, 3328(%rsp)
.Ltmp8094:
	.loc	1 1420 28
	movq	1672(%r12), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	%r14, %rdi
.Ltmp8095:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_159
.Ltmp8096:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_97
.Ltmp8097:
	.loc	5 0 16 is_stmt 0
	movq	544(%rsp), %r8
	leaq	(%r11,%r8,4), %rax
.Ltmp8098:
	vsubps	%ymm3, %ymm12, %ymm3
.Ltmp8099:
	.loc	1 1420 28 is_stmt 1
	movq	1664(%r12), %rcx
.Ltmp8100:
	.loc	12 551 14
	vmovups	(%rcx,%rdi,4), %ymm15
.Ltmp8101:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rax), %ymm5
.Ltmp8102:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm3, %ymm15, %ymm3
	vmovaps	1536(%rsp), %ymm12
.Ltmp8103:
	.loc	29 585 19
	vblendvps	%ymm12, %ymm15, %ymm3, %ymm3
.Ltmp8104:
	.loc	12 551 14
	vmovups	%ymm5, (%rcx,%rdi,4)
.Ltmp8105:
	.loc	12 551 14 is_stmt 0
	vmovups	%ymm3, (%rax)
	movq	88(%rsp), %rsi
.Ltmp8106:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%r8, %rdx
	jb	.LBB34_252
.Ltmp8107:
	.loc	5 438 16
	cmpq	$7, %rdx
	vmovaps	%ymm2, %ymm15
	jbe	.LBB34_158
.Ltmp8108:
	.loc	1 1392 25
	movq	1888(%r12), %rsi
.Ltmp8109:
	.loc	1 1388 17
	movq	2040(%r12), %rdi
.Ltmp8110:
	.loc	1 1392 45
	movq	%rdi, %r15
	imulq	224(%rsp), %r15
.Ltmp8111:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r15, %rdx
	jb	.LBB34_248
.Ltmp8112:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_158
.Ltmp8113:
	.loc	5 0 16 is_stmt 0
	vmovaps	992(%rsp), %ymm3
	vblendvps	%ymm3, %ymm10, %ymm4, %ymm3
	vmovaps	192(%rsp), %ymm5
.Ltmp8114:
	vdivps	%ymm3, %ymm5, %ymm4
	vcmpgt_oqps	%ymm5, %ymm3, %ymm3
	vbroadcastss	.LCPI34_2(%rip), %ymm12
	vblendvps	%ymm3, %ymm4, %ymm12, %ymm3
.Ltmp8115:
	.loc	1 1392 25 is_stmt 1
	movq	1880(%r12), %rax
.Ltmp8116:
	.loc	12 551 14
	vmovups	%ymm3, (%rax,%r15,4)
.Ltmp8117:
	.loc	1 1259 17
	movq	2040(%r12), %rdx
.Ltmp8118:
	.loc	13 37 12
	testq	%rdx, %rdx
	je	.LBB34_212
.Ltmp8119:
	.loc	13 0 12 is_stmt 0
	movq	%r15, 960(%rsp)
	movq	%rdi, 928(%rsp)
	movq	2032(%r12), %rax
	movq	%rax, 384(%rsp)
	movq	224(%rsp), %rax
	leaq	1(%rax), %r8
	cmpq	%r13, %r8
	movq	%r13, %rax
	movl	$0, %ecx
	cmovbq	%rcx, %rax
	movq	2024(%r12), %rcx
	subq	%rax, %r8
	movq	1880(%r12), %rax
	movq	1888(%r12), %rsi
	movq	1968(%r12), %rdi
	movq	%rdi, 448(%rsp)
	movq	1960(%r12), %r15
	movq	1936(%r12), %rdi
	movq	%rdi, 32(%rsp)
	movq	1928(%r12), %r12
	imulq	%rdx, %r8
	movq	%r8, 480(%rsp)
	movq	%rdx, %rbx
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB34_195
	.p2align	4
.LBB34_247:
	xorl	%r8d, %r8d
.LBB34_210:
	decq	%rbx
	addq	$4, %rdi
.Ltmp8120:
	movl	%r8d, (%r15,%r11,4)
.Ltmp8121:
	incq	%r11
.Ltmp8122:
	.loc	13 37 12 is_stmt 1
	testq	%rbx, %rbx
	je	.LBB34_211
.LBB34_195:
.Ltmp8123:
	.loc	7 1714 9
	cmpq	$32, %rdi
.Ltmp8124:
	.loc	6 180 28
	je	.LBB34_211
.Ltmp8125:
	.loc	1 1263 21
	cmpq	384(%rsp), %r11
	je	.LBB34_626
	leaq	(%r11,%r11,2), %r8
	movl	4(%rcx,%r8,4), %r9d
.Ltmp8126:
	.loc	1 1265 23
	addq	224(%rsp), %r9
.Ltmp8127:
	.loc	1 1266 12
	cmpq	%r13, %r9
	movl	$0, %r10d
	cmovaeq	%r13, %r10
	subq	%r10, %r9
.Ltmp8128:
	.loc	1 1273 42
	movq	%r9, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1273 22 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_627
.Ltmp8129:
	.loc	1 1274 24 is_stmt 1
	cmpq	448(%rsp), %r11
	je	.LBB34_628
.Ltmp8130:
	.loc	1 0 0 is_stmt 0
	movl	(%rcx,%r8,4), %r10d
.Ltmp8131:
	vmovss	(%rax,%r14,4), %xmm3
.Ltmp8132:
	.loc	1 1274 24
	movl	(%r15,%r11,4), %r8d
	testq	%r8, %r8
.Ltmp8133:
	.loc	1 1275 26 is_stmt 1
	je	.LBB34_202
	.loc	1 1278 24
	cmpq	32(%rsp), %r11
	jae	.LBB34_206
	vmovss	(%r12,%r11,4), %xmm4
.Ltmp8134:
	.loc	1 903 8
	vucomiss	%xmm4, %xmm3
	jbe	.LBB34_202
.Ltmp8135:
	.loc	1 0 8 is_stmt 0
	vmovaps	%xmm4, %xmm3
.LBB34_202:
.Ltmp8136:
	.loc	1 1280 9 is_stmt 1
	cmpq	32(%rsp), %r11
	je	.LBB34_208
	vmovss	%xmm3, (%r12,%r11,4)
	.loc	1 1281 24
	incq	%r8
	cmpq	%r10, %r8
.Ltmp8137:
	.loc	1 1282 23
	jne	.LBB34_204
	.loc	1 1282 9 is_stmt 0
	vmovss	%xmm3, 576(%rsp,%rdi)
	.loc	1 1288 30 is_stmt 1
	vmovss	(%rax,%r14,4), %xmm3
	.loc	1 0 30 is_stmt 0
.Ltmp8138:
	.p2align	4
.LBB34_244:
.Ltmp8139:
	.loc	1 1291 65 is_stmt 1
	movq	%r9, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	1 1291 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_245
.Ltmp8140:
	.loc	1 903 8 is_stmt 1
	vminss	(%rax,%r14,4), %xmm3, %xmm3
.Ltmp8141:
	.loc	1 1292 17
	vmovss	%xmm3, (%rax,%r14,4)
	.loc	1 1293 20
	testq	%r9, %r9
	cmoveq	%r13, %r9
	.loc	1 1296 17
	decq	%r9
.Ltmp8142:
	.loc	10 1916 50
	decq	%r10
.Ltmp8143:
	.loc	3 900 12
	jne	.LBB34_244
	jmp	.LBB34_247
.Ltmp8144:
	.loc	3 0 12 is_stmt 0
.Ltmp8145:
	.p2align	4
.LBB34_204:
	movq	480(%rsp), %r9
	.loc	1 1285 44 is_stmt 1
	leaq	(%r11,%r9), %r14
	.loc	1 1285 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB34_205
	vmovss	(%rax,%r14,4), %xmm4
.Ltmp8146:
	.loc	1 903 8 is_stmt 1
	vminss	%xmm3, %xmm4, %xmm3
.Ltmp8147:
	.loc	1 1282 9
	vmovss	%xmm3, 576(%rsp,%rdi)
	jmp	.LBB34_210
.Ltmp8148:
	.loc	1 0 9 is_stmt 0
.Ltmp8149:
	.p2align	4
.LBB34_211:
	.loc	12 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm13
	movq	440(%rsp), %r12
	movq	2240(%rsp), %r14
	movq	928(%rsp), %rdi
	movq	960(%rsp), %r15
.Ltmp8150:
.LBB34_212:
	.loc	1 1412 26
	movq	1904(%r12), %rsi
	vmovaps	%ymm13, %ymm4
.Ltmp8151:
	.loc	13 37 12
	testq	%rdi, %rdi
	movq	184(%rsp), %rbx
	movq	1312(%rsp), %r11
	je	.LBB34_238
.Ltmp8152:
	.loc	13 0 12 is_stmt 0
	movq	2032(%r12), %r9
.Ltmp8153:
	.loc	1 1403 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB34_142
	.loc	1 0 42 is_stmt 0
	movq	2024(%r12), %r10
	.loc	1 1403 42
	movl	8(%r10), %r8d
	.loc	1 1403 28
	addq	224(%rsp), %r8
.Ltmp8154:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %r8
	movl	$0, %eax
	cmovaeq	%r13, %rax
	subq	%rax, %r8
	.loc	1 1407 40
	imulq	%rdi, %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_251
	.loc	1 0 25
	movq	1896(%r12), %rdx
	.loc	1 1407 25
	vmovss	(%rdx,%r8,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 576(%rsp)
.Ltmp8155:
	.loc	13 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB34_237
.Ltmp8156:
	.loc	1 1403 42
	cmpq	$1, %r9
	je	.LBB34_140
	movl	20(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8157:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_251
	vmovss	4(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 580(%rsp)
.Ltmp8158:
	.loc	13 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB34_237
.Ltmp8159:
	.loc	1 1403 42
	cmpq	$2, %r9
	je	.LBB34_164
	movl	32(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8160:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_251
	vmovss	8(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 584(%rsp)
.Ltmp8161:
	.loc	13 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB34_237
.Ltmp8162:
	.loc	1 1403 42
	cmpq	$3, %r9
	je	.LBB34_168
	movl	44(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8163:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_251
	vmovss	12(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 588(%rsp)
.Ltmp8164:
	.loc	13 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB34_237
.Ltmp8165:
	.loc	1 1403 42
	cmpq	$4, %r9
	je	.LBB34_172
	movl	56(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8166:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_251
	vmovss	16(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 592(%rsp)
.Ltmp8167:
	.loc	13 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB34_237
.Ltmp8168:
	.loc	1 1403 42
	cmpq	$5, %r9
	je	.LBB34_176
	movl	68(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8169:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_251
	vmovss	20(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 596(%rsp)
.Ltmp8170:
	.loc	13 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB34_237
.Ltmp8171:
	.loc	1 1403 42
	cmpq	$6, %r9
	je	.LBB34_180
	movl	80(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8172:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_251
	vmovss	24(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 600(%rsp)
.Ltmp8173:
	.loc	13 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB34_237
.Ltmp8174:
	.loc	1 1403 42
	cmpq	$7, %r9
	je	.LBB34_184
	movl	92(%r10), %eax
	.loc	1 1403 28 is_stmt 0
	addq	224(%rsp), %rax
.Ltmp8175:
	.loc	1 1404 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	1 1407 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	1 1407 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB34_251
	vmovss	28(%rdx,%rax,4), %xmm3
	.loc	1 1407 13
	vmovss	%xmm3, 604(%rsp)
.Ltmp8176:
	.loc	1 0 13
.Ltmp8177:
	.p2align	4
.LBB34_237:
	.loc	12 551 14 is_stmt 1
	vmovaps	576(%rsp), %ymm4
.Ltmp8178:
.LBB34_238:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm3
	vmulps	%ymm3, %ymm13, %ymm3
	vroundps	$9, %ymm3, %ymm3
	vbroadcastss	.LCPI34_4(%rip), %ymm5
	vmulps	%ymm5, %ymm3, %ymm3
.Ltmp8179:
	.loc	29 48 14 is_stmt 1
	vaddps	%ymm0, %ymm3, %ymm0
.Ltmp8180:
	.loc	29 347 14
	vsubps	%ymm4, %ymm0, %ymm0
.Ltmp8181:
	.loc	1 1411 5
	vmovaps	%ymm0, 4096(%rsp)
.Ltmp8182:
	.loc	5 580 12
	movq	%rsi, %rdx
	subq	%r15, %rdx
	jb	.LBB34_629
.Ltmp8183:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_158
.Ltmp8184:
	.loc	1 1412 26
	movq	1896(%r12), %rax
.Ltmp8185:
	.loc	12 551 14
	vmovups	%ymm3, (%rax,%r15,4)
.Ltmp8186:
	.loc	29 360 14
	vdivps	4128(%rsp), %ymm0, %ymm3
.Ltmp8187:
	.loc	1 1416 43
	vmovaps	4064(%rsp), %ymm4
.Ltmp8188:
	.loc	29 347 14
	vsubps	%ymm3, %ymm12, %ymm3
.Ltmp8189:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm4, %ymm3, %ymm5
.Ltmp8190:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm5, %ymm15, %ymm5
.Ltmp8191:
	.loc	29 48 14
	vaddps	%ymm5, %ymm4, %ymm4
.Ltmp8192:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm3, %ymm3
.Ltmp8193:
	.loc	29 82 19
	vandps	%ymm3, %ymm9, %ymm4
.Ltmp8194:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm5
	vcmplt_oqps	%ymm5, %ymm4, %ymm4
.Ltmp8195:
	.loc	29 82 19
	vandnps	%ymm3, %ymm4, %ymm3
.Ltmp8196:
	.loc	1 1417 5
	vmovaps	%ymm3, 4064(%rsp)
.Ltmp8197:
	.loc	1 1420 28
	movq	1872(%r12), %rsi
	.loc	1 1420 44 is_stmt 0
	imulq	%r14, %rdi
.Ltmp8198:
	.loc	5 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB34_250
.Ltmp8199:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB34_158
.Ltmp8200:
	.loc	1 0 0 is_stmt 0
	incq	%r11
	movq	432(%rsp), %rax
	movq	544(%rsp), %rcx
.Ltmp8201:
	leaq	(%rax,%rcx,4), %rax
.Ltmp8202:
	vsubps	%ymm3, %ymm12, %ymm3
.Ltmp8203:
	.loc	1 1420 28 is_stmt 1
	movq	1864(%r12), %rcx
.Ltmp8204:
	.loc	12 551 14
	vmovups	(%rcx,%rdi,4), %ymm4
.Ltmp8205:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rax), %ymm5
.Ltmp8206:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm4, %ymm3, %ymm3
	vmovaps	1536(%rsp), %ymm9
.Ltmp8207:
	.loc	29 585 19
	vblendvps	%ymm9, %ymm4, %ymm3, %ymm3
.Ltmp8208:
	.loc	12 551 14
	vmovups	%ymm5, (%rcx,%rdi,4)
.Ltmp8209:
	.loc	1 1878 13
	incq	%r14
	.loc	1 1879 16
	cmpq	1632(%r12), %r14
.Ltmp8210:
	.loc	12 551 14
	vmovups	%ymm3, (%rax)
	movl	$0, %ecx
.Ltmp8211:
	.loc	1 1879 16
	cmoveq	%rcx, %r14
	movq	224(%rsp), %rdx
	.loc	1 1882 13
	incq	%rdx
	.loc	1 1883 16
	cmpq	%r13, %rdx
	movl	$0, %eax
	movq	%rax, 704(%rsp)
	cmoveq	%rcx, %rdx
	movq	%rdx, 224(%rsp)
.Ltmp8212:
	.loc	10 1916 50
	cmpq	2432(%rsp), %r11
	movq	2592(%rsp), %r10
	movq	2656(%rsp), %r8
	movq	768(%rsp), %r9
.Ltmp8213:
	.loc	3 900 12
	jne	.LBB34_95
	jmp	.LBB34_80
.Ltmp8214:
.LBB34_254:
	.loc	3 0 12 is_stmt 0
	movq	184(%rsp), %rbx
.LBB34_255:
	leaq	7712(%rsp), %rdi
	movq	760(%rsp), %rsi
.Ltmp8215:
	.loc	1 1889 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	8448(%rsp), %rdi
	jmp	.LBB34_74
.Ltmp8216:
.LBB34_72:
	.loc	1 0 14 is_stmt 0
	movq	184(%rsp), %rbx
.LBB34_73:
	leaq	2688(%rsp), %rdi
	movq	760(%rsp), %rsi
.Ltmp8217:
	.loc	1 1889 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	3424(%rsp), %rdi
.Ltmp8218:
.LBB34_74:
	.loc	1 0 14 is_stmt 0
	movq	752(%rsp), %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movl	%r14d, 1640(%r12)
	movq	224(%rsp), %rax
	movl	%eax, 1644(%r12)
.Ltmp8219:
	.loc	1 2255 35 is_stmt 1
	cmpb	$0, 2556(%rsp)
	je	.LBB34_558
.LBB34_607:
	.loc	1 0 35 is_stmt 0
	movq	760(%rsp), %rdi
	.loc	1 2256 26 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2256 16 is_stmt 0
	testb	%al, %al
	je	.LBB34_558
	.loc	1 0 16
	movq	752(%rsp), %rdi
	.loc	1 2257 27 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	1 2257 16 is_stmt 0
	testb	%al, %al
	je	.LBB34_558
	.loc	1 0 16
	movq	4216(%rsp), %rsi
	cmpq	%rbx, %rsi
.Ltmp8220:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB34_654
.Ltmp8221:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rax
	movq	536(%rsp), %rcx
	.p2align	4
.LBB34_611:
.Ltmp8222:
	.loc	15 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB34_615
.Ltmp8223:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp8224:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%r9d, %r9d
	xorl	%r8d, %r8d
.Ltmp8225:
	.loc	16 0 18 is_stmt 0
.Ltmp8226:
	.p2align	4
.LBB34_613:
	.loc	17 134 13 is_stmt 1
	orl	(%rcx,%r8), %r9d
.Ltmp8227:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp8228:
	.loc	6 180 28
	jne	.LBB34_613
.Ltmp8229:
	.loc	18 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp8230:
	.loc	19 2054 74
	subq	%rdx, %rax
.Ltmp8231:
	.loc	17 136 12
	testl	%r9d, %r9d
	je	.LBB34_611
	jmp	.LBB34_558
.Ltmp8232:
.LBB34_399:
	.loc	1 1696 12
	testb	%r8b, %r8b
	je	.LBB34_400
	.loc	1 0 12 is_stmt 0
	leaq	6016(%rsp), %rdi
	leaq	1648(%r12), %rbx
.Ltmp8233:
	.loc	1 1943 24 is_stmt 1
	movq	%rbx, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	6752(%rsp), %rdi
	leaq	1848(%r12), %r15
.Ltmp8234:
	.loc	1 1944 25
	movq	%r15, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp8235:
	.loc	1 1949 19
	movzbl	1536(%r12), %eax
	movb	%al, 32(%rsp)
.Ltmp8236:
	.loc	1 1950 21
	movzbl	1537(%r12), %eax
	movb	%al, 224(%rsp)
.Ltmp8237:
	.loc	1 1951 16
	movq	1624(%r12), %r14
.Ltmp8238:
	.loc	1 1952 16
	movq	1632(%r12), %r13
.Ltmp8239:
	.loc	1 1953 27
	movl	1640(%r12), %eax
	movq	%rax, 992(%rsp)
.Ltmp8240:
	.loc	1 1954 27
	movl	1644(%r12), %eax
	movq	%rax, 424(%rsp)
	leaq	10216(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r12
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r12
	leaq	9184(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r12
	movq	440(%rsp), %r12
	leaq	800(%rsp), %rdi
.Ltmp8241:
	.loc	1 1961 32
	movq	%rbx, %rsi
	movq	%r14, %rdx
	movq	%r13, 4200(%rsp)
	movq	%r13, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
.Ltmp8242:
	.loc	1 1962 33
	movq	1624(%r12), %rdx
	movq	1632(%r12), %rcx
	leaq	576(%rsp), %rdi
	movq	%r15, %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	312(%rsp), %r10
.Ltmp8243:
	.loc	4 3758 16
	leaq	31(%r10), %r9
	shrq	$5, %r9
.Ltmp8244:
	.loc	8 446 20
	je	.LBB34_478
.Ltmp8245:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp8246:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 256(%rsp)
	vmovaps	%ymm1, 2240(%rsp)
	cmpb	$0, 32(%rsp)
	jne	.LBB34_481
.Ltmp8247:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 2240(%rsp)
.LBB34_481:
	movabsq	$2305843009213693944, %rcx
	cmpb	$0, 224(%rsp)
	jne	.LBB34_483
	vmovaps	%ymm0, 256(%rsp)
.LBB34_483:
	vmovaps	6016(%rsp), %ymm3
	vmovaps	6048(%rsp), %ymm9
	vmovaps	6080(%rsp), %ymm14
	vmovaps	6112(%rsp), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	vmovaps	6144(%rsp), %ymm0
	vmovaps	%ymm0, 5952(%rsp)
	vmovaps	6176(%rsp), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	6208(%rsp), %ymm0
	vmovaps	%ymm0, 5984(%rsp)
	vmovaps	6240(%rsp), %ymm0
	vmovaps	%ymm0, 704(%rsp)
	vmovaps	6272(%rsp), %ymm0
	vmovaps	%ymm0, 5920(%rsp)
	vmovaps	6304(%rsp), %ymm0
	vmovaps	%ymm0, 5888(%rsp)
	vmovaps	6336(%rsp), %ymm0
	vmovaps	%ymm0, 5856(%rsp)
	vmovaps	6368(%rsp), %ymm0
	vmovaps	%ymm0, 7488(%rsp)
	vmovaps	6752(%rsp), %ymm5
	vmovaps	6784(%rsp), %ymm15
	vmovaps	6816(%rsp), %ymm1
	vmovaps	6848(%rsp), %ymm0
	vmovaps	%ymm0, 448(%rsp)
	movl	904(%rsp), %eax
	movl	%eax, 24(%rsp)
	movl	680(%rsp), %eax
	movl	%eax, 28(%rsp)
	movl	$32, %edi
	addq	$7, %rcx
	movq	%rcx, 4184(%rsp)
	movq	432(%rsp), %r11
	movq	536(%rsp), %r8
	movq	%r10, %rcx
	xorl	%esi, %esi
	vmovaps	6880(%rsp), %ymm0
	vmovaps	%ymm0, 5824(%rsp)
	vmovaps	6912(%rsp), %ymm8
	vmovaps	6944(%rsp), %ymm12
	vmovaps	6976(%rsp), %ymm6
	vmovaps	7008(%rsp), %ymm4
	vmovaps	7040(%rsp), %ymm13
	vmovaps	7072(%rsp), %ymm10
	vmovaps	7104(%rsp), %ymm11
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 1056(%rsp)
	vmovaps	576(%rsp), %ymm0
	vmovaps	%ymm0, 1088(%rsp)
	vmovaps	6688(%rsp), %ymm7
	vmovaps	7424(%rsp), %ymm0
	vmovaps	%ymm0, 352(%rsp)
	movq	184(%rsp), %rbx
.LBB34_486:
	vmovaps	%ymm11, 5792(%rsp)
.Ltmp8248:
	.loc	4 2584 13 is_stmt 1
	cmpq	$32, %rcx
	movl	$32, %eax
	cmovbq	%rcx, %rax
	cmpq	$1, %rax
	movq	%rax, 4208(%rsp)
	adcq	$0, %rax
.Ltmp8249:
	.loc	1 1967 55
	movq	%r10, %rdx
	subq	%rsi, %rdx
.Ltmp8250:
	.loc	10 1078 5
	cmpq	$32, %rdx
	cmovaeq	%rdi, %rdx
.Ltmp8251:
	.loc	1 1968 31
	leaq	(,%rsi,8), %rdi
	movq	%rsi, 1040(%rsp)
	movq	%rdx, 2640(%rsp)
.Ltmp8252:
	.loc	1 1972 39
	addq	%rdx, %rsi
	shlq	$3, %rsi
.Ltmp8253:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB34_488
	cmpq	%rbx, %rsi
	ja	.LBB34_488
.Ltmp8254:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm12, 5696(%rsp)
	vmovaps	%ymm8, 7520(%rsp)
	vmovaps	%ymm7, 7552(%rsp)
	vmovaps	%ymm10, 480(%rsp)
	vmovaps	%ymm13, 224(%rsp)
	vmovaps	%ymm4, 5728(%rsp)
	vmovaps	%ymm6, 384(%rsp)
	movq	%rcx, 4192(%rsp)
	vmovaps	%ymm3, %ymm13
	vmovaps	%ymm5, 5760(%rsp)
	vmovaps	%ymm1, 1632(%rsp)
	vmovaps	%ymm15, 1120(%rsp)
	shll	$5, %eax
.Ltmp8255:
	.loc	11 304 12 is_stmt 1
	cmpq	1040(%rsp), %r10
	jne	.LBB34_492
	.loc	11 0 12 is_stmt 0
	vmovaps	5952(%rsp), %ymm12
	vmovaps	5984(%rsp), %ymm8
	vmovaps	704(%rsp), %ymm15
	vmovaps	5920(%rsp), %ymm5
	vmovaps	5888(%rsp), %ymm7
	vmovaps	5856(%rsp), %ymm11
	vmovaps	7488(%rsp), %ymm10
	.loc	11 304 12
	jmp	.LBB34_494
.Ltmp8256:
.LBB34_492:
	.loc	11 0 12
	vmovaps	(%r12), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	32(%r12), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	64(%r12), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	96(%r12), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	128(%r12), %ymm0
	vmovaps	%ymm0, 928(%rsp)
	vmovaps	160(%r12), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	192(%r12), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	224(%r12), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	256(%r12), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	288(%r12), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	vmovaps	320(%r12), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm9, %ymm4
	vmovaps	5856(%rsp), %ymm6
	vmovaps	5888(%rsp), %ymm3
	vmovaps	5920(%rsp), %ymm0
	vmovaps	704(%rsp), %ymm1
	vmovaps	352(%r12), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	384(%r12), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	vmovaps	416(%r12), %ymm5
	vmovaps	%ymm5, 2112(%rsp)
	vmovaps	448(%r12), %ymm5
	vmovaps	%ymm5, 2080(%rsp)
	vmovaps	480(%r12), %ymm5
	vmovaps	%ymm5, 1504(%rsp)
	vmovaps	512(%r12), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	544(%r12), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	576(%r12), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	608(%r12), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	640(%r12), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	672(%r12), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	704(%r12), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	736(%r12), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	768(%r12), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	800(%r12), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	832(%r12), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	864(%r12), %ymm2
	vmovaps	%ymm2, 1184(%rsp)
	vmovaps	896(%r12), %ymm2
	vmovaps	%ymm2, 1408(%rsp)
	vmovaps	928(%r12), %ymm2
	vmovaps	%ymm2, 1760(%rsp)
	vmovaps	960(%r12), %ymm2
	vmovaps	%ymm2, 1152(%rsp)
	vmovaps	992(%r12), %ymm2
	vmovaps	%ymm2, 1376(%rsp)
	vmovaps	1024(%r12), %ymm2
	vmovaps	%ymm2, 1728(%rsp)
	vmovaps	1056(%r12), %ymm2
	vmovaps	%ymm2, 1696(%rsp)
	vmovaps	1088(%r12), %ymm2
	vmovaps	%ymm2, 1664(%rsp)
	vmovaps	1120(%r12), %ymm2
	vmovaps	%ymm2, 2400(%rsp)
	vmovaps	1152(%r12), %ymm2
	vmovaps	%ymm2, 2368(%rsp)
	vmovaps	1184(%r12), %ymm2
	vmovaps	%ymm2, 2336(%rsp)
	vmovaps	1216(%r12), %ymm2
	vmovaps	%ymm2, 2304(%rsp)
	vmovaps	1248(%r12), %ymm2
	vmovaps	%ymm2, 2272(%rsp)
	vmovaps	1280(%r12), %ymm2
	vmovaps	%ymm2, 2496(%rsp)
	vmovaps	1312(%r12), %ymm2
	vmovaps	%ymm2, 2464(%rsp)
	vmovaps	1344(%r12), %ymm2
	vmovaps	%ymm2, 2560(%rsp)
	vmovaps	1376(%r12), %ymm2
	vmovaps	%ymm2, 2432(%rsp)
	vmovaps	1408(%r12), %ymm2
	vmovaps	%ymm2, 2592(%rsp)
	vmovaps	1440(%r12), %ymm2
	vmovaps	%ymm2, 2656(%rsp)
	vmovaps	1472(%r12), %ymm2
	vmovaps	%ymm2, 768(%rsp)
	vmovaps	1504(%r12), %ymm2
	vmovaps	%ymm2, 704(%rsp)
	vmovaps	5952(%rsp), %ymm12
	vmovaps	5984(%rsp), %ymm8
	.p2align	4
.LBB34_493:
	vmovaps	%ymm6, 32(%rsp)
	vmovaps	%ymm13, %ymm9
.Ltmp8257:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r8,%rcx), %ymm13
.Ltmp8258:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm13, %ymm5
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp8259:
	.loc	29 48 14
	vaddps	%ymm2, %ymm5, %ymm5
.Ltmp8260:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm13, %ymm6
.Ltmp8261:
	.loc	29 48 14
	vaddps	%ymm2, %ymm6, %ymm6
.Ltmp8262:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm13, %ymm7
.Ltmp8263:
	.loc	29 48 14
	vaddps	%ymm2, %ymm7, %ymm7
.Ltmp8264:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm9, %ymm11
.Ltmp8265:
	.loc	29 48 14
	vaddps	%ymm5, %ymm11, %ymm5
.Ltmp8266:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm9, %ymm11
.Ltmp8267:
	.loc	29 48 14
	vaddps	%ymm6, %ymm11, %ymm6
.Ltmp8268:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm9, %ymm11
.Ltmp8269:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm7
.Ltmp8270:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm13, %ymm11
.Ltmp8271:
	.loc	29 48 14
	vaddps	%ymm2, %ymm11, %ymm11
.Ltmp8272:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm9, %ymm15
.Ltmp8273:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
	vmovaps	%ymm8, %ymm15
	vmovaps	192(%rsp), %ymm8
	vmovaps	%ymm12, %ymm2
	vmovaps	96(%rsp), %ymm12
	vmovaps	%ymm14, %ymm10
	vmovaps	%ymm4, %ymm14
.Ltmp8274:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm4, %ymm4
.Ltmp8275:
	.loc	29 48 14
	vaddps	%ymm4, %ymm5, %ymm4
.Ltmp8276:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm14, %ymm5
.Ltmp8277:
	.loc	29 48 14
	vaddps	%ymm5, %ymm6, %ymm5
.Ltmp8278:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm14, %ymm6
.Ltmp8279:
	.loc	29 48 14
	vaddps	%ymm6, %ymm7, %ymm6
.Ltmp8280:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm14, %ymm7
.Ltmp8281:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm7
.Ltmp8282:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm10, %ymm11
.Ltmp8283:
	.loc	29 48 14
	vaddps	%ymm4, %ymm11, %ymm4
.Ltmp8284:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm10, %ymm11
.Ltmp8285:
	.loc	29 48 14
	vaddps	%ymm5, %ymm11, %ymm5
.Ltmp8286:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm10, %ymm11
.Ltmp8287:
	.loc	29 48 14
	vaddps	%ymm6, %ymm11, %ymm6
	vmovaps	%ymm10, 96(%rsp)
.Ltmp8288:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm10, %ymm11
.Ltmp8289:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm7
.Ltmp8290:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm12, %ymm11
.Ltmp8291:
	.loc	29 48 14
	vaddps	%ymm4, %ymm11, %ymm4
.Ltmp8292:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm12, %ymm11
.Ltmp8293:
	.loc	29 48 14
	vaddps	%ymm5, %ymm11, %ymm5
.Ltmp8294:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm12, %ymm11
.Ltmp8295:
	.loc	29 48 14
	vaddps	%ymm6, %ymm11, %ymm6
.Ltmp8296:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm12, %ymm11
.Ltmp8297:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm7
.Ltmp8298:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm2, %ymm11
.Ltmp8299:
	.loc	29 48 14
	vaddps	%ymm4, %ymm11, %ymm4
.Ltmp8300:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm2, %ymm11
.Ltmp8301:
	.loc	29 48 14
	vaddps	%ymm5, %ymm11, %ymm5
.Ltmp8302:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm2, %ymm11
.Ltmp8303:
	.loc	29 48 14
	vaddps	%ymm6, %ymm11, %ymm6
	vmovaps	%ymm2, 192(%rsp)
.Ltmp8304:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm2, %ymm11
.Ltmp8305:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm7
.Ltmp8306:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm8, %ymm11
.Ltmp8307:
	.loc	29 48 14
	vaddps	%ymm4, %ymm11, %ymm4
.Ltmp8308:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm8, %ymm11
.Ltmp8309:
	.loc	29 48 14
	vaddps	%ymm5, %ymm11, %ymm5
.Ltmp8310:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm8, %ymm11
.Ltmp8311:
	.loc	29 48 14
	vaddps	%ymm6, %ymm11, %ymm6
.Ltmp8312:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm8, %ymm11
.Ltmp8313:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm7
.Ltmp8314:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm15, %ymm11
.Ltmp8315:
	.loc	29 48 14
	vaddps	%ymm4, %ymm11, %ymm4
.Ltmp8316:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm15, %ymm11
.Ltmp8317:
	.loc	29 48 14
	vaddps	%ymm5, %ymm11, %ymm11
.Ltmp8318:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm15, %ymm5
.Ltmp8319:
	.loc	29 48 14
	vaddps	%ymm5, %ymm6, %ymm6
.Ltmp8320:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm15, %ymm5
.Ltmp8321:
	.loc	29 48 14
	vaddps	%ymm5, %ymm7, %ymm7
	vmovaps	%ymm1, %ymm5
.Ltmp8322:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm1, %ymm1
.Ltmp8323:
	.loc	29 48 14
	vaddps	%ymm1, %ymm4, %ymm1
.Ltmp8324:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm5, %ymm4
.Ltmp8325:
	.loc	29 48 14
	vaddps	%ymm4, %ymm11, %ymm4
.Ltmp8326:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm5, %ymm11
.Ltmp8327:
	.loc	29 48 14
	vaddps	%ymm6, %ymm11, %ymm6
.Ltmp8328:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm5, %ymm11
.Ltmp8329:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm11
	vmovaps	%ymm0, %ymm7
.Ltmp8330:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm0, %ymm0
.Ltmp8331:
	.loc	29 48 14
	vaddps	%ymm0, %ymm1, %ymm0
.Ltmp8332:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm7, %ymm1
.Ltmp8333:
	.loc	29 48 14
	vaddps	%ymm1, %ymm4, %ymm1
.Ltmp8334:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm7, %ymm4
.Ltmp8335:
	.loc	29 48 14
	vaddps	%ymm4, %ymm6, %ymm4
.Ltmp8336:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm7, %ymm6
.Ltmp8337:
	.loc	29 48 14
	vaddps	%ymm6, %ymm11, %ymm6
	vmovaps	%ymm3, %ymm11
.Ltmp8338:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm3, %ymm3
.Ltmp8339:
	.loc	29 48 14
	vaddps	%ymm3, %ymm0, %ymm0
.Ltmp8340:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm11, %ymm3
.Ltmp8341:
	.loc	29 48 14
	vaddps	%ymm3, %ymm1, %ymm1
.Ltmp8342:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm11, %ymm3
.Ltmp8343:
	.loc	29 48 14
	vaddps	%ymm3, %ymm4, %ymm3
.Ltmp8344:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm11, %ymm4
.Ltmp8345:
	.loc	29 48 14
	vaddps	%ymm4, %ymm6, %ymm4
	vmovaps	32(%rsp), %ymm10
.Ltmp8346:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm10, %ymm2
.Ltmp8347:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp8348:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm10, %ymm2
.Ltmp8349:
	.loc	29 48 14
	vaddps	%ymm2, %ymm1, %ymm1
.Ltmp8350:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm10, %ymm2
.Ltmp8351:
	.loc	29 48 14
	vaddps	%ymm2, %ymm3, %ymm2
.Ltmp8352:
	.loc	29 283 14
	vmulps	704(%rsp), %ymm10, %ymm3
.Ltmp8353:
	.loc	29 48 14
	vaddps	%ymm3, %ymm4, %ymm3
.Ltmp8354:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm4
.Ltmp8355:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm4, %ymm0, %ymm0
.Ltmp8356:
	.loc	29 82 19
	vandps	%ymm4, %ymm8, %ymm6
.Ltmp8357:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm6, %ymm0
.Ltmp8358:
	.loc	29 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp8359:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8360:
	.loc	29 82 19
	vandps	%ymm4, %ymm2, %ymm1
.Ltmp8361:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8362:
	.loc	29 82 19
	vandps	%ymm4, %ymm3, %ymm1
.Ltmp8363:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8364:
	.loc	12 551 14
	vmovups	%ymm0, 10216(%rsp,%rcx)
.Ltmp8365:
	.loc	11 304 12
	addq	$32, %rcx
	vmovaps	%ymm9, %ymm4
	vmovaps	%ymm11, %ymm6
	vmovaps	%ymm7, %ymm3
	vmovaps	%ymm5, %ymm0
	vmovaps	%ymm15, %ymm1
	cmpq	%rcx, %rax
	jne	.LBB34_493
.Ltmp8366:
.LBB34_494:
	.loc	11 0 12 is_stmt 0
	vmovaps	%ymm9, 1344(%rsp)
	vmovaps	%ymm13, 1568(%rsp)
	vmovaps	%ymm14, 1600(%rsp)
	vmovaps	96(%rsp), %ymm0
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 6112(%rsp)
	vmovaps	%ymm12, 6144(%rsp)
	vmovaps	192(%rsp), %ymm0
	vmovaps	%ymm0, 6176(%rsp)
	vmovaps	%ymm8, 6208(%rsp)
	vmovaps	%ymm15, 6240(%rsp)
	vmovaps	%ymm5, 6272(%rsp)
	vmovaps	%ymm7, 6304(%rsp)
	vmovaps	%ymm11, 6336(%rsp)
	vmovaps	%ymm10, 6368(%rsp)
.Ltmp8367:
	.loc	5 438 16
	cmpq	88(%rsp), %rsi
	ja	.LBB34_495
.Ltmp8368:
	.loc	5 0 16 is_stmt 0
	vmovaps	%ymm10, 7488(%rsp)
	vmovaps	%ymm11, 5856(%rsp)
	vmovaps	%ymm7, 5888(%rsp)
	vmovaps	%ymm5, 5920(%rsp)
	vmovaps	%ymm15, 704(%rsp)
	vmovaps	%ymm8, 5984(%rsp)
	vmovaps	%ymm12, 5952(%rsp)
.Ltmp8369:
	.loc	11 304 12 is_stmt 1
	cmpq	1040(%rsp), %r10
	vmovaps	224(%rsp), %ymm13
.Ltmp8370:
	.loc	11 304 12 is_stmt 0
	jne	.LBB34_499
	.loc	11 0 12
	movq	992(%rsp), %r13
	vmovaps	1120(%rsp), %ymm3
	vmovaps	1632(%rsp), %ymm15
	vmovaps	5760(%rsp), %ymm5
	vmovaps	384(%rsp), %ymm9
	vmovaps	5728(%rsp), %ymm7
	vmovaps	448(%rsp), %ymm0
	vmovaps	5824(%rsp), %ymm14
	vmovaps	7520(%rsp), %ymm8
	vmovaps	5696(%rsp), %ymm12
	vmovaps	5792(%rsp), %ymm11
	.loc	11 304 12
	jmp	.LBB34_501
.Ltmp8371:
.LBB34_499:
	.loc	11 0 12
	vmovaps	(%r12), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	vmovaps	32(%r12), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	64(%r12), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	96(%r12), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	128(%r12), %ymm0
	vmovaps	%ymm0, 1536(%rsp)
	vmovaps	160(%r12), %ymm0
	vmovaps	%ymm0, 928(%rsp)
	vmovaps	192(%r12), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	224(%r12), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	256(%r12), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	288(%r12), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	320(%r12), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	xorl	%ecx, %ecx
	vmovaps	5696(%rsp), %ymm2
	vmovaps	7520(%rsp), %ymm1
	vmovaps	5824(%rsp), %ymm4
	vmovaps	448(%rsp), %ymm6
	vmovaps	1632(%rsp), %ymm10
	vmovaps	352(%r12), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	384(%r12), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	416(%r12), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	448(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	480(%r12), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	512(%r12), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	544(%r12), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	576(%r12), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	608(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	640(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	672(%r12), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	704(%r12), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	736(%r12), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	768(%r12), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	800(%r12), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	832(%r12), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	864(%r12), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	896(%r12), %ymm0
	vmovaps	%ymm0, 1184(%rsp)
	vmovaps	928(%r12), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	960(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	992(%r12), %ymm0
	vmovaps	%ymm0, 1152(%rsp)
	vmovaps	1024(%r12), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1056(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1088(%r12), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	1120(%r12), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1152(%r12), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1184(%r12), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1216(%r12), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	1248(%r12), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	1280(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	1312(%r12), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1344(%r12), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1376(%r12), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 2432(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 2592(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 2656(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	movq	992(%rsp), %r13
	vmovaps	1120(%rsp), %ymm3
	vmovaps	5760(%rsp), %ymm5
	vmovaps	384(%rsp), %ymm9
	vmovaps	5728(%rsp), %ymm7
	.p2align	4
.LBB34_500:
	vmovaps	%ymm13, 224(%rsp)
	vmovaps	%ymm9, 384(%rsp)
	vmovaps	%ymm3, %ymm15
	vmovaps	%ymm5, %ymm3
.Ltmp8372:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r11,%rcx), %ymm5
.Ltmp8373:
	.loc	29 283 14
	vmulps	32(%rsp), %ymm5, %ymm8
	vxorps	%xmm11, %xmm11, %xmm11
.Ltmp8374:
	.loc	29 48 14
	vaddps	%ymm11, %ymm8, %ymm8
.Ltmp8375:
	.loc	29 283 14
	vmulps	544(%rsp), %ymm5, %ymm9
.Ltmp8376:
	.loc	29 48 14
	vaddps	%ymm11, %ymm9, %ymm9
.Ltmp8377:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm5, %ymm14
.Ltmp8378:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm14
	vmovaps	%ymm2, %ymm0
.Ltmp8379:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm3, %ymm2
.Ltmp8380:
	.loc	29 48 14
	vaddps	%ymm2, %ymm8, %ymm2
.Ltmp8381:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm3, %ymm8
.Ltmp8382:
	.loc	29 48 14
	vaddps	%ymm8, %ymm9, %ymm8
.Ltmp8383:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm3, %ymm9
.Ltmp8384:
	.loc	29 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8385:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm5, %ymm14
.Ltmp8386:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm14
.Ltmp8387:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm3, %ymm13
.Ltmp8388:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8389:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm15, %ymm14
.Ltmp8390:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp8391:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm15, %ymm14
.Ltmp8392:
	.loc	29 48 14
	vaddps	%ymm14, %ymm8, %ymm8
.Ltmp8393:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm15, %ymm14
.Ltmp8394:
	.loc	29 48 14
	vaddps	%ymm14, %ymm9, %ymm9
.Ltmp8395:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm15, %ymm14
.Ltmp8396:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
	vmovaps	%ymm10, 448(%rsp)
.Ltmp8397:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm10, %ymm14
.Ltmp8398:
	.loc	29 48 14
	vaddps	%ymm2, %ymm14, %ymm2
.Ltmp8399:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm10, %ymm14
.Ltmp8400:
	.loc	29 48 14
	vaddps	%ymm14, %ymm8, %ymm8
.Ltmp8401:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm10, %ymm14
.Ltmp8402:
	.loc	29 48 14
	vaddps	%ymm14, %ymm9, %ymm9
.Ltmp8403:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm10, %ymm14
.Ltmp8404:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
	vmovaps	%ymm6, %ymm14
.Ltmp8405:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm6, %ymm6
.Ltmp8406:
	.loc	29 48 14
	vaddps	%ymm6, %ymm2, %ymm2
.Ltmp8407:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm14, %ymm6
.Ltmp8408:
	.loc	29 48 14
	vaddps	%ymm6, %ymm8, %ymm6
.Ltmp8409:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm14, %ymm8
.Ltmp8410:
	.loc	29 48 14
	vaddps	%ymm8, %ymm9, %ymm9
.Ltmp8411:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm14, %ymm8
.Ltmp8412:
	.loc	29 48 14
	vaddps	%ymm8, %ymm13, %ymm13
	vmovaps	%ymm4, %ymm8
.Ltmp8413:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm4, %ymm4
.Ltmp8414:
	.loc	29 48 14
	vaddps	%ymm4, %ymm2, %ymm2
.Ltmp8415:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm8, %ymm4
.Ltmp8416:
	.loc	29 48 14
	vaddps	%ymm4, %ymm6, %ymm4
.Ltmp8417:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm8, %ymm6
.Ltmp8418:
	.loc	29 48 14
	vaddps	%ymm6, %ymm9, %ymm6
.Ltmp8419:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm8, %ymm9
.Ltmp8420:
	.loc	29 48 14
	vaddps	%ymm9, %ymm13, %ymm13
	vmovaps	%ymm1, %ymm12
.Ltmp8421:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm1, %ymm1
.Ltmp8422:
	.loc	29 48 14
	vaddps	%ymm1, %ymm2, %ymm1
.Ltmp8423:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm12, %ymm2
.Ltmp8424:
	.loc	29 48 14
	vaddps	%ymm2, %ymm4, %ymm2
.Ltmp8425:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm12, %ymm4
.Ltmp8426:
	.loc	29 48 14
	vaddps	%ymm4, %ymm6, %ymm4
.Ltmp8427:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm12, %ymm6
.Ltmp8428:
	.loc	29 48 14
	vaddps	%ymm6, %ymm13, %ymm6
	vmovaps	480(%rsp), %ymm11
	vmovaps	224(%rsp), %ymm13
	vmovaps	%ymm7, %ymm10
	vmovaps	384(%rsp), %ymm7
	vmovaps	%ymm0, %ymm9
.Ltmp8429:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm0, %ymm0
.Ltmp8430:
	.loc	29 48 14
	vaddps	%ymm0, %ymm1, %ymm0
.Ltmp8431:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm9, %ymm1
.Ltmp8432:
	.loc	29 48 14
	vaddps	%ymm1, %ymm2, %ymm1
.Ltmp8433:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm9, %ymm2
.Ltmp8434:
	.loc	29 48 14
	vaddps	%ymm2, %ymm4, %ymm2
.Ltmp8435:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm9, %ymm4
.Ltmp8436:
	.loc	29 48 14
	vaddps	%ymm4, %ymm6, %ymm4
.Ltmp8437:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm7, %ymm6
.Ltmp8438:
	.loc	29 48 14
	vaddps	%ymm6, %ymm0, %ymm0
.Ltmp8439:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm7, %ymm6
.Ltmp8440:
	.loc	29 48 14
	vaddps	%ymm6, %ymm1, %ymm1
.Ltmp8441:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm7, %ymm6
.Ltmp8442:
	.loc	29 48 14
	vaddps	%ymm6, %ymm2, %ymm2
.Ltmp8443:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm7, %ymm6
.Ltmp8444:
	.loc	29 48 14
	vaddps	%ymm6, %ymm4, %ymm4
.Ltmp8445:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm10, %ymm6
.Ltmp8446:
	.loc	29 48 14
	vaddps	%ymm6, %ymm0, %ymm0
.Ltmp8447:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm10, %ymm6
.Ltmp8448:
	.loc	29 48 14
	vaddps	%ymm6, %ymm1, %ymm1
.Ltmp8449:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm10, %ymm6
.Ltmp8450:
	.loc	29 48 14
	vaddps	%ymm6, %ymm2, %ymm2
.Ltmp8451:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm10, %ymm6
.Ltmp8452:
	.loc	29 48 14
	vaddps	%ymm6, %ymm4, %ymm4
.Ltmp8453:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm13, %ymm6
.Ltmp8454:
	.loc	29 48 14
	vaddps	%ymm6, %ymm0, %ymm0
.Ltmp8455:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm13, %ymm6
.Ltmp8456:
	.loc	29 48 14
	vaddps	%ymm6, %ymm1, %ymm1
.Ltmp8457:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm13, %ymm6
.Ltmp8458:
	.loc	29 48 14
	vaddps	%ymm6, %ymm2, %ymm2
	vmovaps	%ymm13, 480(%rsp)
.Ltmp8459:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm13, %ymm6
.Ltmp8460:
	.loc	29 48 14
	vaddps	%ymm6, %ymm4, %ymm4
.Ltmp8461:
	.loc	29 283 14
	vmulps	2432(%rsp), %ymm11, %ymm6
.Ltmp8462:
	.loc	29 48 14
	vaddps	%ymm6, %ymm0, %ymm0
.Ltmp8463:
	.loc	29 283 14
	vmulps	2592(%rsp), %ymm11, %ymm6
.Ltmp8464:
	.loc	29 48 14
	vaddps	%ymm6, %ymm1, %ymm1
.Ltmp8465:
	.loc	29 283 14
	vmulps	2656(%rsp), %ymm11, %ymm6
.Ltmp8466:
	.loc	29 48 14
	vaddps	%ymm6, %ymm2, %ymm2
.Ltmp8467:
	.loc	29 283 14
	vmulps	768(%rsp), %ymm11, %ymm6
.Ltmp8468:
	.loc	29 48 14
	vaddps	%ymm6, %ymm4, %ymm4
.Ltmp8469:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm6
.Ltmp8470:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm6, %ymm0, %ymm0
.Ltmp8471:
	.loc	29 82 19
	vandps	%ymm6, %ymm12, %ymm13
.Ltmp8472:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm13, %ymm0
	vmovaps	%ymm10, %ymm13
.Ltmp8473:
	.loc	29 82 19
	vandps	%ymm6, %ymm1, %ymm1
.Ltmp8474:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8475:
	.loc	29 82 19
	vandps	%ymm6, %ymm2, %ymm1
.Ltmp8476:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8477:
	.loc	29 82 19
	vandps	%ymm6, %ymm4, %ymm1
.Ltmp8478:
	.loc	29 233 14
	vmaxps	%ymm1, %ymm0, %ymm0
.Ltmp8479:
	.loc	12 551 14
	vmovups	%ymm0, 9184(%rsp,%rcx)
	vmovaps	448(%rsp), %ymm0
.Ltmp8480:
	.loc	11 304 12
	addq	$32, %rcx
	vmovaps	%ymm12, %ymm2
	vmovaps	%ymm8, %ymm1
	vmovaps	%ymm14, %ymm4
	vmovaps	%ymm0, %ymm6
	vmovaps	%ymm15, %ymm10
	cmpq	%rcx, %rax
	jne	.LBB34_500
.Ltmp8481:
.LBB34_501:
	.loc	11 0 12 is_stmt 0
	vmovaps	%ymm15, 1632(%rsp)
	vmovaps	%ymm0, 448(%rsp)
	.loc	1 1764 5 is_stmt 1
	vmovaps	%ymm0, 6848(%rsp)
	vmovaps	%ymm14, 5824(%rsp)
	vmovaps	%ymm14, 6880(%rsp)
	vmovaps	%ymm8, 6912(%rsp)
	vmovaps	%ymm12, 6944(%rsp)
	vmovaps	%ymm9, 6976(%rsp)
	vmovaps	%ymm7, 7008(%rsp)
	vmovaps	%ymm13, 7040(%rsp)
	vmovaps	480(%rsp), %ymm10
	vmovaps	%ymm10, 7072(%rsp)
	vmovaps	%ymm11, 7104(%rsp)
.Ltmp8482:
	.loc	11 304 12
	cmpq	1040(%rsp), %r10
	vmovaps	%ymm7, %ymm4
	vmovaps	%ymm9, %ymm6
.Ltmp8483:
	.loc	1 1984 19
	jne	.LBB34_503
.Ltmp8484:
	.loc	1 0 19 is_stmt 0
	vmovaps	%ymm3, %ymm15
	vmovaps	1344(%rsp), %ymm9
	vmovaps	7552(%rsp), %ymm7
.LBB34_485:
	movq	%r13, 992(%rsp)
	movq	1040(%rsp), %rsi
	addq	$32, %rsi
	decq	%r9
	movq	4192(%rsp), %rcx
.Ltmp8485:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %rcx
	addq	$1024, %r8
	addq	$1024, %r11
	testq	%r9, %r9
	vmovaps	1632(%rsp), %ymm1
	vmovaps	1600(%rsp), %ymm14
	vmovaps	1568(%rsp), %ymm3
	movl	$32, %edi
	jne	.LBB34_486
	jmp	.LBB34_546
.Ltmp8486:
.LBB34_503:
	.loc	8 0 20 is_stmt 0
	vmovaps	%ymm6, 384(%rsp)
	vmovaps	%ymm4, 5728(%rsp)
	vmovaps	%ymm11, 5792(%rsp)
	vmovaps	%ymm12, 5696(%rsp)
	movq	%r11, 1664(%rsp)
	movq	%r9, 1696(%rsp)
.Ltmp8487:
	.loc	1 1990 21 is_stmt 1
	movq	888(%rsp), %r15
	movq	896(%rsp), %rdi
	.loc	1 1991 21
	movq	664(%rsp), %rsi
	movq	672(%rsp), %rax
	movq	%rax, 1760(%rsp)
	vmovaps	6400(%rsp), %ymm9
	vmovaps	6528(%rsp), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	6720(%rsp), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	7136(%rsp), %ymm14
	vmovaps	7264(%rsp), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	7456(%rsp), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	movq	832(%rsp), %rbx
	movl	$0, %r11d
	movq	840(%rsp), %rdx
	movq	880(%rsp), %rax
	movq	%rax, 2176(%rsp)
	movq	848(%rsp), %rax
	movq	%rax, 1984(%rsp)
	movq	856(%rsp), %rax
	movq	%rax, 2080(%rsp)
	movq	872(%rsp), %rax
	movq	%rax, 1504(%rsp)
	movq	864(%rsp), %rax
	movq	%rax, 1952(%rsp)
	movq	608(%rsp), %r9
	movq	616(%rsp), %rax
	vmovaps	1056(%rsp), %ymm15
	vmovaps	1088(%rsp), %ymm10
	movq	656(%rsp), %rcx
	movq	%rcx, 1920(%rsp)
	movq	624(%rsp), %rcx
	movq	%rcx, 1824(%rsp)
	movq	632(%rsp), %rcx
	movq	%rcx, 1472(%rsp)
	movq	648(%rsp), %rcx
	movq	%rcx, 1440(%rsp)
	movq	640(%rsp), %rcx
	movq	%rcx, 1792(%rsp)
	movl	28(%rsp), %ecx
	movq	%rcx, 32(%rsp)
	movl	24(%rsp), %ecx
	movq	%rcx, 128(%rsp)
	xorl	%r10d, %r10d
	vmovaps	%ymm3, 1120(%rsp)
	vmovaps	7552(%rsp), %ymm7
	vbroadcastss	.LCPI34_2(%rip), %ymm12
	movq	2640(%rsp), %rcx
	movq	%r8, 2648(%rsp)
	movq	%r15, 1152(%rsp)
	movq	%rdi, 1376(%rsp)
	movq	%rsi, 1728(%rsp)
.LBB34_504:
	.loc	1 0 21 is_stmt 0
	movq	%r13, 992(%rsp)
	movq	%r10, 224(%rsp)
	.loc	1 1989 21 is_stmt 1
	movq	%rcx, %r8
	subq	%r10, %r8
.Ltmp8488:
	.loc	1 1575 16
	movq	1624(%r12), %r10
.Ltmp8489:
	.loc	1 1576 16
	movq	1632(%r12), %r13
	movq	424(%rsp), %r12
.Ltmp8490:
	.loc	1 1577 25
	leaq	1(%r12), %rcx
.Ltmp8491:
	.loc	1 1148 8
	cmpq	%r10, %rcx
	movq	%r10, %rcx
	cmovbq	%r11, %rcx
	negq	%rcx
	addq	%r12, %rcx
	incq	%rcx
	movq	%rcx, 1536(%rsp)
.Ltmp8492:
	.loc	1 1578 28
	addq	%r12, %r15
.Ltmp8493:
	.loc	1 1148 8
	cmpq	%r10, %r15
	movq	%r10, %rcx
	cmovbq	%r11, %rcx
	subq	%rcx, %r15
.Ltmp8494:
	.loc	1 1579 29
	addq	%r12, %rsi
.Ltmp8495:
	.loc	1 1148 8
	cmpq	%r10, %rsi
	movq	%r10, %rcx
	cmovbq	%r11, %rcx
	subq	%rcx, %rsi
.Ltmp8496:
	.loc	1 1580 33
	addq	%r12, %rdi
.Ltmp8497:
	.loc	1 1148 8
	cmpq	%r10, %rdi
	movq	%r10, %rcx
	cmovbq	%r11, %rcx
	subq	%rcx, %rdi
	movq	%rdi, 928(%rsp)
	movq	1760(%rsp), %rcx
.Ltmp8498:
	.loc	1 1581 34
	leaq	(%rcx,%r12), %rdi
.Ltmp8499:
	.loc	1 1148 8
	cmpq	%r10, %rdi
	movq	%r10, %rcx
	cmovbq	%r11, %rcx
	subq	%rcx, %rdi
.Ltmp8500:
	.loc	1 1583 14
	movq	%r10, %r11
	subq	%r12, %r11
.Ltmp8501:
	.loc	10 1078 5
	cmpq	%r8, %r11
	cmovbq	%r11, %r8
	movq	992(%rsp), %rcx
.Ltmp8502:
	.loc	1 1584 14
	subq	%rcx, %r13
.Ltmp8503:
	.loc	10 1078 5
	cmpq	%r8, %r13
	movq	%r13, 544(%rsp)
	cmovbq	%r13, %r8
.Ltmp8504:
	.loc	1 1585 14
	movq	%r10, %r12
	movq	1536(%rsp), %rcx
	subq	%rcx, %r12
.Ltmp8505:
	.loc	10 1078 5
	cmpq	%r8, %r12
	cmovbq	%r12, %r8
.Ltmp8506:
	.loc	1 1586 14
	movq	%r10, %r13
	movq	%r15, 960(%rsp)
	subq	%r15, %r13
.Ltmp8507:
	.loc	10 1078 5
	cmpq	%r8, %r13
	cmovbq	%r13, %r8
.Ltmp8508:
	.loc	1 1587 14
	movq	%r10, %r15
	movq	%rsi, 1312(%rsp)
	subq	%rsi, %r15
.Ltmp8509:
	.loc	10 1078 5
	cmpq	%r8, %r15
	cmovbq	%r15, %r8
.Ltmp8510:
	.loc	1 1588 14
	movq	%r10, %rcx
	movq	928(%rsp), %rsi
	subq	%rsi, %rcx
.Ltmp8511:
	.loc	10 1078 5
	cmpq	%r8, %rcx
	cmovbq	%rcx, %r8
	movq	%rdi, 1280(%rsp)
.Ltmp8512:
	.loc	1 1589 14
	subq	%rdi, %r10
.Ltmp8513:
	.loc	10 1078 5
	cmpq	%r8, %r10
	cmovbq	%r10, %r8
	movq	1040(%rsp), %rsi
	movq	224(%rsp), %rdi
.Ltmp8514:
	.loc	1 1995 28
	addq	%rsi, %rdi
	movq	%r8, 1184(%rsp)
.Ltmp8515:
	.loc	1 1997 55
	leaq	(%r8,%rdi), %rsi
.Ltmp8516:
	.loc	1 1995 28
	shlq	$3, %rdi
.Ltmp8517:
	.loc	1 1997 55
	shlq	$3, %rsi
.Ltmp8518:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB34_506
	cmpq	184(%rsp), %rsi
	ja	.LBB34_506
.Ltmp8519:
	.loc	5 451 16
	cmpq	88(%rsp), %rsi
	ja	.LBB34_508
.Ltmp8520:
	.loc	5 0 16 is_stmt 0
	movq	224(%rsp), %rsi
.Ltmp8521:
	.loc	1 1999 46 is_stmt 1
	leaq	(,%rsi,8), %r8
	movq	%r8, 320(%rsp)
	movq	1184(%rsp), %r8
	.loc	1 1999 61 is_stmt 0
	addq	%r8, %rsi
	movq	%rsi, 1408(%rsp)
	cmpq	$33, %rsi
.Ltmp8522:
	.loc	4 1050 16 is_stmt 1
	jae	.LBB34_642
.Ltmp8523:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm13, %ymm11
.Ltmp8524:
	.loc	11 304 12 is_stmt 1
	testq	%r8, %r8
	je	.LBB34_511
.Ltmp8525:
	.loc	11 0 12 is_stmt 0
	cmpq	%r15, %r13
	cmovbq	%r13, %r15
	cmpq	%rcx, %r15
	cmovaeq	%rcx, %r15
	cmpq	%r10, %r15
	cmovaeq	%r10, %r15
	cmpq	%r12, %r15
	cmovaeq	%r12, %r15
	cmpq	%r11, %r15
	cmovaeq	%r11, %r15
	movq	536(%rsp), %rcx
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 2208(%rsp)
	movq	544(%rsp), %rcx
	cmpq	%rcx, %r15
	cmovaeq	%rcx, %r15
	movq	432(%rsp), %rcx
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 2144(%rsp)
	movq	4208(%rsp), %rcx
	subq	224(%rsp), %rcx
	movq	320(%rsp), %rdi
	leaq	(%rsp,%rdi,4), %rsi
	addq	$10216, %rsi
	movq	%rsi, 1248(%rsp)
	leaq	(%rsp,%rdi,4), %rsi
	addq	$9184, %rsi
	movq	%rsi, 1216(%rsp)
	cmpq	%rcx, %r15
	cmovbq	%r15, %rcx
	andq	4184(%rsp), %rcx
	movq	%rcx, 2112(%rsp)
	xorl	%r12d, %r12d
	vmovaps	%ymm15, %ymm2
	vmovaps	%ymm10, %ymm13
	movq	424(%rsp), %r15
	.p2align	4
.LBB34_513:
.Ltmp8526:
	.loc	1 1502 26 is_stmt 1
	leaq	(%r12,%r15), %rcx
.Ltmp8527:
	.loc	1 1137 16
	leaq	(,%rcx,8), %r8
.Ltmp8528:
	.loc	1 1138 33
	leaq	8(,%rcx,8), %r11
.Ltmp8529:
	.loc	4 1050 16
	leaq	7(,%rcx,8), %rcx
	cmpq	%rdx, %rcx
	jae	.LBB34_643
.Ltmp8530:
	.loc	1 0 0 is_stmt 0
	movq	%r12, %rcx
	shlq	$5, %rcx
	movq	1248(%rsp), %rsi
	vmovups	(%rsi,%rcx), %ymm3
	movq	1216(%rsp), %rsi
	vmovups	(%rsi,%rcx), %ymm0
.Ltmp8531:
	vmaxps	%ymm3, %ymm0, %ymm1
	vmovaps	2240(%rsp), %ymm4
.Ltmp8532:
	.loc	29 585 19 is_stmt 1
	vblendvps	%ymm4, %ymm1, %ymm3, %ymm3
.Ltmp8533:
	.loc	29 360 14
	vdivps	%ymm3, %ymm9, %ymm4
	movq	960(%rsp), %rsi
.Ltmp8534:
	.loc	1 0 0 is_stmt 0
	addq	%r12, %rsi
.Ltmp8535:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm9, %ymm3, %ymm3
.Ltmp8536:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm4, %ymm12, %ymm3
.Ltmp8537:
	.loc	12 551 14
	vmovups	%ymm3, (%rbx,%r8,4)
.Ltmp8538:
	.loc	1 1130 16
	leaq	(,%rsi,8), %rdi
.Ltmp8539:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %r10
	cmpq	%rdx, %r10
	vmovaps	1632(%rsp), %ymm10
	jae	.LBB34_515
.Ltmp8540:
	.loc	4 0 16 is_stmt 0
	movq	%r11, 224(%rsp)
.Ltmp8541:
	.loc	12 551 14 is_stmt 1
	vmovups	(%rbx,%rdi,4), %ymm3
	vmovaps	%ymm3, %ymm15
	movq	128(%rsp), %r13
.Ltmp8542:
	.loc	1 1206 22
	testl	%r13d, %r13d
	je	.LBB34_518
.Ltmp8543:
	.loc	29 257 14
	vminps	%ymm3, %ymm2, %ymm15
.Ltmp8544:
.LBB34_518:
	.loc	29 0 14 is_stmt 0
	movq	1536(%rsp), %rdi
	leaq	(%r12,%rdi), %r10
.Ltmp8545:
	movl	%r13d, %r13d
.Ltmp8546:
	.loc	1 1212 20 is_stmt 1
	incq	%r13
	movq	2176(%rsp), %rdi
	movq	%rdi, %r15
	cmpq	%rdi, %r13
.Ltmp8547:
	.loc	1 1213 22
	jne	.LBB34_519
	.loc	1 0 22 is_stmt 0
.Ltmp8548:
	.p2align	4
.LBB34_521:
.Ltmp8549:
	.loc	1 1130 16 is_stmt 1
	leaq	(,%rsi,8), %rdi
.Ltmp8550:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %r13
	cmpq	%rdx, %r13
	jae	.LBB34_515
.Ltmp8551:
	.loc	29 257 14
	vminps	(%rbx,%rdi,4), %ymm3, %ymm3
.Ltmp8552:
	.loc	12 551 14
	vmovups	%ymm3, (%rbx,%rdi,4)
.Ltmp8553:
	.loc	1 1224 16
	testq	%rsi, %rsi
	cmoveq	%r14, %rsi
	.loc	1 1227 13
	decq	%rsi
.Ltmp8554:
	.loc	10 1916 50
	decq	%r15
.Ltmp8555:
	.loc	3 900 12
	jne	.LBB34_521
.Ltmp8556:
	.loc	3 0 12 is_stmt 0
	xorl	%r13d, %r13d
	vmovaps	%ymm15, %ymm2
	jmp	.LBB34_524
	.p2align	4
.LBB34_519:
.Ltmp8557:
	.loc	1 1130 16 is_stmt 1
	leaq	(,%r10,8), %rdi
.Ltmp8558:
	.loc	4 1050 16
	leaq	7(,%r10,8), %rsi
	cmpq	%rdx, %rsi
	jae	.LBB34_515
.Ltmp8559:
	.loc	12 551 14
	vmovups	(%rbx,%rdi,4), %ymm2
.Ltmp8560:
	.loc	29 257 14
	vminps	%ymm15, %ymm2, %ymm2
.Ltmp8561:
.LBB34_524:
	.loc	29 0 14 is_stmt 0
	movq	928(%rsp), %rsi
	leaq	(%r12,%rsi), %rdi
.Ltmp8562:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%rdi,8), %rsi
.Ltmp8563:
	.loc	1 1130 16
	shlq	$3, %rdi
	movq	2080(%rsp), %r11
.Ltmp8564:
	.loc	4 1050 16
	cmpq	%r11, %rsi
	jae	.LBB34_525
.Ltmp8565:
	.loc	4 0 16 is_stmt 0
	cmpq	%r11, 224(%rsp)
.Ltmp8566:
	.loc	4 1050 16
	ja	.LBB34_644
.Ltmp8567:
	.loc	4 0 16
	movq	992(%rsp), %rsi
	leaq	(%r12,%rsi), %r15
.Ltmp8568:
	vbroadcastss	.LCPI34_3(%rip), %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
	vroundps	$9, %ymm2, %ymm2
	vbroadcastss	.LCPI34_4(%rip), %ymm3
	vmulps	%ymm3, %ymm2, %ymm2
.Ltmp8569:
	vaddps	%ymm7, %ymm2, %ymm3
	movq	1984(%rsp), %rsi
	vsubps	(%rsi,%rdi,4), %ymm3, %ymm7
.Ltmp8570:
	.loc	12 551 14 is_stmt 1
	vmovups	%ymm2, (%rsi,%r8,4)
.Ltmp8571:
	.loc	29 360 14
	vdivps	2016(%rsp), %ymm7, %ymm2
.Ltmp8572:
	.loc	1 1659 43
	vmovaps	6656(%rsp), %ymm3
.Ltmp8573:
	.loc	29 347 14
	vsubps	%ymm2, %ymm12, %ymm2
.Ltmp8574:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm3, %ymm2, %ymm4
.Ltmp8575:
	.loc	29 283 14 is_stmt 1
	vmulps	2048(%rsp), %ymm4, %ymm4
.Ltmp8576:
	.loc	29 48 14
	vaddps	%ymm4, %ymm3, %ymm3
.Ltmp8577:
	.loc	29 233 14
	vmaxps	%ymm3, %ymm2, %ymm3
.Ltmp8578:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	vandps	%ymm2, %ymm3, %ymm4
.Ltmp8579:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm6
	vcmplt_oqps	%ymm6, %ymm4, %ymm4
.Ltmp8580:
	.loc	29 82 19
	vandnps	%ymm3, %ymm4, %ymm3
.Ltmp8581:
	.loc	1 1660 5
	vmovaps	%ymm3, 6656(%rsp)
.Ltmp8582:
	.loc	1 1130 16
	leaq	(,%r15,8), %rdi
.Ltmp8583:
	.loc	1 1131 25
	leaq	8(,%r15,8), %rsi
.Ltmp8584:
	.loc	4 1050 16
	leaq	7(,%r15,8), %r15
	cmpq	1504(%rsp), %r15
	jae	.LBB34_645
.Ltmp8585:
	.loc	4 0 16 is_stmt 0
	movq	%r8, 544(%rsp)
	movq	2208(%rsp), %r8
	leaq	(%r8,%rcx), %r15
.Ltmp8586:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm3, %ymm12, %ymm3
	movq	1952(%rsp), %r11
.Ltmp8587:
	.loc	12 551 14
	vmovups	(%r11,%rdi,4), %ymm4
.Ltmp8588:
	.loc	12 551 14 is_stmt 0
	vmovups	(%r15), %ymm6
	vmovups	%ymm6, (%r11,%rdi,4)
.Ltmp8589:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm4, %ymm3, %ymm3
	vmovaps	256(%rsp), %ymm6
.Ltmp8590:
	.loc	29 585 19
	vblendvps	%ymm6, %ymm4, %ymm3, %ymm3
.Ltmp8591:
	.loc	12 551 14
	vmovups	%ymm3, (%r15)
	cmpq	%rax, 224(%rsp)
.Ltmp8592:
	.loc	4 1050 16
	ja	.LBB34_646
.Ltmp8593:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, 320(%rsp)
	movq	%r13, 128(%rsp)
	vmovaps	2240(%rsp), %ymm3
	vblendvps	%ymm3, %ymm1, %ymm0, %ymm0
.Ltmp8594:
	.loc	29 360 14 is_stmt 1
	vdivps	%ymm0, %ymm14, %ymm1
	movq	1312(%rsp), %r8
.Ltmp8595:
	.loc	1 0 0 is_stmt 0
	leaq	(%r12,%r8), %r13
.Ltmp8596:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm14, %ymm0, %ymm0
.Ltmp8597:
	.loc	29 585 19
	vblendvps	%ymm0, %ymm1, %ymm12, %ymm0
	movq	544(%rsp), %rsi
.Ltmp8598:
	.loc	12 551 14
	vmovups	%ymm0, (%r9,%rsi,4)
.Ltmp8599:
	.loc	1 1130 16
	leaq	(,%r13,8), %r15
.Ltmp8600:
	.loc	4 1050 16
	leaq	7(,%r13,8), %r11
	cmpq	%rax, %r11
	jae	.LBB34_647
.Ltmp8601:
	.loc	4 0 16 is_stmt 0
	movq	%r14, %r8
.Ltmp8602:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r9,%r15,4), %ymm0
	vmovaps	%ymm0, %ymm10
.Ltmp8603:
	.loc	1 1206 22
	cmpl	$0, 32(%rsp)
	je	.LBB34_533
.Ltmp8604:
	.loc	29 257 14
	vminps	%ymm0, %ymm13, %ymm10
.Ltmp8605:
.LBB34_533:
	.loc	29 0 14 is_stmt 0
	movq	32(%rsp), %r14
	movl	%r14d, %r14d
.Ltmp8606:
	.loc	1 1212 20 is_stmt 1
	incq	%r14
	movq	1920(%rsp), %r11
	movq	%r11, %r15
	movq	%r14, 32(%rsp)
	cmpq	%r11, %r14
	movq	%r8, %r14
.Ltmp8607:
	.loc	1 1213 22
	jne	.LBB34_534
	.loc	1 0 22 is_stmt 0
.Ltmp8608:
	.p2align	4
.LBB34_536:
.Ltmp8609:
	.loc	1 1130 16 is_stmt 1
	leaq	(,%r13,8), %r10
.Ltmp8610:
	.loc	4 1050 16
	leaq	7(,%r13,8), %r11
	cmpq	%rax, %r11
	jae	.LBB34_648
.Ltmp8611:
	.loc	29 257 14
	vminps	(%r9,%r10,4), %ymm0, %ymm0
.Ltmp8612:
	.loc	12 551 14
	vmovups	%ymm0, (%r9,%r10,4)
.Ltmp8613:
	.loc	1 1224 16
	testq	%r13, %r13
	cmoveq	%r14, %r13
	.loc	1 1227 13
	decq	%r13
.Ltmp8614:
	.loc	10 1916 50
	decq	%r15
.Ltmp8615:
	.loc	3 900 12
	jne	.LBB34_536
.Ltmp8616:
	.loc	3 0 12 is_stmt 0
	movq	$0, 32(%rsp)
	vmovaps	%ymm10, %ymm0
	jmp	.LBB34_539
	.p2align	4
.LBB34_534:
.Ltmp8617:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r10,8), %r11
.Ltmp8618:
	.loc	1 1130 16
	shlq	$3, %r10
.Ltmp8619:
	.loc	4 1050 16
	cmpq	%rax, %r11
	jae	.LBB34_648
.Ltmp8620:
	.loc	12 551 14
	vmovups	(%r9,%r10,4), %ymm0
.Ltmp8621:
	.loc	29 257 14
	vminps	%ymm10, %ymm0, %ymm0
.Ltmp8622:
.LBB34_539:
	.loc	29 0 14 is_stmt 0
	movq	1280(%rsp), %r8
	leaq	(%r12,%r8), %r10
.Ltmp8623:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r10,8), %r11
.Ltmp8624:
	.loc	1 1130 16
	shlq	$3, %r10
	movq	1472(%rsp), %r15
.Ltmp8625:
	.loc	4 1050 16
	cmpq	%r15, %r11
	movq	%r15, %r11
	movq	424(%rsp), %r15
	movq	2648(%rsp), %r8
	movq	224(%rsp), %rsi
	jae	.LBB34_540
.Ltmp8626:
	.loc	4 0 16 is_stmt 0
	cmpq	%r11, %rsi
.Ltmp8627:
	.loc	4 1050 16
	ja	.LBB34_649
.Ltmp8628:
	.loc	1 0 0
	vbroadcastss	.LCPI34_3(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm0
	vbroadcastss	.LCPI34_4(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp8629:
	vaddps	352(%rsp), %ymm0, %ymm1
	movq	1824(%rsp), %r11
	vsubps	(%r11,%r10,4), %ymm1, %ymm1
	vmovaps	%ymm1, 352(%rsp)
.Ltmp8630:
	.loc	29 360 14 is_stmt 1
	vdivps	1856(%rsp), %ymm1, %ymm1
	movq	544(%rsp), %rsi
.Ltmp8631:
	.loc	12 551 14
	vmovups	%ymm0, (%r11,%rsi,4)
.Ltmp8632:
	.loc	1 1659 43
	vmovaps	7392(%rsp), %ymm0
.Ltmp8633:
	.loc	29 347 14
	vsubps	%ymm1, %ymm12, %ymm1
.Ltmp8634:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm0, %ymm1, %ymm3
.Ltmp8635:
	.loc	29 283 14 is_stmt 1
	vmulps	1888(%rsp), %ymm3, %ymm3
.Ltmp8636:
	.loc	29 48 14
	vaddps	%ymm3, %ymm0, %ymm0
.Ltmp8637:
	.loc	29 233 14
	vmaxps	%ymm0, %ymm1, %ymm0
.Ltmp8638:
	.loc	29 82 19
	vandps	%ymm2, %ymm0, %ymm1
.Ltmp8639:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm2
	vcmplt_oqps	%ymm2, %ymm1, %ymm1
.Ltmp8640:
	.loc	29 82 19
	vandnps	%ymm0, %ymm1, %ymm0
.Ltmp8641:
	.loc	1 1660 5
	vmovaps	%ymm0, 7392(%rsp)
	movq	320(%rsp), %rsi
	cmpq	1440(%rsp), %rsi
.Ltmp8642:
	.loc	4 1050 16
	ja	.LBB34_650
.Ltmp8643:
	.loc	1 0 0 is_stmt 0
	addq	2144(%rsp), %rcx
	incq	%r12
.Ltmp8644:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm0, %ymm12, %ymm0
	movq	1792(%rsp), %rsi
.Ltmp8645:
	.loc	12 551 14
	vmovups	(%rsi,%rdi,4), %ymm1
.Ltmp8646:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rcx), %ymm2
	vmovups	%ymm2, (%rsi,%rdi,4)
.Ltmp8647:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm1, %ymm0, %ymm0
	vmovaps	256(%rsp), %ymm2
.Ltmp8648:
	.loc	29 585 19
	vblendvps	%ymm2, %ymm1, %ymm0, %ymm0
.Ltmp8649:
	.loc	12 551 14
	vmovups	%ymm0, (%rcx)
	vmovaps	%ymm15, %ymm2
	vmovaps	%ymm10, %ymm13
.Ltmp8650:
	.loc	11 304 12
	cmpq	2112(%rsp), %r12
	jne	.LBB34_513
	jmp	.LBB34_545
.Ltmp8651:
.LBB34_511:
	.loc	11 0 12 is_stmt 0
	movq	424(%rsp), %r15
	movq	2648(%rsp), %r8
.LBB34_545:
	movq	1184(%rsp), %rdi
	.loc	1 2056 39 is_stmt 1
	addq	%rdi, %r15
.Ltmp8652:
	.loc	1 1148 8
	cmpq	%r14, %r15
	movq	%r14, %rcx
	movl	$0, %r11d
	cmovbq	%r11, %rcx
	subq	%rcx, %r15
	movq	%r15, 424(%rsp)
	movq	992(%rsp), %r13
.Ltmp8653:
	.loc	1 2057 39
	addq	%rdi, %r13
	movq	4200(%rsp), %rcx
.Ltmp8654:
	.loc	1 1148 8
	cmpq	%rcx, %r13
	cmovbq	%r11, %rcx
	subq	%rcx, %r13
	movq	1408(%rsp), %rsi
	movq	%rsi, %r10
	movq	2640(%rsp), %rcx
.Ltmp8655:
	.loc	1 1984 19
	cmpq	%rcx, %rsi
	movq	440(%rsp), %r12
	vmovaps	%ymm11, %ymm13
	movq	1152(%rsp), %r15
	movq	1376(%rsp), %rdi
	movq	1728(%rsp), %rsi
	jb	.LBB34_504
.Ltmp8656:
	.loc	1 0 19 is_stmt 0
	vmovaps	%ymm7, 6688(%rsp)
	vmovaps	352(%rsp), %ymm0
	vmovaps	%ymm0, 7424(%rsp)
	movq	32(%rsp), %rax
	movl	%eax, 28(%rsp)
	movq	128(%rsp), %rax
	movl	%eax, 24(%rsp)
	vmovaps	%ymm10, 1088(%rsp)
	vmovaps	%ymm15, 1056(%rsp)
	movq	184(%rsp), %rbx
	movq	312(%rsp), %r10
	movq	1696(%rsp), %r9
	movq	1664(%rsp), %r11
	vmovaps	5696(%rsp), %ymm12
	vmovaps	5792(%rsp), %ymm11
	vmovaps	5728(%rsp), %ymm4
	vmovaps	384(%rsp), %ymm6
	vmovaps	480(%rsp), %ymm10
	vmovaps	1344(%rsp), %ymm9
	vmovaps	1120(%rsp), %ymm15
	jmp	.LBB34_485
.LBB34_400:
	leaq	4224(%rsp), %rdi
	leaq	1648(%r12), %rbx
.Ltmp8657:
	.loc	1 1943 24 is_stmt 1
	movq	%rbx, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	4960(%rsp), %rdi
	leaq	1848(%r12), %r15
.Ltmp8658:
	.loc	1 1944 25
	movq	%r15, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp8659:
	.loc	1 1949 19
	movzbl	1536(%r12), %eax
	movb	%al, 384(%rsp)
.Ltmp8660:
	.loc	1 1950 21
	movzbl	1537(%r12), %eax
	movb	%al, 32(%rsp)
.Ltmp8661:
	.loc	1 1951 16
	movq	1624(%r12), %r14
.Ltmp8662:
	.loc	1 1952 16
	movq	1632(%r12), %r13
.Ltmp8663:
	.loc	1 1953 27
	movl	1640(%r12), %eax
	movq	%rax, 2432(%rsp)
.Ltmp8664:
	.loc	1 1954 27
	movl	1644(%r12), %eax
	movq	%rax, 320(%rsp)
	leaq	10216(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r12
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r12
	leaq	9184(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r12
	movq	440(%rsp), %r12
	leaq	800(%rsp), %rdi
.Ltmp8665:
	.loc	1 1961 32
	movq	%rbx, %rsi
	movq	%r14, 224(%rsp)
	movq	%r14, %rdx
	movq	%r13, 704(%rsp)
	movq	%r13, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
.Ltmp8666:
	.loc	1 1962 33
	movq	1624(%r12), %rdx
	movq	1632(%r12), %rcx
	leaq	576(%rsp), %rdi
	movq	%r15, %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	312(%rsp), %rdx
.Ltmp8667:
	.loc	4 3758 16
	leaq	31(%rdx), %r8
	shrq	$5, %r8
.Ltmp8668:
	.loc	8 446 20
	je	.LBB34_401
.Ltmp8669:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp8670:
	.loc	29 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 2240(%rsp)
	vmovaps	%ymm1, 544(%rsp)
	cmpb	$0, 384(%rsp)
	jne	.LBB34_413
.Ltmp8671:
	.loc	29 0 19 is_stmt 0
	vmovaps	%ymm0, 544(%rsp)
.LBB34_413:
	movabsq	$2305843009213693944, %rax
	cmpb	$0, 32(%rsp)
	movq	320(%rsp), %r15
	jne	.LBB34_415
	vmovaps	%ymm0, 2240(%rsp)
.LBB34_415:
	movl	$32, %r9d
	addq	$7, %rax
	movq	%rax, 424(%rsp)
	vxorps	%xmm2, %xmm2, %xmm2
	movq	432(%rsp), %r10
	movq	536(%rsp), %r11
	movq	%rdx, %r13
	xorl	%ecx, %ecx
	movq	184(%rsp), %rbx
.Ltmp8672:
	.loc	8 446 20 is_stmt 1
	jmp	.LBB34_418
.Ltmp8673:
.LBB34_14:
	.loc	1 2207 37
	movq	1992(%r12), %rsi
	movq	2000(%r12), %r8
.Ltmp8674:
	.loc	6 314 17
	shlq	$4, %r8
	.loc	6 0 17 is_stmt 0
.Ltmp8675:
	.p2align	4
.LBB34_15:
.Ltmp8676:
	.loc	7 1714 9 is_stmt 1
	testq	%r8, %r8
.Ltmp8677:
	.loc	6 180 28
	je	.LBB34_18
.Ltmp8678:
	.loc	1 690 21
	cmpl	$0, 12(%rsi)
.Ltmp8679:
	.loc	6 315 25
	jne	.LBB34_30
	addq	$-16, %r8
	movl	4(%rsi), %edi
	cmpl	%edi, (%rsi)
	.loc	6 0 0 is_stmt 0
	leaq	16(%rsi), %rsi
	.loc	6 315 25
	je	.LBB34_15
	jmp	.LBB34_30
.Ltmp8680:
.LBB34_97:
	.loc	6 0 25
	vmovaps	320(%rsp), %ymm0
.Ltmp8681:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8682:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8683:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8684:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8685:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8686:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8687:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8688:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_98
.Ltmp8689:
.LBB34_158:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp8690:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8691:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8692:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8693:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8694:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8695:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8696:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8697:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_98:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8698:
.LBB34_99:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8699:
.LBB34_18:
	cmpq	%rdx, %r9
.Ltmp8700:
	.loc	4 1050 16 is_stmt 1
	ja	.LBB34_622
.Ltmp8701:
	.loc	4 0 16 is_stmt 0
	movq	%r9, %r11
	movq	%rbx, %rsi
	.p2align	4
.LBB34_20:
.Ltmp8702:
	.loc	15 1504 12 is_stmt 1
	testq	%r11, %r11
	je	.LBB34_24
.Ltmp8703:
	.loc	10 1078 5
	cmpq	$32, %r11
	movl	$32, %edi
	cmovbq	%r11, %rdi
.Ltmp8704:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp8705:
	.loc	16 0 18 is_stmt 0
.Ltmp8706:
	.p2align	4
.LBB34_22:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp8707:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp8708:
	.loc	6 180 28
	jne	.LBB34_22
.Ltmp8709:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp8710:
	.loc	19 2054 74
	subq	%rdi, %r11
.Ltmp8711:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB34_20
	jmp	.LBB34_30
.Ltmp8712:
.LBB34_615:
	.loc	5 438 16
	cmpq	88(%rsp), %rsi
	ja	.LBB34_655
.Ltmp8713:
	.loc	5 0 16 is_stmt 0
	movq	432(%rsp), %rax
	.p2align	4
.LBB34_617:
.Ltmp8714:
	.loc	17 131 18 is_stmt 1
	movq	%rsi, %rcx
.Ltmp8715:
	.loc	15 1504 12
	testq	%rsi, %rsi
	je	.LBB34_621
.Ltmp8716:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp8717:
	.loc	16 961 18
	leal	(,%rdx,4), %edi
	xorl	%r9d, %r9d
	xorl	%r8d, %r8d
.Ltmp8718:
	.loc	16 0 18 is_stmt 0
.Ltmp8719:
	.p2align	4
.LBB34_619:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r8), %r9d
.Ltmp8720:
	.loc	7 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp8721:
	.loc	6 180 28
	jne	.LBB34_619
.Ltmp8722:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp8723:
	.loc	19 2054 74
	movq	%rcx, %rsi
	subq	%rdx, %rsi
.Ltmp8724:
	.loc	17 136 12
	testl	%r9d, %r9d
	je	.LBB34_617
.Ltmp8725:
.LBB34_621:
	.loc	15 1504 12
	testq	%rcx, %rcx
	sete	%cl
	jmp	.LBB34_559
.Ltmp8726:
.LBB34_478:
	.loc	15 0 12 is_stmt 0
	movq	184(%rsp), %rbx
.LBB34_547:
.Ltmp8727:
	.loc	1 2062 13 is_stmt 1
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 7584(%rsp)
	.loc	1 2063 13
	movl	904(%rsp), %ecx
	.loc	1 2064 13
	vmovaps	576(%rsp), %ymm0
	vmovaps	%ymm0, 7616(%rsp)
.Ltmp8728:
	.loc	1 2072 23
	movq	1736(%r12), %rdx
.Ltmp8729:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp8730:
	.loc	1 0 0 is_stmt 0
	movl	680(%rsp), %eax
.Ltmp8731:
	.loc	1 2072 23 is_stmt 1
	movq	1728(%r12), %rdx
.Ltmp8732:
	.loc	12 551 14
	vmovaps	7584(%rsp), %ymm0
	vmovups	%ymm0, (%rdx)
.Ltmp8733:
	.loc	1 2073 5
	movq	1768(%r12), %rdx
.Ltmp8734:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp8735:
	.loc	6 180 28
	je	.LBB34_551
.Ltmp8736:
	.loc	6 0 28 is_stmt 0
	movq	1760(%r12), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB34_550:
.Ltmp8737:
	.loc	20 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp8738:
	.loc	7 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp8739:
	.loc	6 180 28
	jne	.LBB34_550
.Ltmp8740:
.LBB34_551:
	.loc	1 2074 24
	movq	1936(%r12), %rdx
.Ltmp8741:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp8742:
	.loc	1 2074 24
	movq	1928(%r12), %rcx
.Ltmp8743:
	.loc	12 551 14
	vmovaps	7616(%rsp), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp8744:
	.loc	1 2075 5
	movq	1968(%r12), %rcx
.Ltmp8745:
	.loc	7 1714 9
	testq	%rcx, %rcx
.Ltmp8746:
	.loc	6 180 28
	je	.LBB34_555
.Ltmp8747:
	.loc	6 0 28 is_stmt 0
	movq	1960(%r12), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB34_554:
.Ltmp8748:
	.loc	20 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp8749:
	.loc	7 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp8750:
	.loc	6 180 28
	jne	.LBB34_554
.Ltmp8751:
.LBB34_555:
	.loc	6 0 28 is_stmt 0
	leaq	6016(%rsp), %rdi
	movq	760(%rsp), %rsi
	.loc	1 2077 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	6752(%rsp), %rdi
	movq	752(%rsp), %rsi
	.loc	1 2078 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	992(%rsp), %rax
	.loc	1 2079 5
	movl	%eax, 1640(%r12)
	movq	424(%rsp), %rax
.Ltmp8752:
	.loc	1 0 0 is_stmt 0
	movl	%eax, 1644(%r12)
.Ltmp8753:
	.loc	1 2255 35 is_stmt 1
	cmpb	$0, 2556(%rsp)
	jne	.LBB34_607
	jmp	.LBB34_558
.LBB34_401:
	.loc	1 0 35 is_stmt 0
	movq	184(%rsp), %rbx
	movq	320(%rsp), %r15
.LBB34_402:
.Ltmp8754:
	.loc	1 2062 13 is_stmt 1
	vmovaps	800(%rsp), %ymm0
	vmovaps	%ymm0, 7648(%rsp)
	.loc	1 2063 13
	movl	904(%rsp), %ecx
	.loc	1 2064 13
	vmovaps	576(%rsp), %ymm0
	vmovaps	%ymm0, 7680(%rsp)
.Ltmp8755:
	.loc	1 2072 23
	movq	1736(%r12), %rdx
.Ltmp8756:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp8757:
	.loc	1 0 0 is_stmt 0
	movl	680(%rsp), %eax
.Ltmp8758:
	.loc	1 2072 23 is_stmt 1
	movq	1728(%r12), %rdx
.Ltmp8759:
	.loc	12 551 14
	vmovaps	7648(%rsp), %ymm0
	vmovups	%ymm0, (%rdx)
.Ltmp8760:
	.loc	1 2073 5
	movq	1768(%r12), %rdx
.Ltmp8761:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp8762:
	.loc	6 180 28
	je	.LBB34_406
.Ltmp8763:
	.loc	6 0 28 is_stmt 0
	movq	1760(%r12), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB34_405:
.Ltmp8764:
	.loc	20 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp8765:
	.loc	7 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp8766:
	.loc	6 180 28
	jne	.LBB34_405
.Ltmp8767:
.LBB34_406:
	.loc	1 2074 24
	movq	1936(%r12), %rdx
.Ltmp8768:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB34_99
.Ltmp8769:
	.loc	1 2074 24
	movq	1928(%r12), %rcx
.Ltmp8770:
	.loc	12 551 14
	vmovaps	7680(%rsp), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp8771:
	.loc	1 2075 5
	movq	1968(%r12), %rcx
.Ltmp8772:
	.loc	7 1714 9
	testq	%rcx, %rcx
.Ltmp8773:
	.loc	6 180 28
	je	.LBB34_410
.Ltmp8774:
	.loc	6 0 28 is_stmt 0
	movq	1960(%r12), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB34_409:
.Ltmp8775:
	.loc	20 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp8776:
	.loc	7 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp8777:
	.loc	6 180 28
	jne	.LBB34_409
.Ltmp8778:
.LBB34_410:
	.loc	6 0 28 is_stmt 0
	leaq	4224(%rsp), %rdi
	movq	760(%rsp), %rsi
	.loc	1 2077 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	4960(%rsp), %rdi
	movq	752(%rsp), %rsi
	.loc	1 2078 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	2432(%rsp), %rax
	.loc	1 2079 5
	movl	%eax, 1640(%r12)
	.loc	1 2080 5
	movl	%r15d, 1644(%r12)
.Ltmp8779:
	.loc	1 2255 35
	cmpb	$0, 2556(%rsp)
	jne	.LBB34_607
.LBB34_558:
	.loc	1 0 35 is_stmt 0
	xorl	%ecx, %ecx
.LBB34_559:
	movq	1048(%rsp), %rdx
	movabsq	$2305843009213693944, %rax
	.loc	1 2255 9 is_stmt 1
	movb	%cl, 2152(%r12)
	.loc	1 2260 30
	movzbl	2144(%r12), %ecx
	.loc	1 2260 9 is_stmt 0
	movb	%cl, 2153(%r12)
	.loc	1 2261 21 is_stmt 1
	movq	16(%rdx), %rcx
	movq	%rcx, 9200(%rsp)
	vmovups	(%rdx), %xmm0
	vmovaps	%xmm0, 9184(%rsp)
.Ltmp8780:
	.loc	29 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
.Ltmp8781:
	.loc	19 2155 12
	movq	%rbx, %rcx
	vmovaps	%ymm0, %ymm1
	andq	%rax, %rcx
	movq	536(%rsp), %r8
	je	.LBB34_562
.Ltmp8782:
	.loc	19 0 12 is_stmt 0
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	xorl	%edx, %edx
	vbroadcastss	.LCPI34_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB34_561:
.Ltmp8783:
	.loc	29 82 19 is_stmt 1
	vandps	(%r8,%rdx,4), %ymm2, %ymm4
.Ltmp8784:
	.loc	29 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp8785:
	.loc	29 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp8786:
	.loc	19 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB34_561
.Ltmp8787:
.LBB34_562:
	.loc	1 0 0 is_stmt 0
	movl	2120(%r12), %ebx
.Ltmp8788:
	.loc	1 2268 0 is_stmt 1
	movq	1584(%r12), %r11
	movq	1592(%r12), %r14
	movq	1600(%r12), %r9
	movq	1608(%r12), %r10
.Ltmp8789:
	.loc	30 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
	movq	432(%rsp), %rdi
.Ltmp8790:
	.loc	17 208 8
	jae	.LBB34_567
	.loc	17 0 8 is_stmt 0
	movq	88(%rsp), %rdx
	vmovaps	%ymm0, %ymm1
.Ltmp8791:
	.loc	19 2155 12 is_stmt 1
	andq	%rax, %rdx
	je	.LBB34_566
.Ltmp8792:
	.loc	19 0 12 is_stmt 0
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	xorl	%esi, %esi
	vbroadcastss	.LCPI34_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB34_565:
.Ltmp8793:
	.loc	29 82 19 is_stmt 1
	vandps	(%rdi,%rsi,4), %ymm2, %ymm4
.Ltmp8794:
	.loc	29 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp8795:
	.loc	29 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp8796:
	.loc	19 2155 12
	addq	$8, %rsi
	cmpq	%rsi, %rdx
	jne	.LBB34_565
.Ltmp8797:
.LBB34_566:
	.loc	30 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp8798:
	.loc	17 208 34
	jb	.LBB34_606
.LBB34_567:
	.loc	17 0 34 is_stmt 0
	movq	%r11, 448(%rsp)
	movq	%r10, 384(%rsp)
	movq	%r9, 32(%rsp)
	vmovaps	%ymm0, %ymm1
.Ltmp8799:
	.loc	19 2155 12 is_stmt 1
	testq	%rcx, %rcx
.Ltmp8800:
	.loc	19 2155 12 is_stmt 0
	je	.LBB34_570
.Ltmp8801:
	.loc	19 0 12
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	xorl	%edx, %edx
	vbroadcastss	.LCPI34_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB34_569:
.Ltmp8802:
	.loc	29 82 19 is_stmt 1
	vandps	(%r8,%rdx,4), %ymm2, %ymm4
.Ltmp8803:
	.loc	29 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp8804:
	.loc	29 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp8805:
	.loc	19 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB34_569
.Ltmp8806:
.LBB34_570:
	.loc	29 585 19
	vpsrad	$31, %ymm1, %ymm2
	vpbroadcastd	.LCPI34_2(%rip), %ymm1
	vpandn	%ymm1, %ymm2, %ymm2
.Ltmp8807:
	.loc	17 185 12
	vmovd	%xmm2, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	movl	%edx, 96(%rsp)
	vpextrd	$1, %xmm2, %edx
	xorl	%ecx, %ecx
	testl	%edx, %edx
	setne	%cl
	vpextrd	$2, %xmm2, %edx
	addl	%ecx, %ecx
	xorl	%esi, %esi
	testl	%edx, %edx
	setne	%sil
	shll	$2, %esi
	movl	%esi, 192(%rsp)
	vpextrd	$3, %xmm2, %edx
	xorl	%esi, %esi
	testl	%edx, %edx
	setne	%sil
	shll	$3, %esi
	vextracti128	$1, %ymm2, %xmm2
	vmovd	%xmm2, %edx
	xorl	%r8d, %r8d
	testl	%edx, %edx
	setne	%r8b
	shll	$4, %r8d
	movl	%r8d, 544(%rsp)
	vpextrd	$1, %xmm2, %edx
	xorl	%r10d, %r10d
	testl	%edx, %edx
	setne	%r10b
	vpextrd	$2, %xmm2, %edx
	shll	$5, %r10d
	xorl	%r9d, %r9d
	testl	%edx, %edx
	setne	%r9b
	shll	$6, %r9d
	vpextrd	$3, %xmm2, %edx
	xorl	%r11d, %r11d
	testl	%edx, %edx
	setne	%r11b
	shll	$7, %r11d
.Ltmp8808:
	.loc	19 2155 12
	andq	88(%rsp), %rax
	movl	%ebx, 224(%rsp)
	movq	%r14, 480(%rsp)
	je	.LBB34_573
.Ltmp8809:
	.loc	19 0 12 is_stmt 0
	xorl	%edx, %edx
	vbroadcastss	.LCPI34_0(%rip), %ymm2
	vbroadcastss	.LCPI34_6(%rip), %ymm3
	.p2align	4
.LBB34_572:
.Ltmp8810:
	.loc	29 82 19 is_stmt 1
	vandps	(%rdi,%rdx,4), %ymm2, %ymm4
.Ltmp8811:
	.loc	29 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp8812:
	.loc	29 82 19
	vandps	%ymm4, %ymm0, %ymm0
.Ltmp8813:
	.loc	19 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rax
	jne	.LBB34_572
.Ltmp8814:
.LBB34_573:
	.loc	29 585 19
	vpsrad	$31, %ymm0, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
.Ltmp8815:
	.loc	17 185 12
	vmovd	%xmm0, %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	vpextrd	$1, %xmm0, %edx
	setne	%al
	xorl	%ebx, %ebx
	testl	%edx, %edx
	setne	%bl
	addl	%ebx, %ebx
	vpextrd	$2, %xmm0, %edx
	xorl	%r15d, %r15d
	testl	%edx, %edx
	setne	%r15b
	vpextrd	$3, %xmm0, %edx
	shll	$2, %r15d
	xorl	%r12d, %r12d
	testl	%edx, %edx
	setne	%r12b
	shll	$3, %r12d
	vextracti128	$1, %ymm0, %xmm0
	vmovd	%xmm0, %edx
	xorl	%r13d, %r13d
	testl	%edx, %edx
	setne	%r13b
	vpextrd	$1, %xmm0, %edx
	shll	$4, %r13d
	xorl	%r14d, %r14d
	testl	%edx, %edx
	setne	%r14b
	shll	$5, %r14d
	vpextrd	$2, %xmm0, %edx
	xorl	%edi, %edi
	testl	%edx, %edx
	setne	%dil
	vpextrd	$3, %xmm0, %r8d
	shll	$6, %edi
	xorl	%edx, %edx
	testl	%r8d, %r8d
	setne	%dl
	shll	$7, %edx
.Ltmp8816:
	.loc	17 185 12 is_stmt 0
	orl	%edi, %edx
.Ltmp8817:
	.loc	17 185 12
	orl	96(%rsp), %ecx
	orl	192(%rsp), %ecx
	orl	544(%rsp), %esi
	orl	%r10d, %esi
	orl	%ecx, %esi
	orl	%r11d, %r9d
	orl	%esi, %r9d
.Ltmp8818:
	.loc	17 185 12
	orl	%eax, %r9d
	orl	%ebx, %r9d
	orl	%r15d, %r12d
	orl	%r9d, %r12d
	orl	%r13d, %r14d
	orl	%r12d, %r14d
.Ltmp8819:
	.loc	17 211 5 is_stmt 1
	orl	%edx, %r14d
	movq	440(%rsp), %r12
	movl	%r14d, 1576(%r12)
	.loc	17 212 31
	movq	1568(%r12), %rax
.Ltmp8820:
	.loc	4 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp8821:
	.loc	17 212 5
	movq	%rcx, 1568(%r12)
	movq	184(%rsp), %rdx
.Ltmp8822:
	.loc	7 1714 9
	testq	%rdx, %rdx
.Ltmp8823:
	.loc	6 180 28
	je	.LBB34_575
.Ltmp8824:
	.loc	16 961 18
	shlq	$2, %rdx
	movq	536(%rsp), %rdi
.Ltmp8825:
	.loc	20 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp8826:
.LBB34_575:
	.loc	20 0 13 is_stmt 0
	movq	88(%rsp), %rdx
.Ltmp8827:
	.loc	7 1714 9 is_stmt 1
	testq	%rdx, %rdx
	movq	480(%rsp), %r14
.Ltmp8828:
	.loc	6 180 28
	je	.LBB34_577
.Ltmp8829:
	.loc	16 961 18
	shlq	$2, %rdx
	movq	432(%rsp), %rdi
.Ltmp8830:
	.loc	20 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp8831:
.LBB34_577:
	.loc	1 2269 18
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %rbx
	leaq	9184(%rsp), %r15
	movq	760(%rsp), %rdi
	movq	%r15, %rsi
	movq	448(%rsp), %rdx
	movq	%r14, %rcx
	movl	224(%rsp), %r14d
	movl	%r14d, %r8d
	vzeroupper
	callq	*%rbx
	movq	752(%rsp), %rdi
	.loc	1 2270 19
	movq	%r15, %rsi
	movq	32(%rsp), %rdx
	movq	384(%rsp), %rcx
	movl	%r14d, %r8d
	callq	*%rbx
	.loc	1 2271 13
	movq	$0, 1640(%r12)
.Ltmp8832:
.LBB34_606:
	.loc	1 2273 6
	leaq	-40(%rbp), %rsp
	.loc	1 2273 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	vzeroupper
	retq
.LBB34_306:
	.cfi_def_cfa %rbp, 16
.Ltmp8833:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
.Ltmp8834:
	.loc	5 581 13 is_stmt 0
	movq	%r9, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8835:
.LBB34_515:
	.loc	5 0 13
	vmovaps	1568(%rsp), %ymm0
.Ltmp8836:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp8837:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	%ymm10, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp8838:
	leaq	8(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8839:
.LBB34_24:
	movq	312(%rsp), %rsi
	leaq	(,%rsi,8), %rsi
.Ltmp8840:
	.loc	5 438 16 is_stmt 1
	cmpq	88(%rsp), %rsi
	ja	.LBB34_35
.Ltmp8841:
	.loc	5 0 16 is_stmt 0
	movq	%rsi, %r11
	movq	432(%rsp), %rsi
	.p2align	4
.LBB34_26:
.Ltmp8842:
	.loc	15 1504 12 is_stmt 1
	testq	%r11, %r11
	je	.LBB34_578
.Ltmp8843:
	.loc	10 1078 5
	cmpq	$32, %r11
	movl	$32, %edi
	cmovbq	%r11, %rdi
.Ltmp8844:
	.loc	16 961 18
	leal	(,%rdi,4), %r9d
	xorl	%r8d, %r8d
	xorl	%r10d, %r10d
.Ltmp8845:
	.loc	16 0 18 is_stmt 0
.Ltmp8846:
	.p2align	4
.LBB34_28:
	.loc	17 134 13 is_stmt 1
	orl	(%rsi,%r10), %r8d
.Ltmp8847:
	.loc	7 1714 9
	addq	$4, %r10
	cmpq	%r10, %r9
.Ltmp8848:
	.loc	6 180 28
	jne	.LBB34_28
.Ltmp8849:
	.loc	18 863 18
	leaq	(%rsi,%rdi,4), %rsi
.Ltmp8850:
	.loc	19 2054 74
	subq	%rdi, %r11
.Ltmp8851:
	.loc	17 136 12
	testl	%r8d, %r8d
	je	.LBB34_26
	jmp	.LBB34_30
.Ltmp8852:
.LBB34_648:
	.loc	17 0 12 is_stmt 0
	vmovaps	1568(%rsp), %ymm0
.Ltmp8853:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp8854:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %ecx
	movl	%ecx, 904(%rsp)
	movl	28(%rsp), %ecx
	movl	%ecx, 680(%rsp)
.Ltmp8855:
	.loc	1 1131 25 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp8856:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r10, %rdi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8857:
.LBB34_190:
	.loc	5 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp8858:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8859:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8860:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8861:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8862:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8863:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8864:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8865:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8866:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_422064f3ca430d31d9007f55b436c6ca(%rip), %rcx
	movq	544(%rsp), %rdi
.Ltmp8867:
	.loc	1 0 0 is_stmt 0
	movq	%rbx, %rsi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_389:
.Ltmp8868:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_422064f3ca430d31d9007f55b436c6ca(%rip), %rcx
	movq	96(%rsp), %rdi
.Ltmp8869:
	.loc	1 0 0 is_stmt 0
	movq	%rbx, %rsi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_150:
	vmovaps	320(%rsp), %ymm0
.Ltmp8870:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8871:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8872:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8873:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8874:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8875:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8876:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8877:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8878:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8879:
.LBB34_625:
	.loc	5 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp8880:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8881:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8882:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8883:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8884:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8885:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8886:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8887:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8888:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	%rbx, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8889:
.LBB34_631:
	.loc	5 581 13
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
	movq	%r11, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8890:
.LBB34_159:
	.loc	5 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp8891:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8892:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8893:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8894:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8895:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8896:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8897:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8898:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8899:
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_252:
	vmovaps	320(%rsp), %ymm0
.Ltmp8900:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8901:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8902:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8903:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8904:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8905:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8906:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8907:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8908:
	.loc	5 569 13 is_stmt 1
	leaq	.Lalloc_54f0ebc64763f3f022885a8e201aab88(%rip), %rcx
	movq	544(%rsp), %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8909:
.LBB34_398:
	.loc	5 569 13
	leaq	.Lalloc_54f0ebc64763f3f022885a8e201aab88(%rip), %rcx
	movq	96(%rsp), %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8910:
.LBB34_248:
	.loc	5 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp8911:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8912:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8913:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8914:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8915:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8916:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8917:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8918:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8919:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_56df7c041d29359441bca272bf4e38e3(%rip), %rcx
.Ltmp8920:
	.loc	5 581 13 is_stmt 0
	movq	%r15, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8921:
.LBB34_629:
	.loc	5 0 13
	vmovaps	320(%rsp), %ymm0
.Ltmp8922:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8923:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8924:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8925:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8926:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8927:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8928:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8929:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8930:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
.Ltmp8931:
	.loc	5 581 13 is_stmt 0
	movq	%r15, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8932:
.LBB34_632:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_90498045d73339daaf9e4f537508f58b(%rip), %rcx
.Ltmp8933:
	.loc	5 581 13 is_stmt 0
	movq	%r9, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8934:
.LBB34_250:
	.loc	5 0 13
	vmovaps	320(%rsp), %ymm0
.Ltmp8935:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8936:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8937:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8938:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8939:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8940:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8941:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8942:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8943:
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_396:
	movq	%rcx, %rdi
.LBB34_397:
	leaq	.Lalloc_da8af254b6d507a8e2ca31e544bfd21d(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8944:
.LBB34_578:
	movb	$1, %sil
	.loc	1 2210 12 is_stmt 1
	cmpb	$0, 2152(%r12)
	je	.LBB34_31
.Ltmp8945:
	.loc	1 663 31
	movq	1768(%r12), %rcx
	.loc	1 663 57 is_stmt 0
	movq	1832(%r12), %rax
.Ltmp8946:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp8947:
	.loc	11 304 12
	testq	%rcx, %rcx
	je	.LBB34_586
.Ltmp8948:
	.loc	11 0 12 is_stmt 0
	movq	1760(%r12), %rsi
	movq	1824(%r12), %rdi
	xorl	%r8d, %r8d
	jmp	.LBB34_581
.LBB34_584:
.Ltmp8949:
	.loc	1 665 22 is_stmt 1
	xorl	%edx, %edx
	divq	%r9
.LBB34_585:
	.loc	1 665 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp8950:
	.loc	1 0 0
	incq	%r8
.Ltmp8951:
	.loc	11 304 12 is_stmt 1
	addq	$12, %rdi
	cmpq	%r8, %rcx
	je	.LBB34_586
.Ltmp8952:
.LBB34_581:
	.loc	1 664 26
	movl	(%rdi), %r9d
	testq	%r9, %r9
	movq	312(%rsp), %rax
.Ltmp8953:
	.loc	1 665 42
	je	.LBB34_651
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
	jb	.LBB34_584
	xorl	%edx, %edx
	divl	%r9d
	jmp	.LBB34_585
.Ltmp8954:
.LBB34_586:
	.loc	1 663 31 is_stmt 1
	movq	1968(%r12), %rcx
	.loc	1 663 57 is_stmt 0
	movq	2032(%r12), %rax
.Ltmp8955:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp8956:
	.loc	11 304 12
	testq	%rcx, %rcx
	movq	312(%rsp), %rax
	je	.LBB34_593
.Ltmp8957:
	.loc	11 0 12 is_stmt 0
	movq	1960(%r12), %rsi
	movq	2024(%r12), %rdi
	xorl	%r8d, %r8d
	jmp	.LBB34_588
.LBB34_591:
.Ltmp8958:
	.loc	1 665 22 is_stmt 1
	movq	%rdx, %rax
	xorl	%edx, %edx
	divq	%r9
.LBB34_592:
	.loc	1 665 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp8959:
	.loc	1 0 0
	incq	%r8
.Ltmp8960:
	.loc	11 304 12 is_stmt 1
	addq	$12, %rdi
	cmpq	%r8, %rcx
	movq	312(%rsp), %rax
	je	.LBB34_593
.Ltmp8961:
.LBB34_588:
	.loc	1 664 26
	movl	(%rdi), %r9d
	testq	%r9, %r9
.Ltmp8962:
	.loc	1 665 42
	je	.LBB34_651
	.loc	1 665 24 is_stmt 0
	movl	(%rsi,%r8,4), %r10d
	.loc	1 665 42
	xorl	%edx, %edx
	divl	%r9d
	.loc	1 665 23
	addq	%r10, %rdx
	.loc	1 665 22
	btq	$32, %rdx
	jb	.LBB34_591
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%r9d
	jmp	.LBB34_592
.Ltmp8963:
.LBB34_593:
	.loc	1 2227 26 is_stmt 1
	movq	1632(%r12), %rsi
.Ltmp8964:
	.loc	1 455 44
	testq	%rsi, %rsi
	je	.LBB34_652
.Ltmp8965:
	.loc	1 2227 26
	movq	1624(%r12), %rcx
.Ltmp8966:
	.loc	1 455 23
	movl	1640(%r12), %edi
	.loc	1 455 44 is_stmt 0
	cmpq	%rsi, %rax
	jb	.LBB34_596
	.loc	1 0 44
	movq	312(%rsp), %rax
	.loc	1 455 44
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB34_596:
	.loc	1 455 22
	addq	%rdi, %rax
	.loc	1 455 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB34_597
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB34_599
.Ltmp8967:
.LBB34_623:
	.loc	1 0 0
	leaq	.Lalloc_5cbcacccdbb7b907c55dbc42154fcf08(%rip), %rcx
	movq	%rax, %rdi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_145:
	vmovaps	320(%rsp), %ymm0
.Ltmp8968:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8969:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8970:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8971:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8972:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8973:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8974:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8975:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_146
.Ltmp8976:
.LBB34_245:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp8977:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp8978:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp8979:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp8980:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp8981:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8982:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp8983:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp8984:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_146:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp8985:
.LBB34_147:
	leaq	.Lalloc_91a4c6b9b17ebf4d863f9a70b6dc929a(%rip), %rdx
	movq	%r14, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_416:
	movl	192(%rsp), %eax
	movl	%eax, 904(%rsp)
	vmovaps	%ymm9, 800(%rsp)
	movl	480(%rsp), %eax
	movl	%eax, 680(%rsp)
	vmovaps	%ymm5, 576(%rsp)
	movq	312(%rsp), %rdx
	movq	2368(%rsp), %r8
	movl	$32, %r9d
	movq	2336(%rsp), %r10
	movq	2304(%rsp), %r11
	movq	2272(%rsp), %r13
	movq	2592(%rsp), %rcx
.LBB34_417:
	movq	%r14, 2432(%rsp)
.Ltmp8986:
	addq	$32, %rcx
	decq	%r8
.Ltmp8987:
	.loc	8 446 20 is_stmt 1
	addq	$-32, %r13
	addq	$1024, %r11
	addq	$1024, %r10
	testq	%r8, %r8
	je	.LBB34_402
.LBB34_418:
.Ltmp8988:
	.loc	4 2584 13
	cmpq	$32, %r13
	movl	$32, %eax
	cmovbq	%r13, %rax
	cmpq	$1, %rax
	movq	%rax, 768(%rsp)
	adcq	$0, %rax
.Ltmp8989:
	.loc	1 1967 55
	movq	%rdx, %rsi
	subq	%rcx, %rsi
.Ltmp8990:
	.loc	10 1078 5
	cmpq	$32, %rsi
	cmovaeq	%r9, %rsi
.Ltmp8991:
	.loc	1 1968 31
	leaq	(,%rcx,8), %rdi
	movq	%rsi, 2656(%rsp)
.Ltmp8992:
	.loc	1 1972 39
	addq	%rcx, %rsi
	shlq	$3, %rsi
.Ltmp8993:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB34_489
	cmpq	%rbx, %rsi
	ja	.LBB34_489
.Ltmp8994:
	.loc	1 0 0 is_stmt 0
	shll	$5, %eax
.Ltmp8995:
	.loc	1 1759 23 is_stmt 1
	vmovaps	4224(%rsp), %ymm6
	vmovaps	4256(%rsp), %ymm14
	vmovaps	4288(%rsp), %ymm1
	vmovaps	4320(%rsp), %ymm5
	vmovaps	4352(%rsp), %ymm4
	vmovaps	4384(%rsp), %ymm3
	vmovaps	4416(%rsp), %ymm10
	vmovaps	4448(%rsp), %ymm0
	vmovaps	4480(%rsp), %ymm7
	vmovaps	4512(%rsp), %ymm11
	vmovaps	4544(%rsp), %ymm13
	movq	%rcx, 2592(%rsp)
.Ltmp8996:
	.loc	11 304 12
	cmpq	%rcx, %rdx
	vmovaps	%ymm1, 32(%rsp)
	jne	.LBB34_422
.Ltmp8997:
	.loc	1 0 0 is_stmt 0
	vmovaps	4576(%rsp), %ymm12
	vmovaps	%ymm14, %ymm1
.Ltmp8998:
	.loc	11 304 12
	jmp	.LBB34_424
.Ltmp8999:
.LBB34_422:
	.loc	11 0 12
	vmovaps	(%r12), %ymm8
	vmovaps	%ymm8, 480(%rsp)
	vmovaps	32(%r12), %ymm8
	vmovaps	%ymm8, 96(%rsp)
	vmovaps	64(%r12), %ymm8
	vmovaps	%ymm8, 192(%rsp)
	vmovaps	96(%r12), %ymm8
	vmovaps	%ymm8, 128(%rsp)
	vmovaps	128(%r12), %ymm8
	vmovaps	%ymm8, 256(%rsp)
	vmovaps	160(%r12), %ymm8
	vmovaps	%ymm8, 352(%rsp)
	vmovaps	192(%r12), %ymm8
	vmovaps	%ymm8, 320(%rsp)
	vmovaps	224(%r12), %ymm8
	vmovaps	%ymm8, 992(%rsp)
	vmovaps	256(%r12), %ymm8
	vmovaps	%ymm8, 1536(%rsp)
	vmovaps	288(%r12), %ymm8
	vmovaps	%ymm8, 928(%rsp)
	vmovaps	320(%r12), %ymm8
	vmovaps	%ymm8, 960(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm13, 384(%rsp)
	vmovaps	%ymm11, %ymm8
	vmovaps	%ymm7, %ymm15
	vmovaps	%ymm0, %ymm9
	vmovaps	352(%r12), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	384(%r12), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	416(%r12), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	448(%r12), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	vmovaps	480(%r12), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	512(%r12), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	544(%r12), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	576(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	608(%r12), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	640(%r12), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	672(%r12), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	704(%r12), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	736(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	768(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	800(%r12), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	832(%r12), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	864(%r12), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	896(%r12), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	928(%r12), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	960(%r12), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	992(%r12), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1024(%r12), %ymm0
	vmovaps	%ymm0, 1184(%rsp)
	vmovaps	1056(%r12), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	1088(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1120(%r12), %ymm0
	vmovaps	%ymm0, 1152(%rsp)
	vmovaps	1152(%r12), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1184(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1216(%r12), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	1248(%r12), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1280(%r12), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1312(%r12), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1344(%r12), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	1376(%r12), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	.p2align	4
.LBB34_423:
.Ltmp9000:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r11,%rcx), %ymm1
.Ltmp9001:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm1, %ymm0
.Ltmp9002:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp9003:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm1, %ymm7
.Ltmp9004:
	.loc	29 48 14
	vaddps	%ymm2, %ymm7, %ymm7
.Ltmp9005:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm1, %ymm11
.Ltmp9006:
	.loc	29 48 14
	vaddps	%ymm2, %ymm11, %ymm11
	vmovaps	%ymm6, 448(%rsp)
.Ltmp9007:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm6, %ymm12
.Ltmp9008:
	.loc	29 48 14
	vaddps	%ymm0, %ymm12, %ymm12
.Ltmp9009:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm6, %ymm0
.Ltmp9010:
	.loc	29 48 14
	vaddps	%ymm0, %ymm7, %ymm7
.Ltmp9011:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm6, %ymm0
.Ltmp9012:
	.loc	29 48 14
	vaddps	%ymm0, %ymm11, %ymm11
.Ltmp9013:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm1, %ymm0
.Ltmp9014:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp9015:
	.loc	29 283 14
	vmulps	992(%rsp), %ymm6, %ymm13
.Ltmp9016:
	.loc	29 48 14
	vaddps	%ymm0, %ymm13, %ymm13
	vmovaps	%ymm10, %ymm0
	vmovaps	%ymm4, %ymm6
	vmovaps	%ymm5, %ymm10
	vmovaps	32(%rsp), %ymm5
	vmovaps	%ymm14, %ymm2
.Ltmp9017:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm14, %ymm14
.Ltmp9018:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp9019:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm2, %ymm14
.Ltmp9020:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp9021:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm2, %ymm14
.Ltmp9022:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm11
	vmovaps	%ymm2, 32(%rsp)
.Ltmp9023:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm2, %ymm14
.Ltmp9024:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp9025:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm5, %ymm14
.Ltmp9026:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp9027:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm5, %ymm14
.Ltmp9028:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp9029:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm5, %ymm14
.Ltmp9030:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm11
.Ltmp9031:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm5, %ymm14
.Ltmp9032:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp9033:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm10, %ymm14
.Ltmp9034:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp9035:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm10, %ymm14
.Ltmp9036:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp9037:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm10, %ymm14
.Ltmp9038:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm11
.Ltmp9039:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm10, %ymm14
.Ltmp9040:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp9041:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm4, %ymm14
.Ltmp9042:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp9043:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm4, %ymm14
.Ltmp9044:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp9045:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm4, %ymm14
.Ltmp9046:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm11
.Ltmp9047:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm4, %ymm14
.Ltmp9048:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp9049:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm3, %ymm14
.Ltmp9050:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp9051:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm3, %ymm14
.Ltmp9052:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm7
.Ltmp9053:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm3, %ymm14
.Ltmp9054:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm11
.Ltmp9055:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm3, %ymm14
.Ltmp9056:
	.loc	29 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp9057:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm0, %ymm14
.Ltmp9058:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp9059:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm0, %ymm14
.Ltmp9060:
	.loc	29 48 14
	vaddps	%ymm7, %ymm14, %ymm14
.Ltmp9061:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm0, %ymm7
.Ltmp9062:
	.loc	29 48 14
	vaddps	%ymm7, %ymm11, %ymm11
.Ltmp9063:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm0, %ymm7
.Ltmp9064:
	.loc	29 48 14
	vaddps	%ymm7, %ymm13, %ymm13
	vmovaps	%ymm9, %ymm7
.Ltmp9065:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm9, %ymm9
.Ltmp9066:
	.loc	29 48 14
	vaddps	%ymm9, %ymm12, %ymm9
.Ltmp9067:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm7, %ymm12
.Ltmp9068:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp9069:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm7, %ymm14
.Ltmp9070:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm14
.Ltmp9071:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm7, %ymm11
.Ltmp9072:
	.loc	29 48 14
	vaddps	%ymm11, %ymm13, %ymm13
	vmovaps	%ymm15, %ymm11
.Ltmp9073:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm15, %ymm15
.Ltmp9074:
	.loc	29 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp9075:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm11, %ymm15
.Ltmp9076:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp9077:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm11, %ymm15
.Ltmp9078:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp9079:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm11, %ymm15
.Ltmp9080:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm15
	vmovaps	%ymm8, %ymm13
.Ltmp9081:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm8, %ymm8
.Ltmp9082:
	.loc	29 48 14
	vaddps	%ymm8, %ymm9, %ymm8
.Ltmp9083:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm13, %ymm9
.Ltmp9084:
	.loc	29 48 14
	vaddps	%ymm9, %ymm12, %ymm9
.Ltmp9085:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm13, %ymm12
.Ltmp9086:
	.loc	29 48 14
	vaddps	%ymm12, %ymm14, %ymm14
.Ltmp9087:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm13, %ymm12
.Ltmp9088:
	.loc	29 48 14
	vaddps	%ymm12, %ymm15, %ymm15
	vmovaps	384(%rsp), %ymm12
.Ltmp9089:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm12, %ymm2
.Ltmp9090:
	.loc	29 48 14
	vaddps	%ymm2, %ymm8, %ymm2
.Ltmp9091:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm12, %ymm8
.Ltmp9092:
	.loc	29 48 14
	vaddps	%ymm8, %ymm9, %ymm8
.Ltmp9093:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm12, %ymm9
.Ltmp9094:
	.loc	29 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp9095:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm12, %ymm14
.Ltmp9096:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp9097:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm15
.Ltmp9098:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm2, %ymm15, %ymm2
.Ltmp9099:
	.loc	29 82 19
	vandps	%ymm3, %ymm15, %ymm4
.Ltmp9100:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm2, %ymm4, %ymm2
.Ltmp9101:
	.loc	29 82 19
	vandps	%ymm15, %ymm8, %ymm4
.Ltmp9102:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm2, %ymm2
.Ltmp9103:
	.loc	29 82 19
	vandps	%ymm15, %ymm9, %ymm4
.Ltmp9104:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm2, %ymm2
.Ltmp9105:
	.loc	29 82 19
	vandps	%ymm15, %ymm14, %ymm4
.Ltmp9106:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm2, %ymm2
	vmovaps	%ymm10, %ymm4
	vmovaps	%ymm3, %ymm10
	vmovaps	%ymm6, %ymm3
	vmovaps	%ymm1, %ymm6
	vmovaps	448(%rsp), %ymm1
.Ltmp9107:
	.loc	12 551 14
	vmovups	%ymm2, 10216(%rsp,%rcx)
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp9108:
	.loc	11 304 12
	addq	$32, %rcx
	vmovaps	%ymm1, %ymm14
	vmovaps	%ymm13, 384(%rsp)
	vmovaps	%ymm11, %ymm8
	vmovaps	%ymm7, %ymm15
	vmovaps	%ymm0, %ymm9
	cmpq	%rcx, %rax
	jne	.LBB34_423
.Ltmp9109:
.LBB34_424:
	.loc	1 1764 5
	vmovaps	%ymm6, 4224(%rsp)
	vmovaps	%ymm1, 4256(%rsp)
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, 4288(%rsp)
	vmovaps	%ymm5, 4320(%rsp)
	vmovaps	%ymm4, 4352(%rsp)
	vmovaps	%ymm3, 4384(%rsp)
	vmovaps	%ymm10, 4416(%rsp)
	vmovaps	%ymm0, 4448(%rsp)
	vmovaps	%ymm7, 4480(%rsp)
	vmovaps	%ymm11, 4512(%rsp)
	vmovaps	%ymm13, 4544(%rsp)
	vmovaps	%ymm12, 4576(%rsp)
.Ltmp9110:
	.loc	5 438 16
	cmpq	88(%rsp), %rsi
	ja	.LBB34_496
.Ltmp9111:
	.loc	1 1759 23
	vmovaps	4960(%rsp), %ymm6
	vmovaps	4992(%rsp), %ymm13
	vmovaps	5024(%rsp), %ymm1
	vmovaps	5056(%rsp), %ymm5
	vmovaps	5088(%rsp), %ymm4
	vmovaps	5120(%rsp), %ymm3
	vmovaps	5152(%rsp), %ymm10
	vmovaps	5184(%rsp), %ymm0
	vmovaps	5216(%rsp), %ymm15
	vmovaps	5248(%rsp), %ymm11
	vmovaps	5280(%rsp), %ymm14
.Ltmp9112:
	.loc	11 304 12
	cmpq	2592(%rsp), %rdx
	vmovaps	%ymm1, 32(%rsp)
.Ltmp9113:
	.loc	11 304 12 is_stmt 0
	jne	.LBB34_427
.Ltmp9114:
	.loc	1 0 0
	vmovaps	5312(%rsp), %ymm12
	movq	2432(%rsp), %r14
	vmovaps	%ymm13, %ymm1
.Ltmp9115:
	.loc	11 304 12
	jmp	.LBB34_429
.Ltmp9116:
.LBB34_427:
	.loc	11 0 12
	vmovaps	(%r12), %ymm7
	vmovaps	%ymm7, 480(%rsp)
	vmovaps	32(%r12), %ymm7
	vmovaps	%ymm7, 96(%rsp)
	vmovaps	64(%r12), %ymm7
	vmovaps	%ymm7, 192(%rsp)
	vmovaps	96(%r12), %ymm7
	vmovaps	%ymm7, 128(%rsp)
	vmovaps	128(%r12), %ymm7
	vmovaps	%ymm7, 256(%rsp)
	vmovaps	160(%r12), %ymm7
	vmovaps	%ymm7, 352(%rsp)
	vmovaps	192(%r12), %ymm7
	vmovaps	%ymm7, 320(%rsp)
	vmovaps	224(%r12), %ymm7
	vmovaps	%ymm7, 992(%rsp)
	vmovaps	256(%r12), %ymm7
	vmovaps	%ymm7, 1536(%rsp)
	vmovaps	288(%r12), %ymm7
	vmovaps	%ymm7, 928(%rsp)
	vmovaps	320(%r12), %ymm7
	vmovaps	%ymm7, 960(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm14, 384(%rsp)
	vmovaps	%ymm11, %ymm7
	vmovaps	%ymm15, %ymm8
	vmovaps	%ymm0, %ymm9
	vmovaps	352(%r12), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	vmovaps	384(%r12), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	vmovaps	416(%r12), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	448(%r12), %ymm0
	vmovaps	%ymm0, 1216(%rsp)
	vmovaps	480(%r12), %ymm0
	vmovaps	%ymm0, 2208(%rsp)
	vmovaps	512(%r12), %ymm0
	vmovaps	%ymm0, 2176(%rsp)
	vmovaps	544(%r12), %ymm0
	vmovaps	%ymm0, 2144(%rsp)
	vmovaps	576(%r12), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	608(%r12), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	vmovaps	640(%r12), %ymm0
	vmovaps	%ymm0, 1504(%rsp)
	vmovaps	672(%r12), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	704(%r12), %ymm0
	vmovaps	%ymm0, 2016(%rsp)
	vmovaps	736(%r12), %ymm0
	vmovaps	%ymm0, 1984(%rsp)
	vmovaps	768(%r12), %ymm0
	vmovaps	%ymm0, 1952(%rsp)
	vmovaps	800(%r12), %ymm0
	vmovaps	%ymm0, 1920(%rsp)
	vmovaps	832(%r12), %ymm0
	vmovaps	%ymm0, 1472(%rsp)
	vmovaps	864(%r12), %ymm0
	vmovaps	%ymm0, 1440(%rsp)
	vmovaps	896(%r12), %ymm0
	vmovaps	%ymm0, 1888(%rsp)
	vmovaps	928(%r12), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	960(%r12), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	vmovaps	992(%r12), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	1024(%r12), %ymm0
	vmovaps	%ymm0, 1184(%rsp)
	vmovaps	1056(%r12), %ymm0
	vmovaps	%ymm0, 1408(%rsp)
	vmovaps	1088(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	1120(%r12), %ymm0
	vmovaps	%ymm0, 1152(%rsp)
	vmovaps	1152(%r12), %ymm0
	vmovaps	%ymm0, 1376(%rsp)
	vmovaps	1184(%r12), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1216(%r12), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	1248(%r12), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	1280(%r12), %ymm0
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	1312(%r12), %ymm0
	vmovaps	%ymm0, 2368(%rsp)
	vmovaps	1344(%r12), %ymm0
	vmovaps	%ymm0, 2336(%rsp)
	vmovaps	1376(%r12), %ymm0
	vmovaps	%ymm0, 2304(%rsp)
	vmovaps	1408(%r12), %ymm0
	vmovaps	%ymm0, 2272(%rsp)
	vmovaps	1440(%r12), %ymm0
	vmovaps	%ymm0, 2496(%rsp)
	vmovaps	1472(%r12), %ymm0
	vmovaps	%ymm0, 2464(%rsp)
	vmovaps	1504(%r12), %ymm0
	vmovaps	%ymm0, 2560(%rsp)
	movq	2432(%rsp), %r14
	.p2align	4
.LBB34_428:
.Ltmp9117:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r10,%rcx), %ymm1
.Ltmp9118:
	.loc	29 283 14
	vmulps	480(%rsp), %ymm1, %ymm0
.Ltmp9119:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp9120:
	.loc	29 283 14
	vmulps	96(%rsp), %ymm1, %ymm11
.Ltmp9121:
	.loc	29 48 14
	vaddps	%ymm2, %ymm11, %ymm11
.Ltmp9122:
	.loc	29 283 14
	vmulps	192(%rsp), %ymm1, %ymm12
.Ltmp9123:
	.loc	29 48 14
	vaddps	%ymm2, %ymm12, %ymm12
	vmovaps	%ymm6, 448(%rsp)
.Ltmp9124:
	.loc	29 283 14
	vmulps	256(%rsp), %ymm6, %ymm14
.Ltmp9125:
	.loc	29 48 14
	vaddps	%ymm0, %ymm14, %ymm14
.Ltmp9126:
	.loc	29 283 14
	vmulps	352(%rsp), %ymm6, %ymm0
.Ltmp9127:
	.loc	29 48 14
	vaddps	%ymm0, %ymm11, %ymm11
.Ltmp9128:
	.loc	29 283 14
	vmulps	320(%rsp), %ymm6, %ymm0
.Ltmp9129:
	.loc	29 48 14
	vaddps	%ymm0, %ymm12, %ymm12
.Ltmp9130:
	.loc	29 283 14
	vmulps	128(%rsp), %ymm1, %ymm0
.Ltmp9131:
	.loc	29 48 14
	vaddps	%ymm2, %ymm0, %ymm0
.Ltmp9132:
	.loc	29 283 14
	vmulps	992(%rsp), %ymm6, %ymm15
.Ltmp9133:
	.loc	29 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	%ymm10, %ymm0
	vmovaps	%ymm4, %ymm6
	vmovaps	%ymm5, %ymm10
	vmovaps	32(%rsp), %ymm5
	vmovaps	%ymm13, %ymm2
.Ltmp9134:
	.loc	29 283 14
	vmulps	1536(%rsp), %ymm13, %ymm13
.Ltmp9135:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp9136:
	.loc	29 283 14
	vmulps	928(%rsp), %ymm2, %ymm14
.Ltmp9137:
	.loc	29 48 14
	vaddps	%ymm14, %ymm11, %ymm11
.Ltmp9138:
	.loc	29 283 14
	vmulps	960(%rsp), %ymm2, %ymm14
.Ltmp9139:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
	vmovaps	%ymm2, 32(%rsp)
.Ltmp9140:
	.loc	29 283 14
	vmulps	1312(%rsp), %ymm2, %ymm14
.Ltmp9141:
	.loc	29 48 14
	vaddps	%ymm14, %ymm15, %ymm14
.Ltmp9142:
	.loc	29 283 14
	vmulps	1280(%rsp), %ymm5, %ymm15
.Ltmp9143:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp9144:
	.loc	29 283 14
	vmulps	1248(%rsp), %ymm5, %ymm15
.Ltmp9145:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp9146:
	.loc	29 283 14
	vmulps	1216(%rsp), %ymm5, %ymm15
.Ltmp9147:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp9148:
	.loc	29 283 14
	vmulps	2208(%rsp), %ymm5, %ymm15
.Ltmp9149:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp9150:
	.loc	29 283 14
	vmulps	2176(%rsp), %ymm10, %ymm15
.Ltmp9151:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp9152:
	.loc	29 283 14
	vmulps	2144(%rsp), %ymm10, %ymm15
.Ltmp9153:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp9154:
	.loc	29 283 14
	vmulps	2112(%rsp), %ymm10, %ymm15
.Ltmp9155:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp9156:
	.loc	29 283 14
	vmulps	2080(%rsp), %ymm10, %ymm15
.Ltmp9157:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp9158:
	.loc	29 283 14
	vmulps	1504(%rsp), %ymm4, %ymm15
.Ltmp9159:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp9160:
	.loc	29 283 14
	vmulps	2048(%rsp), %ymm4, %ymm15
.Ltmp9161:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp9162:
	.loc	29 283 14
	vmulps	2016(%rsp), %ymm4, %ymm15
.Ltmp9163:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp9164:
	.loc	29 283 14
	vmulps	1984(%rsp), %ymm4, %ymm15
.Ltmp9165:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp9166:
	.loc	29 283 14
	vmulps	1952(%rsp), %ymm3, %ymm15
.Ltmp9167:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp9168:
	.loc	29 283 14
	vmulps	1920(%rsp), %ymm3, %ymm15
.Ltmp9169:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp9170:
	.loc	29 283 14
	vmulps	1472(%rsp), %ymm3, %ymm15
.Ltmp9171:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp9172:
	.loc	29 283 14
	vmulps	1440(%rsp), %ymm3, %ymm15
.Ltmp9173:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
.Ltmp9174:
	.loc	29 283 14
	vmulps	1888(%rsp), %ymm0, %ymm15
.Ltmp9175:
	.loc	29 48 14
	vaddps	%ymm15, %ymm13, %ymm13
.Ltmp9176:
	.loc	29 283 14
	vmulps	1856(%rsp), %ymm0, %ymm15
.Ltmp9177:
	.loc	29 48 14
	vaddps	%ymm15, %ymm11, %ymm11
.Ltmp9178:
	.loc	29 283 14
	vmulps	1824(%rsp), %ymm0, %ymm15
.Ltmp9179:
	.loc	29 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp9180:
	.loc	29 283 14
	vmulps	1792(%rsp), %ymm0, %ymm15
.Ltmp9181:
	.loc	29 48 14
	vaddps	%ymm15, %ymm14, %ymm14
	vmovaps	%ymm9, %ymm15
.Ltmp9182:
	.loc	29 283 14
	vmulps	1184(%rsp), %ymm9, %ymm9
.Ltmp9183:
	.loc	29 48 14
	vaddps	%ymm9, %ymm13, %ymm9
.Ltmp9184:
	.loc	29 283 14
	vmulps	1408(%rsp), %ymm15, %ymm13
.Ltmp9185:
	.loc	29 48 14
	vaddps	%ymm13, %ymm11, %ymm13
.Ltmp9186:
	.loc	29 283 14
	vmulps	1760(%rsp), %ymm15, %ymm11
.Ltmp9187:
	.loc	29 48 14
	vaddps	%ymm11, %ymm12, %ymm12
.Ltmp9188:
	.loc	29 283 14
	vmulps	1152(%rsp), %ymm15, %ymm11
.Ltmp9189:
	.loc	29 48 14
	vaddps	%ymm11, %ymm14, %ymm14
	vmovaps	%ymm8, %ymm11
.Ltmp9190:
	.loc	29 283 14
	vmulps	1376(%rsp), %ymm8, %ymm8
.Ltmp9191:
	.loc	29 48 14
	vaddps	%ymm8, %ymm9, %ymm8
.Ltmp9192:
	.loc	29 283 14
	vmulps	1728(%rsp), %ymm11, %ymm9
.Ltmp9193:
	.loc	29 48 14
	vaddps	%ymm9, %ymm13, %ymm9
.Ltmp9194:
	.loc	29 283 14
	vmulps	1696(%rsp), %ymm11, %ymm13
.Ltmp9195:
	.loc	29 48 14
	vaddps	%ymm13, %ymm12, %ymm12
.Ltmp9196:
	.loc	29 283 14
	vmulps	1664(%rsp), %ymm11, %ymm13
.Ltmp9197:
	.loc	29 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	%ymm7, %ymm14
.Ltmp9198:
	.loc	29 283 14
	vmulps	2400(%rsp), %ymm7, %ymm7
.Ltmp9199:
	.loc	29 48 14
	vaddps	%ymm7, %ymm8, %ymm7
.Ltmp9200:
	.loc	29 283 14
	vmulps	2368(%rsp), %ymm14, %ymm8
.Ltmp9201:
	.loc	29 48 14
	vaddps	%ymm8, %ymm9, %ymm8
.Ltmp9202:
	.loc	29 283 14
	vmulps	2336(%rsp), %ymm14, %ymm9
.Ltmp9203:
	.loc	29 48 14
	vaddps	%ymm9, %ymm12, %ymm9
.Ltmp9204:
	.loc	29 283 14
	vmulps	2304(%rsp), %ymm14, %ymm12
.Ltmp9205:
	.loc	29 48 14
	vaddps	%ymm12, %ymm13, %ymm13
	vmovaps	384(%rsp), %ymm12
.Ltmp9206:
	.loc	29 283 14
	vmulps	2272(%rsp), %ymm12, %ymm2
.Ltmp9207:
	.loc	29 48 14
	vaddps	%ymm2, %ymm7, %ymm2
.Ltmp9208:
	.loc	29 283 14
	vmulps	2496(%rsp), %ymm12, %ymm7
.Ltmp9209:
	.loc	29 48 14
	vaddps	%ymm7, %ymm8, %ymm7
.Ltmp9210:
	.loc	29 283 14
	vmulps	2464(%rsp), %ymm12, %ymm8
.Ltmp9211:
	.loc	29 48 14
	vaddps	%ymm8, %ymm9, %ymm8
.Ltmp9212:
	.loc	29 283 14
	vmulps	2560(%rsp), %ymm12, %ymm9
.Ltmp9213:
	.loc	29 48 14
	vaddps	%ymm9, %ymm13, %ymm9
.Ltmp9214:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm13
.Ltmp9215:
	.loc	29 82 19 is_stmt 0
	vandps	%ymm2, %ymm13, %ymm2
.Ltmp9216:
	.loc	29 82 19
	vandps	%ymm3, %ymm13, %ymm4
.Ltmp9217:
	.loc	29 233 14 is_stmt 1
	vmaxps	%ymm2, %ymm4, %ymm2
.Ltmp9218:
	.loc	29 82 19
	vandps	%ymm7, %ymm13, %ymm4
.Ltmp9219:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm2, %ymm2
.Ltmp9220:
	.loc	29 82 19
	vandps	%ymm13, %ymm8, %ymm4
.Ltmp9221:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm2, %ymm2
.Ltmp9222:
	.loc	29 82 19
	vandps	%ymm13, %ymm9, %ymm4
.Ltmp9223:
	.loc	29 233 14
	vmaxps	%ymm4, %ymm2, %ymm2
	vmovaps	%ymm10, %ymm4
	vmovaps	%ymm3, %ymm10
	vmovaps	%ymm6, %ymm3
	vmovaps	%ymm1, %ymm6
	vmovaps	448(%rsp), %ymm1
.Ltmp9224:
	.loc	12 551 14
	vmovups	%ymm2, 9184(%rsp,%rcx)
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp9225:
	.loc	11 304 12
	addq	$32, %rcx
	vmovaps	%ymm1, %ymm13
	vmovaps	%ymm14, 384(%rsp)
	vmovaps	%ymm11, %ymm7
	vmovaps	%ymm15, %ymm8
	vmovaps	%ymm0, %ymm9
	cmpq	%rcx, %rax
	jne	.LBB34_428
.Ltmp9226:
.LBB34_429:
	.loc	1 1764 5
	vmovaps	%ymm6, 4960(%rsp)
	vmovaps	%ymm1, 4992(%rsp)
	vmovaps	32(%rsp), %ymm1
	vmovaps	%ymm1, 5024(%rsp)
	vmovaps	%ymm5, 5056(%rsp)
	vmovaps	%ymm4, 5088(%rsp)
	vmovaps	%ymm3, 5120(%rsp)
	vmovaps	%ymm10, 5152(%rsp)
	vmovaps	%ymm0, 5184(%rsp)
	vmovaps	%ymm15, 5216(%rsp)
	vmovaps	%ymm11, 5248(%rsp)
	vmovaps	%ymm14, 5280(%rsp)
	vmovaps	%ymm12, 5312(%rsp)
	movq	2592(%rsp), %rcx
.Ltmp9227:
	.loc	11 304 12
	cmpq	%rcx, %rdx
.Ltmp9228:
	.loc	1 1984 19
	je	.LBB34_417
	.loc	1 0 19 is_stmt 0
	movq	%r13, 2272(%rsp)
	movq	%r11, 2304(%rsp)
	movq	%r10, 2336(%rsp)
	movq	%r8, 2368(%rsp)
	movq	888(%rsp), %rdi
	movq	896(%rsp), %rax
	movq	%rax, 1728(%rsp)
	movq	664(%rsp), %rsi
	movq	672(%rsp), %rax
	movq	%rax, 1696(%rsp)
	movq	832(%rsp), %r13
	movq	840(%rsp), %rdx
	movq	880(%rsp), %rax
	movq	%rax, 1504(%rsp)
	movq	848(%rsp), %rax
	movq	%rax, 1920(%rsp)
	movq	856(%rsp), %rax
	movq	%rax, 1952(%rsp)
	movq	872(%rsp), %rax
	movq	%rax, 1472(%rsp)
	movq	864(%rsp), %rax
	movq	%rax, 1440(%rsp)
	movq	608(%rsp), %r8
	movq	616(%rsp), %rax
	movq	%rax, 1888(%rsp)
	vmovaps	800(%rsp), %ymm9
	vmovaps	576(%rsp), %ymm5
	movq	656(%rsp), %rcx
	movq	%rcx, 1856(%rsp)
	movq	624(%rsp), %r9
	movq	632(%rsp), %r11
	movq	648(%rsp), %rcx
	movq	%rcx, 1184(%rsp)
	movq	640(%rsp), %rcx
	movq	%rcx, 1408(%rsp)
	movl	904(%rsp), %ecx
	movl	%ecx, 192(%rsp)
	movl	680(%rsp), %ecx
	movl	%ecx, 480(%rsp)
	xorl	%r10d, %r10d
	vbroadcastss	.LCPI34_2(%rip), %ymm14
	movq	2656(%rsp), %rcx
	xorl	%eax, %eax
	movq	%r11, 1792(%rsp)
	movq	%rdi, 1664(%rsp)
	movq	%rsi, 2400(%rsp)
	movq	%r9, 1824(%rsp)
	movq	%rdx, 1760(%rsp)
.LBB34_431:
	movq	%r10, 32(%rsp)
	.loc	1 1989 21 is_stmt 1
	movq	%rcx, %r9
	subq	%r10, %r9
.Ltmp9229:
	.loc	1 1575 16
	movq	1624(%r12), %r10
.Ltmp9230:
	.loc	1 1576 16
	movq	1632(%r12), %r11
.Ltmp9231:
	.loc	1 1577 25
	leaq	1(%r15), %rcx
.Ltmp9232:
	.loc	1 1148 8
	cmpq	%r10, %rcx
	movq	%r10, %rcx
	cmovbq	%rax, %rcx
	negq	%rcx
	leaq	(%r15,%rcx), %r12
	incq	%r12
.Ltmp9233:
	.loc	1 1578 28
	leaq	(%rdi,%r15), %rbx
.Ltmp9234:
	.loc	1 1148 8
	cmpq	%r10, %rbx
	movq	%r10, %rcx
	cmovbq	%rax, %rcx
	subq	%rcx, %rbx
.Ltmp9235:
	.loc	1 1579 29
	leaq	(%rsi,%r15), %rdi
.Ltmp9236:
	.loc	1 1148 8
	cmpq	%r10, %rdi
	movq	%r10, %rcx
	cmovbq	%rax, %rcx
	subq	%rcx, %rdi
	movq	1728(%rsp), %rcx
.Ltmp9237:
	.loc	1 1580 33
	leaq	(%rcx,%r15), %rsi
.Ltmp9238:
	.loc	1 1148 8
	cmpq	%r10, %rsi
	movq	%r10, %rcx
	cmovbq	%rax, %rcx
	subq	%rcx, %rsi
	movq	%rsi, 992(%rsp)
	movq	1696(%rsp), %rcx
.Ltmp9239:
	.loc	1 1581 34
	leaq	(%rcx,%r15), %rsi
.Ltmp9240:
	.loc	1 1148 8
	cmpq	%r10, %rsi
	movq	%r10, %rcx
	cmovbq	%rax, %rcx
	movq	%r11, %rax
	subq	%rcx, %rsi
.Ltmp9241:
	.loc	1 1583 14
	movq	%r10, %r11
	movq	%r15, 320(%rsp)
	subq	%r15, %r11
.Ltmp9242:
	.loc	10 1078 5
	cmpq	%r9, %r11
	cmovbq	%r11, %r9
	movq	%r14, 2432(%rsp)
.Ltmp9243:
	.loc	1 1584 14
	subq	%r14, %rax
.Ltmp9244:
	.loc	10 1078 5
	cmpq	%r9, %rax
	cmovbq	%rax, %r9
.Ltmp9245:
	.loc	1 1585 14
	movq	%r10, %r15
	movq	%r12, 928(%rsp)
	subq	%r12, %r15
.Ltmp9246:
	.loc	10 1078 5
	cmpq	%r9, %r15
	cmovbq	%r15, %r9
.Ltmp9247:
	.loc	1 1586 14
	movq	%r10, %r12
	movq	%rbx, 1536(%rsp)
	subq	%rbx, %r12
.Ltmp9248:
	.loc	10 1078 5
	cmpq	%r9, %r12
	cmovbq	%r12, %r9
.Ltmp9249:
	.loc	1 1587 14
	movq	%r10, %rbx
	movq	%rdi, 960(%rsp)
	subq	%rdi, %rbx
.Ltmp9250:
	.loc	10 1078 5
	cmpq	%r9, %rbx
	cmovbq	%rbx, %r9
.Ltmp9251:
	.loc	1 1588 14
	movq	%r10, %rcx
	movq	992(%rsp), %rdi
	subq	%rdi, %rcx
.Ltmp9252:
	.loc	10 1078 5
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	movq	%rsi, 1312(%rsp)
.Ltmp9253:
	.loc	1 1589 14
	subq	%rsi, %r10
.Ltmp9254:
	.loc	10 1078 5
	cmpq	%r9, %r10
	cmovbq	%r10, %r9
	movq	2592(%rsp), %rsi
	movq	32(%rsp), %r14
.Ltmp9255:
	.loc	1 1995 28
	leaq	(%r14,%rsi), %rdi
	movq	%r9, 1152(%rsp)
.Ltmp9256:
	.loc	1 1997 55
	leaq	(%r9,%rdi), %rsi
.Ltmp9257:
	.loc	1 1995 28
	shlq	$3, %rdi
.Ltmp9258:
	.loc	1 1997 55
	shlq	$3, %rsi
.Ltmp9259:
	.loc	4 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB34_433
	cmpq	184(%rsp), %rsi
	ja	.LBB34_433
.Ltmp9260:
	.loc	5 451 16
	cmpq	88(%rsp), %rsi
	ja	.LBB34_436
.Ltmp9261:
	.loc	1 1999 46
	leaq	(,%r14,8), %rsi
	movq	%rsi, 384(%rsp)
	movq	1152(%rsp), %r9
	.loc	1 1999 61 is_stmt 0
	leaq	(%r9,%r14), %rsi
	movq	%rsi, 1376(%rsp)
	cmpq	$33, %rsi
.Ltmp9262:
	.loc	4 1050 16 is_stmt 1
	jae	.LBB34_633
.Ltmp9263:
	.loc	11 304 12
	testq	%r9, %r9
	je	.LBB34_440
.Ltmp9264:
	.loc	11 0 12 is_stmt 0
	cmpq	%rbx, %r12
	cmovbq	%r12, %rbx
	cmpq	%rcx, %rbx
	cmovaeq	%rcx, %rbx
	cmpq	%r10, %rbx
	cmovaeq	%r10, %rbx
	cmpq	%r15, %rbx
	cmovaeq	%r15, %rbx
	cmpq	%r11, %rbx
	cmovaeq	%r11, %rbx
	movq	536(%rsp), %rcx
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 2080(%rsp)
	movq	432(%rsp), %rcx
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 2016(%rsp)
	cmpq	%rax, %rbx
	cmovaeq	%rax, %rbx
	movq	768(%rsp), %rcx
	subq	%r14, %rcx
	movq	384(%rsp), %rax
	leaq	(%rsp,%rax,4), %rsi
	addq	$10216, %rsi
	movq	%rsi, 2176(%rsp)
	leaq	(%rsp,%rax,4), %rsi
	addq	$9184, %rsi
	movq	%rsi, 2144(%rsp)
	cmpq	%rcx, %rbx
	cmovbq	%rbx, %rcx
	andq	424(%rsp), %rcx
	movq	%rcx, 1984(%rsp)
.Ltmp9265:
	.loc	1 853 44 is_stmt 1
	vmovaps	4736(%rsp), %ymm0
	vmovaps	%ymm0, 32(%rsp)
	.loc	1 853 61 is_stmt 0
	vmovaps	4800(%rsp), %ymm15
.Ltmp9266:
	.loc	1 853 44
	vmovaps	5344(%rsp), %ymm0
	vmovaps	%ymm0, 384(%rsp)
	.loc	1 853 73
	vmovaps	5376(%rsp), %ymm0
	vmovaps	%ymm0, 1280(%rsp)
	.loc	1 853 61
	vmovaps	5408(%rsp), %ymm11
.Ltmp9267:
	.loc	1 851 26 is_stmt 1
	vmovaps	5440(%rsp), %ymm6
.Ltmp9268:
	.loc	1 851 26 is_stmt 0
	vmovaps	5568(%rsp), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	vmovaps	4608(%rsp), %ymm1
	vmovaps	4640(%rsp), %ymm0
	vmovaps	%ymm0, 1248(%rsp)
	vmovaps	4672(%rsp), %ymm2
	vmovaps	4704(%rsp), %ymm0
	vmovaps	4768(%rsp), %ymm3
	vmovaps	%ymm3, 1216(%rsp)
	vmovaps	5504(%rsp), %ymm3
	vmovaps	%ymm3, 2208(%rsp)
	vmovaps	4928(%rsp), %ymm3
	vmovaps	%ymm3, 2112(%rsp)
	vmovaps	5664(%rsp), %ymm3
	vmovaps	%ymm3, 2048(%rsp)
	vmovaps	4832(%rsp), %ymm7
	xorl	%ecx, %ecx
	vmovaps	5472(%rsp), %ymm3
	vmovaps	%ymm3, 448(%rsp)
	vmovaps	5536(%rsp), %ymm10
	vmovaps	4896(%rsp), %ymm8
	vmovaps	5632(%rsp), %ymm4
	movq	320(%rsp), %r15
.Ltmp9269:
	.loc	1 0 26
.Ltmp9270:
	.p2align	4
.LBB34_442:
	vmovaps	%ymm9, %ymm13
	vmovaps	%ymm5, 352(%rsp)
	vbroadcastss	.LCPI34_1(%rip), %ymm9
.Ltmp9271:
	.loc	29 347 14 is_stmt 1
	vaddps	%ymm0, %ymm9, %ymm0
	vxorps	%xmm12, %xmm12, %xmm12
.Ltmp9272:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm0, %ymm0
.Ltmp9273:
	.loc	29 871 14
	vcmpgt_oqps	%ymm12, %ymm0, %ymm3
.Ltmp9274:
	.loc	29 48 14
	vaddps	%ymm2, %ymm1, %ymm1
	vmovaps	1248(%rsp), %ymm5
.Ltmp9275:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm1, %ymm5, %ymm1
.Ltmp9276:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm2, %ymm12, %ymm2
.Ltmp9277:
	.loc	29 347 14 is_stmt 1
	vaddps	%ymm7, %ymm9, %ymm3
.Ltmp9278:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm3, %ymm3
	vmovaps	%ymm3, 256(%rsp)
.Ltmp9279:
	.loc	29 871 14
	vcmpgt_oqps	%ymm12, %ymm3, %ymm3
.Ltmp9280:
	.loc	29 48 14
	vaddps	32(%rsp), %ymm15, %ymm5
	vmovaps	1216(%rsp), %ymm7
.Ltmp9281:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm5, %ymm7, %ymm5
	vmovaps	%ymm5, 32(%rsp)
.Ltmp9282:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm15, %ymm12, %ymm15
.Ltmp9283:
	.loc	29 347 14 is_stmt 1
	vaddps	%ymm6, %ymm9, %ymm3
.Ltmp9284:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm3, %ymm3
	vmovaps	%ymm3, 128(%rsp)
.Ltmp9285:
	.loc	29 871 14
	vcmpgt_oqps	%ymm12, %ymm3, %ymm3
.Ltmp9286:
	.loc	29 48 14
	vaddps	384(%rsp), %ymm11, %ymm7
	vmovaps	1280(%rsp), %ymm5
.Ltmp9287:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm7, %ymm5, %ymm5
	vmovaps	%ymm5, 384(%rsp)
.Ltmp9288:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm11, %ymm12, %ymm11
.Ltmp9289:
	.loc	29 347 14 is_stmt 1
	vaddps	96(%rsp), %ymm9, %ymm3
.Ltmp9290:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm3, %ymm5
.Ltmp9291:
	.loc	29 871 14
	vcmpgt_oqps	%ymm12, %ymm5, %ymm3
.Ltmp9292:
	.loc	29 48 14
	vaddps	448(%rsp), %ymm10, %ymm7
	vmovaps	2208(%rsp), %ymm6
.Ltmp9293:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm7, %ymm6, %ymm6
	vmovaps	%ymm6, 448(%rsp)
.Ltmp9294:
	.loc	29 585 19 is_stmt 0
	vblendvps	%ymm3, %ymm10, %ymm12, %ymm10
.Ltmp9295:
	.loc	1 1502 26 is_stmt 1
	leaq	(%rcx,%r15), %rsi
.Ltmp9296:
	.loc	1 1137 16
	leaq	(,%rsi,8), %r9
.Ltmp9297:
	.loc	1 1138 33
	leaq	8(,%rsi,8), %r14
.Ltmp9298:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %rsi
	cmpq	%rdx, %rsi
	jae	.LBB34_634
.Ltmp9299:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm5, 96(%rsp)
	movq	%rcx, %rbx
	shlq	$5, %rbx
	movq	2176(%rsp), %rsi
	vmovups	(%rsi,%rbx), %ymm9
	movq	2144(%rsp), %rsi
	vmovups	(%rsi,%rbx), %ymm3
.Ltmp9300:
	vmaxps	%ymm9, %ymm3, %ymm7
	vmovaps	544(%rsp), %ymm5
.Ltmp9301:
	.loc	29 585 19 is_stmt 1
	vblendvps	%ymm5, %ymm7, %ymm9, %ymm9
.Ltmp9302:
	.loc	29 360 14
	vdivps	%ymm9, %ymm1, %ymm12
	movq	1536(%rsp), %rsi
.Ltmp9303:
	.loc	1 0 0 is_stmt 0
	addq	%rcx, %rsi
.Ltmp9304:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm1, %ymm9, %ymm9
.Ltmp9305:
	.loc	29 585 19
	vblendvps	%ymm9, %ymm12, %ymm14, %ymm9
.Ltmp9306:
	.loc	12 551 14
	vmovups	%ymm9, (%r13,%r9,4)
.Ltmp9307:
	.loc	1 1130 16
	leaq	(,%rsi,8), %rdi
.Ltmp9308:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %r10
	cmpq	%rdx, %r10
	movl	192(%rsp), %r12d
	jae	.LBB34_444
.Ltmp9309:
	.loc	12 551 14
	vmovups	(%r13,%rdi,4), %ymm9
	vmovaps	%ymm9, %ymm6
.Ltmp9310:
	.loc	1 1206 22
	testl	%r12d, %r12d
	je	.LBB34_449
.Ltmp9311:
	.loc	29 257 14
	vminps	%ymm9, %ymm13, %ymm6
.Ltmp9312:
.LBB34_449:
	.loc	29 0 14 is_stmt 0
	movq	928(%rsp), %rdi
	leaq	(%rcx,%rdi), %r10
.Ltmp9313:
	movl	%r12d, %r11d
.Ltmp9314:
	.loc	1 1212 20 is_stmt 1
	incq	%r11
	movq	1504(%rsp), %rdi
	movq	%rdi, %r15
	cmpq	%rdi, %r11
.Ltmp9315:
	.loc	1 1213 22
	jne	.LBB34_450
	.loc	1 0 22 is_stmt 0
.Ltmp9316:
	.p2align	4
.LBB34_453:
.Ltmp9317:
	.loc	1 1130 16 is_stmt 1
	leaq	(,%rsi,8), %rdi
.Ltmp9318:
	.loc	4 1050 16
	leaq	7(,%rsi,8), %r11
	cmpq	%rdx, %r11
	jae	.LBB34_451
.Ltmp9319:
	.loc	29 257 14
	vminps	(%r13,%rdi,4), %ymm9, %ymm9
.Ltmp9320:
	.loc	12 551 14
	vmovups	%ymm9, (%r13,%rdi,4)
.Ltmp9321:
	.loc	1 1224 16
	testq	%rsi, %rsi
	cmoveq	224(%rsp), %rsi
	.loc	1 1227 13
	decq	%rsi
.Ltmp9322:
	.loc	10 1916 50
	decq	%r15
.Ltmp9323:
	.loc	3 900 12
	jne	.LBB34_453
.Ltmp9324:
	.loc	3 0 12 is_stmt 0
	xorl	%r15d, %r15d
	vmovaps	%ymm6, %ymm9
	jmp	.LBB34_456
	.p2align	4
.LBB34_450:
.Ltmp9325:
	.loc	1 1130 16 is_stmt 1
	leaq	(,%r10,8), %rdi
.Ltmp9326:
	.loc	4 1050 16
	leaq	7(,%r10,8), %rsi
	cmpq	%rdx, %rsi
	jae	.LBB34_451
.Ltmp9327:
	.loc	12 551 14
	vmovups	(%r13,%rdi,4), %ymm9
.Ltmp9328:
	.loc	29 257 14
	vminps	%ymm6, %ymm9, %ymm9
	movl	%r11d, %r15d
.Ltmp9329:
.LBB34_456:
	.loc	29 0 14 is_stmt 0
	movq	992(%rsp), %rsi
	leaq	(%rcx,%rsi), %rdi
.Ltmp9330:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%rdi,8), %rsi
.Ltmp9331:
	.loc	1 1130 16
	shlq	$3, %rdi
	movq	1952(%rsp), %r11
.Ltmp9332:
	.loc	4 1050 16
	cmpq	%r11, %rsi
	movq	1920(%rsp), %r12
	jae	.LBB34_457
.Ltmp9333:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm5
	vmulps	%ymm5, %ymm9, %ymm9
	vroundps	$9, %ymm9, %ymm9
	vbroadcastss	.LCPI34_4(%rip), %ymm5
	vmulps	%ymm5, %ymm9, %ymm9
.Ltmp9334:
	.loc	29 48 14 is_stmt 1
	vaddps	%ymm8, %ymm9, %ymm8
.Ltmp9335:
	.loc	29 347 14
	vsubps	(%r12,%rdi,4), %ymm8, %ymm8
.Ltmp9336:
	.loc	1 1654 5
	vmovaps	%ymm8, 4896(%rsp)
	cmpq	%r11, %r14
.Ltmp9337:
	.loc	4 1050 16
	ja	.LBB34_635
.Ltmp9338:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm6, %ymm13
	movq	2432(%rsp), %rsi
	leaq	(%rcx,%rsi), %r11
.Ltmp9339:
	.loc	12 551 14 is_stmt 1
	vmovups	%ymm9, (%r12,%r9,4)
.Ltmp9340:
	.loc	29 360 14
	vdivps	2112(%rsp), %ymm8, %ymm9
.Ltmp9341:
	.loc	1 1659 43
	vmovaps	4864(%rsp), %ymm12
.Ltmp9342:
	.loc	29 347 14
	vsubps	%ymm9, %ymm14, %ymm9
	vmovaps	%ymm14, %ymm6
.Ltmp9343:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm12, %ymm9, %ymm14
.Ltmp9344:
	.loc	29 283 14 is_stmt 1
	vmulps	32(%rsp), %ymm14, %ymm14
.Ltmp9345:
	.loc	29 48 14
	vaddps	%ymm14, %ymm12, %ymm12
.Ltmp9346:
	.loc	29 233 14
	vmaxps	%ymm12, %ymm9, %ymm12
.Ltmp9347:
	.loc	29 82 19
	vbroadcastss	.LCPI34_0(%rip), %ymm9
	vandps	%ymm9, %ymm12, %ymm14
.Ltmp9348:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm5
	vcmplt_oqps	%ymm5, %ymm14, %ymm14
.Ltmp9349:
	.loc	29 82 19
	vandnps	%ymm12, %ymm14, %ymm12
.Ltmp9350:
	.loc	1 1660 5
	vmovaps	%ymm12, 4864(%rsp)
.Ltmp9351:
	.loc	1 1130 16
	leaq	(,%r11,8), %rdi
.Ltmp9352:
	.loc	1 1131 25
	leaq	8(,%r11,8), %rsi
.Ltmp9353:
	.loc	4 1050 16
	leaq	7(,%r11,8), %r11
	cmpq	1472(%rsp), %r11
	jae	.LBB34_636
.Ltmp9354:
	.loc	4 0 16 is_stmt 0
	movq	%rsi, %rdx
	movq	%r14, %rsi
	movq	%r9, %r14
	movq	2080(%rsp), %r9
	leaq	(%r9,%rbx), %r11
.Ltmp9355:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm12, %ymm6, %ymm12
	movq	1440(%rsp), %r12
.Ltmp9356:
	.loc	12 551 14
	vmovups	(%r12,%rdi,4), %ymm14
.Ltmp9357:
	.loc	12 551 14 is_stmt 0
	vmovups	(%r11), %ymm5
	vmovups	%ymm5, (%r12,%rdi,4)
.Ltmp9358:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm14, %ymm12, %ymm5
	vmovaps	2240(%rsp), %ymm12
.Ltmp9359:
	.loc	29 585 19
	vblendvps	%ymm12, %ymm14, %ymm5, %ymm5
.Ltmp9360:
	.loc	12 551 14
	vmovups	%ymm5, (%r11)
	movq	%rsi, %rax
	movq	1888(%rsp), %r9
	cmpq	%r9, %rsi
	movq	%r9, %rsi
.Ltmp9361:
	.loc	4 1050 16
	ja	.LBB34_637
.Ltmp9362:
	.loc	4 0 16 is_stmt 0
	movl	%r15d, 192(%rsp)
	vmovaps	544(%rsp), %ymm5
	vblendvps	%ymm5, %ymm7, %ymm3, %ymm3
	vmovaps	384(%rsp), %ymm7
.Ltmp9363:
	.loc	29 360 14 is_stmt 1
	vdivps	%ymm3, %ymm7, %ymm5
	movq	960(%rsp), %r9
.Ltmp9364:
	.loc	1 0 0 is_stmt 0
	leaq	(%rcx,%r9), %r12
.Ltmp9365:
	.loc	29 871 14 is_stmt 1
	vcmpgt_oqps	%ymm7, %ymm3, %ymm3
.Ltmp9366:
	.loc	29 585 19
	vblendvps	%ymm3, %ymm5, %ymm6, %ymm3
.Ltmp9367:
	.loc	12 551 14
	vmovups	%ymm3, (%r8,%r14,4)
.Ltmp9368:
	.loc	1 1130 16
	leaq	(,%r12,8), %r15
.Ltmp9369:
	.loc	4 1050 16
	leaq	7(,%r12,8), %r11
	cmpq	%rsi, %r11
	vmovaps	%ymm13, %ymm12
	jae	.LBB34_638
.Ltmp9370:
	.loc	4 0 16 is_stmt 0
	vmovaps	%ymm6, %ymm14
.Ltmp9371:
	.loc	12 551 14 is_stmt 1
	vmovups	(%r8,%r15,4), %ymm7
	vmovaps	%ymm7, %ymm3
.Ltmp9372:
	.loc	1 1206 22
	cmpl	$0, 480(%rsp)
	je	.LBB34_464
	.loc	1 0 22 is_stmt 0
	vmovaps	352(%rsp), %ymm3
.Ltmp9373:
	.loc	29 257 14 is_stmt 1
	vminps	%ymm7, %ymm3, %ymm3
.Ltmp9374:
.LBB34_464:
	.loc	1 0 0 is_stmt 0
	movl	480(%rsp), %r11d
.Ltmp9375:
	.loc	1 1212 20 is_stmt 1
	incq	%r11
	movq	1856(%rsp), %r9
	movq	%r9, %r15
	cmpq	%r9, %r11
	movq	1824(%rsp), %r9
.Ltmp9376:
	.loc	1 1213 22
	jne	.LBB34_465
	.loc	1 0 22 is_stmt 0
.Ltmp9377:
	.p2align	4
.LBB34_467:
.Ltmp9378:
	.loc	1 1130 16 is_stmt 1
	leaq	(,%r12,8), %r10
.Ltmp9379:
	.loc	4 1050 16
	leaq	7(,%r12,8), %r11
	cmpq	%rsi, %r11
	jae	.LBB34_639
.Ltmp9380:
	.loc	29 257 14
	vminps	(%r8,%r10,4), %ymm7, %ymm7
.Ltmp9381:
	.loc	12 551 14
	vmovups	%ymm7, (%r8,%r10,4)
.Ltmp9382:
	.loc	1 1224 16
	testq	%r12, %r12
	cmoveq	224(%rsp), %r12
	.loc	1 1227 13
	decq	%r12
.Ltmp9383:
	.loc	10 1916 50
	decq	%r15
.Ltmp9384:
	.loc	3 900 12
	jne	.LBB34_467
.Ltmp9385:
	.loc	3 0 12 is_stmt 0
	movl	$0, 480(%rsp)
	vmovaps	%ymm3, %ymm7
	jmp	.LBB34_470
	.p2align	4
.LBB34_465:
.Ltmp9386:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r10,8), %r15
.Ltmp9387:
	.loc	1 1130 16
	shlq	$3, %r10
.Ltmp9388:
	.loc	4 1050 16
	cmpq	%rsi, %r15
	jae	.LBB34_639
.Ltmp9389:
	.loc	12 551 14
	vmovups	(%r8,%r10,4), %ymm5
.Ltmp9390:
	.loc	29 257 14
	vminps	%ymm3, %ymm5, %ymm7
	movl	%r11d, 480(%rsp)
.Ltmp9391:
.LBB34_470:
	.loc	29 0 14 is_stmt 0
	movq	1312(%rsp), %r10
	addq	%rcx, %r10
.Ltmp9392:
	.loc	4 1050 16 is_stmt 1
	leaq	7(,%r10,8), %r11
.Ltmp9393:
	.loc	1 1130 16
	shlq	$3, %r10
	movq	1792(%rsp), %r15
.Ltmp9394:
	.loc	4 1050 16
	cmpq	%r15, %r11
	movq	%r15, %r11
	movq	440(%rsp), %r12
	movq	320(%rsp), %r15
	jae	.LBB34_471
.Ltmp9395:
	.loc	1 0 0 is_stmt 0
	vbroadcastss	.LCPI34_3(%rip), %ymm5
	vmulps	%ymm5, %ymm7, %ymm5
	vroundps	$9, %ymm5, %ymm5
	vbroadcastss	.LCPI34_4(%rip), %ymm6
	vmulps	%ymm6, %ymm5, %ymm7
.Ltmp9396:
	.loc	29 48 14 is_stmt 1
	vaddps	%ymm4, %ymm7, %ymm4
.Ltmp9397:
	.loc	29 347 14
	vsubps	(%r9,%r10,4), %ymm4, %ymm4
.Ltmp9398:
	.loc	1 1654 5
	vmovaps	%ymm4, 5632(%rsp)
	cmpq	%r11, %rax
.Ltmp9399:
	.loc	4 1050 16
	ja	.LBB34_640
.Ltmp9400:
	.loc	29 360 14
	vdivps	2048(%rsp), %ymm4, %ymm5
.Ltmp9401:
	.loc	12 551 14
	vmovups	%ymm7, (%r9,%r14,4)
.Ltmp9402:
	.loc	1 1659 43
	vmovaps	5600(%rsp), %ymm7
.Ltmp9403:
	.loc	29 347 14
	vsubps	%ymm5, %ymm14, %ymm5
.Ltmp9404:
	.loc	29 347 14 is_stmt 0
	vsubps	%ymm7, %ymm5, %ymm12
.Ltmp9405:
	.loc	29 283 14 is_stmt 1
	vmulps	448(%rsp), %ymm12, %ymm12
.Ltmp9406:
	.loc	29 48 14
	vaddps	%ymm7, %ymm12, %ymm7
.Ltmp9407:
	.loc	29 233 14
	vmaxps	%ymm7, %ymm5, %ymm5
.Ltmp9408:
	.loc	29 82 19
	vandps	%ymm5, %ymm9, %ymm7
.Ltmp9409:
	.loc	29 871 14
	vbroadcastss	.LCPI34_5(%rip), %ymm6
	vcmplt_oqps	%ymm6, %ymm7, %ymm7
.Ltmp9410:
	.loc	29 82 19
	vandnps	%ymm5, %ymm7, %ymm7
.Ltmp9411:
	.loc	1 1660 5
	vmovaps	%ymm7, 5600(%rsp)
	cmpq	1184(%rsp), %rdx
.Ltmp9412:
	.loc	4 1050 16
	ja	.LBB34_641
.Ltmp9413:
	.loc	1 0 0 is_stmt 0
	addq	2016(%rsp), %rbx
	incq	%rcx
.Ltmp9414:
	.loc	29 347 14 is_stmt 1
	vsubps	%ymm7, %ymm14, %ymm5
	movq	1408(%rsp), %rsi
.Ltmp9415:
	.loc	12 551 14
	vmovups	(%rsi,%rdi,4), %ymm7
.Ltmp9416:
	.loc	12 551 14 is_stmt 0
	vmovups	(%rbx), %ymm9
	vmovups	%ymm9, (%rsi,%rdi,4)
.Ltmp9417:
	.loc	29 283 14 is_stmt 1
	vmulps	%ymm7, %ymm5, %ymm5
	vmovaps	2240(%rsp), %ymm9
.Ltmp9418:
	.loc	29 585 19
	vblendvps	%ymm9, %ymm7, %ymm5, %ymm5
.Ltmp9419:
	.loc	12 551 14
	vmovups	%ymm5, (%rbx)
	vmovaps	%ymm3, %ymm5
	vmovaps	%ymm13, %ymm9
.Ltmp9420:
	.loc	11 304 12
	cmpq	1984(%rsp), %rcx
	vmovaps	128(%rsp), %ymm6
	vmovaps	256(%rsp), %ymm7
	movq	1760(%rsp), %rdx
	jne	.LBB34_442
.Ltmp9421:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9422:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
.Ltmp9423:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm7, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9424:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
.Ltmp9425:
	.loc	1 851 9
	vmovaps	%ymm6, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9426:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9427:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9428:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
	vmovaps	%ymm3, %ymm5
	vmovaps	%ymm13, %ymm9
	movq	184(%rsp), %rbx
	xorl	%eax, %eax
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB34_476
.Ltmp9429:
.LBB34_440:
	movq	440(%rsp), %r12
	movq	184(%rsp), %rbx
	movq	320(%rsp), %r15
	xorl	%eax, %eax
.LBB34_476:
	movq	1152(%rsp), %rdi
	.loc	1 2056 39 is_stmt 1
	addq	%rdi, %r15
	movq	224(%rsp), %rcx
.Ltmp9430:
	.loc	1 1148 8
	cmpq	%rcx, %r15
	cmovbq	%rax, %rcx
	subq	%rcx, %r15
	movq	2432(%rsp), %r14
.Ltmp9431:
	.loc	1 2057 39
	addq	%rdi, %r14
	movq	704(%rsp), %rcx
.Ltmp9432:
	.loc	1 1148 8
	cmpq	%rcx, %r14
	cmovbq	%rax, %rcx
	subq	%rcx, %r14
	movq	1376(%rsp), %rsi
	movq	%rsi, %r10
	movq	2656(%rsp), %rcx
.Ltmp9433:
	.loc	1 1984 19
	cmpq	%rcx, %rsi
	movq	1664(%rsp), %rdi
	movq	2400(%rsp), %rsi
	jb	.LBB34_431
	jmp	.LBB34_416
.LBB34_451:
.Ltmp9434:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9435:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9436:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9437:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9438:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9439:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9440:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9441:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
.Ltmp9442:
	movl	%r12d, 904(%rsp)
.Ltmp9443:
	vmovaps	%ymm6, 800(%rsp)
.Ltmp9444:
.LBB34_445:
	movl	480(%rsp), %eax
.Ltmp9445:
	movl	%eax, 680(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9446:
	vmovaps	%ymm0, 576(%rsp)
.Ltmp9447:
	leaq	8(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_639:
.Ltmp9448:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9449:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9450:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9451:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9452:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9453:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9454:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9455:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
	movl	192(%rsp), %ecx
.Ltmp9456:
	movl	%ecx, 904(%rsp)
.Ltmp9457:
	vmovaps	%ymm12, 800(%rsp)
	movl	480(%rsp), %ecx
.Ltmp9458:
	movl	%ecx, 680(%rsp)
.Ltmp9459:
	vmovaps	%ymm3, 576(%rsp)
	movq	%rsi, %rdx
.Ltmp9460:
	.loc	1 1131 25 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp9461:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9462:
.LBB34_624:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_f6bbc99b95dcf27c100d71299ef7abde(%rip), %rcx
	movq	%rax, %rdi
	movq	88(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9463:
.LBB34_597:
	.loc	1 455 21 is_stmt 1
	xorl	%edx, %edx
	divl	%esi
.LBB34_599:
	.loc	1 455 9 is_stmt 0
	movl	%edx, 1640(%r12)
	.loc	1 456 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB34_653
	.loc	1 456 23 is_stmt 0
	movl	1644(%r12), %esi
	movq	312(%rsp), %rdx
	.loc	1 456 44
	cmpq	%rcx, %rdx
	jb	.LBB34_602
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ecx
.LBB34_602:
	.loc	1 456 22
	addq	%rsi, %rdx
	.loc	1 456 21
	movq	%rdx, %rax
	orq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB34_603
	movq	%rdx, %rax
	xorl	%edx, %edx
	divq	%rcx
	jmp	.LBB34_605
.LBB34_603:
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ecx
.LBB34_605:
	.loc	1 456 9
	movl	%edx, 1644(%r12)
	jmp	.LBB34_606
.Ltmp9464:
.LBB34_546:
	.loc	1 0 0
	vmovaps	%ymm3, 6016(%rsp)
	vmovaps	%ymm9, 6048(%rsp)
	vmovaps	%ymm14, 6080(%rsp)
.Ltmp9465:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	%ymm15, 6784(%rsp)
	vmovaps	%ymm1, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
	jmp	.LBB34_547
.Ltmp9466:
.LBB34_643:
	vmovaps	1568(%rsp), %ymm0
.Ltmp9467:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9468:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9469:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r8, %rdi
	movq	%r11, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9470:
.LBB34_634:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9471:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9472:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9473:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9474:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9475:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
.Ltmp9476:
	.loc	1 851 9
	vmovaps	%ymm5, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9477:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
	movl	192(%rsp), %eax
.Ltmp9478:
	movl	%eax, 904(%rsp)
.Ltmp9479:
	vmovaps	%ymm13, 800(%rsp)
	movl	480(%rsp), %eax
.Ltmp9480:
	movl	%eax, 680(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9481:
	vmovaps	%ymm0, 576(%rsp)
.Ltmp9482:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r9, %rdi
	movq	%r14, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9483:
.LBB34_444:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9484:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9485:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9486:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9487:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9488:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9489:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9490:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
.Ltmp9491:
	movl	%r12d, 904(%rsp)
.Ltmp9492:
	vmovaps	%ymm13, 800(%rsp)
	jmp	.LBB34_445
.Ltmp9493:
.LBB34_525:
	vmovaps	1568(%rsp), %ymm0
.Ltmp9494:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9495:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	%ymm10, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9496:
	leaq	8(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_457:
.Ltmp9497:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9498:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9499:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9500:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9501:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9502:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9503:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9504:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
.Ltmp9505:
	movl	%r15d, 904(%rsp)
.Ltmp9506:
	vmovaps	%ymm6, 800(%rsp)
	movl	480(%rsp), %eax
.Ltmp9507:
	movl	%eax, 680(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9508:
	vmovaps	%ymm0, 576(%rsp)
.Ltmp9509:
	leaq	8(%rdi), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_644:
	vmovaps	1568(%rsp), %ymm0
.Ltmp9510:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9511:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	%ymm10, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9512:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r8, %rdi
	movq	224(%rsp), %rsi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9513:
.LBB34_635:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9514:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9515:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9516:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9517:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9518:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9519:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9520:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
.Ltmp9521:
	movl	%r15d, 904(%rsp)
.Ltmp9522:
	vmovaps	%ymm6, 800(%rsp)
	movl	480(%rsp), %eax
.Ltmp9523:
	movl	%eax, 680(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9524:
	vmovaps	%ymm0, 576(%rsp)
.Ltmp9525:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r9, %rdi
	movq	%r14, %rsi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9526:
.LBB34_645:
	.loc	5 0 13 is_stmt 0
	vmovaps	1568(%rsp), %ymm0
.Ltmp9527:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9528:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	%ymm10, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9529:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	1504(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9530:
.LBB34_636:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9531:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9532:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9533:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9534:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9535:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9536:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9537:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
.Ltmp9538:
	movl	%r15d, 904(%rsp)
.Ltmp9539:
	vmovaps	%ymm13, 800(%rsp)
	movl	480(%rsp), %eax
.Ltmp9540:
	movl	%eax, 680(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9541:
	vmovaps	%ymm0, 576(%rsp)
.Ltmp9542:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	1472(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9543:
.LBB34_646:
	.loc	5 0 13 is_stmt 0
	vmovaps	1568(%rsp), %ymm0
.Ltmp9544:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9545:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	%ymm10, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %ecx
	movl	%ecx, 904(%rsp)
	movl	28(%rsp), %ecx
	movl	%ecx, 680(%rsp)
.Ltmp9546:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	544(%rsp), %rdi
	movq	224(%rsp), %rsi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9547:
.LBB34_637:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9548:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9549:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9550:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9551:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9552:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9553:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9554:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
.Ltmp9555:
	movl	%r15d, 904(%rsp)
.Ltmp9556:
	vmovaps	%ymm13, 800(%rsp)
	movl	480(%rsp), %ecx
.Ltmp9557:
	movl	%ecx, 680(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9558:
	vmovaps	%ymm0, 576(%rsp)
.Ltmp9559:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r14, %rdi
	movq	%rsi, %rdx
	movq	%rax, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9560:
.LBB34_647:
	.loc	5 0 13 is_stmt 0
	vmovaps	1568(%rsp), %ymm0
.Ltmp9561:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9562:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	%ymm10, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %ecx
	movl	%ecx, 904(%rsp)
	movl	28(%rsp), %ecx
	movl	%ecx, 680(%rsp)
.Ltmp9563:
	.loc	1 1131 25 is_stmt 1
	leaq	8(%r15), %rsi
.Ltmp9564:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r15, %rdi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9565:
.LBB34_638:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9566:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9567:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9568:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9569:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9570:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9571:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9572:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
	movl	192(%rsp), %ecx
.Ltmp9573:
	movl	%ecx, 904(%rsp)
.Ltmp9574:
	vmovaps	%ymm12, 800(%rsp)
	movl	480(%rsp), %ecx
.Ltmp9575:
	movl	%ecx, 680(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9576:
	vmovaps	%ymm0, 576(%rsp)
	movq	%rsi, %rdx
.Ltmp9577:
	.loc	1 1131 25 is_stmt 1
	leaq	8(%r15), %rsi
.Ltmp9578:
	.loc	5 443 13
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r15, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9579:
.LBB34_540:
	.loc	5 0 13 is_stmt 0
	vmovaps	1568(%rsp), %ymm0
.Ltmp9580:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9581:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
	jmp	.LBB34_541
.Ltmp9582:
.LBB34_471:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9583:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9584:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9585:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9586:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9587:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9588:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9589:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
	movl	192(%rsp), %eax
.Ltmp9590:
	movl	%eax, 904(%rsp)
.Ltmp9591:
	vmovaps	%ymm12, 800(%rsp)
	movl	480(%rsp), %eax
.Ltmp9592:
	movl	%eax, 680(%rsp)
.Ltmp9593:
	vmovaps	%ymm3, 576(%rsp)
.Ltmp9594:
.LBB34_541:
	leaq	8(%r10), %rsi
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%r10, %rdi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_649:
	vmovaps	1568(%rsp), %ymm0
.Ltmp9595:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9596:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9597:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	544(%rsp), %rdi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9598:
.LBB34_640:
	.loc	5 0 13 is_stmt 0
	movq	%rax, %rsi
.Ltmp9599:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9600:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9601:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9602:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9603:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9604:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9605:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9606:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
	movl	192(%rsp), %eax
.Ltmp9607:
	movl	%eax, 904(%rsp)
.Ltmp9608:
	vmovaps	%ymm12, 800(%rsp)
	movl	480(%rsp), %eax
.Ltmp9609:
	movl	%eax, 680(%rsp)
.Ltmp9610:
	vmovaps	%ymm3, 576(%rsp)
.Ltmp9611:
	.loc	5 456 13 is_stmt 1
	leaq	.Lalloc_aac110d217bd6a4c15b81b08a4ef3ef7(%rip), %rcx
	movq	%r14, %rdi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9612:
.LBB34_650:
	.loc	5 0 13 is_stmt 0
	vmovaps	1568(%rsp), %ymm0
.Ltmp9613:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9614:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9615:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	1440(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9616:
.LBB34_641:
	.loc	1 851 0
	vmovaps	%ymm0, 4704(%rsp)
.Ltmp9617:
	.loc	1 853 0
	vmovaps	%ymm1, 4608(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9618:
	.loc	1 851 0 is_stmt 1
	vmovaps	%ymm0, 4832(%rsp)
	vmovaps	32(%rsp), %ymm0
.Ltmp9619:
	.loc	1 853 9
	vmovaps	%ymm0, 4736(%rsp)
	.loc	1 854 9
	vmovaps	%ymm15, 4800(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9620:
	.loc	1 851 9
	vmovaps	%ymm0, 5440(%rsp)
	vmovaps	384(%rsp), %ymm0
.Ltmp9621:
	.loc	1 853 9
	vmovaps	%ymm0, 5344(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 5408(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9622:
	.loc	1 851 9
	vmovaps	%ymm0, 5568(%rsp)
	vmovaps	448(%rsp), %ymm0
.Ltmp9623:
	.loc	1 853 0
	vmovaps	%ymm0, 5472(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm10, 5536(%rsp)
	movl	192(%rsp), %eax
.Ltmp9624:
	movl	%eax, 904(%rsp)
.Ltmp9625:
	vmovaps	%ymm13, 800(%rsp)
	movl	480(%rsp), %eax
.Ltmp9626:
	movl	%eax, 680(%rsp)
.Ltmp9627:
	vmovaps	%ymm3, 576(%rsp)
.Ltmp9628:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_f20c129b493c5309be2c04d6b670f551(%rip), %rcx
	movq	%rdx, %rsi
	movq	1184(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9629:
.LBB34_117:
	.loc	5 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9630:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9631:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9632:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9633:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9634:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9635:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9636:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9637:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_118
.Ltmp9638:
.LBB34_130:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9639:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9640:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9641:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9642:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9643:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9644:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9645:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9646:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_131
.Ltmp9647:
.LBB34_120:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9648:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9649:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9650:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9651:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9652:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9653:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9654:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9655:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_121
.Ltmp9656:
.LBB34_123:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9657:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9658:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9659:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9660:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9661:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9662:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9663:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9664:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_124
.Ltmp9665:
.LBB34_208:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9666:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9667:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9668:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9669:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9670:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9671:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9672:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9673:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_131:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp9674:
.LBB34_132:
	leaq	.Lalloc_2b690e2c7763f11809942906fc2ca813(%rip), %rdx
	movq	32(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_626:
	vmovaps	320(%rsp), %ymm0
.Ltmp9675:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9676:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9677:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9678:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9679:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9680:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9681:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9682:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_118:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp9683:
.LBB34_119:
	leaq	.Lalloc_4a8785a681d008a9bfd0cd82628ea9cb(%rip), %rdx
	movq	384(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_627:
	vmovaps	320(%rsp), %ymm0
.Ltmp9684:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9685:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9686:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9687:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9688:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9689:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9690:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9691:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_121:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp9692:
.LBB34_122:
	leaq	.Lalloc_cbce7773ac40979e4ba2385da3aec116(%rip), %rdx
	movq	%r14, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_628:
	vmovaps	320(%rsp), %ymm0
.Ltmp9693:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9694:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9695:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9696:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9697:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9698:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9699:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9700:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_124:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp9701:
.LBB34_125:
	leaq	.Lalloc_ec0d48f73ebfc2755df5cedaa60b5c0a(%rip), %rdx
	movq	448(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_126:
	vmovaps	320(%rsp), %ymm0
.Ltmp9702:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9703:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9704:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9705:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9706:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9707:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9708:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9709:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_127
.Ltmp9710:
.LBB34_206:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9711:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9712:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9713:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9714:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9715:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9716:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9717:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9718:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_127:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp9719:
.LBB34_128:
	leaq	.Lalloc_2c461872bb652d4796cdcf89c28c82c8(%rip), %rdx
	movq	%r11, %rdi
	movq	32(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_114:
	vmovaps	320(%rsp), %ymm0
.Ltmp9720:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9721:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9722:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9723:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9724:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9725:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9726:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9727:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_115
.Ltmp9728:
.LBB34_205:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9729:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9730:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9731:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9732:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9733:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9734:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9735:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9736:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_115:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp9737:
.LBB34_116:
	leaq	.Lalloc_9fa421ae81817f58fcfc4a3243223891(%rip), %rdx
	movq	%r14, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB34_506:
	vmovaps	1568(%rsp), %ymm0
.Ltmp9738:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9739:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9740:
	leaq	.Lalloc_debdb702bca99cd8ca93ec127a5c320a(%rip), %rcx
	movq	184(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_433:
	movl	192(%rsp), %eax
	movl	%eax, 904(%rsp)
	vmovaps	%ymm9, 800(%rsp)
	movl	480(%rsp), %eax
	movl	%eax, 680(%rsp)
	vmovaps	%ymm5, 576(%rsp)
	leaq	.Lalloc_debdb702bca99cd8ca93ec127a5c320a(%rip), %rcx
	movq	184(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_642:
	vmovaps	1568(%rsp), %ymm0
.Ltmp9741:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9742:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
	movq	1408(%rsp), %rsi
.Ltmp9743:
	.loc	1 1999 61 is_stmt 1
	shlq	$3, %rsi
.Ltmp9744:
	.loc	5 443 13
	leaq	.Lalloc_0ad51c1dc139f477f06f604c0b5d4a59(%rip), %rcx
	movl	$256, %edx
	movq	320(%rsp), %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9745:
.LBB34_633:
	.loc	5 0 13 is_stmt 0
	movl	192(%rsp), %eax
	movl	%eax, 904(%rsp)
	vmovaps	%ymm9, 800(%rsp)
	movl	480(%rsp), %eax
	movl	%eax, 680(%rsp)
	vmovaps	%ymm5, 576(%rsp)
	movq	1376(%rsp), %rsi
.Ltmp9746:
	.loc	1 1999 61 is_stmt 1
	shlq	$3, %rsi
.Ltmp9747:
	.loc	5 443 13
	leaq	.Lalloc_0ad51c1dc139f477f06f604c0b5d4a59(%rip), %rcx
	movl	$256, %edx
	movq	384(%rsp), %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9748:
.LBB34_488:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm3, 6016(%rsp)
	vmovaps	%ymm9, 6048(%rsp)
	vmovaps	%ymm14, 6080(%rsp)
.Ltmp9749:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	%ymm15, 6784(%rsp)
	vmovaps	%ymm1, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9750:
.LBB34_489:
	leaq	.Lalloc_e7f134ea71d3d762bf72c0ef5353d5ff(%rip), %rcx
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9751:
.LBB34_654:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_5792a3affd2091ba045d8adc49b05663(%rip), %rcx
	xorl	%edi, %edi
	movq	%rbx, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9752:
.LBB34_508:
	.loc	5 0 13 is_stmt 0
	vmovaps	1568(%rsp), %ymm0
.Ltmp9753:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
.Ltmp9754:
	vmovaps	%ymm5, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9755:
	leaq	.Lalloc_01912ef845d3ed33a157086defa4d008(%rip), %rcx
	movq	88(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_436:
	movl	192(%rsp), %eax
	movl	%eax, 904(%rsp)
	vmovaps	%ymm9, 800(%rsp)
	movl	480(%rsp), %eax
	movl	%eax, 680(%rsp)
	vmovaps	%ymm5, 576(%rsp)
	leaq	.Lalloc_01912ef845d3ed33a157086defa4d008(%rip), %rcx
	movq	88(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB34_495:
	vmovaps	1568(%rsp), %ymm0
.Ltmp9756:
	vmovaps	%ymm0, 6016(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 6048(%rsp)
	vmovaps	1600(%rsp), %ymm0
	vmovaps	%ymm0, 6080(%rsp)
	vmovaps	5760(%rsp), %ymm0
.Ltmp9757:
	vmovaps	%ymm0, 6752(%rsp)
	vmovaps	1120(%rsp), %ymm0
	vmovaps	%ymm0, 6784(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 6816(%rsp)
	vmovaps	1056(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movl	24(%rsp), %eax
	movl	%eax, 904(%rsp)
	movl	28(%rsp), %eax
	movl	%eax, 680(%rsp)
.Ltmp9758:
.LBB34_496:
	leaq	.Lalloc_480b8302a9d65cc7541e43747df38ed7(%rip), %rcx
	movq	88(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9759:
.LBB34_622:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_34e4746889305f9c657ea46721b345d0(%rip), %rcx
	xorl	%edi, %edi
	movq	%r9, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9760:
.LBB34_187:
	.loc	5 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9761:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9762:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9763:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9764:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9765:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9766:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9767:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9768:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	jmp	.LBB34_188
.Ltmp9769:
.LBB34_251:
	.loc	1 0 0 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp9770:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9771:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9772:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9773:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9774:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9775:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9776:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9777:
	.loc	1 853 0
	vmovaps	%ymm15, 3936(%rsp)
.LBB34_188:
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp9778:
.LBB34_189:
	leaq	.Lalloc_b305c1483509cfb31fdec21ff8752674(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9779:
.LBB34_655:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_e8e5016f8f28771294ff88b89dfac103(%rip), %rcx
.Ltmp9780:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	88(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9781:
.LBB34_299:
	.loc	5 0 13
	movl	$1, %eax
	jmp	.LBB34_300
.LBB34_140:
	movl	$1, %eax
	jmp	.LBB34_141
.LBB34_35:
.Ltmp9782:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_22a7c5212c1e2b1f20d68ba89a9d6a79(%rip), %rcx
.Ltmp9783:
	.loc	5 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	88(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp9784:
.LBB34_366:
	.loc	5 0 13
	movl	$2, %eax
	jmp	.LBB34_300
.LBB34_164:
	movl	$2, %eax
	jmp	.LBB34_141
.LBB34_168:
	movl	$3, %eax
	jmp	.LBB34_141
.LBB34_370:
	movl	$3, %eax
	jmp	.LBB34_300
.LBB34_374:
	movl	$4, %eax
	jmp	.LBB34_300
.LBB34_172:
	movl	$4, %eax
	jmp	.LBB34_141
.LBB34_378:
	movl	$5, %eax
	jmp	.LBB34_300
.LBB34_176:
	movl	$5, %eax
	jmp	.LBB34_141
.LBB34_180:
	movl	$6, %eax
	jmp	.LBB34_141
.LBB34_382:
	movl	$6, %eax
	jmp	.LBB34_300
.LBB34_386:
	movl	$7, %eax
.LBB34_300:
	movq	%rax, 768(%rsp)
.LBB34_301:
.Ltmp9785:
	.loc	1 1403 42 is_stmt 1
	leaq	.Lalloc_a6feb40b34112df5f214f84424dd2c7c(%rip), %rdx
	movq	768(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9786:
.LBB34_184:
	.loc	1 0 42 is_stmt 0
	movl	$7, %eax
.LBB34_141:
	movq	%rax, 704(%rsp)
.LBB34_142:
	vmovaps	320(%rsp), %ymm0
.Ltmp9787:
	.loc	1 851 9 is_stmt 1
	vmovaps	%ymm0, 3168(%rsp)
.Ltmp9788:
	.loc	1 853 9
	vmovaps	%ymm6, 3072(%rsp)
	.loc	1 854 9
	vmovaps	%ymm7, 3136(%rsp)
	vmovaps	352(%rsp), %ymm0
.Ltmp9789:
	.loc	1 851 9
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	96(%rsp), %ymm0
.Ltmp9790:
	.loc	1 853 9
	vmovaps	%ymm0, 3200(%rsp)
	.loc	1 854 9
	vmovaps	%ymm14, 3264(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp9791:
	.loc	1 851 9
	vmovaps	%ymm0, 3904(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp9792:
	.loc	1 853 9
	vmovaps	%ymm0, 3808(%rsp)
	.loc	1 854 9
	vmovaps	%ymm11, 3872(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp9793:
	.loc	1 851 9
	vmovaps	%ymm0, 4032(%rsp)
.Ltmp9794:
	.loc	1 853 0
	vmovaps	%ymm2, 3936(%rsp)
	.loc	1 0 0 is_stmt 0
	vmovaps	%ymm8, 4000(%rsp)
.Ltmp9795:
	.loc	1 1403 42 is_stmt 1
	leaq	.Lalloc_a6feb40b34112df5f214f84424dd2c7c(%rip), %rdx
	movq	704(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp9796:
.LBB34_651:
	.loc	1 665 42
	leaq	.Lalloc_bac57976a2bdbfad4a3a85d5d1c7648c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp9797:
.LBB34_652:
	.loc	1 455 44
	leaq	.Lalloc_f0ee36f67d9a332211aa5518dd2ebfd5(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB34_653:
	.loc	1 456 44
	leaq	.Lalloc_33d4d33e0a850133578789055882dcf9(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp9798:
.Lfunc_end34:
	.size	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_, .Lfunc_end34-_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_
