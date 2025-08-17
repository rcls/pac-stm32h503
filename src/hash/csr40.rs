#[doc = "Register `CSR40` reader"]
pub type R = crate::R<CSR40_SPEC>;
#[doc = "Register `CSR40` writer"]
pub type W = crate::W<CSR40_SPEC>;
#[doc = "Field `CS40` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS40_R = crate::FieldReader<u32>;
#[doc = "Field `CS40` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS40_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS40(&self) -> CS40_R {
        CS40_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS40(&mut self) -> CS40_W<'_, CSR40_SPEC> {
        CS40_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 40\n\nYou can [`read`](crate::Reg::read) this register and get [`csr40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR40_SPEC;
impl crate::RegisterSpec for CSR40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr40::R`](R) reader structure"]
impl crate::Readable for CSR40_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr40::W`](W) writer structure"]
impl crate::Writable for CSR40_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR40 to value 0"]
impl crate::Resettable for CSR40_SPEC {}
