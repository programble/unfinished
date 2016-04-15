section .rodata

tpl.push:
  push strict dword 0

tpl.add:
  pop eax
  add [esp], eax

tpl.sub:
  pop eax
  sub [esp], eax

tpl.mul:
  pop eax
  imul dword [esp]
  mov [esp], eax

tpl.div:
  pop eax
  xor edx, edx
  idiv dword [esp]
  mov [esp], eax

tpl.mod:
  pop eax
  xor edx, edx
  idiv dword [esp]
  mov [esp], edx

tpl.not:
  not dword [esp]
