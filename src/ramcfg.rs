#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub M1CR: M1CR,
    _reserved1: [u8; 0x04],
    pub M1ISR: M1ISR,
    _reserved2: [u8; 0x1c],
    pub M1ERKEYR: M1ERKEYR,
    _reserved3: [u8; 0x14],
    pub M2CR: M2CR,
    pub M2IER: M2IER,
    pub M2ISR: M2ISR,
    pub M2SEAR: M2SEAR,
    pub M2DEAR: M2DEAR,
    pub M2ICR: M2ICR,
    pub M2WPR1: M2WPR1,
    _reserved10: [u8; 0x08],
    pub M2ECCKEYR: M2ECCKEYR,
    pub M2ERKEYR: M2ERKEYR,
    _reserved12: [u8; 0x18],
    pub M3IER: M3IER,
    pub M3ISR: M3ISR,
    pub M3SEAR: M3SEAR,
    pub M3DEAR: M3DEAR,
    pub M3ICR: M3ICR,
    _reserved17: [u8; 0x0c],
    pub M3ECCKEYR: M3ECCKEYR,
    pub M3ERKEYR: M3ERKEYR,
    _reserved19: [u8; 0x3c],
    pub M4ERKEYR: M4ERKEYR,
    _reserved20: [u8; 0x14],
    pub M5CR: M5CR,
    pub M5IER: M5IER,
    pub M5ISR: M5ISR,
    pub M5SEAR: M5SEAR,
    pub M5DEAR: M5DEAR,
    pub M5ICR: M5ICR,
    _reserved26: [u8; 0x0c],
    pub M5ECCKEYR: M5ECCKEYR,
    pub M5ERKEYR: M5ERKEYR,
}
impl RegisterBlock {
    #[doc = "0x00 - RAMCFG memory 1 control register"]
    #[inline(always)]
    pub const fn M1CR(&self) -> &M1CR {
        &self.M1CR
    }
    #[doc = "0x08 - RAMCFG memory interrupt status register"]
    #[inline(always)]
    pub const fn M1ISR(&self) -> &M1ISR {
        &self.M1ISR
    }
    #[doc = "0x28 - RAMCFG memory 1 erase key register"]
    #[inline(always)]
    pub const fn M1ERKEYR(&self) -> &M1ERKEYR {
        &self.M1ERKEYR
    }
    #[doc = "0x40 - RAMCFG memory 2 control register"]
    #[inline(always)]
    pub const fn M2CR(&self) -> &M2CR {
        &self.M2CR
    }
    #[doc = "0x44 - RAMCFG memory 2 interrupt enable register"]
    #[inline(always)]
    pub const fn M2IER(&self) -> &M2IER {
        &self.M2IER
    }
    #[doc = "0x48 - RAMCFG memory interrupt status register"]
    #[inline(always)]
    pub const fn M2ISR(&self) -> &M2ISR {
        &self.M2ISR
    }
    #[doc = "0x4c - RAMCFG memory 2 ECC single error address register"]
    #[inline(always)]
    pub const fn M2SEAR(&self) -> &M2SEAR {
        &self.M2SEAR
    }
    #[doc = "0x50 - RAMCFG memory 2 ECC double error address register"]
    #[inline(always)]
    pub const fn M2DEAR(&self) -> &M2DEAR {
        &self.M2DEAR
    }
    #[doc = "0x54 - RAMCFG memory 2 interrupt clear register 2"]
    #[inline(always)]
    pub const fn M2ICR(&self) -> &M2ICR {
        &self.M2ICR
    }
    #[doc = "0x58 - RAMCFG memory 2 write protection register 1"]
    #[inline(always)]
    pub const fn M2WPR1(&self) -> &M2WPR1 {
        &self.M2WPR1
    }
    #[doc = "0x64 - RAMCFG memory 2 ECC key register"]
    #[inline(always)]
    pub const fn M2ECCKEYR(&self) -> &M2ECCKEYR {
        &self.M2ECCKEYR
    }
    #[doc = "0x68 - RAMCFG memory 2 erase key register"]
    #[inline(always)]
    pub const fn M2ERKEYR(&self) -> &M2ERKEYR {
        &self.M2ERKEYR
    }
    #[doc = "0x84 - RAMCFG memory 3 interrupt enable register"]
    #[inline(always)]
    pub const fn M3IER(&self) -> &M3IER {
        &self.M3IER
    }
    #[doc = "0x88 - RAMCFG memory interrupt status register"]
    #[inline(always)]
    pub const fn M3ISR(&self) -> &M3ISR {
        &self.M3ISR
    }
    #[doc = "0x8c - RAMCFG memory 3 ECC single error address register"]
    #[inline(always)]
    pub const fn M3SEAR(&self) -> &M3SEAR {
        &self.M3SEAR
    }
    #[doc = "0x90 - RAMCFG memory 3 ECC double error address register"]
    #[inline(always)]
    pub const fn M3DEAR(&self) -> &M3DEAR {
        &self.M3DEAR
    }
    #[doc = "0x94 - RAMCFG memory 3 interrupt clear register 3"]
    #[inline(always)]
    pub const fn M3ICR(&self) -> &M3ICR {
        &self.M3ICR
    }
    #[doc = "0xa4 - RAMCFG memory 3 ECC key register"]
    #[inline(always)]
    pub const fn M3ECCKEYR(&self) -> &M3ECCKEYR {
        &self.M3ECCKEYR
    }
    #[doc = "0xa8 - RAMCFG memory 3 erase key register"]
    #[inline(always)]
    pub const fn M3ERKEYR(&self) -> &M3ERKEYR {
        &self.M3ERKEYR
    }
    #[doc = "0xe8 - RAMCFG memory 4 erase key register"]
    #[inline(always)]
    pub const fn M4ERKEYR(&self) -> &M4ERKEYR {
        &self.M4ERKEYR
    }
    #[doc = "0x100 - RAMCFG memory 5 control register"]
    #[inline(always)]
    pub const fn M5CR(&self) -> &M5CR {
        &self.M5CR
    }
    #[doc = "0x104 - RAMCFG memory 5 interrupt enable register"]
    #[inline(always)]
    pub const fn M5IER(&self) -> &M5IER {
        &self.M5IER
    }
    #[doc = "0x108 - RAMCFG memory interrupt status register"]
    #[inline(always)]
    pub const fn M5ISR(&self) -> &M5ISR {
        &self.M5ISR
    }
    #[doc = "0x10c - RAMCFG memory 5 ECC single error address register"]
    #[inline(always)]
    pub const fn M5SEAR(&self) -> &M5SEAR {
        &self.M5SEAR
    }
    #[doc = "0x110 - RAMCFG memory 5 ECC double error address register"]
    #[inline(always)]
    pub const fn M5DEAR(&self) -> &M5DEAR {
        &self.M5DEAR
    }
    #[doc = "0x114 - RAMCFG memory 5 interrupt clear register 5"]
    #[inline(always)]
    pub const fn M5ICR(&self) -> &M5ICR {
        &self.M5ICR
    }
    #[doc = "0x124 - RAMCFG memory 5 ECC key register"]
    #[inline(always)]
    pub const fn M5ECCKEYR(&self) -> &M5ECCKEYR {
        &self.M5ECCKEYR
    }
    #[doc = "0x128 - RAMCFG memory 5 erase key register"]
    #[inline(always)]
    pub const fn M5ERKEYR(&self) -> &M5ERKEYR {
        &self.M5ERKEYR
    }
}
#[doc = "M1CR (rw) register accessor: RAMCFG memory 1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1cr`] module"]
pub type M1CR = crate::Reg<m1cr::M1CR_SPEC>;
#[doc = "RAMCFG memory 1 control register"]
pub mod m1cr;
#[doc = "M1ISR (r) register accessor: RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1isr`] module"]
pub type M1ISR = crate::Reg<m1isr::M1ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod m1isr;
#[doc = "M1ERKEYR (w) register accessor: RAMCFG memory 1 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1erkeyr`] module"]
pub type M1ERKEYR = crate::Reg<m1erkeyr::M1ERKEYR_SPEC>;
#[doc = "RAMCFG memory 1 erase key register"]
pub mod m1erkeyr;
#[doc = "M2CR (rw) register accessor: RAMCFG memory 2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2cr`] module"]
pub type M2CR = crate::Reg<m2cr::M2CR_SPEC>;
#[doc = "RAMCFG memory 2 control register"]
pub mod m2cr;
#[doc = "M2IER (rw) register accessor: RAMCFG memory 2 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2ier`] module"]
pub type M2IER = crate::Reg<m2ier::M2IER_SPEC>;
#[doc = "RAMCFG memory 2 interrupt enable register"]
pub mod m2ier;
#[doc = "M2ISR (r) register accessor: RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2isr`] module"]
pub type M2ISR = crate::Reg<m2isr::M2ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod m2isr;
#[doc = "M2SEAR (r) register accessor: RAMCFG memory 2 ECC single error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2sear`] module"]
pub type M2SEAR = crate::Reg<m2sear::M2SEAR_SPEC>;
#[doc = "RAMCFG memory 2 ECC single error address register"]
pub mod m2sear;
#[doc = "M2DEAR (r) register accessor: RAMCFG memory 2 ECC double error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2dear`] module"]
pub type M2DEAR = crate::Reg<m2dear::M2DEAR_SPEC>;
#[doc = "RAMCFG memory 2 ECC double error address register"]
pub mod m2dear;
#[doc = "M2ICR (rw) register accessor: RAMCFG memory 2 interrupt clear register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`m2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2icr`] module"]
pub type M2ICR = crate::Reg<m2icr::M2ICR_SPEC>;
#[doc = "RAMCFG memory 2 interrupt clear register 2"]
pub mod m2icr;
#[doc = "M2WPR1 (rw) register accessor: RAMCFG memory 2 write protection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`m2wpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2wpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2wpr1`] module"]
pub type M2WPR1 = crate::Reg<m2wpr1::M2WPR1_SPEC>;
#[doc = "RAMCFG memory 2 write protection register 1"]
pub mod m2wpr1;
#[doc = "M2ECCKEYR (w) register accessor: RAMCFG memory 2 ECC key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2ecckeyr`] module"]
pub type M2ECCKEYR = crate::Reg<m2ecckeyr::M2ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 2 ECC key register"]
pub mod m2ecckeyr;
#[doc = "M2ERKEYR (w) register accessor: RAMCFG memory 2 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2erkeyr`] module"]
pub type M2ERKEYR = crate::Reg<m2erkeyr::M2ERKEYR_SPEC>;
#[doc = "RAMCFG memory 2 erase key register"]
pub mod m2erkeyr;
#[doc = "M3IER (rw) register accessor: RAMCFG memory 3 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3ier`] module"]
pub type M3IER = crate::Reg<m3ier::M3IER_SPEC>;
#[doc = "RAMCFG memory 3 interrupt enable register"]
pub mod m3ier;
#[doc = "M3ISR (r) register accessor: RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3isr`] module"]
pub type M3ISR = crate::Reg<m3isr::M3ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod m3isr;
#[doc = "M3SEAR (r) register accessor: RAMCFG memory 3 ECC single error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3sear`] module"]
pub type M3SEAR = crate::Reg<m3sear::M3SEAR_SPEC>;
#[doc = "RAMCFG memory 3 ECC single error address register"]
pub mod m3sear;
#[doc = "M3DEAR (r) register accessor: RAMCFG memory 3 ECC double error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3dear`] module"]
pub type M3DEAR = crate::Reg<m3dear::M3DEAR_SPEC>;
#[doc = "RAMCFG memory 3 ECC double error address register"]
pub mod m3dear;
#[doc = "M3ICR (rw) register accessor: RAMCFG memory 3 interrupt clear register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`m3icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3icr`] module"]
pub type M3ICR = crate::Reg<m3icr::M3ICR_SPEC>;
#[doc = "RAMCFG memory 3 interrupt clear register 3"]
pub mod m3icr;
#[doc = "M3ECCKEYR (w) register accessor: RAMCFG memory 3 ECC key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3ecckeyr`] module"]
pub type M3ECCKEYR = crate::Reg<m3ecckeyr::M3ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 3 ECC key register"]
pub mod m3ecckeyr;
#[doc = "M3ERKEYR (w) register accessor: RAMCFG memory 3 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3erkeyr`] module"]
pub type M3ERKEYR = crate::Reg<m3erkeyr::M3ERKEYR_SPEC>;
#[doc = "RAMCFG memory 3 erase key register"]
pub mod m3erkeyr;
#[doc = "M4ERKEYR (w) register accessor: RAMCFG memory 4 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4erkeyr`] module"]
pub type M4ERKEYR = crate::Reg<m4erkeyr::M4ERKEYR_SPEC>;
#[doc = "RAMCFG memory 4 erase key register"]
pub mod m4erkeyr;
#[doc = "M5CR (rw) register accessor: RAMCFG memory 5 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5cr`] module"]
pub type M5CR = crate::Reg<m5cr::M5CR_SPEC>;
#[doc = "RAMCFG memory 5 control register"]
pub mod m5cr;
#[doc = "M5IER (rw) register accessor: RAMCFG memory 5 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5ier`] module"]
pub type M5IER = crate::Reg<m5ier::M5IER_SPEC>;
#[doc = "RAMCFG memory 5 interrupt enable register"]
pub mod m5ier;
#[doc = "M5ISR (r) register accessor: RAMCFG memory interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5isr`] module"]
pub type M5ISR = crate::Reg<m5isr::M5ISR_SPEC>;
#[doc = "RAMCFG memory interrupt status register"]
pub mod m5isr;
#[doc = "M5SEAR (r) register accessor: RAMCFG memory 5 ECC single error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5sear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5sear`] module"]
pub type M5SEAR = crate::Reg<m5sear::M5SEAR_SPEC>;
#[doc = "RAMCFG memory 5 ECC single error address register"]
pub mod m5sear;
#[doc = "M5DEAR (r) register accessor: RAMCFG memory 5 ECC double error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5dear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5dear`] module"]
pub type M5DEAR = crate::Reg<m5dear::M5DEAR_SPEC>;
#[doc = "RAMCFG memory 5 ECC double error address register"]
pub mod m5dear;
#[doc = "M5ICR (rw) register accessor: RAMCFG memory 5 interrupt clear register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`m5icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5icr`] module"]
pub type M5ICR = crate::Reg<m5icr::M5ICR_SPEC>;
#[doc = "RAMCFG memory 5 interrupt clear register 5"]
pub mod m5icr;
#[doc = "M5ECCKEYR (w) register accessor: RAMCFG memory 5 ECC key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5ecckeyr`] module"]
pub type M5ECCKEYR = crate::Reg<m5ecckeyr::M5ECCKEYR_SPEC>;
#[doc = "RAMCFG memory 5 ECC key register"]
pub mod m5ecckeyr;
#[doc = "M5ERKEYR (w) register accessor: RAMCFG memory 5 erase key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5erkeyr`] module"]
pub type M5ERKEYR = crate::Reg<m5erkeyr::M5ERKEYR_SPEC>;
#[doc = "RAMCFG memory 5 erase key register"]
pub mod m5erkeyr;
