#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub ACR: ACR,
    pub NSKEYR: NSKEYR,
    _reserved2: [u8; 0x04],
    pub OPTKEYR: OPTKEYR,
    _reserved3: [u8; 0x08],
    pub OPSR: OPSR,
    pub OPTCR: OPTCR,
    pub NSSR: NSSR,
    pub SECSR: SECSR,
    pub NSCR: NSCR,
    _reserved8: [u8; 0x04],
    pub NSCCR: NSCCR,
    _reserved9: [u8; 0x08],
    pub PRIVCFGR: PRIVCFGR,
    _reserved10: [u8; 0x08],
    pub HDPEXTR: HDPEXTR,
    _reserved11: [u8; 0x04],
    pub OPTSR_CUR: OPTSR_CUR,
    pub OPTSR_PRG: OPTSR_PRG,
    _reserved13: [u8; 0x18],
    pub OPTSR2_CUR: OPTSR2_CUR,
    pub OPTSR2_PRG: OPTSR2_PRG,
    _reserved15: [u8; 0x08],
    pub NSBOOTR_CUR: NSBOOTR_CUR,
    pub NSBOOTR_PRG: NSBOOTR_PRG,
    _reserved17: [u8; 0x08],
    pub OTPBLR_CUR: OTPBLR_CUR,
    pub OTPBLR_PRG: OTPBLR_PRG,
    _reserved19: [u8; 0x28],
    pub PRIVBB1R: PRIVBB1R,
    _reserved20: [u8; 0x24],
    pub WRPSGN1R_CUR: WRPSGN1R_CUR,
    pub WRPSGN1R_PRG: WRPSGN1R_PRG,
    _reserved22: [u8; 0x08],
    pub HDP1R_CUR: HDP1R_CUR,
    pub HDP1R_PRG: HDP1R_PRG,
    pub ECCCORR: ECCCORR,
    pub ECCDETR: ECCDETR,
    pub ECCDR: ECCDR,
    _reserved27: [u8; 0xdc],
    pub WRPSGN2R_CUR: WRPSGN2R_CUR,
    pub WRPSGN2R_PRG: WRPSGN2R_PRG,
    _reserved29: [u8; 0x08],
    pub HDP2R_CUR: HDP2R_CUR,
    pub HDP2R_PRG: HDP2R_PRG,
}
impl RegisterBlock {
    #[doc = "0x00 - FLASH access control register"]
    #[inline(always)]
    pub const fn ACR(&self) -> &ACR {
        &self.ACR
    }
    #[doc = "0x04 - FLASH key register"]
    #[inline(always)]
    pub const fn NSKEYR(&self) -> &NSKEYR {
        &self.NSKEYR
    }
    #[doc = "0x0c - FLASH option key register"]
    #[inline(always)]
    pub const fn OPTKEYR(&self) -> &OPTKEYR {
        &self.OPTKEYR
    }
    #[doc = "0x18 - FLASH operation status register"]
    #[inline(always)]
    pub const fn OPSR(&self) -> &OPSR {
        &self.OPSR
    }
    #[doc = "0x1c - FLASH option control register"]
    #[inline(always)]
    pub const fn OPTCR(&self) -> &OPTCR {
        &self.OPTCR
    }
    #[doc = "0x20 - FLASH non-secure status register"]
    #[inline(always)]
    pub const fn NSSR(&self) -> &NSSR {
        &self.NSSR
    }
    #[doc = "0x24 - FLASH secure status register"]
    #[inline(always)]
    pub const fn SECSR(&self) -> &SECSR {
        &self.SECSR
    }
    #[doc = "0x28 - FLASH Non Secure control register"]
    #[inline(always)]
    pub const fn NSCR(&self) -> &NSCR {
        &self.NSCR
    }
    #[doc = "0x30 - FLASH non-secure clear control register"]
    #[inline(always)]
    pub const fn NSCCR(&self) -> &NSCCR {
        &self.NSCCR
    }
    #[doc = "0x3c - FLASH privilege configuration register"]
    #[inline(always)]
    pub const fn PRIVCFGR(&self) -> &PRIVCFGR {
        &self.PRIVCFGR
    }
    #[doc = "0x48 - FLASH HDP extension register"]
    #[inline(always)]
    pub const fn HDPEXTR(&self) -> &HDPEXTR {
        &self.HDPEXTR
    }
    #[doc = "0x50 - FLASH option status register"]
    #[inline(always)]
    pub const fn OPTSR_CUR(&self) -> &OPTSR_CUR {
        &self.OPTSR_CUR
    }
    #[doc = "0x54 - FLASH option status register"]
    #[inline(always)]
    pub const fn OPTSR_PRG(&self) -> &OPTSR_PRG {
        &self.OPTSR_PRG
    }
    #[doc = "0x70 - FLASH option status register 2"]
    #[inline(always)]
    pub const fn OPTSR2_CUR(&self) -> &OPTSR2_CUR {
        &self.OPTSR2_CUR
    }
    #[doc = "0x74 - FLASH option status register 2"]
    #[inline(always)]
    pub const fn OPTSR2_PRG(&self) -> &OPTSR2_PRG {
        &self.OPTSR2_PRG
    }
    #[doc = "0x80 - FLASH non-secure unique boot entry register"]
    #[inline(always)]
    pub const fn NSBOOTR_CUR(&self) -> &NSBOOTR_CUR {
        &self.NSBOOTR_CUR
    }
    #[doc = "0x84 - FLASH non-secure unique boot entry address"]
    #[inline(always)]
    pub const fn NSBOOTR_PRG(&self) -> &NSBOOTR_PRG {
        &self.NSBOOTR_PRG
    }
    #[doc = "0x90 - FLASH non-secure OTP block lock"]
    #[inline(always)]
    pub const fn OTPBLR_CUR(&self) -> &OTPBLR_CUR {
        &self.OTPBLR_CUR
    }
    #[doc = "0x94 - FLASH non-secure OTP block lock"]
    #[inline(always)]
    pub const fn OTPBLR_PRG(&self) -> &OTPBLR_PRG {
        &self.OTPBLR_PRG
    }
    #[doc = "0xc0 - FLASH privilege register for bank 1"]
    #[inline(always)]
    pub const fn PRIVBB1R(&self) -> &PRIVBB1R {
        &self.PRIVBB1R
    }
    #[doc = "0xe8 - FLASH write sector protection for Bank1"]
    #[inline(always)]
    pub const fn WRPSGN1R_CUR(&self) -> &WRPSGN1R_CUR {
        &self.WRPSGN1R_CUR
    }
    #[doc = "0xec - FLASH write sector protection for Bank1"]
    #[inline(always)]
    pub const fn WRPSGN1R_PRG(&self) -> &WRPSGN1R_PRG {
        &self.WRPSGN1R_PRG
    }
    #[doc = "0xf8 - FLASH HDP Bank1 register"]
    #[inline(always)]
    pub const fn HDP1R_CUR(&self) -> &HDP1R_CUR {
        &self.HDP1R_CUR
    }
    #[doc = "0xfc - FLASH HDP Bank1 register"]
    #[inline(always)]
    pub const fn HDP1R_PRG(&self) -> &HDP1R_PRG {
        &self.HDP1R_PRG
    }
    #[doc = "0x100 - FLASH Flash ECC correction register"]
    #[inline(always)]
    pub const fn ECCCORR(&self) -> &ECCCORR {
        &self.ECCCORR
    }
    #[doc = "0x104 - FLASH ECC detection register"]
    #[inline(always)]
    pub const fn ECCDETR(&self) -> &ECCDETR {
        &self.ECCDETR
    }
    #[doc = "0x108 - FLASH ECC data"]
    #[inline(always)]
    pub const fn ECCDR(&self) -> &ECCDR {
        &self.ECCDR
    }
    #[doc = "0x1e8 - FLASH write sector protection for Bank2"]
    #[inline(always)]
    pub const fn WRPSGN2R_CUR(&self) -> &WRPSGN2R_CUR {
        &self.WRPSGN2R_CUR
    }
    #[doc = "0x1ec - FLASH write sector protection for Bank2"]
    #[inline(always)]
    pub const fn WRPSGN2R_PRG(&self) -> &WRPSGN2R_PRG {
        &self.WRPSGN2R_PRG
    }
    #[doc = "0x1f8 - FLASH HDP Bank2 register"]
    #[inline(always)]
    pub const fn HDP2R_CUR(&self) -> &HDP2R_CUR {
        &self.HDP2R_CUR
    }
    #[doc = "0x1fc - FLASH HDP Bank2 register"]
    #[inline(always)]
    pub const fn HDP2R_PRG(&self) -> &HDP2R_PRG {
        &self.HDP2R_PRG
    }
}
#[doc = "ACR (rw) register accessor: FLASH access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "FLASH access control register"]
pub mod acr;
#[doc = "NSKEYR (w) register accessor: FLASH key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nskeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nskeyr`] module"]
pub type NSKEYR = crate::Reg<nskeyr::NSKEYR_SPEC>;
#[doc = "FLASH key register"]
pub mod nskeyr;
#[doc = "OPTKEYR (w) register accessor: FLASH option key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`] module"]
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "OPSR (r) register accessor: FLASH operation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opsr`] module"]
pub type OPSR = crate::Reg<opsr::OPSR_SPEC>;
#[doc = "FLASH operation status register"]
pub mod opsr;
#[doc = "OPTCR (rw) register accessor: FLASH option control register\n\nYou can [`read`](crate::Reg::read) this register and get [`optcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr`] module"]
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "NSSR (r) register accessor: FLASH non-secure status register\n\nYou can [`read`](crate::Reg::read) this register and get [`nssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nssr`] module"]
pub type NSSR = crate::Reg<nssr::NSSR_SPEC>;
#[doc = "FLASH non-secure status register"]
pub mod nssr;
#[doc = "SECSR (r) register accessor: FLASH secure status register\n\nYou can [`read`](crate::Reg::read) this register and get [`secsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secsr`] module"]
pub type SECSR = crate::Reg<secsr::SECSR_SPEC>;
#[doc = "FLASH secure status register"]
pub mod secsr;
#[doc = "NSCR (rw) register accessor: FLASH Non Secure control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nscr`] module"]
pub type NSCR = crate::Reg<nscr::NSCR_SPEC>;
#[doc = "FLASH Non Secure control register"]
pub mod nscr;
#[doc = "NSCCR (w) register accessor: FLASH non-secure clear control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsccr`] module"]
pub type NSCCR = crate::Reg<nsccr::NSCCR_SPEC>;
#[doc = "FLASH non-secure clear control register"]
pub mod nsccr;
#[doc = "PRIVCFGR (w) register accessor: FLASH privilege configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`] module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
#[doc = "FLASH privilege configuration register"]
pub mod privcfgr;
#[doc = "HDPEXTR (rw) register accessor: FLASH HDP extension register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdpextr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpextr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdpextr`] module"]
pub type HDPEXTR = crate::Reg<hdpextr::HDPEXTR_SPEC>;
#[doc = "FLASH HDP extension register"]
pub mod hdpextr;
#[doc = "OPTSR_CUR (r) register accessor: FLASH option status register\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_cur`] module"]
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CUR_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "OPTSR_PRG (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_prg`] module"]
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRG_SPEC>;
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "OPTSR2_CUR (r) register accessor: FLASH option status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr2_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr2_cur`] module"]
pub type OPTSR2_CUR = crate::Reg<optsr2_cur::OPTSR2_CUR_SPEC>;
#[doc = "FLASH option status register 2"]
pub mod optsr2_cur;
#[doc = "OPTSR2_PRG (rw) register accessor: FLASH option status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr2_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr2_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr2_prg`] module"]
pub type OPTSR2_PRG = crate::Reg<optsr2_prg::OPTSR2_PRG_SPEC>;
#[doc = "FLASH option status register 2"]
pub mod optsr2_prg;
#[doc = "NSBOOTR_CUR (r) register accessor: FLASH non-secure unique boot entry register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsbootr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsbootr_cur`] module"]
pub type NSBOOTR_CUR = crate::Reg<nsbootr_cur::NSBOOTR_CUR_SPEC>;
#[doc = "FLASH non-secure unique boot entry register"]
pub mod nsbootr_cur;
#[doc = "NSBOOTR_PRG (rw) register accessor: FLASH non-secure unique boot entry address\n\nYou can [`read`](crate::Reg::read) this register and get [`nsbootr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsbootr_prg`] module"]
pub type NSBOOTR_PRG = crate::Reg<nsbootr_prg::NSBOOTR_PRG_SPEC>;
#[doc = "FLASH non-secure unique boot entry address"]
pub mod nsbootr_prg;
#[doc = "OTPBLR_CUR (r) register accessor: FLASH non-secure OTP block lock\n\nYou can [`read`](crate::Reg::read) this register and get [`otpblr_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpblr_cur`] module"]
pub type OTPBLR_CUR = crate::Reg<otpblr_cur::OTPBLR_CUR_SPEC>;
#[doc = "FLASH non-secure OTP block lock"]
pub mod otpblr_cur;
#[doc = "OTPBLR_PRG (rw) register accessor: FLASH non-secure OTP block lock\n\nYou can [`read`](crate::Reg::read) this register and get [`otpblr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpblr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpblr_prg`] module"]
pub type OTPBLR_PRG = crate::Reg<otpblr_prg::OTPBLR_PRG_SPEC>;
#[doc = "FLASH non-secure OTP block lock"]
pub mod otpblr_prg;
#[doc = "PRIVBB1R (rw) register accessor: FLASH privilege register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`privbb1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privbb1r`] module"]
pub type PRIVBB1R = crate::Reg<privbb1r::PRIVBB1R_SPEC>;
#[doc = "FLASH privilege register for bank 1"]
pub mod privbb1r;
#[doc = "WRPSGN1R_CUR (r) register accessor: FLASH write sector protection for Bank1\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpsgn1r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpsgn1r_cur`] module"]
pub type WRPSGN1R_CUR = crate::Reg<wrpsgn1r_cur::WRPSGN1R_CUR_SPEC>;
#[doc = "FLASH write sector protection for Bank1"]
pub mod wrpsgn1r_cur;
#[doc = "WRPSGN1R_PRG (rw) register accessor: FLASH write sector protection for Bank1\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpsgn1r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsgn1r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpsgn1r_prg`] module"]
pub type WRPSGN1R_PRG = crate::Reg<wrpsgn1r_prg::WRPSGN1R_PRG_SPEC>;
#[doc = "FLASH write sector protection for Bank1"]
pub mod wrpsgn1r_prg;
#[doc = "HDP1R_CUR (r) register accessor: FLASH HDP Bank1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdp1r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdp1r_cur`] module"]
pub type HDP1R_CUR = crate::Reg<hdp1r_cur::HDP1R_CUR_SPEC>;
#[doc = "FLASH HDP Bank1 register"]
pub mod hdp1r_cur;
#[doc = "HDP1R_PRG (r) register accessor: FLASH HDP Bank1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdp1r_prg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdp1r_prg`] module"]
pub type HDP1R_PRG = crate::Reg<hdp1r_prg::HDP1R_PRG_SPEC>;
#[doc = "FLASH HDP Bank1 register"]
pub mod hdp1r_prg;
#[doc = "ECCCORR (rw) register accessor: FLASH Flash ECC correction register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecccorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecccorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecccorr`] module"]
pub type ECCCORR = crate::Reg<ecccorr::ECCCORR_SPEC>;
#[doc = "FLASH Flash ECC correction register"]
pub mod ecccorr;
#[doc = "ECCDETR (rw) register accessor: FLASH ECC detection register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccdetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccdetr`] module"]
pub type ECCDETR = crate::Reg<eccdetr::ECCDETR_SPEC>;
#[doc = "FLASH ECC detection register"]
pub mod eccdetr;
#[doc = "ECCDR (r) register accessor: FLASH ECC data\n\nYou can [`read`](crate::Reg::read) this register and get [`eccdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccdr`] module"]
pub type ECCDR = crate::Reg<eccdr::ECCDR_SPEC>;
#[doc = "FLASH ECC data"]
pub mod eccdr;
#[doc = "WRPSGN2R_CUR (r) register accessor: FLASH write sector protection for Bank2\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpsgn2r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpsgn2r_cur`] module"]
pub type WRPSGN2R_CUR = crate::Reg<wrpsgn2r_cur::WRPSGN2R_CUR_SPEC>;
#[doc = "FLASH write sector protection for Bank2"]
pub mod wrpsgn2r_cur;
#[doc = "WRPSGN2R_PRG (rw) register accessor: FLASH write sector protection for Bank2\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpsgn2r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsgn2r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpsgn2r_prg`] module"]
pub type WRPSGN2R_PRG = crate::Reg<wrpsgn2r_prg::WRPSGN2R_PRG_SPEC>;
#[doc = "FLASH write sector protection for Bank2"]
pub mod wrpsgn2r_prg;
#[doc = "HDP2R_CUR (r) register accessor: FLASH HDP Bank2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdp2r_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdp2r_cur`] module"]
pub type HDP2R_CUR = crate::Reg<hdp2r_cur::HDP2R_CUR_SPEC>;
#[doc = "FLASH HDP Bank2 register"]
pub mod hdp2r_cur;
#[doc = "HDP2R_PRG (rw) register accessor: FLASH HDP Bank2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdp2r_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp2r_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdp2r_prg`] module"]
pub type HDP2R_PRG = crate::Reg<hdp2r_prg::HDP2R_PRG_SPEC>;
#[doc = "FLASH HDP Bank2 register"]
pub mod hdp2r_prg;
