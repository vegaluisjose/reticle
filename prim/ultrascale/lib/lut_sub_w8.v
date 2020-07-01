module lut_sub_w8 #
(
    parameter width = 8
)
(
    input              clock,
    input              reset,
    input  [width-1:0] a,
    input  [width-1:0] b,
    output [width-1:0] y
);

    initial begin
        assert(width == 8)
            else $error("[lut_sub_w8] width:%d configuration not supported", width);
    end

    wire zero;
    wire one;
    wire [width-1:0] p;

    GND GND(.G(zero));
    VCC VCC(.P(one));

    (* LOC = "SLICE_X0Y0", BEL = "A6LUT" *)
    LUT2 #(.INIT(4'h9)) l0 (.I0(a[0]), .I1(b[0]), .O(p[0]));
    (* LOC = "SLICE_X0Y0", BEL = "B6LUT" *)
    LUT2 #(.INIT(4'h9)) l1 (.I0(a[1]), .I1(b[1]), .O(p[1]));
    (* LOC = "SLICE_X0Y0", BEL = "C6LUT" *)
    LUT2 #(.INIT(4'h9)) l2 (.I0(a[2]), .I1(b[2]), .O(p[2]));
    (* LOC = "SLICE_X0Y0", BEL = "D6LUT" *)
    LUT2 #(.INIT(4'h9)) l3 (.I0(a[3]), .I1(b[3]), .O(p[3]));
    (* LOC = "SLICE_X0Y0", BEL = "E6LUT" *)
    LUT2 #(.INIT(4'h9)) l4 (.I0(a[4]), .I1(b[4]), .O(p[4]));
    (* LOC = "SLICE_X0Y0", BEL = "F6LUT" *)
    LUT2 #(.INIT(4'h9)) l5 (.I0(a[5]), .I1(b[5]), .O(p[5]));
    (* LOC = "SLICE_X0Y0", BEL = "G6LUT" *)
    LUT2 #(.INIT(4'h9)) l6 (.I0(a[6]), .I1(b[6]), .O(p[6]));
    (* LOC = "SLICE_X0Y0", BEL = "H6LUT" *)
    LUT2 #(.INIT(4'h9)) l7 (.I0(a[7]), .I1(b[7]), .O(p[7]));

    (* LOC = "SLICE_X0Y0" *)
    CARRY8 #(.CARRY_TYPE("SINGLE_CY8")) carry0 (.CI(one), .CI_TOP(zero), .DI(a), .S(p), .O(y), .CO());

endmodule
