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
    } > RAM AT> ROM

    .rodata ALIGN(4K) : {
        *(.rodata*)
    } > RAM AT> ROM

    ROM_TEXT_START = LOADADDR(.text);
    ROM_TEXT_SIZE = SIZEOF(.text);
    RAM_TEXT_START = 0x40000000;

    . = RAM_TEXT_START + SIZEOF(.text);
    . = ALIGN(4K);
    ROM_RODATA_START = LOADADDR(.rodata);
    ROM_RODATA_SIZE = SIZEOF(.rodata);
    RAM_RODATA_START = .;

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