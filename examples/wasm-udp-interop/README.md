# WASM-UDP interop

An example showing how a capsule can be made to receive, modify and send back UDP datagrams.

## How to run

In this directory, run

    laze build -b nrf52840dk run

This example has only been tested using `network-config-static` which uses the hardcoded `10.41.0.61/24` IP adress.
Look [here](../README.md#networking) for more information about network configuration.