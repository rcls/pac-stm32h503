#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR: CR,
    pub CFGR: CFGR,
    pub ISR: ISR,
    pub ICR: ICR,
}
impl RegisterBlock {
    #[doc = "0x00 - CRS control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x04 - CRS configuration register"]
    #[inline(always)]
    pub const fn CFGR(&self) -> &CFGR {
        &self.CFGR
    }
    #[doc = "0x08 - CRS interrupt and status register"]
    #[inline(always)]
    pub const fn ISR(&self) -> &ISR {
        &self.ISR
    }
    #[doc = "0x0c - CRS interrupt flag clear register"]
    #[inline(always)]
    pub const fn ICR(&self) -> &ICR {
        &self.ICR
    }
}
#[doc = "CR (rw) register accessor: CRS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CRS control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: CRS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "CRS configuration register"]
pub mod cfgr;
#[doc = "ISR (r) register accessor: CRS interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "CRS interrupt and status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: CRS interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "CRS interrupt flag clear register"]
pub mod icr;
