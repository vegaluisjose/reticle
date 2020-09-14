module test_add_i8v4_i8v4_i8v4();

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
    reg [7:0] b_0, b_1, b_2, b_3;
    wire [7:0] y_0, y_1, y_2, y_3;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
            a_0 <= -8'd4;
            a_1 <= 8'd2;
            a_2 <= 8'd2;
            a_3 <= 8'd1;
            b_0 <= 8'd1;
            b_1 <= 8'd3;
            b_2 <= 8'd0;
            b_3 <= 8'd1;
        end
        else begin
            case (step)
                0: begin
                    if (y_0 != -8'd3 | y_1 != 8'd5 | y_2 != 8'd2 | y_3 != 8'd2) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    add_i8v4_i8v4_i8v4 dut(.clock(clock), .reset(reset), .a_0(a_0), .a_1(a_1), .a_2(a_2), .a_3(a_3), .b_0(b_0), .b_1(b_1), .b_2(b_2), .b_3(b_3), .y_0(y_0), .y_1(y_1), .y_2(y_2), .y_3(y_3));

endmodule
