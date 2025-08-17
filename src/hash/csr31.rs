#[doc = "Register `CSR31` reader"]
pub type R = crate::R<CSR31_SPEC>;
#[doc = "Register `CSR31` writer"]
pub type W = crate::W<CSR31_SPEC>;
#[doc = "Field `CS31` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS31_R = crate::FieldReader<u32>;
#[doc = "Field `CS31` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS31_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS31(&self) -> CS31_R {
        CS31_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS31(&mut self) -> CS31_W<'_, CSR31_SPEC> {
        CS31_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`csr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR31_SPEC;
impl crate::RegisterSpec for CSR31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr31::R`](R) reader structure"]
impl crate::Readable for CSR31_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr31::W`](W) writer structure"]
impl crate::Writable for CSR31_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR31 to value 0"]
impl crate::Resettable for CSR31_SPEC {}
