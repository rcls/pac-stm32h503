#[doc = "Register `HPMS` reader"]
pub type R = crate::R<HPMS_SPEC>;
#[doc = "Field `BIDX` reader - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\] = 1."]
pub type BIDX_R = crate::FieldReader;
#[doc = "Message storage indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSI_A {
    #[doc = "0: No FIFO selected"]
    B_0x0 = 0,
    #[doc = "1: FIFO overrun"]
    B_0x1 = 1,
    #[doc = "2: Message stored in FIFO 0"]
    B_0x2 = 2,
    #[doc = "3: Message stored in FIFO 1"]
    B_0x3 = 3,
}
impl From<MSI_A> for u8 {
    #[inline(always)]
    fn from(variant: MSI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSI_A {
    type Ux = u8;
}
impl crate::IsEnum for MSI_A {}
#[doc = "Field `MSI` reader - Message storage indicator"]
pub type MSI_R = crate::FieldReader<MSI_A>;
impl MSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSI_A {
        match self.bits {
            0 => MSI_A::B_0x0,
            1 => MSI_A::B_0x1,
            2 => MSI_A::B_0x2,
            3 => MSI_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No FIFO selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSI_A::B_0x0
    }
    #[doc = "FIFO overrun"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSI_A::B_0x1
    }
    #[doc = "Message stored in FIFO 0"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MSI_A::B_0x2
    }
    #[doc = "Message stored in FIFO 1"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MSI_A::B_0x3
    }
}
#[doc = "Field `FIDX` reader - Filter index Index of matching filter element. Range is 0 to RXGFC\\[LSS\\] - 1 or RXGFC\\[LSE\\] - 1."]
pub type FIDX_R = crate::FieldReader;
#[doc = "Filter list Indicates the filter list of the matching filter element.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLST_A {
    #[doc = "0: Standard filter list"]
    B_0x0 = 0,
    #[doc = "1: Extended filter list"]
    B_0x1 = 1,
}
impl From<FLST_A> for bool {
    #[inline(always)]
    fn from(variant: FLST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLST` reader - Filter list Indicates the filter list of the matching filter element."]
pub type FLST_R = crate::BitReader<FLST_A>;
impl FLST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLST_A {
        match self.bits {
            false => FLST_A::B_0x0,
            true => FLST_A::B_0x1,
        }
    }
    #[doc = "Standard filter list"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FLST_A::B_0x0
    }
    #[doc = "Extended filter list"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FLST_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:2 - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\] = 1."]
    #[inline(always)]
    pub fn BIDX(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:7 - Message storage indicator"]
    #[inline(always)]
    pub fn MSI(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Filter index Index of matching filter element. Range is 0 to RXGFC\\[LSS\\] - 1 or RXGFC\\[LSE\\] - 1."]
    #[inline(always)]
    pub fn FIDX(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Filter list Indicates the filter list of the matching filter element."]
    #[inline(always)]
    pub fn FLST(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "FDCAN high-priority message status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPMS_SPEC;
impl crate::RegisterSpec for HPMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpms::R`](R) reader structure"]
impl crate::Readable for HPMS_SPEC {}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HPMS_SPEC {}
