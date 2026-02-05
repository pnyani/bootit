# bootit
A lightweight next boot selector

## What is Bootit?
Bootit is basically a CLI tool that makes your PC reboot to specific OS for one time.
```bash
bootit scan # list IDs of bootable entries

bootit alias add windows 1
bootit alias add archlinux 2
```
Then, reboot to it with
```bash
it windows
```
or
```bash
it 2
```
or whatever. It does not change boot order.
