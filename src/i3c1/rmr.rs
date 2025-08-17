#[doc = "Register `RMR` reader"]
pub type R = crate::R<RMR_SPEC>;
#[doc = "Field `IBIRDCNT` reader - IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register."]
pub type IBIRDCNT_R = crate::FieldReader;
#[doc = "Field `RCODE` reader - received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code."]
pub type RCODE_R = crate::FieldReader;
#[doc = "Field `RADD` reader - received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request."]
pub type RADD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the I3C_IBIDR register."]
    #[inline(always)]
    pub fn IBIRDCNT(&self) -> IBIRDCNT_R {
        IBIRDCNT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code."]
    #[inline(always)]
    pub fn RCODE(&self) -> RCODE_R {
        RCODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 17:23 - received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request."]
    #[inline(always)]
    pub fn RADD(&self) -> RADD_R {
        RADD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
#[doc = "I3C received message register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMR_SPEC;
impl crate::RegisterSpec for RMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmr::R`](R) reader structure"]
impl crate::Readable for RMR_SPEC {}
#[doc = "`reset()` method sets RMR to value 0"]
impl crate::Resettable for RMR_SPEC {}
