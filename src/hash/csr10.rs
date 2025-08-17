#[doc = "Register `CSR10` reader"]
pub type R = crate::R<CSR10_SPEC>;
#[doc = "Register `CSR10` writer"]
pub type W = crate::W<CSR10_SPEC>;
#[doc = "Field `CS10` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS10_R = crate::FieldReader<u32>;
#[doc = "Field `CS10` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS10(&self) -> CS10_R {
        CS10_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS10(&mut self) -> CS10_W<'_, CSR10_SPEC> {
        CS10_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`csr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR10_SPEC;
impl crate::RegisterSpec for CSR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr10::R`](R) reader structure"]
impl crate::Readable for CSR10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr10::W`](W) writer structure"]
impl crate::Writable for CSR10_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR10 to value 0"]
impl crate::Resettable for CSR10_SPEC {}
