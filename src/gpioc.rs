#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub MODER: MODER,
    pub OTYPER: OTYPER,
    pub OSPEEDR: OSPEEDR,
    pub PUPDR: PUPDR,
    pub IDR: IDR,
    pub ODR: ODR,
    pub BSRR: BSRR,
    pub LCKR: LCKR,
    pub AFRL: AFRL,
    pub AFRH: AFRH,
    pub BRR: BRR,
    pub HSLVR: HSLVR,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    #[inline(always)]
    pub const fn MODER(&self) -> &MODER {
        &self.MODER
    }
    #[doc = "0x04 - GPIO port output type register"]
    #[inline(always)]
    pub const fn OTYPER(&self) -> &OTYPER {
        &self.OTYPER
    }
    #[doc = "0x08 - GPIO port output speed register"]
    #[inline(always)]
    pub const fn OSPEEDR(&self) -> &OSPEEDR {
        &self.OSPEEDR
    }
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn PUPDR(&self) -> &PUPDR {
        &self.PUPDR
    }
    #[doc = "0x10 - GPIO port input data register"]
    #[inline(always)]
    pub const fn IDR(&self) -> &IDR {
        &self.IDR
    }
    #[doc = "0x14 - GPIO port output data register"]
    #[inline(always)]
    pub const fn ODR(&self) -> &ODR {
        &self.ODR
    }
    #[doc = "0x18 - GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn BSRR(&self) -> &BSRR {
        &self.BSRR
    }
    #[doc = "0x1c - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn LCKR(&self) -> &LCKR {
        &self.LCKR
    }
    #[doc = "0x20 - GPIO alternate function low register"]
    #[inline(always)]
    pub const fn AFRL(&self) -> &AFRL {
        &self.AFRL
    }
    #[doc = "0x24 - GPIO alternate function high register"]
    #[inline(always)]
    pub const fn AFRH(&self) -> &AFRH {
        &self.AFRH
    }
    #[doc = "0x28 - GPIO port bit reset register"]
    #[inline(always)]
    pub const fn BRR(&self) -> &BRR {
        &self.BRR
    }
    #[doc = "0x2c - GPIO high-speed low-voltage register"]
    #[inline(always)]
    pub const fn HSLVR(&self) -> &HSLVR {
        &self.HSLVR
    }
}
#[doc = "MODER (rw) register accessor: GPIO port mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`moder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moder`] module"]
pub type MODER = crate::Reg<moder::MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod moder;
#[doc = "OTYPER (rw) register accessor: GPIO port output type register\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otyper`] module"]
pub type OTYPER = crate::Reg<otyper::OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod otyper;
#[doc = "OSPEEDR (rw) register accessor: GPIO port output speed register\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospeedr`] module"]
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod ospeedr;
#[doc = "PUPDR (rw) register accessor: GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pupdr`] module"]
pub type PUPDR = crate::Reg<pupdr::PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pupdr;
#[doc = "IDR (r) register accessor: GPIO port input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod idr;
#[doc = "ODR (rw) register accessor: GPIO port output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`] module"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod odr;
#[doc = "BSRR (w) register accessor: GPIO port bit set/reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`] module"]
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod bsrr;
#[doc = "LCKR (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`] module"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "GPIO port configuration lock register"]
pub mod lckr;
#[doc = "AFRL (rw) register accessor: GPIO alternate function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`] module"]
pub type AFRL = crate::Reg<afrl::AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: GPIO alternate function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrh`] module"]
pub type AFRH = crate::Reg<afrh::AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod afrh;
#[doc = "BRR (w) register accessor: GPIO port bit reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "GPIO port bit reset register"]
pub mod brr;
#[doc = "HSLVR (rw) register accessor: GPIO high-speed low-voltage register\n\nYou can [`read`](crate::Reg::read) this register and get [`hslvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hslvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hslvr`] module"]
pub type HSLVR = crate::Reg<hslvr::HSLVR_SPEC>;
#[doc = "GPIO high-speed low-voltage register"]
pub mod hslvr;
