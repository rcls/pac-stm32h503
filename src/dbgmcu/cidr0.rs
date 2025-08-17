#[doc = "Register `CIDR0` reader"]
pub type R = crate::R<CIDR0_SPEC>;
#[doc = "component identification bits \\[7:0\\]\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREAMBLE_A {
    #[doc = "13: common identification value"]
    B_0x0D = 13,
}
impl From<PREAMBLE_A> for u8 {
    #[inline(always)]
    fn from(variant: PREAMBLE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PREAMBLE_A {
    type Ux = u8;
}
impl crate::IsEnum for PREAMBLE_A {}
#[doc = "Field `PREAMBLE` reader - component identification bits \\[7:0\\]"]
pub type PREAMBLE_R = crate::FieldReader<PREAMBLE_A>;
impl PREAMBLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PREAMBLE_A> {
        match self.bits {
            13 => Some(PREAMBLE_A::B_0x0D),
            _ => None,
        }
    }
    #[doc = "common identification value"]
    #[inline(always)]
    pub fn is_B_0x0D(&self) -> bool {
        *self == PREAMBLE_A::B_0x0D
    }
}
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[7:0\\]"]
    #[inline(always)]
    pub fn PREAMBLE(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR0_SPEC;
impl crate::RegisterSpec for CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr0::R`](R) reader structure"]
impl crate::Readable for CIDR0_SPEC {}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for CIDR0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
