#[doc = "Register `ISTR` reader"]
pub type R = crate::R<ISTR_SPEC>;
#[doc = "Register `ISTR` writer"]
pub type W = crate::W<ISTR_SPEC>;
#[doc = "Field `IDN` reader - Device Endpoint / host channel identification number These bits are written by the hardware according to the host channel or device endpoint number, which generated the interrupt request. If several endpoint/channel transactions are pending, the hardware writes the identification number related to the endpoint/channel having the highest priority defined in the following way: two levels are defined, in order of priority: isochronous and double-buffered bulk channels/endpoints are considered first and then the others are examined. If more than one endpoint/channel from the same set is requesting an interrupt, the IDN bits in USB_ISTR register are assigned according to the lowest requesting register, CHEP0R having the highest priority followed by CHEP1R and so on. The application software can assign a register to each endpoint/channel according to this priority scheme, so as to order the concurring endpoint/channel requests in a suitable way. These bits are read only."]
pub type IDN_R = crate::FieldReader;
#[doc = "Field `DIR` reader - Direction of transaction This bit is written by the hardware according to the direction of the successful transaction, which generated the interrupt request. If DIR bit = 0, VTTX bit is set in the USB_CHEPnR register related to the interrupting endpoint. The interrupting transaction is of IN type (data transmitted by the USB peripheral to the host PC). If DIR bit = 1, VTRX bit or both VTTX/VTRX are set in the USB_CHEPnR register related to the interrupting endpoint. The interrupting transaction is of OUT type (data received by the USB peripheral from the host PC) or two pending transactions are waiting to be processed. This information can be used by the application software to access the USB_CHEPnR bits related to the triggering transaction since it represents the direction having the interrupt pending. This bit is read-only."]
pub type DIR_R = crate::BitReader;
#[doc = "Field `L1REQ` reader - LPM L1 state request Device mode This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type L1REQ_R = crate::BitReader;
#[doc = "Field `L1REQ` writer - LPM L1 state request Device mode This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type L1REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESOF` reader - Expected start of frame Device mode This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1 ms, but if the device does not receive it properly, the suspend timer issues this interrupt. If three consecutive ESOF interrupts are generated (for example three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the suspend timer is not yet locked. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type ESOF_R = crate::BitReader;
#[doc = "Field `ESOF` writer - Expected start of frame Device mode This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1 ms, but if the device does not receive it properly, the suspend timer issues this interrupt. If three consecutive ESOF interrupts are generated (for example three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the suspend timer is not yet locked. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type ESOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1 ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1 ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_DCON` reader - USB reset request (Device mode) or device connect/disconnect (Host mode) Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22 cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is seen for 22 bit times consecutively from connected state."]
pub type RST_DCON_R = crate::BitReader;
#[doc = "Field `RST_DCON` writer - USB reset request (Device mode) or device connect/disconnect (Host mode) Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22 cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is seen for 22 bit times consecutively from connected state."]
pub type RST_DCON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Suspend mode request Device mode This bit is set by the hardware when no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspend mode request Device mode This bit is set by the hardware when no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP` reader - Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the SUSPRDY bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (for example wakeup unit) about the start of the resume process. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type WKUP_R = crate::BitReader;
#[doc = "Field `WKUP` writer - Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the SUSPRDY bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (for example wakeup unit) about the start of the resume process. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type WKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic redundancy check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (for example loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic redundancy check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (for example loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAOVR` reader - Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host retries the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type PMAOVR_R = crate::BitReader;
#[doc = "Field `PMAOVR` writer - Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host retries the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type PMAOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - Completed transfer in host mode This bit is set by the hardware to indicate that an endpoint/channel has successfully completed a transaction; using DIR and IDN bits software can determine which endpoint/channel requested the interrupt. This bit is read-only."]
pub type CTR_R = crate::BitReader;
#[doc = "Field `THR512` reader - 512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel."]
pub type THR512_R = crate::BitReader;
#[doc = "Field `THR512` writer - 512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel."]
pub type THR512_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDISC` reader - Device connection Host mode This bit is set when a device connection is detected. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type DDISC_R = crate::BitReader;
#[doc = "Field `DDISC` writer - Device connection Host mode This bit is set when a device connection is detected. This bit is read/write but only 0 can be written and writing 1 has no effect."]
pub type DDISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Device connection status Host mode: This bit contains information about device connection status. It is set by hardware when a LS/FS device is attached to the host while it is reset when the device is disconnected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCON_STAT_A {
    #[doc = "0: No device connected"]
    B_0x0 = 0,
    #[doc = "1: FS or LS device connected to the host"]
    B_0x1 = 1,
}
impl From<DCON_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: DCON_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCON_STAT` reader - Device connection status Host mode: This bit contains information about device connection status. It is set by hardware when a LS/FS device is attached to the host while it is reset when the device is disconnected."]
pub type DCON_STAT_R = crate::BitReader<DCON_STAT_A>;
impl DCON_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCON_STAT_A {
        match self.bits {
            false => DCON_STAT_A::B_0x0,
            true => DCON_STAT_A::B_0x1,
        }
    }
    #[doc = "No device connected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DCON_STAT_A::B_0x0
    }
    #[doc = "FS or LS device connected to the host"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DCON_STAT_A::B_0x1
    }
}
#[doc = "Field `LS_DCON` reader - Low speed device connected Host mode: This bit is set by hardware when an LS device connection is detected. Device connection is signaled after LS J-state is sampled for 22 consecutive cycles of the USB clock (48 MHz) from the unconnected state."]
pub type LS_DCON_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Device Endpoint / host channel identification number These bits are written by the hardware according to the host channel or device endpoint number, which generated the interrupt request. If several endpoint/channel transactions are pending, the hardware writes the identification number related to the endpoint/channel having the highest priority defined in the following way: two levels are defined, in order of priority: isochronous and double-buffered bulk channels/endpoints are considered first and then the others are examined. If more than one endpoint/channel from the same set is requesting an interrupt, the IDN bits in USB_ISTR register are assigned according to the lowest requesting register, CHEP0R having the highest priority followed by CHEP1R and so on. The application software can assign a register to each endpoint/channel according to this priority scheme, so as to order the concurring endpoint/channel requests in a suitable way. These bits are read only."]
    #[inline(always)]
    pub fn IDN(&self) -> IDN_R {
        IDN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction This bit is written by the hardware according to the direction of the successful transaction, which generated the interrupt request. If DIR bit = 0, VTTX bit is set in the USB_CHEPnR register related to the interrupting endpoint. The interrupting transaction is of IN type (data transmitted by the USB peripheral to the host PC). If DIR bit = 1, VTRX bit or both VTTX/VTRX are set in the USB_CHEPnR register related to the interrupting endpoint. The interrupting transaction is of OUT type (data received by the USB peripheral from the host PC) or two pending transactions are waiting to be processed. This information can be used by the application software to access the USB_CHEPnR bits related to the triggering transaction since it represents the direction having the interrupt pending. This bit is read-only."]
    #[inline(always)]
    pub fn DIR(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request Device mode This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn L1REQ(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame Device mode This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1 ms, but if the device does not receive it properly, the suspend timer issues this interrupt. If three consecutive ESOF interrupts are generated (for example three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the suspend timer is not yet locked. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn ESOF(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1 ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn SOF(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22 cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is seen for 22 bit times consecutively from connected state."]
    #[inline(always)]
    pub fn RST_DCON(&self) -> RST_DCON_R {
        RST_DCON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode request Device mode This bit is set by the hardware when no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn SUSP(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the SUSPRDY bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (for example wakeup unit) about the start of the resume process. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn WKUP(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic redundancy check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (for example loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn ERR(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host retries the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn PMAOVR(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Completed transfer in host mode This bit is set by the hardware to indicate that an endpoint/channel has successfully completed a transaction; using DIR and IDN bits software can determine which endpoint/channel requested the interrupt. This bit is read-only."]
    #[inline(always)]
    pub fn CTR(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel."]
    #[inline(always)]
    pub fn THR512(&self) -> THR512_R {
        THR512_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device connection Host mode This bit is set when a device connection is detected. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn DDISC(&self) -> DDISC_R {
        DDISC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 29 - Device connection status Host mode: This bit contains information about device connection status. It is set by hardware when a LS/FS device is attached to the host while it is reset when the device is disconnected."]
    #[inline(always)]
    pub fn DCON_STAT(&self) -> DCON_STAT_R {
        DCON_STAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low speed device connected Host mode: This bit is set by hardware when an LS device connection is detected. Device connection is signaled after LS J-state is sampled for 22 consecutive cycles of the USB clock (48 MHz) from the unconnected state."]
    #[inline(always)]
    pub fn LS_DCON(&self) -> LS_DCON_R {
        LS_DCON_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - LPM L1 state request Device mode This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn L1REQ(&mut self) -> L1REQ_W<'_, ISTR_SPEC> {
        L1REQ_W::new(self, 7)
    }
    #[doc = "Bit 8 - Expected start of frame Device mode This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1 ms, but if the device does not receive it properly, the suspend timer issues this interrupt. If three consecutive ESOF interrupts are generated (for example three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the suspend timer is not yet locked. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn ESOF(&mut self) -> ESOF_W<'_, ISTR_SPEC> {
        ESOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1 ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn SOF(&mut self) -> SOF_W<'_, ISTR_SPEC> {
        SOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22 cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is seen for 22 bit times consecutively from connected state."]
    #[inline(always)]
    pub fn RST_DCON(&mut self) -> RST_DCON_W<'_, ISTR_SPEC> {
        RST_DCON_W::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend mode request Device mode This bit is set by the hardware when no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn SUSP(&mut self) -> SUSP_W<'_, ISTR_SPEC> {
        SUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the SUSPRDY bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (for example wakeup unit) about the start of the resume process. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn WKUP(&mut self) -> WKUP_W<'_, ISTR_SPEC> {
        WKUP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic redundancy check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (for example loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn ERR(&mut self) -> ERR_W<'_, ISTR_SPEC> {
        ERR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host retries the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn PMAOVR(&mut self) -> PMAOVR_W<'_, ISTR_SPEC> {
        PMAOVR_W::new(self, 14)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel."]
    #[inline(always)]
    pub fn THR512(&mut self) -> THR512_W<'_, ISTR_SPEC> {
        THR512_W::new(self, 16)
    }
    #[doc = "Bit 17 - Device connection Host mode This bit is set when a device connection is detected. This bit is read/write but only 0 can be written and writing 1 has no effect."]
    #[inline(always)]
    pub fn DDISC(&mut self) -> DDISC_W<'_, ISTR_SPEC> {
        DDISC_W::new(self, 17)
    }
}
#[doc = "USB interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTR_SPEC;
impl crate::RegisterSpec for ISTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istr::R`](R) reader structure"]
impl crate::Readable for ISTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`istr::W`](W) writer structure"]
impl crate::Writable for ISTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for ISTR_SPEC {}
