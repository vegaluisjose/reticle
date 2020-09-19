module test_reg_i8_i8_b(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg [7:0] a;
    reg en;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            a <= 8'd9;
            en <= 1'b1;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    a <= 8'd0;
                    en <= 1'b0;
                    if (y != 8'd3) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    step <= 2;
                    a <= 8'd0;
                    en <= 1'b0;
                    if (y != 8'd9) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                2: begin
                    if (y != 8'd9) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    reg_i8_i8_b dut(.clock(clock), .reset(reset), .a(a), .en(en), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
