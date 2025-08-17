#[doc = "Register `CIDR2` reader"]
pub type R = crate::R<CIDR2_SPEC>;
#[doc = "component identification bits \\[23:16\\]\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREAMBLE_A {
    #[doc = "5: common identification value"]
    B_0x05 = 5,
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
#[doc = "Field `PREAMBLE` reader - component identification bits \\[23:16\\]"]
pub type PREAMBLE_R = crate::FieldReader<PREAMBLE_A>;
impl PREAMBLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PREAMBLE_A> {
        match self.bits {
            5 => Some(PREAMBLE_A::B_0x05),
            _ => None,
        }
    }
    #[doc = "common identification value"]
    #[inline(always)]
    pub fn is_B_0x05(&self) -> bool {
        *self == PREAMBLE_A::B_0x05
    }
}
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[23:16\\]"]
    #[inline(always)]
    pub fn PREAMBLE(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR2_SPEC;
impl crate::RegisterSpec for CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr2::R`](R) reader structure"]
impl crate::Readable for CIDR2_SPEC {}
#[doc = "`reset()` method sets CIDR2 to value 0x05"]
impl crate::Resettable for CIDR2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
