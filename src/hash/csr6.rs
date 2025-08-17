#[doc = "Register `CSR6` reader"]
pub type R = crate::R<CSR6_SPEC>;
#[doc = "Register `CSR6` writer"]
pub type W = crate::W<CSR6_SPEC>;
#[doc = "Field `CS6` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS6_R = crate::FieldReader<u32>;
#[doc = "Field `CS6` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS6(&self) -> CS6_R {
        CS6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS6(&mut self) -> CS6_W<'_, CSR6_SPEC> {
        CS6_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`csr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR6_SPEC;
impl crate::RegisterSpec for CSR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr6::R`](R) reader structure"]
impl crate::Readable for CSR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr6::W`](W) writer structure"]
impl crate::Writable for CSR6_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR6 to value 0"]
impl crate::Resettable for CSR6_SPEC {}
