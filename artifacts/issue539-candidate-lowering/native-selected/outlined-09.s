_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_:
.Lfunc_begin26:
	.loc	1 1338 0 is_stmt 1
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	subq	$48, %rsp
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -16
	.loc	1 970 23 prologue_end
	movq	8(%rsi), %rdx
.Ltmp4015:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB26_48
.Ltmp4016:
	.loc	1 0 0 is_stmt 0
	movq	192(%rsi), %rax
.Ltmp4017:
	movq	(%rsi), %rcx
.Ltmp4018:
	.loc	11 551 14 is_stmt 1
	vmovups	(%rdi), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp4019:
	.loc	5 580 12
	movq	%rdx, %r8
	subq	%rax, %r8
	jb	.LBB26_4
.Ltmp4020:
	.loc	5 451 16
	cmpq	$7, %r8
	jbe	.LBB26_3
.Ltmp4021:
	.loc	1 972 0
	leaq	32(%rdi), %r8
.Ltmp4022:
	.loc	11 551 14
	vmovups	(%r8), %ymm0
	vmovups	%ymm0, (%rcx,%rax,4)
.Ltmp4023:
	.loc	1 970 77
	leaq	(%rax,%rax), %rbx
.Ltmp4024:
	.loc	5 580 12
	movq	%rdx, %r8
	subq	%rbx, %r8
	jb	.LBB26_49
.Ltmp4025:
	.loc	5 451 16
	cmpq	$7, %r8
	jbe	.LBB26_3
.Ltmp4026:
	.loc	1 973 0
	leaq	64(%rdi), %r8
.Ltmp4027:
	.loc	11 551 14
	vmovups	(%r8), %ymm0
	vmovups	%ymm0, (%rcx,%rbx,4)
.Ltmp4028:
	.loc	1 970 77
	leaq	(%rax,%rax,2), %r9
.Ltmp4029:
	.loc	5 580 12
	movq	%rdx, %r8
	subq	%r9, %r8
	jb	.LBB26_51
.Ltmp4030:
	.loc	5 451 16
	cmpq	$7, %r8
	jbe	.LBB26_3
.Ltmp4031:
	.loc	1 974 0
	leaq	96(%rdi), %r8
.Ltmp4032:
	.loc	11 551 14
	vmovups	(%r8), %ymm0
	vmovups	%ymm0, (%rcx,%r9,4)
.Ltmp4033:
	.loc	1 970 77
	leaq	(,%rax,4), %r9
.Ltmp4034:
	.loc	5 580 12
	movq	%rdx, %r8
	subq	%r9, %r8
	jb	.LBB26_51
.Ltmp4035:
	.loc	5 451 16
	cmpq	$7, %r8
	jbe	.LBB26_3
.Ltmp4036:
	.loc	1 975 0
	leaq	128(%rdi), %r8
.Ltmp4037:
	.loc	11 551 14
	vmovups	(%r8), %ymm0
	vmovups	%ymm0, (%rcx,%r9,4)
.Ltmp4038:
	.loc	1 970 77
	leaq	(%rax,%rax,4), %r8
.Ltmp4039:
	.loc	5 580 12
	movq	%rdx, %r9
	subq	%r8, %r9
	jb	.LBB26_26
.Ltmp4040:
	.loc	5 451 16
	cmpq	$7, %r9
	jbe	.LBB26_53
.Ltmp4041:
	.loc	1 976 0
	leaq	160(%rdi), %r9
.Ltmp4042:
	.loc	11 551 14
	vmovups	(%r9), %ymm0
	vmovups	%ymm0, (%rcx,%r8,4)
.Ltmp4043:
	.loc	1 970 77
	leaq	(%rbx,%rbx,2), %r10
.Ltmp4044:
	.loc	5 580 12
	movq	%rdx, %r9
	subq	%r10, %r9
	jb	.LBB26_50
.Ltmp4045:
	.loc	5 451 16
	cmpq	$7, %r9
	jbe	.LBB26_53
.Ltmp4046:
	.loc	1 977 0
	leaq	192(%rdi), %r9
.Ltmp4047:
	.loc	11 551 14
	vmovups	(%r9), %ymm0
	vmovups	%ymm0, (%rcx,%r10,4)
.Ltmp4048:
	.loc	1 970 77
	leaq	(,%rax,8), %r9
	movq	%r9, %r10
	subq	%rax, %r10
.Ltmp4049:
	.loc	5 580 12
	movq	%rdx, %r11
	subq	%r10, %r11
	jb	.LBB26_50
