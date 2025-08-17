#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    pub HDPLCR: HDPLCR,
    pub HDPLSR: HDPLSR,
    _reserved2: [u8; 0x08],
    pub DBGCR: DBGCR,
    pub DBGLOCKR: DBGLOCKR,
    _reserved4: [u8; 0xd8],
    pub PMCR: PMCR,
    pub FPUIMR: FPUIMR,
    pub MESR: MESR,
    _reserved7: [u8; 0x04],
    pub CCCSR: CCCSR,
    pub CCVALR: CCVALR,
    pub CCSWCR: CCSWCR,
    _reserved10: [u8; 0x04],
    pub CFGR2: CFGR2,
    _reserved11: [u8; 0x20],
    pub CNSLCKR: CNSLCKR,
    _reserved12: [u8; 0x04],
    pub ECCNMIR: ECCNMIR,
}
impl RegisterBlock {
    #[doc = "0x10 - SBS temporal isolation control register"]
    #[inline(always)]
    pub const fn HDPLCR(&self) -> &HDPLCR {
        &self.HDPLCR
    }
    #[doc = "0x14 - SBS temporal isolation status register"]
    #[inline(always)]
    pub const fn HDPLSR(&self) -> &HDPLSR {
        &self.HDPLSR
    }
    #[doc = "0x20 - SBS debug control register"]
    #[inline(always)]
    pub const fn DBGCR(&self) -> &DBGCR {
        &self.DBGCR
    }
    #[doc = "0x24 - SBS debug lock register"]
    #[inline(always)]
    pub const fn DBGLOCKR(&self) -> &DBGLOCKR {
        &self.DBGLOCKR
    }
    #[doc = "0x100 - SBS product mode and configuration register"]
    #[inline(always)]
    pub const fn PMCR(&self) -> &PMCR {
        &self.PMCR
    }
    #[doc = "0x104 - SBS FPU interrupt mask register"]
    #[inline(always)]
    pub const fn FPUIMR(&self) -> &FPUIMR {
        &self.FPUIMR
    }
    #[doc = "0x108 - SBS memory erase status register"]
    #[inline(always)]
    pub const fn MESR(&self) -> &MESR {
        &self.MESR
    }
    #[doc = "0x110 - SBS compensation cell for I/Os control and status register"]
    #[inline(always)]
    pub const fn CCCSR(&self) -> &CCCSR {
        &self.CCCSR
    }
    #[doc = "0x114 - SBS compensation cell for I/Os value register"]
    #[inline(always)]
    pub const fn CCVALR(&self) -> &CCVALR {
        &self.CCVALR
    }
    #[doc = "0x118 - SBS compensation cell for I/Os software code register"]
    #[inline(always)]
    pub const fn CCSWCR(&self) -> &CCSWCR {
        &self.CCSWCR
    }
    #[doc = "0x120 - SBS Class B register"]
    #[inline(always)]
    pub const fn CFGR2(&self) -> &CFGR2 {
        &self.CFGR2
    }
    #[doc = "0x144 - SBS CPU lock register"]
    #[inline(always)]
    pub const fn CNSLCKR(&self) -> &CNSLCKR {
        &self.CNSLCKR
    }
    #[doc = "0x14c - SBS flift ECC NMI mask register"]
    #[inline(always)]
    pub const fn ECCNMIR(&self) -> &ECCNMIR {
        &self.ECCNMIR
    }
}
#[doc = "HDPLCR (rw) register accessor: SBS temporal isolation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdplcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdplcr`] module"]
pub type HDPLCR = crate::Reg<hdplcr::HDPLCR_SPEC>;
#[doc = "SBS temporal isolation control register"]
pub mod hdplcr;
#[doc = "HDPLSR (r) register accessor: SBS temporal isolation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdplsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdplsr`] module"]
pub type HDPLSR = crate::Reg<hdplsr::HDPLSR_SPEC>;
#[doc = "SBS temporal isolation status register"]
pub mod hdplsr;
#[doc = "DBGCR (rw) register accessor: SBS debug control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgcr`] module"]
pub type DBGCR = crate::Reg<dbgcr::DBGCR_SPEC>;
#[doc = "SBS debug control register"]
pub mod dbgcr;
#[doc = "DBGLOCKR (rw) register accessor: SBS debug lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbglockr`] module"]
pub type DBGLOCKR = crate::Reg<dbglockr::DBGLOCKR_SPEC>;
#[doc = "SBS debug lock register"]
pub mod dbglockr;
#[doc = "PMCR (rw) register accessor: SBS product mode and configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmcr`] module"]
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
#[doc = "SBS product mode and configuration register"]
pub mod pmcr;
#[doc = "FPUIMR (rw) register accessor: SBS FPU interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpuimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpuimr`] module"]
pub type FPUIMR = crate::Reg<fpuimr::FPUIMR_SPEC>;
#[doc = "SBS FPU interrupt mask register"]
pub mod fpuimr;
#[doc = "MESR (rw) register accessor: SBS memory erase status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mesr`] module"]
pub type MESR = crate::Reg<mesr::MESR_SPEC>;
#[doc = "SBS memory erase status register"]
pub mod mesr;
#[doc = "CCCSR (rw) register accessor: SBS compensation cell for I/Os control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccsr`] module"]
pub type CCCSR = crate::Reg<cccsr::CCCSR_SPEC>;
#[doc = "SBS compensation cell for I/Os control and status register"]
pub mod cccsr;
#[doc = "CCVALR (r) register accessor: SBS compensation cell for I/Os value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccvalr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccvalr`] module"]
pub type CCVALR = crate::Reg<ccvalr::CCVALR_SPEC>;
#[doc = "SBS compensation cell for I/Os value register"]
pub mod ccvalr;
#[doc = "CCSWCR (rw) register accessor: SBS compensation cell for I/Os software code register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccswcr`] module"]
pub type CCSWCR = crate::Reg<ccswcr::CCSWCR_SPEC>;
#[doc = "SBS compensation cell for I/Os software code register"]
pub mod ccswcr;
#[doc = "CFGR2 (rw) register accessor: SBS Class B register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "SBS Class B register"]
pub mod cfgr2;
#[doc = "CNSLCKR (rw) register accessor: SBS CPU lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnslckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnslckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnslckr`] module"]
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKR_SPEC>;
#[doc = "SBS CPU lock register"]
pub mod cnslckr;
#[doc = "ECCNMIR (rw) register accessor: SBS flift ECC NMI mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccnmir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccnmir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccnmir`] module"]
pub type ECCNMIR = crate::Reg<eccnmir::ECCNMIR_SPEC>;
#[doc = "SBS flift ECC NMI mask register"]
pub mod eccnmir;
