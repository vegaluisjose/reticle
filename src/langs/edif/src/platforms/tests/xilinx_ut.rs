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
use edifier::string_helpers::match_check;
use platforms::xilinx::*;

// Test 1: we should get a lut2 element
#[test]
fn lut2() {
    let ed = new_lut2();

    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(
        actual,
        "(cell LUT2 (celltype GENERIC) (view netlist (viewtype NETLIST) (interface (port O (direction OUTPUT)) (port I0 (direction INPUT)) (port I1 (direction INPUT)))))"
    );
    //assert_eq!(match_check(actual), 0);
}

// Test 2: we should get an instance with properties
//         for a placed lut2 element
#[test]
fn lut2_instance() {
    let mut proplist = PropertyList(Vec::new());
    proplist.push(lut2_prop_ini("4'h6"));
    proplist.push(lut2_prop_box("PRIMITIVE"));
    proplist.push(lut2_prop_loc("SLICE_X0Y0"));
    proplist.push(lut2_prop_bel("A6LUT"));

    let contents = ContentInstance {
        token: StringToken::new("i0"),
        viewref: "netlist".to_string(),
        cellref: CellRef::new("LUT2", "hdi_primitives"),
        properties: proplist,
    };

    let actual = serde_sexpr::to_string(&contents).unwrap();

    assert_eq!(
        actual,
        r#"(instance i0 (viewref netlist (cellref LUT2 (libraryref hdi_primitives))) (property INIT (string "4'h6")) (property BOX_TYPE (string "PRIMITIVE")) (property LOC (string "SLICE_X0Y0")) (property BEL (string "A6LUT")))"#
    );
}

// Test 3: we should get an instance with properties
//         for a placed lut2 element, being renamed
#[test]
fn lut2_instance_renamed() {
    let mut proplist = PropertyList(Vec::new());
    proplist.push(lut2_prop_ini("4'h6"));
    proplist.push(lut2_prop_box("PRIMITIVE"));
    proplist.push(lut2_prop_loc("SLICE_X0Y0"));
    proplist.push(lut2_prop_bel("A6LUT"));

    let contents = ContentInstance {
        token: StringToken::new_renamed(
            "address_loop_2__output_data_pc_vector_mux_lut",
            r#"address_loop[2].output_data.pc_vector_mux_lut"#,
        ),
        viewref: "netlist".to_string(),
        cellref: CellRef::new("LUT2", "hdi_primitives"),
        properties: proplist,
    };

    let actual = serde_sexpr::to_string(&contents).unwrap();

    assert_eq!(
        actual,
        r#"(instance (rename address_loop_2__output_data_pc_vector_mux_lut "address_loop[2].output_data.pc_vector_mux_lut") (viewref netlist (cellref LUT2 (libraryref hdi_primitives))) (property INIT (string "4'h6")) (property BOX_TYPE (string "PRIMITIVE")) (property LOC (string "SLICE_X0Y0")) (property BEL (string "A6LUT")))"#
    );
}

// Test 4: Testing arrays
#[test]
fn simple_array() {
    let ed = carry8();

    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(
        actual,
        r#"(cell CARRY8 (celltype GENERIC) (view netlist (viewtype NETLIST) (interface (port CI (direction INPUT)) (port CI_TOP (direction INPUT)) (port (array (rename CO "CO[7:0]") 8) (direction OUTPUT)) (port (array (rename O "O[7:0]") 8) (direction OUTPUT)) (port (array (rename DI "DI[7:0]") 8) (direction INPUT)) (port (array (rename S "S[7:0]") 8) (direction INPUT)))))"#
    );
    assert_eq!(match_check(actual), 0);
}

// Test 5: Testing the inverter example
#[test]
fn test_inverter_top() {
    let edif = inverter();

    let actual = serde_sexpr::to_string(&edif).unwrap();

    assert_eq!(
        actual,
        r#"(edif inverter (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)) (Library hdi_primitives (edifLevel 0) (technology (numberDefinition)) (cell LUT2 (celltype GENERIC) (view netlist (viewtype NETLIST) (interface (port O (direction OUTPUT)) (port I0 (direction INPUT)) (port I1 (direction INPUT))))) (cell INV (celltype GENERIC) (view netlist (viewtype NETLIST) (interface (port I (direction INPUT)) (port O (direction OUTPUT)))))) (Library work (edifLevel 0) (technology (numberDefinition)) (cell inverter (celltype GENERIC) (view inverter (viewtype NETLIST) (interface (port a (direction INPUT)) (port b (direction INPUT)) (port y (direction OUTPUT))) (contents (instance y_INST_0 (viewref netlist (cellref LUT2 (libraryref hdi_primitives))) (property INIT (string "4'h8"))) (net a (joined (portref I0 (instanceref y_INST_0)) (portref a))) (net b (joined (portref I1 (instanceref y_INST_0)) (portref b))) (net y (joined (portref O (instanceref y_INST_0)) (portref y))))))) (design inverter (cellref inverter (libraryref work)) (property part (string "xczu3eg-sbva484-1-e"))))"#
    );
    assert_eq!(match_check(actual), 0);
}
