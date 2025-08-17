#[doc = "Register `CCR6` reader"]
pub type R = crate::R<CCR6_SPEC>;
#[doc = "Register `CCR6` writer"]
pub type W = crate::W<CCR6_SPEC>;
#[doc = "Field `CCR6` reader - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\\[15:0\\]. The CCR6\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\\[19:4\\]. The CCR6\\[3:0\\] bitfield contains the dithered part."]
pub type CCR6_R = crate::FieldReader<u32>;
#[doc = "Field `CCR6` writer - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\\[15:0\\]. The CCR6\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\\[19:4\\]. The CCR6\\[3:0\\] bitfield contains the dithered part."]
pub type CCR6_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\\[15:0\\]. The CCR6\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\\[19:4\\]. The CCR6\\[3:0\\] bitfield contains the dithered part."]
    #[inline(always)]
    pub fn CCR6(&self) -> CCR6_R {
        CCR6_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc6 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR6\\[15:0\\]. The CCR6\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR6\\[19:4\\]. The CCR6\\[3:0\\] bitfield contains the dithered part."]
    #[inline(always)]
    pub fn CCR6(&mut self) -> CCR6_W<'_, CCR6_SPEC> {
        CCR6_W::new(self, 0)
    }
}
#[doc = "TIM1 capture/compare register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR6_SPEC;
impl crate::RegisterSpec for CCR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr6::R`](R) reader structure"]
impl crate::Readable for CCR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr6::W`](W) writer structure"]
impl crate::Writable for CCR6_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCR6 to value 0"]
impl crate::Resettable for CCR6_SPEC {}
