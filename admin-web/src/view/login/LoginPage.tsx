import React, {useEffect, useState} from 'react';
import {Button, Form, Input} from "antd";
import {FormattedMessage, useIntl} from "react-intl";
import {checkSampleTokenCorrect, getAuthType, savePersistenceToken} from "../../api/authAPI";
import {updateHttpToken} from "../../api/http";
import {Navigate, useLocation} from "react-router-dom";
import './LoginPage.less'
import {useAuth} from "../../auth";
import {initKeycloak} from "../../api/keycloakAPI";

export default function LoginPage() {
    const [authType, setAuthType] = useState<string>()
    const auth = useAuth()
    const intl = useIntl()
    const [form] = Form.useForm()
    const location = useLocation()

    const submit = async () => {
        await form.validateFields()
        let token = form.getFieldValue('token')
        await checkSampleTokenCorrect(token)
        token = `ST ${token}`
        updateHttpToken(token)
        savePersistenceToken(token)
        auth.signIn()
    }
    useEffect(() => {
        if (!auth.isLogin) {
            getAuthType().then(r => {
                setAuthType(r.data.type)
                if (r.data.type !== 'ST') {
                    initKeycloak(r.data).then(() => {
                        auth.signIn()
                    })
                }
            })
        }
    }, [auth])


    if (auth.isLogin) {
        let from = location.state?.from?.pathname || '/network'
        if (from === '/login') {
            from = '/network'
        }
        return <Navigate to={from} replace={true}/>
    }
    if (authType !== 'ST') {
        return <div>Loading...</div>
    }

    return (
        <div className="App">
            <div className="App-header">
                <h2>ForNet</h2>
                <Form form={form}>
                    <Form.Item label={intl.formatMessage({id: 'admin_token'})}
                               name="token"
                               requiredMark='optional'
                               tooltip={intl.formatMessage({id: 'admin_token_desc'})}
                               rules={[{required: true}]}
                    >
                        <Input/>
                    </Form.Item>
                    <Button onClick={submit} htmlType='submit'><FormattedMessage id='login'/></Button>
                </Form>
            </div>
        </div>
    );


}
