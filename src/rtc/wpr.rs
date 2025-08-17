#[doc = "Register `WPR` writer"]
pub type W = crate::W<WPR_SPEC>;
#[doc = "Field `KEY` writer - Write protection key This byte is written by software. Reading this byte always returns 0x00. Refer to RTC register write protection for a description of how to unlock RTC register write protection."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write protection key This byte is written by software. Reading this byte always returns 0x00. Refer to RTC register write protection for a description of how to unlock RTC register write protection."]
    #[inline(always)]
    pub fn KEY(&mut self) -> KEY_W<'_, WPR_SPEC> {
        KEY_W::new(self, 0)
    }
}
#[doc = "RTC write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPR_SPEC;
impl crate::RegisterSpec for WPR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpr::W`](W) writer structure"]
impl crate::Writable for WPR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WPR_SPEC {}
