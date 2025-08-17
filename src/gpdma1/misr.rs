#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISR_SPEC>;
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS0_A {
    #[doc = "0: no interrupt occurred on channel x"]
    B_0x0 = 0,
    #[doc = "1: an interrupt occurred on channel x"]
    B_0x1 = 1,
}
impl From<MIS0_A> for bool {
    #[inline(always)]
    fn from(variant: MIS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS0` reader - masked interrupt status of channel x"]
pub type MIS0_R = crate::BitReader<MIS0_A>;
impl MIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS0_A {
        match self.bits {
            false => MIS0_A::B_0x0,
            true => MIS0_A::B_0x1,
        }
    }
    #[doc = "no interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIS0_A::B_0x0
    }
    #[doc = "an interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIS0_A::B_0x1
    }
}
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS1_A {
    #[doc = "0: no interrupt occurred on channel x"]
    B_0x0 = 0,
    #[doc = "1: an interrupt occurred on channel x"]
    B_0x1 = 1,
}
impl From<MIS1_A> for bool {
    #[inline(always)]
    fn from(variant: MIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS1` reader - masked interrupt status of channel x"]
pub type MIS1_R = crate::BitReader<MIS1_A>;
impl MIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS1_A {
        match self.bits {
            false => MIS1_A::B_0x0,
            true => MIS1_A::B_0x1,
        }
    }
    #[doc = "no interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIS1_A::B_0x0
    }
    #[doc = "an interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIS1_A::B_0x1
    }
}
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS2_A {
    #[doc = "0: no interrupt occurred on channel x"]
    B_0x0 = 0,
    #[doc = "1: an interrupt occurred on channel x"]
    B_0x1 = 1,
}
impl From<MIS2_A> for bool {
    #[inline(always)]
    fn from(variant: MIS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS2` reader - masked interrupt status of channel x"]
pub type MIS2_R = crate::BitReader<MIS2_A>;
impl MIS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS2_A {
        match self.bits {
            false => MIS2_A::B_0x0,
            true => MIS2_A::B_0x1,
        }
    }
    #[doc = "no interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIS2_A::B_0x0
    }
    #[doc = "an interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIS2_A::B_0x1
    }
}
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS3_A {
    #[doc = "0: no interrupt occurred on channel x"]
    B_0x0 = 0,
    #[doc = "1: an interrupt occurred on channel x"]
    B_0x1 = 1,
}
impl From<MIS3_A> for bool {
    #[inline(always)]
    fn from(variant: MIS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS3` reader - masked interrupt status of channel x"]
pub type MIS3_R = crate::BitReader<MIS3_A>;
impl MIS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS3_A {
        match self.bits {
            false => MIS3_A::B_0x0,
            true => MIS3_A::B_0x1,
        }
    }
    #[doc = "no interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIS3_A::B_0x0
    }
    #[doc = "an interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIS3_A::B_0x1
    }
}
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS4_A {
    #[doc = "0: no interrupt occurred on channel x"]
    B_0x0 = 0,
    #[doc = "1: an interrupt occurred on channel x"]
    B_0x1 = 1,
}
impl From<MIS4_A> for bool {
    #[inline(always)]
    fn from(variant: MIS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS4` reader - masked interrupt status of channel x"]
pub type MIS4_R = crate::BitReader<MIS4_A>;
impl MIS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS4_A {
        match self.bits {
            false => MIS4_A::B_0x0,
            true => MIS4_A::B_0x1,
        }
    }
    #[doc = "no interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIS4_A::B_0x0
    }
    #[doc = "an interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIS4_A::B_0x1
    }
}
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS5_A {
    #[doc = "0: no interrupt occurred on channel x"]
    B_0x0 = 0,
    #[doc = "1: an interrupt occurred on channel x"]
    B_0x1 = 1,
}
impl From<MIS5_A> for bool {
    #[inline(always)]
    fn from(variant: MIS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS5` reader - masked interrupt status of channel x"]
pub type MIS5_R = crate::BitReader<MIS5_A>;
impl MIS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS5_A {
        match self.bits {
            false => MIS5_A::B_0x0,
            true => MIS5_A::B_0x1,
        }
    }
    #[doc = "no interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIS5_A::B_0x0
    }
    #[doc = "an interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIS5_A::B_0x1
    }
}
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS6_A {
    #[doc = "0: no interrupt occurred on channel x"]
    B_0x0 = 0,
    #[doc = "1: an interrupt occurred on channel x"]
    B_0x1 = 1,
}
impl From<MIS6_A> for bool {
    #[inline(always)]
    fn from(variant: MIS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS6` reader - masked interrupt status of channel x"]
pub type MIS6_R = crate::BitReader<MIS6_A>;
impl MIS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS6_A {
        match self.bits {
            false => MIS6_A::B_0x0,
            true => MIS6_A::B_0x1,
        }
    }
    #[doc = "no interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIS6_A::B_0x0
    }
    #[doc = "an interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIS6_A::B_0x1
    }
}
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS7_A {
    #[doc = "0: no interrupt occurred on channel x"]
    B_0x0 = 0,
    #[doc = "1: an interrupt occurred on channel x"]
    B_0x1 = 1,
}
impl From<MIS7_A> for bool {
    #[inline(always)]
    fn from(variant: MIS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS7` reader - masked interrupt status of channel x"]
pub type MIS7_R = crate::BitReader<MIS7_A>;
impl MIS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS7_A {
        match self.bits {
            false => MIS7_A::B_0x0,
            true => MIS7_A::B_0x1,
        }
    }
    #[doc = "no interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MIS7_A::B_0x0
    }
    #[doc = "an interrupt occurred on channel x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MIS7_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn MIS0(&self) -> MIS0_R {
        MIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn MIS1(&self) -> MIS1_R {
        MIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn MIS2(&self) -> MIS2_R {
        MIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn MIS3(&self) -> MIS3_R {
        MIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn MIS4(&self) -> MIS4_R {
        MIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn MIS5(&self) -> MIS5_R {
        MIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn MIS6(&self) -> MIS6_R {
        MIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn MIS7(&self) -> MIS7_R {
        MIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "GPDMA masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISR_SPEC {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {}