.Ltmp4050:
	.loc	5 451 16
	cmpq	$7, %r11
	jbe	.LBB26_54
.Ltmp4051:
	.loc	1 978 0
	leaq	224(%rdi), %r11
.Ltmp4052:
	.loc	11 551 14
	vmovups	(%r11), %ymm0
	vmovups	%ymm0, (%rcx,%r10,4)
.Ltmp4053:
	.loc	5 580 12
	movq	%rdx, %r10
	subq	%r9, %r10
	jb	.LBB26_51
.Ltmp4054:
	.loc	5 451 16
	cmpq	$7, %r10
	jbe	.LBB26_55
.Ltmp4055:
	.loc	1 979 0
	leaq	256(%rdi), %r10
.Ltmp4056:
	.loc	11 551 14
	vmovups	(%r10), %ymm0
	vmovups	%ymm0, (%rcx,%r9,4)
.Ltmp4057:
	.loc	1 970 77
	leaq	(%rax,%rax,8), %r10
.Ltmp4058:
	.loc	5 580 12
	movq	%rdx, %r9
	subq	%r10, %r9
	jb	.LBB26_50
.Ltmp4059:
	.loc	5 451 16
	cmpq	$7, %r9
	jbe	.LBB26_53
.Ltmp4060:
	.loc	1 980 0
	leaq	288(%rdi), %r9
.Ltmp4061:
	.loc	11 551 14
	vmovups	(%r9), %ymm0
	vmovups	%ymm0, (%rcx,%r10,4)
.Ltmp4062:
	.loc	1 970 77
	leaq	(%rbx,%rbx,4), %r9
.Ltmp4063:
	.loc	5 580 12
	movq	%rdx, %r10
	subq	%r9, %r10
	jb	.LBB26_51
.Ltmp4064:
	.loc	5 451 16
	cmpq	$7, %r10
	jbe	.LBB26_55
.Ltmp4065:
	.loc	1 981 0
	leaq	320(%rdi), %r10
.Ltmp4066:
	.loc	11 551 14
	vmovups	(%r10), %ymm0
	vmovups	%ymm0, (%rcx,%r9,4)
.Ltmp4067:
	.loc	1 970 77
	leaq	(%rax,%r8,2), %r8
.Ltmp4068:
	.loc	5 580 12
	movq	%rdx, %rax
	subq	%r8, %rax
	jb	.LBB26_26
.Ltmp4069:
	.loc	5 451 16
	cmpq	$7, %rax
	jbe	.LBB26_25
.Ltmp4070:
	.loc	11 551 14
	vmovups	352(%rdi), %ymm0
	vmovups	%ymm0, (%rcx,%r8,4)
.Ltmp4071:
	.loc	1 1340 30
	movq	72(%rsi), %rdx
.Ltmp4072:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB26_48
.Ltmp4073:
	.loc	1 1340 30
	movq	64(%rsi), %rax
.Ltmp4074:
	.loc	11 551 14
	vmovups	640(%rdi), %ymm0
	vmovups	%ymm0, (%rax)
.Ltmp4075:
	.loc	1 1341 28
	movq	104(%rsi), %rdx
.Ltmp4076:
	.loc	5 451 16
	cmpq	$7, %rdx
	jbe	.LBB26_48
.Ltmp4077:
	.loc	1 1341 28
	movq	96(%rsi), %rax
.Ltmp4078:
	.loc	11 551 14
	vmovups	672(%rdi), %ymm0
	vmovups	%ymm0, (%rax)
.Ltmp4079:
	.loc	1 1342 28
	movq	136(%rsi), %rax
.Ltmp4080:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp4081:
	.loc	6 180 28
	je	.LBB26_38
.Ltmp4082:
	.loc	1 1342 0
	vmovss	384(%rdi), %xmm2
	vmovss	388(%rdi), %xmm0
	vmovss	392(%rdi), %xmm12
	vmovss	396(%rdi), %xmm9
	vmovss	400(%rdi), %xmm1
	vmovss	%xmm1, 44(%rsp)
	vmovss	404(%rdi), %xmm1
	vmovss	%xmm1, 40(%rsp)
	vmovss	408(%rdi), %xmm1
	vmovss	%xmm1, 32(%rsp)
	vmovss	412(%rdi), %xmm1
	vmovss	%xmm1, 20(%rsp)
	vmovss	448(%rdi), %xmm5
	vmovss	452(%rdi), %xmm1
	vmovss	456(%rdi), %xmm13
	vmovss	480(%rdi), %xmm3
