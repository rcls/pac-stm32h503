#[doc = "Register `GETMXDSR` reader"]
pub type R = crate::R<GETMXDSR_SPEC>;
#[doc = "Register `GETMXDSR` writer"]
pub type W = crate::W<GETMXDSR_SPEC>;
#[doc = "controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HOFFAS_A {
    #[doc = "0: activity state 0 is the initial activity state of this I3C before and when becoming controller"]
    B_0x0 = 0,
    #[doc = "1: activity state 1 is the initial activity state of this I3C when becoming controller"]
    B_0x1 = 1,
    #[doc = "2: activity state 2 is the initial activity state of this I3C when becoming controller"]
    B_0x2 = 2,
    #[doc = "3: activity state 3 is the initial activity state of this I3C when becoming controller"]
    B_0x3 = 3,
}
impl From<HOFFAS_A> for u8 {
    #[inline(always)]
    fn from(variant: HOFFAS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HOFFAS_A {
    type Ux = u8;
}
impl crate::IsEnum for HOFFAS_A {}
#[doc = "Field `HOFFAS` reader - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
pub type HOFFAS_R = crate::FieldReader<HOFFAS_A>;
impl HOFFAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HOFFAS_A {
        match self.bits {
            0 => HOFFAS_A::B_0x0,
            1 => HOFFAS_A::B_0x1,
            2 => HOFFAS_A::B_0x2,
            3 => HOFFAS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "activity state 0 is the initial activity state of this I3C before and when becoming controller"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HOFFAS_A::B_0x0
    }
    #[doc = "activity state 1 is the initial activity state of this I3C when becoming controller"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HOFFAS_A::B_0x1
    }
    #[doc = "activity state 2 is the initial activity state of this I3C when becoming controller"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == HOFFAS_A::B_0x2
    }
    #[doc = "activity state 3 is the initial activity state of this I3C when becoming controller"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == HOFFAS_A::B_0x3
    }
}
#[doc = "Field `HOFFAS` writer - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
pub type HOFFAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HOFFAS_A, crate::Safe>;
impl<'a, REG> HOFFAS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "activity state 0 is the initial activity state of this I3C before and when becoming controller"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HOFFAS_A::B_0x0)
    }
    #[doc = "activity state 1 is the initial activity state of this I3C when becoming controller"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HOFFAS_A::B_0x1)
    }
    #[doc = "activity state 2 is the initial activity state of this I3C when becoming controller"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(HOFFAS_A::B_0x2)
    }
    #[doc = "activity state 3 is the initial activity state of this I3C when becoming controller"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(HOFFAS_A::B_0x3)
    }
}
#[doc = "GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\] and LSB=0. - Max read turnaround time is between 256 s and 65535 s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 s and 16 s.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMT_A {
    #[doc = "0: format 1 (2 bytes with MaxWr with no defining byte, MaxRd)"]
    B_0x0 = 0,
    #[doc = "1: format 2: (5 bytes with MaxWr with no defining byte, MaxRd, MaxRdTurn)"]
    B_0x1 = 1,
    #[doc = "2: format 2 (5 bytes with MaxWr with no defining byte, MaxRd, and middle byte of MaxRdTurn)"]
    B_0x2 = 2,
    #[doc = "3: format 2 (5 bytes with MaxWr with no defining byte, MaxRd, MSB of MaxRdTurn)"]
    B_0x3 = 3,
}
impl From<FMT_A> for u8 {
    #[inline(always)]
    fn from(variant: FMT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMT_A {
    type Ux = u8;
}
impl crate::IsEnum for FMT_A {}
#[doc = "Field `FMT` reader - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\] and LSB=0. - Max read turnaround time is between 256 s and 65535 s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 s and 16 s."]
pub type FMT_R = crate::FieldReader<FMT_A>;
impl FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMT_A {
        match self.bits {
            0 => FMT_A::B_0x0,
            1 => FMT_A::B_0x1,
            2 => FMT_A::B_0x2,
            3 => FMT_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "format 1 (2 bytes with MaxWr with no defining byte, MaxRd)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FMT_A::B_0x0
    }
    #[doc = "format 2: (5 bytes with MaxWr with no defining byte, MaxRd, MaxRdTurn)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FMT_A::B_0x1
    }
    #[doc = "format 2 (5 bytes with MaxWr with no defining byte, MaxRd, and middle byte of MaxRdTurn)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == FMT_A::B_0x2
    }
    #[doc = "format 2 (5 bytes with MaxWr with no defining byte, MaxRd, MSB of MaxRdTurn)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == FMT_A::B_0x3
    }
}
#[doc = "Field `FMT` writer - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\] and LSB=0. - Max read turnaround time is between 256 s and 65535 s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 s and 16 s."]
pub type FMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FMT_A, crate::Safe>;
impl<'a, REG> FMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "format 1 (2 bytes with MaxWr with no defining byte, MaxRd)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FMT_A::B_0x0)
    }
    #[doc = "format 2: (5 bytes with MaxWr with no defining byte, MaxRd, MaxRdTurn)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FMT_A::B_0x1)
    }
    #[doc = "format 2 (5 bytes with MaxWr with no defining byte, MaxRd, and middle byte of MaxRdTurn)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(FMT_A::B_0x2)
    }
    #[doc = "format 2 (5 bytes with MaxWr with no defining byte, MaxRd, MSB of MaxRdTurn)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(FMT_A::B_0x3)
    }
}
#[doc = "Field `RDTURN` reader - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\] field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
pub type RDTURN_R = crate::FieldReader;
#[doc = "Field `RDTURN` writer - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\] field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
pub type RDTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\] bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCO_A {
    #[doc = "0: tSCO = 12 ns"]
    B_0x0 = 0,
    #[doc = "1: tSCO 12 ns (and refer to the datasheet for more details)"]
    B_0x1 = 1,
}
impl From<TSCO_A> for bool {
    #[inline(always)]
    fn from(variant: TSCO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSCO` reader - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\] bits."]
pub type TSCO_R = crate::BitReader<TSCO_A>;
impl TSCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSCO_A {
        match self.bits {
            false => TSCO_A::B_0x0,
            true => TSCO_A::B_0x1,
        }
    }
    #[doc = "tSCO = 12 ns"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSCO_A::B_0x0
    }
    #[doc = "tSCO 12 ns (and refer to the datasheet for more details)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSCO_A::B_0x1
    }
}
#[doc = "Field `TSCO` writer - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\] bits."]
pub type TSCO_W<'a, REG> = crate::BitWriter<'a, REG, TSCO_A>;
impl<'a, REG> TSCO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tSCO = 12 ns"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSCO_A::B_0x0)
    }
    #[doc = "tSCO 12 ns (and refer to the datasheet for more details)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSCO_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:1 - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
    #[inline(always)]
    pub fn HOFFAS(&self) -> HOFFAS_R {
        HOFFAS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\] and LSB=0. - Max read turnaround time is between 256 s and 65535 s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 s and 16 s."]
    #[inline(always)]
    pub fn FMT(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\] field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
    #[inline(always)]
    pub fn RDTURN(&self) -> RDTURN_R {
        RDTURN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\] bits."]
    #[inline(always)]
    pub fn TSCO(&self) -> TSCO_R {
        TSCO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
    #[inline(always)]
    pub fn HOFFAS(&mut self) -> HOFFAS_W<'_, GETMXDSR_SPEC> {
        HOFFAS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\] and LSB=0. - Max read turnaround time is between 256 s and 65535 s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 s and 16 s."]
    #[inline(always)]
    pub fn FMT(&mut self) -> FMT_W<'_, GETMXDSR_SPEC> {
        FMT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\] field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
    #[inline(always)]
    pub fn RDTURN(&mut self) -> RDTURN_W<'_, GETMXDSR_SPEC> {
        RDTURN_W::new(self, 16)
    }
    #[doc = "Bit 24 - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\] bits."]
    #[inline(always)]
    pub fn TSCO(&mut self) -> TSCO_W<'_, GETMXDSR_SPEC> {
        TSCO_W::new(self, 24)
    }
}
#[doc = "I3C get capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`getmxdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getmxdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GETMXDSR_SPEC;
impl crate::RegisterSpec for GETMXDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`getmxdsr::R`](R) reader structure"]
impl crate::Readable for GETMXDSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`getmxdsr::W`](W) writer structure"]
impl crate::Writable for GETMXDSR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets GETMXDSR to value 0"]
impl crate::Resettable for GETMXDSR_SPEC {}
