module test_fsm(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg start;
    reg done;
    wire [7:0] state;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            start <= 1'b0;
            done <= 1'b0;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    start <= 1'b1;
                    done <= 1'b0;
                    if (state != 8'd0) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    step <= 2;
                    start <= 1'b0;
                    done <= 1'b1;
                    if (state != 8'd0) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                2: begin
                    step <= 3;
                    start <= 1'b0;
                    done <= 1'b0;
                    if (state != 8'd1) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                end
                3: begin
                    if (state != 8'd0) begin
                        $display("~~FAIL~~");
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    fsm dut(.clock(clock), .reset(reset), .start(start), .done(done), .state(state));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
