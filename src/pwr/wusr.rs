#[doc = "Register `WUSR` reader"]
pub type R = crate::R<WUSR_SPEC>;
#[doc = "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1_A {
    #[doc = "0: no wakeup event occurred."]
    B_0x0 = 0,
    #[doc = "1: a wakeup event received from WUFx pin."]
    B_0x1 = 1,
}
impl From<WUF1_A> for bool {
    #[inline(always)]
    fn from(variant: WUF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF1` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF1_R = crate::BitReader<WUF1_A>;
impl WUF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF1_A {
        match self.bits {
            false => WUF1_A::B_0x0,
            true => WUF1_A::B_0x1,
        }
    }
    #[doc = "no wakeup event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUF1_A::B_0x0
    }
    #[doc = "a wakeup event received from WUFx pin."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUF1_A::B_0x1
    }
}
#[doc = "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF2_A {
    #[doc = "0: no wakeup event occurred."]
    B_0x0 = 0,
    #[doc = "1: a wakeup event received from WUFx pin."]
    B_0x1 = 1,
}
impl From<WUF2_A> for bool {
    #[inline(always)]
    fn from(variant: WUF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF2` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF2_R = crate::BitReader<WUF2_A>;
impl WUF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF2_A {
        match self.bits {
            false => WUF2_A::B_0x0,
            true => WUF2_A::B_0x1,
        }
    }
    #[doc = "no wakeup event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUF2_A::B_0x0
    }
    #[doc = "a wakeup event received from WUFx pin."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUF2_A::B_0x1
    }
}
#[doc = "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF3_A {
    #[doc = "0: no wakeup event occurred."]
    B_0x0 = 0,
    #[doc = "1: a wakeup event received from WUFx pin."]
    B_0x1 = 1,
}
impl From<WUF3_A> for bool {
    #[inline(always)]
    fn from(variant: WUF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF3` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF3_R = crate::BitReader<WUF3_A>;
impl WUF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF3_A {
        match self.bits {
            false => WUF3_A::B_0x0,
            true => WUF3_A::B_0x1,
        }
    }
    #[doc = "no wakeup event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUF3_A::B_0x0
    }
    #[doc = "a wakeup event received from WUFx pin."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUF3_A::B_0x1
    }
}
#[doc = "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF4_A {
    #[doc = "0: no wakeup event occurred."]
    B_0x0 = 0,
    #[doc = "1: a wakeup event received from WUFx pin."]
    B_0x1 = 1,
}
impl From<WUF4_A> for bool {
    #[inline(always)]
    fn from(variant: WUF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF4` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF4_R = crate::BitReader<WUF4_A>;
impl WUF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF4_A {
        match self.bits {
            false => WUF4_A::B_0x0,
            true => WUF4_A::B_0x1,
        }
    }
    #[doc = "no wakeup event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUF4_A::B_0x0
    }
    #[doc = "a wakeup event received from WUFx pin."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUF4_A::B_0x1
    }
}
#[doc = "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF5_A {
    #[doc = "0: no wakeup event occurred."]
    B_0x0 = 0,
    #[doc = "1: a wakeup event received from WUFx pin."]
    B_0x1 = 1,
}
impl From<WUF5_A> for bool {
    #[inline(always)]
    fn from(variant: WUF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF5` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF5_R = crate::BitReader<WUF5_A>;
impl WUF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF5_A {
        match self.bits {
            false => WUF5_A::B_0x0,
            true => WUF5_A::B_0x1,
        }
    }
    #[doc = "no wakeup event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WUF5_A::B_0x0
    }
    #[doc = "a wakeup event received from WUFx pin."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WUF5_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn WUF1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn WUF2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn WUF3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn WUF4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn WUF5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "PWR wakeup status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wusr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUSR_SPEC;
impl crate::RegisterSpec for WUSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wusr::R`](R) reader structure"]
impl crate::Readable for WUSR_SPEC {}
#[doc = "`reset()` method sets WUSR to value 0"]
impl crate::Resettable for WUSR_SPEC {}
