import axios from "axios";
import {message} from "antd";
import {clearPersistenceToken} from "./authAPI";

const instance = axios.create({
    baseURL: process.env.REACT_APP_BASE_API,
    timeout: 5 * 1000,
})

export interface CreatedSuccess {
    id: number
}

export interface Page<T> {
    total: number
    list: T[]
}

export function defaultPage<T>(): Page<T> {
    return {total: 0, list: []}
}

instance.interceptors.response.use(function (response) {
    // Any status code that lie within the range of 2xx cause this function to trigger
    // Do something with response data
    return response;
}, function (error) {
    // Any status codes that falls outside the range of 2xx cause this function to trigger
    // Do something with response error
    if (error.response.status === 401) {
        clearPersistenceToken()
        //window.location.reload()
        message.error('auth expire, please reload');
    } else if (error.response.data && !error.config.disableDefaultErrorHandler) {
        message.error(error.response.data)
    }
    return Promise.reject(error);
})

let tokenInterceptorId: number | null = null
let token: string

export function updateHttpToken(tokenStr: string) {
    token = tokenStr
    if (tokenInterceptorId != null) {
        return
        //instance.interceptors.request.eject(tokenInterceptorId)
        //tokenInterceptorId = null
    }
    tokenInterceptorId = instance.interceptors.request.use(
        config => {
            if (config.headers) {
                config.headers['Authorization'] = token
            } else {
	 	//@ts-ignore    
		config.headers = {Authorization: token}    
            }
            return config
        }
    )
}

export default instance;
