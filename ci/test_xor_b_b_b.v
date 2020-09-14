module test_xor_b_b_b();

    reg clock = 1'b0;
    reg reset = 1'b0;

    always #500 clock = ~clock;

    initial begin
        reset = 1'b1;
        repeat(16)@(negedge clock);
        reset = 1'b0;
    end

    reg [31:0] step;

    reg a, b;
    wire y;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
            a <= 1'b0;
            b <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    a <= 1'b1;
                    b <= 1'b0;
                    if (y != 1'b0) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                1: begin
                    step <= 2;
                    a <= 1'b0;
                    b <= 1'b1;
                    if (y != 1'b1) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                2: begin
                    step <= 3;
                    a <= 1'b1;
                    b <= 1'b1;
                    if (y != 1'b1) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                3: begin
                    if (y != 1'b0) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    xor_b_b_b dut(.clock(clock), .reset(reset), .a(a), .b(b), .y(y));

endmodule
