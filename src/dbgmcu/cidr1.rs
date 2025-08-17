#[doc = "Register `CIDR1` reader"]
pub type R = crate::R<CIDR1_SPEC>;
#[doc = "component identification bits \\[11:8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREAMBLE_A {
    #[doc = "0: common identification value"]
    B_0x0 = 0,
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
#[doc = "Field `PREAMBLE` reader - component identification bits \\[11:8\\]"]
pub type PREAMBLE_R = crate::FieldReader<PREAMBLE_A>;
impl PREAMBLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PREAMBLE_A> {
        match self.bits {
            0 => Some(PREAMBLE_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "common identification value"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PREAMBLE_A::B_0x0
    }
}
#[doc = "component identification bits \\[15:12\\] - component class\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLASS_A {
    #[doc = "15: Non-CoreSight component"]
    B_0xF = 15,
}
impl From<CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLASS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLASS_A {
    type Ux = u8;
}
impl crate::IsEnum for CLASS_A {}
#[doc = "Field `CLASS` reader - component identification bits \\[15:12\\] - component class"]
pub type CLASS_R = crate::FieldReader<CLASS_A>;
impl CLASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLASS_A> {
        match self.bits {
            15 => Some(CLASS_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "Non-CoreSight component"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == CLASS_A::B_0xF
    }
}
impl R {
    #[doc = "Bits 0:3 - component identification bits \\[11:8\\]"]
    #[inline(always)]
    pub fn PREAMBLE(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - component identification bits \\[15:12\\] - component class"]
    #[inline(always)]
    pub fn CLASS(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR1_SPEC;
impl crate::RegisterSpec for CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr1::R`](R) reader structure"]
impl crate::Readable for CIDR1_SPEC {}
#[doc = "`reset()` method sets CIDR1 to value 0xf0"]
impl crate::Resettable for CIDR1_SPEC {
    const RESET_VALUE: u32 = 0xf0;
}
