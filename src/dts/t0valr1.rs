#[doc = "Register `T0VALR1` reader"]
pub type R = crate::R<T0VALR1_SPEC>;
#[doc = "Field `TS1_FMT0` reader - Engineering value of the frequency measured at T0 for temperature sensor 1 This value is expressed in 0.1 kHz."]
pub type TS1_FMT0_R = crate::FieldReader<u16>;
#[doc = "Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS1_T0_A {
    #[doc = "0: 30 C"]
    B_0x0 = 0,
    #[doc = "1: 130 C"]
    B_0x1 = 1,
}
impl From<TS1_T0_A> for u8 {
    #[inline(always)]
    fn from(variant: TS1_T0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS1_T0_A {
    type Ux = u8;
}
impl crate::IsEnum for TS1_T0_A {}
#[doc = "Field `TS1_T0` reader - Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used."]
pub type TS1_T0_R = crate::FieldReader<TS1_T0_A>;
impl TS1_T0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TS1_T0_A> {
        match self.bits {
            0 => Some(TS1_T0_A::B_0x0),
            1 => Some(TS1_T0_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "30 C"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TS1_T0_A::B_0x0
    }
    #[doc = "130 C"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TS1_T0_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:15 - Engineering value of the frequency measured at T0 for temperature sensor 1 This value is expressed in 0.1 kHz."]
    #[inline(always)]
    pub fn TS1_FMT0(&self) -> TS1_FMT0_R {
        TS1_FMT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used."]
    #[inline(always)]
    pub fn TS1_T0(&self) -> TS1_T0_R {
        TS1_T0_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "Temperature sensor T0 value register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`t0valr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0VALR1_SPEC;
impl crate::RegisterSpec for T0VALR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0valr1::R`](R) reader structure"]
impl crate::Readable for T0VALR1_SPEC {}
#[doc = "`reset()` method sets T0VALR1 to value 0"]
impl crate::Resettable for T0VALR1_SPEC {}
