pub mod xsd;

#[macro_export]
macro_rules! impl_string_based {
    ($reader:ident,$st:block) => {
        {
        let mut ret = String::new();
        let mut err = None;
        let mut level:u32 = 0;
        loop {
            let event = $reader.peek()?;
            match event {
                xml::reader::XmlEvent::StartElement { name:_, attributes:_, namespace:_ } => {
                    if level == 0 {
                        level += 1;
                    }else{
                        err = Some("#name should be a text, but get a complex type".to_string());
                        break;
                    }
                }
                xml::reader::XmlEvent::EndElement { name: _ } => {
                    level = level - 1;
                    if level == 0 {
                        break;
                    }
                }
                xml::reader::XmlEvent::Characters(ref text_content) => {
                    ret.write_str(text_content).unwrap();
                }
                _ => {
                    // pass
                }
            }
            $reader.next_event()?;
        }
        if let Some(err_str) = err {
            Err(err_str)
        } else {
            let sb = Self::default();
            sb.str = ret;
            Ok(sb)
        }
        }
    };
}
