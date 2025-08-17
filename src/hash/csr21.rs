#[doc = "Register `CSR21` reader"]
pub type R = crate::R<CSR21_SPEC>;
#[doc = "Register `CSR21` writer"]
pub type W = crate::W<CSR21_SPEC>;
#[doc = "Field `CS21` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS21_R = crate::FieldReader<u32>;
#[doc = "Field `CS21` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS21_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS21(&self) -> CS21_R {
        CS21_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS21(&mut self) -> CS21_W<'_, CSR21_SPEC> {
        CS21_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`csr21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR21_SPEC;
impl crate::RegisterSpec for CSR21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr21::R`](R) reader structure"]
impl crate::Readable for CSR21_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr21::W`](W) writer structure"]
impl crate::Writable for CSR21_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR21 to value 0"]
impl crate::Resettable for CSR21_SPEC {}
