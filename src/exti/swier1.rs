#[doc = "Register `SWIER1` reader"]
pub type R = crate::R<SWIER1_SPEC>;
#[doc = "Register `SWIER1` writer"]
pub type W = crate::W<SWIER1_SPEC>;
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI0_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI0_A> for bool {
    #[inline(always)]
    fn from(variant: SWI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI0` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI0_R = crate::BitReader<SWI0_A>;
impl SWI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI0_A {
        match self.bits {
            false => SWI0_A::B_0x0,
            true => SWI0_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI0_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI0_A::B_0x1
    }
}
#[doc = "Field `SWI0` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG, SWI0_A>;
impl<'a, REG> SWI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI0_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI0_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI1_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI1_A> for bool {
    #[inline(always)]
    fn from(variant: SWI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI1` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI1_R = crate::BitReader<SWI1_A>;
impl SWI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI1_A {
        match self.bits {
            false => SWI1_A::B_0x0,
            true => SWI1_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI1_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI1_A::B_0x1
    }
}
#[doc = "Field `SWI1` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI1_W<'a, REG> = crate::BitWriter<'a, REG, SWI1_A>;
impl<'a, REG> SWI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI1_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI1_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI2_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI2_A> for bool {
    #[inline(always)]
    fn from(variant: SWI2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI2` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI2_R = crate::BitReader<SWI2_A>;
impl SWI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI2_A {
        match self.bits {
            false => SWI2_A::B_0x0,
            true => SWI2_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI2_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI2_A::B_0x1
    }
}
#[doc = "Field `SWI2` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG, SWI2_A>;
impl<'a, REG> SWI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI2_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI2_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI3_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI3_A> for bool {
    #[inline(always)]
    fn from(variant: SWI3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI3` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI3_R = crate::BitReader<SWI3_A>;
impl SWI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI3_A {
        match self.bits {
            false => SWI3_A::B_0x0,
            true => SWI3_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI3_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI3_A::B_0x1
    }
}
#[doc = "Field `SWI3` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI3_W<'a, REG> = crate::BitWriter<'a, REG, SWI3_A>;
impl<'a, REG> SWI3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI3_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI3_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI4_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI4_A> for bool {
    #[inline(always)]
    fn from(variant: SWI4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI4` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI4_R = crate::BitReader<SWI4_A>;
impl SWI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI4_A {
        match self.bits {
            false => SWI4_A::B_0x0,
            true => SWI4_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI4_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI4_A::B_0x1
    }
}
#[doc = "Field `SWI4` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI4_W<'a, REG> = crate::BitWriter<'a, REG, SWI4_A>;
impl<'a, REG> SWI4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI4_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI4_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI5_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI5_A> for bool {
    #[inline(always)]
    fn from(variant: SWI5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI5` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI5_R = crate::BitReader<SWI5_A>;
impl SWI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI5_A {
        match self.bits {
            false => SWI5_A::B_0x0,
            true => SWI5_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI5_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI5_A::B_0x1
    }
}
#[doc = "Field `SWI5` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI5_W<'a, REG> = crate::BitWriter<'a, REG, SWI5_A>;
impl<'a, REG> SWI5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI5_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI5_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI6_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI6_A> for bool {
    #[inline(always)]
    fn from(variant: SWI6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI6` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI6_R = crate::BitReader<SWI6_A>;
impl SWI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI6_A {
        match self.bits {
            false => SWI6_A::B_0x0,
            true => SWI6_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI6_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI6_A::B_0x1
    }
}
#[doc = "Field `SWI6` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI6_W<'a, REG> = crate::BitWriter<'a, REG, SWI6_A>;
impl<'a, REG> SWI6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI6_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI6_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI7_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI7_A> for bool {
    #[inline(always)]
    fn from(variant: SWI7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI7` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI7_R = crate::BitReader<SWI7_A>;
impl SWI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI7_A {
        match self.bits {
            false => SWI7_A::B_0x0,
            true => SWI7_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI7_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI7_A::B_0x1
    }
}
#[doc = "Field `SWI7` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI7_W<'a, REG> = crate::BitWriter<'a, REG, SWI7_A>;
impl<'a, REG> SWI7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI7_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI7_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI8_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI8_A> for bool {
    #[inline(always)]
    fn from(variant: SWI8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI8` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI8_R = crate::BitReader<SWI8_A>;
impl SWI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI8_A {
        match self.bits {
            false => SWI8_A::B_0x0,
            true => SWI8_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI8_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI8_A::B_0x1
    }
}
#[doc = "Field `SWI8` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI8_W<'a, REG> = crate::BitWriter<'a, REG, SWI8_A>;
impl<'a, REG> SWI8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI8_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI8_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI9_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI9_A> for bool {
    #[inline(always)]
    fn from(variant: SWI9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI9` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI9_R = crate::BitReader<SWI9_A>;
impl SWI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI9_A {
        match self.bits {
            false => SWI9_A::B_0x0,
            true => SWI9_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI9_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI9_A::B_0x1
    }
}
#[doc = "Field `SWI9` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI9_W<'a, REG> = crate::BitWriter<'a, REG, SWI9_A>;
impl<'a, REG> SWI9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI9_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI9_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI10_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI10_A> for bool {
    #[inline(always)]
    fn from(variant: SWI10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI10` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI10_R = crate::BitReader<SWI10_A>;
impl SWI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI10_A {
        match self.bits {
            false => SWI10_A::B_0x0,
            true => SWI10_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI10_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI10_A::B_0x1
    }
}
#[doc = "Field `SWI10` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI10_W<'a, REG> = crate::BitWriter<'a, REG, SWI10_A>;
impl<'a, REG> SWI10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI10_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI10_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI11_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI11_A> for bool {
    #[inline(always)]
    fn from(variant: SWI11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI11` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI11_R = crate::BitReader<SWI11_A>;
impl SWI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI11_A {
        match self.bits {
            false => SWI11_A::B_0x0,
            true => SWI11_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI11_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI11_A::B_0x1
    }
}
#[doc = "Field `SWI11` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI11_W<'a, REG> = crate::BitWriter<'a, REG, SWI11_A>;
impl<'a, REG> SWI11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI11_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI11_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI12_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI12_A> for bool {
    #[inline(always)]
    fn from(variant: SWI12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI12` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI12_R = crate::BitReader<SWI12_A>;
impl SWI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI12_A {
        match self.bits {
            false => SWI12_A::B_0x0,
            true => SWI12_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI12_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI12_A::B_0x1
    }
}
#[doc = "Field `SWI12` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI12_W<'a, REG> = crate::BitWriter<'a, REG, SWI12_A>;
impl<'a, REG> SWI12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI12_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI12_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI13_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI13_A> for bool {
    #[inline(always)]
    fn from(variant: SWI13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI13` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI13_R = crate::BitReader<SWI13_A>;
impl SWI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI13_A {
        match self.bits {
            false => SWI13_A::B_0x0,
            true => SWI13_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI13_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI13_A::B_0x1
    }
}
#[doc = "Field `SWI13` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI13_W<'a, REG> = crate::BitWriter<'a, REG, SWI13_A>;
impl<'a, REG> SWI13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI13_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI13_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI14_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI14_A> for bool {
    #[inline(always)]
    fn from(variant: SWI14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI14` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI14_R = crate::BitReader<SWI14_A>;
impl SWI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI14_A {
        match self.bits {
            false => SWI14_A::B_0x0,
            true => SWI14_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI14_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI14_A::B_0x1
    }
}
#[doc = "Field `SWI14` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI14_W<'a, REG> = crate::BitWriter<'a, REG, SWI14_A>;
impl<'a, REG> SWI14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI14_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI14_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI15_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI15_A> for bool {
    #[inline(always)]
    fn from(variant: SWI15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI15` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI15_R = crate::BitReader<SWI15_A>;
impl SWI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI15_A {
        match self.bits {
            false => SWI15_A::B_0x0,
            true => SWI15_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI15_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI15_A::B_0x1
    }
}
#[doc = "Field `SWI15` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI15_W<'a, REG> = crate::BitWriter<'a, REG, SWI15_A>;
impl<'a, REG> SWI15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI15_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI15_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI16_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI16_A> for bool {
    #[inline(always)]
    fn from(variant: SWI16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI16` reader - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI16_R = crate::BitReader<SWI16_A>;
impl SWI16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI16_A {
        match self.bits {
            false => SWI16_A::B_0x0,
            true => SWI16_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI16_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI16_A::B_0x1
    }
}
#[doc = "Field `SWI16` writer - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI16_W<'a, REG> = crate::BitWriter<'a, REG, SWI16_A>;
impl<'a, REG> SWI16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI16_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI16_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI0(&mut self) -> SWI0_W<'_, SWIER1_SPEC> {
        SWI0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI1(&mut self) -> SWI1_W<'_, SWIER1_SPEC> {
        SWI1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI2(&mut self) -> SWI2_W<'_, SWIER1_SPEC> {
        SWI2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI3(&mut self) -> SWI3_W<'_, SWIER1_SPEC> {
        SWI3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI4(&mut self) -> SWI4_W<'_, SWIER1_SPEC> {
        SWI4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI5(&mut self) -> SWI5_W<'_, SWIER1_SPEC> {
        SWI5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI6(&mut self) -> SWI6_W<'_, SWIER1_SPEC> {
        SWI6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI7(&mut self) -> SWI7_W<'_, SWIER1_SPEC> {
        SWI7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI8(&mut self) -> SWI8_W<'_, SWIER1_SPEC> {
        SWI8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI9(&mut self) -> SWI9_W<'_, SWIER1_SPEC> {
        SWI9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI10(&mut self) -> SWI10_W<'_, SWIER1_SPEC> {
        SWI10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI11(&mut self) -> SWI11_W<'_, SWIER1_SPEC> {
        SWI11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI12(&mut self) -> SWI12_W<'_, SWIER1_SPEC> {
        SWI12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI13(&mut self) -> SWI13_W<'_, SWIER1_SPEC> {
        SWI13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI14(&mut self) -> SWI14_W<'_, SWIER1_SPEC> {
        SWI14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI15(&mut self) -> SWI15_W<'_, SWIER1_SPEC> {
        SWI15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Software interrupt on event x (x = 16 to 0) When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI16(&mut self) -> SWI16_W<'_, SWIER1_SPEC> {
        SWI16_W::new(self, 16)
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER1_SPEC;
impl crate::RegisterSpec for SWIER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier1::R`](R) reader structure"]
impl crate::Readable for SWIER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swier1::W`](W) writer structure"]
impl crate::Writable for SWIER1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SWIER1 to value 0"]
impl crate::Resettable for SWIER1_SPEC {}
