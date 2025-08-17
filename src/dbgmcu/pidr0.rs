#[doc = "Register `PIDR0` reader"]
pub type R = crate::R<PIDR0_SPEC>;
#[doc = "part number bits \\[7:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARTNUM_A {
    #[doc = "0: DBGMCU part number"]
    B_0x00 = 0,
}
impl From<PARTNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: PARTNUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PARTNUM_A {
    type Ux = u8;
}
impl crate::IsEnum for PARTNUM_A {}
#[doc = "Field `PARTNUM` reader - part number bits \\[7:0\\]"]
pub type PARTNUM_R = crate::FieldReader<PARTNUM_A>;
impl PARTNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PARTNUM_A> {
        match self.bits {
            0 => Some(PARTNUM_A::B_0x00),
            _ => None,
        }
    }
    #[doc = "DBGMCU part number"]
    #[inline(always)]
    pub fn is_B_0x00(&self) -> bool {
        *self == PARTNUM_A::B_0x00
    }
}
impl R {
    #[doc = "Bits 0:7 - part number bits \\[7:0\\]"]
    #[inline(always)]
    pub fn PARTNUM(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR0_SPEC;
impl crate::RegisterSpec for PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr0::R`](R) reader structure"]
impl crate::Readable for PIDR0_SPEC {}
#[doc = "`reset()` method sets PIDR0 to value 0"]
impl crate::Resettable for PIDR0_SPEC {}
