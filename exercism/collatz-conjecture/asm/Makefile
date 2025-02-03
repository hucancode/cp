run-asm:
	docker build -t asm_c_program . && docker run --rm -v $(PWD):/workspace -it asm_c_program sh -c "nasm -f elf64 -o /var/asm.o *.asm && gcc -o /var/program *.c /var/asm.o && timeout 5 /var/program"
