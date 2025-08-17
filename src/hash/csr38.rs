#[doc = "Register `CSR38` reader"]
pub type R = crate::R<CSR38_SPEC>;
#[doc = "Register `CSR38` writer"]
pub type W = crate::W<CSR38_SPEC>;
#[doc = "Field `CS38` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS38_R = crate::FieldReader<u32>;
#[doc = "Field `CS38` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS38_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS38(&self) -> CS38_R {
        CS38_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS38(&mut self) -> CS38_W<'_, CSR38_SPEC> {
        CS38_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 38\n\nYou can [`read`](crate::Reg::read) this register and get [`csr38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR38_SPEC;
impl crate::RegisterSpec for CSR38_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr38::R`](R) reader structure"]
impl crate::Readable for CSR38_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr38::W`](W) writer structure"]
impl crate::Writable for CSR38_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR38 to value 0"]
impl crate::Resettable for CSR38_SPEC {}
