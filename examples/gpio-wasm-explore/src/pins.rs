use ariel_os::hal::peripherals;


#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_14,
    btn1: P0_12
});