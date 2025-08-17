#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISR_SPEC>;
#[doc = "Field `ALRAMF` reader - Alarm A masked flag This flag is set by hardware when the alarm A interrupt occurs."]
pub type ALRAMF_R = crate::BitReader;
#[doc = "Field `ALRBMF` reader - Alarm B masked flag This flag is set by hardware when the alarm B interrupt occurs."]
pub type ALRBMF_R = crate::BitReader;
#[doc = "Field `WUTMF` reader - Wakeup timer masked flag This flag is set by hardware when the wakeup timer interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
pub type WUTMF_R = crate::BitReader;
#[doc = "Field `TSMF` reader - Timestamp masked flag This flag is set by hardware when a timestamp interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF."]
pub type TSMF_R = crate::BitReader;
#[doc = "Field `TSOVMF` reader - Timestamp overflow masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type TSOVMF_R = crate::BitReader;
#[doc = "Field `ITSMF` reader - Internal timestamp masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestampinterrupt is raised."]
pub type ITSMF_R = crate::BitReader;
#[doc = "Field `SSRUMF` reader - SSR underflow masked flag This flag is set by hardware when the SSR underflow interrupt occurs."]
pub type SSRUMF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A masked flag This flag is set by hardware when the alarm A interrupt occurs."]
    #[inline(always)]
    pub fn ALRAMF(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag This flag is set by hardware when the alarm B interrupt occurs."]
    #[inline(always)]
    pub fn ALRBMF(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag This flag is set by hardware when the wakeup timer interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again."]
    #[inline(always)]
    pub fn WUTMF(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp masked flag This flag is set by hardware when a timestamp interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF."]
    #[inline(always)]
    pub fn TSMF(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    pub fn TSOVMF(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestampinterrupt is raised."]
    #[inline(always)]
    pub fn ITSMF(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SSR underflow masked flag This flag is set by hardware when the SSR underflow interrupt occurs."]
    #[inline(always)]
    pub fn SSRUMF(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "RTC masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISR_SPEC {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {}
