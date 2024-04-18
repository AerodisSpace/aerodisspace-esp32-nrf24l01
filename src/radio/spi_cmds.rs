#![allow(unused)]
pub struct CommandNRF24SPI {}

/// Commands for the NRF24L01+ SPI interface
impl CommandNRF24SPI {
    /// 0000 AAAA Read command and status registers
    pub const R_REGISTER: u8 = 0x00;
    /// 0010 AAAA Write command and status registers
    pub const W_REGISTER: u8 = 0x20;
    /// 0110 0001 Read RX payload
    pub const R_RX_PAYLOAD: u8 = 0x61;
    /// 1010 0000 Write TX payload
    pub const W_TX_PAYLOAD: u8 = 0xA0;
    /// 1110 0001 Flush TX FIFO
    pub const FLUSH_TX: u8 = 0xE1;
    /// 1110 0010 Flush RX FIFO
    pub const FLUSH_RX: u8 = 0xE2;
    /// 1110 0011 Reuse last transmitted payload
    pub const REUSE_TX_PL: u8 = 0xE3;
    /// 0110 0000 Read RX payload width
    pub const R_RX_PL_WID: u8 = 0x60;
    /// 1010 1PPP Write Payload to be transmitted together with ACK packet
    pub const W_ACK_PAYLOAD: u8 = 0xA8;
    /// 1011 0000 Disable AUTOACK on this specific packet
    pub const W_TX_PAYLOAD_NOACK: u8 = 0xB0;
    /// 1111 1111 No Operation, might be used to read the STATUS register
    pub const NOP: u8 = 0xFF;
}
