#[doc = "Register `CSR28` reader"]
pub type R = crate::R<CSR28_SPEC>;
#[doc = "Register `CSR28` writer"]
pub type W = crate::W<CSR28_SPEC>;
#[doc = "Field `CS28` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS28_R = crate::FieldReader<u32>;
#[doc = "Field `CS28` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS28_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS28(&self) -> CS28_R {
        CS28_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS28(&mut self) -> CS28_W<'_, CSR28_SPEC> {
        CS28_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`csr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR28_SPEC;
impl crate::RegisterSpec for CSR28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr28::R`](R) reader structure"]
impl crate::Readable for CSR28_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr28::W`](W) writer structure"]
impl crate::Writable for CSR28_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR28 to value 0"]
impl crate::Resettable for CSR28_SPEC {}
