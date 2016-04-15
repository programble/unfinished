LD = ld
NASM = nasm
NASM_FLAGS = -f elf64

OBJECTS = jit.o

befunjit: $(OBJECTS)
	ld -o $@ $^

%.o: %.asm
	nasm $(NASM_FLAGS) -o $@ $^

clean:
	rm -f befunjit $(OBJECTS)

.PHONY: clean
