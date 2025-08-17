#[doc = "Register `DHR8R2` reader"]
pub type R = crate::R<DHR8R2_SPEC>;
#[doc = "Register `DHR8R2` writer"]
pub type W = crate::W<DHR8R2_SPEC>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
pub type DACC2DHR_R = crate::FieldReader;
#[doc = "Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DACC2DHRB` reader - DAC channel2 8-bit right-aligned data These bits are written by software. They specify 8-bit data for DAC channel2 when the DAC operates in Double data mode."]
pub type DACC2DHRB_R = crate::FieldReader;
#[doc = "Field `DACC2DHRB` writer - DAC channel2 8-bit right-aligned data These bits are written by software. They specify 8-bit data for DAC channel2 when the DAC operates in Double data mode."]
pub type DACC2DHRB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn DACC2DHR(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data These bits are written by software. They specify 8-bit data for DAC channel2 when the DAC operates in Double data mode."]
    #[inline(always)]
    pub fn DACC2DHRB(&self) -> DACC2DHRB_R {
        DACC2DHRB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn DACC2DHR(&mut self) -> DACC2DHR_W<'_, DHR8R2_SPEC> {
        DACC2DHR_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data These bits are written by software. They specify 8-bit data for DAC channel2 when the DAC operates in Double data mode."]
    #[inline(always)]
    pub fn DACC2DHRB(&mut self) -> DACC2DHRB_W<'_, DHR8R2_SPEC> {
        DACC2DHRB_W::new(self, 8)
    }
}
#[doc = "DAC channel2 8-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR8R2_SPEC;
impl crate::RegisterSpec for DHR8R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr8r2::R`](R) reader structure"]
impl crate::Readable for DHR8R2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dhr8r2::W`](W) writer structure"]
impl crate::Writable for DHR8R2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DHR8R2 to value 0"]
impl crate::Resettable for DHR8R2_SPEC {}
