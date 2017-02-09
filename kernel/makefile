
all: build/evaos.iso

build/evaos.iso: build/isofiles/boot/kernel.bin build/isofiles/boot/grub/grub.cfg
	grub-mkrescue -o build/evaos.iso -d /usr/lib/grub/i386-pc  build/isofiles

build/isofiles/boot/kernel.bin: build/multiboot_header.o build/boot.o build/long_mode_start.o src/linker.ld
	ld --nmagic -o build/isofiles/boot/kernel.bin -T src/linker.ld build/multiboot_header.o build/boot.o build/long_mode_start.o

build/multiboot_header.o: src/multiboot_header.asm
	nasm -f elf64 -o build/multiboot_header.o src/multiboot_header.asm

build/boot.o: src/boot.asm
	nasm -f elf64 -o build/boot.o src/boot.asm

build/long_mode_start.o:
	nasm -f elf64 -o build/long_mode_start.o src/long_mode_start.asm


build/isofiles/boot/grub/grub.cfg: src/grub.cfg
	cp src/grub.cfg build/isofiles/boot/grub/

run: all
	qemu-system-x86_64 -cdrom build/evaos.iso


clean:
	rm -rf build/
	mkdir -p build/isofiles/boot/grub

