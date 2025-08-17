#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR1: CR1,
    _reserved1: [u8; 0x02],
    pub CR2: CR2,
    pub SMCR: SMCR,
    pub DIER: DIER,
    pub SR: SR,
    pub EGR: EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_CCMR1: [u8; 0x04],
    _reserved_7_CCMR2: [u8; 0x04],
    pub CCER: CCER,
    pub CNT: CNT,
    pub PSC: PSC,
    _reserved11: [u8; 0x02],
    pub ARR: ARR,
    pub RCR: RCR,
    _reserved13: [u8; 0x02],
    pub CCR1: CCR1,
    pub CCR2: CCR2,
    pub CCR3: CCR3,
    pub CCR4: CCR4,
    pub BDTR: BDTR,
    pub CCR5: CCR5,
    pub CCR6: CCR6,
    pub CCMR3: CCMR3,
    pub DTR2: DTR2,
    pub ECR: ECR,
    pub TISEL: TISEL,
    pub AF1: AF1,
    pub AF2: AF2,
    _reserved26: [u8; 0x0374],
    pub DCR: DCR,
    pub DMAR: DMAR,
}
impl RegisterBlock {
    #[doc = "0x00 - TIM1 control register 1"]
    #[inline(always)]
    pub const fn CR1(&self) -> &CR1 {
        &self.CR1
    }
    #[doc = "0x04 - TIM1 control register 2"]
    #[inline(always)]
    pub const fn CR2(&self) -> &CR2 {
        &self.CR2
    }
    #[doc = "0x08 - TIM1 slave mode control register"]
    #[inline(always)]
    pub const fn SMCR(&self) -> &SMCR {
        &self.SMCR
    }
    #[doc = "0x0c - TIM1 DMA/interrupt enable register"]
    #[inline(always)]
    pub const fn DIER(&self) -> &DIER {
        &self.DIER
    }
    #[doc = "0x10 - TIM1 status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x14 - TIM1 event generation register"]
    #[inline(always)]
    pub const fn EGR(&self) -> &EGR {
        &self.EGR
    }
    #[doc = "0x18 - TIM1 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn CCMR1_Output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - TIM1 capture/compare mode register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn CCMR1_Input(&self) -> &CCMR1_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn CCMR2_Output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - TIM1 capture/compare mode register 2 \\[alternate\\]"]
    #[inline(always)]
    pub const fn CCMR2_Input(&self) -> &CCMR2_INPUT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - TIM1 capture/compare enable register"]
    #[inline(always)]
    pub const fn CCER(&self) -> &CCER {
        &self.CCER
    }
    #[doc = "0x24 - TIM1 counter"]
    #[inline(always)]
    pub const fn CNT(&self) -> &CNT {
        &self.CNT
    }
    #[doc = "0x28 - TIM1 prescaler"]
    #[inline(always)]
    pub const fn PSC(&self) -> &PSC {
        &self.PSC
    }
    #[doc = "0x2c - TIM1 auto-reload register"]
    #[inline(always)]
    pub const fn ARR(&self) -> &ARR {
        &self.ARR
    }
    #[doc = "0x30 - TIM1 repetition counter register"]
    #[inline(always)]
    pub const fn RCR(&self) -> &RCR {
        &self.RCR
    }
    #[doc = "0x34 - TIM1 capture/compare register 1"]
    #[inline(always)]
    pub const fn CCR1(&self) -> &CCR1 {
        &self.CCR1
    }
    #[doc = "0x38 - TIM1 capture/compare register 2"]
    #[inline(always)]
    pub const fn CCR2(&self) -> &CCR2 {
        &self.CCR2
    }
    #[doc = "0x3c - TIM1 capture/compare register 3"]
    #[inline(always)]
    pub const fn CCR3(&self) -> &CCR3 {
        &self.CCR3
    }
    #[doc = "0x40 - TIM1 capture/compare register 4"]
    #[inline(always)]
    pub const fn CCR4(&self) -> &CCR4 {
        &self.CCR4
    }
    #[doc = "0x44 - TIM1 break and dead-time register"]
    #[inline(always)]
    pub const fn BDTR(&self) -> &BDTR {
        &self.BDTR
    }
    #[doc = "0x48 - TIM1 capture/compare register 5"]
    #[inline(always)]
    pub const fn CCR5(&self) -> &CCR5 {
        &self.CCR5
    }
    #[doc = "0x4c - TIM1 capture/compare register 6"]
    #[inline(always)]
    pub const fn CCR6(&self) -> &CCR6 {
        &self.CCR6
    }
    #[doc = "0x50 - TIM1 capture/compare mode register 3"]
    #[inline(always)]
    pub const fn CCMR3(&self) -> &CCMR3 {
        &self.CCMR3
    }
    #[doc = "0x54 - TIM1 timer deadtime register 2"]
    #[inline(always)]
    pub const fn DTR2(&self) -> &DTR2 {
        &self.DTR2
    }
    #[doc = "0x58 - TIM1 timer encoder control register"]
    #[inline(always)]
    pub const fn ECR(&self) -> &ECR {
        &self.ECR
    }
    #[doc = "0x5c - TIM1 timer input selection register"]
    #[inline(always)]
    pub const fn TISEL(&self) -> &TISEL {
        &self.TISEL
    }
    #[doc = "0x60 - TIM1 alternate function option register 1"]
    #[inline(always)]
    pub const fn AF1(&self) -> &AF1 {
        &self.AF1
    }
    #[doc = "0x64 - TIM1 alternate function register 2"]
    #[inline(always)]
    pub const fn AF2(&self) -> &AF2 {
        &self.AF2
    }
    #[doc = "0x3dc - TIM1 DMA control register"]
    #[inline(always)]
    pub const fn DCR(&self) -> &DCR {
        &self.DCR
    }
    #[doc = "0x3e0 - TIM1 DMA address for full transfer"]
    #[inline(always)]
    pub const fn DMAR(&self) -> &DMAR {
        &self.DMAR
    }
}
#[doc = "CR1 (rw) register accessor: TIM1 control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "TIM1 control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: TIM1 control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "TIM1 control register 2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: TIM1 slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`] module"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "TIM1 slave mode control register"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: TIM1 DMA/interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`] module"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "TIM1 DMA/interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: TIM1 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TIM1 status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: TIM1 event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`] module"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "TIM1 event generation register"]
pub mod egr;
#[doc = "CCMR1_Input (rw) register accessor: TIM1 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_input`] module"]
#[doc(alias = "CCMR1_Input")]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_input;
#[doc = "CCMR1_Output (rw) register accessor: TIM1 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_output`] module"]
#[doc(alias = "CCMR1_Output")]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 1 \\[alternate\\]"]
pub mod ccmr1_output;
#[doc = "CCMR2_Input (rw) register accessor: TIM1 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_input`] module"]
#[doc(alias = "CCMR2_Input")]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]"]
pub mod ccmr2_input;
#[doc = "CCMR2_Output (rw) register accessor: TIM1 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_output`] module"]
#[doc(alias = "CCMR2_Output")]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
#[doc = "TIM1 capture/compare mode register 2 \\[alternate\\]"]
pub mod ccmr2_output;
#[doc = "CCER (rw) register accessor: TIM1 capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`] module"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "TIM1 capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: TIM1 counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "TIM1 counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: TIM1 prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "TIM1 prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: TIM1 auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "TIM1 auto-reload register"]
pub mod arr;
#[doc = "RCR (rw) register accessor: TIM1 repetition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`] module"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "TIM1 repetition counter register"]
pub mod rcr;
#[doc = "CCR1 (rw) register accessor: TIM1 capture/compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "TIM1 capture/compare register 1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: TIM1 capture/compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "TIM1 capture/compare register 2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: TIM1 capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "TIM1 capture/compare register 3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: TIM1 capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`] module"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "TIM1 capture/compare register 4"]
pub mod ccr4;
#[doc = "BDTR (rw) register accessor: TIM1 break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtr`] module"]
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
#[doc = "TIM1 break and dead-time register"]
pub mod bdtr;
#[doc = "CCR5 (rw) register accessor: TIM1 capture/compare register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5`] module"]
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
#[doc = "TIM1 capture/compare register 5"]
pub mod ccr5;
#[doc = "CCR6 (rw) register accessor: TIM1 capture/compare register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr6`] module"]
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
#[doc = "TIM1 capture/compare register 6"]
pub mod ccr6;
#[doc = "CCMR3 (rw) register accessor: TIM1 capture/compare mode register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr3`] module"]
pub type CCMR3 = crate::Reg<ccmr3::CCMR3_SPEC>;
#[doc = "TIM1 capture/compare mode register 3"]
pub mod ccmr3;
#[doc = "DTR2 (rw) register accessor: TIM1 timer deadtime register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtr2`] module"]
pub type DTR2 = crate::Reg<dtr2::DTR2_SPEC>;
#[doc = "TIM1 timer deadtime register 2"]
pub mod dtr2;
#[doc = "ECR (rw) register accessor: TIM1 timer encoder control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "TIM1 timer encoder control register"]
pub mod ecr;
#[doc = "TISEL (rw) register accessor: TIM1 timer input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisel`] module"]
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
#[doc = "TIM1 timer input selection register"]
pub mod tisel;
#[doc = "AF1 (rw) register accessor: TIM1 alternate function option register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af1`] module"]
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
#[doc = "TIM1 alternate function option register 1"]
pub mod af1;
#[doc = "AF2 (rw) register accessor: TIM1 alternate function register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af2`] module"]
pub type AF2 = crate::Reg<af2::AF2_SPEC>;
#[doc = "TIM1 alternate function register 2"]
pub mod af2;
#[doc = "DCR (rw) register accessor: TIM1 DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`] module"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "TIM1 DMA control register"]
pub mod dcr;
#[doc = "DMAR (rw) register accessor: TIM1 DMA address for full transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`dmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmar`] module"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "TIM1 DMA address for full transfer"]
pub mod dmar;
