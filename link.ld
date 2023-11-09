ENTRY(boot)

PHDRS {
    text PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

SECTIONS
{
    . = 0x00100000;

    .boot ALIGN(4K) : AT(ADDR(.boot))
    {
        *(.boot*)
    } :text

    .text ALIGN(4K) : AT(ADDR(.text))
    {
        *(.text.itable*)
        *(.text*)
    } :text

    .rodata ALIGN(4K) : AT(ADDR(.rodata)) {
        *(.rodata*)
    } :data

    ROM_TEXT_START = ADDR(.text);
    ROM_TEXT_END = .;

    . = 0x40000000;
    RAM_TEXT_START = .;
    . += ROM_TEXT_END - ROM_TEXT_START;

    .bss ALIGN(4K) (NOLOAD) : AT(ADDR(.bss))
    {
        *(.bss.stack*)
        . = ALIGN(16);
        . += 16; /* avoiding overlap */
        *(.bss.kmalloc*)
        . = ALIGN(16);
        *(.bss.ttable*)
    } :bss = 0
}