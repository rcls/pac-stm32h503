#[doc = "Register `PIDR3` reader"]
pub type R = crate::R<PIDR3_SPEC>;
#[doc = "customer modified\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMOD_A {
    #[doc = "0: no customer modifications"]
    B_0x0 = 0,
}
impl From<CMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMOD_A {
    type Ux = u8;
}
impl crate::IsEnum for CMOD_A {}
#[doc = "Field `CMOD` reader - customer modified"]
pub type CMOD_R = crate::FieldReader<CMOD_A>;
impl CMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMOD_A> {
        match self.bits {
            0 => Some(CMOD_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "no customer modifications"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CMOD_A::B_0x0
    }
}
#[doc = "metal fix version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REVAND_A {
    #[doc = "0: no metal fix"]
    B_0x0 = 0,
}
impl From<REVAND_A> for u8 {
    #[inline(always)]
    fn from(variant: REVAND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REVAND_A {
    type Ux = u8;
}
impl crate::IsEnum for REVAND_A {}
#[doc = "Field `REVAND` reader - metal fix version"]
pub type REVAND_R = crate::FieldReader<REVAND_A>;
impl REVAND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REVAND_A> {
        match self.bits {
            0 => Some(REVAND_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "no metal fix"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == REVAND_A::B_0x0
    }
}
impl R {
    #[doc = "Bits 0:3 - customer modified"]
    #[inline(always)]
    pub fn CMOD(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - metal fix version"]
    #[inline(always)]
    pub fn REVAND(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR3_SPEC;
impl crate::RegisterSpec for PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr3::R`](R) reader structure"]
impl crate::Readable for PIDR3_SPEC {}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for PIDR3_SPEC {}
