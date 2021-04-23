module main (
    input wire clock,
    input wire reset,
    input wire [7:0] a,
    input wire [7:0] b,
    input wire en,
    output reg [7:0] y
);
    wire [7:0] t0;
    assign t0 = a + b;
    always @(posedge clock) begin
        if(reset) begin
            y <= 0;
        end else if(en) begin
            y <= t0;
        end
    end
endmodule
