override KARCH        := aarch64
override IMAGE_NAME    := palmeto
override BUILD_PROFILE := dev
override PROFILE_DIR   := $(if $(filter dev,$(BUILD_PROFILE)),debug,$(BUILD_PROFILE))
override BUILD_TYPE    := $(if $(filter dev,$(BUILD_PROFILE)),chk,fre)

KERNEL_DIR   := kernel
KERNEL_BIN   := target/aarch64-kernel/$(PROFILE_DIR)/$(IMAGE_NAME)
LIMINE_DIR   := build/limine
OUT_DIR      := build/$(BUILD_TYPE)
IMAGE_DIR    := $(OUT_DIR)/img_root
ISO_DIR      := $(OUT_DIR)/iso_root
SYM_DIR      := $(OUT_DIR)/symbols
IMAGE        := $(OUT_DIR)/$(IMAGE_NAME).img
ISO          := $(OUT_DIR)/$(IMAGE_NAME).iso
OVMF_FW      := build/ovmf/QEMU_EFI.fd

RUST_HOST    := $(shell rustc -vV | sed -n 's/host: //p')
LLVM_OBJCOPY := $(shell rustc --print sysroot)/lib/rustlib/$(RUST_HOST)/bin/llvm-objcopy

.PHONY: all kernel limine image iso symbols run clean distclean ovmf

all: image iso

kernel:
	cargo build --profile $(BUILD_PROFILE) -Zjson-target-spec

limine:
	if [ ! -d $(LIMINE_DIR) ]; then \
	    git clone https://github.com/limine-bootloader/limine.git \
	        --branch=v9.x-binary --depth=1 $(LIMINE_DIR); \
	fi
	$(MAKE) -C $(LIMINE_DIR)

ovmf:
	mkdir -p build/ovmf
	cp /usr/share/AAVMF/AAVMF_CODE.fd build/ovmf/code.fd 2>/dev/null || \
		cp /usr/share/qemu/edk2-aarch64-code.fd build/ovmf/code.fd 2>/dev/null || true
	cp /usr/share/AAVMF/AAVMF_VARS.fd build/ovmf/vars.fd 2>/dev/null || \
		cp /usr/share/qemu/edk2-arm-vars.fd build/ovmf/vars.fd 2>/dev/null || true

symbols: kernel
	mkdir -p $(SYM_DIR)
	$(LLVM_OBJCOPY) --only-keep-debug $(KERNEL_BIN) $(SYM_DIR)/$(IMAGE_NAME).dbg
	cp $(KERNEL_BIN) $(SYM_DIR)/$(IMAGE_NAME).stripped
	$(LLVM_OBJCOPY) --strip-debug --add-gnu-debuglink=$(SYM_DIR)/$(IMAGE_NAME).dbg $(SYM_DIR)/$(IMAGE_NAME).stripped

image: kernel limine
	rm -rf $(IMAGE_DIR)
	mkdir -p $(IMAGE_DIR)/EFI/BOOT $(IMAGE_DIR)/boot
	cp $(KERNEL_BIN) $(IMAGE_DIR)/boot/kernel
	cp limine.conf $(IMAGE_DIR)/boot/limine.conf
	cp $(LIMINE_DIR)/BOOTAA64.EFI $(IMAGE_DIR)/EFI/BOOT/BOOTAA64.EFI
	mkdir -p $(OUT_DIR)
	rm -f $(IMAGE)
	dd if=/dev/zero of=$(IMAGE) bs=1M count=64
	sgdisk $(IMAGE) -n 1:2048:0 -t 1:ef00 -m 1
	mformat -i $(IMAGE)@@1M -F ::
	mmd -i $(IMAGE)@@1M ::/EFI ::/EFI/BOOT ::/boot
	mcopy -i $(IMAGE)@@1M $(IMAGE_DIR)/EFI/BOOT/BOOTAA64.EFI ::/EFI/BOOT/
	mcopy -i $(IMAGE)@@1M $(IMAGE_DIR)/boot/kernel ::/boot/
	mcopy -i $(IMAGE)@@1M $(IMAGE_DIR)/boot/limine.conf ::/boot/

iso: kernel limine
	rm -rf $(ISO_DIR)
	mkdir -p $(ISO_DIR)/boot/limine $(ISO_DIR)/EFI/BOOT
	cp $(KERNEL_BIN) $(ISO_DIR)/boot/kernel
	cp limine.conf $(ISO_DIR)/boot/limine.conf
	cp $(LIMINE_DIR)/limine-uefi-cd.bin $(ISO_DIR)/boot/limine/
	cp $(LIMINE_DIR)/BOOTAA64.EFI $(ISO_DIR)/EFI/BOOT/BOOTAA64.EFI
	mkdir -p $(OUT_DIR)
	xorriso -as mkisofs -R -r -J \
	    --efi-boot boot/limine/limine-uefi-cd.bin \
	    -efi-boot-part --efi-boot-image --protective-msdos-label \
	    $(ISO_DIR) -o $(ISO)

run: image ovmf
	qemu-system-aarch64 \
		-M virt,acpi=off \
		-cpu cortex-a72 \
		-m 512M \
		-drive file=build/ovmf/code.fd,if=pflash,format=raw,readonly=on \
		-drive file=build/ovmf/vars.fd,if=pflash,format=raw \
		-fw_cfg name=opt/org.tianocore/BootTimeout,string=0 \
		-drive file=$(IMAGE),format=raw,if=none,id=hd0 \
		-device virtio-blk-pci,drive=hd0 \
		-device ramfb \
		-device virtio-gpu-pci \
		-display gtk \
		-serial stdio

clean:
	cargo clean
	rm -rf build