#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR: CR,
    pub CFGR: CFGR,
    _reserved2: [u8; 0x08],
    pub RDR: RDR,
    pub RDWR: RDWR,
    pub TDR: TDR,
    pub TDWR: TDWR,
    pub IBIDR: IBIDR,
    pub TGTTDR: TGTTDR,
    _reserved8: [u8; 0x08],
    pub SR: SR,
    pub SER: SER,
    _reserved10: [u8; 0x08],
    pub RMR: RMR,
    _reserved11: [u8; 0x0c],
    pub EVR: EVR,
    pub IER: IER,
    pub CEVR: CEVR,
    _reserved14: [u8; 0x04],
    pub DEVR0: DEVR0,
    pub DEVR1: DEVR1,
    pub DEVR2: DEVR2,
    pub DEVR3: DEVR3,
    pub DEVR4: DEVR4,
    _reserved19: [u8; 0x1c],
    pub MAXRLR: MAXRLR,
    pub MAXWLR: MAXWLR,
    _reserved21: [u8; 0x08],
    pub TIMINGR0: TIMINGR0,
    pub TIMINGR1: TIMINGR1,
    pub TIMINGR2: TIMINGR2,
    _reserved24: [u8; 0x14],
    pub BCR: BCR,
    pub DCR: DCR,
    pub GETCAPR: GETCAPR,
    pub CRCAPR: CRCAPR,
    pub GETMXDSR: GETMXDSR,
    pub EPIDR: EPIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - I3C message control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x04 - I3C configuration register"]
    #[inline(always)]
    pub const fn CFGR(&self) -> &CFGR {
        &self.CFGR
    }
    #[doc = "0x10 - I3C receive data byte register"]
    #[inline(always)]
    pub const fn RDR(&self) -> &RDR {
        &self.RDR
    }
    #[doc = "0x14 - I3C receive data word register"]
    #[inline(always)]
    pub const fn RDWR(&self) -> &RDWR {
        &self.RDWR
    }
    #[doc = "0x18 - I3C transmit data byte register"]
    #[inline(always)]
    pub const fn TDR(&self) -> &TDR {
        &self.TDR
    }
    #[doc = "0x1c - I3C transmit data word register"]
    #[inline(always)]
    pub const fn TDWR(&self) -> &TDWR {
        &self.TDWR
    }
    #[doc = "0x20 - I3C IBI payload data register"]
    #[inline(always)]
    pub const fn IBIDR(&self) -> &IBIDR {
        &self.IBIDR
    }
    #[doc = "0x24 - I3C target transmit configuration register"]
    #[inline(always)]
    pub const fn TGTTDR(&self) -> &TGTTDR {
        &self.TGTTDR
    }
    #[doc = "0x30 - I3C status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x34 - I3C status error register"]
    #[inline(always)]
    pub const fn SER(&self) -> &SER {
        &self.SER
    }
    #[doc = "0x40 - I3C received message register"]
    #[inline(always)]
    pub const fn RMR(&self) -> &RMR {
        &self.RMR
    }
    #[doc = "0x50 - I3C event register"]
    #[inline(always)]
    pub const fn EVR(&self) -> &EVR {
        &self.EVR
    }
    #[doc = "0x54 - I3C interrupt enable register"]
    #[inline(always)]
    pub const fn IER(&self) -> &IER {
        &self.IER
    }
    #[doc = "0x58 - I3C clear event register"]
    #[inline(always)]
    pub const fn CEVR(&self) -> &CEVR {
        &self.CEVR
    }
    #[doc = "0x60 - I3C own device characteristics register"]
    #[inline(always)]
    pub const fn DEVR0(&self) -> &DEVR0 {
        &self.DEVR0
    }
    #[doc = "0x64 - I3C device 1 characteristics register"]
    #[inline(always)]
    pub const fn DEVR1(&self) -> &DEVR1 {
        &self.DEVR1
    }
    #[doc = "0x68 - I3C device 2 characteristics register"]
    #[inline(always)]
    pub const fn DEVR2(&self) -> &DEVR2 {
        &self.DEVR2
    }
    #[doc = "0x6c - I3C device 3 characteristics register"]
    #[inline(always)]
    pub const fn DEVR3(&self) -> &DEVR3 {
        &self.DEVR3
    }
    #[doc = "0x70 - I3C device 4 characteristics register"]
    #[inline(always)]
    pub const fn DEVR4(&self) -> &DEVR4 {
        &self.DEVR4
    }
    #[doc = "0x90 - I3C maximum read length register"]
    #[inline(always)]
    pub const fn MAXRLR(&self) -> &MAXRLR {
        &self.MAXRLR
    }
    #[doc = "0x94 - I3C maximum write length register"]
    #[inline(always)]
    pub const fn MAXWLR(&self) -> &MAXWLR {
        &self.MAXWLR
    }
    #[doc = "0xa0 - I3C timing register 0"]
    #[inline(always)]
    pub const fn TIMINGR0(&self) -> &TIMINGR0 {
        &self.TIMINGR0
    }
    #[doc = "0xa4 - I3C timing register 1"]
    #[inline(always)]
    pub const fn TIMINGR1(&self) -> &TIMINGR1 {
        &self.TIMINGR1
    }
    #[doc = "0xa8 - I3C timing register 2"]
    #[inline(always)]
    pub const fn TIMINGR2(&self) -> &TIMINGR2 {
        &self.TIMINGR2
    }
    #[doc = "0xc0 - I3C bus characteristics register"]
    #[inline(always)]
    pub const fn BCR(&self) -> &BCR {
        &self.BCR
    }
    #[doc = "0xc4 - I3C device characteristics register"]
    #[inline(always)]
    pub const fn DCR(&self) -> &DCR {
        &self.DCR
    }
    #[doc = "0xc8 - I3C get capability register"]
    #[inline(always)]
    pub const fn GETCAPR(&self) -> &GETCAPR {
        &self.GETCAPR
    }
    #[doc = "0xcc - I3C controller-role capability register"]
    #[inline(always)]
    pub const fn CRCAPR(&self) -> &CRCAPR {
        &self.CRCAPR
    }
    #[doc = "0xd0 - I3C get capability register"]
    #[inline(always)]
    pub const fn GETMXDSR(&self) -> &GETMXDSR {
        &self.GETMXDSR
    }
    #[doc = "0xd4 - I3C extended provisioned ID register"]
    #[inline(always)]
    pub const fn EPIDR(&self) -> &EPIDR {
        &self.EPIDR
    }
}
#[doc = "CR (w) register accessor: I3C message control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "I3C message control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: I3C configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "I3C configuration register"]
pub mod cfgr;
#[doc = "RDR (r) register accessor: I3C receive data byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`] module"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "I3C receive data byte register"]
pub mod rdr;
#[doc = "RDWR (r) register accessor: I3C receive data word register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdwr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdwr`] module"]
pub type RDWR = crate::Reg<rdwr::RDWR_SPEC>;
#[doc = "I3C receive data word register"]
pub mod rdwr;
#[doc = "TDR (w) register accessor: I3C transmit data byte register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`] module"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "I3C transmit data byte register"]
pub mod tdr;
#[doc = "TDWR (w) register accessor: I3C transmit data word register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdwr`] module"]
pub type TDWR = crate::Reg<tdwr::TDWR_SPEC>;
#[doc = "I3C transmit data word register"]
pub mod tdwr;
#[doc = "IBIDR (rw) register accessor: I3C IBI payload data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibidr`] module"]
pub type IBIDR = crate::Reg<ibidr::IBIDR_SPEC>;
#[doc = "I3C IBI payload data register"]
pub mod ibidr;
#[doc = "TGTTDR (rw) register accessor: I3C target transmit configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgttdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgttdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgttdr`] module"]
pub type TGTTDR = crate::Reg<tgttdr::TGTTDR_SPEC>;
#[doc = "I3C target transmit configuration register"]
pub mod tgttdr;
#[doc = "SR (r) register accessor: I3C status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "I3C status register"]
pub mod sr;
#[doc = "SER (r) register accessor: I3C status error register\n\nYou can [`read`](crate::Reg::read) this register and get [`ser::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser`] module"]
pub type SER = crate::Reg<ser::SER_SPEC>;
#[doc = "I3C status error register"]
pub mod ser;
#[doc = "RMR (r) register accessor: I3C received message register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmr`] module"]
pub type RMR = crate::Reg<rmr::RMR_SPEC>;
#[doc = "I3C received message register"]
pub mod rmr;
#[doc = "EVR (r) register accessor: I3C event register\n\nYou can [`read`](crate::Reg::read) this register and get [`evr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evr`] module"]
pub type EVR = crate::Reg<evr::EVR_SPEC>;
#[doc = "I3C event register"]
pub mod evr;
#[doc = "IER (r) register accessor: I3C interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "I3C interrupt enable register"]
pub mod ier;
#[doc = "CEVR (w) register accessor: I3C clear event register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cevr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cevr`] module"]
pub type CEVR = crate::Reg<cevr::CEVR_SPEC>;
#[doc = "I3C clear event register"]
pub mod cevr;
#[doc = "DEVR0 (rw) register accessor: I3C own device characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`devr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr0`] module"]
pub type DEVR0 = crate::Reg<devr0::DEVR0_SPEC>;
#[doc = "I3C own device characteristics register"]
pub mod devr0;
#[doc = "DEVR1 (rw) register accessor: I3C device 1 characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`devr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr1`] module"]
pub type DEVR1 = crate::Reg<devr1::DEVR1_SPEC>;
#[doc = "I3C device 1 characteristics register"]
pub mod devr1;
#[doc = "DEVR2 (rw) register accessor: I3C device 2 characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`devr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr2`] module"]
pub type DEVR2 = crate::Reg<devr2::DEVR2_SPEC>;
#[doc = "I3C device 2 characteristics register"]
pub mod devr2;
#[doc = "DEVR3 (rw) register accessor: I3C device 3 characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`devr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr3`] module"]
pub type DEVR3 = crate::Reg<devr3::DEVR3_SPEC>;
#[doc = "I3C device 3 characteristics register"]
pub mod devr3;
#[doc = "DEVR4 (rw) register accessor: I3C device 4 characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`devr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devr4`] module"]
pub type DEVR4 = crate::Reg<devr4::DEVR4_SPEC>;
#[doc = "I3C device 4 characteristics register"]
pub mod devr4;
#[doc = "MAXRLR (rw) register accessor: I3C maximum read length register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxrlr`] module"]
pub type MAXRLR = crate::Reg<maxrlr::MAXRLR_SPEC>;
#[doc = "I3C maximum read length register"]
pub mod maxrlr;
#[doc = "MAXWLR (rw) register accessor: I3C maximum write length register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxwlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxwlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxwlr`] module"]
pub type MAXWLR = crate::Reg<maxwlr::MAXWLR_SPEC>;
#[doc = "I3C maximum write length register"]
pub mod maxwlr;
#[doc = "TIMINGR0 (rw) register accessor: I3C timing register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr0`] module"]
pub type TIMINGR0 = crate::Reg<timingr0::TIMINGR0_SPEC>;
#[doc = "I3C timing register 0"]
pub mod timingr0;
#[doc = "TIMINGR1 (rw) register accessor: I3C timing register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr1`] module"]
pub type TIMINGR1 = crate::Reg<timingr1::TIMINGR1_SPEC>;
#[doc = "I3C timing register 1"]
pub mod timingr1;
#[doc = "TIMINGR2 (rw) register accessor: I3C timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr2`] module"]
pub type TIMINGR2 = crate::Reg<timingr2::TIMINGR2_SPEC>;
#[doc = "I3C timing register 2"]
pub mod timingr2;
#[doc = "BCR (rw) register accessor: I3C bus characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`] module"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "I3C bus characteristics register"]
pub mod bcr;
#[doc = "DCR (rw) register accessor: I3C device characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`] module"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "I3C device characteristics register"]
pub mod dcr;
#[doc = "GETCAPR (rw) register accessor: I3C get capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`getcapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getcapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@getcapr`] module"]
pub type GETCAPR = crate::Reg<getcapr::GETCAPR_SPEC>;
#[doc = "I3C get capability register"]
pub mod getcapr;
#[doc = "CRCAPR (rw) register accessor: I3C controller-role capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcapr`] module"]
pub type CRCAPR = crate::Reg<crcapr::CRCAPR_SPEC>;
#[doc = "I3C controller-role capability register"]
pub mod crcapr;
#[doc = "GETMXDSR (rw) register accessor: I3C get capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`getmxdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getmxdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@getmxdsr`] module"]
pub type GETMXDSR = crate::Reg<getmxdsr::GETMXDSR_SPEC>;
#[doc = "I3C get capability register"]
pub mod getmxdsr;
#[doc = "EPIDR (rw) register accessor: I3C extended provisioned ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`epidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epidr`] module"]
pub type EPIDR = crate::Reg<epidr::EPIDR_SPEC>;
#[doc = "I3C extended provisioned ID register"]
pub mod epidr;
