module test_mux_b_b_b_b(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg cond, t, f;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            cond <= 1'b0;
            t <= 1'b1;
            f <= 1'b0;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
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
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    if (y != 1'b1) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    mux_b_b_b_b dut(.clock(clock), .reset(reset), .cond(cond), .t(t), .f(f), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
