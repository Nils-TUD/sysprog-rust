MODE ?= release
BUILD_DIR = build

KERNEL = $(BUILD_DIR)/kernel
USER1 = $(BUILD_DIR)/user1
USER2 = $(BUILD_DIR)/user2
TARGET = i686-unknown-kernel

LINKFLAGS = -m32 -static -ffreestanding -nostdlib -Wall -Wextra \
	-L $(BUILD_DIR)/$(TARGET)/$(MODE) -Wl,--build-id=none -Wl,-T,linker.ld

CARGO_FLAGS = --target $(TARGET) -Z build-std=core,alloc
ifeq ($(MODE),release)
  CARGO_FLAGS += --release
endif

QEMU_FLAGS = -serial stdio -d cpu_reset,pcall,guest_errors -D debug.txt

all: $(KERNEL) $(USER1) $(USER2)

$(KERNEL):	cargo
	gcc $(LINKFLAGS) -Wl,--section-start=.text=0x100000 -o $@ -l kernel

$(USER1):	cargo
	gcc $(LINKFLAGS) -Wl,--section-start=.text=0x800000 -o $@ -l testapp

$(USER2):	cargo
	gcc $(LINKFLAGS) -Wl,--section-start=.text=0x900000 -o $@ -l testapp

dis: $(KERNEL)
	objdump -SC $(KERNEL) | less

elf: $(KERNEL)
	readelf -aW $(KERNEL) | less

run: $(KERNEL) $(USER1) $(USER2)
	qemu-system-i386 $(QEMU_FLAGS) -kernel $(KERNEL) -initrd "$(USER1),$(USER2)" | tee log.txt

dbg: $(KERNEL) $(USER1) $(USER2)
	./debug.sh $(KERNEL) $(USER1) $(USER2) "$(QEMU_FLAGS)"

cargo:
	RUST_TARGET_PATH=`readlink -f .` CARGO_TARGET_DIR=$(BUILD_DIR) \
		cargo build $(CARGO_FLAGS)
