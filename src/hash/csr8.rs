#[doc = "Register `CSR8` reader"]
pub type R = crate::R<CSR8_SPEC>;
#[doc = "Register `CSR8` writer"]
pub type W = crate::W<CSR8_SPEC>;
#[doc = "Field `CS8` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS8_R = crate::FieldReader<u32>;
#[doc = "Field `CS8` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS8(&self) -> CS8_R {
        CS8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS8(&mut self) -> CS8_W<'_, CSR8_SPEC> {
        CS8_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`csr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR8_SPEC;
impl crate::RegisterSpec for CSR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr8::R`](R) reader structure"]
impl crate::Readable for CSR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr8::W`](W) writer structure"]
impl crate::Writable for CSR8_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR8 to value 0"]
impl crate::Resettable for CSR8_SPEC {}
