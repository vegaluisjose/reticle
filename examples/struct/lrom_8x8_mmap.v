module main (
    input wire clock,
    input wire reset,
    input wire [2:0] addr,
    output wire [7:0] y
);
    wire gnd;
    wire vcc;
    GND _gnd (
        .G(gnd)
    );
    VCC _vcc (
        .P(vcc)
    );
    (*LOC = "SLICE_X0Y0", BEL = "H6LUT"*)
    RAM64M8 # (
        .INIT_A(64'h0001010001000100),
        .INIT_B(64'h0100010100000101),
        .INIT_C(64'h0000010101010000),
        .INIT_D(64'h0000010101010101),
        .INIT_E(64'h0001010001000100),
        .INIT_F(64'h0100010100000101),
        .INIT_G(64'h0000010101010000),
        .INIT_H(64'h0000010101010101),
        .IS_WCLK_INVERTED(1'b0)
    ) __y (
        .ADDRA({addr[2], addr[1], addr[0], gnd, gnd, gnd}),
        .ADDRB({addr[2], addr[1], addr[0], gnd, gnd, gnd}),
        .ADDRC({addr[2], addr[1], addr[0], gnd, gnd, gnd}),
        .ADDRD({addr[2], addr[1], addr[0], gnd, gnd, gnd}),
        .ADDRE({addr[2], addr[1], addr[0], gnd, gnd, gnd}),
        .ADDRF({addr[2], addr[1], addr[0], gnd, gnd, gnd}),
        .ADDRG({addr[2], addr[1], addr[0], gnd, gnd, gnd}),
        .ADDRH({addr[2], addr[1], addr[0], gnd, gnd, gnd}),
        .DIA(gnd),
        .DIB(gnd),
        .DIC(gnd),
        .DID(gnd),
        .DIE(gnd),
        .DIF(gnd),
        .DIG(gnd),
        .DIH(gnd),
        .DOA(y[0]),
        .DOB(y[1]),
        .DOC(y[2]),
        .DOD(y[3]),
        .DOE(y[4]),
        .DOF(y[5]),
        .DOG(y[6]),
        .DOH(y[7]),
        .WCLK(clock),
        .WE(gnd)
    );
endmodule
