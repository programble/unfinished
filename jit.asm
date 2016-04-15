%use smartalign

global _start
extern os.exit

section .rodata

ins.LEN equ 8

ins.nop:
  nop
  align ins.LEN
ins.push:
  push strict dword 0
  align ins.LEN
ins.pop:
  add esp, 4
  align ins.LEN
ins.dup:
  push dword [esp]
  align ins.LEN
ins.swap:
  pop eax
  xchg eax, [esp]
  push eax
  align ins.LEN

ins.add:
  pop eax
  add [esp], eax
  align ins.LEN
ins.sub:
  pop eax
  sub [esp], eax
  align ins.LEN
ins.mul:
  pop eax
  imul dword [esp]
  mov [esp], eax
  align ins.LEN
ins.div:
  pop ebx
  pop eax
  cdq
  idiv ebx
  push eax
  align ins.LEN
ins.mod:
  pop ebx
  pop eax
  cdq
  idiv ebx
  push edx
  align ins.LEN

ins.not:
  not dword [esp]
  align ins.LEN

ins.end:
  jmp os.exit
  align ins.LEN

section .data

txt.COLS equ 80
txt.ROWS equ 25

txt.data: times txt.COLS * txt.ROWS db ' '

section .text

txt.east:
  %rep txt.ROWS
    times txt.COLS * ins.LEN nop
    jmp $ - txt.COLS * ins.LEN
  %endrep
txt.west:
  %rep txt.ROWS
    times txt.COLS * ins.LEN nop
    jmp $ - txt.COLS * ins.LEN
  %endrep
txt.south:
  %rep txt.COLS
    times txt.ROWS * ins.LEN nop
    jmp $ - txt.ROWS * ins.LEN
  %endrep
txt.north:
  %rep txt.COLS
    times txt.ROWS * ins.LEN nop
    jmp $ - txt.ROWS * ins.LEN
  %endrep

_start:
  jmp txt.east
