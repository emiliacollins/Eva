BITS 32

	%define MB_ERROR '0'		; Multiboot not supported
	%define CPUID_ERROR '1'		; CPUID not supported
	%define LM_ERROR '2'		; Long mode not supported

	%define MB_MAGIC 0x36d76289	; Magic number per multiboot standard

	%define STACK_SIZE 64
	

	
global _start

	
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; text is for runnable code
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
section .text
_start:
	mov esp, stack_top	; Set up stack

	;; confirm in post-multiboot env, long mode supported
	call checkMultiboot
	call checkCPUID
	call checkLongMode
	
	mov dword [0xb8000], 0x2f4b2f4f
	hlt					 


;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; check if long mode is supported by processor
checkLongMode:
	;; test if querying for long mode (by extension long mode) is available
	mov eax, 0x80000000	; get highest query code supported
	cpuid
	cmp eax, 0x80000001	; 0x80000001 is long mode query
	jb .checkFailed

	;; test if long mode is available
	mov eax, 0x80000001	; long mode query code
	cpuid
	test edx, 1 << 29	; 29th bit indicates long mode
	jz .checkFailed
	ret
.checkFailed:
	mov al, LM_ERROR

	
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; checks if cpuid api is supported by processor. supported if 21st flag bit
;;; can be flipped
checkCPUID:
	;; get a copy of the flags
	pushfd
	pop eax

	;; back up original flags config
	mov ecx, eax

	;; flip ID bit in flags
	xor eax, 1 << 21

	;; see if bit flip holds
	push eax
	popfd

	pushfd
	pop eax

	;; restore original flag values
	push ecx
	popfd

	;; check if ID bit flipped
	cmp ecx, eax
	je .checkFailed
	ret
.checkFailed:
	mov al, CPUID_ERROR
	jmp error


	
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; check for magic number indicating kernel loaded in multiboot compliance
checkMultiboot:
	cmp eax, MB_MAGIC	
	jne .checkFailed
	ret
.checkFailed:
	mov al, MB_ERROR
	jmp error

	
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Print out character in al in format 'ERR: c'
error:
	;; VGA buffer starts at 0xb8000
	;; Each character in buffer is a word
	;; first byte is formatting, second is char
	mov dword [0xb8000], 0x4f524f45 		; 'ER'
	mov dword [0xb8004], 0x4f3a4f52			; 'R:'
	mov dword [0xb8008], 0x4f004f20			; ' ' 
	mov byte [0xb800a], al
	hlt


;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; bss is for statically declared data
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
section .bss
stack_bottom:
	resb STACK_SIZE		; Reserve 64 bytes of memory for stack
stack_top:	
