#[doc = "Register `SIDR` reader"]
pub type R = crate::R<SIDR_SPEC>;
#[doc = "Size Identification SID\\[31:8\\]: fixed code that characterizes the ADC_SIDR register. This field is always read at 0xA3C5DD. SID\\[7:0\\]: read-only numeric field that returns the address offset (in Kbytes) of the identification registers from the IP base address:\n\nValue on reset: 2747653377"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SID_A {
    #[doc = "1: 1 Kbytes address offset"]
    B_0x1 = 1,
    #[doc = "2: 2 Kbytes address offset"]
    B_0x2 = 2,
    #[doc = "4: 4 Kbytes address offset"]
    B_0x4 = 4,
    #[doc = "8: 8 Kbytes address offset"]
    B_0x8 = 8,
}
impl From<SID_A> for u32 {
    #[inline(always)]
    fn from(variant: SID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SID_A {
    type Ux = u32;
}
impl crate::IsEnum for SID_A {}
#[doc = "Field `SID` reader - Size Identification SID\\[31:8\\]: fixed code that characterizes the ADC_SIDR register. This field is always read at 0xA3C5DD. SID\\[7:0\\]: read-only numeric field that returns the address offset (in Kbytes) of the identification registers from the IP base address:"]
pub type SID_R = crate::FieldReader<SID_A>;
impl SID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SID_A> {
        match self.bits {
            1 => Some(SID_A::B_0x1),
            2 => Some(SID_A::B_0x2),
            4 => Some(SID_A::B_0x4),
            8 => Some(SID_A::B_0x8),
            _ => None,
        }
    }
    #[doc = "1 Kbytes address offset"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SID_A::B_0x1
    }
    #[doc = "2 Kbytes address offset"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SID_A::B_0x2
    }
    #[doc = "4 Kbytes address offset"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SID_A::B_0x4
    }
    #[doc = "8 Kbytes address offset"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == SID_A::B_0x8
    }
}
impl R {
    #[doc = "Bits 0:31 - Size Identification SID\\[31:8\\]: fixed code that characterizes the ADC_SIDR register. This field is always read at 0xA3C5DD. SID\\[7:0\\]: read-only numeric field that returns the address offset (in Kbytes) of the identification registers from the IP base address:"]
    #[inline(always)]
    pub fn SID(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "ADC size identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIDR_SPEC;
impl crate::RegisterSpec for SIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidr::R`](R) reader structure"]
impl crate::Readable for SIDR_SPEC {}
#[doc = "`reset()` method sets SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for SIDR_SPEC {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
