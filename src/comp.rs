#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub SR: SR,
    pub ICFR: ICFR,
    _reserved2: [u8; 0x04],
    pub CFGR1: CFGR1,
    pub CFGR2: CFGR2,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x04 - Comparator interrupt clear flag register"]
    #[inline(always)]
    pub const fn ICFR(&self) -> &ICFR {
        &self.ICFR
    }
    #[doc = "0x0c - Comparator configuration register 1"]
    #[inline(always)]
    pub const fn CFGR1(&self) -> &CFGR1 {
        &self.CFGR1
    }
    #[doc = "0x10 - Comparator configuration register 2"]
    #[inline(always)]
    pub const fn CFGR2(&self) -> &CFGR2 {
        &self.CFGR2
    }
}
#[doc = "SR (r) register accessor: Comparator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Comparator status register"]
pub mod sr;
#[doc = "ICFR (rw) register accessor: Comparator interrupt clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`icfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icfr`] module"]
pub type ICFR = crate::Reg<icfr::ICFR_SPEC>;
#[doc = "Comparator interrupt clear flag register"]
pub mod icfr;
#[doc = "CFGR1 (rw) register accessor: Comparator configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`] module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "Comparator configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: Comparator configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "Comparator configuration register 2"]
pub mod cfgr2;
