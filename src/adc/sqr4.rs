#[doc = "Register `SQR4` reader"]
pub type R = crate::R<SQR4_SPEC>;
#[doc = "Register `SQR4` writer"]
pub type W = crate::W<SQR4_SPEC>;
#[doc = "Field `SQ15` reader - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ15_R = crate::FieldReader;
#[doc = "Field `SQ15` writer - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ15_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ16` reader - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ16_R = crate::FieldReader;
#[doc = "Field `SQ16` writer - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ16_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ15(&mut self) -> SQ15_W<'_, SQR4_SPEC> {
        SQ15_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ16(&mut self) -> SQ16_W<'_, SQR4_SPEC> {
        SQ16_W::new(self, 6)
    }
}
#[doc = "ADC regular sequence register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR4_SPEC;
impl crate::RegisterSpec for SQR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr4::R`](R) reader structure"]
impl crate::Readable for SQR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr4::W`](W) writer structure"]
impl crate::Writable for SQR4_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SQR4 to value 0"]
impl crate::Resettable for SQR4_SPEC {}
