#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGR_SPEC>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGR_SPEC>;
#[doc = "PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSPRIV_A {
    #[doc = "0: Read and write to PWR functions can be done by privileged or unprivileged access."]
    B_0x0 = 0,
    #[doc = "1: Read and write to PWR functions can be done by privileged access only."]
    B_0x1 = 1,
}
impl From<NSPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: NSPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSPRIV` reader - PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access."]
pub type NSPRIV_R = crate::BitReader<NSPRIV_A>;
impl NSPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NSPRIV_A {
        match self.bits {
            false => NSPRIV_A::B_0x0,
            true => NSPRIV_A::B_0x1,
        }
    }
    #[doc = "Read and write to PWR functions can be done by privileged or unprivileged access."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NSPRIV_A::B_0x0
    }
    #[doc = "Read and write to PWR functions can be done by privileged access only."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NSPRIV_A::B_0x1
    }
}
#[doc = "Field `NSPRIV` writer - PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access."]
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG, NSPRIV_A>;
impl<'a, REG> NSPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read and write to PWR functions can be done by privileged or unprivileged access."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NSPRIV_A::B_0x0)
    }
    #[doc = "Read and write to PWR functions can be done by privileged access only."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NSPRIV_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 1 - PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access."]
    #[inline(always)]
    pub fn NSPRIV(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access."]
    #[inline(always)]
    pub fn NSPRIV(&mut self) -> NSPRIV_W<'_, PRIVCFGR_SPEC> {
        NSPRIV_W::new(self, 1)
    }
}
#[doc = "PWR privilege configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR_SPEC;
impl crate::RegisterSpec for PRIVCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PRIVCFGR to value 0"]
impl crate::Resettable for PRIVCFGR_SPEC {}
