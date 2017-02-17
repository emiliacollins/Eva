ENTRY(_start)

SECTIONS {
	 . = 1M;

	 .boot :
	 {
		/* Place multiboot header at beginning, make sure not removed */
		KEEP(*(.multiboot_header))
	 }

	 /* Counteract rust compiler optimizations to defragment kernel sections */
	 .text :
	 {
		*(.text .text.*)
	 }
	 .rodata :
	 {
		*(.rodata .rodata.*)
	 }
	 .data.rel.ro :
	 {
		*(.data.rel.ro.local*)
		*(.data.rel.ro .data.rel.ro.*)
	 }

}
