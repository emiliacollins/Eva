;;; NOTE: This code was written following a tutorial at
;;;       http://os.phil-opp.com/multiboot-kernel.html

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; This file contains a header defined by the multiboot standard for multiboot
;;; enabled kernels.
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

section .multiboot_header
header_start:
    dd 0xE85250D6                   ;; Magic number
    dd 0			    ;; 0 indicates i386 architecture
    dd header_end - header_start    ;; Size of multiboot header

    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
    ;; Optional multiboot flags may be used here per the multiboot standard


    ;; End tag
    dw 0                            ;; End tag has format u16=0
    dw 0                            ;; u16=0
    dd 8                            ;; u32=8
header_end:
