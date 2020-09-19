module test_mux_i8_b_i8_i8(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg [7:0] t, f;
    reg cond;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            cond <= 1'b0;
            t <= 8'd2;
            f <= 8'd6;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    cond <= 1'b1;
                    t <= 8'd2;
                    f <= 8'd6;
                    if (y != 8'd6) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    if (y != 8'd2) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    mux_i8_b_i8_i8 dut(.clock(clock), .reset(reset), .cond(cond), .t(t), .f(f), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
