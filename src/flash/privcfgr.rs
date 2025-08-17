#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGR_SPEC>;
#[doc = "privilege attribute for non secure registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSPRIV_A {
    #[doc = "0: access to non secure registers is always granted"]
    B_0x0 = 0,
    #[doc = "1: access to non secure registers is denied in case of non privileged access."]
    B_0x1 = 1,
}
impl From<NSPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: NSPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSPRIV` writer - privilege attribute for non secure registers"]
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG, NSPRIV_A>;
impl<'a, REG> NSPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "access to non secure registers is always granted"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NSPRIV_A::B_0x0)
    }
    #[doc = "access to non secure registers is denied in case of non privileged access."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NSPRIV_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 1 - privilege attribute for non secure registers"]
    #[inline(always)]
    pub fn NSPRIV(&mut self) -> NSPRIV_W<'_, PRIVCFGR_SPEC> {
        NSPRIV_W::new(self, 1)
    }
}
#[doc = "FLASH privilege configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR_SPEC;
impl crate::RegisterSpec for PRIVCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PRIVCFGR to value 0"]
impl crate::Resettable for PRIVCFGR_SPEC {}
