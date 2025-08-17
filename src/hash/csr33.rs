#[doc = "Register `CSR33` reader"]
pub type R = crate::R<CSR33_SPEC>;
#[doc = "Register `CSR33` writer"]
pub type W = crate::W<CSR33_SPEC>;
#[doc = "Field `CS33` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS33_R = crate::FieldReader<u32>;
#[doc = "Field `CS33` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS33_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS33(&self) -> CS33_R {
        CS33_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS33(&mut self) -> CS33_W<'_, CSR33_SPEC> {
        CS33_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 33\n\nYou can [`read`](crate::Reg::read) this register and get [`csr33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR33_SPEC;
impl crate::RegisterSpec for CSR33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr33::R`](R) reader structure"]
impl crate::Readable for CSR33_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr33::W`](W) writer structure"]
impl crate::Writable for CSR33_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR33 to value 0"]
impl crate::Resettable for CSR33_SPEC {}
