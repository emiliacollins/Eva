BITS 64
	
global long_mode_start

section .text
long_mode_start:
	mov rax, 0x2f592f412f4b2f4f 	; OKAY in white on green
	mov qword [0xb8000], rax
	hlt
