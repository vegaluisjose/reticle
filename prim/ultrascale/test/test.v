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
            assert (y == y_ref) $display ("[test_dsp_add_width_8] PASS");
                else $error("[test_dsp_add_width_8] FAIL");
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
            assert (y == y_ref) $display ("[test_dsp_add_width_32] PASS");
                else $error("[test_dsp_add_width_32] FAIL");
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
            assert (y0 == y0_ref && y1 == y1_ref) $display ("[test_dsp_add_v2_width_24] PASS");
                else $error("[test_dsp_add_v2_width_24] FAIL");
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

    test_dsp_add_width_8 t0 (clock, reset, cycles);
    test_dsp_add_width_32 t1 (clock, reset, cycles);
    test_dsp_add_v2_width_24 t2 (clock, reset, cycles);

endmodule
