#[doc = "Register `IMR2` reader"]
pub type R = crate::R<IMR2_SPEC>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<IMR2_SPEC>;
#[doc = "CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM37_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM37_A> for bool {
    #[inline(always)]
    fn from(variant: IM37_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM37` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM37_R = crate::BitReader<IM37_A>;
impl IM37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM37_A {
        match self.bits {
            false => IM37_A::B_0x0,
            true => IM37_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM37_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM37_A::B_0x1
    }
}
#[doc = "Field `IM37` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM37_W<'a, REG> = crate::BitWriter<'a, REG, IM37_A>;
impl<'a, REG> IM37_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM37_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM37_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM38_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM38_A> for bool {
    #[inline(always)]
    fn from(variant: IM38_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM38` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM38_R = crate::BitReader<IM38_A>;
impl IM38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM38_A {
        match self.bits {
            false => IM38_A::B_0x0,
            true => IM38_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM38_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM38_A::B_0x1
    }
}
#[doc = "Field `IM38` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM38_W<'a, REG> = crate::BitWriter<'a, REG, IM38_A>;
impl<'a, REG> IM38_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM38_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM38_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM39_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM39_A> for bool {
    #[inline(always)]
    fn from(variant: IM39_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM39` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM39_R = crate::BitReader<IM39_A>;
impl IM39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM39_A {
        match self.bits {
            false => IM39_A::B_0x0,
            true => IM39_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM39_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM39_A::B_0x1
    }
}
#[doc = "Field `IM39` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM39_W<'a, REG> = crate::BitWriter<'a, REG, IM39_A>;
impl<'a, REG> IM39_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM39_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM39_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM40_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM40_A> for bool {
    #[inline(always)]
    fn from(variant: IM40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM40` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM40_R = crate::BitReader<IM40_A>;
impl IM40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM40_A {
        match self.bits {
            false => IM40_A::B_0x0,
            true => IM40_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM40_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM40_A::B_0x1
    }
}
#[doc = "Field `IM40` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM40_W<'a, REG> = crate::BitWriter<'a, REG, IM40_A>;
impl<'a, REG> IM40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM40_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM40_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM41_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM41_A> for bool {
    #[inline(always)]
    fn from(variant: IM41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM41` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM41_R = crate::BitReader<IM41_A>;
impl IM41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM41_A {
        match self.bits {
            false => IM41_A::B_0x0,
            true => IM41_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM41_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM41_A::B_0x1
    }
}
#[doc = "Field `IM41` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM41_W<'a, REG> = crate::BitWriter<'a, REG, IM41_A>;
impl<'a, REG> IM41_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM41_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM41_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM42_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM42_A> for bool {
    #[inline(always)]
    fn from(variant: IM42_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM42` reader - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM42_R = crate::BitReader<IM42_A>;
impl IM42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM42_A {
        match self.bits {
            false => IM42_A::B_0x0,
            true => IM42_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM42_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM42_A::B_0x1
    }
}
#[doc = "Field `IM42` writer - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM42_W<'a, REG> = crate::BitWriter<'a, REG, IM42_A>;
impl<'a, REG> IM42_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM42_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM42_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM47_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM47_A> for bool {
    #[inline(always)]
    fn from(variant: IM47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM47` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM47_R = crate::BitReader<IM47_A>;
impl IM47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM47_A {
        match self.bits {
            false => IM47_A::B_0x0,
            true => IM47_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM47_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM47_A::B_0x1
    }
}
#[doc = "Field `IM47` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM47_W<'a, REG> = crate::BitWriter<'a, REG, IM47_A>;
impl<'a, REG> IM47_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM47_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM47_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM49_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM49_A> for bool {
    #[inline(always)]
    fn from(variant: IM49_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM49` reader - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM49_R = crate::BitReader<IM49_A>;
impl IM49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM49_A {
        match self.bits {
            false => IM49_A::B_0x0,
            true => IM49_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM49_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM49_A::B_0x1
    }
}
#[doc = "Field `IM49` writer - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM49_W<'a, REG> = crate::BitWriter<'a, REG, IM49_A>;
impl<'a, REG> IM49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM49_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM49_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM50_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM50_A> for bool {
    #[inline(always)]
    fn from(variant: IM50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM50` reader - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM50_R = crate::BitReader<IM50_A>;
impl IM50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM50_A {
        match self.bits {
            false => IM50_A::B_0x0,
            true => IM50_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM50_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM50_A::B_0x1
    }
}
#[doc = "Field `IM50` writer - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM50_W<'a, REG> = crate::BitWriter<'a, REG, IM50_A>;
impl<'a, REG> IM50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM50_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM50_A::B_0x1)
    }
}
#[doc = "CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IM53_A {
    #[doc = "0: Wakeup with interrupt request from input event x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with interrupt request from input event x is unmasked."]
    B_0x1 = 1,
}
impl From<IM53_A> for bool {
    #[inline(always)]
    fn from(variant: IM53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM53` reader - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM53_R = crate::BitReader<IM53_A>;
impl IM53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IM53_A {
        match self.bits {
            false => IM53_A::B_0x0,
            true => IM53_A::B_0x1,
        }
    }
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IM53_A::B_0x0
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IM53_A::B_0x1
    }
}
#[doc = "Field `IM53` writer - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type IM53_W<'a, REG> = crate::BitWriter<'a, REG, IM53_A>;
impl<'a, REG> IM53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with interrupt request from input event x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IM53_A::B_0x0)
    }
    #[doc = "Wakeup with interrupt request from input event x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IM53_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM47(&self) -> IM47_R {
        IM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM49(&self) -> IM49_R {
        IM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM50(&self) -> IM50_R {
        IM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM53(&self) -> IM53_R {
        IM53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM37(&mut self) -> IM37_W<'_, IMR2_SPEC> {
        IM37_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM38(&mut self) -> IM38_W<'_, IMR2_SPEC> {
        IM38_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM39(&mut self) -> IM39_W<'_, IMR2_SPEC> {
        IM39_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM40(&mut self) -> IM40_W<'_, IMR2_SPEC> {
        IM40_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM41(&mut self) -> IM41_W<'_, IMR2_SPEC> {
        IM41_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM42(&mut self) -> IM42_W<'_, IMR2_SPEC> {
        IM42_W::new(self, 10)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM47(&mut self) -> IM47_W<'_, IMR2_SPEC> {
        IM47_W::new(self, 15)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM49(&mut self) -> IM49_W<'_, IMR2_SPEC> {
        IM49_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM50(&mut self) -> IM50_W<'_, IMR2_SPEC> {
        IM50_W::new(self, 18)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, IMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, IMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn IM53(&mut self) -> IM53_W<'_, IMR2_SPEC> {
        IM53_W::new(self, 21)
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for IMR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for IMR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IMR2 to value 0x00db_bfff"]
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: u32 = 0x00db_bfff;
}
