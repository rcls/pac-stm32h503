#[doc = "Register `CSR16` reader"]
pub type R = crate::R<CSR16_SPEC>;
#[doc = "Register `CSR16` writer"]
pub type W = crate::W<CSR16_SPEC>;
#[doc = "Field `CS16` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS16_R = crate::FieldReader<u32>;
#[doc = "Field `CS16` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS16_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS16(&self) -> CS16_R {
        CS16_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS16(&mut self) -> CS16_W<'_, CSR16_SPEC> {
        CS16_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`csr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR16_SPEC;
impl crate::RegisterSpec for CSR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr16::R`](R) reader structure"]
impl crate::Readable for CSR16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr16::W`](W) writer structure"]
impl crate::Writable for CSR16_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR16 to value 0"]
impl crate::Resettable for CSR16_SPEC {}
