//! Complete the two TODOs in this file.

// similar to #include in C
use embassy_stm32::{Peripherals, can::Can, usart::BufferedUart};
use embassy_stm32::gpio::{Flex,Level,Output,Speed};
use embassy_stm32::usart::Config;
use embassy_stm32::bind_interrupts;

use crate::swd::Swd;

static mut UART_RX_BUF: [u8; 128] = [0u8; 128];
static mut UART_TX_BUF: [u8; 128] = [0u8; 128];

bind_interrupts!(struct Irqs {
    UART7 => embassy_stm32::usart::BufferedInterruptHandler<embassy_stm32::peripherals::UART7>;
});

// construct a structural body
pub struct Ports {
    //construct variables
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // TODO 1: Pick two exposed 3.3 V GPIOs and pass them to Swd::new.
    // Use Output for SWCLK (initially low), and Flex for bidirectional SWDIO.
    // PB0 Output PP
    let swclk = Output::new(p.PA0, Level::Low, Speed::VeryHigh);
    let swdio = Flex::new(p.PA2);
    let swd = Some(Swd::new(swclk, swdio));


    // TODO 2: Configure a buffered UART at 115200 baud, 8N1.
    // Identify the peripheral, RX/TX pins and interrupt from the schematic.
    // Bind the interrupt and provide static RX and TX buffers.
    let mut uart_cfg = Config::default();
    uart_cfg.baudrate = 115200;

    let uart_inst = unsafe {
    let rx_buf: &mut [u8] = &mut *(&raw mut UART_RX_BUF);
    let tx_buf: &mut [u8] = &mut *(&raw mut UART_TX_BUF);
    BufferedUart::new(
        p.UART7,
        p.PE7,
        p.PE8,
        rx_buf,
        tx_buf,
        Irqs,
        uart_cfg,
    )
};



    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart: uart_inst.ok() }, swd)
}
