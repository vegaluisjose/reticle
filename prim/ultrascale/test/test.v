module test_dsp_add_8 (
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
            assert (y == y_ref) $display ("[test_dsp_add_8] PASS");
                else $error("[test_dsp_add_8] FAIL");
        end
    end

endmodule

module test_dsp_add_32 (
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
            assert (y == y_ref) $display ("[test_dsp_add_32] PASS");
                else $error("[test_dsp_add_32] FAIL");
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

    test_dsp_add_8 t0 (clock, reset, cycles);
    test_dsp_add_32 t1 (clock, reset, cycles);

endmodule