.Ltmp4083:
	.loc	1 840 13
	vcvttss2si	%xmm3, %r9
	xorl	%edx, %edx
	vxorps	%xmm6, %xmm6, %xmm6
	vucomiss	%xmm6, %xmm3
	cmovbl	%edx, %r9d
.Ltmp4084:
	.loc	1 1342 0
	vmovss	460(%rdi), %xmm15
.Ltmp4085:
	.loc	1 840 13
	vucomiss	.LCPI26_0(%rip), %xmm3
.Ltmp4086:
	.loc	1 1342 0
	vmovss	464(%rdi), %xmm11
	vmovss	468(%rdi), %xmm3
	vmovss	%xmm3, 36(%rsp)
	vmovss	472(%rdi), %xmm3
	vmovss	%xmm3, 28(%rsp)
	movq	128(%rsi), %rcx
	movl	$-1, %r8d
.Ltmp4087:
	.loc	1 840 13
	cmoval	%r8d, %r9d
.Ltmp4088:
	.loc	1 1342 0
	vmovss	476(%rdi), %xmm3
	vmovss	%xmm3, 16(%rsp)
	vmovss	484(%rdi), %xmm7
	vmovss	488(%rdi), %xmm4
	vmovss	492(%rdi), %xmm3
	vmovss	496(%rdi), %xmm14
	vmovss	500(%rdi), %xmm10
	vmovss	504(%rdi), %xmm8
	vmovss	%xmm8, 24(%rsp)
	vmovss	508(%rdi), %xmm8
	vmovss	%xmm8, 12(%rsp)
.Ltmp4089:
	.loc	1 838 13
	vmovss	%xmm2, (%rcx)
	.loc	1 839 13
	vmovss	%xmm5, 8(%rcx)
	.loc	1 840 13
	movl	%r9d, 12(%rcx)
.Ltmp4090:
	.loc	7 1714 9
	cmpq	$1, %rax
.Ltmp4091:
	.loc	6 180 28
	je	.LBB26_38
.Ltmp4092:
	.loc	1 840 13
	vcvttss2si	%xmm7, %r9
	vucomiss	%xmm6, %xmm7
	cmovbl	%edx, %r9d
	vucomiss	.LCPI26_0(%rip), %xmm7
	cmoval	%r8d, %r9d
	.loc	1 838 13
	vmovss	%xmm0, 16(%rcx)
	.loc	1 839 13
	vmovss	%xmm1, 24(%rcx)
	.loc	1 840 13
	movl	%r9d, 28(%rcx)
.Ltmp4093:
	.loc	7 1714 9
	cmpq	$2, %rax
.Ltmp4094:
	.loc	6 180 28
	je	.LBB26_38
.Ltmp4095:
	.loc	1 840 13
	vcvttss2si	%xmm4, %r9
	xorl	%edx, %edx
	vxorps	%xmm0, %xmm0, %xmm0
	vucomiss	%xmm0, %xmm4
	cmovbl	%edx, %r9d
	vucomiss	.LCPI26_0(%rip), %xmm4
	cmoval	%r8d, %r9d
	.loc	1 838 13
	vmovss	%xmm12, 32(%rcx)
	.loc	1 839 13
	vmovss	%xmm13, 40(%rcx)
	.loc	1 840 13
	movl	%r9d, 44(%rcx)
.Ltmp4096:
	.loc	7 1714 9
	cmpq	$3, %rax
.Ltmp4097:
	.loc	6 180 28
	je	.LBB26_38
.Ltmp4098:
	.loc	1 840 13
	vcvttss2si	%xmm3, %r9
	vucomiss	%xmm0, %xmm3
	cmovbl	%edx, %r9d
	vucomiss	.LCPI26_0(%rip), %xmm3
	cmoval	%r8d, %r9d
	.loc	1 838 13
	vmovss	%xmm9, 48(%rcx)
	.loc	1 839 13
	vmovss	%xmm15, 56(%rcx)
	.loc	1 840 13
	movl	%r9d, 60(%rcx)
.Ltmp4099:
	.loc	7 1714 9
	cmpq	$4, %rax
.Ltmp4100:
	.loc	6 180 28
	je	.LBB26_38
