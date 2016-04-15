%include "const.mac"

section .data

jit.data:
  times 80 * 25 db ' '

section .text

jit.right:
  %rep 25
    times 80 * tpl.LEN nop
    jmp $ - 80 * tpl.LEN
  %endrep

jit.left:
  %rep 25
    times 80 * tpl.LEN nop
    jmp $ - 80 * tpl.LEN
  %endrep

jit.down:
  %rep 80
    times 25 * tpl.LEN nop
    jmp $ - 25 * tpl.LEN
  %endrep

jit.up:
  %rep 80
    times 25 * tpl.LEN nop
    jmp $ - 25 * tpl.LEN
  %endrep

global _start
_start:
  jmp jit.right
