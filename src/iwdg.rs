#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub KR: KR,
    pub PR: PR,
    pub RLR: RLR,
    pub SR: SR,
    pub WINR: WINR,
    pub EWCR: EWCR,
}
impl RegisterBlock {
    #[doc = "0x00 - IWDG key register"]
    #[inline(always)]
    pub const fn KR(&self) -> &KR {
        &self.KR
    }
    #[doc = "0x04 - IWDG prescaler register"]
    #[inline(always)]
    pub const fn PR(&self) -> &PR {
        &self.PR
    }
    #[doc = "0x08 - IWDG reload register"]
    #[inline(always)]
    pub const fn RLR(&self) -> &RLR {
        &self.RLR
    }
    #[doc = "0x0c - IWDG status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x10 - IWDG window register"]
    #[inline(always)]
    pub const fn WINR(&self) -> &WINR {
        &self.WINR
    }
    #[doc = "0x14 - IWDG early wakeup interrupt register"]
    #[inline(always)]
    pub const fn EWCR(&self) -> &EWCR {
        &self.EWCR
    }
}
#[doc = "KR (w) register accessor: IWDG key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`] module"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "IWDG key register"]
pub mod kr;
#[doc = "PR (rw) register accessor: IWDG prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "IWDG prescaler register"]
pub mod pr;
#[doc = "RLR (rw) register accessor: IWDG reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`] module"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "IWDG reload register"]
pub mod rlr;
#[doc = "SR (r) register accessor: IWDG status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "IWDG status register"]
pub mod sr;
#[doc = "WINR (rw) register accessor: IWDG window register\n\nYou can [`read`](crate::Reg::read) this register and get [`winr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winr`] module"]
pub type WINR = crate::Reg<winr::WINR_SPEC>;
#[doc = "IWDG window register"]
pub mod winr;
#[doc = "EWCR (rw) register accessor: IWDG early wakeup interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewcr`] module"]
pub type EWCR = crate::Reg<ewcr::EWCR_SPEC>;
#[doc = "IWDG early wakeup interrupt register"]
pub mod ewcr;
