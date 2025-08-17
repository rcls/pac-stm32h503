#[doc = "Register `SECSR` reader"]
pub type R = crate::R<SECSR_SPEC>;
#[doc = "busy flag BSY flag indicates that a FLASH memory is busy by an operation (write, erase, option byte change, OBK operations, PUF operation). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECBSY_A {
    #[doc = "0: no programming, erase or option byte change operation being executed"]
    B_0x0 = 0,
    #[doc = "1: programming, erase or option byte change operation being executed"]
    B_0x1 = 1,
}
impl From<SECBSY_A> for bool {
    #[inline(always)]
    fn from(variant: SECBSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECBSY` reader - busy flag BSY flag indicates that a FLASH memory is busy by an operation (write, erase, option byte change, OBK operations, PUF operation). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
pub type SECBSY_R = crate::BitReader<SECBSY_A>;
impl SECBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECBSY_A {
        match self.bits {
            false => SECBSY_A::B_0x0,
            true => SECBSY_A::B_0x1,
        }
    }
    #[doc = "no programming, erase or option byte change operation being executed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECBSY_A::B_0x0
    }
    #[doc = "programming, erase or option byte change operation being executed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECBSY_A::B_0x1
    }
}
#[doc = "write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECWBNE_A {
    #[doc = "0: write buffer empty or full"]
    B_0x0 = 0,
    #[doc = "1: write buffer waiting data to complete"]
    B_0x1 = 1,
}
impl From<SECWBNE_A> for bool {
    #[inline(always)]
    fn from(variant: SECWBNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECWBNE` reader - write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
pub type SECWBNE_R = crate::BitReader<SECWBNE_A>;
impl SECWBNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECWBNE_A {
        match self.bits {
            false => SECWBNE_A::B_0x0,
            true => SECWBNE_A::B_0x1,
        }
    }
    #[doc = "write buffer empty or full"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECWBNE_A::B_0x0
    }
    #[doc = "write buffer waiting data to complete"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECWBNE_A::B_0x1
    }
}
#[doc = "data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECDBNE_A {
    #[doc = "0: data buffer not used"]
    B_0x0 = 0,
    #[doc = "1: data buffer used, wait"]
    B_0x1 = 1,
}
impl From<SECDBNE_A> for bool {
    #[inline(always)]
    fn from(variant: SECDBNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECDBNE` reader - data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
pub type SECDBNE_R = crate::BitReader<SECDBNE_A>;
impl SECDBNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECDBNE_A {
        match self.bits {
            false => SECDBNE_A::B_0x0,
            true => SECDBNE_A::B_0x1,
        }
    }
    #[doc = "data buffer not used"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECDBNE_A::B_0x0
    }
    #[doc = "data buffer used, wait"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECDBNE_A::B_0x1
    }
}
#[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECEOP_A {
    #[doc = "0: no operation completed"]
    B_0x0 = 0,
    #[doc = "1: a operation completed"]
    B_0x1 = 1,
}
impl From<SECEOP_A> for bool {
    #[inline(always)]
    fn from(variant: SECEOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECEOP` reader - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
pub type SECEOP_R = crate::BitReader<SECEOP_A>;
impl SECEOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECEOP_A {
        match self.bits {
            false => SECEOP_A::B_0x0,
            true => SECEOP_A::B_0x1,
        }
    }
    #[doc = "no operation completed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECEOP_A::B_0x0
    }
    #[doc = "a operation completed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECEOP_A::B_0x1
    }
}
#[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECWRPERR_A {
    #[doc = "0: no write protection error occurred"]
    B_0x0 = 0,
    #[doc = "1: a write protection error occurred"]
    B_0x1 = 1,
}
impl From<SECWRPERR_A> for bool {
    #[inline(always)]
    fn from(variant: SECWRPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECWRPERR` reader - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
pub type SECWRPERR_R = crate::BitReader<SECWRPERR_A>;
impl SECWRPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECWRPERR_A {
        match self.bits {
            false => SECWRPERR_A::B_0x0,
            true => SECWRPERR_A::B_0x1,
        }
    }
    #[doc = "no write protection error occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECWRPERR_A::B_0x0
    }
    #[doc = "a write protection error occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECWRPERR_A::B_0x1
    }
}
#[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECPGSERR_A {
    #[doc = "0: no sequence error occurred"]
    B_0x0 = 0,
    #[doc = "1: a sequence error occurred"]
    B_0x1 = 1,
}
impl From<SECPGSERR_A> for bool {
    #[inline(always)]
    fn from(variant: SECPGSERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECPGSERR` reader - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
pub type SECPGSERR_R = crate::BitReader<SECPGSERR_A>;
impl SECPGSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECPGSERR_A {
        match self.bits {
            false => SECPGSERR_A::B_0x0,
            true => SECPGSERR_A::B_0x1,
        }
    }
    #[doc = "no sequence error occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECPGSERR_A::B_0x0
    }
    #[doc = "a sequence error occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECPGSERR_A::B_0x1
    }
}
#[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSTRBERR_A {
    #[doc = "0: no strobe error occurred"]
    B_0x0 = 0,
    #[doc = "1: a strobe error occurred"]
    B_0x1 = 1,
}
impl From<SECSTRBERR_A> for bool {
    #[inline(always)]
    fn from(variant: SECSTRBERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECSTRBERR` reader - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
pub type SECSTRBERR_R = crate::BitReader<SECSTRBERR_A>;
impl SECSTRBERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECSTRBERR_A {
        match self.bits {
            false => SECSTRBERR_A::B_0x0,
            true => SECSTRBERR_A::B_0x1,
        }
    }
    #[doc = "no strobe error occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECSTRBERR_A::B_0x0
    }
    #[doc = "a strobe error occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECSTRBERR_A::B_0x1
    }
}
#[doc = "inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECINCERR_A {
    #[doc = "0: no inconsistency error occurred"]
    B_0x0 = 0,
    #[doc = "1: a inconsistency error occurred"]
    B_0x1 = 1,
}
impl From<SECINCERR_A> for bool {
    #[inline(always)]
    fn from(variant: SECINCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECINCERR` reader - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
pub type SECINCERR_R = crate::BitReader<SECINCERR_A>;
impl SECINCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECINCERR_A {
        match self.bits {
            false => SECINCERR_A::B_0x0,
            true => SECINCERR_A::B_0x1,
        }
    }
    #[doc = "no inconsistency error occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SECINCERR_A::B_0x0
    }
    #[doc = "a inconsistency error occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SECINCERR_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - busy flag BSY flag indicates that a FLASH memory is busy by an operation (write, erase, option byte change, OBK operations, PUF operation). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
    #[inline(always)]
    pub fn SECBSY(&self) -> SECBSY_R {
        SECBSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
    #[inline(always)]
    pub fn SECWBNE(&self) -> SECWBNE_R {
        SECWBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
    #[inline(always)]
    pub fn SECDBNE(&self) -> SECDBNE_R {
        SECDBNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
    #[inline(always)]
    pub fn SECEOP(&self) -> SECEOP_R {
        SECEOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
    #[inline(always)]
    pub fn SECWRPERR(&self) -> SECWRPERR_R {
        SECWRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
    #[inline(always)]
    pub fn SECPGSERR(&self) -> SECPGSERR_R {
        SECPGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
    #[inline(always)]
    pub fn SECSTRBERR(&self) -> SECSTRBERR_R {
        SECSTRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
    #[inline(always)]
    pub fn SECINCERR(&self) -> SECINCERR_R {
        SECINCERR_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "FLASH secure status register\n\nYou can [`read`](crate::Reg::read) this register and get [`secsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECSR_SPEC;
impl crate::RegisterSpec for SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secsr::R`](R) reader structure"]
impl crate::Readable for SECSR_SPEC {}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSR_SPEC {}
