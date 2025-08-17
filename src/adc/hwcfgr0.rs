#[doc = "Register `HWCFGR0` reader"]
pub type R = crate::R<HWCFGR0_SPEC>;
#[doc = "Number of ADCs implemented\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCNUM_A {
    #[doc = "1: One ADC instance implemented"]
    B_0x1 = 1,
    #[doc = "2: Two ADC instances implemented"]
    B_0x2 = 2,
    #[doc = "3: Three ADCs instances implemented"]
    B_0x3 = 3,
}
impl From<ADCNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCNUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCNUM_A {
    type Ux = u8;
}
impl crate::IsEnum for ADCNUM_A {}
#[doc = "Field `ADCNUM` reader - Number of ADCs implemented"]
pub type ADCNUM_R = crate::FieldReader<ADCNUM_A>;
impl ADCNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADCNUM_A> {
        match self.bits {
            1 => Some(ADCNUM_A::B_0x1),
            2 => Some(ADCNUM_A::B_0x2),
            3 => Some(ADCNUM_A::B_0x3),
            _ => None,
        }
    }
    #[doc = "One ADC instance implemented"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADCNUM_A::B_0x1
    }
    #[doc = "Two ADC instances implemented"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ADCNUM_A::B_0x2
    }
    #[doc = "Three ADCs instances implemented"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ADCNUM_A::B_0x3
    }
}
#[doc = "Number of pipeline stages\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULPIPE_A {
    #[doc = "1: One-stage pipeline"]
    B_0x1 = 1,
}
impl From<MULPIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: MULPIPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MULPIPE_A {
    type Ux = u8;
}
impl crate::IsEnum for MULPIPE_A {}
#[doc = "Field `MULPIPE` reader - Number of pipeline stages"]
pub type MULPIPE_R = crate::FieldReader<MULPIPE_A>;
impl MULPIPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MULPIPE_A> {
        match self.bits {
            1 => Some(MULPIPE_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "One-stage pipeline"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MULPIPE_A::B_0x1
    }
}
#[doc = "Number of option bits 0002: 2 option bits implemented in the ADC option register (ADC_OR) at address offset 0xC8\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPBITS_A {
    #[doc = "0: No option register implemented"]
    B_0x0 = 0,
}
impl From<OPBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: OPBITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPBITS_A {
    type Ux = u8;
}
impl crate::IsEnum for OPBITS_A {}
#[doc = "Field `OPBITS` reader - Number of option bits 0002: 2 option bits implemented in the ADC option register (ADC_OR) at address offset 0xC8"]
pub type OPBITS_R = crate::FieldReader<OPBITS_A>;
impl OPBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPBITS_A> {
        match self.bits {
            0 => Some(OPBITS_A::B_0x0),
            _ => None,
        }
    }
    #[doc = "No option register implemented"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPBITS_A::B_0x0
    }
}
#[doc = "Idle value for non-selected channels\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLEVALUE_A {
    #[doc = "0: Dummy channel selection is 0x13"]
    B_0x0 = 0,
    #[doc = "1: Dummy channel selection is 0x1F"]
    B_0x1 = 1,
}
impl From<IDLEVALUE_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLEVALUE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDLEVALUE_A {
    type Ux = u8;
}
impl crate::IsEnum for IDLEVALUE_A {}
#[doc = "Field `IDLEVALUE` reader - Idle value for non-selected channels"]
pub type IDLEVALUE_R = crate::FieldReader<IDLEVALUE_A>;
impl IDLEVALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDLEVALUE_A> {
        match self.bits {
            0 => Some(IDLEVALUE_A::B_0x0),
            1 => Some(IDLEVALUE_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "Dummy channel selection is 0x13"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IDLEVALUE_A::B_0x0
    }
    #[doc = "Dummy channel selection is 0x1F"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IDLEVALUE_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of ADCs implemented"]
    #[inline(always)]
    pub fn ADCNUM(&self) -> ADCNUM_R {
        ADCNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of pipeline stages"]
    #[inline(always)]
    pub fn MULPIPE(&self) -> MULPIPE_R {
        MULPIPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of option bits 0002: 2 option bits implemented in the ADC option register (ADC_OR) at address offset 0xC8"]
    #[inline(always)]
    pub fn OPBITS(&self) -> OPBITS_R {
        OPBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Idle value for non-selected channels"]
    #[inline(always)]
    pub fn IDLEVALUE(&self) -> IDLEVALUE_R {
        IDLEVALUE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "ADC hardware configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR0_SPEC;
impl crate::RegisterSpec for HWCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr0::R`](R) reader structure"]
impl crate::Readable for HWCFGR0_SPEC {}
#[doc = "`reset()` method sets HWCFGR0 to value 0x1211"]
impl crate::Resettable for HWCFGR0_SPEC {
    const RESET_VALUE: u32 = 0x1211;
}
