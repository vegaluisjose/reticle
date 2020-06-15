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

    logic [width-1:0] a;
    logic [width-1:0] b;
    logic [width-1:0] c;
    logic [width-1:0] d;
    logic [width-1:0] y;
    logic [width-1:0] z;
    logic [width-1:0] y_ref;
    logic [width-1:0] z_ref;

    assign a = -24'd1;
    assign b = 24'd16;
    assign c = 24'd23;
    assign d = 24'd7;
    assign y_ref = a + b;
    assign z_ref = c + d;

    dsp_add_v2 #(.width(width)) dut (clock, reset, a, b, c, d, y, z);

    always @(posedge clock) begin
        if (!reset && (cycles == 32'd0)) begin
            assert (y == y_ref && z == z_ref) $display ("[test_dsp_add_v2_width_24] PASS");
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
