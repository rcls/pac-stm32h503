#[doc = "Register `EMR1` reader"]
pub type R = crate::R<EMR1_SPEC>;
#[doc = "Register `EMR1` writer"]
pub type W = crate::W<EMR1_SPEC>;
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM0_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM0_A> for bool {
    #[inline(always)]
    fn from(variant: EM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM0` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM0_R = crate::BitReader<EM0_A>;
impl EM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM0_A {
        match self.bits {
            false => EM0_A::B_0x0,
            true => EM0_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM0_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM0_A::B_0x1
    }
}
#[doc = "Field `EM0` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM0_W<'a, REG> = crate::BitWriter<'a, REG, EM0_A>;
impl<'a, REG> EM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM0_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM0_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM1_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM1_A> for bool {
    #[inline(always)]
    fn from(variant: EM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM1` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM1_R = crate::BitReader<EM1_A>;
impl EM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM1_A {
        match self.bits {
            false => EM1_A::B_0x0,
            true => EM1_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM1_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM1_A::B_0x1
    }
}
#[doc = "Field `EM1` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM1_W<'a, REG> = crate::BitWriter<'a, REG, EM1_A>;
impl<'a, REG> EM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM1_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM1_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM2_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM2_A> for bool {
    #[inline(always)]
    fn from(variant: EM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM2` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM2_R = crate::BitReader<EM2_A>;
impl EM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM2_A {
        match self.bits {
            false => EM2_A::B_0x0,
            true => EM2_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM2_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM2_A::B_0x1
    }
}
#[doc = "Field `EM2` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM2_W<'a, REG> = crate::BitWriter<'a, REG, EM2_A>;
impl<'a, REG> EM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM2_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM2_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM3_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM3_A> for bool {
    #[inline(always)]
    fn from(variant: EM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM3` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM3_R = crate::BitReader<EM3_A>;
impl EM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM3_A {
        match self.bits {
            false => EM3_A::B_0x0,
            true => EM3_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM3_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM3_A::B_0x1
    }
}
#[doc = "Field `EM3` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM3_W<'a, REG> = crate::BitWriter<'a, REG, EM3_A>;
impl<'a, REG> EM3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM3_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM3_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM4_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM4_A> for bool {
    #[inline(always)]
    fn from(variant: EM4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM4` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM4_R = crate::BitReader<EM4_A>;
impl EM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM4_A {
        match self.bits {
            false => EM4_A::B_0x0,
            true => EM4_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM4_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM4_A::B_0x1
    }
}
#[doc = "Field `EM4` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM4_W<'a, REG> = crate::BitWriter<'a, REG, EM4_A>;
impl<'a, REG> EM4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM4_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM4_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM5_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM5_A> for bool {
    #[inline(always)]
    fn from(variant: EM5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM5` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM5_R = crate::BitReader<EM5_A>;
impl EM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM5_A {
        match self.bits {
            false => EM5_A::B_0x0,
            true => EM5_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM5_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM5_A::B_0x1
    }
}
#[doc = "Field `EM5` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM5_W<'a, REG> = crate::BitWriter<'a, REG, EM5_A>;
impl<'a, REG> EM5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM5_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM5_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM6_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM6_A> for bool {
    #[inline(always)]
    fn from(variant: EM6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM6` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM6_R = crate::BitReader<EM6_A>;
impl EM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM6_A {
        match self.bits {
            false => EM6_A::B_0x0,
            true => EM6_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM6_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM6_A::B_0x1
    }
}
#[doc = "Field `EM6` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM6_W<'a, REG> = crate::BitWriter<'a, REG, EM6_A>;
impl<'a, REG> EM6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM6_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM6_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM7_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM7_A> for bool {
    #[inline(always)]
    fn from(variant: EM7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM7` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM7_R = crate::BitReader<EM7_A>;
impl EM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM7_A {
        match self.bits {
            false => EM7_A::B_0x0,
            true => EM7_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM7_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM7_A::B_0x1
    }
}
#[doc = "Field `EM7` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM7_W<'a, REG> = crate::BitWriter<'a, REG, EM7_A>;
impl<'a, REG> EM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM7_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM7_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM8_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM8_A> for bool {
    #[inline(always)]
    fn from(variant: EM8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM8` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM8_R = crate::BitReader<EM8_A>;
impl EM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM8_A {
        match self.bits {
            false => EM8_A::B_0x0,
            true => EM8_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM8_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM8_A::B_0x1
    }
}
#[doc = "Field `EM8` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM8_W<'a, REG> = crate::BitWriter<'a, REG, EM8_A>;
impl<'a, REG> EM8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM8_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM8_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM9_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM9_A> for bool {
    #[inline(always)]
    fn from(variant: EM9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM9` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM9_R = crate::BitReader<EM9_A>;
impl EM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM9_A {
        match self.bits {
            false => EM9_A::B_0x0,
            true => EM9_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM9_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM9_A::B_0x1
    }
}
#[doc = "Field `EM9` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM9_W<'a, REG> = crate::BitWriter<'a, REG, EM9_A>;
impl<'a, REG> EM9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM9_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM9_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM10_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM10_A> for bool {
    #[inline(always)]
    fn from(variant: EM10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM10` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM10_R = crate::BitReader<EM10_A>;
impl EM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM10_A {
        match self.bits {
            false => EM10_A::B_0x0,
            true => EM10_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM10_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM10_A::B_0x1
    }
}
#[doc = "Field `EM10` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM10_W<'a, REG> = crate::BitWriter<'a, REG, EM10_A>;
impl<'a, REG> EM10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM10_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM10_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM11_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM11_A> for bool {
    #[inline(always)]
    fn from(variant: EM11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM11` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM11_R = crate::BitReader<EM11_A>;
impl EM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM11_A {
        match self.bits {
            false => EM11_A::B_0x0,
            true => EM11_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM11_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM11_A::B_0x1
    }
}
#[doc = "Field `EM11` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM11_W<'a, REG> = crate::BitWriter<'a, REG, EM11_A>;
impl<'a, REG> EM11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM11_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM11_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM12_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM12_A> for bool {
    #[inline(always)]
    fn from(variant: EM12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM12` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM12_R = crate::BitReader<EM12_A>;
impl EM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM12_A {
        match self.bits {
            false => EM12_A::B_0x0,
            true => EM12_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM12_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM12_A::B_0x1
    }
}
#[doc = "Field `EM12` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM12_W<'a, REG> = crate::BitWriter<'a, REG, EM12_A>;
impl<'a, REG> EM12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM12_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM12_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM13_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM13_A> for bool {
    #[inline(always)]
    fn from(variant: EM13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM13` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM13_R = crate::BitReader<EM13_A>;
impl EM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM13_A {
        match self.bits {
            false => EM13_A::B_0x0,
            true => EM13_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM13_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM13_A::B_0x1
    }
}
#[doc = "Field `EM13` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM13_W<'a, REG> = crate::BitWriter<'a, REG, EM13_A>;
impl<'a, REG> EM13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM13_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM13_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM14_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM14_A> for bool {
    #[inline(always)]
    fn from(variant: EM14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM14` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM14_R = crate::BitReader<EM14_A>;
impl EM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM14_A {
        match self.bits {
            false => EM14_A::B_0x0,
            true => EM14_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM14_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM14_A::B_0x1
    }
}
#[doc = "Field `EM14` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM14_W<'a, REG> = crate::BitWriter<'a, REG, EM14_A>;
impl<'a, REG> EM14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM14_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM14_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM15_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM15_A> for bool {
    #[inline(always)]
    fn from(variant: EM15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM15` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM15_R = crate::BitReader<EM15_A>;
impl EM15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM15_A {
        match self.bits {
            false => EM15_A::B_0x0,
            true => EM15_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM15_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM15_A::B_0x1
    }
}
#[doc = "Field `EM15` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM15_W<'a, REG> = crate::BitWriter<'a, REG, EM15_A>;
impl<'a, REG> EM15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM15_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM15_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM16_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM16_A> for bool {
    #[inline(always)]
    fn from(variant: EM16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM16` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM16_R = crate::BitReader<EM16_A>;
impl EM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM16_A {
        match self.bits {
            false => EM16_A::B_0x0,
            true => EM16_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM16_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM16_A::B_0x1
    }
}
#[doc = "Field `EM16` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM16_W<'a, REG> = crate::BitWriter<'a, REG, EM16_A>;
impl<'a, REG> EM16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM16_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM16_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM17_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM17_A> for bool {
    #[inline(always)]
    fn from(variant: EM17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM17` reader - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM17_R = crate::BitReader<EM17_A>;
impl EM17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM17_A {
        match self.bits {
            false => EM17_A::B_0x0,
            true => EM17_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM17_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM17_A::B_0x1
    }
}
#[doc = "Field `EM17` writer - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM17_W<'a, REG> = crate::BitWriter<'a, REG, EM17_A>;
impl<'a, REG> EM17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM17_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM17_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM19_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM19_A> for bool {
    #[inline(always)]
    fn from(variant: EM19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM19` reader - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM19_R = crate::BitReader<EM19_A>;
impl EM19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM19_A {
        match self.bits {
            false => EM19_A::B_0x0,
            true => EM19_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM19_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM19_A::B_0x1
    }
}
#[doc = "Field `EM19` writer - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM19_W<'a, REG> = crate::BitWriter<'a, REG, EM19_A>;
impl<'a, REG> EM19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM19_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM19_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM21_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM21_A> for bool {
    #[inline(always)]
    fn from(variant: EM21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM21` reader - CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM21_R = crate::BitReader<EM21_A>;
impl EM21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM21_A {
        match self.bits {
            false => EM21_A::B_0x0,
            true => EM21_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM21_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM21_A::B_0x1
    }
}
#[doc = "Field `EM21` writer - CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM21_W<'a, REG> = crate::BitWriter<'a, REG, EM21_A>;
impl<'a, REG> EM21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM21_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM21_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM22_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM22_A> for bool {
    #[inline(always)]
    fn from(variant: EM22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM22` reader - CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM22_R = crate::BitReader<EM22_A>;
impl EM22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM22_A {
        match self.bits {
            false => EM22_A::B_0x0,
            true => EM22_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM22_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM22_A::B_0x1
    }
}
#[doc = "Field `EM22` writer - CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM22_W<'a, REG> = crate::BitWriter<'a, REG, EM22_A>;
impl<'a, REG> EM22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM22_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM22_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM24_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM24_A> for bool {
    #[inline(always)]
    fn from(variant: EM24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM24` reader - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM24_R = crate::BitReader<EM24_A>;
impl EM24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM24_A {
        match self.bits {
            false => EM24_A::B_0x0,
            true => EM24_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM24_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM24_A::B_0x1
    }
}
#[doc = "Field `EM24` writer - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM24_W<'a, REG> = crate::BitWriter<'a, REG, EM24_A>;
impl<'a, REG> EM24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM24_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM24_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM25_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM25_A> for bool {
    #[inline(always)]
    fn from(variant: EM25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM25` reader - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM25_R = crate::BitReader<EM25_A>;
impl EM25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM25_A {
        match self.bits {
            false => EM25_A::B_0x0,
            true => EM25_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM25_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM25_A::B_0x1
    }
}
#[doc = "Field `EM25` writer - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM25_W<'a, REG> = crate::BitWriter<'a, REG, EM25_A>;
impl<'a, REG> EM25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM25_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM25_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM26_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM26_A> for bool {
    #[inline(always)]
    fn from(variant: EM26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM26` reader - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM26_R = crate::BitReader<EM26_A>;
impl EM26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM26_A {
        match self.bits {
            false => EM26_A::B_0x0,
            true => EM26_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM26_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM26_A::B_0x1
    }
}
#[doc = "Field `EM26` writer - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM26_W<'a, REG> = crate::BitWriter<'a, REG, EM26_A>;
impl<'a, REG> EM26_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM26_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM26_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM27_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM27_A> for bool {
    #[inline(always)]
    fn from(variant: EM27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM27` reader - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM27_R = crate::BitReader<EM27_A>;
impl EM27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM27_A {
        match self.bits {
            false => EM27_A::B_0x0,
            true => EM27_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM27_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM27_A::B_0x1
    }
}
#[doc = "Field `EM27` writer - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM27_W<'a, REG> = crate::BitWriter<'a, REG, EM27_A>;
impl<'a, REG> EM27_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM27_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM27_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM28_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM28_A> for bool {
    #[inline(always)]
    fn from(variant: EM28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM28` reader - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM28_R = crate::BitReader<EM28_A>;
impl EM28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM28_A {
        match self.bits {
            false => EM28_A::B_0x0,
            true => EM28_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM28_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM28_A::B_0x1
    }
}
#[doc = "Field `EM28` writer - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM28_W<'a, REG> = crate::BitWriter<'a, REG, EM28_A>;
impl<'a, REG> EM28_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM28_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM28_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM29_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM29_A> for bool {
    #[inline(always)]
    fn from(variant: EM29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM29` reader - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM29_R = crate::BitReader<EM29_A>;
impl EM29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM29_A {
        match self.bits {
            false => EM29_A::B_0x0,
            true => EM29_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM29_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM29_A::B_0x1
    }
}
#[doc = "Field `EM29` writer - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM29_W<'a, REG> = crate::BitWriter<'a, REG, EM29_A>;
impl<'a, REG> EM29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM29_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM29_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM22(&self) -> EM22_R {
        EM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM24(&self) -> EM24_R {
        EM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM25(&self) -> EM25_R {
        EM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM26(&self) -> EM26_R {
        EM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM27(&self) -> EM27_R {
        EM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM28(&self) -> EM28_R {
        EM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM29(&self) -> EM29_R {
        EM29_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM0(&mut self) -> EM0_W<'_, EMR1_SPEC> {
        EM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM1(&mut self) -> EM1_W<'_, EMR1_SPEC> {
        EM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM2(&mut self) -> EM2_W<'_, EMR1_SPEC> {
        EM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM3(&mut self) -> EM3_W<'_, EMR1_SPEC> {
        EM3_W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM4(&mut self) -> EM4_W<'_, EMR1_SPEC> {
        EM4_W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM5(&mut self) -> EM5_W<'_, EMR1_SPEC> {
        EM5_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM6(&mut self) -> EM6_W<'_, EMR1_SPEC> {
        EM6_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM7(&mut self) -> EM7_W<'_, EMR1_SPEC> {
        EM7_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM8(&mut self) -> EM8_W<'_, EMR1_SPEC> {
        EM8_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM9(&mut self) -> EM9_W<'_, EMR1_SPEC> {
        EM9_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM10(&mut self) -> EM10_W<'_, EMR1_SPEC> {
        EM10_W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM11(&mut self) -> EM11_W<'_, EMR1_SPEC> {
        EM11_W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM12(&mut self) -> EM12_W<'_, EMR1_SPEC> {
        EM12_W::new(self, 12)
    }
    #[doc = "Bit 13 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM13(&mut self) -> EM13_W<'_, EMR1_SPEC> {
        EM13_W::new(self, 13)
    }
    #[doc = "Bit 14 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM14(&mut self) -> EM14_W<'_, EMR1_SPEC> {
        EM14_W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM15(&mut self) -> EM15_W<'_, EMR1_SPEC> {
        EM15_W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM16(&mut self) -> EM16_W<'_, EMR1_SPEC> {
        EM16_W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM17(&mut self) -> EM17_W<'_, EMR1_SPEC> {
        EM17_W::new(self, 17)
    }
    #[doc = "Bit 19 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM19(&mut self) -> EM19_W<'_, EMR1_SPEC> {
        EM19_W::new(self, 19)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM21(&mut self) -> EM21_W<'_, EMR1_SPEC> {
        EM21_W::new(self, 21)
    }
    #[doc = "Bit 22 - CPU wakeup with event generation mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM22(&mut self) -> EM22_W<'_, EMR1_SPEC> {
        EM22_W::new(self, 22)
    }
    #[doc = "Bit 24 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM24(&mut self) -> EM24_W<'_, EMR1_SPEC> {
        EM24_W::new(self, 24)
    }
    #[doc = "Bit 25 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM25(&mut self) -> EM25_W<'_, EMR1_SPEC> {
        EM25_W::new(self, 25)
    }
    #[doc = "Bit 26 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM26(&mut self) -> EM26_W<'_, EMR1_SPEC> {
        EM26_W::new(self, 26)
    }
    #[doc = "Bit 27 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM27(&mut self) -> EM27_W<'_, EMR1_SPEC> {
        EM27_W::new(self, 27)
    }
    #[doc = "Bit 28 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM28(&mut self) -> EM28_W<'_, EMR1_SPEC> {
        EM28_W::new(self, 28)
    }
    #[doc = "Bit 29 - CPU wakeup with event generation mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM29(&mut self) -> EM29_W<'_, EMR1_SPEC> {
        EM29_W::new(self, 29)
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR1_SPEC;
impl crate::RegisterSpec for EMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr1::R`](R) reader structure"]
impl crate::Readable for EMR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emr1::W`](W) writer structure"]
impl crate::Writable for EMR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EMR1 to value 0xfffe_0000"]
impl crate::Resettable for EMR1_SPEC {
    const RESET_VALUE: u32 = 0xfffe_0000;
}
