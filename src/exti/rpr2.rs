#[doc = "Register `RPR2` reader"]
pub type R = crate::R<RPR2_SPEC>;
#[doc = "Register `RPR2` writer"]
pub type W = crate::W<RPR2_SPEC>;
#[doc = "configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF50_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF50_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF50` reader - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF50_R = crate::BitReader<RPIF50_A>;
impl RPIF50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF50_A {
        match self.bits {
            false => RPIF50_A::B_0x0,
            true => RPIF50_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF50_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF50_A::B_0x1
    }
}
#[doc = "Field `RPIF50` writer - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF50_W<'a, REG> = crate::BitWriter<'a, REG, RPIF50_A>;
impl<'a, REG> RPIF50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF50_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF50_A::B_0x1)
    }
}
#[doc = "configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF53_A {
    #[doc = "0: No rising edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Rising edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<RPIF53_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF53` reader - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF53_R = crate::BitReader<RPIF53_A>;
impl RPIF53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPIF53_A {
        match self.bits {
            false => RPIF53_A::B_0x0,
            true => RPIF53_A::B_0x1,
        }
    }
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RPIF53_A::B_0x0
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RPIF53_A::B_0x1
    }
}
#[doc = "Field `RPIF53` writer - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type RPIF53_W<'a, REG> = crate::BitWriter<'a, REG, RPIF53_A>;
impl<'a, REG> RPIF53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF53_A::B_0x0)
    }
    #[doc = "Rising edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF53_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 18 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF50(&self) -> RPIF50_R {
        RPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF53(&self) -> RPIF53_R {
        RPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF50(&mut self) -> RPIF50_W<'_, RPR2_SPEC> {
        RPIF50_W::new(self, 18)
    }
    #[doc = "Bit 21 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn RPIF53(&mut self) -> RPIF53_W<'_, RPR2_SPEC> {
        RPIF53_W::new(self, 21)
    }
}
#[doc = "EXTI rising edge pending register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR2_SPEC;
impl crate::RegisterSpec for RPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr2::R`](R) reader structure"]
impl crate::Readable for RPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rpr2::W`](W) writer structure"]
impl crate::Writable for RPR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2_SPEC {}
