#[doc = "Register `HTCR` reader"]
pub type R = crate::R<HTCR_SPEC>;
#[doc = "Register `HTCR` writer"]
pub type W = crate::W<HTCR_SPEC>;
#[doc = "Field `HTCFG` reader - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
pub type HTCFG_R = crate::FieldReader<u32>;
#[doc = "Field `HTCFG` writer - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
pub type HTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
    #[inline(always)]
    pub fn HTCFG(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration This configuration is used by RNG to configure the health tests. See Section 23.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written."]
    #[inline(always)]
    pub fn HTCFG(&mut self) -> HTCFG_W<'_, HTCR_SPEC> {
        HTCFG_W::new(self, 0)
    }
}
#[doc = "RNG health test control register\n\nYou can [`read`](crate::Reg::read) this register and get [`htcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTCR_SPEC;
impl crate::RegisterSpec for HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htcr::R`](R) reader structure"]
impl crate::Readable for HTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`htcr::W`](W) writer structure"]
impl crate::Writable for HTCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HTCR to value 0x72ac"]
impl crate::Resettable for HTCR_SPEC {
    const RESET_VALUE: u32 = 0x72ac;
}
