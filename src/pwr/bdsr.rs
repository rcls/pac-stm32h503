#[doc = "Register `BDSR` reader"]
pub type R = crate::R<BDSR_SPEC>;
#[doc = "backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRRDY_A {
    #[doc = "0: backup regulator not ready"]
    B_0x0 = 0,
    #[doc = "1: backup regulator ready"]
    B_0x1 = 1,
}
impl From<BRRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BRRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRRDY` reader - backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
pub type BRRDY_R = crate::BitReader<BRRDY_A>;
impl BRRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRRDY_A {
        match self.bits {
            false => BRRDY_A::B_0x0,
            true => BRRDY_A::B_0x1,
        }
    }
    #[doc = "backup regulator not ready"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BRRDY_A::B_0x0
    }
    #[doc = "backup regulator ready"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BRRDY_A::B_0x1
    }
}
#[doc = "V BAT level monitoring versus low threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATL_A {
    #[doc = "0: V BAT level above low threshold level"]
    B_0x0 = 0,
    #[doc = "1: V BAT level equal or below low threshold level"]
    B_0x1 = 1,
}
impl From<VBATL_A> for bool {
    #[inline(always)]
    fn from(variant: VBATL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATL` reader - V BAT level monitoring versus low threshold"]
pub type VBATL_R = crate::BitReader<VBATL_A>;
impl VBATL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATL_A {
        match self.bits {
            false => VBATL_A::B_0x0,
            true => VBATL_A::B_0x1,
        }
    }
    #[doc = "V BAT level above low threshold level"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VBATL_A::B_0x0
    }
    #[doc = "V BAT level equal or below low threshold level"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VBATL_A::B_0x1
    }
}
#[doc = "V BAT level monitoring versus high threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATH_A {
    #[doc = "0: V BAT level below high threshold level"]
    B_0x0 = 0,
    #[doc = "1: V BAT level equal or above high threshold level"]
    B_0x1 = 1,
}
impl From<VBATH_A> for bool {
    #[inline(always)]
    fn from(variant: VBATH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATH` reader - V BAT level monitoring versus high threshold"]
pub type VBATH_R = crate::BitReader<VBATH_A>;
impl VBATH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATH_A {
        match self.bits {
            false => VBATH_A::B_0x0,
            true => VBATH_A::B_0x1,
        }
    }
    #[doc = "V BAT level below high threshold level"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VBATH_A::B_0x0
    }
    #[doc = "V BAT level equal or above high threshold level"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VBATH_A::B_0x1
    }
}
#[doc = "temperature level monitoring versus low threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPL_A {
    #[doc = "0: temperature above low threshold level"]
    B_0x0 = 0,
    #[doc = "1: temperature equal or below low threshold level"]
    B_0x1 = 1,
}
impl From<TEMPL_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPL` reader - temperature level monitoring versus low threshold"]
pub type TEMPL_R = crate::BitReader<TEMPL_A>;
impl TEMPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEMPL_A {
        match self.bits {
            false => TEMPL_A::B_0x0,
            true => TEMPL_A::B_0x1,
        }
    }
    #[doc = "temperature above low threshold level"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEMPL_A::B_0x0
    }
    #[doc = "temperature equal or below low threshold level"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEMPL_A::B_0x1
    }
}
#[doc = "temperature level monitoring versus high threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPH_A {
    #[doc = "0: temperature below high threshold level"]
    B_0x0 = 0,
    #[doc = "1: temperature equal or above high threshold level"]
    B_0x1 = 1,
}
impl From<TEMPH_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPH` reader - temperature level monitoring versus high threshold"]
pub type TEMPH_R = crate::BitReader<TEMPH_A>;
impl TEMPH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEMPH_A {
        match self.bits {
            false => TEMPH_A::B_0x0,
            true => TEMPH_A::B_0x1,
        }
    }
    #[doc = "temperature below high threshold level"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TEMPH_A::B_0x0
    }
    #[doc = "temperature equal or above high threshold level"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TEMPH_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 16 - backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
    #[inline(always)]
    pub fn BRRDY(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - V BAT level monitoring versus low threshold"]
    #[inline(always)]
    pub fn VBATL(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - V BAT level monitoring versus high threshold"]
    #[inline(always)]
    pub fn VBATH(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - temperature level monitoring versus low threshold"]
    #[inline(always)]
    pub fn TEMPL(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - temperature level monitoring versus high threshold"]
    #[inline(always)]
    pub fn TEMPH(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "PWR Backup domain status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDSR_SPEC;
impl crate::RegisterSpec for BDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdsr::R`](R) reader structure"]
impl crate::Readable for BDSR_SPEC {}
#[doc = "`reset()` method sets BDSR to value 0"]
impl crate::Resettable for BDSR_SPEC {}
