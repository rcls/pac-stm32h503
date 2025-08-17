#[doc = "Register `ARR` reader"]
pub type R = crate::R<ARR_SPEC>;
#[doc = "Register `ARR` writer"]
pub type W = crate::W<ARR_SPEC>;
#[doc = "Field `ARR` reader - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 65.3.3: Time-base unit on page 4457 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\] bitfield contains the dithered part."]
pub type ARR_R = crate::FieldReader<u32>;
#[doc = "Field `ARR` writer - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 65.3.3: Time-base unit on page 4457 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\] bitfield contains the dithered part."]
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 65.3.3: Time-base unit on page 4457 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\] bitfield contains the dithered part."]
    #[inline(always)]
    pub fn ARR(&self) -> ARR_R {
        ARR_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the Section 65.3.3: Time-base unit on page 4457 for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null. Non-dithering mode (DITHEN = 0) The register holds the auto-reload value. Dithering mode (DITHEN = 1) The register holds the integer part in ARR\\[19:4\\]. The ARR\\[3:0\\] bitfield contains the dithered part."]
    #[inline(always)]
    pub fn ARR(&mut self) -> ARR_W<'_, ARR_SPEC> {
        ARR_W::new(self, 0)
    }
}
#[doc = "TIM1 auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arr::R`](R) reader structure"]
impl crate::Readable for ARR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arr::W`](W) writer structure"]
impl crate::Writable for ARR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ARR to value 0xffff"]
impl crate::Resettable for ARR_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
