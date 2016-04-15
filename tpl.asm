%macro pad 1
  times 8 - $ + %1 nop
%endmacro

section .rodata

extern os.exit

global tpl.nop
global tpl.push, tpl.pop, tpl.dup, tpl.swap
global tpl.add, tpl.sub, tpl.mil, tpl.div, tpl.mod,
global tpl.not
global tpl.end

tpl.nop:
pad tpl.nop

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

tpl.end:
  jmp os.exit
pad tpl.end
