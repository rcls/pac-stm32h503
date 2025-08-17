#[doc = "Register `CSR3` reader"]
pub type R = crate::R<CSR3_SPEC>;
#[doc = "Register `CSR3` writer"]
pub type W = crate::W<CSR3_SPEC>;
#[doc = "Field `CS3` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS3_R = crate::FieldReader<u32>;
#[doc = "Field `CS3` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS3(&self) -> CS3_R {
        CS3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS3(&mut self) -> CS3_W<'_, CSR3_SPEC> {
        CS3_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`csr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR3_SPEC;
impl crate::RegisterSpec for CSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr3::R`](R) reader structure"]
impl crate::Readable for CSR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr3::W`](W) writer structure"]
impl crate::Writable for CSR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR3 to value 0"]
impl crate::Resettable for CSR3_SPEC {}
