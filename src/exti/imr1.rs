#[doc = "Register `IMR1` reader"]
pub type R = crate::R<IMR1_SPEC>;
#[doc = "Register `IMR1` writer"]
pub type W = crate::W<IMR1_SPEC>;
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM0_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM0_A> for bool {
    #[inline(always)]
    fn from(variant: IM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM0` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM0_R = crate::BitReader<IM0_A>;
impl IM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM0_A {
        match self.bits {
            false => IM0_A::B_0x0,
            true => IM0_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM0_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM0_A::B_0x1
    }
}
#[doc = "Field `IM0` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM0_W<'a, REG> = crate::BitWriter<'a, REG, IM0_A>;
impl<'a, REG> IM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM0_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM0_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM1_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM1_A> for bool {
    #[inline(always)]
    fn from(variant: IM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM1` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM1_R = crate::BitReader<IM1_A>;
impl IM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM1_A {
        match self.bits {
            false => IM1_A::B_0x0,
            true => IM1_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM1_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM1_A::B_0x1
    }
}
#[doc = "Field `IM1` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM1_W<'a, REG> = crate::BitWriter<'a, REG, IM1_A>;
impl<'a, REG> IM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM1_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM1_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM2_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM2_A> for bool {
    #[inline(always)]
    fn from(variant: IM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM2` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM2_R = crate::BitReader<IM2_A>;
impl IM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM2_A {
        match self.bits {
            false => IM2_A::B_0x0,
            true => IM2_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM2_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM2_A::B_0x1
    }
}
#[doc = "Field `IM2` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM2_W<'a, REG> = crate::BitWriter<'a, REG, IM2_A>;
impl<'a, REG> IM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM2_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM2_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM3_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM3_A> for bool {
    #[inline(always)]
    fn from(variant: IM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM3` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM3_R = crate::BitReader<IM3_A>;
impl IM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM3_A {
        match self.bits {
            false => IM3_A::B_0x0,
            true => IM3_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM3_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM3_A::B_0x1
    }
}
#[doc = "Field `IM3` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM3_W<'a, REG> = crate::BitWriter<'a, REG, IM3_A>;
impl<'a, REG> IM3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM3_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM3_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM4_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM4_A> for bool {
    #[inline(always)]
    fn from(variant: IM4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM4` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM4_R = crate::BitReader<IM4_A>;
impl IM4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM4_A {
        match self.bits {
            false => IM4_A::B_0x0,
            true => IM4_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM4_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM4_A::B_0x1
    }
}
#[doc = "Field `IM4` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM4_W<'a, REG> = crate::BitWriter<'a, REG, IM4_A>;
impl<'a, REG> IM4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM4_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM4_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM5_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM5_A> for bool {
    #[inline(always)]
    fn from(variant: IM5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM5` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM5_R = crate::BitReader<IM5_A>;
impl IM5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM5_A {
        match self.bits {
            false => IM5_A::B_0x0,
            true => IM5_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM5_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM5_A::B_0x1
    }
}
#[doc = "Field `IM5` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM5_W<'a, REG> = crate::BitWriter<'a, REG, IM5_A>;
impl<'a, REG> IM5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM5_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM5_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM6_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM6_A> for bool {
    #[inline(always)]
    fn from(variant: IM6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM6` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM6_R = crate::BitReader<IM6_A>;
impl IM6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM6_A {
        match self.bits {
            false => IM6_A::B_0x0,
            true => IM6_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM6_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM6_A::B_0x1
    }
}
#[doc = "Field `IM6` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM6_W<'a, REG> = crate::BitWriter<'a, REG, IM6_A>;
impl<'a, REG> IM6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM6_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM6_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM7_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM7_A> for bool {
    #[inline(always)]
    fn from(variant: IM7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM7` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM7_R = crate::BitReader<IM7_A>;
impl IM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM7_A {
        match self.bits {
            false => IM7_A::B_0x0,
            true => IM7_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM7_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM7_A::B_0x1
    }
}
#[doc = "Field `IM7` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM7_W<'a, REG> = crate::BitWriter<'a, REG, IM7_A>;
impl<'a, REG> IM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM7_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM7_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM8_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM8_A> for bool {
    #[inline(always)]
    fn from(variant: IM8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM8` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM8_R = crate::BitReader<IM8_A>;
impl IM8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM8_A {
        match self.bits {
            false => IM8_A::B_0x0,
            true => IM8_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM8_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM8_A::B_0x1
    }
}
#[doc = "Field `IM8` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM8_W<'a, REG> = crate::BitWriter<'a, REG, IM8_A>;
impl<'a, REG> IM8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM8_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM8_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM9_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM9_A> for bool {
    #[inline(always)]
    fn from(variant: IM9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM9` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM9_R = crate::BitReader<IM9_A>;
impl IM9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM9_A {
        match self.bits {
            false => IM9_A::B_0x0,
            true => IM9_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM9_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM9_A::B_0x1
    }
}
#[doc = "Field `IM9` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM9_W<'a, REG> = crate::BitWriter<'a, REG, IM9_A>;
impl<'a, REG> IM9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM9_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM9_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM10_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM10_A> for bool {
    #[inline(always)]
    fn from(variant: IM10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM10` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM10_R = crate::BitReader<IM10_A>;
impl IM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM10_A {
        match self.bits {
            false => IM10_A::B_0x0,
            true => IM10_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM10_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM10_A::B_0x1
    }
}
#[doc = "Field `IM10` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM10_W<'a, REG> = crate::BitWriter<'a, REG, IM10_A>;
impl<'a, REG> IM10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM10_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM10_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM11_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM11_A> for bool {
    #[inline(always)]
    fn from(variant: IM11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM11` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM11_R = crate::BitReader<IM11_A>;
impl IM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM11_A {
        match self.bits {
            false => IM11_A::B_0x0,
            true => IM11_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM11_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM11_A::B_0x1
    }
}
#[doc = "Field `IM11` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM11_W<'a, REG> = crate::BitWriter<'a, REG, IM11_A>;
impl<'a, REG> IM11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM11_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM11_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM12_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM12_A> for bool {
    #[inline(always)]
    fn from(variant: IM12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM12` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM12_R = crate::BitReader<IM12_A>;
impl IM12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM12_A {
        match self.bits {
            false => IM12_A::B_0x0,
            true => IM12_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM12_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM12_A::B_0x1
    }
}
#[doc = "Field `IM12` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM12_W<'a, REG> = crate::BitWriter<'a, REG, IM12_A>;
impl<'a, REG> IM12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM12_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM12_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM13_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM13_A> for bool {
    #[inline(always)]
    fn from(variant: IM13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM13` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM13_R = crate::BitReader<IM13_A>;
impl IM13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM13_A {
        match self.bits {
            false => IM13_A::B_0x0,
            true => IM13_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM13_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM13_A::B_0x1
    }
}
#[doc = "Field `IM13` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM13_W<'a, REG> = crate::BitWriter<'a, REG, IM13_A>;
impl<'a, REG> IM13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM13_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM13_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM14_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM14_A> for bool {
    #[inline(always)]
    fn from(variant: IM14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM14` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM14_R = crate::BitReader<IM14_A>;
impl IM14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM14_A {
        match self.bits {
            false => IM14_A::B_0x0,
            true => IM14_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM14_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM14_A::B_0x1
    }
}
#[doc = "Field `IM14` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM14_W<'a, REG> = crate::BitWriter<'a, REG, IM14_A>;
impl<'a, REG> IM14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM14_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM14_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM15_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM15_A> for bool {
    #[inline(always)]
    fn from(variant: IM15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM15` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM15_R = crate::BitReader<IM15_A>;
impl IM15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM15_A {
        match self.bits {
            false => IM15_A::B_0x0,
            true => IM15_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM15_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM15_A::B_0x1
    }
}
#[doc = "Field `IM15` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM15_W<'a, REG> = crate::BitWriter<'a, REG, IM15_A>;
impl<'a, REG> IM15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM15_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM15_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM16_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM16_A> for bool {
    #[inline(always)]
    fn from(variant: IM16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM16` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM16_R = crate::BitReader<IM16_A>;
impl IM16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM16_A {
        match self.bits {
            false => IM16_A::B_0x0,
            true => IM16_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM16_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM16_A::B_0x1
    }
}
#[doc = "Field `IM16` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM16_W<'a, REG> = crate::BitWriter<'a, REG, IM16_A>;
impl<'a, REG> IM16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM16_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM16_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM17_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM17_A> for bool {
    #[inline(always)]
    fn from(variant: IM17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM17` reader - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM17_R = crate::BitReader<IM17_A>;
impl IM17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM17_A {
        match self.bits {
            false => IM17_A::B_0x0,
            true => IM17_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM17_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM17_A::B_0x1
    }
}
#[doc = "Field `IM17` writer - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM17_W<'a, REG> = crate::BitWriter<'a, REG, IM17_A>;
impl<'a, REG> IM17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM17_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM17_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM19_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM19_A> for bool {
    #[inline(always)]
    fn from(variant: IM19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM19` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM19_R = crate::BitReader<IM19_A>;
impl IM19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM19_A {
        match self.bits {
            false => IM19_A::B_0x0,
            true => IM19_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM19_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM19_A::B_0x1
    }
}
#[doc = "Field `IM19` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM19_W<'a, REG> = crate::BitWriter<'a, REG, IM19_A>;
impl<'a, REG> IM19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM19_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM19_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM21_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM21_A> for bool {
    #[inline(always)]
    fn from(variant: IM21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM21` reader - CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM21_R = crate::BitReader<IM21_A>;
impl IM21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM21_A {
        match self.bits {
            false => IM21_A::B_0x0,
            true => IM21_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM21_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM21_A::B_0x1
    }
}
#[doc = "Field `IM21` writer - CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM21_W<'a, REG> = crate::BitWriter<'a, REG, IM21_A>;
impl<'a, REG> IM21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM21_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM21_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM22_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM22_A> for bool {
    #[inline(always)]
    fn from(variant: IM22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM22` reader - CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM22_R = crate::BitReader<IM22_A>;
impl IM22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM22_A {
        match self.bits {
            false => IM22_A::B_0x0,
            true => IM22_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM22_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM22_A::B_0x1
    }
}
#[doc = "Field `IM22` writer - CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM22_W<'a, REG> = crate::BitWriter<'a, REG, IM22_A>;
impl<'a, REG> IM22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM22_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM22_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM24_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM24_A> for bool {
    #[inline(always)]
    fn from(variant: IM24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM24` reader - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM24_R = crate::BitReader<IM24_A>;
impl IM24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM24_A {
        match self.bits {
            false => IM24_A::B_0x0,
            true => IM24_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM24_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM24_A::B_0x1
    }
}
#[doc = "Field `IM24` writer - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM24_W<'a, REG> = crate::BitWriter<'a, REG, IM24_A>;
impl<'a, REG> IM24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM24_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM24_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM25_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM25_A> for bool {
    #[inline(always)]
    fn from(variant: IM25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM25` reader - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM25_R = crate::BitReader<IM25_A>;
impl IM25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM25_A {
        match self.bits {
            false => IM25_A::B_0x0,
            true => IM25_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM25_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM25_A::B_0x1
    }
}
#[doc = "Field `IM25` writer - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM25_W<'a, REG> = crate::BitWriter<'a, REG, IM25_A>;
impl<'a, REG> IM25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM25_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM25_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM26_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM26_A> for bool {
    #[inline(always)]
    fn from(variant: IM26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM26` reader - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM26_R = crate::BitReader<IM26_A>;
impl IM26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM26_A {
        match self.bits {
            false => IM26_A::B_0x0,
            true => IM26_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM26_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM26_A::B_0x1
    }
}
#[doc = "Field `IM26` writer - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM26_W<'a, REG> = crate::BitWriter<'a, REG, IM26_A>;
impl<'a, REG> IM26_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM26_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM26_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM27_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM27_A> for bool {
    #[inline(always)]
    fn from(variant: IM27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM27` reader - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM27_R = crate::BitReader<IM27_A>;
impl IM27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM27_A {
        match self.bits {
            false => IM27_A::B_0x0,
            true => IM27_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM27_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM27_A::B_0x1
    }
}
#[doc = "Field `IM27` writer - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM27_W<'a, REG> = crate::BitWriter<'a, REG, IM27_A>;
impl<'a, REG> IM27_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM27_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM27_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM28_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM28_A> for bool {
    #[inline(always)]
    fn from(variant: IM28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM28` reader - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM28_R = crate::BitReader<IM28_A>;
impl IM28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM28_A {
        match self.bits {
            false => IM28_A::B_0x0,
            true => IM28_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM28_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM28_A::B_0x1
    }
}
#[doc = "Field `IM28` writer - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM28_W<'a, REG> = crate::BitWriter<'a, REG, IM28_A>;
impl<'a, REG> IM28_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM28_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM28_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM29_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM29_A> for bool {
    #[inline(always)]
    fn from(variant: IM29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM29` reader - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM29_R = crate::BitReader<IM29_A>;
impl IM29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM29_A {
        match self.bits {
            false => IM29_A::B_0x0,
            true => IM29_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM29_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM29_A::B_0x1
    }
}
#[doc = "Field `IM29` writer - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM29_W<'a, REG> = crate::BitWriter<'a, REG, IM29_A>;
impl<'a, REG> IM29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM29_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM29_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM24(&self) -> IM24_R {
        IM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM26(&self) -> IM26_R {
        IM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM27(&self) -> IM27_R {
        IM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM28(&self) -> IM28_R {
        IM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM29(&self) -> IM29_R {
        IM29_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM0(&mut self) -> IM0_W<'_, IMR1_SPEC> {
        IM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM1(&mut self) -> IM1_W<'_, IMR1_SPEC> {
        IM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM2(&mut self) -> IM2_W<'_, IMR1_SPEC> {
        IM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM3(&mut self) -> IM3_W<'_, IMR1_SPEC> {
        IM3_W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM4(&mut self) -> IM4_W<'_, IMR1_SPEC> {
        IM4_W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM5(&mut self) -> IM5_W<'_, IMR1_SPEC> {
        IM5_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM6(&mut self) -> IM6_W<'_, IMR1_SPEC> {
        IM6_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM7(&mut self) -> IM7_W<'_, IMR1_SPEC> {
        IM7_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM8(&mut self) -> IM8_W<'_, IMR1_SPEC> {
        IM8_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM9(&mut self) -> IM9_W<'_, IMR1_SPEC> {
        IM9_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM10(&mut self) -> IM10_W<'_, IMR1_SPEC> {
        IM10_W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM11(&mut self) -> IM11_W<'_, IMR1_SPEC> {
        IM11_W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM12(&mut self) -> IM12_W<'_, IMR1_SPEC> {
        IM12_W::new(self, 12)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM13(&mut self) -> IM13_W<'_, IMR1_SPEC> {
        IM13_W::new(self, 13)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM14(&mut self) -> IM14_W<'_, IMR1_SPEC> {
        IM14_W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM15(&mut self) -> IM15_W<'_, IMR1_SPEC> {
        IM15_W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM16(&mut self) -> IM16_W<'_, IMR1_SPEC> {
        IM16_W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input x (x = 17 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM17(&mut self) -> IM17_W<'_, IMR1_SPEC> {
        IM17_W::new(self, 17)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM19(&mut self) -> IM19_W<'_, IMR1_SPEC> {
        IM19_W::new(self, 19)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM21(&mut self) -> IM21_W<'_, IMR1_SPEC> {
        IM21_W::new(self, 21)
    }
    #[doc = "Bit 22 - CPU wakeup with interrupt mask on event input x (x = 22 to 21) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM22(&mut self) -> IM22_W<'_, IMR1_SPEC> {
        IM22_W::new(self, 22)
    }
    #[doc = "Bit 24 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM24(&mut self) -> IM24_W<'_, IMR1_SPEC> {
        IM24_W::new(self, 24)
    }
    #[doc = "Bit 25 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM25(&mut self) -> IM25_W<'_, IMR1_SPEC> {
        IM25_W::new(self, 25)
    }
    #[doc = "Bit 26 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM26(&mut self) -> IM26_W<'_, IMR1_SPEC> {
        IM26_W::new(self, 26)
    }
    #[doc = "Bit 27 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM27(&mut self) -> IM27_W<'_, IMR1_SPEC> {
        IM27_W::new(self, 27)
    }
    #[doc = "Bit 28 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM28(&mut self) -> IM28_W<'_, IMR1_SPEC> {
        IM28_W::new(self, 28)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input x (x = 29 to 24) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM29(&mut self) -> IM29_W<'_, IMR1_SPEC> {
        IM29_W::new(self, 29)
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr1::R`](R) reader structure"]
impl crate::Readable for IMR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr1::W`](W) writer structure"]
impl crate::Writable for IMR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IMR1 to value 0xfffe_0000"]
impl crate::Resettable for IMR1_SPEC {
    const RESET_VALUE: u32 = 0xfffe_0000;
}
