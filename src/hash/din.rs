#[doc = "Register `DIN` writer"]
pub type W = crate::W<DIN_SPEC>;
#[doc = "Field `DATAIN` writer - Data input Writing this register pushes the current register content into the FIFO, and the register takes the new value presented on the AHB bus. Reading this register returns zeros."]
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data input Writing this register pushes the current register content into the FIFO, and the register takes the new value presented on the AHB bus. Reading this register returns zeros."]
    #[inline(always)]
    pub fn DATAIN(&mut self) -> DATAIN_W<'_, DIN_SPEC> {
        DATAIN_W::new(self, 0)
    }
}
#[doc = "HASH data input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIN_SPEC;
impl crate::RegisterSpec for DIN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`din::W`](W) writer structure"]
impl crate::Writable for DIN_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DIN to value 0"]
impl crate::Resettable for DIN_SPEC {}
