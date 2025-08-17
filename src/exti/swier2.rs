#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2_SPEC>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2_SPEC>;
#[doc = "Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI50_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI50_A> for bool {
    #[inline(always)]
    fn from(variant: SWI50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI50` reader - Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI50_R = crate::BitReader<SWI50_A>;
impl SWI50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI50_A {
        match self.bits {
            false => SWI50_A::B_0x0,
            true => SWI50_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI50_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI50_A::B_0x1
    }
}
#[doc = "Field `SWI50` writer - Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI50_W<'a, REG> = crate::BitWriter<'a, REG, SWI50_A>;
impl<'a, REG> SWI50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI50_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI50_A::B_0x1)
    }
}
#[doc = "Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI53_A {
    #[doc = "0: Writing 0 has no effect."]
    B_0x0 = 0,
    #[doc = "1: Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    B_0x1 = 1,
}
impl From<SWI53_A> for bool {
    #[inline(always)]
    fn from(variant: SWI53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI53` reader - Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI53_R = crate::BitReader<SWI53_A>;
impl SWI53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWI53_A {
        match self.bits {
            false => SWI53_A::B_0x0,
            true => SWI53_A::B_0x1,
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWI53_A::B_0x0
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWI53_A::B_0x1
    }
}
#[doc = "Field `SWI53` writer - Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
pub type SWI53_W<'a, REG> = crate::BitWriter<'a, REG, SWI53_A>;
impl<'a, REG> SWI53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWI53_A::B_0x0)
    }
    #[doc = "Writing 1 triggers a rising edge event on event x. This bit is auto cleared by hardware."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWI53_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 18 - Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI50(&self) -> SWI50_R {
        SWI50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI53(&self) -> SWI53_R {
        SWI53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI50(&mut self) -> SWI50_W<'_, SWIER2_SPEC> {
        SWI50_W::new(self, 18)
    }
    #[doc = "Bit 21 - Software interrupt on event x When EXTI_PRIVCFGR.PRIVx is disabled, SWIx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SWIx can only be accessed with privileged access. Unprivileged write to this SWIx is discarded, unprivileged read returns 0. A software interrupt is generated independent from the setting in EXTI_RTSR and EXTI_FTSR. It always returns 0 when read."]
    #[inline(always)]
    pub fn SWI53(&mut self) -> SWI53_W<'_, SWIER2_SPEC> {
        SWI53_W::new(self, 21)
    }
}
#[doc = "EXTI software interrupt event register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for SWIER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for SWIER2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2_SPEC {}
