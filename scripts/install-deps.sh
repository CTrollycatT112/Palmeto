#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Linux" ]]; then
    echo "Dependency installation is only supported on Linux."
    exit 1
fi

if [[ "${EUID:-$(id -u)}" -eq 0 ]]; then
    SUDO=""
else
    SUDO="sudo"
fi

if command -v apt-get >/dev/null 2>&1; then
    $SUDO apt-get update
    $SUDO apt-get install --no-install-recommends -y \
        build-essential \
        gcc-aarch64-linux-gnu \
        libc6-dev-arm64-cross \
        gdisk \
        mtools \
        xorriso \
        qemu-system-arm \
        qemu-efi-aarch64 \
        curl
elif command -v pacman >/dev/null 2>&1; then
    $SUDO pacman -Sy --needed \
        base-devel \
        aarch64-linux-gnu-gcc \
        gptfdisk \
        mtools \
        libisoburn \
        qemu-system-aarch64 \
        qemu-efi-aarch64 \
        curl
elif command -v dnf >/dev/null 2>&1; then
    $SUDO dnf install -y \
        make \
        gcc \
        gcc-aarch64-linux-gnu \
        gdisk \
        mtools \
        xorriso \
        qemu-system-aarch64 \
        edk2-aarch64 \
        curl
else
    echo "No supported package manager was found. Supported managers: apt, pacman, dnf."
    exit 1
fi

if ! command -v rustup >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"
fi

rustup toolchain install nightly --profile minimal
rustup component add rust-src llvm-tools-preview --toolchain nightly

echo "Dependencies installed. Run scripts/check-deps.sh to verify them."