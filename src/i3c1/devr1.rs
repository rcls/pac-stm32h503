#[doc = "Register `DEVR1` reader"]
pub type R = crate::R<DEVR1_SPEC>;
#[doc = "Register `DEVR1` writer"]
pub type W = crate::W<DEVR1_SPEC>;
#[doc = "Field `DA` reader - assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
pub type DA_R = crate::FieldReader;
#[doc = "Field `DA` writer - assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIACK_A {
    #[doc = "0: an IBI request from target x is to be NACKed"]
    B_0x0 = 0,
    #[doc = "1: an IBI request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
    B_0x1 = 1,
}
impl From<IBIACK_A> for bool {
    #[inline(always)]
    fn from(variant: IBIACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBIACK` reader - IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared."]
pub type IBIACK_R = crate::BitReader<IBIACK_A>;
impl IBIACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBIACK_A {
        match self.bits {
            false => IBIACK_A::B_0x0,
            true => IBIACK_A::B_0x1,
        }
    }
    #[doc = "an IBI request from target x is to be NACKed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IBIACK_A::B_0x0
    }
    #[doc = "an IBI request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IBIACK_A::B_0x1
    }
}
#[doc = "Field `IBIACK` writer - IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared."]
pub type IBIACK_W<'a, REG> = crate::BitWriter<'a, REG, IBIACK_A>;
impl<'a, REG> IBIACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "an IBI request from target x is to be NACKed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IBIACK_A::B_0x0)
    }
    #[doc = "an IBI request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IBIACK_A::B_0x1)
    }
}
#[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRACK_A {
    #[doc = "0: a controller-role request from target x is to be NACKed"]
    B_0x0 = 0,
    #[doc = "1: a controller-role request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
    B_0x1 = 1,
}
impl From<CRACK_A> for bool {
    #[inline(always)]
    fn from(variant: CRACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRACK` reader - controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared."]
pub type CRACK_R = crate::BitReader<CRACK_A>;
impl CRACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRACK_A {
        match self.bits {
            false => CRACK_A::B_0x0,
            true => CRACK_A::B_0x1,
        }
    }
    #[doc = "a controller-role request from target x is to be NACKed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRACK_A::B_0x0
    }
    #[doc = "a controller-role request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRACK_A::B_0x1
    }
}
#[doc = "Field `CRACK` writer - controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared."]
pub type CRACK_W<'a, REG> = crate::BitWriter<'a, REG, CRACK_A>;
impl<'a, REG> CRACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a controller-role request from target x is to be NACKed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRACK_A::B_0x0)
    }
    #[doc = "a controller-role request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRACK_A::B_0x1)
    }
}
#[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\] bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIDEN_A {
    #[doc = "0: no data byte follows the acknowledged IBI from target x"]
    B_0x0 = 0,
    #[doc = "1: the mandatory data byte MDB\\[7:0\\] follows the acknowledged IBI from target x"]
    B_0x1 = 1,
}
impl From<IBIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: IBIDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBIDEN` reader - IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\] bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
pub type IBIDEN_R = crate::BitReader<IBIDEN_A>;
impl IBIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBIDEN_A {
        match self.bits {
            false => IBIDEN_A::B_0x0,
            true => IBIDEN_A::B_0x1,
        }
    }
    #[doc = "no data byte follows the acknowledged IBI from target x"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IBIDEN_A::B_0x0
    }
    #[doc = "the mandatory data byte MDB\\[7:0\\] follows the acknowledged IBI from target x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IBIDEN_A::B_0x1
    }
}
#[doc = "Field `IBIDEN` writer - IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\] bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
pub type IBIDEN_W<'a, REG> = crate::BitWriter<'a, REG, IBIDEN_A>;
impl<'a, REG> IBIDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no data byte follows the acknowledged IBI from target x"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IBIDEN_A::B_0x0)
    }
    #[doc = "the mandatory data byte MDB\\[7:0\\] follows the acknowledged IBI from target x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IBIDEN_A::B_0x1)
    }
}
#[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3'b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP_A {
    #[doc = "0: I3C transfer is not stopped and C-FIFO is not flushed"]
    B_0x0 = 0,
    #[doc = "1: I3C transfer is stopped and C-FIFO is flushed on a received IBI request from target x"]
    B_0x1 = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3'b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
pub type SUSP_R = crate::BitReader<SUSP_A>;
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::B_0x0,
            true => SUSP_A::B_0x1,
        }
    }
    #[doc = "I3C transfer is not stopped and C-FIFO is not flushed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SUSP_A::B_0x0
    }
    #[doc = "I3C transfer is stopped and C-FIFO is flushed on a received IBI request from target x"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SUSP_A::B_0x1
    }
}
#[doc = "Field `SUSP` writer - suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3'b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG, SUSP_A>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I3C transfer is not stopped and C-FIFO is not flushed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP_A::B_0x0)
    }
    #[doc = "I3C transfer is stopped and C-FIFO is flushed on a received IBI request from target x"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP_A::B_0x1)
    }
}
#[doc = "DA\\[6:0\\] write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\] and IBIDEN values. Then, to be able to next modify DA\\[6:0\\] or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\] or IBIDEN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_A {
    #[doc = "0: write to I3C_DEVRx.DA\\[7:0\\] and to I3C_DEVRx.IBIDEN is allowed"]
    B_0x0 = 0,
    #[doc = "1: write I3C_DEVRx.DA\\[7:0\\] and to I3C_DEVRx.IBIDEN is disabled/locked"]
    B_0x1 = 1,
}
impl From<DIS_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS` reader - DA\\[6:0\\] write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\] and IBIDEN values. Then, to be able to next modify DA\\[6:0\\] or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\] or IBIDEN."]
pub type DIS_R = crate::BitReader<DIS_A>;
impl DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIS_A {
        match self.bits {
            false => DIS_A::B_0x0,
            true => DIS_A::B_0x1,
        }
    }
    #[doc = "write to I3C_DEVRx.DA\\[7:0\\] and to I3C_DEVRx.IBIDEN is allowed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DIS_A::B_0x0
    }
    #[doc = "write I3C_DEVRx.DA\\[7:0\\] and to I3C_DEVRx.IBIDEN is disabled/locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DIS_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 1:7 - assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
    #[inline(always)]
    pub fn DA(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared."]
    #[inline(always)]
    pub fn IBIACK(&self) -> IBIACK_R {
        IBIACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared."]
    #[inline(always)]
    pub fn CRACK(&self) -> CRACK_R {
        CRACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\] bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
    #[inline(always)]
    pub fn IBIDEN(&self) -> IBIDEN_R {
        IBIDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3'b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
    #[inline(always)]
    pub fn SUSP(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - DA\\[6:0\\] write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\] and IBIDEN values. Then, to be able to next modify DA\\[6:0\\] or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\] or IBIDEN."]
    #[inline(always)]
    pub fn DIS(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
    #[inline(always)]
    pub fn DA(&mut self) -> DA_W<'_, DEVR1_SPEC> {
        DA_W::new(self, 1)
    }
    #[doc = "Bit 16 - IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared."]
    #[inline(always)]
    pub fn IBIACK(&mut self) -> IBIACK_W<'_, DEVR1_SPEC> {
        IBIACK_W::new(self, 16)
    }
    #[doc = "Bit 17 - controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\] from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\] into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared."]
    #[inline(always)]
    pub fn CRACK(&mut self) -> CRACK_W<'_, DEVR1_SPEC> {
        CRACK_W::new(self, 17)
    }
    #[doc = "Bit 18 - IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\] bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1."]
    #[inline(always)]
    pub fn IBIDEN(&mut self) -> IBIDEN_W<'_, DEVR1_SPEC> {
        IBIDEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3'b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
    #[inline(always)]
    pub fn SUSP(&mut self) -> SUSP_W<'_, DEVR1_SPEC> {
        SUSP_W::new(self, 19)
    }
}
#[doc = "I3C device 1 characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`devr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVR1_SPEC;
impl crate::RegisterSpec for DEVR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devr1::R`](R) reader structure"]
impl crate::Readable for DEVR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devr1::W`](W) writer structure"]
impl crate::Writable for DEVR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DEVR1 to value 0"]
impl crate::Resettable for DEVR1_SPEC {}
