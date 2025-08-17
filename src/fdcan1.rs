#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CREL: CREL,
    pub ENDN: ENDN,
    _reserved2: [u8; 0x04],
    pub DBTP: DBTP,
    pub TEST: TEST,
    pub RWD: RWD,
    pub CCCR: CCCR,
    pub NBTP: NBTP,
    pub TSCC: TSCC,
    pub TSCV: TSCV,
    pub TOCC: TOCC,
    pub TOCV: TOCV,
    _reserved11: [u8; 0x10],
    pub ECR: ECR,
    pub PSR: PSR,
    pub TDCR: TDCR,
    _reserved14: [u8; 0x04],
    pub IR: IR,
    pub IE: IE,
    pub ILS: ILS,
    pub ILE: ILE,
    _reserved18: [u8; 0x20],
    pub RXGFC: RXGFC,
    pub XIDAM: XIDAM,
    pub HPMS: HPMS,
    _reserved21: [u8; 0x04],
    pub RXF0S: RXF0S,
    pub RXF0A: RXF0A,
    pub RXF1S: RXF1S,
    pub RXF1A: RXF1A,
    _reserved25: [u8; 0x20],
    pub TXBC: TXBC,
    pub TXFQS: TXFQS,
    pub TXBRP: TXBRP,
    pub TXBAR: TXBAR,
    pub TXBCR: TXBCR,
    pub TXBTO: TXBTO,
    pub TXBCF: TXBCF,
    pub TXBTIE: TXBTIE,
    pub TXBCIE: TXBCIE,
    pub TXEFS: TXEFS,
    pub TXEFA: TXEFA,
    _reserved36: [u8; 0x14],
    pub CKDIV: CKDIV,
}
impl RegisterBlock {
    #[doc = "0x00 - FDCAN core release register"]
    #[inline(always)]
    pub const fn CREL(&self) -> &CREL {
        &self.CREL
    }
    #[doc = "0x04 - FDCAN endian register"]
    #[inline(always)]
    pub const fn ENDN(&self) -> &ENDN {
        &self.ENDN
    }
    #[doc = "0x0c - FDCAN data bit timing and prescaler register"]
    #[inline(always)]
    pub const fn DBTP(&self) -> &DBTP {
        &self.DBTP
    }
    #[doc = "0x10 - FDCAN test register"]
    #[inline(always)]
    pub const fn TEST(&self) -> &TEST {
        &self.TEST
    }
    #[doc = "0x14 - FDCAN RAM watchdog register"]
    #[inline(always)]
    pub const fn RWD(&self) -> &RWD {
        &self.RWD
    }
    #[doc = "0x18 - FDCAN CC control register"]
    #[inline(always)]
    pub const fn CCCR(&self) -> &CCCR {
        &self.CCCR
    }
    #[doc = "0x1c - FDCAN nominal bit timing and prescaler register"]
    #[inline(always)]
    pub const fn NBTP(&self) -> &NBTP {
        &self.NBTP
    }
    #[doc = "0x20 - FDCAN timestamp counter configuration register"]
    #[inline(always)]
    pub const fn TSCC(&self) -> &TSCC {
        &self.TSCC
    }
    #[doc = "0x24 - FDCAN timestamp counter value register"]
    #[inline(always)]
    pub const fn TSCV(&self) -> &TSCV {
        &self.TSCV
    }
    #[doc = "0x28 - FDCAN timeout counter configuration register"]
    #[inline(always)]
    pub const fn TOCC(&self) -> &TOCC {
        &self.TOCC
    }
    #[doc = "0x2c - FDCAN timeout counter value register"]
    #[inline(always)]
    pub const fn TOCV(&self) -> &TOCV {
        &self.TOCV
    }
    #[doc = "0x40 - FDCAN error counter register"]
    #[inline(always)]
    pub const fn ECR(&self) -> &ECR {
        &self.ECR
    }
    #[doc = "0x44 - FDCAN protocol status register"]
    #[inline(always)]
    pub const fn PSR(&self) -> &PSR {
        &self.PSR
    }
    #[doc = "0x48 - FDCAN transmitter delay compensation register"]
    #[inline(always)]
    pub const fn TDCR(&self) -> &TDCR {
        &self.TDCR
    }
    #[doc = "0x50 - FDCAN interrupt register"]
    #[inline(always)]
    pub const fn IR(&self) -> &IR {
        &self.IR
    }
    #[doc = "0x54 - FDCAN interrupt enable register"]
    #[inline(always)]
    pub const fn IE(&self) -> &IE {
        &self.IE
    }
    #[doc = "0x58 - FDCAN interrupt line select register"]
    #[inline(always)]
    pub const fn ILS(&self) -> &ILS {
        &self.ILS
    }
    #[doc = "0x5c - FDCAN interrupt line enable register"]
    #[inline(always)]
    pub const fn ILE(&self) -> &ILE {
        &self.ILE
    }
    #[doc = "0x80 - FDCAN global filter configuration register"]
    #[inline(always)]
    pub const fn RXGFC(&self) -> &RXGFC {
        &self.RXGFC
    }
    #[doc = "0x84 - FDCAN extended ID and mask register"]
    #[inline(always)]
    pub const fn XIDAM(&self) -> &XIDAM {
        &self.XIDAM
    }
    #[doc = "0x88 - FDCAN high-priority message status register"]
    #[inline(always)]
    pub const fn HPMS(&self) -> &HPMS {
        &self.HPMS
    }
    #[doc = "0x90 - FDCAN Rx FIFO 0 status register"]
    #[inline(always)]
    pub const fn RXF0S(&self) -> &RXF0S {
        &self.RXF0S
    }
    #[doc = "0x94 - CAN Rx FIFO 0 acknowledge register"]
    #[inline(always)]
    pub const fn RXF0A(&self) -> &RXF0A {
        &self.RXF0A
    }
    #[doc = "0x98 - FDCAN Rx FIFO 1 status register"]
    #[inline(always)]
    pub const fn RXF1S(&self) -> &RXF1S {
        &self.RXF1S
    }
    #[doc = "0x9c - FDCAN Rx FIFO 1 acknowledge register"]
    #[inline(always)]
    pub const fn RXF1A(&self) -> &RXF1A {
        &self.RXF1A
    }
    #[doc = "0xc0 - FDCAN Tx buffer configuration register"]
    #[inline(always)]
    pub const fn TXBC(&self) -> &TXBC {
        &self.TXBC
    }
    #[doc = "0xc4 - FDCAN Tx FIFO/queue status register"]
    #[inline(always)]
    pub const fn TXFQS(&self) -> &TXFQS {
        &self.TXFQS
    }
    #[doc = "0xc8 - FDCAN Tx buffer request pending register"]
    #[inline(always)]
    pub const fn TXBRP(&self) -> &TXBRP {
        &self.TXBRP
    }
    #[doc = "0xcc - FDCAN Tx buffer add request register"]
    #[inline(always)]
    pub const fn TXBAR(&self) -> &TXBAR {
        &self.TXBAR
    }
    #[doc = "0xd0 - FDCAN Tx buffer cancellation request register"]
    #[inline(always)]
    pub const fn TXBCR(&self) -> &TXBCR {
        &self.TXBCR
    }
    #[doc = "0xd4 - FDCAN Tx buffer transmission occurred register"]
    #[inline(always)]
    pub const fn TXBTO(&self) -> &TXBTO {
        &self.TXBTO
    }
    #[doc = "0xd8 - FDCAN Tx buffer cancellation finished register"]
    #[inline(always)]
    pub const fn TXBCF(&self) -> &TXBCF {
        &self.TXBCF
    }
    #[doc = "0xdc - FDCAN Tx buffer transmission interrupt enable register"]
    #[inline(always)]
    pub const fn TXBTIE(&self) -> &TXBTIE {
        &self.TXBTIE
    }
    #[doc = "0xe0 - FDCAN Tx buffer cancellation finished interrupt enable register"]
    #[inline(always)]
    pub const fn TXBCIE(&self) -> &TXBCIE {
        &self.TXBCIE
    }
    #[doc = "0xe4 - FDCAN Tx event FIFO status register"]
    #[inline(always)]
    pub const fn TXEFS(&self) -> &TXEFS {
        &self.TXEFS
    }
    #[doc = "0xe8 - FDCAN Tx event FIFO acknowledge register"]
    #[inline(always)]
    pub const fn TXEFA(&self) -> &TXEFA {
        &self.TXEFA
    }
    #[doc = "0x100 - FDCAN CFG clock divider register"]
    #[inline(always)]
    pub const fn CKDIV(&self) -> &CKDIV {
        &self.CKDIV
    }
}
#[doc = "CREL (r) register accessor: FDCAN core release register\n\nYou can [`read`](crate::Reg::read) this register and get [`crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`] module"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "FDCAN core release register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: FDCAN endian register\n\nYou can [`read`](crate::Reg::read) this register and get [`endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`] module"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "FDCAN endian register"]
pub mod endn;
#[doc = "DBTP (rw) register accessor: FDCAN data bit timing and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`] module"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "FDCAN data bit timing and prescaler register"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: FDCAN test register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`] module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "FDCAN test register"]
pub mod test;
#[doc = "RWD (rw) register accessor: FDCAN RAM watchdog register\n\nYou can [`read`](crate::Reg::read) this register and get [`rwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`] module"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "FDCAN RAM watchdog register"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: FDCAN CC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`] module"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "FDCAN CC control register"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: FDCAN nominal bit timing and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`] module"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "FDCAN nominal bit timing and prescaler register"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: FDCAN timestamp counter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`] module"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "FDCAN timestamp counter configuration register"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: FDCAN timestamp counter value register\n\nYou can [`read`](crate::Reg::read) this register and get [`tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`] module"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "FDCAN timestamp counter value register"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: FDCAN timeout counter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`] module"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "FDCAN timeout counter configuration register"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: FDCAN timeout counter value register\n\nYou can [`read`](crate::Reg::read) this register and get [`tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`] module"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "FDCAN timeout counter value register"]
pub mod tocv;
#[doc = "ECR (rw) register accessor: FDCAN error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "FDCAN error counter register"]
pub mod ecr;
#[doc = "PSR (rw) register accessor: FDCAN protocol status register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`] module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "FDCAN protocol status register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: FDCAN transmitter delay compensation register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`] module"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "FDCAN transmitter delay compensation register"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: FDCAN interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`] module"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "FDCAN interrupt register"]
pub mod ir;
#[doc = "IE (rw) register accessor: FDCAN interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`] module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "FDCAN interrupt enable register"]
pub mod ie;
#[doc = "ILS (rw) register accessor: FDCAN interrupt line select register\n\nYou can [`read`](crate::Reg::read) this register and get [`ils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`] module"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "FDCAN interrupt line select register"]
pub mod ils;
#[doc = "ILE (rw) register accessor: FDCAN interrupt line enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`] module"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "FDCAN interrupt line enable register"]
pub mod ile;
#[doc = "RXGFC (rw) register accessor: FDCAN global filter configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxgfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxgfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxgfc`] module"]
pub type RXGFC = crate::Reg<rxgfc::RXGFC_SPEC>;
#[doc = "FDCAN global filter configuration register"]
pub mod rxgfc;
#[doc = "XIDAM (rw) register accessor: FDCAN extended ID and mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`] module"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "FDCAN extended ID and mask register"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: FDCAN high-priority message status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`] module"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "FDCAN high-priority message status register"]
pub mod hpms;
#[doc = "RXF0S (r) register accessor: FDCAN Rx FIFO 0 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`] module"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "FDCAN Rx FIFO 0 status register"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: CAN Rx FIFO 0 acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`] module"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "CAN Rx FIFO 0 acknowledge register"]
pub mod rxf0a;
#[doc = "RXF1S (r) register accessor: FDCAN Rx FIFO 1 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`] module"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "FDCAN Rx FIFO 1 status register"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: FDCAN Rx FIFO 1 acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`] module"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "FDCAN Rx FIFO 1 acknowledge register"]
pub mod rxf1a;
#[doc = "TXBC (rw) register accessor: FDCAN Tx buffer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`] module"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "FDCAN Tx buffer configuration register"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: FDCAN Tx FIFO/queue status register\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`] module"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "FDCAN Tx FIFO/queue status register"]
pub mod txfqs;
#[doc = "TXBRP (r) register accessor: FDCAN Tx buffer request pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`] module"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "FDCAN Tx buffer request pending register"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: FDCAN Tx buffer add request register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`] module"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "FDCAN Tx buffer add request register"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: FDCAN Tx buffer cancellation request register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`] module"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "FDCAN Tx buffer cancellation request register"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: FDCAN Tx buffer transmission occurred register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`] module"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "FDCAN Tx buffer transmission occurred register"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: FDCAN Tx buffer cancellation finished register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`] module"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "FDCAN Tx buffer cancellation finished register"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: FDCAN Tx buffer transmission interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`] module"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "FDCAN Tx buffer transmission interrupt enable register"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: FDCAN Tx buffer cancellation finished interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`] module"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register"]
pub mod txbcie;
#[doc = "TXEFS (r) register accessor: FDCAN Tx event FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`] module"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "FDCAN Tx event FIFO status register"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: FDCAN Tx event FIFO acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`] module"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "FDCAN Tx event FIFO acknowledge register"]
pub mod txefa;
#[doc = "CKDIV (rw) register accessor: FDCAN CFG clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckdiv`] module"]
pub type CKDIV = crate::Reg<ckdiv::CKDIV_SPEC>;
#[doc = "FDCAN CFG clock divider register"]
pub mod ckdiv;
