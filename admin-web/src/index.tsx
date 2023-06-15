import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.less';
import {Provider} from "react-redux";
import {store} from "./store";
import AppRouter from "./routes";
import {ConfigProvider} from "antd";
import 'dayjs/locale/zh-cn'
import 'dayjs/locale/en'
import {RawIntlProvider} from 'react-intl'
import intl, {antdLocale} from './local/intl'
import {clearPersistenceToken, getPersistenceToken} from "./api/authAPI";
import {initKeycloak} from "./api/keycloakAPI";
import {getAppInfo} from "./api/infoAPI";

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);
const render = () => root.render(
    <Provider store={store}>
        <ConfigProvider locale={antdLocale}>
            <RawIntlProvider value={intl}>
                <AppRouter/>
            </RawIntlProvider>
        </ConfigProvider>
    </Provider>
);//

window.onbeforeunload = () => {
    const token = getPersistenceToken()
    if (token != null && token.startsWith('ST')) {
        clearPersistenceToken()
    }
}

const init = () => {
    const token = getPersistenceToken()
    if (token != null && token.startsWith('Bearer')) {
        getAppInfo().then(r => {
            //admin changes auth type
            if (r.data.type === 'ST') {
                clearPersistenceToken()
                window.location.reload()
            } else {
                initKeycloak(r.data).finally(() => {
                    render()
                })
            }
        }).catch(() => {
            render()
        })
    } else {
        render()
    }
}

init()

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
// reportWebVitals(console.log);
