#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    pub TZSC_PRIVCFGR1: TZSC_PRIVCFGR1,
    pub TZSC_PRIVCFGR2: TZSC_PRIVCFGR2,
    pub TZSC_PRIVCFGR3: TZSC_PRIVCFGR3,
    _reserved3: [u8; 0x44],
    pub TZSC_MPCWM4ACFGR: TZSC_MPCWM4ACFGR,
    pub TZSC_MPCWM4AR: TZSC_MPCWM4AR,
    pub TZSC_MPCWM4BCFGR: TZSC_MPCWM4BCFGR,
    pub TZSC_MPCWM4BR: TZSC_MPCWM4BR,
    _reserved7: [u8; 0x0180],
    pub MPCBB1_PRIVCFGR: [MPCBB1_PRIVCFGR; 32],
    _reserved8: [u8; 0x0380],
    pub MPCBB2_PRIVCFGR: [MPCBB2_PRIVCFGR; 32],
}
impl RegisterBlock {
    #[doc = "0x20 - GTZC1 TZSC privilege configuration register 1"]
    #[inline(always)]
    pub const fn TZSC_PRIVCFGR1(&self) -> &TZSC_PRIVCFGR1 {
        &self.TZSC_PRIVCFGR1
    }
    #[doc = "0x24 - GTZC1 TZSC privilege configuration register 2"]
    #[inline(always)]
    pub const fn TZSC_PRIVCFGR2(&self) -> &TZSC_PRIVCFGR2 {
        &self.TZSC_PRIVCFGR2
    }
    #[doc = "0x28 - GTZC1 TZSC privilege configuration register 3"]
    #[inline(always)]
    pub const fn TZSC_PRIVCFGR3(&self) -> &TZSC_PRIVCFGR3 {
        &self.TZSC_PRIVCFGR3
    }
    #[doc = "0x70 - GTZC1 TZSC BKPSRAM sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn TZSC_MPCWM4ACFGR(&self) -> &TZSC_MPCWM4ACFGR {
        &self.TZSC_MPCWM4ACFGR
    }
    #[doc = "0x74 - GTZC1 TZSC BKPSRAM sub-region A watermark register"]
    #[inline(always)]
    pub const fn TZSC_MPCWM4AR(&self) -> &TZSC_MPCWM4AR {
        &self.TZSC_MPCWM4AR
    }
    #[doc = "0x78 - GTZC1 TZSC BKPSRAM sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn TZSC_MPCWM4BCFGR(&self) -> &TZSC_MPCWM4BCFGR {
        &self.TZSC_MPCWM4BCFGR
    }
    #[doc = "0x7c - GTZC1 TZSC BKPSRAM sub-region B watermark register"]
    #[inline(always)]
    pub const fn TZSC_MPCWM4BR(&self) -> &TZSC_MPCWM4BR {
        &self.TZSC_MPCWM4BR
    }
    #[doc = "0x200..0x280 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn MPCBB1_PRIVCFGR(&self, n: usize) -> &MPCBB1_PRIVCFGR {
        &self.MPCBB1_PRIVCFGR[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x280 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub fn MPCBB1_PRIVCFGR_iter(&self) -> impl Iterator<Item = &MPCBB1_PRIVCFGR> {
        self.MPCBB1_PRIVCFGR.iter()
    }
    #[doc = "0x600..0x680 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn MPCBB2_PRIVCFGR(&self, n: usize) -> &MPCBB2_PRIVCFGR {
        &self.MPCBB2_PRIVCFGR[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x680 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub fn MPCBB2_PRIVCFGR_iter(&self) -> impl Iterator<Item = &MPCBB2_PRIVCFGR> {
        self.MPCBB2_PRIVCFGR.iter()
    }
}
#[doc = "TZSC_PRIVCFGR1 (rw) register accessor: GTZC1 TZSC privilege configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr1`] module"]
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 1"]
pub mod tzsc_privcfgr1;
#[doc = "TZSC_PRIVCFGR2 (rw) register accessor: GTZC1 TZSC privilege configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr2`] module"]
pub type TZSC_PRIVCFGR2 = crate::Reg<tzsc_privcfgr2::TZSC_PRIVCFGR2_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 2"]
pub mod tzsc_privcfgr2;
#[doc = "TZSC_PRIVCFGR3 (rw) register accessor: GTZC1 TZSC privilege configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_privcfgr3`] module"]
pub type TZSC_PRIVCFGR3 = crate::Reg<tzsc_privcfgr3::TZSC_PRIVCFGR3_SPEC>;
#[doc = "GTZC1 TZSC privilege configuration register 3"]
pub mod tzsc_privcfgr3;
#[doc = "TZSC_MPCWM4ACFGR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region A watermark configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4acfgr`] module"]
pub type TZSC_MPCWM4ACFGR = crate::Reg<tzsc_mpcwm4acfgr::TZSC_MPCWM4ACFGR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark configuration register"]
pub mod tzsc_mpcwm4acfgr;
#[doc = "TZSC_MPCWM4AR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region A watermark register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4ar`] module"]
pub type TZSC_MPCWM4AR = crate::Reg<tzsc_mpcwm4ar::TZSC_MPCWM4AR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark register"]
pub mod tzsc_mpcwm4ar;
#[doc = "TZSC_MPCWM4BCFGR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region B watermark configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4bcfgr`] module"]
pub type TZSC_MPCWM4BCFGR = crate::Reg<tzsc_mpcwm4bcfgr::TZSC_MPCWM4BCFGR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register"]
pub mod tzsc_mpcwm4bcfgr;
#[doc = "TZSC_MPCWM4BR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region B watermark register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzsc_mpcwm4br`] module"]
pub type TZSC_MPCWM4BR = crate::Reg<tzsc_mpcwm4br::TZSC_MPCWM4BR_SPEC>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark register"]
pub mod tzsc_mpcwm4br;
#[doc = "MPCBB1_PRIVCFGR (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb1_privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb1_privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_privcfgr`] module"]
pub type MPCBB1_PRIVCFGR = crate::Reg<mpcbb1_privcfgr::MPCBB1_PRIVCFGR_SPEC>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
pub mod mpcbb1_privcfgr;
#[doc = "MPCBB2_PRIVCFGR (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb2_privcfgr`] module"]
pub type MPCBB2_PRIVCFGR = crate::Reg<mpcbb2_privcfgr::MPCBB2_PRIVCFGR_SPEC>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
pub mod mpcbb2_privcfgr;
