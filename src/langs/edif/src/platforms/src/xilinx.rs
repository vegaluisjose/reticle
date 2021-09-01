/*
Copyright 2021 Pedro M. Torruella N.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
use edifier::ast::*;

pub fn new_lut2() -> Cell {
    let porto = InterfacePort::new_output("O");

    let porti0 = InterfacePort::new_input("I0");

    let porti1 = InterfacePort::new_input("I1");

    let interface = CellInterface(vec![porto, porti0, porti1]);

    let cellview = CellView {
        name: "netlist".to_string(),
        interface,
        contents: CellContents(Vec::new()),
        properties: PropertyList::from(Vec::new()),
    };
    Cell {
        name: "LUT2".to_string(),
        views: CellViews(vec![cellview]),
    }
}

pub fn lut2_prop_ini<S>(val: S) -> Property
where
    S: AsRef<str>,
{
    Property {
        name: "INIT".to_string(),
        property: PropertyValue::from(val.as_ref().to_string()),
    }
}

pub fn lut2_prop_box<S>(val: S) -> Property
where
    S: AsRef<str>,
{
    Property {
        name: "BOX_TYPE".to_string(),
        property: PropertyValue::from(val.as_ref().to_string()),
    }
}

pub fn lut2_prop_loc<S>(val: S) -> Property
where
    S: AsRef<str>,
{
    Property {
        name: "LOC".to_string(),
        property: PropertyValue::from(val.as_ref().to_string()),
    }
}

pub fn lut2_prop_bel<S>(val: S) -> Property
where
    S: AsRef<str>,
{
    Property {
        name: "BEL".to_string(),
        property: PropertyValue::from(val.as_ref().to_string()),
    }
}

pub fn carry8() -> Cell {
    let mut interface = CellInterface(Vec::new());

    interface.push(InterfacePort::new_input("CI"));
    interface.push(InterfacePort::new_input("CI_TOP"));
    interface.push(InterfacePort::new(
        PortToken::new_array("CO", "CO[7:0]", 8),
        PortDirection::Output,
    ));
    interface.push(InterfacePort::new(
        PortToken::new_array("O", "O[7:0]", 8),
        PortDirection::Output,
    ));
    interface.push(InterfacePort::new(
        PortToken::new_array("DI", "DI[7:0]", 8),
        PortDirection::Input,
    ));
    interface.push(InterfacePort::new(
        PortToken::new_array("S", "S[7:0]", 8),
        PortDirection::Input,
    ));

    let cellview = CellView {
        name: "netlist".to_string(),
        interface,
        contents: CellContents(Vec::new()),
        properties: PropertyList::from(Vec::new()),
    };
    Cell {
        name: "CARRY8".to_string(),
        views: CellViews(vec![cellview]),
    }
}

pub fn inverter() -> Edif {
    let top_name = "inverter".to_string();
    let work_name = "work".to_string();
    let netlist = "netlist".to_string();

    let elems = Cells::from(vec![
        Cell {
            name: "LUT2".to_string(),
            views: CellViews(vec![CellView {
                name: netlist.clone(),
                interface: CellInterface(vec![
                    InterfacePort::new_output("O"),
                    InterfacePort::new_input("I0"),
                    InterfacePort::new_input("I1"),
                ]),
                contents: CellContents(Vec::new()),
                properties: PropertyList::from(Vec::new()),
            }]),
        },
        Cell {
            name: "INV".to_string(),
            views: CellViews(vec![CellView {
                name: netlist.clone(),
                interface: CellInterface(vec![
                    InterfacePort::new_input("I"),
                    InterfacePort::new_output("O"),
                ]),
                contents: CellContents(Vec::new()),
                properties: PropertyList::from(Vec::new()),
            }]),
        },
    ]);

    let lib_prims = Library {
        name: "hdi_primitives".to_string(),
        elements: elems,
    };

    let yinst0_name = "y_INST_0".to_string();

    let yinst0 = ContentElement::from(ContentInstance {
        token: StringToken::new(yinst0_name.clone()),
        viewref: netlist,
        cellref: CellRef::new("LUT2", "hdi_primitives"),
        properties: PropertyList(vec![Property::new_string("INIT", "4'h8")]),
    });

    let neta_name = "a".to_string();
    let neta = ContentElement::from(ContentNet::new_with_ports(
        neta_name.clone(),
        PortList(vec![
            PortRef::new_with_ref("I0", InstanceRef::new(yinst0_name.clone())),
            PortRef::new(neta_name.clone()),
        ]),
    ));

    let netb_name = "b".to_string();
    let netb = ContentElement::from(ContentNet::new_with_ports(
        netb_name.clone(),
        PortList(vec![
            PortRef::new_with_ref("I1", InstanceRef::new(yinst0_name.clone())),
            PortRef::new(netb_name.clone()),
        ]),
    ));

    let nety_name = "y".to_string();
    let nety = ContentElement::from(ContentNet::new_with_ports(
        nety_name.clone(),
        PortList(vec![
            PortRef::new_with_ref("O", InstanceRef::new(yinst0_name)),
            PortRef::new(nety_name.clone()),
        ]),
    ));

    let inv = Cell {
        name: top_name.clone(),
        views: CellViews(vec![CellView {
            name: top_name.clone(),
            interface: CellInterface(vec![
                InterfacePort::new_input(neta_name),
                InterfacePort::new_input(netb_name),
                InterfacePort::new_output(nety_name),
            ]),
            contents: CellContents(vec![yinst0, neta, netb, nety]),
            properties: PropertyList::from(Vec::new()),
        }]),
    };

    let lib_work = Library {
        name: work_name.clone(),
        elements: Cells::from(vec![inv]),
    };

    let design_inv = Design::new_with_prop(
        top_name.clone(),
        CellRef::new(top_name.clone(), work_name),
        PropertyList(vec![Property::new_string("part", "xczu3eg-sbva484-1-e")]),
    );

    let libp = EdifElement::from(lib_prims);
    let libw = EdifElement::from(lib_work);
    let desi = EdifElement::from(design_inv);

    Edif {
        name: top_name,
        elements: EdifElements::from(vec![libp, libw, desi]),
    }
}

pub fn dsp() -> Edif {
    let top_name = "dsp2".to_string();
    let work_name = "work".to_string();
    let netlist = "netlist".to_string();

    // Building the Primitive's library
    let elems = Cells::from(vec![
        Cell {
            name: "GND".to_string(),
            views: CellViews(vec![CellView {
                name: netlist.clone(),
                interface: CellInterface(vec![InterfacePort::new_output("G")]),
                contents: CellContents(Vec::new()),
                properties: PropertyList::from(Vec::new()),
            }]),
        },
        Cell {
            name: "VCC".to_string(),
            views: CellViews(vec![CellView {
                name: netlist.clone(),
                interface: CellInterface(vec![InterfacePort::new_output("P")]),
                contents: CellContents(Vec::new()),
                properties: PropertyList::from(Vec::new()),
            }]),
        },
        Cell {
            name: "DSP48E2".to_string(),
            views: CellViews(vec![CellView {
                name: netlist.clone(),
                interface: CellInterface(vec![
                    InterfacePort::new_output("CARRYCASCOUT"),
                    InterfacePort::new_output("MULTSIGNOUT"),
                    InterfacePort::new_output("OVERFLOW"),
                    InterfacePort::new_output("PATTERNBDETECT"),
                    InterfacePort::new_output("PATTERNDETECT"),
                    InterfacePort::new_output("UNDERFLOW"),
                    InterfacePort::new_input("CARRYCASCIN"),
                    InterfacePort::new_input("CARRYIN"),
                    InterfacePort::new_input("CEA1"),
                    InterfacePort::new_input("CEA2"),
                    InterfacePort::new_input("CEAD"),
                    InterfacePort::new_input("CEALUMODE"),
                    InterfacePort::new_input("CEB1"),
                    InterfacePort::new_input("CEB2"),
                    InterfacePort::new_input("CEC"),
                    InterfacePort::new_input("CECARRYIN"),
                    InterfacePort::new_input("CECTRL"),
                    InterfacePort::new_input("CED"),
                    InterfacePort::new_input("CEINMODE"),
                    InterfacePort::new_input("CEM"),
                    InterfacePort::new_input("CEP"),
                    InterfacePort::new_input("CLK"),
                    InterfacePort::new_input("MULTSIGNIN"),
                    InterfacePort::new_input("RSTA"),
                    InterfacePort::new_input("RSTALLCARRYIN"),
                    InterfacePort::new_input("RSTALUMODE"),
                    InterfacePort::new_input("RSTB"),
                    InterfacePort::new_input("RSTC"),
                    InterfacePort::new_input("RSTCTRL"),
                    InterfacePort::new_input("RSTD"),
                    InterfacePort::new_input("RSTINMODE"),
                    InterfacePort::new_input("RSTM"),
                    InterfacePort::new_input("RSTP"),
                    InterfacePort::new(
                        PortToken::new_array("ACOUT", "ACOUT[29:0]", 30),
                        PortDirection::Output,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("BCOUT", "BCOUT[17:0]", 18),
                        PortDirection::Output,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("CARRYOUT", "CARRYOUT[3:0]", 4),
                        PortDirection::Output,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("PCOUT", "PCOUT[47:0]", 48),
                        PortDirection::Output,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("P", "P[47:0]", 48),
                        PortDirection::Output,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("XOROUT", "XOROUT[7:0]", 8),
                        PortDirection::Output,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("ACIN", "ACIN[29:0]", 30),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("ALUMODE", "ALUMODE[3:0]", 4),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("A", "A[29:0]", 30),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("BCIN", "BCIN[17:0]", 18),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("B", "B[17:0]", 18),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("CARRYINSEL", "CARRYINSEL[2:0]", 3),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("C", "C[47:0]", 48),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("D", "D[26:0]", 27),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("INMODE", "INMODE[4:0]", 5),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("OPMODE", "OPMODE[8:0]", 9),
                        PortDirection::Input,
                    ),
                    InterfacePort::new(
                        PortToken::new_array("PCIN", "PCIN[47:0]", 48),
                        PortDirection::Input,
                    ),
                ]),
                contents: CellContents(Vec::new()),
                properties: PropertyList::from(Vec::new()),
            }]),
        },
        Cell {
            name: "INV".to_string(),
            views: CellViews(vec![CellView {
                name: netlist.clone(),
                interface: CellInterface(vec![
                    InterfacePort::new_input("I"),
                    InterfacePort::new_output("O"),
                ]),
                contents: CellContents(Vec::new()),
                properties: PropertyList::from(Vec::new()),
            }]),
        },
    ]);

    let lib_prims = Library {
        name: "hdi_primitives".to_string(),
        elements: elems,
    };

    // Building the elemtns for the work library
    let gnd_name = "GND".to_string();
    let gnd_ref = InstanceRef::new(gnd_name.clone());
    let gnd = ContentElement::from(ContentInstance {
        token: StringToken::new(gnd_name),
        viewref: netlist.clone(),
        cellref: CellRef::new("GND", "hdi_primitives"),
        properties: PropertyList(vec![]),
    });

    let gnd1_name = "GND_1".to_string();
    let gnd1_ref = InstanceRef::new(gnd1_name.clone());
    let gnd1 = ContentElement::from(ContentInstance {
        token: StringToken::new(gnd1_name),
        viewref: netlist.clone(),
        cellref: CellRef::new("GND", "hdi_primitives"),
        properties: PropertyList(vec![]),
    });

    let vcc_name = "VCC".to_string();
    let vccref = InstanceRef::new(vcc_name.clone());
    let vcc = ContentElement::from(ContentInstance {
        token: StringToken::new(vcc_name),
        viewref: netlist.clone(),
        cellref: CellRef::new("VCC", "hdi_primitives"),
        properties: PropertyList(vec![]),
    });

    let vcc1_name = "VCC_1".to_string();
    let vcc1_ref = InstanceRef::new(vcc1_name.clone());
    let vcc1 = ContentElement::from(ContentInstance {
        token: StringToken::new(vcc1_name),
        viewref: netlist.clone(),
        cellref: CellRef::new("VCC", "hdi_primitives"),
        properties: PropertyList(vec![]),
    });

    let dsp_name = "res".to_string();
    let dspref = InstanceRef::new(dsp_name.clone());
    let dspinst = ContentElement::from(ContentInstance {
        token: StringToken::new(dsp_name),
        viewref: netlist,
        cellref: CellRef::new("DSP48E2", "hdi_primitives"),
        properties: PropertyList(vec![
            Property::new_string("XORSIMD", "XOR24_48_96"),
            Property::new_string("USE_WIDEXOR", "FALSE"),
            Property::new_integer("ACASCREG", 0),
            Property::new_integer("ADREG", 1),
            Property::new_integer("ALUMODEREG", 0),
            Property::new_string("AMULTSEL", "A"),
            Property::new_integer("AREG", 0),
            Property::new_string("AUTORESET_PATDET", "NO_RESET"),
            Property::new_string("AUTORESET_PRIORITY", "RESET"),
            Property::new_string("A_INPUT", "DIRECT"),
            Property::new_integer("BCASCREG", 0),
            Property::new_string("BMULTSEL", "B"),
            Property::new_integer("BREG", 0),
            Property::new_string("B_INPUT", "DIRECT"),
            Property::new_integer("CARRYINREG", 0),
            Property::new_integer("CARRYINSELREG", 0),
            Property::new_integer("CREG", 1),
            Property::new_integer("DREG", 1),
            Property::new_integer("INMODEREG", 0),
            Property::new_string("MASK", "48'h3FFFFFFFFFFF"),
            Property::new_string("METHODOLOGY_DRC_VIOS", r#"{SYNTH-13 {cell *THIS*}}"#),
            Property::new_integer("MREG", 0),
            Property::new_integer("OPMODEREG", 0),
            Property::new_string("PATTERN", "48'h000000000000"),
            Property::new_string("PREADDINSEL", "A"),
            Property::new_integer("PREG", 0),
            Property::new_string("RND", "48'h000000000000"),
            Property::new_string("SEL_MASK", "MASK"),
            Property::new_string("SEL_PATTERN", "PATTERN"),
            Property::new_string("USE_MULT", "MULTIPLY"),
            Property::new_string("USE_PATTERN_DETECT", "NO_PATDET"),
            Property::new_string("USE_SIMD", "ONE48"),
            Property::new_string("OPT_MODIFIED", "SWEEP"),
        ]),
    });

    let const_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "&_const0_",
        "<const0>",
        PortList(vec![
            PortRef::new_member_with_ref("ACIN", 29, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 19, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 18, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 17, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 16, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 15, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 14, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 13, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 12, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 11, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 10, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 28, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 9, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 8, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 7, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 6, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 5, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 4, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 3, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 2, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 1, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 0, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 27, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 26, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 25, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 24, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 23, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 22, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 21, dspref.clone()),
            PortRef::new_member_with_ref("ACIN", 20, dspref.clone()),
            PortRef::new_member_with_ref("ALUMODE", 3, dspref.clone()),
            PortRef::new_member_with_ref("ALUMODE", 2, dspref.clone()),
            PortRef::new_member_with_ref("ALUMODE", 1, dspref.clone()),
            PortRef::new_member_with_ref("ALUMODE", 0, dspref.clone()),
            PortRef::new_member_with_ref("A", 19, dspref.clone()),
            PortRef::new_member_with_ref("A", 18, dspref.clone()),
            PortRef::new_member_with_ref("A", 17, dspref.clone()),
            PortRef::new_member_with_ref("A", 16, dspref.clone()),
            PortRef::new_member_with_ref("A", 15, dspref.clone()),
            PortRef::new_member_with_ref("A", 14, dspref.clone()),
            PortRef::new_member_with_ref("A", 13, dspref.clone()),
            PortRef::new_member_with_ref("A", 12, dspref.clone()),
            PortRef::new_member_with_ref("A", 11, dspref.clone()),
            PortRef::new_member_with_ref("A", 10, dspref.clone()),
            PortRef::new_member_with_ref("A", 9, dspref.clone()),
            PortRef::new_member_with_ref("A", 8, dspref.clone()),
            PortRef::new_member_with_ref("A", 7, dspref.clone()),
            PortRef::new_member_with_ref("A", 6, dspref.clone()),
            PortRef::new_member_with_ref("A", 5, dspref.clone()),
            PortRef::new_member_with_ref("A", 4, dspref.clone()),
            PortRef::new_member_with_ref("A", 3, dspref.clone()),
            PortRef::new_member_with_ref("A", 2, dspref.clone()),
            PortRef::new_member_with_ref("A", 1, dspref.clone()),
            PortRef::new_member_with_ref("A", 0, dspref.clone()),
            PortRef::new_member_with_ref("A", 21, dspref.clone()),
            PortRef::new_member_with_ref("A", 20, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 17, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 7, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 6, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 5, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 4, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 3, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 2, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 1, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 0, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 16, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 15, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 14, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 13, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 12, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 11, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 10, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 9, dspref.clone()),
            PortRef::new_member_with_ref("BCIN", 8, dspref.clone()),
            PortRef::new_member_with_ref("B", 7, dspref.clone()),
            PortRef::new_member_with_ref("B", 6, dspref.clone()),
            PortRef::new_member_with_ref("B", 5, dspref.clone()),
            PortRef::new_member_with_ref("B", 4, dspref.clone()),
            PortRef::new_member_with_ref("B", 3, dspref.clone()),
            PortRef::new_member_with_ref("B", 2, dspref.clone()),
            PortRef::new_member_with_ref("B", 1, dspref.clone()),
            PortRef::new_member_with_ref("B", 0, dspref.clone()),
            PortRef::new_member_with_ref("B", 9, dspref.clone()),
            PortRef::new_member_with_ref("B", 8, dspref.clone()),
            PortRef::new_with_ref("CARRYCASCIN", dspref.clone()),
            PortRef::new_with_ref("CARRYIN", dspref.clone()),
            PortRef::new_member_with_ref("CARRYINSEL", 2, dspref.clone()),
            PortRef::new_member_with_ref("CARRYINSEL", 1, dspref.clone()),
            PortRef::new_member_with_ref("CARRYINSEL", 0, dspref.clone()),
            PortRef::new_with_ref("CEA1", dspref.clone()),
            PortRef::new_with_ref("CEA2", dspref.clone()),
            PortRef::new_with_ref("CEAD", dspref.clone()),
            PortRef::new_with_ref("CEALUMODE", dspref.clone()),
            PortRef::new_with_ref("CEB1", dspref.clone()),
            PortRef::new_with_ref("CEB2", dspref.clone()),
            PortRef::new_with_ref("CEC", dspref.clone()),
            PortRef::new_with_ref("CECARRYIN", dspref.clone()),
            PortRef::new_with_ref("CECTRL", dspref.clone()),
            PortRef::new_with_ref("CED", dspref.clone()),
            PortRef::new_with_ref("CEINMODE", dspref.clone()),
            PortRef::new_with_ref("CEM", dspref.clone()),
            PortRef::new_with_ref("CEP", dspref.clone()),
            PortRef::new_with_ref("CLK", dspref.clone()),
            PortRef::new_with_ref("G", gnd_ref),
            PortRef::new_member_with_ref("INMODE", 4, dspref.clone()),
            PortRef::new_member_with_ref("INMODE", 3, dspref.clone()),
            PortRef::new_member_with_ref("INMODE", 2, dspref.clone()),
            PortRef::new_member_with_ref("INMODE", 1, dspref.clone()),
            PortRef::new_member_with_ref("INMODE", 0, dspref.clone()),
            PortRef::new_with_ref("MULTSIGNIN", dspref.clone()),
            PortRef::new_member_with_ref("OPMODE", 7, dspref.clone()),
            PortRef::new_member_with_ref("OPMODE", 5, dspref.clone()),
            PortRef::new_member_with_ref("OPMODE", 4, dspref.clone()),
            PortRef::new_member_with_ref("OPMODE", 3, dspref.clone()),
            PortRef::new_member_with_ref("OPMODE", 2, dspref.clone()),
            PortRef::new_member_with_ref("OPMODE", 1, dspref.clone()),
            PortRef::new_member_with_ref("OPMODE", 0, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 47, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 37, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 36, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 35, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 34, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 33, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 32, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 31, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 30, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 29, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 28, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 46, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 27, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 26, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 25, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 24, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 23, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 22, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 21, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 20, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 19, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 18, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 45, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 17, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 16, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 15, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 14, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 13, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 12, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 11, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 10, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 9, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 8, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 44, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 7, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 6, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 5, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 4, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 3, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 2, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 1, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 0, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 43, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 42, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 41, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 40, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 39, dspref.clone()),
            PortRef::new_member_with_ref("PCIN", 38, dspref.clone()),
            PortRef::new_with_ref("RSTA", dspref.clone()),
            PortRef::new_with_ref("RSTALLCARRYIN", dspref.clone()),
            PortRef::new_with_ref("RSTALUMODE", dspref.clone()),
            PortRef::new_with_ref("RSTB", dspref.clone()),
            PortRef::new_with_ref("RSTC", dspref.clone()),
            PortRef::new_with_ref("RSTCTRL", dspref.clone()),
            PortRef::new_with_ref("RSTD", dspref.clone()),
            PortRef::new_with_ref("RSTINMODE", dspref.clone()),
            PortRef::new_with_ref("RSTM", dspref.clone()),
            PortRef::new_with_ref("RSTP", dspref.clone()),
        ]),
    ));

    let const1_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "&_const1_",
        "<const1>",
        PortList(vec![
            PortRef::new_member_with_ref("OPMODE", 8, dspref.clone()),
            PortRef::new_member_with_ref("OPMODE", 6, dspref.clone()),
            PortRef::new_with_ref("P", vccref),
        ]),
    ));

    let nety_name = "GND_2".to_string();
    let gnd2 = ContentElement::from(ContentNet::new_with_ports(
        nety_name,
        PortList(vec![
            PortRef::new_member_with_ref("D", 26, dspref.clone()),
            PortRef::new_member_with_ref("D", 16, dspref.clone()),
            PortRef::new_member_with_ref("D", 15, dspref.clone()),
            PortRef::new_member_with_ref("D", 14, dspref.clone()),
            PortRef::new_member_with_ref("D", 13, dspref.clone()),
            PortRef::new_member_with_ref("D", 12, dspref.clone()),
            PortRef::new_member_with_ref("D", 11, dspref.clone()),
            PortRef::new_member_with_ref("D", 10, dspref.clone()),
            PortRef::new_member_with_ref("D", 9, dspref.clone()),
            PortRef::new_member_with_ref("D", 8, dspref.clone()),
            PortRef::new_member_with_ref("D", 7, dspref.clone()),
            PortRef::new_member_with_ref("D", 25, dspref.clone()),
            PortRef::new_member_with_ref("D", 6, dspref.clone()),
            PortRef::new_member_with_ref("D", 5, dspref.clone()),
            PortRef::new_member_with_ref("D", 4, dspref.clone()),
            PortRef::new_member_with_ref("D", 3, dspref.clone()),
            PortRef::new_member_with_ref("D", 2, dspref.clone()),
            PortRef::new_member_with_ref("D", 1, dspref.clone()),
            PortRef::new_member_with_ref("D", 0, dspref.clone()),
            PortRef::new_member_with_ref("D", 24, dspref.clone()),
            PortRef::new_member_with_ref("D", 23, dspref.clone()),
            PortRef::new_member_with_ref("D", 22, dspref.clone()),
            PortRef::new_member_with_ref("D", 21, dspref.clone()),
            PortRef::new_member_with_ref("D", 20, dspref.clone()),
            PortRef::new_member_with_ref("D", 19, dspref.clone()),
            PortRef::new_member_with_ref("D", 18, dspref.clone()),
            PortRef::new_member_with_ref("D", 17, dspref.clone()),
            PortRef::new_with_ref("G", gnd1_ref),
        ]),
    ));

    let vcc2_name = "VCC_2".to_string();
    let vcc2 = ContentElement::from(ContentNet::new_with_ports(
        vcc2_name,
        PortList(vec![
            PortRef::new_member_with_ref("C", 47, dspref.clone()),
            PortRef::new_member_with_ref("C", 37, dspref.clone()),
            PortRef::new_member_with_ref("C", 36, dspref.clone()),
            PortRef::new_member_with_ref("C", 35, dspref.clone()),
            PortRef::new_member_with_ref("C", 34, dspref.clone()),
            PortRef::new_member_with_ref("C", 33, dspref.clone()),
            PortRef::new_member_with_ref("C", 32, dspref.clone()),
            PortRef::new_member_with_ref("C", 31, dspref.clone()),
            PortRef::new_member_with_ref("C", 30, dspref.clone()),
            PortRef::new_member_with_ref("C", 29, dspref.clone()),
            PortRef::new_member_with_ref("C", 28, dspref.clone()),
            PortRef::new_member_with_ref("C", 46, dspref.clone()),
            PortRef::new_member_with_ref("C", 27, dspref.clone()),
            PortRef::new_member_with_ref("C", 26, dspref.clone()),
            PortRef::new_member_with_ref("C", 25, dspref.clone()),
            PortRef::new_member_with_ref("C", 24, dspref.clone()),
            PortRef::new_member_with_ref("C", 23, dspref.clone()),
            PortRef::new_member_with_ref("C", 22, dspref.clone()),
            PortRef::new_member_with_ref("C", 21, dspref.clone()),
            PortRef::new_member_with_ref("C", 20, dspref.clone()),
            PortRef::new_member_with_ref("C", 19, dspref.clone()),
            PortRef::new_member_with_ref("C", 18, dspref.clone()),
            PortRef::new_member_with_ref("C", 45, dspref.clone()),
            PortRef::new_member_with_ref("C", 17, dspref.clone()),
            PortRef::new_member_with_ref("C", 16, dspref.clone()),
            PortRef::new_member_with_ref("C", 15, dspref.clone()),
            PortRef::new_member_with_ref("C", 14, dspref.clone()),
            PortRef::new_member_with_ref("C", 13, dspref.clone()),
            PortRef::new_member_with_ref("C", 12, dspref.clone()),
            PortRef::new_member_with_ref("C", 11, dspref.clone()),
            PortRef::new_member_with_ref("C", 10, dspref.clone()),
            PortRef::new_member_with_ref("C", 9, dspref.clone()),
            PortRef::new_member_with_ref("C", 8, dspref.clone()),
            PortRef::new_member_with_ref("C", 44, dspref.clone()),
            PortRef::new_member_with_ref("C", 7, dspref.clone()),
            PortRef::new_member_with_ref("C", 6, dspref.clone()),
            PortRef::new_member_with_ref("C", 5, dspref.clone()),
            PortRef::new_member_with_ref("C", 4, dspref.clone()),
            PortRef::new_member_with_ref("C", 3, dspref.clone()),
            PortRef::new_member_with_ref("C", 2, dspref.clone()),
            PortRef::new_member_with_ref("C", 1, dspref.clone()),
            PortRef::new_member_with_ref("C", 0, dspref.clone()),
            PortRef::new_member_with_ref("C", 43, dspref.clone()),
            PortRef::new_member_with_ref("C", 42, dspref.clone()),
            PortRef::new_member_with_ref("C", 41, dspref.clone()),
            PortRef::new_member_with_ref("C", 40, dspref.clone()),
            PortRef::new_member_with_ref("C", 39, dspref.clone()),
            PortRef::new_member_with_ref("C", 38, dspref.clone()),
            PortRef::new_with_ref("P", vcc1_ref),
        ]),
    ));

    let a0_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "a_0_",
        "a[0]",
        PortList(vec![
            PortRef::new_member_with_ref("A", 29, dspref.clone()),
            PortRef::new_member("a", 7),
        ]),
    ));
    let a1_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "a_1_",
        "a[1]",
        PortList(vec![
            PortRef::new_member_with_ref("A", 28, dspref.clone()),
            PortRef::new_member("a", 6),
        ]),
    ));
    let a2_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "a_2_",
        "a[2]",
        PortList(vec![
            PortRef::new_member_with_ref("A", 27, dspref.clone()),
            PortRef::new_member("a", 5),
        ]),
    ));
    let a3_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "a_3_",
        "a[3]",
        PortList(vec![
            PortRef::new_member_with_ref("A", 26, dspref.clone()),
            PortRef::new_member("a", 4),
        ]),
    ));
    let a4_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "a_4_",
        "a[4]",
        PortList(vec![
            PortRef::new_member_with_ref("A", 25, dspref.clone()),
            PortRef::new_member("a", 3),
        ]),
    ));
    let a5_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "a_5_",
        "a[5]",
        PortList(vec![
            PortRef::new_member_with_ref("A", 24, dspref.clone()),
            PortRef::new_member("a", 2),
        ]),
    ));
    let a6_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "a_6_",
        "a[6]",
        PortList(vec![
            PortRef::new_member_with_ref("A", 23, dspref.clone()),
            PortRef::new_member("a", 1),
        ]),
    ));
    let a7_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "a_7_",
        "a[7]",
        PortList(vec![
            PortRef::new_member_with_ref("A", 22, dspref.clone()),
            PortRef::new_member("a", 0),
        ]),
    ));
    let b0_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "b_0_",
        "b[0]",
        PortList(vec![
            PortRef::new_member_with_ref("B", 17, dspref.clone()),
            PortRef::new_member("b", 7),
        ]),
    ));
    let b1_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "b_1_",
        "b[1]",
        PortList(vec![
            PortRef::new_member_with_ref("B", 16, dspref.clone()),
            PortRef::new_member("b", 6),
        ]),
    ));
    let b2_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "b_2_",
        "b[2]",
        PortList(vec![
            PortRef::new_member_with_ref("B", 15, dspref.clone()),
            PortRef::new_member("b", 5),
        ]),
    ));
    let b3_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "b_3_",
        "b[3]",
        PortList(vec![
            PortRef::new_member_with_ref("B", 14, dspref.clone()),
            PortRef::new_member("b", 4),
        ]),
    ));
    let b4_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "b_4_",
        "b[4]",
        PortList(vec![
            PortRef::new_member_with_ref("B", 13, dspref.clone()),
            PortRef::new_member("b", 3),
        ]),
    ));
    let b5_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "b_5_",
        "b[5]",
        PortList(vec![
            PortRef::new_member_with_ref("B", 12, dspref.clone()),
            PortRef::new_member("b", 2),
        ]),
    ));
    let b6_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "b_6_",
        "b[6]",
        PortList(vec![
            PortRef::new_member_with_ref("B", 11, dspref.clone()),
            PortRef::new_member("b", 1),
        ]),
    ));
    let b7_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "b_7_",
        "b[7]",
        PortList(vec![
            PortRef::new_member_with_ref("B", 10, dspref.clone()),
            PortRef::new_member("b", 0),
        ]),
    ));
    let y0_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "y_0_",
        "y[0]",
        PortList(vec![
            PortRef::new_member_with_ref("P", 47, dspref.clone()),
            PortRef::new_member("y", 7),
        ]),
    ));
    let y1_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "y_1_",
        "y[1]",
        PortList(vec![
            PortRef::new_member_with_ref("P", 46, dspref.clone()),
            PortRef::new_member("y", 6),
        ]),
    ));
    let y2_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "y_2_",
        "y[2]",
        PortList(vec![
            PortRef::new_member_with_ref("P", 45, dspref.clone()),
            PortRef::new_member("y", 5),
        ]),
    ));
    let y3_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "y_3_",
        "y[3]",
        PortList(vec![
            PortRef::new_member_with_ref("P", 44, dspref.clone()),
            PortRef::new_member("y", 4),
        ]),
    ));
    let y4_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "y_4_",
        "y[4]",
        PortList(vec![
            PortRef::new_member_with_ref("P", 43, dspref.clone()),
            PortRef::new_member("y", 3),
        ]),
    ));
    let y5_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "y_5_",
        "y[5]",
        PortList(vec![
            PortRef::new_member_with_ref("P", 42, dspref.clone()),
            PortRef::new_member("y", 2),
        ]),
    ));
    let y6_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "y_6_",
        "y[6]",
        PortList(vec![
            PortRef::new_member_with_ref("P", 41, dspref.clone()),
            PortRef::new_member("y", 1),
        ]),
    ));
    let y7_net = ContentElement::from(ContentNet::new_renamed_with_ports(
        "y_7_",
        "y[7]",
        PortList(vec![
            PortRef::new_member_with_ref("P", 40, dspref),
            PortRef::new_member("y", 0),
        ]),
    ));

    let dsp_cell = Cell {
        name: top_name.clone(),
        views: CellViews(vec![CellView {
            name: top_name.clone(),
            interface: CellInterface(vec![
                InterfacePort::new_input_array("a", "a[7:0]", 8),
                InterfacePort::new_input_array("b", "b[7:0]", 8),
                InterfacePort::new_output_array("y", "y[7:0]", 8),
            ]),
            contents: CellContents(vec![
                gnd, gnd1, vcc, vcc1, dspinst, const_net, const1_net, gnd2, vcc2, a0_net, a1_net,
                a2_net, a3_net, a4_net, a5_net, a6_net, a7_net, b0_net, b1_net, b2_net, b3_net,
                b4_net, b5_net, b6_net, b7_net, y0_net, y1_net, y2_net, y3_net, y4_net, y5_net,
                y6_net, y7_net,
            ]),
            properties: PropertyList(vec![Property::new_string("ECO_CHECKSUM", "2ac8fb58")]),
        }]),
    };

    let lib_work = Library {
        name: work_name.clone(),
        elements: Cells::from(vec![dsp_cell]),
    };

    let design_dsp = Design::new_with_prop(
        top_name.clone(),
        CellRef::new(top_name.clone(), work_name),
        PropertyList(vec![Property::new_string("part", "xczu3eg-sbva484-1-e")]),
    );

    let libp = EdifElement::from(lib_prims);
    let libw = EdifElement::from(lib_work);
    let desi = EdifElement::from(design_dsp);

    Edif {
        name: top_name,
        elements: EdifElements::from(vec![libp, libw, desi]),
    }
}
