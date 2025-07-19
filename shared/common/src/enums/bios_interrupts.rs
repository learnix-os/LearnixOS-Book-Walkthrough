#[repr(u8)]
pub enum BiosInterrupts {
    DISK = 0x13,
}

#[repr(u8)]
pub enum Disk {
    ExtendedRead = 0x42,
}
