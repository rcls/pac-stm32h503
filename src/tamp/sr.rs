#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `TAMP1F` reader - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input."]
pub type TAMP1F_R = crate::BitReader;
#[doc = "Field `TAMP2F` reader - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input."]
pub type TAMP2F_R = crate::BitReader;
#[doc = "Field `ITAMP1F` reader - Internal tamper 1 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 1."]
pub type ITAMP1F_R = crate::BitReader;
#[doc = "Field `ITAMP2F` reader - Internal tamper 2 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 2."]
pub type ITAMP2F_R = crate::BitReader;
#[doc = "Field `ITAMP3F` reader - Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
pub type ITAMP3F_R = crate::BitReader;
#[doc = "Field `ITAMP4F` reader - Internal tamper 4 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
pub type ITAMP4F_R = crate::BitReader;
#[doc = "Field `ITAMP5F` reader - Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
pub type ITAMP5F_R = crate::BitReader;
#[doc = "Field `ITAMP6F` reader - Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
pub type ITAMP6F_R = crate::BitReader;
#[doc = "Field `ITAMP7F` reader - Internal tamper 7 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 7."]
pub type ITAMP7F_R = crate::BitReader;
#[doc = "Field `ITAMP8F` reader - Internal tamper 8 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 8."]
pub type ITAMP8F_R = crate::BitReader;
#[doc = "Field `ITAMP9F` reader - Internal tamper 9 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 9."]
pub type ITAMP9F_R = crate::BitReader;
#[doc = "Field `ITAMP11F` reader - Internal tamper 11 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 11."]
pub type ITAMP11F_R = crate::BitReader;
#[doc = "Field `ITAMP12F` reader - Internal tamper 12 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 12."]
pub type ITAMP12F_R = crate::BitReader;
#[doc = "Field `ITAMP13F` reader - Internal tamper 13 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 13."]
pub type ITAMP13F_R = crate::BitReader;
#[doc = "Field `ITAMP15F` reader - Internal tamper 15 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 15."]
pub type ITAMP15F_R = crate::BitReader;
#[doc = "Field `ITAMP15F` writer - Internal tamper 15 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 15."]
pub type ITAMP15F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input."]
    #[inline(always)]
    pub fn TAMP1F(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input."]
    #[inline(always)]
    pub fn TAMP2F(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal tamper 1 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 1."]
    #[inline(always)]
    pub fn ITAMP1F(&self) -> ITAMP1F_R {
        ITAMP1F_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal tamper 2 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 2."]
    #[inline(always)]
    pub fn ITAMP2F(&self) -> ITAMP2F_R {
        ITAMP2F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
    #[inline(always)]
    pub fn ITAMP3F(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
    #[inline(always)]
    pub fn ITAMP4F(&self) -> ITAMP4F_R {
        ITAMP4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
    #[inline(always)]
    pub fn ITAMP5F(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
    #[inline(always)]
    pub fn ITAMP6F(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Internal tamper 7 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 7."]
    #[inline(always)]
    pub fn ITAMP7F(&self) -> ITAMP7F_R {
        ITAMP7F_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal tamper 8 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 8."]
    #[inline(always)]
    pub fn ITAMP8F(&self) -> ITAMP8F_R {
        ITAMP8F_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Internal tamper 9 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 9."]
    #[inline(always)]
    pub fn ITAMP9F(&self) -> ITAMP9F_R {
        ITAMP9F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Internal tamper 11 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 11."]
    #[inline(always)]
    pub fn ITAMP11F(&self) -> ITAMP11F_R {
        ITAMP11F_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Internal tamper 12 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 12."]
    #[inline(always)]
    pub fn ITAMP12F(&self) -> ITAMP12F_R {
        ITAMP12F_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Internal tamper 13 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 13."]
    #[inline(always)]
    pub fn ITAMP13F(&self) -> ITAMP13F_R {
        ITAMP13F_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Internal tamper 15 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 15."]
    #[inline(always)]
    pub fn ITAMP15F(&self) -> ITAMP15F_R {
        ITAMP15F_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Internal tamper 15 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 15."]
    #[inline(always)]
    pub fn ITAMP15F(&mut self) -> ITAMP15F_W<'_, SR_SPEC> {
        ITAMP15F_W::new(self, 30)
    }
}
#[doc = "TAMP status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {}
