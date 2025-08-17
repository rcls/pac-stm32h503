#[doc = "Register `DHR8RD` reader"]
pub type R = crate::R<DHR8RD_SPEC>;
#[doc = "Register `DHR8RD` writer"]
pub type W = crate::W<DHR8RD_SPEC>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
pub type DACC1DHR_R = crate::FieldReader;
#[doc = "Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
pub type DACC1DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
pub type DACC2DHR_R = crate::FieldReader;
#[doc = "Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    pub fn DACC1DHR(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn DACC2DHR(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    pub fn DACC1DHR(&mut self) -> DACC1DHR_W<'_, DHR8RD_SPEC> {
        DACC1DHR_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn DACC2DHR(&mut self) -> DACC2DHR_W<'_, DHR8RD_SPEC> {
        DACC2DHR_W::new(self, 8)
    }
}
#[doc = "Dual DAC 8-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR8RD_SPEC;
impl crate::RegisterSpec for DHR8RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr8rd::R`](R) reader structure"]
impl crate::Readable for DHR8RD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dhr8rd::W`](W) writer structure"]
impl crate::Writable for DHR8RD_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DHR8RD to value 0"]
impl crate::Resettable for DHR8RD_SPEC {}
