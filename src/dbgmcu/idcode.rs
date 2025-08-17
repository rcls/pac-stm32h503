#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IDCODE_SPEC>;
#[doc = "device identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DEV_ID_A {
    #[doc = "1140: STM32H503"]
    B_0x474 = 1140,
}
impl From<DEV_ID_A> for u16 {
    #[inline(always)]
    fn from(variant: DEV_ID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEV_ID_A {
    type Ux = u16;
}
impl crate::IsEnum for DEV_ID_A {}
#[doc = "Field `DEV_ID` reader - device identification"]
pub type DEV_ID_R = crate::FieldReader<DEV_ID_A>;
impl DEV_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DEV_ID_A> {
        match self.bits {
            1140 => Some(DEV_ID_A::B_0x474),
            _ => None,
        }
    }
    #[doc = "STM32H503"]
    #[inline(always)]
    pub fn is_B_0x474(&self) -> bool {
        *self == DEV_ID_A::B_0x474
    }
}
#[doc = "revision This field indicates the revision of the device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum REV_ID_A {
    #[doc = "4096: Revision A"]
    B_0x1000 = 4096,
}
impl From<REV_ID_A> for u16 {
    #[inline(always)]
    fn from(variant: REV_ID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV_ID_A {
    type Ux = u16;
}
impl crate::IsEnum for REV_ID_A {}
#[doc = "Field `REV_ID` reader - revision This field indicates the revision of the device."]
pub type REV_ID_R = crate::FieldReader<REV_ID_A>;
impl REV_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV_ID_A> {
        match self.bits {
            4096 => Some(REV_ID_A::B_0x1000),
            _ => None,
        }
    }
    #[doc = "Revision A"]
    #[inline(always)]
    pub fn is_B_0x1000(&self) -> bool {
        *self == REV_ID_A::B_0x1000
    }
}
impl R {
    #[doc = "Bits 0:11 - device identification"]
    #[inline(always)]
    pub fn DEV_ID(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - revision This field indicates the revision of the device."]
    #[inline(always)]
    pub fn REV_ID(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBGMCU identity code register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDCODE_SPEC;
impl crate::RegisterSpec for IDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IDCODE_SPEC {}
#[doc = "`reset()` method sets IDCODE to value 0x6000"]
impl crate::Resettable for IDCODE_SPEC {
    const RESET_VALUE: u32 = 0x6000;
}
