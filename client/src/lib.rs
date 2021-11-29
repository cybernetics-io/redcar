mod config;
mod client;

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::client::{Client};

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn put_and_range_works() {
        aw!(async {
            let conf = Config{
                host: "http://127.0.0.1:8519"
            };
            let mut c = Client::new(&conf);
            let key = String::from("bar");
            let val = String::from("foo");
            c.put(key.clone().into_bytes(), val.clone().into_bytes()).await;
            let r_val = c.range(key.clone().into_bytes()).await;
            if r_val[0] != val.clone().into_bytes() {
                panic!("r val is not val: {:?} {:?}", String::from_utf8(r_val[0].clone()).unwrap(), val)
            }
        });
    }
}
