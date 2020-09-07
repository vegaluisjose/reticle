module test_fsm();

    reg clock = 1'b0;
    reg reset = 1'b0;

    always #500 clock = ~clock;

    initial begin
        reset = 1'b1;
        repeat(16)@(negedge clock);
        reset = 1'b0;
    end

    reg [31:0] step;

    reg start;
    reg done;
    wire [7:0] state;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
            start <= 1'b0;
            done <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    start <= 1'b1;
                    done <= 1'b0;
                    if (state != 0) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                1: begin
                    step <= 2;
                    start <= 1'b0;
                    done <= 1'b1;
                    if (state != 0) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                2: begin
                    step <= 3;
                    start <= 1'b0;
                    done <= 1'b0;
                    if (state != 1) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                3: begin
                    if (state != 0) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    fsm dut(.clock(clock), .reset(reset), .start(start), .done(done), .state(state));

endmodule
