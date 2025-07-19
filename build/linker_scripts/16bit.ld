OUTPUT_FORMAT(binary)
ENTRY(start)

SECTIONS {

    . = 0x7c00;
        
    .boot : { *(.boot) }
    .text : { *(.text .text.*) }
    .bss : { *(.bss .bss.*) }
    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    /DISCARD/ : {
        *(.eh_frame .eh_frame.*)
        *(.eh_frame_hdr .eh_frame_hdr.*) 
    }

    . = 0x7c00 + 510;
    
    .magic_number : { SHORT(0xaa55) }
}