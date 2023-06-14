import {useIntl} from "react-intl";
import {Button, message, Modal, Popconfirm, Space, Table} from "antd";
import {ColumnsType} from "antd/es/table";
import {deleteNetwork, getNetworkInviteCode, Network, networkList} from "../../api/networkAPI";
import {useEffect, useState} from "react";
import DayjsFormat from "../../component/DayjsFormat";
import {Link, useSearchParams} from "react-router-dom";
import {defaultPage, ID, Page} from "../../api/http";
import {QRCodeCanvas} from "qrcode.react";
import copy from "copy-to-clipboard";
import {getPersistenceToken, getSSOInviteCode} from "../../api/authAPI";
import _ from 'lodash'

export default function NetworkListPage() {
    const [{list, total}, setData] = useState<Page<Network>>(defaultPage())
    const [showModal, setShowModal] = useState<string | undefined>()
    const [showSSO, setShowSSO] = useState<string|undefined>()
    const intl = useIntl()
    const [searchParams, setSearchParams] = useSearchParams();
    const page = parseInt(searchParams.get('page') ?? '1')
    const name = searchParams.get('name')
    useEffect(() => {
        networkList(name, page).then((d) => setData(d))
    }, [name, page])

    const showInviteModel = (id: ID) => {
        if((getPersistenceToken()??'').startsWith('Bearer')) {
            getSSOInviteCode(id).then(r => setShowSSO(r))
        }
        getNetworkInviteCode(id).then(setShowModal)
    }
    const deleteNetworkAction = async (id: ID) => {
        await deleteNetwork(id)
        message.info(intl.formatMessage({id: 'result.deleteSuccess'},
            {'0': intl.formatMessage({id: 'nav.network'})}))
        networkList(name, page).then((d) => setData(d))
    }

    const columns: ColumnsType<Network> = [
        {
            title: 'ID',
            dataIndex: 'id',
        },
        {
            title: intl.formatMessage({id: 'name'}),
            dataIndex: 'name',
        },
        {
            title: 'Address Range',
            dataIndex: 'addressRange',
        },
        {
            title: intl.formatMessage({id: 'updatedAt'}),
            render(_, item) {
                return <DayjsFormat dateTime={item.updatedAt}/>
            }
        },
        {
            title: () => {
                return (<Link to="/network/create"><Button
                    type="ghost">{intl.formatMessage({id: 'create'})}</Button></Link>)
            },
            render: (_, item) => {
                return <Space>
                    {/*<Button type="text">Graph</Button>*/}
                    <Link to={`/network/${item.id}`}><Button
                        type="text">{intl.formatMessage({id: 'detail'})}</Button></Link>
                    <Link to={`/network/${item.id}/node`}><Button
                        type="text">{intl.formatMessage({id: 'nav.node'})}</Button></Link>
                    <Button type="text"
                            onClick={() => showInviteModel(item.id)}>{intl.formatMessage({id: 'invite_code'})}</Button>
                    <Popconfirm
                        onConfirm={() => deleteNetworkAction(item.id)}
                        title={intl.formatMessage({id:'action.deleteConfirm'},{'0': intl.formatMessage({id: 'nav.network'})})}
                    >
                        <Button type="text">{intl.formatMessage({id: 'delete'})}</Button>
                    </Popconfirm>

                </Space>
            }
        }
    ]
    const showSSOCommand = showSSO!== undefined ? <>
        SSO Login (Recommend, needs web browser to login):<br/>
        <Button type="text" onClick={() => {
            copy(`fornet-cli join ${showSSO}`, {format:'text/plain'})
            message.info(intl.formatMessage({id: 'copy_it'}))
        }}>{`fornet-cli join ${showSSO}`}</Button>
        <br/>
        <br/>
    </>: null
    return (<div style={{flex: 1}}>
        <Table dataSource={list} columns={columns} rowKey='id' bordered style={{flex: 1, minHeight: '500px'}}
               pagination={{
                   pageSize: 10, total, current: page,
                   hideOnSinglePage: true,
                   onChange(page, x) {
                       setSearchParams(_.pickBy({page: `${page}`, name: name as any}, _.identity))
                   }
               }}/>

        <Modal open={showModal !== undefined}
               title={intl.formatMessage({id: 'invite_code'})}
               onCancel={() => setShowModal(undefined)}
               width={800}
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
                    <QRCodeCanvas value={showSSO ?? showModal ?? ''} size={150}/>
                </div>
            </div>
            <br/>
            {showSSOCommand}
            Command Line{showSSO? ' (where can not use browser)': ''}:<br/>
            <Button type="text"
                    onClick={() => {
                        copy(`fornet-cli join ${showModal}`, {format: 'text/plain'})
                        message.info(intl.formatMessage({id: 'copy_it'}))
                    }
                    }>{`fornet-cli join ${showModal}`}</Button>
            <br/>
        </Modal>
    </div>)
}


