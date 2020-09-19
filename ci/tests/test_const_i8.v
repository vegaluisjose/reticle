module test_const_i8(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    wire [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    if (y != 8'd3) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    if (y != 8'd3) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    const_i8 dut(.clock(clock), .reset(reset), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
