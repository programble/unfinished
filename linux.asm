global os.exit

os.exit:
  mov eax, 1
  xor ebx, ebx
  int 0x80
