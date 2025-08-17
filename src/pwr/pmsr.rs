#[doc = "Register `PMSR` reader"]
pub type R = crate::R<PMSR_SPEC>;
#[doc = "Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF_A {
    #[doc = "0: system has not been in Stop mode."]
    B_0x0 = 0,
    #[doc = "1: system has been in Stop mode."]
    B_0x1 = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPF` reader - Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit."]
pub type STOPF_R = crate::BitReader<STOPF_A>;
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::B_0x0,
            true => STOPF_A::B_0x1,
        }
    }
    #[doc = "system has not been in Stop mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STOPF_A::B_0x0
    }
    #[doc = "system has been in Stop mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STOPF_A::B_0x1
    }
}
#[doc = "System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBF_A {
    #[doc = "0: system has not been in Standby mode."]
    B_0x0 = 0,
    #[doc = "1: system has been in Standby mode."]
    B_0x1 = 1,
}
impl From<SBF_A> for bool {
    #[inline(always)]
    fn from(variant: SBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` reader - System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit."]
pub type SBF_R = crate::BitReader<SBF_A>;
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBF_A {
        match self.bits {
            false => SBF_A::B_0x0,
            true => SBF_A::B_0x1,
        }
    }
    #[doc = "system has not been in Standby mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SBF_A::B_0x0
    }
    #[doc = "system has been in Standby mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SBF_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 5 - Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit."]
    #[inline(always)]
    pub fn STOPF(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit."]
    #[inline(always)]
    pub fn SBF(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "PWR status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMSR_SPEC;
impl crate::RegisterSpec for PMSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmsr::R`](R) reader structure"]
impl crate::Readable for PMSR_SPEC {}
#[doc = "`reset()` method sets PMSR to value 0"]
impl crate::Resettable for PMSR_SPEC {}
