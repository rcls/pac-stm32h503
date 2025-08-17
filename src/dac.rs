#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR: CR,
    pub SWTRGR: SWTRGR,
    pub DHR12R1: DHR12R1,
    pub DHR12L1: DHR12L1,
    pub DHR8R1: DHR8R1,
    pub DHR12R2: DHR12R2,
    pub DHR12L2: DHR12L2,
    pub DHR8R2: DHR8R2,
    pub DHR12RD: DHR12RD,
    pub DHR12LD: DHR12LD,
    pub DHR8RD: DHR8RD,
    pub DOR1: DOR1,
    pub DOR2: DOR2,
    pub SR: SR,
    pub CCR: CCR,
    pub MCR: MCR,
    pub SHSR1: SHSR1,
    pub SHSR2: SHSR2,
    pub SHHR: SHHR,
    pub SHRR: SHRR,
}
impl RegisterBlock {
    #[doc = "0x00 - DAC control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x04 - DAC software trigger register"]
    #[inline(always)]
    pub const fn SWTRGR(&self) -> &SWTRGR {
        &self.SWTRGR
    }
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn DHR12R1(&self) -> &DHR12R1 {
        &self.DHR12R1
    }
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn DHR12L1(&self) -> &DHR12L1 {
        &self.DHR12L1
    }
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn DHR8R1(&self) -> &DHR8R1 {
        &self.DHR8R1
    }
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn DHR12R2(&self) -> &DHR12R2 {
        &self.DHR12R2
    }
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn DHR12L2(&self) -> &DHR12L2 {
        &self.DHR12L2
    }
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn DHR8R2(&self) -> &DHR8R2 {
        &self.DHR8R2
    }
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn DHR12RD(&self) -> &DHR12RD {
        &self.DHR12RD
    }
    #[doc = "0x24 - Dual DAC 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn DHR12LD(&self) -> &DHR12LD {
        &self.DHR12LD
    }
    #[doc = "0x28 - Dual DAC 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn DHR8RD(&self) -> &DHR8RD {
        &self.DHR8RD
    }
    #[doc = "0x2c - DAC channel1 data output register"]
    #[inline(always)]
    pub const fn DOR1(&self) -> &DOR1 {
        &self.DOR1
    }
    #[doc = "0x30 - DAC channel2 data output register"]
    #[inline(always)]
    pub const fn DOR2(&self) -> &DOR2 {
        &self.DOR2
    }
    #[doc = "0x34 - DAC status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x38 - DAC calibration control register"]
    #[inline(always)]
    pub const fn CCR(&self) -> &CCR {
        &self.CCR
    }
    #[doc = "0x3c - DAC mode control register"]
    #[inline(always)]
    pub const fn MCR(&self) -> &MCR {
        &self.MCR
    }
    #[doc = "0x40 - DAC channel1 sample and hold sample time register"]
    #[inline(always)]
    pub const fn SHSR1(&self) -> &SHSR1 {
        &self.SHSR1
    }
    #[doc = "0x44 - DAC channel2 sample and hold sample time register"]
    #[inline(always)]
    pub const fn SHSR2(&self) -> &SHSR2 {
        &self.SHSR2
    }
    #[doc = "0x48 - DAC sample and hold time register"]
    #[inline(always)]
    pub const fn SHHR(&self) -> &SHHR {
        &self.SHHR
    }
    #[doc = "0x4c - DAC sample and hold refresh time register"]
    #[inline(always)]
    pub const fn SHRR(&self) -> &SHRR {
        &self.SHRR
    }
}
#[doc = "CR (rw) register accessor: DAC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DAC control register"]
pub mod cr;
#[doc = "SWTRGR (w) register accessor: DAC software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrgr`] module"]
pub type SWTRGR = crate::Reg<swtrgr::SWTRGR_SPEC>;
#[doc = "DAC software trigger register"]
pub mod swtrgr;
#[doc = "DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r1`] module"]
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l1`] module"]
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r1`] module"]
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "DHR12R2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r2`] module"]
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2_SPEC>;
#[doc = "DAC channel2 12-bit right aligned data holding register"]
pub mod dhr12r2;
#[doc = "DHR12L2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l2`] module"]
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2_SPEC>;
#[doc = "DAC channel2 12-bit left aligned data holding register"]
pub mod dhr12l2;
#[doc = "DHR8R2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r2`] module"]
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2_SPEC>;
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
pub mod dhr8r2;
#[doc = "DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12rd`] module"]
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RD_SPEC>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "DHR12LD (rw) register accessor: Dual DAC 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12ld`] module"]
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LD_SPEC>;
#[doc = "Dual DAC 12-bit left aligned data holding register"]
pub mod dhr12ld;
#[doc = "DHR8RD (rw) register accessor: Dual DAC 8-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8rd`] module"]
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RD_SPEC>;
#[doc = "Dual DAC 8-bit right aligned data holding register"]
pub mod dhr8rd;
#[doc = "DOR1 (r) register accessor: DAC channel1 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor1`] module"]
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
#[doc = "DAC channel1 data output register"]
pub mod dor1;
#[doc = "DOR2 (r) register accessor: DAC channel2 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor2`] module"]
pub type DOR2 = crate::Reg<dor2::DOR2_SPEC>;
#[doc = "DAC channel2 data output register"]
pub mod dor2;
#[doc = "SR (rw) register accessor: DAC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "DAC status register"]
pub mod sr;
#[doc = "CCR (rw) register accessor: DAC calibration control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "DAC calibration control register"]
pub mod ccr;
#[doc = "MCR (rw) register accessor: DAC mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "DAC mode control register"]
pub mod mcr;
#[doc = "SHSR1 (rw) register accessor: DAC channel1 sample and hold sample time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shsr1`] module"]
pub type SHSR1 = crate::Reg<shsr1::SHSR1_SPEC>;
#[doc = "DAC channel1 sample and hold sample time register"]
pub mod shsr1;
#[doc = "SHSR2 (rw) register accessor: DAC channel2 sample and hold sample time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shsr2`] module"]
pub type SHSR2 = crate::Reg<shsr2::SHSR2_SPEC>;
#[doc = "DAC channel2 sample and hold sample time register"]
pub mod shsr2;
#[doc = "SHHR (rw) register accessor: DAC sample and hold time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shhr`] module"]
pub type SHHR = crate::Reg<shhr::SHHR_SPEC>;
#[doc = "DAC sample and hold time register"]
pub mod shhr;
#[doc = "SHRR (rw) register accessor: DAC sample and hold refresh time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrr`] module"]
pub type SHRR = crate::Reg<shrr::SHRR_SPEC>;
#[doc = "DAC sample and hold refresh time register"]
pub mod shrr;
