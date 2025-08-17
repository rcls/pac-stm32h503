#[doc = "Register `SER` reader"]
pub type R = crate::R<SER_SPEC>;
#[doc = "protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7'hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7'hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7'hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CODERR_A {
    #[doc = "0: CE0 error (transaction after sending CCC):"]
    B_0x0 = 0,
    #[doc = "1: CE1 error (monitoring error):"]
    B_0x1 = 1,
    #[doc = "2: CE2 error (no response to broadcast address):"]
    B_0x2 = 2,
    #[doc = "3: CE3 error (failed controller-role hand-off):"]
    B_0x3 = 3,
    #[doc = "8: TE0 error (invalid broadcast address 7'hE+W):"]
    B_0x8 = 8,
    #[doc = "9: TE1 error (CCC code):"]
    B_0x9 = 9,
    #[doc = "10: TE2 error (write data):"]
    B_0xA = 10,
    #[doc = "11: TE3 error (assigned address during dynamic address arbitration):"]
    B_0xB = 11,
    #[doc = "12: TE4 error (7'hE+R missing after Sr during dynamic address arbitration):"]
    B_0xC = 12,
    #[doc = "13: TE5 error (transaction after detecting CCC):"]
    B_0xD = 13,
    #[doc = "14: TE6 error (monitoring error):"]
    B_0xE = 14,
}
impl From<CODERR_A> for u8 {
    #[inline(always)]
    fn from(variant: CODERR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CODERR_A {
    type Ux = u8;
}
impl crate::IsEnum for CODERR_A {}
#[doc = "Field `CODERR` reader - protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7'hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7'hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7'hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved"]
pub type CODERR_R = crate::FieldReader<CODERR_A>;
impl CODERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CODERR_A> {
        match self.bits {
            0 => Some(CODERR_A::B_0x0),
            1 => Some(CODERR_A::B_0x1),
            2 => Some(CODERR_A::B_0x2),
            3 => Some(CODERR_A::B_0x3),
            8 => Some(CODERR_A::B_0x8),
            9 => Some(CODERR_A::B_0x9),
            10 => Some(CODERR_A::B_0xA),
            11 => Some(CODERR_A::B_0xB),
            12 => Some(CODERR_A::B_0xC),
            13 => Some(CODERR_A::B_0xD),
            14 => Some(CODERR_A::B_0xE),
            _ => None,
        }
    }
    #[doc = "CE0 error (transaction after sending CCC):"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CODERR_A::B_0x0
    }
    #[doc = "CE1 error (monitoring error):"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CODERR_A::B_0x1
    }
    #[doc = "CE2 error (no response to broadcast address):"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CODERR_A::B_0x2
    }
    #[doc = "CE3 error (failed controller-role hand-off):"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CODERR_A::B_0x3
    }
    #[doc = "TE0 error (invalid broadcast address 7'hE+W):"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == CODERR_A::B_0x8
    }
    #[doc = "TE1 error (CCC code):"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == CODERR_A::B_0x9
    }
    #[doc = "TE2 error (write data):"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == CODERR_A::B_0xA
    }
    #[doc = "TE3 error (assigned address during dynamic address arbitration):"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == CODERR_A::B_0xB
    }
    #[doc = "TE4 error (7'hE+R missing after Sr during dynamic address arbitration):"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == CODERR_A::B_0xC
    }
    #[doc = "TE5 error (transaction after detecting CCC):"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == CODERR_A::B_0xD
    }
    #[doc = "TE6 error (monitoring error):"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == CODERR_A::B_0xE
    }
}
#[doc = "protocol error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERR_A {
    #[doc = "0: no detected error"]
    B_0x0 = 0,
    #[doc = "1: whatever controller or target, hardware detected a protocol error, as detailed in CODERR\\[3:0\\]"]
    B_0x1 = 1,
}
impl From<PERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERR` reader - protocol error"]
pub type PERR_R = crate::BitReader<PERR_A>;
impl PERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PERR_A {
        match self.bits {
            false => PERR_A::B_0x0,
            true => PERR_A::B_0x1,
        }
    }
    #[doc = "no detected error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PERR_A::B_0x0
    }
    #[doc = "whatever controller or target, hardware detected a protocol error, as detailed in CODERR\\[3:0\\]"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PERR_A::B_0x1
    }
}
#[doc = "SCL stall error (when the I3C is acting as target)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STALL_A {
    #[doc = "0: no detected error"]
    B_0x0 = 0,
    #[doc = "1: target detected that SCL was stable for more than 125 s during a I3C SDR read"]
    B_0x1 = 1,
}
impl From<STALL_A> for bool {
    #[inline(always)]
    fn from(variant: STALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL` reader - SCL stall error (when the I3C is acting as target)"]
