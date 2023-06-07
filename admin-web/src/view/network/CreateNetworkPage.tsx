import {Button, Form, Input, message, Select} from "antd";
import {useIntl} from "react-intl";
import {CreateNetwork, createNetwork, NetworkProtocol} from "../../api/networkAPI";
import {useNavigate} from "react-router-dom";

const {Option} = Select;
export function CreateNetworkPage() {
    const [form] = Form.useForm<CreateNetwork>()
    const navi = useNavigate()
    const intl = useIntl()
    const onFinish = async () => {
        // form.validateFields().then((data) => {
        //     console.log(data)
        // })
        const data = await form.validateFields()
        const {id} = (await createNetwork(data)).data
        message.info(intl.formatMessage({id: 'result.createSuccess'}, {'0': intl.formatMessage({id: 'nav.network'})}
        ))
        navi(`/network/${id}`, {replace: true})
    }
    return (<Form
        style={{maxWidth: '300px', margin: '30px auto'}}
        form={form}
        labelCol={{span: 8}}
        wrapperCol={{span: 16}}
    >
        <Form.Item
            label={intl.formatMessage({id: 'name'})}
            required
            name="name"
            rules={[{required: true}]}
        >
            <Input/>
        </Form.Item>
        <Form.Item
            label="IP Range"
            required
            rules={[{required: true}]}
            name="addressRange"
        >
            <Input placeholder="10.2.0.0/16"/>
        </Form.Item>
        <Form.Item
            label="Protocol"
            required
            rules={[{required: true}]}
            name="protocol"
        >
            <Select>
                <Option value={NetworkProtocol.TCP}>TCP</Option>
                <Option value={NetworkProtocol.UDP}>UDP</Option>
            </Select>
        </Form.Item>
        <Form.Item wrapperCol={{offset: 8, span: 16}}>
            <Button type="ghost" onClick={onFinish}>{intl.formatMessage({id: 'submit'})}</Button>
        </Form.Item>
    </Form>)
}
