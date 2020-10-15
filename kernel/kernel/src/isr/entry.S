// macro to build a default-isr-handler
.macro BUILD_DEF_ISR no
    .global isr_\no
    isr_\no:
    push    $0                      // error-code (no error here)
    push    $\no                    // the interrupt-number
    jmp     isr_common
.endm

// macro to build an error-isr-handler
.macro BUILD_ERR_ISR no
    .global isr_\no
    isr_\no:
    // the error-code has already been pushed
    push    $\no                    // the interrupt-number
    jmp     isr_common
.endm

// our ISRs
BUILD_DEF_ISR 0
BUILD_DEF_ISR 1
BUILD_DEF_ISR 2
BUILD_DEF_ISR 3
BUILD_DEF_ISR 4
BUILD_DEF_ISR 5
BUILD_DEF_ISR 6
BUILD_DEF_ISR 7
BUILD_ERR_ISR 8
BUILD_DEF_ISR 9
BUILD_ERR_ISR 10
BUILD_ERR_ISR 11
BUILD_ERR_ISR 12
BUILD_ERR_ISR 13
BUILD_ERR_ISR 14
BUILD_DEF_ISR 15
BUILD_DEF_ISR 16
BUILD_ERR_ISR 17
BUILD_DEF_ISR 18
BUILD_DEF_ISR 19
BUILD_DEF_ISR 20
BUILD_DEF_ISR 21
BUILD_DEF_ISR 22
BUILD_DEF_ISR 23
BUILD_DEF_ISR 24
BUILD_DEF_ISR 25
BUILD_DEF_ISR 26
BUILD_DEF_ISR 27
BUILD_DEF_ISR 28
BUILD_DEF_ISR 29
BUILD_DEF_ISR 30
BUILD_DEF_ISR 31
BUILD_DEF_ISR 32
BUILD_DEF_ISR 33
BUILD_DEF_ISR 34
BUILD_DEF_ISR 35
BUILD_DEF_ISR 36
BUILD_DEF_ISR 37
BUILD_DEF_ISR 38
BUILD_DEF_ISR 39
BUILD_DEF_ISR 40
BUILD_DEF_ISR 41
BUILD_DEF_ISR 42
BUILD_DEF_ISR 43
BUILD_DEF_ISR 44
BUILD_DEF_ISR 45
BUILD_DEF_ISR 46
BUILD_DEF_ISR 47
BUILD_DEF_ISR 48

.global isr_null
isr_null:
    push    $0                      // error-code (no error here)
    push    $32                     // the interrupt-number
    jmp     isr_common

.global isr_common
isr_common:
    // save all general purpose registers
    pusha

    // call C++ routine with state as argument
    push    %esp
    call    isr_handler
    add     $4, %esp

    // restore all general purpose registers
    popa

    // remove error-code and interrupt-number from stack and return
    add     $(4 * 2), %esp
    iret
