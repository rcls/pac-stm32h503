#[doc = "Register `ALRABINR` reader"]
pub type R = crate::R<ALRABINR_SPEC>;
#[doc = "Register `ALRABINR` writer"]
pub type W = crate::W<ALRABINR_SPEC>;
#[doc = "Field `SS` reader - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\] is the mirror of SS\\[14:0\\] in the RTC_ALRMASSRR, and so can also be read or written through RTC_ALRMASSR."]
pub type SS_R = crate::FieldReader<u32>;
#[doc = "Field `SS` writer - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\] is the mirror of SS\\[14:0\\] in the RTC_ALRMASSRR, and so can also be read or written through RTC_ALRMASSR."]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\] is the mirror of SS\\[14:0\\] in the RTC_ALRMASSRR, and so can also be read or written through RTC_ALRMASSR."]
    #[inline(always)]
    pub fn SS(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\] is the mirror of SS\\[14:0\\] in the RTC_ALRMASSRR, and so can also be read or written through RTC_ALRMASSR."]
    #[inline(always)]
    pub fn SS(&mut self) -> SS_W<'_, ALRABINR_SPEC> {
        SS_W::new(self, 0)
    }
}
#[doc = "RTC alarm A binary mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrabinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrabinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRABINR_SPEC;
impl crate::RegisterSpec for ALRABINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrabinr::R`](R) reader structure"]
impl crate::Readable for ALRABINR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrabinr::W`](W) writer structure"]
impl crate::Writable for ALRABINR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ALRABINR to value 0"]
impl crate::Resettable for ALRABINR_SPEC {}
