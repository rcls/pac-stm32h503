#[doc = "Register `CHEP6R` reader"]
pub type R = crate::R<CHEP6R_SPEC>;
#[doc = "Register `CHEP6R` writer"]
pub type W = crate::W<CHEP6R_SPEC>;
#[doc = "Field `EA` reader - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
pub type EA_R = crate::FieldReader;
#[doc = "Field `EA` writer - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
pub type EA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STATTX` writer - Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes 0, the value remains unchanged, while writing 1 makes the bit value to toggle. Hardware sets the STATTX bits to NAK, when a correct transfer has occurred (VTTX = 1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints and usage in Device mode). If the endpoint is defined as isochronous, its status can only be 'VALID' or 'DISABLED'. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STATTX bits to 'STALL' or 'NAK' for an isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing 1. Host mode The STATTX bits contain the information about the channel status. Refer to for the full descriptions ('Host mode' descriptions). Whereas in Device mode, these bits contain the status that are given out on the following transaction, in Host mode they capture the status last received from the device. If a NAK is received, STATTX contains the value indicating NAK."]
pub type STATTX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOGTX` writer - Data toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (0 = DATA0, 1 = DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint. If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to Device mode) If the endpoint/channel is isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (refer to ). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes 0, the value of DTOGTX remains unchanged, while writing 1 makes the bit value to toggle. This bit is read/write but it can only be toggled by writing 1."]
pub type DTOGTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTTX` reader - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written. Host mode Same as VTRX behavior but for USB OUT and SETUP transactions."]
pub type VTTX_R = crate::BitReader;
#[doc = "Field `VTTX` writer - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written. Host mode Same as VTRX behavior but for USB OUT and SETUP transactions."]
pub type VTTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPKIND` reader - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the UTYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints and usage in Device mode. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered 'STALL' instead of 'ACK'. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
pub type EPKIND_R = crate::BitReader;
#[doc = "Field `EPKIND` writer - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the UTYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints and usage in Device mode. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered 'STALL' instead of 'ACK'. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
pub type EPKIND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTYPE` reader - USB type of transaction These bits configure the behavior of this endpoint/channel as described in Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of isochronous channels/endpoints is explained in transfers in Device mode"]
pub type UTYPE_R = crate::FieldReader;
#[doc = "Field `UTYPE` writer - USB type of transaction These bits configure the behavior of this endpoint/channel as described in Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of isochronous channels/endpoints is explained in transfers in Device mode"]
pub type UTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETUP` reader - Setup transaction completed Device mode This bit is read-only and it is set by the hardware when the last completed transaction is a SETUP. This bit changes its value only for control endpoints. It must be examined, in the case of a successful receive transaction (VTRX event), to determine the type of transaction occurred. To protect the interrupt service routine from the changes in SETUP bits due to next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when VTRX is at 0. This bit is read-only. Host mode This bit is set by the software to send a SETUP transaction on a control endpoint. This bit changes its value only for control endpoints. It is cleared by hardware when the SETUP transaction is acknowledged and VTTX interrupt generated."]
pub type SETUP_R = crate::BitReader;
#[doc = "Field `STATRX` writer - Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page 2492. These bits can be toggled by software to initialize their value. When the application software writes 0, the value remains unchanged, while writing 1 makes the bit value to toggle. Hardware sets the STATRX bits to NAK when a correct transfer has occurred (VTRX = 1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledges a new transaction. Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints and usage in Device mode). If the endpoint is defined as isochronous, its status can be only 'VALID' or 'DISABLED', so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STATRX bits to 'STALL' or 'NAK' for an isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing 1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STATRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the host execution list. If the aborted transaction was already under execution it is regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID A host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the host frame scheduler to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel is re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate."]
pub type STATRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOGRX` writer - Data Toggle, for reception transfers If the endpoint/channel is not isochronous, this bit contains the expected value of the data toggle bit (0 = DATA0, 1 = DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device) or acknowledged by device (in host). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to Device mode). If the endpoint/channel is isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Isochronous transfers in Device mode). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes 0, the value of DTOGRX remains unchanged, while writing 1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1."]
pub type DTOGRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRX` reader - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STATRX field of this register. One NAKed transaction keeps pending and is automatically retried by the host at the next frame, or the host can immediately retry by resetting STATRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STATRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STATRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STATRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STATRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. - A transaction ended with error sets this bit. Errors can be seen via the bits ERR_RX (host mode only). This bit is read/write but only 0 can be written, writing 1 has no effect."]
pub type VTRX_R = crate::BitReader;
#[doc = "Field `VTRX` writer - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STATRX field of this register. One NAKed transaction keeps pending and is automatically retried by the host at the next frame, or the host can immediately retry by resetting STATRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STATRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STATRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STATRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STATRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. - A transaction ended with error sets this bit. Errors can be seen via the bits ERR_RX (host mode only). This bit is read/write but only 0 can be written, writing 1 has no effect."]
pub type VTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDR` reader - Host mode Device address assigned to the endpoint during the enumeration process."]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Host mode Device address assigned to the endpoint during the enumeration process."]
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NAK` reader - Host mode This bit is set by the hardware when a device responds with a NAK. Software can use this bit to monitor the number of NAKs received from a device."]
pub type NAK_R = crate::BitReader;
#[doc = "Field `NAK` writer - Host mode This bit is set by the hardware when a device responds with a NAK. Software can use this bit to monitor the number of NAKs received from a device."]
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low speed endpoint host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LS_EP_A {
    #[doc = "0: Full speed endpoint"]
    B_0x0 = 0,
    #[doc = "1: Low speed endpoint"]
    B_0x1 = 1,
}
impl From<LS_EP_A> for bool {
    #[inline(always)]
    fn from(variant: LS_EP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LS_EP` reader - Low speed endpoint host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint."]
pub type LS_EP_R = crate::BitReader<LS_EP_A>;
impl LS_EP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LS_EP_A {
        match self.bits {
            false => LS_EP_A::B_0x0,
            true => LS_EP_A::B_0x1,
        }
    }
    #[doc = "Full speed endpoint"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LS_EP_A::B_0x0
    }
    #[doc = "Low speed endpoint"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LS_EP_A::B_0x1
    }
}
#[doc = "Field `LS_EP` writer - Low speed endpoint host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint."]
pub type LS_EP_W<'a, REG> = crate::BitWriter<'a, REG, LS_EP_A>;
impl<'a, REG> LS_EP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full speed endpoint"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LS_EP_A::B_0x0)
    }
    #[doc = "Low speed endpoint"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LS_EP_A::B_0x1)
    }
}
#[doc = "Field `ERR_TX` reader - Received error for an OUT/SETUP transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
pub type ERR_TX_R = crate::BitReader;
#[doc = "Field `ERR_TX` writer - Received error for an OUT/SETUP transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
pub type ERR_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_RX` reader - Received error for an IN transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
pub type ERR_RX_R = crate::BitReader;
#[doc = "Field `ERR_RX` writer - Received error for an IN transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
pub type ERR_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THREE_ERR_TX_A {
    #[doc = "0: Less than 3 errors received."]
    B_0x0 = 0,
    #[doc = "1: More than 3 errors received, last error is timeout error."]
    B_0x1 = 1,
    #[doc = "2: More than 3 errors received, last error is data error (CRC error)."]
    B_0x2 = 2,
    #[doc = "3: More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    B_0x3 = 3,
}
impl From<THREE_ERR_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: THREE_ERR_TX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for THREE_ERR_TX_A {
    type Ux = u8;
}
impl crate::IsEnum for THREE_ERR_TX_A {}
#[doc = "Field `THREE_ERR_TX` reader - Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
pub type THREE_ERR_TX_R = crate::FieldReader<THREE_ERR_TX_A>;
impl THREE_ERR_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THREE_ERR_TX_A {
        match self.bits {
            0 => THREE_ERR_TX_A::B_0x0,
            1 => THREE_ERR_TX_A::B_0x1,
            2 => THREE_ERR_TX_A::B_0x2,
            3 => THREE_ERR_TX_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Less than 3 errors received."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == THREE_ERR_TX_A::B_0x0
    }
    #[doc = "More than 3 errors received, last error is timeout error."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == THREE_ERR_TX_A::B_0x1
    }
    #[doc = "More than 3 errors received, last error is data error (CRC error)."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == THREE_ERR_TX_A::B_0x2
    }
    #[doc = "More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == THREE_ERR_TX_A::B_0x3
    }
}
#[doc = "Field `THREE_ERR_TX` writer - Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
pub type THREE_ERR_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, THREE_ERR_TX_A, crate::Safe>;
impl<'a, REG> THREE_ERR_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Less than 3 errors received."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_ERR_TX_A::B_0x0)
    }
    #[doc = "More than 3 errors received, last error is timeout error."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_ERR_TX_A::B_0x1)
    }
    #[doc = "More than 3 errors received, last error is data error (CRC error)."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_ERR_TX_A::B_0x2)
    }
    #[doc = "More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_ERR_TX_A::B_0x3)
    }
}
#[doc = "Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THREE_ERR_RX_A {
    #[doc = "0: Less than 3 errors received."]
    B_0x0 = 0,
    #[doc = "1: More than 3 errors received, last error is timeout error."]
    B_0x1 = 1,
    #[doc = "2: More than 3 errors received, last error is data error (CRC error)."]
    B_0x2 = 2,
    #[doc = "3: More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    B_0x3 = 3,
}
impl From<THREE_ERR_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: THREE_ERR_RX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for THREE_ERR_RX_A {
    type Ux = u8;
}
impl crate::IsEnum for THREE_ERR_RX_A {}
#[doc = "Field `THREE_ERR_RX` reader - Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
pub type THREE_ERR_RX_R = crate::FieldReader<THREE_ERR_RX_A>;
impl THREE_ERR_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THREE_ERR_RX_A {
        match self.bits {
            0 => THREE_ERR_RX_A::B_0x0,
            1 => THREE_ERR_RX_A::B_0x1,
            2 => THREE_ERR_RX_A::B_0x2,
            3 => THREE_ERR_RX_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Less than 3 errors received."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == THREE_ERR_RX_A::B_0x0
    }
    #[doc = "More than 3 errors received, last error is timeout error."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == THREE_ERR_RX_A::B_0x1
    }
    #[doc = "More than 3 errors received, last error is data error (CRC error)."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == THREE_ERR_RX_A::B_0x2
    }
    #[doc = "More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == THREE_ERR_RX_A::B_0x3
    }
}
#[doc = "Field `THREE_ERR_RX` writer - Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
pub type THREE_ERR_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, THREE_ERR_RX_A, crate::Safe>;
impl<'a, REG> THREE_ERR_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Less than 3 errors received."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_ERR_RX_A::B_0x0)
    }
    #[doc = "More than 3 errors received, last error is timeout error."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_ERR_RX_A::B_0x1)
    }
    #[doc = "More than 3 errors received, last error is data error (CRC error)."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_ERR_RX_A::B_0x2)
    }
    #[doc = "More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_ERR_RX_A::B_0x3)
    }
}
impl R {
    #[doc = "Bits 0:3 - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
    #[inline(always)]
    pub fn EA(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written. Host mode Same as VTRX behavior but for USB OUT and SETUP transactions."]
    #[inline(always)]
    pub fn VTTX(&self) -> VTTX_R {
        VTTX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the UTYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints and usage in Device mode. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered 'STALL' instead of 'ACK'. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
    #[inline(always)]
    pub fn EPKIND(&self) -> EPKIND_R {
        EPKIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - USB type of transaction These bits configure the behavior of this endpoint/channel as described in Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of isochronous channels/endpoints is explained in transfers in Device mode"]
    #[inline(always)]
    pub fn UTYPE(&self) -> UTYPE_R {
        UTYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed Device mode This bit is read-only and it is set by the hardware when the last completed transaction is a SETUP. This bit changes its value only for control endpoints. It must be examined, in the case of a successful receive transaction (VTRX event), to determine the type of transaction occurred. To protect the interrupt service routine from the changes in SETUP bits due to next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when VTRX is at 0. This bit is read-only. Host mode This bit is set by the software to send a SETUP transaction on a control endpoint. This bit changes its value only for control endpoints. It is cleared by hardware when the SETUP transaction is acknowledged and VTTX interrupt generated."]
    #[inline(always)]
    pub fn SETUP(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STATRX field of this register. One NAKed transaction keeps pending and is automatically retried by the host at the next frame, or the host can immediately retry by resetting STATRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STATRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STATRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STATRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STATRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. - A transaction ended with error sets this bit. Errors can be seen via the bits ERR_RX (host mode only). This bit is read/write but only 0 can be written, writing 1 has no effect."]
    #[inline(always)]
    pub fn VTRX(&self) -> VTRX_R {
        VTRX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Host mode Device address assigned to the endpoint during the enumeration process."]
    #[inline(always)]
    pub fn DEVADDR(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Host mode This bit is set by the hardware when a device responds with a NAK. Software can use this bit to monitor the number of NAKs received from a device."]
    #[inline(always)]
    pub fn NAK(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Low speed endpoint host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint."]
    #[inline(always)]
    pub fn LS_EP(&self) -> LS_EP_R {
        LS_EP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Received error for an OUT/SETUP transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
    #[inline(always)]
    pub fn ERR_TX(&self) -> ERR_TX_R {
        ERR_TX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Received error for an IN transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
    #[inline(always)]
    pub fn ERR_RX(&self) -> ERR_RX_R {
        ERR_RX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
    #[inline(always)]
    pub fn THREE_ERR_TX(&self) -> THREE_ERR_TX_R {
        THREE_ERR_TX_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
    #[inline(always)]
    pub fn THREE_ERR_RX(&self) -> THREE_ERR_RX_R {
        THREE_ERR_RX_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
    #[inline(always)]
    pub fn EA(&mut self) -> EA_W<'_, CHEP6R_SPEC> {
        EA_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes 0, the value remains unchanged, while writing 1 makes the bit value to toggle. Hardware sets the STATTX bits to NAK, when a correct transfer has occurred (VTTX = 1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints and usage in Device mode). If the endpoint is defined as isochronous, its status can only be 'VALID' or 'DISABLED'. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STATTX bits to 'STALL' or 'NAK' for an isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing 1. Host mode The STATTX bits contain the information about the channel status. Refer to for the full descriptions ('Host mode' descriptions). Whereas in Device mode, these bits contain the status that are given out on the following transaction, in Host mode they capture the status last received from the device. If a NAK is received, STATTX contains the value indicating NAK."]
    #[inline(always)]
    pub fn STATTX(&mut self) -> STATTX_W<'_, CHEP6R_SPEC> {
        STATTX_W::new(self, 4)
    }
    #[doc = "Bit 6 - Data toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (0 = DATA0, 1 = DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint. If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to Device mode) If the endpoint/channel is isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (refer to ). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes 0, the value of DTOGTX remains unchanged, while writing 1 makes the bit value to toggle. This bit is read/write but it can only be toggled by writing 1."]
    #[inline(always)]
    pub fn DTOGTX(&mut self) -> DTOGTX_W<'_, CHEP6R_SPEC> {
        DTOGTX_W::new(self, 6)
    }
    #[doc = "Bit 7 - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written. Host mode Same as VTRX behavior but for USB OUT and SETUP transactions."]
    #[inline(always)]
    pub fn VTTX(&mut self) -> VTTX_W<'_, CHEP6R_SPEC> {
        VTTX_W::new(self, 7)
    }
    #[doc = "Bit 8 - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the UTYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints and usage in Device mode. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered 'STALL' instead of 'ACK'. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
    #[inline(always)]
    pub fn EPKIND(&mut self) -> EPKIND_W<'_, CHEP6R_SPEC> {
        EPKIND_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - USB type of transaction These bits configure the behavior of this endpoint/channel as described in Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of isochronous channels/endpoints is explained in transfers in Device mode"]
    #[inline(always)]
    pub fn UTYPE(&mut self) -> UTYPE_W<'_, CHEP6R_SPEC> {
        UTYPE_W::new(self, 9)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page 2492. These bits can be toggled by software to initialize their value. When the application software writes 0, the value remains unchanged, while writing 1 makes the bit value to toggle. Hardware sets the STATRX bits to NAK when a correct transfer has occurred (VTRX = 1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledges a new transaction. Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints and usage in Device mode). If the endpoint is defined as isochronous, its status can be only 'VALID' or 'DISABLED', so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STATRX bits to 'STALL' or 'NAK' for an isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing 1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STATRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the host execution list. If the aborted transaction was already under execution it is regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID A host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the host frame scheduler to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel is re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate."]
    #[inline(always)]
    pub fn STATRX(&mut self) -> STATRX_W<'_, CHEP6R_SPEC> {
        STATRX_W::new(self, 12)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers If the endpoint/channel is not isochronous, this bit contains the expected value of the data toggle bit (0 = DATA0, 1 = DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device) or acknowledged by device (in host). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to Device mode). If the endpoint/channel is isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Isochronous transfers in Device mode). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes 0, the value of DTOGRX remains unchanged, while writing 1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1."]
    #[inline(always)]
    pub fn DTOGRX(&mut self) -> DTOGRX_W<'_, CHEP6R_SPEC> {
        DTOGRX_W::new(self, 14)
    }
    #[doc = "Bit 15 - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STATRX field of this register. One NAKed transaction keeps pending and is automatically retried by the host at the next frame, or the host can immediately retry by resetting STATRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STATRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STATRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STATRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STATRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. - A transaction ended with error sets this bit. Errors can be seen via the bits ERR_RX (host mode only). This bit is read/write but only 0 can be written, writing 1 has no effect."]
    #[inline(always)]
    pub fn VTRX(&mut self) -> VTRX_W<'_, CHEP6R_SPEC> {
        VTRX_W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Host mode Device address assigned to the endpoint during the enumeration process."]
    #[inline(always)]
    pub fn DEVADDR(&mut self) -> DEVADDR_W<'_, CHEP6R_SPEC> {
        DEVADDR_W::new(self, 16)
    }
    #[doc = "Bit 23 - Host mode This bit is set by the hardware when a device responds with a NAK. Software can use this bit to monitor the number of NAKs received from a device."]
    #[inline(always)]
    pub fn NAK(&mut self) -> NAK_W<'_, CHEP6R_SPEC> {
        NAK_W::new(self, 23)
    }
    #[doc = "Bit 24 - Low speed endpoint host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint."]
    #[inline(always)]
    pub fn LS_EP(&mut self) -> LS_EP_W<'_, CHEP6R_SPEC> {
        LS_EP_W::new(self, 24)
    }
    #[doc = "Bit 25 - Received error for an OUT/SETUP transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
    #[inline(always)]
    pub fn ERR_TX(&mut self) -> ERR_TX_W<'_, CHEP6R_SPEC> {
        ERR_TX_W::new(self, 25)
    }
    #[doc = "Bit 26 - Received error for an IN transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
    #[inline(always)]
    pub fn ERR_RX(&mut self) -> ERR_RX_W<'_, CHEP6R_SPEC> {
        ERR_RX_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
    #[inline(always)]
    pub fn THREE_ERR_TX(&mut self) -> THREE_ERR_TX_W<'_, CHEP6R_SPEC> {
        THREE_ERR_TX_W::new(self, 27)
    }
    #[doc = "Bits 29:30 - Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
    #[inline(always)]
    pub fn THREE_ERR_RX(&mut self) -> THREE_ERR_RX_W<'_, CHEP6R_SPEC> {
        THREE_ERR_RX_W::new(self, 29)
    }
}
#[doc = "USB endpoint/channel 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`chep6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHEP6R_SPEC;
impl crate::RegisterSpec for CHEP6R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chep6r::R`](R) reader structure"]
impl crate::Readable for CHEP6R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chep6r::W`](W) writer structure"]
impl crate::Writable for CHEP6R_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CHEP6R to value 0"]
impl crate::Resettable for CHEP6R_SPEC {}
