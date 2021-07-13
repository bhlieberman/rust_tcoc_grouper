use std::rc::Rc;

pub struct Claim<T> {
    claim_id: String,
    code: String,
    codes: Vec<T>
}

pub trait IClaim {
    fn set_claim_id(&mut self, claim_id: String) -> ();
    fn get_claim_id(&self) -> String;
}

impl<T> IClaim for Claim<T> {
    fn set_claim_id(&mut self, claim_id: String) -> () {
        self.claim_id = claim_id;
    }
    fn get_claim_id(&self) -> String {
        return self.claim_id.clone()
    }
}

pub trait ICode {
    fn set_value(&mut self, value: String) -> ();
    fn get_value(&self) -> String;
}

pub trait ICodeList<T>: ICode {
    fn add_code(&mut self, value: T) -> ();
    fn get_codes(&self) -> Rc<&Vec<T>>;
}

impl<T> ICode for Claim<T> {
    fn set_value(&mut self, value: String) -> () {
        self.code = value;
    }

    fn get_value(&self) -> String {
        return self.code.clone()
    }
}

impl<T> ICodeList<T> for Claim<T> {
    fn add_code(&mut self, value: T) -> () {
        self.codes.insert(0, value);
    }

    fn get_codes(&self) -> Rc<&Vec<T>> {
        return Rc::new(&self.codes)
    }
}

