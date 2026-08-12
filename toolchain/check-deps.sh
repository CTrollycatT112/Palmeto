#!/usr/bin/env bash
set -u

MISSING=0
DISTRO_HINT="apt install"
if command -v pacman >/dev/null 2>&1; then DISTRO_HINT="pacman -S"; fi
if command -v dnf >/dev/null 2>&1; then DISTRO_HINT="dnf install"; fi

check_bin() {
    local bin="$1" pkg_apt="$2" pkg_pacman="$3" pkg_dnf="$4"
    if command -v "$bin" >/dev/null 2>&1; then
        echo "  [ok] $bin"
    else
        local pkg="$pkg_apt"
        case "$DISTRO_HINT" in
            "pacman -S") pkg="$pkg_pacman" ;;
            "dnf install") pkg="$pkg_dnf" ;;
        esac
        echo "  [MISSING] $bin  ->  sudo $DISTRO_HINT $pkg"
        MISSING=1
    fi
}

echo "Checking host tools..."
check_bin git         git            git            git
check_bin make        build-essential base-devel     make
check_bin cc          build-essential base-devel     gcc
check_bin dd           coreutils      coreutils      coreutils
check_bin sgdisk      gdisk          gptfdisk       gdisk
check_bin mformat     mtools         mtools         mtools
check_bin mcopy       mtools         mtools         mtools
check_bin xorriso     xorriso        libisoburn     xorriso
check_bin qemu-system-aarch64  qemu-system-arm  qemu-system-arm  qemu-system-aarch64

echo
echo "Checking Rust toolchain..."
if command -v rustup >/dev/null 2>&1; then
    if rustup component list --toolchain nightly 2>/dev/null | grep -q "rust-src.*installed"; then
        echo "  [ok] rust-src (nightly)"
    else
        echo "  [MISSING] rust-src  ->  rustup component add rust-src --toolchain nightly"
        MISSING=1
    fi
    if rustup component list --toolchain nightly 2>/dev/null | grep -q "llvm-tools.*installed"; then
        echo "  [ok] llvm-tools (nightly)"
    else
        echo "  [MISSING] llvm-tools  ->  rustup component add llvm-tools --toolchain nightly"
        MISSING=1
    fi
else
    echo "  [MISSING] rustup itself -> https://rustup.rs"
    MISSING=1
fi

echo
echo "Checking AAVMF UEFI firmware..."
if [ -f /usr/share/AAVMF/AAVMF_CODE.fd ]; then
    echo "  [ok] /usr/share/AAVMF/AAVMF_CODE.fd"
elif [ -f /usr/share/edk2/aarch64/QEMU_EFI.fd ]; then
    echo "  [ok] /usr/share/edk2/aarch64/QEMU_EFI.fd (adjust OVMF_FW path in Makefile)"
else
    echo "  [MISSING] AAVMF/edk2-aarch64 firmware  ->  sudo $DISTRO_HINT qemu-efi-aarch64"
    MISSING=1
fi

echo
if [ "$MISSING" -eq 0 ]; then
    echo "All good — run 'make all' or 'make run'."
else
    echo "Some tools are missing — install the ones flagged above, then re-run this script."
fi
exit "$MISSING"