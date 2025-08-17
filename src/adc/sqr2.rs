#[doc = "Register `SQR2` reader"]
pub type R = crate::R<SQR2_SPEC>;
#[doc = "Register `SQR2` writer"]
pub type W = crate::W<SQR2_SPEC>;
#[doc = "Field `SQ5` reader - 5th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 5th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ5_R = crate::FieldReader;
#[doc = "Field `SQ5` writer - 5th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 5th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ6` reader - 6th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 6th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ6_R = crate::FieldReader;
#[doc = "Field `SQ6` writer - 6th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 6th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ7` reader - 7th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 7th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ7_R = crate::FieldReader;
#[doc = "Field `SQ7` writer - 7th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 7th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ8` reader - 8th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 8th in the regular conversion sequence Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ8_R = crate::FieldReader;
#[doc = "Field `SQ8` writer - 8th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 8th in the regular conversion sequence Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ9` reader - 9th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 9th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ9_R = crate::FieldReader;
#[doc = "Field `SQ9` writer - 9th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 9th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type SQ9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 5th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 5th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ5(&self) -> SQ5_R {
        SQ5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - 6th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 6th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - 7th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 7th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - 8th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 8th in the regular conversion sequence Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 9th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 9th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 5th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 5th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ5(&mut self) -> SQ5_W<'_, SQR2_SPEC> {
        SQ5_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - 6th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 6th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ6(&mut self) -> SQ6_W<'_, SQR2_SPEC> {
        SQ6_W::new(self, 6)
    }
    #[doc = "Bits 12:16 - 7th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 7th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ7(&mut self) -> SQ7_W<'_, SQR2_SPEC> {
        SQ7_W::new(self, 12)
    }
    #[doc = "Bits 18:22 - 8th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 8th in the regular conversion sequence Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ8(&mut self) -> SQ8_W<'_, SQR2_SPEC> {
        SQ8_W::new(self, 18)
    }
    #[doc = "Bits 24:28 - 9th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 9th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn SQ9(&mut self) -> SQ9_W<'_, SQR2_SPEC> {
        SQ9_W::new(self, 24)
    }
}
#[doc = "ADC regular sequence register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr2::R`](R) reader structure"]
impl crate::Readable for SQR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr2::W`](W) writer structure"]
impl crate::Writable for SQR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2_SPEC {}
