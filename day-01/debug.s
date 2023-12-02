	.text
	.file	"<stdin>"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:
	xorl	%eax, %eax
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	STDIN,@object                   # @STDIN
	.section	.rodata,"a",@progbits
	.globl	STDIN
	.p2align	2, 0x0
STDIN:
	.long	0                               # 0x0
	.size	STDIN, 4

	.type	STDOUT,@object                  # @STDOUT
	.globl	STDOUT
	.p2align	2, 0x0
STDOUT:
	.long	1                               # 0x1
	.size	STDOUT, 4

	.type	STDERR,@object                  # @STDERR
	.globl	STDERR
	.p2align	2, 0x0
STDERR:
	.long	2                               # 0x2
	.size	STDERR, 4

	.type	SEEK_SET,@object                # @SEEK_SET
	.globl	SEEK_SET
	.p2align	2, 0x0
SEEK_SET:
	.long	0                               # 0x0
	.size	SEEK_SET, 4

	.type	SEEK_CUR,@object                # @SEEK_CUR
	.globl	SEEK_CUR
	.p2align	2, 0x0
SEEK_CUR:
	.long	1                               # 0x1
	.size	SEEK_CUR, 4

	.type	SEEK_END,@object                # @SEEK_END
	.globl	SEEK_END
	.p2align	2, 0x0
SEEK_END:
	.long	2                               # 0x2
	.size	SEEK_END, 4

	.type	O_RDONLY,@object                # @O_RDONLY
	.globl	O_RDONLY
	.p2align	2, 0x0
O_RDONLY:
	.long	0                               # 0x0
	.size	O_RDONLY, 4

	.type	O_WRONLY,@object                # @O_WRONLY
	.globl	O_WRONLY
	.p2align	2, 0x0
O_WRONLY:
	.long	1                               # 0x1
	.size	O_WRONLY, 4

	.type	O_RDWR,@object                  # @O_RDWR
	.globl	O_RDWR
	.p2align	2, 0x0
O_RDWR:
	.long	2                               # 0x2
	.size	O_RDWR, 4

	.type	O_CREAT,@object                 # @O_CREAT
	.globl	O_CREAT
	.p2align	2, 0x0
O_CREAT:
	.long	64                              # 0x40
	.size	O_CREAT, 4

	.type	O_TRUNC,@object                 # @O_TRUNC
	.globl	O_TRUNC
	.p2align	2, 0x0
O_TRUNC:
	.long	512                             # 0x200
	.size	O_TRUNC, 4

	.section	".note.GNU-stack","",@progbits
