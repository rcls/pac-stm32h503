#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `SYNCOKC` reader - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register."]
pub type SYNCOKC_R = crate::BitReader;
#[doc = "Field `SYNCOKC` writer - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register."]
pub type SYNCOKC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCWARNC` reader - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register."]
pub type SYNCWARNC_R = crate::BitReader;
#[doc = "Field `SYNCWARNC` writer - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register."]
pub type SYNCWARNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRC` reader - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register."]
pub type ERRC_R = crate::BitReader;
#[doc = "Field `ERRC` writer - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register."]
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESYNCC` reader - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register."]
pub type ESYNCC_R = crate::BitReader;
#[doc = "Field `ESYNCC` writer - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register."]
pub type ESYNCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn SYNCOKC(&self) -> SYNCOKC_R {
        SYNCOKC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn SYNCWARNC(&self) -> SYNCWARNC_R {
        SYNCWARNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn ERRC(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn ESYNCC(&self) -> ESYNCC_R {
        ESYNCC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn SYNCOKC(&mut self) -> SYNCOKC_W<'_, ICR_SPEC> {
        SYNCOKC_W::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn SYNCWARNC(&mut self) -> SYNCWARNC_W<'_, ICR_SPEC> {
        SYNCWARNC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn ERRC(&mut self) -> ERRC_W<'_, ICR_SPEC> {
        ERRC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register."]
    #[inline(always)]
    pub fn ESYNCC(&mut self) -> ESYNCC_W<'_, ICR_SPEC> {
        ESYNCC_W::new(self, 3)
    }
}
#[doc = "CRS interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {}
