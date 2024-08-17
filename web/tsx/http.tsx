// 添加请求拦截器
axios.interceptors.request.use(
    function (config: any) {
        if (!config.headers) {
            config.headers = { "content-type": "application/json" };
        } else if (!config.headers["content-type"]) {
            config.headers["content-type"] = "application/json";
        }
        return config;
    },
    function (error) {
        // 对请求错误做些什么
        return Promise.reject(error);
    },
);

axios.interceptors.response.use(
    function (response: any) {
        let data = response.data;
        if (data.error) {
            return {
                code: data.error.code,
                msg: data.error.name,
                data: response.config.url,
            };
        } else {
            return data;
        }
    },
    function (error: any) {
        let errJson = error.toJSON();
        return {
            code: errJson.status,
            msg: errJson.msg,
            data: errJson.code,
        };
    },
);
