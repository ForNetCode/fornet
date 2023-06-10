import {useEffect} from "react";
import {getNetwork, Network, NetworkProtocol, updateNetwork} from "../../api/networkAPI";
import {useParams} from "react-router-dom";
import {Button, Col, Form, Input, InputNumber, message, Row, Select} from "antd";
import {useIntl} from "react-intl";
import {useForm} from "antd/es/form/Form";

const {Option} = Select;

export default function NetworkDetailPage() {
    const {networkId} = useParams<{ networkId: string }>()
    const intl = useIntl()
    const [form] = useForm<Network>()

    useEffect(() => {
        getNetwork(networkId!).then((d) => {
            form.setFieldsValue(d)
        })
    }, [networkId, form])

    const submit = async () => {
        const data = await form.validateFields()
        await updateNetwork(networkId!, data)
        message.info(intl.formatMessage({id: 'result.updateSuccess'}, {'0': intl.formatMessage({id: 'nav.network'})}))
    }
    return (
        <>
            <Form form={form} requiredMark={false} className="detail_edit"
                  style={{maxWidth: '1000px', margin: '30px auto'}}>
                <Row>
                    <Col span={8}>
                        <Form.Item name="name" rules={[{required: true}]} label={intl.formatMessage({id: 'name'})}>
                            <Input/>
                        </Form.Item>
                    </Col>
                    <Col span={8}>
                        <Form.Item name="addressRange" label="IPv4 Range" rules={[{required: true}]}>
                            <Input/>
                        </Form.Item>
                    </Col>
                    <Form.Item rules={[{required: true}]} name={['setting', 'port']} label="Default Port">
                        <InputNumber controls={false}
                                     min={2000}
                                     max={56000}
                        />
                    </Form.Item>
                </Row>
                <Row>
                    <Col span={8}>
                        <Form.Item rules={[{required: true}]} name={['setting', 'mtu']}
                                   label="Default MTU">
                            <InputNumber controls={false}
                                         min={1000}
                                         max={1600}
                            /></Form.Item>
                    </Col>
                    <Col span={8}>
                        <Form.Item rules={[{required: true}]} name={['setting', 'keepAlive']}
                                   label="Default Keep Alive">
                            <InputNumber controls={false}
                                         style={{width: "100%"}}
                                         placeholder="network default keepAlive"
                                         min={10}
                                         max={600}/>
                        </Form.Item>
                    </Col>
                    <Col span={8}>
                        <Form.Item rules={[{required: true}]} name={['setting','protocol']}
                                   label="Protocol">
                            <Select>
                                <Option value={NetworkProtocol.TCP}>TCP</Option>
                                <Option value={NetworkProtocol.UDP}>UDP</Option>
                            </Select>
                        </Form.Item>
                    </Col>
                </Row>
            </Form>
            <div style={{textAlign: 'center', marginTop: '20px'}}>
                <Button onClick={submit}>{intl.formatMessage({id: 'submit'})}</Button>
            </div>
        </>
    )
}
// <Col span={8}>
//     <Form.Item name={['setting', 'dns']} label="Default DNS"><Input/></Form.Item>
// </Col>