#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "clear busy end flag Set by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBSYENDF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clears BSYENDF flag in ICACHE_SR."]
    B_0x1 = 1,
}
impl From<CBSYENDF_A> for bool {
    #[inline(always)]
    fn from(variant: CBSYENDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBSYENDF` writer - clear busy end flag Set by software."]
pub type CBSYENDF_W<'a, REG> = crate::BitWriter<'a, REG, CBSYENDF_A>;
impl<'a, REG> CBSYENDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CBSYENDF_A::B_0x0)
    }
    #[doc = "clears BSYENDF flag in ICACHE_SR."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CBSYENDF_A::B_0x1)
    }
}
#[doc = "clear cache error flag Set by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERRF_A {
    #[doc = "0: no effect"]
    B_0x0 = 0,
    #[doc = "1: clears ERRF flag in ICACHE_SR"]
    B_0x1 = 1,
}
impl From<CERRF_A> for bool {
    #[inline(always)]
    fn from(variant: CERRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERRF` writer - clear cache error flag Set by software."]
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG, CERRF_A>;
impl<'a, REG> CERRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CERRF_A::B_0x0)
    }
    #[doc = "clears ERRF flag in ICACHE_SR"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CERRF_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 1 - clear busy end flag Set by software."]
    #[inline(always)]
    pub fn CBSYENDF(&mut self) -> CBSYENDF_W<'_, FCR_SPEC> {
        CBSYENDF_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear cache error flag Set by software."]
    #[inline(always)]
    pub fn CERRF(&mut self) -> CERRF_W<'_, FCR_SPEC> {
        CERRF_W::new(self, 2)
    }
}
#[doc = "ICACHE flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {}
