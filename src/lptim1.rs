#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_ISR: [u8; 0x04],
    _reserved_1_ICR: [u8; 0x04],
    _reserved_2_DIER: [u8; 0x04],
    pub CFGR: CFGR,
    pub CR: CR,
    pub CCR1: CCR1,
    pub ARR: ARR,
    pub CNT: CNT,
    _reserved8: [u8; 0x08],
    pub RCR: RCR,
    pub CCMR1: CCMR1,
    _reserved10: [u8; 0x04],
    pub CCR2: CCR2,
}
impl RegisterBlock {
    #[doc = "0x00 - LPTIM1 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn ISR_intput(&self) -> &ISR_INTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - LPTIM1 interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn ISR_output(&self) -> &ISR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - LPTIM interrupt clear register"]
    #[inline(always)]
    pub const fn ICR_intput(&self) -> &ICR_INTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - LPTIM1 interrupt clear register \\[alternate\\]"]
    #[inline(always)]
    pub const fn ICR_output(&self) -> &ICR_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - LPTIM interrupt enable register"]
    #[inline(always)]
    pub const fn DIER_intput(&self) -> &DIER_INTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - LPTIM1 interrupt enable register \\[alternate\\]"]
    #[inline(always)]
    pub const fn DIER_output(&self) -> &DIER_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - LPTIM configuration register"]
    #[inline(always)]
    pub const fn CFGR(&self) -> &CFGR {
        &self.CFGR
    }
    #[doc = "0x10 - LPTIM control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x14 - LPTIM compare register 1"]
    #[inline(always)]
    pub const fn CCR1(&self) -> &CCR1 {
        &self.CCR1
    }
    #[doc = "0x18 - LPTIM autoreload register"]
    #[inline(always)]
    pub const fn ARR(&self) -> &ARR {
        &self.ARR
    }
    #[doc = "0x1c - LPTIM counter register"]
    #[inline(always)]
    pub const fn CNT(&self) -> &CNT {
        &self.CNT
    }
    #[doc = "0x28 - LPTIM repetition register"]
    #[inline(always)]
    pub const fn RCR(&self) -> &RCR {
        &self.RCR
    }
    #[doc = "0x2c - LPTIM capture/compare mode register 1"]
    #[inline(always)]
    pub const fn CCMR1(&self) -> &CCMR1 {
        &self.CCMR1
    }
    #[doc = "0x34 - LPTIM compare register 2"]
    #[inline(always)]
    pub const fn CCR2(&self) -> &CCR2 {
        &self.CCR2
    }
}
#[doc = "ISR_output (r) register accessor: LPTIM1 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`isr_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr_output`] module"]
#[doc(alias = "ISR_output")]
pub type ISR_OUTPUT = crate::Reg<isr_output::ISR_OUTPUT_SPEC>;
#[doc = "LPTIM1 interrupt and status register \\[alternate\\]"]
pub mod isr_output;
#[doc = "ISR_intput (r) register accessor: LPTIM1 interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`isr_intput::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr_intput`] module"]
#[doc(alias = "ISR_intput")]
pub type ISR_INTPUT = crate::Reg<isr_intput::ISR_INTPUT_SPEC>;
#[doc = "LPTIM1 interrupt and status register \\[alternate\\]"]
pub mod isr_intput;
#[doc = "ICR_output (w) register accessor: LPTIM1 interrupt clear register \\[alternate\\]\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr_output`] module"]
#[doc(alias = "ICR_output")]
pub type ICR_OUTPUT = crate::Reg<icr_output::ICR_OUTPUT_SPEC>;
#[doc = "LPTIM1 interrupt clear register \\[alternate\\]"]
pub mod icr_output;
#[doc = "ICR_intput (w) register accessor: LPTIM interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_intput::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr_intput`] module"]
#[doc(alias = "ICR_intput")]
pub type ICR_INTPUT = crate::Reg<icr_intput::ICR_INTPUT_SPEC>;
#[doc = "LPTIM interrupt clear register"]
pub mod icr_intput;
#[doc = "DIER_output (rw) register accessor: LPTIM1 interrupt enable register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`dier_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier_output`] module"]
#[doc(alias = "DIER_output")]
pub type DIER_OUTPUT = crate::Reg<dier_output::DIER_OUTPUT_SPEC>;
#[doc = "LPTIM1 interrupt enable register \\[alternate\\]"]
pub mod dier_output;
#[doc = "DIER_intput (rw) register accessor: LPTIM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier_intput::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_intput::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier_intput`] module"]
#[doc(alias = "DIER_intput")]
pub type DIER_INTPUT = crate::Reg<dier_intput::DIER_INTPUT_SPEC>;
#[doc = "LPTIM interrupt enable register"]
pub mod dier_intput;
#[doc = "CFGR (rw) register accessor: LPTIM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "LPTIM configuration register"]
pub mod cfgr;
#[doc = "CR (rw) register accessor: LPTIM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "LPTIM control register"]
pub mod cr;
#[doc = "CCR1 (rw) register accessor: LPTIM compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "LPTIM compare register 1"]
pub mod ccr1;
#[doc = "ARR (rw) register accessor: LPTIM autoreload register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "LPTIM autoreload register"]
pub mod arr;
#[doc = "CNT (r) register accessor: LPTIM counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "LPTIM counter register"]
pub mod cnt;
#[doc = "RCR (rw) register accessor: LPTIM repetition register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`] module"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "LPTIM repetition register"]
pub mod rcr;
#[doc = "CCMR1 (rw) register accessor: LPTIM capture/compare mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1`] module"]
pub type CCMR1 = crate::Reg<ccmr1::CCMR1_SPEC>;
#[doc = "LPTIM capture/compare mode register 1"]
pub mod ccmr1;
#[doc = "CCR2 (rw) register accessor: LPTIM compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "LPTIM compare register 2"]
pub mod ccr2;
