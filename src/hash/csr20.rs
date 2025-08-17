#[doc = "Register `CSR20` reader"]
pub type R = crate::R<CSR20_SPEC>;
#[doc = "Register `CSR20` writer"]
pub type W = crate::W<CSR20_SPEC>;
#[doc = "Field `CS20` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS20_R = crate::FieldReader<u32>;
#[doc = "Field `CS20` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS20_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS20(&self) -> CS20_R {
        CS20_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS20(&mut self) -> CS20_W<'_, CSR20_SPEC> {
        CS20_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`csr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR20_SPEC;
impl crate::RegisterSpec for CSR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr20::R`](R) reader structure"]
impl crate::Readable for CSR20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr20::W`](W) writer structure"]
impl crate::Writable for CSR20_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR20 to value 0"]
impl crate::Resettable for CSR20_SPEC {}
