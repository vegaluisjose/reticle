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

    wire [6:0] fail;
    wire [6:0] finish;

    test_add_i8_i8_i8 t0(.clock(clock), .reset(reset), .fail(fail[0]), .finish(finish[0]));
    test_sub_i8_i8_i8 t1(.clock(clock), .reset(reset), .fail(fail[1]), .finish(finish[1]));
    test_add_i8v4_i8v4_i8v4 t2(.clock(clock), .reset(reset), .fail(fail[2]), .finish(finish[2]));
    test_sub_i8v4_i8v4_i8v4 t3(.clock(clock), .reset(reset), .fail(fail[3]), .finish(finish[3]));
    test_add_mul_i8_i8_i8_i8 t4(.clock(clock), .reset(reset), .fail(fail[4]), .finish(finish[4]));
    test_add_reg_mul_i8_i8_i8_b_i8 t5(.clock(clock), .reset(reset), .fail(fail[5]), .finish(finish[5]));
    test_mul_i8_i8_i8 t6(.clock(clock), .reset(reset), .fail(fail[6]), .finish(finish[6]));
    test_and_i8_i8_i8 t7(.clock(clock), .reset(reset), .fail(fail[7]), .finish(finish[7]));
    test_id_i8_i8 t8(.clock(clock), .reset(reset), .fail(fail[8]), .finish(finish[8]));
    test_nor_i8_i8_i8 t9(.clock(clock), .reset(reset), .fail(fail[9]), .finish(finish[9]));
    test_not_i8_i8 t10(.clock(clock), .reset(reset), .fail(fail[10]), .finish(finish[10]));
    test_or_i8_i8_i8 t11(.clock(clock), .reset(reset), .fail(fail[11]), .finish(finish[11]));
    test_xnor_i8_i8_i8 t12(.clock(clock), .reset(reset), .fail(fail[12]), .finish(finish[12]));
    test_xor_i8_i8_i8 t13(.clock(clock), .reset(reset), .fail(fail[13]), .finish(finish[13]));

    always @(posedge clock) begin
        if (|fail) begin
            $finish;
        end
        if (&finish) begin
            $finish;
        end
    end

endmodule
