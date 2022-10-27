struct MyiPhone13Pro {
    is_on: bool 
}

impl MyiPhone13Pro {
    fn new(is_on: bool) -> MyiPhone13Pro {
        MyiPhone13Pro { is_on: is_on }
    }
}

trait CanTurnOn {
    fn turnon(&mut self); 
}

impl CanTurnOn for MyiPhone13Pro {
    fn turnon(&mut self) {
        self.is_on = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test]
    fn turnon_phone() {
        let mut phone = MyiPhone13Pro::new(false); 
        phone.turnon();
        assert!(phone.is_on); 
    }
}