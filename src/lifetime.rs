#[allow(dead_code)]
struct Foo<'a> {
    v: Option<&'a str>,
}

impl<'a> Foo<'a> {
    #[allow(dead_code)]
    fn mod_with_ref(&mut self) -> Option<&'a str> {
        let v = &mut self.v?;
        *v = "mod with ref";
        Some(v)
    }

    #[allow(dead_code)]
    fn mod_with_as_mut(&mut self) -> Option<&'a str> {
        let v = self.v.as_mut()?;
        *v = "mod with as mut";
        Some(v)
    }
}

#[allow(dead_code)]
struct Bar {
    v: Option<String>,
}
impl Bar {
    // #[allow(dead_code)]
    // fn mod_with_ref(&mut self) -> Option<&String> {
    //     let v = &mut self.v?;
    //     *v = "mod with ref".to_string();
    //     Some(v)
    // }

    #[allow(dead_code)]
    fn mod_with_as_mut(&mut self) -> Option<&String> {
        let v = self.v.as_mut()?;
        *v = "mod with as mut".to_string();
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_with_ref_lifetime() {
        let mut foo = Foo { v: Some("init") };
        assert_eq!(foo.mod_with_ref(), Some("mod with ref"));
        //NOTE: value didn't change despite &mut
        //Why? option? is usually move semantic which is why we cannot compile Bar::mod_with_ref
        // but since Foo is a ref &'a then it is copy semantic
        // then we do *v = "mod with ref".to_string(); it changes different address
        assert_eq!(foo.v, Some("init"));

        //as mut
        assert_eq!(foo.mod_with_as_mut(), Some("mod with as mut"));
        assert_eq!(foo.v, Some("mod with as mut"));

        // Bar
        let mut bar = Bar {
            v: Some("init".to_string()),
        };
        assert_eq!(bar.mod_with_as_mut(), Some(&"mod with as mut".to_string()));
        assert_eq!(bar.v, Some("mod with as mut".to_string()));
    }
}
