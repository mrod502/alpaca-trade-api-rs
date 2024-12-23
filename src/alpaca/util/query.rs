use std::collections::HashMap;
use urlencoding::encode;

#[derive(Clone)]
pub(crate) struct Query {
    values: HashMap<String, Vec<String>>,
}
#[macro_export]
macro_rules! query {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_qry = Query::new();
            $(
                let (tmp_key,tmp_val) = $x;
                if let Some(tmp_val) = tmp_val{
                    temp_qry.add(tmp_key, tmp_val);
                }
            )*
            temp_qry
        }
    };
}

impl Query {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, key: &str, val: impl ToString) {
        if let Some(v) = self.values.get_mut(key) {
            v.push(val.to_string());
        } else {
            self.values.insert(key.into(), vec![val.to_string()]);
        }
    }
    pub(crate) fn add_opt(&mut self, key: &str, val: Option<impl ToString>) {
        if let Some(v) = val {
            self.add(key, v);
        }
    }

    pub(crate) fn add_vec(&mut self, key: &str, vals: Vec<impl ToString>) {
        if vals.len() == 0 {
            return;
        }
        let str_vals = vals.into_iter().map(move |v| v.to_string());
        if let Some(v) = self.values.get_mut(key) {
            v.extend(str_vals);
        } else {
            self.values.insert(key.into(), str_vals.collect());
        }
    }
    pub(crate) fn to_query(self, prepend: bool) -> String {
        if self.values.len() == 0 {
            return "".to_string();
        }
        let mut pairs = vec![];
        for (k, v) in self.values.into_iter() {
            for val in v.into_iter() {
                pairs.push(format!("{}={}", k, encode(&val)));
            }
        }

        if prepend {
            format!("?{}", pairs.join("&"))
        } else {
            pairs.join("&")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Query;

    #[test]
    fn test_query() {
        let mut q = Query::new();
        q.add("smth", "val1");
        q.add("smth", "val2");
        assert_eq!("smth=val1&smth=val2", q.to_query(false));
        println!("{}", true.to_string());
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        self.clone().to_query(true)
    }
}
