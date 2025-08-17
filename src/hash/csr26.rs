#[doc = "Register `CSR26` reader"]
pub type R = crate::R<CSR26_SPEC>;
#[doc = "Register `CSR26` writer"]
pub type W = crate::W<CSR26_SPEC>;
#[doc = "Field `CS26` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS26_R = crate::FieldReader<u32>;
#[doc = "Field `CS26` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS26_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS26(&self) -> CS26_R {
        CS26_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS26(&mut self) -> CS26_W<'_, CSR26_SPEC> {
        CS26_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`csr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR26_SPEC;
impl crate::RegisterSpec for CSR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr26::R`](R) reader structure"]
impl crate::Readable for CSR26_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr26::W`](W) writer structure"]
impl crate::Writable for CSR26_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR26 to value 0"]
impl crate::Resettable for CSR26_SPEC {}