pub type STALL_R = crate::BitReader<STALL_A>;
impl STALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STALL_A {
        match self.bits {
            false => STALL_A::B_0x0,
            true => STALL_A::B_0x1,
        }
    }
    #[doc = "no detected error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == STALL_A::B_0x0
    }
    #[doc = "target detected that SCL was stable for more than 125 s during a I3C SDR read"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == STALL_A::B_0x1
    }
}
#[doc = "RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVR_A {
    #[doc = "0: no detected error"]
    B_0x0 = 0,
    #[doc = "1: whatever controller or target, hardware detected either:"]
    B_0x1 = 1,
}
impl From<DOVR_A> for bool {
    #[inline(always)]
    fn from(variant: DOVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOVR` reader - RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received"]
pub type DOVR_R = crate::BitReader<DOVR_A>;
impl DOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DOVR_A {
        match self.bits {
            false => DOVR_A::B_0x0,
            true => DOVR_A::B_0x1,
        }
    }
    #[doc = "no detected error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DOVR_A::B_0x0
    }
    #[doc = "whatever controller or target, hardware detected either:"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DOVR_A::B_0x1
    }
}
#[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVR_A {
    #[doc = "0: no detected error"]
    B_0x0 = 0,
    #[doc = "1: controller detected either:"]
    B_0x1 = 1,
}
impl From<COVR_A> for bool {
    #[inline(always)]
    fn from(variant: COVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COVR` reader - C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends"]
pub type COVR_R = crate::BitReader<COVR_A>;
impl COVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COVR_A {
        match self.bits {
            false => COVR_A::B_0x0,
            true => COVR_A::B_0x1,
        }
    }
    #[doc = "no detected error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == COVR_A::B_0x0
    }
    #[doc = "controller detected either:"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == COVR_A::B_0x1
    }
}
#[doc = "address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANACK_A {
    #[doc = "0: no detected error"]
    B_0x0 = 0,
    #[doc = "1: controller detected that the static/dynamic address was not acknowledged by a target, either during:"]
    B_0x1 = 1,
}
impl From<ANACK_A> for bool {
    #[inline(always)]
    fn from(variant: ANACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACK` reader - address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer"]
pub type ANACK_R = crate::BitReader<ANACK_A>;
impl ANACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANACK_A {
        match self.bits {
            false => ANACK_A::B_0x0,
            true => ANACK_A::B_0x1,
        }
    }
    #[doc = "no detected error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ANACK_A::B_0x0
    }
    #[doc = "controller detected that the static/dynamic address was not acknowledged by a target, either during:"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ANACK_A::B_0x1
    }
}
#[doc = "data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNACK_A {
    #[doc = "0: no detected error"]
    B_0x0 = 0,
    #[doc = "1: controller detected that a data byte is not acknowledged by a target, either during:"]
    B_0x1 = 1,
}
impl From<DNACK_A> for bool {
    #[inline(always)]
    fn from(variant: DNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNACK` reader - data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure"]
pub type DNACK_R = crate::BitReader<DNACK_A>;
impl DNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DNACK_A {
        match self.bits {
            false => DNACK_A::B_0x0,
            true => DNACK_A::B_0x1,
        }
    }
    #[doc = "no detected error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DNACK_A::B_0x0
    }
    #[doc = "controller detected that a data byte is not acknowledged by a target, either during:"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DNACK_A::B_0x1
    }
}
#[doc = "data error (when the I3C is acting as controller)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DERR_A {
    #[doc = "0: no detected error"]
    B_0x0 = 0,
    #[doc = "1: controller detected a data error during the controller-role hand-off procedure (GETACCCR CCC, formerly known as GETACCMST) when the received target address or/and the parity bit do no match. Active controller keeps controller-role."]
    B_0x1 = 1,
}
impl From<DERR_A> for bool {
    #[inline(always)]
    fn from(variant: DERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DERR` reader - data error (when the I3C is acting as controller)"]
pub type DERR_R = crate::BitReader<DERR_A>;
impl DERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DERR_A {
        match self.bits {
            false => DERR_A::B_0x0,
            true => DERR_A::B_0x1,
        }
    }
    #[doc = "no detected error"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DERR_A::B_0x0
    }
    #[doc = "controller detected a data error during the controller-role hand-off procedure (GETACCCR CCC, formerly known as GETACCMST) when the received target address or/and the parity bit do no match. Active controller keeps controller-role."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DERR_A::B_0x1
    }
}
impl R {
    #[doc = "Bits 0:3 - protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7'hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7'hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7'hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved"]
    #[inline(always)]
    pub fn CODERR(&self) -> CODERR_R {
        CODERR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - protocol error"]
    #[inline(always)]
    pub fn PERR(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCL stall error (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn STALL(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received"]
    #[inline(always)]
    pub fn DOVR(&self) -> DOVR_R {
        DOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends"]
    #[inline(always)]
    pub fn COVR(&self) -> COVR_R {
        COVR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer"]
    #[inline(always)]
    pub fn ANACK(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure"]
    #[inline(always)]
    pub fn DNACK(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - data error (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn DERR(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "I3C status error register\n\nYou can [`read`](crate::Reg::read) this register and get [`ser::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SER_SPEC;
impl crate::RegisterSpec for SER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser::R`](R) reader structure"]
impl crate::Readable for SER_SPEC {}
#[doc = "`reset()` method sets SER to value 0"]
impl crate::Resettable for SER_SPEC {}
