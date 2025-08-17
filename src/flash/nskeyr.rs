#[doc = "Register `NSKEYR` writer"]
pub type W = crate::W<NSKEYR_SPEC>;
#[doc = "Field `NSKEY` writer - Non-volatile memory configuration access unlock key"]
pub type NSKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Non-volatile memory configuration access unlock key"]
    #[inline(always)]
    pub fn NSKEY(&mut self) -> NSKEY_W<'_, NSKEYR_SPEC> {
        NSKEY_W::new(self, 0)
    }
}
#[doc = "FLASH key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nskeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSKEYR_SPEC;
impl crate::RegisterSpec for NSKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nskeyr::W`](W) writer structure"]
impl crate::Writable for NSKEYR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets NSKEYR to value 0"]
impl crate::Resettable for NSKEYR_SPEC {}
