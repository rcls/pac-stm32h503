#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DCNT_A {
    #[doc = "0: no data to transfer"]
    B_0x0 = 0,
    #[doc = "1: 1 byte"]
    B_0x1 = 1,
    #[doc = "2: 2 bytes"]
    B_0x2 = 2,
    #[doc = "65535: 64 Kbytes - 1 byte"]
    B_0xFFFF = 65535,
}
impl From<DCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: DCNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCNT_A {
    type Ux = u16;
}
impl crate::IsEnum for DCNT_A {}
#[doc = "Field `DCNT` writer - count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ..."]
pub type DCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, DCNT_A>;
impl<'a, REG> DCNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "no data to transfer"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DCNT_A::B_0x0)
    }
    #[doc = "1 byte"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DCNT_A::B_0x1)
    }
    #[doc = "2 bytes"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DCNT_A::B_0x2)
    }
    #[doc = "64 Kbytes - 1 byte"]
    #[inline(always)]
    pub fn B_0xFFFF(self) -> &'a mut crate::W<REG> {
        self.variant(DCNT_A::B_0xFFFF)
    }
}
#[doc = "read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNW_A {
    #[doc = "0: write message"]
    B_0x0 = 0,
    #[doc = "1: read message"]
    B_0x1 = 1,
}
impl From<RNW_A> for bool {
    #[inline(always)]
    fn from(variant: RNW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNW` writer - read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus."]
pub type RNW_W<'a, REG> = crate::BitWriter<'a, REG, RNW_A>;
impl<'a, REG> RNW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write message"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RNW_A::B_0x0)
    }
    #[doc = "read message"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RNW_A::B_0x1)
    }
}
#[doc = "Field `ADD` writer - 7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message)"]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "message type (whatever I3C is acting as controller/target) Bits\\[26:0\\] are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL 'stuck at' recovery. Bits\\[26:0\\] are ignored. If I3C_CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred private message is: {S / S+7'h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit static address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7'h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7'h02 addr + RnW=0 {S +} 7-bit I3C_DEVR0.DA\\[6:0\\] + RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit I3C_DEVR0.DA\\[6:0\\] + RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on I3C_BCR.BCR2) transmitted IBI payload data is defined by I3C_CR.DCNT\\[15:0\\] and must be consistently programmed vs the maximum IBI payload data size which is defined by I3C_IBIDR.IBIP\\[2:0\\]. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MTYPE_A {
    #[doc = "0: SCL output clock stops running until next control word is executed"]
    B_0x0_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER = 0,
    #[doc = "1: header message"]
    B_0x1_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER = 1,
    #[doc = "2: private message"]
    B_0x2_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER = 2,
    #[doc = "3: direct message (2nd part of an I3C SDR direct CCC command)"]
    B_0x3_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER = 3,
    #[doc = "4: legacy I2C message"]
    B_0x4_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER = 4,
    #[doc = "6: reserved (for this 1st alternate register description)"]
    B_0x6_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER = 6,
    #[doc = "8: hot-join request (W)"]
    B_0x8_WHEN_I3C_IS_ACTING_AS_I3C_TARGET = 8,
    #[doc = "9: controller-role request (W)"]
    B_0x9_WHEN_I3C_IS_ACTING_AS_I3C_TARGET = 9,
    #[doc = "10: IBI (in-band interrupt) request (R)"]
    B_0xA_WHEN_I3C_IS_ACTING_AS_I3C_TARGET = 10,
}
impl From<MTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: MTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MTYPE_A {
    type Ux = u8;
}
impl crate::IsEnum for MTYPE_A {}
#[doc = "Field `MTYPE` writer - message type (whatever I3C is acting as controller/target) Bits\\[26:0\\] are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL 'stuck at' recovery. Bits\\[26:0\\] are ignored. If I3C_CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred private message is: {S / S+7'h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit static address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7'h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7'h02 addr + RnW=0 {S +} 7-bit I3C_DEVR0.DA\\[6:0\\] + RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit I3C_DEVR0.DA\\[6:0\\] + RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on I3C_BCR.BCR2) transmitted IBI payload data is defined by I3C_CR.DCNT\\[15:0\\] and must be consistently programmed vs the maximum IBI payload data size which is defined by I3C_IBIDR.IBIP\\[2:0\\]. Others: reserved"]
pub type MTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MTYPE_A>;
impl<'a, REG> MTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCL output clock stops running until next control word is executed"]
    #[inline(always)]
    pub fn B_0x0_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0x0_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER)
    }
    #[doc = "header message"]
    #[inline(always)]
    pub fn B_0x1_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0x1_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER)
    }
    #[doc = "private message"]
    #[inline(always)]
    pub fn B_0x2_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0x2_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER)
    }
    #[doc = "direct message (2nd part of an I3C SDR direct CCC command)"]
    #[inline(always)]
    pub fn B_0x3_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0x3_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER)
    }
    #[doc = "legacy I2C message"]
    #[inline(always)]
    pub fn B_0x4_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0x4_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER)
    }
    #[doc = "reserved (for this 1st alternate register description)"]
    #[inline(always)]
    pub fn B_0x6_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0x6_WHEN_I3C_IS_ACTING_AS_I3C_CONTROLLER)
    }
    #[doc = "hot-join request (W)"]
    #[inline(always)]
    pub fn B_0x8_WHEN_I3C_IS_ACTING_AS_I3C_TARGET(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0x8_WHEN_I3C_IS_ACTING_AS_I3C_TARGET)
    }
    #[doc = "controller-role request (W)"]
    #[inline(always)]
    pub fn B_0x9_WHEN_I3C_IS_ACTING_AS_I3C_TARGET(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0x9_WHEN_I3C_IS_ACTING_AS_I3C_TARGET)
    }
    #[doc = "IBI (in-band interrupt) request (R)"]
    #[inline(always)]
    pub fn B_0xA_WHEN_I3C_IS_ACTING_AS_I3C_TARGET(self) -> &'a mut crate::W<REG> {
        self.variant(MTYPE_A::B_0xA_WHEN_I3C_IS_ACTING_AS_I3C_TARGET)
    }
}
#[doc = "message end type (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEND_A {
    #[doc = "0: this message from controller ends with a Repeated START (Sr)"]
    B_0x0 = 0,
    #[doc = "1: this message from controller ends with a STOP (P), being the last message of a frame"]
    B_0x1 = 1,
}
impl From<MEND_A> for bool {
    #[inline(always)]
    fn from(variant: MEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEND` writer - message end type (when the I3C is acting as controller)"]
pub type MEND_W<'a, REG> = crate::BitWriter<'a, REG, MEND_A>;
impl<'a, REG> MEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "this message from controller ends with a Repeated START (Sr)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MEND_A::B_0x0)
    }
    #[doc = "this message from controller ends with a STOP (P), being the last message of a frame"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MEND_A::B_0x1)
    }
}
impl W {
    #[doc = "Bits 0:15 - count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ..."]
    #[inline(always)]
    pub fn DCNT(&mut self) -> DCNT_W<'_, CR_SPEC> {
        DCNT_W::new(self, 0)
    }
    #[doc = "Bit 16 - read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus."]
    #[inline(always)]
    pub fn RNW(&mut self) -> RNW_W<'_, CR_SPEC> {
        RNW_W::new(self, 16)
    }
    #[doc = "Bits 17:23 - 7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message)"]
    #[inline(always)]
    pub fn ADD(&mut self) -> ADD_W<'_, CR_SPEC> {
        ADD_W::new(self, 17)
    }
    #[doc = "Bits 27:30 - message type (whatever I3C is acting as controller/target) Bits\\[26:0\\] are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL 'stuck at' recovery. Bits\\[26:0\\] are ignored. If I3C_CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred private message is: {S / S+7'h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\\[23:17\\] (ADD\\[6:0\\]) is the emitted 7-bit static address. Bit\\[16\\] (RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7'h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7'h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7'h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7'h02 addr + RnW=0 {S +} 7-bit I3C_DEVR0.DA\\[6:0\\] + RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit I3C_DEVR0.DA\\[6:0\\] + RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on I3C_BCR.BCR2) transmitted IBI payload data is defined by I3C_CR.DCNT\\[15:0\\] and must be consistently programmed vs the maximum IBI payload data size which is defined by I3C_IBIDR.IBIP\\[2:0\\]. Others: reserved"]
    #[inline(always)]
    pub fn MTYPE(&mut self) -> MTYPE_W<'_, CR_SPEC> {
        MTYPE_W::new(self, 27)
    }
    #[doc = "Bit 31 - message end type (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn MEND(&mut self) -> MEND_W<'_, CR_SPEC> {
        MEND_W::new(self, 31)
    }
}
#[doc = "I3C message control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {}
