#[doc = "Register `PIDR2` reader"]
pub type R = crate::R<PIDR2_SPEC>;
#[doc = "JEP106 identity code bits \\[6:4\\]\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEP106ID_A {
    #[doc = "2: STMicroelectronics JEDEC code"]
    B_0x2 = 2,
}
impl From<JEP106ID_A> for u8 {
    #[inline(always)]
    fn from(variant: JEP106ID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEP106ID_A {
    type Ux = u8;
}
impl crate::IsEnum for JEP106ID_A {}
#[doc = "Field `JEP106ID` reader - JEP106 identity code bits \\[6:4\\]"]
pub type JEP106ID_R = crate::FieldReader<JEP106ID_A>;
impl JEP106ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<JEP106ID_A> {
        match self.bits {
            2 => Some(JEP106ID_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "STMicroelectronics JEDEC code"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == JEP106ID_A::B_0x2
    }
}
#[doc = "JEDEC assigned value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEDEC_A {
    #[doc = "1: designer identification specified by JEDEC"]
    B_0x1 = 1,
}
impl From<JEDEC_A> for bool {
    #[inline(always)]
    fn from(variant: JEDEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEDEC` reader - JEDEC assigned value"]
pub type JEDEC_R = crate::BitReader<JEDEC_A>;
impl JEDEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<JEDEC_A> {
        match self.bits {
            true => Some(JEDEC_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "designer identification specified by JEDEC"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JEDEC_A::B_0x1
    }
}
#[doc = "component revision number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REVISION_A {
    #[doc = "0: r0p0"]
    B_0x0 = 0,
}
impl From<REVISION_A> for u8 {
    #[inline(always)]
    fn from(variant: REVISION_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REVISION_A {
    type Ux = u8;
}
impl crate::IsEnum for REVISION_A {}
#[doc = "Field `REVISION` reader - component revision number"]
pub type REVISION_R = crate::FieldReader<REVISION_A>;
impl REVISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REVISION_A> {
        match self.bits {
            0 => Some(REVISION_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "r0p0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == REVISION_A::B_0x0
    }
}
impl R {
    #[doc = "Bits 0:2 - JEP106 identity code bits \\[6:4\\]"]
    #[inline(always)]
    pub fn JEP106ID(&self) -> JEP106ID_R {
        JEP106ID_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEDEC assigned value"]
    #[inline(always)]
    pub fn JEDEC(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - component revision number"]
    #[inline(always)]
    pub fn REVISION(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR2_SPEC;
impl crate::RegisterSpec for PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr2::R`](R) reader structure"]
impl crate::Readable for PIDR2_SPEC {}
#[doc = "`reset()` method sets PIDR2 to value 0x0a"]
impl crate::Resettable for PIDR2_SPEC {
    const RESET_VALUE: u32 = 0x0a;
}
