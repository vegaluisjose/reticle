# Run Xilinx Vivado on Docker

## Build instructions

1. Create an account in [Xilinx](https://www.xilinx.com/registration/create-account.html)
2. Download Vivado web installer e.g., for `2020.1` it would be `Xilinx_Unified_2020.1_0602_1208_Lin64.bin`. The file name is set in both `credential.sh` and `install.sh`.
3. Move `Xilinx_Unified_2020.1_0602_1208_Lin64.bin` to the root folder of this repository
4. Make sure docker image size limit is greater than 90GB. The final image is around 35GB.
5. Run `./credential.sh` script for creating auth token and install config. This step should generate a `wi_authentication_key` and `install_config.txt` file. Select option 2 in product list and 1 in edition list when prompted for installing the free edition of Vivado (WebPACK).
6. Run `./install.sh` script to install vivado
