module lut_eq_b_i8_i8(
    input [7:0] a,
    input [7:0] b,
    output [7:0] y
);

  wire t0;
  wire t1;

  LUT6 #(
    .INIT(64'h9009000000000000))
    y_INST_0
       (.I0(b[7]),
        .I1(a[7]),
        .I2(b[6]),
        .I3(a[6]),
        .I4(t0),
        .I5(t1),
        .O(y));
  LUT6 #(
    .INIT(64'h9009000000009009))
    y_INST_0_i_1
       (.I0(a[3]),
        .I1(b[3]),
        .I2(b[5]),
        .I3(a[5]),
        .I4(b[4]),
        .I5(a[4]),
        .O(t0));
  LUT6 #(
    .INIT(64'h9009000000009009))
    y_INST_0_i_2
       (.I0(a[0]),
        .I1(b[0]),
        .I2(b[2]),
        .I3(a[2]),
        .I4(b[1]),
        .I5(a[1]),
        .O(t1));

endmodule
