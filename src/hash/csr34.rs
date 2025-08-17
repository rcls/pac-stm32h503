#[doc = "Register `CSR34` reader"]
pub type R = crate::R<CSR34_SPEC>;
#[doc = "Register `CSR34` writer"]
pub type W = crate::W<CSR34_SPEC>;
#[doc = "Field `CS34` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS34_R = crate::FieldReader<u32>;
#[doc = "Field `CS34` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS34_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS34(&self) -> CS34_R {
        CS34_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS34(&mut self) -> CS34_W<'_, CSR34_SPEC> {
        CS34_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 34\n\nYou can [`read`](crate::Reg::read) this register and get [`csr34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR34_SPEC;
impl crate::RegisterSpec for CSR34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr34::R`](R) reader structure"]
impl crate::Readable for CSR34_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr34::W`](W) writer structure"]
impl crate::Writable for CSR34_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR34 to value 0"]
impl crate::Resettable for CSR34_SPEC {}
