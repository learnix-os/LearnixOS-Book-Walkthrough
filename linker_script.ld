OUTPUT_FORMAT(binary)
ENTRY(main)

SECTIONS {

    . = 0x7c00;

    .fill : {
        FILL(0)
        . = 0x7c00 + 510;
    }

    .magic_number : { SHORT(0xaa55) }
}