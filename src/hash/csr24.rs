#[doc = "Register `CSR24` reader"]
pub type R = crate::R<CSR24_SPEC>;
#[doc = "Register `CSR24` writer"]
pub type W = crate::W<CSR24_SPEC>;
#[doc = "Field `CS24` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS24_R = crate::FieldReader<u32>;
#[doc = "Field `CS24` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS24_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS24(&self) -> CS24_R {
        CS24_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS24(&mut self) -> CS24_W<'_, CSR24_SPEC> {
        CS24_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`csr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR24_SPEC;
impl crate::RegisterSpec for CSR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr24::R`](R) reader structure"]
impl crate::Readable for CSR24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr24::W`](W) writer structure"]
impl crate::Writable for CSR24_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR24 to value 0"]
impl crate::Resettable for CSR24_SPEC {}
