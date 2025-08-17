#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_CR1: [u8; 0x04],
    pub CR2: CR2,
    pub CR3: CR3,
    pub BRR: BRR,
    _reserved4: [u8; 0x08],
    pub RQR: RQR,
    _reserved_5_ISR: [u8; 0x04],
    pub ICR: ICR,
    pub RDR: RDR,
    pub TDR: TDR,
    pub PRESC: PRESC,
}
impl RegisterBlock {
    #[doc = "0x00 - LPUART control register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn CR1_disabled(&self) -> &CR1_DISABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - LPUART control register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn CR1_enabled(&self) -> &CR1_ENABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - LPUART control register 2"]
    #[inline(always)]
    pub const fn CR2(&self) -> &CR2 {
        &self.CR2
    }
    #[doc = "0x08 - LPUART control register 3"]
    #[inline(always)]
    pub const fn CR3(&self) -> &CR3 {
        &self.CR3
    }
    #[doc = "0x0c - LPUART baud rate register"]
    #[inline(always)]
    pub const fn BRR(&self) -> &BRR {
        &self.BRR
    }
    #[doc = "0x18 - LPUART request register"]
    #[inline(always)]
    pub const fn RQR(&self) -> &RQR {
        &self.RQR
    }
    #[doc = "0x1c - LPUART interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn ISR_disabled(&self) -> &ISR_DISABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - LPUART interrupt and status register \\[alternate\\]"]
    #[inline(always)]
    pub const fn ISR_enabled(&self) -> &ISR_ENABLED {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - LPUART interrupt flag clear register"]
    #[inline(always)]
    pub const fn ICR(&self) -> &ICR {
        &self.ICR
    }
    #[doc = "0x24 - LPUART receive data register"]
    #[inline(always)]
    pub const fn RDR(&self) -> &RDR {
        &self.RDR
    }
    #[doc = "0x28 - LPUART transmit data register"]
    #[inline(always)]
    pub const fn TDR(&self) -> &TDR {
        &self.TDR
    }
    #[doc = "0x2c - LPUART prescaler register"]
    #[inline(always)]
    pub const fn PRESC(&self) -> &PRESC {
        &self.PRESC
    }
}
#[doc = "CR1_enabled (rw) register accessor: LPUART control register 1 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1_enabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_enabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1_enabled`] module"]
#[doc(alias = "CR1_enabled")]
pub type CR1_ENABLED = crate::Reg<cr1_enabled::CR1_ENABLED_SPEC>;
#[doc = "LPUART control register 1 \\[alternate\\]"]
pub mod cr1_enabled;
#[doc = "CR1_disabled (rw) register accessor: LPUART control register 1 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1_disabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_disabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1_disabled`] module"]
#[doc(alias = "CR1_disabled")]
pub type CR1_DISABLED = crate::Reg<cr1_disabled::CR1_DISABLED_SPEC>;
#[doc = "LPUART control register 1 \\[alternate\\]"]
pub mod cr1_disabled;
#[doc = "CR2 (rw) register accessor: LPUART control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "LPUART control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: LPUART control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "LPUART control register 3"]
pub mod cr3;
#[doc = "BRR (rw) register accessor: LPUART baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "LPUART baud rate register"]
pub mod brr;
#[doc = "RQR (w) register accessor: LPUART request register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqr`] module"]
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
#[doc = "LPUART request register"]
pub mod rqr;
#[doc = "ISR_enabled (r) register accessor: LPUART interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`isr_enabled::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr_enabled`] module"]
#[doc(alias = "ISR_enabled")]
pub type ISR_ENABLED = crate::Reg<isr_enabled::ISR_ENABLED_SPEC>;
#[doc = "LPUART interrupt and status register \\[alternate\\]"]
pub mod isr_enabled;
#[doc = "ISR_disabled (r) register accessor: LPUART interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`isr_disabled::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr_disabled`] module"]
#[doc(alias = "ISR_disabled")]
pub type ISR_DISABLED = crate::Reg<isr_disabled::ISR_DISABLED_SPEC>;
#[doc = "LPUART interrupt and status register \\[alternate\\]"]
pub mod isr_disabled;
#[doc = "ICR (w) register accessor: LPUART interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "LPUART interrupt flag clear register"]
pub mod icr;
#[doc = "RDR (r) register accessor: LPUART receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`] module"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "LPUART receive data register"]
pub mod rdr;
#[doc = "TDR (rw) register accessor: LPUART transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`] module"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "LPUART transmit data register"]
pub mod tdr;
#[doc = "PRESC (rw) register accessor: LPUART prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`presc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presc`] module"]
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
#[doc = "LPUART prescaler register"]
pub mod presc;
