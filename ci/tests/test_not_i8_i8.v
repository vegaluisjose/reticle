module test_not_i8_i8(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg [7:0] a;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            a <= 8'd3;
        end
        else begin
            case (step)
                0: begin
                    if (y != -8'd4) begin
                        $display("[test_not_i8_i8] ~~FAIL~~ res:%d exp:-4", $signed(y));
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    not_i8_i8 dut(.clock(clock), .reset(reset), .a(a), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
