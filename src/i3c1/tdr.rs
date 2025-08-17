#[doc = "Register `TDR` writer"]
pub type W = crate::W<TDR_SPEC>;
#[doc = "Field `TDB0` writer - 8-bit data to transmit on I3C bus."]
pub type TDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - 8-bit data to transmit on I3C bus."]
    #[inline(always)]
    pub fn TDB0(&mut self) -> TDB0_W<'_, TDR_SPEC> {
        TDB0_W::new(self, 0)
    }
}
#[doc = "I3C transmit data byte register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TDR_SPEC {}
