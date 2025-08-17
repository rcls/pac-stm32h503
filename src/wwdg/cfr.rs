#[doc = "Register `CFR` reader"]
pub type R = crate::R<CFR_SPEC>;
#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFR_SPEC>;
#[doc = "Field `W` reader - 7-bit window value These bits contain the window value to be compared with the down-counter."]
pub type W_R = crate::FieldReader;
#[doc = "Field `W` writer - 7-bit window value These bits contain the window value to be compared with the down-counter."]
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EWI` reader - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
pub type EWI_R = crate::BitReader;
#[doc = "Field `EWI` writer - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer base The timebase of the prescaler can be modified as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDGTB_A {
    #[doc = "0: CK counter clock (PCLK div 4096) div 1"]
    B_0x0 = 0,
    #[doc = "1: CK counter clock (PCLK div 4096) div 2"]
    B_0x1 = 1,
    #[doc = "2: CK counter clock (PCLK div 4096) div 4"]
    B_0x2 = 2,
    #[doc = "3: CK counter clock (PCLK div 4096) div 8"]
    B_0x3 = 3,
    #[doc = "4: CK counter clock (PCLK div 4096) div 16"]
    B_0x4 = 4,
    #[doc = "5: CK counter clock (PCLK div 4096) div 32"]
    B_0x5 = 5,
    #[doc = "6: CK counter clock (PCLK div 4096) div 64"]
    B_0x6 = 6,
    #[doc = "7: CK counter clock (PCLK div 4096) div 128"]
    B_0x7 = 7,
}
impl From<WDGTB_A> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDGTB_A {
    type Ux = u8;
}
impl crate::IsEnum for WDGTB_A {}
#[doc = "Field `WDGTB` reader - Timer base The timebase of the prescaler can be modified as follows:"]
pub type WDGTB_R = crate::FieldReader<WDGTB_A>;
impl WDGTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDGTB_A {
        match self.bits {
            0 => WDGTB_A::B_0x0,
            1 => WDGTB_A::B_0x1,
            2 => WDGTB_A::B_0x2,
            3 => WDGTB_A::B_0x3,
            4 => WDGTB_A::B_0x4,
            5 => WDGTB_A::B_0x5,
            6 => WDGTB_A::B_0x6,
            7 => WDGTB_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "CK counter clock (PCLK div 4096) div 1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WDGTB_A::B_0x0
    }
    #[doc = "CK counter clock (PCLK div 4096) div 2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WDGTB_A::B_0x1
    }
    #[doc = "CK counter clock (PCLK div 4096) div 4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == WDGTB_A::B_0x2
    }
    #[doc = "CK counter clock (PCLK div 4096) div 8"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == WDGTB_A::B_0x3
    }
    #[doc = "CK counter clock (PCLK div 4096) div 16"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == WDGTB_A::B_0x4
    }
    #[doc = "CK counter clock (PCLK div 4096) div 32"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == WDGTB_A::B_0x5
    }
    #[doc = "CK counter clock (PCLK div 4096) div 64"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == WDGTB_A::B_0x6
    }
    #[doc = "CK counter clock (PCLK div 4096) div 128"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == WDGTB_A::B_0x7
    }
}
#[doc = "Field `WDGTB` writer - Timer base The timebase of the prescaler can be modified as follows:"]
pub type WDGTB_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WDGTB_A, crate::Safe>;
impl<'a, REG> WDGTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK counter clock (PCLK div 4096) div 1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0x0)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0x1)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0x2)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 8"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0x3)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 16"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0x4)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 32"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0x5)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 64"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0x6)
    }
    #[doc = "CK counter clock (PCLK div 4096) div 128"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0x7)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter."]
    #[inline(always)]
    pub fn W(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
    #[inline(always)]
    pub fn EWI(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:"]
    #[inline(always)]
    pub fn WDGTB(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter."]
    #[inline(always)]
    pub fn W(&mut self) -> W_W<'_, CFR_SPEC> {
        W_W::new(self, 0)
    }
    #[doc = "Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
    #[inline(always)]
    pub fn EWI(&mut self) -> EWI_W<'_, CFR_SPEC> {
        EWI_W::new(self, 9)
    }
    #[doc = "Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:"]
    #[inline(always)]
    pub fn WDGTB(&mut self) -> WDGTB_W<'_, CFR_SPEC> {
        WDGTB_W::new(self, 11)
    }
}
#[doc = "WWDG configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfr::R`](R) reader structure"]
impl crate::Readable for CFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CFR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFR to value 0x7f"]
impl crate::Resettable for CFR_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
