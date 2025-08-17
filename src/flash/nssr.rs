#[doc = "Register `NSSR` reader"]
pub type R = crate::R<NSSR_SPEC>;
#[doc = "busy flag BSY flag indicates that a Flash memory is busy by an operation (write, erase, option byte change). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSY_A {
    #[doc = "0: no programming, erase or option byte change operation being executed"]
    B_0x0 = 0,
    #[doc = "1: programming, erase or option byte change operation being executed"]
    B_0x1 = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - busy flag BSY flag indicates that a Flash memory is busy by an operation (write, erase, option byte change). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
pub type BSY_R = crate::BitReader<BSY_A>;
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::B_0x0,
            true => BSY_A::B_0x1,
        }
    }
    #[doc = "no programming, erase or option byte change operation being executed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BSY_A::B_0x0
    }
    #[doc = "programming, erase or option byte change operation being executed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BSY_A::B_0x1
    }
}
#[doc = "write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WBNE_A {
    #[doc = "0: write buffer empty or full"]
    B_0x0 = 0,
    #[doc = "1: write buffer waiting data to complete"]
    B_0x1 = 1,
}
impl From<WBNE_A> for bool {
    #[inline(always)]
    fn from(variant: WBNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WBNE` reader - write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
pub type WBNE_R = crate::BitReader<WBNE_A>;
impl WBNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WBNE_A {
        match self.bits {
            false => WBNE_A::B_0x0,
            true => WBNE_A::B_0x1,
        }
    }
    #[doc = "write buffer empty or full"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WBNE_A::B_0x0
    }
    #[doc = "write buffer waiting data to complete"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WBNE_A::B_0x1
    }
}
#[doc = "data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBNE_A {
    #[doc = "0: data buffer not used"]
    B_0x0 = 0,
    #[doc = "1: data buffer used, wait"]
    B_0x1 = 1,
}
impl From<DBNE_A> for bool {
    #[inline(always)]
    fn from(variant: DBNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBNE` reader - data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
pub type DBNE_R = crate::BitReader<DBNE_A>;
impl DBNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBNE_A {
        match self.bits {
            false => DBNE_A::B_0x0,
            true => DBNE_A::B_0x1,
        }
    }
    #[doc = "data buffer not used"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBNE_A::B_0x0
    }
    #[doc = "data buffer used, wait"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBNE_A::B_0x1
    }
}
#[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOP_A {
    #[doc = "0: no operation completed"]
    B_0x0 = 0,
    #[doc = "1: a operation completed"]
    B_0x1 = 1,
}
impl From<EOP_A> for bool {
    #[inline(always)]
    fn from(variant: EOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` reader - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register."]
pub type EOP_R = crate::BitReader<EOP_A>;
impl EOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOP_A {
        match self.bits {
            false => EOP_A::B_0x0,
            true => EOP_A::B_0x1,
        }
    }
    #[doc = "no operation completed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EOP_A::B_0x0
    }
    #[doc = "a operation completed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EOP_A::B_0x1
    }
}
#[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERR_A {
    #[doc = "0: no write protection error occurred"]
    B_0x0 = 0,
    #[doc = "1: a write protection error occurred"]
    B_0x1 = 1,
}
impl From<WRPERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` reader - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR."]
pub type WRPERR_R = crate::BitReader<WRPERR_A>;
impl WRPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRPERR_A {
        match self.bits {
            false => WRPERR_A::B_0x0,
            true => WRPERR_A::B_0x1,
        }
    }
    #[doc = "no write protection error occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WRPERR_A::B_0x0
    }
    #[doc = "a write protection error occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WRPERR_A::B_0x1
    }
}
#[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERR_A {
    #[doc = "0: no sequence error occurred"]
    B_0x0 = 0,
    #[doc = "1: a sequence error occurred"]
    B_0x1 = 1,
}
impl From<PGSERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGSERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSERR` reader - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR."]
pub type PGSERR_R = crate::BitReader<PGSERR_A>;
impl PGSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGSERR_A {
        match self.bits {
            false => PGSERR_A::B_0x0,
            true => PGSERR_A::B_0x1,
        }
    }
    #[doc = "no sequence error occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PGSERR_A::B_0x0
    }
    #[doc = "a sequence error occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PGSERR_A::B_0x1
    }
}
#[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRBERR_A {
    #[doc = "0: no strobe error occurred"]
    B_0x0 = 0,
    #[doc = "1: a strobe error occurred"]
    B_0x1 = 1,
}
impl From<STRBERR_A> for bool {
    #[inline(always)]
    fn from(variant: STRBERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRBERR` reader - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR."]
