set top_name [lindex $argv 0]
set verilog_file [lindex $argv 1]
set utilization_file [lindex $argv 2]
set timing_file [lindex $argv 3]
set dcp_file [lindex $argv 4]
set netlist_file [lindex $argv 5]
set edif_file [lindex $argv 6]
set edif_file_placed [lindex $argv 6].placed
set edif_file_routed [lindex $argv 6].routed
#set part_name "xczu3eg-sbva484-1-e"

# Set target part 
#set_part $part_name

# Read the netlist from third-party synthesis tool
read_edif $edif_file

# Read in the IP XCIs
#read_ip ip1.xci
#read_ip ip2.xci

# read in top level constraints
#read_xdc top.xdc

# Implement the design
link_design -top $top_name

# at this point we would have done opt_design and placement
#phys_opt_design 
place_design -directive Default
write_edif -force $edif_file_placed
route_design -directive Default
write_edif -force $edif_file_routed
report_timing -file $timing_file
report_utilization -file $utilization_file
#write_bitstream -file $top_name.bit