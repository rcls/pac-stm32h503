#[doc = "Register `TIMINGR1` reader"]
pub type R = crate::R<TIMINGR1_SPEC>;
#[doc = "Register `TIMINGR1` writer"]
pub type W = crate::W<TIMINGR1_SPEC>;
#[doc = "Field `AVAL` reader - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 s. This timing is defined by: tAVAL = (AVAL\\[7:0\\] + 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 s . This timing is defined by: tIDLE = (AVAL\\[7:0\\] + 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\\[7:0\\] + 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 s. This timing is defined by: tSTALL = (AVAL\\[7:0\\] + 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\\[1:0\\], as follows: If ASNCR\\[1:0\\]=00: tNEWCRLock = (AVAL\\[7:0\\] + 1) x tI3CCLK If ASNCR\\[1:0\\]=01: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 100 x tI3CCLK If ASNCR\\[1:0\\]=10: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 2000 x tI3CCLK If ASNCR\\[1:0\\]=11: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 50000 x tI3CCLK"]
pub type AVAL_R = crate::FieldReader;
#[doc = "Field `AVAL` writer - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 s. This timing is defined by: tAVAL = (AVAL\\[7:0\\] + 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 s . This timing is defined by: tIDLE = (AVAL\\[7:0\\] + 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\\[7:0\\] + 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 s. This timing is defined by: tSTALL = (AVAL\\[7:0\\] + 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\\[1:0\\], as follows: If ASNCR\\[1:0\\]=00: tNEWCRLock = (AVAL\\[7:0\\] + 1) x tI3CCLK If ASNCR\\[1:0\\]=01: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 100 x tI3CCLK If ASNCR\\[1:0\\]=10: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 2000 x tI3CCLK If ASNCR\\[1:0\\]=11: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 50000 x tI3CCLK"]
pub type AVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ASNCR` reader - activity state of the new controller (when I3C is acting as active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\\[7:0\\]. This field can be modified only when the I3C is acting as controller."]
pub type ASNCR_R = crate::FieldReader;
#[doc = "Field `ASNCR` writer - activity state of the new controller (when I3C is acting as active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\\[7:0\\]. This field can be modified only when the I3C is acting as controller."]
pub type ASNCR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREE` reader - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller)"]
pub type FREE_R = crate::FieldReader;
#[doc = "Field `FREE` writer - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller)"]
pub type FREE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDA_HD_A {
    #[doc = "0: SDA hold time = 0,5 x tI3CCLK"]
    B_0x0 = 0,
    #[doc = "1: SDA hold time = 1,5 x tI3CCLK"]
    B_0x1 = 1,
}
impl From<SDA_HD_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_HD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDA_HD` reader - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):"]
pub type SDA_HD_R = crate::BitReader<SDA_HD_A>;
impl SDA_HD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDA_HD_A {
        match self.bits {
            false => SDA_HD_A::B_0x0,
            true => SDA_HD_A::B_0x1,
        }
    }
    #[doc = "SDA hold time = 0,5 x tI3CCLK"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SDA_HD_A::B_0x0
    }
    #[doc = "SDA hold time = 1,5 x tI3CCLK"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SDA_HD_A::B_0x1
    }
}
#[doc = "Field `SDA_HD` writer - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):"]
pub type SDA_HD_W<'a, REG> = crate::BitWriter<'a, REG, SDA_HD_A>;
impl<'a, REG> SDA_HD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDA hold time = 0,5 x tI3CCLK"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SDA_HD_A::B_0x0)
    }
    #[doc = "SDA hold time = 1,5 x tI3CCLK"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SDA_HD_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:7 - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 s. This timing is defined by: tAVAL = (AVAL\\[7:0\\] + 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 s . This timing is defined by: tIDLE = (AVAL\\[7:0\\] + 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\\[7:0\\] + 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 s. This timing is defined by: tSTALL = (AVAL\\[7:0\\] + 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\\[1:0\\], as follows: If ASNCR\\[1:0\\]=00: tNEWCRLock = (AVAL\\[7:0\\] + 1) x tI3CCLK If ASNCR\\[1:0\\]=01: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 100 x tI3CCLK If ASNCR\\[1:0\\]=10: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 2000 x tI3CCLK If ASNCR\\[1:0\\]=11: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 50000 x tI3CCLK"]
    #[inline(always)]
    pub fn AVAL(&self) -> AVAL_R {
        AVAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - activity state of the new controller (when I3C is acting as active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\\[7:0\\]. This field can be modified only when the I3C is acting as controller."]
    #[inline(always)]
    pub fn ASNCR(&self) -> ASNCR_R {
        ASNCR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:22 - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn FREE(&self) -> FREE_R {
        FREE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):"]
    #[inline(always)]
    pub fn SDA_HD(&self) -> SDA_HD_R {
        SDA_HD_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 s. This timing is defined by: tAVAL = (AVAL\\[7:0\\] + 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 s . This timing is defined by: tIDLE = (AVAL\\[7:0\\] + 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\\[7:0\\] + 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 s. This timing is defined by: tSTALL = (AVAL\\[7:0\\] + 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\\[1:0\\], as follows: If ASNCR\\[1:0\\]=00: tNEWCRLock = (AVAL\\[7:0\\] + 1) x tI3CCLK If ASNCR\\[1:0\\]=01: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 100 x tI3CCLK If ASNCR\\[1:0\\]=10: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 2000 x tI3CCLK If ASNCR\\[1:0\\]=11: tNEWCRLock = (AVAL\\[7:0\\] + 1) x 50000 x tI3CCLK"]
    #[inline(always)]
    pub fn AVAL(&mut self) -> AVAL_W<'_, TIMINGR1_SPEC> {
        AVAL_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - activity state of the new controller (when I3C is acting as active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\\[7:0\\]. This field can be modified only when the I3C is acting as controller."]
    #[inline(always)]
    pub fn ASNCR(&mut self) -> ASNCR_W<'_, TIMINGR1_SPEC> {
        ASNCR_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn FREE(&mut self) -> FREE_W<'_, TIMINGR1_SPEC> {
        FREE_W::new(self, 16)
    }
    #[doc = "Bit 28 - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):"]
    #[inline(always)]
    pub fn SDA_HD(&mut self) -> SDA_HD_W<'_, TIMINGR1_SPEC> {
        SDA_HD_W::new(self, 28)
    }
}
#[doc = "I3C timing register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMINGR1_SPEC;
impl crate::RegisterSpec for TIMINGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timingr1::R`](R) reader structure"]
impl crate::Readable for TIMINGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timingr1::W`](W) writer structure"]
impl crate::Writable for TIMINGR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TIMINGR1 to value 0"]
impl crate::Resettable for TIMINGR1_SPEC {}
