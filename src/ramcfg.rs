#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub _1CR: _1CR,
    _reserved1: [u8; 0x04],
    pub _1ISR: _1ISR,
    _reserved2: [u8; 0x1c],
    pub _1ERKEYR: _1ERKEYR,
    _reserved3: [u8; 0x14],
    pub _2CR: _2CR,
    pub _2IER: _2IER,
    pub _2ISR: _2ISR,
    pub _2SEAR: _2SEAR,
    pub _2DEAR: _2DEAR,
    pub _2ICR: _2ICR,
    pub _2WPR1: _2WPR1,
    _reserved10: [u8; 0x08],
    pub _2ECCKEYR: _2ECCKEYR,
    pub _2ERKEYR: _2ERKEYR,
    _reserved12: [u8; 0x18],
    pub _3IER: _3IER,
    pub _3ISR: _3ISR,
    pub _3SEAR: _3SEAR,
    pub _3DEAR: _3DEAR,
    pub _3ICR: _3ICR,
    _reserved17: [u8; 0x0c],
    pub _3ECCKEYR: _3ECCKEYR,
    pub _3ERKEYR: _3ERKEYR,
    _reserved19: [u8; 0x3c],
    pub _4ERKEYR: _4ERKEYR,
    _reserved20: [u8; 0x14],
    pub _5CR: _5CR,
    pub _5IER: _5IER,
    pub _5ISR: _5ISR,
    pub _5SEAR: _5SEAR,
    pub _5DEAR: _5DEAR,
    pub _5ICR: _5ICR,
    _reserved26: [u8; 0x0c],
    pub _5ECCKEYR: _5ECCKEYR,
    pub _5ERKEYR: _5ERKEYR,
}
impl RegisterBlock {
    #[doc = "0x00 - RAMCFG memory 1 control register"]
    #[inline(always)]
    pub const fn _1CR(&self) -> &_1CR {
        &self._1CR
    }
    #[doc = "0x08 - RAMCFG memory interrupt status register"]
    #[inline(always)]
    pub const fn _1ISR(&self) -> &_1ISR {
        &self._1ISR
    }
    #[doc = "0x28 - RAMCFG memory 1 erase key register"]
    #[inline(always)]
    pub const fn _1ERKEYR(&self) -> &_1ERKEYR {
        &self._1ERKEYR
    }
    #[doc = "0x40 - RAMCFG memory 2 control register"]
    #[inline(always)]
    pub const fn _2CR(&self) -> &_2CR {
        &self._2CR
    }
    #[doc = "0x44 - RAMCFG memory 2 interrupt enable register"]
    #[inline(always)]
    pub const fn _2IER(&self) -> &_2IER {
        &self._2IER
    }
    #[doc = "0x48 - RAMCFG memory interrupt status register"]
    #[inline(always)]
    pub const fn _2ISR(&self) -> &_2ISR {
        &self._2ISR
    }
    #[doc = "0x4c - RAMCFG memory 2 ECC single error address register"]
    #[inline(always)]
    pub const fn _2SEAR(&self) -> &_2SEAR {
        &self._2SEAR
    }
    #[doc = "0x50 - RAMCFG memory 2 ECC double error address register"]
    #[inline(always)]
    pub const fn _2DEAR(&self) -> &_2DEAR {
        &self._2DEAR
    }
    #[doc = "0x54 - RAMCFG memory 2 interrupt clear register 2"]
    #[inline(always)]
    pub const fn _2ICR(&self) -> &_2ICR {
        &self._2ICR
    }
    #[doc = "0x58 - RAMCFG memory 2 write protection register 1"]
    #[inline(always)]
    pub const fn _2WPR1(&self) -> &_2WPR1 {
        &self._2WPR1
    }
    #[doc = "0x64 - RAMCFG memory 2 ECC key register"]
    #[inline(always)]
    pub const fn _2ECCKEYR(&self) -> &_2ECCKEYR {
        &self._2ECCKEYR
    }
    #[doc = "0x68 - RAMCFG memory 2 erase key register"]
    #[inline(always)]
    pub const fn _2ERKEYR(&self) -> &_2ERKEYR {
        &self._2ERKEYR
    }
    #[doc = "0x84 - RAMCFG memory 3 interrupt enable register"]
    #[inline(always)]
    pub const fn _3IER(&self) -> &_3IER {
        &self._3IER
    }
    #[doc = "0x88 - RAMCFG memory interrupt status register"]
    #[inline(always)]
    pub const fn _3ISR(&self) -> &_3ISR {
        &self._3ISR
    }
    #[doc = "0x8c - RAMCFG memory 3 ECC single error address register"]
    #[inline(always)]
    pub const fn _3SEAR(&self) -> &_3SEAR {
        &self._3SEAR
    }
    #[doc = "0x90 - RAMCFG memory 3 ECC double error address register"]
    #[inline(always)]
    pub const fn _3DEAR(&self) -> &_3DEAR {
        &self._3DEAR
    }
    #[doc = "0x94 - RAMCFG memory 3 interrupt clear register 3"]
    #[inline(always)]
    pub const fn _3ICR(&self) -> &_3ICR {
        &self._3ICR
    }
    #[doc = "0xa4 - RAMCFG memory 3 ECC key register"]
    #[inline(always)]
    pub const fn _3ECCKEYR(&self) -> &_3ECCKEYR {
        &self._3ECCKEYR
    }
    #[doc = "0xa8 - RAMCFG memory 3 erase key register"]
    #[inline(always)]
    pub const fn _3ERKEYR(&self) -> &_3ERKEYR {
        &self._3ERKEYR
    }
    #[doc = "0xe8 - RAMCFG memory 4 erase key register"]
    #[inline(always)]
    pub const fn _4ERKEYR(&self) -> &_4ERKEYR {
        &self._4ERKEYR
    }
    #[doc = "0x100 - RAMCFG memory 5 control register"]
    #[inline(always)]
    pub const fn _5CR(&self) -> &_5CR {
        &self._5CR
    }
    #[doc = "0x104 - RAMCFG memory 5 interrupt enable register"]
    #[inline(always)]
    pub const fn _5IER(&self) -> &_5IER {
        &self._5IER
    }
    #[doc = "0x108 - RAMCFG memory interrupt status register"]
    #[inline(always)]
    pub const fn _5ISR(&self) -> &_5ISR {
        &self._5ISR
    }
    #[doc = "0x10c - RAMCFG memory 5 ECC single error address register"]
    #[inline(always)]
    pub const fn _5SEAR(&self) -> &_5SEAR {
        &self._5SEAR
    }
    #[doc = "0x110 - RAMCFG memory 5 ECC double error address register"]
    #[inline(always)]
    pub const fn _5DEAR(&self) -> &_5DEAR {
        &self._5DEAR
    }
    #[doc = "0x114 - RAMCFG memory 5 interrupt clear register 5"]
    #[inline(always)]
    pub const fn _5ICR(&self) -> &_5ICR {
        &self._5ICR
    }
    #[doc = "0x124 - RAMCFG memory 5 ECC key register"]
    #[inline(always)]
    pub const fn _5ECCKEYR(&self) -> &_5ECCKEYR {
        &self._5ECCKEYR
    }
    #[doc = "0x128 - RAMCFG memory 5 erase key register"]
    #[inline(always)]
    pub const fn _5ERKEYR(&self) -> &_5ERKEYR {
        &self._5ERKEYR
    }
}
#[doc = "1CR (rw) register accessor: RAMCFG memory 1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`_1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1cr`] module"]
#[doc(alias = "1CR")]
pub type _1CR = crate::Reg<_1cr::_1CR_SPEC>;
#[doc = "RAMCFG memory 1 control register"]
pub mod _1cr;
#[doc = "1ISR (r) register accessor: RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`_1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1isr`] module"]
#[doc(alias = "1ISR")]
pub type _1ISR = crate::Reg<_1isr::_1ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod _1isr;
#[doc = "1ERKEYR (w) register accessor: RAMCFG memory 1 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_1erkeyr`] module"]
#[doc(alias = "1ERKEYR")]
pub type _1ERKEYR = crate::Reg<_1erkeyr::_1ERKEYR_SPEC>;
#[doc = "RAMCFG memory 1 erase key register"]
pub mod _1erkeyr;
#[doc = "2CR (rw) register accessor: RAMCFG memory 2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`_2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2cr`] module"]
#[doc(alias = "2CR")]
pub type _2CR = crate::Reg<_2cr::_2CR_SPEC>;
#[doc = "RAMCFG memory 2 control register"]
pub mod _2cr;
#[doc = "2IER (rw) register accessor: RAMCFG memory 2 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`_2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2ier`] module"]
#[doc(alias = "2IER")]
pub type _2IER = crate::Reg<_2ier::_2IER_SPEC>;
#[doc = "RAMCFG memory 2 interrupt enable register"]
pub mod _2ier;
#[doc = "2ISR (r) register accessor: RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`_2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2isr`] module"]
#[doc(alias = "2ISR")]
pub type _2ISR = crate::Reg<_2isr::_2ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod _2isr;
#[doc = "2SEAR (r) register accessor: RAMCFG memory 2 ECC single error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`_2sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2sear`] module"]
#[doc(alias = "2SEAR")]
pub type _2SEAR = crate::Reg<_2sear::_2SEAR_SPEC>;
#[doc = "RAMCFG memory 2 ECC single error address register"]
pub mod _2sear;
#[doc = "2DEAR (r) register accessor: RAMCFG memory 2 ECC double error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`_2dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2dear`] module"]
#[doc(alias = "2DEAR")]
pub type _2DEAR = crate::Reg<_2dear::_2DEAR_SPEC>;
#[doc = "RAMCFG memory 2 ECC double error address register"]
pub mod _2dear;
#[doc = "2ICR (rw) register accessor: RAMCFG memory 2 interrupt clear register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`_2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2icr`] module"]
#[doc(alias = "2ICR")]
pub type _2ICR = crate::Reg<_2icr::_2ICR_SPEC>;
#[doc = "RAMCFG memory 2 interrupt clear register 2"]
pub mod _2icr;
#[doc = "2WPR1 (rw) register accessor: RAMCFG memory 2 write protection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`_2wpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2wpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2wpr1`] module"]
#[doc(alias = "2WPR1")]
pub type _2WPR1 = crate::Reg<_2wpr1::_2WPR1_SPEC>;
#[doc = "RAMCFG memory 2 write protection register 1"]
pub mod _2wpr1;
#[doc = "2ECCKEYR (w) register accessor: RAMCFG memory 2 ECC key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2ecckeyr`] module"]
#[doc(alias = "2ECCKEYR")]
pub type _2ECCKEYR = crate::Reg<_2ecckeyr::_2ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 2 ECC key register"]
pub mod _2ecckeyr;
#[doc = "2ERKEYR (w) register accessor: RAMCFG memory 2 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_2erkeyr`] module"]
#[doc(alias = "2ERKEYR")]
pub type _2ERKEYR = crate::Reg<_2erkeyr::_2ERKEYR_SPEC>;
#[doc = "RAMCFG memory 2 erase key register"]
pub mod _2erkeyr;
#[doc = "3IER (rw) register accessor: RAMCFG memory 3 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`_3ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3ier`] module"]
#[doc(alias = "3IER")]
pub type _3IER = crate::Reg<_3ier::_3IER_SPEC>;
#[doc = "RAMCFG memory 3 interrupt enable register"]
pub mod _3ier;
#[doc = "3ISR (r) register accessor: RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`_3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3isr`] module"]
#[doc(alias = "3ISR")]
pub type _3ISR = crate::Reg<_3isr::_3ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod _3isr;
#[doc = "3SEAR (r) register accessor: RAMCFG memory 3 ECC single error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`_3sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3sear`] module"]
#[doc(alias = "3SEAR")]
pub type _3SEAR = crate::Reg<_3sear::_3SEAR_SPEC>;
#[doc = "RAMCFG memory 3 ECC single error address register"]
pub mod _3sear;
#[doc = "3DEAR (r) register accessor: RAMCFG memory 3 ECC double error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`_3dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3dear`] module"]
#[doc(alias = "3DEAR")]
pub type _3DEAR = crate::Reg<_3dear::_3DEAR_SPEC>;
#[doc = "RAMCFG memory 3 ECC double error address register"]
pub mod _3dear;
#[doc = "3ICR (rw) register accessor: RAMCFG memory 3 interrupt clear register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`_3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3icr`] module"]
#[doc(alias = "3ICR")]
pub type _3ICR = crate::Reg<_3icr::_3ICR_SPEC>;
#[doc = "RAMCFG memory 3 interrupt clear register 3"]
pub mod _3icr;
#[doc = "3ECCKEYR (w) register accessor: RAMCFG memory 3 ECC key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3ecckeyr`] module"]
#[doc(alias = "3ECCKEYR")]
pub type _3ECCKEYR = crate::Reg<_3ecckeyr::_3ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 3 ECC key register"]
pub mod _3ecckeyr;
#[doc = "3ERKEYR (w) register accessor: RAMCFG memory 3 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3erkeyr`] module"]
#[doc(alias = "3ERKEYR")]
pub type _3ERKEYR = crate::Reg<_3erkeyr::_3ERKEYR_SPEC>;
#[doc = "RAMCFG memory 3 erase key register"]
pub mod _3erkeyr;
#[doc = "4ERKEYR (w) register accessor: RAMCFG memory 4 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_4erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_4erkeyr`] module"]
#[doc(alias = "4ERKEYR")]
pub type _4ERKEYR = crate::Reg<_4erkeyr::_4ERKEYR_SPEC>;
#[doc = "RAMCFG memory 4 erase key register"]
pub mod _4erkeyr;
#[doc = "5CR (rw) register accessor: RAMCFG memory 5 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`_5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5cr`] module"]
#[doc(alias = "5CR")]
pub type _5CR = crate::Reg<_5cr::_5CR_SPEC>;
#[doc = "RAMCFG memory 5 control register"]
pub mod _5cr;
#[doc = "5IER (rw) register accessor: RAMCFG memory 5 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`_5ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5ier`] module"]
#[doc(alias = "5IER")]
pub type _5IER = crate::Reg<_5ier::_5IER_SPEC>;
#[doc = "RAMCFG memory 5 interrupt enable register"]
pub mod _5ier;
#[doc = "5ISR (r) register accessor: RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`_5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5isr`] module"]
#[doc(alias = "5ISR")]
pub type _5ISR = crate::Reg<_5isr::_5ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod _5isr;
#[doc = "5SEAR (r) register accessor: RAMCFG memory 5 ECC single error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`_5sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5sear`] module"]
#[doc(alias = "5SEAR")]
pub type _5SEAR = crate::Reg<_5sear::_5SEAR_SPEC>;
#[doc = "RAMCFG memory 5 ECC single error address register"]
pub mod _5sear;
#[doc = "5DEAR (r) register accessor: RAMCFG memory 5 ECC double error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`_5dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5dear`] module"]
#[doc(alias = "5DEAR")]
pub type _5DEAR = crate::Reg<_5dear::_5DEAR_SPEC>;
#[doc = "RAMCFG memory 5 ECC double error address register"]
pub mod _5dear;
#[doc = "5ICR (rw) register accessor: RAMCFG memory 5 interrupt clear register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`_5icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5icr`] module"]
#[doc(alias = "5ICR")]
pub type _5ICR = crate::Reg<_5icr::_5ICR_SPEC>;
#[doc = "RAMCFG memory 5 interrupt clear register 5"]
pub mod _5icr;
#[doc = "5ECCKEYR (w) register accessor: RAMCFG memory 5 ECC key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5ecckeyr`] module"]
#[doc(alias = "5ECCKEYR")]
pub type _5ECCKEYR = crate::Reg<_5ecckeyr::_5ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 5 ECC key register"]
pub mod _5ecckeyr;
#[doc = "5ERKEYR (w) register accessor: RAMCFG memory 5 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_5erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_5erkeyr`] module"]
#[doc(alias = "5ERKEYR")]
pub type _5ERKEYR = crate::Reg<_5erkeyr::_5ERKEYR_SPEC>;
#[doc = "RAMCFG memory 5 erase key register"]
pub mod _5erkeyr;
