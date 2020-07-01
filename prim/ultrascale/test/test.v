module test_dsp_add_width_8 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 8;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = -8'd1;
    assign b = 8'd16;

    assign y_ref = a + b;

    dsp_add #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_add_width_8");
                else $error("[FAIL] test_dsp_add_width_8");
        end
    end

endmodule

module test_dsp_add_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd1;
    assign b = 32'hffff0001;

    assign y_ref = a + b;

    dsp_add #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_add_width_32");
                else $error("[FAIL] test_dsp_add_width_32");
        end
    end

endmodule

module test_dsp_add_v2_width_24 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 24;

    logic [width-1:0] a0;
    logic [width-1:0] b0;
    logic [width-1:0] a1;
    logic [width-1:0] b1;
    logic [width-1:0] y0;
    logic [width-1:0] y1;
    logic [width-1:0] y0_ref;
    logic [width-1:0] y1_ref;

    assign a0 = -24'd1;
    assign b0 = 24'd16;
    assign a1 = 24'd23;
    assign b1 = 24'd7;

    assign y0_ref = a0 + b0;
    assign y1_ref = a1 + b1;

    dsp_add_v2 #(.width(width)) dut (clock, reset, a0, b0, a1, b1, y0, y1);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y0 == y0_ref && y1 == y1_ref) $display ("[PASS] test_dsp_add_v2_width_24");
                else $error("[FAIL] test_dsp_add_v2_width_24");
        end
    end

endmodule

module test_dsp_add_v4_width_12 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 12;

    logic [width-1:0] a0;
    logic [width-1:0] b0;
    logic [width-1:0] a1;
    logic [width-1:0] b1;
    logic [width-1:0] a2;
    logic [width-1:0] b2;
    logic [width-1:0] a3;
    logic [width-1:0] b3;
    logic [width-1:0] y0;
    logic [width-1:0] y1;
    logic [width-1:0] y2;
    logic [width-1:0] y3;
    logic [width-1:0] y0_ref;
    logic [width-1:0] y1_ref;
    logic [width-1:0] y2_ref;
    logic [width-1:0] y3_ref;

    assign a0 = -12'd1;
    assign b0 = 12'd16;
    assign a1 = 12'd23;
    assign b1 = 12'd7;
    assign a2 = 12'd255;
    assign b2 = 12'd7;
    assign a3 = -12'd20;
    assign b3 = -12'd7;

    assign y0_ref = a0 + b0;
    assign y1_ref = a1 + b1;
    assign y2_ref = a2 + b2;
    assign y3_ref = a3 + b3;

    dsp_add_v4 #(.width(width)) dut (clock, reset, a0, b0, a1, b1, a2, b2, a3, b3, y0, y1, y2, y3);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y0 == y0_ref && y1 == y1_ref && y2 == y2_ref && y3 == y3_ref) $display ("[PASS] test_dsp_add_v4_width_12");
                else $error("[FAIL] test_dsp_add_v4_width_12");
        end
    end

endmodule

module test_dsp_add_v3_width_12 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 12;

    logic [width-1:0] a0;
    logic [width-1:0] b0;
    logic [width-1:0] a1;
    logic [width-1:0] b1;
    logic [width-1:0] a2;
    logic [width-1:0] b2;
    logic [width-1:0] y0;
    logic [width-1:0] y1;
    logic [width-1:0] y2;
    logic [width-1:0] y0_ref;
    logic [width-1:0] y1_ref;
    logic [width-1:0] y2_ref;

    assign a0 = 12'd1;
    assign b0 = -12'd16;
    assign a1 = -12'd23;
    assign b1 = -12'd7;
    assign a2 = 12'd25;
    assign b2 = 12'd7;

    assign y0_ref = a0 + b0;
    assign y1_ref = a1 + b1;
    assign y2_ref = a2 + b2;

    dsp_add_v3 #(.width(width)) dut (clock, reset, a0, b0, a1, b1, a2, b2, y0, y1, y2);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y0 == y0_ref && y1 == y1_ref && y2 == y2_ref) $display ("[PASS] test_dsp_add_v3_width_12");
                else $error("[FAIL] test_dsp_add_v3_width_12 ");
        end
    end

