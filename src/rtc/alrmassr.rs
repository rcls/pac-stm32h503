#[doc = "Register `ALRMASSR` reader"]
pub type R = crate::R<ALRMASSR_SPEC>;
#[doc = "Register `ALRMASSR` writer"]
pub type W = crate::W<ALRMASSR_SPEC>;
#[doc = "Field `SS` reader - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\] in the RTC_ALRMABINR, and so can also be read or written through RTC_ALRMABINR."]
pub type SS_R = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\] in the RTC_ALRMABINR, and so can also be read or written through RTC_ALRMABINR."]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKSS_A {
    #[doc = "0: No comparison on subseconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    B_0x0 = 0,
    #[doc = "1: SS\\[31:1\\] are don't care in Alarm A comparison. Only SS\\[0\\] is compared."]
    B_0x1 = 1,
}
impl From<MASKSS_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MASKSS_A {
    type Ux = u8;
}
impl crate::IsEnum for MASKSS_A {}
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation."]
pub type MASKSS_R = crate::FieldReader<MASKSS_A>;
impl MASKSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MASKSS_A> {
        match self.bits {
            0 => Some(MASKSS_A::B_0x0),
            1 => Some(MASKSS_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "No comparison on subseconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MASKSS_A::B_0x0
    }
    #[doc = "SS\\[31:1\\] are don't care in Alarm A comparison. Only SS\\[0\\] is compared."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MASKSS_A::B_0x1
    }
}
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation."]
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 6, MASKSS_A>;
impl<'a, REG> MASKSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No comparison on subseconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0x0)
    }
    #[doc = "SS\\[31:1\\] are don't care in Alarm A comparison. Only SS\\[0\\] is compared."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0x1)
    }
}
#[doc = "Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCLR_A {
    #[doc = "0: The synchronous binary counter (SS\\[31:0\\] in RTC_SSR) is free-running."]
    B_0x0 = 0,
    #[doc = "1: The synchronous binary counter (SS\\[31:0\\] in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR.SS\\[31:0\\] value and is automatically reloaded with 0xFFFF FFFF one ck_apre cycle after reaching RTC_ALRMABINR.SS\\[31:0\\]."]
    B_0x1 = 1,
}
impl From<SSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SSCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCLR` reader - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
pub type SSCLR_R = crate::BitReader<SSCLR_A>;
impl SSCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCLR_A {
        match self.bits {
            false => SSCLR_A::B_0x0,
            true => SSCLR_A::B_0x1,
        }
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\] in RTC_SSR) is free-running."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SSCLR_A::B_0x0
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\] in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR.SS\\[31:0\\] value and is automatically reloaded with 0xFFFF FFFF one ck_apre cycle after reaching RTC_ALRMABINR.SS\\[31:0\\]."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SSCLR_A::B_0x1
    }
}
#[doc = "Field `SSCLR` writer - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
pub type SSCLR_W<'a, REG> = crate::BitWriter<'a, REG, SSCLR_A>;
impl<'a, REG> SSCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The synchronous binary counter (SS\\[31:0\\] in RTC_SSR) is free-running."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCLR_A::B_0x0)
    }
    #[doc = "The synchronous binary counter (SS\\[31:0\\] in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR.SS\\[31:0\\] value and is automatically reloaded with 0xFFFF FFFF one ck_apre cycle after reaching RTC_ALRMABINR.SS\\[31:0\\]."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCLR_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:14 - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\] in the RTC_ALRMABINR, and so can also be read or written through RTC_ALRMABINR."]
    #[inline(always)]
    pub fn SS(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:29 - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn MASKSS(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
    #[inline(always)]
    pub fn SSCLR(&self) -> SSCLR_R {
        SSCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Subseconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared. This field is the mirror of SS\\[14:0\\] in the RTC_ALRMABINR, and so can also be read or written through RTC_ALRMABINR."]
    #[inline(always)]
    pub fn SS(&mut self) -> SS_W<'_, ALRMASSR_SPEC> {
        SS_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask the most-significant bits starting at this bit ... From 32 to 63: All 32 SS bits are compared and must match to activate alarm. Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are never compared. These bits can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn MASKSS(&mut self) -> MASKSS_W<'_, ALRMASSR_SPEC> {
        MASKSS_W::new(self, 24)
    }
    #[doc = "Bit 31 - Clear synchronous counter on alarm (Binary mode only) Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11)."]
    #[inline(always)]
    pub fn SSCLR(&mut self) -> SSCLR_W<'_, ALRMASSR_SPEC> {
        SSCLR_W::new(self, 31)
    }
}
#[doc = "RTC alarm A subsecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmassr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmassr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMASSR_SPEC;
impl crate::RegisterSpec for ALRMASSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmassr::R`](R) reader structure"]
impl crate::Readable for ALRMASSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrmassr::W`](W) writer structure"]
impl crate::Writable for ALRMASSR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ALRMASSR to value 0"]
impl crate::Resettable for ALRMASSR_SPEC {}
