_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_:
.Lfunc_begin25:
	.loc	1 1320 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
.Ltmp3835:
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	andq	$-32, %rsp
	subq	$800, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.loc	1 949 19 prologue_end
	movq	8(%rsi), %rdx
.Ltmp3836:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB25_88
.Ltmp3837:
	.loc	5 0 16 is_stmt 0
	movq	%rdi, %rbx
	movq	192(%rsi), %rdi
.Ltmp3838:
	.loc	5 568 12 is_stmt 1
	movq	%rdx, %rax
	subq	%rdi, %rax
	jb	.LBB25_92
.Ltmp3839:
	.loc	5 438 16
	cmpq	$7, %rax
	jbe	.LBB25_93
.Ltmp3840:
	.loc	1 949 57
	leaq	(%rdi,%rdi), %rax
.Ltmp3841:
	.loc	5 568 12
	movq	%rdx, %r8
	subq	%rax, %r8
	jb	.LBB25_94
.Ltmp3842:
	.loc	5 438 16
	cmpq	$7, %r8
	jbe	.LBB25_89
.Ltmp3843:
	.loc	1 949 57
	leaq	(%rdi,%rdi,2), %rcx
.Ltmp3844:
	.loc	5 568 12
	movq	%rdx, %r8
	subq	%rcx, %r8
	jb	.LBB25_95
.Ltmp3845:
	.loc	5 438 16
	cmpq	$7, %r8
	jbe	.LBB25_89
.Ltmp3846:
	.loc	1 949 57
	leaq	(,%rdi,4), %r8
.Ltmp3847:
	.loc	5 568 12
	movq	%rdx, %r9
	subq	%r8, %r9
	jb	.LBB25_96
.Ltmp3848:
	.loc	5 438 16
	cmpq	$7, %r9
	jbe	.LBB25_97
.Ltmp3849:
	.loc	1 949 57
	leaq	(%rdi,%rdi,4), %r9
.Ltmp3850:
	.loc	5 568 12
	movq	%rdx, %r10
	subq	%r9, %r10
	jb	.LBB25_98
.Ltmp3851:
	.loc	5 438 16
	cmpq	$7, %r10
	jbe	.LBB25_99
.Ltmp3852:
	.loc	1 949 57
	leaq	(%rax,%rax,2), %r10
.Ltmp3853:
	.loc	5 568 12
	movq	%rdx, %r11
	subq	%r10, %r11
	jb	.LBB25_100
.Ltmp3854:
	.loc	5 438 16
	cmpq	$7, %r11
	jbe	.LBB25_101
.Ltmp3855:
	.loc	1 949 57
	leaq	(,%rdi,8), %r11
	movq	%r11, %r14
	subq	%rdi, %r14
.Ltmp3856:
	.loc	5 568 12
	movq	%rdx, %r15
	subq	%r14, %r15
	jb	.LBB25_102
.Ltmp3857:
	.loc	5 438 16
	cmpq	$7, %r15
	jbe	.LBB25_90
.Ltmp3858:
	.loc	5 568 12
	movq	%rdx, %r15
	subq	%r11, %r15
	jb	.LBB25_103
.Ltmp3859:
	.loc	5 438 16
	cmpq	$7, %r15
	jbe	.LBB25_90
.Ltmp3860:
	.loc	1 949 57
	leaq	(%rdi,%rdi,8), %r15
.Ltmp3861:
	.loc	5 568 12
	movq	%rdx, %r12
	subq	%r15, %r12
	jb	.LBB25_104
.Ltmp3862:
	.loc	5 438 16
	cmpq	$7, %r12
	jbe	.LBB25_105
.Ltmp3863:
	.loc	5 0 16 is_stmt 0
	movq	%r10, 152(%rsp)
.Ltmp3864:
	.loc	1 949 57 is_stmt 1
	leaq	(%rax,%rax,4), %r12
.Ltmp3865:
	.loc	5 568 12
	movq	%rdx, %r13
	subq	%r12, %r13
	jb	.LBB25_106
.Ltmp3866:
	.loc	5 438 16
	cmpq	$7, %r13
	jbe	.LBB25_91
.Ltmp3867:
	.loc	5 0 16 is_stmt 0
	movq	%r8, %r10
	movq	%rcx, %r8
	movq	%rsi, %rcx
.Ltmp3868:
	.loc	1 949 57 is_stmt 1
	leaq	(%rdi,%r9,2), %rsi
.Ltmp3869:
	.loc	5 568 12
	movq	%rdx, %r13
	subq	%rsi, %r13
	jb	.LBB25_107
.Ltmp3870:
	.loc	5 438 16
	cmpq	$7, %r13
	jbe	.LBB25_91
.Ltmp3871:
	.loc	5 0 16 is_stmt 0
	movq	%rcx, %r13
	movq	(%rcx), %rdx
.Ltmp3872:
	.loc	25 151 30 is_stmt 1
	vmovups	(%rdx,%rsi,4), %ymm0
	vmovaps	%ymm0, 736(%rsp)
.Ltmp3873:
	.loc	1 950 9
	vmovups	(%rdx), %ymm0
	vmovaps	%ymm0, 384(%rsp)
	vmovups	(%rdx,%rdi,4), %ymm0
	vmovaps	%ymm0, 416(%rsp)
	vmovups	(%rdx,%rax,4), %ymm0
	vmovaps	%ymm0, 448(%rsp)
	vmovups	(%rdx,%r8,4), %ymm0
	vmovaps	%ymm0, 480(%rsp)
	vmovups	(%rdx,%r10,4), %ymm0
	vmovaps	%ymm0, 512(%rsp)
	vmovups	(%rdx,%r9,4), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	movq	152(%rsp), %rax
	vmovups	(%rdx,%rax,4), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	vmovups	(%rdx,%r14,4), %ymm0
	vmovaps	%ymm0, 608(%rsp)
	vmovups	(%rdx,%r11,4), %ymm0
	vmovaps	%ymm0, 640(%rsp)
	vmovups	(%rdx,%r15,4), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovups	(%rdx,%r12,4), %ymm0
	vmovaps	%ymm0, 704(%rsp)
.Ltmp3874:
	.loc	1 1323 30
	movq	184(%rcx), %rax
	vxorps	%xmm3, %xmm3, %xmm3
