#[doc = "Register `CSR27` reader"]
pub type R = crate::R<CSR27_SPEC>;
#[doc = "Register `CSR27` writer"]
pub type W = crate::W<CSR27_SPEC>;
#[doc = "Field `CS27` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS27_R = crate::FieldReader<u32>;
#[doc = "Field `CS27` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS27_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS27(&self) -> CS27_R {
        CS27_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS27(&mut self) -> CS27_W<'_, CSR27_SPEC> {
        CS27_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`csr27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR27_SPEC;
impl crate::RegisterSpec for CSR27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr27::R`](R) reader structure"]
impl crate::Readable for CSR27_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr27::W`](W) writer structure"]
impl crate::Writable for CSR27_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR27 to value 0"]
impl crate::Resettable for CSR27_SPEC {}
