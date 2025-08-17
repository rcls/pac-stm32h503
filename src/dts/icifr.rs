#[doc = "Register `ICIFR` reader"]
pub type R = crate::R<ICIFR_SPEC>;
#[doc = "Register `ICIFR` writer"]
pub type W = crate::W<ICIFR_SPEC>;
#[doc = "Field `TS1_CITEF` reader - Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register."]
pub type TS1_CITEF_R = crate::BitReader;
#[doc = "Field `TS1_CITEF` writer - Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register."]
pub type TS1_CITEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CITLF` reader - Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register."]
pub type TS1_CITLF_R = crate::BitReader;
#[doc = "Field `TS1_CITLF` writer - Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register."]
pub type TS1_CITLF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CITHF` reader - Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register."]
pub type TS1_CITHF_R = crate::BitReader;
#[doc = "Field `TS1_CITHF` writer - Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register."]
pub type TS1_CITHF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CAITEF` reader - Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register."]
pub type TS1_CAITEF_R = crate::BitReader;
#[doc = "Field `TS1_CAITEF` writer - Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register."]
pub type TS1_CAITEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CAITLF` reader - Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register."]
pub type TS1_CAITLF_R = crate::BitReader;
#[doc = "Field `TS1_CAITLF` writer - Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register."]
pub type TS1_CAITLF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CAITHF` reader - Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register."]
pub type TS1_CAITHF_R = crate::BitReader;
#[doc = "Field `TS1_CAITHF` writer - Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register."]
pub type TS1_CAITHF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CITEF(&self) -> TS1_CITEF_R {
        TS1_CITEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CITLF(&self) -> TS1_CITLF_R {
        TS1_CITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CITHF(&self) -> TS1_CITHF_R {
        TS1_CITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CAITEF(&self) -> TS1_CAITEF_R {
        TS1_CAITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CAITLF(&self) -> TS1_CAITLF_R {
        TS1_CAITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CAITHF(&self) -> TS1_CAITHF_R {
        TS1_CAITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CITEF(&mut self) -> TS1_CITEF_W<'_, ICIFR_SPEC> {
        TS1_CITEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CITLF(&mut self) -> TS1_CITLF_W<'_, ICIFR_SPEC> {
        TS1_CITLF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CITHF(&mut self) -> TS1_CITHF_W<'_, ICIFR_SPEC> {
        TS1_CITHF_W::new(self, 2)
    }
    #[doc = "Bit 4 - Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CAITEF(&mut self) -> TS1_CAITEF_W<'_, ICIFR_SPEC> {
        TS1_CAITEF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CAITLF(&mut self) -> TS1_CAITLF_W<'_, ICIFR_SPEC> {
        TS1_CAITLF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register."]
    #[inline(always)]
    pub fn TS1_CAITHF(&mut self) -> TS1_CAITHF_W<'_, ICIFR_SPEC> {
        TS1_CAITHF_W::new(self, 6)
    }
}
#[doc = "Temperature sensor clear interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`icifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICIFR_SPEC;
impl crate::RegisterSpec for ICIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icifr::R`](R) reader structure"]
impl crate::Readable for ICIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icifr::W`](W) writer structure"]
impl crate::Writable for ICIFR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICIFR to value 0"]
impl crate::Resettable for ICIFR_SPEC {}
