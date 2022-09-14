```js
// Use
axios.interceptors.request.use(
  async (config) => {
    const headers = {
      Accept: "application/json",
      "Content-Type": "application/json",
      "X-Requested-With": "XMLHttpRequest",
    };
    config.headers = headers;
    return config;
  },
  (error) => {
    return Promise.reject(error); 
  }
);

axios.interceptors.response.use(
  (response) => {
    if (response?.data?.IsRequiredLogin === true) {      
      window.location.href = ApplicationSetting.onixworkUrl + '/Home/Index';          
    }
    return response;
  },
  async (error) => {
    return Promise.reject(error);
  }
);
```
