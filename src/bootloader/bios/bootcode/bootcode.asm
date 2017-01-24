	BITS 16
	ORG 0x7C00
	
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
Constants:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
	
	%define NEW_OFFSET 0x0600
	%define OLD_OFFSET 0x7C00
	%define MBR_SIZE 512
	%define PARTITION_TABLE_START 446
	%define PARTITION_ENTRY_SIZE 16
	%define ENTRY_COUNT 4
	%define BOOTABLE_BIT_MASK 0x80
	%define MBR_SEGMENT 0x07C0
	%define STACK_SEGMENTS 	288	; (4096 + 512) / 16
	%define STACK_SIZE 4096
	
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
PreCopy:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

DisableInterrupts:
	cli
	
SetupStack:			; Initialize a 4k stack
	mov ax, MBR_SEGMENT + STACK_SEGMENTS
	mov ss, ax
	mov sp, STACK_SIZE

InitSegRegs:			; Initialize all segment registers to zero
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	
RelocateMBR:	
	xor ax, ax		; Zero ax for segment register zeroing

	mov es, ax		; Set copy location to 0:0x0600
	mov di, NEW_OFFSET

	mov ds, ax		; Set origin location to 0:0x7C00
	mov si, OLD_OFFSET

	mov cx, MBR_SIZE	; Copy all 512 bytes of MBR

	rep movsb		; Copy MBR one byte at a time
	
	jmp 0:PostCopy		; Jump to copied MBR after copy code

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
PostCopy:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;; 

ReenableInterrupts:
	sti

ReadPartitionTable:	
	mov bx, PARTITION_TABLE_START
	mov cx, ENTRY_COUNT
	
.bootableScan:
	mov ax, [bx]
	test ax, BOOTABLE_BIT_MASK
	jnz LoadPartition
	add bx, PARTITION_ENTRY_SIZE
	loop .bootableScan
	
NoBootFound:
	int 0x18		; Report not bootable to bios and return control
	
LoadPartition:
	jmp $
	
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
	
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

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
Data:
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
	
times 446 - ($ - $$) db 0
