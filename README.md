# Palmeto

Palmeto is an operating system targeting AArch64. It was made for one thing: the Libre Computer "Le Potato" board.

## Why the Le Potato?

The Le Potato costs around $60 right now. You might install Linux on it — the problem is that even Linux is slow on 2GB of RAM

The goal of this operating system is to give myself a system that's actually usable, for just $60

## Desktop focused

Even if Linux is technically usable on the Le Potato, the daily user doesn't know Linux. It's confusing for most people, and it's built more for servers than for everyday use

Palmeto is built as a desktop-first operating system, focusing on providing a Windows 11-like feel — just for $60

Sounds crazy, but the kernel will be heavily optimized. I'll be doing all kinds of aggressive compression, swapping, and more to make sure 2GB of RAM is enough

## The kernel

The design is mostly UNIX-focused, but more of a hybrid/safer version

Certain things — like the CFS scheduler — have been removed in favor of MLFQ

The kernel is designed to be lightweight, using smart compression and swapping to ensure maximum speed on low-end hardware

## Why even do this?

Even though nobody would use this OS daily, I'm doing this purely for myself

I have a desktop PC, so I don't need my Le Potato board. It would be such a cool thing to have my own OS running on it — and the learning you get from OS dev is unbeatable

## AI help

AI will not be used to generate any code for the O/S, 
as I want to learn, not copy..

However, there is some cases where AI will be used:

Build system - I don't know many standard build tools, ai will be used here

Reviewing - It can review the code i write, spot any critical bugs, then i will fix them MYSELF

## Building and running the operating system

Before running,make sure you have the tools Run `toolchain/check-deps.sh`

MAKE:
```bash
make all
make run
make clean
```