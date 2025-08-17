#[doc = "Register `ALRBBINR` reader"]
pub type R = crate::R<ALRBBINR_SPEC>;
#[doc = "Register `ALRBBINR` writer"]
pub type W = crate::W<ALRBBINR_SPEC>;
#[doc = "Field `SS` reader - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\] is the mirror of SS\\[14:0\\] in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR."]
pub type SS_R = crate::FieldReader<u32>;
#[doc = "Field `SS` writer - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\] is the mirror of SS\\[14:0\\] in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR."]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\] is the mirror of SS\\[14:0\\] in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR."]
    #[inline(always)]
    pub fn SS(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\\[14:0\\] is the mirror of SS\\[14:0\\] in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR."]
    #[inline(always)]
    pub fn SS(&mut self) -> SS_W<'_, ALRBBINR_SPEC> {
        SS_W::new(self, 0)
    }
}
#[doc = "RTC alarm B binary mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrbbinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrbbinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRBBINR_SPEC;
impl crate::RegisterSpec for ALRBBINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrbbinr::R`](R) reader structure"]
impl crate::Readable for ALRBBINR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrbbinr::W`](W) writer structure"]
impl crate::Writable for ALRBBINR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ALRBBINR to value 0"]
impl crate::Resettable for ALRBBINR_SPEC {}
