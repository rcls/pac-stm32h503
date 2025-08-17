#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR: CR,
    pub CFR: CFR,
    pub SR: SR,
}
impl RegisterBlock {
    #[doc = "0x00 - WWDG control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x04 - WWDG configuration register"]
    #[inline(always)]
    pub const fn CFR(&self) -> &CFR {
        &self.CFR
    }
    #[doc = "0x08 - WWDG status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
}
#[doc = "CR (rw) register accessor: WWDG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "WWDG control register"]
pub mod cr;
#[doc = "CFR (rw) register accessor: WWDG configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`] module"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "WWDG configuration register"]
pub mod cfr;
#[doc = "SR (rw) register accessor: WWDG status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "WWDG status register"]
pub mod sr;
