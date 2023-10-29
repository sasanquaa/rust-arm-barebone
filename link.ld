ENTRY(start)

MEMORY
{
    ROM (rx) : ORIGIN = 0x00000000, LENGTH = 1M /* Virt board ROM start */
    RAM (rwx) : ORIGIN = 0x40000000, LENGTH = 127M /* Virt board RAM start */
}

PHDRS {
    text PT_LOAD;
    bss PT_LOAD;
}

SECTIONS
{
    .text ALIGN(4K) : AT(ADDR(.text))
    {
        *(.text*)
        *(.rodata*)
    } > ROM :text
    .bss ALIGN(4K) (NOLOAD) : AT(ADDR(.text) + SIZEOF(.text))
    {
        . += 32768; /* 32 KiB for stack */
        STACK_TOP = .;
        . += 16;
        . += 2097152; /* 2 MiB for kmalloc */
        *(.ARM.exidx) /* ??? */
    } > RAM :bss = 0
}