#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2_SPEC>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2_SPEC>;
#[doc = "Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT50_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT50_A> for bool {
    #[inline(always)]
    fn from(variant: RT50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT50` reader - Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT50_R = crate::BitReader<RT50_A>;
impl RT50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT50_A {
        match self.bits {
            false => RT50_A::B_0x0,
            true => RT50_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT50_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT50_A::B_0x1
    }
}
#[doc = "Field `RT50` writer - Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT50_W<'a, REG> = crate::BitWriter<'a, REG, RT50_A>;
impl<'a, REG> RT50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT50_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT50_A::B_0x1)
    }
}
#[doc = "Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT53_A {
    #[doc = "0: Rising trigger disabled (for event and interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Rising trigger enabled (for event and interrupt) for input line"]
    B_0x1 = 1,
}
impl From<RT53_A> for bool {
    #[inline(always)]
    fn from(variant: RT53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT53` reader - Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT53_R = crate::BitReader<RT53_A>;
impl RT53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RT53_A {
        match self.bits {
            false => RT53_A::B_0x0,
            true => RT53_A::B_0x1,
        }
    }
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RT53_A::B_0x0
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RT53_A::B_0x1
    }
}
#[doc = "Field `RT53` writer - Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
pub type RT53_W<'a, REG> = crate::BitWriter<'a, REG, RT53_A>;
impl<'a, REG> RT53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RT53_A::B_0x0)
    }
    #[doc = "Rising trigger enabled (for event and interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RT53_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 18 - Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT50(&self) -> RT50_R {
        RT50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT53(&self) -> RT53_R {
        RT53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT50(&mut self) -> RT50_W<'_, RTSR2_SPEC> {
        RT50_W::new(self, 18)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, RTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RTx can only be accessed with privileged access. Unprivileged write to this bit x is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn RT53(&mut self) -> RT53_W<'_, RTSR2_SPEC> {
        RT53_W::new(self, 21)
    }
}
#[doc = "EXTI rising trigger selection register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR2_SPEC;
impl crate::RegisterSpec for RTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr2::R`](R) reader structure"]
impl crate::Readable for RTSR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure"]
impl crate::Writable for RTSR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for RTSR2_SPEC {}
