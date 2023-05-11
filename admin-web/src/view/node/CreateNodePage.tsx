import {useNavigate, useParams} from "react-router-dom";
import {Button, Form, Input, Radio, message, InputNumber, Col, Row} from "antd";
import {useIntl} from "react-intl";
import {createNode, CreateNode, NodeType} from "../../api/nodeAPI";
import {enumToDesc} from "../../local/intl";
import {getNetwork, Network} from "../../api/networkAPI";
import {useEffect, useState} from "react";
import {RelayConfigView} from "./NodeDetailPage";

export function CreateNodePage() {
    let {networkId} = useParams<{ networkId: string }>()
    const [network, setNetwork] = useState<Network>()
    const [form] = Form.useForm<CreateNode>()
    const nodeType = Form.useWatch('nodeType', form);
    const navi = useNavigate()
    const intl = useIntl()
    useEffect(() => {
        if (networkId) {
            getNetwork(parseInt(networkId)).then(r => setNetwork(r))
        }
    }, [networkId])

    if (!network) {
        return <></>
    }

    const onFinish = async () => {
        const data = await form.validateFields()
        if (data.nodeType === NodeType.Client) {
            delete data.setting.endpoint
            delete data.setting.postUp
            delete data.setting.postDown

        }
        if (!data.ip) {
            delete data.ip
        }

        const {id} = (await createNode(parseInt(networkId as string), data)).data
        message.info(intl.formatMessage({id: 'result.createSuccess'}, {'0': intl.formatMessage({id: 'nav.node'})}))
        navi(`/network/${networkId}/node/${id}`, {replace: true})
    }

    return (
        <Form
            style={{maxWidth: '1000px',margin: '30px 50px'}}
            labelCol={{span:8}}
            wrapperCol={{span: 16}}
            form={form}
            initialValues={{
                nodeType: NodeType.Client,
                setting: {
                    postUp: 'iptables -A FORWARD -i fort0 -j ACCEPT; iptables -A FORWARD -o fort0 -j ACCEPT; iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE',
                    postDown: 'iptables -D FORWARD -i fort0 -j ACCEPT; iptables -D FORWARD -o fort0 -j ACCEPT; iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE',
                }

            }}
        >
            <Row justify="center"><h3>Node Config</h3></Row>
            <Row>
                <Col span={8}>
                    <Form.Item
                        required
                        name="name"
                        label={intl.formatMessage({id: 'name'})}
                        rules={[{required: true}]}
                    ><Input/></Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item name="ip" label="IP" tooltip="If empty, would auto set"><Input
                        placeholder={`network ip range:${network.addressRange}`}/></Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item name={['setting', 'mtu']} label="MTU" rules={[{type: 'integer'}]}>
                        <InputNumber controls={false}
                                     style={{width: "100%"}}
                                     placeholder={`network default mtu: ${network.setting.mtu}`}
                                     min={1000}
                                     max={1600}/></Form.Item>

                </Col>
            </Row>
            <Row>
                <Col span={8}>
                    <Form.Item name={['setting', 'keepAlive']} label="Keep Alive">
                        <InputNumber controls={false}
                                     style={{width: "100%"}}
                                     placeholder={`network default keepAlive: ${network.setting.keepAlive}`}
                                     min={10}
                                     max={600}
                        /></Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item name={['setting', 'port']} label="Port">
                        <InputNumber controls={false}
                                     style={{width: "100%"}}
                                     min={2000}
                                     max={56000}
                                     placeholder={`network default port: ${network.setting.port}`}/>
                    </Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item name="nodeType"
                               label={`${intl.formatMessage({id: 'nav.node'})}${intl.formatMessage({id: 'type'})}`}>
                        <Radio.Group>
                            <Radio value={NodeType.Client}>{enumToDesc('node.type', NodeType.Client)}</Radio>
                            <Radio value={NodeType.Relay}>{enumToDesc('node.type', NodeType.Relay)}</Radio>
                        </Radio.Group>
                    </Form.Item>
                </Col>
            </Row>
            <RelayConfigView  hide={form.getFieldValue('nodeType') === NodeType.Client}/>
            <Form.Item wrapperCol={{offset: 10}}>
                <Button type="ghost" onClick={onFinish}>{intl.formatMessage({id: 'submit'})}</Button>
            </Form.Item>
        </Form>)
}
