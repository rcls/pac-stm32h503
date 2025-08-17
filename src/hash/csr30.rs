#[doc = "Register `CSR30` reader"]
pub type R = crate::R<CSR30_SPEC>;
#[doc = "Register `CSR30` writer"]
pub type W = crate::W<CSR30_SPEC>;
#[doc = "Field `CS30` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS30_R = crate::FieldReader<u32>;
#[doc = "Field `CS30` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS30_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS30(&self) -> CS30_R {
        CS30_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS30(&mut self) -> CS30_W<'_, CSR30_SPEC> {
        CS30_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`csr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR30_SPEC;
impl crate::RegisterSpec for CSR30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr30::R`](R) reader structure"]
impl crate::Readable for CSR30_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr30::W`](W) writer structure"]
impl crate::Writable for CSR30_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR30 to value 0"]
impl crate::Resettable for CSR30_SPEC {}
