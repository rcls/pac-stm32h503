#[doc = "Register `CNTR` reader"]
pub type R = crate::R<CNTR_SPEC>;
#[doc = "Register `CNTR` writer"]
pub type W = crate::W<CNTR_SPEC>;
#[doc = "USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRST_A {
    #[doc = "0: No effect"]
    B_0x0_DEVICE_MODE = 0,
    #[doc = "1: USB core is under reset"]
    B_0x1_DEVICE_MODE = 1,
}
impl From<USBRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRST` reader - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
pub type USBRST_R = crate::BitReader<USBRST_A>;
impl USBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBRST_A {
        match self.bits {
            false => USBRST_A::B_0x0_DEVICE_MODE,
            true => USBRST_A::B_0x1_DEVICE_MODE,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_B_0x0_DEVICE_MODE(&self) -> bool {
        *self == USBRST_A::B_0x0_DEVICE_MODE
    }
    #[doc = "USB core is under reset"]
    #[inline(always)]
    pub fn is_B_0x1_DEVICE_MODE(&self) -> bool {
        *self == USBRST_A::B_0x1_DEVICE_MODE
    }
}
#[doc = "Field `USBRST` writer - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG, USBRST_A>;
impl<'a, REG> USBRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn B_0x0_DEVICE_MODE(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST_A::B_0x0_DEVICE_MODE)
    }
    #[doc = "USB core is under reset"]
    #[inline(always)]
    pub fn B_0x1_DEVICE_MODE(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST_A::B_0x1_DEVICE_MODE)
    }
}
#[doc = "Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDWN_A {
    #[doc = "0: Exit power down."]
    B_0x0 = 0,
    #[doc = "1: Enter power down mode."]
    B_0x1 = 1,
}
impl From<PDWN_A> for bool {
    #[inline(always)]
    fn from(variant: PDWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWN` reader - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
