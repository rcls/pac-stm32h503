#[doc = "Register `CALR` reader"]
pub type R = crate::R<CALR_SPEC>;
#[doc = "Register `CALR` writer"]
pub type W = crate::W<CALR_SPEC>;
#[doc = "Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2sup20/sup RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
pub type CALM_R = crate::FieldReader<u16>;
#[doc = "Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2sup20/sup RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
pub type CALM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "RTC low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCAL_A {
    #[doc = "0: Calibration window is 2sup20/sup RTCCLK, which is a high-consumption mode. This mode must be set only when less than 32s calibration window is required."]
    B_0x0 = 0,
    #[doc = "1: Calibration window is 2sup20/sup ck_apre, which is the required configuration for ultra-low consumption mode."]
    B_0x1 = 1,
}
impl From<LPCAL_A> for bool {
    #[inline(always)]
    fn from(variant: LPCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPCAL` reader - RTC low-power mode"]
pub type LPCAL_R = crate::BitReader<LPCAL_A>;
impl LPCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPCAL_A {
        match self.bits {
            false => LPCAL_A::B_0x0,
            true => LPCAL_A::B_0x1,
        }
    }
    #[doc = "Calibration window is 2sup20/sup RTCCLK, which is a high-consumption mode. This mode must be set only when less than 32s calibration window is required."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPCAL_A::B_0x0
    }
    #[doc = "Calibration window is 2sup20/sup ck_apre, which is the required configuration for ultra-low consumption mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPCAL_A::B_0x1
    }
}
#[doc = "Field `LPCAL` writer - RTC low-power mode"]
pub type LPCAL_W<'a, REG> = crate::BitWriter<'a, REG, LPCAL_A>;
impl<'a, REG> LPCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration window is 2sup20/sup RTCCLK, which is a high-consumption mode. This mode must be set only when less than 32s calibration window is required."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPCAL_A::B_0x0)
    }
    #[doc = "Calibration window is 2sup20/sup ck_apre, which is the required configuration for ultra-low consumption mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPCAL_A::B_0x1)
    }
}
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\] is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW16_R = crate::BitReader;
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\] is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\] are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW8_R = crate::BitReader;
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\] are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALW8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. Refer to Section 31.3.15: RTC smooth digital calibration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALP_A {
    #[doc = "0: No RTCCLK pulses are added."]
    B_0x0 = 0,
    #[doc = "1: One RTCCLK pulse is effectively inserted every 2sup11/sup pulses (frequency increased by 488.5 ppm)."]
    B_0x1 = 1,
}
impl From<CALP_A> for bool {
    #[inline(always)]
    fn from(variant: CALP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALP_R = crate::BitReader<CALP_A>;
impl CALP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALP_A {
        match self.bits {
            false => CALP_A::B_0x0,
            true => CALP_A::B_0x1,
        }
    }
    #[doc = "No RTCCLK pulses are added."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CALP_A::B_0x0
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 2sup11/sup pulses (frequency increased by 488.5 ppm)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CALP_A::B_0x1
    }
}
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. Refer to Section 31.3.15: RTC smooth digital calibration."]
pub type CALP_W<'a, REG> = crate::BitWriter<'a, REG, CALP_A>;
impl<'a, REG> CALP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No RTCCLK pulses are added."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CALP_A::B_0x0)
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 2sup11/sup pulses (frequency increased by 488.5 ppm)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CALP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2sup20/sup RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
    #[inline(always)]
    pub fn CALM(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - RTC low-power mode"]
    #[inline(always)]
    pub fn LPCAL(&self) -> LPCAL_R {
        LPCAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\] is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn CALW16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\] are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn CALW8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn CALP(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 2sup20/sup RTCCLK pulses (32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the calendar with a resolution of 0.9537 ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See Section 31.3.15: RTC smooth digital calibration on page 1092."]
    #[inline(always)]
    pub fn CALM(&mut self) -> CALM_W<'_, CALR_SPEC> {
        CALM_W::new(self, 0)
    }
    #[doc = "Bit 12 - RTC low-power mode"]
    #[inline(always)]
    pub fn LPCAL(&mut self) -> LPCAL_W<'_, CALR_SPEC> {
        LPCAL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\] is stuck at 0 when CALW16 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn CALW16(&mut self) -> CALW16_W<'_, CALR_SPEC> {
        CALW16_W::new(self, 13)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\] are stuck at 00 when CALW8 = 1. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn CALW8(&mut self) -> CALW8_W<'_, CALR_SPEC> {
        CALW8_W::new(self, 14)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. Refer to Section 31.3.15: RTC smooth digital calibration."]
    #[inline(always)]
    pub fn CALP(&mut self) -> CALP_W<'_, CALR_SPEC> {
        CALP_W::new(self, 15)
    }
}
#[doc = "RTC calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`calr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALR_SPEC;
impl crate::RegisterSpec for CALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calr::R`](R) reader structure"]
impl crate::Readable for CALR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calr::W`](W) writer structure"]
impl crate::Writable for CALR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CALR to value 0"]
impl crate::Resettable for CALR_SPEC {}