.Ltmp3875:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp3876:
	.loc	6 180 28
	je	.LBB25_33
.Ltmp3877:
	.loc	1 0 0 is_stmt 0
	movq	176(%r13), %rcx
.Ltmp3878:
	.loc	1 1324 28 is_stmt 1
	movl	(%rcx), %edx
	.loc	1 1324 13 is_stmt 0
	vcvtsi2ss	%rdx, %xmm15, %xmm4
.Ltmp3879:
	.loc	7 1714 9 is_stmt 1
	cmpq	$1, %rax
.Ltmp3880:
	.loc	6 180 28
	je	.LBB25_34
.Ltmp3881:
	.loc	1 1324 28
	movl	12(%rcx), %edx
	.loc	1 1324 13 is_stmt 0
	vcvtsi2ss	%rdx, %xmm15, %xmm5
.Ltmp3882:
	.loc	7 1714 9 is_stmt 1
	cmpq	$2, %rax
.Ltmp3883:
	.loc	6 180 28
	je	.LBB25_35
.Ltmp3884:
	.loc	1 1324 28
	movl	24(%rcx), %edx
	.loc	1 1324 13 is_stmt 0
	vcvtsi2ss	%rdx, %xmm15, %xmm7
.Ltmp3885:
	.loc	7 1714 9 is_stmt 1
	cmpq	$3, %rax
.Ltmp3886:
	.loc	6 180 28
	je	.LBB25_36
.Ltmp3887:
	.loc	1 1324 28
	movl	36(%rcx), %edx
	.loc	1 1324 13 is_stmt 0
	vcvtsi2ss	%rdx, %xmm15, %xmm8
.Ltmp3888:
	.loc	7 1714 9 is_stmt 1
	cmpq	$4, %rax
.Ltmp3889:
	.loc	6 180 28
	je	.LBB25_37
.Ltmp3890:
	.loc	1 1324 28
	movl	48(%rcx), %edx
	.loc	1 1324 13 is_stmt 0
	vcvtsi2ss	%rdx, %xmm15, %xmm1
.Ltmp3891:
	.loc	7 1714 9 is_stmt 1
	cmpq	$5, %rax
.Ltmp3892:
	.loc	6 180 28
	je	.LBB25_38
.Ltmp3893:
	.loc	1 1324 28
	movl	60(%rcx), %edx
	.loc	1 1324 13 is_stmt 0
	vcvtsi2ss	%rdx, %xmm15, %xmm2
.Ltmp3894:
	.loc	7 1714 9 is_stmt 1
	cmpq	$6, %rax
.Ltmp3895:
	.loc	6 180 28
	je	.LBB25_39
.Ltmp3896:
	.loc	1 1324 28
	movl	72(%rcx), %edx
	.loc	1 1324 13 is_stmt 0
	vcvtsi2ss	%rdx, %xmm15, %xmm6
.Ltmp3897:
	.loc	7 1714 9 is_stmt 1
	cmpq	$7, %rax
.Ltmp3898:
	.loc	6 180 28
	je	.LBB25_40
.Ltmp3899:
	.loc	7 1714 9
	cmpq	$8, %rax
.Ltmp3900:
	.loc	6 180 28
	jne	.LBB25_109
.Ltmp3901:
	.loc	1 0 0 is_stmt 0
	movl	84(%rcx), %eax
	vcvtsi2ss	%rax, %xmm15, %xmm3
.Ltmp3902:
	.loc	6 180 28
	jmp	.LBB25_40
.Ltmp3903:
.LBB25_33:
	.loc	6 0 28
	vxorps	%xmm4, %xmm4, %xmm4
.LBB25_34:
	vxorps	%xmm5, %xmm5, %xmm5
.LBB25_35:
	vxorps	%xmm7, %xmm7, %xmm7
.LBB25_36:
	vxorps	%xmm8, %xmm8, %xmm8
.LBB25_37:
	vxorps	%xmm1, %xmm1, %xmm1
.LBB25_38:
	vxorps	%xmm2, %xmm2, %xmm2
.LBB25_39:
	vxorps	%xmm6, %xmm6, %xmm6
.LBB25_40:
	.loc	1 1328 32 is_stmt 1
	movq	72(%r13), %rdx
.Ltmp3904:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB25_88
.Ltmp3905:
	.loc	1 1328 32
	movq	64(%r13), %rax
.Ltmp3906:
	.loc	25 151 30
	vmovups	(%rax), %ymm0
	vmovaps	%ymm0, 320(%rsp)
.Ltmp3907:
	.loc	1 1329 30
	movq	104(%r13), %rdx
.Ltmp3908:
	.loc	5 438 16
	cmpq	$7, %rdx
	jbe	.LBB25_88
.Ltmp3909:
	.loc	5 0 16 is_stmt 0
	vmovss	%xmm6, 288(%rsp)
	vmovss	%xmm2, 292(%rsp)
	vmovss	%xmm1, 296(%rsp)
	vmovss	%xmm8, 300(%rsp)
	vmovss	%xmm7, 304(%rsp)
	vmovss	%xmm5, 308(%rsp)
	vmovss	%xmm4, 312(%rsp)
	vmovss	%xmm3, 316(%rsp)
	.loc	1 1329 30 is_stmt 1
	movq	96(%r13), %rax
.Ltmp3910:
	.loc	25 151 30
	vmovups	(%rax), %ymm0
	vmovaps	%ymm0, 352(%rsp)
