module test_vadd_const();

    reg clock = 1'b0;
    reg reset = 1'b0;

    always #500 clock = ~clock;

    initial begin
        reset = 1'b1;
        repeat(16)@(negedge clock);
        reset = 1'b0;
    end

    reg [31:0] step;

    reg [7:0] a_0, a_1, a_2, a_3;
    wire [7:0] y_0, y_1, y_2, y_3;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
            a_0 <= -8'd4;
            a_1 <= 8'd2;
            a_2 <= 8'd0;
            a_3 <= 8'd1;
        end
        else begin
            case (step)
                0: begin
                    if (y_0 != -8'd2 | y_1 != -8'd2 | y_2 != 8'd5 | y_3 != -8'd2) begin
                        $error("Error");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    vadd_const dut(.clock(clock), .reset(reset), .a_0(a_0), .a_1(a_1), .a_2(a_2), .a_3(a_3), .y_0(y_0), .y_1(y_1), .y_2(y_2), .y_3(y_3));

endmodule
