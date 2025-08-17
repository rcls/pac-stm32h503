#[doc = "Register `CSR17` reader"]
pub type R = crate::R<CSR17_SPEC>;
#[doc = "Register `CSR17` writer"]
pub type W = crate::W<CSR17_SPEC>;
#[doc = "Field `CS17` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS17_R = crate::FieldReader<u32>;
#[doc = "Field `CS17` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS17_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS17(&self) -> CS17_R {
        CS17_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS17(&mut self) -> CS17_W<'_, CSR17_SPEC> {
        CS17_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`csr17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR17_SPEC;
impl crate::RegisterSpec for CSR17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr17::R`](R) reader structure"]
impl crate::Readable for CSR17_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr17::W`](W) writer structure"]
impl crate::Writable for CSR17_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR17 to value 0"]
impl crate::Resettable for CSR17_SPEC {}
