#[macro_export]
macro_rules! inputs {
    ($( $name:expr => $ty:expr ),*) => {{
        let mut ports: Vec<Port> = Vec::new();
        $(
            let dtype = DataType::from_str($ty).unwrap();
            let port = Port::Input{
                id:$name.to_string(),
                datatype: dtype,
            };
            ports.push(port);
        )*
        ports
    }};
}
