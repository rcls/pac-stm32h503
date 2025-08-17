#[doc = "Register `CSR39` reader"]
pub type R = crate::R<CSR39_SPEC>;
#[doc = "Register `CSR39` writer"]
pub type W = crate::W<CSR39_SPEC>;
#[doc = "Field `CS39` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS39_R = crate::FieldReader<u32>;
#[doc = "Field `CS39` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS39_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS39(&self) -> CS39_R {
        CS39_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS39(&mut self) -> CS39_W<'_, CSR39_SPEC> {
        CS39_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 39\n\nYou can [`read`](crate::Reg::read) this register and get [`csr39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR39_SPEC;
impl crate::RegisterSpec for CSR39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr39::R`](R) reader structure"]
impl crate::Readable for CSR39_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr39::W`](W) writer structure"]
impl crate::Writable for CSR39_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR39 to value 0"]
impl crate::Resettable for CSR39_SPEC {}
