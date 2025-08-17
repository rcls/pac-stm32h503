#[doc = "Register `CSR22` reader"]
pub type R = crate::R<CSR22_SPEC>;
#[doc = "Register `CSR22` writer"]
pub type W = crate::W<CSR22_SPEC>;
#[doc = "Field `CS22` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS22_R = crate::FieldReader<u32>;
#[doc = "Field `CS22` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS22_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS22(&self) -> CS22_R {
        CS22_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS22(&mut self) -> CS22_W<'_, CSR22_SPEC> {
        CS22_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`csr22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR22_SPEC;
impl crate::RegisterSpec for CSR22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr22::R`](R) reader structure"]
impl crate::Readable for CSR22_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr22::W`](W) writer structure"]
impl crate::Writable for CSR22_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR22 to value 0"]
impl crate::Resettable for CSR22_SPEC {}
