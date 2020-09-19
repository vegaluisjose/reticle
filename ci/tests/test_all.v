module test_all();

    reg clock = 1'b0;
    reg start = 1'b0;

    always #500 clock = ~clock;

    initial begin
        start = 1'b0;
        repeat(16)@(negedge clock);
        start = 1'b1;
    end

    wire reset;

    assign reset = ~start | glbl.GSR;

    wire t0_fail;
    wire t0_finish;

    test_add_i8_i8_i8 t0(.clock(clock), .reset(reset), .fail(t0_fail), .finish(t0_finish));

    always @(posedge clock) begin
        if (t0_fail) begin
            $finish;
        end
        if (t0_finish) begin
            $finish;
        end
    end

endmodule
