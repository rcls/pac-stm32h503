#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub TR: TR,
    pub DR: DR,
    pub SSR: SSR,
    pub ICSR: ICSR,
    pub PRER: PRER,
    pub WUTR: WUTR,
    pub CR: CR,
    pub PRIVCFGR: PRIVCFGR,
    _reserved8: [u8; 0x04],
    pub WPR: WPR,
    pub CALR: CALR,
    pub SHIFTR: SHIFTR,
    pub TSTR: TSTR,
    pub TSDR: TSDR,
    pub TSSSR: TSSSR,
    _reserved14: [u8; 0x04],
    pub ALRMAR: ALRMAR,
    pub ALRMASSR: ALRMASSR,
    pub ALRMBR: ALRMBR,
    pub ALRMBSSR: ALRMBSSR,
    pub SR: SR,
    pub MISR: MISR,
    _reserved20: [u8; 0x04],
    pub SCR: SCR,
    _reserved21: [u8; 0x10],
    pub ALRABINR: ALRABINR,
    pub ALRBBINR: ALRBBINR,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC time register"]
    #[inline(always)]
    pub const fn TR(&self) -> &TR {
        &self.TR
    }
    #[doc = "0x04 - RTC date register"]
    #[inline(always)]
    pub const fn DR(&self) -> &DR {
        &self.DR
    }
    #[doc = "0x08 - RTC subsecond register"]
    #[inline(always)]
    pub const fn SSR(&self) -> &SSR {
        &self.SSR
    }
    #[doc = "0x0c - RTC initialization control and status register"]
    #[inline(always)]
    pub const fn ICSR(&self) -> &ICSR {
        &self.ICSR
    }
    #[doc = "0x10 - RTC prescaler register"]
    #[inline(always)]
    pub const fn PRER(&self) -> &PRER {
        &self.PRER
    }
    #[doc = "0x14 - RTC wakeup timer register"]
    #[inline(always)]
    pub const fn WUTR(&self) -> &WUTR {
        &self.WUTR
    }
    #[doc = "0x18 - RTC control register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x1c - RTC privilege mode control register"]
    #[inline(always)]
    pub const fn PRIVCFGR(&self) -> &PRIVCFGR {
        &self.PRIVCFGR
    }
    #[doc = "0x24 - RTC write protection register"]
    #[inline(always)]
    pub const fn WPR(&self) -> &WPR {
        &self.WPR
    }
    #[doc = "0x28 - RTC calibration register"]
    #[inline(always)]
    pub const fn CALR(&self) -> &CALR {
        &self.CALR
    }
    #[doc = "0x2c - RTC shift control register"]
    #[inline(always)]
    pub const fn SHIFTR(&self) -> &SHIFTR {
        &self.SHIFTR
    }
    #[doc = "0x30 - RTC timestamp time register"]
    #[inline(always)]
    pub const fn TSTR(&self) -> &TSTR {
        &self.TSTR
    }
    #[doc = "0x34 - RTC timestamp date register"]
    #[inline(always)]
    pub const fn TSDR(&self) -> &TSDR {
        &self.TSDR
    }
    #[doc = "0x38 - RTC timestamp subsecond register"]
    #[inline(always)]
    pub const fn TSSSR(&self) -> &TSSSR {
        &self.TSSSR
    }
    #[doc = "0x40 - RTC alarm A register"]
    #[inline(always)]
    pub const fn ALRMAR(&self) -> &ALRMAR {
        &self.ALRMAR
    }
    #[doc = "0x44 - RTC alarm A subsecond register"]
    #[inline(always)]
    pub const fn ALRMASSR(&self) -> &ALRMASSR {
        &self.ALRMASSR
    }
    #[doc = "0x48 - RTC alarm B register"]
    #[inline(always)]
    pub const fn ALRMBR(&self) -> &ALRMBR {
        &self.ALRMBR
    }
    #[doc = "0x4c - RTC alarm B subsecond register"]
    #[inline(always)]
    pub const fn ALRMBSSR(&self) -> &ALRMBSSR {
        &self.ALRMBSSR
    }
    #[doc = "0x50 - RTC status register"]
    #[inline(always)]
    pub const fn SR(&self) -> &SR {
        &self.SR
    }
    #[doc = "0x54 - RTC masked interrupt status register"]
    #[inline(always)]
    pub const fn MISR(&self) -> &MISR {
        &self.MISR
    }
    #[doc = "0x5c - RTC status clear register"]
    #[inline(always)]
    pub const fn SCR(&self) -> &SCR {
        &self.SCR
    }
    #[doc = "0x70 - RTC alarm A binary mode register"]
    #[inline(always)]
    pub const fn ALRABINR(&self) -> &ALRABINR {
        &self.ALRABINR
    }
    #[doc = "0x74 - RTC alarm B binary mode register"]
    #[inline(always)]
    pub const fn ALRBBINR(&self) -> &ALRBBINR {
        &self.ALRBBINR
    }
}
#[doc = "TR (rw) register accessor: RTC time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`] module"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "RTC time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: RTC date register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "RTC date register"]
pub mod dr;
#[doc = "SSR (r) register accessor: RTC subsecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`] module"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "RTC subsecond register"]
pub mod ssr;
#[doc = "ICSR (rw) register accessor: RTC initialization control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr`] module"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "RTC initialization control and status register"]
pub mod icsr;
#[doc = "PRER (rw) register accessor: RTC prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer`] module"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "RTC prescaler register"]
pub mod prer;
#[doc = "WUTR (rw) register accessor: RTC wakeup timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wutr`] module"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "RTC wakeup timer register"]
pub mod wutr;
#[doc = "CR (rw) register accessor: RTC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RTC control register"]
pub mod cr;
#[doc = "PRIVCFGR (rw) register accessor: RTC privilege mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`] module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "RTC privilege mode control register"]
pub mod privcfgr;
#[doc = "WPR (w) register accessor: RTC write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`] module"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "CALR (rw) register accessor: RTC calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`] module"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "RTC calibration register"]
pub mod calr;
#[doc = "SHIFTR (w) register accessor: RTC shift control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftr`] module"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "RTC shift control register"]
pub mod shiftr;
#[doc = "TSTR (r) register accessor: RTC timestamp time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstr`] module"]
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
#[doc = "RTC timestamp time register"]
pub mod tstr;
#[doc = "TSDR (r) register accessor: RTC timestamp date register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsdr`] module"]
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
#[doc = "RTC timestamp date register"]
pub mod tsdr;
#[doc = "TSSSR (r) register accessor: RTC timestamp subsecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsssr`] module"]
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
#[doc = "RTC timestamp subsecond register"]
pub mod tsssr;
#[doc = "ALRMAR (rw) register accessor: RTC alarm A register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmar`] module"]
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
#[doc = "RTC alarm A register"]
pub mod alrmar;
#[doc = "ALRMASSR (rw) register accessor: RTC alarm A subsecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmassr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmassr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmassr`] module"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
#[doc = "RTC alarm A subsecond register"]
pub mod alrmassr;
#[doc = "ALRMBR (rw) register accessor: RTC alarm B register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmbr`] module"]
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
#[doc = "RTC alarm B register"]
pub mod alrmbr;
#[doc = "ALRMBSSR (rw) register accessor: RTC alarm B subsecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmbssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmbssr`] module"]
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
#[doc = "RTC alarm B subsecond register"]
pub mod alrmbssr;
#[doc = "SR (r) register accessor: RTC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "RTC status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: RTC masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`] module"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "RTC masked interrupt status register"]
pub mod misr;
#[doc = "SCR (w) register accessor: RTC status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "RTC status clear register"]
pub mod scr;
#[doc = "ALRABINR (rw) register accessor: RTC alarm A binary mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrabinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrabinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrabinr`] module"]
pub type ALRABINR = crate::Reg<alrabinr::ALRABINR_SPEC>;
#[doc = "RTC alarm A binary mode register"]
pub mod alrabinr;
#[doc = "ALRBBINR (rw) register accessor: RTC alarm B binary mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrbbinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrbbinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrbbinr`] module"]
pub type ALRBBINR = crate::Reg<alrbbinr::ALRBBINR_SPEC>;
#[doc = "RTC alarm B binary mode register"]
pub mod alrbbinr;
