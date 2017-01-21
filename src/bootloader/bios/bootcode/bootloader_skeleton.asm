	BITS 16
	ORG 0x7C00

	nop

	times 510 - ($ - $$) db 0
	db 0xAA
	db 0x55
