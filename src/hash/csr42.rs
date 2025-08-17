#[doc = "Register `CSR42` reader"]
pub type R = crate::R<CSR42_SPEC>;
#[doc = "Register `CSR42` writer"]
pub type W = crate::W<CSR42_SPEC>;
#[doc = "Field `CS42` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS42_R = crate::FieldReader<u32>;
#[doc = "Field `CS42` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS42_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS42(&self) -> CS42_R {
        CS42_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS42(&mut self) -> CS42_W<'_, CSR42_SPEC> {
        CS42_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 42\n\nYou can [`read`](crate::Reg::read) this register and get [`csr42::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr42::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR42_SPEC;
impl crate::RegisterSpec for CSR42_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr42::R`](R) reader structure"]
impl crate::Readable for CSR42_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr42::W`](W) writer structure"]
impl crate::Writable for CSR42_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR42 to value 0"]
impl crate::Resettable for CSR42_SPEC {}
