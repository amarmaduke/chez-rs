
include!("../bindings.rs");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe { Sscheme_init(None); }
    }
}
