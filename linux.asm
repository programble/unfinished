global os.exit, os.outa, os.inpa

section .data

buf db 0

section .text

os.exit:
  mov eax, 1
  xor ebx, ebx
  int 0x80

os.outa:
  mov [buf], al
  mov eax, 4
  mov ebx, 1
  mov ecx, buf
  mov edx, 1
  int 0x80
  ret

os.inpa:
  mov eax, 3
  mov ebx, 0
  mov ecx, buf
  mov edx, 1
  int 0x80
  mov al, [buf]
  ret
