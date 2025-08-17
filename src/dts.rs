#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CFGR1: CFGR1,
    _reserved1: [u8; 0x04],
    pub T0VALR1: T0VALR1,
    _reserved2: [u8; 0x04],
    pub RAMPVALR: RAMPVALR,
    pub ITR1: ITR1,
    _reserved4: [u8; 0x04],
    pub DR: DR,
    pub SR: SR,
    pub ITENR: ITENR,
    pub ICIFR: ICIFR,
    pub OR: OR,
}
impl RegisterBlock {
    #[doc = "0x00 - Temperature sensor configuration register 1"]
    #[inline(always)]
    pub const fn CFGR1(&self) -> &CFGR1 {
        &self.CFGR1
    }
    #[doc = "0x08 - Temperature sensor T0 value register 1"]
    #[inline(always)]
    pub const fn T0VALR1(&self) -> &T0VALR1 {
        &self.T0VALR1
    }
    #[doc = "0x10 - Temperature sensor ramp value register"]
    #[inline(always)]
    pub const fn RAMPVALR(&self) -> &RAMPVALR {
        &self.RAMPVALR
    }
    #[doc = "0x14 - Temperature sensor interrupt threshold register 1"]
    #[inline(always)]
    pub const fn ITR1(&self) -> &ITR1 {
        &self.ITR1
    }
    #[doc = "0x1c - Temperature sensor data register"]
    #[inline(always)]
    pub const fn DR(&self) -> &DR {
        &self.DR
    }
    #[doc = "0x20 - Temperature sensor status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x24 - Temperature sensor interrupt enable register"]
    #[inline(always)]
    pub const fn ITENR(&self) -> &ITENR {
        &self.ITENR
    }
    #[doc = "0x28 - Temperature sensor clear interrupt flag register"]
    #[inline(always)]
    pub const fn ICIFR(&self) -> &ICIFR {
        &self.ICIFR
    }
    #[doc = "0x2c - Temperature sensor option register"]
    #[inline(always)]
    pub const fn OR(&self) -> &OR {
        &self.OR
    }
}
#[doc = "CFGR1 (rw) register accessor: Temperature sensor configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`] module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "Temperature sensor configuration register 1"]
pub mod cfgr1;
#[doc = "T0VALR1 (r) register accessor: Temperature sensor T0 value register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`t0valr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0valr1`] module"]
pub type T0VALR1 = crate::Reg<t0valr1::T0VALR1_SPEC>;
#[doc = "Temperature sensor T0 value register 1"]
pub mod t0valr1;
#[doc = "RAMPVALR (r) register accessor: Temperature sensor ramp value register\n\nYou can [`read`](crate::Reg::read) this register and get [`rampvalr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rampvalr`] module"]
pub type RAMPVALR = crate::Reg<rampvalr::RAMPVALR_SPEC>;
#[doc = "Temperature sensor ramp value register"]
pub mod rampvalr;
#[doc = "ITR1 (rw) register accessor: Temperature sensor interrupt threshold register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`itr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itr1`] module"]
pub type ITR1 = crate::Reg<itr1::ITR1_SPEC>;
#[doc = "Temperature sensor interrupt threshold register 1"]
pub mod itr1;
#[doc = "DR (rw) register accessor: Temperature sensor data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Temperature sensor data register"]
pub mod dr;
#[doc = "SR (r) register accessor: Temperature sensor status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Temperature sensor status register"]
pub mod sr;
#[doc = "ITENR (rw) register accessor: Temperature sensor interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`itenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itenr`] module"]
pub type ITENR = crate::Reg<itenr::ITENR_SPEC>;
#[doc = "Temperature sensor interrupt enable register"]
pub mod itenr;
#[doc = "ICIFR (rw) register accessor: Temperature sensor clear interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`icifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icifr`] module"]
pub type ICIFR = crate::Reg<icifr::ICIFR_SPEC>;
#[doc = "Temperature sensor clear interrupt flag register"]
pub mod icifr;
#[doc = "OR (rw) register accessor: Temperature sensor option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`] module"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "Temperature sensor option register"]
pub mod or;
