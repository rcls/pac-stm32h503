#[doc = "Register `OPSR` reader"]
pub type R = crate::R<OPSR_SPEC>;
#[doc = "Field `ADDR_OP` reader - Interrupted operation address."]
pub type ADDR_OP_R = crate::FieldReader<u32>;
#[doc = "Field `BK_OP` reader - Interrupted operation bank It indicates which bank was concerned by operation."]
pub type BK_OP_R = crate::BitReader;
#[doc = "Field `SYSF_OP` reader - Operation in system Flash memory interrupted Indicates that reset interrupted an ongoing operation in System Flash."]
pub type SYSF_OP_R = crate::BitReader;
#[doc = "Field `OTP_OP` reader - OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area."]
pub type OTP_OP_R = crate::BitReader;
#[doc = "Flash memory operation code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CODE_OP_A {
    #[doc = "0: No Flash operation on going during previous reset"]
    B_0x0 = 0,
    #[doc = "1: Single write operation interrupted"]
    B_0x1 = 1,
    #[doc = "3: Sector erase operation interrupted"]
    B_0x3 = 3,
    #[doc = "4: Bank erase operation interrupted"]
    B_0x4 = 4,
    #[doc = "5: Mass erase operation interrupted"]
    B_0x5 = 5,
    #[doc = "6: Option change operation interrupted"]
    B_0x6 = 6,
}
impl From<CODE_OP_A> for u8 {
    #[inline(always)]
    fn from(variant: CODE_OP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CODE_OP_A {
    type Ux = u8;
}
impl crate::IsEnum for CODE_OP_A {}
#[doc = "Field `CODE_OP` reader - Flash memory operation code"]
pub type CODE_OP_R = crate::FieldReader<CODE_OP_A>;
impl CODE_OP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CODE_OP_A> {
        match self.bits {
            0 => Some(CODE_OP_A::B_0x0),
            1 => Some(CODE_OP_A::B_0x1),
            3 => Some(CODE_OP_A::B_0x3),
            4 => Some(CODE_OP_A::B_0x4),
            5 => Some(CODE_OP_A::B_0x5),
            6 => Some(CODE_OP_A::B_0x6),
            _ => None,
        }
    }
    #[doc = "No Flash operation on going during previous reset"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CODE_OP_A::B_0x0
    }
    #[doc = "Single write operation interrupted"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CODE_OP_A::B_0x1
    }
    #[doc = "Sector erase operation interrupted"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CODE_OP_A::B_0x3
    }
    #[doc = "Bank erase operation interrupted"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == CODE_OP_A::B_0x4
    }
    #[doc = "Mass erase operation interrupted"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == CODE_OP_A::B_0x5
    }
    #[doc = "Option change operation interrupted"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == CODE_OP_A::B_0x6
    }
}
impl R {
    #[doc = "Bits 0:19 - Interrupted operation address."]
    #[inline(always)]
    pub fn ADDR_OP(&self) -> ADDR_OP_R {
        ADDR_OP_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 22 - Interrupted operation bank It indicates which bank was concerned by operation."]
    #[inline(always)]
    pub fn BK_OP(&self) -> BK_OP_R {
        BK_OP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Operation in system Flash memory interrupted Indicates that reset interrupted an ongoing operation in System Flash."]
    #[inline(always)]
    pub fn SYSF_OP(&self) -> SYSF_OP_R {
        SYSF_OP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area."]
    #[inline(always)]
    pub fn OTP_OP(&self) -> OTP_OP_R {
        OTP_OP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Flash memory operation code"]
    #[inline(always)]
    pub fn CODE_OP(&self) -> CODE_OP_R {
        CODE_OP_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "FLASH operation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPSR_SPEC;
impl crate::RegisterSpec for OPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opsr::R`](R) reader structure"]
impl crate::Readable for OPSR_SPEC {}
#[doc = "`reset()` method sets OPSR to value 0"]
impl crate::Resettable for OPSR_SPEC {}
