module test_dsp_add_8 (
    input clock,
    input reset
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
        if (~reset) begin
            assert (y == y_ref) $display ("[dsp_add] PASS");
                else $error("[dsp_add] FAIL");
        end
    end

endmodule

module test();
    logic clock = 1'b0;
    logic reset;

    always #10 clock = ~clock;

    // reset for 1 cycles
    initial begin
        reset = 1'b1;
        repeat(1)@(negedge clock);
        reset = 1'b0;
    end

    // run for 5 cycles
    initial begin
        repeat(5)@(negedge clock);
        $finish;
    end

    test_dsp_add_8 t0 (clock, reset);

endmodule
