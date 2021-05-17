use std::collections::HashMap;

#[derive(Debug)]
pub struct QStr<'buf> {
    data: HashMap<&'buf str, Val<'buf>>,
}
#[derive(Debug)]
pub enum Val<'buf> {
    Sin(&'buf str),
    Mul(Vec<&'buf str>),
}

impl<'buf> QStr<'buf> {
    pub fn get(&self, key: &str) -> Option<&Val> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QStr<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for substr in s.split('&') {
            let mut key = substr;
            let mut val = "";
            if let Some(i) = substr.find('=') {
                key = &substr[..i];
                val = &substr[i+1..];
            }

            data.entry(key)
            .and_modify(|exis: &mut Val| match exis {
                Val::Sin(preval) => {
                    //let mut vec = vec![preval, val];
                    *exis = Val::Mul(vec![preval, val]);
                },
                Val::Mul(vec) => vec.push(val),
            })
            .or_insert(Val::Sin(val));
        }

        QStr { data }
        
    }
}