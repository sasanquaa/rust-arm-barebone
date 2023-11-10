ENTRY(boot)

MEMORY {
    ROM(rx) : ORIGIN = 0x00100000, LENGTH = 1M
    RAM(rwx) : ORIGIN = 0x40000000, LENGTH = 127M
}

SECTIONS
{
    .boot ALIGN(4K) : {
        *(.boot*)
    } > ROM

    .text ALIGN(4K) : {
        *(.text.itable*)
        *(.text*)
    } > ROM

    .rodata ALIGN(4K) : {
        *(.rodata*)
    } > ROM

    .bss ALIGN(4K) (NOLOAD) : AT(ADDR(.bss))
    {
        *(.bss.stack*)
        . = ALIGN(16);
        . += 16; /* avoiding overlap */
        *(.bss.kmalloc*)
        . = ALIGN(16);
        *(.bss.ttable*)
        *(.bss*)
    } > RAM = 0
}