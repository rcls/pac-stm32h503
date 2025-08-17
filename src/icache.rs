#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR: CR,
    pub SR: SR,
    pub IER: IER,
    pub FCR: FCR,
    pub HMONR: HMONR,
    pub MMONR: MMONR,
}
impl RegisterBlock {
    #[doc = "0x00 - ICACHE control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x04 - ICACHE status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x08 - ICACHE interrupt enable register"]
    #[inline(always)]
    pub const fn IER(&self) -> &IER {
        &self.IER
    }
    #[doc = "0x0c - ICACHE flag clear register"]
    #[inline(always)]
    pub const fn FCR(&self) -> &FCR {
        &self.FCR
    }
    #[doc = "0x10 - ICACHE hit monitor register"]
    #[inline(always)]
    pub const fn HMONR(&self) -> &HMONR {
        &self.HMONR
    }
    #[doc = "0x14 - ICACHE miss monitor register"]
    #[inline(always)]
    pub const fn MMONR(&self) -> &MMONR {
        &self.MMONR
    }
}
#[doc = "CR (rw) register accessor: ICACHE control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ICACHE control register"]
pub mod cr;
#[doc = "SR (r) register accessor: ICACHE status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "ICACHE status register"]
pub mod sr;
#[doc = "IER (rw) register accessor: ICACHE interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "ICACHE interrupt enable register"]
pub mod ier;
#[doc = "FCR (w) register accessor: ICACHE flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "ICACHE flag clear register"]
pub mod fcr;
#[doc = "HMONR (r) register accessor: ICACHE hit monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`hmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hmonr`] module"]
pub type HMONR = crate::Reg<hmonr::HMONR_SPEC>;
#[doc = "ICACHE hit monitor register"]
pub mod hmonr;
#[doc = "MMONR (r) register accessor: ICACHE miss monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmonr`] module"]
pub type MMONR = crate::Reg<mmonr::MMONR_SPEC>;
#[doc = "ICACHE miss monitor register"]
pub mod mmonr;
