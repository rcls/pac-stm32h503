#[doc = "Register `SHSR1` reader"]
pub type R = crate::R<SHSR1_SPEC>;
#[doc = "Register `SHSR1` writer"]
pub type W = crate::W<SHSR1_SPEC>;
#[doc = "Field `TSAMPLE1` reader - DAC channel1 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWST1 of DAC_SR register is low, If BWST1 = 1, the write operation is ignored."]
pub type TSAMPLE1_R = crate::FieldReader<u16>;
#[doc = "Field `TSAMPLE1` writer - DAC channel1 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWST1 of DAC_SR register is low, If BWST1 = 1, the write operation is ignored."]
pub type TSAMPLE1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC channel1 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWST1 of DAC_SR register is low, If BWST1 = 1, the write operation is ignored."]
    #[inline(always)]
    pub fn TSAMPLE1(&self) -> TSAMPLE1_R {
        TSAMPLE1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC channel1 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWST1 of DAC_SR register is low, If BWST1 = 1, the write operation is ignored."]
    #[inline(always)]
    pub fn TSAMPLE1(&mut self) -> TSAMPLE1_W<'_, SHSR1_SPEC> {
        TSAMPLE1_W::new(self, 0)
    }
}
#[doc = "DAC channel1 sample and hold sample time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHSR1_SPEC;
impl crate::RegisterSpec for SHSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shsr1::R`](R) reader structure"]
impl crate::Readable for SHSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shsr1::W`](W) writer structure"]
impl crate::Writable for SHSR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SHSR1 to value 0"]
impl crate::Resettable for SHSR1_SPEC {}
