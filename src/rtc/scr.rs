#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `CALRAF` writer - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register."]
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRBF` writer - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register."]
pub type CALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUTF` writer - Clear wakeup timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register."]
pub type CWUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` writer - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF."]
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSOVF` writer - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITSF` writer - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register."]
pub type CITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSRUF` writer - Clear SSR underflow flag Writing '1' in this bit clears the SSRUF in the RTC_SR register."]
pub type CSSRUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register."]
    #[inline(always)]
    pub fn CALRAF(&mut self) -> CALRAF_W<'_, SCR_SPEC> {
        CALRAF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register."]
    #[inline(always)]
    pub fn CALRBF(&mut self) -> CALRBF_W<'_, SCR_SPEC> {
        CALRBF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register."]
    #[inline(always)]
    pub fn CWUTF(&mut self) -> CWUTF_W<'_, SCR_SPEC> {
        CWUTF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF."]
    #[inline(always)]
    pub fn CTSF(&mut self) -> CTSF_W<'_, SCR_SPEC> {
        CTSF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    pub fn CTSOVF(&mut self) -> CTSOVF_W<'_, SCR_SPEC> {
        CTSOVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register."]
    #[inline(always)]
    pub fn CITSF(&mut self) -> CITSF_W<'_, SCR_SPEC> {
        CITSF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear SSR underflow flag Writing '1' in this bit clears the SSRUF in the RTC_SR register."]
    #[inline(always)]
    pub fn CSSRUF(&mut self) -> CSSRUF_W<'_, SCR_SPEC> {
        CSSRUF_W::new(self, 6)
    }
}
#[doc = "RTC status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {}
