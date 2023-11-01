ENTRY(start)

MEMORY
{
    ROM (rx) : ORIGIN = 0x00100000, LENGTH = 1M /* Virt board ROM start */
    RAM (rwx) : ORIGIN = 0x40000000, LENGTH = 127M /* Virt board RAM start */
}

PHDRS {
    text PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

SECTIONS
{
    .text ALIGN(4K) : AT(ADDR(.text))
    {
        *(.text.table*)
        *(.text*)
    } > ROM :text

    .rodata ALIGN(4K) : AT(ADDR(.rodata)) {
        *(.rodata*)
    } > ROM :data

    .bss ALIGN(4K) (NOLOAD) : AT(ADDR(.bss))
    {
        *(.bss.stack*)
        . = ALIGN(16);
        . += 16; /* avoiding overlap */
        *(.bss.kmalloc*)
    } > RAM :bss = 0
}