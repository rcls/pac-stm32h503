#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR1: CR1,
    pub CR2: CR2,
    pub CFG1: CFG1,
    pub CFG2: CFG2,
    pub IER: IER,
    pub SR: SR,
    pub IFCR: IFCR,
    _reserved7: [u8; 0x04],
    pub TXDR: TXDR,
    _reserved8: [u8; 0x0c],
    pub RXDR: RXDR,
    _reserved9: [u8; 0x0c],
    pub CRCPOLY: CRCPOLY,
    pub TXCRC: TXCRC,
    pub RXCRC: RXCRC,
    pub UDRDR: UDRDR,
    pub I2SCFGR: I2SCFGR,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI/I2S control register 1"]
    #[inline(always)]
    pub const fn CR1(&self) -> &CR1 {
        &self.CR1
    }
    #[doc = "0x04 - SPI/I2S control register 2"]
    #[inline(always)]
    pub const fn CR2(&self) -> &CR2 {
        &self.CR2
    }
    #[doc = "0x08 - SPI/I2S configuration register 1"]
    #[inline(always)]
    pub const fn CFG1(&self) -> &CFG1 {
        &self.CFG1
    }
    #[doc = "0x0c - SPI/I2S configuration register 2"]
    #[inline(always)]
    pub const fn CFG2(&self) -> &CFG2 {
        &self.CFG2
    }
    #[doc = "0x10 - SPI/I2S interrupt enable register"]
    #[inline(always)]
    pub const fn IER(&self) -> &IER {
        &self.IER
    }
    #[doc = "0x14 - SPI/I2S status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x18 - SPI/I2S interrupt/status flags clear register"]
    #[inline(always)]
    pub const fn IFCR(&self) -> &IFCR {
        &self.IFCR
    }
    #[doc = "0x20 - SPI/I2S transmit data register"]
    #[inline(always)]
    pub const fn TXDR(&self) -> &TXDR {
        &self.TXDR
    }
    #[doc = "0x30 - SPI/I2S receive data register"]
    #[inline(always)]
    pub const fn RXDR(&self) -> &RXDR {
        &self.RXDR
    }
    #[doc = "0x40 - SPI/I2S polynomial register"]
    #[inline(always)]
    pub const fn CRCPOLY(&self) -> &CRCPOLY {
        &self.CRCPOLY
    }
    #[doc = "0x44 - SPI/I2S transmitter CRC register"]
    #[inline(always)]
    pub const fn TXCRC(&self) -> &TXCRC {
        &self.TXCRC
    }
    #[doc = "0x48 - SPI/I2S receiver CRC register"]
    #[inline(always)]
    pub const fn RXCRC(&self) -> &RXCRC {
        &self.RXCRC
    }
    #[doc = "0x4c - SPI/I2S underrun data register"]
    #[inline(always)]
    pub const fn UDRDR(&self) -> &UDRDR {
        &self.UDRDR
    }
    #[doc = "0x50 - SPI/I2S configuration register"]
    #[inline(always)]
    pub const fn I2SCFGR(&self) -> &I2SCFGR {
        &self.I2SCFGR
    }
}
#[doc = "CR1 (rw) register accessor: SPI/I2S control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "SPI/I2S control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: SPI/I2S control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "SPI/I2S control register 2"]
pub mod cr2;
#[doc = "CFG1 (rw) register accessor: SPI/I2S configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "SPI/I2S configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: SPI/I2S configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`] module"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "SPI/I2S configuration register 2"]
pub mod cfg2;
#[doc = "IER (rw) register accessor: SPI/I2S interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "SPI/I2S interrupt enable register"]
pub mod ier;
#[doc = "SR (r) register accessor: SPI/I2S status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SPI/I2S status register"]
pub mod sr;
#[doc = "IFCR (w) register accessor: SPI/I2S interrupt/status flags clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "SPI/I2S interrupt/status flags clear register"]
pub mod ifcr;
#[doc = "TXDR (w) register accessor: SPI/I2S transmit data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`] module"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "SPI/I2S transmit data register"]
pub mod txdr;
#[doc = "RXDR (r) register accessor: SPI/I2S receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`] module"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "SPI/I2S receive data register"]
pub mod rxdr;
#[doc = "CRCPOLY (rw) register accessor: SPI/I2S polynomial register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`] module"]
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
#[doc = "SPI/I2S polynomial register"]
pub mod crcpoly;
#[doc = "TXCRC (r) register accessor: SPI/I2S transmitter CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`txcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcrc`] module"]
pub type TXCRC = crate::Reg<txcrc::TXCRC_SPEC>;
#[doc = "SPI/I2S transmitter CRC register"]
pub mod txcrc;
#[doc = "RXCRC (r) register accessor: SPI/I2S receiver CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrc`] module"]
pub type RXCRC = crate::Reg<rxcrc::RXCRC_SPEC>;
#[doc = "SPI/I2S receiver CRC register"]
pub mod rxcrc;
#[doc = "UDRDR (rw) register accessor: SPI/I2S underrun data register\n\nYou can [`read`](crate::Reg::read) this register and get [`udrdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udrdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udrdr`] module"]
pub type UDRDR = crate::Reg<udrdr::UDRDR_SPEC>;
#[doc = "SPI/I2S underrun data register"]
pub mod udrdr;
#[doc = "I2SCFGR (rw) register accessor: SPI/I2S configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2scfgr`] module"]
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
#[doc = "SPI/I2S configuration register"]
pub mod i2scfgr;
