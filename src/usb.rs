#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CHEPR: [CHEPR; 8],
    _reserved1: [u8; 0x20],
    pub CNTR: CNTR,
    pub ISTR: ISTR,
    pub FNR: FNR,
    pub DADDR: DADDR,
    _reserved5: [u8; 0x04],
    pub LPMCSR: LPMCSR,
    pub BCDR: BCDR,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - USB endpoint/channel 0 register"]
    #[inline(always)]
    pub const fn CHEPR(&self, n: usize) -> &CHEPR {
        &self.CHEPR[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - USB endpoint/channel 0 register"]
    #[inline(always)]
    pub fn CHEPR_iter(&self) -> impl Iterator<Item = &CHEPR> {
        self.CHEPR.iter()
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn CNTR(&self) -> &CNTR {
        &self.CNTR
    }
    #[doc = "0x44 - USB interrupt status register"]
    #[inline(always)]
    pub const fn ISTR(&self) -> &ISTR {
        &self.ISTR
    }
    #[doc = "0x48 - USB frame number register"]
    #[inline(always)]
    pub const fn FNR(&self) -> &FNR {
        &self.FNR
    }
    #[doc = "0x4c - USB_DADDR"]
    #[inline(always)]
    pub const fn DADDR(&self) -> &DADDR {
        &self.DADDR
    }
    #[doc = "0x54 - USB_LPMCSR"]
    #[inline(always)]
    pub const fn LPMCSR(&self) -> &LPMCSR {
        &self.LPMCSR
    }
    #[doc = "0x58 - USB_BCDR"]
    #[inline(always)]
    pub const fn BCDR(&self) -> &BCDR {
        &self.BCDR
    }
}
#[doc = "CHEPR (rw) register accessor: USB endpoint/channel 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chepr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chepr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chepr`] module"]
pub type CHEPR = crate::Reg<chepr::CHEPR_SPEC>;
#[doc = "USB endpoint/channel 0 register"]
pub mod chepr;
#[doc = "CNTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`] module"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = ""]
pub mod cntr;
#[doc = "ISTR (rw) register accessor: USB interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istr`] module"]
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
#[doc = "USB interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: USB frame number register\n\nYou can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnr`] module"]
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
#[doc = "USB frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: USB_DADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`] module"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "USB_DADDR"]
pub mod daddr;
#[doc = "LPMCSR (rw) register accessor: USB_LPMCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmcsr`] module"]
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSR_SPEC>;
#[doc = "USB_LPMCSR"]
pub mod lpmcsr;
#[doc = "BCDR (rw) register accessor: USB_BCDR\n\nYou can [`read`](crate::Reg::read) this register and get [`bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdr`] module"]
pub type BCDR = crate::Reg<bcdr::BCDR_SPEC>;
#[doc = "USB_BCDR"]
pub mod bcdr;
