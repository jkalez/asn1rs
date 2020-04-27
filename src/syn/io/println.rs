use crate::prelude::*;

#[derive(Default)]
pub struct PrintlnWriter(usize);

impl PrintlnWriter {
    fn indented_println(&self, text: &str) {
        println!("{}{}", " ".repeat(self.0), text);
    }

    fn with_increased_indentation<R, F: Fn(&mut Self) -> R>(&mut self, f: F) -> R {
        self.0 += 1;
        let r = f(self);
        self.0 -= 1;
        r
    }
}

impl Writer for PrintlnWriter {
    type Error = ();

    fn write_sequence<C: sequence::Constraint, F: Fn(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        f: F,
    ) -> Result<(), Self::Error> {
        self.indented_println(&format!("Writing sequence {}", C::NAME));
        self.with_increased_indentation(|w| f(w))
    }

    fn write_opt<T: WritableType>(&mut self, value: Option<&T::Type>) -> Result<(), Self::Error> {
        self.indented_println("Writing OPTIONAL");
        self.with_increased_indentation(|w| {
            if let Some(value) = value {
                w.indented_println("Some");
                w.with_increased_indentation(|w| T::write_value(w, value))
            } else {
                w.indented_println("None");
                Ok(())
            }
        })
    }

    fn write_int(&mut self, value: i64, (min, max): (i64, i64)) -> Result<(), Self::Error> {
        self.indented_println(&format!("WRITING Integer({}..{}) {}", min, max, value));
        Ok(())
    }

    fn write_int_max(&mut self, value: u64) -> Result<(), Self::Error> {
        self.indented_println(&format!("WRITING Integer {}", value));
        Ok(())
    }

    fn write_utf8string<C: utf8string::Constraint>(
        &mut self,
        value: &str,
    ) -> Result<(), Self::Error> {
        self.indented_println(&format!(
            "Writing Utf8String({}..{}): {}",
            C::MIN
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MIN")),
            C::MAX
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MAX")),
            value
        ));
        Ok(())
    }
}
