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

    wire [9:0] fail;
    wire [9:0] finish;

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
    test_and_b_b_b t14(.clock(clock), .reset(reset), .fail(fail[14]), .finish(finish[14]));
    test_const_i8 t15(.clock(clock), .reset(reset), .fail(fail[15]), .finish(finish[15]));
    test_dsp_add_i8_i8_i8 t16(.clock(clock), .reset(reset), .fail(fail[16]), .finish(finish[16]));
    test_dsp_and_i8_i8_i8 t17(.clock(clock), .reset(reset), .fail(fail[17]), .finish(finish[17]));
    test_dsp_or_i8_i8_i8 t18(.clock(clock), .reset(reset), .fail(fail[18]), .finish(finish[18]));
    test_dsp_sub_i8_i8_i8 t19(.clock(clock), .reset(reset), .fail(fail[19]), .finish(finish[19]));
    test_eq_b_b_b t20(.clock(clock), .reset(reset), .fail(fail[20]), .finish(finish[20]));
    test_eq_b_i8_i8 t21(.clock(clock), .reset(reset), .fail(fail[21]), .finish(finish[21]));
    test_fsm t22(.clock(clock), .reset(reset), .fail(fail[22]), .finish(finish[22]));
    test_id_b_b t23(.clock(clock), .reset(reset), .fail(fail[23]), .finish(finish[23]));
    test_lut_add_i8_i8_i8 t24(.clock(clock), .reset(reset), .fail(fail[24]), .finish(finish[24]));
    test_lut_and_i8_i8_i8 t25(.clock(clock), .reset(reset), .fail(fail[25]), .finish(finish[25]));
    test_lut_or_i8_i8_i8 t26(.clock(clock), .reset(reset), .fail(fail[26]), .finish(finish[26]));
    test_lut_sub_i8_i8_i8 t27(.clock(clock), .reset(reset), .fail(fail[27]), .finish(finish[27]));
    test_mux_b_b_b_b t28(.clock(clock), .reset(reset), .fail(fail[28]), .finish(finish[28]));
    test_mux_i8_b_i8_i8 t29(.clock(clock), .reset(reset), .fail(fail[29]), .finish(finish[29]));
    test_nand_b_b_b t30(.clock(clock), .reset(reset), .fail(fail[30]), .finish(finish[30]));
    test_nand_i8_i8_i8 t31(.clock(clock), .reset(reset), .fail(fail[31]), .finish(finish[31]));
    test_neq_b_b_b t32(.clock(clock), .reset(reset), .fail(fail[32]), .finish(finish[32]));
    test_neq_b_i8_i8 t33(.clock(clock), .reset(reset), .fail(fail[33]), .finish(finish[33]));
    test_nor_b_b_b t34(.clock(clock), .reset(reset), .fail(fail[34]), .finish(finish[34]));
    test_not_b_b t35(.clock(clock), .reset(reset), .fail(fail[35]), .finish(finish[35]));
    test_or_b_b_b t36(.clock(clock), .reset(reset), .fail(fail[36]), .finish(finish[36]));
    test_pipeline t37(.clock(clock), .reset(reset), .fail(fail[37]), .finish(finish[37]));
    test_reg_i8_i8_b t38(.clock(clock), .reset(reset), .fail(fail[38]), .finish(finish[38]));
    test_vadd_const t39(.clock(clock), .reset(reset), .fail(fail[39]), .finish(finish[39]));
    test_xnor_b_b_b t40(.clock(clock), .reset(reset), .fail(fail[40]), .finish(finish[40]));
    test_xor_b_b_b t41(.clock(clock), .reset(reset), .fail(fail[41]), .finish(finish[41]));
    test_dot t42(.clock(clock), .reset(reset), .fail(fail[42]), .finish(finish[42]));

    always @(posedge clock) begin
        if (|fail) begin
            $finish;
        end
        if (&finish) begin
            $finish;
        end
    end

endmodule
