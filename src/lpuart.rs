#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR1: CR1,
    pub CR2: CR2,
    pub CR3: CR3,
    pub BRR: BRR,
    _reserved4: [u8; 0x08],
    pub RQR: RQR,
    pub ISR: ISR,
    pub ICR: ICR,
    pub RDR: RDR,
    pub TDR: TDR,
    pub PRESC: PRESC,
}
impl RegisterBlock {
    #[doc = "0x00 - LPUART control register 1 \\[alternate\\]"]
    #[inline(always)]
    pub const fn CR1(&self) -> &CR1 {
        &self.CR1
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
    pub const fn ISR(&self) -> &ISR {
        &self.ISR
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
#[doc = "CR1 (rw) register accessor: LPUART control register 1 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "LPUART control register 1 \\[alternate\\]"]
pub mod cr1;
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
#[doc = "ISR (r) register accessor: LPUART interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "LPUART interrupt and status register \\[alternate\\]"]
pub mod isr;
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