pub type PDWN_R = crate::BitReader<PDWN_A>;
impl PDWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDWN_A {
        match self.bits {
            false => PDWN_A::B_0x0,
            true => PDWN_A::B_0x1,
        }
    }
    #[doc = "Exit power down."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PDWN_A::B_0x0
    }
    #[doc = "Enter power down mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PDWN_A::B_0x1
    }
}
#[doc = "Field `PDWN` writer - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
pub type PDWN_W<'a, REG> = crate::BitWriter<'a, REG, PDWN_A>;
impl<'a, REG> PDWN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exit power down."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PDWN_A::B_0x0)
    }
    #[doc = "Enter power down mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PDWN_A::B_0x1)
    }
}
#[doc = "Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPRDY_A {
    #[doc = "0: Normal operation"]
    B_0x0 = 0,
    #[doc = "1: Suspend state"]
    B_0x1 = 1,
}
impl From<SUSPRDY_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPRDY` reader - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
pub type SUSPRDY_R = crate::BitReader<SUSPRDY_A>;
impl SUSPRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPRDY_A {
        match self.bits {
            false => SUSPRDY_A::B_0x0,
            true => SUSPRDY_A::B_0x1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SUSPRDY_A::B_0x0
    }
    #[doc = "Suspend state"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SUSPRDY_A::B_0x1
    }
}
#[doc = "Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPEN_A {
    #[doc = "0: No effect."]
    B_0x0_DEVICE_MODE = 0,
    #[doc = "1: Enter L1/L2 suspend"]
    B_0x1_DEVICE_MODE = 1,
}
impl From<SUSPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPEN` reader - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
pub type SUSPEN_R = crate::BitReader<SUSPEN_A>;
impl SUSPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPEN_A {
        match self.bits {
            false => SUSPEN_A::B_0x0_DEVICE_MODE,
            true => SUSPEN_A::B_0x1_DEVICE_MODE,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_B_0x0_DEVICE_MODE(&self) -> bool {
        *self == SUSPEN_A::B_0x0_DEVICE_MODE
    }
    #[doc = "Enter L1/L2 suspend"]
    #[inline(always)]
    pub fn is_B_0x1_DEVICE_MODE(&self) -> bool {
        *self == SUSPEN_A::B_0x1_DEVICE_MODE
    }
}
#[doc = "Field `SUSPEN` writer - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
pub type SUSPEN_W<'a, REG> = crate::BitWriter<'a, REG, SUSPEN_A>;
impl<'a, REG> SUSPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn B_0x0_DEVICE_MODE(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPEN_A::B_0x0_DEVICE_MODE)
    }
    #[doc = "Enter L1/L2 suspend"]
    #[inline(always)]
    pub fn B_0x1_DEVICE_MODE(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPEN_A::B_0x1_DEVICE_MODE)
    }
}
#[doc = "L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2RES_A {
    #[doc = "0: No effect"]
    B_0x0 = 0,
    #[doc = "1: Send L2 resume signaling to device"]
    B_0x1 = 1,
}
impl From<L2RES_A> for bool {
    #[inline(always)]
    fn from(variant: L2RES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L2RES` reader - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
pub type L2RES_R = crate::BitReader<L2RES_A>;
impl L2RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2RES_A {
        match self.bits {
            false => L2RES_A::B_0x0,
            true => L2RES_A::B_0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == L2RES_A::B_0x0
    }
    #[doc = "Send L2 resume signaling to device"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == L2RES_A::B_0x1
    }
}
#[doc = "Field `L2RES` writer - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
pub type L2RES_W<'a, REG> = crate::BitWriter<'a, REG, L2RES_A>;
impl<'a, REG> L2RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L2RES_A::B_0x0)
    }
    #[doc = "Send L2 resume signaling to device"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L2RES_A::B_0x1)
    }
}
#[doc = "L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 us remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RES_A {
    #[doc = "0: No effect"]
    B_0x0 = 0,
    #[doc = "1: Send 50 us remote-wakeup signaling to host"]
    B_0x1 = 1,
}
impl From<L1RES_A> for bool {
    #[inline(always)]
    fn from(variant: L1RES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1RES` reader - L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 us remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware."]
pub type L1RES_R = crate::BitReader<L1RES_A>;
impl L1RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1RES_A {
        match self.bits {
            false => L1RES_A::B_0x0,
            true => L1RES_A::B_0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == L1RES_A::B_0x0
    }
    #[doc = "Send 50 us remote-wakeup signaling to host"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == L1RES_A::B_0x1
    }
}
#[doc = "Field `L1RES` writer - L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 us remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware."]
pub type L1RES_W<'a, REG> = crate::BitWriter<'a, REG, L1RES_A>;
impl<'a, REG> L1RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L1RES_A::B_0x0)
    }
    #[doc = "Send 50 us remote-wakeup signaling to host"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L1RES_A::B_0x1)
    }
}
#[doc = "LPM L1 state request interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQM_A {
    #[doc = "0: LPM L1 state request (L1REQ) interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<L1REQM_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1REQM` reader - LPM L1 state request interrupt mask"]
