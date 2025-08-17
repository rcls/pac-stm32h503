#[doc = "Register `DHR12R2` reader"]
pub type R = crate::R<DHR12R2_SPEC>;
#[doc = "Register `DHR12R2` writer"]
pub type W = crate::W<DHR12R2_SPEC>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2."]
pub type DACC2DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2."]
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC2DHRB` reader - DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2 when the DAC operates in DMA Double data mode."]
pub type DACC2DHRB_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHRB` writer - DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2 when the DAC operates in DMA Double data mode."]
pub type DACC2DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn DACC2DHR(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2 when the DAC operates in DMA Double data mode."]
    #[inline(always)]
    pub fn DACC2DHRB(&self) -> DACC2DHRB_R {
        DACC2DHRB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn DACC2DHR(&mut self) -> DACC2DHR_W<'_, DHR12R2_SPEC> {
        DACC2DHR_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2 when the DAC operates in DMA Double data mode."]
    #[inline(always)]
    pub fn DACC2DHRB(&mut self) -> DACC2DHRB_W<'_, DHR12R2_SPEC> {
        DACC2DHRB_W::new(self, 16)
    }
}
#[doc = "DAC channel2 12-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR12R2_SPEC;
impl crate::RegisterSpec for DHR12R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12r2::R`](R) reader structure"]
impl crate::Readable for DHR12R2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dhr12r2::W`](W) writer structure"]
impl crate::Writable for DHR12R2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DHR12R2 to value 0"]
impl crate::Resettable for DHR12R2_SPEC {}
