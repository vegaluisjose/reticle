// this module tests whether we can reuse the output
// of a dsp into other dsps i.e.,
// def foo(a: i8, b: i8, c: i8, d: i8) -> (y: i8)
//     t0: i8 = mul(b, c);
//     t1: i8 = add(t0, a);
//     t2: i8 = add(t0, d);
//     t3: i8 = add(t1, t2);
//     y: i8 = id(t3);
// }
module dsp_add_add_add_mul (
    input clock,
    input reset,
    input [7:0] a,
    input [7:0] b,
    input [7:0] c,
    input [7:0] d,
    output [7:0] y
);

    wire [7:0] t0, t1, t2, t3;
    dsp_mul #(.width(8)) i0 (clock, reset, b, c, t0);
    dsp_add #(.width(8)) i1 (clock, reset, t0, a, t1);
    dsp_add #(.width(8)) i2 (clock, reset, t0, d, t2);
    dsp_add #(.width(8)) i3 (clock, reset, t1, t2, t3);

    assign y = t3;

endmodule
