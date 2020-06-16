OUT_DIR = $(abspath .)/out
LIB_DIR = $(abspath .)/lib

NAME = dsp_sub

IMPL_FILE = \
$(LIB_DIR)/$(NAME).v \

SIM_FILES = \
$(LIB_DIR)/dsp_add.v \
$(LIB_DIR)/dsp_add_v2.v \
$(LIB_DIR)/dsp_add_v4.v \
$(LIB_DIR)/dsp_add_v3.v \
$(LIB_DIR)/dsp_mul.v \
$(LIB_DIR)/dsp_sub.v \
$(LIB_DIR)/dsp_sub_v2.v \
$(LIB_DIR)/dsp_sub_v4.v \
$(LIB_DIR)/dsp_sub_v3.v \
$(LIB_DIR)/dsp_and.v \
$(LIB_DIR)/dsp_or.v \
$(LIB_DIR)/dsp_xor.v

TIME_FILE = $(OUT_DIR)/$(NAME)_time.txt
UTIL_FILE = $(OUT_DIR)/$(NAME)_util.txt
DCP_FILE = $(OUT_DIR)/$(NAME).dcp

default: test

.PHONY: test
test: xvlog xelab

.PHONY: impl
impl: | $(OUT_DIR)
	vivado -mode batch -source impl/vivado.tcl -tclargs $(NAME) $(IMPL_FILE) $(UTIL_FILE) $(TIME_FILE) $(DCP_FILE)

xvlog:
	xvlog --sv --incr --relax test/test.v $(SIM_FILES) $(XILINX_VIVADO)/data/verilog/src/glbl.v

xelab:
	xelab --snapshot test --timescale 1ps/1ps -R -L unisims_ver glbl test

$(OUT_DIR):
	mkdir -p $@

clean:
	-rm -rf *.log *.wdb *.jou *.dir *.pb $(OUT_DIR) hd_visual
