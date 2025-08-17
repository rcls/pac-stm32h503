#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR: CR,
    _reserved1: [u8; 0x0c],
    pub HSICFGR: HSICFGR,
    pub CRRCR: CRRCR,
    pub CSICFGR: CSICFGR,
    pub CFGR1: CFGR1,
    pub CFGR2: CFGR2,
    _reserved6: [u8; 0x04],
    pub PLL1CFGR: PLL1CFGR,
    pub PLL2CFGR: PLL2CFGR,
    _reserved8: [u8; 0x04],
    pub PLL1DIVR: PLL1DIVR,
    pub PLL1FRACR: PLL1FRACR,
    pub PLL2DIVR: PLL2DIVR,
    pub PLL2FRACR: PLL2FRACR,
    _reserved12: [u8; 0x0c],
    pub CIER: CIER,
    pub CIFR: CIFR,
    pub CICR: CICR,
    _reserved15: [u8; 0x04],
    pub AHB1RSTR: AHB1RSTR,
    pub AHB2RSTR: AHB2RSTR,
    _reserved17: [u8; 0x0c],
    pub APB1LRSTR: APB1LRSTR,
    pub APB1HRSTR: APB1HRSTR,
    pub APB2RSTR: APB2RSTR,
    pub APB3RSTR: APB3RSTR,
    _reserved21: [u8; 0x04],
    pub AHB1ENR: AHB1ENR,
    pub AHB2ENR: AHB2ENR,
    _reserved23: [u8; 0x0c],
    pub APB1LENR: APB1LENR,
    pub APB1HENR: APB1HENR,
    pub APB2ENR: APB2ENR,
    pub APB3ENR: APB3ENR,
    _reserved27: [u8; 0x04],
    pub AHB1LPENR: AHB1LPENR,
    pub AHB2LPENR: AHB2LPENR,
    _reserved29: [u8; 0x0c],
    pub APB1LLPENR: APB1LLPENR,
    pub APB1HLPENR: APB1HLPENR,
    pub APB2LPENR: APB2LPENR,
    pub APB3LPENR: APB3LPENR,
    _reserved33: [u8; 0x04],
    pub CCIPR1: CCIPR1,
    pub CCIPR2: CCIPR2,
    pub CCIPR3: CCIPR3,
    pub CCIPR4: CCIPR4,
    pub CCIPR5: CCIPR5,
    _reserved38: [u8; 0x04],
    pub BDCR: BDCR,
    pub RSR: RSR,
}
impl RegisterBlock {
    #[doc = "0x00 - RCC clock control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x10 - RCC HSI calibration register"]
    #[inline(always)]
    pub const fn HSICFGR(&self) -> &HSICFGR {
        &self.HSICFGR
    }
    #[doc = "0x14 - RCC clock recovery RC register"]
    #[inline(always)]
    pub const fn CRRCR(&self) -> &CRRCR {
        &self.CRRCR
    }
    #[doc = "0x18 - RCC CSI calibration register"]
    #[inline(always)]
    pub const fn CSICFGR(&self) -> &CSICFGR {
        &self.CSICFGR
    }
    #[doc = "0x1c - RCC clock configuration register"]
    #[inline(always)]
    pub const fn CFGR1(&self) -> &CFGR1 {
        &self.CFGR1
    }
    #[doc = "0x20 - RCC CPU domain clock configuration register 2"]
    #[inline(always)]
    pub const fn CFGR2(&self) -> &CFGR2 {
        &self.CFGR2
    }
    #[doc = "0x28 - RCC PLL clock source selection register"]
    #[inline(always)]
    pub const fn PLL1CFGR(&self) -> &PLL1CFGR {
        &self.PLL1CFGR
    }
    #[doc = "0x2c - RCC PLL clock source selection register"]
    #[inline(always)]
    pub const fn PLL2CFGR(&self) -> &PLL2CFGR {
        &self.PLL2CFGR
    }
    #[doc = "0x34 - RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn PLL1DIVR(&self) -> &PLL1DIVR {
        &self.PLL1DIVR
    }
    #[doc = "0x38 - RCC PLL1 fractional divider register"]
    #[inline(always)]
    pub const fn PLL1FRACR(&self) -> &PLL1FRACR {
        &self.PLL1FRACR
    }
    #[doc = "0x3c - RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn PLL2DIVR(&self) -> &PLL2DIVR {
        &self.PLL2DIVR
    }
    #[doc = "0x40 - RCC PLL2 fractional divider register"]
    #[inline(always)]
    pub const fn PLL2FRACR(&self) -> &PLL2FRACR {
        &self.PLL2FRACR
    }
    #[doc = "0x50 - RCC clock source interrupt enable register"]
    #[inline(always)]
    pub const fn CIER(&self) -> &CIER {
        &self.CIER
    }
    #[doc = "0x54 - RCC clock source interrupt flag register"]
    #[inline(always)]
    pub const fn CIFR(&self) -> &CIFR {
        &self.CIFR
    }
    #[doc = "0x58 - RCC clock source interrupt clear register"]
    #[inline(always)]
    pub const fn CICR(&self) -> &CICR {
        &self.CICR
    }
    #[doc = "0x60 - RCC AHB1 reset register"]
    #[inline(always)]
    pub const fn AHB1RSTR(&self) -> &AHB1RSTR {
        &self.AHB1RSTR
    }
    #[doc = "0x64 - RCC AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn AHB2RSTR(&self) -> &AHB2RSTR {
        &self.AHB2RSTR
    }
    #[doc = "0x74 - RCC APB1 peripheral low reset register"]
    #[inline(always)]
    pub const fn APB1LRSTR(&self) -> &APB1LRSTR {
        &self.APB1LRSTR
    }
    #[doc = "0x78 - RCC APB1 peripheral high reset register"]
    #[inline(always)]
    pub const fn APB1HRSTR(&self) -> &APB1HRSTR {
        &self.APB1HRSTR
    }
    #[doc = "0x7c - RCC APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn APB2RSTR(&self) -> &APB2RSTR {
        &self.APB2RSTR
    }
    #[doc = "0x80 - RCC APB3 peripheral reset register"]
    #[inline(always)]
    pub const fn APB3RSTR(&self) -> &APB3RSTR {
        &self.APB3RSTR
    }
    #[doc = "0x88 - RCC AHB1 peripherals clock register"]
    #[inline(always)]
    pub const fn AHB1ENR(&self) -> &AHB1ENR {
        &self.AHB1ENR
    }
    #[doc = "0x8c - RCC AHB2 peripheral clock register"]
    #[inline(always)]
    pub const fn AHB2ENR(&self) -> &AHB2ENR {
        &self.AHB2ENR
    }
    #[doc = "0x9c - RCC APB1 peripheral clock register"]
    #[inline(always)]
    pub const fn APB1LENR(&self) -> &APB1LENR {
        &self.APB1LENR
    }
    #[doc = "0xa0 - RCC APB1 peripheral clock register"]
    #[inline(always)]
    pub const fn APB1HENR(&self) -> &APB1HENR {
        &self.APB1HENR
    }
    #[doc = "0xa4 - RCC APB2 peripheral clock register"]
    #[inline(always)]
    pub const fn APB2ENR(&self) -> &APB2ENR {
        &self.APB2ENR
    }
    #[doc = "0xa8 - RCC APB3 peripheral clock register"]
    #[inline(always)]
    pub const fn APB3ENR(&self) -> &APB3ENR {
        &self.APB3ENR
    }
    #[doc = "0xb0 - RCC AHB1 sleep clock register"]
    #[inline(always)]
    pub const fn AHB1LPENR(&self) -> &AHB1LPENR {
        &self.AHB1LPENR
    }
    #[doc = "0xb4 - RCC AHB2 sleep clock register"]
    #[inline(always)]
    pub const fn AHB2LPENR(&self) -> &AHB2LPENR {
        &self.AHB2LPENR
    }
    #[doc = "0xc4 - RCC APB1 sleep clock register"]
    #[inline(always)]
    pub const fn APB1LLPENR(&self) -> &APB1LLPENR {
        &self.APB1LLPENR
    }
    #[doc = "0xc8 - RCC APB1 sleep clock register"]
    #[inline(always)]
    pub const fn APB1HLPENR(&self) -> &APB1HLPENR {
        &self.APB1HLPENR
    }
    #[doc = "0xcc - RCC APB2 sleep clock register"]
    #[inline(always)]
    pub const fn APB2LPENR(&self) -> &APB2LPENR {
        &self.APB2LPENR
    }
    #[doc = "0xd0 - RCC APB3 sleep clock register"]
    #[inline(always)]
    pub const fn APB3LPENR(&self) -> &APB3LPENR {
        &self.APB3LPENR
    }
    #[doc = "0xd8 - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn CCIPR1(&self) -> &CCIPR1 {
        &self.CCIPR1
    }
    #[doc = "0xdc - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn CCIPR2(&self) -> &CCIPR2 {
        &self.CCIPR2
    }
    #[doc = "0xe0 - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn CCIPR3(&self) -> &CCIPR3 {
        &self.CCIPR3
    }
    #[doc = "0xe4 - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn CCIPR4(&self) -> &CCIPR4 {
        &self.CCIPR4
    }
    #[doc = "0xe8 - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn CCIPR5(&self) -> &CCIPR5 {
        &self.CCIPR5
    }
    #[doc = "0xf0 - RCC Backup domain control register"]
    #[inline(always)]
    pub const fn BDCR(&self) -> &BDCR {
        &self.BDCR
    }
    #[doc = "0xf4 - RCC reset status register"]
    #[inline(always)]
    pub const fn RSR(&self) -> &RSR {
        &self.RSR
    }
}
#[doc = "CR (rw) register accessor: RCC clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RCC clock control register"]
pub mod cr;
#[doc = "HSICFGR (rw) register accessor: RCC HSI calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsicfgr`] module"]
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGR_SPEC>;
#[doc = "RCC HSI calibration register"]
pub mod hsicfgr;
#[doc = "CRRCR (r) register accessor: RCC clock recovery RC register\n\nYou can [`read`](crate::Reg::read) this register and get [`crrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crrcr`] module"]
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
#[doc = "RCC clock recovery RC register"]
pub mod crrcr;
#[doc = "CSICFGR (rw) register accessor: RCC CSI calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`csicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csicfgr`] module"]
pub type CSICFGR = crate::Reg<csicfgr::CSICFGR_SPEC>;
#[doc = "RCC CSI calibration register"]
pub mod csicfgr;
#[doc = "CFGR1 (rw) register accessor: RCC clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`] module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "RCC clock configuration register"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: RCC CPU domain clock configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "RCC CPU domain clock configuration register 2"]
pub mod cfgr2;
#[doc = "PLL1CFGR (rw) register accessor: RCC PLL clock source selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1cfgr`] module"]
pub type PLL1CFGR = crate::Reg<pll1cfgr::PLL1CFGR_SPEC>;
#[doc = "RCC PLL clock source selection register"]
pub mod pll1cfgr;
#[doc = "PLL2CFGR (rw) register accessor: RCC PLL clock source selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2cfgr`] module"]
pub type PLL2CFGR = crate::Reg<pll2cfgr::PLL2CFGR_SPEC>;
#[doc = "RCC PLL clock source selection register"]
pub mod pll2cfgr;
#[doc = "PLL1DIVR (rw) register accessor: RCC PLL1 dividers register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1divr`] module"]
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVR_SPEC>;
#[doc = "RCC PLL1 dividers register"]
pub mod pll1divr;
#[doc = "PLL1FRACR (rw) register accessor: RCC PLL1 fractional divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1fracr`] module"]
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACR_SPEC>;
#[doc = "RCC PLL1 fractional divider register"]
pub mod pll1fracr;
#[doc = "PLL2DIVR (rw) register accessor: RCC PLL1 dividers register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2divr`] module"]
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVR_SPEC>;
#[doc = "RCC PLL1 dividers register"]
pub mod pll2divr;
#[doc = "PLL2FRACR (rw) register accessor: RCC PLL2 fractional divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2fracr`] module"]
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACR_SPEC>;
#[doc = "RCC PLL2 fractional divider register"]
pub mod pll2fracr;
#[doc = "CIER (rw) register accessor: RCC clock source interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`] module"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "RCC clock source interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: RCC clock source interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`] module"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = "RCC clock source interrupt flag register"]
pub mod cifr;
#[doc = "CICR (rw) register accessor: RCC clock source interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`] module"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = "RCC clock source interrupt clear register"]
pub mod cicr;
#[doc = "AHB1RSTR (rw) register accessor: RCC AHB1 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`] module"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "RCC AHB1 reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: RCC AHB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`] module"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "RCC AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "APB1LRSTR (rw) register accessor: RCC APB1 peripheral low reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lrstr`] module"]
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTR_SPEC>;
#[doc = "RCC APB1 peripheral low reset register"]
pub mod apb1lrstr;
#[doc = "APB1HRSTR (rw) register accessor: RCC APB1 peripheral high reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hrstr`] module"]
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTR_SPEC>;
#[doc = "RCC APB1 peripheral high reset register"]
pub mod apb1hrstr;
#[doc = "APB2RSTR (rw) register accessor: RCC APB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`] module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "RCC APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB3RSTR (rw) register accessor: RCC APB3 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3rstr`] module"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
#[doc = "RCC APB3 peripheral reset register"]
pub mod apb3rstr;
#[doc = "AHB1ENR (rw) register accessor: RCC AHB1 peripherals clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`] module"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "RCC AHB1 peripherals clock register"]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: RCC AHB2 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`] module"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "RCC AHB2 peripheral clock register"]
pub mod ahb2enr;
#[doc = "APB1LENR (rw) register accessor: RCC APB1 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lenr`] module"]
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENR_SPEC>;
#[doc = "RCC APB1 peripheral clock register"]
pub mod apb1lenr;
#[doc = "APB1HENR (rw) register accessor: RCC APB1 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1henr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1henr`] module"]
pub type APB1HENR = crate::Reg<apb1henr::APB1HENR_SPEC>;
#[doc = "RCC APB1 peripheral clock register"]
pub mod apb1henr;
#[doc = "APB2ENR (rw) register accessor: RCC APB2 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`] module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "RCC APB2 peripheral clock register"]
pub mod apb2enr;
#[doc = "APB3ENR (rw) register accessor: RCC APB3 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3enr`] module"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
#[doc = "RCC APB3 peripheral clock register"]
pub mod apb3enr;
#[doc = "AHB1LPENR (rw) register accessor: RCC AHB1 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1lpenr`] module"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
#[doc = "RCC AHB1 sleep clock register"]
pub mod ahb1lpenr;
#[doc = "AHB2LPENR (rw) register accessor: RCC AHB2 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2lpenr`] module"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
#[doc = "RCC AHB2 sleep clock register"]
pub mod ahb2lpenr;
#[doc = "APB1LLPENR (rw) register accessor: RCC APB1 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1llpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1llpenr`] module"]
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENR_SPEC>;
#[doc = "RCC APB1 sleep clock register"]
pub mod apb1llpenr;
#[doc = "APB1HLPENR (rw) register accessor: RCC APB1 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hlpenr`] module"]
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENR_SPEC>;
#[doc = "RCC APB1 sleep clock register"]
pub mod apb1hlpenr;
#[doc = "APB2LPENR (rw) register accessor: RCC APB2 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpenr`] module"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
#[doc = "RCC APB2 sleep clock register"]
pub mod apb2lpenr;
#[doc = "APB3LPENR (rw) register accessor: RCC APB3 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3lpenr`] module"]
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENR_SPEC>;
#[doc = "RCC APB3 sleep clock register"]
pub mod apb3lpenr;
#[doc = "CCIPR1 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr1`] module"]
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr1;
#[doc = "CCIPR2 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr2`] module"]
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr2;
#[doc = "CCIPR3 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr3`] module"]
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr3;
#[doc = "CCIPR4 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr4`] module"]
pub type CCIPR4 = crate::Reg<ccipr4::CCIPR4_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr4;
#[doc = "CCIPR5 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr5`] module"]
pub type CCIPR5 = crate::Reg<ccipr5::CCIPR5_SPEC>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr5;
#[doc = "BDCR (rw) register accessor: RCC Backup domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`] module"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "RCC Backup domain control register"]
pub mod bdcr;
#[doc = "RSR (rw) register accessor: RCC reset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`] module"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "RCC reset status register"]
pub mod rsr;
