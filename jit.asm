section .rodata

%macro pad 1
  times 8 - $ + %1 nop
%endmacro

tpl.push:
  push strict dword 0
pad tpl.push

tpl.pop:
  add esp, 4
pad tpl.pop

tpl.dup:
  push dword [esp]
pad tpl.dup

tpl.swap:
  pop eax
  xchg eax, [esp]
  push eax
pad tpl.swap

tpl.add:
  pop eax
  add [esp], eax
pad tpl.add

tpl.sub:
  pop eax
  sub [esp], eax
pad tpl.sub

tpl.mul:
  pop eax
  imul dword [esp]
  mov [esp], eax
pad tpl.mul

tpl.div:
  pop ecx
  pop eax
  cdq
  idiv ecx
  push eax
pad tpl.div

tpl.mod:
  pop ecx
  pop eax
  cdq
  idiv ecx
  push edx
pad tpl.mod

tpl.not:
  not dword [esp]
pad tpl.not

section .text

global _start
_start:
  jmp _start
