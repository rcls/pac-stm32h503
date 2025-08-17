#[doc = "Register `CCMR3` reader"]
pub type R = crate::R<CCMR3_SPEC>;
#[doc = "Register `CCMR3` writer"]
pub type W = crate::W<CCMR3_SPEC>;
#[doc = "Field `OC5FE` reader - Output compare 5 fast enable"]
pub type OC5FE_R = crate::BitReader;
#[doc = "Field `OC5FE` writer - Output compare 5 fast enable"]
pub type OC5FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5PE` reader - Output compare 5 preload enable"]
pub type OC5PE_R = crate::BitReader;
#[doc = "Field `OC5PE` writer - Output compare 5 preload enable"]
pub type OC5PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M` reader - OC5M\\[2:0\\]: Output compare 5 mode"]
pub type OC5M_R = crate::FieldReader;
#[doc = "Field `OC5M` writer - OC5M\\[2:0\\]: Output compare 5 mode"]
pub type OC5M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC5CE` reader - Output compare 5 clear enable"]
pub type OC5CE_R = crate::BitReader;
#[doc = "Field `OC5CE` writer - Output compare 5 clear enable"]
pub type OC5CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6FE` reader - Output compare 6 fast enable"]
pub type OC6FE_R = crate::BitReader;
#[doc = "Field `OC6FE` writer - Output compare 6 fast enable"]
pub type OC6FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6PE` reader - Output compare 6 preload enable"]
pub type OC6PE_R = crate::BitReader;
#[doc = "Field `OC6PE` writer - Output compare 6 preload enable"]
pub type OC6PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M` reader - OC6M\\[2:0\\]: Output compare 6 mode"]
pub type OC6M_R = crate::FieldReader;
#[doc = "Field `OC6M` writer - OC6M\\[2:0\\]: Output compare 6 mode"]
pub type OC6M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC6CE` reader - Output compare 6 clear enable"]
pub type OC6CE_R = crate::BitReader;
#[doc = "Field `OC6CE` writer - Output compare 6 clear enable"]
pub type OC6CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M_1` reader - OC5M\\[3\\]"]
pub type OC5M_1_R = crate::BitReader;
#[doc = "Field `OC5M_1` writer - OC5M\\[3\\]"]
pub type OC5M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M_1` reader - OC6M\\[3\\]"]
pub type OC6M_1_R = crate::BitReader;
#[doc = "Field `OC6M_1` writer - OC6M\\[3\\]"]
pub type OC6M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn OC5FE(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn OC5PE(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC5M\\[2:0\\]: Output compare 5 mode"]
    #[inline(always)]
    pub fn OC5M(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn OC5CE(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn OC6FE(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn OC6PE(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC6M\\[2:0\\]: Output compare 6 mode"]
    #[inline(always)]
    pub fn OC6M(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn OC6CE(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OC5M\\[3\\]"]
    #[inline(always)]
    pub fn OC5M_1(&self) -> OC5M_1_R {
        OC5M_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - OC6M\\[3\\]"]
    #[inline(always)]
    pub fn OC6M_1(&self) -> OC6M_1_R {
        OC6M_1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn OC5FE(&mut self) -> OC5FE_W<'_, CCMR3_SPEC> {
        OC5FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn OC5PE(&mut self) -> OC5PE_W<'_, CCMR3_SPEC> {
        OC5PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC5M\\[2:0\\]: Output compare 5 mode"]
    #[inline(always)]
    pub fn OC5M(&mut self) -> OC5M_W<'_, CCMR3_SPEC> {
        OC5M_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 5 clear enable"]
    #[inline(always)]
    pub fn OC5CE(&mut self) -> OC5CE_W<'_, CCMR3_SPEC> {
        OC5CE_W::new(self, 7)
    }
    #[doc = "Bit 10 - Output compare 6 fast enable"]
    #[inline(always)]
    pub fn OC6FE(&mut self) -> OC6FE_W<'_, CCMR3_SPEC> {
        OC6FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 6 preload enable"]
    #[inline(always)]
    pub fn OC6PE(&mut self) -> OC6PE_W<'_, CCMR3_SPEC> {
        OC6PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - OC6M\\[2:0\\]: Output compare 6 mode"]
    #[inline(always)]
    pub fn OC6M(&mut self) -> OC6M_W<'_, CCMR3_SPEC> {
        OC6M_W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 6 clear enable"]
    #[inline(always)]
    pub fn OC6CE(&mut self) -> OC6CE_W<'_, CCMR3_SPEC> {
        OC6CE_W::new(self, 15)
    }
    #[doc = "Bit 16 - OC5M\\[3\\]"]
    #[inline(always)]
    pub fn OC5M_1(&mut self) -> OC5M_1_W<'_, CCMR3_SPEC> {
        OC5M_1_W::new(self, 16)
    }
    #[doc = "Bit 24 - OC6M\\[3\\]"]
    #[inline(always)]
    pub fn OC6M_1(&mut self) -> OC6M_1_W<'_, CCMR3_SPEC> {
        OC6M_1_W::new(self, 24)
    }
}
#[doc = "TIM1 capture/compare mode register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR3_SPEC;
impl crate::RegisterSpec for CCMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr3::R`](R) reader structure"]
impl crate::Readable for CCMR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr3::W`](W) writer structure"]
impl crate::Writable for CCMR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCMR3 to value 0"]
impl crate::Resettable for CCMR3_SPEC {}
