%use smartalign

global _start
extern os.exit

section .rodata

ins.LEN equ 8

ins.nop:
  nop dword [dword eax + eax]
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
ins.gt:

ins.str:
ins.east:
ins.west:
ins.south:
ins.north:
ins.rand:
ins.jump:
  jmp short $ + ins.LEN * 2
  align ins.LEN
ins.hif:
ins.vif:
ins.end:
  jmp os.exit
  align ins.LEN

ins.outv:
ins.outa:
ins.inpv:
ins.inpa:

ins.get:
ins.put:

section .data

txt.COLS equ 80
txt.ROWS equ 25

txt.data: times txt.COLS * txt.ROWS db ' '

section .text

txt.east:
  %rep txt.ROWS
    %rep txt.COLS
      nop dword [dword eax + eax]
      align ins.LEN
    %endrep
    jmp $ - txt.COLS * ins.LEN
    align ins.LEN
    jmp $ - txt.COLS * ins.LEN
    align ins.LEN
  %endrep
txt.west:
  %rep txt.ROWS
    %rep txt.COLS
      nop dword [dword eax + eax]
      align ins.LEN
    %endrep
    jmp $ - txt.COLS * ins.LEN
    align ins.LEN
    jmp $ - txt.COLS * ins.LEN
    align ins.LEN
  %endrep
txt.south:
  %rep txt.COLS
    %rep txt.ROWS
      nop dword [dword eax + eax]
      align ins.LEN
    %endrep
    jmp $ - txt.ROWS * ins.LEN
    align ins.LEN
    jmp $ - txt.ROWS * ins.LEN
    align ins.LEN
  %endrep
txt.north:
  %rep txt.COLS
    %rep txt.ROWS
      nop dword [dword eax + eax]
      align ins.LEN
    %endrep
    jmp $ - txt.ROWS * ins.LEN
    align ins.LEN
    jmp $ - txt.ROWS * ins.LEN
    align ins.LEN
  %endrep

jit.table:
  dd jit.not
  dd jit.str
  dd jit.jump
  dd jit.pop
  dd jit.nop
  dd jit.inpv
  dd jit.nop
  dd jit.nop
  dd jit.nop
  dd jit.mul
  dd jit.add
  dd jit.outa
  dd jit.sub
  dd jit.outv
  dd jit.div
  dd jit.push
  dd jit.push
  dd jit.push
  dd jit.push
  dd jit.push
  dd jit.push
  dd jit.push
  dd jit.push
  dd jit.push
  dd jit.push
  dd jit.nop
  dd jit.nop
  dd jit.west
  dd jit.nop
  dd jit.east
  dd jit.rand
  dd jit.end
  times 27 dd jit.nop
  dd jit.swap
  dd jit.nop
  dd jit.north
  dd jit.nop
  dd jit.gt
  times 21 dd jit.nop
  dd jit.south
  times 7 dd jit.nop
  dd jit.inpa
  dd jit.nop

jit.nop:
jit.push:
jit.pop:
jit.dup:
jit.swap:

jit.add:
jit.sub:
jit.mul:
jit.div:
jit.mod:
jit.not:
jit.gt:

jit.str:
jit.east:
jit.west:
jit.south:
jit.north:
jit.rand:
jit.jump:
jit.hif:
jit.vif:
jit.end:

jit.outv:
jit.outa:
jit.inpv:
jit.inpa:

jit.get:
jit.put:

_start:
  jmp txt.east
