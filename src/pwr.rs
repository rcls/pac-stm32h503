#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub PMCR: PMCR,
    pub PMSR: PMSR,
    _reserved2: [u8; 0x08],
    pub VOSCR: VOSCR,
    pub VOSSR: VOSSR,
    _reserved4: [u8; 0x08],
    pub BDCR: BDCR,
    pub DBPCR: DBPCR,
    pub BDSR: BDSR,
    _reserved7: [u8; 0x04],
    pub SCCR: SCCR,
    pub VMCR: VMCR,
    _reserved9: [u8; 0x04],
    pub VMSR: VMSR,
    pub WUSCR: WUSCR,
    pub WUSR: WUSR,
    pub WUCR: WUCR,
    _reserved13: [u8; 0x04],
    pub IORETR: IORETR,
    _reserved14: [u8; 0xb0],
    pub PRIVCFGR: PRIVCFGR,
}
impl RegisterBlock {
    #[doc = "0x00 - PWR power mode control register"]
    #[inline(always)]
    pub const fn PMCR(&self) -> &PMCR {
        &self.PMCR
    }
    #[doc = "0x04 - PWR status register"]
    #[inline(always)]
    pub const fn PMSR(&self) -> &PMSR {
        &self.PMSR
    }
    #[doc = "0x10 - PWR voltage scaling control register"]
    #[inline(always)]
    pub const fn VOSCR(&self) -> &VOSCR {
        &self.VOSCR
    }
    #[doc = "0x14 - PWR voltage scaling status register"]
    #[inline(always)]
    pub const fn VOSSR(&self) -> &VOSSR {
        &self.VOSSR
    }
    #[doc = "0x20 - PWR Backup domain control register"]
    #[inline(always)]
    pub const fn BDCR(&self) -> &BDCR {
        &self.BDCR
    }
    #[doc = "0x24 - PWR disable backup protection control register"]
    #[inline(always)]
    pub const fn DBPCR(&self) -> &DBPCR {
        &self.DBPCR
    }
    #[doc = "0x28 - PWR Backup domain status register"]
    #[inline(always)]
    pub const fn BDSR(&self) -> &BDSR {
        &self.BDSR
    }
    #[doc = "0x30 - PWR supply configuration control register"]
    #[inline(always)]
    pub const fn SCCR(&self) -> &SCCR {
        &self.SCCR
    }
    #[doc = "0x34 - PWR voltage monitor control register"]
    #[inline(always)]
    pub const fn VMCR(&self) -> &VMCR {
        &self.VMCR
    }
    #[doc = "0x3c - PWR voltage monitor status register"]
    #[inline(always)]
    pub const fn VMSR(&self) -> &VMSR {
        &self.VMSR
    }
    #[doc = "0x40 - PWR wakeup status clear register"]
    #[inline(always)]
    pub const fn WUSCR(&self) -> &WUSCR {
        &self.WUSCR
    }
    #[doc = "0x44 - PWR wakeup status register"]
    #[inline(always)]
    pub const fn WUSR(&self) -> &WUSR {
        &self.WUSR
    }
    #[doc = "0x48 - PWR wakeup configuration register"]
    #[inline(always)]
    pub const fn WUCR(&self) -> &WUCR {
        &self.WUCR
    }
    #[doc = "0x50 - PWR I/O retention register"]
    #[inline(always)]
    pub const fn IORETR(&self) -> &IORETR {
        &self.IORETR
    }
    #[doc = "0x104 - PWR privilege configuration register"]
    #[inline(always)]
    pub const fn PRIVCFGR(&self) -> &PRIVCFGR {
        &self.PRIVCFGR
    }
}
#[doc = "PMCR (rw) register accessor: PWR power mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmcr`] module"]
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
#[doc = "PWR power mode control register"]
pub mod pmcr;
#[doc = "PMSR (r) register accessor: PWR status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmsr`] module"]
pub type PMSR = crate::Reg<pmsr::PMSR_SPEC>;
#[doc = "PWR status register"]
pub mod pmsr;
#[doc = "VOSCR (rw) register accessor: PWR voltage scaling control register\n\nYou can [`read`](crate::Reg::read) this register and get [`voscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`voscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@voscr`] module"]
pub type VOSCR = crate::Reg<voscr::VOSCR_SPEC>;
#[doc = "PWR voltage scaling control register"]
pub mod voscr;
#[doc = "VOSSR (r) register accessor: PWR voltage scaling status register\n\nYou can [`read`](crate::Reg::read) this register and get [`vossr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vossr`] module"]
pub type VOSSR = crate::Reg<vossr::VOSSR_SPEC>;
#[doc = "PWR voltage scaling status register"]
pub mod vossr;
#[doc = "BDCR (rw) register accessor: PWR Backup domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`] module"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "PWR Backup domain control register"]
pub mod bdcr;
#[doc = "DBPCR (rw) register accessor: PWR disable backup protection control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbpcr`] module"]
pub type DBPCR = crate::Reg<dbpcr::DBPCR_SPEC>;
#[doc = "PWR disable backup protection control register"]
pub mod dbpcr;
#[doc = "BDSR (r) register accessor: PWR Backup domain status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdsr`] module"]
pub type BDSR = crate::Reg<bdsr::BDSR_SPEC>;
#[doc = "PWR Backup domain status register"]
pub mod bdsr;
#[doc = "SCCR (rw) register accessor: PWR supply configuration control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccr`] module"]
pub type SCCR = crate::Reg<sccr::SCCR_SPEC>;
#[doc = "PWR supply configuration control register"]
pub mod sccr;
#[doc = "VMCR (rw) register accessor: PWR voltage monitor control register\n\nYou can [`read`](crate::Reg::read) this register and get [`vmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmcr`] module"]
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
#[doc = "PWR voltage monitor control register"]
pub mod vmcr;
#[doc = "VMSR (r) register accessor: PWR voltage monitor status register\n\nYou can [`read`](crate::Reg::read) this register and get [`vmsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmsr`] module"]
pub type VMSR = crate::Reg<vmsr::VMSR_SPEC>;
#[doc = "PWR voltage monitor status register"]
pub mod vmsr;
#[doc = "WUSCR (w) register accessor: PWR wakeup status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wuscr`] module"]
pub type WUSCR = crate::Reg<wuscr::WUSCR_SPEC>;
#[doc = "PWR wakeup status clear register"]
pub mod wuscr;
#[doc = "WUSR (r) register accessor: PWR wakeup status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wusr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wusr`] module"]
pub type WUSR = crate::Reg<wusr::WUSR_SPEC>;
#[doc = "PWR wakeup status register"]
pub mod wusr;
#[doc = "WUCR (rw) register accessor: PWR wakeup configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`wucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wucr`] module"]
pub type WUCR = crate::Reg<wucr::WUCR_SPEC>;
#[doc = "PWR wakeup configuration register"]
pub mod wucr;
#[doc = "IORETR (rw) register accessor: PWR I/O retention register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioretr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioretr`] module"]
pub type IORETR = crate::Reg<ioretr::IORETR_SPEC>;
#[doc = "PWR I/O retention register"]
pub mod ioretr;
#[doc = "PRIVCFGR (rw) register accessor: PWR privilege configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`] module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "PWR privilege configuration register"]
pub mod privcfgr;
