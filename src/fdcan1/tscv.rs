#[doc = "Register `TSCV` reader"]
pub type R = crate::R<TSCV_SPEC>;
#[doc = "Register `TSCV` writer"]
pub type W = crate::W<TSCV_SPEC>;
#[doc = "Field `TSC` reader - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx)."]
pub type TSC_R = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx)."]
pub type TSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx)."]
    #[inline(always)]
    pub fn TSC(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx)."]
    #[inline(always)]
    pub fn TSC(&mut self) -> TSC_W<'_, TSCV_SPEC> {
        TSC_W::new(self, 0)
    }
}
#[doc = "FDCAN timestamp counter value register\n\nYou can [`read`](crate::Reg::read) this register and get [`tscv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscv::R`](R) reader structure"]
impl crate::Readable for TSCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscv::W`](W) writer structure"]
impl crate::Writable for TSCV_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TSCV_SPEC {}
