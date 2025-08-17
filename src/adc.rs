#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub ISR: ISR,
    pub IER: IER,
    pub CR: CR,
    pub CFGR: CFGR,
    pub CFGR2: CFGR2,
    pub SMPR1: SMPR1,
    pub SMPR2: SMPR2,
    _reserved7: [u8; 0x04],
    pub TR1: TR1,
    pub TR2: TR2,
    pub TR3: TR3,
    _reserved10: [u8; 0x04],
    pub SQR1: SQR1,
    pub SQR2: SQR2,
    pub SQR3: SQR3,
    pub SQR4: SQR4,
    pub DR: DR,
    _reserved15: [u8; 0x08],
    pub JSQR: JSQR,
    _reserved16: [u8; 0x10],
    pub OFR1: OFR1,
    pub OFR2: OFR2,
    pub OFR3: OFR3,
    pub OFR4: OFR4,
    _reserved20: [u8; 0x10],
    pub JDR1: JDR1,
    pub JDR2: JDR2,
    pub JDR3: JDR3,
    pub JDR4: JDR4,
    _reserved24: [u8; 0x10],
    pub AWD2CR: AWD2CR,
    pub AWD3CR: AWD3CR,
    _reserved26: [u8; 0x08],
    pub DIFSEL: DIFSEL,
    pub CALFACT: CALFACT,
    _reserved28: [u8; 0x10],
    pub OR: OR,
    _reserved29: [u8; 0x023c],
    pub CCR: CCR,
    _reserved30: [u8; 0xe4],
    pub HWCFGR0: HWCFGR0,
    pub VERR: VERR,
    pub IPDR: IPDR,
    pub SIDR: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn ISR(&self) -> &ISR {
        &self.ISR
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn IER(&self) -> &IER {
        &self.IER
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x0c - ADC configuration register"]
    #[inline(always)]
    pub const fn CFGR(&self) -> &CFGR {
        &self.CFGR
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn CFGR2(&self) -> &CFGR2 {
        &self.CFGR2
    }
    #[doc = "0x14 - ADC sample time register 1"]
    #[inline(always)]
    pub const fn SMPR1(&self) -> &SMPR1 {
        &self.SMPR1
    }
    #[doc = "0x18 - ADC sample time register 2"]
    #[inline(always)]
    pub const fn SMPR2(&self) -> &SMPR2 {
        &self.SMPR2
    }
    #[doc = "0x20 - ADC watchdog threshold register 1"]
    #[inline(always)]
    pub const fn TR1(&self) -> &TR1 {
        &self.TR1
    }
    #[doc = "0x24 - ADC watchdog threshold register 2"]
    #[inline(always)]
    pub const fn TR2(&self) -> &TR2 {
        &self.TR2
    }
    #[doc = "0x28 - ADC watchdog threshold register 3"]
    #[inline(always)]
    pub const fn TR3(&self) -> &TR3 {
        &self.TR3
    }
    #[doc = "0x30 - ADC regular sequence register 1"]
    #[inline(always)]
    pub const fn SQR1(&self) -> &SQR1 {
        &self.SQR1
    }
    #[doc = "0x34 - ADC regular sequence register 2"]
    #[inline(always)]
    pub const fn SQR2(&self) -> &SQR2 {
        &self.SQR2
    }
    #[doc = "0x38 - ADC regular sequence register 3"]
    #[inline(always)]
    pub const fn SQR3(&self) -> &SQR3 {
        &self.SQR3
    }
    #[doc = "0x3c - ADC regular sequence register 4"]
    #[inline(always)]
    pub const fn SQR4(&self) -> &SQR4 {
        &self.SQR4
    }
    #[doc = "0x40 - ADC regular data register"]
    #[inline(always)]
    pub const fn DR(&self) -> &DR {
        &self.DR
    }
    #[doc = "0x4c - ADC injected sequence register"]
    #[inline(always)]
    pub const fn JSQR(&self) -> &JSQR {
        &self.JSQR
    }
    #[doc = "0x60 - ADC offset 1 register"]
    #[inline(always)]
    pub const fn OFR1(&self) -> &OFR1 {
        &self.OFR1
    }
    #[doc = "0x64 - ADC offset 2 register"]
    #[inline(always)]
    pub const fn OFR2(&self) -> &OFR2 {
        &self.OFR2
    }
    #[doc = "0x68 - ADC offset 3 register"]
    #[inline(always)]
    pub const fn OFR3(&self) -> &OFR3 {
        &self.OFR3
    }
    #[doc = "0x6c - ADC offset 4 register"]
    #[inline(always)]
    pub const fn OFR4(&self) -> &OFR4 {
        &self.OFR4
    }
    #[doc = "0x80 - ADC injected channel 1 data register"]
    #[inline(always)]
    pub const fn JDR1(&self) -> &JDR1 {
        &self.JDR1
    }
    #[doc = "0x84 - ADC injected channel 2 data register"]
    #[inline(always)]
    pub const fn JDR2(&self) -> &JDR2 {
        &self.JDR2
    }
    #[doc = "0x88 - ADC injected channel 3 data register"]
    #[inline(always)]
    pub const fn JDR3(&self) -> &JDR3 {
        &self.JDR3
    }
    #[doc = "0x8c - ADC injected channel 4 data register"]
    #[inline(always)]
    pub const fn JDR4(&self) -> &JDR4 {
        &self.JDR4
    }
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration Register"]
    #[inline(always)]
    pub const fn AWD2CR(&self) -> &AWD2CR {
        &self.AWD2CR
    }
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration Register"]
    #[inline(always)]
    pub const fn AWD3CR(&self) -> &AWD3CR {
        &self.AWD3CR
    }
    #[doc = "0xb0 - ADC Differential mode Selection Register"]
    #[inline(always)]
    pub const fn DIFSEL(&self) -> &DIFSEL {
        &self.DIFSEL
    }
    #[doc = "0xb4 - ADC Calibration Factors"]
    #[inline(always)]
    pub const fn CALFACT(&self) -> &CALFACT {
        &self.CALFACT
    }
    #[doc = "0xc8 - ADC option register"]
    #[inline(always)]
    pub const fn OR(&self) -> &OR {
        &self.OR
    }
    #[doc = "0x308 - ADC common control register"]
    #[inline(always)]
    pub const fn CCR(&self) -> &CCR {
        &self.CCR
    }
    #[doc = "0x3f0 - ADC hardware configuration register"]
    #[inline(always)]
    pub const fn HWCFGR0(&self) -> &HWCFGR0 {
        &self.HWCFGR0
    }
    #[doc = "0x3f4 - ADC version register"]
    #[inline(always)]
    pub const fn VERR(&self) -> &VERR {
        &self.VERR
    }
    #[doc = "0x3f8 - ADC identification register"]
    #[inline(always)]
    pub const fn IPDR(&self) -> &IPDR {
        &self.IPDR
    }
    #[doc = "0x3fc - ADC size identification register"]
    #[inline(always)]
    pub const fn SIDR(&self) -> &SIDR {
        &self.SIDR
    }
}
#[doc = "ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: ADC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "ADC configuration register"]
pub mod cfgr;
#[doc = "CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR1 (rw) register accessor: ADC sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr1`] module"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "ADC sample time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: ADC sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr2`] module"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "ADC sample time register 2"]
pub mod smpr2;
#[doc = "TR1 (rw) register accessor: ADC watchdog threshold register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr1`] module"]
pub type TR1 = crate::Reg<tr1::TR1_SPEC>;
#[doc = "ADC watchdog threshold register 1"]
pub mod tr1;
#[doc = "TR2 (rw) register accessor: ADC watchdog threshold register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr2`] module"]
pub type TR2 = crate::Reg<tr2::TR2_SPEC>;
#[doc = "ADC watchdog threshold register 2"]
pub mod tr2;
#[doc = "TR3 (rw) register accessor: ADC watchdog threshold register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr3`] module"]
pub type TR3 = crate::Reg<tr3::TR3_SPEC>;
#[doc = "ADC watchdog threshold register 3"]
pub mod tr3;
#[doc = "SQR1 (rw) register accessor: ADC regular sequence register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr1`] module"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "ADC regular sequence register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: ADC regular sequence register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr2`] module"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "ADC regular sequence register 2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: ADC regular sequence register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr3`] module"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "ADC regular sequence register 3"]
pub mod sqr3;
#[doc = "SQR4 (rw) register accessor: ADC regular sequence register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr4`] module"]
pub type SQR4 = crate::Reg<sqr4::SQR4_SPEC>;
#[doc = "ADC regular sequence register 4"]
pub mod sqr4;
#[doc = "DR (r) register accessor: ADC regular data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "ADC regular data register"]
pub mod dr;
#[doc = "JSQR (rw) register accessor: ADC injected sequence register\n\nYou can [`read`](crate::Reg::read) this register and get [`jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jsqr`] module"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "ADC injected sequence register"]
pub mod jsqr;
#[doc = "OFR1 (rw) register accessor: ADC offset 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr1`] module"]
pub type OFR1 = crate::Reg<ofr1::OFR1_SPEC>;
#[doc = "ADC offset 1 register"]
pub mod ofr1;
#[doc = "OFR2 (rw) register accessor: ADC offset 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr2`] module"]
pub type OFR2 = crate::Reg<ofr2::OFR2_SPEC>;
#[doc = "ADC offset 2 register"]
pub mod ofr2;
#[doc = "OFR3 (rw) register accessor: ADC offset 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr3`] module"]
pub type OFR3 = crate::Reg<ofr3::OFR3_SPEC>;
#[doc = "ADC offset 3 register"]
pub mod ofr3;
#[doc = "OFR4 (rw) register accessor: ADC offset 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr4`] module"]
pub type OFR4 = crate::Reg<ofr4::OFR4_SPEC>;
#[doc = "ADC offset 4 register"]
pub mod ofr4;
#[doc = "JDR1 (r) register accessor: ADC injected channel 1 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr1`] module"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "ADC injected channel 1 data register"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: ADC injected channel 2 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr2`] module"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "ADC injected channel 2 data register"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: ADC injected channel 3 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr3`] module"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "ADC injected channel 3 data register"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: ADC injected channel 4 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr4`] module"]
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
#[doc = "ADC injected channel 4 data register"]
pub mod jdr4;
#[doc = "AWD2CR (rw) register accessor: ADC Analog Watchdog 2 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2cr`] module"]
pub type AWD2CR = crate::Reg<awd2cr::AWD2CR_SPEC>;
#[doc = "ADC Analog Watchdog 2 Configuration Register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: ADC Analog Watchdog 3 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3cr`] module"]
pub type AWD3CR = crate::Reg<awd3cr::AWD3CR_SPEC>;
#[doc = "ADC Analog Watchdog 3 Configuration Register"]
pub mod awd3cr;
#[doc = "DIFSEL (rw) register accessor: ADC Differential mode Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`difsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@difsel`] module"]
pub type DIFSEL = crate::Reg<difsel::DIFSEL_SPEC>;
#[doc = "ADC Differential mode Selection Register"]
pub mod difsel;
#[doc = "CALFACT (rw) register accessor: ADC Calibration Factors\n\nYou can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfact`] module"]
pub type CALFACT = crate::Reg<calfact::CALFACT_SPEC>;
#[doc = "ADC Calibration Factors"]
pub mod calfact;
#[doc = "OR (rw) register accessor: ADC option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`] module"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "ADC option register"]
pub mod or;
#[doc = "CCR (rw) register accessor: ADC common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ADC common control register"]
pub mod ccr;
#[doc = "HWCFGR0 (r) register accessor: ADC hardware configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr0`] module"]
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0_SPEC>;
#[doc = "ADC hardware configuration register"]
pub mod hwcfgr0;
#[doc = "VERR (r) register accessor: ADC version register\n\nYou can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`] module"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "ADC version register"]
pub mod verr;
#[doc = "IPDR (r) register accessor: ADC identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipdr`] module"]
pub type IPDR = crate::Reg<ipdr::IPDR_SPEC>;
#[doc = "ADC identification register"]
pub mod ipdr;
#[doc = "SIDR (r) register accessor: ADC size identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`] module"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "ADC size identification register"]
pub mod sidr;
