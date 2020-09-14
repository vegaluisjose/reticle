module test_mux_b_b_b_b();

    reg clock = 1'b0;
    reg reset = 1'b0;

    always #500 clock = ~clock;

    initial begin
        reset = 1'b1;
        repeat(16)@(negedge clock);
        reset = 1'b0;
    end

    reg [31:0] step;

    reg cond, t, f;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
            cond <= 1'b0;
            t <= 1'b1;
            f <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    cond <= 1'b1;
                    t <= 1'b1;
                    f <= 1'b0;
                    if (y != 1'b0) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                1: begin
                    if (y != 1'b1) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    mux_b_b_b_b dut(.clock(clock), .reset(reset), .cond(cond), .t(t), .f(f), .y(y));

endmodule
