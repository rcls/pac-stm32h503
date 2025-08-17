#[doc = "Register `DBTP` reader"]
pub type R = crate::R<DBTP_SPEC>;
#[doc = "Register `DBTP` writer"]
pub type W = crate::W<DBTP_SPEC>;
#[doc = "Field `DSJW` reader - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
pub type DSJW_R = crate::FieldReader;
#[doc = "Field `DSJW` writer - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
pub type DSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq."]
pub type DTSEG2_R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq."]
pub type DTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq."]
pub type DTSEG1_R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq."]
pub type DTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
pub type DBRP_R = crate::FieldReader;
#[doc = "Field `DBRP` writer - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
pub type DBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Transceiver delay compensation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDC_A {
    #[doc = "0: Transceiver delay compensation disabled"]
    B_0x0 = 0,
    #[doc = "1: Transceiver delay compensation enabled"]
    B_0x1 = 1,
}
impl From<TDC_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - Transceiver delay compensation"]
pub type TDC_R = crate::BitReader<TDC_A>;
impl TDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDC_A {
        match self.bits {
            false => TDC_A::B_0x0,
            true => TDC_A::B_0x1,
        }
    }
    #[doc = "Transceiver delay compensation disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TDC_A::B_0x0
    }
    #[doc = "Transceiver delay compensation enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TDC_A::B_0x1
    }
}
#[doc = "Field `TDC` writer - Transceiver delay compensation"]
pub type TDC_W<'a, REG> = crate::BitWriter<'a, REG, TDC_A>;
impl<'a, REG> TDC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transceiver delay compensation disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TDC_A::B_0x0)
    }
    #[doc = "Transceiver delay compensation enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TDC_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
    #[inline(always)]
    pub fn DSJW(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq."]
    #[inline(always)]
    pub fn DTSEG2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq."]
    #[inline(always)]
    pub fn DTSEG1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
    #[inline(always)]
    pub fn DBRP(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver delay compensation"]
    #[inline(always)]
    pub fn TDC(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Synchronization jump width Must always be smaller than DTSEG2, valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1: tSJW = (DSJW + 1) x tq."]
    #[inline(always)]
    pub fn DSJW(&mut self) -> DSJW_W<'_, DBTP_SPEC> {
        DSJW_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point Valid values are 0 to 15. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS2 = (DTSEG2 + 1) x tq."]
    #[inline(always)]
    pub fn DTSEG2(&mut self) -> DTSEG2_W<'_, DBTP_SPEC> {
        DTSEG2_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point Valid values are 0 to 31. The value used by the hardware is the one programmed, incremented by 1, i.e. tBS1 = (DTSEG1 + 1) x tq."]
    #[inline(always)]
    pub fn DTSEG1(&mut self) -> DTSEG1_W<'_, DBTP_SPEC> {
        DTSEG1_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data bit rate prescaler The value by which the oscillator frequency is divided to generate the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Baud Rate Prescaler are 0 to 31. The hardware interpreters this value as the value programmed plus 1."]
    #[inline(always)]
    pub fn DBRP(&mut self) -> DBRP_W<'_, DBTP_SPEC> {
        DBRP_W::new(self, 16)
    }
    #[doc = "Bit 23 - Transceiver delay compensation"]
    #[inline(always)]
    pub fn TDC(&mut self) -> TDC_W<'_, DBTP_SPEC> {
        TDC_W::new(self, 23)
    }
}
#[doc = "FDCAN data bit timing and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBTP_SPEC;
impl crate::RegisterSpec for DBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbtp::R`](R) reader structure"]
impl crate::Readable for DBTP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbtp::W`](W) writer structure"]
impl crate::Writable for DBTP_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBTP to value 0x0a33"]
impl crate::Resettable for DBTP_SPEC {
    const RESET_VALUE: u32 = 0x0a33;
}
