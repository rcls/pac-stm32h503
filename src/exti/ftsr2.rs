#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<FTSR2_SPEC>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<FTSR2_SPEC>;
#[doc = "Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT50_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT50_A> for bool {
    #[inline(always)]
    fn from(variant: FT50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT50` reader - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT50_R = crate::BitReader<FT50_A>;
impl FT50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT50_A {
        match self.bits {
            false => FT50_A::B_0x0,
            true => FT50_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT50_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT50_A::B_0x1
    }
}
#[doc = "Field `FT50` writer - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT50_W<'a, REG> = crate::BitWriter<'a, REG, FT50_A>;
impl<'a, REG> FT50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT50_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT50_A::B_0x1)
    }
}
#[doc = "Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT53_A {
    #[doc = "0: Falling trigger disabled (for event and Interrupt) for input line"]
    B_0x0 = 0,
    #[doc = "1: Falling trigger enabled (for event and Interrupt) for input line."]
    B_0x1 = 1,
}
impl From<FT53_A> for bool {
    #[inline(always)]
    fn from(variant: FT53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT53` reader - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT53_R = crate::BitReader<FT53_A>;
impl FT53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FT53_A {
        match self.bits {
            false => FT53_A::B_0x0,
            true => FT53_A::B_0x1,
        }
    }
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FT53_A::B_0x0
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FT53_A::B_0x1
    }
}
#[doc = "Field `FT53` writer - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
pub type FT53_W<'a, REG> = crate::BitWriter<'a, REG, FT53_A>;
impl<'a, REG> FT53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling trigger disabled (for event and Interrupt) for input line"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FT53_A::B_0x0)
    }
    #[doc = "Falling trigger enabled (for event and Interrupt) for input line."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FT53_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 18 - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT50(&self) -> FT50_R {
        FT50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT53(&self) -> FT53_R {
        FT53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT50(&mut self) -> FT50_W<'_, FTSR2_SPEC> {
        FT50_W::new(self, 18)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of configurable event input x When EXTI_PRIVCFGR.PRIVx is disabled, FTx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FTx can only be accessed with privileged access. Unprivileged write to this FTx is discarded, unprivileged read returns 0."]
    #[inline(always)]
    pub fn FT53(&mut self) -> FT53_W<'_, FTSR2_SPEC> {
        FT53_W::new(self, 21)
    }
}
#[doc = "EXTI falling trigger selection register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSR2_SPEC;
impl crate::RegisterSpec for FTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr2::R`](R) reader structure"]
impl crate::Readable for FTSR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure"]
impl crate::Writable for FTSR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2_SPEC {}
