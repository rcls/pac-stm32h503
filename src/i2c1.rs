#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR1: CR1,
    pub CR2: CR2,
    pub OAR1: OAR1,
    pub OAR2: OAR2,
    pub TIMINGR: TIMINGR,
    pub TIMEOUTR: TIMEOUTR,
    pub ISR: ISR,
    pub ICR: ICR,
    pub PECR: PECR,
    pub RXDR: RXDR,
    pub TXDR: TXDR,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C control register 1"]
    #[inline(always)]
    pub const fn CR1(&self) -> &CR1 {
        &self.CR1
    }
    #[doc = "0x04 - I2C control register 2"]
    #[inline(always)]
    pub const fn CR2(&self) -> &CR2 {
        &self.CR2
    }
    #[doc = "0x08 - I2C own address 1 register"]
    #[inline(always)]
    pub const fn OAR1(&self) -> &OAR1 {
        &self.OAR1
    }
    #[doc = "0x0c - I2C own address 2 register"]
    #[inline(always)]
    pub const fn OAR2(&self) -> &OAR2 {
        &self.OAR2
    }
    #[doc = "0x10 - I2C timing register"]
    #[inline(always)]
    pub const fn TIMINGR(&self) -> &TIMINGR {
        &self.TIMINGR
    }
    #[doc = "0x14 - I2C timeout register"]
    #[inline(always)]
    pub const fn TIMEOUTR(&self) -> &TIMEOUTR {
        &self.TIMEOUTR
    }
    #[doc = "0x18 - I2C interrupt and status register"]
    #[inline(always)]
    pub const fn ISR(&self) -> &ISR {
        &self.ISR
    }
    #[doc = "0x1c - I2C interrupt clear register"]
    #[inline(always)]
    pub const fn ICR(&self) -> &ICR {
        &self.ICR
    }
    #[doc = "0x20 - I2C PEC register"]
    #[inline(always)]
    pub const fn PECR(&self) -> &PECR {
        &self.PECR
    }
    #[doc = "0x24 - I2C receive data register"]
    #[inline(always)]
    pub const fn RXDR(&self) -> &RXDR {
        &self.RXDR
    }
    #[doc = "0x28 - I2C transmit data register"]
    #[inline(always)]
    pub const fn TXDR(&self) -> &TXDR {
        &self.TXDR
    }
}
#[doc = "CR1 (rw) register accessor: I2C control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "I2C control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: I2C control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "I2C control register 2"]
pub mod cr2;
#[doc = "OAR1 (rw) register accessor: I2C own address 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar1`] module"]
pub type OAR1 = crate::Reg<oar1::OAR1_SPEC>;
#[doc = "I2C own address 1 register"]
pub mod oar1;
#[doc = "OAR2 (rw) register accessor: I2C own address 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar2`] module"]
pub type OAR2 = crate::Reg<oar2::OAR2_SPEC>;
#[doc = "I2C own address 2 register"]
pub mod oar2;
#[doc = "TIMINGR (rw) register accessor: I2C timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr`] module"]
pub type TIMINGR = crate::Reg<timingr::TIMINGR_SPEC>;
#[doc = "I2C timing register"]
pub mod timingr;
#[doc = "TIMEOUTR (rw) register accessor: I2C timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeoutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeoutr`] module"]
pub type TIMEOUTR = crate::Reg<timeoutr::TIMEOUTR_SPEC>;
#[doc = "I2C timeout register"]
pub mod timeoutr;
#[doc = "ISR (rw) register accessor: I2C interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "I2C interrupt and status register"]
pub mod isr;
#[doc = "ICR (w) register accessor: I2C interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "I2C interrupt clear register"]
pub mod icr;
#[doc = "PECR (r) register accessor: I2C PEC register\n\nYou can [`read`](crate::Reg::read) this register and get [`pecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pecr`] module"]
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
#[doc = "I2C PEC register"]
pub mod pecr;
#[doc = "RXDR (r) register accessor: I2C receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`] module"]
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
#[doc = "I2C receive data register"]
pub mod rxdr;
#[doc = "TXDR (rw) register accessor: I2C transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`] module"]
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
#[doc = "I2C transmit data register"]
pub mod txdr;
