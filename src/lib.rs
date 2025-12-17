use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod paya {
    use std::collections::HashMap;

    use deboa::{Deboa, cookie::DeboaCookie, request::DeboaRequest};
    use http::{
        HeaderMap, HeaderName, HeaderValue, Method,
        header::{CONTENT_TYPE, HOST},
    };
    use pyo3::prelude::*;
    use pyo3::pycell::PyRefMut;
    use pyo3_async_runtimes::tokio::future_into_py;
    use url::Url;

    #[pyclass]
    pub struct Paya {
        base_url: Url,
        token: Option<String>,
        method: Method,
        path: String,
        cookies: Option<HashMap<String, DeboaCookie>>,
        headers: HeaderMap,
        retries: u32,
        body: Vec<u8>,
    }

    #[pymethods]
    impl Paya {
        #[new]
        fn new(url: &str) -> Paya {
            let base_url = Url::parse(url).expect("Please provide a valid URL!");
            let mut headers = HeaderMap::new();
            headers.insert(
                HOST,
                HeaderValue::from_str(base_url.host_str().unwrap()).unwrap(),
            );
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str("application/json").unwrap(),
            );

            Paya {
                base_url,
                token: None,
                method: Method::GET,
                path: String::new(),
                retries: 0,
                headers,
                cookies: None,
                body: Vec::new(),
            }
        }

        fn cookie<'a>(mut slf: PyRefMut<'a, Self>, key: &str, value: &str) -> PyRefMut<'a, Self> {
            let cookie = DeboaCookie::new(key, value);
            if let Some(cookies) = &mut slf.cookies {
                cookies.insert(key.to_string(), cookie);
            } else {
                slf.cookies = Some(
                  HashMap::from([(key.to_string(), cookie)])
                );
            }
            slf
        }

        fn header<'a>(mut slf: PyRefMut<'a, Self>, key: &str, value: &str) -> PyRefMut<'a, Self> {
            slf.headers.insert(
                HeaderName::from_bytes(key.as_bytes()).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
            slf
        }

        fn bearer_auth<'a>(mut slf: PyRefMut<'a, Self>, token: &str) -> PyRefMut<'a, Self> {
            slf.token = Some(token.to_string());
            slf
        }

        fn set_content_type<'a>(mut slf: PyRefMut<'a, Self>, content_type: &str) -> PyRefMut<'a, Self> {
            slf.headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(content_type).unwrap(),
            );
            slf
        }

        fn retries<'a>(mut slf: PyRefMut<'a, Self>, retries: u32) -> PyRefMut<'a, Self> {
            slf.retries = retries;
            slf
        }

        fn body<'a>(mut slf: PyRefMut<'a, Self>, body: Vec<u8>) -> PyRefMut<'a, Self> {
            slf.body = body;
            slf
        }

        fn get<'a>(mut slf: PyRefMut<'a, Self>, path: &str) -> PyRefMut<'a, Self> {
            slf.path = path.to_string();
            slf.method = Method::GET;
            slf
        }

        fn post<'a>(mut slf: PyRefMut<'a, Self>, path: &str) -> PyRefMut<'a, Self> {
            slf.path = path.to_string();
            slf.method = Method::POST;
            slf
        }

        fn put<'a>(mut slf: PyRefMut<'a, Self>, path: &str) -> PyRefMut<'a, Self> {
            slf.path = path.to_string();
            slf.method = Method::PUT;
            slf
        }

        fn delete<'a>(mut slf: PyRefMut<'a, Self>, path: &str) -> PyRefMut<'a, Self> {
            slf.path = path.to_string();
            slf.method = Method::DELETE;
            slf
        }

        fn patch<'a>(mut slf: PyRefMut<'a, Self>, path: &str) -> PyRefMut<'a, Self> {
            slf.path = path.to_string();
            slf.method = Method::PATCH;
            slf
        }
    }

    #[pyclass]
    pub struct Massa {
        status_code: u16,
        body: Vec<u8>,
        headers: HeaderMap,
    }

    #[pymethods]
    impl Massa {
        pub fn headers(&self, key: &str) -> Option<&str> {
            let header = self.headers.get(key);
            if let Some(header) = header {
                Some(header.to_str().unwrap())
            } else {
                None
            }
        }

        pub fn status_code(&self) -> u16 {
            self.status_code
        }

        pub fn body(&self) -> Vec<u8> {
            self.body.clone()
        }
    }

    #[pyfunction]
    fn send<'p>(py: Python<'p>, paya: &Paya) -> PyResult<Bound<'p, PyAny>> {
        let mut base_url = paya.base_url.clone();
        let path_and_query = paya.path.split_once('?');
        let path = if let Some((path, query)) = path_and_query {
            base_url.set_query(Some(query));
            path
        } else {
            &paya.path
        };

        let base_path = paya.base_url.path();
        if base_path == "/" {
            base_url.set_path(path);
        } else {
            base_url.set_path(&format!("{}{}", base_path, path));
        }

        let request = DeboaRequest::from(base_url.as_str())
            .expect("Invalid URL!")
            .retries(paya.retries)
            .method(paya.method.clone())
            .headers(paya.headers.clone())
            .raw_body(&paya.body);

        let request = if let Some(cookies) = &paya.cookies {
            request.cookies(cookies.clone())
        } else {
            request
        };

        let request = if let Some(token) = &paya.token {
            request.bearer_auth(token)
        } else {
            request
        };

        let request = request.build().expect("Could not send request!");

        future_into_py(py, async move {
            let mut client = Deboa::new();

            let mut response = client
                .execute(request)
                .await
                .expect("Could not send request!");

            let volta = Massa {
                headers: response.headers().clone(),
                status_code: response.status().as_u16(),
                body: response.raw_body().await,
            };

            Ok(volta)
        })
    }
}
