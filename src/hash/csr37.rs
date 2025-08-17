#[doc = "Register `CSR37` reader"]
pub type R = crate::R<CSR37_SPEC>;
#[doc = "Register `CSR37` writer"]
pub type W = crate::W<CSR37_SPEC>;
#[doc = "Field `CS37` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS37_R = crate::FieldReader<u32>;
#[doc = "Field `CS37` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS37_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS37(&self) -> CS37_R {
        CS37_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS37(&mut self) -> CS37_W<'_, CSR37_SPEC> {
        CS37_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 37\n\nYou can [`read`](crate::Reg::read) this register and get [`csr37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR37_SPEC;
impl crate::RegisterSpec for CSR37_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr37::R`](R) reader structure"]
impl crate::Readable for CSR37_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr37::W`](W) writer structure"]
impl crate::Writable for CSR37_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR37 to value 0"]
impl crate::Resettable for CSR37_SPEC {}
