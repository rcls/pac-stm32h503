#[doc = "Register `CSR7` reader"]
pub type R = crate::R<CSR7_SPEC>;
#[doc = "Register `CSR7` writer"]
pub type W = crate::W<CSR7_SPEC>;
#[doc = "Field `CS7` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS7_R = crate::FieldReader<u32>;
#[doc = "Field `CS7` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS7(&self) -> CS7_R {
        CS7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS7(&mut self) -> CS7_W<'_, CSR7_SPEC> {
        CS7_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`csr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR7_SPEC;
impl crate::RegisterSpec for CSR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr7::R`](R) reader structure"]
impl crate::Readable for CSR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr7::W`](W) writer structure"]
impl crate::Writable for CSR7_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR7 to value 0"]
impl crate::Resettable for CSR7_SPEC {}
