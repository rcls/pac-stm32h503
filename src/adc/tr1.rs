#[doc = "Register `TR1` reader"]
pub type R = crate::R<TR1_SPEC>;
#[doc = "Register `TR1` writer"]
pub type W = crate::W<TR1_SPEC>;
#[doc = "Field `LT1` reader - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type LT1_R = crate::FieldReader<u16>;
#[doc = "Field `LT1` writer - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type LT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWDFILT_A {
    #[doc = "0: No filtering"]
    B_0x0 = 0,
    #[doc = "1: two consecutive detection generates an AWDx flag or an interrupt"]
    B_0x1 = 1,
    #[doc = "7: Eight consecutive detection generates an AWDx flag or an interrupt"]
    B_0x7 = 7,
}
impl From<AWDFILT_A> for u8 {
    #[inline(always)]
    fn from(variant: AWDFILT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWDFILT_A {
    type Ux = u8;
}
impl crate::IsEnum for AWDFILT_A {}
#[doc = "Field `AWDFILT` reader - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWDFILT_R = crate::FieldReader<AWDFILT_A>;
impl AWDFILT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AWDFILT_A> {
        match self.bits {
            0 => Some(AWDFILT_A::B_0x0),
            1 => Some(AWDFILT_A::B_0x1),
            7 => Some(AWDFILT_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AWDFILT_A::B_0x0
    }
    #[doc = "two consecutive detection generates an AWDx flag or an interrupt"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AWDFILT_A::B_0x1
    }
    #[doc = "Eight consecutive detection generates an AWDx flag or an interrupt"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == AWDFILT_A::B_0x7
    }
}
#[doc = "Field `AWDFILT` writer - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWDFILT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, AWDFILT_A>;
impl<'a, REG> AWDFILT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT_A::B_0x0)
    }
    #[doc = "two consecutive detection generates an AWDx flag or an interrupt"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT_A::B_0x1)
    }
    #[doc = "Eight consecutive detection generates an AWDx flag or an interrupt"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AWDFILT_A::B_0x7)
    }
}
#[doc = "Field `HT1` reader - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type HT1_R = crate::FieldReader<u16>;
#[doc = "Field `HT1` writer - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type HT1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn LT1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWDFILT(&self) -> AWDFILT_R {
        AWDFILT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:27 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn HT1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn LT1(&mut self) -> LT1_W<'_, TR1_SPEC> {
        LT1_W::new(self, 0)
    }
    #[doc = "Bits 12:14 - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn AWDFILT(&mut self) -> AWDFILT_W<'_, TR1_SPEC> {
        AWDFILT_W::new(self, 12)
    }
    #[doc = "Bits 16:27 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTx, AWD_LTx, AWDx) Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn HT1(&mut self) -> HT1_W<'_, TR1_SPEC> {
        HT1_W::new(self, 16)
    }
}
#[doc = "ADC watchdog threshold register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR1_SPEC;
impl crate::RegisterSpec for TR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr1::R`](R) reader structure"]
impl crate::Readable for TR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr1::W`](W) writer structure"]
impl crate::Writable for TR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TR1 to value 0x0fff_0000"]
impl crate::Resettable for TR1_SPEC {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
