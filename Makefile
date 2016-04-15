LD = ld
NASM = nasm
OBJDUMP = objdump
GDB = gdb

LD_FLAGS = -m elf_i386
NASM_FLAGS = -f elf32 -g
OBJDUMP_FLAGS = -M intel -D
GDB_FLAGS = -ex 'set disassembly-flavor intel' -ex 'display/i $$pc'

OBJECTS = jit.o linux.o

befunjit: $(OBJECTS)
	$(LD) $(LD_FLAGS) -o $@ $^

%.o: %.asm
	$(NASM) $(NASM_FLAGS) -o $@ $<

objdump: befunjit
	$(OBJDUMP) $(OBJDUMP_FLAGS) $<

gdb: befunjit
	$(GDB) $(GDB_FLAGS) $<

clean:
	rm -f befunjit $(OBJECTS)

.PHONY: objdump clean
