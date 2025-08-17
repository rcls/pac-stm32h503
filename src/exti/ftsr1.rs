#[doc = "Register `FTSR1` reader"]
pub type R = crate::R<FTSR1_SPEC>;
#[doc = "Register `FTSR1` writer"]
pub type W = crate::W<FTSR1_SPEC>;
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT0_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT0_A> for bool {
    #[inline(always)]
    fn from(variant: FT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT0` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT0_R = crate::BitReader<FT0_A>;
impl FT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT0_A {
        match self.bits {
            false => FT0_A::B_0x0,
            true => FT0_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT0_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT0_A::B_0x1
    }
}
#[doc = "Field `FT0` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT0_W<'a, REG> = crate::BitWriter<'a, REG, FT0_A>;
impl<'a, REG> FT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT0_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT0_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT1_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT1_A> for bool {
    #[inline(always)]
    fn from(variant: FT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT1` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT1_R = crate::BitReader<FT1_A>;
impl FT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT1_A {
        match self.bits {
            false => FT1_A::B_0x0,
            true => FT1_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT1_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT1_A::B_0x1
    }
}
#[doc = "Field `FT1` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT1_W<'a, REG> = crate::BitWriter<'a, REG, FT1_A>;
impl<'a, REG> FT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT1_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT1_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT2_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT2_A> for bool {
    #[inline(always)]
    fn from(variant: FT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT2` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT2_R = crate::BitReader<FT2_A>;
impl FT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT2_A {
        match self.bits {
            false => FT2_A::B_0x0,
            true => FT2_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT2_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT2_A::B_0x1
    }
}
#[doc = "Field `FT2` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT2_W<'a, REG> = crate::BitWriter<'a, REG, FT2_A>;
impl<'a, REG> FT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT2_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT2_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT3_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT3_A> for bool {
    #[inline(always)]
    fn from(variant: FT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT3` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT3_R = crate::BitReader<FT3_A>;
impl FT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT3_A {
        match self.bits {
            false => FT3_A::B_0x0,
            true => FT3_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT3_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT3_A::B_0x1
    }
}
#[doc = "Field `FT3` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT3_W<'a, REG> = crate::BitWriter<'a, REG, FT3_A>;
impl<'a, REG> FT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT3_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT3_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT4_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT4_A> for bool {
    #[inline(always)]
    fn from(variant: FT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT4` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT4_R = crate::BitReader<FT4_A>;
impl FT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT4_A {
        match self.bits {
            false => FT4_A::B_0x0,
            true => FT4_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT4_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT4_A::B_0x1
    }
}
#[doc = "Field `FT4` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT4_W<'a, REG> = crate::BitWriter<'a, REG, FT4_A>;
impl<'a, REG> FT4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT4_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT4_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT5_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT5_A> for bool {
    #[inline(always)]
    fn from(variant: FT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT5` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT5_R = crate::BitReader<FT5_A>;
impl FT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT5_A {
        match self.bits {
            false => FT5_A::B_0x0,
            true => FT5_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT5_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT5_A::B_0x1
    }
}
#[doc = "Field `FT5` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT5_W<'a, REG> = crate::BitWriter<'a, REG, FT5_A>;
impl<'a, REG> FT5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT5_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT5_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT6_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT6_A> for bool {
    #[inline(always)]
    fn from(variant: FT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT6` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT6_R = crate::BitReader<FT6_A>;
impl FT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT6_A {
        match self.bits {
            false => FT6_A::B_0x0,
            true => FT6_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT6_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT6_A::B_0x1
    }
}
#[doc = "Field `FT6` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT6_W<'a, REG> = crate::BitWriter<'a, REG, FT6_A>;
impl<'a, REG> FT6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT6_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT6_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT7_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT7_A> for bool {
    #[inline(always)]
    fn from(variant: FT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT7` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT7_R = crate::BitReader<FT7_A>;
impl FT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT7_A {
        match self.bits {
            false => FT7_A::B_0x0,
            true => FT7_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT7_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT7_A::B_0x1
    }
}
#[doc = "Field `FT7` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT7_W<'a, REG> = crate::BitWriter<'a, REG, FT7_A>;
impl<'a, REG> FT7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT7_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT7_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT8_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT8_A> for bool {
    #[inline(always)]
    fn from(variant: FT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT8` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT8_R = crate::BitReader<FT8_A>;
impl FT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT8_A {
        match self.bits {
            false => FT8_A::B_0x0,
            true => FT8_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT8_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT8_A::B_0x1
    }
}
#[doc = "Field `FT8` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT8_W<'a, REG> = crate::BitWriter<'a, REG, FT8_A>;
impl<'a, REG> FT8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT8_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT8_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT9_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT9_A> for bool {
    #[inline(always)]
    fn from(variant: FT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT9` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT9_R = crate::BitReader<FT9_A>;
impl FT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT9_A {
        match self.bits {
            false => FT9_A::B_0x0,
            true => FT9_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT9_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT9_A::B_0x1
    }
}
#[doc = "Field `FT9` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT9_W<'a, REG> = crate::BitWriter<'a, REG, FT9_A>;
impl<'a, REG> FT9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT9_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT9_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT10_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT10_A> for bool {
    #[inline(always)]
    fn from(variant: FT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT10` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT10_R = crate::BitReader<FT10_A>;
impl FT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT10_A {
        match self.bits {
            false => FT10_A::B_0x0,
            true => FT10_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT10_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT10_A::B_0x1
    }
}
#[doc = "Field `FT10` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT10_W<'a, REG> = crate::BitWriter<'a, REG, FT10_A>;
impl<'a, REG> FT10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT10_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT10_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT11_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT11_A> for bool {
    #[inline(always)]
    fn from(variant: FT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT11` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT11_R = crate::BitReader<FT11_A>;
impl FT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT11_A {
        match self.bits {
            false => FT11_A::B_0x0,
            true => FT11_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT11_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT11_A::B_0x1
    }
}
#[doc = "Field `FT11` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT11_W<'a, REG> = crate::BitWriter<'a, REG, FT11_A>;
impl<'a, REG> FT11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT11_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT11_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT12_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT12_A> for bool {
    #[inline(always)]
    fn from(variant: FT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT12` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT12_R = crate::BitReader<FT12_A>;
impl FT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT12_A {
        match self.bits {
            false => FT12_A::B_0x0,
            true => FT12_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT12_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT12_A::B_0x1
    }
}
#[doc = "Field `FT12` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT12_W<'a, REG> = crate::BitWriter<'a, REG, FT12_A>;
impl<'a, REG> FT12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT12_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT12_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT13_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT13_A> for bool {
    #[inline(always)]
    fn from(variant: FT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT13` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT13_R = crate::BitReader<FT13_A>;
impl FT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT13_A {
        match self.bits {
            false => FT13_A::B_0x0,
            true => FT13_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT13_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT13_A::B_0x1
    }
}
#[doc = "Field `FT13` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT13_W<'a, REG> = crate::BitWriter<'a, REG, FT13_A>;
impl<'a, REG> FT13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT13_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT13_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT14_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT14_A> for bool {
    #[inline(always)]
    fn from(variant: FT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT14` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT14_R = crate::BitReader<FT14_A>;
impl FT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT14_A {
        match self.bits {
            false => FT14_A::B_0x0,
            true => FT14_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT14_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT14_A::B_0x1
    }
}
#[doc = "Field `FT14` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT14_W<'a, REG> = crate::BitWriter<'a, REG, FT14_A>;
impl<'a, REG> FT14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT14_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT14_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT15_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT15_A> for bool {
    #[inline(always)]
    fn from(variant: FT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT15` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT15_R = crate::BitReader<FT15_A>;
impl FT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT15_A {
        match self.bits {
            false => FT15_A::B_0x0,
            true => FT15_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT15_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT15_A::B_0x1
    }
}
#[doc = "Field `FT15` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT15_W<'a, REG> = crate::BitWriter<'a, REG, FT15_A>;
impl<'a, REG> FT15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT15_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT15_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT16_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT16_A> for bool {
    #[inline(always)]
    fn from(variant: FT16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT16` reader - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT16_R = crate::BitReader<FT16_A>;
impl FT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT16_A {
        match self.bits {
            false => FT16_A::B_0x0,
            true => FT16_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT16_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT16_A::B_0x1
    }
}
#[doc = "Field `FT16` writer - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT16_W<'a, REG> = crate::BitWriter<'a, REG, FT16_A>;
impl<'a, REG> FT16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT16_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT16_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT0(&self) -> FT0_R {
        FT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT0(&mut self) -> FT0_W<'_, FTSR1_SPEC> {
        FT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT1(&mut self) -> FT1_W<'_, FTSR1_SPEC> {
        FT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT2(&mut self) -> FT2_W<'_, FTSR1_SPEC> {
        FT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT3(&mut self) -> FT3_W<'_, FTSR1_SPEC> {
        FT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT4(&mut self) -> FT4_W<'_, FTSR1_SPEC> {
        FT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT5(&mut self) -> FT5_W<'_, FTSR1_SPEC> {
        FT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT6(&mut self) -> FT6_W<'_, FTSR1_SPEC> {
        FT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT7(&mut self) -> FT7_W<'_, FTSR1_SPEC> {
        FT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT8(&mut self) -> FT8_W<'_, FTSR1_SPEC> {
        FT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT9(&mut self) -> FT9_W<'_, FTSR1_SPEC> {
        FT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT10(&mut self) -> FT10_W<'_, FTSR1_SPEC> {
        FT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT11(&mut self) -> FT11_W<'_, FTSR1_SPEC> {
        FT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT12(&mut self) -> FT12_W<'_, FTSR1_SPEC> {
        FT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT13(&mut self) -> FT13_W<'_, FTSR1_SPEC> {
        FT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT14(&mut self) -> FT14_W<'_, FTSR1_SPEC> {
        FT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT15(&mut self) -> FT15_W<'_, FTSR1_SPEC> {
        FT15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Falling trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT16(&mut self) -> FT16_W<'_, FTSR1_SPEC> {
        FT16_W::new(self, 16)
    }
}
#[doc = "EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSR1_SPEC;
impl crate::RegisterSpec for FTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr1::R`](R) reader structure"]
impl crate::Readable for FTSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ftsr1::W`](W) writer structure"]
impl crate::Writable for FTSR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FTSR1 to value 0"]
impl crate::Resettable for FTSR1_SPEC {}
