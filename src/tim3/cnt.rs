#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNT_SPEC>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CNT_SPEC>;
#[doc = "Field `CNT` reader - Counter value' Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\\[15:0\\]. The fractional part is not available."]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Counter value' Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\\[15:0\\]. The fractional part is not available."]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UIFCPY` reader - Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 Reserved If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
pub type UIFCPY_R = crate::BitReader;
#[doc = "Field `UIFCPY` writer - Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 Reserved If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Counter value' Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\\[15:0\\]. The fractional part is not available."]
    #[inline(always)]
    pub fn CNT(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 Reserved If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    pub fn UIFCPY(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value' Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\\[15:0\\]. The fractional part is not available."]
    #[inline(always)]
    pub fn CNT(&mut self) -> CNT_W<'_, CNT_SPEC> {
        CNT_W::new(self, 0)
    }
    #[doc = "Bit 31 - Value depends on IUFREMAP in TIMx_CR1. If UIFREMAP = 0 Reserved If UIFREMAP = 1 UIFCPY: UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register"]
    #[inline(always)]
    pub fn UIFCPY(&mut self) -> UIFCPY_W<'_, CNT_SPEC> {
        UIFCPY_W::new(self, 31)
    }
}
#[doc = "TIM3 counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {}
