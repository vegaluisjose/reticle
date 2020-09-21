module test_dot(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg en;
    reg [7:0] bias;
    reg [7:0] a;
    reg [7:0] b;
    reg [7:0] c;
    reg [7:0] d;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            en <= 1'b1;
            bias <= 8'd0;
            a <= -8'd2;
            b <= 8'd3;
            c <= 8'd0;
            d <= 8'd0;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    en <= 1'b1;
                    bias <= 8'd0;
                    a <= 8'd1;
                    b <= 8'd2;
                    c <= 8'd7;
                    d <= 8'd2;
                    if (y != 8'd0) begin
                        $display("[test_dot] ~~FAIL~~ res:%d exp:0", $signed(y));
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    step <= 2;
                    en <= 1'b1;
                    bias <= 8'd3;
                    a <= 8'd0;
                    b <= 8'd0;
                    c <= -8'd3;
                    d <= 8'd4;
                    if (y != 8'd0) begin
                        $display("[test_dot] ~~FAIL~~ res:%d exp:0", $signed(y));
                        t_fail <= 1'b1;
                    end
                end
                2: begin
                    step <= 3;
                    en <= 1'b1;
                    bias <= 8'd9;
                    a <= 8'd0;
                    b <= 8'd0;
                    c <= 8'd0;
                    d <= 8'd0;
                    if (y != 8'd0) begin
                        $display("[test_dot] ~~FAIL~~ res:%d exp:0", $signed(y));
                        t_fail <= 1'b1;
                    end
                end
                3: begin
                    step <= 4;
                    en <= 1'b1;
                    bias <= 8'd0;
                    a <= 8'd0;
                    b <= 8'd0;
                    c <= 8'd0;
                    d <= 8'd0;
                    if (y != 8'd0) begin
                        $display("[test_dot] ~~FAIL~~ res:%d exp:0", $signed(y));
                        t_fail <= 1'b1;
                    end
                end
                4: begin
                    step <= 5;
                    en <= 1'b0;
                    bias <= 8'd0;
                    a <= 8'd0;
                    b <= 8'd0;
                    c <= 8'd0;
                    d <= 8'd0;
                    if (y != 8'd11) begin
                        $display("[test_dot] ~~FAIL~~ res:%d exp:11", $signed(y));
                        t_fail <= 1'b1;
                    end
                end
                5: begin
                    if (y != -8'd1) begin
                        $display("[test_dot] ~~FAIL~~ res:%d exp:-1", $signed(y));
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    dot dut(.clock(clock), .reset(reset), .en(en), .bias(bias), .a(a), .b(b), .c(c), .d(d), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
