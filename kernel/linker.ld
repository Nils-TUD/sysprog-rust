ENTRY(_start)

PHDRS
{
    text    PT_LOAD;
    data    PT_LOAD;
    bss     PT_LOAD;
}

SECTIONS
{
   . = 1M;

   .text BLOCK(4K) : ALIGN(4K)
   {
      *(.text*)
   } : text

   .rodata :
   {
      *(.rodata*)
   } : text

   .data BLOCK(4K) : ALIGN(4K)
   {
      *(.data*)
      *(COMMON)
   } : data

   .bss BLOCK(4K) : ALIGN(4K)
   {
      *(.bss*)
      PROVIDE(bss_end = .);
   } : bss
}
