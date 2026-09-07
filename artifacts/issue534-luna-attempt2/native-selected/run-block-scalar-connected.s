_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_:
.Lfunc_begin21:
	.loc	6 676 0
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
	subq	$280, %rsp
	.cfi_def_cfa_offset 336
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rcx, 152(%rsp)
	movq	%rdx, %r11
	movq	%rsi, 88(%rsp)
	movq	%rdi, %rbx
	movq	336(%rsp), %r10
.Ltmp1087:
	.loc	6 685 25 prologue_end
	movl	1220(%rdi), %r14d
.Ltmp1088:
	.loc	8 1078 5
	cmpq	%r14, %r10
	movq	%r14, %rsi
	cmovbq	%r10, %rsi
.Ltmp1089:
	.loc	6 686 12
	testq	%rsi, %rsi
	movq	%rsi, 200(%rsp)
	movq	%r8, 216(%rsp)
	movq	%rdx, 208(%rsp)
	je	.LBB21_495
.Ltmp1090:
	.loc	18 1161 15
	movq	(%r9), %rax
	movq	%rax, 64(%rsp)
	testq	%rax, %rax
	.loc	18 1161 9 is_stmt 0
	je	.LBB21_8
.Ltmp1091:
	.loc	18 1162 29 is_stmt 1
	movq	8(%r9), %rdx
	cmpq	%rdx, %rsi
.Ltmp1092:
	.loc	15 1050 16
	ja	.LBB21_787
.Ltmp1093:
	.loc	18 1162 29
	movq	24(%r9), %rdx
.Ltmp1094:
	.file	25 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/slice/index.rs"
	.loc	25 438 16
	cmpq	%rdx, %rsi
	ja	.LBB21_791
.Ltmp1095:
	.loc	18 1162 29
	movq	16(%r9), %rax
	movq	%rax, 96(%rsp)
	cmpq	%r11, %rsi
.Ltmp1096:
	.loc	15 1050 16
	ja	.LBB21_9
.Ltmp1097:
.LBB21_5:
	.loc	25 451 16
	cmpq	%r8, %rsi
	ja	.LBB21_788
.Ltmp1098:
	.loc	21 61 8
	movl	136(%rbx), %eax
	cmpl	200(%rbx), %eax
	sete	%al
	movb	$2, %cl
	subb	%al, %cl
	xorl	%r10d, %r10d
	cmpl	$1, 68(%rbx)
	movzbl	%cl, %eax
	cmovel	%r10d, %eax
	movl	%eax, 48(%rsp)
.Ltmp1099:
	.loc	6 736 31
	movq	112(%rbx), %rdx
	.loc	6 741 31
	movq	176(%rbx), %r12
.Ltmp1100:
	.loc	18 1161 15
	cmpq	$0, 64(%rsp)
.Ltmp1101:
	.loc	18 1039 9
	je	.LBB21_10
.Ltmp1102:
	.loc	18 0 9 is_stmt 0
	movq	%rsi, %r10
	jmp	.LBB21_11
.LBB21_8:
	cmpq	%r11, %rsi
.Ltmp1103:
	.loc	15 1050 16 is_stmt 1
	jbe	.LBB21_5
.Ltmp1104:
.LBB21_9:
	.loc	25 456 13
	leaq	.Lalloc_dd5f55065f566218c9f31cb2a4357231(%rip), %rcx
	xorl	%edi, %edi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp1105:
.LBB21_10:
	.loc	25 0 13 is_stmt 0
	movl	$4, %eax
	movq	%rax, 96(%rsp)
	movq	%rax, 64(%rsp)
.LBB21_11:
.Ltmp1106:
	movq	104(%rbx), %rbp
	movq	120(%rbx), %rax
	movq	%rax, 40(%rsp)
	movq	128(%rbx), %rax
	movq	%rax, 8(%rsp)
	movq	168(%rbx), %rax
	movq	%rax, 24(%rsp)
	movq	184(%rbx), %rax
	movq	%rax, 32(%rsp)
	movq	192(%rbx), %rax
	movq	%rax, 16(%rsp)
	movl	1212(%rbx), %ecx
	movl	1216(%rbx), %edi
.Ltmp1107:
	.loc	21 353 16 is_stmt 1
	movl	1208(%rbx), %r13d
	cmpq	%r12, %rdx
	movq	%rdx, 72(%rsp)
	movq	%rbp, 80(%rsp)
	movq	%r13, 56(%rsp)
	movq	%r9, 120(%rsp)
	movq	%r14, 144(%rsp)
	jbe	.LBB21_111
	.loc	21 0 16 is_stmt 0
	movq	%r10, 176(%rsp)
.Ltmp1108:
	.loc	25 438 16 is_stmt 1
	movq	%r10, %r9
	negq	%r9
	movq	%rsi, %rax
	negq	%rax
	movl	%r13d, %r8d
	subl	%edi, %r8d
	movl	$1, %r15d
	movzbl	48(%rsp), %esi
	movl	%esi, 112(%rsp)
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$8388608, %r11d
	xorl	%r10d, %r10d
	movq	152(%rsp), %rsi
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_14
.Ltmp1109:
	.loc	25 0 16 is_stmt 0
.Ltmp1110:
	.p2align	4
.LBB21_13:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp1111:
	.file	26 "/home/bl/misofm/engine-gate-detector-access" "crates/lane/src/scalar.rs"
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp1112:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1113:
	.loc	26 92 9
	vmulss	768(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1114:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1115:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp1116:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp1117:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp1118:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp1119:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp1120:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp1121:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp1122:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm15
.Ltmp1123:
	.loc	26 61 9
	vaddss	%xmm2, %xmm15, %xmm2
.Ltmp1124:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp1125:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp1126:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp1127:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp1128:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp1129:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp1130:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1131:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm4, %xmm2
.Ltmp1132:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
.Ltmp1133:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp1134:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp1135:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
.Ltmp1136:
	.loc	26 161 24
	vblendvps	%xmm9, %xmm2, %xmm4, %xmm2
.Ltmp1137:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm4
.Ltmp1138:
	.loc	26 66 9
	vsubss	%xmm4, %xmm3, %xmm3
.Ltmp1139:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp1140:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp1141:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1142:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp1143:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1144:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp1145:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1146:
	.loc	26 61 9
	vaddss	%xmm6, %xmm15, %xmm6
.Ltmp1147:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1148:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp1149:
	.loc	26 178 22
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp1150:
	.loc	7 1244 18
	vmovd	%xmm4, %edx
.Ltmp1151:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp1152:
	.loc	7 1291 18
	vmovd	%edx, %xmm4
.Ltmp1153:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp1154:
	.loc	21 510 5
	vmovss	%xmm10, 940(%rbx)
	vmovaps	128(%rsp), %xmm6
.Ltmp1155:
	.loc	26 71 9
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp1156:
	.loc	26 161 24
	vcmpneqss	%xmm5, %xmm10, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	vcmpnltss	780(%rbx), %xmm5, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	movq	88(%rsp), %rdx
.Ltmp1157:
	.loc	26 56 9
	vmovss	%xmm2, -4(%rdx,%r15,4)
.Ltmp1158:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rsi,%r15,4)
.Ltmp1159:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rax,%r15), %rdx
	incq	%rdx
	incq	%r15
	cmpq	$1, %rdx
	movq	72(%rsp), %rdx
	movq	80(%rsp), %rbp
.Ltmp1160:
	.loc	11 900 12
	je	.LBB21_494
.LBB21_14:
.Ltmp1161:
	.loc	15 971 17
	leaq	(%rax,%r15), %rdi
.Ltmp1162:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_775
.Ltmp1163:
	.loc	21 0 0 is_stmt 0
	leal	(%r15,%r13), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp1164:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp1165:
	.loc	25 0 16 is_stmt 0
	movq	88(%rsp), %r13
.Ltmp1166:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r13,%r15,4), %xmm2
.Ltmp1167:
	.loc	26 56 9
	vmovss	%xmm2, (%rbp,%rdi,4)
.Ltmp1168:
	.loc	25 451 16
	cmpq	%rdi, %r12
	jbe	.LBB21_782
.Ltmp1169:
	.loc	21 0 0 is_stmt 0
	leaq	(%r9,%r15), %r13
.Ltmp1170:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rsi,%r15,4), %xmm2
	movq	24(%rsp), %r14
.Ltmp1171:
	.loc	26 56 9
	vmovss	%xmm2, (%r14,%rdi,4)
.Ltmp1172:
	.loc	25 438 16
	cmpq	$1, %r13
	je	.LBB21_776
.Ltmp1173:
	.loc	25 451 16
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_711
.Ltmp1174:
	.loc	25 0 16 is_stmt 0
	movq	64(%rsp), %r13
.Ltmp1175:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r13,%r15,4), %xmm2
	movq	40(%rsp), %r13
.Ltmp1176:
	.loc	26 56 9
	vmovss	%xmm2, (%r13,%rdi,4)
.Ltmp1177:
	.loc	25 451 16
	cmpq	%rdi, 16(%rsp)
	jbe	.LBB21_712
.Ltmp1178:
	.loc	25 0 16 is_stmt 0
	movq	96(%rsp), %r13
.Ltmp1179:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r13,%r15,4), %xmm2
	movq	32(%rsp), %r14
.Ltmp1180:
	.loc	26 56 9
	vmovss	%xmm2, (%r14,%rdi,4)
.Ltmp1181:
	.loc	21 370 21
	leal	(%r8,%r15), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp1182:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp1183:
	.loc	25 438 16 is_stmt 0
	cmpq	%rdi, %r12
	jbe	.LBB21_783
.Ltmp1184:
	.loc	26 51 9 is_stmt 1
	vmovss	(%rbp,%rdi,4), %xmm4
	movq	24(%rsp), %rdx
.Ltmp1185:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm2
	movq	56(%rsp), %rdx
.Ltmp1186:
	.loc	21 0 0
	addl	%r15d, %edx
	movl	136(%rbx), %r13d
	movl	200(%rbx), %ebp
	notl	%r13d
	addl	%edx, %r13d
	andl	%ecx, %r13d
	notl	%ebp
	addl	%edx, %ebp
	andl	%ecx, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 48(%rsp)
	vmovaps	%xmm2, 128(%rsp)
	je	.LBB21_27
	cmpl	$1, 112(%rsp)
	movq	16(%rsp), %rdx
	movl	$841731191, %r14d
	jne	.LBB21_30
	.loc	21 0 0 is_stmt 0
	cmpq	%r13, 8(%rsp)
	jbe	.LBB21_801
.Ltmp1187:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, %rdx
	jbe	.LBB21_804
.Ltmp1188:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm6
	movq	32(%rsp), %rdx
.Ltmp1189:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm15
	vmovaps	%xmm15, %xmm10
	vmovaps	%xmm6, %xmm12
	jmp	.LBB21_35
.Ltmp1190:
	.loc	21 0 33 is_stmt 0
.Ltmp1191:
	.p2align	4
.LBB21_27:
	cmpq	%r13, 8(%rsp)
	movq	16(%rsp), %rdx
	movl	$841731191, %r14d
.Ltmp1192:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_802
.Ltmp1193:
	.loc	21 237 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_798
.Ltmp1194:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm10
	movq	32(%rsp), %rdx
.Ltmp1195:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm15
	vmovaps	%xmm10, %xmm12
.Ltmp1196:
	.loc	26 51 9
	jmp	.LBB21_35
.Ltmp1197:
	.loc	26 0 9 is_stmt 0
.Ltmp1198:
	.p2align	4
.LBB21_30:
	cmpq	%r13, 8(%rsp)
.Ltmp1199:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_803
	.loc	21 267 33
	cmpq	%r13, %rdx
	jbe	.LBB21_799
	.loc	21 268 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_800
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	vmovss	(%rdx,%r13,4), %xmm12
	movq	32(%rsp), %rdi
	vmovss	(%rdi,%r13,4), %xmm10
	.loc	21 268 33 is_stmt 1
	vmovss	(%rdi,%rbp,4), %xmm15
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm6
.Ltmp1200:
.LBB21_35:
	.loc	21 439 26
	vmovss	804(%rbx), %xmm9
