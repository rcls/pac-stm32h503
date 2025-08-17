#[doc = "Register `CSR18` reader"]
pub type R = crate::R<CSR18_SPEC>;
#[doc = "Register `CSR18` writer"]
pub type W = crate::W<CSR18_SPEC>;
#[doc = "Field `CS18` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS18_R = crate::FieldReader<u32>;
#[doc = "Field `CS18` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS18_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS18(&self) -> CS18_R {
        CS18_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS18(&mut self) -> CS18_W<'_, CSR18_SPEC> {
        CS18_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`csr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR18_SPEC;
impl crate::RegisterSpec for CSR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr18::R`](R) reader structure"]
impl crate::Readable for CSR18_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr18::W`](W) writer structure"]
impl crate::Writable for CSR18_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR18 to value 0"]
impl crate::Resettable for CSR18_SPEC {}
