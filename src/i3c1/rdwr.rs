#[doc = "Register `RDWR` reader"]
pub type R = crate::R<RDWR_SPEC>;
#[doc = "Field `RDB0` reader - 8-bit received data (earliest byte on I3C bus)."]
pub type RDB0_R = crate::FieldReader;
#[doc = "Field `RDB1` reader - 8-bit received data (next byte after RDB0 on I3C bus)."]
pub type RDB1_R = crate::FieldReader;
#[doc = "Field `RDB2` reader - 8-bit received data (next byte after RDB1 on I3C bus)."]
pub type RDB2_R = crate::FieldReader;
#[doc = "Field `RDB3` reader - 8-bit received data (latest byte on I3C bus)."]
pub type RDB3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit received data (earliest byte on I3C bus)."]
    #[inline(always)]
    pub fn RDB0(&self) -> RDB0_R {
        RDB0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit received data (next byte after RDB0 on I3C bus)."]
    #[inline(always)]
    pub fn RDB1(&self) -> RDB1_R {
        RDB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit received data (next byte after RDB1 on I3C bus)."]
    #[inline(always)]
    pub fn RDB2(&self) -> RDB2_R {
        RDB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit received data (latest byte on I3C bus)."]
    #[inline(always)]
    pub fn RDB3(&self) -> RDB3_R {
        RDB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "I3C receive data word register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdwr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDWR_SPEC;
impl crate::RegisterSpec for RDWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdwr::R`](R) reader structure"]
impl crate::Readable for RDWR_SPEC {}
#[doc = "`reset()` method sets RDWR to value 0"]
impl crate::Resettable for RDWR_SPEC {}
