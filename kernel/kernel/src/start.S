.extern init
.extern task_start

// multiboot constants
.set MULTIBOOT_PAGE_ALIGN,          1 << 0
.set MULTIBOOT_MEMORY_INFO,         1 << 1
.set MULTIBOOT_HEADER_MAGIC,        0x1BADB002
.set MULTIBOOT_MAGIC,               0x2BADB002
.set MULTIBOOT_HEADER_FLAGS,        MULTIBOOT_PAGE_ALIGN | MULTIBOOT_MEMORY_INFO
.set MULTIBOOT_CHECKSUM,            -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

// multiboot header (needed to boot from GRUB)
.align  4
.long   MULTIBOOT_HEADER_MAGIC
.long   MULTIBOOT_HEADER_FLAGS
.long   MULTIBOOT_CHECKSUM

// entry point
.global _start
_start:
    // set stack pointer
    mov     $init_stack, %esp
    sub     $4, %esp

    // call init(&multiboot_header, &ustack)
    push    %esp
    push    %ebx
    call    init
    add     $4, %esp
    pop     %edx

    // return to user mode
    mov     (%edx), %edi    // stack pointer
    mov     %eax, %esi      // entry point
    jmp     task_start

.section .bss

.align 4
.rept 1024
    .long  0
.endr
init_stack:
