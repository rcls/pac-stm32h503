#[doc = "Register `CSR48` reader"]
pub type R = crate::R<CSR48_SPEC>;
#[doc = "Register `CSR48` writer"]
pub type W = crate::W<CSR48_SPEC>;
#[doc = "Field `CS48` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS48_R = crate::FieldReader<u32>;
#[doc = "Field `CS48` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS48_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS48(&self) -> CS48_R {
        CS48_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS48(&mut self) -> CS48_W<'_, CSR48_SPEC> {
        CS48_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 48\n\nYou can [`read`](crate::Reg::read) this register and get [`csr48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR48_SPEC;
impl crate::RegisterSpec for CSR48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr48::R`](R) reader structure"]
impl crate::Readable for CSR48_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr48::W`](W) writer structure"]
impl crate::Writable for CSR48_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR48 to value 0"]
impl crate::Resettable for CSR48_SPEC {}
