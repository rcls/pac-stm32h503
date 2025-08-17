#[doc = "Register `ATSEEDR` writer"]
pub type W = crate::W<ATSEEDR_SPEC>;
#[doc = "Field `SEED` writer - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
pub type SEED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
    #[inline(always)]
    pub fn SEED(&mut self) -> SEED_W<'_, ATSEEDR_SPEC> {
        SEED_W::new(self, 0)
    }
}
#[doc = "TAMP active tamper seed register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atseedr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATSEEDR_SPEC;
impl crate::RegisterSpec for ATSEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`atseedr::W`](W) writer structure"]
impl crate::Writable for ATSEEDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ATSEEDR to value 0"]
impl crate::Resettable for ATSEEDR_SPEC {}
