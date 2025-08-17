#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: I3C is disabled"]
    B_0x0 = 0,
    #[doc = "1: I3C is enabled"]
    B_0x1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)"]
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::B_0x0,
            true => EN_A::B_0x1,
        }
    }
    #[doc = "I3C is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EN_A::B_0x0
    }
    #[doc = "I3C is enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EN_A::B_0x1
    }
}
#[doc = "Field `EN` writer - I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I3C is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0x0)
    }
    #[doc = "I3C is enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0x1)
    }
}
#[doc = "initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRINIT_A {
    #[doc = "0: target role"]
    B_0x0 = 0,
    #[doc = "1: controller role"]
    B_0x1 = 1,
}
impl From<CRINIT_A> for bool {
    #[inline(always)]
    fn from(variant: CRINIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRINIT` reader - initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
pub type CRINIT_R = crate::BitReader<CRINIT_A>;
impl CRINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRINIT_A {
        match self.bits {
            false => CRINIT_A::B_0x0,
            true => CRINIT_A::B_0x1,
        }
    }
    #[doc = "target role"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CRINIT_A::B_0x0
    }
    #[doc = "controller role"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CRINIT_A::B_0x1
    }
}
#[doc = "Field `CRINIT` writer - initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
pub type CRINIT_W<'a, REG> = crate::BitWriter<'a, REG, CRINIT_A>;
impl<'a, REG> CRINIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "target role"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRINIT_A::B_0x0)
    }
    #[doc = "controller role"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRINIT_A::B_0x1)
    }
}
#[doc = "no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOARBH_A {
    #[doc = "0: An arbitrable header (7'h7E + RnW=0) is emitted after a START and before a legacy I2C message or an I3C SDR private read/write message (default)."]
    B_0x0 = 0,
    #[doc = "1: No arbitrable header"]
    B_0x1 = 1,
}
impl From<NOARBH_A> for bool {
    #[inline(always)]
    fn from(variant: NOARBH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOARBH` reader - no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
pub type NOARBH_R = crate::BitReader<NOARBH_A>;
impl NOARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOARBH_A {
        match self.bits {
            false => NOARBH_A::B_0x0,
            true => NOARBH_A::B_0x1,
        }
    }
    #[doc = "An arbitrable header (7'h7E + RnW=0) is emitted after a START and before a legacy I2C message or an I3C SDR private read/write message (default)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == NOARBH_A::B_0x0
    }
    #[doc = "No arbitrable header"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == NOARBH_A::B_0x1
    }
}
#[doc = "Field `NOARBH` writer - no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
pub type NOARBH_W<'a, REG> = crate::BitWriter<'a, REG, NOARBH_A>;
impl<'a, REG> NOARBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An arbitrable header (7'h7E + RnW=0) is emitted after a START and before a legacy I2C message or an I3C SDR private read/write message (default)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NOARBH_A::B_0x0)
    }
    #[doc = "No arbitrable header"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NOARBH_A::B_0x1)
    }
}
#[doc = "HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTPTRN_A {
    #[doc = "0: standard STOP emitted at the end of a frame"]
    B_0x0 = 0,
    #[doc = "1: HDR reset pattern is inserted before the STOP of any emitted frame that includes a RSTACT CCC command"]
    B_0x1 = 1,
}
impl From<RSTPTRN_A> for bool {
    #[inline(always)]
    fn from(variant: RSTPTRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTPTRN` reader - HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
pub type RSTPTRN_R = crate::BitReader<RSTPTRN_A>;
impl RSTPTRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTPTRN_A {
        match self.bits {
            false => RSTPTRN_A::B_0x0,
            true => RSTPTRN_A::B_0x1,
        }
    }
    #[doc = "standard STOP emitted at the end of a frame"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RSTPTRN_A::B_0x0
    }
    #[doc = "HDR reset pattern is inserted before the STOP of any emitted frame that includes a RSTACT CCC command"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RSTPTRN_A::B_0x1
    }
}
#[doc = "Field `RSTPTRN` writer - HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
pub type RSTPTRN_W<'a, REG> = crate::BitWriter<'a, REG, RSTPTRN_A>;
impl<'a, REG> RSTPTRN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "standard STOP emitted at the end of a frame"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RSTPTRN_A::B_0x0)
    }
    #[doc = "HDR reset pattern is inserted before the STOP of any emitted frame that includes a RSTACT CCC command"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RSTPTRN_A::B_0x1)
    }
}
#[doc = "HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn't assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXITPTRN_A {
    #[doc = "0: HDR Exit Pattern is not sent after the message header (MTYPE\\[3:0\\]=0001)"]
    B_0x0 = 0,
    #[doc = "1: HDR Exit Pattern is sent after the message header (MTYPE\\[3:0\\]=0001) to generate an escalation fault"]
    B_0x1 = 1,
}
impl From<EXITPTRN_A> for bool {
    #[inline(always)]
    fn from(variant: EXITPTRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXITPTRN` reader - HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn't assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
pub type EXITPTRN_R = crate::BitReader<EXITPTRN_A>;
impl EXITPTRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXITPTRN_A {
        match self.bits {
            false => EXITPTRN_A::B_0x0,
            true => EXITPTRN_A::B_0x1,
        }
    }
    #[doc = "HDR Exit Pattern is not sent after the message header (MTYPE\\[3:0\\]=0001)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXITPTRN_A::B_0x0
    }
    #[doc = "HDR Exit Pattern is sent after the message header (MTYPE\\[3:0\\]=0001) to generate an escalation fault"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXITPTRN_A::B_0x1
    }
}
#[doc = "Field `EXITPTRN` writer - HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn't assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
pub type EXITPTRN_W<'a, REG> = crate::BitWriter<'a, REG, EXITPTRN_A>;
impl<'a, REG> EXITPTRN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HDR Exit Pattern is not sent after the message header (MTYPE\\[3:0\\]=0001)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXITPTRN_A::B_0x0)
    }
    #[doc = "HDR Exit Pattern is sent after the message header (MTYPE\\[3:0\\]=0001) to generate an escalation fault"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXITPTRN_A::B_0x1)
    }
}
#[doc = "High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HKSDAEN_A {
    #[doc = "0: High-Keeper is disabled"]
    B_0x0 = 0,
    #[doc = "1: High-Keeper is enabled, and the weak pull-up is effective on the T bit, instead of the open-drain class pull-up."]
    B_0x1 = 1,
}
impl From<HKSDAEN_A> for bool {
    #[inline(always)]
    fn from(variant: HKSDAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HKSDAEN` reader - High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
pub type HKSDAEN_R = crate::BitReader<HKSDAEN_A>;
impl HKSDAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HKSDAEN_A {
        match self.bits {
            false => HKSDAEN_A::B_0x0,
            true => HKSDAEN_A::B_0x1,
        }
    }
    #[doc = "High-Keeper is disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HKSDAEN_A::B_0x0
    }
    #[doc = "High-Keeper is enabled, and the weak pull-up is effective on the T bit, instead of the open-drain class pull-up."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HKSDAEN_A::B_0x1
    }
}
#[doc = "Field `HKSDAEN` writer - High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
pub type HKSDAEN_W<'a, REG> = crate::BitWriter<'a, REG, HKSDAEN_A>;
impl<'a, REG> HKSDAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High-Keeper is disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HKSDAEN_A::B_0x0)
    }
    #[doc = "High-Keeper is enabled, and the weak pull-up is effective on the T bit, instead of the open-drain class pull-up."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HKSDAEN_A::B_0x1)
    }
}
#[doc = "Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HJACK_A {
    #[doc = "0: Hot Join request is NACKed"]
    B_0x0 = 0,
    #[doc = "1: Hot Join request is ACKed"]
    B_0x1 = 1,
}
impl From<HJACK_A> for bool {
    #[inline(always)]
    fn from(variant: HJACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HJACK` reader - Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
pub type HJACK_R = crate::BitReader<HJACK_A>;
impl HJACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HJACK_A {
        match self.bits {
            false => HJACK_A::B_0x0,
            true => HJACK_A::B_0x1,
        }
    }
    #[doc = "Hot Join request is NACKed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HJACK_A::B_0x0
    }
    #[doc = "Hot Join request is ACKed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HJACK_A::B_0x1
    }
}
#[doc = "Field `HJACK` writer - Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
pub type HJACK_W<'a, REG> = crate::BitWriter<'a, REG, HJACK_A>;
impl<'a, REG> HJACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hot Join request is NACKed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HJACK_A::B_0x0)
    }
    #[doc = "Hot Join request is ACKed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HJACK_A::B_0x1)
    }
}
#[doc = "RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    #[doc = "0: DMA mode is disabled for RX-FIFO"]
    B_0x0 = 0,
    #[doc = "1: DMA mode is enabled for RX-FIFO"]
    B_0x1 = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::B_0x0,
            true => RXDMAEN_A::B_0x1,
        }
    }
    #[doc = "DMA mode is disabled for RX-FIFO"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXDMAEN_A::B_0x0
    }
    #[doc = "DMA mode is enabled for RX-FIFO"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXDMAEN_A::B_0x1
    }
}
#[doc = "Field `RXDMAEN` writer - RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN_A>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for RX-FIFO"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN_A::B_0x0)
    }
    #[doc = "DMA mode is enabled for RX-FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN_A::B_0x1)
    }
}
#[doc = "RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFLUSH_A {
    #[doc = "0: no action"]
    B_0x0 = 0,
    #[doc = "1: flush RX-FIFO"]
    B_0x1 = 1,
}
impl From<RXFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: RXFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFLUSH` writer - RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written."]
pub type RXFLUSH_W<'a, REG> = crate::BitWriter<'a, REG, RXFLUSH_A>;
impl<'a, REG> RXFLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXFLUSH_A::B_0x0)
    }
    #[doc = "flush RX-FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXFLUSH_A::B_0x1)
    }
}
#[doc = "RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXTHRES_A {
    #[doc = "0: 1-byte threshold"]
    B_0x0 = 0,
    #[doc = "1: 4-byte threshold"]
    B_0x1 = 1,
}
impl From<RXTHRES_A> for bool {
    #[inline(always)]
    fn from(variant: RXTHRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTHRES` reader - RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
