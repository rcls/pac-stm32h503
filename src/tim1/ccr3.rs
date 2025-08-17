#[doc = "Register `CCR3` reader"]
pub type R = crate::R<CCR3_SPEC>;
#[doc = "Register `CCR3` writer"]
pub type W = crate::W<CCR3_SPEC>;
#[doc = "Field `CCR3` reader - Capture/compare value If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc3 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR3\\[15:0\\]. The CCR3\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR3\\[19:4\\]. The CCR3\\[3:0\\] bitfield contains the dithered part. If channel CC3 is configured as input: CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR3\\[15:0\\]. The CCR3\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR3\\[19:4\\]. The CCR3\\[3:0\\] bits are reset."]
pub type CCR3_R = crate::FieldReader<u32>;
#[doc = "Field `CCR3` writer - Capture/compare value If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc3 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR3\\[15:0\\]. The CCR3\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR3\\[19:4\\]. The CCR3\\[3:0\\] bitfield contains the dithered part. If channel CC3 is configured as input: CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR3\\[15:0\\]. The CCR3\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR3\\[19:4\\]. The CCR3\\[3:0\\] bits are reset."]
pub type CCR3_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Capture/compare value If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc3 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR3\\[15:0\\]. The CCR3\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR3\\[19:4\\]. The CCR3\\[3:0\\] bitfield contains the dithered part. If channel CC3 is configured as input: CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR3\\[15:0\\]. The CCR3\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR3\\[19:4\\]. The CCR3\\[3:0\\] bits are reset."]
    #[inline(always)]
    pub fn CCR3(&self) -> CCR3_R {
        CCR3_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/compare value If channel CC3 is configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC3PE). Else the preload value is copied in the active capture/compare 3 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc3 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR3\\[15:0\\]. The CCR3\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR3\\[19:4\\]. The CCR3\\[3:0\\] bitfield contains the dithered part. If channel CC3 is configured as input: CCR3 is the counter value transferred by the last input capture 3 event (tim_ic3). The TIMx_CCR3 register is read-only and cannot be programmed. Non-dithering mode (DITHEN = 0) The register holds the capture value in CCR3\\[15:0\\]. The CCR3\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the capture in CCR3\\[19:4\\]. The CCR3\\[3:0\\] bits are reset."]
    #[inline(always)]
    pub fn CCR3(&mut self) -> CCR3_W<'_, CCR3_SPEC> {
        CCR3_W::new(self, 0)
    }
}
#[doc = "TIM1 capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR3_SPEC;
impl crate::RegisterSpec for CCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3::R`](R) reader structure"]
impl crate::Readable for CCR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr3::W`](W) writer structure"]
impl crate::Writable for CCR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for CCR3_SPEC {}
