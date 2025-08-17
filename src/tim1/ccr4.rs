#[doc = "Register `CCR4` reader"]
pub type R = crate::R<CCR4_SPEC>;
#[doc = "Register `CCR4` writer"]
pub type W = crate::W<CCR4_SPEC>;
#[doc = "Field `CCR4` reader - Capture/compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on tim_oc4 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR4\\[15:0\\]. The CCR4\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR4\\[19:4\\]. The CCR4\\[3:0\\] bitfield contains the dithered part. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR4\\[15:0\\]. The CCR4\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR4\\[19:4\\]. The CCR4\\[3:0\\] bits are reset."]
pub type CCR4_R = crate::FieldReader<u32>;
#[doc = "Field `CCR4` writer - Capture/compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on tim_oc4 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR4\\[15:0\\]. The CCR4\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR4\\[19:4\\]. The CCR4\\[3:0\\] bitfield contains the dithered part. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR4\\[15:0\\]. The CCR4\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR4\\[19:4\\]. The CCR4\\[3:0\\] bits are reset."]
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Capture/compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on tim_oc4 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR4\\[15:0\\]. The CCR4\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR4\\[19:4\\]. The CCR4\\[3:0\\] bitfield contains the dithered part. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR4\\[15:0\\]. The CCR4\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR4\\[19:4\\]. The CCR4\\[3:0\\] bits are reset."]
    #[inline(always)]
    pub fn CCR4(&self) -> CCR4_R {
        CCR4_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on tim_oc4 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR4\\[15:0\\]. The CCR4\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR4\\[19:4\\]. The CCR4\\[3:0\\] bitfield contains the dithered part. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (tim_ic4). The TIMx_CCR4 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR4\\[15:0\\]. The CCR4\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR4\\[19:4\\]. The CCR4\\[3:0\\] bits are reset."]
    #[inline(always)]
    pub fn CCR4(&mut self) -> CCR4_W<'_, CCR4_SPEC> {
        CCR4_W::new(self, 0)
    }
}
#[doc = "TIM1 capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR4_SPEC;
impl crate::RegisterSpec for CCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr4::R`](R) reader structure"]
impl crate::Readable for CCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr4::W`](W) writer structure"]
impl crate::Writable for CCR4_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCR4 to value 0"]
impl crate::Resettable for CCR4_SPEC {}