pub type RXTHRES_R = crate::BitReader<RXTHRES_A>;
impl RXTHRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXTHRES_A {
        match self.bits {
            false => RXTHRES_A::B_0x0,
            true => RXTHRES_A::B_0x1,
        }
    }
    #[doc = "1-byte threshold"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RXTHRES_A::B_0x0
    }
    #[doc = "4-byte threshold"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RXTHRES_A::B_0x1
    }
}
#[doc = "Field `RXTHRES` writer - RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
pub type RXTHRES_W<'a, REG> = crate::BitWriter<'a, REG, RXTHRES_A>;
impl<'a, REG> RXTHRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1-byte threshold"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXTHRES_A::B_0x0)
    }
    #[doc = "4-byte threshold"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXTHRES_A::B_0x1)
    }
}
#[doc = "TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    #[doc = "0: DMA mode is disabled for TX-FIFO"]
    B_0x0 = 0,
    #[doc = "1: DMA mode is enabled for TX-FIFO"]
    B_0x1 = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::B_0x0,
            true => TXDMAEN_A::B_0x1,
        }
    }
    #[doc = "DMA mode is disabled for TX-FIFO"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXDMAEN_A::B_0x0
    }
    #[doc = "DMA mode is enabled for TX-FIFO"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXDMAEN_A::B_0x1
    }
}
#[doc = "Field `TXDMAEN` writer - TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN_A>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for TX-FIFO"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN_A::B_0x0)
    }
    #[doc = "DMA mode is enabled for TX-FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN_A::B_0x1)
    }
}
#[doc = "TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. I3C_SR.ABT=1 and I3C_SR.XDCNT\\[15:0\\] I3C_TGTTDR.TGTTDCNT\\[15:0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFLUSH_A {
    #[doc = "0: no action"]
    B_0x0 = 0,
    #[doc = "1: flush TX-FIFO"]
    B_0x1 = 1,
}
impl From<TXFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: TXFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFLUSH` writer - TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. I3C_SR.ABT=1 and I3C_SR.XDCNT\\[15:0\\] I3C_TGTTDR.TGTTDCNT\\[15:0\\])."]
pub type TXFLUSH_W<'a, REG> = crate::BitWriter<'a, REG, TXFLUSH_A>;
impl<'a, REG> TXFLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXFLUSH_A::B_0x0)
    }
    #[doc = "flush TX-FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXFLUSH_A::B_0x1)
    }
}
#[doc = "TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTHRES_A {
    #[doc = "0: 1-byte threshold"]
    B_0x0 = 0,
    #[doc = "1: 4-byte threshold"]
    B_0x1 = 1,
}
impl From<TXTHRES_A> for bool {
    #[inline(always)]
    fn from(variant: TXTHRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTHRES` reader - TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