pub type STRBERR_R = crate::BitReader<STRBERR_A>;
impl STRBERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STRBERR_A {
        match self.bits {
            false => STRBERR_A::B_0x0,
            true => STRBERR_A::B_0x1,
        }
    }
    #[doc = "no strobe error occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STRBERR_A::B_0x0
    }
    #[doc = "a strobe error occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STRBERR_A::B_0x1
    }
}
#[doc = "inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears INCERR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INCERR_A {
    #[doc = "0: no inconsistency error occurs"]
    B_0x0 = 0,
    #[doc = "1: a inconsistency error occurs"]
    B_0x1 = 1,
}
impl From<INCERR_A> for bool {
    #[inline(always)]
    fn from(variant: INCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCERR` reader - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears INCERR."]
pub type INCERR_R = crate::BitReader<INCERR_A>;
impl INCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INCERR_A {
        match self.bits {
            false => INCERR_A::B_0x0,
            true => INCERR_A::B_0x1,
        }
    }
    #[doc = "no inconsistency error occurs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == INCERR_A::B_0x0
    }
    #[doc = "a inconsistency error occurs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == INCERR_A::B_0x1
    }
}
#[doc = "Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_CCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTCHANGEERR_A {
    #[doc = "0: no option byte change errors occurred"]
    B_0x0 = 0,
    #[doc = "1: one or more errors occurred during an option byte change operation."]
    B_0x1 = 1,
}
impl From<OPTCHANGEERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTCHANGEERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTCHANGEERR` reader - Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_CCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set."]
pub type OPTCHANGEERR_R = crate::BitReader<OPTCHANGEERR_A>;
impl OPTCHANGEERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTCHANGEERR_A {
        match self.bits {
            false => OPTCHANGEERR_A::B_0x0,
            true => OPTCHANGEERR_A::B_0x1,
        }
    }
    #[doc = "no option byte change errors occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPTCHANGEERR_A::B_0x0
    }
    #[doc = "one or more errors occurred during an option byte change operation."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPTCHANGEERR_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - busy flag BSY flag indicates that a Flash memory is busy by an operation (write, erase, option byte change). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
    #[inline(always)]
    pub fn BSY(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
    #[inline(always)]
    pub fn WBNE(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
    #[inline(always)]
    pub fn DBNE(&self) -> DBNE_R {
        DBNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register."]
    #[inline(always)]
    pub fn EOP(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR."]
    #[inline(always)]
    pub fn WRPERR(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR."]
    #[inline(always)]
    pub fn PGSERR(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR."]
    #[inline(always)]
    pub fn STRBERR(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears INCERR."]
    #[inline(always)]
    pub fn INCERR(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_CCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set."]
    #[inline(always)]
    pub fn OPTCHANGEERR(&self) -> OPTCHANGEERR_R {
        OPTCHANGEERR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "FLASH non-secure status register\n\nYou can [`read`](crate::Reg::read) this register and get [`nssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSSR_SPEC;
impl crate::RegisterSpec for NSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nssr::R`](R) reader structure"]
impl crate::Readable for NSSR_SPEC {}
#[doc = "`reset()` method sets NSSR to value 0"]
impl crate::Resettable for NSSR_SPEC {}
