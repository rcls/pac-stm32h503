#[doc = "Register `CSR53` reader"]
pub type R = crate::R<CSR53_SPEC>;
#[doc = "Register `CSR53` writer"]
pub type W = crate::W<CSR53_SPEC>;
#[doc = "Field `CS53` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS53_R = crate::FieldReader<u32>;
#[doc = "Field `CS53` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS53_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS53(&self) -> CS53_R {
        CS53_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS53(&mut self) -> CS53_W<'_, CSR53_SPEC> {
        CS53_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 53\n\nYou can [`read`](crate::Reg::read) this register and get [`csr53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR53_SPEC;
impl crate::RegisterSpec for CSR53_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr53::R`](R) reader structure"]
impl crate::Readable for CSR53_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr53::W`](W) writer structure"]
impl crate::Writable for CSR53_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR53 to value 0"]
impl crate::Resettable for CSR53_SPEC {}
