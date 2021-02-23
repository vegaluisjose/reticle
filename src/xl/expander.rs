use crate::asm::ast as asm;
use crate::util::errors::Error;
use crate::xl::ast as xl;

#[derive(Clone, Debug)]
pub struct Expander {
    pub count: u64,
    pub prefix: String,
    pub sig: xl::Sig,
    pub body: Vec<xl::Instr>,
}

impl Default for Expander {
    fn default() -> Self {
        Expander {
            count: 0,
            prefix: String::new(),
            sig: xl::Sig::default(),
            body: Vec::new(),
        }
    }
}

impl Expander {
    pub fn sig(&self) -> &xl::Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<xl::Instr> {
        &self.body
    }
    pub fn expand_const(&mut self, instr: &asm::InstrWire) -> Result<(), Error> {
        let attr_term = instr.attr().get_term(0)?;
        let value = attr_term.get_val()?;
        let dst_term = instr.dst().get_term(0)?;
        if let Some(width) = dst_term.width() {
            for i in 0..width {
                let lsb = value >> i;
                let mask = lsb & 1;
                let op = if mask == 1 {
                    xl::OpBasc::Vcc
                } else {
                    xl::OpBasc::Gnd
                };
                let instr_basc = xl::InstrBasc {
                    op,
                    attr: xl::Expr::default(),
                    dst: xl::Expr::default(),
                    arg: xl::Expr::default(),
                };
                self.add_instr(xl::Instr::from(instr_basc));
            }
        }
        Ok(())
    }
    pub fn add_instr(&mut self, instr: xl::Instr) {
        self.body.push(instr);
    }
    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
    }
    pub fn set_sig(&mut self, sig: xl::Sig) {
        self.sig = sig;
    }
}