pub type TXTHRES_R = crate::BitReader<TXTHRES_A>;
impl TXTHRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXTHRES_A {
        match self.bits {
            false => TXTHRES_A::B_0x0,
            true => TXTHRES_A::B_0x1,
        }
    }
    #[doc = "1-byte threshold"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TXTHRES_A::B_0x0
    }
    #[doc = "4-byte threshold"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TXTHRES_A::B_0x1
    }
}
#[doc = "Field `TXTHRES` writer - TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
pub type TXTHRES_W<'a, REG> = crate::BitWriter<'a, REG, TXTHRES_A>;
impl<'a, REG> TXTHRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1-byte threshold"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXTHRES_A::B_0x0)
    }
    #[doc = "4-byte threshold"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXTHRES_A::B_0x1)
    }
}
#[doc = "S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMAEN_A {
    #[doc = "0: DMA mode is disabled for S-FIFO"]
    B_0x0 = 0,
    #[doc = "1: DMA mode is enabled for S-FIFO"]
    B_0x1 = 1,
}
impl From<SDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMAEN` reader - S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type SDMAEN_R = crate::BitReader<SDMAEN_A>;
impl SDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDMAEN_A {
        match self.bits {
            false => SDMAEN_A::B_0x0,
            true => SDMAEN_A::B_0x1,
        }
    }
    #[doc = "DMA mode is disabled for S-FIFO"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SDMAEN_A::B_0x0
    }
    #[doc = "DMA mode is enabled for S-FIFO"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SDMAEN_A::B_0x1
    }
}
#[doc = "Field `SDMAEN` writer - S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type SDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, SDMAEN_A>;
impl<'a, REG> SDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for S-FIFO"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SDMAEN_A::B_0x0)
    }
    #[doc = "DMA mode is enabled for S-FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SDMAEN_A::B_0x1)
    }
}
#[doc = "S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFLUSH_A {
    #[doc = "0: no action"]
    B_0x0 = 0,
    #[doc = "1: flush S-FIFO"]
    B_0x1 = 1,
}
impl From<SFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: SFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFLUSH` writer - S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller)."]
pub type SFLUSH_W<'a, REG> = crate::BitWriter<'a, REG, SFLUSH_A>;
impl<'a, REG> SFLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SFLUSH_A::B_0x0)
    }
    #[doc = "flush S-FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SFLUSH_A::B_0x1)
    }
}
#[doc = "S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMODE_A {
    #[doc = "0: S-FIFO is disabled, and the"]
    B_0x0 = 0,
    #[doc = "1: S-FIFO is enabled."]
    B_0x1 = 1,
}
impl From<RMODE_A> for bool {
    #[inline(always)]
    fn from(variant: RMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMODE` reader - S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
pub type RMODE_R = crate::BitReader<RMODE_A>;
impl RMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RMODE_A {
        match self.bits {
            false => RMODE_A::B_0x0,
            true => RMODE_A::B_0x1,
        }
    }
    #[doc = "S-FIFO is disabled, and the"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RMODE_A::B_0x0
    }
    #[doc = "S-FIFO is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RMODE_A::B_0x1
    }
}
#[doc = "Field `RMODE` writer - S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
pub type RMODE_W<'a, REG> = crate::BitWriter<'a, REG, RMODE_A>;
impl<'a, REG> RMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S-FIFO is disabled, and the"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RMODE_A::B_0x0)
    }
    #[doc = "S-FIFO is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RMODE_A::B_0x1)
    }
}
#[doc = "transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMODE_A {
    #[doc = "0: C-FIFO and TX-FIFO are not preloaded before starting to emit a frame transfer."]
    B_0x0 = 0,
    #[doc = "1: C-FIFO and TX-FIFO are first preloaded (also TX-FIFO if needed (depending on the frame format) before starting to emit a frame transfer."]
    B_0x1 = 1,
}
impl From<TMODE_A> for bool {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMODE` reader - transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
pub type TMODE_R = crate::BitReader<TMODE_A>;
impl TMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMODE_A {
        match self.bits {
            false => TMODE_A::B_0x0,
            true => TMODE_A::B_0x1,
        }
    }
    #[doc = "C-FIFO and TX-FIFO are not preloaded before starting to emit a frame transfer."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TMODE_A::B_0x0
    }
    #[doc = "C-FIFO and TX-FIFO are first preloaded (also TX-FIFO if needed (depending on the frame format) before starting to emit a frame transfer."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TMODE_A::B_0x1
    }
}
#[doc = "Field `TMODE` writer - transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
pub type TMODE_W<'a, REG> = crate::BitWriter<'a, REG, TMODE_A>;
impl<'a, REG> TMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "C-FIFO and TX-FIFO are not preloaded before starting to emit a frame transfer."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::B_0x0)
    }
    #[doc = "C-FIFO and TX-FIFO are first preloaded (also TX-FIFO if needed (depending on the frame format) before starting to emit a frame transfer."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TMODE_A::B_0x1)
    }
}
#[doc = "C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDMAEN_A {
    #[doc = "0: DMA mode is disabled for C-FIFO"]
    B_0x0 = 0,
    #[doc = "1: DMA mode is enabled for C-FIFO"]
    B_0x1 = 1,
}
impl From<CDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMAEN` reader - C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type CDMAEN_R = crate::BitReader<CDMAEN_A>;
impl CDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CDMAEN_A {
        match self.bits {
            false => CDMAEN_A::B_0x0,
            true => CDMAEN_A::B_0x1,
        }
    }
    #[doc = "DMA mode is disabled for C-FIFO"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CDMAEN_A::B_0x0
    }
    #[doc = "DMA mode is enabled for C-FIFO"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CDMAEN_A::B_0x1
    }
}
#[doc = "Field `CDMAEN` writer - C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type CDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, CDMAEN_A>;
impl<'a, REG> CDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for C-FIFO"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CDMAEN_A::B_0x0)
    }
    #[doc = "DMA mode is enabled for C-FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CDMAEN_A::B_0x1)
    }
}
#[doc = "C-FIFO flush (when I3C is acting as controller) This bit can only be written.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFLUSH_A {
    #[doc = "0: no action"]
    B_0x0 = 0,
    #[doc = "1: flush C-FIFO"]
    B_0x1 = 1,
}
impl From<CFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: CFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFLUSH` writer - C-FIFO flush (when I3C is acting as controller) This bit can only be written."]
pub type CFLUSH_W<'a, REG> = crate::BitWriter<'a, REG, CFLUSH_A>;
impl<'a, REG> CFLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CFLUSH_A::B_0x0)
    }
    #[doc = "flush C-FIFO"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CFLUSH_A::B_0x1)
    }
}
#[doc = "frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. I3C_CR) while C-FIFO is empty (i.e. I3C_EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e I3C_CR.MEND=0), it causes the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a next control word is needed).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSFSET_A {
    #[doc = "0: no action"]
    B_0x0 = 0,
    #[doc = "1: setting this bit initiates a frame transfer by causing the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a control word is needed)"]
    B_0x1 = 1,
}
impl From<TSFSET_A> for bool {
    #[inline(always)]
    fn from(variant: TSFSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSFSET` writer - frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. I3C_CR) while C-FIFO is empty (i.e. I3C_EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e I3C_CR.MEND=0), it causes the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a next control word is needed)."]
pub type TSFSET_W<'a, REG> = crate::BitWriter<'a, REG, TSFSET_A>;
impl<'a, REG> TSFSET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSFSET_A::B_0x0)
    }
    #[doc = "setting this bit initiates a frame transfer by causing the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a control word is needed)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSFSET_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)"]
    #[inline(always)]
    pub fn EN(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
    #[inline(always)]
    pub fn CRINIT(&self) -> CRINIT_R {
        CRINIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
    #[inline(always)]
    pub fn NOARBH(&self) -> NOARBH_R {
        NOARBH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
    #[inline(always)]
    pub fn RSTPTRN(&self) -> RSTPTRN_R {
        RSTPTRN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn't assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
    #[inline(always)]
    pub fn EXITPTRN(&self) -> EXITPTRN_R {
        EXITPTRN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
    #[inline(always)]
    pub fn HKSDAEN(&self) -> HKSDAEN_R {
        HKSDAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
    #[inline(always)]
    pub fn HJACK(&self) -> HJACK_R {
        HJACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn RXDMAEN(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
    #[inline(always)]
    pub fn RXTHRES(&self) -> RXTHRES_R {
        RXTHRES_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn TXDMAEN(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
    #[inline(always)]
    pub fn TXTHRES(&self) -> TXTHRES_R {
        TXTHRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn SDMAEN(&self) -> SDMAEN_R {
        SDMAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
    #[inline(always)]
    pub fn RMODE(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
    #[inline(always)]
    pub fn TMODE(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn CDMAEN(&self) -> CDMAEN_R {
        CDMAEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)"]
    #[inline(always)]
    pub fn EN(&mut self) -> EN_W<'_, CFGR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
    #[inline(always)]
    pub fn CRINIT(&mut self) -> CRINIT_W<'_, CFGR_SPEC> {
        CRINIT_W::new(self, 1)
    }
    #[doc = "Bit 2 - no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
    #[inline(always)]
    pub fn NOARBH(&mut self) -> NOARBH_W<'_, CFGR_SPEC> {
        NOARBH_W::new(self, 2)
    }
    #[doc = "Bit 3 - HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
    #[inline(always)]
    pub fn RSTPTRN(&mut self) -> RSTPTRN_W<'_, CFGR_SPEC> {
        RSTPTRN_W::new(self, 3)
    }
    #[doc = "Bit 4 - HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn't assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
    #[inline(always)]
    pub fn EXITPTRN(&mut self) -> EXITPTRN_W<'_, CFGR_SPEC> {
        EXITPTRN_W::new(self, 4)
    }
    #[doc = "Bit 5 - High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
    #[inline(always)]
    pub fn HKSDAEN(&mut self) -> HKSDAEN_W<'_, CFGR_SPEC> {
        HKSDAEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
    #[inline(always)]
    pub fn HJACK(&mut self) -> HJACK_W<'_, CFGR_SPEC> {
        HJACK_W::new(self, 7)
    }
    #[doc = "Bit 8 - RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn RXDMAEN(&mut self) -> RXDMAEN_W<'_, CFGR_SPEC> {
        RXDMAEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written."]
    #[inline(always)]
    pub fn RXFLUSH(&mut self) -> RXFLUSH_W<'_, CFGR_SPEC> {
        RXFLUSH_W::new(self, 9)
    }
    #[doc = "Bit 10 - RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
    #[inline(always)]
    pub fn RXTHRES(&mut self) -> RXTHRES_W<'_, CFGR_SPEC> {
        RXTHRES_W::new(self, 10)
    }
    #[doc = "Bit 12 - TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn TXDMAEN(&mut self) -> TXDMAEN_W<'_, CFGR_SPEC> {
        TXDMAEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. I3C_SR.ABT=1 and I3C_SR.XDCNT\\[15:0\\] I3C_TGTTDR.TGTTDCNT\\[15:0\\])."]
    #[inline(always)]
    pub fn TXFLUSH(&mut self) -> TXFLUSH_W<'_, CFGR_SPEC> {
        TXFLUSH_W::new(self, 13)
    }
    #[doc = "Bit 14 - TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
    #[inline(always)]
    pub fn TXTHRES(&mut self) -> TXTHRES_W<'_, CFGR_SPEC> {
        TXTHRES_W::new(self, 14)
    }
    #[doc = "Bit 16 - S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn SDMAEN(&mut self) -> SDMAEN_W<'_, CFGR_SPEC> {
        SDMAEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller)."]
    #[inline(always)]
    pub fn SFLUSH(&mut self) -> SFLUSH_W<'_, CFGR_SPEC> {
        SFLUSH_W::new(self, 17)
    }
    #[doc = "Bit 18 - S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
    #[inline(always)]
    pub fn RMODE(&mut self) -> RMODE_W<'_, CFGR_SPEC> {
        RMODE_W::new(self, 18)
    }
    #[doc = "Bit 19 - transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
    #[inline(always)]
    pub fn TMODE(&mut self) -> TMODE_W<'_, CFGR_SPEC> {
        TMODE_W::new(self, 19)
    }
    #[doc = "Bit 20 - C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn CDMAEN(&mut self) -> CDMAEN_W<'_, CFGR_SPEC> {
        CDMAEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - C-FIFO flush (when I3C is acting as controller) This bit can only be written."]
    #[inline(always)]
    pub fn CFLUSH(&mut self) -> CFLUSH_W<'_, CFGR_SPEC> {
        CFLUSH_W::new(self, 21)
    }
    #[doc = "Bit 30 - frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. I3C_CR) while C-FIFO is empty (i.e. I3C_EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e I3C_CR.MEND=0), it causes the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a next control word is needed)."]
    #[inline(always)]
    pub fn TSFSET(&mut self) -> TSFSET_W<'_, CFGR_SPEC> {
        TSFSET_W::new(self, 30)
    }
}
#[doc = "I3C configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {}
