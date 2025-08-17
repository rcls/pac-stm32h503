#[doc = "Register `CSR4` reader"]
pub type R = crate::R<CSR4_SPEC>;
#[doc = "Register `CSR4` writer"]
pub type W = crate::W<CSR4_SPEC>;
#[doc = "Field `CS4` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS4_R = crate::FieldReader<u32>;
#[doc = "Field `CS4` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS4(&self) -> CS4_R {
        CS4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS4(&mut self) -> CS4_W<'_, CSR4_SPEC> {
        CS4_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`csr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR4_SPEC;
impl crate::RegisterSpec for CSR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr4::R`](R) reader structure"]
impl crate::Readable for CSR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr4::W`](W) writer structure"]
impl crate::Writable for CSR4_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR4 to value 0"]
impl crate::Resettable for CSR4_SPEC {}
