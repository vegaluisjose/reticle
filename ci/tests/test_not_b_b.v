module test_not_b_b(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg a;
    wire y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            a <= 1'b0;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    a <= 1'b1;
                    if (y != 1'b1) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    if (y != 1'b0) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    not_b_b dut(.clock(clock), .reset(reset), .a(a), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
