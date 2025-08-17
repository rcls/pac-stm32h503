#[doc = "Register `CSR52` reader"]
pub type R = crate::R<CSR52_SPEC>;
#[doc = "Register `CSR52` writer"]
pub type W = crate::W<CSR52_SPEC>;
#[doc = "Field `CS52` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS52_R = crate::FieldReader<u32>;
#[doc = "Field `CS52` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS52_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS52(&self) -> CS52_R {
        CS52_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS52(&mut self) -> CS52_W<'_, CSR52_SPEC> {
        CS52_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 52\n\nYou can [`read`](crate::Reg::read) this register and get [`csr52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR52_SPEC;
impl crate::RegisterSpec for CSR52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr52::R`](R) reader structure"]
impl crate::Readable for CSR52_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr52::W`](W) writer structure"]
impl crate::Writable for CSR52_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR52 to value 0"]
impl crate::Resettable for CSR52_SPEC {}
