module test_muladd();

    reg clock = 1'b0;
    reg reset = 1'b0;

    always #500 clock = ~clock;

    initial begin
        reset = 1'b1;
        repeat(16)@(negedge clock);
        reset = 1'b0;
    end

    reg [31:0] step;

    reg [7:0] a;
    reg [7:0] b;
    reg [7:0] c;
    reg en;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
            a <= 8'd4;
            b <= 8'd2;
            c <= 8'd3;
            en <= 1'b1;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    a <= 8'd0;
                    b <= 8'd0;
                    c <= 8'd3;
                    en <= 1'b0;
                    if (y != 8'd3) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                1: begin
                    if (y != 8'd11) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    muladd dut(.clock(clock), .reset(reset), .a(a), .b(b), .c(c), .en(en), .y(y));

endmodule
