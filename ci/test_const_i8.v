module test_const_i8();

    reg clock = 1'b0;
    reg reset = 1'b0;

    always #500 clock = ~clock;

    initial begin
        reset = 1'b1;
        repeat(16)@(negedge clock);
        reset = 1'b0;
    end

    reg [31:0] step;

    wire [7:0] y;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    if (y != 8'd3) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                1: begin
                    if (y != 8'd3) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    const_i8 dut(.clock(clock), .reset(reset), .y(y));

endmodule
