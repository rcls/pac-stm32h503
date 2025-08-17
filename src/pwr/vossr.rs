#[doc = "Register `VOSSR` reader"]
pub type R = crate::R<VOSSR_SPEC>;
#[doc = "Ready bit for V CORE voltage scaling output selection.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSRDY_A {
    #[doc = "0: Not ready, voltage level below VOS selected level."]
    B_0x0 = 0,
    #[doc = "1: Ready, voltage level at or above VOS selected level."]
    B_0x1 = 1,
}
impl From<VOSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: VOSRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOSRDY` reader - Ready bit for V CORE voltage scaling output selection."]
pub type VOSRDY_R = crate::BitReader<VOSRDY_A>;
impl VOSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOSRDY_A {
        match self.bits {
            false => VOSRDY_A::B_0x0,
            true => VOSRDY_A::B_0x1,
        }
    }
    #[doc = "Not ready, voltage level below VOS selected level."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VOSRDY_A::B_0x0
    }
    #[doc = "Ready, voltage level at or above VOS selected level."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VOSRDY_A::B_0x1
    }
}
#[doc = "Voltage level ready for currently used VOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTVOSRDY_A {
    #[doc = "0: V CORE is above or below the current voltage scaling provided by ACTVOS\\[1:0\\]."]
    B_0x0 = 0,
    #[doc = "1: V CORE is equal to the current voltage scaling provided by ACTVOS\\[1:0\\]"]
    B_0x1 = 1,
}
impl From<ACTVOSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ACTVOSRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTVOSRDY` reader - Voltage level ready for currently used VOS"]
pub type ACTVOSRDY_R = crate::BitReader<ACTVOSRDY_A>;
impl ACTVOSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACTVOSRDY_A {
        match self.bits {
            false => ACTVOSRDY_A::B_0x0,
            true => ACTVOSRDY_A::B_0x1,
        }
    }
    #[doc = "V CORE is above or below the current voltage scaling provided by ACTVOS\\[1:0\\]."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ACTVOSRDY_A::B_0x0
    }
    #[doc = "V CORE is equal to the current voltage scaling provided by ACTVOS\\[1:0\\]"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ACTVOSRDY_A::B_0x1
    }
}
#[doc = "voltage output scaling currently applied to V CORE This field provides the last VOS value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTVOS_A {
    #[doc = "0: VOS3 (lowest power)"]
    B_0x0 = 0,
    #[doc = "1: VOS2"]
    B_0x1 = 1,
    #[doc = "2: VOS1"]
    B_0x2 = 2,
    #[doc = "3: VOS0 (highest frequency)"]
    B_0x3 = 3,
}
impl From<ACTVOS_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTVOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTVOS_A {
    type Ux = u8;
}
impl crate::IsEnum for ACTVOS_A {}
#[doc = "Field `ACTVOS` reader - voltage output scaling currently applied to V CORE This field provides the last VOS value."]
pub type ACTVOS_R = crate::FieldReader<ACTVOS_A>;
impl ACTVOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACTVOS_A {
        match self.bits {
            0 => ACTVOS_A::B_0x0,
            1 => ACTVOS_A::B_0x1,
            2 => ACTVOS_A::B_0x2,
            3 => ACTVOS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "VOS3 (lowest power)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ACTVOS_A::B_0x0
    }
    #[doc = "VOS2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ACTVOS_A::B_0x1
    }
    #[doc = "VOS1"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ACTVOS_A::B_0x2
    }
    #[doc = "VOS0 (highest frequency)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ACTVOS_A::B_0x3
    }
}
impl R {
    #[doc = "Bit 3 - Ready bit for V CORE voltage scaling output selection."]
    #[inline(always)]
    pub fn VOSRDY(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - Voltage level ready for currently used VOS"]
    #[inline(always)]
    pub fn ACTVOSRDY(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - voltage output scaling currently applied to V CORE This field provides the last VOS value."]
    #[inline(always)]
    pub fn ACTVOS(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "PWR voltage scaling status register\n\nYou can [`read`](crate::Reg::read) this register and get [`vossr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VOSSR_SPEC;
impl crate::RegisterSpec for VOSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vossr::R`](R) reader structure"]
impl crate::Readable for VOSSR_SPEC {}
#[doc = "`reset()` method sets VOSSR to value 0x08"]
impl crate::Resettable for VOSSR_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
