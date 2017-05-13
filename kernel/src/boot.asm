BITS 32

	%define MB_ERROR '0'		; Multiboot not supported
	%define CPUID_ERROR '1'		; CPUID not supported
	%define LM_ERROR '2'		; Long mode not supported

	%define MB_MAGIC 0x36d76289	; Magic number per multiboot standard

	%define STACK_SIZE 4096 * 4
	%define PAGE_SIZE 4096

extern long_mode_start
	
global _start

	
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; text is for runnable code
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
section .text
_start:
	mov esp, stack_top	; Set up stack
	mov edi, ebx		; Move multiboot info addr into first param reg

	;; confirm in post-multiboot env, long mode supported
	call checkMultiboot
	call checkCPUID
	call checkLongMode


	;; enable paging and switch to long mode
	call initPageTables
	call enablePaging

	;; enable sse
	call enableSSE

	;; load GDT, reads 16&32
	lgdt [gdt64.lgdtPacket]

	jmp gdt64.code:long_mode_start


;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Enable SSE for Rust
enableSSE:
	;; Test if SSE supported
	mov eax, 1		; Get feature bits
	cpuid
	test edx, 1 << 25
	jz .checkFailed

	;; Enable SSE
	mov eax, cr0		
	and ax, 0xFFFB		; disable floating point coprocessor emulation
	or ax, 0x2		; enable floating point coprocessor monitoring
	mov cr0, eax		; write changes
	
	mov eax, cr4
	or ax, 3 << 9		; enable use of SSE instructions and FPU
				; enable unmasked sse exceptions
	mov cr4, eax

	ret
.checkFailed:	
	mov al, "a"
	jmp error
	
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; enable paging
enablePaging:
	;; Make page table cpu accessable
	mov eax, page_map	; CPU reads page map out of cr3
	mov cr3, eax

	;; Enable physical address extension (PAE)
	mov eax, cr4		; PAE flag is in cr4
	or eax, 1 << 5		; Enable PAE flag
	mov cr4, eax		; write changes
	
	;; Enable long mode
	mov ecx, 0xC0000080	; point ecx to EFER MSR (long mode bit is there)
	rdmsr			; read EFER MSR into edx:eax
	or eax, 1 << 8		; set long mode bit
	wrmsr			; write edx:eax to EFER MSR

	;; Enable paging
	mov eax, cr0		; paging bit is in cr0
	or eax, 1 << 31		; set paging bit
	mov cr0, eax		; write changes

	ret

	
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; initialize page table system
initPageTables:
	
	;; Recursively map the page map 
	mov eax, page_map
	or eax, 0b11
	mov [page_map + 511 * 8], eax

	mov eax, pointer_table
	or eax, 0b11		    ; set 'entry present' and 'writable' bits
	mov [page_map], eax	    ; link pointer table as first entry in page 
	                            ; map 

	mov eax, page_directory
	or eax, 0b11		    ; set 'entry present' and 'writable' bits
	mov [pointer_table], eax    ; link page directory as first entry in
				    ; pointer table

	mov ecx, 0
	.entryLoop:
		mov eax, 0x200000   ; each page frame is 2MiB
		mul ecx
		or eax, 0b10000011 ; set 'present', 'writable', and 'huge' bits
		; entries are 8 bytes, advance appropriately + copy entry there
		mov [page_directory + ecx * 8], eax

		inc ecx
		cmp ecx, 512
		jne .entryLoop
	
	ret

	
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
	jmp error

	
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
align 4096
	
;; Reserve space for paging 
page_map:			; Reserve space for Page-Map Level-4 Table
	resb PAGE_SIZE
pointer_table:			; Reserve space for Page-Directory Pointer Table
	resb PAGE_SIZE
page_directory:			; Reserve space for Page-Directory Table
	resb PAGE_SIZE
	
;; Reserve space for stack
stack_bottom:
	resb STACK_SIZE		; Reserve 64 bytes of memory for stack
stack_top:	


	
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; read-only data
;;; ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
section .rodata
gdt64:
	dq 0			; GDT must always start with a 0 entry 
.code: equ $ - gdt64		; offset of code segment in GDT
	;; sets executable, code segment, present, and 64-bit entry bits
	dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)

;; info packet containing size and start of gdt for lgdt instruction 
.lgdtPacket:		 
	dw $ - gdt64 - 1	; GDT size must always be greater than 0 and is
				; reported as size - 1
	dq gdt64		; Address of GDT
