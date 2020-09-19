module test_all();

    reg clock = 1'b0;
    reg start = 1'b0;

    always #500 clock = ~clock;

    initial begin
        start = 1'b0;
        repeat(16)@(negedge clock);
        start = 1'b1;
    end

    wire reset;

    assign reset = ~start | glbl.GSR;

    wire [4:0] fail;
    wire [4:0] finish;

    test_add_i8_i8_i8 t0(.clock(clock), .reset(reset), .fail(fail[0]), .finish(finish[0]));
    test_sub_i8_i8_i8 t1(.clock(clock), .reset(reset), .fail(fail[1]), .finish(finish[1]));
    test_add_i8v4_i8v4_i8v4 t2(.clock(clock), .reset(reset), .fail(fail[2]), .finish(finish[2]));
    test_sub_i8v4_i8v4_i8v4 t3(.clock(clock), .reset(reset), .fail(fail[3]), .finish(finish[3]));
    test_add_mul_i8_i8_i8_i8 t4(.clock(clock), .reset(reset), .fail(fail[4]), .finish(finish[4]));

    always @(posedge clock) begin
        if (|fail) begin
            $finish;
        end
        if (&finish) begin
            $finish;
        end
    end

endmodule
