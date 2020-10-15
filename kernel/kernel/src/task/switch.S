// fn task_switch(prev: &mut KernelState, next: &KernelState);
.global task_switch
task_switch:
    mov     4(%esp), %eax    // prev (=current)
    mov     8(%esp), %ecx    // next

    // save registers
    mov     %ebx,  0(%eax)
    mov     %esp,  4(%eax)
    mov     %ebp,  8(%eax)
    mov     %esi, 12(%eax)
    mov     %edi, 16(%eax)
    pushf                    // load eflags
    pop     20(%eax)         // store

    // restore registers
    mov     16(%ecx), %edi
    mov     12(%ecx), %esi
    mov      8(%ecx), %ebp
    mov      4(%ecx), %esp
    mov      0(%ecx), %ebx
    push    20(%ecx)
    popf                     // load eflags

    ret

// the first code a task executes after task_switch.
// expects the stack pointer in %edi and the entry point in %esi
.global task_start
task_start:
    // init segment registers
    mov     $(4 << 3) | 3,%cx
    mov     %cx,%ds
    mov     %cx,%es
    mov     %cx,%fs
    mov     %cx,%gs

    // return to idle code
    push    $(4 << 3) | 3   // ss
    push    %edi            // esp
    push    $0x200          // eflags: interrupts enabled
    push    $(3 << 3) | 3   // cs
    push    %esi            // eip (return value from init)
    iret
