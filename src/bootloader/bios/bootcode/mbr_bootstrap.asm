	BITS 16
	ORG 0x0600		; Will be loaded into 7c00, but use 600 to resolve jumps
	
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
Constants:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
	
	%define OLD_OFFSET 0x7C00
	%define NEW_OFFSET 0x0600

	%define MBR_SIZE 512

	%define PARTITION_TABLE_OFFSET 446

	%define PARTITION_ENTRY_SIZE 16

	%define ENTRY_COUNT 4
	%define ENTRY_CYLINDER_OFFSET 3
	%define ENTRY_HEAD_OFFSET 1
	%define ENTRY_SECTOR_OFFSET 2
	%define LBA_OFFSET 8

	%define BOOTABLE_BIT_MASK 0x80

	%define DAP_LOC 0x800
	%define DAP_SIZE 0x10

	%define BOOT_SIGNATURE 0x55AA
	%define BOOT_SIGNATURE_OFFSET 0x1FE
	
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
PreCopy:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

DisableInterrupts:
	cli
	
SetupStack:
	xor ax, ax
	mov ss, ax		; Stack segment SS:SP is 0
	mov sp, OLD_OFFSET	; bottom of stack (grows downwards) starts just below code

InitSegRegs:			; Initialize all segment registers to zero
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	
RelocateMBR:	
	mov es, ax		; Set copy location to 0:0x0600
	mov di, NEW_OFFSET

	mov ds, ax		; Set origin location to 0:0x7C00
	mov si, OLD_OFFSET

	mov cx, MBR_SIZE	; Copy all 512 bytes of MBR

	cld			; Clear directional flag (want to increment)
	rep movsb		; Copy MBR one byte at a time

	jmp 0:PostCopy		; Jump to copied MBR after copy code (need 0 segment to indicate absolute not relative)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
PostCopy:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
	
ReenableInterrupts:
	sti

ReadPartitionTable:
	mov [bootDrive], dl				; save boot drive from bios
	mov bx, NEW_OFFSET + PARTITION_TABLE_OFFSET	; mov table addr into bx
	mov cx, ENTRY_COUNT				; 4 partitions to test
	
.bootableScan:
	mov al, [bx]					; get status byte
	test al, BOOTABLE_BIT_MASK			; test if its bootable
	jz .continue					; bootable -> load vbr
	mov ax, BOOT_SIGNATURE
	mov dx, [NEW_OFFSET+BOOT_SIGNATURE_OFFSET]
	cmp ax, dx
	je LoadPartition
.continue:
	add bx, PARTITION_ENTRY_SIZE			; go to next partition
	loop .bootableScan
	
NoBootFound:
	int 0x18					; Report not bootable to bios and return control
	
LoadPartition:
	mov [bootPartition], bx
.checkForLBAExtension:
	mov ah, 0x41
	mov bx, 0x55AA
	mov dl, [bootDrive]
	int 0x13

	jc .chsLoad					; unsupported -> load via chs
	jmp .lbaLoad					; supported -> load via lba

.lbaLoad:
	mov si, DAP_LOC
	mov [si], BYTE DAP_SIZE				; 0-1 size of dap (16)
	inc si

	mov [si], BYTE 0x00				; 1-2 0
	inc si

	mov [si], WORD 0x0001				; 2-4 number of sectors to read (1)
	add si, 2

	mov [si], WORD OLD_OFFSET 			; Offset to read to (0x7c00)
	add si, 2

	mov [si], WORD 0x0000				; Segment to read to (0)
	add si, 2

	mov bx, [bootPartition]
	mov cx, WORD [bx+LBA_OFFSET] 			; Load LBA of partition
	mov [si], cx
	add si, 2
	mov cx, WORD [bx+LBA_OFFSET+2]
	mov [si], cx
	add si, 2
	mov [si], DWORD 0x00000000

	mov ah, 0x42
	mov dl, [bootDrive]

	mov si, DAP_LOC
	int 0x13

	jnc VBRHandoff

	
.chsLoad:	
	mov ch, [bx + ENTRY_CYLINDER_OFFSET] 		; Cylinder number
	mov cl, [bx + ENTRY_SECTOR_OFFSET]   		; Sector Number
	mov dh, [bx + ENTRY_HEAD_OFFSET]     		; Head Number
	mov dl, [bootDrive]				; drive number

	xor bx, bx
	mov es, bx					; Segment to write to (0)
	mov bx, OLD_OFFSET				; offset to write to (0x7c00)

	mov ah, 0x02					; function code
	mov al, 1					; sectors to read

	cld						; read forwards

	int 0x13					; read from disk

	jmp VBRHandoff					; Hand off control to loaded vbr
	
	
VBRHandoff:
	mov dl, [bootDrive]				; Restore boot drive for vbr
	mov si, [bootPartition]				; Partition entry of boot partition for vbr
	mov bp, NEW_OFFSET + PARTITION_TABLE_OFFSET 	; Partition table addr for vbr
	jmp OLD_OFFSET					; Jump to vbr code
	
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
Data:;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

	bootPartition dw 0
	bootDrive db 0
	
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
	

 times 446 - ($ - $$) db 0