endmodule

module test_dsp_mul_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = -32'd255;
    assign b = 32'd3;

    assign y_ref = a * b;

    dsp_mul #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_mul_width_32");
                else $error("[FAIL] test_dsp_mul_width_32");
        end
    end

endmodule

module test_dsp_sub_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd10;
    assign b = 32'd1;

    assign y_ref = a - b;

    dsp_sub #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_sub_width_32");
                else $error("[FAIL] test_dsp_sub_width_32");
        end
    end

endmodule

module test_dsp_sub_v2_width_24 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 24;

    logic [width-1:0] a0;
    logic [width-1:0] b0;
    logic [width-1:0] a1;
    logic [width-1:0] b1;
    logic [width-1:0] y0;
    logic [width-1:0] y1;
    logic [width-1:0] y0_ref;
    logic [width-1:0] y1_ref;

    assign a0 = -24'd1;
    assign b0 = 24'd16;
    assign a1 = 24'd23;
    assign b1 = 24'd7;

    assign y0_ref = a0 - b0;
    assign y1_ref = a1 - b1;

    dsp_sub_v2 #(.width(width)) dut (clock, reset, a0, b0, a1, b1, y0, y1);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y0 == y0_ref && y1 == y1_ref) $display ("[PASS] test_dsp_sub_v2_width_24");
                else $error("[FAIL] test_dsp_sub_v2_width_24");
        end
    end

endmodule

module test_dsp_sub_v4_width_12 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 12;

    logic [width-1:0] a0;
    logic [width-1:0] b0;
    logic [width-1:0] a1;
    logic [width-1:0] b1;
    logic [width-1:0] a2;
    logic [width-1:0] b2;
    logic [width-1:0] a3;
    logic [width-1:0] b3;
    logic [width-1:0] y0;
    logic [width-1:0] y1;
    logic [width-1:0] y2;
    logic [width-1:0] y3;
    logic [width-1:0] y0_ref;
    logic [width-1:0] y1_ref;
    logic [width-1:0] y2_ref;
    logic [width-1:0] y3_ref;

    assign a0 = -12'd1;
    assign b0 = 12'd16;
    assign a1 = 12'd23;
    assign b1 = 12'd7;
    assign a2 = 12'd255;
    assign b2 = 12'd7;
    assign a3 = -12'd20;
    assign b3 = -12'd7;

    assign y0_ref = a0 - b0;
    assign y1_ref = a1 - b1;
    assign y2_ref = a2 - b2;
    assign y3_ref = a3 - b3;

    dsp_sub_v4 #(.width(width)) dut (clock, reset, a0, b0, a1, b1, a2, b2, a3, b3, y0, y1, y2, y3);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y0 == y0_ref && y1 == y1_ref && y2 == y2_ref && y3 == y3_ref) $display ("[PASS] test_dsp_sub_v4_width_12");
                else $error("[FAIL] test_dsp_sub_v4_width_12");
        end
    end

endmodule

module test_dsp_sub_v3_width_12 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 12;

    logic [width-1:0] a0;
    logic [width-1:0] b0;
    logic [width-1:0] a1;
    logic [width-1:0] b1;
    logic [width-1:0] a2;
    logic [width-1:0] b2;
    logic [width-1:0] y0;
    logic [width-1:0] y1;
    logic [width-1:0] y2;
    logic [width-1:0] y0_ref;
    logic [width-1:0] y1_ref;
    logic [width-1:0] y2_ref;

    assign a0 = 12'd1;
    assign b0 = 12'd16;
    assign a1 = 12'd23;
    assign b1 = 12'd7;
    assign a2 = 12'd25;
    assign b2 = 12'd7;

    assign y0_ref = a0 - b0;
    assign y1_ref = a1 - b1;
    assign y2_ref = a2 - b2;

    dsp_sub_v3 #(.width(width)) dut (clock, reset, a0, b0, a1, b1, a2, b2, y0, y1, y2);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y0 == y0_ref && y1 == y1_ref && y2 == y2_ref) $display ("[PASS] test_dsp_sub_v3_width_12");
                else $error("[FAIL] test_dsp_sub_v3_width_12 ");
        end
    end

endmodule

