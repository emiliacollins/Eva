	BITS 16
	ORG 0x7C00

	nop

	times 218 - ($ - $$) db 0
