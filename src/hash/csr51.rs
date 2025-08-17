#[doc = "Register `CSR51` reader"]
pub type R = crate::R<CSR51_SPEC>;
#[doc = "Register `CSR51` writer"]
pub type W = crate::W<CSR51_SPEC>;
#[doc = "Field `CS51` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS51_R = crate::FieldReader<u32>;
#[doc = "Field `CS51` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS51_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS51(&self) -> CS51_R {
        CS51_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS51(&mut self) -> CS51_W<'_, CSR51_SPEC> {
        CS51_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 51\n\nYou can [`read`](crate::Reg::read) this register and get [`csr51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR51_SPEC;
impl crate::RegisterSpec for CSR51_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr51::R`](R) reader structure"]
impl crate::Readable for CSR51_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr51::W`](W) writer structure"]
impl crate::Writable for CSR51_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR51 to value 0"]
impl crate::Resettable for CSR51_SPEC {}
