#[doc = "Register `RPR1` reader"]
pub type R = crate::R<RPR1_SPEC>;
#[doc = "Register `RPR1` writer"]
pub type W = crate::W<RPR1_SPEC>;
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF0_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF0_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF0` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF0_R = crate::BitReader<RPIF0_A>;
impl RPIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF0_A {
        match self.bits {
            false => RPIF0_A::B_0x0,
            true => RPIF0_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF0_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF0_A::B_0x1
    }
}
#[doc = "Field `RPIF0` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF0_W<'a, REG> = crate::BitWriter<'a, REG, RPIF0_A>;
impl<'a, REG> RPIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF0_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF0_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF1_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF1_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF1` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF1_R = crate::BitReader<RPIF1_A>;
impl RPIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF1_A {
        match self.bits {
            false => RPIF1_A::B_0x0,
            true => RPIF1_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF1_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF1_A::B_0x1
    }
}
#[doc = "Field `RPIF1` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF1_W<'a, REG> = crate::BitWriter<'a, REG, RPIF1_A>;
impl<'a, REG> RPIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF1_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF1_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF2_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF2_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF2` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF2_R = crate::BitReader<RPIF2_A>;
impl RPIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF2_A {
        match self.bits {
            false => RPIF2_A::B_0x0,
            true => RPIF2_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF2_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF2_A::B_0x1
    }
}
#[doc = "Field `RPIF2` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF2_W<'a, REG> = crate::BitWriter<'a, REG, RPIF2_A>;
impl<'a, REG> RPIF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF2_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF2_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF3_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF3_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF3` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF3_R = crate::BitReader<RPIF3_A>;
impl RPIF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF3_A {
        match self.bits {
            false => RPIF3_A::B_0x0,
            true => RPIF3_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF3_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF3_A::B_0x1
    }
}
#[doc = "Field `RPIF3` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF3_W<'a, REG> = crate::BitWriter<'a, REG, RPIF3_A>;
impl<'a, REG> RPIF3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF3_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF3_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF4_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF4_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF4` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF4_R = crate::BitReader<RPIF4_A>;
impl RPIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF4_A {
        match self.bits {
            false => RPIF4_A::B_0x0,
            true => RPIF4_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF4_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF4_A::B_0x1
    }
}
#[doc = "Field `RPIF4` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF4_W<'a, REG> = crate::BitWriter<'a, REG, RPIF4_A>;
impl<'a, REG> RPIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF4_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF4_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF5_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF5_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF5` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF5_R = crate::BitReader<RPIF5_A>;
impl RPIF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF5_A {
        match self.bits {
            false => RPIF5_A::B_0x0,
            true => RPIF5_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF5_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF5_A::B_0x1
    }
}
#[doc = "Field `RPIF5` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF5_W<'a, REG> = crate::BitWriter<'a, REG, RPIF5_A>;
impl<'a, REG> RPIF5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF5_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF5_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF6_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF6_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF6` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF6_R = crate::BitReader<RPIF6_A>;
impl RPIF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF6_A {
        match self.bits {
            false => RPIF6_A::B_0x0,
            true => RPIF6_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF6_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF6_A::B_0x1
    }
}
#[doc = "Field `RPIF6` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF6_W<'a, REG> = crate::BitWriter<'a, REG, RPIF6_A>;
impl<'a, REG> RPIF6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF6_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF6_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF7_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF7_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF7` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF7_R = crate::BitReader<RPIF7_A>;
impl RPIF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF7_A {
        match self.bits {
            false => RPIF7_A::B_0x0,
            true => RPIF7_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF7_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF7_A::B_0x1
    }
}
#[doc = "Field `RPIF7` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF7_W<'a, REG> = crate::BitWriter<'a, REG, RPIF7_A>;
impl<'a, REG> RPIF7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF7_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF7_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF8_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF8_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF8` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF8_R = crate::BitReader<RPIF8_A>;
impl RPIF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF8_A {
        match self.bits {
            false => RPIF8_A::B_0x0,
            true => RPIF8_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF8_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF8_A::B_0x1
    }
}
#[doc = "Field `RPIF8` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF8_W<'a, REG> = crate::BitWriter<'a, REG, RPIF8_A>;
impl<'a, REG> RPIF8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF8_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF8_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF9_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF9_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF9` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF9_R = crate::BitReader<RPIF9_A>;
impl RPIF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF9_A {
        match self.bits {
            false => RPIF9_A::B_0x0,
            true => RPIF9_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF9_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF9_A::B_0x1
    }
}
#[doc = "Field `RPIF9` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF9_W<'a, REG> = crate::BitWriter<'a, REG, RPIF9_A>;
impl<'a, REG> RPIF9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF9_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF9_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF10_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF10_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF10` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF10_R = crate::BitReader<RPIF10_A>;
impl RPIF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF10_A {
        match self.bits {
            false => RPIF10_A::B_0x0,
            true => RPIF10_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF10_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF10_A::B_0x1
    }
}
#[doc = "Field `RPIF10` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF10_W<'a, REG> = crate::BitWriter<'a, REG, RPIF10_A>;
impl<'a, REG> RPIF10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF10_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF10_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF11_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF11_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF11` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF11_R = crate::BitReader<RPIF11_A>;
impl RPIF11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF11_A {
        match self.bits {
            false => RPIF11_A::B_0x0,
            true => RPIF11_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF11_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF11_A::B_0x1
    }
}
#[doc = "Field `RPIF11` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF11_W<'a, REG> = crate::BitWriter<'a, REG, RPIF11_A>;
impl<'a, REG> RPIF11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF11_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF11_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF12_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF12_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF12` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF12_R = crate::BitReader<RPIF12_A>;
impl RPIF12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF12_A {
        match self.bits {
            false => RPIF12_A::B_0x0,
            true => RPIF12_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF12_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF12_A::B_0x1
    }
}
#[doc = "Field `RPIF12` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF12_W<'a, REG> = crate::BitWriter<'a, REG, RPIF12_A>;
impl<'a, REG> RPIF12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF12_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF12_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF13_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF13_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF13` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF13_R = crate::BitReader<RPIF13_A>;
impl RPIF13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF13_A {
        match self.bits {
            false => RPIF13_A::B_0x0,
            true => RPIF13_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF13_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF13_A::B_0x1
    }
}
#[doc = "Field `RPIF13` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF13_W<'a, REG> = crate::BitWriter<'a, REG, RPIF13_A>;
impl<'a, REG> RPIF13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF13_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF13_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF14_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF14_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF14` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF14_R = crate::BitReader<RPIF14_A>;
impl RPIF14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF14_A {
        match self.bits {
            false => RPIF14_A::B_0x0,
            true => RPIF14_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF14_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF14_A::B_0x1
    }
}
#[doc = "Field `RPIF14` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF14_W<'a, REG> = crate::BitWriter<'a, REG, RPIF14_A>;
impl<'a, REG> RPIF14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF14_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF14_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF15_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF15_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF15` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF15_R = crate::BitReader<RPIF15_A>;
impl RPIF15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF15_A {
        match self.bits {
            false => RPIF15_A::B_0x0,
            true => RPIF15_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF15_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF15_A::B_0x1
    }
}
#[doc = "Field `RPIF15` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF15_W<'a, REG> = crate::BitWriter<'a, REG, RPIF15_A>;
impl<'a, REG> RPIF15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF15_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF15_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF16_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF16_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF16` reader - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF16_R = crate::BitReader<RPIF16_A>;
impl RPIF16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF16_A {
        match self.bits {
            false => RPIF16_A::B_0x0,
            true => RPIF16_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF16_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF16_A::B_0x1
    }
}
#[doc = "Field `RPIF16` writer - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF16_W<'a, REG> = crate::BitWriter<'a, REG, RPIF16_A>;
impl<'a, REG> RPIF16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF16_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF16_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF0(&mut self) -> RPIF0_W<'_, RPR1_SPEC> {
        RPIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF1(&mut self) -> RPIF1_W<'_, RPR1_SPEC> {
        RPIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF2(&mut self) -> RPIF2_W<'_, RPR1_SPEC> {
        RPIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF3(&mut self) -> RPIF3_W<'_, RPR1_SPEC> {
        RPIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF4(&mut self) -> RPIF4_W<'_, RPR1_SPEC> {
        RPIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF5(&mut self) -> RPIF5_W<'_, RPR1_SPEC> {
        RPIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF6(&mut self) -> RPIF6_W<'_, RPR1_SPEC> {
        RPIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF7(&mut self) -> RPIF7_W<'_, RPR1_SPEC> {
        RPIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF8(&mut self) -> RPIF8_W<'_, RPR1_SPEC> {
        RPIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF9(&mut self) -> RPIF9_W<'_, RPR1_SPEC> {
        RPIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF10(&mut self) -> RPIF10_W<'_, RPR1_SPEC> {
        RPIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF11(&mut self) -> RPIF11_W<'_, RPR1_SPEC> {
        RPIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF12(&mut self) -> RPIF12_W<'_, RPR1_SPEC> {
        RPIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF13(&mut self) -> RPIF13_W<'_, RPR1_SPEC> {
        RPIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF14(&mut self) -> RPIF14_W<'_, RPR1_SPEC> {
        RPIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF15(&mut self) -> RPIF15_W<'_, RPR1_SPEC> {
        RPIF15_W::new(self, 15)
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge pending bit (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF16(&mut self) -> RPIF16_W<'_, RPR1_SPEC> {
        RPIF16_W::new(self, 16)
    }
}
#[doc = "EXTI rising edge pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR1_SPEC;
impl crate::RegisterSpec for RPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr1::R`](R) reader structure"]
impl crate::Readable for RPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rpr1::W`](W) writer structure"]
impl crate::Writable for RPR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RPR1 to value 0"]
impl crate::Resettable for RPR1_SPEC {}
