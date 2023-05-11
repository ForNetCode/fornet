import {createIntl, createIntlCache} from 'react-intl'
import dayjs from "dayjs";

import "dayjs/locale/zh-cn"
import "dayjs/locale/en"

import zhCN from 'antd/es/locale/zh_CN'
import enUS from 'antd/es/locale/en_US'
import zhCNLocal from './zh-CN'
import enUSLocal from './en-US'

import localizedFormat from "dayjs/plugin/localizedFormat";

// This is optional but highly recommended
// since it prevents memory leak
const cache = createIntlCache()


const language: string = navigator.language.split(/[-_]/)[0];
// const language:string = 'en'

// dayjs.extend(isLeapYear)
dayjs.locale(language === 'zh' ? 'zh-cn' : 'en')
dayjs.extend(localizedFormat)

const locale = language === 'zh' ? {
    antd: zhCN,
    app: zhCNLocal,
} : {
    antd: enUS,
    app: enUSLocal
}

const intl = createIntl({
    locale: language,
    messages: locale.app,
}, cache)

export const antdLocale = locale.antd;

export default intl;


export function enumToDesc(prefixKey:string, number:number) {
    return intl.formatMessage({id: `${prefixKey}.${number}`})
}
export function enumToDescNode(prefixKey:string, number:number) {
    return (<>{enumToDesc(prefixKey, number)}</>)
}