.Ltmp4101:
	.loc	1 840 13
	vcvttss2si	%xmm14, %r9
	xorl	%edx, %edx
	vucomiss	%xmm0, %xmm14
	cmovbl	%edx, %r9d
	vucomiss	.LCPI26_0(%rip), %xmm14
	cmoval	%r8d, %r9d
	vmovss	44(%rsp), %xmm1
	.loc	1 838 13
	vmovss	%xmm1, 64(%rcx)
	.loc	1 839 13
	vmovss	%xmm11, 72(%rcx)
	.loc	1 840 13
	movl	%r9d, 76(%rcx)
.Ltmp4102:
	.loc	7 1714 9
	cmpq	$5, %rax
.Ltmp4103:
	.loc	6 180 28
	je	.LBB26_38
.Ltmp4104:
	.loc	1 840 13
	vcvttss2si	%xmm10, %r9
	vucomiss	%xmm0, %xmm10
	cmovbl	%edx, %r9d
	vucomiss	.LCPI26_0(%rip), %xmm10
	cmoval	%r8d, %r9d
	vmovss	40(%rsp), %xmm0
	.loc	1 838 13
	vmovss	%xmm0, 80(%rcx)
	vmovss	36(%rsp), %xmm0
	.loc	1 839 13
	vmovss	%xmm0, 88(%rcx)
	.loc	1 840 13
	movl	%r9d, 92(%rcx)
.Ltmp4105:
	.loc	7 1714 9
	cmpq	$6, %rax
.Ltmp4106:
	.loc	6 180 28
	je	.LBB26_38
.Ltmp4107:
	.loc	6 0 28 is_stmt 0
	vmovss	24(%rsp), %xmm1
.Ltmp4108:
	.loc	1 840 13 is_stmt 1
	vcvttss2si	%xmm1, %r9
	xorl	%edx, %edx
	vxorps	%xmm0, %xmm0, %xmm0
	vucomiss	%xmm0, %xmm1
	cmovbl	%edx, %r9d
	vucomiss	.LCPI26_0(%rip), %xmm1
	cmoval	%r8d, %r9d
	vmovss	32(%rsp), %xmm1
	.loc	1 838 13
	vmovss	%xmm1, 96(%rcx)
	vmovss	28(%rsp), %xmm1
	.loc	1 839 13
	vmovss	%xmm1, 104(%rcx)
	.loc	1 840 13
	movl	%r9d, 108(%rcx)
.Ltmp4109:
	.loc	7 1714 9
	cmpq	$7, %rax
.Ltmp4110:
	.loc	6 180 28
	je	.LBB26_38
.Ltmp4111:
	.loc	6 0 28 is_stmt 0
	vmovss	12(%rsp), %xmm1
.Ltmp4112:
	.loc	1 840 13 is_stmt 1
	vcvttss2si	%xmm1, %r9
	vucomiss	%xmm0, %xmm1
	cmovbl	%edx, %r9d
	vucomiss	.LCPI26_0(%rip), %xmm1
	cmoval	%r8d, %r9d
	vmovss	20(%rsp), %xmm0
	.loc	1 838 13
	vmovss	%xmm0, 112(%rcx)
	vmovss	16(%rsp), %xmm0
	.loc	1 839 13
	vmovss	%xmm0, 120(%rcx)
	.loc	1 840 13
	movl	%r9d, 124(%rcx)
.Ltmp4113:
	.loc	7 1714 9
	cmpq	$8, %rax
.Ltmp4114:
	.loc	6 180 28
	jne	.LBB26_52
.Ltmp4115:
.LBB26_38:
	.loc	1 1343 30
	movq	152(%rsi), %rax
.Ltmp4116:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp4117:
	.loc	6 180 28
	je	.LBB26_47
.Ltmp4118:
	.loc	1 1343 0
	vmovss	512(%rdi), %xmm2
	vmovss	516(%rdi), %xmm0
	vmovss	520(%rdi), %xmm12
	vmovss	524(%rdi), %xmm9
	vmovss	528(%rdi), %xmm1
	vmovss	%xmm1, 44(%rsp)
	vmovss	532(%rdi), %xmm1
	vmovss	%xmm1, 40(%rsp)
	vmovss	536(%rdi), %xmm1
	vmovss	%xmm1, 32(%rsp)
	vmovss	540(%rdi), %xmm1
	vmovss	%xmm1, 20(%rsp)
	vmovss	576(%rdi), %xmm5
	vmovss	580(%rdi), %xmm1
	vmovss	584(%rdi), %xmm13
	vmovss	608(%rdi), %xmm3
