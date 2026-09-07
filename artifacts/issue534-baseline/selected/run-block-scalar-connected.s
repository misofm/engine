_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_:
.Lfunc_begin21:
	.loc	6 673 0
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
	subq	$440, %rsp
	.cfi_def_cfa_offset 496
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r15
	movq	%rsi, %r13
	movq	%rdi, %rbx
	movq	496(%rsp), %r11
.Ltmp1087:
	.loc	6 682 25 prologue_end
	movl	1220(%rdi), %eax
.Ltmp1088:
	.loc	8 1078 5
	cmpq	%rax, %r11
	movq	%rax, %rsi
	cmovbq	%r11, %rsi
.Ltmp1089:
	.loc	6 683 12
	testq	%rsi, %rsi
	movq	%rcx, 248(%rsp)
	movq	%r13, 240(%rsp)
	movq	%rsi, 344(%rsp)
	movq	%r8, 368(%rsp)
	movq	%rdx, 360(%rsp)
	je	.LBB21_1
.Ltmp1090:
	.loc	18 1161 15
	movq	(%r9), %rdx
	movq	%rdx, 40(%rsp)
	testq	%rdx, %rdx
	.loc	18 1161 9 is_stmt 0
	je	.LBB21_12
.Ltmp1091:
	.loc	18 1162 29 is_stmt 1
	movq	8(%r9), %rdx
	cmpq	%rdx, %rsi
.Ltmp1092:
	.loc	15 1050 16
	ja	.LBB21_706
.Ltmp1093:
	.loc	18 1162 29
	movq	24(%r9), %rdx
.Ltmp1094:
	.file	25 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/slice/index.rs"
	.loc	25 438 16
	cmpq	%rdx, %rsi
	ja	.LBB21_707
.Ltmp1095:
	.loc	18 1162 29
	movq	16(%r9), %rdx
	movq	%rdx, 48(%rsp)
	cmpq	%r15, %rsi
.Ltmp1096:
	.loc	15 1050 16
	ja	.LBB21_708
.Ltmp1097:
.LBB21_17:
	.loc	25 451 16
	cmpq	%r8, %rsi
	ja	.LBB21_709
.Ltmp1098:
	.loc	6 730 27
	movq	112(%rbx), %rbp
	.loc	6 735 27
	movq	176(%rbx), %rdx
.Ltmp1099:
	.loc	18 1161 15
	cmpq	$0, 40(%rsp)
.Ltmp1100:
	.loc	18 1039 9
	je	.LBB21_19
.Ltmp1101:
	.loc	18 0 9 is_stmt 0
	movq	%rsi, %r10
	jmp	.LBB21_21
.LBB21_12:
	cmpq	%r15, %rsi
.Ltmp1102:
	.loc	15 1050 16 is_stmt 1
	jbe	.LBB21_17
.Ltmp1103:
.LBB21_708:
	.loc	25 456 13
	leaq	.Lalloc_bc75afababbf20974edb8b966373fd0c(%rip), %rcx
	xorl	%edi, %edi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp1104:
.LBB21_19:
	.loc	25 0 13 is_stmt 0
	xorl	%r10d, %r10d
	movl	$4, %edi
	movq	%rdi, 48(%rsp)
	movq	%rdi, 40(%rsp)
.LBB21_21:
.Ltmp1105:
	movq	104(%rbx), %rdi
	movq	%rdi, 8(%rsp)
	movq	120(%rbx), %r11
	movq	128(%rbx), %rdi
	movq	%rdi, (%rsp)
	movq	168(%rbx), %rdi
	movq	%rdi, 24(%rsp)
	movq	184(%rbx), %r14
	movq	192(%rbx), %rdi
	movq	%rdi, 32(%rsp)
	movl	1212(%rbx), %edi
	movl	1216(%rbx), %r8d
.Ltmp1106:
	.loc	21 238 16 is_stmt 1
	movl	1208(%rbx), %r12d
	cmpq	%rdx, %rbp
	movq	%rbp, 176(%rsp)
	movq	%r12, 16(%rsp)
	movq	%rax, 256(%rsp)
	movq	%r9, 152(%rsp)
	jbe	.LBB21_112
	.loc	21 0 16 is_stmt 0
	movq	%r10, 56(%rsp)
.Ltmp1107:
	.loc	25 438 16 is_stmt 1
	movq	%r10, %r9
	negq	%r9
	movq	%rsi, %r10
	negq	%r10
	movl	%r12d, %esi
	subl	%r8d, %esi
	movl	$1, %r15d
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	xorl	%eax, %eax
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB21_23
.Ltmp1108:
	.loc	25 0 16 is_stmt 0
.Ltmp1109:
	.p2align	4
.LBB21_111:
	.loc	21 392 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp1110:
	.file	26 "/home/bl/misofm/engine-gate-detector-access" "crates/lane/src/scalar.rs"
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp1111:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1112:
	.loc	26 92 9
	vmulss	768(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1113:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1114:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp1115:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp1116:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp1117:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp1118:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp1119:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp1120:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp1121:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm9
.Ltmp1122:
	.loc	26 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp1123:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp1124:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp1125:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp1126:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp1127:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp1128:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp1129:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1130:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm5, %xmm2
.Ltmp1131:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm15, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm5, %xmm2
.Ltmp1132:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp1133:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp1134:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm6
.Ltmp1135:
	.loc	26 161 24
	vblendvps	%xmm6, %xmm2, %xmm5, %xmm2
.Ltmp1136:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm5
.Ltmp1137:
	.loc	26 66 9
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp1138:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp1139:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp1140:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1141:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp1142:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1143:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp1144:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1145:
	.loc	26 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1146:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1147:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp1148:
	.loc	26 178 22
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp1149:
	.loc	7 1244 18
	vmovd	%xmm5, %r8d
.Ltmp1150:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp1151:
	.loc	7 1291 18
	vmovd	%r8d, %xmm5
.Ltmp1152:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm3
.Ltmp1153:
	.loc	21 394 5
	vmovss	%xmm10, 940(%rbx)
.Ltmp1154:
	.loc	26 71 9
	vmulss	%xmm3, %xmm4, %xmm3
.Ltmp1155:
	.loc	26 161 24
	vcmpneqss	%xmm15, %xmm10, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	vcmpnltss	780(%rbx), %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	movq	240(%rsp), %r13
.Ltmp1156:
	.loc	26 56 9
	vmovss	%xmm2, -4(%r13,%r15,4)
	movq	248(%rsp), %rcx
.Ltmp1157:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rcx,%r15,4)
.Ltmp1158:
	.loc	8 1916 50 is_stmt 1
	leaq	(%r10,%r15), %r8
	incq	%r8
	incq	%r15
	cmpq	$1, %r8
	movq	176(%rsp), %rbp
.Ltmp1159:
	.loc	11 900 12
	je	.LBB21_374
.LBB21_23:
.Ltmp1160:
	.loc	15 971 17
	leaq	(%r10,%r15), %r8
.Ltmp1161:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_460
.Ltmp1162:
	.loc	21 0 0 is_stmt 0
	leal	(%r12,%r15), %r8d
	decl	%r8d
	andl	%edi, %r8d
.Ltmp1163:
	.loc	25 451 16 is_stmt 1
	cmpq	%r8, %rbp
	jbe	.LBB21_462
.Ltmp1164:
	.loc	26 51 9
	vmovss	-4(%r13,%r15,4), %xmm2
	movq	8(%rsp), %r12
.Ltmp1165:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%r8,4)
.Ltmp1166:
	.loc	25 451 16
	cmpq	%r8, %rdx
	jbe	.LBB21_463
.Ltmp1167:
	.loc	21 0 0 is_stmt 0
	leaq	(%r9,%r15), %r12
.Ltmp1168:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r15,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp1169:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%r8,4)
.Ltmp1170:
	.loc	25 438 16
	cmpq	$1, %r12
	je	.LBB21_464
.Ltmp1171:
	.loc	25 451 16
	cmpq	%r8, (%rsp)
	jbe	.LBB21_465
.Ltmp1172:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %rcx
.Ltmp1173:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r15,4), %xmm2
.Ltmp1174:
	.loc	26 56 9
	vmovss	%xmm2, (%r11,%r8,4)
	movq	32(%rsp), %rcx
.Ltmp1175:
	.loc	25 451 16
	cmpq	%r8, %rcx
	jbe	.LBB21_466
.Ltmp1176:
	.loc	25 0 16 is_stmt 0
	movq	48(%rsp), %r12
.Ltmp1177:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
.Ltmp1178:
	.loc	26 56 9
	vmovss	%xmm2, (%r14,%r8,4)
.Ltmp1179:
	.loc	21 255 21
	leal	(%rsi,%r15), %r12d
	decl	%r12d
	andl	%edi, %r12d
.Ltmp1180:
	.loc	25 438 16
	cmpq	%r12, %rbp
	jbe	.LBB21_467
.Ltmp1181:
	.loc	25 438 16 is_stmt 0
	cmpq	%r12, %rdx
	jbe	.LBB21_469
.Ltmp1182:
	.loc	25 0 16
	movq	16(%rsp), %r8
	addl	%r15d, %r8d
	movl	136(%rbx), %r13d
	notl	%r13d
	addl	%r8d, %r13d
	andl	%edi, %r13d
	cmpq	%r13, (%rsp)
	jbe	.LBB21_473
	cmpq	%r13, %rcx
	jbe	.LBB21_472
	movl	200(%rbx), %ebp
	notl	%ebp
	addl	%r8d, %ebp
	andl	%edi, %ebp
	cmpq	%rbp, %rcx
	jbe	.LBB21_471
	cmpq	%rbp, (%rsp)
	jbe	.LBB21_470
.Ltmp1183:
	.loc	21 323 26 is_stmt 1
	vmovss	804(%rbx), %xmm9
.Ltmp1184:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp1185:
	.loc	21 325 44
	vmovss	800(%rbx), %xmm10
	.loc	21 325 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp1186:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm8, %xmm3
	jne	.LBB21_38
.Ltmp1187:
	.loc	26 161 24
	jp	.LBB21_38
.Ltmp1188:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm3
.LBB21_38:
	jne	.LBB21_41
.Ltmp1189:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_41
.Ltmp1190:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm10, %xmm10, %xmm10
.LBB21_41:
.Ltmp1191:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm9
.Ltmp1192:
	.loc	26 161 24
	jbe	.LBB21_43
.Ltmp1193:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm8
.LBB21_43:
	movq	8(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm5
	movq	24(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm4
	vmovss	(%r14,%r13,4), %xmm11
.Ltmp1194:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r11,%r13,4), %xmm12
.Ltmp1195:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r14,%rbp,4), %xmm2
	vmovaps	%xmm2, 192(%rsp)
.Ltmp1196:
	.loc	26 103 24
	vmovss	(%r11,%rbp,4), %xmm7
.Ltmp1197:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 331 13
	vmovss	%xmm10, 800(%rbx)
.Ltmp1198:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1199:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp1200:
	.loc	21 332 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp1201:
	.loc	21 323 26
	vmovss	820(%rbx), %xmm10
.Ltmp1202:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm10
.Ltmp1203:
	.loc	21 325 27
	vmovss	808(%rbx), %xmm9
	.loc	21 325 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp1204:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
	jne	.LBB21_46
.Ltmp1205:
	.loc	26 161 24
	jp	.LBB21_46
.Ltmp1206:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_46:
	jne	.LBB21_49
.Ltmp1207:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_49
.Ltmp1208:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_49:
.Ltmp1209:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm10
.Ltmp1210:
	.loc	26 161 24
	jbe	.LBB21_51
.Ltmp1211:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_51:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp1212:
	.loc	26 66 9
	vaddss	%xmm1, %xmm10, %xmm2
.Ltmp1213:
	.loc	26 161 24
	vcmpnltss	%xmm10, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm10, %xmm2, %xmm2
.Ltmp1214:
	.loc	21 332 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp1215:
	.loc	21 323 26
	vmovss	836(%rbx), %xmm13
