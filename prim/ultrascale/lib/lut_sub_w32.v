module lut_sub_w32 #
(
    parameter width = 16
)
(
    input              clock,
    input              reset,
    input  [width-1:0] a,
    input  [width-1:0] b,
    output [width-1:0] y
);

    initial begin
        assert(width == 32)
            else $error("[lut_sub_w32] width:%d configuration not supported", width);
    end

    wire zero;
    wire one;
    wire [width-1:0] p;
    wire [width-1:0] co;

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
    CARRY8 #(.CARRY_TYPE("SINGLE_CY8")) carry0 (.CI(one), .CI_TOP(zero), .DI(a[7:0]), .S(p[7:0]), .O(y[7:0]), .CO(co[7:0]));

    (* LOC = "SLICE_X0Y1", BEL = "A6LUT" *)
    LUT2 #(.INIT(4'h9)) l8 (.I0(a[8]), .I1(b[8]), .O(p[8]));
    (* LOC = "SLICE_X0Y1", BEL = "B6LUT" *)
    LUT2 #(.INIT(4'h9)) l9 (.I0(a[9]), .I1(b[9]), .O(p[9]));
    (* LOC = "SLICE_X0Y1", BEL = "C6LUT" *)
    LUT2 #(.INIT(4'h9)) l10 (.I0(a[10]), .I1(b[10]), .O(p[10]));
    (* LOC = "SLICE_X0Y1", BEL = "D6LUT" *)
    LUT2 #(.INIT(4'h9)) l11 (.I0(a[11]), .I1(b[11]), .O(p[11]));
    (* LOC = "SLICE_X0Y1", BEL = "E6LUT" *)
    LUT2 #(.INIT(4'h9)) l12 (.I0(a[12]), .I1(b[12]), .O(p[12]));
    (* LOC = "SLICE_X0Y1", BEL = "F6LUT" *)
    LUT2 #(.INIT(4'h9)) l13 (.I0(a[13]), .I1(b[13]), .O(p[13]));
    (* LOC = "SLICE_X0Y1", BEL = "G6LUT" *)
    LUT2 #(.INIT(4'h9)) l14 (.I0(a[14]), .I1(b[14]), .O(p[14]));
    (* LOC = "SLICE_X0Y1", BEL = "H6LUT" *)
    LUT2 #(.INIT(4'h9)) l15 (.I0(a[15]), .I1(b[15]), .O(p[15]));

    (* LOC = "SLICE_X0Y1" *)
    CARRY8 #(.CARRY_TYPE("SINGLE_CY8")) carry1 (.CI(co[7]), .CI_TOP(zero), .DI(a[15:8]), .S(p[15:8]), .O(y[15:8]), .CO(co[15:8]));
    
    (* LOC = "SLICE_X0Y2", BEL = "A6LUT" *)
    LUT2 #(.INIT(4'h9)) l16 (.I0(a[16]), .I1(b[16]), .O(p[16]));
    (* LOC = "SLICE_X0Y2", BEL = "B6LUT" *)
    LUT2 #(.INIT(4'h9)) l17 (.I0(a[17]), .I1(b[17]), .O(p[17]));
    (* LOC = "SLICE_X0Y2", BEL = "C6LUT" *)
    LUT2 #(.INIT(4'h9)) l18 (.I0(a[18]), .I1(b[18]), .O(p[18]));
    (* LOC = "SLICE_X0Y2", BEL = "D6LUT" *)
    LUT2 #(.INIT(4'h9)) l19 (.I0(a[19]), .I1(b[19]), .O(p[19]));
    (* LOC = "SLICE_X0Y2", BEL = "E6LUT" *)
    LUT2 #(.INIT(4'h9)) l20 (.I0(a[20]), .I1(b[20]), .O(p[20]));
    (* LOC = "SLICE_X0Y2", BEL = "F6LUT" *)
    LUT2 #(.INIT(4'h9)) l21 (.I0(a[21]), .I1(b[21]), .O(p[21]));
    (* LOC = "SLICE_X0Y2", BEL = "G6LUT" *)
    LUT2 #(.INIT(4'h9)) l22 (.I0(a[22]), .I1(b[22]), .O(p[22]));
    (* LOC = "SLICE_X0Y2", BEL = "H6LUT" *)
    LUT2 #(.INIT(4'h9)) l23 (.I0(a[23]), .I1(b[23]), .O(p[23]));

    (* LOC = "SLICE_X0Y2" *)
    CARRY8 #(.CARRY_TYPE("SINGLE_CY8")) carry2 (.CI(co[15]), .CI_TOP(zero), .DI(a[23:16]), .S(p[23:16]), .O(y[23:16]), .CO(co[23:16]));

    (* LOC = "SLICE_X0Y3", BEL = "A6LUT" *)
    LUT2 #(.INIT(4'h9)) l24 (.I0(a[24]), .I1(b[24]), .O(p[24]));
    (* LOC = "SLICE_X0Y3", BEL = "B6LUT" *)
    LUT2 #(.INIT(4'h9)) l25 (.I0(a[25]), .I1(b[25]), .O(p[25]));
    (* LOC = "SLICE_X0Y3", BEL = "C6LUT" *)
    LUT2 #(.INIT(4'h9)) l26 (.I0(a[26]), .I1(b[26]), .O(p[26]));
    (* LOC = "SLICE_X0Y3", BEL = "D6LUT" *)
    LUT2 #(.INIT(4'h9)) l27 (.I0(a[27]), .I1(b[27]), .O(p[27]));
    (* LOC = "SLICE_X0Y3", BEL = "E6LUT" *)
    LUT2 #(.INIT(4'h9)) l28 (.I0(a[28]), .I1(b[28]), .O(p[28]));
    (* LOC = "SLICE_X0Y3", BEL = "F6LUT" *)
    LUT2 #(.INIT(4'h9)) l29 (.I0(a[29]), .I1(b[29]), .O(p[29]));
    (* LOC = "SLICE_X0Y3", BEL = "G6LUT" *)
    LUT2 #(.INIT(4'h9)) l30 (.I0(a[30]), .I1(b[30]), .O(p[30]));
    (* LOC = "SLICE_X0Y3", BEL = "H6LUT" *)
    LUT2 #(.INIT(4'h9)) l31 (.I0(a[31]), .I1(b[31]), .O(p[31]));

    (* LOC = "SLICE_X0Y3" *)
    CARRY8 #(.CARRY_TYPE("SINGLE_CY8")) carry3 (.CI(co[23]), .CI_TOP(zero), .DI(a[31:24]), .S(p[31:24]), .O(y[31:24]), .CO());

endmodule
