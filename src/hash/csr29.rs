#[doc = "Register `CSR29` reader"]
pub type R = crate::R<CSR29_SPEC>;
#[doc = "Register `CSR29` writer"]
pub type W = crate::W<CSR29_SPEC>;
#[doc = "Field `CS29` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS29_R = crate::FieldReader<u32>;
#[doc = "Field `CS29` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS29_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS29(&self) -> CS29_R {
        CS29_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS29(&mut self) -> CS29_W<'_, CSR29_SPEC> {
        CS29_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`csr29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR29_SPEC;
impl crate::RegisterSpec for CSR29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr29::R`](R) reader structure"]
impl crate::Readable for CSR29_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr29::W`](W) writer structure"]
impl crate::Writable for CSR29_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR29 to value 0"]
impl crate::Resettable for CSR29_SPEC {}
