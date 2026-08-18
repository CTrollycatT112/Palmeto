# Palmeto

[![License](https://img.shields.io/github/license/CTrollycatT112/Palmeto)](https://github.com/CTrollycatT112/Palmeto/blob/main/LICENSE)
[![Stars](https://img.shields.io/github/stars/CTrollycatT112/Palmeto)](https://github.com/CTrollycatT112/Palmeto/stargazers)
[![Build](https://github.com/CTrollycatT112/Palmeto/actions/workflows/ci.yml/badge.svg)](https://github.com/CTrollycatT112/Palmeto/actions/workflows/ci.yml)
[![Issues](https://img.shields.io/github/issues/CTrollycatT112/Palmeto)](https://github.com/CTrollycatT112/Palmeto/issues)

Palmeto is an operating system targeting AArch64 Le Potato

## Desktop focused

Palmeto is built as a desktop operating system, focusing on providing a Windows 11-like feel, just for $60

Sounds crazy, but the kernel will be heavily optimized, I'll be doing all kinds of aggressive compression, swapping, and more to make sure 2GB of RAM is enough

## The kernel

The design is mostly UNIX-focused, but more of a hybrid/safer version

Certain things - like the CFS scheduler, have been removed in favor of MLFQ

The kernel is designed to be lightweight, using smart compression and swapping for speed on low-end hardware

## Why even do this?

Even though nobody would use this OS daily, I'm doing this purely for myself

I have a desktop PC, so I don't need my Le Potato board. It would be such a cool thing to have my own OS running on it, and the learning you get from OS dev is unbeatable

## AI help

AI will not be used to generate any code for the O/S,
as I want to learn, not copy...

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
