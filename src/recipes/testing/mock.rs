//inner http client
trait HttpClient {
    fn get(&self, path: &str) -> u16;
}

struct DefaultHttpClient {}
impl Default for DefaultHttpClient {
    fn default() -> Self {
        DefaultHttpClient{}
    }
}

impl HttpClient for DefaultHttpClient {
    fn get(&self, url: &str) -> u16 {
        let resp = reqwest::get(url).unwrap();
        resp.status().as_u16()
    }
}

struct HttpBin<'a> {
    host: &'a str,
    client: Box<dyn HttpClient>
}

impl<'a> Default for HttpBin<'a> {
    fn default() -> Self {
        HttpBin{
            host: "https://httpbin.org",
            client: Box::new(DefaultHttpClient::default())
        }
    }
}

impl<'a> HttpBin<'a> {
    pub fn is_path_ok(&self, path: &'a str) -> bool {
        let url = format!("{}{}", self.host, path);
        let status_code = self.client.get(url.as_str());

        status_code == 200
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //simple fake http client
    struct FakeHttpClient {
        status: u16
    }
    impl FakeHttpClient {
        pub fn new(status: u16) -> Self {
            FakeHttpClient{status}
        }
    }
    impl HttpClient for FakeHttpClient {
        fn get(&self, _path: &str) -> u16 {
            self.status
        }
    }

    #[test]
    fn test_httpbin_is_path_ok() {
        let s = HttpBin{
            host: "https://httpbin.org",
            client: Box::new(FakeHttpClient{status: 200})
        };

        let current = s.is_path_ok("/status/200");
        let expected = true;
        assert_eq!(expected, current);
    }

    #[test]
    fn test_httpbin_is_path_ok_error() {
        let s = HttpBin{
            host: "https://httpbin.org",
            client: Box::new(FakeHttpClient{status: 404})
        };

        let current = s.is_path_ok("/status/200");
        let expected = false;
        assert_eq!(expected, current);
    }
}