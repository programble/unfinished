LD = ld
NASM = nasm

LD_FLAGS = -m elf_i386
NASM_FLAGS = -f elf32

OBJECTS = jit.o tpl.o

befunjit: $(OBJECTS)
	ld $(LD_FLAGS) -o $@ $^

%.o: %.asm
	nasm $(NASM_FLAGS) -o $@ $^

clean:
	rm -f befunjit $(OBJECTS)

.PHONY: clean
