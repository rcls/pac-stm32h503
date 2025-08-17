#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR1: CR1,
    pub CR2: CR2,
    pub CR3: CR3,
    pub FLTCR: FLTCR,
    pub ATCR1: ATCR1,
    pub ATSEEDR: ATSEEDR,
    pub ATOR: ATOR,
    pub ATCR2: ATCR2,
    pub CFGR: CFGR,
    pub PRIVCFGR: PRIVCFGR,
    _reserved10: [u8; 0x04],
    pub IER: IER,
    pub SR: SR,
    pub MISR: MISR,
    _reserved13: [u8; 0x04],
    pub SCR: SCR,
    pub COUNT1R: COUNT1R,
    _reserved15: [u8; 0x10],
    pub ERCFGR: ERCFGR,
    _reserved16: [u8; 0xa8],
    pub BKP0R: BKP0R,
    pub BKP1R: BKP1R,
    pub BKP2R: BKP2R,
    pub BKP3R: BKP3R,
    pub BKP4R: BKP4R,
    pub BKP5R: BKP5R,
    pub BKP6R: BKP6R,
    pub BKP7R: BKP7R,
    pub BKP8R: BKP8R,
    pub BKP9R: BKP9R,
    pub BKP10R: BKP10R,
    pub BKP11R: BKP11R,
    pub BKP12R: BKP12R,
    pub BKP13R: BKP13R,
    pub BKP14R: BKP14R,
    pub BKP15R: BKP15R,
    pub BKP16R: BKP16R,
    pub BKP17R: BKP17R,
    pub BKP18R: BKP18R,
    pub BKP19R: BKP19R,
    pub BKP20R: BKP20R,
    pub BKP21R: BKP21R,
    pub BKP22R: BKP22R,
    pub BKP23R: BKP23R,
    pub BKP24R: BKP24R,
    pub BKP25R: BKP25R,
    pub BKP26R: BKP26R,
    pub BKP27R: BKP27R,
    pub BKP28R: BKP28R,
    pub BKP29R: BKP29R,
    pub BKP30R: BKP30R,
    pub BKP31R: BKP31R,
}
impl RegisterBlock {
    #[doc = "0x00 - TAMP control register 1"]
    #[inline(always)]
    pub const fn CR1(&self) -> &CR1 {
        &self.CR1
    }
    #[doc = "0x04 - TAMP control register 2"]
    #[inline(always)]
    pub const fn CR2(&self) -> &CR2 {
        &self.CR2
    }
    #[doc = "0x08 - TAMP control register 3"]
    #[inline(always)]
    pub const fn CR3(&self) -> &CR3 {
        &self.CR3
    }
    #[doc = "0x0c - TAMP filter control register"]
    #[inline(always)]
    pub const fn FLTCR(&self) -> &FLTCR {
        &self.FLTCR
    }
    #[doc = "0x10 - TAMP active tamper control register 1"]
    #[inline(always)]
    pub const fn ATCR1(&self) -> &ATCR1 {
        &self.ATCR1
    }
    #[doc = "0x14 - TAMP active tamper seed register"]
    #[inline(always)]
    pub const fn ATSEEDR(&self) -> &ATSEEDR {
        &self.ATSEEDR
    }
    #[doc = "0x18 - TAMP active tamper output register"]
    #[inline(always)]
    pub const fn ATOR(&self) -> &ATOR {
        &self.ATOR
    }
    #[doc = "0x1c - TAMP active tamper control register 2"]
    #[inline(always)]
    pub const fn ATCR2(&self) -> &ATCR2 {
        &self.ATCR2
    }
    #[doc = "0x20 - TAMP configuration register"]
    #[inline(always)]
    pub const fn CFGR(&self) -> &CFGR {
        &self.CFGR
    }
    #[doc = "0x24 - TAMP privilege configuration register"]
    #[inline(always)]
    pub const fn PRIVCFGR(&self) -> &PRIVCFGR {
        &self.PRIVCFGR
    }
    #[doc = "0x2c - TAMP interrupt enable register"]
    #[inline(always)]
    pub const fn IER(&self) -> &IER {
        &self.IER
    }
    #[doc = "0x30 - TAMP status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x34 - TAMP masked interrupt status register"]
    #[inline(always)]
    pub const fn MISR(&self) -> &MISR {
        &self.MISR
    }
    #[doc = "0x3c - TAMP status clear register"]
    #[inline(always)]
    pub const fn SCR(&self) -> &SCR {
        &self.SCR
    }
    #[doc = "0x40 - TAMP monotonic counter 1 register"]
    #[inline(always)]
    pub const fn COUNT1R(&self) -> &COUNT1R {
        &self.COUNT1R
    }
    #[doc = "0x54 - TAMP erase configuration register"]
    #[inline(always)]
    pub const fn ERCFGR(&self) -> &ERCFGR {
        &self.ERCFGR
    }
    #[doc = "0x100 - TAMP backup 0 register"]
    #[inline(always)]
    pub const fn BKP0R(&self) -> &BKP0R {
        &self.BKP0R
    }
    #[doc = "0x104 - TAMP backup 1 register"]
    #[inline(always)]
    pub const fn BKP1R(&self) -> &BKP1R {
        &self.BKP1R
    }
    #[doc = "0x108 - TAMP backup 2 register"]
    #[inline(always)]
    pub const fn BKP2R(&self) -> &BKP2R {
        &self.BKP2R
    }
    #[doc = "0x10c - TAMP backup 3 register"]
    #[inline(always)]
    pub const fn BKP3R(&self) -> &BKP3R {
        &self.BKP3R
    }
    #[doc = "0x110 - TAMP backup 4 register"]
    #[inline(always)]
    pub const fn BKP4R(&self) -> &BKP4R {
        &self.BKP4R
    }
    #[doc = "0x114 - TAMP backup 5 register"]
    #[inline(always)]
    pub const fn BKP5R(&self) -> &BKP5R {
        &self.BKP5R
    }
    #[doc = "0x118 - TAMP backup 6 register"]
    #[inline(always)]
    pub const fn BKP6R(&self) -> &BKP6R {
        &self.BKP6R
    }
    #[doc = "0x11c - TAMP backup 7 register"]
    #[inline(always)]
    pub const fn BKP7R(&self) -> &BKP7R {
        &self.BKP7R
    }
    #[doc = "0x120 - TAMP backup 8 register"]
    #[inline(always)]
    pub const fn BKP8R(&self) -> &BKP8R {
        &self.BKP8R
    }
    #[doc = "0x124 - TAMP backup 9 register"]
    #[inline(always)]
    pub const fn BKP9R(&self) -> &BKP9R {
        &self.BKP9R
    }
    #[doc = "0x128 - TAMP backup 10 register"]
    #[inline(always)]
    pub const fn BKP10R(&self) -> &BKP10R {
        &self.BKP10R
    }
    #[doc = "0x12c - TAMP backup 11 register"]
    #[inline(always)]
    pub const fn BKP11R(&self) -> &BKP11R {
        &self.BKP11R
    }
    #[doc = "0x130 - TAMP backup 12 register"]
    #[inline(always)]
    pub const fn BKP12R(&self) -> &BKP12R {
        &self.BKP12R
    }
    #[doc = "0x134 - TAMP backup 13 register"]
    #[inline(always)]
    pub const fn BKP13R(&self) -> &BKP13R {
        &self.BKP13R
    }
    #[doc = "0x138 - TAMP backup 14 register"]
    #[inline(always)]
    pub const fn BKP14R(&self) -> &BKP14R {
        &self.BKP14R
    }
    #[doc = "0x13c - TAMP backup 15 register"]
    #[inline(always)]
    pub const fn BKP15R(&self) -> &BKP15R {
        &self.BKP15R
    }
    #[doc = "0x140 - TAMP backup 16 register"]
    #[inline(always)]
    pub const fn BKP16R(&self) -> &BKP16R {
        &self.BKP16R
    }
    #[doc = "0x144 - TAMP backup 17 register"]
    #[inline(always)]
    pub const fn BKP17R(&self) -> &BKP17R {
        &self.BKP17R
    }
    #[doc = "0x148 - TAMP backup 18 register"]
    #[inline(always)]
    pub const fn BKP18R(&self) -> &BKP18R {
        &self.BKP18R
    }
    #[doc = "0x14c - TAMP backup 19 register"]
    #[inline(always)]
    pub const fn BKP19R(&self) -> &BKP19R {
        &self.BKP19R
    }
    #[doc = "0x150 - TAMP backup 20 register"]
    #[inline(always)]
    pub const fn BKP20R(&self) -> &BKP20R {
        &self.BKP20R
    }
    #[doc = "0x154 - TAMP backup 21 register"]
    #[inline(always)]
    pub const fn BKP21R(&self) -> &BKP21R {
        &self.BKP21R
    }
    #[doc = "0x158 - TAMP backup 22 register"]
    #[inline(always)]
    pub const fn BKP22R(&self) -> &BKP22R {
        &self.BKP22R
    }
    #[doc = "0x15c - TAMP backup 23 register"]
    #[inline(always)]
    pub const fn BKP23R(&self) -> &BKP23R {
        &self.BKP23R
    }
    #[doc = "0x160 - TAMP backup 24 register"]
    #[inline(always)]
    pub const fn BKP24R(&self) -> &BKP24R {
        &self.BKP24R
    }
    #[doc = "0x164 - TAMP backup 25 register"]
    #[inline(always)]
    pub const fn BKP25R(&self) -> &BKP25R {
        &self.BKP25R
    }
    #[doc = "0x168 - TAMP backup 26 register"]
    #[inline(always)]
    pub const fn BKP26R(&self) -> &BKP26R {
        &self.BKP26R
    }
    #[doc = "0x16c - TAMP backup 27 register"]
    #[inline(always)]
    pub const fn BKP27R(&self) -> &BKP27R {
        &self.BKP27R
    }
    #[doc = "0x170 - TAMP backup 28 register"]
    #[inline(always)]
    pub const fn BKP28R(&self) -> &BKP28R {
        &self.BKP28R
    }
    #[doc = "0x174 - TAMP backup 29 register"]
    #[inline(always)]
    pub const fn BKP29R(&self) -> &BKP29R {
        &self.BKP29R
    }
    #[doc = "0x178 - TAMP backup 30 register"]
    #[inline(always)]
    pub const fn BKP30R(&self) -> &BKP30R {
        &self.BKP30R
    }
    #[doc = "0x17c - TAMP backup 31 register"]
    #[inline(always)]
    pub const fn BKP31R(&self) -> &BKP31R {
        &self.BKP31R
    }
}
#[doc = "CR1 (rw) register accessor: TAMP control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TAMP control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: TAMP control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "TAMP control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: TAMP control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "TAMP control register 3"]
pub mod cr3;
#[doc = "FLTCR (rw) register accessor: TAMP filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltcr`] module"]
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
#[doc = "TAMP filter control register"]
pub mod fltcr;
#[doc = "ATCR1 (rw) register accessor: TAMP active tamper control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`atcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atcr1`] module"]
pub type ATCR1 = crate::Reg<atcr1::ATCR1_SPEC>;
#[doc = "TAMP active tamper control register 1"]
pub mod atcr1;
#[doc = "ATSEEDR (w) register accessor: TAMP active tamper seed register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atseedr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atseedr`] module"]
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDR_SPEC>;
#[doc = "TAMP active tamper seed register"]
pub mod atseedr;
#[doc = "ATOR (r) register accessor: TAMP active tamper output register\n\nYou can [`read`](crate::Reg::read) this register and get [`ator::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ator`] module"]
pub type ATOR = crate::Reg<ator::ATOR_SPEC>;
#[doc = "TAMP active tamper output register"]
pub mod ator;
#[doc = "ATCR2 (rw) register accessor: TAMP active tamper control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`atcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atcr2`] module"]
pub type ATCR2 = crate::Reg<atcr2::ATCR2_SPEC>;
#[doc = "TAMP active tamper control register 2"]
pub mod atcr2;
#[doc = "CFGR (rw) register accessor: TAMP configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "TAMP configuration register"]
pub mod cfgr;
#[doc = "PRIVCFGR (rw) register accessor: TAMP privilege configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`] module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "TAMP privilege configuration register"]
pub mod privcfgr;
#[doc = "IER (rw) register accessor: TAMP interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "TAMP interrupt enable register"]
pub mod ier;
#[doc = "SR (rw) register accessor: TAMP status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TAMP status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: TAMP masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`] module"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "TAMP masked interrupt status register"]
pub mod misr;
#[doc = "SCR (w) register accessor: TAMP status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "TAMP status clear register"]
pub mod scr;
#[doc = "COUNT1R (r) register accessor: TAMP monotonic counter 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`count1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count1r`] module"]
pub type COUNT1R = crate::Reg<count1r::COUNT1R_SPEC>;
#[doc = "TAMP monotonic counter 1 register"]
pub mod count1r;
#[doc = "ERCFGR (rw) register accessor: TAMP erase configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ercfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ercfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ercfgr`] module"]
pub type ERCFGR = crate::Reg<ercfgr::ERCFGR_SPEC>;
#[doc = "TAMP erase configuration register"]
pub mod ercfgr;
#[doc = "BKP0R (rw) register accessor: TAMP backup 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp0r`] module"]
pub type BKP0R = crate::Reg<bkp0r::BKP0R_SPEC>;
#[doc = "TAMP backup 0 register"]
pub mod bkp0r;
#[doc = "BKP1R (rw) register accessor: TAMP backup 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp1r`] module"]
pub type BKP1R = crate::Reg<bkp1r::BKP1R_SPEC>;
#[doc = "TAMP backup 1 register"]
pub mod bkp1r;
#[doc = "BKP2R (rw) register accessor: TAMP backup 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp2r`] module"]
pub type BKP2R = crate::Reg<bkp2r::BKP2R_SPEC>;
#[doc = "TAMP backup 2 register"]
pub mod bkp2r;
#[doc = "BKP3R (rw) register accessor: TAMP backup 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp3r`] module"]
pub type BKP3R = crate::Reg<bkp3r::BKP3R_SPEC>;
#[doc = "TAMP backup 3 register"]
pub mod bkp3r;
#[doc = "BKP4R (rw) register accessor: TAMP backup 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp4r`] module"]
pub type BKP4R = crate::Reg<bkp4r::BKP4R_SPEC>;
#[doc = "TAMP backup 4 register"]
pub mod bkp4r;
#[doc = "BKP5R (rw) register accessor: TAMP backup 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp5r`] module"]
pub type BKP5R = crate::Reg<bkp5r::BKP5R_SPEC>;
#[doc = "TAMP backup 5 register"]
pub mod bkp5r;
#[doc = "BKP6R (rw) register accessor: TAMP backup 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp6r`] module"]
pub type BKP6R = crate::Reg<bkp6r::BKP6R_SPEC>;
#[doc = "TAMP backup 6 register"]
pub mod bkp6r;
#[doc = "BKP7R (rw) register accessor: TAMP backup 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp7r`] module"]
pub type BKP7R = crate::Reg<bkp7r::BKP7R_SPEC>;
#[doc = "TAMP backup 7 register"]
pub mod bkp7r;
#[doc = "BKP8R (rw) register accessor: TAMP backup 8 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp8r`] module"]
pub type BKP8R = crate::Reg<bkp8r::BKP8R_SPEC>;
#[doc = "TAMP backup 8 register"]
pub mod bkp8r;
#[doc = "BKP9R (rw) register accessor: TAMP backup 9 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp9r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp9r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp9r`] module"]
pub type BKP9R = crate::Reg<bkp9r::BKP9R_SPEC>;
#[doc = "TAMP backup 9 register"]
pub mod bkp9r;
#[doc = "BKP10R (rw) register accessor: TAMP backup 10 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp10r`] module"]
pub type BKP10R = crate::Reg<bkp10r::BKP10R_SPEC>;
#[doc = "TAMP backup 10 register"]
pub mod bkp10r;
#[doc = "BKP11R (rw) register accessor: TAMP backup 11 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp11r`] module"]
pub type BKP11R = crate::Reg<bkp11r::BKP11R_SPEC>;
#[doc = "TAMP backup 11 register"]
pub mod bkp11r;
#[doc = "BKP12R (rw) register accessor: TAMP backup 12 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp12r`] module"]
pub type BKP12R = crate::Reg<bkp12r::BKP12R_SPEC>;
#[doc = "TAMP backup 12 register"]
pub mod bkp12r;
#[doc = "BKP13R (rw) register accessor: TAMP backup 13 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp13r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp13r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp13r`] module"]
pub type BKP13R = crate::Reg<bkp13r::BKP13R_SPEC>;
#[doc = "TAMP backup 13 register"]
pub mod bkp13r;
#[doc = "BKP14R (rw) register accessor: TAMP backup 14 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp14r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp14r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp14r`] module"]
pub type BKP14R = crate::Reg<bkp14r::BKP14R_SPEC>;
#[doc = "TAMP backup 14 register"]
pub mod bkp14r;
#[doc = "BKP15R (rw) register accessor: TAMP backup 15 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp15r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp15r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp15r`] module"]
pub type BKP15R = crate::Reg<bkp15r::BKP15R_SPEC>;
#[doc = "TAMP backup 15 register"]
pub mod bkp15r;
#[doc = "BKP16R (rw) register accessor: TAMP backup 16 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp16r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp16r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp16r`] module"]
pub type BKP16R = crate::Reg<bkp16r::BKP16R_SPEC>;
#[doc = "TAMP backup 16 register"]
pub mod bkp16r;
#[doc = "BKP17R (rw) register accessor: TAMP backup 17 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp17r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp17r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp17r`] module"]
pub type BKP17R = crate::Reg<bkp17r::BKP17R_SPEC>;
#[doc = "TAMP backup 17 register"]
pub mod bkp17r;
#[doc = "BKP18R (rw) register accessor: TAMP backup 18 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp18r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp18r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp18r`] module"]
pub type BKP18R = crate::Reg<bkp18r::BKP18R_SPEC>;
#[doc = "TAMP backup 18 register"]
pub mod bkp18r;
#[doc = "BKP19R (rw) register accessor: TAMP backup 19 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp19r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp19r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp19r`] module"]
pub type BKP19R = crate::Reg<bkp19r::BKP19R_SPEC>;
#[doc = "TAMP backup 19 register"]
pub mod bkp19r;
#[doc = "BKP20R (rw) register accessor: TAMP backup 20 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp20r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp20r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp20r`] module"]
pub type BKP20R = crate::Reg<bkp20r::BKP20R_SPEC>;
#[doc = "TAMP backup 20 register"]
pub mod bkp20r;
#[doc = "BKP21R (rw) register accessor: TAMP backup 21 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp21r`] module"]
pub type BKP21R = crate::Reg<bkp21r::BKP21R_SPEC>;
#[doc = "TAMP backup 21 register"]
pub mod bkp21r;
#[doc = "BKP22R (rw) register accessor: TAMP backup 22 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp22r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp22r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp22r`] module"]
pub type BKP22R = crate::Reg<bkp22r::BKP22R_SPEC>;
#[doc = "TAMP backup 22 register"]
pub mod bkp22r;
#[doc = "BKP23R (rw) register accessor: TAMP backup 23 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp23r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp23r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp23r`] module"]
pub type BKP23R = crate::Reg<bkp23r::BKP23R_SPEC>;
#[doc = "TAMP backup 23 register"]
pub mod bkp23r;
#[doc = "BKP24R (rw) register accessor: TAMP backup 24 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp24r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp24r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp24r`] module"]
pub type BKP24R = crate::Reg<bkp24r::BKP24R_SPEC>;
#[doc = "TAMP backup 24 register"]
pub mod bkp24r;
#[doc = "BKP25R (rw) register accessor: TAMP backup 25 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp25r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp25r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp25r`] module"]
pub type BKP25R = crate::Reg<bkp25r::BKP25R_SPEC>;
#[doc = "TAMP backup 25 register"]
pub mod bkp25r;
#[doc = "BKP26R (rw) register accessor: TAMP backup 26 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp26r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp26r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp26r`] module"]
pub type BKP26R = crate::Reg<bkp26r::BKP26R_SPEC>;
#[doc = "TAMP backup 26 register"]
pub mod bkp26r;
#[doc = "BKP27R (rw) register accessor: TAMP backup 27 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp27r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp27r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp27r`] module"]
pub type BKP27R = crate::Reg<bkp27r::BKP27R_SPEC>;
#[doc = "TAMP backup 27 register"]
pub mod bkp27r;
#[doc = "BKP28R (rw) register accessor: TAMP backup 28 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp28r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp28r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp28r`] module"]
pub type BKP28R = crate::Reg<bkp28r::BKP28R_SPEC>;
#[doc = "TAMP backup 28 register"]
pub mod bkp28r;
#[doc = "BKP29R (rw) register accessor: TAMP backup 29 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp29r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp29r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp29r`] module"]
pub type BKP29R = crate::Reg<bkp29r::BKP29R_SPEC>;
#[doc = "TAMP backup 29 register"]
pub mod bkp29r;
#[doc = "BKP30R (rw) register accessor: TAMP backup 30 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp30r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp30r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp30r`] module"]
pub type BKP30R = crate::Reg<bkp30r::BKP30R_SPEC>;
#[doc = "TAMP backup 30 register"]
pub mod bkp30r;
#[doc = "BKP31R (rw) register accessor: TAMP backup 31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp31r`] module"]
pub type BKP31R = crate::Reg<bkp31r::BKP31R_SPEC>;
#[doc = "TAMP backup 31 register"]
pub mod bkp31r;
