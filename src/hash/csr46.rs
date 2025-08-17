#[doc = "Register `CSR46` reader"]
pub type R = crate::R<CSR46_SPEC>;
#[doc = "Register `CSR46` writer"]
pub type W = crate::W<CSR46_SPEC>;
#[doc = "Field `CS46` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS46_R = crate::FieldReader<u32>;
#[doc = "Field `CS46` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS46_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS46(&self) -> CS46_R {
        CS46_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS46(&mut self) -> CS46_W<'_, CSR46_SPEC> {
        CS46_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 46\n\nYou can [`read`](crate::Reg::read) this register and get [`csr46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR46_SPEC;
impl crate::RegisterSpec for CSR46_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr46::R`](R) reader structure"]
impl crate::Readable for CSR46_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr46::W`](W) writer structure"]
impl crate::Writable for CSR46_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR46 to value 0"]
impl crate::Resettable for CSR46_SPEC {}
