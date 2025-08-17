#[doc = "Register `FPR1` reader"]
pub type R = crate::R<FPR1_SPEC>;
#[doc = "Register `FPR1` writer"]
pub type W = crate::W<FPR1_SPEC>;
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF0_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF0_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF0` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF0_R = crate::BitReader<FPIF0_A>;
impl FPIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF0_A {
        match self.bits {
            false => FPIF0_A::B_0x0,
            true => FPIF0_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF0_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF0_A::B_0x1
    }
}
#[doc = "Field `FPIF0` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF0_W<'a, REG> = crate::BitWriter<'a, REG, FPIF0_A>;
impl<'a, REG> FPIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF0_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF0_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF1_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF1_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF1` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF1_R = crate::BitReader<FPIF1_A>;
impl FPIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF1_A {
        match self.bits {
            false => FPIF1_A::B_0x0,
            true => FPIF1_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF1_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF1_A::B_0x1
    }
}
#[doc = "Field `FPIF1` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF1_W<'a, REG> = crate::BitWriter<'a, REG, FPIF1_A>;
impl<'a, REG> FPIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF1_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF1_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF2_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF2_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF2` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF2_R = crate::BitReader<FPIF2_A>;
impl FPIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF2_A {
        match self.bits {
            false => FPIF2_A::B_0x0,
            true => FPIF2_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF2_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF2_A::B_0x1
    }
}
#[doc = "Field `FPIF2` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF2_W<'a, REG> = crate::BitWriter<'a, REG, FPIF2_A>;
impl<'a, REG> FPIF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF2_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF2_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF3_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF3_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF3` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF3_R = crate::BitReader<FPIF3_A>;
impl FPIF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF3_A {
        match self.bits {
            false => FPIF3_A::B_0x0,
            true => FPIF3_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF3_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF3_A::B_0x1
    }
}
#[doc = "Field `FPIF3` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF3_W<'a, REG> = crate::BitWriter<'a, REG, FPIF3_A>;
impl<'a, REG> FPIF3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF3_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF3_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF4_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF4_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF4` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF4_R = crate::BitReader<FPIF4_A>;
impl FPIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF4_A {
        match self.bits {
            false => FPIF4_A::B_0x0,
            true => FPIF4_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF4_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF4_A::B_0x1
    }
}
#[doc = "Field `FPIF4` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF4_W<'a, REG> = crate::BitWriter<'a, REG, FPIF4_A>;
impl<'a, REG> FPIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF4_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF4_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF5_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF5_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF5` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF5_R = crate::BitReader<FPIF5_A>;
impl FPIF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF5_A {
        match self.bits {
            false => FPIF5_A::B_0x0,
            true => FPIF5_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF5_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF5_A::B_0x1
    }
}
#[doc = "Field `FPIF5` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF5_W<'a, REG> = crate::BitWriter<'a, REG, FPIF5_A>;
impl<'a, REG> FPIF5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF5_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF5_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF6_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF6_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF6` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF6_R = crate::BitReader<FPIF6_A>;
impl FPIF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF6_A {
        match self.bits {
            false => FPIF6_A::B_0x0,
            true => FPIF6_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF6_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF6_A::B_0x1
    }
}
#[doc = "Field `FPIF6` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF6_W<'a, REG> = crate::BitWriter<'a, REG, FPIF6_A>;
impl<'a, REG> FPIF6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF6_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF6_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF7_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF7_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF7` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF7_R = crate::BitReader<FPIF7_A>;
impl FPIF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF7_A {
        match self.bits {
            false => FPIF7_A::B_0x0,
            true => FPIF7_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF7_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF7_A::B_0x1
    }
}
#[doc = "Field `FPIF7` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF7_W<'a, REG> = crate::BitWriter<'a, REG, FPIF7_A>;
impl<'a, REG> FPIF7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF7_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF7_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF8_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF8_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF8` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF8_R = crate::BitReader<FPIF8_A>;
impl FPIF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF8_A {
        match self.bits {
            false => FPIF8_A::B_0x0,
            true => FPIF8_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF8_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF8_A::B_0x1
    }
}
#[doc = "Field `FPIF8` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF8_W<'a, REG> = crate::BitWriter<'a, REG, FPIF8_A>;
impl<'a, REG> FPIF8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF8_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF8_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF9_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF9_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF9` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF9_R = crate::BitReader<FPIF9_A>;
impl FPIF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF9_A {
        match self.bits {
            false => FPIF9_A::B_0x0,
            true => FPIF9_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF9_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF9_A::B_0x1
    }
}
#[doc = "Field `FPIF9` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF9_W<'a, REG> = crate::BitWriter<'a, REG, FPIF9_A>;
impl<'a, REG> FPIF9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF9_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF9_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF10_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF10_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF10` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF10_R = crate::BitReader<FPIF10_A>;
impl FPIF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF10_A {
        match self.bits {
            false => FPIF10_A::B_0x0,
            true => FPIF10_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF10_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF10_A::B_0x1
    }
}
#[doc = "Field `FPIF10` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF10_W<'a, REG> = crate::BitWriter<'a, REG, FPIF10_A>;
impl<'a, REG> FPIF10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF10_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF10_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF11_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF11_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF11` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF11_R = crate::BitReader<FPIF11_A>;
impl FPIF11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF11_A {
        match self.bits {
            false => FPIF11_A::B_0x0,
            true => FPIF11_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF11_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF11_A::B_0x1
    }
}
#[doc = "Field `FPIF11` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF11_W<'a, REG> = crate::BitWriter<'a, REG, FPIF11_A>;
impl<'a, REG> FPIF11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF11_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF11_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF12_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF12_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF12` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF12_R = crate::BitReader<FPIF12_A>;
impl FPIF12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF12_A {
        match self.bits {
            false => FPIF12_A::B_0x0,
            true => FPIF12_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF12_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF12_A::B_0x1
    }
}
#[doc = "Field `FPIF12` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF12_W<'a, REG> = crate::BitWriter<'a, REG, FPIF12_A>;
impl<'a, REG> FPIF12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF12_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF12_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF13_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF13_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF13` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF13_R = crate::BitReader<FPIF13_A>;
impl FPIF13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF13_A {
        match self.bits {
            false => FPIF13_A::B_0x0,
            true => FPIF13_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF13_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF13_A::B_0x1
    }
}
#[doc = "Field `FPIF13` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF13_W<'a, REG> = crate::BitWriter<'a, REG, FPIF13_A>;
impl<'a, REG> FPIF13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF13_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF13_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF14_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF14_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF14` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF14_R = crate::BitReader<FPIF14_A>;
impl FPIF14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF14_A {
        match self.bits {
            false => FPIF14_A::B_0x0,
            true => FPIF14_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF14_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF14_A::B_0x1
    }
}
#[doc = "Field `FPIF14` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF14_W<'a, REG> = crate::BitWriter<'a, REG, FPIF14_A>;
impl<'a, REG> FPIF14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF14_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF14_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF15_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF15_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF15` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF15_R = crate::BitReader<FPIF15_A>;
impl FPIF15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF15_A {
        match self.bits {
            false => FPIF15_A::B_0x0,
            true => FPIF15_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF15_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF15_A::B_0x1
    }
}
#[doc = "Field `FPIF15` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF15_W<'a, REG> = crate::BitWriter<'a, REG, FPIF15_A>;
impl<'a, REG> FPIF15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF15_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF15_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF16_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF16_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF16` reader - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF16_R = crate::BitReader<FPIF16_A>;
impl FPIF16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF16_A {
        match self.bits {
            false => FPIF16_A::B_0x0,
            true => FPIF16_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF16_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF16_A::B_0x1
    }
}
#[doc = "Field `FPIF16` writer - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF16_W<'a, REG> = crate::BitWriter<'a, REG, FPIF16_A>;
impl<'a, REG> FPIF16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF16_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF16_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF0(&mut self) -> FPIF0_W<'_, FPR1_SPEC> {
        FPIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF1(&mut self) -> FPIF1_W<'_, FPR1_SPEC> {
        FPIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF2(&mut self) -> FPIF2_W<'_, FPR1_SPEC> {
        FPIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF3(&mut self) -> FPIF3_W<'_, FPR1_SPEC> {
        FPIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF4(&mut self) -> FPIF4_W<'_, FPR1_SPEC> {
        FPIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF5(&mut self) -> FPIF5_W<'_, FPR1_SPEC> {
        FPIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF6(&mut self) -> FPIF6_W<'_, FPR1_SPEC> {
        FPIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF7(&mut self) -> FPIF7_W<'_, FPR1_SPEC> {
        FPIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF8(&mut self) -> FPIF8_W<'_, FPR1_SPEC> {
        FPIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF9(&mut self) -> FPIF9_W<'_, FPR1_SPEC> {
        FPIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF10(&mut self) -> FPIF10_W<'_, FPR1_SPEC> {
        FPIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF11(&mut self) -> FPIF11_W<'_, FPR1_SPEC> {
        FPIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF12(&mut self) -> FPIF12_W<'_, FPR1_SPEC> {
        FPIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF13(&mut self) -> FPIF13_W<'_, FPR1_SPEC> {
        FPIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF14(&mut self) -> FPIF14_W<'_, FPR1_SPEC> {
        FPIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF15(&mut self) -> FPIF15_W<'_, FPR1_SPEC> {
        FPIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - configurable event inputs x falling edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF16(&mut self) -> FPIF16_W<'_, FPR1_SPEC> {
        FPIF16_W::new(self, 16)
    }
}
#[doc = "EXTI falling edge pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPR1_SPEC;
impl crate::RegisterSpec for FPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpr1::R`](R) reader structure"]
impl crate::Readable for FPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpr1::W`](W) writer structure"]
impl crate::Writable for FPR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FPR1 to value 0"]
impl crate::Resettable for FPR1_SPEC {}
