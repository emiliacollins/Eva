ENTRY(_start)

SECTIONS {
	 . = 1M;

	 .boot :
	 {
		/* Place multiboot header at beginning */
		*(.multiboot_header)
	 }
	 .text :
	 {
		*(.text)
	 }


	 /* .data : */
	 /* { */
	 /* 	*(.data) */
	 /* } */
}
