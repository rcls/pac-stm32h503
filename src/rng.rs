#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR: CR,
    pub SR: SR,
    pub DR: DR,
    _reserved3: [u8; 0x04],
    pub HTCR: HTCR,
}
impl RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x04 - RNG status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x08 - RNG data register"]
    #[inline(always)]
    pub const fn DR(&self) -> &DR {
        &self.DR
    }
    #[doc = "0x10 - RNG health test control register"]
    #[inline(always)]
    pub const fn HTCR(&self) -> &HTCR {
        &self.HTCR
    }
}
#[doc = "CR (rw) register accessor: RNG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RNG control register"]
pub mod cr;
#[doc = "SR (rw) register accessor: RNG status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "RNG status register"]
pub mod sr;
#[doc = "DR (r) register accessor: RNG data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "RNG data register"]
pub mod dr;
#[doc = "HTCR (rw) register accessor: RNG health test control register\n\nYou can [`read`](crate::Reg::read) this register and get [`htcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htcr`] module"]
pub type HTCR = crate::Reg<htcr::HTCR_SPEC>;
#[doc = "RNG health test control register"]
pub mod htcr;
