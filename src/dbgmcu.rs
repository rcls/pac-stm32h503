#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub IDCODE: IDCODE,
    pub CR: CR,
    pub APB1LFZR: APB1LFZR,
    pub APB1HFZR: APB1HFZR,
    pub APB2FZR: APB2FZR,
    pub APB3FZR: APB3FZR,
    _reserved6: [u8; 0x08],
    pub AHB1FZR: AHB1FZR,
    _reserved7: [u8; 0xd8],
    pub SR: SR,
    pub DBG_AUTH_HOST: DBG_AUTH_HOST,
    pub DBG_AUTH_DEVICE: DBG_AUTH_DEVICE,
    pub DBG_AUTH_ACK: DBG_AUTH_ACK,
    _reserved11: [u8; 0x0ec4],
    pub PIDR4: PIDR4,
    _reserved12: [u8; 0x0c],
    pub PIDR0: PIDR0,
    pub PIDR1: PIDR1,
    pub PIDR2: PIDR2,
    pub PIDR3: PIDR3,
    pub CIDR0: CIDR0,
    pub CIDR1: CIDR1,
    pub CIDR2: CIDR2,
    pub CIDR3: CIDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - DBGMCU identity code register"]
    #[inline(always)]
    pub const fn IDCODE(&self) -> &IDCODE {
        &self.IDCODE
    }
    #[doc = "0x04 - DBGMCU configuration register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x08 - DBGMCU APB1L peripheral freeze register"]
    #[inline(always)]
    pub const fn APB1LFZR(&self) -> &APB1LFZR {
        &self.APB1LFZR
    }
    #[doc = "0x0c - DBGMCU APB1H peripheral freeze register"]
    #[inline(always)]
    pub const fn APB1HFZR(&self) -> &APB1HFZR {
        &self.APB1HFZR
    }
    #[doc = "0x10 - DBGMCU APB2 peripheral freeze register"]
    #[inline(always)]
    pub const fn APB2FZR(&self) -> &APB2FZR {
        &self.APB2FZR
    }
    #[doc = "0x14 - DBGMCU APB3 peripheral freeze register"]
    #[inline(always)]
    pub const fn APB3FZR(&self) -> &APB3FZR {
        &self.APB3FZR
    }
    #[doc = "0x20 - DBGMCU AHB1 peripheral freeze register"]
    #[inline(always)]
    pub const fn AHB1FZR(&self) -> &AHB1FZR {
        &self.AHB1FZR
    }
    #[doc = "0xfc - DBGMCU status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x100 - DBGMCU debug authentication mailbox host register"]
    #[inline(always)]
    pub const fn DBG_AUTH_HOST(&self) -> &DBG_AUTH_HOST {
        &self.DBG_AUTH_HOST
    }
    #[doc = "0x104 - DBGMCU debug authentication mailbox device register"]
    #[inline(always)]
    pub const fn DBG_AUTH_DEVICE(&self) -> &DBG_AUTH_DEVICE {
        &self.DBG_AUTH_DEVICE
    }
    #[doc = "0x108 - DBGMCU debug authentication mailbox acknowledge register"]
    #[inline(always)]
    pub const fn DBG_AUTH_ACK(&self) -> &DBG_AUTH_ACK {
        &self.DBG_AUTH_ACK
    }
    #[doc = "0xfd0 - DBGMCU CoreSight peripheral identity register 4"]
    #[inline(always)]
    pub const fn PIDR4(&self) -> &PIDR4 {
        &self.PIDR4
    }
    #[doc = "0xfe0 - DBGMCU CoreSight peripheral identity register 0"]
    #[inline(always)]
    pub const fn PIDR0(&self) -> &PIDR0 {
        &self.PIDR0
    }
    #[doc = "0xfe4 - DBGMCU CoreSight peripheral identity register 1"]
    #[inline(always)]
    pub const fn PIDR1(&self) -> &PIDR1 {
        &self.PIDR1
    }
    #[doc = "0xfe8 - DBGMCU CoreSight peripheral identity register 2"]
    #[inline(always)]
    pub const fn PIDR2(&self) -> &PIDR2 {
        &self.PIDR2
    }
    #[doc = "0xfec - DBGMCU CoreSight peripheral identity register 3"]
    #[inline(always)]
    pub const fn PIDR3(&self) -> &PIDR3 {
        &self.PIDR3
    }
    #[doc = "0xff0 - DBGMCU CoreSight component identity register 0"]
    #[inline(always)]
    pub const fn CIDR0(&self) -> &CIDR0 {
        &self.CIDR0
    }
    #[doc = "0xff4 - DBGMCU CoreSight component identity register 1"]
    #[inline(always)]
    pub const fn CIDR1(&self) -> &CIDR1 {
        &self.CIDR1
    }
    #[doc = "0xff8 - DBGMCU CoreSight component identity register 2"]
    #[inline(always)]
    pub const fn CIDR2(&self) -> &CIDR2 {
        &self.CIDR2
    }
    #[doc = "0xffc - DBGMCU CoreSight component identity register 3"]
    #[inline(always)]
    pub const fn CIDR3(&self) -> &CIDR3 {
        &self.CIDR3
    }
}
#[doc = "IDCODE (r) register accessor: DBGMCU identity code register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`] module"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "DBGMCU identity code register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: DBGMCU configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DBGMCU configuration register"]
pub mod cr;
#[doc = "APB1LFZR (rw) register accessor: DBGMCU APB1L peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lfzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lfzr`] module"]
pub type APB1LFZR = crate::Reg<apb1lfzr::APB1LFZR_SPEC>;
#[doc = "DBGMCU APB1L peripheral freeze register"]
pub mod apb1lfzr;
#[doc = "APB1HFZR (rw) register accessor: DBGMCU APB1H peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hfzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hfzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hfzr`] module"]
pub type APB1HFZR = crate::Reg<apb1hfzr::APB1HFZR_SPEC>;
#[doc = "DBGMCU APB1H peripheral freeze register"]
pub mod apb1hfzr;
#[doc = "APB2FZR (rw) register accessor: DBGMCU APB2 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2fzr`] module"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
#[doc = "DBGMCU APB2 peripheral freeze register"]
pub mod apb2fzr;
#[doc = "APB3FZR (rw) register accessor: DBGMCU APB3 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3fzr`] module"]
pub type APB3FZR = crate::Reg<apb3fzr::APB3FZR_SPEC>;
#[doc = "DBGMCU APB3 peripheral freeze register"]
pub mod apb3fzr;
#[doc = "AHB1FZR (rw) register accessor: DBGMCU AHB1 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1fzr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1fzr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1fzr`] module"]
pub type AHB1FZR = crate::Reg<ahb1fzr::AHB1FZR_SPEC>;
#[doc = "DBGMCU AHB1 peripheral freeze register"]
pub mod ahb1fzr;
#[doc = "SR (w) register accessor: DBGMCU status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "DBGMCU status register"]
pub mod sr;
#[doc = "DBG_AUTH_HOST (rw) register accessor: DBGMCU debug authentication mailbox host register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_auth_host::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_host::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_auth_host`] module"]
pub type DBG_AUTH_HOST = crate::Reg<dbg_auth_host::DBG_AUTH_HOST_SPEC>;
#[doc = "DBGMCU debug authentication mailbox host register"]
pub mod dbg_auth_host;
#[doc = "DBG_AUTH_DEVICE (r) register accessor: DBGMCU debug authentication mailbox device register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_auth_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_auth_device`] module"]
pub type DBG_AUTH_DEVICE = crate::Reg<dbg_auth_device::DBG_AUTH_DEVICE_SPEC>;
#[doc = "DBGMCU debug authentication mailbox device register"]
pub mod dbg_auth_device;
#[doc = "DBG_AUTH_ACK (rw) register accessor: DBGMCU debug authentication mailbox acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_auth_ack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_ack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_auth_ack`] module"]
pub type DBG_AUTH_ACK = crate::Reg<dbg_auth_ack::DBG_AUTH_ACK_SPEC>;
#[doc = "DBGMCU debug authentication mailbox acknowledge register"]
pub mod dbg_auth_ack;
#[doc = "PIDR4 (r) register accessor: DBGMCU CoreSight peripheral identity register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 4"]
pub mod pidr4;
#[doc = "PIDR0 (r) register accessor: DBGMCU CoreSight peripheral identity register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`] module"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: DBGMCU CoreSight peripheral identity register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`] module"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: DBGMCU CoreSight peripheral identity register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`] module"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: DBGMCU CoreSight peripheral identity register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`] module"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "DBGMCU CoreSight peripheral identity register 3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: DBGMCU CoreSight component identity register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`] module"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "DBGMCU CoreSight component identity register 0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: DBGMCU CoreSight component identity register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`] module"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "DBGMCU CoreSight component identity register 1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: DBGMCU CoreSight component identity register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`] module"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "DBGMCU CoreSight component identity register 2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: DBGMCU CoreSight component identity register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`] module"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "DBGMCU CoreSight component identity register 3"]
pub mod cidr3;
