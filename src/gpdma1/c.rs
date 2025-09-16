#[repr(C)]
#[doc = "Cluster for C\\[%s\\]"]
pub struct C {
    pub LBAR: LBAR,
    _reserved1: [u8; 0x3c],
    pub TR1: TR1,
    pub TR2: TR2,
    pub BR1: BR1,
    pub SAR: SAR,
    pub DAR: DAR,
    _reserved6: [u8; 0x28],
    pub LLR: LLR,
}
impl C {
    #[doc = "0x00 - GPDMA channel 0 linked-list base address register"]
    #[inline(always)]
    pub const fn LBAR(&self) -> &LBAR {
        &self.LBAR
    }
    #[doc = "0x40 - GPDMA channel 0 transfer register 1"]
    #[inline(always)]
    pub const fn TR1(&self) -> &TR1 {
        &self.TR1
    }
    #[doc = "0x44 - GPDMA channel 0 transfer register 2"]
    #[inline(always)]
    pub const fn TR2(&self) -> &TR2 {
        &self.TR2
    }
    #[doc = "0x48 - GPDMA channel 0 block register 1"]
    #[inline(always)]
    pub const fn BR1(&self) -> &BR1 {
        &self.BR1
    }
    #[doc = "0x4c - GPDMA channel 0 source address register"]
    #[inline(always)]
    pub const fn SAR(&self) -> &SAR {
        &self.SAR
    }
    #[doc = "0x50 - GPDMA channel 0 destination address register"]
    #[inline(always)]
    pub const fn DAR(&self) -> &DAR {
        &self.DAR
    }
    #[doc = "0x7c - GPDMA channel 0 linked-list address register"]
    #[inline(always)]
    pub const fn LLR(&self) -> &LLR {
        &self.LLR
    }
}
#[doc = "LBAR (rw) register accessor: GPDMA channel 0 linked-list base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`lbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lbar`] module"]
pub type LBAR = crate::Reg<lbar::LBAR_SPEC>;
#[doc = "GPDMA channel 0 linked-list base address register"]
pub mod lbar;
#[doc = "TR1 (rw) register accessor: GPDMA channel 0 transfer register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr1`] module"]
pub type TR1 = crate::Reg<tr1::TR1_SPEC>;
#[doc = "GPDMA channel 0 transfer register 1"]
pub mod tr1;
#[doc = "TR2 (rw) register accessor: GPDMA channel 0 transfer register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr2`] module"]
pub type TR2 = crate::Reg<tr2::TR2_SPEC>;
#[doc = "GPDMA channel 0 transfer register 2"]
pub mod tr2;
#[doc = "BR1 (rw) register accessor: GPDMA channel 0 block register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`br1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@br1`] module"]
pub type BR1 = crate::Reg<br1::BR1_SPEC>;
#[doc = "GPDMA channel 0 block register 1"]
pub mod br1;
#[doc = "SAR (rw) register accessor: GPDMA channel 0 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`] module"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "GPDMA channel 0 source address register"]
pub mod sar;
#[doc = "DAR (rw) register accessor: GPDMA channel 0 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar`] module"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "GPDMA channel 0 destination address register"]
pub mod dar;
#[doc = "LLR (rw) register accessor: GPDMA channel 0 linked-list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`llr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llr`] module"]
pub type LLR = crate::Reg<llr::LLR_SPEC>;
#[doc = "GPDMA channel 0 linked-list address register"]
pub mod llr;
