	BITS 16
	ORG 0x7c00


	mov si, msg
	call PrintString
	jmp $













;;; String address in si
PrintString:
	push ax
	push si
.printloop:
	mov al, [si]
	inc si
	test al, al
	jz .done
	call PrintChar
	jmp .printloop
.done:
	pop si
	pop ax
	ret

	
;; Char in al
PrintChar:
	push ax
	push bx
	
	mov ah, 0x0E
	mov bh, 0
	int 0x10

	pop bx
	pop ax
	
	ret
msg:	db "IN VBR!",0
