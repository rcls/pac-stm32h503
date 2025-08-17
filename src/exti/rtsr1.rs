#[doc = "Register `RTSR1` reader"]
pub type R = crate::R<RTSR1_SPEC>;
#[doc = "Register `RTSR1` writer"]
pub type W = crate::W<RTSR1_SPEC>;
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT0_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT0_A> for bool {
    #[inline(always)]
    fn from(variant: RT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT0_R = crate::BitReader<RT0_A>;
impl RT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT0_A {
        match self.bits {
            false => RT0_A::B_0x0,
            true => RT0_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT0_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT0_A::B_0x1
    }
}
#[doc = "Field `RT0` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT0_W<'a, REG> = crate::BitWriter<'a, REG, RT0_A>;
impl<'a, REG> RT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT0_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT0_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT1_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT1_A> for bool {
    #[inline(always)]
    fn from(variant: RT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT1_R = crate::BitReader<RT1_A>;
impl RT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT1_A {
        match self.bits {
            false => RT1_A::B_0x0,
            true => RT1_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT1_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT1_A::B_0x1
    }
}
#[doc = "Field `RT1` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT1_W<'a, REG> = crate::BitWriter<'a, REG, RT1_A>;
impl<'a, REG> RT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT1_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT1_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT2_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT2_A> for bool {
    #[inline(always)]
    fn from(variant: RT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT2` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT2_R = crate::BitReader<RT2_A>;
impl RT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT2_A {
        match self.bits {
            false => RT2_A::B_0x0,
            true => RT2_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT2_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT2_A::B_0x1
    }
}
#[doc = "Field `RT2` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT2_W<'a, REG> = crate::BitWriter<'a, REG, RT2_A>;
impl<'a, REG> RT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT2_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT2_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT3_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT3_A> for bool {
    #[inline(always)]
    fn from(variant: RT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT3` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT3_R = crate::BitReader<RT3_A>;
impl RT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT3_A {
        match self.bits {
            false => RT3_A::B_0x0,
            true => RT3_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT3_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT3_A::B_0x1
    }
}
#[doc = "Field `RT3` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT3_W<'a, REG> = crate::BitWriter<'a, REG, RT3_A>;
impl<'a, REG> RT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT3_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT3_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT4_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT4_A> for bool {
    #[inline(always)]
    fn from(variant: RT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT4` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT4_R = crate::BitReader<RT4_A>;
impl RT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT4_A {
        match self.bits {
            false => RT4_A::B_0x0,
            true => RT4_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT4_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT4_A::B_0x1
    }
}
#[doc = "Field `RT4` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT4_W<'a, REG> = crate::BitWriter<'a, REG, RT4_A>;
impl<'a, REG> RT4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT4_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT4_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT5_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT5_A> for bool {
    #[inline(always)]
    fn from(variant: RT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT5` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT5_R = crate::BitReader<RT5_A>;
impl RT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT5_A {
        match self.bits {
            false => RT5_A::B_0x0,
            true => RT5_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT5_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT5_A::B_0x1
    }
}
#[doc = "Field `RT5` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT5_W<'a, REG> = crate::BitWriter<'a, REG, RT5_A>;
impl<'a, REG> RT5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT5_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT5_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT6_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT6_A> for bool {
    #[inline(always)]
    fn from(variant: RT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT6` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT6_R = crate::BitReader<RT6_A>;
impl RT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT6_A {
        match self.bits {
            false => RT6_A::B_0x0,
            true => RT6_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT6_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT6_A::B_0x1
    }
}
#[doc = "Field `RT6` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT6_W<'a, REG> = crate::BitWriter<'a, REG, RT6_A>;
impl<'a, REG> RT6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT6_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT6_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT7_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT7_A> for bool {
    #[inline(always)]
    fn from(variant: RT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT7` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT7_R = crate::BitReader<RT7_A>;
impl RT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT7_A {
        match self.bits {
            false => RT7_A::B_0x0,
            true => RT7_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT7_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT7_A::B_0x1
    }
}
#[doc = "Field `RT7` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT7_W<'a, REG> = crate::BitWriter<'a, REG, RT7_A>;
impl<'a, REG> RT7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT7_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT7_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT8_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT8_A> for bool {
    #[inline(always)]
    fn from(variant: RT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT8` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT8_R = crate::BitReader<RT8_A>;
impl RT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT8_A {
        match self.bits {
            false => RT8_A::B_0x0,
            true => RT8_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT8_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT8_A::B_0x1
    }
}
#[doc = "Field `RT8` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT8_W<'a, REG> = crate::BitWriter<'a, REG, RT8_A>;
impl<'a, REG> RT8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT8_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT8_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT9_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT9_A> for bool {
    #[inline(always)]
    fn from(variant: RT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT9` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT9_R = crate::BitReader<RT9_A>;
impl RT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT9_A {
        match self.bits {
            false => RT9_A::B_0x0,
            true => RT9_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT9_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT9_A::B_0x1
    }
}
#[doc = "Field `RT9` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT9_W<'a, REG> = crate::BitWriter<'a, REG, RT9_A>;
impl<'a, REG> RT9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT9_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT9_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT10_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT10_A> for bool {
    #[inline(always)]
    fn from(variant: RT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT10` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT10_R = crate::BitReader<RT10_A>;
impl RT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT10_A {
        match self.bits {
            false => RT10_A::B_0x0,
            true => RT10_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT10_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT10_A::B_0x1
    }
}
#[doc = "Field `RT10` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT10_W<'a, REG> = crate::BitWriter<'a, REG, RT10_A>;
impl<'a, REG> RT10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT10_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT10_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT11_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT11_A> for bool {
    #[inline(always)]
    fn from(variant: RT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT11` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT11_R = crate::BitReader<RT11_A>;
impl RT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT11_A {
        match self.bits {
            false => RT11_A::B_0x0,
            true => RT11_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT11_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT11_A::B_0x1
    }
}
#[doc = "Field `RT11` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT11_W<'a, REG> = crate::BitWriter<'a, REG, RT11_A>;
impl<'a, REG> RT11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT11_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT11_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT12_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT12_A> for bool {
    #[inline(always)]
    fn from(variant: RT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT12` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT12_R = crate::BitReader<RT12_A>;
impl RT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT12_A {
        match self.bits {
            false => RT12_A::B_0x0,
            true => RT12_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT12_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT12_A::B_0x1
    }
}
#[doc = "Field `RT12` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT12_W<'a, REG> = crate::BitWriter<'a, REG, RT12_A>;
impl<'a, REG> RT12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT12_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT12_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT13_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT13_A> for bool {
    #[inline(always)]
    fn from(variant: RT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT13` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT13_R = crate::BitReader<RT13_A>;
impl RT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT13_A {
        match self.bits {
            false => RT13_A::B_0x0,
            true => RT13_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT13_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT13_A::B_0x1
    }
}
#[doc = "Field `RT13` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT13_W<'a, REG> = crate::BitWriter<'a, REG, RT13_A>;
impl<'a, REG> RT13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT13_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT13_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT14_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT14_A> for bool {
    #[inline(always)]
    fn from(variant: RT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT14` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT14_R = crate::BitReader<RT14_A>;
impl RT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT14_A {
        match self.bits {
            false => RT14_A::B_0x0,
            true => RT14_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT14_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT14_A::B_0x1
    }
}
#[doc = "Field `RT14` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT14_W<'a, REG> = crate::BitWriter<'a, REG, RT14_A>;
impl<'a, REG> RT14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT14_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT14_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT15_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT15_A> for bool {
    #[inline(always)]
    fn from(variant: RT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT15` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT15_R = crate::BitReader<RT15_A>;
impl RT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT15_A {
        match self.bits {
            false => RT15_A::B_0x0,
            true => RT15_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT15_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT15_A::B_0x1
    }
}
#[doc = "Field `RT15` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT15_W<'a, REG> = crate::BitWriter<'a, REG, RT15_A>;
impl<'a, REG> RT15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT15_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT15_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT16_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT16_A> for bool {
    #[inline(always)]
    fn from(variant: RT16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT16` reader - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT16_R = crate::BitReader<RT16_A>;
impl RT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT16_A {
        match self.bits {
            false => RT16_A::B_0x0,
            true => RT16_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT16_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT16_A::B_0x1
    }
}
#[doc = "Field `RT16` writer - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT16_W<'a, REG> = crate::BitWriter<'a, REG, RT16_A>;
impl<'a, REG> RT16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT16_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT16_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT0(&mut self) -> RT0_W<'_, RTSR1_SPEC> {
        RT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT1(&mut self) -> RT1_W<'_, RTSR1_SPEC> {
        RT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT2(&mut self) -> RT2_W<'_, RTSR1_SPEC> {
        RT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT3(&mut self) -> RT3_W<'_, RTSR1_SPEC> {
        RT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT4(&mut self) -> RT4_W<'_, RTSR1_SPEC> {
        RT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT5(&mut self) -> RT5_W<'_, RTSR1_SPEC> {
        RT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT6(&mut self) -> RT6_W<'_, RTSR1_SPEC> {
        RT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT7(&mut self) -> RT7_W<'_, RTSR1_SPEC> {
        RT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT8(&mut self) -> RT8_W<'_, RTSR1_SPEC> {
        RT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT9(&mut self) -> RT9_W<'_, RTSR1_SPEC> {
        RT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT10(&mut self) -> RT10_W<'_, RTSR1_SPEC> {
        RT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT11(&mut self) -> RT11_W<'_, RTSR1_SPEC> {
        RT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT12(&mut self) -> RT12_W<'_, RTSR1_SPEC> {
        RT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT13(&mut self) -> RT13_W<'_, RTSR1_SPEC> {
        RT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT14(&mut self) -> RT14_W<'_, RTSR1_SPEC> {
        RT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT15(&mut self) -> RT15_W<'_, RTSR1_SPEC> {
        RT15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of configurable event input x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT16(&mut self) -> RT16_W<'_, RTSR1_SPEC> {
        RT16_W::new(self, 16)
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR1_SPEC;
impl crate::RegisterSpec for RTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr1::R`](R) reader structure"]
impl crate::Readable for RTSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtsr1::W`](W) writer structure"]
impl crate::Writable for RTSR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RTSR1 to value 0"]
impl crate::Resettable for RTSR1_SPEC {}