module test_dsp_and_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd10;
    assign b = 32'd1;

    assign y_ref = a & b;

    dsp_and #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_and_width_32");
                else $error("[FAIL] test_dsp_and_width_32");
        end
    end

endmodule

module test_dsp_or_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd10;
    assign b = 32'd1;

    assign y_ref = a | b;

    dsp_or #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_or_width_32");
                else $error("[FAIL] test_dsp_or_width_32");
        end
    end

endmodule

module test_dsp_xor_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd10;
    assign b = 32'd1;

    assign y_ref = a ^ b;

    dsp_xor #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_xor_width_32");
                else $error("[FAIL] test_dsp_xor_width_32");
        end
    end

endmodule

module test_dsp_nor_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd10;
    assign b = 32'd1;

    assign y_ref = ~(a | b);

    dsp_nor #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_nor_width_32");
                else $error("[FAIL] test_dsp_nor_width_32");
        end
    end

endmodule

module test_dsp_xnor_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd10;
    assign b = 32'd1;

    assign y_ref = ~(a ^ b);

    dsp_xnor #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_xnor_width_32");
                else $error("[FAIL] test_dsp_xnor_width_32");
        end
    end

endmodule

module test_dsp_nand_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd10;
    assign b = 32'd1;

    assign y_ref = ~(a & b);

    dsp_nand #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_nand_width_32");
                else $error("[FAIL] test_dsp_nand_width_32");
        end
    end

endmodule

module test_dsp_not_width_32 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 32;

    logic [width-1:0] a;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 32'd14893;

    assign y_ref = ~a;

    dsp_not #(.width(width)) dut (clock, reset, a, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_dsp_not_width_32");
                else $error("[FAIL] test_dsp_not_width_32");
        end
    end

endmodule

module test_lut_add_width_8 (
    input        clock,
    input        reset,
    input [31:0] cycles
);
    localparam width = 8;

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] y;
    logic [width-1:0] y_ref;

    assign a = 8'd14;
    assign b = -8'd8;

    assign y_ref = a + b;

    lut_add_w8 #(.width(width)) dut (clock, reset, a, b, y);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref) $display ("[PASS] test_lut_add_width_8");
                else $error("[FAIL] test_lut_add_width_8");
        end
    end

endmodule

module test();
    logic clock = 1'b0;
    logic reset;
    logic [31:0] cycles;

    always #10 clock = ~clock;

    // reset for 1 cycles
    initial begin
        reset = 1'b1;
        repeat(1)@(negedge clock);
        reset = 1'b0;
    end

    // cycle counter
    always @(posedge clock) begin
        if (reset) begin
            cycles <= 32'd0;
        end
        else begin
            cycles <= cycles + 1'b1;
        end
    end

    // run for 5 cycles
    always @(posedge clock) begin
        if (cycles == 32'd5) begin
            $finish;
        end
    end

    test_dsp_add_width_8     t0  (clock, reset, cycles);
    test_dsp_add_width_32    t1  (clock, reset, cycles);
    test_dsp_add_v2_width_24 t2  (clock, reset, cycles);
    test_dsp_add_v4_width_12 t3  (clock, reset, cycles);
    test_dsp_add_v3_width_12 t4  (clock, reset, cycles);
    test_dsp_mul_width_32    t5  (clock, reset, cycles);
    test_dsp_sub_width_32    t6  (clock, reset, cycles);
    test_dsp_sub_v2_width_24 t7  (clock, reset, cycles);
    test_dsp_sub_v4_width_12 t8  (clock, reset, cycles);
    test_dsp_sub_v3_width_12 t9  (clock, reset, cycles);
    test_dsp_and_width_32    t10 (clock, reset, cycles);
    test_dsp_or_width_32     t11 (clock, reset, cycles);
    test_dsp_xor_width_32    t12 (clock, reset, cycles);
    test_dsp_nor_width_32    t13 (clock, reset, cycles);
    test_dsp_xnor_width_32   t14 (clock, reset, cycles);
    test_dsp_nand_width_32   t15 (clock, reset, cycles);
    test_dsp_not_width_32    t16 (clock, reset, cycles);
    test_lut_add_width_8     t17 (clock, reset, cycles);

endmodule
