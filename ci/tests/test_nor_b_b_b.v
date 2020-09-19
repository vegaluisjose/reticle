module test_nor_b_b_b(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg a, b;
    wire y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            a <= 1'b0;
            b <= 1'b0;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    a <= 1'b1;
                    b <= 1'b0;
                    if (y != 1'b1) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    step <= 2;
                    a <= 1'b0;
                    b <= 1'b1;
                    if (y != 1'b0) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                2: begin
                    step <= 3;
                    a <= 1'b1;
                    b <= 1'b1;
                    if (y != 1'b0) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                3: begin
                    if (y != 1'b0) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    nor_b_b_b dut(.clock(clock), .reset(reset), .a(a), .b(b), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
