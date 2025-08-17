#[doc = "Register `CSR14` reader"]
pub type R = crate::R<CSR14_SPEC>;
#[doc = "Register `CSR14` writer"]
pub type W = crate::W<CSR14_SPEC>;
#[doc = "Field `CS14` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS14_R = crate::FieldReader<u32>;
#[doc = "Field `CS14` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS14_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS14(&self) -> CS14_R {
        CS14_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS14(&mut self) -> CS14_W<'_, CSR14_SPEC> {
        CS14_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`csr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR14_SPEC;
impl crate::RegisterSpec for CSR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr14::R`](R) reader structure"]
impl crate::Readable for CSR14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr14::W`](W) writer structure"]
impl crate::Writable for CSR14_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR14 to value 0"]
impl crate::Resettable for CSR14_SPEC {}
