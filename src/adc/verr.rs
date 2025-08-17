#[doc = "Register `VERR` reader"]
pub type R = crate::R<VERR_SPEC>;
#[doc = "Minor revision These bits returns the ADC IP minor revision 0002: Major revision = X.2\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MINREV_A {
    #[doc = "1: Major revision = X.1"]
    B_0x1 = 1,
}
impl From<MINREV_A> for u8 {
    #[inline(always)]
    fn from(variant: MINREV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MINREV_A {
    type Ux = u8;
}
impl crate::IsEnum for MINREV_A {}
#[doc = "Field `MINREV` reader - Minor revision These bits returns the ADC IP minor revision 0002: Major revision = X.2"]
pub type MINREV_R = crate::FieldReader<MINREV_A>;
impl MINREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MINREV_A> {
        match self.bits {
            1 => Some(MINREV_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "Major revision = X.1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MINREV_A::B_0x1
    }
}
#[doc = "Major revision These bits returns the ADC IP major revision\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAJREV_A {
    #[doc = "1: Major revision = 1.X"]
    B_0x1 = 1,
}
impl From<MAJREV_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJREV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAJREV_A {
    type Ux = u8;
}
impl crate::IsEnum for MAJREV_A {}
#[doc = "Field `MAJREV` reader - Major revision These bits returns the ADC IP major revision"]
pub type MAJREV_R = crate::FieldReader<MAJREV_A>;
impl MAJREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAJREV_A> {
        match self.bits {
            1 => Some(MAJREV_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "Major revision = 1.X"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MAJREV_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:3 - Minor revision These bits returns the ADC IP minor revision 0002: Major revision = X.2"]
    #[inline(always)]
    pub fn MINREV(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major revision These bits returns the ADC IP major revision"]
    #[inline(always)]
    pub fn MAJREV(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "ADC version register\n\nYou can [`read`](crate::Reg::read) this register and get [`verr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERR_SPEC;
impl crate::RegisterSpec for VERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verr::R`](R) reader structure"]
impl crate::Readable for VERR_SPEC {}
#[doc = "`reset()` method sets VERR to value 0x12"]
impl crate::Resettable for VERR_SPEC {
    const RESET_VALUE: u32 = 0x12;
}
