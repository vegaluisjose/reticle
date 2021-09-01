OUT_DIR = $(abspath .)/out

NAME ?= "xilinx_inverter"
IMPL_FILE = $(NAME).v
EDIF_FILE = $(NAME).edf
NETLIST_FILE = $(NAME)_netlist.v
TIME_FILE = $(OUT_DIR)/$(NAME)_time.txt
UTIL_FILE = $(OUT_DIR)/$(NAME)_util.txt
DCP_FILE = $(OUT_DIR)/$(NAME).dcp

.PHONY: all
all: | $(OUT_DIR)
	vivado -mode batch -source load_edif.tcl -tclargs $(NAME) $(IMPL_FILE) $(UTIL_FILE) $(TIME_FILE) $(DCP_FILE) $(NETLIST_FILE) $(EDIF_FILE)


$(OUT_DIR):
	mkdir -p $@

clean-logs:
	rm -rf ./*jou ./*.log

clean:
	rm -rf $(OUT_DIR)