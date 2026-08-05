override KARCH        := aarch64
override IMAGE_NAME    := palmeto
override BUILD_PROFILE := dev
override PROFILE_DIR   := $(if $(filter dev,$(BUILD_PROFILE)),debug,$(BUILD_PROFILE))

KERNEL_DIR   := kernel
KERNEL_BIN   := target/aarch64-kernel/$(PROFILE_DIR)/$(IMAGE_NAME)
LIMINE_DIR   := limine
IMAGE_DIR    := build/image
IMAGE        := build/$(IMAGE_NAME).hdd
OVMF_FW      := build/ovmf/QEMU_EFI.fd

.PHONY: all kernel limine image run clean distclean ovmf

all: image

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
	cp /usr/share/AAVMF/AAVMF_CODE.fd $(OVMF_FW) 2>/dev/null || \
		echo "Could not find AAVMF firmware — install qemu-efi-aarch64 (apt) or edk2-aarch64 (pacman/dnf) and point OVMF_FW at it manually."

image: kernel limine
	rm -rf $(IMAGE_DIR)
	mkdir -p $(IMAGE_DIR)/EFI/BOOT $(IMAGE_DIR)/boot
	cp $(KERNEL_BIN) $(IMAGE_DIR)/boot/kernel
	cp limine.conf $(IMAGE_DIR)/boot/limine.conf
	cp $(LIMINE_DIR)/BOOTAA64.EFI $(IMAGE_DIR)/EFI/BOOT/BOOTAA64.EFI
	mkdir -p build
	rm -f $(IMAGE)
	dd if=/dev/zero of=$(IMAGE) bs=1M count=64
	sgdisk $(IMAGE) -n 1:2048:0 -t 1:ef00 -m 1
	mformat -i $(IMAGE)@@1M -F ::
	mmd -i $(IMAGE)@@1M ::/EFI ::/EFI/BOOT ::/boot
	mcopy -i $(IMAGE)@@1M $(IMAGE_DIR)/EFI/BOOT/BOOTAA64.EFI ::/EFI/BOOT/
	mcopy -i $(IMAGE)@@1M $(IMAGE_DIR)/boot/kernel ::/boot/
	mcopy -i $(IMAGE)@@1M $(IMAGE_DIR)/boot/limine.conf ::/boot/
	
run: image ovmf
	qemu-system-aarch64 \
		-M virt \
		-cpu cortex-a72 \
		-m 512M \
		-bios $(OVMF_FW) \
		-drive file=$(IMAGE),format=raw,if=none,id=hd0 \
		-device virtio-blk-device,drive=hd0 \
		-device ramfb \
		-device virtio-gpu-pci \
		-display gtk \
		-serial stdio

clean:
	cargo clean
	rm -rf build
	rm -rf limine