#[doc = "Register `CSR2` reader"]
pub type R = crate::R<CSR2_SPEC>;
#[doc = "Register `CSR2` writer"]
pub type W = crate::W<CSR2_SPEC>;
#[doc = "Field `CS2` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS2_R = crate::FieldReader<u32>;
#[doc = "Field `CS2` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS2(&self) -> CS2_R {
        CS2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS2(&mut self) -> CS2_W<'_, CSR2_SPEC> {
        CS2_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR2_SPEC;
impl crate::RegisterSpec for CSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr2::R`](R) reader structure"]
impl crate::Readable for CSR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr2::W`](W) writer structure"]
impl crate::Writable for CSR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR2 to value 0"]
impl crate::Resettable for CSR2_SPEC {}
