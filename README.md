# minos
- a fork of `https://github.com/vinc/moros`, a simple and lightweight operating system for the x86_64 architecture.
- the goal of this fork is to improve the codebase and add new features, but mainly to learn about operating systems - I'm not an expert in this field by any means!
## Running
```bash
make image output=video keyboard=qwerty
make qemu output=video nic=rtl8139
```
or on a codespace
```bash
make image output=serial keyboard=qwerty && make qemu output=serial nic=rtl8139
```