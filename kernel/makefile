RUST_BUILD_DIR=target/x86_64-unknown-linux-gnu/debug


all: build/evaos.iso $(RUST_BUILD_DIR)/libeva_os.a 

build/evaos.iso: build/isofiles/boot/kernel.bin build/isofiles/boot/grub/grub.cfg
	grub-mkrescue -o build/evaos.iso -d /usr/lib/grub/i386-pc  build/isofiles

build/isofiles/boot/kernel.bin: build/multiboot_header.o build/boot.o build/long_mode_start.o src/linker.ld $(RUST_BUILD_DIR)/libeva_os.a
	ld --gc-sections --nmagic -o build/isofiles/boot/kernel.bin -T src/linker.ld build/multiboot_header.o build/boot.o build/long_mode_start.o $(RUST_BUILD_DIR)/libeva_os.a

build/multiboot_header.o: src/multiboot_header.asm
	nasm -f elf64 -o build/multiboot_header.o src/multiboot_header.asm

build/boot.o: src/boot.asm
	nasm -f elf64 -o build/boot.o src/boot.asm

build/long_mode_start.o: src/long_mode_start.asm
	nasm -f elf64 -o build/long_mode_start.o src/long_mode_start.asm


build/isofiles/boot/grub/grub.cfg: src/grub.cfg
	cp src/grub.cfg build/isofiles/boot/grub/

$(RUST_BUILD_DIR)/libeva_os.a: src/lib.rs src/vga_interface.rs Cargo.toml src/memory/*
	cargo build --target=x86_64-unknown-linux-gnu

run: all
	qemu-system-x86_64 -m 2G -cdrom build/evaos.iso


clean:
	rm -rf build/
	cargo clean
	mkdir -p build/isofiles/boot/grub

