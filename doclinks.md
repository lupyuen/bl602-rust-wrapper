# Links to be injected into the documentation

## GPIO

| Function | Description | Section | URL
| -------- | ----------- | ------- | ---
| bl_gpio_enable_output | Configure a GPIO Pin for Output Mode. | Enable GPIO | https://lupyuen.github.io/articles/led#enable-gpio
| bl_gpio_enable_input | Configure a GPIO Pin for Input Mode. | Enable GPIO | https://lupyuen.github.io/articles/led#enable-gpio
| bl_gpio_output_set | Set the output value of a GPIO Pin. | Read and Write GPIO | https://lupyuen.github.io/articles/led#read-and-write-gpio
| bl_gpio_input_get | Get the input value of a GPIO Pin, by reference. | Read and Write GPIO | https://lupyuen.github.io/articles/led#read-and-write-gpio
| bl_gpio_input_get_value | Get the input value of GPIO Pin. | Read and Write GPIO | https://lupyuen.github.io/articles/led#read-and-write-gpio
| bl_gpio_int_clear | Clear GPIO Interrupt. | GPIO Interrupts | https://lupyuen.github.io/articles/led#gpio-interrupts
| bl_gpio_intmask | Set GPIO Interrupt Mask. | GPIO Interrupts | https://lupyuen.github.io/articles/led#gpio-interrupts
| bl_set_gpio_intmod | Set GPIO Interrupt Mode. | GPIO Interrupts | https://lupyuen.github.io/articles/led#gpio-interrupts
| bl_gpio_register | Register GPIO Interrupt. | GPIO Interrupts | https://lupyuen.github.io/articles/led#gpio-interrupts

## PWM

| Function | Description | Section | URL
| -------- | ----------- | ------- | ---
| bl_pwm_init | Designate a GPIO Pin as a PWM Channel. | Initialise PWM | https://lupyuen.github.io/articles/led#initialise-pwm
| bl_pwm_set_freq | Set the Frequency of a PWM Channel. | PWM Frequency and Duty Cycle | https://lupyuen.github.io/articles/led#pwm-frequency-and-duty-cycle
| bl_pwm_set_duty | Set the Duty Cycle of a PWM Channel. | PWM Frequency and Duty Cycle | https://lupyuen.github.io/articles/led#pwm-frequency-and-duty-cycle
| bl_pwm_get_duty | Get the Duty Cycle of a PWM Channel. | PWM Frequency and Duty Cycle | https://lupyuen.github.io/articles/led#pwm-frequency-and-duty-cycle
| bl_pwm_start | Start a PWM Channel. | PWM Operation | https://lupyuen.github.io/articles/led#pwm-operation
| bl_pwm_stop | Stop a PWM Channel. | PWM Operation | https://lupyuen.github.io/articles/led#pwm-operation

## I2C

| Function | Description | Section | URL
| -------- | ----------- | ------- | ---
| i2c_gpio_init | Init an I2C Port. | Assign I2C Pins and set I2C Frequency | https://lupyuen.github.io/articles/i2c#assign-i2c-pins-and-set-i2c-frequency
| i2c_set_freq | Set the Frequency of an I2C Port. | Assign I2C Pins and set I2C Frequency | https://lupyuen.github.io/articles/i2c#assign-i2c-pins-and-set-i2c-frequency
| I2C_Disable | Disable an I2C Port. | Enable I2C Interrupts | https://lupyuen.github.io/articles/i2c#enable-i2c-interrupts
| I2C_IntMask | Set Interrupt Mask for an I2C Port. | Enable I2C Interrupts | https://lupyuen.github.io/articles/i2c#enable-i2c-interrupts
| i2c_transfer_start | Start an I2C Data Transfer. | Start I2C Transfer | https://lupyuen.github.io/articles/i2c#start-i2c-transfer
| i2c_clear_status | Clear the I2C Error Status for an I2C Port. | Stop I2C Read | https://lupyuen.github.io/articles/i2c#stop-i2c-read

## ADC

| Function | Description | Section | URL
| -------- | ----------- | ------- | ---

## SPI

| Function | Description | Section | URL
| -------- | ----------- | ------- | ---
| spi_init | Initialise an SPI Port. | Initialise SPI Port | https://lupyuen.github.io/articles/spi#initialise-spi-port
| hal_spi_transfer | Execute an SPI Data Transfer. | Execute the SPI Transfers | https://lupyuen.github.io/articles/spi#execute-the-spi-transfers

## WiFi

| Function | Description | Section | URL
| -------- | ----------- | ------- | ---
| hal_wifi_start_firmware_task | Start the WiFi Firmware Task. | Start WiFi Firmware Task | https://lupyuen.github.io/articles/wifi#start-wifi-firmware-task
| wifi_mgmr_start_background | Start the WiFi Manager Task. | Start WiFi Manager Task | https://lupyuen.github.io/articles/wifi#start-wifi-manager-task
| wifi_mgmr_sta_enable | Enable the WiFi Client. | Connect to WiFi Network | https://lupyuen.github.io/articles/wifi#connect-to-wifi-network
| wifi_mgmr_sta_connect | Connect to a WiFi Access Point. | Connect to WiFi Network | https://lupyuen.github.io/articles/wifi#connect-to-wifi-network