.Ltmp4119:
	.loc	1 840 13
	vcvttss2si	%xmm3, %r8
	xorl	%edx, %edx
	vxorps	%xmm6, %xmm6, %xmm6
	vucomiss	%xmm6, %xmm3
	cmovbl	%edx, %r8d
.Ltmp4120:
	.loc	1 1343 0
	vmovss	588(%rdi), %xmm15
.Ltmp4121:
	.loc	1 840 13
	vucomiss	.LCPI26_0(%rip), %xmm3
.Ltmp4122:
	.loc	1 1343 0
	vmovss	592(%rdi), %xmm11
	vmovss	596(%rdi), %xmm3
	vmovss	%xmm3, 36(%rsp)
	vmovss	600(%rdi), %xmm3
	vmovss	%xmm3, 28(%rsp)
	movq	144(%rsi), %rcx
	movl	$-1, %esi
.Ltmp4123:
	.loc	1 840 13
	cmoval	%esi, %r8d
.Ltmp4124:
	.loc	1 1343 0
	vmovss	604(%rdi), %xmm3
	vmovss	%xmm3, 16(%rsp)
	vmovss	612(%rdi), %xmm7
	vmovss	616(%rdi), %xmm4
	vmovss	620(%rdi), %xmm3
	vmovss	624(%rdi), %xmm14
	vmovss	628(%rdi), %xmm10
	vmovss	632(%rdi), %xmm8
	vmovss	%xmm8, 24(%rsp)
	vmovss	636(%rdi), %xmm8
	vmovss	%xmm8, 12(%rsp)
.Ltmp4125:
	.loc	1 838 13
	vmovss	%xmm2, (%rcx)
	.loc	1 839 13
	vmovss	%xmm5, 8(%rcx)
	.loc	1 840 13
	movl	%r8d, 12(%rcx)
.Ltmp4126:
	.loc	7 1714 9
	cmpq	$1, %rax
.Ltmp4127:
	.loc	6 180 28
	je	.LBB26_47
.Ltmp4128:
	.loc	1 840 13
	vcvttss2si	%xmm7, %rdi
	vucomiss	%xmm6, %xmm7
	cmovbl	%edx, %edi
	vucomiss	.LCPI26_0(%rip), %xmm7
	cmoval	%esi, %edi
	.loc	1 838 13
	vmovss	%xmm0, 16(%rcx)
	.loc	1 839 13
	vmovss	%xmm1, 24(%rcx)
	.loc	1 840 13
	movl	%edi, 28(%rcx)
.Ltmp4129:
	.loc	7 1714 9
	cmpq	$2, %rax
.Ltmp4130:
	.loc	6 180 28
	je	.LBB26_47
.Ltmp4131:
	.loc	1 840 13
	vcvttss2si	%xmm4, %rdi
	xorl	%edx, %edx
	vxorps	%xmm0, %xmm0, %xmm0
	vucomiss	%xmm0, %xmm4
	cmovbl	%edx, %edi
	vucomiss	.LCPI26_0(%rip), %xmm4
	cmoval	%esi, %edi
	.loc	1 838 13
	vmovss	%xmm12, 32(%rcx)
	.loc	1 839 13
	vmovss	%xmm13, 40(%rcx)
	.loc	1 840 13
	movl	%edi, 44(%rcx)
.Ltmp4132:
	.loc	7 1714 9
	cmpq	$3, %rax
.Ltmp4133:
	.loc	6 180 28
	je	.LBB26_47
.Ltmp4134:
	.loc	1 840 13
	vcvttss2si	%xmm3, %rdi
	vucomiss	%xmm0, %xmm3
	cmovbl	%edx, %edi
	vucomiss	.LCPI26_0(%rip), %xmm3
	cmoval	%esi, %edi
	.loc	1 838 13
	vmovss	%xmm9, 48(%rcx)
	.loc	1 839 13
	vmovss	%xmm15, 56(%rcx)
	.loc	1 840 13
	movl	%edi, 60(%rcx)
.Ltmp4135:
	.loc	7 1714 9
	cmpq	$4, %rax
.Ltmp4136:
	.loc	6 180 28
	je	.LBB26_47
.Ltmp4137:
	.loc	1 840 13
	vcvttss2si	%xmm14, %rdi
	xorl	%edx, %edx
	vucomiss	%xmm0, %xmm14
	cmovbl	%edx, %edi
	vucomiss	.LCPI26_0(%rip), %xmm14
	cmoval	%esi, %edi
	vmovss	44(%rsp), %xmm1
	.loc	1 838 13
	vmovss	%xmm1, 64(%rcx)
	.loc	1 839 13
	vmovss	%xmm11, 72(%rcx)
	.loc	1 840 13
	movl	%edi, 76(%rcx)
