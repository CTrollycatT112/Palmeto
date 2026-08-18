# Palmeto Operating System

![GitHub License](https://img.shields.io/github/license/CTrollycatT112/Palmeto?style=flat&color=green)
![GitHub Repo stars](https://img.shields.io/github/stars/CTrollycatT112/Palmeto?style=flat)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/CTrollycatT112/Palmeto/ci.yml)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/CTrollycatT112/Palmeto?style=flat)

Palmeto is an operating system targeting AArch64 Le Potato

## About Palmeto

Palmeto is a hobby operating system, meant to be used as a desktop operating system for the libre potato.

The libre potato is a small board that currently costs around 60$. There isn't much options for an operating system on such a cheap board, and people who don't know linux basically have 0.

The goal for this project is to make a user friendly, Desktop-focused operating system.

## The kernel

The palmeto kernel is designed to be a UNIX and NT hybrid. It will use NT style 'managers' for things such as:

- Cache manager
- I/O manager
- Object Manager

It will use UNIX style for other things, such as interrupt handling, vfs, and more.

## Why even do this?

Even though nobody would use this OS daily, I'm doing this purely for myself.

I have a desktop PC, so I don't need my Le Potato board. It would be such a cool thing to have my own OS running on it, and the learning you get from OS dev is unbeatable.

## AI help

AI will not be used to generate any code for the O/S, as I want to learn, not copy...

However, there is some cases where AI will be used:

- Build system - I don't know many standard build tools, ai will be used here

- Reviewing - It can review the code i write, spot any critical bugs, then i will fix them

## Building and running the operating system

Before running, make sure you have the tools by running:

```bash
make deps
```

Then you can use:

```bash
make all
make run
make clean
```
