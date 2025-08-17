#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub OPAMP1_CSR: OPAMP1_CSR,
    pub OPAMP1_OTR: OPAMP1_OTR,
    pub OPAMP1_HSOTR: OPAMP1_HSOTR,
    pub OPAMP_OR: OPAMP_OR,
}
impl RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    #[inline(always)]
    pub const fn OPAMP1_CSR(&self) -> &OPAMP1_CSR {
        &self.OPAMP1_CSR
    }
    #[doc = "0x04 - OPAMP1 trimming register in normal mode"]
    #[inline(always)]
    pub const fn OPAMP1_OTR(&self) -> &OPAMP1_OTR {
        &self.OPAMP1_OTR
    }
    #[doc = "0x08 - OPAMP1 trimming register in high-speed mode"]
    #[inline(always)]
    pub const fn OPAMP1_HSOTR(&self) -> &OPAMP1_HSOTR {
        &self.OPAMP1_HSOTR
    }
    #[doc = "0x0c - OPAMP option register"]
    #[inline(always)]
    pub const fn OPAMP_OR(&self) -> &OPAMP_OR {
        &self.OPAMP_OR
    }
}
#[doc = "OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_csr`] module"]
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP1_OTR (rw) register accessor: OPAMP1 trimming register in normal mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_otr`] module"]
pub type OPAMP1_OTR = crate::Reg<opamp1_otr::OPAMP1_OTR_SPEC>;
#[doc = "OPAMP1 trimming register in normal mode"]
pub mod opamp1_otr;
#[doc = "OPAMP1_HSOTR (rw) register accessor: OPAMP1 trimming register in high-speed mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_hsotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_hsotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_hsotr`] module"]
pub type OPAMP1_HSOTR = crate::Reg<opamp1_hsotr::OPAMP1_HSOTR_SPEC>;
#[doc = "OPAMP1 trimming register in high-speed mode"]
pub mod opamp1_hsotr;
#[doc = "OPAMP_OR (rw) register accessor: OPAMP option register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp_or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp_or`] module"]
pub type OPAMP_OR = crate::Reg<opamp_or::OPAMP_OR_SPEC>;
#[doc = "OPAMP option register"]
pub mod opamp_or;
