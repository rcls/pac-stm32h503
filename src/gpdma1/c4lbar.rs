#[doc = "Register `C4LBAR` reader"]
pub type R = crate::R<C4LBAR_SPEC>;
#[doc = "Register `C4LBAR` writer"]
pub type W = crate::W<C4LBAR_SPEC>;
#[doc = "Field `LBA` reader - linked-list base address of GPDMA channel x"]
pub type LBA_R = crate::FieldReader<u16>;
#[doc = "Field `LBA` writer - linked-list base address of GPDMA channel x"]
pub type LBA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - linked-list base address of GPDMA channel x"]
    #[inline(always)]
    pub fn LBA(&self) -> LBA_R {
        LBA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - linked-list base address of GPDMA channel x"]
    #[inline(always)]
    pub fn LBA(&mut self) -> LBA_W<'_, C4LBAR_SPEC> {
        LBA_W::new(self, 16)
    }
}
#[doc = "GPDMA channel 4 linked-list base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4lbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4lbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4LBAR_SPEC;
impl crate::RegisterSpec for C4LBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c4lbar::R`](R) reader structure"]
impl crate::Readable for C4LBAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c4lbar::W`](W) writer structure"]
impl crate::Writable for C4LBAR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C4LBAR to value 0"]
impl crate::Resettable for C4LBAR_SPEC {}
