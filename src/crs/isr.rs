#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOKF_A {
    #[doc = "0: No SYNC event OK signalized"]
    B_0x0 = 0,
    #[doc = "1: SYNC event OK signalized"]
    B_0x1 = 1,
}
impl From<SYNCOKF_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCOKF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCOKF` reader - SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register."]
pub type SYNCOKF_R = crate::BitReader<SYNCOKF_A>;
impl SYNCOKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCOKF_A {
        match self.bits {
            false => SYNCOKF_A::B_0x0,
            true => SYNCOKF_A::B_0x1,
        }
    }
    #[doc = "No SYNC event OK signalized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCOKF_A::B_0x0
    }
    #[doc = "SYNC event OK signalized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCOKF_A::B_0x1
    }
}
#[doc = "SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCWARNF_A {
    #[doc = "0: No SYNC warning signalized"]
    B_0x0 = 0,
    #[doc = "1: SYNC warning signalized"]
    B_0x1 = 1,
}
impl From<SYNCWARNF_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCWARNF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCWARNF` reader - SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register."]
pub type SYNCWARNF_R = crate::BitReader<SYNCWARNF_A>;
impl SYNCWARNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCWARNF_A {
        match self.bits {
            false => SYNCWARNF_A::B_0x0,
            true => SYNCWARNF_A::B_0x1,
        }
    }
    #[doc = "No SYNC warning signalized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCWARNF_A::B_0x0
    }
    #[doc = "SYNC warning signalized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCWARNF_A::B_0x1
    }
}
#[doc = "Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRF_A {
    #[doc = "0: No synchronization or trimming error signalized"]
    B_0x0 = 0,
    #[doc = "1: Synchronization or trimming error signalized"]
    B_0x1 = 1,
}
impl From<ERRF_A> for bool {
    #[inline(always)]
    fn from(variant: ERRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRF` reader - Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits."]
pub type ERRF_R = crate::BitReader<ERRF_A>;
impl ERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRF_A {
        match self.bits {
            false => ERRF_A::B_0x0,
            true => ERRF_A::B_0x1,
        }
    }
    #[doc = "No synchronization or trimming error signalized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ERRF_A::B_0x0
    }
    #[doc = "Synchronization or trimming error signalized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ERRF_A::B_0x1
    }
}
#[doc = "Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESYNCF_A {
    #[doc = "0: No expected SYNC signalized"]
    B_0x0 = 0,
    #[doc = "1: Expected SYNC signalized"]
    B_0x1 = 1,
}
impl From<ESYNCF_A> for bool {
    #[inline(always)]
    fn from(variant: ESYNCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESYNCF` reader - Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register."]
pub type ESYNCF_R = crate::BitReader<ESYNCF_A>;
impl ESYNCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESYNCF_A {
        match self.bits {
            false => ESYNCF_A::B_0x0,
            true => ESYNCF_A::B_0x1,
        }
    }
    #[doc = "No expected SYNC signalized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ESYNCF_A::B_0x0
    }
    #[doc = "Expected SYNC signalized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ESYNCF_A::B_0x1
    }
}
#[doc = "SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action has to be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCERR_A {
    #[doc = "0: No SYNC error signalized"]
    B_0x0 = 0,
    #[doc = "1: SYNC error signalized"]
    B_0x1 = 1,
}
impl From<SYNCERR_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCERR` reader - SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action has to be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type SYNCERR_R = crate::BitReader<SYNCERR_A>;
impl SYNCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCERR_A {
        match self.bits {
            false => SYNCERR_A::B_0x0,
            true => SYNCERR_A::B_0x1,
        }
    }
    #[doc = "No SYNC error signalized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCERR_A::B_0x0
    }
    #[doc = "SYNC error signalized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCERR_A::B_0x1
    }
}
#[doc = "SYNC missed This flag is set by hardware when the frequency error counter reached value FELIM * 128 and no SYNC was detected, meaning either that a SYNC pulse was missed or that the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, and that some other action has to be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC) and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCMISS_A {
    #[doc = "0: No SYNC missed error signalized"]
    B_0x0 = 0,
    #[doc = "1: SYNC missed error signalized"]
    B_0x1 = 1,
}
impl From<SYNCMISS_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCMISS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCMISS` reader - SYNC missed This flag is set by hardware when the frequency error counter reached value FELIM * 128 and no SYNC was detected, meaning either that a SYNC pulse was missed or that the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, and that some other action has to be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC) and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type SYNCMISS_R = crate::BitReader<SYNCMISS_A>;
impl SYNCMISS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCMISS_A {
        match self.bits {
            false => SYNCMISS_A::B_0x0,
            true => SYNCMISS_A::B_0x1,
        }
    }
    #[doc = "No SYNC missed error signalized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCMISS_A::B_0x0
    }
    #[doc = "SYNC missed error signalized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCMISS_A::B_0x1
    }
}
#[doc = "Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIMOVF_A {
    #[doc = "0: No trimming error signalized"]
    B_0x0 = 0,
    #[doc = "1: Trimming error signalized"]
    B_0x1 = 1,
}
impl From<TRIMOVF_A> for bool {
    #[inline(always)]
    fn from(variant: TRIMOVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIMOVF` reader - Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
pub type TRIMOVF_R = crate::BitReader<TRIMOVF_A>;
impl TRIMOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRIMOVF_A {
        match self.bits {
            false => TRIMOVF_A::B_0x0,
            true => TRIMOVF_A::B_0x1,
        }
    }
    #[doc = "No trimming error signalized"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRIMOVF_A::B_0x0
    }
    #[doc = "Trimming error signalized"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRIMOVF_A::B_0x1
    }
}
#[doc = "Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEDIR_A {
    #[doc = "0: Upcounting direction, the actual frequency is above the target."]
    B_0x0 = 0,
    #[doc = "1: Downcounting direction, the actual frequency is below the target."]
    B_0x1 = 1,
}
impl From<FEDIR_A> for bool {
    #[inline(always)]
    fn from(variant: FEDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEDIR` reader - Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target."]
pub type FEDIR_R = crate::BitReader<FEDIR_A>;
impl FEDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEDIR_A {
        match self.bits {
            false => FEDIR_A::B_0x0,
            true => FEDIR_A::B_0x1,
        }
    }
    #[doc = "Upcounting direction, the actual frequency is above the target."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FEDIR_A::B_0x0
    }
    #[doc = "Downcounting direction, the actual frequency is below the target."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FEDIR_A::B_0x1
    }
}
#[doc = "Field `FECAP` reader - Frequency error capture FECAP is the frequency error counter value latched in the time of the last SYNC event. Refer to Section 10.5.3 for more details about FECAP usage."]
pub type FECAP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - SYNC event OK flag This flag is set by hardware when the measured frequency error is smaller than FELIM * 3. This means that either no adjustment of the TRIM value is needed or that an adjustment by one trimming step is enough to compensate the frequency error. An interrupt is generated if the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCOKC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn SYNCOKF(&self) -> SYNCOKF_R {
        SYNCOKF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning flag This flag is set by hardware when the measured frequency error is greater than or equal to FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the SYNCWARNC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn SYNCWARNF(&self) -> SYNCWARNF_R {
        SYNCWARNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error flag This flag is set by hardware in case of any synchronization or trimming error. It is the logical OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits."]
    #[inline(always)]
    pub fn ERRF(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC flag This flag is set by hardware when the frequency error counter reached a zero value. An interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by software by setting the ESYNCC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn ESYNCF(&self) -> ESYNCF_R {
        ESYNCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SYNC error This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the measured frequency error is greater than or equal to FELIM * 128. This means that the frequency error is too big (internal frequency too low) to be compensated by adjusting the TRIM value, and that some other action has to be taken. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn SYNCERR(&self) -> SYNCERR_R {
        SYNCERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SYNC missed This flag is set by hardware when the frequency error counter reached value FELIM * 128 and no SYNC was detected, meaning either that a SYNC pulse was missed or that the frequency error is too big (internal frequency too high) to be compensated by adjusting the TRIM value, and that some other action has to be taken. At this point, the frequency error counter is stopped (waiting for a next SYNC) and an interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn SYNCMISS(&self) -> SYNCMISS_R {
        SYNCMISS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trimming overflow or underflow This flag is set by hardware when the automatic trimming tries to over- or under-flow the TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register."]
    #[inline(always)]
    pub fn TRIMOVF(&self) -> TRIMOVF_R {
        TRIMOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Frequency error direction FEDIR is the counting direction of the frequency error counter latched in the time of the last SYNC event. It shows whether the actual frequency is below or above the target."]
    #[inline(always)]
    pub fn FEDIR(&self) -> FEDIR_R {
        FEDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Frequency error capture FECAP is the frequency error counter value latched in the time of the last SYNC event. Refer to Section 10.5.3 for more details about FECAP usage."]
    #[inline(always)]
    pub fn FECAP(&self) -> FECAP_R {
        FECAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "CRS interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {}