.Ltmp3911:
	.loc	1 1331 38
	movq	136(%r13), %rax
	vxorps	%xmm4, %xmm4, %xmm4
	vxorps	%xmm9, %xmm9, %xmm9
	vxorps	%xmm11, %xmm11, %xmm11
	vxorps	%xmm14, %xmm14, %xmm14
	vxorps	%xmm1, %xmm1, %xmm1
	vmovss	%xmm1, 148(%rsp)
	vmovss	%xmm1, 120(%rsp)
	vmovss	%xmm1, 88(%rsp)
	vmovss	%xmm1, 56(%rsp)
	vxorps	%xmm10, %xmm10, %xmm10
	vxorps	%xmm13, %xmm13, %xmm13
	vmovss	%xmm1, 212(%rsp)
	vmovss	%xmm1, 188(%rsp)
	vmovss	%xmm1, 144(%rsp)
	vmovss	%xmm1, 116(%rsp)
	vmovss	%xmm1, 84(%rsp)
	vmovss	%xmm1, 52(%rsp)
	vxorps	%xmm12, %xmm12, %xmm12
	vxorps	%xmm2, %xmm2, %xmm2
	vmovss	%xmm2, 208(%rsp)
	vmovss	%xmm2, 184(%rsp)
	vmovss	%xmm2, 140(%rsp)
	vmovss	%xmm2, 112(%rsp)
	vmovss	%xmm2, 80(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 48(%rsp)
	vxorps	%xmm15, %xmm15, %xmm15
	vmovss	%xmm2, 236(%rsp)
	vxorps	%xmm6, %xmm6, %xmm6
	vmovss	%xmm6, 180(%rsp)
	vmovss	%xmm0, 108(%rsp)
	vxorps	%xmm3, %xmm3, %xmm3
	vmovss	%xmm3, 44(%rsp)
.Ltmp3912:
	.loc	7 1714 9
	testq	%rax, %rax
.Ltmp3913:
	.loc	6 180 28
	je	.LBB25_55
.Ltmp3914:
	.loc	1 1331 0
	movq	128(%r13), %rcx
.Ltmp3915:
	.loc	1 815 29
	vmovss	(%rcx), %xmm15
	.loc	1 816 28
	vmovss	4(%rcx), %xmm12
	.loc	1 817 26
	vmovss	8(%rcx), %xmm10
	.loc	1 818 41
	movzwl	12(%rcx), %edx
.Ltmp3916:
	.loc	43 82 17
	vxorps	%xmm5, %xmm5, %xmm5
	vcvtsi2ss	%edx, %xmm5, %xmm4
.Ltmp3917:
	.loc	7 1714 9
	cmpq	$1, %rax
.Ltmp3918:
	.loc	6 180 28
	jne	.LBB25_45
	.loc	6 0 28 is_stmt 0
	vmovss	%xmm1, 148(%rsp)
	vmovss	%xmm1, 120(%rsp)
	vmovss	%xmm1, 88(%rsp)
	vmovss	%xmm1, 56(%rsp)
	vmovss	%xmm1, 212(%rsp)
	vmovss	%xmm1, 188(%rsp)
	vmovss	%xmm1, 144(%rsp)
	vmovss	%xmm1, 116(%rsp)
	vmovss	%xmm1, 84(%rsp)
	vmovss	%xmm1, 52(%rsp)
	vmovss	%xmm2, 208(%rsp)
	vmovss	%xmm2, 184(%rsp)
	vmovss	%xmm2, 140(%rsp)
	vmovss	%xmm2, 112(%rsp)
	vmovss	%xmm2, 80(%rsp)
	vmovss	%xmm0, 48(%rsp)
	vmovss	%xmm2, 236(%rsp)
	vmovss	%xmm6, 180(%rsp)
	vmovss	%xmm0, 108(%rsp)
	vmovss	%xmm3, 44(%rsp)
	.loc	6 180 28
	jmp	.LBB25_55
.Ltmp3919:
.LBB25_45:
	.loc	1 815 29 is_stmt 1
	vmovss	16(%rcx), %xmm11
	.loc	1 816 28
	vmovss	20(%rcx), %xmm1
	vmovss	%xmm1, 152(%rsp)
	.loc	1 817 26
	vmovss	24(%rcx), %xmm13
	.loc	1 818 41
	movzwl	28(%rcx), %edx
.Ltmp3920:
	.loc	43 82 17
	vxorps	%xmm5, %xmm5, %xmm5
	vcvtsi2ss	%edx, %xmm5, %xmm9
.Ltmp3921:
	.loc	7 1714 9
	cmpq	$2, %rax
	vmovss	%xmm11, 236(%rsp)
.Ltmp3922:
	.loc	6 180 28
	jne	.LBB25_47
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm11, %xmm11, %xmm11
	vxorps	%xmm1, %xmm1, %xmm1
	vmovss	%xmm1, 148(%rsp)
	vmovss	%xmm1, 120(%rsp)
	vmovss	%xmm1, 88(%rsp)
	vmovss	%xmm1, 56(%rsp)
	vmovss	%xmm1, 212(%rsp)
	vmovss	%xmm1, 188(%rsp)
	vmovss	%xmm1, 144(%rsp)
	vmovss	%xmm1, 116(%rsp)
	vmovss	%xmm1, 84(%rsp)
	vmovss	%xmm1, 52(%rsp)
	vmovss	%xmm1, 208(%rsp)
	vmovss	%xmm1, 184(%rsp)
	vmovss	%xmm1, 140(%rsp)
	vmovss	%xmm1, 112(%rsp)
	vmovss	%xmm1, 80(%rsp)
	vmovss	%xmm0, 48(%rsp)
	vmovss	%xmm1, 180(%rsp)
	vmovss	%xmm1, 108(%rsp)
	vmovss	%xmm1, 44(%rsp)
	vmovss	152(%rsp), %xmm1
	.loc	6 180 28
	jmp	.LBB25_55
.Ltmp3923:
.LBB25_47:
	.loc	1 815 29 is_stmt 1
	vmovss	32(%rcx), %xmm1
	vmovss	%xmm1, 76(%rsp)
	.loc	1 816 28
	vmovss	36(%rcx), %xmm2
	.loc	1 817 26
	vmovss	40(%rcx), %xmm1
	vmovss	%xmm1, 212(%rsp)
	.loc	1 818 41
	movzwl	44(%rcx), %edx
.Ltmp3924:
	.loc	43 82 17
	vxorps	%xmm5, %xmm5, %xmm5
	vcvtsi2ss	%edx, %xmm5, %xmm11
.Ltmp3925:
	.loc	7 1714 9
	cmpq	$3, %rax
	vmovss	%xmm2, 208(%rsp)
.Ltmp3926:
	.loc	6 180 28
	jne	.LBB25_49
.Ltmp3927:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vmovss	%xmm1, 148(%rsp)
	vmovss	%xmm1, 120(%rsp)
	vmovss	%xmm1, 88(%rsp)
	vmovss	%xmm1, 56(%rsp)
	vmovss	%xmm1, 188(%rsp)
	vmovss	%xmm1, 144(%rsp)
	vmovss	%xmm1, 116(%rsp)
	vmovss	%xmm1, 84(%rsp)
	vmovss	%xmm1, 52(%rsp)
	vmovss	%xmm1, 184(%rsp)
	vmovss	%xmm1, 140(%rsp)
	vmovss	%xmm1, 112(%rsp)
	vmovss	%xmm1, 80(%rsp)
	vmovss	%xmm0, 48(%rsp)
	vmovss	%xmm1, 180(%rsp)
	jmp	.LBB25_54
.LBB25_49:
.Ltmp3928:
	.loc	1 815 29 is_stmt 1
	vmovss	48(%rcx), %xmm1
	vmovss	%xmm1, 180(%rsp)
	.loc	1 816 28
	vmovss	52(%rcx), %xmm1
	vmovss	%xmm1, 184(%rsp)
	.loc	1 817 26
	vmovss	56(%rcx), %xmm1
	.loc	1 818 41
	movzwl	60(%rcx), %edx
.Ltmp3929:
	.loc	43 82 17
	vxorps	%xmm5, %xmm5, %xmm5
	vcvtsi2ss	%edx, %xmm5, %xmm14
.Ltmp3930:
	.loc	7 1714 9
	cmpq	$4, %rax
	vmovss	%xmm1, 188(%rsp)
.Ltmp3931:
	.loc	6 180 28
	jne	.LBB25_51
.Ltmp3932:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vmovss	%xmm1, 148(%rsp)
	vmovss	%xmm1, 120(%rsp)
	vmovss	%xmm1, 88(%rsp)
	vmovss	%xmm1, 56(%rsp)
	vmovss	%xmm1, 144(%rsp)
	vmovss	%xmm1, 116(%rsp)
	vmovss	%xmm1, 84(%rsp)
	vmovss	%xmm1, 52(%rsp)
	vmovss	%xmm1, 140(%rsp)
	jmp	.LBB25_53
.LBB25_51:
.Ltmp3933:
	.loc	1 815 29 is_stmt 1
	vmovss	64(%rcx), %xmm6
	.loc	1 816 28
	vmovss	68(%rcx), %xmm1
	vmovss	%xmm1, 140(%rsp)
	.loc	1 817 26
	vmovss	72(%rcx), %xmm1
	vmovss	%xmm1, 144(%rsp)
	.loc	1 818 41
	movzwl	76(%rcx), %edx
.Ltmp3934:
	.loc	43 82 17
	vxorps	%xmm5, %xmm5, %xmm5
	vcvtsi2ss	%edx, %xmm5, %xmm3
.Ltmp3935:
	.loc	7 1714 9
	cmpq	$5, %rax
	vmovss	%xmm3, 148(%rsp)
.Ltmp3936:
	.loc	6 180 28
	jne	.LBB25_75
.Ltmp3937:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vmovss	%xmm1, 120(%rsp)
	vmovss	%xmm1, 88(%rsp)
	vmovss	%xmm1, 56(%rsp)
	vmovss	%xmm1, 116(%rsp)
	vmovss	%xmm1, 84(%rsp)
	vmovss	%xmm1, 52(%rsp)
.LBB25_53:
	vmovss	%xmm1, 112(%rsp)
	vmovss	%xmm1, 80(%rsp)
	vmovss	%xmm0, 48(%rsp)
.LBB25_54:
	vmovss	%xmm1, 108(%rsp)
	vmovss	%xmm1, 44(%rsp)
	vmovss	152(%rsp), %xmm1
	vmovss	76(%rsp), %xmm2
.LBB25_55:
	.loc	1 1332 40 is_stmt 1
	movq	152(%r13), %rcx
.Ltmp3938:
	.loc	7 1714 9
	testq	%rcx, %rcx
	vmovss	%xmm4, 284(%rsp)
	vmovss	%xmm9, 280(%rsp)
	vmovss	%xmm10, 276(%rsp)
	vmovss	%xmm11, 272(%rsp)
	vmovss	%xmm12, 268(%rsp)
	vmovss	%xmm13, 264(%rsp)
	vmovss	%xmm14, 260(%rsp)
	vmovss	%xmm15, 256(%rsp)
	vmovss	%xmm1, 152(%rsp)
	vmovss	%xmm2, 76(%rsp)
	vmovss	%xmm6, 176(%rsp)
	vmovss	%xmm0, 232(%rsp)
.Ltmp3939:
	.loc	6 180 28
	je	.LBB25_58
.Ltmp3940:
	.loc	1 1332 0
	movq	144(%r13), %rax
.Ltmp3941:
	.loc	1 815 29
	vmovss	(%rax), %xmm3
	vmovss	%xmm3, 252(%rsp)
	.loc	1 816 28
	vmovss	4(%rax), %xmm8
	.loc	1 817 26
	vmovss	8(%rax), %xmm7
	.loc	1 818 41
	movzwl	12(%rax), %edx
.Ltmp3942:
	.loc	43 82 17
	vxorps	%xmm5, %xmm5, %xmm5
	vcvtsi2ss	%edx, %xmm5, %xmm5
.Ltmp3943:
	.loc	7 1714 9
	cmpq	$1, %rcx
	vmovss	%xmm5, 248(%rsp)
	vmovss	%xmm7, 244(%rsp)
	vmovss	%xmm8, 240(%rsp)
.Ltmp3944:
	.loc	6 180 28
	jne	.LBB25_67
.Ltmp3945:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	%xmm0, 228(%rsp)
	vmovss	%xmm0, 192(%rsp)
	vmovss	%xmm0, 172(%rsp)
	vmovss	%xmm0, 124(%rsp)
	vmovss	%xmm0, 104(%rsp)
	vmovss	%xmm0, 68(%rsp)
	vmovss	%xmm0, 224(%rsp)
	vmovss	%xmm0, 204(%rsp)
	vmovss	%xmm0, 168(%rsp)
	vmovss	%xmm0, 136(%rsp)
	vmovss	%xmm0, 100(%rsp)
	vmovss	%xmm0, 72(%rsp)
	vmovss	%xmm0, 36(%rsp)
	vmovss	%xmm0, 220(%rsp)
	vmovss	%xmm0, 200(%rsp)
	vmovss	%xmm0, 164(%rsp)
	vmovss	%xmm0, 132(%rsp)
	vmovss	%xmm0, 96(%rsp)
	vmovss	%xmm0, 64(%rsp)
	vmovss	%xmm0, 32(%rsp)
	jmp	.LBB25_59
.LBB25_58:
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	%xmm0, 248(%rsp)
	vmovss	%xmm0, 228(%rsp)
	vmovss	%xmm0, 192(%rsp)
	vmovss	%xmm0, 172(%rsp)
	vmovss	%xmm0, 124(%rsp)
	vmovss	%xmm0, 104(%rsp)
	vmovss	%xmm0, 68(%rsp)
	vmovss	%xmm0, 244(%rsp)
	vmovss	%xmm0, 224(%rsp)
	vmovss	%xmm0, 204(%rsp)
	vmovss	%xmm0, 168(%rsp)
	vmovss	%xmm0, 136(%rsp)
	vmovss	%xmm0, 100(%rsp)
	vmovss	%xmm0, 72(%rsp)
	vmovss	%xmm0, 36(%rsp)
	vmovss	%xmm0, 240(%rsp)
	vmovss	%xmm0, 220(%rsp)
	vmovss	%xmm0, 200(%rsp)
	vmovss	%xmm0, 164(%rsp)
	vmovss	%xmm0, 132(%rsp)
	vmovss	%xmm0, 96(%rsp)
	vmovss	%xmm0, 64(%rsp)
	vmovss	%xmm0, 32(%rsp)
	vmovss	%xmm0, 252(%rsp)
.LBB25_59:
	vmovss	%xmm0, 216(%rsp)
.LBB25_60:
	vmovss	%xmm0, 196(%rsp)
.LBB25_61:
	vmovss	%xmm0, 160(%rsp)
.LBB25_62:
	vmovss	%xmm0, 128(%rsp)
.LBB25_63:
	vmovss	%xmm0, 92(%rsp)
.LBB25_64:
	vmovss	%xmm0, 60(%rsp)
.LBB25_65:
	vmovss	%xmm0, 28(%rsp)
.LBB25_66:
	leaq	384(%rsp), %rsi
	.loc	1 1326 9 is_stmt 1
	movl	$384, %edx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	vmovaps	320(%rsp), %ymm0
	vmovaps	%ymm0, 640(%rbx)
	vmovaps	352(%rsp), %ymm0
	vmovaps	%ymm0, 672(%rbx)
	vmovss	312(%rsp), %xmm0
	vmovss	%xmm0, 704(%rbx)
	vmovss	308(%rsp), %xmm0
	vmovss	%xmm0, 708(%rbx)
	vmovss	304(%rsp), %xmm0
	vmovss	%xmm0, 712(%rbx)
	vmovss	300(%rsp), %xmm0
	vmovss	%xmm0, 716(%rbx)
	vmovss	296(%rsp), %xmm0
	vmovss	%xmm0, 720(%rbx)
	vmovss	292(%rsp), %xmm0
	vmovss	%xmm0, 724(%rbx)
	vmovss	288(%rsp), %xmm0
	vmovss	%xmm0, 728(%rbx)
	vmovss	316(%rsp), %xmm0
	vmovss	%xmm0, 732(%rbx)
	vmovss	256(%rsp), %xmm0
	vmovss	%xmm0, 384(%rbx)
	vmovss	236(%rsp), %xmm0
	vmovss	%xmm0, 388(%rbx)
	vmovss	76(%rsp), %xmm0
	vmovss	%xmm0, 392(%rbx)
	vmovss	180(%rsp), %xmm0
	vmovss	%xmm0, 396(%rbx)
	vmovss	176(%rsp), %xmm0
	vmovss	%xmm0, 400(%rbx)
	vmovss	108(%rsp), %xmm0
	vmovss	%xmm0, 404(%rbx)
	vmovss	232(%rsp), %xmm0
	vmovss	%xmm0, 408(%rbx)
	vmovss	44(%rsp), %xmm0
	vmovss	%xmm0, 412(%rbx)
	vmovss	268(%rsp), %xmm0
	vmovss	%xmm0, 416(%rbx)
	vmovss	152(%rsp), %xmm0
	vmovss	%xmm0, 420(%rbx)
	vmovss	208(%rsp), %xmm0
	vmovss	%xmm0, 424(%rbx)
	vmovss	184(%rsp), %xmm0
	vmovss	%xmm0, 428(%rbx)
	vmovss	140(%rsp), %xmm0
	vmovss	%xmm0, 432(%rbx)
	vmovss	112(%rsp), %xmm0
	vmovss	%xmm0, 436(%rbx)
	vmovss	80(%rsp), %xmm0
	vmovss	%xmm0, 440(%rbx)
	vmovss	48(%rsp), %xmm0
	vmovss	%xmm0, 444(%rbx)
	vmovss	276(%rsp), %xmm0
	vmovss	%xmm0, 448(%rbx)
	vmovss	264(%rsp), %xmm0
	vmovss	%xmm0, 452(%rbx)
	vmovss	212(%rsp), %xmm0
	vmovss	%xmm0, 456(%rbx)
	vmovss	188(%rsp), %xmm0
	vmovss	%xmm0, 460(%rbx)
	vmovss	144(%rsp), %xmm0
	vmovss	%xmm0, 464(%rbx)
	vmovss	116(%rsp), %xmm0
	vmovss	%xmm0, 468(%rbx)
	vmovss	84(%rsp), %xmm0
	vmovss	%xmm0, 472(%rbx)
	vmovss	52(%rsp), %xmm0
	vmovss	%xmm0, 476(%rbx)
	vmovss	284(%rsp), %xmm0
	vmovss	%xmm0, 480(%rbx)
	vmovss	280(%rsp), %xmm0
	vmovss	%xmm0, 484(%rbx)
	vmovss	272(%rsp), %xmm0
	vmovss	%xmm0, 488(%rbx)
	vmovss	260(%rsp), %xmm0
	vmovss	%xmm0, 492(%rbx)
	vmovss	148(%rsp), %xmm0
	vmovss	%xmm0, 496(%rbx)
	vmovss	120(%rsp), %xmm0
	vmovss	%xmm0, 500(%rbx)
	vmovss	88(%rsp), %xmm0
	vmovss	%xmm0, 504(%rbx)
	vmovss	56(%rsp), %xmm0
	vmovss	%xmm0, 508(%rbx)
	vmovss	252(%rsp), %xmm0
	vmovss	%xmm0, 512(%rbx)
	vmovss	216(%rsp), %xmm0
	vmovss	%xmm0, 516(%rbx)
	vmovss	196(%rsp), %xmm0
	vmovss	%xmm0, 520(%rbx)
	vmovss	160(%rsp), %xmm0
	vmovss	%xmm0, 524(%rbx)
	vmovss	128(%rsp), %xmm0
	vmovss	%xmm0, 528(%rbx)
	vmovss	92(%rsp), %xmm0
	vmovss	%xmm0, 532(%rbx)
	vmovss	60(%rsp), %xmm0
	vmovss	%xmm0, 536(%rbx)
	vmovss	28(%rsp), %xmm0
	vmovss	%xmm0, 540(%rbx)
	vmovss	240(%rsp), %xmm0
	vmovss	%xmm0, 544(%rbx)
	vmovss	220(%rsp), %xmm0
	vmovss	%xmm0, 548(%rbx)
	vmovss	200(%rsp), %xmm0
	vmovss	%xmm0, 552(%rbx)
	vmovss	164(%rsp), %xmm0
	vmovss	%xmm0, 556(%rbx)
	vmovss	132(%rsp), %xmm0
	vmovss	%xmm0, 560(%rbx)
	vmovss	96(%rsp), %xmm0
	vmovss	%xmm0, 564(%rbx)
	vmovss	64(%rsp), %xmm0
	vmovss	%xmm0, 568(%rbx)
	vmovss	32(%rsp), %xmm0
	vmovss	%xmm0, 572(%rbx)
	vmovss	244(%rsp), %xmm0
	vmovss	%xmm0, 576(%rbx)
	vmovss	224(%rsp), %xmm0
	vmovss	%xmm0, 580(%rbx)
	vmovss	204(%rsp), %xmm0
	vmovss	%xmm0, 584(%rbx)
	vmovss	168(%rsp), %xmm0
	vmovss	%xmm0, 588(%rbx)
	vmovss	136(%rsp), %xmm0
	vmovss	%xmm0, 592(%rbx)
	vmovss	100(%rsp), %xmm0
	vmovss	%xmm0, 596(%rbx)
	vmovss	72(%rsp), %xmm0
	vmovss	%xmm0, 600(%rbx)
	vmovss	36(%rsp), %xmm0
	vmovss	%xmm0, 604(%rbx)
	vmovss	248(%rsp), %xmm0
	vmovss	%xmm0, 608(%rbx)
	vmovss	228(%rsp), %xmm0
	vmovss	%xmm0, 612(%rbx)
	vmovss	192(%rsp), %xmm0
	vmovss	%xmm0, 616(%rbx)
	vmovss	172(%rsp), %xmm0
	vmovss	%xmm0, 620(%rbx)
	vmovss	124(%rsp), %xmm0
	vmovss	%xmm0, 624(%rbx)
	vmovss	104(%rsp), %xmm0
	vmovss	%xmm0, 628(%rbx)
	vmovss	68(%rsp), %xmm0
	vmovss	%xmm0, 632(%rbx)
	vmovss	40(%rsp), %xmm0
	vmovss	%xmm0, 636(%rbx)
.Ltmp3946:
	.loc	1 1334 6
	leaq	-40(%rbp), %rsp
	.loc	1 1334 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	vzeroupper
	retq
.LBB25_67:
	.cfi_def_cfa %rbp, 16
.Ltmp3947:
	.loc	1 815 29 is_stmt 1
	vmovss	16(%rax), %xmm3
	vmovss	%xmm3, 216(%rsp)
	.loc	1 816 28
	vmovss	20(%rax), %xmm3
	vmovss	%xmm3, 220(%rsp)
	.loc	1 817 26
	vmovss	24(%rax), %xmm3
	vmovss	%xmm3, 224(%rsp)
	.loc	1 818 41
	movzwl	28(%rax), %edx
.Ltmp3948:
	.loc	43 82 17
	vxorps	%xmm9, %xmm9, %xmm9
	vcvtsi2ss	%edx, %xmm9, %xmm3
	vmovss	%xmm3, 228(%rsp)
.Ltmp3949:
	.loc	7 1714 9
	cmpq	$2, %rcx
.Ltmp3950:
	.loc	6 180 28
	jne	.LBB25_69
.Ltmp3951:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	%xmm0, 192(%rsp)
	vmovss	%xmm0, 172(%rsp)
	vmovss	%xmm0, 124(%rsp)
	vmovss	%xmm0, 104(%rsp)
	vmovss	%xmm0, 68(%rsp)
	vmovss	%xmm0, 204(%rsp)
	vmovss	%xmm0, 168(%rsp)
	vmovss	%xmm0, 136(%rsp)
	vmovss	%xmm0, 100(%rsp)
	vmovss	%xmm0, 72(%rsp)
	vmovss	%xmm0, 36(%rsp)
	vmovss	%xmm0, 200(%rsp)
	vmovss	%xmm0, 164(%rsp)
	vmovss	%xmm0, 132(%rsp)
	vmovss	%xmm0, 96(%rsp)
	vmovss	%xmm0, 64(%rsp)
	vmovss	%xmm0, 32(%rsp)
	jmp	.LBB25_60
.LBB25_69:
.Ltmp3952:
	.loc	1 815 29 is_stmt 1
	vmovss	32(%rax), %xmm3
	vmovss	%xmm3, 196(%rsp)
	.loc	1 816 28
	vmovss	36(%rax), %xmm3
	vmovss	%xmm3, 200(%rsp)
	.loc	1 817 26
	vmovss	40(%rax), %xmm3
	vmovss	%xmm3, 204(%rsp)
	.loc	1 818 41
	movzwl	44(%rax), %edx
.Ltmp3953:
	.loc	43 82 17
	vxorps	%xmm9, %xmm9, %xmm9
	vcvtsi2ss	%edx, %xmm9, %xmm3
.Ltmp3954:
	.loc	7 1714 9
	cmpq	$3, %rcx
	vmovss	%xmm3, 192(%rsp)
.Ltmp3955:
	.loc	6 180 28
	jne	.LBB25_71
.Ltmp3956:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	%xmm0, 172(%rsp)
	vmovss	%xmm0, 124(%rsp)
	vmovss	%xmm0, 104(%rsp)
	vmovss	%xmm0, 68(%rsp)
	vmovss	%xmm0, 168(%rsp)
	vmovss	%xmm0, 136(%rsp)
	vmovss	%xmm0, 100(%rsp)
	vmovss	%xmm0, 72(%rsp)
	vmovss	%xmm0, 36(%rsp)
	vmovss	%xmm0, 164(%rsp)
	vmovss	%xmm0, 132(%rsp)
	vmovss	%xmm0, 96(%rsp)
	vmovss	%xmm0, 64(%rsp)
	vmovss	%xmm0, 32(%rsp)
	jmp	.LBB25_61
.LBB25_71:
.Ltmp3957:
	.loc	1 815 29 is_stmt 1
	vmovss	48(%rax), %xmm3
	vmovss	%xmm3, 160(%rsp)
	.loc	1 816 28
	vmovss	52(%rax), %xmm3
	vmovss	%xmm3, 164(%rsp)
	.loc	1 817 26
	vmovss	56(%rax), %xmm3
	vmovss	%xmm3, 168(%rsp)
	.loc	1 818 41
	movzwl	60(%rax), %edx
.Ltmp3958:
	.loc	43 82 17
	vxorps	%xmm9, %xmm9, %xmm9
	vcvtsi2ss	%edx, %xmm9, %xmm3
	vmovss	%xmm3, 172(%rsp)
.Ltmp3959:
	.loc	7 1714 9
	cmpq	$4, %rcx
.Ltmp3960:
	.loc	6 180 28
	jne	.LBB25_73
.Ltmp3961:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	%xmm0, 124(%rsp)
	vmovss	%xmm0, 104(%rsp)
	vmovss	%xmm0, 68(%rsp)
	vmovss	%xmm0, 136(%rsp)
	vmovss	%xmm0, 100(%rsp)
	vmovss	%xmm0, 72(%rsp)
	vmovss	%xmm0, 36(%rsp)
	vmovss	%xmm0, 132(%rsp)
	vmovss	%xmm0, 96(%rsp)
	vmovss	%xmm0, 64(%rsp)
	vmovss	%xmm0, 32(%rsp)
	jmp	.LBB25_62
.LBB25_73:
.Ltmp3962:
	.loc	1 815 29 is_stmt 1
	vmovss	64(%rax), %xmm3
	vmovss	%xmm3, 128(%rsp)
	.loc	1 816 28
	vmovss	68(%rax), %xmm3
	vmovss	%xmm3, 132(%rsp)
	.loc	1 817 26
	vmovss	72(%rax), %xmm3
	vmovss	%xmm3, 136(%rsp)
	.loc	1 818 41
	movzwl	76(%rax), %edx
.Ltmp3963:
	.loc	43 82 17
	vxorps	%xmm9, %xmm9, %xmm9
	vcvtsi2ss	%edx, %xmm9, %xmm3
.Ltmp3964:
	.loc	7 1714 9
	cmpq	$5, %rcx
	vmovss	%xmm3, 124(%rsp)
.Ltmp3965:
	.loc	6 180 28
	jne	.LBB25_77
.Ltmp3966:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	%xmm0, 104(%rsp)
	vmovss	%xmm0, 68(%rsp)
	vmovss	%xmm0, 100(%rsp)
	vmovss	%xmm0, 72(%rsp)
	vmovss	%xmm0, 36(%rsp)
	vmovss	%xmm0, 96(%rsp)
	vmovss	%xmm0, 64(%rsp)
	vmovss	%xmm0, 32(%rsp)
	jmp	.LBB25_63
.LBB25_75:
	vmovss	%xmm6, 176(%rsp)
.Ltmp3967:
	.loc	1 815 29 is_stmt 1
	vmovss	80(%rcx), %xmm0
	vmovss	%xmm0, 108(%rsp)
	.loc	1 816 28
	vmovss	84(%rcx), %xmm6
	.loc	1 817 26
	vmovss	88(%rcx), %xmm1
	vmovss	%xmm1, 116(%rsp)
	.loc	1 818 41
	movzwl	92(%rcx), %edx
.Ltmp3968:
	.loc	43 82 17
	vxorps	%xmm5, %xmm5, %xmm5
	vcvtsi2ss	%edx, %xmm5, %xmm1
	vmovss	%xmm1, 120(%rsp)
.Ltmp3969:
	.loc	7 1714 9
	cmpq	$6, %rax
	vmovss	%xmm6, 112(%rsp)
.Ltmp3970:
	.loc	6 180 28
	jne	.LBB25_79
.Ltmp3971:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vmovss	%xmm1, 88(%rsp)
	vmovss	%xmm1, 56(%rsp)
	vmovss	%xmm1, 84(%rsp)
	vmovss	%xmm1, 52(%rsp)
	vmovss	%xmm1, 80(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 48(%rsp)
	jmp	.LBB25_81
.LBB25_77:
.Ltmp3972:
	.loc	1 815 29 is_stmt 1
	vmovss	80(%rax), %xmm3
	vmovss	%xmm3, 92(%rsp)
	.loc	1 816 28
	vmovss	84(%rax), %xmm3
	vmovss	%xmm3, 96(%rsp)
	.loc	1 817 26
	vmovss	88(%rax), %xmm3
	vmovss	%xmm3, 100(%rsp)
	.loc	1 818 41
	movzwl	92(%rax), %edx
.Ltmp3973:
	.loc	43 82 17
	vxorps	%xmm9, %xmm9, %xmm9
	vcvtsi2ss	%edx, %xmm9, %xmm3
	vmovss	%xmm3, 104(%rsp)
.Ltmp3974:
	.loc	7 1714 9
	cmpq	$6, %rcx
.Ltmp3975:
	.loc	6 180 28
	jne	.LBB25_82
.Ltmp3976:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	%xmm0, 68(%rsp)
	vmovss	%xmm0, 72(%rsp)
	vmovss	%xmm0, 36(%rsp)
	vmovss	%xmm0, 64(%rsp)
	vmovss	%xmm0, 32(%rsp)
	jmp	.LBB25_64
.LBB25_79:
.Ltmp3977:
	.loc	1 815 29 is_stmt 1
	vmovss	96(%rcx), %xmm0
	.loc	1 816 28
	vmovss	100(%rcx), %xmm1
	vmovss	%xmm1, 80(%rsp)
	.loc	1 817 26
	vmovss	104(%rcx), %xmm1
	vmovss	%xmm1, 84(%rsp)
	.loc	1 818 41
	movzwl	108(%rcx), %edx
.Ltmp3978:
	.loc	43 82 17
	vxorps	%xmm5, %xmm5, %xmm5
	vcvtsi2ss	%edx, %xmm5, %xmm1
	vmovss	%xmm1, 88(%rsp)
.Ltmp3979:
	.loc	7 1714 9
	cmpq	$7, %rax
.Ltmp3980:
	.loc	6 180 28
	jne	.LBB25_84
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm1, %xmm1, %xmm1
	vmovss	%xmm1, 56(%rsp)
	vmovss	%xmm1, 52(%rsp)
	vmovaps	%xmm0, %xmm1
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovaps	%xmm1, %xmm0
	vxorps	%xmm1, %xmm1, %xmm1
.LBB25_81:
	vmovss	%xmm1, 44(%rsp)
	vmovss	152(%rsp), %xmm1
	vmovss	76(%rsp), %xmm2
	vmovss	176(%rsp), %xmm6
	.loc	6 180 28 is_stmt 1
	jmp	.LBB25_55
.Ltmp3981:
.LBB25_82:
	.loc	1 815 29
	vmovss	96(%rax), %xmm3
	vmovss	%xmm3, 60(%rsp)
	.loc	1 816 28
	vmovss	100(%rax), %xmm3
	vmovss	%xmm3, 64(%rsp)
	.loc	1 817 26
	vmovss	104(%rax), %xmm3
	vmovss	%xmm3, 72(%rsp)
	.loc	1 818 41
	movzwl	108(%rax), %edx
.Ltmp3982:
	.loc	43 82 17
	vxorps	%xmm9, %xmm9, %xmm9
	vcvtsi2ss	%edx, %xmm9, %xmm3
.Ltmp3983:
	.loc	7 1714 9
	cmpq	$7, %rcx
	vmovss	%xmm3, 68(%rsp)
.Ltmp3984:
	.loc	6 180 28
	jne	.LBB25_86
.Ltmp3985:
	.loc	6 0 28 is_stmt 0
	vxorps	%xmm0, %xmm0, %xmm0
	vmovss	%xmm0, 40(%rsp)
	vmovss	%xmm0, 36(%rsp)
	vmovss	%xmm0, 32(%rsp)
	jmp	.LBB25_65
.LBB25_84:
	vmovss	%xmm0, 232(%rsp)
.Ltmp3986:
	.loc	7 1714 9 is_stmt 1
	cmpq	$8, %rax
.Ltmp3987:
	.loc	6 180 28
	jne	.LBB25_108
.Ltmp3988:
	.loc	1 0 0 is_stmt 0
	vmovss	112(%rcx), %xmm0
	vmovss	%xmm0, 44(%rsp)
	vmovss	116(%rcx), %xmm0
	vmovss	%xmm0, 48(%rsp)
	vmovss	120(%rcx), %xmm1
	vmovss	%xmm1, 52(%rsp)
	movzwl	124(%rcx), %eax
	vcvtsi2ss	%eax, %xmm8, %xmm1
	vmovss	%xmm1, 56(%rsp)
	vmovss	152(%rsp), %xmm1
	vmovss	76(%rsp), %xmm2
	vmovss	176(%rsp), %xmm6
	vmovss	232(%rsp), %xmm0
	jmp	.LBB25_55
.Ltmp3989:
.LBB25_86:
	.loc	7 1714 9 is_stmt 1
	cmpq	$8, %rcx
.Ltmp3990:
	.loc	6 180 28
	jne	.LBB25_108
.Ltmp3991:
	.loc	1 0 0 is_stmt 0
	vmovss	112(%rax), %xmm0
	vmovss	%xmm0, 28(%rsp)
	vmovss	116(%rax), %xmm0
	vmovss	%xmm0, 32(%rsp)
	vmovss	120(%rax), %xmm0
	vmovss	%xmm0, 36(%rsp)
	movzwl	124(%rax), %eax
	vcvtsi2ss	%eax, %xmm9, %xmm0
	vmovss	%xmm0, 40(%rsp)
	jmp	.LBB25_66
.Ltmp3992:
.LBB25_88:
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB25_89:
.Ltmp3993:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB25_90:
	.loc	5 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r15, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB25_91:
	.loc	5 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3994:
.LBB25_92:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3995:
.LBB25_93:
	.loc	5 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%rax, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3996:
.LBB25_94:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3997:
.LBB25_95:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rax
	movq	%rcx, %rdi
	movq	%rdx, %rsi
	movq	%rax, %rcx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3998:
.LBB25_96:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r8, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp3999:
.LBB25_97:
	.loc	5 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r9, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4000:
.LBB25_98:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r9, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4001:
.LBB25_99:
	.loc	5 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r10, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4002:
.LBB25_100:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r10, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4003:
.LBB25_101:
	.loc	5 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r11, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4004:
.LBB25_102:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r14, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4005:
.LBB25_103:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r11, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4006:
.LBB25_104:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r15, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4007:
.LBB25_105:
	.loc	5 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r12, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4008:
.LBB25_106:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%r12, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4009:
.LBB25_107:
	.loc	5 569 13
	leaq	.Lalloc_7f43518a494eccc88ee423e0d076b927(%rip), %rcx
	movq	%rsi, %rdi
	movq	%rdx, %rsi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4010:
.LBB25_108:
	.loc	1 815 13
	leaq	.Lalloc_34ec9deef3d1165c325897b5845d9fe1(%rip), %rdx
.Ltmp4011:
	.loc	1 0 0 is_stmt 0
	movl	$8, %edi
	movl	$8, %esi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB25_109:
.Ltmp4012:
	.loc	1 1324 13 is_stmt 1
	leaq	.Lalloc_3862c97910d0689d8b59a4255cb98beb(%rip), %rdx
.Ltmp4013:
	.loc	1 0 0 is_stmt 0
	movl	$8, %edi
	movl	$8, %esi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4014:
.Lfunc_end25:
	.size	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_, .Lfunc_end25-_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