.Ltmp1201:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp1202:
	.loc	21 441 44
	vmovss	800(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp1203:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm8, %xmm11
.Ltmp1204:
	.loc	26 161 24
	jne	.LBB21_38
	jp	.LBB21_38
.Ltmp1205:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm11
.LBB21_38:
.Ltmp1206:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_41
	jp	.LBB21_41
.Ltmp1207:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_41:
.Ltmp1208:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm9
.Ltmp1209:
	.loc	26 161 24
	jbe	.LBB21_43
.Ltmp1210:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm8
.LBB21_43:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 800(%rbx)
.Ltmp1211:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1212:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp1213:
	.loc	21 448 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp1214:
	.loc	21 439 26
	vmovss	820(%rbx), %xmm11
.Ltmp1215:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp1216:
	.loc	21 441 27
	vmovss	808(%rbx), %xmm9
	.loc	21 441 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp1217:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp1218:
	.loc	26 161 24
	jne	.LBB21_46
	jp	.LBB21_46
.Ltmp1219:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_46:
.Ltmp1220:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_49
	jp	.LBB21_49
.Ltmp1221:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_49:
.Ltmp1222:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp1223:
	.loc	26 161 24
	jbe	.LBB21_51
.Ltmp1224:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_51:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp1225:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1226:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp1227:
	.loc	21 448 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp1228:
	.loc	21 439 26
	vmovss	836(%rbx), %xmm13
.Ltmp1229:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1230:
	.loc	21 441 27
	vmovss	824(%rbx), %xmm11
	.loc	21 441 44 is_stmt 0
	vmovss	832(%rbx), %xmm3
.Ltmp1231:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm14
.Ltmp1232:
	.loc	26 161 24
	jne	.LBB21_54
	jp	.LBB21_54
.Ltmp1233:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_54:
.Ltmp1234:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_57
	jp	.LBB21_57
.Ltmp1235:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_57:
.Ltmp1236:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp1237:
	.loc	26 161 24
	jbe	.LBB21_59
.Ltmp1238:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm11
.LBB21_59:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 824(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp1239:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1240:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1241:
	.loc	21 448 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp1242:
	.loc	21 439 26
	vmovss	852(%rbx), %xmm14
.Ltmp1243:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1244:
	.loc	21 441 44
	vmovss	848(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp1245:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp1246:
	.loc	26 161 24
	jne	.LBB21_62
	jp	.LBB21_62
.Ltmp1247:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_62:
.Ltmp1248:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_65
	jp	.LBB21_65
.Ltmp1249:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_65:
.Ltmp1250:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp1251:
	.loc	26 161 24
	jbe	.LBB21_67
.Ltmp1252:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_67:
.Ltmp1253:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1254:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp1255:
	.loc	21 442 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp1256:
	.loc	26 103 24
	vandps	%xmm7, %xmm12, %xmm3
.Ltmp1257:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm10, %xmm7
.Ltmp1258:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp1259:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp1260:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp1261:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp1262:
	.loc	21 448 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp1263:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm2
.Ltmp1264:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1265:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp1266:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm2
.Ltmp1267:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm10
.Ltmp1268:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm2
.Ltmp1269:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm3
.Ltmp1270:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1271:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp1272:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp1273:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1274:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1275:
	.loc	26 161 24
	cmovbel	%r14d, %edx
.Ltmp1276:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1277:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1278:
	.loc	26 161 24
	cmovbel	%r11d, %edx
.Ltmp1279:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1280:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1281:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1282:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1283:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm7
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp1284:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1285:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1286:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1287:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1288:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1289:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1290:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1291:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1292:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp1293:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1294:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp1295:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1296:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1297:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1298:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1299:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm10
.Ltmp1300:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp1301:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm10
.Ltmp1302:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp1303:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm10
.Ltmp1304:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1305:
	.loc	26 149 9
	movl	%r13d, %edx
.Ltmp1306:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm3
.Ltmp1307:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm3
.Ltmp1308:
	.loc	26 149 9
	notl	%edx
.Ltmp1309:
	.loc	26 139 9
	cmovbel	%r10d, %edx
.Ltmp1310:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm2
.Ltmp1311:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1312:
	.loc	21 491 9
	vmovss	752(%rbx), %xmm12
.Ltmp1313:
	.loc	26 144 9
	cmoval	%r13d, %ebp
.Ltmp1314:
	.loc	26 139 9
	cmovbel	%r10d, %edx
.Ltmp1315:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_69
.Ltmp1316:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_69:
.Ltmp1317:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_71
.Ltmp1318:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_71:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm3
.Ltmp1319:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 498 5
	movl	%edx, 856(%rbx)
.Ltmp1320:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1321:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm10, %xmm7
.Ltmp1322:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm7, %xmm2, %xmm2
.Ltmp1323:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm7
	vxorps	%xmm7, %xmm11, %xmm7
.Ltmp1324:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm2, %xmm2
.Ltmp1325:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm7
	vandps	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_73
.Ltmp1326:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_73:
.Ltmp1327:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp1328:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp1329:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1330:
	.loc	26 92 9
	vmulss	744(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1331:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1332:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp1333:
	.loc	21 510 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp1334:
	.loc	21 439 26
	vmovss	880(%rbx), %xmm11
.Ltmp1335:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp1336:
	.loc	21 441 44
	vmovss	876(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp1337:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp1338:
	.loc	26 161 24
	jne	.LBB21_76
	jp	.LBB21_76
.Ltmp1339:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_76:
.Ltmp1340:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_79
	jp	.LBB21_79
.Ltmp1341:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_79:
.Ltmp1342:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp1343:
	.loc	26 161 24
	jbe	.LBB21_81
.Ltmp1344:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_81:
.Ltmp1345:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm5, %xmm9
.Ltmp1346:
	.loc	21 442 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp1347:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1348:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp1349:
	.loc	21 448 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp1350:
	.loc	21 439 26
	vmovss	896(%rbx), %xmm12
.Ltmp1351:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp1352:
	.loc	21 441 44
	vmovss	892(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp1353:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp1354:
	.loc	26 161 24
	jne	.LBB21_84
	jp	.LBB21_84
.Ltmp1355:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_84:
.Ltmp1356:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_87
	jp	.LBB21_87
.Ltmp1357:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_87:
.Ltmp1358:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm12
.Ltmp1359:
	.loc	26 161 24
	jbe	.LBB21_89
.Ltmp1360:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_89:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp1361:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp1362:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp1363:
	.loc	21 448 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp1364:
	.loc	21 439 26
	vmovss	912(%rbx), %xmm13
.Ltmp1365:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1366:
	.loc	21 441 44
	vmovss	908(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp1367:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp1368:
	.loc	26 161 24
	jne	.LBB21_92
	jp	.LBB21_92
.Ltmp1369:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_92:
.Ltmp1370:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_95
	jp	.LBB21_95
.Ltmp1371:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_95:
.Ltmp1372:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp1373:
	.loc	26 161 24
	jbe	.LBB21_97
.Ltmp1374:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_97:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp1375:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1376:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1377:
	.loc	21 448 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp1378:
	.loc	21 439 26
	vmovss	928(%rbx), %xmm14
.Ltmp1379:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1380:
	.loc	21 441 44
	vmovss	924(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp1381:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp1382:
	.loc	26 161 24
	jne	.LBB21_100
	jp	.LBB21_100
.Ltmp1383:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_100:
.Ltmp1384:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_103
	jp	.LBB21_103
.Ltmp1385:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_103:
.Ltmp1386:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp1387:
	.loc	26 161 24
	jbe	.LBB21_105
.Ltmp1388:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_105:
.Ltmp1389:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1390:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp1391:
	.loc	21 442 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp1392:
	.loc	26 103 24
	vandps	%xmm7, %xmm15, %xmm3
.Ltmp1393:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm6, %xmm6
.Ltmp1394:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp1395:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp1396:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp1397:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp1398:
	.loc	21 448 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp1399:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm2
.Ltmp1400:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1401:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp1402:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm2
.Ltmp1403:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp1404:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp1405:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp1406:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1407:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp1408:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp1409:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1410:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1411:
	.loc	26 161 24
	cmovbel	%r14d, %edx
.Ltmp1412:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1413:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1414:
	.loc	26 161 24
	cmovbel	%r11d, %edx
.Ltmp1415:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1416:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1417:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1418:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1419:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp1420:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1421:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1422:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1423:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1424:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1425:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1426:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1427:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1428:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp1429:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1430:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp1431:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1432:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1433:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1434:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1435:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp1436:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp1437:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp1438:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp1439:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp1440:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1441:
	.loc	26 149 9
	movl	%r13d, %edx
.Ltmp1442:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm6
.Ltmp1443:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm6
.Ltmp1444:
	.loc	26 149 9
	notl	%edx
.Ltmp1445:
	.loc	26 139 9
	cmovbel	%r10d, %edx
.Ltmp1446:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm2
.Ltmp1447:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1448:
	.loc	21 491 9
	vmovss	776(%rbx), %xmm3
.Ltmp1449:
	.loc	26 144 9
	cmoval	%r13d, %ebp
.Ltmp1450:
	.loc	26 139 9
	cmovbel	%r10d, %edx
.Ltmp1451:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_107
.Ltmp1452:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_107:
	movq	56(%rsp), %r13
.Ltmp1453:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_109
.Ltmp1454:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_109:
.Ltmp1455:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp1456:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm13
.Ltmp1457:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 498 5
	movl	%edx, 932(%rbx)
.Ltmp1458:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1459:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp1460:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1461:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp1462:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp1463:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_13
.Ltmp1464:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_13
.LBB21_111:
	cmpq	%r10, %rsi
	jbe	.LBB21_208
	movq	%r10, 176(%rsp)
.Ltmp1465:
	.loc	25 438 16 is_stmt 1
	movq	%r10, %rax
	negq	%rax
	movl	%r13d, %r8d
	subl	%edi, %r8d
	movq	%rsi, %r9
	negq	%r9
	movl	$1, %r15d
	movzbl	48(%rsp), %r11d
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$841731191, %r14d
	movl	$8388608, %r10d
	xorl	%r12d, %r12d
	movq	152(%rsp), %rsi
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_114
.Ltmp1466:
	.loc	25 0 16 is_stmt 0
.Ltmp1467:
	.p2align	4
.LBB21_113:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp1468:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp1469:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1470:
	.loc	26 92 9
	vmulss	768(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1471:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1472:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp1473:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp1474:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp1475:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp1476:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp1477:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp1478:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp1479:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm15
.Ltmp1480:
	.loc	26 61 9
	vaddss	%xmm2, %xmm15, %xmm2
.Ltmp1481:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp1482:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp1483:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp1484:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp1485:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp1486:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp1487:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1488:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm4, %xmm2
.Ltmp1489:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
.Ltmp1490:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp1491:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp1492:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
.Ltmp1493:
	.loc	26 161 24
	vblendvps	%xmm9, %xmm2, %xmm4, %xmm2
.Ltmp1494:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm4
.Ltmp1495:
	.loc	26 66 9
	vsubss	%xmm4, %xmm3, %xmm3
.Ltmp1496:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp1497:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp1498:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1499:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp1500:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1501:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp1502:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1503:
	.loc	26 61 9
	vaddss	%xmm6, %xmm15, %xmm6
.Ltmp1504:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1505:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp1506:
	.loc	26 178 22
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp1507:
	.loc	7 1244 18
	vmovd	%xmm4, %edx
.Ltmp1508:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp1509:
	.loc	7 1291 18
	vmovd	%edx, %xmm4
.Ltmp1510:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp1511:
	.loc	21 510 5
	vmovss	%xmm10, 940(%rbx)
	vmovaps	128(%rsp), %xmm6
.Ltmp1512:
	.loc	26 71 9
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp1513:
	.loc	26 161 24
	vcmpneqss	%xmm5, %xmm10, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	vcmpnltss	780(%rbx), %xmm5, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	movq	88(%rsp), %rdx
.Ltmp1514:
	.loc	26 56 9
	vmovss	%xmm2, -4(%rdx,%r15,4)
.Ltmp1515:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rsi,%r15,4)
.Ltmp1516:
	.loc	8 1916 50 is_stmt 1
	leaq	(%r9,%r15), %rdx
	incq	%rdx
	incq	%r15
	cmpq	$1, %rdx
	movq	72(%rsp), %rdx
.Ltmp1517:
	.loc	11 900 12
	je	.LBB21_494
.Ltmp1518:
.LBB21_114:
	.loc	21 361 22
	leal	(%r15,%r13), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp1519:
	.loc	25 451 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp1520:
	.loc	21 0 0 is_stmt 0
	leaq	(%rax,%r15), %r13
	movq	88(%rsp), %rbp
.Ltmp1521:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rbp,%r15,4), %xmm2
	movq	80(%rsp), %rbp
.Ltmp1522:
	.loc	26 56 9
	vmovss	%xmm2, (%rbp,%rdi,4)
.Ltmp1523:
	.loc	26 51 9
	vmovss	-4(%rsi,%r15,4), %xmm2
	movq	24(%rsp), %rdx
.Ltmp1524:
	.loc	26 56 9
	vmovss	%xmm2, (%rdx,%rdi,4)
.Ltmp1525:
	.loc	25 438 16
	cmpq	$1, %r13
	je	.LBB21_776
.Ltmp1526:
	.loc	25 451 16
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_711
.Ltmp1527:
	.loc	25 0 16 is_stmt 0
	movq	64(%rsp), %rdx
.Ltmp1528:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rdx,%r15,4), %xmm2
	movq	40(%rsp), %rdx
.Ltmp1529:
	.loc	26 56 9
	vmovss	%xmm2, (%rdx,%rdi,4)
.Ltmp1530:
	.loc	25 451 16
	cmpq	%rdi, 16(%rsp)
	jbe	.LBB21_712
.Ltmp1531:
	.loc	25 0 16 is_stmt 0
	movq	96(%rsp), %rdx
.Ltmp1532:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rdx,%r15,4), %xmm2
	movq	32(%rsp), %rdx
.Ltmp1533:
	.loc	26 56 9
	vmovss	%xmm2, (%rdx,%rdi,4)
.Ltmp1534:
	.loc	21 370 21
	leal	(%r8,%r15), %edi
	decl	%edi
	andl	%ecx, %edi
	movq	72(%rsp), %rdx
.Ltmp1535:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp1536:
	.loc	26 51 9
	vmovss	(%rbp,%rdi,4), %xmm4
	movq	24(%rsp), %rdx
.Ltmp1537:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm2
	movq	56(%rsp), %rdx
.Ltmp1538:
	.loc	21 0 0
	addl	%r15d, %edx
	movl	136(%rbx), %r13d
	movl	200(%rbx), %ebp
	notl	%r13d
	addl	%edx, %r13d
	andl	%ecx, %r13d
	notl	%ebp
	addl	%edx, %ebp
	andl	%ecx, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 48(%rsp)
	vmovaps	%xmm2, 128(%rsp)
	je	.LBB21_124
	cmpl	$1, %r11d
	movq	16(%rsp), %rdx
	jne	.LBB21_127
	.loc	21 0 0 is_stmt 0
	cmpq	%r13, 8(%rsp)
	jbe	.LBB21_801
.Ltmp1539:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, %rdx
	jbe	.LBB21_804
.Ltmp1540:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm6
	movq	32(%rsp), %rdx
.Ltmp1541:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm15
	vmovaps	%xmm15, %xmm10
	vmovaps	%xmm6, %xmm12
	jmp	.LBB21_132
.Ltmp1542:
	.loc	21 0 33 is_stmt 0
.Ltmp1543:
	.p2align	4
.LBB21_124:
	cmpq	%r13, 8(%rsp)
	movq	16(%rsp), %rdx
.Ltmp1544:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_802
.Ltmp1545:
	.loc	21 237 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_798
.Ltmp1546:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm10
	movq	32(%rsp), %rdx
.Ltmp1547:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm15
	vmovaps	%xmm10, %xmm12
.Ltmp1548:
	.loc	26 51 9
	jmp	.LBB21_132
.Ltmp1549:
	.loc	26 0 9 is_stmt 0
.Ltmp1550:
	.p2align	4
.LBB21_127:
	cmpq	%r13, 8(%rsp)
.Ltmp1551:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_803
	.loc	21 267 33
	cmpq	%r13, %rdx
	jbe	.LBB21_799
	.loc	21 268 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_800
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	vmovss	(%rdx,%r13,4), %xmm12
	movq	32(%rsp), %rdi
	vmovss	(%rdi,%r13,4), %xmm10
	.loc	21 268 33 is_stmt 1
	vmovss	(%rdi,%rbp,4), %xmm15
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm6
.Ltmp1552:
.LBB21_132:
	.loc	21 439 26
	vmovss	804(%rbx), %xmm9
.Ltmp1553:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp1554:
	.loc	21 441 44
	vmovss	800(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp1555:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm8, %xmm11
.Ltmp1556:
	.loc	26 161 24
	jne	.LBB21_135
	jp	.LBB21_135
.Ltmp1557:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm11
.LBB21_135:
.Ltmp1558:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_138
	jp	.LBB21_138
.Ltmp1559:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_138:
.Ltmp1560:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm9
.Ltmp1561:
	.loc	26 161 24
	jbe	.LBB21_140
.Ltmp1562:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm8
.LBB21_140:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 800(%rbx)
.Ltmp1563:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1564:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp1565:
	.loc	21 448 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp1566:
	.loc	21 439 26
	vmovss	820(%rbx), %xmm11
.Ltmp1567:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp1568:
	.loc	21 441 27
	vmovss	808(%rbx), %xmm9
	.loc	21 441 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp1569:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp1570:
	.loc	26 161 24
	jne	.LBB21_143
	jp	.LBB21_143
.Ltmp1571:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_143:
.Ltmp1572:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_146
	jp	.LBB21_146
.Ltmp1573:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_146:
.Ltmp1574:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp1575:
	.loc	26 161 24
	jbe	.LBB21_148
.Ltmp1576:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_148:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp1577:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1578:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp1579:
	.loc	21 448 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp1580:
	.loc	21 439 26
	vmovss	836(%rbx), %xmm13
.Ltmp1581:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1582:
	.loc	21 441 27
	vmovss	824(%rbx), %xmm11
	.loc	21 441 44 is_stmt 0
	vmovss	832(%rbx), %xmm3
.Ltmp1583:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm14
.Ltmp1584:
	.loc	26 161 24
	jne	.LBB21_151
	jp	.LBB21_151
.Ltmp1585:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_151:
.Ltmp1586:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_154
	jp	.LBB21_154
.Ltmp1587:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_154:
.Ltmp1588:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp1589:
	.loc	26 161 24
	jbe	.LBB21_156
.Ltmp1590:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm11
.LBB21_156:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 824(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp1591:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1592:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1593:
	.loc	21 448 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp1594:
	.loc	21 439 26
	vmovss	852(%rbx), %xmm14
.Ltmp1595:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1596:
	.loc	21 441 44
	vmovss	848(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp1597:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp1598:
	.loc	26 161 24
	jne	.LBB21_159
	jp	.LBB21_159
.Ltmp1599:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_159:
.Ltmp1600:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_162
	jp	.LBB21_162
.Ltmp1601:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_162:
.Ltmp1602:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp1603:
	.loc	26 161 24
	jbe	.LBB21_164
.Ltmp1604:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_164:
.Ltmp1605:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1606:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp1607:
	.loc	21 442 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp1608:
	.loc	26 103 24
	vandps	%xmm7, %xmm12, %xmm3
.Ltmp1609:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm10, %xmm7
.Ltmp1610:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp1611:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp1612:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp1613:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp1614:
	.loc	21 448 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp1615:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm2
.Ltmp1616:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1617:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp1618:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm2
.Ltmp1619:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm10
.Ltmp1620:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm2
.Ltmp1621:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm3
.Ltmp1622:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1623:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp1624:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp1625:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1626:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1627:
	.loc	26 161 24
	cmovbel	%r14d, %edx
.Ltmp1628:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1629:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1630:
	.loc	26 161 24
	cmovbel	%r10d, %edx
.Ltmp1631:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1632:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1633:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1634:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1635:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm7
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp1636:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1637:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1638:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1639:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1640:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1641:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1642:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1643:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1644:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp1645:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1646:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp1647:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1648:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1649:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1650:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1651:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm10
.Ltmp1652:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp1653:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm10
.Ltmp1654:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp1655:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm10
.Ltmp1656:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1657:
	.loc	26 149 9
	movl	%r13d, %edx
.Ltmp1658:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm3
.Ltmp1659:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm3
.Ltmp1660:
	.loc	26 149 9
	notl	%edx
.Ltmp1661:
	.loc	26 139 9
	cmovbel	%r12d, %edx
.Ltmp1662:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm2
.Ltmp1663:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1664:
	.loc	21 491 9
	vmovss	752(%rbx), %xmm12
.Ltmp1665:
	.loc	26 144 9
	cmoval	%r13d, %ebp
.Ltmp1666:
	.loc	26 139 9
	cmovbel	%r12d, %edx
.Ltmp1667:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_166
.Ltmp1668:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_166:
.Ltmp1669:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_168
.Ltmp1670:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_168:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm3
.Ltmp1671:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 498 5
	movl	%edx, 856(%rbx)
.Ltmp1672:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1673:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm10, %xmm7
.Ltmp1674:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm7, %xmm2, %xmm2
.Ltmp1675:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm7
	vxorps	%xmm7, %xmm11, %xmm7
.Ltmp1676:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm2, %xmm2
.Ltmp1677:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm7
	vandps	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_170
.Ltmp1678:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_170:
.Ltmp1679:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp1680:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp1681:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1682:
	.loc	26 92 9
	vmulss	744(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1683:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1684:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp1685:
	.loc	21 510 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp1686:
	.loc	21 439 26
	vmovss	880(%rbx), %xmm11
.Ltmp1687:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp1688:
	.loc	21 441 44
	vmovss	876(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp1689:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp1690:
	.loc	26 161 24
	jne	.LBB21_173
	jp	.LBB21_173
.Ltmp1691:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_173:
.Ltmp1692:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_176
	jp	.LBB21_176
.Ltmp1693:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_176:
.Ltmp1694:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp1695:
	.loc	26 161 24
	jbe	.LBB21_178
.Ltmp1696:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_178:
.Ltmp1697:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm5, %xmm9
.Ltmp1698:
	.loc	21 442 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp1699:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1700:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp1701:
	.loc	21 448 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp1702:
	.loc	21 439 26
	vmovss	896(%rbx), %xmm12
.Ltmp1703:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp1704:
	.loc	21 441 44
	vmovss	892(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp1705:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp1706:
	.loc	26 161 24
	jne	.LBB21_181
	jp	.LBB21_181
.Ltmp1707:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_181:
.Ltmp1708:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_184
	jp	.LBB21_184
.Ltmp1709:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_184:
.Ltmp1710:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm12
.Ltmp1711:
	.loc	26 161 24
	jbe	.LBB21_186
.Ltmp1712:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_186:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp1713:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp1714:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp1715:
	.loc	21 448 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp1716:
	.loc	21 439 26
	vmovss	912(%rbx), %xmm13
.Ltmp1717:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1718:
	.loc	21 441 44
	vmovss	908(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp1719:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp1720:
	.loc	26 161 24
	jne	.LBB21_189
	jp	.LBB21_189
.Ltmp1721:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_189:
.Ltmp1722:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_192
	jp	.LBB21_192
.Ltmp1723:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_192:
.Ltmp1724:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp1725:
	.loc	26 161 24
	jbe	.LBB21_194
.Ltmp1726:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_194:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp1727:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1728:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1729:
	.loc	21 448 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp1730:
	.loc	21 439 26
	vmovss	928(%rbx), %xmm14
.Ltmp1731:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1732:
	.loc	21 441 44
	vmovss	924(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp1733:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp1734:
	.loc	26 161 24
	jne	.LBB21_197
	jp	.LBB21_197
.Ltmp1735:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_197:
.Ltmp1736:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_200
	jp	.LBB21_200
.Ltmp1737:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_200:
.Ltmp1738:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp1739:
	.loc	26 161 24
	jbe	.LBB21_202
.Ltmp1740:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_202:
.Ltmp1741:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1742:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp1743:
	.loc	21 442 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp1744:
	.loc	26 103 24
	vandps	%xmm7, %xmm15, %xmm3
.Ltmp1745:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm6, %xmm6
.Ltmp1746:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp1747:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp1748:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp1749:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp1750:
	.loc	21 448 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp1751:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm2
.Ltmp1752:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1753:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp1754:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm2
.Ltmp1755:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp1756:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp1757:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp1758:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1759:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp1760:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp1761:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1762:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1763:
	.loc	26 161 24
	cmovbel	%r14d, %edx
.Ltmp1764:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1765:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1766:
	.loc	26 161 24
	cmovbel	%r10d, %edx
.Ltmp1767:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1768:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1769:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1770:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1771:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp1772:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1773:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1774:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1775:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1776:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1777:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1778:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1779:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1780:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp1781:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1782:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp1783:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1784:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1785:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1786:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1787:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp1788:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp1789:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp1790:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp1791:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp1792:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1793:
	.loc	26 149 9
	movl	%r13d, %edx
.Ltmp1794:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm6
.Ltmp1795:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm6
.Ltmp1796:
	.loc	26 149 9
	notl	%edx
.Ltmp1797:
	.loc	26 139 9
	cmovbel	%r12d, %edx
.Ltmp1798:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm2
.Ltmp1799:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1800:
	.loc	21 491 9
	vmovss	776(%rbx), %xmm3
.Ltmp1801:
	.loc	26 144 9
	cmoval	%r13d, %ebp
.Ltmp1802:
	.loc	26 139 9
	cmovbel	%r12d, %edx
.Ltmp1803:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_204
.Ltmp1804:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_204:
	movq	56(%rsp), %r13
.Ltmp1805:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_206
.Ltmp1806:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_206:
.Ltmp1807:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp1808:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm13
.Ltmp1809:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 498 5
	movl	%edx, 932(%rbx)
.Ltmp1810:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1811:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp1812:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1813:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp1814:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp1815:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_113
.Ltmp1816:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_113
.LBB21_208:
	cmpq	8(%rsp), %rdx
	jbe	.LBB21_305
.Ltmp1817:
	.loc	25 438 16 is_stmt 1
	movq	%rsi, %rax
	negq	%rax
	movl	%r13d, %r8d
	subl	%edi, %r8d
	movl	$1, %r15d
	movzbl	48(%rsp), %r9d
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$841731191, %r11d
	movl	$8388608, %r10d
	xorl	%r14d, %r14d
	movq	152(%rsp), %rsi
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_211
.Ltmp1818:
	.loc	25 0 16 is_stmt 0
.Ltmp1819:
	.p2align	4
.LBB21_210:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp1820:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp1821:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1822:
	.loc	26 92 9
	vmulss	768(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1823:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1824:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp1825:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp1826:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp1827:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp1828:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp1829:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp1830:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp1831:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm15
.Ltmp1832:
	.loc	26 61 9
	vaddss	%xmm2, %xmm15, %xmm2
.Ltmp1833:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp1834:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp1835:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp1836:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp1837:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp1838:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp1839:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1840:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm4, %xmm2
.Ltmp1841:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
.Ltmp1842:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp1843:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp1844:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
.Ltmp1845:
	.loc	26 161 24
	vblendvps	%xmm9, %xmm2, %xmm4, %xmm2
.Ltmp1846:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm4
.Ltmp1847:
	.loc	26 66 9
	vsubss	%xmm4, %xmm3, %xmm3
.Ltmp1848:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp1849:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp1850:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1851:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp1852:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1853:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp1854:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1855:
	.loc	26 61 9
	vaddss	%xmm6, %xmm15, %xmm6
.Ltmp1856:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1857:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp1858:
	.loc	26 178 22
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp1859:
	.loc	7 1244 18
	vmovd	%xmm4, %edx
.Ltmp1860:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp1861:
	.loc	7 1291 18
	vmovd	%edx, %xmm4
.Ltmp1862:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp1863:
	.loc	21 510 5
	vmovss	%xmm10, 940(%rbx)
	vmovaps	128(%rsp), %xmm6
.Ltmp1864:
	.loc	26 71 9
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp1865:
	.loc	26 161 24
	vcmpneqss	%xmm5, %xmm10, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	vcmpnltss	780(%rbx), %xmm5, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	movq	88(%rsp), %rdx
.Ltmp1866:
	.loc	26 56 9
	vmovss	%xmm2, -4(%rdx,%r15,4)
.Ltmp1867:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rsi,%r15,4)
.Ltmp1868:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rax,%r15), %rdx
	incq	%rdx
	incq	%r15
	cmpq	$1, %rdx
	movq	72(%rsp), %rdx
	movq	80(%rsp), %rbp
.Ltmp1869:
	.loc	11 900 12
	je	.LBB21_494
.LBB21_211:
.Ltmp1870:
	.loc	15 971 17
	leaq	(%rax,%r15), %rdi
.Ltmp1871:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_775
.Ltmp1872:
	.loc	21 0 0 is_stmt 0
	leal	(%r15,%r13), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp1873:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp1874:
	.loc	25 0 16 is_stmt 0
	movq	88(%rsp), %r12
.Ltmp1875:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
.Ltmp1876:
	.loc	26 56 9
	vmovss	%xmm2, (%rbp,%rdi,4)
.Ltmp1877:
	.loc	26 51 9
	vmovss	-4(%rsi,%r15,4), %xmm2
	movq	24(%rsp), %r12
.Ltmp1878:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
.Ltmp1879:
	.loc	25 451 16
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_711
.Ltmp1880:
	.loc	25 0 16 is_stmt 0
	movq	64(%rsp), %r12
.Ltmp1881:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
	movq	40(%rsp), %r12
.Ltmp1882:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
.Ltmp1883:
	.loc	25 451 16
	cmpq	%rdi, 16(%rsp)
	jbe	.LBB21_712
.Ltmp1884:
	.loc	25 0 16 is_stmt 0
	movq	96(%rsp), %r12
.Ltmp1885:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
	movq	32(%rsp), %r12
.Ltmp1886:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
.Ltmp1887:
	.loc	21 370 21
	leal	(%r8,%r15), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp1888:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp1889:
	.loc	26 51 9
	vmovss	(%rbp,%rdi,4), %xmm4
	movq	24(%rsp), %rdx
.Ltmp1890:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm2
.Ltmp1891:
	.loc	21 0 0
	leal	(%r15,%r13), %edx
	movl	136(%rbx), %r13d
	movl	200(%rbx), %ebp
	notl	%r13d
	addl	%edx, %r13d
	andl	%ecx, %r13d
	notl	%ebp
	addl	%edx, %ebp
	andl	%ecx, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 48(%rsp)
	vmovaps	%xmm2, 128(%rsp)
	je	.LBB21_221
	cmpl	$1, %r9d
	movq	16(%rsp), %rdx
	jne	.LBB21_224
	.loc	21 0 0 is_stmt 0
	cmpq	%r13, 8(%rsp)
	jbe	.LBB21_801
.Ltmp1892:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, %rdx
	jbe	.LBB21_804
.Ltmp1893:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm6
	movq	32(%rsp), %rdx
.Ltmp1894:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm15
	vmovaps	%xmm15, %xmm10
	vmovaps	%xmm6, %xmm12
	jmp	.LBB21_229
.Ltmp1895:
	.loc	21 0 33 is_stmt 0
.Ltmp1896:
	.p2align	4
.LBB21_221:
	cmpq	%r13, 8(%rsp)
	movq	16(%rsp), %rdx
.Ltmp1897:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_802
.Ltmp1898:
	.loc	21 237 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_798
.Ltmp1899:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm10
	movq	32(%rsp), %rdx
.Ltmp1900:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm15
	vmovaps	%xmm10, %xmm12
.Ltmp1901:
	.loc	26 51 9
	jmp	.LBB21_229
.Ltmp1902:
	.loc	26 0 9 is_stmt 0
.Ltmp1903:
	.p2align	4
.LBB21_224:
	cmpq	%r13, 8(%rsp)
.Ltmp1904:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_803
	.loc	21 267 33
	cmpq	%r13, %rdx
	jbe	.LBB21_799
	.loc	21 268 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_800
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	vmovss	(%rdx,%r13,4), %xmm12
	movq	32(%rsp), %rdi
	vmovss	(%rdi,%r13,4), %xmm10
	.loc	21 268 33 is_stmt 1
	vmovss	(%rdi,%rbp,4), %xmm15
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm6
.Ltmp1905:
.LBB21_229:
	.loc	21 439 26
	vmovss	804(%rbx), %xmm9
.Ltmp1906:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp1907:
	.loc	21 441 44
	vmovss	800(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp1908:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm8, %xmm11
	movq	56(%rsp), %r13
.Ltmp1909:
	.loc	26 161 24
	jne	.LBB21_232
	jp	.LBB21_232
.Ltmp1910:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm11
.LBB21_232:
.Ltmp1911:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_235
	jp	.LBB21_235
.Ltmp1912:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_235:
.Ltmp1913:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm9
.Ltmp1914:
	.loc	26 161 24
	jbe	.LBB21_237
.Ltmp1915:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm8
.LBB21_237:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 800(%rbx)
.Ltmp1916:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1917:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp1918:
	.loc	21 448 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp1919:
	.loc	21 439 26
	vmovss	820(%rbx), %xmm11
.Ltmp1920:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp1921:
	.loc	21 441 27
	vmovss	808(%rbx), %xmm9
	.loc	21 441 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp1922:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp1923:
	.loc	26 161 24
	jne	.LBB21_240
	jp	.LBB21_240
.Ltmp1924:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_240:
.Ltmp1925:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_243
	jp	.LBB21_243
.Ltmp1926:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_243:
.Ltmp1927:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp1928:
	.loc	26 161 24
	jbe	.LBB21_245
.Ltmp1929:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_245:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp1930:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1931:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp1932:
	.loc	21 448 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp1933:
	.loc	21 439 26
	vmovss	836(%rbx), %xmm13
.Ltmp1934:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1935:
	.loc	21 441 27
	vmovss	824(%rbx), %xmm11
	.loc	21 441 44 is_stmt 0
	vmovss	832(%rbx), %xmm3
.Ltmp1936:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm14
.Ltmp1937:
	.loc	26 161 24
	jne	.LBB21_248
	jp	.LBB21_248
.Ltmp1938:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_248:
.Ltmp1939:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_251
	jp	.LBB21_251
.Ltmp1940:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_251:
.Ltmp1941:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp1942:
	.loc	26 161 24
	jbe	.LBB21_253
.Ltmp1943:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm11
.LBB21_253:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 824(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp1944:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1945:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1946:
	.loc	21 448 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp1947:
	.loc	21 439 26
	vmovss	852(%rbx), %xmm14
.Ltmp1948:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1949:
	.loc	21 441 44
	vmovss	848(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp1950:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp1951:
	.loc	26 161 24
	jne	.LBB21_256
	jp	.LBB21_256
.Ltmp1952:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_256:
.Ltmp1953:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_259
	jp	.LBB21_259
.Ltmp1954:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_259:
.Ltmp1955:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp1956:
	.loc	26 161 24
	jbe	.LBB21_261
.Ltmp1957:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_261:
.Ltmp1958:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1959:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp1960:
	.loc	21 442 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp1961:
	.loc	26 103 24
	vandps	%xmm7, %xmm12, %xmm3
.Ltmp1962:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm10, %xmm7
.Ltmp1963:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp1964:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp1965:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp1966:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp1967:
	.loc	21 448 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp1968:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm2
.Ltmp1969:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp1970:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp1971:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm2
.Ltmp1972:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm10
.Ltmp1973:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm2
.Ltmp1974:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm3
.Ltmp1975:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1976:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp1977:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp1978:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1979:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1980:
	.loc	26 161 24
	cmovbel	%r11d, %edx
.Ltmp1981:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp1982:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1983:
	.loc	26 161 24
	cmovbel	%r10d, %edx
.Ltmp1984:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1985:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1986:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1987:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1988:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm7
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp1989:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1990:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1991:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1992:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1993:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1994:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1995:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1996:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1997:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp1998:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1999:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp2000:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2001:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2002:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2003:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2004:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm10
.Ltmp2005:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp2006:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm10
.Ltmp2007:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2008:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm10
.Ltmp2009:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2010:
	.loc	26 149 9
	movl	%r12d, %edx
.Ltmp2011:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm3
.Ltmp2012:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm3
.Ltmp2013:
	.loc	26 149 9
	notl	%edx
.Ltmp2014:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2015:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm2
.Ltmp2016:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2017:
	.loc	21 491 9
	vmovss	752(%rbx), %xmm12
.Ltmp2018:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2019:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2020:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_263
.Ltmp2021:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_263:
.Ltmp2022:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_265
.Ltmp2023:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_265:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm3
.Ltmp2024:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 498 5
	movl	%edx, 856(%rbx)
.Ltmp2025:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2026:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm10, %xmm7
.Ltmp2027:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm7, %xmm2, %xmm2
.Ltmp2028:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm7
	vxorps	%xmm7, %xmm11, %xmm7
.Ltmp2029:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm2, %xmm2
.Ltmp2030:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm7
	vandps	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_267
.Ltmp2031:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_267:
.Ltmp2032:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp2033:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp2034:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2035:
	.loc	26 92 9
	vmulss	744(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2036:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2037:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp2038:
	.loc	21 510 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp2039:
	.loc	21 439 26
	vmovss	880(%rbx), %xmm11
.Ltmp2040:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp2041:
	.loc	21 441 44
	vmovss	876(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp2042:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp2043:
	.loc	26 161 24
	jne	.LBB21_270
	jp	.LBB21_270
.Ltmp2044:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_270:
.Ltmp2045:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_273
	jp	.LBB21_273
.Ltmp2046:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_273:
.Ltmp2047:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp2048:
	.loc	26 161 24
	jbe	.LBB21_275
.Ltmp2049:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_275:
.Ltmp2050:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm5, %xmm9
.Ltmp2051:
	.loc	21 442 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp2052:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2053:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp2054:
	.loc	21 448 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp2055:
	.loc	21 439 26
	vmovss	896(%rbx), %xmm12
.Ltmp2056:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp2057:
	.loc	21 441 44
	vmovss	892(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp2058:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp2059:
	.loc	26 161 24
	jne	.LBB21_278
	jp	.LBB21_278
.Ltmp2060:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_278:
.Ltmp2061:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_281
	jp	.LBB21_281
.Ltmp2062:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_281:
.Ltmp2063:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm12
.Ltmp2064:
	.loc	26 161 24
	jbe	.LBB21_283
.Ltmp2065:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_283:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp2066:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp2067:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp2068:
	.loc	21 448 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp2069:
	.loc	21 439 26
	vmovss	912(%rbx), %xmm13
.Ltmp2070:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2071:
	.loc	21 441 44
	vmovss	908(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp2072:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp2073:
	.loc	26 161 24
	jne	.LBB21_286
	jp	.LBB21_286
.Ltmp2074:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_286:
.Ltmp2075:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_289
	jp	.LBB21_289
.Ltmp2076:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_289:
.Ltmp2077:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp2078:
	.loc	26 161 24
	jbe	.LBB21_291
.Ltmp2079:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_291:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp2080:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2081:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2082:
	.loc	21 448 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp2083:
	.loc	21 439 26
	vmovss	928(%rbx), %xmm14
.Ltmp2084:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2085:
	.loc	21 441 44
	vmovss	924(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp2086:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2087:
	.loc	26 161 24
	jne	.LBB21_294
	jp	.LBB21_294
.Ltmp2088:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_294:
.Ltmp2089:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_297
	jp	.LBB21_297
.Ltmp2090:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_297:
.Ltmp2091:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp2092:
	.loc	26 161 24
	jbe	.LBB21_299
.Ltmp2093:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_299:
.Ltmp2094:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2095:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp2096:
	.loc	21 442 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp2097:
	.loc	26 103 24
	vandps	%xmm7, %xmm15, %xmm3
.Ltmp2098:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm6, %xmm6
.Ltmp2099:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp2100:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp2101:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp2102:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp2103:
	.loc	21 448 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp2104:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm2
.Ltmp2105:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2106:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp2107:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm2
.Ltmp2108:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp2109:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp2110:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp2111:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2112:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp2113:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp2114:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2115:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2116:
	.loc	26 161 24
	cmovbel	%r11d, %edx
.Ltmp2117:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2118:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2119:
	.loc	26 161 24
	cmovbel	%r10d, %edx
.Ltmp2120:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2121:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2122:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2123:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2124:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp2125:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2126:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2127:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2128:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2129:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2130:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2131:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2132:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2133:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp2134:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2135:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp2136:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2137:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2138:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2139:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2140:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp2141:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp2142:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp2143:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2144:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp2145:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2146:
	.loc	26 149 9
	movl	%r12d, %edx
.Ltmp2147:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm6
.Ltmp2148:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm6
.Ltmp2149:
	.loc	26 149 9
	notl	%edx
.Ltmp2150:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2151:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm2
.Ltmp2152:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2153:
	.loc	21 491 9
	vmovss	776(%rbx), %xmm3
.Ltmp2154:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2155:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2156:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_301
.Ltmp2157:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_301:
.Ltmp2158:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_303
.Ltmp2159:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_303:
.Ltmp2160:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp2161:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm13
.Ltmp2162:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 498 5
	movl	%edx, 932(%rbx)
.Ltmp2163:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2164:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp2165:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2166:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp2167:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp2168:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_210
.Ltmp2169:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_210
.LBB21_305:
	cmpq	16(%rsp), %rdx
	jbe	.LBB21_400
.Ltmp2170:
	.loc	25 438 16 is_stmt 1
	movq	%rsi, %rax
	negq	%rax
	movl	%r13d, %r8d
	subl	%edi, %r8d
	movl	$1, %r15d
	movzbl	48(%rsp), %r9d
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$841731191, %r10d
	movl	$8388608, %r11d
	xorl	%r14d, %r14d
	movq	152(%rsp), %rsi
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_308
.Ltmp2171:
	.loc	25 0 16 is_stmt 0
.Ltmp2172:
	.p2align	4
.LBB21_307:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp2173:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp2174:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2175:
	.loc	26 92 9
	vmulss	768(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2176:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2177:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp2178:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp2179:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp2180:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp2181:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp2182:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp2183:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp2184:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm15
.Ltmp2185:
	.loc	26 61 9
	vaddss	%xmm2, %xmm15, %xmm2
.Ltmp2186:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp2187:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp2188:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp2189:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp2190:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp2191:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp2192:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2193:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm4, %xmm2
.Ltmp2194:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
.Ltmp2195:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp2196:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp2197:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
.Ltmp2198:
	.loc	26 161 24
	vblendvps	%xmm9, %xmm2, %xmm4, %xmm2
.Ltmp2199:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm4
.Ltmp2200:
	.loc	26 66 9
	vsubss	%xmm4, %xmm3, %xmm3
.Ltmp2201:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp2202:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp2203:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2204:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2205:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2206:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp2207:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2208:
	.loc	26 61 9
	vaddss	%xmm6, %xmm15, %xmm6
.Ltmp2209:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp2210:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp2211:
	.loc	26 178 22
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp2212:
	.loc	7 1244 18
	vmovd	%xmm4, %edx
.Ltmp2213:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp2214:
	.loc	7 1291 18
	vmovd	%edx, %xmm4
.Ltmp2215:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp2216:
	.loc	21 510 5
	vmovss	%xmm10, 940(%rbx)
	vmovaps	128(%rsp), %xmm6
.Ltmp2217:
	.loc	26 71 9
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp2218:
	.loc	26 161 24
	vcmpneqss	%xmm5, %xmm10, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	vcmpnltss	780(%rbx), %xmm5, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	movq	88(%rsp), %rdx
.Ltmp2219:
	.loc	26 56 9
	vmovss	%xmm2, -4(%rdx,%r15,4)
.Ltmp2220:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rsi,%r15,4)
.Ltmp2221:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rax,%r15), %rdx
	incq	%rdx
	incq	%r15
	cmpq	$1, %rdx
	movq	72(%rsp), %rdx
	movq	80(%rsp), %rbp
.Ltmp2222:
	.loc	11 900 12
	je	.LBB21_494
.LBB21_308:
.Ltmp2223:
	.loc	15 971 17
	leaq	(%rax,%r15), %rdi
.Ltmp2224:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_775
.Ltmp2225:
	.loc	21 0 0 is_stmt 0
	leal	(%r15,%r13), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp2226:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp2227:
	.loc	25 0 16 is_stmt 0
	movq	88(%rsp), %r12
.Ltmp2228:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
.Ltmp2229:
	.loc	26 56 9
	vmovss	%xmm2, (%rbp,%rdi,4)
.Ltmp2230:
	.loc	26 51 9
	vmovss	-4(%rsi,%r15,4), %xmm2
	movq	24(%rsp), %r12
.Ltmp2231:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
	movq	64(%rsp), %r12
.Ltmp2232:
	.loc	26 51 9
	vmovss	-4(%r12,%r15,4), %xmm2
	movq	40(%rsp), %r12
.Ltmp2233:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
.Ltmp2234:
	.loc	25 451 16
	cmpq	%rdi, 16(%rsp)
	jbe	.LBB21_712
.Ltmp2235:
	.loc	25 0 16 is_stmt 0
	movq	96(%rsp), %r12
.Ltmp2236:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
	movq	32(%rsp), %r12
.Ltmp2237:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
.Ltmp2238:
	.loc	21 370 21
	leal	(%r8,%r15), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp2239:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp2240:
	.loc	26 51 9
	vmovss	(%rbp,%rdi,4), %xmm4
	movq	24(%rsp), %rdx
.Ltmp2241:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm2
.Ltmp2242:
	.loc	21 0 0
	leal	(%r15,%r13), %edx
	movl	136(%rbx), %r13d
	movl	200(%rbx), %ebp
	notl	%r13d
	addl	%edx, %r13d
	andl	%ecx, %r13d
	notl	%ebp
	addl	%edx, %ebp
	andl	%ecx, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 48(%rsp)
	vmovaps	%xmm2, 128(%rsp)
	je	.LBB21_317
	cmpl	$1, %r9d
	movq	16(%rsp), %rdx
	jne	.LBB21_320
	.loc	21 0 0 is_stmt 0
	cmpq	%r13, 8(%rsp)
	jbe	.LBB21_801
.Ltmp2243:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, %rdx
	jbe	.LBB21_804
.Ltmp2244:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm6
	movq	32(%rsp), %rdx
.Ltmp2245:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm15
	vmovaps	%xmm15, %xmm10
	vmovaps	%xmm6, %xmm12
	jmp	.LBB21_324
.Ltmp2246:
	.loc	21 0 33 is_stmt 0
.Ltmp2247:
	.p2align	4
.LBB21_317:
	cmpq	%r13, 8(%rsp)
	movq	16(%rsp), %rdx
.Ltmp2248:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_802
.Ltmp2249:
	.loc	21 237 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_798
.Ltmp2250:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm10
	movq	32(%rsp), %rdx
.Ltmp2251:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm15
	vmovaps	%xmm10, %xmm12
.Ltmp2252:
	.loc	26 51 9
	jmp	.LBB21_324
.Ltmp2253:
	.loc	26 0 9 is_stmt 0
.Ltmp2254:
	.p2align	4
.LBB21_320:
	cmpq	%r13, 8(%rsp)
.Ltmp2255:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_803
	.loc	21 267 33
	cmpq	%r13, %rdx
	jbe	.LBB21_799
	.loc	21 268 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_800
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	vmovss	(%rdx,%r13,4), %xmm12
	movq	32(%rsp), %rdi
	.loc	21 267 33 is_stmt 1
	vmovss	(%rdi,%r13,4), %xmm10
	.loc	21 268 33
	vmovss	(%rdi,%rbp,4), %xmm15
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm6
.Ltmp2256:
.LBB21_324:
	.loc	21 439 26
	vmovss	804(%rbx), %xmm9
.Ltmp2257:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp2258:
	.loc	21 441 44
	vmovss	800(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp2259:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm8, %xmm11
	movq	56(%rsp), %r13
.Ltmp2260:
	.loc	26 161 24
	jne	.LBB21_327
	jp	.LBB21_327
.Ltmp2261:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm11
.LBB21_327:
.Ltmp2262:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_330
	jp	.LBB21_330
.Ltmp2263:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_330:
.Ltmp2264:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm9
.Ltmp2265:
	.loc	26 161 24
	jbe	.LBB21_332
.Ltmp2266:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm8
.LBB21_332:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 800(%rbx)
.Ltmp2267:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2268:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp2269:
	.loc	21 448 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp2270:
	.loc	21 439 26
	vmovss	820(%rbx), %xmm11
.Ltmp2271:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp2272:
	.loc	21 441 27
	vmovss	808(%rbx), %xmm9
	.loc	21 441 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp2273:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp2274:
	.loc	26 161 24
	jne	.LBB21_335
	jp	.LBB21_335
.Ltmp2275:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_335:
.Ltmp2276:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_338
	jp	.LBB21_338
.Ltmp2277:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_338:
.Ltmp2278:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp2279:
	.loc	26 161 24
	jbe	.LBB21_340
.Ltmp2280:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_340:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp2281:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2282:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp2283:
	.loc	21 448 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp2284:
	.loc	21 439 26
	vmovss	836(%rbx), %xmm13
.Ltmp2285:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2286:
	.loc	21 441 27
	vmovss	824(%rbx), %xmm11
	.loc	21 441 44 is_stmt 0
	vmovss	832(%rbx), %xmm3
.Ltmp2287:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm14
.Ltmp2288:
	.loc	26 161 24
	jne	.LBB21_343
	jp	.LBB21_343
.Ltmp2289:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_343:
.Ltmp2290:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_346
	jp	.LBB21_346
.Ltmp2291:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_346:
.Ltmp2292:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp2293:
	.loc	26 161 24
	jbe	.LBB21_348
.Ltmp2294:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm11
.LBB21_348:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 824(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp2295:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2296:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2297:
	.loc	21 448 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp2298:
	.loc	21 439 26
	vmovss	852(%rbx), %xmm14
.Ltmp2299:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2300:
	.loc	21 441 44
	vmovss	848(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp2301:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2302:
	.loc	26 161 24
	jne	.LBB21_351
	jp	.LBB21_351
.Ltmp2303:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_351:
.Ltmp2304:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_354
	jp	.LBB21_354
.Ltmp2305:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_354:
.Ltmp2306:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp2307:
	.loc	26 161 24
	jbe	.LBB21_356
.Ltmp2308:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_356:
.Ltmp2309:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2310:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp2311:
	.loc	21 442 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp2312:
	.loc	26 103 24
	vandps	%xmm7, %xmm12, %xmm3
.Ltmp2313:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm10, %xmm7
.Ltmp2314:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp2315:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp2316:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp2317:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp2318:
	.loc	21 448 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp2319:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm2
.Ltmp2320:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2321:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp2322:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm2
.Ltmp2323:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm10
.Ltmp2324:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm2
.Ltmp2325:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm3
.Ltmp2326:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2327:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp2328:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp2329:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2330:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2331:
	.loc	26 161 24
	cmovbel	%r10d, %edx
.Ltmp2332:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2333:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2334:
	.loc	26 161 24
	cmovbel	%r11d, %edx
.Ltmp2335:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2336:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2337:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2338:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2339:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm7
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp2340:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2341:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2342:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2343:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2344:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2345:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2346:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2347:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2348:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp2349:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2350:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp2351:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2352:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2353:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2354:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2355:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm10
.Ltmp2356:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp2357:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm10
.Ltmp2358:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2359:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm10
.Ltmp2360:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2361:
	.loc	26 149 9
	movl	%r12d, %edx
.Ltmp2362:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm3
.Ltmp2363:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm3
.Ltmp2364:
	.loc	26 149 9
	notl	%edx
.Ltmp2365:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2366:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm2
.Ltmp2367:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2368:
	.loc	21 491 9
	vmovss	752(%rbx), %xmm12
.Ltmp2369:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2370:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2371:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_358
.Ltmp2372:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_358:
.Ltmp2373:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_360
.Ltmp2374:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_360:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm3
.Ltmp2375:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 498 5
	movl	%edx, 856(%rbx)
.Ltmp2376:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2377:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm10, %xmm7
.Ltmp2378:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm7, %xmm2, %xmm2
.Ltmp2379:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm7
	vxorps	%xmm7, %xmm11, %xmm7
.Ltmp2380:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm2, %xmm2
.Ltmp2381:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm7
	vandps	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_362
.Ltmp2382:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_362:
.Ltmp2383:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp2384:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp2385:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2386:
	.loc	26 92 9
	vmulss	744(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2387:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2388:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp2389:
	.loc	21 510 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp2390:
	.loc	21 439 26
	vmovss	880(%rbx), %xmm11
.Ltmp2391:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp2392:
	.loc	21 441 44
	vmovss	876(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp2393:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp2394:
	.loc	26 161 24
	jne	.LBB21_365
	jp	.LBB21_365
.Ltmp2395:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_365:
.Ltmp2396:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_368
	jp	.LBB21_368
.Ltmp2397:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_368:
.Ltmp2398:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp2399:
	.loc	26 161 24
	jbe	.LBB21_370
.Ltmp2400:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_370:
.Ltmp2401:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm5, %xmm9
.Ltmp2402:
	.loc	21 442 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp2403:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2404:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp2405:
	.loc	21 448 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp2406:
	.loc	21 439 26
	vmovss	896(%rbx), %xmm12
.Ltmp2407:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp2408:
	.loc	21 441 44
	vmovss	892(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp2409:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp2410:
	.loc	26 161 24
	jne	.LBB21_373
	jp	.LBB21_373
.Ltmp2411:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_373:
.Ltmp2412:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_376
	jp	.LBB21_376
.Ltmp2413:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_376:
.Ltmp2414:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm12
.Ltmp2415:
	.loc	26 161 24
	jbe	.LBB21_378
.Ltmp2416:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_378:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp2417:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp2418:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp2419:
	.loc	21 448 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp2420:
	.loc	21 439 26
	vmovss	912(%rbx), %xmm13
.Ltmp2421:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2422:
	.loc	21 441 44
	vmovss	908(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp2423:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp2424:
	.loc	26 161 24
	jne	.LBB21_381
	jp	.LBB21_381
.Ltmp2425:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_381:
.Ltmp2426:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_384
	jp	.LBB21_384
.Ltmp2427:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_384:
.Ltmp2428:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp2429:
	.loc	26 161 24
	jbe	.LBB21_386
.Ltmp2430:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_386:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp2431:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2432:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2433:
	.loc	21 448 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp2434:
	.loc	21 439 26
	vmovss	928(%rbx), %xmm14
.Ltmp2435:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2436:
	.loc	21 441 44
	vmovss	924(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp2437:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2438:
	.loc	26 161 24
	jne	.LBB21_389
	jp	.LBB21_389
.Ltmp2439:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_389:
.Ltmp2440:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_392
	jp	.LBB21_392
.Ltmp2441:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_392:
.Ltmp2442:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp2443:
	.loc	26 161 24
	jbe	.LBB21_394
.Ltmp2444:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_394:
.Ltmp2445:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2446:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp2447:
	.loc	21 442 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp2448:
	.loc	26 103 24
	vandps	%xmm7, %xmm15, %xmm3
.Ltmp2449:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm6, %xmm6
.Ltmp2450:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp2451:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp2452:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp2453:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp2454:
	.loc	21 448 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp2455:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm2
.Ltmp2456:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2457:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp2458:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm2
.Ltmp2459:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp2460:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp2461:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp2462:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2463:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp2464:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp2465:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2466:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2467:
	.loc	26 161 24
	cmovbel	%r10d, %edx
.Ltmp2468:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2469:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2470:
	.loc	26 161 24
	cmovbel	%r11d, %edx
.Ltmp2471:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2472:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2473:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2474:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2475:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp2476:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2477:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2478:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2479:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2480:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2481:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2482:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2483:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2484:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp2485:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2486:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp2487:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2488:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2489:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2490:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2491:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp2492:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp2493:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp2494:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2495:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp2496:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2497:
	.loc	26 149 9
	movl	%r12d, %edx
.Ltmp2498:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm6
.Ltmp2499:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm6
.Ltmp2500:
	.loc	26 149 9
	notl	%edx
.Ltmp2501:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2502:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm2
.Ltmp2503:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2504:
	.loc	21 491 9
	vmovss	776(%rbx), %xmm3
.Ltmp2505:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2506:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2507:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_396
.Ltmp2508:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_396:
.Ltmp2509:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_398
.Ltmp2510:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_398:
.Ltmp2511:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp2512:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm13
.Ltmp2513:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 498 5
	movl	%edx, 932(%rbx)
.Ltmp2514:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2515:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp2516:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2517:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp2518:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp2519:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_307
.Ltmp2520:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_307
.LBB21_400:
.Ltmp2521:
	.loc	25 438 16 is_stmt 1
	movq	%rsi, %rax
	negq	%rax
	movl	%r13d, %r8d
	subl	%edi, %r8d
	movl	$1, %r15d
	movzbl	48(%rsp), %esi
	movl	%esi, 112(%rsp)
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$841731191, %r10d
	movl	$8388608, %r11d
	xorl	%r14d, %r14d
	movq	152(%rsp), %rsi
	movq	24(%rsp), %r9
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_402
.Ltmp2522:
	.loc	25 0 16 is_stmt 0
.Ltmp2523:
	.p2align	4
.LBB21_401:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp2524:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp2525:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2526:
	.loc	26 92 9
	vmulss	768(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2527:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2528:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp2529:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp2530:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp2531:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp2532:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp2533:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp2534:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp2535:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm15
.Ltmp2536:
	.loc	26 61 9
	vaddss	%xmm2, %xmm15, %xmm2
.Ltmp2537:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp2538:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp2539:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp2540:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp2541:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp2542:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp2543:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2544:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm4, %xmm2
.Ltmp2545:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm5, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm4, %xmm2
.Ltmp2546:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp2547:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp2548:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
.Ltmp2549:
	.loc	26 161 24
	vblendvps	%xmm9, %xmm2, %xmm4, %xmm2
.Ltmp2550:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm4
.Ltmp2551:
	.loc	26 66 9
	vsubss	%xmm4, %xmm3, %xmm3
.Ltmp2552:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp2553:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp2554:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2555:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2556:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2557:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp2558:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2559:
	.loc	26 61 9
	vaddss	%xmm6, %xmm15, %xmm6
.Ltmp2560:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp2561:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp2562:
	.loc	26 178 22
	vaddss	%xmm7, %xmm4, %xmm4
.Ltmp2563:
	.loc	7 1244 18
	vmovd	%xmm4, %edx
.Ltmp2564:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp2565:
	.loc	7 1291 18
	vmovd	%edx, %xmm4
.Ltmp2566:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp2567:
	.loc	21 510 5
	vmovss	%xmm10, 940(%rbx)
	vmovaps	128(%rsp), %xmm6
.Ltmp2568:
	.loc	26 71 9
	vmulss	%xmm3, %xmm6, %xmm3
.Ltmp2569:
	.loc	26 161 24
	vcmpneqss	%xmm5, %xmm10, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	vcmpnltss	780(%rbx), %xmm5, %xmm4
	vblendvps	%xmm4, %xmm3, %xmm6, %xmm3
	movq	88(%rsp), %rdx
.Ltmp2570:
	.loc	26 56 9
	vmovss	%xmm2, -4(%rdx,%r15,4)
.Ltmp2571:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rsi,%r15,4)
.Ltmp2572:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rax,%r15), %rdx
	incq	%rdx
	incq	%r15
	cmpq	$1, %rdx
	movq	72(%rsp), %rdx
	movq	80(%rsp), %rbp
.Ltmp2573:
	.loc	11 900 12
	je	.LBB21_494
.LBB21_402:
.Ltmp2574:
	.loc	15 971 17
	leaq	(%rax,%r15), %rdi
.Ltmp2575:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_775
.Ltmp2576:
	.loc	21 0 0 is_stmt 0
	leal	(%r15,%r13), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp2577:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp2578:
	.loc	25 0 16 is_stmt 0
	movq	88(%rsp), %r12
.Ltmp2579:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
.Ltmp2580:
	.loc	26 56 9
	vmovss	%xmm2, (%rbp,%rdi,4)
.Ltmp2581:
	.loc	26 51 9
	vmovss	-4(%rsi,%r15,4), %xmm2
.Ltmp2582:
	.loc	26 56 9
	vmovss	%xmm2, (%r9,%rdi,4)
	movq	64(%rsp), %r12
.Ltmp2583:
	.loc	26 51 9
	vmovss	-4(%r12,%r15,4), %xmm2
	movq	40(%rsp), %r12
.Ltmp2584:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
	movq	96(%rsp), %r12
.Ltmp2585:
	.loc	26 51 9
	vmovss	-4(%r12,%r15,4), %xmm2
	movq	32(%rsp), %r12
.Ltmp2586:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
.Ltmp2587:
	.loc	21 370 21
	leal	(%r8,%r15), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp2588:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp2589:
	.loc	26 51 9
	vmovss	(%rbp,%rdi,4), %xmm4
.Ltmp2590:
	.loc	26 51 9 is_stmt 0
	vmovss	(%r9,%rdi,4), %xmm2
.Ltmp2591:
	.loc	21 0 0
	leal	(%r15,%r13), %edx
	movl	136(%rbx), %r13d
	movl	200(%rbx), %ebp
	notl	%r13d
	addl	%edx, %r13d
	andl	%ecx, %r13d
	notl	%ebp
	addl	%edx, %ebp
	andl	%ecx, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 48(%rsp)
	vmovaps	%xmm2, 128(%rsp)
	je	.LBB21_410
	cmpl	$1, 112(%rsp)
	movq	16(%rsp), %rdx
	jne	.LBB21_413
	.loc	21 0 0 is_stmt 0
	cmpq	%r13, 8(%rsp)
.Ltmp2592:
	.loc	21 251 32 is_stmt 1
	jbe	.LBB21_801
.Ltmp2593:
	.loc	21 252 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_804
.Ltmp2594:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm6
	movq	32(%rsp), %rdx
.Ltmp2595:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm15
	vmovaps	%xmm15, %xmm10
	vmovaps	%xmm6, %xmm12
	jmp	.LBB21_418
.Ltmp2596:
	.loc	21 0 33 is_stmt 0
.Ltmp2597:
	.p2align	4
.LBB21_410:
	cmpq	%r13, 8(%rsp)
	movq	16(%rsp), %rdx
.Ltmp2598:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_802
.Ltmp2599:
	.loc	21 237 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_798
.Ltmp2600:
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%r13,4), %xmm10
	movq	32(%rsp), %rdx
.Ltmp2601:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm15
	vmovaps	%xmm10, %xmm12
.Ltmp2602:
	.loc	26 51 9
	jmp	.LBB21_418
.Ltmp2603:
	.loc	26 0 9 is_stmt 0
.Ltmp2604:
	.p2align	4
.LBB21_413:
	cmpq	%r13, 8(%rsp)
.Ltmp2605:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_803
	.loc	21 267 33
	cmpq	%r13, %rdx
	jbe	.LBB21_799
	.loc	21 268 33
	cmpq	%rbp, %rdx
	jbe	.LBB21_800
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	40(%rsp), %rdx
	vmovss	(%rdx,%r13,4), %xmm12
	movq	32(%rsp), %rdi
	vmovss	(%rdi,%r13,4), %xmm10
	.loc	21 268 33 is_stmt 1
	vmovss	(%rdi,%rbp,4), %xmm15
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm6
.Ltmp2606:
.LBB21_418:
	.loc	21 439 26
	vmovss	804(%rbx), %xmm9
.Ltmp2607:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp2608:
	.loc	21 441 44
	vmovss	800(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp2609:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm8, %xmm11
	movq	56(%rsp), %r13
.Ltmp2610:
	.loc	26 161 24
	jne	.LBB21_421
	jp	.LBB21_421
.Ltmp2611:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm11
.LBB21_421:
.Ltmp2612:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_424
	jp	.LBB21_424
.Ltmp2613:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_424:
.Ltmp2614:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm9
.Ltmp2615:
	.loc	26 161 24
	jbe	.LBB21_426
.Ltmp2616:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm8
.LBB21_426:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 800(%rbx)
.Ltmp2617:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2618:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp2619:
	.loc	21 448 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp2620:
	.loc	21 439 26
	vmovss	820(%rbx), %xmm11
.Ltmp2621:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp2622:
	.loc	21 441 27
	vmovss	808(%rbx), %xmm9
	.loc	21 441 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp2623:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp2624:
	.loc	26 161 24
	jne	.LBB21_429
	jp	.LBB21_429
.Ltmp2625:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_429:
.Ltmp2626:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_432
	jp	.LBB21_432
.Ltmp2627:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_432:
.Ltmp2628:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp2629:
	.loc	26 161 24
	jbe	.LBB21_434
.Ltmp2630:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_434:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp2631:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2632:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp2633:
	.loc	21 448 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp2634:
	.loc	21 439 26
	vmovss	836(%rbx), %xmm13
.Ltmp2635:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2636:
	.loc	21 441 27
	vmovss	824(%rbx), %xmm11
	.loc	21 441 44 is_stmt 0
	vmovss	832(%rbx), %xmm3
.Ltmp2637:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm14
.Ltmp2638:
	.loc	26 161 24
	jne	.LBB21_437
	jp	.LBB21_437
.Ltmp2639:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_437:
.Ltmp2640:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_440
	jp	.LBB21_440
.Ltmp2641:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_440:
.Ltmp2642:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp2643:
	.loc	26 161 24
	jbe	.LBB21_442
.Ltmp2644:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm11
.LBB21_442:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 824(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp2645:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2646:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2647:
	.loc	21 448 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp2648:
	.loc	21 439 26
	vmovss	852(%rbx), %xmm14
.Ltmp2649:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2650:
	.loc	21 441 44
	vmovss	848(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp2651:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2652:
	.loc	26 161 24
	jne	.LBB21_445
	jp	.LBB21_445
.Ltmp2653:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_445:
.Ltmp2654:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_448
	jp	.LBB21_448
.Ltmp2655:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_448:
.Ltmp2656:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp2657:
	.loc	26 161 24
	jbe	.LBB21_450
.Ltmp2658:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_450:
.Ltmp2659:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2660:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp2661:
	.loc	21 442 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp2662:
	.loc	26 103 24
	vandps	%xmm7, %xmm12, %xmm3
.Ltmp2663:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm10, %xmm7
.Ltmp2664:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp2665:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp2666:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp2667:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp2668:
	.loc	21 448 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp2669:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm2
.Ltmp2670:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2671:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp2672:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm2
.Ltmp2673:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm10
.Ltmp2674:
	.loc	26 71 9
	vmulss	%xmm3, %xmm10, %xmm2
.Ltmp2675:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm10, %xmm3
.Ltmp2676:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2677:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp2678:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp2679:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2680:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2681:
	.loc	26 161 24
	cmovbel	%r10d, %edx
.Ltmp2682:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2683:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2684:
	.loc	26 161 24
	cmovbel	%r11d, %edx
.Ltmp2685:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2686:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2687:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2688:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2689:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm7
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp2690:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2691:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2692:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2693:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2694:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2695:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2696:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2697:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2698:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp2699:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2700:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp2701:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2702:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2703:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2704:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2705:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm10
.Ltmp2706:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp2707:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm10
.Ltmp2708:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2709:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm10
.Ltmp2710:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2711:
	.loc	26 149 9
	movl	%r12d, %edx
.Ltmp2712:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm3
.Ltmp2713:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm3
.Ltmp2714:
	.loc	26 149 9
	notl	%edx
.Ltmp2715:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2716:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm2
.Ltmp2717:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2718:
	.loc	21 491 9
	vmovss	752(%rbx), %xmm12
.Ltmp2719:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2720:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2721:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_452
.Ltmp2722:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_452:
.Ltmp2723:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_454
.Ltmp2724:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_454:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm3
.Ltmp2725:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 498 5
	movl	%edx, 856(%rbx)
.Ltmp2726:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2727:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm10, %xmm7
.Ltmp2728:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm7, %xmm2, %xmm2
.Ltmp2729:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm7
	vxorps	%xmm7, %xmm11, %xmm7
.Ltmp2730:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm2, %xmm2
.Ltmp2731:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm7
	vandps	%xmm7, %xmm3, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_456
.Ltmp2732:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_456:
.Ltmp2733:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp2734:
	.loc	26 124 14
	xorl	%edx, %edx
	vucomiss	%xmm3, %xmm2
	setbe	%dl
.Ltmp2735:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2736:
	.loc	26 92 9
	vmulss	744(%rbx,%rdx,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2737:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2738:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp2739:
	.loc	21 510 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp2740:
	.loc	21 439 26
	vmovss	880(%rbx), %xmm11
.Ltmp2741:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp2742:
	.loc	21 441 44
	vmovss	876(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp2743:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp2744:
	.loc	26 161 24
	jne	.LBB21_459
	jp	.LBB21_459
.Ltmp2745:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_459:
.Ltmp2746:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_462
	jp	.LBB21_462
.Ltmp2747:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_462:
.Ltmp2748:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm11
.Ltmp2749:
	.loc	26 161 24
	jbe	.LBB21_464
.Ltmp2750:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_464:
.Ltmp2751:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm5, %xmm9
.Ltmp2752:
	.loc	21 442 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp2753:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2754:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp2755:
	.loc	21 448 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp2756:
	.loc	21 439 26
	vmovss	896(%rbx), %xmm12
.Ltmp2757:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp2758:
	.loc	21 441 44
	vmovss	892(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp2759:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp2760:
	.loc	26 161 24
	jne	.LBB21_467
	jp	.LBB21_467
.Ltmp2761:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_467:
.Ltmp2762:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_470
	jp	.LBB21_470
.Ltmp2763:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_470:
.Ltmp2764:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm12
.Ltmp2765:
	.loc	26 161 24
	jbe	.LBB21_472
.Ltmp2766:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_472:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp2767:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp2768:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp2769:
	.loc	21 448 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp2770:
	.loc	21 439 26
	vmovss	912(%rbx), %xmm13
.Ltmp2771:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2772:
	.loc	21 441 44
	vmovss	908(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp2773:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp2774:
	.loc	26 161 24
	jne	.LBB21_475
	jp	.LBB21_475
.Ltmp2775:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_475:
.Ltmp2776:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_478
	jp	.LBB21_478
.Ltmp2777:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_478:
.Ltmp2778:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm13
.Ltmp2779:
	.loc	26 161 24
	jbe	.LBB21_480
.Ltmp2780:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_480:
	.loc	21 442 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp2781:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2782:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm5, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2783:
	.loc	21 448 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp2784:
	.loc	21 439 26
	vmovss	928(%rbx), %xmm14
.Ltmp2785:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2786:
	.loc	21 441 44
	vmovss	924(%rbx), %xmm3
	.loc	21 441 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp2787:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2788:
	.loc	26 161 24
	jne	.LBB21_483
	jp	.LBB21_483
.Ltmp2789:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_483:
.Ltmp2790:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_486
	jp	.LBB21_486
.Ltmp2791:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_486:
.Ltmp2792:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm14
.Ltmp2793:
	.loc	26 161 24
	jbe	.LBB21_488
.Ltmp2794:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_488:
.Ltmp2795:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2796:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm5, %xmm7
	vblendvps	%xmm7, %xmm14, %xmm2, %xmm2
.Ltmp2797:
	.loc	21 442 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 447 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm7
.Ltmp2798:
	.loc	26 103 24
	vandps	%xmm7, %xmm15, %xmm3
.Ltmp2799:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm7, %xmm6, %xmm6
.Ltmp2800:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp2801:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp2802:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp2803:
	.loc	26 161 24 is_stmt 1
	cmoval	%edx, %ebp
.Ltmp2804:
	.loc	21 448 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp2805:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm2
.Ltmp2806:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2807:
	.loc	26 161 24
	cmovbel	%edx, %ebp
.Ltmp2808:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm2
.Ltmp2809:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp2810:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp2811:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp2812:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2813:
	.loc	7 1244 18
	vmovd	%xmm2, %edx
.Ltmp2814:
	.loc	26 161 24
	cmovbel	%ebp, %edx
.Ltmp2815:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2816:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2817:
	.loc	26 161 24
	cmovbel	%r10d, %edx
.Ltmp2818:
	.loc	7 1291 18
	vmovd	%edx, %xmm2
.Ltmp2819:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2820:
	.loc	26 161 24
	cmovbel	%r11d, %edx
.Ltmp2821:
	.loc	26 185 42
	movl	%edx, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2822:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2823:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2824:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2825:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp2826:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2827:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2828:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2829:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2830:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2831:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2832:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2833:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2834:
	.loc	26 187 28
	shrl	$23, %edx
	orl	$1258291200, %edx
.Ltmp2835:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2836:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp2837:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2838:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2839:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2840:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2841:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp2842:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp2843:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp2844:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2845:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp2846:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2847:
	.loc	26 149 9
	movl	%r12d, %edx
.Ltmp2848:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm6
.Ltmp2849:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm6
.Ltmp2850:
	.loc	26 149 9
	notl	%edx
.Ltmp2851:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2852:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm2
.Ltmp2853:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm2
.Ltmp2854:
	.loc	21 491 9
	vmovss	776(%rbx), %xmm3
.Ltmp2855:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2856:
	.loc	26 139 9
	cmovbel	%r14d, %edx
.Ltmp2857:
	.loc	26 161 24
	testb	$1, %dl
	je	.LBB21_490
.Ltmp2858:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_490:
.Ltmp2859:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_492
.Ltmp2860:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_492:
.Ltmp2861:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp2862:
	orl	%ebp, %edx
	andl	$1065353216, %edx
	vmovd	%edx, %xmm13
.Ltmp2863:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 498 5
	movl	%edx, 932(%rbx)
.Ltmp2864:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2865:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp2866:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2867:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp2868:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp2869:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm5, %xmm3
	vcmpltps	%xmm5, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %edx
	testb	$1, %dl
	jne	.LBB21_401
.Ltmp2870:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_401
.LBB21_494:
	movq	200(%rsp), %rsi
.Ltmp2871:
	.loc	15 2584 13 is_stmt 1
	leal	(%rsi,%r13), %eax
.Ltmp2872:
	.loc	21 413 5
	movl	%eax, 1208(%rbx)
	movq	336(%rsp), %r10
	movq	216(%rsp), %r8
	movq	208(%rsp), %r11
	movq	120(%rsp), %r9
	movq	144(%rsp), %r14
.Ltmp2873:
.LBB21_495:
	.loc	6 691 12
	cmpq	%r14, %r10
	jbe	.LBB21_744
.Ltmp2874:
	.loc	18 1161 15
	movq	(%r9), %rcx
	testq	%rcx, %rcx
	.loc	18 1161 9 is_stmt 0
	je	.LBB21_503
.Ltmp2875:
	.loc	18 1162 29 is_stmt 1
	movq	8(%r9), %rdx
.Ltmp2876:
	.loc	25 568 12
	movq	%rdx, %r15
	subq	%rsi, %r15
	jb	.LBB21_795
.Ltmp2877:
	.loc	18 1162 29
	movq	24(%r9), %rdx
.Ltmp2878:
	.loc	25 568 12
	movq	%rdx, %rax
	subq	%rsi, %rax
	jb	.LBB21_796
.Ltmp2879:
	.loc	18 1162 29
	movq	16(%r9), %rdx
.Ltmp2880:
	.loc	25 89 24
	leaq	(%rcx,%rsi,4), %rcx
	movq	%rcx, 80(%rsp)
.Ltmp2881:
	.loc	25 89 24 is_stmt 0
	leaq	(%rdx,%rsi,4), %rcx
	movq	%rcx, 112(%rsp)
.Ltmp2882:
	.loc	25 580 12 is_stmt 1
	movq	%rsi, %rcx
	subq	%r11, %rcx
	movq	%rcx, 144(%rsp)
	ja	.LBB21_504
.Ltmp2883:
.LBB21_500:
	.loc	25 580 12 is_stmt 0
	movq	%rsi, %rcx
	subq	%r8, %rcx
	movq	%rcx, 120(%rsp)
	ja	.LBB21_789
.Ltmp2884:
	.loc	21 61 8 is_stmt 1
	movl	136(%rbx), %ecx
	cmpl	200(%rbx), %ecx
	sete	%cl
	movb	$2, %dl
	subb	%cl, %dl
	xorl	%r12d, %r12d
	cmpl	$1, 68(%rbx)
	movzbl	%dl, %ecx
	cmovel	%r12d, %ecx
	movl	%ecx, 128(%rsp)
.Ltmp2885:
	.loc	18 1039 15
	cmpq	$0, 80(%rsp)
	.loc	18 1039 9 is_stmt 0
	je	.LBB21_505
.Ltmp2886:
	.loc	18 0 9
	movq	%rax, %r12
	jmp	.LBB21_506
.LBB21_503:
	movq	$0, 80(%rsp)
.Ltmp2887:
	.loc	25 580 12 is_stmt 1
	movq	%rsi, %rcx
	subq	%r11, %rcx
	movq	%rcx, 144(%rsp)
	jbe	.LBB21_500
.LBB21_504:
	.loc	25 581 13
	leaq	.Lalloc_1e79f4c3c2f015f90ab54e70b61044ab(%rip), %rcx
	movq	%rsi, %rdi
	movq	%r11, %rsi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp2888:
.LBB21_505:
	.loc	25 0 13 is_stmt 0
	movl	$4, %eax
	movq	%rax, 112(%rsp)
	movq	%rax, 80(%rsp)
	xorl	%r15d, %r15d
.LBB21_506:
	movq	88(%rsp), %rax
	leaq	(%rax,%rsi,4), %rax
	movq	%rax, 40(%rsp)
	movq	152(%rsp), %rax
	leaq	(%rax,%rsi,4), %rax
	movq	%rax, 32(%rsp)
.Ltmp2889:
	movq	104(%rbx), %rax
	movq	%rax, 72(%rsp)
	movq	112(%rbx), %rdx
	movq	120(%rbx), %rax
	movq	%rax, 16(%rsp)
	movq	128(%rbx), %rax
	movq	%rax, 8(%rsp)
	movq	168(%rbx), %rax
	movq	%rax, 56(%rsp)
	movq	176(%rbx), %r13
	movq	184(%rbx), %rax
	movq	%rax, 24(%rsp)
	movq	192(%rbx), %rax
	movq	%rax, 96(%rsp)
	movl	1212(%rbx), %eax
	movl	1216(%rbx), %edi
.Ltmp2890:
	.loc	21 353 16 is_stmt 1
	movl	1208(%rbx), %ebp
	cmpq	%r8, %r11
	movq	%rdx, 64(%rsp)
	movq	%rbp, 48(%rsp)
	movq	%r12, 168(%rsp)
	jbe	.LBB21_543
	.loc	21 0 16 is_stmt 0
	subq	%rsi, %r8
	movq	%r8, 160(%rsp)
.Ltmp2891:
	.loc	25 451 16 is_stmt 1
	negq	%r12
	movq	%r15, 176(%rsp)
	movq	%r15, %r8
	negq	%r8
	movl	%ebp, %r9d
	subl	%edi, %r9d
	movq	%rsi, %r11
	subq	%r10, %r11
	movl	$1, %r14d
	movzbl	128(%rsp), %ecx
	movl	%ecx, 144(%rsp)
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_4(%rip), %xmm10
	movl	$841731191, %esi
	movl	$8388608, %r10d
	vmovss	.LCPI21_1(%rip), %xmm11
	xorl	%r15d, %r15d
	vmovss	.LCPI21_25(%rip), %xmm13
	vmovss	.LCPI21_26(%rip), %xmm14
	jmp	.LBB21_509
.Ltmp2892:
	.loc	25 0 16 is_stmt 0
.Ltmp2893:
	.p2align	4
.LBB21_508:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm8
.Ltmp2894:
	.loc	26 124 14
	xorl	%ecx, %ecx
	vucomiss	%xmm8, %xmm5
	setbe	%cl
.Ltmp2895:
	.loc	26 66 9
	vsubss	%xmm8, %xmm5, %xmm5
.Ltmp2896:
	.loc	26 92 9
	vmulss	768(%rbx,%rcx,4), %xmm5, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
.Ltmp2897:
	.loc	26 103 24
	vandps	%xmm5, %xmm12, %xmm8
.Ltmp2898:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm8, %xmm8
	vandps	%xmm5, %xmm8, %xmm5
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp2899:
	.loc	26 71 9
	vmulss	%xmm7, %xmm9, %xmm8
	vmovss	.LCPI21_22(%rip), %xmm10
.Ltmp2900:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp2901:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_23(%rip), %xmm11
.Ltmp2902:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp2903:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_24(%rip), %xmm12
.Ltmp2904:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp2905:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp2906:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp2907:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
	vmovss	.LCPI21_0(%rip), %xmm1
.Ltmp2908:
	.loc	26 61 9
	vaddss	%xmm1, %xmm7, %xmm7
.Ltmp2909:
	.loc	26 178 22
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp2910:
	.loc	7 1244 18
	vmovd	%xmm6, %ecx
.Ltmp2911:
	.loc	26 179 24
	shll	$23, %ecx
.Ltmp2912:
	.loc	7 1291 18
	vmovd	%ecx, %xmm6
.Ltmp2913:
	.loc	26 71 9
	vmulss	%xmm6, %xmm7, %xmm6
.Ltmp2914:
	.loc	26 161 24
	vcmpnltss	756(%rbx), %xmm2, %xmm7
.Ltmp2915:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp2916:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm6, %xmm4, %xmm3
.Ltmp2917:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm5, %xmm6
.Ltmp2918:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
.Ltmp2919:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm6, %xmm6
.Ltmp2920:
	.loc	26 161 24
	vblendvps	%xmm7, %xmm3, %xmm4, %xmm3
.Ltmp2921:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm6, %xmm6, %xmm7
.Ltmp2922:
	.loc	26 66 9
	vsubss	%xmm7, %xmm6, %xmm6
.Ltmp2923:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm8
.Ltmp2924:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp2925:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp2926:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp2927:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp2928:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp2929:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp2930:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp2931:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm6
.Ltmp2932:
	.loc	26 61 9
	vaddss	%xmm1, %xmm6, %xmm6
.Ltmp2933:
	.loc	26 178 22
	vaddss	%xmm7, %xmm14, %xmm7
.Ltmp2934:
	.loc	7 1244 18
	vmovd	%xmm7, %ecx
.Ltmp2935:
	.loc	26 179 24
	shll	$23, %ecx
.Ltmp2936:
	.loc	7 1291 18
	vmovd	%ecx, %xmm7
.Ltmp2937:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp2938:
	.loc	21 510 5
	vmovss	%xmm5, 940(%rbx)
.Ltmp2939:
	.loc	26 71 9
	vmulss	%xmm6, %xmm15, %xmm6
.Ltmp2940:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm5, %xmm5
	vblendvps	%xmm5, %xmm6, %xmm15, %xmm5
	vcmpnltss	780(%rbx), %xmm2, %xmm2
	vblendvps	%xmm2, %xmm5, %xmm15, %xmm2
	movq	40(%rsp), %rcx
.Ltmp2941:
	.loc	26 56 9
	vmovss	%xmm3, -4(%rcx,%r14,4)
	movq	32(%rsp), %rcx
.Ltmp2942:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm2, -4(%rcx,%r14,4)
.Ltmp2943:
	.loc	8 1916 50 is_stmt 1
	leaq	(%r11,%r14), %rcx
	incq	%rcx
	incq	%r14
	cmpq	$1, %rcx
	movq	64(%rsp), %rdx
	movq	48(%rsp), %rbp
	vmovss	.LCPI21_4(%rip), %xmm10
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovaps	%xmm0, %xmm11
.Ltmp2944:
	.loc	11 900 12
	je	.LBB21_743
.Ltmp2945:
.LBB21_509:
	.loc	21 361 22
	leal	(%r14,%rbp), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp2946:
	.loc	25 451 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp2947:
	.loc	25 0 16 is_stmt 0
	movq	120(%rsp), %rcx
	leaq	(%rcx,%r14), %rbp
	movq	40(%rsp), %rcx
.Ltmp2948:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r14,4), %xmm2
	movq	72(%rsp), %rcx
.Ltmp2949:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%rdi,4)
.Ltmp2950:
	.loc	25 438 16
	cmpq	$1, %rbp
	je	.LBB21_784
.Ltmp2951:
	.loc	25 451 16
	cmpq	%rdi, %r13
	jbe	.LBB21_780
.Ltmp2952:
	.loc	21 0 0 is_stmt 0
	leaq	(%r8,%r14), %rbp
	movq	32(%rsp), %rcx
.Ltmp2953:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r14,4), %xmm2
	movq	56(%rsp), %rcx
.Ltmp2954:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%rdi,4)
.Ltmp2955:
	.loc	25 438 16
	cmpq	$1, %rbp
	je	.LBB21_779
.Ltmp2956:
	.loc	25 451 16
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_711
.Ltmp2957:
	.loc	21 0 0 is_stmt 0
	leaq	(%r12,%r14), %rbp
	movq	80(%rsp), %rcx
.Ltmp2958:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r14,4), %xmm2
	movq	16(%rsp), %rcx
.Ltmp2959:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%rdi,4)
.Ltmp2960:
	.loc	25 438 16
	cmpq	$1, %rbp
	je	.LBB21_778
.Ltmp2961:
	.loc	25 451 16
	cmpq	%rdi, 96(%rsp)
	jbe	.LBB21_777
.Ltmp2962:
	.loc	25 0 16 is_stmt 0
	movq	112(%rsp), %rcx
.Ltmp2963:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r14,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp2964:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%rdi,4)
.Ltmp2965:
	.loc	21 370 21
	leal	(%r9,%r14), %edi
	decl	%edi
	andl	%eax, %edi
	movq	64(%rsp), %rdx
.Ltmp2966:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp2967:
	.loc	25 438 16 is_stmt 0
	cmpq	%rdi, %r13
	jbe	.LBB21_781
.Ltmp2968:
	.loc	25 0 16
	movq	72(%rsp), %rcx
.Ltmp2969:
	.loc	26 51 9 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm4
	movq	56(%rsp), %rcx
.Ltmp2970:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rcx,%rdi,4), %xmm15
	movq	48(%rsp), %rcx
.Ltmp2971:
	.loc	21 0 0
	addl	%r14d, %ecx
	movl	136(%rbx), %edi
	movl	200(%rbx), %ebp
	notl	%edi
	addl	%ecx, %edi
	andl	%eax, %edi
	notl	%ebp
	addl	%ecx, %ebp
	andl	%eax, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$2, 128(%rsp)
	je	.LBB21_523
	cmpl	$1, 144(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
	jne	.LBB21_528
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
.Ltmp2972:
	.loc	21 251 32 is_stmt 1
	jbe	.LBB21_809
.Ltmp2973:
	.loc	21 252 33
	cmpq	%rbp, 96(%rsp)
	jbe	.LBB21_811
.Ltmp2974:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm5
	movq	24(%rsp), %rcx
.Ltmp2975:
	.loc	21 252 33
	vmovss	(%rcx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm5, %xmm3
.Ltmp2976:
	.loc	26 51 9
	jmp	.LBB21_531
.Ltmp2977:
	.loc	26 0 9 is_stmt 0
.Ltmp2978:
	.p2align	4
.LBB21_523:
	cmpq	%rdi, 8(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
.Ltmp2979:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_810
	.loc	21 0 33 is_stmt 0
	movq	96(%rsp), %rcx
	.loc	21 267 33 is_stmt 1
	cmpq	%rdi, %rcx
	jbe	.LBB21_807
	.loc	21 268 33
	cmpq	%rbp, %rcx
	jbe	.LBB21_808
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	vmovss	(%rcx,%rdi,4), %xmm3
	movq	24(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm2
	.loc	21 268 33 is_stmt 1
	vmovss	(%rdx,%rbp,4), %xmm6
	.loc	21 269 33
	vmovss	(%rcx,%rbp,4), %xmm5
	jmp	.LBB21_531
.Ltmp2980:
	.loc	21 0 33 is_stmt 0
.Ltmp2981:
	.p2align	4
.LBB21_528:
	cmpq	%rdi, 8(%rsp)
.Ltmp2982:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_805
.Ltmp2983:
	.loc	21 237 33
	cmpq	%rbp, 96(%rsp)
	jbe	.LBB21_806
.Ltmp2984:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp2985:
	.loc	21 237 33
	vmovss	(%rcx,%rbp,4), %xmm5
	vmovaps	%xmm5, %xmm6
	vmovaps	%xmm2, %xmm3
.Ltmp2986:
.LBB21_531:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm3
.Ltmp2987:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm12, %xmm7
.Ltmp2988:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp2989:
	.loc	7 1244 18
	vmovd	%xmm3, %ecx
.Ltmp2990:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp2991:
	.loc	26 161 24 is_stmt 1
	cmoval	%ecx, %ebp
.Ltmp2992:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm8
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp2993:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp2994:
	.loc	26 161 24
	cmovbel	%ecx, %ebp
.Ltmp2995:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm8
.Ltmp2996:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp2997:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp2998:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm7, %xmm7
.Ltmp2999:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3000:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3001:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3002:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp3003:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm3
.Ltmp3004:
	.loc	26 161 24
	cmovbel	%esi, %edi
.Ltmp3005:
	.loc	21 451 21
	vmovss	792(%rbx), %xmm3
.Ltmp3006:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3007:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm7
.Ltmp3008:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3009:
	.loc	26 185 42
	movl	%edi, %ecx
	andl	$8388607, %ecx
	orl	$1065353216, %ecx
.Ltmp3010:
	.loc	7 1291 18
	vmovd	%ecx, %xmm7
.Ltmp3011:
	.loc	26 66 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp3012:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm7, %xmm8
.Ltmp3013:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm9
	vsubss	%xmm8, %xmm9, %xmm8
.Ltmp3014:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3015:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm8, %xmm8
.Ltmp3016:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3017:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm8, %xmm8
.Ltmp3018:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3019:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm8, %xmm8
.Ltmp3020:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3021:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm8, %xmm8
.Ltmp3022:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3023:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3024:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm9, %xmm9
.Ltmp3025:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
.Ltmp3026:
	.loc	26 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp3027:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm7, %xmm7
.Ltmp3028:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm7, %xmm7
.Ltmp3029:
	.loc	26 66 9
	vsubss	840(%rbx), %xmm3, %xmm8
.Ltmp3030:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm7, %xmm7
.Ltmp3031:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm7
.Ltmp3032:
	.loc	26 28 5
	movl	$0, %ecx
	adcl	$-1, %ecx
.Ltmp3033:
	.loc	26 129 14
	vucomiss	%xmm3, %xmm7
.Ltmp3034:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3035:
	.loc	26 149 9
	movl	%ecx, %edi
.Ltmp3036:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm8
.Ltmp3037:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3038:
	.loc	26 149 9
	notl	%edi
.Ltmp3039:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3040:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm9
.Ltmp3041:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp3042:
	.loc	26 144 9
	cmoval	%ecx, %ebp
.Ltmp3043:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3044:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_533
.Ltmp3045:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm8, %xmm8
.LBB21_533:
.Ltmp3046:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_535
.Ltmp3047:
	.loc	21 0 0 is_stmt 0
	vmovss	752(%rbx), %xmm8
.LBB21_535:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm9
.Ltmp3048:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm8, 860(%rbx)
	.loc	21 498 5
	movl	%edi, 856(%rbx)
.Ltmp3049:
	.loc	26 66 9
	vaddss	808(%rbx), %xmm11, %xmm8
.Ltmp3050:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp3051:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm8, %xmm3
.Ltmp3052:
	.loc	26 98 24
	vmovss	824(%rbx), %xmm7
	vbroadcastss	.LCPI21_16(%rip), %xmm8
	vxorps	%xmm7, %xmm8, %xmm7
.Ltmp3053:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm3, %xmm3
.Ltmp3054:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm9, %xmm2, %xmm7
	vcmpltps	%xmm2, %xmm3, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovd	%xmm7, %ecx
	testb	$1, %cl
	jne	.LBB21_537
.Ltmp3055:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_537:
.Ltmp3056:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm7
.Ltmp3057:
	.loc	26 124 14
	xorl	%ecx, %ecx
	vucomiss	%xmm7, %xmm3
	setbe	%cl
.Ltmp3058:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp3059:
	.loc	26 92 9
	vmulss	744(%rbx,%rcx,4), %xmm3, %xmm3
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3060:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm7
.Ltmp3061:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm7, %xmm7
	vandps	%xmm3, %xmm7, %xmm3
.Ltmp3062:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm6
.Ltmp3063:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm5, %xmm12, %xmm5
.Ltmp3064:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm6
.Ltmp3065:
	.loc	7 1244 18
	vmovd	%xmm6, %ecx
.Ltmp3066:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm5, %ebp
.Ltmp3067:
	.loc	26 161 24 is_stmt 1
	cmoval	%ecx, %ebp
.Ltmp3068:
	.loc	21 510 5
	vmovss	%xmm3, 864(%rbx)
.Ltmp3069:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm7
.Ltmp3070:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp3071:
	.loc	26 161 24
	cmovbel	%ecx, %ebp
.Ltmp3072:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm7
.Ltmp3073:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp3074:
	.loc	26 71 9
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp3075:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm5, %xmm5
.Ltmp3076:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm5, %xmm5
.Ltmp3077:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp3078:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3079:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp3080:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm5
.Ltmp3081:
	.loc	26 161 24
	cmovbel	%esi, %edi
.Ltmp3082:
	.loc	21 451 21
	vmovss	868(%rbx), %xmm5
.Ltmp3083:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp3084:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm6
.Ltmp3085:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3086:
	.loc	26 185 42
	movl	%edi, %ecx
	andl	$8388607, %ecx
	orl	$1065353216, %ecx
.Ltmp3087:
	.loc	7 1291 18
	vmovd	%ecx, %xmm6
.Ltmp3088:
	.loc	26 66 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp3089:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm6, %xmm7
.Ltmp3090:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm8
	vsubss	%xmm7, %xmm8, %xmm7
.Ltmp3091:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3092:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm7, %xmm7
.Ltmp3093:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3094:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm7, %xmm7
.Ltmp3095:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3096:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm7, %xmm7
.Ltmp3097:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3098:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm7, %xmm7
.Ltmp3099:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3100:
	.loc	7 1291 18
	vmovd	%edi, %xmm8
.Ltmp3101:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm8, %xmm8
.Ltmp3102:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp3103:
	.loc	26 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp3104:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm6, %xmm6
.Ltmp3105:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm6, %xmm6
.Ltmp3106:
	.loc	26 66 9
	vsubss	916(%rbx), %xmm5, %xmm7
.Ltmp3107:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm6, %xmm8
.Ltmp3108:
	.loc	26 129 14
	vucomiss	%xmm7, %xmm8
.Ltmp3109:
	.loc	26 28 5
	movl	$0, %ecx
	adcl	$-1, %ecx
.Ltmp3110:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm8
.Ltmp3111:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3112:
	.loc	26 149 9
	movl	%ecx, %edi
.Ltmp3113:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm9
.Ltmp3114:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp3115:
	.loc	26 149 9
	notl	%edi
.Ltmp3116:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3117:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm6
.Ltmp3118:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm6
.Ltmp3119:
	.loc	26 144 9
	cmoval	%ecx, %ebp
.Ltmp3120:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3121:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_539
.Ltmp3122:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm9, %xmm9
.LBB21_539:
.Ltmp3123:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_541
.Ltmp3124:
	.loc	21 0 0 is_stmt 0
	vmovss	776(%rbx), %xmm9
.Ltmp3125:
.LBB21_541:
	vmulss	.LCPI21_18(%rip), %xmm3, %xmm6
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
	vminss	.LCPI21_20(%rip), %xmm6, %xmm7
	vroundss	$9, %xmm7, %xmm7, %xmm6
	vsubss	%xmm6, %xmm7, %xmm7
.Ltmp3126:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp3127:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm9, 936(%rbx)
	.loc	21 498 5
	movl	%edi, 932(%rbx)
	vmovaps	%xmm11, %xmm0
.Ltmp3128:
	.loc	26 66 9
	vaddss	884(%rbx), %xmm11, %xmm9
.Ltmp3129:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm5, %xmm8, %xmm5
.Ltmp3130:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm9, %xmm5
.Ltmp3131:
	.loc	26 98 24
	vmovss	900(%rbx), %xmm8
	vbroadcastss	.LCPI21_16(%rip), %xmm9
	vxorps	%xmm9, %xmm8, %xmm8
.Ltmp3132:
	.loc	26 161 24
	vmaxss	%xmm8, %xmm5, %xmm5
.Ltmp3133:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm10, %xmm2, %xmm8
	vcmpltps	%xmm2, %xmm5, %xmm9
	vandps	%xmm9, %xmm8, %xmm8
	vmovd	%xmm8, %ecx
	testb	$1, %cl
	jne	.LBB21_508
.Ltmp3134:
	.loc	26 0 44
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_508
.LBB21_543:
	movq	%r11, %r14
	subq	%rsi, %r14
	movzbl	128(%rsp), %ecx
	movl	%ecx, 120(%rsp)
	cmpq	%r13, %rdx
	jbe	.LBB21_580
	movq	%r14, 160(%rsp)
.Ltmp3135:
	.loc	25 438 16 is_stmt 1
	movq	%r12, %rcx
	negq	%rcx
	movq	%r15, 176(%rsp)
	movq	%r15, %r9
	negq	%r9
	movl	%ebp, %r8d
	subl	%edi, %r8d
	movq	%rsi, %r11
	subq	%r10, %r11
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_4(%rip), %xmm10
	movl	$8388608, %r10d
	vmovss	.LCPI21_1(%rip), %xmm11
	xorl	%r15d, %r15d
	vmovss	.LCPI21_25(%rip), %xmm13
	vmovss	.LCPI21_26(%rip), %xmm14
	jmp	.LBB21_546
.Ltmp3136:
	.loc	25 0 16 is_stmt 0
.Ltmp3137:
	.p2align	4
.LBB21_545:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm8
.Ltmp3138:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm8, %xmm5
	setbe	%dil
.Ltmp3139:
	.loc	26 66 9
	vsubss	%xmm8, %xmm5, %xmm5
.Ltmp3140:
	.loc	26 92 9
	vmulss	768(%rbx,%rdi,4), %xmm5, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
.Ltmp3141:
	.loc	26 103 24
	vandps	%xmm5, %xmm12, %xmm8
.Ltmp3142:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm8, %xmm8
	vandps	%xmm5, %xmm8, %xmm5
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp3143:
	.loc	26 71 9
	vmulss	%xmm7, %xmm9, %xmm8
	vmovss	.LCPI21_22(%rip), %xmm10
.Ltmp3144:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp3145:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_23(%rip), %xmm11
.Ltmp3146:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp3147:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_24(%rip), %xmm12
.Ltmp3148:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp3149:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3150:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp3151:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
	vmovss	.LCPI21_0(%rip), %xmm1
.Ltmp3152:
	.loc	26 61 9
	vaddss	%xmm1, %xmm7, %xmm7
.Ltmp3153:
	.loc	26 178 22
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp3154:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp3155:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3156:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp3157:
	.loc	26 71 9
	vmulss	%xmm6, %xmm7, %xmm6
.Ltmp3158:
	.loc	26 161 24
	vcmpnltss	756(%rbx), %xmm2, %xmm7
.Ltmp3159:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp3160:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm6, %xmm4, %xmm3
.Ltmp3161:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm5, %xmm6
.Ltmp3162:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
.Ltmp3163:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm6, %xmm6
.Ltmp3164:
	.loc	26 161 24
	vblendvps	%xmm7, %xmm3, %xmm4, %xmm3
.Ltmp3165:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm6, %xmm6, %xmm7
.Ltmp3166:
	.loc	26 66 9
	vsubss	%xmm7, %xmm6, %xmm6
.Ltmp3167:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm8
.Ltmp3168:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp3169:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3170:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp3171:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3172:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp3173:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3174:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp3175:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm6
.Ltmp3176:
	.loc	26 61 9
	vaddss	%xmm1, %xmm6, %xmm6
.Ltmp3177:
	.loc	26 178 22
	vaddss	%xmm7, %xmm14, %xmm7
.Ltmp3178:
	.loc	7 1244 18
	vmovd	%xmm7, %edi
.Ltmp3179:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3180:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3181:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp3182:
	.loc	21 510 5
	vmovss	%xmm5, 940(%rbx)
.Ltmp3183:
	.loc	26 71 9
	vmulss	%xmm6, %xmm15, %xmm6
.Ltmp3184:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm5, %xmm5
	vblendvps	%xmm5, %xmm6, %xmm15, %xmm5
	vcmpnltss	780(%rbx), %xmm2, %xmm2
	vblendvps	%xmm2, %xmm5, %xmm15, %xmm2
	movq	40(%rsp), %rdx
.Ltmp3185:
	.loc	26 56 9
	vmovss	%xmm3, -4(%rdx,%r14,4)
	movq	32(%rsp), %rdx
.Ltmp3186:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm2, -4(%rdx,%r14,4)
.Ltmp3187:
	.loc	8 1916 50 is_stmt 1
	leaq	(%r11,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	movq	64(%rsp), %rdx
	movq	48(%rsp), %rbp
	vmovss	.LCPI21_4(%rip), %xmm10
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovaps	%xmm0, %xmm11
.Ltmp3188:
	.loc	11 900 12
	je	.LBB21_743
.LBB21_546:
	.loc	11 0 12 is_stmt 0
	movq	144(%rsp), %rsi
.Ltmp3189:
	.loc	15 971 17 is_stmt 1
	leaq	(%rsi,%r14), %rdi
.Ltmp3190:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_786
.Ltmp3191:
	.loc	21 0 0 is_stmt 0
	leal	(%r14,%rbp), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp3192:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp3193:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %rdx
.Ltmp3194:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rdx,%r14,4), %xmm2
	movq	72(%rsp), %rdx
.Ltmp3195:
	.loc	26 56 9
	vmovss	%xmm2, (%rdx,%rdi,4)
.Ltmp3196:
	.loc	25 451 16
	cmpq	%rdi, %r13
	movq	96(%rsp), %rsi
	jbe	.LBB21_780
.Ltmp3197:
	.loc	21 0 0 is_stmt 0
	leaq	(%r9,%r14), %r12
	movq	32(%rsp), %rdx
.Ltmp3198:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rdx,%r14,4), %xmm2
	movq	56(%rsp), %rdx
.Ltmp3199:
	.loc	26 56 9
	vmovss	%xmm2, (%rdx,%rdi,4)
.Ltmp3200:
	.loc	25 438 16
	cmpq	$1, %r12
	je	.LBB21_779
.Ltmp3201:
	.loc	25 451 16
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_711
.Ltmp3202:
	.loc	21 0 0 is_stmt 0
	leaq	(%rcx,%r14), %r12
	movq	80(%rsp), %rbp
.Ltmp3203:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rbp,%r14,4), %xmm2
	movq	16(%rsp), %rdx
.Ltmp3204:
	.loc	26 56 9
	vmovss	%xmm2, (%rdx,%rdi,4)
.Ltmp3205:
	.loc	25 438 16
	cmpq	$1, %r12
	je	.LBB21_778
.Ltmp3206:
	.loc	25 451 16
	cmpq	%rdi, %rsi
	jbe	.LBB21_777
.Ltmp3207:
	.loc	25 0 16 is_stmt 0
	movq	112(%rsp), %r12
.Ltmp3208:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r14,4), %xmm2
	movq	24(%rsp), %rdx
.Ltmp3209:
	.loc	26 56 9
	vmovss	%xmm2, (%rdx,%rdi,4)
.Ltmp3210:
	.loc	21 370 21
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%eax, %edi
	movq	64(%rsp), %rdx
.Ltmp3211:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp3212:
	.loc	25 438 16 is_stmt 0
	cmpq	%rdi, %r13
	jbe	.LBB21_781
.Ltmp3213:
	.loc	25 0 16
	movq	72(%rsp), %rdx
.Ltmp3214:
	.loc	26 51 9 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm4
	movq	56(%rsp), %rdx
.Ltmp3215:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm15
	movq	48(%rsp), %rdx
.Ltmp3216:
	.loc	21 0 0
	leal	(%rdx,%r14), %r12d
	movl	136(%rbx), %edi
	movl	200(%rbx), %ebp
	notl	%edi
	addl	%r12d, %edi
	andl	%eax, %edi
	notl	%ebp
	addl	%r12d, %ebp
	andl	%eax, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 128(%rsp)
	je	.LBB21_560
	cmpl	$1, 120(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
	jne	.LBB21_563
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_809
.Ltmp3217:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, %rsi
	jbe	.LBB21_811
.Ltmp3218:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm5
	movq	24(%rsp), %rdx
.Ltmp3219:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm5, %xmm3
	jmp	.LBB21_568
.Ltmp3220:
	.loc	21 0 33 is_stmt 0
.Ltmp3221:
	.p2align	4
.LBB21_560:
	cmpq	%rdi, 8(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
.Ltmp3222:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_805
.Ltmp3223:
	.loc	21 237 33
	cmpq	%rbp, %rsi
	jbe	.LBB21_806
.Ltmp3224:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm2
	movq	24(%rsp), %rdx
.Ltmp3225:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm5
	vmovaps	%xmm5, %xmm6
	vmovaps	%xmm2, %xmm3
	jmp	.LBB21_568
.Ltmp3226:
	.loc	21 0 33 is_stmt 0
.Ltmp3227:
	.p2align	4
.LBB21_563:
	cmpq	%rdi, 8(%rsp)
.Ltmp3228:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_810
	.loc	21 267 33
	cmpq	%rdi, %rsi
	jbe	.LBB21_807
	.loc	21 268 33
	cmpq	%rbp, %rsi
	jbe	.LBB21_808
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm3
	movq	24(%rsp), %rsi
	vmovss	(%rsi,%rdi,4), %xmm2
	.loc	21 268 33 is_stmt 1
	vmovss	(%rsi,%rbp,4), %xmm6
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm5
.Ltmp3229:
.LBB21_568:
	.loc	21 0 33 is_stmt 0
	movl	$841731191, %edx
.Ltmp3230:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm3, %xmm12, %xmm3
.Ltmp3231:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm12, %xmm7
.Ltmp3232:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp3233:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3234:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp3235:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
.Ltmp3236:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm8
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp3237:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3238:
	.loc	26 161 24
	cmovbel	%edi, %ebp
.Ltmp3239:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm8
.Ltmp3240:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3241:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp3242:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm7, %xmm7
.Ltmp3243:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3244:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3245:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3246:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp3247:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm3
.Ltmp3248:
	.loc	26 161 24
	cmovbel	%edx, %edi
.Ltmp3249:
	.loc	21 451 21
	vmovss	792(%rbx), %xmm3
.Ltmp3250:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3251:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm7
.Ltmp3252:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3253:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp3254:
	.loc	7 1291 18
	vmovd	%ebp, %xmm7
.Ltmp3255:
	.loc	26 66 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp3256:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm7, %xmm8
.Ltmp3257:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm9
	vsubss	%xmm8, %xmm9, %xmm8
.Ltmp3258:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3259:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm8, %xmm8
.Ltmp3260:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3261:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm8, %xmm8
.Ltmp3262:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3263:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm8, %xmm8
.Ltmp3264:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3265:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm8, %xmm8
.Ltmp3266:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3267:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3268:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm9, %xmm9
.Ltmp3269:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
.Ltmp3270:
	.loc	26 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp3271:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm7, %xmm7
.Ltmp3272:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm7, %xmm7
.Ltmp3273:
	.loc	26 66 9
	vsubss	840(%rbx), %xmm3, %xmm8
.Ltmp3274:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm7, %xmm7
.Ltmp3275:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm7
.Ltmp3276:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp3277:
	.loc	26 129 14
	vucomiss	%xmm3, %xmm7
.Ltmp3278:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3279:
	.loc	26 149 9
	movl	%r12d, %edi
.Ltmp3280:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm8
.Ltmp3281:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3282:
	.loc	26 149 9
	notl	%edi
.Ltmp3283:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3284:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm9
.Ltmp3285:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp3286:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp3287:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3288:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_570
.Ltmp3289:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm8, %xmm8
.LBB21_570:
.Ltmp3290:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_572
.Ltmp3291:
	.loc	21 0 0 is_stmt 0
	vmovss	752(%rbx), %xmm8
.LBB21_572:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm9
.Ltmp3292:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm8, 860(%rbx)
	.loc	21 498 5
	movl	%edi, 856(%rbx)
.Ltmp3293:
	.loc	26 66 9
	vaddss	808(%rbx), %xmm11, %xmm8
.Ltmp3294:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp3295:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm8, %xmm3
.Ltmp3296:
	.loc	26 98 24
	vmovss	824(%rbx), %xmm7
	vbroadcastss	.LCPI21_16(%rip), %xmm8
	vxorps	%xmm7, %xmm8, %xmm7
.Ltmp3297:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm3, %xmm3
.Ltmp3298:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm9, %xmm2, %xmm7
	vcmpltps	%xmm2, %xmm3, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovd	%xmm7, %edi
	testb	$1, %dil
	jne	.LBB21_574
.Ltmp3299:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_574:
.Ltmp3300:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm7
.Ltmp3301:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm7, %xmm3
	setbe	%dil
.Ltmp3302:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp3303:
	.loc	26 92 9
	vmulss	744(%rbx,%rdi,4), %xmm3, %xmm3
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3304:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm7
.Ltmp3305:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm7, %xmm7
	vandps	%xmm3, %xmm7, %xmm3
.Ltmp3306:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm6
.Ltmp3307:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm5, %xmm12, %xmm5
.Ltmp3308:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm6
.Ltmp3309:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp3310:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm5, %ebp
.Ltmp3311:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
.Ltmp3312:
	.loc	21 510 5
	vmovss	%xmm3, 864(%rbx)
.Ltmp3313:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm7
.Ltmp3314:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp3315:
	.loc	26 161 24
	cmovbel	%edi, %ebp
.Ltmp3316:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm7
.Ltmp3317:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp3318:
	.loc	26 71 9
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp3319:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm5, %xmm5
.Ltmp3320:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm5, %xmm5
.Ltmp3321:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp3322:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3323:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp3324:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm5
.Ltmp3325:
	.loc	26 161 24
	cmovbel	%edx, %edi
.Ltmp3326:
	.loc	21 451 21
	vmovss	868(%rbx), %xmm5
.Ltmp3327:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp3328:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm6
.Ltmp3329:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3330:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp3331:
	.loc	7 1291 18
	vmovd	%ebp, %xmm6
.Ltmp3332:
	.loc	26 66 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp3333:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm6, %xmm7
.Ltmp3334:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm8
	vsubss	%xmm7, %xmm8, %xmm7
.Ltmp3335:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3336:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm7, %xmm7
.Ltmp3337:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3338:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm7, %xmm7
.Ltmp3339:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3340:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm7, %xmm7
.Ltmp3341:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3342:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm7, %xmm7
.Ltmp3343:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3344:
	.loc	7 1291 18
	vmovd	%edi, %xmm8
.Ltmp3345:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm8, %xmm8
.Ltmp3346:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp3347:
	.loc	26 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp3348:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm6, %xmm6
.Ltmp3349:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm6, %xmm6
.Ltmp3350:
	.loc	26 66 9
	vsubss	916(%rbx), %xmm5, %xmm7
.Ltmp3351:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm6, %xmm8
.Ltmp3352:
	.loc	26 129 14
	vucomiss	%xmm7, %xmm8
.Ltmp3353:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp3354:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm8
.Ltmp3355:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3356:
	.loc	26 149 9
	movl	%r12d, %edi
.Ltmp3357:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm9
.Ltmp3358:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp3359:
	.loc	26 149 9
	notl	%edi
.Ltmp3360:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3361:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm6
.Ltmp3362:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm6
.Ltmp3363:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp3364:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3365:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_576
.Ltmp3366:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm9, %xmm9
.LBB21_576:
.Ltmp3367:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_578
.Ltmp3368:
	.loc	21 0 0 is_stmt 0
	vmovss	776(%rbx), %xmm9
.Ltmp3369:
.LBB21_578:
	vmulss	.LCPI21_18(%rip), %xmm3, %xmm6
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
	vminss	.LCPI21_20(%rip), %xmm6, %xmm7
	vroundss	$9, %xmm7, %xmm7, %xmm6
	vsubss	%xmm6, %xmm7, %xmm7
.Ltmp3370:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp3371:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm9, 936(%rbx)
	.loc	21 498 5
	movl	%edi, 932(%rbx)
	vmovaps	%xmm11, %xmm0
.Ltmp3372:
	.loc	26 66 9
	vaddss	884(%rbx), %xmm11, %xmm9
.Ltmp3373:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm5, %xmm8, %xmm5
.Ltmp3374:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm9, %xmm5
.Ltmp3375:
	.loc	26 98 24
	vmovss	900(%rbx), %xmm8
	vbroadcastss	.LCPI21_16(%rip), %xmm9
	vxorps	%xmm9, %xmm8, %xmm8
.Ltmp3376:
	.loc	26 161 24
	vmaxss	%xmm8, %xmm5, %xmm5
.Ltmp3377:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm10, %xmm2, %xmm8
	vcmpltps	%xmm2, %xmm5, %xmm9
	vandps	%xmm9, %xmm8, %xmm8
	vmovd	%xmm8, %edi
	testb	$1, %dil
	jne	.LBB21_545
.Ltmp3378:
	.loc	26 0 44
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_545
.LBB21_580:
	cmpq	%r15, %r14
	jbe	.LBB21_614
.Ltmp3379:
	.loc	25 451 16 is_stmt 1
	movq	%r12, %r13
	negq	%r13
	movq	%r15, 176(%rsp)
	movq	%r15, %rcx
	negq	%rcx
	movl	%ebp, %r8d
	subl	%edi, %r8d
	movq	%rsi, %r9
	subq	%r10, %r9
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_4(%rip), %xmm10
	movl	$841731191, %r10d
	movl	$8388608, %r11d
	vmovss	.LCPI21_1(%rip), %xmm11
	xorl	%r15d, %r15d
	vmovss	.LCPI21_25(%rip), %xmm13
	vmovss	.LCPI21_26(%rip), %xmm14
	jmp	.LBB21_583
.Ltmp3380:
	.loc	25 0 16 is_stmt 0
.Ltmp3381:
	.p2align	4
.LBB21_582:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm8
.Ltmp3382:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm8, %xmm5
	setbe	%dil
.Ltmp3383:
	.loc	26 66 9
	vsubss	%xmm8, %xmm5, %xmm5
.Ltmp3384:
	.loc	26 92 9
	vmulss	768(%rbx,%rdi,4), %xmm5, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
.Ltmp3385:
	.loc	26 103 24
	vandps	%xmm5, %xmm12, %xmm8
.Ltmp3386:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm8, %xmm8
	vandps	%xmm5, %xmm8, %xmm5
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp3387:
	.loc	26 71 9
	vmulss	%xmm7, %xmm9, %xmm8
	vmovss	.LCPI21_22(%rip), %xmm10
.Ltmp3388:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp3389:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_23(%rip), %xmm11
.Ltmp3390:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp3391:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_24(%rip), %xmm12
.Ltmp3392:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp3393:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3394:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp3395:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
	vmovss	.LCPI21_0(%rip), %xmm1
.Ltmp3396:
	.loc	26 61 9
	vaddss	%xmm1, %xmm7, %xmm7
.Ltmp3397:
	.loc	26 178 22
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp3398:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp3399:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3400:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp3401:
	.loc	26 71 9
	vmulss	%xmm6, %xmm7, %xmm6
.Ltmp3402:
	.loc	26 161 24
	vcmpnltss	756(%rbx), %xmm2, %xmm7
.Ltmp3403:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp3404:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm6, %xmm4, %xmm3
.Ltmp3405:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm5, %xmm6
.Ltmp3406:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
.Ltmp3407:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm6, %xmm6
.Ltmp3408:
	.loc	26 161 24
	vblendvps	%xmm7, %xmm3, %xmm4, %xmm3
.Ltmp3409:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm6, %xmm6, %xmm7
.Ltmp3410:
	.loc	26 66 9
	vsubss	%xmm7, %xmm6, %xmm6
.Ltmp3411:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm8
.Ltmp3412:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp3413:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3414:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp3415:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3416:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp3417:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3418:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp3419:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm6
.Ltmp3420:
	.loc	26 61 9
	vaddss	%xmm1, %xmm6, %xmm6
.Ltmp3421:
	.loc	26 178 22
	vaddss	%xmm7, %xmm14, %xmm7
.Ltmp3422:
	.loc	7 1244 18
	vmovd	%xmm7, %edi
.Ltmp3423:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3424:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3425:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp3426:
	.loc	21 510 5
	vmovss	%xmm5, 940(%rbx)
.Ltmp3427:
	.loc	26 71 9
	vmulss	%xmm6, %xmm15, %xmm6
.Ltmp3428:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm5, %xmm5
	vblendvps	%xmm5, %xmm6, %xmm15, %xmm5
	vcmpnltss	780(%rbx), %xmm2, %xmm2
	vblendvps	%xmm2, %xmm5, %xmm15, %xmm2
	movq	40(%rsp), %rdx
.Ltmp3429:
	.loc	26 56 9
	vmovss	%xmm3, -4(%rdx,%r14,4)
	movq	32(%rsp), %rdx
.Ltmp3430:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm2, -4(%rdx,%r14,4)
.Ltmp3431:
	.loc	8 1916 50 is_stmt 1
	leaq	(%r9,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	movq	64(%rsp), %rdx
	movq	48(%rsp), %rbp
	vmovss	.LCPI21_4(%rip), %xmm10
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovaps	%xmm0, %xmm11
.Ltmp3432:
	.loc	11 900 12
	je	.LBB21_743
.Ltmp3433:
.LBB21_583:
	.loc	21 361 22
	leal	(%r14,%rbp), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp3434:
	.loc	25 451 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp3435:
	.loc	21 0 0 is_stmt 0
	leaq	(%rcx,%r14), %r12
	movq	40(%rsp), %rsi
.Ltmp3436:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rsi,%r14,4), %xmm2
	movq	72(%rsp), %rsi
.Ltmp3437:
	.loc	26 56 9
	vmovss	%xmm2, (%rsi,%rdi,4)
	movq	32(%rsp), %rsi
.Ltmp3438:
	.loc	26 51 9
	vmovss	-4(%rsi,%r14,4), %xmm2
	movq	56(%rsp), %rsi
.Ltmp3439:
	.loc	26 56 9
	vmovss	%xmm2, (%rsi,%rdi,4)
.Ltmp3440:
	.loc	25 438 16
	cmpq	$1, %r12
	je	.LBB21_779
.Ltmp3441:
	.loc	25 451 16
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_711
.Ltmp3442:
	.loc	21 0 0 is_stmt 0
	leaq	(%r14,%r13), %r12
	movq	80(%rsp), %rsi
.Ltmp3443:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rsi,%r14,4), %xmm2
	movq	16(%rsp), %rsi
.Ltmp3444:
	.loc	26 56 9
	vmovss	%xmm2, (%rsi,%rdi,4)
.Ltmp3445:
	.loc	25 438 16
	cmpq	$1, %r12
	je	.LBB21_778
.Ltmp3446:
	.loc	25 0 16 is_stmt 0
	movq	96(%rsp), %rsi
.Ltmp3447:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB21_777
.Ltmp3448:
	.loc	25 0 16 is_stmt 0
	movq	112(%rsp), %r12
.Ltmp3449:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r14,4), %xmm2
	movq	24(%rsp), %r12
.Ltmp3450:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
.Ltmp3451:
	.loc	21 370 21
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp3452:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp3453:
	.loc	25 0 16 is_stmt 0
	movq	72(%rsp), %rdx
.Ltmp3454:
	.loc	26 51 9 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm4
	movq	56(%rsp), %rdx
.Ltmp3455:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm15
.Ltmp3456:
	.loc	21 0 0
	leal	(%r14,%rbp), %r12d
	movl	136(%rbx), %edi
	movl	200(%rbx), %ebp
	notl	%edi
	addl	%r12d, %edi
	andl	%eax, %edi
	notl	%ebp
	addl	%r12d, %ebp
	andl	%eax, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 128(%rsp)
	je	.LBB21_594
	cmpl	$1, 120(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
	jne	.LBB21_597
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_809
.Ltmp3457:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, %rsi
	jbe	.LBB21_811
.Ltmp3458:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm5
	movq	24(%rsp), %rdx
.Ltmp3459:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm5, %xmm3
	jmp	.LBB21_602
.Ltmp3460:
	.loc	21 0 33 is_stmt 0
.Ltmp3461:
	.p2align	4
.LBB21_594:
	cmpq	%rdi, 8(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
.Ltmp3462:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_805
.Ltmp3463:
	.loc	21 237 33
	cmpq	%rbp, %rsi
	jbe	.LBB21_806
.Ltmp3464:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm2
	movq	24(%rsp), %rdx
.Ltmp3465:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm5
	vmovaps	%xmm5, %xmm6
	vmovaps	%xmm2, %xmm3
.Ltmp3466:
	.loc	26 51 9
	jmp	.LBB21_602
.Ltmp3467:
	.loc	26 0 9 is_stmt 0
.Ltmp3468:
	.p2align	4
.LBB21_597:
	cmpq	%rdi, 8(%rsp)
.Ltmp3469:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_810
	.loc	21 267 33
	cmpq	%rdi, %rsi
	jbe	.LBB21_807
	.loc	21 268 33
	cmpq	%rbp, %rsi
	jbe	.LBB21_808
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm3
	movq	24(%rsp), %rsi
	vmovss	(%rsi,%rdi,4), %xmm2
	.loc	21 268 33 is_stmt 1
	vmovss	(%rsi,%rbp,4), %xmm6
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm5
.Ltmp3470:
.LBB21_602:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm3
.Ltmp3471:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm12, %xmm7
.Ltmp3472:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp3473:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3474:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp3475:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
.Ltmp3476:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm8
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp3477:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3478:
	.loc	26 161 24
	cmovbel	%edi, %ebp
.Ltmp3479:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm8
.Ltmp3480:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3481:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp3482:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm7, %xmm7
.Ltmp3483:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3484:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3485:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3486:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp3487:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm3
.Ltmp3488:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3489:
	.loc	21 451 21
	vmovss	792(%rbx), %xmm3
.Ltmp3490:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3491:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm7
.Ltmp3492:
	.loc	26 161 24
	cmovbel	%r11d, %edi
.Ltmp3493:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp3494:
	.loc	7 1291 18
	vmovd	%ebp, %xmm7
.Ltmp3495:
	.loc	26 66 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp3496:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm7, %xmm8
.Ltmp3497:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm9
	vsubss	%xmm8, %xmm9, %xmm8
.Ltmp3498:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3499:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm8, %xmm8
.Ltmp3500:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3501:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm8, %xmm8
.Ltmp3502:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3503:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm8, %xmm8
.Ltmp3504:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3505:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm8, %xmm8
.Ltmp3506:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3507:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3508:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm9, %xmm9
.Ltmp3509:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
.Ltmp3510:
	.loc	26 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp3511:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm7, %xmm7
.Ltmp3512:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm7, %xmm7
.Ltmp3513:
	.loc	26 66 9
	vsubss	840(%rbx), %xmm3, %xmm8
.Ltmp3514:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm7, %xmm7
.Ltmp3515:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm7
.Ltmp3516:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp3517:
	.loc	26 129 14
	vucomiss	%xmm3, %xmm7
.Ltmp3518:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3519:
	.loc	26 149 9
	movl	%r12d, %edi
.Ltmp3520:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm8
.Ltmp3521:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3522:
	.loc	26 149 9
	notl	%edi
.Ltmp3523:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3524:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm9
.Ltmp3525:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp3526:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp3527:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3528:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_604
.Ltmp3529:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm8, %xmm8
.LBB21_604:
.Ltmp3530:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_606
.Ltmp3531:
	.loc	21 0 0 is_stmt 0
	vmovss	752(%rbx), %xmm8
.LBB21_606:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm9
.Ltmp3532:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm8, 860(%rbx)
	.loc	21 498 5
	movl	%edi, 856(%rbx)
.Ltmp3533:
	.loc	26 66 9
	vaddss	808(%rbx), %xmm11, %xmm8
.Ltmp3534:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp3535:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm8, %xmm3
.Ltmp3536:
	.loc	26 98 24
	vmovss	824(%rbx), %xmm7
	vbroadcastss	.LCPI21_16(%rip), %xmm8
	vxorps	%xmm7, %xmm8, %xmm7
.Ltmp3537:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm3, %xmm3
.Ltmp3538:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm9, %xmm2, %xmm7
	vcmpltps	%xmm2, %xmm3, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovd	%xmm7, %edi
	testb	$1, %dil
	jne	.LBB21_608
.Ltmp3539:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_608:
.Ltmp3540:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm7
.Ltmp3541:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm7, %xmm3
	setbe	%dil
.Ltmp3542:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp3543:
	.loc	26 92 9
	vmulss	744(%rbx,%rdi,4), %xmm3, %xmm3
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3544:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm7
.Ltmp3545:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm7, %xmm7
	vandps	%xmm3, %xmm7, %xmm3
.Ltmp3546:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm6
.Ltmp3547:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm5, %xmm12, %xmm5
.Ltmp3548:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm6
.Ltmp3549:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp3550:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm5, %ebp
.Ltmp3551:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
.Ltmp3552:
	.loc	21 510 5
	vmovss	%xmm3, 864(%rbx)
.Ltmp3553:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm7
.Ltmp3554:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp3555:
	.loc	26 161 24
	cmovbel	%edi, %ebp
.Ltmp3556:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm7
.Ltmp3557:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp3558:
	.loc	26 71 9
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp3559:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm5, %xmm5
.Ltmp3560:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm5, %xmm5
.Ltmp3561:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp3562:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3563:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp3564:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm5
.Ltmp3565:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3566:
	.loc	21 451 21
	vmovss	868(%rbx), %xmm5
.Ltmp3567:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp3568:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm6
.Ltmp3569:
	.loc	26 161 24
	cmovbel	%r11d, %edi
.Ltmp3570:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp3571:
	.loc	7 1291 18
	vmovd	%ebp, %xmm6
.Ltmp3572:
	.loc	26 66 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp3573:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm6, %xmm7
.Ltmp3574:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm8
	vsubss	%xmm7, %xmm8, %xmm7
.Ltmp3575:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3576:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm7, %xmm7
.Ltmp3577:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3578:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm7, %xmm7
.Ltmp3579:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3580:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm7, %xmm7
.Ltmp3581:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3582:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm7, %xmm7
.Ltmp3583:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3584:
	.loc	7 1291 18
	vmovd	%edi, %xmm8
.Ltmp3585:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm8, %xmm8
.Ltmp3586:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp3587:
	.loc	26 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp3588:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm6, %xmm6
.Ltmp3589:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm6, %xmm6
.Ltmp3590:
	.loc	26 66 9
	vsubss	916(%rbx), %xmm5, %xmm7
.Ltmp3591:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm6, %xmm8
.Ltmp3592:
	.loc	26 129 14
	vucomiss	%xmm7, %xmm8
.Ltmp3593:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp3594:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm8
.Ltmp3595:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3596:
	.loc	26 149 9
	movl	%r12d, %edi
.Ltmp3597:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm9
.Ltmp3598:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp3599:
	.loc	26 149 9
	notl	%edi
.Ltmp3600:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3601:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm6
.Ltmp3602:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm6
.Ltmp3603:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp3604:
	.loc	26 139 9
	cmovbel	%r15d, %edi
.Ltmp3605:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_610
.Ltmp3606:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm9, %xmm9
.LBB21_610:
.Ltmp3607:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_612
.Ltmp3608:
	.loc	21 0 0 is_stmt 0
	vmovss	776(%rbx), %xmm9
.Ltmp3609:
.LBB21_612:
	vmulss	.LCPI21_18(%rip), %xmm3, %xmm6
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
	vminss	.LCPI21_20(%rip), %xmm6, %xmm7
	vroundss	$9, %xmm7, %xmm7, %xmm6
	vsubss	%xmm6, %xmm7, %xmm7
.Ltmp3610:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp3611:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm9, 936(%rbx)
	.loc	21 498 5
	movl	%edi, 932(%rbx)
	vmovaps	%xmm11, %xmm0
.Ltmp3612:
	.loc	26 66 9
	vaddss	884(%rbx), %xmm11, %xmm9
.Ltmp3613:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm5, %xmm8, %xmm5
.Ltmp3614:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm9, %xmm5
.Ltmp3615:
	.loc	26 98 24
	vmovss	900(%rbx), %xmm8
	vbroadcastss	.LCPI21_16(%rip), %xmm9
	vxorps	%xmm9, %xmm8, %xmm8
.Ltmp3616:
	.loc	26 161 24
	vmaxss	%xmm8, %xmm5, %xmm5
.Ltmp3617:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm10, %xmm2, %xmm8
	vcmpltps	%xmm2, %xmm5, %xmm9
	vandps	%xmm9, %xmm8, %xmm8
	vmovd	%xmm8, %edi
	testb	$1, %dil
	jne	.LBB21_582
.Ltmp3618:
	.loc	26 0 44
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_582
.LBB21_614:
	cmpq	8(%rsp), %rdx
	jbe	.LBB21_648
	movq	%r14, 160(%rsp)
.Ltmp3619:
	.loc	25 438 16 is_stmt 1
	negq	%r12
	movl	%ebp, %ecx
	subl	%edi, %ecx
	movq	%rsi, %r8
	subq	%r10, %r8
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm4
	vmovss	.LCPI21_4(%rip), %xmm10
	movl	$841731191, %r9d
	movl	$8388608, %r10d
	vmovss	.LCPI21_1(%rip), %xmm11
	xorl	%r11d, %r11d
	movq	72(%rsp), %r13
	vmovss	.LCPI21_25(%rip), %xmm13
	vmovss	.LCPI21_26(%rip), %xmm14
	jmp	.LBB21_617
.Ltmp3620:
	.loc	25 0 16 is_stmt 0
.Ltmp3621:
	.p2align	4
.LBB21_616:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm8
.Ltmp3622:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm8, %xmm5
	setbe	%dil
.Ltmp3623:
	.loc	26 66 9
	vsubss	%xmm8, %xmm5, %xmm5
.Ltmp3624:
	.loc	26 92 9
	vmulss	768(%rbx,%rdi,4), %xmm5, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
.Ltmp3625:
	.loc	26 103 24
	vandps	%xmm5, %xmm12, %xmm8
.Ltmp3626:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm8, %xmm8
	vandps	%xmm5, %xmm8, %xmm5
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp3627:
	.loc	26 71 9
	vmulss	%xmm7, %xmm9, %xmm8
	vmovss	.LCPI21_22(%rip), %xmm10
.Ltmp3628:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp3629:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_23(%rip), %xmm11
.Ltmp3630:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp3631:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_24(%rip), %xmm12
.Ltmp3632:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp3633:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3634:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp3635:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
	vmovss	.LCPI21_0(%rip), %xmm1
.Ltmp3636:
	.loc	26 61 9
	vaddss	%xmm1, %xmm7, %xmm7
.Ltmp3637:
	.loc	26 178 22
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp3638:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp3639:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3640:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp3641:
	.loc	26 71 9
	vmulss	%xmm6, %xmm7, %xmm6
.Ltmp3642:
	.loc	26 161 24
	vcmpnltss	756(%rbx), %xmm2, %xmm7
.Ltmp3643:
	.loc	26 71 9
	vmulss	%xmm6, %xmm15, %xmm6
.Ltmp3644:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm6, %xmm15, %xmm3
.Ltmp3645:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm5, %xmm6
.Ltmp3646:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
.Ltmp3647:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm6, %xmm6
.Ltmp3648:
	.loc	26 161 24
	vblendvps	%xmm7, %xmm3, %xmm15, %xmm3
.Ltmp3649:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm6, %xmm6, %xmm7
.Ltmp3650:
	.loc	26 66 9
	vsubss	%xmm7, %xmm6, %xmm6
.Ltmp3651:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm8
.Ltmp3652:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp3653:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3654:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp3655:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3656:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp3657:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3658:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp3659:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm6
.Ltmp3660:
	.loc	26 61 9
	vaddss	%xmm1, %xmm6, %xmm6
.Ltmp3661:
	.loc	26 178 22
	vaddss	%xmm7, %xmm14, %xmm7
.Ltmp3662:
	.loc	7 1244 18
	vmovd	%xmm7, %edi
.Ltmp3663:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3664:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3665:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp3666:
	.loc	21 510 5
	vmovss	%xmm5, 940(%rbx)
	vmovaps	176(%rsp), %xmm1
.Ltmp3667:
	.loc	26 71 9
	vmulss	%xmm6, %xmm1, %xmm6
.Ltmp3668:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm5, %xmm5
	vblendvps	%xmm5, %xmm6, %xmm1, %xmm5
	vcmpnltss	780(%rbx), %xmm2, %xmm2
	vblendvps	%xmm2, %xmm5, %xmm1, %xmm2
	movq	40(%rsp), %rdx
.Ltmp3669:
	.loc	26 56 9
	vmovss	%xmm3, -4(%rdx,%r14,4)
	movq	32(%rsp), %rdx
.Ltmp3670:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm2, -4(%rdx,%r14,4)
.Ltmp3671:
	.loc	8 1916 50 is_stmt 1
	leaq	(%r8,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	movq	64(%rsp), %rdx
	movq	48(%rsp), %rbp
	vmovss	.LCPI21_4(%rip), %xmm10
	vmovaps	%xmm0, %xmm11
.Ltmp3672:
	.loc	11 900 12
	je	.LBB21_743
.LBB21_617:
	.loc	11 0 12 is_stmt 0
	movq	144(%rsp), %rsi
.Ltmp3673:
	.loc	15 971 17 is_stmt 1
	leaq	(%rsi,%r14), %rdi
.Ltmp3674:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_786
.Ltmp3675:
	.loc	21 0 0 is_stmt 0
	leal	(%r14,%rbp), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp3676:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp3677:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %rsi
.Ltmp3678:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rsi,%r14,4), %xmm2
.Ltmp3679:
	.loc	26 56 9
	vmovss	%xmm2, (%r13,%rdi,4)
	movq	32(%rsp), %rsi
.Ltmp3680:
	.loc	26 51 9
	vmovss	-4(%rsi,%r14,4), %xmm2
	movq	56(%rsp), %rsi
.Ltmp3681:
	.loc	26 56 9
	vmovss	%xmm2, (%rsi,%rdi,4)
.Ltmp3682:
	.loc	25 451 16
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_711
.Ltmp3683:
	.loc	21 0 0 is_stmt 0
	leaq	(%r12,%r14), %r15
	movq	80(%rsp), %rsi
.Ltmp3684:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rsi,%r14,4), %xmm2
	movq	16(%rsp), %rsi
.Ltmp3685:
	.loc	26 56 9
	vmovss	%xmm2, (%rsi,%rdi,4)
.Ltmp3686:
	.loc	25 438 16
	cmpq	$1, %r15
	je	.LBB21_778
.Ltmp3687:
	.loc	25 0 16 is_stmt 0
	movq	96(%rsp), %rsi
.Ltmp3688:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB21_777
.Ltmp3689:
	.loc	25 0 16 is_stmt 0
	movq	112(%rsp), %r15
.Ltmp3690:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r15,%r14,4), %xmm2
	movq	24(%rsp), %r15
.Ltmp3691:
	.loc	26 56 9
	vmovss	%xmm2, (%r15,%rdi,4)
.Ltmp3692:
	.loc	21 370 21
	leal	(%rcx,%r14), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp3693:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp3694:
	.loc	26 51 9
	vmovss	(%r13,%rdi,4), %xmm15
	movq	56(%rsp), %rdx
.Ltmp3695:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm0
.Ltmp3696:
	.loc	21 0 0
	leal	(%r14,%rbp), %r15d
	movl	136(%rbx), %edi
	movl	200(%rbx), %ebp
	notl	%edi
	addl	%r15d, %edi
	andl	%eax, %edi
	notl	%ebp
	addl	%r15d, %ebp
	andl	%eax, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 128(%rsp)
	vmovaps	%xmm0, 176(%rsp)
	je	.LBB21_628
	cmpl	$1, 120(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
	jne	.LBB21_631
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_809
.Ltmp3697:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, %rsi
	jbe	.LBB21_811
.Ltmp3698:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm5
	movq	24(%rsp), %rdx
.Ltmp3699:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm5, %xmm3
	jmp	.LBB21_636
.Ltmp3700:
	.loc	21 0 33 is_stmt 0
.Ltmp3701:
	.p2align	4
.LBB21_628:
	cmpq	%rdi, 8(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
.Ltmp3702:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_805
.Ltmp3703:
	.loc	21 237 33
	cmpq	%rbp, %rsi
	jbe	.LBB21_806
.Ltmp3704:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm2
	movq	24(%rsp), %rdx
.Ltmp3705:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm5
	vmovaps	%xmm5, %xmm6
	vmovaps	%xmm2, %xmm3
.Ltmp3706:
	.loc	26 51 9
	jmp	.LBB21_636
.Ltmp3707:
	.loc	26 0 9 is_stmt 0
.Ltmp3708:
	.p2align	4
.LBB21_631:
	cmpq	%rdi, 8(%rsp)
.Ltmp3709:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_810
	.loc	21 267 33
	cmpq	%rdi, %rsi
	jbe	.LBB21_807
	.loc	21 268 33
	cmpq	%rbp, %rsi
	jbe	.LBB21_808
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm3
	movq	24(%rsp), %rsi
	vmovss	(%rsi,%rdi,4), %xmm2
	.loc	21 268 33 is_stmt 1
	vmovss	(%rsi,%rbp,4), %xmm6
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm5
.Ltmp3710:
.LBB21_636:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm3
.Ltmp3711:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm12, %xmm7
.Ltmp3712:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp3713:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3714:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp3715:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
.Ltmp3716:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm8
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp3717:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3718:
	.loc	26 161 24
	cmovbel	%edi, %ebp
.Ltmp3719:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm8
.Ltmp3720:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3721:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp3722:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm4, %xmm7, %xmm7
.Ltmp3723:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3724:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3725:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3726:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp3727:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm3
.Ltmp3728:
	.loc	26 161 24
	cmovbel	%r9d, %edi
.Ltmp3729:
	.loc	21 451 21
	vmovss	792(%rbx), %xmm3
.Ltmp3730:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3731:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm7
.Ltmp3732:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3733:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp3734:
	.loc	7 1291 18
	vmovd	%ebp, %xmm7
.Ltmp3735:
	.loc	26 66 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp3736:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm7, %xmm8
.Ltmp3737:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm9
	vsubss	%xmm8, %xmm9, %xmm8
.Ltmp3738:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3739:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm8, %xmm8
.Ltmp3740:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3741:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm8, %xmm8
.Ltmp3742:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3743:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm8, %xmm8
.Ltmp3744:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3745:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm8, %xmm8
.Ltmp3746:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3747:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3748:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm9, %xmm9
.Ltmp3749:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
.Ltmp3750:
	.loc	26 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp3751:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm7, %xmm7
.Ltmp3752:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm7, %xmm7
.Ltmp3753:
	.loc	26 66 9
	vsubss	840(%rbx), %xmm3, %xmm8
.Ltmp3754:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm7, %xmm7
.Ltmp3755:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm7
.Ltmp3756:
	.loc	26 28 5
	movl	$0, %r15d
	adcl	$-1, %r15d
.Ltmp3757:
	.loc	26 129 14
	vucomiss	%xmm3, %xmm7
.Ltmp3758:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3759:
	.loc	26 149 9
	movl	%r15d, %edi
.Ltmp3760:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm8
.Ltmp3761:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3762:
	.loc	26 149 9
	notl	%edi
.Ltmp3763:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp3764:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm9
.Ltmp3765:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp3766:
	.loc	26 144 9
	cmoval	%r15d, %ebp
.Ltmp3767:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp3768:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_638
.Ltmp3769:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm8, %xmm8
.LBB21_638:
.Ltmp3770:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_640
.Ltmp3771:
	.loc	21 0 0 is_stmt 0
	vmovss	752(%rbx), %xmm8
.LBB21_640:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm9
.Ltmp3772:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm8, 860(%rbx)
	.loc	21 498 5
	movl	%edi, 856(%rbx)
.Ltmp3773:
	.loc	26 66 9
	vaddss	808(%rbx), %xmm11, %xmm8
.Ltmp3774:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp3775:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm8, %xmm3
.Ltmp3776:
	.loc	26 98 24
	vmovss	824(%rbx), %xmm7
	vbroadcastss	.LCPI21_16(%rip), %xmm8
	vxorps	%xmm7, %xmm8, %xmm7
.Ltmp3777:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm3, %xmm3
.Ltmp3778:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm9, %xmm2, %xmm7
	vcmpltps	%xmm2, %xmm3, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovd	%xmm7, %edi
	testb	$1, %dil
	jne	.LBB21_642
.Ltmp3779:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_642:
.Ltmp3780:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm7
.Ltmp3781:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm7, %xmm3
	setbe	%dil
.Ltmp3782:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp3783:
	.loc	26 92 9
	vmulss	744(%rbx,%rdi,4), %xmm3, %xmm3
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3784:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm7
.Ltmp3785:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm7, %xmm7
	vandps	%xmm3, %xmm7, %xmm3
.Ltmp3786:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm6
.Ltmp3787:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm5, %xmm12, %xmm5
.Ltmp3788:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm6
.Ltmp3789:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp3790:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm5, %ebp
.Ltmp3791:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
.Ltmp3792:
	.loc	21 510 5
	vmovss	%xmm3, 864(%rbx)
.Ltmp3793:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm7
.Ltmp3794:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp3795:
	.loc	26 161 24
	cmovbel	%edi, %ebp
.Ltmp3796:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm7
.Ltmp3797:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp3798:
	.loc	26 71 9
	vmulss	%xmm4, %xmm6, %xmm6
.Ltmp3799:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm4, %xmm5, %xmm5
.Ltmp3800:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm5, %xmm5
.Ltmp3801:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp3802:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3803:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp3804:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm5
.Ltmp3805:
	.loc	26 161 24
	cmovbel	%r9d, %edi
.Ltmp3806:
	.loc	21 451 21
	vmovss	868(%rbx), %xmm5
.Ltmp3807:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp3808:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm6
.Ltmp3809:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3810:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp3811:
	.loc	7 1291 18
	vmovd	%ebp, %xmm6
.Ltmp3812:
	.loc	26 66 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp3813:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm6, %xmm7
.Ltmp3814:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm8
	vsubss	%xmm7, %xmm8, %xmm7
.Ltmp3815:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3816:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm7, %xmm7
.Ltmp3817:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3818:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm7, %xmm7
.Ltmp3819:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3820:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm7, %xmm7
.Ltmp3821:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp3822:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm7, %xmm7
.Ltmp3823:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3824:
	.loc	7 1291 18
	vmovd	%edi, %xmm8
.Ltmp3825:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm8, %xmm8
.Ltmp3826:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp3827:
	.loc	26 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp3828:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm6, %xmm6
.Ltmp3829:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm6, %xmm6
.Ltmp3830:
	.loc	26 66 9
	vsubss	916(%rbx), %xmm5, %xmm7
.Ltmp3831:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm6, %xmm8
.Ltmp3832:
	.loc	26 129 14
	vucomiss	%xmm7, %xmm8
.Ltmp3833:
	.loc	26 28 5
	movl	$0, %r15d
	adcl	$-1, %r15d
.Ltmp3834:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm8
.Ltmp3835:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3836:
	.loc	26 149 9
	movl	%r15d, %edi
.Ltmp3837:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm9
.Ltmp3838:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp3839:
	.loc	26 149 9
	notl	%edi
.Ltmp3840:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp3841:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm6
.Ltmp3842:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm6
.Ltmp3843:
	.loc	26 144 9
	cmoval	%r15d, %ebp
.Ltmp3844:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp3845:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_644
.Ltmp3846:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm9, %xmm9
.LBB21_644:
.Ltmp3847:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_646
.Ltmp3848:
	.loc	21 0 0 is_stmt 0
	vmovss	776(%rbx), %xmm9
.Ltmp3849:
.LBB21_646:
	vmulss	.LCPI21_18(%rip), %xmm3, %xmm6
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
	vminss	.LCPI21_20(%rip), %xmm6, %xmm7
	vroundss	$9, %xmm7, %xmm7, %xmm6
	vsubss	%xmm6, %xmm7, %xmm7
.Ltmp3850:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp3851:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm9, 936(%rbx)
	.loc	21 498 5
	movl	%edi, 932(%rbx)
	vmovaps	%xmm11, %xmm0
.Ltmp3852:
	.loc	26 66 9
	vaddss	884(%rbx), %xmm11, %xmm9
.Ltmp3853:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm5, %xmm8, %xmm5
.Ltmp3854:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm9, %xmm5
.Ltmp3855:
	.loc	26 98 24
	vmovss	900(%rbx), %xmm8
	vbroadcastss	.LCPI21_16(%rip), %xmm9
	vxorps	%xmm9, %xmm8, %xmm8
.Ltmp3856:
	.loc	26 161 24
	vmaxss	%xmm8, %xmm5, %xmm5
.Ltmp3857:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm10, %xmm2, %xmm8
	vcmpltps	%xmm2, %xmm5, %xmm9
	vandps	%xmm9, %xmm8, %xmm8
	vmovd	%xmm8, %edi
	testb	$1, %dil
	jne	.LBB21_616
.Ltmp3858:
	.loc	26 0 44
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_616
.LBB21_648:
	cmpq	%r12, %r14
	jbe	.LBB21_680
.Ltmp3859:
	.loc	25 451 16 is_stmt 1
	negq	%r12
	movl	%ebp, %ecx
	subl	%edi, %ecx
	movq	%rsi, %r8
	subq	%r10, %r8
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_4(%rip), %xmm10
	movl	$841731191, %r9d
	movl	$8388608, %r10d
	vmovss	.LCPI21_1(%rip), %xmm11
	xorl	%r11d, %r11d
	movq	72(%rsp), %r13
	vmovss	.LCPI21_25(%rip), %xmm13
	vmovss	.LCPI21_26(%rip), %xmm14
	jmp	.LBB21_651
.Ltmp3860:
	.loc	25 0 16 is_stmt 0
.Ltmp3861:
	.p2align	4
.LBB21_650:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm8
.Ltmp3862:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm8, %xmm5
	setbe	%dil
.Ltmp3863:
	.loc	26 66 9
	vsubss	%xmm8, %xmm5, %xmm5
.Ltmp3864:
	.loc	26 92 9
	vmulss	768(%rbx,%rdi,4), %xmm5, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
.Ltmp3865:
	.loc	26 103 24
	vandps	%xmm5, %xmm12, %xmm8
.Ltmp3866:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm8, %xmm8
	vandps	%xmm5, %xmm8, %xmm5
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp3867:
	.loc	26 71 9
	vmulss	%xmm7, %xmm9, %xmm8
	vmovss	.LCPI21_22(%rip), %xmm10
.Ltmp3868:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp3869:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_23(%rip), %xmm11
.Ltmp3870:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp3871:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_24(%rip), %xmm12
.Ltmp3872:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp3873:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3874:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp3875:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
	vmovss	.LCPI21_0(%rip), %xmm1
.Ltmp3876:
	.loc	26 61 9
	vaddss	%xmm1, %xmm7, %xmm7
.Ltmp3877:
	.loc	26 178 22
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp3878:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp3879:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3880:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp3881:
	.loc	26 71 9
	vmulss	%xmm6, %xmm7, %xmm6
.Ltmp3882:
	.loc	26 161 24
	vcmpnltss	756(%rbx), %xmm2, %xmm7
.Ltmp3883:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp3884:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm6, %xmm4, %xmm3
.Ltmp3885:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm5, %xmm6
.Ltmp3886:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
.Ltmp3887:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm6, %xmm6
.Ltmp3888:
	.loc	26 161 24
	vblendvps	%xmm7, %xmm3, %xmm4, %xmm3
.Ltmp3889:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm6, %xmm6, %xmm7
.Ltmp3890:
	.loc	26 66 9
	vsubss	%xmm7, %xmm6, %xmm6
.Ltmp3891:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm8
.Ltmp3892:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp3893:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3894:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp3895:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3896:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp3897:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp3898:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp3899:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm6
.Ltmp3900:
	.loc	26 61 9
	vaddss	%xmm1, %xmm6, %xmm6
.Ltmp3901:
	.loc	26 178 22
	vaddss	%xmm7, %xmm14, %xmm7
.Ltmp3902:
	.loc	7 1244 18
	vmovd	%xmm7, %edi
.Ltmp3903:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3904:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3905:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp3906:
	.loc	21 510 5
	vmovss	%xmm5, 940(%rbx)
.Ltmp3907:
	.loc	26 71 9
	vmulss	%xmm6, %xmm15, %xmm6
.Ltmp3908:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm5, %xmm5
	vblendvps	%xmm5, %xmm6, %xmm15, %xmm5
	vcmpnltss	780(%rbx), %xmm2, %xmm2
	vblendvps	%xmm2, %xmm5, %xmm15, %xmm2
	movq	40(%rsp), %rdx
.Ltmp3909:
	.loc	26 56 9
	vmovss	%xmm3, -4(%rdx,%r14,4)
	movq	32(%rsp), %rdx
.Ltmp3910:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm2, -4(%rdx,%r14,4)
.Ltmp3911:
	.loc	8 1916 50 is_stmt 1
	leaq	(%r8,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	movq	64(%rsp), %rdx
	movq	48(%rsp), %rbp
	vmovss	.LCPI21_4(%rip), %xmm10
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovaps	%xmm0, %xmm11
.Ltmp3912:
	.loc	11 900 12
	je	.LBB21_743
.Ltmp3913:
.LBB21_651:
	.loc	21 361 22
	leal	(%r14,%rbp), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp3914:
	.loc	25 451 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_773
.Ltmp3915:
	.loc	21 0 0 is_stmt 0
	leaq	(%r12,%r14), %r15
	movq	40(%rsp), %rsi
.Ltmp3916:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rsi,%r14,4), %xmm2
.Ltmp3917:
	.loc	26 56 9
	vmovss	%xmm2, (%r13,%rdi,4)
	movq	32(%rsp), %rsi
.Ltmp3918:
	.loc	26 51 9
	vmovss	-4(%rsi,%r14,4), %xmm2
	movq	56(%rsp), %rsi
.Ltmp3919:
	.loc	26 56 9
	vmovss	%xmm2, (%rsi,%rdi,4)
	movq	80(%rsp), %rsi
.Ltmp3920:
	.loc	26 51 9
	vmovss	-4(%rsi,%r14,4), %xmm2
	movq	16(%rsp), %rsi
.Ltmp3921:
	.loc	26 56 9
	vmovss	%xmm2, (%rsi,%rdi,4)
.Ltmp3922:
	.loc	25 438 16
	cmpq	$1, %r15
	je	.LBB21_778
.Ltmp3923:
	.loc	25 0 16 is_stmt 0
	movq	96(%rsp), %rsi
.Ltmp3924:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB21_777
.Ltmp3925:
	.loc	25 0 16 is_stmt 0
	movq	112(%rsp), %r15
.Ltmp3926:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r15,%r14,4), %xmm2
	movq	24(%rsp), %r15
.Ltmp3927:
	.loc	26 56 9
	vmovss	%xmm2, (%r15,%rdi,4)
.Ltmp3928:
	.loc	21 370 21
	leal	(%rcx,%r14), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp3929:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp3930:
	.loc	26 51 9
	vmovss	(%r13,%rdi,4), %xmm4
	movq	56(%rsp), %rdx
.Ltmp3931:
	.loc	26 51 9 is_stmt 0
	vmovss	(%rdx,%rdi,4), %xmm15
.Ltmp3932:
	.loc	21 0 0
	leal	(%r14,%rbp), %r15d
	movl	136(%rbx), %edi
	movl	200(%rbx), %ebp
	notl	%edi
	addl	%r15d, %edi
	andl	%eax, %edi
	notl	%ebp
	addl	%r15d, %ebp
	andl	%eax, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 128(%rsp)
	je	.LBB21_660
	cmpl	$1, 120(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
	jne	.LBB21_663
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_809
.Ltmp3933:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, %rsi
	jbe	.LBB21_811
.Ltmp3934:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm5
	movq	24(%rsp), %rdx
.Ltmp3935:
	.loc	21 252 33
	vmovss	(%rdx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm5, %xmm3
	jmp	.LBB21_668
.Ltmp3936:
.LBB21_660:
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
	vbroadcastss	.LCPI21_2(%rip), %xmm12
.Ltmp3937:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_805
.Ltmp3938:
	.loc	21 237 33
	cmpq	%rbp, %rsi
	jbe	.LBB21_806
.Ltmp3939:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm2
	movq	24(%rsp), %rdx
.Ltmp3940:
	.loc	21 237 33
	vmovss	(%rdx,%rbp,4), %xmm5
	vmovaps	%xmm5, %xmm6
	vmovaps	%xmm2, %xmm3
.Ltmp3941:
	.loc	26 51 9
	jmp	.LBB21_668
.Ltmp3942:
.LBB21_663:
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
.Ltmp3943:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_810
	.loc	21 267 33
	cmpq	%rdi, %rsi
	jbe	.LBB21_807
	.loc	21 268 33
	cmpq	%rbp, %rsi
	jbe	.LBB21_808
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm3
	movq	24(%rsp), %rsi
	vmovss	(%rsi,%rdi,4), %xmm2
	.loc	21 268 33 is_stmt 1
	vmovss	(%rsi,%rbp,4), %xmm6
	.loc	21 269 33
	vmovss	(%rdx,%rbp,4), %xmm5
.Ltmp3944:
.LBB21_668:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm3
.Ltmp3945:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm12, %xmm7
.Ltmp3946:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp3947:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3948:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %ebp
.Ltmp3949:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
.Ltmp3950:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm8
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp3951:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3952:
	.loc	26 161 24
	cmovbel	%edi, %ebp
.Ltmp3953:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm8
.Ltmp3954:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3955:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp3956:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm7, %xmm7
.Ltmp3957:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp3958:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp3959:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3960:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp3961:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm3
.Ltmp3962:
	.loc	26 161 24
	cmovbel	%r9d, %edi
.Ltmp3963:
	.loc	21 451 21
	vmovss	792(%rbx), %xmm3
.Ltmp3964:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp3965:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm7
.Ltmp3966:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp3967:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp3968:
	.loc	7 1291 18
	vmovd	%ebp, %xmm7
.Ltmp3969:
	.loc	26 66 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp3970:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm7, %xmm8
.Ltmp3971:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm9
	vsubss	%xmm8, %xmm9, %xmm8
.Ltmp3972:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3973:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm8, %xmm8
.Ltmp3974:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3975:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm8, %xmm8
.Ltmp3976:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3977:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm8, %xmm8
.Ltmp3978:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp3979:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm8, %xmm8
.Ltmp3980:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3981:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3982:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm9, %xmm9
.Ltmp3983:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
.Ltmp3984:
	.loc	26 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp3985:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm7, %xmm7
.Ltmp3986:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm7, %xmm7
.Ltmp3987:
	.loc	26 66 9
	vsubss	840(%rbx), %xmm3, %xmm8
.Ltmp3988:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm7, %xmm7
.Ltmp3989:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm7
.Ltmp3990:
	.loc	26 28 5
	movl	$0, %r15d
	adcl	$-1, %r15d
.Ltmp3991:
	.loc	26 129 14
	vucomiss	%xmm3, %xmm7
.Ltmp3992:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3993:
	.loc	26 149 9
	movl	%r15d, %edi
.Ltmp3994:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm8
.Ltmp3995:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp3996:
	.loc	26 149 9
	notl	%edi
.Ltmp3997:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp3998:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm9
.Ltmp3999:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp4000:
	.loc	26 144 9
	cmoval	%r15d, %ebp
.Ltmp4001:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp4002:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_670
.Ltmp4003:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm8, %xmm8
.LBB21_670:
.Ltmp4004:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_672
.Ltmp4005:
	.loc	21 0 0 is_stmt 0
	vmovss	752(%rbx), %xmm8
.LBB21_672:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm9
.Ltmp4006:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm8, 860(%rbx)
	.loc	21 498 5
	movl	%edi, 856(%rbx)
.Ltmp4007:
	.loc	26 66 9
	vaddss	808(%rbx), %xmm11, %xmm8
.Ltmp4008:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp4009:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm8, %xmm3
.Ltmp4010:
	.loc	26 98 24
	vmovss	824(%rbx), %xmm7
	vbroadcastss	.LCPI21_16(%rip), %xmm8
	vxorps	%xmm7, %xmm8, %xmm7
.Ltmp4011:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm3, %xmm3
.Ltmp4012:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm9, %xmm2, %xmm7
	vcmpltps	%xmm2, %xmm3, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovd	%xmm7, %edi
	testb	$1, %dil
	jne	.LBB21_674
.Ltmp4013:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_674:
.Ltmp4014:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm7
.Ltmp4015:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm7, %xmm3
	setbe	%dil
.Ltmp4016:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp4017:
	.loc	26 92 9
	vmulss	744(%rbx,%rdi,4), %xmm3, %xmm3
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp4018:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm7
.Ltmp4019:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm7, %xmm7
	vandps	%xmm3, %xmm7, %xmm3
.Ltmp4020:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm6
.Ltmp4021:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm5, %xmm12, %xmm5
.Ltmp4022:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm6
.Ltmp4023:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp4024:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm5, %ebp
.Ltmp4025:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
.Ltmp4026:
	.loc	21 510 5
	vmovss	%xmm3, 864(%rbx)
.Ltmp4027:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm7
.Ltmp4028:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp4029:
	.loc	26 161 24
	cmovbel	%edi, %ebp
.Ltmp4030:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm7
.Ltmp4031:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp4032:
	.loc	26 71 9
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp4033:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm5, %xmm5
.Ltmp4034:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm5, %xmm5
.Ltmp4035:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp4036:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp4037:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp4038:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm5
.Ltmp4039:
	.loc	26 161 24
	cmovbel	%r9d, %edi
.Ltmp4040:
	.loc	21 451 21
	vmovss	868(%rbx), %xmm5
.Ltmp4041:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp4042:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm6
.Ltmp4043:
	.loc	26 161 24
	cmovbel	%r10d, %edi
.Ltmp4044:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp4045:
	.loc	7 1291 18
	vmovd	%ebp, %xmm6
.Ltmp4046:
	.loc	26 66 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp4047:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm6, %xmm7
.Ltmp4048:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm8
	vsubss	%xmm7, %xmm8, %xmm7
.Ltmp4049:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4050:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm7, %xmm7
.Ltmp4051:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4052:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm7, %xmm7
.Ltmp4053:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4054:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm7, %xmm7
.Ltmp4055:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4056:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm7, %xmm7
.Ltmp4057:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp4058:
	.loc	7 1291 18
	vmovd	%edi, %xmm8
.Ltmp4059:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm8, %xmm8
.Ltmp4060:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp4061:
	.loc	26 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp4062:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm6, %xmm6
.Ltmp4063:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm6, %xmm6
.Ltmp4064:
	.loc	26 66 9
	vsubss	916(%rbx), %xmm5, %xmm7
.Ltmp4065:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm6, %xmm8
.Ltmp4066:
	.loc	26 129 14
	vucomiss	%xmm7, %xmm8
.Ltmp4067:
	.loc	26 28 5
	movl	$0, %r15d
	adcl	$-1, %r15d
.Ltmp4068:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm8
.Ltmp4069:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp4070:
	.loc	26 149 9
	movl	%r15d, %edi
.Ltmp4071:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm9
.Ltmp4072:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp4073:
	.loc	26 149 9
	notl	%edi
.Ltmp4074:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp4075:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm6
.Ltmp4076:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm6
.Ltmp4077:
	.loc	26 144 9
	cmoval	%r15d, %ebp
.Ltmp4078:
	.loc	26 139 9
	cmovbel	%r11d, %edi
.Ltmp4079:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_676
.Ltmp4080:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm9, %xmm9
.LBB21_676:
.Ltmp4081:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	je	.LBB21_678
.Ltmp4082:
	.loc	21 0 0 is_stmt 0
	vmovss	776(%rbx), %xmm9
.Ltmp4083:
.LBB21_678:
	vmulss	.LCPI21_18(%rip), %xmm3, %xmm6
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
	vminss	.LCPI21_20(%rip), %xmm6, %xmm7
	vroundss	$9, %xmm7, %xmm7, %xmm6
	vsubss	%xmm6, %xmm7, %xmm7
.Ltmp4084:
	orl	%ebp, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp4085:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm9, 936(%rbx)
	.loc	21 498 5
	movl	%edi, 932(%rbx)
	vmovaps	%xmm11, %xmm0
.Ltmp4086:
	.loc	26 66 9
	vaddss	884(%rbx), %xmm11, %xmm9
.Ltmp4087:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm5, %xmm8, %xmm5
.Ltmp4088:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm9, %xmm5
.Ltmp4089:
	.loc	26 98 24
	vmovss	900(%rbx), %xmm8
	vbroadcastss	.LCPI21_16(%rip), %xmm9
	vxorps	%xmm9, %xmm8, %xmm8
.Ltmp4090:
	.loc	26 161 24
	vmaxss	%xmm8, %xmm5, %xmm5
.Ltmp4091:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm10, %xmm2, %xmm8
	vcmpltps	%xmm2, %xmm5, %xmm9
	vandps	%xmm9, %xmm8, %xmm8
	vmovd	%xmm8, %edi
	testb	$1, %dil
	jne	.LBB21_650
.Ltmp4092:
	.loc	26 0 44
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_650
.LBB21_680:
	movq	%r14, %r13
	cmpq	96(%rsp), %rdx
	jbe	.LBB21_713
.Ltmp4093:
	.loc	25 438 16 is_stmt 1
	movl	%ebp, %r15d
	subl	%edi, %r15d
	subq	%r10, %rsi
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_4(%rip), %xmm10
	movl	$841731191, %r8d
	movl	$8388608, %r9d
	vmovss	.LCPI21_1(%rip), %xmm11
	xorl	%r10d, %r10d
	movq	56(%rsp), %r12
	vmovss	.LCPI21_25(%rip), %xmm13
	vmovss	.LCPI21_26(%rip), %xmm14
	jmp	.LBB21_683
.Ltmp4094:
	.loc	25 0 16 is_stmt 0
.Ltmp4095:
	.p2align	4
.LBB21_682:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm8
.Ltmp4096:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm8, %xmm5
	setbe	%dil
.Ltmp4097:
	.loc	26 66 9
	vsubss	%xmm8, %xmm5, %xmm5
.Ltmp4098:
	.loc	26 92 9
	vmulss	768(%rbx,%rdi,4), %xmm5, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
.Ltmp4099:
	.loc	26 103 24
	vandps	%xmm5, %xmm12, %xmm8
.Ltmp4100:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm8, %xmm8
	vandps	%xmm5, %xmm8, %xmm5
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp4101:
	.loc	26 71 9
	vmulss	%xmm7, %xmm9, %xmm8
	vmovss	.LCPI21_22(%rip), %xmm10
.Ltmp4102:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp4103:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_23(%rip), %xmm11
.Ltmp4104:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp4105:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_24(%rip), %xmm12
.Ltmp4106:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp4107:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4108:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp4109:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
	vmovss	.LCPI21_0(%rip), %xmm1
.Ltmp4110:
	.loc	26 61 9
	vaddss	%xmm1, %xmm7, %xmm7
.Ltmp4111:
	.loc	26 178 22
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp4112:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp4113:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp4114:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp4115:
	.loc	26 71 9
	vmulss	%xmm6, %xmm7, %xmm6
.Ltmp4116:
	.loc	26 161 24
	vcmpnltss	756(%rbx), %xmm2, %xmm7
.Ltmp4117:
	.loc	26 71 9
	vmulss	%xmm6, %xmm4, %xmm6
.Ltmp4118:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm6, %xmm4, %xmm3
.Ltmp4119:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm5, %xmm6
.Ltmp4120:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
.Ltmp4121:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm6, %xmm6
.Ltmp4122:
	.loc	26 161 24
	vblendvps	%xmm7, %xmm3, %xmm4, %xmm3
.Ltmp4123:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm6, %xmm6, %xmm7
.Ltmp4124:
	.loc	26 66 9
	vsubss	%xmm7, %xmm6, %xmm6
.Ltmp4125:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm8
.Ltmp4126:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp4127:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp4128:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp4129:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp4130:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp4131:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp4132:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp4133:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm6
.Ltmp4134:
	.loc	26 61 9
	vaddss	%xmm1, %xmm6, %xmm6
.Ltmp4135:
	.loc	26 178 22
	vaddss	%xmm7, %xmm14, %xmm7
.Ltmp4136:
	.loc	7 1244 18
	vmovd	%xmm7, %edi
.Ltmp4137:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp4138:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp4139:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp4140:
	.loc	21 510 5
	vmovss	%xmm5, 940(%rbx)
.Ltmp4141:
	.loc	26 71 9
	vmulss	%xmm6, %xmm15, %xmm6
.Ltmp4142:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm5, %xmm5
	vblendvps	%xmm5, %xmm6, %xmm15, %xmm5
	vcmpnltss	780(%rbx), %xmm2, %xmm2
	vblendvps	%xmm2, %xmm5, %xmm15, %xmm2
	movq	40(%rsp), %rcx
.Ltmp4143:
	.loc	26 56 9
	vmovss	%xmm3, -4(%rcx,%r14,4)
	movq	32(%rsp), %rcx
.Ltmp4144:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm2, -4(%rcx,%r14,4)
.Ltmp4145:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rsi,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	vmovss	.LCPI21_4(%rip), %xmm10
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovaps	%xmm0, %xmm11
.Ltmp4146:
	.loc	11 900 12
	je	.LBB21_743
.LBB21_683:
	.loc	11 0 12 is_stmt 0
	movq	144(%rsp), %rcx
.Ltmp4147:
	.loc	15 971 17 is_stmt 1
	leaq	(%rcx,%r14), %rdi
.Ltmp4148:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_790
.Ltmp4149:
	.loc	21 0 0 is_stmt 0
	leal	(%r14,%rbp), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp4150:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	movq	72(%rsp), %r11
	vbroadcastss	.LCPI21_2(%rip), %xmm12
	jbe	.LBB21_773
.Ltmp4151:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %rcx
.Ltmp4152:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r14,4), %xmm2
.Ltmp4153:
	.loc	26 56 9
	vmovss	%xmm2, (%r11,%rdi,4)
	movq	32(%rsp), %rcx
.Ltmp4154:
	.loc	26 51 9
	vmovss	-4(%rcx,%r14,4), %xmm2
.Ltmp4155:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
	movq	80(%rsp), %rcx
.Ltmp4156:
	.loc	26 51 9
	vmovss	-4(%rcx,%r14,4), %xmm2
	movq	16(%rsp), %rcx
.Ltmp4157:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%rdi,4)
.Ltmp4158:
	.loc	25 451 16
	cmpq	%rdi, 96(%rsp)
	jbe	.LBB21_777
.Ltmp4159:
	.loc	25 0 16 is_stmt 0
	movq	112(%rsp), %rcx
.Ltmp4160:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r14,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp4161:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%rdi,4)
.Ltmp4162:
	.loc	21 370 21
	leal	(%r15,%r14), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp4163:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp4164:
	.loc	26 51 9
	vmovss	(%r11,%rdi,4), %xmm4
.Ltmp4165:
	.loc	26 51 9 is_stmt 0
	vmovss	(%r12,%rdi,4), %xmm15
.Ltmp4166:
	.loc	21 0 0
	leal	(%r14,%rbp), %r11d
	movl	136(%rbx), %edi
	movl	200(%rbx), %ebp
	notl	%edi
	addl	%r11d, %edi
	andl	%eax, %edi
	notl	%ebp
	addl	%r11d, %ebp
	andl	%eax, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 128(%rsp)
	je	.LBB21_692
	cmpl	$1, 120(%rsp)
	jne	.LBB21_695
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
	jbe	.LBB21_809
.Ltmp4167:
	.loc	21 252 33 is_stmt 1
	cmpq	%rbp, 96(%rsp)
	jbe	.LBB21_811
.Ltmp4168:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm5
	movq	24(%rsp), %rcx
.Ltmp4169:
	.loc	21 252 33
	vmovss	(%rcx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm5, %xmm3
	jmp	.LBB21_699
.Ltmp4170:
.LBB21_692:
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
.Ltmp4171:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_805
.Ltmp4172:
	.loc	21 237 33
	cmpq	%rbp, 96(%rsp)
	jbe	.LBB21_806
.Ltmp4173:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp4174:
	.loc	21 237 33
	vmovss	(%rcx,%rbp,4), %xmm5
	vmovaps	%xmm5, %xmm6
	vmovaps	%xmm2, %xmm3
.Ltmp4175:
	.loc	26 51 9
	jmp	.LBB21_699
.Ltmp4176:
.LBB21_695:
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
.Ltmp4177:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_810
	.loc	21 0 33 is_stmt 0
	movq	96(%rsp), %rcx
	.loc	21 267 33 is_stmt 1
	cmpq	%rdi, %rcx
	jbe	.LBB21_807
	.loc	21 268 33
	cmpq	%rbp, %rcx
	jbe	.LBB21_808
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	vmovss	(%rcx,%rdi,4), %xmm3
	movq	24(%rsp), %rdx
	.loc	21 267 33 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm2
	.loc	21 268 33
	vmovss	(%rdx,%rbp,4), %xmm6
	.loc	21 269 33
	vmovss	(%rcx,%rbp,4), %xmm5
.Ltmp4178:
.LBB21_699:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm3
.Ltmp4179:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm12, %xmm7
.Ltmp4180:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp4181:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp4182:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %r11d
.Ltmp4183:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r11d
.Ltmp4184:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm8
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp4185:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp4186:
	.loc	26 161 24
	cmovbel	%edi, %r11d
.Ltmp4187:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm8
.Ltmp4188:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp4189:
	.loc	26 71 9
	vmulss	%xmm1, %xmm3, %xmm3
.Ltmp4190:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm7, %xmm7
.Ltmp4191:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp4192:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp4193:
	.loc	26 161 24
	cmovbel	%r11d, %edi
.Ltmp4194:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp4195:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm3
.Ltmp4196:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp4197:
	.loc	21 451 21
	vmovss	792(%rbx), %xmm3
.Ltmp4198:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp4199:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm7
.Ltmp4200:
	.loc	26 161 24
	cmovbel	%r9d, %edi
.Ltmp4201:
	.loc	26 185 42
	movl	%edi, %r11d
	andl	$8388607, %r11d
	orl	$1065353216, %r11d
.Ltmp4202:
	.loc	7 1291 18
	vmovd	%r11d, %xmm7
.Ltmp4203:
	.loc	26 66 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp4204:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm7, %xmm8
.Ltmp4205:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm9
	vsubss	%xmm8, %xmm9, %xmm8
.Ltmp4206:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4207:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm8, %xmm8
.Ltmp4208:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4209:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm8, %xmm8
.Ltmp4210:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4211:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm8, %xmm8
.Ltmp4212:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4213:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm8, %xmm8
.Ltmp4214:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp4215:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp4216:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm9, %xmm9
.Ltmp4217:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
.Ltmp4218:
	.loc	26 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp4219:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm7, %xmm7
.Ltmp4220:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm7, %xmm7
.Ltmp4221:
	.loc	26 66 9
	vsubss	840(%rbx), %xmm3, %xmm8
.Ltmp4222:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm7, %xmm7
.Ltmp4223:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm7
.Ltmp4224:
	.loc	26 28 5
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp4225:
	.loc	26 129 14
	vucomiss	%xmm3, %xmm7
.Ltmp4226:
	.loc	26 144 9
	movl	$0, %r11d
	adcl	$-1, %r11d
.Ltmp4227:
	.loc	26 149 9
	movl	%ebp, %edi
.Ltmp4228:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm8
.Ltmp4229:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp4230:
	.loc	26 149 9
	notl	%edi
.Ltmp4231:
	.loc	26 139 9
	cmovbel	%r10d, %edi
.Ltmp4232:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm9
.Ltmp4233:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp4234:
	.loc	26 144 9
	cmoval	%ebp, %r11d
.Ltmp4235:
	.loc	26 139 9
	cmovbel	%r10d, %edi
.Ltmp4236:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_701
.Ltmp4237:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm8, %xmm8
.LBB21_701:
.Ltmp4238:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r11b
	je	.LBB21_703
.Ltmp4239:
	.loc	21 0 0 is_stmt 0
	vmovss	752(%rbx), %xmm8
.LBB21_703:
	orl	%r11d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm9
.Ltmp4240:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm8, 860(%rbx)
	.loc	21 498 5
	movl	%edi, 856(%rbx)
.Ltmp4241:
	.loc	26 66 9
	vaddss	808(%rbx), %xmm11, %xmm8
.Ltmp4242:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp4243:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm8, %xmm3
.Ltmp4244:
	.loc	26 98 24
	vmovss	824(%rbx), %xmm7
	vbroadcastss	.LCPI21_16(%rip), %xmm8
	vxorps	%xmm7, %xmm8, %xmm7
.Ltmp4245:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm3, %xmm3
.Ltmp4246:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm9, %xmm2, %xmm7
	vcmpltps	%xmm2, %xmm3, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovd	%xmm7, %edi
	testb	$1, %dil
	jne	.LBB21_705
.Ltmp4247:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_705:
.Ltmp4248:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm7
.Ltmp4249:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm7, %xmm3
	setbe	%dil
.Ltmp4250:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp4251:
	.loc	26 92 9
	vmulss	744(%rbx,%rdi,4), %xmm3, %xmm3
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp4252:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm7
.Ltmp4253:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm7, %xmm7
	vandps	%xmm3, %xmm7, %xmm3
.Ltmp4254:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm6
.Ltmp4255:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm5, %xmm12, %xmm5
.Ltmp4256:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm6
.Ltmp4257:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp4258:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm5, %r11d
.Ltmp4259:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r11d
.Ltmp4260:
	.loc	21 510 5
	vmovss	%xmm3, 864(%rbx)
.Ltmp4261:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm7
.Ltmp4262:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp4263:
	.loc	26 161 24
	cmovbel	%edi, %r11d
.Ltmp4264:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm7
.Ltmp4265:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp4266:
	.loc	26 71 9
	vmulss	%xmm1, %xmm6, %xmm6
.Ltmp4267:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm5, %xmm5
.Ltmp4268:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm5, %xmm5
.Ltmp4269:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp4270:
	.loc	26 161 24
	cmovbel	%r11d, %edi
.Ltmp4271:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp4272:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm5
.Ltmp4273:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp4274:
	.loc	21 451 21
	vmovss	868(%rbx), %xmm5
.Ltmp4275:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp4276:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm6
.Ltmp4277:
	.loc	26 161 24
	cmovbel	%r9d, %edi
.Ltmp4278:
	.loc	26 185 42
	movl	%edi, %r11d
	andl	$8388607, %r11d
	orl	$1065353216, %r11d
.Ltmp4279:
	.loc	7 1291 18
	vmovd	%r11d, %xmm6
.Ltmp4280:
	.loc	26 66 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp4281:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm6, %xmm7
.Ltmp4282:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm8
	vsubss	%xmm7, %xmm8, %xmm7
.Ltmp4283:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4284:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm7, %xmm7
.Ltmp4285:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4286:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm7, %xmm7
.Ltmp4287:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4288:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm7, %xmm7
.Ltmp4289:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4290:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm7, %xmm7
.Ltmp4291:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp4292:
	.loc	7 1291 18
	vmovd	%edi, %xmm8
.Ltmp4293:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm8, %xmm8
.Ltmp4294:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp4295:
	.loc	26 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp4296:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm6, %xmm6
.Ltmp4297:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm6, %xmm6
.Ltmp4298:
	.loc	26 66 9
	vsubss	916(%rbx), %xmm5, %xmm7
.Ltmp4299:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm6, %xmm8
.Ltmp4300:
	.loc	26 129 14
	vucomiss	%xmm7, %xmm8
.Ltmp4301:
	.loc	26 28 5
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp4302:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm8
.Ltmp4303:
	.loc	26 144 9
	movl	$0, %r11d
	adcl	$-1, %r11d
.Ltmp4304:
	.loc	26 149 9
	movl	%ebp, %edi
.Ltmp4305:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm9
.Ltmp4306:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp4307:
	.loc	26 149 9
	notl	%edi
.Ltmp4308:
	.loc	26 139 9
	cmovbel	%r10d, %edi
.Ltmp4309:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm6
.Ltmp4310:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm6
.Ltmp4311:
	.loc	26 144 9
	cmoval	%ebp, %r11d
.Ltmp4312:
	.loc	26 139 9
	cmovbel	%r10d, %edi
.Ltmp4313:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_707
.Ltmp4314:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm9, %xmm9
.LBB21_707:
	movq	64(%rsp), %rdx
	movq	48(%rsp), %rbp
.Ltmp4315:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r11b
	je	.LBB21_709
.Ltmp4316:
	.loc	21 0 0 is_stmt 0
	vmovss	776(%rbx), %xmm9
.Ltmp4317:
.LBB21_709:
	vmulss	.LCPI21_18(%rip), %xmm3, %xmm6
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
	vminss	.LCPI21_20(%rip), %xmm6, %xmm7
	vroundss	$9, %xmm7, %xmm7, %xmm6
	vsubss	%xmm6, %xmm7, %xmm7
.Ltmp4318:
	orl	%r11d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp4319:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm9, 936(%rbx)
	.loc	21 498 5
	movl	%edi, 932(%rbx)
	vmovaps	%xmm11, %xmm0
.Ltmp4320:
	.loc	26 66 9
	vaddss	884(%rbx), %xmm11, %xmm9
.Ltmp4321:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm5, %xmm8, %xmm5
.Ltmp4322:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm9, %xmm5
.Ltmp4323:
	.loc	26 98 24
	vmovss	900(%rbx), %xmm8
	vbroadcastss	.LCPI21_16(%rip), %xmm9
	vxorps	%xmm9, %xmm8, %xmm8
.Ltmp4324:
	.loc	26 161 24
	vmaxss	%xmm8, %xmm5, %xmm5
.Ltmp4325:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm10, %xmm2, %xmm8
	vcmpltps	%xmm2, %xmm5, %xmm9
	vandps	%xmm9, %xmm8, %xmm8
	vmovd	%xmm8, %edi
	testb	$1, %dil
	jne	.LBB21_682
.Ltmp4326:
	.loc	26 0 44
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_682
.LBB21_711:
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_9b3ed103a299f61e5929b9f3196be990(%rip), %rcx
	movq	8(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB21_712:
	leaq	1(%rdi), %rsi
.Ltmp4327:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_a44e1bcb659c545f1ffe97492fde2ec9(%rip), %rcx
	movq	16(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4328:
.LBB21_713:
	.loc	25 438 16
	movl	%ebp, %r15d
	subl	%edi, %r15d
	subq	%r10, %rsi
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm4
	vmovss	.LCPI21_4(%rip), %xmm10
	movl	$841731191, %r8d
	movl	$8388608, %r9d
	vmovss	.LCPI21_1(%rip), %xmm11
	xorl	%r10d, %r10d
	movq	56(%rsp), %r12
	vmovss	.LCPI21_25(%rip), %xmm13
	vmovss	.LCPI21_26(%rip), %xmm14
	jmp	.LBB21_715
.Ltmp4329:
	.loc	25 0 16 is_stmt 0
.Ltmp4330:
	.p2align	4
.LBB21_714:
	.loc	21 508 36 is_stmt 1
	vmovss	940(%rbx), %xmm8
.Ltmp4331:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm8, %xmm5
	setbe	%dil
.Ltmp4332:
	.loc	26 66 9
	vsubss	%xmm8, %xmm5, %xmm5
.Ltmp4333:
	.loc	26 92 9
	vmulss	768(%rbx,%rdi,4), %xmm5, %xmm5
	vaddss	%xmm5, %xmm8, %xmm5
.Ltmp4334:
	.loc	26 103 24
	vandps	%xmm5, %xmm12, %xmm8
.Ltmp4335:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm8, %xmm8
	vandps	%xmm5, %xmm8, %xmm5
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp4336:
	.loc	26 71 9
	vmulss	%xmm7, %xmm9, %xmm8
	vmovss	.LCPI21_22(%rip), %xmm10
.Ltmp4337:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp4338:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_23(%rip), %xmm11
.Ltmp4339:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp4340:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
	vmovss	.LCPI21_24(%rip), %xmm12
.Ltmp4341:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp4342:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4343:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp4344:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
	vmovss	.LCPI21_0(%rip), %xmm1
.Ltmp4345:
	.loc	26 61 9
	vaddss	%xmm1, %xmm7, %xmm7
.Ltmp4346:
	.loc	26 178 22
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp4347:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp4348:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp4349:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp4350:
	.loc	26 71 9
	vmulss	%xmm6, %xmm7, %xmm6
.Ltmp4351:
	.loc	26 161 24
	vcmpnltss	756(%rbx), %xmm2, %xmm7
.Ltmp4352:
	.loc	26 71 9
	vmulss	%xmm6, %xmm15, %xmm6
.Ltmp4353:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm3, %xmm3
	vblendvps	%xmm3, %xmm6, %xmm15, %xmm3
.Ltmp4354:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm5, %xmm6
.Ltmp4355:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
.Ltmp4356:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm6, %xmm6
.Ltmp4357:
	.loc	26 161 24
	vblendvps	%xmm7, %xmm3, %xmm15, %xmm3
.Ltmp4358:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm6, %xmm6, %xmm7
.Ltmp4359:
	.loc	26 66 9
	vsubss	%xmm7, %xmm6, %xmm6
.Ltmp4360:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm8
.Ltmp4361:
	.loc	26 61 9
	vaddss	%xmm10, %xmm8, %xmm8
.Ltmp4362:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp4363:
	.loc	26 61 9
	vaddss	%xmm11, %xmm8, %xmm8
.Ltmp4364:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp4365:
	.loc	26 61 9
	vaddss	%xmm12, %xmm8, %xmm8
.Ltmp4366:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm8
.Ltmp4367:
	.loc	26 61 9
	vaddss	%xmm13, %xmm8, %xmm8
.Ltmp4368:
	.loc	26 71 9
	vmulss	%xmm6, %xmm8, %xmm6
.Ltmp4369:
	.loc	26 61 9
	vaddss	%xmm1, %xmm6, %xmm6
.Ltmp4370:
	.loc	26 178 22
	vaddss	%xmm7, %xmm14, %xmm7
.Ltmp4371:
	.loc	7 1244 18
	vmovd	%xmm7, %edi
.Ltmp4372:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp4373:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp4374:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp4375:
	.loc	21 510 5
	vmovss	%xmm5, 940(%rbx)
	vmovaps	176(%rsp), %xmm1
.Ltmp4376:
	.loc	26 71 9
	vmulss	%xmm6, %xmm1, %xmm6
.Ltmp4377:
	.loc	26 161 24
	vcmpneqss	%xmm2, %xmm5, %xmm5
	vblendvps	%xmm5, %xmm6, %xmm1, %xmm5
	vcmpnltss	780(%rbx), %xmm2, %xmm2
	vblendvps	%xmm2, %xmm5, %xmm1, %xmm2
	movq	40(%rsp), %rcx
.Ltmp4378:
	.loc	26 56 9
	vmovss	%xmm3, -4(%rcx,%r14,4)
	movq	32(%rsp), %rcx
.Ltmp4379:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm2, -4(%rcx,%r14,4)
.Ltmp4380:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rsi,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	vmovss	.LCPI21_4(%rip), %xmm10
	vmovaps	%xmm0, %xmm11
.Ltmp4381:
	.loc	11 900 12
	je	.LBB21_743
.LBB21_715:
	.loc	11 0 12 is_stmt 0
	movq	144(%rsp), %rcx
.Ltmp4382:
	.loc	15 971 17 is_stmt 1
	leaq	(%rcx,%r14), %rdi
.Ltmp4383:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_790
.Ltmp4384:
	.loc	21 0 0 is_stmt 0
	leal	(%r14,%rbp), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp4385:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	movq	72(%rsp), %r11
	vbroadcastss	.LCPI21_2(%rip), %xmm12
	jbe	.LBB21_773
.Ltmp4386:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %rcx
.Ltmp4387:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r14,4), %xmm2
.Ltmp4388:
	.loc	26 56 9
	vmovss	%xmm2, (%r11,%rdi,4)
	movq	32(%rsp), %rcx
.Ltmp4389:
	.loc	26 51 9
	vmovss	-4(%rcx,%r14,4), %xmm2
.Ltmp4390:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%rdi,4)
	movq	80(%rsp), %rcx
.Ltmp4391:
	.loc	26 51 9
	vmovss	-4(%rcx,%r14,4), %xmm2
	movq	16(%rsp), %rcx
.Ltmp4392:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%rdi,4)
	movq	112(%rsp), %rcx
.Ltmp4393:
	.loc	26 51 9
	vmovss	-4(%rcx,%r14,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp4394:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%rdi,4)
.Ltmp4395:
	.loc	21 370 21
	leal	(%r15,%r14), %edi
	decl	%edi
	andl	%eax, %edi
.Ltmp4396:
	.loc	25 438 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_774
.Ltmp4397:
	.loc	26 51 9
	vmovss	(%r11,%rdi,4), %xmm15
.Ltmp4398:
	.loc	26 51 9 is_stmt 0
	vmovss	(%r12,%rdi,4), %xmm0
.Ltmp4399:
	.loc	21 0 0
	leal	(%r14,%rbp), %r11d
	movl	136(%rbx), %edi
	movl	200(%rbx), %ebp
	notl	%edi
	addl	%r11d, %edi
	andl	%eax, %edi
	notl	%ebp
	addl	%r11d, %ebp
	andl	%eax, %ebp
	.loc	21 229 5 is_stmt 1
	cmpb	$0, 128(%rsp)
	vmovaps	%xmm0, 176(%rsp)
	je	.LBB21_723
	cmpl	$1, 120(%rsp)
	jne	.LBB21_726
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
.Ltmp4400:
	.loc	21 251 32 is_stmt 1
	jbe	.LBB21_809
.Ltmp4401:
	.loc	21 252 33
	cmpq	%rbp, 96(%rsp)
	jbe	.LBB21_811
.Ltmp4402:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	.loc	21 251 32 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm5
	movq	24(%rsp), %rcx
.Ltmp4403:
	.loc	21 252 33
	vmovss	(%rcx,%rbp,4), %xmm6
	vmovaps	%xmm6, %xmm2
	vmovaps	%xmm5, %xmm3
	jmp	.LBB21_731
.Ltmp4404:
.LBB21_723:
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
.Ltmp4405:
	.loc	21 236 32 is_stmt 1
	jbe	.LBB21_805
.Ltmp4406:
	.loc	21 237 33
	cmpq	%rbp, 96(%rsp)
	jbe	.LBB21_806
.Ltmp4407:
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	.loc	21 236 32 is_stmt 1
	vmovss	(%rcx,%rdi,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp4408:
	.loc	21 237 33
	vmovss	(%rcx,%rbp,4), %xmm5
	vmovaps	%xmm5, %xmm6
	vmovaps	%xmm2, %xmm3
.Ltmp4409:
	.loc	26 51 9
	jmp	.LBB21_731
.Ltmp4410:
.LBB21_726:
	.loc	21 0 0 is_stmt 0
	cmpq	%rdi, 8(%rsp)
.Ltmp4411:
	.loc	21 266 33 is_stmt 1
	jbe	.LBB21_810
	.loc	21 0 33 is_stmt 0
	movq	96(%rsp), %rcx
	.loc	21 267 33 is_stmt 1
	cmpq	%rdi, %rcx
	jbe	.LBB21_807
	.loc	21 268 33
	cmpq	%rbp, %rcx
	jbe	.LBB21_808
	.loc	21 269 33
	cmpq	%rbp, 8(%rsp)
	jbe	.LBB21_797
	.loc	21 0 33 is_stmt 0
	movq	16(%rsp), %rcx
	vmovss	(%rcx,%rdi,4), %xmm3
	movq	24(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm2
	.loc	21 268 33 is_stmt 1
	vmovss	(%rdx,%rbp,4), %xmm6
	.loc	21 269 33
	vmovss	(%rcx,%rbp,4), %xmm5
.Ltmp4412:
.LBB21_731:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm3
.Ltmp4413:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm2, %xmm12, %xmm7
.Ltmp4414:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm3
.Ltmp4415:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp4416:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm7, %r11d
.Ltmp4417:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r11d
.Ltmp4418:
	.loc	21 459 23
	vmovss	760(%rbx), %xmm8
	vxorps	%xmm2, %xmm2, %xmm2
.Ltmp4419:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp4420:
	.loc	26 161 24
	cmovbel	%edi, %r11d
.Ltmp4421:
	.loc	21 461 9
	vmovss	764(%rbx), %xmm8
.Ltmp4422:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp4423:
	.loc	26 71 9
	vmulss	%xmm4, %xmm3, %xmm3
.Ltmp4424:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm4, %xmm7, %xmm7
.Ltmp4425:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp4426:
	.loc	7 1244 18
	vmovd	%xmm3, %edi
.Ltmp4427:
	.loc	26 161 24
	cmovbel	%r11d, %edi
.Ltmp4428:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp4429:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm3
.Ltmp4430:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp4431:
	.loc	21 451 21
	vmovss	792(%rbx), %xmm3
.Ltmp4432:
	.loc	7 1291 18
	vmovd	%edi, %xmm7
.Ltmp4433:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm7
.Ltmp4434:
	.loc	26 161 24
	cmovbel	%r9d, %edi
.Ltmp4435:
	.loc	26 185 42
	movl	%edi, %r11d
	andl	$8388607, %r11d
	orl	$1065353216, %r11d
.Ltmp4436:
	.loc	7 1291 18
	vmovd	%r11d, %xmm7
.Ltmp4437:
	.loc	26 66 9
	vaddss	%xmm7, %xmm11, %xmm7
.Ltmp4438:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm7, %xmm8
.Ltmp4439:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm9
	vsubss	%xmm8, %xmm9, %xmm8
.Ltmp4440:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4441:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm8, %xmm8
.Ltmp4442:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4443:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm8, %xmm8
.Ltmp4444:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4445:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm8, %xmm8
.Ltmp4446:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm8
.Ltmp4447:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm8, %xmm8
.Ltmp4448:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp4449:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp4450:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm9, %xmm9
.Ltmp4451:
	.loc	26 71 9
	vmulss	%xmm7, %xmm8, %xmm7
.Ltmp4452:
	.loc	26 61 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp4453:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm7, %xmm7
.Ltmp4454:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm7, %xmm7
.Ltmp4455:
	.loc	26 66 9
	vsubss	840(%rbx), %xmm3, %xmm8
.Ltmp4456:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm7, %xmm7
.Ltmp4457:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm7
.Ltmp4458:
	.loc	26 28 5
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp4459:
	.loc	26 129 14
	vucomiss	%xmm3, %xmm7
.Ltmp4460:
	.loc	26 144 9
	movl	$0, %r11d
	adcl	$-1, %r11d
.Ltmp4461:
	.loc	26 149 9
	movl	%ebp, %edi
.Ltmp4462:
	.loc	21 486 47
	vmovss	860(%rbx), %xmm8
.Ltmp4463:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm8
.Ltmp4464:
	.loc	26 149 9
	notl	%edi
.Ltmp4465:
	.loc	26 139 9
	cmovbel	%r10d, %edi
.Ltmp4466:
	.loc	21 478 20
	vmovss	856(%rbx), %xmm9
.Ltmp4467:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp4468:
	.loc	26 144 9
	cmoval	%ebp, %r11d
.Ltmp4469:
	.loc	26 139 9
	cmovbel	%r10d, %edi
.Ltmp4470:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_733
.Ltmp4471:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm8, %xmm8
.LBB21_733:
.Ltmp4472:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r11b
	je	.LBB21_735
.Ltmp4473:
	.loc	21 0 0 is_stmt 0
	vmovss	752(%rbx), %xmm8
.LBB21_735:
	orl	%r11d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm9
.Ltmp4474:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm8, 860(%rbx)
	.loc	21 498 5
	movl	%edi, 856(%rbx)
.Ltmp4475:
	.loc	26 66 9
	vaddss	808(%rbx), %xmm11, %xmm8
.Ltmp4476:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm3, %xmm7, %xmm3
.Ltmp4477:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm8, %xmm3
.Ltmp4478:
	.loc	26 98 24
	vmovss	824(%rbx), %xmm7
	vbroadcastss	.LCPI21_16(%rip), %xmm8
	vxorps	%xmm7, %xmm8, %xmm7
.Ltmp4479:
	.loc	26 161 24
	vmaxss	%xmm7, %xmm3, %xmm3
.Ltmp4480:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm9, %xmm2, %xmm7
	vcmpltps	%xmm2, %xmm3, %xmm8
	vandps	%xmm7, %xmm8, %xmm7
	vmovd	%xmm7, %edi
	testb	$1, %dil
	jne	.LBB21_737
.Ltmp4481:
	.loc	26 0 44
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_737:
.Ltmp4482:
	.loc	21 508 36 is_stmt 1
	vmovss	864(%rbx), %xmm7
.Ltmp4483:
	.loc	26 124 14
	xorl	%edi, %edi
	vucomiss	%xmm7, %xmm3
	setbe	%dil
.Ltmp4484:
	.loc	26 66 9
	vsubss	%xmm7, %xmm3, %xmm3
.Ltmp4485:
	.loc	26 92 9
	vmulss	744(%rbx,%rdi,4), %xmm3, %xmm3
	vaddss	%xmm3, %xmm7, %xmm3
.Ltmp4486:
	.loc	26 103 24
	vandps	%xmm3, %xmm12, %xmm7
.Ltmp4487:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm7, %xmm7
	vandps	%xmm3, %xmm7, %xmm3
.Ltmp4488:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm6
.Ltmp4489:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm5, %xmm12, %xmm5
.Ltmp4490:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm5, %xmm6
.Ltmp4491:
	.loc	7 1244 18
	vmovd	%xmm6, %edi
.Ltmp4492:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm5, %r11d
.Ltmp4493:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r11d
.Ltmp4494:
	.loc	21 510 5
	vmovss	%xmm3, 864(%rbx)
.Ltmp4495:
	.loc	21 459 23
	vmovss	784(%rbx), %xmm7
.Ltmp4496:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp4497:
	.loc	26 161 24
	cmovbel	%edi, %r11d
.Ltmp4498:
	.loc	21 461 9
	vmovss	788(%rbx), %xmm7
.Ltmp4499:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm7
.Ltmp4500:
	.loc	26 71 9
	vmulss	%xmm4, %xmm6, %xmm6
.Ltmp4501:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm4, %xmm5, %xmm5
.Ltmp4502:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm6, %xmm5, %xmm5
.Ltmp4503:
	.loc	7 1244 18
	vmovd	%xmm5, %edi
.Ltmp4504:
	.loc	26 161 24
	cmovbel	%r11d, %edi
.Ltmp4505:
	.loc	7 1291 18
	vmovd	%edi, %xmm5
.Ltmp4506:
	.loc	26 124 14
	vucomiss	%xmm10, %xmm5
.Ltmp4507:
	.loc	26 161 24
	cmovbel	%r8d, %edi
.Ltmp4508:
	.loc	21 451 21
	vmovss	868(%rbx), %xmm5
.Ltmp4509:
	.loc	7 1291 18
	vmovd	%edi, %xmm6
.Ltmp4510:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm6
.Ltmp4511:
	.loc	26 161 24
	cmovbel	%r9d, %edi
.Ltmp4512:
	.loc	26 185 42
	movl	%edi, %r11d
	andl	$8388607, %r11d
	orl	$1065353216, %r11d
.Ltmp4513:
	.loc	7 1291 18
	vmovd	%r11d, %xmm6
.Ltmp4514:
	.loc	26 66 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp4515:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm6, %xmm7
.Ltmp4516:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm8
	vsubss	%xmm7, %xmm8, %xmm7
.Ltmp4517:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4518:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm7, %xmm7
.Ltmp4519:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4520:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm7, %xmm7
.Ltmp4521:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4522:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm7, %xmm7
.Ltmp4523:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm7
.Ltmp4524:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm7, %xmm7
.Ltmp4525:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp4526:
	.loc	7 1291 18
	vmovd	%edi, %xmm8
.Ltmp4527:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm8, %xmm8
.Ltmp4528:
	.loc	26 71 9
	vmulss	%xmm7, %xmm6, %xmm6
.Ltmp4529:
	.loc	26 61 9
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp4530:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm6, %xmm6
.Ltmp4531:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm6, %xmm6
.Ltmp4532:
	.loc	26 66 9
	vsubss	916(%rbx), %xmm5, %xmm7
.Ltmp4533:
	.loc	26 161 24
	vmaxss	.LCPI21_15(%rip), %xmm6, %xmm8
.Ltmp4534:
	.loc	26 129 14
	vucomiss	%xmm7, %xmm8
.Ltmp4535:
	.loc	26 28 5
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp4536:
	.loc	26 129 14
	vucomiss	%xmm5, %xmm8
.Ltmp4537:
	.loc	26 144 9
	movl	$0, %r11d
	adcl	$-1, %r11d
.Ltmp4538:
	.loc	26 149 9
	movl	%ebp, %edi
.Ltmp4539:
	.loc	21 486 47
	vmovss	936(%rbx), %xmm9
.Ltmp4540:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm9
.Ltmp4541:
	.loc	26 149 9
	notl	%edi
.Ltmp4542:
	.loc	26 139 9
	cmovbel	%r10d, %edi
.Ltmp4543:
	.loc	21 478 20
	vmovss	932(%rbx), %xmm6
.Ltmp4544:
	.loc	26 124 14
	vucomiss	%xmm2, %xmm6
.Ltmp4545:
	.loc	26 144 9
	cmoval	%ebp, %r11d
.Ltmp4546:
	.loc	26 139 9
	cmovbel	%r10d, %edi
.Ltmp4547:
	.loc	26 161 24
	testb	$1, %dil
	je	.LBB21_739
.Ltmp4548:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm11, %xmm9, %xmm9
.LBB21_739:
	movq	64(%rsp), %rdx
	movq	48(%rsp), %rbp
.Ltmp4549:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r11b
	je	.LBB21_741
.Ltmp4550:
	.loc	21 0 0 is_stmt 0
	vmovss	776(%rbx), %xmm9
.Ltmp4551:
.LBB21_741:
	vmulss	.LCPI21_18(%rip), %xmm3, %xmm6
	vmaxss	.LCPI21_19(%rip), %xmm6, %xmm6
	vminss	.LCPI21_20(%rip), %xmm6, %xmm7
	vroundss	$9, %xmm7, %xmm7, %xmm6
	vsubss	%xmm6, %xmm7, %xmm7
.Ltmp4552:
	orl	%r11d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp4553:
	.loc	21 489 5 is_stmt 1
	vmovss	%xmm9, 936(%rbx)
	.loc	21 498 5
	movl	%edi, 932(%rbx)
	vmovaps	%xmm11, %xmm0
.Ltmp4554:
	.loc	26 66 9
	vaddss	884(%rbx), %xmm11, %xmm9
.Ltmp4555:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm5, %xmm8, %xmm5
.Ltmp4556:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm5, %xmm9, %xmm5
.Ltmp4557:
	.loc	26 98 24
	vmovss	900(%rbx), %xmm8
	vbroadcastss	.LCPI21_16(%rip), %xmm9
	vxorps	%xmm9, %xmm8, %xmm8
.Ltmp4558:
	.loc	26 161 24
	vmaxss	%xmm8, %xmm5, %xmm5
.Ltmp4559:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm10, %xmm2, %xmm8
	vcmpltps	%xmm2, %xmm5, %xmm9
	vandps	%xmm9, %xmm8, %xmm8
	vmovd	%xmm8, %edi
	testb	$1, %dil
	jne	.LBB21_714
.Ltmp4560:
	.loc	26 0 44
	vxorps	%xmm5, %xmm5, %xmm5
	jmp	.LBB21_714
.LBB21_743:
	movq	336(%rsp), %r10
	movq	%r10, %rax
	movq	200(%rsp), %rsi
	subq	%rsi, %rax
.Ltmp4561:
	.loc	15 2584 13 is_stmt 1
	addl	%ebp, %eax
.Ltmp4562:
	.loc	21 413 5
	movl	%eax, 1208(%rbx)
	movq	216(%rsp), %r8
	movq	208(%rsp), %r11
.Ltmp4563:
.LBB21_744:
	.loc	6 701 9
	subl	%esi, 1220(%rbx)
	movq	344(%rsp), %rcx
.Ltmp4564:
	.loc	6 773 33
	leaq	240(%rsp), %r9
	movq	88(%rsp), %rax
	movq	%rax, 240(%rsp)
	movq	%r11, 248(%rsp)
	movq	%rcx, %r11
	movq	152(%rsp), %rax
	movq	%rax, 256(%rsp)
	movq	%r8, 264(%rsp)
	movl	72(%rbx), %eax
	movq	%rax, 88(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sd	%rax, %xmm0, %xmm3
	movq	1200(%rbx), %rbp
	movl	1216(%rbx), %eax
	movl	%eax, 48(%rsp)
	movl	$24, %r14d
	xorl	%r13d, %r13d
	vbroadcastss	.LCPI21_2(%rip), %xmm4
	vmovss	.LCPI21_27(%rip), %xmm5
	vmovsd	.LCPI21_28(%rip), %xmm6
	vmovsd	.LCPI21_29(%rip), %xmm7
	vmovsd	.LCPI21_30(%rip), %xmm8
	xorl	%r15d, %r15d
	xorl	%ecx, %ecx
	vmovsd	%xmm3, 64(%rsp)
	vmovaps	%xmm4, 96(%rsp)
	jmp	.LBB21_747
	.loc	6 0 33 is_stmt 0
.Ltmp4565:
	.p2align	4
.LBB21_745:
	movq	(%r11,%r14), %rax
.Ltmp4566:
	.loc	15 2428 13 is_stmt 1
	addq	%r10, %rax
	movq	$-1, %rcx
	cmovbq	%rcx, %rax
.Ltmp4567:
	.loc	6 794 17
	movq	%rax, (%r11,%r14)
	leaq	240(%rsp), %r9
.Ltmp4568:
.LBB21_746:
	.loc	6 0 17 is_stmt 0
	movl	$1, %ecx
	movl	$32, %r14d
.Ltmp4569:
	.file	27 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/ops/index_range.rs"
	.loc	27 131 12 is_stmt 1
	testb	$1, %r15b
	movb	$1, %r15b
	jne	.LBB21_772
.Ltmp4570:
.LBB21_747:
	.loc	25 253 13
	movq	%rcx, %rdx
	shlq	$4, %rdx
.Ltmp4571:
	.loc	1 1733 9
	movq	(%r9,%rdx), %rax
	movq	8(%r9,%rdx), %rdi
.Ltmp4572:
	.loc	6 774 13
	imulq	$76, %rcx, %r12
	vmovss	864(%rbx,%r12), %xmm0
.Ltmp4573:
	.loc	16 2155 12
	testq	%rdi, %rdi
	je	.LBB21_755
.Ltmp4574:
	.loc	16 0 12 is_stmt 0
	movl	$-1, %esi
	xorl	%r8d, %r8d
	.p2align	4
.LBB21_749:
.Ltmp4575:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rax,%r8,4), %xmm1
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp4576:
	.loc	26 114 14
	vucomiss	%xmm1, %xmm5
.Ltmp4577:
	.loc	26 139 9
	cmovbel	%r13d, %esi
.Ltmp4578:
	.loc	16 2155 12
	incq	%r8
	cmpq	%r8, %rdi
	jne	.LBB21_749
.Ltmp4579:
	.loc	16 0 12 is_stmt 0
	vandps	%xmm4, %xmm0, %xmm0
	vucomiss	%xmm0, %xmm5
	.loc	6 775 16 is_stmt 1
	jbe	.LBB21_752
	cmpl	$-1, %esi
	je	.LBB21_746
.LBB21_752:
	.loc	6 0 16 is_stmt 0
	movl	$-1, %esi
	xorl	%r8d, %r8d
	.p2align	4
.LBB21_753:
.Ltmp4580:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rax,%r8,4), %xmm1
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp4581:
	.loc	26 114 14
	vucomiss	%xmm1, %xmm5
.Ltmp4582:
	.loc	26 139 9
	cmovbel	%r13d, %esi
.Ltmp4583:
	.loc	16 2155 12
	incq	%r8
	cmpq	%r8, %rdi
	jne	.LBB21_753
.Ltmp4584:
	.file	28 "/home/bl/misofm/engine-gate-detector-access" "crates/effect-runtime/src/bank.rs"
	.loc	28 185 12
	notl	%esi
	xorl	%r8d, %r8d
	testl	$1065353216, %esi
	setne	%r8b
	jmp	.LBB21_757
.Ltmp4585:
	.loc	28 0 12 is_stmt 0
.Ltmp4586:
	.p2align	4
.LBB21_755:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm4, %xmm0, %xmm0
.Ltmp4587:
	.loc	26 114 14
	vucomiss	%xmm0, %xmm5
.Ltmp4588:
	.loc	6 775 43
	ja	.LBB21_746
	.loc	6 0 43 is_stmt 0
	xorl	%r8d, %r8d
.LBB21_757:
.Ltmp4589:
	.loc	26 114 14 is_stmt 1
	xorl	%esi, %esi
	vucomiss	%xmm0, %xmm5
	setbe	%sil
	orl	%r8d, %esi
	je	.LBB21_746
.Ltmp4590:
	.loc	26 0 14 is_stmt 0
	testq	%r10, %r10
.Ltmp4591:
	.loc	11 900 12 is_stmt 1
	je	.LBB21_762
.Ltmp4592:
	.loc	11 0 12 is_stmt 0
	xorl	%esi, %esi
	.p2align	4
.LBB21_760:
.Ltmp4593:
	.loc	6 785 21 is_stmt 1
	cmpq	%rsi, %rdi
	je	.LBB21_792
	movl	$0, (%rax,%rsi,4)
.Ltmp4594:
	.loc	15 971 17
	incq	%rsi
.Ltmp4595:
	.loc	8 1916 50
	cmpq	%rsi, %r10
.Ltmp4596:
	.loc	11 900 12
	jne	.LBB21_760
.Ltmp4597:
.LBB21_762:
	.loc	11 0 12 is_stmt 0
	movq	%rcx, %r8
	shlq	$6, %r8
	testq	%rbp, %rbp
.Ltmp4598:
	.loc	11 900 12
	je	.LBB21_767
.Ltmp4599:
	.loc	11 0 12
	leaq	104(%rbx), %rax
	leaq	(%rax,%r8), %r9
	movq	8(%r9), %rdi
	xorl	%eax, %eax
	.p2align	4
.LBB21_764:
.Ltmp4600:
	.loc	6 565 13 is_stmt 1
	cmpq	%rax, %rdi
	je	.LBB21_794
	movq	(%r9), %rsi
	movl	$0, (%rsi,%rax,4)
	.loc	6 567 17
	movq	24(%r9), %rsi
	cmpq	%rsi, %rax
	jae	.LBB21_793
	movq	16(%r9), %rsi
	movl	$0, (%rsi,%rax,4)
.Ltmp4601:
	.loc	6 0 0 is_stmt 0
	incq	%rax
.Ltmp4602:
	.loc	8 1916 50 is_stmt 1
	cmpq	%rax, %rbp
.Ltmp4603:
	.loc	11 900 12
	jne	.LBB21_764
.Ltmp4604:
.LBB21_767:
	.loc	11 0 12 is_stmt 0
	movq	%rcx, %rax
	shlq	$5, %rax
	leaq	232(%rbx), %rsi
	addq	%rsi, %rax
	leaq	944(%rbx), %rsi
	addq	%rsi, %rdx
.Ltmp4605:
	.loc	6 509 22 is_stmt 1
	vmovss	(%rax), %xmm12
	vmovss	4(%rax), %xmm11
	vmovss	8(%rax), %xmm10
	vmovss	12(%rax), %xmm9
	vmovss	16(%rax), %xmm0
	vmovss	20(%rax), %xmm2
	vmovss	24(%rax), %xmm13
	vmovss	28(%rax), %xmm1
.Ltmp4606:
	.loc	6 510 9
	vmovss	%xmm1, (%rdx)
	vmovss	%xmm0, 4(%rdx)
	vmovss	%xmm2, 8(%rdx)
	vmovss	%xmm13, 12(%rdx)
.Ltmp4607:
	.loc	9 82 17
	vcvtss2sd	%xmm1, %xmm1, %xmm1
.Ltmp4608:
	.loc	6 406 20
	vmulsd	%xmm1, %xmm3, %xmm1
	vdivsd	%xmm6, %xmm1, %xmm1
	.loc	6 406 19 is_stmt 0
	vaddsd	%xmm7, %xmm1, %xmm1
.Ltmp4609:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm1, %xmm1, %xmm1
	vmovq	%xmm1, %rax
.Ltmp4610:
	.loc	6 407 9
	testq	%rax, %rax
	sets	%dl
	movabsq	$9223372036854775807, %rsi
	andq	%rsi, %rax
	movabsq	$-4503599627370496, %rsi
	addq	%rax, %rsi
	shrq	$53, %rsi
	cmpl	$1023, %esi
	setb	%sil
	andb	%dl, %sil
	movabsq	$9218868437227405311, %rdx
	cmpq	%rdx, %rax
	setg	%al
	orb	%sil, %al
	jne	.LBB21_745
	vucomisd	%xmm8, %xmm1
	ja	.LBB21_745
.Ltmp4611:
	.loc	9 82 17
	vcvtss2sd	%xmm2, %xmm2, %xmm2
.Ltmp4612:
	.loc	6 406 20
	vmulsd	%xmm2, %xmm3, %xmm2
	vdivsd	%xmm6, %xmm2, %xmm2
	.loc	6 406 19 is_stmt 0
	vaddsd	%xmm7, %xmm2, %xmm2
.Ltmp4613:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm2, %xmm2, %xmm2
	vmovq	%xmm2, %rax
.Ltmp4614:
	.loc	6 407 9
	testq	%rax, %rax
	sets	%dl
	movabsq	$9223372036854775807, %rsi
	andq	%rsi, %rax
	movabsq	$-4503599627370496, %rsi
	addq	%rax, %rsi
	shrq	$53, %rsi
	cmpl	$1023, %esi
	setb	%sil
	andb	%dl, %sil
	movabsq	$9218868437227405311, %rdx
	cmpq	%rdx, %rax
	setg	%al
	orb	%sil, %al
	jne	.LBB21_745
	vucomisd	%xmm8, %xmm2
	ja	.LBB21_745
.Ltmp4615:
	.loc	6 0 9 is_stmt 0
	addq	%rbx, %r8
	leaq	792(%rbx), %rax
	addq	%rax, %r12
	leaq	(%rcx,%rcx,2), %rax
	leaq	744(%rbx), %rcx
	leaq	(%rcx,%rax,8), %rsi
	movq	%rsi, 8(%rsp)
	vxorps	%xmm5, %xmm5, %xmm5
.Ltmp4616:
	.loc	6 410 10 is_stmt 1
	vmaxsd	%xmm1, %xmm5, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rax
.Ltmp4617:
	.loc	6 410 10 is_stmt 0
	vmaxsd	%xmm2, %xmm5, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rcx
	movl	48(%rsp), %edx
.Ltmp4618:
	.loc	6 536 9 is_stmt 1
	subl	%eax, %edx
	cmovbl	%r13d, %edx
	movl	%edx, 136(%r8)
	.loc	6 537 62
	movl	%ecx, %eax
	vcvtsi2ss	%rax, %xmm15, %xmm1
	vmovss	%xmm1, 16(%rsp)
.Ltmp4619:
	.loc	6 401 5
	vmovss	%xmm1, 8(%rsi)
	movq	88(%rsp), %rdi
	vmovss	%xmm9, 40(%rsp)
	vmovss	%xmm10, 32(%rsp)
	vmovss	%xmm11, 24(%rsp)
	vmovss	%xmm12, 56(%rsp)
	vmovss	%xmm13, 72(%rsp)
.Ltmp4620:
	.loc	6 541 13
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movq	8(%rsp), %rax
.Ltmp4621:
	.loc	6 401 5
	vmovss	%xmm0, (%rax)
	vmovss	72(%rsp), %xmm0
	movq	88(%rsp), %rdi
.Ltmp4622:
	.loc	6 546 13
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	vmovsd	.LCPI21_30(%rip), %xmm8
	vmovsd	.LCPI21_29(%rip), %xmm7
	vmovsd	.LCPI21_28(%rip), %xmm6
	vmovss	.LCPI21_27(%rip), %xmm5
	vmovaps	96(%rsp), %xmm4
	vmovsd	64(%rsp), %xmm3
	movq	344(%rsp), %r11
	movq	336(%rsp), %r10
	movq	8(%rsp), %rax
.Ltmp4623:
	.loc	6 401 5
	vmovss	%xmm0, 4(%rax)
.Ltmp4624:
	.loc	6 401 5 is_stmt 0
	movl	$0, 72(%r12)
.Ltmp4625:
	.loc	6 401 5
	movl	$1065353216, 64(%r12)
	vmovss	16(%rsp), %xmm0
.Ltmp4626:
	.loc	6 401 5
	vmovss	%xmm0, 68(%r12)
	vmovss	56(%rsp), %xmm0
.Ltmp4627:
	.loc	6 401 5
	vmovss	%xmm0, (%r12)
.Ltmp4628:
	.loc	6 401 5
	vmovss	%xmm0, 4(%r12)
.Ltmp4629:
	.loc	6 401 5
	movq	$0, 8(%r12)
	vmovss	24(%rsp), %xmm0
.Ltmp4630:
	.loc	6 401 5
	vmovss	%xmm0, 16(%r12)
.Ltmp4631:
	.loc	6 401 5
	vmovss	%xmm0, 20(%r12)
.Ltmp4632:
	.loc	6 401 5
	movq	$0, 24(%r12)
	vmovss	32(%rsp), %xmm0
.Ltmp4633:
	.loc	6 401 5
	vmovss	%xmm0, 32(%r12)
.Ltmp4634:
	.loc	6 401 5
	vmovss	%xmm0, 36(%r12)
.Ltmp4635:
	.loc	6 401 5
	movq	$0, 40(%r12)
	vmovss	40(%rsp), %xmm0
.Ltmp4636:
	.loc	6 401 5
	vmovss	%xmm0, 48(%r12)
.Ltmp4637:
	.loc	6 401 5
	vmovss	%xmm0, 52(%r12)
.Ltmp4638:
	.loc	6 401 5
	movq	$0, 56(%r12)
	jmp	.LBB21_745
.Ltmp4639:
.LBB21_772:
	.loc	6 703 6 epilogue_begin is_stmt 1
	addq	$280, %rsp
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
.LBB21_773:
	.cfi_def_cfa_offset 336
	.loc	6 0 6 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp4640:
	leaq	.Lalloc_1eeef3195352adc58f4bfdb015316fef(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB21_774:
	leaq	1(%rdi), %rsi
	leaq	.Lalloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB21_775:
.Ltmp4641:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_d0dbd696a058609bd94bbe1fa75d0723(%rip), %rcx
	movq	200(%rsp), %rdi
.Ltmp4642:
	.loc	25 443 13 is_stmt 0
	movq	%r15, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4643:
.LBB21_776:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_31945e5cade31a240e4e50932dbe7a26(%rip), %rcx
	movq	176(%rsp), %rdi
.Ltmp4644:
	.loc	25 443 13 is_stmt 0
	movq	%r15, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4645:
.LBB21_777:
	.loc	25 0 13
	leaq	1(%rdi), %rsi
.Ltmp4646:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_a44e1bcb659c545f1ffe97492fde2ec9(%rip), %rcx
	movq	96(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4647:
.LBB21_778:
	.loc	25 0 13 is_stmt 0
	movq	168(%rsp), %rdi
.Ltmp4648:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_b346da41cf48210ef0dcce3f5e66934c(%rip), %rcx
.Ltmp4649:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4650:
.LBB21_779:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_31945e5cade31a240e4e50932dbe7a26(%rip), %rcx
	movq	176(%rsp), %rdi
.Ltmp4651:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4652:
.LBB21_780:
	.loc	25 0 13
	leaq	1(%rdi), %rsi
.Ltmp4653:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_b9d56679ca30f2caa500ddf2e89d00cb(%rip), %rcx
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4654:
.LBB21_781:
	.loc	25 0 13 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp4655:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_f198228fd40f4c04a48a745576c5c5ee(%rip), %rcx
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4656:
.LBB21_782:
	.loc	25 0 13 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp4657:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_b9d56679ca30f2caa500ddf2e89d00cb(%rip), %rcx
	movq	%r12, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4658:
.LBB21_783:
	.loc	25 0 13 is_stmt 0
	leaq	1(%rdi), %rsi
.Ltmp4659:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_f198228fd40f4c04a48a745576c5c5ee(%rip), %rcx
	movq	%r12, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4660:
.LBB21_784:
	.loc	25 443 13
	leaq	.Lalloc_d9529ff5ddc99dd60299cff5ff3cd676(%rip), %rcx
	movq	160(%rsp), %rdi
.Ltmp4661:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4662:
.LBB21_786:
	.loc	25 0 13
	movq	160(%rsp), %rdi
.Ltmp4663:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_d0dbd696a058609bd94bbe1fa75d0723(%rip), %rcx
.Ltmp4664:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4665:
.LBB21_787:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_0f518473f2bd55505cd4207a72f88500(%rip), %rcx
	xorl	%edi, %edi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4666:
.LBB21_788:
	.loc	25 456 13
	leaq	.Lalloc_1d7cc6e40c752396aa7def7556a6c433(%rip), %rcx
	xorl	%edi, %edi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4667:
.LBB21_789:
	.loc	25 581 13
	leaq	.Lalloc_a6d4388bd1c2ee005f6a969a0e3ca0f4(%rip), %rcx
	movq	%rsi, %rdi
	movq	%r8, %rsi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4668:
.LBB21_790:
	.loc	25 0 13 is_stmt 0
	movq	%r13, %rdi
.Ltmp4669:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_d0dbd696a058609bd94bbe1fa75d0723(%rip), %rcx
.Ltmp4670:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4671:
.LBB21_791:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_22dea5e023495603116a6a1e47cd356c(%rip), %rcx
	xorl	%edi, %edi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4672:
.LBB21_792:
	.loc	6 785 21
	leaq	.Lalloc_6797264598a169e4722ae66c7bc497b8(%rip), %rdx
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4673:
.LBB21_793:
	.loc	6 567 17
	leaq	.Lalloc_9043cc21280ce18935f79be264d94710(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_794:
	.loc	6 565 13
	leaq	.Lalloc_835aafef72e8508601474e7b1f4172a9(%rip), %rdx
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4674:
.LBB21_795:
	.loc	25 569 13
	leaq	.Lalloc_b376fc9587b2ace3f5c69e612169ea0b(%rip), %rcx
.Ltmp4675:
	.loc	25 569 13 is_stmt 0
	movq	%rsi, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4676:
.LBB21_796:
	.loc	25 569 13 is_stmt 1
	leaq	.Lalloc_23ac76c781f7ef32d3975251265b95f9(%rip), %rcx
.Ltmp4677:
	.loc	25 569 13 is_stmt 0
	movq	%rsi, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4678:
.LBB21_797:
	.loc	6 0 0
	leaq	.Lalloc_ed84bd2438b7b7e3dfb2147bac09e923(%rip), %rdx
	movq	%rbp, %rdi
	movq	8(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_798:
.Ltmp4679:
	.loc	21 237 33 is_stmt 1
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
.Ltmp4680:
	.loc	21 0 0 is_stmt 0
	movq	%rbp, %rdi
	movq	16(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_799:
.Ltmp4681:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	%r13, %rdi
	movq	16(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_800:
	.loc	21 268 33
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp4682:
	.loc	21 0 0 is_stmt 0
	movq	%rbp, %rdi
	movq	16(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_801:
.Ltmp4683:
	.loc	21 251 32 is_stmt 1
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
.Ltmp4684:
	.loc	21 0 0 is_stmt 0
	movq	%r13, %rdi
	movq	8(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_802:
.Ltmp4685:
	.loc	21 236 32 is_stmt 1
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
.Ltmp4686:
	.loc	21 0 0 is_stmt 0
	movq	%r13, %rdi
	movq	8(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_803:
.Ltmp4687:
	.loc	21 266 33 is_stmt 1
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
.Ltmp4688:
	.loc	21 0 0 is_stmt 0
	movq	%r13, %rdi
	movq	8(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_804:
.Ltmp4689:
	.loc	21 252 33 is_stmt 1
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp4690:
	.loc	21 0 0 is_stmt 0
	movq	%rbp, %rdi
	movq	16(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4691:
.LBB21_805:
	.loc	21 236 32 is_stmt 1
	leaq	.Lalloc_cba26bb3a04aaba20f9c249edc5e133d(%rip), %rdx
	movq	8(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_806:
.Ltmp4692:
	.loc	21 237 33
	leaq	.Lalloc_2ad2d313de59c36d62f4a7d75017a71f(%rip), %rdx
.Ltmp4693:
	.loc	21 0 0 is_stmt 0
	movq	%rbp, %rdi
	movq	96(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_807:
.Ltmp4694:
	.loc	21 267 33 is_stmt 1
	leaq	.Lalloc_60256fc2b51ee5bb69caa4304418caff(%rip), %rdx
	movq	96(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_808:
	.loc	21 268 33
	leaq	.Lalloc_03bcac171bcf0d517141d8cd3ac9ded0(%rip), %rdx
.Ltmp4695:
	.loc	21 0 0 is_stmt 0
	movq	%rbp, %rdi
	movq	96(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_809:
.Ltmp4696:
	.loc	21 251 32 is_stmt 1
	leaq	.Lalloc_e2e01e5f964095ee8e5aa091c7d92b88(%rip), %rdx
	movq	8(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4697:
.LBB21_810:
	.loc	21 266 33
	leaq	.Lalloc_4b7b2eb7fa1b19ad0451b09b71313122(%rip), %rdx
	movq	8(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4698:
.LBB21_811:
	.loc	21 252 33
	leaq	.Lalloc_48dca199019b49040caac979cf1ddc5a(%rip), %rdx
.Ltmp4699:
	.loc	21 0 0 is_stmt 0
	movq	%rbp, %rdi
	movq	96(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4700:
.Lfunc_end21:
	.size	_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_, .Lfunc_end21-_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_
