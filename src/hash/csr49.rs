#[doc = "Register `CSR49` reader"]
pub type R = crate::R<CSR49_SPEC>;
#[doc = "Register `CSR49` writer"]
pub type W = crate::W<CSR49_SPEC>;
#[doc = "Field `CS49` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS49_R = crate::FieldReader<u32>;
#[doc = "Field `CS49` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS49_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS49(&self) -> CS49_R {
        CS49_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS49(&mut self) -> CS49_W<'_, CSR49_SPEC> {
        CS49_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 49\n\nYou can [`read`](crate::Reg::read) this register and get [`csr49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR49_SPEC;
impl crate::RegisterSpec for CSR49_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr49::R`](R) reader structure"]
impl crate::Readable for CSR49_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr49::W`](W) writer structure"]
impl crate::Writable for CSR49_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR49 to value 0"]
impl crate::Resettable for CSR49_SPEC {}
