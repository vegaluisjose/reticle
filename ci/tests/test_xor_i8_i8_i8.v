module test_xor_i8_i8_i8(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg [7:0] a, b;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            a <= 8'd3;
            b <= 8'd12;
        end
        else begin
            case (step)
                0: begin
                    if (y != 8'd15) begin
                        $display("[test_xor_i8_i8_i8] ~~FAIL~~ res:%d exp:15", $signed(y));
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    xor_i8_i8_i8 dut(.clock(clock), .reset(reset), .a(a), .b(b), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
