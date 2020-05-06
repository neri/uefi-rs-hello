.PHONEY: all clean run install love

RUST_ARCH	= x86_64-unknown-uefi
MNT			= ./mnt
EFI_BOOT	= $(MNT)/EFI/BOOT
TARGET		= target/$(RUST_ARCH)/release/uefi-rs-hello.efi
EXECUTABLE	= $(EFI_BOOT)/BOOTX64.EFI
OVMF		= var/ovmfx64.fd

all: $(TARGET)

$(EFI_BOOT):
	mkdir -p $(EFI_BOOT)

$(TARGET): src/**
	cargo xbuild --target $(RUST_ARCH) --release

clean:
	-rm -rf target

$(EXECUTABLE): $(TARGET) $(EFI_BOOT)
	cp $< $@

install: $(EXECUTABLE)

run: install $(OVMF)
	qemu-system-x86_64 -bios $(OVMF) -drive format=raw,file=fat:rw:$(MNT) -monitor stdio
	#-nographic 
