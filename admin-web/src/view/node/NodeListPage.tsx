import {Button, message, Modal, Space, Table} from "antd";
import {useEffect, useState} from "react";
import {getNodeActiveCode, getNodeList, Node, NodeStatus, updateNodeStatus} from "../../api/nodeAPI";
import {Link, useParams} from "react-router-dom";
import {ColumnsType} from "antd/es/table";
import {useIntl} from "react-intl";
import {enumToDesc} from "../../local/intl";
import {QRCodeCanvas} from "qrcode.react";
import copy from "copy-to-clipboard";
import {ID} from "../../api/http";

export default function NodeListPage() {
    const {networkId} = useParams<{ networkId: string }>()
    const [showActiveCode, setShowActiveCode] = useState<string>()
    const [data, setData] = useState<Node[]>([])
    const intl = useIntl()
    useEffect(() => {
        getNodeList(networkId!).then((d) => setData(d.data))
    }, [networkId])

    const showActiveModalAction = async (networkId: ID, nodeId: ID) => {
        const activeCode = await getNodeActiveCode(networkId, nodeId)
        setShowActiveCode(activeCode)
    }
    const updateNodeStatusAction = async (networkId: ID, nodeId: ID, status: NodeStatus.Forbid | NodeStatus.Normal) => {
        await updateNodeStatus(networkId, nodeId, status)
        message.info(intl.formatMessage({id: 'result.updateSuccess'}, {'0': intl.formatMessage({id: 'status'})}))
        getNodeList(networkId).then((d) => setData(d.data))
    }

    const columns: ColumnsType<Node> = [
        {
            title: 'ID',
            dataIndex: 'id',
        },
        {
            title: intl.formatMessage({id: 'name'}),
            dataIndex: 'name',
        },
        {
            title: 'IP',
            dataIndex: 'ip',
        },
        {
            title: intl.formatMessage({id: 'type'}),
            render(_, item) {
                return enumToDesc('node.type', item.nodeType)
            }
        }, {
            title: intl.formatMessage({id: 'status'}),
            render(_, item) {
                return enumToDesc('node.status', item.status)
            }
        },
        {
            title() {
                return (<Link to={`/network/${networkId}/node/create`}><Button
                    type="ghost">{intl.formatMessage({id: 'create'})}</Button></Link>)
            },
            render(_, item) {
                const showInviteBtn = item.status === NodeStatus.Waiting ? <Button
                    onClick={() => showActiveModalAction(item.networkId, item.id)}>{intl.formatMessage({id: 'active_code'})}</Button> : null
                let changeStatusBtn = null;
                if(item.status === NodeStatus.Normal) {
                    changeStatusBtn = <Button
                        onClick={() => updateNodeStatusAction(item.networkId, item.id, NodeStatus.Forbid)}>
                        {enumToDesc('node.status', NodeStatus.Forbid)}
                    </Button>
                } else if (item.status !== NodeStatus.Waiting) {
                    changeStatusBtn = <Button
                        onClick={() => updateNodeStatusAction(item.networkId, item.id, NodeStatus.Normal)}>
                        {enumToDesc('node.status', NodeStatus.Normal)}
                    </Button>
                }

                return <Space>
                    <Link to={`/network/${networkId}/node/${item.id}`}>
                        <Button>{intl.formatMessage({id: 'detail'})}</Button>
                    </Link>
                    {showInviteBtn}
                    {changeStatusBtn}
                </Space>
            }
        }
    ]
    return (<>
        <Table dataSource={data} columns={columns} rowKey='id' bordered style={{flex: 1, minHeight: 500}}/>
        <Modal open={showActiveCode !== undefined}
               width={800}
               title={intl.formatMessage({id: 'active_code'})}
               onCancel={() => setShowActiveCode(undefined)}
               centered
               footer={null}
        >
            <div style={{width: '100%', display: 'none'}}>
                QRCode For Mobile:<br/><br/>
                <div style={{
                    background: 'white',
                    padding: '8px',
                    width: '166px',
                    height: '166px',
                    display: 'block',
                    margin: "0 auto"
                }}>
                    <QRCodeCanvas value={showActiveCode ?? ''} size={150}/>
                </div>
            </div>
                <br/>
                Command Line:<br/>
                <Button type="text"
                        onClick={() => {
                            copy(`fornet-cli join ${showActiveCode}`, {format: 'text/plain'})
                            message.info(intl.formatMessage({id: 'copy_it'}))
                        }
                        }>{`fornet-cli join ${showActiveCode}`}</Button>
        </Modal>
    </>)
}
