#[doc = "Register `PIDR1` reader"]
pub type R = crate::R<PIDR1_SPEC>;
#[doc = "part number bits \\[11:8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARTNUM_A {
    #[doc = "0: DBGMCU part number"]
    B_0x0 = 0,
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
#[doc = "Field `PARTNUM` reader - part number bits \\[11:8\\]"]
pub type PARTNUM_R = crate::FieldReader<PARTNUM_A>;
impl PARTNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PARTNUM_A> {
        match self.bits {
            0 => Some(PARTNUM_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "DBGMCU part number"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PARTNUM_A::B_0x0
    }
}
#[doc = "JEP106 identity code bits \\[3:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEP106ID_A {
    #[doc = "0: STMicroelectronics JEDEC code"]
    B_0x0 = 0,
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
#[doc = "Field `JEP106ID` reader - JEP106 identity code bits \\[3:0\\]"]
pub type JEP106ID_R = crate::FieldReader<JEP106ID_A>;
impl JEP106ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<JEP106ID_A> {
        match self.bits {
            0 => Some(JEP106ID_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "STMicroelectronics JEDEC code"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JEP106ID_A::B_0x0
    }
}
impl R {
    #[doc = "Bits 0:3 - part number bits \\[11:8\\]"]
    #[inline(always)]
    pub fn PARTNUM(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity code bits \\[3:0\\]"]
    #[inline(always)]
    pub fn JEP106ID(&self) -> JEP106ID_R {
        JEP106ID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR1_SPEC;
impl crate::RegisterSpec for PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr1::R`](R) reader structure"]
impl crate::Readable for PIDR1_SPEC {}
#[doc = "`reset()` method sets PIDR1 to value 0"]
impl crate::Resettable for PIDR1_SPEC {}