.Ltmp4138:
	.loc	7 1714 9
	cmpq	$5, %rax
.Ltmp4139:
	.loc	6 180 28
	je	.LBB26_47
.Ltmp4140:
	.loc	1 840 13
	vcvttss2si	%xmm10, %rdi
	vucomiss	%xmm0, %xmm10
	cmovbl	%edx, %edi
	vucomiss	.LCPI26_0(%rip), %xmm10
	cmoval	%esi, %edi
	vmovss	40(%rsp), %xmm0
	.loc	1 838 13
	vmovss	%xmm0, 80(%rcx)
	vmovss	36(%rsp), %xmm0
	.loc	1 839 13
	vmovss	%xmm0, 88(%rcx)
	.loc	1 840 13
	movl	%edi, 92(%rcx)
.Ltmp4141:
	.loc	7 1714 9
	cmpq	$6, %rax
.Ltmp4142:
	.loc	6 180 28
	je	.LBB26_47
.Ltmp4143:
	.loc	6 0 28 is_stmt 0
	vmovss	24(%rsp), %xmm1
.Ltmp4144:
	.loc	1 840 13 is_stmt 1
	vcvttss2si	%xmm1, %rdi
	xorl	%edx, %edx
	vxorps	%xmm0, %xmm0, %xmm0
	vucomiss	%xmm0, %xmm1
	cmovbl	%edx, %edi
	vucomiss	.LCPI26_0(%rip), %xmm1
	cmoval	%esi, %edi
	vmovss	32(%rsp), %xmm1
	.loc	1 838 13
	vmovss	%xmm1, 96(%rcx)
	vmovss	28(%rsp), %xmm1
	.loc	1 839 13
	vmovss	%xmm1, 104(%rcx)
	.loc	1 840 13
	movl	%edi, 108(%rcx)
.Ltmp4145:
	.loc	7 1714 9
	cmpq	$7, %rax
.Ltmp4146:
	.loc	6 180 28
	je	.LBB26_47
.Ltmp4147:
	.loc	6 0 28 is_stmt 0
	vmovss	12(%rsp), %xmm1
.Ltmp4148:
	.loc	1 840 13 is_stmt 1
	vcvttss2si	%xmm1, %rdi
	vucomiss	%xmm0, %xmm1
	cmovbl	%edx, %edi
	vucomiss	.LCPI26_0(%rip), %xmm1
	cmoval	%esi, %edi
	vmovss	20(%rsp), %xmm0
	.loc	1 838 13
	vmovss	%xmm0, 112(%rcx)
	vmovss	16(%rsp), %xmm0
	.loc	1 839 13
	vmovss	%xmm0, 120(%rcx)
	.loc	1 840 13
	movl	%edi, 124(%rcx)
.Ltmp4149:
	.loc	7 1714 9
	cmpq	$8, %rax
.Ltmp4150:
	.loc	6 180 28
	jne	.LBB26_52
.Ltmp4151:
.LBB26_47:
	.loc	1 1344 6 epilogue_begin
	addq	$48, %rsp
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.LBB26_3:
	.cfi_def_cfa_offset 64
.Ltmp4152:
	.loc	5 456 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4153:
.LBB26_51:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%r9, %rdi
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4154:
.LBB26_53:
	.loc	5 456 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r9, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4155:
.LBB26_50:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%r10, %rdi
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4156:
.LBB26_48:
	.loc	1 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB26_26:
.Ltmp4157:
	.loc	5 581 13 is_stmt 1
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%r8, %rdi
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4158:
.LBB26_55:
	.loc	5 456 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r10, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4159:
.LBB26_4:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%rax, %rdi
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4160:
.LBB26_49:
	.loc	5 581 13
	leaq	.Lalloc_2607b4735793736b813e4d78bd281000(%rip), %rcx
	movq	%rbx, %rdi
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4161:
.LBB26_54:
	.loc	5 456 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r11, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4162:
.LBB26_25:
	.loc	5 456 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4163:
.LBB26_52:
	.loc	1 838 28
	leaq	.Lalloc_4e7cf9526cbf9c5bb2e965885a8a18fa(%rip), %rdx
	movl	$8, %edi
	movl	$8, %esi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4164:
.Lfunc_end26:
	.size	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_, .Lfunc_end26-_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
