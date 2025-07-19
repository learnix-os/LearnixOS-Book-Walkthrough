// The `repr(C)` means that the order of the structure fields will be as specified
// Because rust ABI doesn't state that this is promised.
// The `repr(Packed) states that there will no padding due to alignment in this struct
#[repr(C, packed)]
pub struct DiskAddressPacket {
    /// The size of the packet
    packet_size: u8,

    /// Zero
    zero: u8,

    /// How many sectors to read
    num_of_sectors: u16,

    /// Which address in memory to save the data
    memory_address: u16,

    /// Memory segment for the address
    segment: u16,

    /// The LBA address of the first sector
    abs_block_num: u64,
}
