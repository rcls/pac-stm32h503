#[doc = "Register `CSR15` reader"]
pub type R = crate::R<CSR15_SPEC>;
#[doc = "Register `CSR15` writer"]
pub type W = crate::W<CSR15_SPEC>;
#[doc = "Field `CS15` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS15_R = crate::FieldReader<u32>;
#[doc = "Field `CS15` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS15_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS15(&self) -> CS15_R {
        CS15_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS15(&mut self) -> CS15_W<'_, CSR15_SPEC> {
        CS15_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`csr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR15_SPEC;
impl crate::RegisterSpec for CSR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr15::R`](R) reader structure"]
impl crate::Readable for CSR15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr15::W`](W) writer structure"]
impl crate::Writable for CSR15_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR15 to value 0"]
impl crate::Resettable for CSR15_SPEC {}
