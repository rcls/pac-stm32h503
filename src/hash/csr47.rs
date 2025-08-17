#[doc = "Register `CSR47` reader"]
pub type R = crate::R<CSR47_SPEC>;
#[doc = "Register `CSR47` writer"]
pub type W = crate::W<CSR47_SPEC>;
#[doc = "Field `CS47` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS47_R = crate::FieldReader<u32>;
#[doc = "Field `CS47` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS47_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS47(&self) -> CS47_R {
        CS47_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS47(&mut self) -> CS47_W<'_, CSR47_SPEC> {
        CS47_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 47\n\nYou can [`read`](crate::Reg::read) this register and get [`csr47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR47_SPEC;
impl crate::RegisterSpec for CSR47_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr47::R`](R) reader structure"]
impl crate::Readable for CSR47_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr47::W`](W) writer structure"]
impl crate::Writable for CSR47_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR47 to value 0"]
impl crate::Resettable for CSR47_SPEC {}
