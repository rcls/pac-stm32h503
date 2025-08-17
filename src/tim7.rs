#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR1: CR1,
    _reserved1: [u8; 0x02],
    pub CR2: CR2,
    _reserved2: [u8; 0x06],
    pub DIER: DIER,
    _reserved3: [u8; 0x02],
    pub SR: SR,
    _reserved4: [u8; 0x02],
    pub EGR: EGR,
    _reserved5: [u8; 0x0e],
    pub CNT: CNT,
    pub PSC: PSC,
    _reserved7: [u8; 0x02],
    pub ARR: ARR,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM7 control register 1"]
    #[inline(always)]
    pub const fn CR1(&self) -> &CR1 {
        &self.CR1
    }
    #[doc = "0x04 - TIM7 control register 2"]
    #[inline(always)]
    pub const fn CR2(&self) -> &CR2 {
        &self.CR2
    }
    #[doc = "0x0c - TIM7 DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn DIER(&self) -> &DIER {
        &self.DIER
    }
    #[doc = "0x10 - TIM7 status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x14 - TIM7 event generation register"]
    #[inline(always)]
    pub const fn EGR(&self) -> &EGR {
        &self.EGR
    }
    #[doc = "0x24 - TIM7 counter"]
    #[inline(always)]
    pub const fn CNT(&self) -> &CNT {
        &self.CNT
    }
    #[doc = "0x28 - TIM7 prescaler"]
    #[inline(always)]
    pub const fn PSC(&self) -> &PSC {
        &self.PSC
    }
    #[doc = "0x2c - TIM7 auto-reload register"]
    #[inline(always)]
    pub const fn ARR(&self) -> &ARR {
        &self.ARR
    }
}
#[doc = "CR1 (rw) register accessor: TIM7 control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM7 control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: TIM7 control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "TIM7 control register 2"]
pub mod cr2;
#[doc = "DIER (rw) register accessor: TIM7 DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`] module"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "TIM7 DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: TIM7 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TIM7 status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: TIM7 event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`] module"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM7 event generation register"]
pub mod egr;
#[doc = "CNT (rw) register accessor: TIM7 counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "TIM7 counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: TIM7 prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "TIM7 prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: TIM7 auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "TIM7 auto-reload register"]
pub mod arr;
