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

//use std::process::Command;
//use Edif;
use edifier::ast::*;
use edifier::string_helpers::*;

/*
#[test]
fn runs_without_arguments() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.assert().success();
}*/

// Test 1: we should get a minimal edif with no elements
#[test]
fn empty_edif() {
    let ed = Edif {
        name: "ed".to_string(),
        elements: EdifElements::from(Vec::new()),
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(
        actual,
        "(edif ed (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)))"
    );
    assert_eq!(match_check(actual), 0);
}

// Test 2: with library elements
#[test]
fn edif_lib() {
    let libelem = EdifElement::from(Library::new("mylib"));
    let ed = Edif {
        name: "ed2".to_string(),
        elements: EdifElements::from(vec![libelem]),
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(actual,
        "(edif ed2 (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)) (Library mylib (edifLevel 0) (technology (numberDefinition))))" );
    assert_eq!(match_check(actual), 0);
}

// Test 2.1: with library that has cells
#[test]
fn edif_lib_cells() {
    let libelem = EdifElement::from(Library::new_with_cells(
        "mylib",
        Cells::from(vec![Cell::new_with_views(
            "mycell",
            CellViews::from(vec![CellView::new("myview")]),
        )]),
    ));

    let ed = Edif {
        name: "ed2".to_string(),
        elements: EdifElements::from(vec![libelem]),
    };

    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(actual,
        "(edif ed2 (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)) (Library mylib (edifLevel 0) (technology (numberDefinition)) (cell mycell (celltype GENERIC) (view myview (viewtype NETLIST)))))" );
    assert_eq!(match_check(actual), 0);
}

// Test 3: cell with no elements
#[test]
fn cell_empty() {
    let mycell = Cell::new("mycell");
    let actual = serde_sexpr::to_string(&mycell).unwrap();
    assert_eq!(actual, "(cell mycell (celltype GENERIC))");
    assert_eq!(match_check(actual), 0);
}

// Test 4: cell view with no elements
#[test]
fn cellview_empty() {
    let myview = CellView::new("myview");
    let actual = serde_sexpr::to_string(&myview).unwrap();
    assert_eq!(actual, "(view myview (viewtype NETLIST))");
    assert_eq!(match_check(actual), 0);
}

// Test 4.1: cell view with properties
#[test]
fn cellview_w_props() {
    let mut myview = CellView::new("myview");
    myview
        .properties
        .push(Property::new_string("usability", "very_high_please"));
    let actual = serde_sexpr::to_string(&myview).unwrap();
    assert_eq!(
        actual,
        r#"(view myview (viewtype NETLIST) (property usability (string "very_high_please")))"#
    );
    assert_eq!(match_check(actual), 0);
}

// Test 5: interface with no elements
#[test]
fn interface_empty() {
    let myinterface = CellInterface(Vec::new());
    let actual = serde_sexpr::to_string(&myinterface).unwrap();
    assert_eq!(actual, "()");
    assert_eq!(match_check(actual), 0);
}

// Test 6: interface with 2 elements
#[test]
fn interface_some() {
    let porta = InterfacePort::new_input("a");
    let portb = InterfacePort::new_input("b");
    let myinterface = CellInterface(vec![porta, portb]);
    let actual = serde_sexpr::to_string(&myinterface).unwrap();
    assert_eq!(
        actual,
        "(interface (port a (direction INPUT)) (port b (direction INPUT)))"
    );
    assert_eq!(match_check(actual), 0);
}

// Test 7: contents  empty
#[test]
fn contents_empty() {
    let mycontent = CellContents(Vec::new());
    let actual = serde_sexpr::to_string(&mycontent).unwrap();
    assert_eq!(actual, "()");
    assert_eq!(match_check(actual), 0);
}

//test 8: content instance with no properties
#[test]
fn contents_instance_simple() {
    let myinstance = ContentInstance {
        token: StringToken::new("lut4"),
        viewref: "myview".to_string(),
        cellref: CellRef::new("mycellref", "mylibref"),
        properties: PropertyList(Vec::new()),
    };
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(
        actual,
        "(instance lut4 (viewref myview (cellref mycellref (libraryref mylibref))))"
    );
    assert_eq!(match_check(actual), 0);
}

//test 8.1: content instance with properties
#[test]
fn contents_instance_props() {
    let props = PropertyList(vec![
        Property {
            name: "adjustability".to_string(),
            property: PropertyValue::Integer(11),
        },
        Property {
            name: "usability".to_string(),
            property: PropertyValue::String("very_high_please".to_string()),
        },
    ]);
    let myinstance = ContentInstance {
        token: StringToken::new("dsp1"),
        viewref: "myview".to_string(),
        cellref: CellRef::new("mycellref", "mylibref"),
        properties: props,
    };
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(
        actual,
        r#"(instance dsp1 (viewref myview (cellref mycellref (libraryref mylibref))) (property adjustability (integer 11)) (property usability (string "very_high_please")))"#
    );
    assert_eq!(match_check(actual), 0);
}

//test 9: content net
#[test]
fn net_empty() {
    let myinstance = ContentNet::new("y");
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(actual, "(net y)");
    assert_eq!(match_check(actual), 0);
}

//test 9.1: content net renamed
#[test]
fn net_renamed() {
    let myinstance = ContentNet::new_renamed("u_u_sad_4", "u_u_sad[4]");
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(actual, r#"(net (rename u_u_sad_4 "u_u_sad[4]"))"#);
    assert_eq!(match_check(actual), 0);
}

//test 10: port reference
#[test]
fn portref() {
    let myport = PortRef {
        token: PortRefToken::new("y"),
        instanceref: InstanceRef::new("myinst"),
    };
    let actual = serde_sexpr::to_string(&myport).unwrap();
    assert_eq!(actual, "(portref y (instanceref myinst))");
    assert_eq!(match_check(actual), 0);
}

//test 11: multiple port reference
#[test]
fn multi_portref() {
    let myport1 = PortRef {
        token: PortRefToken::new("y"),
        instanceref: InstanceRef::new("myinst"),
    };
    let myport2 = PortRef {
        token: PortRefToken::new("x"),
        instanceref: InstanceRef::new(""),
    };
    let myport3 = PortRef {
        token: PortRefToken::Member(PortMember {
            name: "some_other_port_".to_string(),
            index: 9,
        }),
        instanceref: InstanceRef::new("the_inst"),
    };
    let mut myinstance = ContentNet::new("y");
    myinstance.portlist = PortList(vec![myport1, myport2, myport3]);

    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(
        actual,
        "(net y (joined (portref y (instanceref myinst)) (portref x) (portref (member some_other_port_ 9) (instanceref the_inst))))"
    );
    assert_eq!(match_check(actual), 0);
}

// Test 12: contents with something inside.
#[test]
fn contents_elements() {
    let mut mycontent = CellContents(Vec::new());
    mycontent.push(ContentElement::Net(ContentNet::new("a")));
    mycontent.push(ContentElement::Net(ContentNet::new("b")));

    let actual = serde_sexpr::to_string(&mycontent).unwrap();
    assert_eq!(actual, "(contents (net a) (net b))");
    assert_eq!(match_check(actual), 0);
}

// Test 13: property values
#[test]
fn property_values() {
    let mypropint = PropertyValue::Integer(42);
    let actual = serde_sexpr::to_string(&mypropint).unwrap();
    assert_eq!(actual, "(integer 42)");
    assert_eq!(match_check(actual), 0);

    let mypropstr = PropertyValue::from("64'h00AA00FF33CC0F00".to_string());
    let actual = serde_sexpr::to_string(&mypropstr).unwrap();
    assert_eq!(actual, r#"(string "64'h00AA00FF33CC0F00")"#);
    assert_eq!(match_check(actual), 0);
}

// Test 14: property
#[test]
fn property_complete() {
    let mypropval = PropertyValue::from(
        "256'h0000000000000000000000000000000000000000000000000000000000000000".to_string(),
    );
    let myprop = Property {
        name: "INITP_01".to_string(),
        property: mypropval,
    };
    let actual = serde_sexpr::to_string(&myprop).unwrap();
    assert_eq!(
        actual,
        r#"(property INITP_01 (string "256'h0000000000000000000000000000000000000000000000000000000000000000"))"#
    );
    assert_eq!(match_check(actual), 0);
}

// Test 15: rename
#[test]
fn rename() {
    let myren = Rename::new("my_exp_0", r#"my_exp[0]"#);
    let actual = serde_sexpr::to_string(&myren).unwrap();
    assert_eq!(actual, r#"(rename my_exp_0 "my_exp[0]")"#);
    assert_eq!(match_check(actual), 0);
}

// Test 16: References
#[test]
fn references() {
    let libref = LibraryRef::new("anew_libref");
    let actual = serde_sexpr::to_string(&libref).unwrap();
    assert_eq!(actual, r#"(libraryref anew_libref)"#);
    assert_eq!(match_check(actual), 0);

    let instref = InstanceRef::new("anew_instance");
    let actual = serde_sexpr::to_string(&instref).unwrap();
    assert_eq!(actual, r#"(instanceref anew_instance)"#);
    assert_eq!(match_check(actual), 0);
}

// Test 17: Designs
#[test]
fn design_empty() {
    let design = Design::new("mydesign", CellRef::new("LUT4", "hdi_primitives"));
    let actual = serde_sexpr::to_string(&design).unwrap();
    assert_eq!(
        actual,
        r#"(design mydesign (cellref LUT4 (libraryref hdi_primitives)))"#
    );
    assert_eq!(match_check(actual), 0);
}

// Test 18: Designs with properties
#[test]
fn design_props() {
    let mut proplist = PropertyList(Vec::new());
    proplist.push(Property::new_string(
        "XLNX_PROJ_DIR",
        "/home/clavin/testRW/picoblaze",
    ));
    proplist.push(Property::new_string("part", "xcvu3p-ffvc1517-2-i"));

    let design =
        Design::new_with_prop("mydesign", CellRef::new("LUT4", "hdi_primitives"), proplist);
    let actual = serde_sexpr::to_string(&design).unwrap();
    assert_eq!(
        actual,
        r#"(design mydesign (cellref LUT4 (libraryref hdi_primitives)) (property XLNX_PROJ_DIR (string "/home/clavin/testRW/picoblaze")) (property part (string "xcvu3p-ffvc1517-2-i")))"#
    );
    assert_eq!(match_check(actual), 0);
}
