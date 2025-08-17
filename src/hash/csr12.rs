#[doc = "Register `CSR12` reader"]
pub type R = crate::R<CSR12_SPEC>;
#[doc = "Register `CSR12` writer"]
pub type W = crate::W<CSR12_SPEC>;
#[doc = "Field `CS12` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS12_R = crate::FieldReader<u32>;
#[doc = "Field `CS12` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS12_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS12(&self) -> CS12_R {
        CS12_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS12(&mut self) -> CS12_W<'_, CSR12_SPEC> {
        CS12_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`csr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR12_SPEC;
impl crate::RegisterSpec for CSR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr12::R`](R) reader structure"]
impl crate::Readable for CSR12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr12::W`](W) writer structure"]
impl crate::Writable for CSR12_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR12 to value 0"]
impl crate::Resettable for CSR12_SPEC {}
