#!/usr/bin/env bash
set -u

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
MISSING=0
DISTRO_HINT="apt install"

if command -v pacman >/dev/null 2>&1; then
    DISTRO_HINT="pacman -S"
elif command -v dnf >/dev/null 2>&1; then
    DISTRO_HINT="dnf install"
fi

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
check_bin git                     git                 git                 git
check_bin make                    build-essential     base-devel          make
check_bin cc                      build-essential     base-devel          gcc
check_bin dd                      coreutils           coreutils           coreutils
check_bin sgdisk                  gdisk               gptfdisk            gdisk
check_bin mformat                 mtools              mtools              mtools
check_bin mcopy                   mtools              mtools              mtools
check_bin xorriso                 xorriso             libisoburn          xorriso
check_bin qemu-system-aarch64     qemu-system-arm     qemu-system-arm     qemu-system-aarch64
check_bin aarch64-linux-gnu-gcc   gcc-aarch64-linux-gnu aarch64-linux-gnu-gcc gcc-aarch64-linux-gnu

echo
echo "Checking Rust toolchain..."
if command -v rustup >/dev/null 2>&1; then
    if rustup component list --toolchain nightly 2>/dev/null | grep -q "rust-src.*installed"; then
        echo "  [ok] rust-src (nightly)"
    else
        echo "  [MISSING] rust-src (nightly)"
        MISSING=1
    fi
    if rustup component list --toolchain nightly 2>/dev/null | grep -q "llvm-tools.*installed"; then
        echo "  [ok] llvm-tools (nightly)"
    else
        echo "  [MISSING] llvm-tools (nightly)"
        MISSING=1
    fi
    if rustup component list --toolchain nightly 2>/dev/null | grep -q "clippy.*installed"; then
        echo "  [ok] clippy (nightly)"
    else
        echo "  [MISSING] clippy (nightly)"
        MISSING=1
    fi
else
    echo "  [MISSING] rustup"
    MISSING=1
fi

echo
echo "Checking AAVMF UEFI firmware..."
if [[ -f /usr/share/AAVMF/AAVMF_CODE.fd ]]; then
    echo "  [ok] /usr/share/AAVMF/AAVMF_CODE.fd"
elif [[ -f /usr/share/edk2/aarch64/QEMU_EFI.fd ]]; then
    echo "  [ok] /usr/share/edk2/aarch64/QEMU_EFI.fd"
else
    echo "  [MISSING] AAVMF/edk2-aarch64 firmware"
    MISSING=1
fi

echo
if [[ "$MISSING" -eq 0 ]]; then
    echo "All good - run 'make all' or 'make run'."
    exit 0
fi

echo "Some dependencies are missing."
read -r -p "Do you want to install missing deps? [Y/N] " answer
if [[ "$answer" =~ ^[Yy]([Ee][Ss])?$ ]]; then
    "$SCRIPT_DIR/install-deps.sh"
    exit $?
fi

echo "No dependencies were installed."
exit 1