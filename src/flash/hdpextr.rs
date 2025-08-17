#[doc = "Register `HDPEXTR` reader"]
pub type R = crate::R<HDPEXTR_SPEC>;
#[doc = "Register `HDPEXTR` writer"]
pub type W = crate::W<HDPEXTR_SPEC>;
#[doc = "Field `HDP1_EXT` reader - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
pub type HDP1_EXT_R = crate::FieldReader;
#[doc = "Field `HDP1_EXT` writer - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
pub type HDP1_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HDP2_EXT` reader - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
pub type HDP2_EXT_R = crate::FieldReader;
#[doc = "Field `HDP2_EXT` writer - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
pub type HDP2_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
    #[inline(always)]
    pub fn HDP1_EXT(&self) -> HDP1_EXT_R {
        HDP1_EXT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
    #[inline(always)]
    pub fn HDP2_EXT(&self) -> HDP2_EXT_R {
        HDP2_EXT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
    #[inline(always)]
    pub fn HDP1_EXT(&mut self) -> HDP1_EXT_W<'_, HDPEXTR_SPEC> {
        HDP1_EXT_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
    #[inline(always)]
    pub fn HDP2_EXT(&mut self) -> HDP2_EXT_W<'_, HDPEXTR_SPEC> {
        HDP2_EXT_W::new(self, 16)
    }
}
#[doc = "FLASH HDP extension register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdpextr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdpextr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDPEXTR_SPEC;
impl crate::RegisterSpec for HDPEXTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdpextr::R`](R) reader structure"]
impl crate::Readable for HDPEXTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hdpextr::W`](W) writer structure"]
impl crate::Writable for HDPEXTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HDPEXTR to value 0"]
impl crate::Resettable for HDPEXTR_SPEC {}
