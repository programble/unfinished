%use smartalign

global _start
extern os.exit, os.outa, os.inpa

section .rodata

ins.LEN equ 16

ins.nop:
  nop dword [dword eax + eax]
  nop dword [dword eax + eax]

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
  pop eax
  test eax, eax
  setz al
  movzx eax, al
  push eax
  align ins.LEN
ins.gt:
  pop eax
  pop edx
  cmp edx, eax
  setg al
  movzx eax, al
  push eax
  align ins.LEN

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
  mov ebp, os.exit
  jmp ebp
  align ins.LEN

ins.outv:
ins.outa:
  mov ebp, os.outa
  pop eax
  call ebp
  align ins.LEN
ins.inpv:
ins.inpa:
  mov ebp, os.inpa
  call ebp
  movzx eax, al
  push eax
  align ins.LEN

ins.get:
ins.put:

section .data

txt.COLS equ 80
txt.ROWS equ 25

txt.data: times txt.COLS * txt.ROWS db ' '

txt.east:
  %rep txt.ROWS
    %rep txt.COLS
      nop dword [dword eax + eax]
      nop dword [dword eax + eax]
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
      nop dword [dword eax + eax]
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
      nop dword [dword eax + eax]
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
      nop dword [dword eax + eax]
    %endrep
    jmp $ - txt.ROWS * ins.LEN
    align ins.LEN
    jmp $ - txt.ROWS * ins.LEN
    align ins.LEN
  %endrep

section .text

jit.write:
  movzx eax, bh
  mov edx, txt.COLS + 2
  mul edx
  movzx edx, bl
  add eax, edx
  mov edx, ins.LEN
  mul edx
  lea edi, [txt.east + eax]
  mov ecx, ins.LEN / 4
  rep movsd
  ret

_start:
  mov bx, 0x0000
  mov esi, ins.inpa
  call jit.write
  inc bx
  mov esi, ins.outa
  call jit.write
  jmp txt.east