pub type L1REQM_R = crate::BitReader<L1REQM_A>;
impl L1REQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1REQM_A {
        match self.bits {
            false => L1REQM_A::B_0x0,
            true => L1REQM_A::B_0x1,
        }
    }
    #[doc = "LPM L1 state request (L1REQ) interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == L1REQM_A::B_0x0
    }
    #[doc = "L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == L1REQM_A::B_0x1
    }
}
#[doc = "Field `L1REQM` writer - LPM L1 state request interrupt mask"]
pub type L1REQM_W<'a, REG> = crate::BitWriter<'a, REG, L1REQM_A>;
impl<'a, REG> L1REQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPM L1 state request (L1REQ) interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQM_A::B_0x0)
    }
    #[doc = "L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQM_A::B_0x1)
    }
}
#[doc = "Expected start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFM_A {
    #[doc = "0: Expected start of frame (ESOF) interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<ESOFM_A> for bool {
    #[inline(always)]
    fn from(variant: ESOFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub type ESOFM_R = crate::BitReader<ESOFM_A>;
impl ESOFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESOFM_A {
        match self.bits {
            false => ESOFM_A::B_0x0,
            true => ESOFM_A::B_0x1,
        }
    }
    #[doc = "Expected start of frame (ESOF) interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ESOFM_A::B_0x0
    }
    #[doc = "ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ESOFM_A::B_0x1
    }
}
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub type ESOFM_W<'a, REG> = crate::BitWriter<'a, REG, ESOFM_A>;
impl<'a, REG> ESOFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Expected start of frame (ESOF) interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFM_A::B_0x0)
    }
    #[doc = "ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFM_A::B_0x1)
    }
}
#[doc = "Start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFM_A {
    #[doc = "0: SOF interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<SOFM_A> for bool {
    #[inline(always)]
    fn from(variant: SOFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub type SOFM_R = crate::BitReader<SOFM_A>;
impl SOFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFM_A {
        match self.bits {
            false => SOFM_A::B_0x0,
            true => SOFM_A::B_0x1,
        }
    }
    #[doc = "SOF interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SOFM_A::B_0x0
    }
    #[doc = "SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SOFM_A::B_0x1
    }
}
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG, SOFM_A>;
impl<'a, REG> SOFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOF interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SOFM_A::B_0x0)
    }
    #[doc = "SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SOFM_A::B_0x1)
    }
}
#[doc = "USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_DCONM_A {
    #[doc = "0: RESET interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<RST_DCONM_A> for bool {
    #[inline(always)]
    fn from(variant: RST_DCONM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_DCONM` reader - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
pub type RST_DCONM_R = crate::BitReader<RST_DCONM_A>;
impl RST_DCONM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST_DCONM_A {
        match self.bits {
            false => RST_DCONM_A::B_0x0,
            true => RST_DCONM_A::B_0x1,
        }
    }
    #[doc = "RESET interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RST_DCONM_A::B_0x0
    }
    #[doc = "RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RST_DCONM_A::B_0x1
    }
}
#[doc = "Field `RST_DCONM` writer - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
pub type RST_DCONM_W<'a, REG> = crate::BitWriter<'a, REG, RST_DCONM_A>;
impl<'a, REG> RST_DCONM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RESET interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RST_DCONM_A::B_0x0)
    }
    #[doc = "RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RST_DCONM_A::B_0x1)
    }
}
#[doc = "Suspend mode interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPM_A {
    #[doc = "0: Suspend mode request (SUSP) interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<SUSPM_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub type SUSPM_R = crate::BitReader<SUSPM_A>;
impl SUSPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPM_A {
        match self.bits {
            false => SUSPM_A::B_0x0,
            true => SUSPM_A::B_0x1,
        }
    }
    #[doc = "Suspend mode request (SUSP) interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SUSPM_A::B_0x0
    }
    #[doc = "SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SUSPM_A::B_0x1
    }
}
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub type SUSPM_W<'a, REG> = crate::BitWriter<'a, REG, SUSPM_A>;
impl<'a, REG> SUSPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Suspend mode request (SUSP) interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPM_A::B_0x0)
    }
    #[doc = "SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPM_A::B_0x1)
    }
}
#[doc = "Wakeup interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPM_A {
    #[doc = "0: WKUP interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<WKUPM_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub type WKUPM_R = crate::BitReader<WKUPM_A>;
impl WKUPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKUPM_A {
        match self.bits {
            false => WKUPM_A::B_0x0,
            true => WKUPM_A::B_0x1,
        }
    }
    #[doc = "WKUP interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WKUPM_A::B_0x0
    }
    #[doc = "WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WKUPM_A::B_0x1
    }
}
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub type WKUPM_W<'a, REG> = crate::BitWriter<'a, REG, WKUPM_A>;
impl<'a, REG> WKUPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPM_A::B_0x0)
    }
    #[doc = "WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPM_A::B_0x1)
    }
}
#[doc = "Error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRM_A {
    #[doc = "0: ERR interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<ERRM_A> for bool {
    #[inline(always)]
    fn from(variant: ERRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub type ERRM_R = crate::BitReader<ERRM_A>;
impl ERRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRM_A {
        match self.bits {
            false => ERRM_A::B_0x0,
            true => ERRM_A::B_0x1,
        }
    }
    #[doc = "ERR interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ERRM_A::B_0x0
    }
    #[doc = "ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ERRM_A::B_0x1
    }
}
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub type ERRM_W<'a, REG> = crate::BitWriter<'a, REG, ERRM_A>;
impl<'a, REG> ERRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERR interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERRM_A::B_0x0)
    }
    #[doc = "ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRM_A::B_0x1)
    }
}
#[doc = "Packet memory area over / underrun interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRM_A {
    #[doc = "0: PMAOVR interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<PMAOVRM_A> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_R = crate::BitReader<PMAOVRM_A>;
impl PMAOVRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMAOVRM_A {
        match self.bits {
            false => PMAOVRM_A::B_0x0,
            true => PMAOVRM_A::B_0x1,
        }
    }
    #[doc = "PMAOVR interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PMAOVRM_A::B_0x0
    }
    #[doc = "PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PMAOVRM_A::B_0x1
    }
}
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_W<'a, REG> = crate::BitWriter<'a, REG, PMAOVRM_A>;
impl<'a, REG> PMAOVRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PMAOVR interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRM_A::B_0x0)
    }
    #[doc = "PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRM_A::B_0x1)
    }
}
#[doc = "Correct transfer interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRM_A {
    #[doc = "0: Correct transfer (CTR) interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0x1 = 1,
}
impl From<CTRM_A> for bool {
    #[inline(always)]
    fn from(variant: CTRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub type CTRM_R = crate::BitReader<CTRM_A>;
impl CTRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTRM_A {
        match self.bits {
            false => CTRM_A::B_0x0,
            true => CTRM_A::B_0x1,
        }
    }
    #[doc = "Correct transfer (CTR) interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CTRM_A::B_0x0
    }
    #[doc = "CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CTRM_A::B_0x1
    }
}
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub type CTRM_W<'a, REG> = crate::BitWriter<'a, REG, CTRM_A>;
impl<'a, REG> CTRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Correct transfer (CTR) interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CTRM_A::B_0x0)
    }
    #[doc = "CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CTRM_A::B_0x1)
    }
}
#[doc = "512 byte threshold interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THR512M_A {
    #[doc = "0: 512 byte threshold interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: 512 byte threshold interrupt enabled"]
    B_0x1 = 1,
}
impl From<THR512M_A> for bool {
    #[inline(always)]
    fn from(variant: THR512M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THR512M` reader - 512 byte threshold interrupt mask"]
pub type THR512M_R = crate::BitReader<THR512M_A>;
impl THR512M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THR512M_A {
        match self.bits {
            false => THR512M_A::B_0x0,
            true => THR512M_A::B_0x1,
        }
    }
    #[doc = "512 byte threshold interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == THR512M_A::B_0x0
    }
    #[doc = "512 byte threshold interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == THR512M_A::B_0x1
    }
}
#[doc = "Field `THR512M` writer - 512 byte threshold interrupt mask"]
pub type THR512M_W<'a, REG> = crate::BitWriter<'a, REG, THR512M_A>;
impl<'a, REG> THR512M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "512 byte threshold interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(THR512M_A::B_0x0)
    }
    #[doc = "512 byte threshold interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(THR512M_A::B_0x1)
    }
}
#[doc = "Device disconnection mask Host mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDISCM_A {
    #[doc = "0: Device disconnection interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Device disconnection interrupt enabled"]
    B_0x1 = 1,
}
impl From<DDISCM_A> for bool {
    #[inline(always)]
    fn from(variant: DDISCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDISCM` reader - Device disconnection mask Host mode"]
pub type DDISCM_R = crate::BitReader<DDISCM_A>;
impl DDISCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDISCM_A {
        match self.bits {
            false => DDISCM_A::B_0x0,
            true => DDISCM_A::B_0x1,
        }
    }
    #[doc = "Device disconnection interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DDISCM_A::B_0x0
    }
    #[doc = "Device disconnection interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DDISCM_A::B_0x1
    }
}
#[doc = "Field `DDISCM` writer - Device disconnection mask Host mode"]
pub type DDISCM_W<'a, REG> = crate::BitWriter<'a, REG, DDISCM_A>;
impl<'a, REG> DDISCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device disconnection interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DDISCM_A::B_0x0)
    }
    #[doc = "Device disconnection interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DDISCM_A::B_0x1)
    }
}
#[doc = "HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOST_A {
    #[doc = "0: USB Device function"]
    B_0x0 = 0,
    #[doc = "1: USB host function"]
    B_0x1 = 1,
}
impl From<HOST_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST` reader - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
pub type HOST_R = crate::BitReader<HOST_A>;
impl HOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HOST_A {
        match self.bits {
            false => HOST_A::B_0x0,
            true => HOST_A::B_0x1,
        }
    }
    #[doc = "USB Device function"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HOST_A::B_0x0
    }
    #[doc = "USB host function"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HOST_A::B_0x1
    }
}
#[doc = "Field `HOST` writer - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
pub type HOST_W<'a, REG> = crate::BitWriter<'a, REG, HOST_A>;
impl<'a, REG> HOST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB Device function"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HOST_A::B_0x0)
    }
    #[doc = "USB host function"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HOST_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
    #[inline(always)]
    pub fn USBRST(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
    #[inline(always)]
    pub fn PDWN(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
    #[inline(always)]
    pub fn SUSPRDY(&self) -> SUSPRDY_R {
        SUSPRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
    #[inline(always)]
    pub fn SUSPEN(&self) -> SUSPEN_R {
        SUSPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
    #[inline(always)]
    pub fn L2RES(&self) -> L2RES_R {
        L2RES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 us remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn L1RES(&self) -> L1RES_R {
        L1RES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn L1REQM(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn ESOFM(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn SOFM(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
    #[inline(always)]
    pub fn RST_DCONM(&self) -> RST_DCONM_R {
        RST_DCONM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn SUSPM(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn WKUPM(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn ERRM(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn PMAOVRM(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn CTRM(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt mask"]
    #[inline(always)]
    pub fn THR512M(&self) -> THR512M_R {
        THR512M_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device disconnection mask Host mode"]
    #[inline(always)]
    pub fn DDISCM(&self) -> DDISCM_R {
        DDISCM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
    #[inline(always)]
    pub fn HOST(&self) -> HOST_R {
        HOST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
    #[inline(always)]
    pub fn USBRST(&mut self) -> USBRST_W<'_, CNTR_SPEC> {
        USBRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
    #[inline(always)]
    pub fn PDWN(&mut self) -> PDWN_W<'_, CNTR_SPEC> {
        PDWN_W::new(self, 1)
    }
    #[doc = "Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3 ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY = 1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
    #[inline(always)]
    pub fn SUSPEN(&mut self) -> SUSPEN_W<'_, CNTR_SPEC> {
        SUSPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1 ms and no more than 15 ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
    #[inline(always)]
    pub fn L2RES(&mut self) -> L2RES_W<'_, CNTR_SPEC> {
        L2RES_W::new(self, 4)
    }
    #[doc = "Bit 5 - L1 remote wakeup / resume driver Device mode Software sets this bit to send a LPM L1 50 us remote wakeup signaling to the host. After the signaling ends, this bit is cleared by hardware."]
    #[inline(always)]
    pub fn L1RES(&mut self) -> L1RES_W<'_, CNTR_SPEC> {
        L1RES_W::new(self, 5)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn L1REQM(&mut self) -> L1REQM_W<'_, CNTR_SPEC> {
        L1REQM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn ESOFM(&mut self) -> ESOFM_W<'_, CNTR_SPEC> {
        ESOFM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn SOFM(&mut self) -> SOFM_W<'_, CNTR_SPEC> {
        SOFM_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
    #[inline(always)]
    pub fn RST_DCONM(&mut self) -> RST_DCONM_W<'_, CNTR_SPEC> {
        RST_DCONM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn SUSPM(&mut self) -> SUSPM_W<'_, CNTR_SPEC> {
        SUSPM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn WKUPM(&mut self) -> WKUPM_W<'_, CNTR_SPEC> {
        WKUPM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn ERRM(&mut self) -> ERRM_W<'_, CNTR_SPEC> {
        ERRM_W::new(self, 13)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn PMAOVRM(&mut self) -> PMAOVRM_W<'_, CNTR_SPEC> {
        PMAOVRM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn CTRM(&mut self) -> CTRM_W<'_, CNTR_SPEC> {
        CTRM_W::new(self, 15)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt mask"]
    #[inline(always)]
    pub fn THR512M(&mut self) -> THR512M_W<'_, CNTR_SPEC> {
        THR512M_W::new(self, 16)
    }
    #[doc = "Bit 17 - Device disconnection mask Host mode"]
    #[inline(always)]
    pub fn DDISCM(&mut self) -> DDISCM_W<'_, CNTR_SPEC> {
        DDISCM_W::new(self, 17)
    }
    #[doc = "Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
    #[inline(always)]
    pub fn HOST(&mut self) -> HOST_W<'_, CNTR_SPEC> {
        HOST_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTR_SPEC;
impl crate::RegisterSpec for CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntr::W`](W) writer structure"]
impl crate::Writable for CNTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CNTR to value 0x03"]
impl crate::Resettable for CNTR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
