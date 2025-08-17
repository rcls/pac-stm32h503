#[doc = "Register `FPR2` reader"]
pub type R = crate::R<FPR2_SPEC>;
#[doc = "Register `FPR2` writer"]
pub type W = crate::W<FPR2_SPEC>;
#[doc = "configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF50_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF50_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF50` reader - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF50_R = crate::BitReader<FPIF50_A>;
impl FPIF50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF50_A {
        match self.bits {
            false => FPIF50_A::B_0x0,
            true => FPIF50_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF50_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF50_A::B_0x1
    }
}
#[doc = "Field `FPIF50` writer - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF50_W<'a, REG> = crate::BitWriter<'a, REG, FPIF50_A>;
impl<'a, REG> FPIF50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF50_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF50_A::B_0x1)
    }
}
#[doc = "configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF53_A {
    #[doc = "0: No falling edge trigger request occurred"]
    B_0x0 = 0,
    #[doc = "1: Falling edge trigger request occurred"]
    B_0x1 = 1,
}
impl From<FPIF53_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPIF53` reader - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF53_R = crate::BitReader<FPIF53_A>;
impl FPIF53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPIF53_A {
        match self.bits {
            false => FPIF53_A::B_0x0,
            true => FPIF53_A::B_0x1,
        }
    }
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FPIF53_A::B_0x0
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FPIF53_A::B_0x1
    }
}
#[doc = "Field `FPIF53` writer - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
pub type FPIF53_W<'a, REG> = crate::BitWriter<'a, REG, FPIF53_A>;
impl<'a, REG> FPIF53_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF53_A::B_0x0)
    }
    #[doc = "Falling edge trigger request occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF53_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 18 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF50(&self) -> FPIF50_R {
        FPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF53(&self) -> FPIF53_R {
        FPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF50(&mut self) -> FPIF50_W<'_, FPR2_SPEC> {
        FPIF50_W::new(self, 18)
    }
    #[doc = "Bit 21 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn FPIF53(&mut self) -> FPIF53_W<'_, FPR2_SPEC> {
        FPIF53_W::new(self, 21)
    }
}
#[doc = "EXTI falling edge pending register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPR2_SPEC;
impl crate::RegisterSpec for FPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpr2::R`](R) reader structure"]
impl crate::Readable for FPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpr2::W`](W) writer structure"]
impl crate::Writable for FPR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FPR2 to value 0"]
impl crate::Resettable for FPR2_SPEC {}
