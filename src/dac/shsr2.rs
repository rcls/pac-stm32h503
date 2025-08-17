#[doc = "Register `SHSR2` reader"]
pub type R = crate::R<SHSR2_SPEC>;
#[doc = "Register `SHSR2` writer"]
pub type W = crate::W<SHSR2_SPEC>;
#[doc = "Field `TSAMPLE2` reader - DAC channel2 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWST2 of DAC_SR register is low, if BWST2 = 1, the write operation is ignored."]
pub type TSAMPLE2_R = crate::FieldReader<u16>;
#[doc = "Field `TSAMPLE2` writer - DAC channel2 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWST2 of DAC_SR register is low, if BWST2 = 1, the write operation is ignored."]
pub type TSAMPLE2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC channel2 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWST2 of DAC_SR register is low, if BWST2 = 1, the write operation is ignored."]
    #[inline(always)]
    pub fn TSAMPLE2(&self) -> TSAMPLE2_R {
        TSAMPLE2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC channel2 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWST2 of DAC_SR register is low, if BWST2 = 1, the write operation is ignored."]
    #[inline(always)]
    pub fn TSAMPLE2(&mut self) -> TSAMPLE2_W<'_, SHSR2_SPEC> {
        TSAMPLE2_W::new(self, 0)
    }
}
#[doc = "DAC channel2 sample and hold sample time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHSR2_SPEC;
impl crate::RegisterSpec for SHSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shsr2::R`](R) reader structure"]
impl crate::Readable for SHSR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shsr2::W`](W) writer structure"]
impl crate::Writable for SHSR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SHSR2 to value 0"]
impl crate::Resettable for SHSR2_SPEC {}
