#[doc = "Register `CSR32` reader"]
pub type R = crate::R<CSR32_SPEC>;
#[doc = "Register `CSR32` writer"]
pub type W = crate::W<CSR32_SPEC>;
#[doc = "Field `CS32` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS32_R = crate::FieldReader<u32>;
#[doc = "Field `CS32` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS32(&self) -> CS32_R {
        CS32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS32(&mut self) -> CS32_W<'_, CSR32_SPEC> {
        CS32_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 32\n\nYou can [`read`](crate::Reg::read) this register and get [`csr32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR32_SPEC;
impl crate::RegisterSpec for CSR32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr32::R`](R) reader structure"]
impl crate::Readable for CSR32_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr32::W`](W) writer structure"]
impl crate::Writable for CSR32_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR32 to value 0"]
impl crate::Resettable for CSR32_SPEC {}
