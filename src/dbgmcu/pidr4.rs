#[doc = "Register `PIDR4` reader"]
pub type R = crate::R<PIDR4_SPEC>;
#[doc = "JEP106 continuation code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEP106CON_A {
    #[doc = "0: STMicroelectronics JEDEC code"]
    B_0x0 = 0,
}
impl From<JEP106CON_A> for u8 {
    #[inline(always)]
    fn from(variant: JEP106CON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEP106CON_A {
    type Ux = u8;
}
impl crate::IsEnum for JEP106CON_A {}
#[doc = "Field `JEP106CON` reader - JEP106 continuation code"]
pub type JEP106CON_R = crate::FieldReader<JEP106CON_A>;
impl JEP106CON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<JEP106CON_A> {
        match self.bits {
            0 => Some(JEP106CON_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "STMicroelectronics JEDEC code"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JEP106CON_A::B_0x0
    }
}
#[doc = "register file size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: The register file occupies a single 4-Kbyte region."]
    B_0x0 = 0,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for SIZE_A {}
#[doc = "Field `SIZE` reader - register file size"]
pub type SIZE_R = crate::FieldReader<SIZE_A>;
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "The register file occupies a single 4-Kbyte region."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SIZE_A::B_0x0
    }
}
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn JEP106CON(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - register file size"]
    #[inline(always)]
    pub fn SIZE(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR4_SPEC;
impl crate::RegisterSpec for PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr4::R`](R) reader structure"]
impl crate::Readable for PIDR4_SPEC {}
#[doc = "`reset()` method sets PIDR4 to value 0"]
impl crate::Resettable for PIDR4_SPEC {}
