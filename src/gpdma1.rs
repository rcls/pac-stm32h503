#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    pub PRIVCFGR: PRIVCFGR,
    _reserved1: [u8; 0x04],
    pub MISR: MISR,
    _reserved2: [u8; 0x40],
    pub C: [C; 8],
}
impl RegisterBlock {
    #[doc = "0x04 - GPDMA privileged configuration register"]
    #[inline(always)]
    pub const fn PRIVCFGR(&self) -> &PRIVCFGR {
        &self.PRIVCFGR
    }
    #[doc = "0x0c - GPDMA masked interrupt status register"]
    #[inline(always)]
    pub const fn MISR(&self) -> &MISR {
        &self.MISR
    }
    #[doc = "0x50..0x450 - Cluster for C\\[%s\\]"]
    #[inline(always)]
    pub const fn C(&self, n: usize) -> &C {
        &self.C[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x450 - Cluster for C\\[%s\\]"]
    #[inline(always)]
    pub fn C_iter(&self) -> impl Iterator<Item = &C> {
        self.C.iter()
    }
}
#[doc = "PRIVCFGR (rw) register accessor: GPDMA privileged configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`] module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "GPDMA privileged configuration register"]
pub mod privcfgr;
#[doc = "MISR (r) register accessor: GPDMA masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`] module"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "GPDMA masked interrupt status register"]
pub mod misr;
#[doc = "Cluster for C\\[%s\\]"]
pub use self::c::C;
#[doc = r"Cluster"]
#[doc = "Cluster for C\\[%s\\]"]
pub mod c;
