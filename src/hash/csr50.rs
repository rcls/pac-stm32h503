#[doc = "Register `CSR50` reader"]
pub type R = crate::R<CSR50_SPEC>;
#[doc = "Register `CSR50` writer"]
pub type W = crate::W<CSR50_SPEC>;
#[doc = "Field `CS50` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS50_R = crate::FieldReader<u32>;
#[doc = "Field `CS50` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS50_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS50(&self) -> CS50_R {
        CS50_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS50(&mut self) -> CS50_W<'_, CSR50_SPEC> {
        CS50_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 50\n\nYou can [`read`](crate::Reg::read) this register and get [`csr50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR50_SPEC;
impl crate::RegisterSpec for CSR50_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr50::R`](R) reader structure"]
impl crate::Readable for CSR50_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr50::W`](W) writer structure"]
impl crate::Writable for CSR50_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR50 to value 0"]
impl crate::Resettable for CSR50_SPEC {}
