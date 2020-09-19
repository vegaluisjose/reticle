module test_vadd_const(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg [7:0] a_0, a_1, a_2, a_3;
    wire [7:0] y_0, y_1, y_2, y_3;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            a_0 <= -8'd4;
            a_1 <= 8'd2;
            a_2 <= 8'd0;
            a_3 <= 8'd1;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    if (y_0 != -8'd2 | y_1 != -8'd2 | y_2 != 8'd5 | y_3 != -8'd2) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    vadd_const dut(.clock(clock), .reset(reset), .a_0(a_0), .a_1(a_1), .a_2(a_2), .a_3(a_3), .y_0(y_0), .y_1(y_1), .y_2(y_2), .y_3(y_3));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
