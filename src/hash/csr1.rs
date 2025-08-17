#[doc = "Register `CSR1` reader"]
pub type R = crate::R<CSR1_SPEC>;
#[doc = "Register `CSR1` writer"]
pub type W = crate::W<CSR1_SPEC>;
#[doc = "Field `CS1` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS1_R = crate::FieldReader<u32>;
#[doc = "Field `CS1` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS1(&self) -> CS1_R {
        CS1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS1(&mut self) -> CS1_W<'_, CSR1_SPEC> {
        CS1_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR1_SPEC;
impl crate::RegisterSpec for CSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr1::R`](R) reader structure"]
impl crate::Readable for CSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr1::W`](W) writer structure"]
impl crate::Writable for CSR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR1 to value 0"]
impl crate::Resettable for CSR1_SPEC {}
