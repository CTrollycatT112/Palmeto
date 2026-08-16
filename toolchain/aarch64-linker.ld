OUTPUT_FORMAT(elf64-littleaarch64)
OUTPUT_ARCH(aarch64)

ENTRY(_start)

KERNEL_VMA = 0xffffffff80000000;

PHDRS
{
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)); /* Execute | Read */
    rodata  PT_LOAD    FLAGS((1 << 2));            /* Read Only */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)); /* Write | Read */
    dynamic PT_DYNAMIC FLAGS((1 << 1) | (1 << 2));
}

SECTIONS
{
    . = KERNEL_VMA;

    .text : {
        __start_text = .;
        *(.text .text.*)
        __stop_text = .;
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        __start_rodata = .;
        *(.rodata .rodata.*)
        __stop_rodata = .;
    } :rodata

    .requests_start_marker : {
        KEEP(*(.requests_start_marker))
    } :rodata

    .requests : {
        KEEP(*(.requests))
    } :rodata

    .requests_end_marker : {
        KEEP(*(.requests_end_marker))
    } :rodata

    .rela.dyn : {
        __rela_start = .;
        *(.rela.dyn)
        *(.rela.plt)
        __rela_end = .;
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        __start_data = .;
        *(.data .data.*)
        *(.got)
        *(.got.plt)
        __stop_data = .;
    } :data

    .dynamic : {
        *(.dynamic)
        . = ALIGN(16);
        QUAD(0); QUAD(0); 
    } :data :dynamic

    .bss : {
        __start_bss = .;
        *(COMMON)
        *(.bss .bss.*)
        __stop_bss = .;
    } :data

    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
        *(.comment)
    }
}
