#[doc = "Register `EMR2` reader"]
pub type R = crate::R<EMR2_SPEC>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<EMR2_SPEC>;
#[doc = "CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM37_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM37_A> for bool {
    #[inline(always)]
    fn from(variant: EM37_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM37` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM37_R = crate::BitReader<EM37_A>;
impl EM37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM37_A {
        match self.bits {
            false => EM37_A::B_0x0,
            true => EM37_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM37_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM37_A::B_0x1
    }
}
#[doc = "Field `EM37` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM37_W<'a, REG> = crate::BitWriter<'a, REG, EM37_A>;
impl<'a, REG> EM37_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM37_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM37_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM38_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM38_A> for bool {
    #[inline(always)]
    fn from(variant: EM38_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM38` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM38_R = crate::BitReader<EM38_A>;
impl EM38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM38_A {
        match self.bits {
            false => EM38_A::B_0x0,
            true => EM38_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM38_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM38_A::B_0x1
    }
}
#[doc = "Field `EM38` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM38_W<'a, REG> = crate::BitWriter<'a, REG, EM38_A>;
impl<'a, REG> EM38_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM38_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM38_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM39_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM39_A> for bool {
    #[inline(always)]
    fn from(variant: EM39_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM39` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM39_R = crate::BitReader<EM39_A>;
impl EM39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM39_A {
        match self.bits {
            false => EM39_A::B_0x0,
            true => EM39_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM39_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM39_A::B_0x1
    }
}
#[doc = "Field `EM39` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM39_W<'a, REG> = crate::BitWriter<'a, REG, EM39_A>;
impl<'a, REG> EM39_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM39_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM39_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM40_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM40_A> for bool {
    #[inline(always)]
    fn from(variant: EM40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM40` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM40_R = crate::BitReader<EM40_A>;
impl EM40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM40_A {
        match self.bits {
            false => EM40_A::B_0x0,
            true => EM40_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM40_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM40_A::B_0x1
    }
}
#[doc = "Field `EM40` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM40_W<'a, REG> = crate::BitWriter<'a, REG, EM40_A>;
impl<'a, REG> EM40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM40_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM40_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM41_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM41_A> for bool {
    #[inline(always)]
    fn from(variant: EM41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM41` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM41_R = crate::BitReader<EM41_A>;
impl EM41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM41_A {
        match self.bits {
            false => EM41_A::B_0x0,
            true => EM41_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM41_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM41_A::B_0x1
    }
}
#[doc = "Field `EM41` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM41_W<'a, REG> = crate::BitWriter<'a, REG, EM41_A>;
impl<'a, REG> EM41_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM41_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM41_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM42_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM42_A> for bool {
    #[inline(always)]
    fn from(variant: EM42_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM42` reader - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM42_R = crate::BitReader<EM42_A>;
impl EM42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM42_A {
        match self.bits {
            false => EM42_A::B_0x0,
            true => EM42_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM42_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM42_A::B_0x1
    }
}
#[doc = "Field `EM42` writer - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM42_W<'a, REG> = crate::BitWriter<'a, REG, EM42_A>;
impl<'a, REG> EM42_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM42_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM42_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM47_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM47_A> for bool {
    #[inline(always)]
    fn from(variant: EM47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM47` reader - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM47_R = crate::BitReader<EM47_A>;
impl EM47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM47_A {
        match self.bits {
            false => EM47_A::B_0x0,
            true => EM47_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM47_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM47_A::B_0x1
    }
}
#[doc = "Field `EM47` writer - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM47_W<'a, REG> = crate::BitWriter<'a, REG, EM47_A>;
impl<'a, REG> EM47_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM47_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM47_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM49_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM49_A> for bool {
    #[inline(always)]
    fn from(variant: EM49_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM49` reader - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM49_R = crate::BitReader<EM49_A>;
impl EM49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM49_A {
        match self.bits {
            false => EM49_A::B_0x0,
            true => EM49_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM49_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM49_A::B_0x1
    }
}
#[doc = "Field `EM49` writer - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM49_W<'a, REG> = crate::BitWriter<'a, REG, EM49_A>;
impl<'a, REG> EM49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM49_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM49_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM50_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM50_A> for bool {
    #[inline(always)]
    fn from(variant: EM50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM50` reader - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM50_R = crate::BitReader<EM50_A>;
impl EM50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM50_A {
        match self.bits {
            false => EM50_A::B_0x0,
            true => EM50_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM50_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM50_A::B_0x1
    }
}
#[doc = "Field `EM50` writer - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM50_W<'a, REG> = crate::BitWriter<'a, REG, EM50_A>;
impl<'a, REG> EM50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM50_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM50_A::B_0x1)
    }
}
#[doc = "CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM53_A {
    #[doc = "0: Wakeup with event generation from Line x is masked."]
    B_0x0 = 0,
    #[doc = "1: Wakeup with event generation from Line x is unmasked."]
    B_0x1 = 1,
}
impl From<EM53_A> for bool {
    #[inline(always)]
    fn from(variant: EM53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM53` reader - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM53_R = crate::BitReader<EM53_A>;
impl EM53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EM53_A {
        match self.bits {
            false => EM53_A::B_0x0,
            true => EM53_A::B_0x1,
        }
    }
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EM53_A::B_0x0
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EM53_A::B_0x1
    }
}
#[doc = "Field `EM53` writer - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
pub type EM53_W<'a, REG> = crate::BitWriter<'a, REG, EM53_A>;
impl<'a, REG> EM53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup with event generation from Line x is masked."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EM53_A::B_0x0)
    }
    #[doc = "Wakeup with event generation from Line x is unmasked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EM53_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM38(&self) -> EM38_R {
        EM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM39(&self) -> EM39_R {
        EM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM42(&self) -> EM42_R {
        EM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM47(&self) -> EM47_R {
        EM47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM49(&self) -> EM49_R {
        EM49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM50(&self) -> EM50_R {
        EM50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM53(&self) -> EM53_R {
        EM53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM37(&mut self) -> EM37_W<'_, EMR2_SPEC> {
        EM37_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM38(&mut self) -> EM38_W<'_, EMR2_SPEC> {
        EM38_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM39(&mut self) -> EM39_W<'_, EMR2_SPEC> {
        EM39_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM40(&mut self) -> EM40_W<'_, EMR2_SPEC> {
        EM40_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM41(&mut self) -> EM41_W<'_, EMR2_SPEC> {
        EM41_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with event generation mask on event input x (x = 42 to 37) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM42(&mut self) -> EM42_W<'_, EMR2_SPEC> {
        EM42_W::new(self, 10)
    }
    #[doc = "Bit 15 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM47(&mut self) -> EM47_W<'_, EMR2_SPEC> {
        EM47_W::new(self, 15)
    }
    #[doc = "Bit 17 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM49(&mut self) -> EM49_W<'_, EMR2_SPEC> {
        EM49_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU wakeup with event generation mask on event input x (x = 50 to 49) When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM50(&mut self) -> EM50_W<'_, EMR2_SPEC> {
        EM50_W::new(self, 18)
    }
    #[doc = "Bit 21 - CPU wakeup with event generation mask on event input x When EXTI_PRIVCFGR.PRIVx is disabled, EMx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, EMx can only be accessed with privileged access. Unprivileged write to this bit is discarded."]
    #[inline(always)]
    pub fn EM53(&mut self) -> EM53_W<'_, EMR2_SPEC> {
        EM53_W::new(self, 21)
    }
}
#[doc = "EXTI CPU wakeup with event mask register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR2_SPEC;
impl crate::RegisterSpec for EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for EMR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for EMR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EMR2 to value 0x00db_bfff"]
impl crate::Resettable for EMR2_SPEC {
    const RESET_VALUE: u32 = 0x00db_bfff;
}
