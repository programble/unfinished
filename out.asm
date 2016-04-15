%include "const.mac"

global out.data, out.right, out.left, out.down, out.up

section .data

out.data:
  times 80 * 25 db ' '

section .text

out.right:
  %rep 25
    times 80 * tpl.LEN nop
    jmp $ - 80 * tpl.LEN
  %endrep

out.left:
  %rep 25
    times 80 * tpl.LEN nop
    jmp $ - 80 * tpl.LEN
  %endrep

out.down:
  %rep 80
    times 25 * tpl.LEN nop
    jmp $ - 25 * tpl.LEN
  %endrep

out.up:
  %rep 80
    times 25 * tpl.LEN nop
    jmp $ - 25 * tpl.LEN
  %endrep
