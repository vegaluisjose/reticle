module dsp_not #
(
    parameter width = 48
)
(
    input              clock,
    input              reset,
    input  [width-1:0] a,
    output [width-1:0] y
);
    logic [3:0] dsp_alumode;
    logic [2:0] dsp_carryinsel;
    logic [4:0] dsp_inmode;
    logic [8:0] dsp_opmode;
    logic [29:0] dsp_a;
    logic [17:0] dsp_b;
    logic [47:0] dsp_c;
    logic [47:0] dsp_p;
    logic [47:0] dsp_tmp;
    logic ce;

    initial begin
        assert(width > 0 && width <= 48)
            else $error("[dsp_not] width:%d configuration not supported", width);
    end

    assign dsp_alumode = 4'b1101;
    assign dsp_inmode = 5'b00000;
    assign dsp_opmode = 9'b000110011;
    assign dsp_carryinsel = 3'd0;
    assign ce = 1'b0;

    localparam extend = 48 - width;

    assign dsp_b = {18{1'b1}};
    assign dsp_a = {30{1'b1}};
    assign dsp_c = {{extend{1'b0}}, a};

    assign y = dsp_p[width-1:0];

    // DSP48E2: 48-bit Multi-Functional Arithmetic Block
    //          Virtex UltraScale+
    // Xilinx HDL Language Template, version 2020.1

    (* LOC = "DSP48E2_X0Y8" *)
    DSP48E2 #(
        // Feature Control Attributes: Data Path Selection
        .AMULTSEL("A"),                    // Selects A input to multiplier (A, AD)
        .A_INPUT("DIRECT"),                // Selects A input source, "DIRECT" (A port) or "CASCADE" (ACIN port)
        .BMULTSEL("B"),                    // Selects B input to multiplier (AD, B)
        .B_INPUT("DIRECT"),                // Selects B input source, "DIRECT" (B port) or "CASCADE" (BCIN port)
        .PREADDINSEL("A"),                 // Selects input to pre-adder (A, B)
        .RND(48'h000000000000),            // Rounding Constant
        .USE_MULT("NONE"),                 // Select multiplier usage (DYNAMIC, MULTIPLY, NONE)
        .USE_SIMD("ONE48"),                // SIMD selection (FOUR12, ONE48, TWO24)
        .USE_WIDEXOR("FALSE"),             // Use the Wide XOR function (FALSE, TRUE)
        .XORSIMD("XOR24_48_96"),           // Mode of operation for the Wide XOR (XOR12, XOR24_48_96)
        // Pattern Detector Attributes: Pattern Detection Configuration
        .AUTORESET_PATDET("NO_RESET"),     // NO_RESET, RESET_MATCH, RESET_NOT_MATCH
        .AUTORESET_PRIORITY("RESET"),      // Priority of AUTORESET vs. CEP (CEP, RESET).
        .MASK(48'h3fffffffffff),           // 48-bit mask value for pattern detect (1=ignore)
        .PATTERN(48'h000000000000),        // 48-bit pattern match for pattern detect
        .SEL_MASK("MASK"),                 // C, MASK, ROUNDING_MODE1, ROUNDING_MODE2
        .SEL_PATTERN("PATTERN"),           // Select pattern value (C, PATTERN)
        .USE_PATTERN_DETECT("NO_PATDET"),  // Enable pattern detect (NO_PATDET, PATDET)
        // Programmable Inversion Attributes: Specifies built-in programmable inversion on specific pins
        .IS_ALUMODE_INVERTED(4'b0000),     // Optional inversion for ALUMODE
        .IS_CARRYIN_INVERTED(1'b0),        // Optional inversion for CARRYIN
        .IS_CLK_INVERTED(1'b0),            // Optional inversion for CLK
        .IS_INMODE_INVERTED(5'b00000),     // Optional inversion for INMODE
        .IS_OPMODE_INVERTED(9'b000000000), // Optional inversion for OPMODE
        .IS_RSTALLCARRYIN_INVERTED(1'b0),  // Optional inversion for RSTALLCARRYIN
        .IS_RSTALUMODE_INVERTED(1'b0),     // Optional inversion for RSTALUMODE
        .IS_RSTA_INVERTED(1'b0),           // Optional inversion for RSTA
        .IS_RSTB_INVERTED(1'b0),           // Optional inversion for RSTB
        .IS_RSTCTRL_INVERTED(1'b0),        // Optional inversion for RSTCTRL
        .IS_RSTC_INVERTED(1'b0),           // Optional inversion for RSTC
        .IS_RSTD_INVERTED(1'b0),           // Optional inversion for RSTD
        .IS_RSTINMODE_INVERTED(1'b0),      // Optional inversion for RSTINMODE
        .IS_RSTM_INVERTED(1'b0),           // Optional inversion for RSTM
        .IS_RSTP_INVERTED(1'b0),           // Optional inversion for RSTP
        // Register Control Attributes: Pipeline Register Configuration
        .ACASCREG(0),                      // Number of pipeline stages between A/ACIN and ACOUT (0-2)
        .ADREG(0),                         // Pipeline stages for pre-adder (0-1)
        .ALUMODEREG(0),                    // Pipeline stages for ALUMODE (0-1)
        .AREG(0),                          // Pipeline stages for A (0-2)
        .BCASCREG(0),                      // Number of pipeline stages between B/BCIN and BCOUT (0-2)
        .BREG(0),                          // Pipeline stages for B (0-2)
        .CARRYINREG(0),                    // Pipeline stages for CARRYIN (0-1)
        .CARRYINSELREG(0),                 // Pipeline stages for CARRYINSEL (0-1)
        .CREG(0),                          // Pipeline stages for C (0-1)
        .DREG(0),                          // Pipeline stages for D (0-1)
        .INMODEREG(0),                     // Pipeline stages for INMODE (0-1)
        .MREG(0),                          // Multiplier pipeline stages (0-1)
        .OPMODEREG(0),                     // Pipeline stages for OPMODE (0-1)
        .PREG(0)                           // Number of pipeline stages for P (0-1)
    )
    DSP48E2_inst (
        // Cascade outputs: Cascade Ports
        .ACOUT(),                        // 30-bit output: A port cascade
        .BCOUT(),                        // 18-bit output: B cascade
        .CARRYCASCOUT(),                 // 1-bit output: Cascade carry
        .MULTSIGNOUT(),                  // 1-bit output: Multiplier sign cascade
        .PCOUT(),                        // 48-bit output: Cascade output
        // Control outputs: Control Inputs/Status Bits
        .OVERFLOW(),                     // 1-bit output: Overflow in add/acc
        .PATTERNBDETECT(),               // 1-bit output: Pattern bar detect
        .PATTERNDETECT(),                // 1-bit output: Pattern detect
        .UNDERFLOW(),                    // 1-bit output: Underflow in add/acc
        // Data outputs: Data Ports
        .CARRYOUT(),                     // 4-bit output: Carry
        .P(dsp_p),                       // 48-bit output: Primary data
        .XOROUT(),                       // 8-bit output: XOR data
        // Cascade inputs: Cascade Ports
        .ACIN(30'd0),                    // 30-bit input: A cascade data
        .BCIN(18'd0),                    // 18-bit input: B cascade
        .CARRYCASCIN(1'b0),              // 1-bit input: Cascade carry
        .MULTSIGNIN(1'b0),               // 1-bit input: Multiplier sign cascade
        .PCIN(48'd0),                    // 48-bit input: P cascade
        // Control inputs: Control Inputs/Status Bits
        .ALUMODE(dsp_alumode),           // 4-bit input: ALU control
        .CARRYINSEL(dsp_carryinsel),     // 3-bit input: Carry select
        .CLK(clock),                     // 1-bit input: Clock
        .INMODE(dsp_inmode),             // 5-bit input: INMODE control
        .OPMODE(dsp_opmode),             // 9-bit input: Operation mode
        // Data inputs: Data Ports
        .A(dsp_a),                       // 30-bit input: A data
        .B(dsp_b),                       // 18-bit input: B data
        .C(dsp_c),                       // 48-bit input: C data
        .CARRYIN(1'b0),                  // 1-bit input: Carry-in
        .D(27'd0),                       // 27-bit input: D data
        // Reset/Clock Enable inputs: Reset/Clock Enable Inputs
        .CEA1(ce),                       // 1-bit input: Clock enable for 1st stage AREG
        .CEA2(ce),                       // 1-bit input: Clock enable for 2nd stage AREG
        .CEAD(ce),                       // 1-bit input: Clock enable for ADREG
        .CEALUMODE(ce),                  // 1-bit input: Clock enable for ALUMODE
        .CEB1(ce),                       // 1-bit input: Clock enable for 1st stage BREG
        .CEB2(ce),                       // 1-bit input: Clock enable for 2nd stage BREG
        .CEC(ce),                        // 1-bit input: Clock enable for CREG
        .CECARRYIN(ce),                  // 1-bit input: Clock enable for CARRYINREG
        .CECTRL(ce),                     // 1-bit input: Clock enable for OPMODEREG and CARRYINSELREG
        .CED(ce),                        // 1-bit input: Clock enable for DREG
        .CEINMODE(ce),                   // 1-bit input: Clock enable for INMODEREG
        .CEM(ce),                        // 1-bit input: Clock enable for MREG
        .CEP(ce),                        // 1-bit input: Clock enable for PREG
        .RSTA(reset),                    // 1-bit input: Reset for AREG
        .RSTALLCARRYIN(reset),           // 1-bit input: Reset for CARRYINREG
        .RSTALUMODE(reset),              // 1-bit input: Reset for ALUMODEREG
        .RSTB(reset),                    // 1-bit input: Reset for BREG
        .RSTC(reset),                    // 1-bit input: Reset for CREG
        .RSTCTRL(reset),                 // 1-bit input: Reset for OPMODEREG and CARRYINSELREG
        .RSTD(reset),                    // 1-bit input: Reset for DREG and ADREG
        .RSTINMODE(reset),               // 1-bit input: Reset for INMODEREG
        .RSTM(reset),                    // 1-bit input: Reset for MREG
        .RSTP(reset)                     // 1-bit input: Reset for PREG
    );

    // End of DSP48E2_inst instantiation

endmodule
