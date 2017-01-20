	BITS 16
	ORG 0x7C00

	mov si, Hello_World
	call PrintString
	jmp Infiniloop

PrintCharacter: 		; Assume character to print in al
	push ax
	push bx

	mov ah, 0x0E
	mov bl, 0x07
	mov bh, 0x00

	int 0x10

	pop bx
	pop ax
	ret
	
PrintString:			; Assume addr of string is in si 
	push ax
	push si
	
	.printloop:
		mov al, [si]
		or al, al
		jz .done
		call PrintCharacter
		inc si
		jmp .printloop
	.done:
		pop si
		pop cx
		pop ax

		ret

Infiniloop:
	jmp $
	
	
	Hello_World db "Hello, World!",0x0a, 0x0d,0

	
	times 510-($-$$) db 0       
	dw 0xAA55
