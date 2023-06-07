import {useParams} from "react-router-dom";
import {Button, Form, Input, InputNumber, message, Row, Col} from "antd";
import {useIntl} from "react-intl";
import {getNode, NodeType, Node, updateNode} from "../../api/nodeAPI";
import {enumToDesc} from "../../local/intl";
import {useEffect, useState} from "react";
import {getNetwork, Network} from "../../api/networkAPI";


export function RelayConfigView({hide}: {hide:boolean}) {

    const postUpTooltip =(
        <>
            <p>This command run after node up, it normally used for config os to allow ForNet redirect network packages. The command would be:</p>
            <p style={{fontStyle:'italic'}}>iptables -A FORWARD -i for0 -j ACCEPT; iptables -A FORWARD -o for0 -j ACCEPT; iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE</p>
            <p>(Please remember to change <span style={{fontWeight:'bold'}}>eth0</span> to correct ethernet interface)</p>
        </>
    )
    const postDownToolTip = (
        <>
            <p>This command run after node down, it normally used for config OS to remove ForNet postUp command effects. The command would be:</p>
            <p style={{fontStyle:'italic'}}>iptables -D FORWARD -i for0 -j ACCEPT; iptables -D FORWARD -o for0 -j ACCEPT; iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE</p>
            <p>(Please remember to change <span style={{fontWeight:'bold'}}>eth0</span> to correct ethernet interface)</p>

        </>
    )
    if(hide) {
        return <></>
    }
    return (
        <>
            <Row justify="center"><h3>Relay Config</h3></Row>

            <Row>
                <Col span={8}>
                    <Form.Item name={['setting', 'endpoint']} required rules={[{required: true}]} label="Endpoint">
                        <Input type="text" placeholder="Public IP V4"/></Form.Item>
                </Col>
            </Row>
            <Row>
                <Col span={16}>
                <Form.Item
                    tooltip={postUpTooltip}
                    labelCol={{span:4}}
                    wrapperCol={{span: 20}}
                    name={['setting', 'postUp']} required rules={[{required: true}]} label="PostUp">
                    <Input type="text" /></Form.Item>
                </Col>
                <Col offset={1} span={7}>

                </Col>
            </Row>
            <Row>
                <Col span={16}>
                    <Form.Item
                        labelCol={{span:4}}
                        wrapperCol={{span: 20}}
                        tooltip={postDownToolTip}
                        name={['setting', 'postDown']} required rules={[{required: true}]} label="PostDown">
                        <Input type="text" /></Form.Item>
                </Col>
            </Row>
        </>
    )
}
export default function NodeDetailPage() {
    let {networkId, nodeId} = useParams<{ networkId: string, nodeId: string }>()
    const [network, setNetwork] = useState<Network>()
    const [form] = Form.useForm<Node>()
    const intl = useIntl()

    useEffect(() => {
        if (networkId && nodeId) {
            Promise.all([getNetwork(networkId), getNode(networkId, nodeId)]).then(r => {
                setNetwork(r[0])
                form.setFieldsValue(r[1])
            })

        }
        // eslint-disable-next-line
    }, [networkId, nodeId])
    if (!network) {
        return <></>
    }


    const onFinish = async () => {
        const data = await form.validateFields()
        const updateData = {
            name: data.name,
            setting: data.setting,
        }
        await updateNode(networkId!, nodeId!, updateData)
        message.info(intl.formatMessage({id: 'result.updateSuccess'}, {'0': intl.formatMessage({id: 'nav.node'})}))
    }

    return (
        <Form
            style={{maxWidth: '1000px',margin: '30px 50px'}}
            labelCol={{span:8}}
            wrapperCol={{span: 16}}
            form={form}
            initialValues={{
                nodeType: NodeType.Client,
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
                    <Form.Item name="ip" label="IP"><Input placeholder="10.0.0.1" disabled/></Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item name={['setting', 'mtu']} label="MTU" rules={[{type: 'integer'}]}>
                        <InputNumber controls={false}
                                     style={{width: "100%"}}
                                     placeholder={`network default mtu: ${network?.setting.mtu}`}
                                     min={1000}
                                     max={1600}/></Form.Item>

                </Col>
            </Row>
            <Row>
                <Col span={8}>
                    <Form.Item name={['setting', 'keepAlive']} label="Keep Alive">
                        <InputNumber controls={false}
                                     style={{width: "100%"}}
                                     placeholder={`network default keepAlive: ${network?.setting.keepAlive}`}
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
                                     placeholder={`network default port: ${network?.setting.port}`}/>
                    </Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item name="nodeType"
                               label={`${intl.formatMessage({id: 'nav.node'})} ${intl.formatMessage({id: 'type'})}`}>
                        {enumToDesc('node.type', form.getFieldValue('nodeType'))}
                    </Form.Item>
                </Col>
            </Row>
            <RelayConfigView  hide={form.getFieldValue('nodeType') === NodeType.Client }/>
            <Form.Item wrapperCol={{offset: 10}}>
                <Button type="ghost" onClick={onFinish}>{intl.formatMessage({id: 'submit'})}</Button>
            </Form.Item>
        </Form>)
}
