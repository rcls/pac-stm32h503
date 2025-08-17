#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub DR: DR,
    pub IDR: IDR,
    pub CR: CR,
    _reserved3: [u8; 0x04],
    pub INIT: INIT,
    pub POL: POL,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC data register"]
    #[inline(always)]
    pub const fn DR(&self) -> &DR {
        &self.DR
    }
    #[doc = "0x04 - CRC independent data register"]
    #[inline(always)]
    pub const fn IDR(&self) -> &IDR {
        &self.IDR
    }
    #[doc = "0x08 - CRC control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x10 - CRC initial value"]
    #[inline(always)]
    pub const fn INIT(&self) -> &INIT {
        &self.INIT
    }
    #[doc = "0x14 - CRC polynomial"]
    #[inline(always)]
    pub const fn POL(&self) -> &POL {
        &self.POL
    }
}
#[doc = "DR (rw) register accessor: CRC data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "CRC data register"]
pub mod dr;
#[doc = "IDR (rw) register accessor: CRC independent data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "CRC independent data register"]
pub mod idr;
#[doc = "CR (rw) register accessor: CRC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CRC control register"]
pub mod cr;
#[doc = "INIT (rw) register accessor: CRC initial value\n\nYou can [`read`](crate::Reg::read) this register and get [`init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@init`] module"]
pub type INIT = crate::Reg<init::INIT_SPEC>;
#[doc = "CRC initial value"]
pub mod init;
#[doc = "POL (rw) register accessor: CRC polynomial\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pol`] module"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "CRC polynomial"]
pub mod pol;
