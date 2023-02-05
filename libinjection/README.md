## Table of Contents

- [Summary](#summary)
- [Usage](#usage)

## Summary

inject and run code within notepad.exe

## Usage
Create the `.dll` file (injected code)
```pwsh
cd natalib
cargo build --lib
```
check if the app works
```pwsh
cd nataliedotexe
$proc = Start-Process -PassThru notepad ; Sleep 1 ; cargo run -- $proc.Id; Wait-Process $proc.Id ; $proc.ExitCode
```
expected output:
```pwsh
> kernel32 handle: HINSTANCE(140734093262848)
> LoadLibraryW address: 0x7fff35a48900
> Press enter to wreak havoc...
> 
> Created thread HANDLE(284), tid = 10300
> 112
```

