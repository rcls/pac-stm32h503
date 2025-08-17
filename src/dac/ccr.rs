#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `OTRIM1` reader - DAC channel1 offset trimming value"]
pub type OTRIM1_R = crate::FieldReader;
#[doc = "Field `OTRIM1` writer - DAC channel1 offset trimming value"]
pub type OTRIM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OTRIM2` reader - DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation."]
pub type OTRIM2_R = crate::FieldReader;
#[doc = "Field `OTRIM2` writer - DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation."]
pub type OTRIM2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DAC channel1 offset trimming value"]
    #[inline(always)]
    pub fn OTRIM1(&self) -> OTRIM1_R {
        OTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn OTRIM2(&self) -> OTRIM2_R {
        OTRIM2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DAC channel1 offset trimming value"]
    #[inline(always)]
    pub fn OTRIM1(&mut self) -> OTRIM1_W<'_, CCR_SPEC> {
        OTRIM1_W::new(self, 0)
    }
    #[doc = "Bits 16:20 - DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn OTRIM2(&mut self) -> OTRIM2_W<'_, CCR_SPEC> {
        OTRIM2_W::new(self, 16)
    }
}
#[doc = "DAC calibration control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {}
