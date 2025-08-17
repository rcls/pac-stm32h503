#[doc = "Register `CSR44` reader"]
pub type R = crate::R<CSR44_SPEC>;
#[doc = "Register `CSR44` writer"]
pub type W = crate::W<CSR44_SPEC>;
#[doc = "Field `CS44` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS44_R = crate::FieldReader<u32>;
#[doc = "Field `CS44` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS44_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS44(&self) -> CS44_R {
        CS44_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS44(&mut self) -> CS44_W<'_, CSR44_SPEC> {
        CS44_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 44\n\nYou can [`read`](crate::Reg::read) this register and get [`csr44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR44_SPEC;
impl crate::RegisterSpec for CSR44_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr44::R`](R) reader structure"]
impl crate::Readable for CSR44_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr44::W`](W) writer structure"]
impl crate::Writable for CSR44_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR44 to value 0"]
impl crate::Resettable for CSR44_SPEC {}
