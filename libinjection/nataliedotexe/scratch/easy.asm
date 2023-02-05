global _main

_main:
  ; NtTerminateProcess for Windows 11 build 22621
  ; cf. https://hfiref0x.github.io/NT10_syscalls.html
  mov eax, 0x2C
  ; HANDLE -1 = this process
  mov r10, -1
  ; exit code 77
  mov rdx, 77
  syscall

  ; NtTerminateProcess should not return
  hlt