.Ltmp1216:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1217:
	.loc	21 325 44
	vmovss	832(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	824(%rbx), %xmm10
.Ltmp1218:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm14
	jne	.LBB21_54
.Ltmp1219:
	.loc	26 161 24
	jp	.LBB21_54
.Ltmp1220:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_54:
	jne	.LBB21_57
.Ltmp1221:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_57
.Ltmp1222:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_57:
.Ltmp1223:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp1224:
	.loc	26 161 24
	jbe	.LBB21_59
.Ltmp1225:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm10
.LBB21_59:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm10, 824(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp1226:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1227:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1228:
	.loc	21 332 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp1229:
	.loc	21 323 26
	vmovss	852(%rbx), %xmm14
.Ltmp1230:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1231:
	.loc	21 325 44
	vmovss	848(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp1232:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
	jne	.LBB21_62
.Ltmp1233:
	.loc	26 161 24
	jp	.LBB21_62
.Ltmp1234:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_62:
	jne	.LBB21_65
.Ltmp1235:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_65
.Ltmp1236:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_65:
.Ltmp1237:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp1238:
	.loc	26 161 24
	jbe	.LBB21_67
.Ltmp1239:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_67:
.Ltmp1240:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1241:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp1242:
	.loc	21 326 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp1243:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm3
.Ltmp1244:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm11, %xmm6
.Ltmp1245:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp1246:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp1247:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp1248:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp1249:
	.loc	21 332 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp1250:
	.loc	21 343 23
	vmovss	760(%rbx), %xmm2
.Ltmp1251:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1252:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp1253:
	.loc	21 345 9
	vmovss	764(%rbx), %xmm2
.Ltmp1254:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm11
.Ltmp1255:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm2
.Ltmp1256:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm6, %xmm11, %xmm3
.Ltmp1257:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1258:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp1259:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp1260:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1261:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1262:
	.loc	26 161 24
	movl	$841731191, %ecx
	cmovbel	%ecx, %r8d
.Ltmp1263:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1264:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1265:
	.loc	26 161 24
	movl	$8388608, %ecx
	cmovbel	%ecx, %r8d
.Ltmp1266:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1267:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1268:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1269:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1270:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp1271:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1272:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1273:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1274:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1275:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1276:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1277:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1278:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1279:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp1280:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1281:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp1282:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1283:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1284:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1285:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1286:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm11
.Ltmp1287:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp1288:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm11
.Ltmp1289:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp1290:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm11
.Ltmp1291:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1292:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp1293:
	.loc	21 370 47
	vmovss	860(%rbx), %xmm3
.Ltmp1294:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm3
.Ltmp1295:
	.loc	26 149 9
	notl	%r8d
.Ltmp1296:
	.loc	26 139 9
	cmovbel	%eax, %r8d
.Ltmp1297:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm2
.Ltmp1298:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1299:
	.loc	21 375 9
	vmovss	752(%rbx), %xmm12
.Ltmp1300:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp1301:
	.loc	26 139 9
	cmovbel	%eax, %r8d
.Ltmp1302:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_69
.Ltmp1303:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_69:
.Ltmp1304:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_71
.Ltmp1305:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_71:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm3
.Ltmp1306:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 382 5
	movl	%r8d, 856(%rbx)
.Ltmp1307:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1308:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm11, %xmm6
.Ltmp1309:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm6, %xmm2, %xmm2
.Ltmp1310:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm6
	vxorps	%xmm6, %xmm10, %xmm6
.Ltmp1311:
	.loc	26 161 24
	vmaxss	%xmm6, %xmm2, %xmm2
.Ltmp1312:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm6
	vandps	%xmm6, %xmm3, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_73
.Ltmp1313:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_73:
.Ltmp1314:
	.loc	21 392 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp1315:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp1316:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1317:
	.loc	26 92 9
	vmulss	744(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1318:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1319:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp1320:
	.loc	21 394 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp1321:
	.loc	21 323 26
	vmovss	880(%rbx), %xmm11
.Ltmp1322:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp1323:
	.loc	21 325 44
	vmovss	876(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp1324:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
	jne	.LBB21_76
.Ltmp1325:
	.loc	26 161 24
	jp	.LBB21_76
.Ltmp1326:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_76:
	jne	.LBB21_79
.Ltmp1327:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_79
.Ltmp1328:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_79:
.Ltmp1329:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp1330:
	.loc	26 161 24
	jbe	.LBB21_81
.Ltmp1331:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_81:
.Ltmp1332:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm15, %xmm2
	vmovaps	%xmm2, 160(%rsp)
.Ltmp1333:
	.loc	21 326 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp1334:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1335:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp1336:
	.loc	21 332 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp1337:
	.loc	21 323 26
	vmovss	896(%rbx), %xmm12
.Ltmp1338:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp1339:
	.loc	21 325 44
	vmovss	892(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp1340:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
	jne	.LBB21_84
.Ltmp1341:
	.loc	26 161 24
	jp	.LBB21_84
.Ltmp1342:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_84:
	jne	.LBB21_87
.Ltmp1343:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_87
.Ltmp1344:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_87:
.Ltmp1345:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm12
.Ltmp1346:
	.loc	26 161 24
	jbe	.LBB21_89
.Ltmp1347:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_89:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp1348:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp1349:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp1350:
	.loc	21 332 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp1351:
	.loc	21 323 26
	vmovss	912(%rbx), %xmm13
.Ltmp1352:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1353:
	.loc	21 325 44
	vmovss	908(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp1354:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
	jne	.LBB21_92
.Ltmp1355:
	.loc	26 161 24
	jp	.LBB21_92
.Ltmp1356:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_92:
	jne	.LBB21_95
.Ltmp1357:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_95
.Ltmp1358:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_95:
.Ltmp1359:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp1360:
	.loc	26 161 24
	jbe	.LBB21_97
.Ltmp1361:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_97:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp1362:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1363:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1364:
	.loc	21 332 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp1365:
	.loc	21 323 26
	vmovss	928(%rbx), %xmm14
.Ltmp1366:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1367:
	.loc	21 325 44
	vmovss	924(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp1368:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
	jne	.LBB21_100
.Ltmp1369:
	.loc	26 161 24
	jp	.LBB21_100
.Ltmp1370:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_100:
	jne	.LBB21_103
.Ltmp1371:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_103
.Ltmp1372:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_103:
.Ltmp1373:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp1374:
	.loc	26 161 24
	jbe	.LBB21_105
.Ltmp1375:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_105:
.Ltmp1376:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1377:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp1378:
	.loc	21 326 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp1379:
	.loc	26 103 24
	vandps	192(%rsp), %xmm6, %xmm3
.Ltmp1380:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm7, %xmm6
.Ltmp1381:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp1382:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp1383:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp1384:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp1385:
	.loc	21 332 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp1386:
	.loc	21 343 23
	vmovss	784(%rbx), %xmm2
.Ltmp1387:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1388:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp1389:
	.loc	21 345 9
	vmovss	788(%rbx), %xmm2
.Ltmp1390:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp1391:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp1392:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp1393:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1394:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp1395:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp1396:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1397:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1398:
	.loc	26 161 24
	movl	$841731191, %ecx
	cmovbel	%ecx, %r8d
.Ltmp1399:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1400:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1401:
	.loc	26 161 24
	movl	$8388608, %ecx
	cmovbel	%ecx, %r8d
.Ltmp1402:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1403:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1404:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1405:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1406:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp1407:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1408:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1409:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1410:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1411:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1412:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1413:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1414:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1415:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp1416:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1417:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp1418:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1419:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1420:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1421:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1422:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp1423:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp1424:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp1425:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp1426:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp1427:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1428:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp1429:
	.loc	21 370 47
	vmovss	936(%rbx), %xmm6
.Ltmp1430:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm6
.Ltmp1431:
	.loc	26 149 9
	notl	%r8d
.Ltmp1432:
	.loc	26 139 9
	cmovbel	%eax, %r8d
.Ltmp1433:
	.loc	21 362 20
	vmovss	932(%rbx), %xmm2
.Ltmp1434:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1435:
	.loc	21 375 9
	vmovss	776(%rbx), %xmm3
.Ltmp1436:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp1437:
	.loc	26 139 9
	cmovbel	%eax, %r8d
.Ltmp1438:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_107
.Ltmp1439:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_107:
	movq	16(%rsp), %r12
.Ltmp1440:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_109
.Ltmp1441:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_109:
.Ltmp1442:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp1443:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm13
.Ltmp1444:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp1445:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1446:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp1447:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1448:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp1449:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp1450:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_111
.Ltmp1451:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_111
.LBB21_112:
	cmpq	%r10, %rsi
	jbe	.LBB21_200
	movq	%r10, 56(%rsp)
.Ltmp1452:
	.loc	25 438 16 is_stmt 1
	movq	%r10, %rax
	negq	%rax
	movl	%r12d, %edx
	subl	%r8d, %edx
	negq	%rsi
	movl	$1, %r15d
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$841731191, %r9d
	xorl	%r10d, %r10d
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB21_114
.Ltmp1453:
	.loc	25 0 16 is_stmt 0
.Ltmp1454:
	.p2align	4
.LBB21_199:
	.loc	21 392 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp1455:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp1456:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1457:
	.loc	26 92 9
	vmulss	768(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1458:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1459:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp1460:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp1461:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp1462:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp1463:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp1464:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp1465:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp1466:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm9
.Ltmp1467:
	.loc	26 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp1468:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp1469:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp1470:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp1471:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp1472:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp1473:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp1474:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1475:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm5, %xmm2
.Ltmp1476:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm15, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm5, %xmm2
.Ltmp1477:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp1478:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp1479:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm6
.Ltmp1480:
	.loc	26 161 24
	vblendvps	%xmm6, %xmm2, %xmm5, %xmm2
.Ltmp1481:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm5
.Ltmp1482:
	.loc	26 66 9
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp1483:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp1484:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp1485:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1486:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp1487:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1488:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp1489:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1490:
	.loc	26 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1491:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1492:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp1493:
	.loc	26 178 22
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp1494:
	.loc	7 1244 18
	vmovd	%xmm5, %r8d
.Ltmp1495:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp1496:
	.loc	7 1291 18
	vmovd	%r8d, %xmm5
.Ltmp1497:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm3
.Ltmp1498:
	.loc	21 394 5
	vmovss	%xmm10, 940(%rbx)
.Ltmp1499:
	.loc	26 71 9
	vmulss	%xmm3, %xmm4, %xmm3
.Ltmp1500:
	.loc	26 161 24
	vcmpneqss	%xmm15, %xmm10, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	vcmpnltss	780(%rbx), %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	movq	240(%rsp), %r13
.Ltmp1501:
	.loc	26 56 9
	vmovss	%xmm2, -4(%r13,%r15,4)
	movq	248(%rsp), %rcx
.Ltmp1502:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rcx,%r15,4)
.Ltmp1503:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rsi,%r15), %r8
	incq	%r8
	incq	%r15
	cmpq	$1, %r8
	movq	176(%rsp), %rbp
.Ltmp1504:
	.loc	11 900 12
	je	.LBB21_374
.Ltmp1505:
.LBB21_114:
	.loc	21 246 22
	leal	(%r12,%r15), %r8d
	decl	%r8d
	andl	%edi, %r8d
.Ltmp1506:
	.loc	25 451 16
	cmpq	%r8, %rbp
	jbe	.LBB21_462
.Ltmp1507:
	.loc	21 0 0 is_stmt 0
	leaq	(%rax,%r15), %r12
.Ltmp1508:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r13,%r15,4), %xmm2
	movq	8(%rsp), %r13
.Ltmp1509:
	.loc	26 56 9
	vmovss	%xmm2, (%r13,%r8,4)
.Ltmp1510:
	.loc	26 51 9
	vmovss	-4(%rcx,%r15,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp1511:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%r8,4)
.Ltmp1512:
	.loc	25 438 16
	cmpq	$1, %r12
	je	.LBB21_464
.Ltmp1513:
	.loc	25 451 16
	cmpq	%r8, (%rsp)
	jbe	.LBB21_465
.Ltmp1514:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %rcx
.Ltmp1515:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r15,4), %xmm2
.Ltmp1516:
	.loc	26 56 9
	vmovss	%xmm2, (%r11,%r8,4)
	movq	32(%rsp), %rcx
.Ltmp1517:
	.loc	25 451 16
	cmpq	%r8, %rcx
	jbe	.LBB21_466
.Ltmp1518:
	.loc	25 0 16 is_stmt 0
	movq	48(%rsp), %r12
.Ltmp1519:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
.Ltmp1520:
	.loc	26 56 9
	vmovss	%xmm2, (%r14,%r8,4)
.Ltmp1521:
	.loc	21 255 21
	leal	(%rdx,%r15), %r12d
	decl	%r12d
	andl	%edi, %r12d
.Ltmp1522:
	.loc	25 438 16
	cmpq	%r12, %rbp
	jbe	.LBB21_467
.Ltmp1523:
	.loc	25 0 16 is_stmt 0
	movq	16(%rsp), %r8
	addl	%r15d, %r8d
	movl	136(%rbx), %r13d
	notl	%r13d
	addl	%r8d, %r13d
	andl	%edi, %r13d
	cmpq	%r13, (%rsp)
	jbe	.LBB21_473
	cmpq	%r13, %rcx
	jbe	.LBB21_472
	movl	200(%rbx), %ebp
	notl	%ebp
	addl	%r8d, %ebp
	andl	%edi, %ebp
	cmpq	%rbp, %rcx
	jbe	.LBB21_471
	cmpq	%rbp, (%rsp)
	jbe	.LBB21_470
.Ltmp1524:
	.loc	21 323 26 is_stmt 1
	vmovss	804(%rbx), %xmm9
.Ltmp1525:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp1526:
	.loc	21 325 44
	vmovss	800(%rbx), %xmm10
	.loc	21 325 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp1527:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm8, %xmm3
	jne	.LBB21_126
.Ltmp1528:
	.loc	26 161 24
	jp	.LBB21_126
.Ltmp1529:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm3
.LBB21_126:
	jne	.LBB21_129
.Ltmp1530:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_129
.Ltmp1531:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm10, %xmm10, %xmm10
.LBB21_129:
.Ltmp1532:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm9
.Ltmp1533:
	.loc	26 161 24
	jbe	.LBB21_131
.Ltmp1534:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm8
.LBB21_131:
	movq	8(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm5
	movq	24(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm4
	vmovss	(%r14,%r13,4), %xmm11
.Ltmp1535:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r11,%r13,4), %xmm12
.Ltmp1536:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r14,%rbp,4), %xmm2
	vmovaps	%xmm2, 192(%rsp)
.Ltmp1537:
	.loc	26 103 24
	vmovss	(%r11,%rbp,4), %xmm7
.Ltmp1538:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 331 13
	vmovss	%xmm10, 800(%rbx)
.Ltmp1539:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1540:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp1541:
	.loc	21 332 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp1542:
	.loc	21 323 26
	vmovss	820(%rbx), %xmm10
.Ltmp1543:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm10
.Ltmp1544:
	.loc	21 325 27
	vmovss	808(%rbx), %xmm9
	.loc	21 325 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp1545:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
	jne	.LBB21_134
.Ltmp1546:
	.loc	26 161 24
	jp	.LBB21_134
.Ltmp1547:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_134:
	jne	.LBB21_137
.Ltmp1548:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_137
.Ltmp1549:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_137:
.Ltmp1550:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm10
.Ltmp1551:
	.loc	26 161 24
	jbe	.LBB21_139
.Ltmp1552:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_139:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp1553:
	.loc	26 66 9
	vaddss	%xmm1, %xmm10, %xmm2
.Ltmp1554:
	.loc	26 161 24
	vcmpnltss	%xmm10, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm10, %xmm2, %xmm2
.Ltmp1555:
	.loc	21 332 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp1556:
	.loc	21 323 26
	vmovss	836(%rbx), %xmm13
.Ltmp1557:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1558:
	.loc	21 325 44
	vmovss	832(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	824(%rbx), %xmm10
.Ltmp1559:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm14
	jne	.LBB21_142
.Ltmp1560:
	.loc	26 161 24
	jp	.LBB21_142
.Ltmp1561:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_142:
	jne	.LBB21_145
.Ltmp1562:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_145
.Ltmp1563:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_145:
.Ltmp1564:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp1565:
	.loc	26 161 24
	jbe	.LBB21_147
.Ltmp1566:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm10
.LBB21_147:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm10, 824(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp1567:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1568:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1569:
	.loc	21 332 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp1570:
	.loc	21 323 26
	vmovss	852(%rbx), %xmm14
.Ltmp1571:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1572:
	.loc	21 325 44
	vmovss	848(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp1573:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
	jne	.LBB21_150
.Ltmp1574:
	.loc	26 161 24
	jp	.LBB21_150
.Ltmp1575:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_150:
	jne	.LBB21_153
.Ltmp1576:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_153
.Ltmp1577:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_153:
.Ltmp1578:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp1579:
	.loc	26 161 24
	jbe	.LBB21_155
.Ltmp1580:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_155:
.Ltmp1581:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1582:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp1583:
	.loc	21 326 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp1584:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm3
.Ltmp1585:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm11, %xmm6
.Ltmp1586:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp1587:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp1588:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp1589:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp1590:
	.loc	21 332 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp1591:
	.loc	21 343 23
	vmovss	760(%rbx), %xmm2
.Ltmp1592:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1593:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp1594:
	.loc	21 345 9
	vmovss	764(%rbx), %xmm2
.Ltmp1595:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm11
.Ltmp1596:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm2
.Ltmp1597:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm6, %xmm11, %xmm3
.Ltmp1598:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1599:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp1600:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp1601:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1602:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1603:
	.loc	26 161 24
	cmovbel	%r9d, %r8d
.Ltmp1604:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1605:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1606:
	.loc	26 161 24
	movl	$8388608, %ecx
	cmovbel	%ecx, %r8d
.Ltmp1607:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1608:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1609:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1610:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1611:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp1612:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1613:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1614:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1615:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1616:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1617:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1618:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1619:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1620:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp1621:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1622:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp1623:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1624:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1625:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1626:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1627:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm11
.Ltmp1628:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp1629:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm11
.Ltmp1630:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp1631:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm11
.Ltmp1632:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1633:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp1634:
	.loc	21 370 47
	vmovss	860(%rbx), %xmm3
.Ltmp1635:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm3
.Ltmp1636:
	.loc	26 149 9
	notl	%r8d
.Ltmp1637:
	.loc	26 139 9
	cmovbel	%r10d, %r8d
.Ltmp1638:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm2
.Ltmp1639:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1640:
	.loc	21 375 9
	vmovss	752(%rbx), %xmm12
.Ltmp1641:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp1642:
	.loc	26 139 9
	cmovbel	%r10d, %r8d
.Ltmp1643:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_157
.Ltmp1644:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_157:
.Ltmp1645:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_159
.Ltmp1646:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_159:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm3
.Ltmp1647:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 382 5
	movl	%r8d, 856(%rbx)
.Ltmp1648:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1649:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm11, %xmm6
.Ltmp1650:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm6, %xmm2, %xmm2
.Ltmp1651:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm6
	vxorps	%xmm6, %xmm10, %xmm6
.Ltmp1652:
	.loc	26 161 24
	vmaxss	%xmm6, %xmm2, %xmm2
.Ltmp1653:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm6
	vandps	%xmm6, %xmm3, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_161
.Ltmp1654:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_161:
.Ltmp1655:
	.loc	21 392 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp1656:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp1657:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1658:
	.loc	26 92 9
	vmulss	744(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1659:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1660:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp1661:
	.loc	21 394 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp1662:
	.loc	21 323 26
	vmovss	880(%rbx), %xmm11
.Ltmp1663:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp1664:
	.loc	21 325 44
	vmovss	876(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp1665:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
	jne	.LBB21_164
.Ltmp1666:
	.loc	26 161 24
	jp	.LBB21_164
.Ltmp1667:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_164:
	jne	.LBB21_167
.Ltmp1668:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_167
.Ltmp1669:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_167:
.Ltmp1670:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp1671:
	.loc	26 161 24
	jbe	.LBB21_169
.Ltmp1672:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_169:
.Ltmp1673:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm15, %xmm2
	vmovaps	%xmm2, 160(%rsp)
.Ltmp1674:
	.loc	21 326 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp1675:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1676:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp1677:
	.loc	21 332 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp1678:
	.loc	21 323 26
	vmovss	896(%rbx), %xmm12
.Ltmp1679:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp1680:
	.loc	21 325 44
	vmovss	892(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp1681:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
	jne	.LBB21_172
.Ltmp1682:
	.loc	26 161 24
	jp	.LBB21_172
.Ltmp1683:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_172:
	jne	.LBB21_175
.Ltmp1684:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_175
.Ltmp1685:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_175:
.Ltmp1686:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm12
.Ltmp1687:
	.loc	26 161 24
	jbe	.LBB21_177
.Ltmp1688:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_177:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp1689:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp1690:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp1691:
	.loc	21 332 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp1692:
	.loc	21 323 26
	vmovss	912(%rbx), %xmm13
.Ltmp1693:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1694:
	.loc	21 325 44
	vmovss	908(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp1695:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
	jne	.LBB21_180
.Ltmp1696:
	.loc	26 161 24
	jp	.LBB21_180
.Ltmp1697:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_180:
	jne	.LBB21_183
.Ltmp1698:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_183
.Ltmp1699:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_183:
.Ltmp1700:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp1701:
	.loc	26 161 24
	jbe	.LBB21_185
.Ltmp1702:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_185:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp1703:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1704:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1705:
	.loc	21 332 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp1706:
	.loc	21 323 26
	vmovss	928(%rbx), %xmm14
.Ltmp1707:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1708:
	.loc	21 325 44
	vmovss	924(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp1709:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
	jne	.LBB21_188
.Ltmp1710:
	.loc	26 161 24
	jp	.LBB21_188
.Ltmp1711:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_188:
	jne	.LBB21_191
.Ltmp1712:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_191
.Ltmp1713:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_191:
.Ltmp1714:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp1715:
	.loc	26 161 24
	jbe	.LBB21_193
.Ltmp1716:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_193:
.Ltmp1717:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1718:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp1719:
	.loc	21 326 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp1720:
	.loc	26 103 24
	vandps	192(%rsp), %xmm6, %xmm3
.Ltmp1721:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm7, %xmm6
.Ltmp1722:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp1723:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp1724:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp1725:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp1726:
	.loc	21 332 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp1727:
	.loc	21 343 23
	vmovss	784(%rbx), %xmm2
.Ltmp1728:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1729:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp1730:
	.loc	21 345 9
	vmovss	788(%rbx), %xmm2
.Ltmp1731:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp1732:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp1733:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp1734:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1735:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp1736:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp1737:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1738:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1739:
	.loc	26 161 24
	cmovbel	%r9d, %r8d
.Ltmp1740:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1741:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1742:
	.loc	26 161 24
	cmovbel	%ecx, %r8d
.Ltmp1743:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1744:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1745:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1746:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1747:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp1748:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1749:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1750:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1751:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1752:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1753:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1754:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1755:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1756:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp1757:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1758:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp1759:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1760:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1761:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1762:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1763:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp1764:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp1765:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp1766:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp1767:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp1768:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1769:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp1770:
	.loc	21 370 47
	vmovss	936(%rbx), %xmm6
.Ltmp1771:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm6
.Ltmp1772:
	.loc	26 149 9
	notl	%r8d
.Ltmp1773:
	.loc	26 139 9
	cmovbel	%r10d, %r8d
.Ltmp1774:
	.loc	21 362 20
	vmovss	932(%rbx), %xmm2
.Ltmp1775:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1776:
	.loc	21 375 9
	vmovss	776(%rbx), %xmm3
.Ltmp1777:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp1778:
	.loc	26 139 9
	cmovbel	%r10d, %r8d
.Ltmp1779:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_195
.Ltmp1780:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_195:
	movq	16(%rsp), %r12
.Ltmp1781:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_197
.Ltmp1782:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_197:
.Ltmp1783:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp1784:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm13
.Ltmp1785:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp1786:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp1787:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp1788:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1789:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp1790:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp1791:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_199
.Ltmp1792:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_199
.LBB21_200:
	cmpq	(%rsp), %rbp
	jbe	.LBB21_288
.Ltmp1793:
	.loc	25 438 16 is_stmt 1
	movq	%rsi, %rax
	negq	%rax
	movl	%r12d, %edx
	subl	%r8d, %edx
	movl	$1, %r15d
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$841731191, %r9d
	movl	$8388608, %r10d
	xorl	%esi, %esi
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB21_202
.Ltmp1794:
	.loc	25 0 16 is_stmt 0
.Ltmp1795:
	.p2align	4
.LBB21_287:
	.loc	21 392 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp1796:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp1797:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1798:
	.loc	26 92 9
	vmulss	768(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1799:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp1800:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp1801:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp1802:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp1803:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp1804:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp1805:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp1806:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp1807:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm9
.Ltmp1808:
	.loc	26 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp1809:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp1810:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp1811:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp1812:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp1813:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp1814:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp1815:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1816:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm5, %xmm2
.Ltmp1817:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm15, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm5, %xmm2
.Ltmp1818:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp1819:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp1820:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm6
.Ltmp1821:
	.loc	26 161 24
	vblendvps	%xmm6, %xmm2, %xmm5, %xmm2
.Ltmp1822:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm5
.Ltmp1823:
	.loc	26 66 9
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp1824:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp1825:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp1826:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1827:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp1828:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1829:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp1830:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp1831:
	.loc	26 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp1832:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp1833:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp1834:
	.loc	26 178 22
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp1835:
	.loc	7 1244 18
	vmovd	%xmm5, %r8d
.Ltmp1836:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp1837:
	.loc	7 1291 18
	vmovd	%r8d, %xmm5
.Ltmp1838:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm3
.Ltmp1839:
	.loc	21 394 5
	vmovss	%xmm10, 940(%rbx)
.Ltmp1840:
	.loc	26 71 9
	vmulss	%xmm3, %xmm4, %xmm3
.Ltmp1841:
	.loc	26 161 24
	vcmpneqss	%xmm15, %xmm10, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	vcmpnltss	780(%rbx), %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	movq	240(%rsp), %r13
.Ltmp1842:
	.loc	26 56 9
	vmovss	%xmm2, -4(%r13,%r15,4)
	movq	248(%rsp), %rcx
.Ltmp1843:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rcx,%r15,4)
.Ltmp1844:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rax,%r15), %r8
	incq	%r8
	incq	%r15
	cmpq	$1, %r8
	movq	176(%rsp), %rbp
.Ltmp1845:
	.loc	11 900 12
	je	.LBB21_374
.LBB21_202:
.Ltmp1846:
	.loc	15 971 17
	leaq	(%rax,%r15), %r8
.Ltmp1847:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_460
.Ltmp1848:
	.loc	21 0 0 is_stmt 0
	leal	(%r12,%r15), %r8d
	decl	%r8d
	andl	%edi, %r8d
.Ltmp1849:
	.loc	25 451 16 is_stmt 1
	cmpq	%r8, %rbp
	jbe	.LBB21_462
.Ltmp1850:
	.loc	26 51 9
	vmovss	-4(%r13,%r15,4), %xmm2
	movq	8(%rsp), %r12
.Ltmp1851:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%r8,4)
.Ltmp1852:
	.loc	26 51 9
	vmovss	-4(%rcx,%r15,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp1853:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%r8,4)
.Ltmp1854:
	.loc	25 451 16
	cmpq	%r8, (%rsp)
	jbe	.LBB21_465
.Ltmp1855:
	.loc	25 0 16 is_stmt 0
	movq	40(%rsp), %rcx
.Ltmp1856:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rcx,%r15,4), %xmm2
.Ltmp1857:
	.loc	26 56 9
	vmovss	%xmm2, (%r11,%r8,4)
	movq	32(%rsp), %rcx
.Ltmp1858:
	.loc	25 451 16
	cmpq	%r8, %rcx
	jbe	.LBB21_466
.Ltmp1859:
	.loc	25 0 16 is_stmt 0
	movq	48(%rsp), %r12
.Ltmp1860:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
.Ltmp1861:
	.loc	26 56 9
	vmovss	%xmm2, (%r14,%r8,4)
.Ltmp1862:
	.loc	21 255 21
	leal	(%rdx,%r15), %r12d
	decl	%r12d
	andl	%edi, %r12d
.Ltmp1863:
	.loc	25 438 16
	cmpq	%r12, %rbp
	jbe	.LBB21_467
.Ltmp1864:
	.loc	25 0 16 is_stmt 0
	movq	16(%rsp), %r8
	addl	%r15d, %r8d
	movl	136(%rbx), %r13d
	notl	%r13d
	addl	%r8d, %r13d
	andl	%edi, %r13d
	cmpq	%r13, (%rsp)
	jbe	.LBB21_473
	cmpq	%r13, %rcx
	jbe	.LBB21_472
	movl	200(%rbx), %ebp
	notl	%ebp
	addl	%r8d, %ebp
	andl	%edi, %ebp
	cmpq	%rbp, %rcx
	jbe	.LBB21_471
	cmpq	%rbp, (%rsp)
	jbe	.LBB21_470
.Ltmp1865:
	.loc	21 323 26 is_stmt 1
	vmovss	804(%rbx), %xmm9
.Ltmp1866:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp1867:
	.loc	21 325 44
	vmovss	800(%rbx), %xmm10
	.loc	21 325 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp1868:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm8, %xmm3
	jne	.LBB21_214
.Ltmp1869:
	.loc	26 161 24
	jp	.LBB21_214
.Ltmp1870:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm3
.LBB21_214:
	jne	.LBB21_217
.Ltmp1871:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_217
.Ltmp1872:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm10, %xmm10, %xmm10
.LBB21_217:
.Ltmp1873:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm9
.Ltmp1874:
	.loc	26 161 24
	jbe	.LBB21_219
.Ltmp1875:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm8
.LBB21_219:
	movq	8(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm5
	movq	24(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm4
	vmovss	(%r14,%r13,4), %xmm11
.Ltmp1876:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r11,%r13,4), %xmm12
.Ltmp1877:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r14,%rbp,4), %xmm2
	vmovaps	%xmm2, 192(%rsp)
.Ltmp1878:
	.loc	26 103 24
	vmovss	(%r11,%rbp,4), %xmm7
.Ltmp1879:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 331 13
	vmovss	%xmm10, 800(%rbx)
.Ltmp1880:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1881:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp1882:
	.loc	21 332 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp1883:
	.loc	21 323 26
	vmovss	820(%rbx), %xmm10
.Ltmp1884:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm10
.Ltmp1885:
	.loc	21 325 27
	vmovss	808(%rbx), %xmm9
	.loc	21 325 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp1886:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
	jne	.LBB21_222
.Ltmp1887:
	.loc	26 161 24
	jp	.LBB21_222
.Ltmp1888:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_222:
	jne	.LBB21_225
.Ltmp1889:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_225
.Ltmp1890:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_225:
.Ltmp1891:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm10
.Ltmp1892:
	.loc	26 161 24
	jbe	.LBB21_227
.Ltmp1893:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_227:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp1894:
	.loc	26 66 9
	vaddss	%xmm1, %xmm10, %xmm2
.Ltmp1895:
	.loc	26 161 24
	vcmpnltss	%xmm10, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm10, %xmm2, %xmm2
.Ltmp1896:
	.loc	21 332 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp1897:
	.loc	21 323 26
	vmovss	836(%rbx), %xmm13
.Ltmp1898:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp1899:
	.loc	21 325 44
	vmovss	832(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	824(%rbx), %xmm10
.Ltmp1900:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm14
	jne	.LBB21_230
.Ltmp1901:
	.loc	26 161 24
	jp	.LBB21_230
.Ltmp1902:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_230:
	jne	.LBB21_233
.Ltmp1903:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_233
.Ltmp1904:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_233:
.Ltmp1905:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp1906:
	.loc	26 161 24
	jbe	.LBB21_235
.Ltmp1907:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm10
.LBB21_235:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm10, 824(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp1908:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp1909:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp1910:
	.loc	21 332 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp1911:
	.loc	21 323 26
	vmovss	852(%rbx), %xmm14
.Ltmp1912:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp1913:
	.loc	21 325 44
	vmovss	848(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp1914:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
	jne	.LBB21_238
.Ltmp1915:
	.loc	26 161 24
	jp	.LBB21_238
.Ltmp1916:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_238:
	jne	.LBB21_241
.Ltmp1917:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_241
.Ltmp1918:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_241:
.Ltmp1919:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp1920:
	.loc	26 161 24
	jbe	.LBB21_243
.Ltmp1921:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_243:
.Ltmp1922:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp1923:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp1924:
	.loc	21 326 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp1925:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm3
.Ltmp1926:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm11, %xmm6
.Ltmp1927:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp1928:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp1929:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp1930:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp1931:
	.loc	21 332 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp1932:
	.loc	21 343 23
	vmovss	760(%rbx), %xmm2
.Ltmp1933:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1934:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp1935:
	.loc	21 345 9
	vmovss	764(%rbx), %xmm2
.Ltmp1936:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm11
.Ltmp1937:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm2
.Ltmp1938:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm6, %xmm11, %xmm3
.Ltmp1939:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1940:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp1941:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp1942:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1943:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp1944:
	.loc	26 161 24
	cmovbel	%r9d, %r8d
.Ltmp1945:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp1946:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp1947:
	.loc	26 161 24
	cmovbel	%r10d, %r8d
.Ltmp1948:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp1949:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp1950:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp1951:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp1952:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp1953:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1954:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp1955:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1956:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp1957:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1958:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp1959:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp1960:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp1961:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp1962:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp1963:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp1964:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp1965:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp1966:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp1967:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp1968:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm11
.Ltmp1969:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp1970:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm11
.Ltmp1971:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp1972:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm11
.Ltmp1973:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp1974:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp1975:
	.loc	21 370 47
	vmovss	860(%rbx), %xmm3
.Ltmp1976:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm3
.Ltmp1977:
	.loc	26 149 9
	notl	%r8d
.Ltmp1978:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp1979:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm2
.Ltmp1980:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp1981:
	.loc	21 375 9
	vmovss	752(%rbx), %xmm12
.Ltmp1982:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp1983:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp1984:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_245
.Ltmp1985:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_245:
.Ltmp1986:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_247
.Ltmp1987:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_247:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm3
.Ltmp1988:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 382 5
	movl	%r8d, 856(%rbx)
.Ltmp1989:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp1990:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm11, %xmm6
.Ltmp1991:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm6, %xmm2, %xmm2
.Ltmp1992:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm6
	vxorps	%xmm6, %xmm10, %xmm6
.Ltmp1993:
	.loc	26 161 24
	vmaxss	%xmm6, %xmm2, %xmm2
.Ltmp1994:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm6
	vandps	%xmm6, %xmm3, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_249
.Ltmp1995:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_249:
.Ltmp1996:
	.loc	21 392 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp1997:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp1998:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp1999:
	.loc	26 92 9
	vmulss	744(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2000:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2001:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp2002:
	.loc	21 394 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp2003:
	.loc	21 323 26
	vmovss	880(%rbx), %xmm11
.Ltmp2004:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp2005:
	.loc	21 325 44
	vmovss	876(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp2006:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
	jne	.LBB21_252
.Ltmp2007:
	.loc	26 161 24
	jp	.LBB21_252
.Ltmp2008:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_252:
	jne	.LBB21_255
.Ltmp2009:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_255
.Ltmp2010:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_255:
.Ltmp2011:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp2012:
	.loc	26 161 24
	jbe	.LBB21_257
.Ltmp2013:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_257:
.Ltmp2014:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm15, %xmm2
	vmovaps	%xmm2, 160(%rsp)
.Ltmp2015:
	.loc	21 326 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp2016:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2017:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp2018:
	.loc	21 332 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp2019:
	.loc	21 323 26
	vmovss	896(%rbx), %xmm12
.Ltmp2020:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp2021:
	.loc	21 325 44
	vmovss	892(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp2022:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
	jne	.LBB21_260
.Ltmp2023:
	.loc	26 161 24
	jp	.LBB21_260
.Ltmp2024:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_260:
	jne	.LBB21_263
.Ltmp2025:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_263
.Ltmp2026:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_263:
.Ltmp2027:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm12
.Ltmp2028:
	.loc	26 161 24
	jbe	.LBB21_265
.Ltmp2029:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_265:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp2030:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp2031:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp2032:
	.loc	21 332 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp2033:
	.loc	21 323 26
	vmovss	912(%rbx), %xmm13
.Ltmp2034:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2035:
	.loc	21 325 44
	vmovss	908(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp2036:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
	jne	.LBB21_268
.Ltmp2037:
	.loc	26 161 24
	jp	.LBB21_268
.Ltmp2038:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_268:
	jne	.LBB21_271
.Ltmp2039:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_271
.Ltmp2040:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_271:
.Ltmp2041:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp2042:
	.loc	26 161 24
	jbe	.LBB21_273
.Ltmp2043:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_273:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp2044:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2045:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2046:
	.loc	21 332 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp2047:
	.loc	21 323 26
	vmovss	928(%rbx), %xmm14
.Ltmp2048:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2049:
	.loc	21 325 44
	vmovss	924(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp2050:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
	jne	.LBB21_276
.Ltmp2051:
	.loc	26 161 24
	jp	.LBB21_276
.Ltmp2052:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_276:
	jne	.LBB21_279
.Ltmp2053:
	.loc	26 161 44 is_stmt 1
	jp	.LBB21_279
.Ltmp2054:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_279:
.Ltmp2055:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp2056:
	.loc	26 161 24
	jbe	.LBB21_281
.Ltmp2057:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_281:
.Ltmp2058:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2059:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp2060:
	.loc	21 326 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp2061:
	.loc	26 103 24
	vandps	192(%rsp), %xmm6, %xmm3
.Ltmp2062:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm7, %xmm6
.Ltmp2063:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp2064:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp2065:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp2066:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp2067:
	.loc	21 332 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp2068:
	.loc	21 343 23
	vmovss	784(%rbx), %xmm2
.Ltmp2069:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2070:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp2071:
	.loc	21 345 9
	vmovss	788(%rbx), %xmm2
.Ltmp2072:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp2073:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp2074:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp2075:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2076:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp2077:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp2078:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2079:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2080:
	.loc	26 161 24
	cmovbel	%r9d, %r8d
.Ltmp2081:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2082:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2083:
	.loc	26 161 24
	cmovbel	%r10d, %r8d
.Ltmp2084:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2085:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2086:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2087:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2088:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp2089:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2090:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2091:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2092:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2093:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2094:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2095:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2096:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2097:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp2098:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2099:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp2100:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2101:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2102:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2103:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2104:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp2105:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp2106:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp2107:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2108:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp2109:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2110:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp2111:
	.loc	21 370 47
	vmovss	936(%rbx), %xmm6
.Ltmp2112:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm6
.Ltmp2113:
	.loc	26 149 9
	notl	%r8d
.Ltmp2114:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2115:
	.loc	21 362 20
	vmovss	932(%rbx), %xmm2
.Ltmp2116:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2117:
	.loc	21 375 9
	vmovss	776(%rbx), %xmm3
.Ltmp2118:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2119:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2120:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_283
.Ltmp2121:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_283:
	movq	16(%rsp), %r12
.Ltmp2122:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_285
.Ltmp2123:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_285:
.Ltmp2124:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp2125:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm13
.Ltmp2126:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp2127:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2128:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp2129:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2130:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp2131:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp2132:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_287
.Ltmp2133:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_287
.LBB21_288:
	cmpq	32(%rsp), %rbp
	jbe	.LBB21_289
.Ltmp2134:
	.loc	25 438 16 is_stmt 1
	movq	%rsi, %rax
	negq	%rax
	movl	%r12d, %edx
	subl	%r8d, %edx
	movl	$1, %r15d
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$841731191, %r9d
	movl	$8388608, %r10d
	xorl	%esi, %esi
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB21_376
.Ltmp2135:
	.loc	25 0 16 is_stmt 0
.Ltmp2136:
	.p2align	4
.LBB21_459:
	.loc	21 392 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp2137:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp2138:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2139:
	.loc	26 92 9
	vmulss	768(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2140:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2141:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp2142:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp2143:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp2144:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp2145:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp2146:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp2147:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp2148:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm9
.Ltmp2149:
	.loc	26 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp2150:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp2151:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp2152:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp2153:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp2154:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp2155:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp2156:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2157:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm5, %xmm2
.Ltmp2158:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm15, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm5, %xmm2
.Ltmp2159:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp2160:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp2161:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm6
.Ltmp2162:
	.loc	26 161 24
	vblendvps	%xmm6, %xmm2, %xmm5, %xmm2
.Ltmp2163:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm5
.Ltmp2164:
	.loc	26 66 9
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp2165:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp2166:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp2167:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2168:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2169:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2170:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp2171:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2172:
	.loc	26 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2173:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp2174:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp2175:
	.loc	26 178 22
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp2176:
	.loc	7 1244 18
	vmovd	%xmm5, %r8d
.Ltmp2177:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp2178:
	.loc	7 1291 18
	vmovd	%r8d, %xmm5
.Ltmp2179:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm3
.Ltmp2180:
	.loc	21 394 5
	vmovss	%xmm10, 940(%rbx)
.Ltmp2181:
	.loc	26 71 9
	vmulss	%xmm3, %xmm4, %xmm3
.Ltmp2182:
	.loc	26 161 24
	vcmpneqss	%xmm15, %xmm10, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	vcmpnltss	780(%rbx), %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	movq	240(%rsp), %r13
.Ltmp2183:
	.loc	26 56 9
	vmovss	%xmm2, -4(%r13,%r15,4)
	movq	248(%rsp), %rcx
.Ltmp2184:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rcx,%r15,4)
.Ltmp2185:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rax,%r15), %r8
	incq	%r8
	incq	%r15
	cmpq	$1, %r8
	movq	176(%rsp), %rbp
.Ltmp2186:
	.loc	11 900 12
	je	.LBB21_374
.LBB21_376:
.Ltmp2187:
	.loc	15 971 17
	leaq	(%rax,%r15), %r8
.Ltmp2188:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_460
.Ltmp2189:
	.loc	21 0 0 is_stmt 0
	leal	(%r12,%r15), %r8d
	decl	%r8d
	andl	%edi, %r8d
.Ltmp2190:
	.loc	25 451 16 is_stmt 1
	cmpq	%r8, %rbp
	jbe	.LBB21_462
.Ltmp2191:
	.loc	26 51 9
	vmovss	-4(%r13,%r15,4), %xmm2
	movq	8(%rsp), %r12
.Ltmp2192:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%r8,4)
.Ltmp2193:
	.loc	26 51 9
	vmovss	-4(%rcx,%r15,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp2194:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%r8,4)
	movq	40(%rsp), %rcx
.Ltmp2195:
	.loc	26 51 9
	vmovss	-4(%rcx,%r15,4), %xmm2
.Ltmp2196:
	.loc	26 56 9
	vmovss	%xmm2, (%r11,%r8,4)
	movq	32(%rsp), %rcx
.Ltmp2197:
	.loc	25 451 16
	cmpq	%r8, %rcx
	jbe	.LBB21_466
.Ltmp2198:
	.loc	25 0 16 is_stmt 0
	movq	48(%rsp), %r12
.Ltmp2199:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r15,4), %xmm2
.Ltmp2200:
	.loc	26 56 9
	vmovss	%xmm2, (%r14,%r8,4)
.Ltmp2201:
	.loc	21 255 21
	leal	(%rdx,%r15), %r12d
	decl	%r12d
	andl	%edi, %r12d
.Ltmp2202:
	.loc	25 438 16
	cmpq	%r12, %rbp
	jbe	.LBB21_467
.Ltmp2203:
	.loc	25 0 16 is_stmt 0
	movq	16(%rsp), %r8
	addl	%r15d, %r8d
	movl	136(%rbx), %r13d
	notl	%r13d
	addl	%r8d, %r13d
	andl	%edi, %r13d
	cmpq	%r13, (%rsp)
	jbe	.LBB21_473
	cmpq	%r13, %rcx
.Ltmp2204:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB21_472
.Ltmp2205:
	.loc	21 0 29 is_stmt 0
	movl	200(%rbx), %ebp
	notl	%ebp
	addl	%r8d, %ebp
	andl	%edi, %ebp
	cmpq	%rbp, %rcx
	jbe	.LBB21_471
.Ltmp2206:
	.loc	21 323 26 is_stmt 1
	vmovss	804(%rbx), %xmm9
.Ltmp2207:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp2208:
	.loc	21 325 44
	vmovss	800(%rbx), %xmm10
	.loc	21 325 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp2209:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm8, %xmm3
.Ltmp2210:
	.loc	26 161 24
	jne	.LBB21_386
	jp	.LBB21_386
.Ltmp2211:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm3
.LBB21_386:
.Ltmp2212:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_389
	jp	.LBB21_389
.Ltmp2213:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm10, %xmm10, %xmm10
.LBB21_389:
.Ltmp2214:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm9
.Ltmp2215:
	.loc	26 161 24
	jbe	.LBB21_391
.Ltmp2216:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm8
.LBB21_391:
	movq	8(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm5
	movq	24(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm4
.Ltmp2217:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r11,%r13,4), %xmm11
.Ltmp2218:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r14,%r13,4), %xmm12
.Ltmp2219:
	.loc	26 103 24
	vmovss	(%r14,%rbp,4), %xmm2
	vmovaps	%xmm2, 192(%rsp)
.Ltmp2220:
	.loc	26 103 24
	vmovss	(%r11,%rbp,4), %xmm7
.Ltmp2221:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 331 13
	vmovss	%xmm10, 800(%rbx)
.Ltmp2222:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2223:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp2224:
	.loc	21 332 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp2225:
	.loc	21 323 26
	vmovss	820(%rbx), %xmm10
.Ltmp2226:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm10
.Ltmp2227:
	.loc	21 325 27
	vmovss	808(%rbx), %xmm9
	.loc	21 325 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp2228:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp2229:
	.loc	26 161 24
	jne	.LBB21_394
	jp	.LBB21_394
.Ltmp2230:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_394:
.Ltmp2231:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_397
	jp	.LBB21_397
.Ltmp2232:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_397:
.Ltmp2233:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm10
.Ltmp2234:
	.loc	26 161 24
	jbe	.LBB21_399
.Ltmp2235:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_399:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp2236:
	.loc	26 66 9
	vaddss	%xmm1, %xmm10, %xmm2
.Ltmp2237:
	.loc	26 161 24
	vcmpnltss	%xmm10, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm10, %xmm2, %xmm2
.Ltmp2238:
	.loc	21 332 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp2239:
	.loc	21 323 26
	vmovss	836(%rbx), %xmm13
.Ltmp2240:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2241:
	.loc	21 325 44
	vmovss	832(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	824(%rbx), %xmm10
.Ltmp2242:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm14
.Ltmp2243:
	.loc	26 161 24
	jne	.LBB21_402
	jp	.LBB21_402
.Ltmp2244:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_402:
.Ltmp2245:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_405
	jp	.LBB21_405
.Ltmp2246:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_405:
.Ltmp2247:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp2248:
	.loc	26 161 24
	jbe	.LBB21_407
.Ltmp2249:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm10
.LBB21_407:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm10, 824(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp2250:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2251:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2252:
	.loc	21 332 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp2253:
	.loc	21 323 26
	vmovss	852(%rbx), %xmm14
.Ltmp2254:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2255:
	.loc	21 325 44
	vmovss	848(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp2256:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2257:
	.loc	26 161 24
	jne	.LBB21_410
	jp	.LBB21_410
.Ltmp2258:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_410:
.Ltmp2259:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_413
	jp	.LBB21_413
.Ltmp2260:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_413:
.Ltmp2261:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp2262:
	.loc	26 161 24
	jbe	.LBB21_415
.Ltmp2263:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_415:
.Ltmp2264:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2265:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp2266:
	.loc	21 326 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp2267:
	.loc	26 103 24
	vandps	%xmm6, %xmm11, %xmm3
.Ltmp2268:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm12, %xmm6
.Ltmp2269:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp2270:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp2271:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp2272:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp2273:
	.loc	21 332 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp2274:
	.loc	21 343 23
	vmovss	760(%rbx), %xmm2
.Ltmp2275:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2276:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp2277:
	.loc	21 345 9
	vmovss	764(%rbx), %xmm2
.Ltmp2278:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm11
.Ltmp2279:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm2
.Ltmp2280:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm6, %xmm11, %xmm3
.Ltmp2281:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2282:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp2283:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp2284:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2285:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2286:
	.loc	26 161 24
	cmovbel	%r9d, %r8d
.Ltmp2287:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2288:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2289:
	.loc	26 161 24
	cmovbel	%r10d, %r8d
.Ltmp2290:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2291:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2292:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2293:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2294:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp2295:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2296:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2297:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2298:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2299:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2300:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2301:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2302:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2303:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp2304:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2305:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp2306:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2307:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2308:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2309:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2310:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm11
.Ltmp2311:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp2312:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm11
.Ltmp2313:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2314:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm11
.Ltmp2315:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2316:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp2317:
	.loc	21 370 47
	vmovss	860(%rbx), %xmm3
.Ltmp2318:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm3
.Ltmp2319:
	.loc	26 149 9
	notl	%r8d
.Ltmp2320:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2321:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm2
.Ltmp2322:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2323:
	.loc	21 375 9
	vmovss	752(%rbx), %xmm12
.Ltmp2324:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2325:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2326:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_417
.Ltmp2327:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_417:
.Ltmp2328:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_419
.Ltmp2329:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_419:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm3
.Ltmp2330:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 382 5
	movl	%r8d, 856(%rbx)
.Ltmp2331:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2332:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm11, %xmm6
.Ltmp2333:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm6, %xmm2, %xmm2
.Ltmp2334:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm6
	vxorps	%xmm6, %xmm10, %xmm6
.Ltmp2335:
	.loc	26 161 24
	vmaxss	%xmm6, %xmm2, %xmm2
.Ltmp2336:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm6
	vandps	%xmm6, %xmm3, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_421
.Ltmp2337:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_421:
.Ltmp2338:
	.loc	21 392 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp2339:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp2340:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2341:
	.loc	26 92 9
	vmulss	744(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2342:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2343:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp2344:
	.loc	21 394 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp2345:
	.loc	21 323 26
	vmovss	880(%rbx), %xmm11
.Ltmp2346:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp2347:
	.loc	21 325 44
	vmovss	876(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp2348:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp2349:
	.loc	26 161 24
	jne	.LBB21_424
	jp	.LBB21_424
.Ltmp2350:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_424:
.Ltmp2351:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_427
	jp	.LBB21_427
.Ltmp2352:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_427:
.Ltmp2353:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp2354:
	.loc	26 161 24
	jbe	.LBB21_429
.Ltmp2355:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_429:
.Ltmp2356:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm15, %xmm2
	vmovaps	%xmm2, 160(%rsp)
.Ltmp2357:
	.loc	21 326 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp2358:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2359:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp2360:
	.loc	21 332 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp2361:
	.loc	21 323 26
	vmovss	896(%rbx), %xmm12
.Ltmp2362:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp2363:
	.loc	21 325 44
	vmovss	892(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp2364:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp2365:
	.loc	26 161 24
	jne	.LBB21_432
	jp	.LBB21_432
.Ltmp2366:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_432:
.Ltmp2367:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_435
	jp	.LBB21_435
.Ltmp2368:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_435:
.Ltmp2369:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm12
.Ltmp2370:
	.loc	26 161 24
	jbe	.LBB21_437
.Ltmp2371:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_437:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp2372:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp2373:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp2374:
	.loc	21 332 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp2375:
	.loc	21 323 26
	vmovss	912(%rbx), %xmm13
.Ltmp2376:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2377:
	.loc	21 325 44
	vmovss	908(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp2378:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp2379:
	.loc	26 161 24
	jne	.LBB21_440
	jp	.LBB21_440
.Ltmp2380:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_440:
.Ltmp2381:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_443
	jp	.LBB21_443
.Ltmp2382:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_443:
.Ltmp2383:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp2384:
	.loc	26 161 24
	jbe	.LBB21_445
.Ltmp2385:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_445:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp2386:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2387:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2388:
	.loc	21 332 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp2389:
	.loc	21 323 26
	vmovss	928(%rbx), %xmm14
.Ltmp2390:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2391:
	.loc	21 325 44
	vmovss	924(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp2392:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2393:
	.loc	26 161 24
	jne	.LBB21_448
	jp	.LBB21_448
.Ltmp2394:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_448:
.Ltmp2395:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_451
	jp	.LBB21_451
.Ltmp2396:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_451:
.Ltmp2397:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp2398:
	.loc	26 161 24
	jbe	.LBB21_453
.Ltmp2399:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_453:
.Ltmp2400:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2401:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp2402:
	.loc	21 326 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp2403:
	.loc	26 103 24
	vandps	192(%rsp), %xmm6, %xmm3
.Ltmp2404:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm7, %xmm6
.Ltmp2405:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp2406:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp2407:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp2408:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp2409:
	.loc	21 332 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp2410:
	.loc	21 343 23
	vmovss	784(%rbx), %xmm2
.Ltmp2411:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2412:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp2413:
	.loc	21 345 9
	vmovss	788(%rbx), %xmm2
.Ltmp2414:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp2415:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp2416:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp2417:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2418:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp2419:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp2420:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2421:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2422:
	.loc	26 161 24
	cmovbel	%r9d, %r8d
.Ltmp2423:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2424:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2425:
	.loc	26 161 24
	cmovbel	%r10d, %r8d
.Ltmp2426:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2427:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2428:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2429:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2430:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp2431:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2432:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2433:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2434:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2435:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2436:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2437:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2438:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2439:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp2440:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2441:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp2442:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2443:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2444:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2445:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2446:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp2447:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp2448:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp2449:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2450:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp2451:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2452:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp2453:
	.loc	21 370 47
	vmovss	936(%rbx), %xmm6
.Ltmp2454:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm6
.Ltmp2455:
	.loc	26 149 9
	notl	%r8d
.Ltmp2456:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2457:
	.loc	21 362 20
	vmovss	932(%rbx), %xmm2
.Ltmp2458:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2459:
	.loc	21 375 9
	vmovss	776(%rbx), %xmm3
.Ltmp2460:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2461:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2462:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_455
.Ltmp2463:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_455:
	movq	16(%rsp), %r12
.Ltmp2464:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_457
.Ltmp2465:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_457:
.Ltmp2466:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp2467:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm13
.Ltmp2468:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp2469:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2470:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp2471:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2472:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp2473:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp2474:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_459
.Ltmp2475:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_459
.LBB21_289:
.Ltmp2476:
	.loc	25 438 16 is_stmt 1
	movq	%rsi, %rax
	negq	%rax
	movl	%r12d, %edx
	subl	%r8d, %edx
	movl	$1, %r15d
	vmovss	.LCPI21_0(%rip), %xmm0
	vmovss	.LCPI21_1(%rip), %xmm1
	movl	$841731191, %r9d
	movl	$8388608, %r10d
	xorl	%esi, %esi
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB21_290
.Ltmp2477:
	.loc	25 0 16 is_stmt 0
.Ltmp2478:
	.p2align	4
.LBB21_373:
	.loc	21 392 36 is_stmt 1
	vmovss	940(%rbx), %xmm3
.Ltmp2479:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp2480:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2481:
	.loc	26 92 9
	vmulss	768(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2482:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2483:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm10
	vmovss	.LCPI21_21(%rip), %xmm11
.Ltmp2484:
	.loc	26 71 9
	vmulss	%xmm7, %xmm11, %xmm2
	vmovss	.LCPI21_22(%rip), %xmm12
.Ltmp2485:
	.loc	26 61 9
	vaddss	%xmm2, %xmm12, %xmm2
.Ltmp2486:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_23(%rip), %xmm13
.Ltmp2487:
	.loc	26 61 9
	vaddss	%xmm2, %xmm13, %xmm2
.Ltmp2488:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_24(%rip), %xmm14
.Ltmp2489:
	.loc	26 61 9
	vaddss	%xmm2, %xmm14, %xmm2
.Ltmp2490:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
	vmovss	.LCPI21_25(%rip), %xmm9
.Ltmp2491:
	.loc	26 61 9
	vaddss	%xmm2, %xmm9, %xmm2
.Ltmp2492:
	.loc	26 71 9
	vmulss	%xmm2, %xmm7, %xmm2
.Ltmp2493:
	.loc	26 61 9
	vaddss	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_26(%rip), %xmm7
.Ltmp2494:
	.loc	26 178 22
	vaddss	%xmm7, %xmm6, %xmm3
.Ltmp2495:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp2496:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp2497:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp2498:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2499:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm5, %xmm2
.Ltmp2500:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm15, %xmm8, %xmm3
	vblendvps	%xmm3, %xmm2, %xmm5, %xmm2
.Ltmp2501:
	.loc	26 71 9
	vmulss	.LCPI21_18(%rip), %xmm10, %xmm3
.Ltmp2502:
	.loc	26 161 24
	vmaxss	.LCPI21_19(%rip), %xmm3, %xmm3
.Ltmp2503:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm3, %xmm3
	vmovaps	160(%rsp), %xmm6
.Ltmp2504:
	.loc	26 161 24
	vblendvps	%xmm6, %xmm2, %xmm5, %xmm2
.Ltmp2505:
	.loc	7 1783 9 is_stmt 1
	vroundss	$9, %xmm3, %xmm3, %xmm5
.Ltmp2506:
	.loc	26 66 9
	vsubss	%xmm5, %xmm3, %xmm3
.Ltmp2507:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm6
.Ltmp2508:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm6
.Ltmp2509:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2510:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp2511:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2512:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp2513:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm6
.Ltmp2514:
	.loc	26 61 9
	vaddss	%xmm6, %xmm9, %xmm6
.Ltmp2515:
	.loc	26 71 9
	vmulss	%xmm6, %xmm3, %xmm3
.Ltmp2516:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp2517:
	.loc	26 178 22
	vaddss	%xmm7, %xmm5, %xmm5
.Ltmp2518:
	.loc	7 1244 18
	vmovd	%xmm5, %r8d
.Ltmp2519:
	.loc	26 179 24
	shll	$23, %r8d
.Ltmp2520:
	.loc	7 1291 18
	vmovd	%r8d, %xmm5
.Ltmp2521:
	.loc	26 71 9
	vmulss	%xmm5, %xmm3, %xmm3
.Ltmp2522:
	.loc	21 394 5
	vmovss	%xmm10, 940(%rbx)
.Ltmp2523:
	.loc	26 71 9
	vmulss	%xmm3, %xmm4, %xmm3
.Ltmp2524:
	.loc	26 161 24
	vcmpneqss	%xmm15, %xmm10, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	vcmpnltss	780(%rbx), %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm4, %xmm3
	movq	240(%rsp), %r13
.Ltmp2525:
	.loc	26 56 9
	vmovss	%xmm2, -4(%r13,%r15,4)
	movq	248(%rsp), %rcx
.Ltmp2526:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm3, -4(%rcx,%r15,4)
.Ltmp2527:
	.loc	8 1916 50 is_stmt 1
	leaq	(%rax,%r15), %r8
	incq	%r8
	incq	%r15
	cmpq	$1, %r8
	movq	176(%rsp), %rbp
.Ltmp2528:
	.loc	11 900 12
	je	.LBB21_374
.LBB21_290:
.Ltmp2529:
	.loc	15 971 17
	leaq	(%rax,%r15), %r8
.Ltmp2530:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_460
.Ltmp2531:
	.loc	21 0 0 is_stmt 0
	leal	(%r12,%r15), %r8d
	decl	%r8d
	andl	%edi, %r8d
.Ltmp2532:
	.loc	25 451 16 is_stmt 1
	cmpq	%r8, %rbp
	jbe	.LBB21_462
.Ltmp2533:
	.loc	26 51 9
	vmovss	-4(%r13,%r15,4), %xmm2
	movq	8(%rsp), %r12
.Ltmp2534:
	.loc	26 56 9
	vmovss	%xmm2, (%r12,%r8,4)
.Ltmp2535:
	.loc	26 51 9
	vmovss	-4(%rcx,%r15,4), %xmm2
	movq	24(%rsp), %rcx
.Ltmp2536:
	.loc	26 56 9
	vmovss	%xmm2, (%rcx,%r8,4)
	movq	40(%rsp), %rcx
.Ltmp2537:
	.loc	26 51 9
	vmovss	-4(%rcx,%r15,4), %xmm2
.Ltmp2538:
	.loc	26 56 9
	vmovss	%xmm2, (%r11,%r8,4)
	movq	48(%rsp), %rcx
.Ltmp2539:
	.loc	26 51 9
	vmovss	-4(%rcx,%r15,4), %xmm2
.Ltmp2540:
	.loc	26 56 9
	vmovss	%xmm2, (%r14,%r8,4)
.Ltmp2541:
	.loc	21 255 21
	leal	(%rdx,%r15), %r12d
	decl	%r12d
	andl	%edi, %r12d
.Ltmp2542:
	.loc	25 438 16
	cmpq	%r12, %rbp
	jbe	.LBB21_467
.Ltmp2543:
	.loc	25 0 16 is_stmt 0
	movq	16(%rsp), %rcx
	leal	(%rcx,%r15), %r8d
	movl	136(%rbx), %r13d
	notl	%r13d
	addl	%r8d, %r13d
	andl	%edi, %r13d
	cmpq	%r13, (%rsp)
	jbe	.LBB21_473
	movq	32(%rsp), %rcx
	cmpq	%r13, %rcx
.Ltmp2544:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB21_472
	.loc	21 0 29 is_stmt 0
	movl	200(%rbx), %ebp
	notl	%ebp
	addl	%r8d, %ebp
	andl	%edi, %ebp
	cmpq	%rbp, %rcx
	jbe	.LBB21_471
	cmpq	%rbp, (%rsp)
	.loc	21 277 29 is_stmt 1
	jbe	.LBB21_470
.Ltmp2545:
	.loc	21 323 26
	vmovss	804(%rbx), %xmm9
.Ltmp2546:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm9
.Ltmp2547:
	.loc	21 325 44
	vmovss	800(%rbx), %xmm10
	.loc	21 325 27 is_stmt 0
	vmovss	792(%rbx), %xmm8
.Ltmp2548:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm8, %xmm3
.Ltmp2549:
	.loc	26 161 24
	jne	.LBB21_300
	jp	.LBB21_300
.Ltmp2550:
	.loc	26 0 24 is_stmt 0
	vmovss	796(%rbx), %xmm3
.LBB21_300:
.Ltmp2551:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_303
	jp	.LBB21_303
.Ltmp2552:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm10, %xmm10, %xmm10
.LBB21_303:
.Ltmp2553:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm9
.Ltmp2554:
	.loc	26 161 24
	jbe	.LBB21_305
.Ltmp2555:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm8
.LBB21_305:
	movq	8(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm5
	movq	24(%rsp), %rcx
	vmovss	(%rcx,%r12,4), %xmm4
	vmovss	(%r14,%r13,4), %xmm11
.Ltmp2556:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r11,%r13,4), %xmm12
.Ltmp2557:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r14,%rbp,4), %xmm2
	vmovaps	%xmm2, 192(%rsp)
.Ltmp2558:
	.loc	26 103 24
	vmovss	(%r11,%rbp,4), %xmm7
.Ltmp2559:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm8, 792(%rbx)
	.loc	21 331 13
	vmovss	%xmm10, 800(%rbx)
.Ltmp2560:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2561:
	.loc	26 161 24
	vcmpnltss	%xmm9, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm9, %xmm2, %xmm2
.Ltmp2562:
	.loc	21 332 13
	vmovss	%xmm2, 804(%rbx)
.Ltmp2563:
	.loc	21 323 26
	vmovss	820(%rbx), %xmm10
.Ltmp2564:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm10
.Ltmp2565:
	.loc	21 325 27
	vmovss	808(%rbx), %xmm9
	.loc	21 325 44 is_stmt 0
	vmovss	816(%rbx), %xmm3
.Ltmp2566:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm9, %xmm13
.Ltmp2567:
	.loc	26 161 24
	jne	.LBB21_308
	jp	.LBB21_308
.Ltmp2568:
	.loc	26 0 24 is_stmt 0
	vmovss	812(%rbx), %xmm13
.LBB21_308:
.Ltmp2569:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_311
	jp	.LBB21_311
.Ltmp2570:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_311:
.Ltmp2571:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm10
.Ltmp2572:
	.loc	26 161 24
	jbe	.LBB21_313
.Ltmp2573:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm13, %xmm9
.LBB21_313:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm9, 808(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 816(%rbx)
.Ltmp2574:
	.loc	26 66 9
	vaddss	%xmm1, %xmm10, %xmm2
.Ltmp2575:
	.loc	26 161 24
	vcmpnltss	%xmm10, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm10, %xmm2, %xmm2
.Ltmp2576:
	.loc	21 332 13
	vmovss	%xmm2, 820(%rbx)
.Ltmp2577:
	.loc	21 323 26
	vmovss	836(%rbx), %xmm13
.Ltmp2578:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2579:
	.loc	21 325 44
	vmovss	832(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	824(%rbx), %xmm10
.Ltmp2580:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm14
.Ltmp2581:
	.loc	26 161 24
	jne	.LBB21_316
	jp	.LBB21_316
.Ltmp2582:
	.loc	26 0 24 is_stmt 0
	vmovss	828(%rbx), %xmm14
.LBB21_316:
.Ltmp2583:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_319
	jp	.LBB21_319
.Ltmp2584:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_319:
.Ltmp2585:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp2586:
	.loc	26 161 24
	jbe	.LBB21_321
.Ltmp2587:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm14, %xmm10
.LBB21_321:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm10, 824(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 832(%rbx)
.Ltmp2588:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2589:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2590:
	.loc	21 332 13
	vmovss	%xmm2, 836(%rbx)
.Ltmp2591:
	.loc	21 323 26
	vmovss	852(%rbx), %xmm14
.Ltmp2592:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2593:
	.loc	21 325 44
	vmovss	848(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	840(%rbx), %xmm13
.Ltmp2594:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2595:
	.loc	26 161 24
	jne	.LBB21_324
	jp	.LBB21_324
.Ltmp2596:
	.loc	26 0 24 is_stmt 0
	vmovss	844(%rbx), %xmm2
.LBB21_324:
.Ltmp2597:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_327
	jp	.LBB21_327
.Ltmp2598:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_327:
.Ltmp2599:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp2600:
	.loc	26 161 24
	jbe	.LBB21_329
.Ltmp2601:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_329:
.Ltmp2602:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2603:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp2604:
	.loc	21 326 13
	vmovss	%xmm13, 840(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 848(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp2605:
	.loc	26 103 24
	vandps	%xmm6, %xmm12, %xmm3
.Ltmp2606:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm11, %xmm6
.Ltmp2607:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp2608:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp2609:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp2610:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp2611:
	.loc	21 332 13
	vmovss	%xmm2, 852(%rbx)
.Ltmp2612:
	.loc	21 343 23
	vmovss	760(%rbx), %xmm2
.Ltmp2613:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2614:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp2615:
	.loc	21 345 9
	vmovss	764(%rbx), %xmm2
.Ltmp2616:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm11
.Ltmp2617:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm2
.Ltmp2618:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm6, %xmm11, %xmm3
.Ltmp2619:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2620:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp2621:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp2622:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2623:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2624:
	.loc	26 161 24
	cmovbel	%r9d, %r8d
.Ltmp2625:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2626:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2627:
	.loc	26 161 24
	cmovbel	%r10d, %r8d
.Ltmp2628:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2629:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2630:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2631:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2632:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp2633:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2634:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2635:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2636:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2637:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2638:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2639:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2640:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2641:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp2642:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2643:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp2644:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2645:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2646:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2647:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2648:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm11
.Ltmp2649:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm8, %xmm2
.Ltmp2650:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm11
.Ltmp2651:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2652:
	.loc	26 129 14
	vucomiss	%xmm8, %xmm11
.Ltmp2653:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2654:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp2655:
	.loc	21 370 47
	vmovss	860(%rbx), %xmm3
.Ltmp2656:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm3
.Ltmp2657:
	.loc	26 149 9
	notl	%r8d
.Ltmp2658:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2659:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm2
.Ltmp2660:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2661:
	.loc	21 375 9
	vmovss	752(%rbx), %xmm12
.Ltmp2662:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2663:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2664:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_331
.Ltmp2665:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm3, %xmm3
.LBB21_331:
.Ltmp2666:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_333
.Ltmp2667:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm3, %xmm12
.LBB21_333:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm3
.Ltmp2668:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm12, 860(%rbx)
	.loc	21 382 5
	movl	%r8d, 856(%rbx)
.Ltmp2669:
	.loc	26 66 9
	vaddss	%xmm1, %xmm9, %xmm2
.Ltmp2670:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm8, %xmm11, %xmm6
.Ltmp2671:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm6, %xmm2, %xmm2
.Ltmp2672:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm6
	vxorps	%xmm6, %xmm10, %xmm6
.Ltmp2673:
	.loc	26 161 24
	vmaxss	%xmm6, %xmm2, %xmm2
.Ltmp2674:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm3, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm6
	vandps	%xmm6, %xmm3, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_335
.Ltmp2675:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
.LBB21_335:
.Ltmp2676:
	.loc	21 392 36 is_stmt 1
	vmovss	864(%rbx), %xmm3
.Ltmp2677:
	.loc	26 124 14
	xorl	%r8d, %r8d
	vucomiss	%xmm3, %xmm2
	setbe	%r8b
.Ltmp2678:
	.loc	26 66 9
	vsubss	%xmm3, %xmm2, %xmm2
.Ltmp2679:
	.loc	26 92 9
	vmulss	744(%rbx,%r8,4), %xmm2, %xmm2
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2680:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp2681:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm8
.Ltmp2682:
	.loc	21 394 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp2683:
	.loc	21 323 26
	vmovss	880(%rbx), %xmm11
.Ltmp2684:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm11
.Ltmp2685:
	.loc	21 325 44
	vmovss	876(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	868(%rbx), %xmm10
.Ltmp2686:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm10, %xmm2
.Ltmp2687:
	.loc	26 161 24
	jne	.LBB21_338
	jp	.LBB21_338
.Ltmp2688:
	.loc	26 0 24 is_stmt 0
	vmovss	872(%rbx), %xmm2
.LBB21_338:
.Ltmp2689:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_341
	jp	.LBB21_341
.Ltmp2690:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_341:
.Ltmp2691:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm11
.Ltmp2692:
	.loc	26 161 24
	jbe	.LBB21_343
.Ltmp2693:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm10
.LBB21_343:
.Ltmp2694:
	.loc	26 161 24 is_stmt 1
	vcmpnltss	756(%rbx), %xmm15, %xmm2
	vmovaps	%xmm2, 160(%rsp)
.Ltmp2695:
	.loc	21 326 13
	vmovss	%xmm10, 868(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 876(%rbx)
.Ltmp2696:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2697:
	.loc	26 161 24
	vcmpnltss	%xmm11, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm11, %xmm2, %xmm2
.Ltmp2698:
	.loc	21 332 13
	vmovss	%xmm2, 880(%rbx)
.Ltmp2699:
	.loc	21 323 26
	vmovss	896(%rbx), %xmm12
.Ltmp2700:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm12
.Ltmp2701:
	.loc	21 325 44
	vmovss	892(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	884(%rbx), %xmm11
.Ltmp2702:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm11, %xmm2
.Ltmp2703:
	.loc	26 161 24
	jne	.LBB21_346
	jp	.LBB21_346
.Ltmp2704:
	.loc	26 0 24 is_stmt 0
	vmovss	888(%rbx), %xmm2
.LBB21_346:
.Ltmp2705:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_349
	jp	.LBB21_349
.Ltmp2706:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_349:
.Ltmp2707:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm12
.Ltmp2708:
	.loc	26 161 24
	jbe	.LBB21_351
.Ltmp2709:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm11
.LBB21_351:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm11, 884(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 892(%rbx)
.Ltmp2710:
	.loc	26 66 9
	vaddss	%xmm1, %xmm12, %xmm2
.Ltmp2711:
	.loc	26 161 24
	vcmpnltss	%xmm12, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm12, %xmm2, %xmm2
.Ltmp2712:
	.loc	21 332 13
	vmovss	%xmm2, 896(%rbx)
.Ltmp2713:
	.loc	21 323 26
	vmovss	912(%rbx), %xmm13
.Ltmp2714:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm13
.Ltmp2715:
	.loc	21 325 44
	vmovss	908(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	900(%rbx), %xmm12
.Ltmp2716:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm12, %xmm2
.Ltmp2717:
	.loc	26 161 24
	jne	.LBB21_354
	jp	.LBB21_354
.Ltmp2718:
	.loc	26 0 24 is_stmt 0
	vmovss	904(%rbx), %xmm2
.LBB21_354:
.Ltmp2719:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_357
	jp	.LBB21_357
.Ltmp2720:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_357:
.Ltmp2721:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm13
.Ltmp2722:
	.loc	26 161 24
	jbe	.LBB21_359
.Ltmp2723:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm12
.LBB21_359:
	.loc	21 326 13 is_stmt 1
	vmovss	%xmm12, 900(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 908(%rbx)
.Ltmp2724:
	.loc	26 66 9
	vaddss	%xmm1, %xmm13, %xmm2
.Ltmp2725:
	.loc	26 161 24
	vcmpnltss	%xmm13, %xmm15, %xmm3
	vblendvps	%xmm3, %xmm13, %xmm2, %xmm2
.Ltmp2726:
	.loc	21 332 13
	vmovss	%xmm2, 912(%rbx)
.Ltmp2727:
	.loc	21 323 26
	vmovss	928(%rbx), %xmm14
.Ltmp2728:
	.loc	26 134 14
	vucomiss	%xmm0, %xmm14
.Ltmp2729:
	.loc	21 325 44
	vmovss	924(%rbx), %xmm3
	.loc	21 325 27 is_stmt 0
	vmovss	916(%rbx), %xmm13
.Ltmp2730:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm3, %xmm13, %xmm2
.Ltmp2731:
	.loc	26 161 24
	jne	.LBB21_362
	jp	.LBB21_362
.Ltmp2732:
	.loc	26 0 24 is_stmt 0
	vmovss	920(%rbx), %xmm2
.LBB21_362:
.Ltmp2733:
	.loc	26 161 44 is_stmt 1
	jne	.LBB21_365
	jp	.LBB21_365
.Ltmp2734:
	.loc	26 0 44 is_stmt 0
	vxorps	%xmm3, %xmm3, %xmm3
.LBB21_365:
.Ltmp2735:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm15, %xmm14
.Ltmp2736:
	.loc	26 161 24
	jbe	.LBB21_367
.Ltmp2737:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm2, %xmm13
.LBB21_367:
.Ltmp2738:
	.loc	26 66 9 is_stmt 1
	vaddss	%xmm1, %xmm14, %xmm2
.Ltmp2739:
	.loc	26 161 24
	vcmpnltss	%xmm14, %xmm15, %xmm6
	vblendvps	%xmm6, %xmm14, %xmm2, %xmm2
.Ltmp2740:
	.loc	21 326 13
	vmovss	%xmm13, 916(%rbx)
	.loc	21 331 13
	vmovss	%xmm3, 924(%rbx)
	vbroadcastss	.LCPI21_2(%rip), %xmm6
.Ltmp2741:
	.loc	26 103 24
	vandps	192(%rsp), %xmm6, %xmm3
.Ltmp2742:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm6, %xmm7, %xmm6
.Ltmp2743:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm6, %xmm3
.Ltmp2744:
	.loc	7 1244 18
	vmovd	%xmm3, %r8d
.Ltmp2745:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm6, %ebp
.Ltmp2746:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %ebp
.Ltmp2747:
	.loc	21 332 13
	vmovss	%xmm2, 928(%rbx)
.Ltmp2748:
	.loc	21 343 23
	vmovss	784(%rbx), %xmm2
.Ltmp2749:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2750:
	.loc	26 161 24
	cmovbel	%r8d, %ebp
.Ltmp2751:
	.loc	21 345 9
	vmovss	788(%rbx), %xmm2
.Ltmp2752:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
	vmovss	.LCPI21_3(%rip), %xmm7
.Ltmp2753:
	.loc	26 71 9
	vmulss	%xmm7, %xmm3, %xmm2
.Ltmp2754:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm7, %xmm6, %xmm3
.Ltmp2755:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2756:
	.loc	7 1244 18
	vmovd	%xmm2, %r8d
.Ltmp2757:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp2758:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2759:
	.loc	26 124 14
	vucomiss	.LCPI21_4(%rip), %xmm2
.Ltmp2760:
	.loc	26 161 24
	cmovbel	%r9d, %r8d
.Ltmp2761:
	.loc	7 1291 18
	vmovd	%r8d, %xmm2
.Ltmp2762:
	.loc	26 124 14
	vucomiss	.LCPI21_5(%rip), %xmm2
.Ltmp2763:
	.loc	26 161 24
	cmovbel	%r10d, %r8d
.Ltmp2764:
	.loc	26 185 42
	movl	%r8d, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp2765:
	.loc	7 1291 18
	vmovd	%ebp, %xmm2
.Ltmp2766:
	.loc	26 66 9
	vaddss	%xmm1, %xmm2, %xmm2
.Ltmp2767:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm2, %xmm3
.Ltmp2768:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm6
	vsubss	%xmm3, %xmm6, %xmm3
.Ltmp2769:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2770:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp2771:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2772:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp2773:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2774:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp2775:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm3
.Ltmp2776:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp2777:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp2778:
	.loc	26 71 9
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2779:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp2780:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp2781:
	.loc	26 61 9
	vaddss	%xmm2, %xmm3, %xmm2
.Ltmp2782:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm2, %xmm2
.Ltmp2783:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm2, %xmm2
.Ltmp2784:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm2, %xmm14
.Ltmp2785:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm13, %xmm10, %xmm2
.Ltmp2786:
	.loc	26 129 14
	vucomiss	%xmm2, %xmm14
.Ltmp2787:
	.loc	26 28 5
	movl	$0, %r12d
	adcl	$-1, %r12d
.Ltmp2788:
	.loc	26 129 14
	vucomiss	%xmm10, %xmm14
.Ltmp2789:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp2790:
	.loc	26 149 9
	movl	%r12d, %r8d
.Ltmp2791:
	.loc	21 370 47
	vmovss	936(%rbx), %xmm6
.Ltmp2792:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm6
.Ltmp2793:
	.loc	26 149 9
	notl	%r8d
.Ltmp2794:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2795:
	.loc	21 362 20
	vmovss	932(%rbx), %xmm2
.Ltmp2796:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm2
.Ltmp2797:
	.loc	21 375 9
	vmovss	776(%rbx), %xmm3
.Ltmp2798:
	.loc	26 144 9
	cmoval	%r12d, %ebp
.Ltmp2799:
	.loc	26 139 9
	cmovbel	%esi, %r8d
.Ltmp2800:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_369
.Ltmp2801:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm1, %xmm6, %xmm6
.LBB21_369:
	movq	16(%rsp), %r12
.Ltmp2802:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_371
.Ltmp2803:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm6, %xmm3
.LBB21_371:
.Ltmp2804:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm2
	vmaxss	.LCPI21_19(%rip), %xmm2, %xmm2
	vminss	.LCPI21_20(%rip), %xmm2, %xmm2
	vroundss	$9, %xmm2, %xmm2, %xmm6
	vsubss	%xmm6, %xmm2, %xmm7
.Ltmp2805:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm13
.Ltmp2806:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp2807:
	.loc	26 66 9
	vaddss	%xmm1, %xmm11, %xmm2
.Ltmp2808:
	.loc	26 66 9 is_stmt 0
	vsubss	%xmm10, %xmm14, %xmm3
.Ltmp2809:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm2, %xmm2
.Ltmp2810:
	.loc	26 98 24
	vbroadcastss	.LCPI21_16(%rip), %xmm3
	vxorps	%xmm3, %xmm12, %xmm3
.Ltmp2811:
	.loc	26 161 24
	vmaxss	%xmm3, %xmm2, %xmm2
.Ltmp2812:
	.loc	26 161 44 is_stmt 0
	vcmpnltps	%xmm13, %xmm15, %xmm3
	vcmpltps	%xmm15, %xmm2, %xmm10
	vandps	%xmm3, %xmm10, %xmm3
	vmovd	%xmm3, %r8d
	testb	$1, %r8b
	jne	.LBB21_373
.Ltmp2813:
	.loc	26 0 44
	vxorps	%xmm2, %xmm2, %xmm2
	jmp	.LBB21_373
.LBB21_374:
	movq	344(%rsp), %rsi
.Ltmp2814:
	.loc	15 2584 13 is_stmt 1
	leal	(%rsi,%r12), %eax
.Ltmp2815:
	.loc	21 297 5
	movl	%eax, 1208(%rbx)
	movq	496(%rsp), %r11
	movq	368(%rsp), %r8
	movq	360(%rsp), %r15
	movq	152(%rsp), %r9
	movq	256(%rsp), %rax
.Ltmp2816:
.LBB21_1:
	.loc	6 688 12
	cmpq	%rax, %r11
	jbe	.LBB21_2
.Ltmp2817:
	.loc	18 1161 15
	movq	(%r9), %rax
	testq	%rax, %rax
	.loc	18 1161 9 is_stmt 0
	je	.LBB21_495
.Ltmp2818:
	.loc	18 1162 29 is_stmt 1
	movq	8(%r9), %rdx
.Ltmp2819:
	.loc	25 568 12
	movq	%rdx, %rdi
	subq	%rsi, %rdi
	movq	%rdi, 280(%rsp)
	jb	.LBB21_499
.Ltmp2820:
	.loc	18 1162 29
	movq	24(%r9), %rdx
.Ltmp2821:
	.loc	25 568 12
	movq	%rdx, %rdi
	subq	%rsi, %rdi
	movq	%rdi, 288(%rsp)
	jb	.LBB21_498
.Ltmp2822:
	.loc	18 1162 29
	movq	16(%r9), %rdx
.Ltmp2823:
	.loc	25 89 24
	leaq	(%rax,%rsi,4), %rax
	movq	%rax, 152(%rsp)
.Ltmp2824:
	.loc	25 89 24 is_stmt 0
	leaq	(%rdx,%rsi,4), %rax
	movq	%rax, 232(%rsp)
.Ltmp2825:
	.loc	25 580 12 is_stmt 1
	movq	%rsi, %rax
	subq	%r15, %rax
	movq	%rax, 224(%rsp)
	ja	.LBB21_713
.Ltmp2826:
.LBB21_503:
	.loc	25 580 12 is_stmt 0
	movq	%rsi, %rax
	subq	%r8, %rax
	movq	%rax, 296(%rsp)
	ja	.LBB21_705
.Ltmp2827:
	.loc	18 1039 15 is_stmt 1
	cmpq	$0, 152(%rsp)
	.loc	18 1039 9 is_stmt 0
	jne	.LBB21_506
.Ltmp2828:
	.loc	18 0 9
	movl	$4, %eax
	movq	%rax, 232(%rsp)
	movq	$0, 288(%rsp)
	movq	%rax, 152(%rsp)
	movq	$0, 280(%rsp)
.LBB21_506:
	leaq	(,%rsi,4), %r12
	addq	%r13, %r12
	leaq	(%rcx,%rsi,4), %rax
	movq	%rax, 16(%rsp)
.Ltmp2829:
	movq	104(%rbx), %rax
	movq	%rax, 24(%rsp)
	movq	112(%rbx), %r9
	movq	120(%rbx), %rax
	movq	%rax, (%rsp)
	movq	128(%rbx), %rdx
	movq	168(%rbx), %rax
	movq	%rax, 40(%rsp)
	movq	176(%rbx), %rbp
	movq	184(%rbx), %rax
	movq	%rax, 8(%rsp)
	movq	192(%rbx), %rax
	movq	%rax, 56(%rsp)
	movl	1212(%rbx), %ecx
	movl	1216(%rbx), %edi
.Ltmp2830:
	.loc	21 238 16 is_stmt 1
	movl	1208(%rbx), %r13d
	movl	136(%rbx), %r14d
	movl	200(%rbx), %r10d
	vmovss	792(%rbx), %xmm11
	vmovss	760(%rbx), %xmm0
	vmovss	%xmm0, 128(%rsp)
	vmovss	764(%rbx), %xmm0
	vmovss	%xmm0, 124(%rsp)
	vsubss	840(%rbx), %xmm11, %xmm0
	vmovss	%xmm0, 120(%rsp)
	vmovss	752(%rbx), %xmm0
	vmovss	%xmm0, 116(%rsp)
	vmovss	.LCPI21_1(%rip), %xmm2
	vaddss	808(%rbx), %xmm2, %xmm0
	vmovss	%xmm0, 112(%rsp)
	vmovss	824(%rbx), %xmm0
	vbroadcastss	.LCPI21_16(%rip), %xmm1
	vxorps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 320(%rsp)
	vmovss	756(%rbx), %xmm0
	vmovss	%xmm0, 108(%rsp)
	vmovss	868(%rbx), %xmm0
	vmovss	784(%rbx), %xmm3
	vmovss	%xmm3, 104(%rsp)
	vmovss	%xmm0, 32(%rsp)
	vsubss	916(%rbx), %xmm0, %xmm0
	vmovss	%xmm0, 100(%rsp)
	vmovss	788(%rbx), %xmm0
	vmovss	%xmm0, 96(%rsp)
	vmovss	776(%rbx), %xmm0
	vmovss	%xmm0, 92(%rsp)
	vmovss	900(%rbx), %xmm0
	vxorps	%xmm1, %xmm0, %xmm0
	vmovaps	%xmm0, 304(%rsp)
	vaddss	884(%rbx), %xmm2, %xmm0
	vmovss	%xmm0, 88(%rsp)
	movl	744(%rbx), %eax
	movl	%eax, 84(%rsp)
	movl	748(%rbx), %eax
	movl	%eax, 80(%rsp)
	movl	768(%rbx), %eax
	movl	%eax, 76(%rsp)
	movl	772(%rbx), %eax
	movl	%eax, 72(%rsp)
	vmovss	780(%rbx), %xmm0
	vmovss	%xmm0, 68(%rsp)
	vmovss	860(%rbx), %xmm4
	vmovss	864(%rbx), %xmm7
	vmovss	936(%rbx), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	940(%rbx), %xmm0
	vmovaps	%xmm0, 176(%rsp)
	cmpq	%r8, %r15
	movq	%rdx, 160(%rsp)
	movq	%r9, 256(%rsp)
	movq	%r12, 136(%rsp)
	movq	%r13, 144(%rsp)
	vmovss	%xmm11, 132(%rsp)
	jbe	.LBB21_511
	.loc	21 0 16 is_stmt 0
	subq	%rsi, %r8
	movq	%r8, 216(%rsp)
	movq	288(%rsp), %rax
.Ltmp2831:
	.loc	25 451 16 is_stmt 1
	negq	%rax
	movq	%rax, 352(%rsp)
	movq	280(%rsp), %rax
	negq	%rax
	movq	%rax, 224(%rsp)
	movq	%r11, %rax
	movl	%r13d, %r11d
	subl	%r10d, %r11d
	movl	%r13d, %r10d
	subl	%r14d, %r10d
	movl	%r13d, %r9d
	subl	%edi, %r9d
	movq	%rsi, %r15
	subq	%rax, %r15
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm10
	vmovss	.LCPI21_4(%rip), %xmm9
	vmovss	.LCPI21_5(%rip), %xmm12
	xorl	%eax, %eax
	movq	%r13, %r8
	vxorps	%xmm15, %xmm15, %xmm15
	jmp	.LBB21_508
.Ltmp2832:
	.loc	25 0 16 is_stmt 0
.Ltmp2833:
	.p2align	4
.LBB21_700:
	vmovaps	176(%rsp), %xmm2
.Ltmp2834:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm0
	movl	76(%rsp), %esi
.Ltmp2835:
	.loc	26 161 24
	cmovbel	72(%rsp), %esi
.Ltmp2836:
	.loc	7 1291 18
	vmovd	%esi, %xmm9
.Ltmp2837:
	.loc	26 66 9
	vsubss	%xmm2, %xmm0, %xmm0
.Ltmp2838:
	.loc	26 92 9
	vmulss	%xmm0, %xmm9, %xmm0
	vaddss	%xmm0, %xmm2, %xmm0
.Ltmp2839:
	.loc	26 103 24
	vandps	%xmm0, %xmm14, %xmm2
.Ltmp2840:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm2, %xmm2
	vandps	%xmm0, %xmm2, %xmm7
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp2841:
	.loc	26 71 9
	vmulss	%xmm5, %xmm9, %xmm0
	vmovss	.LCPI21_22(%rip), %xmm11
.Ltmp2842:
	.loc	26 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp2843:
	.loc	26 71 9
	vmulss	%xmm0, %xmm5, %xmm0
	vmovss	.LCPI21_23(%rip), %xmm14
.Ltmp2844:
	.loc	26 61 9
	vaddss	%xmm0, %xmm14, %xmm0
.Ltmp2845:
	.loc	26 71 9
	vmulss	%xmm0, %xmm5, %xmm0
	vmovss	.LCPI21_24(%rip), %xmm4
.Ltmp2846:
	.loc	26 61 9
	vaddss	%xmm4, %xmm0, %xmm0
.Ltmp2847:
	.loc	26 71 9
	vmulss	%xmm0, %xmm5, %xmm0
	vmovss	.LCPI21_25(%rip), %xmm2
.Ltmp2848:
	.loc	26 61 9
	vaddss	%xmm2, %xmm0, %xmm0
.Ltmp2849:
	.loc	26 71 9
	vmulss	%xmm0, %xmm5, %xmm0
.Ltmp2850:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI21_18(%rip), %xmm7, %xmm5
.Ltmp2851:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI21_19(%rip), %xmm5, %xmm5
.Ltmp2852:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm5, %xmm5
	vmovss	.LCPI21_0(%rip), %xmm12
.Ltmp2853:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm0, %xmm12, %xmm0
	vmovss	.LCPI21_26(%rip), %xmm10
.Ltmp2854:
	.loc	26 178 22
	vaddss	%xmm6, %xmm10, %xmm6
.Ltmp2855:
	.loc	7 1244 18
	vmovd	%xmm6, %esi
.Ltmp2856:
	.loc	26 179 24
	shll	$23, %esi
.Ltmp2857:
	.loc	7 1291 18
	vmovd	%esi, %xmm6
.Ltmp2858:
	.loc	26 71 9
	vmulss	%xmm6, %xmm0, %xmm0
.Ltmp2859:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm0, %xmm3, %xmm0
.Ltmp2860:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm15, %xmm8, %xmm6
	vblendvps	%xmm6, %xmm0, %xmm3, %xmm0
	vcmpnltss	108(%rsp), %xmm15, %xmm6
	vblendvps	%xmm6, %xmm0, %xmm3, %xmm0
.Ltmp2861:
	.loc	7 1783 9
	vroundss	$9, %xmm5, %xmm5, %xmm3
.Ltmp2862:
	.loc	26 66 9
	vsubss	%xmm3, %xmm5, %xmm5
.Ltmp2863:
	.loc	26 71 9
	vmulss	%xmm5, %xmm9, %xmm6
.Ltmp2864:
	.loc	26 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp2865:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp2866:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp2867:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp2868:
	.loc	26 61 9
	vaddss	%xmm4, %xmm6, %xmm6
.Ltmp2869:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp2870:
	.loc	26 61 9
	vaddss	%xmm2, %xmm6, %xmm6
.Ltmp2871:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm5
.Ltmp2872:
	.loc	26 61 9
	vaddss	%xmm5, %xmm12, %xmm5
.Ltmp2873:
	.loc	26 178 22
	vaddss	%xmm3, %xmm10, %xmm3
.Ltmp2874:
	.loc	7 1244 18
	vmovd	%xmm3, %esi
.Ltmp2875:
	.loc	26 179 24
	shll	$23, %esi
.Ltmp2876:
	.loc	7 1291 18
	vmovd	%esi, %xmm3
.Ltmp2877:
	.loc	26 71 9
	vmulss	%xmm3, %xmm5, %xmm3
.Ltmp2878:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp2879:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm1, %xmm3
	vcmpnltss	68(%rsp), %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm1, %xmm1
.Ltmp2880:
	.loc	26 56 9
	vmovss	%xmm0, -4(%r12,%r14,4)
	movq	16(%rsp), %rdx
.Ltmp2881:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm1, -4(%rdx,%r14,4)
	vmovaps	%xmm7, 176(%rsp)
.Ltmp2882:
	.loc	21 394 5 is_stmt 1
	vmovss	%xmm7, 940(%rbx)
.Ltmp2883:
	.loc	8 1916 50
	leaq	(%r15,%r14), %rsi
	incq	%rsi
	incq	%r14
	cmpq	$1, %rsi
	movq	144(%rsp), %r8
	vmovss	132(%rsp), %xmm11
	vmovss	.LCPI21_4(%rip), %xmm9
	vmovss	.LCPI21_5(%rip), %xmm12
	vmovss	192(%rsp), %xmm4
	vmovss	%xmm13, 48(%rsp)
	vmovss	.LCPI21_3(%rip), %xmm10
	vmovaps	%xmm8, %xmm7
.Ltmp2884:
	.loc	11 900 12
	je	.LBB21_587
.Ltmp2885:
.LBB21_508:
	.loc	21 246 22
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	movq	256(%rsp), %r13
.Ltmp2886:
	.loc	25 451 16
	cmpq	%rdi, %r13
	jbe	.LBB21_565
.Ltmp2887:
	.loc	25 0 16 is_stmt 0
	movq	296(%rsp), %rdx
	leaq	(%rdx,%r14), %rsi
.Ltmp2888:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r14,4), %xmm0
	movq	24(%rsp), %rdx
.Ltmp2889:
	.loc	26 56 9
	vmovss	%xmm0, (%rdx,%rdi,4)
.Ltmp2890:
	.loc	25 438 16
	cmpq	$1, %rsi
	je	.LBB21_510
.Ltmp2891:
	.loc	25 451 16
	cmpq	%rdi, %rbp
	jbe	.LBB21_714
.Ltmp2892:
	.loc	25 0 16 is_stmt 0
	movq	224(%rsp), %rdx
	leaq	(%rdx,%r14), %rsi
	movq	16(%rsp), %rdx
.Ltmp2893:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rdx,%r14,4), %xmm0
	movq	40(%rsp), %rdx
.Ltmp2894:
	.loc	26 56 9
	vmovss	%xmm0, (%rdx,%rdi,4)
.Ltmp2895:
	.loc	25 438 16
	cmpq	$1, %rsi
	je	.LBB21_674
.Ltmp2896:
	.loc	25 0 16 is_stmt 0
	movq	160(%rsp), %rdx
.Ltmp2897:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_637
.Ltmp2898:
	.loc	25 0 16 is_stmt 0
	movq	352(%rsp), %rsi
	addq	%r14, %rsi
	movq	152(%rsp), %r8
.Ltmp2899:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r8,%r14,4), %xmm0
	movq	(%rsp), %r8
.Ltmp2900:
	.loc	26 56 9
	vmovss	%xmm0, (%r8,%rdi,4)
.Ltmp2901:
	.loc	25 438 16
	cmpq	$1, %rsi
	je	.LBB21_677
.Ltmp2902:
	.loc	25 0 16 is_stmt 0
	movq	56(%rsp), %rsi
.Ltmp2903:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB21_715
.Ltmp2904:
	.loc	25 0 16 is_stmt 0
	movq	232(%rsp), %r8
.Ltmp2905:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r8,%r14,4), %xmm0
	movq	8(%rsp), %r8
.Ltmp2906:
	.loc	26 56 9
	vmovss	%xmm0, (%r8,%rdi,4)
.Ltmp2907:
	.loc	21 255 21
	leal	(%r9,%r14), %r12d
	decl	%r12d
	andl	%ecx, %r12d
.Ltmp2908:
	.loc	25 438 16
	cmpq	%r12, %r13
	jbe	.LBB21_568
.Ltmp2909:
	.loc	25 438 16 is_stmt 0
	cmpq	%r12, %rbp
	jbe	.LBB21_681
.Ltmp2910:
	.loc	25 0 16
	leal	(%r10,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	cmpq	%rdi, %rdx
	jbe	.LBB21_704
	cmpq	%rdi, %rsi
	vmovss	.LCPI21_1(%rip), %xmm6
	vbroadcastss	.LCPI21_2(%rip), %xmm14
.Ltmp2911:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB21_703
	.loc	21 0 29 is_stmt 0
	leal	(%r11,%r14), %r8d
	decl	%r8d
	andl	%ecx, %r8d
	cmpq	%r8, %rsi
	jbe	.LBB21_702
	cmpq	%r8, %rdx
	.loc	21 277 29 is_stmt 1
	jbe	.LBB21_701
.Ltmp2912:
	.loc	21 0 29 is_stmt 0
	movq	8(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm0
	movq	(%rsp), %rdx
.Ltmp2913:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm1
	vandps	%xmm1, %xmm14, %xmm1
.Ltmp2914:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm0, %xmm14, %xmm0
.Ltmp2915:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
.Ltmp2916:
	.loc	7 1244 18
	vmovd	%xmm1, %esi
.Ltmp2917:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm0, %edi
.Ltmp2918:
	.loc	26 161 24 is_stmt 1
	cmoval	%esi, %edi
	vmovss	128(%rsp), %xmm3
	vucomiss	%xmm15, %xmm3
.Ltmp2919:
	.loc	26 161 24 is_stmt 0
	cmovbel	%esi, %edi
	vmovss	124(%rsp), %xmm3
	vucomiss	%xmm15, %xmm3
.Ltmp2920:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm10, %xmm1
.Ltmp2921:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm0, %xmm10, %xmm0
.Ltmp2922:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm1, %xmm0, %xmm0
.Ltmp2923:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp2924:
	.loc	26 161 24
	cmovbel	%edi, %esi
.Ltmp2925:
	.loc	7 1291 18
	vmovd	%esi, %xmm0
.Ltmp2926:
	.loc	26 124 14
	vucomiss	%xmm9, %xmm0
.Ltmp2927:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %esi
.Ltmp2928:
	.loc	7 1291 18
	vmovd	%esi, %xmm0
.Ltmp2929:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm0
.Ltmp2930:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %esi
.Ltmp2931:
	.loc	26 185 42
	movl	%esi, %edi
	andl	$8388607, %edi
	orl	$1065353216, %edi
.Ltmp2932:
	.loc	7 1291 18
	vmovd	%edi, %xmm0
.Ltmp2933:
	.loc	26 66 9
	vaddss	%xmm6, %xmm0, %xmm0
.Ltmp2934:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm0, %xmm1
.Ltmp2935:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm3
	vsubss	%xmm1, %xmm3, %xmm1
.Ltmp2936:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp2937:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm1, %xmm1
.Ltmp2938:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp2939:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm1, %xmm1
.Ltmp2940:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp2941:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm1, %xmm1
.Ltmp2942:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp2943:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm1, %xmm1
.Ltmp2944:
	.loc	26 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp2945:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm0
.Ltmp2946:
	.loc	7 1291 18
	vmovd	%esi, %xmm1
.Ltmp2947:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm1, %xmm1
.Ltmp2948:
	.loc	26 61 9
	vaddss	%xmm0, %xmm1, %xmm0
.Ltmp2949:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm0, %xmm0
.Ltmp2950:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm0, %xmm0
.Ltmp2951:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm0, %xmm0
.Ltmp2952:
	.loc	26 129 14 is_stmt 1
	vucomiss	120(%rsp), %xmm0
.Ltmp2953:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp2954:
	.loc	26 129 14
	vucomiss	%xmm11, %xmm0
.Ltmp2955:
	.loc	26 144 9
	movl	$0, %esi
	adcl	$-1, %esi
.Ltmp2956:
	.loc	26 149 9
	movl	%r13d, %edi
.Ltmp2957:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm4
.Ltmp2958:
	.loc	26 149 9
	notl	%edi
.Ltmp2959:
	.loc	26 139 9
	cmovbel	%eax, %edi
.Ltmp2960:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm1
.Ltmp2961:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm1
.Ltmp2962:
	.loc	26 144 9
	cmoval	%r13d, %esi
.Ltmp2963:
	.loc	26 139 9
	cmovbel	%eax, %edi
.Ltmp2964:
	.loc	26 161 24
	testb	$1, %dil
	jne	.LBB21_687
.Ltmp2965:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm1
	vmovss	116(%rsp), %xmm4
.Ltmp2966:
	.loc	26 161 24
	testb	$1, %sil
	je	.LBB21_690
	jmp	.LBB21_691
.Ltmp2967:
	.loc	26 0 24
.Ltmp2968:
	.p2align	4
.LBB21_687:
	vaddss	%xmm6, %xmm4, %xmm1
	vmovss	116(%rsp), %xmm4
.Ltmp2969:
	.loc	26 161 24
	testb	$1, %sil
	jne	.LBB21_691
.Ltmp2970:
.LBB21_690:
	.loc	26 0 24
	vmovaps	%xmm1, %xmm4
.LBB21_691:
	orl	%esi, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm1
.Ltmp2971:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm11, %xmm0, %xmm0
.Ltmp2972:
	.loc	26 71 9
	vmulss	112(%rsp), %xmm0, %xmm0
.Ltmp2973:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm0, %xmm0
.Ltmp2974:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm1, %xmm15, %xmm1
	vcmpltss	%xmm15, %xmm0, %xmm3
	vandps	%xmm3, %xmm1, %xmm1
	vmovd	%xmm1, %esi
	testb	$1, %sil
	jne	.LBB21_693
.Ltmp2975:
	.loc	26 0 44
	vxorps	%xmm0, %xmm0, %xmm0
.LBB21_693:
.Ltmp2976:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm0
	movl	84(%rsp), %r13d
.Ltmp2977:
	.loc	26 161 24
	cmovbel	80(%rsp), %r13d
	movq	8(%rsp), %rdx
.Ltmp2978:
	.loc	26 103 24
	vmovss	(%rdx,%r8,4), %xmm1
	vandps	%xmm1, %xmm14, %xmm1
	movq	(%rsp), %rdx
.Ltmp2979:
	.loc	26 103 24 is_stmt 0
	vmovss	(%rdx,%r8,4), %xmm3
	vandps	%xmm3, %xmm14, %xmm3
.Ltmp2980:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm3, %xmm1
.Ltmp2981:
	.loc	7 1244 18
	vmovd	%xmm1, %esi
.Ltmp2982:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm3, %r8d
.Ltmp2983:
	.loc	26 161 24 is_stmt 1
	cmoval	%esi, %r8d
	vmovss	104(%rsp), %xmm5
	vucomiss	%xmm15, %xmm5
.Ltmp2984:
	.loc	26 161 24 is_stmt 0
	cmovbel	%esi, %r8d
	vmovss	96(%rsp), %xmm5
	vucomiss	%xmm15, %xmm5
.Ltmp2985:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm10, %xmm1
.Ltmp2986:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm10, %xmm3
.Ltmp2987:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm1, %xmm3, %xmm1
.Ltmp2988:
	.loc	7 1244 18
	vmovd	%xmm1, %esi
.Ltmp2989:
	.loc	26 161 24
	cmovbel	%r8d, %esi
.Ltmp2990:
	.loc	7 1291 18
	vmovd	%esi, %xmm1
.Ltmp2991:
	.loc	26 124 14
	vucomiss	%xmm9, %xmm1
.Ltmp2992:
	.loc	7 1291 18
	vmovd	%r13d, %xmm1
.Ltmp2993:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %esi
.Ltmp2994:
	.loc	26 66 9
	vsubss	%xmm7, %xmm0, %xmm0
.Ltmp2995:
	.loc	7 1291 18
	vmovd	%esi, %xmm3
.Ltmp2996:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm3
.Ltmp2997:
	.loc	26 92 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp2998:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %esi
.Ltmp2999:
	.loc	26 185 42
	movl	%esi, %r8d
	andl	$8388607, %r8d
	orl	$1065353216, %r8d
.Ltmp3000:
	.loc	7 1291 18
	vmovd	%r8d, %xmm0
.Ltmp3001:
	.loc	26 66 9
	vaddss	%xmm6, %xmm0, %xmm0
.Ltmp3002:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm0, %xmm3
.Ltmp3003:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm5
	vsubss	%xmm3, %xmm5, %xmm3
.Ltmp3004:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp3005:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp3006:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp3007:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp3008:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp3009:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp3010:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp3011:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp3012:
	.loc	26 187 28
	shrl	$23, %esi
	orl	$1258291200, %esi
.Ltmp3013:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm0
.Ltmp3014:
	.loc	7 1291 18
	vmovd	%esi, %xmm3
.Ltmp3015:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp3016:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm0
	movq	24(%rsp), %rdx
	vmovss	(%rdx,%r12,4), %xmm3
.Ltmp3017:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm0, %xmm0
.Ltmp3018:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm0, %xmm0
.Ltmp3019:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm0, %xmm0
.Ltmp3020:
	.loc	26 129 14 is_stmt 1
	vucomiss	100(%rsp), %xmm0
.Ltmp3021:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp3022:
	.loc	26 129 14
	vucomiss	32(%rsp), %xmm0
.Ltmp3023:
	.loc	26 92 9
	vaddss	%xmm1, %xmm7, %xmm1
.Ltmp3024:
	.loc	26 144 9
	movl	$0, %esi
	adcl	$-1, %esi
.Ltmp3025:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%rbx), %xmm5
.Ltmp3026:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r8d
	vmovss	48(%rsp), %xmm7
.Ltmp3027:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm7
.Ltmp3028:
	.loc	26 149 9
	notl	%r8d
.Ltmp3029:
	.loc	26 139 9
	cmovbel	%eax, %r8d
.Ltmp3030:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm5
.Ltmp3031:
	.loc	26 103 24
	vandps	%xmm1, %xmm14, %xmm5
.Ltmp3032:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm5, %xmm5
	vandps	%xmm1, %xmm5, %xmm8
	movq	40(%rsp), %rdx
	vmovss	(%rdx,%r12,4), %xmm1
.Ltmp3033:
	.loc	21 373 5
	vmovss	%xmm4, 860(%rbx)
	.loc	21 382 5
	movl	%edi, 856(%rbx)
.Ltmp3034:
	.loc	21 394 5
	vmovss	%xmm8, 864(%rbx)
.Ltmp3035:
	.loc	26 144 9
	cmoval	%r13d, %esi
.Ltmp3036:
	.loc	26 139 9
	cmovbel	%eax, %r8d
.Ltmp3037:
	.loc	26 161 24
	testb	$1, %r8b
	jne	.LBB21_694
.Ltmp3038:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm7, %xmm5
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm6
.Ltmp3039:
	.loc	26 161 24
	testb	$1, %sil
	je	.LBB21_697
	jmp	.LBB21_698
.Ltmp3040:
	.loc	26 0 24
.Ltmp3041:
	.p2align	4
.LBB21_694:
	vaddss	%xmm6, %xmm7, %xmm5
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm6
.Ltmp3042:
	.loc	26 161 24
	testb	$1, %sil
	jne	.LBB21_698
.Ltmp3043:
.LBB21_697:
	.loc	26 0 24
	vmovaps	%xmm5, %xmm6
.LBB21_698:
	vmovaps	%xmm6, %xmm11
	vmovss	%xmm4, 192(%rsp)
.Ltmp3044:
	vmulss	.LCPI21_18(%rip), %xmm8, %xmm5
	vmaxss	.LCPI21_19(%rip), %xmm5, %xmm5
	vminss	.LCPI21_20(%rip), %xmm5, %xmm5
	vroundss	$9, %xmm5, %xmm5, %xmm6
	vsubss	%xmm6, %xmm5, %xmm5
.Ltmp3045:
	orl	%esi, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm9
	vmovaps	%xmm11, %xmm13
.Ltmp3046:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm11, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp3047:
	.loc	26 66 9
	vsubss	32(%rsp), %xmm0, %xmm0
.Ltmp3048:
	.loc	26 71 9
	vmulss	88(%rsp), %xmm0, %xmm0
.Ltmp3049:
	.loc	26 161 24
	vmaxss	304(%rsp), %xmm0, %xmm0
.Ltmp3050:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm9, %xmm15, %xmm9
	vcmpltss	%xmm15, %xmm0, %xmm11
	vandps	%xmm11, %xmm9, %xmm9
	vmovd	%xmm9, %esi
	testb	$1, %sil
	jne	.LBB21_700
.Ltmp3051:
	.loc	26 0 44
	vxorps	%xmm0, %xmm0, %xmm0
	jmp	.LBB21_700
.LBB21_511:
	movq	%r15, %rax
	subq	%rsi, %rax
	cmpq	%rbp, %r9
	jbe	.LBB21_528
	movq	%rax, 216(%rsp)
	movq	288(%rsp), %rax
.Ltmp3052:
	.loc	25 438 16 is_stmt 1
	negq	%rax
	movq	%rax, 352(%rsp)
	movq	280(%rsp), %rax
	negq	%rax
	movq	%rax, 296(%rsp)
	movq	%r11, %rax
	movq	144(%rsp), %r8
	movl	%r8d, %r11d
	subl	%r10d, %r11d
	movl	%r8d, %r10d
	subl	%r14d, %r10d
	movl	%r8d, %r9d
	subl	%edi, %r9d
	movq	%rsi, %r15
	subq	%rax, %r15
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm2
	vmovss	.LCPI21_4(%rip), %xmm9
	vmovss	.LCPI21_5(%rip), %xmm12
	xorl	%eax, %eax
	vxorps	%xmm15, %xmm15, %xmm15
	vmovss	.LCPI21_23(%rip), %xmm14
	vmovss	.LCPI21_24(%rip), %xmm10
	jmp	.LBB21_513
.Ltmp3053:
	.loc	25 0 16 is_stmt 0
.Ltmp3054:
	.p2align	4
.LBB21_671:
	vmovaps	176(%rsp), %xmm2
.Ltmp3055:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm0
	movl	76(%rsp), %edx
.Ltmp3056:
	.loc	26 161 24
	cmovbel	72(%rsp), %edx
.Ltmp3057:
	.loc	7 1291 18
	vmovd	%edx, %xmm9
.Ltmp3058:
	.loc	26 66 9
	vsubss	%xmm2, %xmm0, %xmm0
.Ltmp3059:
	.loc	26 92 9
	vmulss	%xmm0, %xmm9, %xmm0
	vaddss	%xmm0, %xmm2, %xmm0
.Ltmp3060:
	.loc	26 103 24
	vandps	%xmm0, %xmm13, %xmm2
.Ltmp3061:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm2, %xmm2
	vandps	%xmm0, %xmm2, %xmm2
	vmovss	.LCPI21_21(%rip), %xmm9
.Ltmp3062:
	.loc	26 71 9
	vmulss	%xmm5, %xmm9, %xmm0
	vmovss	.LCPI21_22(%rip), %xmm11
.Ltmp3063:
	.loc	26 61 9
	vaddss	%xmm0, %xmm11, %xmm0
.Ltmp3064:
	.loc	26 71 9
	vmulss	%xmm0, %xmm5, %xmm0
.Ltmp3065:
	.loc	26 61 9
	vaddss	%xmm0, %xmm14, %xmm0
.Ltmp3066:
	.loc	26 71 9
	vmulss	%xmm0, %xmm5, %xmm0
.Ltmp3067:
	.loc	26 61 9
	vaddss	%xmm0, %xmm10, %xmm0
.Ltmp3068:
	.loc	26 71 9
	vmulss	%xmm0, %xmm5, %xmm0
	vmovss	.LCPI21_25(%rip), %xmm13
.Ltmp3069:
	.loc	26 61 9
	vaddss	%xmm0, %xmm13, %xmm0
.Ltmp3070:
	.loc	26 71 9
	vmulss	%xmm0, %xmm5, %xmm0
.Ltmp3071:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI21_18(%rip), %xmm2, %xmm5
.Ltmp3072:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI21_19(%rip), %xmm5, %xmm5
.Ltmp3073:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm5, %xmm5
	vmovss	.LCPI21_0(%rip), %xmm4
.Ltmp3074:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm4, %xmm0, %xmm0
	vmovss	.LCPI21_26(%rip), %xmm8
.Ltmp3075:
	.loc	26 178 22
	vaddss	%xmm6, %xmm8, %xmm6
.Ltmp3076:
	.loc	7 1244 18
	vmovd	%xmm6, %edx
.Ltmp3077:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp3078:
	.loc	7 1291 18
	vmovd	%edx, %xmm6
.Ltmp3079:
	.loc	26 71 9
	vmulss	%xmm6, %xmm0, %xmm0
.Ltmp3080:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm0, %xmm3, %xmm0
	vmovaps	%xmm12, %xmm7
.Ltmp3081:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm15, %xmm12, %xmm6
	vblendvps	%xmm6, %xmm0, %xmm3, %xmm0
	vcmpnltss	108(%rsp), %xmm15, %xmm6
	vblendvps	%xmm6, %xmm0, %xmm3, %xmm0
.Ltmp3082:
	.loc	7 1783 9
	vroundss	$9, %xmm5, %xmm5, %xmm3
.Ltmp3083:
	.loc	26 66 9
	vsubss	%xmm3, %xmm5, %xmm5
.Ltmp3084:
	.loc	26 71 9
	vmulss	%xmm5, %xmm9, %xmm6
.Ltmp3085:
	.loc	26 61 9
	vaddss	%xmm6, %xmm11, %xmm6
.Ltmp3086:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp3087:
	.loc	26 61 9
	vaddss	%xmm6, %xmm14, %xmm6
.Ltmp3088:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp3089:
	.loc	26 61 9
	vaddss	%xmm6, %xmm10, %xmm6
.Ltmp3090:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm6
.Ltmp3091:
	.loc	26 61 9
	vaddss	%xmm6, %xmm13, %xmm6
.Ltmp3092:
	.loc	26 71 9
	vmulss	%xmm6, %xmm5, %xmm5
.Ltmp3093:
	.loc	26 61 9
	vaddss	%xmm4, %xmm5, %xmm5
.Ltmp3094:
	.loc	26 178 22
	vaddss	%xmm3, %xmm8, %xmm3
.Ltmp3095:
	.loc	7 1244 18
	vmovd	%xmm3, %edx
.Ltmp3096:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp3097:
	.loc	7 1291 18
	vmovd	%edx, %xmm3
.Ltmp3098:
	.loc	26 71 9
	vmulss	%xmm3, %xmm5, %xmm3
.Ltmp3099:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm1, %xmm3
.Ltmp3100:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm2, %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm1, %xmm3
	vcmpnltss	68(%rsp), %xmm15, %xmm5
	vblendvps	%xmm5, %xmm3, %xmm1, %xmm1
.Ltmp3101:
	.loc	26 56 9
	vmovss	%xmm0, -4(%r12,%r14,4)
	movq	16(%rsp), %rdx
.Ltmp3102:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm1, -4(%rdx,%r14,4)
	vmovaps	%xmm2, 176(%rsp)
.Ltmp3103:
	.loc	21 394 5 is_stmt 1
	vmovss	%xmm2, 940(%rbx)
.Ltmp3104:
	.loc	8 1916 50
	leaq	(%r15,%r14), %rdx
	incq	%rdx
	incq	%r14
	cmpq	$1, %rdx
	movq	144(%rsp), %r8
	vmovss	132(%rsp), %xmm11
	vmovss	.LCPI21_4(%rip), %xmm9
	vmovss	.LCPI21_5(%rip), %xmm12
	vmovss	.LCPI21_3(%rip), %xmm2
	vmovss	192(%rsp), %xmm4
.Ltmp3105:
	.loc	11 900 12
	je	.LBB21_587
.LBB21_513:
	.loc	11 0 12 is_stmt 0
	movq	224(%rsp), %rdx
.Ltmp3106:
	.loc	15 971 17 is_stmt 1
	leaq	(%rdx,%r14), %rdi
.Ltmp3107:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_561
.Ltmp3108:
	.loc	21 0 0 is_stmt 0
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	movq	256(%rsp), %r13
.Ltmp3109:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %r13
	jbe	.LBB21_565
.Ltmp3110:
	.loc	26 51 9
	vmovss	-4(%r12,%r14,4), %xmm0
	movq	24(%rsp), %rdx
.Ltmp3111:
	.loc	26 56 9
	vmovss	%xmm0, (%rdx,%rdi,4)
.Ltmp3112:
	.loc	25 451 16
	cmpq	%rdi, %rbp
	movq	56(%rsp), %rsi
	jbe	.LBB21_714
.Ltmp3113:
	.loc	25 0 16 is_stmt 0
	movq	296(%rsp), %rdx
	leaq	(%rdx,%r14), %r8
	movq	16(%rsp), %rdx
.Ltmp3114:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rdx,%r14,4), %xmm0
	movq	40(%rsp), %rdx
.Ltmp3115:
	.loc	26 56 9
	vmovss	%xmm0, (%rdx,%rdi,4)
.Ltmp3116:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_674
.Ltmp3117:
	.loc	25 0 16 is_stmt 0
	movq	160(%rsp), %rdx
.Ltmp3118:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_637
.Ltmp3119:
	.loc	25 0 16 is_stmt 0
	movq	352(%rsp), %r8
	addq	%r14, %r8
	movq	152(%rsp), %r12
.Ltmp3120:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r14,4), %xmm0
	movq	(%rsp), %r12
.Ltmp3121:
	.loc	26 56 9
	vmovss	%xmm0, (%r12,%rdi,4)
.Ltmp3122:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_677
.Ltmp3123:
	.loc	25 451 16
	cmpq	%rdi, %rsi
	jbe	.LBB21_715
.Ltmp3124:
	.loc	25 0 16 is_stmt 0
	movq	232(%rsp), %r8
.Ltmp3125:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r8,%r14,4), %xmm0
	movq	8(%rsp), %r8
.Ltmp3126:
	.loc	26 56 9
	vmovss	%xmm0, (%r8,%rdi,4)
.Ltmp3127:
	.loc	21 255 21
	leal	(%r9,%r14), %r12d
	decl	%r12d
	andl	%ecx, %r12d
.Ltmp3128:
	.loc	25 438 16
	cmpq	%r12, %r13
	jbe	.LBB21_568
.Ltmp3129:
	.loc	25 438 16 is_stmt 0
	cmpq	%r12, %rbp
	jbe	.LBB21_681
.Ltmp3130:
	.loc	25 0 16
	leal	(%r10,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	cmpq	%rdi, %rdx
	jbe	.LBB21_704
	cmpq	%rdi, %rsi
	vmovss	.LCPI21_1(%rip), %xmm6
	vbroadcastss	.LCPI21_2(%rip), %xmm13
.Ltmp3131:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB21_703
	.loc	21 0 29 is_stmt 0
	leal	(%r11,%r14), %r8d
	decl	%r8d
	andl	%ecx, %r8d
	cmpq	%r8, %rsi
	jbe	.LBB21_702
	cmpq	%r8, %rdx
	.loc	21 277 29 is_stmt 1
	jbe	.LBB21_701
.Ltmp3132:
	.loc	21 0 29 is_stmt 0
	movq	8(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm0
	movq	(%rsp), %rdx
.Ltmp3133:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm1
	vandps	%xmm1, %xmm13, %xmm1
.Ltmp3134:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm0, %xmm13, %xmm0
.Ltmp3135:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm0, %xmm1
.Ltmp3136:
	.loc	7 1244 18
	vmovd	%xmm1, %edi
.Ltmp3137:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm0, %r13d
.Ltmp3138:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r13d
	vmovss	128(%rsp), %xmm3
	vucomiss	%xmm15, %xmm3
.Ltmp3139:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r13d
	vmovss	124(%rsp), %xmm3
	vucomiss	%xmm15, %xmm3
.Ltmp3140:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm2, %xmm1, %xmm1
.Ltmp3141:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm0, %xmm0
.Ltmp3142:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm1, %xmm0, %xmm0
.Ltmp3143:
	.loc	7 1244 18
	vmovd	%xmm0, %edi
.Ltmp3144:
	.loc	26 161 24
	cmovbel	%r13d, %edi
.Ltmp3145:
	.loc	7 1291 18
	vmovd	%edi, %xmm0
.Ltmp3146:
	.loc	26 124 14
	vucomiss	%xmm9, %xmm0
.Ltmp3147:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %edi
.Ltmp3148:
	.loc	7 1291 18
	vmovd	%edi, %xmm0
.Ltmp3149:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm0
.Ltmp3150:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %edi
.Ltmp3151:
	.loc	26 185 42
	movl	%edi, %r13d
	andl	$8388607, %r13d
	orl	$1065353216, %r13d
.Ltmp3152:
	.loc	7 1291 18
	vmovd	%r13d, %xmm0
.Ltmp3153:
	.loc	26 66 9
	vaddss	%xmm6, %xmm0, %xmm0
.Ltmp3154:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm0, %xmm1
.Ltmp3155:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm3
	vsubss	%xmm1, %xmm3, %xmm1
.Ltmp3156:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp3157:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm1, %xmm1
.Ltmp3158:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp3159:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm1, %xmm1
.Ltmp3160:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp3161:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm1, %xmm1
.Ltmp3162:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp3163:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm1, %xmm1
.Ltmp3164:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3165:
	.loc	26 71 9
	vmulss	%xmm1, %xmm0, %xmm0
.Ltmp3166:
	.loc	7 1291 18
	vmovd	%edi, %xmm1
.Ltmp3167:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm1, %xmm1
.Ltmp3168:
	.loc	26 61 9
	vaddss	%xmm0, %xmm1, %xmm0
.Ltmp3169:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm0, %xmm0
.Ltmp3170:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm0, %xmm0
.Ltmp3171:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm0, %xmm0
.Ltmp3172:
	.loc	26 129 14 is_stmt 1
	vucomiss	120(%rsp), %xmm0
.Ltmp3173:
	.loc	26 28 5
	movl	$0, %edi
	adcl	$-1, %edi
.Ltmp3174:
	.loc	26 129 14
	vucomiss	%xmm11, %xmm0
.Ltmp3175:
	.loc	26 144 9
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp3176:
	.loc	26 149 9
	movl	%edi, %esi
.Ltmp3177:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm4
.Ltmp3178:
	.loc	26 149 9
	notl	%esi
.Ltmp3179:
	.loc	26 139 9
	cmovbel	%eax, %esi
.Ltmp3180:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm1
.Ltmp3181:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm1
.Ltmp3182:
	.loc	26 144 9
	cmoval	%edi, %r13d
.Ltmp3183:
	.loc	26 139 9
	cmovbel	%eax, %esi
.Ltmp3184:
	.loc	26 161 24
	testb	$1, %sil
	jne	.LBB21_527
.Ltmp3185:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm1
	vmovss	116(%rsp), %xmm4
.Ltmp3186:
	.loc	26 161 24
	testb	$1, %r13b
	je	.LBB21_661
	jmp	.LBB21_662
.Ltmp3187:
	.loc	26 0 24
.Ltmp3188:
	.p2align	4
.LBB21_527:
	vaddss	%xmm6, %xmm4, %xmm1
	vmovss	116(%rsp), %xmm4
.Ltmp3189:
	.loc	26 161 24
	testb	$1, %r13b
	jne	.LBB21_662
.Ltmp3190:
.LBB21_661:
	.loc	26 0 24
	vmovaps	%xmm1, %xmm4
.LBB21_662:
	orl	%r13d, %esi
	andl	$1065353216, %esi
	vmovd	%esi, %xmm1
.Ltmp3191:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm11, %xmm0, %xmm0
.Ltmp3192:
	.loc	26 71 9
	vmulss	112(%rsp), %xmm0, %xmm0
.Ltmp3193:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm0, %xmm0
.Ltmp3194:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm1, %xmm15, %xmm1
	vcmpltss	%xmm15, %xmm0, %xmm3
	vandps	%xmm3, %xmm1, %xmm1
	vmovd	%xmm1, %edi
	testb	$1, %dil
	jne	.LBB21_664
.Ltmp3195:
	.loc	26 0 44
	vxorps	%xmm0, %xmm0, %xmm0
.LBB21_664:
.Ltmp3196:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm0
	movl	84(%rsp), %r13d
.Ltmp3197:
	.loc	26 161 24
	cmovbel	80(%rsp), %r13d
	movq	8(%rsp), %rdx
.Ltmp3198:
	.loc	26 103 24
	vmovss	(%rdx,%r8,4), %xmm1
	vandps	%xmm1, %xmm13, %xmm1
	movq	(%rsp), %rdx
.Ltmp3199:
	.loc	26 103 24 is_stmt 0
	vmovss	(%rdx,%r8,4), %xmm3
	vandps	%xmm3, %xmm13, %xmm3
.Ltmp3200:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm3, %xmm1
.Ltmp3201:
	.loc	7 1244 18
	vmovd	%xmm1, %edi
.Ltmp3202:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm3, %edx
.Ltmp3203:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %edx
	vmovss	104(%rsp), %xmm5
	vucomiss	%xmm15, %xmm5
.Ltmp3204:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %edx
	vmovss	96(%rsp), %xmm5
	vucomiss	%xmm15, %xmm5
.Ltmp3205:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm2, %xmm1, %xmm1
.Ltmp3206:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm2, %xmm3, %xmm3
.Ltmp3207:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm1, %xmm3, %xmm1
.Ltmp3208:
	.loc	7 1244 18
	vmovd	%xmm1, %r8d
.Ltmp3209:
	.loc	26 161 24
	cmovbel	%edx, %r8d
.Ltmp3210:
	.loc	7 1291 18
	vmovd	%r8d, %xmm1
.Ltmp3211:
	.loc	26 124 14
	vucomiss	%xmm9, %xmm1
.Ltmp3212:
	.loc	7 1291 18
	vmovd	%r13d, %xmm1
.Ltmp3213:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %r8d
.Ltmp3214:
	.loc	26 66 9
	vsubss	%xmm7, %xmm0, %xmm0
.Ltmp3215:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp3216:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm3
.Ltmp3217:
	.loc	26 92 9
	vmulss	%xmm1, %xmm0, %xmm1
.Ltmp3218:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %r8d
.Ltmp3219:
	.loc	26 185 42
	movl	%r8d, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp3220:
	.loc	7 1291 18
	vmovd	%edx, %xmm0
.Ltmp3221:
	.loc	26 66 9
	vaddss	%xmm6, %xmm0, %xmm0
.Ltmp3222:
	.loc	26 71 9
	vmulss	.LCPI21_6(%rip), %xmm0, %xmm3
.Ltmp3223:
	.loc	26 61 9
	vmovss	.LCPI21_7(%rip), %xmm5
	vsubss	%xmm3, %xmm5, %xmm3
.Ltmp3224:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp3225:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm3, %xmm3
.Ltmp3226:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp3227:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm3, %xmm3
.Ltmp3228:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp3229:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm3, %xmm3
.Ltmp3230:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm3
.Ltmp3231:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm3, %xmm3
.Ltmp3232:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp3233:
	.loc	26 71 9
	vmulss	%xmm3, %xmm0, %xmm0
.Ltmp3234:
	.loc	7 1291 18
	vmovd	%r8d, %xmm3
.Ltmp3235:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm3, %xmm3
.Ltmp3236:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm0
	movq	24(%rsp), %rdx
	vmovss	(%rdx,%r12,4), %xmm3
.Ltmp3237:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm0, %xmm0
.Ltmp3238:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm0, %xmm0
.Ltmp3239:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm0, %xmm0
.Ltmp3240:
	.loc	26 129 14 is_stmt 1
	vucomiss	100(%rsp), %xmm0
.Ltmp3241:
	.loc	26 28 5
	movl	$0, %edx
	adcl	$-1, %edx
.Ltmp3242:
	.loc	26 129 14
	vucomiss	32(%rsp), %xmm0
.Ltmp3243:
	.loc	26 92 9
	vaddss	%xmm1, %xmm7, %xmm1
.Ltmp3244:
	.loc	26 144 9
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp3245:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%rbx), %xmm5
.Ltmp3246:
	.loc	26 149 9 is_stmt 1
	movl	%edx, %r8d
	vmovss	48(%rsp), %xmm8
.Ltmp3247:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm8
.Ltmp3248:
	.loc	26 149 9
	notl	%r8d
.Ltmp3249:
	.loc	26 139 9
	cmovbel	%eax, %r8d
.Ltmp3250:
	.loc	26 124 14
	vucomiss	%xmm15, %xmm5
.Ltmp3251:
	.loc	26 103 24
	vandps	%xmm1, %xmm13, %xmm5
.Ltmp3252:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm5, %xmm5
	vandps	%xmm1, %xmm5, %xmm12
	movq	40(%rsp), %rdi
	vmovss	(%rdi,%r12,4), %xmm1
.Ltmp3253:
	.loc	21 373 5
	vmovss	%xmm4, 860(%rbx)
	.loc	21 382 5
	movl	%esi, 856(%rbx)
.Ltmp3254:
	.loc	21 394 5
	vmovss	%xmm12, 864(%rbx)
.Ltmp3255:
	.loc	26 144 9
	cmoval	%edx, %r13d
.Ltmp3256:
	.loc	26 139 9
	cmovbel	%eax, %r8d
.Ltmp3257:
	.loc	26 161 24
	testb	$1, %r8b
	jne	.LBB21_665
.Ltmp3258:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm8, %xmm5
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm11
.Ltmp3259:
	.loc	26 161 24
	testb	$1, %r13b
	je	.LBB21_668
	jmp	.LBB21_669
.Ltmp3260:
	.loc	26 0 24
.Ltmp3261:
	.p2align	4
.LBB21_665:
	vaddss	%xmm6, %xmm8, %xmm5
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm11
.Ltmp3262:
	.loc	26 161 24
	testb	$1, %r13b
	jne	.LBB21_669
.Ltmp3263:
.LBB21_668:
	.loc	26 0 24
	vmovaps	%xmm5, %xmm11
.LBB21_669:
	vmovss	%xmm4, 192(%rsp)
.Ltmp3264:
	vmulss	.LCPI21_18(%rip), %xmm12, %xmm5
	vmaxss	.LCPI21_19(%rip), %xmm5, %xmm5
	vminss	.LCPI21_20(%rip), %xmm5, %xmm5
	vroundss	$9, %xmm5, %xmm5, %xmm6
	vsubss	%xmm6, %xmm5, %xmm5
.Ltmp3265:
	orl	%r13d, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm9
	vmovss	%xmm11, 48(%rsp)
.Ltmp3266:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm11, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp3267:
	.loc	26 66 9
	vsubss	32(%rsp), %xmm0, %xmm0
.Ltmp3268:
	.loc	26 71 9
	vmulss	88(%rsp), %xmm0, %xmm0
.Ltmp3269:
	.loc	26 161 24
	vmaxss	304(%rsp), %xmm0, %xmm0
.Ltmp3270:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm9, %xmm15, %xmm9
	vcmpltss	%xmm15, %xmm0, %xmm11
	vandps	%xmm11, %xmm9, %xmm9
	vmovd	%xmm9, %edx
	testb	$1, %dl
	jne	.LBB21_671
.Ltmp3271:
	.loc	26 0 44
	vxorps	%xmm0, %xmm0, %xmm0
	jmp	.LBB21_671
.LBB21_495:
	movq	$0, 152(%rsp)
.Ltmp3272:
	.loc	25 580 12 is_stmt 1
	movq	%rsi, %rax
	subq	%r15, %rax
	movq	%rax, 224(%rsp)
	jbe	.LBB21_503
.LBB21_713:
	.loc	25 581 13
	leaq	.Lalloc_e6adad6d678d00eb6b02b8162f35ebb4(%rip), %rcx
	movq	%rsi, %rdi
	movq	%r15, %rsi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3273:
.LBB21_528:
	.loc	25 0 13 is_stmt 0
	movq	280(%rsp), %r13
	cmpq	%r13, %rax
	jbe	.LBB21_532
	movq	288(%rsp), %rax
.Ltmp3274:
	.loc	25 451 16 is_stmt 1
	negq	%rax
	movq	%rax, 224(%rsp)
	negq	%r13
	movq	%r13, 296(%rsp)
	movq	144(%rsp), %r8
	movl	%r8d, %r9d
	subl	%r10d, %r9d
	movl	%r8d, %r10d
	subl	%r14d, %r10d
	movq	%r11, %rdx
	movl	%r8d, %r11d
	subl	%edi, %r11d
	movq	%rsi, %rax
	subq	%rdx, %rax
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm3
	vmovss	.LCPI21_4(%rip), %xmm12
	vmovss	.LCPI21_5(%rip), %xmm5
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_7(%rip), %xmm14
	vmovss	.LCPI21_15(%rip), %xmm13
	xorl	%r13d, %r13d
	vmovss	.LCPI21_19(%rip), %xmm2
	jmp	.LBB21_530
.Ltmp3275:
	.loc	25 0 16 is_stmt 0
.Ltmp3276:
	.p2align	4
.LBB21_658:
	vmovaps	176(%rsp), %xmm2
.Ltmp3277:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm14
	movl	76(%rsp), %edx
.Ltmp3278:
	.loc	26 161 24
	cmovbel	72(%rsp), %edx
.Ltmp3279:
	.loc	7 1291 18
	vmovd	%edx, %xmm0
.Ltmp3280:
	.loc	26 66 9
	vsubss	%xmm2, %xmm14, %xmm14
.Ltmp3281:
	.loc	26 92 9
	vmulss	%xmm0, %xmm14, %xmm0
	vaddss	%xmm0, %xmm2, %xmm0
.Ltmp3282:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm2
	vandps	%xmm2, %xmm0, %xmm2
.Ltmp3283:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm2, %xmm2
	vandps	%xmm0, %xmm2, %xmm14
	vmovss	.LCPI21_21(%rip), %xmm15
.Ltmp3284:
	.loc	26 71 9
	vmulss	%xmm15, %xmm12, %xmm0
	vmovss	.LCPI21_22(%rip), %xmm6
.Ltmp3285:
	.loc	26 61 9
	vaddss	%xmm6, %xmm0, %xmm0
.Ltmp3286:
	.loc	26 71 9
	vmulss	%xmm0, %xmm12, %xmm0
	vmovss	.LCPI21_23(%rip), %xmm1
.Ltmp3287:
	.loc	26 61 9
	vaddss	%xmm1, %xmm0, %xmm0
.Ltmp3288:
	.loc	26 71 9
	vmulss	%xmm0, %xmm12, %xmm0
	vmovss	.LCPI21_24(%rip), %xmm3
.Ltmp3289:
	.loc	26 61 9
	vaddss	%xmm3, %xmm0, %xmm0
.Ltmp3290:
	.loc	26 71 9
	vmulss	%xmm0, %xmm12, %xmm0
	vmovss	.LCPI21_25(%rip), %xmm5
.Ltmp3291:
	.loc	26 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp3292:
	.loc	26 71 9
	vmulss	%xmm0, %xmm12, %xmm0
.Ltmp3293:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI21_18(%rip), %xmm14, %xmm12
	vmovaps	%xmm13, %xmm2
.Ltmp3294:
	.loc	26 161 24 is_stmt 1
	vmaxss	%xmm13, %xmm12, %xmm12
.Ltmp3295:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm12, %xmm12
	vmovss	.LCPI21_0(%rip), %xmm13
.Ltmp3296:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm0, %xmm13, %xmm0
	vmovss	.LCPI21_26(%rip), %xmm4
.Ltmp3297:
	.loc	26 178 22
	vaddss	%xmm4, %xmm11, %xmm11
.Ltmp3298:
	.loc	7 1244 18
	vmovd	%xmm11, %edx
.Ltmp3299:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp3300:
	.loc	7 1291 18
	vmovd	%edx, %xmm11
.Ltmp3301:
	.loc	26 71 9
	vmulss	%xmm0, %xmm11, %xmm0
.Ltmp3302:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm0, %xmm10, %xmm0
.Ltmp3303:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm8, %xmm11
	vblendvps	%xmm11, %xmm0, %xmm10, %xmm0
	vcmpnltss	108(%rsp), %xmm8, %xmm11
	vblendvps	%xmm11, %xmm0, %xmm10, %xmm0
.Ltmp3304:
	.loc	7 1783 9
	vroundss	$9, %xmm12, %xmm12, %xmm10
.Ltmp3305:
	.loc	26 66 9
	vsubss	%xmm10, %xmm12, %xmm11
.Ltmp3306:
	.loc	26 71 9
	vmulss	%xmm15, %xmm11, %xmm12
.Ltmp3307:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm12
.Ltmp3308:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3309:
	.loc	26 61 9
	vaddss	%xmm1, %xmm12, %xmm12
.Ltmp3310:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3311:
	.loc	26 61 9
	vaddss	%xmm3, %xmm12, %xmm12
.Ltmp3312:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3313:
	.loc	26 61 9
	vaddss	%xmm5, %xmm12, %xmm12
.Ltmp3314:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm11
.Ltmp3315:
	.loc	26 61 9
	vaddss	%xmm13, %xmm11, %xmm11
.Ltmp3316:
	.loc	26 178 22
	vaddss	%xmm4, %xmm10, %xmm10
.Ltmp3317:
	.loc	7 1244 18
	vmovd	%xmm10, %edx
.Ltmp3318:
	.loc	26 179 24
	shll	$23, %edx
.Ltmp3319:
	.loc	7 1291 18
	vmovd	%edx, %xmm10
.Ltmp3320:
	.loc	26 71 9
	vmulss	%xmm10, %xmm11, %xmm10
.Ltmp3321:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3322:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm8, %xmm14, %xmm11
	vblendvps	%xmm11, %xmm10, %xmm9, %xmm10
	vcmpnltss	68(%rsp), %xmm8, %xmm8
	vblendvps	%xmm8, %xmm10, %xmm9, %xmm8
.Ltmp3323:
	.loc	26 56 9
	vmovss	%xmm0, -4(%r12,%r14,4)
	movq	16(%rsp), %rdx
.Ltmp3324:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm8, -4(%rdx,%r14,4)
	vmovaps	%xmm14, 176(%rsp)
.Ltmp3325:
	.loc	21 394 5 is_stmt 1
	vmovss	%xmm14, 940(%rbx)
.Ltmp3326:
	.loc	8 1916 50
	leaq	(%rax,%r14), %rdx
	incq	%rdx
	incq	%r14
	cmpq	$1, %rdx
	movq	144(%rsp), %r8
	vmovss	132(%rsp), %xmm11
	vmovss	.LCPI21_4(%rip), %xmm12
	vmovss	.LCPI21_5(%rip), %xmm5
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_7(%rip), %xmm14
	vmovss	.LCPI21_3(%rip), %xmm3
	vmovss	.LCPI21_15(%rip), %xmm13
	vmovss	192(%rsp), %xmm4
.Ltmp3327:
	.loc	11 900 12
	je	.LBB21_587
.Ltmp3328:
.LBB21_530:
	.loc	21 246 22
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
.Ltmp3329:
	.loc	25 451 16
	cmpq	%rdi, 256(%rsp)
	jbe	.LBB21_531
.Ltmp3330:
	.loc	25 0 16 is_stmt 0
	movq	296(%rsp), %rdx
	leaq	(%rdx,%r14), %r8
.Ltmp3331:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r14,4), %xmm8
	movq	24(%rsp), %rdx
.Ltmp3332:
	.loc	26 56 9
	vmovss	%xmm8, (%rdx,%rdi,4)
	movq	16(%rsp), %rdx
.Ltmp3333:
	.loc	26 51 9
	vmovss	-4(%rdx,%r14,4), %xmm8
	movq	40(%rsp), %rdx
.Ltmp3334:
	.loc	26 56 9
	vmovss	%xmm8, (%rdx,%rdi,4)
.Ltmp3335:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_674
.Ltmp3336:
	.loc	25 451 16
	cmpq	%rdi, 160(%rsp)
	jbe	.LBB21_636
.Ltmp3337:
	.loc	25 0 16 is_stmt 0
	movq	224(%rsp), %rdx
	leaq	(%rdx,%r14), %r8
	movq	152(%rsp), %rdx
.Ltmp3338:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rdx,%r14,4), %xmm8
	movq	(%rsp), %rdx
.Ltmp3339:
	.loc	26 56 9
	vmovss	%xmm8, (%rdx,%rdi,4)
.Ltmp3340:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_677
.Ltmp3341:
	.loc	25 0 16 is_stmt 0
	movq	56(%rsp), %rdx
.Ltmp3342:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rdx
	jbe	.LBB21_715
.Ltmp3343:
	.loc	25 0 16 is_stmt 0
	movq	232(%rsp), %rsi
.Ltmp3344:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rsi,%r14,4), %xmm8
	movq	8(%rsp), %rsi
.Ltmp3345:
	.loc	26 56 9
	vmovss	%xmm8, (%rsi,%rdi,4)
.Ltmp3346:
	.loc	21 255 21
	leal	(%r11,%r14), %r12d
	decl	%r12d
	andl	%ecx, %r12d
.Ltmp3347:
	.loc	25 438 16
	cmpq	%r12, 256(%rsp)
	jbe	.LBB21_567
.Ltmp3348:
	.loc	25 0 16 is_stmt 0
	leal	(%r10,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	cmpq	%rdi, 160(%rsp)
	jbe	.LBB21_704
	cmpq	%rdi, %rdx
.Ltmp3349:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB21_703
	.loc	21 0 29 is_stmt 0
	leal	(%r9,%r14), %r8d
	decl	%r8d
	andl	%ecx, %r8d
	cmpq	%r8, %rdx
	jbe	.LBB21_702
	cmpq	%r8, 160(%rsp)
	.loc	21 277 29 is_stmt 1
	jbe	.LBB21_701
.Ltmp3350:
	.loc	21 0 29 is_stmt 0
	movq	8(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm8
	movq	(%rsp), %rdx
.Ltmp3351:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm9
	vbroadcastss	.LCPI21_2(%rip), %xmm0
	vandps	%xmm0, %xmm9, %xmm9
.Ltmp3352:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm0, %xmm8, %xmm10
.Ltmp3353:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm9
.Ltmp3354:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp3355:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm10, %ebp
.Ltmp3356:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
	vxorps	%xmm8, %xmm8, %xmm8
	vmovss	128(%rsp), %xmm0
	vucomiss	%xmm8, %xmm0
.Ltmp3357:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %ebp
	vmovss	124(%rsp), %xmm0
	vucomiss	%xmm8, %xmm0
.Ltmp3358:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp3359:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm10, %xmm10
.Ltmp3360:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp3361:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp3362:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3363:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3364:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm9
.Ltmp3365:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %edi
.Ltmp3366:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3367:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm9
.Ltmp3368:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %edi
.Ltmp3369:
	.loc	26 185 42
	movl	%edi, %ebp
	andl	$8388607, %ebp
	orl	$1065353216, %ebp
.Ltmp3370:
	.loc	7 1291 18
	vmovd	%ebp, %xmm9
	vmovss	.LCPI21_1(%rip), %xmm0
.Ltmp3371:
	.loc	26 66 9
	vaddss	%xmm0, %xmm9, %xmm9
.Ltmp3372:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm10
.Ltmp3373:
	.loc	26 61 9
	vsubss	%xmm10, %xmm14, %xmm10
.Ltmp3374:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3375:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm10, %xmm10
.Ltmp3376:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3377:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm10, %xmm10
.Ltmp3378:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3379:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm10, %xmm10
.Ltmp3380:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3381:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm10, %xmm10
.Ltmp3382:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3383:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp3384:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp3385:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm10, %xmm10
.Ltmp3386:
	.loc	26 61 9
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp3387:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm9, %xmm9
.Ltmp3388:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm9, %xmm9
.Ltmp3389:
	.loc	26 161 24 is_stmt 0
	vmaxss	%xmm13, %xmm9, %xmm9
.Ltmp3390:
	.loc	26 129 14 is_stmt 1
	vucomiss	120(%rsp), %xmm9
.Ltmp3391:
	.loc	26 28 5
	movl	$0, %edi
	adcl	$-1, %edi
.Ltmp3392:
	.loc	26 129 14
	vucomiss	%xmm11, %xmm9
.Ltmp3393:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3394:
	.loc	26 149 9
	movl	%edi, %r15d
.Ltmp3395:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm4
.Ltmp3396:
	.loc	26 149 9
	notl	%r15d
.Ltmp3397:
	.loc	26 139 9
	cmovbel	%r13d, %r15d
.Ltmp3398:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm10
.Ltmp3399:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm10
.Ltmp3400:
	.loc	26 144 9
	cmoval	%edi, %ebp
.Ltmp3401:
	.loc	26 139 9
	cmovbel	%r13d, %r15d
.Ltmp3402:
	.loc	26 161 24
	testb	$1, %r15b
	jne	.LBB21_646
.Ltmp3403:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp3404:
	.loc	26 161 24
	testb	$1, %bpl
	je	.LBB21_649
	jmp	.LBB21_650
.Ltmp3405:
	.loc	26 0 24
.Ltmp3406:
	.p2align	4
.LBB21_646:
	vaddss	%xmm0, %xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp3407:
	.loc	26 161 24
	testb	$1, %bpl
	jne	.LBB21_650
.Ltmp3408:
.LBB21_649:
	.loc	26 0 24
	vmovaps	%xmm10, %xmm4
.LBB21_650:
	orl	%ebp, %r15d
	andl	$1065353216, %r15d
	vmovd	%r15d, %xmm10
.Ltmp3409:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm11, %xmm9, %xmm9
.Ltmp3410:
	.loc	26 71 9
	vmulss	112(%rsp), %xmm9, %xmm9
.Ltmp3411:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm9, %xmm9
.Ltmp3412:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm10, %xmm8, %xmm10
	vcmpltss	%xmm8, %xmm9, %xmm11
	vandps	%xmm11, %xmm10, %xmm10
	vmovd	%xmm10, %edi
	testb	$1, %dil
	jne	.LBB21_652
.Ltmp3413:
	.loc	26 0 44
	vxorps	%xmm9, %xmm9, %xmm9
.LBB21_652:
.Ltmp3414:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm9
	movl	84(%rsp), %ebp
.Ltmp3415:
	.loc	26 161 24
	cmovbel	80(%rsp), %ebp
	movq	8(%rsp), %rdx
.Ltmp3416:
	.loc	26 103 24
	vmovss	(%rdx,%r8,4), %xmm10
	vbroadcastss	.LCPI21_2(%rip), %xmm1
	vandps	%xmm1, %xmm10, %xmm10
	movq	(%rsp), %rdx
.Ltmp3417:
	.loc	26 103 24 is_stmt 0
	vmovss	(%rdx,%r8,4), %xmm11
	vandps	%xmm1, %xmm11, %xmm11
.Ltmp3418:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm11, %xmm10
.Ltmp3419:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp3420:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm11, %edx
.Ltmp3421:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %edx
	vmovss	104(%rsp), %xmm0
	vucomiss	%xmm8, %xmm0
.Ltmp3422:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %edx
	vmovss	96(%rsp), %xmm0
	vucomiss	%xmm8, %xmm0
.Ltmp3423:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm10, %xmm10
.Ltmp3424:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm11, %xmm11
.Ltmp3425:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm11, %xmm10
.Ltmp3426:
	.loc	7 1244 18
	vmovd	%xmm10, %r8d
.Ltmp3427:
	.loc	26 161 24
	cmovbel	%edx, %r8d
.Ltmp3428:
	.loc	7 1291 18
	vmovd	%r8d, %xmm10
.Ltmp3429:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm10
.Ltmp3430:
	.loc	7 1291 18
	vmovd	%ebp, %xmm10
.Ltmp3431:
	.loc	26 161 24
	movl	$841731191, %edx
	cmovbel	%edx, %r8d
.Ltmp3432:
	.loc	26 66 9
	vsubss	%xmm7, %xmm9, %xmm9
.Ltmp3433:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp3434:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm11
.Ltmp3435:
	.loc	26 92 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp3436:
	.loc	26 161 24
	movl	$8388608, %edx
	cmovbel	%edx, %r8d
.Ltmp3437:
	.loc	26 185 42
	movl	%r8d, %edx
	andl	$8388607, %edx
	orl	$1065353216, %edx
.Ltmp3438:
	.loc	7 1291 18
	vmovd	%edx, %xmm10
	vmovss	.LCPI21_1(%rip), %xmm0
.Ltmp3439:
	.loc	26 66 9
	vaddss	%xmm0, %xmm10, %xmm10
.Ltmp3440:
	.loc	26 71 9
	vmulss	%xmm6, %xmm10, %xmm11
.Ltmp3441:
	.loc	26 61 9
	vsubss	%xmm11, %xmm14, %xmm11
.Ltmp3442:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3443:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm11, %xmm11
.Ltmp3444:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3445:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm11, %xmm11
.Ltmp3446:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3447:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm11, %xmm11
.Ltmp3448:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3449:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm11, %xmm11
.Ltmp3450:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp3451:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm10
.Ltmp3452:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp3453:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm11, %xmm11
.Ltmp3454:
	.loc	26 61 9
	vaddss	%xmm10, %xmm11, %xmm11
	movq	24(%rsp), %rdx
	vmovss	(%rdx,%r12,4), %xmm10
.Ltmp3455:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm11, %xmm11
.Ltmp3456:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm11, %xmm11
.Ltmp3457:
	.loc	26 161 24 is_stmt 0
	vmaxss	%xmm13, %xmm11, %xmm14
.Ltmp3458:
	.loc	26 129 14 is_stmt 1
	vucomiss	100(%rsp), %xmm14
.Ltmp3459:
	.loc	26 28 5
	movl	$0, %edx
	adcl	$-1, %edx
.Ltmp3460:
	.loc	26 129 14
	vucomiss	32(%rsp), %xmm14
.Ltmp3461:
	.loc	26 92 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp3462:
	.loc	26 144 9
	movl	$0, %ebp
	adcl	$-1, %ebp
.Ltmp3463:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%rbx), %xmm9
.Ltmp3464:
	.loc	26 149 9 is_stmt 1
	movl	%edx, %r8d
	vmovss	48(%rsp), %xmm11
.Ltmp3465:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm11
.Ltmp3466:
	.loc	26 149 9
	notl	%r8d
.Ltmp3467:
	.loc	26 139 9
	cmovbel	%r13d, %r8d
.Ltmp3468:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm9
.Ltmp3469:
	.loc	26 103 24
	vandps	%xmm1, %xmm7, %xmm9
.Ltmp3470:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm9, %xmm9
	vandps	%xmm7, %xmm9, %xmm7
	movq	40(%rsp), %rsi
	vmovss	(%rsi,%r12,4), %xmm9
.Ltmp3471:
	.loc	21 373 5
	vmovss	%xmm4, 860(%rbx)
	.loc	21 382 5
	movl	%r15d, 856(%rbx)
.Ltmp3472:
	.loc	21 394 5
	vmovss	%xmm7, 864(%rbx)
.Ltmp3473:
	.loc	26 144 9
	cmoval	%edx, %ebp
.Ltmp3474:
	.loc	26 139 9
	cmovbel	%r13d, %r8d
.Ltmp3475:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_654
.Ltmp3476:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm0, %xmm11, %xmm11
.LBB21_654:
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm0
.Ltmp3477:
	.loc	26 161 24 is_stmt 1
	testb	$1, %bpl
	jne	.LBB21_656
.Ltmp3478:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm0
.LBB21_656:
	vmovss	%xmm4, 192(%rsp)
.Ltmp3479:
	vmulss	.LCPI21_18(%rip), %xmm7, %xmm11
	vmovaps	%xmm2, %xmm13
	vmaxss	%xmm2, %xmm11, %xmm11
	vminss	.LCPI21_20(%rip), %xmm11, %xmm12
	vroundss	$9, %xmm12, %xmm12, %xmm11
	vsubss	%xmm11, %xmm12, %xmm12
.Ltmp3480:
	orl	%ebp, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm15
	vmovss	%xmm0, 48(%rsp)
.Ltmp3481:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm0, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp3482:
	.loc	26 66 9
	vsubss	32(%rsp), %xmm14, %xmm14
.Ltmp3483:
	.loc	26 71 9
	vmulss	88(%rsp), %xmm14, %xmm14
.Ltmp3484:
	.loc	26 161 24
	vmaxss	304(%rsp), %xmm14, %xmm14
.Ltmp3485:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm15, %xmm8, %xmm15
	vcmpltss	%xmm8, %xmm14, %xmm0
	vandps	%xmm0, %xmm15, %xmm0
	vmovd	%xmm0, %edx
	testb	$1, %dl
	jne	.LBB21_658
.Ltmp3486:
	.loc	26 0 44
	vxorps	%xmm14, %xmm14, %xmm14
	jmp	.LBB21_658
.LBB21_532:
	movq	%rax, 216(%rsp)
	cmpq	%rdx, %r9
	jbe	.LBB21_546
	movq	288(%rsp), %rax
.Ltmp3487:
	.loc	25 438 16 is_stmt 1
	negq	%rax
	movq	%rax, 192(%rsp)
	movq	144(%rsp), %r8
	movl	%r8d, %eax
	subl	%r10d, %eax
	movq	%rax, 296(%rsp)
	movl	%r8d, %r9d
	subl	%r14d, %r9d
	movl	%r8d, %r10d
	subl	%edi, %r10d
	movq	%rsi, %rax
	subq	%r11, %rax
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm3
	vmovss	.LCPI21_4(%rip), %xmm12
	movl	$841731191, %ebp
	vmovss	.LCPI21_5(%rip), %xmm5
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_7(%rip), %xmm14
	vmovss	.LCPI21_15(%rip), %xmm13
	xorl	%r15d, %r15d
	jmp	.LBB21_534
.Ltmp3488:
	.loc	25 0 16 is_stmt 0
.Ltmp3489:
	.p2align	4
.LBB21_633:
	vmovaps	176(%rsp), %xmm2
.Ltmp3490:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm14
	movl	76(%rsp), %edi
.Ltmp3491:
	.loc	26 161 24
	cmovbel	72(%rsp), %edi
.Ltmp3492:
	.loc	7 1291 18
	vmovd	%edi, %xmm0
.Ltmp3493:
	.loc	26 66 9
	vsubss	%xmm2, %xmm14, %xmm14
.Ltmp3494:
	.loc	26 92 9
	vmulss	%xmm0, %xmm14, %xmm0
	vaddss	%xmm0, %xmm2, %xmm0
.Ltmp3495:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm2
	vandps	%xmm2, %xmm0, %xmm2
.Ltmp3496:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm2, %xmm2
	vandps	%xmm0, %xmm2, %xmm14
	vmovss	.LCPI21_21(%rip), %xmm15
.Ltmp3497:
	.loc	26 71 9
	vmulss	%xmm15, %xmm12, %xmm0
	vmovss	.LCPI21_22(%rip), %xmm6
.Ltmp3498:
	.loc	26 61 9
	vaddss	%xmm6, %xmm0, %xmm0
.Ltmp3499:
	.loc	26 71 9
	vmulss	%xmm0, %xmm12, %xmm0
	vmovss	.LCPI21_23(%rip), %xmm1
.Ltmp3500:
	.loc	26 61 9
	vaddss	%xmm1, %xmm0, %xmm0
.Ltmp3501:
	.loc	26 71 9
	vmulss	%xmm0, %xmm12, %xmm0
	vmovss	.LCPI21_24(%rip), %xmm3
.Ltmp3502:
	.loc	26 61 9
	vaddss	%xmm3, %xmm0, %xmm0
.Ltmp3503:
	.loc	26 71 9
	vmulss	%xmm0, %xmm12, %xmm0
	vmovss	.LCPI21_25(%rip), %xmm5
.Ltmp3504:
	.loc	26 61 9
	vaddss	%xmm5, %xmm0, %xmm0
.Ltmp3505:
	.loc	26 71 9
	vmulss	%xmm0, %xmm12, %xmm0
.Ltmp3506:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI21_18(%rip), %xmm14, %xmm12
.Ltmp3507:
	.loc	26 161 24 is_stmt 1
	vmaxss	%xmm13, %xmm12, %xmm12
.Ltmp3508:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm12, %xmm12
	vmovss	.LCPI21_0(%rip), %xmm13
.Ltmp3509:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm0, %xmm13, %xmm0
	vmovss	.LCPI21_26(%rip), %xmm2
.Ltmp3510:
	.loc	26 178 22
	vaddss	%xmm2, %xmm11, %xmm11
.Ltmp3511:
	.loc	7 1244 18
	vmovd	%xmm11, %edi
.Ltmp3512:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3513:
	.loc	7 1291 18
	vmovd	%edi, %xmm11
.Ltmp3514:
	.loc	26 71 9
	vmulss	%xmm0, %xmm11, %xmm0
.Ltmp3515:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm0, %xmm10, %xmm0
.Ltmp3516:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm8, %xmm11
	vblendvps	%xmm11, %xmm0, %xmm10, %xmm0
	vcmpnltss	108(%rsp), %xmm8, %xmm11
	vblendvps	%xmm11, %xmm0, %xmm10, %xmm0
.Ltmp3517:
	.loc	7 1783 9
	vroundss	$9, %xmm12, %xmm12, %xmm10
.Ltmp3518:
	.loc	26 66 9
	vsubss	%xmm10, %xmm12, %xmm11
.Ltmp3519:
	.loc	26 71 9
	vmulss	%xmm15, %xmm11, %xmm12
.Ltmp3520:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm12
.Ltmp3521:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3522:
	.loc	26 61 9
	vaddss	%xmm1, %xmm12, %xmm12
.Ltmp3523:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3524:
	.loc	26 61 9
	vaddss	%xmm3, %xmm12, %xmm12
.Ltmp3525:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3526:
	.loc	26 61 9
	vaddss	%xmm5, %xmm12, %xmm12
.Ltmp3527:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm11
.Ltmp3528:
	.loc	26 61 9
	vaddss	%xmm13, %xmm11, %xmm11
.Ltmp3529:
	.loc	26 178 22
	vaddss	%xmm2, %xmm10, %xmm10
.Ltmp3530:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp3531:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3532:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp3533:
	.loc	26 71 9
	vmulss	%xmm10, %xmm11, %xmm10
.Ltmp3534:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3535:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm8, %xmm14, %xmm11
	vblendvps	%xmm11, %xmm10, %xmm9, %xmm10
	vcmpnltss	68(%rsp), %xmm8, %xmm8
	vblendvps	%xmm8, %xmm10, %xmm9, %xmm8
.Ltmp3536:
	.loc	26 56 9
	vmovss	%xmm0, -4(%r12,%r14,4)
	movq	16(%rsp), %rdx
.Ltmp3537:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm8, -4(%rdx,%r14,4)
	vmovaps	%xmm14, 176(%rsp)
.Ltmp3538:
	.loc	21 394 5 is_stmt 1
	vmovss	%xmm14, 940(%rbx)
.Ltmp3539:
	.loc	8 1916 50
	leaq	(%rax,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	movq	144(%rsp), %r8
	vmovss	132(%rsp), %xmm11
	vmovss	.LCPI21_4(%rip), %xmm12
	vmovss	.LCPI21_5(%rip), %xmm5
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_7(%rip), %xmm14
	vmovss	.LCPI21_3(%rip), %xmm3
	vmovss	.LCPI21_15(%rip), %xmm13
.Ltmp3540:
	.loc	11 900 12
	je	.LBB21_587
.LBB21_534:
	.loc	11 0 12 is_stmt 0
	movq	224(%rsp), %rdx
.Ltmp3541:
	.loc	15 971 17 is_stmt 1
	leaq	(%rdx,%r14), %rdi
.Ltmp3542:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_561
.Ltmp3543:
	.loc	21 0 0 is_stmt 0
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	movq	256(%rsp), %r13
.Ltmp3544:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %r13
	jbe	.LBB21_565
.Ltmp3545:
	.loc	26 51 9
	vmovss	-4(%r12,%r14,4), %xmm8
	movq	24(%rsp), %rdx
.Ltmp3546:
	.loc	26 56 9
	vmovss	%xmm8, (%rdx,%rdi,4)
	movq	16(%rsp), %rdx
.Ltmp3547:
	.loc	26 51 9
	vmovss	-4(%rdx,%r14,4), %xmm8
	movq	40(%rsp), %rdx
.Ltmp3548:
	.loc	26 56 9
	vmovss	%xmm8, (%rdx,%rdi,4)
	movq	160(%rsp), %rdx
.Ltmp3549:
	.loc	25 451 16
	cmpq	%rdi, %rdx
	jbe	.LBB21_637
.Ltmp3550:
	.loc	25 0 16 is_stmt 0
	movq	192(%rsp), %rsi
	leaq	(%rsi,%r14), %r8
	movq	152(%rsp), %rsi
.Ltmp3551:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%rsi,%r14,4), %xmm8
	movq	(%rsp), %rsi
.Ltmp3552:
	.loc	26 56 9
	vmovss	%xmm8, (%rsi,%rdi,4)
.Ltmp3553:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_677
.Ltmp3554:
	.loc	25 0 16 is_stmt 0
	movq	56(%rsp), %rsi
.Ltmp3555:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB21_715
.Ltmp3556:
	.loc	25 0 16 is_stmt 0
	movq	232(%rsp), %r8
.Ltmp3557:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r8,%r14,4), %xmm8
	movq	8(%rsp), %r8
.Ltmp3558:
	.loc	26 56 9
	vmovss	%xmm8, (%r8,%rdi,4)
.Ltmp3559:
	.loc	21 255 21
	leal	(%r10,%r14), %r12d
	decl	%r12d
	andl	%ecx, %r12d
.Ltmp3560:
	.loc	25 438 16
	cmpq	%r12, %r13
	jbe	.LBB21_568
.Ltmp3561:
	.loc	25 0 16 is_stmt 0
	leal	(%r9,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	cmpq	%rdi, %rdx
	jbe	.LBB21_704
	cmpq	%rdi, %rsi
.Ltmp3562:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB21_703
	.loc	21 0 29 is_stmt 0
	movq	296(%rsp), %r8
	addl	%r14d, %r8d
	decl	%r8d
	andl	%ecx, %r8d
	cmpq	%r8, %rsi
	jbe	.LBB21_702
	cmpq	%r8, %rdx
	movl	$8388608, %esi
	.loc	21 277 29 is_stmt 1
	jbe	.LBB21_701
.Ltmp3563:
	.loc	21 0 29 is_stmt 0
	movq	8(%rsp), %rdx
	vmovss	(%rdx,%rdi,4), %xmm8
	movq	(%rsp), %rdx
.Ltmp3564:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rdx,%rdi,4), %xmm9
	vbroadcastss	.LCPI21_2(%rip), %xmm0
	vandps	%xmm0, %xmm9, %xmm9
.Ltmp3565:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm0, %xmm8, %xmm10
.Ltmp3566:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm9
.Ltmp3567:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp3568:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm10, %r13d
.Ltmp3569:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r13d
	vxorps	%xmm8, %xmm8, %xmm8
	vmovss	128(%rsp), %xmm0
	vucomiss	%xmm8, %xmm0
.Ltmp3570:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r13d
	vmovss	124(%rsp), %xmm0
	vucomiss	%xmm8, %xmm0
.Ltmp3571:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm9, %xmm9
.Ltmp3572:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm10, %xmm10
.Ltmp3573:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp3574:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp3575:
	.loc	26 161 24
	cmovbel	%r13d, %edi
.Ltmp3576:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3577:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm9
.Ltmp3578:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3579:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3580:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm9
.Ltmp3581:
	.loc	26 161 24
	cmovbel	%esi, %edi
.Ltmp3582:
	.loc	26 185 42
	movl	%edi, %r13d
	andl	$8388607, %r13d
	orl	$1065353216, %r13d
.Ltmp3583:
	.loc	7 1291 18
	vmovd	%r13d, %xmm9
	vmovss	.LCPI21_1(%rip), %xmm0
.Ltmp3584:
	.loc	26 66 9
	vaddss	%xmm0, %xmm9, %xmm9
.Ltmp3585:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm10
.Ltmp3586:
	.loc	26 61 9
	vsubss	%xmm10, %xmm14, %xmm10
.Ltmp3587:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3588:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm10, %xmm10
.Ltmp3589:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3590:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm10, %xmm10
.Ltmp3591:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3592:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm10, %xmm10
.Ltmp3593:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3594:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm10, %xmm10
.Ltmp3595:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3596:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp3597:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp3598:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm10, %xmm10
.Ltmp3599:
	.loc	26 61 9
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp3600:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm9, %xmm9
.Ltmp3601:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm9, %xmm9
.Ltmp3602:
	.loc	26 161 24 is_stmt 0
	vmaxss	%xmm13, %xmm9, %xmm9
.Ltmp3603:
	.loc	26 129 14 is_stmt 1
	vucomiss	120(%rsp), %xmm9
.Ltmp3604:
	.loc	26 28 5
	movl	$0, %edi
	adcl	$-1, %edi
.Ltmp3605:
	.loc	26 129 14
	vucomiss	%xmm11, %xmm9
.Ltmp3606:
	.loc	26 144 9
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp3607:
	.loc	26 149 9
	movl	%edi, %r11d
.Ltmp3608:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm4
.Ltmp3609:
	.loc	26 149 9
	notl	%r11d
.Ltmp3610:
	.loc	26 139 9
	cmovbel	%r15d, %r11d
.Ltmp3611:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm10
.Ltmp3612:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm10
.Ltmp3613:
	.loc	26 144 9
	cmoval	%edi, %r13d
.Ltmp3614:
	.loc	26 139 9
	cmovbel	%r15d, %r11d
.Ltmp3615:
	.loc	26 161 24
	testb	$1, %r11b
	jne	.LBB21_545
.Ltmp3616:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp3617:
	.loc	26 161 24
	testb	$1, %r13b
	je	.LBB21_624
	jmp	.LBB21_625
.Ltmp3618:
	.loc	26 0 24
.Ltmp3619:
	.p2align	4
.LBB21_545:
	vaddss	%xmm0, %xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp3620:
	.loc	26 161 24
	testb	$1, %r13b
	jne	.LBB21_625
.Ltmp3621:
.LBB21_624:
	.loc	26 0 24
	vmovaps	%xmm10, %xmm4
.LBB21_625:
	orl	%r13d, %r11d
	andl	$1065353216, %r11d
	vmovd	%r11d, %xmm10
.Ltmp3622:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm11, %xmm9, %xmm9
.Ltmp3623:
	.loc	26 71 9
	vmulss	112(%rsp), %xmm9, %xmm9
.Ltmp3624:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm9, %xmm9
.Ltmp3625:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm10, %xmm8, %xmm10
	vcmpltss	%xmm8, %xmm9, %xmm11
	vandps	%xmm11, %xmm10, %xmm10
	vmovd	%xmm10, %edi
	testb	$1, %dil
	jne	.LBB21_627
.Ltmp3626:
	.loc	26 0 44
	vxorps	%xmm9, %xmm9, %xmm9
.LBB21_627:
.Ltmp3627:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm9
	movl	84(%rsp), %r13d
.Ltmp3628:
	.loc	26 161 24
	cmovbel	80(%rsp), %r13d
	movq	8(%rsp), %rdx
.Ltmp3629:
	.loc	26 103 24
	vmovss	(%rdx,%r8,4), %xmm10
	vbroadcastss	.LCPI21_2(%rip), %xmm1
	vandps	%xmm1, %xmm10, %xmm10
	movq	(%rsp), %rdx
.Ltmp3630:
	.loc	26 103 24 is_stmt 0
	vmovss	(%rdx,%r8,4), %xmm11
	vandps	%xmm1, %xmm11, %xmm11
.Ltmp3631:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm11, %xmm10
.Ltmp3632:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp3633:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm11, %ebp
.Ltmp3634:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
	vmovss	104(%rsp), %xmm0
	vucomiss	%xmm8, %xmm0
.Ltmp3635:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %ebp
	vmovss	96(%rsp), %xmm0
	vucomiss	%xmm8, %xmm0
.Ltmp3636:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm3, %xmm10, %xmm10
.Ltmp3637:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm11, %xmm11
.Ltmp3638:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm11, %xmm10
.Ltmp3639:
	.loc	7 1244 18
	vmovd	%xmm10, %r8d
.Ltmp3640:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp3641:
	.loc	7 1291 18
	vmovd	%r8d, %xmm10
.Ltmp3642:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm10
.Ltmp3643:
	.loc	7 1291 18
	vmovd	%r13d, %xmm10
	movl	$841731191, %ebp
.Ltmp3644:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp3645:
	.loc	26 66 9
	vsubss	%xmm7, %xmm9, %xmm9
.Ltmp3646:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp3647:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm11
.Ltmp3648:
	.loc	26 92 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp3649:
	.loc	26 161 24
	cmovbel	%esi, %r8d
.Ltmp3650:
	.loc	26 185 42
	movl	%r8d, %edi
	andl	$8388607, %edi
	orl	$1065353216, %edi
.Ltmp3651:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
	vmovss	.LCPI21_1(%rip), %xmm0
.Ltmp3652:
	.loc	26 66 9
	vaddss	%xmm0, %xmm10, %xmm10
.Ltmp3653:
	.loc	26 71 9
	vmulss	%xmm6, %xmm10, %xmm11
.Ltmp3654:
	.loc	26 61 9
	vsubss	%xmm11, %xmm14, %xmm11
.Ltmp3655:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3656:
	.loc	26 61 9
	vaddss	.LCPI21_8(%rip), %xmm11, %xmm11
.Ltmp3657:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3658:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm11, %xmm11
.Ltmp3659:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3660:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm11, %xmm11
.Ltmp3661:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3662:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm11, %xmm11
.Ltmp3663:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp3664:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm10
.Ltmp3665:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp3666:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm11, %xmm11
.Ltmp3667:
	.loc	26 61 9
	vaddss	%xmm10, %xmm11, %xmm11
	movq	24(%rsp), %rdx
	vmovss	(%rdx,%r12,4), %xmm10
.Ltmp3668:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm11, %xmm11
.Ltmp3669:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm11, %xmm11
.Ltmp3670:
	.loc	26 161 24 is_stmt 0
	vmaxss	%xmm13, %xmm11, %xmm14
.Ltmp3671:
	.loc	26 129 14 is_stmt 1
	vucomiss	100(%rsp), %xmm14
.Ltmp3672:
	.loc	26 28 5
	movl	$0, %edi
	adcl	$-1, %edi
.Ltmp3673:
	.loc	26 129 14
	vucomiss	32(%rsp), %xmm14
.Ltmp3674:
	.loc	26 92 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp3675:
	.loc	26 144 9
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp3676:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%rbx), %xmm9
.Ltmp3677:
	.loc	26 149 9 is_stmt 1
	movl	%edi, %r8d
	vmovss	48(%rsp), %xmm11
.Ltmp3678:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm11
.Ltmp3679:
	.loc	26 149 9
	notl	%r8d
.Ltmp3680:
	.loc	26 139 9
	cmovbel	%r15d, %r8d
.Ltmp3681:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm9
.Ltmp3682:
	.loc	26 103 24
	vandps	%xmm1, %xmm7, %xmm9
.Ltmp3683:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm9, %xmm9
	vandps	%xmm7, %xmm9, %xmm7
	movq	40(%rsp), %rdx
	vmovss	(%rdx,%r12,4), %xmm9
.Ltmp3684:
	.loc	21 373 5
	vmovss	%xmm4, 860(%rbx)
	.loc	21 382 5
	movl	%r11d, 856(%rbx)
.Ltmp3685:
	.loc	21 394 5
	vmovss	%xmm7, 864(%rbx)
.Ltmp3686:
	.loc	26 144 9
	cmoval	%edi, %r13d
.Ltmp3687:
	.loc	26 139 9
	cmovbel	%r15d, %r8d
.Ltmp3688:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_629
.Ltmp3689:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm0, %xmm11, %xmm11
.LBB21_629:
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm0
.Ltmp3690:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r13b
	jne	.LBB21_631
.Ltmp3691:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm0
.LBB21_631:
.Ltmp3692:
	vmulss	.LCPI21_18(%rip), %xmm7, %xmm11
	vmovss	.LCPI21_19(%rip), %xmm1
	vmovaps	%xmm1, %xmm13
	vmaxss	%xmm1, %xmm11, %xmm11
	vminss	.LCPI21_20(%rip), %xmm11, %xmm12
	vroundss	$9, %xmm12, %xmm12, %xmm11
	vsubss	%xmm11, %xmm12, %xmm12
.Ltmp3693:
	orl	%r13d, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm15
	vmovss	%xmm0, 48(%rsp)
.Ltmp3694:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm0, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp3695:
	.loc	26 66 9
	vsubss	32(%rsp), %xmm14, %xmm14
.Ltmp3696:
	.loc	26 71 9
	vmulss	88(%rsp), %xmm14, %xmm14
.Ltmp3697:
	.loc	26 161 24
	vmaxss	304(%rsp), %xmm14, %xmm14
.Ltmp3698:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm15, %xmm8, %xmm15
	vcmpltss	%xmm8, %xmm14, %xmm0
	vandps	%xmm0, %xmm15, %xmm0
	vmovd	%xmm0, %edi
	testb	$1, %dil
	jne	.LBB21_633
.Ltmp3699:
	.loc	26 0 44
	vxorps	%xmm14, %xmm14, %xmm14
	jmp	.LBB21_633
.LBB21_546:
	movq	288(%rsp), %rax
	cmpq	%rax, 216(%rsp)
	jbe	.LBB21_558
.Ltmp3700:
	.loc	25 451 16 is_stmt 1
	negq	%rax
	movq	%rax, 192(%rsp)
	movq	144(%rsp), %r8
	movl	%r8d, %edx
	subl	%r10d, %edx
	movl	%r8d, %eax
	subl	%r14d, %eax
	movl	%r8d, %r9d
	subl	%edi, %r9d
	movq	%rsi, %r10
	subq	%r11, %r10
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_4(%rip), %xmm12
	movl	$841731191, %ebp
	vmovss	.LCPI21_5(%rip), %xmm5
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_7(%rip), %xmm14
	vmovss	.LCPI21_8(%rip), %xmm13
	vmovss	.LCPI21_12(%rip), %xmm2
	xorl	%r15d, %r15d
	jmp	.LBB21_548
.Ltmp3701:
	.loc	25 0 16 is_stmt 0
.Ltmp3702:
	.p2align	4
.LBB21_621:
	vmovaps	176(%rsp), %xmm2
.Ltmp3703:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm14
	movl	76(%rsp), %edi
.Ltmp3704:
	.loc	26 161 24
	cmovbel	72(%rsp), %edi
.Ltmp3705:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp3706:
	.loc	26 66 9
	vsubss	%xmm2, %xmm14, %xmm14
.Ltmp3707:
	.loc	26 92 9
	vmulss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm3, %xmm2, %xmm2
.Ltmp3708:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp3709:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm14
	vmovss	.LCPI21_21(%rip), %xmm2
.Ltmp3710:
	.loc	26 71 9
	vmulss	%xmm2, %xmm12, %xmm3
	vmovss	.LCPI21_22(%rip), %xmm15
.Ltmp3711:
	.loc	26 61 9
	vaddss	%xmm3, %xmm15, %xmm3
.Ltmp3712:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_23(%rip), %xmm6
.Ltmp3713:
	.loc	26 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp3714:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_24(%rip), %xmm0
.Ltmp3715:
	.loc	26 61 9
	vaddss	%xmm0, %xmm3, %xmm3
.Ltmp3716:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_25(%rip), %xmm1
.Ltmp3717:
	.loc	26 61 9
	vaddss	%xmm1, %xmm3, %xmm3
.Ltmp3718:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
.Ltmp3719:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI21_18(%rip), %xmm14, %xmm12
.Ltmp3720:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI21_19(%rip), %xmm12, %xmm12
.Ltmp3721:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm12, %xmm12
	vmovss	.LCPI21_0(%rip), %xmm5
.Ltmp3722:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm5, %xmm3, %xmm3
	vmovss	.LCPI21_26(%rip), %xmm13
.Ltmp3723:
	.loc	26 178 22
	vaddss	%xmm13, %xmm11, %xmm11
.Ltmp3724:
	.loc	7 1244 18
	vmovd	%xmm11, %edi
.Ltmp3725:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3726:
	.loc	7 1291 18
	vmovd	%edi, %xmm11
.Ltmp3727:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp3728:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm10, %xmm3
.Ltmp3729:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm8, %xmm11
	vblendvps	%xmm11, %xmm3, %xmm10, %xmm3
	vcmpnltss	108(%rsp), %xmm8, %xmm11
	vblendvps	%xmm11, %xmm3, %xmm10, %xmm3
.Ltmp3730:
	.loc	7 1783 9
	vroundss	$9, %xmm12, %xmm12, %xmm10
.Ltmp3731:
	.loc	26 66 9
	vsubss	%xmm10, %xmm12, %xmm11
.Ltmp3732:
	.loc	26 71 9
	vmulss	%xmm2, %xmm11, %xmm12
.Ltmp3733:
	.loc	26 61 9
	vaddss	%xmm15, %xmm12, %xmm12
.Ltmp3734:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3735:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm12
.Ltmp3736:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3737:
	.loc	26 61 9
	vaddss	%xmm0, %xmm12, %xmm12
.Ltmp3738:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3739:
	.loc	26 61 9
	vaddss	%xmm1, %xmm12, %xmm12
.Ltmp3740:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm11
.Ltmp3741:
	.loc	26 61 9
	vaddss	%xmm5, %xmm11, %xmm11
.Ltmp3742:
	.loc	26 178 22
	vaddss	%xmm13, %xmm10, %xmm10
.Ltmp3743:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp3744:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3745:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp3746:
	.loc	26 71 9
	vmulss	%xmm10, %xmm11, %xmm10
.Ltmp3747:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3748:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm8, %xmm14, %xmm11
	vblendvps	%xmm11, %xmm10, %xmm9, %xmm10
	vcmpnltss	68(%rsp), %xmm8, %xmm8
	vblendvps	%xmm8, %xmm10, %xmm9, %xmm8
.Ltmp3749:
	.loc	26 56 9
	vmovss	%xmm3, -4(%r12,%r14,4)
	movq	16(%rsp), %rsi
.Ltmp3750:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm8, -4(%rsi,%r14,4)
	vmovaps	%xmm14, 176(%rsp)
.Ltmp3751:
	.loc	21 394 5 is_stmt 1
	vmovss	%xmm14, 940(%rbx)
.Ltmp3752:
	.loc	8 1916 50
	leaq	(%r10,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	movq	144(%rsp), %r8
	vmovss	132(%rsp), %xmm11
	vmovss	.LCPI21_4(%rip), %xmm12
	vmovss	.LCPI21_5(%rip), %xmm5
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_7(%rip), %xmm14
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_12(%rip), %xmm2
	vmovss	.LCPI21_8(%rip), %xmm13
.Ltmp3753:
	.loc	11 900 12
	je	.LBB21_587
.Ltmp3754:
.LBB21_548:
	.loc	21 246 22
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	movq	256(%rsp), %r13
.Ltmp3755:
	.loc	25 451 16
	cmpq	%rdi, %r13
	jbe	.LBB21_565
.Ltmp3756:
	.loc	25 0 16 is_stmt 0
	movq	192(%rsp), %rsi
	leaq	(%rsi,%r14), %r8
.Ltmp3757:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r12,%r14,4), %xmm8
	movq	24(%rsp), %rsi
.Ltmp3758:
	.loc	26 56 9
	vmovss	%xmm8, (%rsi,%rdi,4)
	movq	16(%rsp), %rsi
.Ltmp3759:
	.loc	26 51 9
	vmovss	-4(%rsi,%r14,4), %xmm8
	movq	40(%rsp), %rsi
.Ltmp3760:
	.loc	26 56 9
	vmovss	%xmm8, (%rsi,%rdi,4)
	movq	152(%rsp), %rsi
.Ltmp3761:
	.loc	26 51 9
	vmovss	-4(%rsi,%r14,4), %xmm8
	movq	(%rsp), %rsi
.Ltmp3762:
	.loc	26 56 9
	vmovss	%xmm8, (%rsi,%rdi,4)
.Ltmp3763:
	.loc	25 438 16
	cmpq	$1, %r8
	je	.LBB21_677
.Ltmp3764:
	.loc	25 0 16 is_stmt 0
	movq	56(%rsp), %rsi
.Ltmp3765:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %rsi
	jbe	.LBB21_715
.Ltmp3766:
	.loc	25 0 16 is_stmt 0
	movq	232(%rsp), %r8
.Ltmp3767:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r8,%r14,4), %xmm8
	movq	8(%rsp), %r8
.Ltmp3768:
	.loc	26 56 9
	vmovss	%xmm8, (%r8,%rdi,4)
.Ltmp3769:
	.loc	21 255 21
	leal	(%r9,%r14), %r12d
	decl	%r12d
	andl	%ecx, %r12d
.Ltmp3770:
	.loc	25 438 16
	cmpq	%r12, %r13
	jbe	.LBB21_568
.Ltmp3771:
	.loc	25 0 16 is_stmt 0
	leal	(%rax,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	movq	160(%rsp), %r11
	cmpq	%rdi, %r11
	jbe	.LBB21_704
	cmpq	%rdi, %rsi
	jbe	.LBB21_703
	leal	(%rdx,%r14), %r8d
	decl	%r8d
	andl	%ecx, %r8d
	cmpq	%r8, %rsi
	jbe	.LBB21_702
	cmpq	%r8, %r11
	jbe	.LBB21_701
	movq	8(%rsp), %rsi
	vmovss	(%rsi,%rdi,4), %xmm8
	movq	(%rsp), %rsi
.Ltmp3772:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rsi,%rdi,4), %xmm9
	vbroadcastss	.LCPI21_2(%rip), %xmm0
	vandps	%xmm0, %xmm9, %xmm9
.Ltmp3773:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm0, %xmm8, %xmm10
.Ltmp3774:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm9
.Ltmp3775:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp3776:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm10, %r13d
.Ltmp3777:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r13d
	vxorps	%xmm8, %xmm8, %xmm8
	vmovss	128(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp3778:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r13d
	vmovss	124(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp3779:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm9, %xmm9
.Ltmp3780:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm10, %xmm10
.Ltmp3781:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp3782:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp3783:
	.loc	26 161 24
	cmovbel	%r13d, %edi
.Ltmp3784:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3785:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm9
.Ltmp3786:
	.loc	26 161 24
	cmovbel	%ebp, %edi
.Ltmp3787:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3788:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm9
.Ltmp3789:
	.loc	26 161 24
	movl	$8388608, %esi
	cmovbel	%esi, %edi
.Ltmp3790:
	.loc	26 185 42
	movl	%edi, %r13d
	andl	$8388607, %r13d
	orl	$1065353216, %r13d
.Ltmp3791:
	.loc	7 1291 18
	vmovd	%r13d, %xmm9
	vmovss	.LCPI21_1(%rip), %xmm3
.Ltmp3792:
	.loc	26 66 9
	vaddss	%xmm3, %xmm9, %xmm9
.Ltmp3793:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm10
.Ltmp3794:
	.loc	26 61 9
	vsubss	%xmm10, %xmm14, %xmm10
.Ltmp3795:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3796:
	.loc	26 61 9
	vaddss	%xmm13, %xmm10, %xmm10
.Ltmp3797:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3798:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm10, %xmm10
.Ltmp3799:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3800:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm10, %xmm10
.Ltmp3801:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3802:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm10, %xmm10
.Ltmp3803:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp3804:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp3805:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp3806:
	.loc	26 187 13
	vaddss	%xmm2, %xmm10, %xmm10
.Ltmp3807:
	.loc	26 61 9
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp3808:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm9, %xmm9
.Ltmp3809:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm9, %xmm9
.Ltmp3810:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm9, %xmm9
.Ltmp3811:
	.loc	26 129 14 is_stmt 1
	vucomiss	120(%rsp), %xmm9
.Ltmp3812:
	.loc	26 28 5
	movl	$0, %edi
	adcl	$-1, %edi
.Ltmp3813:
	.loc	26 129 14
	vucomiss	%xmm11, %xmm9
.Ltmp3814:
	.loc	26 144 9
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp3815:
	.loc	26 149 9
	movl	%edi, %r11d
.Ltmp3816:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm4
.Ltmp3817:
	.loc	26 149 9
	notl	%r11d
.Ltmp3818:
	.loc	26 139 9
	cmovbel	%r15d, %r11d
.Ltmp3819:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm10
.Ltmp3820:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm10
.Ltmp3821:
	.loc	26 144 9
	cmoval	%edi, %r13d
.Ltmp3822:
	.loc	26 139 9
	cmovbel	%r15d, %r11d
.Ltmp3823:
	.loc	26 161 24
	testb	$1, %r11b
	jne	.LBB21_557
.Ltmp3824:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp3825:
	.loc	26 161 24
	testb	$1, %r13b
	je	.LBB21_612
	jmp	.LBB21_613
.Ltmp3826:
	.loc	26 0 24
.Ltmp3827:
	.p2align	4
.LBB21_557:
	vaddss	%xmm3, %xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp3828:
	.loc	26 161 24
	testb	$1, %r13b
	jne	.LBB21_613
.Ltmp3829:
.LBB21_612:
	.loc	26 0 24
	vmovaps	%xmm10, %xmm4
.LBB21_613:
	orl	%r13d, %r11d
	andl	$1065353216, %r11d
	vmovd	%r11d, %xmm10
.Ltmp3830:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm11, %xmm9, %xmm9
.Ltmp3831:
	.loc	26 71 9
	vmulss	112(%rsp), %xmm9, %xmm9
.Ltmp3832:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm9, %xmm9
.Ltmp3833:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm10, %xmm8, %xmm10
	vcmpltss	%xmm8, %xmm9, %xmm11
	vandps	%xmm11, %xmm10, %xmm10
	vmovd	%xmm10, %edi
	testb	$1, %dil
	jne	.LBB21_615
.Ltmp3834:
	.loc	26 0 44
	vxorps	%xmm9, %xmm9, %xmm9
.LBB21_615:
.Ltmp3835:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm9
	movl	84(%rsp), %r13d
.Ltmp3836:
	.loc	26 161 24
	cmovbel	80(%rsp), %r13d
	movq	8(%rsp), %rsi
.Ltmp3837:
	.loc	26 103 24
	vmovss	(%rsi,%r8,4), %xmm10
	vbroadcastss	.LCPI21_2(%rip), %xmm0
	vandps	%xmm0, %xmm10, %xmm10
	movq	(%rsp), %rsi
.Ltmp3838:
	.loc	26 103 24 is_stmt 0
	vmovss	(%rsi,%r8,4), %xmm11
	vandps	%xmm0, %xmm11, %xmm11
.Ltmp3839:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm11, %xmm10
.Ltmp3840:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp3841:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm11, %ebp
.Ltmp3842:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %ebp
	vmovss	104(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp3843:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %ebp
	vmovss	96(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp3844:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm10, %xmm10
.Ltmp3845:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm11, %xmm11
.Ltmp3846:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm11, %xmm10
.Ltmp3847:
	.loc	7 1244 18
	vmovd	%xmm10, %r8d
.Ltmp3848:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp3849:
	.loc	7 1291 18
	vmovd	%r8d, %xmm10
.Ltmp3850:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm10
.Ltmp3851:
	.loc	7 1291 18
	vmovd	%r13d, %xmm10
	movl	$841731191, %ebp
.Ltmp3852:
	.loc	26 161 24
	cmovbel	%ebp, %r8d
.Ltmp3853:
	.loc	26 66 9
	vsubss	%xmm7, %xmm9, %xmm9
.Ltmp3854:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp3855:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm11
.Ltmp3856:
	.loc	26 92 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp3857:
	.loc	26 161 24
	movl	$8388608, %esi
	cmovbel	%esi, %r8d
.Ltmp3858:
	.loc	26 185 42
	movl	%r8d, %edi
	andl	$8388607, %edi
	orl	$1065353216, %edi
.Ltmp3859:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
	vmovss	.LCPI21_1(%rip), %xmm3
.Ltmp3860:
	.loc	26 66 9
	vaddss	%xmm3, %xmm10, %xmm10
.Ltmp3861:
	.loc	26 71 9
	vmulss	%xmm6, %xmm10, %xmm11
.Ltmp3862:
	.loc	26 61 9
	vsubss	%xmm11, %xmm14, %xmm11
.Ltmp3863:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3864:
	.loc	26 61 9
	vaddss	%xmm13, %xmm11, %xmm11
.Ltmp3865:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3866:
	.loc	26 61 9
	vaddss	.LCPI21_9(%rip), %xmm11, %xmm11
.Ltmp3867:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3868:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm11, %xmm11
.Ltmp3869:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp3870:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm11, %xmm11
.Ltmp3871:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp3872:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm10
.Ltmp3873:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp3874:
	.loc	26 187 13
	vaddss	%xmm2, %xmm11, %xmm11
.Ltmp3875:
	.loc	26 61 9
	vaddss	%xmm10, %xmm11, %xmm11
	movq	24(%rsp), %rsi
	vmovss	(%rsi,%r12,4), %xmm10
.Ltmp3876:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm11, %xmm11
.Ltmp3877:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm11, %xmm11
.Ltmp3878:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm11, %xmm14
.Ltmp3879:
	.loc	26 129 14 is_stmt 1
	vucomiss	100(%rsp), %xmm14
.Ltmp3880:
	.loc	26 28 5
	movl	$0, %edi
	adcl	$-1, %edi
.Ltmp3881:
	.loc	26 129 14
	vucomiss	32(%rsp), %xmm14
.Ltmp3882:
	.loc	26 92 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp3883:
	.loc	26 144 9
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp3884:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%rbx), %xmm9
.Ltmp3885:
	.loc	26 149 9 is_stmt 1
	movl	%edi, %r8d
	vmovss	48(%rsp), %xmm11
.Ltmp3886:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm11
.Ltmp3887:
	.loc	26 149 9
	notl	%r8d
.Ltmp3888:
	.loc	26 139 9
	cmovbel	%r15d, %r8d
.Ltmp3889:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm9
.Ltmp3890:
	.loc	26 103 24
	vandps	%xmm0, %xmm7, %xmm9
.Ltmp3891:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm9, %xmm9
	vandps	%xmm7, %xmm9, %xmm7
	movq	40(%rsp), %rsi
	vmovss	(%rsi,%r12,4), %xmm9
.Ltmp3892:
	.loc	21 373 5
	vmovss	%xmm4, 860(%rbx)
	.loc	21 382 5
	movl	%r11d, 856(%rbx)
.Ltmp3893:
	.loc	21 394 5
	vmovss	%xmm7, 864(%rbx)
.Ltmp3894:
	.loc	26 144 9
	cmoval	%edi, %r13d
.Ltmp3895:
	.loc	26 139 9
	cmovbel	%r15d, %r8d
.Ltmp3896:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_617
.Ltmp3897:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm3, %xmm11, %xmm11
.LBB21_617:
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm3
.Ltmp3898:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r13b
	jne	.LBB21_619
.Ltmp3899:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm3
.LBB21_619:
.Ltmp3900:
	vmulss	.LCPI21_18(%rip), %xmm7, %xmm11
	vmaxss	.LCPI21_19(%rip), %xmm11, %xmm11
	vminss	.LCPI21_20(%rip), %xmm11, %xmm12
	vroundss	$9, %xmm12, %xmm12, %xmm11
	vsubss	%xmm11, %xmm12, %xmm12
.Ltmp3901:
	orl	%r13d, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm15
	vmovss	%xmm3, 48(%rsp)
.Ltmp3902:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp3903:
	.loc	26 66 9
	vsubss	32(%rsp), %xmm14, %xmm14
.Ltmp3904:
	.loc	26 71 9
	vmulss	88(%rsp), %xmm14, %xmm14
.Ltmp3905:
	.loc	26 161 24
	vmaxss	304(%rsp), %xmm14, %xmm14
.Ltmp3906:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm15, %xmm8, %xmm15
	vcmpltss	%xmm8, %xmm14, %xmm3
	vandps	%xmm3, %xmm15, %xmm3
	vmovd	%xmm3, %edi
	testb	$1, %dil
	jne	.LBB21_621
.Ltmp3907:
	.loc	26 0 44
	vxorps	%xmm14, %xmm14, %xmm14
	jmp	.LBB21_621
.LBB21_558:
	cmpq	56(%rsp), %r9
	jbe	.LBB21_559
	movq	144(%rsp), %r8
.Ltmp3908:
	.loc	25 438 16 is_stmt 1
	movl	%r8d, %eax
	subl	%r10d, %eax
	movl	%r8d, %edx
	subl	%r14d, %edx
	movl	%r8d, %r10d
	subl	%edi, %r10d
	movq	%rsi, %r9
	subq	%r11, %r9
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_4(%rip), %xmm12
	vmovss	.LCPI21_5(%rip), %xmm5
	movl	$8388608, %r11d
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_7(%rip), %xmm13
	vmovss	.LCPI21_8(%rip), %xmm14
	vmovss	.LCPI21_9(%rip), %xmm2
	xorl	%ebp, %ebp
	jmp	.LBB21_589
.Ltmp3909:
	.loc	25 0 16 is_stmt 0
.Ltmp3910:
	.p2align	4
.LBB21_609:
	vmovaps	176(%rsp), %xmm2
.Ltmp3911:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm14
	movl	76(%rsp), %edi
.Ltmp3912:
	.loc	26 161 24
	cmovbel	72(%rsp), %edi
.Ltmp3913:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp3914:
	.loc	26 66 9
	vsubss	%xmm2, %xmm14, %xmm14
.Ltmp3915:
	.loc	26 92 9
	vmulss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm3, %xmm2, %xmm2
.Ltmp3916:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp3917:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm14
	vmovss	.LCPI21_21(%rip), %xmm2
.Ltmp3918:
	.loc	26 71 9
	vmulss	%xmm2, %xmm12, %xmm3
	vmovss	.LCPI21_22(%rip), %xmm15
.Ltmp3919:
	.loc	26 61 9
	vaddss	%xmm3, %xmm15, %xmm3
.Ltmp3920:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_23(%rip), %xmm6
.Ltmp3921:
	.loc	26 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp3922:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_24(%rip), %xmm13
.Ltmp3923:
	.loc	26 61 9
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp3924:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_25(%rip), %xmm1
.Ltmp3925:
	.loc	26 61 9
	vaddss	%xmm1, %xmm3, %xmm3
.Ltmp3926:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
.Ltmp3927:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI21_18(%rip), %xmm14, %xmm12
.Ltmp3928:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI21_19(%rip), %xmm12, %xmm12
.Ltmp3929:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm12, %xmm12
	vmovss	.LCPI21_0(%rip), %xmm5
.Ltmp3930:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm5, %xmm3, %xmm3
	vmovss	.LCPI21_26(%rip), %xmm0
.Ltmp3931:
	.loc	26 178 22
	vaddss	%xmm0, %xmm11, %xmm11
.Ltmp3932:
	.loc	7 1244 18
	vmovd	%xmm11, %edi
.Ltmp3933:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3934:
	.loc	7 1291 18
	vmovd	%edi, %xmm11
.Ltmp3935:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp3936:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm10, %xmm3
.Ltmp3937:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm8, %xmm11
	vblendvps	%xmm11, %xmm3, %xmm10, %xmm3
	vcmpnltss	108(%rsp), %xmm8, %xmm11
	vblendvps	%xmm11, %xmm3, %xmm10, %xmm3
.Ltmp3938:
	.loc	7 1783 9
	vroundss	$9, %xmm12, %xmm12, %xmm10
.Ltmp3939:
	.loc	26 66 9
	vsubss	%xmm10, %xmm12, %xmm11
.Ltmp3940:
	.loc	26 71 9
	vmulss	%xmm2, %xmm11, %xmm12
.Ltmp3941:
	.loc	26 61 9
	vaddss	%xmm15, %xmm12, %xmm12
.Ltmp3942:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3943:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm12
.Ltmp3944:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3945:
	.loc	26 61 9
	vaddss	%xmm13, %xmm12, %xmm12
.Ltmp3946:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp3947:
	.loc	26 61 9
	vaddss	%xmm1, %xmm12, %xmm12
.Ltmp3948:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm11
.Ltmp3949:
	.loc	26 61 9
	vaddss	%xmm5, %xmm11, %xmm11
.Ltmp3950:
	.loc	26 178 22
	vaddss	%xmm0, %xmm10, %xmm10
.Ltmp3951:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp3952:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp3953:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp3954:
	.loc	26 71 9
	vmulss	%xmm10, %xmm11, %xmm10
.Ltmp3955:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp3956:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm8, %xmm14, %xmm11
	vblendvps	%xmm11, %xmm10, %xmm9, %xmm10
	vcmpnltss	68(%rsp), %xmm8, %xmm8
	vblendvps	%xmm8, %xmm10, %xmm9, %xmm8
.Ltmp3957:
	.loc	26 56 9
	vmovss	%xmm3, -4(%r12,%r14,4)
	movq	16(%rsp), %rdi
.Ltmp3958:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm8, -4(%rdi,%r14,4)
	vmovaps	%xmm14, 176(%rsp)
.Ltmp3959:
	.loc	21 394 5 is_stmt 1
	vmovss	%xmm14, 940(%rbx)
.Ltmp3960:
	.loc	8 1916 50
	leaq	(%r9,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	movq	144(%rsp), %r8
	vmovss	132(%rsp), %xmm11
	vmovss	.LCPI21_4(%rip), %xmm12
	vmovss	.LCPI21_5(%rip), %xmm5
	vmovss	.LCPI21_7(%rip), %xmm13
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_8(%rip), %xmm14
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_9(%rip), %xmm2
.Ltmp3961:
	.loc	11 900 12
	je	.LBB21_587
.LBB21_589:
	.loc	11 0 12 is_stmt 0
	movq	224(%rsp), %rsi
.Ltmp3962:
	.loc	15 971 17 is_stmt 1
	leaq	(%rsi,%r14), %rdi
.Ltmp3963:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_561
.Ltmp3964:
	.loc	21 0 0 is_stmt 0
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	movq	256(%rsp), %r13
.Ltmp3965:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %r13
	movq	160(%rsp), %r8
	jbe	.LBB21_565
.Ltmp3966:
	.loc	26 51 9
	vmovss	-4(%r12,%r14,4), %xmm8
	movq	24(%rsp), %r15
.Ltmp3967:
	.loc	26 56 9
	vmovss	%xmm8, (%r15,%rdi,4)
	movq	16(%rsp), %r15
.Ltmp3968:
	.loc	26 51 9
	vmovss	-4(%r15,%r14,4), %xmm8
	movq	40(%rsp), %r15
.Ltmp3969:
	.loc	26 56 9
	vmovss	%xmm8, (%r15,%rdi,4)
	movq	152(%rsp), %r15
.Ltmp3970:
	.loc	26 51 9
	vmovss	-4(%r15,%r14,4), %xmm8
	movq	(%rsp), %r15
.Ltmp3971:
	.loc	26 56 9
	vmovss	%xmm8, (%r15,%rdi,4)
.Ltmp3972:
	.loc	25 451 16
	cmpq	%rdi, 56(%rsp)
	jbe	.LBB21_715
.Ltmp3973:
	.loc	25 0 16 is_stmt 0
	movq	232(%rsp), %r15
.Ltmp3974:
	.loc	26 51 9 is_stmt 1
	vmovss	-4(%r15,%r14,4), %xmm8
	movq	8(%rsp), %rsi
.Ltmp3975:
	.loc	26 56 9
	vmovss	%xmm8, (%rsi,%rdi,4)
.Ltmp3976:
	.loc	21 255 21
	leal	(%r10,%r14), %r12d
	decl	%r12d
	andl	%ecx, %r12d
.Ltmp3977:
	.loc	25 438 16
	cmpq	%r12, %r13
	jbe	.LBB21_568
.Ltmp3978:
	.loc	25 0 16 is_stmt 0
	leal	(%rdx,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	cmpq	%rdi, %r8
	jbe	.LBB21_704
	movq	56(%rsp), %r15
	cmpq	%rdi, %r15
.Ltmp3979:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB21_703
.Ltmp3980:
	.loc	21 0 29 is_stmt 0
	leal	(%rax,%r14), %r8d
	decl	%r8d
	andl	%ecx, %r8d
	cmpq	%r8, %r15
	jbe	.LBB21_702
	movq	(%rsp), %r15
.Ltmp3981:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r15,%rdi,4), %xmm8
	vbroadcastss	.LCPI21_2(%rip), %xmm0
	vandps	%xmm0, %xmm8, %xmm9
	movq	8(%rsp), %rsi
.Ltmp3982:
	.loc	26 103 24 is_stmt 0
	vmovss	(%rsi,%rdi,4), %xmm8
	vandps	%xmm0, %xmm8, %xmm10
.Ltmp3983:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm9
.Ltmp3984:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp3985:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm10, %r15d
.Ltmp3986:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r15d
	vxorps	%xmm8, %xmm8, %xmm8
	vmovss	128(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp3987:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r15d
	vmovss	124(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp3988:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm9, %xmm9
.Ltmp3989:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm10, %xmm10
.Ltmp3990:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp3991:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp3992:
	.loc	26 161 24
	cmovbel	%r15d, %edi
.Ltmp3993:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3994:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm9
.Ltmp3995:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %edi
.Ltmp3996:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp3997:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm9
.Ltmp3998:
	.loc	26 161 24
	cmovbel	%r11d, %edi
.Ltmp3999:
	.loc	26 185 42
	movl	%edi, %r15d
	andl	$8388607, %r15d
	orl	$1065353216, %r15d
.Ltmp4000:
	.loc	7 1291 18
	vmovd	%r15d, %xmm9
	vmovss	.LCPI21_1(%rip), %xmm3
.Ltmp4001:
	.loc	26 66 9
	vaddss	%xmm3, %xmm9, %xmm9
.Ltmp4002:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm10
.Ltmp4003:
	.loc	26 61 9
	vsubss	%xmm10, %xmm13, %xmm10
.Ltmp4004:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4005:
	.loc	26 61 9
	vaddss	%xmm14, %xmm10, %xmm10
.Ltmp4006:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4007:
	.loc	26 61 9
	vaddss	%xmm2, %xmm10, %xmm10
.Ltmp4008:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4009:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm10, %xmm10
.Ltmp4010:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4011:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm10, %xmm10
.Ltmp4012:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp4013:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp4014:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp4015:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm10, %xmm10
.Ltmp4016:
	.loc	26 61 9
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp4017:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm9, %xmm9
.Ltmp4018:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm9, %xmm9
.Ltmp4019:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm9, %xmm9
.Ltmp4020:
	.loc	26 129 14 is_stmt 1
	vucomiss	120(%rsp), %xmm9
.Ltmp4021:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp4022:
	.loc	26 129 14
	vucomiss	%xmm11, %xmm9
.Ltmp4023:
	.loc	26 144 9
	movl	$0, %r15d
	adcl	$-1, %r15d
.Ltmp4024:
	.loc	26 149 9
	movl	%r13d, %edi
.Ltmp4025:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm4
.Ltmp4026:
	.loc	26 149 9
	notl	%edi
.Ltmp4027:
	.loc	26 139 9
	cmovbel	%ebp, %edi
.Ltmp4028:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm10
.Ltmp4029:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm10
.Ltmp4030:
	.loc	26 144 9
	cmoval	%r13d, %r15d
.Ltmp4031:
	.loc	26 139 9
	cmovbel	%ebp, %edi
.Ltmp4032:
	.loc	26 161 24
	testb	$1, %dil
	jne	.LBB21_597
.Ltmp4033:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp4034:
	.loc	26 161 24
	testb	$1, %r15b
	je	.LBB21_600
	jmp	.LBB21_601
.Ltmp4035:
.LBB21_597:
	.loc	21 0 0
	vaddss	%xmm3, %xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp4036:
	.loc	26 161 24
	testb	$1, %r15b
	jne	.LBB21_601
.Ltmp4037:
.LBB21_600:
	.loc	26 0 24
	vmovaps	%xmm10, %xmm4
.LBB21_601:
	orl	%r15d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp4038:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm11, %xmm9, %xmm9
.Ltmp4039:
	.loc	26 71 9
	vmulss	112(%rsp), %xmm9, %xmm9
.Ltmp4040:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm9, %xmm9
.Ltmp4041:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm10, %xmm8, %xmm10
	vcmpltss	%xmm8, %xmm9, %xmm11
	vandps	%xmm11, %xmm10, %xmm10
	vmovd	%xmm10, %r15d
	testb	$1, %r15b
	jne	.LBB21_603
.Ltmp4042:
	.loc	26 0 44
	vxorps	%xmm9, %xmm9, %xmm9
.LBB21_603:
.Ltmp4043:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm9
	movl	84(%rsp), %r15d
.Ltmp4044:
	.loc	26 161 24
	cmovbel	80(%rsp), %r15d
	movq	8(%rsp), %rsi
.Ltmp4045:
	.loc	26 103 24
	vmovss	(%rsi,%r8,4), %xmm10
	vbroadcastss	.LCPI21_2(%rip), %xmm0
	vandps	%xmm0, %xmm10, %xmm10
	movq	(%rsp), %r13
.Ltmp4046:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r13,%r8,4), %xmm11
	vandps	%xmm0, %xmm11, %xmm11
.Ltmp4047:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm11, %xmm10
.Ltmp4048:
	.loc	7 1244 18
	vmovd	%xmm10, %r8d
.Ltmp4049:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm11, %r13d
.Ltmp4050:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %r13d
	vmovss	104(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp4051:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r8d, %r13d
	vmovss	96(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp4052:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm10, %xmm10
.Ltmp4053:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm11, %xmm11
.Ltmp4054:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm11, %xmm10
.Ltmp4055:
	.loc	7 1244 18
	vmovd	%xmm10, %r8d
.Ltmp4056:
	.loc	26 161 24
	cmovbel	%r13d, %r8d
.Ltmp4057:
	.loc	7 1291 18
	vmovd	%r8d, %xmm10
.Ltmp4058:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm10
.Ltmp4059:
	.loc	7 1291 18
	vmovd	%r15d, %xmm10
.Ltmp4060:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %r8d
.Ltmp4061:
	.loc	26 66 9
	vsubss	%xmm7, %xmm9, %xmm9
.Ltmp4062:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp4063:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm11
.Ltmp4064:
	.loc	26 92 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp4065:
	.loc	26 161 24
	cmovbel	%r11d, %r8d
.Ltmp4066:
	.loc	26 185 42
	movl	%r8d, %r15d
	andl	$8388607, %r15d
	orl	$1065353216, %r15d
.Ltmp4067:
	.loc	7 1291 18
	vmovd	%r15d, %xmm10
	vmovss	.LCPI21_1(%rip), %xmm3
.Ltmp4068:
	.loc	26 66 9
	vaddss	%xmm3, %xmm10, %xmm10
.Ltmp4069:
	.loc	26 71 9
	vmulss	%xmm6, %xmm10, %xmm11
.Ltmp4070:
	.loc	26 61 9
	vsubss	%xmm11, %xmm13, %xmm11
.Ltmp4071:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp4072:
	.loc	26 61 9
	vaddss	%xmm14, %xmm11, %xmm11
.Ltmp4073:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp4074:
	.loc	26 61 9
	vaddss	%xmm2, %xmm11, %xmm11
.Ltmp4075:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp4076:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm11, %xmm11
.Ltmp4077:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp4078:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm11, %xmm11
.Ltmp4079:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp4080:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm10
.Ltmp4081:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp4082:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm11, %xmm11
.Ltmp4083:
	.loc	26 61 9
	vaddss	%xmm10, %xmm11, %xmm11
	movq	24(%rsp), %r8
	vmovss	(%r8,%r12,4), %xmm10
.Ltmp4084:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm11, %xmm11
.Ltmp4085:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm11, %xmm11
.Ltmp4086:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm11, %xmm14
.Ltmp4087:
	.loc	26 129 14 is_stmt 1
	vucomiss	100(%rsp), %xmm14
.Ltmp4088:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp4089:
	.loc	26 129 14
	vucomiss	32(%rsp), %xmm14
.Ltmp4090:
	.loc	26 92 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp4091:
	.loc	26 144 9
	movl	$0, %r15d
	adcl	$-1, %r15d
.Ltmp4092:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%rbx), %xmm9
.Ltmp4093:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r8d
	vmovss	48(%rsp), %xmm11
.Ltmp4094:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm11
.Ltmp4095:
	.loc	26 149 9
	notl	%r8d
.Ltmp4096:
	.loc	26 139 9
	cmovbel	%ebp, %r8d
.Ltmp4097:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm9
.Ltmp4098:
	.loc	26 103 24
	vandps	%xmm0, %xmm7, %xmm9
.Ltmp4099:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm9, %xmm9
	vandps	%xmm7, %xmm9, %xmm7
	movq	40(%rsp), %rsi
	vmovss	(%rsi,%r12,4), %xmm9
.Ltmp4100:
	.loc	21 373 5
	vmovss	%xmm4, 860(%rbx)
	.loc	21 382 5
	movl	%edi, 856(%rbx)
.Ltmp4101:
	.loc	21 394 5
	vmovss	%xmm7, 864(%rbx)
.Ltmp4102:
	.loc	26 144 9
	cmoval	%r13d, %r15d
.Ltmp4103:
	.loc	26 139 9
	cmovbel	%ebp, %r8d
.Ltmp4104:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_605
.Ltmp4105:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm3, %xmm11, %xmm11
.LBB21_605:
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm3
.Ltmp4106:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r15b
	jne	.LBB21_607
.Ltmp4107:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm3
.LBB21_607:
.Ltmp4108:
	vmulss	.LCPI21_18(%rip), %xmm7, %xmm11
	vmaxss	.LCPI21_19(%rip), %xmm11, %xmm11
	vminss	.LCPI21_20(%rip), %xmm11, %xmm12
	vroundss	$9, %xmm12, %xmm12, %xmm11
	vsubss	%xmm11, %xmm12, %xmm12
.Ltmp4109:
	orl	%r15d, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm15
	vmovss	%xmm3, 48(%rsp)
.Ltmp4110:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp4111:
	.loc	26 66 9
	vsubss	32(%rsp), %xmm14, %xmm14
.Ltmp4112:
	.loc	26 71 9
	vmulss	88(%rsp), %xmm14, %xmm14
.Ltmp4113:
	.loc	26 161 24
	vmaxss	304(%rsp), %xmm14, %xmm14
.Ltmp4114:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm15, %xmm8, %xmm15
	vcmpltss	%xmm8, %xmm14, %xmm3
	vandps	%xmm3, %xmm15, %xmm3
	vmovd	%xmm3, %edi
	testb	$1, %dil
	jne	.LBB21_609
.Ltmp4115:
	.loc	26 0 44
	vxorps	%xmm14, %xmm14, %xmm14
	jmp	.LBB21_609
.LBB21_462:
	leaq	1(%r8), %rsi
.Ltmp4116:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_cad5a7b24766ffcdda9cbfe066eb4078(%rip), %rcx
	movq	%r8, %rdi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4117:
.LBB21_467:
	.loc	25 0 13 is_stmt 0
	leaq	1(%r12), %rsi
.Ltmp4118:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_ba99eeeb3482270ebe1c674b4013dd6a(%rip), %rcx
.Ltmp4119:
	.loc	6 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB21_466:
	leaq	1(%r8), %rsi
.Ltmp4120:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_ad8d9ef662eaae13c96be161fe1fd2e1(%rip), %rcx
	movq	%r8, %rdi
	movq	32(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4121:
.LBB21_465:
	.loc	25 0 13 is_stmt 0
	leaq	1(%r8), %rsi
.Ltmp4122:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_a8b70dfa50e86ad5a1ce9afbe0da1688(%rip), %rcx
	movq	%r8, %rdi
	movq	(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4123:
.LBB21_559:
	.loc	25 0 13 is_stmt 0
	movq	144(%rsp), %r8
.Ltmp4124:
	.loc	25 438 16 is_stmt 1
	movl	%r8d, %eax
	subl	%r10d, %eax
	movl	%r8d, %edx
	subl	%r14d, %edx
	movl	%r8d, %r10d
	subl	%edi, %r10d
	movq	%rsi, %r9
	subq	%r11, %r9
	movl	$1, %r14d
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_4(%rip), %xmm12
	vmovss	.LCPI21_5(%rip), %xmm5
	movl	$8388608, %r11d
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_7(%rip), %xmm13
	vmovss	.LCPI21_8(%rip), %xmm14
	vmovss	.LCPI21_9(%rip), %xmm2
	xorl	%ebp, %ebp
	jmp	.LBB21_560
.Ltmp4125:
	.loc	25 0 16 is_stmt 0
.Ltmp4126:
	.p2align	4
.LBB21_586:
	vmovaps	176(%rsp), %xmm2
.Ltmp4127:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm2, %xmm14
	movl	76(%rsp), %edi
.Ltmp4128:
	.loc	26 161 24
	cmovbel	72(%rsp), %edi
.Ltmp4129:
	.loc	7 1291 18
	vmovd	%edi, %xmm3
.Ltmp4130:
	.loc	26 66 9
	vsubss	%xmm2, %xmm14, %xmm14
.Ltmp4131:
	.loc	26 92 9
	vmulss	%xmm3, %xmm14, %xmm3
	vaddss	%xmm3, %xmm2, %xmm2
.Ltmp4132:
	.loc	26 103 24
	vbroadcastss	.LCPI21_2(%rip), %xmm3
	vandps	%xmm3, %xmm2, %xmm3
.Ltmp4133:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm3, %xmm3
	vandps	%xmm2, %xmm3, %xmm14
	vmovss	.LCPI21_21(%rip), %xmm2
.Ltmp4134:
	.loc	26 71 9
	vmulss	%xmm2, %xmm12, %xmm3
	vmovss	.LCPI21_22(%rip), %xmm15
.Ltmp4135:
	.loc	26 61 9
	vaddss	%xmm3, %xmm15, %xmm3
.Ltmp4136:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_23(%rip), %xmm6
.Ltmp4137:
	.loc	26 61 9
	vaddss	%xmm6, %xmm3, %xmm3
.Ltmp4138:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_24(%rip), %xmm13
.Ltmp4139:
	.loc	26 61 9
	vaddss	%xmm3, %xmm13, %xmm3
.Ltmp4140:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
	vmovss	.LCPI21_25(%rip), %xmm1
.Ltmp4141:
	.loc	26 61 9
	vaddss	%xmm1, %xmm3, %xmm3
.Ltmp4142:
	.loc	26 71 9
	vmulss	%xmm3, %xmm12, %xmm3
.Ltmp4143:
	.loc	26 71 9 is_stmt 0
	vmulss	.LCPI21_18(%rip), %xmm14, %xmm12
.Ltmp4144:
	.loc	26 161 24 is_stmt 1
	vmaxss	.LCPI21_19(%rip), %xmm12, %xmm12
.Ltmp4145:
	.loc	26 161 24 is_stmt 0
	vminss	.LCPI21_20(%rip), %xmm12, %xmm12
	vmovss	.LCPI21_0(%rip), %xmm0
.Ltmp4146:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm0, %xmm3, %xmm3
	vmovss	.LCPI21_26(%rip), %xmm5
.Ltmp4147:
	.loc	26 178 22
	vaddss	%xmm5, %xmm11, %xmm11
.Ltmp4148:
	.loc	7 1244 18
	vmovd	%xmm11, %edi
.Ltmp4149:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp4150:
	.loc	7 1291 18
	vmovd	%edi, %xmm11
.Ltmp4151:
	.loc	26 71 9
	vmulss	%xmm3, %xmm11, %xmm3
.Ltmp4152:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm3, %xmm10, %xmm3
.Ltmp4153:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm7, %xmm8, %xmm11
	vblendvps	%xmm11, %xmm3, %xmm10, %xmm3
	vcmpnltss	108(%rsp), %xmm8, %xmm11
	vblendvps	%xmm11, %xmm3, %xmm10, %xmm3
.Ltmp4154:
	.loc	7 1783 9
	vroundss	$9, %xmm12, %xmm12, %xmm10
.Ltmp4155:
	.loc	26 66 9
	vsubss	%xmm10, %xmm12, %xmm11
.Ltmp4156:
	.loc	26 71 9
	vmulss	%xmm2, %xmm11, %xmm12
.Ltmp4157:
	.loc	26 61 9
	vaddss	%xmm15, %xmm12, %xmm12
.Ltmp4158:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp4159:
	.loc	26 61 9
	vaddss	%xmm6, %xmm12, %xmm12
.Ltmp4160:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp4161:
	.loc	26 61 9
	vaddss	%xmm13, %xmm12, %xmm12
.Ltmp4162:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm12
.Ltmp4163:
	.loc	26 61 9
	vaddss	%xmm1, %xmm12, %xmm12
.Ltmp4164:
	.loc	26 71 9
	vmulss	%xmm12, %xmm11, %xmm11
.Ltmp4165:
	.loc	26 61 9
	vaddss	%xmm0, %xmm11, %xmm11
.Ltmp4166:
	.loc	26 178 22
	vaddss	%xmm5, %xmm10, %xmm10
.Ltmp4167:
	.loc	7 1244 18
	vmovd	%xmm10, %edi
.Ltmp4168:
	.loc	26 179 24
	shll	$23, %edi
.Ltmp4169:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp4170:
	.loc	26 71 9
	vmulss	%xmm10, %xmm11, %xmm10
.Ltmp4171:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4172:
	.loc	26 161 24 is_stmt 1
	vcmpneqss	%xmm8, %xmm14, %xmm11
	vblendvps	%xmm11, %xmm10, %xmm9, %xmm10
	vcmpnltss	68(%rsp), %xmm8, %xmm8
	vblendvps	%xmm8, %xmm10, %xmm9, %xmm8
.Ltmp4173:
	.loc	26 56 9
	vmovss	%xmm3, -4(%r12,%r14,4)
	movq	16(%rsp), %rdi
.Ltmp4174:
	.loc	26 56 9 is_stmt 0
	vmovss	%xmm8, -4(%rdi,%r14,4)
	vmovaps	%xmm14, 176(%rsp)
.Ltmp4175:
	.loc	21 394 5 is_stmt 1
	vmovss	%xmm14, 940(%rbx)
.Ltmp4176:
	.loc	8 1916 50
	leaq	(%r9,%r14), %rdi
	incq	%rdi
	incq	%r14
	cmpq	$1, %rdi
	movq	144(%rsp), %r8
	vmovss	132(%rsp), %xmm11
	vmovss	.LCPI21_4(%rip), %xmm12
	vmovss	.LCPI21_5(%rip), %xmm5
	vmovss	.LCPI21_7(%rip), %xmm13
	vmovss	.LCPI21_6(%rip), %xmm6
	vmovss	.LCPI21_8(%rip), %xmm14
	vmovss	.LCPI21_3(%rip), %xmm1
	vmovss	.LCPI21_9(%rip), %xmm2
.Ltmp4177:
	.loc	11 900 12
	je	.LBB21_587
.LBB21_560:
	.loc	11 0 12 is_stmt 0
	movq	224(%rsp), %rsi
.Ltmp4178:
	.loc	15 971 17 is_stmt 1
	leaq	(%rsi,%r14), %rdi
.Ltmp4179:
	.loc	25 438 16
	cmpq	$1, %rdi
	je	.LBB21_561
.Ltmp4180:
	.loc	21 0 0 is_stmt 0
	leal	(%r8,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	movq	256(%rsp), %r8
.Ltmp4181:
	.loc	25 451 16 is_stmt 1
	cmpq	%rdi, %r8
	movq	160(%rsp), %r13
	jbe	.LBB21_564
.Ltmp4182:
	.loc	26 51 9
	vmovss	-4(%r12,%r14,4), %xmm8
	movq	24(%rsp), %r15
.Ltmp4183:
	.loc	26 56 9
	vmovss	%xmm8, (%r15,%rdi,4)
	movq	16(%rsp), %r15
.Ltmp4184:
	.loc	26 51 9
	vmovss	-4(%r15,%r14,4), %xmm8
	movq	40(%rsp), %r15
.Ltmp4185:
	.loc	26 56 9
	vmovss	%xmm8, (%r15,%rdi,4)
	movq	152(%rsp), %r15
.Ltmp4186:
	.loc	26 51 9
	vmovss	-4(%r15,%r14,4), %xmm8
	movq	(%rsp), %r15
.Ltmp4187:
	.loc	26 56 9
	vmovss	%xmm8, (%r15,%rdi,4)
	movq	232(%rsp), %r15
.Ltmp4188:
	.loc	26 51 9
	vmovss	-4(%r15,%r14,4), %xmm8
	movq	8(%rsp), %rsi
.Ltmp4189:
	.loc	26 56 9
	vmovss	%xmm8, (%rsi,%rdi,4)
.Ltmp4190:
	.loc	21 255 21
	leal	(%r10,%r14), %r12d
	decl	%r12d
	andl	%ecx, %r12d
.Ltmp4191:
	.loc	25 438 16
	cmpq	%r12, %r8
	jbe	.LBB21_567
.Ltmp4192:
	.loc	25 0 16 is_stmt 0
	leal	(%rdx,%r14), %edi
	decl	%edi
	andl	%ecx, %edi
	cmpq	%rdi, %r13
	jbe	.LBB21_704
	movq	56(%rsp), %r15
	cmpq	%rdi, %r15
.Ltmp4193:
	.loc	21 275 29 is_stmt 1
	jbe	.LBB21_703
	.loc	21 0 29 is_stmt 0
	leal	(%rax,%r14), %r8d
	decl	%r8d
	andl	%ecx, %r8d
	cmpq	%r8, %r15
	jbe	.LBB21_702
	cmpq	%r8, %r13
	.loc	21 277 29 is_stmt 1
	jbe	.LBB21_701
.Ltmp4194:
	.loc	21 0 29 is_stmt 0
	movq	8(%rsp), %rsi
	vmovss	(%rsi,%rdi,4), %xmm8
	movq	(%rsp), %r15
.Ltmp4195:
	.loc	26 103 24 is_stmt 1
	vmovss	(%r15,%rdi,4), %xmm9
	vbroadcastss	.LCPI21_2(%rip), %xmm0
	vandps	%xmm0, %xmm9, %xmm9
.Ltmp4196:
	.loc	26 103 24 is_stmt 0
	vandps	%xmm0, %xmm8, %xmm10
.Ltmp4197:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm10, %xmm9
.Ltmp4198:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp4199:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm10, %r15d
.Ltmp4200:
	.loc	26 161 24 is_stmt 1
	cmoval	%edi, %r15d
	vxorps	%xmm8, %xmm8, %xmm8
	vmovss	128(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp4201:
	.loc	26 161 24 is_stmt 0
	cmovbel	%edi, %r15d
	vmovss	124(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp4202:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm9, %xmm9
.Ltmp4203:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm10, %xmm10
.Ltmp4204:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp4205:
	.loc	7 1244 18
	vmovd	%xmm9, %edi
.Ltmp4206:
	.loc	26 161 24
	cmovbel	%r15d, %edi
.Ltmp4207:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp4208:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm9
.Ltmp4209:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %edi
.Ltmp4210:
	.loc	7 1291 18
	vmovd	%edi, %xmm9
.Ltmp4211:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm9
.Ltmp4212:
	.loc	26 161 24
	cmovbel	%r11d, %edi
.Ltmp4213:
	.loc	26 185 42
	movl	%edi, %r15d
	andl	$8388607, %r15d
	orl	$1065353216, %r15d
.Ltmp4214:
	.loc	7 1291 18
	vmovd	%r15d, %xmm9
	vmovss	.LCPI21_1(%rip), %xmm3
.Ltmp4215:
	.loc	26 66 9
	vaddss	%xmm3, %xmm9, %xmm9
.Ltmp4216:
	.loc	26 71 9
	vmulss	%xmm6, %xmm9, %xmm10
.Ltmp4217:
	.loc	26 61 9
	vsubss	%xmm10, %xmm13, %xmm10
.Ltmp4218:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4219:
	.loc	26 61 9
	vaddss	%xmm14, %xmm10, %xmm10
.Ltmp4220:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4221:
	.loc	26 61 9
	vaddss	%xmm2, %xmm10, %xmm10
.Ltmp4222:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4223:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm10, %xmm10
.Ltmp4224:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm10
.Ltmp4225:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm10, %xmm10
.Ltmp4226:
	.loc	26 187 28
	shrl	$23, %edi
	orl	$1258291200, %edi
.Ltmp4227:
	.loc	26 71 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp4228:
	.loc	7 1291 18
	vmovd	%edi, %xmm10
.Ltmp4229:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm10, %xmm10
.Ltmp4230:
	.loc	26 61 9
	vaddss	%xmm9, %xmm10, %xmm9
.Ltmp4231:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm9, %xmm9
.Ltmp4232:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm9, %xmm9
.Ltmp4233:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm9, %xmm9
.Ltmp4234:
	.loc	26 129 14 is_stmt 1
	vucomiss	120(%rsp), %xmm9
.Ltmp4235:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp4236:
	.loc	26 129 14
	vucomiss	%xmm11, %xmm9
.Ltmp4237:
	.loc	26 144 9
	movl	$0, %r15d
	adcl	$-1, %r15d
.Ltmp4238:
	.loc	26 149 9
	movl	%r13d, %edi
.Ltmp4239:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm4
.Ltmp4240:
	.loc	26 149 9
	notl	%edi
.Ltmp4241:
	.loc	26 139 9
	cmovbel	%ebp, %edi
.Ltmp4242:
	.loc	21 362 20
	vmovss	856(%rbx), %xmm10
.Ltmp4243:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm10
.Ltmp4244:
	.loc	26 144 9
	cmoval	%r13d, %r15d
.Ltmp4245:
	.loc	26 139 9
	cmovbel	%ebp, %edi
.Ltmp4246:
	.loc	26 161 24
	testb	$1, %dil
	jne	.LBB21_574
.Ltmp4247:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp4248:
	.loc	26 161 24
	testb	$1, %r15b
	je	.LBB21_577
	jmp	.LBB21_578
.Ltmp4249:
.LBB21_574:
	.loc	21 0 0
	vaddss	%xmm3, %xmm4, %xmm10
	vmovss	116(%rsp), %xmm4
.Ltmp4250:
	.loc	26 161 24
	testb	$1, %r15b
	jne	.LBB21_578
.Ltmp4251:
.LBB21_577:
	.loc	26 0 24
	vmovaps	%xmm10, %xmm4
.LBB21_578:
	orl	%r15d, %edi
	andl	$1065353216, %edi
	vmovd	%edi, %xmm10
.Ltmp4252:
	.loc	26 66 9 is_stmt 1
	vsubss	%xmm11, %xmm9, %xmm9
.Ltmp4253:
	.loc	26 71 9
	vmulss	112(%rsp), %xmm9, %xmm9
.Ltmp4254:
	.loc	26 161 24
	vmaxss	320(%rsp), %xmm9, %xmm9
.Ltmp4255:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm10, %xmm8, %xmm10
	vcmpltss	%xmm8, %xmm9, %xmm11
	vandps	%xmm11, %xmm10, %xmm10
	vmovd	%xmm10, %r15d
	testb	$1, %r15b
	jne	.LBB21_580
.Ltmp4256:
	.loc	26 0 44
	vxorps	%xmm9, %xmm9, %xmm9
.LBB21_580:
.Ltmp4257:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm7, %xmm9
	movl	84(%rsp), %r15d
.Ltmp4258:
	.loc	26 161 24
	cmovbel	80(%rsp), %r15d
	movq	8(%rsp), %rsi
.Ltmp4259:
	.loc	26 103 24
	vmovss	(%rsi,%r8,4), %xmm10
	vbroadcastss	.LCPI21_2(%rip), %xmm0
	vandps	%xmm0, %xmm10, %xmm10
	movq	(%rsp), %r13
.Ltmp4260:
	.loc	26 103 24 is_stmt 0
	vmovss	(%r13,%r8,4), %xmm11
	vandps	%xmm0, %xmm11, %xmm11
.Ltmp4261:
	.loc	26 124 14 is_stmt 1
	vucomiss	%xmm11, %xmm10
.Ltmp4262:
	.loc	7 1244 18
	vmovd	%xmm10, %r8d
.Ltmp4263:
	.loc	7 1244 18 is_stmt 0
	vmovd	%xmm11, %r13d
.Ltmp4264:
	.loc	26 161 24 is_stmt 1
	cmoval	%r8d, %r13d
	vmovss	104(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp4265:
	.loc	26 161 24 is_stmt 0
	cmovbel	%r8d, %r13d
	vmovss	96(%rsp), %xmm3
	vucomiss	%xmm8, %xmm3
.Ltmp4266:
	.loc	26 71 9 is_stmt 1
	vmulss	%xmm1, %xmm10, %xmm10
.Ltmp4267:
	.loc	26 71 9 is_stmt 0
	vmulss	%xmm1, %xmm11, %xmm11
.Ltmp4268:
	.loc	26 61 9 is_stmt 1
	vaddss	%xmm10, %xmm11, %xmm10
.Ltmp4269:
	.loc	7 1244 18
	vmovd	%xmm10, %r8d
.Ltmp4270:
	.loc	26 161 24
	cmovbel	%r13d, %r8d
.Ltmp4271:
	.loc	7 1291 18
	vmovd	%r8d, %xmm10
.Ltmp4272:
	.loc	26 124 14
	vucomiss	%xmm12, %xmm10
.Ltmp4273:
	.loc	7 1291 18
	vmovd	%r15d, %xmm10
.Ltmp4274:
	.loc	26 161 24
	movl	$841731191, %esi
	cmovbel	%esi, %r8d
.Ltmp4275:
	.loc	26 66 9
	vsubss	%xmm7, %xmm9, %xmm9
.Ltmp4276:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp4277:
	.loc	26 124 14
	vucomiss	%xmm5, %xmm11
.Ltmp4278:
	.loc	26 92 9
	vmulss	%xmm10, %xmm9, %xmm9
.Ltmp4279:
	.loc	26 161 24
	cmovbel	%r11d, %r8d
.Ltmp4280:
	.loc	26 185 42
	movl	%r8d, %r15d
	andl	$8388607, %r15d
	orl	$1065353216, %r15d
.Ltmp4281:
	.loc	7 1291 18
	vmovd	%r15d, %xmm10
	vmovss	.LCPI21_1(%rip), %xmm3
.Ltmp4282:
	.loc	26 66 9
	vaddss	%xmm3, %xmm10, %xmm10
.Ltmp4283:
	.loc	26 71 9
	vmulss	%xmm6, %xmm10, %xmm11
.Ltmp4284:
	.loc	26 61 9
	vsubss	%xmm11, %xmm13, %xmm11
.Ltmp4285:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp4286:
	.loc	26 61 9
	vaddss	%xmm14, %xmm11, %xmm11
.Ltmp4287:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp4288:
	.loc	26 61 9
	vaddss	%xmm2, %xmm11, %xmm11
.Ltmp4289:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp4290:
	.loc	26 61 9
	vaddss	.LCPI21_10(%rip), %xmm11, %xmm11
.Ltmp4291:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm11
.Ltmp4292:
	.loc	26 61 9
	vaddss	.LCPI21_11(%rip), %xmm11, %xmm11
.Ltmp4293:
	.loc	26 187 28
	shrl	$23, %r8d
	orl	$1258291200, %r8d
.Ltmp4294:
	.loc	26 71 9
	vmulss	%xmm11, %xmm10, %xmm10
.Ltmp4295:
	.loc	7 1291 18
	vmovd	%r8d, %xmm11
.Ltmp4296:
	.loc	26 187 13
	vaddss	.LCPI21_12(%rip), %xmm11, %xmm11
.Ltmp4297:
	.loc	26 61 9
	vaddss	%xmm10, %xmm11, %xmm11
	movq	24(%rsp), %r8
	vmovss	(%r8,%r12,4), %xmm10
.Ltmp4298:
	.loc	26 71 9
	vmulss	.LCPI21_13(%rip), %xmm11, %xmm11
.Ltmp4299:
	.loc	26 161 24
	vminss	.LCPI21_14(%rip), %xmm11, %xmm11
.Ltmp4300:
	.loc	26 161 24 is_stmt 0
	vmaxss	.LCPI21_15(%rip), %xmm11, %xmm14
.Ltmp4301:
	.loc	26 129 14 is_stmt 1
	vucomiss	100(%rsp), %xmm14
.Ltmp4302:
	.loc	26 28 5
	movl	$0, %r13d
	adcl	$-1, %r13d
.Ltmp4303:
	.loc	26 129 14
	vucomiss	32(%rsp), %xmm14
.Ltmp4304:
	.loc	26 92 9
	vaddss	%xmm7, %xmm9, %xmm7
.Ltmp4305:
	.loc	26 144 9
	movl	$0, %r15d
	adcl	$-1, %r15d
.Ltmp4306:
	.loc	21 0 0 is_stmt 0
	vmovss	932(%rbx), %xmm9
.Ltmp4307:
	.loc	26 149 9 is_stmt 1
	movl	%r13d, %r8d
	vmovss	48(%rsp), %xmm11
.Ltmp4308:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm11
.Ltmp4309:
	.loc	26 149 9
	notl	%r8d
.Ltmp4310:
	.loc	26 139 9
	cmovbel	%ebp, %r8d
.Ltmp4311:
	.loc	26 124 14
	vucomiss	%xmm8, %xmm9
.Ltmp4312:
	.loc	26 103 24
	vandps	%xmm0, %xmm7, %xmm9
.Ltmp4313:
	.loc	26 166 24
	vcmpnltss	.LCPI21_17(%rip), %xmm9, %xmm9
	vandps	%xmm7, %xmm9, %xmm7
	movq	40(%rsp), %rsi
	vmovss	(%rsi,%r12,4), %xmm9
.Ltmp4314:
	.loc	21 373 5
	vmovss	%xmm4, 860(%rbx)
	.loc	21 382 5
	movl	%edi, 856(%rbx)
.Ltmp4315:
	.loc	21 394 5
	vmovss	%xmm7, 864(%rbx)
.Ltmp4316:
	.loc	26 144 9
	cmoval	%r13d, %r15d
.Ltmp4317:
	.loc	26 139 9
	cmovbel	%ebp, %r8d
.Ltmp4318:
	.loc	26 161 24
	testb	$1, %r8b
	je	.LBB21_582
.Ltmp4319:
	.loc	21 0 0 is_stmt 0
	vaddss	%xmm3, %xmm11, %xmm11
.LBB21_582:
	movq	136(%rsp), %r12
	vmovss	92(%rsp), %xmm3
.Ltmp4320:
	.loc	26 161 24 is_stmt 1
	testb	$1, %r15b
	jne	.LBB21_584
.Ltmp4321:
	.loc	26 0 24 is_stmt 0
	vmovaps	%xmm11, %xmm3
.LBB21_584:
.Ltmp4322:
	vmulss	.LCPI21_18(%rip), %xmm7, %xmm11
	vmaxss	.LCPI21_19(%rip), %xmm11, %xmm11
	vminss	.LCPI21_20(%rip), %xmm11, %xmm12
	vroundss	$9, %xmm12, %xmm12, %xmm11
	vsubss	%xmm11, %xmm12, %xmm12
.Ltmp4323:
	orl	%r15d, %r8d
	andl	$1065353216, %r8d
	vmovd	%r8d, %xmm15
	vmovss	%xmm3, 48(%rsp)
.Ltmp4324:
	.loc	21 373 5 is_stmt 1
	vmovss	%xmm3, 936(%rbx)
	.loc	21 382 5
	movl	%r8d, 932(%rbx)
.Ltmp4325:
	.loc	26 66 9
	vsubss	32(%rsp), %xmm14, %xmm14
.Ltmp4326:
	.loc	26 71 9
	vmulss	88(%rsp), %xmm14, %xmm14
.Ltmp4327:
	.loc	26 161 24
	vmaxss	304(%rsp), %xmm14, %xmm14
.Ltmp4328:
	.loc	26 161 44 is_stmt 0
	vcmpnltss	%xmm15, %xmm8, %xmm15
	vcmpltss	%xmm8, %xmm14, %xmm3
	vandps	%xmm3, %xmm15, %xmm3
	vmovd	%xmm3, %edi
	testb	$1, %dil
	jne	.LBB21_586
.Ltmp4329:
	.loc	26 0 44
	vxorps	%xmm14, %xmm14, %xmm14
	jmp	.LBB21_586
.LBB21_587:
	movq	496(%rsp), %r11
	movq	%r11, %rax
	movq	344(%rsp), %rsi
	subq	%rsi, %rax
.Ltmp4330:
	.loc	15 2584 13 is_stmt 1
	addl	%r8d, %eax
.Ltmp4331:
	.loc	21 297 5
	movl	%eax, 1208(%rbx)
	movq	368(%rsp), %r8
	movq	360(%rsp), %r15
	movq	248(%rsp), %rcx
	movq	240(%rsp), %r13
.Ltmp4332:
.LBB21_2:
	.loc	6 698 9
	subl	%esi, 1220(%rbx)
	movq	504(%rsp), %r10
.Ltmp4333:
	.loc	6 765 33
	leaq	400(%rsp), %r9
	movq	%r13, 400(%rsp)
	movq	%r15, 408(%rsp)
	movq	%rcx, 416(%rsp)
	movq	%r8, 424(%rsp)
	movl	72(%rbx), %eax
	movq	%rax, 176(%rsp)
	vxorps	%xmm15, %xmm15, %xmm15
	vcvtsi2sd	%rax, %xmm15, %xmm3
	leaq	744(%rbx), %rax
	movq	%rax, 248(%rsp)
	leaq	792(%rbx), %rax
	movq	%rax, 240(%rsp)
	movq	1200(%rbx), %rbp
	movl	1216(%rbx), %eax
	movl	%eax, 160(%rsp)
	movl	$24, %r14d
	xorl	%r13d, %r13d
	vbroadcastss	.LCPI21_2(%rip), %xmm4
	vmovss	.LCPI21_27(%rip), %xmm5
	vmovsd	.LCPI21_28(%rip), %xmm6
	vmovsd	.LCPI21_29(%rip), %xmm7
	vmovsd	.LCPI21_30(%rip), %xmm8
	xorl	%r15d, %r15d
	xorl	%ecx, %ecx
	movq	%rbx, 376(%rsp)
	vmovsd	%xmm3, 192(%rsp)
	vmovaps	%xmm4, 256(%rsp)
	jmp	.LBB21_3
	.loc	6 0 33 is_stmt 0
.Ltmp4334:
	.p2align	4
.LBB21_491:
	movq	(%r10,%r14), %rax
.Ltmp4335:
	.loc	15 2428 13 is_stmt 1
	addq	%r11, %rax
	movq	$-1, %rcx
	cmovbq	%rcx, %rax
.Ltmp4336:
	.loc	6 786 17
	movq	%rax, (%r10,%r14)
	leaq	400(%rsp), %r9
.Ltmp4337:
.LBB21_492:
	.loc	6 0 17 is_stmt 0
	movl	$1, %ecx
	movl	$32, %r14d
.Ltmp4338:
	.file	27 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/ops/index_range.rs"
	.loc	27 131 12 is_stmt 1
	testb	$1, %r15b
	movb	$1, %r15b
	jne	.LBB21_493
.Ltmp4339:
.LBB21_3:
	.loc	25 253 13
	movq	%rcx, %rdx
	shlq	$4, %rdx
.Ltmp4340:
	.loc	1 1733 9
	movq	(%r9,%rdx), %rax
	movq	8(%r9,%rdx), %rdi
.Ltmp4341:
	.loc	6 766 13
	imulq	$76, %rcx, %r12
	vmovss	864(%rbx,%r12), %xmm0
.Ltmp4342:
	.loc	16 2155 12
	testq	%rdi, %rdi
	je	.LBB21_474
.Ltmp4343:
	.loc	16 0 12 is_stmt 0
	movl	$-1, %esi
	xorl	%r8d, %r8d
	.p2align	4
.LBB21_5:
.Ltmp4344:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rax,%r8,4), %xmm1
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp4345:
	.loc	26 114 14
	vucomiss	%xmm1, %xmm5
.Ltmp4346:
	.loc	26 139 9
	cmovbel	%r13d, %esi
.Ltmp4347:
	.loc	16 2155 12
	incq	%r8
	cmpq	%r8, %rdi
	jne	.LBB21_5
.Ltmp4348:
	.loc	16 0 12 is_stmt 0
	vandps	%xmm4, %xmm0, %xmm0
	vucomiss	%xmm0, %xmm5
	.loc	6 767 16 is_stmt 1
	jbe	.LBB21_8
	cmpl	$-1, %esi
	je	.LBB21_492
.LBB21_8:
	.loc	6 0 16 is_stmt 0
	movl	$-1, %esi
	xorl	%r8d, %r8d
	.p2align	4
.LBB21_9:
.Ltmp4349:
	.loc	26 103 24 is_stmt 1
	vmovss	(%rax,%r8,4), %xmm1
	vandps	%xmm4, %xmm1, %xmm1
.Ltmp4350:
	.loc	26 114 14
	vucomiss	%xmm1, %xmm5
.Ltmp4351:
	.loc	26 139 9
	cmovbel	%r13d, %esi
.Ltmp4352:
	.loc	16 2155 12
	incq	%r8
	cmpq	%r8, %rdi
	jne	.LBB21_9
.Ltmp4353:
	.file	28 "/home/bl/misofm/engine-gate-detector-access" "crates/effect-runtime/src/bank.rs"
	.loc	28 185 12
	notl	%esi
	xorl	%r8d, %r8d
	testl	$1065353216, %esi
	setne	%r8b
	jmp	.LBB21_476
.Ltmp4354:
	.loc	28 0 12 is_stmt 0
.Ltmp4355:
	.p2align	4
.LBB21_474:
	.loc	26 103 24 is_stmt 1
	vandps	%xmm4, %xmm0, %xmm0
.Ltmp4356:
	.loc	26 114 14
	vucomiss	%xmm0, %xmm5
.Ltmp4357:
	.loc	6 767 43
	ja	.LBB21_492
	.loc	6 0 43 is_stmt 0
	xorl	%r8d, %r8d
.LBB21_476:
.Ltmp4358:
	.loc	26 114 14 is_stmt 1
	xorl	%esi, %esi
	vucomiss	%xmm0, %xmm5
	setbe	%sil
	orl	%r8d, %esi
	je	.LBB21_492
.Ltmp4359:
	.loc	26 0 14 is_stmt 0
	testq	%r11, %r11
.Ltmp4360:
	.loc	11 900 12 is_stmt 1
	je	.LBB21_481
.Ltmp4361:
	.loc	11 0 12 is_stmt 0
	xorl	%esi, %esi
	.p2align	4
.LBB21_479:
.Ltmp4362:
	.loc	6 777 21 is_stmt 1
	cmpq	%rsi, %rdi
	je	.LBB21_712
	movl	$0, (%rax,%rsi,4)
.Ltmp4363:
	.loc	15 971 17
	incq	%rsi
.Ltmp4364:
	.loc	8 1916 50
	cmpq	%rsi, %r11
.Ltmp4365:
	.loc	11 900 12
	jne	.LBB21_479
.Ltmp4366:
.LBB21_481:
	.loc	11 0 12 is_stmt 0
	movq	%rcx, %r8
	shlq	$6, %r8
	testq	%rbp, %rbp
.Ltmp4367:
	.loc	11 900 12
	je	.LBB21_486
.Ltmp4368:
	.loc	11 0 12
	leaq	104(%rbx), %rax
	leaq	(%rax,%r8), %r9
	movq	8(%r9), %rdi
	xorl	%eax, %eax
	.p2align	4
.LBB21_483:
.Ltmp4369:
	.loc	6 562 13 is_stmt 1
	cmpq	%rax, %rdi
	je	.LBB21_710
	movq	(%r9), %rsi
	movl	$0, (%rsi,%rax,4)
	.loc	6 564 17
	movq	24(%r9), %rsi
	cmpq	%rsi, %rax
	jae	.LBB21_711
	movq	16(%r9), %rsi
	movl	$0, (%rsi,%rax,4)
.Ltmp4370:
	.loc	6 0 0 is_stmt 0
	incq	%rax
.Ltmp4371:
	.loc	8 1916 50 is_stmt 1
	cmpq	%rax, %rbp
.Ltmp4372:
	.loc	11 900 12
	jne	.LBB21_483
.Ltmp4373:
.LBB21_486:
	.loc	11 0 12 is_stmt 0
	movq	%rcx, %rax
	shlq	$5, %rax
	leaq	232(%rbx), %rsi
	addq	%rsi, %rax
	leaq	944(%rbx), %rsi
	addq	%rsi, %rdx
.Ltmp4374:
	.loc	6 506 22 is_stmt 1
	vmovss	(%rax), %xmm12
	vmovss	4(%rax), %xmm11
	vmovss	8(%rax), %xmm10
	vmovss	12(%rax), %xmm9
	vmovss	16(%rax), %xmm0
	vmovss	20(%rax), %xmm2
	vmovss	24(%rax), %xmm13
	vmovss	28(%rax), %xmm1
.Ltmp4375:
	.loc	6 507 9
	vmovss	%xmm1, (%rdx)
	vmovss	%xmm0, 4(%rdx)
	vmovss	%xmm2, 8(%rdx)
	vmovss	%xmm13, 12(%rdx)
.Ltmp4376:
	.loc	9 82 17
	vcvtss2sd	%xmm1, %xmm1, %xmm1
.Ltmp4377:
	.loc	6 403 20
	vmulsd	%xmm1, %xmm3, %xmm1
	vdivsd	%xmm6, %xmm1, %xmm1
	.loc	6 403 19 is_stmt 0
	vaddsd	%xmm7, %xmm1, %xmm1
.Ltmp4378:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm1, %xmm1, %xmm1
	vmovq	%xmm1, %rax
.Ltmp4379:
	.loc	6 404 9
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
	jne	.LBB21_491
	vucomisd	%xmm8, %xmm1
	ja	.LBB21_491
.Ltmp4380:
	.loc	9 82 17
	vcvtss2sd	%xmm2, %xmm2, %xmm2
.Ltmp4381:
	.loc	6 403 20
	vmulsd	%xmm2, %xmm3, %xmm2
	vdivsd	%xmm6, %xmm2, %xmm2
	.loc	6 403 19 is_stmt 0
	vaddsd	%xmm7, %xmm2, %xmm2
.Ltmp4382:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm2, %xmm2, %xmm2
	vmovq	%xmm2, %rax
.Ltmp4383:
	.loc	6 404 9
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
	jne	.LBB21_491
	vucomisd	%xmm8, %xmm2
	ja	.LBB21_491
.Ltmp4384:
	.loc	6 0 9 is_stmt 0
	addq	%rbx, %r8
	addq	240(%rsp), %r12
	leaq	(%rcx,%rcx,2), %rax
	movq	248(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rsi
	movq	%rsi, (%rsp)
	vxorps	%xmm5, %xmm5, %xmm5
.Ltmp4385:
	.loc	6 407 10 is_stmt 1
	vmaxsd	%xmm1, %xmm5, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rax
.Ltmp4386:
	.loc	6 407 10 is_stmt 0
	vmaxsd	%xmm2, %xmm5, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rcx
	movl	160(%rsp), %edx
.Ltmp4387:
	.loc	6 533 9 is_stmt 1
	subl	%eax, %edx
	cmovbl	%r13d, %edx
	movl	%edx, 136(%r8)
	.loc	6 534 62
	movl	%ecx, %eax
	vcvtsi2ss	%rax, %xmm15, %xmm1
	vmovss	%xmm1, 16(%rsp)
.Ltmp4388:
	.loc	6 398 5
	vmovss	%xmm1, 8(%rsi)
	movq	176(%rsp), %rbx
.Ltmp4389:
	.loc	6 538 13
	movl	%ebx, %edi
	vmovss	%xmm9, 8(%rsp)
	vmovss	%xmm10, 24(%rsp)
	vmovss	%xmm11, 40(%rsp)
	vmovss	%xmm12, 32(%rsp)
	vmovss	%xmm13, 48(%rsp)
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movq	(%rsp), %rax
.Ltmp4390:
	.loc	6 398 5
	vmovss	%xmm0, (%rax)
	vmovss	48(%rsp), %xmm0
.Ltmp4391:
	.loc	6 543 13
	movl	%ebx, %edi
	movq	376(%rsp), %rbx
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	vmovsd	.LCPI21_30(%rip), %xmm8
	vmovsd	.LCPI21_29(%rip), %xmm7
	vmovsd	.LCPI21_28(%rip), %xmm6
	vmovss	.LCPI21_27(%rip), %xmm5
	vmovaps	256(%rsp), %xmm4
	vmovsd	192(%rsp), %xmm3
	movq	504(%rsp), %r10
	movq	496(%rsp), %r11
	movq	(%rsp), %rax
.Ltmp4392:
	.loc	6 398 5
	vmovss	%xmm0, 4(%rax)
.Ltmp4393:
	.loc	6 398 5 is_stmt 0
	movl	$0, 72(%r12)
.Ltmp4394:
	.loc	6 398 5
	movl	$1065353216, 64(%r12)
	vmovss	16(%rsp), %xmm0
.Ltmp4395:
	.loc	6 398 5
	vmovss	%xmm0, 68(%r12)
	vmovss	32(%rsp), %xmm0
.Ltmp4396:
	.loc	6 398 5
	vmovss	%xmm0, (%r12)
.Ltmp4397:
	.loc	6 398 5
	vmovss	%xmm0, 4(%r12)
.Ltmp4398:
	.loc	6 398 5
	movq	$0, 8(%r12)
	vmovss	40(%rsp), %xmm0
.Ltmp4399:
	.loc	6 398 5
	vmovss	%xmm0, 16(%r12)
.Ltmp4400:
	.loc	6 398 5
	vmovss	%xmm0, 20(%r12)
.Ltmp4401:
	.loc	6 398 5
	movq	$0, 24(%r12)
	vmovss	24(%rsp), %xmm0
.Ltmp4402:
	.loc	6 398 5
	vmovss	%xmm0, 32(%r12)
.Ltmp4403:
	.loc	6 398 5
	vmovss	%xmm0, 36(%r12)
.Ltmp4404:
	.loc	6 398 5
	movq	$0, 40(%r12)
	vmovss	8(%rsp), %xmm0
.Ltmp4405:
	.loc	6 398 5
	vmovss	%xmm0, 48(%r12)
.Ltmp4406:
	.loc	6 398 5
	vmovss	%xmm0, 52(%r12)
.Ltmp4407:
	.loc	6 398 5
	movq	$0, 56(%r12)
	jmp	.LBB21_491
.Ltmp4408:
.LBB21_493:
	.loc	6 700 6 epilogue_begin is_stmt 1
	addq	$440, %rsp
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
.LBB21_460:
	.cfi_def_cfa_offset 496
.Ltmp4409:
	.loc	25 443 13
	leaq	.Lalloc_97ed0782c9e7425b1b215f66cb64a347(%rip), %rcx
	movq	344(%rsp), %rdi
.Ltmp4410:
	.loc	25 443 13 is_stmt 0
	movq	%r15, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4411:
.LBB21_464:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_e2f4f8781efb7cd41cc45e538dfda195(%rip), %rcx
	movq	56(%rsp), %rdi
.Ltmp4412:
	.loc	25 443 13 is_stmt 0
	movq	%r15, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4413:
.LBB21_715:
	.loc	25 0 13
	leaq	1(%rdi), %rsi
.Ltmp4414:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_ad8d9ef662eaae13c96be161fe1fd2e1(%rip), %rcx
	movq	56(%rsp), %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4415:
.LBB21_677:
	.loc	25 443 13
	leaq	.Lalloc_202abc45ab97a16825849449dde94dfa(%rip), %rcx
	movq	288(%rsp), %rdi
.Ltmp4416:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4417:
.LBB21_674:
	.loc	25 0 13
	movq	280(%rsp), %rdi
.Ltmp4418:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_e2f4f8781efb7cd41cc45e538dfda195(%rip), %rcx
.Ltmp4419:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4420:
.LBB21_714:
	.loc	25 0 13
	leaq	1(%rdi), %rsi
.Ltmp4421:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_72e24c5578221343e8a784545a3910d2(%rip), %rcx
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4422:
.LBB21_681:
	.loc	25 0 13 is_stmt 0
	leaq	1(%r12), %rsi
.Ltmp4423:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_0f4c27429e4885e1b619f1e4a3587acc(%rip), %rcx
.Ltmp4424:
	.loc	6 0 0 is_stmt 0
	movq	%r12, %rdi
	movq	%rbp, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB21_463:
	leaq	1(%r8), %rsi
.Ltmp4425:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_72e24c5578221343e8a784545a3910d2(%rip), %rcx
	movq	%r8, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4426:
.LBB21_469:
	.loc	25 0 13 is_stmt 0
	leaq	1(%r12), %rsi
.Ltmp4427:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_0f4c27429e4885e1b619f1e4a3587acc(%rip), %rcx
	movq	%r12, %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4428:
.LBB21_510:
	.loc	25 443 13
	leaq	.Lalloc_aebeae245abdbd708a3d39b73262e641(%rip), %rcx
	movq	216(%rsp), %rdi
.Ltmp4429:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4430:
.LBB21_561:
	.loc	25 0 13
	movq	216(%rsp), %rdi
.Ltmp4431:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_97ed0782c9e7425b1b215f66cb64a347(%rip), %rcx
.Ltmp4432:
	.loc	25 443 13 is_stmt 0
	movq	%r14, %rsi
	movq	%rdi, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4433:
.LBB21_567:
	.loc	25 0 13
	movq	256(%rsp), %r13
.LBB21_568:
	leaq	1(%r12), %rsi
.Ltmp4434:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_ba99eeeb3482270ebe1c674b4013dd6a(%rip), %rcx
	movq	%r12, %rdi
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4435:
.LBB21_531:
	.loc	25 0 13 is_stmt 0
	movq	256(%rsp), %r13
.LBB21_565:
	leaq	1(%rdi), %rsi
.Ltmp4436:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_cad5a7b24766ffcdda9cbfe066eb4078(%rip), %rcx
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4437:
.LBB21_636:
	.loc	25 0 13 is_stmt 0
	movq	160(%rsp), %rdx
.LBB21_637:
	leaq	1(%rdi), %rsi
.Ltmp4438:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_a8b70dfa50e86ad5a1ce9afbe0da1688(%rip), %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4439:
.LBB21_706:
	.loc	25 443 13
	leaq	.Lalloc_2154676760dbbfbf0360c56922cbcced(%rip), %rcx
	xorl	%edi, %edi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4440:
.LBB21_709:
	.loc	25 456 13
	leaq	.Lalloc_a0e50f2d670fa7dbeabdb68abcb7ac10(%rip), %rcx
	xorl	%edi, %edi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4441:
.LBB21_705:
	.loc	25 581 13
	leaq	.Lalloc_9c634077742a23e4340a846355ecc484(%rip), %rcx
	movq	%rsi, %rdi
	movq	%r8, %rsi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4442:
.LBB21_707:
	.loc	25 443 13
	leaq	.Lalloc_fc4bf442cf077b790fbd54430ab2bad8(%rip), %rcx
	xorl	%edi, %edi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4443:
.LBB21_712:
	.loc	6 777 21
	leaq	.Lalloc_30d570589940b68d7f77af9df77eb2ee(%rip), %rdx
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4444:
.LBB21_711:
	.loc	6 564 17
	leaq	.Lalloc_25eed8128df6f66da1727b7a67d9a9d8(%rip), %rdx
	movq	%rax, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_710:
	.loc	6 562 13
	leaq	.Lalloc_f8f0512af3f0ba047152c3c522a44b59(%rip), %rdx
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4445:
.LBB21_499:
	.loc	25 569 13
	leaq	.Lalloc_30a58d326fa295091a02a55f77ea093b(%rip), %rcx
.Ltmp4446:
	.loc	25 569 13 is_stmt 0
	movq	%rsi, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4447:
.LBB21_498:
	.loc	25 569 13 is_stmt 1
	leaq	.Lalloc_9ed981539d8cc9a26fc3fd872195988f(%rip), %rcx
.Ltmp4448:
	.loc	25 569 13 is_stmt 0
	movq	%rsi, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4449:
.LBB21_564:
	.loc	25 0 13
	movq	%r8, %r13
	leaq	1(%rdi), %rsi
.Ltmp4450:
	.loc	25 456 13 is_stmt 1
	leaq	.Lalloc_cad5a7b24766ffcdda9cbfe066eb4078(%rip), %rcx
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4451:
.LBB21_473:
	.loc	21 274 29
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r13, %rdi
	movq	(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_472:
	.loc	21 275 29
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r13, %rdi
	movq	32(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_471:
	.loc	21 276 29
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%rbp, %rdi
	movq	32(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_470:
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rbp, %rdi
	movq	(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4452:
.LBB21_702:
	.loc	21 276 29
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
	movq	%r8, %rdi
	movq	56(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_703:
	.loc	21 275 29
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	56(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_704:
	.loc	21 274 29
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	160(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB21_701:
	.loc	21 277 29
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%r8, %rdi
	movq	160(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4453:
.Lfunc_end21:
	.size	_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_, .Lfunc_end21-_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_
